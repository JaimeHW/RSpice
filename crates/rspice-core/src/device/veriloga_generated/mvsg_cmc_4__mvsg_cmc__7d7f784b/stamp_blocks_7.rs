#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
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
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_cbfps1t_slot: &mut f64,
        var_cbfps1t_dn4_slot: &mut f64,
        var_cbfps1t_rv_slot: &mut f64,
        var_cbfps2t_slot: &mut f64,
        var_cbfps2t_dn4_slot: &mut f64,
        var_cbfps2t_rv_slot: &mut f64,
        var_cbfps3t_slot: &mut f64,
        var_cbfps3t_dn4_slot: &mut f64,
        var_cbfps3t_rv_slot: &mut f64,
        var_cbfps4t_slot: &mut f64,
        var_cbfps4t_dn4_slot: &mut f64,
        var_cbfps4t_rv_slot: &mut f64,
        var_ccfps1t_slot: &mut f64,
        var_ccfps1t_dn4_slot: &mut f64,
        var_ccfps1t_rv_slot: &mut f64,
        var_ccfps2t_slot: &mut f64,
        var_ccfps2t_dn4_slot: &mut f64,
        var_ccfps2t_rv_slot: &mut f64,
        var_ccfps3t_slot: &mut f64,
        var_ccfps3t_dn4_slot: &mut f64,
        var_ccfps3t_rv_slot: &mut f64,
        var_ccfps4t_slot: &mut f64,
        var_ccfps4t_dn4_slot: &mut f64,
        var_ccfps4t_rv_slot: &mut f64,
        var_cgfps1t_slot: &mut f64,
        var_cgfps1t_dn4_slot: &mut f64,
        var_cgfps1t_rv_slot: &mut f64,
        var_cgfps2t_slot: &mut f64,
        var_cgfps2t_dn4_slot: &mut f64,
        var_cgfps2t_rv_slot: &mut f64,
        var_cgfps3t_slot: &mut f64,
        var_cgfps3t_dn4_slot: &mut f64,
        var_cgfps3t_rv_slot: &mut f64,
        var_cgfps4t_slot: &mut f64,
        var_cgfps4t_dn4_slot: &mut f64,
        var_cgfps4t_rv_slot: &mut f64,
        var_cgt_slot: &mut f64,
        var_cgt_dn4_slot: &mut f64,
        var_cgt_rv_slot: &mut f64,
        var_cofdmt_slot: &mut f64,
        var_cofdmt0_slot: &mut f64,
        var_cofdmt0_dn4_slot: &mut f64,
        var_cofdmt0_rv_slot: &mut f64,
        var_cofdmt_dn4_slot: &mut f64,
        var_cofdmt_rv_slot: &mut f64,
        var_cofdsmt_slot: &mut f64,
        var_cofdsmt0_slot: &mut f64,
        var_cofdsmt0_dn4_slot: &mut f64,
        var_cofdsmt0_rv_slot: &mut f64,
        var_cofdsmt_dn4_slot: &mut f64,
        var_cofdsmt_rv_slot: &mut f64,
        var_cofdsubmt_slot: &mut f64,
        var_cofdsubmt0_slot: &mut f64,
        var_cofdsubmt0_dn4_slot: &mut f64,
        var_cofdsubmt0_rv_slot: &mut f64,
        var_cofdsubmt_dn4_slot: &mut f64,
        var_cofdsubmt_rv_slot: &mut f64,
        var_cofgsubmt_slot: &mut f64,
        var_cofgsubmt0_slot: &mut f64,
        var_cofgsubmt0_dn4_slot: &mut f64,
        var_cofgsubmt0_rv_slot: &mut f64,
        var_cofgsubmt_dn4_slot: &mut f64,
        var_cofgsubmt_rv_slot: &mut f64,
        var_cofsmt_slot: &mut f64,
        var_cofsmt0_slot: &mut f64,
        var_cofsmt0_dn4_slot: &mut f64,
        var_cofsmt0_rv_slot: &mut f64,
        var_cofsmt_dn4_slot: &mut f64,
        var_cofsmt_rv_slot: &mut f64,
        var_cofssubmt_slot: &mut f64,
        var_cofssubmt0_slot: &mut f64,
        var_cofssubmt0_dn4_slot: &mut f64,
        var_cofssubmt0_rv_slot: &mut f64,
        var_cofssubmt_dn4_slot: &mut f64,
        var_cofssubmt_rv_slot: &mut f64,
        var_guard2_slot: &mut f64,
        var_guard2_rv_slot: &mut f64,
        var_guard3_slot: &mut f64,
        var_guard3_rv_slot: &mut f64,
        var_phit_slot: &mut f64,
        var_phit_dn4_slot: &mut f64,
        var_phit_rv_slot: &mut f64,
        var_tambk_slot: &mut f64,
        var_tambk_rv_slot: &mut f64,
        var_tdut_slot: &mut f64,
        var_tdut_dn4_slot: &mut f64,
        var_tdut_rv_slot: &mut f64,
        var_tnomk_slot: &mut f64,
        var_tnomk_rv_slot: &mut f64,
        var_tsh_slot: &mut f64,
        var_tsh_dn4_slot: &mut f64,
        var_tsh_rv_slot: &mut f64,
    ) {
        let ctx_temp = ctx.temperature();
        let nv4 = ctx.node_voltage(nodes[4]);
        let mut var_cbfps1t: f64 = *var_cbfps1t_slot;
        let mut var_cbfps1t_dn4: f64 = *var_cbfps1t_dn4_slot;
        let mut var_cbfps1t_rv: f64 = *var_cbfps1t_rv_slot;
        let mut var_cbfps2t: f64 = *var_cbfps2t_slot;
        let mut var_cbfps2t_dn4: f64 = *var_cbfps2t_dn4_slot;
        let mut var_cbfps2t_rv: f64 = *var_cbfps2t_rv_slot;
        let mut var_cbfps3t: f64 = *var_cbfps3t_slot;
        let mut var_cbfps3t_dn4: f64 = *var_cbfps3t_dn4_slot;
        let mut var_cbfps3t_rv: f64 = *var_cbfps3t_rv_slot;
        let mut var_cbfps4t: f64 = *var_cbfps4t_slot;
        let mut var_cbfps4t_dn4: f64 = *var_cbfps4t_dn4_slot;
        let mut var_cbfps4t_rv: f64 = *var_cbfps4t_rv_slot;
        let mut var_ccfps1t: f64 = *var_ccfps1t_slot;
        let mut var_ccfps1t_dn4: f64 = *var_ccfps1t_dn4_slot;
        let mut var_ccfps1t_rv: f64 = *var_ccfps1t_rv_slot;
        let mut var_ccfps2t: f64 = *var_ccfps2t_slot;
        let mut var_ccfps2t_dn4: f64 = *var_ccfps2t_dn4_slot;
        let mut var_ccfps2t_rv: f64 = *var_ccfps2t_rv_slot;
        let mut var_ccfps3t: f64 = *var_ccfps3t_slot;
        let mut var_ccfps3t_dn4: f64 = *var_ccfps3t_dn4_slot;
        let mut var_ccfps3t_rv: f64 = *var_ccfps3t_rv_slot;
        let mut var_ccfps4t: f64 = *var_ccfps4t_slot;
        let mut var_ccfps4t_dn4: f64 = *var_ccfps4t_dn4_slot;
        let mut var_ccfps4t_rv: f64 = *var_ccfps4t_rv_slot;
        let mut var_cgfps1t: f64 = *var_cgfps1t_slot;
        let mut var_cgfps1t_dn4: f64 = *var_cgfps1t_dn4_slot;
        let mut var_cgfps1t_rv: f64 = *var_cgfps1t_rv_slot;
        let mut var_cgfps2t: f64 = *var_cgfps2t_slot;
        let mut var_cgfps2t_dn4: f64 = *var_cgfps2t_dn4_slot;
        let mut var_cgfps2t_rv: f64 = *var_cgfps2t_rv_slot;
        let mut var_cgfps3t: f64 = *var_cgfps3t_slot;
        let mut var_cgfps3t_dn4: f64 = *var_cgfps3t_dn4_slot;
        let mut var_cgfps3t_rv: f64 = *var_cgfps3t_rv_slot;
        let mut var_cgfps4t: f64 = *var_cgfps4t_slot;
        let mut var_cgfps4t_dn4: f64 = *var_cgfps4t_dn4_slot;
        let mut var_cgfps4t_rv: f64 = *var_cgfps4t_rv_slot;
        let mut var_cgt: f64 = *var_cgt_slot;
        let mut var_cgt_dn4: f64 = *var_cgt_dn4_slot;
        let mut var_cgt_rv: f64 = *var_cgt_rv_slot;
        let mut var_cofdmt: f64 = *var_cofdmt_slot;
        let mut var_cofdmt0: f64 = *var_cofdmt0_slot;
        let mut var_cofdmt0_dn4: f64 = *var_cofdmt0_dn4_slot;
        let mut var_cofdmt0_rv: f64 = *var_cofdmt0_rv_slot;
        let mut var_cofdmt_dn4: f64 = *var_cofdmt_dn4_slot;
        let mut var_cofdmt_rv: f64 = *var_cofdmt_rv_slot;
        let mut var_cofdsmt: f64 = *var_cofdsmt_slot;
        let mut var_cofdsmt0: f64 = *var_cofdsmt0_slot;
        let mut var_cofdsmt0_dn4: f64 = *var_cofdsmt0_dn4_slot;
        let mut var_cofdsmt0_rv: f64 = *var_cofdsmt0_rv_slot;
        let mut var_cofdsmt_dn4: f64 = *var_cofdsmt_dn4_slot;
        let mut var_cofdsmt_rv: f64 = *var_cofdsmt_rv_slot;
        let mut var_cofdsubmt: f64 = *var_cofdsubmt_slot;
        let mut var_cofdsubmt0: f64 = *var_cofdsubmt0_slot;
        let mut var_cofdsubmt0_dn4: f64 = *var_cofdsubmt0_dn4_slot;
        let mut var_cofdsubmt0_rv: f64 = *var_cofdsubmt0_rv_slot;
        let mut var_cofdsubmt_dn4: f64 = *var_cofdsubmt_dn4_slot;
        let mut var_cofdsubmt_rv: f64 = *var_cofdsubmt_rv_slot;
        let mut var_cofgsubmt: f64 = *var_cofgsubmt_slot;
        let mut var_cofgsubmt0: f64 = *var_cofgsubmt0_slot;
        let mut var_cofgsubmt0_dn4: f64 = *var_cofgsubmt0_dn4_slot;
        let mut var_cofgsubmt0_rv: f64 = *var_cofgsubmt0_rv_slot;
        let mut var_cofgsubmt_dn4: f64 = *var_cofgsubmt_dn4_slot;
        let mut var_cofgsubmt_rv: f64 = *var_cofgsubmt_rv_slot;
        let mut var_cofsmt: f64 = *var_cofsmt_slot;
        let mut var_cofsmt0: f64 = *var_cofsmt0_slot;
        let mut var_cofsmt0_dn4: f64 = *var_cofsmt0_dn4_slot;
        let mut var_cofsmt0_rv: f64 = *var_cofsmt0_rv_slot;
        let mut var_cofsmt_dn4: f64 = *var_cofsmt_dn4_slot;
        let mut var_cofsmt_rv: f64 = *var_cofsmt_rv_slot;
        let mut var_cofssubmt: f64 = *var_cofssubmt_slot;
        let mut var_cofssubmt0: f64 = *var_cofssubmt0_slot;
        let mut var_cofssubmt0_dn4: f64 = *var_cofssubmt0_dn4_slot;
        let mut var_cofssubmt0_rv: f64 = *var_cofssubmt0_rv_slot;
        let mut var_cofssubmt_dn4: f64 = *var_cofssubmt_dn4_slot;
        let mut var_cofssubmt_rv: f64 = *var_cofssubmt_rv_slot;
        let mut var_guard2: f64 = *var_guard2_slot;
        let mut var_guard2_rv: f64 = *var_guard2_rv_slot;
        let mut var_guard3: f64 = *var_guard3_slot;
        let mut var_guard3_rv: f64 = *var_guard3_rv_slot;
        let mut var_phit: f64 = *var_phit_slot;
        let mut var_phit_dn4: f64 = *var_phit_dn4_slot;
        let mut var_phit_rv: f64 = *var_phit_rv_slot;
        let mut var_tambk: f64 = *var_tambk_slot;
        let mut var_tambk_rv: f64 = *var_tambk_rv_slot;
        let mut var_tdut: f64 = *var_tdut_slot;
        let mut var_tdut_dn4: f64 = *var_tdut_dn4_slot;
        let mut var_tdut_rv: f64 = *var_tdut_rv_slot;
        let mut var_tnomk: f64 = *var_tnomk_slot;
        let mut var_tnomk_rv: f64 = *var_tnomk_rv_slot;
        let mut var_tsh: f64 = *var_tsh_slot;
        let mut var_tsh_dn4: f64 = *var_tsh_dn4_slot;
        let mut var_tsh_rv: f64 = *var_tsh_rv_slot;

        let assign10_e2189: f64 = (p.p5 + 273.15);
        var_tnomk = assign10_e2189;
        var_tnomk_rv = 0.0;

        let assign20_e2190: f64 = ctx_temp;
        var_tambk = assign20_e2190;
        var_tambk_rv = 0.0;

        var_tsh = (nv4 - 0.0);
        var_tsh_dn4 = 1.0;
        var_tsh_rv = 0.0;

        let assign50_e2198: f64 = (var_tambk + p.p3);
        let assign50_e2200: f64 = (assign50_e2198 + var_tsh);
        var_tdut = assign50_e2200;
        var_tdut_dn4 = var_tsh_dn4;
        var_tdut_rv = 0.0;

        let assign60_e2203: f64 = (-270.0);
        let assign60_e2205: f64 = (assign60_e2203 + 273.15);
        let assign60_e2206: f64 = if var_tdut < assign60_e2205 { 1.0 } else { 0.0 };
        var_guard2 = assign60_e2206;
        var_guard2_rv = 0.0;

        let (assign70_e2213, assign70_e2213_d_n4,) = {
    if (var_guard2 != 0.0) {
        let assign70_e2209: f64 = (-270.0);
        let assign70_e2211: f64 = (assign70_e2209 + 273.15);
        (assign70_e2211, 0.0,)
    } else {
        (var_tdut, var_tdut_dn4,)
    }
};
        var_tdut = assign70_e2213;
        var_tdut_dn4 = assign70_e2213_d_n4;
        var_tdut_rv = 0.0;

        let assign80_e2217: f64 = (1500.0 + 273.15);
        let assign80_e2218: f64 = if var_tdut > assign80_e2217 { 1.0 } else { 0.0 };
        var_guard3 = assign80_e2218;
        var_guard3_rv = 0.0;

        let (assign90_e2227, assign90_e2227_d_n4,) = {
    if ((var_guard2 == 0.0) && (var_guard3 != 0.0)) {
        let assign90_e2225: f64 = (1500.0 + 273.15);
        (assign90_e2225, 0.0,)
    } else {
        (var_tdut, var_tdut_dn4,)
    }
};
        var_tdut = assign90_e2227;
        var_tdut_dn4 = assign90_e2227_d_n4;
        var_tdut_rv = 0.0;

        let assign290_e2401: f64 = (1.38062e-23 * var_tdut);
        let assign290_e2403: f64 = (assign290_e2401 / 1.60219e-19);
        var_phit = assign290_e2403;
        var_phit_dn4 = ((1.38062e-23 * var_tdut_dn4) / 1.60219e-19);
        var_phit_rv = 0.0;

        let assign340_e2428: f64 = (var_tdut - var_tnomk);
        let assign340_e2429: f64 = (p.p21 * assign340_e2428);
        let assign340_e2430: f64 = (1.0 + assign340_e2429);
        let (assign340_e2441, assign340_e2441_d_n4,) = {
    if (assign340_e2430 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign340_e2438: f64 = (var_tdut - var_tnomk);
        let assign340_e2439: f64 = (p.p21 * assign340_e2438);
        let assign340_e2440: f64 = (1.0 + assign340_e2439);
        (assign340_e2440, (p.p21 * var_tdut_dn4),)
    }
};
        let assign340_e2442: f64 = (p.p9 * assign340_e2441);
        var_cofsmt = assign340_e2442;
        var_cofsmt_dn4 = (p.p9 * assign340_e2441_d_n4);
        var_cofsmt_rv = 0.0;

        let assign350_e2448: f64 = (var_tdut - var_tnomk);
        let assign350_e2449: f64 = (p.p22 * assign350_e2448);
        let assign350_e2450: f64 = (1.0 + assign350_e2449);
        let (assign350_e2461, assign350_e2461_d_n4,) = {
    if (assign350_e2450 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign350_e2458: f64 = (var_tdut - var_tnomk);
        let assign350_e2459: f64 = (p.p22 * assign350_e2458);
        let assign350_e2460: f64 = (1.0 + assign350_e2459);
        (assign350_e2460, (p.p22 * var_tdut_dn4),)
    }
};
        let assign350_e2462: f64 = (p.p10 * assign350_e2461);
        var_cofdmt = assign350_e2462;
        var_cofdmt_dn4 = (p.p10 * assign350_e2461_d_n4);
        var_cofdmt_rv = 0.0;

        let assign360_e2468: f64 = (var_tdut - var_tnomk);
        let assign360_e2469: f64 = (p.p23 * assign360_e2468);
        let assign360_e2470: f64 = (1.0 + assign360_e2469);
        let (assign360_e2481, assign360_e2481_d_n4,) = {
    if (assign360_e2470 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign360_e2478: f64 = (var_tdut - var_tnomk);
        let assign360_e2479: f64 = (p.p23 * assign360_e2478);
        let assign360_e2480: f64 = (1.0 + assign360_e2479);
        (assign360_e2480, (p.p23 * var_tdut_dn4),)
    }
};
        let assign360_e2482: f64 = (p.p11 * assign360_e2481);
        var_cofdsmt = assign360_e2482;
        var_cofdsmt_dn4 = (p.p11 * assign360_e2481_d_n4);
        var_cofdsmt_rv = 0.0;

        let assign370_e2488: f64 = (var_tdut - var_tnomk);
        let assign370_e2489: f64 = (p.p24 * assign370_e2488);
        let assign370_e2490: f64 = (1.0 + assign370_e2489);
        let (assign370_e2501, assign370_e2501_d_n4,) = {
    if (assign370_e2490 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign370_e2498: f64 = (var_tdut - var_tnomk);
        let assign370_e2499: f64 = (p.p24 * assign370_e2498);
        let assign370_e2500: f64 = (1.0 + assign370_e2499);
        (assign370_e2500, (p.p24 * var_tdut_dn4),)
    }
};
        let assign370_e2502: f64 = (p.p13 * assign370_e2501);
        var_cofssubmt = assign370_e2502;
        var_cofssubmt_dn4 = (p.p13 * assign370_e2501_d_n4);
        var_cofssubmt_rv = 0.0;

        let assign380_e2508: f64 = (var_tdut - var_tnomk);
        let assign380_e2509: f64 = (p.p25 * assign380_e2508);
        let assign380_e2510: f64 = (1.0 + assign380_e2509);
        let (assign380_e2521, assign380_e2521_d_n4,) = {
    if (assign380_e2510 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign380_e2518: f64 = (var_tdut - var_tnomk);
        let assign380_e2519: f64 = (p.p25 * assign380_e2518);
        let assign380_e2520: f64 = (1.0 + assign380_e2519);
        (assign380_e2520, (p.p25 * var_tdut_dn4),)
    }
};
        let assign380_e2522: f64 = (p.p12 * assign380_e2521);
        var_cofdsubmt = assign380_e2522;
        var_cofdsubmt_dn4 = (p.p12 * assign380_e2521_d_n4);
        var_cofdsubmt_rv = 0.0;

        let assign390_e2528: f64 = (var_tdut - var_tnomk);
        let assign390_e2529: f64 = (p.p26 * assign390_e2528);
        let assign390_e2530: f64 = (1.0 + assign390_e2529);
        let (assign390_e2541, assign390_e2541_d_n4,) = {
    if (assign390_e2530 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign390_e2538: f64 = (var_tdut - var_tnomk);
        let assign390_e2539: f64 = (p.p26 * assign390_e2538);
        let assign390_e2540: f64 = (1.0 + assign390_e2539);
        (assign390_e2540, (p.p26 * var_tdut_dn4),)
    }
};
        let assign390_e2542: f64 = (p.p14 * assign390_e2541);
        var_cofgsubmt = assign390_e2542;
        var_cofgsubmt_dn4 = (p.p14 * assign390_e2541_d_n4);
        var_cofgsubmt_rv = 0.0;

        let assign400_e2548: f64 = (var_tdut - var_tnomk);
        let assign400_e2549: f64 = (p.p21 * assign400_e2548);
        let assign400_e2550: f64 = (1.0 + assign400_e2549);
        let (assign400_e2561, assign400_e2561_d_n4,) = {
    if (assign400_e2550 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign400_e2558: f64 = (var_tdut - var_tnomk);
        let assign400_e2559: f64 = (p.p21 * assign400_e2558);
        let assign400_e2560: f64 = (1.0 + assign400_e2559);
        (assign400_e2560, (p.p21 * var_tdut_dn4),)
    }
};
        let assign400_e2562: f64 = (p.p15 * assign400_e2561);
        var_cofsmt0 = assign400_e2562;
        var_cofsmt0_dn4 = (p.p15 * assign400_e2561_d_n4);
        var_cofsmt0_rv = 0.0;

        let assign410_e2568: f64 = (var_tdut - var_tnomk);
        let assign410_e2569: f64 = (p.p22 * assign410_e2568);
        let assign410_e2570: f64 = (1.0 + assign410_e2569);
        let (assign410_e2581, assign410_e2581_d_n4,) = {
    if (assign410_e2570 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign410_e2578: f64 = (var_tdut - var_tnomk);
        let assign410_e2579: f64 = (p.p22 * assign410_e2578);
        let assign410_e2580: f64 = (1.0 + assign410_e2579);
        (assign410_e2580, (p.p22 * var_tdut_dn4),)
    }
};
        let assign410_e2582: f64 = (p.p16 * assign410_e2581);
        var_cofdmt0 = assign410_e2582;
        var_cofdmt0_dn4 = (p.p16 * assign410_e2581_d_n4);
        var_cofdmt0_rv = 0.0;

        let assign420_e2588: f64 = (var_tdut - var_tnomk);
        let assign420_e2589: f64 = (p.p23 * assign420_e2588);
        let assign420_e2590: f64 = (1.0 + assign420_e2589);
        let (assign420_e2601, assign420_e2601_d_n4,) = {
    if (assign420_e2590 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign420_e2598: f64 = (var_tdut - var_tnomk);
        let assign420_e2599: f64 = (p.p23 * assign420_e2598);
        let assign420_e2600: f64 = (1.0 + assign420_e2599);
        (assign420_e2600, (p.p23 * var_tdut_dn4),)
    }
};
        let assign420_e2602: f64 = (p.p17 * assign420_e2601);
        var_cofdsmt0 = assign420_e2602;
        var_cofdsmt0_dn4 = (p.p17 * assign420_e2601_d_n4);
        var_cofdsmt0_rv = 0.0;

        let assign430_e2608: f64 = (var_tdut - var_tnomk);
        let assign430_e2609: f64 = (p.p24 * assign430_e2608);
        let assign430_e2610: f64 = (1.0 + assign430_e2609);
        let (assign430_e2621, assign430_e2621_d_n4,) = {
    if (assign430_e2610 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign430_e2618: f64 = (var_tdut - var_tnomk);
        let assign430_e2619: f64 = (p.p24 * assign430_e2618);
        let assign430_e2620: f64 = (1.0 + assign430_e2619);
        (assign430_e2620, (p.p24 * var_tdut_dn4),)
    }
};
        let assign430_e2622: f64 = (p.p19 * assign430_e2621);
        var_cofssubmt0 = assign430_e2622;
        var_cofssubmt0_dn4 = (p.p19 * assign430_e2621_d_n4);
        var_cofssubmt0_rv = 0.0;

        let assign440_e2628: f64 = (var_tdut - var_tnomk);
        let assign440_e2629: f64 = (p.p25 * assign440_e2628);
        let assign440_e2630: f64 = (1.0 + assign440_e2629);
        let (assign440_e2641, assign440_e2641_d_n4,) = {
    if (assign440_e2630 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign440_e2638: f64 = (var_tdut - var_tnomk);
        let assign440_e2639: f64 = (p.p25 * assign440_e2638);
        let assign440_e2640: f64 = (1.0 + assign440_e2639);
        (assign440_e2640, (p.p25 * var_tdut_dn4),)
    }
};
        let assign440_e2642: f64 = (p.p18 * assign440_e2641);
        var_cofdsubmt0 = assign440_e2642;
        var_cofdsubmt0_dn4 = (p.p18 * assign440_e2641_d_n4);
        var_cofdsubmt0_rv = 0.0;

        let assign450_e2648: f64 = (var_tdut - var_tnomk);
        let assign450_e2649: f64 = (p.p26 * assign450_e2648);
        let assign450_e2650: f64 = (1.0 + assign450_e2649);
        let (assign450_e2661, assign450_e2661_d_n4,) = {
    if (assign450_e2650 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign450_e2658: f64 = (var_tdut - var_tnomk);
        let assign450_e2659: f64 = (p.p26 * assign450_e2658);
        let assign450_e2660: f64 = (1.0 + assign450_e2659);
        (assign450_e2660, (p.p26 * var_tdut_dn4),)
    }
};
        let assign450_e2662: f64 = (p.p20 * assign450_e2661);
        var_cofgsubmt0 = assign450_e2662;
        var_cofgsubmt0_dn4 = (p.p20 * assign450_e2661_d_n4);
        var_cofgsubmt0_rv = 0.0;

        let assign460_e2668: f64 = (var_tdut - var_tnomk);
        let assign460_e2669: f64 = (p.p8 * assign460_e2668);
        let assign460_e2670: f64 = (1.0 + assign460_e2669);
        let (assign460_e2681, assign460_e2681_d_n4,) = {
    if (assign460_e2670 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign460_e2678: f64 = (var_tdut - var_tnomk);
        let assign460_e2679: f64 = (p.p8 * assign460_e2678);
        let assign460_e2680: f64 = (1.0 + assign460_e2679);
        (assign460_e2680, (p.p8 * var_tdut_dn4),)
    }
};
        let assign460_e2682: f64 = (p.p7 * assign460_e2681);
        var_cgt = assign460_e2682;
        var_cgt_dn4 = (p.p7 * assign460_e2681_d_n4);
        var_cgt_rv = 0.0;

        let assign470_e2688: f64 = (var_tdut - var_tnomk);
        let assign470_e2689: f64 = (p.p82 * assign470_e2688);
        let assign470_e2690: f64 = (1.0 + assign470_e2689);
        let (assign470_e2701, assign470_e2701_d_n4,) = {
    if (assign470_e2690 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign470_e2698: f64 = (var_tdut - var_tnomk);
        let assign470_e2699: f64 = (p.p82 * assign470_e2698);
        let assign470_e2700: f64 = (1.0 + assign470_e2699);
        (assign470_e2700, (p.p82 * var_tdut_dn4),)
    }
};
        let assign470_e2702: f64 = (p.p81 * assign470_e2701);
        var_cgfps1t = assign470_e2702;
        var_cgfps1t_dn4 = (p.p81 * assign470_e2701_d_n4);
        var_cgfps1t_rv = 0.0;

        let assign480_e2708: f64 = (var_tdut - var_tnomk);
        let assign480_e2709: f64 = (p.p104 * assign480_e2708);
        let assign480_e2710: f64 = (1.0 + assign480_e2709);
        let (assign480_e2721, assign480_e2721_d_n4,) = {
    if (assign480_e2710 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign480_e2718: f64 = (var_tdut - var_tnomk);
        let assign480_e2719: f64 = (p.p104 * assign480_e2718);
        let assign480_e2720: f64 = (1.0 + assign480_e2719);
        (assign480_e2720, (p.p104 * var_tdut_dn4),)
    }
};
        let assign480_e2722: f64 = (p.p103 * assign480_e2721);
        var_cgfps2t = assign480_e2722;
        var_cgfps2t_dn4 = (p.p103 * assign480_e2721_d_n4);
        var_cgfps2t_rv = 0.0;

        let assign490_e2728: f64 = (var_tdut - var_tnomk);
        let assign490_e2729: f64 = (p.p126 * assign490_e2728);
        let assign490_e2730: f64 = (1.0 + assign490_e2729);
        let (assign490_e2741, assign490_e2741_d_n4,) = {
    if (assign490_e2730 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign490_e2738: f64 = (var_tdut - var_tnomk);
        let assign490_e2739: f64 = (p.p126 * assign490_e2738);
        let assign490_e2740: f64 = (1.0 + assign490_e2739);
        (assign490_e2740, (p.p126 * var_tdut_dn4),)
    }
};
        let assign490_e2742: f64 = (p.p125 * assign490_e2741);
        var_cgfps3t = assign490_e2742;
        var_cgfps3t_dn4 = (p.p125 * assign490_e2741_d_n4);
        var_cgfps3t_rv = 0.0;

        let assign500_e2748: f64 = (var_tdut - var_tnomk);
        let assign500_e2749: f64 = (p.p148 * assign500_e2748);
        let assign500_e2750: f64 = (1.0 + assign500_e2749);
        let (assign500_e2761, assign500_e2761_d_n4,) = {
    if (assign500_e2750 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign500_e2758: f64 = (var_tdut - var_tnomk);
        let assign500_e2759: f64 = (p.p148 * assign500_e2758);
        let assign500_e2760: f64 = (1.0 + assign500_e2759);
        (assign500_e2760, (p.p148 * var_tdut_dn4),)
    }
};
        let assign500_e2762: f64 = (p.p147 * assign500_e2761);
        var_cgfps4t = assign500_e2762;
        var_cgfps4t_dn4 = (p.p147 * assign500_e2761_d_n4);
        var_cgfps4t_rv = 0.0;

        let assign510_e2768: f64 = (var_tdut - var_tnomk);
        let assign510_e2769: f64 = (p.p87 * assign510_e2768);
        let assign510_e2770: f64 = (1.0 + assign510_e2769);
        let (assign510_e2781, assign510_e2781_d_n4,) = {
    if (assign510_e2770 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign510_e2778: f64 = (var_tdut - var_tnomk);
        let assign510_e2779: f64 = (p.p87 * assign510_e2778);
        let assign510_e2780: f64 = (1.0 + assign510_e2779);
        (assign510_e2780, (p.p87 * var_tdut_dn4),)
    }
};
        let assign510_e2782: f64 = (p.p86 * assign510_e2781);
        var_ccfps1t = assign510_e2782;
        var_ccfps1t_dn4 = (p.p86 * assign510_e2781_d_n4);
        var_ccfps1t_rv = 0.0;

        let assign520_e2788: f64 = (var_tdut - var_tnomk);
        let assign520_e2789: f64 = (p.p109 * assign520_e2788);
        let assign520_e2790: f64 = (1.0 + assign520_e2789);
        let (assign520_e2801, assign520_e2801_d_n4,) = {
    if (assign520_e2790 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign520_e2798: f64 = (var_tdut - var_tnomk);
        let assign520_e2799: f64 = (p.p109 * assign520_e2798);
        let assign520_e2800: f64 = (1.0 + assign520_e2799);
        (assign520_e2800, (p.p109 * var_tdut_dn4),)
    }
};
        let assign520_e2802: f64 = (p.p108 * assign520_e2801);
        var_ccfps2t = assign520_e2802;
        var_ccfps2t_dn4 = (p.p108 * assign520_e2801_d_n4);
        var_ccfps2t_rv = 0.0;

        let assign530_e2808: f64 = (var_tdut - var_tnomk);
        let assign530_e2809: f64 = (p.p131 * assign530_e2808);
        let assign530_e2810: f64 = (1.0 + assign530_e2809);
        let (assign530_e2821, assign530_e2821_d_n4,) = {
    if (assign530_e2810 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign530_e2818: f64 = (var_tdut - var_tnomk);
        let assign530_e2819: f64 = (p.p131 * assign530_e2818);
        let assign530_e2820: f64 = (1.0 + assign530_e2819);
        (assign530_e2820, (p.p131 * var_tdut_dn4),)
    }
};
        let assign530_e2822: f64 = (p.p130 * assign530_e2821);
        var_ccfps3t = assign530_e2822;
        var_ccfps3t_dn4 = (p.p130 * assign530_e2821_d_n4);
        var_ccfps3t_rv = 0.0;

        let assign540_e2828: f64 = (var_tdut - var_tnomk);
        let assign540_e2829: f64 = (p.p153 * assign540_e2828);
        let assign540_e2830: f64 = (1.0 + assign540_e2829);
        let (assign540_e2841, assign540_e2841_d_n4,) = {
    if (assign540_e2830 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign540_e2838: f64 = (var_tdut - var_tnomk);
        let assign540_e2839: f64 = (p.p153 * assign540_e2838);
        let assign540_e2840: f64 = (1.0 + assign540_e2839);
        (assign540_e2840, (p.p153 * var_tdut_dn4),)
    }
};
        let assign540_e2842: f64 = (p.p152 * assign540_e2841);
        var_ccfps4t = assign540_e2842;
        var_ccfps4t_dn4 = (p.p152 * assign540_e2841_d_n4);
        var_ccfps4t_rv = 0.0;

        let assign550_e2848: f64 = (var_tdut - var_tnomk);
        let assign550_e2849: f64 = (p.p89 * assign550_e2848);
        let assign550_e2850: f64 = (1.0 + assign550_e2849);
        let (assign550_e2861, assign550_e2861_d_n4,) = {
    if (assign550_e2850 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign550_e2858: f64 = (var_tdut - var_tnomk);
        let assign550_e2859: f64 = (p.p89 * assign550_e2858);
        let assign550_e2860: f64 = (1.0 + assign550_e2859);
        (assign550_e2860, (p.p89 * var_tdut_dn4),)
    }
};
        let assign550_e2862: f64 = (p.p88 * assign550_e2861);
        var_cbfps1t = assign550_e2862;
        var_cbfps1t_dn4 = (p.p88 * assign550_e2861_d_n4);
        var_cbfps1t_rv = 0.0;

        let assign560_e2868: f64 = (var_tdut - var_tnomk);
        let assign560_e2869: f64 = (p.p111 * assign560_e2868);
        let assign560_e2870: f64 = (1.0 + assign560_e2869);
        let (assign560_e2881, assign560_e2881_d_n4,) = {
    if (assign560_e2870 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign560_e2878: f64 = (var_tdut - var_tnomk);
        let assign560_e2879: f64 = (p.p111 * assign560_e2878);
        let assign560_e2880: f64 = (1.0 + assign560_e2879);
        (assign560_e2880, (p.p111 * var_tdut_dn4),)
    }
};
        let assign560_e2882: f64 = (p.p110 * assign560_e2881);
        var_cbfps2t = assign560_e2882;
        var_cbfps2t_dn4 = (p.p110 * assign560_e2881_d_n4);
        var_cbfps2t_rv = 0.0;

        let assign570_e2888: f64 = (var_tdut - var_tnomk);
        let assign570_e2889: f64 = (p.p133 * assign570_e2888);
        let assign570_e2890: f64 = (1.0 + assign570_e2889);
        let (assign570_e2901, assign570_e2901_d_n4,) = {
    if (assign570_e2890 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign570_e2898: f64 = (var_tdut - var_tnomk);
        let assign570_e2899: f64 = (p.p133 * assign570_e2898);
        let assign570_e2900: f64 = (1.0 + assign570_e2899);
        (assign570_e2900, (p.p133 * var_tdut_dn4),)
    }
};
        let assign570_e2902: f64 = (p.p132 * assign570_e2901);
        var_cbfps3t = assign570_e2902;
        var_cbfps3t_dn4 = (p.p132 * assign570_e2901_d_n4);
        var_cbfps3t_rv = 0.0;

        let assign580_e2908: f64 = (var_tdut - var_tnomk);
        let assign580_e2909: f64 = (p.p155 * assign580_e2908);
        let assign580_e2910: f64 = (1.0 + assign580_e2909);
        let (assign580_e2921, assign580_e2921_d_n4,) = {
    if (assign580_e2910 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign580_e2918: f64 = (var_tdut - var_tnomk);
        let assign580_e2919: f64 = (p.p155 * assign580_e2918);
        let assign580_e2920: f64 = (1.0 + assign580_e2919);
        (assign580_e2920, (p.p155 * var_tdut_dn4),)
    }
};
        let assign580_e2922: f64 = (p.p154 * assign580_e2921);
        var_cbfps4t = assign580_e2922;
        var_cbfps4t_dn4 = (p.p154 * assign580_e2921_d_n4);
        var_cbfps4t_rv = 0.0;

        *var_cbfps1t_slot = var_cbfps1t;
        *var_cbfps1t_dn4_slot = var_cbfps1t_dn4;
        *var_cbfps1t_rv_slot = var_cbfps1t_rv;
        *var_cbfps2t_slot = var_cbfps2t;
        *var_cbfps2t_dn4_slot = var_cbfps2t_dn4;
        *var_cbfps2t_rv_slot = var_cbfps2t_rv;
        *var_cbfps3t_slot = var_cbfps3t;
        *var_cbfps3t_dn4_slot = var_cbfps3t_dn4;
        *var_cbfps3t_rv_slot = var_cbfps3t_rv;
        *var_cbfps4t_slot = var_cbfps4t;
        *var_cbfps4t_dn4_slot = var_cbfps4t_dn4;
        *var_cbfps4t_rv_slot = var_cbfps4t_rv;
        *var_ccfps1t_slot = var_ccfps1t;
        *var_ccfps1t_dn4_slot = var_ccfps1t_dn4;
        *var_ccfps1t_rv_slot = var_ccfps1t_rv;
        *var_ccfps2t_slot = var_ccfps2t;
        *var_ccfps2t_dn4_slot = var_ccfps2t_dn4;
        *var_ccfps2t_rv_slot = var_ccfps2t_rv;
        *var_ccfps3t_slot = var_ccfps3t;
        *var_ccfps3t_dn4_slot = var_ccfps3t_dn4;
        *var_ccfps3t_rv_slot = var_ccfps3t_rv;
        *var_ccfps4t_slot = var_ccfps4t;
        *var_ccfps4t_dn4_slot = var_ccfps4t_dn4;
        *var_ccfps4t_rv_slot = var_ccfps4t_rv;
        *var_cgfps1t_slot = var_cgfps1t;
        *var_cgfps1t_dn4_slot = var_cgfps1t_dn4;
        *var_cgfps1t_rv_slot = var_cgfps1t_rv;
        *var_cgfps2t_slot = var_cgfps2t;
        *var_cgfps2t_dn4_slot = var_cgfps2t_dn4;
        *var_cgfps2t_rv_slot = var_cgfps2t_rv;
        *var_cgfps3t_slot = var_cgfps3t;
        *var_cgfps3t_dn4_slot = var_cgfps3t_dn4;
        *var_cgfps3t_rv_slot = var_cgfps3t_rv;
        *var_cgfps4t_slot = var_cgfps4t;
        *var_cgfps4t_dn4_slot = var_cgfps4t_dn4;
        *var_cgfps4t_rv_slot = var_cgfps4t_rv;
        *var_cgt_slot = var_cgt;
        *var_cgt_dn4_slot = var_cgt_dn4;
        *var_cgt_rv_slot = var_cgt_rv;
        *var_cofdmt_slot = var_cofdmt;
        *var_cofdmt0_slot = var_cofdmt0;
        *var_cofdmt0_dn4_slot = var_cofdmt0_dn4;
        *var_cofdmt0_rv_slot = var_cofdmt0_rv;
        *var_cofdmt_dn4_slot = var_cofdmt_dn4;
        *var_cofdmt_rv_slot = var_cofdmt_rv;
        *var_cofdsmt_slot = var_cofdsmt;
        *var_cofdsmt0_slot = var_cofdsmt0;
        *var_cofdsmt0_dn4_slot = var_cofdsmt0_dn4;
        *var_cofdsmt0_rv_slot = var_cofdsmt0_rv;
        *var_cofdsmt_dn4_slot = var_cofdsmt_dn4;
        *var_cofdsmt_rv_slot = var_cofdsmt_rv;
        *var_cofdsubmt_slot = var_cofdsubmt;
        *var_cofdsubmt0_slot = var_cofdsubmt0;
        *var_cofdsubmt0_dn4_slot = var_cofdsubmt0_dn4;
        *var_cofdsubmt0_rv_slot = var_cofdsubmt0_rv;
        *var_cofdsubmt_dn4_slot = var_cofdsubmt_dn4;
        *var_cofdsubmt_rv_slot = var_cofdsubmt_rv;
        *var_cofgsubmt_slot = var_cofgsubmt;
        *var_cofgsubmt0_slot = var_cofgsubmt0;
        *var_cofgsubmt0_dn4_slot = var_cofgsubmt0_dn4;
        *var_cofgsubmt0_rv_slot = var_cofgsubmt0_rv;
        *var_cofgsubmt_dn4_slot = var_cofgsubmt_dn4;
        *var_cofgsubmt_rv_slot = var_cofgsubmt_rv;
        *var_cofsmt_slot = var_cofsmt;
        *var_cofsmt0_slot = var_cofsmt0;
        *var_cofsmt0_dn4_slot = var_cofsmt0_dn4;
        *var_cofsmt0_rv_slot = var_cofsmt0_rv;
        *var_cofsmt_dn4_slot = var_cofsmt_dn4;
        *var_cofsmt_rv_slot = var_cofsmt_rv;
        *var_cofssubmt_slot = var_cofssubmt;
        *var_cofssubmt0_slot = var_cofssubmt0;
        *var_cofssubmt0_dn4_slot = var_cofssubmt0_dn4;
        *var_cofssubmt0_rv_slot = var_cofssubmt0_rv;
        *var_cofssubmt_dn4_slot = var_cofssubmt_dn4;
        *var_cofssubmt_rv_slot = var_cofssubmt_rv;
        *var_guard2_slot = var_guard2;
        *var_guard2_rv_slot = var_guard2_rv;
        *var_guard3_slot = var_guard3;
        *var_guard3_rv_slot = var_guard3_rv;
        *var_phit_slot = var_phit;
        *var_phit_dn4_slot = var_phit_dn4;
        *var_phit_rv_slot = var_phit_rv;
        *var_tambk_slot = var_tambk;
        *var_tambk_rv_slot = var_tambk_rv;
        *var_tdut_slot = var_tdut;
        *var_tdut_dn4_slot = var_tdut_dn4;
        *var_tdut_rv_slot = var_tdut_rv;
        *var_tnomk_slot = var_tnomk;
        *var_tnomk_rv_slot = var_tnomk_rv;
        *var_tsh_slot = var_tsh;
        *var_tsh_dn4_slot = var_tsh_dn4;
        *var_tsh_rv_slot = var_tsh_rv;
    }

    pub(super) fn stamp_reactive_block_1(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_tdut: f64,
        var_tdut_dn4: f64,
        var_tnomk: f64,
        var_cbfp1t_slot: &mut f64,
        var_cbfp1t_dn4_slot: &mut f64,
        var_cbfp1t_rv_slot: &mut f64,
        var_cbfp2t_slot: &mut f64,
        var_cbfp2t_dn4_slot: &mut f64,
        var_cbfp2t_rv_slot: &mut f64,
        var_cbfp3t_slot: &mut f64,
        var_cbfp3t_dn4_slot: &mut f64,
        var_cbfp3t_rv_slot: &mut f64,
        var_cbfp4t_slot: &mut f64,
        var_cbfp4t_dn4_slot: &mut f64,
        var_cbfp4t_rv_slot: &mut f64,
        var_ccfp1t_slot: &mut f64,
        var_ccfp1t_dn4_slot: &mut f64,
        var_ccfp1t_rv_slot: &mut f64,
        var_ccfp2t_slot: &mut f64,
        var_ccfp2t_dn4_slot: &mut f64,
        var_ccfp2t_rv_slot: &mut f64,
        var_ccfp3t_slot: &mut f64,
        var_ccfp3t_dn4_slot: &mut f64,
        var_ccfp3t_rv_slot: &mut f64,
        var_ccfp4t_slot: &mut f64,
        var_ccfp4t_dn4_slot: &mut f64,
        var_ccfp4t_rv_slot: &mut f64,
        var_cgfp1t_slot: &mut f64,
        var_cgfp1t_dn4_slot: &mut f64,
        var_cgfp1t_rv_slot: &mut f64,
        var_cgfp2t_slot: &mut f64,
        var_cgfp2t_dn4_slot: &mut f64,
        var_cgfp2t_rv_slot: &mut f64,
        var_cgfp3t_slot: &mut f64,
        var_cgfp3t_dn4_slot: &mut f64,
        var_cgfp3t_rv_slot: &mut f64,
        var_cgfp4t_slot: &mut f64,
        var_cgfp4t_dn4_slot: &mut f64,
        var_cgfp4t_rv_slot: &mut f64,
        var_chargefrac_slot: &mut f64,
        var_chargefrac_dn22_slot: &mut f64,
        var_chargefrac_dn23_slot: &mut f64,
        var_chargefrac_dn25_slot: &mut f64,
        var_chargefrac_dn26_slot: &mut f64,
        var_chargefrac_rv_slot: &mut f64,
        var_chargefracd_slot: &mut f64,
        var_chargefracd_dn22_slot: &mut f64,
        var_chargefracd_dn23_slot: &mut f64,
        var_chargefracd_rv_slot: &mut f64,
        var_chargefracg_slot: &mut f64,
        var_chargefracg_dn25_slot: &mut f64,
        var_chargefracg_dn26_slot: &mut f64,
        var_chargefracg_rv_slot: &mut f64,
        var_guard12_slot: &mut f64,
        var_guard12_rv_slot: &mut f64,
        var_guard13_slot: &mut f64,
        var_guard13_rv_slot: &mut f64,
        var_guard16_slot: &mut f64,
        var_guard16_rv_slot: &mut f64,
        var_guard17_slot: &mut f64,
        var_guard17_rv_slot: &mut f64,
        var_guard18_slot: &mut f64,
        var_guard18_rv_slot: &mut f64,
        var_vbfps1_slot: &mut f64,
        var_vbfps1_dn10_slot: &mut f64,
        var_vbfps1_dn3_slot: &mut f64,
        var_vbfps1_rv_slot: &mut f64,
        var_vbfps2_slot: &mut f64,
        var_vbfps2_dn11_slot: &mut f64,
        var_vbfps2_dn3_slot: &mut f64,
        var_vbfps2_rv_slot: &mut f64,
        var_vcfps1_slot: &mut f64,
        var_vcfps1_dn10_slot: &mut f64,
        var_vcfps1_dn2_slot: &mut f64,
        var_vcfps1_dn7_slot: &mut f64,
        var_vcfps1_rv_slot: &mut f64,
        var_vcfps2_slot: &mut f64,
        var_vcfps2_dn11_slot: &mut f64,
        var_vcfps2_dn2_slot: &mut f64,
        var_vcfps2_dn7_slot: &mut f64,
        var_vcfps2_rv_slot: &mut f64,
        var_vdlinput_slot: &mut f64,
        var_vdlinput_dn22_slot: &mut f64,
        var_vdlinput_rv_slot: &mut f64,
        var_vdloutput_slot: &mut f64,
        var_vdloutput_dn23_slot: &mut f64,
        var_vdloutput_rv_slot: &mut f64,
        var_vdsfps1_slot: &mut f64,
        var_vdsfps1_dn10_slot: &mut f64,
        var_vdsfps1_dn9_slot: &mut f64,
        var_vdsfps1_rv_slot: &mut f64,
        var_vdsfps2_slot: &mut f64,
        var_vdsfps2_dn10_slot: &mut f64,
        var_vdsfps2_dn11_slot: &mut f64,
        var_vdsfps2_rv_slot: &mut f64,
        var_vdsi_slot: &mut f64,
        var_vdsi_dn5_slot: &mut f64,
        var_vdsi_dn9_slot: &mut f64,
        var_vdsi_rv_slot: &mut f64,
        var_vglinput_slot: &mut f64,
        var_vglinput_dn25_slot: &mut f64,
        var_vglinput_rv_slot: &mut f64,
        var_vgloutput_slot: &mut f64,
        var_vgloutput_dn26_slot: &mut f64,
        var_vgloutput_rv_slot: &mut f64,
        var_vgsfps1_slot: &mut f64,
        var_vgsfps1_dn10_slot: &mut f64,
        var_vgsfps1_dn2_slot: &mut f64,
        var_vgsfps1_dn7_slot: &mut f64,
        var_vgsfps1_rv_slot: &mut f64,
        var_vgsfps2_slot: &mut f64,
        var_vgsfps2_dn11_slot: &mut f64,
        var_vgsfps2_dn2_slot: &mut f64,
        var_vgsfps2_dn7_slot: &mut f64,
        var_vgsfps2_rv_slot: &mut f64,
        var_vgsi_slot: &mut f64,
        var_vgsi_dn8_slot: &mut f64,
        var_vgsi_dn9_slot: &mut f64,
        var_vgsi_rv_slot: &mut f64,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv22 = ctx.node_voltage(nodes[22]);
        let nv23 = ctx.node_voltage(nodes[23]);
        let nv25 = ctx.node_voltage(nodes[25]);
        let nv26 = ctx.node_voltage(nodes[26]);
        let mut var_cbfp1t: f64 = *var_cbfp1t_slot;
        let mut var_cbfp1t_dn4: f64 = *var_cbfp1t_dn4_slot;
        let mut var_cbfp1t_rv: f64 = *var_cbfp1t_rv_slot;
        let mut var_cbfp2t: f64 = *var_cbfp2t_slot;
        let mut var_cbfp2t_dn4: f64 = *var_cbfp2t_dn4_slot;
        let mut var_cbfp2t_rv: f64 = *var_cbfp2t_rv_slot;
        let mut var_cbfp3t: f64 = *var_cbfp3t_slot;
        let mut var_cbfp3t_dn4: f64 = *var_cbfp3t_dn4_slot;
        let mut var_cbfp3t_rv: f64 = *var_cbfp3t_rv_slot;
        let mut var_cbfp4t: f64 = *var_cbfp4t_slot;
        let mut var_cbfp4t_dn4: f64 = *var_cbfp4t_dn4_slot;
        let mut var_cbfp4t_rv: f64 = *var_cbfp4t_rv_slot;
        let mut var_ccfp1t: f64 = *var_ccfp1t_slot;
        let mut var_ccfp1t_dn4: f64 = *var_ccfp1t_dn4_slot;
        let mut var_ccfp1t_rv: f64 = *var_ccfp1t_rv_slot;
        let mut var_ccfp2t: f64 = *var_ccfp2t_slot;
        let mut var_ccfp2t_dn4: f64 = *var_ccfp2t_dn4_slot;
        let mut var_ccfp2t_rv: f64 = *var_ccfp2t_rv_slot;
        let mut var_ccfp3t: f64 = *var_ccfp3t_slot;
        let mut var_ccfp3t_dn4: f64 = *var_ccfp3t_dn4_slot;
        let mut var_ccfp3t_rv: f64 = *var_ccfp3t_rv_slot;
        let mut var_ccfp4t: f64 = *var_ccfp4t_slot;
        let mut var_ccfp4t_dn4: f64 = *var_ccfp4t_dn4_slot;
        let mut var_ccfp4t_rv: f64 = *var_ccfp4t_rv_slot;
        let mut var_cgfp1t: f64 = *var_cgfp1t_slot;
        let mut var_cgfp1t_dn4: f64 = *var_cgfp1t_dn4_slot;
        let mut var_cgfp1t_rv: f64 = *var_cgfp1t_rv_slot;
        let mut var_cgfp2t: f64 = *var_cgfp2t_slot;
        let mut var_cgfp2t_dn4: f64 = *var_cgfp2t_dn4_slot;
        let mut var_cgfp2t_rv: f64 = *var_cgfp2t_rv_slot;
        let mut var_cgfp3t: f64 = *var_cgfp3t_slot;
        let mut var_cgfp3t_dn4: f64 = *var_cgfp3t_dn4_slot;
        let mut var_cgfp3t_rv: f64 = *var_cgfp3t_rv_slot;
        let mut var_cgfp4t: f64 = *var_cgfp4t_slot;
        let mut var_cgfp4t_dn4: f64 = *var_cgfp4t_dn4_slot;
        let mut var_cgfp4t_rv: f64 = *var_cgfp4t_rv_slot;
        let mut var_chargefrac: f64 = *var_chargefrac_slot;
        let mut var_chargefrac_dn22: f64 = *var_chargefrac_dn22_slot;
        let mut var_chargefrac_dn23: f64 = *var_chargefrac_dn23_slot;
        let mut var_chargefrac_dn25: f64 = *var_chargefrac_dn25_slot;
        let mut var_chargefrac_dn26: f64 = *var_chargefrac_dn26_slot;
        let mut var_chargefrac_rv: f64 = *var_chargefrac_rv_slot;
        let mut var_chargefracd: f64 = *var_chargefracd_slot;
        let mut var_chargefracd_dn22: f64 = *var_chargefracd_dn22_slot;
        let mut var_chargefracd_dn23: f64 = *var_chargefracd_dn23_slot;
        let mut var_chargefracd_rv: f64 = *var_chargefracd_rv_slot;
        let mut var_chargefracg: f64 = *var_chargefracg_slot;
        let mut var_chargefracg_dn25: f64 = *var_chargefracg_dn25_slot;
        let mut var_chargefracg_dn26: f64 = *var_chargefracg_dn26_slot;
        let mut var_chargefracg_rv: f64 = *var_chargefracg_rv_slot;
        let mut var_guard12: f64 = *var_guard12_slot;
        let mut var_guard12_rv: f64 = *var_guard12_rv_slot;
        let mut var_guard13: f64 = *var_guard13_slot;
        let mut var_guard13_rv: f64 = *var_guard13_rv_slot;
        let mut var_guard16: f64 = *var_guard16_slot;
        let mut var_guard16_rv: f64 = *var_guard16_rv_slot;
        let mut var_guard17: f64 = *var_guard17_slot;
        let mut var_guard17_rv: f64 = *var_guard17_rv_slot;
        let mut var_guard18: f64 = *var_guard18_slot;
        let mut var_guard18_rv: f64 = *var_guard18_rv_slot;
        let mut var_vbfps1: f64 = *var_vbfps1_slot;
        let mut var_vbfps1_dn10: f64 = *var_vbfps1_dn10_slot;
        let mut var_vbfps1_dn3: f64 = *var_vbfps1_dn3_slot;
        let mut var_vbfps1_rv: f64 = *var_vbfps1_rv_slot;
        let mut var_vbfps2: f64 = *var_vbfps2_slot;
        let mut var_vbfps2_dn11: f64 = *var_vbfps2_dn11_slot;
        let mut var_vbfps2_dn3: f64 = *var_vbfps2_dn3_slot;
        let mut var_vbfps2_rv: f64 = *var_vbfps2_rv_slot;
        let mut var_vcfps1: f64 = *var_vcfps1_slot;
        let mut var_vcfps1_dn10: f64 = *var_vcfps1_dn10_slot;
        let mut var_vcfps1_dn2: f64 = *var_vcfps1_dn2_slot;
        let mut var_vcfps1_dn7: f64 = *var_vcfps1_dn7_slot;
        let mut var_vcfps1_rv: f64 = *var_vcfps1_rv_slot;
        let mut var_vcfps2: f64 = *var_vcfps2_slot;
        let mut var_vcfps2_dn11: f64 = *var_vcfps2_dn11_slot;
        let mut var_vcfps2_dn2: f64 = *var_vcfps2_dn2_slot;
        let mut var_vcfps2_dn7: f64 = *var_vcfps2_dn7_slot;
        let mut var_vcfps2_rv: f64 = *var_vcfps2_rv_slot;
        let mut var_vdlinput: f64 = *var_vdlinput_slot;
        let mut var_vdlinput_dn22: f64 = *var_vdlinput_dn22_slot;
        let mut var_vdlinput_rv: f64 = *var_vdlinput_rv_slot;
        let mut var_vdloutput: f64 = *var_vdloutput_slot;
        let mut var_vdloutput_dn23: f64 = *var_vdloutput_dn23_slot;
        let mut var_vdloutput_rv: f64 = *var_vdloutput_rv_slot;
        let mut var_vdsfps1: f64 = *var_vdsfps1_slot;
        let mut var_vdsfps1_dn10: f64 = *var_vdsfps1_dn10_slot;
        let mut var_vdsfps1_dn9: f64 = *var_vdsfps1_dn9_slot;
        let mut var_vdsfps1_rv: f64 = *var_vdsfps1_rv_slot;
        let mut var_vdsfps2: f64 = *var_vdsfps2_slot;
        let mut var_vdsfps2_dn10: f64 = *var_vdsfps2_dn10_slot;
        let mut var_vdsfps2_dn11: f64 = *var_vdsfps2_dn11_slot;
        let mut var_vdsfps2_rv: f64 = *var_vdsfps2_rv_slot;
        let mut var_vdsi: f64 = *var_vdsi_slot;
        let mut var_vdsi_dn5: f64 = *var_vdsi_dn5_slot;
        let mut var_vdsi_dn9: f64 = *var_vdsi_dn9_slot;
        let mut var_vdsi_rv: f64 = *var_vdsi_rv_slot;
        let mut var_vglinput: f64 = *var_vglinput_slot;
        let mut var_vglinput_dn25: f64 = *var_vglinput_dn25_slot;
        let mut var_vglinput_rv: f64 = *var_vglinput_rv_slot;
        let mut var_vgloutput: f64 = *var_vgloutput_slot;
        let mut var_vgloutput_dn26: f64 = *var_vgloutput_dn26_slot;
        let mut var_vgloutput_rv: f64 = *var_vgloutput_rv_slot;
        let mut var_vgsfps1: f64 = *var_vgsfps1_slot;
        let mut var_vgsfps1_dn10: f64 = *var_vgsfps1_dn10_slot;
        let mut var_vgsfps1_dn2: f64 = *var_vgsfps1_dn2_slot;
        let mut var_vgsfps1_dn7: f64 = *var_vgsfps1_dn7_slot;
        let mut var_vgsfps1_rv: f64 = *var_vgsfps1_rv_slot;
        let mut var_vgsfps2: f64 = *var_vgsfps2_slot;
        let mut var_vgsfps2_dn11: f64 = *var_vgsfps2_dn11_slot;
        let mut var_vgsfps2_dn2: f64 = *var_vgsfps2_dn2_slot;
        let mut var_vgsfps2_dn7: f64 = *var_vgsfps2_dn7_slot;
        let mut var_vgsfps2_rv: f64 = *var_vgsfps2_rv_slot;
        let mut var_vgsi: f64 = *var_vgsi_slot;
        let mut var_vgsi_dn8: f64 = *var_vgsi_dn8_slot;
        let mut var_vgsi_dn9: f64 = *var_vgsi_dn9_slot;
        let mut var_vgsi_rv: f64 = *var_vgsi_rv_slot;

        let assign590_e2928: f64 = (var_tdut - var_tnomk);
        let assign590_e2929: f64 = (p.p170 * assign590_e2928);
        let assign590_e2930: f64 = (1.0 + assign590_e2929);
        let (assign590_e2941, assign590_e2941_d_n4,) = {
    if (assign590_e2930 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign590_e2938: f64 = (var_tdut - var_tnomk);
        let assign590_e2939: f64 = (p.p170 * assign590_e2938);
        let assign590_e2940: f64 = (1.0 + assign590_e2939);
        (assign590_e2940, (p.p170 * var_tdut_dn4),)
    }
};
        let assign590_e2942: f64 = (p.p169 * assign590_e2941);
        var_cgfp1t = assign590_e2942;
        var_cgfp1t_dn4 = (p.p169 * assign590_e2941_d_n4);
        var_cgfp1t_rv = 0.0;

        let assign600_e2948: f64 = (var_tdut - var_tnomk);
        let assign600_e2949: f64 = (p.p192 * assign600_e2948);
        let assign600_e2950: f64 = (1.0 + assign600_e2949);
        let (assign600_e2961, assign600_e2961_d_n4,) = {
    if (assign600_e2950 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign600_e2958: f64 = (var_tdut - var_tnomk);
        let assign600_e2959: f64 = (p.p192 * assign600_e2958);
        let assign600_e2960: f64 = (1.0 + assign600_e2959);
        (assign600_e2960, (p.p192 * var_tdut_dn4),)
    }
};
        let assign600_e2962: f64 = (p.p191 * assign600_e2961);
        var_cgfp2t = assign600_e2962;
        var_cgfp2t_dn4 = (p.p191 * assign600_e2961_d_n4);
        var_cgfp2t_rv = 0.0;

        let assign610_e2968: f64 = (var_tdut - var_tnomk);
        let assign610_e2969: f64 = (p.p214 * assign610_e2968);
        let assign610_e2970: f64 = (1.0 + assign610_e2969);
        let (assign610_e2981, assign610_e2981_d_n4,) = {
    if (assign610_e2970 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign610_e2978: f64 = (var_tdut - var_tnomk);
        let assign610_e2979: f64 = (p.p214 * assign610_e2978);
        let assign610_e2980: f64 = (1.0 + assign610_e2979);
        (assign610_e2980, (p.p214 * var_tdut_dn4),)
    }
};
        let assign610_e2982: f64 = (p.p213 * assign610_e2981);
        var_cgfp3t = assign610_e2982;
        var_cgfp3t_dn4 = (p.p213 * assign610_e2981_d_n4);
        var_cgfp3t_rv = 0.0;

        let assign620_e2988: f64 = (var_tdut - var_tnomk);
        let assign620_e2989: f64 = (p.p236 * assign620_e2988);
        let assign620_e2990: f64 = (1.0 + assign620_e2989);
        let (assign620_e3001, assign620_e3001_d_n4,) = {
    if (assign620_e2990 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign620_e2998: f64 = (var_tdut - var_tnomk);
        let assign620_e2999: f64 = (p.p236 * assign620_e2998);
        let assign620_e3000: f64 = (1.0 + assign620_e2999);
        (assign620_e3000, (p.p236 * var_tdut_dn4),)
    }
};
        let assign620_e3002: f64 = (p.p235 * assign620_e3001);
        var_cgfp4t = assign620_e3002;
        var_cgfp4t_dn4 = (p.p235 * assign620_e3001_d_n4);
        var_cgfp4t_rv = 0.0;

        let assign630_e3008: f64 = (var_tdut - var_tnomk);
        let assign630_e3009: f64 = (p.p175 * assign630_e3008);
        let assign630_e3010: f64 = (1.0 + assign630_e3009);
        let (assign630_e3021, assign630_e3021_d_n4,) = {
    if (assign630_e3010 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign630_e3018: f64 = (var_tdut - var_tnomk);
        let assign630_e3019: f64 = (p.p175 * assign630_e3018);
        let assign630_e3020: f64 = (1.0 + assign630_e3019);
        (assign630_e3020, (p.p175 * var_tdut_dn4),)
    }
};
        let assign630_e3022: f64 = (p.p174 * assign630_e3021);
        var_ccfp1t = assign630_e3022;
        var_ccfp1t_dn4 = (p.p174 * assign630_e3021_d_n4);
        var_ccfp1t_rv = 0.0;

        let assign640_e3028: f64 = (var_tdut - var_tnomk);
        let assign640_e3029: f64 = (p.p197 * assign640_e3028);
        let assign640_e3030: f64 = (1.0 + assign640_e3029);
        let (assign640_e3041, assign640_e3041_d_n4,) = {
    if (assign640_e3030 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign640_e3038: f64 = (var_tdut - var_tnomk);
        let assign640_e3039: f64 = (p.p197 * assign640_e3038);
        let assign640_e3040: f64 = (1.0 + assign640_e3039);
        (assign640_e3040, (p.p197 * var_tdut_dn4),)
    }
};
        let assign640_e3042: f64 = (p.p196 * assign640_e3041);
        var_ccfp2t = assign640_e3042;
        var_ccfp2t_dn4 = (p.p196 * assign640_e3041_d_n4);
        var_ccfp2t_rv = 0.0;

        let assign650_e3048: f64 = (var_tdut - var_tnomk);
        let assign650_e3049: f64 = (p.p219 * assign650_e3048);
        let assign650_e3050: f64 = (1.0 + assign650_e3049);
        let (assign650_e3061, assign650_e3061_d_n4,) = {
    if (assign650_e3050 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign650_e3058: f64 = (var_tdut - var_tnomk);
        let assign650_e3059: f64 = (p.p219 * assign650_e3058);
        let assign650_e3060: f64 = (1.0 + assign650_e3059);
        (assign650_e3060, (p.p219 * var_tdut_dn4),)
    }
};
        let assign650_e3062: f64 = (p.p218 * assign650_e3061);
        var_ccfp3t = assign650_e3062;
        var_ccfp3t_dn4 = (p.p218 * assign650_e3061_d_n4);
        var_ccfp3t_rv = 0.0;

        let assign660_e3068: f64 = (var_tdut - var_tnomk);
        let assign660_e3069: f64 = (p.p241 * assign660_e3068);
        let assign660_e3070: f64 = (1.0 + assign660_e3069);
        let (assign660_e3081, assign660_e3081_d_n4,) = {
    if (assign660_e3070 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign660_e3078: f64 = (var_tdut - var_tnomk);
        let assign660_e3079: f64 = (p.p241 * assign660_e3078);
        let assign660_e3080: f64 = (1.0 + assign660_e3079);
        (assign660_e3080, (p.p241 * var_tdut_dn4),)
    }
};
        let assign660_e3082: f64 = (p.p240 * assign660_e3081);
        var_ccfp4t = assign660_e3082;
        var_ccfp4t_dn4 = (p.p240 * assign660_e3081_d_n4);
        var_ccfp4t_rv = 0.0;

        let assign670_e3088: f64 = (var_tdut - var_tnomk);
        let assign670_e3089: f64 = (p.p177 * assign670_e3088);
        let assign670_e3090: f64 = (1.0 + assign670_e3089);
        let (assign670_e3101, assign670_e3101_d_n4,) = {
    if (assign670_e3090 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign670_e3098: f64 = (var_tdut - var_tnomk);
        let assign670_e3099: f64 = (p.p177 * assign670_e3098);
        let assign670_e3100: f64 = (1.0 + assign670_e3099);
        (assign670_e3100, (p.p177 * var_tdut_dn4),)
    }
};
        let assign670_e3102: f64 = (p.p176 * assign670_e3101);
        var_cbfp1t = assign670_e3102;
        var_cbfp1t_dn4 = (p.p176 * assign670_e3101_d_n4);
        var_cbfp1t_rv = 0.0;

        let assign680_e3108: f64 = (var_tdut - var_tnomk);
        let assign680_e3109: f64 = (p.p199 * assign680_e3108);
        let assign680_e3110: f64 = (1.0 + assign680_e3109);
        let (assign680_e3121, assign680_e3121_d_n4,) = {
    if (assign680_e3110 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign680_e3118: f64 = (var_tdut - var_tnomk);
        let assign680_e3119: f64 = (p.p199 * assign680_e3118);
        let assign680_e3120: f64 = (1.0 + assign680_e3119);
        (assign680_e3120, (p.p199 * var_tdut_dn4),)
    }
};
        let assign680_e3122: f64 = (p.p198 * assign680_e3121);
        var_cbfp2t = assign680_e3122;
        var_cbfp2t_dn4 = (p.p198 * assign680_e3121_d_n4);
        var_cbfp2t_rv = 0.0;

        let assign690_e3128: f64 = (var_tdut - var_tnomk);
        let assign690_e3129: f64 = (p.p221 * assign690_e3128);
        let assign690_e3130: f64 = (1.0 + assign690_e3129);
        let (assign690_e3141, assign690_e3141_d_n4,) = {
    if (assign690_e3130 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign690_e3138: f64 = (var_tdut - var_tnomk);
        let assign690_e3139: f64 = (p.p221 * assign690_e3138);
        let assign690_e3140: f64 = (1.0 + assign690_e3139);
        (assign690_e3140, (p.p221 * var_tdut_dn4),)
    }
};
        let assign690_e3142: f64 = (p.p220 * assign690_e3141);
        var_cbfp3t = assign690_e3142;
        var_cbfp3t_dn4 = (p.p220 * assign690_e3141_d_n4);
        var_cbfp3t_rv = 0.0;

        let assign700_e3148: f64 = (var_tdut - var_tnomk);
        let assign700_e3149: f64 = (p.p243 * assign700_e3148);
        let assign700_e3150: f64 = (1.0 + assign700_e3149);
        let (assign700_e3161, assign700_e3161_d_n4,) = {
    if (assign700_e3150 < 0.01) {
        (0.01, 0.0,)
    } else {
        let assign700_e3158: f64 = (var_tdut - var_tnomk);
        let assign700_e3159: f64 = (p.p243 * assign700_e3158);
        let assign700_e3160: f64 = (1.0 + assign700_e3159);
        (assign700_e3160, (p.p243 * var_tdut_dn4),)
    }
};
        let assign700_e3162: f64 = (p.p242 * assign700_e3161);
        var_cbfp4t = assign700_e3162;
        var_cbfp4t_dn4 = (p.p242 * assign700_e3161_d_n4);
        var_cbfp4t_rv = 0.0;

        let assign710_e3165: f64 = (p.p6 * (nv5 - nv9));
        var_vdsi = assign710_e3165;
        var_vdsi_dn5 = p.p6;
        var_vdsi_dn9 = (-p.p6);
        var_vdsi_rv = 0.0;

        let assign720_e3168: f64 = (p.p6 * (nv8 - nv9));
        var_vgsi = assign720_e3168;
        var_vgsi_dn8 = p.p6;
        var_vgsi_dn9 = (-p.p6);
        var_vgsi_rv = 0.0;

        var_vdlinput = 0.0;
        var_vdlinput_dn22 = 0.0;
        var_vdlinput_rv = 0.0;

        var_vglinput = 0.0;
        var_vglinput_dn25 = 0.0;
        var_vglinput_rv = 0.0;

        var_vdloutput = 0.0;
        var_vdloutput_dn23 = 0.0;
        var_vdloutput_rv = 0.0;

        var_vgloutput = 0.0;
        var_vgloutput_dn26 = 0.0;
        var_vgloutput_rv = 0.0;

        var_chargefracd = 0.0;
        var_chargefracd_dn22 = 0.0;
        var_chargefracd_dn23 = 0.0;
        var_chargefracd_rv = 0.0;

        var_chargefracg = 0.0;
        var_chargefracg_dn25 = 0.0;
        var_chargefracg_dn26 = 0.0;
        var_chargefracg_rv = 0.0;

        var_chargefrac = 1.0;
        var_chargefrac_dn22 = 0.0;
        var_chargefrac_dn23 = 0.0;
        var_chargefrac_dn25 = 0.0;
        var_chargefrac_dn26 = 0.0;
        var_chargefrac_rv = 0.0;

        let assign910_e3295: f64 = if p.p328 == 1.0 { 1.0 } else { 0.0 };
        var_guard12 = assign910_e3295;
        var_guard12_rv = 0.0;

        let assign950_e3413: f64 = if p.p328 == 2.0 { 1.0 } else { 0.0 };
        var_guard13 = assign950_e3413;
        var_guard13_rv = 0.0;

        let (assign960_e3420, assign960_e3420_d_n22,) = {
    if ((var_guard12 == 0.0) && (var_guard13 != 0.0)) {
        ((nv22 - 0.0), 1.0,)
    } else {
        (var_vdlinput, var_vdlinput_dn22,)
    }
};
        var_vdlinput = assign960_e3420;
        var_vdlinput_dn22 = assign960_e3420_d_n22;
        var_vdlinput_rv = 0.0;

        let (assign970_e3427, assign970_e3427_d_n23,) = {
    if ((var_guard12 == 0.0) && (var_guard13 != 0.0)) {
        ((nv23 - 0.0), 1.0,)
    } else {
        (var_vdloutput, var_vdloutput_dn23,)
    }
};
        var_vdloutput = assign970_e3427;
        var_vdloutput_dn23 = assign970_e3427_d_n23;
        var_vdloutput_rv = 0.0;

        let (assign980_e3439, assign980_e3439_d_n22, assign980_e3439_d_n23,) = {
    if ((var_guard12 == 0.0) && (var_guard13 != 0.0)) {
        let assign980_e3434: f64 = (var_vdloutput - var_vdlinput);
        let assign980_e3435: f64 = (assign980_e3434).abs();
        let assign980_e3437: f64 = (assign980_e3435 / p.p338);
        (assign980_e3437, (if assign980_e3434 >= 0.0 { (-var_vdlinput_dn22) } else { (-(-var_vdlinput_dn22)) } / p.p338), (if assign980_e3434 >= 0.0 { var_vdloutput_dn23 } else { (-var_vdloutput_dn23) } / p.p338),)
    } else {
        (var_chargefracd, var_chargefracd_dn22, var_chargefracd_dn23,)
    }
};
        var_chargefracd = assign980_e3439;
        var_chargefracd_dn22 = assign980_e3439_d_n22;
        var_chargefracd_dn23 = assign980_e3439_d_n23;
        var_chargefracd_rv = 0.0;

        let (assign990_e3446, assign990_e3446_d_n25,) = {
    if ((var_guard12 == 0.0) && (var_guard13 != 0.0)) {
        ((nv25 - 0.0), 1.0,)
    } else {
        (var_vglinput, var_vglinput_dn25,)
    }
};
        var_vglinput = assign990_e3446;
        var_vglinput_dn25 = assign990_e3446_d_n25;
        var_vglinput_rv = 0.0;

        let (assign1000_e3453, assign1000_e3453_d_n26,) = {
    if ((var_guard12 == 0.0) && (var_guard13 != 0.0)) {
        ((nv26 - 0.0), 1.0,)
    } else {
        (var_vgloutput, var_vgloutput_dn26,)
    }
};
        var_vgloutput = assign1000_e3453;
        var_vgloutput_dn26 = assign1000_e3453_d_n26;
        var_vgloutput_rv = 0.0;

        let (assign1010_e3465, assign1010_e3465_d_n25, assign1010_e3465_d_n26,) = {
    if ((var_guard12 == 0.0) && (var_guard13 != 0.0)) {
        let assign1010_e3460: f64 = (var_vgloutput - var_vglinput);
        let assign1010_e3461: f64 = (assign1010_e3460).abs();
        let assign1010_e3463: f64 = (assign1010_e3461 / p.p337);
        (assign1010_e3463, (if assign1010_e3460 >= 0.0 { (-var_vglinput_dn25) } else { (-(-var_vglinput_dn25)) } / p.p337), (if assign1010_e3460 >= 0.0 { var_vgloutput_dn26 } else { (-var_vgloutput_dn26) } / p.p337),)
    } else {
        (var_chargefracg, var_chargefracg_dn25, var_chargefracg_dn26,)
    }
};
        var_chargefracg = assign1010_e3465;
        var_chargefracg_dn25 = assign1010_e3465_d_n25;
        var_chargefracg_dn26 = assign1010_e3465_d_n26;
        var_chargefracg_rv = 0.0;

        let (assign1020_e3478, assign1020_e3478_d_n22, assign1020_e3478_d_n23, assign1020_e3478_d_n25, assign1020_e3478_d_n26,) = {
    if ((var_guard12 == 0.0) && (var_guard13 != 0.0)) {
        let assign1020_e3473: f64 = (1.0 + var_chargefracd);
        let assign1020_e3475: f64 = (assign1020_e3473 + var_chargefracg);
        let assign1020_e3476: f64 = (1.0 / assign1020_e3475);
        (assign1020_e3476, (-(var_chargefracd_dn22 / (assign1020_e3475 * assign1020_e3475))), (-(var_chargefracd_dn23 / (assign1020_e3475 * assign1020_e3475))), (-(var_chargefracg_dn25 / (assign1020_e3475 * assign1020_e3475))), (-(var_chargefracg_dn26 / (assign1020_e3475 * assign1020_e3475))),)
    } else {
        (var_chargefrac, var_chargefrac_dn22, var_chargefrac_dn23, var_chargefrac_dn25, var_chargefrac_dn26,)
    }
};
        var_chargefrac = assign1020_e3478;
        var_chargefrac_dn22 = assign1020_e3478_d_n22;
        var_chargefrac_dn23 = assign1020_e3478_d_n23;
        var_chargefrac_dn25 = assign1020_e3478_d_n25;
        var_chargefrac_dn26 = assign1020_e3478_d_n26;
        var_chargefrac_rv = 0.0;

        let assign1110_e3597: f64 = if p.p78 == 1.0 { 1.0 } else { 0.0 };
        var_guard16 = assign1110_e3597;
        var_guard16_rv = 0.0;

        let (assign1120_e3603, assign1120_e3603_d_n2, assign1120_e3603_d_n7, assign1120_e3603_d_n10,) = {
    if (var_guard16 != 0.0) {
        let assign1120_e3601: f64 = (p.p6 * (nv7 - nv10));
        (assign1120_e3601, 0.0, p.p6, (-p.p6),)
    } else {
        (var_vgsfps1, var_vgsfps1_dn2, var_vgsfps1_dn7, var_vgsfps1_dn10,)
    }
};
        var_vgsfps1 = assign1120_e3603;
        var_vgsfps1_dn2 = assign1120_e3603_d_n2;
        var_vgsfps1_dn7 = assign1120_e3603_d_n7;
        var_vgsfps1_dn10 = assign1120_e3603_d_n10;
        var_vgsfps1_rv = 0.0;

        let (assign1130_e3609, assign1130_e3609_d_n2, assign1130_e3609_d_n7, assign1130_e3609_d_n10,) = {
    if (var_guard16 != 0.0) {
        let assign1130_e3607: f64 = (p.p6 * (nv2 - nv10));
        (assign1130_e3607, p.p6, 0.0, (-p.p6),)
    } else {
        (var_vcfps1, var_vcfps1_dn2, var_vcfps1_dn7, var_vcfps1_dn10,)
    }
};
        var_vcfps1 = assign1130_e3609;
        var_vcfps1_dn2 = assign1130_e3609_d_n2;
        var_vcfps1_dn7 = assign1130_e3609_d_n7;
        var_vcfps1_dn10 = assign1130_e3609_d_n10;
        var_vcfps1_rv = 0.0;

        let (assign1140_e3616, assign1140_e3616_d_n2, assign1140_e3616_d_n7, assign1140_e3616_d_n10,) = {
    if (var_guard16 == 0.0) {
        let assign1140_e3614: f64 = (p.p6 * (nv2 - nv10));
        (assign1140_e3614, p.p6, 0.0, (-p.p6),)
    } else {
        (var_vgsfps1, var_vgsfps1_dn2, var_vgsfps1_dn7, var_vgsfps1_dn10,)
    }
};
        var_vgsfps1 = assign1140_e3616;
        var_vgsfps1_dn2 = assign1140_e3616_d_n2;
        var_vgsfps1_dn7 = assign1140_e3616_d_n7;
        var_vgsfps1_dn10 = assign1140_e3616_d_n10;
        var_vgsfps1_rv = 0.0;

        let (assign1150_e3623, assign1150_e3623_d_n2, assign1150_e3623_d_n7, assign1150_e3623_d_n10,) = {
    if (var_guard16 == 0.0) {
        let assign1150_e3621: f64 = (p.p6 * (nv7 - nv10));
        (assign1150_e3621, 0.0, p.p6, (-p.p6),)
    } else {
        (var_vcfps1, var_vcfps1_dn2, var_vcfps1_dn7, var_vcfps1_dn10,)
    }
};
        var_vcfps1 = assign1150_e3623;
        var_vcfps1_dn2 = assign1150_e3623_d_n2;
        var_vcfps1_dn7 = assign1150_e3623_d_n7;
        var_vcfps1_dn10 = assign1150_e3623_d_n10;
        var_vcfps1_rv = 0.0;

        let assign1160_e3626: f64 = (p.p6 * (nv9 - nv10));
        var_vdsfps1 = assign1160_e3626;
        var_vdsfps1_dn9 = p.p6;
        var_vdsfps1_dn10 = (-p.p6);
        var_vdsfps1_rv = 0.0;

        let assign1170_e3629: f64 = (p.p6 * (nv3 - nv10));
        var_vbfps1 = assign1170_e3629;
        var_vbfps1_dn3 = p.p6;
        var_vbfps1_dn10 = (-p.p6);
        var_vbfps1_rv = 0.0;

        let assign1180_e3632: f64 = if p.p100 == 1.0 { 1.0 } else { 0.0 };
        var_guard17 = assign1180_e3632;
        var_guard17_rv = 0.0;

        let (assign1190_e3638, assign1190_e3638_d_n2, assign1190_e3638_d_n7, assign1190_e3638_d_n11,) = {
    if (var_guard17 != 0.0) {
        let assign1190_e3636: f64 = (p.p6 * (nv7 - nv11));
        (assign1190_e3636, 0.0, p.p6, (-p.p6),)
    } else {
        (var_vgsfps2, var_vgsfps2_dn2, var_vgsfps2_dn7, var_vgsfps2_dn11,)
    }
};
        var_vgsfps2 = assign1190_e3638;
        var_vgsfps2_dn2 = assign1190_e3638_d_n2;
        var_vgsfps2_dn7 = assign1190_e3638_d_n7;
        var_vgsfps2_dn11 = assign1190_e3638_d_n11;
        var_vgsfps2_rv = 0.0;

        let (assign1200_e3644, assign1200_e3644_d_n2, assign1200_e3644_d_n7, assign1200_e3644_d_n11,) = {
    if (var_guard17 != 0.0) {
        let assign1200_e3642: f64 = (p.p6 * (nv2 - nv11));
        (assign1200_e3642, p.p6, 0.0, (-p.p6),)
    } else {
        (var_vcfps2, var_vcfps2_dn2, var_vcfps2_dn7, var_vcfps2_dn11,)
    }
};
        var_vcfps2 = assign1200_e3644;
        var_vcfps2_dn2 = assign1200_e3644_d_n2;
        var_vcfps2_dn7 = assign1200_e3644_d_n7;
        var_vcfps2_dn11 = assign1200_e3644_d_n11;
        var_vcfps2_rv = 0.0;

        let (assign1210_e3651, assign1210_e3651_d_n2, assign1210_e3651_d_n7, assign1210_e3651_d_n11,) = {
    if (var_guard17 == 0.0) {
        let assign1210_e3649: f64 = (p.p6 * (nv2 - nv11));
        (assign1210_e3649, p.p6, 0.0, (-p.p6),)
    } else {
        (var_vgsfps2, var_vgsfps2_dn2, var_vgsfps2_dn7, var_vgsfps2_dn11,)
    }
};
        var_vgsfps2 = assign1210_e3651;
        var_vgsfps2_dn2 = assign1210_e3651_d_n2;
        var_vgsfps2_dn7 = assign1210_e3651_d_n7;
        var_vgsfps2_dn11 = assign1210_e3651_d_n11;
        var_vgsfps2_rv = 0.0;

        let (assign1220_e3658, assign1220_e3658_d_n2, assign1220_e3658_d_n7, assign1220_e3658_d_n11,) = {
    if (var_guard17 == 0.0) {
        let assign1220_e3656: f64 = (p.p6 * (nv7 - nv11));
        (assign1220_e3656, 0.0, p.p6, (-p.p6),)
    } else {
        (var_vcfps2, var_vcfps2_dn2, var_vcfps2_dn7, var_vcfps2_dn11,)
    }
};
        var_vcfps2 = assign1220_e3658;
        var_vcfps2_dn2 = assign1220_e3658_d_n2;
        var_vcfps2_dn7 = assign1220_e3658_d_n7;
        var_vcfps2_dn11 = assign1220_e3658_d_n11;
        var_vcfps2_rv = 0.0;

        let assign1230_e3661: f64 = (p.p6 * (nv10 - nv11));
        var_vdsfps2 = assign1230_e3661;
        var_vdsfps2_dn10 = p.p6;
        var_vdsfps2_dn11 = (-p.p6);
        var_vdsfps2_rv = 0.0;

        let assign1240_e3664: f64 = (p.p6 * (nv3 - nv11));
        var_vbfps2 = assign1240_e3664;
        var_vbfps2_dn3 = p.p6;
        var_vbfps2_dn11 = (-p.p6);
        var_vbfps2_rv = 0.0;

        let assign1250_e3667: f64 = if p.p122 == 1.0 { 1.0 } else { 0.0 };
        var_guard18 = assign1250_e3667;
        var_guard18_rv = 0.0;

        *var_cbfp1t_slot = var_cbfp1t;
        *var_cbfp1t_dn4_slot = var_cbfp1t_dn4;
        *var_cbfp1t_rv_slot = var_cbfp1t_rv;
        *var_cbfp2t_slot = var_cbfp2t;
        *var_cbfp2t_dn4_slot = var_cbfp2t_dn4;
        *var_cbfp2t_rv_slot = var_cbfp2t_rv;
        *var_cbfp3t_slot = var_cbfp3t;
        *var_cbfp3t_dn4_slot = var_cbfp3t_dn4;
        *var_cbfp3t_rv_slot = var_cbfp3t_rv;
        *var_cbfp4t_slot = var_cbfp4t;
        *var_cbfp4t_dn4_slot = var_cbfp4t_dn4;
        *var_cbfp4t_rv_slot = var_cbfp4t_rv;
        *var_ccfp1t_slot = var_ccfp1t;
        *var_ccfp1t_dn4_slot = var_ccfp1t_dn4;
        *var_ccfp1t_rv_slot = var_ccfp1t_rv;
        *var_ccfp2t_slot = var_ccfp2t;
        *var_ccfp2t_dn4_slot = var_ccfp2t_dn4;
        *var_ccfp2t_rv_slot = var_ccfp2t_rv;
        *var_ccfp3t_slot = var_ccfp3t;
        *var_ccfp3t_dn4_slot = var_ccfp3t_dn4;
        *var_ccfp3t_rv_slot = var_ccfp3t_rv;
        *var_ccfp4t_slot = var_ccfp4t;
        *var_ccfp4t_dn4_slot = var_ccfp4t_dn4;
        *var_ccfp4t_rv_slot = var_ccfp4t_rv;
        *var_cgfp1t_slot = var_cgfp1t;
        *var_cgfp1t_dn4_slot = var_cgfp1t_dn4;
        *var_cgfp1t_rv_slot = var_cgfp1t_rv;
        *var_cgfp2t_slot = var_cgfp2t;
        *var_cgfp2t_dn4_slot = var_cgfp2t_dn4;
        *var_cgfp2t_rv_slot = var_cgfp2t_rv;
        *var_cgfp3t_slot = var_cgfp3t;
        *var_cgfp3t_dn4_slot = var_cgfp3t_dn4;
        *var_cgfp3t_rv_slot = var_cgfp3t_rv;
        *var_cgfp4t_slot = var_cgfp4t;
        *var_cgfp4t_dn4_slot = var_cgfp4t_dn4;
        *var_cgfp4t_rv_slot = var_cgfp4t_rv;
        *var_chargefrac_slot = var_chargefrac;
        *var_chargefrac_dn22_slot = var_chargefrac_dn22;
        *var_chargefrac_dn23_slot = var_chargefrac_dn23;
        *var_chargefrac_dn25_slot = var_chargefrac_dn25;
        *var_chargefrac_dn26_slot = var_chargefrac_dn26;
        *var_chargefrac_rv_slot = var_chargefrac_rv;
        *var_chargefracd_slot = var_chargefracd;
        *var_chargefracd_dn22_slot = var_chargefracd_dn22;
        *var_chargefracd_dn23_slot = var_chargefracd_dn23;
        *var_chargefracd_rv_slot = var_chargefracd_rv;
        *var_chargefracg_slot = var_chargefracg;
        *var_chargefracg_dn25_slot = var_chargefracg_dn25;
        *var_chargefracg_dn26_slot = var_chargefracg_dn26;
        *var_chargefracg_rv_slot = var_chargefracg_rv;
        *var_guard12_slot = var_guard12;
        *var_guard12_rv_slot = var_guard12_rv;
        *var_guard13_slot = var_guard13;
        *var_guard13_rv_slot = var_guard13_rv;
        *var_guard16_slot = var_guard16;
        *var_guard16_rv_slot = var_guard16_rv;
        *var_guard17_slot = var_guard17;
        *var_guard17_rv_slot = var_guard17_rv;
        *var_guard18_slot = var_guard18;
        *var_guard18_rv_slot = var_guard18_rv;
        *var_vbfps1_slot = var_vbfps1;
        *var_vbfps1_dn10_slot = var_vbfps1_dn10;
        *var_vbfps1_dn3_slot = var_vbfps1_dn3;
        *var_vbfps1_rv_slot = var_vbfps1_rv;
        *var_vbfps2_slot = var_vbfps2;
        *var_vbfps2_dn11_slot = var_vbfps2_dn11;
        *var_vbfps2_dn3_slot = var_vbfps2_dn3;
        *var_vbfps2_rv_slot = var_vbfps2_rv;
        *var_vcfps1_slot = var_vcfps1;
        *var_vcfps1_dn10_slot = var_vcfps1_dn10;
        *var_vcfps1_dn2_slot = var_vcfps1_dn2;
        *var_vcfps1_dn7_slot = var_vcfps1_dn7;
        *var_vcfps1_rv_slot = var_vcfps1_rv;
        *var_vcfps2_slot = var_vcfps2;
        *var_vcfps2_dn11_slot = var_vcfps2_dn11;
        *var_vcfps2_dn2_slot = var_vcfps2_dn2;
        *var_vcfps2_dn7_slot = var_vcfps2_dn7;
        *var_vcfps2_rv_slot = var_vcfps2_rv;
        *var_vdlinput_slot = var_vdlinput;
        *var_vdlinput_dn22_slot = var_vdlinput_dn22;
        *var_vdlinput_rv_slot = var_vdlinput_rv;
        *var_vdloutput_slot = var_vdloutput;
        *var_vdloutput_dn23_slot = var_vdloutput_dn23;
        *var_vdloutput_rv_slot = var_vdloutput_rv;
        *var_vdsfps1_slot = var_vdsfps1;
        *var_vdsfps1_dn10_slot = var_vdsfps1_dn10;
        *var_vdsfps1_dn9_slot = var_vdsfps1_dn9;
        *var_vdsfps1_rv_slot = var_vdsfps1_rv;
        *var_vdsfps2_slot = var_vdsfps2;
        *var_vdsfps2_dn10_slot = var_vdsfps2_dn10;
        *var_vdsfps2_dn11_slot = var_vdsfps2_dn11;
        *var_vdsfps2_rv_slot = var_vdsfps2_rv;
        *var_vdsi_slot = var_vdsi;
        *var_vdsi_dn5_slot = var_vdsi_dn5;
        *var_vdsi_dn9_slot = var_vdsi_dn9;
        *var_vdsi_rv_slot = var_vdsi_rv;
        *var_vglinput_slot = var_vglinput;
        *var_vglinput_dn25_slot = var_vglinput_dn25;
        *var_vglinput_rv_slot = var_vglinput_rv;
        *var_vgloutput_slot = var_vgloutput;
        *var_vgloutput_dn26_slot = var_vgloutput_dn26;
        *var_vgloutput_rv_slot = var_vgloutput_rv;
        *var_vgsfps1_slot = var_vgsfps1;
        *var_vgsfps1_dn10_slot = var_vgsfps1_dn10;
        *var_vgsfps1_dn2_slot = var_vgsfps1_dn2;
        *var_vgsfps1_dn7_slot = var_vgsfps1_dn7;
        *var_vgsfps1_rv_slot = var_vgsfps1_rv;
        *var_vgsfps2_slot = var_vgsfps2;
        *var_vgsfps2_dn11_slot = var_vgsfps2_dn11;
        *var_vgsfps2_dn2_slot = var_vgsfps2_dn2;
        *var_vgsfps2_dn7_slot = var_vgsfps2_dn7;
        *var_vgsfps2_rv_slot = var_vgsfps2_rv;
        *var_vgsi_slot = var_vgsi;
        *var_vgsi_dn8_slot = var_vgsi_dn8;
        *var_vgsi_dn9_slot = var_vgsi_dn9;
        *var_vgsi_rv_slot = var_vgsi_rv;
    }

    pub(super) fn stamp_reactive_block_2(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_guard18: f64,
        var_fn25_calc_iq__qgdout_slot: &mut f64,
        var_fn25_calc_iq__qgdout_dn16_slot: &mut f64,
        var_fn25_calc_iq__qgdout_dn17_slot: &mut f64,
        var_fn25_calc_iq__qgdout_dn2_slot: &mut f64,
        var_fn25_calc_iq__qgdout_dn4_slot: &mut f64,
        var_fn25_calc_iq__qgdout_dn7_slot: &mut f64,
        var_fn25_calc_iq__qgdout_rv_slot: &mut f64,
        var_fn25_calc_iq__qgsout_slot: &mut f64,
        var_fn25_calc_iq__qgsout_dn16_slot: &mut f64,
        var_fn25_calc_iq__qgsout_dn17_slot: &mut f64,
        var_fn25_calc_iq__qgsout_dn2_slot: &mut f64,
        var_fn25_calc_iq__qgsout_dn4_slot: &mut f64,
        var_fn25_calc_iq__qgsout_dn7_slot: &mut f64,
        var_fn25_calc_iq__qgsout_rv_slot: &mut f64,
        var_guard19_slot: &mut f64,
        var_guard19_rv_slot: &mut f64,
        var_guard20_slot: &mut f64,
        var_guard20_rv_slot: &mut f64,
        var_guard21_slot: &mut f64,
        var_guard21_rv_slot: &mut f64,
        var_guard22_slot: &mut f64,
        var_guard22_rv_slot: &mut f64,
        var_guard23_slot: &mut f64,
        var_guard23_rv_slot: &mut f64,
        var_guard24_slot: &mut f64,
        var_guard24_rv_slot: &mut f64,
        var_qbfp4_slot: &mut f64,
        var_qbfp4_dn16_slot: &mut f64,
        var_qbfp4_dn17_slot: &mut f64,
        var_qbfp4_dn2_slot: &mut f64,
        var_qbfp4_dn3_slot: &mut f64,
        var_qbfp4_dn4_slot: &mut f64,
        var_qbfp4_dn7_slot: &mut f64,
        var_qbfp4_rv_slot: &mut f64,
        var_qcfp4_slot: &mut f64,
        var_qcfp4_dn16_slot: &mut f64,
        var_qcfp4_dn17_slot: &mut f64,
        var_qcfp4_dn2_slot: &mut f64,
        var_qcfp4_dn3_slot: &mut f64,
        var_qcfp4_dn4_slot: &mut f64,
        var_qcfp4_dn7_slot: &mut f64,
        var_qcfp4_rv_slot: &mut f64,
        var_qgdfp4_slot: &mut f64,
        var_qgdfp4_dn16_slot: &mut f64,
        var_qgdfp4_dn17_slot: &mut f64,
        var_qgdfp4_dn2_slot: &mut f64,
        var_qgdfp4_dn4_slot: &mut f64,
        var_qgdfp4_dn7_slot: &mut f64,
        var_qgdfp4_rv_slot: &mut f64,
        var_qgsfp4_slot: &mut f64,
        var_qgsfp4_dn16_slot: &mut f64,
        var_qgsfp4_dn17_slot: &mut f64,
        var_qgsfp4_dn2_slot: &mut f64,
        var_qgsfp4_dn4_slot: &mut f64,
        var_qgsfp4_dn7_slot: &mut f64,
        var_qgsfp4_rv_slot: &mut f64,
        var_qsfp4_slot: &mut f64,
        var_qsfp4_dn16_slot: &mut f64,
        var_qsfp4_dn17_slot: &mut f64,
        var_qsfp4_dn2_slot: &mut f64,
        var_qsfp4_dn3_slot: &mut f64,
        var_qsfp4_dn4_slot: &mut f64,
        var_qsfp4_dn7_slot: &mut f64,
        var_qsfp4_rv_slot: &mut f64,
        var_vbfp1_slot: &mut f64,
        var_vbfp1_dn3_slot: &mut f64,
        var_vbfp1_dn5_slot: &mut f64,
        var_vbfp1_rv_slot: &mut f64,
        var_vbfp2_slot: &mut f64,
        var_vbfp2_dn14_slot: &mut f64,
        var_vbfp2_dn3_slot: &mut f64,
        var_vbfp2_rv_slot: &mut f64,
        var_vbfp3_slot: &mut f64,
        var_vbfp3_dn15_slot: &mut f64,
        var_vbfp3_dn3_slot: &mut f64,
        var_vbfp3_rv_slot: &mut f64,
        var_vbfp4_slot: &mut f64,
        var_vbfp4_dn16_slot: &mut f64,
        var_vbfp4_dn3_slot: &mut f64,
        var_vbfp4_rv_slot: &mut f64,
        var_vbfps3_slot: &mut f64,
        var_vbfps3_dn12_slot: &mut f64,
        var_vbfps3_dn3_slot: &mut f64,
        var_vbfps3_rv_slot: &mut f64,
        var_vbfps4_slot: &mut f64,
        var_vbfps4_dn13_slot: &mut f64,
        var_vbfps4_dn3_slot: &mut f64,
        var_vbfps4_rv_slot: &mut f64,
        var_vcfp1_slot: &mut f64,
        var_vcfp1_dn2_slot: &mut f64,
        var_vcfp1_dn5_slot: &mut f64,
        var_vcfp1_dn7_slot: &mut f64,
        var_vcfp1_rv_slot: &mut f64,
        var_vcfp2_slot: &mut f64,
        var_vcfp2_dn14_slot: &mut f64,
        var_vcfp2_dn2_slot: &mut f64,
        var_vcfp2_dn7_slot: &mut f64,
        var_vcfp2_rv_slot: &mut f64,
        var_vcfp3_slot: &mut f64,
        var_vcfp3_dn15_slot: &mut f64,
        var_vcfp3_dn2_slot: &mut f64,
        var_vcfp3_dn7_slot: &mut f64,
        var_vcfp3_rv_slot: &mut f64,
        var_vcfp4_slot: &mut f64,
        var_vcfp4_dn16_slot: &mut f64,
        var_vcfp4_dn2_slot: &mut f64,
        var_vcfp4_dn7_slot: &mut f64,
        var_vcfp4_rv_slot: &mut f64,
        var_vcfps3_slot: &mut f64,
        var_vcfps3_dn12_slot: &mut f64,
        var_vcfps3_dn2_slot: &mut f64,
        var_vcfps3_dn7_slot: &mut f64,
        var_vcfps3_rv_slot: &mut f64,
        var_vcfps4_slot: &mut f64,
        var_vcfps4_dn13_slot: &mut f64,
        var_vcfps4_dn2_slot: &mut f64,
        var_vcfps4_dn7_slot: &mut f64,
        var_vcfps4_rv_slot: &mut f64,
        var_vdsfp1_slot: &mut f64,
        var_vdsfp1_dn14_slot: &mut f64,
        var_vdsfp1_dn5_slot: &mut f64,
        var_vdsfp1_rv_slot: &mut f64,
        var_vdsfp2_slot: &mut f64,
        var_vdsfp2_dn14_slot: &mut f64,
        var_vdsfp2_dn15_slot: &mut f64,
        var_vdsfp2_rv_slot: &mut f64,
        var_vdsfp3_slot: &mut f64,
        var_vdsfp3_dn15_slot: &mut f64,
        var_vdsfp3_dn16_slot: &mut f64,
        var_vdsfp3_rv_slot: &mut f64,
        var_vdsfp4_slot: &mut f64,
        var_vdsfp4_dn16_slot: &mut f64,
        var_vdsfp4_dn17_slot: &mut f64,
        var_vdsfp4_rv_slot: &mut f64,
        var_vdsfps3_slot: &mut f64,
        var_vdsfps3_dn11_slot: &mut f64,
        var_vdsfps3_dn12_slot: &mut f64,
        var_vdsfps3_rv_slot: &mut f64,
        var_vdsfps4_slot: &mut f64,
        var_vdsfps4_dn12_slot: &mut f64,
        var_vdsfps4_dn13_slot: &mut f64,
        var_vdsfps4_rv_slot: &mut f64,
        var_vgsfp1_slot: &mut f64,
        var_vgsfp1_dn2_slot: &mut f64,
        var_vgsfp1_dn5_slot: &mut f64,
        var_vgsfp1_dn7_slot: &mut f64,
        var_vgsfp1_rv_slot: &mut f64,
        var_vgsfp2_slot: &mut f64,
        var_vgsfp2_dn14_slot: &mut f64,
        var_vgsfp2_dn2_slot: &mut f64,
        var_vgsfp2_dn7_slot: &mut f64,
        var_vgsfp2_rv_slot: &mut f64,
        var_vgsfp3_slot: &mut f64,
        var_vgsfp3_dn15_slot: &mut f64,
        var_vgsfp3_dn2_slot: &mut f64,
        var_vgsfp3_dn7_slot: &mut f64,
        var_vgsfp3_rv_slot: &mut f64,
        var_vgsfp4_slot: &mut f64,
        var_vgsfp4_dn16_slot: &mut f64,
        var_vgsfp4_dn2_slot: &mut f64,
        var_vgsfp4_dn7_slot: &mut f64,
        var_vgsfp4_rv_slot: &mut f64,
        var_vgsfps3_slot: &mut f64,
        var_vgsfps3_dn12_slot: &mut f64,
        var_vgsfps3_dn2_slot: &mut f64,
        var_vgsfps3_dn7_slot: &mut f64,
        var_vgsfps3_rv_slot: &mut f64,
        var_vgsfps4_slot: &mut f64,
        var_vgsfps4_dn13_slot: &mut f64,
        var_vgsfps4_dn2_slot: &mut f64,
        var_vgsfps4_dn7_slot: &mut f64,
        var_vgsfps4_rv_slot: &mut f64,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let mut var_fn25_calc_iq__qgdout: f64 = *var_fn25_calc_iq__qgdout_slot;
        let mut var_fn25_calc_iq__qgdout_dn16: f64 = *var_fn25_calc_iq__qgdout_dn16_slot;
        let mut var_fn25_calc_iq__qgdout_dn17: f64 = *var_fn25_calc_iq__qgdout_dn17_slot;
        let mut var_fn25_calc_iq__qgdout_dn2: f64 = *var_fn25_calc_iq__qgdout_dn2_slot;
        let mut var_fn25_calc_iq__qgdout_dn4: f64 = *var_fn25_calc_iq__qgdout_dn4_slot;
        let mut var_fn25_calc_iq__qgdout_dn7: f64 = *var_fn25_calc_iq__qgdout_dn7_slot;
        let mut var_fn25_calc_iq__qgdout_rv: f64 = *var_fn25_calc_iq__qgdout_rv_slot;
        let mut var_fn25_calc_iq__qgsout: f64 = *var_fn25_calc_iq__qgsout_slot;
        let mut var_fn25_calc_iq__qgsout_dn16: f64 = *var_fn25_calc_iq__qgsout_dn16_slot;
        let mut var_fn25_calc_iq__qgsout_dn17: f64 = *var_fn25_calc_iq__qgsout_dn17_slot;
        let mut var_fn25_calc_iq__qgsout_dn2: f64 = *var_fn25_calc_iq__qgsout_dn2_slot;
        let mut var_fn25_calc_iq__qgsout_dn4: f64 = *var_fn25_calc_iq__qgsout_dn4_slot;
        let mut var_fn25_calc_iq__qgsout_dn7: f64 = *var_fn25_calc_iq__qgsout_dn7_slot;
        let mut var_fn25_calc_iq__qgsout_rv: f64 = *var_fn25_calc_iq__qgsout_rv_slot;
        let mut var_guard19: f64 = *var_guard19_slot;
        let mut var_guard19_rv: f64 = *var_guard19_rv_slot;
        let mut var_guard20: f64 = *var_guard20_slot;
        let mut var_guard20_rv: f64 = *var_guard20_rv_slot;
        let mut var_guard21: f64 = *var_guard21_slot;
        let mut var_guard21_rv: f64 = *var_guard21_rv_slot;
        let mut var_guard22: f64 = *var_guard22_slot;
        let mut var_guard22_rv: f64 = *var_guard22_rv_slot;
        let mut var_guard23: f64 = *var_guard23_slot;
        let mut var_guard23_rv: f64 = *var_guard23_rv_slot;
        let mut var_guard24: f64 = *var_guard24_slot;
        let mut var_guard24_rv: f64 = *var_guard24_rv_slot;
        let mut var_qbfp4: f64 = *var_qbfp4_slot;
        let mut var_qbfp4_dn16: f64 = *var_qbfp4_dn16_slot;
        let mut var_qbfp4_dn17: f64 = *var_qbfp4_dn17_slot;
        let mut var_qbfp4_dn2: f64 = *var_qbfp4_dn2_slot;
        let mut var_qbfp4_dn3: f64 = *var_qbfp4_dn3_slot;
        let mut var_qbfp4_dn4: f64 = *var_qbfp4_dn4_slot;
        let mut var_qbfp4_dn7: f64 = *var_qbfp4_dn7_slot;
        let mut var_qbfp4_rv: f64 = *var_qbfp4_rv_slot;
        let mut var_qcfp4: f64 = *var_qcfp4_slot;
        let mut var_qcfp4_dn16: f64 = *var_qcfp4_dn16_slot;
        let mut var_qcfp4_dn17: f64 = *var_qcfp4_dn17_slot;
        let mut var_qcfp4_dn2: f64 = *var_qcfp4_dn2_slot;
        let mut var_qcfp4_dn3: f64 = *var_qcfp4_dn3_slot;
        let mut var_qcfp4_dn4: f64 = *var_qcfp4_dn4_slot;
        let mut var_qcfp4_dn7: f64 = *var_qcfp4_dn7_slot;
        let mut var_qcfp4_rv: f64 = *var_qcfp4_rv_slot;
        let mut var_qgdfp4: f64 = *var_qgdfp4_slot;
        let mut var_qgdfp4_dn16: f64 = *var_qgdfp4_dn16_slot;
        let mut var_qgdfp4_dn17: f64 = *var_qgdfp4_dn17_slot;
        let mut var_qgdfp4_dn2: f64 = *var_qgdfp4_dn2_slot;
        let mut var_qgdfp4_dn4: f64 = *var_qgdfp4_dn4_slot;
        let mut var_qgdfp4_dn7: f64 = *var_qgdfp4_dn7_slot;
        let mut var_qgdfp4_rv: f64 = *var_qgdfp4_rv_slot;
        let mut var_qgsfp4: f64 = *var_qgsfp4_slot;
        let mut var_qgsfp4_dn16: f64 = *var_qgsfp4_dn16_slot;
        let mut var_qgsfp4_dn17: f64 = *var_qgsfp4_dn17_slot;
        let mut var_qgsfp4_dn2: f64 = *var_qgsfp4_dn2_slot;
        let mut var_qgsfp4_dn4: f64 = *var_qgsfp4_dn4_slot;
        let mut var_qgsfp4_dn7: f64 = *var_qgsfp4_dn7_slot;
        let mut var_qgsfp4_rv: f64 = *var_qgsfp4_rv_slot;
        let mut var_qsfp4: f64 = *var_qsfp4_slot;
        let mut var_qsfp4_dn16: f64 = *var_qsfp4_dn16_slot;
        let mut var_qsfp4_dn17: f64 = *var_qsfp4_dn17_slot;
        let mut var_qsfp4_dn2: f64 = *var_qsfp4_dn2_slot;
        let mut var_qsfp4_dn3: f64 = *var_qsfp4_dn3_slot;
        let mut var_qsfp4_dn4: f64 = *var_qsfp4_dn4_slot;
        let mut var_qsfp4_dn7: f64 = *var_qsfp4_dn7_slot;
        let mut var_qsfp4_rv: f64 = *var_qsfp4_rv_slot;
        let mut var_vbfp1: f64 = *var_vbfp1_slot;
        let mut var_vbfp1_dn3: f64 = *var_vbfp1_dn3_slot;
        let mut var_vbfp1_dn5: f64 = *var_vbfp1_dn5_slot;
        let mut var_vbfp1_rv: f64 = *var_vbfp1_rv_slot;
        let mut var_vbfp2: f64 = *var_vbfp2_slot;
        let mut var_vbfp2_dn14: f64 = *var_vbfp2_dn14_slot;
        let mut var_vbfp2_dn3: f64 = *var_vbfp2_dn3_slot;
        let mut var_vbfp2_rv: f64 = *var_vbfp2_rv_slot;
        let mut var_vbfp3: f64 = *var_vbfp3_slot;
        let mut var_vbfp3_dn15: f64 = *var_vbfp3_dn15_slot;
        let mut var_vbfp3_dn3: f64 = *var_vbfp3_dn3_slot;
        let mut var_vbfp3_rv: f64 = *var_vbfp3_rv_slot;
        let mut var_vbfp4: f64 = *var_vbfp4_slot;
        let mut var_vbfp4_dn16: f64 = *var_vbfp4_dn16_slot;
        let mut var_vbfp4_dn3: f64 = *var_vbfp4_dn3_slot;
        let mut var_vbfp4_rv: f64 = *var_vbfp4_rv_slot;
        let mut var_vbfps3: f64 = *var_vbfps3_slot;
        let mut var_vbfps3_dn12: f64 = *var_vbfps3_dn12_slot;
        let mut var_vbfps3_dn3: f64 = *var_vbfps3_dn3_slot;
        let mut var_vbfps3_rv: f64 = *var_vbfps3_rv_slot;
        let mut var_vbfps4: f64 = *var_vbfps4_slot;
        let mut var_vbfps4_dn13: f64 = *var_vbfps4_dn13_slot;
        let mut var_vbfps4_dn3: f64 = *var_vbfps4_dn3_slot;
        let mut var_vbfps4_rv: f64 = *var_vbfps4_rv_slot;
        let mut var_vcfp1: f64 = *var_vcfp1_slot;
        let mut var_vcfp1_dn2: f64 = *var_vcfp1_dn2_slot;
        let mut var_vcfp1_dn5: f64 = *var_vcfp1_dn5_slot;
        let mut var_vcfp1_dn7: f64 = *var_vcfp1_dn7_slot;
        let mut var_vcfp1_rv: f64 = *var_vcfp1_rv_slot;
        let mut var_vcfp2: f64 = *var_vcfp2_slot;
        let mut var_vcfp2_dn14: f64 = *var_vcfp2_dn14_slot;
        let mut var_vcfp2_dn2: f64 = *var_vcfp2_dn2_slot;
        let mut var_vcfp2_dn7: f64 = *var_vcfp2_dn7_slot;
        let mut var_vcfp2_rv: f64 = *var_vcfp2_rv_slot;
        let mut var_vcfp3: f64 = *var_vcfp3_slot;
        let mut var_vcfp3_dn15: f64 = *var_vcfp3_dn15_slot;
        let mut var_vcfp3_dn2: f64 = *var_vcfp3_dn2_slot;
        let mut var_vcfp3_dn7: f64 = *var_vcfp3_dn7_slot;
        let mut var_vcfp3_rv: f64 = *var_vcfp3_rv_slot;
        let mut var_vcfp4: f64 = *var_vcfp4_slot;
        let mut var_vcfp4_dn16: f64 = *var_vcfp4_dn16_slot;
        let mut var_vcfp4_dn2: f64 = *var_vcfp4_dn2_slot;
        let mut var_vcfp4_dn7: f64 = *var_vcfp4_dn7_slot;
        let mut var_vcfp4_rv: f64 = *var_vcfp4_rv_slot;
        let mut var_vcfps3: f64 = *var_vcfps3_slot;
        let mut var_vcfps3_dn12: f64 = *var_vcfps3_dn12_slot;
        let mut var_vcfps3_dn2: f64 = *var_vcfps3_dn2_slot;
        let mut var_vcfps3_dn7: f64 = *var_vcfps3_dn7_slot;
        let mut var_vcfps3_rv: f64 = *var_vcfps3_rv_slot;
        let mut var_vcfps4: f64 = *var_vcfps4_slot;
        let mut var_vcfps4_dn13: f64 = *var_vcfps4_dn13_slot;
        let mut var_vcfps4_dn2: f64 = *var_vcfps4_dn2_slot;
        let mut var_vcfps4_dn7: f64 = *var_vcfps4_dn7_slot;
        let mut var_vcfps4_rv: f64 = *var_vcfps4_rv_slot;
        let mut var_vdsfp1: f64 = *var_vdsfp1_slot;
        let mut var_vdsfp1_dn14: f64 = *var_vdsfp1_dn14_slot;
        let mut var_vdsfp1_dn5: f64 = *var_vdsfp1_dn5_slot;
        let mut var_vdsfp1_rv: f64 = *var_vdsfp1_rv_slot;
        let mut var_vdsfp2: f64 = *var_vdsfp2_slot;
        let mut var_vdsfp2_dn14: f64 = *var_vdsfp2_dn14_slot;
        let mut var_vdsfp2_dn15: f64 = *var_vdsfp2_dn15_slot;
        let mut var_vdsfp2_rv: f64 = *var_vdsfp2_rv_slot;
        let mut var_vdsfp3: f64 = *var_vdsfp3_slot;
        let mut var_vdsfp3_dn15: f64 = *var_vdsfp3_dn15_slot;
        let mut var_vdsfp3_dn16: f64 = *var_vdsfp3_dn16_slot;
        let mut var_vdsfp3_rv: f64 = *var_vdsfp3_rv_slot;
        let mut var_vdsfp4: f64 = *var_vdsfp4_slot;
        let mut var_vdsfp4_dn16: f64 = *var_vdsfp4_dn16_slot;
        let mut var_vdsfp4_dn17: f64 = *var_vdsfp4_dn17_slot;
        let mut var_vdsfp4_rv: f64 = *var_vdsfp4_rv_slot;
        let mut var_vdsfps3: f64 = *var_vdsfps3_slot;
        let mut var_vdsfps3_dn11: f64 = *var_vdsfps3_dn11_slot;
        let mut var_vdsfps3_dn12: f64 = *var_vdsfps3_dn12_slot;
        let mut var_vdsfps3_rv: f64 = *var_vdsfps3_rv_slot;
        let mut var_vdsfps4: f64 = *var_vdsfps4_slot;
        let mut var_vdsfps4_dn12: f64 = *var_vdsfps4_dn12_slot;
        let mut var_vdsfps4_dn13: f64 = *var_vdsfps4_dn13_slot;
        let mut var_vdsfps4_rv: f64 = *var_vdsfps4_rv_slot;
        let mut var_vgsfp1: f64 = *var_vgsfp1_slot;
        let mut var_vgsfp1_dn2: f64 = *var_vgsfp1_dn2_slot;
        let mut var_vgsfp1_dn5: f64 = *var_vgsfp1_dn5_slot;
        let mut var_vgsfp1_dn7: f64 = *var_vgsfp1_dn7_slot;
        let mut var_vgsfp1_rv: f64 = *var_vgsfp1_rv_slot;
        let mut var_vgsfp2: f64 = *var_vgsfp2_slot;
        let mut var_vgsfp2_dn14: f64 = *var_vgsfp2_dn14_slot;
        let mut var_vgsfp2_dn2: f64 = *var_vgsfp2_dn2_slot;
        let mut var_vgsfp2_dn7: f64 = *var_vgsfp2_dn7_slot;
        let mut var_vgsfp2_rv: f64 = *var_vgsfp2_rv_slot;
        let mut var_vgsfp3: f64 = *var_vgsfp3_slot;
        let mut var_vgsfp3_dn15: f64 = *var_vgsfp3_dn15_slot;
        let mut var_vgsfp3_dn2: f64 = *var_vgsfp3_dn2_slot;
        let mut var_vgsfp3_dn7: f64 = *var_vgsfp3_dn7_slot;
        let mut var_vgsfp3_rv: f64 = *var_vgsfp3_rv_slot;
        let mut var_vgsfp4: f64 = *var_vgsfp4_slot;
        let mut var_vgsfp4_dn16: f64 = *var_vgsfp4_dn16_slot;
        let mut var_vgsfp4_dn2: f64 = *var_vgsfp4_dn2_slot;
        let mut var_vgsfp4_dn7: f64 = *var_vgsfp4_dn7_slot;
        let mut var_vgsfp4_rv: f64 = *var_vgsfp4_rv_slot;
        let mut var_vgsfps3: f64 = *var_vgsfps3_slot;
        let mut var_vgsfps3_dn12: f64 = *var_vgsfps3_dn12_slot;
        let mut var_vgsfps3_dn2: f64 = *var_vgsfps3_dn2_slot;
        let mut var_vgsfps3_dn7: f64 = *var_vgsfps3_dn7_slot;
        let mut var_vgsfps3_rv: f64 = *var_vgsfps3_rv_slot;
        let mut var_vgsfps4: f64 = *var_vgsfps4_slot;
        let mut var_vgsfps4_dn13: f64 = *var_vgsfps4_dn13_slot;
        let mut var_vgsfps4_dn2: f64 = *var_vgsfps4_dn2_slot;
        let mut var_vgsfps4_dn7: f64 = *var_vgsfps4_dn7_slot;
        let mut var_vgsfps4_rv: f64 = *var_vgsfps4_rv_slot;

        let (assign1260_e3673, assign1260_e3673_d_n2, assign1260_e3673_d_n7, assign1260_e3673_d_n12,) = {
    if (var_guard18 != 0.0) {
        let assign1260_e3671: f64 = (p.p6 * (nv7 - nv12));
        (assign1260_e3671, 0.0, p.p6, (-p.p6),)
    } else {
        (var_vgsfps3, var_vgsfps3_dn2, var_vgsfps3_dn7, var_vgsfps3_dn12,)
    }
};
        var_vgsfps3 = assign1260_e3673;
        var_vgsfps3_dn2 = assign1260_e3673_d_n2;
        var_vgsfps3_dn7 = assign1260_e3673_d_n7;
        var_vgsfps3_dn12 = assign1260_e3673_d_n12;
        var_vgsfps3_rv = 0.0;

        let (assign1270_e3679, assign1270_e3679_d_n2, assign1270_e3679_d_n7, assign1270_e3679_d_n12,) = {
    if (var_guard18 != 0.0) {
        let assign1270_e3677: f64 = (p.p6 * (nv2 - nv12));
        (assign1270_e3677, p.p6, 0.0, (-p.p6),)
    } else {
        (var_vcfps3, var_vcfps3_dn2, var_vcfps3_dn7, var_vcfps3_dn12,)
    }
};
        var_vcfps3 = assign1270_e3679;
        var_vcfps3_dn2 = assign1270_e3679_d_n2;
        var_vcfps3_dn7 = assign1270_e3679_d_n7;
        var_vcfps3_dn12 = assign1270_e3679_d_n12;
        var_vcfps3_rv = 0.0;

        let (assign1280_e3686, assign1280_e3686_d_n2, assign1280_e3686_d_n7, assign1280_e3686_d_n12,) = {
    if (var_guard18 == 0.0) {
        let assign1280_e3684: f64 = (p.p6 * (nv2 - nv12));
        (assign1280_e3684, p.p6, 0.0, (-p.p6),)
    } else {
        (var_vgsfps3, var_vgsfps3_dn2, var_vgsfps3_dn7, var_vgsfps3_dn12,)
    }
};
        var_vgsfps3 = assign1280_e3686;
        var_vgsfps3_dn2 = assign1280_e3686_d_n2;
        var_vgsfps3_dn7 = assign1280_e3686_d_n7;
        var_vgsfps3_dn12 = assign1280_e3686_d_n12;
        var_vgsfps3_rv = 0.0;

        let (assign1290_e3693, assign1290_e3693_d_n2, assign1290_e3693_d_n7, assign1290_e3693_d_n12,) = {
    if (var_guard18 == 0.0) {
        let assign1290_e3691: f64 = (p.p6 * (nv7 - nv12));
        (assign1290_e3691, 0.0, p.p6, (-p.p6),)
    } else {
        (var_vcfps3, var_vcfps3_dn2, var_vcfps3_dn7, var_vcfps3_dn12,)
    }
};
        var_vcfps3 = assign1290_e3693;
        var_vcfps3_dn2 = assign1290_e3693_d_n2;
        var_vcfps3_dn7 = assign1290_e3693_d_n7;
        var_vcfps3_dn12 = assign1290_e3693_d_n12;
        var_vcfps3_rv = 0.0;

        let assign1300_e3696: f64 = (p.p6 * (nv11 - nv12));
        var_vdsfps3 = assign1300_e3696;
        var_vdsfps3_dn11 = p.p6;
        var_vdsfps3_dn12 = (-p.p6);
        var_vdsfps3_rv = 0.0;

        let assign1310_e3699: f64 = (p.p6 * (nv3 - nv12));
        var_vbfps3 = assign1310_e3699;
        var_vbfps3_dn3 = p.p6;
        var_vbfps3_dn12 = (-p.p6);
        var_vbfps3_rv = 0.0;

        let assign1320_e3702: f64 = if p.p144 == 1.0 { 1.0 } else { 0.0 };
        var_guard19 = assign1320_e3702;
        var_guard19_rv = 0.0;

        let (assign1330_e3708, assign1330_e3708_d_n2, assign1330_e3708_d_n7, assign1330_e3708_d_n13,) = {
    if (var_guard19 != 0.0) {
        let assign1330_e3706: f64 = (p.p6 * (nv7 - nv13));
        (assign1330_e3706, 0.0, p.p6, (-p.p6),)
    } else {
        (var_vgsfps4, var_vgsfps4_dn2, var_vgsfps4_dn7, var_vgsfps4_dn13,)
    }
};
        var_vgsfps4 = assign1330_e3708;
        var_vgsfps4_dn2 = assign1330_e3708_d_n2;
        var_vgsfps4_dn7 = assign1330_e3708_d_n7;
        var_vgsfps4_dn13 = assign1330_e3708_d_n13;
        var_vgsfps4_rv = 0.0;

        let (assign1340_e3714, assign1340_e3714_d_n2, assign1340_e3714_d_n7, assign1340_e3714_d_n13,) = {
    if (var_guard19 != 0.0) {
        let assign1340_e3712: f64 = (p.p6 * (nv2 - nv13));
        (assign1340_e3712, p.p6, 0.0, (-p.p6),)
    } else {
        (var_vcfps4, var_vcfps4_dn2, var_vcfps4_dn7, var_vcfps4_dn13,)
    }
};
        var_vcfps4 = assign1340_e3714;
        var_vcfps4_dn2 = assign1340_e3714_d_n2;
        var_vcfps4_dn7 = assign1340_e3714_d_n7;
        var_vcfps4_dn13 = assign1340_e3714_d_n13;
        var_vcfps4_rv = 0.0;

        let (assign1350_e3721, assign1350_e3721_d_n2, assign1350_e3721_d_n7, assign1350_e3721_d_n13,) = {
    if (var_guard19 == 0.0) {
        let assign1350_e3719: f64 = (p.p6 * (nv2 - nv13));
        (assign1350_e3719, p.p6, 0.0, (-p.p6),)
    } else {
        (var_vgsfps4, var_vgsfps4_dn2, var_vgsfps4_dn7, var_vgsfps4_dn13,)
    }
};
        var_vgsfps4 = assign1350_e3721;
        var_vgsfps4_dn2 = assign1350_e3721_d_n2;
        var_vgsfps4_dn7 = assign1350_e3721_d_n7;
        var_vgsfps4_dn13 = assign1350_e3721_d_n13;
        var_vgsfps4_rv = 0.0;

        let (assign1360_e3728, assign1360_e3728_d_n2, assign1360_e3728_d_n7, assign1360_e3728_d_n13,) = {
    if (var_guard19 == 0.0) {
        let assign1360_e3726: f64 = (p.p6 * (nv7 - nv13));
        (assign1360_e3726, 0.0, p.p6, (-p.p6),)
    } else {
        (var_vcfps4, var_vcfps4_dn2, var_vcfps4_dn7, var_vcfps4_dn13,)
    }
};
        var_vcfps4 = assign1360_e3728;
        var_vcfps4_dn2 = assign1360_e3728_d_n2;
        var_vcfps4_dn7 = assign1360_e3728_d_n7;
        var_vcfps4_dn13 = assign1360_e3728_d_n13;
        var_vcfps4_rv = 0.0;

        let assign1370_e3731: f64 = (p.p6 * (nv12 - nv13));
        var_vdsfps4 = assign1370_e3731;
        var_vdsfps4_dn12 = p.p6;
        var_vdsfps4_dn13 = (-p.p6);
        var_vdsfps4_rv = 0.0;

        let assign1380_e3734: f64 = (p.p6 * (nv3 - nv13));
        var_vbfps4 = assign1380_e3734;
        var_vbfps4_dn3 = p.p6;
        var_vbfps4_dn13 = (-p.p6);
        var_vbfps4_rv = 0.0;

        let assign1390_e3737: f64 = if p.p166 == 1.0 { 1.0 } else { 0.0 };
        var_guard20 = assign1390_e3737;
        var_guard20_rv = 0.0;

        let (assign1400_e3743, assign1400_e3743_d_n2, assign1400_e3743_d_n5, assign1400_e3743_d_n7,) = {
    if (var_guard20 != 0.0) {
        let assign1400_e3741: f64 = (p.p6 * (nv7 - nv5));
        (assign1400_e3741, 0.0, (-p.p6), p.p6,)
    } else {
        (var_vgsfp1, var_vgsfp1_dn2, var_vgsfp1_dn5, var_vgsfp1_dn7,)
    }
};
        var_vgsfp1 = assign1400_e3743;
        var_vgsfp1_dn2 = assign1400_e3743_d_n2;
        var_vgsfp1_dn5 = assign1400_e3743_d_n5;
        var_vgsfp1_dn7 = assign1400_e3743_d_n7;
        var_vgsfp1_rv = 0.0;

        let (assign1410_e3749, assign1410_e3749_d_n2, assign1410_e3749_d_n5, assign1410_e3749_d_n7,) = {
    if (var_guard20 != 0.0) {
        let assign1410_e3747: f64 = (p.p6 * (nv2 - nv5));
        (assign1410_e3747, p.p6, (-p.p6), 0.0,)
    } else {
        (var_vcfp1, var_vcfp1_dn2, var_vcfp1_dn5, var_vcfp1_dn7,)
    }
};
        var_vcfp1 = assign1410_e3749;
        var_vcfp1_dn2 = assign1410_e3749_d_n2;
        var_vcfp1_dn5 = assign1410_e3749_d_n5;
        var_vcfp1_dn7 = assign1410_e3749_d_n7;
        var_vcfp1_rv = 0.0;

        let (assign1420_e3756, assign1420_e3756_d_n2, assign1420_e3756_d_n5, assign1420_e3756_d_n7,) = {
    if (var_guard20 == 0.0) {
        let assign1420_e3754: f64 = (p.p6 * (nv2 - nv5));
        (assign1420_e3754, p.p6, (-p.p6), 0.0,)
    } else {
        (var_vgsfp1, var_vgsfp1_dn2, var_vgsfp1_dn5, var_vgsfp1_dn7,)
    }
};
        var_vgsfp1 = assign1420_e3756;
        var_vgsfp1_dn2 = assign1420_e3756_d_n2;
        var_vgsfp1_dn5 = assign1420_e3756_d_n5;
        var_vgsfp1_dn7 = assign1420_e3756_d_n7;
        var_vgsfp1_rv = 0.0;

        let (assign1430_e3763, assign1430_e3763_d_n2, assign1430_e3763_d_n5, assign1430_e3763_d_n7,) = {
    if (var_guard20 == 0.0) {
        let assign1430_e3761: f64 = (p.p6 * (nv7 - nv5));
        (assign1430_e3761, 0.0, (-p.p6), p.p6,)
    } else {
        (var_vcfp1, var_vcfp1_dn2, var_vcfp1_dn5, var_vcfp1_dn7,)
    }
};
        var_vcfp1 = assign1430_e3763;
        var_vcfp1_dn2 = assign1430_e3763_d_n2;
        var_vcfp1_dn5 = assign1430_e3763_d_n5;
        var_vcfp1_dn7 = assign1430_e3763_d_n7;
        var_vcfp1_rv = 0.0;

        let assign1440_e3766: f64 = (p.p6 * (nv14 - nv5));
        var_vdsfp1 = assign1440_e3766;
        var_vdsfp1_dn5 = (-p.p6);
        var_vdsfp1_dn14 = p.p6;
        var_vdsfp1_rv = 0.0;

        let assign1450_e3769: f64 = (p.p6 * (nv3 - nv5));
        var_vbfp1 = assign1450_e3769;
        var_vbfp1_dn3 = p.p6;
        var_vbfp1_dn5 = (-p.p6);
        var_vbfp1_rv = 0.0;

        let assign1460_e3772: f64 = if p.p188 == 1.0 { 1.0 } else { 0.0 };
        var_guard21 = assign1460_e3772;
        var_guard21_rv = 0.0;

        let (assign1470_e3778, assign1470_e3778_d_n2, assign1470_e3778_d_n7, assign1470_e3778_d_n14,) = {
    if (var_guard21 != 0.0) {
        let assign1470_e3776: f64 = (p.p6 * (nv7 - nv14));
        (assign1470_e3776, 0.0, p.p6, (-p.p6),)
    } else {
        (var_vgsfp2, var_vgsfp2_dn2, var_vgsfp2_dn7, var_vgsfp2_dn14,)
    }
};
        var_vgsfp2 = assign1470_e3778;
        var_vgsfp2_dn2 = assign1470_e3778_d_n2;
        var_vgsfp2_dn7 = assign1470_e3778_d_n7;
        var_vgsfp2_dn14 = assign1470_e3778_d_n14;
        var_vgsfp2_rv = 0.0;

        let (assign1480_e3784, assign1480_e3784_d_n2, assign1480_e3784_d_n7, assign1480_e3784_d_n14,) = {
    if (var_guard21 != 0.0) {
        let assign1480_e3782: f64 = (p.p6 * (nv2 - nv14));
        (assign1480_e3782, p.p6, 0.0, (-p.p6),)
    } else {
        (var_vcfp2, var_vcfp2_dn2, var_vcfp2_dn7, var_vcfp2_dn14,)
    }
};
        var_vcfp2 = assign1480_e3784;
        var_vcfp2_dn2 = assign1480_e3784_d_n2;
        var_vcfp2_dn7 = assign1480_e3784_d_n7;
        var_vcfp2_dn14 = assign1480_e3784_d_n14;
        var_vcfp2_rv = 0.0;

        let (assign1490_e3791, assign1490_e3791_d_n2, assign1490_e3791_d_n7, assign1490_e3791_d_n14,) = {
    if (var_guard21 == 0.0) {
        let assign1490_e3789: f64 = (p.p6 * (nv2 - nv14));
        (assign1490_e3789, p.p6, 0.0, (-p.p6),)
    } else {
        (var_vgsfp2, var_vgsfp2_dn2, var_vgsfp2_dn7, var_vgsfp2_dn14,)
    }
};
        var_vgsfp2 = assign1490_e3791;
        var_vgsfp2_dn2 = assign1490_e3791_d_n2;
        var_vgsfp2_dn7 = assign1490_e3791_d_n7;
        var_vgsfp2_dn14 = assign1490_e3791_d_n14;
        var_vgsfp2_rv = 0.0;

        let (assign1500_e3798, assign1500_e3798_d_n2, assign1500_e3798_d_n7, assign1500_e3798_d_n14,) = {
    if (var_guard21 == 0.0) {
        let assign1500_e3796: f64 = (p.p6 * (nv7 - nv14));
        (assign1500_e3796, 0.0, p.p6, (-p.p6),)
    } else {
        (var_vcfp2, var_vcfp2_dn2, var_vcfp2_dn7, var_vcfp2_dn14,)
    }
};
        var_vcfp2 = assign1500_e3798;
        var_vcfp2_dn2 = assign1500_e3798_d_n2;
        var_vcfp2_dn7 = assign1500_e3798_d_n7;
        var_vcfp2_dn14 = assign1500_e3798_d_n14;
        var_vcfp2_rv = 0.0;

        let assign1510_e3801: f64 = (p.p6 * (nv15 - nv14));
        var_vdsfp2 = assign1510_e3801;
        var_vdsfp2_dn14 = (-p.p6);
        var_vdsfp2_dn15 = p.p6;
        var_vdsfp2_rv = 0.0;

        let assign1520_e3804: f64 = (p.p6 * (nv3 - nv14));
        var_vbfp2 = assign1520_e3804;
        var_vbfp2_dn3 = p.p6;
        var_vbfp2_dn14 = (-p.p6);
        var_vbfp2_rv = 0.0;

        let assign1530_e3807: f64 = if p.p210 == 1.0 { 1.0 } else { 0.0 };
        var_guard22 = assign1530_e3807;
        var_guard22_rv = 0.0;

        let (assign1540_e3813, assign1540_e3813_d_n2, assign1540_e3813_d_n7, assign1540_e3813_d_n15,) = {
    if (var_guard22 != 0.0) {
        let assign1540_e3811: f64 = (p.p6 * (nv7 - nv15));
        (assign1540_e3811, 0.0, p.p6, (-p.p6),)
    } else {
        (var_vgsfp3, var_vgsfp3_dn2, var_vgsfp3_dn7, var_vgsfp3_dn15,)
    }
};
        var_vgsfp3 = assign1540_e3813;
        var_vgsfp3_dn2 = assign1540_e3813_d_n2;
        var_vgsfp3_dn7 = assign1540_e3813_d_n7;
        var_vgsfp3_dn15 = assign1540_e3813_d_n15;
        var_vgsfp3_rv = 0.0;

        let (assign1550_e3819, assign1550_e3819_d_n2, assign1550_e3819_d_n7, assign1550_e3819_d_n15,) = {
    if (var_guard22 != 0.0) {
        let assign1550_e3817: f64 = (p.p6 * (nv2 - nv15));
        (assign1550_e3817, p.p6, 0.0, (-p.p6),)
    } else {
        (var_vcfp3, var_vcfp3_dn2, var_vcfp3_dn7, var_vcfp3_dn15,)
    }
};
        var_vcfp3 = assign1550_e3819;
        var_vcfp3_dn2 = assign1550_e3819_d_n2;
        var_vcfp3_dn7 = assign1550_e3819_d_n7;
        var_vcfp3_dn15 = assign1550_e3819_d_n15;
        var_vcfp3_rv = 0.0;

        let (assign1560_e3826, assign1560_e3826_d_n2, assign1560_e3826_d_n7, assign1560_e3826_d_n15,) = {
    if (var_guard22 == 0.0) {
        let assign1560_e3824: f64 = (p.p6 * (nv2 - nv15));
        (assign1560_e3824, p.p6, 0.0, (-p.p6),)
    } else {
        (var_vgsfp3, var_vgsfp3_dn2, var_vgsfp3_dn7, var_vgsfp3_dn15,)
    }
};
        var_vgsfp3 = assign1560_e3826;
        var_vgsfp3_dn2 = assign1560_e3826_d_n2;
        var_vgsfp3_dn7 = assign1560_e3826_d_n7;
        var_vgsfp3_dn15 = assign1560_e3826_d_n15;
        var_vgsfp3_rv = 0.0;

        let (assign1570_e3833, assign1570_e3833_d_n2, assign1570_e3833_d_n7, assign1570_e3833_d_n15,) = {
    if (var_guard22 == 0.0) {
        let assign1570_e3831: f64 = (p.p6 * (nv7 - nv15));
        (assign1570_e3831, 0.0, p.p6, (-p.p6),)
    } else {
        (var_vcfp3, var_vcfp3_dn2, var_vcfp3_dn7, var_vcfp3_dn15,)
    }
};
        var_vcfp3 = assign1570_e3833;
        var_vcfp3_dn2 = assign1570_e3833_d_n2;
        var_vcfp3_dn7 = assign1570_e3833_d_n7;
        var_vcfp3_dn15 = assign1570_e3833_d_n15;
        var_vcfp3_rv = 0.0;

        let assign1580_e3836: f64 = (p.p6 * (nv16 - nv15));
        var_vdsfp3 = assign1580_e3836;
        var_vdsfp3_dn15 = (-p.p6);
        var_vdsfp3_dn16 = p.p6;
        var_vdsfp3_rv = 0.0;

        let assign1590_e3839: f64 = (p.p6 * (nv3 - nv15));
        var_vbfp3 = assign1590_e3839;
        var_vbfp3_dn3 = p.p6;
        var_vbfp3_dn15 = (-p.p6);
        var_vbfp3_rv = 0.0;

        let assign1600_e3842: f64 = if p.p232 == 1.0 { 1.0 } else { 0.0 };
        var_guard23 = assign1600_e3842;
        var_guard23_rv = 0.0;

        let (assign1610_e3848, assign1610_e3848_d_n2, assign1610_e3848_d_n7, assign1610_e3848_d_n16,) = {
    if (var_guard23 != 0.0) {
        let assign1610_e3846: f64 = (p.p6 * (nv7 - nv16));
        (assign1610_e3846, 0.0, p.p6, (-p.p6),)
    } else {
        (var_vgsfp4, var_vgsfp4_dn2, var_vgsfp4_dn7, var_vgsfp4_dn16,)
    }
};
        var_vgsfp4 = assign1610_e3848;
        var_vgsfp4_dn2 = assign1610_e3848_d_n2;
        var_vgsfp4_dn7 = assign1610_e3848_d_n7;
        var_vgsfp4_dn16 = assign1610_e3848_d_n16;
        var_vgsfp4_rv = 0.0;

        let (assign1620_e3854, assign1620_e3854_d_n2, assign1620_e3854_d_n7, assign1620_e3854_d_n16,) = {
    if (var_guard23 != 0.0) {
        let assign1620_e3852: f64 = (p.p6 * (nv2 - nv16));
        (assign1620_e3852, p.p6, 0.0, (-p.p6),)
    } else {
        (var_vcfp4, var_vcfp4_dn2, var_vcfp4_dn7, var_vcfp4_dn16,)
    }
};
        var_vcfp4 = assign1620_e3854;
        var_vcfp4_dn2 = assign1620_e3854_d_n2;
        var_vcfp4_dn7 = assign1620_e3854_d_n7;
        var_vcfp4_dn16 = assign1620_e3854_d_n16;
        var_vcfp4_rv = 0.0;

        let (assign1630_e3861, assign1630_e3861_d_n2, assign1630_e3861_d_n7, assign1630_e3861_d_n16,) = {
    if (var_guard23 == 0.0) {
        let assign1630_e3859: f64 = (p.p6 * (nv2 - nv16));
        (assign1630_e3859, p.p6, 0.0, (-p.p6),)
    } else {
        (var_vgsfp4, var_vgsfp4_dn2, var_vgsfp4_dn7, var_vgsfp4_dn16,)
    }
};
        var_vgsfp4 = assign1630_e3861;
        var_vgsfp4_dn2 = assign1630_e3861_d_n2;
        var_vgsfp4_dn7 = assign1630_e3861_d_n7;
        var_vgsfp4_dn16 = assign1630_e3861_d_n16;
        var_vgsfp4_rv = 0.0;

        let (assign1640_e3868, assign1640_e3868_d_n2, assign1640_e3868_d_n7, assign1640_e3868_d_n16,) = {
    if (var_guard23 == 0.0) {
        let assign1640_e3866: f64 = (p.p6 * (nv7 - nv16));
        (assign1640_e3866, 0.0, p.p6, (-p.p6),)
    } else {
        (var_vcfp4, var_vcfp4_dn2, var_vcfp4_dn7, var_vcfp4_dn16,)
    }
};
        var_vcfp4 = assign1640_e3868;
        var_vcfp4_dn2 = assign1640_e3868_d_n2;
        var_vcfp4_dn7 = assign1640_e3868_d_n7;
        var_vcfp4_dn16 = assign1640_e3868_d_n16;
        var_vcfp4_rv = 0.0;

        let assign1650_e3871: f64 = (p.p6 * (nv17 - nv16));
        var_vdsfp4 = assign1650_e3871;
        var_vdsfp4_dn16 = (-p.p6);
        var_vdsfp4_dn17 = p.p6;
        var_vdsfp4_rv = 0.0;

        let assign1660_e3874: f64 = (p.p6 * (nv3 - nv16));
        var_vbfp4 = assign1660_e3874;
        var_vbfp4_dn3 = p.p6;
        var_vbfp4_dn16 = (-p.p6);
        var_vbfp4_rv = 0.0;

        var_qgsfp4 = 0.0;
        var_qgsfp4_dn2 = 0.0;
        var_qgsfp4_dn4 = 0.0;
        var_qgsfp4_dn7 = 0.0;
        var_qgsfp4_dn16 = 0.0;
        var_qgsfp4_dn17 = 0.0;
        var_qgsfp4_rv = 0.0;

        var_qgdfp4 = 0.0;
        var_qgdfp4_dn2 = 0.0;
        var_qgdfp4_dn4 = 0.0;
        var_qgdfp4_dn7 = 0.0;
        var_qgdfp4_dn16 = 0.0;
        var_qgdfp4_dn17 = 0.0;
        var_qgdfp4_rv = 0.0;

        var_qcfp4 = 0.0;
        var_qcfp4_dn2 = 0.0;
        var_qcfp4_dn3 = 0.0;
        var_qcfp4_dn4 = 0.0;
        var_qcfp4_dn7 = 0.0;
        var_qcfp4_dn16 = 0.0;
        var_qcfp4_dn17 = 0.0;
        var_qcfp4_rv = 0.0;

        var_qbfp4 = 0.0;
        var_qbfp4_dn2 = 0.0;
        var_qbfp4_dn3 = 0.0;
        var_qbfp4_dn4 = 0.0;
        var_qbfp4_dn7 = 0.0;
        var_qbfp4_dn16 = 0.0;
        var_qbfp4_dn17 = 0.0;
        var_qbfp4_rv = 0.0;

        var_qsfp4 = 0.0;
        var_qsfp4_dn2 = 0.0;
        var_qsfp4_dn3 = 0.0;
        var_qsfp4_dn4 = 0.0;
        var_qsfp4_dn7 = 0.0;
        var_qsfp4_dn16 = 0.0;
        var_qsfp4_dn17 = 0.0;
        var_qsfp4_rv = 0.0;

        let assign1750_e3885: f64 = if p.p233 > p.p354 { 1.0 } else { 0.0 };
        var_guard24 = assign1750_e3885;
        var_guard24_rv = 0.0;

        let (assign1780_e3897, assign1780_e3897_d_n2, assign1780_e3897_d_n4, assign1780_e3897_d_n7, assign1780_e3897_d_n16, assign1780_e3897_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__qgsout, var_fn25_calc_iq__qgsout_dn2, var_fn25_calc_iq__qgsout_dn4, var_fn25_calc_iq__qgsout_dn7, var_fn25_calc_iq__qgsout_dn16, var_fn25_calc_iq__qgsout_dn17,)
    }
};
        var_fn25_calc_iq__qgsout = assign1780_e3897;
        var_fn25_calc_iq__qgsout_dn2 = assign1780_e3897_d_n2;
        var_fn25_calc_iq__qgsout_dn4 = assign1780_e3897_d_n4;
        var_fn25_calc_iq__qgsout_dn7 = assign1780_e3897_d_n7;
        var_fn25_calc_iq__qgsout_dn16 = assign1780_e3897_d_n16;
        var_fn25_calc_iq__qgsout_dn17 = assign1780_e3897_d_n17;
        var_fn25_calc_iq__qgsout_rv = 0.0;

        let (assign1790_e3901, assign1790_e3901_d_n2, assign1790_e3901_d_n4, assign1790_e3901_d_n7, assign1790_e3901_d_n16, assign1790_e3901_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__qgdout, var_fn25_calc_iq__qgdout_dn2, var_fn25_calc_iq__qgdout_dn4, var_fn25_calc_iq__qgdout_dn7, var_fn25_calc_iq__qgdout_dn16, var_fn25_calc_iq__qgdout_dn17,)
    }
};
        var_fn25_calc_iq__qgdout = assign1790_e3901;
        var_fn25_calc_iq__qgdout_dn2 = assign1790_e3901_d_n2;
        var_fn25_calc_iq__qgdout_dn4 = assign1790_e3901_d_n4;
        var_fn25_calc_iq__qgdout_dn7 = assign1790_e3901_d_n7;
        var_fn25_calc_iq__qgdout_dn16 = assign1790_e3901_d_n16;
        var_fn25_calc_iq__qgdout_dn17 = assign1790_e3901_d_n17;
        var_fn25_calc_iq__qgdout_rv = 0.0;

        *var_fn25_calc_iq__qgdout_slot = var_fn25_calc_iq__qgdout;
        *var_fn25_calc_iq__qgdout_dn16_slot = var_fn25_calc_iq__qgdout_dn16;
        *var_fn25_calc_iq__qgdout_dn17_slot = var_fn25_calc_iq__qgdout_dn17;
        *var_fn25_calc_iq__qgdout_dn2_slot = var_fn25_calc_iq__qgdout_dn2;
        *var_fn25_calc_iq__qgdout_dn4_slot = var_fn25_calc_iq__qgdout_dn4;
        *var_fn25_calc_iq__qgdout_dn7_slot = var_fn25_calc_iq__qgdout_dn7;
        *var_fn25_calc_iq__qgdout_rv_slot = var_fn25_calc_iq__qgdout_rv;
        *var_fn25_calc_iq__qgsout_slot = var_fn25_calc_iq__qgsout;
        *var_fn25_calc_iq__qgsout_dn16_slot = var_fn25_calc_iq__qgsout_dn16;
        *var_fn25_calc_iq__qgsout_dn17_slot = var_fn25_calc_iq__qgsout_dn17;
        *var_fn25_calc_iq__qgsout_dn2_slot = var_fn25_calc_iq__qgsout_dn2;
        *var_fn25_calc_iq__qgsout_dn4_slot = var_fn25_calc_iq__qgsout_dn4;
        *var_fn25_calc_iq__qgsout_dn7_slot = var_fn25_calc_iq__qgsout_dn7;
        *var_fn25_calc_iq__qgsout_rv_slot = var_fn25_calc_iq__qgsout_rv;
        *var_guard19_slot = var_guard19;
        *var_guard19_rv_slot = var_guard19_rv;
        *var_guard20_slot = var_guard20;
        *var_guard20_rv_slot = var_guard20_rv;
        *var_guard21_slot = var_guard21;
        *var_guard21_rv_slot = var_guard21_rv;
        *var_guard22_slot = var_guard22;
        *var_guard22_rv_slot = var_guard22_rv;
        *var_guard23_slot = var_guard23;
        *var_guard23_rv_slot = var_guard23_rv;
        *var_guard24_slot = var_guard24;
        *var_guard24_rv_slot = var_guard24_rv;
        *var_qbfp4_slot = var_qbfp4;
        *var_qbfp4_dn16_slot = var_qbfp4_dn16;
        *var_qbfp4_dn17_slot = var_qbfp4_dn17;
        *var_qbfp4_dn2_slot = var_qbfp4_dn2;
        *var_qbfp4_dn3_slot = var_qbfp4_dn3;
        *var_qbfp4_dn4_slot = var_qbfp4_dn4;
        *var_qbfp4_dn7_slot = var_qbfp4_dn7;
        *var_qbfp4_rv_slot = var_qbfp4_rv;
        *var_qcfp4_slot = var_qcfp4;
        *var_qcfp4_dn16_slot = var_qcfp4_dn16;
        *var_qcfp4_dn17_slot = var_qcfp4_dn17;
        *var_qcfp4_dn2_slot = var_qcfp4_dn2;
        *var_qcfp4_dn3_slot = var_qcfp4_dn3;
        *var_qcfp4_dn4_slot = var_qcfp4_dn4;
        *var_qcfp4_dn7_slot = var_qcfp4_dn7;
        *var_qcfp4_rv_slot = var_qcfp4_rv;
        *var_qgdfp4_slot = var_qgdfp4;
        *var_qgdfp4_dn16_slot = var_qgdfp4_dn16;
        *var_qgdfp4_dn17_slot = var_qgdfp4_dn17;
        *var_qgdfp4_dn2_slot = var_qgdfp4_dn2;
        *var_qgdfp4_dn4_slot = var_qgdfp4_dn4;
        *var_qgdfp4_dn7_slot = var_qgdfp4_dn7;
        *var_qgdfp4_rv_slot = var_qgdfp4_rv;
        *var_qgsfp4_slot = var_qgsfp4;
        *var_qgsfp4_dn16_slot = var_qgsfp4_dn16;
        *var_qgsfp4_dn17_slot = var_qgsfp4_dn17;
        *var_qgsfp4_dn2_slot = var_qgsfp4_dn2;
        *var_qgsfp4_dn4_slot = var_qgsfp4_dn4;
        *var_qgsfp4_dn7_slot = var_qgsfp4_dn7;
        *var_qgsfp4_rv_slot = var_qgsfp4_rv;
        *var_qsfp4_slot = var_qsfp4;
        *var_qsfp4_dn16_slot = var_qsfp4_dn16;
        *var_qsfp4_dn17_slot = var_qsfp4_dn17;
        *var_qsfp4_dn2_slot = var_qsfp4_dn2;
        *var_qsfp4_dn3_slot = var_qsfp4_dn3;
        *var_qsfp4_dn4_slot = var_qsfp4_dn4;
        *var_qsfp4_dn7_slot = var_qsfp4_dn7;
        *var_qsfp4_rv_slot = var_qsfp4_rv;
        *var_vbfp1_slot = var_vbfp1;
        *var_vbfp1_dn3_slot = var_vbfp1_dn3;
        *var_vbfp1_dn5_slot = var_vbfp1_dn5;
        *var_vbfp1_rv_slot = var_vbfp1_rv;
        *var_vbfp2_slot = var_vbfp2;
        *var_vbfp2_dn14_slot = var_vbfp2_dn14;
        *var_vbfp2_dn3_slot = var_vbfp2_dn3;
        *var_vbfp2_rv_slot = var_vbfp2_rv;
        *var_vbfp3_slot = var_vbfp3;
        *var_vbfp3_dn15_slot = var_vbfp3_dn15;
        *var_vbfp3_dn3_slot = var_vbfp3_dn3;
        *var_vbfp3_rv_slot = var_vbfp3_rv;
        *var_vbfp4_slot = var_vbfp4;
        *var_vbfp4_dn16_slot = var_vbfp4_dn16;
        *var_vbfp4_dn3_slot = var_vbfp4_dn3;
        *var_vbfp4_rv_slot = var_vbfp4_rv;
        *var_vbfps3_slot = var_vbfps3;
        *var_vbfps3_dn12_slot = var_vbfps3_dn12;
        *var_vbfps3_dn3_slot = var_vbfps3_dn3;
        *var_vbfps3_rv_slot = var_vbfps3_rv;
        *var_vbfps4_slot = var_vbfps4;
        *var_vbfps4_dn13_slot = var_vbfps4_dn13;
        *var_vbfps4_dn3_slot = var_vbfps4_dn3;
        *var_vbfps4_rv_slot = var_vbfps4_rv;
        *var_vcfp1_slot = var_vcfp1;
        *var_vcfp1_dn2_slot = var_vcfp1_dn2;
        *var_vcfp1_dn5_slot = var_vcfp1_dn5;
        *var_vcfp1_dn7_slot = var_vcfp1_dn7;
        *var_vcfp1_rv_slot = var_vcfp1_rv;
        *var_vcfp2_slot = var_vcfp2;
        *var_vcfp2_dn14_slot = var_vcfp2_dn14;
        *var_vcfp2_dn2_slot = var_vcfp2_dn2;
        *var_vcfp2_dn7_slot = var_vcfp2_dn7;
        *var_vcfp2_rv_slot = var_vcfp2_rv;
        *var_vcfp3_slot = var_vcfp3;
        *var_vcfp3_dn15_slot = var_vcfp3_dn15;
        *var_vcfp3_dn2_slot = var_vcfp3_dn2;
        *var_vcfp3_dn7_slot = var_vcfp3_dn7;
        *var_vcfp3_rv_slot = var_vcfp3_rv;
        *var_vcfp4_slot = var_vcfp4;
        *var_vcfp4_dn16_slot = var_vcfp4_dn16;
        *var_vcfp4_dn2_slot = var_vcfp4_dn2;
        *var_vcfp4_dn7_slot = var_vcfp4_dn7;
        *var_vcfp4_rv_slot = var_vcfp4_rv;
        *var_vcfps3_slot = var_vcfps3;
        *var_vcfps3_dn12_slot = var_vcfps3_dn12;
        *var_vcfps3_dn2_slot = var_vcfps3_dn2;
        *var_vcfps3_dn7_slot = var_vcfps3_dn7;
        *var_vcfps3_rv_slot = var_vcfps3_rv;
        *var_vcfps4_slot = var_vcfps4;
        *var_vcfps4_dn13_slot = var_vcfps4_dn13;
        *var_vcfps4_dn2_slot = var_vcfps4_dn2;
        *var_vcfps4_dn7_slot = var_vcfps4_dn7;
        *var_vcfps4_rv_slot = var_vcfps4_rv;
        *var_vdsfp1_slot = var_vdsfp1;
        *var_vdsfp1_dn14_slot = var_vdsfp1_dn14;
        *var_vdsfp1_dn5_slot = var_vdsfp1_dn5;
        *var_vdsfp1_rv_slot = var_vdsfp1_rv;
        *var_vdsfp2_slot = var_vdsfp2;
        *var_vdsfp2_dn14_slot = var_vdsfp2_dn14;
        *var_vdsfp2_dn15_slot = var_vdsfp2_dn15;
        *var_vdsfp2_rv_slot = var_vdsfp2_rv;
        *var_vdsfp3_slot = var_vdsfp3;
        *var_vdsfp3_dn15_slot = var_vdsfp3_dn15;
        *var_vdsfp3_dn16_slot = var_vdsfp3_dn16;
        *var_vdsfp3_rv_slot = var_vdsfp3_rv;
        *var_vdsfp4_slot = var_vdsfp4;
        *var_vdsfp4_dn16_slot = var_vdsfp4_dn16;
        *var_vdsfp4_dn17_slot = var_vdsfp4_dn17;
        *var_vdsfp4_rv_slot = var_vdsfp4_rv;
        *var_vdsfps3_slot = var_vdsfps3;
        *var_vdsfps3_dn11_slot = var_vdsfps3_dn11;
        *var_vdsfps3_dn12_slot = var_vdsfps3_dn12;
        *var_vdsfps3_rv_slot = var_vdsfps3_rv;
        *var_vdsfps4_slot = var_vdsfps4;
        *var_vdsfps4_dn12_slot = var_vdsfps4_dn12;
        *var_vdsfps4_dn13_slot = var_vdsfps4_dn13;
        *var_vdsfps4_rv_slot = var_vdsfps4_rv;
        *var_vgsfp1_slot = var_vgsfp1;
        *var_vgsfp1_dn2_slot = var_vgsfp1_dn2;
        *var_vgsfp1_dn5_slot = var_vgsfp1_dn5;
        *var_vgsfp1_dn7_slot = var_vgsfp1_dn7;
        *var_vgsfp1_rv_slot = var_vgsfp1_rv;
        *var_vgsfp2_slot = var_vgsfp2;
        *var_vgsfp2_dn14_slot = var_vgsfp2_dn14;
        *var_vgsfp2_dn2_slot = var_vgsfp2_dn2;
        *var_vgsfp2_dn7_slot = var_vgsfp2_dn7;
        *var_vgsfp2_rv_slot = var_vgsfp2_rv;
        *var_vgsfp3_slot = var_vgsfp3;
        *var_vgsfp3_dn15_slot = var_vgsfp3_dn15;
        *var_vgsfp3_dn2_slot = var_vgsfp3_dn2;
        *var_vgsfp3_dn7_slot = var_vgsfp3_dn7;
        *var_vgsfp3_rv_slot = var_vgsfp3_rv;
        *var_vgsfp4_slot = var_vgsfp4;
        *var_vgsfp4_dn16_slot = var_vgsfp4_dn16;
        *var_vgsfp4_dn2_slot = var_vgsfp4_dn2;
        *var_vgsfp4_dn7_slot = var_vgsfp4_dn7;
        *var_vgsfp4_rv_slot = var_vgsfp4_rv;
        *var_vgsfps3_slot = var_vgsfps3;
        *var_vgsfps3_dn12_slot = var_vgsfps3_dn12;
        *var_vgsfps3_dn2_slot = var_vgsfps3_dn2;
        *var_vgsfps3_dn7_slot = var_vgsfps3_dn7;
        *var_vgsfps3_rv_slot = var_vgsfps3_rv;
        *var_vgsfps4_slot = var_vgsfps4;
        *var_vgsfps4_dn13_slot = var_vgsfps4_dn13;
        *var_vgsfps4_dn2_slot = var_vgsfps4_dn2;
        *var_vgsfps4_dn7_slot = var_vgsfps4_dn7;
        *var_vgsfps4_rv_slot = var_vgsfps4_rv;
    }

    pub(super) fn stamp_reactive_block_3(
        p: &Parameters,
        var_cbfp4t: f64,
        var_cbfp4t_dn4: f64,
        var_ccfp4t: f64,
        var_ccfp4t_dn4: f64,
        var_cgfp4t: f64,
        var_cgfp4t_dn4: f64,
        var_guard24: f64,
        var_phit: f64,
        var_phit_dn4: f64,
        var_tdut: f64,
        var_tdut_dn4: f64,
        var_tnomk: f64,
        var_vbfp4: f64,
        var_vbfp4_dn16: f64,
        var_vbfp4_dn3: f64,
        var_vcfp4: f64,
        var_vcfp4_dn16: f64,
        var_vcfp4_dn2: f64,
        var_vcfp4_dn7: f64,
        var_vdsfp4: f64,
        var_vdsfp4_dn16: f64,
        var_vdsfp4_dn17: f64,
        var_vgsfp4: f64,
        var_vgsfp4_dn16: f64,
        var_vgsfp4_dn2: f64,
        var_vgsfp4_dn7: f64,
        var_fn25_calc_iq__alpha_slot: &mut f64,
        var_fn25_calc_iq__alpha_phit_slot: &mut f64,
        var_fn25_calc_iq__alpha_phit_dn4_slot: &mut f64,
        var_fn25_calc_iq__alpha_phit_rv_slot: &mut f64,
        var_fn25_calc_iq__alpha_rv_slot: &mut f64,
        var_fn25_calc_iq__beta_slot: &mut f64,
        var_fn25_calc_iq__beta_rv_slot: &mut f64,
        var_fn25_calc_iq__cb_slot: &mut f64,
        var_fn25_calc_iq__cb_dn4_slot: &mut f64,
        var_fn25_calc_iq__cb_rv_slot: &mut f64,
        var_fn25_calc_iq__cc_slot: &mut f64,
        var_fn25_calc_iq__cc_dn4_slot: &mut f64,
        var_fn25_calc_iq__cc_rv_slot: &mut f64,
        var_fn25_calc_iq__cgin_slot: &mut f64,
        var_fn25_calc_iq__cgin_dn4_slot: &mut f64,
        var_fn25_calc_iq__cgin_rv_slot: &mut f64,
        var_fn25_calc_iq__cs_slot: &mut f64,
        var_fn25_calc_iq__cs_rv_slot: &mut f64,
        var_fn25_calc_iq__delta_slot: &mut f64,
        var_fn25_calc_iq__delta1_slot: &mut f64,
        var_fn25_calc_iq__delta1_rv_slot: &mut f64,
        var_fn25_calc_iq__delta2_slot: &mut f64,
        var_fn25_calc_iq__delta2_rv_slot: &mut f64,
        var_fn25_calc_iq__delta_dn16_slot: &mut f64,
        var_fn25_calc_iq__delta_dn17_slot: &mut f64,
        var_fn25_calc_iq__delta_rv_slot: &mut f64,
        var_fn25_calc_iq__dibsat_slot: &mut f64,
        var_fn25_calc_iq__dibsat_rv_slot: &mut f64,
        var_fn25_calc_iq__epsilon_slot: &mut f64,
        var_fn25_calc_iq__epsilon_rv_slot: &mut f64,
        var_fn25_calc_iq__ffs_slot: &mut f64,
        var_fn25_calc_iq__ffs_dn16_slot: &mut f64,
        var_fn25_calc_iq__ffs_dn17_slot: &mut f64,
        var_fn25_calc_iq__ffs_dn2_slot: &mut f64,
        var_fn25_calc_iq__ffs_dn3_slot: &mut f64,
        var_fn25_calc_iq__ffs_dn4_slot: &mut f64,
        var_fn25_calc_iq__ffs_dn7_slot: &mut f64,
        var_fn25_calc_iq__ffs_rv_slot: &mut f64,
        var_fn25_calc_iq__lambda_slot: &mut f64,
        var_fn25_calc_iq__lambda_rv_slot: &mut f64,
        var_fn25_calc_iq__lin_slot: &mut f64,
        var_fn25_calc_iq__lin_rv_slot: &mut f64,
        var_fn25_calc_iq__mtheta_slot: &mut f64,
        var_fn25_calc_iq__mtheta_rv_slot: &mut f64,
        var_fn25_calc_iq__mu0_slot: &mut f64,
        var_fn25_calc_iq__mu0_rv_slot: &mut f64,
        var_fn25_calc_iq__n_slot: &mut f64,
        var_fn25_calc_iq__n_dn16_slot: &mut f64,
        var_fn25_calc_iq__n_dn17_slot: &mut f64,
        var_fn25_calc_iq__n_dn4_slot: &mut f64,
        var_fn25_calc_iq__n_rv_slot: &mut f64,
        var_fn25_calc_iq__nd_slot: &mut f64,
        var_fn25_calc_iq__nd_rv_slot: &mut f64,
        var_fn25_calc_iq__ngf_slot: &mut f64,
        var_fn25_calc_iq__ngf_rv_slot: &mut f64,
        var_fn25_calc_iq__phitin_slot: &mut f64,
        var_fn25_calc_iq__phitin_dn4_slot: &mut f64,
        var_fn25_calc_iq__phitin_rv_slot: &mut f64,
        var_fn25_calc_iq__qbout_slot: &mut f64,
        var_fn25_calc_iq__qbout_dn16_slot: &mut f64,
        var_fn25_calc_iq__qbout_dn17_slot: &mut f64,
        var_fn25_calc_iq__qbout_dn2_slot: &mut f64,
        var_fn25_calc_iq__qbout_dn3_slot: &mut f64,
        var_fn25_calc_iq__qbout_dn4_slot: &mut f64,
        var_fn25_calc_iq__qbout_dn7_slot: &mut f64,
        var_fn25_calc_iq__qbout_rv_slot: &mut f64,
        var_fn25_calc_iq__qcbflag_slot: &mut f64,
        var_fn25_calc_iq__qcbflag_rv_slot: &mut f64,
        var_fn25_calc_iq__qcout_slot: &mut f64,
        var_fn25_calc_iq__qcout_dn16_slot: &mut f64,
        var_fn25_calc_iq__qcout_dn17_slot: &mut f64,
        var_fn25_calc_iq__qcout_dn2_slot: &mut f64,
        var_fn25_calc_iq__qcout_dn3_slot: &mut f64,
        var_fn25_calc_iq__qcout_dn4_slot: &mut f64,
        var_fn25_calc_iq__qcout_dn7_slot: &mut f64,
        var_fn25_calc_iq__qcout_rv_slot: &mut f64,
        var_fn25_calc_iq__qgsflag_slot: &mut f64,
        var_fn25_calc_iq__qgsflag_rv_slot: &mut f64,
        var_fn25_calc_iq__qsout_slot: &mut f64,
        var_fn25_calc_iq__qsout_dn16_slot: &mut f64,
        var_fn25_calc_iq__qsout_dn17_slot: &mut f64,
        var_fn25_calc_iq__qsout_dn2_slot: &mut f64,
        var_fn25_calc_iq__qsout_dn3_slot: &mut f64,
        var_fn25_calc_iq__qsout_dn4_slot: &mut f64,
        var_fn25_calc_iq__qsout_dn7_slot: &mut f64,
        var_fn25_calc_iq__qsout_rv_slot: &mut f64,
        var_fn25_calc_iq__ss_slot: &mut f64,
        var_fn25_calc_iq__ss_rv_slot: &mut f64,
        var_fn25_calc_iq__tambin_slot: &mut f64,
        var_fn25_calc_iq__tambin_dn4_slot: &mut f64,
        var_fn25_calc_iq__tambin_rv_slot: &mut f64,
        var_fn25_calc_iq__tnomin_slot: &mut f64,
        var_fn25_calc_iq__tnomin_rv_slot: &mut f64,
        var_fn25_calc_iq__trapfracdl_slot: &mut f64,
        var_fn25_calc_iq__trapfracdl_rv_slot: &mut f64,
        var_fn25_calc_iq__type_slot: &mut f64,
        var_fn25_calc_iq__type_rv_slot: &mut f64,
        var_fn25_calc_iq__vbin_slot: &mut f64,
        var_fn25_calc_iq__vbin_dn16_slot: &mut f64,
        var_fn25_calc_iq__vbin_dn3_slot: &mut f64,
        var_fn25_calc_iq__vbin_rv_slot: &mut f64,
        var_fn25_calc_iq__vcin_slot: &mut f64,
        var_fn25_calc_iq__vcin_dn16_slot: &mut f64,
        var_fn25_calc_iq__vcin_dn2_slot: &mut f64,
        var_fn25_calc_iq__vcin_dn7_slot: &mut f64,
        var_fn25_calc_iq__vcin_rv_slot: &mut f64,
        var_fn25_calc_iq__vdsat1_slot: &mut f64,
        var_fn25_calc_iq__vdsat1_dn16_slot: &mut f64,
        var_fn25_calc_iq__vdsat1_dn17_slot: &mut f64,
        var_fn25_calc_iq__vdsat1_dn2_slot: &mut f64,
        var_fn25_calc_iq__vdsat1_dn3_slot: &mut f64,
        var_fn25_calc_iq__vdsat1_dn4_slot: &mut f64,
        var_fn25_calc_iq__vdsat1_dn7_slot: &mut f64,
        var_fn25_calc_iq__vdsat1_rv_slot: &mut f64,
        var_fn25_calc_iq__vdsin_slot: &mut f64,
        var_fn25_calc_iq__vdsin_dn16_slot: &mut f64,
        var_fn25_calc_iq__vdsin_dn17_slot: &mut f64,
        var_fn25_calc_iq__vdsin_rv_slot: &mut f64,
        var_fn25_calc_iq__vel0_slot: &mut f64,
        var_fn25_calc_iq__vel0_rv_slot: &mut f64,
        var_fn25_calc_iq__vgsin_slot: &mut f64,
        var_fn25_calc_iq__vgsin_dn16_slot: &mut f64,
        var_fn25_calc_iq__vgsin_dn2_slot: &mut f64,
        var_fn25_calc_iq__vgsin_dn7_slot: &mut f64,
        var_fn25_calc_iq__vgsin_rv_slot: &mut f64,
        var_fn25_calc_iq__vsatdibl_slot: &mut f64,
        var_fn25_calc_iq__vsatdibl_dn16_slot: &mut f64,
        var_fn25_calc_iq__vsatdibl_dn17_slot: &mut f64,
        var_fn25_calc_iq__vsatdibl_rv_slot: &mut f64,
        var_fn25_calc_iq__vtdibl_slot: &mut f64,
        var_fn25_calc_iq__vtdibl_dn16_slot: &mut f64,
        var_fn25_calc_iq__vtdibl_dn17_slot: &mut f64,
        var_fn25_calc_iq__vtdibl_dn4_slot: &mut f64,
        var_fn25_calc_iq__vtdibl_rv_slot: &mut f64,
        var_fn25_calc_iq__vtheta_slot: &mut f64,
        var_fn25_calc_iq__vtheta_rv_slot: &mut f64,
        var_fn25_calc_iq__vto_slot: &mut f64,
        var_fn25_calc_iq__vto_rv_slot: &mut f64,
        var_fn25_calc_iq__vtof_slot: &mut f64,
        var_fn25_calc_iq__vtof_dn4_slot: &mut f64,
        var_fn25_calc_iq__vtof_rv_slot: &mut f64,
        var_fn25_calc_iq__vtzeta_slot: &mut f64,
        var_fn25_calc_iq__vtzeta_rv_slot: &mut f64,
        var_fn25_calc_iq__vzeta_slot: &mut f64,
        var_fn25_calc_iq__vzeta_rv_slot: &mut f64,
        var_fn25_calc_iq__w_slot: &mut f64,
        var_fn25_calc_iq__w_rv_slot: &mut f64,
    ) {
        let mut var_fn25_calc_iq__alpha: f64 = *var_fn25_calc_iq__alpha_slot;
        let mut var_fn25_calc_iq__alpha_phit: f64 = *var_fn25_calc_iq__alpha_phit_slot;
        let mut var_fn25_calc_iq__alpha_phit_dn4: f64 = *var_fn25_calc_iq__alpha_phit_dn4_slot;
        let mut var_fn25_calc_iq__alpha_phit_rv: f64 = *var_fn25_calc_iq__alpha_phit_rv_slot;
        let mut var_fn25_calc_iq__alpha_rv: f64 = *var_fn25_calc_iq__alpha_rv_slot;
        let mut var_fn25_calc_iq__beta: f64 = *var_fn25_calc_iq__beta_slot;
        let mut var_fn25_calc_iq__beta_rv: f64 = *var_fn25_calc_iq__beta_rv_slot;
        let mut var_fn25_calc_iq__cb: f64 = *var_fn25_calc_iq__cb_slot;
        let mut var_fn25_calc_iq__cb_dn4: f64 = *var_fn25_calc_iq__cb_dn4_slot;
        let mut var_fn25_calc_iq__cb_rv: f64 = *var_fn25_calc_iq__cb_rv_slot;
        let mut var_fn25_calc_iq__cc: f64 = *var_fn25_calc_iq__cc_slot;
        let mut var_fn25_calc_iq__cc_dn4: f64 = *var_fn25_calc_iq__cc_dn4_slot;
        let mut var_fn25_calc_iq__cc_rv: f64 = *var_fn25_calc_iq__cc_rv_slot;
        let mut var_fn25_calc_iq__cgin: f64 = *var_fn25_calc_iq__cgin_slot;
        let mut var_fn25_calc_iq__cgin_dn4: f64 = *var_fn25_calc_iq__cgin_dn4_slot;
        let mut var_fn25_calc_iq__cgin_rv: f64 = *var_fn25_calc_iq__cgin_rv_slot;
        let mut var_fn25_calc_iq__cs: f64 = *var_fn25_calc_iq__cs_slot;
        let mut var_fn25_calc_iq__cs_rv: f64 = *var_fn25_calc_iq__cs_rv_slot;
        let mut var_fn25_calc_iq__delta: f64 = *var_fn25_calc_iq__delta_slot;
        let mut var_fn25_calc_iq__delta1: f64 = *var_fn25_calc_iq__delta1_slot;
        let mut var_fn25_calc_iq__delta1_rv: f64 = *var_fn25_calc_iq__delta1_rv_slot;
        let mut var_fn25_calc_iq__delta2: f64 = *var_fn25_calc_iq__delta2_slot;
        let mut var_fn25_calc_iq__delta2_rv: f64 = *var_fn25_calc_iq__delta2_rv_slot;
        let mut var_fn25_calc_iq__delta_dn16: f64 = *var_fn25_calc_iq__delta_dn16_slot;
        let mut var_fn25_calc_iq__delta_dn17: f64 = *var_fn25_calc_iq__delta_dn17_slot;
        let mut var_fn25_calc_iq__delta_rv: f64 = *var_fn25_calc_iq__delta_rv_slot;
        let mut var_fn25_calc_iq__dibsat: f64 = *var_fn25_calc_iq__dibsat_slot;
        let mut var_fn25_calc_iq__dibsat_rv: f64 = *var_fn25_calc_iq__dibsat_rv_slot;
        let mut var_fn25_calc_iq__epsilon: f64 = *var_fn25_calc_iq__epsilon_slot;
        let mut var_fn25_calc_iq__epsilon_rv: f64 = *var_fn25_calc_iq__epsilon_rv_slot;
        let mut var_fn25_calc_iq__ffs: f64 = *var_fn25_calc_iq__ffs_slot;
        let mut var_fn25_calc_iq__ffs_dn16: f64 = *var_fn25_calc_iq__ffs_dn16_slot;
        let mut var_fn25_calc_iq__ffs_dn17: f64 = *var_fn25_calc_iq__ffs_dn17_slot;
        let mut var_fn25_calc_iq__ffs_dn2: f64 = *var_fn25_calc_iq__ffs_dn2_slot;
        let mut var_fn25_calc_iq__ffs_dn3: f64 = *var_fn25_calc_iq__ffs_dn3_slot;
        let mut var_fn25_calc_iq__ffs_dn4: f64 = *var_fn25_calc_iq__ffs_dn4_slot;
        let mut var_fn25_calc_iq__ffs_dn7: f64 = *var_fn25_calc_iq__ffs_dn7_slot;
        let mut var_fn25_calc_iq__ffs_rv: f64 = *var_fn25_calc_iq__ffs_rv_slot;
        let mut var_fn25_calc_iq__lambda: f64 = *var_fn25_calc_iq__lambda_slot;
        let mut var_fn25_calc_iq__lambda_rv: f64 = *var_fn25_calc_iq__lambda_rv_slot;
        let mut var_fn25_calc_iq__lin: f64 = *var_fn25_calc_iq__lin_slot;
        let mut var_fn25_calc_iq__lin_rv: f64 = *var_fn25_calc_iq__lin_rv_slot;
        let mut var_fn25_calc_iq__mtheta: f64 = *var_fn25_calc_iq__mtheta_slot;
        let mut var_fn25_calc_iq__mtheta_rv: f64 = *var_fn25_calc_iq__mtheta_rv_slot;
        let mut var_fn25_calc_iq__mu0: f64 = *var_fn25_calc_iq__mu0_slot;
        let mut var_fn25_calc_iq__mu0_rv: f64 = *var_fn25_calc_iq__mu0_rv_slot;
        let mut var_fn25_calc_iq__n: f64 = *var_fn25_calc_iq__n_slot;
        let mut var_fn25_calc_iq__n_dn16: f64 = *var_fn25_calc_iq__n_dn16_slot;
        let mut var_fn25_calc_iq__n_dn17: f64 = *var_fn25_calc_iq__n_dn17_slot;
        let mut var_fn25_calc_iq__n_dn4: f64 = *var_fn25_calc_iq__n_dn4_slot;
        let mut var_fn25_calc_iq__n_rv: f64 = *var_fn25_calc_iq__n_rv_slot;
        let mut var_fn25_calc_iq__nd: f64 = *var_fn25_calc_iq__nd_slot;
        let mut var_fn25_calc_iq__nd_rv: f64 = *var_fn25_calc_iq__nd_rv_slot;
        let mut var_fn25_calc_iq__ngf: f64 = *var_fn25_calc_iq__ngf_slot;
        let mut var_fn25_calc_iq__ngf_rv: f64 = *var_fn25_calc_iq__ngf_rv_slot;
        let mut var_fn25_calc_iq__phitin: f64 = *var_fn25_calc_iq__phitin_slot;
        let mut var_fn25_calc_iq__phitin_dn4: f64 = *var_fn25_calc_iq__phitin_dn4_slot;
        let mut var_fn25_calc_iq__phitin_rv: f64 = *var_fn25_calc_iq__phitin_rv_slot;
        let mut var_fn25_calc_iq__qbout: f64 = *var_fn25_calc_iq__qbout_slot;
        let mut var_fn25_calc_iq__qbout_dn16: f64 = *var_fn25_calc_iq__qbout_dn16_slot;
        let mut var_fn25_calc_iq__qbout_dn17: f64 = *var_fn25_calc_iq__qbout_dn17_slot;
        let mut var_fn25_calc_iq__qbout_dn2: f64 = *var_fn25_calc_iq__qbout_dn2_slot;
        let mut var_fn25_calc_iq__qbout_dn3: f64 = *var_fn25_calc_iq__qbout_dn3_slot;
        let mut var_fn25_calc_iq__qbout_dn4: f64 = *var_fn25_calc_iq__qbout_dn4_slot;
        let mut var_fn25_calc_iq__qbout_dn7: f64 = *var_fn25_calc_iq__qbout_dn7_slot;
        let mut var_fn25_calc_iq__qbout_rv: f64 = *var_fn25_calc_iq__qbout_rv_slot;
        let mut var_fn25_calc_iq__qcbflag: f64 = *var_fn25_calc_iq__qcbflag_slot;
        let mut var_fn25_calc_iq__qcbflag_rv: f64 = *var_fn25_calc_iq__qcbflag_rv_slot;
        let mut var_fn25_calc_iq__qcout: f64 = *var_fn25_calc_iq__qcout_slot;
        let mut var_fn25_calc_iq__qcout_dn16: f64 = *var_fn25_calc_iq__qcout_dn16_slot;
        let mut var_fn25_calc_iq__qcout_dn17: f64 = *var_fn25_calc_iq__qcout_dn17_slot;
        let mut var_fn25_calc_iq__qcout_dn2: f64 = *var_fn25_calc_iq__qcout_dn2_slot;
        let mut var_fn25_calc_iq__qcout_dn3: f64 = *var_fn25_calc_iq__qcout_dn3_slot;
        let mut var_fn25_calc_iq__qcout_dn4: f64 = *var_fn25_calc_iq__qcout_dn4_slot;
        let mut var_fn25_calc_iq__qcout_dn7: f64 = *var_fn25_calc_iq__qcout_dn7_slot;
        let mut var_fn25_calc_iq__qcout_rv: f64 = *var_fn25_calc_iq__qcout_rv_slot;
        let mut var_fn25_calc_iq__qgsflag: f64 = *var_fn25_calc_iq__qgsflag_slot;
        let mut var_fn25_calc_iq__qgsflag_rv: f64 = *var_fn25_calc_iq__qgsflag_rv_slot;
        let mut var_fn25_calc_iq__qsout: f64 = *var_fn25_calc_iq__qsout_slot;
        let mut var_fn25_calc_iq__qsout_dn16: f64 = *var_fn25_calc_iq__qsout_dn16_slot;
        let mut var_fn25_calc_iq__qsout_dn17: f64 = *var_fn25_calc_iq__qsout_dn17_slot;
        let mut var_fn25_calc_iq__qsout_dn2: f64 = *var_fn25_calc_iq__qsout_dn2_slot;
        let mut var_fn25_calc_iq__qsout_dn3: f64 = *var_fn25_calc_iq__qsout_dn3_slot;
        let mut var_fn25_calc_iq__qsout_dn4: f64 = *var_fn25_calc_iq__qsout_dn4_slot;
        let mut var_fn25_calc_iq__qsout_dn7: f64 = *var_fn25_calc_iq__qsout_dn7_slot;
        let mut var_fn25_calc_iq__qsout_rv: f64 = *var_fn25_calc_iq__qsout_rv_slot;
        let mut var_fn25_calc_iq__ss: f64 = *var_fn25_calc_iq__ss_slot;
        let mut var_fn25_calc_iq__ss_rv: f64 = *var_fn25_calc_iq__ss_rv_slot;
        let mut var_fn25_calc_iq__tambin: f64 = *var_fn25_calc_iq__tambin_slot;
        let mut var_fn25_calc_iq__tambin_dn4: f64 = *var_fn25_calc_iq__tambin_dn4_slot;
        let mut var_fn25_calc_iq__tambin_rv: f64 = *var_fn25_calc_iq__tambin_rv_slot;
        let mut var_fn25_calc_iq__tnomin: f64 = *var_fn25_calc_iq__tnomin_slot;
        let mut var_fn25_calc_iq__tnomin_rv: f64 = *var_fn25_calc_iq__tnomin_rv_slot;
        let mut var_fn25_calc_iq__trapfracdl: f64 = *var_fn25_calc_iq__trapfracdl_slot;
        let mut var_fn25_calc_iq__trapfracdl_rv: f64 = *var_fn25_calc_iq__trapfracdl_rv_slot;
        let mut var_fn25_calc_iq__type: f64 = *var_fn25_calc_iq__type_slot;
        let mut var_fn25_calc_iq__type_rv: f64 = *var_fn25_calc_iq__type_rv_slot;
        let mut var_fn25_calc_iq__vbin: f64 = *var_fn25_calc_iq__vbin_slot;
        let mut var_fn25_calc_iq__vbin_dn16: f64 = *var_fn25_calc_iq__vbin_dn16_slot;
        let mut var_fn25_calc_iq__vbin_dn3: f64 = *var_fn25_calc_iq__vbin_dn3_slot;
        let mut var_fn25_calc_iq__vbin_rv: f64 = *var_fn25_calc_iq__vbin_rv_slot;
        let mut var_fn25_calc_iq__vcin: f64 = *var_fn25_calc_iq__vcin_slot;
        let mut var_fn25_calc_iq__vcin_dn16: f64 = *var_fn25_calc_iq__vcin_dn16_slot;
        let mut var_fn25_calc_iq__vcin_dn2: f64 = *var_fn25_calc_iq__vcin_dn2_slot;
        let mut var_fn25_calc_iq__vcin_dn7: f64 = *var_fn25_calc_iq__vcin_dn7_slot;
        let mut var_fn25_calc_iq__vcin_rv: f64 = *var_fn25_calc_iq__vcin_rv_slot;
        let mut var_fn25_calc_iq__vdsat1: f64 = *var_fn25_calc_iq__vdsat1_slot;
        let mut var_fn25_calc_iq__vdsat1_dn16: f64 = *var_fn25_calc_iq__vdsat1_dn16_slot;
        let mut var_fn25_calc_iq__vdsat1_dn17: f64 = *var_fn25_calc_iq__vdsat1_dn17_slot;
        let mut var_fn25_calc_iq__vdsat1_dn2: f64 = *var_fn25_calc_iq__vdsat1_dn2_slot;
        let mut var_fn25_calc_iq__vdsat1_dn3: f64 = *var_fn25_calc_iq__vdsat1_dn3_slot;
        let mut var_fn25_calc_iq__vdsat1_dn4: f64 = *var_fn25_calc_iq__vdsat1_dn4_slot;
        let mut var_fn25_calc_iq__vdsat1_dn7: f64 = *var_fn25_calc_iq__vdsat1_dn7_slot;
        let mut var_fn25_calc_iq__vdsat1_rv: f64 = *var_fn25_calc_iq__vdsat1_rv_slot;
        let mut var_fn25_calc_iq__vdsin: f64 = *var_fn25_calc_iq__vdsin_slot;
        let mut var_fn25_calc_iq__vdsin_dn16: f64 = *var_fn25_calc_iq__vdsin_dn16_slot;
        let mut var_fn25_calc_iq__vdsin_dn17: f64 = *var_fn25_calc_iq__vdsin_dn17_slot;
        let mut var_fn25_calc_iq__vdsin_rv: f64 = *var_fn25_calc_iq__vdsin_rv_slot;
        let mut var_fn25_calc_iq__vel0: f64 = *var_fn25_calc_iq__vel0_slot;
        let mut var_fn25_calc_iq__vel0_rv: f64 = *var_fn25_calc_iq__vel0_rv_slot;
        let mut var_fn25_calc_iq__vgsin: f64 = *var_fn25_calc_iq__vgsin_slot;
        let mut var_fn25_calc_iq__vgsin_dn16: f64 = *var_fn25_calc_iq__vgsin_dn16_slot;
        let mut var_fn25_calc_iq__vgsin_dn2: f64 = *var_fn25_calc_iq__vgsin_dn2_slot;
        let mut var_fn25_calc_iq__vgsin_dn7: f64 = *var_fn25_calc_iq__vgsin_dn7_slot;
        let mut var_fn25_calc_iq__vgsin_rv: f64 = *var_fn25_calc_iq__vgsin_rv_slot;
        let mut var_fn25_calc_iq__vsatdibl: f64 = *var_fn25_calc_iq__vsatdibl_slot;
        let mut var_fn25_calc_iq__vsatdibl_dn16: f64 = *var_fn25_calc_iq__vsatdibl_dn16_slot;
        let mut var_fn25_calc_iq__vsatdibl_dn17: f64 = *var_fn25_calc_iq__vsatdibl_dn17_slot;
        let mut var_fn25_calc_iq__vsatdibl_rv: f64 = *var_fn25_calc_iq__vsatdibl_rv_slot;
        let mut var_fn25_calc_iq__vtdibl: f64 = *var_fn25_calc_iq__vtdibl_slot;
        let mut var_fn25_calc_iq__vtdibl_dn16: f64 = *var_fn25_calc_iq__vtdibl_dn16_slot;
        let mut var_fn25_calc_iq__vtdibl_dn17: f64 = *var_fn25_calc_iq__vtdibl_dn17_slot;
        let mut var_fn25_calc_iq__vtdibl_dn4: f64 = *var_fn25_calc_iq__vtdibl_dn4_slot;
        let mut var_fn25_calc_iq__vtdibl_rv: f64 = *var_fn25_calc_iq__vtdibl_rv_slot;
        let mut var_fn25_calc_iq__vtheta: f64 = *var_fn25_calc_iq__vtheta_slot;
        let mut var_fn25_calc_iq__vtheta_rv: f64 = *var_fn25_calc_iq__vtheta_rv_slot;
        let mut var_fn25_calc_iq__vto: f64 = *var_fn25_calc_iq__vto_slot;
        let mut var_fn25_calc_iq__vto_rv: f64 = *var_fn25_calc_iq__vto_rv_slot;
        let mut var_fn25_calc_iq__vtof: f64 = *var_fn25_calc_iq__vtof_slot;
        let mut var_fn25_calc_iq__vtof_dn4: f64 = *var_fn25_calc_iq__vtof_dn4_slot;
        let mut var_fn25_calc_iq__vtof_rv: f64 = *var_fn25_calc_iq__vtof_rv_slot;
        let mut var_fn25_calc_iq__vtzeta: f64 = *var_fn25_calc_iq__vtzeta_slot;
        let mut var_fn25_calc_iq__vtzeta_rv: f64 = *var_fn25_calc_iq__vtzeta_rv_slot;
        let mut var_fn25_calc_iq__vzeta: f64 = *var_fn25_calc_iq__vzeta_slot;
        let mut var_fn25_calc_iq__vzeta_rv: f64 = *var_fn25_calc_iq__vzeta_rv_slot;
        let mut var_fn25_calc_iq__w: f64 = *var_fn25_calc_iq__w_slot;
        let mut var_fn25_calc_iq__w_rv: f64 = *var_fn25_calc_iq__w_rv_slot;

        let (assign1800_e3905, assign1800_e3905_d_n2, assign1800_e3905_d_n3, assign1800_e3905_d_n4, assign1800_e3905_d_n7, assign1800_e3905_d_n16, assign1800_e3905_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__qcout, var_fn25_calc_iq__qcout_dn2, var_fn25_calc_iq__qcout_dn3, var_fn25_calc_iq__qcout_dn4, var_fn25_calc_iq__qcout_dn7, var_fn25_calc_iq__qcout_dn16, var_fn25_calc_iq__qcout_dn17,)
    }
};
        var_fn25_calc_iq__qcout = assign1800_e3905;
        var_fn25_calc_iq__qcout_dn2 = assign1800_e3905_d_n2;
        var_fn25_calc_iq__qcout_dn3 = assign1800_e3905_d_n3;
        var_fn25_calc_iq__qcout_dn4 = assign1800_e3905_d_n4;
        var_fn25_calc_iq__qcout_dn7 = assign1800_e3905_d_n7;
        var_fn25_calc_iq__qcout_dn16 = assign1800_e3905_d_n16;
        var_fn25_calc_iq__qcout_dn17 = assign1800_e3905_d_n17;
        var_fn25_calc_iq__qcout_rv = 0.0;

        let (assign1810_e3909, assign1810_e3909_d_n2, assign1810_e3909_d_n3, assign1810_e3909_d_n4, assign1810_e3909_d_n7, assign1810_e3909_d_n16, assign1810_e3909_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__qbout, var_fn25_calc_iq__qbout_dn2, var_fn25_calc_iq__qbout_dn3, var_fn25_calc_iq__qbout_dn4, var_fn25_calc_iq__qbout_dn7, var_fn25_calc_iq__qbout_dn16, var_fn25_calc_iq__qbout_dn17,)
    }
};
        var_fn25_calc_iq__qbout = assign1810_e3909;
        var_fn25_calc_iq__qbout_dn2 = assign1810_e3909_d_n2;
        var_fn25_calc_iq__qbout_dn3 = assign1810_e3909_d_n3;
        var_fn25_calc_iq__qbout_dn4 = assign1810_e3909_d_n4;
        var_fn25_calc_iq__qbout_dn7 = assign1810_e3909_d_n7;
        var_fn25_calc_iq__qbout_dn16 = assign1810_e3909_d_n16;
        var_fn25_calc_iq__qbout_dn17 = assign1810_e3909_d_n17;
        var_fn25_calc_iq__qbout_rv = 0.0;

        let (assign1820_e3913, assign1820_e3913_d_n2, assign1820_e3913_d_n3, assign1820_e3913_d_n4, assign1820_e3913_d_n7, assign1820_e3913_d_n16, assign1820_e3913_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__qsout, var_fn25_calc_iq__qsout_dn2, var_fn25_calc_iq__qsout_dn3, var_fn25_calc_iq__qsout_dn4, var_fn25_calc_iq__qsout_dn7, var_fn25_calc_iq__qsout_dn16, var_fn25_calc_iq__qsout_dn17,)
    }
};
        var_fn25_calc_iq__qsout = assign1820_e3913;
        var_fn25_calc_iq__qsout_dn2 = assign1820_e3913_d_n2;
        var_fn25_calc_iq__qsout_dn3 = assign1820_e3913_d_n3;
        var_fn25_calc_iq__qsout_dn4 = assign1820_e3913_d_n4;
        var_fn25_calc_iq__qsout_dn7 = assign1820_e3913_d_n7;
        var_fn25_calc_iq__qsout_dn16 = assign1820_e3913_d_n16;
        var_fn25_calc_iq__qsout_dn17 = assign1820_e3913_d_n17;
        var_fn25_calc_iq__qsout_rv = 0.0;

        let (assign1830_e3917, assign1830_e3917_d_n4, assign1830_e3917_d_n16, assign1830_e3917_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__vtdibl, var_fn25_calc_iq__vtdibl_dn4, var_fn25_calc_iq__vtdibl_dn16, var_fn25_calc_iq__vtdibl_dn17,)
    }
};
        var_fn25_calc_iq__vtdibl = assign1830_e3917;
        var_fn25_calc_iq__vtdibl_dn4 = assign1830_e3917_d_n4;
        var_fn25_calc_iq__vtdibl_dn16 = assign1830_e3917_d_n16;
        var_fn25_calc_iq__vtdibl_dn17 = assign1830_e3917_d_n17;
        var_fn25_calc_iq__vtdibl_rv = 0.0;

        let (assign1840_e3921, assign1840_e3921_d_n2, assign1840_e3921_d_n3, assign1840_e3921_d_n4, assign1840_e3921_d_n7, assign1840_e3921_d_n16, assign1840_e3921_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__vdsat1, var_fn25_calc_iq__vdsat1_dn2, var_fn25_calc_iq__vdsat1_dn3, var_fn25_calc_iq__vdsat1_dn4, var_fn25_calc_iq__vdsat1_dn7, var_fn25_calc_iq__vdsat1_dn16, var_fn25_calc_iq__vdsat1_dn17,)
    }
};
        var_fn25_calc_iq__vdsat1 = assign1840_e3921;
        var_fn25_calc_iq__vdsat1_dn2 = assign1840_e3921_d_n2;
        var_fn25_calc_iq__vdsat1_dn3 = assign1840_e3921_d_n3;
        var_fn25_calc_iq__vdsat1_dn4 = assign1840_e3921_d_n4;
        var_fn25_calc_iq__vdsat1_dn7 = assign1840_e3921_d_n7;
        var_fn25_calc_iq__vdsat1_dn16 = assign1840_e3921_d_n16;
        var_fn25_calc_iq__vdsat1_dn17 = assign1840_e3921_d_n17;
        var_fn25_calc_iq__vdsat1_rv = 0.0;

        let (assign1850_e3925, assign1850_e3925_d_n2, assign1850_e3925_d_n7, assign1850_e3925_d_n16,) = {
    if (var_guard24 != 0.0) {
        (var_vgsfp4, var_vgsfp4_dn2, var_vgsfp4_dn7, var_vgsfp4_dn16,)
    } else {
        (var_fn25_calc_iq__vgsin, var_fn25_calc_iq__vgsin_dn2, var_fn25_calc_iq__vgsin_dn7, var_fn25_calc_iq__vgsin_dn16,)
    }
};
        var_fn25_calc_iq__vgsin = assign1850_e3925;
        var_fn25_calc_iq__vgsin_dn2 = assign1850_e3925_d_n2;
        var_fn25_calc_iq__vgsin_dn7 = assign1850_e3925_d_n7;
        var_fn25_calc_iq__vgsin_dn16 = assign1850_e3925_d_n16;
        var_fn25_calc_iq__vgsin_rv = 0.0;

        let (assign1860_e3929, assign1860_e3929_d_n16, assign1860_e3929_d_n17,) = {
    if (var_guard24 != 0.0) {
        (var_vdsfp4, var_vdsfp4_dn16, var_vdsfp4_dn17,)
    } else {
        (var_fn25_calc_iq__vdsin, var_fn25_calc_iq__vdsin_dn16, var_fn25_calc_iq__vdsin_dn17,)
    }
};
        var_fn25_calc_iq__vdsin = assign1860_e3929;
        var_fn25_calc_iq__vdsin_dn16 = assign1860_e3929_d_n16;
        var_fn25_calc_iq__vdsin_dn17 = assign1860_e3929_d_n17;
        var_fn25_calc_iq__vdsin_rv = 0.0;

        let (assign1870_e3933,) = {
    if (var_guard24 != 0.0) {
        (p.p239,)
    } else {
        (var_fn25_calc_iq__qcbflag,)
    }
};
        var_fn25_calc_iq__qcbflag = assign1870_e3933;
        var_fn25_calc_iq__qcbflag_rv = 0.0;

        let (assign1880_e3937, assign1880_e3937_d_n2, assign1880_e3937_d_n7, assign1880_e3937_d_n16,) = {
    if (var_guard24 != 0.0) {
        (var_vcfp4, var_vcfp4_dn2, var_vcfp4_dn7, var_vcfp4_dn16,)
    } else {
        (var_fn25_calc_iq__vcin, var_fn25_calc_iq__vcin_dn2, var_fn25_calc_iq__vcin_dn7, var_fn25_calc_iq__vcin_dn16,)
    }
};
        var_fn25_calc_iq__vcin = assign1880_e3937;
        var_fn25_calc_iq__vcin_dn2 = assign1880_e3937_d_n2;
        var_fn25_calc_iq__vcin_dn7 = assign1880_e3937_d_n7;
        var_fn25_calc_iq__vcin_dn16 = assign1880_e3937_d_n16;
        var_fn25_calc_iq__vcin_rv = 0.0;

        let (assign1890_e3941, assign1890_e3941_d_n3, assign1890_e3941_d_n16,) = {
    if (var_guard24 != 0.0) {
        (var_vbfp4, var_vbfp4_dn3, var_vbfp4_dn16,)
    } else {
        (var_fn25_calc_iq__vbin, var_fn25_calc_iq__vbin_dn3, var_fn25_calc_iq__vbin_dn16,)
    }
};
        var_fn25_calc_iq__vbin = assign1890_e3941;
        var_fn25_calc_iq__vbin_dn3 = assign1890_e3941_d_n3;
        var_fn25_calc_iq__vbin_dn16 = assign1890_e3941_d_n16;
        var_fn25_calc_iq__vbin_rv = 0.0;

        let (assign1900_e3945,) = {
    if (var_guard24 != 0.0) {
        (p.p237,)
    } else {
        (var_fn25_calc_iq__qgsflag,)
    }
};
        var_fn25_calc_iq__qgsflag = assign1900_e3945;
        var_fn25_calc_iq__qgsflag_rv = 0.0;

        let (assign1910_e3949, assign1910_e3949_d_n4,) = {
    if (var_guard24 != 0.0) {
        (var_tdut, var_tdut_dn4,)
    } else {
        (var_fn25_calc_iq__tambin, var_fn25_calc_iq__tambin_dn4,)
    }
};
        var_fn25_calc_iq__tambin = assign1910_e3949;
        var_fn25_calc_iq__tambin_dn4 = assign1910_e3949_d_n4;
        var_fn25_calc_iq__tambin_rv = 0.0;

        let (assign1920_e3953,) = {
    if (var_guard24 != 0.0) {
        (var_tnomk,)
    } else {
        (var_fn25_calc_iq__tnomin,)
    }
};
        var_fn25_calc_iq__tnomin = assign1920_e3953;
        var_fn25_calc_iq__tnomin_rv = 0.0;

        let (assign1930_e3957, assign1930_e3957_d_n4,) = {
    if (var_guard24 != 0.0) {
        (var_phit, var_phit_dn4,)
    } else {
        (var_fn25_calc_iq__phitin, var_fn25_calc_iq__phitin_dn4,)
    }
};
        var_fn25_calc_iq__phitin = assign1930_e3957;
        var_fn25_calc_iq__phitin_dn4 = assign1930_e3957_d_n4;
        var_fn25_calc_iq__phitin_rv = 0.0;

        let (assign1940_e3961,) = {
    if (var_guard24 != 0.0) {
        (p.p0,)
    } else {
        (var_fn25_calc_iq__w,)
    }
};
        var_fn25_calc_iq__w = assign1940_e3961;
        var_fn25_calc_iq__w_rv = 0.0;

        let (assign1950_e3965,) = {
    if (var_guard24 != 0.0) {
        (p.p233,)
    } else {
        (var_fn25_calc_iq__lin,)
    }
};
        var_fn25_calc_iq__lin = assign1950_e3965;
        var_fn25_calc_iq__lin_rv = 0.0;

        let (assign1960_e3969, assign1960_e3969_d_n4,) = {
    if (var_guard24 != 0.0) {
        (var_cgfp4t, var_cgfp4t_dn4,)
    } else {
        (var_fn25_calc_iq__cgin, var_fn25_calc_iq__cgin_dn4,)
    }
};
        var_fn25_calc_iq__cgin = assign1960_e3969;
        var_fn25_calc_iq__cgin_dn4 = assign1960_e3969_d_n4;
        var_fn25_calc_iq__cgin_rv = 0.0;

        let (assign1970_e3973,) = {
    if (var_guard24 != 0.0) {
        (p.p238,)
    } else {
        (var_fn25_calc_iq__cs,)
    }
};
        var_fn25_calc_iq__cs = assign1970_e3973;
        var_fn25_calc_iq__cs_rv = 0.0;

        let (assign1980_e3977, assign1980_e3977_d_n4,) = {
    if (var_guard24 != 0.0) {
        (var_ccfp4t, var_ccfp4t_dn4,)
    } else {
        (var_fn25_calc_iq__cc, var_fn25_calc_iq__cc_dn4,)
    }
};
        var_fn25_calc_iq__cc = assign1980_e3977;
        var_fn25_calc_iq__cc_dn4 = assign1980_e3977_d_n4;
        var_fn25_calc_iq__cc_rv = 0.0;

        let (assign1990_e3981, assign1990_e3981_d_n4,) = {
    if (var_guard24 != 0.0) {
        (var_cbfp4t, var_cbfp4t_dn4,)
    } else {
        (var_fn25_calc_iq__cb, var_fn25_calc_iq__cb_dn4,)
    }
};
        var_fn25_calc_iq__cb = assign1990_e3981;
        var_fn25_calc_iq__cb_dn4 = assign1990_e3981_d_n4;
        var_fn25_calc_iq__cb_rv = 0.0;

        let (assign2000_e3985,) = {
    if (var_guard24 != 0.0) {
        (p.p234,)
    } else {
        (var_fn25_calc_iq__vto,)
    }
};
        var_fn25_calc_iq__vto = assign2000_e3985;
        var_fn25_calc_iq__vto_rv = 0.0;

        let (assign2010_e3989,) = {
    if (var_guard24 != 0.0) {
        (p.p248,)
    } else {
        (var_fn25_calc_iq__ss,)
    }
};
        var_fn25_calc_iq__ss = assign2010_e3989;
        var_fn25_calc_iq__ss_rv = 0.0;

        let (assign2020_e3993,) = {
    if (var_guard24 != 0.0) {
        (p.p247,)
    } else {
        (var_fn25_calc_iq__delta1,)
    }
};
        var_fn25_calc_iq__delta1 = assign2020_e3993;
        var_fn25_calc_iq__delta1_rv = 0.0;

        let (assign2030_e3997,) = {
    if (var_guard24 != 0.0) {
        (0.0,)
    } else {
        (var_fn25_calc_iq__delta2,)
    }
};
        var_fn25_calc_iq__delta2 = assign2030_e3997;
        var_fn25_calc_iq__delta2_rv = 0.0;

        let (assign2040_e4001,) = {
    if (var_guard24 != 0.0) {
        (p.p249,)
    } else {
        (var_fn25_calc_iq__nd,)
    }
};
        var_fn25_calc_iq__nd = assign2040_e4001;
        var_fn25_calc_iq__nd_rv = 0.0;

        let (assign2050_e4005,) = {
    if (var_guard24 != 0.0) {
        (p.p253,)
    } else {
        (var_fn25_calc_iq__alpha,)
    }
};
        var_fn25_calc_iq__alpha = assign2050_e4005;
        var_fn25_calc_iq__alpha_rv = 0.0;

        let (assign2060_e4009,) = {
    if (var_guard24 != 0.0) {
        (p.p244,)
    } else {
        (var_fn25_calc_iq__vel0,)
    }
};
        var_fn25_calc_iq__vel0 = assign2060_e4009;
        var_fn25_calc_iq__vel0_rv = 0.0;

        let (assign2070_e4013,) = {
    if (var_guard24 != 0.0) {
        (p.p245,)
    } else {
        (var_fn25_calc_iq__mu0,)
    }
};
        var_fn25_calc_iq__mu0 = assign2070_e4013;
        var_fn25_calc_iq__mu0_rv = 0.0;

        let (assign2080_e4017,) = {
    if (var_guard24 != 0.0) {
        (p.p246,)
    } else {
        (var_fn25_calc_iq__beta,)
    }
};
        var_fn25_calc_iq__beta = assign2080_e4017;
        var_fn25_calc_iq__beta_rv = 0.0;

        let (assign2090_e4021,) = {
    if (var_guard24 != 0.0) {
        (p.p252,)
    } else {
        (var_fn25_calc_iq__mtheta,)
    }
};
        var_fn25_calc_iq__mtheta = assign2090_e4021;
        var_fn25_calc_iq__mtheta_rv = 0.0;

        let (assign2100_e4025,) = {
    if (var_guard24 != 0.0) {
        (p.p251,)
    } else {
        (var_fn25_calc_iq__vtheta,)
    }
};
        var_fn25_calc_iq__vtheta = assign2100_e4025;
        var_fn25_calc_iq__vtheta_rv = 0.0;

        let (assign2110_e4029,) = {
    if (var_guard24 != 0.0) {
        (p.p250,)
    } else {
        (var_fn25_calc_iq__vtzeta,)
    }
};
        var_fn25_calc_iq__vtzeta = assign2110_e4029;
        var_fn25_calc_iq__vtzeta_rv = 0.0;

        let (assign2120_e4033,) = {
    if (var_guard24 != 0.0) {
        (p.p39,)
    } else {
        (var_fn25_calc_iq__dibsat,)
    }
};
        var_fn25_calc_iq__dibsat = assign2120_e4033;
        var_fn25_calc_iq__dibsat_rv = 0.0;

        let (assign2130_e4037,) = {
    if (var_guard24 != 0.0) {
        (p.p47,)
    } else {
        (var_fn25_calc_iq__epsilon,)
    }
};
        var_fn25_calc_iq__epsilon = assign2130_e4037;
        var_fn25_calc_iq__epsilon_rv = 0.0;

        let (assign2140_e4041,) = {
    if (var_guard24 != 0.0) {
        (p.p45,)
    } else {
        (var_fn25_calc_iq__vzeta,)
    }
};
        var_fn25_calc_iq__vzeta = assign2140_e4041;
        var_fn25_calc_iq__vzeta_rv = 0.0;

        let (assign2150_e4045,) = {
    if (var_guard24 != 0.0) {
        (p.p42,)
    } else {
        (var_fn25_calc_iq__lambda,)
    }
};
        var_fn25_calc_iq__lambda = assign2150_e4045;
        var_fn25_calc_iq__lambda_rv = 0.0;

        let (assign2160_e4049,) = {
    if (var_guard24 != 0.0) {
        (p.p2,)
    } else {
        (var_fn25_calc_iq__ngf,)
    }
};
        var_fn25_calc_iq__ngf = assign2160_e4049;
        var_fn25_calc_iq__ngf_rv = 0.0;

        let (assign2170_e4053,) = {
    if (var_guard24 != 0.0) {
        (p.p6,)
    } else {
        (var_fn25_calc_iq__type,)
    }
};
        var_fn25_calc_iq__type = assign2170_e4053;
        var_fn25_calc_iq__type_rv = 0.0;

        let (assign2180_e4057,) = {
    if (var_guard24 != 0.0) {
        (1.0,)
    } else {
        (var_fn25_calc_iq__trapfracdl,)
    }
};
        var_fn25_calc_iq__trapfracdl = assign2180_e4057;
        var_fn25_calc_iq__trapfracdl_rv = 0.0;

        let (assign2190_e4061, assign2190_e4061_d_n4,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__alpha_phit, var_fn25_calc_iq__alpha_phit_dn4,)
    }
};
        var_fn25_calc_iq__alpha_phit = assign2190_e4061;
        var_fn25_calc_iq__alpha_phit_dn4 = assign2190_e4061_d_n4;
        var_fn25_calc_iq__alpha_phit_rv = 0.0;

        let (assign2200_e4065, assign2200_e4065_d_n16, assign2200_e4065_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__delta, var_fn25_calc_iq__delta_dn16, var_fn25_calc_iq__delta_dn17,)
    }
};
        var_fn25_calc_iq__delta = assign2200_e4065;
        var_fn25_calc_iq__delta_dn16 = assign2200_e4065_d_n16;
        var_fn25_calc_iq__delta_dn17 = assign2200_e4065_d_n17;
        var_fn25_calc_iq__delta_rv = 0.0;

        let (assign2210_e4069, assign2210_e4069_d_n4, assign2210_e4069_d_n16, assign2210_e4069_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__n, var_fn25_calc_iq__n_dn4, var_fn25_calc_iq__n_dn16, var_fn25_calc_iq__n_dn17,)
    }
};
        var_fn25_calc_iq__n = assign2210_e4069;
        var_fn25_calc_iq__n_dn4 = assign2210_e4069_d_n4;
        var_fn25_calc_iq__n_dn16 = assign2210_e4069_d_n16;
        var_fn25_calc_iq__n_dn17 = assign2210_e4069_d_n17;
        var_fn25_calc_iq__n_rv = 0.0;

        let (assign2220_e4073, assign2220_e4073_d_n4,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__vtof, var_fn25_calc_iq__vtof_dn4,)
    }
};
        var_fn25_calc_iq__vtof = assign2220_e4073;
        var_fn25_calc_iq__vtof_dn4 = assign2220_e4073_d_n4;
        var_fn25_calc_iq__vtof_rv = 0.0;

        let (assign2230_e4077, assign2230_e4077_d_n16, assign2230_e4077_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__vsatdibl, var_fn25_calc_iq__vsatdibl_dn16, var_fn25_calc_iq__vsatdibl_dn17,)
    }
};
        var_fn25_calc_iq__vsatdibl = assign2230_e4077;
        var_fn25_calc_iq__vsatdibl_dn16 = assign2230_e4077_d_n16;
        var_fn25_calc_iq__vsatdibl_dn17 = assign2230_e4077_d_n17;
        var_fn25_calc_iq__vsatdibl_rv = 0.0;

        let (assign2240_e4081, assign2240_e4081_d_n2, assign2240_e4081_d_n3, assign2240_e4081_d_n4, assign2240_e4081_d_n7, assign2240_e4081_d_n16, assign2240_e4081_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__ffs, var_fn25_calc_iq__ffs_dn2, var_fn25_calc_iq__ffs_dn3, var_fn25_calc_iq__ffs_dn4, var_fn25_calc_iq__ffs_dn7, var_fn25_calc_iq__ffs_dn16, var_fn25_calc_iq__ffs_dn17,)
    }
};
        var_fn25_calc_iq__ffs = assign2240_e4081;
        var_fn25_calc_iq__ffs_dn2 = assign2240_e4081_d_n2;
        var_fn25_calc_iq__ffs_dn3 = assign2240_e4081_d_n3;
        var_fn25_calc_iq__ffs_dn4 = assign2240_e4081_d_n4;
        var_fn25_calc_iq__ffs_dn7 = assign2240_e4081_d_n7;
        var_fn25_calc_iq__ffs_dn16 = assign2240_e4081_d_n16;
        var_fn25_calc_iq__ffs_dn17 = assign2240_e4081_d_n17;
        var_fn25_calc_iq__ffs_rv = 0.0;

        *var_fn25_calc_iq__alpha_slot = var_fn25_calc_iq__alpha;
        *var_fn25_calc_iq__alpha_phit_slot = var_fn25_calc_iq__alpha_phit;
        *var_fn25_calc_iq__alpha_phit_dn4_slot = var_fn25_calc_iq__alpha_phit_dn4;
        *var_fn25_calc_iq__alpha_phit_rv_slot = var_fn25_calc_iq__alpha_phit_rv;
        *var_fn25_calc_iq__alpha_rv_slot = var_fn25_calc_iq__alpha_rv;
        *var_fn25_calc_iq__beta_slot = var_fn25_calc_iq__beta;
        *var_fn25_calc_iq__beta_rv_slot = var_fn25_calc_iq__beta_rv;
        *var_fn25_calc_iq__cb_slot = var_fn25_calc_iq__cb;
        *var_fn25_calc_iq__cb_dn4_slot = var_fn25_calc_iq__cb_dn4;
        *var_fn25_calc_iq__cb_rv_slot = var_fn25_calc_iq__cb_rv;
        *var_fn25_calc_iq__cc_slot = var_fn25_calc_iq__cc;
        *var_fn25_calc_iq__cc_dn4_slot = var_fn25_calc_iq__cc_dn4;
        *var_fn25_calc_iq__cc_rv_slot = var_fn25_calc_iq__cc_rv;
        *var_fn25_calc_iq__cgin_slot = var_fn25_calc_iq__cgin;
        *var_fn25_calc_iq__cgin_dn4_slot = var_fn25_calc_iq__cgin_dn4;
        *var_fn25_calc_iq__cgin_rv_slot = var_fn25_calc_iq__cgin_rv;
        *var_fn25_calc_iq__cs_slot = var_fn25_calc_iq__cs;
        *var_fn25_calc_iq__cs_rv_slot = var_fn25_calc_iq__cs_rv;
        *var_fn25_calc_iq__delta_slot = var_fn25_calc_iq__delta;
        *var_fn25_calc_iq__delta1_slot = var_fn25_calc_iq__delta1;
        *var_fn25_calc_iq__delta1_rv_slot = var_fn25_calc_iq__delta1_rv;
        *var_fn25_calc_iq__delta2_slot = var_fn25_calc_iq__delta2;
        *var_fn25_calc_iq__delta2_rv_slot = var_fn25_calc_iq__delta2_rv;
        *var_fn25_calc_iq__delta_dn16_slot = var_fn25_calc_iq__delta_dn16;
        *var_fn25_calc_iq__delta_dn17_slot = var_fn25_calc_iq__delta_dn17;
        *var_fn25_calc_iq__delta_rv_slot = var_fn25_calc_iq__delta_rv;
        *var_fn25_calc_iq__dibsat_slot = var_fn25_calc_iq__dibsat;
        *var_fn25_calc_iq__dibsat_rv_slot = var_fn25_calc_iq__dibsat_rv;
        *var_fn25_calc_iq__epsilon_slot = var_fn25_calc_iq__epsilon;
        *var_fn25_calc_iq__epsilon_rv_slot = var_fn25_calc_iq__epsilon_rv;
        *var_fn25_calc_iq__ffs_slot = var_fn25_calc_iq__ffs;
        *var_fn25_calc_iq__ffs_dn16_slot = var_fn25_calc_iq__ffs_dn16;
        *var_fn25_calc_iq__ffs_dn17_slot = var_fn25_calc_iq__ffs_dn17;
        *var_fn25_calc_iq__ffs_dn2_slot = var_fn25_calc_iq__ffs_dn2;
        *var_fn25_calc_iq__ffs_dn3_slot = var_fn25_calc_iq__ffs_dn3;
        *var_fn25_calc_iq__ffs_dn4_slot = var_fn25_calc_iq__ffs_dn4;
        *var_fn25_calc_iq__ffs_dn7_slot = var_fn25_calc_iq__ffs_dn7;
        *var_fn25_calc_iq__ffs_rv_slot = var_fn25_calc_iq__ffs_rv;
        *var_fn25_calc_iq__lambda_slot = var_fn25_calc_iq__lambda;
        *var_fn25_calc_iq__lambda_rv_slot = var_fn25_calc_iq__lambda_rv;
        *var_fn25_calc_iq__lin_slot = var_fn25_calc_iq__lin;
        *var_fn25_calc_iq__lin_rv_slot = var_fn25_calc_iq__lin_rv;
        *var_fn25_calc_iq__mtheta_slot = var_fn25_calc_iq__mtheta;
        *var_fn25_calc_iq__mtheta_rv_slot = var_fn25_calc_iq__mtheta_rv;
        *var_fn25_calc_iq__mu0_slot = var_fn25_calc_iq__mu0;
        *var_fn25_calc_iq__mu0_rv_slot = var_fn25_calc_iq__mu0_rv;
        *var_fn25_calc_iq__n_slot = var_fn25_calc_iq__n;
        *var_fn25_calc_iq__n_dn16_slot = var_fn25_calc_iq__n_dn16;
        *var_fn25_calc_iq__n_dn17_slot = var_fn25_calc_iq__n_dn17;
        *var_fn25_calc_iq__n_dn4_slot = var_fn25_calc_iq__n_dn4;
        *var_fn25_calc_iq__n_rv_slot = var_fn25_calc_iq__n_rv;
        *var_fn25_calc_iq__nd_slot = var_fn25_calc_iq__nd;
        *var_fn25_calc_iq__nd_rv_slot = var_fn25_calc_iq__nd_rv;
        *var_fn25_calc_iq__ngf_slot = var_fn25_calc_iq__ngf;
        *var_fn25_calc_iq__ngf_rv_slot = var_fn25_calc_iq__ngf_rv;
        *var_fn25_calc_iq__phitin_slot = var_fn25_calc_iq__phitin;
        *var_fn25_calc_iq__phitin_dn4_slot = var_fn25_calc_iq__phitin_dn4;
        *var_fn25_calc_iq__phitin_rv_slot = var_fn25_calc_iq__phitin_rv;
        *var_fn25_calc_iq__qbout_slot = var_fn25_calc_iq__qbout;
        *var_fn25_calc_iq__qbout_dn16_slot = var_fn25_calc_iq__qbout_dn16;
        *var_fn25_calc_iq__qbout_dn17_slot = var_fn25_calc_iq__qbout_dn17;
        *var_fn25_calc_iq__qbout_dn2_slot = var_fn25_calc_iq__qbout_dn2;
        *var_fn25_calc_iq__qbout_dn3_slot = var_fn25_calc_iq__qbout_dn3;
        *var_fn25_calc_iq__qbout_dn4_slot = var_fn25_calc_iq__qbout_dn4;
        *var_fn25_calc_iq__qbout_dn7_slot = var_fn25_calc_iq__qbout_dn7;
        *var_fn25_calc_iq__qbout_rv_slot = var_fn25_calc_iq__qbout_rv;
        *var_fn25_calc_iq__qcbflag_slot = var_fn25_calc_iq__qcbflag;
        *var_fn25_calc_iq__qcbflag_rv_slot = var_fn25_calc_iq__qcbflag_rv;
        *var_fn25_calc_iq__qcout_slot = var_fn25_calc_iq__qcout;
        *var_fn25_calc_iq__qcout_dn16_slot = var_fn25_calc_iq__qcout_dn16;
        *var_fn25_calc_iq__qcout_dn17_slot = var_fn25_calc_iq__qcout_dn17;
        *var_fn25_calc_iq__qcout_dn2_slot = var_fn25_calc_iq__qcout_dn2;
        *var_fn25_calc_iq__qcout_dn3_slot = var_fn25_calc_iq__qcout_dn3;
        *var_fn25_calc_iq__qcout_dn4_slot = var_fn25_calc_iq__qcout_dn4;
        *var_fn25_calc_iq__qcout_dn7_slot = var_fn25_calc_iq__qcout_dn7;
        *var_fn25_calc_iq__qcout_rv_slot = var_fn25_calc_iq__qcout_rv;
        *var_fn25_calc_iq__qgsflag_slot = var_fn25_calc_iq__qgsflag;
        *var_fn25_calc_iq__qgsflag_rv_slot = var_fn25_calc_iq__qgsflag_rv;
        *var_fn25_calc_iq__qsout_slot = var_fn25_calc_iq__qsout;
        *var_fn25_calc_iq__qsout_dn16_slot = var_fn25_calc_iq__qsout_dn16;
        *var_fn25_calc_iq__qsout_dn17_slot = var_fn25_calc_iq__qsout_dn17;
        *var_fn25_calc_iq__qsout_dn2_slot = var_fn25_calc_iq__qsout_dn2;
        *var_fn25_calc_iq__qsout_dn3_slot = var_fn25_calc_iq__qsout_dn3;
        *var_fn25_calc_iq__qsout_dn4_slot = var_fn25_calc_iq__qsout_dn4;
        *var_fn25_calc_iq__qsout_dn7_slot = var_fn25_calc_iq__qsout_dn7;
        *var_fn25_calc_iq__qsout_rv_slot = var_fn25_calc_iq__qsout_rv;
        *var_fn25_calc_iq__ss_slot = var_fn25_calc_iq__ss;
        *var_fn25_calc_iq__ss_rv_slot = var_fn25_calc_iq__ss_rv;
        *var_fn25_calc_iq__tambin_slot = var_fn25_calc_iq__tambin;
        *var_fn25_calc_iq__tambin_dn4_slot = var_fn25_calc_iq__tambin_dn4;
        *var_fn25_calc_iq__tambin_rv_slot = var_fn25_calc_iq__tambin_rv;
        *var_fn25_calc_iq__tnomin_slot = var_fn25_calc_iq__tnomin;
        *var_fn25_calc_iq__tnomin_rv_slot = var_fn25_calc_iq__tnomin_rv;
        *var_fn25_calc_iq__trapfracdl_slot = var_fn25_calc_iq__trapfracdl;
        *var_fn25_calc_iq__trapfracdl_rv_slot = var_fn25_calc_iq__trapfracdl_rv;
        *var_fn25_calc_iq__type_slot = var_fn25_calc_iq__type;
        *var_fn25_calc_iq__type_rv_slot = var_fn25_calc_iq__type_rv;
        *var_fn25_calc_iq__vbin_slot = var_fn25_calc_iq__vbin;
        *var_fn25_calc_iq__vbin_dn16_slot = var_fn25_calc_iq__vbin_dn16;
        *var_fn25_calc_iq__vbin_dn3_slot = var_fn25_calc_iq__vbin_dn3;
        *var_fn25_calc_iq__vbin_rv_slot = var_fn25_calc_iq__vbin_rv;
        *var_fn25_calc_iq__vcin_slot = var_fn25_calc_iq__vcin;
        *var_fn25_calc_iq__vcin_dn16_slot = var_fn25_calc_iq__vcin_dn16;
        *var_fn25_calc_iq__vcin_dn2_slot = var_fn25_calc_iq__vcin_dn2;
        *var_fn25_calc_iq__vcin_dn7_slot = var_fn25_calc_iq__vcin_dn7;
        *var_fn25_calc_iq__vcin_rv_slot = var_fn25_calc_iq__vcin_rv;
        *var_fn25_calc_iq__vdsat1_slot = var_fn25_calc_iq__vdsat1;
        *var_fn25_calc_iq__vdsat1_dn16_slot = var_fn25_calc_iq__vdsat1_dn16;
        *var_fn25_calc_iq__vdsat1_dn17_slot = var_fn25_calc_iq__vdsat1_dn17;
        *var_fn25_calc_iq__vdsat1_dn2_slot = var_fn25_calc_iq__vdsat1_dn2;
        *var_fn25_calc_iq__vdsat1_dn3_slot = var_fn25_calc_iq__vdsat1_dn3;
        *var_fn25_calc_iq__vdsat1_dn4_slot = var_fn25_calc_iq__vdsat1_dn4;
        *var_fn25_calc_iq__vdsat1_dn7_slot = var_fn25_calc_iq__vdsat1_dn7;
        *var_fn25_calc_iq__vdsat1_rv_slot = var_fn25_calc_iq__vdsat1_rv;
        *var_fn25_calc_iq__vdsin_slot = var_fn25_calc_iq__vdsin;
        *var_fn25_calc_iq__vdsin_dn16_slot = var_fn25_calc_iq__vdsin_dn16;
        *var_fn25_calc_iq__vdsin_dn17_slot = var_fn25_calc_iq__vdsin_dn17;
        *var_fn25_calc_iq__vdsin_rv_slot = var_fn25_calc_iq__vdsin_rv;
        *var_fn25_calc_iq__vel0_slot = var_fn25_calc_iq__vel0;
        *var_fn25_calc_iq__vel0_rv_slot = var_fn25_calc_iq__vel0_rv;
        *var_fn25_calc_iq__vgsin_slot = var_fn25_calc_iq__vgsin;
        *var_fn25_calc_iq__vgsin_dn16_slot = var_fn25_calc_iq__vgsin_dn16;
        *var_fn25_calc_iq__vgsin_dn2_slot = var_fn25_calc_iq__vgsin_dn2;
        *var_fn25_calc_iq__vgsin_dn7_slot = var_fn25_calc_iq__vgsin_dn7;
        *var_fn25_calc_iq__vgsin_rv_slot = var_fn25_calc_iq__vgsin_rv;
        *var_fn25_calc_iq__vsatdibl_slot = var_fn25_calc_iq__vsatdibl;
        *var_fn25_calc_iq__vsatdibl_dn16_slot = var_fn25_calc_iq__vsatdibl_dn16;
        *var_fn25_calc_iq__vsatdibl_dn17_slot = var_fn25_calc_iq__vsatdibl_dn17;
        *var_fn25_calc_iq__vsatdibl_rv_slot = var_fn25_calc_iq__vsatdibl_rv;
        *var_fn25_calc_iq__vtdibl_slot = var_fn25_calc_iq__vtdibl;
        *var_fn25_calc_iq__vtdibl_dn16_slot = var_fn25_calc_iq__vtdibl_dn16;
        *var_fn25_calc_iq__vtdibl_dn17_slot = var_fn25_calc_iq__vtdibl_dn17;
        *var_fn25_calc_iq__vtdibl_dn4_slot = var_fn25_calc_iq__vtdibl_dn4;
        *var_fn25_calc_iq__vtdibl_rv_slot = var_fn25_calc_iq__vtdibl_rv;
        *var_fn25_calc_iq__vtheta_slot = var_fn25_calc_iq__vtheta;
        *var_fn25_calc_iq__vtheta_rv_slot = var_fn25_calc_iq__vtheta_rv;
        *var_fn25_calc_iq__vto_slot = var_fn25_calc_iq__vto;
        *var_fn25_calc_iq__vto_rv_slot = var_fn25_calc_iq__vto_rv;
        *var_fn25_calc_iq__vtof_slot = var_fn25_calc_iq__vtof;
        *var_fn25_calc_iq__vtof_dn4_slot = var_fn25_calc_iq__vtof_dn4;
        *var_fn25_calc_iq__vtof_rv_slot = var_fn25_calc_iq__vtof_rv;
        *var_fn25_calc_iq__vtzeta_slot = var_fn25_calc_iq__vtzeta;
        *var_fn25_calc_iq__vtzeta_rv_slot = var_fn25_calc_iq__vtzeta_rv;
        *var_fn25_calc_iq__vzeta_slot = var_fn25_calc_iq__vzeta;
        *var_fn25_calc_iq__vzeta_rv_slot = var_fn25_calc_iq__vzeta_rv;
        *var_fn25_calc_iq__w_slot = var_fn25_calc_iq__w;
        *var_fn25_calc_iq__w_rv_slot = var_fn25_calc_iq__w_rv;
    }

    pub(super) fn stamp_reactive_block_4(
        var_guard24: f64,
        var_fn25_calc_iq__eta_slot: &mut f64,
        var_fn25_calc_iq__eta0_slot: &mut f64,
        var_fn25_calc_iq__eta0_dn16_slot: &mut f64,
        var_fn25_calc_iq__eta0_dn17_slot: &mut f64,
        var_fn25_calc_iq__eta0_dn2_slot: &mut f64,
        var_fn25_calc_iq__eta0_dn4_slot: &mut f64,
        var_fn25_calc_iq__eta0_dn7_slot: &mut f64,
        var_fn25_calc_iq__eta0_rv_slot: &mut f64,
        var_fn25_calc_iq__eta_dn16_slot: &mut f64,
        var_fn25_calc_iq__eta_dn17_slot: &mut f64,
        var_fn25_calc_iq__eta_dn2_slot: &mut f64,
        var_fn25_calc_iq__eta_dn3_slot: &mut f64,
        var_fn25_calc_iq__eta_dn4_slot: &mut f64,
        var_fn25_calc_iq__eta_dn7_slot: &mut f64,
        var_fn25_calc_iq__eta_rv_slot: &mut f64,
        var_fn25_calc_iq__etad_slot: &mut f64,
        var_fn25_calc_iq__etad_dn16_slot: &mut f64,
        var_fn25_calc_iq__etad_dn17_slot: &mut f64,
        var_fn25_calc_iq__etad_dn2_slot: &mut f64,
        var_fn25_calc_iq__etad_dn3_slot: &mut f64,
        var_fn25_calc_iq__etad_dn4_slot: &mut f64,
        var_fn25_calc_iq__etad_dn7_slot: &mut f64,
        var_fn25_calc_iq__etad_rv_slot: &mut f64,
        var_fn25_calc_iq__etas_slot: &mut f64,
        var_fn25_calc_iq__etas0_slot: &mut f64,
        var_fn25_calc_iq__etas0_dn16_slot: &mut f64,
        var_fn25_calc_iq__etas0_dn17_slot: &mut f64,
        var_fn25_calc_iq__etas0_dn2_slot: &mut f64,
        var_fn25_calc_iq__etas0_dn4_slot: &mut f64,
        var_fn25_calc_iq__etas0_dn7_slot: &mut f64,
        var_fn25_calc_iq__etas0_rv_slot: &mut f64,
        var_fn25_calc_iq__etas_dn16_slot: &mut f64,
        var_fn25_calc_iq__etas_dn17_slot: &mut f64,
        var_fn25_calc_iq__etas_dn2_slot: &mut f64,
        var_fn25_calc_iq__etas_dn3_slot: &mut f64,
        var_fn25_calc_iq__etas_dn4_slot: &mut f64,
        var_fn25_calc_iq__etas_dn7_slot: &mut f64,
        var_fn25_calc_iq__etas_rv_slot: &mut f64,
        var_fn25_calc_iq__fds_slot: &mut f64,
        var_fn25_calc_iq__fds_dn16_slot: &mut f64,
        var_fn25_calc_iq__fds_dn17_slot: &mut f64,
        var_fn25_calc_iq__fds_dn2_slot: &mut f64,
        var_fn25_calc_iq__fds_dn3_slot: &mut f64,
        var_fn25_calc_iq__fds_dn4_slot: &mut f64,
        var_fn25_calc_iq__fds_dn7_slot: &mut f64,
        var_fn25_calc_iq__fds_rv_slot: &mut f64,
        var_fn25_calc_iq__ff_slot: &mut f64,
        var_fn25_calc_iq__ff0_slot: &mut f64,
        var_fn25_calc_iq__ff0_dn16_slot: &mut f64,
        var_fn25_calc_iq__ff0_dn17_slot: &mut f64,
        var_fn25_calc_iq__ff0_dn2_slot: &mut f64,
        var_fn25_calc_iq__ff0_dn4_slot: &mut f64,
        var_fn25_calc_iq__ff0_dn7_slot: &mut f64,
        var_fn25_calc_iq__ff0_rv_slot: &mut f64,
        var_fn25_calc_iq__ff_dn16_slot: &mut f64,
        var_fn25_calc_iq__ff_dn17_slot: &mut f64,
        var_fn25_calc_iq__ff_dn2_slot: &mut f64,
        var_fn25_calc_iq__ff_dn3_slot: &mut f64,
        var_fn25_calc_iq__ff_dn4_slot: &mut f64,
        var_fn25_calc_iq__ff_dn7_slot: &mut f64,
        var_fn25_calc_iq__ff_rv_slot: &mut f64,
        var_fn25_calc_iq__ffd_slot: &mut f64,
        var_fn25_calc_iq__ffd_dn16_slot: &mut f64,
        var_fn25_calc_iq__ffd_dn17_slot: &mut f64,
        var_fn25_calc_iq__ffd_dn2_slot: &mut f64,
        var_fn25_calc_iq__ffd_dn3_slot: &mut f64,
        var_fn25_calc_iq__ffd_dn4_slot: &mut f64,
        var_fn25_calc_iq__ffd_dn7_slot: &mut f64,
        var_fn25_calc_iq__ffd_rv_slot: &mut f64,
        var_fn25_calc_iq__ffs0_slot: &mut f64,
        var_fn25_calc_iq__ffs0_dn16_slot: &mut f64,
        var_fn25_calc_iq__ffs0_dn17_slot: &mut f64,
        var_fn25_calc_iq__ffs0_dn2_slot: &mut f64,
        var_fn25_calc_iq__ffs0_dn4_slot: &mut f64,
        var_fn25_calc_iq__ffs0_dn7_slot: &mut f64,
        var_fn25_calc_iq__ffs0_rv_slot: &mut f64,
        var_fn25_calc_iq__fsd_slot: &mut f64,
        var_fn25_calc_iq__fsd_dn16_slot: &mut f64,
        var_fn25_calc_iq__fsd_dn17_slot: &mut f64,
        var_fn25_calc_iq__fsd_dn2_slot: &mut f64,
        var_fn25_calc_iq__fsd_dn3_slot: &mut f64,
        var_fn25_calc_iq__fsd_dn4_slot: &mut f64,
        var_fn25_calc_iq__fsd_dn7_slot: &mut f64,
        var_fn25_calc_iq__fsd_rv_slot: &mut f64,
        var_fn25_calc_iq__muf_slot: &mut f64,
        var_fn25_calc_iq__muf0_slot: &mut f64,
        var_fn25_calc_iq__muf0_dn4_slot: &mut f64,
        var_fn25_calc_iq__muf0_rv_slot: &mut f64,
        var_fn25_calc_iq__muf_dn16_slot: &mut f64,
        var_fn25_calc_iq__muf_dn17_slot: &mut f64,
        var_fn25_calc_iq__muf_dn2_slot: &mut f64,
        var_fn25_calc_iq__muf_dn3_slot: &mut f64,
        var_fn25_calc_iq__muf_dn4_slot: &mut f64,
        var_fn25_calc_iq__muf_dn7_slot: &mut f64,
        var_fn25_calc_iq__muf_rv_slot: &mut f64,
        var_fn25_calc_iq__n0_slot: &mut f64,
        var_fn25_calc_iq__n0_dn4_slot: &mut f64,
        var_fn25_calc_iq__n0_rv_slot: &mut f64,
        var_fn25_calc_iq__qinvd_slot: &mut f64,
        var_fn25_calc_iq__qinvd_dn16_slot: &mut f64,
        var_fn25_calc_iq__qinvd_dn17_slot: &mut f64,
        var_fn25_calc_iq__qinvd_dn2_slot: &mut f64,
        var_fn25_calc_iq__qinvd_dn3_slot: &mut f64,
        var_fn25_calc_iq__qinvd_dn4_slot: &mut f64,
        var_fn25_calc_iq__qinvd_dn7_slot: &mut f64,
        var_fn25_calc_iq__qinvd_rv_slot: &mut f64,
        var_fn25_calc_iq__qinvs_slot: &mut f64,
        var_fn25_calc_iq__qinvs0_slot: &mut f64,
        var_fn25_calc_iq__qinvs0_dn16_slot: &mut f64,
        var_fn25_calc_iq__qinvs0_dn17_slot: &mut f64,
        var_fn25_calc_iq__qinvs0_dn2_slot: &mut f64,
        var_fn25_calc_iq__qinvs0_dn4_slot: &mut f64,
        var_fn25_calc_iq__qinvs0_dn7_slot: &mut f64,
        var_fn25_calc_iq__qinvs0_rv_slot: &mut f64,
        var_fn25_calc_iq__qinvs_dn16_slot: &mut f64,
        var_fn25_calc_iq__qinvs_dn17_slot: &mut f64,
        var_fn25_calc_iq__qinvs_dn2_slot: &mut f64,
        var_fn25_calc_iq__qinvs_dn3_slot: &mut f64,
        var_fn25_calc_iq__qinvs_dn4_slot: &mut f64,
        var_fn25_calc_iq__qinvs_dn7_slot: &mut f64,
        var_fn25_calc_iq__qinvs_rv_slot: &mut f64,
        var_fn25_calc_iq__qinvv_slot: &mut f64,
        var_fn25_calc_iq__qinvv0_slot: &mut f64,
        var_fn25_calc_iq__qinvv0_dn16_slot: &mut f64,
        var_fn25_calc_iq__qinvv0_dn17_slot: &mut f64,
        var_fn25_calc_iq__qinvv0_dn2_slot: &mut f64,
        var_fn25_calc_iq__qinvv0_dn4_slot: &mut f64,
        var_fn25_calc_iq__qinvv0_dn7_slot: &mut f64,
        var_fn25_calc_iq__qinvv0_rv_slot: &mut f64,
        var_fn25_calc_iq__qinvv_dn16_slot: &mut f64,
        var_fn25_calc_iq__qinvv_dn17_slot: &mut f64,
        var_fn25_calc_iq__qinvv_dn2_slot: &mut f64,
        var_fn25_calc_iq__qinvv_dn3_slot: &mut f64,
        var_fn25_calc_iq__qinvv_dn4_slot: &mut f64,
        var_fn25_calc_iq__qinvv_dn7_slot: &mut f64,
        var_fn25_calc_iq__qinvv_rv_slot: &mut f64,
        var_fn25_calc_iq__qref_slot: &mut f64,
        var_fn25_calc_iq__qref0_slot: &mut f64,
        var_fn25_calc_iq__qref0_dn4_slot: &mut f64,
        var_fn25_calc_iq__qref0_rv_slot: &mut f64,
        var_fn25_calc_iq__qref_dn16_slot: &mut f64,
        var_fn25_calc_iq__qref_dn17_slot: &mut f64,
        var_fn25_calc_iq__qref_dn4_slot: &mut f64,
        var_fn25_calc_iq__qref_rv_slot: &mut f64,
        var_fn25_calc_iq__tfacmobin_slot: &mut f64,
        var_fn25_calc_iq__tfacmobin_dn4_slot: &mut f64,
        var_fn25_calc_iq__tfacmobin_rv_slot: &mut f64,
        var_fn25_calc_iq__two_n_phit_slot: &mut f64,
        var_fn25_calc_iq__two_n_phit0_slot: &mut f64,
        var_fn25_calc_iq__two_n_phit0_dn4_slot: &mut f64,
        var_fn25_calc_iq__two_n_phit0_rv_slot: &mut f64,
        var_fn25_calc_iq__two_n_phit_dn16_slot: &mut f64,
        var_fn25_calc_iq__two_n_phit_dn17_slot: &mut f64,
        var_fn25_calc_iq__two_n_phit_dn4_slot: &mut f64,
        var_fn25_calc_iq__two_n_phit_rv_slot: &mut f64,
        var_fn25_calc_iq__vdsat_slot: &mut f64,
        var_fn25_calc_iq__vdsat10_slot: &mut f64,
        var_fn25_calc_iq__vdsat10_dn16_slot: &mut f64,
        var_fn25_calc_iq__vdsat10_dn17_slot: &mut f64,
        var_fn25_calc_iq__vdsat10_dn2_slot: &mut f64,
        var_fn25_calc_iq__vdsat10_dn4_slot: &mut f64,
        var_fn25_calc_iq__vdsat10_dn7_slot: &mut f64,
        var_fn25_calc_iq__vdsat10_rv_slot: &mut f64,
        var_fn25_calc_iq__vdsat_dn16_slot: &mut f64,
        var_fn25_calc_iq__vdsat_dn17_slot: &mut f64,
        var_fn25_calc_iq__vdsat_dn2_slot: &mut f64,
        var_fn25_calc_iq__vdsat_dn3_slot: &mut f64,
        var_fn25_calc_iq__vdsat_dn4_slot: &mut f64,
        var_fn25_calc_iq__vdsat_dn7_slot: &mut f64,
        var_fn25_calc_iq__vdsat_rv_slot: &mut f64,
        var_fn25_calc_iq__vdsats_slot: &mut f64,
        var_fn25_calc_iq__vdsats0_slot: &mut f64,
        var_fn25_calc_iq__vdsats0_dn4_slot: &mut f64,
        var_fn25_calc_iq__vdsats0_rv_slot: &mut f64,
        var_fn25_calc_iq__vdsats1_slot: &mut f64,
        var_fn25_calc_iq__vdsats10_slot: &mut f64,
        var_fn25_calc_iq__vdsats10_dn16_slot: &mut f64,
        var_fn25_calc_iq__vdsats10_dn17_slot: &mut f64,
        var_fn25_calc_iq__vdsats10_dn2_slot: &mut f64,
        var_fn25_calc_iq__vdsats10_dn4_slot: &mut f64,
        var_fn25_calc_iq__vdsats10_dn7_slot: &mut f64,
        var_fn25_calc_iq__vdsats10_rv_slot: &mut f64,
        var_fn25_calc_iq__vdsats1_dn16_slot: &mut f64,
        var_fn25_calc_iq__vdsats1_dn17_slot: &mut f64,
        var_fn25_calc_iq__vdsats1_dn2_slot: &mut f64,
        var_fn25_calc_iq__vdsats1_dn3_slot: &mut f64,
        var_fn25_calc_iq__vdsats1_dn4_slot: &mut f64,
        var_fn25_calc_iq__vdsats1_dn7_slot: &mut f64,
        var_fn25_calc_iq__vdsats1_rv_slot: &mut f64,
        var_fn25_calc_iq__vdsats_dn16_slot: &mut f64,
        var_fn25_calc_iq__vdsats_dn17_slot: &mut f64,
        var_fn25_calc_iq__vdsats_dn2_slot: &mut f64,
        var_fn25_calc_iq__vdsats_dn3_slot: &mut f64,
        var_fn25_calc_iq__vdsats_dn4_slot: &mut f64,
        var_fn25_calc_iq__vdsats_dn7_slot: &mut f64,
        var_fn25_calc_iq__vdsats_rv_slot: &mut f64,
        var_fn25_calc_iq__vdsc_slot: &mut f64,
        var_fn25_calc_iq__vdsc_dn16_slot: &mut f64,
        var_fn25_calc_iq__vdsc_dn17_slot: &mut f64,
        var_fn25_calc_iq__vdsc_dn2_slot: &mut f64,
        var_fn25_calc_iq__vdsc_dn3_slot: &mut f64,
        var_fn25_calc_iq__vdsc_dn4_slot: &mut f64,
        var_fn25_calc_iq__vdsc_dn7_slot: &mut f64,
        var_fn25_calc_iq__vdsc_rv_slot: &mut f64,
        var_fn25_calc_iq__vdx_slot: &mut f64,
        var_fn25_calc_iq__vdx_dn16_slot: &mut f64,
        var_fn25_calc_iq__vdx_dn17_slot: &mut f64,
        var_fn25_calc_iq__vdx_dn2_slot: &mut f64,
        var_fn25_calc_iq__vdx_dn3_slot: &mut f64,
        var_fn25_calc_iq__vdx_dn4_slot: &mut f64,
        var_fn25_calc_iq__vdx_dn7_slot: &mut f64,
        var_fn25_calc_iq__vdx_rv_slot: &mut f64,
        var_fn25_calc_iq__vsx_slot: &mut f64,
        var_fn25_calc_iq__vsx_dn16_slot: &mut f64,
        var_fn25_calc_iq__vsx_dn17_slot: &mut f64,
        var_fn25_calc_iq__vsx_dn2_slot: &mut f64,
        var_fn25_calc_iq__vsx_dn3_slot: &mut f64,
        var_fn25_calc_iq__vsx_dn4_slot: &mut f64,
        var_fn25_calc_iq__vsx_dn7_slot: &mut f64,
        var_fn25_calc_iq__vsx_rv_slot: &mut f64,
        var_fn25_calc_iq__vx_slot: &mut f64,
        var_fn25_calc_iq__vx0_slot: &mut f64,
        var_fn25_calc_iq__vx0_dn4_slot: &mut f64,
        var_fn25_calc_iq__vx0_rv_slot: &mut f64,
        var_fn25_calc_iq__vx_dn16_slot: &mut f64,
        var_fn25_calc_iq__vx_dn17_slot: &mut f64,
        var_fn25_calc_iq__vx_dn2_slot: &mut f64,
        var_fn25_calc_iq__vx_dn3_slot: &mut f64,
        var_fn25_calc_iq__vx_dn4_slot: &mut f64,
        var_fn25_calc_iq__vx_dn7_slot: &mut f64,
        var_fn25_calc_iq__vx_rv_slot: &mut f64,
    ) {
        let mut var_fn25_calc_iq__eta: f64 = *var_fn25_calc_iq__eta_slot;
        let mut var_fn25_calc_iq__eta0: f64 = *var_fn25_calc_iq__eta0_slot;
        let mut var_fn25_calc_iq__eta0_dn16: f64 = *var_fn25_calc_iq__eta0_dn16_slot;
        let mut var_fn25_calc_iq__eta0_dn17: f64 = *var_fn25_calc_iq__eta0_dn17_slot;
        let mut var_fn25_calc_iq__eta0_dn2: f64 = *var_fn25_calc_iq__eta0_dn2_slot;
        let mut var_fn25_calc_iq__eta0_dn4: f64 = *var_fn25_calc_iq__eta0_dn4_slot;
        let mut var_fn25_calc_iq__eta0_dn7: f64 = *var_fn25_calc_iq__eta0_dn7_slot;
        let mut var_fn25_calc_iq__eta0_rv: f64 = *var_fn25_calc_iq__eta0_rv_slot;
        let mut var_fn25_calc_iq__eta_dn16: f64 = *var_fn25_calc_iq__eta_dn16_slot;
        let mut var_fn25_calc_iq__eta_dn17: f64 = *var_fn25_calc_iq__eta_dn17_slot;
        let mut var_fn25_calc_iq__eta_dn2: f64 = *var_fn25_calc_iq__eta_dn2_slot;
        let mut var_fn25_calc_iq__eta_dn3: f64 = *var_fn25_calc_iq__eta_dn3_slot;
        let mut var_fn25_calc_iq__eta_dn4: f64 = *var_fn25_calc_iq__eta_dn4_slot;
        let mut var_fn25_calc_iq__eta_dn7: f64 = *var_fn25_calc_iq__eta_dn7_slot;
        let mut var_fn25_calc_iq__eta_rv: f64 = *var_fn25_calc_iq__eta_rv_slot;
        let mut var_fn25_calc_iq__etad: f64 = *var_fn25_calc_iq__etad_slot;
        let mut var_fn25_calc_iq__etad_dn16: f64 = *var_fn25_calc_iq__etad_dn16_slot;
        let mut var_fn25_calc_iq__etad_dn17: f64 = *var_fn25_calc_iq__etad_dn17_slot;
        let mut var_fn25_calc_iq__etad_dn2: f64 = *var_fn25_calc_iq__etad_dn2_slot;
        let mut var_fn25_calc_iq__etad_dn3: f64 = *var_fn25_calc_iq__etad_dn3_slot;
        let mut var_fn25_calc_iq__etad_dn4: f64 = *var_fn25_calc_iq__etad_dn4_slot;
        let mut var_fn25_calc_iq__etad_dn7: f64 = *var_fn25_calc_iq__etad_dn7_slot;
        let mut var_fn25_calc_iq__etad_rv: f64 = *var_fn25_calc_iq__etad_rv_slot;
        let mut var_fn25_calc_iq__etas: f64 = *var_fn25_calc_iq__etas_slot;
        let mut var_fn25_calc_iq__etas0: f64 = *var_fn25_calc_iq__etas0_slot;
        let mut var_fn25_calc_iq__etas0_dn16: f64 = *var_fn25_calc_iq__etas0_dn16_slot;
        let mut var_fn25_calc_iq__etas0_dn17: f64 = *var_fn25_calc_iq__etas0_dn17_slot;
        let mut var_fn25_calc_iq__etas0_dn2: f64 = *var_fn25_calc_iq__etas0_dn2_slot;
        let mut var_fn25_calc_iq__etas0_dn4: f64 = *var_fn25_calc_iq__etas0_dn4_slot;
        let mut var_fn25_calc_iq__etas0_dn7: f64 = *var_fn25_calc_iq__etas0_dn7_slot;
        let mut var_fn25_calc_iq__etas0_rv: f64 = *var_fn25_calc_iq__etas0_rv_slot;
        let mut var_fn25_calc_iq__etas_dn16: f64 = *var_fn25_calc_iq__etas_dn16_slot;
        let mut var_fn25_calc_iq__etas_dn17: f64 = *var_fn25_calc_iq__etas_dn17_slot;
        let mut var_fn25_calc_iq__etas_dn2: f64 = *var_fn25_calc_iq__etas_dn2_slot;
        let mut var_fn25_calc_iq__etas_dn3: f64 = *var_fn25_calc_iq__etas_dn3_slot;
        let mut var_fn25_calc_iq__etas_dn4: f64 = *var_fn25_calc_iq__etas_dn4_slot;
        let mut var_fn25_calc_iq__etas_dn7: f64 = *var_fn25_calc_iq__etas_dn7_slot;
        let mut var_fn25_calc_iq__etas_rv: f64 = *var_fn25_calc_iq__etas_rv_slot;
        let mut var_fn25_calc_iq__fds: f64 = *var_fn25_calc_iq__fds_slot;
        let mut var_fn25_calc_iq__fds_dn16: f64 = *var_fn25_calc_iq__fds_dn16_slot;
        let mut var_fn25_calc_iq__fds_dn17: f64 = *var_fn25_calc_iq__fds_dn17_slot;
        let mut var_fn25_calc_iq__fds_dn2: f64 = *var_fn25_calc_iq__fds_dn2_slot;
        let mut var_fn25_calc_iq__fds_dn3: f64 = *var_fn25_calc_iq__fds_dn3_slot;
        let mut var_fn25_calc_iq__fds_dn4: f64 = *var_fn25_calc_iq__fds_dn4_slot;
        let mut var_fn25_calc_iq__fds_dn7: f64 = *var_fn25_calc_iq__fds_dn7_slot;
        let mut var_fn25_calc_iq__fds_rv: f64 = *var_fn25_calc_iq__fds_rv_slot;
        let mut var_fn25_calc_iq__ff: f64 = *var_fn25_calc_iq__ff_slot;
        let mut var_fn25_calc_iq__ff0: f64 = *var_fn25_calc_iq__ff0_slot;
        let mut var_fn25_calc_iq__ff0_dn16: f64 = *var_fn25_calc_iq__ff0_dn16_slot;
        let mut var_fn25_calc_iq__ff0_dn17: f64 = *var_fn25_calc_iq__ff0_dn17_slot;
        let mut var_fn25_calc_iq__ff0_dn2: f64 = *var_fn25_calc_iq__ff0_dn2_slot;
        let mut var_fn25_calc_iq__ff0_dn4: f64 = *var_fn25_calc_iq__ff0_dn4_slot;
        let mut var_fn25_calc_iq__ff0_dn7: f64 = *var_fn25_calc_iq__ff0_dn7_slot;
        let mut var_fn25_calc_iq__ff0_rv: f64 = *var_fn25_calc_iq__ff0_rv_slot;
        let mut var_fn25_calc_iq__ff_dn16: f64 = *var_fn25_calc_iq__ff_dn16_slot;
        let mut var_fn25_calc_iq__ff_dn17: f64 = *var_fn25_calc_iq__ff_dn17_slot;
        let mut var_fn25_calc_iq__ff_dn2: f64 = *var_fn25_calc_iq__ff_dn2_slot;
        let mut var_fn25_calc_iq__ff_dn3: f64 = *var_fn25_calc_iq__ff_dn3_slot;
        let mut var_fn25_calc_iq__ff_dn4: f64 = *var_fn25_calc_iq__ff_dn4_slot;
        let mut var_fn25_calc_iq__ff_dn7: f64 = *var_fn25_calc_iq__ff_dn7_slot;
        let mut var_fn25_calc_iq__ff_rv: f64 = *var_fn25_calc_iq__ff_rv_slot;
        let mut var_fn25_calc_iq__ffd: f64 = *var_fn25_calc_iq__ffd_slot;
        let mut var_fn25_calc_iq__ffd_dn16: f64 = *var_fn25_calc_iq__ffd_dn16_slot;
        let mut var_fn25_calc_iq__ffd_dn17: f64 = *var_fn25_calc_iq__ffd_dn17_slot;
        let mut var_fn25_calc_iq__ffd_dn2: f64 = *var_fn25_calc_iq__ffd_dn2_slot;
        let mut var_fn25_calc_iq__ffd_dn3: f64 = *var_fn25_calc_iq__ffd_dn3_slot;
        let mut var_fn25_calc_iq__ffd_dn4: f64 = *var_fn25_calc_iq__ffd_dn4_slot;
        let mut var_fn25_calc_iq__ffd_dn7: f64 = *var_fn25_calc_iq__ffd_dn7_slot;
        let mut var_fn25_calc_iq__ffd_rv: f64 = *var_fn25_calc_iq__ffd_rv_slot;
        let mut var_fn25_calc_iq__ffs0: f64 = *var_fn25_calc_iq__ffs0_slot;
        let mut var_fn25_calc_iq__ffs0_dn16: f64 = *var_fn25_calc_iq__ffs0_dn16_slot;
        let mut var_fn25_calc_iq__ffs0_dn17: f64 = *var_fn25_calc_iq__ffs0_dn17_slot;
        let mut var_fn25_calc_iq__ffs0_dn2: f64 = *var_fn25_calc_iq__ffs0_dn2_slot;
        let mut var_fn25_calc_iq__ffs0_dn4: f64 = *var_fn25_calc_iq__ffs0_dn4_slot;
        let mut var_fn25_calc_iq__ffs0_dn7: f64 = *var_fn25_calc_iq__ffs0_dn7_slot;
        let mut var_fn25_calc_iq__ffs0_rv: f64 = *var_fn25_calc_iq__ffs0_rv_slot;
        let mut var_fn25_calc_iq__fsd: f64 = *var_fn25_calc_iq__fsd_slot;
        let mut var_fn25_calc_iq__fsd_dn16: f64 = *var_fn25_calc_iq__fsd_dn16_slot;
        let mut var_fn25_calc_iq__fsd_dn17: f64 = *var_fn25_calc_iq__fsd_dn17_slot;
        let mut var_fn25_calc_iq__fsd_dn2: f64 = *var_fn25_calc_iq__fsd_dn2_slot;
        let mut var_fn25_calc_iq__fsd_dn3: f64 = *var_fn25_calc_iq__fsd_dn3_slot;
        let mut var_fn25_calc_iq__fsd_dn4: f64 = *var_fn25_calc_iq__fsd_dn4_slot;
        let mut var_fn25_calc_iq__fsd_dn7: f64 = *var_fn25_calc_iq__fsd_dn7_slot;
        let mut var_fn25_calc_iq__fsd_rv: f64 = *var_fn25_calc_iq__fsd_rv_slot;
        let mut var_fn25_calc_iq__muf: f64 = *var_fn25_calc_iq__muf_slot;
        let mut var_fn25_calc_iq__muf0: f64 = *var_fn25_calc_iq__muf0_slot;
        let mut var_fn25_calc_iq__muf0_dn4: f64 = *var_fn25_calc_iq__muf0_dn4_slot;
        let mut var_fn25_calc_iq__muf0_rv: f64 = *var_fn25_calc_iq__muf0_rv_slot;
        let mut var_fn25_calc_iq__muf_dn16: f64 = *var_fn25_calc_iq__muf_dn16_slot;
        let mut var_fn25_calc_iq__muf_dn17: f64 = *var_fn25_calc_iq__muf_dn17_slot;
        let mut var_fn25_calc_iq__muf_dn2: f64 = *var_fn25_calc_iq__muf_dn2_slot;
        let mut var_fn25_calc_iq__muf_dn3: f64 = *var_fn25_calc_iq__muf_dn3_slot;
        let mut var_fn25_calc_iq__muf_dn4: f64 = *var_fn25_calc_iq__muf_dn4_slot;
        let mut var_fn25_calc_iq__muf_dn7: f64 = *var_fn25_calc_iq__muf_dn7_slot;
        let mut var_fn25_calc_iq__muf_rv: f64 = *var_fn25_calc_iq__muf_rv_slot;
        let mut var_fn25_calc_iq__n0: f64 = *var_fn25_calc_iq__n0_slot;
        let mut var_fn25_calc_iq__n0_dn4: f64 = *var_fn25_calc_iq__n0_dn4_slot;
        let mut var_fn25_calc_iq__n0_rv: f64 = *var_fn25_calc_iq__n0_rv_slot;
        let mut var_fn25_calc_iq__qinvd: f64 = *var_fn25_calc_iq__qinvd_slot;
        let mut var_fn25_calc_iq__qinvd_dn16: f64 = *var_fn25_calc_iq__qinvd_dn16_slot;
        let mut var_fn25_calc_iq__qinvd_dn17: f64 = *var_fn25_calc_iq__qinvd_dn17_slot;
        let mut var_fn25_calc_iq__qinvd_dn2: f64 = *var_fn25_calc_iq__qinvd_dn2_slot;
        let mut var_fn25_calc_iq__qinvd_dn3: f64 = *var_fn25_calc_iq__qinvd_dn3_slot;
        let mut var_fn25_calc_iq__qinvd_dn4: f64 = *var_fn25_calc_iq__qinvd_dn4_slot;
        let mut var_fn25_calc_iq__qinvd_dn7: f64 = *var_fn25_calc_iq__qinvd_dn7_slot;
        let mut var_fn25_calc_iq__qinvd_rv: f64 = *var_fn25_calc_iq__qinvd_rv_slot;
        let mut var_fn25_calc_iq__qinvs: f64 = *var_fn25_calc_iq__qinvs_slot;
        let mut var_fn25_calc_iq__qinvs0: f64 = *var_fn25_calc_iq__qinvs0_slot;
        let mut var_fn25_calc_iq__qinvs0_dn16: f64 = *var_fn25_calc_iq__qinvs0_dn16_slot;
        let mut var_fn25_calc_iq__qinvs0_dn17: f64 = *var_fn25_calc_iq__qinvs0_dn17_slot;
        let mut var_fn25_calc_iq__qinvs0_dn2: f64 = *var_fn25_calc_iq__qinvs0_dn2_slot;
        let mut var_fn25_calc_iq__qinvs0_dn4: f64 = *var_fn25_calc_iq__qinvs0_dn4_slot;
        let mut var_fn25_calc_iq__qinvs0_dn7: f64 = *var_fn25_calc_iq__qinvs0_dn7_slot;
        let mut var_fn25_calc_iq__qinvs0_rv: f64 = *var_fn25_calc_iq__qinvs0_rv_slot;
        let mut var_fn25_calc_iq__qinvs_dn16: f64 = *var_fn25_calc_iq__qinvs_dn16_slot;
        let mut var_fn25_calc_iq__qinvs_dn17: f64 = *var_fn25_calc_iq__qinvs_dn17_slot;
        let mut var_fn25_calc_iq__qinvs_dn2: f64 = *var_fn25_calc_iq__qinvs_dn2_slot;
        let mut var_fn25_calc_iq__qinvs_dn3: f64 = *var_fn25_calc_iq__qinvs_dn3_slot;
        let mut var_fn25_calc_iq__qinvs_dn4: f64 = *var_fn25_calc_iq__qinvs_dn4_slot;
        let mut var_fn25_calc_iq__qinvs_dn7: f64 = *var_fn25_calc_iq__qinvs_dn7_slot;
        let mut var_fn25_calc_iq__qinvs_rv: f64 = *var_fn25_calc_iq__qinvs_rv_slot;
        let mut var_fn25_calc_iq__qinvv: f64 = *var_fn25_calc_iq__qinvv_slot;
        let mut var_fn25_calc_iq__qinvv0: f64 = *var_fn25_calc_iq__qinvv0_slot;
        let mut var_fn25_calc_iq__qinvv0_dn16: f64 = *var_fn25_calc_iq__qinvv0_dn16_slot;
        let mut var_fn25_calc_iq__qinvv0_dn17: f64 = *var_fn25_calc_iq__qinvv0_dn17_slot;
        let mut var_fn25_calc_iq__qinvv0_dn2: f64 = *var_fn25_calc_iq__qinvv0_dn2_slot;
        let mut var_fn25_calc_iq__qinvv0_dn4: f64 = *var_fn25_calc_iq__qinvv0_dn4_slot;
        let mut var_fn25_calc_iq__qinvv0_dn7: f64 = *var_fn25_calc_iq__qinvv0_dn7_slot;
        let mut var_fn25_calc_iq__qinvv0_rv: f64 = *var_fn25_calc_iq__qinvv0_rv_slot;
        let mut var_fn25_calc_iq__qinvv_dn16: f64 = *var_fn25_calc_iq__qinvv_dn16_slot;
        let mut var_fn25_calc_iq__qinvv_dn17: f64 = *var_fn25_calc_iq__qinvv_dn17_slot;
        let mut var_fn25_calc_iq__qinvv_dn2: f64 = *var_fn25_calc_iq__qinvv_dn2_slot;
        let mut var_fn25_calc_iq__qinvv_dn3: f64 = *var_fn25_calc_iq__qinvv_dn3_slot;
        let mut var_fn25_calc_iq__qinvv_dn4: f64 = *var_fn25_calc_iq__qinvv_dn4_slot;
        let mut var_fn25_calc_iq__qinvv_dn7: f64 = *var_fn25_calc_iq__qinvv_dn7_slot;
        let mut var_fn25_calc_iq__qinvv_rv: f64 = *var_fn25_calc_iq__qinvv_rv_slot;
        let mut var_fn25_calc_iq__qref: f64 = *var_fn25_calc_iq__qref_slot;
        let mut var_fn25_calc_iq__qref0: f64 = *var_fn25_calc_iq__qref0_slot;
        let mut var_fn25_calc_iq__qref0_dn4: f64 = *var_fn25_calc_iq__qref0_dn4_slot;
        let mut var_fn25_calc_iq__qref0_rv: f64 = *var_fn25_calc_iq__qref0_rv_slot;
        let mut var_fn25_calc_iq__qref_dn16: f64 = *var_fn25_calc_iq__qref_dn16_slot;
        let mut var_fn25_calc_iq__qref_dn17: f64 = *var_fn25_calc_iq__qref_dn17_slot;
        let mut var_fn25_calc_iq__qref_dn4: f64 = *var_fn25_calc_iq__qref_dn4_slot;
        let mut var_fn25_calc_iq__qref_rv: f64 = *var_fn25_calc_iq__qref_rv_slot;
        let mut var_fn25_calc_iq__tfacmobin: f64 = *var_fn25_calc_iq__tfacmobin_slot;
        let mut var_fn25_calc_iq__tfacmobin_dn4: f64 = *var_fn25_calc_iq__tfacmobin_dn4_slot;
        let mut var_fn25_calc_iq__tfacmobin_rv: f64 = *var_fn25_calc_iq__tfacmobin_rv_slot;
        let mut var_fn25_calc_iq__two_n_phit: f64 = *var_fn25_calc_iq__two_n_phit_slot;
        let mut var_fn25_calc_iq__two_n_phit0: f64 = *var_fn25_calc_iq__two_n_phit0_slot;
        let mut var_fn25_calc_iq__two_n_phit0_dn4: f64 = *var_fn25_calc_iq__two_n_phit0_dn4_slot;
        let mut var_fn25_calc_iq__two_n_phit0_rv: f64 = *var_fn25_calc_iq__two_n_phit0_rv_slot;
        let mut var_fn25_calc_iq__two_n_phit_dn16: f64 = *var_fn25_calc_iq__two_n_phit_dn16_slot;
        let mut var_fn25_calc_iq__two_n_phit_dn17: f64 = *var_fn25_calc_iq__two_n_phit_dn17_slot;
        let mut var_fn25_calc_iq__two_n_phit_dn4: f64 = *var_fn25_calc_iq__two_n_phit_dn4_slot;
        let mut var_fn25_calc_iq__two_n_phit_rv: f64 = *var_fn25_calc_iq__two_n_phit_rv_slot;
        let mut var_fn25_calc_iq__vdsat: f64 = *var_fn25_calc_iq__vdsat_slot;
        let mut var_fn25_calc_iq__vdsat10: f64 = *var_fn25_calc_iq__vdsat10_slot;
        let mut var_fn25_calc_iq__vdsat10_dn16: f64 = *var_fn25_calc_iq__vdsat10_dn16_slot;
        let mut var_fn25_calc_iq__vdsat10_dn17: f64 = *var_fn25_calc_iq__vdsat10_dn17_slot;
        let mut var_fn25_calc_iq__vdsat10_dn2: f64 = *var_fn25_calc_iq__vdsat10_dn2_slot;
        let mut var_fn25_calc_iq__vdsat10_dn4: f64 = *var_fn25_calc_iq__vdsat10_dn4_slot;
        let mut var_fn25_calc_iq__vdsat10_dn7: f64 = *var_fn25_calc_iq__vdsat10_dn7_slot;
        let mut var_fn25_calc_iq__vdsat10_rv: f64 = *var_fn25_calc_iq__vdsat10_rv_slot;
        let mut var_fn25_calc_iq__vdsat_dn16: f64 = *var_fn25_calc_iq__vdsat_dn16_slot;
        let mut var_fn25_calc_iq__vdsat_dn17: f64 = *var_fn25_calc_iq__vdsat_dn17_slot;
        let mut var_fn25_calc_iq__vdsat_dn2: f64 = *var_fn25_calc_iq__vdsat_dn2_slot;
        let mut var_fn25_calc_iq__vdsat_dn3: f64 = *var_fn25_calc_iq__vdsat_dn3_slot;
        let mut var_fn25_calc_iq__vdsat_dn4: f64 = *var_fn25_calc_iq__vdsat_dn4_slot;
        let mut var_fn25_calc_iq__vdsat_dn7: f64 = *var_fn25_calc_iq__vdsat_dn7_slot;
        let mut var_fn25_calc_iq__vdsat_rv: f64 = *var_fn25_calc_iq__vdsat_rv_slot;
        let mut var_fn25_calc_iq__vdsats: f64 = *var_fn25_calc_iq__vdsats_slot;
        let mut var_fn25_calc_iq__vdsats0: f64 = *var_fn25_calc_iq__vdsats0_slot;
        let mut var_fn25_calc_iq__vdsats0_dn4: f64 = *var_fn25_calc_iq__vdsats0_dn4_slot;
        let mut var_fn25_calc_iq__vdsats0_rv: f64 = *var_fn25_calc_iq__vdsats0_rv_slot;
        let mut var_fn25_calc_iq__vdsats1: f64 = *var_fn25_calc_iq__vdsats1_slot;
        let mut var_fn25_calc_iq__vdsats10: f64 = *var_fn25_calc_iq__vdsats10_slot;
        let mut var_fn25_calc_iq__vdsats10_dn16: f64 = *var_fn25_calc_iq__vdsats10_dn16_slot;
        let mut var_fn25_calc_iq__vdsats10_dn17: f64 = *var_fn25_calc_iq__vdsats10_dn17_slot;
        let mut var_fn25_calc_iq__vdsats10_dn2: f64 = *var_fn25_calc_iq__vdsats10_dn2_slot;
        let mut var_fn25_calc_iq__vdsats10_dn4: f64 = *var_fn25_calc_iq__vdsats10_dn4_slot;
        let mut var_fn25_calc_iq__vdsats10_dn7: f64 = *var_fn25_calc_iq__vdsats10_dn7_slot;
        let mut var_fn25_calc_iq__vdsats10_rv: f64 = *var_fn25_calc_iq__vdsats10_rv_slot;
        let mut var_fn25_calc_iq__vdsats1_dn16: f64 = *var_fn25_calc_iq__vdsats1_dn16_slot;
        let mut var_fn25_calc_iq__vdsats1_dn17: f64 = *var_fn25_calc_iq__vdsats1_dn17_slot;
        let mut var_fn25_calc_iq__vdsats1_dn2: f64 = *var_fn25_calc_iq__vdsats1_dn2_slot;
        let mut var_fn25_calc_iq__vdsats1_dn3: f64 = *var_fn25_calc_iq__vdsats1_dn3_slot;
        let mut var_fn25_calc_iq__vdsats1_dn4: f64 = *var_fn25_calc_iq__vdsats1_dn4_slot;
        let mut var_fn25_calc_iq__vdsats1_dn7: f64 = *var_fn25_calc_iq__vdsats1_dn7_slot;
        let mut var_fn25_calc_iq__vdsats1_rv: f64 = *var_fn25_calc_iq__vdsats1_rv_slot;
        let mut var_fn25_calc_iq__vdsats_dn16: f64 = *var_fn25_calc_iq__vdsats_dn16_slot;
        let mut var_fn25_calc_iq__vdsats_dn17: f64 = *var_fn25_calc_iq__vdsats_dn17_slot;
        let mut var_fn25_calc_iq__vdsats_dn2: f64 = *var_fn25_calc_iq__vdsats_dn2_slot;
        let mut var_fn25_calc_iq__vdsats_dn3: f64 = *var_fn25_calc_iq__vdsats_dn3_slot;
        let mut var_fn25_calc_iq__vdsats_dn4: f64 = *var_fn25_calc_iq__vdsats_dn4_slot;
        let mut var_fn25_calc_iq__vdsats_dn7: f64 = *var_fn25_calc_iq__vdsats_dn7_slot;
        let mut var_fn25_calc_iq__vdsats_rv: f64 = *var_fn25_calc_iq__vdsats_rv_slot;
        let mut var_fn25_calc_iq__vdsc: f64 = *var_fn25_calc_iq__vdsc_slot;
        let mut var_fn25_calc_iq__vdsc_dn16: f64 = *var_fn25_calc_iq__vdsc_dn16_slot;
        let mut var_fn25_calc_iq__vdsc_dn17: f64 = *var_fn25_calc_iq__vdsc_dn17_slot;
        let mut var_fn25_calc_iq__vdsc_dn2: f64 = *var_fn25_calc_iq__vdsc_dn2_slot;
        let mut var_fn25_calc_iq__vdsc_dn3: f64 = *var_fn25_calc_iq__vdsc_dn3_slot;
        let mut var_fn25_calc_iq__vdsc_dn4: f64 = *var_fn25_calc_iq__vdsc_dn4_slot;
        let mut var_fn25_calc_iq__vdsc_dn7: f64 = *var_fn25_calc_iq__vdsc_dn7_slot;
        let mut var_fn25_calc_iq__vdsc_rv: f64 = *var_fn25_calc_iq__vdsc_rv_slot;
        let mut var_fn25_calc_iq__vdx: f64 = *var_fn25_calc_iq__vdx_slot;
        let mut var_fn25_calc_iq__vdx_dn16: f64 = *var_fn25_calc_iq__vdx_dn16_slot;
        let mut var_fn25_calc_iq__vdx_dn17: f64 = *var_fn25_calc_iq__vdx_dn17_slot;
        let mut var_fn25_calc_iq__vdx_dn2: f64 = *var_fn25_calc_iq__vdx_dn2_slot;
        let mut var_fn25_calc_iq__vdx_dn3: f64 = *var_fn25_calc_iq__vdx_dn3_slot;
        let mut var_fn25_calc_iq__vdx_dn4: f64 = *var_fn25_calc_iq__vdx_dn4_slot;
        let mut var_fn25_calc_iq__vdx_dn7: f64 = *var_fn25_calc_iq__vdx_dn7_slot;
        let mut var_fn25_calc_iq__vdx_rv: f64 = *var_fn25_calc_iq__vdx_rv_slot;
        let mut var_fn25_calc_iq__vsx: f64 = *var_fn25_calc_iq__vsx_slot;
        let mut var_fn25_calc_iq__vsx_dn16: f64 = *var_fn25_calc_iq__vsx_dn16_slot;
        let mut var_fn25_calc_iq__vsx_dn17: f64 = *var_fn25_calc_iq__vsx_dn17_slot;
        let mut var_fn25_calc_iq__vsx_dn2: f64 = *var_fn25_calc_iq__vsx_dn2_slot;
        let mut var_fn25_calc_iq__vsx_dn3: f64 = *var_fn25_calc_iq__vsx_dn3_slot;
        let mut var_fn25_calc_iq__vsx_dn4: f64 = *var_fn25_calc_iq__vsx_dn4_slot;
        let mut var_fn25_calc_iq__vsx_dn7: f64 = *var_fn25_calc_iq__vsx_dn7_slot;
        let mut var_fn25_calc_iq__vsx_rv: f64 = *var_fn25_calc_iq__vsx_rv_slot;
        let mut var_fn25_calc_iq__vx: f64 = *var_fn25_calc_iq__vx_slot;
        let mut var_fn25_calc_iq__vx0: f64 = *var_fn25_calc_iq__vx0_slot;
        let mut var_fn25_calc_iq__vx0_dn4: f64 = *var_fn25_calc_iq__vx0_dn4_slot;
        let mut var_fn25_calc_iq__vx0_rv: f64 = *var_fn25_calc_iq__vx0_rv_slot;
        let mut var_fn25_calc_iq__vx_dn16: f64 = *var_fn25_calc_iq__vx_dn16_slot;
        let mut var_fn25_calc_iq__vx_dn17: f64 = *var_fn25_calc_iq__vx_dn17_slot;
        let mut var_fn25_calc_iq__vx_dn2: f64 = *var_fn25_calc_iq__vx_dn2_slot;
        let mut var_fn25_calc_iq__vx_dn3: f64 = *var_fn25_calc_iq__vx_dn3_slot;
        let mut var_fn25_calc_iq__vx_dn4: f64 = *var_fn25_calc_iq__vx_dn4_slot;
        let mut var_fn25_calc_iq__vx_dn7: f64 = *var_fn25_calc_iq__vx_dn7_slot;
        let mut var_fn25_calc_iq__vx_rv: f64 = *var_fn25_calc_iq__vx_rv_slot;

        let (assign2250_e4085, assign2250_e4085_d_n4, assign2250_e4085_d_n16, assign2250_e4085_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__two_n_phit, var_fn25_calc_iq__two_n_phit_dn4, var_fn25_calc_iq__two_n_phit_dn16, var_fn25_calc_iq__two_n_phit_dn17,)
    }
};
        var_fn25_calc_iq__two_n_phit = assign2250_e4085;
        var_fn25_calc_iq__two_n_phit_dn4 = assign2250_e4085_d_n4;
        var_fn25_calc_iq__two_n_phit_dn16 = assign2250_e4085_d_n16;
        var_fn25_calc_iq__two_n_phit_dn17 = assign2250_e4085_d_n17;
        var_fn25_calc_iq__two_n_phit_rv = 0.0;

        let (assign2260_e4089, assign2260_e4089_d_n4, assign2260_e4089_d_n16, assign2260_e4089_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__qref, var_fn25_calc_iq__qref_dn4, var_fn25_calc_iq__qref_dn16, var_fn25_calc_iq__qref_dn17,)
    }
};
        var_fn25_calc_iq__qref = assign2260_e4089;
        var_fn25_calc_iq__qref_dn4 = assign2260_e4089_d_n4;
        var_fn25_calc_iq__qref_dn16 = assign2260_e4089_d_n16;
        var_fn25_calc_iq__qref_dn17 = assign2260_e4089_d_n17;
        var_fn25_calc_iq__qref_rv = 0.0;

        let (assign2270_e4093, assign2270_e4093_d_n2, assign2270_e4093_d_n3, assign2270_e4093_d_n4, assign2270_e4093_d_n7, assign2270_e4093_d_n16, assign2270_e4093_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__etas, var_fn25_calc_iq__etas_dn2, var_fn25_calc_iq__etas_dn3, var_fn25_calc_iq__etas_dn4, var_fn25_calc_iq__etas_dn7, var_fn25_calc_iq__etas_dn16, var_fn25_calc_iq__etas_dn17,)
    }
};
        var_fn25_calc_iq__etas = assign2270_e4093;
        var_fn25_calc_iq__etas_dn2 = assign2270_e4093_d_n2;
        var_fn25_calc_iq__etas_dn3 = assign2270_e4093_d_n3;
        var_fn25_calc_iq__etas_dn4 = assign2270_e4093_d_n4;
        var_fn25_calc_iq__etas_dn7 = assign2270_e4093_d_n7;
        var_fn25_calc_iq__etas_dn16 = assign2270_e4093_d_n16;
        var_fn25_calc_iq__etas_dn17 = assign2270_e4093_d_n17;
        var_fn25_calc_iq__etas_rv = 0.0;

        let (assign2280_e4097, assign2280_e4097_d_n2, assign2280_e4097_d_n3, assign2280_e4097_d_n4, assign2280_e4097_d_n7, assign2280_e4097_d_n16, assign2280_e4097_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__qinvs, var_fn25_calc_iq__qinvs_dn2, var_fn25_calc_iq__qinvs_dn3, var_fn25_calc_iq__qinvs_dn4, var_fn25_calc_iq__qinvs_dn7, var_fn25_calc_iq__qinvs_dn16, var_fn25_calc_iq__qinvs_dn17,)
    }
};
        var_fn25_calc_iq__qinvs = assign2280_e4097;
        var_fn25_calc_iq__qinvs_dn2 = assign2280_e4097_d_n2;
        var_fn25_calc_iq__qinvs_dn3 = assign2280_e4097_d_n3;
        var_fn25_calc_iq__qinvs_dn4 = assign2280_e4097_d_n4;
        var_fn25_calc_iq__qinvs_dn7 = assign2280_e4097_d_n7;
        var_fn25_calc_iq__qinvs_dn16 = assign2280_e4097_d_n16;
        var_fn25_calc_iq__qinvs_dn17 = assign2280_e4097_d_n17;
        var_fn25_calc_iq__qinvs_rv = 0.0;

        let (assign2290_e4101, assign2290_e4101_d_n2, assign2290_e4101_d_n3, assign2290_e4101_d_n4, assign2290_e4101_d_n7, assign2290_e4101_d_n16, assign2290_e4101_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__muf, var_fn25_calc_iq__muf_dn2, var_fn25_calc_iq__muf_dn3, var_fn25_calc_iq__muf_dn4, var_fn25_calc_iq__muf_dn7, var_fn25_calc_iq__muf_dn16, var_fn25_calc_iq__muf_dn17,)
    }
};
        var_fn25_calc_iq__muf = assign2290_e4101;
        var_fn25_calc_iq__muf_dn2 = assign2290_e4101_d_n2;
        var_fn25_calc_iq__muf_dn3 = assign2290_e4101_d_n3;
        var_fn25_calc_iq__muf_dn4 = assign2290_e4101_d_n4;
        var_fn25_calc_iq__muf_dn7 = assign2290_e4101_d_n7;
        var_fn25_calc_iq__muf_dn16 = assign2290_e4101_d_n16;
        var_fn25_calc_iq__muf_dn17 = assign2290_e4101_d_n17;
        var_fn25_calc_iq__muf_rv = 0.0;

        let (assign2300_e4105, assign2300_e4105_d_n2, assign2300_e4105_d_n3, assign2300_e4105_d_n4, assign2300_e4105_d_n7, assign2300_e4105_d_n16, assign2300_e4105_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__vx, var_fn25_calc_iq__vx_dn2, var_fn25_calc_iq__vx_dn3, var_fn25_calc_iq__vx_dn4, var_fn25_calc_iq__vx_dn7, var_fn25_calc_iq__vx_dn16, var_fn25_calc_iq__vx_dn17,)
    }
};
        var_fn25_calc_iq__vx = assign2300_e4105;
        var_fn25_calc_iq__vx_dn2 = assign2300_e4105_d_n2;
        var_fn25_calc_iq__vx_dn3 = assign2300_e4105_d_n3;
        var_fn25_calc_iq__vx_dn4 = assign2300_e4105_d_n4;
        var_fn25_calc_iq__vx_dn7 = assign2300_e4105_d_n7;
        var_fn25_calc_iq__vx_dn16 = assign2300_e4105_d_n16;
        var_fn25_calc_iq__vx_dn17 = assign2300_e4105_d_n17;
        var_fn25_calc_iq__vx_rv = 0.0;

        let (assign2320_e4113, assign2320_e4113_d_n4,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__n0, var_fn25_calc_iq__n0_dn4,)
    }
};
        var_fn25_calc_iq__n0 = assign2320_e4113;
        var_fn25_calc_iq__n0_dn4 = assign2320_e4113_d_n4;
        var_fn25_calc_iq__n0_rv = 0.0;

        let (assign2330_e4117, assign2330_e4117_d_n2, assign2330_e4117_d_n4, assign2330_e4117_d_n7, assign2330_e4117_d_n16, assign2330_e4117_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__ffs0, var_fn25_calc_iq__ffs0_dn2, var_fn25_calc_iq__ffs0_dn4, var_fn25_calc_iq__ffs0_dn7, var_fn25_calc_iq__ffs0_dn16, var_fn25_calc_iq__ffs0_dn17,)
    }
};
        var_fn25_calc_iq__ffs0 = assign2330_e4117;
        var_fn25_calc_iq__ffs0_dn2 = assign2330_e4117_d_n2;
        var_fn25_calc_iq__ffs0_dn4 = assign2330_e4117_d_n4;
        var_fn25_calc_iq__ffs0_dn7 = assign2330_e4117_d_n7;
        var_fn25_calc_iq__ffs0_dn16 = assign2330_e4117_d_n16;
        var_fn25_calc_iq__ffs0_dn17 = assign2330_e4117_d_n17;
        var_fn25_calc_iq__ffs0_rv = 0.0;

        let (assign2340_e4121, assign2340_e4121_d_n4,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__two_n_phit0, var_fn25_calc_iq__two_n_phit0_dn4,)
    }
};
        var_fn25_calc_iq__two_n_phit0 = assign2340_e4121;
        var_fn25_calc_iq__two_n_phit0_dn4 = assign2340_e4121_d_n4;
        var_fn25_calc_iq__two_n_phit0_rv = 0.0;

        let (assign2350_e4125, assign2350_e4125_d_n4,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__qref0, var_fn25_calc_iq__qref0_dn4,)
    }
};
        var_fn25_calc_iq__qref0 = assign2350_e4125;
        var_fn25_calc_iq__qref0_dn4 = assign2350_e4125_d_n4;
        var_fn25_calc_iq__qref0_rv = 0.0;

        let (assign2360_e4129, assign2360_e4129_d_n2, assign2360_e4129_d_n4, assign2360_e4129_d_n7, assign2360_e4129_d_n16, assign2360_e4129_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__etas0, var_fn25_calc_iq__etas0_dn2, var_fn25_calc_iq__etas0_dn4, var_fn25_calc_iq__etas0_dn7, var_fn25_calc_iq__etas0_dn16, var_fn25_calc_iq__etas0_dn17,)
    }
};
        var_fn25_calc_iq__etas0 = assign2360_e4129;
        var_fn25_calc_iq__etas0_dn2 = assign2360_e4129_d_n2;
        var_fn25_calc_iq__etas0_dn4 = assign2360_e4129_d_n4;
        var_fn25_calc_iq__etas0_dn7 = assign2360_e4129_d_n7;
        var_fn25_calc_iq__etas0_dn16 = assign2360_e4129_d_n16;
        var_fn25_calc_iq__etas0_dn17 = assign2360_e4129_d_n17;
        var_fn25_calc_iq__etas0_rv = 0.0;

        let (assign2370_e4133, assign2370_e4133_d_n2, assign2370_e4133_d_n4, assign2370_e4133_d_n7, assign2370_e4133_d_n16, assign2370_e4133_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__qinvs0, var_fn25_calc_iq__qinvs0_dn2, var_fn25_calc_iq__qinvs0_dn4, var_fn25_calc_iq__qinvs0_dn7, var_fn25_calc_iq__qinvs0_dn16, var_fn25_calc_iq__qinvs0_dn17,)
    }
};
        var_fn25_calc_iq__qinvs0 = assign2370_e4133;
        var_fn25_calc_iq__qinvs0_dn2 = assign2370_e4133_d_n2;
        var_fn25_calc_iq__qinvs0_dn4 = assign2370_e4133_d_n4;
        var_fn25_calc_iq__qinvs0_dn7 = assign2370_e4133_d_n7;
        var_fn25_calc_iq__qinvs0_dn16 = assign2370_e4133_d_n16;
        var_fn25_calc_iq__qinvs0_dn17 = assign2370_e4133_d_n17;
        var_fn25_calc_iq__qinvs0_rv = 0.0;

        let (assign2380_e4137, assign2380_e4137_d_n4,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__muf0, var_fn25_calc_iq__muf0_dn4,)
    }
};
        var_fn25_calc_iq__muf0 = assign2380_e4137;
        var_fn25_calc_iq__muf0_dn4 = assign2380_e4137_d_n4;
        var_fn25_calc_iq__muf0_rv = 0.0;

        let (assign2390_e4141, assign2390_e4141_d_n4,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__vx0, var_fn25_calc_iq__vx0_dn4,)
    }
};
        var_fn25_calc_iq__vx0 = assign2390_e4141;
        var_fn25_calc_iq__vx0_dn4 = assign2390_e4141_d_n4;
        var_fn25_calc_iq__vx0_rv = 0.0;

        let (assign2400_e4145, assign2400_e4145_d_n4,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__tfacmobin, var_fn25_calc_iq__tfacmobin_dn4,)
    }
};
        var_fn25_calc_iq__tfacmobin = assign2400_e4145;
        var_fn25_calc_iq__tfacmobin_dn4 = assign2400_e4145_d_n4;
        var_fn25_calc_iq__tfacmobin_rv = 0.0;

        let (assign2410_e4149, assign2410_e4149_d_n2, assign2410_e4149_d_n3, assign2410_e4149_d_n4, assign2410_e4149_d_n7, assign2410_e4149_d_n16, assign2410_e4149_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__ff, var_fn25_calc_iq__ff_dn2, var_fn25_calc_iq__ff_dn3, var_fn25_calc_iq__ff_dn4, var_fn25_calc_iq__ff_dn7, var_fn25_calc_iq__ff_dn16, var_fn25_calc_iq__ff_dn17,)
    }
};
        var_fn25_calc_iq__ff = assign2410_e4149;
        var_fn25_calc_iq__ff_dn2 = assign2410_e4149_d_n2;
        var_fn25_calc_iq__ff_dn3 = assign2410_e4149_d_n3;
        var_fn25_calc_iq__ff_dn4 = assign2410_e4149_d_n4;
        var_fn25_calc_iq__ff_dn7 = assign2410_e4149_d_n7;
        var_fn25_calc_iq__ff_dn16 = assign2410_e4149_d_n16;
        var_fn25_calc_iq__ff_dn17 = assign2410_e4149_d_n17;
        var_fn25_calc_iq__ff_rv = 0.0;

        let (assign2420_e4153, assign2420_e4153_d_n2, assign2420_e4153_d_n3, assign2420_e4153_d_n4, assign2420_e4153_d_n7, assign2420_e4153_d_n16, assign2420_e4153_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__eta, var_fn25_calc_iq__eta_dn2, var_fn25_calc_iq__eta_dn3, var_fn25_calc_iq__eta_dn4, var_fn25_calc_iq__eta_dn7, var_fn25_calc_iq__eta_dn16, var_fn25_calc_iq__eta_dn17,)
    }
};
        var_fn25_calc_iq__eta = assign2420_e4153;
        var_fn25_calc_iq__eta_dn2 = assign2420_e4153_d_n2;
        var_fn25_calc_iq__eta_dn3 = assign2420_e4153_d_n3;
        var_fn25_calc_iq__eta_dn4 = assign2420_e4153_d_n4;
        var_fn25_calc_iq__eta_dn7 = assign2420_e4153_d_n7;
        var_fn25_calc_iq__eta_dn16 = assign2420_e4153_d_n16;
        var_fn25_calc_iq__eta_dn17 = assign2420_e4153_d_n17;
        var_fn25_calc_iq__eta_rv = 0.0;

        let (assign2430_e4157, assign2430_e4157_d_n2, assign2430_e4157_d_n3, assign2430_e4157_d_n4, assign2430_e4157_d_n7, assign2430_e4157_d_n16, assign2430_e4157_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__qinvv, var_fn25_calc_iq__qinvv_dn2, var_fn25_calc_iq__qinvv_dn3, var_fn25_calc_iq__qinvv_dn4, var_fn25_calc_iq__qinvv_dn7, var_fn25_calc_iq__qinvv_dn16, var_fn25_calc_iq__qinvv_dn17,)
    }
};
        var_fn25_calc_iq__qinvv = assign2430_e4157;
        var_fn25_calc_iq__qinvv_dn2 = assign2430_e4157_d_n2;
        var_fn25_calc_iq__qinvv_dn3 = assign2430_e4157_d_n3;
        var_fn25_calc_iq__qinvv_dn4 = assign2430_e4157_d_n4;
        var_fn25_calc_iq__qinvv_dn7 = assign2430_e4157_d_n7;
        var_fn25_calc_iq__qinvv_dn16 = assign2430_e4157_d_n16;
        var_fn25_calc_iq__qinvv_dn17 = assign2430_e4157_d_n17;
        var_fn25_calc_iq__qinvv_rv = 0.0;

        let (assign2440_e4161, assign2440_e4161_d_n2, assign2440_e4161_d_n4, assign2440_e4161_d_n7, assign2440_e4161_d_n16, assign2440_e4161_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__ff0, var_fn25_calc_iq__ff0_dn2, var_fn25_calc_iq__ff0_dn4, var_fn25_calc_iq__ff0_dn7, var_fn25_calc_iq__ff0_dn16, var_fn25_calc_iq__ff0_dn17,)
    }
};
        var_fn25_calc_iq__ff0 = assign2440_e4161;
        var_fn25_calc_iq__ff0_dn2 = assign2440_e4161_d_n2;
        var_fn25_calc_iq__ff0_dn4 = assign2440_e4161_d_n4;
        var_fn25_calc_iq__ff0_dn7 = assign2440_e4161_d_n7;
        var_fn25_calc_iq__ff0_dn16 = assign2440_e4161_d_n16;
        var_fn25_calc_iq__ff0_dn17 = assign2440_e4161_d_n17;
        var_fn25_calc_iq__ff0_rv = 0.0;

        let (assign2450_e4165, assign2450_e4165_d_n2, assign2450_e4165_d_n4, assign2450_e4165_d_n7, assign2450_e4165_d_n16, assign2450_e4165_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__eta0, var_fn25_calc_iq__eta0_dn2, var_fn25_calc_iq__eta0_dn4, var_fn25_calc_iq__eta0_dn7, var_fn25_calc_iq__eta0_dn16, var_fn25_calc_iq__eta0_dn17,)
    }
};
        var_fn25_calc_iq__eta0 = assign2450_e4165;
        var_fn25_calc_iq__eta0_dn2 = assign2450_e4165_d_n2;
        var_fn25_calc_iq__eta0_dn4 = assign2450_e4165_d_n4;
        var_fn25_calc_iq__eta0_dn7 = assign2450_e4165_d_n7;
        var_fn25_calc_iq__eta0_dn16 = assign2450_e4165_d_n16;
        var_fn25_calc_iq__eta0_dn17 = assign2450_e4165_d_n17;
        var_fn25_calc_iq__eta0_rv = 0.0;

        let (assign2460_e4169, assign2460_e4169_d_n2, assign2460_e4169_d_n4, assign2460_e4169_d_n7, assign2460_e4169_d_n16, assign2460_e4169_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__qinvv0, var_fn25_calc_iq__qinvv0_dn2, var_fn25_calc_iq__qinvv0_dn4, var_fn25_calc_iq__qinvv0_dn7, var_fn25_calc_iq__qinvv0_dn16, var_fn25_calc_iq__qinvv0_dn17,)
    }
};
        var_fn25_calc_iq__qinvv0 = assign2460_e4169;
        var_fn25_calc_iq__qinvv0_dn2 = assign2460_e4169_d_n2;
        var_fn25_calc_iq__qinvv0_dn4 = assign2460_e4169_d_n4;
        var_fn25_calc_iq__qinvv0_dn7 = assign2460_e4169_d_n7;
        var_fn25_calc_iq__qinvv0_dn16 = assign2460_e4169_d_n16;
        var_fn25_calc_iq__qinvv0_dn17 = assign2460_e4169_d_n17;
        var_fn25_calc_iq__qinvv0_rv = 0.0;

        let (assign2470_e4173, assign2470_e4173_d_n2, assign2470_e4173_d_n3, assign2470_e4173_d_n4, assign2470_e4173_d_n7, assign2470_e4173_d_n16, assign2470_e4173_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__vdsats, var_fn25_calc_iq__vdsats_dn2, var_fn25_calc_iq__vdsats_dn3, var_fn25_calc_iq__vdsats_dn4, var_fn25_calc_iq__vdsats_dn7, var_fn25_calc_iq__vdsats_dn16, var_fn25_calc_iq__vdsats_dn17,)
    }
};
        var_fn25_calc_iq__vdsats = assign2470_e4173;
        var_fn25_calc_iq__vdsats_dn2 = assign2470_e4173_d_n2;
        var_fn25_calc_iq__vdsats_dn3 = assign2470_e4173_d_n3;
        var_fn25_calc_iq__vdsats_dn4 = assign2470_e4173_d_n4;
        var_fn25_calc_iq__vdsats_dn7 = assign2470_e4173_d_n7;
        var_fn25_calc_iq__vdsats_dn16 = assign2470_e4173_d_n16;
        var_fn25_calc_iq__vdsats_dn17 = assign2470_e4173_d_n17;
        var_fn25_calc_iq__vdsats_rv = 0.0;

        let (assign2480_e4177, assign2480_e4177_d_n2, assign2480_e4177_d_n3, assign2480_e4177_d_n4, assign2480_e4177_d_n7, assign2480_e4177_d_n16, assign2480_e4177_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__vdsats1, var_fn25_calc_iq__vdsats1_dn2, var_fn25_calc_iq__vdsats1_dn3, var_fn25_calc_iq__vdsats1_dn4, var_fn25_calc_iq__vdsats1_dn7, var_fn25_calc_iq__vdsats1_dn16, var_fn25_calc_iq__vdsats1_dn17,)
    }
};
        var_fn25_calc_iq__vdsats1 = assign2480_e4177;
        var_fn25_calc_iq__vdsats1_dn2 = assign2480_e4177_d_n2;
        var_fn25_calc_iq__vdsats1_dn3 = assign2480_e4177_d_n3;
        var_fn25_calc_iq__vdsats1_dn4 = assign2480_e4177_d_n4;
        var_fn25_calc_iq__vdsats1_dn7 = assign2480_e4177_d_n7;
        var_fn25_calc_iq__vdsats1_dn16 = assign2480_e4177_d_n16;
        var_fn25_calc_iq__vdsats1_dn17 = assign2480_e4177_d_n17;
        var_fn25_calc_iq__vdsats1_rv = 0.0;

        let (assign2490_e4181, assign2490_e4181_d_n2, assign2490_e4181_d_n3, assign2490_e4181_d_n4, assign2490_e4181_d_n7, assign2490_e4181_d_n16, assign2490_e4181_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__vdsat, var_fn25_calc_iq__vdsat_dn2, var_fn25_calc_iq__vdsat_dn3, var_fn25_calc_iq__vdsat_dn4, var_fn25_calc_iq__vdsat_dn7, var_fn25_calc_iq__vdsat_dn16, var_fn25_calc_iq__vdsat_dn17,)
    }
};
        var_fn25_calc_iq__vdsat = assign2490_e4181;
        var_fn25_calc_iq__vdsat_dn2 = assign2490_e4181_d_n2;
        var_fn25_calc_iq__vdsat_dn3 = assign2490_e4181_d_n3;
        var_fn25_calc_iq__vdsat_dn4 = assign2490_e4181_d_n4;
        var_fn25_calc_iq__vdsat_dn7 = assign2490_e4181_d_n7;
        var_fn25_calc_iq__vdsat_dn16 = assign2490_e4181_d_n16;
        var_fn25_calc_iq__vdsat_dn17 = assign2490_e4181_d_n17;
        var_fn25_calc_iq__vdsat_rv = 0.0;

        let (assign2500_e4185, assign2500_e4185_d_n2, assign2500_e4185_d_n3, assign2500_e4185_d_n4, assign2500_e4185_d_n7, assign2500_e4185_d_n16, assign2500_e4185_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__fsd, var_fn25_calc_iq__fsd_dn2, var_fn25_calc_iq__fsd_dn3, var_fn25_calc_iq__fsd_dn4, var_fn25_calc_iq__fsd_dn7, var_fn25_calc_iq__fsd_dn16, var_fn25_calc_iq__fsd_dn17,)
    }
};
        var_fn25_calc_iq__fsd = assign2500_e4185;
        var_fn25_calc_iq__fsd_dn2 = assign2500_e4185_d_n2;
        var_fn25_calc_iq__fsd_dn3 = assign2500_e4185_d_n3;
        var_fn25_calc_iq__fsd_dn4 = assign2500_e4185_d_n4;
        var_fn25_calc_iq__fsd_dn7 = assign2500_e4185_d_n7;
        var_fn25_calc_iq__fsd_dn16 = assign2500_e4185_d_n16;
        var_fn25_calc_iq__fsd_dn17 = assign2500_e4185_d_n17;
        var_fn25_calc_iq__fsd_rv = 0.0;

        let (assign2510_e4189, assign2510_e4189_d_n2, assign2510_e4189_d_n3, assign2510_e4189_d_n4, assign2510_e4189_d_n7, assign2510_e4189_d_n16, assign2510_e4189_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__vdx, var_fn25_calc_iq__vdx_dn2, var_fn25_calc_iq__vdx_dn3, var_fn25_calc_iq__vdx_dn4, var_fn25_calc_iq__vdx_dn7, var_fn25_calc_iq__vdx_dn16, var_fn25_calc_iq__vdx_dn17,)
    }
};
        var_fn25_calc_iq__vdx = assign2510_e4189;
        var_fn25_calc_iq__vdx_dn2 = assign2510_e4189_d_n2;
        var_fn25_calc_iq__vdx_dn3 = assign2510_e4189_d_n3;
        var_fn25_calc_iq__vdx_dn4 = assign2510_e4189_d_n4;
        var_fn25_calc_iq__vdx_dn7 = assign2510_e4189_d_n7;
        var_fn25_calc_iq__vdx_dn16 = assign2510_e4189_d_n16;
        var_fn25_calc_iq__vdx_dn17 = assign2510_e4189_d_n17;
        var_fn25_calc_iq__vdx_rv = 0.0;

        let (assign2520_e4193, assign2520_e4193_d_n2, assign2520_e4193_d_n3, assign2520_e4193_d_n4, assign2520_e4193_d_n7, assign2520_e4193_d_n16, assign2520_e4193_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__fds, var_fn25_calc_iq__fds_dn2, var_fn25_calc_iq__fds_dn3, var_fn25_calc_iq__fds_dn4, var_fn25_calc_iq__fds_dn7, var_fn25_calc_iq__fds_dn16, var_fn25_calc_iq__fds_dn17,)
    }
};
        var_fn25_calc_iq__fds = assign2520_e4193;
        var_fn25_calc_iq__fds_dn2 = assign2520_e4193_d_n2;
        var_fn25_calc_iq__fds_dn3 = assign2520_e4193_d_n3;
        var_fn25_calc_iq__fds_dn4 = assign2520_e4193_d_n4;
        var_fn25_calc_iq__fds_dn7 = assign2520_e4193_d_n7;
        var_fn25_calc_iq__fds_dn16 = assign2520_e4193_d_n16;
        var_fn25_calc_iq__fds_dn17 = assign2520_e4193_d_n17;
        var_fn25_calc_iq__fds_rv = 0.0;

        let (assign2530_e4197, assign2530_e4197_d_n2, assign2530_e4197_d_n3, assign2530_e4197_d_n4, assign2530_e4197_d_n7, assign2530_e4197_d_n16, assign2530_e4197_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__vsx, var_fn25_calc_iq__vsx_dn2, var_fn25_calc_iq__vsx_dn3, var_fn25_calc_iq__vsx_dn4, var_fn25_calc_iq__vsx_dn7, var_fn25_calc_iq__vsx_dn16, var_fn25_calc_iq__vsx_dn17,)
    }
};
        var_fn25_calc_iq__vsx = assign2530_e4197;
        var_fn25_calc_iq__vsx_dn2 = assign2530_e4197_d_n2;
        var_fn25_calc_iq__vsx_dn3 = assign2530_e4197_d_n3;
        var_fn25_calc_iq__vsx_dn4 = assign2530_e4197_d_n4;
        var_fn25_calc_iq__vsx_dn7 = assign2530_e4197_d_n7;
        var_fn25_calc_iq__vsx_dn16 = assign2530_e4197_d_n16;
        var_fn25_calc_iq__vsx_dn17 = assign2530_e4197_d_n17;
        var_fn25_calc_iq__vsx_rv = 0.0;

        let (assign2540_e4201, assign2540_e4201_d_n2, assign2540_e4201_d_n3, assign2540_e4201_d_n4, assign2540_e4201_d_n7, assign2540_e4201_d_n16, assign2540_e4201_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__ffd, var_fn25_calc_iq__ffd_dn2, var_fn25_calc_iq__ffd_dn3, var_fn25_calc_iq__ffd_dn4, var_fn25_calc_iq__ffd_dn7, var_fn25_calc_iq__ffd_dn16, var_fn25_calc_iq__ffd_dn17,)
    }
};
        var_fn25_calc_iq__ffd = assign2540_e4201;
        var_fn25_calc_iq__ffd_dn2 = assign2540_e4201_d_n2;
        var_fn25_calc_iq__ffd_dn3 = assign2540_e4201_d_n3;
        var_fn25_calc_iq__ffd_dn4 = assign2540_e4201_d_n4;
        var_fn25_calc_iq__ffd_dn7 = assign2540_e4201_d_n7;
        var_fn25_calc_iq__ffd_dn16 = assign2540_e4201_d_n16;
        var_fn25_calc_iq__ffd_dn17 = assign2540_e4201_d_n17;
        var_fn25_calc_iq__ffd_rv = 0.0;

        let (assign2550_e4205, assign2550_e4205_d_n2, assign2550_e4205_d_n3, assign2550_e4205_d_n4, assign2550_e4205_d_n7, assign2550_e4205_d_n16, assign2550_e4205_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__etad, var_fn25_calc_iq__etad_dn2, var_fn25_calc_iq__etad_dn3, var_fn25_calc_iq__etad_dn4, var_fn25_calc_iq__etad_dn7, var_fn25_calc_iq__etad_dn16, var_fn25_calc_iq__etad_dn17,)
    }
};
        var_fn25_calc_iq__etad = assign2550_e4205;
        var_fn25_calc_iq__etad_dn2 = assign2550_e4205_d_n2;
        var_fn25_calc_iq__etad_dn3 = assign2550_e4205_d_n3;
        var_fn25_calc_iq__etad_dn4 = assign2550_e4205_d_n4;
        var_fn25_calc_iq__etad_dn7 = assign2550_e4205_d_n7;
        var_fn25_calc_iq__etad_dn16 = assign2550_e4205_d_n16;
        var_fn25_calc_iq__etad_dn17 = assign2550_e4205_d_n17;
        var_fn25_calc_iq__etad_rv = 0.0;

        let (assign2560_e4209, assign2560_e4209_d_n2, assign2560_e4209_d_n3, assign2560_e4209_d_n4, assign2560_e4209_d_n7, assign2560_e4209_d_n16, assign2560_e4209_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__qinvd, var_fn25_calc_iq__qinvd_dn2, var_fn25_calc_iq__qinvd_dn3, var_fn25_calc_iq__qinvd_dn4, var_fn25_calc_iq__qinvd_dn7, var_fn25_calc_iq__qinvd_dn16, var_fn25_calc_iq__qinvd_dn17,)
    }
};
        var_fn25_calc_iq__qinvd = assign2560_e4209;
        var_fn25_calc_iq__qinvd_dn2 = assign2560_e4209_d_n2;
        var_fn25_calc_iq__qinvd_dn3 = assign2560_e4209_d_n3;
        var_fn25_calc_iq__qinvd_dn4 = assign2560_e4209_d_n4;
        var_fn25_calc_iq__qinvd_dn7 = assign2560_e4209_d_n7;
        var_fn25_calc_iq__qinvd_dn16 = assign2560_e4209_d_n16;
        var_fn25_calc_iq__qinvd_dn17 = assign2560_e4209_d_n17;
        var_fn25_calc_iq__qinvd_rv = 0.0;

        let (assign2570_e4213, assign2570_e4213_d_n2, assign2570_e4213_d_n3, assign2570_e4213_d_n4, assign2570_e4213_d_n7, assign2570_e4213_d_n16, assign2570_e4213_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__vdsc, var_fn25_calc_iq__vdsc_dn2, var_fn25_calc_iq__vdsc_dn3, var_fn25_calc_iq__vdsc_dn4, var_fn25_calc_iq__vdsc_dn7, var_fn25_calc_iq__vdsc_dn16, var_fn25_calc_iq__vdsc_dn17,)
    }
};
        var_fn25_calc_iq__vdsc = assign2570_e4213;
        var_fn25_calc_iq__vdsc_dn2 = assign2570_e4213_d_n2;
        var_fn25_calc_iq__vdsc_dn3 = assign2570_e4213_d_n3;
        var_fn25_calc_iq__vdsc_dn4 = assign2570_e4213_d_n4;
        var_fn25_calc_iq__vdsc_dn7 = assign2570_e4213_d_n7;
        var_fn25_calc_iq__vdsc_dn16 = assign2570_e4213_d_n16;
        var_fn25_calc_iq__vdsc_dn17 = assign2570_e4213_d_n17;
        var_fn25_calc_iq__vdsc_rv = 0.0;

        let (assign2600_e4225, assign2600_e4225_d_n4,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__vdsats0, var_fn25_calc_iq__vdsats0_dn4,)
    }
};
        var_fn25_calc_iq__vdsats0 = assign2600_e4225;
        var_fn25_calc_iq__vdsats0_dn4 = assign2600_e4225_d_n4;
        var_fn25_calc_iq__vdsats0_rv = 0.0;

        let (assign2610_e4229, assign2610_e4229_d_n2, assign2610_e4229_d_n4, assign2610_e4229_d_n7, assign2610_e4229_d_n16, assign2610_e4229_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__vdsats10, var_fn25_calc_iq__vdsats10_dn2, var_fn25_calc_iq__vdsats10_dn4, var_fn25_calc_iq__vdsats10_dn7, var_fn25_calc_iq__vdsats10_dn16, var_fn25_calc_iq__vdsats10_dn17,)
    }
};
        var_fn25_calc_iq__vdsats10 = assign2610_e4229;
        var_fn25_calc_iq__vdsats10_dn2 = assign2610_e4229_d_n2;
        var_fn25_calc_iq__vdsats10_dn4 = assign2610_e4229_d_n4;
        var_fn25_calc_iq__vdsats10_dn7 = assign2610_e4229_d_n7;
        var_fn25_calc_iq__vdsats10_dn16 = assign2610_e4229_d_n16;
        var_fn25_calc_iq__vdsats10_dn17 = assign2610_e4229_d_n17;
        var_fn25_calc_iq__vdsats10_rv = 0.0;

        let (assign2620_e4233, assign2620_e4233_d_n2, assign2620_e4233_d_n4, assign2620_e4233_d_n7, assign2620_e4233_d_n16, assign2620_e4233_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__vdsat10, var_fn25_calc_iq__vdsat10_dn2, var_fn25_calc_iq__vdsat10_dn4, var_fn25_calc_iq__vdsat10_dn7, var_fn25_calc_iq__vdsat10_dn16, var_fn25_calc_iq__vdsat10_dn17,)
    }
};
        var_fn25_calc_iq__vdsat10 = assign2620_e4233;
        var_fn25_calc_iq__vdsat10_dn2 = assign2620_e4233_d_n2;
        var_fn25_calc_iq__vdsat10_dn4 = assign2620_e4233_d_n4;
        var_fn25_calc_iq__vdsat10_dn7 = assign2620_e4233_d_n7;
        var_fn25_calc_iq__vdsat10_dn16 = assign2620_e4233_d_n16;
        var_fn25_calc_iq__vdsat10_dn17 = assign2620_e4233_d_n17;
        var_fn25_calc_iq__vdsat10_rv = 0.0;

        *var_fn25_calc_iq__eta_slot = var_fn25_calc_iq__eta;
        *var_fn25_calc_iq__eta0_slot = var_fn25_calc_iq__eta0;
        *var_fn25_calc_iq__eta0_dn16_slot = var_fn25_calc_iq__eta0_dn16;
        *var_fn25_calc_iq__eta0_dn17_slot = var_fn25_calc_iq__eta0_dn17;
        *var_fn25_calc_iq__eta0_dn2_slot = var_fn25_calc_iq__eta0_dn2;
        *var_fn25_calc_iq__eta0_dn4_slot = var_fn25_calc_iq__eta0_dn4;
        *var_fn25_calc_iq__eta0_dn7_slot = var_fn25_calc_iq__eta0_dn7;
        *var_fn25_calc_iq__eta0_rv_slot = var_fn25_calc_iq__eta0_rv;
        *var_fn25_calc_iq__eta_dn16_slot = var_fn25_calc_iq__eta_dn16;
        *var_fn25_calc_iq__eta_dn17_slot = var_fn25_calc_iq__eta_dn17;
        *var_fn25_calc_iq__eta_dn2_slot = var_fn25_calc_iq__eta_dn2;
        *var_fn25_calc_iq__eta_dn3_slot = var_fn25_calc_iq__eta_dn3;
        *var_fn25_calc_iq__eta_dn4_slot = var_fn25_calc_iq__eta_dn4;
        *var_fn25_calc_iq__eta_dn7_slot = var_fn25_calc_iq__eta_dn7;
        *var_fn25_calc_iq__eta_rv_slot = var_fn25_calc_iq__eta_rv;
        *var_fn25_calc_iq__etad_slot = var_fn25_calc_iq__etad;
        *var_fn25_calc_iq__etad_dn16_slot = var_fn25_calc_iq__etad_dn16;
        *var_fn25_calc_iq__etad_dn17_slot = var_fn25_calc_iq__etad_dn17;
        *var_fn25_calc_iq__etad_dn2_slot = var_fn25_calc_iq__etad_dn2;
        *var_fn25_calc_iq__etad_dn3_slot = var_fn25_calc_iq__etad_dn3;
        *var_fn25_calc_iq__etad_dn4_slot = var_fn25_calc_iq__etad_dn4;
        *var_fn25_calc_iq__etad_dn7_slot = var_fn25_calc_iq__etad_dn7;
        *var_fn25_calc_iq__etad_rv_slot = var_fn25_calc_iq__etad_rv;
        *var_fn25_calc_iq__etas_slot = var_fn25_calc_iq__etas;
        *var_fn25_calc_iq__etas0_slot = var_fn25_calc_iq__etas0;
        *var_fn25_calc_iq__etas0_dn16_slot = var_fn25_calc_iq__etas0_dn16;
        *var_fn25_calc_iq__etas0_dn17_slot = var_fn25_calc_iq__etas0_dn17;
        *var_fn25_calc_iq__etas0_dn2_slot = var_fn25_calc_iq__etas0_dn2;
        *var_fn25_calc_iq__etas0_dn4_slot = var_fn25_calc_iq__etas0_dn4;
        *var_fn25_calc_iq__etas0_dn7_slot = var_fn25_calc_iq__etas0_dn7;
        *var_fn25_calc_iq__etas0_rv_slot = var_fn25_calc_iq__etas0_rv;
        *var_fn25_calc_iq__etas_dn16_slot = var_fn25_calc_iq__etas_dn16;
        *var_fn25_calc_iq__etas_dn17_slot = var_fn25_calc_iq__etas_dn17;
        *var_fn25_calc_iq__etas_dn2_slot = var_fn25_calc_iq__etas_dn2;
        *var_fn25_calc_iq__etas_dn3_slot = var_fn25_calc_iq__etas_dn3;
        *var_fn25_calc_iq__etas_dn4_slot = var_fn25_calc_iq__etas_dn4;
        *var_fn25_calc_iq__etas_dn7_slot = var_fn25_calc_iq__etas_dn7;
        *var_fn25_calc_iq__etas_rv_slot = var_fn25_calc_iq__etas_rv;
        *var_fn25_calc_iq__fds_slot = var_fn25_calc_iq__fds;
        *var_fn25_calc_iq__fds_dn16_slot = var_fn25_calc_iq__fds_dn16;
        *var_fn25_calc_iq__fds_dn17_slot = var_fn25_calc_iq__fds_dn17;
        *var_fn25_calc_iq__fds_dn2_slot = var_fn25_calc_iq__fds_dn2;
        *var_fn25_calc_iq__fds_dn3_slot = var_fn25_calc_iq__fds_dn3;
        *var_fn25_calc_iq__fds_dn4_slot = var_fn25_calc_iq__fds_dn4;
        *var_fn25_calc_iq__fds_dn7_slot = var_fn25_calc_iq__fds_dn7;
        *var_fn25_calc_iq__fds_rv_slot = var_fn25_calc_iq__fds_rv;
        *var_fn25_calc_iq__ff_slot = var_fn25_calc_iq__ff;
        *var_fn25_calc_iq__ff0_slot = var_fn25_calc_iq__ff0;
        *var_fn25_calc_iq__ff0_dn16_slot = var_fn25_calc_iq__ff0_dn16;
        *var_fn25_calc_iq__ff0_dn17_slot = var_fn25_calc_iq__ff0_dn17;
        *var_fn25_calc_iq__ff0_dn2_slot = var_fn25_calc_iq__ff0_dn2;
        *var_fn25_calc_iq__ff0_dn4_slot = var_fn25_calc_iq__ff0_dn4;
        *var_fn25_calc_iq__ff0_dn7_slot = var_fn25_calc_iq__ff0_dn7;
        *var_fn25_calc_iq__ff0_rv_slot = var_fn25_calc_iq__ff0_rv;
        *var_fn25_calc_iq__ff_dn16_slot = var_fn25_calc_iq__ff_dn16;
        *var_fn25_calc_iq__ff_dn17_slot = var_fn25_calc_iq__ff_dn17;
        *var_fn25_calc_iq__ff_dn2_slot = var_fn25_calc_iq__ff_dn2;
        *var_fn25_calc_iq__ff_dn3_slot = var_fn25_calc_iq__ff_dn3;
        *var_fn25_calc_iq__ff_dn4_slot = var_fn25_calc_iq__ff_dn4;
        *var_fn25_calc_iq__ff_dn7_slot = var_fn25_calc_iq__ff_dn7;
        *var_fn25_calc_iq__ff_rv_slot = var_fn25_calc_iq__ff_rv;
        *var_fn25_calc_iq__ffd_slot = var_fn25_calc_iq__ffd;
        *var_fn25_calc_iq__ffd_dn16_slot = var_fn25_calc_iq__ffd_dn16;
        *var_fn25_calc_iq__ffd_dn17_slot = var_fn25_calc_iq__ffd_dn17;
        *var_fn25_calc_iq__ffd_dn2_slot = var_fn25_calc_iq__ffd_dn2;
        *var_fn25_calc_iq__ffd_dn3_slot = var_fn25_calc_iq__ffd_dn3;
        *var_fn25_calc_iq__ffd_dn4_slot = var_fn25_calc_iq__ffd_dn4;
        *var_fn25_calc_iq__ffd_dn7_slot = var_fn25_calc_iq__ffd_dn7;
        *var_fn25_calc_iq__ffd_rv_slot = var_fn25_calc_iq__ffd_rv;
        *var_fn25_calc_iq__ffs0_slot = var_fn25_calc_iq__ffs0;
        *var_fn25_calc_iq__ffs0_dn16_slot = var_fn25_calc_iq__ffs0_dn16;
        *var_fn25_calc_iq__ffs0_dn17_slot = var_fn25_calc_iq__ffs0_dn17;
        *var_fn25_calc_iq__ffs0_dn2_slot = var_fn25_calc_iq__ffs0_dn2;
        *var_fn25_calc_iq__ffs0_dn4_slot = var_fn25_calc_iq__ffs0_dn4;
        *var_fn25_calc_iq__ffs0_dn7_slot = var_fn25_calc_iq__ffs0_dn7;
        *var_fn25_calc_iq__ffs0_rv_slot = var_fn25_calc_iq__ffs0_rv;
        *var_fn25_calc_iq__fsd_slot = var_fn25_calc_iq__fsd;
        *var_fn25_calc_iq__fsd_dn16_slot = var_fn25_calc_iq__fsd_dn16;
        *var_fn25_calc_iq__fsd_dn17_slot = var_fn25_calc_iq__fsd_dn17;
        *var_fn25_calc_iq__fsd_dn2_slot = var_fn25_calc_iq__fsd_dn2;
        *var_fn25_calc_iq__fsd_dn3_slot = var_fn25_calc_iq__fsd_dn3;
        *var_fn25_calc_iq__fsd_dn4_slot = var_fn25_calc_iq__fsd_dn4;
        *var_fn25_calc_iq__fsd_dn7_slot = var_fn25_calc_iq__fsd_dn7;
        *var_fn25_calc_iq__fsd_rv_slot = var_fn25_calc_iq__fsd_rv;
        *var_fn25_calc_iq__muf_slot = var_fn25_calc_iq__muf;
        *var_fn25_calc_iq__muf0_slot = var_fn25_calc_iq__muf0;
        *var_fn25_calc_iq__muf0_dn4_slot = var_fn25_calc_iq__muf0_dn4;
        *var_fn25_calc_iq__muf0_rv_slot = var_fn25_calc_iq__muf0_rv;
        *var_fn25_calc_iq__muf_dn16_slot = var_fn25_calc_iq__muf_dn16;
        *var_fn25_calc_iq__muf_dn17_slot = var_fn25_calc_iq__muf_dn17;
        *var_fn25_calc_iq__muf_dn2_slot = var_fn25_calc_iq__muf_dn2;
        *var_fn25_calc_iq__muf_dn3_slot = var_fn25_calc_iq__muf_dn3;
        *var_fn25_calc_iq__muf_dn4_slot = var_fn25_calc_iq__muf_dn4;
        *var_fn25_calc_iq__muf_dn7_slot = var_fn25_calc_iq__muf_dn7;
        *var_fn25_calc_iq__muf_rv_slot = var_fn25_calc_iq__muf_rv;
        *var_fn25_calc_iq__n0_slot = var_fn25_calc_iq__n0;
        *var_fn25_calc_iq__n0_dn4_slot = var_fn25_calc_iq__n0_dn4;
        *var_fn25_calc_iq__n0_rv_slot = var_fn25_calc_iq__n0_rv;
        *var_fn25_calc_iq__qinvd_slot = var_fn25_calc_iq__qinvd;
        *var_fn25_calc_iq__qinvd_dn16_slot = var_fn25_calc_iq__qinvd_dn16;
        *var_fn25_calc_iq__qinvd_dn17_slot = var_fn25_calc_iq__qinvd_dn17;
        *var_fn25_calc_iq__qinvd_dn2_slot = var_fn25_calc_iq__qinvd_dn2;
        *var_fn25_calc_iq__qinvd_dn3_slot = var_fn25_calc_iq__qinvd_dn3;
        *var_fn25_calc_iq__qinvd_dn4_slot = var_fn25_calc_iq__qinvd_dn4;
        *var_fn25_calc_iq__qinvd_dn7_slot = var_fn25_calc_iq__qinvd_dn7;
        *var_fn25_calc_iq__qinvd_rv_slot = var_fn25_calc_iq__qinvd_rv;
        *var_fn25_calc_iq__qinvs_slot = var_fn25_calc_iq__qinvs;
        *var_fn25_calc_iq__qinvs0_slot = var_fn25_calc_iq__qinvs0;
        *var_fn25_calc_iq__qinvs0_dn16_slot = var_fn25_calc_iq__qinvs0_dn16;
        *var_fn25_calc_iq__qinvs0_dn17_slot = var_fn25_calc_iq__qinvs0_dn17;
        *var_fn25_calc_iq__qinvs0_dn2_slot = var_fn25_calc_iq__qinvs0_dn2;
        *var_fn25_calc_iq__qinvs0_dn4_slot = var_fn25_calc_iq__qinvs0_dn4;
        *var_fn25_calc_iq__qinvs0_dn7_slot = var_fn25_calc_iq__qinvs0_dn7;
        *var_fn25_calc_iq__qinvs0_rv_slot = var_fn25_calc_iq__qinvs0_rv;
        *var_fn25_calc_iq__qinvs_dn16_slot = var_fn25_calc_iq__qinvs_dn16;
        *var_fn25_calc_iq__qinvs_dn17_slot = var_fn25_calc_iq__qinvs_dn17;
        *var_fn25_calc_iq__qinvs_dn2_slot = var_fn25_calc_iq__qinvs_dn2;
        *var_fn25_calc_iq__qinvs_dn3_slot = var_fn25_calc_iq__qinvs_dn3;
        *var_fn25_calc_iq__qinvs_dn4_slot = var_fn25_calc_iq__qinvs_dn4;
        *var_fn25_calc_iq__qinvs_dn7_slot = var_fn25_calc_iq__qinvs_dn7;
        *var_fn25_calc_iq__qinvs_rv_slot = var_fn25_calc_iq__qinvs_rv;
        *var_fn25_calc_iq__qinvv_slot = var_fn25_calc_iq__qinvv;
        *var_fn25_calc_iq__qinvv0_slot = var_fn25_calc_iq__qinvv0;
        *var_fn25_calc_iq__qinvv0_dn16_slot = var_fn25_calc_iq__qinvv0_dn16;
        *var_fn25_calc_iq__qinvv0_dn17_slot = var_fn25_calc_iq__qinvv0_dn17;
        *var_fn25_calc_iq__qinvv0_dn2_slot = var_fn25_calc_iq__qinvv0_dn2;
        *var_fn25_calc_iq__qinvv0_dn4_slot = var_fn25_calc_iq__qinvv0_dn4;
        *var_fn25_calc_iq__qinvv0_dn7_slot = var_fn25_calc_iq__qinvv0_dn7;
        *var_fn25_calc_iq__qinvv0_rv_slot = var_fn25_calc_iq__qinvv0_rv;
        *var_fn25_calc_iq__qinvv_dn16_slot = var_fn25_calc_iq__qinvv_dn16;
        *var_fn25_calc_iq__qinvv_dn17_slot = var_fn25_calc_iq__qinvv_dn17;
        *var_fn25_calc_iq__qinvv_dn2_slot = var_fn25_calc_iq__qinvv_dn2;
        *var_fn25_calc_iq__qinvv_dn3_slot = var_fn25_calc_iq__qinvv_dn3;
        *var_fn25_calc_iq__qinvv_dn4_slot = var_fn25_calc_iq__qinvv_dn4;
        *var_fn25_calc_iq__qinvv_dn7_slot = var_fn25_calc_iq__qinvv_dn7;
        *var_fn25_calc_iq__qinvv_rv_slot = var_fn25_calc_iq__qinvv_rv;
        *var_fn25_calc_iq__qref_slot = var_fn25_calc_iq__qref;
        *var_fn25_calc_iq__qref0_slot = var_fn25_calc_iq__qref0;
        *var_fn25_calc_iq__qref0_dn4_slot = var_fn25_calc_iq__qref0_dn4;
        *var_fn25_calc_iq__qref0_rv_slot = var_fn25_calc_iq__qref0_rv;
        *var_fn25_calc_iq__qref_dn16_slot = var_fn25_calc_iq__qref_dn16;
        *var_fn25_calc_iq__qref_dn17_slot = var_fn25_calc_iq__qref_dn17;
        *var_fn25_calc_iq__qref_dn4_slot = var_fn25_calc_iq__qref_dn4;
        *var_fn25_calc_iq__qref_rv_slot = var_fn25_calc_iq__qref_rv;
        *var_fn25_calc_iq__tfacmobin_slot = var_fn25_calc_iq__tfacmobin;
        *var_fn25_calc_iq__tfacmobin_dn4_slot = var_fn25_calc_iq__tfacmobin_dn4;
        *var_fn25_calc_iq__tfacmobin_rv_slot = var_fn25_calc_iq__tfacmobin_rv;
        *var_fn25_calc_iq__two_n_phit_slot = var_fn25_calc_iq__two_n_phit;
        *var_fn25_calc_iq__two_n_phit0_slot = var_fn25_calc_iq__two_n_phit0;
        *var_fn25_calc_iq__two_n_phit0_dn4_slot = var_fn25_calc_iq__two_n_phit0_dn4;
        *var_fn25_calc_iq__two_n_phit0_rv_slot = var_fn25_calc_iq__two_n_phit0_rv;
        *var_fn25_calc_iq__two_n_phit_dn16_slot = var_fn25_calc_iq__two_n_phit_dn16;
        *var_fn25_calc_iq__two_n_phit_dn17_slot = var_fn25_calc_iq__two_n_phit_dn17;
        *var_fn25_calc_iq__two_n_phit_dn4_slot = var_fn25_calc_iq__two_n_phit_dn4;
        *var_fn25_calc_iq__two_n_phit_rv_slot = var_fn25_calc_iq__two_n_phit_rv;
        *var_fn25_calc_iq__vdsat_slot = var_fn25_calc_iq__vdsat;
        *var_fn25_calc_iq__vdsat10_slot = var_fn25_calc_iq__vdsat10;
        *var_fn25_calc_iq__vdsat10_dn16_slot = var_fn25_calc_iq__vdsat10_dn16;
        *var_fn25_calc_iq__vdsat10_dn17_slot = var_fn25_calc_iq__vdsat10_dn17;
        *var_fn25_calc_iq__vdsat10_dn2_slot = var_fn25_calc_iq__vdsat10_dn2;
        *var_fn25_calc_iq__vdsat10_dn4_slot = var_fn25_calc_iq__vdsat10_dn4;
        *var_fn25_calc_iq__vdsat10_dn7_slot = var_fn25_calc_iq__vdsat10_dn7;
        *var_fn25_calc_iq__vdsat10_rv_slot = var_fn25_calc_iq__vdsat10_rv;
        *var_fn25_calc_iq__vdsat_dn16_slot = var_fn25_calc_iq__vdsat_dn16;
        *var_fn25_calc_iq__vdsat_dn17_slot = var_fn25_calc_iq__vdsat_dn17;
        *var_fn25_calc_iq__vdsat_dn2_slot = var_fn25_calc_iq__vdsat_dn2;
        *var_fn25_calc_iq__vdsat_dn3_slot = var_fn25_calc_iq__vdsat_dn3;
        *var_fn25_calc_iq__vdsat_dn4_slot = var_fn25_calc_iq__vdsat_dn4;
        *var_fn25_calc_iq__vdsat_dn7_slot = var_fn25_calc_iq__vdsat_dn7;
        *var_fn25_calc_iq__vdsat_rv_slot = var_fn25_calc_iq__vdsat_rv;
        *var_fn25_calc_iq__vdsats_slot = var_fn25_calc_iq__vdsats;
        *var_fn25_calc_iq__vdsats0_slot = var_fn25_calc_iq__vdsats0;
        *var_fn25_calc_iq__vdsats0_dn4_slot = var_fn25_calc_iq__vdsats0_dn4;
        *var_fn25_calc_iq__vdsats0_rv_slot = var_fn25_calc_iq__vdsats0_rv;
        *var_fn25_calc_iq__vdsats1_slot = var_fn25_calc_iq__vdsats1;
        *var_fn25_calc_iq__vdsats10_slot = var_fn25_calc_iq__vdsats10;
        *var_fn25_calc_iq__vdsats10_dn16_slot = var_fn25_calc_iq__vdsats10_dn16;
        *var_fn25_calc_iq__vdsats10_dn17_slot = var_fn25_calc_iq__vdsats10_dn17;
        *var_fn25_calc_iq__vdsats10_dn2_slot = var_fn25_calc_iq__vdsats10_dn2;
        *var_fn25_calc_iq__vdsats10_dn4_slot = var_fn25_calc_iq__vdsats10_dn4;
        *var_fn25_calc_iq__vdsats10_dn7_slot = var_fn25_calc_iq__vdsats10_dn7;
        *var_fn25_calc_iq__vdsats10_rv_slot = var_fn25_calc_iq__vdsats10_rv;
        *var_fn25_calc_iq__vdsats1_dn16_slot = var_fn25_calc_iq__vdsats1_dn16;
        *var_fn25_calc_iq__vdsats1_dn17_slot = var_fn25_calc_iq__vdsats1_dn17;
        *var_fn25_calc_iq__vdsats1_dn2_slot = var_fn25_calc_iq__vdsats1_dn2;
        *var_fn25_calc_iq__vdsats1_dn3_slot = var_fn25_calc_iq__vdsats1_dn3;
        *var_fn25_calc_iq__vdsats1_dn4_slot = var_fn25_calc_iq__vdsats1_dn4;
        *var_fn25_calc_iq__vdsats1_dn7_slot = var_fn25_calc_iq__vdsats1_dn7;
        *var_fn25_calc_iq__vdsats1_rv_slot = var_fn25_calc_iq__vdsats1_rv;
        *var_fn25_calc_iq__vdsats_dn16_slot = var_fn25_calc_iq__vdsats_dn16;
        *var_fn25_calc_iq__vdsats_dn17_slot = var_fn25_calc_iq__vdsats_dn17;
        *var_fn25_calc_iq__vdsats_dn2_slot = var_fn25_calc_iq__vdsats_dn2;
        *var_fn25_calc_iq__vdsats_dn3_slot = var_fn25_calc_iq__vdsats_dn3;
        *var_fn25_calc_iq__vdsats_dn4_slot = var_fn25_calc_iq__vdsats_dn4;
        *var_fn25_calc_iq__vdsats_dn7_slot = var_fn25_calc_iq__vdsats_dn7;
        *var_fn25_calc_iq__vdsats_rv_slot = var_fn25_calc_iq__vdsats_rv;
        *var_fn25_calc_iq__vdsc_slot = var_fn25_calc_iq__vdsc;
        *var_fn25_calc_iq__vdsc_dn16_slot = var_fn25_calc_iq__vdsc_dn16;
        *var_fn25_calc_iq__vdsc_dn17_slot = var_fn25_calc_iq__vdsc_dn17;
        *var_fn25_calc_iq__vdsc_dn2_slot = var_fn25_calc_iq__vdsc_dn2;
        *var_fn25_calc_iq__vdsc_dn3_slot = var_fn25_calc_iq__vdsc_dn3;
        *var_fn25_calc_iq__vdsc_dn4_slot = var_fn25_calc_iq__vdsc_dn4;
        *var_fn25_calc_iq__vdsc_dn7_slot = var_fn25_calc_iq__vdsc_dn7;
        *var_fn25_calc_iq__vdsc_rv_slot = var_fn25_calc_iq__vdsc_rv;
        *var_fn25_calc_iq__vdx_slot = var_fn25_calc_iq__vdx;
        *var_fn25_calc_iq__vdx_dn16_slot = var_fn25_calc_iq__vdx_dn16;
        *var_fn25_calc_iq__vdx_dn17_slot = var_fn25_calc_iq__vdx_dn17;
        *var_fn25_calc_iq__vdx_dn2_slot = var_fn25_calc_iq__vdx_dn2;
        *var_fn25_calc_iq__vdx_dn3_slot = var_fn25_calc_iq__vdx_dn3;
        *var_fn25_calc_iq__vdx_dn4_slot = var_fn25_calc_iq__vdx_dn4;
        *var_fn25_calc_iq__vdx_dn7_slot = var_fn25_calc_iq__vdx_dn7;
        *var_fn25_calc_iq__vdx_rv_slot = var_fn25_calc_iq__vdx_rv;
        *var_fn25_calc_iq__vsx_slot = var_fn25_calc_iq__vsx;
        *var_fn25_calc_iq__vsx_dn16_slot = var_fn25_calc_iq__vsx_dn16;
        *var_fn25_calc_iq__vsx_dn17_slot = var_fn25_calc_iq__vsx_dn17;
        *var_fn25_calc_iq__vsx_dn2_slot = var_fn25_calc_iq__vsx_dn2;
        *var_fn25_calc_iq__vsx_dn3_slot = var_fn25_calc_iq__vsx_dn3;
        *var_fn25_calc_iq__vsx_dn4_slot = var_fn25_calc_iq__vsx_dn4;
        *var_fn25_calc_iq__vsx_dn7_slot = var_fn25_calc_iq__vsx_dn7;
        *var_fn25_calc_iq__vsx_rv_slot = var_fn25_calc_iq__vsx_rv;
        *var_fn25_calc_iq__vx_slot = var_fn25_calc_iq__vx;
        *var_fn25_calc_iq__vx0_slot = var_fn25_calc_iq__vx0;
        *var_fn25_calc_iq__vx0_dn4_slot = var_fn25_calc_iq__vx0_dn4;
        *var_fn25_calc_iq__vx0_rv_slot = var_fn25_calc_iq__vx0_rv;
        *var_fn25_calc_iq__vx_dn16_slot = var_fn25_calc_iq__vx_dn16;
        *var_fn25_calc_iq__vx_dn17_slot = var_fn25_calc_iq__vx_dn17;
        *var_fn25_calc_iq__vx_dn2_slot = var_fn25_calc_iq__vx_dn2;
        *var_fn25_calc_iq__vx_dn3_slot = var_fn25_calc_iq__vx_dn3;
        *var_fn25_calc_iq__vx_dn4_slot = var_fn25_calc_iq__vx_dn4;
        *var_fn25_calc_iq__vx_dn7_slot = var_fn25_calc_iq__vx_dn7;
        *var_fn25_calc_iq__vx_rv_slot = var_fn25_calc_iq__vx_rv;
    }

    pub(super) fn stamp_reactive_block_5(
        p: &Parameters,
        var_fn25_calc_iq__alpha: f64,
        var_fn25_calc_iq__beta: f64,
        var_fn25_calc_iq__dibsat: f64,
        var_fn25_calc_iq__epsilon: f64,
        var_fn25_calc_iq__nd: f64,
        var_fn25_calc_iq__phitin: f64,
        var_fn25_calc_iq__phitin_dn4: f64,
        var_fn25_calc_iq__ss: f64,
        var_fn25_calc_iq__tambin: f64,
        var_fn25_calc_iq__tambin_dn4: f64,
        var_fn25_calc_iq__tnomin: f64,
        var_fn25_calc_iq__vdsin: f64,
        var_fn25_calc_iq__vdsin_dn16: f64,
        var_fn25_calc_iq__vdsin_dn17: f64,
        var_fn25_calc_iq__vgsin: f64,
        var_fn25_calc_iq__vgsin_dn16: f64,
        var_fn25_calc_iq__vgsin_dn2: f64,
        var_fn25_calc_iq__vgsin_dn7: f64,
        var_fn25_calc_iq__vto: f64,
        var_fn25_calc_iq__vtzeta: f64,
        var_guard24: f64,
        var_fn25_calc_iq__absvdsin_slot: &mut f64,
        var_fn25_calc_iq__absvdsin_dn16_slot: &mut f64,
        var_fn25_calc_iq__absvdsin_dn17_slot: &mut f64,
        var_fn25_calc_iq__absvdsin_rv_slot: &mut f64,
        var_fn25_calc_iq__alpha_phit_slot: &mut f64,
        var_fn25_calc_iq__alpha_phit_dn4_slot: &mut f64,
        var_fn25_calc_iq__alpha_phit_rv_slot: &mut f64,
        var_fn25_calc_iq__etab_slot: &mut f64,
        var_fn25_calc_iq__etab_dn16_slot: &mut f64,
        var_fn25_calc_iq__etab_dn3_slot: &mut f64,
        var_fn25_calc_iq__etab_dn4_slot: &mut f64,
        var_fn25_calc_iq__etab_rv_slot: &mut f64,
        var_fn25_calc_iq__etac_slot: &mut f64,
        var_fn25_calc_iq__etac_dn16_slot: &mut f64,
        var_fn25_calc_iq__etac_dn2_slot: &mut f64,
        var_fn25_calc_iq__etac_dn4_slot: &mut f64,
        var_fn25_calc_iq__etac_dn7_slot: &mut f64,
        var_fn25_calc_iq__etac_rv_slot: &mut f64,
        var_fn25_calc_iq__etad0_slot: &mut f64,
        var_fn25_calc_iq__etad0_dn16_slot: &mut f64,
        var_fn25_calc_iq__etad0_dn17_slot: &mut f64,
        var_fn25_calc_iq__etad0_dn2_slot: &mut f64,
        var_fn25_calc_iq__etad0_dn4_slot: &mut f64,
        var_fn25_calc_iq__etad0_dn7_slot: &mut f64,
        var_fn25_calc_iq__etad0_rv_slot: &mut f64,
        var_fn25_calc_iq__etags_slot: &mut f64,
        var_fn25_calc_iq__etags_dn16_slot: &mut f64,
        var_fn25_calc_iq__etags_dn2_slot: &mut f64,
        var_fn25_calc_iq__etags_dn4_slot: &mut f64,
        var_fn25_calc_iq__etags_dn7_slot: &mut f64,
        var_fn25_calc_iq__etags_rv_slot: &mut f64,
        var_fn25_calc_iq__exparg_slot: &mut f64,
        var_fn25_calc_iq__exparg0_slot: &mut f64,
        var_fn25_calc_iq__exparg0_dn16_slot: &mut f64,
        var_fn25_calc_iq__exparg0_dn17_slot: &mut f64,
        var_fn25_calc_iq__exparg0_dn2_slot: &mut f64,
        var_fn25_calc_iq__exparg0_dn4_slot: &mut f64,
        var_fn25_calc_iq__exparg0_dn7_slot: &mut f64,
        var_fn25_calc_iq__exparg0_rv_slot: &mut f64,
        var_fn25_calc_iq__exparg_dn16_slot: &mut f64,
        var_fn25_calc_iq__exparg_dn17_slot: &mut f64,
        var_fn25_calc_iq__exparg_dn2_slot: &mut f64,
        var_fn25_calc_iq__exparg_dn3_slot: &mut f64,
        var_fn25_calc_iq__exparg_dn4_slot: &mut f64,
        var_fn25_calc_iq__exparg_dn7_slot: &mut f64,
        var_fn25_calc_iq__exparg_rv_slot: &mut f64,
        var_fn25_calc_iq__fds0_slot: &mut f64,
        var_fn25_calc_iq__fds0_dn16_slot: &mut f64,
        var_fn25_calc_iq__fds0_dn17_slot: &mut f64,
        var_fn25_calc_iq__fds0_dn2_slot: &mut f64,
        var_fn25_calc_iq__fds0_dn4_slot: &mut f64,
        var_fn25_calc_iq__fds0_dn7_slot: &mut f64,
        var_fn25_calc_iq__fds0_rv_slot: &mut f64,
        var_fn25_calc_iq__ffd0_slot: &mut f64,
        var_fn25_calc_iq__ffd0_dn16_slot: &mut f64,
        var_fn25_calc_iq__ffd0_dn17_slot: &mut f64,
        var_fn25_calc_iq__ffd0_dn2_slot: &mut f64,
        var_fn25_calc_iq__ffd0_dn4_slot: &mut f64,
        var_fn25_calc_iq__ffd0_dn7_slot: &mut f64,
        var_fn25_calc_iq__ffd0_rv_slot: &mut f64,
        var_fn25_calc_iq__fsd0_slot: &mut f64,
        var_fn25_calc_iq__fsd0_dn16_slot: &mut f64,
        var_fn25_calc_iq__fsd0_dn17_slot: &mut f64,
        var_fn25_calc_iq__fsd0_dn2_slot: &mut f64,
        var_fn25_calc_iq__fsd0_dn4_slot: &mut f64,
        var_fn25_calc_iq__fsd0_dn7_slot: &mut f64,
        var_fn25_calc_iq__fsd0_rv_slot: &mut f64,
        var_fn25_calc_iq__myarg_slot: &mut f64,
        var_fn25_calc_iq__myarg0_slot: &mut f64,
        var_fn25_calc_iq__myarg0_dn4_slot: &mut f64,
        var_fn25_calc_iq__myarg0_rv_slot: &mut f64,
        var_fn25_calc_iq__myarg_dn16_slot: &mut f64,
        var_fn25_calc_iq__myarg_dn17_slot: &mut f64,
        var_fn25_calc_iq__myarg_dn2_slot: &mut f64,
        var_fn25_calc_iq__myarg_dn3_slot: &mut f64,
        var_fn25_calc_iq__myarg_dn4_slot: &mut f64,
        var_fn25_calc_iq__myarg_dn7_slot: &mut f64,
        var_fn25_calc_iq__myarg_rv_slot: &mut f64,
        var_fn25_calc_iq__n_slot: &mut f64,
        var_fn25_calc_iq__n_dn16_slot: &mut f64,
        var_fn25_calc_iq__n_dn17_slot: &mut f64,
        var_fn25_calc_iq__n_dn4_slot: &mut f64,
        var_fn25_calc_iq__n_rv_slot: &mut f64,
        var_fn25_calc_iq__qd_slot: &mut f64,
        var_fn25_calc_iq__qd1_slot: &mut f64,
        var_fn25_calc_iq__qd1_dn16_slot: &mut f64,
        var_fn25_calc_iq__qd1_dn17_slot: &mut f64,
        var_fn25_calc_iq__qd1_dn2_slot: &mut f64,
        var_fn25_calc_iq__qd1_dn4_slot: &mut f64,
        var_fn25_calc_iq__qd1_dn7_slot: &mut f64,
        var_fn25_calc_iq__qd1_rv_slot: &mut f64,
        var_fn25_calc_iq__qd2_slot: &mut f64,
        var_fn25_calc_iq__qd2_dn16_slot: &mut f64,
        var_fn25_calc_iq__qd2_dn17_slot: &mut f64,
        var_fn25_calc_iq__qd2_dn2_slot: &mut f64,
        var_fn25_calc_iq__qd2_dn4_slot: &mut f64,
        var_fn25_calc_iq__qd2_dn7_slot: &mut f64,
        var_fn25_calc_iq__qd2_rv_slot: &mut f64,
        var_fn25_calc_iq__qd3_slot: &mut f64,
        var_fn25_calc_iq__qd3_dn16_slot: &mut f64,
        var_fn25_calc_iq__qd3_dn17_slot: &mut f64,
        var_fn25_calc_iq__qd3_dn2_slot: &mut f64,
        var_fn25_calc_iq__qd3_dn4_slot: &mut f64,
        var_fn25_calc_iq__qd3_dn7_slot: &mut f64,
        var_fn25_calc_iq__qd3_rv_slot: &mut f64,
        var_fn25_calc_iq__qd_dn16_slot: &mut f64,
        var_fn25_calc_iq__qd_dn17_slot: &mut f64,
        var_fn25_calc_iq__qd_dn2_slot: &mut f64,
        var_fn25_calc_iq__qd_dn4_slot: &mut f64,
        var_fn25_calc_iq__qd_dn7_slot: &mut f64,
        var_fn25_calc_iq__qd_rv_slot: &mut f64,
        var_fn25_calc_iq__qinvd0_slot: &mut f64,
        var_fn25_calc_iq__qinvd0_dn16_slot: &mut f64,
        var_fn25_calc_iq__qinvd0_dn17_slot: &mut f64,
        var_fn25_calc_iq__qinvd0_dn2_slot: &mut f64,
        var_fn25_calc_iq__qinvd0_dn4_slot: &mut f64,
        var_fn25_calc_iq__qinvd0_dn7_slot: &mut f64,
        var_fn25_calc_iq__qinvd0_rv_slot: &mut f64,
        var_fn25_calc_iq__qinvdd_slot: &mut f64,
        var_fn25_calc_iq__qinvdd_dn16_slot: &mut f64,
        var_fn25_calc_iq__qinvdd_dn17_slot: &mut f64,
        var_fn25_calc_iq__qinvdd_dn2_slot: &mut f64,
        var_fn25_calc_iq__qinvdd_dn4_slot: &mut f64,
        var_fn25_calc_iq__qinvdd_dn7_slot: &mut f64,
        var_fn25_calc_iq__qinvdd_rv_slot: &mut f64,
        var_fn25_calc_iq__qs_slot: &mut f64,
        var_fn25_calc_iq__qs2_slot: &mut f64,
        var_fn25_calc_iq__qs2_dn16_slot: &mut f64,
        var_fn25_calc_iq__qs2_dn17_slot: &mut f64,
        var_fn25_calc_iq__qs2_dn2_slot: &mut f64,
        var_fn25_calc_iq__qs2_dn4_slot: &mut f64,
        var_fn25_calc_iq__qs2_dn7_slot: &mut f64,
        var_fn25_calc_iq__qs2_rv_slot: &mut f64,
        var_fn25_calc_iq__qs3_slot: &mut f64,
        var_fn25_calc_iq__qs3_dn16_slot: &mut f64,
        var_fn25_calc_iq__qs3_dn17_slot: &mut f64,
        var_fn25_calc_iq__qs3_dn2_slot: &mut f64,
        var_fn25_calc_iq__qs3_dn4_slot: &mut f64,
        var_fn25_calc_iq__qs3_dn7_slot: &mut f64,
        var_fn25_calc_iq__qs3_rv_slot: &mut f64,
        var_fn25_calc_iq__qs_dn16_slot: &mut f64,
        var_fn25_calc_iq__qs_dn17_slot: &mut f64,
        var_fn25_calc_iq__qs_dn2_slot: &mut f64,
        var_fn25_calc_iq__qs_dn4_slot: &mut f64,
        var_fn25_calc_iq__qs_dn7_slot: &mut f64,
        var_fn25_calc_iq__qs_rv_slot: &mut f64,
        var_fn25_calc_iq__qsqd_slot: &mut f64,
        var_fn25_calc_iq__qsqd_dn16_slot: &mut f64,
        var_fn25_calc_iq__qsqd_dn17_slot: &mut f64,
        var_fn25_calc_iq__qsqd_dn2_slot: &mut f64,
        var_fn25_calc_iq__qsqd_dn4_slot: &mut f64,
        var_fn25_calc_iq__qsqd_dn7_slot: &mut f64,
        var_fn25_calc_iq__qsqd_rv_slot: &mut f64,
        var_fn25_calc_iq__tfacmobin_slot: &mut f64,
        var_fn25_calc_iq__tfacmobin_dn4_slot: &mut f64,
        var_fn25_calc_iq__tfacmobin_rv_slot: &mut f64,
        var_fn25_calc_iq__vdx0_slot: &mut f64,
        var_fn25_calc_iq__vdx0_dn16_slot: &mut f64,
        var_fn25_calc_iq__vdx0_dn17_slot: &mut f64,
        var_fn25_calc_iq__vdx0_dn2_slot: &mut f64,
        var_fn25_calc_iq__vdx0_dn4_slot: &mut f64,
        var_fn25_calc_iq__vdx0_dn7_slot: &mut f64,
        var_fn25_calc_iq__vdx0_rv_slot: &mut f64,
        var_fn25_calc_iq__vgdin_slot: &mut f64,
        var_fn25_calc_iq__vgdin_dn16_slot: &mut f64,
        var_fn25_calc_iq__vgdin_dn17_slot: &mut f64,
        var_fn25_calc_iq__vgdin_dn2_slot: &mut f64,
        var_fn25_calc_iq__vgdin_dn7_slot: &mut f64,
        var_fn25_calc_iq__vgdin_rv_slot: &mut f64,
        var_fn25_calc_iq__vsatdibl_slot: &mut f64,
        var_fn25_calc_iq__vsatdibl_dn16_slot: &mut f64,
        var_fn25_calc_iq__vsatdibl_dn17_slot: &mut f64,
        var_fn25_calc_iq__vsatdibl_rv_slot: &mut f64,
        var_fn25_calc_iq__vsx0_slot: &mut f64,
        var_fn25_calc_iq__vsx0_dn16_slot: &mut f64,
        var_fn25_calc_iq__vsx0_dn17_slot: &mut f64,
        var_fn25_calc_iq__vsx0_dn2_slot: &mut f64,
        var_fn25_calc_iq__vsx0_dn4_slot: &mut f64,
        var_fn25_calc_iq__vsx0_dn7_slot: &mut f64,
        var_fn25_calc_iq__vsx0_rv_slot: &mut f64,
        var_fn25_calc_iq__vtof_slot: &mut f64,
        var_fn25_calc_iq__vtof_dn4_slot: &mut f64,
        var_fn25_calc_iq__vtof_rv_slot: &mut f64,
        var_guard26_slot: &mut f64,
        var_guard26_rv_slot: &mut f64,
    ) {
        let mut var_fn25_calc_iq__absvdsin: f64 = *var_fn25_calc_iq__absvdsin_slot;
        let mut var_fn25_calc_iq__absvdsin_dn16: f64 = *var_fn25_calc_iq__absvdsin_dn16_slot;
        let mut var_fn25_calc_iq__absvdsin_dn17: f64 = *var_fn25_calc_iq__absvdsin_dn17_slot;
        let mut var_fn25_calc_iq__absvdsin_rv: f64 = *var_fn25_calc_iq__absvdsin_rv_slot;
        let mut var_fn25_calc_iq__alpha_phit: f64 = *var_fn25_calc_iq__alpha_phit_slot;
        let mut var_fn25_calc_iq__alpha_phit_dn4: f64 = *var_fn25_calc_iq__alpha_phit_dn4_slot;
        let mut var_fn25_calc_iq__alpha_phit_rv: f64 = *var_fn25_calc_iq__alpha_phit_rv_slot;
        let mut var_fn25_calc_iq__etab: f64 = *var_fn25_calc_iq__etab_slot;
        let mut var_fn25_calc_iq__etab_dn16: f64 = *var_fn25_calc_iq__etab_dn16_slot;
        let mut var_fn25_calc_iq__etab_dn3: f64 = *var_fn25_calc_iq__etab_dn3_slot;
        let mut var_fn25_calc_iq__etab_dn4: f64 = *var_fn25_calc_iq__etab_dn4_slot;
        let mut var_fn25_calc_iq__etab_rv: f64 = *var_fn25_calc_iq__etab_rv_slot;
        let mut var_fn25_calc_iq__etac: f64 = *var_fn25_calc_iq__etac_slot;
        let mut var_fn25_calc_iq__etac_dn16: f64 = *var_fn25_calc_iq__etac_dn16_slot;
        let mut var_fn25_calc_iq__etac_dn2: f64 = *var_fn25_calc_iq__etac_dn2_slot;
        let mut var_fn25_calc_iq__etac_dn4: f64 = *var_fn25_calc_iq__etac_dn4_slot;
        let mut var_fn25_calc_iq__etac_dn7: f64 = *var_fn25_calc_iq__etac_dn7_slot;
        let mut var_fn25_calc_iq__etac_rv: f64 = *var_fn25_calc_iq__etac_rv_slot;
        let mut var_fn25_calc_iq__etad0: f64 = *var_fn25_calc_iq__etad0_slot;
        let mut var_fn25_calc_iq__etad0_dn16: f64 = *var_fn25_calc_iq__etad0_dn16_slot;
        let mut var_fn25_calc_iq__etad0_dn17: f64 = *var_fn25_calc_iq__etad0_dn17_slot;
        let mut var_fn25_calc_iq__etad0_dn2: f64 = *var_fn25_calc_iq__etad0_dn2_slot;
        let mut var_fn25_calc_iq__etad0_dn4: f64 = *var_fn25_calc_iq__etad0_dn4_slot;
        let mut var_fn25_calc_iq__etad0_dn7: f64 = *var_fn25_calc_iq__etad0_dn7_slot;
        let mut var_fn25_calc_iq__etad0_rv: f64 = *var_fn25_calc_iq__etad0_rv_slot;
        let mut var_fn25_calc_iq__etags: f64 = *var_fn25_calc_iq__etags_slot;
        let mut var_fn25_calc_iq__etags_dn16: f64 = *var_fn25_calc_iq__etags_dn16_slot;
        let mut var_fn25_calc_iq__etags_dn2: f64 = *var_fn25_calc_iq__etags_dn2_slot;
        let mut var_fn25_calc_iq__etags_dn4: f64 = *var_fn25_calc_iq__etags_dn4_slot;
        let mut var_fn25_calc_iq__etags_dn7: f64 = *var_fn25_calc_iq__etags_dn7_slot;
        let mut var_fn25_calc_iq__etags_rv: f64 = *var_fn25_calc_iq__etags_rv_slot;
        let mut var_fn25_calc_iq__exparg: f64 = *var_fn25_calc_iq__exparg_slot;
        let mut var_fn25_calc_iq__exparg0: f64 = *var_fn25_calc_iq__exparg0_slot;
        let mut var_fn25_calc_iq__exparg0_dn16: f64 = *var_fn25_calc_iq__exparg0_dn16_slot;
        let mut var_fn25_calc_iq__exparg0_dn17: f64 = *var_fn25_calc_iq__exparg0_dn17_slot;
        let mut var_fn25_calc_iq__exparg0_dn2: f64 = *var_fn25_calc_iq__exparg0_dn2_slot;
        let mut var_fn25_calc_iq__exparg0_dn4: f64 = *var_fn25_calc_iq__exparg0_dn4_slot;
        let mut var_fn25_calc_iq__exparg0_dn7: f64 = *var_fn25_calc_iq__exparg0_dn7_slot;
        let mut var_fn25_calc_iq__exparg0_rv: f64 = *var_fn25_calc_iq__exparg0_rv_slot;
        let mut var_fn25_calc_iq__exparg_dn16: f64 = *var_fn25_calc_iq__exparg_dn16_slot;
        let mut var_fn25_calc_iq__exparg_dn17: f64 = *var_fn25_calc_iq__exparg_dn17_slot;
        let mut var_fn25_calc_iq__exparg_dn2: f64 = *var_fn25_calc_iq__exparg_dn2_slot;
        let mut var_fn25_calc_iq__exparg_dn3: f64 = *var_fn25_calc_iq__exparg_dn3_slot;
        let mut var_fn25_calc_iq__exparg_dn4: f64 = *var_fn25_calc_iq__exparg_dn4_slot;
        let mut var_fn25_calc_iq__exparg_dn7: f64 = *var_fn25_calc_iq__exparg_dn7_slot;
        let mut var_fn25_calc_iq__exparg_rv: f64 = *var_fn25_calc_iq__exparg_rv_slot;
        let mut var_fn25_calc_iq__fds0: f64 = *var_fn25_calc_iq__fds0_slot;
        let mut var_fn25_calc_iq__fds0_dn16: f64 = *var_fn25_calc_iq__fds0_dn16_slot;
        let mut var_fn25_calc_iq__fds0_dn17: f64 = *var_fn25_calc_iq__fds0_dn17_slot;
        let mut var_fn25_calc_iq__fds0_dn2: f64 = *var_fn25_calc_iq__fds0_dn2_slot;
        let mut var_fn25_calc_iq__fds0_dn4: f64 = *var_fn25_calc_iq__fds0_dn4_slot;
        let mut var_fn25_calc_iq__fds0_dn7: f64 = *var_fn25_calc_iq__fds0_dn7_slot;
        let mut var_fn25_calc_iq__fds0_rv: f64 = *var_fn25_calc_iq__fds0_rv_slot;
        let mut var_fn25_calc_iq__ffd0: f64 = *var_fn25_calc_iq__ffd0_slot;
        let mut var_fn25_calc_iq__ffd0_dn16: f64 = *var_fn25_calc_iq__ffd0_dn16_slot;
        let mut var_fn25_calc_iq__ffd0_dn17: f64 = *var_fn25_calc_iq__ffd0_dn17_slot;
        let mut var_fn25_calc_iq__ffd0_dn2: f64 = *var_fn25_calc_iq__ffd0_dn2_slot;
        let mut var_fn25_calc_iq__ffd0_dn4: f64 = *var_fn25_calc_iq__ffd0_dn4_slot;
        let mut var_fn25_calc_iq__ffd0_dn7: f64 = *var_fn25_calc_iq__ffd0_dn7_slot;
        let mut var_fn25_calc_iq__ffd0_rv: f64 = *var_fn25_calc_iq__ffd0_rv_slot;
        let mut var_fn25_calc_iq__fsd0: f64 = *var_fn25_calc_iq__fsd0_slot;
        let mut var_fn25_calc_iq__fsd0_dn16: f64 = *var_fn25_calc_iq__fsd0_dn16_slot;
        let mut var_fn25_calc_iq__fsd0_dn17: f64 = *var_fn25_calc_iq__fsd0_dn17_slot;
        let mut var_fn25_calc_iq__fsd0_dn2: f64 = *var_fn25_calc_iq__fsd0_dn2_slot;
        let mut var_fn25_calc_iq__fsd0_dn4: f64 = *var_fn25_calc_iq__fsd0_dn4_slot;
        let mut var_fn25_calc_iq__fsd0_dn7: f64 = *var_fn25_calc_iq__fsd0_dn7_slot;
        let mut var_fn25_calc_iq__fsd0_rv: f64 = *var_fn25_calc_iq__fsd0_rv_slot;
        let mut var_fn25_calc_iq__myarg: f64 = *var_fn25_calc_iq__myarg_slot;
        let mut var_fn25_calc_iq__myarg0: f64 = *var_fn25_calc_iq__myarg0_slot;
        let mut var_fn25_calc_iq__myarg0_dn4: f64 = *var_fn25_calc_iq__myarg0_dn4_slot;
        let mut var_fn25_calc_iq__myarg0_rv: f64 = *var_fn25_calc_iq__myarg0_rv_slot;
        let mut var_fn25_calc_iq__myarg_dn16: f64 = *var_fn25_calc_iq__myarg_dn16_slot;
        let mut var_fn25_calc_iq__myarg_dn17: f64 = *var_fn25_calc_iq__myarg_dn17_slot;
        let mut var_fn25_calc_iq__myarg_dn2: f64 = *var_fn25_calc_iq__myarg_dn2_slot;
        let mut var_fn25_calc_iq__myarg_dn3: f64 = *var_fn25_calc_iq__myarg_dn3_slot;
        let mut var_fn25_calc_iq__myarg_dn4: f64 = *var_fn25_calc_iq__myarg_dn4_slot;
        let mut var_fn25_calc_iq__myarg_dn7: f64 = *var_fn25_calc_iq__myarg_dn7_slot;
        let mut var_fn25_calc_iq__myarg_rv: f64 = *var_fn25_calc_iq__myarg_rv_slot;
        let mut var_fn25_calc_iq__n: f64 = *var_fn25_calc_iq__n_slot;
        let mut var_fn25_calc_iq__n_dn16: f64 = *var_fn25_calc_iq__n_dn16_slot;
        let mut var_fn25_calc_iq__n_dn17: f64 = *var_fn25_calc_iq__n_dn17_slot;
        let mut var_fn25_calc_iq__n_dn4: f64 = *var_fn25_calc_iq__n_dn4_slot;
        let mut var_fn25_calc_iq__n_rv: f64 = *var_fn25_calc_iq__n_rv_slot;
        let mut var_fn25_calc_iq__qd: f64 = *var_fn25_calc_iq__qd_slot;
        let mut var_fn25_calc_iq__qd1: f64 = *var_fn25_calc_iq__qd1_slot;
        let mut var_fn25_calc_iq__qd1_dn16: f64 = *var_fn25_calc_iq__qd1_dn16_slot;
        let mut var_fn25_calc_iq__qd1_dn17: f64 = *var_fn25_calc_iq__qd1_dn17_slot;
        let mut var_fn25_calc_iq__qd1_dn2: f64 = *var_fn25_calc_iq__qd1_dn2_slot;
        let mut var_fn25_calc_iq__qd1_dn4: f64 = *var_fn25_calc_iq__qd1_dn4_slot;
        let mut var_fn25_calc_iq__qd1_dn7: f64 = *var_fn25_calc_iq__qd1_dn7_slot;
        let mut var_fn25_calc_iq__qd1_rv: f64 = *var_fn25_calc_iq__qd1_rv_slot;
        let mut var_fn25_calc_iq__qd2: f64 = *var_fn25_calc_iq__qd2_slot;
        let mut var_fn25_calc_iq__qd2_dn16: f64 = *var_fn25_calc_iq__qd2_dn16_slot;
        let mut var_fn25_calc_iq__qd2_dn17: f64 = *var_fn25_calc_iq__qd2_dn17_slot;
        let mut var_fn25_calc_iq__qd2_dn2: f64 = *var_fn25_calc_iq__qd2_dn2_slot;
        let mut var_fn25_calc_iq__qd2_dn4: f64 = *var_fn25_calc_iq__qd2_dn4_slot;
        let mut var_fn25_calc_iq__qd2_dn7: f64 = *var_fn25_calc_iq__qd2_dn7_slot;
        let mut var_fn25_calc_iq__qd2_rv: f64 = *var_fn25_calc_iq__qd2_rv_slot;
        let mut var_fn25_calc_iq__qd3: f64 = *var_fn25_calc_iq__qd3_slot;
        let mut var_fn25_calc_iq__qd3_dn16: f64 = *var_fn25_calc_iq__qd3_dn16_slot;
        let mut var_fn25_calc_iq__qd3_dn17: f64 = *var_fn25_calc_iq__qd3_dn17_slot;
        let mut var_fn25_calc_iq__qd3_dn2: f64 = *var_fn25_calc_iq__qd3_dn2_slot;
        let mut var_fn25_calc_iq__qd3_dn4: f64 = *var_fn25_calc_iq__qd3_dn4_slot;
        let mut var_fn25_calc_iq__qd3_dn7: f64 = *var_fn25_calc_iq__qd3_dn7_slot;
        let mut var_fn25_calc_iq__qd3_rv: f64 = *var_fn25_calc_iq__qd3_rv_slot;
        let mut var_fn25_calc_iq__qd_dn16: f64 = *var_fn25_calc_iq__qd_dn16_slot;
        let mut var_fn25_calc_iq__qd_dn17: f64 = *var_fn25_calc_iq__qd_dn17_slot;
        let mut var_fn25_calc_iq__qd_dn2: f64 = *var_fn25_calc_iq__qd_dn2_slot;
        let mut var_fn25_calc_iq__qd_dn4: f64 = *var_fn25_calc_iq__qd_dn4_slot;
        let mut var_fn25_calc_iq__qd_dn7: f64 = *var_fn25_calc_iq__qd_dn7_slot;
        let mut var_fn25_calc_iq__qd_rv: f64 = *var_fn25_calc_iq__qd_rv_slot;
        let mut var_fn25_calc_iq__qinvd0: f64 = *var_fn25_calc_iq__qinvd0_slot;
        let mut var_fn25_calc_iq__qinvd0_dn16: f64 = *var_fn25_calc_iq__qinvd0_dn16_slot;
        let mut var_fn25_calc_iq__qinvd0_dn17: f64 = *var_fn25_calc_iq__qinvd0_dn17_slot;
        let mut var_fn25_calc_iq__qinvd0_dn2: f64 = *var_fn25_calc_iq__qinvd0_dn2_slot;
        let mut var_fn25_calc_iq__qinvd0_dn4: f64 = *var_fn25_calc_iq__qinvd0_dn4_slot;
        let mut var_fn25_calc_iq__qinvd0_dn7: f64 = *var_fn25_calc_iq__qinvd0_dn7_slot;
        let mut var_fn25_calc_iq__qinvd0_rv: f64 = *var_fn25_calc_iq__qinvd0_rv_slot;
        let mut var_fn25_calc_iq__qinvdd: f64 = *var_fn25_calc_iq__qinvdd_slot;
        let mut var_fn25_calc_iq__qinvdd_dn16: f64 = *var_fn25_calc_iq__qinvdd_dn16_slot;
        let mut var_fn25_calc_iq__qinvdd_dn17: f64 = *var_fn25_calc_iq__qinvdd_dn17_slot;
        let mut var_fn25_calc_iq__qinvdd_dn2: f64 = *var_fn25_calc_iq__qinvdd_dn2_slot;
        let mut var_fn25_calc_iq__qinvdd_dn4: f64 = *var_fn25_calc_iq__qinvdd_dn4_slot;
        let mut var_fn25_calc_iq__qinvdd_dn7: f64 = *var_fn25_calc_iq__qinvdd_dn7_slot;
        let mut var_fn25_calc_iq__qinvdd_rv: f64 = *var_fn25_calc_iq__qinvdd_rv_slot;
        let mut var_fn25_calc_iq__qs: f64 = *var_fn25_calc_iq__qs_slot;
        let mut var_fn25_calc_iq__qs2: f64 = *var_fn25_calc_iq__qs2_slot;
        let mut var_fn25_calc_iq__qs2_dn16: f64 = *var_fn25_calc_iq__qs2_dn16_slot;
        let mut var_fn25_calc_iq__qs2_dn17: f64 = *var_fn25_calc_iq__qs2_dn17_slot;
        let mut var_fn25_calc_iq__qs2_dn2: f64 = *var_fn25_calc_iq__qs2_dn2_slot;
        let mut var_fn25_calc_iq__qs2_dn4: f64 = *var_fn25_calc_iq__qs2_dn4_slot;
        let mut var_fn25_calc_iq__qs2_dn7: f64 = *var_fn25_calc_iq__qs2_dn7_slot;
        let mut var_fn25_calc_iq__qs2_rv: f64 = *var_fn25_calc_iq__qs2_rv_slot;
        let mut var_fn25_calc_iq__qs3: f64 = *var_fn25_calc_iq__qs3_slot;
        let mut var_fn25_calc_iq__qs3_dn16: f64 = *var_fn25_calc_iq__qs3_dn16_slot;
        let mut var_fn25_calc_iq__qs3_dn17: f64 = *var_fn25_calc_iq__qs3_dn17_slot;
        let mut var_fn25_calc_iq__qs3_dn2: f64 = *var_fn25_calc_iq__qs3_dn2_slot;
        let mut var_fn25_calc_iq__qs3_dn4: f64 = *var_fn25_calc_iq__qs3_dn4_slot;
        let mut var_fn25_calc_iq__qs3_dn7: f64 = *var_fn25_calc_iq__qs3_dn7_slot;
        let mut var_fn25_calc_iq__qs3_rv: f64 = *var_fn25_calc_iq__qs3_rv_slot;
        let mut var_fn25_calc_iq__qs_dn16: f64 = *var_fn25_calc_iq__qs_dn16_slot;
        let mut var_fn25_calc_iq__qs_dn17: f64 = *var_fn25_calc_iq__qs_dn17_slot;
        let mut var_fn25_calc_iq__qs_dn2: f64 = *var_fn25_calc_iq__qs_dn2_slot;
        let mut var_fn25_calc_iq__qs_dn4: f64 = *var_fn25_calc_iq__qs_dn4_slot;
        let mut var_fn25_calc_iq__qs_dn7: f64 = *var_fn25_calc_iq__qs_dn7_slot;
        let mut var_fn25_calc_iq__qs_rv: f64 = *var_fn25_calc_iq__qs_rv_slot;
        let mut var_fn25_calc_iq__qsqd: f64 = *var_fn25_calc_iq__qsqd_slot;
        let mut var_fn25_calc_iq__qsqd_dn16: f64 = *var_fn25_calc_iq__qsqd_dn16_slot;
        let mut var_fn25_calc_iq__qsqd_dn17: f64 = *var_fn25_calc_iq__qsqd_dn17_slot;
        let mut var_fn25_calc_iq__qsqd_dn2: f64 = *var_fn25_calc_iq__qsqd_dn2_slot;
        let mut var_fn25_calc_iq__qsqd_dn4: f64 = *var_fn25_calc_iq__qsqd_dn4_slot;
        let mut var_fn25_calc_iq__qsqd_dn7: f64 = *var_fn25_calc_iq__qsqd_dn7_slot;
        let mut var_fn25_calc_iq__qsqd_rv: f64 = *var_fn25_calc_iq__qsqd_rv_slot;
        let mut var_fn25_calc_iq__tfacmobin: f64 = *var_fn25_calc_iq__tfacmobin_slot;
        let mut var_fn25_calc_iq__tfacmobin_dn4: f64 = *var_fn25_calc_iq__tfacmobin_dn4_slot;
        let mut var_fn25_calc_iq__tfacmobin_rv: f64 = *var_fn25_calc_iq__tfacmobin_rv_slot;
        let mut var_fn25_calc_iq__vdx0: f64 = *var_fn25_calc_iq__vdx0_slot;
        let mut var_fn25_calc_iq__vdx0_dn16: f64 = *var_fn25_calc_iq__vdx0_dn16_slot;
        let mut var_fn25_calc_iq__vdx0_dn17: f64 = *var_fn25_calc_iq__vdx0_dn17_slot;
        let mut var_fn25_calc_iq__vdx0_dn2: f64 = *var_fn25_calc_iq__vdx0_dn2_slot;
        let mut var_fn25_calc_iq__vdx0_dn4: f64 = *var_fn25_calc_iq__vdx0_dn4_slot;
        let mut var_fn25_calc_iq__vdx0_dn7: f64 = *var_fn25_calc_iq__vdx0_dn7_slot;
        let mut var_fn25_calc_iq__vdx0_rv: f64 = *var_fn25_calc_iq__vdx0_rv_slot;
        let mut var_fn25_calc_iq__vgdin: f64 = *var_fn25_calc_iq__vgdin_slot;
        let mut var_fn25_calc_iq__vgdin_dn16: f64 = *var_fn25_calc_iq__vgdin_dn16_slot;
        let mut var_fn25_calc_iq__vgdin_dn17: f64 = *var_fn25_calc_iq__vgdin_dn17_slot;
        let mut var_fn25_calc_iq__vgdin_dn2: f64 = *var_fn25_calc_iq__vgdin_dn2_slot;
        let mut var_fn25_calc_iq__vgdin_dn7: f64 = *var_fn25_calc_iq__vgdin_dn7_slot;
        let mut var_fn25_calc_iq__vgdin_rv: f64 = *var_fn25_calc_iq__vgdin_rv_slot;
        let mut var_fn25_calc_iq__vsatdibl: f64 = *var_fn25_calc_iq__vsatdibl_slot;
        let mut var_fn25_calc_iq__vsatdibl_dn16: f64 = *var_fn25_calc_iq__vsatdibl_dn16_slot;
        let mut var_fn25_calc_iq__vsatdibl_dn17: f64 = *var_fn25_calc_iq__vsatdibl_dn17_slot;
        let mut var_fn25_calc_iq__vsatdibl_rv: f64 = *var_fn25_calc_iq__vsatdibl_rv_slot;
        let mut var_fn25_calc_iq__vsx0: f64 = *var_fn25_calc_iq__vsx0_slot;
        let mut var_fn25_calc_iq__vsx0_dn16: f64 = *var_fn25_calc_iq__vsx0_dn16_slot;
        let mut var_fn25_calc_iq__vsx0_dn17: f64 = *var_fn25_calc_iq__vsx0_dn17_slot;
        let mut var_fn25_calc_iq__vsx0_dn2: f64 = *var_fn25_calc_iq__vsx0_dn2_slot;
        let mut var_fn25_calc_iq__vsx0_dn4: f64 = *var_fn25_calc_iq__vsx0_dn4_slot;
        let mut var_fn25_calc_iq__vsx0_dn7: f64 = *var_fn25_calc_iq__vsx0_dn7_slot;
        let mut var_fn25_calc_iq__vsx0_rv: f64 = *var_fn25_calc_iq__vsx0_rv_slot;
        let mut var_fn25_calc_iq__vtof: f64 = *var_fn25_calc_iq__vtof_slot;
        let mut var_fn25_calc_iq__vtof_dn4: f64 = *var_fn25_calc_iq__vtof_dn4_slot;
        let mut var_fn25_calc_iq__vtof_rv: f64 = *var_fn25_calc_iq__vtof_rv_slot;
        let mut var_guard26: f64 = *var_guard26_slot;
        let mut var_guard26_rv: f64 = *var_guard26_rv_slot;

        let (assign2630_e4237, assign2630_e4237_d_n2, assign2630_e4237_d_n4, assign2630_e4237_d_n7, assign2630_e4237_d_n16, assign2630_e4237_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__fsd0, var_fn25_calc_iq__fsd0_dn2, var_fn25_calc_iq__fsd0_dn4, var_fn25_calc_iq__fsd0_dn7, var_fn25_calc_iq__fsd0_dn16, var_fn25_calc_iq__fsd0_dn17,)
    }
};
        var_fn25_calc_iq__fsd0 = assign2630_e4237;
        var_fn25_calc_iq__fsd0_dn2 = assign2630_e4237_d_n2;
        var_fn25_calc_iq__fsd0_dn4 = assign2630_e4237_d_n4;
        var_fn25_calc_iq__fsd0_dn7 = assign2630_e4237_d_n7;
        var_fn25_calc_iq__fsd0_dn16 = assign2630_e4237_d_n16;
        var_fn25_calc_iq__fsd0_dn17 = assign2630_e4237_d_n17;
        var_fn25_calc_iq__fsd0_rv = 0.0;

        let (assign2640_e4241, assign2640_e4241_d_n2, assign2640_e4241_d_n4, assign2640_e4241_d_n7, assign2640_e4241_d_n16, assign2640_e4241_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__vdx0, var_fn25_calc_iq__vdx0_dn2, var_fn25_calc_iq__vdx0_dn4, var_fn25_calc_iq__vdx0_dn7, var_fn25_calc_iq__vdx0_dn16, var_fn25_calc_iq__vdx0_dn17,)
    }
};
        var_fn25_calc_iq__vdx0 = assign2640_e4241;
        var_fn25_calc_iq__vdx0_dn2 = assign2640_e4241_d_n2;
        var_fn25_calc_iq__vdx0_dn4 = assign2640_e4241_d_n4;
        var_fn25_calc_iq__vdx0_dn7 = assign2640_e4241_d_n7;
        var_fn25_calc_iq__vdx0_dn16 = assign2640_e4241_d_n16;
        var_fn25_calc_iq__vdx0_dn17 = assign2640_e4241_d_n17;
        var_fn25_calc_iq__vdx0_rv = 0.0;

        let (assign2650_e4245, assign2650_e4245_d_n2, assign2650_e4245_d_n4, assign2650_e4245_d_n7, assign2650_e4245_d_n16, assign2650_e4245_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__fds0, var_fn25_calc_iq__fds0_dn2, var_fn25_calc_iq__fds0_dn4, var_fn25_calc_iq__fds0_dn7, var_fn25_calc_iq__fds0_dn16, var_fn25_calc_iq__fds0_dn17,)
    }
};
        var_fn25_calc_iq__fds0 = assign2650_e4245;
        var_fn25_calc_iq__fds0_dn2 = assign2650_e4245_d_n2;
        var_fn25_calc_iq__fds0_dn4 = assign2650_e4245_d_n4;
        var_fn25_calc_iq__fds0_dn7 = assign2650_e4245_d_n7;
        var_fn25_calc_iq__fds0_dn16 = assign2650_e4245_d_n16;
        var_fn25_calc_iq__fds0_dn17 = assign2650_e4245_d_n17;
        var_fn25_calc_iq__fds0_rv = 0.0;

        let (assign2660_e4249, assign2660_e4249_d_n2, assign2660_e4249_d_n4, assign2660_e4249_d_n7, assign2660_e4249_d_n16, assign2660_e4249_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__vsx0, var_fn25_calc_iq__vsx0_dn2, var_fn25_calc_iq__vsx0_dn4, var_fn25_calc_iq__vsx0_dn7, var_fn25_calc_iq__vsx0_dn16, var_fn25_calc_iq__vsx0_dn17,)
    }
};
        var_fn25_calc_iq__vsx0 = assign2660_e4249;
        var_fn25_calc_iq__vsx0_dn2 = assign2660_e4249_d_n2;
        var_fn25_calc_iq__vsx0_dn4 = assign2660_e4249_d_n4;
        var_fn25_calc_iq__vsx0_dn7 = assign2660_e4249_d_n7;
        var_fn25_calc_iq__vsx0_dn16 = assign2660_e4249_d_n16;
        var_fn25_calc_iq__vsx0_dn17 = assign2660_e4249_d_n17;
        var_fn25_calc_iq__vsx0_rv = 0.0;

        let (assign2670_e4253, assign2670_e4253_d_n2, assign2670_e4253_d_n4, assign2670_e4253_d_n7, assign2670_e4253_d_n16, assign2670_e4253_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__ffd0, var_fn25_calc_iq__ffd0_dn2, var_fn25_calc_iq__ffd0_dn4, var_fn25_calc_iq__ffd0_dn7, var_fn25_calc_iq__ffd0_dn16, var_fn25_calc_iq__ffd0_dn17,)
    }
};
        var_fn25_calc_iq__ffd0 = assign2670_e4253;
        var_fn25_calc_iq__ffd0_dn2 = assign2670_e4253_d_n2;
        var_fn25_calc_iq__ffd0_dn4 = assign2670_e4253_d_n4;
        var_fn25_calc_iq__ffd0_dn7 = assign2670_e4253_d_n7;
        var_fn25_calc_iq__ffd0_dn16 = assign2670_e4253_d_n16;
        var_fn25_calc_iq__ffd0_dn17 = assign2670_e4253_d_n17;
        var_fn25_calc_iq__ffd0_rv = 0.0;

        let (assign2680_e4257, assign2680_e4257_d_n2, assign2680_e4257_d_n4, assign2680_e4257_d_n7, assign2680_e4257_d_n16, assign2680_e4257_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__etad0, var_fn25_calc_iq__etad0_dn2, var_fn25_calc_iq__etad0_dn4, var_fn25_calc_iq__etad0_dn7, var_fn25_calc_iq__etad0_dn16, var_fn25_calc_iq__etad0_dn17,)
    }
};
        var_fn25_calc_iq__etad0 = assign2680_e4257;
        var_fn25_calc_iq__etad0_dn2 = assign2680_e4257_d_n2;
        var_fn25_calc_iq__etad0_dn4 = assign2680_e4257_d_n4;
        var_fn25_calc_iq__etad0_dn7 = assign2680_e4257_d_n7;
        var_fn25_calc_iq__etad0_dn16 = assign2680_e4257_d_n16;
        var_fn25_calc_iq__etad0_dn17 = assign2680_e4257_d_n17;
        var_fn25_calc_iq__etad0_rv = 0.0;

        let (assign2690_e4261, assign2690_e4261_d_n2, assign2690_e4261_d_n4, assign2690_e4261_d_n7, assign2690_e4261_d_n16, assign2690_e4261_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__qinvd0, var_fn25_calc_iq__qinvd0_dn2, var_fn25_calc_iq__qinvd0_dn4, var_fn25_calc_iq__qinvd0_dn7, var_fn25_calc_iq__qinvd0_dn16, var_fn25_calc_iq__qinvd0_dn17,)
    }
};
        var_fn25_calc_iq__qinvd0 = assign2690_e4261;
        var_fn25_calc_iq__qinvd0_dn2 = assign2690_e4261_d_n2;
        var_fn25_calc_iq__qinvd0_dn4 = assign2690_e4261_d_n4;
        var_fn25_calc_iq__qinvd0_dn7 = assign2690_e4261_d_n7;
        var_fn25_calc_iq__qinvd0_dn16 = assign2690_e4261_d_n16;
        var_fn25_calc_iq__qinvd0_dn17 = assign2690_e4261_d_n17;
        var_fn25_calc_iq__qinvd0_rv = 0.0;

        let (assign2700_e4265, assign2700_e4265_d_n2, assign2700_e4265_d_n4, assign2700_e4265_d_n7, assign2700_e4265_d_n16, assign2700_e4265_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__qs2, var_fn25_calc_iq__qs2_dn2, var_fn25_calc_iq__qs2_dn4, var_fn25_calc_iq__qs2_dn7, var_fn25_calc_iq__qs2_dn16, var_fn25_calc_iq__qs2_dn17,)
    }
};
        var_fn25_calc_iq__qs2 = assign2700_e4265;
        var_fn25_calc_iq__qs2_dn2 = assign2700_e4265_d_n2;
        var_fn25_calc_iq__qs2_dn4 = assign2700_e4265_d_n4;
        var_fn25_calc_iq__qs2_dn7 = assign2700_e4265_d_n7;
        var_fn25_calc_iq__qs2_dn16 = assign2700_e4265_d_n16;
        var_fn25_calc_iq__qs2_dn17 = assign2700_e4265_d_n17;
        var_fn25_calc_iq__qs2_rv = 0.0;

        let (assign2710_e4269, assign2710_e4269_d_n2, assign2710_e4269_d_n4, assign2710_e4269_d_n7, assign2710_e4269_d_n16, assign2710_e4269_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__qs3, var_fn25_calc_iq__qs3_dn2, var_fn25_calc_iq__qs3_dn4, var_fn25_calc_iq__qs3_dn7, var_fn25_calc_iq__qs3_dn16, var_fn25_calc_iq__qs3_dn17,)
    }
};
        var_fn25_calc_iq__qs3 = assign2710_e4269;
        var_fn25_calc_iq__qs3_dn2 = assign2710_e4269_d_n2;
        var_fn25_calc_iq__qs3_dn4 = assign2710_e4269_d_n4;
        var_fn25_calc_iq__qs3_dn7 = assign2710_e4269_d_n7;
        var_fn25_calc_iq__qs3_dn16 = assign2710_e4269_d_n16;
        var_fn25_calc_iq__qs3_dn17 = assign2710_e4269_d_n17;
        var_fn25_calc_iq__qs3_rv = 0.0;

        let (assign2720_e4273, assign2720_e4273_d_n2, assign2720_e4273_d_n4, assign2720_e4273_d_n7, assign2720_e4273_d_n16, assign2720_e4273_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__qd2, var_fn25_calc_iq__qd2_dn2, var_fn25_calc_iq__qd2_dn4, var_fn25_calc_iq__qd2_dn7, var_fn25_calc_iq__qd2_dn16, var_fn25_calc_iq__qd2_dn17,)
    }
};
        var_fn25_calc_iq__qd2 = assign2720_e4273;
        var_fn25_calc_iq__qd2_dn2 = assign2720_e4273_d_n2;
        var_fn25_calc_iq__qd2_dn4 = assign2720_e4273_d_n4;
        var_fn25_calc_iq__qd2_dn7 = assign2720_e4273_d_n7;
        var_fn25_calc_iq__qd2_dn16 = assign2720_e4273_d_n16;
        var_fn25_calc_iq__qd2_dn17 = assign2720_e4273_d_n17;
        var_fn25_calc_iq__qd2_rv = 0.0;

        let (assign2730_e4277, assign2730_e4277_d_n2, assign2730_e4277_d_n4, assign2730_e4277_d_n7, assign2730_e4277_d_n16, assign2730_e4277_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__qd3, var_fn25_calc_iq__qd3_dn2, var_fn25_calc_iq__qd3_dn4, var_fn25_calc_iq__qd3_dn7, var_fn25_calc_iq__qd3_dn16, var_fn25_calc_iq__qd3_dn17,)
    }
};
        var_fn25_calc_iq__qd3 = assign2730_e4277;
        var_fn25_calc_iq__qd3_dn2 = assign2730_e4277_d_n2;
        var_fn25_calc_iq__qd3_dn4 = assign2730_e4277_d_n4;
        var_fn25_calc_iq__qd3_dn7 = assign2730_e4277_d_n7;
        var_fn25_calc_iq__qd3_dn16 = assign2730_e4277_d_n16;
        var_fn25_calc_iq__qd3_dn17 = assign2730_e4277_d_n17;
        var_fn25_calc_iq__qd3_rv = 0.0;

        let (assign2740_e4281, assign2740_e4281_d_n2, assign2740_e4281_d_n4, assign2740_e4281_d_n7, assign2740_e4281_d_n16, assign2740_e4281_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__qsqd, var_fn25_calc_iq__qsqd_dn2, var_fn25_calc_iq__qsqd_dn4, var_fn25_calc_iq__qsqd_dn7, var_fn25_calc_iq__qsqd_dn16, var_fn25_calc_iq__qsqd_dn17,)
    }
};
        var_fn25_calc_iq__qsqd = assign2740_e4281;
        var_fn25_calc_iq__qsqd_dn2 = assign2740_e4281_d_n2;
        var_fn25_calc_iq__qsqd_dn4 = assign2740_e4281_d_n4;
        var_fn25_calc_iq__qsqd_dn7 = assign2740_e4281_d_n7;
        var_fn25_calc_iq__qsqd_dn16 = assign2740_e4281_d_n16;
        var_fn25_calc_iq__qsqd_dn17 = assign2740_e4281_d_n17;
        var_fn25_calc_iq__qsqd_rv = 0.0;

        let (assign2750_e4285, assign2750_e4285_d_n2, assign2750_e4285_d_n4, assign2750_e4285_d_n7, assign2750_e4285_d_n16, assign2750_e4285_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__qinvdd, var_fn25_calc_iq__qinvdd_dn2, var_fn25_calc_iq__qinvdd_dn4, var_fn25_calc_iq__qinvdd_dn7, var_fn25_calc_iq__qinvdd_dn16, var_fn25_calc_iq__qinvdd_dn17,)
    }
};
        var_fn25_calc_iq__qinvdd = assign2750_e4285;
        var_fn25_calc_iq__qinvdd_dn2 = assign2750_e4285_d_n2;
        var_fn25_calc_iq__qinvdd_dn4 = assign2750_e4285_d_n4;
        var_fn25_calc_iq__qinvdd_dn7 = assign2750_e4285_d_n7;
        var_fn25_calc_iq__qinvdd_dn16 = assign2750_e4285_d_n16;
        var_fn25_calc_iq__qinvdd_dn17 = assign2750_e4285_d_n17;
        var_fn25_calc_iq__qinvdd_rv = 0.0;

        let (assign2760_e4289, assign2760_e4289_d_n2, assign2760_e4289_d_n4, assign2760_e4289_d_n7, assign2760_e4289_d_n16, assign2760_e4289_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__qd1, var_fn25_calc_iq__qd1_dn2, var_fn25_calc_iq__qd1_dn4, var_fn25_calc_iq__qd1_dn7, var_fn25_calc_iq__qd1_dn16, var_fn25_calc_iq__qd1_dn17,)
    }
};
        var_fn25_calc_iq__qd1 = assign2760_e4289;
        var_fn25_calc_iq__qd1_dn2 = assign2760_e4289_d_n2;
        var_fn25_calc_iq__qd1_dn4 = assign2760_e4289_d_n4;
        var_fn25_calc_iq__qd1_dn7 = assign2760_e4289_d_n7;
        var_fn25_calc_iq__qd1_dn16 = assign2760_e4289_d_n16;
        var_fn25_calc_iq__qd1_dn17 = assign2760_e4289_d_n17;
        var_fn25_calc_iq__qd1_rv = 0.0;

        let (assign2770_e4293, assign2770_e4293_d_n2, assign2770_e4293_d_n4, assign2770_e4293_d_n7, assign2770_e4293_d_n16, assign2770_e4293_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__qs, var_fn25_calc_iq__qs_dn2, var_fn25_calc_iq__qs_dn4, var_fn25_calc_iq__qs_dn7, var_fn25_calc_iq__qs_dn16, var_fn25_calc_iq__qs_dn17,)
    }
};
        var_fn25_calc_iq__qs = assign2770_e4293;
        var_fn25_calc_iq__qs_dn2 = assign2770_e4293_d_n2;
        var_fn25_calc_iq__qs_dn4 = assign2770_e4293_d_n4;
        var_fn25_calc_iq__qs_dn7 = assign2770_e4293_d_n7;
        var_fn25_calc_iq__qs_dn16 = assign2770_e4293_d_n16;
        var_fn25_calc_iq__qs_dn17 = assign2770_e4293_d_n17;
        var_fn25_calc_iq__qs_rv = 0.0;

        let (assign2780_e4297, assign2780_e4297_d_n2, assign2780_e4297_d_n4, assign2780_e4297_d_n7, assign2780_e4297_d_n16, assign2780_e4297_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__qd, var_fn25_calc_iq__qd_dn2, var_fn25_calc_iq__qd_dn4, var_fn25_calc_iq__qd_dn7, var_fn25_calc_iq__qd_dn16, var_fn25_calc_iq__qd_dn17,)
    }
};
        var_fn25_calc_iq__qd = assign2780_e4297;
        var_fn25_calc_iq__qd_dn2 = assign2780_e4297_d_n2;
        var_fn25_calc_iq__qd_dn4 = assign2780_e4297_d_n4;
        var_fn25_calc_iq__qd_dn7 = assign2780_e4297_d_n7;
        var_fn25_calc_iq__qd_dn16 = assign2780_e4297_d_n16;
        var_fn25_calc_iq__qd_dn17 = assign2780_e4297_d_n17;
        var_fn25_calc_iq__qd_rv = 0.0;

        let (assign2790_e4301, assign2790_e4301_d_n2, assign2790_e4301_d_n4, assign2790_e4301_d_n7, assign2790_e4301_d_n16,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__etac, var_fn25_calc_iq__etac_dn2, var_fn25_calc_iq__etac_dn4, var_fn25_calc_iq__etac_dn7, var_fn25_calc_iq__etac_dn16,)
    }
};
        var_fn25_calc_iq__etac = assign2790_e4301;
        var_fn25_calc_iq__etac_dn2 = assign2790_e4301_d_n2;
        var_fn25_calc_iq__etac_dn4 = assign2790_e4301_d_n4;
        var_fn25_calc_iq__etac_dn7 = assign2790_e4301_d_n7;
        var_fn25_calc_iq__etac_dn16 = assign2790_e4301_d_n16;
        var_fn25_calc_iq__etac_rv = 0.0;

        let (assign2800_e4305, assign2800_e4305_d_n3, assign2800_e4305_d_n4, assign2800_e4305_d_n16,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__etab, var_fn25_calc_iq__etab_dn3, var_fn25_calc_iq__etab_dn4, var_fn25_calc_iq__etab_dn16,)
    }
};
        var_fn25_calc_iq__etab = assign2800_e4305;
        var_fn25_calc_iq__etab_dn3 = assign2800_e4305_d_n3;
        var_fn25_calc_iq__etab_dn4 = assign2800_e4305_d_n4;
        var_fn25_calc_iq__etab_dn16 = assign2800_e4305_d_n16;
        var_fn25_calc_iq__etab_rv = 0.0;

        let (assign2810_e4309, assign2810_e4309_d_n2, assign2810_e4309_d_n4, assign2810_e4309_d_n7, assign2810_e4309_d_n16,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__etags, var_fn25_calc_iq__etags_dn2, var_fn25_calc_iq__etags_dn4, var_fn25_calc_iq__etags_dn7, var_fn25_calc_iq__etags_dn16,)
    }
};
        var_fn25_calc_iq__etags = assign2810_e4309;
        var_fn25_calc_iq__etags_dn2 = assign2810_e4309_d_n2;
        var_fn25_calc_iq__etags_dn4 = assign2810_e4309_d_n4;
        var_fn25_calc_iq__etags_dn7 = assign2810_e4309_d_n7;
        var_fn25_calc_iq__etags_dn16 = assign2810_e4309_d_n16;
        var_fn25_calc_iq__etags_rv = 0.0;

        let (assign2820_e4313, assign2820_e4313_d_n2, assign2820_e4313_d_n3, assign2820_e4313_d_n4, assign2820_e4313_d_n7, assign2820_e4313_d_n16, assign2820_e4313_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__exparg, var_fn25_calc_iq__exparg_dn2, var_fn25_calc_iq__exparg_dn3, var_fn25_calc_iq__exparg_dn4, var_fn25_calc_iq__exparg_dn7, var_fn25_calc_iq__exparg_dn16, var_fn25_calc_iq__exparg_dn17,)
    }
};
        var_fn25_calc_iq__exparg = assign2820_e4313;
        var_fn25_calc_iq__exparg_dn2 = assign2820_e4313_d_n2;
        var_fn25_calc_iq__exparg_dn3 = assign2820_e4313_d_n3;
        var_fn25_calc_iq__exparg_dn4 = assign2820_e4313_d_n4;
        var_fn25_calc_iq__exparg_dn7 = assign2820_e4313_d_n7;
        var_fn25_calc_iq__exparg_dn16 = assign2820_e4313_d_n16;
        var_fn25_calc_iq__exparg_dn17 = assign2820_e4313_d_n17;
        var_fn25_calc_iq__exparg_rv = 0.0;

        let (assign2830_e4317, assign2830_e4317_d_n2, assign2830_e4317_d_n3, assign2830_e4317_d_n4, assign2830_e4317_d_n7, assign2830_e4317_d_n16, assign2830_e4317_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__myarg, var_fn25_calc_iq__myarg_dn2, var_fn25_calc_iq__myarg_dn3, var_fn25_calc_iq__myarg_dn4, var_fn25_calc_iq__myarg_dn7, var_fn25_calc_iq__myarg_dn16, var_fn25_calc_iq__myarg_dn17,)
    }
};
        var_fn25_calc_iq__myarg = assign2830_e4317;
        var_fn25_calc_iq__myarg_dn2 = assign2830_e4317_d_n2;
        var_fn25_calc_iq__myarg_dn3 = assign2830_e4317_d_n3;
        var_fn25_calc_iq__myarg_dn4 = assign2830_e4317_d_n4;
        var_fn25_calc_iq__myarg_dn7 = assign2830_e4317_d_n7;
        var_fn25_calc_iq__myarg_dn16 = assign2830_e4317_d_n16;
        var_fn25_calc_iq__myarg_dn17 = assign2830_e4317_d_n17;
        var_fn25_calc_iq__myarg_rv = 0.0;

        let (assign2840_e4321, assign2840_e4321_d_n16, assign2840_e4321_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__absvdsin, var_fn25_calc_iq__absvdsin_dn16, var_fn25_calc_iq__absvdsin_dn17,)
    }
};
        var_fn25_calc_iq__absvdsin = assign2840_e4321;
        var_fn25_calc_iq__absvdsin_dn16 = assign2840_e4321_d_n16;
        var_fn25_calc_iq__absvdsin_dn17 = assign2840_e4321_d_n17;
        var_fn25_calc_iq__absvdsin_rv = 0.0;

        let (assign2850_e4325, assign2850_e4325_d_n2, assign2850_e4325_d_n7, assign2850_e4325_d_n16, assign2850_e4325_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__vgdin, var_fn25_calc_iq__vgdin_dn2, var_fn25_calc_iq__vgdin_dn7, var_fn25_calc_iq__vgdin_dn16, var_fn25_calc_iq__vgdin_dn17,)
    }
};
        var_fn25_calc_iq__vgdin = assign2850_e4325;
        var_fn25_calc_iq__vgdin_dn2 = assign2850_e4325_d_n2;
        var_fn25_calc_iq__vgdin_dn7 = assign2850_e4325_d_n7;
        var_fn25_calc_iq__vgdin_dn16 = assign2850_e4325_d_n16;
        var_fn25_calc_iq__vgdin_dn17 = assign2850_e4325_d_n17;
        var_fn25_calc_iq__vgdin_rv = 0.0;

        let (assign2860_e4329, assign2860_e4329_d_n2, assign2860_e4329_d_n4, assign2860_e4329_d_n7, assign2860_e4329_d_n16, assign2860_e4329_d_n17,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__exparg0, var_fn25_calc_iq__exparg0_dn2, var_fn25_calc_iq__exparg0_dn4, var_fn25_calc_iq__exparg0_dn7, var_fn25_calc_iq__exparg0_dn16, var_fn25_calc_iq__exparg0_dn17,)
    }
};
        var_fn25_calc_iq__exparg0 = assign2860_e4329;
        var_fn25_calc_iq__exparg0_dn2 = assign2860_e4329_d_n2;
        var_fn25_calc_iq__exparg0_dn4 = assign2860_e4329_d_n4;
        var_fn25_calc_iq__exparg0_dn7 = assign2860_e4329_d_n7;
        var_fn25_calc_iq__exparg0_dn16 = assign2860_e4329_d_n16;
        var_fn25_calc_iq__exparg0_dn17 = assign2860_e4329_d_n17;
        var_fn25_calc_iq__exparg0_rv = 0.0;

        let (assign2870_e4333, assign2870_e4333_d_n4,) = {
    if (var_guard24 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__myarg0, var_fn25_calc_iq__myarg0_dn4,)
    }
};
        var_fn25_calc_iq__myarg0 = assign2870_e4333;
        var_fn25_calc_iq__myarg0_dn4 = assign2870_e4333_d_n4;
        var_fn25_calc_iq__myarg0_rv = 0.0;

        let (assign2880_e4360, assign2880_e4360_d_n16, assign2880_e4360_d_n17,) = {
    if (var_guard24 != 0.0) {
        let (assign2880_e4358, assign2880_e4358_d_n16, assign2880_e4358_d_n17,) = {
            if (p.p52 != 0.0) {
                let assign2880_e4342: f64 = (0.001 / p.p53);
                let assign2880_e4344: f64 = (assign2880_e4342 * var_fn25_calc_iq__vdsin);
                let assign2880_e4345: f64 = (assign2880_e4344).tanh();
                let assign2880_e4346: f64 = (var_fn25_calc_iq__vdsin * assign2880_e4345);
                (assign2880_e4346, ((var_fn25_calc_iq__vdsin_dn16 * assign2880_e4345) + (var_fn25_calc_iq__vdsin * ((assign2880_e4342 * var_fn25_calc_iq__vdsin_dn16) / ((assign2880_e4344).cosh() * (assign2880_e4344).cosh())))), ((var_fn25_calc_iq__vdsin_dn17 * assign2880_e4345) + (var_fn25_calc_iq__vdsin * ((assign2880_e4342 * var_fn25_calc_iq__vdsin_dn17) / ((assign2880_e4344).cosh() * (assign2880_e4344).cosh())))),)
            } else {
                let (assign2880_e4357, assign2880_e4357_d_n16, assign2880_e4357_d_n17,) = {
                    if (p.p52 == 0.0) {
                        let assign2880_e4352: f64 = (var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsin);
                        let assign2880_e4354: f64 = (assign2880_e4352 + p.p53);
                        let assign2880_e4355: f64 = (assign2880_e4354).sqrt();
                        (assign2880_e4355, (((var_fn25_calc_iq__vdsin_dn16 * var_fn25_calc_iq__vdsin) + (var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsin_dn16)) / (2.0 * assign2880_e4355)), (((var_fn25_calc_iq__vdsin_dn17 * var_fn25_calc_iq__vdsin) + (var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsin_dn17)) / (2.0 * assign2880_e4355)),)
                    } else {
                        (0.0, 0.0, 0.0,)
                    }
                };
                (assign2880_e4357, assign2880_e4357_d_n16, assign2880_e4357_d_n17,)
            }
        };
        (assign2880_e4358, assign2880_e4358_d_n16, assign2880_e4358_d_n17,)
    } else {
        (var_fn25_calc_iq__absvdsin, var_fn25_calc_iq__absvdsin_dn16, var_fn25_calc_iq__absvdsin_dn17,)
    }
};
        var_fn25_calc_iq__absvdsin = assign2880_e4360;
        var_fn25_calc_iq__absvdsin_dn16 = assign2880_e4360_d_n16;
        var_fn25_calc_iq__absvdsin_dn17 = assign2880_e4360_d_n17;
        var_fn25_calc_iq__absvdsin_rv = 0.0;

        let (assign2890_e4366, assign2890_e4366_d_n2, assign2890_e4366_d_n7, assign2890_e4366_d_n16, assign2890_e4366_d_n17,) = {
    if (var_guard24 != 0.0) {
        let assign2890_e4364: f64 = (var_fn25_calc_iq__vgsin - var_fn25_calc_iq__vdsin);
        (assign2890_e4364, var_fn25_calc_iq__vgsin_dn2, var_fn25_calc_iq__vgsin_dn7, (var_fn25_calc_iq__vgsin_dn16 - var_fn25_calc_iq__vdsin_dn16), (-var_fn25_calc_iq__vdsin_dn17),)
    } else {
        (var_fn25_calc_iq__vgdin, var_fn25_calc_iq__vgdin_dn2, var_fn25_calc_iq__vgdin_dn7, var_fn25_calc_iq__vgdin_dn16, var_fn25_calc_iq__vgdin_dn17,)
    }
};
        var_fn25_calc_iq__vgdin = assign2890_e4366;
        var_fn25_calc_iq__vgdin_dn2 = assign2890_e4366_d_n2;
        var_fn25_calc_iq__vgdin_dn7 = assign2890_e4366_d_n7;
        var_fn25_calc_iq__vgdin_dn16 = assign2890_e4366_d_n16;
        var_fn25_calc_iq__vgdin_dn17 = assign2890_e4366_d_n17;
        var_fn25_calc_iq__vgdin_rv = 0.0;

        let (assign2900_e4372, assign2900_e4372_d_n4,) = {
    if (var_guard24 != 0.0) {
        let assign2900_e4370: f64 = (var_fn25_calc_iq__alpha * var_fn25_calc_iq__phitin);
        (assign2900_e4370, (var_fn25_calc_iq__alpha * var_fn25_calc_iq__phitin_dn4),)
    } else {
        (var_fn25_calc_iq__alpha_phit, var_fn25_calc_iq__alpha_phit_dn4,)
    }
};
        var_fn25_calc_iq__alpha_phit = assign2900_e4372;
        var_fn25_calc_iq__alpha_phit_dn4 = assign2900_e4372_d_n4;
        var_fn25_calc_iq__alpha_phit_rv = 0.0;

        let (assign2910_e4384, assign2910_e4384_d_n4, assign2910_e4384_d_n16, assign2910_e4384_d_n17,) = {
    if (var_guard24 != 0.0) {
        let assign2910_e4377: f64 = (2.302585092994046 * var_fn25_calc_iq__phitin);
        let assign2910_e4378: f64 = (var_fn25_calc_iq__ss / assign2910_e4377);
        let assign2910_e4381: f64 = (var_fn25_calc_iq__nd * var_fn25_calc_iq__absvdsin);
        let assign2910_e4382: f64 = (assign2910_e4378 + assign2910_e4381);
        (assign2910_e4382, (-((var_fn25_calc_iq__ss * (2.302585092994046 * var_fn25_calc_iq__phitin_dn4)) / (assign2910_e4377 * assign2910_e4377))), (var_fn25_calc_iq__nd * var_fn25_calc_iq__absvdsin_dn16), (var_fn25_calc_iq__nd * var_fn25_calc_iq__absvdsin_dn17),)
    } else {
        (var_fn25_calc_iq__n, var_fn25_calc_iq__n_dn4, var_fn25_calc_iq__n_dn16, var_fn25_calc_iq__n_dn17,)
    }
};
        var_fn25_calc_iq__n = assign2910_e4384;
        var_fn25_calc_iq__n_dn4 = assign2910_e4384_d_n4;
        var_fn25_calc_iq__n_dn16 = assign2910_e4384_d_n16;
        var_fn25_calc_iq__n_dn17 = assign2910_e4384_d_n17;
        var_fn25_calc_iq__n_rv = 0.0;

        let (assign2920_e4394, assign2920_e4394_d_n4,) = {
    if (var_guard24 != 0.0) {
        let assign2920_e4390: f64 = (var_fn25_calc_iq__tambin - var_fn25_calc_iq__tnomin);
        let assign2920_e4391: f64 = (var_fn25_calc_iq__vtzeta * assign2920_e4390);
        let assign2920_e4392: f64 = (var_fn25_calc_iq__vto + assign2920_e4391);
        (assign2920_e4392, (var_fn25_calc_iq__vtzeta * var_fn25_calc_iq__tambin_dn4),)
    } else {
        (var_fn25_calc_iq__vtof, var_fn25_calc_iq__vtof_dn4,)
    }
};
        var_fn25_calc_iq__vtof = assign2920_e4394;
        var_fn25_calc_iq__vtof_dn4 = assign2920_e4394_d_n4;
        var_fn25_calc_iq__vtof_rv = 0.0;

        let (assign2930_e4402, assign2930_e4402_d_n4,) = {
    if (var_guard24 != 0.0) {
        let assign2930_e4398: f64 = (var_fn25_calc_iq__tambin / var_fn25_calc_iq__tnomin);
        let assign2930_e4400: f64 = (assign2930_e4398).powf(var_fn25_calc_iq__epsilon);
        (assign2930_e4400, if 0.0 == 0.0 && ((var_fn25_calc_iq__epsilon) as f64).is_finite() && ((var_fn25_calc_iq__epsilon) as f64).fract() == 0.0 { if var_fn25_calc_iq__epsilon == 0.0 { 0.0 } else { (var_fn25_calc_iq__epsilon * ((assign2930_e4398).powf(var_fn25_calc_iq__epsilon - 1.0) * (var_fn25_calc_iq__tambin_dn4 / var_fn25_calc_iq__tnomin))) } } else { (assign2930_e4400 * (var_fn25_calc_iq__epsilon * ((var_fn25_calc_iq__tambin_dn4 / var_fn25_calc_iq__tnomin) / assign2930_e4398))) },)
    } else {
        (var_fn25_calc_iq__tfacmobin, var_fn25_calc_iq__tfacmobin_dn4,)
    }
};
        var_fn25_calc_iq__tfacmobin = assign2930_e4402;
        var_fn25_calc_iq__tfacmobin_dn4 = assign2930_e4402_d_n4;
        var_fn25_calc_iq__tfacmobin_rv = 0.0;

        let assign2940_e4405: f64 = if var_fn25_calc_iq__dibsat != 0.0 { 1.0 } else { 0.0 };
        var_guard26 = assign2940_e4405;
        var_guard26_rv = 0.0;

        let (assign2950_e4423, assign2950_e4423_d_n16, assign2950_e4423_d_n17,) = {
    if ((var_guard24 != 0.0) && (var_guard26 != 0.0)) {
        let assign2950_e4413: f64 = (var_fn25_calc_iq__absvdsin / var_fn25_calc_iq__dibsat);
        let assign2950_e4415: f64 = (assign2950_e4413).powf(var_fn25_calc_iq__beta);
        let assign2950_e4416: f64 = (1.0 + assign2950_e4415);
        let assign2950_e4419: f64 = (1.0 / var_fn25_calc_iq__beta);
        let assign2950_e4420: f64 = (assign2950_e4416).powf(assign2950_e4419);
        let assign2950_e4421: f64 = (var_fn25_calc_iq__absvdsin / assign2950_e4420);
        (assign2950_e4421, (((var_fn25_calc_iq__absvdsin_dn16 * assign2950_e4420) - (var_fn25_calc_iq__absvdsin * if 0.0 == 0.0 && ((assign2950_e4419) as f64).is_finite() && ((assign2950_e4419) as f64).fract() == 0.0 { if assign2950_e4419 == 0.0 { 0.0 } else { (assign2950_e4419 * ((assign2950_e4416).powf(assign2950_e4419 - 1.0) * if 0.0 == 0.0 && ((var_fn25_calc_iq__beta) as f64).is_finite() && ((var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (var_fn25_calc_iq__beta * ((assign2950_e4413).powf(var_fn25_calc_iq__beta - 1.0) * (var_fn25_calc_iq__absvdsin_dn16 / var_fn25_calc_iq__dibsat))) } } else { (assign2950_e4415 * (var_fn25_calc_iq__beta * ((var_fn25_calc_iq__absvdsin_dn16 / var_fn25_calc_iq__dibsat) / assign2950_e4413))) })) } } else { (assign2950_e4420 * (assign2950_e4419 * (if 0.0 == 0.0 && ((var_fn25_calc_iq__beta) as f64).is_finite() && ((var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (var_fn25_calc_iq__beta * ((assign2950_e4413).powf(var_fn25_calc_iq__beta - 1.0) * (var_fn25_calc_iq__absvdsin_dn16 / var_fn25_calc_iq__dibsat))) } } else { (assign2950_e4415 * (var_fn25_calc_iq__beta * ((var_fn25_calc_iq__absvdsin_dn16 / var_fn25_calc_iq__dibsat) / assign2950_e4413))) } / assign2950_e4416))) })) / (assign2950_e4420 * assign2950_e4420)), (((var_fn25_calc_iq__absvdsin_dn17 * assign2950_e4420) - (var_fn25_calc_iq__absvdsin * if 0.0 == 0.0 && ((assign2950_e4419) as f64).is_finite() && ((assign2950_e4419) as f64).fract() == 0.0 { if assign2950_e4419 == 0.0 { 0.0 } else { (assign2950_e4419 * ((assign2950_e4416).powf(assign2950_e4419 - 1.0) * if 0.0 == 0.0 && ((var_fn25_calc_iq__beta) as f64).is_finite() && ((var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (var_fn25_calc_iq__beta * ((assign2950_e4413).powf(var_fn25_calc_iq__beta - 1.0) * (var_fn25_calc_iq__absvdsin_dn17 / var_fn25_calc_iq__dibsat))) } } else { (assign2950_e4415 * (var_fn25_calc_iq__beta * ((var_fn25_calc_iq__absvdsin_dn17 / var_fn25_calc_iq__dibsat) / assign2950_e4413))) })) } } else { (assign2950_e4420 * (assign2950_e4419 * (if 0.0 == 0.0 && ((var_fn25_calc_iq__beta) as f64).is_finite() && ((var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (var_fn25_calc_iq__beta * ((assign2950_e4413).powf(var_fn25_calc_iq__beta - 1.0) * (var_fn25_calc_iq__absvdsin_dn17 / var_fn25_calc_iq__dibsat))) } } else { (assign2950_e4415 * (var_fn25_calc_iq__beta * ((var_fn25_calc_iq__absvdsin_dn17 / var_fn25_calc_iq__dibsat) / assign2950_e4413))) } / assign2950_e4416))) })) / (assign2950_e4420 * assign2950_e4420)),)
    } else {
        (var_fn25_calc_iq__vsatdibl, var_fn25_calc_iq__vsatdibl_dn16, var_fn25_calc_iq__vsatdibl_dn17,)
    }
};
        var_fn25_calc_iq__vsatdibl = assign2950_e4423;
        var_fn25_calc_iq__vsatdibl_dn16 = assign2950_e4423_d_n16;
        var_fn25_calc_iq__vsatdibl_dn17 = assign2950_e4423_d_n17;
        var_fn25_calc_iq__vsatdibl_rv = 0.0;

        let (assign2960_e4430, assign2960_e4430_d_n16, assign2960_e4430_d_n17,) = {
    if ((var_guard24 != 0.0) && (var_guard26 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__vsatdibl, var_fn25_calc_iq__vsatdibl_dn16, var_fn25_calc_iq__vsatdibl_dn17,)
    }
};
        var_fn25_calc_iq__vsatdibl = assign2960_e4430;
        var_fn25_calc_iq__vsatdibl_dn16 = assign2960_e4430_d_n16;
        var_fn25_calc_iq__vsatdibl_dn17 = assign2960_e4430_d_n17;
        var_fn25_calc_iq__vsatdibl_rv = 0.0;

        *var_fn25_calc_iq__absvdsin_slot = var_fn25_calc_iq__absvdsin;
        *var_fn25_calc_iq__absvdsin_dn16_slot = var_fn25_calc_iq__absvdsin_dn16;
        *var_fn25_calc_iq__absvdsin_dn17_slot = var_fn25_calc_iq__absvdsin_dn17;
        *var_fn25_calc_iq__absvdsin_rv_slot = var_fn25_calc_iq__absvdsin_rv;
        *var_fn25_calc_iq__alpha_phit_slot = var_fn25_calc_iq__alpha_phit;
        *var_fn25_calc_iq__alpha_phit_dn4_slot = var_fn25_calc_iq__alpha_phit_dn4;
        *var_fn25_calc_iq__alpha_phit_rv_slot = var_fn25_calc_iq__alpha_phit_rv;
        *var_fn25_calc_iq__etab_slot = var_fn25_calc_iq__etab;
        *var_fn25_calc_iq__etab_dn16_slot = var_fn25_calc_iq__etab_dn16;
        *var_fn25_calc_iq__etab_dn3_slot = var_fn25_calc_iq__etab_dn3;
        *var_fn25_calc_iq__etab_dn4_slot = var_fn25_calc_iq__etab_dn4;
        *var_fn25_calc_iq__etab_rv_slot = var_fn25_calc_iq__etab_rv;
        *var_fn25_calc_iq__etac_slot = var_fn25_calc_iq__etac;
        *var_fn25_calc_iq__etac_dn16_slot = var_fn25_calc_iq__etac_dn16;
        *var_fn25_calc_iq__etac_dn2_slot = var_fn25_calc_iq__etac_dn2;
        *var_fn25_calc_iq__etac_dn4_slot = var_fn25_calc_iq__etac_dn4;
        *var_fn25_calc_iq__etac_dn7_slot = var_fn25_calc_iq__etac_dn7;
        *var_fn25_calc_iq__etac_rv_slot = var_fn25_calc_iq__etac_rv;
        *var_fn25_calc_iq__etad0_slot = var_fn25_calc_iq__etad0;
        *var_fn25_calc_iq__etad0_dn16_slot = var_fn25_calc_iq__etad0_dn16;
        *var_fn25_calc_iq__etad0_dn17_slot = var_fn25_calc_iq__etad0_dn17;
        *var_fn25_calc_iq__etad0_dn2_slot = var_fn25_calc_iq__etad0_dn2;
        *var_fn25_calc_iq__etad0_dn4_slot = var_fn25_calc_iq__etad0_dn4;
        *var_fn25_calc_iq__etad0_dn7_slot = var_fn25_calc_iq__etad0_dn7;
        *var_fn25_calc_iq__etad0_rv_slot = var_fn25_calc_iq__etad0_rv;
        *var_fn25_calc_iq__etags_slot = var_fn25_calc_iq__etags;
        *var_fn25_calc_iq__etags_dn16_slot = var_fn25_calc_iq__etags_dn16;
        *var_fn25_calc_iq__etags_dn2_slot = var_fn25_calc_iq__etags_dn2;
        *var_fn25_calc_iq__etags_dn4_slot = var_fn25_calc_iq__etags_dn4;
        *var_fn25_calc_iq__etags_dn7_slot = var_fn25_calc_iq__etags_dn7;
        *var_fn25_calc_iq__etags_rv_slot = var_fn25_calc_iq__etags_rv;
        *var_fn25_calc_iq__exparg_slot = var_fn25_calc_iq__exparg;
        *var_fn25_calc_iq__exparg0_slot = var_fn25_calc_iq__exparg0;
        *var_fn25_calc_iq__exparg0_dn16_slot = var_fn25_calc_iq__exparg0_dn16;
        *var_fn25_calc_iq__exparg0_dn17_slot = var_fn25_calc_iq__exparg0_dn17;
        *var_fn25_calc_iq__exparg0_dn2_slot = var_fn25_calc_iq__exparg0_dn2;
        *var_fn25_calc_iq__exparg0_dn4_slot = var_fn25_calc_iq__exparg0_dn4;
        *var_fn25_calc_iq__exparg0_dn7_slot = var_fn25_calc_iq__exparg0_dn7;
        *var_fn25_calc_iq__exparg0_rv_slot = var_fn25_calc_iq__exparg0_rv;
        *var_fn25_calc_iq__exparg_dn16_slot = var_fn25_calc_iq__exparg_dn16;
        *var_fn25_calc_iq__exparg_dn17_slot = var_fn25_calc_iq__exparg_dn17;
        *var_fn25_calc_iq__exparg_dn2_slot = var_fn25_calc_iq__exparg_dn2;
        *var_fn25_calc_iq__exparg_dn3_slot = var_fn25_calc_iq__exparg_dn3;
        *var_fn25_calc_iq__exparg_dn4_slot = var_fn25_calc_iq__exparg_dn4;
        *var_fn25_calc_iq__exparg_dn7_slot = var_fn25_calc_iq__exparg_dn7;
        *var_fn25_calc_iq__exparg_rv_slot = var_fn25_calc_iq__exparg_rv;
        *var_fn25_calc_iq__fds0_slot = var_fn25_calc_iq__fds0;
        *var_fn25_calc_iq__fds0_dn16_slot = var_fn25_calc_iq__fds0_dn16;
        *var_fn25_calc_iq__fds0_dn17_slot = var_fn25_calc_iq__fds0_dn17;
        *var_fn25_calc_iq__fds0_dn2_slot = var_fn25_calc_iq__fds0_dn2;
        *var_fn25_calc_iq__fds0_dn4_slot = var_fn25_calc_iq__fds0_dn4;
        *var_fn25_calc_iq__fds0_dn7_slot = var_fn25_calc_iq__fds0_dn7;
        *var_fn25_calc_iq__fds0_rv_slot = var_fn25_calc_iq__fds0_rv;
        *var_fn25_calc_iq__ffd0_slot = var_fn25_calc_iq__ffd0;
        *var_fn25_calc_iq__ffd0_dn16_slot = var_fn25_calc_iq__ffd0_dn16;
        *var_fn25_calc_iq__ffd0_dn17_slot = var_fn25_calc_iq__ffd0_dn17;
        *var_fn25_calc_iq__ffd0_dn2_slot = var_fn25_calc_iq__ffd0_dn2;
        *var_fn25_calc_iq__ffd0_dn4_slot = var_fn25_calc_iq__ffd0_dn4;
        *var_fn25_calc_iq__ffd0_dn7_slot = var_fn25_calc_iq__ffd0_dn7;
        *var_fn25_calc_iq__ffd0_rv_slot = var_fn25_calc_iq__ffd0_rv;
        *var_fn25_calc_iq__fsd0_slot = var_fn25_calc_iq__fsd0;
        *var_fn25_calc_iq__fsd0_dn16_slot = var_fn25_calc_iq__fsd0_dn16;
        *var_fn25_calc_iq__fsd0_dn17_slot = var_fn25_calc_iq__fsd0_dn17;
        *var_fn25_calc_iq__fsd0_dn2_slot = var_fn25_calc_iq__fsd0_dn2;
        *var_fn25_calc_iq__fsd0_dn4_slot = var_fn25_calc_iq__fsd0_dn4;
        *var_fn25_calc_iq__fsd0_dn7_slot = var_fn25_calc_iq__fsd0_dn7;
        *var_fn25_calc_iq__fsd0_rv_slot = var_fn25_calc_iq__fsd0_rv;
        *var_fn25_calc_iq__myarg_slot = var_fn25_calc_iq__myarg;
        *var_fn25_calc_iq__myarg0_slot = var_fn25_calc_iq__myarg0;
        *var_fn25_calc_iq__myarg0_dn4_slot = var_fn25_calc_iq__myarg0_dn4;
        *var_fn25_calc_iq__myarg0_rv_slot = var_fn25_calc_iq__myarg0_rv;
        *var_fn25_calc_iq__myarg_dn16_slot = var_fn25_calc_iq__myarg_dn16;
        *var_fn25_calc_iq__myarg_dn17_slot = var_fn25_calc_iq__myarg_dn17;
        *var_fn25_calc_iq__myarg_dn2_slot = var_fn25_calc_iq__myarg_dn2;
        *var_fn25_calc_iq__myarg_dn3_slot = var_fn25_calc_iq__myarg_dn3;
        *var_fn25_calc_iq__myarg_dn4_slot = var_fn25_calc_iq__myarg_dn4;
        *var_fn25_calc_iq__myarg_dn7_slot = var_fn25_calc_iq__myarg_dn7;
        *var_fn25_calc_iq__myarg_rv_slot = var_fn25_calc_iq__myarg_rv;
        *var_fn25_calc_iq__n_slot = var_fn25_calc_iq__n;
        *var_fn25_calc_iq__n_dn16_slot = var_fn25_calc_iq__n_dn16;
        *var_fn25_calc_iq__n_dn17_slot = var_fn25_calc_iq__n_dn17;
        *var_fn25_calc_iq__n_dn4_slot = var_fn25_calc_iq__n_dn4;
        *var_fn25_calc_iq__n_rv_slot = var_fn25_calc_iq__n_rv;
        *var_fn25_calc_iq__qd_slot = var_fn25_calc_iq__qd;
        *var_fn25_calc_iq__qd1_slot = var_fn25_calc_iq__qd1;
        *var_fn25_calc_iq__qd1_dn16_slot = var_fn25_calc_iq__qd1_dn16;
        *var_fn25_calc_iq__qd1_dn17_slot = var_fn25_calc_iq__qd1_dn17;
        *var_fn25_calc_iq__qd1_dn2_slot = var_fn25_calc_iq__qd1_dn2;
        *var_fn25_calc_iq__qd1_dn4_slot = var_fn25_calc_iq__qd1_dn4;
        *var_fn25_calc_iq__qd1_dn7_slot = var_fn25_calc_iq__qd1_dn7;
        *var_fn25_calc_iq__qd1_rv_slot = var_fn25_calc_iq__qd1_rv;
        *var_fn25_calc_iq__qd2_slot = var_fn25_calc_iq__qd2;
        *var_fn25_calc_iq__qd2_dn16_slot = var_fn25_calc_iq__qd2_dn16;
        *var_fn25_calc_iq__qd2_dn17_slot = var_fn25_calc_iq__qd2_dn17;
        *var_fn25_calc_iq__qd2_dn2_slot = var_fn25_calc_iq__qd2_dn2;
        *var_fn25_calc_iq__qd2_dn4_slot = var_fn25_calc_iq__qd2_dn4;
        *var_fn25_calc_iq__qd2_dn7_slot = var_fn25_calc_iq__qd2_dn7;
        *var_fn25_calc_iq__qd2_rv_slot = var_fn25_calc_iq__qd2_rv;
        *var_fn25_calc_iq__qd3_slot = var_fn25_calc_iq__qd3;
        *var_fn25_calc_iq__qd3_dn16_slot = var_fn25_calc_iq__qd3_dn16;
        *var_fn25_calc_iq__qd3_dn17_slot = var_fn25_calc_iq__qd3_dn17;
        *var_fn25_calc_iq__qd3_dn2_slot = var_fn25_calc_iq__qd3_dn2;
        *var_fn25_calc_iq__qd3_dn4_slot = var_fn25_calc_iq__qd3_dn4;
        *var_fn25_calc_iq__qd3_dn7_slot = var_fn25_calc_iq__qd3_dn7;
        *var_fn25_calc_iq__qd3_rv_slot = var_fn25_calc_iq__qd3_rv;
        *var_fn25_calc_iq__qd_dn16_slot = var_fn25_calc_iq__qd_dn16;
        *var_fn25_calc_iq__qd_dn17_slot = var_fn25_calc_iq__qd_dn17;
        *var_fn25_calc_iq__qd_dn2_slot = var_fn25_calc_iq__qd_dn2;
        *var_fn25_calc_iq__qd_dn4_slot = var_fn25_calc_iq__qd_dn4;
        *var_fn25_calc_iq__qd_dn7_slot = var_fn25_calc_iq__qd_dn7;
        *var_fn25_calc_iq__qd_rv_slot = var_fn25_calc_iq__qd_rv;
        *var_fn25_calc_iq__qinvd0_slot = var_fn25_calc_iq__qinvd0;
        *var_fn25_calc_iq__qinvd0_dn16_slot = var_fn25_calc_iq__qinvd0_dn16;
        *var_fn25_calc_iq__qinvd0_dn17_slot = var_fn25_calc_iq__qinvd0_dn17;
        *var_fn25_calc_iq__qinvd0_dn2_slot = var_fn25_calc_iq__qinvd0_dn2;
        *var_fn25_calc_iq__qinvd0_dn4_slot = var_fn25_calc_iq__qinvd0_dn4;
        *var_fn25_calc_iq__qinvd0_dn7_slot = var_fn25_calc_iq__qinvd0_dn7;
        *var_fn25_calc_iq__qinvd0_rv_slot = var_fn25_calc_iq__qinvd0_rv;
        *var_fn25_calc_iq__qinvdd_slot = var_fn25_calc_iq__qinvdd;
        *var_fn25_calc_iq__qinvdd_dn16_slot = var_fn25_calc_iq__qinvdd_dn16;
        *var_fn25_calc_iq__qinvdd_dn17_slot = var_fn25_calc_iq__qinvdd_dn17;
        *var_fn25_calc_iq__qinvdd_dn2_slot = var_fn25_calc_iq__qinvdd_dn2;
        *var_fn25_calc_iq__qinvdd_dn4_slot = var_fn25_calc_iq__qinvdd_dn4;
        *var_fn25_calc_iq__qinvdd_dn7_slot = var_fn25_calc_iq__qinvdd_dn7;
        *var_fn25_calc_iq__qinvdd_rv_slot = var_fn25_calc_iq__qinvdd_rv;
        *var_fn25_calc_iq__qs_slot = var_fn25_calc_iq__qs;
        *var_fn25_calc_iq__qs2_slot = var_fn25_calc_iq__qs2;
        *var_fn25_calc_iq__qs2_dn16_slot = var_fn25_calc_iq__qs2_dn16;
        *var_fn25_calc_iq__qs2_dn17_slot = var_fn25_calc_iq__qs2_dn17;
        *var_fn25_calc_iq__qs2_dn2_slot = var_fn25_calc_iq__qs2_dn2;
        *var_fn25_calc_iq__qs2_dn4_slot = var_fn25_calc_iq__qs2_dn4;
        *var_fn25_calc_iq__qs2_dn7_slot = var_fn25_calc_iq__qs2_dn7;
        *var_fn25_calc_iq__qs2_rv_slot = var_fn25_calc_iq__qs2_rv;
        *var_fn25_calc_iq__qs3_slot = var_fn25_calc_iq__qs3;
        *var_fn25_calc_iq__qs3_dn16_slot = var_fn25_calc_iq__qs3_dn16;
        *var_fn25_calc_iq__qs3_dn17_slot = var_fn25_calc_iq__qs3_dn17;
        *var_fn25_calc_iq__qs3_dn2_slot = var_fn25_calc_iq__qs3_dn2;
        *var_fn25_calc_iq__qs3_dn4_slot = var_fn25_calc_iq__qs3_dn4;
        *var_fn25_calc_iq__qs3_dn7_slot = var_fn25_calc_iq__qs3_dn7;
        *var_fn25_calc_iq__qs3_rv_slot = var_fn25_calc_iq__qs3_rv;
        *var_fn25_calc_iq__qs_dn16_slot = var_fn25_calc_iq__qs_dn16;
        *var_fn25_calc_iq__qs_dn17_slot = var_fn25_calc_iq__qs_dn17;
        *var_fn25_calc_iq__qs_dn2_slot = var_fn25_calc_iq__qs_dn2;
        *var_fn25_calc_iq__qs_dn4_slot = var_fn25_calc_iq__qs_dn4;
        *var_fn25_calc_iq__qs_dn7_slot = var_fn25_calc_iq__qs_dn7;
        *var_fn25_calc_iq__qs_rv_slot = var_fn25_calc_iq__qs_rv;
        *var_fn25_calc_iq__qsqd_slot = var_fn25_calc_iq__qsqd;
        *var_fn25_calc_iq__qsqd_dn16_slot = var_fn25_calc_iq__qsqd_dn16;
        *var_fn25_calc_iq__qsqd_dn17_slot = var_fn25_calc_iq__qsqd_dn17;
        *var_fn25_calc_iq__qsqd_dn2_slot = var_fn25_calc_iq__qsqd_dn2;
        *var_fn25_calc_iq__qsqd_dn4_slot = var_fn25_calc_iq__qsqd_dn4;
        *var_fn25_calc_iq__qsqd_dn7_slot = var_fn25_calc_iq__qsqd_dn7;
        *var_fn25_calc_iq__qsqd_rv_slot = var_fn25_calc_iq__qsqd_rv;
        *var_fn25_calc_iq__tfacmobin_slot = var_fn25_calc_iq__tfacmobin;
        *var_fn25_calc_iq__tfacmobin_dn4_slot = var_fn25_calc_iq__tfacmobin_dn4;
        *var_fn25_calc_iq__tfacmobin_rv_slot = var_fn25_calc_iq__tfacmobin_rv;
        *var_fn25_calc_iq__vdx0_slot = var_fn25_calc_iq__vdx0;
        *var_fn25_calc_iq__vdx0_dn16_slot = var_fn25_calc_iq__vdx0_dn16;
        *var_fn25_calc_iq__vdx0_dn17_slot = var_fn25_calc_iq__vdx0_dn17;
        *var_fn25_calc_iq__vdx0_dn2_slot = var_fn25_calc_iq__vdx0_dn2;
        *var_fn25_calc_iq__vdx0_dn4_slot = var_fn25_calc_iq__vdx0_dn4;
        *var_fn25_calc_iq__vdx0_dn7_slot = var_fn25_calc_iq__vdx0_dn7;
        *var_fn25_calc_iq__vdx0_rv_slot = var_fn25_calc_iq__vdx0_rv;
        *var_fn25_calc_iq__vgdin_slot = var_fn25_calc_iq__vgdin;
        *var_fn25_calc_iq__vgdin_dn16_slot = var_fn25_calc_iq__vgdin_dn16;
        *var_fn25_calc_iq__vgdin_dn17_slot = var_fn25_calc_iq__vgdin_dn17;
        *var_fn25_calc_iq__vgdin_dn2_slot = var_fn25_calc_iq__vgdin_dn2;
        *var_fn25_calc_iq__vgdin_dn7_slot = var_fn25_calc_iq__vgdin_dn7;
        *var_fn25_calc_iq__vgdin_rv_slot = var_fn25_calc_iq__vgdin_rv;
        *var_fn25_calc_iq__vsatdibl_slot = var_fn25_calc_iq__vsatdibl;
        *var_fn25_calc_iq__vsatdibl_dn16_slot = var_fn25_calc_iq__vsatdibl_dn16;
        *var_fn25_calc_iq__vsatdibl_dn17_slot = var_fn25_calc_iq__vsatdibl_dn17;
        *var_fn25_calc_iq__vsatdibl_rv_slot = var_fn25_calc_iq__vsatdibl_rv;
        *var_fn25_calc_iq__vsx0_slot = var_fn25_calc_iq__vsx0;
        *var_fn25_calc_iq__vsx0_dn16_slot = var_fn25_calc_iq__vsx0_dn16;
        *var_fn25_calc_iq__vsx0_dn17_slot = var_fn25_calc_iq__vsx0_dn17;
        *var_fn25_calc_iq__vsx0_dn2_slot = var_fn25_calc_iq__vsx0_dn2;
        *var_fn25_calc_iq__vsx0_dn4_slot = var_fn25_calc_iq__vsx0_dn4;
        *var_fn25_calc_iq__vsx0_dn7_slot = var_fn25_calc_iq__vsx0_dn7;
        *var_fn25_calc_iq__vsx0_rv_slot = var_fn25_calc_iq__vsx0_rv;
        *var_fn25_calc_iq__vtof_slot = var_fn25_calc_iq__vtof;
        *var_fn25_calc_iq__vtof_dn4_slot = var_fn25_calc_iq__vtof_dn4;
        *var_fn25_calc_iq__vtof_rv_slot = var_fn25_calc_iq__vtof_rv;
        *var_guard26_slot = var_guard26;
        *var_guard26_rv_slot = var_guard26_rv;
    }

    pub(super) fn stamp_reactive_block_6(
        p: &Parameters,
        var_fn25_calc_iq__absvdsin: f64,
        var_fn25_calc_iq__absvdsin_dn16: f64,
        var_fn25_calc_iq__absvdsin_dn17: f64,
        var_fn25_calc_iq__alpha_phit: f64,
        var_fn25_calc_iq__alpha_phit_dn4: f64,
        var_fn25_calc_iq__beta: f64,
        var_fn25_calc_iq__cgin: f64,
        var_fn25_calc_iq__cgin_dn4: f64,
        var_fn25_calc_iq__delta1: f64,
        var_fn25_calc_iq__delta2: f64,
        var_fn25_calc_iq__lambda: f64,
        var_fn25_calc_iq__lin: f64,
        var_fn25_calc_iq__mtheta: f64,
        var_fn25_calc_iq__mu0: f64,
        var_fn25_calc_iq__n: f64,
        var_fn25_calc_iq__n_dn16: f64,
        var_fn25_calc_iq__n_dn17: f64,
        var_fn25_calc_iq__n_dn4: f64,
        var_fn25_calc_iq__phitin: f64,
        var_fn25_calc_iq__phitin_dn4: f64,
        var_fn25_calc_iq__tambin: f64,
        var_fn25_calc_iq__tambin_dn4: f64,
        var_fn25_calc_iq__tfacmobin: f64,
        var_fn25_calc_iq__tfacmobin_dn4: f64,
        var_fn25_calc_iq__tnomin: f64,
        var_fn25_calc_iq__vdsin: f64,
        var_fn25_calc_iq__vdsin_dn16: f64,
        var_fn25_calc_iq__vdsin_dn17: f64,
        var_fn25_calc_iq__vel0: f64,
        var_fn25_calc_iq__vgdin: f64,
        var_fn25_calc_iq__vgdin_dn16: f64,
        var_fn25_calc_iq__vgdin_dn17: f64,
        var_fn25_calc_iq__vgdin_dn2: f64,
        var_fn25_calc_iq__vgdin_dn7: f64,
        var_fn25_calc_iq__vgsin: f64,
        var_fn25_calc_iq__vgsin_dn16: f64,
        var_fn25_calc_iq__vgsin_dn2: f64,
        var_fn25_calc_iq__vgsin_dn7: f64,
        var_fn25_calc_iq__vsatdibl: f64,
        var_fn25_calc_iq__vsatdibl_dn16: f64,
        var_fn25_calc_iq__vsatdibl_dn17: f64,
        var_fn25_calc_iq__vtheta: f64,
        var_fn25_calc_iq__vtof: f64,
        var_fn25_calc_iq__vtof_dn4: f64,
        var_fn25_calc_iq__vzeta: f64,
        var_guard24: f64,
        var_fn25_calc_iq__delta_slot: &mut f64,
        var_fn25_calc_iq__delta_dn16_slot: &mut f64,
        var_fn25_calc_iq__delta_dn17_slot: &mut f64,
        var_fn25_calc_iq__delta_rv_slot: &mut f64,
        var_fn25_calc_iq__eta_slot: &mut f64,
        var_fn25_calc_iq__eta_dn16_slot: &mut f64,
        var_fn25_calc_iq__eta_dn17_slot: &mut f64,
        var_fn25_calc_iq__eta_dn2_slot: &mut f64,
        var_fn25_calc_iq__eta_dn3_slot: &mut f64,
        var_fn25_calc_iq__eta_dn4_slot: &mut f64,
        var_fn25_calc_iq__eta_dn7_slot: &mut f64,
        var_fn25_calc_iq__eta_rv_slot: &mut f64,
        var_fn25_calc_iq__exparg_slot: &mut f64,
        var_fn25_calc_iq__exparg_dn16_slot: &mut f64,
        var_fn25_calc_iq__exparg_dn17_slot: &mut f64,
        var_fn25_calc_iq__exparg_dn2_slot: &mut f64,
        var_fn25_calc_iq__exparg_dn3_slot: &mut f64,
        var_fn25_calc_iq__exparg_dn4_slot: &mut f64,
        var_fn25_calc_iq__exparg_dn7_slot: &mut f64,
        var_fn25_calc_iq__exparg_rv_slot: &mut f64,
        var_fn25_calc_iq__ff_slot: &mut f64,
        var_fn25_calc_iq__ff_dn16_slot: &mut f64,
        var_fn25_calc_iq__ff_dn17_slot: &mut f64,
        var_fn25_calc_iq__ff_dn2_slot: &mut f64,
        var_fn25_calc_iq__ff_dn3_slot: &mut f64,
        var_fn25_calc_iq__ff_dn4_slot: &mut f64,
        var_fn25_calc_iq__ff_dn7_slot: &mut f64,
        var_fn25_calc_iq__ff_rv_slot: &mut f64,
        var_fn25_calc_iq__fsd_slot: &mut f64,
        var_fn25_calc_iq__fsd_dn16_slot: &mut f64,
        var_fn25_calc_iq__fsd_dn17_slot: &mut f64,
        var_fn25_calc_iq__fsd_dn2_slot: &mut f64,
        var_fn25_calc_iq__fsd_dn3_slot: &mut f64,
        var_fn25_calc_iq__fsd_dn4_slot: &mut f64,
        var_fn25_calc_iq__fsd_dn7_slot: &mut f64,
        var_fn25_calc_iq__fsd_rv_slot: &mut f64,
        var_fn25_calc_iq__muf_slot: &mut f64,
        var_fn25_calc_iq__muf_dn16_slot: &mut f64,
        var_fn25_calc_iq__muf_dn17_slot: &mut f64,
        var_fn25_calc_iq__muf_dn2_slot: &mut f64,
        var_fn25_calc_iq__muf_dn3_slot: &mut f64,
        var_fn25_calc_iq__muf_dn4_slot: &mut f64,
        var_fn25_calc_iq__muf_dn7_slot: &mut f64,
        var_fn25_calc_iq__muf_rv_slot: &mut f64,
        var_fn25_calc_iq__myarg_slot: &mut f64,
        var_fn25_calc_iq__myarg_dn16_slot: &mut f64,
        var_fn25_calc_iq__myarg_dn17_slot: &mut f64,
        var_fn25_calc_iq__myarg_dn2_slot: &mut f64,
        var_fn25_calc_iq__myarg_dn3_slot: &mut f64,
        var_fn25_calc_iq__myarg_dn4_slot: &mut f64,
        var_fn25_calc_iq__myarg_dn7_slot: &mut f64,
        var_fn25_calc_iq__myarg_rv_slot: &mut f64,
        var_fn25_calc_iq__qinvv_slot: &mut f64,
        var_fn25_calc_iq__qinvv_dn16_slot: &mut f64,
        var_fn25_calc_iq__qinvv_dn17_slot: &mut f64,
        var_fn25_calc_iq__qinvv_dn2_slot: &mut f64,
        var_fn25_calc_iq__qinvv_dn3_slot: &mut f64,
        var_fn25_calc_iq__qinvv_dn4_slot: &mut f64,
        var_fn25_calc_iq__qinvv_dn7_slot: &mut f64,
        var_fn25_calc_iq__qinvv_rv_slot: &mut f64,
        var_fn25_calc_iq__qref_slot: &mut f64,
        var_fn25_calc_iq__qref_dn16_slot: &mut f64,
        var_fn25_calc_iq__qref_dn17_slot: &mut f64,
        var_fn25_calc_iq__qref_dn4_slot: &mut f64,
        var_fn25_calc_iq__qref_rv_slot: &mut f64,
        var_fn25_calc_iq__two_n_phit_slot: &mut f64,
        var_fn25_calc_iq__two_n_phit_dn16_slot: &mut f64,
        var_fn25_calc_iq__two_n_phit_dn17_slot: &mut f64,
        var_fn25_calc_iq__two_n_phit_dn4_slot: &mut f64,
        var_fn25_calc_iq__two_n_phit_rv_slot: &mut f64,
        var_fn25_calc_iq__vdsat_slot: &mut f64,
        var_fn25_calc_iq__vdsat1_slot: &mut f64,
        var_fn25_calc_iq__vdsat1_dn16_slot: &mut f64,
        var_fn25_calc_iq__vdsat1_dn17_slot: &mut f64,
        var_fn25_calc_iq__vdsat1_dn2_slot: &mut f64,
        var_fn25_calc_iq__vdsat1_dn3_slot: &mut f64,
        var_fn25_calc_iq__vdsat1_dn4_slot: &mut f64,
        var_fn25_calc_iq__vdsat1_dn7_slot: &mut f64,
        var_fn25_calc_iq__vdsat1_rv_slot: &mut f64,
        var_fn25_calc_iq__vdsat_dn16_slot: &mut f64,
        var_fn25_calc_iq__vdsat_dn17_slot: &mut f64,
        var_fn25_calc_iq__vdsat_dn2_slot: &mut f64,
        var_fn25_calc_iq__vdsat_dn3_slot: &mut f64,
        var_fn25_calc_iq__vdsat_dn4_slot: &mut f64,
        var_fn25_calc_iq__vdsat_dn7_slot: &mut f64,
        var_fn25_calc_iq__vdsat_rv_slot: &mut f64,
        var_fn25_calc_iq__vdsats_slot: &mut f64,
        var_fn25_calc_iq__vdsats1_slot: &mut f64,
        var_fn25_calc_iq__vdsats1_dn16_slot: &mut f64,
        var_fn25_calc_iq__vdsats1_dn17_slot: &mut f64,
        var_fn25_calc_iq__vdsats1_dn2_slot: &mut f64,
        var_fn25_calc_iq__vdsats1_dn3_slot: &mut f64,
        var_fn25_calc_iq__vdsats1_dn4_slot: &mut f64,
        var_fn25_calc_iq__vdsats1_dn7_slot: &mut f64,
        var_fn25_calc_iq__vdsats1_rv_slot: &mut f64,
        var_fn25_calc_iq__vdsats_dn16_slot: &mut f64,
        var_fn25_calc_iq__vdsats_dn17_slot: &mut f64,
        var_fn25_calc_iq__vdsats_dn2_slot: &mut f64,
        var_fn25_calc_iq__vdsats_dn3_slot: &mut f64,
        var_fn25_calc_iq__vdsats_dn4_slot: &mut f64,
        var_fn25_calc_iq__vdsats_dn7_slot: &mut f64,
        var_fn25_calc_iq__vdsats_rv_slot: &mut f64,
        var_fn25_calc_iq__vdx_slot: &mut f64,
        var_fn25_calc_iq__vdx_dn16_slot: &mut f64,
        var_fn25_calc_iq__vdx_dn17_slot: &mut f64,
        var_fn25_calc_iq__vdx_dn2_slot: &mut f64,
        var_fn25_calc_iq__vdx_dn3_slot: &mut f64,
        var_fn25_calc_iq__vdx_dn4_slot: &mut f64,
        var_fn25_calc_iq__vdx_dn7_slot: &mut f64,
        var_fn25_calc_iq__vdx_rv_slot: &mut f64,
        var_fn25_calc_iq__vtdibl_slot: &mut f64,
        var_fn25_calc_iq__vtdibl_dn16_slot: &mut f64,
        var_fn25_calc_iq__vtdibl_dn17_slot: &mut f64,
        var_fn25_calc_iq__vtdibl_dn4_slot: &mut f64,
        var_fn25_calc_iq__vtdibl_rv_slot: &mut f64,
        var_fn25_calc_iq__vx_slot: &mut f64,
        var_fn25_calc_iq__vx_dn16_slot: &mut f64,
        var_fn25_calc_iq__vx_dn17_slot: &mut f64,
        var_fn25_calc_iq__vx_dn2_slot: &mut f64,
        var_fn25_calc_iq__vx_dn3_slot: &mut f64,
        var_fn25_calc_iq__vx_dn4_slot: &mut f64,
        var_fn25_calc_iq__vx_dn7_slot: &mut f64,
        var_fn25_calc_iq__vx_rv_slot: &mut f64,
        var_guard27_slot: &mut f64,
        var_guard27_rv_slot: &mut f64,
        var_guard28_slot: &mut f64,
        var_guard28_rv_slot: &mut f64,
        var_guard29_slot: &mut f64,
        var_guard29_rv_slot: &mut f64,
        var_guard30_slot: &mut f64,
        var_guard30_rv_slot: &mut f64,
    ) {
        let mut var_fn25_calc_iq__delta: f64 = *var_fn25_calc_iq__delta_slot;
        let mut var_fn25_calc_iq__delta_dn16: f64 = *var_fn25_calc_iq__delta_dn16_slot;
        let mut var_fn25_calc_iq__delta_dn17: f64 = *var_fn25_calc_iq__delta_dn17_slot;
        let mut var_fn25_calc_iq__delta_rv: f64 = *var_fn25_calc_iq__delta_rv_slot;
        let mut var_fn25_calc_iq__eta: f64 = *var_fn25_calc_iq__eta_slot;
        let mut var_fn25_calc_iq__eta_dn16: f64 = *var_fn25_calc_iq__eta_dn16_slot;
        let mut var_fn25_calc_iq__eta_dn17: f64 = *var_fn25_calc_iq__eta_dn17_slot;
        let mut var_fn25_calc_iq__eta_dn2: f64 = *var_fn25_calc_iq__eta_dn2_slot;
        let mut var_fn25_calc_iq__eta_dn3: f64 = *var_fn25_calc_iq__eta_dn3_slot;
        let mut var_fn25_calc_iq__eta_dn4: f64 = *var_fn25_calc_iq__eta_dn4_slot;
        let mut var_fn25_calc_iq__eta_dn7: f64 = *var_fn25_calc_iq__eta_dn7_slot;
        let mut var_fn25_calc_iq__eta_rv: f64 = *var_fn25_calc_iq__eta_rv_slot;
        let mut var_fn25_calc_iq__exparg: f64 = *var_fn25_calc_iq__exparg_slot;
        let mut var_fn25_calc_iq__exparg_dn16: f64 = *var_fn25_calc_iq__exparg_dn16_slot;
        let mut var_fn25_calc_iq__exparg_dn17: f64 = *var_fn25_calc_iq__exparg_dn17_slot;
        let mut var_fn25_calc_iq__exparg_dn2: f64 = *var_fn25_calc_iq__exparg_dn2_slot;
        let mut var_fn25_calc_iq__exparg_dn3: f64 = *var_fn25_calc_iq__exparg_dn3_slot;
        let mut var_fn25_calc_iq__exparg_dn4: f64 = *var_fn25_calc_iq__exparg_dn4_slot;
        let mut var_fn25_calc_iq__exparg_dn7: f64 = *var_fn25_calc_iq__exparg_dn7_slot;
        let mut var_fn25_calc_iq__exparg_rv: f64 = *var_fn25_calc_iq__exparg_rv_slot;
        let mut var_fn25_calc_iq__ff: f64 = *var_fn25_calc_iq__ff_slot;
        let mut var_fn25_calc_iq__ff_dn16: f64 = *var_fn25_calc_iq__ff_dn16_slot;
        let mut var_fn25_calc_iq__ff_dn17: f64 = *var_fn25_calc_iq__ff_dn17_slot;
        let mut var_fn25_calc_iq__ff_dn2: f64 = *var_fn25_calc_iq__ff_dn2_slot;
        let mut var_fn25_calc_iq__ff_dn3: f64 = *var_fn25_calc_iq__ff_dn3_slot;
        let mut var_fn25_calc_iq__ff_dn4: f64 = *var_fn25_calc_iq__ff_dn4_slot;
        let mut var_fn25_calc_iq__ff_dn7: f64 = *var_fn25_calc_iq__ff_dn7_slot;
        let mut var_fn25_calc_iq__ff_rv: f64 = *var_fn25_calc_iq__ff_rv_slot;
        let mut var_fn25_calc_iq__fsd: f64 = *var_fn25_calc_iq__fsd_slot;
        let mut var_fn25_calc_iq__fsd_dn16: f64 = *var_fn25_calc_iq__fsd_dn16_slot;
        let mut var_fn25_calc_iq__fsd_dn17: f64 = *var_fn25_calc_iq__fsd_dn17_slot;
        let mut var_fn25_calc_iq__fsd_dn2: f64 = *var_fn25_calc_iq__fsd_dn2_slot;
        let mut var_fn25_calc_iq__fsd_dn3: f64 = *var_fn25_calc_iq__fsd_dn3_slot;
        let mut var_fn25_calc_iq__fsd_dn4: f64 = *var_fn25_calc_iq__fsd_dn4_slot;
        let mut var_fn25_calc_iq__fsd_dn7: f64 = *var_fn25_calc_iq__fsd_dn7_slot;
        let mut var_fn25_calc_iq__fsd_rv: f64 = *var_fn25_calc_iq__fsd_rv_slot;
        let mut var_fn25_calc_iq__muf: f64 = *var_fn25_calc_iq__muf_slot;
        let mut var_fn25_calc_iq__muf_dn16: f64 = *var_fn25_calc_iq__muf_dn16_slot;
        let mut var_fn25_calc_iq__muf_dn17: f64 = *var_fn25_calc_iq__muf_dn17_slot;
        let mut var_fn25_calc_iq__muf_dn2: f64 = *var_fn25_calc_iq__muf_dn2_slot;
        let mut var_fn25_calc_iq__muf_dn3: f64 = *var_fn25_calc_iq__muf_dn3_slot;
        let mut var_fn25_calc_iq__muf_dn4: f64 = *var_fn25_calc_iq__muf_dn4_slot;
        let mut var_fn25_calc_iq__muf_dn7: f64 = *var_fn25_calc_iq__muf_dn7_slot;
        let mut var_fn25_calc_iq__muf_rv: f64 = *var_fn25_calc_iq__muf_rv_slot;
        let mut var_fn25_calc_iq__myarg: f64 = *var_fn25_calc_iq__myarg_slot;
        let mut var_fn25_calc_iq__myarg_dn16: f64 = *var_fn25_calc_iq__myarg_dn16_slot;
        let mut var_fn25_calc_iq__myarg_dn17: f64 = *var_fn25_calc_iq__myarg_dn17_slot;
        let mut var_fn25_calc_iq__myarg_dn2: f64 = *var_fn25_calc_iq__myarg_dn2_slot;
        let mut var_fn25_calc_iq__myarg_dn3: f64 = *var_fn25_calc_iq__myarg_dn3_slot;
        let mut var_fn25_calc_iq__myarg_dn4: f64 = *var_fn25_calc_iq__myarg_dn4_slot;
        let mut var_fn25_calc_iq__myarg_dn7: f64 = *var_fn25_calc_iq__myarg_dn7_slot;
        let mut var_fn25_calc_iq__myarg_rv: f64 = *var_fn25_calc_iq__myarg_rv_slot;
        let mut var_fn25_calc_iq__qinvv: f64 = *var_fn25_calc_iq__qinvv_slot;
        let mut var_fn25_calc_iq__qinvv_dn16: f64 = *var_fn25_calc_iq__qinvv_dn16_slot;
        let mut var_fn25_calc_iq__qinvv_dn17: f64 = *var_fn25_calc_iq__qinvv_dn17_slot;
        let mut var_fn25_calc_iq__qinvv_dn2: f64 = *var_fn25_calc_iq__qinvv_dn2_slot;
        let mut var_fn25_calc_iq__qinvv_dn3: f64 = *var_fn25_calc_iq__qinvv_dn3_slot;
        let mut var_fn25_calc_iq__qinvv_dn4: f64 = *var_fn25_calc_iq__qinvv_dn4_slot;
        let mut var_fn25_calc_iq__qinvv_dn7: f64 = *var_fn25_calc_iq__qinvv_dn7_slot;
        let mut var_fn25_calc_iq__qinvv_rv: f64 = *var_fn25_calc_iq__qinvv_rv_slot;
        let mut var_fn25_calc_iq__qref: f64 = *var_fn25_calc_iq__qref_slot;
        let mut var_fn25_calc_iq__qref_dn16: f64 = *var_fn25_calc_iq__qref_dn16_slot;
        let mut var_fn25_calc_iq__qref_dn17: f64 = *var_fn25_calc_iq__qref_dn17_slot;
        let mut var_fn25_calc_iq__qref_dn4: f64 = *var_fn25_calc_iq__qref_dn4_slot;
        let mut var_fn25_calc_iq__qref_rv: f64 = *var_fn25_calc_iq__qref_rv_slot;
        let mut var_fn25_calc_iq__two_n_phit: f64 = *var_fn25_calc_iq__two_n_phit_slot;
        let mut var_fn25_calc_iq__two_n_phit_dn16: f64 = *var_fn25_calc_iq__two_n_phit_dn16_slot;
        let mut var_fn25_calc_iq__two_n_phit_dn17: f64 = *var_fn25_calc_iq__two_n_phit_dn17_slot;
        let mut var_fn25_calc_iq__two_n_phit_dn4: f64 = *var_fn25_calc_iq__two_n_phit_dn4_slot;
        let mut var_fn25_calc_iq__two_n_phit_rv: f64 = *var_fn25_calc_iq__two_n_phit_rv_slot;
        let mut var_fn25_calc_iq__vdsat: f64 = *var_fn25_calc_iq__vdsat_slot;
        let mut var_fn25_calc_iq__vdsat1: f64 = *var_fn25_calc_iq__vdsat1_slot;
        let mut var_fn25_calc_iq__vdsat1_dn16: f64 = *var_fn25_calc_iq__vdsat1_dn16_slot;
        let mut var_fn25_calc_iq__vdsat1_dn17: f64 = *var_fn25_calc_iq__vdsat1_dn17_slot;
        let mut var_fn25_calc_iq__vdsat1_dn2: f64 = *var_fn25_calc_iq__vdsat1_dn2_slot;
        let mut var_fn25_calc_iq__vdsat1_dn3: f64 = *var_fn25_calc_iq__vdsat1_dn3_slot;
        let mut var_fn25_calc_iq__vdsat1_dn4: f64 = *var_fn25_calc_iq__vdsat1_dn4_slot;
        let mut var_fn25_calc_iq__vdsat1_dn7: f64 = *var_fn25_calc_iq__vdsat1_dn7_slot;
        let mut var_fn25_calc_iq__vdsat1_rv: f64 = *var_fn25_calc_iq__vdsat1_rv_slot;
        let mut var_fn25_calc_iq__vdsat_dn16: f64 = *var_fn25_calc_iq__vdsat_dn16_slot;
        let mut var_fn25_calc_iq__vdsat_dn17: f64 = *var_fn25_calc_iq__vdsat_dn17_slot;
        let mut var_fn25_calc_iq__vdsat_dn2: f64 = *var_fn25_calc_iq__vdsat_dn2_slot;
        let mut var_fn25_calc_iq__vdsat_dn3: f64 = *var_fn25_calc_iq__vdsat_dn3_slot;
        let mut var_fn25_calc_iq__vdsat_dn4: f64 = *var_fn25_calc_iq__vdsat_dn4_slot;
        let mut var_fn25_calc_iq__vdsat_dn7: f64 = *var_fn25_calc_iq__vdsat_dn7_slot;
        let mut var_fn25_calc_iq__vdsat_rv: f64 = *var_fn25_calc_iq__vdsat_rv_slot;
        let mut var_fn25_calc_iq__vdsats: f64 = *var_fn25_calc_iq__vdsats_slot;
        let mut var_fn25_calc_iq__vdsats1: f64 = *var_fn25_calc_iq__vdsats1_slot;
        let mut var_fn25_calc_iq__vdsats1_dn16: f64 = *var_fn25_calc_iq__vdsats1_dn16_slot;
        let mut var_fn25_calc_iq__vdsats1_dn17: f64 = *var_fn25_calc_iq__vdsats1_dn17_slot;
        let mut var_fn25_calc_iq__vdsats1_dn2: f64 = *var_fn25_calc_iq__vdsats1_dn2_slot;
        let mut var_fn25_calc_iq__vdsats1_dn3: f64 = *var_fn25_calc_iq__vdsats1_dn3_slot;
        let mut var_fn25_calc_iq__vdsats1_dn4: f64 = *var_fn25_calc_iq__vdsats1_dn4_slot;
        let mut var_fn25_calc_iq__vdsats1_dn7: f64 = *var_fn25_calc_iq__vdsats1_dn7_slot;
        let mut var_fn25_calc_iq__vdsats1_rv: f64 = *var_fn25_calc_iq__vdsats1_rv_slot;
        let mut var_fn25_calc_iq__vdsats_dn16: f64 = *var_fn25_calc_iq__vdsats_dn16_slot;
        let mut var_fn25_calc_iq__vdsats_dn17: f64 = *var_fn25_calc_iq__vdsats_dn17_slot;
        let mut var_fn25_calc_iq__vdsats_dn2: f64 = *var_fn25_calc_iq__vdsats_dn2_slot;
        let mut var_fn25_calc_iq__vdsats_dn3: f64 = *var_fn25_calc_iq__vdsats_dn3_slot;
        let mut var_fn25_calc_iq__vdsats_dn4: f64 = *var_fn25_calc_iq__vdsats_dn4_slot;
        let mut var_fn25_calc_iq__vdsats_dn7: f64 = *var_fn25_calc_iq__vdsats_dn7_slot;
        let mut var_fn25_calc_iq__vdsats_rv: f64 = *var_fn25_calc_iq__vdsats_rv_slot;
        let mut var_fn25_calc_iq__vdx: f64 = *var_fn25_calc_iq__vdx_slot;
        let mut var_fn25_calc_iq__vdx_dn16: f64 = *var_fn25_calc_iq__vdx_dn16_slot;
        let mut var_fn25_calc_iq__vdx_dn17: f64 = *var_fn25_calc_iq__vdx_dn17_slot;
        let mut var_fn25_calc_iq__vdx_dn2: f64 = *var_fn25_calc_iq__vdx_dn2_slot;
        let mut var_fn25_calc_iq__vdx_dn3: f64 = *var_fn25_calc_iq__vdx_dn3_slot;
        let mut var_fn25_calc_iq__vdx_dn4: f64 = *var_fn25_calc_iq__vdx_dn4_slot;
        let mut var_fn25_calc_iq__vdx_dn7: f64 = *var_fn25_calc_iq__vdx_dn7_slot;
        let mut var_fn25_calc_iq__vdx_rv: f64 = *var_fn25_calc_iq__vdx_rv_slot;
        let mut var_fn25_calc_iq__vtdibl: f64 = *var_fn25_calc_iq__vtdibl_slot;
        let mut var_fn25_calc_iq__vtdibl_dn16: f64 = *var_fn25_calc_iq__vtdibl_dn16_slot;
        let mut var_fn25_calc_iq__vtdibl_dn17: f64 = *var_fn25_calc_iq__vtdibl_dn17_slot;
        let mut var_fn25_calc_iq__vtdibl_dn4: f64 = *var_fn25_calc_iq__vtdibl_dn4_slot;
        let mut var_fn25_calc_iq__vtdibl_rv: f64 = *var_fn25_calc_iq__vtdibl_rv_slot;
        let mut var_fn25_calc_iq__vx: f64 = *var_fn25_calc_iq__vx_slot;
        let mut var_fn25_calc_iq__vx_dn16: f64 = *var_fn25_calc_iq__vx_dn16_slot;
        let mut var_fn25_calc_iq__vx_dn17: f64 = *var_fn25_calc_iq__vx_dn17_slot;
        let mut var_fn25_calc_iq__vx_dn2: f64 = *var_fn25_calc_iq__vx_dn2_slot;
        let mut var_fn25_calc_iq__vx_dn3: f64 = *var_fn25_calc_iq__vx_dn3_slot;
        let mut var_fn25_calc_iq__vx_dn4: f64 = *var_fn25_calc_iq__vx_dn4_slot;
        let mut var_fn25_calc_iq__vx_dn7: f64 = *var_fn25_calc_iq__vx_dn7_slot;
        let mut var_fn25_calc_iq__vx_rv: f64 = *var_fn25_calc_iq__vx_rv_slot;
        let mut var_guard27: f64 = *var_guard27_slot;
        let mut var_guard27_rv: f64 = *var_guard27_rv_slot;
        let mut var_guard28: f64 = *var_guard28_slot;
        let mut var_guard28_rv: f64 = *var_guard28_rv_slot;
        let mut var_guard29: f64 = *var_guard29_slot;
        let mut var_guard29_rv: f64 = *var_guard29_rv_slot;
        let mut var_guard30: f64 = *var_guard30_slot;
        let mut var_guard30_rv: f64 = *var_guard30_rv_slot;

        let (assign2970_e4440, assign2970_e4440_d_n16, assign2970_e4440_d_n17,) = {
    if (var_guard24 != 0.0) {
        let assign2970_e4435: f64 = (var_fn25_calc_iq__vsatdibl * var_fn25_calc_iq__delta2);
        let assign2970_e4436: f64 = (var_fn25_calc_iq__delta1 - assign2970_e4435);
        let assign2970_e4438: f64 = (assign2970_e4436 * var_fn25_calc_iq__absvdsin);
        (assign2970_e4438, (((-(var_fn25_calc_iq__vsatdibl_dn16 * var_fn25_calc_iq__delta2)) * var_fn25_calc_iq__absvdsin) + (assign2970_e4436 * var_fn25_calc_iq__absvdsin_dn16)), (((-(var_fn25_calc_iq__vsatdibl_dn17 * var_fn25_calc_iq__delta2)) * var_fn25_calc_iq__absvdsin) + (assign2970_e4436 * var_fn25_calc_iq__absvdsin_dn17)),)
    } else {
        (var_fn25_calc_iq__delta, var_fn25_calc_iq__delta_dn16, var_fn25_calc_iq__delta_dn17,)
    }
};
        var_fn25_calc_iq__delta = assign2970_e4440;
        var_fn25_calc_iq__delta_dn16 = assign2970_e4440_d_n16;
        var_fn25_calc_iq__delta_dn17 = assign2970_e4440_d_n17;
        var_fn25_calc_iq__delta_rv = 0.0;

        let (assign2980_e4446, assign2980_e4446_d_n4, assign2980_e4446_d_n16, assign2980_e4446_d_n17,) = {
    if (var_guard24 != 0.0) {
        let assign2980_e4444: f64 = (var_fn25_calc_iq__vtof - var_fn25_calc_iq__delta);
        (assign2980_e4444, var_fn25_calc_iq__vtof_dn4, (-var_fn25_calc_iq__delta_dn16), (-var_fn25_calc_iq__delta_dn17),)
    } else {
        (var_fn25_calc_iq__vtdibl, var_fn25_calc_iq__vtdibl_dn4, var_fn25_calc_iq__vtdibl_dn16, var_fn25_calc_iq__vtdibl_dn17,)
    }
};
        var_fn25_calc_iq__vtdibl = assign2980_e4446;
        var_fn25_calc_iq__vtdibl_dn4 = assign2980_e4446_d_n4;
        var_fn25_calc_iq__vtdibl_dn16 = assign2980_e4446_d_n16;
        var_fn25_calc_iq__vtdibl_dn17 = assign2980_e4446_d_n17;
        var_fn25_calc_iq__vtdibl_rv = 0.0;

        let (assign2990_e4454, assign2990_e4454_d_n4, assign2990_e4454_d_n16, assign2990_e4454_d_n17,) = {
    if (var_guard24 != 0.0) {
        let assign2990_e4450: f64 = (2.0 * var_fn25_calc_iq__n);
        let assign2990_e4452: f64 = (assign2990_e4450 * var_fn25_calc_iq__phitin);
        (assign2990_e4452, (((2.0 * var_fn25_calc_iq__n_dn4) * var_fn25_calc_iq__phitin) + (assign2990_e4450 * var_fn25_calc_iq__phitin_dn4)), ((2.0 * var_fn25_calc_iq__n_dn16) * var_fn25_calc_iq__phitin), ((2.0 * var_fn25_calc_iq__n_dn17) * var_fn25_calc_iq__phitin),)
    } else {
        (var_fn25_calc_iq__two_n_phit, var_fn25_calc_iq__two_n_phit_dn4, var_fn25_calc_iq__two_n_phit_dn16, var_fn25_calc_iq__two_n_phit_dn17,)
    }
};
        var_fn25_calc_iq__two_n_phit = assign2990_e4454;
        var_fn25_calc_iq__two_n_phit_dn4 = assign2990_e4454_d_n4;
        var_fn25_calc_iq__two_n_phit_dn16 = assign2990_e4454_d_n16;
        var_fn25_calc_iq__two_n_phit_dn17 = assign2990_e4454_d_n17;
        var_fn25_calc_iq__two_n_phit_rv = 0.0;

        let (assign3000_e4460, assign3000_e4460_d_n4, assign3000_e4460_d_n16, assign3000_e4460_d_n17,) = {
    if (var_guard24 != 0.0) {
        let assign3000_e4458: f64 = (var_fn25_calc_iq__cgin * var_fn25_calc_iq__two_n_phit);
        (assign3000_e4458, ((var_fn25_calc_iq__cgin_dn4 * var_fn25_calc_iq__two_n_phit) + (var_fn25_calc_iq__cgin * var_fn25_calc_iq__two_n_phit_dn4)), (var_fn25_calc_iq__cgin * var_fn25_calc_iq__two_n_phit_dn16), (var_fn25_calc_iq__cgin * var_fn25_calc_iq__two_n_phit_dn17),)
    } else {
        (var_fn25_calc_iq__qref, var_fn25_calc_iq__qref_dn4, var_fn25_calc_iq__qref_dn16, var_fn25_calc_iq__qref_dn17,)
    }
};
        var_fn25_calc_iq__qref = assign3000_e4460;
        var_fn25_calc_iq__qref_dn4 = assign3000_e4460_d_n4;
        var_fn25_calc_iq__qref_dn16 = assign3000_e4460_d_n16;
        var_fn25_calc_iq__qref_dn17 = assign3000_e4460_d_n17;
        var_fn25_calc_iq__qref_rv = 0.0;

        let (assign3010_e4470, assign3010_e4470_d_n2, assign3010_e4470_d_n3, assign3010_e4470_d_n4, assign3010_e4470_d_n7, assign3010_e4470_d_n16, assign3010_e4470_d_n17,) = {
    if (var_guard24 != 0.0) {
        let assign3010_e4465: f64 = (p.p51 * var_fn25_calc_iq__alpha_phit);
        let assign3010_e4467: f64 = (assign3010_e4465 / 2.0);
        let assign3010_e4468: f64 = (var_fn25_calc_iq__vtdibl - assign3010_e4467);
        (assign3010_e4468, 0.0, 0.0, (var_fn25_calc_iq__vtdibl_dn4 - ((p.p51 * var_fn25_calc_iq__alpha_phit_dn4) / 2.0)), 0.0, var_fn25_calc_iq__vtdibl_dn16, var_fn25_calc_iq__vtdibl_dn17,)
    } else {
        (var_fn25_calc_iq__myarg, var_fn25_calc_iq__myarg_dn2, var_fn25_calc_iq__myarg_dn3, var_fn25_calc_iq__myarg_dn4, var_fn25_calc_iq__myarg_dn7, var_fn25_calc_iq__myarg_dn16, var_fn25_calc_iq__myarg_dn17,)
    }
};
        var_fn25_calc_iq__myarg = assign3010_e4470;
        var_fn25_calc_iq__myarg_dn2 = assign3010_e4470_d_n2;
        var_fn25_calc_iq__myarg_dn3 = assign3010_e4470_d_n3;
        var_fn25_calc_iq__myarg_dn4 = assign3010_e4470_d_n4;
        var_fn25_calc_iq__myarg_dn7 = assign3010_e4470_d_n7;
        var_fn25_calc_iq__myarg_dn16 = assign3010_e4470_d_n16;
        var_fn25_calc_iq__myarg_dn17 = assign3010_e4470_d_n17;
        var_fn25_calc_iq__myarg_rv = 0.0;

        let (assign3020_e4521, assign3020_e4521_d_n2, assign3020_e4521_d_n3, assign3020_e4521_d_n4, assign3020_e4521_d_n7, assign3020_e4521_d_n16, assign3020_e4521_d_n17,) = {
    if (var_guard24 != 0.0) {
        let (assign3020_e4515, assign3020_e4515_d_n2, assign3020_e4515_d_n7, assign3020_e4515_d_n16, assign3020_e4515_d_n17,) = {
            if (p.p52 != 0.0) {
                let assign3020_e4479: f64 = (var_fn25_calc_iq__vgsin + var_fn25_calc_iq__vgdin);
                let assign3020_e4482: f64 = (var_fn25_calc_iq__vgsin - var_fn25_calc_iq__vgdin);
                let assign3020_e4485: f64 = (0.001 / p.p53);
                let assign3020_e4488: f64 = (var_fn25_calc_iq__vgsin - var_fn25_calc_iq__vgdin);
                let assign3020_e4489: f64 = (assign3020_e4485 * assign3020_e4488);
                let assign3020_e4490: f64 = (assign3020_e4489).tanh();
                let assign3020_e4491: f64 = (assign3020_e4482 * assign3020_e4490);
                let assign3020_e4492: f64 = (assign3020_e4479 + assign3020_e4491);
                let assign3020_e4493: f64 = (0.5 * assign3020_e4492);
                (assign3020_e4493, (0.5 * ((var_fn25_calc_iq__vgsin_dn2 + var_fn25_calc_iq__vgdin_dn2) + (((var_fn25_calc_iq__vgsin_dn2 - var_fn25_calc_iq__vgdin_dn2) * assign3020_e4490) + (assign3020_e4482 * ((assign3020_e4485 * (var_fn25_calc_iq__vgsin_dn2 - var_fn25_calc_iq__vgdin_dn2)) / ((assign3020_e4489).cosh() * (assign3020_e4489).cosh())))))), (0.5 * ((var_fn25_calc_iq__vgsin_dn7 + var_fn25_calc_iq__vgdin_dn7) + (((var_fn25_calc_iq__vgsin_dn7 - var_fn25_calc_iq__vgdin_dn7) * assign3020_e4490) + (assign3020_e4482 * ((assign3020_e4485 * (var_fn25_calc_iq__vgsin_dn7 - var_fn25_calc_iq__vgdin_dn7)) / ((assign3020_e4489).cosh() * (assign3020_e4489).cosh())))))), (0.5 * ((var_fn25_calc_iq__vgsin_dn16 + var_fn25_calc_iq__vgdin_dn16) + (((var_fn25_calc_iq__vgsin_dn16 - var_fn25_calc_iq__vgdin_dn16) * assign3020_e4490) + (assign3020_e4482 * ((assign3020_e4485 * (var_fn25_calc_iq__vgsin_dn16 - var_fn25_calc_iq__vgdin_dn16)) / ((assign3020_e4489).cosh() * (assign3020_e4489).cosh())))))), (0.5 * (var_fn25_calc_iq__vgdin_dn17 + (((-var_fn25_calc_iq__vgdin_dn17) * assign3020_e4490) + (assign3020_e4482 * ((assign3020_e4485 * (-var_fn25_calc_iq__vgdin_dn17)) / ((assign3020_e4489).cosh() * (assign3020_e4489).cosh())))))),)
            } else {
                let (assign3020_e4514, assign3020_e4514_d_n2, assign3020_e4514_d_n7, assign3020_e4514_d_n16, assign3020_e4514_d_n17,) = {
                    if (p.p52 == 0.0) {
                        let assign3020_e4500: f64 = (var_fn25_calc_iq__vgsin + var_fn25_calc_iq__vgdin);
                        let assign3020_e4503: f64 = (var_fn25_calc_iq__vgsin - var_fn25_calc_iq__vgdin);
                        let assign3020_e4506: f64 = (var_fn25_calc_iq__vgsin - var_fn25_calc_iq__vgdin);
                        let assign3020_e4507: f64 = (assign3020_e4503 * assign3020_e4506);
                        let assign3020_e4509: f64 = (assign3020_e4507 + p.p53);
                        let assign3020_e4510: f64 = (assign3020_e4509).sqrt();
                        let assign3020_e4511: f64 = (assign3020_e4500 + assign3020_e4510);
                        let assign3020_e4512: f64 = (0.5 * assign3020_e4511);
                        (assign3020_e4512, (0.5 * ((var_fn25_calc_iq__vgsin_dn2 + var_fn25_calc_iq__vgdin_dn2) + ((((var_fn25_calc_iq__vgsin_dn2 - var_fn25_calc_iq__vgdin_dn2) * assign3020_e4506) + (assign3020_e4503 * (var_fn25_calc_iq__vgsin_dn2 - var_fn25_calc_iq__vgdin_dn2))) / (2.0 * assign3020_e4510)))), (0.5 * ((var_fn25_calc_iq__vgsin_dn7 + var_fn25_calc_iq__vgdin_dn7) + ((((var_fn25_calc_iq__vgsin_dn7 - var_fn25_calc_iq__vgdin_dn7) * assign3020_e4506) + (assign3020_e4503 * (var_fn25_calc_iq__vgsin_dn7 - var_fn25_calc_iq__vgdin_dn7))) / (2.0 * assign3020_e4510)))), (0.5 * ((var_fn25_calc_iq__vgsin_dn16 + var_fn25_calc_iq__vgdin_dn16) + ((((var_fn25_calc_iq__vgsin_dn16 - var_fn25_calc_iq__vgdin_dn16) * assign3020_e4506) + (assign3020_e4503 * (var_fn25_calc_iq__vgsin_dn16 - var_fn25_calc_iq__vgdin_dn16))) / (2.0 * assign3020_e4510)))), (0.5 * (var_fn25_calc_iq__vgdin_dn17 + ((((-var_fn25_calc_iq__vgdin_dn17) * assign3020_e4506) + (assign3020_e4503 * (-var_fn25_calc_iq__vgdin_dn17))) / (2.0 * assign3020_e4510)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign3020_e4514, assign3020_e4514_d_n2, assign3020_e4514_d_n7, assign3020_e4514_d_n16, assign3020_e4514_d_n17,)
            }
        };
        let assign3020_e4517: f64 = (assign3020_e4515 - var_fn25_calc_iq__myarg);
        let assign3020_e4519: f64 = (assign3020_e4517 / var_fn25_calc_iq__alpha_phit);
        (assign3020_e4519, ((assign3020_e4515_d_n2 - var_fn25_calc_iq__myarg_dn2) / var_fn25_calc_iq__alpha_phit), ((-var_fn25_calc_iq__myarg_dn3) / var_fn25_calc_iq__alpha_phit), ((((-var_fn25_calc_iq__myarg_dn4) * var_fn25_calc_iq__alpha_phit) - (assign3020_e4517 * var_fn25_calc_iq__alpha_phit_dn4)) / (var_fn25_calc_iq__alpha_phit * var_fn25_calc_iq__alpha_phit)), ((assign3020_e4515_d_n7 - var_fn25_calc_iq__myarg_dn7) / var_fn25_calc_iq__alpha_phit), ((assign3020_e4515_d_n16 - var_fn25_calc_iq__myarg_dn16) / var_fn25_calc_iq__alpha_phit), ((assign3020_e4515_d_n17 - var_fn25_calc_iq__myarg_dn17) / var_fn25_calc_iq__alpha_phit),)
    } else {
        (var_fn25_calc_iq__exparg, var_fn25_calc_iq__exparg_dn2, var_fn25_calc_iq__exparg_dn3, var_fn25_calc_iq__exparg_dn4, var_fn25_calc_iq__exparg_dn7, var_fn25_calc_iq__exparg_dn16, var_fn25_calc_iq__exparg_dn17,)
    }
};
        var_fn25_calc_iq__exparg = assign3020_e4521;
        var_fn25_calc_iq__exparg_dn2 = assign3020_e4521_d_n2;
        var_fn25_calc_iq__exparg_dn3 = assign3020_e4521_d_n3;
        var_fn25_calc_iq__exparg_dn4 = assign3020_e4521_d_n4;
        var_fn25_calc_iq__exparg_dn7 = assign3020_e4521_d_n7;
        var_fn25_calc_iq__exparg_dn16 = assign3020_e4521_d_n16;
        var_fn25_calc_iq__exparg_dn17 = assign3020_e4521_d_n17;
        var_fn25_calc_iq__exparg_rv = 0.0;

        let assign3030_e4524: f64 = if var_fn25_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        var_guard27 = assign3030_e4524;
        var_guard27_rv = 0.0;

        let (assign3040_e4530, assign3040_e4530_d_n2, assign3040_e4530_d_n3, assign3040_e4530_d_n4, assign3040_e4530_d_n7, assign3040_e4530_d_n16, assign3040_e4530_d_n17,) = {
    if ((var_guard24 != 0.0) && (var_guard27 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__ff, var_fn25_calc_iq__ff_dn2, var_fn25_calc_iq__ff_dn3, var_fn25_calc_iq__ff_dn4, var_fn25_calc_iq__ff_dn7, var_fn25_calc_iq__ff_dn16, var_fn25_calc_iq__ff_dn17,)
    }
};
        var_fn25_calc_iq__ff = assign3040_e4530;
        var_fn25_calc_iq__ff_dn2 = assign3040_e4530_d_n2;
        var_fn25_calc_iq__ff_dn3 = assign3040_e4530_d_n3;
        var_fn25_calc_iq__ff_dn4 = assign3040_e4530_d_n4;
        var_fn25_calc_iq__ff_dn7 = assign3040_e4530_d_n7;
        var_fn25_calc_iq__ff_dn16 = assign3040_e4530_d_n16;
        var_fn25_calc_iq__ff_dn17 = assign3040_e4530_d_n17;
        var_fn25_calc_iq__ff_rv = 0.0;

        let assign3050_e4533: f64 = (-50.0);
        let assign3050_e4534: f64 = if var_fn25_calc_iq__exparg < assign3050_e4533 { 1.0 } else { 0.0 };
        var_guard28 = assign3050_e4534;
        var_guard28_rv = 0.0;

        let (assign3060_e4543, assign3060_e4543_d_n2, assign3060_e4543_d_n3, assign3060_e4543_d_n4, assign3060_e4543_d_n7, assign3060_e4543_d_n16, assign3060_e4543_d_n17,) = {
    if (((var_guard24 != 0.0) && (var_guard27 == 0.0)) && (var_guard28 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__ff, var_fn25_calc_iq__ff_dn2, var_fn25_calc_iq__ff_dn3, var_fn25_calc_iq__ff_dn4, var_fn25_calc_iq__ff_dn7, var_fn25_calc_iq__ff_dn16, var_fn25_calc_iq__ff_dn17,)
    }
};
        var_fn25_calc_iq__ff = assign3060_e4543;
        var_fn25_calc_iq__ff_dn2 = assign3060_e4543_d_n2;
        var_fn25_calc_iq__ff_dn3 = assign3060_e4543_d_n3;
        var_fn25_calc_iq__ff_dn4 = assign3060_e4543_d_n4;
        var_fn25_calc_iq__ff_dn7 = assign3060_e4543_d_n7;
        var_fn25_calc_iq__ff_dn16 = assign3060_e4543_d_n16;
        var_fn25_calc_iq__ff_dn17 = assign3060_e4543_d_n17;
        var_fn25_calc_iq__ff_rv = 0.0;

        let (assign3070_e4558, assign3070_e4558_d_n2, assign3070_e4558_d_n3, assign3070_e4558_d_n4, assign3070_e4558_d_n7, assign3070_e4558_d_n16, assign3070_e4558_d_n17,) = {
    if (((var_guard24 != 0.0) && (var_guard27 == 0.0)) && (var_guard28 == 0.0)) {
        let assign3070_e4554: f64 = (var_fn25_calc_iq__exparg).exp();
        let assign3070_e4555: f64 = (1.0 + assign3070_e4554);
        let assign3070_e4556: f64 = (1.0 / assign3070_e4555);
        (assign3070_e4556, (-((assign3070_e4554 * var_fn25_calc_iq__exparg_dn2) / (assign3070_e4555 * assign3070_e4555))), (-((assign3070_e4554 * var_fn25_calc_iq__exparg_dn3) / (assign3070_e4555 * assign3070_e4555))), (-((assign3070_e4554 * var_fn25_calc_iq__exparg_dn4) / (assign3070_e4555 * assign3070_e4555))), (-((assign3070_e4554 * var_fn25_calc_iq__exparg_dn7) / (assign3070_e4555 * assign3070_e4555))), (-((assign3070_e4554 * var_fn25_calc_iq__exparg_dn16) / (assign3070_e4555 * assign3070_e4555))), (-((assign3070_e4554 * var_fn25_calc_iq__exparg_dn17) / (assign3070_e4555 * assign3070_e4555))),)
    } else {
        (var_fn25_calc_iq__ff, var_fn25_calc_iq__ff_dn2, var_fn25_calc_iq__ff_dn3, var_fn25_calc_iq__ff_dn4, var_fn25_calc_iq__ff_dn7, var_fn25_calc_iq__ff_dn16, var_fn25_calc_iq__ff_dn17,)
    }
};
        var_fn25_calc_iq__ff = assign3070_e4558;
        var_fn25_calc_iq__ff_dn2 = assign3070_e4558_d_n2;
        var_fn25_calc_iq__ff_dn3 = assign3070_e4558_d_n3;
        var_fn25_calc_iq__ff_dn4 = assign3070_e4558_d_n4;
        var_fn25_calc_iq__ff_dn7 = assign3070_e4558_d_n7;
        var_fn25_calc_iq__ff_dn16 = assign3070_e4558_d_n16;
        var_fn25_calc_iq__ff_dn17 = assign3070_e4558_d_n17;
        var_fn25_calc_iq__ff_rv = 0.0;

        let (assign3080_e4617, assign3080_e4617_d_n2, assign3080_e4617_d_n3, assign3080_e4617_d_n4, assign3080_e4617_d_n7, assign3080_e4617_d_n16, assign3080_e4617_d_n17,) = {
    if (var_guard24 != 0.0) {
        let (assign3080_e4603, assign3080_e4603_d_n2, assign3080_e4603_d_n7, assign3080_e4603_d_n16, assign3080_e4603_d_n17,) = {
            if (p.p52 != 0.0) {
                let assign3080_e4567: f64 = (var_fn25_calc_iq__vgsin + var_fn25_calc_iq__vgdin);
                let assign3080_e4570: f64 = (var_fn25_calc_iq__vgsin - var_fn25_calc_iq__vgdin);
                let assign3080_e4573: f64 = (0.001 / p.p53);
                let assign3080_e4576: f64 = (var_fn25_calc_iq__vgsin - var_fn25_calc_iq__vgdin);
                let assign3080_e4577: f64 = (assign3080_e4573 * assign3080_e4576);
                let assign3080_e4578: f64 = (assign3080_e4577).tanh();
                let assign3080_e4579: f64 = (assign3080_e4570 * assign3080_e4578);
                let assign3080_e4580: f64 = (assign3080_e4567 + assign3080_e4579);
                let assign3080_e4581: f64 = (0.5 * assign3080_e4580);
                (assign3080_e4581, (0.5 * ((var_fn25_calc_iq__vgsin_dn2 + var_fn25_calc_iq__vgdin_dn2) + (((var_fn25_calc_iq__vgsin_dn2 - var_fn25_calc_iq__vgdin_dn2) * assign3080_e4578) + (assign3080_e4570 * ((assign3080_e4573 * (var_fn25_calc_iq__vgsin_dn2 - var_fn25_calc_iq__vgdin_dn2)) / ((assign3080_e4577).cosh() * (assign3080_e4577).cosh())))))), (0.5 * ((var_fn25_calc_iq__vgsin_dn7 + var_fn25_calc_iq__vgdin_dn7) + (((var_fn25_calc_iq__vgsin_dn7 - var_fn25_calc_iq__vgdin_dn7) * assign3080_e4578) + (assign3080_e4570 * ((assign3080_e4573 * (var_fn25_calc_iq__vgsin_dn7 - var_fn25_calc_iq__vgdin_dn7)) / ((assign3080_e4577).cosh() * (assign3080_e4577).cosh())))))), (0.5 * ((var_fn25_calc_iq__vgsin_dn16 + var_fn25_calc_iq__vgdin_dn16) + (((var_fn25_calc_iq__vgsin_dn16 - var_fn25_calc_iq__vgdin_dn16) * assign3080_e4578) + (assign3080_e4570 * ((assign3080_e4573 * (var_fn25_calc_iq__vgsin_dn16 - var_fn25_calc_iq__vgdin_dn16)) / ((assign3080_e4577).cosh() * (assign3080_e4577).cosh())))))), (0.5 * (var_fn25_calc_iq__vgdin_dn17 + (((-var_fn25_calc_iq__vgdin_dn17) * assign3080_e4578) + (assign3080_e4570 * ((assign3080_e4573 * (-var_fn25_calc_iq__vgdin_dn17)) / ((assign3080_e4577).cosh() * (assign3080_e4577).cosh())))))),)
            } else {
                let (assign3080_e4602, assign3080_e4602_d_n2, assign3080_e4602_d_n7, assign3080_e4602_d_n16, assign3080_e4602_d_n17,) = {
                    if (p.p52 == 0.0) {
                        let assign3080_e4588: f64 = (var_fn25_calc_iq__vgsin + var_fn25_calc_iq__vgdin);
                        let assign3080_e4591: f64 = (var_fn25_calc_iq__vgsin - var_fn25_calc_iq__vgdin);
                        let assign3080_e4594: f64 = (var_fn25_calc_iq__vgsin - var_fn25_calc_iq__vgdin);
                        let assign3080_e4595: f64 = (assign3080_e4591 * assign3080_e4594);
                        let assign3080_e4597: f64 = (assign3080_e4595 + p.p53);
                        let assign3080_e4598: f64 = (assign3080_e4597).sqrt();
                        let assign3080_e4599: f64 = (assign3080_e4588 + assign3080_e4598);
                        let assign3080_e4600: f64 = (0.5 * assign3080_e4599);
                        (assign3080_e4600, (0.5 * ((var_fn25_calc_iq__vgsin_dn2 + var_fn25_calc_iq__vgdin_dn2) + ((((var_fn25_calc_iq__vgsin_dn2 - var_fn25_calc_iq__vgdin_dn2) * assign3080_e4594) + (assign3080_e4591 * (var_fn25_calc_iq__vgsin_dn2 - var_fn25_calc_iq__vgdin_dn2))) / (2.0 * assign3080_e4598)))), (0.5 * ((var_fn25_calc_iq__vgsin_dn7 + var_fn25_calc_iq__vgdin_dn7) + ((((var_fn25_calc_iq__vgsin_dn7 - var_fn25_calc_iq__vgdin_dn7) * assign3080_e4594) + (assign3080_e4591 * (var_fn25_calc_iq__vgsin_dn7 - var_fn25_calc_iq__vgdin_dn7))) / (2.0 * assign3080_e4598)))), (0.5 * ((var_fn25_calc_iq__vgsin_dn16 + var_fn25_calc_iq__vgdin_dn16) + ((((var_fn25_calc_iq__vgsin_dn16 - var_fn25_calc_iq__vgdin_dn16) * assign3080_e4594) + (assign3080_e4591 * (var_fn25_calc_iq__vgsin_dn16 - var_fn25_calc_iq__vgdin_dn16))) / (2.0 * assign3080_e4598)))), (0.5 * (var_fn25_calc_iq__vgdin_dn17 + ((((-var_fn25_calc_iq__vgdin_dn17) * assign3080_e4594) + (assign3080_e4591 * (-var_fn25_calc_iq__vgdin_dn17))) / (2.0 * assign3080_e4598)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign3080_e4602, assign3080_e4602_d_n2, assign3080_e4602_d_n7, assign3080_e4602_d_n16, assign3080_e4602_d_n17,)
            }
        };
        let assign3080_e4607: f64 = (p.p51 * 0.1);
        let assign3080_e4609: f64 = (assign3080_e4607 * var_fn25_calc_iq__alpha_phit);
        let assign3080_e4611: f64 = (assign3080_e4609 * var_fn25_calc_iq__ff);
        let assign3080_e4612: f64 = (var_fn25_calc_iq__vtdibl - assign3080_e4611);
        let assign3080_e4613: f64 = (assign3080_e4603 - assign3080_e4612);
        let assign3080_e4615: f64 = (assign3080_e4613 / var_fn25_calc_iq__two_n_phit);
        (assign3080_e4615, ((assign3080_e4603_d_n2 - (-(assign3080_e4609 * var_fn25_calc_iq__ff_dn2))) / var_fn25_calc_iq__two_n_phit), ((-(-(assign3080_e4609 * var_fn25_calc_iq__ff_dn3))) / var_fn25_calc_iq__two_n_phit), ((((-(var_fn25_calc_iq__vtdibl_dn4 - (((assign3080_e4607 * var_fn25_calc_iq__alpha_phit_dn4) * var_fn25_calc_iq__ff) + (assign3080_e4609 * var_fn25_calc_iq__ff_dn4)))) * var_fn25_calc_iq__two_n_phit) - (assign3080_e4613 * var_fn25_calc_iq__two_n_phit_dn4)) / (var_fn25_calc_iq__two_n_phit * var_fn25_calc_iq__two_n_phit)), ((assign3080_e4603_d_n7 - (-(assign3080_e4609 * var_fn25_calc_iq__ff_dn7))) / var_fn25_calc_iq__two_n_phit), ((((assign3080_e4603_d_n16 - (var_fn25_calc_iq__vtdibl_dn16 - (assign3080_e4609 * var_fn25_calc_iq__ff_dn16))) * var_fn25_calc_iq__two_n_phit) - (assign3080_e4613 * var_fn25_calc_iq__two_n_phit_dn16)) / (var_fn25_calc_iq__two_n_phit * var_fn25_calc_iq__two_n_phit)), ((((assign3080_e4603_d_n17 - (var_fn25_calc_iq__vtdibl_dn17 - (assign3080_e4609 * var_fn25_calc_iq__ff_dn17))) * var_fn25_calc_iq__two_n_phit) - (assign3080_e4613 * var_fn25_calc_iq__two_n_phit_dn17)) / (var_fn25_calc_iq__two_n_phit * var_fn25_calc_iq__two_n_phit)),)
    } else {
        (var_fn25_calc_iq__eta, var_fn25_calc_iq__eta_dn2, var_fn25_calc_iq__eta_dn3, var_fn25_calc_iq__eta_dn4, var_fn25_calc_iq__eta_dn7, var_fn25_calc_iq__eta_dn16, var_fn25_calc_iq__eta_dn17,)
    }
};
        var_fn25_calc_iq__eta = assign3080_e4617;
        var_fn25_calc_iq__eta_dn2 = assign3080_e4617_d_n2;
        var_fn25_calc_iq__eta_dn3 = assign3080_e4617_d_n3;
        var_fn25_calc_iq__eta_dn4 = assign3080_e4617_d_n4;
        var_fn25_calc_iq__eta_dn7 = assign3080_e4617_d_n7;
        var_fn25_calc_iq__eta_dn16 = assign3080_e4617_d_n16;
        var_fn25_calc_iq__eta_dn17 = assign3080_e4617_d_n17;
        var_fn25_calc_iq__eta_rv = 0.0;

        let assign3090_e4620: f64 = if var_fn25_calc_iq__eta > 50.0 { 1.0 } else { 0.0 };
        var_guard29 = assign3090_e4620;
        var_guard29_rv = 0.0;

        let (assign3100_e4628, assign3100_e4628_d_n2, assign3100_e4628_d_n3, assign3100_e4628_d_n4, assign3100_e4628_d_n7, assign3100_e4628_d_n16, assign3100_e4628_d_n17,) = {
    if ((var_guard24 != 0.0) && (var_guard29 != 0.0)) {
        let assign3100_e4626: f64 = (var_fn25_calc_iq__qref * var_fn25_calc_iq__eta);
        (assign3100_e4626, (var_fn25_calc_iq__qref * var_fn25_calc_iq__eta_dn2), (var_fn25_calc_iq__qref * var_fn25_calc_iq__eta_dn3), ((var_fn25_calc_iq__qref_dn4 * var_fn25_calc_iq__eta) + (var_fn25_calc_iq__qref * var_fn25_calc_iq__eta_dn4)), (var_fn25_calc_iq__qref * var_fn25_calc_iq__eta_dn7), ((var_fn25_calc_iq__qref_dn16 * var_fn25_calc_iq__eta) + (var_fn25_calc_iq__qref * var_fn25_calc_iq__eta_dn16)), ((var_fn25_calc_iq__qref_dn17 * var_fn25_calc_iq__eta) + (var_fn25_calc_iq__qref * var_fn25_calc_iq__eta_dn17)),)
    } else {
        (var_fn25_calc_iq__qinvv, var_fn25_calc_iq__qinvv_dn2, var_fn25_calc_iq__qinvv_dn3, var_fn25_calc_iq__qinvv_dn4, var_fn25_calc_iq__qinvv_dn7, var_fn25_calc_iq__qinvv_dn16, var_fn25_calc_iq__qinvv_dn17,)
    }
};
        var_fn25_calc_iq__qinvv = assign3100_e4628;
        var_fn25_calc_iq__qinvv_dn2 = assign3100_e4628_d_n2;
        var_fn25_calc_iq__qinvv_dn3 = assign3100_e4628_d_n3;
        var_fn25_calc_iq__qinvv_dn4 = assign3100_e4628_d_n4;
        var_fn25_calc_iq__qinvv_dn7 = assign3100_e4628_d_n7;
        var_fn25_calc_iq__qinvv_dn16 = assign3100_e4628_d_n16;
        var_fn25_calc_iq__qinvv_dn17 = assign3100_e4628_d_n17;
        var_fn25_calc_iq__qinvv_rv = 0.0;

        let assign3110_e4631: f64 = (-50.0);
        let assign3110_e4632: f64 = if var_fn25_calc_iq__eta < assign3110_e4631 { 1.0 } else { 0.0 };
        var_guard30 = assign3110_e4632;
        var_guard30_rv = 0.0;

        let (assign3120_e4644, assign3120_e4644_d_n2, assign3120_e4644_d_n3, assign3120_e4644_d_n4, assign3120_e4644_d_n7, assign3120_e4644_d_n16, assign3120_e4644_d_n17,) = {
    if (((var_guard24 != 0.0) && (var_guard29 == 0.0)) && (var_guard30 != 0.0)) {
        let assign3120_e4641: f64 = (var_fn25_calc_iq__eta).exp();
        let assign3120_e4642: f64 = (var_fn25_calc_iq__qref * assign3120_e4641);
        (assign3120_e4642, (var_fn25_calc_iq__qref * (assign3120_e4641 * var_fn25_calc_iq__eta_dn2)), (var_fn25_calc_iq__qref * (assign3120_e4641 * var_fn25_calc_iq__eta_dn3)), ((var_fn25_calc_iq__qref_dn4 * assign3120_e4641) + (var_fn25_calc_iq__qref * (assign3120_e4641 * var_fn25_calc_iq__eta_dn4))), (var_fn25_calc_iq__qref * (assign3120_e4641 * var_fn25_calc_iq__eta_dn7)), ((var_fn25_calc_iq__qref_dn16 * assign3120_e4641) + (var_fn25_calc_iq__qref * (assign3120_e4641 * var_fn25_calc_iq__eta_dn16))), ((var_fn25_calc_iq__qref_dn17 * assign3120_e4641) + (var_fn25_calc_iq__qref * (assign3120_e4641 * var_fn25_calc_iq__eta_dn17))),)
    } else {
        (var_fn25_calc_iq__qinvv, var_fn25_calc_iq__qinvv_dn2, var_fn25_calc_iq__qinvv_dn3, var_fn25_calc_iq__qinvv_dn4, var_fn25_calc_iq__qinvv_dn7, var_fn25_calc_iq__qinvv_dn16, var_fn25_calc_iq__qinvv_dn17,)
    }
};
        var_fn25_calc_iq__qinvv = assign3120_e4644;
        var_fn25_calc_iq__qinvv_dn2 = assign3120_e4644_d_n2;
        var_fn25_calc_iq__qinvv_dn3 = assign3120_e4644_d_n3;
        var_fn25_calc_iq__qinvv_dn4 = assign3120_e4644_d_n4;
        var_fn25_calc_iq__qinvv_dn7 = assign3120_e4644_d_n7;
        var_fn25_calc_iq__qinvv_dn16 = assign3120_e4644_d_n16;
        var_fn25_calc_iq__qinvv_dn17 = assign3120_e4644_d_n17;
        var_fn25_calc_iq__qinvv_rv = 0.0;

        let (assign3130_e4660, assign3130_e4660_d_n2, assign3130_e4660_d_n3, assign3130_e4660_d_n4, assign3130_e4660_d_n7, assign3130_e4660_d_n16, assign3130_e4660_d_n17,) = {
    if (((var_guard24 != 0.0) && (var_guard29 == 0.0)) && (var_guard30 == 0.0)) {
        let assign3130_e4655: f64 = (var_fn25_calc_iq__eta).exp();
        let assign3130_e4656: f64 = (1.0 + assign3130_e4655);
        let assign3130_e4657: f64 = (assign3130_e4656).ln();
        let assign3130_e4658: f64 = (var_fn25_calc_iq__qref * assign3130_e4657);
        (assign3130_e4658, (var_fn25_calc_iq__qref * ((assign3130_e4655 * var_fn25_calc_iq__eta_dn2) / assign3130_e4656)), (var_fn25_calc_iq__qref * ((assign3130_e4655 * var_fn25_calc_iq__eta_dn3) / assign3130_e4656)), ((var_fn25_calc_iq__qref_dn4 * assign3130_e4657) + (var_fn25_calc_iq__qref * ((assign3130_e4655 * var_fn25_calc_iq__eta_dn4) / assign3130_e4656))), (var_fn25_calc_iq__qref * ((assign3130_e4655 * var_fn25_calc_iq__eta_dn7) / assign3130_e4656)), ((var_fn25_calc_iq__qref_dn16 * assign3130_e4657) + (var_fn25_calc_iq__qref * ((assign3130_e4655 * var_fn25_calc_iq__eta_dn16) / assign3130_e4656))), ((var_fn25_calc_iq__qref_dn17 * assign3130_e4657) + (var_fn25_calc_iq__qref * ((assign3130_e4655 * var_fn25_calc_iq__eta_dn17) / assign3130_e4656))),)
    } else {
        (var_fn25_calc_iq__qinvv, var_fn25_calc_iq__qinvv_dn2, var_fn25_calc_iq__qinvv_dn3, var_fn25_calc_iq__qinvv_dn4, var_fn25_calc_iq__qinvv_dn7, var_fn25_calc_iq__qinvv_dn16, var_fn25_calc_iq__qinvv_dn17,)
    }
};
        var_fn25_calc_iq__qinvv = assign3130_e4660;
        var_fn25_calc_iq__qinvv_dn2 = assign3130_e4660_d_n2;
        var_fn25_calc_iq__qinvv_dn3 = assign3130_e4660_d_n3;
        var_fn25_calc_iq__qinvv_dn4 = assign3130_e4660_d_n4;
        var_fn25_calc_iq__qinvv_dn7 = assign3130_e4660_d_n7;
        var_fn25_calc_iq__qinvv_dn16 = assign3130_e4660_d_n16;
        var_fn25_calc_iq__qinvv_dn17 = assign3130_e4660_d_n17;
        var_fn25_calc_iq__qinvv_rv = 0.0;

        let (assign3140_e4674, assign3140_e4674_d_n2, assign3140_e4674_d_n3, assign3140_e4674_d_n4, assign3140_e4674_d_n7, assign3140_e4674_d_n16, assign3140_e4674_d_n17,) = {
    if (var_guard24 != 0.0) {
        let assign3140_e4667: f64 = (var_fn25_calc_iq__mtheta * var_fn25_calc_iq__qinvv);
        let assign3140_e4669: f64 = (assign3140_e4667 / var_fn25_calc_iq__cgin);
        let assign3140_e4670: f64 = (1.0 + assign3140_e4669);
        let assign3140_e4671: f64 = (var_fn25_calc_iq__tfacmobin * assign3140_e4670);
        let assign3140_e4672: f64 = (var_fn25_calc_iq__mu0 / assign3140_e4671);
        (assign3140_e4672, (-((var_fn25_calc_iq__mu0 * (var_fn25_calc_iq__tfacmobin * ((var_fn25_calc_iq__mtheta * var_fn25_calc_iq__qinvv_dn2) / var_fn25_calc_iq__cgin))) / (assign3140_e4671 * assign3140_e4671))), (-((var_fn25_calc_iq__mu0 * (var_fn25_calc_iq__tfacmobin * ((var_fn25_calc_iq__mtheta * var_fn25_calc_iq__qinvv_dn3) / var_fn25_calc_iq__cgin))) / (assign3140_e4671 * assign3140_e4671))), (-((var_fn25_calc_iq__mu0 * ((var_fn25_calc_iq__tfacmobin_dn4 * assign3140_e4670) + (var_fn25_calc_iq__tfacmobin * ((((var_fn25_calc_iq__mtheta * var_fn25_calc_iq__qinvv_dn4) * var_fn25_calc_iq__cgin) - (assign3140_e4667 * var_fn25_calc_iq__cgin_dn4)) / (var_fn25_calc_iq__cgin * var_fn25_calc_iq__cgin))))) / (assign3140_e4671 * assign3140_e4671))), (-((var_fn25_calc_iq__mu0 * (var_fn25_calc_iq__tfacmobin * ((var_fn25_calc_iq__mtheta * var_fn25_calc_iq__qinvv_dn7) / var_fn25_calc_iq__cgin))) / (assign3140_e4671 * assign3140_e4671))), (-((var_fn25_calc_iq__mu0 * (var_fn25_calc_iq__tfacmobin * ((var_fn25_calc_iq__mtheta * var_fn25_calc_iq__qinvv_dn16) / var_fn25_calc_iq__cgin))) / (assign3140_e4671 * assign3140_e4671))), (-((var_fn25_calc_iq__mu0 * (var_fn25_calc_iq__tfacmobin * ((var_fn25_calc_iq__mtheta * var_fn25_calc_iq__qinvv_dn17) / var_fn25_calc_iq__cgin))) / (assign3140_e4671 * assign3140_e4671))),)
    } else {
        (var_fn25_calc_iq__muf, var_fn25_calc_iq__muf_dn2, var_fn25_calc_iq__muf_dn3, var_fn25_calc_iq__muf_dn4, var_fn25_calc_iq__muf_dn7, var_fn25_calc_iq__muf_dn16, var_fn25_calc_iq__muf_dn17,)
    }
};
        var_fn25_calc_iq__muf = assign3140_e4674;
        var_fn25_calc_iq__muf_dn2 = assign3140_e4674_d_n2;
        var_fn25_calc_iq__muf_dn3 = assign3140_e4674_d_n3;
        var_fn25_calc_iq__muf_dn4 = assign3140_e4674_d_n4;
        var_fn25_calc_iq__muf_dn7 = assign3140_e4674_d_n7;
        var_fn25_calc_iq__muf_dn16 = assign3140_e4674_d_n16;
        var_fn25_calc_iq__muf_dn17 = assign3140_e4674_d_n17;
        var_fn25_calc_iq__muf_rv = 0.0;

        let (assign3150_e4706, assign3150_e4706_d_n2, assign3150_e4706_d_n3, assign3150_e4706_d_n4, assign3150_e4706_d_n7, assign3150_e4706_d_n16, assign3150_e4706_d_n17,) = {
    if (var_guard24 != 0.0) {
        let assign3150_e4680: f64 = (var_fn25_calc_iq__vzeta * var_fn25_calc_iq__tnomin);
        let assign3150_e4681: f64 = (1.0 + assign3150_e4680);
        let assign3150_e4685: f64 = (var_fn25_calc_iq__vzeta * var_fn25_calc_iq__tambin);
        let assign3150_e4686: f64 = (1.0 + assign3150_e4685);
        let assign3150_e4687: f64 = (assign3150_e4681 / assign3150_e4686);
        let assign3150_e4688: f64 = (var_fn25_calc_iq__vel0 * assign3150_e4687);
        let assign3150_e4692: f64 = (var_fn25_calc_iq__lambda * var_fn25_calc_iq__absvdsin);
        let assign3150_e4694: f64 = (assign3150_e4692 / var_fn25_calc_iq__lin);
        let assign3150_e4695: f64 = (1.0 + assign3150_e4694);
        let assign3150_e4696: f64 = (assign3150_e4688 * assign3150_e4695);
        let assign3150_e4700: f64 = (var_fn25_calc_iq__vtheta * var_fn25_calc_iq__qinvv);
        let assign3150_e4702: f64 = (assign3150_e4700 / var_fn25_calc_iq__cgin);
        let assign3150_e4703: f64 = (1.0 + assign3150_e4702);
        let assign3150_e4704: f64 = (assign3150_e4696 / assign3150_e4703);
        (assign3150_e4704, (-((assign3150_e4696 * ((var_fn25_calc_iq__vtheta * var_fn25_calc_iq__qinvv_dn2) / var_fn25_calc_iq__cgin)) / (assign3150_e4703 * assign3150_e4703))), (-((assign3150_e4696 * ((var_fn25_calc_iq__vtheta * var_fn25_calc_iq__qinvv_dn3) / var_fn25_calc_iq__cgin)) / (assign3150_e4703 * assign3150_e4703))), (((((var_fn25_calc_iq__vel0 * (-((assign3150_e4681 * (var_fn25_calc_iq__vzeta * var_fn25_calc_iq__tambin_dn4)) / (assign3150_e4686 * assign3150_e4686)))) * assign3150_e4695) * assign3150_e4703) - (assign3150_e4696 * ((((var_fn25_calc_iq__vtheta * var_fn25_calc_iq__qinvv_dn4) * var_fn25_calc_iq__cgin) - (assign3150_e4700 * var_fn25_calc_iq__cgin_dn4)) / (var_fn25_calc_iq__cgin * var_fn25_calc_iq__cgin)))) / (assign3150_e4703 * assign3150_e4703)), (-((assign3150_e4696 * ((var_fn25_calc_iq__vtheta * var_fn25_calc_iq__qinvv_dn7) / var_fn25_calc_iq__cgin)) / (assign3150_e4703 * assign3150_e4703))), ((((assign3150_e4688 * ((var_fn25_calc_iq__lambda * var_fn25_calc_iq__absvdsin_dn16) / var_fn25_calc_iq__lin)) * assign3150_e4703) - (assign3150_e4696 * ((var_fn25_calc_iq__vtheta * var_fn25_calc_iq__qinvv_dn16) / var_fn25_calc_iq__cgin))) / (assign3150_e4703 * assign3150_e4703)), ((((assign3150_e4688 * ((var_fn25_calc_iq__lambda * var_fn25_calc_iq__absvdsin_dn17) / var_fn25_calc_iq__lin)) * assign3150_e4703) - (assign3150_e4696 * ((var_fn25_calc_iq__vtheta * var_fn25_calc_iq__qinvv_dn17) / var_fn25_calc_iq__cgin))) / (assign3150_e4703 * assign3150_e4703)),)
    } else {
        (var_fn25_calc_iq__vx, var_fn25_calc_iq__vx_dn2, var_fn25_calc_iq__vx_dn3, var_fn25_calc_iq__vx_dn4, var_fn25_calc_iq__vx_dn7, var_fn25_calc_iq__vx_dn16, var_fn25_calc_iq__vx_dn17,)
    }
};
        var_fn25_calc_iq__vx = assign3150_e4706;
        var_fn25_calc_iq__vx_dn2 = assign3150_e4706_d_n2;
        var_fn25_calc_iq__vx_dn3 = assign3150_e4706_d_n3;
        var_fn25_calc_iq__vx_dn4 = assign3150_e4706_d_n4;
        var_fn25_calc_iq__vx_dn7 = assign3150_e4706_d_n7;
        var_fn25_calc_iq__vx_dn16 = assign3150_e4706_d_n16;
        var_fn25_calc_iq__vx_dn17 = assign3150_e4706_d_n17;
        var_fn25_calc_iq__vx_rv = 0.0;

        let (assign3170_e4732, assign3170_e4732_d_n2, assign3170_e4732_d_n3, assign3170_e4732_d_n4, assign3170_e4732_d_n7, assign3170_e4732_d_n16, assign3170_e4732_d_n17,) = {
    if (var_guard24 != 0.0) {
        let assign3170_e4728: f64 = (var_fn25_calc_iq__vx * var_fn25_calc_iq__lin);
        let assign3170_e4730: f64 = (assign3170_e4728 / var_fn25_calc_iq__muf);
        (assign3170_e4730, ((((var_fn25_calc_iq__vx_dn2 * var_fn25_calc_iq__lin) * var_fn25_calc_iq__muf) - (assign3170_e4728 * var_fn25_calc_iq__muf_dn2)) / (var_fn25_calc_iq__muf * var_fn25_calc_iq__muf)), ((((var_fn25_calc_iq__vx_dn3 * var_fn25_calc_iq__lin) * var_fn25_calc_iq__muf) - (assign3170_e4728 * var_fn25_calc_iq__muf_dn3)) / (var_fn25_calc_iq__muf * var_fn25_calc_iq__muf)), ((((var_fn25_calc_iq__vx_dn4 * var_fn25_calc_iq__lin) * var_fn25_calc_iq__muf) - (assign3170_e4728 * var_fn25_calc_iq__muf_dn4)) / (var_fn25_calc_iq__muf * var_fn25_calc_iq__muf)), ((((var_fn25_calc_iq__vx_dn7 * var_fn25_calc_iq__lin) * var_fn25_calc_iq__muf) - (assign3170_e4728 * var_fn25_calc_iq__muf_dn7)) / (var_fn25_calc_iq__muf * var_fn25_calc_iq__muf)), ((((var_fn25_calc_iq__vx_dn16 * var_fn25_calc_iq__lin) * var_fn25_calc_iq__muf) - (assign3170_e4728 * var_fn25_calc_iq__muf_dn16)) / (var_fn25_calc_iq__muf * var_fn25_calc_iq__muf)), ((((var_fn25_calc_iq__vx_dn17 * var_fn25_calc_iq__lin) * var_fn25_calc_iq__muf) - (assign3170_e4728 * var_fn25_calc_iq__muf_dn17)) / (var_fn25_calc_iq__muf * var_fn25_calc_iq__muf)),)
    } else {
        (var_fn25_calc_iq__vdsats, var_fn25_calc_iq__vdsats_dn2, var_fn25_calc_iq__vdsats_dn3, var_fn25_calc_iq__vdsats_dn4, var_fn25_calc_iq__vdsats_dn7, var_fn25_calc_iq__vdsats_dn16, var_fn25_calc_iq__vdsats_dn17,)
    }
};
        var_fn25_calc_iq__vdsats = assign3170_e4732;
        var_fn25_calc_iq__vdsats_dn2 = assign3170_e4732_d_n2;
        var_fn25_calc_iq__vdsats_dn3 = assign3170_e4732_d_n3;
        var_fn25_calc_iq__vdsats_dn4 = assign3170_e4732_d_n4;
        var_fn25_calc_iq__vdsats_dn7 = assign3170_e4732_d_n7;
        var_fn25_calc_iq__vdsats_dn16 = assign3170_e4732_d_n16;
        var_fn25_calc_iq__vdsats_dn17 = assign3170_e4732_d_n17;
        var_fn25_calc_iq__vdsats_rv = 0.0;

        let (assign3180_e4749, assign3180_e4749_d_n2, assign3180_e4749_d_n3, assign3180_e4749_d_n4, assign3180_e4749_d_n7, assign3180_e4749_d_n16, assign3180_e4749_d_n17,) = {
    if (var_guard24 != 0.0) {
        let assign3180_e4738: f64 = (2.0 * var_fn25_calc_iq__qinvv);
        let assign3180_e4740: f64 = (assign3180_e4738 / var_fn25_calc_iq__cgin);
        let assign3180_e4742: f64 = (assign3180_e4740 / var_fn25_calc_iq__vdsats);
        let assign3180_e4743: f64 = (1.0 + assign3180_e4742);
        let assign3180_e4744: f64 = (assign3180_e4743).sqrt();
        let assign3180_e4745: f64 = (var_fn25_calc_iq__vdsats * assign3180_e4744);
        let assign3180_e4747: f64 = (assign3180_e4745 - var_fn25_calc_iq__vdsats);
        (assign3180_e4747, (((var_fn25_calc_iq__vdsats_dn2 * assign3180_e4744) + (var_fn25_calc_iq__vdsats * ((((((2.0 * var_fn25_calc_iq__qinvv_dn2) / var_fn25_calc_iq__cgin) * var_fn25_calc_iq__vdsats) - (assign3180_e4740 * var_fn25_calc_iq__vdsats_dn2)) / (var_fn25_calc_iq__vdsats * var_fn25_calc_iq__vdsats)) / (2.0 * assign3180_e4744)))) - var_fn25_calc_iq__vdsats_dn2), (((var_fn25_calc_iq__vdsats_dn3 * assign3180_e4744) + (var_fn25_calc_iq__vdsats * ((((((2.0 * var_fn25_calc_iq__qinvv_dn3) / var_fn25_calc_iq__cgin) * var_fn25_calc_iq__vdsats) - (assign3180_e4740 * var_fn25_calc_iq__vdsats_dn3)) / (var_fn25_calc_iq__vdsats * var_fn25_calc_iq__vdsats)) / (2.0 * assign3180_e4744)))) - var_fn25_calc_iq__vdsats_dn3), (((var_fn25_calc_iq__vdsats_dn4 * assign3180_e4744) + (var_fn25_calc_iq__vdsats * ((((((((2.0 * var_fn25_calc_iq__qinvv_dn4) * var_fn25_calc_iq__cgin) - (assign3180_e4738 * var_fn25_calc_iq__cgin_dn4)) / (var_fn25_calc_iq__cgin * var_fn25_calc_iq__cgin)) * var_fn25_calc_iq__vdsats) - (assign3180_e4740 * var_fn25_calc_iq__vdsats_dn4)) / (var_fn25_calc_iq__vdsats * var_fn25_calc_iq__vdsats)) / (2.0 * assign3180_e4744)))) - var_fn25_calc_iq__vdsats_dn4), (((var_fn25_calc_iq__vdsats_dn7 * assign3180_e4744) + (var_fn25_calc_iq__vdsats * ((((((2.0 * var_fn25_calc_iq__qinvv_dn7) / var_fn25_calc_iq__cgin) * var_fn25_calc_iq__vdsats) - (assign3180_e4740 * var_fn25_calc_iq__vdsats_dn7)) / (var_fn25_calc_iq__vdsats * var_fn25_calc_iq__vdsats)) / (2.0 * assign3180_e4744)))) - var_fn25_calc_iq__vdsats_dn7), (((var_fn25_calc_iq__vdsats_dn16 * assign3180_e4744) + (var_fn25_calc_iq__vdsats * ((((((2.0 * var_fn25_calc_iq__qinvv_dn16) / var_fn25_calc_iq__cgin) * var_fn25_calc_iq__vdsats) - (assign3180_e4740 * var_fn25_calc_iq__vdsats_dn16)) / (var_fn25_calc_iq__vdsats * var_fn25_calc_iq__vdsats)) / (2.0 * assign3180_e4744)))) - var_fn25_calc_iq__vdsats_dn16), (((var_fn25_calc_iq__vdsats_dn17 * assign3180_e4744) + (var_fn25_calc_iq__vdsats * ((((((2.0 * var_fn25_calc_iq__qinvv_dn17) / var_fn25_calc_iq__cgin) * var_fn25_calc_iq__vdsats) - (assign3180_e4740 * var_fn25_calc_iq__vdsats_dn17)) / (var_fn25_calc_iq__vdsats * var_fn25_calc_iq__vdsats)) / (2.0 * assign3180_e4744)))) - var_fn25_calc_iq__vdsats_dn17),)
    } else {
        (var_fn25_calc_iq__vdsats1, var_fn25_calc_iq__vdsats1_dn2, var_fn25_calc_iq__vdsats1_dn3, var_fn25_calc_iq__vdsats1_dn4, var_fn25_calc_iq__vdsats1_dn7, var_fn25_calc_iq__vdsats1_dn16, var_fn25_calc_iq__vdsats1_dn17,)
    }
};
        var_fn25_calc_iq__vdsats1 = assign3180_e4749;
        var_fn25_calc_iq__vdsats1_dn2 = assign3180_e4749_d_n2;
        var_fn25_calc_iq__vdsats1_dn3 = assign3180_e4749_d_n3;
        var_fn25_calc_iq__vdsats1_dn4 = assign3180_e4749_d_n4;
        var_fn25_calc_iq__vdsats1_dn7 = assign3180_e4749_d_n7;
        var_fn25_calc_iq__vdsats1_dn16 = assign3180_e4749_d_n16;
        var_fn25_calc_iq__vdsats1_dn17 = assign3180_e4749_d_n17;
        var_fn25_calc_iq__vdsats1_rv = 0.0;

        let (assign3190_e4761, assign3190_e4761_d_n2, assign3190_e4761_d_n3, assign3190_e4761_d_n4, assign3190_e4761_d_n7, assign3190_e4761_d_n16, assign3190_e4761_d_n17,) = {
    if (var_guard24 != 0.0) {
        let assign3190_e4754: f64 = (1.0 - var_fn25_calc_iq__ff);
        let assign3190_e4755: f64 = (var_fn25_calc_iq__vdsats * assign3190_e4754);
        let assign3190_e4758: f64 = (var_fn25_calc_iq__two_n_phit * var_fn25_calc_iq__ff);
        let assign3190_e4759: f64 = (assign3190_e4755 + assign3190_e4758);
        (assign3190_e4759, (((var_fn25_calc_iq__vdsats_dn2 * assign3190_e4754) + (var_fn25_calc_iq__vdsats * (-var_fn25_calc_iq__ff_dn2))) + (var_fn25_calc_iq__two_n_phit * var_fn25_calc_iq__ff_dn2)), (((var_fn25_calc_iq__vdsats_dn3 * assign3190_e4754) + (var_fn25_calc_iq__vdsats * (-var_fn25_calc_iq__ff_dn3))) + (var_fn25_calc_iq__two_n_phit * var_fn25_calc_iq__ff_dn3)), (((var_fn25_calc_iq__vdsats_dn4 * assign3190_e4754) + (var_fn25_calc_iq__vdsats * (-var_fn25_calc_iq__ff_dn4))) + ((var_fn25_calc_iq__two_n_phit_dn4 * var_fn25_calc_iq__ff) + (var_fn25_calc_iq__two_n_phit * var_fn25_calc_iq__ff_dn4))), (((var_fn25_calc_iq__vdsats_dn7 * assign3190_e4754) + (var_fn25_calc_iq__vdsats * (-var_fn25_calc_iq__ff_dn7))) + (var_fn25_calc_iq__two_n_phit * var_fn25_calc_iq__ff_dn7)), (((var_fn25_calc_iq__vdsats_dn16 * assign3190_e4754) + (var_fn25_calc_iq__vdsats * (-var_fn25_calc_iq__ff_dn16))) + ((var_fn25_calc_iq__two_n_phit_dn16 * var_fn25_calc_iq__ff) + (var_fn25_calc_iq__two_n_phit * var_fn25_calc_iq__ff_dn16))), (((var_fn25_calc_iq__vdsats_dn17 * assign3190_e4754) + (var_fn25_calc_iq__vdsats * (-var_fn25_calc_iq__ff_dn17))) + ((var_fn25_calc_iq__two_n_phit_dn17 * var_fn25_calc_iq__ff) + (var_fn25_calc_iq__two_n_phit * var_fn25_calc_iq__ff_dn17))),)
    } else {
        (var_fn25_calc_iq__vdsat, var_fn25_calc_iq__vdsat_dn2, var_fn25_calc_iq__vdsat_dn3, var_fn25_calc_iq__vdsat_dn4, var_fn25_calc_iq__vdsat_dn7, var_fn25_calc_iq__vdsat_dn16, var_fn25_calc_iq__vdsat_dn17,)
    }
};
        var_fn25_calc_iq__vdsat = assign3190_e4761;
        var_fn25_calc_iq__vdsat_dn2 = assign3190_e4761_d_n2;
        var_fn25_calc_iq__vdsat_dn3 = assign3190_e4761_d_n3;
        var_fn25_calc_iq__vdsat_dn4 = assign3190_e4761_d_n4;
        var_fn25_calc_iq__vdsat_dn7 = assign3190_e4761_d_n7;
        var_fn25_calc_iq__vdsat_dn16 = assign3190_e4761_d_n16;
        var_fn25_calc_iq__vdsat_dn17 = assign3190_e4761_d_n17;
        var_fn25_calc_iq__vdsat_rv = 0.0;

        let (assign3200_e4773, assign3200_e4773_d_n2, assign3200_e4773_d_n3, assign3200_e4773_d_n4, assign3200_e4773_d_n7, assign3200_e4773_d_n16, assign3200_e4773_d_n17,) = {
    if (var_guard24 != 0.0) {
        let assign3200_e4766: f64 = (1.0 - var_fn25_calc_iq__ff);
        let assign3200_e4767: f64 = (var_fn25_calc_iq__vdsats1 * assign3200_e4766);
        let assign3200_e4770: f64 = (var_fn25_calc_iq__two_n_phit * var_fn25_calc_iq__ff);
        let assign3200_e4771: f64 = (assign3200_e4767 + assign3200_e4770);
        (assign3200_e4771, (((var_fn25_calc_iq__vdsats1_dn2 * assign3200_e4766) + (var_fn25_calc_iq__vdsats1 * (-var_fn25_calc_iq__ff_dn2))) + (var_fn25_calc_iq__two_n_phit * var_fn25_calc_iq__ff_dn2)), (((var_fn25_calc_iq__vdsats1_dn3 * assign3200_e4766) + (var_fn25_calc_iq__vdsats1 * (-var_fn25_calc_iq__ff_dn3))) + (var_fn25_calc_iq__two_n_phit * var_fn25_calc_iq__ff_dn3)), (((var_fn25_calc_iq__vdsats1_dn4 * assign3200_e4766) + (var_fn25_calc_iq__vdsats1 * (-var_fn25_calc_iq__ff_dn4))) + ((var_fn25_calc_iq__two_n_phit_dn4 * var_fn25_calc_iq__ff) + (var_fn25_calc_iq__two_n_phit * var_fn25_calc_iq__ff_dn4))), (((var_fn25_calc_iq__vdsats1_dn7 * assign3200_e4766) + (var_fn25_calc_iq__vdsats1 * (-var_fn25_calc_iq__ff_dn7))) + (var_fn25_calc_iq__two_n_phit * var_fn25_calc_iq__ff_dn7)), (((var_fn25_calc_iq__vdsats1_dn16 * assign3200_e4766) + (var_fn25_calc_iq__vdsats1 * (-var_fn25_calc_iq__ff_dn16))) + ((var_fn25_calc_iq__two_n_phit_dn16 * var_fn25_calc_iq__ff) + (var_fn25_calc_iq__two_n_phit * var_fn25_calc_iq__ff_dn16))), (((var_fn25_calc_iq__vdsats1_dn17 * assign3200_e4766) + (var_fn25_calc_iq__vdsats1 * (-var_fn25_calc_iq__ff_dn17))) + ((var_fn25_calc_iq__two_n_phit_dn17 * var_fn25_calc_iq__ff) + (var_fn25_calc_iq__two_n_phit * var_fn25_calc_iq__ff_dn17))),)
    } else {
        (var_fn25_calc_iq__vdsat1, var_fn25_calc_iq__vdsat1_dn2, var_fn25_calc_iq__vdsat1_dn3, var_fn25_calc_iq__vdsat1_dn4, var_fn25_calc_iq__vdsat1_dn7, var_fn25_calc_iq__vdsat1_dn16, var_fn25_calc_iq__vdsat1_dn17,)
    }
};
        var_fn25_calc_iq__vdsat1 = assign3200_e4773;
        var_fn25_calc_iq__vdsat1_dn2 = assign3200_e4773_d_n2;
        var_fn25_calc_iq__vdsat1_dn3 = assign3200_e4773_d_n3;
        var_fn25_calc_iq__vdsat1_dn4 = assign3200_e4773_d_n4;
        var_fn25_calc_iq__vdsat1_dn7 = assign3200_e4773_d_n7;
        var_fn25_calc_iq__vdsat1_dn16 = assign3200_e4773_d_n16;
        var_fn25_calc_iq__vdsat1_dn17 = assign3200_e4773_d_n17;
        var_fn25_calc_iq__vdsat1_rv = 0.0;

        let (assign3210_e4842, assign3210_e4842_d_n2, assign3210_e4842_d_n3, assign3210_e4842_d_n4, assign3210_e4842_d_n7, assign3210_e4842_d_n16, assign3210_e4842_d_n17,) = {
    if (var_guard24 != 0.0) {
        let (assign3210_e4832, assign3210_e4832_d_n2, assign3210_e4832_d_n3, assign3210_e4832_d_n4, assign3210_e4832_d_n7, assign3210_e4832_d_n16, assign3210_e4832_d_n17,) = {
            if (p.p52 != 0.0) {
                let assign3210_e4785: f64 = (var_fn25_calc_iq__vdsin / var_fn25_calc_iq__vdsat1);
                let assign3210_e4786: f64 = assign3210_e4785;
                let assign3210_e4790: f64 = (var_fn25_calc_iq__vdsin / var_fn25_calc_iq__vdsat1);
                let assign3210_e4791: f64 = (-assign3210_e4790);
                let assign3210_e4794: f64 = (0.001 / p.p53);
                let assign3210_e4798: f64 = (var_fn25_calc_iq__vdsin / var_fn25_calc_iq__vdsat1);
                let assign3210_e4799: f64 = (-assign3210_e4798);
                let assign3210_e4800: f64 = (assign3210_e4794 * assign3210_e4799);
                let assign3210_e4801: f64 = (assign3210_e4800).tanh();
                let assign3210_e4802: f64 = (assign3210_e4791 * assign3210_e4801);
                let assign3210_e4803: f64 = (assign3210_e4786 + assign3210_e4802);
                let assign3210_e4804: f64 = (0.5 * assign3210_e4803);
                (assign3210_e4804, (0.5 * ((-((var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat1_dn2) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1))) + (((-(-((var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat1_dn2) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1)))) * assign3210_e4801) + (assign3210_e4791 * ((assign3210_e4794 * (-(-((var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat1_dn2) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1))))) / ((assign3210_e4800).cosh() * (assign3210_e4800).cosh())))))), (0.5 * ((-((var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat1_dn3) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1))) + (((-(-((var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat1_dn3) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1)))) * assign3210_e4801) + (assign3210_e4791 * ((assign3210_e4794 * (-(-((var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat1_dn3) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1))))) / ((assign3210_e4800).cosh() * (assign3210_e4800).cosh())))))), (0.5 * ((-((var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat1_dn4) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1))) + (((-(-((var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat1_dn4) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1)))) * assign3210_e4801) + (assign3210_e4791 * ((assign3210_e4794 * (-(-((var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat1_dn4) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1))))) / ((assign3210_e4800).cosh() * (assign3210_e4800).cosh())))))), (0.5 * ((-((var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat1_dn7) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1))) + (((-(-((var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat1_dn7) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1)))) * assign3210_e4801) + (assign3210_e4791 * ((assign3210_e4794 * (-(-((var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat1_dn7) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1))))) / ((assign3210_e4800).cosh() * (assign3210_e4800).cosh())))))), (0.5 * ((((var_fn25_calc_iq__vdsin_dn16 * var_fn25_calc_iq__vdsat1) - (var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat1_dn16)) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1)) + (((-(((var_fn25_calc_iq__vdsin_dn16 * var_fn25_calc_iq__vdsat1) - (var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat1_dn16)) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1))) * assign3210_e4801) + (assign3210_e4791 * ((assign3210_e4794 * (-(((var_fn25_calc_iq__vdsin_dn16 * var_fn25_calc_iq__vdsat1) - (var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat1_dn16)) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1)))) / ((assign3210_e4800).cosh() * (assign3210_e4800).cosh())))))), (0.5 * ((((var_fn25_calc_iq__vdsin_dn17 * var_fn25_calc_iq__vdsat1) - (var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat1_dn17)) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1)) + (((-(((var_fn25_calc_iq__vdsin_dn17 * var_fn25_calc_iq__vdsat1) - (var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat1_dn17)) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1))) * assign3210_e4801) + (assign3210_e4791 * ((assign3210_e4794 * (-(((var_fn25_calc_iq__vdsin_dn17 * var_fn25_calc_iq__vdsat1) - (var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat1_dn17)) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1)))) / ((assign3210_e4800).cosh() * (assign3210_e4800).cosh())))))),)
            } else {
                let (assign3210_e4831, assign3210_e4831_d_n2, assign3210_e4831_d_n3, assign3210_e4831_d_n4, assign3210_e4831_d_n7, assign3210_e4831_d_n16, assign3210_e4831_d_n17,) = {
                    if (p.p52 == 0.0) {
                        let assign3210_e4812: f64 = (var_fn25_calc_iq__vdsin / var_fn25_calc_iq__vdsat1);
                        let assign3210_e4813: f64 = assign3210_e4812;
                        let assign3210_e4817: f64 = (var_fn25_calc_iq__vdsin / var_fn25_calc_iq__vdsat1);
                        let assign3210_e4818: f64 = (-assign3210_e4817);
                        let assign3210_e4822: f64 = (var_fn25_calc_iq__vdsin / var_fn25_calc_iq__vdsat1);
                        let assign3210_e4823: f64 = (-assign3210_e4822);
                        let assign3210_e4824: f64 = (assign3210_e4818 * assign3210_e4823);
                        let assign3210_e4826: f64 = (assign3210_e4824 + p.p53);
                        let assign3210_e4827: f64 = (assign3210_e4826).sqrt();
                        let assign3210_e4828: f64 = (assign3210_e4813 + assign3210_e4827);
                        let assign3210_e4829: f64 = (0.5 * assign3210_e4828);
                        (assign3210_e4829, (0.5 * ((-((var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat1_dn2) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1))) + ((((-(-((var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat1_dn2) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1)))) * assign3210_e4823) + (assign3210_e4818 * (-(-((var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat1_dn2) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1)))))) / (2.0 * assign3210_e4827)))), (0.5 * ((-((var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat1_dn3) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1))) + ((((-(-((var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat1_dn3) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1)))) * assign3210_e4823) + (assign3210_e4818 * (-(-((var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat1_dn3) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1)))))) / (2.0 * assign3210_e4827)))), (0.5 * ((-((var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat1_dn4) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1))) + ((((-(-((var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat1_dn4) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1)))) * assign3210_e4823) + (assign3210_e4818 * (-(-((var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat1_dn4) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1)))))) / (2.0 * assign3210_e4827)))), (0.5 * ((-((var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat1_dn7) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1))) + ((((-(-((var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat1_dn7) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1)))) * assign3210_e4823) + (assign3210_e4818 * (-(-((var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat1_dn7) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1)))))) / (2.0 * assign3210_e4827)))), (0.5 * ((((var_fn25_calc_iq__vdsin_dn16 * var_fn25_calc_iq__vdsat1) - (var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat1_dn16)) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1)) + ((((-(((var_fn25_calc_iq__vdsin_dn16 * var_fn25_calc_iq__vdsat1) - (var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat1_dn16)) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1))) * assign3210_e4823) + (assign3210_e4818 * (-(((var_fn25_calc_iq__vdsin_dn16 * var_fn25_calc_iq__vdsat1) - (var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat1_dn16)) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1))))) / (2.0 * assign3210_e4827)))), (0.5 * ((((var_fn25_calc_iq__vdsin_dn17 * var_fn25_calc_iq__vdsat1) - (var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat1_dn17)) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1)) + ((((-(((var_fn25_calc_iq__vdsin_dn17 * var_fn25_calc_iq__vdsat1) - (var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat1_dn17)) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1))) * assign3210_e4823) + (assign3210_e4818 * (-(((var_fn25_calc_iq__vdsin_dn17 * var_fn25_calc_iq__vdsat1) - (var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat1_dn17)) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1))))) / (2.0 * assign3210_e4827)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign3210_e4831, assign3210_e4831_d_n2, assign3210_e4831_d_n3, assign3210_e4831_d_n4, assign3210_e4831_d_n7, assign3210_e4831_d_n16, assign3210_e4831_d_n17,)
            }
        };
        let assign3210_e4834: f64 = (assign3210_e4832).powf(var_fn25_calc_iq__beta);
        let assign3210_e4835: f64 = (1.0 + assign3210_e4834);
        let assign3210_e4838: f64 = (1.0 / var_fn25_calc_iq__beta);
        let assign3210_e4839: f64 = (assign3210_e4835).powf(assign3210_e4838);
        let assign3210_e4840: f64 = (1.0 / assign3210_e4839);
        (assign3210_e4840, (-(if 0.0 == 0.0 && ((assign3210_e4838) as f64).is_finite() && ((assign3210_e4838) as f64).fract() == 0.0 { if assign3210_e4838 == 0.0 { 0.0 } else { (assign3210_e4838 * ((assign3210_e4835).powf(assign3210_e4838 - 1.0) * if 0.0 == 0.0 && ((var_fn25_calc_iq__beta) as f64).is_finite() && ((var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (var_fn25_calc_iq__beta * ((assign3210_e4832).powf(var_fn25_calc_iq__beta - 1.0) * assign3210_e4832_d_n2)) } } else { (assign3210_e4834 * (var_fn25_calc_iq__beta * (assign3210_e4832_d_n2 / assign3210_e4832))) })) } } else { (assign3210_e4839 * (assign3210_e4838 * (if 0.0 == 0.0 && ((var_fn25_calc_iq__beta) as f64).is_finite() && ((var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (var_fn25_calc_iq__beta * ((assign3210_e4832).powf(var_fn25_calc_iq__beta - 1.0) * assign3210_e4832_d_n2)) } } else { (assign3210_e4834 * (var_fn25_calc_iq__beta * (assign3210_e4832_d_n2 / assign3210_e4832))) } / assign3210_e4835))) } / (assign3210_e4839 * assign3210_e4839))), (-(if 0.0 == 0.0 && ((assign3210_e4838) as f64).is_finite() && ((assign3210_e4838) as f64).fract() == 0.0 { if assign3210_e4838 == 0.0 { 0.0 } else { (assign3210_e4838 * ((assign3210_e4835).powf(assign3210_e4838 - 1.0) * if 0.0 == 0.0 && ((var_fn25_calc_iq__beta) as f64).is_finite() && ((var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (var_fn25_calc_iq__beta * ((assign3210_e4832).powf(var_fn25_calc_iq__beta - 1.0) * assign3210_e4832_d_n3)) } } else { (assign3210_e4834 * (var_fn25_calc_iq__beta * (assign3210_e4832_d_n3 / assign3210_e4832))) })) } } else { (assign3210_e4839 * (assign3210_e4838 * (if 0.0 == 0.0 && ((var_fn25_calc_iq__beta) as f64).is_finite() && ((var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (var_fn25_calc_iq__beta * ((assign3210_e4832).powf(var_fn25_calc_iq__beta - 1.0) * assign3210_e4832_d_n3)) } } else { (assign3210_e4834 * (var_fn25_calc_iq__beta * (assign3210_e4832_d_n3 / assign3210_e4832))) } / assign3210_e4835))) } / (assign3210_e4839 * assign3210_e4839))), (-(if 0.0 == 0.0 && ((assign3210_e4838) as f64).is_finite() && ((assign3210_e4838) as f64).fract() == 0.0 { if assign3210_e4838 == 0.0 { 0.0 } else { (assign3210_e4838 * ((assign3210_e4835).powf(assign3210_e4838 - 1.0) * if 0.0 == 0.0 && ((var_fn25_calc_iq__beta) as f64).is_finite() && ((var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (var_fn25_calc_iq__beta * ((assign3210_e4832).powf(var_fn25_calc_iq__beta - 1.0) * assign3210_e4832_d_n4)) } } else { (assign3210_e4834 * (var_fn25_calc_iq__beta * (assign3210_e4832_d_n4 / assign3210_e4832))) })) } } else { (assign3210_e4839 * (assign3210_e4838 * (if 0.0 == 0.0 && ((var_fn25_calc_iq__beta) as f64).is_finite() && ((var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (var_fn25_calc_iq__beta * ((assign3210_e4832).powf(var_fn25_calc_iq__beta - 1.0) * assign3210_e4832_d_n4)) } } else { (assign3210_e4834 * (var_fn25_calc_iq__beta * (assign3210_e4832_d_n4 / assign3210_e4832))) } / assign3210_e4835))) } / (assign3210_e4839 * assign3210_e4839))), (-(if 0.0 == 0.0 && ((assign3210_e4838) as f64).is_finite() && ((assign3210_e4838) as f64).fract() == 0.0 { if assign3210_e4838 == 0.0 { 0.0 } else { (assign3210_e4838 * ((assign3210_e4835).powf(assign3210_e4838 - 1.0) * if 0.0 == 0.0 && ((var_fn25_calc_iq__beta) as f64).is_finite() && ((var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (var_fn25_calc_iq__beta * ((assign3210_e4832).powf(var_fn25_calc_iq__beta - 1.0) * assign3210_e4832_d_n7)) } } else { (assign3210_e4834 * (var_fn25_calc_iq__beta * (assign3210_e4832_d_n7 / assign3210_e4832))) })) } } else { (assign3210_e4839 * (assign3210_e4838 * (if 0.0 == 0.0 && ((var_fn25_calc_iq__beta) as f64).is_finite() && ((var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (var_fn25_calc_iq__beta * ((assign3210_e4832).powf(var_fn25_calc_iq__beta - 1.0) * assign3210_e4832_d_n7)) } } else { (assign3210_e4834 * (var_fn25_calc_iq__beta * (assign3210_e4832_d_n7 / assign3210_e4832))) } / assign3210_e4835))) } / (assign3210_e4839 * assign3210_e4839))), (-(if 0.0 == 0.0 && ((assign3210_e4838) as f64).is_finite() && ((assign3210_e4838) as f64).fract() == 0.0 { if assign3210_e4838 == 0.0 { 0.0 } else { (assign3210_e4838 * ((assign3210_e4835).powf(assign3210_e4838 - 1.0) * if 0.0 == 0.0 && ((var_fn25_calc_iq__beta) as f64).is_finite() && ((var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (var_fn25_calc_iq__beta * ((assign3210_e4832).powf(var_fn25_calc_iq__beta - 1.0) * assign3210_e4832_d_n16)) } } else { (assign3210_e4834 * (var_fn25_calc_iq__beta * (assign3210_e4832_d_n16 / assign3210_e4832))) })) } } else { (assign3210_e4839 * (assign3210_e4838 * (if 0.0 == 0.0 && ((var_fn25_calc_iq__beta) as f64).is_finite() && ((var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (var_fn25_calc_iq__beta * ((assign3210_e4832).powf(var_fn25_calc_iq__beta - 1.0) * assign3210_e4832_d_n16)) } } else { (assign3210_e4834 * (var_fn25_calc_iq__beta * (assign3210_e4832_d_n16 / assign3210_e4832))) } / assign3210_e4835))) } / (assign3210_e4839 * assign3210_e4839))), (-(if 0.0 == 0.0 && ((assign3210_e4838) as f64).is_finite() && ((assign3210_e4838) as f64).fract() == 0.0 { if assign3210_e4838 == 0.0 { 0.0 } else { (assign3210_e4838 * ((assign3210_e4835).powf(assign3210_e4838 - 1.0) * if 0.0 == 0.0 && ((var_fn25_calc_iq__beta) as f64).is_finite() && ((var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (var_fn25_calc_iq__beta * ((assign3210_e4832).powf(var_fn25_calc_iq__beta - 1.0) * assign3210_e4832_d_n17)) } } else { (assign3210_e4834 * (var_fn25_calc_iq__beta * (assign3210_e4832_d_n17 / assign3210_e4832))) })) } } else { (assign3210_e4839 * (assign3210_e4838 * (if 0.0 == 0.0 && ((var_fn25_calc_iq__beta) as f64).is_finite() && ((var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (var_fn25_calc_iq__beta * ((assign3210_e4832).powf(var_fn25_calc_iq__beta - 1.0) * assign3210_e4832_d_n17)) } } else { (assign3210_e4834 * (var_fn25_calc_iq__beta * (assign3210_e4832_d_n17 / assign3210_e4832))) } / assign3210_e4835))) } / (assign3210_e4839 * assign3210_e4839))),)
    } else {
        (var_fn25_calc_iq__fsd, var_fn25_calc_iq__fsd_dn2, var_fn25_calc_iq__fsd_dn3, var_fn25_calc_iq__fsd_dn4, var_fn25_calc_iq__fsd_dn7, var_fn25_calc_iq__fsd_dn16, var_fn25_calc_iq__fsd_dn17,)
    }
};
        var_fn25_calc_iq__fsd = assign3210_e4842;
        var_fn25_calc_iq__fsd_dn2 = assign3210_e4842_d_n2;
        var_fn25_calc_iq__fsd_dn3 = assign3210_e4842_d_n3;
        var_fn25_calc_iq__fsd_dn4 = assign3210_e4842_d_n4;
        var_fn25_calc_iq__fsd_dn7 = assign3210_e4842_d_n7;
        var_fn25_calc_iq__fsd_dn16 = assign3210_e4842_d_n16;
        var_fn25_calc_iq__fsd_dn17 = assign3210_e4842_d_n17;
        var_fn25_calc_iq__fsd_rv = 0.0;

        let (assign3220_e4848, assign3220_e4848_d_n2, assign3220_e4848_d_n3, assign3220_e4848_d_n4, assign3220_e4848_d_n7, assign3220_e4848_d_n16, assign3220_e4848_d_n17,) = {
    if (var_guard24 != 0.0) {
        let assign3220_e4846: f64 = (var_fn25_calc_iq__vdsin * var_fn25_calc_iq__fsd);
        (assign3220_e4846, (var_fn25_calc_iq__vdsin * var_fn25_calc_iq__fsd_dn2), (var_fn25_calc_iq__vdsin * var_fn25_calc_iq__fsd_dn3), (var_fn25_calc_iq__vdsin * var_fn25_calc_iq__fsd_dn4), (var_fn25_calc_iq__vdsin * var_fn25_calc_iq__fsd_dn7), ((var_fn25_calc_iq__vdsin_dn16 * var_fn25_calc_iq__fsd) + (var_fn25_calc_iq__vdsin * var_fn25_calc_iq__fsd_dn16)), ((var_fn25_calc_iq__vdsin_dn17 * var_fn25_calc_iq__fsd) + (var_fn25_calc_iq__vdsin * var_fn25_calc_iq__fsd_dn17)),)
    } else {
        (var_fn25_calc_iq__vdx, var_fn25_calc_iq__vdx_dn2, var_fn25_calc_iq__vdx_dn3, var_fn25_calc_iq__vdx_dn4, var_fn25_calc_iq__vdx_dn7, var_fn25_calc_iq__vdx_dn16, var_fn25_calc_iq__vdx_dn17,)
    }
};
        var_fn25_calc_iq__vdx = assign3220_e4848;
        var_fn25_calc_iq__vdx_dn2 = assign3220_e4848_d_n2;
        var_fn25_calc_iq__vdx_dn3 = assign3220_e4848_d_n3;
        var_fn25_calc_iq__vdx_dn4 = assign3220_e4848_d_n4;
        var_fn25_calc_iq__vdx_dn7 = assign3220_e4848_d_n7;
        var_fn25_calc_iq__vdx_dn16 = assign3220_e4848_d_n16;
        var_fn25_calc_iq__vdx_dn17 = assign3220_e4848_d_n17;
        var_fn25_calc_iq__vdx_rv = 0.0;

        *var_fn25_calc_iq__delta_slot = var_fn25_calc_iq__delta;
        *var_fn25_calc_iq__delta_dn16_slot = var_fn25_calc_iq__delta_dn16;
        *var_fn25_calc_iq__delta_dn17_slot = var_fn25_calc_iq__delta_dn17;
        *var_fn25_calc_iq__delta_rv_slot = var_fn25_calc_iq__delta_rv;
        *var_fn25_calc_iq__eta_slot = var_fn25_calc_iq__eta;
        *var_fn25_calc_iq__eta_dn16_slot = var_fn25_calc_iq__eta_dn16;
        *var_fn25_calc_iq__eta_dn17_slot = var_fn25_calc_iq__eta_dn17;
        *var_fn25_calc_iq__eta_dn2_slot = var_fn25_calc_iq__eta_dn2;
        *var_fn25_calc_iq__eta_dn3_slot = var_fn25_calc_iq__eta_dn3;
        *var_fn25_calc_iq__eta_dn4_slot = var_fn25_calc_iq__eta_dn4;
        *var_fn25_calc_iq__eta_dn7_slot = var_fn25_calc_iq__eta_dn7;
        *var_fn25_calc_iq__eta_rv_slot = var_fn25_calc_iq__eta_rv;
        *var_fn25_calc_iq__exparg_slot = var_fn25_calc_iq__exparg;
        *var_fn25_calc_iq__exparg_dn16_slot = var_fn25_calc_iq__exparg_dn16;
        *var_fn25_calc_iq__exparg_dn17_slot = var_fn25_calc_iq__exparg_dn17;
        *var_fn25_calc_iq__exparg_dn2_slot = var_fn25_calc_iq__exparg_dn2;
        *var_fn25_calc_iq__exparg_dn3_slot = var_fn25_calc_iq__exparg_dn3;
        *var_fn25_calc_iq__exparg_dn4_slot = var_fn25_calc_iq__exparg_dn4;
        *var_fn25_calc_iq__exparg_dn7_slot = var_fn25_calc_iq__exparg_dn7;
        *var_fn25_calc_iq__exparg_rv_slot = var_fn25_calc_iq__exparg_rv;
        *var_fn25_calc_iq__ff_slot = var_fn25_calc_iq__ff;
        *var_fn25_calc_iq__ff_dn16_slot = var_fn25_calc_iq__ff_dn16;
        *var_fn25_calc_iq__ff_dn17_slot = var_fn25_calc_iq__ff_dn17;
        *var_fn25_calc_iq__ff_dn2_slot = var_fn25_calc_iq__ff_dn2;
        *var_fn25_calc_iq__ff_dn3_slot = var_fn25_calc_iq__ff_dn3;
        *var_fn25_calc_iq__ff_dn4_slot = var_fn25_calc_iq__ff_dn4;
        *var_fn25_calc_iq__ff_dn7_slot = var_fn25_calc_iq__ff_dn7;
        *var_fn25_calc_iq__ff_rv_slot = var_fn25_calc_iq__ff_rv;
        *var_fn25_calc_iq__fsd_slot = var_fn25_calc_iq__fsd;
        *var_fn25_calc_iq__fsd_dn16_slot = var_fn25_calc_iq__fsd_dn16;
        *var_fn25_calc_iq__fsd_dn17_slot = var_fn25_calc_iq__fsd_dn17;
        *var_fn25_calc_iq__fsd_dn2_slot = var_fn25_calc_iq__fsd_dn2;
        *var_fn25_calc_iq__fsd_dn3_slot = var_fn25_calc_iq__fsd_dn3;
        *var_fn25_calc_iq__fsd_dn4_slot = var_fn25_calc_iq__fsd_dn4;
        *var_fn25_calc_iq__fsd_dn7_slot = var_fn25_calc_iq__fsd_dn7;
        *var_fn25_calc_iq__fsd_rv_slot = var_fn25_calc_iq__fsd_rv;
        *var_fn25_calc_iq__muf_slot = var_fn25_calc_iq__muf;
        *var_fn25_calc_iq__muf_dn16_slot = var_fn25_calc_iq__muf_dn16;
        *var_fn25_calc_iq__muf_dn17_slot = var_fn25_calc_iq__muf_dn17;
        *var_fn25_calc_iq__muf_dn2_slot = var_fn25_calc_iq__muf_dn2;
        *var_fn25_calc_iq__muf_dn3_slot = var_fn25_calc_iq__muf_dn3;
        *var_fn25_calc_iq__muf_dn4_slot = var_fn25_calc_iq__muf_dn4;
        *var_fn25_calc_iq__muf_dn7_slot = var_fn25_calc_iq__muf_dn7;
        *var_fn25_calc_iq__muf_rv_slot = var_fn25_calc_iq__muf_rv;
        *var_fn25_calc_iq__myarg_slot = var_fn25_calc_iq__myarg;
        *var_fn25_calc_iq__myarg_dn16_slot = var_fn25_calc_iq__myarg_dn16;
        *var_fn25_calc_iq__myarg_dn17_slot = var_fn25_calc_iq__myarg_dn17;
        *var_fn25_calc_iq__myarg_dn2_slot = var_fn25_calc_iq__myarg_dn2;
        *var_fn25_calc_iq__myarg_dn3_slot = var_fn25_calc_iq__myarg_dn3;
        *var_fn25_calc_iq__myarg_dn4_slot = var_fn25_calc_iq__myarg_dn4;
        *var_fn25_calc_iq__myarg_dn7_slot = var_fn25_calc_iq__myarg_dn7;
        *var_fn25_calc_iq__myarg_rv_slot = var_fn25_calc_iq__myarg_rv;
        *var_fn25_calc_iq__qinvv_slot = var_fn25_calc_iq__qinvv;
        *var_fn25_calc_iq__qinvv_dn16_slot = var_fn25_calc_iq__qinvv_dn16;
        *var_fn25_calc_iq__qinvv_dn17_slot = var_fn25_calc_iq__qinvv_dn17;
        *var_fn25_calc_iq__qinvv_dn2_slot = var_fn25_calc_iq__qinvv_dn2;
        *var_fn25_calc_iq__qinvv_dn3_slot = var_fn25_calc_iq__qinvv_dn3;
        *var_fn25_calc_iq__qinvv_dn4_slot = var_fn25_calc_iq__qinvv_dn4;
        *var_fn25_calc_iq__qinvv_dn7_slot = var_fn25_calc_iq__qinvv_dn7;
        *var_fn25_calc_iq__qinvv_rv_slot = var_fn25_calc_iq__qinvv_rv;
        *var_fn25_calc_iq__qref_slot = var_fn25_calc_iq__qref;
        *var_fn25_calc_iq__qref_dn16_slot = var_fn25_calc_iq__qref_dn16;
        *var_fn25_calc_iq__qref_dn17_slot = var_fn25_calc_iq__qref_dn17;
        *var_fn25_calc_iq__qref_dn4_slot = var_fn25_calc_iq__qref_dn4;
        *var_fn25_calc_iq__qref_rv_slot = var_fn25_calc_iq__qref_rv;
        *var_fn25_calc_iq__two_n_phit_slot = var_fn25_calc_iq__two_n_phit;
        *var_fn25_calc_iq__two_n_phit_dn16_slot = var_fn25_calc_iq__two_n_phit_dn16;
        *var_fn25_calc_iq__two_n_phit_dn17_slot = var_fn25_calc_iq__two_n_phit_dn17;
        *var_fn25_calc_iq__two_n_phit_dn4_slot = var_fn25_calc_iq__two_n_phit_dn4;
        *var_fn25_calc_iq__two_n_phit_rv_slot = var_fn25_calc_iq__two_n_phit_rv;
        *var_fn25_calc_iq__vdsat_slot = var_fn25_calc_iq__vdsat;
        *var_fn25_calc_iq__vdsat1_slot = var_fn25_calc_iq__vdsat1;
        *var_fn25_calc_iq__vdsat1_dn16_slot = var_fn25_calc_iq__vdsat1_dn16;
        *var_fn25_calc_iq__vdsat1_dn17_slot = var_fn25_calc_iq__vdsat1_dn17;
        *var_fn25_calc_iq__vdsat1_dn2_slot = var_fn25_calc_iq__vdsat1_dn2;
        *var_fn25_calc_iq__vdsat1_dn3_slot = var_fn25_calc_iq__vdsat1_dn3;
        *var_fn25_calc_iq__vdsat1_dn4_slot = var_fn25_calc_iq__vdsat1_dn4;
        *var_fn25_calc_iq__vdsat1_dn7_slot = var_fn25_calc_iq__vdsat1_dn7;
        *var_fn25_calc_iq__vdsat1_rv_slot = var_fn25_calc_iq__vdsat1_rv;
        *var_fn25_calc_iq__vdsat_dn16_slot = var_fn25_calc_iq__vdsat_dn16;
        *var_fn25_calc_iq__vdsat_dn17_slot = var_fn25_calc_iq__vdsat_dn17;
        *var_fn25_calc_iq__vdsat_dn2_slot = var_fn25_calc_iq__vdsat_dn2;
        *var_fn25_calc_iq__vdsat_dn3_slot = var_fn25_calc_iq__vdsat_dn3;
        *var_fn25_calc_iq__vdsat_dn4_slot = var_fn25_calc_iq__vdsat_dn4;
        *var_fn25_calc_iq__vdsat_dn7_slot = var_fn25_calc_iq__vdsat_dn7;
        *var_fn25_calc_iq__vdsat_rv_slot = var_fn25_calc_iq__vdsat_rv;
        *var_fn25_calc_iq__vdsats_slot = var_fn25_calc_iq__vdsats;
        *var_fn25_calc_iq__vdsats1_slot = var_fn25_calc_iq__vdsats1;
        *var_fn25_calc_iq__vdsats1_dn16_slot = var_fn25_calc_iq__vdsats1_dn16;
        *var_fn25_calc_iq__vdsats1_dn17_slot = var_fn25_calc_iq__vdsats1_dn17;
        *var_fn25_calc_iq__vdsats1_dn2_slot = var_fn25_calc_iq__vdsats1_dn2;
        *var_fn25_calc_iq__vdsats1_dn3_slot = var_fn25_calc_iq__vdsats1_dn3;
        *var_fn25_calc_iq__vdsats1_dn4_slot = var_fn25_calc_iq__vdsats1_dn4;
        *var_fn25_calc_iq__vdsats1_dn7_slot = var_fn25_calc_iq__vdsats1_dn7;
        *var_fn25_calc_iq__vdsats1_rv_slot = var_fn25_calc_iq__vdsats1_rv;
        *var_fn25_calc_iq__vdsats_dn16_slot = var_fn25_calc_iq__vdsats_dn16;
        *var_fn25_calc_iq__vdsats_dn17_slot = var_fn25_calc_iq__vdsats_dn17;
        *var_fn25_calc_iq__vdsats_dn2_slot = var_fn25_calc_iq__vdsats_dn2;
        *var_fn25_calc_iq__vdsats_dn3_slot = var_fn25_calc_iq__vdsats_dn3;
        *var_fn25_calc_iq__vdsats_dn4_slot = var_fn25_calc_iq__vdsats_dn4;
        *var_fn25_calc_iq__vdsats_dn7_slot = var_fn25_calc_iq__vdsats_dn7;
        *var_fn25_calc_iq__vdsats_rv_slot = var_fn25_calc_iq__vdsats_rv;
        *var_fn25_calc_iq__vdx_slot = var_fn25_calc_iq__vdx;
        *var_fn25_calc_iq__vdx_dn16_slot = var_fn25_calc_iq__vdx_dn16;
        *var_fn25_calc_iq__vdx_dn17_slot = var_fn25_calc_iq__vdx_dn17;
        *var_fn25_calc_iq__vdx_dn2_slot = var_fn25_calc_iq__vdx_dn2;
        *var_fn25_calc_iq__vdx_dn3_slot = var_fn25_calc_iq__vdx_dn3;
        *var_fn25_calc_iq__vdx_dn4_slot = var_fn25_calc_iq__vdx_dn4;
        *var_fn25_calc_iq__vdx_dn7_slot = var_fn25_calc_iq__vdx_dn7;
        *var_fn25_calc_iq__vdx_rv_slot = var_fn25_calc_iq__vdx_rv;
        *var_fn25_calc_iq__vtdibl_slot = var_fn25_calc_iq__vtdibl;
        *var_fn25_calc_iq__vtdibl_dn16_slot = var_fn25_calc_iq__vtdibl_dn16;
        *var_fn25_calc_iq__vtdibl_dn17_slot = var_fn25_calc_iq__vtdibl_dn17;
        *var_fn25_calc_iq__vtdibl_dn4_slot = var_fn25_calc_iq__vtdibl_dn4;
        *var_fn25_calc_iq__vtdibl_rv_slot = var_fn25_calc_iq__vtdibl_rv;
        *var_fn25_calc_iq__vx_slot = var_fn25_calc_iq__vx;
        *var_fn25_calc_iq__vx_dn16_slot = var_fn25_calc_iq__vx_dn16;
        *var_fn25_calc_iq__vx_dn17_slot = var_fn25_calc_iq__vx_dn17;
        *var_fn25_calc_iq__vx_dn2_slot = var_fn25_calc_iq__vx_dn2;
        *var_fn25_calc_iq__vx_dn3_slot = var_fn25_calc_iq__vx_dn3;
        *var_fn25_calc_iq__vx_dn4_slot = var_fn25_calc_iq__vx_dn4;
        *var_fn25_calc_iq__vx_dn7_slot = var_fn25_calc_iq__vx_dn7;
        *var_fn25_calc_iq__vx_rv_slot = var_fn25_calc_iq__vx_rv;
        *var_guard27_slot = var_guard27;
        *var_guard27_rv_slot = var_guard27_rv;
        *var_guard28_slot = var_guard28;
        *var_guard28_rv_slot = var_guard28_rv;
        *var_guard29_slot = var_guard29;
        *var_guard29_rv_slot = var_guard29_rv;
        *var_guard30_slot = var_guard30;
        *var_guard30_rv_slot = var_guard30_rv;
    }

    pub(super) fn stamp_reactive_block_7(
        p: &Parameters,
        var_fn25_calc_iq__alpha_phit: f64,
        var_fn25_calc_iq__alpha_phit_dn4: f64,
        var_fn25_calc_iq__beta: f64,
        var_fn25_calc_iq__cgin: f64,
        var_fn25_calc_iq__cgin_dn4: f64,
        var_fn25_calc_iq__phitin: f64,
        var_fn25_calc_iq__phitin_dn4: f64,
        var_fn25_calc_iq__qref: f64,
        var_fn25_calc_iq__qref_dn16: f64,
        var_fn25_calc_iq__qref_dn17: f64,
        var_fn25_calc_iq__qref_dn4: f64,
        var_fn25_calc_iq__ss: f64,
        var_fn25_calc_iq__two_n_phit: f64,
        var_fn25_calc_iq__two_n_phit_dn16: f64,
        var_fn25_calc_iq__two_n_phit_dn17: f64,
        var_fn25_calc_iq__two_n_phit_dn4: f64,
        var_fn25_calc_iq__vdsat: f64,
        var_fn25_calc_iq__vdsat1: f64,
        var_fn25_calc_iq__vdsat1_dn16: f64,
        var_fn25_calc_iq__vdsat1_dn17: f64,
        var_fn25_calc_iq__vdsat1_dn2: f64,
        var_fn25_calc_iq__vdsat1_dn3: f64,
        var_fn25_calc_iq__vdsat1_dn4: f64,
        var_fn25_calc_iq__vdsat1_dn7: f64,
        var_fn25_calc_iq__vdsat_dn16: f64,
        var_fn25_calc_iq__vdsat_dn17: f64,
        var_fn25_calc_iq__vdsat_dn2: f64,
        var_fn25_calc_iq__vdsat_dn3: f64,
        var_fn25_calc_iq__vdsat_dn4: f64,
        var_fn25_calc_iq__vdsat_dn7: f64,
        var_fn25_calc_iq__vdsin: f64,
        var_fn25_calc_iq__vdsin_dn16: f64,
        var_fn25_calc_iq__vdsin_dn17: f64,
        var_fn25_calc_iq__vdx: f64,
        var_fn25_calc_iq__vdx_dn16: f64,
        var_fn25_calc_iq__vdx_dn17: f64,
        var_fn25_calc_iq__vdx_dn2: f64,
        var_fn25_calc_iq__vdx_dn3: f64,
        var_fn25_calc_iq__vdx_dn4: f64,
        var_fn25_calc_iq__vdx_dn7: f64,
        var_fn25_calc_iq__vgdin: f64,
        var_fn25_calc_iq__vgdin_dn16: f64,
        var_fn25_calc_iq__vgdin_dn17: f64,
        var_fn25_calc_iq__vgdin_dn2: f64,
        var_fn25_calc_iq__vgdin_dn7: f64,
        var_fn25_calc_iq__vgsin: f64,
        var_fn25_calc_iq__vgsin_dn16: f64,
        var_fn25_calc_iq__vgsin_dn2: f64,
        var_fn25_calc_iq__vgsin_dn7: f64,
        var_fn25_calc_iq__vtdibl: f64,
        var_fn25_calc_iq__vtdibl_dn16: f64,
        var_fn25_calc_iq__vtdibl_dn17: f64,
        var_fn25_calc_iq__vtdibl_dn4: f64,
        var_fn25_calc_iq__vtof: f64,
        var_fn25_calc_iq__vtof_dn4: f64,
        var_guard24: f64,
        var_fn25_calc_iq__etad_slot: &mut f64,
        var_fn25_calc_iq__etad_dn16_slot: &mut f64,
        var_fn25_calc_iq__etad_dn17_slot: &mut f64,
        var_fn25_calc_iq__etad_dn2_slot: &mut f64,
        var_fn25_calc_iq__etad_dn3_slot: &mut f64,
        var_fn25_calc_iq__etad_dn4_slot: &mut f64,
        var_fn25_calc_iq__etad_dn7_slot: &mut f64,
        var_fn25_calc_iq__etad_rv_slot: &mut f64,
        var_fn25_calc_iq__etas_slot: &mut f64,
        var_fn25_calc_iq__etas_dn16_slot: &mut f64,
        var_fn25_calc_iq__etas_dn17_slot: &mut f64,
        var_fn25_calc_iq__etas_dn2_slot: &mut f64,
        var_fn25_calc_iq__etas_dn3_slot: &mut f64,
        var_fn25_calc_iq__etas_dn4_slot: &mut f64,
        var_fn25_calc_iq__etas_dn7_slot: &mut f64,
        var_fn25_calc_iq__etas_rv_slot: &mut f64,
        var_fn25_calc_iq__exparg_slot: &mut f64,
        var_fn25_calc_iq__exparg_dn16_slot: &mut f64,
        var_fn25_calc_iq__exparg_dn17_slot: &mut f64,
        var_fn25_calc_iq__exparg_dn2_slot: &mut f64,
        var_fn25_calc_iq__exparg_dn3_slot: &mut f64,
        var_fn25_calc_iq__exparg_dn4_slot: &mut f64,
        var_fn25_calc_iq__exparg_dn7_slot: &mut f64,
        var_fn25_calc_iq__exparg_rv_slot: &mut f64,
        var_fn25_calc_iq__fds_slot: &mut f64,
        var_fn25_calc_iq__fds_dn16_slot: &mut f64,
        var_fn25_calc_iq__fds_dn17_slot: &mut f64,
        var_fn25_calc_iq__fds_dn2_slot: &mut f64,
        var_fn25_calc_iq__fds_dn3_slot: &mut f64,
        var_fn25_calc_iq__fds_dn4_slot: &mut f64,
        var_fn25_calc_iq__fds_dn7_slot: &mut f64,
        var_fn25_calc_iq__fds_rv_slot: &mut f64,
        var_fn25_calc_iq__ffd_slot: &mut f64,
        var_fn25_calc_iq__ffd_dn16_slot: &mut f64,
        var_fn25_calc_iq__ffd_dn17_slot: &mut f64,
        var_fn25_calc_iq__ffd_dn2_slot: &mut f64,
        var_fn25_calc_iq__ffd_dn3_slot: &mut f64,
        var_fn25_calc_iq__ffd_dn4_slot: &mut f64,
        var_fn25_calc_iq__ffd_dn7_slot: &mut f64,
        var_fn25_calc_iq__ffd_rv_slot: &mut f64,
        var_fn25_calc_iq__ffs_slot: &mut f64,
        var_fn25_calc_iq__ffs_dn16_slot: &mut f64,
        var_fn25_calc_iq__ffs_dn17_slot: &mut f64,
        var_fn25_calc_iq__ffs_dn2_slot: &mut f64,
        var_fn25_calc_iq__ffs_dn3_slot: &mut f64,
        var_fn25_calc_iq__ffs_dn4_slot: &mut f64,
        var_fn25_calc_iq__ffs_dn7_slot: &mut f64,
        var_fn25_calc_iq__ffs_rv_slot: &mut f64,
        var_fn25_calc_iq__myarg_slot: &mut f64,
        var_fn25_calc_iq__myarg0_slot: &mut f64,
        var_fn25_calc_iq__myarg0_dn4_slot: &mut f64,
        var_fn25_calc_iq__myarg0_rv_slot: &mut f64,
        var_fn25_calc_iq__myarg_dn16_slot: &mut f64,
        var_fn25_calc_iq__myarg_dn17_slot: &mut f64,
        var_fn25_calc_iq__myarg_dn2_slot: &mut f64,
        var_fn25_calc_iq__myarg_dn3_slot: &mut f64,
        var_fn25_calc_iq__myarg_dn4_slot: &mut f64,
        var_fn25_calc_iq__myarg_dn7_slot: &mut f64,
        var_fn25_calc_iq__myarg_rv_slot: &mut f64,
        var_fn25_calc_iq__n0_slot: &mut f64,
        var_fn25_calc_iq__n0_dn4_slot: &mut f64,
        var_fn25_calc_iq__n0_rv_slot: &mut f64,
        var_fn25_calc_iq__qinvd_slot: &mut f64,
        var_fn25_calc_iq__qinvd_dn16_slot: &mut f64,
        var_fn25_calc_iq__qinvd_dn17_slot: &mut f64,
        var_fn25_calc_iq__qinvd_dn2_slot: &mut f64,
        var_fn25_calc_iq__qinvd_dn3_slot: &mut f64,
        var_fn25_calc_iq__qinvd_dn4_slot: &mut f64,
        var_fn25_calc_iq__qinvd_dn7_slot: &mut f64,
        var_fn25_calc_iq__qinvd_rv_slot: &mut f64,
        var_fn25_calc_iq__qinvs_slot: &mut f64,
        var_fn25_calc_iq__qinvs_dn16_slot: &mut f64,
        var_fn25_calc_iq__qinvs_dn17_slot: &mut f64,
        var_fn25_calc_iq__qinvs_dn2_slot: &mut f64,
        var_fn25_calc_iq__qinvs_dn3_slot: &mut f64,
        var_fn25_calc_iq__qinvs_dn4_slot: &mut f64,
        var_fn25_calc_iq__qinvs_dn7_slot: &mut f64,
        var_fn25_calc_iq__qinvs_rv_slot: &mut f64,
        var_fn25_calc_iq__qref0_slot: &mut f64,
        var_fn25_calc_iq__qref0_dn4_slot: &mut f64,
        var_fn25_calc_iq__qref0_rv_slot: &mut f64,
        var_fn25_calc_iq__two_n_phit0_slot: &mut f64,
        var_fn25_calc_iq__two_n_phit0_dn4_slot: &mut f64,
        var_fn25_calc_iq__two_n_phit0_rv_slot: &mut f64,
        var_fn25_calc_iq__vdsc_slot: &mut f64,
        var_fn25_calc_iq__vdsc_dn16_slot: &mut f64,
        var_fn25_calc_iq__vdsc_dn17_slot: &mut f64,
        var_fn25_calc_iq__vdsc_dn2_slot: &mut f64,
        var_fn25_calc_iq__vdsc_dn3_slot: &mut f64,
        var_fn25_calc_iq__vdsc_dn4_slot: &mut f64,
        var_fn25_calc_iq__vdsc_dn7_slot: &mut f64,
        var_fn25_calc_iq__vdsc_rv_slot: &mut f64,
        var_fn25_calc_iq__vsx_slot: &mut f64,
        var_fn25_calc_iq__vsx_dn16_slot: &mut f64,
        var_fn25_calc_iq__vsx_dn17_slot: &mut f64,
        var_fn25_calc_iq__vsx_dn2_slot: &mut f64,
        var_fn25_calc_iq__vsx_dn3_slot: &mut f64,
        var_fn25_calc_iq__vsx_dn4_slot: &mut f64,
        var_fn25_calc_iq__vsx_dn7_slot: &mut f64,
        var_fn25_calc_iq__vsx_rv_slot: &mut f64,
        var_guard31_slot: &mut f64,
        var_guard31_rv_slot: &mut f64,
        var_guard32_slot: &mut f64,
        var_guard32_rv_slot: &mut f64,
        var_guard33_slot: &mut f64,
        var_guard33_rv_slot: &mut f64,
        var_guard34_slot: &mut f64,
        var_guard34_rv_slot: &mut f64,
        var_guard35_slot: &mut f64,
        var_guard35_rv_slot: &mut f64,
        var_guard36_slot: &mut f64,
        var_guard36_rv_slot: &mut f64,
        var_guard37_slot: &mut f64,
        var_guard37_rv_slot: &mut f64,
        var_guard38_slot: &mut f64,
        var_guard38_rv_slot: &mut f64,
    ) {
        let mut var_fn25_calc_iq__etad: f64 = *var_fn25_calc_iq__etad_slot;
        let mut var_fn25_calc_iq__etad_dn16: f64 = *var_fn25_calc_iq__etad_dn16_slot;
        let mut var_fn25_calc_iq__etad_dn17: f64 = *var_fn25_calc_iq__etad_dn17_slot;
        let mut var_fn25_calc_iq__etad_dn2: f64 = *var_fn25_calc_iq__etad_dn2_slot;
        let mut var_fn25_calc_iq__etad_dn3: f64 = *var_fn25_calc_iq__etad_dn3_slot;
        let mut var_fn25_calc_iq__etad_dn4: f64 = *var_fn25_calc_iq__etad_dn4_slot;
        let mut var_fn25_calc_iq__etad_dn7: f64 = *var_fn25_calc_iq__etad_dn7_slot;
        let mut var_fn25_calc_iq__etad_rv: f64 = *var_fn25_calc_iq__etad_rv_slot;
        let mut var_fn25_calc_iq__etas: f64 = *var_fn25_calc_iq__etas_slot;
        let mut var_fn25_calc_iq__etas_dn16: f64 = *var_fn25_calc_iq__etas_dn16_slot;
        let mut var_fn25_calc_iq__etas_dn17: f64 = *var_fn25_calc_iq__etas_dn17_slot;
        let mut var_fn25_calc_iq__etas_dn2: f64 = *var_fn25_calc_iq__etas_dn2_slot;
        let mut var_fn25_calc_iq__etas_dn3: f64 = *var_fn25_calc_iq__etas_dn3_slot;
        let mut var_fn25_calc_iq__etas_dn4: f64 = *var_fn25_calc_iq__etas_dn4_slot;
        let mut var_fn25_calc_iq__etas_dn7: f64 = *var_fn25_calc_iq__etas_dn7_slot;
        let mut var_fn25_calc_iq__etas_rv: f64 = *var_fn25_calc_iq__etas_rv_slot;
        let mut var_fn25_calc_iq__exparg: f64 = *var_fn25_calc_iq__exparg_slot;
        let mut var_fn25_calc_iq__exparg_dn16: f64 = *var_fn25_calc_iq__exparg_dn16_slot;
        let mut var_fn25_calc_iq__exparg_dn17: f64 = *var_fn25_calc_iq__exparg_dn17_slot;
        let mut var_fn25_calc_iq__exparg_dn2: f64 = *var_fn25_calc_iq__exparg_dn2_slot;
        let mut var_fn25_calc_iq__exparg_dn3: f64 = *var_fn25_calc_iq__exparg_dn3_slot;
        let mut var_fn25_calc_iq__exparg_dn4: f64 = *var_fn25_calc_iq__exparg_dn4_slot;
        let mut var_fn25_calc_iq__exparg_dn7: f64 = *var_fn25_calc_iq__exparg_dn7_slot;
        let mut var_fn25_calc_iq__exparg_rv: f64 = *var_fn25_calc_iq__exparg_rv_slot;
        let mut var_fn25_calc_iq__fds: f64 = *var_fn25_calc_iq__fds_slot;
        let mut var_fn25_calc_iq__fds_dn16: f64 = *var_fn25_calc_iq__fds_dn16_slot;
        let mut var_fn25_calc_iq__fds_dn17: f64 = *var_fn25_calc_iq__fds_dn17_slot;
        let mut var_fn25_calc_iq__fds_dn2: f64 = *var_fn25_calc_iq__fds_dn2_slot;
        let mut var_fn25_calc_iq__fds_dn3: f64 = *var_fn25_calc_iq__fds_dn3_slot;
        let mut var_fn25_calc_iq__fds_dn4: f64 = *var_fn25_calc_iq__fds_dn4_slot;
        let mut var_fn25_calc_iq__fds_dn7: f64 = *var_fn25_calc_iq__fds_dn7_slot;
        let mut var_fn25_calc_iq__fds_rv: f64 = *var_fn25_calc_iq__fds_rv_slot;
        let mut var_fn25_calc_iq__ffd: f64 = *var_fn25_calc_iq__ffd_slot;
        let mut var_fn25_calc_iq__ffd_dn16: f64 = *var_fn25_calc_iq__ffd_dn16_slot;
        let mut var_fn25_calc_iq__ffd_dn17: f64 = *var_fn25_calc_iq__ffd_dn17_slot;
        let mut var_fn25_calc_iq__ffd_dn2: f64 = *var_fn25_calc_iq__ffd_dn2_slot;
        let mut var_fn25_calc_iq__ffd_dn3: f64 = *var_fn25_calc_iq__ffd_dn3_slot;
        let mut var_fn25_calc_iq__ffd_dn4: f64 = *var_fn25_calc_iq__ffd_dn4_slot;
        let mut var_fn25_calc_iq__ffd_dn7: f64 = *var_fn25_calc_iq__ffd_dn7_slot;
        let mut var_fn25_calc_iq__ffd_rv: f64 = *var_fn25_calc_iq__ffd_rv_slot;
        let mut var_fn25_calc_iq__ffs: f64 = *var_fn25_calc_iq__ffs_slot;
        let mut var_fn25_calc_iq__ffs_dn16: f64 = *var_fn25_calc_iq__ffs_dn16_slot;
        let mut var_fn25_calc_iq__ffs_dn17: f64 = *var_fn25_calc_iq__ffs_dn17_slot;
        let mut var_fn25_calc_iq__ffs_dn2: f64 = *var_fn25_calc_iq__ffs_dn2_slot;
        let mut var_fn25_calc_iq__ffs_dn3: f64 = *var_fn25_calc_iq__ffs_dn3_slot;
        let mut var_fn25_calc_iq__ffs_dn4: f64 = *var_fn25_calc_iq__ffs_dn4_slot;
        let mut var_fn25_calc_iq__ffs_dn7: f64 = *var_fn25_calc_iq__ffs_dn7_slot;
        let mut var_fn25_calc_iq__ffs_rv: f64 = *var_fn25_calc_iq__ffs_rv_slot;
        let mut var_fn25_calc_iq__myarg: f64 = *var_fn25_calc_iq__myarg_slot;
        let mut var_fn25_calc_iq__myarg0: f64 = *var_fn25_calc_iq__myarg0_slot;
        let mut var_fn25_calc_iq__myarg0_dn4: f64 = *var_fn25_calc_iq__myarg0_dn4_slot;
        let mut var_fn25_calc_iq__myarg0_rv: f64 = *var_fn25_calc_iq__myarg0_rv_slot;
        let mut var_fn25_calc_iq__myarg_dn16: f64 = *var_fn25_calc_iq__myarg_dn16_slot;
        let mut var_fn25_calc_iq__myarg_dn17: f64 = *var_fn25_calc_iq__myarg_dn17_slot;
        let mut var_fn25_calc_iq__myarg_dn2: f64 = *var_fn25_calc_iq__myarg_dn2_slot;
        let mut var_fn25_calc_iq__myarg_dn3: f64 = *var_fn25_calc_iq__myarg_dn3_slot;
        let mut var_fn25_calc_iq__myarg_dn4: f64 = *var_fn25_calc_iq__myarg_dn4_slot;
        let mut var_fn25_calc_iq__myarg_dn7: f64 = *var_fn25_calc_iq__myarg_dn7_slot;
        let mut var_fn25_calc_iq__myarg_rv: f64 = *var_fn25_calc_iq__myarg_rv_slot;
        let mut var_fn25_calc_iq__n0: f64 = *var_fn25_calc_iq__n0_slot;
        let mut var_fn25_calc_iq__n0_dn4: f64 = *var_fn25_calc_iq__n0_dn4_slot;
        let mut var_fn25_calc_iq__n0_rv: f64 = *var_fn25_calc_iq__n0_rv_slot;
        let mut var_fn25_calc_iq__qinvd: f64 = *var_fn25_calc_iq__qinvd_slot;
        let mut var_fn25_calc_iq__qinvd_dn16: f64 = *var_fn25_calc_iq__qinvd_dn16_slot;
        let mut var_fn25_calc_iq__qinvd_dn17: f64 = *var_fn25_calc_iq__qinvd_dn17_slot;
        let mut var_fn25_calc_iq__qinvd_dn2: f64 = *var_fn25_calc_iq__qinvd_dn2_slot;
        let mut var_fn25_calc_iq__qinvd_dn3: f64 = *var_fn25_calc_iq__qinvd_dn3_slot;
        let mut var_fn25_calc_iq__qinvd_dn4: f64 = *var_fn25_calc_iq__qinvd_dn4_slot;
        let mut var_fn25_calc_iq__qinvd_dn7: f64 = *var_fn25_calc_iq__qinvd_dn7_slot;
        let mut var_fn25_calc_iq__qinvd_rv: f64 = *var_fn25_calc_iq__qinvd_rv_slot;
        let mut var_fn25_calc_iq__qinvs: f64 = *var_fn25_calc_iq__qinvs_slot;
        let mut var_fn25_calc_iq__qinvs_dn16: f64 = *var_fn25_calc_iq__qinvs_dn16_slot;
        let mut var_fn25_calc_iq__qinvs_dn17: f64 = *var_fn25_calc_iq__qinvs_dn17_slot;
        let mut var_fn25_calc_iq__qinvs_dn2: f64 = *var_fn25_calc_iq__qinvs_dn2_slot;
        let mut var_fn25_calc_iq__qinvs_dn3: f64 = *var_fn25_calc_iq__qinvs_dn3_slot;
        let mut var_fn25_calc_iq__qinvs_dn4: f64 = *var_fn25_calc_iq__qinvs_dn4_slot;
        let mut var_fn25_calc_iq__qinvs_dn7: f64 = *var_fn25_calc_iq__qinvs_dn7_slot;
        let mut var_fn25_calc_iq__qinvs_rv: f64 = *var_fn25_calc_iq__qinvs_rv_slot;
        let mut var_fn25_calc_iq__qref0: f64 = *var_fn25_calc_iq__qref0_slot;
        let mut var_fn25_calc_iq__qref0_dn4: f64 = *var_fn25_calc_iq__qref0_dn4_slot;
        let mut var_fn25_calc_iq__qref0_rv: f64 = *var_fn25_calc_iq__qref0_rv_slot;
        let mut var_fn25_calc_iq__two_n_phit0: f64 = *var_fn25_calc_iq__two_n_phit0_slot;
        let mut var_fn25_calc_iq__two_n_phit0_dn4: f64 = *var_fn25_calc_iq__two_n_phit0_dn4_slot;
        let mut var_fn25_calc_iq__two_n_phit0_rv: f64 = *var_fn25_calc_iq__two_n_phit0_rv_slot;
        let mut var_fn25_calc_iq__vdsc: f64 = *var_fn25_calc_iq__vdsc_slot;
        let mut var_fn25_calc_iq__vdsc_dn16: f64 = *var_fn25_calc_iq__vdsc_dn16_slot;
        let mut var_fn25_calc_iq__vdsc_dn17: f64 = *var_fn25_calc_iq__vdsc_dn17_slot;
        let mut var_fn25_calc_iq__vdsc_dn2: f64 = *var_fn25_calc_iq__vdsc_dn2_slot;
        let mut var_fn25_calc_iq__vdsc_dn3: f64 = *var_fn25_calc_iq__vdsc_dn3_slot;
        let mut var_fn25_calc_iq__vdsc_dn4: f64 = *var_fn25_calc_iq__vdsc_dn4_slot;
        let mut var_fn25_calc_iq__vdsc_dn7: f64 = *var_fn25_calc_iq__vdsc_dn7_slot;
        let mut var_fn25_calc_iq__vdsc_rv: f64 = *var_fn25_calc_iq__vdsc_rv_slot;
        let mut var_fn25_calc_iq__vsx: f64 = *var_fn25_calc_iq__vsx_slot;
        let mut var_fn25_calc_iq__vsx_dn16: f64 = *var_fn25_calc_iq__vsx_dn16_slot;
        let mut var_fn25_calc_iq__vsx_dn17: f64 = *var_fn25_calc_iq__vsx_dn17_slot;
        let mut var_fn25_calc_iq__vsx_dn2: f64 = *var_fn25_calc_iq__vsx_dn2_slot;
        let mut var_fn25_calc_iq__vsx_dn3: f64 = *var_fn25_calc_iq__vsx_dn3_slot;
        let mut var_fn25_calc_iq__vsx_dn4: f64 = *var_fn25_calc_iq__vsx_dn4_slot;
        let mut var_fn25_calc_iq__vsx_dn7: f64 = *var_fn25_calc_iq__vsx_dn7_slot;
        let mut var_fn25_calc_iq__vsx_rv: f64 = *var_fn25_calc_iq__vsx_rv_slot;
        let mut var_guard31: f64 = *var_guard31_slot;
        let mut var_guard31_rv: f64 = *var_guard31_rv_slot;
        let mut var_guard32: f64 = *var_guard32_slot;
        let mut var_guard32_rv: f64 = *var_guard32_rv_slot;
        let mut var_guard33: f64 = *var_guard33_slot;
        let mut var_guard33_rv: f64 = *var_guard33_rv_slot;
        let mut var_guard34: f64 = *var_guard34_slot;
        let mut var_guard34_rv: f64 = *var_guard34_rv_slot;
        let mut var_guard35: f64 = *var_guard35_slot;
        let mut var_guard35_rv: f64 = *var_guard35_rv_slot;
        let mut var_guard36: f64 = *var_guard36_slot;
        let mut var_guard36_rv: f64 = *var_guard36_rv_slot;
        let mut var_guard37: f64 = *var_guard37_slot;
        let mut var_guard37_rv: f64 = *var_guard37_rv_slot;
        let mut var_guard38: f64 = *var_guard38_slot;
        let mut var_guard38_rv: f64 = *var_guard38_rv_slot;

        let (assign3230_e4923, assign3230_e4923_d_n2, assign3230_e4923_d_n3, assign3230_e4923_d_n4, assign3230_e4923_d_n7, assign3230_e4923_d_n16, assign3230_e4923_d_n17,) = {
    if (var_guard24 != 0.0) {
        let (assign3230_e4913, assign3230_e4913_d_n2, assign3230_e4913_d_n3, assign3230_e4913_d_n4, assign3230_e4913_d_n7, assign3230_e4913_d_n16, assign3230_e4913_d_n17,) = {
            if (p.p52 != 0.0) {
                let assign3230_e4859: f64 = (-var_fn25_calc_iq__vdsin);
                let assign3230_e4861: f64 = (assign3230_e4859 / var_fn25_calc_iq__vdsat1);
                let assign3230_e4862: f64 = assign3230_e4861;
                let assign3230_e4865: f64 = (-var_fn25_calc_iq__vdsin);
                let assign3230_e4867: f64 = (assign3230_e4865 / var_fn25_calc_iq__vdsat1);
                let assign3230_e4868: f64 = (-assign3230_e4867);
                let assign3230_e4871: f64 = (0.001 / p.p53);
                let assign3230_e4874: f64 = (-var_fn25_calc_iq__vdsin);
                let assign3230_e4876: f64 = (assign3230_e4874 / var_fn25_calc_iq__vdsat1);
                let assign3230_e4877: f64 = (-assign3230_e4876);
                let assign3230_e4878: f64 = (assign3230_e4871 * assign3230_e4877);
                let assign3230_e4879: f64 = (assign3230_e4878).tanh();
                let assign3230_e4880: f64 = (assign3230_e4868 * assign3230_e4879);
                let assign3230_e4881: f64 = (assign3230_e4862 + assign3230_e4880);
                let assign3230_e4882: f64 = (0.5 * assign3230_e4881);
                (assign3230_e4882, (0.5 * ((-((assign3230_e4859 * var_fn25_calc_iq__vdsat1_dn2) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1))) + (((-(-((assign3230_e4865 * var_fn25_calc_iq__vdsat1_dn2) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1)))) * assign3230_e4879) + (assign3230_e4868 * ((assign3230_e4871 * (-(-((assign3230_e4874 * var_fn25_calc_iq__vdsat1_dn2) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1))))) / ((assign3230_e4878).cosh() * (assign3230_e4878).cosh())))))), (0.5 * ((-((assign3230_e4859 * var_fn25_calc_iq__vdsat1_dn3) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1))) + (((-(-((assign3230_e4865 * var_fn25_calc_iq__vdsat1_dn3) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1)))) * assign3230_e4879) + (assign3230_e4868 * ((assign3230_e4871 * (-(-((assign3230_e4874 * var_fn25_calc_iq__vdsat1_dn3) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1))))) / ((assign3230_e4878).cosh() * (assign3230_e4878).cosh())))))), (0.5 * ((-((assign3230_e4859 * var_fn25_calc_iq__vdsat1_dn4) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1))) + (((-(-((assign3230_e4865 * var_fn25_calc_iq__vdsat1_dn4) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1)))) * assign3230_e4879) + (assign3230_e4868 * ((assign3230_e4871 * (-(-((assign3230_e4874 * var_fn25_calc_iq__vdsat1_dn4) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1))))) / ((assign3230_e4878).cosh() * (assign3230_e4878).cosh())))))), (0.5 * ((-((assign3230_e4859 * var_fn25_calc_iq__vdsat1_dn7) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1))) + (((-(-((assign3230_e4865 * var_fn25_calc_iq__vdsat1_dn7) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1)))) * assign3230_e4879) + (assign3230_e4868 * ((assign3230_e4871 * (-(-((assign3230_e4874 * var_fn25_calc_iq__vdsat1_dn7) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1))))) / ((assign3230_e4878).cosh() * (assign3230_e4878).cosh())))))), (0.5 * (((((-var_fn25_calc_iq__vdsin_dn16) * var_fn25_calc_iq__vdsat1) - (assign3230_e4859 * var_fn25_calc_iq__vdsat1_dn16)) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1)) + (((-((((-var_fn25_calc_iq__vdsin_dn16) * var_fn25_calc_iq__vdsat1) - (assign3230_e4865 * var_fn25_calc_iq__vdsat1_dn16)) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1))) * assign3230_e4879) + (assign3230_e4868 * ((assign3230_e4871 * (-((((-var_fn25_calc_iq__vdsin_dn16) * var_fn25_calc_iq__vdsat1) - (assign3230_e4874 * var_fn25_calc_iq__vdsat1_dn16)) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1)))) / ((assign3230_e4878).cosh() * (assign3230_e4878).cosh())))))), (0.5 * (((((-var_fn25_calc_iq__vdsin_dn17) * var_fn25_calc_iq__vdsat1) - (assign3230_e4859 * var_fn25_calc_iq__vdsat1_dn17)) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1)) + (((-((((-var_fn25_calc_iq__vdsin_dn17) * var_fn25_calc_iq__vdsat1) - (assign3230_e4865 * var_fn25_calc_iq__vdsat1_dn17)) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1))) * assign3230_e4879) + (assign3230_e4868 * ((assign3230_e4871 * (-((((-var_fn25_calc_iq__vdsin_dn17) * var_fn25_calc_iq__vdsat1) - (assign3230_e4874 * var_fn25_calc_iq__vdsat1_dn17)) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1)))) / ((assign3230_e4878).cosh() * (assign3230_e4878).cosh())))))),)
            } else {
                let (assign3230_e4912, assign3230_e4912_d_n2, assign3230_e4912_d_n3, assign3230_e4912_d_n4, assign3230_e4912_d_n7, assign3230_e4912_d_n16, assign3230_e4912_d_n17,) = {
                    if (p.p52 == 0.0) {
                        let assign3230_e4889: f64 = (-var_fn25_calc_iq__vdsin);
                        let assign3230_e4891: f64 = (assign3230_e4889 / var_fn25_calc_iq__vdsat1);
                        let assign3230_e4892: f64 = assign3230_e4891;
                        let assign3230_e4895: f64 = (-var_fn25_calc_iq__vdsin);
                        let assign3230_e4897: f64 = (assign3230_e4895 / var_fn25_calc_iq__vdsat1);
                        let assign3230_e4898: f64 = (-assign3230_e4897);
                        let assign3230_e4901: f64 = (-var_fn25_calc_iq__vdsin);
                        let assign3230_e4903: f64 = (assign3230_e4901 / var_fn25_calc_iq__vdsat1);
                        let assign3230_e4904: f64 = (-assign3230_e4903);
                        let assign3230_e4905: f64 = (assign3230_e4898 * assign3230_e4904);
                        let assign3230_e4907: f64 = (assign3230_e4905 + p.p53);
                        let assign3230_e4908: f64 = (assign3230_e4907).sqrt();
                        let assign3230_e4909: f64 = (assign3230_e4892 + assign3230_e4908);
                        let assign3230_e4910: f64 = (0.5 * assign3230_e4909);
                        (assign3230_e4910, (0.5 * ((-((assign3230_e4889 * var_fn25_calc_iq__vdsat1_dn2) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1))) + ((((-(-((assign3230_e4895 * var_fn25_calc_iq__vdsat1_dn2) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1)))) * assign3230_e4904) + (assign3230_e4898 * (-(-((assign3230_e4901 * var_fn25_calc_iq__vdsat1_dn2) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1)))))) / (2.0 * assign3230_e4908)))), (0.5 * ((-((assign3230_e4889 * var_fn25_calc_iq__vdsat1_dn3) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1))) + ((((-(-((assign3230_e4895 * var_fn25_calc_iq__vdsat1_dn3) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1)))) * assign3230_e4904) + (assign3230_e4898 * (-(-((assign3230_e4901 * var_fn25_calc_iq__vdsat1_dn3) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1)))))) / (2.0 * assign3230_e4908)))), (0.5 * ((-((assign3230_e4889 * var_fn25_calc_iq__vdsat1_dn4) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1))) + ((((-(-((assign3230_e4895 * var_fn25_calc_iq__vdsat1_dn4) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1)))) * assign3230_e4904) + (assign3230_e4898 * (-(-((assign3230_e4901 * var_fn25_calc_iq__vdsat1_dn4) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1)))))) / (2.0 * assign3230_e4908)))), (0.5 * ((-((assign3230_e4889 * var_fn25_calc_iq__vdsat1_dn7) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1))) + ((((-(-((assign3230_e4895 * var_fn25_calc_iq__vdsat1_dn7) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1)))) * assign3230_e4904) + (assign3230_e4898 * (-(-((assign3230_e4901 * var_fn25_calc_iq__vdsat1_dn7) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1)))))) / (2.0 * assign3230_e4908)))), (0.5 * (((((-var_fn25_calc_iq__vdsin_dn16) * var_fn25_calc_iq__vdsat1) - (assign3230_e4889 * var_fn25_calc_iq__vdsat1_dn16)) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1)) + ((((-((((-var_fn25_calc_iq__vdsin_dn16) * var_fn25_calc_iq__vdsat1) - (assign3230_e4895 * var_fn25_calc_iq__vdsat1_dn16)) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1))) * assign3230_e4904) + (assign3230_e4898 * (-((((-var_fn25_calc_iq__vdsin_dn16) * var_fn25_calc_iq__vdsat1) - (assign3230_e4901 * var_fn25_calc_iq__vdsat1_dn16)) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1))))) / (2.0 * assign3230_e4908)))), (0.5 * (((((-var_fn25_calc_iq__vdsin_dn17) * var_fn25_calc_iq__vdsat1) - (assign3230_e4889 * var_fn25_calc_iq__vdsat1_dn17)) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1)) + ((((-((((-var_fn25_calc_iq__vdsin_dn17) * var_fn25_calc_iq__vdsat1) - (assign3230_e4895 * var_fn25_calc_iq__vdsat1_dn17)) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1))) * assign3230_e4904) + (assign3230_e4898 * (-((((-var_fn25_calc_iq__vdsin_dn17) * var_fn25_calc_iq__vdsat1) - (assign3230_e4901 * var_fn25_calc_iq__vdsat1_dn17)) / (var_fn25_calc_iq__vdsat1 * var_fn25_calc_iq__vdsat1))))) / (2.0 * assign3230_e4908)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign3230_e4912, assign3230_e4912_d_n2, assign3230_e4912_d_n3, assign3230_e4912_d_n4, assign3230_e4912_d_n7, assign3230_e4912_d_n16, assign3230_e4912_d_n17,)
            }
        };
        let assign3230_e4915: f64 = (assign3230_e4913).powf(var_fn25_calc_iq__beta);
        let assign3230_e4916: f64 = (1.0 + assign3230_e4915);
        let assign3230_e4919: f64 = (1.0 / var_fn25_calc_iq__beta);
        let assign3230_e4920: f64 = (assign3230_e4916).powf(assign3230_e4919);
        let assign3230_e4921: f64 = (1.0 / assign3230_e4920);
        (assign3230_e4921, (-(if 0.0 == 0.0 && ((assign3230_e4919) as f64).is_finite() && ((assign3230_e4919) as f64).fract() == 0.0 { if assign3230_e4919 == 0.0 { 0.0 } else { (assign3230_e4919 * ((assign3230_e4916).powf(assign3230_e4919 - 1.0) * if 0.0 == 0.0 && ((var_fn25_calc_iq__beta) as f64).is_finite() && ((var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (var_fn25_calc_iq__beta * ((assign3230_e4913).powf(var_fn25_calc_iq__beta - 1.0) * assign3230_e4913_d_n2)) } } else { (assign3230_e4915 * (var_fn25_calc_iq__beta * (assign3230_e4913_d_n2 / assign3230_e4913))) })) } } else { (assign3230_e4920 * (assign3230_e4919 * (if 0.0 == 0.0 && ((var_fn25_calc_iq__beta) as f64).is_finite() && ((var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (var_fn25_calc_iq__beta * ((assign3230_e4913).powf(var_fn25_calc_iq__beta - 1.0) * assign3230_e4913_d_n2)) } } else { (assign3230_e4915 * (var_fn25_calc_iq__beta * (assign3230_e4913_d_n2 / assign3230_e4913))) } / assign3230_e4916))) } / (assign3230_e4920 * assign3230_e4920))), (-(if 0.0 == 0.0 && ((assign3230_e4919) as f64).is_finite() && ((assign3230_e4919) as f64).fract() == 0.0 { if assign3230_e4919 == 0.0 { 0.0 } else { (assign3230_e4919 * ((assign3230_e4916).powf(assign3230_e4919 - 1.0) * if 0.0 == 0.0 && ((var_fn25_calc_iq__beta) as f64).is_finite() && ((var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (var_fn25_calc_iq__beta * ((assign3230_e4913).powf(var_fn25_calc_iq__beta - 1.0) * assign3230_e4913_d_n3)) } } else { (assign3230_e4915 * (var_fn25_calc_iq__beta * (assign3230_e4913_d_n3 / assign3230_e4913))) })) } } else { (assign3230_e4920 * (assign3230_e4919 * (if 0.0 == 0.0 && ((var_fn25_calc_iq__beta) as f64).is_finite() && ((var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (var_fn25_calc_iq__beta * ((assign3230_e4913).powf(var_fn25_calc_iq__beta - 1.0) * assign3230_e4913_d_n3)) } } else { (assign3230_e4915 * (var_fn25_calc_iq__beta * (assign3230_e4913_d_n3 / assign3230_e4913))) } / assign3230_e4916))) } / (assign3230_e4920 * assign3230_e4920))), (-(if 0.0 == 0.0 && ((assign3230_e4919) as f64).is_finite() && ((assign3230_e4919) as f64).fract() == 0.0 { if assign3230_e4919 == 0.0 { 0.0 } else { (assign3230_e4919 * ((assign3230_e4916).powf(assign3230_e4919 - 1.0) * if 0.0 == 0.0 && ((var_fn25_calc_iq__beta) as f64).is_finite() && ((var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (var_fn25_calc_iq__beta * ((assign3230_e4913).powf(var_fn25_calc_iq__beta - 1.0) * assign3230_e4913_d_n4)) } } else { (assign3230_e4915 * (var_fn25_calc_iq__beta * (assign3230_e4913_d_n4 / assign3230_e4913))) })) } } else { (assign3230_e4920 * (assign3230_e4919 * (if 0.0 == 0.0 && ((var_fn25_calc_iq__beta) as f64).is_finite() && ((var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (var_fn25_calc_iq__beta * ((assign3230_e4913).powf(var_fn25_calc_iq__beta - 1.0) * assign3230_e4913_d_n4)) } } else { (assign3230_e4915 * (var_fn25_calc_iq__beta * (assign3230_e4913_d_n4 / assign3230_e4913))) } / assign3230_e4916))) } / (assign3230_e4920 * assign3230_e4920))), (-(if 0.0 == 0.0 && ((assign3230_e4919) as f64).is_finite() && ((assign3230_e4919) as f64).fract() == 0.0 { if assign3230_e4919 == 0.0 { 0.0 } else { (assign3230_e4919 * ((assign3230_e4916).powf(assign3230_e4919 - 1.0) * if 0.0 == 0.0 && ((var_fn25_calc_iq__beta) as f64).is_finite() && ((var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (var_fn25_calc_iq__beta * ((assign3230_e4913).powf(var_fn25_calc_iq__beta - 1.0) * assign3230_e4913_d_n7)) } } else { (assign3230_e4915 * (var_fn25_calc_iq__beta * (assign3230_e4913_d_n7 / assign3230_e4913))) })) } } else { (assign3230_e4920 * (assign3230_e4919 * (if 0.0 == 0.0 && ((var_fn25_calc_iq__beta) as f64).is_finite() && ((var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (var_fn25_calc_iq__beta * ((assign3230_e4913).powf(var_fn25_calc_iq__beta - 1.0) * assign3230_e4913_d_n7)) } } else { (assign3230_e4915 * (var_fn25_calc_iq__beta * (assign3230_e4913_d_n7 / assign3230_e4913))) } / assign3230_e4916))) } / (assign3230_e4920 * assign3230_e4920))), (-(if 0.0 == 0.0 && ((assign3230_e4919) as f64).is_finite() && ((assign3230_e4919) as f64).fract() == 0.0 { if assign3230_e4919 == 0.0 { 0.0 } else { (assign3230_e4919 * ((assign3230_e4916).powf(assign3230_e4919 - 1.0) * if 0.0 == 0.0 && ((var_fn25_calc_iq__beta) as f64).is_finite() && ((var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (var_fn25_calc_iq__beta * ((assign3230_e4913).powf(var_fn25_calc_iq__beta - 1.0) * assign3230_e4913_d_n16)) } } else { (assign3230_e4915 * (var_fn25_calc_iq__beta * (assign3230_e4913_d_n16 / assign3230_e4913))) })) } } else { (assign3230_e4920 * (assign3230_e4919 * (if 0.0 == 0.0 && ((var_fn25_calc_iq__beta) as f64).is_finite() && ((var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (var_fn25_calc_iq__beta * ((assign3230_e4913).powf(var_fn25_calc_iq__beta - 1.0) * assign3230_e4913_d_n16)) } } else { (assign3230_e4915 * (var_fn25_calc_iq__beta * (assign3230_e4913_d_n16 / assign3230_e4913))) } / assign3230_e4916))) } / (assign3230_e4920 * assign3230_e4920))), (-(if 0.0 == 0.0 && ((assign3230_e4919) as f64).is_finite() && ((assign3230_e4919) as f64).fract() == 0.0 { if assign3230_e4919 == 0.0 { 0.0 } else { (assign3230_e4919 * ((assign3230_e4916).powf(assign3230_e4919 - 1.0) * if 0.0 == 0.0 && ((var_fn25_calc_iq__beta) as f64).is_finite() && ((var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (var_fn25_calc_iq__beta * ((assign3230_e4913).powf(var_fn25_calc_iq__beta - 1.0) * assign3230_e4913_d_n17)) } } else { (assign3230_e4915 * (var_fn25_calc_iq__beta * (assign3230_e4913_d_n17 / assign3230_e4913))) })) } } else { (assign3230_e4920 * (assign3230_e4919 * (if 0.0 == 0.0 && ((var_fn25_calc_iq__beta) as f64).is_finite() && ((var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (var_fn25_calc_iq__beta * ((assign3230_e4913).powf(var_fn25_calc_iq__beta - 1.0) * assign3230_e4913_d_n17)) } } else { (assign3230_e4915 * (var_fn25_calc_iq__beta * (assign3230_e4913_d_n17 / assign3230_e4913))) } / assign3230_e4916))) } / (assign3230_e4920 * assign3230_e4920))),)
    } else {
        (var_fn25_calc_iq__fds, var_fn25_calc_iq__fds_dn2, var_fn25_calc_iq__fds_dn3, var_fn25_calc_iq__fds_dn4, var_fn25_calc_iq__fds_dn7, var_fn25_calc_iq__fds_dn16, var_fn25_calc_iq__fds_dn17,)
    }
};
        var_fn25_calc_iq__fds = assign3230_e4923;
        var_fn25_calc_iq__fds_dn2 = assign3230_e4923_d_n2;
        var_fn25_calc_iq__fds_dn3 = assign3230_e4923_d_n3;
        var_fn25_calc_iq__fds_dn4 = assign3230_e4923_d_n4;
        var_fn25_calc_iq__fds_dn7 = assign3230_e4923_d_n7;
        var_fn25_calc_iq__fds_dn16 = assign3230_e4923_d_n16;
        var_fn25_calc_iq__fds_dn17 = assign3230_e4923_d_n17;
        var_fn25_calc_iq__fds_rv = 0.0;

        let (assign3240_e4930, assign3240_e4930_d_n2, assign3240_e4930_d_n3, assign3240_e4930_d_n4, assign3240_e4930_d_n7, assign3240_e4930_d_n16, assign3240_e4930_d_n17,) = {
    if (var_guard24 != 0.0) {
        let assign3240_e4926: f64 = (-var_fn25_calc_iq__vdsin);
        let assign3240_e4928: f64 = (assign3240_e4926 * var_fn25_calc_iq__fds);
        (assign3240_e4928, (assign3240_e4926 * var_fn25_calc_iq__fds_dn2), (assign3240_e4926 * var_fn25_calc_iq__fds_dn3), (assign3240_e4926 * var_fn25_calc_iq__fds_dn4), (assign3240_e4926 * var_fn25_calc_iq__fds_dn7), (((-var_fn25_calc_iq__vdsin_dn16) * var_fn25_calc_iq__fds) + (assign3240_e4926 * var_fn25_calc_iq__fds_dn16)), (((-var_fn25_calc_iq__vdsin_dn17) * var_fn25_calc_iq__fds) + (assign3240_e4926 * var_fn25_calc_iq__fds_dn17)),)
    } else {
        (var_fn25_calc_iq__vsx, var_fn25_calc_iq__vsx_dn2, var_fn25_calc_iq__vsx_dn3, var_fn25_calc_iq__vsx_dn4, var_fn25_calc_iq__vsx_dn7, var_fn25_calc_iq__vsx_dn16, var_fn25_calc_iq__vsx_dn17,)
    }
};
        var_fn25_calc_iq__vsx = assign3240_e4930;
        var_fn25_calc_iq__vsx_dn2 = assign3240_e4930_d_n2;
        var_fn25_calc_iq__vsx_dn3 = assign3240_e4930_d_n3;
        var_fn25_calc_iq__vsx_dn4 = assign3240_e4930_d_n4;
        var_fn25_calc_iq__vsx_dn7 = assign3240_e4930_d_n7;
        var_fn25_calc_iq__vsx_dn16 = assign3240_e4930_d_n16;
        var_fn25_calc_iq__vsx_dn17 = assign3240_e4930_d_n17;
        var_fn25_calc_iq__vsx_rv = 0.0;

        let (assign3250_e4938, assign3250_e4938_d_n2, assign3250_e4938_d_n3, assign3250_e4938_d_n4, assign3250_e4938_d_n7, assign3250_e4938_d_n16, assign3250_e4938_d_n17,) = {
    if (var_guard24 != 0.0) {
        let assign3250_e4934: f64 = (var_fn25_calc_iq__vgsin - var_fn25_calc_iq__myarg);
        let assign3250_e4936: f64 = (assign3250_e4934 / var_fn25_calc_iq__alpha_phit);
        (assign3250_e4936, ((var_fn25_calc_iq__vgsin_dn2 - var_fn25_calc_iq__myarg_dn2) / var_fn25_calc_iq__alpha_phit), ((-var_fn25_calc_iq__myarg_dn3) / var_fn25_calc_iq__alpha_phit), ((((-var_fn25_calc_iq__myarg_dn4) * var_fn25_calc_iq__alpha_phit) - (assign3250_e4934 * var_fn25_calc_iq__alpha_phit_dn4)) / (var_fn25_calc_iq__alpha_phit * var_fn25_calc_iq__alpha_phit)), ((var_fn25_calc_iq__vgsin_dn7 - var_fn25_calc_iq__myarg_dn7) / var_fn25_calc_iq__alpha_phit), ((var_fn25_calc_iq__vgsin_dn16 - var_fn25_calc_iq__myarg_dn16) / var_fn25_calc_iq__alpha_phit), ((-var_fn25_calc_iq__myarg_dn17) / var_fn25_calc_iq__alpha_phit),)
    } else {
        (var_fn25_calc_iq__exparg, var_fn25_calc_iq__exparg_dn2, var_fn25_calc_iq__exparg_dn3, var_fn25_calc_iq__exparg_dn4, var_fn25_calc_iq__exparg_dn7, var_fn25_calc_iq__exparg_dn16, var_fn25_calc_iq__exparg_dn17,)
    }
};
        var_fn25_calc_iq__exparg = assign3250_e4938;
        var_fn25_calc_iq__exparg_dn2 = assign3250_e4938_d_n2;
        var_fn25_calc_iq__exparg_dn3 = assign3250_e4938_d_n3;
        var_fn25_calc_iq__exparg_dn4 = assign3250_e4938_d_n4;
        var_fn25_calc_iq__exparg_dn7 = assign3250_e4938_d_n7;
        var_fn25_calc_iq__exparg_dn16 = assign3250_e4938_d_n16;
        var_fn25_calc_iq__exparg_dn17 = assign3250_e4938_d_n17;
        var_fn25_calc_iq__exparg_rv = 0.0;

        let assign3260_e4941: f64 = if var_fn25_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        var_guard31 = assign3260_e4941;
        var_guard31_rv = 0.0;

        let (assign3270_e4947, assign3270_e4947_d_n2, assign3270_e4947_d_n3, assign3270_e4947_d_n4, assign3270_e4947_d_n7, assign3270_e4947_d_n16, assign3270_e4947_d_n17,) = {
    if ((var_guard24 != 0.0) && (var_guard31 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__ffs, var_fn25_calc_iq__ffs_dn2, var_fn25_calc_iq__ffs_dn3, var_fn25_calc_iq__ffs_dn4, var_fn25_calc_iq__ffs_dn7, var_fn25_calc_iq__ffs_dn16, var_fn25_calc_iq__ffs_dn17,)
    }
};
        var_fn25_calc_iq__ffs = assign3270_e4947;
        var_fn25_calc_iq__ffs_dn2 = assign3270_e4947_d_n2;
        var_fn25_calc_iq__ffs_dn3 = assign3270_e4947_d_n3;
        var_fn25_calc_iq__ffs_dn4 = assign3270_e4947_d_n4;
        var_fn25_calc_iq__ffs_dn7 = assign3270_e4947_d_n7;
        var_fn25_calc_iq__ffs_dn16 = assign3270_e4947_d_n16;
        var_fn25_calc_iq__ffs_dn17 = assign3270_e4947_d_n17;
        var_fn25_calc_iq__ffs_rv = 0.0;

        let assign3280_e4950: f64 = (-50.0);
        let assign3280_e4951: f64 = if var_fn25_calc_iq__exparg < assign3280_e4950 { 1.0 } else { 0.0 };
        var_guard32 = assign3280_e4951;
        var_guard32_rv = 0.0;

        let (assign3290_e4960, assign3290_e4960_d_n2, assign3290_e4960_d_n3, assign3290_e4960_d_n4, assign3290_e4960_d_n7, assign3290_e4960_d_n16, assign3290_e4960_d_n17,) = {
    if (((var_guard24 != 0.0) && (var_guard31 == 0.0)) && (var_guard32 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__ffs, var_fn25_calc_iq__ffs_dn2, var_fn25_calc_iq__ffs_dn3, var_fn25_calc_iq__ffs_dn4, var_fn25_calc_iq__ffs_dn7, var_fn25_calc_iq__ffs_dn16, var_fn25_calc_iq__ffs_dn17,)
    }
};
        var_fn25_calc_iq__ffs = assign3290_e4960;
        var_fn25_calc_iq__ffs_dn2 = assign3290_e4960_d_n2;
        var_fn25_calc_iq__ffs_dn3 = assign3290_e4960_d_n3;
        var_fn25_calc_iq__ffs_dn4 = assign3290_e4960_d_n4;
        var_fn25_calc_iq__ffs_dn7 = assign3290_e4960_d_n7;
        var_fn25_calc_iq__ffs_dn16 = assign3290_e4960_d_n16;
        var_fn25_calc_iq__ffs_dn17 = assign3290_e4960_d_n17;
        var_fn25_calc_iq__ffs_rv = 0.0;

        let (assign3300_e4975, assign3300_e4975_d_n2, assign3300_e4975_d_n3, assign3300_e4975_d_n4, assign3300_e4975_d_n7, assign3300_e4975_d_n16, assign3300_e4975_d_n17,) = {
    if (((var_guard24 != 0.0) && (var_guard31 == 0.0)) && (var_guard32 == 0.0)) {
        let assign3300_e4971: f64 = (var_fn25_calc_iq__exparg).exp();
        let assign3300_e4972: f64 = (1.0 + assign3300_e4971);
        let assign3300_e4973: f64 = (1.0 / assign3300_e4972);
        (assign3300_e4973, (-((assign3300_e4971 * var_fn25_calc_iq__exparg_dn2) / (assign3300_e4972 * assign3300_e4972))), (-((assign3300_e4971 * var_fn25_calc_iq__exparg_dn3) / (assign3300_e4972 * assign3300_e4972))), (-((assign3300_e4971 * var_fn25_calc_iq__exparg_dn4) / (assign3300_e4972 * assign3300_e4972))), (-((assign3300_e4971 * var_fn25_calc_iq__exparg_dn7) / (assign3300_e4972 * assign3300_e4972))), (-((assign3300_e4971 * var_fn25_calc_iq__exparg_dn16) / (assign3300_e4972 * assign3300_e4972))), (-((assign3300_e4971 * var_fn25_calc_iq__exparg_dn17) / (assign3300_e4972 * assign3300_e4972))),)
    } else {
        (var_fn25_calc_iq__ffs, var_fn25_calc_iq__ffs_dn2, var_fn25_calc_iq__ffs_dn3, var_fn25_calc_iq__ffs_dn4, var_fn25_calc_iq__ffs_dn7, var_fn25_calc_iq__ffs_dn16, var_fn25_calc_iq__ffs_dn17,)
    }
};
        var_fn25_calc_iq__ffs = assign3300_e4975;
        var_fn25_calc_iq__ffs_dn2 = assign3300_e4975_d_n2;
        var_fn25_calc_iq__ffs_dn3 = assign3300_e4975_d_n3;
        var_fn25_calc_iq__ffs_dn4 = assign3300_e4975_d_n4;
        var_fn25_calc_iq__ffs_dn7 = assign3300_e4975_d_n7;
        var_fn25_calc_iq__ffs_dn16 = assign3300_e4975_d_n16;
        var_fn25_calc_iq__ffs_dn17 = assign3300_e4975_d_n17;
        var_fn25_calc_iq__ffs_rv = 0.0;

        let (assign3310_e4993, assign3310_e4993_d_n2, assign3310_e4993_d_n3, assign3310_e4993_d_n4, assign3310_e4993_d_n7, assign3310_e4993_d_n16, assign3310_e4993_d_n17,) = {
    if (var_guard24 != 0.0) {
        let assign3310_e4979: f64 = (var_fn25_calc_iq__vgdin - var_fn25_calc_iq__vsx);
        let assign3310_e4983: f64 = (p.p51 * 0.1);
        let assign3310_e4985: f64 = (assign3310_e4983 * var_fn25_calc_iq__alpha_phit);
        let assign3310_e4987: f64 = (assign3310_e4985 * var_fn25_calc_iq__ffs);
        let assign3310_e4988: f64 = (var_fn25_calc_iq__vtdibl - assign3310_e4987);
        let assign3310_e4989: f64 = (assign3310_e4979 - assign3310_e4988);
        let assign3310_e4991: f64 = (assign3310_e4989 / var_fn25_calc_iq__two_n_phit);
        (assign3310_e4991, (((var_fn25_calc_iq__vgdin_dn2 - var_fn25_calc_iq__vsx_dn2) - (-(assign3310_e4985 * var_fn25_calc_iq__ffs_dn2))) / var_fn25_calc_iq__two_n_phit), (((-var_fn25_calc_iq__vsx_dn3) - (-(assign3310_e4985 * var_fn25_calc_iq__ffs_dn3))) / var_fn25_calc_iq__two_n_phit), (((((-var_fn25_calc_iq__vsx_dn4) - (var_fn25_calc_iq__vtdibl_dn4 - (((assign3310_e4983 * var_fn25_calc_iq__alpha_phit_dn4) * var_fn25_calc_iq__ffs) + (assign3310_e4985 * var_fn25_calc_iq__ffs_dn4)))) * var_fn25_calc_iq__two_n_phit) - (assign3310_e4989 * var_fn25_calc_iq__two_n_phit_dn4)) / (var_fn25_calc_iq__two_n_phit * var_fn25_calc_iq__two_n_phit)), (((var_fn25_calc_iq__vgdin_dn7 - var_fn25_calc_iq__vsx_dn7) - (-(assign3310_e4985 * var_fn25_calc_iq__ffs_dn7))) / var_fn25_calc_iq__two_n_phit), (((((var_fn25_calc_iq__vgdin_dn16 - var_fn25_calc_iq__vsx_dn16) - (var_fn25_calc_iq__vtdibl_dn16 - (assign3310_e4985 * var_fn25_calc_iq__ffs_dn16))) * var_fn25_calc_iq__two_n_phit) - (assign3310_e4989 * var_fn25_calc_iq__two_n_phit_dn16)) / (var_fn25_calc_iq__two_n_phit * var_fn25_calc_iq__two_n_phit)), (((((var_fn25_calc_iq__vgdin_dn17 - var_fn25_calc_iq__vsx_dn17) - (var_fn25_calc_iq__vtdibl_dn17 - (assign3310_e4985 * var_fn25_calc_iq__ffs_dn17))) * var_fn25_calc_iq__two_n_phit) - (assign3310_e4989 * var_fn25_calc_iq__two_n_phit_dn17)) / (var_fn25_calc_iq__two_n_phit * var_fn25_calc_iq__two_n_phit)),)
    } else {
        (var_fn25_calc_iq__etas, var_fn25_calc_iq__etas_dn2, var_fn25_calc_iq__etas_dn3, var_fn25_calc_iq__etas_dn4, var_fn25_calc_iq__etas_dn7, var_fn25_calc_iq__etas_dn16, var_fn25_calc_iq__etas_dn17,)
    }
};
        var_fn25_calc_iq__etas = assign3310_e4993;
        var_fn25_calc_iq__etas_dn2 = assign3310_e4993_d_n2;
        var_fn25_calc_iq__etas_dn3 = assign3310_e4993_d_n3;
        var_fn25_calc_iq__etas_dn4 = assign3310_e4993_d_n4;
        var_fn25_calc_iq__etas_dn7 = assign3310_e4993_d_n7;
        var_fn25_calc_iq__etas_dn16 = assign3310_e4993_d_n16;
        var_fn25_calc_iq__etas_dn17 = assign3310_e4993_d_n17;
        var_fn25_calc_iq__etas_rv = 0.0;

        let assign3320_e4996: f64 = if var_fn25_calc_iq__etas > 50.0 { 1.0 } else { 0.0 };
        var_guard33 = assign3320_e4996;
        var_guard33_rv = 0.0;

        let (assign3330_e5004, assign3330_e5004_d_n2, assign3330_e5004_d_n3, assign3330_e5004_d_n4, assign3330_e5004_d_n7, assign3330_e5004_d_n16, assign3330_e5004_d_n17,) = {
    if ((var_guard24 != 0.0) && (var_guard33 != 0.0)) {
        let assign3330_e5002: f64 = (var_fn25_calc_iq__qref * var_fn25_calc_iq__etas);
        (assign3330_e5002, (var_fn25_calc_iq__qref * var_fn25_calc_iq__etas_dn2), (var_fn25_calc_iq__qref * var_fn25_calc_iq__etas_dn3), ((var_fn25_calc_iq__qref_dn4 * var_fn25_calc_iq__etas) + (var_fn25_calc_iq__qref * var_fn25_calc_iq__etas_dn4)), (var_fn25_calc_iq__qref * var_fn25_calc_iq__etas_dn7), ((var_fn25_calc_iq__qref_dn16 * var_fn25_calc_iq__etas) + (var_fn25_calc_iq__qref * var_fn25_calc_iq__etas_dn16)), ((var_fn25_calc_iq__qref_dn17 * var_fn25_calc_iq__etas) + (var_fn25_calc_iq__qref * var_fn25_calc_iq__etas_dn17)),)
    } else {
        (var_fn25_calc_iq__qinvs, var_fn25_calc_iq__qinvs_dn2, var_fn25_calc_iq__qinvs_dn3, var_fn25_calc_iq__qinvs_dn4, var_fn25_calc_iq__qinvs_dn7, var_fn25_calc_iq__qinvs_dn16, var_fn25_calc_iq__qinvs_dn17,)
    }
};
        var_fn25_calc_iq__qinvs = assign3330_e5004;
        var_fn25_calc_iq__qinvs_dn2 = assign3330_e5004_d_n2;
        var_fn25_calc_iq__qinvs_dn3 = assign3330_e5004_d_n3;
        var_fn25_calc_iq__qinvs_dn4 = assign3330_e5004_d_n4;
        var_fn25_calc_iq__qinvs_dn7 = assign3330_e5004_d_n7;
        var_fn25_calc_iq__qinvs_dn16 = assign3330_e5004_d_n16;
        var_fn25_calc_iq__qinvs_dn17 = assign3330_e5004_d_n17;
        var_fn25_calc_iq__qinvs_rv = 0.0;

        let assign3340_e5007: f64 = (-50.0);
        let assign3340_e5008: f64 = if var_fn25_calc_iq__etas < assign3340_e5007 { 1.0 } else { 0.0 };
        var_guard34 = assign3340_e5008;
        var_guard34_rv = 0.0;

        let (assign3350_e5020, assign3350_e5020_d_n2, assign3350_e5020_d_n3, assign3350_e5020_d_n4, assign3350_e5020_d_n7, assign3350_e5020_d_n16, assign3350_e5020_d_n17,) = {
    if (((var_guard24 != 0.0) && (var_guard33 == 0.0)) && (var_guard34 != 0.0)) {
        let assign3350_e5017: f64 = (var_fn25_calc_iq__etas).exp();
        let assign3350_e5018: f64 = (var_fn25_calc_iq__qref * assign3350_e5017);
        (assign3350_e5018, (var_fn25_calc_iq__qref * (assign3350_e5017 * var_fn25_calc_iq__etas_dn2)), (var_fn25_calc_iq__qref * (assign3350_e5017 * var_fn25_calc_iq__etas_dn3)), ((var_fn25_calc_iq__qref_dn4 * assign3350_e5017) + (var_fn25_calc_iq__qref * (assign3350_e5017 * var_fn25_calc_iq__etas_dn4))), (var_fn25_calc_iq__qref * (assign3350_e5017 * var_fn25_calc_iq__etas_dn7)), ((var_fn25_calc_iq__qref_dn16 * assign3350_e5017) + (var_fn25_calc_iq__qref * (assign3350_e5017 * var_fn25_calc_iq__etas_dn16))), ((var_fn25_calc_iq__qref_dn17 * assign3350_e5017) + (var_fn25_calc_iq__qref * (assign3350_e5017 * var_fn25_calc_iq__etas_dn17))),)
    } else {
        (var_fn25_calc_iq__qinvs, var_fn25_calc_iq__qinvs_dn2, var_fn25_calc_iq__qinvs_dn3, var_fn25_calc_iq__qinvs_dn4, var_fn25_calc_iq__qinvs_dn7, var_fn25_calc_iq__qinvs_dn16, var_fn25_calc_iq__qinvs_dn17,)
    }
};
        var_fn25_calc_iq__qinvs = assign3350_e5020;
        var_fn25_calc_iq__qinvs_dn2 = assign3350_e5020_d_n2;
        var_fn25_calc_iq__qinvs_dn3 = assign3350_e5020_d_n3;
        var_fn25_calc_iq__qinvs_dn4 = assign3350_e5020_d_n4;
        var_fn25_calc_iq__qinvs_dn7 = assign3350_e5020_d_n7;
        var_fn25_calc_iq__qinvs_dn16 = assign3350_e5020_d_n16;
        var_fn25_calc_iq__qinvs_dn17 = assign3350_e5020_d_n17;
        var_fn25_calc_iq__qinvs_rv = 0.0;

        let (assign3360_e5036, assign3360_e5036_d_n2, assign3360_e5036_d_n3, assign3360_e5036_d_n4, assign3360_e5036_d_n7, assign3360_e5036_d_n16, assign3360_e5036_d_n17,) = {
    if (((var_guard24 != 0.0) && (var_guard33 == 0.0)) && (var_guard34 == 0.0)) {
        let assign3360_e5031: f64 = (var_fn25_calc_iq__etas).exp();
        let assign3360_e5032: f64 = (1.0 + assign3360_e5031);
        let assign3360_e5033: f64 = (assign3360_e5032).ln();
        let assign3360_e5034: f64 = (var_fn25_calc_iq__qref * assign3360_e5033);
        (assign3360_e5034, (var_fn25_calc_iq__qref * ((assign3360_e5031 * var_fn25_calc_iq__etas_dn2) / assign3360_e5032)), (var_fn25_calc_iq__qref * ((assign3360_e5031 * var_fn25_calc_iq__etas_dn3) / assign3360_e5032)), ((var_fn25_calc_iq__qref_dn4 * assign3360_e5033) + (var_fn25_calc_iq__qref * ((assign3360_e5031 * var_fn25_calc_iq__etas_dn4) / assign3360_e5032))), (var_fn25_calc_iq__qref * ((assign3360_e5031 * var_fn25_calc_iq__etas_dn7) / assign3360_e5032)), ((var_fn25_calc_iq__qref_dn16 * assign3360_e5033) + (var_fn25_calc_iq__qref * ((assign3360_e5031 * var_fn25_calc_iq__etas_dn16) / assign3360_e5032))), ((var_fn25_calc_iq__qref_dn17 * assign3360_e5033) + (var_fn25_calc_iq__qref * ((assign3360_e5031 * var_fn25_calc_iq__etas_dn17) / assign3360_e5032))),)
    } else {
        (var_fn25_calc_iq__qinvs, var_fn25_calc_iq__qinvs_dn2, var_fn25_calc_iq__qinvs_dn3, var_fn25_calc_iq__qinvs_dn4, var_fn25_calc_iq__qinvs_dn7, var_fn25_calc_iq__qinvs_dn16, var_fn25_calc_iq__qinvs_dn17,)
    }
};
        var_fn25_calc_iq__qinvs = assign3360_e5036;
        var_fn25_calc_iq__qinvs_dn2 = assign3360_e5036_d_n2;
        var_fn25_calc_iq__qinvs_dn3 = assign3360_e5036_d_n3;
        var_fn25_calc_iq__qinvs_dn4 = assign3360_e5036_d_n4;
        var_fn25_calc_iq__qinvs_dn7 = assign3360_e5036_d_n7;
        var_fn25_calc_iq__qinvs_dn16 = assign3360_e5036_d_n16;
        var_fn25_calc_iq__qinvs_dn17 = assign3360_e5036_d_n17;
        var_fn25_calc_iq__qinvs_rv = 0.0;

        let (assign3370_e5044, assign3370_e5044_d_n2, assign3370_e5044_d_n3, assign3370_e5044_d_n4, assign3370_e5044_d_n7, assign3370_e5044_d_n16, assign3370_e5044_d_n17,) = {
    if (var_guard24 != 0.0) {
        let assign3370_e5040: f64 = (var_fn25_calc_iq__vgdin - var_fn25_calc_iq__myarg);
        let assign3370_e5042: f64 = (assign3370_e5040 / var_fn25_calc_iq__alpha_phit);
        (assign3370_e5042, ((var_fn25_calc_iq__vgdin_dn2 - var_fn25_calc_iq__myarg_dn2) / var_fn25_calc_iq__alpha_phit), ((-var_fn25_calc_iq__myarg_dn3) / var_fn25_calc_iq__alpha_phit), ((((-var_fn25_calc_iq__myarg_dn4) * var_fn25_calc_iq__alpha_phit) - (assign3370_e5040 * var_fn25_calc_iq__alpha_phit_dn4)) / (var_fn25_calc_iq__alpha_phit * var_fn25_calc_iq__alpha_phit)), ((var_fn25_calc_iq__vgdin_dn7 - var_fn25_calc_iq__myarg_dn7) / var_fn25_calc_iq__alpha_phit), ((var_fn25_calc_iq__vgdin_dn16 - var_fn25_calc_iq__myarg_dn16) / var_fn25_calc_iq__alpha_phit), ((var_fn25_calc_iq__vgdin_dn17 - var_fn25_calc_iq__myarg_dn17) / var_fn25_calc_iq__alpha_phit),)
    } else {
        (var_fn25_calc_iq__exparg, var_fn25_calc_iq__exparg_dn2, var_fn25_calc_iq__exparg_dn3, var_fn25_calc_iq__exparg_dn4, var_fn25_calc_iq__exparg_dn7, var_fn25_calc_iq__exparg_dn16, var_fn25_calc_iq__exparg_dn17,)
    }
};
        var_fn25_calc_iq__exparg = assign3370_e5044;
        var_fn25_calc_iq__exparg_dn2 = assign3370_e5044_d_n2;
        var_fn25_calc_iq__exparg_dn3 = assign3370_e5044_d_n3;
        var_fn25_calc_iq__exparg_dn4 = assign3370_e5044_d_n4;
        var_fn25_calc_iq__exparg_dn7 = assign3370_e5044_d_n7;
        var_fn25_calc_iq__exparg_dn16 = assign3370_e5044_d_n16;
        var_fn25_calc_iq__exparg_dn17 = assign3370_e5044_d_n17;
        var_fn25_calc_iq__exparg_rv = 0.0;

        let assign3380_e5047: f64 = if var_fn25_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        var_guard35 = assign3380_e5047;
        var_guard35_rv = 0.0;

        let (assign3390_e5053, assign3390_e5053_d_n2, assign3390_e5053_d_n3, assign3390_e5053_d_n4, assign3390_e5053_d_n7, assign3390_e5053_d_n16, assign3390_e5053_d_n17,) = {
    if ((var_guard24 != 0.0) && (var_guard35 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__ffd, var_fn25_calc_iq__ffd_dn2, var_fn25_calc_iq__ffd_dn3, var_fn25_calc_iq__ffd_dn4, var_fn25_calc_iq__ffd_dn7, var_fn25_calc_iq__ffd_dn16, var_fn25_calc_iq__ffd_dn17,)
    }
};
        var_fn25_calc_iq__ffd = assign3390_e5053;
        var_fn25_calc_iq__ffd_dn2 = assign3390_e5053_d_n2;
        var_fn25_calc_iq__ffd_dn3 = assign3390_e5053_d_n3;
        var_fn25_calc_iq__ffd_dn4 = assign3390_e5053_d_n4;
        var_fn25_calc_iq__ffd_dn7 = assign3390_e5053_d_n7;
        var_fn25_calc_iq__ffd_dn16 = assign3390_e5053_d_n16;
        var_fn25_calc_iq__ffd_dn17 = assign3390_e5053_d_n17;
        var_fn25_calc_iq__ffd_rv = 0.0;

        let assign3400_e5056: f64 = (-50.0);
        let assign3400_e5057: f64 = if var_fn25_calc_iq__exparg < assign3400_e5056 { 1.0 } else { 0.0 };
        var_guard36 = assign3400_e5057;
        var_guard36_rv = 0.0;

        let (assign3410_e5066, assign3410_e5066_d_n2, assign3410_e5066_d_n3, assign3410_e5066_d_n4, assign3410_e5066_d_n7, assign3410_e5066_d_n16, assign3410_e5066_d_n17,) = {
    if (((var_guard24 != 0.0) && (var_guard35 == 0.0)) && (var_guard36 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__ffd, var_fn25_calc_iq__ffd_dn2, var_fn25_calc_iq__ffd_dn3, var_fn25_calc_iq__ffd_dn4, var_fn25_calc_iq__ffd_dn7, var_fn25_calc_iq__ffd_dn16, var_fn25_calc_iq__ffd_dn17,)
    }
};
        var_fn25_calc_iq__ffd = assign3410_e5066;
        var_fn25_calc_iq__ffd_dn2 = assign3410_e5066_d_n2;
        var_fn25_calc_iq__ffd_dn3 = assign3410_e5066_d_n3;
        var_fn25_calc_iq__ffd_dn4 = assign3410_e5066_d_n4;
        var_fn25_calc_iq__ffd_dn7 = assign3410_e5066_d_n7;
        var_fn25_calc_iq__ffd_dn16 = assign3410_e5066_d_n16;
        var_fn25_calc_iq__ffd_dn17 = assign3410_e5066_d_n17;
        var_fn25_calc_iq__ffd_rv = 0.0;

        let (assign3420_e5081, assign3420_e5081_d_n2, assign3420_e5081_d_n3, assign3420_e5081_d_n4, assign3420_e5081_d_n7, assign3420_e5081_d_n16, assign3420_e5081_d_n17,) = {
    if (((var_guard24 != 0.0) && (var_guard35 == 0.0)) && (var_guard36 == 0.0)) {
        let assign3420_e5077: f64 = (var_fn25_calc_iq__exparg).exp();
        let assign3420_e5078: f64 = (1.0 + assign3420_e5077);
        let assign3420_e5079: f64 = (1.0 / assign3420_e5078);
        (assign3420_e5079, (-((assign3420_e5077 * var_fn25_calc_iq__exparg_dn2) / (assign3420_e5078 * assign3420_e5078))), (-((assign3420_e5077 * var_fn25_calc_iq__exparg_dn3) / (assign3420_e5078 * assign3420_e5078))), (-((assign3420_e5077 * var_fn25_calc_iq__exparg_dn4) / (assign3420_e5078 * assign3420_e5078))), (-((assign3420_e5077 * var_fn25_calc_iq__exparg_dn7) / (assign3420_e5078 * assign3420_e5078))), (-((assign3420_e5077 * var_fn25_calc_iq__exparg_dn16) / (assign3420_e5078 * assign3420_e5078))), (-((assign3420_e5077 * var_fn25_calc_iq__exparg_dn17) / (assign3420_e5078 * assign3420_e5078))),)
    } else {
        (var_fn25_calc_iq__ffd, var_fn25_calc_iq__ffd_dn2, var_fn25_calc_iq__ffd_dn3, var_fn25_calc_iq__ffd_dn4, var_fn25_calc_iq__ffd_dn7, var_fn25_calc_iq__ffd_dn16, var_fn25_calc_iq__ffd_dn17,)
    }
};
        var_fn25_calc_iq__ffd = assign3420_e5081;
        var_fn25_calc_iq__ffd_dn2 = assign3420_e5081_d_n2;
        var_fn25_calc_iq__ffd_dn3 = assign3420_e5081_d_n3;
        var_fn25_calc_iq__ffd_dn4 = assign3420_e5081_d_n4;
        var_fn25_calc_iq__ffd_dn7 = assign3420_e5081_d_n7;
        var_fn25_calc_iq__ffd_dn16 = assign3420_e5081_d_n16;
        var_fn25_calc_iq__ffd_dn17 = assign3420_e5081_d_n17;
        var_fn25_calc_iq__ffd_rv = 0.0;

        let (assign3430_e5099, assign3430_e5099_d_n2, assign3430_e5099_d_n3, assign3430_e5099_d_n4, assign3430_e5099_d_n7, assign3430_e5099_d_n16, assign3430_e5099_d_n17,) = {
    if (var_guard24 != 0.0) {
        let assign3430_e5085: f64 = (var_fn25_calc_iq__vgsin - var_fn25_calc_iq__vdx);
        let assign3430_e5089: f64 = (p.p51 * 0.1);
        let assign3430_e5091: f64 = (assign3430_e5089 * var_fn25_calc_iq__alpha_phit);
        let assign3430_e5093: f64 = (assign3430_e5091 * var_fn25_calc_iq__ffd);
        let assign3430_e5094: f64 = (var_fn25_calc_iq__vtdibl - assign3430_e5093);
        let assign3430_e5095: f64 = (assign3430_e5085 - assign3430_e5094);
        let assign3430_e5097: f64 = (assign3430_e5095 / var_fn25_calc_iq__two_n_phit);
        (assign3430_e5097, (((var_fn25_calc_iq__vgsin_dn2 - var_fn25_calc_iq__vdx_dn2) - (-(assign3430_e5091 * var_fn25_calc_iq__ffd_dn2))) / var_fn25_calc_iq__two_n_phit), (((-var_fn25_calc_iq__vdx_dn3) - (-(assign3430_e5091 * var_fn25_calc_iq__ffd_dn3))) / var_fn25_calc_iq__two_n_phit), (((((-var_fn25_calc_iq__vdx_dn4) - (var_fn25_calc_iq__vtdibl_dn4 - (((assign3430_e5089 * var_fn25_calc_iq__alpha_phit_dn4) * var_fn25_calc_iq__ffd) + (assign3430_e5091 * var_fn25_calc_iq__ffd_dn4)))) * var_fn25_calc_iq__two_n_phit) - (assign3430_e5095 * var_fn25_calc_iq__two_n_phit_dn4)) / (var_fn25_calc_iq__two_n_phit * var_fn25_calc_iq__two_n_phit)), (((var_fn25_calc_iq__vgsin_dn7 - var_fn25_calc_iq__vdx_dn7) - (-(assign3430_e5091 * var_fn25_calc_iq__ffd_dn7))) / var_fn25_calc_iq__two_n_phit), (((((var_fn25_calc_iq__vgsin_dn16 - var_fn25_calc_iq__vdx_dn16) - (var_fn25_calc_iq__vtdibl_dn16 - (assign3430_e5091 * var_fn25_calc_iq__ffd_dn16))) * var_fn25_calc_iq__two_n_phit) - (assign3430_e5095 * var_fn25_calc_iq__two_n_phit_dn16)) / (var_fn25_calc_iq__two_n_phit * var_fn25_calc_iq__two_n_phit)), (((((-var_fn25_calc_iq__vdx_dn17) - (var_fn25_calc_iq__vtdibl_dn17 - (assign3430_e5091 * var_fn25_calc_iq__ffd_dn17))) * var_fn25_calc_iq__two_n_phit) - (assign3430_e5095 * var_fn25_calc_iq__two_n_phit_dn17)) / (var_fn25_calc_iq__two_n_phit * var_fn25_calc_iq__two_n_phit)),)
    } else {
        (var_fn25_calc_iq__etad, var_fn25_calc_iq__etad_dn2, var_fn25_calc_iq__etad_dn3, var_fn25_calc_iq__etad_dn4, var_fn25_calc_iq__etad_dn7, var_fn25_calc_iq__etad_dn16, var_fn25_calc_iq__etad_dn17,)
    }
};
        var_fn25_calc_iq__etad = assign3430_e5099;
        var_fn25_calc_iq__etad_dn2 = assign3430_e5099_d_n2;
        var_fn25_calc_iq__etad_dn3 = assign3430_e5099_d_n3;
        var_fn25_calc_iq__etad_dn4 = assign3430_e5099_d_n4;
        var_fn25_calc_iq__etad_dn7 = assign3430_e5099_d_n7;
        var_fn25_calc_iq__etad_dn16 = assign3430_e5099_d_n16;
        var_fn25_calc_iq__etad_dn17 = assign3430_e5099_d_n17;
        var_fn25_calc_iq__etad_rv = 0.0;

        let assign3440_e5102: f64 = if var_fn25_calc_iq__etad > 50.0 { 1.0 } else { 0.0 };
        var_guard37 = assign3440_e5102;
        var_guard37_rv = 0.0;

        let (assign3450_e5110, assign3450_e5110_d_n2, assign3450_e5110_d_n3, assign3450_e5110_d_n4, assign3450_e5110_d_n7, assign3450_e5110_d_n16, assign3450_e5110_d_n17,) = {
    if ((var_guard24 != 0.0) && (var_guard37 != 0.0)) {
        let assign3450_e5108: f64 = (var_fn25_calc_iq__qref * var_fn25_calc_iq__etad);
        (assign3450_e5108, (var_fn25_calc_iq__qref * var_fn25_calc_iq__etad_dn2), (var_fn25_calc_iq__qref * var_fn25_calc_iq__etad_dn3), ((var_fn25_calc_iq__qref_dn4 * var_fn25_calc_iq__etad) + (var_fn25_calc_iq__qref * var_fn25_calc_iq__etad_dn4)), (var_fn25_calc_iq__qref * var_fn25_calc_iq__etad_dn7), ((var_fn25_calc_iq__qref_dn16 * var_fn25_calc_iq__etad) + (var_fn25_calc_iq__qref * var_fn25_calc_iq__etad_dn16)), ((var_fn25_calc_iq__qref_dn17 * var_fn25_calc_iq__etad) + (var_fn25_calc_iq__qref * var_fn25_calc_iq__etad_dn17)),)
    } else {
        (var_fn25_calc_iq__qinvd, var_fn25_calc_iq__qinvd_dn2, var_fn25_calc_iq__qinvd_dn3, var_fn25_calc_iq__qinvd_dn4, var_fn25_calc_iq__qinvd_dn7, var_fn25_calc_iq__qinvd_dn16, var_fn25_calc_iq__qinvd_dn17,)
    }
};
        var_fn25_calc_iq__qinvd = assign3450_e5110;
        var_fn25_calc_iq__qinvd_dn2 = assign3450_e5110_d_n2;
        var_fn25_calc_iq__qinvd_dn3 = assign3450_e5110_d_n3;
        var_fn25_calc_iq__qinvd_dn4 = assign3450_e5110_d_n4;
        var_fn25_calc_iq__qinvd_dn7 = assign3450_e5110_d_n7;
        var_fn25_calc_iq__qinvd_dn16 = assign3450_e5110_d_n16;
        var_fn25_calc_iq__qinvd_dn17 = assign3450_e5110_d_n17;
        var_fn25_calc_iq__qinvd_rv = 0.0;

        let assign3460_e5113: f64 = (-50.0);
        let assign3460_e5114: f64 = if var_fn25_calc_iq__etad < assign3460_e5113 { 1.0 } else { 0.0 };
        var_guard38 = assign3460_e5114;
        var_guard38_rv = 0.0;

        let (assign3470_e5126, assign3470_e5126_d_n2, assign3470_e5126_d_n3, assign3470_e5126_d_n4, assign3470_e5126_d_n7, assign3470_e5126_d_n16, assign3470_e5126_d_n17,) = {
    if (((var_guard24 != 0.0) && (var_guard37 == 0.0)) && (var_guard38 != 0.0)) {
        let assign3470_e5123: f64 = (var_fn25_calc_iq__etad).exp();
        let assign3470_e5124: f64 = (var_fn25_calc_iq__qref * assign3470_e5123);
        (assign3470_e5124, (var_fn25_calc_iq__qref * (assign3470_e5123 * var_fn25_calc_iq__etad_dn2)), (var_fn25_calc_iq__qref * (assign3470_e5123 * var_fn25_calc_iq__etad_dn3)), ((var_fn25_calc_iq__qref_dn4 * assign3470_e5123) + (var_fn25_calc_iq__qref * (assign3470_e5123 * var_fn25_calc_iq__etad_dn4))), (var_fn25_calc_iq__qref * (assign3470_e5123 * var_fn25_calc_iq__etad_dn7)), ((var_fn25_calc_iq__qref_dn16 * assign3470_e5123) + (var_fn25_calc_iq__qref * (assign3470_e5123 * var_fn25_calc_iq__etad_dn16))), ((var_fn25_calc_iq__qref_dn17 * assign3470_e5123) + (var_fn25_calc_iq__qref * (assign3470_e5123 * var_fn25_calc_iq__etad_dn17))),)
    } else {
        (var_fn25_calc_iq__qinvd, var_fn25_calc_iq__qinvd_dn2, var_fn25_calc_iq__qinvd_dn3, var_fn25_calc_iq__qinvd_dn4, var_fn25_calc_iq__qinvd_dn7, var_fn25_calc_iq__qinvd_dn16, var_fn25_calc_iq__qinvd_dn17,)
    }
};
        var_fn25_calc_iq__qinvd = assign3470_e5126;
        var_fn25_calc_iq__qinvd_dn2 = assign3470_e5126_d_n2;
        var_fn25_calc_iq__qinvd_dn3 = assign3470_e5126_d_n3;
        var_fn25_calc_iq__qinvd_dn4 = assign3470_e5126_d_n4;
        var_fn25_calc_iq__qinvd_dn7 = assign3470_e5126_d_n7;
        var_fn25_calc_iq__qinvd_dn16 = assign3470_e5126_d_n16;
        var_fn25_calc_iq__qinvd_dn17 = assign3470_e5126_d_n17;
        var_fn25_calc_iq__qinvd_rv = 0.0;

        let (assign3480_e5142, assign3480_e5142_d_n2, assign3480_e5142_d_n3, assign3480_e5142_d_n4, assign3480_e5142_d_n7, assign3480_e5142_d_n16, assign3480_e5142_d_n17,) = {
    if (((var_guard24 != 0.0) && (var_guard37 == 0.0)) && (var_guard38 == 0.0)) {
        let assign3480_e5137: f64 = (var_fn25_calc_iq__etad).exp();
        let assign3480_e5138: f64 = (1.0 + assign3480_e5137);
        let assign3480_e5139: f64 = (assign3480_e5138).ln();
        let assign3480_e5140: f64 = (var_fn25_calc_iq__qref * assign3480_e5139);
        (assign3480_e5140, (var_fn25_calc_iq__qref * ((assign3480_e5137 * var_fn25_calc_iq__etad_dn2) / assign3480_e5138)), (var_fn25_calc_iq__qref * ((assign3480_e5137 * var_fn25_calc_iq__etad_dn3) / assign3480_e5138)), ((var_fn25_calc_iq__qref_dn4 * assign3480_e5139) + (var_fn25_calc_iq__qref * ((assign3480_e5137 * var_fn25_calc_iq__etad_dn4) / assign3480_e5138))), (var_fn25_calc_iq__qref * ((assign3480_e5137 * var_fn25_calc_iq__etad_dn7) / assign3480_e5138)), ((var_fn25_calc_iq__qref_dn16 * assign3480_e5139) + (var_fn25_calc_iq__qref * ((assign3480_e5137 * var_fn25_calc_iq__etad_dn16) / assign3480_e5138))), ((var_fn25_calc_iq__qref_dn17 * assign3480_e5139) + (var_fn25_calc_iq__qref * ((assign3480_e5137 * var_fn25_calc_iq__etad_dn17) / assign3480_e5138))),)
    } else {
        (var_fn25_calc_iq__qinvd, var_fn25_calc_iq__qinvd_dn2, var_fn25_calc_iq__qinvd_dn3, var_fn25_calc_iq__qinvd_dn4, var_fn25_calc_iq__qinvd_dn7, var_fn25_calc_iq__qinvd_dn16, var_fn25_calc_iq__qinvd_dn17,)
    }
};
        var_fn25_calc_iq__qinvd = assign3480_e5142;
        var_fn25_calc_iq__qinvd_dn2 = assign3480_e5142_d_n2;
        var_fn25_calc_iq__qinvd_dn3 = assign3480_e5142_d_n3;
        var_fn25_calc_iq__qinvd_dn4 = assign3480_e5142_d_n4;
        var_fn25_calc_iq__qinvd_dn7 = assign3480_e5142_d_n7;
        var_fn25_calc_iq__qinvd_dn16 = assign3480_e5142_d_n16;
        var_fn25_calc_iq__qinvd_dn17 = assign3480_e5142_d_n17;
        var_fn25_calc_iq__qinvd_rv = 0.0;

        let (assign3490_e5150, assign3490_e5150_d_n2, assign3490_e5150_d_n3, assign3490_e5150_d_n4, assign3490_e5150_d_n7, assign3490_e5150_d_n16, assign3490_e5150_d_n17,) = {
    if (var_guard24 != 0.0) {
        let assign3490_e5146: f64 = (var_fn25_calc_iq__qinvs - var_fn25_calc_iq__qinvd);
        let assign3490_e5148: f64 = (assign3490_e5146 / var_fn25_calc_iq__cgin);
        (assign3490_e5148, ((var_fn25_calc_iq__qinvs_dn2 - var_fn25_calc_iq__qinvd_dn2) / var_fn25_calc_iq__cgin), ((var_fn25_calc_iq__qinvs_dn3 - var_fn25_calc_iq__qinvd_dn3) / var_fn25_calc_iq__cgin), ((((var_fn25_calc_iq__qinvs_dn4 - var_fn25_calc_iq__qinvd_dn4) * var_fn25_calc_iq__cgin) - (assign3490_e5146 * var_fn25_calc_iq__cgin_dn4)) / (var_fn25_calc_iq__cgin * var_fn25_calc_iq__cgin)), ((var_fn25_calc_iq__qinvs_dn7 - var_fn25_calc_iq__qinvd_dn7) / var_fn25_calc_iq__cgin), ((var_fn25_calc_iq__qinvs_dn16 - var_fn25_calc_iq__qinvd_dn16) / var_fn25_calc_iq__cgin), ((var_fn25_calc_iq__qinvs_dn17 - var_fn25_calc_iq__qinvd_dn17) / var_fn25_calc_iq__cgin),)
    } else {
        (var_fn25_calc_iq__vdsc, var_fn25_calc_iq__vdsc_dn2, var_fn25_calc_iq__vdsc_dn3, var_fn25_calc_iq__vdsc_dn4, var_fn25_calc_iq__vdsc_dn7, var_fn25_calc_iq__vdsc_dn16, var_fn25_calc_iq__vdsc_dn17,)
    }
};
        var_fn25_calc_iq__vdsc = assign3490_e5150;
        var_fn25_calc_iq__vdsc_dn2 = assign3490_e5150_d_n2;
        var_fn25_calc_iq__vdsc_dn3 = assign3490_e5150_d_n3;
        var_fn25_calc_iq__vdsc_dn4 = assign3490_e5150_d_n4;
        var_fn25_calc_iq__vdsc_dn7 = assign3490_e5150_d_n7;
        var_fn25_calc_iq__vdsc_dn16 = assign3490_e5150_d_n16;
        var_fn25_calc_iq__vdsc_dn17 = assign3490_e5150_d_n17;
        var_fn25_calc_iq__vdsc_rv = 0.0;

        let (assign3500_e5156, assign3500_e5156_d_n2, assign3500_e5156_d_n3, assign3500_e5156_d_n4, assign3500_e5156_d_n7, assign3500_e5156_d_n16, assign3500_e5156_d_n17,) = {
    if (var_guard24 != 0.0) {
        let assign3500_e5154: f64 = (var_fn25_calc_iq__vdsc / var_fn25_calc_iq__vdsat);
        (assign3500_e5154, (((var_fn25_calc_iq__vdsc_dn2 * var_fn25_calc_iq__vdsat) - (var_fn25_calc_iq__vdsc * var_fn25_calc_iq__vdsat_dn2)) / (var_fn25_calc_iq__vdsat * var_fn25_calc_iq__vdsat)), (((var_fn25_calc_iq__vdsc_dn3 * var_fn25_calc_iq__vdsat) - (var_fn25_calc_iq__vdsc * var_fn25_calc_iq__vdsat_dn3)) / (var_fn25_calc_iq__vdsat * var_fn25_calc_iq__vdsat)), (((var_fn25_calc_iq__vdsc_dn4 * var_fn25_calc_iq__vdsat) - (var_fn25_calc_iq__vdsc * var_fn25_calc_iq__vdsat_dn4)) / (var_fn25_calc_iq__vdsat * var_fn25_calc_iq__vdsat)), (((var_fn25_calc_iq__vdsc_dn7 * var_fn25_calc_iq__vdsat) - (var_fn25_calc_iq__vdsc * var_fn25_calc_iq__vdsat_dn7)) / (var_fn25_calc_iq__vdsat * var_fn25_calc_iq__vdsat)), (((var_fn25_calc_iq__vdsc_dn16 * var_fn25_calc_iq__vdsat) - (var_fn25_calc_iq__vdsc * var_fn25_calc_iq__vdsat_dn16)) / (var_fn25_calc_iq__vdsat * var_fn25_calc_iq__vdsat)), (((var_fn25_calc_iq__vdsc_dn17 * var_fn25_calc_iq__vdsat) - (var_fn25_calc_iq__vdsc * var_fn25_calc_iq__vdsat_dn17)) / (var_fn25_calc_iq__vdsat * var_fn25_calc_iq__vdsat)),)
    } else {
        (var_fn25_calc_iq__myarg, var_fn25_calc_iq__myarg_dn2, var_fn25_calc_iq__myarg_dn3, var_fn25_calc_iq__myarg_dn4, var_fn25_calc_iq__myarg_dn7, var_fn25_calc_iq__myarg_dn16, var_fn25_calc_iq__myarg_dn17,)
    }
};
        var_fn25_calc_iq__myarg = assign3500_e5156;
        var_fn25_calc_iq__myarg_dn2 = assign3500_e5156_d_n2;
        var_fn25_calc_iq__myarg_dn3 = assign3500_e5156_d_n3;
        var_fn25_calc_iq__myarg_dn4 = assign3500_e5156_d_n4;
        var_fn25_calc_iq__myarg_dn7 = assign3500_e5156_d_n7;
        var_fn25_calc_iq__myarg_dn16 = assign3500_e5156_d_n16;
        var_fn25_calc_iq__myarg_dn17 = assign3500_e5156_d_n17;
        var_fn25_calc_iq__myarg_rv = 0.0;

        let (assign3540_e5225, assign3540_e5225_d_n4,) = {
    if (var_guard24 != 0.0) {
        let assign3540_e5222: f64 = (2.302585092994046 * var_fn25_calc_iq__phitin);
        let assign3540_e5223: f64 = (var_fn25_calc_iq__ss / assign3540_e5222);
        (assign3540_e5223, (-((var_fn25_calc_iq__ss * (2.302585092994046 * var_fn25_calc_iq__phitin_dn4)) / (assign3540_e5222 * assign3540_e5222))),)
    } else {
        (var_fn25_calc_iq__n0, var_fn25_calc_iq__n0_dn4,)
    }
};
        var_fn25_calc_iq__n0 = assign3540_e5225;
        var_fn25_calc_iq__n0_dn4 = assign3540_e5225_d_n4;
        var_fn25_calc_iq__n0_rv = 0.0;

        let (assign3550_e5233, assign3550_e5233_d_n4,) = {
    if (var_guard24 != 0.0) {
        let assign3550_e5229: f64 = (2.0 * var_fn25_calc_iq__n0);
        let assign3550_e5231: f64 = (assign3550_e5229 * var_fn25_calc_iq__phitin);
        (assign3550_e5231, (((2.0 * var_fn25_calc_iq__n0_dn4) * var_fn25_calc_iq__phitin) + (assign3550_e5229 * var_fn25_calc_iq__phitin_dn4)),)
    } else {
        (var_fn25_calc_iq__two_n_phit0, var_fn25_calc_iq__two_n_phit0_dn4,)
    }
};
        var_fn25_calc_iq__two_n_phit0 = assign3550_e5233;
        var_fn25_calc_iq__two_n_phit0_dn4 = assign3550_e5233_d_n4;
        var_fn25_calc_iq__two_n_phit0_rv = 0.0;

        let (assign3560_e5239, assign3560_e5239_d_n4,) = {
    if (var_guard24 != 0.0) {
        let assign3560_e5237: f64 = (var_fn25_calc_iq__cgin * var_fn25_calc_iq__two_n_phit0);
        (assign3560_e5237, ((var_fn25_calc_iq__cgin_dn4 * var_fn25_calc_iq__two_n_phit0) + (var_fn25_calc_iq__cgin * var_fn25_calc_iq__two_n_phit0_dn4)),)
    } else {
        (var_fn25_calc_iq__qref0, var_fn25_calc_iq__qref0_dn4,)
    }
};
        var_fn25_calc_iq__qref0 = assign3560_e5239;
        var_fn25_calc_iq__qref0_dn4 = assign3560_e5239_d_n4;
        var_fn25_calc_iq__qref0_rv = 0.0;

        let (assign3570_e5249, assign3570_e5249_d_n4,) = {
    if (var_guard24 != 0.0) {
        let assign3570_e5244: f64 = (p.p51 * var_fn25_calc_iq__alpha_phit);
        let assign3570_e5246: f64 = (assign3570_e5244 / 2.0);
        let assign3570_e5247: f64 = (var_fn25_calc_iq__vtof - assign3570_e5246);
        (assign3570_e5247, (var_fn25_calc_iq__vtof_dn4 - ((p.p51 * var_fn25_calc_iq__alpha_phit_dn4) / 2.0)),)
    } else {
        (var_fn25_calc_iq__myarg0, var_fn25_calc_iq__myarg0_dn4,)
    }
};
        var_fn25_calc_iq__myarg0 = assign3570_e5249;
        var_fn25_calc_iq__myarg0_dn4 = assign3570_e5249_d_n4;
        var_fn25_calc_iq__myarg0_rv = 0.0;

        *var_fn25_calc_iq__etad_slot = var_fn25_calc_iq__etad;
        *var_fn25_calc_iq__etad_dn16_slot = var_fn25_calc_iq__etad_dn16;
        *var_fn25_calc_iq__etad_dn17_slot = var_fn25_calc_iq__etad_dn17;
        *var_fn25_calc_iq__etad_dn2_slot = var_fn25_calc_iq__etad_dn2;
        *var_fn25_calc_iq__etad_dn3_slot = var_fn25_calc_iq__etad_dn3;
        *var_fn25_calc_iq__etad_dn4_slot = var_fn25_calc_iq__etad_dn4;
        *var_fn25_calc_iq__etad_dn7_slot = var_fn25_calc_iq__etad_dn7;
        *var_fn25_calc_iq__etad_rv_slot = var_fn25_calc_iq__etad_rv;
        *var_fn25_calc_iq__etas_slot = var_fn25_calc_iq__etas;
        *var_fn25_calc_iq__etas_dn16_slot = var_fn25_calc_iq__etas_dn16;
        *var_fn25_calc_iq__etas_dn17_slot = var_fn25_calc_iq__etas_dn17;
        *var_fn25_calc_iq__etas_dn2_slot = var_fn25_calc_iq__etas_dn2;
        *var_fn25_calc_iq__etas_dn3_slot = var_fn25_calc_iq__etas_dn3;
        *var_fn25_calc_iq__etas_dn4_slot = var_fn25_calc_iq__etas_dn4;
        *var_fn25_calc_iq__etas_dn7_slot = var_fn25_calc_iq__etas_dn7;
        *var_fn25_calc_iq__etas_rv_slot = var_fn25_calc_iq__etas_rv;
        *var_fn25_calc_iq__exparg_slot = var_fn25_calc_iq__exparg;
        *var_fn25_calc_iq__exparg_dn16_slot = var_fn25_calc_iq__exparg_dn16;
        *var_fn25_calc_iq__exparg_dn17_slot = var_fn25_calc_iq__exparg_dn17;
        *var_fn25_calc_iq__exparg_dn2_slot = var_fn25_calc_iq__exparg_dn2;
        *var_fn25_calc_iq__exparg_dn3_slot = var_fn25_calc_iq__exparg_dn3;
        *var_fn25_calc_iq__exparg_dn4_slot = var_fn25_calc_iq__exparg_dn4;
        *var_fn25_calc_iq__exparg_dn7_slot = var_fn25_calc_iq__exparg_dn7;
        *var_fn25_calc_iq__exparg_rv_slot = var_fn25_calc_iq__exparg_rv;
        *var_fn25_calc_iq__fds_slot = var_fn25_calc_iq__fds;
        *var_fn25_calc_iq__fds_dn16_slot = var_fn25_calc_iq__fds_dn16;
        *var_fn25_calc_iq__fds_dn17_slot = var_fn25_calc_iq__fds_dn17;
        *var_fn25_calc_iq__fds_dn2_slot = var_fn25_calc_iq__fds_dn2;
        *var_fn25_calc_iq__fds_dn3_slot = var_fn25_calc_iq__fds_dn3;
        *var_fn25_calc_iq__fds_dn4_slot = var_fn25_calc_iq__fds_dn4;
        *var_fn25_calc_iq__fds_dn7_slot = var_fn25_calc_iq__fds_dn7;
        *var_fn25_calc_iq__fds_rv_slot = var_fn25_calc_iq__fds_rv;
        *var_fn25_calc_iq__ffd_slot = var_fn25_calc_iq__ffd;
        *var_fn25_calc_iq__ffd_dn16_slot = var_fn25_calc_iq__ffd_dn16;
        *var_fn25_calc_iq__ffd_dn17_slot = var_fn25_calc_iq__ffd_dn17;
        *var_fn25_calc_iq__ffd_dn2_slot = var_fn25_calc_iq__ffd_dn2;
        *var_fn25_calc_iq__ffd_dn3_slot = var_fn25_calc_iq__ffd_dn3;
        *var_fn25_calc_iq__ffd_dn4_slot = var_fn25_calc_iq__ffd_dn4;
        *var_fn25_calc_iq__ffd_dn7_slot = var_fn25_calc_iq__ffd_dn7;
        *var_fn25_calc_iq__ffd_rv_slot = var_fn25_calc_iq__ffd_rv;
        *var_fn25_calc_iq__ffs_slot = var_fn25_calc_iq__ffs;
        *var_fn25_calc_iq__ffs_dn16_slot = var_fn25_calc_iq__ffs_dn16;
        *var_fn25_calc_iq__ffs_dn17_slot = var_fn25_calc_iq__ffs_dn17;
        *var_fn25_calc_iq__ffs_dn2_slot = var_fn25_calc_iq__ffs_dn2;
        *var_fn25_calc_iq__ffs_dn3_slot = var_fn25_calc_iq__ffs_dn3;
        *var_fn25_calc_iq__ffs_dn4_slot = var_fn25_calc_iq__ffs_dn4;
        *var_fn25_calc_iq__ffs_dn7_slot = var_fn25_calc_iq__ffs_dn7;
        *var_fn25_calc_iq__ffs_rv_slot = var_fn25_calc_iq__ffs_rv;
        *var_fn25_calc_iq__myarg_slot = var_fn25_calc_iq__myarg;
        *var_fn25_calc_iq__myarg0_slot = var_fn25_calc_iq__myarg0;
        *var_fn25_calc_iq__myarg0_dn4_slot = var_fn25_calc_iq__myarg0_dn4;
        *var_fn25_calc_iq__myarg0_rv_slot = var_fn25_calc_iq__myarg0_rv;
        *var_fn25_calc_iq__myarg_dn16_slot = var_fn25_calc_iq__myarg_dn16;
        *var_fn25_calc_iq__myarg_dn17_slot = var_fn25_calc_iq__myarg_dn17;
        *var_fn25_calc_iq__myarg_dn2_slot = var_fn25_calc_iq__myarg_dn2;
        *var_fn25_calc_iq__myarg_dn3_slot = var_fn25_calc_iq__myarg_dn3;
        *var_fn25_calc_iq__myarg_dn4_slot = var_fn25_calc_iq__myarg_dn4;
        *var_fn25_calc_iq__myarg_dn7_slot = var_fn25_calc_iq__myarg_dn7;
        *var_fn25_calc_iq__myarg_rv_slot = var_fn25_calc_iq__myarg_rv;
        *var_fn25_calc_iq__n0_slot = var_fn25_calc_iq__n0;
        *var_fn25_calc_iq__n0_dn4_slot = var_fn25_calc_iq__n0_dn4;
        *var_fn25_calc_iq__n0_rv_slot = var_fn25_calc_iq__n0_rv;
        *var_fn25_calc_iq__qinvd_slot = var_fn25_calc_iq__qinvd;
        *var_fn25_calc_iq__qinvd_dn16_slot = var_fn25_calc_iq__qinvd_dn16;
        *var_fn25_calc_iq__qinvd_dn17_slot = var_fn25_calc_iq__qinvd_dn17;
        *var_fn25_calc_iq__qinvd_dn2_slot = var_fn25_calc_iq__qinvd_dn2;
        *var_fn25_calc_iq__qinvd_dn3_slot = var_fn25_calc_iq__qinvd_dn3;
        *var_fn25_calc_iq__qinvd_dn4_slot = var_fn25_calc_iq__qinvd_dn4;
        *var_fn25_calc_iq__qinvd_dn7_slot = var_fn25_calc_iq__qinvd_dn7;
        *var_fn25_calc_iq__qinvd_rv_slot = var_fn25_calc_iq__qinvd_rv;
        *var_fn25_calc_iq__qinvs_slot = var_fn25_calc_iq__qinvs;
        *var_fn25_calc_iq__qinvs_dn16_slot = var_fn25_calc_iq__qinvs_dn16;
        *var_fn25_calc_iq__qinvs_dn17_slot = var_fn25_calc_iq__qinvs_dn17;
        *var_fn25_calc_iq__qinvs_dn2_slot = var_fn25_calc_iq__qinvs_dn2;
        *var_fn25_calc_iq__qinvs_dn3_slot = var_fn25_calc_iq__qinvs_dn3;
        *var_fn25_calc_iq__qinvs_dn4_slot = var_fn25_calc_iq__qinvs_dn4;
        *var_fn25_calc_iq__qinvs_dn7_slot = var_fn25_calc_iq__qinvs_dn7;
        *var_fn25_calc_iq__qinvs_rv_slot = var_fn25_calc_iq__qinvs_rv;
        *var_fn25_calc_iq__qref0_slot = var_fn25_calc_iq__qref0;
        *var_fn25_calc_iq__qref0_dn4_slot = var_fn25_calc_iq__qref0_dn4;
        *var_fn25_calc_iq__qref0_rv_slot = var_fn25_calc_iq__qref0_rv;
        *var_fn25_calc_iq__two_n_phit0_slot = var_fn25_calc_iq__two_n_phit0;
        *var_fn25_calc_iq__two_n_phit0_dn4_slot = var_fn25_calc_iq__two_n_phit0_dn4;
        *var_fn25_calc_iq__two_n_phit0_rv_slot = var_fn25_calc_iq__two_n_phit0_rv;
        *var_fn25_calc_iq__vdsc_slot = var_fn25_calc_iq__vdsc;
        *var_fn25_calc_iq__vdsc_dn16_slot = var_fn25_calc_iq__vdsc_dn16;
        *var_fn25_calc_iq__vdsc_dn17_slot = var_fn25_calc_iq__vdsc_dn17;
        *var_fn25_calc_iq__vdsc_dn2_slot = var_fn25_calc_iq__vdsc_dn2;
        *var_fn25_calc_iq__vdsc_dn3_slot = var_fn25_calc_iq__vdsc_dn3;
        *var_fn25_calc_iq__vdsc_dn4_slot = var_fn25_calc_iq__vdsc_dn4;
        *var_fn25_calc_iq__vdsc_dn7_slot = var_fn25_calc_iq__vdsc_dn7;
        *var_fn25_calc_iq__vdsc_rv_slot = var_fn25_calc_iq__vdsc_rv;
        *var_fn25_calc_iq__vsx_slot = var_fn25_calc_iq__vsx;
        *var_fn25_calc_iq__vsx_dn16_slot = var_fn25_calc_iq__vsx_dn16;
        *var_fn25_calc_iq__vsx_dn17_slot = var_fn25_calc_iq__vsx_dn17;
        *var_fn25_calc_iq__vsx_dn2_slot = var_fn25_calc_iq__vsx_dn2;
        *var_fn25_calc_iq__vsx_dn3_slot = var_fn25_calc_iq__vsx_dn3;
        *var_fn25_calc_iq__vsx_dn4_slot = var_fn25_calc_iq__vsx_dn4;
        *var_fn25_calc_iq__vsx_dn7_slot = var_fn25_calc_iq__vsx_dn7;
        *var_fn25_calc_iq__vsx_rv_slot = var_fn25_calc_iq__vsx_rv;
        *var_guard31_slot = var_guard31;
        *var_guard31_rv_slot = var_guard31_rv;
        *var_guard32_slot = var_guard32;
        *var_guard32_rv_slot = var_guard32_rv;
        *var_guard33_slot = var_guard33;
        *var_guard33_rv_slot = var_guard33_rv;
        *var_guard34_slot = var_guard34;
        *var_guard34_rv_slot = var_guard34_rv;
        *var_guard35_slot = var_guard35;
        *var_guard35_rv_slot = var_guard35_rv;
        *var_guard36_slot = var_guard36;
        *var_guard36_rv_slot = var_guard36_rv;
        *var_guard37_slot = var_guard37;
        *var_guard37_rv_slot = var_guard37_rv;
        *var_guard38_slot = var_guard38;
        *var_guard38_rv_slot = var_guard38_rv;
    }

    pub(super) fn stamp_reactive_block_8(
        p: &Parameters,
        var_fn25_calc_iq__alpha_phit: f64,
        var_fn25_calc_iq__alpha_phit_dn4: f64,
        var_fn25_calc_iq__beta: f64,
        var_fn25_calc_iq__cgin: f64,
        var_fn25_calc_iq__cgin_dn4: f64,
        var_fn25_calc_iq__lin: f64,
        var_fn25_calc_iq__mu0: f64,
        var_fn25_calc_iq__myarg0: f64,
        var_fn25_calc_iq__myarg0_dn4: f64,
        var_fn25_calc_iq__qref0: f64,
        var_fn25_calc_iq__qref0_dn4: f64,
        var_fn25_calc_iq__tambin: f64,
        var_fn25_calc_iq__tambin_dn4: f64,
        var_fn25_calc_iq__tfacmobin: f64,
        var_fn25_calc_iq__tfacmobin_dn4: f64,
        var_fn25_calc_iq__tnomin: f64,
        var_fn25_calc_iq__two_n_phit0: f64,
        var_fn25_calc_iq__two_n_phit0_dn4: f64,
        var_fn25_calc_iq__vdsin: f64,
        var_fn25_calc_iq__vdsin_dn16: f64,
        var_fn25_calc_iq__vdsin_dn17: f64,
        var_fn25_calc_iq__vel0: f64,
        var_fn25_calc_iq__vgdin: f64,
        var_fn25_calc_iq__vgdin_dn16: f64,
        var_fn25_calc_iq__vgdin_dn17: f64,
        var_fn25_calc_iq__vgdin_dn2: f64,
        var_fn25_calc_iq__vgdin_dn7: f64,
        var_fn25_calc_iq__vgsin: f64,
        var_fn25_calc_iq__vgsin_dn16: f64,
        var_fn25_calc_iq__vgsin_dn2: f64,
        var_fn25_calc_iq__vgsin_dn7: f64,
        var_fn25_calc_iq__vtof: f64,
        var_fn25_calc_iq__vtof_dn4: f64,
        var_fn25_calc_iq__vzeta: f64,
        var_guard24: f64,
        var_fn25_calc_iq__eta0_slot: &mut f64,
        var_fn25_calc_iq__eta0_dn16_slot: &mut f64,
        var_fn25_calc_iq__eta0_dn17_slot: &mut f64,
        var_fn25_calc_iq__eta0_dn2_slot: &mut f64,
        var_fn25_calc_iq__eta0_dn4_slot: &mut f64,
        var_fn25_calc_iq__eta0_dn7_slot: &mut f64,
        var_fn25_calc_iq__eta0_rv_slot: &mut f64,
        var_fn25_calc_iq__exparg0_slot: &mut f64,
        var_fn25_calc_iq__exparg0_dn16_slot: &mut f64,
        var_fn25_calc_iq__exparg0_dn17_slot: &mut f64,
        var_fn25_calc_iq__exparg0_dn2_slot: &mut f64,
        var_fn25_calc_iq__exparg0_dn4_slot: &mut f64,
        var_fn25_calc_iq__exparg0_dn7_slot: &mut f64,
        var_fn25_calc_iq__exparg0_rv_slot: &mut f64,
        var_fn25_calc_iq__fds0_slot: &mut f64,
        var_fn25_calc_iq__fds0_dn16_slot: &mut f64,
        var_fn25_calc_iq__fds0_dn17_slot: &mut f64,
        var_fn25_calc_iq__fds0_dn2_slot: &mut f64,
        var_fn25_calc_iq__fds0_dn4_slot: &mut f64,
        var_fn25_calc_iq__fds0_dn7_slot: &mut f64,
        var_fn25_calc_iq__fds0_rv_slot: &mut f64,
        var_fn25_calc_iq__ff0_slot: &mut f64,
        var_fn25_calc_iq__ff0_dn16_slot: &mut f64,
        var_fn25_calc_iq__ff0_dn17_slot: &mut f64,
        var_fn25_calc_iq__ff0_dn2_slot: &mut f64,
        var_fn25_calc_iq__ff0_dn4_slot: &mut f64,
        var_fn25_calc_iq__ff0_dn7_slot: &mut f64,
        var_fn25_calc_iq__ff0_rv_slot: &mut f64,
        var_fn25_calc_iq__ffs0_slot: &mut f64,
        var_fn25_calc_iq__ffs0_dn16_slot: &mut f64,
        var_fn25_calc_iq__ffs0_dn17_slot: &mut f64,
        var_fn25_calc_iq__ffs0_dn2_slot: &mut f64,
        var_fn25_calc_iq__ffs0_dn4_slot: &mut f64,
        var_fn25_calc_iq__ffs0_dn7_slot: &mut f64,
        var_fn25_calc_iq__ffs0_rv_slot: &mut f64,
        var_fn25_calc_iq__fsd0_slot: &mut f64,
        var_fn25_calc_iq__fsd0_dn16_slot: &mut f64,
        var_fn25_calc_iq__fsd0_dn17_slot: &mut f64,
        var_fn25_calc_iq__fsd0_dn2_slot: &mut f64,
        var_fn25_calc_iq__fsd0_dn4_slot: &mut f64,
        var_fn25_calc_iq__fsd0_dn7_slot: &mut f64,
        var_fn25_calc_iq__fsd0_rv_slot: &mut f64,
        var_fn25_calc_iq__muf0_slot: &mut f64,
        var_fn25_calc_iq__muf0_dn4_slot: &mut f64,
        var_fn25_calc_iq__muf0_rv_slot: &mut f64,
        var_fn25_calc_iq__qinvv0_slot: &mut f64,
        var_fn25_calc_iq__qinvv0_dn16_slot: &mut f64,
        var_fn25_calc_iq__qinvv0_dn17_slot: &mut f64,
        var_fn25_calc_iq__qinvv0_dn2_slot: &mut f64,
        var_fn25_calc_iq__qinvv0_dn4_slot: &mut f64,
        var_fn25_calc_iq__qinvv0_dn7_slot: &mut f64,
        var_fn25_calc_iq__qinvv0_rv_slot: &mut f64,
        var_fn25_calc_iq__vdsat10_slot: &mut f64,
        var_fn25_calc_iq__vdsat10_dn16_slot: &mut f64,
        var_fn25_calc_iq__vdsat10_dn17_slot: &mut f64,
        var_fn25_calc_iq__vdsat10_dn2_slot: &mut f64,
        var_fn25_calc_iq__vdsat10_dn4_slot: &mut f64,
        var_fn25_calc_iq__vdsat10_dn7_slot: &mut f64,
        var_fn25_calc_iq__vdsat10_rv_slot: &mut f64,
        var_fn25_calc_iq__vdsats0_slot: &mut f64,
        var_fn25_calc_iq__vdsats0_dn4_slot: &mut f64,
        var_fn25_calc_iq__vdsats0_rv_slot: &mut f64,
        var_fn25_calc_iq__vdsats10_slot: &mut f64,
        var_fn25_calc_iq__vdsats10_dn16_slot: &mut f64,
        var_fn25_calc_iq__vdsats10_dn17_slot: &mut f64,
        var_fn25_calc_iq__vdsats10_dn2_slot: &mut f64,
        var_fn25_calc_iq__vdsats10_dn4_slot: &mut f64,
        var_fn25_calc_iq__vdsats10_dn7_slot: &mut f64,
        var_fn25_calc_iq__vdsats10_rv_slot: &mut f64,
        var_fn25_calc_iq__vdx0_slot: &mut f64,
        var_fn25_calc_iq__vdx0_dn16_slot: &mut f64,
        var_fn25_calc_iq__vdx0_dn17_slot: &mut f64,
        var_fn25_calc_iq__vdx0_dn2_slot: &mut f64,
        var_fn25_calc_iq__vdx0_dn4_slot: &mut f64,
        var_fn25_calc_iq__vdx0_dn7_slot: &mut f64,
        var_fn25_calc_iq__vdx0_rv_slot: &mut f64,
        var_fn25_calc_iq__vsx0_slot: &mut f64,
        var_fn25_calc_iq__vsx0_dn16_slot: &mut f64,
        var_fn25_calc_iq__vsx0_dn17_slot: &mut f64,
        var_fn25_calc_iq__vsx0_dn2_slot: &mut f64,
        var_fn25_calc_iq__vsx0_dn4_slot: &mut f64,
        var_fn25_calc_iq__vsx0_dn7_slot: &mut f64,
        var_fn25_calc_iq__vsx0_rv_slot: &mut f64,
        var_fn25_calc_iq__vx0_slot: &mut f64,
        var_fn25_calc_iq__vx0_dn4_slot: &mut f64,
        var_fn25_calc_iq__vx0_rv_slot: &mut f64,
        var_guard39_slot: &mut f64,
        var_guard39_rv_slot: &mut f64,
        var_guard40_slot: &mut f64,
        var_guard40_rv_slot: &mut f64,
        var_guard41_slot: &mut f64,
        var_guard41_rv_slot: &mut f64,
        var_guard42_slot: &mut f64,
        var_guard42_rv_slot: &mut f64,
        var_guard43_slot: &mut f64,
        var_guard43_rv_slot: &mut f64,
        var_guard44_slot: &mut f64,
        var_guard44_rv_slot: &mut f64,
    ) {
        let mut var_fn25_calc_iq__eta0: f64 = *var_fn25_calc_iq__eta0_slot;
        let mut var_fn25_calc_iq__eta0_dn16: f64 = *var_fn25_calc_iq__eta0_dn16_slot;
        let mut var_fn25_calc_iq__eta0_dn17: f64 = *var_fn25_calc_iq__eta0_dn17_slot;
        let mut var_fn25_calc_iq__eta0_dn2: f64 = *var_fn25_calc_iq__eta0_dn2_slot;
        let mut var_fn25_calc_iq__eta0_dn4: f64 = *var_fn25_calc_iq__eta0_dn4_slot;
        let mut var_fn25_calc_iq__eta0_dn7: f64 = *var_fn25_calc_iq__eta0_dn7_slot;
        let mut var_fn25_calc_iq__eta0_rv: f64 = *var_fn25_calc_iq__eta0_rv_slot;
        let mut var_fn25_calc_iq__exparg0: f64 = *var_fn25_calc_iq__exparg0_slot;
        let mut var_fn25_calc_iq__exparg0_dn16: f64 = *var_fn25_calc_iq__exparg0_dn16_slot;
        let mut var_fn25_calc_iq__exparg0_dn17: f64 = *var_fn25_calc_iq__exparg0_dn17_slot;
        let mut var_fn25_calc_iq__exparg0_dn2: f64 = *var_fn25_calc_iq__exparg0_dn2_slot;
        let mut var_fn25_calc_iq__exparg0_dn4: f64 = *var_fn25_calc_iq__exparg0_dn4_slot;
        let mut var_fn25_calc_iq__exparg0_dn7: f64 = *var_fn25_calc_iq__exparg0_dn7_slot;
        let mut var_fn25_calc_iq__exparg0_rv: f64 = *var_fn25_calc_iq__exparg0_rv_slot;
        let mut var_fn25_calc_iq__fds0: f64 = *var_fn25_calc_iq__fds0_slot;
        let mut var_fn25_calc_iq__fds0_dn16: f64 = *var_fn25_calc_iq__fds0_dn16_slot;
        let mut var_fn25_calc_iq__fds0_dn17: f64 = *var_fn25_calc_iq__fds0_dn17_slot;
        let mut var_fn25_calc_iq__fds0_dn2: f64 = *var_fn25_calc_iq__fds0_dn2_slot;
        let mut var_fn25_calc_iq__fds0_dn4: f64 = *var_fn25_calc_iq__fds0_dn4_slot;
        let mut var_fn25_calc_iq__fds0_dn7: f64 = *var_fn25_calc_iq__fds0_dn7_slot;
        let mut var_fn25_calc_iq__fds0_rv: f64 = *var_fn25_calc_iq__fds0_rv_slot;
        let mut var_fn25_calc_iq__ff0: f64 = *var_fn25_calc_iq__ff0_slot;
        let mut var_fn25_calc_iq__ff0_dn16: f64 = *var_fn25_calc_iq__ff0_dn16_slot;
        let mut var_fn25_calc_iq__ff0_dn17: f64 = *var_fn25_calc_iq__ff0_dn17_slot;
        let mut var_fn25_calc_iq__ff0_dn2: f64 = *var_fn25_calc_iq__ff0_dn2_slot;
        let mut var_fn25_calc_iq__ff0_dn4: f64 = *var_fn25_calc_iq__ff0_dn4_slot;
        let mut var_fn25_calc_iq__ff0_dn7: f64 = *var_fn25_calc_iq__ff0_dn7_slot;
        let mut var_fn25_calc_iq__ff0_rv: f64 = *var_fn25_calc_iq__ff0_rv_slot;
        let mut var_fn25_calc_iq__ffs0: f64 = *var_fn25_calc_iq__ffs0_slot;
        let mut var_fn25_calc_iq__ffs0_dn16: f64 = *var_fn25_calc_iq__ffs0_dn16_slot;
        let mut var_fn25_calc_iq__ffs0_dn17: f64 = *var_fn25_calc_iq__ffs0_dn17_slot;
        let mut var_fn25_calc_iq__ffs0_dn2: f64 = *var_fn25_calc_iq__ffs0_dn2_slot;
        let mut var_fn25_calc_iq__ffs0_dn4: f64 = *var_fn25_calc_iq__ffs0_dn4_slot;
        let mut var_fn25_calc_iq__ffs0_dn7: f64 = *var_fn25_calc_iq__ffs0_dn7_slot;
        let mut var_fn25_calc_iq__ffs0_rv: f64 = *var_fn25_calc_iq__ffs0_rv_slot;
        let mut var_fn25_calc_iq__fsd0: f64 = *var_fn25_calc_iq__fsd0_slot;
        let mut var_fn25_calc_iq__fsd0_dn16: f64 = *var_fn25_calc_iq__fsd0_dn16_slot;
        let mut var_fn25_calc_iq__fsd0_dn17: f64 = *var_fn25_calc_iq__fsd0_dn17_slot;
        let mut var_fn25_calc_iq__fsd0_dn2: f64 = *var_fn25_calc_iq__fsd0_dn2_slot;
        let mut var_fn25_calc_iq__fsd0_dn4: f64 = *var_fn25_calc_iq__fsd0_dn4_slot;
        let mut var_fn25_calc_iq__fsd0_dn7: f64 = *var_fn25_calc_iq__fsd0_dn7_slot;
        let mut var_fn25_calc_iq__fsd0_rv: f64 = *var_fn25_calc_iq__fsd0_rv_slot;
        let mut var_fn25_calc_iq__muf0: f64 = *var_fn25_calc_iq__muf0_slot;
        let mut var_fn25_calc_iq__muf0_dn4: f64 = *var_fn25_calc_iq__muf0_dn4_slot;
        let mut var_fn25_calc_iq__muf0_rv: f64 = *var_fn25_calc_iq__muf0_rv_slot;
        let mut var_fn25_calc_iq__qinvv0: f64 = *var_fn25_calc_iq__qinvv0_slot;
        let mut var_fn25_calc_iq__qinvv0_dn16: f64 = *var_fn25_calc_iq__qinvv0_dn16_slot;
        let mut var_fn25_calc_iq__qinvv0_dn17: f64 = *var_fn25_calc_iq__qinvv0_dn17_slot;
        let mut var_fn25_calc_iq__qinvv0_dn2: f64 = *var_fn25_calc_iq__qinvv0_dn2_slot;
        let mut var_fn25_calc_iq__qinvv0_dn4: f64 = *var_fn25_calc_iq__qinvv0_dn4_slot;
        let mut var_fn25_calc_iq__qinvv0_dn7: f64 = *var_fn25_calc_iq__qinvv0_dn7_slot;
        let mut var_fn25_calc_iq__qinvv0_rv: f64 = *var_fn25_calc_iq__qinvv0_rv_slot;
        let mut var_fn25_calc_iq__vdsat10: f64 = *var_fn25_calc_iq__vdsat10_slot;
        let mut var_fn25_calc_iq__vdsat10_dn16: f64 = *var_fn25_calc_iq__vdsat10_dn16_slot;
        let mut var_fn25_calc_iq__vdsat10_dn17: f64 = *var_fn25_calc_iq__vdsat10_dn17_slot;
        let mut var_fn25_calc_iq__vdsat10_dn2: f64 = *var_fn25_calc_iq__vdsat10_dn2_slot;
        let mut var_fn25_calc_iq__vdsat10_dn4: f64 = *var_fn25_calc_iq__vdsat10_dn4_slot;
        let mut var_fn25_calc_iq__vdsat10_dn7: f64 = *var_fn25_calc_iq__vdsat10_dn7_slot;
        let mut var_fn25_calc_iq__vdsat10_rv: f64 = *var_fn25_calc_iq__vdsat10_rv_slot;
        let mut var_fn25_calc_iq__vdsats0: f64 = *var_fn25_calc_iq__vdsats0_slot;
        let mut var_fn25_calc_iq__vdsats0_dn4: f64 = *var_fn25_calc_iq__vdsats0_dn4_slot;
        let mut var_fn25_calc_iq__vdsats0_rv: f64 = *var_fn25_calc_iq__vdsats0_rv_slot;
        let mut var_fn25_calc_iq__vdsats10: f64 = *var_fn25_calc_iq__vdsats10_slot;
        let mut var_fn25_calc_iq__vdsats10_dn16: f64 = *var_fn25_calc_iq__vdsats10_dn16_slot;
        let mut var_fn25_calc_iq__vdsats10_dn17: f64 = *var_fn25_calc_iq__vdsats10_dn17_slot;
        let mut var_fn25_calc_iq__vdsats10_dn2: f64 = *var_fn25_calc_iq__vdsats10_dn2_slot;
        let mut var_fn25_calc_iq__vdsats10_dn4: f64 = *var_fn25_calc_iq__vdsats10_dn4_slot;
        let mut var_fn25_calc_iq__vdsats10_dn7: f64 = *var_fn25_calc_iq__vdsats10_dn7_slot;
        let mut var_fn25_calc_iq__vdsats10_rv: f64 = *var_fn25_calc_iq__vdsats10_rv_slot;
        let mut var_fn25_calc_iq__vdx0: f64 = *var_fn25_calc_iq__vdx0_slot;
        let mut var_fn25_calc_iq__vdx0_dn16: f64 = *var_fn25_calc_iq__vdx0_dn16_slot;
        let mut var_fn25_calc_iq__vdx0_dn17: f64 = *var_fn25_calc_iq__vdx0_dn17_slot;
        let mut var_fn25_calc_iq__vdx0_dn2: f64 = *var_fn25_calc_iq__vdx0_dn2_slot;
        let mut var_fn25_calc_iq__vdx0_dn4: f64 = *var_fn25_calc_iq__vdx0_dn4_slot;
        let mut var_fn25_calc_iq__vdx0_dn7: f64 = *var_fn25_calc_iq__vdx0_dn7_slot;
        let mut var_fn25_calc_iq__vdx0_rv: f64 = *var_fn25_calc_iq__vdx0_rv_slot;
        let mut var_fn25_calc_iq__vsx0: f64 = *var_fn25_calc_iq__vsx0_slot;
        let mut var_fn25_calc_iq__vsx0_dn16: f64 = *var_fn25_calc_iq__vsx0_dn16_slot;
        let mut var_fn25_calc_iq__vsx0_dn17: f64 = *var_fn25_calc_iq__vsx0_dn17_slot;
        let mut var_fn25_calc_iq__vsx0_dn2: f64 = *var_fn25_calc_iq__vsx0_dn2_slot;
        let mut var_fn25_calc_iq__vsx0_dn4: f64 = *var_fn25_calc_iq__vsx0_dn4_slot;
        let mut var_fn25_calc_iq__vsx0_dn7: f64 = *var_fn25_calc_iq__vsx0_dn7_slot;
        let mut var_fn25_calc_iq__vsx0_rv: f64 = *var_fn25_calc_iq__vsx0_rv_slot;
        let mut var_fn25_calc_iq__vx0: f64 = *var_fn25_calc_iq__vx0_slot;
        let mut var_fn25_calc_iq__vx0_dn4: f64 = *var_fn25_calc_iq__vx0_dn4_slot;
        let mut var_fn25_calc_iq__vx0_rv: f64 = *var_fn25_calc_iq__vx0_rv_slot;
        let mut var_guard39: f64 = *var_guard39_slot;
        let mut var_guard39_rv: f64 = *var_guard39_rv_slot;
        let mut var_guard40: f64 = *var_guard40_slot;
        let mut var_guard40_rv: f64 = *var_guard40_rv_slot;
        let mut var_guard41: f64 = *var_guard41_slot;
        let mut var_guard41_rv: f64 = *var_guard41_rv_slot;
        let mut var_guard42: f64 = *var_guard42_slot;
        let mut var_guard42_rv: f64 = *var_guard42_rv_slot;
        let mut var_guard43: f64 = *var_guard43_slot;
        let mut var_guard43_rv: f64 = *var_guard43_rv_slot;
        let mut var_guard44: f64 = *var_guard44_slot;
        let mut var_guard44_rv: f64 = *var_guard44_rv_slot;

        let (assign3580_e5300, assign3580_e5300_d_n2, assign3580_e5300_d_n4, assign3580_e5300_d_n7, assign3580_e5300_d_n16, assign3580_e5300_d_n17,) = {
    if (var_guard24 != 0.0) {
        let (assign3580_e5294, assign3580_e5294_d_n2, assign3580_e5294_d_n7, assign3580_e5294_d_n16, assign3580_e5294_d_n17,) = {
            if (p.p52 != 0.0) {
                let assign3580_e5258: f64 = (var_fn25_calc_iq__vgsin + var_fn25_calc_iq__vgdin);
                let assign3580_e5261: f64 = (var_fn25_calc_iq__vgsin - var_fn25_calc_iq__vgdin);
                let assign3580_e5264: f64 = (0.001 / p.p53);
                let assign3580_e5267: f64 = (var_fn25_calc_iq__vgsin - var_fn25_calc_iq__vgdin);
                let assign3580_e5268: f64 = (assign3580_e5264 * assign3580_e5267);
                let assign3580_e5269: f64 = (assign3580_e5268).tanh();
                let assign3580_e5270: f64 = (assign3580_e5261 * assign3580_e5269);
                let assign3580_e5271: f64 = (assign3580_e5258 + assign3580_e5270);
                let assign3580_e5272: f64 = (0.5 * assign3580_e5271);
                (assign3580_e5272, (0.5 * ((var_fn25_calc_iq__vgsin_dn2 + var_fn25_calc_iq__vgdin_dn2) + (((var_fn25_calc_iq__vgsin_dn2 - var_fn25_calc_iq__vgdin_dn2) * assign3580_e5269) + (assign3580_e5261 * ((assign3580_e5264 * (var_fn25_calc_iq__vgsin_dn2 - var_fn25_calc_iq__vgdin_dn2)) / ((assign3580_e5268).cosh() * (assign3580_e5268).cosh())))))), (0.5 * ((var_fn25_calc_iq__vgsin_dn7 + var_fn25_calc_iq__vgdin_dn7) + (((var_fn25_calc_iq__vgsin_dn7 - var_fn25_calc_iq__vgdin_dn7) * assign3580_e5269) + (assign3580_e5261 * ((assign3580_e5264 * (var_fn25_calc_iq__vgsin_dn7 - var_fn25_calc_iq__vgdin_dn7)) / ((assign3580_e5268).cosh() * (assign3580_e5268).cosh())))))), (0.5 * ((var_fn25_calc_iq__vgsin_dn16 + var_fn25_calc_iq__vgdin_dn16) + (((var_fn25_calc_iq__vgsin_dn16 - var_fn25_calc_iq__vgdin_dn16) * assign3580_e5269) + (assign3580_e5261 * ((assign3580_e5264 * (var_fn25_calc_iq__vgsin_dn16 - var_fn25_calc_iq__vgdin_dn16)) / ((assign3580_e5268).cosh() * (assign3580_e5268).cosh())))))), (0.5 * (var_fn25_calc_iq__vgdin_dn17 + (((-var_fn25_calc_iq__vgdin_dn17) * assign3580_e5269) + (assign3580_e5261 * ((assign3580_e5264 * (-var_fn25_calc_iq__vgdin_dn17)) / ((assign3580_e5268).cosh() * (assign3580_e5268).cosh())))))),)
            } else {
                let (assign3580_e5293, assign3580_e5293_d_n2, assign3580_e5293_d_n7, assign3580_e5293_d_n16, assign3580_e5293_d_n17,) = {
                    if (p.p52 == 0.0) {
                        let assign3580_e5279: f64 = (var_fn25_calc_iq__vgsin + var_fn25_calc_iq__vgdin);
                        let assign3580_e5282: f64 = (var_fn25_calc_iq__vgsin - var_fn25_calc_iq__vgdin);
                        let assign3580_e5285: f64 = (var_fn25_calc_iq__vgsin - var_fn25_calc_iq__vgdin);
                        let assign3580_e5286: f64 = (assign3580_e5282 * assign3580_e5285);
                        let assign3580_e5288: f64 = (assign3580_e5286 + p.p53);
                        let assign3580_e5289: f64 = (assign3580_e5288).sqrt();
                        let assign3580_e5290: f64 = (assign3580_e5279 + assign3580_e5289);
                        let assign3580_e5291: f64 = (0.5 * assign3580_e5290);
                        (assign3580_e5291, (0.5 * ((var_fn25_calc_iq__vgsin_dn2 + var_fn25_calc_iq__vgdin_dn2) + ((((var_fn25_calc_iq__vgsin_dn2 - var_fn25_calc_iq__vgdin_dn2) * assign3580_e5285) + (assign3580_e5282 * (var_fn25_calc_iq__vgsin_dn2 - var_fn25_calc_iq__vgdin_dn2))) / (2.0 * assign3580_e5289)))), (0.5 * ((var_fn25_calc_iq__vgsin_dn7 + var_fn25_calc_iq__vgdin_dn7) + ((((var_fn25_calc_iq__vgsin_dn7 - var_fn25_calc_iq__vgdin_dn7) * assign3580_e5285) + (assign3580_e5282 * (var_fn25_calc_iq__vgsin_dn7 - var_fn25_calc_iq__vgdin_dn7))) / (2.0 * assign3580_e5289)))), (0.5 * ((var_fn25_calc_iq__vgsin_dn16 + var_fn25_calc_iq__vgdin_dn16) + ((((var_fn25_calc_iq__vgsin_dn16 - var_fn25_calc_iq__vgdin_dn16) * assign3580_e5285) + (assign3580_e5282 * (var_fn25_calc_iq__vgsin_dn16 - var_fn25_calc_iq__vgdin_dn16))) / (2.0 * assign3580_e5289)))), (0.5 * (var_fn25_calc_iq__vgdin_dn17 + ((((-var_fn25_calc_iq__vgdin_dn17) * assign3580_e5285) + (assign3580_e5282 * (-var_fn25_calc_iq__vgdin_dn17))) / (2.0 * assign3580_e5289)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign3580_e5293, assign3580_e5293_d_n2, assign3580_e5293_d_n7, assign3580_e5293_d_n16, assign3580_e5293_d_n17,)
            }
        };
        let assign3580_e5296: f64 = (assign3580_e5294 - var_fn25_calc_iq__myarg0);
        let assign3580_e5298: f64 = (assign3580_e5296 / var_fn25_calc_iq__alpha_phit);
        (assign3580_e5298, (assign3580_e5294_d_n2 / var_fn25_calc_iq__alpha_phit), ((((-var_fn25_calc_iq__myarg0_dn4) * var_fn25_calc_iq__alpha_phit) - (assign3580_e5296 * var_fn25_calc_iq__alpha_phit_dn4)) / (var_fn25_calc_iq__alpha_phit * var_fn25_calc_iq__alpha_phit)), (assign3580_e5294_d_n7 / var_fn25_calc_iq__alpha_phit), (assign3580_e5294_d_n16 / var_fn25_calc_iq__alpha_phit), (assign3580_e5294_d_n17 / var_fn25_calc_iq__alpha_phit),)
    } else {
        (var_fn25_calc_iq__exparg0, var_fn25_calc_iq__exparg0_dn2, var_fn25_calc_iq__exparg0_dn4, var_fn25_calc_iq__exparg0_dn7, var_fn25_calc_iq__exparg0_dn16, var_fn25_calc_iq__exparg0_dn17,)
    }
};
        var_fn25_calc_iq__exparg0 = assign3580_e5300;
        var_fn25_calc_iq__exparg0_dn2 = assign3580_e5300_d_n2;
        var_fn25_calc_iq__exparg0_dn4 = assign3580_e5300_d_n4;
        var_fn25_calc_iq__exparg0_dn7 = assign3580_e5300_d_n7;
        var_fn25_calc_iq__exparg0_dn16 = assign3580_e5300_d_n16;
        var_fn25_calc_iq__exparg0_dn17 = assign3580_e5300_d_n17;
        var_fn25_calc_iq__exparg0_rv = 0.0;

        let assign3590_e5303: f64 = if var_fn25_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        var_guard39 = assign3590_e5303;
        var_guard39_rv = 0.0;

        let (assign3600_e5309, assign3600_e5309_d_n2, assign3600_e5309_d_n4, assign3600_e5309_d_n7, assign3600_e5309_d_n16, assign3600_e5309_d_n17,) = {
    if ((var_guard24 != 0.0) && (var_guard39 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__ff0, var_fn25_calc_iq__ff0_dn2, var_fn25_calc_iq__ff0_dn4, var_fn25_calc_iq__ff0_dn7, var_fn25_calc_iq__ff0_dn16, var_fn25_calc_iq__ff0_dn17,)
    }
};
        var_fn25_calc_iq__ff0 = assign3600_e5309;
        var_fn25_calc_iq__ff0_dn2 = assign3600_e5309_d_n2;
        var_fn25_calc_iq__ff0_dn4 = assign3600_e5309_d_n4;
        var_fn25_calc_iq__ff0_dn7 = assign3600_e5309_d_n7;
        var_fn25_calc_iq__ff0_dn16 = assign3600_e5309_d_n16;
        var_fn25_calc_iq__ff0_dn17 = assign3600_e5309_d_n17;
        var_fn25_calc_iq__ff0_rv = 0.0;

        let assign3610_e5312: f64 = (-50.0);
        let assign3610_e5313: f64 = if var_fn25_calc_iq__exparg0 < assign3610_e5312 { 1.0 } else { 0.0 };
        var_guard40 = assign3610_e5313;
        var_guard40_rv = 0.0;

        let (assign3620_e5322, assign3620_e5322_d_n2, assign3620_e5322_d_n4, assign3620_e5322_d_n7, assign3620_e5322_d_n16, assign3620_e5322_d_n17,) = {
    if (((var_guard24 != 0.0) && (var_guard39 == 0.0)) && (var_guard40 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__ff0, var_fn25_calc_iq__ff0_dn2, var_fn25_calc_iq__ff0_dn4, var_fn25_calc_iq__ff0_dn7, var_fn25_calc_iq__ff0_dn16, var_fn25_calc_iq__ff0_dn17,)
    }
};
        var_fn25_calc_iq__ff0 = assign3620_e5322;
        var_fn25_calc_iq__ff0_dn2 = assign3620_e5322_d_n2;
        var_fn25_calc_iq__ff0_dn4 = assign3620_e5322_d_n4;
        var_fn25_calc_iq__ff0_dn7 = assign3620_e5322_d_n7;
        var_fn25_calc_iq__ff0_dn16 = assign3620_e5322_d_n16;
        var_fn25_calc_iq__ff0_dn17 = assign3620_e5322_d_n17;
        var_fn25_calc_iq__ff0_rv = 0.0;

        let (assign3630_e5337, assign3630_e5337_d_n2, assign3630_e5337_d_n4, assign3630_e5337_d_n7, assign3630_e5337_d_n16, assign3630_e5337_d_n17,) = {
    if (((var_guard24 != 0.0) && (var_guard39 == 0.0)) && (var_guard40 == 0.0)) {
        let assign3630_e5333: f64 = (var_fn25_calc_iq__exparg0).exp();
        let assign3630_e5334: f64 = (1.0 + assign3630_e5333);
        let assign3630_e5335: f64 = (1.0 / assign3630_e5334);
        (assign3630_e5335, (-((assign3630_e5333 * var_fn25_calc_iq__exparg0_dn2) / (assign3630_e5334 * assign3630_e5334))), (-((assign3630_e5333 * var_fn25_calc_iq__exparg0_dn4) / (assign3630_e5334 * assign3630_e5334))), (-((assign3630_e5333 * var_fn25_calc_iq__exparg0_dn7) / (assign3630_e5334 * assign3630_e5334))), (-((assign3630_e5333 * var_fn25_calc_iq__exparg0_dn16) / (assign3630_e5334 * assign3630_e5334))), (-((assign3630_e5333 * var_fn25_calc_iq__exparg0_dn17) / (assign3630_e5334 * assign3630_e5334))),)
    } else {
        (var_fn25_calc_iq__ff0, var_fn25_calc_iq__ff0_dn2, var_fn25_calc_iq__ff0_dn4, var_fn25_calc_iq__ff0_dn7, var_fn25_calc_iq__ff0_dn16, var_fn25_calc_iq__ff0_dn17,)
    }
};
        var_fn25_calc_iq__ff0 = assign3630_e5337;
        var_fn25_calc_iq__ff0_dn2 = assign3630_e5337_d_n2;
        var_fn25_calc_iq__ff0_dn4 = assign3630_e5337_d_n4;
        var_fn25_calc_iq__ff0_dn7 = assign3630_e5337_d_n7;
        var_fn25_calc_iq__ff0_dn16 = assign3630_e5337_d_n16;
        var_fn25_calc_iq__ff0_dn17 = assign3630_e5337_d_n17;
        var_fn25_calc_iq__ff0_rv = 0.0;

        let (assign3640_e5396, assign3640_e5396_d_n2, assign3640_e5396_d_n4, assign3640_e5396_d_n7, assign3640_e5396_d_n16, assign3640_e5396_d_n17,) = {
    if (var_guard24 != 0.0) {
        let (assign3640_e5382, assign3640_e5382_d_n2, assign3640_e5382_d_n7, assign3640_e5382_d_n16, assign3640_e5382_d_n17,) = {
            if (p.p52 != 0.0) {
                let assign3640_e5346: f64 = (var_fn25_calc_iq__vgsin + var_fn25_calc_iq__vgdin);
                let assign3640_e5349: f64 = (var_fn25_calc_iq__vgsin - var_fn25_calc_iq__vgdin);
                let assign3640_e5352: f64 = (0.001 / p.p53);
                let assign3640_e5355: f64 = (var_fn25_calc_iq__vgsin - var_fn25_calc_iq__vgdin);
                let assign3640_e5356: f64 = (assign3640_e5352 * assign3640_e5355);
                let assign3640_e5357: f64 = (assign3640_e5356).tanh();
                let assign3640_e5358: f64 = (assign3640_e5349 * assign3640_e5357);
                let assign3640_e5359: f64 = (assign3640_e5346 + assign3640_e5358);
                let assign3640_e5360: f64 = (0.5 * assign3640_e5359);
                (assign3640_e5360, (0.5 * ((var_fn25_calc_iq__vgsin_dn2 + var_fn25_calc_iq__vgdin_dn2) + (((var_fn25_calc_iq__vgsin_dn2 - var_fn25_calc_iq__vgdin_dn2) * assign3640_e5357) + (assign3640_e5349 * ((assign3640_e5352 * (var_fn25_calc_iq__vgsin_dn2 - var_fn25_calc_iq__vgdin_dn2)) / ((assign3640_e5356).cosh() * (assign3640_e5356).cosh())))))), (0.5 * ((var_fn25_calc_iq__vgsin_dn7 + var_fn25_calc_iq__vgdin_dn7) + (((var_fn25_calc_iq__vgsin_dn7 - var_fn25_calc_iq__vgdin_dn7) * assign3640_e5357) + (assign3640_e5349 * ((assign3640_e5352 * (var_fn25_calc_iq__vgsin_dn7 - var_fn25_calc_iq__vgdin_dn7)) / ((assign3640_e5356).cosh() * (assign3640_e5356).cosh())))))), (0.5 * ((var_fn25_calc_iq__vgsin_dn16 + var_fn25_calc_iq__vgdin_dn16) + (((var_fn25_calc_iq__vgsin_dn16 - var_fn25_calc_iq__vgdin_dn16) * assign3640_e5357) + (assign3640_e5349 * ((assign3640_e5352 * (var_fn25_calc_iq__vgsin_dn16 - var_fn25_calc_iq__vgdin_dn16)) / ((assign3640_e5356).cosh() * (assign3640_e5356).cosh())))))), (0.5 * (var_fn25_calc_iq__vgdin_dn17 + (((-var_fn25_calc_iq__vgdin_dn17) * assign3640_e5357) + (assign3640_e5349 * ((assign3640_e5352 * (-var_fn25_calc_iq__vgdin_dn17)) / ((assign3640_e5356).cosh() * (assign3640_e5356).cosh())))))),)
            } else {
                let (assign3640_e5381, assign3640_e5381_d_n2, assign3640_e5381_d_n7, assign3640_e5381_d_n16, assign3640_e5381_d_n17,) = {
                    if (p.p52 == 0.0) {
                        let assign3640_e5367: f64 = (var_fn25_calc_iq__vgsin + var_fn25_calc_iq__vgdin);
                        let assign3640_e5370: f64 = (var_fn25_calc_iq__vgsin - var_fn25_calc_iq__vgdin);
                        let assign3640_e5373: f64 = (var_fn25_calc_iq__vgsin - var_fn25_calc_iq__vgdin);
                        let assign3640_e5374: f64 = (assign3640_e5370 * assign3640_e5373);
                        let assign3640_e5376: f64 = (assign3640_e5374 + p.p53);
                        let assign3640_e5377: f64 = (assign3640_e5376).sqrt();
                        let assign3640_e5378: f64 = (assign3640_e5367 + assign3640_e5377);
                        let assign3640_e5379: f64 = (0.5 * assign3640_e5378);
                        (assign3640_e5379, (0.5 * ((var_fn25_calc_iq__vgsin_dn2 + var_fn25_calc_iq__vgdin_dn2) + ((((var_fn25_calc_iq__vgsin_dn2 - var_fn25_calc_iq__vgdin_dn2) * assign3640_e5373) + (assign3640_e5370 * (var_fn25_calc_iq__vgsin_dn2 - var_fn25_calc_iq__vgdin_dn2))) / (2.0 * assign3640_e5377)))), (0.5 * ((var_fn25_calc_iq__vgsin_dn7 + var_fn25_calc_iq__vgdin_dn7) + ((((var_fn25_calc_iq__vgsin_dn7 - var_fn25_calc_iq__vgdin_dn7) * assign3640_e5373) + (assign3640_e5370 * (var_fn25_calc_iq__vgsin_dn7 - var_fn25_calc_iq__vgdin_dn7))) / (2.0 * assign3640_e5377)))), (0.5 * ((var_fn25_calc_iq__vgsin_dn16 + var_fn25_calc_iq__vgdin_dn16) + ((((var_fn25_calc_iq__vgsin_dn16 - var_fn25_calc_iq__vgdin_dn16) * assign3640_e5373) + (assign3640_e5370 * (var_fn25_calc_iq__vgsin_dn16 - var_fn25_calc_iq__vgdin_dn16))) / (2.0 * assign3640_e5377)))), (0.5 * (var_fn25_calc_iq__vgdin_dn17 + ((((-var_fn25_calc_iq__vgdin_dn17) * assign3640_e5373) + (assign3640_e5370 * (-var_fn25_calc_iq__vgdin_dn17))) / (2.0 * assign3640_e5377)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign3640_e5381, assign3640_e5381_d_n2, assign3640_e5381_d_n7, assign3640_e5381_d_n16, assign3640_e5381_d_n17,)
            }
        };
        let assign3640_e5386: f64 = (p.p51 * 0.1);
        let assign3640_e5388: f64 = (assign3640_e5386 * var_fn25_calc_iq__alpha_phit);
        let assign3640_e5390: f64 = (assign3640_e5388 * var_fn25_calc_iq__ff0);
        let assign3640_e5391: f64 = (var_fn25_calc_iq__vtof - assign3640_e5390);
        let assign3640_e5392: f64 = (assign3640_e5382 - assign3640_e5391);
        let assign3640_e5394: f64 = (assign3640_e5392 / var_fn25_calc_iq__two_n_phit0);
        (assign3640_e5394, ((assign3640_e5382_d_n2 - (-(assign3640_e5388 * var_fn25_calc_iq__ff0_dn2))) / var_fn25_calc_iq__two_n_phit0), ((((-(var_fn25_calc_iq__vtof_dn4 - (((assign3640_e5386 * var_fn25_calc_iq__alpha_phit_dn4) * var_fn25_calc_iq__ff0) + (assign3640_e5388 * var_fn25_calc_iq__ff0_dn4)))) * var_fn25_calc_iq__two_n_phit0) - (assign3640_e5392 * var_fn25_calc_iq__two_n_phit0_dn4)) / (var_fn25_calc_iq__two_n_phit0 * var_fn25_calc_iq__two_n_phit0)), ((assign3640_e5382_d_n7 - (-(assign3640_e5388 * var_fn25_calc_iq__ff0_dn7))) / var_fn25_calc_iq__two_n_phit0), ((assign3640_e5382_d_n16 - (-(assign3640_e5388 * var_fn25_calc_iq__ff0_dn16))) / var_fn25_calc_iq__two_n_phit0), ((assign3640_e5382_d_n17 - (-(assign3640_e5388 * var_fn25_calc_iq__ff0_dn17))) / var_fn25_calc_iq__two_n_phit0),)
    } else {
        (var_fn25_calc_iq__eta0, var_fn25_calc_iq__eta0_dn2, var_fn25_calc_iq__eta0_dn4, var_fn25_calc_iq__eta0_dn7, var_fn25_calc_iq__eta0_dn16, var_fn25_calc_iq__eta0_dn17,)
    }
};
        var_fn25_calc_iq__eta0 = assign3640_e5396;
        var_fn25_calc_iq__eta0_dn2 = assign3640_e5396_d_n2;
        var_fn25_calc_iq__eta0_dn4 = assign3640_e5396_d_n4;
        var_fn25_calc_iq__eta0_dn7 = assign3640_e5396_d_n7;
        var_fn25_calc_iq__eta0_dn16 = assign3640_e5396_d_n16;
        var_fn25_calc_iq__eta0_dn17 = assign3640_e5396_d_n17;
        var_fn25_calc_iq__eta0_rv = 0.0;

        let assign3650_e5399: f64 = if var_fn25_calc_iq__eta0 > 50.0 { 1.0 } else { 0.0 };
        var_guard41 = assign3650_e5399;
        var_guard41_rv = 0.0;

        let (assign3660_e5407, assign3660_e5407_d_n2, assign3660_e5407_d_n4, assign3660_e5407_d_n7, assign3660_e5407_d_n16, assign3660_e5407_d_n17,) = {
    if ((var_guard24 != 0.0) && (var_guard41 != 0.0)) {
        let assign3660_e5405: f64 = (var_fn25_calc_iq__qref0 * var_fn25_calc_iq__eta0);
        (assign3660_e5405, (var_fn25_calc_iq__qref0 * var_fn25_calc_iq__eta0_dn2), ((var_fn25_calc_iq__qref0_dn4 * var_fn25_calc_iq__eta0) + (var_fn25_calc_iq__qref0 * var_fn25_calc_iq__eta0_dn4)), (var_fn25_calc_iq__qref0 * var_fn25_calc_iq__eta0_dn7), (var_fn25_calc_iq__qref0 * var_fn25_calc_iq__eta0_dn16), (var_fn25_calc_iq__qref0 * var_fn25_calc_iq__eta0_dn17),)
    } else {
        (var_fn25_calc_iq__qinvv0, var_fn25_calc_iq__qinvv0_dn2, var_fn25_calc_iq__qinvv0_dn4, var_fn25_calc_iq__qinvv0_dn7, var_fn25_calc_iq__qinvv0_dn16, var_fn25_calc_iq__qinvv0_dn17,)
    }
};
        var_fn25_calc_iq__qinvv0 = assign3660_e5407;
        var_fn25_calc_iq__qinvv0_dn2 = assign3660_e5407_d_n2;
        var_fn25_calc_iq__qinvv0_dn4 = assign3660_e5407_d_n4;
        var_fn25_calc_iq__qinvv0_dn7 = assign3660_e5407_d_n7;
        var_fn25_calc_iq__qinvv0_dn16 = assign3660_e5407_d_n16;
        var_fn25_calc_iq__qinvv0_dn17 = assign3660_e5407_d_n17;
        var_fn25_calc_iq__qinvv0_rv = 0.0;

        let assign3670_e5410: f64 = (-50.0);
        let assign3670_e5411: f64 = if var_fn25_calc_iq__eta0 < assign3670_e5410 { 1.0 } else { 0.0 };
        var_guard42 = assign3670_e5411;
        var_guard42_rv = 0.0;

        let (assign3680_e5423, assign3680_e5423_d_n2, assign3680_e5423_d_n4, assign3680_e5423_d_n7, assign3680_e5423_d_n16, assign3680_e5423_d_n17,) = {
    if (((var_guard24 != 0.0) && (var_guard41 == 0.0)) && (var_guard42 != 0.0)) {
        let assign3680_e5420: f64 = (var_fn25_calc_iq__eta0).exp();
        let assign3680_e5421: f64 = (var_fn25_calc_iq__qref0 * assign3680_e5420);
        (assign3680_e5421, (var_fn25_calc_iq__qref0 * (assign3680_e5420 * var_fn25_calc_iq__eta0_dn2)), ((var_fn25_calc_iq__qref0_dn4 * assign3680_e5420) + (var_fn25_calc_iq__qref0 * (assign3680_e5420 * var_fn25_calc_iq__eta0_dn4))), (var_fn25_calc_iq__qref0 * (assign3680_e5420 * var_fn25_calc_iq__eta0_dn7)), (var_fn25_calc_iq__qref0 * (assign3680_e5420 * var_fn25_calc_iq__eta0_dn16)), (var_fn25_calc_iq__qref0 * (assign3680_e5420 * var_fn25_calc_iq__eta0_dn17)),)
    } else {
        (var_fn25_calc_iq__qinvv0, var_fn25_calc_iq__qinvv0_dn2, var_fn25_calc_iq__qinvv0_dn4, var_fn25_calc_iq__qinvv0_dn7, var_fn25_calc_iq__qinvv0_dn16, var_fn25_calc_iq__qinvv0_dn17,)
    }
};
        var_fn25_calc_iq__qinvv0 = assign3680_e5423;
        var_fn25_calc_iq__qinvv0_dn2 = assign3680_e5423_d_n2;
        var_fn25_calc_iq__qinvv0_dn4 = assign3680_e5423_d_n4;
        var_fn25_calc_iq__qinvv0_dn7 = assign3680_e5423_d_n7;
        var_fn25_calc_iq__qinvv0_dn16 = assign3680_e5423_d_n16;
        var_fn25_calc_iq__qinvv0_dn17 = assign3680_e5423_d_n17;
        var_fn25_calc_iq__qinvv0_rv = 0.0;

        let (assign3690_e5439, assign3690_e5439_d_n2, assign3690_e5439_d_n4, assign3690_e5439_d_n7, assign3690_e5439_d_n16, assign3690_e5439_d_n17,) = {
    if (((var_guard24 != 0.0) && (var_guard41 == 0.0)) && (var_guard42 == 0.0)) {
        let assign3690_e5434: f64 = (var_fn25_calc_iq__eta0).exp();
        let assign3690_e5435: f64 = (1.0 + assign3690_e5434);
        let assign3690_e5436: f64 = (assign3690_e5435).ln();
        let assign3690_e5437: f64 = (var_fn25_calc_iq__qref0 * assign3690_e5436);
        (assign3690_e5437, (var_fn25_calc_iq__qref0 * ((assign3690_e5434 * var_fn25_calc_iq__eta0_dn2) / assign3690_e5435)), ((var_fn25_calc_iq__qref0_dn4 * assign3690_e5436) + (var_fn25_calc_iq__qref0 * ((assign3690_e5434 * var_fn25_calc_iq__eta0_dn4) / assign3690_e5435))), (var_fn25_calc_iq__qref0 * ((assign3690_e5434 * var_fn25_calc_iq__eta0_dn7) / assign3690_e5435)), (var_fn25_calc_iq__qref0 * ((assign3690_e5434 * var_fn25_calc_iq__eta0_dn16) / assign3690_e5435)), (var_fn25_calc_iq__qref0 * ((assign3690_e5434 * var_fn25_calc_iq__eta0_dn17) / assign3690_e5435)),)
    } else {
        (var_fn25_calc_iq__qinvv0, var_fn25_calc_iq__qinvv0_dn2, var_fn25_calc_iq__qinvv0_dn4, var_fn25_calc_iq__qinvv0_dn7, var_fn25_calc_iq__qinvv0_dn16, var_fn25_calc_iq__qinvv0_dn17,)
    }
};
        var_fn25_calc_iq__qinvv0 = assign3690_e5439;
        var_fn25_calc_iq__qinvv0_dn2 = assign3690_e5439_d_n2;
        var_fn25_calc_iq__qinvv0_dn4 = assign3690_e5439_d_n4;
        var_fn25_calc_iq__qinvv0_dn7 = assign3690_e5439_d_n7;
        var_fn25_calc_iq__qinvv0_dn16 = assign3690_e5439_d_n16;
        var_fn25_calc_iq__qinvv0_dn17 = assign3690_e5439_d_n17;
        var_fn25_calc_iq__qinvv0_rv = 0.0;

        let (assign3700_e5445, assign3700_e5445_d_n4,) = {
    if (var_guard24 != 0.0) {
        let assign3700_e5443: f64 = (var_fn25_calc_iq__mu0 / var_fn25_calc_iq__tfacmobin);
        (assign3700_e5443, (-((var_fn25_calc_iq__mu0 * var_fn25_calc_iq__tfacmobin_dn4) / (var_fn25_calc_iq__tfacmobin * var_fn25_calc_iq__tfacmobin))),)
    } else {
        (var_fn25_calc_iq__muf0, var_fn25_calc_iq__muf0_dn4,)
    }
};
        var_fn25_calc_iq__muf0 = assign3700_e5445;
        var_fn25_calc_iq__muf0_dn4 = assign3700_e5445_d_n4;
        var_fn25_calc_iq__muf0_rv = 0.0;

        let (assign3710_e5461, assign3710_e5461_d_n4,) = {
    if (var_guard24 != 0.0) {
        let assign3710_e5451: f64 = (var_fn25_calc_iq__vzeta * var_fn25_calc_iq__tnomin);
        let assign3710_e5452: f64 = (1.0 + assign3710_e5451);
        let assign3710_e5456: f64 = (var_fn25_calc_iq__vzeta * var_fn25_calc_iq__tambin);
        let assign3710_e5457: f64 = (1.0 + assign3710_e5456);
        let assign3710_e5458: f64 = (assign3710_e5452 / assign3710_e5457);
        let assign3710_e5459: f64 = (var_fn25_calc_iq__vel0 * assign3710_e5458);
        (assign3710_e5459, (var_fn25_calc_iq__vel0 * (-((assign3710_e5452 * (var_fn25_calc_iq__vzeta * var_fn25_calc_iq__tambin_dn4)) / (assign3710_e5457 * assign3710_e5457)))),)
    } else {
        (var_fn25_calc_iq__vx0, var_fn25_calc_iq__vx0_dn4,)
    }
};
        var_fn25_calc_iq__vx0 = assign3710_e5461;
        var_fn25_calc_iq__vx0_dn4 = assign3710_e5461_d_n4;
        var_fn25_calc_iq__vx0_rv = 0.0;

        let (assign3720_e5469, assign3720_e5469_d_n4,) = {
    if (var_guard24 != 0.0) {
        let assign3720_e5465: f64 = (var_fn25_calc_iq__vx0 * var_fn25_calc_iq__lin);
        let assign3720_e5467: f64 = (assign3720_e5465 / var_fn25_calc_iq__muf0);
        (assign3720_e5467, ((((var_fn25_calc_iq__vx0_dn4 * var_fn25_calc_iq__lin) * var_fn25_calc_iq__muf0) - (assign3720_e5465 * var_fn25_calc_iq__muf0_dn4)) / (var_fn25_calc_iq__muf0 * var_fn25_calc_iq__muf0)),)
    } else {
        (var_fn25_calc_iq__vdsats0, var_fn25_calc_iq__vdsats0_dn4,)
    }
};
        var_fn25_calc_iq__vdsats0 = assign3720_e5469;
        var_fn25_calc_iq__vdsats0_dn4 = assign3720_e5469_d_n4;
        var_fn25_calc_iq__vdsats0_rv = 0.0;

        let (assign3730_e5486, assign3730_e5486_d_n2, assign3730_e5486_d_n4, assign3730_e5486_d_n7, assign3730_e5486_d_n16, assign3730_e5486_d_n17,) = {
    if (var_guard24 != 0.0) {
        let assign3730_e5475: f64 = (2.0 * var_fn25_calc_iq__qinvv0);
        let assign3730_e5477: f64 = (assign3730_e5475 / var_fn25_calc_iq__cgin);
        let assign3730_e5479: f64 = (assign3730_e5477 / var_fn25_calc_iq__vdsats0);
        let assign3730_e5480: f64 = (1.0 + assign3730_e5479);
        let assign3730_e5481: f64 = (assign3730_e5480).sqrt();
        let assign3730_e5482: f64 = (var_fn25_calc_iq__vdsats0 * assign3730_e5481);
        let assign3730_e5484: f64 = (assign3730_e5482 - var_fn25_calc_iq__vdsats0);
        (assign3730_e5484, (var_fn25_calc_iq__vdsats0 * ((((2.0 * var_fn25_calc_iq__qinvv0_dn2) / var_fn25_calc_iq__cgin) / var_fn25_calc_iq__vdsats0) / (2.0 * assign3730_e5481))), (((var_fn25_calc_iq__vdsats0_dn4 * assign3730_e5481) + (var_fn25_calc_iq__vdsats0 * ((((((((2.0 * var_fn25_calc_iq__qinvv0_dn4) * var_fn25_calc_iq__cgin) - (assign3730_e5475 * var_fn25_calc_iq__cgin_dn4)) / (var_fn25_calc_iq__cgin * var_fn25_calc_iq__cgin)) * var_fn25_calc_iq__vdsats0) - (assign3730_e5477 * var_fn25_calc_iq__vdsats0_dn4)) / (var_fn25_calc_iq__vdsats0 * var_fn25_calc_iq__vdsats0)) / (2.0 * assign3730_e5481)))) - var_fn25_calc_iq__vdsats0_dn4), (var_fn25_calc_iq__vdsats0 * ((((2.0 * var_fn25_calc_iq__qinvv0_dn7) / var_fn25_calc_iq__cgin) / var_fn25_calc_iq__vdsats0) / (2.0 * assign3730_e5481))), (var_fn25_calc_iq__vdsats0 * ((((2.0 * var_fn25_calc_iq__qinvv0_dn16) / var_fn25_calc_iq__cgin) / var_fn25_calc_iq__vdsats0) / (2.0 * assign3730_e5481))), (var_fn25_calc_iq__vdsats0 * ((((2.0 * var_fn25_calc_iq__qinvv0_dn17) / var_fn25_calc_iq__cgin) / var_fn25_calc_iq__vdsats0) / (2.0 * assign3730_e5481))),)
    } else {
        (var_fn25_calc_iq__vdsats10, var_fn25_calc_iq__vdsats10_dn2, var_fn25_calc_iq__vdsats10_dn4, var_fn25_calc_iq__vdsats10_dn7, var_fn25_calc_iq__vdsats10_dn16, var_fn25_calc_iq__vdsats10_dn17,)
    }
};
        var_fn25_calc_iq__vdsats10 = assign3730_e5486;
        var_fn25_calc_iq__vdsats10_dn2 = assign3730_e5486_d_n2;
        var_fn25_calc_iq__vdsats10_dn4 = assign3730_e5486_d_n4;
        var_fn25_calc_iq__vdsats10_dn7 = assign3730_e5486_d_n7;
        var_fn25_calc_iq__vdsats10_dn16 = assign3730_e5486_d_n16;
        var_fn25_calc_iq__vdsats10_dn17 = assign3730_e5486_d_n17;
        var_fn25_calc_iq__vdsats10_rv = 0.0;

        let (assign3740_e5498, assign3740_e5498_d_n2, assign3740_e5498_d_n4, assign3740_e5498_d_n7, assign3740_e5498_d_n16, assign3740_e5498_d_n17,) = {
    if (var_guard24 != 0.0) {
        let assign3740_e5491: f64 = (1.0 - var_fn25_calc_iq__ff0);
        let assign3740_e5492: f64 = (var_fn25_calc_iq__vdsats10 * assign3740_e5491);
        let assign3740_e5495: f64 = (var_fn25_calc_iq__two_n_phit0 * var_fn25_calc_iq__ff0);
        let assign3740_e5496: f64 = (assign3740_e5492 + assign3740_e5495);
        (assign3740_e5496, (((var_fn25_calc_iq__vdsats10_dn2 * assign3740_e5491) + (var_fn25_calc_iq__vdsats10 * (-var_fn25_calc_iq__ff0_dn2))) + (var_fn25_calc_iq__two_n_phit0 * var_fn25_calc_iq__ff0_dn2)), (((var_fn25_calc_iq__vdsats10_dn4 * assign3740_e5491) + (var_fn25_calc_iq__vdsats10 * (-var_fn25_calc_iq__ff0_dn4))) + ((var_fn25_calc_iq__two_n_phit0_dn4 * var_fn25_calc_iq__ff0) + (var_fn25_calc_iq__two_n_phit0 * var_fn25_calc_iq__ff0_dn4))), (((var_fn25_calc_iq__vdsats10_dn7 * assign3740_e5491) + (var_fn25_calc_iq__vdsats10 * (-var_fn25_calc_iq__ff0_dn7))) + (var_fn25_calc_iq__two_n_phit0 * var_fn25_calc_iq__ff0_dn7)), (((var_fn25_calc_iq__vdsats10_dn16 * assign3740_e5491) + (var_fn25_calc_iq__vdsats10 * (-var_fn25_calc_iq__ff0_dn16))) + (var_fn25_calc_iq__two_n_phit0 * var_fn25_calc_iq__ff0_dn16)), (((var_fn25_calc_iq__vdsats10_dn17 * assign3740_e5491) + (var_fn25_calc_iq__vdsats10 * (-var_fn25_calc_iq__ff0_dn17))) + (var_fn25_calc_iq__two_n_phit0 * var_fn25_calc_iq__ff0_dn17)),)
    } else {
        (var_fn25_calc_iq__vdsat10, var_fn25_calc_iq__vdsat10_dn2, var_fn25_calc_iq__vdsat10_dn4, var_fn25_calc_iq__vdsat10_dn7, var_fn25_calc_iq__vdsat10_dn16, var_fn25_calc_iq__vdsat10_dn17,)
    }
};
        var_fn25_calc_iq__vdsat10 = assign3740_e5498;
        var_fn25_calc_iq__vdsat10_dn2 = assign3740_e5498_d_n2;
        var_fn25_calc_iq__vdsat10_dn4 = assign3740_e5498_d_n4;
        var_fn25_calc_iq__vdsat10_dn7 = assign3740_e5498_d_n7;
        var_fn25_calc_iq__vdsat10_dn16 = assign3740_e5498_d_n16;
        var_fn25_calc_iq__vdsat10_dn17 = assign3740_e5498_d_n17;
        var_fn25_calc_iq__vdsat10_rv = 0.0;

        let (assign3750_e5567, assign3750_e5567_d_n2, assign3750_e5567_d_n4, assign3750_e5567_d_n7, assign3750_e5567_d_n16, assign3750_e5567_d_n17,) = {
    if (var_guard24 != 0.0) {
        let (assign3750_e5557, assign3750_e5557_d_n2, assign3750_e5557_d_n4, assign3750_e5557_d_n7, assign3750_e5557_d_n16, assign3750_e5557_d_n17,) = {
            if (p.p52 != 0.0) {
                let assign3750_e5510: f64 = (var_fn25_calc_iq__vdsin / var_fn25_calc_iq__vdsat10);
                let assign3750_e5511: f64 = assign3750_e5510;
                let assign3750_e5515: f64 = (var_fn25_calc_iq__vdsin / var_fn25_calc_iq__vdsat10);
                let assign3750_e5516: f64 = (-assign3750_e5515);
                let assign3750_e5519: f64 = (0.001 / p.p53);
                let assign3750_e5523: f64 = (var_fn25_calc_iq__vdsin / var_fn25_calc_iq__vdsat10);
                let assign3750_e5524: f64 = (-assign3750_e5523);
                let assign3750_e5525: f64 = (assign3750_e5519 * assign3750_e5524);
                let assign3750_e5526: f64 = (assign3750_e5525).tanh();
                let assign3750_e5527: f64 = (assign3750_e5516 * assign3750_e5526);
                let assign3750_e5528: f64 = (assign3750_e5511 + assign3750_e5527);
                let assign3750_e5529: f64 = (0.5 * assign3750_e5528);
                (assign3750_e5529, (0.5 * ((-((var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat10_dn2) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10))) + (((-(-((var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat10_dn2) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10)))) * assign3750_e5526) + (assign3750_e5516 * ((assign3750_e5519 * (-(-((var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat10_dn2) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10))))) / ((assign3750_e5525).cosh() * (assign3750_e5525).cosh())))))), (0.5 * ((-((var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat10_dn4) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10))) + (((-(-((var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat10_dn4) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10)))) * assign3750_e5526) + (assign3750_e5516 * ((assign3750_e5519 * (-(-((var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat10_dn4) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10))))) / ((assign3750_e5525).cosh() * (assign3750_e5525).cosh())))))), (0.5 * ((-((var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat10_dn7) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10))) + (((-(-((var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat10_dn7) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10)))) * assign3750_e5526) + (assign3750_e5516 * ((assign3750_e5519 * (-(-((var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat10_dn7) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10))))) / ((assign3750_e5525).cosh() * (assign3750_e5525).cosh())))))), (0.5 * ((((var_fn25_calc_iq__vdsin_dn16 * var_fn25_calc_iq__vdsat10) - (var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat10_dn16)) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10)) + (((-(((var_fn25_calc_iq__vdsin_dn16 * var_fn25_calc_iq__vdsat10) - (var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat10_dn16)) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10))) * assign3750_e5526) + (assign3750_e5516 * ((assign3750_e5519 * (-(((var_fn25_calc_iq__vdsin_dn16 * var_fn25_calc_iq__vdsat10) - (var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat10_dn16)) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10)))) / ((assign3750_e5525).cosh() * (assign3750_e5525).cosh())))))), (0.5 * ((((var_fn25_calc_iq__vdsin_dn17 * var_fn25_calc_iq__vdsat10) - (var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat10_dn17)) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10)) + (((-(((var_fn25_calc_iq__vdsin_dn17 * var_fn25_calc_iq__vdsat10) - (var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat10_dn17)) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10))) * assign3750_e5526) + (assign3750_e5516 * ((assign3750_e5519 * (-(((var_fn25_calc_iq__vdsin_dn17 * var_fn25_calc_iq__vdsat10) - (var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat10_dn17)) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10)))) / ((assign3750_e5525).cosh() * (assign3750_e5525).cosh())))))),)
            } else {
                let (assign3750_e5556, assign3750_e5556_d_n2, assign3750_e5556_d_n4, assign3750_e5556_d_n7, assign3750_e5556_d_n16, assign3750_e5556_d_n17,) = {
                    if (p.p52 == 0.0) {
                        let assign3750_e5537: f64 = (var_fn25_calc_iq__vdsin / var_fn25_calc_iq__vdsat10);
                        let assign3750_e5538: f64 = assign3750_e5537;
                        let assign3750_e5542: f64 = (var_fn25_calc_iq__vdsin / var_fn25_calc_iq__vdsat10);
                        let assign3750_e5543: f64 = (-assign3750_e5542);
                        let assign3750_e5547: f64 = (var_fn25_calc_iq__vdsin / var_fn25_calc_iq__vdsat10);
                        let assign3750_e5548: f64 = (-assign3750_e5547);
                        let assign3750_e5549: f64 = (assign3750_e5543 * assign3750_e5548);
                        let assign3750_e5551: f64 = (assign3750_e5549 + p.p53);
                        let assign3750_e5552: f64 = (assign3750_e5551).sqrt();
                        let assign3750_e5553: f64 = (assign3750_e5538 + assign3750_e5552);
                        let assign3750_e5554: f64 = (0.5 * assign3750_e5553);
                        (assign3750_e5554, (0.5 * ((-((var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat10_dn2) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10))) + ((((-(-((var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat10_dn2) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10)))) * assign3750_e5548) + (assign3750_e5543 * (-(-((var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat10_dn2) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10)))))) / (2.0 * assign3750_e5552)))), (0.5 * ((-((var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat10_dn4) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10))) + ((((-(-((var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat10_dn4) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10)))) * assign3750_e5548) + (assign3750_e5543 * (-(-((var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat10_dn4) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10)))))) / (2.0 * assign3750_e5552)))), (0.5 * ((-((var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat10_dn7) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10))) + ((((-(-((var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat10_dn7) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10)))) * assign3750_e5548) + (assign3750_e5543 * (-(-((var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat10_dn7) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10)))))) / (2.0 * assign3750_e5552)))), (0.5 * ((((var_fn25_calc_iq__vdsin_dn16 * var_fn25_calc_iq__vdsat10) - (var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat10_dn16)) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10)) + ((((-(((var_fn25_calc_iq__vdsin_dn16 * var_fn25_calc_iq__vdsat10) - (var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat10_dn16)) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10))) * assign3750_e5548) + (assign3750_e5543 * (-(((var_fn25_calc_iq__vdsin_dn16 * var_fn25_calc_iq__vdsat10) - (var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat10_dn16)) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10))))) / (2.0 * assign3750_e5552)))), (0.5 * ((((var_fn25_calc_iq__vdsin_dn17 * var_fn25_calc_iq__vdsat10) - (var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat10_dn17)) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10)) + ((((-(((var_fn25_calc_iq__vdsin_dn17 * var_fn25_calc_iq__vdsat10) - (var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat10_dn17)) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10))) * assign3750_e5548) + (assign3750_e5543 * (-(((var_fn25_calc_iq__vdsin_dn17 * var_fn25_calc_iq__vdsat10) - (var_fn25_calc_iq__vdsin * var_fn25_calc_iq__vdsat10_dn17)) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10))))) / (2.0 * assign3750_e5552)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign3750_e5556, assign3750_e5556_d_n2, assign3750_e5556_d_n4, assign3750_e5556_d_n7, assign3750_e5556_d_n16, assign3750_e5556_d_n17,)
            }
        };
        let assign3750_e5559: f64 = (assign3750_e5557).powf(var_fn25_calc_iq__beta);
        let assign3750_e5560: f64 = (1.0 + assign3750_e5559);
        let assign3750_e5563: f64 = (1.0 / var_fn25_calc_iq__beta);
        let assign3750_e5564: f64 = (assign3750_e5560).powf(assign3750_e5563);
        let assign3750_e5565: f64 = (1.0 / assign3750_e5564);
        (assign3750_e5565, (-(if 0.0 == 0.0 && ((assign3750_e5563) as f64).is_finite() && ((assign3750_e5563) as f64).fract() == 0.0 { if assign3750_e5563 == 0.0 { 0.0 } else { (assign3750_e5563 * ((assign3750_e5560).powf(assign3750_e5563 - 1.0) * if 0.0 == 0.0 && ((var_fn25_calc_iq__beta) as f64).is_finite() && ((var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (var_fn25_calc_iq__beta * ((assign3750_e5557).powf(var_fn25_calc_iq__beta - 1.0) * assign3750_e5557_d_n2)) } } else { (assign3750_e5559 * (var_fn25_calc_iq__beta * (assign3750_e5557_d_n2 / assign3750_e5557))) })) } } else { (assign3750_e5564 * (assign3750_e5563 * (if 0.0 == 0.0 && ((var_fn25_calc_iq__beta) as f64).is_finite() && ((var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (var_fn25_calc_iq__beta * ((assign3750_e5557).powf(var_fn25_calc_iq__beta - 1.0) * assign3750_e5557_d_n2)) } } else { (assign3750_e5559 * (var_fn25_calc_iq__beta * (assign3750_e5557_d_n2 / assign3750_e5557))) } / assign3750_e5560))) } / (assign3750_e5564 * assign3750_e5564))), (-(if 0.0 == 0.0 && ((assign3750_e5563) as f64).is_finite() && ((assign3750_e5563) as f64).fract() == 0.0 { if assign3750_e5563 == 0.0 { 0.0 } else { (assign3750_e5563 * ((assign3750_e5560).powf(assign3750_e5563 - 1.0) * if 0.0 == 0.0 && ((var_fn25_calc_iq__beta) as f64).is_finite() && ((var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (var_fn25_calc_iq__beta * ((assign3750_e5557).powf(var_fn25_calc_iq__beta - 1.0) * assign3750_e5557_d_n4)) } } else { (assign3750_e5559 * (var_fn25_calc_iq__beta * (assign3750_e5557_d_n4 / assign3750_e5557))) })) } } else { (assign3750_e5564 * (assign3750_e5563 * (if 0.0 == 0.0 && ((var_fn25_calc_iq__beta) as f64).is_finite() && ((var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (var_fn25_calc_iq__beta * ((assign3750_e5557).powf(var_fn25_calc_iq__beta - 1.0) * assign3750_e5557_d_n4)) } } else { (assign3750_e5559 * (var_fn25_calc_iq__beta * (assign3750_e5557_d_n4 / assign3750_e5557))) } / assign3750_e5560))) } / (assign3750_e5564 * assign3750_e5564))), (-(if 0.0 == 0.0 && ((assign3750_e5563) as f64).is_finite() && ((assign3750_e5563) as f64).fract() == 0.0 { if assign3750_e5563 == 0.0 { 0.0 } else { (assign3750_e5563 * ((assign3750_e5560).powf(assign3750_e5563 - 1.0) * if 0.0 == 0.0 && ((var_fn25_calc_iq__beta) as f64).is_finite() && ((var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (var_fn25_calc_iq__beta * ((assign3750_e5557).powf(var_fn25_calc_iq__beta - 1.0) * assign3750_e5557_d_n7)) } } else { (assign3750_e5559 * (var_fn25_calc_iq__beta * (assign3750_e5557_d_n7 / assign3750_e5557))) })) } } else { (assign3750_e5564 * (assign3750_e5563 * (if 0.0 == 0.0 && ((var_fn25_calc_iq__beta) as f64).is_finite() && ((var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (var_fn25_calc_iq__beta * ((assign3750_e5557).powf(var_fn25_calc_iq__beta - 1.0) * assign3750_e5557_d_n7)) } } else { (assign3750_e5559 * (var_fn25_calc_iq__beta * (assign3750_e5557_d_n7 / assign3750_e5557))) } / assign3750_e5560))) } / (assign3750_e5564 * assign3750_e5564))), (-(if 0.0 == 0.0 && ((assign3750_e5563) as f64).is_finite() && ((assign3750_e5563) as f64).fract() == 0.0 { if assign3750_e5563 == 0.0 { 0.0 } else { (assign3750_e5563 * ((assign3750_e5560).powf(assign3750_e5563 - 1.0) * if 0.0 == 0.0 && ((var_fn25_calc_iq__beta) as f64).is_finite() && ((var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (var_fn25_calc_iq__beta * ((assign3750_e5557).powf(var_fn25_calc_iq__beta - 1.0) * assign3750_e5557_d_n16)) } } else { (assign3750_e5559 * (var_fn25_calc_iq__beta * (assign3750_e5557_d_n16 / assign3750_e5557))) })) } } else { (assign3750_e5564 * (assign3750_e5563 * (if 0.0 == 0.0 && ((var_fn25_calc_iq__beta) as f64).is_finite() && ((var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (var_fn25_calc_iq__beta * ((assign3750_e5557).powf(var_fn25_calc_iq__beta - 1.0) * assign3750_e5557_d_n16)) } } else { (assign3750_e5559 * (var_fn25_calc_iq__beta * (assign3750_e5557_d_n16 / assign3750_e5557))) } / assign3750_e5560))) } / (assign3750_e5564 * assign3750_e5564))), (-(if 0.0 == 0.0 && ((assign3750_e5563) as f64).is_finite() && ((assign3750_e5563) as f64).fract() == 0.0 { if assign3750_e5563 == 0.0 { 0.0 } else { (assign3750_e5563 * ((assign3750_e5560).powf(assign3750_e5563 - 1.0) * if 0.0 == 0.0 && ((var_fn25_calc_iq__beta) as f64).is_finite() && ((var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (var_fn25_calc_iq__beta * ((assign3750_e5557).powf(var_fn25_calc_iq__beta - 1.0) * assign3750_e5557_d_n17)) } } else { (assign3750_e5559 * (var_fn25_calc_iq__beta * (assign3750_e5557_d_n17 / assign3750_e5557))) })) } } else { (assign3750_e5564 * (assign3750_e5563 * (if 0.0 == 0.0 && ((var_fn25_calc_iq__beta) as f64).is_finite() && ((var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (var_fn25_calc_iq__beta * ((assign3750_e5557).powf(var_fn25_calc_iq__beta - 1.0) * assign3750_e5557_d_n17)) } } else { (assign3750_e5559 * (var_fn25_calc_iq__beta * (assign3750_e5557_d_n17 / assign3750_e5557))) } / assign3750_e5560))) } / (assign3750_e5564 * assign3750_e5564))),)
    } else {
        (var_fn25_calc_iq__fsd0, var_fn25_calc_iq__fsd0_dn2, var_fn25_calc_iq__fsd0_dn4, var_fn25_calc_iq__fsd0_dn7, var_fn25_calc_iq__fsd0_dn16, var_fn25_calc_iq__fsd0_dn17,)
    }
};
        var_fn25_calc_iq__fsd0 = assign3750_e5567;
        var_fn25_calc_iq__fsd0_dn2 = assign3750_e5567_d_n2;
        var_fn25_calc_iq__fsd0_dn4 = assign3750_e5567_d_n4;
        var_fn25_calc_iq__fsd0_dn7 = assign3750_e5567_d_n7;
        var_fn25_calc_iq__fsd0_dn16 = assign3750_e5567_d_n16;
        var_fn25_calc_iq__fsd0_dn17 = assign3750_e5567_d_n17;
        var_fn25_calc_iq__fsd0_rv = 0.0;

        let (assign3760_e5573, assign3760_e5573_d_n2, assign3760_e5573_d_n4, assign3760_e5573_d_n7, assign3760_e5573_d_n16, assign3760_e5573_d_n17,) = {
    if (var_guard24 != 0.0) {
        let assign3760_e5571: f64 = (var_fn25_calc_iq__vdsin * var_fn25_calc_iq__fsd0);
        (assign3760_e5571, (var_fn25_calc_iq__vdsin * var_fn25_calc_iq__fsd0_dn2), (var_fn25_calc_iq__vdsin * var_fn25_calc_iq__fsd0_dn4), (var_fn25_calc_iq__vdsin * var_fn25_calc_iq__fsd0_dn7), ((var_fn25_calc_iq__vdsin_dn16 * var_fn25_calc_iq__fsd0) + (var_fn25_calc_iq__vdsin * var_fn25_calc_iq__fsd0_dn16)), ((var_fn25_calc_iq__vdsin_dn17 * var_fn25_calc_iq__fsd0) + (var_fn25_calc_iq__vdsin * var_fn25_calc_iq__fsd0_dn17)),)
    } else {
        (var_fn25_calc_iq__vdx0, var_fn25_calc_iq__vdx0_dn2, var_fn25_calc_iq__vdx0_dn4, var_fn25_calc_iq__vdx0_dn7, var_fn25_calc_iq__vdx0_dn16, var_fn25_calc_iq__vdx0_dn17,)
    }
};
        var_fn25_calc_iq__vdx0 = assign3760_e5573;
        var_fn25_calc_iq__vdx0_dn2 = assign3760_e5573_d_n2;
        var_fn25_calc_iq__vdx0_dn4 = assign3760_e5573_d_n4;
        var_fn25_calc_iq__vdx0_dn7 = assign3760_e5573_d_n7;
        var_fn25_calc_iq__vdx0_dn16 = assign3760_e5573_d_n16;
        var_fn25_calc_iq__vdx0_dn17 = assign3760_e5573_d_n17;
        var_fn25_calc_iq__vdx0_rv = 0.0;

        let (assign3770_e5648, assign3770_e5648_d_n2, assign3770_e5648_d_n4, assign3770_e5648_d_n7, assign3770_e5648_d_n16, assign3770_e5648_d_n17,) = {
    if (var_guard24 != 0.0) {
        let (assign3770_e5638, assign3770_e5638_d_n2, assign3770_e5638_d_n4, assign3770_e5638_d_n7, assign3770_e5638_d_n16, assign3770_e5638_d_n17,) = {
            if (p.p52 != 0.0) {
                let assign3770_e5584: f64 = (-var_fn25_calc_iq__vdsin);
                let assign3770_e5586: f64 = (assign3770_e5584 / var_fn25_calc_iq__vdsat10);
                let assign3770_e5587: f64 = assign3770_e5586;
                let assign3770_e5590: f64 = (-var_fn25_calc_iq__vdsin);
                let assign3770_e5592: f64 = (assign3770_e5590 / var_fn25_calc_iq__vdsat10);
                let assign3770_e5593: f64 = (-assign3770_e5592);
                let assign3770_e5596: f64 = (0.001 / p.p53);
                let assign3770_e5599: f64 = (-var_fn25_calc_iq__vdsin);
                let assign3770_e5601: f64 = (assign3770_e5599 / var_fn25_calc_iq__vdsat10);
                let assign3770_e5602: f64 = (-assign3770_e5601);
                let assign3770_e5603: f64 = (assign3770_e5596 * assign3770_e5602);
                let assign3770_e5604: f64 = (assign3770_e5603).tanh();
                let assign3770_e5605: f64 = (assign3770_e5593 * assign3770_e5604);
                let assign3770_e5606: f64 = (assign3770_e5587 + assign3770_e5605);
                let assign3770_e5607: f64 = (0.5 * assign3770_e5606);
                (assign3770_e5607, (0.5 * ((-((assign3770_e5584 * var_fn25_calc_iq__vdsat10_dn2) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10))) + (((-(-((assign3770_e5590 * var_fn25_calc_iq__vdsat10_dn2) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10)))) * assign3770_e5604) + (assign3770_e5593 * ((assign3770_e5596 * (-(-((assign3770_e5599 * var_fn25_calc_iq__vdsat10_dn2) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10))))) / ((assign3770_e5603).cosh() * (assign3770_e5603).cosh())))))), (0.5 * ((-((assign3770_e5584 * var_fn25_calc_iq__vdsat10_dn4) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10))) + (((-(-((assign3770_e5590 * var_fn25_calc_iq__vdsat10_dn4) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10)))) * assign3770_e5604) + (assign3770_e5593 * ((assign3770_e5596 * (-(-((assign3770_e5599 * var_fn25_calc_iq__vdsat10_dn4) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10))))) / ((assign3770_e5603).cosh() * (assign3770_e5603).cosh())))))), (0.5 * ((-((assign3770_e5584 * var_fn25_calc_iq__vdsat10_dn7) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10))) + (((-(-((assign3770_e5590 * var_fn25_calc_iq__vdsat10_dn7) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10)))) * assign3770_e5604) + (assign3770_e5593 * ((assign3770_e5596 * (-(-((assign3770_e5599 * var_fn25_calc_iq__vdsat10_dn7) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10))))) / ((assign3770_e5603).cosh() * (assign3770_e5603).cosh())))))), (0.5 * (((((-var_fn25_calc_iq__vdsin_dn16) * var_fn25_calc_iq__vdsat10) - (assign3770_e5584 * var_fn25_calc_iq__vdsat10_dn16)) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10)) + (((-((((-var_fn25_calc_iq__vdsin_dn16) * var_fn25_calc_iq__vdsat10) - (assign3770_e5590 * var_fn25_calc_iq__vdsat10_dn16)) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10))) * assign3770_e5604) + (assign3770_e5593 * ((assign3770_e5596 * (-((((-var_fn25_calc_iq__vdsin_dn16) * var_fn25_calc_iq__vdsat10) - (assign3770_e5599 * var_fn25_calc_iq__vdsat10_dn16)) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10)))) / ((assign3770_e5603).cosh() * (assign3770_e5603).cosh())))))), (0.5 * (((((-var_fn25_calc_iq__vdsin_dn17) * var_fn25_calc_iq__vdsat10) - (assign3770_e5584 * var_fn25_calc_iq__vdsat10_dn17)) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10)) + (((-((((-var_fn25_calc_iq__vdsin_dn17) * var_fn25_calc_iq__vdsat10) - (assign3770_e5590 * var_fn25_calc_iq__vdsat10_dn17)) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10))) * assign3770_e5604) + (assign3770_e5593 * ((assign3770_e5596 * (-((((-var_fn25_calc_iq__vdsin_dn17) * var_fn25_calc_iq__vdsat10) - (assign3770_e5599 * var_fn25_calc_iq__vdsat10_dn17)) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10)))) / ((assign3770_e5603).cosh() * (assign3770_e5603).cosh())))))),)
            } else {
                let (assign3770_e5637, assign3770_e5637_d_n2, assign3770_e5637_d_n4, assign3770_e5637_d_n7, assign3770_e5637_d_n16, assign3770_e5637_d_n17,) = {
                    if (p.p52 == 0.0) {
                        let assign3770_e5614: f64 = (-var_fn25_calc_iq__vdsin);
                        let assign3770_e5616: f64 = (assign3770_e5614 / var_fn25_calc_iq__vdsat10);
                        let assign3770_e5617: f64 = assign3770_e5616;
                        let assign3770_e5620: f64 = (-var_fn25_calc_iq__vdsin);
                        let assign3770_e5622: f64 = (assign3770_e5620 / var_fn25_calc_iq__vdsat10);
                        let assign3770_e5623: f64 = (-assign3770_e5622);
                        let assign3770_e5626: f64 = (-var_fn25_calc_iq__vdsin);
                        let assign3770_e5628: f64 = (assign3770_e5626 / var_fn25_calc_iq__vdsat10);
                        let assign3770_e5629: f64 = (-assign3770_e5628);
                        let assign3770_e5630: f64 = (assign3770_e5623 * assign3770_e5629);
                        let assign3770_e5632: f64 = (assign3770_e5630 + p.p53);
                        let assign3770_e5633: f64 = (assign3770_e5632).sqrt();
                        let assign3770_e5634: f64 = (assign3770_e5617 + assign3770_e5633);
                        let assign3770_e5635: f64 = (0.5 * assign3770_e5634);
                        (assign3770_e5635, (0.5 * ((-((assign3770_e5614 * var_fn25_calc_iq__vdsat10_dn2) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10))) + ((((-(-((assign3770_e5620 * var_fn25_calc_iq__vdsat10_dn2) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10)))) * assign3770_e5629) + (assign3770_e5623 * (-(-((assign3770_e5626 * var_fn25_calc_iq__vdsat10_dn2) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10)))))) / (2.0 * assign3770_e5633)))), (0.5 * ((-((assign3770_e5614 * var_fn25_calc_iq__vdsat10_dn4) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10))) + ((((-(-((assign3770_e5620 * var_fn25_calc_iq__vdsat10_dn4) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10)))) * assign3770_e5629) + (assign3770_e5623 * (-(-((assign3770_e5626 * var_fn25_calc_iq__vdsat10_dn4) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10)))))) / (2.0 * assign3770_e5633)))), (0.5 * ((-((assign3770_e5614 * var_fn25_calc_iq__vdsat10_dn7) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10))) + ((((-(-((assign3770_e5620 * var_fn25_calc_iq__vdsat10_dn7) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10)))) * assign3770_e5629) + (assign3770_e5623 * (-(-((assign3770_e5626 * var_fn25_calc_iq__vdsat10_dn7) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10)))))) / (2.0 * assign3770_e5633)))), (0.5 * (((((-var_fn25_calc_iq__vdsin_dn16) * var_fn25_calc_iq__vdsat10) - (assign3770_e5614 * var_fn25_calc_iq__vdsat10_dn16)) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10)) + ((((-((((-var_fn25_calc_iq__vdsin_dn16) * var_fn25_calc_iq__vdsat10) - (assign3770_e5620 * var_fn25_calc_iq__vdsat10_dn16)) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10))) * assign3770_e5629) + (assign3770_e5623 * (-((((-var_fn25_calc_iq__vdsin_dn16) * var_fn25_calc_iq__vdsat10) - (assign3770_e5626 * var_fn25_calc_iq__vdsat10_dn16)) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10))))) / (2.0 * assign3770_e5633)))), (0.5 * (((((-var_fn25_calc_iq__vdsin_dn17) * var_fn25_calc_iq__vdsat10) - (assign3770_e5614 * var_fn25_calc_iq__vdsat10_dn17)) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10)) + ((((-((((-var_fn25_calc_iq__vdsin_dn17) * var_fn25_calc_iq__vdsat10) - (assign3770_e5620 * var_fn25_calc_iq__vdsat10_dn17)) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10))) * assign3770_e5629) + (assign3770_e5623 * (-((((-var_fn25_calc_iq__vdsin_dn17) * var_fn25_calc_iq__vdsat10) - (assign3770_e5626 * var_fn25_calc_iq__vdsat10_dn17)) / (var_fn25_calc_iq__vdsat10 * var_fn25_calc_iq__vdsat10))))) / (2.0 * assign3770_e5633)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign3770_e5637, assign3770_e5637_d_n2, assign3770_e5637_d_n4, assign3770_e5637_d_n7, assign3770_e5637_d_n16, assign3770_e5637_d_n17,)
            }
        };
        let assign3770_e5640: f64 = (assign3770_e5638).powf(var_fn25_calc_iq__beta);
        let assign3770_e5641: f64 = (1.0 + assign3770_e5640);
        let assign3770_e5644: f64 = (1.0 / var_fn25_calc_iq__beta);
        let assign3770_e5645: f64 = (assign3770_e5641).powf(assign3770_e5644);
        let assign3770_e5646: f64 = (1.0 / assign3770_e5645);
        (assign3770_e5646, (-(if 0.0 == 0.0 && ((assign3770_e5644) as f64).is_finite() && ((assign3770_e5644) as f64).fract() == 0.0 { if assign3770_e5644 == 0.0 { 0.0 } else { (assign3770_e5644 * ((assign3770_e5641).powf(assign3770_e5644 - 1.0) * if 0.0 == 0.0 && ((var_fn25_calc_iq__beta) as f64).is_finite() && ((var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (var_fn25_calc_iq__beta * ((assign3770_e5638).powf(var_fn25_calc_iq__beta - 1.0) * assign3770_e5638_d_n2)) } } else { (assign3770_e5640 * (var_fn25_calc_iq__beta * (assign3770_e5638_d_n2 / assign3770_e5638))) })) } } else { (assign3770_e5645 * (assign3770_e5644 * (if 0.0 == 0.0 && ((var_fn25_calc_iq__beta) as f64).is_finite() && ((var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (var_fn25_calc_iq__beta * ((assign3770_e5638).powf(var_fn25_calc_iq__beta - 1.0) * assign3770_e5638_d_n2)) } } else { (assign3770_e5640 * (var_fn25_calc_iq__beta * (assign3770_e5638_d_n2 / assign3770_e5638))) } / assign3770_e5641))) } / (assign3770_e5645 * assign3770_e5645))), (-(if 0.0 == 0.0 && ((assign3770_e5644) as f64).is_finite() && ((assign3770_e5644) as f64).fract() == 0.0 { if assign3770_e5644 == 0.0 { 0.0 } else { (assign3770_e5644 * ((assign3770_e5641).powf(assign3770_e5644 - 1.0) * if 0.0 == 0.0 && ((var_fn25_calc_iq__beta) as f64).is_finite() && ((var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (var_fn25_calc_iq__beta * ((assign3770_e5638).powf(var_fn25_calc_iq__beta - 1.0) * assign3770_e5638_d_n4)) } } else { (assign3770_e5640 * (var_fn25_calc_iq__beta * (assign3770_e5638_d_n4 / assign3770_e5638))) })) } } else { (assign3770_e5645 * (assign3770_e5644 * (if 0.0 == 0.0 && ((var_fn25_calc_iq__beta) as f64).is_finite() && ((var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (var_fn25_calc_iq__beta * ((assign3770_e5638).powf(var_fn25_calc_iq__beta - 1.0) * assign3770_e5638_d_n4)) } } else { (assign3770_e5640 * (var_fn25_calc_iq__beta * (assign3770_e5638_d_n4 / assign3770_e5638))) } / assign3770_e5641))) } / (assign3770_e5645 * assign3770_e5645))), (-(if 0.0 == 0.0 && ((assign3770_e5644) as f64).is_finite() && ((assign3770_e5644) as f64).fract() == 0.0 { if assign3770_e5644 == 0.0 { 0.0 } else { (assign3770_e5644 * ((assign3770_e5641).powf(assign3770_e5644 - 1.0) * if 0.0 == 0.0 && ((var_fn25_calc_iq__beta) as f64).is_finite() && ((var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (var_fn25_calc_iq__beta * ((assign3770_e5638).powf(var_fn25_calc_iq__beta - 1.0) * assign3770_e5638_d_n7)) } } else { (assign3770_e5640 * (var_fn25_calc_iq__beta * (assign3770_e5638_d_n7 / assign3770_e5638))) })) } } else { (assign3770_e5645 * (assign3770_e5644 * (if 0.0 == 0.0 && ((var_fn25_calc_iq__beta) as f64).is_finite() && ((var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (var_fn25_calc_iq__beta * ((assign3770_e5638).powf(var_fn25_calc_iq__beta - 1.0) * assign3770_e5638_d_n7)) } } else { (assign3770_e5640 * (var_fn25_calc_iq__beta * (assign3770_e5638_d_n7 / assign3770_e5638))) } / assign3770_e5641))) } / (assign3770_e5645 * assign3770_e5645))), (-(if 0.0 == 0.0 && ((assign3770_e5644) as f64).is_finite() && ((assign3770_e5644) as f64).fract() == 0.0 { if assign3770_e5644 == 0.0 { 0.0 } else { (assign3770_e5644 * ((assign3770_e5641).powf(assign3770_e5644 - 1.0) * if 0.0 == 0.0 && ((var_fn25_calc_iq__beta) as f64).is_finite() && ((var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (var_fn25_calc_iq__beta * ((assign3770_e5638).powf(var_fn25_calc_iq__beta - 1.0) * assign3770_e5638_d_n16)) } } else { (assign3770_e5640 * (var_fn25_calc_iq__beta * (assign3770_e5638_d_n16 / assign3770_e5638))) })) } } else { (assign3770_e5645 * (assign3770_e5644 * (if 0.0 == 0.0 && ((var_fn25_calc_iq__beta) as f64).is_finite() && ((var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (var_fn25_calc_iq__beta * ((assign3770_e5638).powf(var_fn25_calc_iq__beta - 1.0) * assign3770_e5638_d_n16)) } } else { (assign3770_e5640 * (var_fn25_calc_iq__beta * (assign3770_e5638_d_n16 / assign3770_e5638))) } / assign3770_e5641))) } / (assign3770_e5645 * assign3770_e5645))), (-(if 0.0 == 0.0 && ((assign3770_e5644) as f64).is_finite() && ((assign3770_e5644) as f64).fract() == 0.0 { if assign3770_e5644 == 0.0 { 0.0 } else { (assign3770_e5644 * ((assign3770_e5641).powf(assign3770_e5644 - 1.0) * if 0.0 == 0.0 && ((var_fn25_calc_iq__beta) as f64).is_finite() && ((var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (var_fn25_calc_iq__beta * ((assign3770_e5638).powf(var_fn25_calc_iq__beta - 1.0) * assign3770_e5638_d_n17)) } } else { (assign3770_e5640 * (var_fn25_calc_iq__beta * (assign3770_e5638_d_n17 / assign3770_e5638))) })) } } else { (assign3770_e5645 * (assign3770_e5644 * (if 0.0 == 0.0 && ((var_fn25_calc_iq__beta) as f64).is_finite() && ((var_fn25_calc_iq__beta) as f64).fract() == 0.0 { if var_fn25_calc_iq__beta == 0.0 { 0.0 } else { (var_fn25_calc_iq__beta * ((assign3770_e5638).powf(var_fn25_calc_iq__beta - 1.0) * assign3770_e5638_d_n17)) } } else { (assign3770_e5640 * (var_fn25_calc_iq__beta * (assign3770_e5638_d_n17 / assign3770_e5638))) } / assign3770_e5641))) } / (assign3770_e5645 * assign3770_e5645))),)
    } else {
        (var_fn25_calc_iq__fds0, var_fn25_calc_iq__fds0_dn2, var_fn25_calc_iq__fds0_dn4, var_fn25_calc_iq__fds0_dn7, var_fn25_calc_iq__fds0_dn16, var_fn25_calc_iq__fds0_dn17,)
    }
};
        var_fn25_calc_iq__fds0 = assign3770_e5648;
        var_fn25_calc_iq__fds0_dn2 = assign3770_e5648_d_n2;
        var_fn25_calc_iq__fds0_dn4 = assign3770_e5648_d_n4;
        var_fn25_calc_iq__fds0_dn7 = assign3770_e5648_d_n7;
        var_fn25_calc_iq__fds0_dn16 = assign3770_e5648_d_n16;
        var_fn25_calc_iq__fds0_dn17 = assign3770_e5648_d_n17;
        var_fn25_calc_iq__fds0_rv = 0.0;

        let (assign3780_e5655, assign3780_e5655_d_n2, assign3780_e5655_d_n4, assign3780_e5655_d_n7, assign3780_e5655_d_n16, assign3780_e5655_d_n17,) = {
    if (var_guard24 != 0.0) {
        let assign3780_e5651: f64 = (-var_fn25_calc_iq__vdsin);
        let assign3780_e5653: f64 = (assign3780_e5651 * var_fn25_calc_iq__fds0);
        (assign3780_e5653, (assign3780_e5651 * var_fn25_calc_iq__fds0_dn2), (assign3780_e5651 * var_fn25_calc_iq__fds0_dn4), (assign3780_e5651 * var_fn25_calc_iq__fds0_dn7), (((-var_fn25_calc_iq__vdsin_dn16) * var_fn25_calc_iq__fds0) + (assign3780_e5651 * var_fn25_calc_iq__fds0_dn16)), (((-var_fn25_calc_iq__vdsin_dn17) * var_fn25_calc_iq__fds0) + (assign3780_e5651 * var_fn25_calc_iq__fds0_dn17)),)
    } else {
        (var_fn25_calc_iq__vsx0, var_fn25_calc_iq__vsx0_dn2, var_fn25_calc_iq__vsx0_dn4, var_fn25_calc_iq__vsx0_dn7, var_fn25_calc_iq__vsx0_dn16, var_fn25_calc_iq__vsx0_dn17,)
    }
};
        var_fn25_calc_iq__vsx0 = assign3780_e5655;
        var_fn25_calc_iq__vsx0_dn2 = assign3780_e5655_d_n2;
        var_fn25_calc_iq__vsx0_dn4 = assign3780_e5655_d_n4;
        var_fn25_calc_iq__vsx0_dn7 = assign3780_e5655_d_n7;
        var_fn25_calc_iq__vsx0_dn16 = assign3780_e5655_d_n16;
        var_fn25_calc_iq__vsx0_dn17 = assign3780_e5655_d_n17;
        var_fn25_calc_iq__vsx0_rv = 0.0;

        let (assign3790_e5663, assign3790_e5663_d_n2, assign3790_e5663_d_n4, assign3790_e5663_d_n7, assign3790_e5663_d_n16, assign3790_e5663_d_n17,) = {
    if (var_guard24 != 0.0) {
        let assign3790_e5659: f64 = (var_fn25_calc_iq__vgsin - var_fn25_calc_iq__myarg0);
        let assign3790_e5661: f64 = (assign3790_e5659 / var_fn25_calc_iq__alpha_phit);
        (assign3790_e5661, (var_fn25_calc_iq__vgsin_dn2 / var_fn25_calc_iq__alpha_phit), ((((-var_fn25_calc_iq__myarg0_dn4) * var_fn25_calc_iq__alpha_phit) - (assign3790_e5659 * var_fn25_calc_iq__alpha_phit_dn4)) / (var_fn25_calc_iq__alpha_phit * var_fn25_calc_iq__alpha_phit)), (var_fn25_calc_iq__vgsin_dn7 / var_fn25_calc_iq__alpha_phit), (var_fn25_calc_iq__vgsin_dn16 / var_fn25_calc_iq__alpha_phit), 0.0,)
    } else {
        (var_fn25_calc_iq__exparg0, var_fn25_calc_iq__exparg0_dn2, var_fn25_calc_iq__exparg0_dn4, var_fn25_calc_iq__exparg0_dn7, var_fn25_calc_iq__exparg0_dn16, var_fn25_calc_iq__exparg0_dn17,)
    }
};
        var_fn25_calc_iq__exparg0 = assign3790_e5663;
        var_fn25_calc_iq__exparg0_dn2 = assign3790_e5663_d_n2;
        var_fn25_calc_iq__exparg0_dn4 = assign3790_e5663_d_n4;
        var_fn25_calc_iq__exparg0_dn7 = assign3790_e5663_d_n7;
        var_fn25_calc_iq__exparg0_dn16 = assign3790_e5663_d_n16;
        var_fn25_calc_iq__exparg0_dn17 = assign3790_e5663_d_n17;
        var_fn25_calc_iq__exparg0_rv = 0.0;

        let assign3800_e5666: f64 = if var_fn25_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        var_guard43 = assign3800_e5666;
        var_guard43_rv = 0.0;

        let (assign3810_e5672, assign3810_e5672_d_n2, assign3810_e5672_d_n4, assign3810_e5672_d_n7, assign3810_e5672_d_n16, assign3810_e5672_d_n17,) = {
    if ((var_guard24 != 0.0) && (var_guard43 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__ffs0, var_fn25_calc_iq__ffs0_dn2, var_fn25_calc_iq__ffs0_dn4, var_fn25_calc_iq__ffs0_dn7, var_fn25_calc_iq__ffs0_dn16, var_fn25_calc_iq__ffs0_dn17,)
    }
};
        var_fn25_calc_iq__ffs0 = assign3810_e5672;
        var_fn25_calc_iq__ffs0_dn2 = assign3810_e5672_d_n2;
        var_fn25_calc_iq__ffs0_dn4 = assign3810_e5672_d_n4;
        var_fn25_calc_iq__ffs0_dn7 = assign3810_e5672_d_n7;
        var_fn25_calc_iq__ffs0_dn16 = assign3810_e5672_d_n16;
        var_fn25_calc_iq__ffs0_dn17 = assign3810_e5672_d_n17;
        var_fn25_calc_iq__ffs0_rv = 0.0;

        let assign3820_e5675: f64 = (-50.0);
        let assign3820_e5676: f64 = if var_fn25_calc_iq__exparg0 < assign3820_e5675 { 1.0 } else { 0.0 };
        var_guard44 = assign3820_e5676;
        var_guard44_rv = 0.0;

        let (assign3830_e5685, assign3830_e5685_d_n2, assign3830_e5685_d_n4, assign3830_e5685_d_n7, assign3830_e5685_d_n16, assign3830_e5685_d_n17,) = {
    if (((var_guard24 != 0.0) && (var_guard43 == 0.0)) && (var_guard44 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__ffs0, var_fn25_calc_iq__ffs0_dn2, var_fn25_calc_iq__ffs0_dn4, var_fn25_calc_iq__ffs0_dn7, var_fn25_calc_iq__ffs0_dn16, var_fn25_calc_iq__ffs0_dn17,)
    }
};
        var_fn25_calc_iq__ffs0 = assign3830_e5685;
        var_fn25_calc_iq__ffs0_dn2 = assign3830_e5685_d_n2;
        var_fn25_calc_iq__ffs0_dn4 = assign3830_e5685_d_n4;
        var_fn25_calc_iq__ffs0_dn7 = assign3830_e5685_d_n7;
        var_fn25_calc_iq__ffs0_dn16 = assign3830_e5685_d_n16;
        var_fn25_calc_iq__ffs0_dn17 = assign3830_e5685_d_n17;
        var_fn25_calc_iq__ffs0_rv = 0.0;

        *var_fn25_calc_iq__eta0_slot = var_fn25_calc_iq__eta0;
        *var_fn25_calc_iq__eta0_dn16_slot = var_fn25_calc_iq__eta0_dn16;
        *var_fn25_calc_iq__eta0_dn17_slot = var_fn25_calc_iq__eta0_dn17;
        *var_fn25_calc_iq__eta0_dn2_slot = var_fn25_calc_iq__eta0_dn2;
        *var_fn25_calc_iq__eta0_dn4_slot = var_fn25_calc_iq__eta0_dn4;
        *var_fn25_calc_iq__eta0_dn7_slot = var_fn25_calc_iq__eta0_dn7;
        *var_fn25_calc_iq__eta0_rv_slot = var_fn25_calc_iq__eta0_rv;
        *var_fn25_calc_iq__exparg0_slot = var_fn25_calc_iq__exparg0;
        *var_fn25_calc_iq__exparg0_dn16_slot = var_fn25_calc_iq__exparg0_dn16;
        *var_fn25_calc_iq__exparg0_dn17_slot = var_fn25_calc_iq__exparg0_dn17;
        *var_fn25_calc_iq__exparg0_dn2_slot = var_fn25_calc_iq__exparg0_dn2;
        *var_fn25_calc_iq__exparg0_dn4_slot = var_fn25_calc_iq__exparg0_dn4;
        *var_fn25_calc_iq__exparg0_dn7_slot = var_fn25_calc_iq__exparg0_dn7;
        *var_fn25_calc_iq__exparg0_rv_slot = var_fn25_calc_iq__exparg0_rv;
        *var_fn25_calc_iq__fds0_slot = var_fn25_calc_iq__fds0;
        *var_fn25_calc_iq__fds0_dn16_slot = var_fn25_calc_iq__fds0_dn16;
        *var_fn25_calc_iq__fds0_dn17_slot = var_fn25_calc_iq__fds0_dn17;
        *var_fn25_calc_iq__fds0_dn2_slot = var_fn25_calc_iq__fds0_dn2;
        *var_fn25_calc_iq__fds0_dn4_slot = var_fn25_calc_iq__fds0_dn4;
        *var_fn25_calc_iq__fds0_dn7_slot = var_fn25_calc_iq__fds0_dn7;
        *var_fn25_calc_iq__fds0_rv_slot = var_fn25_calc_iq__fds0_rv;
        *var_fn25_calc_iq__ff0_slot = var_fn25_calc_iq__ff0;
        *var_fn25_calc_iq__ff0_dn16_slot = var_fn25_calc_iq__ff0_dn16;
        *var_fn25_calc_iq__ff0_dn17_slot = var_fn25_calc_iq__ff0_dn17;
        *var_fn25_calc_iq__ff0_dn2_slot = var_fn25_calc_iq__ff0_dn2;
        *var_fn25_calc_iq__ff0_dn4_slot = var_fn25_calc_iq__ff0_dn4;
        *var_fn25_calc_iq__ff0_dn7_slot = var_fn25_calc_iq__ff0_dn7;
        *var_fn25_calc_iq__ff0_rv_slot = var_fn25_calc_iq__ff0_rv;
        *var_fn25_calc_iq__ffs0_slot = var_fn25_calc_iq__ffs0;
        *var_fn25_calc_iq__ffs0_dn16_slot = var_fn25_calc_iq__ffs0_dn16;
        *var_fn25_calc_iq__ffs0_dn17_slot = var_fn25_calc_iq__ffs0_dn17;
        *var_fn25_calc_iq__ffs0_dn2_slot = var_fn25_calc_iq__ffs0_dn2;
        *var_fn25_calc_iq__ffs0_dn4_slot = var_fn25_calc_iq__ffs0_dn4;
        *var_fn25_calc_iq__ffs0_dn7_slot = var_fn25_calc_iq__ffs0_dn7;
        *var_fn25_calc_iq__ffs0_rv_slot = var_fn25_calc_iq__ffs0_rv;
        *var_fn25_calc_iq__fsd0_slot = var_fn25_calc_iq__fsd0;
        *var_fn25_calc_iq__fsd0_dn16_slot = var_fn25_calc_iq__fsd0_dn16;
        *var_fn25_calc_iq__fsd0_dn17_slot = var_fn25_calc_iq__fsd0_dn17;
        *var_fn25_calc_iq__fsd0_dn2_slot = var_fn25_calc_iq__fsd0_dn2;
        *var_fn25_calc_iq__fsd0_dn4_slot = var_fn25_calc_iq__fsd0_dn4;
        *var_fn25_calc_iq__fsd0_dn7_slot = var_fn25_calc_iq__fsd0_dn7;
        *var_fn25_calc_iq__fsd0_rv_slot = var_fn25_calc_iq__fsd0_rv;
        *var_fn25_calc_iq__muf0_slot = var_fn25_calc_iq__muf0;
        *var_fn25_calc_iq__muf0_dn4_slot = var_fn25_calc_iq__muf0_dn4;
        *var_fn25_calc_iq__muf0_rv_slot = var_fn25_calc_iq__muf0_rv;
        *var_fn25_calc_iq__qinvv0_slot = var_fn25_calc_iq__qinvv0;
        *var_fn25_calc_iq__qinvv0_dn16_slot = var_fn25_calc_iq__qinvv0_dn16;
        *var_fn25_calc_iq__qinvv0_dn17_slot = var_fn25_calc_iq__qinvv0_dn17;
        *var_fn25_calc_iq__qinvv0_dn2_slot = var_fn25_calc_iq__qinvv0_dn2;
        *var_fn25_calc_iq__qinvv0_dn4_slot = var_fn25_calc_iq__qinvv0_dn4;
        *var_fn25_calc_iq__qinvv0_dn7_slot = var_fn25_calc_iq__qinvv0_dn7;
        *var_fn25_calc_iq__qinvv0_rv_slot = var_fn25_calc_iq__qinvv0_rv;
        *var_fn25_calc_iq__vdsat10_slot = var_fn25_calc_iq__vdsat10;
        *var_fn25_calc_iq__vdsat10_dn16_slot = var_fn25_calc_iq__vdsat10_dn16;
        *var_fn25_calc_iq__vdsat10_dn17_slot = var_fn25_calc_iq__vdsat10_dn17;
        *var_fn25_calc_iq__vdsat10_dn2_slot = var_fn25_calc_iq__vdsat10_dn2;
        *var_fn25_calc_iq__vdsat10_dn4_slot = var_fn25_calc_iq__vdsat10_dn4;
        *var_fn25_calc_iq__vdsat10_dn7_slot = var_fn25_calc_iq__vdsat10_dn7;
        *var_fn25_calc_iq__vdsat10_rv_slot = var_fn25_calc_iq__vdsat10_rv;
        *var_fn25_calc_iq__vdsats0_slot = var_fn25_calc_iq__vdsats0;
        *var_fn25_calc_iq__vdsats0_dn4_slot = var_fn25_calc_iq__vdsats0_dn4;
        *var_fn25_calc_iq__vdsats0_rv_slot = var_fn25_calc_iq__vdsats0_rv;
        *var_fn25_calc_iq__vdsats10_slot = var_fn25_calc_iq__vdsats10;
        *var_fn25_calc_iq__vdsats10_dn16_slot = var_fn25_calc_iq__vdsats10_dn16;
        *var_fn25_calc_iq__vdsats10_dn17_slot = var_fn25_calc_iq__vdsats10_dn17;
        *var_fn25_calc_iq__vdsats10_dn2_slot = var_fn25_calc_iq__vdsats10_dn2;
        *var_fn25_calc_iq__vdsats10_dn4_slot = var_fn25_calc_iq__vdsats10_dn4;
        *var_fn25_calc_iq__vdsats10_dn7_slot = var_fn25_calc_iq__vdsats10_dn7;
        *var_fn25_calc_iq__vdsats10_rv_slot = var_fn25_calc_iq__vdsats10_rv;
        *var_fn25_calc_iq__vdx0_slot = var_fn25_calc_iq__vdx0;
        *var_fn25_calc_iq__vdx0_dn16_slot = var_fn25_calc_iq__vdx0_dn16;
        *var_fn25_calc_iq__vdx0_dn17_slot = var_fn25_calc_iq__vdx0_dn17;
        *var_fn25_calc_iq__vdx0_dn2_slot = var_fn25_calc_iq__vdx0_dn2;
        *var_fn25_calc_iq__vdx0_dn4_slot = var_fn25_calc_iq__vdx0_dn4;
        *var_fn25_calc_iq__vdx0_dn7_slot = var_fn25_calc_iq__vdx0_dn7;
        *var_fn25_calc_iq__vdx0_rv_slot = var_fn25_calc_iq__vdx0_rv;
        *var_fn25_calc_iq__vsx0_slot = var_fn25_calc_iq__vsx0;
        *var_fn25_calc_iq__vsx0_dn16_slot = var_fn25_calc_iq__vsx0_dn16;
        *var_fn25_calc_iq__vsx0_dn17_slot = var_fn25_calc_iq__vsx0_dn17;
        *var_fn25_calc_iq__vsx0_dn2_slot = var_fn25_calc_iq__vsx0_dn2;
        *var_fn25_calc_iq__vsx0_dn4_slot = var_fn25_calc_iq__vsx0_dn4;
        *var_fn25_calc_iq__vsx0_dn7_slot = var_fn25_calc_iq__vsx0_dn7;
        *var_fn25_calc_iq__vsx0_rv_slot = var_fn25_calc_iq__vsx0_rv;
        *var_fn25_calc_iq__vx0_slot = var_fn25_calc_iq__vx0;
        *var_fn25_calc_iq__vx0_dn4_slot = var_fn25_calc_iq__vx0_dn4;
        *var_fn25_calc_iq__vx0_rv_slot = var_fn25_calc_iq__vx0_rv;
        *var_guard39_slot = var_guard39;
        *var_guard39_rv_slot = var_guard39_rv;
        *var_guard40_slot = var_guard40;
        *var_guard40_rv_slot = var_guard40_rv;
        *var_guard41_slot = var_guard41;
        *var_guard41_rv_slot = var_guard41_rv;
        *var_guard42_slot = var_guard42;
        *var_guard42_rv_slot = var_guard42_rv;
        *var_guard43_slot = var_guard43;
        *var_guard43_rv_slot = var_guard43_rv;
        *var_guard44_slot = var_guard44;
        *var_guard44_rv_slot = var_guard44_rv;
    }

    pub(super) fn stamp_reactive_block_9(
        p: &Parameters,
        var_fn25_calc_iq__alpha_phit: f64,
        var_fn25_calc_iq__alpha_phit_dn4: f64,
        var_fn25_calc_iq__lin: f64,
        var_fn25_calc_iq__myarg0: f64,
        var_fn25_calc_iq__myarg0_dn4: f64,
        var_fn25_calc_iq__ngf: f64,
        var_fn25_calc_iq__qcbflag: f64,
        var_fn25_calc_iq__qref0: f64,
        var_fn25_calc_iq__qref0_dn4: f64,
        var_fn25_calc_iq__trapfracdl: f64,
        var_fn25_calc_iq__two_n_phit0: f64,
        var_fn25_calc_iq__two_n_phit0_dn4: f64,
        var_fn25_calc_iq__type: f64,
        var_fn25_calc_iq__vcin: f64,
        var_fn25_calc_iq__vcin_dn16: f64,
        var_fn25_calc_iq__vcin_dn2: f64,
        var_fn25_calc_iq__vcin_dn7: f64,
        var_fn25_calc_iq__vdx0: f64,
        var_fn25_calc_iq__vdx0_dn16: f64,
        var_fn25_calc_iq__vdx0_dn17: f64,
        var_fn25_calc_iq__vdx0_dn2: f64,
        var_fn25_calc_iq__vdx0_dn4: f64,
        var_fn25_calc_iq__vdx0_dn7: f64,
        var_fn25_calc_iq__vgdin: f64,
        var_fn25_calc_iq__vgdin_dn16: f64,
        var_fn25_calc_iq__vgdin_dn17: f64,
        var_fn25_calc_iq__vgdin_dn2: f64,
        var_fn25_calc_iq__vgdin_dn7: f64,
        var_fn25_calc_iq__vgsin: f64,
        var_fn25_calc_iq__vgsin_dn16: f64,
        var_fn25_calc_iq__vgsin_dn2: f64,
        var_fn25_calc_iq__vgsin_dn7: f64,
        var_fn25_calc_iq__vsx0: f64,
        var_fn25_calc_iq__vsx0_dn16: f64,
        var_fn25_calc_iq__vsx0_dn17: f64,
        var_fn25_calc_iq__vsx0_dn2: f64,
        var_fn25_calc_iq__vsx0_dn4: f64,
        var_fn25_calc_iq__vsx0_dn7: f64,
        var_fn25_calc_iq__vtof: f64,
        var_fn25_calc_iq__vtof_dn4: f64,
        var_fn25_calc_iq__w: f64,
        var_guard24: f64,
        var_guard43: f64,
        var_guard44: f64,
        var_fn25_calc_iq__etac_slot: &mut f64,
        var_fn25_calc_iq__etac_dn16_slot: &mut f64,
        var_fn25_calc_iq__etac_dn2_slot: &mut f64,
        var_fn25_calc_iq__etac_dn4_slot: &mut f64,
        var_fn25_calc_iq__etac_dn7_slot: &mut f64,
        var_fn25_calc_iq__etac_rv_slot: &mut f64,
        var_fn25_calc_iq__etad0_slot: &mut f64,
        var_fn25_calc_iq__etad0_dn16_slot: &mut f64,
        var_fn25_calc_iq__etad0_dn17_slot: &mut f64,
        var_fn25_calc_iq__etad0_dn2_slot: &mut f64,
        var_fn25_calc_iq__etad0_dn4_slot: &mut f64,
        var_fn25_calc_iq__etad0_dn7_slot: &mut f64,
        var_fn25_calc_iq__etad0_rv_slot: &mut f64,
        var_fn25_calc_iq__etas0_slot: &mut f64,
        var_fn25_calc_iq__etas0_dn16_slot: &mut f64,
        var_fn25_calc_iq__etas0_dn17_slot: &mut f64,
        var_fn25_calc_iq__etas0_dn2_slot: &mut f64,
        var_fn25_calc_iq__etas0_dn4_slot: &mut f64,
        var_fn25_calc_iq__etas0_dn7_slot: &mut f64,
        var_fn25_calc_iq__etas0_rv_slot: &mut f64,
        var_fn25_calc_iq__exparg_slot: &mut f64,
        var_fn25_calc_iq__exparg0_slot: &mut f64,
        var_fn25_calc_iq__exparg0_dn16_slot: &mut f64,
        var_fn25_calc_iq__exparg0_dn17_slot: &mut f64,
        var_fn25_calc_iq__exparg0_dn2_slot: &mut f64,
        var_fn25_calc_iq__exparg0_dn4_slot: &mut f64,
        var_fn25_calc_iq__exparg0_dn7_slot: &mut f64,
        var_fn25_calc_iq__exparg0_rv_slot: &mut f64,
        var_fn25_calc_iq__exparg_dn16_slot: &mut f64,
        var_fn25_calc_iq__exparg_dn17_slot: &mut f64,
        var_fn25_calc_iq__exparg_dn2_slot: &mut f64,
        var_fn25_calc_iq__exparg_dn3_slot: &mut f64,
        var_fn25_calc_iq__exparg_dn4_slot: &mut f64,
        var_fn25_calc_iq__exparg_dn7_slot: &mut f64,
        var_fn25_calc_iq__exparg_rv_slot: &mut f64,
        var_fn25_calc_iq__ffd0_slot: &mut f64,
        var_fn25_calc_iq__ffd0_dn16_slot: &mut f64,
        var_fn25_calc_iq__ffd0_dn17_slot: &mut f64,
        var_fn25_calc_iq__ffd0_dn2_slot: &mut f64,
        var_fn25_calc_iq__ffd0_dn4_slot: &mut f64,
        var_fn25_calc_iq__ffd0_dn7_slot: &mut f64,
        var_fn25_calc_iq__ffd0_rv_slot: &mut f64,
        var_fn25_calc_iq__ffs0_slot: &mut f64,
        var_fn25_calc_iq__ffs0_dn16_slot: &mut f64,
        var_fn25_calc_iq__ffs0_dn17_slot: &mut f64,
        var_fn25_calc_iq__ffs0_dn2_slot: &mut f64,
        var_fn25_calc_iq__ffs0_dn4_slot: &mut f64,
        var_fn25_calc_iq__ffs0_dn7_slot: &mut f64,
        var_fn25_calc_iq__ffs0_rv_slot: &mut f64,
        var_fn25_calc_iq__qd_slot: &mut f64,
        var_fn25_calc_iq__qd1_slot: &mut f64,
        var_fn25_calc_iq__qd1_dn16_slot: &mut f64,
        var_fn25_calc_iq__qd1_dn17_slot: &mut f64,
        var_fn25_calc_iq__qd1_dn2_slot: &mut f64,
        var_fn25_calc_iq__qd1_dn4_slot: &mut f64,
        var_fn25_calc_iq__qd1_dn7_slot: &mut f64,
        var_fn25_calc_iq__qd1_rv_slot: &mut f64,
        var_fn25_calc_iq__qd2_slot: &mut f64,
        var_fn25_calc_iq__qd2_dn16_slot: &mut f64,
        var_fn25_calc_iq__qd2_dn17_slot: &mut f64,
        var_fn25_calc_iq__qd2_dn2_slot: &mut f64,
        var_fn25_calc_iq__qd2_dn4_slot: &mut f64,
        var_fn25_calc_iq__qd2_dn7_slot: &mut f64,
        var_fn25_calc_iq__qd2_rv_slot: &mut f64,
        var_fn25_calc_iq__qd3_slot: &mut f64,
        var_fn25_calc_iq__qd3_dn16_slot: &mut f64,
        var_fn25_calc_iq__qd3_dn17_slot: &mut f64,
        var_fn25_calc_iq__qd3_dn2_slot: &mut f64,
        var_fn25_calc_iq__qd3_dn4_slot: &mut f64,
        var_fn25_calc_iq__qd3_dn7_slot: &mut f64,
        var_fn25_calc_iq__qd3_rv_slot: &mut f64,
        var_fn25_calc_iq__qd_dn16_slot: &mut f64,
        var_fn25_calc_iq__qd_dn17_slot: &mut f64,
        var_fn25_calc_iq__qd_dn2_slot: &mut f64,
        var_fn25_calc_iq__qd_dn4_slot: &mut f64,
        var_fn25_calc_iq__qd_dn7_slot: &mut f64,
        var_fn25_calc_iq__qd_rv_slot: &mut f64,
        var_fn25_calc_iq__qgdout_slot: &mut f64,
        var_fn25_calc_iq__qgdout_dn16_slot: &mut f64,
        var_fn25_calc_iq__qgdout_dn17_slot: &mut f64,
        var_fn25_calc_iq__qgdout_dn2_slot: &mut f64,
        var_fn25_calc_iq__qgdout_dn4_slot: &mut f64,
        var_fn25_calc_iq__qgdout_dn7_slot: &mut f64,
        var_fn25_calc_iq__qgdout_rv_slot: &mut f64,
        var_fn25_calc_iq__qgsout_slot: &mut f64,
        var_fn25_calc_iq__qgsout_dn16_slot: &mut f64,
        var_fn25_calc_iq__qgsout_dn17_slot: &mut f64,
        var_fn25_calc_iq__qgsout_dn2_slot: &mut f64,
        var_fn25_calc_iq__qgsout_dn4_slot: &mut f64,
        var_fn25_calc_iq__qgsout_dn7_slot: &mut f64,
        var_fn25_calc_iq__qgsout_rv_slot: &mut f64,
        var_fn25_calc_iq__qinvd0_slot: &mut f64,
        var_fn25_calc_iq__qinvd0_dn16_slot: &mut f64,
        var_fn25_calc_iq__qinvd0_dn17_slot: &mut f64,
        var_fn25_calc_iq__qinvd0_dn2_slot: &mut f64,
        var_fn25_calc_iq__qinvd0_dn4_slot: &mut f64,
        var_fn25_calc_iq__qinvd0_dn7_slot: &mut f64,
        var_fn25_calc_iq__qinvd0_rv_slot: &mut f64,
        var_fn25_calc_iq__qinvdd_slot: &mut f64,
        var_fn25_calc_iq__qinvdd_dn16_slot: &mut f64,
        var_fn25_calc_iq__qinvdd_dn17_slot: &mut f64,
        var_fn25_calc_iq__qinvdd_dn2_slot: &mut f64,
        var_fn25_calc_iq__qinvdd_dn4_slot: &mut f64,
        var_fn25_calc_iq__qinvdd_dn7_slot: &mut f64,
        var_fn25_calc_iq__qinvdd_rv_slot: &mut f64,
        var_fn25_calc_iq__qinvs0_slot: &mut f64,
        var_fn25_calc_iq__qinvs0_dn16_slot: &mut f64,
        var_fn25_calc_iq__qinvs0_dn17_slot: &mut f64,
        var_fn25_calc_iq__qinvs0_dn2_slot: &mut f64,
        var_fn25_calc_iq__qinvs0_dn4_slot: &mut f64,
        var_fn25_calc_iq__qinvs0_dn7_slot: &mut f64,
        var_fn25_calc_iq__qinvs0_rv_slot: &mut f64,
        var_fn25_calc_iq__qs_slot: &mut f64,
        var_fn25_calc_iq__qs2_slot: &mut f64,
        var_fn25_calc_iq__qs2_dn16_slot: &mut f64,
        var_fn25_calc_iq__qs2_dn17_slot: &mut f64,
        var_fn25_calc_iq__qs2_dn2_slot: &mut f64,
        var_fn25_calc_iq__qs2_dn4_slot: &mut f64,
        var_fn25_calc_iq__qs2_dn7_slot: &mut f64,
        var_fn25_calc_iq__qs2_rv_slot: &mut f64,
        var_fn25_calc_iq__qs3_slot: &mut f64,
        var_fn25_calc_iq__qs3_dn16_slot: &mut f64,
        var_fn25_calc_iq__qs3_dn17_slot: &mut f64,
        var_fn25_calc_iq__qs3_dn2_slot: &mut f64,
        var_fn25_calc_iq__qs3_dn4_slot: &mut f64,
        var_fn25_calc_iq__qs3_dn7_slot: &mut f64,
        var_fn25_calc_iq__qs3_rv_slot: &mut f64,
        var_fn25_calc_iq__qs_dn16_slot: &mut f64,
        var_fn25_calc_iq__qs_dn17_slot: &mut f64,
        var_fn25_calc_iq__qs_dn2_slot: &mut f64,
        var_fn25_calc_iq__qs_dn4_slot: &mut f64,
        var_fn25_calc_iq__qs_dn7_slot: &mut f64,
        var_fn25_calc_iq__qs_rv_slot: &mut f64,
        var_fn25_calc_iq__qsqd_slot: &mut f64,
        var_fn25_calc_iq__qsqd_dn16_slot: &mut f64,
        var_fn25_calc_iq__qsqd_dn17_slot: &mut f64,
        var_fn25_calc_iq__qsqd_dn2_slot: &mut f64,
        var_fn25_calc_iq__qsqd_dn4_slot: &mut f64,
        var_fn25_calc_iq__qsqd_dn7_slot: &mut f64,
        var_fn25_calc_iq__qsqd_rv_slot: &mut f64,
        var_guard45_slot: &mut f64,
        var_guard45_rv_slot: &mut f64,
        var_guard46_slot: &mut f64,
        var_guard46_rv_slot: &mut f64,
        var_guard47_slot: &mut f64,
        var_guard47_rv_slot: &mut f64,
        var_guard48_slot: &mut f64,
        var_guard48_rv_slot: &mut f64,
        var_guard49_slot: &mut f64,
        var_guard49_rv_slot: &mut f64,
        var_guard50_slot: &mut f64,
        var_guard50_rv_slot: &mut f64,
        var_guard51_slot: &mut f64,
        var_guard51_rv_slot: &mut f64,
        var_guard52_slot: &mut f64,
        var_guard52_rv_slot: &mut f64,
    ) {
        let mut var_fn25_calc_iq__etac: f64 = *var_fn25_calc_iq__etac_slot;
        let mut var_fn25_calc_iq__etac_dn16: f64 = *var_fn25_calc_iq__etac_dn16_slot;
        let mut var_fn25_calc_iq__etac_dn2: f64 = *var_fn25_calc_iq__etac_dn2_slot;
        let mut var_fn25_calc_iq__etac_dn4: f64 = *var_fn25_calc_iq__etac_dn4_slot;
        let mut var_fn25_calc_iq__etac_dn7: f64 = *var_fn25_calc_iq__etac_dn7_slot;
        let mut var_fn25_calc_iq__etac_rv: f64 = *var_fn25_calc_iq__etac_rv_slot;
        let mut var_fn25_calc_iq__etad0: f64 = *var_fn25_calc_iq__etad0_slot;
        let mut var_fn25_calc_iq__etad0_dn16: f64 = *var_fn25_calc_iq__etad0_dn16_slot;
        let mut var_fn25_calc_iq__etad0_dn17: f64 = *var_fn25_calc_iq__etad0_dn17_slot;
        let mut var_fn25_calc_iq__etad0_dn2: f64 = *var_fn25_calc_iq__etad0_dn2_slot;
        let mut var_fn25_calc_iq__etad0_dn4: f64 = *var_fn25_calc_iq__etad0_dn4_slot;
        let mut var_fn25_calc_iq__etad0_dn7: f64 = *var_fn25_calc_iq__etad0_dn7_slot;
        let mut var_fn25_calc_iq__etad0_rv: f64 = *var_fn25_calc_iq__etad0_rv_slot;
        let mut var_fn25_calc_iq__etas0: f64 = *var_fn25_calc_iq__etas0_slot;
        let mut var_fn25_calc_iq__etas0_dn16: f64 = *var_fn25_calc_iq__etas0_dn16_slot;
        let mut var_fn25_calc_iq__etas0_dn17: f64 = *var_fn25_calc_iq__etas0_dn17_slot;
        let mut var_fn25_calc_iq__etas0_dn2: f64 = *var_fn25_calc_iq__etas0_dn2_slot;
        let mut var_fn25_calc_iq__etas0_dn4: f64 = *var_fn25_calc_iq__etas0_dn4_slot;
        let mut var_fn25_calc_iq__etas0_dn7: f64 = *var_fn25_calc_iq__etas0_dn7_slot;
        let mut var_fn25_calc_iq__etas0_rv: f64 = *var_fn25_calc_iq__etas0_rv_slot;
        let mut var_fn25_calc_iq__exparg: f64 = *var_fn25_calc_iq__exparg_slot;
        let mut var_fn25_calc_iq__exparg0: f64 = *var_fn25_calc_iq__exparg0_slot;
        let mut var_fn25_calc_iq__exparg0_dn16: f64 = *var_fn25_calc_iq__exparg0_dn16_slot;
        let mut var_fn25_calc_iq__exparg0_dn17: f64 = *var_fn25_calc_iq__exparg0_dn17_slot;
        let mut var_fn25_calc_iq__exparg0_dn2: f64 = *var_fn25_calc_iq__exparg0_dn2_slot;
        let mut var_fn25_calc_iq__exparg0_dn4: f64 = *var_fn25_calc_iq__exparg0_dn4_slot;
        let mut var_fn25_calc_iq__exparg0_dn7: f64 = *var_fn25_calc_iq__exparg0_dn7_slot;
        let mut var_fn25_calc_iq__exparg0_rv: f64 = *var_fn25_calc_iq__exparg0_rv_slot;
        let mut var_fn25_calc_iq__exparg_dn16: f64 = *var_fn25_calc_iq__exparg_dn16_slot;
        let mut var_fn25_calc_iq__exparg_dn17: f64 = *var_fn25_calc_iq__exparg_dn17_slot;
        let mut var_fn25_calc_iq__exparg_dn2: f64 = *var_fn25_calc_iq__exparg_dn2_slot;
        let mut var_fn25_calc_iq__exparg_dn3: f64 = *var_fn25_calc_iq__exparg_dn3_slot;
        let mut var_fn25_calc_iq__exparg_dn4: f64 = *var_fn25_calc_iq__exparg_dn4_slot;
        let mut var_fn25_calc_iq__exparg_dn7: f64 = *var_fn25_calc_iq__exparg_dn7_slot;
        let mut var_fn25_calc_iq__exparg_rv: f64 = *var_fn25_calc_iq__exparg_rv_slot;
        let mut var_fn25_calc_iq__ffd0: f64 = *var_fn25_calc_iq__ffd0_slot;
        let mut var_fn25_calc_iq__ffd0_dn16: f64 = *var_fn25_calc_iq__ffd0_dn16_slot;
        let mut var_fn25_calc_iq__ffd0_dn17: f64 = *var_fn25_calc_iq__ffd0_dn17_slot;
        let mut var_fn25_calc_iq__ffd0_dn2: f64 = *var_fn25_calc_iq__ffd0_dn2_slot;
        let mut var_fn25_calc_iq__ffd0_dn4: f64 = *var_fn25_calc_iq__ffd0_dn4_slot;
        let mut var_fn25_calc_iq__ffd0_dn7: f64 = *var_fn25_calc_iq__ffd0_dn7_slot;
        let mut var_fn25_calc_iq__ffd0_rv: f64 = *var_fn25_calc_iq__ffd0_rv_slot;
        let mut var_fn25_calc_iq__ffs0: f64 = *var_fn25_calc_iq__ffs0_slot;
        let mut var_fn25_calc_iq__ffs0_dn16: f64 = *var_fn25_calc_iq__ffs0_dn16_slot;
        let mut var_fn25_calc_iq__ffs0_dn17: f64 = *var_fn25_calc_iq__ffs0_dn17_slot;
        let mut var_fn25_calc_iq__ffs0_dn2: f64 = *var_fn25_calc_iq__ffs0_dn2_slot;
        let mut var_fn25_calc_iq__ffs0_dn4: f64 = *var_fn25_calc_iq__ffs0_dn4_slot;
        let mut var_fn25_calc_iq__ffs0_dn7: f64 = *var_fn25_calc_iq__ffs0_dn7_slot;
        let mut var_fn25_calc_iq__ffs0_rv: f64 = *var_fn25_calc_iq__ffs0_rv_slot;
        let mut var_fn25_calc_iq__qd: f64 = *var_fn25_calc_iq__qd_slot;
        let mut var_fn25_calc_iq__qd1: f64 = *var_fn25_calc_iq__qd1_slot;
        let mut var_fn25_calc_iq__qd1_dn16: f64 = *var_fn25_calc_iq__qd1_dn16_slot;
        let mut var_fn25_calc_iq__qd1_dn17: f64 = *var_fn25_calc_iq__qd1_dn17_slot;
        let mut var_fn25_calc_iq__qd1_dn2: f64 = *var_fn25_calc_iq__qd1_dn2_slot;
        let mut var_fn25_calc_iq__qd1_dn4: f64 = *var_fn25_calc_iq__qd1_dn4_slot;
        let mut var_fn25_calc_iq__qd1_dn7: f64 = *var_fn25_calc_iq__qd1_dn7_slot;
        let mut var_fn25_calc_iq__qd1_rv: f64 = *var_fn25_calc_iq__qd1_rv_slot;
        let mut var_fn25_calc_iq__qd2: f64 = *var_fn25_calc_iq__qd2_slot;
        let mut var_fn25_calc_iq__qd2_dn16: f64 = *var_fn25_calc_iq__qd2_dn16_slot;
        let mut var_fn25_calc_iq__qd2_dn17: f64 = *var_fn25_calc_iq__qd2_dn17_slot;
        let mut var_fn25_calc_iq__qd2_dn2: f64 = *var_fn25_calc_iq__qd2_dn2_slot;
        let mut var_fn25_calc_iq__qd2_dn4: f64 = *var_fn25_calc_iq__qd2_dn4_slot;
        let mut var_fn25_calc_iq__qd2_dn7: f64 = *var_fn25_calc_iq__qd2_dn7_slot;
        let mut var_fn25_calc_iq__qd2_rv: f64 = *var_fn25_calc_iq__qd2_rv_slot;
        let mut var_fn25_calc_iq__qd3: f64 = *var_fn25_calc_iq__qd3_slot;
        let mut var_fn25_calc_iq__qd3_dn16: f64 = *var_fn25_calc_iq__qd3_dn16_slot;
        let mut var_fn25_calc_iq__qd3_dn17: f64 = *var_fn25_calc_iq__qd3_dn17_slot;
        let mut var_fn25_calc_iq__qd3_dn2: f64 = *var_fn25_calc_iq__qd3_dn2_slot;
        let mut var_fn25_calc_iq__qd3_dn4: f64 = *var_fn25_calc_iq__qd3_dn4_slot;
        let mut var_fn25_calc_iq__qd3_dn7: f64 = *var_fn25_calc_iq__qd3_dn7_slot;
        let mut var_fn25_calc_iq__qd3_rv: f64 = *var_fn25_calc_iq__qd3_rv_slot;
        let mut var_fn25_calc_iq__qd_dn16: f64 = *var_fn25_calc_iq__qd_dn16_slot;
        let mut var_fn25_calc_iq__qd_dn17: f64 = *var_fn25_calc_iq__qd_dn17_slot;
        let mut var_fn25_calc_iq__qd_dn2: f64 = *var_fn25_calc_iq__qd_dn2_slot;
        let mut var_fn25_calc_iq__qd_dn4: f64 = *var_fn25_calc_iq__qd_dn4_slot;
        let mut var_fn25_calc_iq__qd_dn7: f64 = *var_fn25_calc_iq__qd_dn7_slot;
        let mut var_fn25_calc_iq__qd_rv: f64 = *var_fn25_calc_iq__qd_rv_slot;
        let mut var_fn25_calc_iq__qgdout: f64 = *var_fn25_calc_iq__qgdout_slot;
        let mut var_fn25_calc_iq__qgdout_dn16: f64 = *var_fn25_calc_iq__qgdout_dn16_slot;
        let mut var_fn25_calc_iq__qgdout_dn17: f64 = *var_fn25_calc_iq__qgdout_dn17_slot;
        let mut var_fn25_calc_iq__qgdout_dn2: f64 = *var_fn25_calc_iq__qgdout_dn2_slot;
        let mut var_fn25_calc_iq__qgdout_dn4: f64 = *var_fn25_calc_iq__qgdout_dn4_slot;
        let mut var_fn25_calc_iq__qgdout_dn7: f64 = *var_fn25_calc_iq__qgdout_dn7_slot;
        let mut var_fn25_calc_iq__qgdout_rv: f64 = *var_fn25_calc_iq__qgdout_rv_slot;
        let mut var_fn25_calc_iq__qgsout: f64 = *var_fn25_calc_iq__qgsout_slot;
        let mut var_fn25_calc_iq__qgsout_dn16: f64 = *var_fn25_calc_iq__qgsout_dn16_slot;
        let mut var_fn25_calc_iq__qgsout_dn17: f64 = *var_fn25_calc_iq__qgsout_dn17_slot;
        let mut var_fn25_calc_iq__qgsout_dn2: f64 = *var_fn25_calc_iq__qgsout_dn2_slot;
        let mut var_fn25_calc_iq__qgsout_dn4: f64 = *var_fn25_calc_iq__qgsout_dn4_slot;
        let mut var_fn25_calc_iq__qgsout_dn7: f64 = *var_fn25_calc_iq__qgsout_dn7_slot;
        let mut var_fn25_calc_iq__qgsout_rv: f64 = *var_fn25_calc_iq__qgsout_rv_slot;
        let mut var_fn25_calc_iq__qinvd0: f64 = *var_fn25_calc_iq__qinvd0_slot;
        let mut var_fn25_calc_iq__qinvd0_dn16: f64 = *var_fn25_calc_iq__qinvd0_dn16_slot;
        let mut var_fn25_calc_iq__qinvd0_dn17: f64 = *var_fn25_calc_iq__qinvd0_dn17_slot;
        let mut var_fn25_calc_iq__qinvd0_dn2: f64 = *var_fn25_calc_iq__qinvd0_dn2_slot;
        let mut var_fn25_calc_iq__qinvd0_dn4: f64 = *var_fn25_calc_iq__qinvd0_dn4_slot;
        let mut var_fn25_calc_iq__qinvd0_dn7: f64 = *var_fn25_calc_iq__qinvd0_dn7_slot;
        let mut var_fn25_calc_iq__qinvd0_rv: f64 = *var_fn25_calc_iq__qinvd0_rv_slot;
        let mut var_fn25_calc_iq__qinvdd: f64 = *var_fn25_calc_iq__qinvdd_slot;
        let mut var_fn25_calc_iq__qinvdd_dn16: f64 = *var_fn25_calc_iq__qinvdd_dn16_slot;
        let mut var_fn25_calc_iq__qinvdd_dn17: f64 = *var_fn25_calc_iq__qinvdd_dn17_slot;
        let mut var_fn25_calc_iq__qinvdd_dn2: f64 = *var_fn25_calc_iq__qinvdd_dn2_slot;
        let mut var_fn25_calc_iq__qinvdd_dn4: f64 = *var_fn25_calc_iq__qinvdd_dn4_slot;
        let mut var_fn25_calc_iq__qinvdd_dn7: f64 = *var_fn25_calc_iq__qinvdd_dn7_slot;
        let mut var_fn25_calc_iq__qinvdd_rv: f64 = *var_fn25_calc_iq__qinvdd_rv_slot;
        let mut var_fn25_calc_iq__qinvs0: f64 = *var_fn25_calc_iq__qinvs0_slot;
        let mut var_fn25_calc_iq__qinvs0_dn16: f64 = *var_fn25_calc_iq__qinvs0_dn16_slot;
        let mut var_fn25_calc_iq__qinvs0_dn17: f64 = *var_fn25_calc_iq__qinvs0_dn17_slot;
        let mut var_fn25_calc_iq__qinvs0_dn2: f64 = *var_fn25_calc_iq__qinvs0_dn2_slot;
        let mut var_fn25_calc_iq__qinvs0_dn4: f64 = *var_fn25_calc_iq__qinvs0_dn4_slot;
        let mut var_fn25_calc_iq__qinvs0_dn7: f64 = *var_fn25_calc_iq__qinvs0_dn7_slot;
        let mut var_fn25_calc_iq__qinvs0_rv: f64 = *var_fn25_calc_iq__qinvs0_rv_slot;
        let mut var_fn25_calc_iq__qs: f64 = *var_fn25_calc_iq__qs_slot;
        let mut var_fn25_calc_iq__qs2: f64 = *var_fn25_calc_iq__qs2_slot;
        let mut var_fn25_calc_iq__qs2_dn16: f64 = *var_fn25_calc_iq__qs2_dn16_slot;
        let mut var_fn25_calc_iq__qs2_dn17: f64 = *var_fn25_calc_iq__qs2_dn17_slot;
        let mut var_fn25_calc_iq__qs2_dn2: f64 = *var_fn25_calc_iq__qs2_dn2_slot;
        let mut var_fn25_calc_iq__qs2_dn4: f64 = *var_fn25_calc_iq__qs2_dn4_slot;
        let mut var_fn25_calc_iq__qs2_dn7: f64 = *var_fn25_calc_iq__qs2_dn7_slot;
        let mut var_fn25_calc_iq__qs2_rv: f64 = *var_fn25_calc_iq__qs2_rv_slot;
        let mut var_fn25_calc_iq__qs3: f64 = *var_fn25_calc_iq__qs3_slot;
        let mut var_fn25_calc_iq__qs3_dn16: f64 = *var_fn25_calc_iq__qs3_dn16_slot;
        let mut var_fn25_calc_iq__qs3_dn17: f64 = *var_fn25_calc_iq__qs3_dn17_slot;
        let mut var_fn25_calc_iq__qs3_dn2: f64 = *var_fn25_calc_iq__qs3_dn2_slot;
        let mut var_fn25_calc_iq__qs3_dn4: f64 = *var_fn25_calc_iq__qs3_dn4_slot;
        let mut var_fn25_calc_iq__qs3_dn7: f64 = *var_fn25_calc_iq__qs3_dn7_slot;
        let mut var_fn25_calc_iq__qs3_rv: f64 = *var_fn25_calc_iq__qs3_rv_slot;
        let mut var_fn25_calc_iq__qs_dn16: f64 = *var_fn25_calc_iq__qs_dn16_slot;
        let mut var_fn25_calc_iq__qs_dn17: f64 = *var_fn25_calc_iq__qs_dn17_slot;
        let mut var_fn25_calc_iq__qs_dn2: f64 = *var_fn25_calc_iq__qs_dn2_slot;
        let mut var_fn25_calc_iq__qs_dn4: f64 = *var_fn25_calc_iq__qs_dn4_slot;
        let mut var_fn25_calc_iq__qs_dn7: f64 = *var_fn25_calc_iq__qs_dn7_slot;
        let mut var_fn25_calc_iq__qs_rv: f64 = *var_fn25_calc_iq__qs_rv_slot;
        let mut var_fn25_calc_iq__qsqd: f64 = *var_fn25_calc_iq__qsqd_slot;
        let mut var_fn25_calc_iq__qsqd_dn16: f64 = *var_fn25_calc_iq__qsqd_dn16_slot;
        let mut var_fn25_calc_iq__qsqd_dn17: f64 = *var_fn25_calc_iq__qsqd_dn17_slot;
        let mut var_fn25_calc_iq__qsqd_dn2: f64 = *var_fn25_calc_iq__qsqd_dn2_slot;
        let mut var_fn25_calc_iq__qsqd_dn4: f64 = *var_fn25_calc_iq__qsqd_dn4_slot;
        let mut var_fn25_calc_iq__qsqd_dn7: f64 = *var_fn25_calc_iq__qsqd_dn7_slot;
        let mut var_fn25_calc_iq__qsqd_rv: f64 = *var_fn25_calc_iq__qsqd_rv_slot;
        let mut var_guard45: f64 = *var_guard45_slot;
        let mut var_guard45_rv: f64 = *var_guard45_rv_slot;
        let mut var_guard46: f64 = *var_guard46_slot;
        let mut var_guard46_rv: f64 = *var_guard46_rv_slot;
        let mut var_guard47: f64 = *var_guard47_slot;
        let mut var_guard47_rv: f64 = *var_guard47_rv_slot;
        let mut var_guard48: f64 = *var_guard48_slot;
        let mut var_guard48_rv: f64 = *var_guard48_rv_slot;
        let mut var_guard49: f64 = *var_guard49_slot;
        let mut var_guard49_rv: f64 = *var_guard49_rv_slot;
        let mut var_guard50: f64 = *var_guard50_slot;
        let mut var_guard50_rv: f64 = *var_guard50_rv_slot;
        let mut var_guard51: f64 = *var_guard51_slot;
        let mut var_guard51_rv: f64 = *var_guard51_rv_slot;
        let mut var_guard52: f64 = *var_guard52_slot;
        let mut var_guard52_rv: f64 = *var_guard52_rv_slot;

        let (assign3840_e5700, assign3840_e5700_d_n2, assign3840_e5700_d_n4, assign3840_e5700_d_n7, assign3840_e5700_d_n16, assign3840_e5700_d_n17,) = {
    if (((var_guard24 != 0.0) && (var_guard43 == 0.0)) && (var_guard44 == 0.0)) {
        let assign3840_e5696: f64 = (var_fn25_calc_iq__exparg0).exp();
        let assign3840_e5697: f64 = (1.0 + assign3840_e5696);
        let assign3840_e5698: f64 = (1.0 / assign3840_e5697);
        (assign3840_e5698, (-((assign3840_e5696 * var_fn25_calc_iq__exparg0_dn2) / (assign3840_e5697 * assign3840_e5697))), (-((assign3840_e5696 * var_fn25_calc_iq__exparg0_dn4) / (assign3840_e5697 * assign3840_e5697))), (-((assign3840_e5696 * var_fn25_calc_iq__exparg0_dn7) / (assign3840_e5697 * assign3840_e5697))), (-((assign3840_e5696 * var_fn25_calc_iq__exparg0_dn16) / (assign3840_e5697 * assign3840_e5697))), (-((assign3840_e5696 * var_fn25_calc_iq__exparg0_dn17) / (assign3840_e5697 * assign3840_e5697))),)
    } else {
        (var_fn25_calc_iq__ffs0, var_fn25_calc_iq__ffs0_dn2, var_fn25_calc_iq__ffs0_dn4, var_fn25_calc_iq__ffs0_dn7, var_fn25_calc_iq__ffs0_dn16, var_fn25_calc_iq__ffs0_dn17,)
    }
};
        var_fn25_calc_iq__ffs0 = assign3840_e5700;
        var_fn25_calc_iq__ffs0_dn2 = assign3840_e5700_d_n2;
        var_fn25_calc_iq__ffs0_dn4 = assign3840_e5700_d_n4;
        var_fn25_calc_iq__ffs0_dn7 = assign3840_e5700_d_n7;
        var_fn25_calc_iq__ffs0_dn16 = assign3840_e5700_d_n16;
        var_fn25_calc_iq__ffs0_dn17 = assign3840_e5700_d_n17;
        var_fn25_calc_iq__ffs0_rv = 0.0;

        let (assign3850_e5718, assign3850_e5718_d_n2, assign3850_e5718_d_n4, assign3850_e5718_d_n7, assign3850_e5718_d_n16, assign3850_e5718_d_n17,) = {
    if (var_guard24 != 0.0) {
        let assign3850_e5704: f64 = (var_fn25_calc_iq__vgdin - var_fn25_calc_iq__vsx0);
        let assign3850_e5708: f64 = (p.p51 * 0.1);
        let assign3850_e5710: f64 = (assign3850_e5708 * var_fn25_calc_iq__alpha_phit);
        let assign3850_e5712: f64 = (assign3850_e5710 * var_fn25_calc_iq__ffs0);
        let assign3850_e5713: f64 = (var_fn25_calc_iq__vtof - assign3850_e5712);
        let assign3850_e5714: f64 = (assign3850_e5704 - assign3850_e5713);
        let assign3850_e5716: f64 = (assign3850_e5714 / var_fn25_calc_iq__two_n_phit0);
        (assign3850_e5716, (((var_fn25_calc_iq__vgdin_dn2 - var_fn25_calc_iq__vsx0_dn2) - (-(assign3850_e5710 * var_fn25_calc_iq__ffs0_dn2))) / var_fn25_calc_iq__two_n_phit0), (((((-var_fn25_calc_iq__vsx0_dn4) - (var_fn25_calc_iq__vtof_dn4 - (((assign3850_e5708 * var_fn25_calc_iq__alpha_phit_dn4) * var_fn25_calc_iq__ffs0) + (assign3850_e5710 * var_fn25_calc_iq__ffs0_dn4)))) * var_fn25_calc_iq__two_n_phit0) - (assign3850_e5714 * var_fn25_calc_iq__two_n_phit0_dn4)) / (var_fn25_calc_iq__two_n_phit0 * var_fn25_calc_iq__two_n_phit0)), (((var_fn25_calc_iq__vgdin_dn7 - var_fn25_calc_iq__vsx0_dn7) - (-(assign3850_e5710 * var_fn25_calc_iq__ffs0_dn7))) / var_fn25_calc_iq__two_n_phit0), (((var_fn25_calc_iq__vgdin_dn16 - var_fn25_calc_iq__vsx0_dn16) - (-(assign3850_e5710 * var_fn25_calc_iq__ffs0_dn16))) / var_fn25_calc_iq__two_n_phit0), (((var_fn25_calc_iq__vgdin_dn17 - var_fn25_calc_iq__vsx0_dn17) - (-(assign3850_e5710 * var_fn25_calc_iq__ffs0_dn17))) / var_fn25_calc_iq__two_n_phit0),)
    } else {
        (var_fn25_calc_iq__etas0, var_fn25_calc_iq__etas0_dn2, var_fn25_calc_iq__etas0_dn4, var_fn25_calc_iq__etas0_dn7, var_fn25_calc_iq__etas0_dn16, var_fn25_calc_iq__etas0_dn17,)
    }
};
        var_fn25_calc_iq__etas0 = assign3850_e5718;
        var_fn25_calc_iq__etas0_dn2 = assign3850_e5718_d_n2;
        var_fn25_calc_iq__etas0_dn4 = assign3850_e5718_d_n4;
        var_fn25_calc_iq__etas0_dn7 = assign3850_e5718_d_n7;
        var_fn25_calc_iq__etas0_dn16 = assign3850_e5718_d_n16;
        var_fn25_calc_iq__etas0_dn17 = assign3850_e5718_d_n17;
        var_fn25_calc_iq__etas0_rv = 0.0;

        let assign3860_e5721: f64 = if var_fn25_calc_iq__etas0 > 50.0 { 1.0 } else { 0.0 };
        var_guard45 = assign3860_e5721;
        var_guard45_rv = 0.0;

        let (assign3870_e5729, assign3870_e5729_d_n2, assign3870_e5729_d_n4, assign3870_e5729_d_n7, assign3870_e5729_d_n16, assign3870_e5729_d_n17,) = {
    if ((var_guard24 != 0.0) && (var_guard45 != 0.0)) {
        let assign3870_e5727: f64 = (var_fn25_calc_iq__qref0 * var_fn25_calc_iq__etas0);
        (assign3870_e5727, (var_fn25_calc_iq__qref0 * var_fn25_calc_iq__etas0_dn2), ((var_fn25_calc_iq__qref0_dn4 * var_fn25_calc_iq__etas0) + (var_fn25_calc_iq__qref0 * var_fn25_calc_iq__etas0_dn4)), (var_fn25_calc_iq__qref0 * var_fn25_calc_iq__etas0_dn7), (var_fn25_calc_iq__qref0 * var_fn25_calc_iq__etas0_dn16), (var_fn25_calc_iq__qref0 * var_fn25_calc_iq__etas0_dn17),)
    } else {
        (var_fn25_calc_iq__qinvs0, var_fn25_calc_iq__qinvs0_dn2, var_fn25_calc_iq__qinvs0_dn4, var_fn25_calc_iq__qinvs0_dn7, var_fn25_calc_iq__qinvs0_dn16, var_fn25_calc_iq__qinvs0_dn17,)
    }
};
        var_fn25_calc_iq__qinvs0 = assign3870_e5729;
        var_fn25_calc_iq__qinvs0_dn2 = assign3870_e5729_d_n2;
        var_fn25_calc_iq__qinvs0_dn4 = assign3870_e5729_d_n4;
        var_fn25_calc_iq__qinvs0_dn7 = assign3870_e5729_d_n7;
        var_fn25_calc_iq__qinvs0_dn16 = assign3870_e5729_d_n16;
        var_fn25_calc_iq__qinvs0_dn17 = assign3870_e5729_d_n17;
        var_fn25_calc_iq__qinvs0_rv = 0.0;

        let assign3880_e5732: f64 = (-50.0);
        let assign3880_e5733: f64 = if var_fn25_calc_iq__etas0 < assign3880_e5732 { 1.0 } else { 0.0 };
        var_guard46 = assign3880_e5733;
        var_guard46_rv = 0.0;

        let (assign3890_e5745, assign3890_e5745_d_n2, assign3890_e5745_d_n4, assign3890_e5745_d_n7, assign3890_e5745_d_n16, assign3890_e5745_d_n17,) = {
    if (((var_guard24 != 0.0) && (var_guard45 == 0.0)) && (var_guard46 != 0.0)) {
        let assign3890_e5742: f64 = (var_fn25_calc_iq__etas0).exp();
        let assign3890_e5743: f64 = (var_fn25_calc_iq__qref0 * assign3890_e5742);
        (assign3890_e5743, (var_fn25_calc_iq__qref0 * (assign3890_e5742 * var_fn25_calc_iq__etas0_dn2)), ((var_fn25_calc_iq__qref0_dn4 * assign3890_e5742) + (var_fn25_calc_iq__qref0 * (assign3890_e5742 * var_fn25_calc_iq__etas0_dn4))), (var_fn25_calc_iq__qref0 * (assign3890_e5742 * var_fn25_calc_iq__etas0_dn7)), (var_fn25_calc_iq__qref0 * (assign3890_e5742 * var_fn25_calc_iq__etas0_dn16)), (var_fn25_calc_iq__qref0 * (assign3890_e5742 * var_fn25_calc_iq__etas0_dn17)),)
    } else {
        (var_fn25_calc_iq__qinvs0, var_fn25_calc_iq__qinvs0_dn2, var_fn25_calc_iq__qinvs0_dn4, var_fn25_calc_iq__qinvs0_dn7, var_fn25_calc_iq__qinvs0_dn16, var_fn25_calc_iq__qinvs0_dn17,)
    }
};
        var_fn25_calc_iq__qinvs0 = assign3890_e5745;
        var_fn25_calc_iq__qinvs0_dn2 = assign3890_e5745_d_n2;
        var_fn25_calc_iq__qinvs0_dn4 = assign3890_e5745_d_n4;
        var_fn25_calc_iq__qinvs0_dn7 = assign3890_e5745_d_n7;
        var_fn25_calc_iq__qinvs0_dn16 = assign3890_e5745_d_n16;
        var_fn25_calc_iq__qinvs0_dn17 = assign3890_e5745_d_n17;
        var_fn25_calc_iq__qinvs0_rv = 0.0;

        let (assign3900_e5761, assign3900_e5761_d_n2, assign3900_e5761_d_n4, assign3900_e5761_d_n7, assign3900_e5761_d_n16, assign3900_e5761_d_n17,) = {
    if (((var_guard24 != 0.0) && (var_guard45 == 0.0)) && (var_guard46 == 0.0)) {
        let assign3900_e5756: f64 = (var_fn25_calc_iq__etas0).exp();
        let assign3900_e5757: f64 = (1.0 + assign3900_e5756);
        let assign3900_e5758: f64 = (assign3900_e5757).ln();
        let assign3900_e5759: f64 = (var_fn25_calc_iq__qref0 * assign3900_e5758);
        (assign3900_e5759, (var_fn25_calc_iq__qref0 * ((assign3900_e5756 * var_fn25_calc_iq__etas0_dn2) / assign3900_e5757)), ((var_fn25_calc_iq__qref0_dn4 * assign3900_e5758) + (var_fn25_calc_iq__qref0 * ((assign3900_e5756 * var_fn25_calc_iq__etas0_dn4) / assign3900_e5757))), (var_fn25_calc_iq__qref0 * ((assign3900_e5756 * var_fn25_calc_iq__etas0_dn7) / assign3900_e5757)), (var_fn25_calc_iq__qref0 * ((assign3900_e5756 * var_fn25_calc_iq__etas0_dn16) / assign3900_e5757)), (var_fn25_calc_iq__qref0 * ((assign3900_e5756 * var_fn25_calc_iq__etas0_dn17) / assign3900_e5757)),)
    } else {
        (var_fn25_calc_iq__qinvs0, var_fn25_calc_iq__qinvs0_dn2, var_fn25_calc_iq__qinvs0_dn4, var_fn25_calc_iq__qinvs0_dn7, var_fn25_calc_iq__qinvs0_dn16, var_fn25_calc_iq__qinvs0_dn17,)
    }
};
        var_fn25_calc_iq__qinvs0 = assign3900_e5761;
        var_fn25_calc_iq__qinvs0_dn2 = assign3900_e5761_d_n2;
        var_fn25_calc_iq__qinvs0_dn4 = assign3900_e5761_d_n4;
        var_fn25_calc_iq__qinvs0_dn7 = assign3900_e5761_d_n7;
        var_fn25_calc_iq__qinvs0_dn16 = assign3900_e5761_d_n16;
        var_fn25_calc_iq__qinvs0_dn17 = assign3900_e5761_d_n17;
        var_fn25_calc_iq__qinvs0_rv = 0.0;

        let (assign3910_e5769, assign3910_e5769_d_n2, assign3910_e5769_d_n4, assign3910_e5769_d_n7, assign3910_e5769_d_n16, assign3910_e5769_d_n17,) = {
    if (var_guard24 != 0.0) {
        let assign3910_e5765: f64 = (var_fn25_calc_iq__vgdin - var_fn25_calc_iq__myarg0);
        let assign3910_e5767: f64 = (assign3910_e5765 / var_fn25_calc_iq__alpha_phit);
        (assign3910_e5767, (var_fn25_calc_iq__vgdin_dn2 / var_fn25_calc_iq__alpha_phit), ((((-var_fn25_calc_iq__myarg0_dn4) * var_fn25_calc_iq__alpha_phit) - (assign3910_e5765 * var_fn25_calc_iq__alpha_phit_dn4)) / (var_fn25_calc_iq__alpha_phit * var_fn25_calc_iq__alpha_phit)), (var_fn25_calc_iq__vgdin_dn7 / var_fn25_calc_iq__alpha_phit), (var_fn25_calc_iq__vgdin_dn16 / var_fn25_calc_iq__alpha_phit), (var_fn25_calc_iq__vgdin_dn17 / var_fn25_calc_iq__alpha_phit),)
    } else {
        (var_fn25_calc_iq__exparg0, var_fn25_calc_iq__exparg0_dn2, var_fn25_calc_iq__exparg0_dn4, var_fn25_calc_iq__exparg0_dn7, var_fn25_calc_iq__exparg0_dn16, var_fn25_calc_iq__exparg0_dn17,)
    }
};
        var_fn25_calc_iq__exparg0 = assign3910_e5769;
        var_fn25_calc_iq__exparg0_dn2 = assign3910_e5769_d_n2;
        var_fn25_calc_iq__exparg0_dn4 = assign3910_e5769_d_n4;
        var_fn25_calc_iq__exparg0_dn7 = assign3910_e5769_d_n7;
        var_fn25_calc_iq__exparg0_dn16 = assign3910_e5769_d_n16;
        var_fn25_calc_iq__exparg0_dn17 = assign3910_e5769_d_n17;
        var_fn25_calc_iq__exparg0_rv = 0.0;

        let assign3920_e5772: f64 = if var_fn25_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        var_guard47 = assign3920_e5772;
        var_guard47_rv = 0.0;

        let (assign3930_e5778, assign3930_e5778_d_n2, assign3930_e5778_d_n4, assign3930_e5778_d_n7, assign3930_e5778_d_n16, assign3930_e5778_d_n17,) = {
    if ((var_guard24 != 0.0) && (var_guard47 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__ffd0, var_fn25_calc_iq__ffd0_dn2, var_fn25_calc_iq__ffd0_dn4, var_fn25_calc_iq__ffd0_dn7, var_fn25_calc_iq__ffd0_dn16, var_fn25_calc_iq__ffd0_dn17,)
    }
};
        var_fn25_calc_iq__ffd0 = assign3930_e5778;
        var_fn25_calc_iq__ffd0_dn2 = assign3930_e5778_d_n2;
        var_fn25_calc_iq__ffd0_dn4 = assign3930_e5778_d_n4;
        var_fn25_calc_iq__ffd0_dn7 = assign3930_e5778_d_n7;
        var_fn25_calc_iq__ffd0_dn16 = assign3930_e5778_d_n16;
        var_fn25_calc_iq__ffd0_dn17 = assign3930_e5778_d_n17;
        var_fn25_calc_iq__ffd0_rv = 0.0;

        let assign3940_e5781: f64 = (-50.0);
        let assign3940_e5782: f64 = if var_fn25_calc_iq__exparg0 < assign3940_e5781 { 1.0 } else { 0.0 };
        var_guard48 = assign3940_e5782;
        var_guard48_rv = 0.0;

        let (assign3950_e5791, assign3950_e5791_d_n2, assign3950_e5791_d_n4, assign3950_e5791_d_n7, assign3950_e5791_d_n16, assign3950_e5791_d_n17,) = {
    if (((var_guard24 != 0.0) && (var_guard47 == 0.0)) && (var_guard48 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn25_calc_iq__ffd0, var_fn25_calc_iq__ffd0_dn2, var_fn25_calc_iq__ffd0_dn4, var_fn25_calc_iq__ffd0_dn7, var_fn25_calc_iq__ffd0_dn16, var_fn25_calc_iq__ffd0_dn17,)
    }
};
        var_fn25_calc_iq__ffd0 = assign3950_e5791;
        var_fn25_calc_iq__ffd0_dn2 = assign3950_e5791_d_n2;
        var_fn25_calc_iq__ffd0_dn4 = assign3950_e5791_d_n4;
        var_fn25_calc_iq__ffd0_dn7 = assign3950_e5791_d_n7;
        var_fn25_calc_iq__ffd0_dn16 = assign3950_e5791_d_n16;
        var_fn25_calc_iq__ffd0_dn17 = assign3950_e5791_d_n17;
        var_fn25_calc_iq__ffd0_rv = 0.0;

        let (assign3960_e5806, assign3960_e5806_d_n2, assign3960_e5806_d_n4, assign3960_e5806_d_n7, assign3960_e5806_d_n16, assign3960_e5806_d_n17,) = {
    if (((var_guard24 != 0.0) && (var_guard47 == 0.0)) && (var_guard48 == 0.0)) {
        let assign3960_e5802: f64 = (var_fn25_calc_iq__exparg0).exp();
        let assign3960_e5803: f64 = (1.0 + assign3960_e5802);
        let assign3960_e5804: f64 = (1.0 / assign3960_e5803);
        (assign3960_e5804, (-((assign3960_e5802 * var_fn25_calc_iq__exparg0_dn2) / (assign3960_e5803 * assign3960_e5803))), (-((assign3960_e5802 * var_fn25_calc_iq__exparg0_dn4) / (assign3960_e5803 * assign3960_e5803))), (-((assign3960_e5802 * var_fn25_calc_iq__exparg0_dn7) / (assign3960_e5803 * assign3960_e5803))), (-((assign3960_e5802 * var_fn25_calc_iq__exparg0_dn16) / (assign3960_e5803 * assign3960_e5803))), (-((assign3960_e5802 * var_fn25_calc_iq__exparg0_dn17) / (assign3960_e5803 * assign3960_e5803))),)
    } else {
        (var_fn25_calc_iq__ffd0, var_fn25_calc_iq__ffd0_dn2, var_fn25_calc_iq__ffd0_dn4, var_fn25_calc_iq__ffd0_dn7, var_fn25_calc_iq__ffd0_dn16, var_fn25_calc_iq__ffd0_dn17,)
    }
};
        var_fn25_calc_iq__ffd0 = assign3960_e5806;
        var_fn25_calc_iq__ffd0_dn2 = assign3960_e5806_d_n2;
        var_fn25_calc_iq__ffd0_dn4 = assign3960_e5806_d_n4;
        var_fn25_calc_iq__ffd0_dn7 = assign3960_e5806_d_n7;
        var_fn25_calc_iq__ffd0_dn16 = assign3960_e5806_d_n16;
        var_fn25_calc_iq__ffd0_dn17 = assign3960_e5806_d_n17;
        var_fn25_calc_iq__ffd0_rv = 0.0;

        let (assign3970_e5824, assign3970_e5824_d_n2, assign3970_e5824_d_n4, assign3970_e5824_d_n7, assign3970_e5824_d_n16, assign3970_e5824_d_n17,) = {
    if (var_guard24 != 0.0) {
        let assign3970_e5810: f64 = (var_fn25_calc_iq__vgsin - var_fn25_calc_iq__vdx0);
        let assign3970_e5814: f64 = (p.p51 * 0.1);
        let assign3970_e5816: f64 = (assign3970_e5814 * var_fn25_calc_iq__alpha_phit);
        let assign3970_e5818: f64 = (assign3970_e5816 * var_fn25_calc_iq__ffd0);
        let assign3970_e5819: f64 = (var_fn25_calc_iq__vtof - assign3970_e5818);
        let assign3970_e5820: f64 = (assign3970_e5810 - assign3970_e5819);
        let assign3970_e5822: f64 = (assign3970_e5820 / var_fn25_calc_iq__two_n_phit0);
        (assign3970_e5822, (((var_fn25_calc_iq__vgsin_dn2 - var_fn25_calc_iq__vdx0_dn2) - (-(assign3970_e5816 * var_fn25_calc_iq__ffd0_dn2))) / var_fn25_calc_iq__two_n_phit0), (((((-var_fn25_calc_iq__vdx0_dn4) - (var_fn25_calc_iq__vtof_dn4 - (((assign3970_e5814 * var_fn25_calc_iq__alpha_phit_dn4) * var_fn25_calc_iq__ffd0) + (assign3970_e5816 * var_fn25_calc_iq__ffd0_dn4)))) * var_fn25_calc_iq__two_n_phit0) - (assign3970_e5820 * var_fn25_calc_iq__two_n_phit0_dn4)) / (var_fn25_calc_iq__two_n_phit0 * var_fn25_calc_iq__two_n_phit0)), (((var_fn25_calc_iq__vgsin_dn7 - var_fn25_calc_iq__vdx0_dn7) - (-(assign3970_e5816 * var_fn25_calc_iq__ffd0_dn7))) / var_fn25_calc_iq__two_n_phit0), (((var_fn25_calc_iq__vgsin_dn16 - var_fn25_calc_iq__vdx0_dn16) - (-(assign3970_e5816 * var_fn25_calc_iq__ffd0_dn16))) / var_fn25_calc_iq__two_n_phit0), (((-var_fn25_calc_iq__vdx0_dn17) - (-(assign3970_e5816 * var_fn25_calc_iq__ffd0_dn17))) / var_fn25_calc_iq__two_n_phit0),)
    } else {
        (var_fn25_calc_iq__etad0, var_fn25_calc_iq__etad0_dn2, var_fn25_calc_iq__etad0_dn4, var_fn25_calc_iq__etad0_dn7, var_fn25_calc_iq__etad0_dn16, var_fn25_calc_iq__etad0_dn17,)
    }
};
        var_fn25_calc_iq__etad0 = assign3970_e5824;
        var_fn25_calc_iq__etad0_dn2 = assign3970_e5824_d_n2;
        var_fn25_calc_iq__etad0_dn4 = assign3970_e5824_d_n4;
        var_fn25_calc_iq__etad0_dn7 = assign3970_e5824_d_n7;
        var_fn25_calc_iq__etad0_dn16 = assign3970_e5824_d_n16;
        var_fn25_calc_iq__etad0_dn17 = assign3970_e5824_d_n17;
        var_fn25_calc_iq__etad0_rv = 0.0;

        let assign3980_e5827: f64 = if var_fn25_calc_iq__etad0 > 50.0 { 1.0 } else { 0.0 };
        var_guard49 = assign3980_e5827;
        var_guard49_rv = 0.0;

        let (assign3990_e5835, assign3990_e5835_d_n2, assign3990_e5835_d_n4, assign3990_e5835_d_n7, assign3990_e5835_d_n16, assign3990_e5835_d_n17,) = {
    if ((var_guard24 != 0.0) && (var_guard49 != 0.0)) {
        let assign3990_e5833: f64 = (var_fn25_calc_iq__qref0 * var_fn25_calc_iq__etad0);
        (assign3990_e5833, (var_fn25_calc_iq__qref0 * var_fn25_calc_iq__etad0_dn2), ((var_fn25_calc_iq__qref0_dn4 * var_fn25_calc_iq__etad0) + (var_fn25_calc_iq__qref0 * var_fn25_calc_iq__etad0_dn4)), (var_fn25_calc_iq__qref0 * var_fn25_calc_iq__etad0_dn7), (var_fn25_calc_iq__qref0 * var_fn25_calc_iq__etad0_dn16), (var_fn25_calc_iq__qref0 * var_fn25_calc_iq__etad0_dn17),)
    } else {
        (var_fn25_calc_iq__qinvd0, var_fn25_calc_iq__qinvd0_dn2, var_fn25_calc_iq__qinvd0_dn4, var_fn25_calc_iq__qinvd0_dn7, var_fn25_calc_iq__qinvd0_dn16, var_fn25_calc_iq__qinvd0_dn17,)
    }
};
        var_fn25_calc_iq__qinvd0 = assign3990_e5835;
        var_fn25_calc_iq__qinvd0_dn2 = assign3990_e5835_d_n2;
        var_fn25_calc_iq__qinvd0_dn4 = assign3990_e5835_d_n4;
        var_fn25_calc_iq__qinvd0_dn7 = assign3990_e5835_d_n7;
        var_fn25_calc_iq__qinvd0_dn16 = assign3990_e5835_d_n16;
        var_fn25_calc_iq__qinvd0_dn17 = assign3990_e5835_d_n17;
        var_fn25_calc_iq__qinvd0_rv = 0.0;

        let assign4000_e5838: f64 = (-50.0);
        let assign4000_e5839: f64 = if var_fn25_calc_iq__etad0 < assign4000_e5838 { 1.0 } else { 0.0 };
        var_guard50 = assign4000_e5839;
        var_guard50_rv = 0.0;

        let (assign4010_e5851, assign4010_e5851_d_n2, assign4010_e5851_d_n4, assign4010_e5851_d_n7, assign4010_e5851_d_n16, assign4010_e5851_d_n17,) = {
    if (((var_guard24 != 0.0) && (var_guard49 == 0.0)) && (var_guard50 != 0.0)) {
        let assign4010_e5848: f64 = (var_fn25_calc_iq__etad0).exp();
        let assign4010_e5849: f64 = (var_fn25_calc_iq__qref0 * assign4010_e5848);
        (assign4010_e5849, (var_fn25_calc_iq__qref0 * (assign4010_e5848 * var_fn25_calc_iq__etad0_dn2)), ((var_fn25_calc_iq__qref0_dn4 * assign4010_e5848) + (var_fn25_calc_iq__qref0 * (assign4010_e5848 * var_fn25_calc_iq__etad0_dn4))), (var_fn25_calc_iq__qref0 * (assign4010_e5848 * var_fn25_calc_iq__etad0_dn7)), (var_fn25_calc_iq__qref0 * (assign4010_e5848 * var_fn25_calc_iq__etad0_dn16)), (var_fn25_calc_iq__qref0 * (assign4010_e5848 * var_fn25_calc_iq__etad0_dn17)),)
    } else {
        (var_fn25_calc_iq__qinvd0, var_fn25_calc_iq__qinvd0_dn2, var_fn25_calc_iq__qinvd0_dn4, var_fn25_calc_iq__qinvd0_dn7, var_fn25_calc_iq__qinvd0_dn16, var_fn25_calc_iq__qinvd0_dn17,)
    }
};
        var_fn25_calc_iq__qinvd0 = assign4010_e5851;
        var_fn25_calc_iq__qinvd0_dn2 = assign4010_e5851_d_n2;
        var_fn25_calc_iq__qinvd0_dn4 = assign4010_e5851_d_n4;
        var_fn25_calc_iq__qinvd0_dn7 = assign4010_e5851_d_n7;
        var_fn25_calc_iq__qinvd0_dn16 = assign4010_e5851_d_n16;
        var_fn25_calc_iq__qinvd0_dn17 = assign4010_e5851_d_n17;
        var_fn25_calc_iq__qinvd0_rv = 0.0;

        let (assign4020_e5867, assign4020_e5867_d_n2, assign4020_e5867_d_n4, assign4020_e5867_d_n7, assign4020_e5867_d_n16, assign4020_e5867_d_n17,) = {
    if (((var_guard24 != 0.0) && (var_guard49 == 0.0)) && (var_guard50 == 0.0)) {
        let assign4020_e5862: f64 = (var_fn25_calc_iq__etad0).exp();
        let assign4020_e5863: f64 = (1.0 + assign4020_e5862);
        let assign4020_e5864: f64 = (assign4020_e5863).ln();
        let assign4020_e5865: f64 = (var_fn25_calc_iq__qref0 * assign4020_e5864);
        (assign4020_e5865, (var_fn25_calc_iq__qref0 * ((assign4020_e5862 * var_fn25_calc_iq__etad0_dn2) / assign4020_e5863)), ((var_fn25_calc_iq__qref0_dn4 * assign4020_e5864) + (var_fn25_calc_iq__qref0 * ((assign4020_e5862 * var_fn25_calc_iq__etad0_dn4) / assign4020_e5863))), (var_fn25_calc_iq__qref0 * ((assign4020_e5862 * var_fn25_calc_iq__etad0_dn7) / assign4020_e5863)), (var_fn25_calc_iq__qref0 * ((assign4020_e5862 * var_fn25_calc_iq__etad0_dn16) / assign4020_e5863)), (var_fn25_calc_iq__qref0 * ((assign4020_e5862 * var_fn25_calc_iq__etad0_dn17) / assign4020_e5863)),)
    } else {
        (var_fn25_calc_iq__qinvd0, var_fn25_calc_iq__qinvd0_dn2, var_fn25_calc_iq__qinvd0_dn4, var_fn25_calc_iq__qinvd0_dn7, var_fn25_calc_iq__qinvd0_dn16, var_fn25_calc_iq__qinvd0_dn17,)
    }
};
        var_fn25_calc_iq__qinvd0 = assign4020_e5867;
        var_fn25_calc_iq__qinvd0_dn2 = assign4020_e5867_d_n2;
        var_fn25_calc_iq__qinvd0_dn4 = assign4020_e5867_d_n4;
        var_fn25_calc_iq__qinvd0_dn7 = assign4020_e5867_d_n7;
        var_fn25_calc_iq__qinvd0_dn16 = assign4020_e5867_d_n16;
        var_fn25_calc_iq__qinvd0_dn17 = assign4020_e5867_d_n17;
        var_fn25_calc_iq__qinvd0_rv = 0.0;

        let (assign4030_e5875, assign4030_e5875_d_n2, assign4030_e5875_d_n4, assign4030_e5875_d_n7, assign4030_e5875_d_n16, assign4030_e5875_d_n17,) = {
    if (var_guard24 != 0.0) {
        let assign4030_e5871: f64 = (var_fn25_calc_iq__qinvs0 * var_fn25_calc_iq__qinvs0);
        let assign4030_e5873: f64 = (assign4030_e5871 + 1e-38);
        (assign4030_e5873, ((var_fn25_calc_iq__qinvs0_dn2 * var_fn25_calc_iq__qinvs0) + (var_fn25_calc_iq__qinvs0 * var_fn25_calc_iq__qinvs0_dn2)), ((var_fn25_calc_iq__qinvs0_dn4 * var_fn25_calc_iq__qinvs0) + (var_fn25_calc_iq__qinvs0 * var_fn25_calc_iq__qinvs0_dn4)), ((var_fn25_calc_iq__qinvs0_dn7 * var_fn25_calc_iq__qinvs0) + (var_fn25_calc_iq__qinvs0 * var_fn25_calc_iq__qinvs0_dn7)), ((var_fn25_calc_iq__qinvs0_dn16 * var_fn25_calc_iq__qinvs0) + (var_fn25_calc_iq__qinvs0 * var_fn25_calc_iq__qinvs0_dn16)), ((var_fn25_calc_iq__qinvs0_dn17 * var_fn25_calc_iq__qinvs0) + (var_fn25_calc_iq__qinvs0 * var_fn25_calc_iq__qinvs0_dn17)),)
    } else {
        (var_fn25_calc_iq__qs2, var_fn25_calc_iq__qs2_dn2, var_fn25_calc_iq__qs2_dn4, var_fn25_calc_iq__qs2_dn7, var_fn25_calc_iq__qs2_dn16, var_fn25_calc_iq__qs2_dn17,)
    }
};
        var_fn25_calc_iq__qs2 = assign4030_e5875;
        var_fn25_calc_iq__qs2_dn2 = assign4030_e5875_d_n2;
        var_fn25_calc_iq__qs2_dn4 = assign4030_e5875_d_n4;
        var_fn25_calc_iq__qs2_dn7 = assign4030_e5875_d_n7;
        var_fn25_calc_iq__qs2_dn16 = assign4030_e5875_d_n16;
        var_fn25_calc_iq__qs2_dn17 = assign4030_e5875_d_n17;
        var_fn25_calc_iq__qs2_rv = 0.0;

        let (assign4040_e5883, assign4040_e5883_d_n2, assign4040_e5883_d_n4, assign4040_e5883_d_n7, assign4040_e5883_d_n16, assign4040_e5883_d_n17,) = {
    if (var_guard24 != 0.0) {
        let assign4040_e5879: f64 = (var_fn25_calc_iq__qs2 * var_fn25_calc_iq__qinvs0);
        let assign4040_e5881: f64 = (assign4040_e5879 + 1e-57);
        (assign4040_e5881, ((var_fn25_calc_iq__qs2_dn2 * var_fn25_calc_iq__qinvs0) + (var_fn25_calc_iq__qs2 * var_fn25_calc_iq__qinvs0_dn2)), ((var_fn25_calc_iq__qs2_dn4 * var_fn25_calc_iq__qinvs0) + (var_fn25_calc_iq__qs2 * var_fn25_calc_iq__qinvs0_dn4)), ((var_fn25_calc_iq__qs2_dn7 * var_fn25_calc_iq__qinvs0) + (var_fn25_calc_iq__qs2 * var_fn25_calc_iq__qinvs0_dn7)), ((var_fn25_calc_iq__qs2_dn16 * var_fn25_calc_iq__qinvs0) + (var_fn25_calc_iq__qs2 * var_fn25_calc_iq__qinvs0_dn16)), ((var_fn25_calc_iq__qs2_dn17 * var_fn25_calc_iq__qinvs0) + (var_fn25_calc_iq__qs2 * var_fn25_calc_iq__qinvs0_dn17)),)
    } else {
        (var_fn25_calc_iq__qs3, var_fn25_calc_iq__qs3_dn2, var_fn25_calc_iq__qs3_dn4, var_fn25_calc_iq__qs3_dn7, var_fn25_calc_iq__qs3_dn16, var_fn25_calc_iq__qs3_dn17,)
    }
};
        var_fn25_calc_iq__qs3 = assign4040_e5883;
        var_fn25_calc_iq__qs3_dn2 = assign4040_e5883_d_n2;
        var_fn25_calc_iq__qs3_dn4 = assign4040_e5883_d_n4;
        var_fn25_calc_iq__qs3_dn7 = assign4040_e5883_d_n7;
        var_fn25_calc_iq__qs3_dn16 = assign4040_e5883_d_n16;
        var_fn25_calc_iq__qs3_dn17 = assign4040_e5883_d_n17;
        var_fn25_calc_iq__qs3_rv = 0.0;

        let (assign4050_e5891, assign4050_e5891_d_n2, assign4050_e5891_d_n4, assign4050_e5891_d_n7, assign4050_e5891_d_n16, assign4050_e5891_d_n17,) = {
    if (var_guard24 != 0.0) {
        let assign4050_e5887: f64 = (var_fn25_calc_iq__qinvd0 * var_fn25_calc_iq__qinvd0);
        let assign4050_e5889: f64 = (assign4050_e5887 + 1e-38);
        (assign4050_e5889, ((var_fn25_calc_iq__qinvd0_dn2 * var_fn25_calc_iq__qinvd0) + (var_fn25_calc_iq__qinvd0 * var_fn25_calc_iq__qinvd0_dn2)), ((var_fn25_calc_iq__qinvd0_dn4 * var_fn25_calc_iq__qinvd0) + (var_fn25_calc_iq__qinvd0 * var_fn25_calc_iq__qinvd0_dn4)), ((var_fn25_calc_iq__qinvd0_dn7 * var_fn25_calc_iq__qinvd0) + (var_fn25_calc_iq__qinvd0 * var_fn25_calc_iq__qinvd0_dn7)), ((var_fn25_calc_iq__qinvd0_dn16 * var_fn25_calc_iq__qinvd0) + (var_fn25_calc_iq__qinvd0 * var_fn25_calc_iq__qinvd0_dn16)), ((var_fn25_calc_iq__qinvd0_dn17 * var_fn25_calc_iq__qinvd0) + (var_fn25_calc_iq__qinvd0 * var_fn25_calc_iq__qinvd0_dn17)),)
    } else {
        (var_fn25_calc_iq__qd2, var_fn25_calc_iq__qd2_dn2, var_fn25_calc_iq__qd2_dn4, var_fn25_calc_iq__qd2_dn7, var_fn25_calc_iq__qd2_dn16, var_fn25_calc_iq__qd2_dn17,)
    }
};
        var_fn25_calc_iq__qd2 = assign4050_e5891;
        var_fn25_calc_iq__qd2_dn2 = assign4050_e5891_d_n2;
        var_fn25_calc_iq__qd2_dn4 = assign4050_e5891_d_n4;
        var_fn25_calc_iq__qd2_dn7 = assign4050_e5891_d_n7;
        var_fn25_calc_iq__qd2_dn16 = assign4050_e5891_d_n16;
        var_fn25_calc_iq__qd2_dn17 = assign4050_e5891_d_n17;
        var_fn25_calc_iq__qd2_rv = 0.0;

        let (assign4060_e5899, assign4060_e5899_d_n2, assign4060_e5899_d_n4, assign4060_e5899_d_n7, assign4060_e5899_d_n16, assign4060_e5899_d_n17,) = {
    if (var_guard24 != 0.0) {
        let assign4060_e5895: f64 = (var_fn25_calc_iq__qd2 * var_fn25_calc_iq__qinvd0);
        let assign4060_e5897: f64 = (assign4060_e5895 + 1e-57);
        (assign4060_e5897, ((var_fn25_calc_iq__qd2_dn2 * var_fn25_calc_iq__qinvd0) + (var_fn25_calc_iq__qd2 * var_fn25_calc_iq__qinvd0_dn2)), ((var_fn25_calc_iq__qd2_dn4 * var_fn25_calc_iq__qinvd0) + (var_fn25_calc_iq__qd2 * var_fn25_calc_iq__qinvd0_dn4)), ((var_fn25_calc_iq__qd2_dn7 * var_fn25_calc_iq__qinvd0) + (var_fn25_calc_iq__qd2 * var_fn25_calc_iq__qinvd0_dn7)), ((var_fn25_calc_iq__qd2_dn16 * var_fn25_calc_iq__qinvd0) + (var_fn25_calc_iq__qd2 * var_fn25_calc_iq__qinvd0_dn16)), ((var_fn25_calc_iq__qd2_dn17 * var_fn25_calc_iq__qinvd0) + (var_fn25_calc_iq__qd2 * var_fn25_calc_iq__qinvd0_dn17)),)
    } else {
        (var_fn25_calc_iq__qd3, var_fn25_calc_iq__qd3_dn2, var_fn25_calc_iq__qd3_dn4, var_fn25_calc_iq__qd3_dn7, var_fn25_calc_iq__qd3_dn16, var_fn25_calc_iq__qd3_dn17,)
    }
};
        var_fn25_calc_iq__qd3 = assign4060_e5899;
        var_fn25_calc_iq__qd3_dn2 = assign4060_e5899_d_n2;
        var_fn25_calc_iq__qd3_dn4 = assign4060_e5899_d_n4;
        var_fn25_calc_iq__qd3_dn7 = assign4060_e5899_d_n7;
        var_fn25_calc_iq__qd3_dn16 = assign4060_e5899_d_n16;
        var_fn25_calc_iq__qd3_dn17 = assign4060_e5899_d_n17;
        var_fn25_calc_iq__qd3_rv = 0.0;

        let (assign4070_e5907, assign4070_e5907_d_n2, assign4070_e5907_d_n4, assign4070_e5907_d_n7, assign4070_e5907_d_n16, assign4070_e5907_d_n17,) = {
    if (var_guard24 != 0.0) {
        let assign4070_e5903: f64 = (var_fn25_calc_iq__qinvs0 * var_fn25_calc_iq__qinvd0);
        let assign4070_e5905: f64 = (assign4070_e5903 + 1e-38);
        (assign4070_e5905, ((var_fn25_calc_iq__qinvs0_dn2 * var_fn25_calc_iq__qinvd0) + (var_fn25_calc_iq__qinvs0 * var_fn25_calc_iq__qinvd0_dn2)), ((var_fn25_calc_iq__qinvs0_dn4 * var_fn25_calc_iq__qinvd0) + (var_fn25_calc_iq__qinvs0 * var_fn25_calc_iq__qinvd0_dn4)), ((var_fn25_calc_iq__qinvs0_dn7 * var_fn25_calc_iq__qinvd0) + (var_fn25_calc_iq__qinvs0 * var_fn25_calc_iq__qinvd0_dn7)), ((var_fn25_calc_iq__qinvs0_dn16 * var_fn25_calc_iq__qinvd0) + (var_fn25_calc_iq__qinvs0 * var_fn25_calc_iq__qinvd0_dn16)), ((var_fn25_calc_iq__qinvs0_dn17 * var_fn25_calc_iq__qinvd0) + (var_fn25_calc_iq__qinvs0 * var_fn25_calc_iq__qinvd0_dn17)),)
    } else {
        (var_fn25_calc_iq__qsqd, var_fn25_calc_iq__qsqd_dn2, var_fn25_calc_iq__qsqd_dn4, var_fn25_calc_iq__qsqd_dn7, var_fn25_calc_iq__qsqd_dn16, var_fn25_calc_iq__qsqd_dn17,)
    }
};
        var_fn25_calc_iq__qsqd = assign4070_e5907;
        var_fn25_calc_iq__qsqd_dn2 = assign4070_e5907_d_n2;
        var_fn25_calc_iq__qsqd_dn4 = assign4070_e5907_d_n4;
        var_fn25_calc_iq__qsqd_dn7 = assign4070_e5907_d_n7;
        var_fn25_calc_iq__qsqd_dn16 = assign4070_e5907_d_n16;
        var_fn25_calc_iq__qsqd_dn17 = assign4070_e5907_d_n17;
        var_fn25_calc_iq__qsqd_rv = 0.0;

        let (assign4080_e5925, assign4080_e5925_d_n2, assign4080_e5925_d_n4, assign4080_e5925_d_n7, assign4080_e5925_d_n16, assign4080_e5925_d_n17,) = {
    if (var_guard24 != 0.0) {
        let assign4080_e5911: f64 = (2.0 / 3.0);
        let assign4080_e5914: f64 = (var_fn25_calc_iq__qs2 + var_fn25_calc_iq__qd2);
        let assign4080_e5916: f64 = (assign4080_e5914 + var_fn25_calc_iq__qsqd);
        let assign4080_e5917: f64 = (assign4080_e5911 * assign4080_e5916);
        let assign4080_e5920: f64 = (var_fn25_calc_iq__qinvs0 + var_fn25_calc_iq__qinvd0);
        let assign4080_e5922: f64 = (assign4080_e5920 + 2e-19);
        let assign4080_e5923: f64 = (assign4080_e5917 / assign4080_e5922);
        (assign4080_e5923, ((((assign4080_e5911 * ((var_fn25_calc_iq__qs2_dn2 + var_fn25_calc_iq__qd2_dn2) + var_fn25_calc_iq__qsqd_dn2)) * assign4080_e5922) - (assign4080_e5917 * (var_fn25_calc_iq__qinvs0_dn2 + var_fn25_calc_iq__qinvd0_dn2))) / (assign4080_e5922 * assign4080_e5922)), ((((assign4080_e5911 * ((var_fn25_calc_iq__qs2_dn4 + var_fn25_calc_iq__qd2_dn4) + var_fn25_calc_iq__qsqd_dn4)) * assign4080_e5922) - (assign4080_e5917 * (var_fn25_calc_iq__qinvs0_dn4 + var_fn25_calc_iq__qinvd0_dn4))) / (assign4080_e5922 * assign4080_e5922)), ((((assign4080_e5911 * ((var_fn25_calc_iq__qs2_dn7 + var_fn25_calc_iq__qd2_dn7) + var_fn25_calc_iq__qsqd_dn7)) * assign4080_e5922) - (assign4080_e5917 * (var_fn25_calc_iq__qinvs0_dn7 + var_fn25_calc_iq__qinvd0_dn7))) / (assign4080_e5922 * assign4080_e5922)), ((((assign4080_e5911 * ((var_fn25_calc_iq__qs2_dn16 + var_fn25_calc_iq__qd2_dn16) + var_fn25_calc_iq__qsqd_dn16)) * assign4080_e5922) - (assign4080_e5917 * (var_fn25_calc_iq__qinvs0_dn16 + var_fn25_calc_iq__qinvd0_dn16))) / (assign4080_e5922 * assign4080_e5922)), ((((assign4080_e5911 * ((var_fn25_calc_iq__qs2_dn17 + var_fn25_calc_iq__qd2_dn17) + var_fn25_calc_iq__qsqd_dn17)) * assign4080_e5922) - (assign4080_e5917 * (var_fn25_calc_iq__qinvs0_dn17 + var_fn25_calc_iq__qinvd0_dn17))) / (assign4080_e5922 * assign4080_e5922)),)
    } else {
        (var_fn25_calc_iq__qinvdd, var_fn25_calc_iq__qinvdd_dn2, var_fn25_calc_iq__qinvdd_dn4, var_fn25_calc_iq__qinvdd_dn7, var_fn25_calc_iq__qinvdd_dn16, var_fn25_calc_iq__qinvdd_dn17,)
    }
};
        var_fn25_calc_iq__qinvdd = assign4080_e5925;
        var_fn25_calc_iq__qinvdd_dn2 = assign4080_e5925_d_n2;
        var_fn25_calc_iq__qinvdd_dn4 = assign4080_e5925_d_n4;
        var_fn25_calc_iq__qinvdd_dn7 = assign4080_e5925_d_n7;
        var_fn25_calc_iq__qinvdd_dn16 = assign4080_e5925_d_n16;
        var_fn25_calc_iq__qinvdd_dn17 = assign4080_e5925_d_n17;
        var_fn25_calc_iq__qinvdd_rv = 0.0;

        let (assign4090_e5959, assign4090_e5959_d_n2, assign4090_e5959_d_n4, assign4090_e5959_d_n7, assign4090_e5959_d_n16, assign4090_e5959_d_n17,) = {
    if (var_guard24 != 0.0) {
        let assign4090_e5930: f64 = (2.0 * var_fn25_calc_iq__qs3);
        let assign4090_e5933: f64 = (3.0 * var_fn25_calc_iq__qd3);
        let assign4090_e5934: f64 = (assign4090_e5930 + assign4090_e5933);
        let assign4090_e5937: f64 = (4.0 * var_fn25_calc_iq__qs2);
        let assign4090_e5939: f64 = (assign4090_e5937 * var_fn25_calc_iq__qinvd0);
        let assign4090_e5940: f64 = (assign4090_e5934 + assign4090_e5939);
        let assign4090_e5943: f64 = (6.0 * var_fn25_calc_iq__qd2);
        let assign4090_e5945: f64 = (assign4090_e5943 * var_fn25_calc_iq__qinvs0);
        let assign4090_e5946: f64 = (assign4090_e5940 + assign4090_e5945);
        let assign4090_e5947: f64 = (2.0 * assign4090_e5946);
        let assign4090_e5951: f64 = (var_fn25_calc_iq__qs2 + var_fn25_calc_iq__qd2);
        let assign4090_e5954: f64 = (2.0 * var_fn25_calc_iq__qsqd);
        let assign4090_e5955: f64 = (assign4090_e5951 + assign4090_e5954);
        let assign4090_e5956: f64 = (15.0 * assign4090_e5955);
        let assign4090_e5957: f64 = (assign4090_e5947 / assign4090_e5956);
        (assign4090_e5957, ((((2.0 * ((((2.0 * var_fn25_calc_iq__qs3_dn2) + (3.0 * var_fn25_calc_iq__qd3_dn2)) + (((4.0 * var_fn25_calc_iq__qs2_dn2) * var_fn25_calc_iq__qinvd0) + (assign4090_e5937 * var_fn25_calc_iq__qinvd0_dn2))) + (((6.0 * var_fn25_calc_iq__qd2_dn2) * var_fn25_calc_iq__qinvs0) + (assign4090_e5943 * var_fn25_calc_iq__qinvs0_dn2)))) * assign4090_e5956) - (assign4090_e5947 * (15.0 * ((var_fn25_calc_iq__qs2_dn2 + var_fn25_calc_iq__qd2_dn2) + (2.0 * var_fn25_calc_iq__qsqd_dn2))))) / (assign4090_e5956 * assign4090_e5956)), ((((2.0 * ((((2.0 * var_fn25_calc_iq__qs3_dn4) + (3.0 * var_fn25_calc_iq__qd3_dn4)) + (((4.0 * var_fn25_calc_iq__qs2_dn4) * var_fn25_calc_iq__qinvd0) + (assign4090_e5937 * var_fn25_calc_iq__qinvd0_dn4))) + (((6.0 * var_fn25_calc_iq__qd2_dn4) * var_fn25_calc_iq__qinvs0) + (assign4090_e5943 * var_fn25_calc_iq__qinvs0_dn4)))) * assign4090_e5956) - (assign4090_e5947 * (15.0 * ((var_fn25_calc_iq__qs2_dn4 + var_fn25_calc_iq__qd2_dn4) + (2.0 * var_fn25_calc_iq__qsqd_dn4))))) / (assign4090_e5956 * assign4090_e5956)), ((((2.0 * ((((2.0 * var_fn25_calc_iq__qs3_dn7) + (3.0 * var_fn25_calc_iq__qd3_dn7)) + (((4.0 * var_fn25_calc_iq__qs2_dn7) * var_fn25_calc_iq__qinvd0) + (assign4090_e5937 * var_fn25_calc_iq__qinvd0_dn7))) + (((6.0 * var_fn25_calc_iq__qd2_dn7) * var_fn25_calc_iq__qinvs0) + (assign4090_e5943 * var_fn25_calc_iq__qinvs0_dn7)))) * assign4090_e5956) - (assign4090_e5947 * (15.0 * ((var_fn25_calc_iq__qs2_dn7 + var_fn25_calc_iq__qd2_dn7) + (2.0 * var_fn25_calc_iq__qsqd_dn7))))) / (assign4090_e5956 * assign4090_e5956)), ((((2.0 * ((((2.0 * var_fn25_calc_iq__qs3_dn16) + (3.0 * var_fn25_calc_iq__qd3_dn16)) + (((4.0 * var_fn25_calc_iq__qs2_dn16) * var_fn25_calc_iq__qinvd0) + (assign4090_e5937 * var_fn25_calc_iq__qinvd0_dn16))) + (((6.0 * var_fn25_calc_iq__qd2_dn16) * var_fn25_calc_iq__qinvs0) + (assign4090_e5943 * var_fn25_calc_iq__qinvs0_dn16)))) * assign4090_e5956) - (assign4090_e5947 * (15.0 * ((var_fn25_calc_iq__qs2_dn16 + var_fn25_calc_iq__qd2_dn16) + (2.0 * var_fn25_calc_iq__qsqd_dn16))))) / (assign4090_e5956 * assign4090_e5956)), ((((2.0 * ((((2.0 * var_fn25_calc_iq__qs3_dn17) + (3.0 * var_fn25_calc_iq__qd3_dn17)) + (((4.0 * var_fn25_calc_iq__qs2_dn17) * var_fn25_calc_iq__qinvd0) + (assign4090_e5937 * var_fn25_calc_iq__qinvd0_dn17))) + (((6.0 * var_fn25_calc_iq__qd2_dn17) * var_fn25_calc_iq__qinvs0) + (assign4090_e5943 * var_fn25_calc_iq__qinvs0_dn17)))) * assign4090_e5956) - (assign4090_e5947 * (15.0 * ((var_fn25_calc_iq__qs2_dn17 + var_fn25_calc_iq__qd2_dn17) + (2.0 * var_fn25_calc_iq__qsqd_dn17))))) / (assign4090_e5956 * assign4090_e5956)),)
    } else {
        (var_fn25_calc_iq__qd1, var_fn25_calc_iq__qd1_dn2, var_fn25_calc_iq__qd1_dn4, var_fn25_calc_iq__qd1_dn7, var_fn25_calc_iq__qd1_dn16, var_fn25_calc_iq__qd1_dn17,)
    }
};
        var_fn25_calc_iq__qd1 = assign4090_e5959;
        var_fn25_calc_iq__qd1_dn2 = assign4090_e5959_d_n2;
        var_fn25_calc_iq__qd1_dn4 = assign4090_e5959_d_n4;
        var_fn25_calc_iq__qd1_dn7 = assign4090_e5959_d_n7;
        var_fn25_calc_iq__qd1_dn16 = assign4090_e5959_d_n16;
        var_fn25_calc_iq__qd1_dn17 = assign4090_e5959_d_n17;
        var_fn25_calc_iq__qd1_rv = 0.0;

        let (assign4100_e5965, assign4100_e5965_d_n2, assign4100_e5965_d_n4, assign4100_e5965_d_n7, assign4100_e5965_d_n16, assign4100_e5965_d_n17,) = {
    if (var_guard24 != 0.0) {
        let assign4100_e5963: f64 = (var_fn25_calc_iq__qinvdd - var_fn25_calc_iq__qd1);
        (assign4100_e5963, (var_fn25_calc_iq__qinvdd_dn2 - var_fn25_calc_iq__qd1_dn2), (var_fn25_calc_iq__qinvdd_dn4 - var_fn25_calc_iq__qd1_dn4), (var_fn25_calc_iq__qinvdd_dn7 - var_fn25_calc_iq__qd1_dn7), (var_fn25_calc_iq__qinvdd_dn16 - var_fn25_calc_iq__qd1_dn16), (var_fn25_calc_iq__qinvdd_dn17 - var_fn25_calc_iq__qd1_dn17),)
    } else {
        (var_fn25_calc_iq__qs, var_fn25_calc_iq__qs_dn2, var_fn25_calc_iq__qs_dn4, var_fn25_calc_iq__qs_dn7, var_fn25_calc_iq__qs_dn16, var_fn25_calc_iq__qs_dn17,)
    }
};
        var_fn25_calc_iq__qs = assign4100_e5965;
        var_fn25_calc_iq__qs_dn2 = assign4100_e5965_d_n2;
        var_fn25_calc_iq__qs_dn4 = assign4100_e5965_d_n4;
        var_fn25_calc_iq__qs_dn7 = assign4100_e5965_d_n7;
        var_fn25_calc_iq__qs_dn16 = assign4100_e5965_d_n16;
        var_fn25_calc_iq__qs_dn17 = assign4100_e5965_d_n17;
        var_fn25_calc_iq__qs_rv = 0.0;

        let (assign4110_e5969, assign4110_e5969_d_n2, assign4110_e5969_d_n4, assign4110_e5969_d_n7, assign4110_e5969_d_n16, assign4110_e5969_d_n17,) = {
    if (var_guard24 != 0.0) {
        (var_fn25_calc_iq__qd1, var_fn25_calc_iq__qd1_dn2, var_fn25_calc_iq__qd1_dn4, var_fn25_calc_iq__qd1_dn7, var_fn25_calc_iq__qd1_dn16, var_fn25_calc_iq__qd1_dn17,)
    } else {
        (var_fn25_calc_iq__qd, var_fn25_calc_iq__qd_dn2, var_fn25_calc_iq__qd_dn4, var_fn25_calc_iq__qd_dn7, var_fn25_calc_iq__qd_dn16, var_fn25_calc_iq__qd_dn17,)
    }
};
        var_fn25_calc_iq__qd = assign4110_e5969;
        var_fn25_calc_iq__qd_dn2 = assign4110_e5969_d_n2;
        var_fn25_calc_iq__qd_dn4 = assign4110_e5969_d_n4;
        var_fn25_calc_iq__qd_dn7 = assign4110_e5969_d_n7;
        var_fn25_calc_iq__qd_dn16 = assign4110_e5969_d_n16;
        var_fn25_calc_iq__qd_dn17 = assign4110_e5969_d_n17;
        var_fn25_calc_iq__qd_rv = 0.0;

        let (assign4120_e5983, assign4120_e5983_d_n2, assign4120_e5983_d_n4, assign4120_e5983_d_n7, assign4120_e5983_d_n16, assign4120_e5983_d_n17,) = {
    if (var_guard24 != 0.0) {
        let assign4120_e5973: f64 = (var_fn25_calc_iq__w * var_fn25_calc_iq__ngf);
        let assign4120_e5975: f64 = (assign4120_e5973 * var_fn25_calc_iq__lin);
        let assign4120_e5977: f64 = (assign4120_e5975 * var_fn25_calc_iq__type);
        let assign4120_e5979: f64 = (assign4120_e5977 * var_fn25_calc_iq__qs);
        let assign4120_e5981: f64 = (assign4120_e5979 * var_fn25_calc_iq__trapfracdl);
        (assign4120_e5981, ((assign4120_e5977 * var_fn25_calc_iq__qs_dn2) * var_fn25_calc_iq__trapfracdl), ((assign4120_e5977 * var_fn25_calc_iq__qs_dn4) * var_fn25_calc_iq__trapfracdl), ((assign4120_e5977 * var_fn25_calc_iq__qs_dn7) * var_fn25_calc_iq__trapfracdl), ((assign4120_e5977 * var_fn25_calc_iq__qs_dn16) * var_fn25_calc_iq__trapfracdl), ((assign4120_e5977 * var_fn25_calc_iq__qs_dn17) * var_fn25_calc_iq__trapfracdl),)
    } else {
        (var_fn25_calc_iq__qgsout, var_fn25_calc_iq__qgsout_dn2, var_fn25_calc_iq__qgsout_dn4, var_fn25_calc_iq__qgsout_dn7, var_fn25_calc_iq__qgsout_dn16, var_fn25_calc_iq__qgsout_dn17,)
    }
};
        var_fn25_calc_iq__qgsout = assign4120_e5983;
        var_fn25_calc_iq__qgsout_dn2 = assign4120_e5983_d_n2;
        var_fn25_calc_iq__qgsout_dn4 = assign4120_e5983_d_n4;
        var_fn25_calc_iq__qgsout_dn7 = assign4120_e5983_d_n7;
        var_fn25_calc_iq__qgsout_dn16 = assign4120_e5983_d_n16;
        var_fn25_calc_iq__qgsout_dn17 = assign4120_e5983_d_n17;
        var_fn25_calc_iq__qgsout_rv = 0.0;

        let (assign4130_e5997, assign4130_e5997_d_n2, assign4130_e5997_d_n4, assign4130_e5997_d_n7, assign4130_e5997_d_n16, assign4130_e5997_d_n17,) = {
    if (var_guard24 != 0.0) {
        let assign4130_e5987: f64 = (var_fn25_calc_iq__w * var_fn25_calc_iq__ngf);
        let assign4130_e5989: f64 = (assign4130_e5987 * var_fn25_calc_iq__lin);
        let assign4130_e5991: f64 = (assign4130_e5989 * var_fn25_calc_iq__type);
        let assign4130_e5993: f64 = (assign4130_e5991 * var_fn25_calc_iq__qd);
        let assign4130_e5995: f64 = (assign4130_e5993 * var_fn25_calc_iq__trapfracdl);
        (assign4130_e5995, ((assign4130_e5991 * var_fn25_calc_iq__qd_dn2) * var_fn25_calc_iq__trapfracdl), ((assign4130_e5991 * var_fn25_calc_iq__qd_dn4) * var_fn25_calc_iq__trapfracdl), ((assign4130_e5991 * var_fn25_calc_iq__qd_dn7) * var_fn25_calc_iq__trapfracdl), ((assign4130_e5991 * var_fn25_calc_iq__qd_dn16) * var_fn25_calc_iq__trapfracdl), ((assign4130_e5991 * var_fn25_calc_iq__qd_dn17) * var_fn25_calc_iq__trapfracdl),)
    } else {
        (var_fn25_calc_iq__qgdout, var_fn25_calc_iq__qgdout_dn2, var_fn25_calc_iq__qgdout_dn4, var_fn25_calc_iq__qgdout_dn7, var_fn25_calc_iq__qgdout_dn16, var_fn25_calc_iq__qgdout_dn17,)
    }
};
        var_fn25_calc_iq__qgdout = assign4130_e5997;
        var_fn25_calc_iq__qgdout_dn2 = assign4130_e5997_d_n2;
        var_fn25_calc_iq__qgdout_dn4 = assign4130_e5997_d_n4;
        var_fn25_calc_iq__qgdout_dn7 = assign4130_e5997_d_n7;
        var_fn25_calc_iq__qgdout_dn16 = assign4130_e5997_d_n16;
        var_fn25_calc_iq__qgdout_dn17 = assign4130_e5997_d_n17;
        var_fn25_calc_iq__qgdout_rv = 0.0;

        let assign4140_e6000: f64 = if var_fn25_calc_iq__qcbflag == 1.0 { 1.0 } else { 0.0 };
        var_guard51 = assign4140_e6000;
        var_guard51_rv = 0.0;

        let (assign4150_e6016, assign4150_e6016_d_n2, assign4150_e6016_d_n4, assign4150_e6016_d_n7, assign4150_e6016_d_n16,) = {
    if ((var_guard24 != 0.0) && (var_guard51 != 0.0)) {
        let assign4150_e6008: f64 = (p.p51 * 0.5);
        let assign4150_e6010: f64 = (assign4150_e6008 * var_fn25_calc_iq__alpha_phit);
        let assign4150_e6011: f64 = (var_fn25_calc_iq__vtof - assign4150_e6010);
        let assign4150_e6012: f64 = (var_fn25_calc_iq__vcin - assign4150_e6011);
        let assign4150_e6014: f64 = (assign4150_e6012 / var_fn25_calc_iq__two_n_phit0);
        (assign4150_e6014, (var_fn25_calc_iq__vcin_dn2 / var_fn25_calc_iq__two_n_phit0), ((((-(var_fn25_calc_iq__vtof_dn4 - (assign4150_e6008 * var_fn25_calc_iq__alpha_phit_dn4))) * var_fn25_calc_iq__two_n_phit0) - (assign4150_e6012 * var_fn25_calc_iq__two_n_phit0_dn4)) / (var_fn25_calc_iq__two_n_phit0 * var_fn25_calc_iq__two_n_phit0)), (var_fn25_calc_iq__vcin_dn7 / var_fn25_calc_iq__two_n_phit0), (var_fn25_calc_iq__vcin_dn16 / var_fn25_calc_iq__two_n_phit0),)
    } else {
        (var_fn25_calc_iq__etac, var_fn25_calc_iq__etac_dn2, var_fn25_calc_iq__etac_dn4, var_fn25_calc_iq__etac_dn7, var_fn25_calc_iq__etac_dn16,)
    }
};
        var_fn25_calc_iq__etac = assign4150_e6016;
        var_fn25_calc_iq__etac_dn2 = assign4150_e6016_d_n2;
        var_fn25_calc_iq__etac_dn4 = assign4150_e6016_d_n4;
        var_fn25_calc_iq__etac_dn7 = assign4150_e6016_d_n7;
        var_fn25_calc_iq__etac_dn16 = assign4150_e6016_d_n16;
        var_fn25_calc_iq__etac_rv = 0.0;

        let assign4160_e6019: f64 = if var_fn25_calc_iq__etac > 50.0 { 1.0 } else { 0.0 };
        var_guard52 = assign4160_e6019;
        var_guard52_rv = 0.0;

        let (assign4170_e6027, assign4170_e6027_d_n2, assign4170_e6027_d_n3, assign4170_e6027_d_n4, assign4170_e6027_d_n7, assign4170_e6027_d_n16, assign4170_e6027_d_n17,) = {
    if (((var_guard24 != 0.0) && (var_guard51 != 0.0)) && (var_guard52 != 0.0)) {
        (var_fn25_calc_iq__etac, var_fn25_calc_iq__etac_dn2, 0.0, var_fn25_calc_iq__etac_dn4, var_fn25_calc_iq__etac_dn7, var_fn25_calc_iq__etac_dn16, 0.0,)
    } else {
        (var_fn25_calc_iq__exparg, var_fn25_calc_iq__exparg_dn2, var_fn25_calc_iq__exparg_dn3, var_fn25_calc_iq__exparg_dn4, var_fn25_calc_iq__exparg_dn7, var_fn25_calc_iq__exparg_dn16, var_fn25_calc_iq__exparg_dn17,)
    }
};
        var_fn25_calc_iq__exparg = assign4170_e6027;
        var_fn25_calc_iq__exparg_dn2 = assign4170_e6027_d_n2;
        var_fn25_calc_iq__exparg_dn3 = assign4170_e6027_d_n3;
        var_fn25_calc_iq__exparg_dn4 = assign4170_e6027_d_n4;
        var_fn25_calc_iq__exparg_dn7 = assign4170_e6027_d_n7;
        var_fn25_calc_iq__exparg_dn16 = assign4170_e6027_d_n16;
        var_fn25_calc_iq__exparg_dn17 = assign4170_e6027_d_n17;
        var_fn25_calc_iq__exparg_rv = 0.0;

        *var_fn25_calc_iq__etac_slot = var_fn25_calc_iq__etac;
        *var_fn25_calc_iq__etac_dn16_slot = var_fn25_calc_iq__etac_dn16;
        *var_fn25_calc_iq__etac_dn2_slot = var_fn25_calc_iq__etac_dn2;
        *var_fn25_calc_iq__etac_dn4_slot = var_fn25_calc_iq__etac_dn4;
        *var_fn25_calc_iq__etac_dn7_slot = var_fn25_calc_iq__etac_dn7;
        *var_fn25_calc_iq__etac_rv_slot = var_fn25_calc_iq__etac_rv;
        *var_fn25_calc_iq__etad0_slot = var_fn25_calc_iq__etad0;
        *var_fn25_calc_iq__etad0_dn16_slot = var_fn25_calc_iq__etad0_dn16;
        *var_fn25_calc_iq__etad0_dn17_slot = var_fn25_calc_iq__etad0_dn17;
        *var_fn25_calc_iq__etad0_dn2_slot = var_fn25_calc_iq__etad0_dn2;
        *var_fn25_calc_iq__etad0_dn4_slot = var_fn25_calc_iq__etad0_dn4;
        *var_fn25_calc_iq__etad0_dn7_slot = var_fn25_calc_iq__etad0_dn7;
        *var_fn25_calc_iq__etad0_rv_slot = var_fn25_calc_iq__etad0_rv;
        *var_fn25_calc_iq__etas0_slot = var_fn25_calc_iq__etas0;
        *var_fn25_calc_iq__etas0_dn16_slot = var_fn25_calc_iq__etas0_dn16;
        *var_fn25_calc_iq__etas0_dn17_slot = var_fn25_calc_iq__etas0_dn17;
        *var_fn25_calc_iq__etas0_dn2_slot = var_fn25_calc_iq__etas0_dn2;
        *var_fn25_calc_iq__etas0_dn4_slot = var_fn25_calc_iq__etas0_dn4;
        *var_fn25_calc_iq__etas0_dn7_slot = var_fn25_calc_iq__etas0_dn7;
        *var_fn25_calc_iq__etas0_rv_slot = var_fn25_calc_iq__etas0_rv;
        *var_fn25_calc_iq__exparg_slot = var_fn25_calc_iq__exparg;
        *var_fn25_calc_iq__exparg0_slot = var_fn25_calc_iq__exparg0;
        *var_fn25_calc_iq__exparg0_dn16_slot = var_fn25_calc_iq__exparg0_dn16;
        *var_fn25_calc_iq__exparg0_dn17_slot = var_fn25_calc_iq__exparg0_dn17;
        *var_fn25_calc_iq__exparg0_dn2_slot = var_fn25_calc_iq__exparg0_dn2;
        *var_fn25_calc_iq__exparg0_dn4_slot = var_fn25_calc_iq__exparg0_dn4;
        *var_fn25_calc_iq__exparg0_dn7_slot = var_fn25_calc_iq__exparg0_dn7;
        *var_fn25_calc_iq__exparg0_rv_slot = var_fn25_calc_iq__exparg0_rv;
        *var_fn25_calc_iq__exparg_dn16_slot = var_fn25_calc_iq__exparg_dn16;
        *var_fn25_calc_iq__exparg_dn17_slot = var_fn25_calc_iq__exparg_dn17;
        *var_fn25_calc_iq__exparg_dn2_slot = var_fn25_calc_iq__exparg_dn2;
        *var_fn25_calc_iq__exparg_dn3_slot = var_fn25_calc_iq__exparg_dn3;
        *var_fn25_calc_iq__exparg_dn4_slot = var_fn25_calc_iq__exparg_dn4;
        *var_fn25_calc_iq__exparg_dn7_slot = var_fn25_calc_iq__exparg_dn7;
        *var_fn25_calc_iq__exparg_rv_slot = var_fn25_calc_iq__exparg_rv;
        *var_fn25_calc_iq__ffd0_slot = var_fn25_calc_iq__ffd0;
        *var_fn25_calc_iq__ffd0_dn16_slot = var_fn25_calc_iq__ffd0_dn16;
        *var_fn25_calc_iq__ffd0_dn17_slot = var_fn25_calc_iq__ffd0_dn17;
        *var_fn25_calc_iq__ffd0_dn2_slot = var_fn25_calc_iq__ffd0_dn2;
        *var_fn25_calc_iq__ffd0_dn4_slot = var_fn25_calc_iq__ffd0_dn4;
        *var_fn25_calc_iq__ffd0_dn7_slot = var_fn25_calc_iq__ffd0_dn7;
        *var_fn25_calc_iq__ffd0_rv_slot = var_fn25_calc_iq__ffd0_rv;
        *var_fn25_calc_iq__ffs0_slot = var_fn25_calc_iq__ffs0;
        *var_fn25_calc_iq__ffs0_dn16_slot = var_fn25_calc_iq__ffs0_dn16;
        *var_fn25_calc_iq__ffs0_dn17_slot = var_fn25_calc_iq__ffs0_dn17;
        *var_fn25_calc_iq__ffs0_dn2_slot = var_fn25_calc_iq__ffs0_dn2;
        *var_fn25_calc_iq__ffs0_dn4_slot = var_fn25_calc_iq__ffs0_dn4;
        *var_fn25_calc_iq__ffs0_dn7_slot = var_fn25_calc_iq__ffs0_dn7;
        *var_fn25_calc_iq__ffs0_rv_slot = var_fn25_calc_iq__ffs0_rv;
        *var_fn25_calc_iq__qd_slot = var_fn25_calc_iq__qd;
        *var_fn25_calc_iq__qd1_slot = var_fn25_calc_iq__qd1;
        *var_fn25_calc_iq__qd1_dn16_slot = var_fn25_calc_iq__qd1_dn16;
        *var_fn25_calc_iq__qd1_dn17_slot = var_fn25_calc_iq__qd1_dn17;
        *var_fn25_calc_iq__qd1_dn2_slot = var_fn25_calc_iq__qd1_dn2;
        *var_fn25_calc_iq__qd1_dn4_slot = var_fn25_calc_iq__qd1_dn4;
        *var_fn25_calc_iq__qd1_dn7_slot = var_fn25_calc_iq__qd1_dn7;
        *var_fn25_calc_iq__qd1_rv_slot = var_fn25_calc_iq__qd1_rv;
        *var_fn25_calc_iq__qd2_slot = var_fn25_calc_iq__qd2;
        *var_fn25_calc_iq__qd2_dn16_slot = var_fn25_calc_iq__qd2_dn16;
        *var_fn25_calc_iq__qd2_dn17_slot = var_fn25_calc_iq__qd2_dn17;
        *var_fn25_calc_iq__qd2_dn2_slot = var_fn25_calc_iq__qd2_dn2;
        *var_fn25_calc_iq__qd2_dn4_slot = var_fn25_calc_iq__qd2_dn4;
        *var_fn25_calc_iq__qd2_dn7_slot = var_fn25_calc_iq__qd2_dn7;
        *var_fn25_calc_iq__qd2_rv_slot = var_fn25_calc_iq__qd2_rv;
        *var_fn25_calc_iq__qd3_slot = var_fn25_calc_iq__qd3;
        *var_fn25_calc_iq__qd3_dn16_slot = var_fn25_calc_iq__qd3_dn16;
        *var_fn25_calc_iq__qd3_dn17_slot = var_fn25_calc_iq__qd3_dn17;
        *var_fn25_calc_iq__qd3_dn2_slot = var_fn25_calc_iq__qd3_dn2;
        *var_fn25_calc_iq__qd3_dn4_slot = var_fn25_calc_iq__qd3_dn4;
        *var_fn25_calc_iq__qd3_dn7_slot = var_fn25_calc_iq__qd3_dn7;
        *var_fn25_calc_iq__qd3_rv_slot = var_fn25_calc_iq__qd3_rv;
        *var_fn25_calc_iq__qd_dn16_slot = var_fn25_calc_iq__qd_dn16;
        *var_fn25_calc_iq__qd_dn17_slot = var_fn25_calc_iq__qd_dn17;
        *var_fn25_calc_iq__qd_dn2_slot = var_fn25_calc_iq__qd_dn2;
        *var_fn25_calc_iq__qd_dn4_slot = var_fn25_calc_iq__qd_dn4;
        *var_fn25_calc_iq__qd_dn7_slot = var_fn25_calc_iq__qd_dn7;
        *var_fn25_calc_iq__qd_rv_slot = var_fn25_calc_iq__qd_rv;
        *var_fn25_calc_iq__qgdout_slot = var_fn25_calc_iq__qgdout;
        *var_fn25_calc_iq__qgdout_dn16_slot = var_fn25_calc_iq__qgdout_dn16;
        *var_fn25_calc_iq__qgdout_dn17_slot = var_fn25_calc_iq__qgdout_dn17;
        *var_fn25_calc_iq__qgdout_dn2_slot = var_fn25_calc_iq__qgdout_dn2;
        *var_fn25_calc_iq__qgdout_dn4_slot = var_fn25_calc_iq__qgdout_dn4;
        *var_fn25_calc_iq__qgdout_dn7_slot = var_fn25_calc_iq__qgdout_dn7;
        *var_fn25_calc_iq__qgdout_rv_slot = var_fn25_calc_iq__qgdout_rv;
        *var_fn25_calc_iq__qgsout_slot = var_fn25_calc_iq__qgsout;
        *var_fn25_calc_iq__qgsout_dn16_slot = var_fn25_calc_iq__qgsout_dn16;
        *var_fn25_calc_iq__qgsout_dn17_slot = var_fn25_calc_iq__qgsout_dn17;
        *var_fn25_calc_iq__qgsout_dn2_slot = var_fn25_calc_iq__qgsout_dn2;
        *var_fn25_calc_iq__qgsout_dn4_slot = var_fn25_calc_iq__qgsout_dn4;
        *var_fn25_calc_iq__qgsout_dn7_slot = var_fn25_calc_iq__qgsout_dn7;
        *var_fn25_calc_iq__qgsout_rv_slot = var_fn25_calc_iq__qgsout_rv;
        *var_fn25_calc_iq__qinvd0_slot = var_fn25_calc_iq__qinvd0;
        *var_fn25_calc_iq__qinvd0_dn16_slot = var_fn25_calc_iq__qinvd0_dn16;
        *var_fn25_calc_iq__qinvd0_dn17_slot = var_fn25_calc_iq__qinvd0_dn17;
        *var_fn25_calc_iq__qinvd0_dn2_slot = var_fn25_calc_iq__qinvd0_dn2;
        *var_fn25_calc_iq__qinvd0_dn4_slot = var_fn25_calc_iq__qinvd0_dn4;
        *var_fn25_calc_iq__qinvd0_dn7_slot = var_fn25_calc_iq__qinvd0_dn7;
        *var_fn25_calc_iq__qinvd0_rv_slot = var_fn25_calc_iq__qinvd0_rv;
        *var_fn25_calc_iq__qinvdd_slot = var_fn25_calc_iq__qinvdd;
        *var_fn25_calc_iq__qinvdd_dn16_slot = var_fn25_calc_iq__qinvdd_dn16;
        *var_fn25_calc_iq__qinvdd_dn17_slot = var_fn25_calc_iq__qinvdd_dn17;
        *var_fn25_calc_iq__qinvdd_dn2_slot = var_fn25_calc_iq__qinvdd_dn2;
        *var_fn25_calc_iq__qinvdd_dn4_slot = var_fn25_calc_iq__qinvdd_dn4;
        *var_fn25_calc_iq__qinvdd_dn7_slot = var_fn25_calc_iq__qinvdd_dn7;
        *var_fn25_calc_iq__qinvdd_rv_slot = var_fn25_calc_iq__qinvdd_rv;
        *var_fn25_calc_iq__qinvs0_slot = var_fn25_calc_iq__qinvs0;
        *var_fn25_calc_iq__qinvs0_dn16_slot = var_fn25_calc_iq__qinvs0_dn16;
        *var_fn25_calc_iq__qinvs0_dn17_slot = var_fn25_calc_iq__qinvs0_dn17;
        *var_fn25_calc_iq__qinvs0_dn2_slot = var_fn25_calc_iq__qinvs0_dn2;
        *var_fn25_calc_iq__qinvs0_dn4_slot = var_fn25_calc_iq__qinvs0_dn4;
        *var_fn25_calc_iq__qinvs0_dn7_slot = var_fn25_calc_iq__qinvs0_dn7;
        *var_fn25_calc_iq__qinvs0_rv_slot = var_fn25_calc_iq__qinvs0_rv;
        *var_fn25_calc_iq__qs_slot = var_fn25_calc_iq__qs;
        *var_fn25_calc_iq__qs2_slot = var_fn25_calc_iq__qs2;
        *var_fn25_calc_iq__qs2_dn16_slot = var_fn25_calc_iq__qs2_dn16;
        *var_fn25_calc_iq__qs2_dn17_slot = var_fn25_calc_iq__qs2_dn17;
        *var_fn25_calc_iq__qs2_dn2_slot = var_fn25_calc_iq__qs2_dn2;
        *var_fn25_calc_iq__qs2_dn4_slot = var_fn25_calc_iq__qs2_dn4;
        *var_fn25_calc_iq__qs2_dn7_slot = var_fn25_calc_iq__qs2_dn7;
        *var_fn25_calc_iq__qs2_rv_slot = var_fn25_calc_iq__qs2_rv;
        *var_fn25_calc_iq__qs3_slot = var_fn25_calc_iq__qs3;
        *var_fn25_calc_iq__qs3_dn16_slot = var_fn25_calc_iq__qs3_dn16;
        *var_fn25_calc_iq__qs3_dn17_slot = var_fn25_calc_iq__qs3_dn17;
        *var_fn25_calc_iq__qs3_dn2_slot = var_fn25_calc_iq__qs3_dn2;
        *var_fn25_calc_iq__qs3_dn4_slot = var_fn25_calc_iq__qs3_dn4;
        *var_fn25_calc_iq__qs3_dn7_slot = var_fn25_calc_iq__qs3_dn7;
        *var_fn25_calc_iq__qs3_rv_slot = var_fn25_calc_iq__qs3_rv;
        *var_fn25_calc_iq__qs_dn16_slot = var_fn25_calc_iq__qs_dn16;
        *var_fn25_calc_iq__qs_dn17_slot = var_fn25_calc_iq__qs_dn17;
        *var_fn25_calc_iq__qs_dn2_slot = var_fn25_calc_iq__qs_dn2;
        *var_fn25_calc_iq__qs_dn4_slot = var_fn25_calc_iq__qs_dn4;
        *var_fn25_calc_iq__qs_dn7_slot = var_fn25_calc_iq__qs_dn7;
        *var_fn25_calc_iq__qs_rv_slot = var_fn25_calc_iq__qs_rv;
        *var_fn25_calc_iq__qsqd_slot = var_fn25_calc_iq__qsqd;
        *var_fn25_calc_iq__qsqd_dn16_slot = var_fn25_calc_iq__qsqd_dn16;
        *var_fn25_calc_iq__qsqd_dn17_slot = var_fn25_calc_iq__qsqd_dn17;
        *var_fn25_calc_iq__qsqd_dn2_slot = var_fn25_calc_iq__qsqd_dn2;
        *var_fn25_calc_iq__qsqd_dn4_slot = var_fn25_calc_iq__qsqd_dn4;
        *var_fn25_calc_iq__qsqd_dn7_slot = var_fn25_calc_iq__qsqd_dn7;
        *var_fn25_calc_iq__qsqd_rv_slot = var_fn25_calc_iq__qsqd_rv;
        *var_guard45_slot = var_guard45;
        *var_guard45_rv_slot = var_guard45_rv;
        *var_guard46_slot = var_guard46;
        *var_guard46_rv_slot = var_guard46_rv;
        *var_guard47_slot = var_guard47;
        *var_guard47_rv_slot = var_guard47_rv;
        *var_guard48_slot = var_guard48;
        *var_guard48_rv_slot = var_guard48_rv;
        *var_guard49_slot = var_guard49;
        *var_guard49_rv_slot = var_guard49_rv;
        *var_guard50_slot = var_guard50;
        *var_guard50_rv_slot = var_guard50_rv;
        *var_guard51_slot = var_guard51;
        *var_guard51_rv_slot = var_guard51_rv;
        *var_guard52_slot = var_guard52;
        *var_guard52_rv_slot = var_guard52_rv;
    }
}
