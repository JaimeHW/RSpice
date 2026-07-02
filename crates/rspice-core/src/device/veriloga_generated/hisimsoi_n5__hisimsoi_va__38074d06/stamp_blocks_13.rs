#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_79(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign21920_e30694, assign21920_e30694_d_n0, assign21920_e30694_d_n2, assign21920_e30694_d_n6, assign21920_e30694_d_n7, assign21920_e30694_d_n10, assign21920_e30694_d_n11, assign21920_e30694_d_n12, assign21920_e30694_d_n17,) = {
    if (locals.var_guard598 != 0.0) {
        let assign21920_e30688: f64 = (locals.var_wk_jnpt_a * locals.var_wk_mu);
        let assign21920_e30690: f64 = (assign21920_e30688 * locals.var_weff_nf);
        let assign21920_e30692: f64 = (assign21920_e30690 * locals.var_ty);
        (assign21920_e30692, ((((locals.var_wk_jnpt_a_dn0 * locals.var_wk_mu) * locals.var_weff_nf) * locals.var_ty) + (assign21920_e30690 * locals.var_ty_dn0)), ((((locals.var_wk_jnpt_a_dn2 * locals.var_wk_mu) * locals.var_weff_nf) * locals.var_ty) + (assign21920_e30690 * locals.var_ty_dn2)), ((((locals.var_wk_jnpt_a_dn6 * locals.var_wk_mu) * locals.var_weff_nf) * locals.var_ty) + (assign21920_e30690 * locals.var_ty_dn6)), ((((locals.var_wk_jnpt_a_dn7 * locals.var_wk_mu) * locals.var_weff_nf) * locals.var_ty) + (assign21920_e30690 * locals.var_ty_dn7)), ((((locals.var_wk_jnpt_a_dn10 * locals.var_wk_mu) * locals.var_weff_nf) * locals.var_ty) + (assign21920_e30690 * locals.var_ty_dn10)), ((((locals.var_wk_jnpt_a_dn11 * locals.var_wk_mu) * locals.var_weff_nf) * locals.var_ty) + (assign21920_e30690 * locals.var_ty_dn11)), ((((locals.var_wk_jnpt_a_dn12 * locals.var_wk_mu) * locals.var_weff_nf) * locals.var_ty) + (assign21920_e30690 * locals.var_ty_dn12)), ((((locals.var_wk_jnpt_a_dn17 * locals.var_wk_mu) * locals.var_weff_nf) * locals.var_ty) + (assign21920_e30690 * locals.var_ty_dn17)),)
    } else {
        (locals.var_idspt1, locals.var_idspt1_dn0, locals.var_idspt1_dn2, locals.var_idspt1_dn6, locals.var_idspt1_dn7, locals.var_idspt1_dn10, locals.var_idspt1_dn11, locals.var_idspt1_dn12, locals.var_idspt1_dn17,)
    }
};
        locals.var_idspt1 = assign21920_e30694;
        locals.var_idspt1_dn0 = assign21920_e30694_d_n0;
        locals.var_idspt1_dn2 = assign21920_e30694_d_n2;
        locals.var_idspt1_dn6 = assign21920_e30694_d_n6;
        locals.var_idspt1_dn7 = assign21920_e30694_d_n7;
        locals.var_idspt1_dn10 = assign21920_e30694_d_n10;
        locals.var_idspt1_dn11 = assign21920_e30694_d_n11;
        locals.var_idspt1_dn12 = assign21920_e30694_d_n12;
        locals.var_idspt1_dn17 = assign21920_e30694_d_n17;
        locals.var_idspt1_rv = 0.0;

        let (assign21930_e30700, assign21930_e30700_d_n0, assign21930_e30700_d_n2, assign21930_e30700_d_n6, assign21930_e30700_d_n7, assign21930_e30700_d_n10, assign21930_e30700_d_n11, assign21930_e30700_d_n12, assign21930_e30700_d_n17,) = {
    if (locals.var_guard598 != 0.0) {
        let assign21930_e30698: f64 = (locals.var_idsorg + locals.var_idspt1);
        (assign21930_e30698, (locals.var_idsorg_dn0 + locals.var_idspt1_dn0), (locals.var_idsorg_dn2 + locals.var_idspt1_dn2), (locals.var_idsorg_dn6 + locals.var_idspt1_dn6), (locals.var_idsorg_dn7 + locals.var_idspt1_dn7), (locals.var_idsorg_dn10 + locals.var_idspt1_dn10), (locals.var_idsorg_dn11 + locals.var_idspt1_dn11), (locals.var_idsorg_dn12 + locals.var_idspt1_dn12), (locals.var_idsorg_dn17 + locals.var_idspt1_dn17),)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12, locals.var_ids_dn17,)
    }
};
        locals.var_ids = assign21930_e30700;
        locals.var_ids_dn0 = assign21930_e30700_d_n0;
        locals.var_ids_dn2 = assign21930_e30700_d_n2;
        locals.var_ids_dn6 = assign21930_e30700_d_n6;
        locals.var_ids_dn7 = assign21930_e30700_d_n7;
        locals.var_ids_dn10 = assign21930_e30700_d_n10;
        locals.var_ids_dn11 = assign21930_e30700_d_n11;
        locals.var_ids_dn12 = assign21930_e30700_d_n12;
        locals.var_ids_dn17 = assign21930_e30700_d_n17;
        locals.var_ids_rv = 0.0;

        let assign21940_e30703: f64 = (locals.var_idspt0 + locals.var_idspt1);
        locals.var_idspt = assign21940_e30703;
        locals.var_idspt_dn0 = (locals.var_idspt0_dn0 + locals.var_idspt1_dn0);
        locals.var_idspt_dn2 = (locals.var_idspt0_dn2 + locals.var_idspt1_dn2);
        locals.var_idspt_dn6 = (locals.var_idspt0_dn6 + locals.var_idspt1_dn6);
        locals.var_idspt_dn7 = (locals.var_idspt0_dn7 + locals.var_idspt1_dn7);
        locals.var_idspt_dn10 = (locals.var_idspt0_dn10 + locals.var_idspt1_dn10);
        locals.var_idspt_dn11 = (locals.var_idspt0_dn11 + locals.var_idspt1_dn11);
        locals.var_idspt_dn12 = (locals.var_idspt0_dn12 + locals.var_idspt1_dn12);
        locals.var_idspt_dn17 = (locals.var_idspt0_dn17 + locals.var_idspt1_dn17);
        locals.var_idspt_rv = 0.0;

        let assign21950_e30710: f64 = if ((p.p43 == 1.0) || (p.p45 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard669 = assign21950_e30710;
        locals.var_guard669_rv = 0.0;

        let assign21960_e30717: f64 = if ((locals.var_flg_noqi == 1.0) || (p.p25 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard682 = assign21960_e30717;
        locals.var_guard682_rv = 0.0;

        let (assign21970_e30723, assign21970_e30723_d_n0, assign21970_e30723_d_n2, assign21970_e30723_d_n6, assign21970_e30723_d_n7, assign21970_e30723_d_n10, assign21970_e30723_d_n11, assign21970_e30723_d_n12, assign21970_e30723_d_n17,) = {
    if ((locals.var_guard669 != 0.0) && (locals.var_guard682 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn6, locals.var_isub_dn7, locals.var_isub_dn10, locals.var_isub_dn11, locals.var_isub_dn12, locals.var_isub_dn17,)
    }
};
        locals.var_isub = assign21970_e30723;
        locals.var_isub_dn0 = assign21970_e30723_d_n0;
        locals.var_isub_dn2 = assign21970_e30723_d_n2;
        locals.var_isub_dn6 = assign21970_e30723_d_n6;
        locals.var_isub_dn7 = assign21970_e30723_d_n7;
        locals.var_isub_dn10 = assign21970_e30723_d_n10;
        locals.var_isub_dn11 = assign21970_e30723_d_n11;
        locals.var_isub_dn12 = assign21970_e30723_d_n12;
        locals.var_isub_dn17 = assign21970_e30723_d_n17;
        locals.var_isub_rv = 0.0;

        let assign21980_e30730: f64 = if ((p.p117 <= 0.0) || (locals.var_mks_vmax <= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard683 = assign21980_e30730;
        locals.var_guard683_rv = 0.0;

        let (assign21990_e30739, assign21990_e30739_d_n0, assign21990_e30739_d_n2, assign21990_e30739_d_n6, assign21990_e30739_d_n7, assign21990_e30739_d_n10, assign21990_e30739_d_n11, assign21990_e30739_d_n12, assign21990_e30739_d_n17,) = {
    if (((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn6, locals.var_isub_dn7, locals.var_isub_dn10, locals.var_isub_dn11, locals.var_isub_dn12, locals.var_isub_dn17,)
    }
};
        locals.var_isub = assign21990_e30739;
        locals.var_isub_dn0 = assign21990_e30739_d_n0;
        locals.var_isub_dn2 = assign21990_e30739_d_n2;
        locals.var_isub_dn6 = assign21990_e30739_d_n6;
        locals.var_isub_dn7 = assign21990_e30739_d_n7;
        locals.var_isub_dn10 = assign21990_e30739_d_n10;
        locals.var_isub_dn11 = assign21990_e30739_d_n11;
        locals.var_isub_dn12 = assign21990_e30739_d_n12;
        locals.var_isub_dn17 = assign21990_e30739_d_n17;
        locals.var_isub_rv = 0.0;

        let (assign22000_e30757, assign22000_e30757_d_n0, assign22000_e30757_d_n2, assign22000_e30757_d_n6, assign22000_e30757_d_n7, assign22000_e30757_d_n10, assign22000_e30757_d_n11, assign22000_e30757_d_n12, assign22000_e30757_d_n17,) = {
    if (((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) {
        let assign22000_e30749: f64 = (locals.var_vgsz - locals.var_vfbsub0);
        let assign22000_e30751: f64 = (assign22000_e30749 + locals.var_dvth);
        let assign22000_e30753: f64 = (assign22000_e30751 - locals.var_dppg);
        let assign22000_e30755: f64 = (assign22000_e30753 + p.p48);
        (assign22000_e30755, ((locals.var_vgsz_dn0 + locals.var_dvth_dn0) - locals.var_dppg_dn0), ((locals.var_vgsz_dn2 + locals.var_dvth_dn2) - locals.var_dppg_dn2), ((locals.var_vgsz_dn6 + locals.var_dvth_dn6) - locals.var_dppg_dn6), ((locals.var_vgsz_dn7 + locals.var_dvth_dn7) - locals.var_dppg_dn7), ((locals.var_vgsz_dn10 + locals.var_dvth_dn10) - locals.var_dppg_dn10), ((locals.var_vgsz_dn11 + locals.var_dvth_dn11) - locals.var_dppg_dn11), ((locals.var_vgsz_dn12 + locals.var_dvth_dn12) - locals.var_dppg_dn12), ((locals.var_vgsz_dn17 + locals.var_dvth_dn17) - locals.var_dppg_dn17),)
    } else {
        (locals.var_vgpsub, locals.var_vgpsub_dn0, locals.var_vgpsub_dn2, locals.var_vgpsub_dn6, locals.var_vgpsub_dn7, locals.var_vgpsub_dn10, locals.var_vgpsub_dn11, locals.var_vgpsub_dn12, locals.var_vgpsub_dn17,)
    }
};
        locals.var_vgpsub = assign22000_e30757;
        locals.var_vgpsub_dn0 = assign22000_e30757_d_n0;
        locals.var_vgpsub_dn2 = assign22000_e30757_d_n2;
        locals.var_vgpsub_dn6 = assign22000_e30757_d_n6;
        locals.var_vgpsub_dn7 = assign22000_e30757_d_n7;
        locals.var_vgpsub_dn10 = assign22000_e30757_d_n10;
        locals.var_vgpsub_dn11 = assign22000_e30757_d_n11;
        locals.var_vgpsub_dn12 = assign22000_e30757_d_n12;
        locals.var_vgpsub_dn17 = assign22000_e30757_d_n17;
        locals.var_vgpsub_rv = 0.0;

        let assign22010_e30760: f64 = if p.p44 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard684 = assign22010_e30760;
        locals.var_guard684_rv = 0.0;

        let (assign22020_e30772, assign22020_e30772_d_n0, assign22020_e30772_d_n2, assign22020_e30772_d_n6, assign22020_e30772_d_n7, assign22020_e30772_d_n10, assign22020_e30772_d_n11, assign22020_e30772_d_n12, assign22020_e30772_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 != 0.0)) {
        (locals.var_vgpsub, locals.var_vgpsub_dn0, locals.var_vgpsub_dn2, locals.var_vgpsub_dn6, locals.var_vgpsub_dn7, locals.var_vgpsub_dn10, locals.var_vgpsub_dn11, locals.var_vgpsub_dn12, locals.var_vgpsub_dn17,)
    } else {
        (locals.var_t1__blk670, locals.var_t1__blk670_dn0, locals.var_t1__blk670_dn2, locals.var_t1__blk670_dn6, locals.var_t1__blk670_dn7, locals.var_t1__blk670_dn10, locals.var_t1__blk670_dn11, locals.var_t1__blk670_dn12, locals.var_t1__blk670_dn17,)
    }
};
        locals.var_t1__blk670 = assign22020_e30772;
        locals.var_t1__blk670_dn0 = assign22020_e30772_d_n0;
        locals.var_t1__blk670_dn2 = assign22020_e30772_d_n2;
        locals.var_t1__blk670_dn6 = assign22020_e30772_d_n6;
        locals.var_t1__blk670_dn7 = assign22020_e30772_d_n7;
        locals.var_t1__blk670_dn10 = assign22020_e30772_d_n10;
        locals.var_t1__blk670_dn11 = assign22020_e30772_d_n11;
        locals.var_t1__blk670_dn12 = assign22020_e30772_d_n12;
        locals.var_t1__blk670_dn17 = assign22020_e30772_d_n17;
        locals.var_t1__blk670_rv = 0.0;

        let (assign22030_e30786, assign22030_e30786_d_n0, assign22030_e30786_d_n2, assign22030_e30786_d_n6, assign22030_e30786_d_n7, assign22030_e30786_d_n10, assign22030_e30786_d_n11, assign22030_e30786_d_n12, assign22030_e30786_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign22030_e30784: f64 = (locals.var_c_fox * locals.var_c_fox);
        (assign22030_e30784, ((locals.var_c_fox_dn0 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn0)), ((locals.var_c_fox_dn2 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn2)), ((locals.var_c_fox_dn6 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn6)), ((locals.var_c_fox_dn7 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn7)), ((locals.var_c_fox_dn10 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn10)), ((locals.var_c_fox_dn11 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn11)), ((locals.var_c_fox_dn12 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn12)), ((locals.var_c_fox_dn17 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn17)),)
    } else {
        (locals.var_t7__blk677, locals.var_t7__blk677_dn0, locals.var_t7__blk677_dn2, locals.var_t7__blk677_dn6, locals.var_t7__blk677_dn7, locals.var_t7__blk677_dn10, locals.var_t7__blk677_dn11, locals.var_t7__blk677_dn12, locals.var_t7__blk677_dn17,)
    }
};
        locals.var_t7__blk677 = assign22030_e30786;
        locals.var_t7__blk677_dn0 = assign22030_e30786_d_n0;
        locals.var_t7__blk677_dn2 = assign22030_e30786_d_n2;
        locals.var_t7__blk677_dn6 = assign22030_e30786_d_n6;
        locals.var_t7__blk677_dn7 = assign22030_e30786_d_n7;
        locals.var_t7__blk677_dn10 = assign22030_e30786_d_n10;
        locals.var_t7__blk677_dn11 = assign22030_e30786_d_n11;
        locals.var_t7__blk677_dn12 = assign22030_e30786_d_n12;
        locals.var_t7__blk677_dn17 = assign22030_e30786_d_n17;
        locals.var_t7__blk677_rv = 0.0;

        let (assign22040_e30798, assign22040_e30798_d_n0, assign22040_e30798_d_n2, assign22040_e30798_d_n6, assign22040_e30798_d_n7, assign22040_e30798_d_n10, assign22040_e30798_d_n11, assign22040_e30798_d_n12, assign22040_e30798_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 != 0.0)) {
        (locals.var_qnsub_esi, locals.var_qnsub_esi_dn0, locals.var_qnsub_esi_dn2, locals.var_qnsub_esi_dn6, locals.var_qnsub_esi_dn7, locals.var_qnsub_esi_dn10, locals.var_qnsub_esi_dn11, locals.var_qnsub_esi_dn12, locals.var_qnsub_esi_dn17,)
    } else {
        (locals.var_t8__blk678, locals.var_t8__blk678_dn0, locals.var_t8__blk678_dn2, locals.var_t8__blk678_dn6, locals.var_t8__blk678_dn7, locals.var_t8__blk678_dn10, locals.var_t8__blk678_dn11, locals.var_t8__blk678_dn12, locals.var_t8__blk678_dn17,)
    }
};
        locals.var_t8__blk678 = assign22040_e30798;
        locals.var_t8__blk678_dn0 = assign22040_e30798_d_n0;
        locals.var_t8__blk678_dn2 = assign22040_e30798_d_n2;
        locals.var_t8__blk678_dn6 = assign22040_e30798_d_n6;
        locals.var_t8__blk678_dn7 = assign22040_e30798_d_n7;
        locals.var_t8__blk678_dn10 = assign22040_e30798_d_n10;
        locals.var_t8__blk678_dn11 = assign22040_e30798_d_n11;
        locals.var_t8__blk678_dn12 = assign22040_e30798_d_n12;
        locals.var_t8__blk678_dn17 = assign22040_e30798_d_n17;
        locals.var_t8__blk678_rv = 0.0;

        let (assign22050_e30812, assign22050_e30812_d_n0, assign22050_e30812_d_n2, assign22050_e30812_d_n6, assign22050_e30812_d_n7, assign22050_e30812_d_n10, assign22050_e30812_d_n11, assign22050_e30812_d_n12, assign22050_e30812_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign22050_e30810: f64 = (locals.var_t8__blk678 / locals.var_t7__blk677);
        (assign22050_e30810, (((locals.var_t8__blk678_dn0 * locals.var_t7__blk677) - (locals.var_t8__blk678 * locals.var_t7__blk677_dn0)) / (locals.var_t7__blk677 * locals.var_t7__blk677)), (((locals.var_t8__blk678_dn2 * locals.var_t7__blk677) - (locals.var_t8__blk678 * locals.var_t7__blk677_dn2)) / (locals.var_t7__blk677 * locals.var_t7__blk677)), (((locals.var_t8__blk678_dn6 * locals.var_t7__blk677) - (locals.var_t8__blk678 * locals.var_t7__blk677_dn6)) / (locals.var_t7__blk677 * locals.var_t7__blk677)), (((locals.var_t8__blk678_dn7 * locals.var_t7__blk677) - (locals.var_t8__blk678 * locals.var_t7__blk677_dn7)) / (locals.var_t7__blk677 * locals.var_t7__blk677)), (((locals.var_t8__blk678_dn10 * locals.var_t7__blk677) - (locals.var_t8__blk678 * locals.var_t7__blk677_dn10)) / (locals.var_t7__blk677 * locals.var_t7__blk677)), (((locals.var_t8__blk678_dn11 * locals.var_t7__blk677) - (locals.var_t8__blk678 * locals.var_t7__blk677_dn11)) / (locals.var_t7__blk677 * locals.var_t7__blk677)), (((locals.var_t8__blk678_dn12 * locals.var_t7__blk677) - (locals.var_t8__blk678 * locals.var_t7__blk677_dn12)) / (locals.var_t7__blk677 * locals.var_t7__blk677)), (((locals.var_t8__blk678_dn17 * locals.var_t7__blk677) - (locals.var_t8__blk678 * locals.var_t7__blk677_dn17)) / (locals.var_t7__blk677 * locals.var_t7__blk677)),)
    } else {
        (locals.var_t3__blk672, locals.var_t3__blk672_dn0, locals.var_t3__blk672_dn2, locals.var_t3__blk672_dn6, locals.var_t3__blk672_dn7, locals.var_t3__blk672_dn10, locals.var_t3__blk672_dn11, locals.var_t3__blk672_dn12, locals.var_t3__blk672_dn17,)
    }
};
        locals.var_t3__blk672 = assign22050_e30812;
        locals.var_t3__blk672_dn0 = assign22050_e30812_d_n0;
        locals.var_t3__blk672_dn2 = assign22050_e30812_d_n2;
        locals.var_t3__blk672_dn6 = assign22050_e30812_d_n6;
        locals.var_t3__blk672_dn7 = assign22050_e30812_d_n7;
        locals.var_t3__blk672_dn10 = assign22050_e30812_d_n10;
        locals.var_t3__blk672_dn11 = assign22050_e30812_d_n11;
        locals.var_t3__blk672_dn12 = assign22050_e30812_d_n12;
        locals.var_t3__blk672_dn17 = assign22050_e30812_d_n17;
        locals.var_t3__blk672_rv = 0.0;

        let (assign22060_e30826, assign22060_e30826_d_n0, assign22060_e30826_d_n2, assign22060_e30826_d_n6, assign22060_e30826_d_n7, assign22060_e30826_d_n10, assign22060_e30826_d_n11, assign22060_e30826_d_n12, assign22060_e30826_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign22060_e30824: f64 = (2.0 / locals.var_t8__blk678);
        (assign22060_e30824, (-((2.0 * locals.var_t8__blk678_dn0) / (locals.var_t8__blk678 * locals.var_t8__blk678))), (-((2.0 * locals.var_t8__blk678_dn2) / (locals.var_t8__blk678 * locals.var_t8__blk678))), (-((2.0 * locals.var_t8__blk678_dn6) / (locals.var_t8__blk678 * locals.var_t8__blk678))), (-((2.0 * locals.var_t8__blk678_dn7) / (locals.var_t8__blk678 * locals.var_t8__blk678))), (-((2.0 * locals.var_t8__blk678_dn10) / (locals.var_t8__blk678 * locals.var_t8__blk678))), (-((2.0 * locals.var_t8__blk678_dn11) / (locals.var_t8__blk678 * locals.var_t8__blk678))), (-((2.0 * locals.var_t8__blk678_dn12) / (locals.var_t8__blk678 * locals.var_t8__blk678))), (-((2.0 * locals.var_t8__blk678_dn17) / (locals.var_t8__blk678 * locals.var_t8__blk678))),)
    } else {
        (locals.var_t9__blk679, locals.var_t9__blk679_dn0, locals.var_t9__blk679_dn2, locals.var_t9__blk679_dn6, locals.var_t9__blk679_dn7, locals.var_t9__blk679_dn10, locals.var_t9__blk679_dn11, locals.var_t9__blk679_dn12, locals.var_t9__blk679_dn17,)
    }
};
        locals.var_t9__blk679 = assign22060_e30826;
        locals.var_t9__blk679_dn0 = assign22060_e30826_d_n0;
        locals.var_t9__blk679_dn2 = assign22060_e30826_d_n2;
        locals.var_t9__blk679_dn6 = assign22060_e30826_d_n6;
        locals.var_t9__blk679_dn7 = assign22060_e30826_d_n7;
        locals.var_t9__blk679_dn10 = assign22060_e30826_d_n10;
        locals.var_t9__blk679_dn11 = assign22060_e30826_d_n11;
        locals.var_t9__blk679_dn12 = assign22060_e30826_d_n12;
        locals.var_t9__blk679_dn17 = assign22060_e30826_d_n17;
        locals.var_t9__blk679_rv = 0.0;

        let (assign22070_e30840, assign22070_e30840_d_n0, assign22070_e30840_d_n2, assign22070_e30840_d_n6, assign22070_e30840_d_n7, assign22070_e30840_d_n10, assign22070_e30840_d_n11, assign22070_e30840_d_n12, assign22070_e30840_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign22070_e30838: f64 = (locals.var_t9__blk679 * locals.var_t7__blk677);
        (assign22070_e30838, ((locals.var_t9__blk679_dn0 * locals.var_t7__blk677) + (locals.var_t9__blk679 * locals.var_t7__blk677_dn0)), ((locals.var_t9__blk679_dn2 * locals.var_t7__blk677) + (locals.var_t9__blk679 * locals.var_t7__blk677_dn2)), ((locals.var_t9__blk679_dn6 * locals.var_t7__blk677) + (locals.var_t9__blk679 * locals.var_t7__blk677_dn6)), ((locals.var_t9__blk679_dn7 * locals.var_t7__blk677) + (locals.var_t9__blk679 * locals.var_t7__blk677_dn7)), ((locals.var_t9__blk679_dn10 * locals.var_t7__blk677) + (locals.var_t9__blk679 * locals.var_t7__blk677_dn10)), ((locals.var_t9__blk679_dn11 * locals.var_t7__blk677) + (locals.var_t9__blk679 * locals.var_t7__blk677_dn11)), ((locals.var_t9__blk679_dn12 * locals.var_t7__blk677) + (locals.var_t9__blk679 * locals.var_t7__blk677_dn12)), ((locals.var_t9__blk679_dn17 * locals.var_t7__blk677) + (locals.var_t9__blk679 * locals.var_t7__blk677_dn17)),)
    } else {
        (locals.var_t4__blk673, locals.var_t4__blk673_dn0, locals.var_t4__blk673_dn2, locals.var_t4__blk673_dn6, locals.var_t4__blk673_dn7, locals.var_t4__blk673_dn10, locals.var_t4__blk673_dn11, locals.var_t4__blk673_dn12, locals.var_t4__blk673_dn17,)
    }
};
        locals.var_t4__blk673 = assign22070_e30840;
        locals.var_t4__blk673_dn0 = assign22070_e30840_d_n0;
        locals.var_t4__blk673_dn2 = assign22070_e30840_d_n2;
        locals.var_t4__blk673_dn6 = assign22070_e30840_d_n6;
        locals.var_t4__blk673_dn7 = assign22070_e30840_d_n7;
        locals.var_t4__blk673_dn10 = assign22070_e30840_d_n10;
        locals.var_t4__blk673_dn11 = assign22070_e30840_d_n11;
        locals.var_t4__blk673_dn12 = assign22070_e30840_d_n12;
        locals.var_t4__blk673_dn17 = assign22070_e30840_d_n17;
        locals.var_t4__blk673_rv = 0.0;

        let (assign22080_e30858, assign22080_e30858_d_n0, assign22080_e30858_d_n2, assign22080_e30858_d_n6, assign22080_e30858_d_n7, assign22080_e30858_d_n10, assign22080_e30858_d_n11, assign22080_e30858_d_n12, assign22080_e30858_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign22080_e30852: f64 = (locals.var_t1__blk670 - locals.var_beta_inv);
        let assign22080_e30855: f64 = (locals.var_xvbs * locals.var_vbspz);
        let assign22080_e30856: f64 = (assign22080_e30852 - assign22080_e30855);
        (assign22080_e30856, (locals.var_t1__blk670_dn0 - (locals.var_xvbs * locals.var_vbspz_dn0)), (locals.var_t1__blk670_dn2 - (locals.var_xvbs * locals.var_vbspz_dn2)), (locals.var_t1__blk670_dn6 - (locals.var_xvbs * locals.var_vbspz_dn6)), (locals.var_t1__blk670_dn7 - (locals.var_xvbs * locals.var_vbspz_dn7)), ((locals.var_t1__blk670_dn10 - locals.var_beta_inv_dn10) - (locals.var_xvbs * locals.var_vbspz_dn10)), (locals.var_t1__blk670_dn11 - (locals.var_xvbs * locals.var_vbspz_dn11)), (locals.var_t1__blk670_dn12 - (locals.var_xvbs * locals.var_vbspz_dn12)), (locals.var_t1__blk670_dn17 - (locals.var_xvbs * locals.var_vbspz_dn17)),)
    } else {
        (locals.var_t5__blk674, locals.var_t5__blk674_dn0, locals.var_t5__blk674_dn2, locals.var_t5__blk674_dn6, locals.var_t5__blk674_dn7, locals.var_t5__blk674_dn10, locals.var_t5__blk674_dn11, locals.var_t5__blk674_dn12, locals.var_t5__blk674_dn17,)
    }
};
        locals.var_t5__blk674 = assign22080_e30858;
        locals.var_t5__blk674_dn0 = assign22080_e30858_d_n0;
        locals.var_t5__blk674_dn2 = assign22080_e30858_d_n2;
        locals.var_t5__blk674_dn6 = assign22080_e30858_d_n6;
        locals.var_t5__blk674_dn7 = assign22080_e30858_d_n7;
        locals.var_t5__blk674_dn10 = assign22080_e30858_d_n10;
        locals.var_t5__blk674_dn11 = assign22080_e30858_d_n11;
        locals.var_t5__blk674_dn12 = assign22080_e30858_d_n12;
        locals.var_t5__blk674_dn17 = assign22080_e30858_d_n17;
        locals.var_t5__blk674_rv = 0.0;

        let (assign22090_e30874, assign22090_e30874_d_n0, assign22090_e30874_d_n2, assign22090_e30874_d_n6, assign22090_e30874_d_n7, assign22090_e30874_d_n10, assign22090_e30874_d_n11, assign22090_e30874_d_n12, assign22090_e30874_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign22090_e30870: f64 = (p.p49 * locals.var_qhs);
        let assign22090_e30872: f64 = (assign22090_e30870 / locals.var_c_soi);
        (assign22090_e30872, ((p.p49 * locals.var_qhs_dn0) / locals.var_c_soi), ((p.p49 * locals.var_qhs_dn2) / locals.var_c_soi), ((p.p49 * locals.var_qhs_dn6) / locals.var_c_soi), ((p.p49 * locals.var_qhs_dn7) / locals.var_c_soi), ((p.p49 * locals.var_qhs_dn10) / locals.var_c_soi), ((p.p49 * locals.var_qhs_dn11) / locals.var_c_soi), ((p.p49 * locals.var_qhs_dn12) / locals.var_c_soi), ((p.p49 * locals.var_qhs_dn17) / locals.var_c_soi),)
    } else {
        (locals.var_dvbssub, locals.var_dvbssub_dn0, locals.var_dvbssub_dn2, locals.var_dvbssub_dn6, locals.var_dvbssub_dn7, locals.var_dvbssub_dn10, locals.var_dvbssub_dn11, locals.var_dvbssub_dn12, locals.var_dvbssub_dn17,)
    }
};
        locals.var_dvbssub = assign22090_e30874;
        locals.var_dvbssub_dn0 = assign22090_e30874_d_n0;
        locals.var_dvbssub_dn2 = assign22090_e30874_d_n2;
        locals.var_dvbssub_dn6 = assign22090_e30874_d_n6;
        locals.var_dvbssub_dn7 = assign22090_e30874_d_n7;
        locals.var_dvbssub_dn10 = assign22090_e30874_d_n10;
        locals.var_dvbssub_dn11 = assign22090_e30874_d_n11;
        locals.var_dvbssub_dn12 = assign22090_e30874_d_n12;
        locals.var_dvbssub_dn17 = assign22090_e30874_d_n17;
        locals.var_dvbssub_rv = 0.0;

        let (assign22100_e30890, assign22100_e30890_d_n0, assign22100_e30890_d_n2, assign22100_e30890_d_n6, assign22100_e30890_d_n7, assign22100_e30890_d_n10, assign22100_e30890_d_n11, assign22100_e30890_d_n12, assign22100_e30890_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign22100_e30887: f64 = (locals.var_xvbs * locals.var_dvbssub);
        let assign22100_e30888: f64 = (locals.var_t5__blk674 - assign22100_e30887);
        (assign22100_e30888, (locals.var_t5__blk674_dn0 - (locals.var_xvbs * locals.var_dvbssub_dn0)), (locals.var_t5__blk674_dn2 - (locals.var_xvbs * locals.var_dvbssub_dn2)), (locals.var_t5__blk674_dn6 - (locals.var_xvbs * locals.var_dvbssub_dn6)), (locals.var_t5__blk674_dn7 - (locals.var_xvbs * locals.var_dvbssub_dn7)), (locals.var_t5__blk674_dn10 - (locals.var_xvbs * locals.var_dvbssub_dn10)), (locals.var_t5__blk674_dn11 - (locals.var_xvbs * locals.var_dvbssub_dn11)), (locals.var_t5__blk674_dn12 - (locals.var_xvbs * locals.var_dvbssub_dn12)), (locals.var_t5__blk674_dn17 - (locals.var_xvbs * locals.var_dvbssub_dn17)),)
    } else {
        (locals.var_t5__blk674, locals.var_t5__blk674_dn0, locals.var_t5__blk674_dn2, locals.var_t5__blk674_dn6, locals.var_t5__blk674_dn7, locals.var_t5__blk674_dn10, locals.var_t5__blk674_dn11, locals.var_t5__blk674_dn12, locals.var_t5__blk674_dn17,)
    }
};
        locals.var_t5__blk674 = assign22100_e30890;
        locals.var_t5__blk674_dn0 = assign22100_e30890_d_n0;
        locals.var_t5__blk674_dn2 = assign22100_e30890_d_n2;
        locals.var_t5__blk674_dn6 = assign22100_e30890_d_n6;
        locals.var_t5__blk674_dn7 = assign22100_e30890_d_n7;
        locals.var_t5__blk674_dn10 = assign22100_e30890_d_n10;
        locals.var_t5__blk674_dn11 = assign22100_e30890_d_n11;
        locals.var_t5__blk674_dn12 = assign22100_e30890_d_n12;
        locals.var_t5__blk674_dn17 = assign22100_e30890_d_n17;
        locals.var_t5__blk674_rv = 0.0;

        let (assign22110_e30906, assign22110_e30906_d_n0, assign22110_e30906_d_n2, assign22110_e30906_d_n6, assign22110_e30906_d_n7, assign22110_e30906_d_n10, assign22110_e30906_d_n11, assign22110_e30906_d_n12, assign22110_e30906_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign22110_e30903: f64 = (locals.var_t4__blk673 * locals.var_t5__blk674);
        let assign22110_e30904: f64 = (1.0 + assign22110_e30903);
        (assign22110_e30904, ((locals.var_t4__blk673_dn0 * locals.var_t5__blk674) + (locals.var_t4__blk673 * locals.var_t5__blk674_dn0)), ((locals.var_t4__blk673_dn2 * locals.var_t5__blk674) + (locals.var_t4__blk673 * locals.var_t5__blk674_dn2)), ((locals.var_t4__blk673_dn6 * locals.var_t5__blk674) + (locals.var_t4__blk673 * locals.var_t5__blk674_dn6)), ((locals.var_t4__blk673_dn7 * locals.var_t5__blk674) + (locals.var_t4__blk673 * locals.var_t5__blk674_dn7)), ((locals.var_t4__blk673_dn10 * locals.var_t5__blk674) + (locals.var_t4__blk673 * locals.var_t5__blk674_dn10)), ((locals.var_t4__blk673_dn11 * locals.var_t5__blk674) + (locals.var_t4__blk673 * locals.var_t5__blk674_dn11)), ((locals.var_t4__blk673_dn12 * locals.var_t5__blk674) + (locals.var_t4__blk673 * locals.var_t5__blk674_dn12)), ((locals.var_t4__blk673_dn17 * locals.var_t5__blk674) + (locals.var_t4__blk673 * locals.var_t5__blk674_dn17)),)
    } else {
        (locals.var_t6w__blk676, locals.var_t6w__blk676_dn0, locals.var_t6w__blk676_dn2, locals.var_t6w__blk676_dn6, locals.var_t6w__blk676_dn7, locals.var_t6w__blk676_dn10, locals.var_t6w__blk676_dn11, locals.var_t6w__blk676_dn12, locals.var_t6w__blk676_dn17,)
    }
};
        locals.var_t6w__blk676 = assign22110_e30906;
        locals.var_t6w__blk676_dn0 = assign22110_e30906_d_n0;
        locals.var_t6w__blk676_dn2 = assign22110_e30906_d_n2;
        locals.var_t6w__blk676_dn6 = assign22110_e30906_d_n6;
        locals.var_t6w__blk676_dn7 = assign22110_e30906_d_n7;
        locals.var_t6w__blk676_dn10 = assign22110_e30906_d_n10;
        locals.var_t6w__blk676_dn11 = assign22110_e30906_d_n11;
        locals.var_t6w__blk676_dn12 = assign22110_e30906_d_n12;
        locals.var_t6w__blk676_dn17 = assign22110_e30906_d_n17;
        locals.var_t6w__blk676_rv = 0.0;

        let (assign22120_e30927, assign22120_e30927_d_n0, assign22120_e30927_d_n2, assign22120_e30927_d_n6, assign22120_e30927_d_n7, assign22120_e30927_d_n10, assign22120_e30927_d_n11, assign22120_e30927_d_n12, assign22120_e30927_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign22120_e30918: f64 = (locals.var_t6w__blk676 * locals.var_t6w__blk676);
        let assign22120_e30921: f64 = (4.0 * 0.001);
        let assign22120_e30923: f64 = (assign22120_e30921 * 0.001);
        let assign22120_e30924: f64 = (assign22120_e30918 + assign22120_e30923);
        let assign22120_e30925: f64 = (assign22120_e30924).sqrt();
        (assign22120_e30925, (((locals.var_t6w__blk676_dn0 * locals.var_t6w__blk676) + (locals.var_t6w__blk676 * locals.var_t6w__blk676_dn0)) / (2.0 * assign22120_e30925)), (((locals.var_t6w__blk676_dn2 * locals.var_t6w__blk676) + (locals.var_t6w__blk676 * locals.var_t6w__blk676_dn2)) / (2.0 * assign22120_e30925)), (((locals.var_t6w__blk676_dn6 * locals.var_t6w__blk676) + (locals.var_t6w__blk676 * locals.var_t6w__blk676_dn6)) / (2.0 * assign22120_e30925)), (((locals.var_t6w__blk676_dn7 * locals.var_t6w__blk676) + (locals.var_t6w__blk676 * locals.var_t6w__blk676_dn7)) / (2.0 * assign22120_e30925)), (((locals.var_t6w__blk676_dn10 * locals.var_t6w__blk676) + (locals.var_t6w__blk676 * locals.var_t6w__blk676_dn10)) / (2.0 * assign22120_e30925)), (((locals.var_t6w__blk676_dn11 * locals.var_t6w__blk676) + (locals.var_t6w__blk676 * locals.var_t6w__blk676_dn11)) / (2.0 * assign22120_e30925)), (((locals.var_t6w__blk676_dn12 * locals.var_t6w__blk676) + (locals.var_t6w__blk676 * locals.var_t6w__blk676_dn12)) / (2.0 * assign22120_e30925)), (((locals.var_t6w__blk676_dn17 * locals.var_t6w__blk676) + (locals.var_t6w__blk676 * locals.var_t6w__blk676_dn17)) / (2.0 * assign22120_e30925)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign22120_e30927;
        locals.var_tmf1_dn0 = assign22120_e30927_d_n0;
        locals.var_tmf1_dn2 = assign22120_e30927_d_n2;
        locals.var_tmf1_dn6 = assign22120_e30927_d_n6;
        locals.var_tmf1_dn7 = assign22120_e30927_d_n7;
        locals.var_tmf1_dn10 = assign22120_e30927_d_n10;
        locals.var_tmf1_dn11 = assign22120_e30927_d_n11;
        locals.var_tmf1_dn12 = assign22120_e30927_d_n12;
        locals.var_tmf1_dn17 = assign22120_e30927_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign22130_e30947, assign22130_e30947_d_n0, assign22130_e30947_d_n2, assign22130_e30947_d_n6, assign22130_e30947_d_n7, assign22130_e30947_d_n10, assign22130_e30947_d_n11, assign22130_e30947_d_n12, assign22130_e30947_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign22130_e30940: f64 = (locals.var_t6w__blk676 + locals.var_tmf1);
        let assign22130_e30941: f64 = (0.5 * assign22130_e30940);
        let assign22130_e30944: f64 = (1e-10 * 0.001);
        let assign22130_e30945: f64 = (assign22130_e30941 + assign22130_e30944);
        (assign22130_e30945, (0.5 * (locals.var_t6w__blk676_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_t6w__blk676_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_t6w__blk676_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_t6w__blk676_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_t6w__blk676_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_t6w__blk676_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_t6w__blk676_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_t6w__blk676_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_t6__blk675, locals.var_t6__blk675_dn0, locals.var_t6__blk675_dn2, locals.var_t6__blk675_dn6, locals.var_t6__blk675_dn7, locals.var_t6__blk675_dn10, locals.var_t6__blk675_dn11, locals.var_t6__blk675_dn12, locals.var_t6__blk675_dn17,)
    }
};
        locals.var_t6__blk675 = assign22130_e30947;
        locals.var_t6__blk675_dn0 = assign22130_e30947_d_n0;
        locals.var_t6__blk675_dn2 = assign22130_e30947_d_n2;
        locals.var_t6__blk675_dn6 = assign22130_e30947_d_n6;
        locals.var_t6__blk675_dn7 = assign22130_e30947_d_n7;
        locals.var_t6__blk675_dn10 = assign22130_e30947_d_n10;
        locals.var_t6__blk675_dn11 = assign22130_e30947_d_n11;
        locals.var_t6__blk675_dn12 = assign22130_e30947_d_n12;
        locals.var_t6__blk675_dn17 = assign22130_e30947_d_n17;
        locals.var_t6__blk675_rv = 0.0;

        let assign22140_e30950: f64 = if locals.var_t6__blk675 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard685 = assign22140_e30950;
        locals.var_guard685_rv = 0.0;

        let (assign22150_e30964, assign22150_e30964_d_n0, assign22150_e30964_d_n2, assign22150_e30964_d_n6, assign22150_e30964_d_n7, assign22150_e30964_d_n10, assign22150_e30964_d_n11, assign22150_e30964_d_n12, assign22150_e30964_d_n17,) = {
    if (((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 != 0.0)) && (locals.var_guard685 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6__blk675, locals.var_t6__blk675_dn0, locals.var_t6__blk675_dn2, locals.var_t6__blk675_dn6, locals.var_t6__blk675_dn7, locals.var_t6__blk675_dn10, locals.var_t6__blk675_dn11, locals.var_t6__blk675_dn12, locals.var_t6__blk675_dn17,)
    }
};
        locals.var_t6__blk675 = assign22150_e30964;
        locals.var_t6__blk675_dn0 = assign22150_e30964_d_n0;
        locals.var_t6__blk675_dn2 = assign22150_e30964_d_n2;
        locals.var_t6__blk675_dn6 = assign22150_e30964_d_n6;
        locals.var_t6__blk675_dn7 = assign22150_e30964_d_n7;
        locals.var_t6__blk675_dn10 = assign22150_e30964_d_n10;
        locals.var_t6__blk675_dn11 = assign22150_e30964_d_n11;
        locals.var_t6__blk675_dn12 = assign22150_e30964_d_n12;
        locals.var_t6__blk675_dn17 = assign22150_e30964_d_n17;
        locals.var_t6__blk675_rv = 0.0;

        let (assign22160_e30978, assign22160_e30978_d_n0, assign22160_e30978_d_n2, assign22160_e30978_d_n6, assign22160_e30978_d_n7, assign22160_e30978_d_n10, assign22160_e30978_d_n11, assign22160_e30978_d_n12, assign22160_e30978_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign22160_e30976: f64 = (locals.var_t6__blk675 + 1e-50);
        (assign22160_e30976, locals.var_t6__blk675_dn0, locals.var_t6__blk675_dn2, locals.var_t6__blk675_dn6, locals.var_t6__blk675_dn7, locals.var_t6__blk675_dn10, locals.var_t6__blk675_dn11, locals.var_t6__blk675_dn12, locals.var_t6__blk675_dn17,)
    } else {
        (locals.var_t6__blk675, locals.var_t6__blk675_dn0, locals.var_t6__blk675_dn2, locals.var_t6__blk675_dn6, locals.var_t6__blk675_dn7, locals.var_t6__blk675_dn10, locals.var_t6__blk675_dn11, locals.var_t6__blk675_dn12, locals.var_t6__blk675_dn17,)
    }
};
        locals.var_t6__blk675 = assign22160_e30978;
        locals.var_t6__blk675_dn0 = assign22160_e30978_d_n0;
        locals.var_t6__blk675_dn2 = assign22160_e30978_d_n2;
        locals.var_t6__blk675_dn6 = assign22160_e30978_d_n6;
        locals.var_t6__blk675_dn7 = assign22160_e30978_d_n7;
        locals.var_t6__blk675_dn10 = assign22160_e30978_d_n10;
        locals.var_t6__blk675_dn11 = assign22160_e30978_d_n11;
        locals.var_t6__blk675_dn12 = assign22160_e30978_d_n12;
        locals.var_t6__blk675_dn17 = assign22160_e30978_d_n17;
        locals.var_t6__blk675_rv = 0.0;

        let (assign22170_e30991, assign22170_e30991_d_n0, assign22170_e30991_d_n2, assign22170_e30991_d_n6, assign22170_e30991_d_n7, assign22170_e30991_d_n10, assign22170_e30991_d_n11, assign22170_e30991_d_n12, assign22170_e30991_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign22170_e30989: f64 = (locals.var_t6__blk675).sqrt();
        (assign22170_e30989, (locals.var_t6__blk675_dn0 / (2.0 * assign22170_e30989)), (locals.var_t6__blk675_dn2 / (2.0 * assign22170_e30989)), (locals.var_t6__blk675_dn6 / (2.0 * assign22170_e30989)), (locals.var_t6__blk675_dn7 / (2.0 * assign22170_e30989)), (locals.var_t6__blk675_dn10 / (2.0 * assign22170_e30989)), (locals.var_t6__blk675_dn11 / (2.0 * assign22170_e30989)), (locals.var_t6__blk675_dn12 / (2.0 * assign22170_e30989)), (locals.var_t6__blk675_dn17 / (2.0 * assign22170_e30989)),)
    } else {
        (locals.var_t6__blk675, locals.var_t6__blk675_dn0, locals.var_t6__blk675_dn2, locals.var_t6__blk675_dn6, locals.var_t6__blk675_dn7, locals.var_t6__blk675_dn10, locals.var_t6__blk675_dn11, locals.var_t6__blk675_dn12, locals.var_t6__blk675_dn17,)
    }
};
        locals.var_t6__blk675 = assign22170_e30991;
        locals.var_t6__blk675_dn0 = assign22170_e30991_d_n0;
        locals.var_t6__blk675_dn2 = assign22170_e30991_d_n2;
        locals.var_t6__blk675_dn6 = assign22170_e30991_d_n6;
        locals.var_t6__blk675_dn7 = assign22170_e30991_d_n7;
        locals.var_t6__blk675_dn10 = assign22170_e30991_d_n10;
        locals.var_t6__blk675_dn11 = assign22170_e30991_d_n11;
        locals.var_t6__blk675_dn12 = assign22170_e30991_d_n12;
        locals.var_t6__blk675_dn17 = assign22170_e30991_d_n17;
        locals.var_t6__blk675_rv = 0.0;

        let (assign22180_e31011, assign22180_e31011_d_n0, assign22180_e31011_d_n2, assign22180_e31011_d_n6, assign22180_e31011_d_n7, assign22180_e31011_d_n10, assign22180_e31011_d_n11, assign22180_e31011_d_n12, assign22180_e31011_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign22180_e31003: f64 = (locals.var_t1__blk670 * locals.var_uc_svgs);
        let assign22180_e31007: f64 = (1.0 - locals.var_t6__blk675);
        let assign22180_e31008: f64 = (locals.var_t3__blk672 * assign22180_e31007);
        let assign22180_e31009: f64 = (assign22180_e31003 + assign22180_e31008);
        (assign22180_e31009, ((locals.var_t1__blk670_dn0 * locals.var_uc_svgs) + ((locals.var_t3__blk672_dn0 * assign22180_e31007) + (locals.var_t3__blk672 * (-locals.var_t6__blk675_dn0)))), ((locals.var_t1__blk670_dn2 * locals.var_uc_svgs) + ((locals.var_t3__blk672_dn2 * assign22180_e31007) + (locals.var_t3__blk672 * (-locals.var_t6__blk675_dn2)))), ((locals.var_t1__blk670_dn6 * locals.var_uc_svgs) + ((locals.var_t3__blk672_dn6 * assign22180_e31007) + (locals.var_t3__blk672 * (-locals.var_t6__blk675_dn6)))), ((locals.var_t1__blk670_dn7 * locals.var_uc_svgs) + ((locals.var_t3__blk672_dn7 * assign22180_e31007) + (locals.var_t3__blk672 * (-locals.var_t6__blk675_dn7)))), ((locals.var_t1__blk670_dn10 * locals.var_uc_svgs) + ((locals.var_t3__blk672_dn10 * assign22180_e31007) + (locals.var_t3__blk672 * (-locals.var_t6__blk675_dn10)))), ((locals.var_t1__blk670_dn11 * locals.var_uc_svgs) + ((locals.var_t3__blk672_dn11 * assign22180_e31007) + (locals.var_t3__blk672 * (-locals.var_t6__blk675_dn11)))), ((locals.var_t1__blk670_dn12 * locals.var_uc_svgs) + ((locals.var_t3__blk672_dn12 * assign22180_e31007) + (locals.var_t3__blk672 * (-locals.var_t6__blk675_dn12)))), ((locals.var_t1__blk670_dn17 * locals.var_uc_svgs) + ((locals.var_t3__blk672_dn17 * assign22180_e31007) + (locals.var_t3__blk672 * (-locals.var_t6__blk675_dn17)))),)
    } else {
        (locals.var_psislsat__blk680, locals.var_psislsat__blk680_dn0, locals.var_psislsat__blk680_dn2, locals.var_psislsat__blk680_dn6, locals.var_psislsat__blk680_dn7, locals.var_psislsat__blk680_dn10, locals.var_psislsat__blk680_dn11, locals.var_psislsat__blk680_dn12, locals.var_psislsat__blk680_dn17,)
    }
};
        locals.var_psislsat__blk680 = assign22180_e31011;
        locals.var_psislsat__blk680_dn0 = assign22180_e31011_d_n0;
        locals.var_psislsat__blk680_dn2 = assign22180_e31011_d_n2;
        locals.var_psislsat__blk680_dn6 = assign22180_e31011_d_n6;
        locals.var_psislsat__blk680_dn7 = assign22180_e31011_d_n7;
        locals.var_psislsat__blk680_dn10 = assign22180_e31011_d_n10;
        locals.var_psislsat__blk680_dn11 = assign22180_e31011_d_n11;
        locals.var_psislsat__blk680_dn12 = assign22180_e31011_d_n12;
        locals.var_psislsat__blk680_dn17 = assign22180_e31011_d_n17;
        locals.var_psislsat__blk680_rv = 0.0;

        let (assign22190_e31033, assign22190_e31033_d_n0, assign22190_e31033_d_n2, assign22190_e31033_d_n6, assign22190_e31033_d_n7, assign22190_e31033_d_n10, assign22190_e31033_d_n11, assign22190_e31033_d_n12, assign22190_e31033_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign22190_e31023: f64 = (p.p122 * locals.var_vdsz);
        let assign22190_e31025: f64 = (assign22190_e31023 + locals.var_ps0z);
        let assign22190_e31028: f64 = (locals.var_xgate * locals.var_zvgs);
        let assign22190_e31030: f64 = (assign22190_e31028 * locals.var_psislsat__blk680);
        let assign22190_e31031: f64 = (assign22190_e31025 - assign22190_e31030);
        (assign22190_e31031, (((p.p122 * locals.var_vdsz_dn0) + locals.var_ps0z_dn0) - (assign22190_e31028 * locals.var_psislsat__blk680_dn0)), (((p.p122 * locals.var_vdsz_dn2) + locals.var_ps0z_dn2) - (assign22190_e31028 * locals.var_psislsat__blk680_dn2)), (((p.p122 * locals.var_vdsz_dn6) + locals.var_ps0z_dn6) - (assign22190_e31028 * locals.var_psislsat__blk680_dn6)), (((p.p122 * locals.var_vdsz_dn7) + locals.var_ps0z_dn7) - (assign22190_e31028 * locals.var_psislsat__blk680_dn7)), (((p.p122 * locals.var_vdsz_dn10) + locals.var_ps0z_dn10) - (assign22190_e31028 * locals.var_psislsat__blk680_dn10)), (((p.p122 * locals.var_vdsz_dn11) + locals.var_ps0z_dn11) - (assign22190_e31028 * locals.var_psislsat__blk680_dn11)), (((p.p122 * locals.var_vdsz_dn12) + locals.var_ps0z_dn12) - (assign22190_e31028 * locals.var_psislsat__blk680_dn12)), (((p.p122 * locals.var_vdsz_dn17) + locals.var_ps0z_dn17) - (assign22190_e31028 * locals.var_psislsat__blk680_dn17)),)
    } else {
        (locals.var_psisubsat__blk681, locals.var_psisubsat__blk681_dn0, locals.var_psisubsat__blk681_dn2, locals.var_psisubsat__blk681_dn6, locals.var_psisubsat__blk681_dn7, locals.var_psisubsat__blk681_dn10, locals.var_psisubsat__blk681_dn11, locals.var_psisubsat__blk681_dn12, locals.var_psisubsat__blk681_dn17,)
    }
};
        locals.var_psisubsat__blk681 = assign22190_e31033;
        locals.var_psisubsat__blk681_dn0 = assign22190_e31033_d_n0;
        locals.var_psisubsat__blk681_dn2 = assign22190_e31033_d_n2;
        locals.var_psisubsat__blk681_dn6 = assign22190_e31033_d_n6;
        locals.var_psisubsat__blk681_dn7 = assign22190_e31033_d_n7;
        locals.var_psisubsat__blk681_dn10 = assign22190_e31033_d_n10;
        locals.var_psisubsat__blk681_dn11 = assign22190_e31033_d_n11;
        locals.var_psisubsat__blk681_dn12 = assign22190_e31033_d_n12;
        locals.var_psisubsat__blk681_dn17 = assign22190_e31033_d_n17;
        locals.var_psisubsat__blk681_rv = 0.0;

        let (assign22200_e31054, assign22200_e31054_d_n0, assign22200_e31054_d_n2, assign22200_e31054_d_n6, assign22200_e31054_d_n7, assign22200_e31054_d_n10, assign22200_e31054_d_n11, assign22200_e31054_d_n12, assign22200_e31054_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign22200_e31045: f64 = (locals.var_psisubsat__blk681 * locals.var_psisubsat__blk681);
        let assign22200_e31048: f64 = (4.0 * 0.01);
        let assign22200_e31050: f64 = (assign22200_e31048 * 0.01);
        let assign22200_e31051: f64 = (assign22200_e31045 + assign22200_e31050);
        let assign22200_e31052: f64 = (assign22200_e31051).sqrt();
        (assign22200_e31052, (((locals.var_psisubsat__blk681_dn0 * locals.var_psisubsat__blk681) + (locals.var_psisubsat__blk681 * locals.var_psisubsat__blk681_dn0)) / (2.0 * assign22200_e31052)), (((locals.var_psisubsat__blk681_dn2 * locals.var_psisubsat__blk681) + (locals.var_psisubsat__blk681 * locals.var_psisubsat__blk681_dn2)) / (2.0 * assign22200_e31052)), (((locals.var_psisubsat__blk681_dn6 * locals.var_psisubsat__blk681) + (locals.var_psisubsat__blk681 * locals.var_psisubsat__blk681_dn6)) / (2.0 * assign22200_e31052)), (((locals.var_psisubsat__blk681_dn7 * locals.var_psisubsat__blk681) + (locals.var_psisubsat__blk681 * locals.var_psisubsat__blk681_dn7)) / (2.0 * assign22200_e31052)), (((locals.var_psisubsat__blk681_dn10 * locals.var_psisubsat__blk681) + (locals.var_psisubsat__blk681 * locals.var_psisubsat__blk681_dn10)) / (2.0 * assign22200_e31052)), (((locals.var_psisubsat__blk681_dn11 * locals.var_psisubsat__blk681) + (locals.var_psisubsat__blk681 * locals.var_psisubsat__blk681_dn11)) / (2.0 * assign22200_e31052)), (((locals.var_psisubsat__blk681_dn12 * locals.var_psisubsat__blk681) + (locals.var_psisubsat__blk681 * locals.var_psisubsat__blk681_dn12)) / (2.0 * assign22200_e31052)), (((locals.var_psisubsat__blk681_dn17 * locals.var_psisubsat__blk681) + (locals.var_psisubsat__blk681 * locals.var_psisubsat__blk681_dn17)) / (2.0 * assign22200_e31052)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign22200_e31054;
        locals.var_tmf1_dn0 = assign22200_e31054_d_n0;
        locals.var_tmf1_dn2 = assign22200_e31054_d_n2;
        locals.var_tmf1_dn6 = assign22200_e31054_d_n6;
        locals.var_tmf1_dn7 = assign22200_e31054_d_n7;
        locals.var_tmf1_dn10 = assign22200_e31054_d_n10;
        locals.var_tmf1_dn11 = assign22200_e31054_d_n11;
        locals.var_tmf1_dn12 = assign22200_e31054_d_n12;
        locals.var_tmf1_dn17 = assign22200_e31054_d_n17;
        locals.var_tmf1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_80(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign22210_e31074, assign22210_e31074_d_n0, assign22210_e31074_d_n2, assign22210_e31074_d_n6, assign22210_e31074_d_n7, assign22210_e31074_d_n10, assign22210_e31074_d_n11, assign22210_e31074_d_n12, assign22210_e31074_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 != 0.0)) {
        let assign22210_e31067: f64 = (locals.var_psisubsat__blk681 + locals.var_tmf1);
        let assign22210_e31068: f64 = (0.5 * assign22210_e31067);
        let assign22210_e31071: f64 = (1e-10 * 0.01);
        let assign22210_e31072: f64 = (assign22210_e31068 + assign22210_e31071);
        (assign22210_e31072, (0.5 * (locals.var_psisubsat__blk681_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_psisubsat__blk681_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_psisubsat__blk681_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_psisubsat__blk681_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_psisubsat__blk681_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_psisubsat__blk681_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_psisubsat__blk681_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_psisubsat__blk681_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_psisubsat__blk681, locals.var_psisubsat__blk681_dn0, locals.var_psisubsat__blk681_dn2, locals.var_psisubsat__blk681_dn6, locals.var_psisubsat__blk681_dn7, locals.var_psisubsat__blk681_dn10, locals.var_psisubsat__blk681_dn11, locals.var_psisubsat__blk681_dn12, locals.var_psisubsat__blk681_dn17,)
    }
};
        locals.var_psisubsat__blk681 = assign22210_e31074;
        locals.var_psisubsat__blk681_dn0 = assign22210_e31074_d_n0;
        locals.var_psisubsat__blk681_dn2 = assign22210_e31074_d_n2;
        locals.var_psisubsat__blk681_dn6 = assign22210_e31074_d_n6;
        locals.var_psisubsat__blk681_dn7 = assign22210_e31074_d_n7;
        locals.var_psisubsat__blk681_dn10 = assign22210_e31074_d_n10;
        locals.var_psisubsat__blk681_dn11 = assign22210_e31074_d_n11;
        locals.var_psisubsat__blk681_dn12 = assign22210_e31074_d_n12;
        locals.var_psisubsat__blk681_dn17 = assign22210_e31074_d_n17;
        locals.var_psisubsat__blk681_rv = 0.0;

        let assign22220_e31077: f64 = if locals.var_psisubsat__blk681 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard686 = assign22220_e31077;
        locals.var_guard686_rv = 0.0;

        let (assign22230_e31091, assign22230_e31091_d_n0, assign22230_e31091_d_n2, assign22230_e31091_d_n6, assign22230_e31091_d_n7, assign22230_e31091_d_n10, assign22230_e31091_d_n11, assign22230_e31091_d_n12, assign22230_e31091_d_n17,) = {
    if (((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 != 0.0)) && (locals.var_guard686 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_psisubsat__blk681, locals.var_psisubsat__blk681_dn0, locals.var_psisubsat__blk681_dn2, locals.var_psisubsat__blk681_dn6, locals.var_psisubsat__blk681_dn7, locals.var_psisubsat__blk681_dn10, locals.var_psisubsat__blk681_dn11, locals.var_psisubsat__blk681_dn12, locals.var_psisubsat__blk681_dn17,)
    }
};
        locals.var_psisubsat__blk681 = assign22230_e31091;
        locals.var_psisubsat__blk681_dn0 = assign22230_e31091_d_n0;
        locals.var_psisubsat__blk681_dn2 = assign22230_e31091_d_n2;
        locals.var_psisubsat__blk681_dn6 = assign22230_e31091_d_n6;
        locals.var_psisubsat__blk681_dn7 = assign22230_e31091_d_n7;
        locals.var_psisubsat__blk681_dn10 = assign22230_e31091_d_n10;
        locals.var_psisubsat__blk681_dn11 = assign22230_e31091_d_n11;
        locals.var_psisubsat__blk681_dn12 = assign22230_e31091_d_n12;
        locals.var_psisubsat__blk681_dn17 = assign22230_e31091_d_n17;
        locals.var_psisubsat__blk681_rv = 0.0;

        let (assign22240_e31106, assign22240_e31106_d_n0, assign22240_e31106_d_n2, assign22240_e31106_d_n6, assign22240_e31106_d_n7, assign22240_e31106_d_n10, assign22240_e31106_d_n11, assign22240_e31106_d_n12, assign22240_e31106_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) {
        let assign22240_e31104: f64 = (locals.var_vg2const * locals.var_vgpsub);
        (assign22240_e31104, ((locals.var_vg2const_dn0 * locals.var_vgpsub) + (locals.var_vg2const * locals.var_vgpsub_dn0)), ((locals.var_vg2const_dn2 * locals.var_vgpsub) + (locals.var_vg2const * locals.var_vgpsub_dn2)), ((locals.var_vg2const_dn6 * locals.var_vgpsub) + (locals.var_vg2const * locals.var_vgpsub_dn6)), ((locals.var_vg2const_dn7 * locals.var_vgpsub) + (locals.var_vg2const * locals.var_vgpsub_dn7)), ((locals.var_vg2const_dn10 * locals.var_vgpsub) + (locals.var_vg2const * locals.var_vgpsub_dn10)), ((locals.var_vg2const_dn11 * locals.var_vgpsub) + (locals.var_vg2const * locals.var_vgpsub_dn11)), ((locals.var_vg2const_dn12 * locals.var_vgpsub) + (locals.var_vg2const * locals.var_vgpsub_dn12)), ((locals.var_vg2const_dn17 * locals.var_vgpsub) + (locals.var_vg2const * locals.var_vgpsub_dn17)),)
    } else {
        (locals.var_t1__blk670, locals.var_t1__blk670_dn0, locals.var_t1__blk670_dn2, locals.var_t1__blk670_dn6, locals.var_t1__blk670_dn7, locals.var_t1__blk670_dn10, locals.var_t1__blk670_dn11, locals.var_t1__blk670_dn12, locals.var_t1__blk670_dn17,)
    }
};
        locals.var_t1__blk670 = assign22240_e31106;
        locals.var_t1__blk670_dn0 = assign22240_e31106_d_n0;
        locals.var_t1__blk670_dn2 = assign22240_e31106_d_n2;
        locals.var_t1__blk670_dn6 = assign22240_e31106_d_n6;
        locals.var_t1__blk670_dn7 = assign22240_e31106_d_n7;
        locals.var_t1__blk670_dn10 = assign22240_e31106_d_n10;
        locals.var_t1__blk670_dn11 = assign22240_e31106_d_n11;
        locals.var_t1__blk670_dn12 = assign22240_e31106_d_n12;
        locals.var_t1__blk670_dn17 = assign22240_e31106_d_n17;
        locals.var_t1__blk670_rv = 0.0;

        let (assign22250_e31123, assign22250_e31123_d_n0, assign22250_e31123_d_n2, assign22250_e31123_d_n6, assign22250_e31123_d_n7, assign22250_e31123_d_n10, assign22250_e31123_d_n11, assign22250_e31123_d_n12, assign22250_e31123_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) {
        let assign22250_e31120: f64 = (locals.var_c_fox * locals.var_c_fox);
        let assign22250_e31121: f64 = (locals.var_qnsub_esi / assign22250_e31120);
        (assign22250_e31121, (((locals.var_qnsub_esi_dn0 * assign22250_e31120) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn0 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn0)))) / (assign22250_e31120 * assign22250_e31120)), (((locals.var_qnsub_esi_dn2 * assign22250_e31120) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn2 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn2)))) / (assign22250_e31120 * assign22250_e31120)), (((locals.var_qnsub_esi_dn6 * assign22250_e31120) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn6 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn6)))) / (assign22250_e31120 * assign22250_e31120)), (((locals.var_qnsub_esi_dn7 * assign22250_e31120) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn7 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn7)))) / (assign22250_e31120 * assign22250_e31120)), (((locals.var_qnsub_esi_dn10 * assign22250_e31120) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn10 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn10)))) / (assign22250_e31120 * assign22250_e31120)), (((locals.var_qnsub_esi_dn11 * assign22250_e31120) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn11 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn11)))) / (assign22250_e31120 * assign22250_e31120)), (((locals.var_qnsub_esi_dn12 * assign22250_e31120) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn12 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn12)))) / (assign22250_e31120 * assign22250_e31120)), (((locals.var_qnsub_esi_dn17 * assign22250_e31120) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn17 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn17)))) / (assign22250_e31120 * assign22250_e31120)),)
    } else {
        (locals.var_t3__blk672, locals.var_t3__blk672_dn0, locals.var_t3__blk672_dn2, locals.var_t3__blk672_dn6, locals.var_t3__blk672_dn7, locals.var_t3__blk672_dn10, locals.var_t3__blk672_dn11, locals.var_t3__blk672_dn12, locals.var_t3__blk672_dn17,)
    }
};
        locals.var_t3__blk672 = assign22250_e31123;
        locals.var_t3__blk672_dn0 = assign22250_e31123_d_n0;
        locals.var_t3__blk672_dn2 = assign22250_e31123_d_n2;
        locals.var_t3__blk672_dn6 = assign22250_e31123_d_n6;
        locals.var_t3__blk672_dn7 = assign22250_e31123_d_n7;
        locals.var_t3__blk672_dn10 = assign22250_e31123_d_n10;
        locals.var_t3__blk672_dn11 = assign22250_e31123_d_n11;
        locals.var_t3__blk672_dn12 = assign22250_e31123_d_n12;
        locals.var_t3__blk672_dn17 = assign22250_e31123_d_n17;
        locals.var_t3__blk672_rv = 0.0;

        let (assign22260_e31142, assign22260_e31142_d_n0, assign22260_e31142_d_n2, assign22260_e31142_d_n6, assign22260_e31142_d_n7, assign22260_e31142_d_n10, assign22260_e31142_d_n11, assign22260_e31142_d_n12, assign22260_e31142_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) {
        let assign22260_e31136: f64 = (2.0 / locals.var_qnsub_esi);
        let assign22260_e31139: f64 = (locals.var_c_fox * locals.var_c_fox);
        let assign22260_e31140: f64 = (assign22260_e31136 * assign22260_e31139);
        (assign22260_e31140, (((-((2.0 * locals.var_qnsub_esi_dn0) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign22260_e31139) + (assign22260_e31136 * ((locals.var_c_fox_dn0 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn0)))), (((-((2.0 * locals.var_qnsub_esi_dn2) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign22260_e31139) + (assign22260_e31136 * ((locals.var_c_fox_dn2 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn2)))), (((-((2.0 * locals.var_qnsub_esi_dn6) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign22260_e31139) + (assign22260_e31136 * ((locals.var_c_fox_dn6 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn6)))), (((-((2.0 * locals.var_qnsub_esi_dn7) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign22260_e31139) + (assign22260_e31136 * ((locals.var_c_fox_dn7 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn7)))), (((-((2.0 * locals.var_qnsub_esi_dn10) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign22260_e31139) + (assign22260_e31136 * ((locals.var_c_fox_dn10 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn10)))), (((-((2.0 * locals.var_qnsub_esi_dn11) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign22260_e31139) + (assign22260_e31136 * ((locals.var_c_fox_dn11 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn11)))), (((-((2.0 * locals.var_qnsub_esi_dn12) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign22260_e31139) + (assign22260_e31136 * ((locals.var_c_fox_dn12 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn12)))), (((-((2.0 * locals.var_qnsub_esi_dn17) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign22260_e31139) + (assign22260_e31136 * ((locals.var_c_fox_dn17 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn17)))),)
    } else {
        (locals.var_t4__blk673, locals.var_t4__blk673_dn0, locals.var_t4__blk673_dn2, locals.var_t4__blk673_dn6, locals.var_t4__blk673_dn7, locals.var_t4__blk673_dn10, locals.var_t4__blk673_dn11, locals.var_t4__blk673_dn12, locals.var_t4__blk673_dn17,)
    }
};
        locals.var_t4__blk673 = assign22260_e31142;
        locals.var_t4__blk673_dn0 = assign22260_e31142_d_n0;
        locals.var_t4__blk673_dn2 = assign22260_e31142_d_n2;
        locals.var_t4__blk673_dn6 = assign22260_e31142_d_n6;
        locals.var_t4__blk673_dn7 = assign22260_e31142_d_n7;
        locals.var_t4__blk673_dn10 = assign22260_e31142_d_n10;
        locals.var_t4__blk673_dn11 = assign22260_e31142_d_n11;
        locals.var_t4__blk673_dn12 = assign22260_e31142_d_n12;
        locals.var_t4__blk673_dn17 = assign22260_e31142_d_n17;
        locals.var_t4__blk673_rv = 0.0;

        let (assign22270_e31161, assign22270_e31161_d_n0, assign22270_e31161_d_n2, assign22270_e31161_d_n6, assign22270_e31161_d_n7, assign22270_e31161_d_n10, assign22270_e31161_d_n11, assign22270_e31161_d_n12, assign22270_e31161_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) {
        let assign22270_e31155: f64 = (locals.var_t1__blk670 - locals.var_beta_inv);
        let assign22270_e31158: f64 = (locals.var_xvbs * locals.var_vbspz);
        let assign22270_e31159: f64 = (assign22270_e31155 - assign22270_e31158);
        (assign22270_e31159, (locals.var_t1__blk670_dn0 - (locals.var_xvbs * locals.var_vbspz_dn0)), (locals.var_t1__blk670_dn2 - (locals.var_xvbs * locals.var_vbspz_dn2)), (locals.var_t1__blk670_dn6 - (locals.var_xvbs * locals.var_vbspz_dn6)), (locals.var_t1__blk670_dn7 - (locals.var_xvbs * locals.var_vbspz_dn7)), ((locals.var_t1__blk670_dn10 - locals.var_beta_inv_dn10) - (locals.var_xvbs * locals.var_vbspz_dn10)), (locals.var_t1__blk670_dn11 - (locals.var_xvbs * locals.var_vbspz_dn11)), (locals.var_t1__blk670_dn12 - (locals.var_xvbs * locals.var_vbspz_dn12)), (locals.var_t1__blk670_dn17 - (locals.var_xvbs * locals.var_vbspz_dn17)),)
    } else {
        (locals.var_t5__blk674, locals.var_t5__blk674_dn0, locals.var_t5__blk674_dn2, locals.var_t5__blk674_dn6, locals.var_t5__blk674_dn7, locals.var_t5__blk674_dn10, locals.var_t5__blk674_dn11, locals.var_t5__blk674_dn12, locals.var_t5__blk674_dn17,)
    }
};
        locals.var_t5__blk674 = assign22270_e31161;
        locals.var_t5__blk674_dn0 = assign22270_e31161_d_n0;
        locals.var_t5__blk674_dn2 = assign22270_e31161_d_n2;
        locals.var_t5__blk674_dn6 = assign22270_e31161_d_n6;
        locals.var_t5__blk674_dn7 = assign22270_e31161_d_n7;
        locals.var_t5__blk674_dn10 = assign22270_e31161_d_n10;
        locals.var_t5__blk674_dn11 = assign22270_e31161_d_n11;
        locals.var_t5__blk674_dn12 = assign22270_e31161_d_n12;
        locals.var_t5__blk674_dn17 = assign22270_e31161_d_n17;
        locals.var_t5__blk674_rv = 0.0;

        let (assign22280_e31178, assign22280_e31178_d_n0, assign22280_e31178_d_n2, assign22280_e31178_d_n6, assign22280_e31178_d_n7, assign22280_e31178_d_n10, assign22280_e31178_d_n11, assign22280_e31178_d_n12, assign22280_e31178_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) {
        let assign22280_e31174: f64 = (p.p49 * locals.var_qhs);
        let assign22280_e31176: f64 = (assign22280_e31174 / locals.var_c_soi);
        (assign22280_e31176, ((p.p49 * locals.var_qhs_dn0) / locals.var_c_soi), ((p.p49 * locals.var_qhs_dn2) / locals.var_c_soi), ((p.p49 * locals.var_qhs_dn6) / locals.var_c_soi), ((p.p49 * locals.var_qhs_dn7) / locals.var_c_soi), ((p.p49 * locals.var_qhs_dn10) / locals.var_c_soi), ((p.p49 * locals.var_qhs_dn11) / locals.var_c_soi), ((p.p49 * locals.var_qhs_dn12) / locals.var_c_soi), ((p.p49 * locals.var_qhs_dn17) / locals.var_c_soi),)
    } else {
        (locals.var_dvbssub, locals.var_dvbssub_dn0, locals.var_dvbssub_dn2, locals.var_dvbssub_dn6, locals.var_dvbssub_dn7, locals.var_dvbssub_dn10, locals.var_dvbssub_dn11, locals.var_dvbssub_dn12, locals.var_dvbssub_dn17,)
    }
};
        locals.var_dvbssub = assign22280_e31178;
        locals.var_dvbssub_dn0 = assign22280_e31178_d_n0;
        locals.var_dvbssub_dn2 = assign22280_e31178_d_n2;
        locals.var_dvbssub_dn6 = assign22280_e31178_d_n6;
        locals.var_dvbssub_dn7 = assign22280_e31178_d_n7;
        locals.var_dvbssub_dn10 = assign22280_e31178_d_n10;
        locals.var_dvbssub_dn11 = assign22280_e31178_d_n11;
        locals.var_dvbssub_dn12 = assign22280_e31178_d_n12;
        locals.var_dvbssub_dn17 = assign22280_e31178_d_n17;
        locals.var_dvbssub_rv = 0.0;

        let (assign22290_e31195, assign22290_e31195_d_n0, assign22290_e31195_d_n2, assign22290_e31195_d_n6, assign22290_e31195_d_n7, assign22290_e31195_d_n10, assign22290_e31195_d_n11, assign22290_e31195_d_n12, assign22290_e31195_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) {
        let assign22290_e31192: f64 = (locals.var_xvbs * locals.var_dvbssub);
        let assign22290_e31193: f64 = (locals.var_t5__blk674 - assign22290_e31192);
        (assign22290_e31193, (locals.var_t5__blk674_dn0 - (locals.var_xvbs * locals.var_dvbssub_dn0)), (locals.var_t5__blk674_dn2 - (locals.var_xvbs * locals.var_dvbssub_dn2)), (locals.var_t5__blk674_dn6 - (locals.var_xvbs * locals.var_dvbssub_dn6)), (locals.var_t5__blk674_dn7 - (locals.var_xvbs * locals.var_dvbssub_dn7)), (locals.var_t5__blk674_dn10 - (locals.var_xvbs * locals.var_dvbssub_dn10)), (locals.var_t5__blk674_dn11 - (locals.var_xvbs * locals.var_dvbssub_dn11)), (locals.var_t5__blk674_dn12 - (locals.var_xvbs * locals.var_dvbssub_dn12)), (locals.var_t5__blk674_dn17 - (locals.var_xvbs * locals.var_dvbssub_dn17)),)
    } else {
        (locals.var_t5__blk674, locals.var_t5__blk674_dn0, locals.var_t5__blk674_dn2, locals.var_t5__blk674_dn6, locals.var_t5__blk674_dn7, locals.var_t5__blk674_dn10, locals.var_t5__blk674_dn11, locals.var_t5__blk674_dn12, locals.var_t5__blk674_dn17,)
    }
};
        locals.var_t5__blk674 = assign22290_e31195;
        locals.var_t5__blk674_dn0 = assign22290_e31195_d_n0;
        locals.var_t5__blk674_dn2 = assign22290_e31195_d_n2;
        locals.var_t5__blk674_dn6 = assign22290_e31195_d_n6;
        locals.var_t5__blk674_dn7 = assign22290_e31195_d_n7;
        locals.var_t5__blk674_dn10 = assign22290_e31195_d_n10;
        locals.var_t5__blk674_dn11 = assign22290_e31195_d_n11;
        locals.var_t5__blk674_dn12 = assign22290_e31195_d_n12;
        locals.var_t5__blk674_dn17 = assign22290_e31195_d_n17;
        locals.var_t5__blk674_rv = 0.0;

        let (assign22300_e31212, assign22300_e31212_d_n0, assign22300_e31212_d_n2, assign22300_e31212_d_n6, assign22300_e31212_d_n7, assign22300_e31212_d_n10, assign22300_e31212_d_n11, assign22300_e31212_d_n12, assign22300_e31212_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) {
        let assign22300_e31209: f64 = (locals.var_t4__blk673 * locals.var_t5__blk674);
        let assign22300_e31210: f64 = (1.0 + assign22300_e31209);
        (assign22300_e31210, ((locals.var_t4__blk673_dn0 * locals.var_t5__blk674) + (locals.var_t4__blk673 * locals.var_t5__blk674_dn0)), ((locals.var_t4__blk673_dn2 * locals.var_t5__blk674) + (locals.var_t4__blk673 * locals.var_t5__blk674_dn2)), ((locals.var_t4__blk673_dn6 * locals.var_t5__blk674) + (locals.var_t4__blk673 * locals.var_t5__blk674_dn6)), ((locals.var_t4__blk673_dn7 * locals.var_t5__blk674) + (locals.var_t4__blk673 * locals.var_t5__blk674_dn7)), ((locals.var_t4__blk673_dn10 * locals.var_t5__blk674) + (locals.var_t4__blk673 * locals.var_t5__blk674_dn10)), ((locals.var_t4__blk673_dn11 * locals.var_t5__blk674) + (locals.var_t4__blk673 * locals.var_t5__blk674_dn11)), ((locals.var_t4__blk673_dn12 * locals.var_t5__blk674) + (locals.var_t4__blk673 * locals.var_t5__blk674_dn12)), ((locals.var_t4__blk673_dn17 * locals.var_t5__blk674) + (locals.var_t4__blk673 * locals.var_t5__blk674_dn17)),)
    } else {
        (locals.var_t6__blk675, locals.var_t6__blk675_dn0, locals.var_t6__blk675_dn2, locals.var_t6__blk675_dn6, locals.var_t6__blk675_dn7, locals.var_t6__blk675_dn10, locals.var_t6__blk675_dn11, locals.var_t6__blk675_dn12, locals.var_t6__blk675_dn17,)
    }
};
        locals.var_t6__blk675 = assign22300_e31212;
        locals.var_t6__blk675_dn0 = assign22300_e31212_d_n0;
        locals.var_t6__blk675_dn2 = assign22300_e31212_d_n2;
        locals.var_t6__blk675_dn6 = assign22300_e31212_d_n6;
        locals.var_t6__blk675_dn7 = assign22300_e31212_d_n7;
        locals.var_t6__blk675_dn10 = assign22300_e31212_d_n10;
        locals.var_t6__blk675_dn11 = assign22300_e31212_d_n11;
        locals.var_t6__blk675_dn12 = assign22300_e31212_d_n12;
        locals.var_t6__blk675_dn17 = assign22300_e31212_d_n17;
        locals.var_t6__blk675_rv = 0.0;

        let (assign22310_e31229, assign22310_e31229_d_n0, assign22310_e31229_d_n2, assign22310_e31229_d_n6, assign22310_e31229_d_n7, assign22310_e31229_d_n10, assign22310_e31229_d_n11, assign22310_e31229_d_n12, assign22310_e31229_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) {
        let assign22310_e31226: f64 = (1.0 + locals.var_t4__blk673);
        let assign22310_e31227: f64 = (2.0 * assign22310_e31226);
        (assign22310_e31227, (2.0 * locals.var_t4__blk673_dn0), (2.0 * locals.var_t4__blk673_dn2), (2.0 * locals.var_t4__blk673_dn6), (2.0 * locals.var_t4__blk673_dn7), (2.0 * locals.var_t4__blk673_dn10), (2.0 * locals.var_t4__blk673_dn11), (2.0 * locals.var_t4__blk673_dn12), (2.0 * locals.var_t4__blk673_dn17),)
    } else {
        (locals.var_t7__blk677, locals.var_t7__blk677_dn0, locals.var_t7__blk677_dn2, locals.var_t7__blk677_dn6, locals.var_t7__blk677_dn7, locals.var_t7__blk677_dn10, locals.var_t7__blk677_dn11, locals.var_t7__blk677_dn12, locals.var_t7__blk677_dn17,)
    }
};
        locals.var_t7__blk677 = assign22310_e31229;
        locals.var_t7__blk677_dn0 = assign22310_e31229_d_n0;
        locals.var_t7__blk677_dn2 = assign22310_e31229_d_n2;
        locals.var_t7__blk677_dn6 = assign22310_e31229_d_n6;
        locals.var_t7__blk677_dn7 = assign22310_e31229_d_n7;
        locals.var_t7__blk677_dn10 = assign22310_e31229_d_n10;
        locals.var_t7__blk677_dn11 = assign22310_e31229_d_n11;
        locals.var_t7__blk677_dn12 = assign22310_e31229_d_n12;
        locals.var_t7__blk677_dn17 = assign22310_e31229_d_n17;
        locals.var_t7__blk677_rv = 0.0;

        let assign22320_e31233: f64 = (1e-50 + locals.var_t7__blk677);
        let assign22320_e31238: f64 = if ((locals.var_t6__blk675 < assign22320_e31233) && (locals.var_t7__blk677 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard687 = assign22320_e31238;
        locals.var_guard687_rv = 0.0;

        let (assign22330_e31257, assign22330_e31257_d_n0, assign22330_e31257_d_n2, assign22330_e31257_d_n6, assign22330_e31257_d_n7, assign22330_e31257_d_n10, assign22330_e31257_d_n11, assign22330_e31257_d_n12, assign22330_e31257_d_n17,) = {
    if (((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign22330_e31253: f64 = (1e-50 + locals.var_t7__blk677);
        let assign22330_e31255: f64 = (assign22330_e31253 - locals.var_t6__blk675);
        (assign22330_e31255, (locals.var_t7__blk677_dn0 - locals.var_t6__blk675_dn0), (locals.var_t7__blk677_dn2 - locals.var_t6__blk675_dn2), (locals.var_t7__blk677_dn6 - locals.var_t6__blk675_dn6), (locals.var_t7__blk677_dn7 - locals.var_t6__blk675_dn7), (locals.var_t7__blk677_dn10 - locals.var_t6__blk675_dn10), (locals.var_t7__blk677_dn11 - locals.var_t6__blk675_dn11), (locals.var_t7__blk677_dn12 - locals.var_t6__blk675_dn12), (locals.var_t7__blk677_dn17 - locals.var_t6__blk675_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign22330_e31257;
        locals.var_tmf1_dn0 = assign22330_e31257_d_n0;
        locals.var_tmf1_dn2 = assign22330_e31257_d_n2;
        locals.var_tmf1_dn6 = assign22330_e31257_d_n6;
        locals.var_tmf1_dn7 = assign22330_e31257_d_n7;
        locals.var_tmf1_dn10 = assign22330_e31257_d_n10;
        locals.var_tmf1_dn11 = assign22330_e31257_d_n11;
        locals.var_tmf1_dn12 = assign22330_e31257_d_n12;
        locals.var_tmf1_dn17 = assign22330_e31257_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign22340_e31274, assign22340_e31274_d_n0, assign22340_e31274_d_n2, assign22340_e31274_d_n6, assign22340_e31274_d_n7, assign22340_e31274_d_n10, assign22340_e31274_d_n11, assign22340_e31274_d_n12, assign22340_e31274_d_n17,) = {
    if (((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign22340_e31272: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign22340_e31272, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12, locals.var_x2_dn17,)
    }
};
        locals.var_x2 = assign22340_e31274;
        locals.var_x2_dn0 = assign22340_e31274_d_n0;
        locals.var_x2_dn2 = assign22340_e31274_d_n2;
        locals.var_x2_dn6 = assign22340_e31274_d_n6;
        locals.var_x2_dn7 = assign22340_e31274_d_n7;
        locals.var_x2_dn10 = assign22340_e31274_d_n10;
        locals.var_x2_dn11 = assign22340_e31274_d_n11;
        locals.var_x2_dn12 = assign22340_e31274_d_n12;
        locals.var_x2_dn17 = assign22340_e31274_d_n17;
        locals.var_x2_rv = 0.0;

        let (assign22350_e31291, assign22350_e31291_d_n0, assign22350_e31291_d_n2, assign22350_e31291_d_n6, assign22350_e31291_d_n7, assign22350_e31291_d_n10, assign22350_e31291_d_n11, assign22350_e31291_d_n12, assign22350_e31291_d_n17,) = {
    if (((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign22350_e31289: f64 = (locals.var_t7__blk677 * locals.var_t7__blk677);
        (assign22350_e31289, ((locals.var_t7__blk677_dn0 * locals.var_t7__blk677) + (locals.var_t7__blk677 * locals.var_t7__blk677_dn0)), ((locals.var_t7__blk677_dn2 * locals.var_t7__blk677) + (locals.var_t7__blk677 * locals.var_t7__blk677_dn2)), ((locals.var_t7__blk677_dn6 * locals.var_t7__blk677) + (locals.var_t7__blk677 * locals.var_t7__blk677_dn6)), ((locals.var_t7__blk677_dn7 * locals.var_t7__blk677) + (locals.var_t7__blk677 * locals.var_t7__blk677_dn7)), ((locals.var_t7__blk677_dn10 * locals.var_t7__blk677) + (locals.var_t7__blk677 * locals.var_t7__blk677_dn10)), ((locals.var_t7__blk677_dn11 * locals.var_t7__blk677) + (locals.var_t7__blk677 * locals.var_t7__blk677_dn11)), ((locals.var_t7__blk677_dn12 * locals.var_t7__blk677) + (locals.var_t7__blk677 * locals.var_t7__blk677_dn12)), ((locals.var_t7__blk677_dn17 * locals.var_t7__blk677) + (locals.var_t7__blk677 * locals.var_t7__blk677_dn17)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
        locals.var_xmax2 = assign22350_e31291;
        locals.var_xmax2_dn0 = assign22350_e31291_d_n0;
        locals.var_xmax2_dn2 = assign22350_e31291_d_n2;
        locals.var_xmax2_dn6 = assign22350_e31291_d_n6;
        locals.var_xmax2_dn7 = assign22350_e31291_d_n7;
        locals.var_xmax2_dn10 = assign22350_e31291_d_n10;
        locals.var_xmax2_dn11 = assign22350_e31291_d_n11;
        locals.var_xmax2_dn12 = assign22350_e31291_d_n12;
        locals.var_xmax2_dn17 = assign22350_e31291_d_n17;
        locals.var_xmax2_rv = 0.0;

        let (assign22360_e31306, assign22360_e31306_d_n0, assign22360_e31306_d_n2, assign22360_e31306_d_n6, assign22360_e31306_d_n7, assign22360_e31306_d_n10, assign22360_e31306_d_n11, assign22360_e31306_d_n12, assign22360_e31306_d_n17,) = {
    if (((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard687 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign22360_e31306;
        locals.var_xp_dn0 = assign22360_e31306_d_n0;
        locals.var_xp_dn2 = assign22360_e31306_d_n2;
        locals.var_xp_dn6 = assign22360_e31306_d_n6;
        locals.var_xp_dn7 = assign22360_e31306_d_n7;
        locals.var_xp_dn10 = assign22360_e31306_d_n10;
        locals.var_xp_dn11 = assign22360_e31306_d_n11;
        locals.var_xp_dn12 = assign22360_e31306_d_n12;
        locals.var_xp_dn17 = assign22360_e31306_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign22370_e31321, assign22370_e31321_d_n0, assign22370_e31321_d_n2, assign22370_e31321_d_n6, assign22370_e31321_d_n7, assign22370_e31321_d_n10, assign22370_e31321_d_n11, assign22370_e31321_d_n12, assign22370_e31321_d_n17,) = {
    if (((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard687 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign22370_e31321;
        locals.var_xmp_dn0 = assign22370_e31321_d_n0;
        locals.var_xmp_dn2 = assign22370_e31321_d_n2;
        locals.var_xmp_dn6 = assign22370_e31321_d_n6;
        locals.var_xmp_dn7 = assign22370_e31321_d_n7;
        locals.var_xmp_dn10 = assign22370_e31321_d_n10;
        locals.var_xmp_dn11 = assign22370_e31321_d_n11;
        locals.var_xmp_dn12 = assign22370_e31321_d_n12;
        locals.var_xmp_dn17 = assign22370_e31321_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign22380_e31336,) = {
    if (((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard687 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign22380_e31336;
        locals.var_m0_rv = 0.0;

        let (assign22390_e31351,) = {
    if (((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard687 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign22390_e31351;
        locals.var_mm_rv = 0.0;

        let (assign22400_e31366, assign22400_e31366_d_n0, assign22400_e31366_d_n2, assign22400_e31366_d_n6, assign22400_e31366_d_n7, assign22400_e31366_d_n10, assign22400_e31366_d_n11, assign22400_e31366_d_n12, assign22400_e31366_d_n17,) = {
    if (((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard687 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign22400_e31366;
        locals.var_arg_dn0 = assign22400_e31366_d_n0;
        locals.var_arg_dn2 = assign22400_e31366_d_n2;
        locals.var_arg_dn6 = assign22400_e31366_d_n6;
        locals.var_arg_dn7 = assign22400_e31366_d_n7;
        locals.var_arg_dn10 = assign22400_e31366_d_n10;
        locals.var_arg_dn11 = assign22400_e31366_d_n11;
        locals.var_arg_dn12 = assign22400_e31366_d_n12;
        locals.var_arg_dn17 = assign22400_e31366_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign22410_e31381, assign22410_e31381_d_n0, assign22410_e31381_d_n2, assign22410_e31381_d_n6, assign22410_e31381_d_n7, assign22410_e31381_d_n10, assign22410_e31381_d_n11, assign22410_e31381_d_n12, assign22410_e31381_d_n17,) = {
    if (((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard687 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign22410_e31381;
        locals.var_dnm_dn0 = assign22410_e31381_d_n0;
        locals.var_dnm_dn2 = assign22410_e31381_d_n2;
        locals.var_dnm_dn6 = assign22410_e31381_d_n6;
        locals.var_dnm_dn7 = assign22410_e31381_d_n7;
        locals.var_dnm_dn10 = assign22410_e31381_d_n10;
        locals.var_dnm_dn11 = assign22410_e31381_d_n11;
        locals.var_dnm_dn12 = assign22410_e31381_d_n12;
        locals.var_dnm_dn17 = assign22410_e31381_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign22420_e31398, assign22420_e31398_d_n0, assign22420_e31398_d_n2, assign22420_e31398_d_n6, assign22420_e31398_d_n7, assign22420_e31398_d_n10, assign22420_e31398_d_n11, assign22420_e31398_d_n12, assign22420_e31398_d_n17,) = {
    if (((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign22420_e31396: f64 = (locals.var_xp * locals.var_x2);
        (assign22420_e31396, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign22420_e31398;
        locals.var_xp_dn0 = assign22420_e31398_d_n0;
        locals.var_xp_dn2 = assign22420_e31398_d_n2;
        locals.var_xp_dn6 = assign22420_e31398_d_n6;
        locals.var_xp_dn7 = assign22420_e31398_d_n7;
        locals.var_xp_dn10 = assign22420_e31398_d_n10;
        locals.var_xp_dn11 = assign22420_e31398_d_n11;
        locals.var_xp_dn12 = assign22420_e31398_d_n12;
        locals.var_xp_dn17 = assign22420_e31398_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign22430_e31415, assign22430_e31415_d_n0, assign22430_e31415_d_n2, assign22430_e31415_d_n6, assign22430_e31415_d_n7, assign22430_e31415_d_n10, assign22430_e31415_d_n11, assign22430_e31415_d_n12, assign22430_e31415_d_n17,) = {
    if (((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign22430_e31413: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign22430_e31413, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign22430_e31415;
        locals.var_xmp_dn0 = assign22430_e31415_d_n0;
        locals.var_xmp_dn2 = assign22430_e31415_d_n2;
        locals.var_xmp_dn6 = assign22430_e31415_d_n6;
        locals.var_xmp_dn7 = assign22430_e31415_d_n7;
        locals.var_xmp_dn10 = assign22430_e31415_d_n10;
        locals.var_xmp_dn11 = assign22430_e31415_d_n11;
        locals.var_xmp_dn12 = assign22430_e31415_d_n12;
        locals.var_xmp_dn17 = assign22430_e31415_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign22440_e31432, assign22440_e31432_d_n0, assign22440_e31432_d_n2, assign22440_e31432_d_n6, assign22440_e31432_d_n7, assign22440_e31432_d_n10, assign22440_e31432_d_n11, assign22440_e31432_d_n12, assign22440_e31432_d_n17,) = {
    if (((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign22440_e31430: f64 = (locals.var_xp * locals.var_x2);
        (assign22440_e31430, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign22440_e31432;
        locals.var_xp_dn0 = assign22440_e31432_d_n0;
        locals.var_xp_dn2 = assign22440_e31432_d_n2;
        locals.var_xp_dn6 = assign22440_e31432_d_n6;
        locals.var_xp_dn7 = assign22440_e31432_d_n7;
        locals.var_xp_dn10 = assign22440_e31432_d_n10;
        locals.var_xp_dn11 = assign22440_e31432_d_n11;
        locals.var_xp_dn12 = assign22440_e31432_d_n12;
        locals.var_xp_dn17 = assign22440_e31432_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign22450_e31449, assign22450_e31449_d_n0, assign22450_e31449_d_n2, assign22450_e31449_d_n6, assign22450_e31449_d_n7, assign22450_e31449_d_n10, assign22450_e31449_d_n11, assign22450_e31449_d_n12, assign22450_e31449_d_n17,) = {
    if (((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign22450_e31447: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign22450_e31447, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign22450_e31449;
        locals.var_xmp_dn0 = assign22450_e31449_d_n0;
        locals.var_xmp_dn2 = assign22450_e31449_d_n2;
        locals.var_xmp_dn6 = assign22450_e31449_d_n6;
        locals.var_xmp_dn7 = assign22450_e31449_d_n7;
        locals.var_xmp_dn10 = assign22450_e31449_d_n10;
        locals.var_xmp_dn11 = assign22450_e31449_d_n11;
        locals.var_xmp_dn12 = assign22450_e31449_d_n12;
        locals.var_xmp_dn17 = assign22450_e31449_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign22460_e31466, assign22460_e31466_d_n0, assign22460_e31466_d_n2, assign22460_e31466_d_n6, assign22460_e31466_d_n7, assign22460_e31466_d_n10, assign22460_e31466_d_n11, assign22460_e31466_d_n12, assign22460_e31466_d_n17,) = {
    if (((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign22460_e31464: f64 = (locals.var_xp * locals.var_x2);
        (assign22460_e31464, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign22460_e31466;
        locals.var_xp_dn0 = assign22460_e31466_d_n0;
        locals.var_xp_dn2 = assign22460_e31466_d_n2;
        locals.var_xp_dn6 = assign22460_e31466_d_n6;
        locals.var_xp_dn7 = assign22460_e31466_d_n7;
        locals.var_xp_dn10 = assign22460_e31466_d_n10;
        locals.var_xp_dn11 = assign22460_e31466_d_n11;
        locals.var_xp_dn12 = assign22460_e31466_d_n12;
        locals.var_xp_dn17 = assign22460_e31466_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign22470_e31483, assign22470_e31483_d_n0, assign22470_e31483_d_n2, assign22470_e31483_d_n6, assign22470_e31483_d_n7, assign22470_e31483_d_n10, assign22470_e31483_d_n11, assign22470_e31483_d_n12, assign22470_e31483_d_n17,) = {
    if (((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign22470_e31481: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign22470_e31481, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign22470_e31483;
        locals.var_xmp_dn0 = assign22470_e31483_d_n0;
        locals.var_xmp_dn2 = assign22470_e31483_d_n2;
        locals.var_xmp_dn6 = assign22470_e31483_d_n6;
        locals.var_xmp_dn7 = assign22470_e31483_d_n7;
        locals.var_xmp_dn10 = assign22470_e31483_d_n10;
        locals.var_xmp_dn11 = assign22470_e31483_d_n11;
        locals.var_xmp_dn12 = assign22470_e31483_d_n12;
        locals.var_xmp_dn17 = assign22470_e31483_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign22480_e31500, assign22480_e31500_d_n0, assign22480_e31500_d_n2, assign22480_e31500_d_n6, assign22480_e31500_d_n7, assign22480_e31500_d_n10, assign22480_e31500_d_n11, assign22480_e31500_d_n12, assign22480_e31500_d_n17,) = {
    if (((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign22480_e31498: f64 = (locals.var_xp * locals.var_x2);
        (assign22480_e31498, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign22480_e31500;
        locals.var_xp_dn0 = assign22480_e31500_d_n0;
        locals.var_xp_dn2 = assign22480_e31500_d_n2;
        locals.var_xp_dn6 = assign22480_e31500_d_n6;
        locals.var_xp_dn7 = assign22480_e31500_d_n7;
        locals.var_xp_dn10 = assign22480_e31500_d_n10;
        locals.var_xp_dn11 = assign22480_e31500_d_n11;
        locals.var_xp_dn12 = assign22480_e31500_d_n12;
        locals.var_xp_dn17 = assign22480_e31500_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign22490_e31517, assign22490_e31517_d_n0, assign22490_e31517_d_n2, assign22490_e31517_d_n6, assign22490_e31517_d_n7, assign22490_e31517_d_n10, assign22490_e31517_d_n11, assign22490_e31517_d_n12, assign22490_e31517_d_n17,) = {
    if (((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign22490_e31515: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign22490_e31515, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign22490_e31517;
        locals.var_xmp_dn0 = assign22490_e31517_d_n0;
        locals.var_xmp_dn2 = assign22490_e31517_d_n2;
        locals.var_xmp_dn6 = assign22490_e31517_d_n6;
        locals.var_xmp_dn7 = assign22490_e31517_d_n7;
        locals.var_xmp_dn10 = assign22490_e31517_d_n10;
        locals.var_xmp_dn11 = assign22490_e31517_d_n11;
        locals.var_xmp_dn12 = assign22490_e31517_d_n12;
        locals.var_xmp_dn17 = assign22490_e31517_d_n17;
        locals.var_xmp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_81(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign22500_e31534, assign22500_e31534_d_n0, assign22500_e31534_d_n2, assign22500_e31534_d_n6, assign22500_e31534_d_n7, assign22500_e31534_d_n10, assign22500_e31534_d_n11, assign22500_e31534_d_n12, assign22500_e31534_d_n17,) = {
    if (((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign22500_e31532: f64 = (locals.var_xp + locals.var_xmp);
        (assign22500_e31532, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12), (locals.var_xp_dn17 + locals.var_xmp_dn17),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign22500_e31534;
        locals.var_arg_dn0 = assign22500_e31534_d_n0;
        locals.var_arg_dn2 = assign22500_e31534_d_n2;
        locals.var_arg_dn6 = assign22500_e31534_d_n6;
        locals.var_arg_dn7 = assign22500_e31534_d_n7;
        locals.var_arg_dn10 = assign22500_e31534_d_n10;
        locals.var_arg_dn11 = assign22500_e31534_d_n11;
        locals.var_arg_dn12 = assign22500_e31534_d_n12;
        locals.var_arg_dn17 = assign22500_e31534_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign22510_e31549, assign22510_e31549_d_n0, assign22510_e31549_d_n2, assign22510_e31549_d_n6, assign22510_e31549_d_n7, assign22510_e31549_d_n10, assign22510_e31549_d_n11, assign22510_e31549_d_n12, assign22510_e31549_d_n17,) = {
    if (((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard687 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign22510_e31549;
        locals.var_dnm_dn0 = assign22510_e31549_d_n0;
        locals.var_dnm_dn2 = assign22510_e31549_d_n2;
        locals.var_dnm_dn6 = assign22510_e31549_d_n6;
        locals.var_dnm_dn7 = assign22510_e31549_d_n7;
        locals.var_dnm_dn10 = assign22510_e31549_d_n10;
        locals.var_dnm_dn11 = assign22510_e31549_d_n11;
        locals.var_dnm_dn12 = assign22510_e31549_d_n12;
        locals.var_dnm_dn17 = assign22510_e31549_d_n17;
        locals.var_dnm_rv = 0.0;

        let assign22520_e31564: f64 = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard688 = assign22520_e31564;
        locals.var_guard688_rv = 0.0;

        let assign22530_e31567: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard689 = assign22530_e31567;
        locals.var_guard689_rv = 0.0;

        let (assign22540_e31586,) = {
    if (((((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard687 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard689 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign22540_e31586;
        locals.var_mm_rv = 0.0;

        let assign22550_e31589: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard690 = assign22550_e31589;
        locals.var_guard690_rv = 0.0;

        let (assign22560_e31611,) = {
    if ((((((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard687 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard690 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign22560_e31611;
        locals.var_mm_rv = 0.0;

        let assign22570_e31614: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard691 = assign22570_e31614;
        locals.var_guard691_rv = 0.0;

        let (assign22580_e31639,) = {
    if (((((((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard687 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard690 == 0.0)) && (locals.var_guard691 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign22580_e31639;
        locals.var_mm_rv = 0.0;

        let assign22590_e31642: f64 = if 4.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard692 = assign22590_e31642;
        locals.var_guard692_rv = 0.0;

        let (assign22600_e31670,) = {
    if ((((((((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard687 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_guard689 == 0.0)) && (locals.var_guard690 == 0.0)) && (locals.var_guard691 == 0.0)) && (locals.var_guard692 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign22600_e31670;
        locals.var_mm_rv = 0.0;

        let (assign22610_e31687,) = {
    if ((((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard687 != 0.0)) && (locals.var_guard688 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign22610_e31687;
        locals.var_m0_rv = 0.0;

        let mut assign22620_loop_guard: usize = 0;
        while {
            let assign22620_cond_e31705: f64 = if (((((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard687 != 0.0)) && (locals.var_guard688 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign22620_cond_e31705 != 0.0
        } {
            assign22620_loop_guard += 1;
            assert!(assign22620_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign22620_body0_e31723, assign22620_body0_e31723_d_n0, assign22620_body0_e31723_d_n2, assign22620_body0_e31723_d_n6, assign22620_body0_e31723_d_n7, assign22620_body0_e31723_d_n10, assign22620_body0_e31723_d_n11, assign22620_body0_e31723_d_n12, assign22620_body0_e31723_d_n17,) = {
    if ((((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard687 != 0.0)) && (locals.var_guard688 != 0.0)) {
        let assign22620_body0_e31721: f64 = (locals.var_dnm).sqrt();
        (assign22620_body0_e31721, (locals.var_dnm_dn0 / (2.0 * assign22620_body0_e31721)), (locals.var_dnm_dn2 / (2.0 * assign22620_body0_e31721)), (locals.var_dnm_dn6 / (2.0 * assign22620_body0_e31721)), (locals.var_dnm_dn7 / (2.0 * assign22620_body0_e31721)), (locals.var_dnm_dn10 / (2.0 * assign22620_body0_e31721)), (locals.var_dnm_dn11 / (2.0 * assign22620_body0_e31721)), (locals.var_dnm_dn12 / (2.0 * assign22620_body0_e31721)), (locals.var_dnm_dn17 / (2.0 * assign22620_body0_e31721)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign22620_body0_e31723;
            locals.var_dnm_dn0 = assign22620_body0_e31723_d_n0;
            locals.var_dnm_dn2 = assign22620_body0_e31723_d_n2;
            locals.var_dnm_dn6 = assign22620_body0_e31723_d_n6;
            locals.var_dnm_dn7 = assign22620_body0_e31723_d_n7;
            locals.var_dnm_dn10 = assign22620_body0_e31723_d_n10;
            locals.var_dnm_dn11 = assign22620_body0_e31723_d_n11;
            locals.var_dnm_dn12 = assign22620_body0_e31723_d_n12;
            locals.var_dnm_dn17 = assign22620_body0_e31723_d_n17;
            locals.var_dnm_rv = 0.0;
            let (assign22620_body1_e31742,) = {
    if ((((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard687 != 0.0)) && (locals.var_guard688 != 0.0)) {
        let assign22620_body1_e31740: f64 = (locals.var_m0 + 1.0);
        (assign22620_body1_e31740,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign22620_body1_e31742;
            locals.var_m0_rv = 0.0;
        }

        let (assign22630_e31766, assign22630_e31766_d_n0, assign22630_e31766_d_n2, assign22630_e31766_d_n6, assign22630_e31766_d_n7, assign22630_e31766_d_n10, assign22630_e31766_d_n11, assign22630_e31766_d_n12, assign22630_e31766_d_n17,) = {
    if ((((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard687 != 0.0)) && (locals.var_guard688 == 0.0)) {
        let assign22630_e31762: f64 = (2.0 * 4.0);
        let assign22630_e31763: f64 = (1.0 / assign22630_e31762);
        let assign22630_e31764: f64 = (locals.var_dnm).powf(assign22630_e31763);
        (assign22630_e31764, if 0.0 == 0.0 && ((assign22630_e31763) as f64).is_finite() && ((assign22630_e31763) as f64).fract() == 0.0 { if assign22630_e31763 == 0.0 { 0.0 } else { (assign22630_e31763 * ((locals.var_dnm).powf(assign22630_e31763 - 1.0) * locals.var_dnm_dn0)) } } else { (assign22630_e31764 * (assign22630_e31763 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign22630_e31763) as f64).is_finite() && ((assign22630_e31763) as f64).fract() == 0.0 { if assign22630_e31763 == 0.0 { 0.0 } else { (assign22630_e31763 * ((locals.var_dnm).powf(assign22630_e31763 - 1.0) * locals.var_dnm_dn2)) } } else { (assign22630_e31764 * (assign22630_e31763 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign22630_e31763) as f64).is_finite() && ((assign22630_e31763) as f64).fract() == 0.0 { if assign22630_e31763 == 0.0 { 0.0 } else { (assign22630_e31763 * ((locals.var_dnm).powf(assign22630_e31763 - 1.0) * locals.var_dnm_dn6)) } } else { (assign22630_e31764 * (assign22630_e31763 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign22630_e31763) as f64).is_finite() && ((assign22630_e31763) as f64).fract() == 0.0 { if assign22630_e31763 == 0.0 { 0.0 } else { (assign22630_e31763 * ((locals.var_dnm).powf(assign22630_e31763 - 1.0) * locals.var_dnm_dn7)) } } else { (assign22630_e31764 * (assign22630_e31763 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign22630_e31763) as f64).is_finite() && ((assign22630_e31763) as f64).fract() == 0.0 { if assign22630_e31763 == 0.0 { 0.0 } else { (assign22630_e31763 * ((locals.var_dnm).powf(assign22630_e31763 - 1.0) * locals.var_dnm_dn10)) } } else { (assign22630_e31764 * (assign22630_e31763 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign22630_e31763) as f64).is_finite() && ((assign22630_e31763) as f64).fract() == 0.0 { if assign22630_e31763 == 0.0 { 0.0 } else { (assign22630_e31763 * ((locals.var_dnm).powf(assign22630_e31763 - 1.0) * locals.var_dnm_dn11)) } } else { (assign22630_e31764 * (assign22630_e31763 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign22630_e31763) as f64).is_finite() && ((assign22630_e31763) as f64).fract() == 0.0 { if assign22630_e31763 == 0.0 { 0.0 } else { (assign22630_e31763 * ((locals.var_dnm).powf(assign22630_e31763 - 1.0) * locals.var_dnm_dn12)) } } else { (assign22630_e31764 * (assign22630_e31763 * (locals.var_dnm_dn12 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign22630_e31763) as f64).is_finite() && ((assign22630_e31763) as f64).fract() == 0.0 { if assign22630_e31763 == 0.0 { 0.0 } else { (assign22630_e31763 * ((locals.var_dnm).powf(assign22630_e31763 - 1.0) * locals.var_dnm_dn17)) } } else { (assign22630_e31764 * (assign22630_e31763 * (locals.var_dnm_dn17 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign22630_e31766;
        locals.var_dnm_dn0 = assign22630_e31766_d_n0;
        locals.var_dnm_dn2 = assign22630_e31766_d_n2;
        locals.var_dnm_dn6 = assign22630_e31766_d_n6;
        locals.var_dnm_dn7 = assign22630_e31766_d_n7;
        locals.var_dnm_dn10 = assign22630_e31766_d_n10;
        locals.var_dnm_dn11 = assign22630_e31766_d_n11;
        locals.var_dnm_dn12 = assign22630_e31766_d_n12;
        locals.var_dnm_dn17 = assign22630_e31766_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign22640_e31783, assign22640_e31783_d_n0, assign22640_e31783_d_n2, assign22640_e31783_d_n6, assign22640_e31783_d_n7, assign22640_e31783_d_n10, assign22640_e31783_d_n11, assign22640_e31783_d_n12, assign22640_e31783_d_n17,) = {
    if (((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign22640_e31781: f64 = (1.0 / locals.var_dnm);
        (assign22640_e31781, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn12 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn17 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign22640_e31783;
        locals.var_dnm_dn0 = assign22640_e31783_d_n0;
        locals.var_dnm_dn2 = assign22640_e31783_d_n2;
        locals.var_dnm_dn6 = assign22640_e31783_d_n6;
        locals.var_dnm_dn7 = assign22640_e31783_d_n7;
        locals.var_dnm_dn10 = assign22640_e31783_d_n10;
        locals.var_dnm_dn11 = assign22640_e31783_d_n11;
        locals.var_dnm_dn12 = assign22640_e31783_d_n12;
        locals.var_dnm_dn17 = assign22640_e31783_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign22650_e31802, assign22650_e31802_d_n0, assign22650_e31802_d_n2, assign22650_e31802_d_n6, assign22650_e31802_d_n7, assign22650_e31802_d_n10, assign22650_e31802_d_n11, assign22650_e31802_d_n12, assign22650_e31802_d_n17,) = {
    if (((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign22650_e31798: f64 = (locals.var_tmf1 * locals.var_t7__blk677);
        let assign22650_e31800: f64 = (assign22650_e31798 * locals.var_dnm);
        (assign22650_e31800, ((((locals.var_tmf1_dn0 * locals.var_t7__blk677) + (locals.var_tmf1 * locals.var_t7__blk677_dn0)) * locals.var_dnm) + (assign22650_e31798 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * locals.var_t7__blk677) + (locals.var_tmf1 * locals.var_t7__blk677_dn2)) * locals.var_dnm) + (assign22650_e31798 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn6 * locals.var_t7__blk677) + (locals.var_tmf1 * locals.var_t7__blk677_dn6)) * locals.var_dnm) + (assign22650_e31798 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * locals.var_t7__blk677) + (locals.var_tmf1 * locals.var_t7__blk677_dn7)) * locals.var_dnm) + (assign22650_e31798 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn10 * locals.var_t7__blk677) + (locals.var_tmf1 * locals.var_t7__blk677_dn10)) * locals.var_dnm) + (assign22650_e31798 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * locals.var_t7__blk677) + (locals.var_tmf1 * locals.var_t7__blk677_dn11)) * locals.var_dnm) + (assign22650_e31798 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn12 * locals.var_t7__blk677) + (locals.var_tmf1 * locals.var_t7__blk677_dn12)) * locals.var_dnm) + (assign22650_e31798 * locals.var_dnm_dn12)), ((((locals.var_tmf1_dn17 * locals.var_t7__blk677) + (locals.var_tmf1 * locals.var_t7__blk677_dn17)) * locals.var_dnm) + (assign22650_e31798 * locals.var_dnm_dn17)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn12, locals.var_tmf0_dn17,)
    }
};
        locals.var_tmf0 = assign22650_e31802;
        locals.var_tmf0_dn0 = assign22650_e31802_d_n0;
        locals.var_tmf0_dn2 = assign22650_e31802_d_n2;
        locals.var_tmf0_dn6 = assign22650_e31802_d_n6;
        locals.var_tmf0_dn7 = assign22650_e31802_d_n7;
        locals.var_tmf0_dn10 = assign22650_e31802_d_n10;
        locals.var_tmf0_dn11 = assign22650_e31802_d_n11;
        locals.var_tmf0_dn12 = assign22650_e31802_d_n12;
        locals.var_tmf0_dn17 = assign22650_e31802_d_n17;
        locals.var_tmf0_rv = 0.0;

        let (assign22660_e31821, assign22660_e31821_d_n0, assign22660_e31821_d_n2, assign22660_e31821_d_n6, assign22660_e31821_d_n7, assign22660_e31821_d_n10, assign22660_e31821_d_n11, assign22660_e31821_d_n12, assign22660_e31821_d_n17,) = {
    if (((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign22660_e31817: f64 = (1e-50 + locals.var_t7__blk677);
        let assign22660_e31819: f64 = (assign22660_e31817 - locals.var_tmf0);
        (assign22660_e31819, (locals.var_t7__blk677_dn0 - locals.var_tmf0_dn0), (locals.var_t7__blk677_dn2 - locals.var_tmf0_dn2), (locals.var_t7__blk677_dn6 - locals.var_tmf0_dn6), (locals.var_t7__blk677_dn7 - locals.var_tmf0_dn7), (locals.var_t7__blk677_dn10 - locals.var_tmf0_dn10), (locals.var_t7__blk677_dn11 - locals.var_tmf0_dn11), (locals.var_t7__blk677_dn12 - locals.var_tmf0_dn12), (locals.var_t7__blk677_dn17 - locals.var_tmf0_dn17),)
    } else {
        (locals.var_t6__blk675, locals.var_t6__blk675_dn0, locals.var_t6__blk675_dn2, locals.var_t6__blk675_dn6, locals.var_t6__blk675_dn7, locals.var_t6__blk675_dn10, locals.var_t6__blk675_dn11, locals.var_t6__blk675_dn12, locals.var_t6__blk675_dn17,)
    }
};
        locals.var_t6__blk675 = assign22660_e31821;
        locals.var_t6__blk675_dn0 = assign22660_e31821_d_n0;
        locals.var_t6__blk675_dn2 = assign22660_e31821_d_n2;
        locals.var_t6__blk675_dn6 = assign22660_e31821_d_n6;
        locals.var_t6__blk675_dn7 = assign22660_e31821_d_n7;
        locals.var_t6__blk675_dn10 = assign22660_e31821_d_n10;
        locals.var_t6__blk675_dn11 = assign22660_e31821_d_n11;
        locals.var_t6__blk675_dn12 = assign22660_e31821_d_n12;
        locals.var_t6__blk675_dn17 = assign22660_e31821_d_n17;
        locals.var_t6__blk675_rv = 0.0;

        let (assign22670_e31837, assign22670_e31837_d_n0, assign22670_e31837_d_n2, assign22670_e31837_d_n6, assign22670_e31837_d_n7, assign22670_e31837_d_n10, assign22670_e31837_d_n11, assign22670_e31837_d_n12, assign22670_e31837_d_n17,) = {
    if (((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard687 == 0.0)) {
        (locals.var_t6__blk675, locals.var_t6__blk675_dn0, locals.var_t6__blk675_dn2, locals.var_t6__blk675_dn6, locals.var_t6__blk675_dn7, locals.var_t6__blk675_dn10, locals.var_t6__blk675_dn11, locals.var_t6__blk675_dn12, locals.var_t6__blk675_dn17,)
    } else {
        (locals.var_t6__blk675, locals.var_t6__blk675_dn0, locals.var_t6__blk675_dn2, locals.var_t6__blk675_dn6, locals.var_t6__blk675_dn7, locals.var_t6__blk675_dn10, locals.var_t6__blk675_dn11, locals.var_t6__blk675_dn12, locals.var_t6__blk675_dn17,)
    }
};
        locals.var_t6__blk675 = assign22670_e31837;
        locals.var_t6__blk675_dn0 = assign22670_e31837_d_n0;
        locals.var_t6__blk675_dn2 = assign22670_e31837_d_n2;
        locals.var_t6__blk675_dn6 = assign22670_e31837_d_n6;
        locals.var_t6__blk675_dn7 = assign22670_e31837_d_n7;
        locals.var_t6__blk675_dn10 = assign22670_e31837_d_n10;
        locals.var_t6__blk675_dn11 = assign22670_e31837_d_n11;
        locals.var_t6__blk675_dn12 = assign22670_e31837_d_n12;
        locals.var_t6__blk675_dn17 = assign22670_e31837_d_n17;
        locals.var_t6__blk675_rv = 0.0;

        let (assign22680_e31856, assign22680_e31856_d_n0, assign22680_e31856_d_n2, assign22680_e31856_d_n6, assign22680_e31856_d_n7, assign22680_e31856_d_n10, assign22680_e31856_d_n11, assign22680_e31856_d_n12, assign22680_e31856_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) {
        let (assign22680_e31854, assign22680_e31854_d_n0, assign22680_e31854_d_n2, assign22680_e31854_d_n6, assign22680_e31854_d_n7, assign22680_e31854_d_n10, assign22680_e31854_d_n11, assign22680_e31854_d_n12, assign22680_e31854_d_n17,) = {
            if (locals.var_t6__blk675 <= 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign22680_e31853: f64 = (locals.var_t6__blk675).sqrt();
                (assign22680_e31853, (locals.var_t6__blk675_dn0 / (2.0 * assign22680_e31853)), (locals.var_t6__blk675_dn2 / (2.0 * assign22680_e31853)), (locals.var_t6__blk675_dn6 / (2.0 * assign22680_e31853)), (locals.var_t6__blk675_dn7 / (2.0 * assign22680_e31853)), (locals.var_t6__blk675_dn10 / (2.0 * assign22680_e31853)), (locals.var_t6__blk675_dn11 / (2.0 * assign22680_e31853)), (locals.var_t6__blk675_dn12 / (2.0 * assign22680_e31853)), (locals.var_t6__blk675_dn17 / (2.0 * assign22680_e31853)),)
            }
        };
        (assign22680_e31854, assign22680_e31854_d_n0, assign22680_e31854_d_n2, assign22680_e31854_d_n6, assign22680_e31854_d_n7, assign22680_e31854_d_n10, assign22680_e31854_d_n11, assign22680_e31854_d_n12, assign22680_e31854_d_n17,)
    } else {
        (locals.var_t6__blk675, locals.var_t6__blk675_dn0, locals.var_t6__blk675_dn2, locals.var_t6__blk675_dn6, locals.var_t6__blk675_dn7, locals.var_t6__blk675_dn10, locals.var_t6__blk675_dn11, locals.var_t6__blk675_dn12, locals.var_t6__blk675_dn17,)
    }
};
        locals.var_t6__blk675 = assign22680_e31856;
        locals.var_t6__blk675_dn0 = assign22680_e31856_d_n0;
        locals.var_t6__blk675_dn2 = assign22680_e31856_d_n2;
        locals.var_t6__blk675_dn6 = assign22680_e31856_d_n6;
        locals.var_t6__blk675_dn7 = assign22680_e31856_d_n7;
        locals.var_t6__blk675_dn10 = assign22680_e31856_d_n10;
        locals.var_t6__blk675_dn11 = assign22680_e31856_d_n11;
        locals.var_t6__blk675_dn12 = assign22680_e31856_d_n12;
        locals.var_t6__blk675_dn17 = assign22680_e31856_d_n17;
        locals.var_t6__blk675_rv = 0.0;

        let (assign22690_e31875, assign22690_e31875_d_n0, assign22690_e31875_d_n2, assign22690_e31875_d_n6, assign22690_e31875_d_n7, assign22690_e31875_d_n10, assign22690_e31875_d_n11, assign22690_e31875_d_n12, assign22690_e31875_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) {
        let assign22690_e31871: f64 = (1.0 - locals.var_t6__blk675);
        let assign22690_e31872: f64 = (locals.var_t3__blk672 * assign22690_e31871);
        let assign22690_e31873: f64 = (locals.var_t1__blk670 + assign22690_e31872);
        (assign22690_e31873, (locals.var_t1__blk670_dn0 + ((locals.var_t3__blk672_dn0 * assign22690_e31871) + (locals.var_t3__blk672 * (-locals.var_t6__blk675_dn0)))), (locals.var_t1__blk670_dn2 + ((locals.var_t3__blk672_dn2 * assign22690_e31871) + (locals.var_t3__blk672 * (-locals.var_t6__blk675_dn2)))), (locals.var_t1__blk670_dn6 + ((locals.var_t3__blk672_dn6 * assign22690_e31871) + (locals.var_t3__blk672 * (-locals.var_t6__blk675_dn6)))), (locals.var_t1__blk670_dn7 + ((locals.var_t3__blk672_dn7 * assign22690_e31871) + (locals.var_t3__blk672 * (-locals.var_t6__blk675_dn7)))), (locals.var_t1__blk670_dn10 + ((locals.var_t3__blk672_dn10 * assign22690_e31871) + (locals.var_t3__blk672 * (-locals.var_t6__blk675_dn10)))), (locals.var_t1__blk670_dn11 + ((locals.var_t3__blk672_dn11 * assign22690_e31871) + (locals.var_t3__blk672 * (-locals.var_t6__blk675_dn11)))), (locals.var_t1__blk670_dn12 + ((locals.var_t3__blk672_dn12 * assign22690_e31871) + (locals.var_t3__blk672 * (-locals.var_t6__blk675_dn12)))), (locals.var_t1__blk670_dn17 + ((locals.var_t3__blk672_dn17 * assign22690_e31871) + (locals.var_t3__blk672 * (-locals.var_t6__blk675_dn17)))),)
    } else {
        (locals.var_psislsat__blk680, locals.var_psislsat__blk680_dn0, locals.var_psislsat__blk680_dn2, locals.var_psislsat__blk680_dn6, locals.var_psislsat__blk680_dn7, locals.var_psislsat__blk680_dn10, locals.var_psislsat__blk680_dn11, locals.var_psislsat__blk680_dn12, locals.var_psislsat__blk680_dn17,)
    }
};
        locals.var_psislsat__blk680 = assign22690_e31875;
        locals.var_psislsat__blk680_dn0 = assign22690_e31875_d_n0;
        locals.var_psislsat__blk680_dn2 = assign22690_e31875_d_n2;
        locals.var_psislsat__blk680_dn6 = assign22690_e31875_d_n6;
        locals.var_psislsat__blk680_dn7 = assign22690_e31875_d_n7;
        locals.var_psislsat__blk680_dn10 = assign22690_e31875_d_n10;
        locals.var_psislsat__blk680_dn11 = assign22690_e31875_d_n11;
        locals.var_psislsat__blk680_dn12 = assign22690_e31875_d_n12;
        locals.var_psislsat__blk680_dn17 = assign22690_e31875_d_n17;
        locals.var_psislsat__blk680_rv = 0.0;

        let (assign22700_e31892, assign22700_e31892_d_n0, assign22700_e31892_d_n2, assign22700_e31892_d_n6, assign22700_e31892_d_n7, assign22700_e31892_d_n10, assign22700_e31892_d_n11, assign22700_e31892_d_n12, assign22700_e31892_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) {
        let assign22700_e31889: f64 = (locals.var_xgate + locals.var_lgle);
        let assign22700_e31890: f64 = (locals.var_lgle / assign22700_e31889);
        (assign22700_e31890, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk671, locals.var_t2__blk671_dn0, locals.var_t2__blk671_dn2, locals.var_t2__blk671_dn6, locals.var_t2__blk671_dn7, locals.var_t2__blk671_dn10, locals.var_t2__blk671_dn11, locals.var_t2__blk671_dn12, locals.var_t2__blk671_dn17,)
    }
};
        locals.var_t2__blk671 = assign22700_e31892;
        locals.var_t2__blk671_dn0 = assign22700_e31892_d_n0;
        locals.var_t2__blk671_dn2 = assign22700_e31892_d_n2;
        locals.var_t2__blk671_dn6 = assign22700_e31892_d_n6;
        locals.var_t2__blk671_dn7 = assign22700_e31892_d_n7;
        locals.var_t2__blk671_dn10 = assign22700_e31892_d_n10;
        locals.var_t2__blk671_dn11 = assign22700_e31892_d_n11;
        locals.var_t2__blk671_dn12 = assign22700_e31892_d_n12;
        locals.var_t2__blk671_dn17 = assign22700_e31892_d_n17;
        locals.var_t2__blk671_rv = 0.0;

        let (assign22710_e31913, assign22710_e31913_d_n0, assign22710_e31913_d_n2, assign22710_e31913_d_n6, assign22710_e31913_d_n7, assign22710_e31913_d_n10, assign22710_e31913_d_n11, assign22710_e31913_d_n12, assign22710_e31913_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) {
        let assign22710_e31905: f64 = (p.p122 * locals.var_vdsz);
        let assign22710_e31907: f64 = (assign22710_e31905 + locals.var_ps0z);
        let assign22710_e31910: f64 = (locals.var_t2__blk671 * locals.var_psislsat__blk680);
        let assign22710_e31911: f64 = (assign22710_e31907 - assign22710_e31910);
        (assign22710_e31911, (((p.p122 * locals.var_vdsz_dn0) + locals.var_ps0z_dn0) - ((locals.var_t2__blk671_dn0 * locals.var_psislsat__blk680) + (locals.var_t2__blk671 * locals.var_psislsat__blk680_dn0))), (((p.p122 * locals.var_vdsz_dn2) + locals.var_ps0z_dn2) - ((locals.var_t2__blk671_dn2 * locals.var_psislsat__blk680) + (locals.var_t2__blk671 * locals.var_psislsat__blk680_dn2))), (((p.p122 * locals.var_vdsz_dn6) + locals.var_ps0z_dn6) - ((locals.var_t2__blk671_dn6 * locals.var_psislsat__blk680) + (locals.var_t2__blk671 * locals.var_psislsat__blk680_dn6))), (((p.p122 * locals.var_vdsz_dn7) + locals.var_ps0z_dn7) - ((locals.var_t2__blk671_dn7 * locals.var_psislsat__blk680) + (locals.var_t2__blk671 * locals.var_psislsat__blk680_dn7))), (((p.p122 * locals.var_vdsz_dn10) + locals.var_ps0z_dn10) - ((locals.var_t2__blk671_dn10 * locals.var_psislsat__blk680) + (locals.var_t2__blk671 * locals.var_psislsat__blk680_dn10))), (((p.p122 * locals.var_vdsz_dn11) + locals.var_ps0z_dn11) - ((locals.var_t2__blk671_dn11 * locals.var_psislsat__blk680) + (locals.var_t2__blk671 * locals.var_psislsat__blk680_dn11))), (((p.p122 * locals.var_vdsz_dn12) + locals.var_ps0z_dn12) - ((locals.var_t2__blk671_dn12 * locals.var_psislsat__blk680) + (locals.var_t2__blk671 * locals.var_psislsat__blk680_dn12))), (((p.p122 * locals.var_vdsz_dn17) + locals.var_ps0z_dn17) - ((locals.var_t2__blk671_dn17 * locals.var_psislsat__blk680) + (locals.var_t2__blk671 * locals.var_psislsat__blk680_dn17))),)
    } else {
        (locals.var_psisubsat__blk681, locals.var_psisubsat__blk681_dn0, locals.var_psisubsat__blk681_dn2, locals.var_psisubsat__blk681_dn6, locals.var_psisubsat__blk681_dn7, locals.var_psisubsat__blk681_dn10, locals.var_psisubsat__blk681_dn11, locals.var_psisubsat__blk681_dn12, locals.var_psisubsat__blk681_dn17,)
    }
};
        locals.var_psisubsat__blk681 = assign22710_e31913;
        locals.var_psisubsat__blk681_dn0 = assign22710_e31913_d_n0;
        locals.var_psisubsat__blk681_dn2 = assign22710_e31913_d_n2;
        locals.var_psisubsat__blk681_dn6 = assign22710_e31913_d_n6;
        locals.var_psisubsat__blk681_dn7 = assign22710_e31913_d_n7;
        locals.var_psisubsat__blk681_dn10 = assign22710_e31913_d_n10;
        locals.var_psisubsat__blk681_dn11 = assign22710_e31913_d_n11;
        locals.var_psisubsat__blk681_dn12 = assign22710_e31913_d_n12;
        locals.var_psisubsat__blk681_dn17 = assign22710_e31913_d_n17;
        locals.var_psisubsat__blk681_rv = 0.0;

        let (assign22720_e31935, assign22720_e31935_d_n0, assign22720_e31935_d_n2, assign22720_e31935_d_n6, assign22720_e31935_d_n7, assign22720_e31935_d_n10, assign22720_e31935_d_n11, assign22720_e31935_d_n12, assign22720_e31935_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) {
        let assign22720_e31926: f64 = (locals.var_psisubsat__blk681 * locals.var_psisubsat__blk681);
        let assign22720_e31929: f64 = (4.0 * 0.001);
        let assign22720_e31931: f64 = (assign22720_e31929 * 0.001);
        let assign22720_e31932: f64 = (assign22720_e31926 + assign22720_e31931);
        let assign22720_e31933: f64 = (assign22720_e31932).sqrt();
        (assign22720_e31933, (((locals.var_psisubsat__blk681_dn0 * locals.var_psisubsat__blk681) + (locals.var_psisubsat__blk681 * locals.var_psisubsat__blk681_dn0)) / (2.0 * assign22720_e31933)), (((locals.var_psisubsat__blk681_dn2 * locals.var_psisubsat__blk681) + (locals.var_psisubsat__blk681 * locals.var_psisubsat__blk681_dn2)) / (2.0 * assign22720_e31933)), (((locals.var_psisubsat__blk681_dn6 * locals.var_psisubsat__blk681) + (locals.var_psisubsat__blk681 * locals.var_psisubsat__blk681_dn6)) / (2.0 * assign22720_e31933)), (((locals.var_psisubsat__blk681_dn7 * locals.var_psisubsat__blk681) + (locals.var_psisubsat__blk681 * locals.var_psisubsat__blk681_dn7)) / (2.0 * assign22720_e31933)), (((locals.var_psisubsat__blk681_dn10 * locals.var_psisubsat__blk681) + (locals.var_psisubsat__blk681 * locals.var_psisubsat__blk681_dn10)) / (2.0 * assign22720_e31933)), (((locals.var_psisubsat__blk681_dn11 * locals.var_psisubsat__blk681) + (locals.var_psisubsat__blk681 * locals.var_psisubsat__blk681_dn11)) / (2.0 * assign22720_e31933)), (((locals.var_psisubsat__blk681_dn12 * locals.var_psisubsat__blk681) + (locals.var_psisubsat__blk681 * locals.var_psisubsat__blk681_dn12)) / (2.0 * assign22720_e31933)), (((locals.var_psisubsat__blk681_dn17 * locals.var_psisubsat__blk681) + (locals.var_psisubsat__blk681 * locals.var_psisubsat__blk681_dn17)) / (2.0 * assign22720_e31933)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign22720_e31935;
        locals.var_tmf1_dn0 = assign22720_e31935_d_n0;
        locals.var_tmf1_dn2 = assign22720_e31935_d_n2;
        locals.var_tmf1_dn6 = assign22720_e31935_d_n6;
        locals.var_tmf1_dn7 = assign22720_e31935_d_n7;
        locals.var_tmf1_dn10 = assign22720_e31935_d_n10;
        locals.var_tmf1_dn11 = assign22720_e31935_d_n11;
        locals.var_tmf1_dn12 = assign22720_e31935_d_n12;
        locals.var_tmf1_dn17 = assign22720_e31935_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign22730_e31956, assign22730_e31956_d_n0, assign22730_e31956_d_n2, assign22730_e31956_d_n6, assign22730_e31956_d_n7, assign22730_e31956_d_n10, assign22730_e31956_d_n11, assign22730_e31956_d_n12, assign22730_e31956_d_n17,) = {
    if ((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) {
        let assign22730_e31949: f64 = (locals.var_psisubsat__blk681 + locals.var_tmf1);
        let assign22730_e31950: f64 = (0.5 * assign22730_e31949);
        let assign22730_e31953: f64 = (1e-10 * 0.001);
        let assign22730_e31954: f64 = (assign22730_e31950 + assign22730_e31953);
        (assign22730_e31954, (0.5 * (locals.var_psisubsat__blk681_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_psisubsat__blk681_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_psisubsat__blk681_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_psisubsat__blk681_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_psisubsat__blk681_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_psisubsat__blk681_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_psisubsat__blk681_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_psisubsat__blk681_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_psisubsat__blk681, locals.var_psisubsat__blk681_dn0, locals.var_psisubsat__blk681_dn2, locals.var_psisubsat__blk681_dn6, locals.var_psisubsat__blk681_dn7, locals.var_psisubsat__blk681_dn10, locals.var_psisubsat__blk681_dn11, locals.var_psisubsat__blk681_dn12, locals.var_psisubsat__blk681_dn17,)
    }
};
        locals.var_psisubsat__blk681 = assign22730_e31956;
        locals.var_psisubsat__blk681_dn0 = assign22730_e31956_d_n0;
        locals.var_psisubsat__blk681_dn2 = assign22730_e31956_d_n2;
        locals.var_psisubsat__blk681_dn6 = assign22730_e31956_d_n6;
        locals.var_psisubsat__blk681_dn7 = assign22730_e31956_d_n7;
        locals.var_psisubsat__blk681_dn10 = assign22730_e31956_d_n10;
        locals.var_psisubsat__blk681_dn11 = assign22730_e31956_d_n11;
        locals.var_psisubsat__blk681_dn12 = assign22730_e31956_d_n12;
        locals.var_psisubsat__blk681_dn17 = assign22730_e31956_d_n17;
        locals.var_psisubsat__blk681_rv = 0.0;

        let assign22740_e31959: f64 = if locals.var_psisubsat__blk681 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard693 = assign22740_e31959;
        locals.var_guard693_rv = 0.0;

        let (assign22750_e31974, assign22750_e31974_d_n0, assign22750_e31974_d_n2, assign22750_e31974_d_n6, assign22750_e31974_d_n7, assign22750_e31974_d_n10, assign22750_e31974_d_n11, assign22750_e31974_d_n12, assign22750_e31974_d_n17,) = {
    if (((((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) && (locals.var_guard684 == 0.0)) && (locals.var_guard693 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_psisubsat__blk681, locals.var_psisubsat__blk681_dn0, locals.var_psisubsat__blk681_dn2, locals.var_psisubsat__blk681_dn6, locals.var_psisubsat__blk681_dn7, locals.var_psisubsat__blk681_dn10, locals.var_psisubsat__blk681_dn11, locals.var_psisubsat__blk681_dn12, locals.var_psisubsat__blk681_dn17,)
    }
};
        locals.var_psisubsat__blk681 = assign22750_e31974;
        locals.var_psisubsat__blk681_dn0 = assign22750_e31974_d_n0;
        locals.var_psisubsat__blk681_dn2 = assign22750_e31974_d_n2;
        locals.var_psisubsat__blk681_dn6 = assign22750_e31974_d_n6;
        locals.var_psisubsat__blk681_dn7 = assign22750_e31974_d_n7;
        locals.var_psisubsat__blk681_dn10 = assign22750_e31974_d_n10;
        locals.var_psisubsat__blk681_dn11 = assign22750_e31974_d_n11;
        locals.var_psisubsat__blk681_dn12 = assign22750_e31974_d_n12;
        locals.var_psisubsat__blk681_dn17 = assign22750_e31974_d_n17;
        locals.var_psisubsat__blk681_rv = 0.0;

        let (assign22760_e31986, assign22760_e31986_d_n0, assign22760_e31986_d_n2, assign22760_e31986_d_n6, assign22760_e31986_d_n7, assign22760_e31986_d_n10, assign22760_e31986_d_n11, assign22760_e31986_d_n12, assign22760_e31986_d_n17,) = {
    if (((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) {
        let assign22760_e31984: f64 = (locals.var_psisubsat__blk681 + 1e-50);
        (assign22760_e31984, locals.var_psisubsat__blk681_dn0, locals.var_psisubsat__blk681_dn2, locals.var_psisubsat__blk681_dn6, locals.var_psisubsat__blk681_dn7, locals.var_psisubsat__blk681_dn10, locals.var_psisubsat__blk681_dn11, locals.var_psisubsat__blk681_dn12, locals.var_psisubsat__blk681_dn17,)
    } else {
        (locals.var_psisubsat__blk681, locals.var_psisubsat__blk681_dn0, locals.var_psisubsat__blk681_dn2, locals.var_psisubsat__blk681_dn6, locals.var_psisubsat__blk681_dn7, locals.var_psisubsat__blk681_dn10, locals.var_psisubsat__blk681_dn11, locals.var_psisubsat__blk681_dn12, locals.var_psisubsat__blk681_dn17,)
    }
};
        locals.var_psisubsat__blk681 = assign22760_e31986;
        locals.var_psisubsat__blk681_dn0 = assign22760_e31986_d_n0;
        locals.var_psisubsat__blk681_dn2 = assign22760_e31986_d_n2;
        locals.var_psisubsat__blk681_dn6 = assign22760_e31986_d_n6;
        locals.var_psisubsat__blk681_dn7 = assign22760_e31986_d_n7;
        locals.var_psisubsat__blk681_dn10 = assign22760_e31986_d_n10;
        locals.var_psisubsat__blk681_dn11 = assign22760_e31986_d_n11;
        locals.var_psisubsat__blk681_dn12 = assign22760_e31986_d_n12;
        locals.var_psisubsat__blk681_dn17 = assign22760_e31986_d_n17;
        locals.var_psisubsat__blk681_rv = 0.0;

        let (assign22770_e32000, assign22770_e32000_d_n0, assign22770_e32000_d_n2, assign22770_e32000_d_n6, assign22770_e32000_d_n7, assign22770_e32000_d_n10, assign22770_e32000_d_n11, assign22770_e32000_d_n12, assign22770_e32000_d_n17,) = {
    if (((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) {
        let assign22770_e31995: f64 = (-locals.var_xsub2);
        let assign22770_e31997: f64 = (assign22770_e31995 / locals.var_psisubsat__blk681);
        let assign22770_e31998: f64 = (assign22770_e31997).exp();
        (assign22770_e31998, (assign22770_e31998 * (-((assign22770_e31995 * locals.var_psisubsat__blk681_dn0) / (locals.var_psisubsat__blk681 * locals.var_psisubsat__blk681)))), (assign22770_e31998 * (-((assign22770_e31995 * locals.var_psisubsat__blk681_dn2) / (locals.var_psisubsat__blk681 * locals.var_psisubsat__blk681)))), (assign22770_e31998 * (-((assign22770_e31995 * locals.var_psisubsat__blk681_dn6) / (locals.var_psisubsat__blk681 * locals.var_psisubsat__blk681)))), (assign22770_e31998 * (-((assign22770_e31995 * locals.var_psisubsat__blk681_dn7) / (locals.var_psisubsat__blk681 * locals.var_psisubsat__blk681)))), (assign22770_e31998 * (-((assign22770_e31995 * locals.var_psisubsat__blk681_dn10) / (locals.var_psisubsat__blk681 * locals.var_psisubsat__blk681)))), (assign22770_e31998 * (-((assign22770_e31995 * locals.var_psisubsat__blk681_dn11) / (locals.var_psisubsat__blk681 * locals.var_psisubsat__blk681)))), (assign22770_e31998 * (-((assign22770_e31995 * locals.var_psisubsat__blk681_dn12) / (locals.var_psisubsat__blk681 * locals.var_psisubsat__blk681)))), (assign22770_e31998 * (-((assign22770_e31995 * locals.var_psisubsat__blk681_dn17) / (locals.var_psisubsat__blk681 * locals.var_psisubsat__blk681)))),)
    } else {
        (locals.var_t2__blk671, locals.var_t2__blk671_dn0, locals.var_t2__blk671_dn2, locals.var_t2__blk671_dn6, locals.var_t2__blk671_dn7, locals.var_t2__blk671_dn10, locals.var_t2__blk671_dn11, locals.var_t2__blk671_dn12, locals.var_t2__blk671_dn17,)
    }
};
        locals.var_t2__blk671 = assign22770_e32000;
        locals.var_t2__blk671_dn0 = assign22770_e32000_d_n0;
        locals.var_t2__blk671_dn2 = assign22770_e32000_d_n2;
        locals.var_t2__blk671_dn6 = assign22770_e32000_d_n6;
        locals.var_t2__blk671_dn7 = assign22770_e32000_d_n7;
        locals.var_t2__blk671_dn10 = assign22770_e32000_d_n10;
        locals.var_t2__blk671_dn11 = assign22770_e32000_d_n11;
        locals.var_t2__blk671_dn12 = assign22770_e32000_d_n12;
        locals.var_t2__blk671_dn17 = assign22770_e32000_d_n17;
        locals.var_t2__blk671_rv = 0.0;

        let (assign22780_e32016, assign22780_e32016_d_n0, assign22780_e32016_d_n2, assign22780_e32016_d_n6, assign22780_e32016_d_n7, assign22780_e32016_d_n10, assign22780_e32016_d_n11, assign22780_e32016_d_n12, assign22780_e32016_d_n17,) = {
    if (((locals.var_guard669 != 0.0) && (locals.var_guard682 == 0.0)) && (locals.var_guard683 == 0.0)) {
        let assign22780_e32010: f64 = (locals.var_xsub1 * locals.var_psisubsat__blk681);
        let assign22780_e32012: f64 = (assign22780_e32010 * locals.var_ids);
        let assign22780_e32014: f64 = (assign22780_e32012 * locals.var_t2__blk671);
        (assign22780_e32014, (((((locals.var_xsub1 * locals.var_psisubsat__blk681_dn0) * locals.var_ids) + (assign22780_e32010 * locals.var_ids_dn0)) * locals.var_t2__blk671) + (assign22780_e32012 * locals.var_t2__blk671_dn0)), (((((locals.var_xsub1 * locals.var_psisubsat__blk681_dn2) * locals.var_ids) + (assign22780_e32010 * locals.var_ids_dn2)) * locals.var_t2__blk671) + (assign22780_e32012 * locals.var_t2__blk671_dn2)), (((((locals.var_xsub1 * locals.var_psisubsat__blk681_dn6) * locals.var_ids) + (assign22780_e32010 * locals.var_ids_dn6)) * locals.var_t2__blk671) + (assign22780_e32012 * locals.var_t2__blk671_dn6)), (((((locals.var_xsub1 * locals.var_psisubsat__blk681_dn7) * locals.var_ids) + (assign22780_e32010 * locals.var_ids_dn7)) * locals.var_t2__blk671) + (assign22780_e32012 * locals.var_t2__blk671_dn7)), (((((locals.var_xsub1 * locals.var_psisubsat__blk681_dn10) * locals.var_ids) + (assign22780_e32010 * locals.var_ids_dn10)) * locals.var_t2__blk671) + (assign22780_e32012 * locals.var_t2__blk671_dn10)), (((((locals.var_xsub1 * locals.var_psisubsat__blk681_dn11) * locals.var_ids) + (assign22780_e32010 * locals.var_ids_dn11)) * locals.var_t2__blk671) + (assign22780_e32012 * locals.var_t2__blk671_dn11)), (((((locals.var_xsub1 * locals.var_psisubsat__blk681_dn12) * locals.var_ids) + (assign22780_e32010 * locals.var_ids_dn12)) * locals.var_t2__blk671) + (assign22780_e32012 * locals.var_t2__blk671_dn12)), (((((locals.var_xsub1 * locals.var_psisubsat__blk681_dn17) * locals.var_ids) + (assign22780_e32010 * locals.var_ids_dn17)) * locals.var_t2__blk671) + (assign22780_e32012 * locals.var_t2__blk671_dn17)),)
    } else {
        (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn6, locals.var_isub_dn7, locals.var_isub_dn10, locals.var_isub_dn11, locals.var_isub_dn12, locals.var_isub_dn17,)
    }
};
        locals.var_isub = assign22780_e32016;
        locals.var_isub_dn0 = assign22780_e32016_d_n0;
        locals.var_isub_dn2 = assign22780_e32016_d_n2;
        locals.var_isub_dn6 = assign22780_e32016_d_n6;
        locals.var_isub_dn7 = assign22780_e32016_d_n7;
        locals.var_isub_dn10 = assign22780_e32016_d_n10;
        locals.var_isub_dn11 = assign22780_e32016_d_n11;
        locals.var_isub_dn12 = assign22780_e32016_d_n12;
        locals.var_isub_dn17 = assign22780_e32016_d_n17;
        locals.var_isub_rv = 0.0;

        let assign22790_e32027: f64 = if (((p.p25 == 1.0) && (p.p26 == 2.0)) && (p.p43 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard694 = assign22790_e32027;
        locals.var_guard694_rv = 0.0;

        let (assign22840_e32101, assign22840_e32101_d_n0, assign22840_e32101_d_n2, assign22840_e32101_d_n6, assign22840_e32101_d_n7, assign22840_e32101_d_n10, assign22840_e32101_d_n11, assign22840_e32101_d_n12, assign22840_e32101_d_n17,) = {
    if (locals.var_guard694 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5__blk698, locals.var_t5__blk698_dn0, locals.var_t5__blk698_dn2, locals.var_t5__blk698_dn6, locals.var_t5__blk698_dn7, locals.var_t5__blk698_dn10, locals.var_t5__blk698_dn11, locals.var_t5__blk698_dn12, locals.var_t5__blk698_dn17,)
    }
};
        locals.var_t5__blk698 = assign22840_e32101;
        locals.var_t5__blk698_dn0 = assign22840_e32101_d_n0;
        locals.var_t5__blk698_dn2 = assign22840_e32101_d_n2;
        locals.var_t5__blk698_dn6 = assign22840_e32101_d_n6;
        locals.var_t5__blk698_dn7 = assign22840_e32101_d_n7;
        locals.var_t5__blk698_dn10 = assign22840_e32101_d_n10;
        locals.var_t5__blk698_dn11 = assign22840_e32101_d_n11;
        locals.var_t5__blk698_dn12 = assign22840_e32101_d_n12;
        locals.var_t5__blk698_dn17 = assign22840_e32101_d_n17;
        locals.var_t5__blk698_rv = 0.0;

        let (assign22850_e32111, assign22850_e32111_d_n0, assign22850_e32111_d_n2, assign22850_e32111_d_n6, assign22850_e32111_d_n7, assign22850_e32111_d_n10, assign22850_e32111_d_n11, assign22850_e32111_d_n12, assign22850_e32111_d_n17,) = {
    if (locals.var_guard694 != 0.0) {
        let assign22850_e32105: f64 = (locals.var_pb2 - locals.var_t5__blk698);
        let assign22850_e32108: f64 = (locals.var_pb2 * 0.01);
        let assign22850_e32109: f64 = (assign22850_e32105 - assign22850_e32108);
        (assign22850_e32109, ((locals.var_pb2_dn0 - locals.var_t5__blk698_dn0) - (locals.var_pb2_dn0 * 0.01)), ((locals.var_pb2_dn2 - locals.var_t5__blk698_dn2) - (locals.var_pb2_dn2 * 0.01)), ((locals.var_pb2_dn6 - locals.var_t5__blk698_dn6) - (locals.var_pb2_dn6 * 0.01)), ((locals.var_pb2_dn7 - locals.var_t5__blk698_dn7) - (locals.var_pb2_dn7 * 0.01)), ((locals.var_pb2_dn10 - locals.var_t5__blk698_dn10) - (locals.var_pb2_dn10 * 0.01)), ((locals.var_pb2_dn11 - locals.var_t5__blk698_dn11) - (locals.var_pb2_dn11 * 0.01)), ((locals.var_pb2_dn12 - locals.var_t5__blk698_dn12) - (locals.var_pb2_dn12 * 0.01)), ((locals.var_pb2_dn17 - locals.var_t5__blk698_dn17) - (locals.var_pb2_dn17 * 0.01)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign22850_e32111;
        locals.var_tmf1_dn0 = assign22850_e32111_d_n0;
        locals.var_tmf1_dn2 = assign22850_e32111_d_n2;
        locals.var_tmf1_dn6 = assign22850_e32111_d_n6;
        locals.var_tmf1_dn7 = assign22850_e32111_d_n7;
        locals.var_tmf1_dn10 = assign22850_e32111_d_n10;
        locals.var_tmf1_dn11 = assign22850_e32111_d_n11;
        locals.var_tmf1_dn12 = assign22850_e32111_d_n12;
        locals.var_tmf1_dn17 = assign22850_e32111_d_n17;
        locals.var_tmf1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_82(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv17 = ctx.node_voltage(nodes[17]);
        let (assign22860_e32121, assign22860_e32121_d_n0, assign22860_e32121_d_n2, assign22860_e32121_d_n6, assign22860_e32121_d_n7, assign22860_e32121_d_n10, assign22860_e32121_d_n11, assign22860_e32121_d_n12, assign22860_e32121_d_n17,) = {
    if (locals.var_guard694 != 0.0) {
        let assign22860_e32115: f64 = (4.0 * locals.var_pb2);
        let assign22860_e32118: f64 = (locals.var_pb2 * 0.01);
        let assign22860_e32119: f64 = (assign22860_e32115 * assign22860_e32118);
        (assign22860_e32119, (((4.0 * locals.var_pb2_dn0) * assign22860_e32118) + (assign22860_e32115 * (locals.var_pb2_dn0 * 0.01))), (((4.0 * locals.var_pb2_dn2) * assign22860_e32118) + (assign22860_e32115 * (locals.var_pb2_dn2 * 0.01))), (((4.0 * locals.var_pb2_dn6) * assign22860_e32118) + (assign22860_e32115 * (locals.var_pb2_dn6 * 0.01))), (((4.0 * locals.var_pb2_dn7) * assign22860_e32118) + (assign22860_e32115 * (locals.var_pb2_dn7 * 0.01))), (((4.0 * locals.var_pb2_dn10) * assign22860_e32118) + (assign22860_e32115 * (locals.var_pb2_dn10 * 0.01))), (((4.0 * locals.var_pb2_dn11) * assign22860_e32118) + (assign22860_e32115 * (locals.var_pb2_dn11 * 0.01))), (((4.0 * locals.var_pb2_dn12) * assign22860_e32118) + (assign22860_e32115 * (locals.var_pb2_dn12 * 0.01))), (((4.0 * locals.var_pb2_dn17) * assign22860_e32118) + (assign22860_e32115 * (locals.var_pb2_dn17 * 0.01))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign22860_e32121;
        locals.var_tmf2_dn0 = assign22860_e32121_d_n0;
        locals.var_tmf2_dn2 = assign22860_e32121_d_n2;
        locals.var_tmf2_dn6 = assign22860_e32121_d_n6;
        locals.var_tmf2_dn7 = assign22860_e32121_d_n7;
        locals.var_tmf2_dn10 = assign22860_e32121_d_n10;
        locals.var_tmf2_dn11 = assign22860_e32121_d_n11;
        locals.var_tmf2_dn12 = assign22860_e32121_d_n12;
        locals.var_tmf2_dn17 = assign22860_e32121_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign22870_e32131, assign22870_e32131_d_n0, assign22870_e32131_d_n2, assign22870_e32131_d_n6, assign22870_e32131_d_n7, assign22870_e32131_d_n10, assign22870_e32131_d_n11, assign22870_e32131_d_n12, assign22870_e32131_d_n17,) = {
    if (locals.var_guard694 != 0.0) {
        let (assign22870_e32129, assign22870_e32129_d_n0, assign22870_e32129_d_n2, assign22870_e32129_d_n6, assign22870_e32129_d_n7, assign22870_e32129_d_n10, assign22870_e32129_d_n11, assign22870_e32129_d_n12, assign22870_e32129_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign22870_e32128: f64 = (-locals.var_tmf2);
                (assign22870_e32128, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign22870_e32129, assign22870_e32129_d_n0, assign22870_e32129_d_n2, assign22870_e32129_d_n6, assign22870_e32129_d_n7, assign22870_e32129_d_n10, assign22870_e32129_d_n11, assign22870_e32129_d_n12, assign22870_e32129_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign22870_e32131;
        locals.var_tmf2_dn0 = assign22870_e32131_d_n0;
        locals.var_tmf2_dn2 = assign22870_e32131_d_n2;
        locals.var_tmf2_dn6 = assign22870_e32131_d_n6;
        locals.var_tmf2_dn7 = assign22870_e32131_d_n7;
        locals.var_tmf2_dn10 = assign22870_e32131_d_n10;
        locals.var_tmf2_dn11 = assign22870_e32131_d_n11;
        locals.var_tmf2_dn12 = assign22870_e32131_d_n12;
        locals.var_tmf2_dn17 = assign22870_e32131_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign22880_e32140, assign22880_e32140_d_n0, assign22880_e32140_d_n2, assign22880_e32140_d_n6, assign22880_e32140_d_n7, assign22880_e32140_d_n10, assign22880_e32140_d_n11, assign22880_e32140_d_n12, assign22880_e32140_d_n17,) = {
    if (locals.var_guard694 != 0.0) {
        let assign22880_e32135: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign22880_e32137: f64 = (assign22880_e32135 + locals.var_tmf2);
        let assign22880_e32138: f64 = (assign22880_e32137).sqrt();
        (assign22880_e32138, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign22880_e32138)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign22880_e32138)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign22880_e32138)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign22880_e32138)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign22880_e32138)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign22880_e32138)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign22880_e32138)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign22880_e32138)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign22880_e32140;
        locals.var_tmf2_dn0 = assign22880_e32140_d_n0;
        locals.var_tmf2_dn2 = assign22880_e32140_d_n2;
        locals.var_tmf2_dn6 = assign22880_e32140_d_n6;
        locals.var_tmf2_dn7 = assign22880_e32140_d_n7;
        locals.var_tmf2_dn10 = assign22880_e32140_d_n10;
        locals.var_tmf2_dn11 = assign22880_e32140_d_n11;
        locals.var_tmf2_dn12 = assign22880_e32140_d_n12;
        locals.var_tmf2_dn17 = assign22880_e32140_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign22890_e32150, assign22890_e32150_d_n0, assign22890_e32150_d_n2, assign22890_e32150_d_n6, assign22890_e32150_d_n7, assign22890_e32150_d_n10, assign22890_e32150_d_n11, assign22890_e32150_d_n12, assign22890_e32150_d_n17,) = {
    if (locals.var_guard694 != 0.0) {
        let assign22890_e32146: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign22890_e32147: f64 = (0.5 * assign22890_e32146);
        let assign22890_e32148: f64 = (locals.var_pb2 - assign22890_e32147);
        (assign22890_e32148, (locals.var_pb2_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_pb2_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_pb2_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_pb2_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_pb2_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_pb2_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_pb2_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_pb2_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_t5__blk698, locals.var_t5__blk698_dn0, locals.var_t5__blk698_dn2, locals.var_t5__blk698_dn6, locals.var_t5__blk698_dn7, locals.var_t5__blk698_dn10, locals.var_t5__blk698_dn11, locals.var_t5__blk698_dn12, locals.var_t5__blk698_dn17,)
    }
};
        locals.var_t5__blk698 = assign22890_e32150;
        locals.var_t5__blk698_dn0 = assign22890_e32150_d_n0;
        locals.var_t5__blk698_dn2 = assign22890_e32150_d_n2;
        locals.var_t5__blk698_dn6 = assign22890_e32150_d_n6;
        locals.var_t5__blk698_dn7 = assign22890_e32150_d_n7;
        locals.var_t5__blk698_dn10 = assign22890_e32150_d_n10;
        locals.var_t5__blk698_dn11 = assign22890_e32150_d_n11;
        locals.var_t5__blk698_dn12 = assign22890_e32150_d_n12;
        locals.var_t5__blk698_dn17 = assign22890_e32150_d_n17;
        locals.var_t5__blk698_rv = 0.0;

        let (assign22910_e32167, assign22910_e32167_d_n0, assign22910_e32167_d_n2, assign22910_e32167_d_n6, assign22910_e32167_d_n7, assign22910_e32167_d_n10, assign22910_e32167_d_n11, assign22910_e32167_d_n12, assign22910_e32167_d_n17,) = {
    if (locals.var_guard694 != 0.0) {
        let assign22910_e32158: f64 = (2.0 * 1.034943e-10);
        let assign22910_e32160: f64 = (assign22910_e32158 * 1.6021918e-19);
        let assign22910_e32162: f64 = (assign22910_e32160 * locals.var_uc_nsubs);
        let assign22910_e32164: f64 = (assign22910_e32162 * locals.var_beta_inv);
        let assign22910_e32165: f64 = (assign22910_e32164).sqrt();
        (assign22910_e32165, (((assign22910_e32160 * locals.var_uc_nsubs_dn0) * locals.var_beta_inv) / (2.0 * assign22910_e32165)), (((assign22910_e32160 * locals.var_uc_nsubs_dn2) * locals.var_beta_inv) / (2.0 * assign22910_e32165)), (((assign22910_e32160 * locals.var_uc_nsubs_dn6) * locals.var_beta_inv) / (2.0 * assign22910_e32165)), (((assign22910_e32160 * locals.var_uc_nsubs_dn7) * locals.var_beta_inv) / (2.0 * assign22910_e32165)), ((((assign22910_e32160 * locals.var_uc_nsubs_dn10) * locals.var_beta_inv) + (assign22910_e32162 * locals.var_beta_inv_dn10)) / (2.0 * assign22910_e32165)), (((assign22910_e32160 * locals.var_uc_nsubs_dn11) * locals.var_beta_inv) / (2.0 * assign22910_e32165)), (((assign22910_e32160 * locals.var_uc_nsubs_dn12) * locals.var_beta_inv) / (2.0 * assign22910_e32165)), (((assign22910_e32160 * locals.var_uc_nsubs_dn17) * locals.var_beta_inv) / (2.0 * assign22910_e32165)),)
    } else {
        (locals.var_t6__blk699, locals.var_t6__blk699_dn0, locals.var_t6__blk699_dn2, locals.var_t6__blk699_dn6, locals.var_t6__blk699_dn7, locals.var_t6__blk699_dn10, locals.var_t6__blk699_dn11, locals.var_t6__blk699_dn12, locals.var_t6__blk699_dn17,)
    }
};
        locals.var_t6__blk699 = assign22910_e32167;
        locals.var_t6__blk699_dn0 = assign22910_e32167_d_n0;
        locals.var_t6__blk699_dn2 = assign22910_e32167_d_n2;
        locals.var_t6__blk699_dn6 = assign22910_e32167_d_n6;
        locals.var_t6__blk699_dn7 = assign22910_e32167_d_n7;
        locals.var_t6__blk699_dn10 = assign22910_e32167_d_n10;
        locals.var_t6__blk699_dn11 = assign22910_e32167_d_n11;
        locals.var_t6__blk699_dn12 = assign22910_e32167_d_n12;
        locals.var_t6__blk699_dn17 = assign22910_e32167_d_n17;
        locals.var_t6__blk699_rv = 0.0;

        let (assign22920_e32177, assign22920_e32177_d_n0, assign22920_e32177_d_n2, assign22920_e32177_d_n6, assign22920_e32177_d_n7, assign22920_e32177_d_n10, assign22920_e32177_d_n11, assign22920_e32177_d_n12, assign22920_e32177_d_n17,) = {
    if (locals.var_guard694 != 0.0) {
        let assign22920_e32173: f64 = (locals.var_ps0z - locals.var_t5__blk698);
        let assign22920_e32174: f64 = (locals.var_beta * assign22920_e32173);
        let assign22920_e32175: f64 = assign22920_e32174;
        (assign22920_e32175, (locals.var_beta * (locals.var_ps0z_dn0 - locals.var_t5__blk698_dn0)), (locals.var_beta * (locals.var_ps0z_dn2 - locals.var_t5__blk698_dn2)), (locals.var_beta * (locals.var_ps0z_dn6 - locals.var_t5__blk698_dn6)), (locals.var_beta * (locals.var_ps0z_dn7 - locals.var_t5__blk698_dn7)), ((locals.var_beta_dn10 * assign22920_e32173) + (locals.var_beta * (locals.var_ps0z_dn10 - locals.var_t5__blk698_dn10))), (locals.var_beta * (locals.var_ps0z_dn11 - locals.var_t5__blk698_dn11)), (locals.var_beta * (locals.var_ps0z_dn12 - locals.var_t5__blk698_dn12)), (locals.var_beta * (locals.var_ps0z_dn17 - locals.var_t5__blk698_dn17)),)
    } else {
        (locals.var_t7__blk700, locals.var_t7__blk700_dn0, locals.var_t7__blk700_dn2, locals.var_t7__blk700_dn6, locals.var_t7__blk700_dn7, locals.var_t7__blk700_dn10, locals.var_t7__blk700_dn11, locals.var_t7__blk700_dn12, locals.var_t7__blk700_dn17,)
    }
};
        locals.var_t7__blk700 = assign22920_e32177;
        locals.var_t7__blk700_dn0 = assign22920_e32177_d_n0;
        locals.var_t7__blk700_dn2 = assign22920_e32177_d_n2;
        locals.var_t7__blk700_dn6 = assign22920_e32177_d_n6;
        locals.var_t7__blk700_dn7 = assign22920_e32177_d_n7;
        locals.var_t7__blk700_dn10 = assign22920_e32177_d_n10;
        locals.var_t7__blk700_dn11 = assign22920_e32177_d_n11;
        locals.var_t7__blk700_dn12 = assign22920_e32177_d_n12;
        locals.var_t7__blk700_dn17 = assign22920_e32177_d_n17;
        locals.var_t7__blk700_rv = 0.0;

        let (assign22930_e32190, assign22930_e32190_d_n0, assign22930_e32190_d_n2, assign22930_e32190_d_n6, assign22930_e32190_d_n7, assign22930_e32190_d_n10, assign22930_e32190_d_n11, assign22930_e32190_d_n12, assign22930_e32190_d_n17,) = {
    if (locals.var_guard694 != 0.0) {
        let (assign22930_e32188, assign22930_e32188_d_n0, assign22930_e32188_d_n2, assign22930_e32188_d_n6, assign22930_e32188_d_n7, assign22930_e32188_d_n10, assign22930_e32188_d_n11, assign22930_e32188_d_n12, assign22930_e32188_d_n17,) = {
            if (locals.var_t7__blk700 > 0.0) {
                let assign22930_e32183: f64 = (locals.var_t7__blk700).sqrt();
                (assign22930_e32183, (locals.var_t7__blk700_dn0 / (2.0 * assign22930_e32183)), (locals.var_t7__blk700_dn2 / (2.0 * assign22930_e32183)), (locals.var_t7__blk700_dn6 / (2.0 * assign22930_e32183)), (locals.var_t7__blk700_dn7 / (2.0 * assign22930_e32183)), (locals.var_t7__blk700_dn10 / (2.0 * assign22930_e32183)), (locals.var_t7__blk700_dn11 / (2.0 * assign22930_e32183)), (locals.var_t7__blk700_dn12 / (2.0 * assign22930_e32183)), (locals.var_t7__blk700_dn17 / (2.0 * assign22930_e32183)),)
            } else {
                let assign22930_e32185: f64 = (-locals.var_t7__blk700);
                let assign22930_e32186: f64 = (assign22930_e32185).sqrt();
                let assign22930_e32187: f64 = (-assign22930_e32186);
                (assign22930_e32187, (-((-locals.var_t7__blk700_dn0) / (2.0 * assign22930_e32186))), (-((-locals.var_t7__blk700_dn2) / (2.0 * assign22930_e32186))), (-((-locals.var_t7__blk700_dn6) / (2.0 * assign22930_e32186))), (-((-locals.var_t7__blk700_dn7) / (2.0 * assign22930_e32186))), (-((-locals.var_t7__blk700_dn10) / (2.0 * assign22930_e32186))), (-((-locals.var_t7__blk700_dn11) / (2.0 * assign22930_e32186))), (-((-locals.var_t7__blk700_dn12) / (2.0 * assign22930_e32186))), (-((-locals.var_t7__blk700_dn17) / (2.0 * assign22930_e32186))),)
            }
        };
        (assign22930_e32188, assign22930_e32188_d_n0, assign22930_e32188_d_n2, assign22930_e32188_d_n6, assign22930_e32188_d_n7, assign22930_e32188_d_n10, assign22930_e32188_d_n11, assign22930_e32188_d_n12, assign22930_e32188_d_n17,)
    } else {
        (locals.var_t7__blk700, locals.var_t7__blk700_dn0, locals.var_t7__blk700_dn2, locals.var_t7__blk700_dn6, locals.var_t7__blk700_dn7, locals.var_t7__blk700_dn10, locals.var_t7__blk700_dn11, locals.var_t7__blk700_dn12, locals.var_t7__blk700_dn17,)
    }
};
        locals.var_t7__blk700 = assign22930_e32190;
        locals.var_t7__blk700_dn0 = assign22930_e32190_d_n0;
        locals.var_t7__blk700_dn2 = assign22930_e32190_d_n2;
        locals.var_t7__blk700_dn6 = assign22930_e32190_d_n6;
        locals.var_t7__blk700_dn7 = assign22930_e32190_d_n7;
        locals.var_t7__blk700_dn10 = assign22930_e32190_d_n10;
        locals.var_t7__blk700_dn11 = assign22930_e32190_d_n11;
        locals.var_t7__blk700_dn12 = assign22930_e32190_d_n12;
        locals.var_t7__blk700_dn17 = assign22930_e32190_d_n17;
        locals.var_t7__blk700_rv = 0.0;

        let (assign22940_e32199, assign22940_e32199_d_n0, assign22940_e32199_d_n2, assign22940_e32199_d_n6, assign22940_e32199_d_n7, assign22940_e32199_d_n10, assign22940_e32199_d_n11, assign22940_e32199_d_n12, assign22940_e32199_d_n17,) = {
    if (locals.var_guard694 != 0.0) {
        let assign22940_e32195: f64 = (locals.var_beta * locals.var_ps0z);
        let assign22940_e32196: f64 = assign22940_e32195;
        let assign22940_e32197: f64 = (assign22940_e32196).sqrt();
        (assign22940_e32197, ((locals.var_beta * locals.var_ps0z_dn0) / (2.0 * assign22940_e32197)), ((locals.var_beta * locals.var_ps0z_dn2) / (2.0 * assign22940_e32197)), ((locals.var_beta * locals.var_ps0z_dn6) / (2.0 * assign22940_e32197)), ((locals.var_beta * locals.var_ps0z_dn7) / (2.0 * assign22940_e32197)), (((locals.var_beta_dn10 * locals.var_ps0z) + (locals.var_beta * locals.var_ps0z_dn10)) / (2.0 * assign22940_e32197)), ((locals.var_beta * locals.var_ps0z_dn11) / (2.0 * assign22940_e32197)), ((locals.var_beta * locals.var_ps0z_dn12) / (2.0 * assign22940_e32197)), ((locals.var_beta * locals.var_ps0z_dn17) / (2.0 * assign22940_e32197)),)
    } else {
        (locals.var_t8__blk701, locals.var_t8__blk701_dn0, locals.var_t8__blk701_dn2, locals.var_t8__blk701_dn6, locals.var_t8__blk701_dn7, locals.var_t8__blk701_dn10, locals.var_t8__blk701_dn11, locals.var_t8__blk701_dn12, locals.var_t8__blk701_dn17,)
    }
};
        locals.var_t8__blk701 = assign22940_e32199;
        locals.var_t8__blk701_dn0 = assign22940_e32199_d_n0;
        locals.var_t8__blk701_dn2 = assign22940_e32199_d_n2;
        locals.var_t8__blk701_dn6 = assign22940_e32199_d_n6;
        locals.var_t8__blk701_dn7 = assign22940_e32199_d_n7;
        locals.var_t8__blk701_dn10 = assign22940_e32199_d_n10;
        locals.var_t8__blk701_dn11 = assign22940_e32199_d_n11;
        locals.var_t8__blk701_dn12 = assign22940_e32199_d_n12;
        locals.var_t8__blk701_dn17 = assign22940_e32199_d_n17;
        locals.var_t8__blk701_rv = 0.0;

        let (assign22950_e32208, assign22950_e32208_d_n0, assign22950_e32208_d_n2, assign22950_e32208_d_n6, assign22950_e32208_d_n7, assign22950_e32208_d_n10, assign22950_e32208_d_n11, assign22950_e32208_d_n12, assign22950_e32208_d_n17,) = {
    if (locals.var_guard694 != 0.0) {
        let assign22950_e32202: f64 = (-locals.var_t6__blk699);
        let assign22950_e32205: f64 = (locals.var_t7__blk700 - locals.var_t8__blk701);
        let assign22950_e32206: f64 = (assign22950_e32202 * assign22950_e32205);
        (assign22950_e32206, (((-locals.var_t6__blk699_dn0) * assign22950_e32205) + (assign22950_e32202 * (locals.var_t7__blk700_dn0 - locals.var_t8__blk701_dn0))), (((-locals.var_t6__blk699_dn2) * assign22950_e32205) + (assign22950_e32202 * (locals.var_t7__blk700_dn2 - locals.var_t8__blk701_dn2))), (((-locals.var_t6__blk699_dn6) * assign22950_e32205) + (assign22950_e32202 * (locals.var_t7__blk700_dn6 - locals.var_t8__blk701_dn6))), (((-locals.var_t6__blk699_dn7) * assign22950_e32205) + (assign22950_e32202 * (locals.var_t7__blk700_dn7 - locals.var_t8__blk701_dn7))), (((-locals.var_t6__blk699_dn10) * assign22950_e32205) + (assign22950_e32202 * (locals.var_t7__blk700_dn10 - locals.var_t8__blk701_dn10))), (((-locals.var_t6__blk699_dn11) * assign22950_e32205) + (assign22950_e32202 * (locals.var_t7__blk700_dn11 - locals.var_t8__blk701_dn11))), (((-locals.var_t6__blk699_dn12) * assign22950_e32205) + (assign22950_e32202 * (locals.var_t7__blk700_dn12 - locals.var_t8__blk701_dn12))), (((-locals.var_t6__blk699_dn17) * assign22950_e32205) + (assign22950_e32202 * (locals.var_t7__blk700_dn17 - locals.var_t8__blk701_dn17))),)
    } else {
        (locals.var_t9__blk702, locals.var_t9__blk702_dn0, locals.var_t9__blk702_dn2, locals.var_t9__blk702_dn6, locals.var_t9__blk702_dn7, locals.var_t9__blk702_dn10, locals.var_t9__blk702_dn11, locals.var_t9__blk702_dn12, locals.var_t9__blk702_dn17,)
    }
};
        locals.var_t9__blk702 = assign22950_e32208;
        locals.var_t9__blk702_dn0 = assign22950_e32208_d_n0;
        locals.var_t9__blk702_dn2 = assign22950_e32208_d_n2;
        locals.var_t9__blk702_dn6 = assign22950_e32208_d_n6;
        locals.var_t9__blk702_dn7 = assign22950_e32208_d_n7;
        locals.var_t9__blk702_dn10 = assign22950_e32208_d_n10;
        locals.var_t9__blk702_dn11 = assign22950_e32208_d_n11;
        locals.var_t9__blk702_dn12 = assign22950_e32208_d_n12;
        locals.var_t9__blk702_dn17 = assign22950_e32208_d_n17;
        locals.var_t9__blk702_rv = 0.0;

        let (assign22960_e32218, assign22960_e32218_d_n0, assign22960_e32218_d_n2, assign22960_e32218_d_n6, assign22960_e32218_d_n7, assign22960_e32218_d_n10, assign22960_e32218_d_n11, assign22960_e32218_d_n12, assign22960_e32218_d_n17,) = {
    if (locals.var_guard694 != 0.0) {
        let assign22960_e32212: f64 = (p.p47 - locals.var_t9__blk702);
        let assign22960_e32215: f64 = (p.p47 * 0.01);
        let assign22960_e32216: f64 = (assign22960_e32212 - assign22960_e32215);
        (assign22960_e32216, (-locals.var_t9__blk702_dn0), (-locals.var_t9__blk702_dn2), (-locals.var_t9__blk702_dn6), (-locals.var_t9__blk702_dn7), (-locals.var_t9__blk702_dn10), (-locals.var_t9__blk702_dn11), (-locals.var_t9__blk702_dn12), (-locals.var_t9__blk702_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign22960_e32218;
        locals.var_tmf1_dn0 = assign22960_e32218_d_n0;
        locals.var_tmf1_dn2 = assign22960_e32218_d_n2;
        locals.var_tmf1_dn6 = assign22960_e32218_d_n6;
        locals.var_tmf1_dn7 = assign22960_e32218_d_n7;
        locals.var_tmf1_dn10 = assign22960_e32218_d_n10;
        locals.var_tmf1_dn11 = assign22960_e32218_d_n11;
        locals.var_tmf1_dn12 = assign22960_e32218_d_n12;
        locals.var_tmf1_dn17 = assign22960_e32218_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign22970_e32228, assign22970_e32228_d_n0, assign22970_e32228_d_n2, assign22970_e32228_d_n6, assign22970_e32228_d_n7, assign22970_e32228_d_n10, assign22970_e32228_d_n11, assign22970_e32228_d_n12, assign22970_e32228_d_n17,) = {
    if (locals.var_guard694 != 0.0) {
        let assign22970_e32222: f64 = (4.0 * p.p47);
        let assign22970_e32225: f64 = (p.p47 * 0.01);
        let assign22970_e32226: f64 = (assign22970_e32222 * assign22970_e32225);
        (assign22970_e32226, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign22970_e32228;
        locals.var_tmf2_dn0 = assign22970_e32228_d_n0;
        locals.var_tmf2_dn2 = assign22970_e32228_d_n2;
        locals.var_tmf2_dn6 = assign22970_e32228_d_n6;
        locals.var_tmf2_dn7 = assign22970_e32228_d_n7;
        locals.var_tmf2_dn10 = assign22970_e32228_d_n10;
        locals.var_tmf2_dn11 = assign22970_e32228_d_n11;
        locals.var_tmf2_dn12 = assign22970_e32228_d_n12;
        locals.var_tmf2_dn17 = assign22970_e32228_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign22980_e32238, assign22980_e32238_d_n0, assign22980_e32238_d_n2, assign22980_e32238_d_n6, assign22980_e32238_d_n7, assign22980_e32238_d_n10, assign22980_e32238_d_n11, assign22980_e32238_d_n12, assign22980_e32238_d_n17,) = {
    if (locals.var_guard694 != 0.0) {
        let (assign22980_e32236, assign22980_e32236_d_n0, assign22980_e32236_d_n2, assign22980_e32236_d_n6, assign22980_e32236_d_n7, assign22980_e32236_d_n10, assign22980_e32236_d_n11, assign22980_e32236_d_n12, assign22980_e32236_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign22980_e32235: f64 = (-locals.var_tmf2);
                (assign22980_e32235, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign22980_e32236, assign22980_e32236_d_n0, assign22980_e32236_d_n2, assign22980_e32236_d_n6, assign22980_e32236_d_n7, assign22980_e32236_d_n10, assign22980_e32236_d_n11, assign22980_e32236_d_n12, assign22980_e32236_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign22980_e32238;
        locals.var_tmf2_dn0 = assign22980_e32238_d_n0;
        locals.var_tmf2_dn2 = assign22980_e32238_d_n2;
        locals.var_tmf2_dn6 = assign22980_e32238_d_n6;
        locals.var_tmf2_dn7 = assign22980_e32238_d_n7;
        locals.var_tmf2_dn10 = assign22980_e32238_d_n10;
        locals.var_tmf2_dn11 = assign22980_e32238_d_n11;
        locals.var_tmf2_dn12 = assign22980_e32238_d_n12;
        locals.var_tmf2_dn17 = assign22980_e32238_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign22990_e32247, assign22990_e32247_d_n0, assign22990_e32247_d_n2, assign22990_e32247_d_n6, assign22990_e32247_d_n7, assign22990_e32247_d_n10, assign22990_e32247_d_n11, assign22990_e32247_d_n12, assign22990_e32247_d_n17,) = {
    if (locals.var_guard694 != 0.0) {
        let assign22990_e32242: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign22990_e32244: f64 = (assign22990_e32242 + locals.var_tmf2);
        let assign22990_e32245: f64 = (assign22990_e32244).sqrt();
        (assign22990_e32245, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign22990_e32245)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign22990_e32245)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign22990_e32245)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign22990_e32245)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign22990_e32245)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign22990_e32245)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign22990_e32245)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign22990_e32245)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign22990_e32247;
        locals.var_tmf2_dn0 = assign22990_e32247_d_n0;
        locals.var_tmf2_dn2 = assign22990_e32247_d_n2;
        locals.var_tmf2_dn6 = assign22990_e32247_d_n6;
        locals.var_tmf2_dn7 = assign22990_e32247_d_n7;
        locals.var_tmf2_dn10 = assign22990_e32247_d_n10;
        locals.var_tmf2_dn11 = assign22990_e32247_d_n11;
        locals.var_tmf2_dn12 = assign22990_e32247_d_n12;
        locals.var_tmf2_dn17 = assign22990_e32247_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign23000_e32257, assign23000_e32257_d_n0, assign23000_e32257_d_n2, assign23000_e32257_d_n6, assign23000_e32257_d_n7, assign23000_e32257_d_n10, assign23000_e32257_d_n11, assign23000_e32257_d_n12, assign23000_e32257_d_n17,) = {
    if (locals.var_guard694 != 0.0) {
        let assign23000_e32253: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign23000_e32254: f64 = (0.5 * assign23000_e32253);
        let assign23000_e32255: f64 = (p.p47 - assign23000_e32254);
        (assign23000_e32255, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (-(0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (-(0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_qhs, locals.var_qhs_dn0, locals.var_qhs_dn2, locals.var_qhs_dn6, locals.var_qhs_dn7, locals.var_qhs_dn10, locals.var_qhs_dn11, locals.var_qhs_dn12, locals.var_qhs_dn17,)
    }
};
        locals.var_qhs = assign23000_e32257;
        locals.var_qhs_dn0 = assign23000_e32257_d_n0;
        locals.var_qhs_dn2 = assign23000_e32257_d_n2;
        locals.var_qhs_dn6 = assign23000_e32257_d_n6;
        locals.var_qhs_dn7 = assign23000_e32257_d_n7;
        locals.var_qhs_dn10 = assign23000_e32257_d_n10;
        locals.var_qhs_dn11 = assign23000_e32257_d_n11;
        locals.var_qhs_dn12 = assign23000_e32257_d_n12;
        locals.var_qhs_dn17 = assign23000_e32257_d_n17;
        locals.var_qhs_rv = 0.0;

        let (assign23050_e32292, assign23050_e32292_d_n17,) = {
    if (locals.var_guard694 != 0.0) {
        let assign23050_e32288: f64 = (1e-9 / 0.0001);
        let assign23050_e32290: f64 = (assign23050_e32288 * (nv17 - 0.0));
        (assign23050_e32290, assign23050_e32288,)
    } else {
        (locals.var_qhs_hist, locals.var_qhs_hist_dn17,)
    }
};
        locals.var_qhs_hist = assign23050_e32292;
        locals.var_qhs_hist_dn17 = assign23050_e32292_d_n17;
        locals.var_qhs_hist_rv = 0.0;

        let (assign23060_e32296, assign23060_e32296_d_n0, assign23060_e32296_d_n2, assign23060_e32296_d_n6, assign23060_e32296_d_n7, assign23060_e32296_d_n10, assign23060_e32296_d_n11, assign23060_e32296_d_n12, assign23060_e32296_d_n17,) = {
    if (locals.var_guard694 != 0.0) {
        (locals.var_qhs_hist, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, locals.var_qhs_hist_dn17,)
    } else {
        (locals.var_qhs, locals.var_qhs_dn0, locals.var_qhs_dn2, locals.var_qhs_dn6, locals.var_qhs_dn7, locals.var_qhs_dn10, locals.var_qhs_dn11, locals.var_qhs_dn12, locals.var_qhs_dn17,)
    }
};
        locals.var_qhs = assign23060_e32296;
        locals.var_qhs_dn0 = assign23060_e32296_d_n0;
        locals.var_qhs_dn2 = assign23060_e32296_d_n2;
        locals.var_qhs_dn6 = assign23060_e32296_d_n6;
        locals.var_qhs_dn7 = assign23060_e32296_d_n7;
        locals.var_qhs_dn10 = assign23060_e32296_d_n10;
        locals.var_qhs_dn11 = assign23060_e32296_d_n11;
        locals.var_qhs_dn12 = assign23060_e32296_d_n12;
        locals.var_qhs_dn17 = assign23060_e32296_d_n17;
        locals.var_qhs_rv = 0.0;

        let assign23080_e32315: f64 = if (((locals.var_flg_noqi == 0.0) && (locals.var_isub > 0.0)) && (p.p146 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard716 = assign23080_e32315;
        locals.var_guard716_rv = 0.0;

        let assign23090_e32318: f64 = if locals.var_subversion < 3.0 { 1.0 } else { 0.0 };
        locals.var_guard717 = assign23090_e32318;
        locals.var_guard717_rv = 0.0;

        let (assign23100_e32324, assign23100_e32324_d_n0, assign23100_e32324_d_n2, assign23100_e32324_d_n6, assign23100_e32324_d_n7, assign23100_e32324_d_n10, assign23100_e32324_d_n11, assign23100_e32324_d_n12, assign23100_e32324_d_n17,) = {
    if ((locals.var_guard716 != 0.0) && (locals.var_guard717 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs0, locals.var_vbs0_dn0, locals.var_vbs0_dn2, locals.var_vbs0_dn6, locals.var_vbs0_dn7, locals.var_vbs0_dn10, locals.var_vbs0_dn11, locals.var_vbs0_dn12, locals.var_vbs0_dn17,)
    }
};
        locals.var_vbs0 = assign23100_e32324;
        locals.var_vbs0_dn0 = assign23100_e32324_d_n0;
        locals.var_vbs0_dn2 = assign23100_e32324_d_n2;
        locals.var_vbs0_dn6 = assign23100_e32324_d_n6;
        locals.var_vbs0_dn7 = assign23100_e32324_d_n7;
        locals.var_vbs0_dn10 = assign23100_e32324_d_n10;
        locals.var_vbs0_dn11 = assign23100_e32324_d_n11;
        locals.var_vbs0_dn12 = assign23100_e32324_d_n12;
        locals.var_vbs0_dn17 = assign23100_e32324_d_n17;
        locals.var_vbs0_rv = 0.0;

        let (assign23110_e32330, assign23110_e32330_d_n0, assign23110_e32330_d_n2, assign23110_e32330_d_n6, assign23110_e32330_d_n7, assign23110_e32330_d_n10, assign23110_e32330_d_n11, assign23110_e32330_d_n12, assign23110_e32330_d_n17,) = {
    if ((locals.var_guard716 != 0.0) && (locals.var_guard717 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbsl, locals.var_vbsl_dn0, locals.var_vbsl_dn2, locals.var_vbsl_dn6, locals.var_vbsl_dn7, locals.var_vbsl_dn10, locals.var_vbsl_dn11, locals.var_vbsl_dn12, locals.var_vbsl_dn17,)
    }
};
        locals.var_vbsl = assign23110_e32330;
        locals.var_vbsl_dn0 = assign23110_e32330_d_n0;
        locals.var_vbsl_dn2 = assign23110_e32330_d_n2;
        locals.var_vbsl_dn6 = assign23110_e32330_d_n6;
        locals.var_vbsl_dn7 = assign23110_e32330_d_n7;
        locals.var_vbsl_dn10 = assign23110_e32330_d_n10;
        locals.var_vbsl_dn11 = assign23110_e32330_d_n11;
        locals.var_vbsl_dn12 = assign23110_e32330_d_n12;
        locals.var_vbsl_dn17 = assign23110_e32330_d_n17;
        locals.var_vbsl_rv = 0.0;

        let (assign23120_e32342, assign23120_e32342_d_n0, assign23120_e32342_d_n2, assign23120_e32342_d_n6, assign23120_e32342_d_n7, assign23120_e32342_d_n10, assign23120_e32342_d_n11, assign23120_e32342_d_n12, assign23120_e32342_d_n17,) = {
    if ((locals.var_guard716 != 0.0) && (locals.var_guard717 == 0.0)) {
        let (assign23120_e32340, assign23120_e32340_d_n0, assign23120_e32340_d_n2, assign23120_e32340_d_n6, assign23120_e32340_d_n7, assign23120_e32340_d_n10, assign23120_e32340_d_n11, assign23120_e32340_d_n12, assign23120_e32340_d_n17,) = {
            if (p.p43 == 1.0) {
                (locals.var_vbs, locals.var_vbs_dn0, locals.var_vbs_dn2, locals.var_vbs_dn6, locals.var_vbs_dn7, locals.var_vbs_dn10, locals.var_vbs_dn11, locals.var_vbs_dn12, locals.var_vbs_dn17,)
            } else {
                (locals.var_phi_b0_soi, locals.var_phi_b0_soi_dn0, locals.var_phi_b0_soi_dn2, locals.var_phi_b0_soi_dn6, locals.var_phi_b0_soi_dn7, locals.var_phi_b0_soi_dn10, locals.var_phi_b0_soi_dn11, locals.var_phi_b0_soi_dn12, locals.var_phi_b0_soi_dn17,)
            }
        };
        (assign23120_e32340, assign23120_e32340_d_n0, assign23120_e32340_d_n2, assign23120_e32340_d_n6, assign23120_e32340_d_n7, assign23120_e32340_d_n10, assign23120_e32340_d_n11, assign23120_e32340_d_n12, assign23120_e32340_d_n17,)
    } else {
        (locals.var_vbs0, locals.var_vbs0_dn0, locals.var_vbs0_dn2, locals.var_vbs0_dn6, locals.var_vbs0_dn7, locals.var_vbs0_dn10, locals.var_vbs0_dn11, locals.var_vbs0_dn12, locals.var_vbs0_dn17,)
    }
};
        locals.var_vbs0 = assign23120_e32342;
        locals.var_vbs0_dn0 = assign23120_e32342_d_n0;
        locals.var_vbs0_dn2 = assign23120_e32342_d_n2;
        locals.var_vbs0_dn6 = assign23120_e32342_d_n6;
        locals.var_vbs0_dn7 = assign23120_e32342_d_n7;
        locals.var_vbs0_dn10 = assign23120_e32342_d_n10;
        locals.var_vbs0_dn11 = assign23120_e32342_d_n11;
        locals.var_vbs0_dn12 = assign23120_e32342_d_n12;
        locals.var_vbs0_dn17 = assign23120_e32342_d_n17;
        locals.var_vbs0_rv = 0.0;

        let (assign23130_e32354, assign23130_e32354_d_n0, assign23130_e32354_d_n2, assign23130_e32354_d_n6, assign23130_e32354_d_n7, assign23130_e32354_d_n10, assign23130_e32354_d_n11, assign23130_e32354_d_n12, assign23130_e32354_d_n17,) = {
    if ((locals.var_guard716 != 0.0) && (locals.var_guard717 == 0.0)) {
        let (assign23130_e32352, assign23130_e32352_d_n0, assign23130_e32352_d_n2, assign23130_e32352_d_n6, assign23130_e32352_d_n7, assign23130_e32352_d_n10, assign23130_e32352_d_n11, assign23130_e32352_d_n12, assign23130_e32352_d_n17,) = {
            if (p.p43 == 1.0) {
                (locals.var_vbs, locals.var_vbs_dn0, locals.var_vbs_dn2, locals.var_vbs_dn6, locals.var_vbs_dn7, locals.var_vbs_dn10, locals.var_vbs_dn11, locals.var_vbs_dn12, locals.var_vbs_dn17,)
            } else {
                (locals.var_phi_bl_soi, locals.var_phi_bl_soi_dn0, locals.var_phi_bl_soi_dn2, locals.var_phi_bl_soi_dn6, locals.var_phi_bl_soi_dn7, locals.var_phi_bl_soi_dn10, locals.var_phi_bl_soi_dn11, locals.var_phi_bl_soi_dn12, locals.var_phi_bl_soi_dn17,)
            }
        };
        (assign23130_e32352, assign23130_e32352_d_n0, assign23130_e32352_d_n2, assign23130_e32352_d_n6, assign23130_e32352_d_n7, assign23130_e32352_d_n10, assign23130_e32352_d_n11, assign23130_e32352_d_n12, assign23130_e32352_d_n17,)
    } else {
        (locals.var_vbsl, locals.var_vbsl_dn0, locals.var_vbsl_dn2, locals.var_vbsl_dn6, locals.var_vbsl_dn7, locals.var_vbsl_dn10, locals.var_vbsl_dn11, locals.var_vbsl_dn12, locals.var_vbsl_dn17,)
    }
};
        locals.var_vbsl = assign23130_e32354;
        locals.var_vbsl_dn0 = assign23130_e32354_d_n0;
        locals.var_vbsl_dn2 = assign23130_e32354_d_n2;
        locals.var_vbsl_dn6 = assign23130_e32354_d_n6;
        locals.var_vbsl_dn7 = assign23130_e32354_d_n7;
        locals.var_vbsl_dn10 = assign23130_e32354_d_n10;
        locals.var_vbsl_dn11 = assign23130_e32354_d_n11;
        locals.var_vbsl_dn12 = assign23130_e32354_d_n12;
        locals.var_vbsl_dn17 = assign23130_e32354_d_n17;
        locals.var_vbsl_rv = 0.0;

        let (assign23140_e32362, assign23140_e32362_d_n0, assign23140_e32362_d_n2, assign23140_e32362_d_n6, assign23140_e32362_d_n7, assign23140_e32362_d_n10, assign23140_e32362_d_n11, assign23140_e32362_d_n12, assign23140_e32362_d_n17,) = {
    if (locals.var_guard716 != 0.0) {
        let assign23140_e32359: f64 = (p.p147 * locals.var_dvth);
        let assign23140_e32360: f64 = (1.0 + assign23140_e32359);
        (assign23140_e32360, (p.p147 * locals.var_dvth_dn0), (p.p147 * locals.var_dvth_dn2), (p.p147 * locals.var_dvth_dn6), (p.p147 * locals.var_dvth_dn7), (p.p147 * locals.var_dvth_dn10), (p.p147 * locals.var_dvth_dn11), (p.p147 * locals.var_dvth_dn12), (p.p147 * locals.var_dvth_dn17),)
    } else {
        (locals.var_t0__blk703, locals.var_t0__blk703_dn0, locals.var_t0__blk703_dn2, locals.var_t0__blk703_dn6, locals.var_t0__blk703_dn7, locals.var_t0__blk703_dn10, locals.var_t0__blk703_dn11, locals.var_t0__blk703_dn12, locals.var_t0__blk703_dn17,)
    }
};
        locals.var_t0__blk703 = assign23140_e32362;
        locals.var_t0__blk703_dn0 = assign23140_e32362_d_n0;
        locals.var_t0__blk703_dn2 = assign23140_e32362_d_n2;
        locals.var_t0__blk703_dn6 = assign23140_e32362_d_n6;
        locals.var_t0__blk703_dn7 = assign23140_e32362_d_n7;
        locals.var_t0__blk703_dn10 = assign23140_e32362_d_n10;
        locals.var_t0__blk703_dn11 = assign23140_e32362_d_n11;
        locals.var_t0__blk703_dn12 = assign23140_e32362_d_n12;
        locals.var_t0__blk703_dn17 = assign23140_e32362_d_n17;
        locals.var_t0__blk703_rv = 0.0;

        let (assign23150_e32370, assign23150_e32370_d_n0, assign23150_e32370_d_n2, assign23150_e32370_d_n6, assign23150_e32370_d_n7, assign23150_e32370_d_n10, assign23150_e32370_d_n11, assign23150_e32370_d_n12, assign23150_e32370_d_n17,) = {
    if (locals.var_guard716 != 0.0) {
        let assign23150_e32366: f64 = (p.p146 * locals.var_t0__blk703);
        let assign23150_e32368: f64 = (assign23150_e32366 * locals.var_isub);
        (assign23150_e32368, (((p.p146 * locals.var_t0__blk703_dn0) * locals.var_isub) + (assign23150_e32366 * locals.var_isub_dn0)), (((p.p146 * locals.var_t0__blk703_dn2) * locals.var_isub) + (assign23150_e32366 * locals.var_isub_dn2)), (((p.p146 * locals.var_t0__blk703_dn6) * locals.var_isub) + (assign23150_e32366 * locals.var_isub_dn6)), (((p.p146 * locals.var_t0__blk703_dn7) * locals.var_isub) + (assign23150_e32366 * locals.var_isub_dn7)), (((p.p146 * locals.var_t0__blk703_dn10) * locals.var_isub) + (assign23150_e32366 * locals.var_isub_dn10)), (((p.p146 * locals.var_t0__blk703_dn11) * locals.var_isub) + (assign23150_e32366 * locals.var_isub_dn11)), (((p.p146 * locals.var_t0__blk703_dn12) * locals.var_isub) + (assign23150_e32366 * locals.var_isub_dn12)), (((p.p146 * locals.var_t0__blk703_dn17) * locals.var_isub) + (assign23150_e32366 * locals.var_isub_dn17)),)
    } else {
        (locals.var_dvbsibpc, locals.var_dvbsibpc_dn0, locals.var_dvbsibpc_dn2, locals.var_dvbsibpc_dn6, locals.var_dvbsibpc_dn7, locals.var_dvbsibpc_dn10, locals.var_dvbsibpc_dn11, locals.var_dvbsibpc_dn12, locals.var_dvbsibpc_dn17,)
    }
};
        locals.var_dvbsibpc = assign23150_e32370;
        locals.var_dvbsibpc_dn0 = assign23150_e32370_d_n0;
        locals.var_dvbsibpc_dn2 = assign23150_e32370_d_n2;
        locals.var_dvbsibpc_dn6 = assign23150_e32370_d_n6;
        locals.var_dvbsibpc_dn7 = assign23150_e32370_d_n7;
        locals.var_dvbsibpc_dn10 = assign23150_e32370_d_n10;
        locals.var_dvbsibpc_dn11 = assign23150_e32370_d_n11;
        locals.var_dvbsibpc_dn12 = assign23150_e32370_d_n12;
        locals.var_dvbsibpc_dn17 = assign23150_e32370_d_n17;
        locals.var_dvbsibpc_rv = 0.0;

        let (assign23160_e32380, assign23160_e32380_d_n0, assign23160_e32380_d_n2, assign23160_e32380_d_n6, assign23160_e32380_d_n7, assign23160_e32380_d_n10, assign23160_e32380_d_n11, assign23160_e32380_d_n12, assign23160_e32380_d_n17,) = {
    if (locals.var_guard716 != 0.0) {
        let assign23160_e32375: f64 = (locals.var_ps0 - locals.var_vbs0);
        let assign23160_e32376: f64 = (locals.var_beta * assign23160_e32375);
        let assign23160_e32378: f64 = (assign23160_e32376 - 1.0);
        (assign23160_e32378, (locals.var_beta * (locals.var_ps0_dn0 - locals.var_vbs0_dn0)), (locals.var_beta * (locals.var_ps0_dn2 - locals.var_vbs0_dn2)), (locals.var_beta * (locals.var_ps0_dn6 - locals.var_vbs0_dn6)), (locals.var_beta * (locals.var_ps0_dn7 - locals.var_vbs0_dn7)), ((locals.var_beta_dn10 * assign23160_e32375) + (locals.var_beta * (locals.var_ps0_dn10 - locals.var_vbs0_dn10))), (locals.var_beta * (locals.var_ps0_dn11 - locals.var_vbs0_dn11)), (locals.var_beta * (locals.var_ps0_dn12 - locals.var_vbs0_dn12)), (locals.var_beta * (locals.var_ps0_dn17 - locals.var_vbs0_dn17)),)
    } else {
        (locals.var_xi0__blk705, locals.var_xi0__blk705_dn0, locals.var_xi0__blk705_dn2, locals.var_xi0__blk705_dn6, locals.var_xi0__blk705_dn7, locals.var_xi0__blk705_dn10, locals.var_xi0__blk705_dn11, locals.var_xi0__blk705_dn12, locals.var_xi0__blk705_dn17,)
    }
};
        locals.var_xi0__blk705 = assign23160_e32380;
        locals.var_xi0__blk705_dn0 = assign23160_e32380_d_n0;
        locals.var_xi0__blk705_dn2 = assign23160_e32380_d_n2;
        locals.var_xi0__blk705_dn6 = assign23160_e32380_d_n6;
        locals.var_xi0__blk705_dn7 = assign23160_e32380_d_n7;
        locals.var_xi0__blk705_dn10 = assign23160_e32380_d_n10;
        locals.var_xi0__blk705_dn11 = assign23160_e32380_d_n11;
        locals.var_xi0__blk705_dn12 = assign23160_e32380_d_n12;
        locals.var_xi0__blk705_dn17 = assign23160_e32380_d_n17;
        locals.var_xi0__blk705_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_83(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign23170_e32393, assign23170_e32393_d_n0, assign23170_e32393_d_n2, assign23170_e32393_d_n6, assign23170_e32393_d_n7, assign23170_e32393_d_n10, assign23170_e32393_d_n11, assign23170_e32393_d_n12, assign23170_e32393_d_n17,) = {
    if (locals.var_guard716 != 0.0) {
        let assign23170_e32384: f64 = (locals.var_xi0__blk705 * locals.var_xi0__blk705);
        let assign23170_e32387: f64 = (4.0 * 0.1);
        let assign23170_e32389: f64 = (assign23170_e32387 * 0.1);
        let assign23170_e32390: f64 = (assign23170_e32384 + assign23170_e32389);
        let assign23170_e32391: f64 = (assign23170_e32390).sqrt();
        (assign23170_e32391, (((locals.var_xi0__blk705_dn0 * locals.var_xi0__blk705) + (locals.var_xi0__blk705 * locals.var_xi0__blk705_dn0)) / (2.0 * assign23170_e32391)), (((locals.var_xi0__blk705_dn2 * locals.var_xi0__blk705) + (locals.var_xi0__blk705 * locals.var_xi0__blk705_dn2)) / (2.0 * assign23170_e32391)), (((locals.var_xi0__blk705_dn6 * locals.var_xi0__blk705) + (locals.var_xi0__blk705 * locals.var_xi0__blk705_dn6)) / (2.0 * assign23170_e32391)), (((locals.var_xi0__blk705_dn7 * locals.var_xi0__blk705) + (locals.var_xi0__blk705 * locals.var_xi0__blk705_dn7)) / (2.0 * assign23170_e32391)), (((locals.var_xi0__blk705_dn10 * locals.var_xi0__blk705) + (locals.var_xi0__blk705 * locals.var_xi0__blk705_dn10)) / (2.0 * assign23170_e32391)), (((locals.var_xi0__blk705_dn11 * locals.var_xi0__blk705) + (locals.var_xi0__blk705 * locals.var_xi0__blk705_dn11)) / (2.0 * assign23170_e32391)), (((locals.var_xi0__blk705_dn12 * locals.var_xi0__blk705) + (locals.var_xi0__blk705 * locals.var_xi0__blk705_dn12)) / (2.0 * assign23170_e32391)), (((locals.var_xi0__blk705_dn17 * locals.var_xi0__blk705) + (locals.var_xi0__blk705 * locals.var_xi0__blk705_dn17)) / (2.0 * assign23170_e32391)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign23170_e32393;
        locals.var_tmf1_dn0 = assign23170_e32393_d_n0;
        locals.var_tmf1_dn2 = assign23170_e32393_d_n2;
        locals.var_tmf1_dn6 = assign23170_e32393_d_n6;
        locals.var_tmf1_dn7 = assign23170_e32393_d_n7;
        locals.var_tmf1_dn10 = assign23170_e32393_d_n10;
        locals.var_tmf1_dn11 = assign23170_e32393_d_n11;
        locals.var_tmf1_dn12 = assign23170_e32393_d_n12;
        locals.var_tmf1_dn17 = assign23170_e32393_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign23180_e32405, assign23180_e32405_d_n0, assign23180_e32405_d_n2, assign23180_e32405_d_n6, assign23180_e32405_d_n7, assign23180_e32405_d_n10, assign23180_e32405_d_n11, assign23180_e32405_d_n12, assign23180_e32405_d_n17,) = {
    if (locals.var_guard716 != 0.0) {
        let assign23180_e32398: f64 = (locals.var_xi0__blk705 + locals.var_tmf1);
        let assign23180_e32399: f64 = (0.5 * assign23180_e32398);
        let assign23180_e32402: f64 = (1e-10 * 0.1);
        let assign23180_e32403: f64 = (assign23180_e32399 + assign23180_e32402);
        (assign23180_e32403, (0.5 * (locals.var_xi0__blk705_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_xi0__blk705_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_xi0__blk705_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_xi0__blk705_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_xi0__blk705_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_xi0__blk705_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_xi0__blk705_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_xi0__blk705_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_xi0__blk705, locals.var_xi0__blk705_dn0, locals.var_xi0__blk705_dn2, locals.var_xi0__blk705_dn6, locals.var_xi0__blk705_dn7, locals.var_xi0__blk705_dn10, locals.var_xi0__blk705_dn11, locals.var_xi0__blk705_dn12, locals.var_xi0__blk705_dn17,)
    }
};
        locals.var_xi0__blk705 = assign23180_e32405;
        locals.var_xi0__blk705_dn0 = assign23180_e32405_d_n0;
        locals.var_xi0__blk705_dn2 = assign23180_e32405_d_n2;
        locals.var_xi0__blk705_dn6 = assign23180_e32405_d_n6;
        locals.var_xi0__blk705_dn7 = assign23180_e32405_d_n7;
        locals.var_xi0__blk705_dn10 = assign23180_e32405_d_n10;
        locals.var_xi0__blk705_dn11 = assign23180_e32405_d_n11;
        locals.var_xi0__blk705_dn12 = assign23180_e32405_d_n12;
        locals.var_xi0__blk705_dn17 = assign23180_e32405_d_n17;
        locals.var_xi0__blk705_rv = 0.0;

        let assign23190_e32408: f64 = if locals.var_xi0__blk705 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard718 = assign23190_e32408;
        locals.var_guard718_rv = 0.0;

        let (assign23200_e32414, assign23200_e32414_d_n0, assign23200_e32414_d_n2, assign23200_e32414_d_n6, assign23200_e32414_d_n7, assign23200_e32414_d_n10, assign23200_e32414_d_n11, assign23200_e32414_d_n12, assign23200_e32414_d_n17,) = {
    if ((locals.var_guard716 != 0.0) && (locals.var_guard718 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xi0__blk705, locals.var_xi0__blk705_dn0, locals.var_xi0__blk705_dn2, locals.var_xi0__blk705_dn6, locals.var_xi0__blk705_dn7, locals.var_xi0__blk705_dn10, locals.var_xi0__blk705_dn11, locals.var_xi0__blk705_dn12, locals.var_xi0__blk705_dn17,)
    }
};
        locals.var_xi0__blk705 = assign23200_e32414;
        locals.var_xi0__blk705_dn0 = assign23200_e32414_d_n0;
        locals.var_xi0__blk705_dn2 = assign23200_e32414_d_n2;
        locals.var_xi0__blk705_dn6 = assign23200_e32414_d_n6;
        locals.var_xi0__blk705_dn7 = assign23200_e32414_d_n7;
        locals.var_xi0__blk705_dn10 = assign23200_e32414_d_n10;
        locals.var_xi0__blk705_dn11 = assign23200_e32414_d_n11;
        locals.var_xi0__blk705_dn12 = assign23200_e32414_d_n12;
        locals.var_xi0__blk705_dn17 = assign23200_e32414_d_n17;
        locals.var_xi0__blk705_rv = 0.0;

        let (assign23210_e32419, assign23210_e32419_d_n0, assign23210_e32419_d_n2, assign23210_e32419_d_n6, assign23210_e32419_d_n7, assign23210_e32419_d_n10, assign23210_e32419_d_n11, assign23210_e32419_d_n12, assign23210_e32419_d_n17,) = {
    if (locals.var_guard716 != 0.0) {
        let assign23210_e32417: f64 = (locals.var_xi0__blk705).sqrt();
        (assign23210_e32417, (locals.var_xi0__blk705_dn0 / (2.0 * assign23210_e32417)), (locals.var_xi0__blk705_dn2 / (2.0 * assign23210_e32417)), (locals.var_xi0__blk705_dn6 / (2.0 * assign23210_e32417)), (locals.var_xi0__blk705_dn7 / (2.0 * assign23210_e32417)), (locals.var_xi0__blk705_dn10 / (2.0 * assign23210_e32417)), (locals.var_xi0__blk705_dn11 / (2.0 * assign23210_e32417)), (locals.var_xi0__blk705_dn12 / (2.0 * assign23210_e32417)), (locals.var_xi0__blk705_dn17 / (2.0 * assign23210_e32417)),)
    } else {
        (locals.var_xi0p12__blk706, locals.var_xi0p12__blk706_dn0, locals.var_xi0p12__blk706_dn2, locals.var_xi0p12__blk706_dn6, locals.var_xi0p12__blk706_dn7, locals.var_xi0p12__blk706_dn10, locals.var_xi0p12__blk706_dn11, locals.var_xi0p12__blk706_dn12, locals.var_xi0p12__blk706_dn17,)
    }
};
        locals.var_xi0p12__blk706 = assign23210_e32419;
        locals.var_xi0p12__blk706_dn0 = assign23210_e32419_d_n0;
        locals.var_xi0p12__blk706_dn2 = assign23210_e32419_d_n2;
        locals.var_xi0p12__blk706_dn6 = assign23210_e32419_d_n6;
        locals.var_xi0p12__blk706_dn7 = assign23210_e32419_d_n7;
        locals.var_xi0p12__blk706_dn10 = assign23210_e32419_d_n10;
        locals.var_xi0p12__blk706_dn11 = assign23210_e32419_d_n11;
        locals.var_xi0p12__blk706_dn12 = assign23210_e32419_d_n12;
        locals.var_xi0p12__blk706_dn17 = assign23210_e32419_d_n17;
        locals.var_xi0p12__blk706_rv = 0.0;

        let (assign23220_e32425, assign23220_e32425_d_n0, assign23220_e32425_d_n2, assign23220_e32425_d_n6, assign23220_e32425_d_n7, assign23220_e32425_d_n10, assign23220_e32425_d_n11, assign23220_e32425_d_n12, assign23220_e32425_d_n17,) = {
    if (locals.var_guard716 != 0.0) {
        let assign23220_e32423: f64 = (locals.var_xi0__blk705 * locals.var_xi0p12__blk706);
        (assign23220_e32423, ((locals.var_xi0__blk705_dn0 * locals.var_xi0p12__blk706) + (locals.var_xi0__blk705 * locals.var_xi0p12__blk706_dn0)), ((locals.var_xi0__blk705_dn2 * locals.var_xi0p12__blk706) + (locals.var_xi0__blk705 * locals.var_xi0p12__blk706_dn2)), ((locals.var_xi0__blk705_dn6 * locals.var_xi0p12__blk706) + (locals.var_xi0__blk705 * locals.var_xi0p12__blk706_dn6)), ((locals.var_xi0__blk705_dn7 * locals.var_xi0p12__blk706) + (locals.var_xi0__blk705 * locals.var_xi0p12__blk706_dn7)), ((locals.var_xi0__blk705_dn10 * locals.var_xi0p12__blk706) + (locals.var_xi0__blk705 * locals.var_xi0p12__blk706_dn10)), ((locals.var_xi0__blk705_dn11 * locals.var_xi0p12__blk706) + (locals.var_xi0__blk705 * locals.var_xi0p12__blk706_dn11)), ((locals.var_xi0__blk705_dn12 * locals.var_xi0p12__blk706) + (locals.var_xi0__blk705 * locals.var_xi0p12__blk706_dn12)), ((locals.var_xi0__blk705_dn17 * locals.var_xi0p12__blk706) + (locals.var_xi0__blk705 * locals.var_xi0p12__blk706_dn17)),)
    } else {
        (locals.var_xi0p32, locals.var_xi0p32_dn0, locals.var_xi0p32_dn2, locals.var_xi0p32_dn6, locals.var_xi0p32_dn7, locals.var_xi0p32_dn10, locals.var_xi0p32_dn11, locals.var_xi0p32_dn12, locals.var_xi0p32_dn17,)
    }
};
        locals.var_xi0p32 = assign23220_e32425;
        locals.var_xi0p32_dn0 = assign23220_e32425_d_n0;
        locals.var_xi0p32_dn2 = assign23220_e32425_d_n2;
        locals.var_xi0p32_dn6 = assign23220_e32425_d_n6;
        locals.var_xi0p32_dn7 = assign23220_e32425_d_n7;
        locals.var_xi0p32_dn10 = assign23220_e32425_d_n10;
        locals.var_xi0p32_dn11 = assign23220_e32425_d_n11;
        locals.var_xi0p32_dn12 = assign23220_e32425_d_n12;
        locals.var_xi0p32_dn17 = assign23220_e32425_d_n17;
        locals.var_xi0p32_rv = 0.0;

        let (assign23230_e32435, assign23230_e32435_d_n0, assign23230_e32435_d_n2, assign23230_e32435_d_n6, assign23230_e32435_d_n7, assign23230_e32435_d_n10, assign23230_e32435_d_n11, assign23230_e32435_d_n12, assign23230_e32435_d_n17,) = {
    if (locals.var_guard716 != 0.0) {
        let assign23230_e32430: f64 = (locals.var_psl - locals.var_vbsl);
        let assign23230_e32431: f64 = (locals.var_beta * assign23230_e32430);
        let assign23230_e32433: f64 = (assign23230_e32431 - 1.0);
        (assign23230_e32433, (locals.var_beta * (locals.var_psl_dn0 - locals.var_vbsl_dn0)), (locals.var_beta * (locals.var_psl_dn2 - locals.var_vbsl_dn2)), (locals.var_beta * (locals.var_psl_dn6 - locals.var_vbsl_dn6)), (locals.var_beta * (locals.var_psl_dn7 - locals.var_vbsl_dn7)), ((locals.var_beta_dn10 * assign23230_e32430) + (locals.var_beta * (locals.var_psl_dn10 - locals.var_vbsl_dn10))), (locals.var_beta * (locals.var_psl_dn11 - locals.var_vbsl_dn11)), (locals.var_beta * (locals.var_psl_dn12 - locals.var_vbsl_dn12)), (locals.var_beta * (locals.var_psl_dn17 - locals.var_vbsl_dn17)),)
    } else {
        (locals.var_xil, locals.var_xil_dn0, locals.var_xil_dn2, locals.var_xil_dn6, locals.var_xil_dn7, locals.var_xil_dn10, locals.var_xil_dn11, locals.var_xil_dn12, locals.var_xil_dn17,)
    }
};
        locals.var_xil = assign23230_e32435;
        locals.var_xil_dn0 = assign23230_e32435_d_n0;
        locals.var_xil_dn2 = assign23230_e32435_d_n2;
        locals.var_xil_dn6 = assign23230_e32435_d_n6;
        locals.var_xil_dn7 = assign23230_e32435_d_n7;
        locals.var_xil_dn10 = assign23230_e32435_d_n10;
        locals.var_xil_dn11 = assign23230_e32435_d_n11;
        locals.var_xil_dn12 = assign23230_e32435_d_n12;
        locals.var_xil_dn17 = assign23230_e32435_d_n17;
        locals.var_xil_rv = 0.0;

        let (assign23240_e32448, assign23240_e32448_d_n0, assign23240_e32448_d_n2, assign23240_e32448_d_n6, assign23240_e32448_d_n7, assign23240_e32448_d_n10, assign23240_e32448_d_n11, assign23240_e32448_d_n12, assign23240_e32448_d_n17,) = {
    if (locals.var_guard716 != 0.0) {
        let assign23240_e32439: f64 = (locals.var_xil * locals.var_xil);
        let assign23240_e32442: f64 = (4.0 * 0.1);
        let assign23240_e32444: f64 = (assign23240_e32442 * 0.1);
        let assign23240_e32445: f64 = (assign23240_e32439 + assign23240_e32444);
        let assign23240_e32446: f64 = (assign23240_e32445).sqrt();
        (assign23240_e32446, (((locals.var_xil_dn0 * locals.var_xil) + (locals.var_xil * locals.var_xil_dn0)) / (2.0 * assign23240_e32446)), (((locals.var_xil_dn2 * locals.var_xil) + (locals.var_xil * locals.var_xil_dn2)) / (2.0 * assign23240_e32446)), (((locals.var_xil_dn6 * locals.var_xil) + (locals.var_xil * locals.var_xil_dn6)) / (2.0 * assign23240_e32446)), (((locals.var_xil_dn7 * locals.var_xil) + (locals.var_xil * locals.var_xil_dn7)) / (2.0 * assign23240_e32446)), (((locals.var_xil_dn10 * locals.var_xil) + (locals.var_xil * locals.var_xil_dn10)) / (2.0 * assign23240_e32446)), (((locals.var_xil_dn11 * locals.var_xil) + (locals.var_xil * locals.var_xil_dn11)) / (2.0 * assign23240_e32446)), (((locals.var_xil_dn12 * locals.var_xil) + (locals.var_xil * locals.var_xil_dn12)) / (2.0 * assign23240_e32446)), (((locals.var_xil_dn17 * locals.var_xil) + (locals.var_xil * locals.var_xil_dn17)) / (2.0 * assign23240_e32446)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign23240_e32448;
        locals.var_tmf1_dn0 = assign23240_e32448_d_n0;
        locals.var_tmf1_dn2 = assign23240_e32448_d_n2;
        locals.var_tmf1_dn6 = assign23240_e32448_d_n6;
        locals.var_tmf1_dn7 = assign23240_e32448_d_n7;
        locals.var_tmf1_dn10 = assign23240_e32448_d_n10;
        locals.var_tmf1_dn11 = assign23240_e32448_d_n11;
        locals.var_tmf1_dn12 = assign23240_e32448_d_n12;
        locals.var_tmf1_dn17 = assign23240_e32448_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign23250_e32460, assign23250_e32460_d_n0, assign23250_e32460_d_n2, assign23250_e32460_d_n6, assign23250_e32460_d_n7, assign23250_e32460_d_n10, assign23250_e32460_d_n11, assign23250_e32460_d_n12, assign23250_e32460_d_n17,) = {
    if (locals.var_guard716 != 0.0) {
        let assign23250_e32453: f64 = (locals.var_xil + locals.var_tmf1);
        let assign23250_e32454: f64 = (0.5 * assign23250_e32453);
        let assign23250_e32457: f64 = (1e-10 * 0.1);
        let assign23250_e32458: f64 = (assign23250_e32454 + assign23250_e32457);
        (assign23250_e32458, (0.5 * (locals.var_xil_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_xil_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_xil_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_xil_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_xil_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_xil_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_xil_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_xil_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_xil, locals.var_xil_dn0, locals.var_xil_dn2, locals.var_xil_dn6, locals.var_xil_dn7, locals.var_xil_dn10, locals.var_xil_dn11, locals.var_xil_dn12, locals.var_xil_dn17,)
    }
};
        locals.var_xil = assign23250_e32460;
        locals.var_xil_dn0 = assign23250_e32460_d_n0;
        locals.var_xil_dn2 = assign23250_e32460_d_n2;
        locals.var_xil_dn6 = assign23250_e32460_d_n6;
        locals.var_xil_dn7 = assign23250_e32460_d_n7;
        locals.var_xil_dn10 = assign23250_e32460_d_n10;
        locals.var_xil_dn11 = assign23250_e32460_d_n11;
        locals.var_xil_dn12 = assign23250_e32460_d_n12;
        locals.var_xil_dn17 = assign23250_e32460_d_n17;
        locals.var_xil_rv = 0.0;

        let assign23260_e32463: f64 = if locals.var_xil < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard719 = assign23260_e32463;
        locals.var_guard719_rv = 0.0;

        let (assign23270_e32469, assign23270_e32469_d_n0, assign23270_e32469_d_n2, assign23270_e32469_d_n6, assign23270_e32469_d_n7, assign23270_e32469_d_n10, assign23270_e32469_d_n11, assign23270_e32469_d_n12, assign23270_e32469_d_n17,) = {
    if ((locals.var_guard716 != 0.0) && (locals.var_guard719 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xil, locals.var_xil_dn0, locals.var_xil_dn2, locals.var_xil_dn6, locals.var_xil_dn7, locals.var_xil_dn10, locals.var_xil_dn11, locals.var_xil_dn12, locals.var_xil_dn17,)
    }
};
        locals.var_xil = assign23270_e32469;
        locals.var_xil_dn0 = assign23270_e32469_d_n0;
        locals.var_xil_dn2 = assign23270_e32469_d_n2;
        locals.var_xil_dn6 = assign23270_e32469_d_n6;
        locals.var_xil_dn7 = assign23270_e32469_d_n7;
        locals.var_xil_dn10 = assign23270_e32469_d_n10;
        locals.var_xil_dn11 = assign23270_e32469_d_n11;
        locals.var_xil_dn12 = assign23270_e32469_d_n12;
        locals.var_xil_dn17 = assign23270_e32469_d_n17;
        locals.var_xil_rv = 0.0;

        let (assign23280_e32474, assign23280_e32474_d_n0, assign23280_e32474_d_n2, assign23280_e32474_d_n6, assign23280_e32474_d_n7, assign23280_e32474_d_n10, assign23280_e32474_d_n11, assign23280_e32474_d_n12, assign23280_e32474_d_n17,) = {
    if (locals.var_guard716 != 0.0) {
        let assign23280_e32472: f64 = (locals.var_xil).sqrt();
        (assign23280_e32472, (locals.var_xil_dn0 / (2.0 * assign23280_e32472)), (locals.var_xil_dn2 / (2.0 * assign23280_e32472)), (locals.var_xil_dn6 / (2.0 * assign23280_e32472)), (locals.var_xil_dn7 / (2.0 * assign23280_e32472)), (locals.var_xil_dn10 / (2.0 * assign23280_e32472)), (locals.var_xil_dn11 / (2.0 * assign23280_e32472)), (locals.var_xil_dn12 / (2.0 * assign23280_e32472)), (locals.var_xil_dn17 / (2.0 * assign23280_e32472)),)
    } else {
        (locals.var_xilp12__blk709, locals.var_xilp12__blk709_dn0, locals.var_xilp12__blk709_dn2, locals.var_xilp12__blk709_dn6, locals.var_xilp12__blk709_dn7, locals.var_xilp12__blk709_dn10, locals.var_xilp12__blk709_dn11, locals.var_xilp12__blk709_dn12, locals.var_xilp12__blk709_dn17,)
    }
};
        locals.var_xilp12__blk709 = assign23280_e32474;
        locals.var_xilp12__blk709_dn0 = assign23280_e32474_d_n0;
        locals.var_xilp12__blk709_dn2 = assign23280_e32474_d_n2;
        locals.var_xilp12__blk709_dn6 = assign23280_e32474_d_n6;
        locals.var_xilp12__blk709_dn7 = assign23280_e32474_d_n7;
        locals.var_xilp12__blk709_dn10 = assign23280_e32474_d_n10;
        locals.var_xilp12__blk709_dn11 = assign23280_e32474_d_n11;
        locals.var_xilp12__blk709_dn12 = assign23280_e32474_d_n12;
        locals.var_xilp12__blk709_dn17 = assign23280_e32474_d_n17;
        locals.var_xilp12__blk709_rv = 0.0;

        let (assign23290_e32480, assign23290_e32480_d_n0, assign23290_e32480_d_n2, assign23290_e32480_d_n6, assign23290_e32480_d_n7, assign23290_e32480_d_n10, assign23290_e32480_d_n11, assign23290_e32480_d_n12, assign23290_e32480_d_n17,) = {
    if (locals.var_guard716 != 0.0) {
        let assign23290_e32478: f64 = (locals.var_xil * locals.var_xilp12__blk709);
        (assign23290_e32478, ((locals.var_xil_dn0 * locals.var_xilp12__blk709) + (locals.var_xil * locals.var_xilp12__blk709_dn0)), ((locals.var_xil_dn2 * locals.var_xilp12__blk709) + (locals.var_xil * locals.var_xilp12__blk709_dn2)), ((locals.var_xil_dn6 * locals.var_xilp12__blk709) + (locals.var_xil * locals.var_xilp12__blk709_dn6)), ((locals.var_xil_dn7 * locals.var_xilp12__blk709) + (locals.var_xil * locals.var_xilp12__blk709_dn7)), ((locals.var_xil_dn10 * locals.var_xilp12__blk709) + (locals.var_xil * locals.var_xilp12__blk709_dn10)), ((locals.var_xil_dn11 * locals.var_xilp12__blk709) + (locals.var_xil * locals.var_xilp12__blk709_dn11)), ((locals.var_xil_dn12 * locals.var_xilp12__blk709) + (locals.var_xil * locals.var_xilp12__blk709_dn12)), ((locals.var_xil_dn17 * locals.var_xilp12__blk709) + (locals.var_xil * locals.var_xilp12__blk709_dn17)),)
    } else {
        (locals.var_xilp32, locals.var_xilp32_dn0, locals.var_xilp32_dn2, locals.var_xilp32_dn6, locals.var_xilp32_dn7, locals.var_xilp32_dn10, locals.var_xilp32_dn11, locals.var_xilp32_dn12, locals.var_xilp32_dn17,)
    }
};
        locals.var_xilp32 = assign23290_e32480;
        locals.var_xilp32_dn0 = assign23290_e32480_d_n0;
        locals.var_xilp32_dn2 = assign23290_e32480_d_n2;
        locals.var_xilp32_dn6 = assign23290_e32480_d_n6;
        locals.var_xilp32_dn7 = assign23290_e32480_d_n7;
        locals.var_xilp32_dn10 = assign23290_e32480_d_n10;
        locals.var_xilp32_dn11 = assign23290_e32480_d_n11;
        locals.var_xilp32_dn12 = assign23290_e32480_d_n12;
        locals.var_xilp32_dn17 = assign23290_e32480_d_n17;
        locals.var_xilp32_rv = 0.0;

        let (assign23300_e32486, assign23300_e32486_d_n0, assign23300_e32486_d_n2, assign23300_e32486_d_n6, assign23300_e32486_d_n7, assign23300_e32486_d_n10, assign23300_e32486_d_n11, assign23300_e32486_d_n12, assign23300_e32486_d_n17,) = {
    if (locals.var_guard716 != 0.0) {
        let assign23300_e32484: f64 = (1.0 / locals.var_xi0__blk705);
        (assign23300_e32484, (-(locals.var_xi0__blk705_dn0 / (locals.var_xi0__blk705 * locals.var_xi0__blk705))), (-(locals.var_xi0__blk705_dn2 / (locals.var_xi0__blk705 * locals.var_xi0__blk705))), (-(locals.var_xi0__blk705_dn6 / (locals.var_xi0__blk705 * locals.var_xi0__blk705))), (-(locals.var_xi0__blk705_dn7 / (locals.var_xi0__blk705 * locals.var_xi0__blk705))), (-(locals.var_xi0__blk705_dn10 / (locals.var_xi0__blk705 * locals.var_xi0__blk705))), (-(locals.var_xi0__blk705_dn11 / (locals.var_xi0__blk705 * locals.var_xi0__blk705))), (-(locals.var_xi0__blk705_dn12 / (locals.var_xi0__blk705 * locals.var_xi0__blk705))), (-(locals.var_xi0__blk705_dn17 / (locals.var_xi0__blk705 * locals.var_xi0__blk705))),)
    } else {
        (locals.var_t10__blk711, locals.var_t10__blk711_dn0, locals.var_t10__blk711_dn2, locals.var_t10__blk711_dn6, locals.var_t10__blk711_dn7, locals.var_t10__blk711_dn10, locals.var_t10__blk711_dn11, locals.var_t10__blk711_dn12, locals.var_t10__blk711_dn17,)
    }
};
        locals.var_t10__blk711 = assign23300_e32486;
        locals.var_t10__blk711_dn0 = assign23300_e32486_d_n0;
        locals.var_t10__blk711_dn2 = assign23300_e32486_d_n2;
        locals.var_t10__blk711_dn6 = assign23300_e32486_d_n6;
        locals.var_t10__blk711_dn7 = assign23300_e32486_d_n7;
        locals.var_t10__blk711_dn10 = assign23300_e32486_d_n10;
        locals.var_t10__blk711_dn11 = assign23300_e32486_d_n11;
        locals.var_t10__blk711_dn12 = assign23300_e32486_d_n12;
        locals.var_t10__blk711_dn17 = assign23300_e32486_d_n17;
        locals.var_t10__blk711_rv = 0.0;

        let (assign23310_e32494, assign23310_e32494_d_n0, assign23310_e32494_d_n2, assign23310_e32494_d_n6, assign23310_e32494_d_n7, assign23310_e32494_d_n10, assign23310_e32494_d_n11, assign23310_e32494_d_n12, assign23310_e32494_d_n17,) = {
    if (locals.var_guard716 != 0.0) {
        let assign23310_e32490: f64 = (locals.var_beta * locals.var_dvbsibpc);
        let assign23310_e32492: f64 = (assign23310_e32490 * locals.var_t10__blk711);
        (assign23310_e32492, (((locals.var_beta * locals.var_dvbsibpc_dn0) * locals.var_t10__blk711) + (assign23310_e32490 * locals.var_t10__blk711_dn0)), (((locals.var_beta * locals.var_dvbsibpc_dn2) * locals.var_t10__blk711) + (assign23310_e32490 * locals.var_t10__blk711_dn2)), (((locals.var_beta * locals.var_dvbsibpc_dn6) * locals.var_t10__blk711) + (assign23310_e32490 * locals.var_t10__blk711_dn6)), (((locals.var_beta * locals.var_dvbsibpc_dn7) * locals.var_t10__blk711) + (assign23310_e32490 * locals.var_t10__blk711_dn7)), ((((locals.var_beta_dn10 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn10)) * locals.var_t10__blk711) + (assign23310_e32490 * locals.var_t10__blk711_dn10)), (((locals.var_beta * locals.var_dvbsibpc_dn11) * locals.var_t10__blk711) + (assign23310_e32490 * locals.var_t10__blk711_dn11)), (((locals.var_beta * locals.var_dvbsibpc_dn12) * locals.var_t10__blk711) + (assign23310_e32490 * locals.var_t10__blk711_dn12)), (((locals.var_beta * locals.var_dvbsibpc_dn17) * locals.var_t10__blk711) + (assign23310_e32490 * locals.var_t10__blk711_dn17)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign23310_e32494;
        locals.var_t1_dn0 = assign23310_e32494_d_n0;
        locals.var_t1_dn2 = assign23310_e32494_d_n2;
        locals.var_t1_dn6 = assign23310_e32494_d_n6;
        locals.var_t1_dn7 = assign23310_e32494_d_n7;
        locals.var_t1_dn10 = assign23310_e32494_d_n10;
        locals.var_t1_dn11 = assign23310_e32494_d_n11;
        locals.var_t1_dn12 = assign23310_e32494_d_n12;
        locals.var_t1_dn17 = assign23310_e32494_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign23320_e32500, assign23320_e32500_d_n0, assign23320_e32500_d_n2, assign23320_e32500_d_n6, assign23320_e32500_d_n7, assign23320_e32500_d_n10, assign23320_e32500_d_n11, assign23320_e32500_d_n12, assign23320_e32500_d_n17,) = {
    if (locals.var_guard716 != 0.0) {
        let assign23320_e32498: f64 = (1.0 / locals.var_xil);
        (assign23320_e32498, (-(locals.var_xil_dn0 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn2 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn6 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn7 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn10 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn11 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn12 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn17 / (locals.var_xil * locals.var_xil))),)
    } else {
        (locals.var_t10__blk711, locals.var_t10__blk711_dn0, locals.var_t10__blk711_dn2, locals.var_t10__blk711_dn6, locals.var_t10__blk711_dn7, locals.var_t10__blk711_dn10, locals.var_t10__blk711_dn11, locals.var_t10__blk711_dn12, locals.var_t10__blk711_dn17,)
    }
};
        locals.var_t10__blk711 = assign23320_e32500;
        locals.var_t10__blk711_dn0 = assign23320_e32500_d_n0;
        locals.var_t10__blk711_dn2 = assign23320_e32500_d_n2;
        locals.var_t10__blk711_dn6 = assign23320_e32500_d_n6;
        locals.var_t10__blk711_dn7 = assign23320_e32500_d_n7;
        locals.var_t10__blk711_dn10 = assign23320_e32500_d_n10;
        locals.var_t10__blk711_dn11 = assign23320_e32500_d_n11;
        locals.var_t10__blk711_dn12 = assign23320_e32500_d_n12;
        locals.var_t10__blk711_dn17 = assign23320_e32500_d_n17;
        locals.var_t10__blk711_rv = 0.0;

        let (assign23330_e32508, assign23330_e32508_d_n0, assign23330_e32508_d_n2, assign23330_e32508_d_n6, assign23330_e32508_d_n7, assign23330_e32508_d_n10, assign23330_e32508_d_n11, assign23330_e32508_d_n12, assign23330_e32508_d_n17,) = {
    if (locals.var_guard716 != 0.0) {
        let assign23330_e32504: f64 = (locals.var_beta * locals.var_dvbsibpc);
        let assign23330_e32506: f64 = (assign23330_e32504 * locals.var_t10__blk711);
        (assign23330_e32506, (((locals.var_beta * locals.var_dvbsibpc_dn0) * locals.var_t10__blk711) + (assign23330_e32504 * locals.var_t10__blk711_dn0)), (((locals.var_beta * locals.var_dvbsibpc_dn2) * locals.var_t10__blk711) + (assign23330_e32504 * locals.var_t10__blk711_dn2)), (((locals.var_beta * locals.var_dvbsibpc_dn6) * locals.var_t10__blk711) + (assign23330_e32504 * locals.var_t10__blk711_dn6)), (((locals.var_beta * locals.var_dvbsibpc_dn7) * locals.var_t10__blk711) + (assign23330_e32504 * locals.var_t10__blk711_dn7)), ((((locals.var_beta_dn10 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn10)) * locals.var_t10__blk711) + (assign23330_e32504 * locals.var_t10__blk711_dn10)), (((locals.var_beta * locals.var_dvbsibpc_dn11) * locals.var_t10__blk711) + (assign23330_e32504 * locals.var_t10__blk711_dn11)), (((locals.var_beta * locals.var_dvbsibpc_dn12) * locals.var_t10__blk711) + (assign23330_e32504 * locals.var_t10__blk711_dn12)), (((locals.var_beta * locals.var_dvbsibpc_dn17) * locals.var_t10__blk711) + (assign23330_e32504 * locals.var_t10__blk711_dn17)),)
    } else {
        (locals.var_t2__blk712, locals.var_t2__blk712_dn0, locals.var_t2__blk712_dn2, locals.var_t2__blk712_dn6, locals.var_t2__blk712_dn7, locals.var_t2__blk712_dn10, locals.var_t2__blk712_dn11, locals.var_t2__blk712_dn12, locals.var_t2__blk712_dn17,)
    }
};
        locals.var_t2__blk712 = assign23330_e32508;
        locals.var_t2__blk712_dn0 = assign23330_e32508_d_n0;
        locals.var_t2__blk712_dn2 = assign23330_e32508_d_n2;
        locals.var_t2__blk712_dn6 = assign23330_e32508_d_n6;
        locals.var_t2__blk712_dn7 = assign23330_e32508_d_n7;
        locals.var_t2__blk712_dn10 = assign23330_e32508_d_n10;
        locals.var_t2__blk712_dn11 = assign23330_e32508_d_n11;
        locals.var_t2__blk712_dn12 = assign23330_e32508_d_n12;
        locals.var_t2__blk712_dn17 = assign23330_e32508_d_n17;
        locals.var_t2__blk712_rv = 0.0;

        let (assign23340_e32520, assign23340_e32520_d_n0, assign23340_e32520_d_n2, assign23340_e32520_d_n6, assign23340_e32520_d_n7, assign23340_e32520_d_n10, assign23340_e32520_d_n11, assign23340_e32520_d_n12, assign23340_e32520_d_n17,) = {
    if (locals.var_guard716 != 0.0) {
        let assign23340_e32513: f64 = (locals.var_xilp32 * locals.var_t2__blk712);
        let assign23340_e32516: f64 = (locals.var_xi0p32 * locals.var_t1);
        let assign23340_e32517: f64 = (assign23340_e32513 - assign23340_e32516);
        let assign23340_e32518: f64 = (locals.var_cnst0soi * assign23340_e32517);
        (assign23340_e32518, ((locals.var_cnst0soi_dn0 * assign23340_e32517) + (locals.var_cnst0soi * (((locals.var_xilp32_dn0 * locals.var_t2__blk712) + (locals.var_xilp32 * locals.var_t2__blk712_dn0)) - ((locals.var_xi0p32_dn0 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn0))))), ((locals.var_cnst0soi_dn2 * assign23340_e32517) + (locals.var_cnst0soi * (((locals.var_xilp32_dn2 * locals.var_t2__blk712) + (locals.var_xilp32 * locals.var_t2__blk712_dn2)) - ((locals.var_xi0p32_dn2 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn2))))), ((locals.var_cnst0soi_dn6 * assign23340_e32517) + (locals.var_cnst0soi * (((locals.var_xilp32_dn6 * locals.var_t2__blk712) + (locals.var_xilp32 * locals.var_t2__blk712_dn6)) - ((locals.var_xi0p32_dn6 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn6))))), ((locals.var_cnst0soi_dn7 * assign23340_e32517) + (locals.var_cnst0soi * (((locals.var_xilp32_dn7 * locals.var_t2__blk712) + (locals.var_xilp32 * locals.var_t2__blk712_dn7)) - ((locals.var_xi0p32_dn7 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn7))))), ((locals.var_cnst0soi_dn10 * assign23340_e32517) + (locals.var_cnst0soi * (((locals.var_xilp32_dn10 * locals.var_t2__blk712) + (locals.var_xilp32 * locals.var_t2__blk712_dn10)) - ((locals.var_xi0p32_dn10 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn10))))), ((locals.var_cnst0soi_dn11 * assign23340_e32517) + (locals.var_cnst0soi * (((locals.var_xilp32_dn11 * locals.var_t2__blk712) + (locals.var_xilp32 * locals.var_t2__blk712_dn11)) - ((locals.var_xi0p32_dn11 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn11))))), ((locals.var_cnst0soi_dn12 * assign23340_e32517) + (locals.var_cnst0soi * (((locals.var_xilp32_dn12 * locals.var_t2__blk712) + (locals.var_xilp32 * locals.var_t2__blk712_dn12)) - ((locals.var_xi0p32_dn12 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn12))))), ((locals.var_cnst0soi_dn17 * assign23340_e32517) + (locals.var_cnst0soi * (((locals.var_xilp32_dn17 * locals.var_t2__blk712) + (locals.var_xilp32 * locals.var_t2__blk712_dn17)) - ((locals.var_xi0p32_dn17 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn17))))),)
    } else {
        (locals.var_dg3, locals.var_dg3_dn0, locals.var_dg3_dn2, locals.var_dg3_dn6, locals.var_dg3_dn7, locals.var_dg3_dn10, locals.var_dg3_dn11, locals.var_dg3_dn12, locals.var_dg3_dn17,)
    }
};
        locals.var_dg3 = assign23340_e32520;
        locals.var_dg3_dn0 = assign23340_e32520_d_n0;
        locals.var_dg3_dn2 = assign23340_e32520_d_n2;
        locals.var_dg3_dn6 = assign23340_e32520_d_n6;
        locals.var_dg3_dn7 = assign23340_e32520_d_n7;
        locals.var_dg3_dn10 = assign23340_e32520_d_n10;
        locals.var_dg3_dn11 = assign23340_e32520_d_n11;
        locals.var_dg3_dn12 = assign23340_e32520_d_n12;
        locals.var_dg3_dn17 = assign23340_e32520_d_n17;
        locals.var_dg3_rv = 0.0;

        let (assign23350_e32535, assign23350_e32535_d_n0, assign23350_e32535_d_n2, assign23350_e32535_d_n6, assign23350_e32535_d_n7, assign23350_e32535_d_n10, assign23350_e32535_d_n11, assign23350_e32535_d_n12, assign23350_e32535_d_n17,) = {
    if (locals.var_guard716 != 0.0) {
        let assign23350_e32524: f64 = (locals.var_cnst0soi * 0.5);
        let assign23350_e32526: f64 = (-locals.var_xilp12__blk709);
        let assign23350_e32528: f64 = (assign23350_e32526 * locals.var_t2__blk712);
        let assign23350_e32531: f64 = (locals.var_xi0p12__blk706 * locals.var_t1);
        let assign23350_e32532: f64 = (assign23350_e32528 + assign23350_e32531);
        let assign23350_e32533: f64 = (assign23350_e32524 * assign23350_e32532);
        (assign23350_e32533, (((locals.var_cnst0soi_dn0 * 0.5) * assign23350_e32532) + (assign23350_e32524 * ((((-locals.var_xilp12__blk709_dn0) * locals.var_t2__blk712) + (assign23350_e32526 * locals.var_t2__blk712_dn0)) + ((locals.var_xi0p12__blk706_dn0 * locals.var_t1) + (locals.var_xi0p12__blk706 * locals.var_t1_dn0))))), (((locals.var_cnst0soi_dn2 * 0.5) * assign23350_e32532) + (assign23350_e32524 * ((((-locals.var_xilp12__blk709_dn2) * locals.var_t2__blk712) + (assign23350_e32526 * locals.var_t2__blk712_dn2)) + ((locals.var_xi0p12__blk706_dn2 * locals.var_t1) + (locals.var_xi0p12__blk706 * locals.var_t1_dn2))))), (((locals.var_cnst0soi_dn6 * 0.5) * assign23350_e32532) + (assign23350_e32524 * ((((-locals.var_xilp12__blk709_dn6) * locals.var_t2__blk712) + (assign23350_e32526 * locals.var_t2__blk712_dn6)) + ((locals.var_xi0p12__blk706_dn6 * locals.var_t1) + (locals.var_xi0p12__blk706 * locals.var_t1_dn6))))), (((locals.var_cnst0soi_dn7 * 0.5) * assign23350_e32532) + (assign23350_e32524 * ((((-locals.var_xilp12__blk709_dn7) * locals.var_t2__blk712) + (assign23350_e32526 * locals.var_t2__blk712_dn7)) + ((locals.var_xi0p12__blk706_dn7 * locals.var_t1) + (locals.var_xi0p12__blk706 * locals.var_t1_dn7))))), (((locals.var_cnst0soi_dn10 * 0.5) * assign23350_e32532) + (assign23350_e32524 * ((((-locals.var_xilp12__blk709_dn10) * locals.var_t2__blk712) + (assign23350_e32526 * locals.var_t2__blk712_dn10)) + ((locals.var_xi0p12__blk706_dn10 * locals.var_t1) + (locals.var_xi0p12__blk706 * locals.var_t1_dn10))))), (((locals.var_cnst0soi_dn11 * 0.5) * assign23350_e32532) + (assign23350_e32524 * ((((-locals.var_xilp12__blk709_dn11) * locals.var_t2__blk712) + (assign23350_e32526 * locals.var_t2__blk712_dn11)) + ((locals.var_xi0p12__blk706_dn11 * locals.var_t1) + (locals.var_xi0p12__blk706 * locals.var_t1_dn11))))), (((locals.var_cnst0soi_dn12 * 0.5) * assign23350_e32532) + (assign23350_e32524 * ((((-locals.var_xilp12__blk709_dn12) * locals.var_t2__blk712) + (assign23350_e32526 * locals.var_t2__blk712_dn12)) + ((locals.var_xi0p12__blk706_dn12 * locals.var_t1) + (locals.var_xi0p12__blk706 * locals.var_t1_dn12))))), (((locals.var_cnst0soi_dn17 * 0.5) * assign23350_e32532) + (assign23350_e32524 * ((((-locals.var_xilp12__blk709_dn17) * locals.var_t2__blk712) + (assign23350_e32526 * locals.var_t2__blk712_dn17)) + ((locals.var_xi0p12__blk706_dn17 * locals.var_t1) + (locals.var_xi0p12__blk706 * locals.var_t1_dn17))))),)
    } else {
        (locals.var_dg4, locals.var_dg4_dn0, locals.var_dg4_dn2, locals.var_dg4_dn6, locals.var_dg4_dn7, locals.var_dg4_dn10, locals.var_dg4_dn11, locals.var_dg4_dn12, locals.var_dg4_dn17,)
    }
};
        locals.var_dg4 = assign23350_e32535;
        locals.var_dg4_dn0 = assign23350_e32535_d_n0;
        locals.var_dg4_dn2 = assign23350_e32535_d_n2;
        locals.var_dg4_dn6 = assign23350_e32535_d_n6;
        locals.var_dg4_dn7 = assign23350_e32535_d_n7;
        locals.var_dg4_dn10 = assign23350_e32535_d_n10;
        locals.var_dg4_dn11 = assign23350_e32535_d_n11;
        locals.var_dg4_dn12 = assign23350_e32535_d_n12;
        locals.var_dg4_dn17 = assign23350_e32535_d_n17;
        locals.var_dg4_rv = 0.0;

        let (assign23360_e32541, assign23360_e32541_d_n0, assign23360_e32541_d_n2, assign23360_e32541_d_n6, assign23360_e32541_d_n7, assign23360_e32541_d_n10, assign23360_e32541_d_n11, assign23360_e32541_d_n12, assign23360_e32541_d_n17,) = {
    if (locals.var_guard716 != 0.0) {
        let assign23360_e32539: f64 = (locals.var_dg3 + locals.var_dg4);
        (assign23360_e32539, (locals.var_dg3_dn0 + locals.var_dg4_dn0), (locals.var_dg3_dn2 + locals.var_dg4_dn2), (locals.var_dg3_dn6 + locals.var_dg4_dn6), (locals.var_dg3_dn7 + locals.var_dg4_dn7), (locals.var_dg3_dn10 + locals.var_dg4_dn10), (locals.var_dg3_dn11 + locals.var_dg4_dn11), (locals.var_dg3_dn12 + locals.var_dg4_dn12), (locals.var_dg3_dn17 + locals.var_dg4_dn17),)
    } else {
        (locals.var_didd, locals.var_didd_dn0, locals.var_didd_dn2, locals.var_didd_dn6, locals.var_didd_dn7, locals.var_didd_dn10, locals.var_didd_dn11, locals.var_didd_dn12, locals.var_didd_dn17,)
    }
};
        locals.var_didd = assign23360_e32541;
        locals.var_didd_dn0 = assign23360_e32541_d_n0;
        locals.var_didd_dn2 = assign23360_e32541_d_n2;
        locals.var_didd_dn6 = assign23360_e32541_d_n6;
        locals.var_didd_dn7 = assign23360_e32541_d_n7;
        locals.var_didd_dn10 = assign23360_e32541_d_n10;
        locals.var_didd_dn11 = assign23360_e32541_d_n11;
        locals.var_didd_dn12 = assign23360_e32541_d_n12;
        locals.var_didd_dn17 = assign23360_e32541_d_n17;
        locals.var_didd_rv = 0.0;

        let (assign23370_e32549, assign23370_e32549_d_n0, assign23370_e32549_d_n2, assign23370_e32549_d_n6, assign23370_e32549_d_n7, assign23370_e32549_d_n10, assign23370_e32549_d_n11, assign23370_e32549_d_n12, assign23370_e32549_d_n17,) = {
    if (locals.var_guard716 != 0.0) {
        let assign23370_e32545: f64 = (locals.var_betawl * locals.var_didd);
        let assign23370_e32547: f64 = (assign23370_e32545 * locals.var_mu);
        (assign23370_e32547, ((((locals.var_betawl_dn0 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn0)) * locals.var_mu) + (assign23370_e32545 * locals.var_mu_dn0)), ((((locals.var_betawl_dn2 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn2)) * locals.var_mu) + (assign23370_e32545 * locals.var_mu_dn2)), ((((locals.var_betawl_dn6 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn6)) * locals.var_mu) + (assign23370_e32545 * locals.var_mu_dn6)), ((((locals.var_betawl_dn7 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn7)) * locals.var_mu) + (assign23370_e32545 * locals.var_mu_dn7)), ((((locals.var_betawl_dn10 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn10)) * locals.var_mu) + (assign23370_e32545 * locals.var_mu_dn10)), ((((locals.var_betawl_dn11 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn11)) * locals.var_mu) + (assign23370_e32545 * locals.var_mu_dn11)), ((((locals.var_betawl_dn12 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn12)) * locals.var_mu) + (assign23370_e32545 * locals.var_mu_dn12)), ((((locals.var_betawl_dn17 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn17)) * locals.var_mu) + (assign23370_e32545 * locals.var_mu_dn17)),)
    } else {
        (locals.var_idsibpc, locals.var_idsibpc_dn0, locals.var_idsibpc_dn2, locals.var_idsibpc_dn6, locals.var_idsibpc_dn7, locals.var_idsibpc_dn10, locals.var_idsibpc_dn11, locals.var_idsibpc_dn12, locals.var_idsibpc_dn17,)
    }
};
        locals.var_idsibpc = assign23370_e32549;
        locals.var_idsibpc_dn0 = assign23370_e32549_d_n0;
        locals.var_idsibpc_dn2 = assign23370_e32549_d_n2;
        locals.var_idsibpc_dn6 = assign23370_e32549_d_n6;
        locals.var_idsibpc_dn7 = assign23370_e32549_d_n7;
        locals.var_idsibpc_dn10 = assign23370_e32549_d_n10;
        locals.var_idsibpc_dn11 = assign23370_e32549_d_n11;
        locals.var_idsibpc_dn12 = assign23370_e32549_d_n12;
        locals.var_idsibpc_dn17 = assign23370_e32549_d_n17;
        locals.var_idsibpc_rv = 0.0;

        let assign23380_e32552: f64 = (locals.var_tfox0 * 100.0);
        locals.var_cgs_tfox0__blk733 = assign23380_e32552;
        locals.var_cgs_tfox0__blk733_rv = 0.0;

        let assign23390_e32555: f64 = (locals.var_c_fox / 10000.0);
        locals.var_cgs_c_fox = assign23390_e32555;
        locals.var_cgs_c_fox_dn0 = (locals.var_c_fox_dn0 / 10000.0);
        locals.var_cgs_c_fox_dn2 = (locals.var_c_fox_dn2 / 10000.0);
        locals.var_cgs_c_fox_dn6 = (locals.var_c_fox_dn6 / 10000.0);
        locals.var_cgs_c_fox_dn7 = (locals.var_c_fox_dn7 / 10000.0);
        locals.var_cgs_c_fox_dn10 = (locals.var_c_fox_dn10 / 10000.0);
        locals.var_cgs_c_fox_dn11 = (locals.var_c_fox_dn11 / 10000.0);
        locals.var_cgs_c_fox_dn12 = (locals.var_c_fox_dn12 / 10000.0);
        locals.var_cgs_c_fox_dn17 = (locals.var_c_fox_dn17 / 10000.0);
        locals.var_cgs_c_fox_rv = 0.0;

        let assign23400_e32558: f64 = (locals.var_leff * 100.0);
        locals.var_cgs_leff__blk735 = assign23400_e32558;
        locals.var_cgs_leff__blk735_rv = 0.0;

        let assign23410_e32561: f64 = (locals.var_weff_nf * 100.0);
        locals.var_cgs_weff_nf__blk736 = assign23410_e32561;
        locals.var_cgs_weff_nf__blk736_rv = 0.0;

        let assign23420_e32564: f64 = (locals.var_ey / 100.0);
        locals.var_cgs_ey = assign23420_e32564;
        locals.var_cgs_ey_dn0 = (locals.var_ey_dn0 / 100.0);
        locals.var_cgs_ey_dn2 = (locals.var_ey_dn2 / 100.0);
        locals.var_cgs_ey_dn6 = (locals.var_ey_dn6 / 100.0);
        locals.var_cgs_ey_dn7 = (locals.var_ey_dn7 / 100.0);
        locals.var_cgs_ey_dn10 = (locals.var_ey_dn10 / 100.0);
        locals.var_cgs_ey_dn11 = (locals.var_ey_dn11 / 100.0);
        locals.var_cgs_ey_dn12 = (locals.var_ey_dn12 / 100.0);
        locals.var_cgs_ey_dn17 = (locals.var_ey_dn17 / 100.0);
        locals.var_cgs_ey_rv = 0.0;

        let assign23430_e32567: f64 = (locals.var_qiu / 10000.0);
        locals.var_cgs_qiu__blk738 = assign23430_e32567;
        locals.var_cgs_qiu__blk738_dn0 = (locals.var_qiu_dn0 / 10000.0);
        locals.var_cgs_qiu__blk738_dn2 = (locals.var_qiu_dn2 / 10000.0);
        locals.var_cgs_qiu__blk738_dn6 = (locals.var_qiu_dn6 / 10000.0);
        locals.var_cgs_qiu__blk738_dn7 = (locals.var_qiu_dn7 / 10000.0);
        locals.var_cgs_qiu__blk738_dn10 = (locals.var_qiu_dn10 / 10000.0);
        locals.var_cgs_qiu__blk738_dn11 = (locals.var_qiu_dn11 / 10000.0);
        locals.var_cgs_qiu__blk738_dn12 = (locals.var_qiu_dn12 / 10000.0);
        locals.var_cgs_qiu__blk738_dn17 = (locals.var_qiu_dn17 / 10000.0);
        locals.var_cgs_qiu__blk738_rv = 0.0;

        let assign23440_e32570: f64 = (locals.var_cnst0soi / 10000.0);
        locals.var_cgs_cnst0soi = assign23440_e32570;
        locals.var_cgs_cnst0soi_dn0 = (locals.var_cnst0soi_dn0 / 10000.0);
        locals.var_cgs_cnst0soi_dn2 = (locals.var_cnst0soi_dn2 / 10000.0);
        locals.var_cgs_cnst0soi_dn6 = (locals.var_cnst0soi_dn6 / 10000.0);
        locals.var_cgs_cnst0soi_dn7 = (locals.var_cnst0soi_dn7 / 10000.0);
        locals.var_cgs_cnst0soi_dn10 = (locals.var_cnst0soi_dn10 / 10000.0);
        locals.var_cgs_cnst0soi_dn11 = (locals.var_cnst0soi_dn11 / 10000.0);
        locals.var_cgs_cnst0soi_dn12 = (locals.var_cnst0soi_dn12 / 10000.0);
        locals.var_cgs_cnst0soi_dn17 = (locals.var_cnst0soi_dn17 / 10000.0);
        locals.var_cgs_cnst0soi_rv = 0.0;

        let assign23450_e32573: f64 = if p.p27 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard740 = assign23450_e32573;
        locals.var_guard740_rv = 0.0;

        let assign23510_e32596: f64 = if locals.var_flg_noqi == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard741 = assign23510_e32596;
        locals.var_guard741_rv = 0.0;

        let (assign23520_e32609, assign23520_e32609_d_n0, assign23520_e32609_d_n2, assign23520_e32609_d_n6, assign23520_e32609_d_n7, assign23520_e32609_d_n10, assign23520_e32609_d_n11, assign23520_e32609_d_n12, assign23520_e32609_d_n17,) = {
    if ((locals.var_guard740 == 0.0) && (locals.var_guard741 != 0.0)) {
        let assign23520_e32603: f64 = (locals.var_ps0z + locals.var_vdsz);
        let assign23520_e32606: f64 = (10.0 * 2.220446049250313e-16);
        let assign23520_e32607: f64 = (assign23520_e32603 - assign23520_e32606);
        (assign23520_e32607, (locals.var_ps0z_dn0 + locals.var_vdsz_dn0), (locals.var_ps0z_dn2 + locals.var_vdsz_dn2), (locals.var_ps0z_dn6 + locals.var_vdsz_dn6), (locals.var_ps0z_dn7 + locals.var_vdsz_dn7), (locals.var_ps0z_dn10 + locals.var_vdsz_dn10), (locals.var_ps0z_dn11 + locals.var_vdsz_dn11), (locals.var_ps0z_dn12 + locals.var_vdsz_dn12), (locals.var_ps0z_dn17 + locals.var_vdsz_dn17),)
    } else {
        (locals.var_psdlz, locals.var_psdlz_dn0, locals.var_psdlz_dn2, locals.var_psdlz_dn6, locals.var_psdlz_dn7, locals.var_psdlz_dn10, locals.var_psdlz_dn11, locals.var_psdlz_dn12, locals.var_psdlz_dn17,)
    }
};
        locals.var_psdlz = assign23520_e32609;
        locals.var_psdlz_dn0 = assign23520_e32609_d_n0;
        locals.var_psdlz_dn2 = assign23520_e32609_d_n2;
        locals.var_psdlz_dn6 = assign23520_e32609_d_n6;
        locals.var_psdlz_dn7 = assign23520_e32609_d_n7;
        locals.var_psdlz_dn10 = assign23520_e32609_d_n10;
        locals.var_psdlz_dn11 = assign23520_e32609_d_n11;
        locals.var_psdlz_dn12 = assign23520_e32609_d_n12;
        locals.var_psdlz_dn17 = assign23520_e32609_d_n17;
        locals.var_psdlz_rv = 0.0;

        let (assign23530_e32630, assign23530_e32630_d_n0, assign23530_e32630_d_n2, assign23530_e32630_d_n6, assign23530_e32630_d_n7, assign23530_e32630_d_n10, assign23530_e32630_d_n11, assign23530_e32630_d_n12, assign23530_e32630_d_n17,) = {
    if ((locals.var_guard740 == 0.0) && (locals.var_guard741 != 0.0)) {
        let assign23530_e32616: f64 = (locals.var_vgsz - locals.var_vfb);
        let assign23530_e32620: f64 = (locals.var_dvth - locals.var_dppg);
        let assign23530_e32621: f64 = (p.p216 * assign23530_e32620);
        let assign23530_e32623: f64 = (assign23530_e32621 * locals.var_cgs_leff__blk735);
        let assign23530_e32624: f64 = (assign23530_e32616 + assign23530_e32623);
        let assign23530_e32627: f64 = (locals.var_psdlz * p.p215);
        let assign23530_e32628: f64 = (assign23530_e32624 - assign23530_e32627);
        (assign23530_e32628, ((locals.var_vgsz_dn0 + ((p.p216 * (locals.var_dvth_dn0 - locals.var_dppg_dn0)) * locals.var_cgs_leff__blk735)) - (locals.var_psdlz_dn0 * p.p215)), ((locals.var_vgsz_dn2 + ((p.p216 * (locals.var_dvth_dn2 - locals.var_dppg_dn2)) * locals.var_cgs_leff__blk735)) - (locals.var_psdlz_dn2 * p.p215)), ((locals.var_vgsz_dn6 + ((p.p216 * (locals.var_dvth_dn6 - locals.var_dppg_dn6)) * locals.var_cgs_leff__blk735)) - (locals.var_psdlz_dn6 * p.p215)), ((locals.var_vgsz_dn7 + ((p.p216 * (locals.var_dvth_dn7 - locals.var_dppg_dn7)) * locals.var_cgs_leff__blk735)) - (locals.var_psdlz_dn7 * p.p215)), ((locals.var_vgsz_dn10 + ((p.p216 * (locals.var_dvth_dn10 - locals.var_dppg_dn10)) * locals.var_cgs_leff__blk735)) - (locals.var_psdlz_dn10 * p.p215)), ((locals.var_vgsz_dn11 + ((p.p216 * (locals.var_dvth_dn11 - locals.var_dppg_dn11)) * locals.var_cgs_leff__blk735)) - (locals.var_psdlz_dn11 * p.p215)), ((locals.var_vgsz_dn12 + ((p.p216 * (locals.var_dvth_dn12 - locals.var_dppg_dn12)) * locals.var_cgs_leff__blk735)) - (locals.var_psdlz_dn12 * p.p215)), ((locals.var_vgsz_dn17 + ((p.p216 * (locals.var_dvth_dn17 - locals.var_dppg_dn17)) * locals.var_cgs_leff__blk735)) - (locals.var_psdlz_dn17 * p.p215)),)
    } else {
        (locals.var_t1__blk722, locals.var_t1__blk722_dn0, locals.var_t1__blk722_dn2, locals.var_t1__blk722_dn6, locals.var_t1__blk722_dn7, locals.var_t1__blk722_dn10, locals.var_t1__blk722_dn11, locals.var_t1__blk722_dn12, locals.var_t1__blk722_dn17,)
    }
};
        locals.var_t1__blk722 = assign23530_e32630;
        locals.var_t1__blk722_dn0 = assign23530_e32630_d_n0;
        locals.var_t1__blk722_dn2 = assign23530_e32630_d_n2;
        locals.var_t1__blk722_dn6 = assign23530_e32630_d_n6;
        locals.var_t1__blk722_dn7 = assign23530_e32630_d_n7;
        locals.var_t1__blk722_dn10 = assign23530_e32630_d_n10;
        locals.var_t1__blk722_dn11 = assign23530_e32630_d_n11;
        locals.var_t1__blk722_dn12 = assign23530_e32630_d_n12;
        locals.var_t1__blk722_dn17 = assign23530_e32630_d_n17;
        locals.var_t1__blk722_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_84(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign23540_e32639, assign23540_e32639_d_n0, assign23540_e32639_d_n2, assign23540_e32639_d_n6, assign23540_e32639_d_n7, assign23540_e32639_d_n10, assign23540_e32639_d_n11, assign23540_e32639_d_n12, assign23540_e32639_d_n17,) = {
    if ((locals.var_guard740 == 0.0) && (locals.var_guard741 != 0.0)) {
        let assign23540_e32637: f64 = (1.0 / locals.var_cgs_tfox0__blk733);
        (assign23540_e32637, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3__blk724, locals.var_t3__blk724_dn0, locals.var_t3__blk724_dn2, locals.var_t3__blk724_dn6, locals.var_t3__blk724_dn7, locals.var_t3__blk724_dn10, locals.var_t3__blk724_dn11, locals.var_t3__blk724_dn12, locals.var_t3__blk724_dn17,)
    }
};
        locals.var_t3__blk724 = assign23540_e32639;
        locals.var_t3__blk724_dn0 = assign23540_e32639_d_n0;
        locals.var_t3__blk724_dn2 = assign23540_e32639_d_n2;
        locals.var_t3__blk724_dn6 = assign23540_e32639_d_n6;
        locals.var_t3__blk724_dn7 = assign23540_e32639_d_n7;
        locals.var_t3__blk724_dn10 = assign23540_e32639_d_n10;
        locals.var_t3__blk724_dn11 = assign23540_e32639_d_n11;
        locals.var_t3__blk724_dn12 = assign23540_e32639_d_n12;
        locals.var_t3__blk724_dn17 = assign23540_e32639_d_n17;
        locals.var_t3__blk724_rv = 0.0;

        let (assign23550_e32648, assign23550_e32648_d_n0, assign23550_e32648_d_n2, assign23550_e32648_d_n6, assign23550_e32648_d_n7, assign23550_e32648_d_n10, assign23550_e32648_d_n11, assign23550_e32648_d_n12, assign23550_e32648_d_n17,) = {
    if ((locals.var_guard740 == 0.0) && (locals.var_guard741 != 0.0)) {
        let assign23550_e32646: f64 = (locals.var_t1__blk722 * locals.var_t3__blk724);
        (assign23550_e32646, ((locals.var_t1__blk722_dn0 * locals.var_t3__blk724) + (locals.var_t1__blk722 * locals.var_t3__blk724_dn0)), ((locals.var_t1__blk722_dn2 * locals.var_t3__blk724) + (locals.var_t1__blk722 * locals.var_t3__blk724_dn2)), ((locals.var_t1__blk722_dn6 * locals.var_t3__blk724) + (locals.var_t1__blk722 * locals.var_t3__blk724_dn6)), ((locals.var_t1__blk722_dn7 * locals.var_t3__blk724) + (locals.var_t1__blk722 * locals.var_t3__blk724_dn7)), ((locals.var_t1__blk722_dn10 * locals.var_t3__blk724) + (locals.var_t1__blk722 * locals.var_t3__blk724_dn10)), ((locals.var_t1__blk722_dn11 * locals.var_t3__blk724) + (locals.var_t1__blk722 * locals.var_t3__blk724_dn11)), ((locals.var_t1__blk722_dn12 * locals.var_t3__blk724) + (locals.var_t1__blk722 * locals.var_t3__blk724_dn12)), ((locals.var_t1__blk722_dn17 * locals.var_t3__blk724) + (locals.var_t1__blk722 * locals.var_t3__blk724_dn17)),)
    } else {
        (locals.var_t2__blk723, locals.var_t2__blk723_dn0, locals.var_t2__blk723_dn2, locals.var_t2__blk723_dn6, locals.var_t2__blk723_dn7, locals.var_t2__blk723_dn10, locals.var_t2__blk723_dn11, locals.var_t2__blk723_dn12, locals.var_t2__blk723_dn17,)
    }
};
        locals.var_t2__blk723 = assign23550_e32648;
        locals.var_t2__blk723_dn0 = assign23550_e32648_d_n0;
        locals.var_t2__blk723_dn2 = assign23550_e32648_d_n2;
        locals.var_t2__blk723_dn6 = assign23550_e32648_d_n6;
        locals.var_t2__blk723_dn7 = assign23550_e32648_d_n7;
        locals.var_t2__blk723_dn10 = assign23550_e32648_d_n10;
        locals.var_t2__blk723_dn11 = assign23550_e32648_d_n11;
        locals.var_t2__blk723_dn12 = assign23550_e32648_d_n12;
        locals.var_t2__blk723_dn17 = assign23550_e32648_d_n17;
        locals.var_t2__blk723_rv = 0.0;

        let (assign23560_e32657, assign23560_e32657_d_n0, assign23560_e32657_d_n2, assign23560_e32657_d_n6, assign23560_e32657_d_n7, assign23560_e32657_d_n10, assign23560_e32657_d_n11, assign23560_e32657_d_n12, assign23560_e32657_d_n17,) = {
    if ((locals.var_guard740 == 0.0) && (locals.var_guard741 != 0.0)) {
        let assign23560_e32655: f64 = (1.0 / p.p217);
        (assign23560_e32655, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3__blk724, locals.var_t3__blk724_dn0, locals.var_t3__blk724_dn2, locals.var_t3__blk724_dn6, locals.var_t3__blk724_dn7, locals.var_t3__blk724_dn10, locals.var_t3__blk724_dn11, locals.var_t3__blk724_dn12, locals.var_t3__blk724_dn17,)
    }
};
        locals.var_t3__blk724 = assign23560_e32657;
        locals.var_t3__blk724_dn0 = assign23560_e32657_d_n0;
        locals.var_t3__blk724_dn2 = assign23560_e32657_d_n2;
        locals.var_t3__blk724_dn6 = assign23560_e32657_d_n6;
        locals.var_t3__blk724_dn7 = assign23560_e32657_d_n7;
        locals.var_t3__blk724_dn10 = assign23560_e32657_d_n10;
        locals.var_t3__blk724_dn11 = assign23560_e32657_d_n11;
        locals.var_t3__blk724_dn12 = assign23560_e32657_d_n12;
        locals.var_t3__blk724_dn17 = assign23560_e32657_d_n17;
        locals.var_t3__blk724_rv = 0.0;

        let (assign23570_e32668, assign23570_e32668_d_n0, assign23570_e32668_d_n2, assign23570_e32668_d_n6, assign23570_e32668_d_n7, assign23570_e32668_d_n10, assign23570_e32668_d_n11, assign23570_e32668_d_n12, assign23570_e32668_d_n17,) = {
    if ((locals.var_guard740 == 0.0) && (locals.var_guard741 != 0.0)) {
        let assign23570_e32665: f64 = (locals.var_cgs_ey * locals.var_t3__blk724);
        let assign23570_e32666: f64 = (1.0 + assign23570_e32665);
        (assign23570_e32666, ((locals.var_cgs_ey_dn0 * locals.var_t3__blk724) + (locals.var_cgs_ey * locals.var_t3__blk724_dn0)), ((locals.var_cgs_ey_dn2 * locals.var_t3__blk724) + (locals.var_cgs_ey * locals.var_t3__blk724_dn2)), ((locals.var_cgs_ey_dn6 * locals.var_t3__blk724) + (locals.var_cgs_ey * locals.var_t3__blk724_dn6)), ((locals.var_cgs_ey_dn7 * locals.var_t3__blk724) + (locals.var_cgs_ey * locals.var_t3__blk724_dn7)), ((locals.var_cgs_ey_dn10 * locals.var_t3__blk724) + (locals.var_cgs_ey * locals.var_t3__blk724_dn10)), ((locals.var_cgs_ey_dn11 * locals.var_t3__blk724) + (locals.var_cgs_ey * locals.var_t3__blk724_dn11)), ((locals.var_cgs_ey_dn12 * locals.var_t3__blk724) + (locals.var_cgs_ey * locals.var_t3__blk724_dn12)), ((locals.var_cgs_ey_dn17 * locals.var_t3__blk724) + (locals.var_cgs_ey * locals.var_t3__blk724_dn17)),)
    } else {
        (locals.var_t7__blk728, locals.var_t7__blk728_dn0, locals.var_t7__blk728_dn2, locals.var_t7__blk728_dn6, locals.var_t7__blk728_dn7, locals.var_t7__blk728_dn10, locals.var_t7__blk728_dn11, locals.var_t7__blk728_dn12, locals.var_t7__blk728_dn17,)
    }
};
        locals.var_t7__blk728 = assign23570_e32668;
        locals.var_t7__blk728_dn0 = assign23570_e32668_d_n0;
        locals.var_t7__blk728_dn2 = assign23570_e32668_d_n2;
        locals.var_t7__blk728_dn6 = assign23570_e32668_d_n6;
        locals.var_t7__blk728_dn7 = assign23570_e32668_d_n7;
        locals.var_t7__blk728_dn10 = assign23570_e32668_d_n10;
        locals.var_t7__blk728_dn11 = assign23570_e32668_d_n11;
        locals.var_t7__blk728_dn12 = assign23570_e32668_d_n12;
        locals.var_t7__blk728_dn17 = assign23570_e32668_d_n17;
        locals.var_t7__blk728_rv = 0.0;

        let (assign23580_e32677, assign23580_e32677_d_n0, assign23580_e32677_d_n2, assign23580_e32677_d_n6, assign23580_e32677_d_n7, assign23580_e32677_d_n10, assign23580_e32677_d_n11, assign23580_e32677_d_n12, assign23580_e32677_d_n17,) = {
    if ((locals.var_guard740 == 0.0) && (locals.var_guard741 != 0.0)) {
        let assign23580_e32675: f64 = (locals.var_t2__blk723 * locals.var_t7__blk728);
        (assign23580_e32675, ((locals.var_t2__blk723_dn0 * locals.var_t7__blk728) + (locals.var_t2__blk723 * locals.var_t7__blk728_dn0)), ((locals.var_t2__blk723_dn2 * locals.var_t7__blk728) + (locals.var_t2__blk723 * locals.var_t7__blk728_dn2)), ((locals.var_t2__blk723_dn6 * locals.var_t7__blk728) + (locals.var_t2__blk723 * locals.var_t7__blk728_dn6)), ((locals.var_t2__blk723_dn7 * locals.var_t7__blk728) + (locals.var_t2__blk723 * locals.var_t7__blk728_dn7)), ((locals.var_t2__blk723_dn10 * locals.var_t7__blk728) + (locals.var_t2__blk723 * locals.var_t7__blk728_dn10)), ((locals.var_t2__blk723_dn11 * locals.var_t7__blk728) + (locals.var_t2__blk723 * locals.var_t7__blk728_dn11)), ((locals.var_t2__blk723_dn12 * locals.var_t7__blk728) + (locals.var_t2__blk723 * locals.var_t7__blk728_dn12)), ((locals.var_t2__blk723_dn17 * locals.var_t7__blk728) + (locals.var_t2__blk723 * locals.var_t7__blk728_dn17)),)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn12, locals.var_etun_dn17,)
    }
};
        locals.var_etun = assign23580_e32677;
        locals.var_etun_dn0 = assign23580_e32677_d_n0;
        locals.var_etun_dn2 = assign23580_e32677_d_n2;
        locals.var_etun_dn6 = assign23580_e32677_d_n6;
        locals.var_etun_dn7 = assign23580_e32677_d_n7;
        locals.var_etun_dn10 = assign23580_e32677_d_n10;
        locals.var_etun_dn11 = assign23580_e32677_d_n11;
        locals.var_etun_dn12 = assign23580_e32677_d_n12;
        locals.var_etun_dn17 = assign23580_e32677_d_n17;
        locals.var_etun_rv = 0.0;

        let (assign23590_e32693, assign23590_e32693_d_n0, assign23590_e32693_d_n2, assign23590_e32693_d_n6, assign23590_e32693_d_n7, assign23590_e32693_d_n10, assign23590_e32693_d_n11, assign23590_e32693_d_n12, assign23590_e32693_d_n17,) = {
    if ((locals.var_guard740 == 0.0) && (locals.var_guard741 != 0.0)) {
        let assign23590_e32684: f64 = (locals.var_etun * locals.var_etun);
        let assign23590_e32687: f64 = (4.0 * 0.01);
        let assign23590_e32689: f64 = (assign23590_e32687 * 0.01);
        let assign23590_e32690: f64 = (assign23590_e32684 + assign23590_e32689);
        let assign23590_e32691: f64 = (assign23590_e32690).sqrt();
        (assign23590_e32691, (((locals.var_etun_dn0 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn0)) / (2.0 * assign23590_e32691)), (((locals.var_etun_dn2 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn2)) / (2.0 * assign23590_e32691)), (((locals.var_etun_dn6 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn6)) / (2.0 * assign23590_e32691)), (((locals.var_etun_dn7 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn7)) / (2.0 * assign23590_e32691)), (((locals.var_etun_dn10 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn10)) / (2.0 * assign23590_e32691)), (((locals.var_etun_dn11 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn11)) / (2.0 * assign23590_e32691)), (((locals.var_etun_dn12 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn12)) / (2.0 * assign23590_e32691)), (((locals.var_etun_dn17 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn17)) / (2.0 * assign23590_e32691)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign23590_e32693;
        locals.var_tmf1_dn0 = assign23590_e32693_d_n0;
        locals.var_tmf1_dn2 = assign23590_e32693_d_n2;
        locals.var_tmf1_dn6 = assign23590_e32693_d_n6;
        locals.var_tmf1_dn7 = assign23590_e32693_d_n7;
        locals.var_tmf1_dn10 = assign23590_e32693_d_n10;
        locals.var_tmf1_dn11 = assign23590_e32693_d_n11;
        locals.var_tmf1_dn12 = assign23590_e32693_d_n12;
        locals.var_tmf1_dn17 = assign23590_e32693_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign23600_e32708, assign23600_e32708_d_n0, assign23600_e32708_d_n2, assign23600_e32708_d_n6, assign23600_e32708_d_n7, assign23600_e32708_d_n10, assign23600_e32708_d_n11, assign23600_e32708_d_n12, assign23600_e32708_d_n17,) = {
    if ((locals.var_guard740 == 0.0) && (locals.var_guard741 != 0.0)) {
        let assign23600_e32701: f64 = (locals.var_etun + locals.var_tmf1);
        let assign23600_e32702: f64 = (0.5 * assign23600_e32701);
        let assign23600_e32705: f64 = (1e-10 * 0.01);
        let assign23600_e32706: f64 = (assign23600_e32702 + assign23600_e32705);
        (assign23600_e32706, (0.5 * (locals.var_etun_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_etun_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_etun_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_etun_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_etun_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_etun_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_etun_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_etun_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn12, locals.var_etun_dn17,)
    }
};
        locals.var_etun = assign23600_e32708;
        locals.var_etun_dn0 = assign23600_e32708_d_n0;
        locals.var_etun_dn2 = assign23600_e32708_d_n2;
        locals.var_etun_dn6 = assign23600_e32708_d_n6;
        locals.var_etun_dn7 = assign23600_e32708_d_n7;
        locals.var_etun_dn10 = assign23600_e32708_d_n10;
        locals.var_etun_dn11 = assign23600_e32708_d_n11;
        locals.var_etun_dn12 = assign23600_e32708_d_n12;
        locals.var_etun_dn17 = assign23600_e32708_d_n17;
        locals.var_etun_rv = 0.0;

        let assign23610_e32711: f64 = if locals.var_etun < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard742 = assign23610_e32711;
        locals.var_guard742_rv = 0.0;

        let (assign23620_e32720, assign23620_e32720_d_n0, assign23620_e32720_d_n2, assign23620_e32720_d_n6, assign23620_e32720_d_n7, assign23620_e32720_d_n10, assign23620_e32720_d_n11, assign23620_e32720_d_n12, assign23620_e32720_d_n17,) = {
    if (((locals.var_guard740 == 0.0) && (locals.var_guard741 != 0.0)) && (locals.var_guard742 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn12, locals.var_etun_dn17,)
    }
};
        locals.var_etun = assign23620_e32720;
        locals.var_etun_dn0 = assign23620_e32720_d_n0;
        locals.var_etun_dn2 = assign23620_e32720_d_n2;
        locals.var_etun_dn6 = assign23620_e32720_d_n6;
        locals.var_etun_dn7 = assign23620_e32720_d_n7;
        locals.var_etun_dn10 = assign23620_e32720_d_n10;
        locals.var_etun_dn11 = assign23620_e32720_d_n11;
        locals.var_etun_dn12 = assign23620_e32720_d_n12;
        locals.var_etun_dn17 = assign23620_e32720_d_n17;
        locals.var_etun_rv = 0.0;

        let (assign23630_e32736, assign23630_e32736_d_n0, assign23630_e32736_d_n2, assign23630_e32736_d_n6, assign23630_e32736_d_n7, assign23630_e32736_d_n10, assign23630_e32736_d_n11, assign23630_e32736_d_n12, assign23630_e32736_d_n17,) = {
    if ((locals.var_guard740 == 0.0) && (locals.var_guard741 != 0.0)) {
        let assign23630_e32727: f64 = (locals.var_vgsz * locals.var_vgsz);
        let assign23630_e32730: f64 = (4.0 * 0.001);
        let assign23630_e32732: f64 = (assign23630_e32730 * 0.001);
        let assign23630_e32733: f64 = (assign23630_e32727 + assign23630_e32732);
        let assign23630_e32734: f64 = (assign23630_e32733).sqrt();
        (assign23630_e32734, (((locals.var_vgsz_dn0 * locals.var_vgsz) + (locals.var_vgsz * locals.var_vgsz_dn0)) / (2.0 * assign23630_e32734)), (((locals.var_vgsz_dn2 * locals.var_vgsz) + (locals.var_vgsz * locals.var_vgsz_dn2)) / (2.0 * assign23630_e32734)), (((locals.var_vgsz_dn6 * locals.var_vgsz) + (locals.var_vgsz * locals.var_vgsz_dn6)) / (2.0 * assign23630_e32734)), (((locals.var_vgsz_dn7 * locals.var_vgsz) + (locals.var_vgsz * locals.var_vgsz_dn7)) / (2.0 * assign23630_e32734)), (((locals.var_vgsz_dn10 * locals.var_vgsz) + (locals.var_vgsz * locals.var_vgsz_dn10)) / (2.0 * assign23630_e32734)), (((locals.var_vgsz_dn11 * locals.var_vgsz) + (locals.var_vgsz * locals.var_vgsz_dn11)) / (2.0 * assign23630_e32734)), (((locals.var_vgsz_dn12 * locals.var_vgsz) + (locals.var_vgsz * locals.var_vgsz_dn12)) / (2.0 * assign23630_e32734)), (((locals.var_vgsz_dn17 * locals.var_vgsz) + (locals.var_vgsz * locals.var_vgsz_dn17)) / (2.0 * assign23630_e32734)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign23630_e32736;
        locals.var_tmf1_dn0 = assign23630_e32736_d_n0;
        locals.var_tmf1_dn2 = assign23630_e32736_d_n2;
        locals.var_tmf1_dn6 = assign23630_e32736_d_n6;
        locals.var_tmf1_dn7 = assign23630_e32736_d_n7;
        locals.var_tmf1_dn10 = assign23630_e32736_d_n10;
        locals.var_tmf1_dn11 = assign23630_e32736_d_n11;
        locals.var_tmf1_dn12 = assign23630_e32736_d_n12;
        locals.var_tmf1_dn17 = assign23630_e32736_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign23640_e32751, assign23640_e32751_d_n0, assign23640_e32751_d_n2, assign23640_e32751_d_n6, assign23640_e32751_d_n7, assign23640_e32751_d_n10, assign23640_e32751_d_n11, assign23640_e32751_d_n12, assign23640_e32751_d_n17,) = {
    if ((locals.var_guard740 == 0.0) && (locals.var_guard741 != 0.0)) {
        let assign23640_e32744: f64 = (locals.var_vgsz + locals.var_tmf1);
        let assign23640_e32745: f64 = (0.5 * assign23640_e32744);
        let assign23640_e32748: f64 = (1e-10 * 0.001);
        let assign23640_e32749: f64 = (assign23640_e32745 + assign23640_e32748);
        (assign23640_e32749, (0.5 * (locals.var_vgsz_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_vgsz_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_vgsz_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_vgsz_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_vgsz_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_vgsz_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_vgsz_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_vgsz_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_t3__blk724, locals.var_t3__blk724_dn0, locals.var_t3__blk724_dn2, locals.var_t3__blk724_dn6, locals.var_t3__blk724_dn7, locals.var_t3__blk724_dn10, locals.var_t3__blk724_dn11, locals.var_t3__blk724_dn12, locals.var_t3__blk724_dn17,)
    }
};
        locals.var_t3__blk724 = assign23640_e32751;
        locals.var_t3__blk724_dn0 = assign23640_e32751_d_n0;
        locals.var_t3__blk724_dn2 = assign23640_e32751_d_n2;
        locals.var_t3__blk724_dn6 = assign23640_e32751_d_n6;
        locals.var_t3__blk724_dn7 = assign23640_e32751_d_n7;
        locals.var_t3__blk724_dn10 = assign23640_e32751_d_n10;
        locals.var_t3__blk724_dn11 = assign23640_e32751_d_n11;
        locals.var_t3__blk724_dn12 = assign23640_e32751_d_n12;
        locals.var_t3__blk724_dn17 = assign23640_e32751_d_n17;
        locals.var_t3__blk724_rv = 0.0;

        let assign23650_e32754: f64 = if locals.var_t3__blk724 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard743 = assign23650_e32754;
        locals.var_guard743_rv = 0.0;

        let (assign23660_e32763, assign23660_e32763_d_n0, assign23660_e32763_d_n2, assign23660_e32763_d_n6, assign23660_e32763_d_n7, assign23660_e32763_d_n10, assign23660_e32763_d_n11, assign23660_e32763_d_n12, assign23660_e32763_d_n17,) = {
    if (((locals.var_guard740 == 0.0) && (locals.var_guard741 != 0.0)) && (locals.var_guard743 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3__blk724, locals.var_t3__blk724_dn0, locals.var_t3__blk724_dn2, locals.var_t3__blk724_dn6, locals.var_t3__blk724_dn7, locals.var_t3__blk724_dn10, locals.var_t3__blk724_dn11, locals.var_t3__blk724_dn12, locals.var_t3__blk724_dn17,)
    }
};
        locals.var_t3__blk724 = assign23660_e32763;
        locals.var_t3__blk724_dn0 = assign23660_e32763_d_n0;
        locals.var_t3__blk724_dn2 = assign23660_e32763_d_n2;
        locals.var_t3__blk724_dn6 = assign23660_e32763_d_n6;
        locals.var_t3__blk724_dn7 = assign23660_e32763_d_n7;
        locals.var_t3__blk724_dn10 = assign23660_e32763_d_n10;
        locals.var_t3__blk724_dn11 = assign23660_e32763_d_n11;
        locals.var_t3__blk724_dn12 = assign23660_e32763_d_n12;
        locals.var_t3__blk724_dn17 = assign23660_e32763_d_n17;
        locals.var_t3__blk724_rv = 0.0;

        let (assign23670_e32772, assign23670_e32772_d_n0, assign23670_e32772_d_n2, assign23670_e32772_d_n6, assign23670_e32772_d_n7, assign23670_e32772_d_n10, assign23670_e32772_d_n11, assign23670_e32772_d_n12, assign23670_e32772_d_n17,) = {
    if ((locals.var_guard740 == 0.0) && (locals.var_guard741 != 0.0)) {
        let assign23670_e32770: f64 = (locals.var_t3__blk724 - p.p226);
        (assign23670_e32770, locals.var_t3__blk724_dn0, locals.var_t3__blk724_dn2, locals.var_t3__blk724_dn6, locals.var_t3__blk724_dn7, locals.var_t3__blk724_dn10, locals.var_t3__blk724_dn11, locals.var_t3__blk724_dn12, locals.var_t3__blk724_dn17,)
    } else {
        (locals.var_t3__blk724, locals.var_t3__blk724_dn0, locals.var_t3__blk724_dn2, locals.var_t3__blk724_dn6, locals.var_t3__blk724_dn7, locals.var_t3__blk724_dn10, locals.var_t3__blk724_dn11, locals.var_t3__blk724_dn12, locals.var_t3__blk724_dn17,)
    }
};
        locals.var_t3__blk724 = assign23670_e32772;
        locals.var_t3__blk724_dn0 = assign23670_e32772_d_n0;
        locals.var_t3__blk724_dn2 = assign23670_e32772_d_n2;
        locals.var_t3__blk724_dn6 = assign23670_e32772_d_n6;
        locals.var_t3__blk724_dn7 = assign23670_e32772_d_n7;
        locals.var_t3__blk724_dn10 = assign23670_e32772_d_n10;
        locals.var_t3__blk724_dn11 = assign23670_e32772_d_n11;
        locals.var_t3__blk724_dn12 = assign23670_e32772_d_n12;
        locals.var_t3__blk724_dn17 = assign23670_e32772_d_n17;
        locals.var_t3__blk724_rv = 0.0;

        let (assign23680_e32781, assign23680_e32781_d_n0, assign23680_e32781_d_n2, assign23680_e32781_d_n6, assign23680_e32781_d_n7, assign23680_e32781_d_n10, assign23680_e32781_d_n11, assign23680_e32781_d_n12, assign23680_e32781_d_n17,) = {
    if ((locals.var_guard740 == 0.0) && (locals.var_guard741 != 0.0)) {
        let assign23680_e32779: f64 = (locals.var_t3__blk724 / 0.1);
        (assign23680_e32779, (locals.var_t3__blk724_dn0 / 0.1), (locals.var_t3__blk724_dn2 / 0.1), (locals.var_t3__blk724_dn6 / 0.1), (locals.var_t3__blk724_dn7 / 0.1), (locals.var_t3__blk724_dn10 / 0.1), (locals.var_t3__blk724_dn11 / 0.1), (locals.var_t3__blk724_dn12 / 0.1), (locals.var_t3__blk724_dn17 / 0.1),)
    } else {
        (locals.var_tx__blk720, locals.var_tx__blk720_dn0, locals.var_tx__blk720_dn2, locals.var_tx__blk720_dn6, locals.var_tx__blk720_dn7, locals.var_tx__blk720_dn10, locals.var_tx__blk720_dn11, locals.var_tx__blk720_dn12, locals.var_tx__blk720_dn17,)
    }
};
        locals.var_tx__blk720 = assign23680_e32781;
        locals.var_tx__blk720_dn0 = assign23680_e32781_d_n0;
        locals.var_tx__blk720_dn2 = assign23680_e32781_d_n2;
        locals.var_tx__blk720_dn6 = assign23680_e32781_d_n6;
        locals.var_tx__blk720_dn7 = assign23680_e32781_d_n7;
        locals.var_tx__blk720_dn10 = assign23680_e32781_d_n10;
        locals.var_tx__blk720_dn11 = assign23680_e32781_d_n11;
        locals.var_tx__blk720_dn12 = assign23680_e32781_d_n12;
        locals.var_tx__blk720_dn17 = assign23680_e32781_d_n17;
        locals.var_tx__blk720_rv = 0.0;

        let (assign23690_e32792, assign23690_e32792_d_n0, assign23690_e32792_d_n2, assign23690_e32792_d_n6, assign23690_e32792_d_n7, assign23690_e32792_d_n10, assign23690_e32792_d_n11, assign23690_e32792_d_n12, assign23690_e32792_d_n17,) = {
    if ((locals.var_guard740 == 0.0) && (locals.var_guard741 != 0.0)) {
        let assign23690_e32789: f64 = (locals.var_tx__blk720 * locals.var_tx__blk720);
        let assign23690_e32790: f64 = (1.0 + assign23690_e32789);
        (assign23690_e32790, ((locals.var_tx__blk720_dn0 * locals.var_tx__blk720) + (locals.var_tx__blk720 * locals.var_tx__blk720_dn0)), ((locals.var_tx__blk720_dn2 * locals.var_tx__blk720) + (locals.var_tx__blk720 * locals.var_tx__blk720_dn2)), ((locals.var_tx__blk720_dn6 * locals.var_tx__blk720) + (locals.var_tx__blk720 * locals.var_tx__blk720_dn6)), ((locals.var_tx__blk720_dn7 * locals.var_tx__blk720) + (locals.var_tx__blk720 * locals.var_tx__blk720_dn7)), ((locals.var_tx__blk720_dn10 * locals.var_tx__blk720) + (locals.var_tx__blk720 * locals.var_tx__blk720_dn10)), ((locals.var_tx__blk720_dn11 * locals.var_tx__blk720) + (locals.var_tx__blk720 * locals.var_tx__blk720_dn11)), ((locals.var_tx__blk720_dn12 * locals.var_tx__blk720) + (locals.var_tx__blk720 * locals.var_tx__blk720_dn12)), ((locals.var_tx__blk720_dn17 * locals.var_tx__blk720) + (locals.var_tx__blk720 * locals.var_tx__blk720_dn17)),)
    } else {
        (locals.var_t2__blk723, locals.var_t2__blk723_dn0, locals.var_t2__blk723_dn2, locals.var_t2__blk723_dn6, locals.var_t2__blk723_dn7, locals.var_t2__blk723_dn10, locals.var_t2__blk723_dn11, locals.var_t2__blk723_dn12, locals.var_t2__blk723_dn17,)
    }
};
        locals.var_t2__blk723 = assign23690_e32792;
        locals.var_t2__blk723_dn0 = assign23690_e32792_d_n0;
        locals.var_t2__blk723_dn2 = assign23690_e32792_d_n2;
        locals.var_t2__blk723_dn6 = assign23690_e32792_d_n6;
        locals.var_t2__blk723_dn7 = assign23690_e32792_d_n7;
        locals.var_t2__blk723_dn10 = assign23690_e32792_d_n10;
        locals.var_t2__blk723_dn11 = assign23690_e32792_d_n11;
        locals.var_t2__blk723_dn12 = assign23690_e32792_d_n12;
        locals.var_t2__blk723_dn17 = assign23690_e32792_d_n17;
        locals.var_t2__blk723_rv = 0.0;

        let (assign23700_e32803, assign23700_e32803_d_n0, assign23700_e32803_d_n2, assign23700_e32803_d_n6, assign23700_e32803_d_n7, assign23700_e32803_d_n10, assign23700_e32803_d_n11, assign23700_e32803_d_n12, assign23700_e32803_d_n17,) = {
    if ((locals.var_guard740 == 0.0) && (locals.var_guard741 != 0.0)) {
        let assign23700_e32800: f64 = (1.0 / locals.var_t2__blk723);
        let assign23700_e32801: f64 = (1.0 - assign23700_e32800);
        (assign23700_e32801, (-(-(locals.var_t2__blk723_dn0 / (locals.var_t2__blk723 * locals.var_t2__blk723)))), (-(-(locals.var_t2__blk723_dn2 / (locals.var_t2__blk723 * locals.var_t2__blk723)))), (-(-(locals.var_t2__blk723_dn6 / (locals.var_t2__blk723 * locals.var_t2__blk723)))), (-(-(locals.var_t2__blk723_dn7 / (locals.var_t2__blk723 * locals.var_t2__blk723)))), (-(-(locals.var_t2__blk723_dn10 / (locals.var_t2__blk723 * locals.var_t2__blk723)))), (-(-(locals.var_t2__blk723_dn11 / (locals.var_t2__blk723 * locals.var_t2__blk723)))), (-(-(locals.var_t2__blk723_dn12 / (locals.var_t2__blk723 * locals.var_t2__blk723)))), (-(-(locals.var_t2__blk723_dn17 / (locals.var_t2__blk723 * locals.var_t2__blk723)))),)
    } else {
        (locals.var_t1__blk722, locals.var_t1__blk722_dn0, locals.var_t1__blk722_dn2, locals.var_t1__blk722_dn6, locals.var_t1__blk722_dn7, locals.var_t1__blk722_dn10, locals.var_t1__blk722_dn11, locals.var_t1__blk722_dn12, locals.var_t1__blk722_dn17,)
    }
};
        locals.var_t1__blk722 = assign23700_e32803;
        locals.var_t1__blk722_dn0 = assign23700_e32803_d_n0;
        locals.var_t1__blk722_dn2 = assign23700_e32803_d_n2;
        locals.var_t1__blk722_dn6 = assign23700_e32803_d_n6;
        locals.var_t1__blk722_dn7 = assign23700_e32803_d_n7;
        locals.var_t1__blk722_dn10 = assign23700_e32803_d_n10;
        locals.var_t1__blk722_dn11 = assign23700_e32803_d_n11;
        locals.var_t1__blk722_dn12 = assign23700_e32803_d_n12;
        locals.var_t1__blk722_dn17 = assign23700_e32803_d_n17;
        locals.var_t1__blk722_rv = 0.0;

        let (assign23710_e32812, assign23710_e32812_d_n0, assign23710_e32812_d_n2, assign23710_e32812_d_n6, assign23710_e32812_d_n7, assign23710_e32812_d_n10, assign23710_e32812_d_n11, assign23710_e32812_d_n12, assign23710_e32812_d_n17,) = {
    if ((locals.var_guard740 == 0.0) && (locals.var_guard741 != 0.0)) {
        let assign23710_e32810: f64 = (locals.var_etun * locals.var_t1__blk722);
        (assign23710_e32810, ((locals.var_etun_dn0 * locals.var_t1__blk722) + (locals.var_etun * locals.var_t1__blk722_dn0)), ((locals.var_etun_dn2 * locals.var_t1__blk722) + (locals.var_etun * locals.var_t1__blk722_dn2)), ((locals.var_etun_dn6 * locals.var_t1__blk722) + (locals.var_etun * locals.var_t1__blk722_dn6)), ((locals.var_etun_dn7 * locals.var_t1__blk722) + (locals.var_etun * locals.var_t1__blk722_dn7)), ((locals.var_etun_dn10 * locals.var_t1__blk722) + (locals.var_etun * locals.var_t1__blk722_dn10)), ((locals.var_etun_dn11 * locals.var_t1__blk722) + (locals.var_etun * locals.var_t1__blk722_dn11)), ((locals.var_etun_dn12 * locals.var_t1__blk722) + (locals.var_etun * locals.var_t1__blk722_dn12)), ((locals.var_etun_dn17 * locals.var_t1__blk722) + (locals.var_etun * locals.var_t1__blk722_dn17)),)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn12, locals.var_etun_dn17,)
    }
};
        locals.var_etun = assign23710_e32812;
        locals.var_etun_dn0 = assign23710_e32812_d_n0;
        locals.var_etun_dn2 = assign23710_e32812_d_n2;
        locals.var_etun_dn6 = assign23710_e32812_d_n6;
        locals.var_etun_dn7 = assign23710_e32812_d_n7;
        locals.var_etun_dn10 = assign23710_e32812_d_n10;
        locals.var_etun_dn11 = assign23710_e32812_d_n11;
        locals.var_etun_dn12 = assign23710_e32812_d_n12;
        locals.var_etun_dn17 = assign23710_e32812_d_n17;
        locals.var_etun_rv = 0.0;

        let (assign23720_e32821, assign23720_e32821_d_n0, assign23720_e32821_d_n2, assign23720_e32821_d_n6, assign23720_e32821_d_n7, assign23720_e32821_d_n10, assign23720_e32821_d_n11, assign23720_e32821_d_n12, assign23720_e32821_d_n17,) = {
    if ((locals.var_guard740 == 0.0) && (locals.var_guard741 != 0.0)) {
        let assign23720_e32819: f64 = (locals.var_cgs_leff__blk735 * locals.var_cgs_weff_nf__blk736);
        (assign23720_e32819, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk721, locals.var_t0__blk721_dn0, locals.var_t0__blk721_dn2, locals.var_t0__blk721_dn6, locals.var_t0__blk721_dn7, locals.var_t0__blk721_dn10, locals.var_t0__blk721_dn11, locals.var_t0__blk721_dn12, locals.var_t0__blk721_dn17,)
    }
};
        locals.var_t0__blk721 = assign23720_e32821;
        locals.var_t0__blk721_dn0 = assign23720_e32821_d_n0;
        locals.var_t0__blk721_dn2 = assign23720_e32821_d_n2;
        locals.var_t0__blk721_dn6 = assign23720_e32821_d_n6;
        locals.var_t0__blk721_dn7 = assign23720_e32821_d_n7;
        locals.var_t0__blk721_dn10 = assign23720_e32821_d_n10;
        locals.var_t0__blk721_dn11 = assign23720_e32821_d_n11;
        locals.var_t0__blk721_dn12 = assign23720_e32821_d_n12;
        locals.var_t0__blk721_dn17 = assign23720_e32821_d_n17;
        locals.var_t0__blk721_rv = 0.0;

        let (assign23730_e32832, assign23730_e32832_d_n0, assign23730_e32832_d_n2, assign23730_e32832_d_n6, assign23730_e32832_d_n7, assign23730_e32832_d_n10, assign23730_e32832_d_n11, assign23730_e32832_d_n12, assign23730_e32832_d_n17,) = {
    if ((locals.var_guard740 == 0.0) && (locals.var_guard741 != 0.0)) {
        let assign23730_e32829: f64 = (p.p219 + locals.var_t0__blk721);
        let assign23730_e32830: f64 = (p.p219 / assign23730_e32829);
        (assign23730_e32830, (-((p.p219 * locals.var_t0__blk721_dn0) / (assign23730_e32829 * assign23730_e32829))), (-((p.p219 * locals.var_t0__blk721_dn2) / (assign23730_e32829 * assign23730_e32829))), (-((p.p219 * locals.var_t0__blk721_dn6) / (assign23730_e32829 * assign23730_e32829))), (-((p.p219 * locals.var_t0__blk721_dn7) / (assign23730_e32829 * assign23730_e32829))), (-((p.p219 * locals.var_t0__blk721_dn10) / (assign23730_e32829 * assign23730_e32829))), (-((p.p219 * locals.var_t0__blk721_dn11) / (assign23730_e32829 * assign23730_e32829))), (-((p.p219 * locals.var_t0__blk721_dn12) / (assign23730_e32829 * assign23730_e32829))), (-((p.p219 * locals.var_t0__blk721_dn17) / (assign23730_e32829 * assign23730_e32829))),)
    } else {
        (locals.var_t7__blk728, locals.var_t7__blk728_dn0, locals.var_t7__blk728_dn2, locals.var_t7__blk728_dn6, locals.var_t7__blk728_dn7, locals.var_t7__blk728_dn10, locals.var_t7__blk728_dn11, locals.var_t7__blk728_dn12, locals.var_t7__blk728_dn17,)
    }
};
        locals.var_t7__blk728 = assign23730_e32832;
        locals.var_t7__blk728_dn0 = assign23730_e32832_d_n0;
        locals.var_t7__blk728_dn2 = assign23730_e32832_d_n2;
        locals.var_t7__blk728_dn6 = assign23730_e32832_d_n6;
        locals.var_t7__blk728_dn7 = assign23730_e32832_d_n7;
        locals.var_t7__blk728_dn10 = assign23730_e32832_d_n10;
        locals.var_t7__blk728_dn11 = assign23730_e32832_d_n11;
        locals.var_t7__blk728_dn12 = assign23730_e32832_d_n12;
        locals.var_t7__blk728_dn17 = assign23730_e32832_d_n17;
        locals.var_t7__blk728_rv = 0.0;

        let (assign23740_e32839, assign23740_e32839_d_n0, assign23740_e32839_d_n2, assign23740_e32839_d_n6, assign23740_e32839_d_n7, assign23740_e32839_d_n10, assign23740_e32839_d_n11, assign23740_e32839_d_n12, assign23740_e32839_d_n17,) = {
    if ((locals.var_guard740 == 0.0) && (locals.var_guard741 != 0.0)) {
        (p.p218, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6__blk727, locals.var_t6__blk727_dn0, locals.var_t6__blk727_dn2, locals.var_t6__blk727_dn6, locals.var_t6__blk727_dn7, locals.var_t6__blk727_dn10, locals.var_t6__blk727_dn11, locals.var_t6__blk727_dn12, locals.var_t6__blk727_dn17,)
    }
};
        locals.var_t6__blk727 = assign23740_e32839;
        locals.var_t6__blk727_dn0 = assign23740_e32839_d_n0;
        locals.var_t6__blk727_dn2 = assign23740_e32839_d_n2;
        locals.var_t6__blk727_dn6 = assign23740_e32839_d_n6;
        locals.var_t6__blk727_dn7 = assign23740_e32839_d_n7;
        locals.var_t6__blk727_dn10 = assign23740_e32839_d_n10;
        locals.var_t6__blk727_dn11 = assign23740_e32839_d_n11;
        locals.var_t6__blk727_dn12 = assign23740_e32839_d_n12;
        locals.var_t6__blk727_dn17 = assign23740_e32839_d_n17;
        locals.var_t6__blk727_rv = 0.0;

        let (assign23760_e32861, assign23760_e32861_d_n0, assign23760_e32861_d_n2, assign23760_e32861_d_n6, assign23760_e32861_d_n7, assign23760_e32861_d_n10, assign23760_e32861_d_n11, assign23760_e32861_d_n12, assign23760_e32861_d_n17,) = {
    if ((locals.var_guard740 == 0.0) && (locals.var_guard741 != 0.0)) {
        let assign23760_e32858: f64 = (locals.var_etun + 1e-50);
        let assign23760_e32859: f64 = (1.0 / assign23760_e32858);
        (assign23760_e32859, (-(locals.var_etun_dn0 / (assign23760_e32858 * assign23760_e32858))), (-(locals.var_etun_dn2 / (assign23760_e32858 * assign23760_e32858))), (-(locals.var_etun_dn6 / (assign23760_e32858 * assign23760_e32858))), (-(locals.var_etun_dn7 / (assign23760_e32858 * assign23760_e32858))), (-(locals.var_etun_dn10 / (assign23760_e32858 * assign23760_e32858))), (-(locals.var_etun_dn11 / (assign23760_e32858 * assign23760_e32858))), (-(locals.var_etun_dn12 / (assign23760_e32858 * assign23760_e32858))), (-(locals.var_etun_dn17 / (assign23760_e32858 * assign23760_e32858))),)
    } else {
        (locals.var_t4__blk725, locals.var_t4__blk725_dn0, locals.var_t4__blk725_dn2, locals.var_t4__blk725_dn6, locals.var_t4__blk725_dn7, locals.var_t4__blk725_dn10, locals.var_t4__blk725_dn11, locals.var_t4__blk725_dn12, locals.var_t4__blk725_dn17,)
    }
};
        locals.var_t4__blk725 = assign23760_e32861;
        locals.var_t4__blk725_dn0 = assign23760_e32861_d_n0;
        locals.var_t4__blk725_dn2 = assign23760_e32861_d_n2;
        locals.var_t4__blk725_dn6 = assign23760_e32861_d_n6;
        locals.var_t4__blk725_dn7 = assign23760_e32861_d_n7;
        locals.var_t4__blk725_dn10 = assign23760_e32861_d_n10;
        locals.var_t4__blk725_dn11 = assign23760_e32861_d_n11;
        locals.var_t4__blk725_dn12 = assign23760_e32861_d_n12;
        locals.var_t4__blk725_dn17 = assign23760_e32861_d_n17;
        locals.var_t4__blk725_rv = 0.0;

        let (assign23770_e32873, assign23770_e32873_d_n0, assign23770_e32873_d_n2, assign23770_e32873_d_n6, assign23770_e32873_d_n7, assign23770_e32873_d_n10, assign23770_e32873_d_n11, assign23770_e32873_d_n12, assign23770_e32873_d_n17,) = {
    if ((locals.var_guard740 == 0.0) && (locals.var_guard741 != 0.0)) {
        let assign23770_e32867: f64 = (-p.p214);
        let assign23770_e32869: f64 = (assign23770_e32867 * locals.var_egp32);
        let assign23770_e32871: f64 = (assign23770_e32869 * locals.var_t4__blk725);
        (assign23770_e32871, (((assign23770_e32867 * locals.var_egp32_dn0) * locals.var_t4__blk725) + (assign23770_e32869 * locals.var_t4__blk725_dn0)), (((assign23770_e32867 * locals.var_egp32_dn2) * locals.var_t4__blk725) + (assign23770_e32869 * locals.var_t4__blk725_dn2)), (((assign23770_e32867 * locals.var_egp32_dn6) * locals.var_t4__blk725) + (assign23770_e32869 * locals.var_t4__blk725_dn6)), (((assign23770_e32867 * locals.var_egp32_dn7) * locals.var_t4__blk725) + (assign23770_e32869 * locals.var_t4__blk725_dn7)), (((assign23770_e32867 * locals.var_egp32_dn10) * locals.var_t4__blk725) + (assign23770_e32869 * locals.var_t4__blk725_dn10)), (((assign23770_e32867 * locals.var_egp32_dn11) * locals.var_t4__blk725) + (assign23770_e32869 * locals.var_t4__blk725_dn11)), (((assign23770_e32867 * locals.var_egp32_dn12) * locals.var_t4__blk725) + (assign23770_e32869 * locals.var_t4__blk725_dn12)), (((assign23770_e32867 * locals.var_egp32_dn17) * locals.var_t4__blk725) + (assign23770_e32869 * locals.var_t4__blk725_dn17)),)
    } else {
        (locals.var_t1__blk722, locals.var_t1__blk722_dn0, locals.var_t1__blk722_dn2, locals.var_t1__blk722_dn6, locals.var_t1__blk722_dn7, locals.var_t1__blk722_dn10, locals.var_t1__blk722_dn11, locals.var_t1__blk722_dn12, locals.var_t1__blk722_dn17,)
    }
};
        locals.var_t1__blk722 = assign23770_e32873;
        locals.var_t1__blk722_dn0 = assign23770_e32873_d_n0;
        locals.var_t1__blk722_dn2 = assign23770_e32873_d_n2;
        locals.var_t1__blk722_dn6 = assign23770_e32873_d_n6;
        locals.var_t1__blk722_dn7 = assign23770_e32873_d_n7;
        locals.var_t1__blk722_dn10 = assign23770_e32873_d_n10;
        locals.var_t1__blk722_dn11 = assign23770_e32873_d_n11;
        locals.var_t1__blk722_dn12 = assign23770_e32873_d_n12;
        locals.var_t1__blk722_dn17 = assign23770_e32873_d_n17;
        locals.var_t1__blk722_rv = 0.0;

        let assign23780_e32876: f64 = (-34.0);
        let assign23780_e32877: f64 = if locals.var_t1__blk722 < assign23780_e32876 { 1.0 } else { 0.0 };
        locals.var_guard744 = assign23780_e32877;
        locals.var_guard744_rv = 0.0;

        let (assign23800_e32897, assign23800_e32897_d_n0, assign23800_e32897_d_n2, assign23800_e32897_d_n6, assign23800_e32897_d_n7, assign23800_e32897_d_n10, assign23800_e32897_d_n11, assign23800_e32897_d_n12, assign23800_e32897_d_n17,) = {
    if (((locals.var_guard740 == 0.0) && (locals.var_guard741 != 0.0)) && (locals.var_guard744 == 0.0)) {
        let assign23800_e32895: f64 = (locals.var_t1__blk722).exp();
        (assign23800_e32895, (assign23800_e32895 * locals.var_t1__blk722_dn0), (assign23800_e32895 * locals.var_t1__blk722_dn2), (assign23800_e32895 * locals.var_t1__blk722_dn6), (assign23800_e32895 * locals.var_t1__blk722_dn7), (assign23800_e32895 * locals.var_t1__blk722_dn10), (assign23800_e32895 * locals.var_t1__blk722_dn11), (assign23800_e32895 * locals.var_t1__blk722_dn12), (assign23800_e32895 * locals.var_t1__blk722_dn17),)
    } else {
        (locals.var_t2__blk723, locals.var_t2__blk723_dn0, locals.var_t2__blk723_dn2, locals.var_t2__blk723_dn6, locals.var_t2__blk723_dn7, locals.var_t2__blk723_dn10, locals.var_t2__blk723_dn11, locals.var_t2__blk723_dn12, locals.var_t2__blk723_dn17,)
    }
};
        locals.var_t2__blk723 = assign23800_e32897;
        locals.var_t2__blk723_dn0 = assign23800_e32897_d_n0;
        locals.var_t2__blk723_dn2 = assign23800_e32897_d_n2;
        locals.var_t2__blk723_dn6 = assign23800_e32897_d_n6;
        locals.var_t2__blk723_dn7 = assign23800_e32897_d_n7;
        locals.var_t2__blk723_dn10 = assign23800_e32897_d_n10;
        locals.var_t2__blk723_dn11 = assign23800_e32897_d_n11;
        locals.var_t2__blk723_dn12 = assign23800_e32897_d_n12;
        locals.var_t2__blk723_dn17 = assign23800_e32897_d_n17;
        locals.var_t2__blk723_rv = 0.0;

        let (assign23810_e32913, assign23810_e32913_d_n0, assign23810_e32913_d_n2, assign23810_e32913_d_n6, assign23810_e32913_d_n7, assign23810_e32913_d_n10, assign23810_e32913_d_n11, assign23810_e32913_d_n12, assign23810_e32913_d_n17,) = {
    if (((locals.var_guard740 == 0.0) && (locals.var_guard741 != 0.0)) && (locals.var_guard744 == 0.0)) {
        let assign23810_e32907: f64 = (p.p213 / locals.var_egp12);
        let assign23810_e32909: f64 = (assign23810_e32907 * 1.6021918e-19);
        let assign23810_e32911: f64 = (assign23810_e32909 * locals.var_t0__blk721);
        (assign23810_e32911, ((((-((p.p213 * locals.var_egp12_dn0) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0__blk721) + (assign23810_e32909 * locals.var_t0__blk721_dn0)), ((((-((p.p213 * locals.var_egp12_dn2) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0__blk721) + (assign23810_e32909 * locals.var_t0__blk721_dn2)), ((((-((p.p213 * locals.var_egp12_dn6) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0__blk721) + (assign23810_e32909 * locals.var_t0__blk721_dn6)), ((((-((p.p213 * locals.var_egp12_dn7) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0__blk721) + (assign23810_e32909 * locals.var_t0__blk721_dn7)), ((((-((p.p213 * locals.var_egp12_dn10) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0__blk721) + (assign23810_e32909 * locals.var_t0__blk721_dn10)), ((((-((p.p213 * locals.var_egp12_dn11) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0__blk721) + (assign23810_e32909 * locals.var_t0__blk721_dn11)), ((((-((p.p213 * locals.var_egp12_dn12) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0__blk721) + (assign23810_e32909 * locals.var_t0__blk721_dn12)), ((((-((p.p213 * locals.var_egp12_dn17) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0__blk721) + (assign23810_e32909 * locals.var_t0__blk721_dn17)),)
    } else {
        (locals.var_t3__blk724, locals.var_t3__blk724_dn0, locals.var_t3__blk724_dn2, locals.var_t3__blk724_dn6, locals.var_t3__blk724_dn7, locals.var_t3__blk724_dn10, locals.var_t3__blk724_dn11, locals.var_t3__blk724_dn12, locals.var_t3__blk724_dn17,)
    }
};
        locals.var_t3__blk724 = assign23810_e32913;
        locals.var_t3__blk724_dn0 = assign23810_e32913_d_n0;
        locals.var_t3__blk724_dn2 = assign23810_e32913_d_n2;
        locals.var_t3__blk724_dn6 = assign23810_e32913_d_n6;
        locals.var_t3__blk724_dn7 = assign23810_e32913_d_n7;
        locals.var_t3__blk724_dn10 = assign23810_e32913_d_n10;
        locals.var_t3__blk724_dn11 = assign23810_e32913_d_n11;
        locals.var_t3__blk724_dn12 = assign23810_e32913_d_n12;
        locals.var_t3__blk724_dn17 = assign23810_e32913_d_n17;
        locals.var_t3__blk724_rv = 0.0;

        let (assign23820_e32925, assign23820_e32925_d_n0, assign23820_e32925_d_n2, assign23820_e32925_d_n6, assign23820_e32925_d_n7, assign23820_e32925_d_n10, assign23820_e32925_d_n11, assign23820_e32925_d_n12, assign23820_e32925_d_n17,) = {
    if (((locals.var_guard740 == 0.0) && (locals.var_guard741 != 0.0)) && (locals.var_guard744 == 0.0)) {
        let assign23820_e32923: f64 = (1.0 / locals.var_cgs_cnst0soi);
        (assign23820_e32923, (-(locals.var_cgs_cnst0soi_dn0 / (locals.var_cgs_cnst0soi * locals.var_cgs_cnst0soi))), (-(locals.var_cgs_cnst0soi_dn2 / (locals.var_cgs_cnst0soi * locals.var_cgs_cnst0soi))), (-(locals.var_cgs_cnst0soi_dn6 / (locals.var_cgs_cnst0soi * locals.var_cgs_cnst0soi))), (-(locals.var_cgs_cnst0soi_dn7 / (locals.var_cgs_cnst0soi * locals.var_cgs_cnst0soi))), (-(locals.var_cgs_cnst0soi_dn10 / (locals.var_cgs_cnst0soi * locals.var_cgs_cnst0soi))), (-(locals.var_cgs_cnst0soi_dn11 / (locals.var_cgs_cnst0soi * locals.var_cgs_cnst0soi))), (-(locals.var_cgs_cnst0soi_dn12 / (locals.var_cgs_cnst0soi * locals.var_cgs_cnst0soi))), (-(locals.var_cgs_cnst0soi_dn17 / (locals.var_cgs_cnst0soi * locals.var_cgs_cnst0soi))),)
    } else {
        (locals.var_t5__blk726, locals.var_t5__blk726_dn0, locals.var_t5__blk726_dn2, locals.var_t5__blk726_dn6, locals.var_t5__blk726_dn7, locals.var_t5__blk726_dn10, locals.var_t5__blk726_dn11, locals.var_t5__blk726_dn12, locals.var_t5__blk726_dn17,)
    }
};
        locals.var_t5__blk726 = assign23820_e32925;
        locals.var_t5__blk726_dn0 = assign23820_e32925_d_n0;
        locals.var_t5__blk726_dn2 = assign23820_e32925_d_n2;
        locals.var_t5__blk726_dn6 = assign23820_e32925_d_n6;
        locals.var_t5__blk726_dn7 = assign23820_e32925_d_n7;
        locals.var_t5__blk726_dn10 = assign23820_e32925_d_n10;
        locals.var_t5__blk726_dn11 = assign23820_e32925_d_n11;
        locals.var_t5__blk726_dn12 = assign23820_e32925_d_n12;
        locals.var_t5__blk726_dn17 = assign23820_e32925_d_n17;
        locals.var_t5__blk726_rv = 0.0;

        let (assign23830_e32942, assign23830_e32942_d_n0, assign23830_e32942_d_n2, assign23830_e32942_d_n6, assign23830_e32942_d_n7, assign23830_e32942_d_n10, assign23830_e32942_d_n11, assign23830_e32942_d_n12, assign23830_e32942_d_n17,) = {
    if (((locals.var_guard740 == 0.0) && (locals.var_guard741 != 0.0)) && (locals.var_guard744 == 0.0)) {
        let assign23830_e32936: f64 = (locals.var_cgs_c_fox * 1e-12);
        let assign23830_e32937: f64 = (locals.var_cgs_qiu__blk738 + assign23830_e32936);
        let assign23830_e32939: f64 = (assign23830_e32937 * locals.var_t5__blk726);
        let assign23830_e32940: f64 = (assign23830_e32939).sqrt();
        (assign23830_e32940, ((((locals.var_cgs_qiu__blk738_dn0 + (locals.var_cgs_c_fox_dn0 * 1e-12)) * locals.var_t5__blk726) + (assign23830_e32937 * locals.var_t5__blk726_dn0)) / (2.0 * assign23830_e32940)), ((((locals.var_cgs_qiu__blk738_dn2 + (locals.var_cgs_c_fox_dn2 * 1e-12)) * locals.var_t5__blk726) + (assign23830_e32937 * locals.var_t5__blk726_dn2)) / (2.0 * assign23830_e32940)), ((((locals.var_cgs_qiu__blk738_dn6 + (locals.var_cgs_c_fox_dn6 * 1e-12)) * locals.var_t5__blk726) + (assign23830_e32937 * locals.var_t5__blk726_dn6)) / (2.0 * assign23830_e32940)), ((((locals.var_cgs_qiu__blk738_dn7 + (locals.var_cgs_c_fox_dn7 * 1e-12)) * locals.var_t5__blk726) + (assign23830_e32937 * locals.var_t5__blk726_dn7)) / (2.0 * assign23830_e32940)), ((((locals.var_cgs_qiu__blk738_dn10 + (locals.var_cgs_c_fox_dn10 * 1e-12)) * locals.var_t5__blk726) + (assign23830_e32937 * locals.var_t5__blk726_dn10)) / (2.0 * assign23830_e32940)), ((((locals.var_cgs_qiu__blk738_dn11 + (locals.var_cgs_c_fox_dn11 * 1e-12)) * locals.var_t5__blk726) + (assign23830_e32937 * locals.var_t5__blk726_dn11)) / (2.0 * assign23830_e32940)), ((((locals.var_cgs_qiu__blk738_dn12 + (locals.var_cgs_c_fox_dn12 * 1e-12)) * locals.var_t5__blk726) + (assign23830_e32937 * locals.var_t5__blk726_dn12)) / (2.0 * assign23830_e32940)), ((((locals.var_cgs_qiu__blk738_dn17 + (locals.var_cgs_c_fox_dn17 * 1e-12)) * locals.var_t5__blk726) + (assign23830_e32937 * locals.var_t5__blk726_dn17)) / (2.0 * assign23830_e32940)),)
    } else {
        (locals.var_t6__blk727, locals.var_t6__blk727_dn0, locals.var_t6__blk727_dn2, locals.var_t6__blk727_dn6, locals.var_t6__blk727_dn7, locals.var_t6__blk727_dn10, locals.var_t6__blk727_dn11, locals.var_t6__blk727_dn12, locals.var_t6__blk727_dn17,)
    }
};
        locals.var_t6__blk727 = assign23830_e32942;
        locals.var_t6__blk727_dn0 = assign23830_e32942_d_n0;
        locals.var_t6__blk727_dn2 = assign23830_e32942_d_n2;
        locals.var_t6__blk727_dn6 = assign23830_e32942_d_n6;
        locals.var_t6__blk727_dn7 = assign23830_e32942_d_n7;
        locals.var_t6__blk727_dn10 = assign23830_e32942_d_n10;
        locals.var_t6__blk727_dn11 = assign23830_e32942_d_n11;
        locals.var_t6__blk727_dn12 = assign23830_e32942_d_n12;
        locals.var_t6__blk727_dn17 = assign23830_e32942_d_n17;
        locals.var_t6__blk727_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_85(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign23840_e32956, assign23840_e32956_d_n0, assign23840_e32956_d_n2, assign23840_e32956_d_n6, assign23840_e32956_d_n7, assign23840_e32956_d_n10, assign23840_e32956_d_n11, assign23840_e32956_d_n12, assign23840_e32956_d_n17,) = {
    if (((locals.var_guard740 == 0.0) && (locals.var_guard741 != 0.0)) && (locals.var_guard744 == 0.0)) {
        let assign23840_e32952: f64 = (locals.var_t2__blk723 * locals.var_t3__blk724);
        let assign23840_e32954: f64 = (assign23840_e32952 * locals.var_t6__blk727);
        (assign23840_e32954, ((((locals.var_t2__blk723_dn0 * locals.var_t3__blk724) + (locals.var_t2__blk723 * locals.var_t3__blk724_dn0)) * locals.var_t6__blk727) + (assign23840_e32952 * locals.var_t6__blk727_dn0)), ((((locals.var_t2__blk723_dn2 * locals.var_t3__blk724) + (locals.var_t2__blk723 * locals.var_t3__blk724_dn2)) * locals.var_t6__blk727) + (assign23840_e32952 * locals.var_t6__blk727_dn2)), ((((locals.var_t2__blk723_dn6 * locals.var_t3__blk724) + (locals.var_t2__blk723 * locals.var_t3__blk724_dn6)) * locals.var_t6__blk727) + (assign23840_e32952 * locals.var_t6__blk727_dn6)), ((((locals.var_t2__blk723_dn7 * locals.var_t3__blk724) + (locals.var_t2__blk723 * locals.var_t3__blk724_dn7)) * locals.var_t6__blk727) + (assign23840_e32952 * locals.var_t6__blk727_dn7)), ((((locals.var_t2__blk723_dn10 * locals.var_t3__blk724) + (locals.var_t2__blk723 * locals.var_t3__blk724_dn10)) * locals.var_t6__blk727) + (assign23840_e32952 * locals.var_t6__blk727_dn10)), ((((locals.var_t2__blk723_dn11 * locals.var_t3__blk724) + (locals.var_t2__blk723 * locals.var_t3__blk724_dn11)) * locals.var_t6__blk727) + (assign23840_e32952 * locals.var_t6__blk727_dn11)), ((((locals.var_t2__blk723_dn12 * locals.var_t3__blk724) + (locals.var_t2__blk723 * locals.var_t3__blk724_dn12)) * locals.var_t6__blk727) + (assign23840_e32952 * locals.var_t6__blk727_dn12)), ((((locals.var_t2__blk723_dn17 * locals.var_t3__blk724) + (locals.var_t2__blk723 * locals.var_t3__blk724_dn17)) * locals.var_t6__blk727) + (assign23840_e32952 * locals.var_t6__blk727_dn17)),)
    } else {
        (locals.var_t4__blk725, locals.var_t4__blk725_dn0, locals.var_t4__blk725_dn2, locals.var_t4__blk725_dn6, locals.var_t4__blk725_dn7, locals.var_t4__blk725_dn10, locals.var_t4__blk725_dn11, locals.var_t4__blk725_dn12, locals.var_t4__blk725_dn17,)
    }
};
        locals.var_t4__blk725 = assign23840_e32956;
        locals.var_t4__blk725_dn0 = assign23840_e32956_d_n0;
        locals.var_t4__blk725_dn2 = assign23840_e32956_d_n2;
        locals.var_t4__blk725_dn6 = assign23840_e32956_d_n6;
        locals.var_t4__blk725_dn7 = assign23840_e32956_d_n7;
        locals.var_t4__blk725_dn10 = assign23840_e32956_d_n10;
        locals.var_t4__blk725_dn11 = assign23840_e32956_d_n11;
        locals.var_t4__blk725_dn12 = assign23840_e32956_d_n12;
        locals.var_t4__blk725_dn17 = assign23840_e32956_d_n17;
        locals.var_t4__blk725_rv = 0.0;

        let (assign23880_e33002, assign23880_e33002_d_n0, assign23880_e33002_d_n2, assign23880_e33002_d_n6, assign23880_e33002_d_n7, assign23880_e33002_d_n10, assign23880_e33002_d_n11, assign23880_e33002_d_n12, assign23880_e33002_d_n17,) = {
    if (locals.var_guard740 == 0.0) {
        let assign23880_e32996: f64 = (-p.p221);
        let assign23880_e32998: f64 = (assign23880_e32996 * locals.var_vgs);
        let assign23880_e33000: f64 = (assign23880_e32998 + p.p222);
        (assign23880_e33000, 0.0, 0.0, (assign23880_e32996 * locals.var_vgs_dn6), (assign23880_e32996 * locals.var_vgs_dn7), 0.0, (assign23880_e32996 * locals.var_vgs_dn11), 0.0, 0.0,)
    } else {
        (locals.var_t0__blk721, locals.var_t0__blk721_dn0, locals.var_t0__blk721_dn2, locals.var_t0__blk721_dn6, locals.var_t0__blk721_dn7, locals.var_t0__blk721_dn10, locals.var_t0__blk721_dn11, locals.var_t0__blk721_dn12, locals.var_t0__blk721_dn17,)
    }
};
        locals.var_t0__blk721 = assign23880_e33002;
        locals.var_t0__blk721_dn0 = assign23880_e33002_d_n0;
        locals.var_t0__blk721_dn2 = assign23880_e33002_d_n2;
        locals.var_t0__blk721_dn6 = assign23880_e33002_d_n6;
        locals.var_t0__blk721_dn7 = assign23880_e33002_d_n7;
        locals.var_t0__blk721_dn10 = assign23880_e33002_d_n10;
        locals.var_t0__blk721_dn11 = assign23880_e33002_d_n11;
        locals.var_t0__blk721_dn12 = assign23880_e33002_d_n12;
        locals.var_t0__blk721_dn17 = assign23880_e33002_d_n17;
        locals.var_t0__blk721_rv = 0.0;

        let (assign23890_e33010, assign23890_e33010_d_n0, assign23890_e33010_d_n2, assign23890_e33010_d_n6, assign23890_e33010_d_n7, assign23890_e33010_d_n10, assign23890_e33010_d_n11, assign23890_e33010_d_n12, assign23890_e33010_d_n17,) = {
    if (locals.var_guard740 == 0.0) {
        let assign23890_e33007: f64 = (locals.var_cgs_tfox0__blk733 * locals.var_t0__blk721);
        let assign23890_e33008: f64 = (assign23890_e33007).exp();
        (assign23890_e33008, (assign23890_e33008 * (locals.var_cgs_tfox0__blk733 * locals.var_t0__blk721_dn0)), (assign23890_e33008 * (locals.var_cgs_tfox0__blk733 * locals.var_t0__blk721_dn2)), (assign23890_e33008 * (locals.var_cgs_tfox0__blk733 * locals.var_t0__blk721_dn6)), (assign23890_e33008 * (locals.var_cgs_tfox0__blk733 * locals.var_t0__blk721_dn7)), (assign23890_e33008 * (locals.var_cgs_tfox0__blk733 * locals.var_t0__blk721_dn10)), (assign23890_e33008 * (locals.var_cgs_tfox0__blk733 * locals.var_t0__blk721_dn11)), (assign23890_e33008 * (locals.var_cgs_tfox0__blk733 * locals.var_t0__blk721_dn12)), (assign23890_e33008 * (locals.var_cgs_tfox0__blk733 * locals.var_t0__blk721_dn17)),)
    } else {
        (locals.var_t2__blk723, locals.var_t2__blk723_dn0, locals.var_t2__blk723_dn2, locals.var_t2__blk723_dn6, locals.var_t2__blk723_dn7, locals.var_t2__blk723_dn10, locals.var_t2__blk723_dn11, locals.var_t2__blk723_dn12, locals.var_t2__blk723_dn17,)
    }
};
        locals.var_t2__blk723 = assign23890_e33010;
        locals.var_t2__blk723_dn0 = assign23890_e33010_d_n0;
        locals.var_t2__blk723_dn2 = assign23890_e33010_d_n2;
        locals.var_t2__blk723_dn6 = assign23890_e33010_d_n6;
        locals.var_t2__blk723_dn7 = assign23890_e33010_d_n7;
        locals.var_t2__blk723_dn10 = assign23890_e33010_d_n10;
        locals.var_t2__blk723_dn11 = assign23890_e33010_d_n11;
        locals.var_t2__blk723_dn12 = assign23890_e33010_d_n12;
        locals.var_t2__blk723_dn17 = assign23890_e33010_d_n17;
        locals.var_t2__blk723_rv = 0.0;

        let (assign23900_e33019, assign23900_e33019_d_n0, assign23900_e33019_d_n2, assign23900_e33019_d_n6, assign23900_e33019_d_n7, assign23900_e33019_d_n10, assign23900_e33019_d_n11, assign23900_e33019_d_n12, assign23900_e33019_d_n17,) = {
    if (locals.var_guard740 == 0.0) {
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_cgs_tfox0__blk733;
        let assign23900_e33015: f64 = (locals.var_vgs * __rspice_inv_cse_0);
        let assign23900_e33017: f64 = (assign23900_e33015 * __rspice_inv_cse_0);
        (assign23900_e33017, 0.0, 0.0, ((locals.var_vgs_dn6 / locals.var_cgs_tfox0__blk733) / locals.var_cgs_tfox0__blk733), ((locals.var_vgs_dn7 / locals.var_cgs_tfox0__blk733) / locals.var_cgs_tfox0__blk733), 0.0, ((locals.var_vgs_dn11 / locals.var_cgs_tfox0__blk733) / locals.var_cgs_tfox0__blk733), 0.0, 0.0,)
    } else {
        (locals.var_t0__blk721, locals.var_t0__blk721_dn0, locals.var_t0__blk721_dn2, locals.var_t0__blk721_dn6, locals.var_t0__blk721_dn7, locals.var_t0__blk721_dn10, locals.var_t0__blk721_dn11, locals.var_t0__blk721_dn12, locals.var_t0__blk721_dn17,)
    }
};
        locals.var_t0__blk721 = assign23900_e33019;
        locals.var_t0__blk721_dn0 = assign23900_e33019_d_n0;
        locals.var_t0__blk721_dn2 = assign23900_e33019_d_n2;
        locals.var_t0__blk721_dn6 = assign23900_e33019_d_n6;
        locals.var_t0__blk721_dn7 = assign23900_e33019_d_n7;
        locals.var_t0__blk721_dn10 = assign23900_e33019_d_n10;
        locals.var_t0__blk721_dn11 = assign23900_e33019_d_n11;
        locals.var_t0__blk721_dn12 = assign23900_e33019_d_n12;
        locals.var_t0__blk721_dn17 = assign23900_e33019_d_n17;
        locals.var_t0__blk721_rv = 0.0;

        let (assign23910_e33026, assign23910_e33026_d_n0, assign23910_e33026_d_n2, assign23910_e33026_d_n6, assign23910_e33026_d_n7, assign23910_e33026_d_n10, assign23910_e33026_d_n11, assign23910_e33026_d_n12, assign23910_e33026_d_n17,) = {
    if (locals.var_guard740 == 0.0) {
        let assign23910_e33024: f64 = (locals.var_vgs * locals.var_t0__blk721);
        (assign23910_e33024, (locals.var_vgs * locals.var_t0__blk721_dn0), (locals.var_vgs * locals.var_t0__blk721_dn2), ((locals.var_vgs_dn6 * locals.var_t0__blk721) + (locals.var_vgs * locals.var_t0__blk721_dn6)), ((locals.var_vgs_dn7 * locals.var_t0__blk721) + (locals.var_vgs * locals.var_t0__blk721_dn7)), (locals.var_vgs * locals.var_t0__blk721_dn10), ((locals.var_vgs_dn11 * locals.var_t0__blk721) + (locals.var_vgs * locals.var_t0__blk721_dn11)), (locals.var_vgs * locals.var_t0__blk721_dn12), (locals.var_vgs * locals.var_t0__blk721_dn17),)
    } else {
        (locals.var_t3__blk724, locals.var_t3__blk724_dn0, locals.var_t3__blk724_dn2, locals.var_t3__blk724_dn6, locals.var_t3__blk724_dn7, locals.var_t3__blk724_dn10, locals.var_t3__blk724_dn11, locals.var_t3__blk724_dn12, locals.var_t3__blk724_dn17,)
    }
};
        locals.var_t3__blk724 = assign23910_e33026;
        locals.var_t3__blk724_dn0 = assign23910_e33026_d_n0;
        locals.var_t3__blk724_dn2 = assign23910_e33026_d_n2;
        locals.var_t3__blk724_dn6 = assign23910_e33026_d_n6;
        locals.var_t3__blk724_dn7 = assign23910_e33026_d_n7;
        locals.var_t3__blk724_dn10 = assign23910_e33026_d_n10;
        locals.var_t3__blk724_dn11 = assign23910_e33026_d_n11;
        locals.var_t3__blk724_dn12 = assign23910_e33026_d_n12;
        locals.var_t3__blk724_dn17 = assign23910_e33026_d_n17;
        locals.var_t3__blk724_rv = 0.0;

        let (assign23920_e33035, assign23920_e33035_d_n0, assign23920_e33035_d_n2, assign23920_e33035_d_n6, assign23920_e33035_d_n7, assign23920_e33035_d_n10, assign23920_e33035_d_n11, assign23920_e33035_d_n12, assign23920_e33035_d_n17,) = {
    if (locals.var_guard740 == 0.0) {
        let assign23920_e33031: f64 = (p.p220 / 1000000.0);
        let assign23920_e33033: f64 = (assign23920_e33031 * locals.var_cgs_weff_nf__blk736);
        (assign23920_e33033, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4__blk725, locals.var_t4__blk725_dn0, locals.var_t4__blk725_dn2, locals.var_t4__blk725_dn6, locals.var_t4__blk725_dn7, locals.var_t4__blk725_dn10, locals.var_t4__blk725_dn11, locals.var_t4__blk725_dn12, locals.var_t4__blk725_dn17,)
    }
};
        locals.var_t4__blk725 = assign23920_e33035;
        locals.var_t4__blk725_dn0 = assign23920_e33035_d_n0;
        locals.var_t4__blk725_dn2 = assign23920_e33035_d_n2;
        locals.var_t4__blk725_dn6 = assign23920_e33035_d_n6;
        locals.var_t4__blk725_dn7 = assign23920_e33035_d_n7;
        locals.var_t4__blk725_dn10 = assign23920_e33035_d_n10;
        locals.var_t4__blk725_dn11 = assign23920_e33035_d_n11;
        locals.var_t4__blk725_dn12 = assign23920_e33035_d_n12;
        locals.var_t4__blk725_dn17 = assign23920_e33035_d_n17;
        locals.var_t4__blk725_rv = 0.0;

        let (assign23960_e33064, assign23960_e33064_d_n0, assign23960_e33064_d_n2, assign23960_e33064_d_n6, assign23960_e33064_d_n7, assign23960_e33064_d_n10, assign23960_e33064_d_n11, assign23960_e33064_d_n12, assign23960_e33064_d_n17,) = {
    if (locals.var_guard740 == 0.0) {
        let assign23960_e33062: f64 = (locals.var_vgs - locals.var_vds);
        (assign23960_e33062, (-locals.var_vds_dn0), (-locals.var_vds_dn2), (locals.var_vgs_dn6 - locals.var_vds_dn6), (locals.var_vgs_dn7 - locals.var_vds_dn7), (-locals.var_vds_dn10), (locals.var_vgs_dn11 - locals.var_vds_dn11), (-locals.var_vds_dn12), (-locals.var_vds_dn17),)
    } else {
        (locals.var_t1__blk722, locals.var_t1__blk722_dn0, locals.var_t1__blk722_dn2, locals.var_t1__blk722_dn6, locals.var_t1__blk722_dn7, locals.var_t1__blk722_dn10, locals.var_t1__blk722_dn11, locals.var_t1__blk722_dn12, locals.var_t1__blk722_dn17,)
    }
};
        locals.var_t1__blk722 = assign23960_e33064;
        locals.var_t1__blk722_dn0 = assign23960_e33064_d_n0;
        locals.var_t1__blk722_dn2 = assign23960_e33064_d_n2;
        locals.var_t1__blk722_dn6 = assign23960_e33064_d_n6;
        locals.var_t1__blk722_dn7 = assign23960_e33064_d_n7;
        locals.var_t1__blk722_dn10 = assign23960_e33064_d_n10;
        locals.var_t1__blk722_dn11 = assign23960_e33064_d_n11;
        locals.var_t1__blk722_dn12 = assign23960_e33064_d_n12;
        locals.var_t1__blk722_dn17 = assign23960_e33064_d_n17;
        locals.var_t1__blk722_rv = 0.0;

        let (assign23970_e33074, assign23970_e33074_d_n0, assign23970_e33074_d_n2, assign23970_e33074_d_n6, assign23970_e33074_d_n7, assign23970_e33074_d_n10, assign23970_e33074_d_n11, assign23970_e33074_d_n12, assign23970_e33074_d_n17,) = {
    if (locals.var_guard740 == 0.0) {
        let assign23970_e33068: f64 = (-p.p221);
        let assign23970_e33070: f64 = (assign23970_e33068 * locals.var_t1__blk722);
        let assign23970_e33072: f64 = (assign23970_e33070 + p.p222);
        (assign23970_e33072, (assign23970_e33068 * locals.var_t1__blk722_dn0), (assign23970_e33068 * locals.var_t1__blk722_dn2), (assign23970_e33068 * locals.var_t1__blk722_dn6), (assign23970_e33068 * locals.var_t1__blk722_dn7), (assign23970_e33068 * locals.var_t1__blk722_dn10), (assign23970_e33068 * locals.var_t1__blk722_dn11), (assign23970_e33068 * locals.var_t1__blk722_dn12), (assign23970_e33068 * locals.var_t1__blk722_dn17),)
    } else {
        (locals.var_t0__blk721, locals.var_t0__blk721_dn0, locals.var_t0__blk721_dn2, locals.var_t0__blk721_dn6, locals.var_t0__blk721_dn7, locals.var_t0__blk721_dn10, locals.var_t0__blk721_dn11, locals.var_t0__blk721_dn12, locals.var_t0__blk721_dn17,)
    }
};
        locals.var_t0__blk721 = assign23970_e33074;
        locals.var_t0__blk721_dn0 = assign23970_e33074_d_n0;
        locals.var_t0__blk721_dn2 = assign23970_e33074_d_n2;
        locals.var_t0__blk721_dn6 = assign23970_e33074_d_n6;
        locals.var_t0__blk721_dn7 = assign23970_e33074_d_n7;
        locals.var_t0__blk721_dn10 = assign23970_e33074_d_n10;
        locals.var_t0__blk721_dn11 = assign23970_e33074_d_n11;
        locals.var_t0__blk721_dn12 = assign23970_e33074_d_n12;
        locals.var_t0__blk721_dn17 = assign23970_e33074_d_n17;
        locals.var_t0__blk721_rv = 0.0;

        let (assign23980_e33082, assign23980_e33082_d_n0, assign23980_e33082_d_n2, assign23980_e33082_d_n6, assign23980_e33082_d_n7, assign23980_e33082_d_n10, assign23980_e33082_d_n11, assign23980_e33082_d_n12, assign23980_e33082_d_n17,) = {
    if (locals.var_guard740 == 0.0) {
        let assign23980_e33079: f64 = (locals.var_cgs_tfox0__blk733 * locals.var_t0__blk721);
        let assign23980_e33080: f64 = (assign23980_e33079).exp();
        (assign23980_e33080, (assign23980_e33080 * (locals.var_cgs_tfox0__blk733 * locals.var_t0__blk721_dn0)), (assign23980_e33080 * (locals.var_cgs_tfox0__blk733 * locals.var_t0__blk721_dn2)), (assign23980_e33080 * (locals.var_cgs_tfox0__blk733 * locals.var_t0__blk721_dn6)), (assign23980_e33080 * (locals.var_cgs_tfox0__blk733 * locals.var_t0__blk721_dn7)), (assign23980_e33080 * (locals.var_cgs_tfox0__blk733 * locals.var_t0__blk721_dn10)), (assign23980_e33080 * (locals.var_cgs_tfox0__blk733 * locals.var_t0__blk721_dn11)), (assign23980_e33080 * (locals.var_cgs_tfox0__blk733 * locals.var_t0__blk721_dn12)), (assign23980_e33080 * (locals.var_cgs_tfox0__blk733 * locals.var_t0__blk721_dn17)),)
    } else {
        (locals.var_t2__blk723, locals.var_t2__blk723_dn0, locals.var_t2__blk723_dn2, locals.var_t2__blk723_dn6, locals.var_t2__blk723_dn7, locals.var_t2__blk723_dn10, locals.var_t2__blk723_dn11, locals.var_t2__blk723_dn12, locals.var_t2__blk723_dn17,)
    }
};
        locals.var_t2__blk723 = assign23980_e33082;
        locals.var_t2__blk723_dn0 = assign23980_e33082_d_n0;
        locals.var_t2__blk723_dn2 = assign23980_e33082_d_n2;
        locals.var_t2__blk723_dn6 = assign23980_e33082_d_n6;
        locals.var_t2__blk723_dn7 = assign23980_e33082_d_n7;
        locals.var_t2__blk723_dn10 = assign23980_e33082_d_n10;
        locals.var_t2__blk723_dn11 = assign23980_e33082_d_n11;
        locals.var_t2__blk723_dn12 = assign23980_e33082_d_n12;
        locals.var_t2__blk723_dn17 = assign23980_e33082_d_n17;
        locals.var_t2__blk723_rv = 0.0;

        let (assign23990_e33091, assign23990_e33091_d_n0, assign23990_e33091_d_n2, assign23990_e33091_d_n6, assign23990_e33091_d_n7, assign23990_e33091_d_n10, assign23990_e33091_d_n11, assign23990_e33091_d_n12, assign23990_e33091_d_n17,) = {
    if (locals.var_guard740 == 0.0) {
        let __rspice_inv_cse_1: f64 = 1.0 / locals.var_cgs_tfox0__blk733;
        let assign23990_e33087: f64 = (locals.var_t1__blk722 * __rspice_inv_cse_1);
        let assign23990_e33089: f64 = (assign23990_e33087 * __rspice_inv_cse_1);
        (assign23990_e33089, ((locals.var_t1__blk722_dn0 / locals.var_cgs_tfox0__blk733) / locals.var_cgs_tfox0__blk733), ((locals.var_t1__blk722_dn2 / locals.var_cgs_tfox0__blk733) / locals.var_cgs_tfox0__blk733), ((locals.var_t1__blk722_dn6 / locals.var_cgs_tfox0__blk733) / locals.var_cgs_tfox0__blk733), ((locals.var_t1__blk722_dn7 / locals.var_cgs_tfox0__blk733) / locals.var_cgs_tfox0__blk733), ((locals.var_t1__blk722_dn10 / locals.var_cgs_tfox0__blk733) / locals.var_cgs_tfox0__blk733), ((locals.var_t1__blk722_dn11 / locals.var_cgs_tfox0__blk733) / locals.var_cgs_tfox0__blk733), ((locals.var_t1__blk722_dn12 / locals.var_cgs_tfox0__blk733) / locals.var_cgs_tfox0__blk733), ((locals.var_t1__blk722_dn17 / locals.var_cgs_tfox0__blk733) / locals.var_cgs_tfox0__blk733),)
    } else {
        (locals.var_t0__blk721, locals.var_t0__blk721_dn0, locals.var_t0__blk721_dn2, locals.var_t0__blk721_dn6, locals.var_t0__blk721_dn7, locals.var_t0__blk721_dn10, locals.var_t0__blk721_dn11, locals.var_t0__blk721_dn12, locals.var_t0__blk721_dn17,)
    }
};
        locals.var_t0__blk721 = assign23990_e33091;
        locals.var_t0__blk721_dn0 = assign23990_e33091_d_n0;
        locals.var_t0__blk721_dn2 = assign23990_e33091_d_n2;
        locals.var_t0__blk721_dn6 = assign23990_e33091_d_n6;
        locals.var_t0__blk721_dn7 = assign23990_e33091_d_n7;
        locals.var_t0__blk721_dn10 = assign23990_e33091_d_n10;
        locals.var_t0__blk721_dn11 = assign23990_e33091_d_n11;
        locals.var_t0__blk721_dn12 = assign23990_e33091_d_n12;
        locals.var_t0__blk721_dn17 = assign23990_e33091_d_n17;
        locals.var_t0__blk721_rv = 0.0;

        let (assign24000_e33098, assign24000_e33098_d_n0, assign24000_e33098_d_n2, assign24000_e33098_d_n6, assign24000_e33098_d_n7, assign24000_e33098_d_n10, assign24000_e33098_d_n11, assign24000_e33098_d_n12, assign24000_e33098_d_n17,) = {
    if (locals.var_guard740 == 0.0) {
        let assign24000_e33096: f64 = (locals.var_t1__blk722 * locals.var_t0__blk721);
        (assign24000_e33096, ((locals.var_t1__blk722_dn0 * locals.var_t0__blk721) + (locals.var_t1__blk722 * locals.var_t0__blk721_dn0)), ((locals.var_t1__blk722_dn2 * locals.var_t0__blk721) + (locals.var_t1__blk722 * locals.var_t0__blk721_dn2)), ((locals.var_t1__blk722_dn6 * locals.var_t0__blk721) + (locals.var_t1__blk722 * locals.var_t0__blk721_dn6)), ((locals.var_t1__blk722_dn7 * locals.var_t0__blk721) + (locals.var_t1__blk722 * locals.var_t0__blk721_dn7)), ((locals.var_t1__blk722_dn10 * locals.var_t0__blk721) + (locals.var_t1__blk722 * locals.var_t0__blk721_dn10)), ((locals.var_t1__blk722_dn11 * locals.var_t0__blk721) + (locals.var_t1__blk722 * locals.var_t0__blk721_dn11)), ((locals.var_t1__blk722_dn12 * locals.var_t0__blk721) + (locals.var_t1__blk722 * locals.var_t0__blk721_dn12)), ((locals.var_t1__blk722_dn17 * locals.var_t0__blk721) + (locals.var_t1__blk722 * locals.var_t0__blk721_dn17)),)
    } else {
        (locals.var_t3__blk724, locals.var_t3__blk724_dn0, locals.var_t3__blk724_dn2, locals.var_t3__blk724_dn6, locals.var_t3__blk724_dn7, locals.var_t3__blk724_dn10, locals.var_t3__blk724_dn11, locals.var_t3__blk724_dn12, locals.var_t3__blk724_dn17,)
    }
};
        locals.var_t3__blk724 = assign24000_e33098;
        locals.var_t3__blk724_dn0 = assign24000_e33098_d_n0;
        locals.var_t3__blk724_dn2 = assign24000_e33098_d_n2;
        locals.var_t3__blk724_dn6 = assign24000_e33098_d_n6;
        locals.var_t3__blk724_dn7 = assign24000_e33098_d_n7;
        locals.var_t3__blk724_dn10 = assign24000_e33098_d_n10;
        locals.var_t3__blk724_dn11 = assign24000_e33098_d_n11;
        locals.var_t3__blk724_dn12 = assign24000_e33098_d_n12;
        locals.var_t3__blk724_dn17 = assign24000_e33098_d_n17;
        locals.var_t3__blk724_rv = 0.0;

        let (assign24010_e33107, assign24010_e33107_d_n0, assign24010_e33107_d_n2, assign24010_e33107_d_n6, assign24010_e33107_d_n7, assign24010_e33107_d_n10, assign24010_e33107_d_n11, assign24010_e33107_d_n12, assign24010_e33107_d_n17,) = {
    if (locals.var_guard740 == 0.0) {
        let assign24010_e33103: f64 = (p.p220 / 1000000.0);
        let assign24010_e33105: f64 = (assign24010_e33103 * locals.var_cgs_weff_nf__blk736);
        (assign24010_e33105, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4__blk725, locals.var_t4__blk725_dn0, locals.var_t4__blk725_dn2, locals.var_t4__blk725_dn6, locals.var_t4__blk725_dn7, locals.var_t4__blk725_dn10, locals.var_t4__blk725_dn11, locals.var_t4__blk725_dn12, locals.var_t4__blk725_dn17,)
    }
};
        locals.var_t4__blk725 = assign24010_e33107;
        locals.var_t4__blk725_dn0 = assign24010_e33107_d_n0;
        locals.var_t4__blk725_dn2 = assign24010_e33107_d_n2;
        locals.var_t4__blk725_dn6 = assign24010_e33107_d_n6;
        locals.var_t4__blk725_dn7 = assign24010_e33107_d_n7;
        locals.var_t4__blk725_dn10 = assign24010_e33107_d_n10;
        locals.var_t4__blk725_dn11 = assign24010_e33107_d_n11;
        locals.var_t4__blk725_dn12 = assign24010_e33107_d_n12;
        locals.var_t4__blk725_dn17 = assign24010_e33107_d_n17;
        locals.var_t4__blk725_rv = 0.0;

        let (assign24050_e33143, assign24050_e33143_d_n0, assign24050_e33143_d_n2, assign24050_e33143_d_n6, assign24050_e33143_d_n7, assign24050_e33143_d_n10, assign24050_e33143_d_n11, assign24050_e33143_d_n12, assign24050_e33143_d_n17,) = {
    if (locals.var_guard740 == 0.0) {
        let assign24050_e33133: f64 = (-locals.var_vgs);
        let assign24050_e33135: f64 = (assign24050_e33133 + locals.var_vbsp);
        let assign24050_e33137: f64 = (assign24050_e33135 + locals.var_vfb);
        let assign24050_e33139: f64 = (assign24050_e33137 + p.p225);
        let assign24050_e33141: f64 = (assign24050_e33139 / locals.var_cgs_tfox0__blk733);
        (assign24050_e33141, (locals.var_vbsp_dn0 / locals.var_cgs_tfox0__blk733), (locals.var_vbsp_dn2 / locals.var_cgs_tfox0__blk733), (((-locals.var_vgs_dn6) + locals.var_vbsp_dn6) / locals.var_cgs_tfox0__blk733), (((-locals.var_vgs_dn7) + locals.var_vbsp_dn7) / locals.var_cgs_tfox0__blk733), (locals.var_vbsp_dn10 / locals.var_cgs_tfox0__blk733), (((-locals.var_vgs_dn11) + locals.var_vbsp_dn11) / locals.var_cgs_tfox0__blk733), (locals.var_vbsp_dn12 / locals.var_cgs_tfox0__blk733), (locals.var_vbsp_dn17 / locals.var_cgs_tfox0__blk733),)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn12, locals.var_etun_dn17,)
    }
};
        locals.var_etun = assign24050_e33143;
        locals.var_etun_dn0 = assign24050_e33143_d_n0;
        locals.var_etun_dn2 = assign24050_e33143_d_n2;
        locals.var_etun_dn6 = assign24050_e33143_d_n6;
        locals.var_etun_dn7 = assign24050_e33143_d_n7;
        locals.var_etun_dn10 = assign24050_e33143_d_n10;
        locals.var_etun_dn11 = assign24050_e33143_d_n11;
        locals.var_etun_dn12 = assign24050_e33143_d_n12;
        locals.var_etun_dn17 = assign24050_e33143_d_n17;
        locals.var_etun_rv = 0.0;

        let (assign24060_e33157, assign24060_e33157_d_n0, assign24060_e33157_d_n2, assign24060_e33157_d_n6, assign24060_e33157_d_n7, assign24060_e33157_d_n10, assign24060_e33157_d_n11, assign24060_e33157_d_n12, assign24060_e33157_d_n17,) = {
    if (locals.var_guard740 == 0.0) {
        let assign24060_e33148: f64 = (locals.var_etun * locals.var_etun);
        let assign24060_e33151: f64 = (4.0 * 0.01);
        let assign24060_e33153: f64 = (assign24060_e33151 * 0.01);
        let assign24060_e33154: f64 = (assign24060_e33148 + assign24060_e33153);
        let assign24060_e33155: f64 = (assign24060_e33154).sqrt();
        (assign24060_e33155, (((locals.var_etun_dn0 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn0)) / (2.0 * assign24060_e33155)), (((locals.var_etun_dn2 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn2)) / (2.0 * assign24060_e33155)), (((locals.var_etun_dn6 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn6)) / (2.0 * assign24060_e33155)), (((locals.var_etun_dn7 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn7)) / (2.0 * assign24060_e33155)), (((locals.var_etun_dn10 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn10)) / (2.0 * assign24060_e33155)), (((locals.var_etun_dn11 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn11)) / (2.0 * assign24060_e33155)), (((locals.var_etun_dn12 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn12)) / (2.0 * assign24060_e33155)), (((locals.var_etun_dn17 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn17)) / (2.0 * assign24060_e33155)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign24060_e33157;
        locals.var_tmf1_dn0 = assign24060_e33157_d_n0;
        locals.var_tmf1_dn2 = assign24060_e33157_d_n2;
        locals.var_tmf1_dn6 = assign24060_e33157_d_n6;
        locals.var_tmf1_dn7 = assign24060_e33157_d_n7;
        locals.var_tmf1_dn10 = assign24060_e33157_d_n10;
        locals.var_tmf1_dn11 = assign24060_e33157_d_n11;
        locals.var_tmf1_dn12 = assign24060_e33157_d_n12;
        locals.var_tmf1_dn17 = assign24060_e33157_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign24070_e33170, assign24070_e33170_d_n0, assign24070_e33170_d_n2, assign24070_e33170_d_n6, assign24070_e33170_d_n7, assign24070_e33170_d_n10, assign24070_e33170_d_n11, assign24070_e33170_d_n12, assign24070_e33170_d_n17,) = {
    if (locals.var_guard740 == 0.0) {
        let assign24070_e33163: f64 = (locals.var_etun + locals.var_tmf1);
        let assign24070_e33164: f64 = (0.5 * assign24070_e33163);
        let assign24070_e33167: f64 = (1e-10 * 0.01);
        let assign24070_e33168: f64 = (assign24070_e33164 + assign24070_e33167);
        (assign24070_e33168, (0.5 * (locals.var_etun_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_etun_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_etun_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_etun_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_etun_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_etun_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_etun_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_etun_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn12, locals.var_etun_dn17,)
    }
};
        locals.var_etun = assign24070_e33170;
        locals.var_etun_dn0 = assign24070_e33170_d_n0;
        locals.var_etun_dn2 = assign24070_e33170_d_n2;
        locals.var_etun_dn6 = assign24070_e33170_d_n6;
        locals.var_etun_dn7 = assign24070_e33170_d_n7;
        locals.var_etun_dn10 = assign24070_e33170_d_n10;
        locals.var_etun_dn11 = assign24070_e33170_d_n11;
        locals.var_etun_dn12 = assign24070_e33170_d_n12;
        locals.var_etun_dn17 = assign24070_e33170_d_n17;
        locals.var_etun_rv = 0.0;

        let assign24080_e33173: f64 = if locals.var_etun < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard747 = assign24080_e33173;
        locals.var_guard747_rv = 0.0;

        let (assign24090_e33180, assign24090_e33180_d_n0, assign24090_e33180_d_n2, assign24090_e33180_d_n6, assign24090_e33180_d_n7, assign24090_e33180_d_n10, assign24090_e33180_d_n11, assign24090_e33180_d_n12, assign24090_e33180_d_n17,) = {
    if ((locals.var_guard740 == 0.0) && (locals.var_guard747 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn12, locals.var_etun_dn17,)
    }
};
        locals.var_etun = assign24090_e33180;
        locals.var_etun_dn0 = assign24090_e33180_d_n0;
        locals.var_etun_dn2 = assign24090_e33180_d_n2;
        locals.var_etun_dn6 = assign24090_e33180_d_n6;
        locals.var_etun_dn7 = assign24090_e33180_d_n7;
        locals.var_etun_dn10 = assign24090_e33180_d_n10;
        locals.var_etun_dn11 = assign24090_e33180_d_n11;
        locals.var_etun_dn12 = assign24090_e33180_d_n12;
        locals.var_etun_dn17 = assign24090_e33180_d_n17;
        locals.var_etun_rv = 0.0;

        let (assign24100_e33187, assign24100_e33187_d_n0, assign24100_e33187_d_n2, assign24100_e33187_d_n6, assign24100_e33187_d_n7, assign24100_e33187_d_n10, assign24100_e33187_d_n11, assign24100_e33187_d_n12, assign24100_e33187_d_n17,) = {
    if (locals.var_guard740 == 0.0) {
        let assign24100_e33185: f64 = (locals.var_etun + 1e-50);
        (assign24100_e33185, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn12, locals.var_etun_dn17,)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn12, locals.var_etun_dn17,)
    }
};
        locals.var_etun = assign24100_e33187;
        locals.var_etun_dn0 = assign24100_e33187_d_n0;
        locals.var_etun_dn2 = assign24100_e33187_d_n2;
        locals.var_etun_dn6 = assign24100_e33187_d_n6;
        locals.var_etun_dn7 = assign24100_e33187_d_n7;
        locals.var_etun_dn10 = assign24100_e33187_d_n10;
        locals.var_etun_dn11 = assign24100_e33187_d_n11;
        locals.var_etun_dn12 = assign24100_e33187_d_n12;
        locals.var_etun_dn17 = assign24100_e33187_d_n17;
        locals.var_etun_rv = 0.0;

        let (assign24110_e33195, assign24110_e33195_d_n0, assign24110_e33195_d_n2, assign24110_e33195_d_n6, assign24110_e33195_d_n7, assign24110_e33195_d_n10, assign24110_e33195_d_n11, assign24110_e33195_d_n12, assign24110_e33195_d_n17,) = {
    if (locals.var_guard740 == 0.0) {
        let assign24110_e33191: f64 = (-p.p224);
        let assign24110_e33193: f64 = (assign24110_e33191 / locals.var_etun);
        (assign24110_e33193, (-((assign24110_e33191 * locals.var_etun_dn0) / (locals.var_etun * locals.var_etun))), (-((assign24110_e33191 * locals.var_etun_dn2) / (locals.var_etun * locals.var_etun))), (-((assign24110_e33191 * locals.var_etun_dn6) / (locals.var_etun * locals.var_etun))), (-((assign24110_e33191 * locals.var_etun_dn7) / (locals.var_etun * locals.var_etun))), (-((assign24110_e33191 * locals.var_etun_dn10) / (locals.var_etun * locals.var_etun))), (-((assign24110_e33191 * locals.var_etun_dn11) / (locals.var_etun * locals.var_etun))), (-((assign24110_e33191 * locals.var_etun_dn12) / (locals.var_etun * locals.var_etun))), (-((assign24110_e33191 * locals.var_etun_dn17) / (locals.var_etun * locals.var_etun))),)
    } else {
        (locals.var_t1__blk722, locals.var_t1__blk722_dn0, locals.var_t1__blk722_dn2, locals.var_t1__blk722_dn6, locals.var_t1__blk722_dn7, locals.var_t1__blk722_dn10, locals.var_t1__blk722_dn11, locals.var_t1__blk722_dn12, locals.var_t1__blk722_dn17,)
    }
};
        locals.var_t1__blk722 = assign24110_e33195;
        locals.var_t1__blk722_dn0 = assign24110_e33195_d_n0;
        locals.var_t1__blk722_dn2 = assign24110_e33195_d_n2;
        locals.var_t1__blk722_dn6 = assign24110_e33195_d_n6;
        locals.var_t1__blk722_dn7 = assign24110_e33195_d_n7;
        locals.var_t1__blk722_dn10 = assign24110_e33195_d_n10;
        locals.var_t1__blk722_dn11 = assign24110_e33195_d_n11;
        locals.var_t1__blk722_dn12 = assign24110_e33195_d_n12;
        locals.var_t1__blk722_dn17 = assign24110_e33195_d_n17;
        locals.var_t1__blk722_rv = 0.0;

        let assign24120_e33198: f64 = (-34.0);
        let assign24120_e33199: f64 = if locals.var_t1__blk722 < assign24120_e33198 { 1.0 } else { 0.0 };
        locals.var_guard748 = assign24120_e33199;
        locals.var_guard748_rv = 0.0;

        let (assign24140_e33215, assign24140_e33215_d_n0, assign24140_e33215_d_n2, assign24140_e33215_d_n6, assign24140_e33215_d_n7, assign24140_e33215_d_n10, assign24140_e33215_d_n11, assign24140_e33215_d_n12, assign24140_e33215_d_n17,) = {
    if ((locals.var_guard740 == 0.0) && (locals.var_guard748 == 0.0)) {
        let assign24140_e33213: f64 = (locals.var_t1__blk722).exp();
        (assign24140_e33213, (assign24140_e33213 * locals.var_t1__blk722_dn0), (assign24140_e33213 * locals.var_t1__blk722_dn2), (assign24140_e33213 * locals.var_t1__blk722_dn6), (assign24140_e33213 * locals.var_t1__blk722_dn7), (assign24140_e33213 * locals.var_t1__blk722_dn10), (assign24140_e33213 * locals.var_t1__blk722_dn11), (assign24140_e33213 * locals.var_t1__blk722_dn12), (assign24140_e33213 * locals.var_t1__blk722_dn17),)
    } else {
        (locals.var_t2__blk723, locals.var_t2__blk723_dn0, locals.var_t2__blk723_dn2, locals.var_t2__blk723_dn6, locals.var_t2__blk723_dn7, locals.var_t2__blk723_dn10, locals.var_t2__blk723_dn11, locals.var_t2__blk723_dn12, locals.var_t2__blk723_dn17,)
    }
};
        locals.var_t2__blk723 = assign24140_e33215;
        locals.var_t2__blk723_dn0 = assign24140_e33215_d_n0;
        locals.var_t2__blk723_dn2 = assign24140_e33215_d_n2;
        locals.var_t2__blk723_dn6 = assign24140_e33215_d_n6;
        locals.var_t2__blk723_dn7 = assign24140_e33215_d_n7;
        locals.var_t2__blk723_dn10 = assign24140_e33215_d_n10;
        locals.var_t2__blk723_dn11 = assign24140_e33215_d_n11;
        locals.var_t2__blk723_dn12 = assign24140_e33215_d_n12;
        locals.var_t2__blk723_dn17 = assign24140_e33215_d_n17;
        locals.var_t2__blk723_rv = 0.0;

        let (assign24150_e33227, assign24150_e33227_d_n0, assign24150_e33227_d_n2, assign24150_e33227_d_n6, assign24150_e33227_d_n7, assign24150_e33227_d_n10, assign24150_e33227_d_n11, assign24150_e33227_d_n12, assign24150_e33227_d_n17,) = {
    if ((locals.var_guard740 == 0.0) && (locals.var_guard748 == 0.0)) {
        let assign24150_e33223: f64 = (p.p223 * locals.var_cgs_weff_nf__blk736);
        let assign24150_e33225: f64 = (assign24150_e33223 * locals.var_cgs_leff__blk735);
        (assign24150_e33225, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3__blk724, locals.var_t3__blk724_dn0, locals.var_t3__blk724_dn2, locals.var_t3__blk724_dn6, locals.var_t3__blk724_dn7, locals.var_t3__blk724_dn10, locals.var_t3__blk724_dn11, locals.var_t3__blk724_dn12, locals.var_t3__blk724_dn17,)
    }
};
        locals.var_t3__blk724 = assign24150_e33227;
        locals.var_t3__blk724_dn0 = assign24150_e33227_d_n0;
        locals.var_t3__blk724_dn2 = assign24150_e33227_d_n2;
        locals.var_t3__blk724_dn6 = assign24150_e33227_d_n6;
        locals.var_t3__blk724_dn7 = assign24150_e33227_d_n7;
        locals.var_t3__blk724_dn10 = assign24150_e33227_d_n10;
        locals.var_t3__blk724_dn11 = assign24150_e33227_d_n11;
        locals.var_t3__blk724_dn12 = assign24150_e33227_d_n12;
        locals.var_t3__blk724_dn17 = assign24150_e33227_d_n17;
        locals.var_t3__blk724_rv = 0.0;

        let assign24180_e33249: f64 = if p.p28 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard756 = assign24180_e33249;
        locals.var_guard756_rv = 0.0;

        let (assign24200_e33270, assign24200_e33270_d_n0, assign24200_e33270_d_n2, assign24200_e33270_d_n6, assign24200_e33270_d_n7, assign24200_e33270_d_n10, assign24200_e33270_d_n11, assign24200_e33270_d_n12, assign24200_e33270_d_n17,) = {
    if (locals.var_guard756 == 0.0) {
        let assign24200_e33259: f64 = (locals.var_vds + p.p210);
        let assign24200_e33260: f64 = (p.p209 * assign24200_e33259);
        let assign24200_e33262: f64 = (assign24200_e33260 - locals.var_vgs);
        let assign24200_e33265: f64 = (locals.var_dvthsc + locals.var_dvthlp);
        let assign24200_e33267: f64 = (assign24200_e33265 * p.p211);
        let assign24200_e33268: f64 = (assign24200_e33262 + assign24200_e33267);
        (assign24200_e33268, ((p.p209 * locals.var_vds_dn0) + ((locals.var_dvthsc_dn0 + locals.var_dvthlp_dn0) * p.p211)), ((p.p209 * locals.var_vds_dn2) + ((locals.var_dvthsc_dn2 + locals.var_dvthlp_dn2) * p.p211)), (((p.p209 * locals.var_vds_dn6) - locals.var_vgs_dn6) + ((locals.var_dvthsc_dn6 + locals.var_dvthlp_dn6) * p.p211)), (((p.p209 * locals.var_vds_dn7) - locals.var_vgs_dn7) + ((locals.var_dvthsc_dn7 + locals.var_dvthlp_dn7) * p.p211)), ((p.p209 * locals.var_vds_dn10) + ((locals.var_dvthsc_dn10 + locals.var_dvthlp_dn10) * p.p211)), (((p.p209 * locals.var_vds_dn11) - locals.var_vgs_dn11) + ((locals.var_dvthsc_dn11 + locals.var_dvthlp_dn11) * p.p211)), ((p.p209 * locals.var_vds_dn12) + ((locals.var_dvthsc_dn12 + locals.var_dvthlp_dn12) * p.p211)), ((p.p209 * locals.var_vds_dn17) + ((locals.var_dvthsc_dn17 + locals.var_dvthlp_dn17) * p.p211)),)
    } else {
        (locals.var_t1__blk749, locals.var_t1__blk749_dn0, locals.var_t1__blk749_dn2, locals.var_t1__blk749_dn6, locals.var_t1__blk749_dn7, locals.var_t1__blk749_dn10, locals.var_t1__blk749_dn11, locals.var_t1__blk749_dn12, locals.var_t1__blk749_dn17,)
    }
};
        locals.var_t1__blk749 = assign24200_e33270;
        locals.var_t1__blk749_dn0 = assign24200_e33270_d_n0;
        locals.var_t1__blk749_dn2 = assign24200_e33270_d_n2;
        locals.var_t1__blk749_dn6 = assign24200_e33270_d_n6;
        locals.var_t1__blk749_dn7 = assign24200_e33270_d_n7;
        locals.var_t1__blk749_dn10 = assign24200_e33270_d_n10;
        locals.var_t1__blk749_dn11 = assign24200_e33270_d_n11;
        locals.var_t1__blk749_dn12 = assign24200_e33270_d_n12;
        locals.var_t1__blk749_dn17 = assign24200_e33270_d_n17;
        locals.var_t1__blk749_rv = 0.0;

        let (assign24210_e33277, assign24210_e33277_d_n0, assign24210_e33277_d_n2, assign24210_e33277_d_n6, assign24210_e33277_d_n7, assign24210_e33277_d_n10, assign24210_e33277_d_n11, assign24210_e33277_d_n12, assign24210_e33277_d_n17,) = {
    if (locals.var_guard756 == 0.0) {
        let assign24210_e33275: f64 = (1.0 / locals.var_tfox0);
        (assign24210_e33275, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk750, locals.var_t2__blk750_dn0, locals.var_t2__blk750_dn2, locals.var_t2__blk750_dn6, locals.var_t2__blk750_dn7, locals.var_t2__blk750_dn10, locals.var_t2__blk750_dn11, locals.var_t2__blk750_dn12, locals.var_t2__blk750_dn17,)
    }
};
        locals.var_t2__blk750 = assign24210_e33277;
        locals.var_t2__blk750_dn0 = assign24210_e33277_d_n0;
        locals.var_t2__blk750_dn2 = assign24210_e33277_d_n2;
        locals.var_t2__blk750_dn6 = assign24210_e33277_d_n6;
        locals.var_t2__blk750_dn7 = assign24210_e33277_d_n7;
        locals.var_t2__blk750_dn10 = assign24210_e33277_d_n10;
        locals.var_t2__blk750_dn11 = assign24210_e33277_d_n11;
        locals.var_t2__blk750_dn12 = assign24210_e33277_d_n12;
        locals.var_t2__blk750_dn17 = assign24210_e33277_d_n17;
        locals.var_t2__blk750_rv = 0.0;

        let (assign24220_e33284, assign24220_e33284_d_n0, assign24220_e33284_d_n2, assign24220_e33284_d_n6, assign24220_e33284_d_n7, assign24220_e33284_d_n10, assign24220_e33284_d_n11, assign24220_e33284_d_n12, assign24220_e33284_d_n17,) = {
    if (locals.var_guard756 == 0.0) {
        let assign24220_e33282: f64 = (locals.var_t1__blk749 * locals.var_t2__blk750);
        (assign24220_e33282, ((locals.var_t1__blk749_dn0 * locals.var_t2__blk750) + (locals.var_t1__blk749 * locals.var_t2__blk750_dn0)), ((locals.var_t1__blk749_dn2 * locals.var_t2__blk750) + (locals.var_t1__blk749 * locals.var_t2__blk750_dn2)), ((locals.var_t1__blk749_dn6 * locals.var_t2__blk750) + (locals.var_t1__blk749 * locals.var_t2__blk750_dn6)), ((locals.var_t1__blk749_dn7 * locals.var_t2__blk750) + (locals.var_t1__blk749 * locals.var_t2__blk750_dn7)), ((locals.var_t1__blk749_dn10 * locals.var_t2__blk750) + (locals.var_t1__blk749 * locals.var_t2__blk750_dn10)), ((locals.var_t1__blk749_dn11 * locals.var_t2__blk750) + (locals.var_t1__blk749 * locals.var_t2__blk750_dn11)), ((locals.var_t1__blk749_dn12 * locals.var_t2__blk750) + (locals.var_t1__blk749 * locals.var_t2__blk750_dn12)), ((locals.var_t1__blk749_dn17 * locals.var_t2__blk750) + (locals.var_t1__blk749 * locals.var_t2__blk750_dn17)),)
    } else {
        (locals.var_e1, locals.var_e1_dn0, locals.var_e1_dn2, locals.var_e1_dn6, locals.var_e1_dn7, locals.var_e1_dn10, locals.var_e1_dn11, locals.var_e1_dn12, locals.var_e1_dn17,)
    }
};
        locals.var_e1 = assign24220_e33284;
        locals.var_e1_dn0 = assign24220_e33284_d_n0;
        locals.var_e1_dn2 = assign24220_e33284_d_n2;
        locals.var_e1_dn6 = assign24220_e33284_d_n6;
        locals.var_e1_dn7 = assign24220_e33284_d_n7;
        locals.var_e1_dn10 = assign24220_e33284_d_n10;
        locals.var_e1_dn11 = assign24220_e33284_d_n11;
        locals.var_e1_dn12 = assign24220_e33284_d_n12;
        locals.var_e1_dn17 = assign24220_e33284_d_n17;
        locals.var_e1_rv = 0.0;

        let (assign24230_e33298, assign24230_e33298_d_n0, assign24230_e33298_d_n2, assign24230_e33298_d_n6, assign24230_e33298_d_n7, assign24230_e33298_d_n10, assign24230_e33298_d_n11, assign24230_e33298_d_n12, assign24230_e33298_d_n17,) = {
    if (locals.var_guard756 == 0.0) {
        let assign24230_e33289: f64 = (locals.var_e1 * locals.var_e1);
        let assign24230_e33292: f64 = (4.0 * 0.01);
        let assign24230_e33294: f64 = (assign24230_e33292 * 0.01);
        let assign24230_e33295: f64 = (assign24230_e33289 + assign24230_e33294);
        let assign24230_e33296: f64 = (assign24230_e33295).sqrt();
        (assign24230_e33296, (((locals.var_e1_dn0 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn0)) / (2.0 * assign24230_e33296)), (((locals.var_e1_dn2 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn2)) / (2.0 * assign24230_e33296)), (((locals.var_e1_dn6 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn6)) / (2.0 * assign24230_e33296)), (((locals.var_e1_dn7 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn7)) / (2.0 * assign24230_e33296)), (((locals.var_e1_dn10 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn10)) / (2.0 * assign24230_e33296)), (((locals.var_e1_dn11 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn11)) / (2.0 * assign24230_e33296)), (((locals.var_e1_dn12 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn12)) / (2.0 * assign24230_e33296)), (((locals.var_e1_dn17 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn17)) / (2.0 * assign24230_e33296)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign24230_e33298;
        locals.var_tmf1_dn0 = assign24230_e33298_d_n0;
        locals.var_tmf1_dn2 = assign24230_e33298_d_n2;
        locals.var_tmf1_dn6 = assign24230_e33298_d_n6;
        locals.var_tmf1_dn7 = assign24230_e33298_d_n7;
        locals.var_tmf1_dn10 = assign24230_e33298_d_n10;
        locals.var_tmf1_dn11 = assign24230_e33298_d_n11;
        locals.var_tmf1_dn12 = assign24230_e33298_d_n12;
        locals.var_tmf1_dn17 = assign24230_e33298_d_n17;
        locals.var_tmf1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_86(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign24240_e33311, assign24240_e33311_d_n0, assign24240_e33311_d_n2, assign24240_e33311_d_n6, assign24240_e33311_d_n7, assign24240_e33311_d_n10, assign24240_e33311_d_n11, assign24240_e33311_d_n12, assign24240_e33311_d_n17,) = {
    if (locals.var_guard756 == 0.0) {
        let assign24240_e33304: f64 = (locals.var_e1 + locals.var_tmf1);
        let assign24240_e33305: f64 = (0.5 * assign24240_e33304);
        let assign24240_e33308: f64 = (1e-10 * 0.01);
        let assign24240_e33309: f64 = (assign24240_e33305 + assign24240_e33308);
        (assign24240_e33309, (0.5 * (locals.var_e1_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_e1_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_e1_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_e1_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_e1_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_e1_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_e1_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_e1_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_egidl, locals.var_egidl_dn0, locals.var_egidl_dn2, locals.var_egidl_dn6, locals.var_egidl_dn7, locals.var_egidl_dn10, locals.var_egidl_dn11, locals.var_egidl_dn12, locals.var_egidl_dn17,)
    }
};
        locals.var_egidl = assign24240_e33311;
        locals.var_egidl_dn0 = assign24240_e33311_d_n0;
        locals.var_egidl_dn2 = assign24240_e33311_d_n2;
        locals.var_egidl_dn6 = assign24240_e33311_d_n6;
        locals.var_egidl_dn7 = assign24240_e33311_d_n7;
        locals.var_egidl_dn10 = assign24240_e33311_d_n10;
        locals.var_egidl_dn11 = assign24240_e33311_d_n11;
        locals.var_egidl_dn12 = assign24240_e33311_d_n12;
        locals.var_egidl_dn17 = assign24240_e33311_d_n17;
        locals.var_egidl_rv = 0.0;

        let assign24250_e33314: f64 = if locals.var_egidl < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard757 = assign24250_e33314;
        locals.var_guard757_rv = 0.0;

        let (assign24260_e33321, assign24260_e33321_d_n0, assign24260_e33321_d_n2, assign24260_e33321_d_n6, assign24260_e33321_d_n7, assign24260_e33321_d_n10, assign24260_e33321_d_n11, assign24260_e33321_d_n12, assign24260_e33321_d_n17,) = {
    if ((locals.var_guard756 == 0.0) && (locals.var_guard757 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_egidl, locals.var_egidl_dn0, locals.var_egidl_dn2, locals.var_egidl_dn6, locals.var_egidl_dn7, locals.var_egidl_dn10, locals.var_egidl_dn11, locals.var_egidl_dn12, locals.var_egidl_dn17,)
    }
};
        locals.var_egidl = assign24260_e33321;
        locals.var_egidl_dn0 = assign24260_e33321_d_n0;
        locals.var_egidl_dn2 = assign24260_e33321_d_n2;
        locals.var_egidl_dn6 = assign24260_e33321_d_n6;
        locals.var_egidl_dn7 = assign24260_e33321_d_n7;
        locals.var_egidl_dn10 = assign24260_e33321_d_n10;
        locals.var_egidl_dn11 = assign24260_e33321_d_n11;
        locals.var_egidl_dn12 = assign24260_e33321_d_n12;
        locals.var_egidl_dn17 = assign24260_e33321_d_n17;
        locals.var_egidl_rv = 0.0;

        let (assign24270_e33330, assign24270_e33330_d_n0, assign24270_e33330_d_n2, assign24270_e33330_d_n6, assign24270_e33330_d_n7, assign24270_e33330_d_n10, assign24270_e33330_d_n11, assign24270_e33330_d_n12, assign24270_e33330_d_n17,) = {
    if (locals.var_guard756 == 0.0) {
        let assign24270_e33327: f64 = (locals.var_egidl + 1e-50);
        let assign24270_e33328: f64 = (1.0 / assign24270_e33327);
        (assign24270_e33328, (-(locals.var_egidl_dn0 / (assign24270_e33327 * assign24270_e33327))), (-(locals.var_egidl_dn2 / (assign24270_e33327 * assign24270_e33327))), (-(locals.var_egidl_dn6 / (assign24270_e33327 * assign24270_e33327))), (-(locals.var_egidl_dn7 / (assign24270_e33327 * assign24270_e33327))), (-(locals.var_egidl_dn10 / (assign24270_e33327 * assign24270_e33327))), (-(locals.var_egidl_dn11 / (assign24270_e33327 * assign24270_e33327))), (-(locals.var_egidl_dn12 / (assign24270_e33327 * assign24270_e33327))), (-(locals.var_egidl_dn17 / (assign24270_e33327 * assign24270_e33327))),)
    } else {
        (locals.var_t3__blk752, locals.var_t3__blk752_dn0, locals.var_t3__blk752_dn2, locals.var_t3__blk752_dn6, locals.var_t3__blk752_dn7, locals.var_t3__blk752_dn10, locals.var_t3__blk752_dn11, locals.var_t3__blk752_dn12, locals.var_t3__blk752_dn17,)
    }
};
        locals.var_t3__blk752 = assign24270_e33330;
        locals.var_t3__blk752_dn0 = assign24270_e33330_d_n0;
        locals.var_t3__blk752_dn2 = assign24270_e33330_d_n2;
        locals.var_t3__blk752_dn6 = assign24270_e33330_d_n6;
        locals.var_t3__blk752_dn7 = assign24270_e33330_d_n7;
        locals.var_t3__blk752_dn10 = assign24270_e33330_d_n10;
        locals.var_t3__blk752_dn11 = assign24270_e33330_d_n11;
        locals.var_t3__blk752_dn12 = assign24270_e33330_d_n12;
        locals.var_t3__blk752_dn17 = assign24270_e33330_d_n17;
        locals.var_t3__blk752_rv = 0.0;

        let (assign24280_e33340, assign24280_e33340_d_n0, assign24280_e33340_d_n2, assign24280_e33340_d_n6, assign24280_e33340_d_n7, assign24280_e33340_d_n10, assign24280_e33340_d_n11, assign24280_e33340_d_n12, assign24280_e33340_d_n17,) = {
    if (locals.var_guard756 == 0.0) {
        let assign24280_e33334: f64 = (-p.p208);
        let assign24280_e33336: f64 = (assign24280_e33334 * locals.var_egp32);
        let assign24280_e33338: f64 = (assign24280_e33336 * locals.var_t3__blk752);
        (assign24280_e33338, (((assign24280_e33334 * locals.var_egp32_dn0) * locals.var_t3__blk752) + (assign24280_e33336 * locals.var_t3__blk752_dn0)), (((assign24280_e33334 * locals.var_egp32_dn2) * locals.var_t3__blk752) + (assign24280_e33336 * locals.var_t3__blk752_dn2)), (((assign24280_e33334 * locals.var_egp32_dn6) * locals.var_t3__blk752) + (assign24280_e33336 * locals.var_t3__blk752_dn6)), (((assign24280_e33334 * locals.var_egp32_dn7) * locals.var_t3__blk752) + (assign24280_e33336 * locals.var_t3__blk752_dn7)), (((assign24280_e33334 * locals.var_egp32_dn10) * locals.var_t3__blk752) + (assign24280_e33336 * locals.var_t3__blk752_dn10)), (((assign24280_e33334 * locals.var_egp32_dn11) * locals.var_t3__blk752) + (assign24280_e33336 * locals.var_t3__blk752_dn11)), (((assign24280_e33334 * locals.var_egp32_dn12) * locals.var_t3__blk752) + (assign24280_e33336 * locals.var_t3__blk752_dn12)), (((assign24280_e33334 * locals.var_egp32_dn17) * locals.var_t3__blk752) + (assign24280_e33336 * locals.var_t3__blk752_dn17)),)
    } else {
        (locals.var_t0__blk753, locals.var_t0__blk753_dn0, locals.var_t0__blk753_dn2, locals.var_t0__blk753_dn6, locals.var_t0__blk753_dn7, locals.var_t0__blk753_dn10, locals.var_t0__blk753_dn11, locals.var_t0__blk753_dn12, locals.var_t0__blk753_dn17,)
    }
};
        locals.var_t0__blk753 = assign24280_e33340;
        locals.var_t0__blk753_dn0 = assign24280_e33340_d_n0;
        locals.var_t0__blk753_dn2 = assign24280_e33340_d_n2;
        locals.var_t0__blk753_dn6 = assign24280_e33340_d_n6;
        locals.var_t0__blk753_dn7 = assign24280_e33340_d_n7;
        locals.var_t0__blk753_dn10 = assign24280_e33340_d_n10;
        locals.var_t0__blk753_dn11 = assign24280_e33340_d_n11;
        locals.var_t0__blk753_dn12 = assign24280_e33340_d_n12;
        locals.var_t0__blk753_dn17 = assign24280_e33340_d_n17;
        locals.var_t0__blk753_rv = 0.0;

        let assign24290_e33343: f64 = (-34.0);
        let assign24290_e33344: f64 = if locals.var_t0__blk753 < assign24290_e33343 { 1.0 } else { 0.0 };
        locals.var_guard758 = assign24290_e33344;
        locals.var_guard758_rv = 0.0;

        let (assign24310_e33360, assign24310_e33360_d_n0, assign24310_e33360_d_n2, assign24310_e33360_d_n6, assign24310_e33360_d_n7, assign24310_e33360_d_n10, assign24310_e33360_d_n11, assign24310_e33360_d_n12, assign24310_e33360_d_n17,) = {
    if ((locals.var_guard756 == 0.0) && (locals.var_guard758 == 0.0)) {
        let assign24310_e33358: f64 = (locals.var_t0__blk753).exp();
        (assign24310_e33358, (assign24310_e33358 * locals.var_t0__blk753_dn0), (assign24310_e33358 * locals.var_t0__blk753_dn2), (assign24310_e33358 * locals.var_t0__blk753_dn6), (assign24310_e33358 * locals.var_t0__blk753_dn7), (assign24310_e33358 * locals.var_t0__blk753_dn10), (assign24310_e33358 * locals.var_t0__blk753_dn11), (assign24310_e33358 * locals.var_t0__blk753_dn12), (assign24310_e33358 * locals.var_t0__blk753_dn17),)
    } else {
        (locals.var_t1__blk749, locals.var_t1__blk749_dn0, locals.var_t1__blk749_dn2, locals.var_t1__blk749_dn6, locals.var_t1__blk749_dn7, locals.var_t1__blk749_dn10, locals.var_t1__blk749_dn11, locals.var_t1__blk749_dn12, locals.var_t1__blk749_dn17,)
    }
};
        locals.var_t1__blk749 = assign24310_e33360;
        locals.var_t1__blk749_dn0 = assign24310_e33360_d_n0;
        locals.var_t1__blk749_dn2 = assign24310_e33360_d_n2;
        locals.var_t1__blk749_dn6 = assign24310_e33360_d_n6;
        locals.var_t1__blk749_dn7 = assign24310_e33360_d_n7;
        locals.var_t1__blk749_dn10 = assign24310_e33360_d_n10;
        locals.var_t1__blk749_dn11 = assign24310_e33360_d_n11;
        locals.var_t1__blk749_dn12 = assign24310_e33360_d_n12;
        locals.var_t1__blk749_dn17 = assign24310_e33360_d_n17;
        locals.var_t1__blk749_rv = 0.0;

        let (assign24320_e33374, assign24320_e33374_d_n0, assign24320_e33374_d_n2, assign24320_e33374_d_n6, assign24320_e33374_d_n7, assign24320_e33374_d_n10, assign24320_e33374_d_n11, assign24320_e33374_d_n12, assign24320_e33374_d_n17,) = {
    if ((locals.var_guard756 == 0.0) && (locals.var_guard758 == 0.0)) {
        let assign24320_e33368: f64 = (p.p207 / locals.var_egp12);
        let assign24320_e33370: f64 = (assign24320_e33368 * 1.6021918e-19);
        let assign24320_e33372: f64 = (assign24320_e33370 * locals.var_weff_nf);
        (assign24320_e33372, (((-((p.p207 * locals.var_egp12_dn0) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((p.p207 * locals.var_egp12_dn2) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((p.p207 * locals.var_egp12_dn6) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((p.p207 * locals.var_egp12_dn7) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((p.p207 * locals.var_egp12_dn10) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((p.p207 * locals.var_egp12_dn11) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((p.p207 * locals.var_egp12_dn12) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((p.p207 * locals.var_egp12_dn17) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf),)
    } else {
        (locals.var_t2__blk750, locals.var_t2__blk750_dn0, locals.var_t2__blk750_dn2, locals.var_t2__blk750_dn6, locals.var_t2__blk750_dn7, locals.var_t2__blk750_dn10, locals.var_t2__blk750_dn11, locals.var_t2__blk750_dn12, locals.var_t2__blk750_dn17,)
    }
};
        locals.var_t2__blk750 = assign24320_e33374;
        locals.var_t2__blk750_dn0 = assign24320_e33374_d_n0;
        locals.var_t2__blk750_dn2 = assign24320_e33374_d_n2;
        locals.var_t2__blk750_dn6 = assign24320_e33374_d_n6;
        locals.var_t2__blk750_dn7 = assign24320_e33374_d_n7;
        locals.var_t2__blk750_dn10 = assign24320_e33374_d_n10;
        locals.var_t2__blk750_dn11 = assign24320_e33374_d_n11;
        locals.var_t2__blk750_dn12 = assign24320_e33374_d_n12;
        locals.var_t2__blk750_dn17 = assign24320_e33374_d_n17;
        locals.var_t2__blk750_rv = 0.0;

        let (assign24340_e33395, assign24340_e33395_d_n0, assign24340_e33395_d_n2, assign24340_e33395_d_n6, assign24340_e33395_d_n7, assign24340_e33395_d_n10, assign24340_e33395_d_n11, assign24340_e33395_d_n12, assign24340_e33395_d_n17,) = {
    if (locals.var_guard756 == 0.0) {
        let assign24340_e33393: f64 = (locals.var_vds - locals.var_vbsp);
        (assign24340_e33393, (locals.var_vds_dn0 - locals.var_vbsp_dn0), (locals.var_vds_dn2 - locals.var_vbsp_dn2), (locals.var_vds_dn6 - locals.var_vbsp_dn6), (locals.var_vds_dn7 - locals.var_vbsp_dn7), (locals.var_vds_dn10 - locals.var_vbsp_dn10), (locals.var_vds_dn11 - locals.var_vbsp_dn11), (locals.var_vds_dn12 - locals.var_vbsp_dn12), (locals.var_vds_dn17 - locals.var_vbsp_dn17),)
    } else {
        (locals.var_vdb, locals.var_vdb_dn0, locals.var_vdb_dn2, locals.var_vdb_dn6, locals.var_vdb_dn7, locals.var_vdb_dn10, locals.var_vdb_dn11, locals.var_vdb_dn12, locals.var_vdb_dn17,)
    }
};
        locals.var_vdb = assign24340_e33395;
        locals.var_vdb_dn0 = assign24340_e33395_d_n0;
        locals.var_vdb_dn2 = assign24340_e33395_d_n2;
        locals.var_vdb_dn6 = assign24340_e33395_d_n6;
        locals.var_vdb_dn7 = assign24340_e33395_d_n7;
        locals.var_vdb_dn10 = assign24340_e33395_d_n10;
        locals.var_vdb_dn11 = assign24340_e33395_d_n11;
        locals.var_vdb_dn12 = assign24340_e33395_d_n12;
        locals.var_vdb_dn17 = assign24340_e33395_d_n17;
        locals.var_vdb_rv = 0.0;

        let assign24350_e33398: f64 = if locals.var_vdb > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard759 = assign24350_e33398;
        locals.var_guard759_rv = 0.0;

        let (assign24360_e33407, assign24360_e33407_d_n0, assign24360_e33407_d_n2, assign24360_e33407_d_n6, assign24360_e33407_d_n7, assign24360_e33407_d_n10, assign24360_e33407_d_n11, assign24360_e33407_d_n12, assign24360_e33407_d_n17,) = {
    if ((locals.var_guard756 == 0.0) && (locals.var_guard759 != 0.0)) {
        let assign24360_e33405: f64 = (locals.var_vdb * locals.var_vdb);
        (assign24360_e33405, ((locals.var_vdb_dn0 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn0)), ((locals.var_vdb_dn2 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn2)), ((locals.var_vdb_dn6 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn6)), ((locals.var_vdb_dn7 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn7)), ((locals.var_vdb_dn10 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn10)), ((locals.var_vdb_dn11 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn11)), ((locals.var_vdb_dn12 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn12)), ((locals.var_vdb_dn17 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn17)),)
    } else {
        (locals.var_t2__blk750, locals.var_t2__blk750_dn0, locals.var_t2__blk750_dn2, locals.var_t2__blk750_dn6, locals.var_t2__blk750_dn7, locals.var_t2__blk750_dn10, locals.var_t2__blk750_dn11, locals.var_t2__blk750_dn12, locals.var_t2__blk750_dn17,)
    }
};
        locals.var_t2__blk750 = assign24360_e33407;
        locals.var_t2__blk750_dn0 = assign24360_e33407_d_n0;
        locals.var_t2__blk750_dn2 = assign24360_e33407_d_n2;
        locals.var_t2__blk750_dn6 = assign24360_e33407_d_n6;
        locals.var_t2__blk750_dn7 = assign24360_e33407_d_n7;
        locals.var_t2__blk750_dn10 = assign24360_e33407_d_n10;
        locals.var_t2__blk750_dn11 = assign24360_e33407_d_n11;
        locals.var_t2__blk750_dn12 = assign24360_e33407_d_n12;
        locals.var_t2__blk750_dn17 = assign24360_e33407_d_n17;
        locals.var_t2__blk750_rv = 0.0;

        let (assign24370_e33416, assign24370_e33416_d_n0, assign24370_e33416_d_n2, assign24370_e33416_d_n6, assign24370_e33416_d_n7, assign24370_e33416_d_n10, assign24370_e33416_d_n11, assign24370_e33416_d_n12, assign24370_e33416_d_n17,) = {
    if ((locals.var_guard756 == 0.0) && (locals.var_guard759 != 0.0)) {
        let assign24370_e33414: f64 = (locals.var_t2__blk750 * locals.var_vdb);
        (assign24370_e33414, ((locals.var_t2__blk750_dn0 * locals.var_vdb) + (locals.var_t2__blk750 * locals.var_vdb_dn0)), ((locals.var_t2__blk750_dn2 * locals.var_vdb) + (locals.var_t2__blk750 * locals.var_vdb_dn2)), ((locals.var_t2__blk750_dn6 * locals.var_vdb) + (locals.var_t2__blk750 * locals.var_vdb_dn6)), ((locals.var_t2__blk750_dn7 * locals.var_vdb) + (locals.var_t2__blk750 * locals.var_vdb_dn7)), ((locals.var_t2__blk750_dn10 * locals.var_vdb) + (locals.var_t2__blk750 * locals.var_vdb_dn10)), ((locals.var_t2__blk750_dn11 * locals.var_vdb) + (locals.var_t2__blk750 * locals.var_vdb_dn11)), ((locals.var_t2__blk750_dn12 * locals.var_vdb) + (locals.var_t2__blk750 * locals.var_vdb_dn12)), ((locals.var_t2__blk750_dn17 * locals.var_vdb) + (locals.var_t2__blk750 * locals.var_vdb_dn17)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign24370_e33416;
        locals.var_t4_dn0 = assign24370_e33416_d_n0;
        locals.var_t4_dn2 = assign24370_e33416_d_n2;
        locals.var_t4_dn6 = assign24370_e33416_d_n6;
        locals.var_t4_dn7 = assign24370_e33416_d_n7;
        locals.var_t4_dn10 = assign24370_e33416_d_n10;
        locals.var_t4_dn11 = assign24370_e33416_d_n11;
        locals.var_t4_dn12 = assign24370_e33416_d_n12;
        locals.var_t4_dn17 = assign24370_e33416_d_n17;
        locals.var_t4_rv = 0.0;

        let (assign24380_e33425, assign24380_e33425_d_n0, assign24380_e33425_d_n2, assign24380_e33425_d_n6, assign24380_e33425_d_n7, assign24380_e33425_d_n10, assign24380_e33425_d_n11, assign24380_e33425_d_n12, assign24380_e33425_d_n17,) = {
    if ((locals.var_guard756 == 0.0) && (locals.var_guard759 != 0.0)) {
        let assign24380_e33423: f64 = (locals.var_t4 + p.p212);
        (assign24380_e33423, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    } else {
        (locals.var_t0__blk753, locals.var_t0__blk753_dn0, locals.var_t0__blk753_dn2, locals.var_t0__blk753_dn6, locals.var_t0__blk753_dn7, locals.var_t0__blk753_dn10, locals.var_t0__blk753_dn11, locals.var_t0__blk753_dn12, locals.var_t0__blk753_dn17,)
    }
};
        locals.var_t0__blk753 = assign24380_e33425;
        locals.var_t0__blk753_dn0 = assign24380_e33425_d_n0;
        locals.var_t0__blk753_dn2 = assign24380_e33425_d_n2;
        locals.var_t0__blk753_dn6 = assign24380_e33425_d_n6;
        locals.var_t0__blk753_dn7 = assign24380_e33425_d_n7;
        locals.var_t0__blk753_dn10 = assign24380_e33425_d_n10;
        locals.var_t0__blk753_dn11 = assign24380_e33425_d_n11;
        locals.var_t0__blk753_dn12 = assign24380_e33425_d_n12;
        locals.var_t0__blk753_dn17 = assign24380_e33425_d_n17;
        locals.var_t0__blk753_rv = 0.0;

        let assign24420_e33454: f64 = if p.p28 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard767 = assign24420_e33454;
        locals.var_guard767_rv = 0.0;

        let (assign24440_e33478, assign24440_e33478_d_n0, assign24440_e33478_d_n2, assign24440_e33478_d_n6, assign24440_e33478_d_n7, assign24440_e33478_d_n10, assign24440_e33478_d_n11, assign24440_e33478_d_n12, assign24440_e33478_d_n17,) = {
    if (locals.var_guard767 == 0.0) {
        let assign24440_e33463: f64 = (-locals.var_vds);
        let assign24440_e33465: f64 = (assign24440_e33463 + p.p210);
        let assign24440_e33466: f64 = (p.p209 * assign24440_e33465);
        let assign24440_e33469: f64 = (locals.var_vgs - locals.var_vds);
        let assign24440_e33470: f64 = (assign24440_e33466 - assign24440_e33469);
        let assign24440_e33473: f64 = (locals.var_dvthsc + locals.var_dvthlp);
        let assign24440_e33475: f64 = (assign24440_e33473 * p.p211);
        let assign24440_e33476: f64 = (assign24440_e33470 + assign24440_e33475);
        (assign24440_e33476, (((p.p209 * (-locals.var_vds_dn0)) - (-locals.var_vds_dn0)) + ((locals.var_dvthsc_dn0 + locals.var_dvthlp_dn0) * p.p211)), (((p.p209 * (-locals.var_vds_dn2)) - (-locals.var_vds_dn2)) + ((locals.var_dvthsc_dn2 + locals.var_dvthlp_dn2) * p.p211)), (((p.p209 * (-locals.var_vds_dn6)) - (locals.var_vgs_dn6 - locals.var_vds_dn6)) + ((locals.var_dvthsc_dn6 + locals.var_dvthlp_dn6) * p.p211)), (((p.p209 * (-locals.var_vds_dn7)) - (locals.var_vgs_dn7 - locals.var_vds_dn7)) + ((locals.var_dvthsc_dn7 + locals.var_dvthlp_dn7) * p.p211)), (((p.p209 * (-locals.var_vds_dn10)) - (-locals.var_vds_dn10)) + ((locals.var_dvthsc_dn10 + locals.var_dvthlp_dn10) * p.p211)), (((p.p209 * (-locals.var_vds_dn11)) - (locals.var_vgs_dn11 - locals.var_vds_dn11)) + ((locals.var_dvthsc_dn11 + locals.var_dvthlp_dn11) * p.p211)), (((p.p209 * (-locals.var_vds_dn12)) - (-locals.var_vds_dn12)) + ((locals.var_dvthsc_dn12 + locals.var_dvthlp_dn12) * p.p211)), (((p.p209 * (-locals.var_vds_dn17)) - (-locals.var_vds_dn17)) + ((locals.var_dvthsc_dn17 + locals.var_dvthlp_dn17) * p.p211)),)
    } else {
        (locals.var_t1__blk760, locals.var_t1__blk760_dn0, locals.var_t1__blk760_dn2, locals.var_t1__blk760_dn6, locals.var_t1__blk760_dn7, locals.var_t1__blk760_dn10, locals.var_t1__blk760_dn11, locals.var_t1__blk760_dn12, locals.var_t1__blk760_dn17,)
    }
};
        locals.var_t1__blk760 = assign24440_e33478;
        locals.var_t1__blk760_dn0 = assign24440_e33478_d_n0;
        locals.var_t1__blk760_dn2 = assign24440_e33478_d_n2;
        locals.var_t1__blk760_dn6 = assign24440_e33478_d_n6;
        locals.var_t1__blk760_dn7 = assign24440_e33478_d_n7;
        locals.var_t1__blk760_dn10 = assign24440_e33478_d_n10;
        locals.var_t1__blk760_dn11 = assign24440_e33478_d_n11;
        locals.var_t1__blk760_dn12 = assign24440_e33478_d_n12;
        locals.var_t1__blk760_dn17 = assign24440_e33478_d_n17;
        locals.var_t1__blk760_rv = 0.0;

        let (assign24450_e33485, assign24450_e33485_d_n0, assign24450_e33485_d_n2, assign24450_e33485_d_n6, assign24450_e33485_d_n7, assign24450_e33485_d_n10, assign24450_e33485_d_n11, assign24450_e33485_d_n12, assign24450_e33485_d_n17,) = {
    if (locals.var_guard767 == 0.0) {
        let assign24450_e33483: f64 = (1.0 / locals.var_tfox0);
        (assign24450_e33483, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk761, locals.var_t2__blk761_dn0, locals.var_t2__blk761_dn2, locals.var_t2__blk761_dn6, locals.var_t2__blk761_dn7, locals.var_t2__blk761_dn10, locals.var_t2__blk761_dn11, locals.var_t2__blk761_dn12, locals.var_t2__blk761_dn17,)
    }
};
        locals.var_t2__blk761 = assign24450_e33485;
        locals.var_t2__blk761_dn0 = assign24450_e33485_d_n0;
        locals.var_t2__blk761_dn2 = assign24450_e33485_d_n2;
        locals.var_t2__blk761_dn6 = assign24450_e33485_d_n6;
        locals.var_t2__blk761_dn7 = assign24450_e33485_d_n7;
        locals.var_t2__blk761_dn10 = assign24450_e33485_d_n10;
        locals.var_t2__blk761_dn11 = assign24450_e33485_d_n11;
        locals.var_t2__blk761_dn12 = assign24450_e33485_d_n12;
        locals.var_t2__blk761_dn17 = assign24450_e33485_d_n17;
        locals.var_t2__blk761_rv = 0.0;

        let (assign24460_e33492, assign24460_e33492_d_n0, assign24460_e33492_d_n2, assign24460_e33492_d_n6, assign24460_e33492_d_n7, assign24460_e33492_d_n10, assign24460_e33492_d_n11, assign24460_e33492_d_n12, assign24460_e33492_d_n17,) = {
    if (locals.var_guard767 == 0.0) {
        let assign24460_e33490: f64 = (locals.var_t1__blk760 * locals.var_t2__blk761);
        (assign24460_e33490, ((locals.var_t1__blk760_dn0 * locals.var_t2__blk761) + (locals.var_t1__blk760 * locals.var_t2__blk761_dn0)), ((locals.var_t1__blk760_dn2 * locals.var_t2__blk761) + (locals.var_t1__blk760 * locals.var_t2__blk761_dn2)), ((locals.var_t1__blk760_dn6 * locals.var_t2__blk761) + (locals.var_t1__blk760 * locals.var_t2__blk761_dn6)), ((locals.var_t1__blk760_dn7 * locals.var_t2__blk761) + (locals.var_t1__blk760 * locals.var_t2__blk761_dn7)), ((locals.var_t1__blk760_dn10 * locals.var_t2__blk761) + (locals.var_t1__blk760 * locals.var_t2__blk761_dn10)), ((locals.var_t1__blk760_dn11 * locals.var_t2__blk761) + (locals.var_t1__blk760 * locals.var_t2__blk761_dn11)), ((locals.var_t1__blk760_dn12 * locals.var_t2__blk761) + (locals.var_t1__blk760 * locals.var_t2__blk761_dn12)), ((locals.var_t1__blk760_dn17 * locals.var_t2__blk761) + (locals.var_t1__blk760 * locals.var_t2__blk761_dn17)),)
    } else {
        (locals.var_e1__blk762, locals.var_e1__blk762_dn0, locals.var_e1__blk762_dn2, locals.var_e1__blk762_dn6, locals.var_e1__blk762_dn7, locals.var_e1__blk762_dn10, locals.var_e1__blk762_dn11, locals.var_e1__blk762_dn12, locals.var_e1__blk762_dn17,)
    }
};
        locals.var_e1__blk762 = assign24460_e33492;
        locals.var_e1__blk762_dn0 = assign24460_e33492_d_n0;
        locals.var_e1__blk762_dn2 = assign24460_e33492_d_n2;
        locals.var_e1__blk762_dn6 = assign24460_e33492_d_n6;
        locals.var_e1__blk762_dn7 = assign24460_e33492_d_n7;
        locals.var_e1__blk762_dn10 = assign24460_e33492_d_n10;
        locals.var_e1__blk762_dn11 = assign24460_e33492_d_n11;
        locals.var_e1__blk762_dn12 = assign24460_e33492_d_n12;
        locals.var_e1__blk762_dn17 = assign24460_e33492_d_n17;
        locals.var_e1__blk762_rv = 0.0;

        let (assign24470_e33506, assign24470_e33506_d_n0, assign24470_e33506_d_n2, assign24470_e33506_d_n6, assign24470_e33506_d_n7, assign24470_e33506_d_n10, assign24470_e33506_d_n11, assign24470_e33506_d_n12, assign24470_e33506_d_n17,) = {
    if (locals.var_guard767 == 0.0) {
        let assign24470_e33497: f64 = (locals.var_e1__blk762 * locals.var_e1__blk762);
        let assign24470_e33500: f64 = (4.0 * 0.01);
        let assign24470_e33502: f64 = (assign24470_e33500 * 0.01);
        let assign24470_e33503: f64 = (assign24470_e33497 + assign24470_e33502);
        let assign24470_e33504: f64 = (assign24470_e33503).sqrt();
        (assign24470_e33504, (((locals.var_e1__blk762_dn0 * locals.var_e1__blk762) + (locals.var_e1__blk762 * locals.var_e1__blk762_dn0)) / (2.0 * assign24470_e33504)), (((locals.var_e1__blk762_dn2 * locals.var_e1__blk762) + (locals.var_e1__blk762 * locals.var_e1__blk762_dn2)) / (2.0 * assign24470_e33504)), (((locals.var_e1__blk762_dn6 * locals.var_e1__blk762) + (locals.var_e1__blk762 * locals.var_e1__blk762_dn6)) / (2.0 * assign24470_e33504)), (((locals.var_e1__blk762_dn7 * locals.var_e1__blk762) + (locals.var_e1__blk762 * locals.var_e1__blk762_dn7)) / (2.0 * assign24470_e33504)), (((locals.var_e1__blk762_dn10 * locals.var_e1__blk762) + (locals.var_e1__blk762 * locals.var_e1__blk762_dn10)) / (2.0 * assign24470_e33504)), (((locals.var_e1__blk762_dn11 * locals.var_e1__blk762) + (locals.var_e1__blk762 * locals.var_e1__blk762_dn11)) / (2.0 * assign24470_e33504)), (((locals.var_e1__blk762_dn12 * locals.var_e1__blk762) + (locals.var_e1__blk762 * locals.var_e1__blk762_dn12)) / (2.0 * assign24470_e33504)), (((locals.var_e1__blk762_dn17 * locals.var_e1__blk762) + (locals.var_e1__blk762 * locals.var_e1__blk762_dn17)) / (2.0 * assign24470_e33504)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign24470_e33506;
        locals.var_tmf1_dn0 = assign24470_e33506_d_n0;
        locals.var_tmf1_dn2 = assign24470_e33506_d_n2;
        locals.var_tmf1_dn6 = assign24470_e33506_d_n6;
        locals.var_tmf1_dn7 = assign24470_e33506_d_n7;
        locals.var_tmf1_dn10 = assign24470_e33506_d_n10;
        locals.var_tmf1_dn11 = assign24470_e33506_d_n11;
        locals.var_tmf1_dn12 = assign24470_e33506_d_n12;
        locals.var_tmf1_dn17 = assign24470_e33506_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign24480_e33519, assign24480_e33519_d_n0, assign24480_e33519_d_n2, assign24480_e33519_d_n6, assign24480_e33519_d_n7, assign24480_e33519_d_n10, assign24480_e33519_d_n11, assign24480_e33519_d_n12, assign24480_e33519_d_n17,) = {
    if (locals.var_guard767 == 0.0) {
        let assign24480_e33512: f64 = (locals.var_e1__blk762 + locals.var_tmf1);
        let assign24480_e33513: f64 = (0.5 * assign24480_e33512);
        let assign24480_e33516: f64 = (1e-10 * 0.01);
        let assign24480_e33517: f64 = (assign24480_e33513 + assign24480_e33516);
        (assign24480_e33517, (0.5 * (locals.var_e1__blk762_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_e1__blk762_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_e1__blk762_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_e1__blk762_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_e1__blk762_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_e1__blk762_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_e1__blk762_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_e1__blk762_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_egisl, locals.var_egisl_dn0, locals.var_egisl_dn2, locals.var_egisl_dn6, locals.var_egisl_dn7, locals.var_egisl_dn10, locals.var_egisl_dn11, locals.var_egisl_dn12, locals.var_egisl_dn17,)
    }
};
        locals.var_egisl = assign24480_e33519;
        locals.var_egisl_dn0 = assign24480_e33519_d_n0;
        locals.var_egisl_dn2 = assign24480_e33519_d_n2;
        locals.var_egisl_dn6 = assign24480_e33519_d_n6;
        locals.var_egisl_dn7 = assign24480_e33519_d_n7;
        locals.var_egisl_dn10 = assign24480_e33519_d_n10;
        locals.var_egisl_dn11 = assign24480_e33519_d_n11;
        locals.var_egisl_dn12 = assign24480_e33519_d_n12;
        locals.var_egisl_dn17 = assign24480_e33519_d_n17;
        locals.var_egisl_rv = 0.0;

        let assign24490_e33522: f64 = if locals.var_egisl < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard768 = assign24490_e33522;
        locals.var_guard768_rv = 0.0;

        let (assign24500_e33529, assign24500_e33529_d_n0, assign24500_e33529_d_n2, assign24500_e33529_d_n6, assign24500_e33529_d_n7, assign24500_e33529_d_n10, assign24500_e33529_d_n11, assign24500_e33529_d_n12, assign24500_e33529_d_n17,) = {
    if ((locals.var_guard767 == 0.0) && (locals.var_guard768 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_egisl, locals.var_egisl_dn0, locals.var_egisl_dn2, locals.var_egisl_dn6, locals.var_egisl_dn7, locals.var_egisl_dn10, locals.var_egisl_dn11, locals.var_egisl_dn12, locals.var_egisl_dn17,)
    }
};
        locals.var_egisl = assign24500_e33529;
        locals.var_egisl_dn0 = assign24500_e33529_d_n0;
        locals.var_egisl_dn2 = assign24500_e33529_d_n2;
        locals.var_egisl_dn6 = assign24500_e33529_d_n6;
        locals.var_egisl_dn7 = assign24500_e33529_d_n7;
        locals.var_egisl_dn10 = assign24500_e33529_d_n10;
        locals.var_egisl_dn11 = assign24500_e33529_d_n11;
        locals.var_egisl_dn12 = assign24500_e33529_d_n12;
        locals.var_egisl_dn17 = assign24500_e33529_d_n17;
        locals.var_egisl_rv = 0.0;

        let (assign24510_e33538, assign24510_e33538_d_n0, assign24510_e33538_d_n2, assign24510_e33538_d_n6, assign24510_e33538_d_n7, assign24510_e33538_d_n10, assign24510_e33538_d_n11, assign24510_e33538_d_n12, assign24510_e33538_d_n17,) = {
    if (locals.var_guard767 == 0.0) {
        let assign24510_e33535: f64 = (locals.var_egisl + 1e-50);
        let assign24510_e33536: f64 = (1.0 / assign24510_e33535);
        (assign24510_e33536, (-(locals.var_egisl_dn0 / (assign24510_e33535 * assign24510_e33535))), (-(locals.var_egisl_dn2 / (assign24510_e33535 * assign24510_e33535))), (-(locals.var_egisl_dn6 / (assign24510_e33535 * assign24510_e33535))), (-(locals.var_egisl_dn7 / (assign24510_e33535 * assign24510_e33535))), (-(locals.var_egisl_dn10 / (assign24510_e33535 * assign24510_e33535))), (-(locals.var_egisl_dn11 / (assign24510_e33535 * assign24510_e33535))), (-(locals.var_egisl_dn12 / (assign24510_e33535 * assign24510_e33535))), (-(locals.var_egisl_dn17 / (assign24510_e33535 * assign24510_e33535))),)
    } else {
        (locals.var_t3__blk763, locals.var_t3__blk763_dn0, locals.var_t3__blk763_dn2, locals.var_t3__blk763_dn6, locals.var_t3__blk763_dn7, locals.var_t3__blk763_dn10, locals.var_t3__blk763_dn11, locals.var_t3__blk763_dn12, locals.var_t3__blk763_dn17,)
    }
};
        locals.var_t3__blk763 = assign24510_e33538;
        locals.var_t3__blk763_dn0 = assign24510_e33538_d_n0;
        locals.var_t3__blk763_dn2 = assign24510_e33538_d_n2;
        locals.var_t3__blk763_dn6 = assign24510_e33538_d_n6;
        locals.var_t3__blk763_dn7 = assign24510_e33538_d_n7;
        locals.var_t3__blk763_dn10 = assign24510_e33538_d_n10;
        locals.var_t3__blk763_dn11 = assign24510_e33538_d_n11;
        locals.var_t3__blk763_dn12 = assign24510_e33538_d_n12;
        locals.var_t3__blk763_dn17 = assign24510_e33538_d_n17;
        locals.var_t3__blk763_rv = 0.0;

        let (assign24520_e33548, assign24520_e33548_d_n0, assign24520_e33548_d_n2, assign24520_e33548_d_n6, assign24520_e33548_d_n7, assign24520_e33548_d_n10, assign24520_e33548_d_n11, assign24520_e33548_d_n12, assign24520_e33548_d_n17,) = {
    if (locals.var_guard767 == 0.0) {
        let assign24520_e33542: f64 = (-p.p208);
        let assign24520_e33544: f64 = (assign24520_e33542 * locals.var_egp32);
        let assign24520_e33546: f64 = (assign24520_e33544 * locals.var_t3__blk763);
        (assign24520_e33546, (((assign24520_e33542 * locals.var_egp32_dn0) * locals.var_t3__blk763) + (assign24520_e33544 * locals.var_t3__blk763_dn0)), (((assign24520_e33542 * locals.var_egp32_dn2) * locals.var_t3__blk763) + (assign24520_e33544 * locals.var_t3__blk763_dn2)), (((assign24520_e33542 * locals.var_egp32_dn6) * locals.var_t3__blk763) + (assign24520_e33544 * locals.var_t3__blk763_dn6)), (((assign24520_e33542 * locals.var_egp32_dn7) * locals.var_t3__blk763) + (assign24520_e33544 * locals.var_t3__blk763_dn7)), (((assign24520_e33542 * locals.var_egp32_dn10) * locals.var_t3__blk763) + (assign24520_e33544 * locals.var_t3__blk763_dn10)), (((assign24520_e33542 * locals.var_egp32_dn11) * locals.var_t3__blk763) + (assign24520_e33544 * locals.var_t3__blk763_dn11)), (((assign24520_e33542 * locals.var_egp32_dn12) * locals.var_t3__blk763) + (assign24520_e33544 * locals.var_t3__blk763_dn12)), (((assign24520_e33542 * locals.var_egp32_dn17) * locals.var_t3__blk763) + (assign24520_e33544 * locals.var_t3__blk763_dn17)),)
    } else {
        (locals.var_t0__blk764, locals.var_t0__blk764_dn0, locals.var_t0__blk764_dn2, locals.var_t0__blk764_dn6, locals.var_t0__blk764_dn7, locals.var_t0__blk764_dn10, locals.var_t0__blk764_dn11, locals.var_t0__blk764_dn12, locals.var_t0__blk764_dn17,)
    }
};
        locals.var_t0__blk764 = assign24520_e33548;
        locals.var_t0__blk764_dn0 = assign24520_e33548_d_n0;
        locals.var_t0__blk764_dn2 = assign24520_e33548_d_n2;
        locals.var_t0__blk764_dn6 = assign24520_e33548_d_n6;
        locals.var_t0__blk764_dn7 = assign24520_e33548_d_n7;
        locals.var_t0__blk764_dn10 = assign24520_e33548_d_n10;
        locals.var_t0__blk764_dn11 = assign24520_e33548_d_n11;
        locals.var_t0__blk764_dn12 = assign24520_e33548_d_n12;
        locals.var_t0__blk764_dn17 = assign24520_e33548_d_n17;
        locals.var_t0__blk764_rv = 0.0;

        let assign24530_e33551: f64 = (-34.0);
        let assign24530_e33552: f64 = if locals.var_t0__blk764 < assign24530_e33551 { 1.0 } else { 0.0 };
        locals.var_guard769 = assign24530_e33552;
        locals.var_guard769_rv = 0.0;

        let (assign24550_e33568, assign24550_e33568_d_n0, assign24550_e33568_d_n2, assign24550_e33568_d_n6, assign24550_e33568_d_n7, assign24550_e33568_d_n10, assign24550_e33568_d_n11, assign24550_e33568_d_n12, assign24550_e33568_d_n17,) = {
    if ((locals.var_guard767 == 0.0) && (locals.var_guard769 == 0.0)) {
        let assign24550_e33566: f64 = (locals.var_t0__blk764).exp();
        (assign24550_e33566, (assign24550_e33566 * locals.var_t0__blk764_dn0), (assign24550_e33566 * locals.var_t0__blk764_dn2), (assign24550_e33566 * locals.var_t0__blk764_dn6), (assign24550_e33566 * locals.var_t0__blk764_dn7), (assign24550_e33566 * locals.var_t0__blk764_dn10), (assign24550_e33566 * locals.var_t0__blk764_dn11), (assign24550_e33566 * locals.var_t0__blk764_dn12), (assign24550_e33566 * locals.var_t0__blk764_dn17),)
    } else {
        (locals.var_t1__blk760, locals.var_t1__blk760_dn0, locals.var_t1__blk760_dn2, locals.var_t1__blk760_dn6, locals.var_t1__blk760_dn7, locals.var_t1__blk760_dn10, locals.var_t1__blk760_dn11, locals.var_t1__blk760_dn12, locals.var_t1__blk760_dn17,)
    }
};
        locals.var_t1__blk760 = assign24550_e33568;
        locals.var_t1__blk760_dn0 = assign24550_e33568_d_n0;
        locals.var_t1__blk760_dn2 = assign24550_e33568_d_n2;
        locals.var_t1__blk760_dn6 = assign24550_e33568_d_n6;
        locals.var_t1__blk760_dn7 = assign24550_e33568_d_n7;
        locals.var_t1__blk760_dn10 = assign24550_e33568_d_n10;
        locals.var_t1__blk760_dn11 = assign24550_e33568_d_n11;
        locals.var_t1__blk760_dn12 = assign24550_e33568_d_n12;
        locals.var_t1__blk760_dn17 = assign24550_e33568_d_n17;
        locals.var_t1__blk760_rv = 0.0;

        let (assign24560_e33578, assign24560_e33578_d_n0, assign24560_e33578_d_n2, assign24560_e33578_d_n6, assign24560_e33578_d_n7, assign24560_e33578_d_n10, assign24560_e33578_d_n11, assign24560_e33578_d_n12, assign24560_e33578_d_n17,) = {
    if ((locals.var_guard767 == 0.0) && (locals.var_guard769 == 0.0)) {
        let assign24560_e33576: f64 = (1.0 / locals.var_egp12);
        (assign24560_e33576, (-(locals.var_egp12_dn0 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn2 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn6 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn7 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn10 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn11 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn12 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn17 / (locals.var_egp12 * locals.var_egp12))),)
    } else {
        (locals.var_t3__blk763, locals.var_t3__blk763_dn0, locals.var_t3__blk763_dn2, locals.var_t3__blk763_dn6, locals.var_t3__blk763_dn7, locals.var_t3__blk763_dn10, locals.var_t3__blk763_dn11, locals.var_t3__blk763_dn12, locals.var_t3__blk763_dn17,)
    }
};
        locals.var_t3__blk763 = assign24560_e33578;
        locals.var_t3__blk763_dn0 = assign24560_e33578_d_n0;
        locals.var_t3__blk763_dn2 = assign24560_e33578_d_n2;
        locals.var_t3__blk763_dn6 = assign24560_e33578_d_n6;
        locals.var_t3__blk763_dn7 = assign24560_e33578_d_n7;
        locals.var_t3__blk763_dn10 = assign24560_e33578_d_n10;
        locals.var_t3__blk763_dn11 = assign24560_e33578_d_n11;
        locals.var_t3__blk763_dn12 = assign24560_e33578_d_n12;
        locals.var_t3__blk763_dn17 = assign24560_e33578_d_n17;
        locals.var_t3__blk763_rv = 0.0;

        let (assign24570_e33592, assign24570_e33592_d_n0, assign24570_e33592_d_n2, assign24570_e33592_d_n6, assign24570_e33592_d_n7, assign24570_e33592_d_n10, assign24570_e33592_d_n11, assign24570_e33592_d_n12, assign24570_e33592_d_n17,) = {
    if ((locals.var_guard767 == 0.0) && (locals.var_guard769 == 0.0)) {
        let assign24570_e33586: f64 = (p.p207 * locals.var_t3__blk763);
        let assign24570_e33588: f64 = (assign24570_e33586 * 1.6021918e-19);
        let assign24570_e33590: f64 = (assign24570_e33588 * locals.var_weff_nf);
        (assign24570_e33590, (((p.p207 * locals.var_t3__blk763_dn0) * 1.6021918e-19) * locals.var_weff_nf), (((p.p207 * locals.var_t3__blk763_dn2) * 1.6021918e-19) * locals.var_weff_nf), (((p.p207 * locals.var_t3__blk763_dn6) * 1.6021918e-19) * locals.var_weff_nf), (((p.p207 * locals.var_t3__blk763_dn7) * 1.6021918e-19) * locals.var_weff_nf), (((p.p207 * locals.var_t3__blk763_dn10) * 1.6021918e-19) * locals.var_weff_nf), (((p.p207 * locals.var_t3__blk763_dn11) * 1.6021918e-19) * locals.var_weff_nf), (((p.p207 * locals.var_t3__blk763_dn12) * 1.6021918e-19) * locals.var_weff_nf), (((p.p207 * locals.var_t3__blk763_dn17) * 1.6021918e-19) * locals.var_weff_nf),)
    } else {
        (locals.var_t2__blk761, locals.var_t2__blk761_dn0, locals.var_t2__blk761_dn2, locals.var_t2__blk761_dn6, locals.var_t2__blk761_dn7, locals.var_t2__blk761_dn10, locals.var_t2__blk761_dn11, locals.var_t2__blk761_dn12, locals.var_t2__blk761_dn17,)
    }
};
        locals.var_t2__blk761 = assign24570_e33592;
        locals.var_t2__blk761_dn0 = assign24570_e33592_d_n0;
        locals.var_t2__blk761_dn2 = assign24570_e33592_d_n2;
        locals.var_t2__blk761_dn6 = assign24570_e33592_d_n6;
        locals.var_t2__blk761_dn7 = assign24570_e33592_d_n7;
        locals.var_t2__blk761_dn10 = assign24570_e33592_d_n10;
        locals.var_t2__blk761_dn11 = assign24570_e33592_d_n11;
        locals.var_t2__blk761_dn12 = assign24570_e33592_d_n12;
        locals.var_t2__blk761_dn17 = assign24570_e33592_d_n17;
        locals.var_t2__blk761_rv = 0.0;

        let (assign24590_e33612, assign24590_e33612_d_n0, assign24590_e33612_d_n2, assign24590_e33612_d_n6, assign24590_e33612_d_n7, assign24590_e33612_d_n10, assign24590_e33612_d_n11, assign24590_e33612_d_n12, assign24590_e33612_d_n17,) = {
    if (locals.var_guard767 == 0.0) {
        let assign24590_e33610: f64 = (-locals.var_vbsp);
        (assign24590_e33610, (-locals.var_vbsp_dn0), (-locals.var_vbsp_dn2), (-locals.var_vbsp_dn6), (-locals.var_vbsp_dn7), (-locals.var_vbsp_dn10), (-locals.var_vbsp_dn11), (-locals.var_vbsp_dn12), (-locals.var_vbsp_dn17),)
    } else {
        (locals.var_vsb, locals.var_vsb_dn0, locals.var_vsb_dn2, locals.var_vsb_dn6, locals.var_vsb_dn7, locals.var_vsb_dn10, locals.var_vsb_dn11, locals.var_vsb_dn12, locals.var_vsb_dn17,)
    }
};
        locals.var_vsb = assign24590_e33612;
        locals.var_vsb_dn0 = assign24590_e33612_d_n0;
        locals.var_vsb_dn2 = assign24590_e33612_d_n2;
        locals.var_vsb_dn6 = assign24590_e33612_d_n6;
        locals.var_vsb_dn7 = assign24590_e33612_d_n7;
        locals.var_vsb_dn10 = assign24590_e33612_d_n10;
        locals.var_vsb_dn11 = assign24590_e33612_d_n11;
        locals.var_vsb_dn12 = assign24590_e33612_d_n12;
        locals.var_vsb_dn17 = assign24590_e33612_d_n17;
        locals.var_vsb_rv = 0.0;

        let assign24600_e33615: f64 = if locals.var_vsb > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard770 = assign24600_e33615;
        locals.var_guard770_rv = 0.0;

        let (assign24610_e33624, assign24610_e33624_d_n0, assign24610_e33624_d_n2, assign24610_e33624_d_n6, assign24610_e33624_d_n7, assign24610_e33624_d_n10, assign24610_e33624_d_n11, assign24610_e33624_d_n12, assign24610_e33624_d_n17,) = {
    if ((locals.var_guard767 == 0.0) && (locals.var_guard770 != 0.0)) {
        let assign24610_e33622: f64 = (locals.var_vsb * locals.var_vsb);
        (assign24610_e33622, ((locals.var_vsb_dn0 * locals.var_vsb) + (locals.var_vsb * locals.var_vsb_dn0)), ((locals.var_vsb_dn2 * locals.var_vsb) + (locals.var_vsb * locals.var_vsb_dn2)), ((locals.var_vsb_dn6 * locals.var_vsb) + (locals.var_vsb * locals.var_vsb_dn6)), ((locals.var_vsb_dn7 * locals.var_vsb) + (locals.var_vsb * locals.var_vsb_dn7)), ((locals.var_vsb_dn10 * locals.var_vsb) + (locals.var_vsb * locals.var_vsb_dn10)), ((locals.var_vsb_dn11 * locals.var_vsb) + (locals.var_vsb * locals.var_vsb_dn11)), ((locals.var_vsb_dn12 * locals.var_vsb) + (locals.var_vsb * locals.var_vsb_dn12)), ((locals.var_vsb_dn17 * locals.var_vsb) + (locals.var_vsb * locals.var_vsb_dn17)),)
    } else {
        (locals.var_t2__blk761, locals.var_t2__blk761_dn0, locals.var_t2__blk761_dn2, locals.var_t2__blk761_dn6, locals.var_t2__blk761_dn7, locals.var_t2__blk761_dn10, locals.var_t2__blk761_dn11, locals.var_t2__blk761_dn12, locals.var_t2__blk761_dn17,)
    }
};
        locals.var_t2__blk761 = assign24610_e33624;
        locals.var_t2__blk761_dn0 = assign24610_e33624_d_n0;
        locals.var_t2__blk761_dn2 = assign24610_e33624_d_n2;
        locals.var_t2__blk761_dn6 = assign24610_e33624_d_n6;
        locals.var_t2__blk761_dn7 = assign24610_e33624_d_n7;
        locals.var_t2__blk761_dn10 = assign24610_e33624_d_n10;
        locals.var_t2__blk761_dn11 = assign24610_e33624_d_n11;
        locals.var_t2__blk761_dn12 = assign24610_e33624_d_n12;
        locals.var_t2__blk761_dn17 = assign24610_e33624_d_n17;
        locals.var_t2__blk761_rv = 0.0;

        let (assign24620_e33633, assign24620_e33633_d_n0, assign24620_e33633_d_n2, assign24620_e33633_d_n6, assign24620_e33633_d_n7, assign24620_e33633_d_n10, assign24620_e33633_d_n11, assign24620_e33633_d_n12, assign24620_e33633_d_n17,) = {
    if ((locals.var_guard767 == 0.0) && (locals.var_guard770 != 0.0)) {
        let assign24620_e33631: f64 = (locals.var_t2__blk761 * locals.var_vsb);
        (assign24620_e33631, ((locals.var_t2__blk761_dn0 * locals.var_vsb) + (locals.var_t2__blk761 * locals.var_vsb_dn0)), ((locals.var_t2__blk761_dn2 * locals.var_vsb) + (locals.var_t2__blk761 * locals.var_vsb_dn2)), ((locals.var_t2__blk761_dn6 * locals.var_vsb) + (locals.var_t2__blk761 * locals.var_vsb_dn6)), ((locals.var_t2__blk761_dn7 * locals.var_vsb) + (locals.var_t2__blk761 * locals.var_vsb_dn7)), ((locals.var_t2__blk761_dn10 * locals.var_vsb) + (locals.var_t2__blk761 * locals.var_vsb_dn10)), ((locals.var_t2__blk761_dn11 * locals.var_vsb) + (locals.var_t2__blk761 * locals.var_vsb_dn11)), ((locals.var_t2__blk761_dn12 * locals.var_vsb) + (locals.var_t2__blk761 * locals.var_vsb_dn12)), ((locals.var_t2__blk761_dn17 * locals.var_vsb) + (locals.var_t2__blk761 * locals.var_vsb_dn17)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign24620_e33633;
        locals.var_t4_dn0 = assign24620_e33633_d_n0;
        locals.var_t4_dn2 = assign24620_e33633_d_n2;
        locals.var_t4_dn6 = assign24620_e33633_d_n6;
        locals.var_t4_dn7 = assign24620_e33633_d_n7;
        locals.var_t4_dn10 = assign24620_e33633_d_n10;
        locals.var_t4_dn11 = assign24620_e33633_d_n11;
        locals.var_t4_dn12 = assign24620_e33633_d_n12;
        locals.var_t4_dn17 = assign24620_e33633_d_n17;
        locals.var_t4_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_87(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign24630_e33642, assign24630_e33642_d_n0, assign24630_e33642_d_n2, assign24630_e33642_d_n6, assign24630_e33642_d_n7, assign24630_e33642_d_n10, assign24630_e33642_d_n11, assign24630_e33642_d_n12, assign24630_e33642_d_n17,) = {
    if ((locals.var_guard767 == 0.0) && (locals.var_guard770 != 0.0)) {
        let assign24630_e33640: f64 = (locals.var_t4 + p.p212);
        (assign24630_e33640, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    } else {
        (locals.var_t0__blk764, locals.var_t0__blk764_dn0, locals.var_t0__blk764_dn2, locals.var_t0__blk764_dn6, locals.var_t0__blk764_dn7, locals.var_t0__blk764_dn10, locals.var_t0__blk764_dn11, locals.var_t0__blk764_dn12, locals.var_t0__blk764_dn17,)
    }
};
        locals.var_t0__blk764 = assign24630_e33642;
        locals.var_t0__blk764_dn0 = assign24630_e33642_d_n0;
        locals.var_t0__blk764_dn2 = assign24630_e33642_d_n2;
        locals.var_t0__blk764_dn6 = assign24630_e33642_d_n6;
        locals.var_t0__blk764_dn7 = assign24630_e33642_d_n7;
        locals.var_t0__blk764_dn10 = assign24630_e33642_d_n10;
        locals.var_t0__blk764_dn11 = assign24630_e33642_d_n11;
        locals.var_t0__blk764_dn12 = assign24630_e33642_d_n12;
        locals.var_t0__blk764_dn17 = assign24630_e33642_d_n17;
        locals.var_t0__blk764_rv = 0.0;

        let assign24670_e33671: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard771 = assign24670_e33671;
        locals.var_guard771_rv = 0.0;

        let (assign24680_e33675,) = {
    if (locals.var_guard771 != 0.0) {
        (locals.var_c_fox0,)
    } else {
        (locals.var_cox0,)
    }
};
        locals.var_cox0 = assign24680_e33675;
        locals.var_cox0_rv = 0.0;

        let (assign24690_e33681,) = {
    if (locals.var_guard771 != 0.0) {
        let assign24690_e33679: f64 = (1.0 / locals.var_cox0);
        (assign24690_e33679,)
    } else {
        (locals.var_cox0_inv,)
    }
};
        locals.var_cox0_inv = assign24690_e33681;
        locals.var_cox0_inv_rv = 0.0;

        let (assign24700_e33685, assign24700_e33685_d_n0, assign24700_e33685_d_n2, assign24700_e33685_d_n6, assign24700_e33685_d_n7, assign24700_e33685_d_n10, assign24700_e33685_d_n11, assign24700_e33685_d_n12, assign24700_e33685_d_n17,) = {
    if (locals.var_guard771 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01__blk838, locals.var_fs01__blk838_dn0, locals.var_fs01__blk838_dn2, locals.var_fs01__blk838_dn6, locals.var_fs01__blk838_dn7, locals.var_fs01__blk838_dn10, locals.var_fs01__blk838_dn11, locals.var_fs01__blk838_dn12, locals.var_fs01__blk838_dn17,)
    }
};
        locals.var_fs01__blk838 = assign24700_e33685;
        locals.var_fs01__blk838_dn0 = assign24700_e33685_d_n0;
        locals.var_fs01__blk838_dn2 = assign24700_e33685_d_n2;
        locals.var_fs01__blk838_dn6 = assign24700_e33685_d_n6;
        locals.var_fs01__blk838_dn7 = assign24700_e33685_d_n7;
        locals.var_fs01__blk838_dn10 = assign24700_e33685_d_n10;
        locals.var_fs01__blk838_dn11 = assign24700_e33685_d_n11;
        locals.var_fs01__blk838_dn12 = assign24700_e33685_d_n12;
        locals.var_fs01__blk838_dn17 = assign24700_e33685_d_n17;
        locals.var_fs01__blk838_rv = 0.0;

        let (assign24710_e33689, assign24710_e33689_d_n0, assign24710_e33689_d_n2, assign24710_e33689_d_n6, assign24710_e33689_d_n7, assign24710_e33689_d_n10, assign24710_e33689_d_n11, assign24710_e33689_d_n12, assign24710_e33689_d_n17,) = {
    if (locals.var_guard771 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb__blk840, locals.var_fb__blk840_dn0, locals.var_fb__blk840_dn2, locals.var_fb__blk840_dn6, locals.var_fb__blk840_dn7, locals.var_fb__blk840_dn10, locals.var_fb__blk840_dn11, locals.var_fb__blk840_dn12, locals.var_fb__blk840_dn17,)
    }
};
        locals.var_fb__blk840 = assign24710_e33689;
        locals.var_fb__blk840_dn0 = assign24710_e33689_d_n0;
        locals.var_fb__blk840_dn2 = assign24710_e33689_d_n2;
        locals.var_fb__blk840_dn6 = assign24710_e33689_d_n6;
        locals.var_fb__blk840_dn7 = assign24710_e33689_d_n7;
        locals.var_fb__blk840_dn10 = assign24710_e33689_d_n10;
        locals.var_fb__blk840_dn11 = assign24710_e33689_d_n11;
        locals.var_fb__blk840_dn12 = assign24710_e33689_d_n12;
        locals.var_fb__blk840_dn17 = assign24710_e33689_d_n17;
        locals.var_fb__blk840_rv = 0.0;

        let (assign24720_e33693, assign24720_e33693_d_n0, assign24720_e33693_d_n2, assign24720_e33693_d_n6, assign24720_e33693_d_n7, assign24720_e33693_d_n10, assign24720_e33693_d_n11, assign24720_e33693_d_n12, assign24720_e33693_d_n17,) = {
    if (locals.var_guard771 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02__blk842, locals.var_fs02__blk842_dn0, locals.var_fs02__blk842_dn2, locals.var_fs02__blk842_dn6, locals.var_fs02__blk842_dn7, locals.var_fs02__blk842_dn10, locals.var_fs02__blk842_dn11, locals.var_fs02__blk842_dn12, locals.var_fs02__blk842_dn17,)
    }
};
        locals.var_fs02__blk842 = assign24720_e33693;
        locals.var_fs02__blk842_dn0 = assign24720_e33693_d_n0;
        locals.var_fs02__blk842_dn2 = assign24720_e33693_d_n2;
        locals.var_fs02__blk842_dn6 = assign24720_e33693_d_n6;
        locals.var_fs02__blk842_dn7 = assign24720_e33693_d_n7;
        locals.var_fs02__blk842_dn10 = assign24720_e33693_d_n10;
        locals.var_fs02__blk842_dn11 = assign24720_e33693_d_n11;
        locals.var_fs02__blk842_dn12 = assign24720_e33693_d_n12;
        locals.var_fs02__blk842_dn17 = assign24720_e33693_d_n17;
        locals.var_fs02__blk842_rv = 0.0;

        let (assign24730_e33698, assign24730_e33698_d_n0, assign24730_e33698_d_n2, assign24730_e33698_d_n6, assign24730_e33698_d_n7, assign24730_e33698_d_n10, assign24730_e33698_d_n11, assign24730_e33698_d_n12, assign24730_e33698_d_n17,) = {
    if (locals.var_guard771 != 0.0) {
        let assign24730_e33696: f64 = (-locals.var_area_bt_n);
        (assign24730_e33696, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk774, locals.var_t2__blk774_dn0, locals.var_t2__blk774_dn2, locals.var_t2__blk774_dn6, locals.var_t2__blk774_dn7, locals.var_t2__blk774_dn10, locals.var_t2__blk774_dn11, locals.var_t2__blk774_dn12, locals.var_t2__blk774_dn17,)
    }
};
        locals.var_t2__blk774 = assign24730_e33698;
        locals.var_t2__blk774_dn0 = assign24730_e33698_d_n0;
        locals.var_t2__blk774_dn2 = assign24730_e33698_d_n2;
        locals.var_t2__blk774_dn6 = assign24730_e33698_d_n6;
        locals.var_t2__blk774_dn7 = assign24730_e33698_d_n7;
        locals.var_t2__blk774_dn10 = assign24730_e33698_d_n10;
        locals.var_t2__blk774_dn11 = assign24730_e33698_d_n11;
        locals.var_t2__blk774_dn12 = assign24730_e33698_d_n12;
        locals.var_t2__blk774_dn17 = assign24730_e33698_d_n17;
        locals.var_t2__blk774_rv = 0.0;

        let (assign24740_e33704, assign24740_e33704_d_n0, assign24740_e33704_d_n2, assign24740_e33704_d_n6, assign24740_e33704_d_n7, assign24740_e33704_d_n10, assign24740_e33704_d_n11, assign24740_e33704_d_n12, assign24740_e33704_d_n17,) = {
    if (locals.var_guard771 != 0.0) {
        let assign24740_e33702: f64 = (locals.var_t2__blk774 * locals.var_qiu);
        (assign24740_e33702, ((locals.var_t2__blk774_dn0 * locals.var_qiu) + (locals.var_t2__blk774 * locals.var_qiu_dn0)), ((locals.var_t2__blk774_dn2 * locals.var_qiu) + (locals.var_t2__blk774 * locals.var_qiu_dn2)), ((locals.var_t2__blk774_dn6 * locals.var_qiu) + (locals.var_t2__blk774 * locals.var_qiu_dn6)), ((locals.var_t2__blk774_dn7 * locals.var_qiu) + (locals.var_t2__blk774 * locals.var_qiu_dn7)), ((locals.var_t2__blk774_dn10 * locals.var_qiu) + (locals.var_t2__blk774 * locals.var_qiu_dn10)), ((locals.var_t2__blk774_dn11 * locals.var_qiu) + (locals.var_t2__blk774 * locals.var_qiu_dn11)), ((locals.var_t2__blk774_dn12 * locals.var_qiu) + (locals.var_t2__blk774 * locals.var_qiu_dn12)), ((locals.var_t2__blk774_dn17 * locals.var_qiu) + (locals.var_t2__blk774 * locals.var_qiu_dn17)),)
    } else {
        (locals.var_t3__blk775, locals.var_t3__blk775_dn0, locals.var_t3__blk775_dn2, locals.var_t3__blk775_dn6, locals.var_t3__blk775_dn7, locals.var_t3__blk775_dn10, locals.var_t3__blk775_dn11, locals.var_t3__blk775_dn12, locals.var_t3__blk775_dn17,)
    }
};
        locals.var_t3__blk775 = assign24740_e33704;
        locals.var_t3__blk775_dn0 = assign24740_e33704_d_n0;
        locals.var_t3__blk775_dn2 = assign24740_e33704_d_n2;
        locals.var_t3__blk775_dn6 = assign24740_e33704_d_n6;
        locals.var_t3__blk775_dn7 = assign24740_e33704_d_n7;
        locals.var_t3__blk775_dn10 = assign24740_e33704_d_n10;
        locals.var_t3__blk775_dn11 = assign24740_e33704_d_n11;
        locals.var_t3__blk775_dn12 = assign24740_e33704_d_n12;
        locals.var_t3__blk775_dn17 = assign24740_e33704_d_n17;
        locals.var_t3__blk775_rv = 0.0;

        let (assign24750_e33712, assign24750_e33712_d_n0, assign24750_e33712_d_n2, assign24750_e33712_d_n6, assign24750_e33712_d_n7, assign24750_e33712_d_n10, assign24750_e33712_d_n11, assign24750_e33712_d_n12, assign24750_e33712_d_n17,) = {
    if (locals.var_guard771 != 0.0) {
        let assign24750_e33709: f64 = (locals.var_t2__blk774 * locals.var_qbu);
        let assign24750_e33710: f64 = (locals.var_t3__blk775 + assign24750_e33709);
        (assign24750_e33710, (locals.var_t3__blk775_dn0 + ((locals.var_t2__blk774_dn0 * locals.var_qbu) + (locals.var_t2__blk774 * locals.var_qbu_dn0))), (locals.var_t3__blk775_dn2 + ((locals.var_t2__blk774_dn2 * locals.var_qbu) + (locals.var_t2__blk774 * locals.var_qbu_dn2))), (locals.var_t3__blk775_dn6 + ((locals.var_t2__blk774_dn6 * locals.var_qbu) + (locals.var_t2__blk774 * locals.var_qbu_dn6))), (locals.var_t3__blk775_dn7 + ((locals.var_t2__blk774_dn7 * locals.var_qbu) + (locals.var_t2__blk774 * locals.var_qbu_dn7))), (locals.var_t3__blk775_dn10 + ((locals.var_t2__blk774_dn10 * locals.var_qbu) + (locals.var_t2__blk774 * locals.var_qbu_dn10))), (locals.var_t3__blk775_dn11 + ((locals.var_t2__blk774_dn11 * locals.var_qbu) + (locals.var_t2__blk774 * locals.var_qbu_dn11))), (locals.var_t3__blk775_dn12 + ((locals.var_t2__blk774_dn12 * locals.var_qbu) + (locals.var_t2__blk774 * locals.var_qbu_dn12))), (locals.var_t3__blk775_dn17 + ((locals.var_t2__blk774_dn17 * locals.var_qbu) + (locals.var_t2__blk774 * locals.var_qbu_dn17))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign24750_e33712;
        locals.var_t4_dn0 = assign24750_e33712_d_n0;
        locals.var_t4_dn2 = assign24750_e33712_d_n2;
        locals.var_t4_dn6 = assign24750_e33712_d_n6;
        locals.var_t4_dn7 = assign24750_e33712_d_n7;
        locals.var_t4_dn10 = assign24750_e33712_d_n10;
        locals.var_t4_dn11 = assign24750_e33712_d_n11;
        locals.var_t4_dn12 = assign24750_e33712_d_n12;
        locals.var_t4_dn17 = assign24750_e33712_d_n17;
        locals.var_t4_rv = 0.0;

        let (assign24760_e33718, assign24760_e33718_d_n0, assign24760_e33718_d_n2, assign24760_e33718_d_n6, assign24760_e33718_d_n7, assign24760_e33718_d_n10, assign24760_e33718_d_n11, assign24760_e33718_d_n12, assign24760_e33718_d_n17,) = {
    if (locals.var_guard771 != 0.0) {
        let assign24760_e33716: f64 = (locals.var_t3__blk775 * locals.var_qdrat);
        (assign24760_e33716, ((locals.var_t3__blk775_dn0 * locals.var_qdrat) + (locals.var_t3__blk775 * locals.var_qdrat_dn0)), ((locals.var_t3__blk775_dn2 * locals.var_qdrat) + (locals.var_t3__blk775 * locals.var_qdrat_dn2)), ((locals.var_t3__blk775_dn6 * locals.var_qdrat) + (locals.var_t3__blk775 * locals.var_qdrat_dn6)), ((locals.var_t3__blk775_dn7 * locals.var_qdrat) + (locals.var_t3__blk775 * locals.var_qdrat_dn7)), ((locals.var_t3__blk775_dn10 * locals.var_qdrat) + (locals.var_t3__blk775 * locals.var_qdrat_dn10)), ((locals.var_t3__blk775_dn11 * locals.var_qdrat) + (locals.var_t3__blk775 * locals.var_qdrat_dn11)), ((locals.var_t3__blk775_dn12 * locals.var_qdrat) + (locals.var_t3__blk775 * locals.var_qdrat_dn12)), ((locals.var_t3__blk775_dn17 * locals.var_qdrat) + (locals.var_t3__blk775 * locals.var_qdrat_dn17)),)
    } else {
        (locals.var_qbody_bt_n_iud, locals.var_qbody_bt_n_iud_dn0, locals.var_qbody_bt_n_iud_dn2, locals.var_qbody_bt_n_iud_dn6, locals.var_qbody_bt_n_iud_dn7, locals.var_qbody_bt_n_iud_dn10, locals.var_qbody_bt_n_iud_dn11, locals.var_qbody_bt_n_iud_dn12, locals.var_qbody_bt_n_iud_dn17,)
    }
};
        locals.var_qbody_bt_n_iud = assign24760_e33718;
        locals.var_qbody_bt_n_iud_dn0 = assign24760_e33718_d_n0;
        locals.var_qbody_bt_n_iud_dn2 = assign24760_e33718_d_n2;
        locals.var_qbody_bt_n_iud_dn6 = assign24760_e33718_d_n6;
        locals.var_qbody_bt_n_iud_dn7 = assign24760_e33718_d_n7;
        locals.var_qbody_bt_n_iud_dn10 = assign24760_e33718_d_n10;
        locals.var_qbody_bt_n_iud_dn11 = assign24760_e33718_d_n11;
        locals.var_qbody_bt_n_iud_dn12 = assign24760_e33718_d_n12;
        locals.var_qbody_bt_n_iud_dn17 = assign24760_e33718_d_n17;
        locals.var_qbody_bt_n_iud_rv = 0.0;

        let (assign24770_e33724, assign24770_e33724_d_n0, assign24770_e33724_d_n2, assign24770_e33724_d_n6, assign24770_e33724_d_n7, assign24770_e33724_d_n10, assign24770_e33724_d_n11, assign24770_e33724_d_n12, assign24770_e33724_d_n17,) = {
    if (locals.var_guard771 != 0.0) {
        let assign24770_e33722: f64 = (locals.var_t3__blk775 - locals.var_qbody_bt_n_iud);
        (assign24770_e33722, (locals.var_t3__blk775_dn0 - locals.var_qbody_bt_n_iud_dn0), (locals.var_t3__blk775_dn2 - locals.var_qbody_bt_n_iud_dn2), (locals.var_t3__blk775_dn6 - locals.var_qbody_bt_n_iud_dn6), (locals.var_t3__blk775_dn7 - locals.var_qbody_bt_n_iud_dn7), (locals.var_t3__blk775_dn10 - locals.var_qbody_bt_n_iud_dn10), (locals.var_t3__blk775_dn11 - locals.var_qbody_bt_n_iud_dn11), (locals.var_t3__blk775_dn12 - locals.var_qbody_bt_n_iud_dn12), (locals.var_t3__blk775_dn17 - locals.var_qbody_bt_n_iud_dn17),)
    } else {
        (locals.var_qbody_bt_n_ius, locals.var_qbody_bt_n_ius_dn0, locals.var_qbody_bt_n_ius_dn2, locals.var_qbody_bt_n_ius_dn6, locals.var_qbody_bt_n_ius_dn7, locals.var_qbody_bt_n_ius_dn10, locals.var_qbody_bt_n_ius_dn11, locals.var_qbody_bt_n_ius_dn12, locals.var_qbody_bt_n_ius_dn17,)
    }
};
        locals.var_qbody_bt_n_ius = assign24770_e33724;
        locals.var_qbody_bt_n_ius_dn0 = assign24770_e33724_d_n0;
        locals.var_qbody_bt_n_ius_dn2 = assign24770_e33724_d_n2;
        locals.var_qbody_bt_n_ius_dn6 = assign24770_e33724_d_n6;
        locals.var_qbody_bt_n_ius_dn7 = assign24770_e33724_d_n7;
        locals.var_qbody_bt_n_ius_dn10 = assign24770_e33724_d_n10;
        locals.var_qbody_bt_n_ius_dn11 = assign24770_e33724_d_n11;
        locals.var_qbody_bt_n_ius_dn12 = assign24770_e33724_d_n12;
        locals.var_qbody_bt_n_ius_dn17 = assign24770_e33724_d_n17;
        locals.var_qbody_bt_n_ius_rv = 0.0;

        let (assign24780_e33730, assign24780_e33730_d_n0, assign24780_e33730_d_n2, assign24780_e33730_d_n6, assign24780_e33730_d_n7, assign24780_e33730_d_n10, assign24780_e33730_d_n11, assign24780_e33730_d_n12, assign24780_e33730_d_n17,) = {
    if (locals.var_guard771 != 0.0) {
        let assign24780_e33728: f64 = (locals.var_t4 * locals.var_qdrat);
        (assign24780_e33728, ((locals.var_t4_dn0 * locals.var_qdrat) + (locals.var_t4 * locals.var_qdrat_dn0)), ((locals.var_t4_dn2 * locals.var_qdrat) + (locals.var_t4 * locals.var_qdrat_dn2)), ((locals.var_t4_dn6 * locals.var_qdrat) + (locals.var_t4 * locals.var_qdrat_dn6)), ((locals.var_t4_dn7 * locals.var_qdrat) + (locals.var_t4 * locals.var_qdrat_dn7)), ((locals.var_t4_dn10 * locals.var_qdrat) + (locals.var_t4 * locals.var_qdrat_dn10)), ((locals.var_t4_dn11 * locals.var_qdrat) + (locals.var_t4 * locals.var_qdrat_dn11)), ((locals.var_t4_dn12 * locals.var_qdrat) + (locals.var_t4 * locals.var_qdrat_dn12)), ((locals.var_t4_dn17 * locals.var_qdrat) + (locals.var_t4 * locals.var_qdrat_dn17)),)
    } else {
        (locals.var_qbody_bt_n_sud, locals.var_qbody_bt_n_sud_dn0, locals.var_qbody_bt_n_sud_dn2, locals.var_qbody_bt_n_sud_dn6, locals.var_qbody_bt_n_sud_dn7, locals.var_qbody_bt_n_sud_dn10, locals.var_qbody_bt_n_sud_dn11, locals.var_qbody_bt_n_sud_dn12, locals.var_qbody_bt_n_sud_dn17,)
    }
};
        locals.var_qbody_bt_n_sud = assign24780_e33730;
        locals.var_qbody_bt_n_sud_dn0 = assign24780_e33730_d_n0;
        locals.var_qbody_bt_n_sud_dn2 = assign24780_e33730_d_n2;
        locals.var_qbody_bt_n_sud_dn6 = assign24780_e33730_d_n6;
        locals.var_qbody_bt_n_sud_dn7 = assign24780_e33730_d_n7;
        locals.var_qbody_bt_n_sud_dn10 = assign24780_e33730_d_n10;
        locals.var_qbody_bt_n_sud_dn11 = assign24780_e33730_d_n11;
        locals.var_qbody_bt_n_sud_dn12 = assign24780_e33730_d_n12;
        locals.var_qbody_bt_n_sud_dn17 = assign24780_e33730_d_n17;
        locals.var_qbody_bt_n_sud_rv = 0.0;

        let (assign24790_e33736, assign24790_e33736_d_n0, assign24790_e33736_d_n2, assign24790_e33736_d_n6, assign24790_e33736_d_n7, assign24790_e33736_d_n10, assign24790_e33736_d_n11, assign24790_e33736_d_n12, assign24790_e33736_d_n17,) = {
    if (locals.var_guard771 != 0.0) {
        let assign24790_e33734: f64 = (locals.var_t4 - locals.var_qbody_bt_n_sud);
        (assign24790_e33734, (locals.var_t4_dn0 - locals.var_qbody_bt_n_sud_dn0), (locals.var_t4_dn2 - locals.var_qbody_bt_n_sud_dn2), (locals.var_t4_dn6 - locals.var_qbody_bt_n_sud_dn6), (locals.var_t4_dn7 - locals.var_qbody_bt_n_sud_dn7), (locals.var_t4_dn10 - locals.var_qbody_bt_n_sud_dn10), (locals.var_t4_dn11 - locals.var_qbody_bt_n_sud_dn11), (locals.var_t4_dn12 - locals.var_qbody_bt_n_sud_dn12), (locals.var_t4_dn17 - locals.var_qbody_bt_n_sud_dn17),)
    } else {
        (locals.var_qbody_bt_n_sus, locals.var_qbody_bt_n_sus_dn0, locals.var_qbody_bt_n_sus_dn2, locals.var_qbody_bt_n_sus_dn6, locals.var_qbody_bt_n_sus_dn7, locals.var_qbody_bt_n_sus_dn10, locals.var_qbody_bt_n_sus_dn11, locals.var_qbody_bt_n_sus_dn12, locals.var_qbody_bt_n_sus_dn17,)
    }
};
        locals.var_qbody_bt_n_sus = assign24790_e33736;
        locals.var_qbody_bt_n_sus_dn0 = assign24790_e33736_d_n0;
        locals.var_qbody_bt_n_sus_dn2 = assign24790_e33736_d_n2;
        locals.var_qbody_bt_n_sus_dn6 = assign24790_e33736_d_n6;
        locals.var_qbody_bt_n_sus_dn7 = assign24790_e33736_d_n7;
        locals.var_qbody_bt_n_sus_dn10 = assign24790_e33736_d_n10;
        locals.var_qbody_bt_n_sus_dn11 = assign24790_e33736_d_n11;
        locals.var_qbody_bt_n_sus_dn12 = assign24790_e33736_d_n12;
        locals.var_qbody_bt_n_sus_dn17 = assign24790_e33736_d_n17;
        locals.var_qbody_bt_n_sus_rv = 0.0;

        let (assign24800_e33742, assign24800_e33742_d_n0, assign24800_e33742_d_n2, assign24800_e33742_d_n6, assign24800_e33742_d_n7, assign24800_e33742_d_n10, assign24800_e33742_d_n11, assign24800_e33742_d_n12, assign24800_e33742_d_n17,) = {
    if ((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) {
        (locals.var_nsub, locals.var_nsub_dn0, locals.var_nsub_dn2, locals.var_nsub_dn6, locals.var_nsub_dn7, locals.var_nsub_dn10, locals.var_nsub_dn11, locals.var_nsub_dn12, locals.var_nsub_dn17,)
    } else {
        (locals.var_uc_nsubbttub, locals.var_uc_nsubbttub_dn0, locals.var_uc_nsubbttub_dn2, locals.var_uc_nsubbttub_dn6, locals.var_uc_nsubbttub_dn7, locals.var_uc_nsubbttub_dn10, locals.var_uc_nsubbttub_dn11, locals.var_uc_nsubbttub_dn12, locals.var_uc_nsubbttub_dn17,)
    }
};
        locals.var_uc_nsubbttub = assign24800_e33742;
        locals.var_uc_nsubbttub_dn0 = assign24800_e33742_d_n0;
        locals.var_uc_nsubbttub_dn2 = assign24800_e33742_d_n2;
        locals.var_uc_nsubbttub_dn6 = assign24800_e33742_d_n6;
        locals.var_uc_nsubbttub_dn7 = assign24800_e33742_d_n7;
        locals.var_uc_nsubbttub_dn10 = assign24800_e33742_d_n10;
        locals.var_uc_nsubbttub_dn11 = assign24800_e33742_d_n11;
        locals.var_uc_nsubbttub_dn12 = assign24800_e33742_d_n12;
        locals.var_uc_nsubbttub_dn17 = assign24800_e33742_d_n17;
        locals.var_uc_nsubbttub_rv = 0.0;

        let (assign24810_e33748,) = {
    if ((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_cbtb_given,)
    }
};
        locals.var_cbtb_given = assign24810_e33748;
        locals.var_cbtb_given_rv = 0.0;

        let assign24820_e33751: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard851 = assign24820_e33751;
        locals.var_guard851_rv = 0.0;

        let assign24830_e33754: f64 = if 1.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard852 = assign24830_e33754;
        locals.var_guard852_rv = 0.0;

        let (assign24840_e33764,) = {
    if (((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) {
        let assign24840_e33762: f64 = (locals.var_area_bt_p * 0.5);
        (assign24840_e33762,)
    } else {
        (locals.var_uc_areabt,)
    }
};
        locals.var_uc_areabt = assign24840_e33764;
        locals.var_uc_areabt_rv = 0.0;

        let (assign24850_e33772,) = {
    if (((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) {
        (p.p292,)
    } else {
        (locals.var_uc_vfbbt,)
    }
};
        locals.var_uc_vfbbt = assign24850_e33772;
        locals.var_uc_vfbbt_rv = 0.0;

        let (assign24860_e33780,) = {
    if (((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) {
        (locals.var_cbtbp_given,)
    } else {
        (locals.var_cbtb_given,)
    }
};
        locals.var_cbtb_given = assign24860_e33780;
        locals.var_cbtb_given_rv = 0.0;

        let (assign24870_e33793,) = {
    if (((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && ((locals.var_guard852 != 0.0) && (locals.var_guard851 == 0.0))) {
        let assign24870_e33791: f64 = (locals.var_area_bt_n * 0.5);
        (assign24870_e33791,)
    } else {
        (locals.var_uc_areabt,)
    }
};
        locals.var_uc_areabt = assign24870_e33793;
        locals.var_uc_areabt_rv = 0.0;

        let (assign24880_e33804,) = {
    if (((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && ((locals.var_guard852 != 0.0) && (locals.var_guard851 == 0.0))) {
        (p.p68,)
    } else {
        (locals.var_uc_vfbbt,)
    }
};
        locals.var_uc_vfbbt = assign24880_e33804;
        locals.var_uc_vfbbt_rv = 0.0;

        let (assign24890_e33815,) = {
    if (((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && ((locals.var_guard852 != 0.0) && (locals.var_guard851 == 0.0))) {
        (locals.var_cbtbn_given,)
    } else {
        (locals.var_cbtb_given,)
    }
};
        locals.var_cbtb_given = assign24890_e33815;
        locals.var_cbtb_given_rv = 0.0;

        let (assign24900_e33826,) = {
    if (((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && ((locals.var_guard852 != 0.0) && (locals.var_guard851 == 0.0))) {
        (1.0,)
    } else {
        (locals.var_cbtb_given,)
    }
};
        locals.var_cbtb_given = assign24900_e33826;
        locals.var_cbtb_given_rv = 0.0;

        let assign24910_e33829: f64 = if locals.var_cbtb_given == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard853 = assign24910_e33829;
        locals.var_guard853_rv = 0.0;

        let (assign24920_e33842, assign24920_e33842_d_n0, assign24920_e33842_d_n2, assign24920_e33842_d_n6, assign24920_e33842_d_n7, assign24920_e33842_d_n10, assign24920_e33842_d_n11, assign24920_e33842_d_n12, assign24920_e33842_d_n17,) = {
    if (((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign24920_e33838: f64 = (locals.var_uc_nsubbttub / locals.var_nsub);
        let assign24920_e33839: f64 = (assign24920_e33838).sqrt();
        let assign24920_e33840: f64 = (locals.var_cnst0soi * assign24920_e33839);
        (assign24920_e33840, ((locals.var_cnst0soi_dn0 * assign24920_e33839) + (locals.var_cnst0soi * ((((locals.var_uc_nsubbttub_dn0 * locals.var_nsub) - (locals.var_uc_nsubbttub * locals.var_nsub_dn0)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign24920_e33839)))), ((locals.var_cnst0soi_dn2 * assign24920_e33839) + (locals.var_cnst0soi * ((((locals.var_uc_nsubbttub_dn2 * locals.var_nsub) - (locals.var_uc_nsubbttub * locals.var_nsub_dn2)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign24920_e33839)))), ((locals.var_cnst0soi_dn6 * assign24920_e33839) + (locals.var_cnst0soi * ((((locals.var_uc_nsubbttub_dn6 * locals.var_nsub) - (locals.var_uc_nsubbttub * locals.var_nsub_dn6)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign24920_e33839)))), ((locals.var_cnst0soi_dn7 * assign24920_e33839) + (locals.var_cnst0soi * ((((locals.var_uc_nsubbttub_dn7 * locals.var_nsub) - (locals.var_uc_nsubbttub * locals.var_nsub_dn7)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign24920_e33839)))), ((locals.var_cnst0soi_dn10 * assign24920_e33839) + (locals.var_cnst0soi * ((((locals.var_uc_nsubbttub_dn10 * locals.var_nsub) - (locals.var_uc_nsubbttub * locals.var_nsub_dn10)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign24920_e33839)))), ((locals.var_cnst0soi_dn11 * assign24920_e33839) + (locals.var_cnst0soi * ((((locals.var_uc_nsubbttub_dn11 * locals.var_nsub) - (locals.var_uc_nsubbttub * locals.var_nsub_dn11)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign24920_e33839)))), ((locals.var_cnst0soi_dn12 * assign24920_e33839) + (locals.var_cnst0soi * ((((locals.var_uc_nsubbttub_dn12 * locals.var_nsub) - (locals.var_uc_nsubbttub * locals.var_nsub_dn12)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign24920_e33839)))), ((locals.var_cnst0soi_dn17 * assign24920_e33839) + (locals.var_cnst0soi * ((((locals.var_uc_nsubbttub_dn17 * locals.var_nsub) - (locals.var_uc_nsubbttub * locals.var_nsub_dn17)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign24920_e33839)))),)
    } else {
        (locals.var_cnst0over, locals.var_cnst0over_dn0, locals.var_cnst0over_dn2, locals.var_cnst0over_dn6, locals.var_cnst0over_dn7, locals.var_cnst0over_dn10, locals.var_cnst0over_dn11, locals.var_cnst0over_dn12, locals.var_cnst0over_dn17,)
    }
};
        locals.var_cnst0over = assign24920_e33842;
        locals.var_cnst0over_dn0 = assign24920_e33842_d_n0;
        locals.var_cnst0over_dn2 = assign24920_e33842_d_n2;
        locals.var_cnst0over_dn6 = assign24920_e33842_d_n6;
        locals.var_cnst0over_dn7 = assign24920_e33842_d_n7;
        locals.var_cnst0over_dn10 = assign24920_e33842_d_n10;
        locals.var_cnst0over_dn11 = assign24920_e33842_d_n11;
        locals.var_cnst0over_dn12 = assign24920_e33842_d_n12;
        locals.var_cnst0over_dn17 = assign24920_e33842_d_n17;
        locals.var_cnst0over_rv = 0.0;

        let (assign24930_e33854,) = {
    if (((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign24930_e33850: f64 = (1.0 - -1.0);
        let assign24930_e33852: f64 = (assign24930_e33850 / 2.0);
        (assign24930_e33852,)
    } else {
        (locals.var_flg_ovloops,)
    }
};
        locals.var_flg_ovloops = assign24930_e33854;
        locals.var_flg_ovloops_rv = 0.0;

        let (assign24940_e33866,) = {
    if (((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign24940_e33862: f64 = (1.0 + -1.0);
        let assign24940_e33864: f64 = (assign24940_e33862 / 2.0);
        (assign24940_e33864,)
    } else {
        (locals.var_flg_ovloopd,)
    }
};
        locals.var_flg_ovloopd = assign24940_e33866;
        locals.var_flg_ovloopd_rv = 0.0;

        let (assign24950_e33882, assign24950_e33882_d_n0, assign24950_e33882_d_n2, assign24950_e33882_d_n6, assign24950_e33882_d_n7, assign24950_e33882_d_n10, assign24950_e33882_d_n11, assign24950_e33882_d_n12, assign24950_e33882_d_n17,) = {
    if (((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign24950_e33874: f64 = (locals.var_modenml * locals.var_vbs);
        let assign24950_e33878: f64 = (locals.var_vbs - locals.var_vds);
        let assign24950_e33879: f64 = (locals.var_modervs * assign24950_e33878);
        let assign24950_e33880: f64 = (assign24950_e33874 + assign24950_e33879);
        (assign24950_e33880, ((locals.var_modenml * locals.var_vbs_dn0) + (locals.var_modervs * (locals.var_vbs_dn0 - locals.var_vds_dn0))), ((locals.var_modenml * locals.var_vbs_dn2) + (locals.var_modervs * (locals.var_vbs_dn2 - locals.var_vds_dn2))), ((locals.var_modenml * locals.var_vbs_dn6) + (locals.var_modervs * (locals.var_vbs_dn6 - locals.var_vds_dn6))), ((locals.var_modenml * locals.var_vbs_dn7) + (locals.var_modervs * (locals.var_vbs_dn7 - locals.var_vds_dn7))), ((locals.var_modenml * locals.var_vbs_dn10) + (locals.var_modervs * (locals.var_vbs_dn10 - locals.var_vds_dn10))), ((locals.var_modenml * locals.var_vbs_dn11) + (locals.var_modervs * (locals.var_vbs_dn11 - locals.var_vds_dn11))), ((locals.var_modenml * locals.var_vbs_dn12) + (locals.var_modervs * (locals.var_vbs_dn12 - locals.var_vds_dn12))), ((locals.var_modenml * locals.var_vbs_dn17) + (locals.var_modervs * (locals.var_vbs_dn17 - locals.var_vds_dn17))),)
    } else {
        (locals.var_vbsgmt, locals.var_vbsgmt_dn0, locals.var_vbsgmt_dn2, locals.var_vbsgmt_dn6, locals.var_vbsgmt_dn7, locals.var_vbsgmt_dn10, locals.var_vbsgmt_dn11, locals.var_vbsgmt_dn12, locals.var_vbsgmt_dn17,)
    }
};
        locals.var_vbsgmt = assign24950_e33882;
        locals.var_vbsgmt_dn0 = assign24950_e33882_d_n0;
        locals.var_vbsgmt_dn2 = assign24950_e33882_d_n2;
        locals.var_vbsgmt_dn6 = assign24950_e33882_d_n6;
        locals.var_vbsgmt_dn7 = assign24950_e33882_d_n7;
        locals.var_vbsgmt_dn10 = assign24950_e33882_d_n10;
        locals.var_vbsgmt_dn11 = assign24950_e33882_d_n11;
        locals.var_vbsgmt_dn12 = assign24950_e33882_d_n12;
        locals.var_vbsgmt_dn17 = assign24950_e33882_d_n17;
        locals.var_vbsgmt_rv = 0.0;

        let (assign24960_e33897, assign24960_e33897_d_n0, assign24960_e33897_d_n2, assign24960_e33897_d_n6, assign24960_e33897_d_n7, assign24960_e33897_d_n10, assign24960_e33897_d_n11, assign24960_e33897_d_n12, assign24960_e33897_d_n17,) = {
    if (((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign24960_e33890: f64 = (locals.var_modenml * locals.var_vds);
        let assign24960_e33893: f64 = (-locals.var_vds);
        let assign24960_e33894: f64 = (locals.var_modervs * assign24960_e33893);
        let assign24960_e33895: f64 = (assign24960_e33890 + assign24960_e33894);
        (assign24960_e33895, ((locals.var_modenml * locals.var_vds_dn0) + (locals.var_modervs * (-locals.var_vds_dn0))), ((locals.var_modenml * locals.var_vds_dn2) + (locals.var_modervs * (-locals.var_vds_dn2))), ((locals.var_modenml * locals.var_vds_dn6) + (locals.var_modervs * (-locals.var_vds_dn6))), ((locals.var_modenml * locals.var_vds_dn7) + (locals.var_modervs * (-locals.var_vds_dn7))), ((locals.var_modenml * locals.var_vds_dn10) + (locals.var_modervs * (-locals.var_vds_dn10))), ((locals.var_modenml * locals.var_vds_dn11) + (locals.var_modervs * (-locals.var_vds_dn11))), ((locals.var_modenml * locals.var_vds_dn12) + (locals.var_modervs * (-locals.var_vds_dn12))), ((locals.var_modenml * locals.var_vds_dn17) + (locals.var_modervs * (-locals.var_vds_dn17))),)
    } else {
        (locals.var_vdsgmt, locals.var_vdsgmt_dn0, locals.var_vdsgmt_dn2, locals.var_vdsgmt_dn6, locals.var_vdsgmt_dn7, locals.var_vdsgmt_dn10, locals.var_vdsgmt_dn11, locals.var_vdsgmt_dn12, locals.var_vdsgmt_dn17,)
    }
};
        locals.var_vdsgmt = assign24960_e33897;
        locals.var_vdsgmt_dn0 = assign24960_e33897_d_n0;
        locals.var_vdsgmt_dn2 = assign24960_e33897_d_n2;
        locals.var_vdsgmt_dn6 = assign24960_e33897_d_n6;
        locals.var_vdsgmt_dn7 = assign24960_e33897_d_n7;
        locals.var_vdsgmt_dn10 = assign24960_e33897_d_n10;
        locals.var_vdsgmt_dn11 = assign24960_e33897_d_n11;
        locals.var_vdsgmt_dn12 = assign24960_e33897_d_n12;
        locals.var_vdsgmt_dn17 = assign24960_e33897_d_n17;
        locals.var_vdsgmt_rv = 0.0;

        let (assign24970_e33913, assign24970_e33913_d_n0, assign24970_e33913_d_n2, assign24970_e33913_d_n6, assign24970_e33913_d_n7, assign24970_e33913_d_n10, assign24970_e33913_d_n11, assign24970_e33913_d_n12, assign24970_e33913_d_n17,) = {
    if (((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign24970_e33905: f64 = (locals.var_modenml * locals.var_vgs);
        let assign24970_e33909: f64 = (locals.var_vgs - locals.var_vds);
        let assign24970_e33910: f64 = (locals.var_modervs * assign24970_e33909);
        let assign24970_e33911: f64 = (assign24970_e33905 + assign24970_e33910);
        (assign24970_e33911, (locals.var_modervs * (-locals.var_vds_dn0)), (locals.var_modervs * (-locals.var_vds_dn2)), ((locals.var_modenml * locals.var_vgs_dn6) + (locals.var_modervs * (locals.var_vgs_dn6 - locals.var_vds_dn6))), ((locals.var_modenml * locals.var_vgs_dn7) + (locals.var_modervs * (locals.var_vgs_dn7 - locals.var_vds_dn7))), (locals.var_modervs * (-locals.var_vds_dn10)), ((locals.var_modenml * locals.var_vgs_dn11) + (locals.var_modervs * (locals.var_vgs_dn11 - locals.var_vds_dn11))), (locals.var_modervs * (-locals.var_vds_dn12)), (locals.var_modervs * (-locals.var_vds_dn17)),)
    } else {
        (locals.var_vgsgmt, locals.var_vgsgmt_dn0, locals.var_vgsgmt_dn2, locals.var_vgsgmt_dn6, locals.var_vgsgmt_dn7, locals.var_vgsgmt_dn10, locals.var_vgsgmt_dn11, locals.var_vgsgmt_dn12, locals.var_vgsgmt_dn17,)
    }
};
        locals.var_vgsgmt = assign24970_e33913;
        locals.var_vgsgmt_dn0 = assign24970_e33913_d_n0;
        locals.var_vgsgmt_dn2 = assign24970_e33913_d_n2;
        locals.var_vgsgmt_dn6 = assign24970_e33913_d_n6;
        locals.var_vgsgmt_dn7 = assign24970_e33913_d_n7;
        locals.var_vgsgmt_dn10 = assign24970_e33913_d_n10;
        locals.var_vgsgmt_dn11 = assign24970_e33913_d_n11;
        locals.var_vgsgmt_dn12 = assign24970_e33913_d_n12;
        locals.var_vgsgmt_dn17 = assign24970_e33913_d_n17;
        locals.var_vgsgmt_rv = 0.0;

        let (assign24980_e33929, assign24980_e33929_d_n0, assign24980_e33929_d_n2, assign24980_e33929_d_n6, assign24980_e33929_d_n7, assign24980_e33929_d_n10, assign24980_e33929_d_n11, assign24980_e33929_d_n12, assign24980_e33929_d_n17,) = {
    if (((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign24980_e33921: f64 = (locals.var_modervs * locals.var_vgs);
        let assign24980_e33925: f64 = (locals.var_vgs - locals.var_vds);
        let assign24980_e33926: f64 = (locals.var_modenml * assign24980_e33925);
        let assign24980_e33927: f64 = (assign24980_e33921 + assign24980_e33926);
        (assign24980_e33927, (locals.var_modenml * (-locals.var_vds_dn0)), (locals.var_modenml * (-locals.var_vds_dn2)), ((locals.var_modervs * locals.var_vgs_dn6) + (locals.var_modenml * (locals.var_vgs_dn6 - locals.var_vds_dn6))), ((locals.var_modervs * locals.var_vgs_dn7) + (locals.var_modenml * (locals.var_vgs_dn7 - locals.var_vds_dn7))), (locals.var_modenml * (-locals.var_vds_dn10)), ((locals.var_modervs * locals.var_vgs_dn11) + (locals.var_modenml * (locals.var_vgs_dn11 - locals.var_vds_dn11))), (locals.var_modenml * (-locals.var_vds_dn12)), (locals.var_modenml * (-locals.var_vds_dn17)),)
    } else {
        (locals.var_vgdgmt, locals.var_vgdgmt_dn0, locals.var_vgdgmt_dn2, locals.var_vgdgmt_dn6, locals.var_vgdgmt_dn7, locals.var_vgdgmt_dn10, locals.var_vgdgmt_dn11, locals.var_vgdgmt_dn12, locals.var_vgdgmt_dn17,)
    }
};
        locals.var_vgdgmt = assign24980_e33929;
        locals.var_vgdgmt_dn0 = assign24980_e33929_d_n0;
        locals.var_vgdgmt_dn2 = assign24980_e33929_d_n2;
        locals.var_vgdgmt_dn6 = assign24980_e33929_d_n6;
        locals.var_vgdgmt_dn7 = assign24980_e33929_d_n7;
        locals.var_vgdgmt_dn10 = assign24980_e33929_d_n10;
        locals.var_vgdgmt_dn11 = assign24980_e33929_d_n11;
        locals.var_vgdgmt_dn12 = assign24980_e33929_d_n12;
        locals.var_vgdgmt_dn17 = assign24980_e33929_d_n17;
        locals.var_vgdgmt_rv = 0.0;

        let (assign24990_e33939, assign24990_e33939_d_n0, assign24990_e33939_d_n2, assign24990_e33939_d_n6, assign24990_e33939_d_n7, assign24990_e33939_d_n10, assign24990_e33939_d_n11, assign24990_e33939_d_n12, assign24990_e33939_d_n17,) = {
    if (((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign24990_e33937: f64 = (locals.var_vdsgmt - locals.var_vbsgmt);
        (assign24990_e33937, (locals.var_vdsgmt_dn0 - locals.var_vbsgmt_dn0), (locals.var_vdsgmt_dn2 - locals.var_vbsgmt_dn2), (locals.var_vdsgmt_dn6 - locals.var_vbsgmt_dn6), (locals.var_vdsgmt_dn7 - locals.var_vbsgmt_dn7), (locals.var_vdsgmt_dn10 - locals.var_vbsgmt_dn10), (locals.var_vdsgmt_dn11 - locals.var_vbsgmt_dn11), (locals.var_vdsgmt_dn12 - locals.var_vbsgmt_dn12), (locals.var_vdsgmt_dn17 - locals.var_vbsgmt_dn17),)
    } else {
        (locals.var_vdbgmt, locals.var_vdbgmt_dn0, locals.var_vdbgmt_dn2, locals.var_vdbgmt_dn6, locals.var_vdbgmt_dn7, locals.var_vdbgmt_dn10, locals.var_vdbgmt_dn11, locals.var_vdbgmt_dn12, locals.var_vdbgmt_dn17,)
    }
};
        locals.var_vdbgmt = assign24990_e33939;
        locals.var_vdbgmt_dn0 = assign24990_e33939_d_n0;
        locals.var_vdbgmt_dn2 = assign24990_e33939_d_n2;
        locals.var_vdbgmt_dn6 = assign24990_e33939_d_n6;
        locals.var_vdbgmt_dn7 = assign24990_e33939_d_n7;
        locals.var_vdbgmt_dn10 = assign24990_e33939_d_n10;
        locals.var_vdbgmt_dn11 = assign24990_e33939_d_n11;
        locals.var_vdbgmt_dn12 = assign24990_e33939_d_n12;
        locals.var_vdbgmt_dn17 = assign24990_e33939_d_n17;
        locals.var_vdbgmt_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_88(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign25000_e33948, assign25000_e33948_d_n0, assign25000_e33948_d_n2, assign25000_e33948_d_n6, assign25000_e33948_d_n7, assign25000_e33948_d_n10, assign25000_e33948_d_n11, assign25000_e33948_d_n12, assign25000_e33948_d_n17,) = {
    if (((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign25000_e33946: f64 = (-locals.var_vbsgmt);
        (assign25000_e33946, (-locals.var_vbsgmt_dn0), (-locals.var_vbsgmt_dn2), (-locals.var_vbsgmt_dn6), (-locals.var_vbsgmt_dn7), (-locals.var_vbsgmt_dn10), (-locals.var_vbsgmt_dn11), (-locals.var_vbsgmt_dn12), (-locals.var_vbsgmt_dn17),)
    } else {
        (locals.var_vsbgmt, locals.var_vsbgmt_dn0, locals.var_vsbgmt_dn2, locals.var_vsbgmt_dn6, locals.var_vsbgmt_dn7, locals.var_vsbgmt_dn10, locals.var_vsbgmt_dn11, locals.var_vsbgmt_dn12, locals.var_vsbgmt_dn17,)
    }
};
        locals.var_vsbgmt = assign25000_e33948;
        locals.var_vsbgmt_dn0 = assign25000_e33948_d_n0;
        locals.var_vsbgmt_dn2 = assign25000_e33948_d_n2;
        locals.var_vsbgmt_dn6 = assign25000_e33948_d_n6;
        locals.var_vsbgmt_dn7 = assign25000_e33948_d_n7;
        locals.var_vsbgmt_dn10 = assign25000_e33948_d_n10;
        locals.var_vsbgmt_dn11 = assign25000_e33948_d_n11;
        locals.var_vsbgmt_dn12 = assign25000_e33948_d_n12;
        locals.var_vsbgmt_dn17 = assign25000_e33948_d_n17;
        locals.var_vsbgmt_rv = 0.0;

        let (assign25010_e33962,) = {
    if (((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign25010_e33956: f64 = (locals.var_flg_ovloops * locals.var_modenml);
        let assign25010_e33959: f64 = (locals.var_flg_ovloopd * locals.var_modervs);
        let assign25010_e33960: f64 = (assign25010_e33956 + assign25010_e33959);
        (assign25010_e33960,)
    } else {
        (locals.var_flg_overs,)
    }
};
        locals.var_flg_overs = assign25010_e33962;
        locals.var_flg_overs_rv = 0.0;

        let (assign25020_e33976,) = {
    if (((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign25020_e33970: f64 = (locals.var_flg_ovloops * locals.var_modervs);
        let assign25020_e33973: f64 = (locals.var_flg_ovloopd * locals.var_modenml);
        let assign25020_e33974: f64 = (assign25020_e33970 + assign25020_e33973);
        (assign25020_e33974,)
    } else {
        (locals.var_flg_overd,)
    }
};
        locals.var_flg_overd = assign25020_e33976;
        locals.var_flg_overd_rv = 0.0;

        let (assign25030_e33990, assign25030_e33990_d_n0, assign25030_e33990_d_n2, assign25030_e33990_d_n6, assign25030_e33990_d_n7, assign25030_e33990_d_n10, assign25030_e33990_d_n11, assign25030_e33990_d_n12, assign25030_e33990_d_n17,) = {
    if (((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign25030_e33984: f64 = (locals.var_flg_overs * locals.var_vgsgmt);
        let assign25030_e33987: f64 = (locals.var_flg_overd * locals.var_vgdgmt);
        let assign25030_e33988: f64 = (assign25030_e33984 + assign25030_e33987);
        (assign25030_e33988, ((locals.var_flg_overs * locals.var_vgsgmt_dn0) + (locals.var_flg_overd * locals.var_vgdgmt_dn0)), ((locals.var_flg_overs * locals.var_vgsgmt_dn2) + (locals.var_flg_overd * locals.var_vgdgmt_dn2)), ((locals.var_flg_overs * locals.var_vgsgmt_dn6) + (locals.var_flg_overd * locals.var_vgdgmt_dn6)), ((locals.var_flg_overs * locals.var_vgsgmt_dn7) + (locals.var_flg_overd * locals.var_vgdgmt_dn7)), ((locals.var_flg_overs * locals.var_vgsgmt_dn10) + (locals.var_flg_overd * locals.var_vgdgmt_dn10)), ((locals.var_flg_overs * locals.var_vgsgmt_dn11) + (locals.var_flg_overd * locals.var_vgdgmt_dn11)), ((locals.var_flg_overs * locals.var_vgsgmt_dn12) + (locals.var_flg_overd * locals.var_vgdgmt_dn12)), ((locals.var_flg_overs * locals.var_vgsgmt_dn17) + (locals.var_flg_overd * locals.var_vgdgmt_dn17)),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn0, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn6, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn10, locals.var_vgbgmt_dn11, locals.var_vgbgmt_dn12, locals.var_vgbgmt_dn17,)
    }
};
        locals.var_vgbgmt = assign25030_e33990;
        locals.var_vgbgmt_dn0 = assign25030_e33990_d_n0;
        locals.var_vgbgmt_dn2 = assign25030_e33990_d_n2;
        locals.var_vgbgmt_dn6 = assign25030_e33990_d_n6;
        locals.var_vgbgmt_dn7 = assign25030_e33990_d_n7;
        locals.var_vgbgmt_dn10 = assign25030_e33990_d_n10;
        locals.var_vgbgmt_dn11 = assign25030_e33990_d_n11;
        locals.var_vgbgmt_dn12 = assign25030_e33990_d_n12;
        locals.var_vgbgmt_dn17 = assign25030_e33990_d_n17;
        locals.var_vgbgmt_rv = 0.0;

        let (assign25040_e34008, assign25040_e34008_d_n0, assign25040_e34008_d_n2, assign25040_e34008_d_n6, assign25040_e34008_d_n7, assign25040_e34008_d_n10, assign25040_e34008_d_n11, assign25040_e34008_d_n12, assign25040_e34008_d_n17,) = {
    if (((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign25040_e33998: f64 = (locals.var_flg_overs * locals.var_vsbgmt);
        let assign25040_e34001: f64 = (locals.var_flg_overd * locals.var_vdbgmt);
        let assign25040_e34002: f64 = (assign25040_e33998 + assign25040_e34001);
        let assign25040_e34005: f64 = (10.0 * 2.220446049250313e-16);
        let assign25040_e34006: f64 = (assign25040_e34002 + assign25040_e34005);
        (assign25040_e34006, ((locals.var_flg_overs * locals.var_vsbgmt_dn0) + (locals.var_flg_overd * locals.var_vdbgmt_dn0)), ((locals.var_flg_overs * locals.var_vsbgmt_dn2) + (locals.var_flg_overd * locals.var_vdbgmt_dn2)), ((locals.var_flg_overs * locals.var_vsbgmt_dn6) + (locals.var_flg_overd * locals.var_vdbgmt_dn6)), ((locals.var_flg_overs * locals.var_vsbgmt_dn7) + (locals.var_flg_overd * locals.var_vdbgmt_dn7)), ((locals.var_flg_overs * locals.var_vsbgmt_dn10) + (locals.var_flg_overd * locals.var_vdbgmt_dn10)), ((locals.var_flg_overs * locals.var_vsbgmt_dn11) + (locals.var_flg_overd * locals.var_vdbgmt_dn11)), ((locals.var_flg_overs * locals.var_vsbgmt_dn12) + (locals.var_flg_overd * locals.var_vdbgmt_dn12)), ((locals.var_flg_overs * locals.var_vsbgmt_dn17) + (locals.var_flg_overd * locals.var_vdbgmt_dn17)),)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn12, locals.var_vxbgmt_dn17,)
    }
};
        locals.var_vxbgmt = assign25040_e34008;
        locals.var_vxbgmt_dn0 = assign25040_e34008_d_n0;
        locals.var_vxbgmt_dn2 = assign25040_e34008_d_n2;
        locals.var_vxbgmt_dn6 = assign25040_e34008_d_n6;
        locals.var_vxbgmt_dn7 = assign25040_e34008_d_n7;
        locals.var_vxbgmt_dn10 = assign25040_e34008_d_n10;
        locals.var_vxbgmt_dn11 = assign25040_e34008_d_n11;
        locals.var_vxbgmt_dn12 = assign25040_e34008_d_n12;
        locals.var_vxbgmt_dn17 = assign25040_e34008_d_n17;
        locals.var_vxbgmt_rv = 0.0;

        let (assign25050_e34017, assign25050_e34017_d_n0, assign25050_e34017_d_n2, assign25050_e34017_d_n6, assign25050_e34017_d_n7, assign25050_e34017_d_n10, assign25050_e34017_d_n11, assign25050_e34017_d_n12, assign25050_e34017_d_n17,) = {
    if (((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign25050_e34015: f64 = (-locals.var_vxbgmt);
        (assign25050_e34015, (-locals.var_vxbgmt_dn0), (-locals.var_vxbgmt_dn2), (-locals.var_vxbgmt_dn6), (-locals.var_vxbgmt_dn7), (-locals.var_vxbgmt_dn10), (-locals.var_vxbgmt_dn11), (-locals.var_vxbgmt_dn12), (-locals.var_vxbgmt_dn17),)
    } else {
        (locals.var_t0__blk772, locals.var_t0__blk772_dn0, locals.var_t0__blk772_dn2, locals.var_t0__blk772_dn6, locals.var_t0__blk772_dn7, locals.var_t0__blk772_dn10, locals.var_t0__blk772_dn11, locals.var_t0__blk772_dn12, locals.var_t0__blk772_dn17,)
    }
};
        locals.var_t0__blk772 = assign25050_e34017;
        locals.var_t0__blk772_dn0 = assign25050_e34017_d_n0;
        locals.var_t0__blk772_dn2 = assign25050_e34017_d_n2;
        locals.var_t0__blk772_dn6 = assign25050_e34017_d_n6;
        locals.var_t0__blk772_dn7 = assign25050_e34017_d_n7;
        locals.var_t0__blk772_dn10 = assign25050_e34017_d_n10;
        locals.var_t0__blk772_dn11 = assign25050_e34017_d_n11;
        locals.var_t0__blk772_dn12 = assign25050_e34017_d_n12;
        locals.var_t0__blk772_dn17 = assign25050_e34017_d_n17;
        locals.var_t0__blk772_rv = 0.0;

        let assign25060_e34020: f64 = if locals.var_t0__blk772 > locals.var_vbs_bnd { 1.0 } else { 0.0 };
        locals.var_guard854 = assign25060_e34020;
        locals.var_guard854_rv = 0.0;

        let (assign25070_e34032, assign25070_e34032_d_n0, assign25070_e34032_d_n2, assign25070_e34032_d_n6, assign25070_e34032_d_n7, assign25070_e34032_d_n10, assign25070_e34032_d_n11, assign25070_e34032_d_n12, assign25070_e34032_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard854 != 0.0)) {
        let assign25070_e34030: f64 = (locals.var_t0__blk772 - locals.var_vbs_bnd);
        (assign25070_e34030, locals.var_t0__blk772_dn0, locals.var_t0__blk772_dn2, locals.var_t0__blk772_dn6, locals.var_t0__blk772_dn7, locals.var_t0__blk772_dn10, locals.var_t0__blk772_dn11, locals.var_t0__blk772_dn12, locals.var_t0__blk772_dn17,)
    } else {
        (locals.var_t1__blk773, locals.var_t1__blk773_dn0, locals.var_t1__blk773_dn2, locals.var_t1__blk773_dn6, locals.var_t1__blk773_dn7, locals.var_t1__blk773_dn10, locals.var_t1__blk773_dn11, locals.var_t1__blk773_dn12, locals.var_t1__blk773_dn17,)
    }
};
        locals.var_t1__blk773 = assign25070_e34032;
        locals.var_t1__blk773_dn0 = assign25070_e34032_d_n0;
        locals.var_t1__blk773_dn2 = assign25070_e34032_d_n2;
        locals.var_t1__blk773_dn6 = assign25070_e34032_d_n6;
        locals.var_t1__blk773_dn7 = assign25070_e34032_d_n7;
        locals.var_t1__blk773_dn10 = assign25070_e34032_d_n10;
        locals.var_t1__blk773_dn11 = assign25070_e34032_d_n11;
        locals.var_t1__blk773_dn12 = assign25070_e34032_d_n12;
        locals.var_t1__blk773_dn17 = assign25070_e34032_d_n17;
        locals.var_t1__blk773_rv = 0.0;

        let (assign25080_e34044, assign25080_e34044_d_n0, assign25080_e34044_d_n2, assign25080_e34044_d_n6, assign25080_e34044_d_n7, assign25080_e34044_d_n10, assign25080_e34044_d_n11, assign25080_e34044_d_n12, assign25080_e34044_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard854 != 0.0)) {
        let assign25080_e34042: f64 = (locals.var_vbs_max - locals.var_vbs_bnd);
        (assign25080_e34042, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk774, locals.var_t2__blk774_dn0, locals.var_t2__blk774_dn2, locals.var_t2__blk774_dn6, locals.var_t2__blk774_dn7, locals.var_t2__blk774_dn10, locals.var_t2__blk774_dn11, locals.var_t2__blk774_dn12, locals.var_t2__blk774_dn17,)
    }
};
        locals.var_t2__blk774 = assign25080_e34044;
        locals.var_t2__blk774_dn0 = assign25080_e34044_d_n0;
        locals.var_t2__blk774_dn2 = assign25080_e34044_d_n2;
        locals.var_t2__blk774_dn6 = assign25080_e34044_d_n6;
        locals.var_t2__blk774_dn7 = assign25080_e34044_d_n7;
        locals.var_t2__blk774_dn10 = assign25080_e34044_d_n10;
        locals.var_t2__blk774_dn11 = assign25080_e34044_d_n11;
        locals.var_t2__blk774_dn12 = assign25080_e34044_d_n12;
        locals.var_t2__blk774_dn17 = assign25080_e34044_d_n17;
        locals.var_t2__blk774_rv = 0.0;

        let (assign25090_e34056, assign25090_e34056_d_n0, assign25090_e34056_d_n2, assign25090_e34056_d_n6, assign25090_e34056_d_n7, assign25090_e34056_d_n10, assign25090_e34056_d_n11, assign25090_e34056_d_n12, assign25090_e34056_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard854 != 0.0)) {
        let assign25090_e34054: f64 = (locals.var_t1__blk773 / locals.var_t2__blk774);
        (assign25090_e34054, (((locals.var_t1__blk773_dn0 * locals.var_t2__blk774) - (locals.var_t1__blk773 * locals.var_t2__blk774_dn0)) / (locals.var_t2__blk774 * locals.var_t2__blk774)), (((locals.var_t1__blk773_dn2 * locals.var_t2__blk774) - (locals.var_t1__blk773 * locals.var_t2__blk774_dn2)) / (locals.var_t2__blk774 * locals.var_t2__blk774)), (((locals.var_t1__blk773_dn6 * locals.var_t2__blk774) - (locals.var_t1__blk773 * locals.var_t2__blk774_dn6)) / (locals.var_t2__blk774 * locals.var_t2__blk774)), (((locals.var_t1__blk773_dn7 * locals.var_t2__blk774) - (locals.var_t1__blk773 * locals.var_t2__blk774_dn7)) / (locals.var_t2__blk774 * locals.var_t2__blk774)), (((locals.var_t1__blk773_dn10 * locals.var_t2__blk774) - (locals.var_t1__blk773 * locals.var_t2__blk774_dn10)) / (locals.var_t2__blk774 * locals.var_t2__blk774)), (((locals.var_t1__blk773_dn11 * locals.var_t2__blk774) - (locals.var_t1__blk773 * locals.var_t2__blk774_dn11)) / (locals.var_t2__blk774 * locals.var_t2__blk774)), (((locals.var_t1__blk773_dn12 * locals.var_t2__blk774) - (locals.var_t1__blk773 * locals.var_t2__blk774_dn12)) / (locals.var_t2__blk774 * locals.var_t2__blk774)), (((locals.var_t1__blk773_dn17 * locals.var_t2__blk774) - (locals.var_t1__blk773 * locals.var_t2__blk774_dn17)) / (locals.var_t2__blk774 * locals.var_t2__blk774)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign25090_e34056;
        locals.var_tmf1_dn0 = assign25090_e34056_d_n0;
        locals.var_tmf1_dn2 = assign25090_e34056_d_n2;
        locals.var_tmf1_dn6 = assign25090_e34056_d_n6;
        locals.var_tmf1_dn7 = assign25090_e34056_d_n7;
        locals.var_tmf1_dn10 = assign25090_e34056_d_n10;
        locals.var_tmf1_dn11 = assign25090_e34056_d_n11;
        locals.var_tmf1_dn12 = assign25090_e34056_d_n12;
        locals.var_tmf1_dn17 = assign25090_e34056_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign25100_e34068, assign25100_e34068_d_n0, assign25100_e34068_d_n2, assign25100_e34068_d_n6, assign25100_e34068_d_n7, assign25100_e34068_d_n10, assign25100_e34068_d_n11, assign25100_e34068_d_n12, assign25100_e34068_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard854 != 0.0)) {
        let assign25100_e34066: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign25100_e34066, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign25100_e34068;
        locals.var_tmf2_dn0 = assign25100_e34068_d_n0;
        locals.var_tmf2_dn2 = assign25100_e34068_d_n2;
        locals.var_tmf2_dn6 = assign25100_e34068_d_n6;
        locals.var_tmf2_dn7 = assign25100_e34068_d_n7;
        locals.var_tmf2_dn10 = assign25100_e34068_d_n10;
        locals.var_tmf2_dn11 = assign25100_e34068_d_n11;
        locals.var_tmf2_dn12 = assign25100_e34068_d_n12;
        locals.var_tmf2_dn17 = assign25100_e34068_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign25110_e34080, assign25110_e34080_d_n0, assign25110_e34080_d_n2, assign25110_e34080_d_n6, assign25110_e34080_d_n7, assign25110_e34080_d_n10, assign25110_e34080_d_n11, assign25110_e34080_d_n12, assign25110_e34080_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard854 != 0.0)) {
        let assign25110_e34078: f64 = (locals.var_tmf2 * locals.var_tmf1);
        (assign25110_e34078, ((locals.var_tmf2_dn0 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn2)), ((locals.var_tmf2_dn6 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn7)), ((locals.var_tmf2_dn10 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn11)), ((locals.var_tmf2_dn12 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn12)), ((locals.var_tmf2_dn17 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn12, locals.var_tmf3_dn17,)
    }
};
        locals.var_tmf3 = assign25110_e34080;
        locals.var_tmf3_dn0 = assign25110_e34080_d_n0;
        locals.var_tmf3_dn2 = assign25110_e34080_d_n2;
        locals.var_tmf3_dn6 = assign25110_e34080_d_n6;
        locals.var_tmf3_dn7 = assign25110_e34080_d_n7;
        locals.var_tmf3_dn10 = assign25110_e34080_d_n10;
        locals.var_tmf3_dn11 = assign25110_e34080_d_n11;
        locals.var_tmf3_dn12 = assign25110_e34080_d_n12;
        locals.var_tmf3_dn17 = assign25110_e34080_d_n17;
        locals.var_tmf3_rv = 0.0;

        let (assign25120_e34092, assign25120_e34092_d_n0, assign25120_e34092_d_n2, assign25120_e34092_d_n6, assign25120_e34092_d_n7, assign25120_e34092_d_n10, assign25120_e34092_d_n11, assign25120_e34092_d_n12, assign25120_e34092_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard854 != 0.0)) {
        let assign25120_e34090: f64 = (locals.var_tmf2 * locals.var_tmf2);
        (assign25120_e34090, ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)), ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)), ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)), ((locals.var_tmf2_dn12 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn12)), ((locals.var_tmf2_dn17 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn17)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn6, locals.var_tmf4_dn7, locals.var_tmf4_dn10, locals.var_tmf4_dn11, locals.var_tmf4_dn12, locals.var_tmf4_dn17,)
    }
};
        locals.var_tmf4 = assign25120_e34092;
        locals.var_tmf4_dn0 = assign25120_e34092_d_n0;
        locals.var_tmf4_dn2 = assign25120_e34092_d_n2;
        locals.var_tmf4_dn6 = assign25120_e34092_d_n6;
        locals.var_tmf4_dn7 = assign25120_e34092_d_n7;
        locals.var_tmf4_dn10 = assign25120_e34092_d_n10;
        locals.var_tmf4_dn11 = assign25120_e34092_d_n11;
        locals.var_tmf4_dn12 = assign25120_e34092_d_n12;
        locals.var_tmf4_dn17 = assign25120_e34092_d_n17;
        locals.var_tmf4_rv = 0.0;

        let (assign25130_e34112, assign25130_e34112_d_n0, assign25130_e34112_d_n2, assign25130_e34112_d_n6, assign25130_e34112_d_n7, assign25130_e34112_d_n10, assign25130_e34112_d_n11, assign25130_e34112_d_n12, assign25130_e34112_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard854 != 0.0)) {
        let assign25130_e34103: f64 = (1.0 + locals.var_tmf1);
        let assign25130_e34105: f64 = (assign25130_e34103 + locals.var_tmf2);
        let assign25130_e34107: f64 = (assign25130_e34105 + locals.var_tmf3);
        let assign25130_e34109: f64 = (assign25130_e34107 + locals.var_tmf4);
        let assign25130_e34110: f64 = (1.0 / assign25130_e34109);
        (assign25130_e34110, (-((((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) + locals.var_tmf3_dn0) + locals.var_tmf4_dn0) / (assign25130_e34109 * assign25130_e34109))), (-((((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) + locals.var_tmf3_dn2) + locals.var_tmf4_dn2) / (assign25130_e34109 * assign25130_e34109))), (-((((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) + locals.var_tmf3_dn6) + locals.var_tmf4_dn6) / (assign25130_e34109 * assign25130_e34109))), (-((((locals.var_tmf1_dn7 + locals.var_tmf2_dn7) + locals.var_tmf3_dn7) + locals.var_tmf4_dn7) / (assign25130_e34109 * assign25130_e34109))), (-((((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) + locals.var_tmf3_dn10) + locals.var_tmf4_dn10) / (assign25130_e34109 * assign25130_e34109))), (-((((locals.var_tmf1_dn11 + locals.var_tmf2_dn11) + locals.var_tmf3_dn11) + locals.var_tmf4_dn11) / (assign25130_e34109 * assign25130_e34109))), (-((((locals.var_tmf1_dn12 + locals.var_tmf2_dn12) + locals.var_tmf3_dn12) + locals.var_tmf4_dn12) / (assign25130_e34109 * assign25130_e34109))), (-((((locals.var_tmf1_dn17 + locals.var_tmf2_dn17) + locals.var_tmf3_dn17) + locals.var_tmf4_dn17) / (assign25130_e34109 * assign25130_e34109))),)
    } else {
        (locals.var_ty__blk780, locals.var_ty__blk780_dn0, locals.var_ty__blk780_dn2, locals.var_ty__blk780_dn6, locals.var_ty__blk780_dn7, locals.var_ty__blk780_dn10, locals.var_ty__blk780_dn11, locals.var_ty__blk780_dn12, locals.var_ty__blk780_dn17,)
    }
};
        locals.var_ty__blk780 = assign25130_e34112;
        locals.var_ty__blk780_dn0 = assign25130_e34112_d_n0;
        locals.var_ty__blk780_dn2 = assign25130_e34112_d_n2;
        locals.var_ty__blk780_dn6 = assign25130_e34112_d_n6;
        locals.var_ty__blk780_dn7 = assign25130_e34112_d_n7;
        locals.var_ty__blk780_dn10 = assign25130_e34112_d_n10;
        locals.var_ty__blk780_dn11 = assign25130_e34112_d_n11;
        locals.var_ty__blk780_dn12 = assign25130_e34112_d_n12;
        locals.var_ty__blk780_dn17 = assign25130_e34112_d_n17;
        locals.var_ty__blk780_rv = 0.0;

        let (assign25150_e34153, assign25150_e34153_d_n0, assign25150_e34153_d_n2, assign25150_e34153_d_n6, assign25150_e34153_d_n7, assign25150_e34153_d_n10, assign25150_e34153_d_n11, assign25150_e34153_d_n12, assign25150_e34153_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard854 != 0.0)) {
        let assign25150_e34150: f64 = (1.0 - locals.var_ty__blk780);
        let assign25150_e34151: f64 = (locals.var_t2__blk774 * assign25150_e34150);
        (assign25150_e34151, ((locals.var_t2__blk774_dn0 * assign25150_e34150) + (locals.var_t2__blk774 * (-locals.var_ty__blk780_dn0))), ((locals.var_t2__blk774_dn2 * assign25150_e34150) + (locals.var_t2__blk774 * (-locals.var_ty__blk780_dn2))), ((locals.var_t2__blk774_dn6 * assign25150_e34150) + (locals.var_t2__blk774 * (-locals.var_ty__blk780_dn6))), ((locals.var_t2__blk774_dn7 * assign25150_e34150) + (locals.var_t2__blk774 * (-locals.var_ty__blk780_dn7))), ((locals.var_t2__blk774_dn10 * assign25150_e34150) + (locals.var_t2__blk774 * (-locals.var_ty__blk780_dn10))), ((locals.var_t2__blk774_dn11 * assign25150_e34150) + (locals.var_t2__blk774 * (-locals.var_ty__blk780_dn11))), ((locals.var_t2__blk774_dn12 * assign25150_e34150) + (locals.var_t2__blk774 * (-locals.var_ty__blk780_dn12))), ((locals.var_t2__blk774_dn17 * assign25150_e34150) + (locals.var_t2__blk774 * (-locals.var_ty__blk780_dn17))),)
    } else {
        (locals.var_ty__blk780, locals.var_ty__blk780_dn0, locals.var_ty__blk780_dn2, locals.var_ty__blk780_dn6, locals.var_ty__blk780_dn7, locals.var_ty__blk780_dn10, locals.var_ty__blk780_dn11, locals.var_ty__blk780_dn12, locals.var_ty__blk780_dn17,)
    }
};
        locals.var_ty__blk780 = assign25150_e34153;
        locals.var_ty__blk780_dn0 = assign25150_e34153_d_n0;
        locals.var_ty__blk780_dn2 = assign25150_e34153_d_n2;
        locals.var_ty__blk780_dn6 = assign25150_e34153_d_n6;
        locals.var_ty__blk780_dn7 = assign25150_e34153_d_n7;
        locals.var_ty__blk780_dn10 = assign25150_e34153_d_n10;
        locals.var_ty__blk780_dn11 = assign25150_e34153_d_n11;
        locals.var_ty__blk780_dn12 = assign25150_e34153_d_n12;
        locals.var_ty__blk780_dn17 = assign25150_e34153_d_n17;
        locals.var_ty__blk780_rv = 0.0;

        let (assign25170_e34176, assign25170_e34176_d_n0, assign25170_e34176_d_n2, assign25170_e34176_d_n6, assign25170_e34176_d_n7, assign25170_e34176_d_n10, assign25170_e34176_d_n11, assign25170_e34176_d_n12, assign25170_e34176_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard854 != 0.0)) {
        let assign25170_e34174: f64 = (locals.var_vbs_bnd + locals.var_ty__blk780);
        (assign25170_e34174, locals.var_ty__blk780_dn0, locals.var_ty__blk780_dn2, locals.var_ty__blk780_dn6, locals.var_ty__blk780_dn7, locals.var_ty__blk780_dn10, locals.var_ty__blk780_dn11, locals.var_ty__blk780_dn12, locals.var_ty__blk780_dn17,)
    } else {
        (locals.var_t10__blk777, locals.var_t10__blk777_dn0, locals.var_t10__blk777_dn2, locals.var_t10__blk777_dn6, locals.var_t10__blk777_dn7, locals.var_t10__blk777_dn10, locals.var_t10__blk777_dn11, locals.var_t10__blk777_dn12, locals.var_t10__blk777_dn17,)
    }
};
        locals.var_t10__blk777 = assign25170_e34176;
        locals.var_t10__blk777_dn0 = assign25170_e34176_d_n0;
        locals.var_t10__blk777_dn2 = assign25170_e34176_d_n2;
        locals.var_t10__blk777_dn6 = assign25170_e34176_d_n6;
        locals.var_t10__blk777_dn7 = assign25170_e34176_d_n7;
        locals.var_t10__blk777_dn10 = assign25170_e34176_d_n10;
        locals.var_t10__blk777_dn11 = assign25170_e34176_d_n11;
        locals.var_t10__blk777_dn12 = assign25170_e34176_d_n12;
        locals.var_t10__blk777_dn17 = assign25170_e34176_d_n17;
        locals.var_t10__blk777_rv = 0.0;

        let (assign25180_e34187, assign25180_e34187_d_n0, assign25180_e34187_d_n2, assign25180_e34187_d_n6, assign25180_e34187_d_n7, assign25180_e34187_d_n10, assign25180_e34187_d_n11, assign25180_e34187_d_n12, assign25180_e34187_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard854 == 0.0)) {
        (locals.var_t0__blk772, locals.var_t0__blk772_dn0, locals.var_t0__blk772_dn2, locals.var_t0__blk772_dn6, locals.var_t0__blk772_dn7, locals.var_t0__blk772_dn10, locals.var_t0__blk772_dn11, locals.var_t0__blk772_dn12, locals.var_t0__blk772_dn17,)
    } else {
        (locals.var_t10__blk777, locals.var_t10__blk777_dn0, locals.var_t10__blk777_dn2, locals.var_t10__blk777_dn6, locals.var_t10__blk777_dn7, locals.var_t10__blk777_dn10, locals.var_t10__blk777_dn11, locals.var_t10__blk777_dn12, locals.var_t10__blk777_dn17,)
    }
};
        locals.var_t10__blk777 = assign25180_e34187;
        locals.var_t10__blk777_dn0 = assign25180_e34187_d_n0;
        locals.var_t10__blk777_dn2 = assign25180_e34187_d_n2;
        locals.var_t10__blk777_dn6 = assign25180_e34187_d_n6;
        locals.var_t10__blk777_dn7 = assign25180_e34187_d_n7;
        locals.var_t10__blk777_dn10 = assign25180_e34187_d_n10;
        locals.var_t10__blk777_dn11 = assign25180_e34187_d_n11;
        locals.var_t10__blk777_dn12 = assign25180_e34187_d_n12;
        locals.var_t10__blk777_dn17 = assign25180_e34187_d_n17;
        locals.var_t10__blk777_rv = 0.0;

        let (assign25200_e34209, assign25200_e34209_d_n0, assign25200_e34209_d_n2, assign25200_e34209_d_n6, assign25200_e34209_d_n7, assign25200_e34209_d_n10, assign25200_e34209_d_n11, assign25200_e34209_d_n12, assign25200_e34209_d_n17,) = {
    if (((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign25200_e34205: f64 = (-locals.var_t10__blk777);
        let assign25200_e34207: f64 = (assign25200_e34205 - 1e-12);
        (assign25200_e34207, (-locals.var_t10__blk777_dn0), (-locals.var_t10__blk777_dn2), (-locals.var_t10__blk777_dn6), (-locals.var_t10__blk777_dn7), (-locals.var_t10__blk777_dn10), (-locals.var_t10__blk777_dn11), (-locals.var_t10__blk777_dn12), (-locals.var_t10__blk777_dn17),)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn12, locals.var_vxbgmtcl_dn17,)
    }
};
        locals.var_vxbgmtcl = assign25200_e34209;
        locals.var_vxbgmtcl_dn0 = assign25200_e34209_d_n0;
        locals.var_vxbgmtcl_dn2 = assign25200_e34209_d_n2;
        locals.var_vxbgmtcl_dn6 = assign25200_e34209_d_n6;
        locals.var_vxbgmtcl_dn7 = assign25200_e34209_d_n7;
        locals.var_vxbgmtcl_dn10 = assign25200_e34209_d_n10;
        locals.var_vxbgmtcl_dn11 = assign25200_e34209_d_n11;
        locals.var_vxbgmtcl_dn12 = assign25200_e34209_d_n12;
        locals.var_vxbgmtcl_dn17 = assign25200_e34209_d_n17;
        locals.var_vxbgmtcl_rv = 0.0;

        let (assign25210_e34219, assign25210_e34219_d_n0, assign25210_e34219_d_n2, assign25210_e34219_d_n6, assign25210_e34219_d_n7, assign25210_e34219_d_n10, assign25210_e34219_d_n11, assign25210_e34219_d_n12, assign25210_e34219_d_n17,) = {
    if (((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign25210_e34217: f64 = (locals.var_cnst0over * locals.var_cox0_inv);
        (assign25210_e34217, (locals.var_cnst0over_dn0 * locals.var_cox0_inv), (locals.var_cnst0over_dn2 * locals.var_cox0_inv), (locals.var_cnst0over_dn6 * locals.var_cox0_inv), (locals.var_cnst0over_dn7 * locals.var_cox0_inv), (locals.var_cnst0over_dn10 * locals.var_cox0_inv), (locals.var_cnst0over_dn11 * locals.var_cox0_inv), (locals.var_cnst0over_dn12 * locals.var_cox0_inv), (locals.var_cnst0over_dn17 * locals.var_cox0_inv),)
    } else {
        (locals.var_fac1__blk802, locals.var_fac1__blk802_dn0, locals.var_fac1__blk802_dn2, locals.var_fac1__blk802_dn6, locals.var_fac1__blk802_dn7, locals.var_fac1__blk802_dn10, locals.var_fac1__blk802_dn11, locals.var_fac1__blk802_dn12, locals.var_fac1__blk802_dn17,)
    }
};
        locals.var_fac1__blk802 = assign25210_e34219;
        locals.var_fac1__blk802_dn0 = assign25210_e34219_d_n0;
        locals.var_fac1__blk802_dn2 = assign25210_e34219_d_n2;
        locals.var_fac1__blk802_dn6 = assign25210_e34219_d_n6;
        locals.var_fac1__blk802_dn7 = assign25210_e34219_d_n7;
        locals.var_fac1__blk802_dn10 = assign25210_e34219_d_n10;
        locals.var_fac1__blk802_dn11 = assign25210_e34219_d_n11;
        locals.var_fac1__blk802_dn12 = assign25210_e34219_d_n12;
        locals.var_fac1__blk802_dn17 = assign25210_e34219_d_n17;
        locals.var_fac1__blk802_rv = 0.0;

        let (assign25220_e34229, assign25220_e34229_d_n0, assign25220_e34229_d_n2, assign25220_e34229_d_n6, assign25220_e34229_d_n7, assign25220_e34229_d_n10, assign25220_e34229_d_n11, assign25220_e34229_d_n12, assign25220_e34229_d_n17,) = {
    if (((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign25220_e34227: f64 = (locals.var_fac1__blk802 * locals.var_fac1__blk802);
        (assign25220_e34227, ((locals.var_fac1__blk802_dn0 * locals.var_fac1__blk802) + (locals.var_fac1__blk802 * locals.var_fac1__blk802_dn0)), ((locals.var_fac1__blk802_dn2 * locals.var_fac1__blk802) + (locals.var_fac1__blk802 * locals.var_fac1__blk802_dn2)), ((locals.var_fac1__blk802_dn6 * locals.var_fac1__blk802) + (locals.var_fac1__blk802 * locals.var_fac1__blk802_dn6)), ((locals.var_fac1__blk802_dn7 * locals.var_fac1__blk802) + (locals.var_fac1__blk802 * locals.var_fac1__blk802_dn7)), ((locals.var_fac1__blk802_dn10 * locals.var_fac1__blk802) + (locals.var_fac1__blk802 * locals.var_fac1__blk802_dn10)), ((locals.var_fac1__blk802_dn11 * locals.var_fac1__blk802) + (locals.var_fac1__blk802 * locals.var_fac1__blk802_dn11)), ((locals.var_fac1__blk802_dn12 * locals.var_fac1__blk802) + (locals.var_fac1__blk802 * locals.var_fac1__blk802_dn12)), ((locals.var_fac1__blk802_dn17 * locals.var_fac1__blk802) + (locals.var_fac1__blk802 * locals.var_fac1__blk802_dn17)),)
    } else {
        (locals.var_fac1p2__blk803, locals.var_fac1p2__blk803_dn0, locals.var_fac1p2__blk803_dn2, locals.var_fac1p2__blk803_dn6, locals.var_fac1p2__blk803_dn7, locals.var_fac1p2__blk803_dn10, locals.var_fac1p2__blk803_dn11, locals.var_fac1p2__blk803_dn12, locals.var_fac1p2__blk803_dn17,)
    }
};
        locals.var_fac1p2__blk803 = assign25220_e34229;
        locals.var_fac1p2__blk803_dn0 = assign25220_e34229_d_n0;
        locals.var_fac1p2__blk803_dn2 = assign25220_e34229_d_n2;
        locals.var_fac1p2__blk803_dn6 = assign25220_e34229_d_n6;
        locals.var_fac1p2__blk803_dn7 = assign25220_e34229_d_n7;
        locals.var_fac1p2__blk803_dn10 = assign25220_e34229_d_n10;
        locals.var_fac1p2__blk803_dn11 = assign25220_e34229_d_n11;
        locals.var_fac1p2__blk803_dn12 = assign25220_e34229_d_n12;
        locals.var_fac1p2__blk803_dn17 = assign25220_e34229_d_n17;
        locals.var_fac1p2__blk803_rv = 0.0;

        let (assign25230_e34239, assign25230_e34239_d_n0, assign25230_e34239_d_n2, assign25230_e34239_d_n6, assign25230_e34239_d_n7, assign25230_e34239_d_n10, assign25230_e34239_d_n11, assign25230_e34239_d_n12, assign25230_e34239_d_n17,) = {
    if (((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign25230_e34237: f64 = (locals.var_vgbgmt - locals.var_uc_vfbbt);
        (assign25230_e34237, locals.var_vgbgmt_dn0, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn6, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn10, locals.var_vgbgmt_dn11, locals.var_vgbgmt_dn12, locals.var_vgbgmt_dn17,)
    } else {
        (locals.var_vgpld, locals.var_vgpld_dn0, locals.var_vgpld_dn2, locals.var_vgpld_dn6, locals.var_vgpld_dn7, locals.var_vgpld_dn10, locals.var_vgpld_dn11, locals.var_vgpld_dn12, locals.var_vgpld_dn17,)
    }
};
        locals.var_vgpld = assign25230_e34239;
        locals.var_vgpld_dn0 = assign25230_e34239_d_n0;
        locals.var_vgpld_dn2 = assign25230_e34239_d_n2;
        locals.var_vgpld_dn6 = assign25230_e34239_d_n6;
        locals.var_vgpld_dn7 = assign25230_e34239_d_n7;
        locals.var_vgpld_dn10 = assign25230_e34239_d_n10;
        locals.var_vgpld_dn11 = assign25230_e34239_d_n11;
        locals.var_vgpld_dn12 = assign25230_e34239_d_n12;
        locals.var_vgpld_dn17 = assign25230_e34239_d_n17;
        locals.var_vgpld_rv = 0.0;

        let (assign25240_e34249, assign25240_e34249_d_n0, assign25240_e34249_d_n2, assign25240_e34249_d_n6, assign25240_e34249_d_n7, assign25240_e34249_d_n10, assign25240_e34249_d_n11, assign25240_e34249_d_n12, assign25240_e34249_d_n17,) = {
    if (((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign25240_e34247: f64 = (locals.var_uc_nsubbttub / locals.var_nin);
        (assign25240_e34247, (((locals.var_uc_nsubbttub_dn0 * locals.var_nin) - (locals.var_uc_nsubbttub * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)), (((locals.var_uc_nsubbttub_dn2 * locals.var_nin) - (locals.var_uc_nsubbttub * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)), (((locals.var_uc_nsubbttub_dn6 * locals.var_nin) - (locals.var_uc_nsubbttub * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)), (((locals.var_uc_nsubbttub_dn7 * locals.var_nin) - (locals.var_uc_nsubbttub * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)), (((locals.var_uc_nsubbttub_dn10 * locals.var_nin) - (locals.var_uc_nsubbttub * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)), (((locals.var_uc_nsubbttub_dn11 * locals.var_nin) - (locals.var_uc_nsubbttub * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)), (((locals.var_uc_nsubbttub_dn12 * locals.var_nin) - (locals.var_uc_nsubbttub * locals.var_nin_dn12)) / (locals.var_nin * locals.var_nin)), (((locals.var_uc_nsubbttub_dn17 * locals.var_nin) - (locals.var_uc_nsubbttub * locals.var_nin_dn17)) / (locals.var_nin * locals.var_nin)),)
    } else {
        (locals.var_t0__blk772, locals.var_t0__blk772_dn0, locals.var_t0__blk772_dn2, locals.var_t0__blk772_dn6, locals.var_t0__blk772_dn7, locals.var_t0__blk772_dn10, locals.var_t0__blk772_dn11, locals.var_t0__blk772_dn12, locals.var_t0__blk772_dn17,)
    }
};
        locals.var_t0__blk772 = assign25240_e34249;
        locals.var_t0__blk772_dn0 = assign25240_e34249_d_n0;
        locals.var_t0__blk772_dn2 = assign25240_e34249_d_n2;
        locals.var_t0__blk772_dn6 = assign25240_e34249_d_n6;
        locals.var_t0__blk772_dn7 = assign25240_e34249_d_n7;
        locals.var_t0__blk772_dn10 = assign25240_e34249_d_n10;
        locals.var_t0__blk772_dn11 = assign25240_e34249_d_n11;
        locals.var_t0__blk772_dn12 = assign25240_e34249_d_n12;
        locals.var_t0__blk772_dn17 = assign25240_e34249_d_n17;
        locals.var_t0__blk772_rv = 0.0;

        let (assign25250_e34262, assign25250_e34262_d_n0, assign25250_e34262_d_n2, assign25250_e34262_d_n6, assign25250_e34262_d_n7, assign25250_e34262_d_n10, assign25250_e34262_d_n11, assign25250_e34262_d_n12, assign25250_e34262_d_n17,) = {
    if (((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign25250_e34257: f64 = (2.0 / locals.var_beta);
        let assign25250_e34259: f64 = (locals.var_t0__blk772).ln();
        let assign25250_e34260: f64 = (assign25250_e34257 * assign25250_e34259);
        (assign25250_e34260, (assign25250_e34257 * (locals.var_t0__blk772_dn0 / locals.var_t0__blk772)), (assign25250_e34257 * (locals.var_t0__blk772_dn2 / locals.var_t0__blk772)), (assign25250_e34257 * (locals.var_t0__blk772_dn6 / locals.var_t0__blk772)), (assign25250_e34257 * (locals.var_t0__blk772_dn7 / locals.var_t0__blk772)), (((-((2.0 * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) * assign25250_e34259) + (assign25250_e34257 * (locals.var_t0__blk772_dn10 / locals.var_t0__blk772))), (assign25250_e34257 * (locals.var_t0__blk772_dn11 / locals.var_t0__blk772)), (assign25250_e34257 * (locals.var_t0__blk772_dn12 / locals.var_t0__blk772)), (assign25250_e34257 * (locals.var_t0__blk772_dn17 / locals.var_t0__blk772)),)
    } else {
        (locals.var_pb2over, locals.var_pb2over_dn0, locals.var_pb2over_dn2, locals.var_pb2over_dn6, locals.var_pb2over_dn7, locals.var_pb2over_dn10, locals.var_pb2over_dn11, locals.var_pb2over_dn12, locals.var_pb2over_dn17,)
    }
};
        locals.var_pb2over = assign25250_e34262;
        locals.var_pb2over_dn0 = assign25250_e34262_d_n0;
        locals.var_pb2over_dn2 = assign25250_e34262_d_n2;
        locals.var_pb2over_dn6 = assign25250_e34262_d_n6;
        locals.var_pb2over_dn7 = assign25250_e34262_d_n7;
        locals.var_pb2over_dn10 = assign25250_e34262_d_n10;
        locals.var_pb2over_dn11 = assign25250_e34262_d_n11;
        locals.var_pb2over_dn12 = assign25250_e34262_d_n12;
        locals.var_pb2over_dn17 = assign25250_e34262_d_n17;
        locals.var_pb2over_rv = 0.0;

        let (assign25260_e34271, assign25260_e34271_d_n0, assign25260_e34271_d_n2, assign25260_e34271_d_n6, assign25260_e34271_d_n7, assign25260_e34271_d_n10, assign25260_e34271_d_n11, assign25260_e34271_d_n12, assign25260_e34271_d_n17,) = {
    if (((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign25260_e34269: f64 = (-locals.var_vxbgmtcl);
        (assign25260_e34269, (-locals.var_vxbgmtcl_dn0), (-locals.var_vxbgmtcl_dn2), (-locals.var_vxbgmtcl_dn6), (-locals.var_vxbgmtcl_dn7), (-locals.var_vxbgmtcl_dn10), (-locals.var_vxbgmtcl_dn11), (-locals.var_vxbgmtcl_dn12), (-locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_vgb_fb_ld, locals.var_vgb_fb_ld_dn0, locals.var_vgb_fb_ld_dn2, locals.var_vgb_fb_ld_dn6, locals.var_vgb_fb_ld_dn7, locals.var_vgb_fb_ld_dn10, locals.var_vgb_fb_ld_dn11, locals.var_vgb_fb_ld_dn12, locals.var_vgb_fb_ld_dn17,)
    }
};
        locals.var_vgb_fb_ld = assign25260_e34271;
        locals.var_vgb_fb_ld_dn0 = assign25260_e34271_d_n0;
        locals.var_vgb_fb_ld_dn2 = assign25260_e34271_d_n2;
        locals.var_vgb_fb_ld_dn6 = assign25260_e34271_d_n6;
        locals.var_vgb_fb_ld_dn7 = assign25260_e34271_d_n7;
        locals.var_vgb_fb_ld_dn10 = assign25260_e34271_d_n10;
        locals.var_vgb_fb_ld_dn11 = assign25260_e34271_d_n11;
        locals.var_vgb_fb_ld_dn12 = assign25260_e34271_d_n12;
        locals.var_vgb_fb_ld_dn17 = assign25260_e34271_d_n17;
        locals.var_vgb_fb_ld_rv = 0.0;

        let assign25270_e34274: f64 = if locals.var_vgpld < locals.var_vgb_fb_ld { 1.0 } else { 0.0 };
        locals.var_guard855 = assign25270_e34274;
        locals.var_guard855_rv = 0.0;

        let (assign25290_e34299, assign25290_e34299_d_n0, assign25290_e34299_d_n2, assign25290_e34299_d_n6, assign25290_e34299_d_n7, assign25290_e34299_d_n10, assign25290_e34299_d_n11, assign25290_e34299_d_n12, assign25290_e34299_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign25290_e34296: f64 = (locals.var_beta * locals.var_cnst0over);
        let assign25290_e34297: f64 = (1.0 / assign25290_e34296);
        (assign25290_e34297, (-((locals.var_beta * locals.var_cnst0over_dn0) / (assign25290_e34296 * assign25290_e34296))), (-((locals.var_beta * locals.var_cnst0over_dn2) / (assign25290_e34296 * assign25290_e34296))), (-((locals.var_beta * locals.var_cnst0over_dn6) / (assign25290_e34296 * assign25290_e34296))), (-((locals.var_beta * locals.var_cnst0over_dn7) / (assign25290_e34296 * assign25290_e34296))), (-(((locals.var_beta_dn10 * locals.var_cnst0over) + (locals.var_beta * locals.var_cnst0over_dn10)) / (assign25290_e34296 * assign25290_e34296))), (-((locals.var_beta * locals.var_cnst0over_dn11) / (assign25290_e34296 * assign25290_e34296))), (-((locals.var_beta * locals.var_cnst0over_dn12) / (assign25290_e34296 * assign25290_e34296))), (-((locals.var_beta * locals.var_cnst0over_dn17) / (assign25290_e34296 * assign25290_e34296))),)
    } else {
        (locals.var_t1__blk773, locals.var_t1__blk773_dn0, locals.var_t1__blk773_dn2, locals.var_t1__blk773_dn6, locals.var_t1__blk773_dn7, locals.var_t1__blk773_dn10, locals.var_t1__blk773_dn11, locals.var_t1__blk773_dn12, locals.var_t1__blk773_dn17,)
    }
};
        locals.var_t1__blk773 = assign25290_e34299;
        locals.var_t1__blk773_dn0 = assign25290_e34299_d_n0;
        locals.var_t1__blk773_dn2 = assign25290_e34299_d_n2;
        locals.var_t1__blk773_dn6 = assign25290_e34299_d_n6;
        locals.var_t1__blk773_dn7 = assign25290_e34299_d_n7;
        locals.var_t1__blk773_dn10 = assign25290_e34299_d_n10;
        locals.var_t1__blk773_dn11 = assign25290_e34299_d_n11;
        locals.var_t1__blk773_dn12 = assign25290_e34299_d_n12;
        locals.var_t1__blk773_dn17 = assign25290_e34299_d_n17;
        locals.var_t1__blk773_rv = 0.0;

        let (assign25300_e34311, assign25300_e34311_d_n0, assign25300_e34311_d_n2, assign25300_e34311_d_n6, assign25300_e34311_d_n7, assign25300_e34311_d_n10, assign25300_e34311_d_n11, assign25300_e34311_d_n12, assign25300_e34311_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign25300_e34309: f64 = (locals.var_t1__blk773 * locals.var_cox0);
        (assign25300_e34309, (locals.var_t1__blk773_dn0 * locals.var_cox0), (locals.var_t1__blk773_dn2 * locals.var_cox0), (locals.var_t1__blk773_dn6 * locals.var_cox0), (locals.var_t1__blk773_dn7 * locals.var_cox0), (locals.var_t1__blk773_dn10 * locals.var_cox0), (locals.var_t1__blk773_dn11 * locals.var_cox0), (locals.var_t1__blk773_dn12 * locals.var_cox0), (locals.var_t1__blk773_dn17 * locals.var_cox0),)
    } else {
        (locals.var_ty__blk780, locals.var_ty__blk780_dn0, locals.var_ty__blk780_dn2, locals.var_ty__blk780_dn6, locals.var_ty__blk780_dn7, locals.var_ty__blk780_dn10, locals.var_ty__blk780_dn11, locals.var_ty__blk780_dn12, locals.var_ty__blk780_dn17,)
    }
};
        locals.var_ty__blk780 = assign25300_e34311;
        locals.var_ty__blk780_dn0 = assign25300_e34311_d_n0;
        locals.var_ty__blk780_dn2 = assign25300_e34311_d_n2;
        locals.var_ty__blk780_dn6 = assign25300_e34311_d_n6;
        locals.var_ty__blk780_dn7 = assign25300_e34311_d_n7;
        locals.var_ty__blk780_dn10 = assign25300_e34311_d_n10;
        locals.var_ty__blk780_dn11 = assign25300_e34311_d_n11;
        locals.var_ty__blk780_dn12 = assign25300_e34311_d_n12;
        locals.var_ty__blk780_dn17 = assign25300_e34311_d_n17;
        locals.var_ty__blk780_rv = 0.0;

        let (assign25310_e34327, assign25310_e34327_d_n0, assign25310_e34327_d_n2, assign25310_e34327_d_n6, assign25310_e34327_d_n7, assign25310_e34327_d_n10, assign25310_e34327_d_n11, assign25310_e34327_d_n12, assign25310_e34327_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign25310_e34322: f64 = (3.0 * 1.414213562373095);
        let assign25310_e34324: f64 = (assign25310_e34322 * locals.var_ty__blk780);
        let assign25310_e34325: f64 = (2.0 + assign25310_e34324);
        (assign25310_e34325, (assign25310_e34322 * locals.var_ty__blk780_dn0), (assign25310_e34322 * locals.var_ty__blk780_dn2), (assign25310_e34322 * locals.var_ty__blk780_dn6), (assign25310_e34322 * locals.var_ty__blk780_dn7), (assign25310_e34322 * locals.var_ty__blk780_dn10), (assign25310_e34322 * locals.var_ty__blk780_dn11), (assign25310_e34322 * locals.var_ty__blk780_dn12), (assign25310_e34322 * locals.var_ty__blk780_dn17),)
    } else {
        (locals.var_ac41__blk807, locals.var_ac41__blk807_dn0, locals.var_ac41__blk807_dn2, locals.var_ac41__blk807_dn6, locals.var_ac41__blk807_dn7, locals.var_ac41__blk807_dn10, locals.var_ac41__blk807_dn11, locals.var_ac41__blk807_dn12, locals.var_ac41__blk807_dn17,)
    }
};
        locals.var_ac41__blk807 = assign25310_e34327;
        locals.var_ac41__blk807_dn0 = assign25310_e34327_d_n0;
        locals.var_ac41__blk807_dn2 = assign25310_e34327_d_n2;
        locals.var_ac41__blk807_dn6 = assign25310_e34327_d_n6;
        locals.var_ac41__blk807_dn7 = assign25310_e34327_d_n7;
        locals.var_ac41__blk807_dn10 = assign25310_e34327_d_n10;
        locals.var_ac41__blk807_dn11 = assign25310_e34327_d_n11;
        locals.var_ac41__blk807_dn12 = assign25310_e34327_d_n12;
        locals.var_ac41__blk807_dn17 = assign25310_e34327_d_n17;
        locals.var_ac41__blk807_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_89(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign25320_e34343, assign25320_e34343_d_n0, assign25320_e34343_d_n2, assign25320_e34343_d_n6, assign25320_e34343_d_n7, assign25320_e34343_d_n10, assign25320_e34343_d_n11, assign25320_e34343_d_n12, assign25320_e34343_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign25320_e34337: f64 = (8.0 * locals.var_ac41__blk807);
        let assign25320_e34339: f64 = (assign25320_e34337 * locals.var_ac41__blk807);
        let assign25320_e34341: f64 = (assign25320_e34339 * locals.var_ac41__blk807);
        (assign25320_e34341, (((((8.0 * locals.var_ac41__blk807_dn0) * locals.var_ac41__blk807) + (assign25320_e34337 * locals.var_ac41__blk807_dn0)) * locals.var_ac41__blk807) + (assign25320_e34339 * locals.var_ac41__blk807_dn0)), (((((8.0 * locals.var_ac41__blk807_dn2) * locals.var_ac41__blk807) + (assign25320_e34337 * locals.var_ac41__blk807_dn2)) * locals.var_ac41__blk807) + (assign25320_e34339 * locals.var_ac41__blk807_dn2)), (((((8.0 * locals.var_ac41__blk807_dn6) * locals.var_ac41__blk807) + (assign25320_e34337 * locals.var_ac41__blk807_dn6)) * locals.var_ac41__blk807) + (assign25320_e34339 * locals.var_ac41__blk807_dn6)), (((((8.0 * locals.var_ac41__blk807_dn7) * locals.var_ac41__blk807) + (assign25320_e34337 * locals.var_ac41__blk807_dn7)) * locals.var_ac41__blk807) + (assign25320_e34339 * locals.var_ac41__blk807_dn7)), (((((8.0 * locals.var_ac41__blk807_dn10) * locals.var_ac41__blk807) + (assign25320_e34337 * locals.var_ac41__blk807_dn10)) * locals.var_ac41__blk807) + (assign25320_e34339 * locals.var_ac41__blk807_dn10)), (((((8.0 * locals.var_ac41__blk807_dn11) * locals.var_ac41__blk807) + (assign25320_e34337 * locals.var_ac41__blk807_dn11)) * locals.var_ac41__blk807) + (assign25320_e34339 * locals.var_ac41__blk807_dn11)), (((((8.0 * locals.var_ac41__blk807_dn12) * locals.var_ac41__blk807) + (assign25320_e34337 * locals.var_ac41__blk807_dn12)) * locals.var_ac41__blk807) + (assign25320_e34339 * locals.var_ac41__blk807_dn12)), (((((8.0 * locals.var_ac41__blk807_dn17) * locals.var_ac41__blk807) + (assign25320_e34337 * locals.var_ac41__blk807_dn17)) * locals.var_ac41__blk807) + (assign25320_e34339 * locals.var_ac41__blk807_dn17)),)
    } else {
        (locals.var_ac4__blk808, locals.var_ac4__blk808_dn0, locals.var_ac4__blk808_dn2, locals.var_ac4__blk808_dn6, locals.var_ac4__blk808_dn7, locals.var_ac4__blk808_dn10, locals.var_ac4__blk808_dn11, locals.var_ac4__blk808_dn12, locals.var_ac4__blk808_dn17,)
    }
};
        locals.var_ac4__blk808 = assign25320_e34343;
        locals.var_ac4__blk808_dn0 = assign25320_e34343_d_n0;
        locals.var_ac4__blk808_dn2 = assign25320_e34343_d_n2;
        locals.var_ac4__blk808_dn6 = assign25320_e34343_d_n6;
        locals.var_ac4__blk808_dn7 = assign25320_e34343_d_n7;
        locals.var_ac4__blk808_dn10 = assign25320_e34343_d_n10;
        locals.var_ac4__blk808_dn11 = assign25320_e34343_d_n11;
        locals.var_ac4__blk808_dn12 = assign25320_e34343_d_n12;
        locals.var_ac4__blk808_dn17 = assign25320_e34343_d_n17;
        locals.var_ac4__blk808_rv = 0.0;

        let (assign25330_e34355, assign25330_e34355_d_n0, assign25330_e34355_d_n2, assign25330_e34355_d_n6, assign25330_e34355_d_n7, assign25330_e34355_d_n10, assign25330_e34355_d_n11, assign25330_e34355_d_n12, assign25330_e34355_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign25330_e34353: f64 = (locals.var_eg - locals.var_pb2over);
        (assign25330_e34353, (locals.var_eg_dn0 - locals.var_pb2over_dn0), (locals.var_eg_dn2 - locals.var_pb2over_dn2), (locals.var_eg_dn6 - locals.var_pb2over_dn6), (locals.var_eg_dn7 - locals.var_pb2over_dn7), (locals.var_eg_dn10 - locals.var_pb2over_dn10), (locals.var_eg_dn11 - locals.var_pb2over_dn11), (locals.var_eg_dn12 - locals.var_pb2over_dn12), (locals.var_eg_dn17 - locals.var_pb2over_dn17),)
    } else {
        (locals.var_ps0_min__blk809, locals.var_ps0_min__blk809_dn0, locals.var_ps0_min__blk809_dn2, locals.var_ps0_min__blk809_dn6, locals.var_ps0_min__blk809_dn7, locals.var_ps0_min__blk809_dn10, locals.var_ps0_min__blk809_dn11, locals.var_ps0_min__blk809_dn12, locals.var_ps0_min__blk809_dn17,)
    }
};
        locals.var_ps0_min__blk809 = assign25330_e34355;
        locals.var_ps0_min__blk809_dn0 = assign25330_e34355_d_n0;
        locals.var_ps0_min__blk809_dn2 = assign25330_e34355_d_n2;
        locals.var_ps0_min__blk809_dn6 = assign25330_e34355_d_n6;
        locals.var_ps0_min__blk809_dn7 = assign25330_e34355_d_n7;
        locals.var_ps0_min__blk809_dn10 = assign25330_e34355_d_n10;
        locals.var_ps0_min__blk809_dn11 = assign25330_e34355_d_n11;
        locals.var_ps0_min__blk809_dn12 = assign25330_e34355_d_n12;
        locals.var_ps0_min__blk809_dn17 = assign25330_e34355_d_n17;
        locals.var_ps0_min__blk809_rv = 0.0;

        let (assign25340_e34369, assign25340_e34369_d_n0, assign25340_e34369_d_n2, assign25340_e34369_d_n6, assign25340_e34369_d_n7, assign25340_e34369_d_n10, assign25340_e34369_d_n11, assign25340_e34369_d_n12, assign25340_e34369_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign25340_e34366: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign25340_e34367: f64 = (locals.var_beta * assign25340_e34366);
        (assign25340_e34367, (locals.var_beta * (locals.var_vgpld_dn0 + locals.var_vxbgmtcl_dn0)), (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)), (locals.var_beta * (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6)), (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)), ((locals.var_beta_dn10 * assign25340_e34366) + (locals.var_beta * (locals.var_vgpld_dn10 + locals.var_vxbgmtcl_dn10))), (locals.var_beta * (locals.var_vgpld_dn11 + locals.var_vxbgmtcl_dn11)), (locals.var_beta * (locals.var_vgpld_dn12 + locals.var_vxbgmtcl_dn12)), (locals.var_beta * (locals.var_vgpld_dn17 + locals.var_vxbgmtcl_dn17)),)
    } else {
        (locals.var_tx__blk779, locals.var_tx__blk779_dn0, locals.var_tx__blk779_dn2, locals.var_tx__blk779_dn6, locals.var_tx__blk779_dn7, locals.var_tx__blk779_dn10, locals.var_tx__blk779_dn11, locals.var_tx__blk779_dn12, locals.var_tx__blk779_dn17,)
    }
};
        locals.var_tx__blk779 = assign25340_e34369;
        locals.var_tx__blk779_dn0 = assign25340_e34369_d_n0;
        locals.var_tx__blk779_dn2 = assign25340_e34369_d_n2;
        locals.var_tx__blk779_dn6 = assign25340_e34369_d_n6;
        locals.var_tx__blk779_dn7 = assign25340_e34369_d_n7;
        locals.var_tx__blk779_dn10 = assign25340_e34369_d_n10;
        locals.var_tx__blk779_dn11 = assign25340_e34369_d_n11;
        locals.var_tx__blk779_dn12 = assign25340_e34369_d_n12;
        locals.var_tx__blk779_dn17 = assign25340_e34369_d_n17;
        locals.var_tx__blk779_rv = 0.0;

        let (assign25350_e34389, assign25350_e34389_d_n0, assign25350_e34389_d_n2, assign25350_e34389_d_n6, assign25350_e34389_d_n7, assign25350_e34389_d_n10, assign25350_e34389_d_n11, assign25350_e34389_d_n12, assign25350_e34389_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign25350_e34379: f64 = (7.0 * 1.414213562373095);
        let assign25350_e34382: f64 = (9.0 * locals.var_ty__blk780);
        let assign25350_e34385: f64 = (locals.var_tx__blk779 - 2.0);
        let assign25350_e34386: f64 = (assign25350_e34382 * assign25350_e34385);
        let assign25350_e34387: f64 = (assign25350_e34379 - assign25350_e34386);
        (assign25350_e34387, (-(((9.0 * locals.var_ty__blk780_dn0) * assign25350_e34385) + (assign25350_e34382 * locals.var_tx__blk779_dn0))), (-(((9.0 * locals.var_ty__blk780_dn2) * assign25350_e34385) + (assign25350_e34382 * locals.var_tx__blk779_dn2))), (-(((9.0 * locals.var_ty__blk780_dn6) * assign25350_e34385) + (assign25350_e34382 * locals.var_tx__blk779_dn6))), (-(((9.0 * locals.var_ty__blk780_dn7) * assign25350_e34385) + (assign25350_e34382 * locals.var_tx__blk779_dn7))), (-(((9.0 * locals.var_ty__blk780_dn10) * assign25350_e34385) + (assign25350_e34382 * locals.var_tx__blk779_dn10))), (-(((9.0 * locals.var_ty__blk780_dn11) * assign25350_e34385) + (assign25350_e34382 * locals.var_tx__blk779_dn11))), (-(((9.0 * locals.var_ty__blk780_dn12) * assign25350_e34385) + (assign25350_e34382 * locals.var_tx__blk779_dn12))), (-(((9.0 * locals.var_ty__blk780_dn17) * assign25350_e34385) + (assign25350_e34382 * locals.var_tx__blk779_dn17))),)
    } else {
        (locals.var_ac31__blk810, locals.var_ac31__blk810_dn0, locals.var_ac31__blk810_dn2, locals.var_ac31__blk810_dn6, locals.var_ac31__blk810_dn7, locals.var_ac31__blk810_dn10, locals.var_ac31__blk810_dn11, locals.var_ac31__blk810_dn12, locals.var_ac31__blk810_dn17,)
    }
};
        locals.var_ac31__blk810 = assign25350_e34389;
        locals.var_ac31__blk810_dn0 = assign25350_e34389_d_n0;
        locals.var_ac31__blk810_dn2 = assign25350_e34389_d_n2;
        locals.var_ac31__blk810_dn6 = assign25350_e34389_d_n6;
        locals.var_ac31__blk810_dn7 = assign25350_e34389_d_n7;
        locals.var_ac31__blk810_dn10 = assign25350_e34389_d_n10;
        locals.var_ac31__blk810_dn11 = assign25350_e34389_d_n11;
        locals.var_ac31__blk810_dn12 = assign25350_e34389_d_n12;
        locals.var_ac31__blk810_dn17 = assign25350_e34389_d_n17;
        locals.var_ac31__blk810_rv = 0.0;

        let (assign25360_e34401, assign25360_e34401_d_n0, assign25360_e34401_d_n2, assign25360_e34401_d_n6, assign25360_e34401_d_n7, assign25360_e34401_d_n10, assign25360_e34401_d_n11, assign25360_e34401_d_n12, assign25360_e34401_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign25360_e34399: f64 = (locals.var_ac31__blk810 * locals.var_ac31__blk810);
        (assign25360_e34399, ((locals.var_ac31__blk810_dn0 * locals.var_ac31__blk810) + (locals.var_ac31__blk810 * locals.var_ac31__blk810_dn0)), ((locals.var_ac31__blk810_dn2 * locals.var_ac31__blk810) + (locals.var_ac31__blk810 * locals.var_ac31__blk810_dn2)), ((locals.var_ac31__blk810_dn6 * locals.var_ac31__blk810) + (locals.var_ac31__blk810 * locals.var_ac31__blk810_dn6)), ((locals.var_ac31__blk810_dn7 * locals.var_ac31__blk810) + (locals.var_ac31__blk810 * locals.var_ac31__blk810_dn7)), ((locals.var_ac31__blk810_dn10 * locals.var_ac31__blk810) + (locals.var_ac31__blk810 * locals.var_ac31__blk810_dn10)), ((locals.var_ac31__blk810_dn11 * locals.var_ac31__blk810) + (locals.var_ac31__blk810 * locals.var_ac31__blk810_dn11)), ((locals.var_ac31__blk810_dn12 * locals.var_ac31__blk810) + (locals.var_ac31__blk810 * locals.var_ac31__blk810_dn12)), ((locals.var_ac31__blk810_dn17 * locals.var_ac31__blk810) + (locals.var_ac31__blk810 * locals.var_ac31__blk810_dn17)),)
    } else {
        (locals.var_ac3__blk811, locals.var_ac3__blk811_dn0, locals.var_ac3__blk811_dn2, locals.var_ac3__blk811_dn6, locals.var_ac3__blk811_dn7, locals.var_ac3__blk811_dn10, locals.var_ac3__blk811_dn11, locals.var_ac3__blk811_dn12, locals.var_ac3__blk811_dn17,)
    }
};
        locals.var_ac3__blk811 = assign25360_e34401;
        locals.var_ac3__blk811_dn0 = assign25360_e34401_d_n0;
        locals.var_ac3__blk811_dn2 = assign25360_e34401_d_n2;
        locals.var_ac3__blk811_dn6 = assign25360_e34401_d_n6;
        locals.var_ac3__blk811_dn7 = assign25360_e34401_d_n7;
        locals.var_ac3__blk811_dn10 = assign25360_e34401_d_n10;
        locals.var_ac3__blk811_dn11 = assign25360_e34401_d_n11;
        locals.var_ac3__blk811_dn12 = assign25360_e34401_d_n12;
        locals.var_ac3__blk811_dn17 = assign25360_e34401_d_n17;
        locals.var_ac3__blk811_rv = 0.0;

        let assign25370_e34405: f64 = (locals.var_ac3__blk811 * 1e-8);
        let assign25370_e34406: f64 = if locals.var_ac4__blk808 < assign25370_e34405 { 1.0 } else { 0.0 };
        locals.var_guard856 = assign25370_e34406;
        locals.var_guard856_rv = 0.0;

        let (assign25380_e34437, assign25380_e34437_d_n0, assign25380_e34437_d_n2, assign25380_e34437_d_n6, assign25380_e34437_d_n7, assign25380_e34437_d_n10, assign25380_e34437_d_n11, assign25380_e34437_d_n12, assign25380_e34437_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard856 != 0.0)) {
        let assign25380_e34417: f64 = (-7.0);
        let assign25380_e34419: f64 = (assign25380_e34417 * 1.414213562373095);
        let assign25380_e34421: f64 = (assign25380_e34419 + locals.var_ac31__blk810);
        let assign25380_e34424: f64 = (0.5 * locals.var_ac4__blk808);
        let assign25380_e34426: f64 = (assign25380_e34424 / locals.var_ac31__blk810);
        let assign25380_e34427: f64 = (assign25380_e34421 + assign25380_e34426);
        let assign25380_e34430: f64 = (9.0 * locals.var_ty__blk780);
        let assign25380_e34433: f64 = (locals.var_tx__blk779 - 2.0);
        let assign25380_e34434: f64 = (assign25380_e34430 * assign25380_e34433);
        let assign25380_e34435: f64 = (assign25380_e34427 + assign25380_e34434);
        (assign25380_e34435, ((locals.var_ac31__blk810_dn0 + ((((0.5 * locals.var_ac4__blk808_dn0) * locals.var_ac31__blk810) - (assign25380_e34424 * locals.var_ac31__blk810_dn0)) / (locals.var_ac31__blk810 * locals.var_ac31__blk810))) + (((9.0 * locals.var_ty__blk780_dn0) * assign25380_e34433) + (assign25380_e34430 * locals.var_tx__blk779_dn0))), ((locals.var_ac31__blk810_dn2 + ((((0.5 * locals.var_ac4__blk808_dn2) * locals.var_ac31__blk810) - (assign25380_e34424 * locals.var_ac31__blk810_dn2)) / (locals.var_ac31__blk810 * locals.var_ac31__blk810))) + (((9.0 * locals.var_ty__blk780_dn2) * assign25380_e34433) + (assign25380_e34430 * locals.var_tx__blk779_dn2))), ((locals.var_ac31__blk810_dn6 + ((((0.5 * locals.var_ac4__blk808_dn6) * locals.var_ac31__blk810) - (assign25380_e34424 * locals.var_ac31__blk810_dn6)) / (locals.var_ac31__blk810 * locals.var_ac31__blk810))) + (((9.0 * locals.var_ty__blk780_dn6) * assign25380_e34433) + (assign25380_e34430 * locals.var_tx__blk779_dn6))), ((locals.var_ac31__blk810_dn7 + ((((0.5 * locals.var_ac4__blk808_dn7) * locals.var_ac31__blk810) - (assign25380_e34424 * locals.var_ac31__blk810_dn7)) / (locals.var_ac31__blk810 * locals.var_ac31__blk810))) + (((9.0 * locals.var_ty__blk780_dn7) * assign25380_e34433) + (assign25380_e34430 * locals.var_tx__blk779_dn7))), ((locals.var_ac31__blk810_dn10 + ((((0.5 * locals.var_ac4__blk808_dn10) * locals.var_ac31__blk810) - (assign25380_e34424 * locals.var_ac31__blk810_dn10)) / (locals.var_ac31__blk810 * locals.var_ac31__blk810))) + (((9.0 * locals.var_ty__blk780_dn10) * assign25380_e34433) + (assign25380_e34430 * locals.var_tx__blk779_dn10))), ((locals.var_ac31__blk810_dn11 + ((((0.5 * locals.var_ac4__blk808_dn11) * locals.var_ac31__blk810) - (assign25380_e34424 * locals.var_ac31__blk810_dn11)) / (locals.var_ac31__blk810 * locals.var_ac31__blk810))) + (((9.0 * locals.var_ty__blk780_dn11) * assign25380_e34433) + (assign25380_e34430 * locals.var_tx__blk779_dn11))), ((locals.var_ac31__blk810_dn12 + ((((0.5 * locals.var_ac4__blk808_dn12) * locals.var_ac31__blk810) - (assign25380_e34424 * locals.var_ac31__blk810_dn12)) / (locals.var_ac31__blk810 * locals.var_ac31__blk810))) + (((9.0 * locals.var_ty__blk780_dn12) * assign25380_e34433) + (assign25380_e34430 * locals.var_tx__blk779_dn12))), ((locals.var_ac31__blk810_dn17 + ((((0.5 * locals.var_ac4__blk808_dn17) * locals.var_ac31__blk810) - (assign25380_e34424 * locals.var_ac31__blk810_dn17)) / (locals.var_ac31__blk810 * locals.var_ac31__blk810))) + (((9.0 * locals.var_ty__blk780_dn17) * assign25380_e34433) + (assign25380_e34430 * locals.var_tx__blk779_dn17))),)
    } else {
        (locals.var_ac1__blk813, locals.var_ac1__blk813_dn0, locals.var_ac1__blk813_dn2, locals.var_ac1__blk813_dn6, locals.var_ac1__blk813_dn7, locals.var_ac1__blk813_dn10, locals.var_ac1__blk813_dn11, locals.var_ac1__blk813_dn12, locals.var_ac1__blk813_dn17,)
    }
};
        locals.var_ac1__blk813 = assign25380_e34437;
        locals.var_ac1__blk813_dn0 = assign25380_e34437_d_n0;
        locals.var_ac1__blk813_dn2 = assign25380_e34437_d_n2;
        locals.var_ac1__blk813_dn6 = assign25380_e34437_d_n6;
        locals.var_ac1__blk813_dn7 = assign25380_e34437_d_n7;
        locals.var_ac1__blk813_dn10 = assign25380_e34437_d_n10;
        locals.var_ac1__blk813_dn11 = assign25380_e34437_d_n11;
        locals.var_ac1__blk813_dn12 = assign25380_e34437_d_n12;
        locals.var_ac1__blk813_dn17 = assign25380_e34437_d_n17;
        locals.var_ac1__blk813_rv = 0.0;

        let (assign25390_e34453, assign25390_e34453_d_n0, assign25390_e34453_d_n2, assign25390_e34453_d_n6, assign25390_e34453_d_n7, assign25390_e34453_d_n10, assign25390_e34453_d_n11, assign25390_e34453_d_n12, assign25390_e34453_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard856 == 0.0)) {
        let assign25390_e34450: f64 = (locals.var_ac4__blk808 + locals.var_ac3__blk811);
        let assign25390_e34451: f64 = (assign25390_e34450).sqrt();
        (assign25390_e34451, ((locals.var_ac4__blk808_dn0 + locals.var_ac3__blk811_dn0) / (2.0 * assign25390_e34451)), ((locals.var_ac4__blk808_dn2 + locals.var_ac3__blk811_dn2) / (2.0 * assign25390_e34451)), ((locals.var_ac4__blk808_dn6 + locals.var_ac3__blk811_dn6) / (2.0 * assign25390_e34451)), ((locals.var_ac4__blk808_dn7 + locals.var_ac3__blk811_dn7) / (2.0 * assign25390_e34451)), ((locals.var_ac4__blk808_dn10 + locals.var_ac3__blk811_dn10) / (2.0 * assign25390_e34451)), ((locals.var_ac4__blk808_dn11 + locals.var_ac3__blk811_dn11) / (2.0 * assign25390_e34451)), ((locals.var_ac4__blk808_dn12 + locals.var_ac3__blk811_dn12) / (2.0 * assign25390_e34451)), ((locals.var_ac4__blk808_dn17 + locals.var_ac3__blk811_dn17) / (2.0 * assign25390_e34451)),)
    } else {
        (locals.var_ac2__blk812, locals.var_ac2__blk812_dn0, locals.var_ac2__blk812_dn2, locals.var_ac2__blk812_dn6, locals.var_ac2__blk812_dn7, locals.var_ac2__blk812_dn10, locals.var_ac2__blk812_dn11, locals.var_ac2__blk812_dn12, locals.var_ac2__blk812_dn17,)
    }
};
        locals.var_ac2__blk812 = assign25390_e34453;
        locals.var_ac2__blk812_dn0 = assign25390_e34453_d_n0;
        locals.var_ac2__blk812_dn2 = assign25390_e34453_d_n2;
        locals.var_ac2__blk812_dn6 = assign25390_e34453_d_n6;
        locals.var_ac2__blk812_dn7 = assign25390_e34453_d_n7;
        locals.var_ac2__blk812_dn10 = assign25390_e34453_d_n10;
        locals.var_ac2__blk812_dn11 = assign25390_e34453_d_n11;
        locals.var_ac2__blk812_dn12 = assign25390_e34453_d_n12;
        locals.var_ac2__blk812_dn17 = assign25390_e34453_d_n17;
        locals.var_ac2__blk812_rv = 0.0;

        let (assign25400_e34479, assign25400_e34479_d_n0, assign25400_e34479_d_n2, assign25400_e34479_d_n6, assign25400_e34479_d_n7, assign25400_e34479_d_n10, assign25400_e34479_d_n11, assign25400_e34479_d_n12, assign25400_e34479_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard856 == 0.0)) {
        let assign25400_e34465: f64 = (-7.0);
        let assign25400_e34467: f64 = (assign25400_e34465 * 1.414213562373095);
        let assign25400_e34469: f64 = (assign25400_e34467 + locals.var_ac2__blk812);
        let assign25400_e34472: f64 = (9.0 * locals.var_ty__blk780);
        let assign25400_e34475: f64 = (locals.var_tx__blk779 - 2.0);
        let assign25400_e34476: f64 = (assign25400_e34472 * assign25400_e34475);
        let assign25400_e34477: f64 = (assign25400_e34469 + assign25400_e34476);
        (assign25400_e34477, (locals.var_ac2__blk812_dn0 + (((9.0 * locals.var_ty__blk780_dn0) * assign25400_e34475) + (assign25400_e34472 * locals.var_tx__blk779_dn0))), (locals.var_ac2__blk812_dn2 + (((9.0 * locals.var_ty__blk780_dn2) * assign25400_e34475) + (assign25400_e34472 * locals.var_tx__blk779_dn2))), (locals.var_ac2__blk812_dn6 + (((9.0 * locals.var_ty__blk780_dn6) * assign25400_e34475) + (assign25400_e34472 * locals.var_tx__blk779_dn6))), (locals.var_ac2__blk812_dn7 + (((9.0 * locals.var_ty__blk780_dn7) * assign25400_e34475) + (assign25400_e34472 * locals.var_tx__blk779_dn7))), (locals.var_ac2__blk812_dn10 + (((9.0 * locals.var_ty__blk780_dn10) * assign25400_e34475) + (assign25400_e34472 * locals.var_tx__blk779_dn10))), (locals.var_ac2__blk812_dn11 + (((9.0 * locals.var_ty__blk780_dn11) * assign25400_e34475) + (assign25400_e34472 * locals.var_tx__blk779_dn11))), (locals.var_ac2__blk812_dn12 + (((9.0 * locals.var_ty__blk780_dn12) * assign25400_e34475) + (assign25400_e34472 * locals.var_tx__blk779_dn12))), (locals.var_ac2__blk812_dn17 + (((9.0 * locals.var_ty__blk780_dn17) * assign25400_e34475) + (assign25400_e34472 * locals.var_tx__blk779_dn17))),)
    } else {
        (locals.var_ac1__blk813, locals.var_ac1__blk813_dn0, locals.var_ac1__blk813_dn2, locals.var_ac1__blk813_dn6, locals.var_ac1__blk813_dn7, locals.var_ac1__blk813_dn10, locals.var_ac1__blk813_dn11, locals.var_ac1__blk813_dn12, locals.var_ac1__blk813_dn17,)
    }
};
        locals.var_ac1__blk813 = assign25400_e34479;
        locals.var_ac1__blk813_dn0 = assign25400_e34479_d_n0;
        locals.var_ac1__blk813_dn2 = assign25400_e34479_d_n2;
        locals.var_ac1__blk813_dn6 = assign25400_e34479_d_n6;
        locals.var_ac1__blk813_dn7 = assign25400_e34479_d_n7;
        locals.var_ac1__blk813_dn10 = assign25400_e34479_d_n10;
        locals.var_ac1__blk813_dn11 = assign25400_e34479_d_n11;
        locals.var_ac1__blk813_dn12 = assign25400_e34479_d_n12;
        locals.var_ac1__blk813_dn17 = assign25400_e34479_d_n17;
        locals.var_ac1__blk813_rv = 0.0;

        let (assign25410_e34491, assign25410_e34491_d_n0, assign25410_e34491_d_n2, assign25410_e34491_d_n6, assign25410_e34491_d_n7, assign25410_e34491_d_n10, assign25410_e34491_d_n11, assign25410_e34491_d_n12, assign25410_e34491_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign25410_e34489: f64 = (locals.var_ac1__blk813).powf(0.3333333333333333);
        (assign25410_e34489, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk813).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk813_dn0)) } } else { (assign25410_e34489 * (0.3333333333333333 * (locals.var_ac1__blk813_dn0 / locals.var_ac1__blk813))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk813).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk813_dn2)) } } else { (assign25410_e34489 * (0.3333333333333333 * (locals.var_ac1__blk813_dn2 / locals.var_ac1__blk813))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk813).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk813_dn6)) } } else { (assign25410_e34489 * (0.3333333333333333 * (locals.var_ac1__blk813_dn6 / locals.var_ac1__blk813))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk813).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk813_dn7)) } } else { (assign25410_e34489 * (0.3333333333333333 * (locals.var_ac1__blk813_dn7 / locals.var_ac1__blk813))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk813).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk813_dn10)) } } else { (assign25410_e34489 * (0.3333333333333333 * (locals.var_ac1__blk813_dn10 / locals.var_ac1__blk813))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk813).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk813_dn11)) } } else { (assign25410_e34489 * (0.3333333333333333 * (locals.var_ac1__blk813_dn11 / locals.var_ac1__blk813))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk813).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk813_dn12)) } } else { (assign25410_e34489 * (0.3333333333333333 * (locals.var_ac1__blk813_dn12 / locals.var_ac1__blk813))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk813).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk813_dn17)) } } else { (assign25410_e34489 * (0.3333333333333333 * (locals.var_ac1__blk813_dn17 / locals.var_ac1__blk813))) },)
    } else {
        (locals.var_acd__blk814, locals.var_acd__blk814_dn0, locals.var_acd__blk814_dn2, locals.var_acd__blk814_dn6, locals.var_acd__blk814_dn7, locals.var_acd__blk814_dn10, locals.var_acd__blk814_dn11, locals.var_acd__blk814_dn12, locals.var_acd__blk814_dn17,)
    }
};
        locals.var_acd__blk814 = assign25410_e34491;
        locals.var_acd__blk814_dn0 = assign25410_e34491_d_n0;
        locals.var_acd__blk814_dn2 = assign25410_e34491_d_n2;
        locals.var_acd__blk814_dn6 = assign25410_e34491_d_n6;
        locals.var_acd__blk814_dn7 = assign25410_e34491_d_n7;
        locals.var_acd__blk814_dn10 = assign25410_e34491_d_n10;
        locals.var_acd__blk814_dn11 = assign25410_e34491_d_n11;
        locals.var_acd__blk814_dn12 = assign25410_e34491_d_n12;
        locals.var_acd__blk814_dn17 = assign25410_e34491_d_n17;
        locals.var_acd__blk814_rv = 0.0;

        let (assign25420_e34518, assign25420_e34518_d_n0, assign25420_e34518_d_n2, assign25420_e34518_d_n6, assign25420_e34518_d_n7, assign25420_e34518_d_n10, assign25420_e34518_d_n11, assign25420_e34518_d_n12, assign25420_e34518_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign25420_e34500: f64 = (-4.0);
        let assign25420_e34502: f64 = (assign25420_e34500 * 1.414213562373095);
        let assign25420_e34505: f64 = (12.0 * locals.var_ty__blk780);
        let assign25420_e34506: f64 = (assign25420_e34502 - assign25420_e34505);
        let assign25420_e34509: f64 = (2.0 * locals.var_acd__blk814);
        let assign25420_e34510: f64 = (assign25420_e34506 + assign25420_e34509);
        let assign25420_e34513: f64 = (1.414213562373095 * locals.var_acd__blk814);
        let assign25420_e34515: f64 = (assign25420_e34513 * locals.var_acd__blk814);
        let assign25420_e34516: f64 = (assign25420_e34510 + assign25420_e34515);
        (assign25420_e34516, (((-(12.0 * locals.var_ty__blk780_dn0)) + (2.0 * locals.var_acd__blk814_dn0)) + (((1.414213562373095 * locals.var_acd__blk814_dn0) * locals.var_acd__blk814) + (assign25420_e34513 * locals.var_acd__blk814_dn0))), (((-(12.0 * locals.var_ty__blk780_dn2)) + (2.0 * locals.var_acd__blk814_dn2)) + (((1.414213562373095 * locals.var_acd__blk814_dn2) * locals.var_acd__blk814) + (assign25420_e34513 * locals.var_acd__blk814_dn2))), (((-(12.0 * locals.var_ty__blk780_dn6)) + (2.0 * locals.var_acd__blk814_dn6)) + (((1.414213562373095 * locals.var_acd__blk814_dn6) * locals.var_acd__blk814) + (assign25420_e34513 * locals.var_acd__blk814_dn6))), (((-(12.0 * locals.var_ty__blk780_dn7)) + (2.0 * locals.var_acd__blk814_dn7)) + (((1.414213562373095 * locals.var_acd__blk814_dn7) * locals.var_acd__blk814) + (assign25420_e34513 * locals.var_acd__blk814_dn7))), (((-(12.0 * locals.var_ty__blk780_dn10)) + (2.0 * locals.var_acd__blk814_dn10)) + (((1.414213562373095 * locals.var_acd__blk814_dn10) * locals.var_acd__blk814) + (assign25420_e34513 * locals.var_acd__blk814_dn10))), (((-(12.0 * locals.var_ty__blk780_dn11)) + (2.0 * locals.var_acd__blk814_dn11)) + (((1.414213562373095 * locals.var_acd__blk814_dn11) * locals.var_acd__blk814) + (assign25420_e34513 * locals.var_acd__blk814_dn11))), (((-(12.0 * locals.var_ty__blk780_dn12)) + (2.0 * locals.var_acd__blk814_dn12)) + (((1.414213562373095 * locals.var_acd__blk814_dn12) * locals.var_acd__blk814) + (assign25420_e34513 * locals.var_acd__blk814_dn12))), (((-(12.0 * locals.var_ty__blk780_dn17)) + (2.0 * locals.var_acd__blk814_dn17)) + (((1.414213562373095 * locals.var_acd__blk814_dn17) * locals.var_acd__blk814) + (assign25420_e34513 * locals.var_acd__blk814_dn17))),)
    } else {
        (locals.var_acn__blk815, locals.var_acn__blk815_dn0, locals.var_acn__blk815_dn2, locals.var_acn__blk815_dn6, locals.var_acn__blk815_dn7, locals.var_acn__blk815_dn10, locals.var_acn__blk815_dn11, locals.var_acn__blk815_dn12, locals.var_acn__blk815_dn17,)
    }
};
        locals.var_acn__blk815 = assign25420_e34518;
        locals.var_acn__blk815_dn0 = assign25420_e34518_d_n0;
        locals.var_acn__blk815_dn2 = assign25420_e34518_d_n2;
        locals.var_acn__blk815_dn6 = assign25420_e34518_d_n6;
        locals.var_acn__blk815_dn7 = assign25420_e34518_d_n7;
        locals.var_acn__blk815_dn10 = assign25420_e34518_d_n10;
        locals.var_acn__blk815_dn11 = assign25420_e34518_d_n11;
        locals.var_acn__blk815_dn12 = assign25420_e34518_d_n12;
        locals.var_acn__blk815_dn17 = assign25420_e34518_d_n17;
        locals.var_acn__blk815_rv = 0.0;

        let (assign25430_e34530, assign25430_e34530_d_n0, assign25430_e34530_d_n2, assign25430_e34530_d_n6, assign25430_e34530_d_n7, assign25430_e34530_d_n10, assign25430_e34530_d_n11, assign25430_e34530_d_n12, assign25430_e34530_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign25430_e34528: f64 = (locals.var_acn__blk815 / locals.var_acd__blk814);
        (assign25430_e34528, (((locals.var_acn__blk815_dn0 * locals.var_acd__blk814) - (locals.var_acn__blk815 * locals.var_acd__blk814_dn0)) / (locals.var_acd__blk814 * locals.var_acd__blk814)), (((locals.var_acn__blk815_dn2 * locals.var_acd__blk814) - (locals.var_acn__blk815 * locals.var_acd__blk814_dn2)) / (locals.var_acd__blk814 * locals.var_acd__blk814)), (((locals.var_acn__blk815_dn6 * locals.var_acd__blk814) - (locals.var_acn__blk815 * locals.var_acd__blk814_dn6)) / (locals.var_acd__blk814 * locals.var_acd__blk814)), (((locals.var_acn__blk815_dn7 * locals.var_acd__blk814) - (locals.var_acn__blk815 * locals.var_acd__blk814_dn7)) / (locals.var_acd__blk814 * locals.var_acd__blk814)), (((locals.var_acn__blk815_dn10 * locals.var_acd__blk814) - (locals.var_acn__blk815 * locals.var_acd__blk814_dn10)) / (locals.var_acd__blk814 * locals.var_acd__blk814)), (((locals.var_acn__blk815_dn11 * locals.var_acd__blk814) - (locals.var_acn__blk815 * locals.var_acd__blk814_dn11)) / (locals.var_acd__blk814 * locals.var_acd__blk814)), (((locals.var_acn__blk815_dn12 * locals.var_acd__blk814) - (locals.var_acn__blk815 * locals.var_acd__blk814_dn12)) / (locals.var_acd__blk814 * locals.var_acd__blk814)), (((locals.var_acn__blk815_dn17 * locals.var_acd__blk814) - (locals.var_acn__blk815 * locals.var_acd__blk814_dn17)) / (locals.var_acd__blk814 * locals.var_acd__blk814)),)
    } else {
        (locals.var_chi__blk816, locals.var_chi__blk816_dn0, locals.var_chi__blk816_dn2, locals.var_chi__blk816_dn6, locals.var_chi__blk816_dn7, locals.var_chi__blk816_dn10, locals.var_chi__blk816_dn11, locals.var_chi__blk816_dn12, locals.var_chi__blk816_dn17,)
    }
};
        locals.var_chi__blk816 = assign25430_e34530;
        locals.var_chi__blk816_dn0 = assign25430_e34530_d_n0;
        locals.var_chi__blk816_dn2 = assign25430_e34530_d_n2;
        locals.var_chi__blk816_dn6 = assign25430_e34530_d_n6;
        locals.var_chi__blk816_dn7 = assign25430_e34530_d_n7;
        locals.var_chi__blk816_dn10 = assign25430_e34530_d_n10;
        locals.var_chi__blk816_dn11 = assign25430_e34530_d_n11;
        locals.var_chi__blk816_dn12 = assign25430_e34530_d_n12;
        locals.var_chi__blk816_dn17 = assign25430_e34530_d_n17;
        locals.var_chi__blk816_rv = 0.0;

        let (assign25440_e34544, assign25440_e34544_d_n0, assign25440_e34544_d_n2, assign25440_e34544_d_n6, assign25440_e34544_d_n7, assign25440_e34544_d_n10, assign25440_e34544_d_n11, assign25440_e34544_d_n12, assign25440_e34544_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign25440_e34540: f64 = (locals.var_chi__blk816 * locals.var_beta_inv);
        let assign25440_e34542: f64 = (assign25440_e34540 - locals.var_vxbgmtcl);
        (assign25440_e34542, ((locals.var_chi__blk816_dn0 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn0), ((locals.var_chi__blk816_dn2 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn2), ((locals.var_chi__blk816_dn6 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn6), ((locals.var_chi__blk816_dn7 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn7), (((locals.var_chi__blk816_dn10 * locals.var_beta_inv) + (locals.var_chi__blk816 * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), ((locals.var_chi__blk816_dn11 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn11), ((locals.var_chi__blk816_dn12 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn12), ((locals.var_chi__blk816_dn17 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_psa__blk817, locals.var_psa__blk817_dn0, locals.var_psa__blk817_dn2, locals.var_psa__blk817_dn6, locals.var_psa__blk817_dn7, locals.var_psa__blk817_dn10, locals.var_psa__blk817_dn11, locals.var_psa__blk817_dn12, locals.var_psa__blk817_dn17,)
    }
};
        locals.var_psa__blk817 = assign25440_e34544;
        locals.var_psa__blk817_dn0 = assign25440_e34544_d_n0;
        locals.var_psa__blk817_dn2 = assign25440_e34544_d_n2;
        locals.var_psa__blk817_dn6 = assign25440_e34544_d_n6;
        locals.var_psa__blk817_dn7 = assign25440_e34544_d_n7;
        locals.var_psa__blk817_dn10 = assign25440_e34544_d_n10;
        locals.var_psa__blk817_dn11 = assign25440_e34544_d_n11;
        locals.var_psa__blk817_dn12 = assign25440_e34544_d_n12;
        locals.var_psa__blk817_dn17 = assign25440_e34544_d_n17;
        locals.var_psa__blk817_rv = 0.0;

        let (assign25450_e34556, assign25450_e34556_d_n0, assign25450_e34556_d_n2, assign25450_e34556_d_n6, assign25450_e34556_d_n7, assign25450_e34556_d_n10, assign25450_e34556_d_n11, assign25450_e34556_d_n12, assign25450_e34556_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign25450_e34554: f64 = (locals.var_psa__blk817 + locals.var_vxbgmtcl);
        (assign25450_e34554, (locals.var_psa__blk817_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_psa__blk817_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_psa__blk817_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_psa__blk817_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_psa__blk817_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_psa__blk817_dn11 + locals.var_vxbgmtcl_dn11), (locals.var_psa__blk817_dn12 + locals.var_vxbgmtcl_dn12), (locals.var_psa__blk817_dn17 + locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_t1__blk773, locals.var_t1__blk773_dn0, locals.var_t1__blk773_dn2, locals.var_t1__blk773_dn6, locals.var_t1__blk773_dn7, locals.var_t1__blk773_dn10, locals.var_t1__blk773_dn11, locals.var_t1__blk773_dn12, locals.var_t1__blk773_dn17,)
    }
};
        locals.var_t1__blk773 = assign25450_e34556;
        locals.var_t1__blk773_dn0 = assign25450_e34556_d_n0;
        locals.var_t1__blk773_dn2 = assign25450_e34556_d_n2;
        locals.var_t1__blk773_dn6 = assign25450_e34556_d_n6;
        locals.var_t1__blk773_dn7 = assign25450_e34556_d_n7;
        locals.var_t1__blk773_dn10 = assign25450_e34556_d_n10;
        locals.var_t1__blk773_dn11 = assign25450_e34556_d_n11;
        locals.var_t1__blk773_dn12 = assign25450_e34556_d_n12;
        locals.var_t1__blk773_dn17 = assign25450_e34556_d_n17;
        locals.var_t1__blk773_rv = 0.0;

        let (assign25460_e34568, assign25460_e34568_d_n0, assign25460_e34568_d_n2, assign25460_e34568_d_n6, assign25460_e34568_d_n7, assign25460_e34568_d_n10, assign25460_e34568_d_n11, assign25460_e34568_d_n12, assign25460_e34568_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign25460_e34566: f64 = (locals.var_t1__blk773 / locals.var_ps0_min__blk809);
        (assign25460_e34566, (((locals.var_t1__blk773_dn0 * locals.var_ps0_min__blk809) - (locals.var_t1__blk773 * locals.var_ps0_min__blk809_dn0)) / (locals.var_ps0_min__blk809 * locals.var_ps0_min__blk809)), (((locals.var_t1__blk773_dn2 * locals.var_ps0_min__blk809) - (locals.var_t1__blk773 * locals.var_ps0_min__blk809_dn2)) / (locals.var_ps0_min__blk809 * locals.var_ps0_min__blk809)), (((locals.var_t1__blk773_dn6 * locals.var_ps0_min__blk809) - (locals.var_t1__blk773 * locals.var_ps0_min__blk809_dn6)) / (locals.var_ps0_min__blk809 * locals.var_ps0_min__blk809)), (((locals.var_t1__blk773_dn7 * locals.var_ps0_min__blk809) - (locals.var_t1__blk773 * locals.var_ps0_min__blk809_dn7)) / (locals.var_ps0_min__blk809 * locals.var_ps0_min__blk809)), (((locals.var_t1__blk773_dn10 * locals.var_ps0_min__blk809) - (locals.var_t1__blk773 * locals.var_ps0_min__blk809_dn10)) / (locals.var_ps0_min__blk809 * locals.var_ps0_min__blk809)), (((locals.var_t1__blk773_dn11 * locals.var_ps0_min__blk809) - (locals.var_t1__blk773 * locals.var_ps0_min__blk809_dn11)) / (locals.var_ps0_min__blk809 * locals.var_ps0_min__blk809)), (((locals.var_t1__blk773_dn12 * locals.var_ps0_min__blk809) - (locals.var_t1__blk773 * locals.var_ps0_min__blk809_dn12)) / (locals.var_ps0_min__blk809 * locals.var_ps0_min__blk809)), (((locals.var_t1__blk773_dn17 * locals.var_ps0_min__blk809) - (locals.var_t1__blk773 * locals.var_ps0_min__blk809_dn17)) / (locals.var_ps0_min__blk809 * locals.var_ps0_min__blk809)),)
    } else {
        (locals.var_t2__blk774, locals.var_t2__blk774_dn0, locals.var_t2__blk774_dn2, locals.var_t2__blk774_dn6, locals.var_t2__blk774_dn7, locals.var_t2__blk774_dn10, locals.var_t2__blk774_dn11, locals.var_t2__blk774_dn12, locals.var_t2__blk774_dn17,)
    }
};
        locals.var_t2__blk774 = assign25460_e34568;
        locals.var_t2__blk774_dn0 = assign25460_e34568_d_n0;
        locals.var_t2__blk774_dn2 = assign25460_e34568_d_n2;
        locals.var_t2__blk774_dn6 = assign25460_e34568_d_n6;
        locals.var_t2__blk774_dn7 = assign25460_e34568_d_n7;
        locals.var_t2__blk774_dn10 = assign25460_e34568_d_n10;
        locals.var_t2__blk774_dn11 = assign25460_e34568_d_n11;
        locals.var_t2__blk774_dn12 = assign25460_e34568_d_n12;
        locals.var_t2__blk774_dn17 = assign25460_e34568_d_n17;
        locals.var_t2__blk774_rv = 0.0;

        let (assign25470_e34583, assign25470_e34583_d_n0, assign25470_e34583_d_n2, assign25470_e34583_d_n6, assign25470_e34583_d_n7, assign25470_e34583_d_n10, assign25470_e34583_d_n11, assign25470_e34583_d_n12, assign25470_e34583_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign25470_e34579: f64 = (locals.var_t2__blk774 * locals.var_t2__blk774);
        let assign25470_e34580: f64 = (1.0 + assign25470_e34579);
        let assign25470_e34581: f64 = (assign25470_e34580).sqrt();
        (assign25470_e34581, (((locals.var_t2__blk774_dn0 * locals.var_t2__blk774) + (locals.var_t2__blk774 * locals.var_t2__blk774_dn0)) / (2.0 * assign25470_e34581)), (((locals.var_t2__blk774_dn2 * locals.var_t2__blk774) + (locals.var_t2__blk774 * locals.var_t2__blk774_dn2)) / (2.0 * assign25470_e34581)), (((locals.var_t2__blk774_dn6 * locals.var_t2__blk774) + (locals.var_t2__blk774 * locals.var_t2__blk774_dn6)) / (2.0 * assign25470_e34581)), (((locals.var_t2__blk774_dn7 * locals.var_t2__blk774) + (locals.var_t2__blk774 * locals.var_t2__blk774_dn7)) / (2.0 * assign25470_e34581)), (((locals.var_t2__blk774_dn10 * locals.var_t2__blk774) + (locals.var_t2__blk774 * locals.var_t2__blk774_dn10)) / (2.0 * assign25470_e34581)), (((locals.var_t2__blk774_dn11 * locals.var_t2__blk774) + (locals.var_t2__blk774 * locals.var_t2__blk774_dn11)) / (2.0 * assign25470_e34581)), (((locals.var_t2__blk774_dn12 * locals.var_t2__blk774) + (locals.var_t2__blk774 * locals.var_t2__blk774_dn12)) / (2.0 * assign25470_e34581)), (((locals.var_t2__blk774_dn17 * locals.var_t2__blk774) + (locals.var_t2__blk774 * locals.var_t2__blk774_dn17)) / (2.0 * assign25470_e34581)),)
    } else {
        (locals.var_t3__blk775, locals.var_t3__blk775_dn0, locals.var_t3__blk775_dn2, locals.var_t3__blk775_dn6, locals.var_t3__blk775_dn7, locals.var_t3__blk775_dn10, locals.var_t3__blk775_dn11, locals.var_t3__blk775_dn12, locals.var_t3__blk775_dn17,)
    }
};
        locals.var_t3__blk775 = assign25470_e34583;
        locals.var_t3__blk775_dn0 = assign25470_e34583_d_n0;
        locals.var_t3__blk775_dn2 = assign25470_e34583_d_n2;
        locals.var_t3__blk775_dn6 = assign25470_e34583_d_n6;
        locals.var_t3__blk775_dn7 = assign25470_e34583_d_n7;
        locals.var_t3__blk775_dn10 = assign25470_e34583_d_n10;
        locals.var_t3__blk775_dn11 = assign25470_e34583_d_n11;
        locals.var_t3__blk775_dn12 = assign25470_e34583_d_n12;
        locals.var_t3__blk775_dn17 = assign25470_e34583_d_n17;
        locals.var_t3__blk775_rv = 0.0;

        let (assign25480_e34597, assign25480_e34597_d_n0, assign25480_e34597_d_n2, assign25480_e34597_d_n6, assign25480_e34597_d_n7, assign25480_e34597_d_n10, assign25480_e34597_d_n11, assign25480_e34597_d_n12, assign25480_e34597_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign25480_e34593: f64 = (locals.var_t1__blk773 / locals.var_t3__blk775);
        let assign25480_e34595: f64 = (assign25480_e34593 - locals.var_vxbgmtcl);
        (assign25480_e34595, ((((locals.var_t1__blk773_dn0 * locals.var_t3__blk775) - (locals.var_t1__blk773 * locals.var_t3__blk775_dn0)) / (locals.var_t3__blk775 * locals.var_t3__blk775)) - locals.var_vxbgmtcl_dn0), ((((locals.var_t1__blk773_dn2 * locals.var_t3__blk775) - (locals.var_t1__blk773 * locals.var_t3__blk775_dn2)) / (locals.var_t3__blk775 * locals.var_t3__blk775)) - locals.var_vxbgmtcl_dn2), ((((locals.var_t1__blk773_dn6 * locals.var_t3__blk775) - (locals.var_t1__blk773 * locals.var_t3__blk775_dn6)) / (locals.var_t3__blk775 * locals.var_t3__blk775)) - locals.var_vxbgmtcl_dn6), ((((locals.var_t1__blk773_dn7 * locals.var_t3__blk775) - (locals.var_t1__blk773 * locals.var_t3__blk775_dn7)) / (locals.var_t3__blk775 * locals.var_t3__blk775)) - locals.var_vxbgmtcl_dn7), ((((locals.var_t1__blk773_dn10 * locals.var_t3__blk775) - (locals.var_t1__blk773 * locals.var_t3__blk775_dn10)) / (locals.var_t3__blk775 * locals.var_t3__blk775)) - locals.var_vxbgmtcl_dn10), ((((locals.var_t1__blk773_dn11 * locals.var_t3__blk775) - (locals.var_t1__blk773 * locals.var_t3__blk775_dn11)) / (locals.var_t3__blk775 * locals.var_t3__blk775)) - locals.var_vxbgmtcl_dn11), ((((locals.var_t1__blk773_dn12 * locals.var_t3__blk775) - (locals.var_t1__blk773 * locals.var_t3__blk775_dn12)) / (locals.var_t3__blk775 * locals.var_t3__blk775)) - locals.var_vxbgmtcl_dn12), ((((locals.var_t1__blk773_dn17 * locals.var_t3__blk775) - (locals.var_t1__blk773 * locals.var_t3__blk775_dn17)) / (locals.var_t3__blk775 * locals.var_t3__blk775)) - locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn12, locals.var_ps0ld_dn17,)
    }
};
        locals.var_ps0ld = assign25480_e34597;
        locals.var_ps0ld_dn0 = assign25480_e34597_d_n0;
        locals.var_ps0ld_dn2 = assign25480_e34597_d_n2;
        locals.var_ps0ld_dn6 = assign25480_e34597_d_n6;
        locals.var_ps0ld_dn7 = assign25480_e34597_d_n7;
        locals.var_ps0ld_dn10 = assign25480_e34597_d_n10;
        locals.var_ps0ld_dn11 = assign25480_e34597_d_n11;
        locals.var_ps0ld_dn12 = assign25480_e34597_d_n12;
        locals.var_ps0ld_dn17 = assign25480_e34597_d_n17;
        locals.var_ps0ld_rv = 0.0;

        let (assign25490_e34609, assign25490_e34609_d_n0, assign25490_e34609_d_n2, assign25490_e34609_d_n6, assign25490_e34609_d_n7, assign25490_e34609_d_n10, assign25490_e34609_d_n11, assign25490_e34609_d_n12, assign25490_e34609_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign25490_e34607: f64 = (locals.var_vgpld - locals.var_ps0ld);
        (assign25490_e34607, (locals.var_vgpld_dn0 - locals.var_ps0ld_dn0), (locals.var_vgpld_dn2 - locals.var_ps0ld_dn2), (locals.var_vgpld_dn6 - locals.var_ps0ld_dn6), (locals.var_vgpld_dn7 - locals.var_ps0ld_dn7), (locals.var_vgpld_dn10 - locals.var_ps0ld_dn10), (locals.var_vgpld_dn11 - locals.var_ps0ld_dn11), (locals.var_vgpld_dn12 - locals.var_ps0ld_dn12), (locals.var_vgpld_dn17 - locals.var_ps0ld_dn17),)
    } else {
        (locals.var_t2__blk774, locals.var_t2__blk774_dn0, locals.var_t2__blk774_dn2, locals.var_t2__blk774_dn6, locals.var_t2__blk774_dn7, locals.var_t2__blk774_dn10, locals.var_t2__blk774_dn11, locals.var_t2__blk774_dn12, locals.var_t2__blk774_dn17,)
    }
};
        locals.var_t2__blk774 = assign25490_e34609;
        locals.var_t2__blk774_dn0 = assign25490_e34609_d_n0;
        locals.var_t2__blk774_dn2 = assign25490_e34609_d_n2;
        locals.var_t2__blk774_dn6 = assign25490_e34609_d_n6;
        locals.var_t2__blk774_dn7 = assign25490_e34609_d_n7;
        locals.var_t2__blk774_dn10 = assign25490_e34609_d_n10;
        locals.var_t2__blk774_dn11 = assign25490_e34609_d_n11;
        locals.var_t2__blk774_dn12 = assign25490_e34609_d_n12;
        locals.var_t2__blk774_dn17 = assign25490_e34609_d_n17;
        locals.var_t2__blk774_rv = 0.0;

        let (assign25500_e34621, assign25500_e34621_d_n0, assign25500_e34621_d_n2, assign25500_e34621_d_n6, assign25500_e34621_d_n7, assign25500_e34621_d_n10, assign25500_e34621_d_n11, assign25500_e34621_d_n12, assign25500_e34621_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign25500_e34619: f64 = (locals.var_cox0 * locals.var_t2__blk774);
        (assign25500_e34619, (locals.var_cox0 * locals.var_t2__blk774_dn0), (locals.var_cox0 * locals.var_t2__blk774_dn2), (locals.var_cox0 * locals.var_t2__blk774_dn6), (locals.var_cox0 * locals.var_t2__blk774_dn7), (locals.var_cox0 * locals.var_t2__blk774_dn10), (locals.var_cox0 * locals.var_t2__blk774_dn11), (locals.var_cox0 * locals.var_t2__blk774_dn12), (locals.var_cox0 * locals.var_t2__blk774_dn17),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17,)
    }
};
        locals.var_qsuld = assign25500_e34621;
        locals.var_qsuld_dn0 = assign25500_e34621_d_n0;
        locals.var_qsuld_dn2 = assign25500_e34621_d_n2;
        locals.var_qsuld_dn6 = assign25500_e34621_d_n6;
        locals.var_qsuld_dn7 = assign25500_e34621_d_n7;
        locals.var_qsuld_dn10 = assign25500_e34621_d_n10;
        locals.var_qsuld_dn11 = assign25500_e34621_d_n11;
        locals.var_qsuld_dn12 = assign25500_e34621_d_n12;
        locals.var_qsuld_dn17 = assign25500_e34621_d_n17;
        locals.var_qsuld_rv = 0.0;

        let (assign25510_e34631, assign25510_e34631_d_n0, assign25510_e34631_d_n2, assign25510_e34631_d_n6, assign25510_e34631_d_n7, assign25510_e34631_d_n10, assign25510_e34631_d_n11, assign25510_e34631_d_n12, assign25510_e34631_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 != 0.0)) {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17,)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn12, locals.var_qbuld_dn17,)
    }
};
        locals.var_qbuld = assign25510_e34631;
        locals.var_qbuld_dn0 = assign25510_e34631_d_n0;
        locals.var_qbuld_dn2 = assign25510_e34631_d_n2;
        locals.var_qbuld_dn6 = assign25510_e34631_d_n6;
        locals.var_qbuld_dn7 = assign25510_e34631_d_n7;
        locals.var_qbuld_dn10 = assign25510_e34631_d_n10;
        locals.var_qbuld_dn11 = assign25510_e34631_d_n11;
        locals.var_qbuld_dn12 = assign25510_e34631_d_n12;
        locals.var_qbuld_dn17 = assign25510_e34631_d_n17;
        locals.var_qbuld_rv = 0.0;

        let (assign25530_e34653, assign25530_e34653_d_n0, assign25530_e34653_d_n2, assign25530_e34653_d_n6, assign25530_e34653_d_n7, assign25530_e34653_d_n10, assign25530_e34653_d_n11, assign25530_e34653_d_n12, assign25530_e34653_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) {
        (3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi__blk816, locals.var_chi__blk816_dn0, locals.var_chi__blk816_dn2, locals.var_chi__blk816_dn6, locals.var_chi__blk816_dn7, locals.var_chi__blk816_dn10, locals.var_chi__blk816_dn11, locals.var_chi__blk816_dn12, locals.var_chi__blk816_dn17,)
    }
};
        locals.var_chi__blk816 = assign25530_e34653;
        locals.var_chi__blk816_dn0 = assign25530_e34653_d_n0;
        locals.var_chi__blk816_dn2 = assign25530_e34653_d_n2;
        locals.var_chi__blk816_dn6 = assign25530_e34653_d_n6;
        locals.var_chi__blk816_dn7 = assign25530_e34653_d_n7;
        locals.var_chi__blk816_dn10 = assign25530_e34653_d_n10;
        locals.var_chi__blk816_dn11 = assign25530_e34653_d_n11;
        locals.var_chi__blk816_dn12 = assign25530_e34653_d_n12;
        locals.var_chi__blk816_dn17 = assign25530_e34653_d_n17;
        locals.var_chi__blk816_rv = 0.0;

        let (assign25540_e34668, assign25540_e34668_d_n0, assign25540_e34668_d_n2, assign25540_e34668_d_n6, assign25540_e34668_d_n7, assign25540_e34668_d_n10, assign25540_e34668_d_n11, assign25540_e34668_d_n12, assign25540_e34668_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) {
        let assign25540_e34664: f64 = (locals.var_chi__blk816 / locals.var_beta);
        let assign25540_e34666: f64 = (assign25540_e34664 - locals.var_vxbgmtcl);
        (assign25540_e34666, ((locals.var_chi__blk816_dn0 / locals.var_beta) - locals.var_vxbgmtcl_dn0), ((locals.var_chi__blk816_dn2 / locals.var_beta) - locals.var_vxbgmtcl_dn2), ((locals.var_chi__blk816_dn6 / locals.var_beta) - locals.var_vxbgmtcl_dn6), ((locals.var_chi__blk816_dn7 / locals.var_beta) - locals.var_vxbgmtcl_dn7), ((((locals.var_chi__blk816_dn10 * locals.var_beta) - (locals.var_chi__blk816 * locals.var_beta_dn10)) / (locals.var_beta * locals.var_beta)) - locals.var_vxbgmtcl_dn10), ((locals.var_chi__blk816_dn11 / locals.var_beta) - locals.var_vxbgmtcl_dn11), ((locals.var_chi__blk816_dn12 / locals.var_beta) - locals.var_vxbgmtcl_dn12), ((locals.var_chi__blk816_dn17 / locals.var_beta) - locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_ps0_inia__blk819, locals.var_ps0_inia__blk819_dn0, locals.var_ps0_inia__blk819_dn2, locals.var_ps0_inia__blk819_dn6, locals.var_ps0_inia__blk819_dn7, locals.var_ps0_inia__blk819_dn10, locals.var_ps0_inia__blk819_dn11, locals.var_ps0_inia__blk819_dn12, locals.var_ps0_inia__blk819_dn17,)
    }
};
        locals.var_ps0_inia__blk819 = assign25540_e34668;
        locals.var_ps0_inia__blk819_dn0 = assign25540_e34668_d_n0;
        locals.var_ps0_inia__blk819_dn2 = assign25540_e34668_d_n2;
        locals.var_ps0_inia__blk819_dn6 = assign25540_e34668_d_n6;
        locals.var_ps0_inia__blk819_dn7 = assign25540_e34668_d_n7;
        locals.var_ps0_inia__blk819_dn10 = assign25540_e34668_d_n10;
        locals.var_ps0_inia__blk819_dn11 = assign25540_e34668_d_n11;
        locals.var_ps0_inia__blk819_dn12 = assign25540_e34668_d_n12;
        locals.var_ps0_inia__blk819_dn17 = assign25540_e34668_d_n17;
        locals.var_ps0_inia__blk819_rv = 0.0;

        let (assign25550_e34681, assign25550_e34681_d_n0, assign25550_e34681_d_n2, assign25550_e34681_d_n6, assign25550_e34681_d_n7, assign25550_e34681_d_n10, assign25550_e34681_d_n11, assign25550_e34681_d_n12, assign25550_e34681_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) {
        let assign25550_e34678: f64 = (-locals.var_chi__blk816);
        let assign25550_e34679: f64 = (assign25550_e34678).exp();
        (assign25550_e34679, (assign25550_e34679 * (-locals.var_chi__blk816_dn0)), (assign25550_e34679 * (-locals.var_chi__blk816_dn2)), (assign25550_e34679 * (-locals.var_chi__blk816_dn6)), (assign25550_e34679 * (-locals.var_chi__blk816_dn7)), (assign25550_e34679 * (-locals.var_chi__blk816_dn10)), (assign25550_e34679 * (-locals.var_chi__blk816_dn11)), (assign25550_e34679 * (-locals.var_chi__blk816_dn12)), (assign25550_e34679 * (-locals.var_chi__blk816_dn17)),)
    } else {
        (locals.var_ty__blk780, locals.var_ty__blk780_dn0, locals.var_ty__blk780_dn2, locals.var_ty__blk780_dn6, locals.var_ty__blk780_dn7, locals.var_ty__blk780_dn10, locals.var_ty__blk780_dn11, locals.var_ty__blk780_dn12, locals.var_ty__blk780_dn17,)
    }
};
        locals.var_ty__blk780 = assign25550_e34681;
        locals.var_ty__blk780_dn0 = assign25550_e34681_d_n0;
        locals.var_ty__blk780_dn2 = assign25550_e34681_d_n2;
        locals.var_ty__blk780_dn6 = assign25550_e34681_d_n6;
        locals.var_ty__blk780_dn7 = assign25550_e34681_d_n7;
        locals.var_ty__blk780_dn10 = assign25550_e34681_d_n10;
        locals.var_ty__blk780_dn11 = assign25550_e34681_d_n11;
        locals.var_ty__blk780_dn12 = assign25550_e34681_d_n12;
        locals.var_ty__blk780_dn17 = assign25550_e34681_d_n17;
        locals.var_ty__blk780_rv = 0.0;

        let (assign25560_e34708, assign25560_e34708_d_n0, assign25560_e34708_d_n2, assign25560_e34708_d_n6, assign25560_e34708_d_n7, assign25560_e34708_d_n10, assign25560_e34708_d_n11, assign25560_e34708_d_n12, assign25560_e34708_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) {
        let assign25560_e34695: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign25560_e34696: f64 = (locals.var_beta * assign25560_e34695);
        let assign25560_e34698: f64 = (assign25560_e34696 - 1.0);
        let assign25560_e34700: f64 = (assign25560_e34698 + locals.var_ty__blk780);
        let assign25560_e34701: f64 = (4.0 * assign25560_e34700);
        let assign25560_e34704: f64 = (locals.var_fac1p2__blk803 * locals.var_beta2);
        let assign25560_e34705: f64 = (assign25560_e34701 / assign25560_e34704);
        let assign25560_e34706: f64 = (1.0 + assign25560_e34705);
        (assign25560_e34706, ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn0 + locals.var_vxbgmtcl_dn0)) + locals.var_ty__blk780_dn0)) * assign25560_e34704) - (assign25560_e34701 * (locals.var_fac1p2__blk803_dn0 * locals.var_beta2))) / (assign25560_e34704 * assign25560_e34704)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)) + locals.var_ty__blk780_dn2)) * assign25560_e34704) - (assign25560_e34701 * (locals.var_fac1p2__blk803_dn2 * locals.var_beta2))) / (assign25560_e34704 * assign25560_e34704)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6)) + locals.var_ty__blk780_dn6)) * assign25560_e34704) - (assign25560_e34701 * (locals.var_fac1p2__blk803_dn6 * locals.var_beta2))) / (assign25560_e34704 * assign25560_e34704)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)) + locals.var_ty__blk780_dn7)) * assign25560_e34704) - (assign25560_e34701 * (locals.var_fac1p2__blk803_dn7 * locals.var_beta2))) / (assign25560_e34704 * assign25560_e34704)), ((((4.0 * (((locals.var_beta_dn10 * assign25560_e34695) + (locals.var_beta * (locals.var_vgpld_dn10 + locals.var_vxbgmtcl_dn10))) + locals.var_ty__blk780_dn10)) * assign25560_e34704) - (assign25560_e34701 * ((locals.var_fac1p2__blk803_dn10 * locals.var_beta2) + (locals.var_fac1p2__blk803 * locals.var_beta2_dn10)))) / (assign25560_e34704 * assign25560_e34704)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn11 + locals.var_vxbgmtcl_dn11)) + locals.var_ty__blk780_dn11)) * assign25560_e34704) - (assign25560_e34701 * (locals.var_fac1p2__blk803_dn11 * locals.var_beta2))) / (assign25560_e34704 * assign25560_e34704)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn12 + locals.var_vxbgmtcl_dn12)) + locals.var_ty__blk780_dn12)) * assign25560_e34704) - (assign25560_e34701 * (locals.var_fac1p2__blk803_dn12 * locals.var_beta2))) / (assign25560_e34704 * assign25560_e34704)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn17 + locals.var_vxbgmtcl_dn17)) + locals.var_ty__blk780_dn17)) * assign25560_e34704) - (assign25560_e34701 * (locals.var_fac1p2__blk803_dn17 * locals.var_beta2))) / (assign25560_e34704 * assign25560_e34704)),)
    } else {
        (locals.var_tx__blk779, locals.var_tx__blk779_dn0, locals.var_tx__blk779_dn2, locals.var_tx__blk779_dn6, locals.var_tx__blk779_dn7, locals.var_tx__blk779_dn10, locals.var_tx__blk779_dn11, locals.var_tx__blk779_dn12, locals.var_tx__blk779_dn17,)
    }
};
        locals.var_tx__blk779 = assign25560_e34708;
        locals.var_tx__blk779_dn0 = assign25560_e34708_d_n0;
        locals.var_tx__blk779_dn2 = assign25560_e34708_d_n2;
        locals.var_tx__blk779_dn6 = assign25560_e34708_d_n6;
        locals.var_tx__blk779_dn7 = assign25560_e34708_d_n7;
        locals.var_tx__blk779_dn10 = assign25560_e34708_d_n10;
        locals.var_tx__blk779_dn11 = assign25560_e34708_d_n11;
        locals.var_tx__blk779_dn12 = assign25560_e34708_d_n12;
        locals.var_tx__blk779_dn17 = assign25560_e34708_d_n17;
        locals.var_tx__blk779_rv = 0.0;

        let assign25570_e34712: f64 = (10.0 * 2.220446049250313e-16);
        let assign25570_e34713: f64 = if locals.var_tx__blk779 < assign25570_e34712 { 1.0 } else { 0.0 };
        locals.var_guard857 = assign25570_e34713;
        locals.var_guard857_rv = 0.0;

        let (assign25580_e34728, assign25580_e34728_d_n0, assign25580_e34728_d_n2, assign25580_e34728_d_n6, assign25580_e34728_d_n7, assign25580_e34728_d_n10, assign25580_e34728_d_n11, assign25580_e34728_d_n12, assign25580_e34728_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard857 != 0.0)) {
        let assign25580_e34726: f64 = (10.0 * 2.220446049250313e-16);
        (assign25580_e34726, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tx__blk779, locals.var_tx__blk779_dn0, locals.var_tx__blk779_dn2, locals.var_tx__blk779_dn6, locals.var_tx__blk779_dn7, locals.var_tx__blk779_dn10, locals.var_tx__blk779_dn11, locals.var_tx__blk779_dn12, locals.var_tx__blk779_dn17,)
    }
};
        locals.var_tx__blk779 = assign25580_e34728;
        locals.var_tx__blk779_dn0 = assign25580_e34728_d_n0;
        locals.var_tx__blk779_dn2 = assign25580_e34728_d_n2;
        locals.var_tx__blk779_dn6 = assign25580_e34728_d_n6;
        locals.var_tx__blk779_dn7 = assign25580_e34728_d_n7;
        locals.var_tx__blk779_dn10 = assign25580_e34728_d_n10;
        locals.var_tx__blk779_dn11 = assign25580_e34728_d_n11;
        locals.var_tx__blk779_dn12 = assign25580_e34728_d_n12;
        locals.var_tx__blk779_dn17 = assign25580_e34728_d_n17;
        locals.var_tx__blk779_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_90(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign25590_e34750, assign25590_e34750_d_n0, assign25590_e34750_d_n2, assign25590_e34750_d_n6, assign25590_e34750_d_n7, assign25590_e34750_d_n10, assign25590_e34750_d_n11, assign25590_e34750_d_n12, assign25590_e34750_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) {
        let assign25590_e34740: f64 = (locals.var_fac1p2__blk803 * locals.var_beta);
        let assign25590_e34742: f64 = (assign25590_e34740 / 2.0);
        let assign25590_e34745: f64 = (locals.var_tx__blk779).sqrt();
        let assign25590_e34746: f64 = (1.0 - assign25590_e34745);
        let assign25590_e34747: f64 = (assign25590_e34742 * assign25590_e34746);
        let assign25590_e34748: f64 = (locals.var_vgpld + assign25590_e34747);
        (assign25590_e34748, (locals.var_vgpld_dn0 + ((((locals.var_fac1p2__blk803_dn0 * locals.var_beta) / 2.0) * assign25590_e34746) + (assign25590_e34742 * (-(locals.var_tx__blk779_dn0 / (2.0 * assign25590_e34745)))))), (locals.var_vgpld_dn2 + ((((locals.var_fac1p2__blk803_dn2 * locals.var_beta) / 2.0) * assign25590_e34746) + (assign25590_e34742 * (-(locals.var_tx__blk779_dn2 / (2.0 * assign25590_e34745)))))), (locals.var_vgpld_dn6 + ((((locals.var_fac1p2__blk803_dn6 * locals.var_beta) / 2.0) * assign25590_e34746) + (assign25590_e34742 * (-(locals.var_tx__blk779_dn6 / (2.0 * assign25590_e34745)))))), (locals.var_vgpld_dn7 + ((((locals.var_fac1p2__blk803_dn7 * locals.var_beta) / 2.0) * assign25590_e34746) + (assign25590_e34742 * (-(locals.var_tx__blk779_dn7 / (2.0 * assign25590_e34745)))))), (locals.var_vgpld_dn10 + (((((locals.var_fac1p2__blk803_dn10 * locals.var_beta) + (locals.var_fac1p2__blk803 * locals.var_beta_dn10)) / 2.0) * assign25590_e34746) + (assign25590_e34742 * (-(locals.var_tx__blk779_dn10 / (2.0 * assign25590_e34745)))))), (locals.var_vgpld_dn11 + ((((locals.var_fac1p2__blk803_dn11 * locals.var_beta) / 2.0) * assign25590_e34746) + (assign25590_e34742 * (-(locals.var_tx__blk779_dn11 / (2.0 * assign25590_e34745)))))), (locals.var_vgpld_dn12 + ((((locals.var_fac1p2__blk803_dn12 * locals.var_beta) / 2.0) * assign25590_e34746) + (assign25590_e34742 * (-(locals.var_tx__blk779_dn12 / (2.0 * assign25590_e34745)))))), (locals.var_vgpld_dn17 + ((((locals.var_fac1p2__blk803_dn17 * locals.var_beta) / 2.0) * assign25590_e34746) + (assign25590_e34742 * (-(locals.var_tx__blk779_dn17 / (2.0 * assign25590_e34745)))))),)
    } else {
        (locals.var_ps0_inia__blk819, locals.var_ps0_inia__blk819_dn0, locals.var_ps0_inia__blk819_dn2, locals.var_ps0_inia__blk819_dn6, locals.var_ps0_inia__blk819_dn7, locals.var_ps0_inia__blk819_dn10, locals.var_ps0_inia__blk819_dn11, locals.var_ps0_inia__blk819_dn12, locals.var_ps0_inia__blk819_dn17,)
    }
};
        locals.var_ps0_inia__blk819 = assign25590_e34750;
        locals.var_ps0_inia__blk819_dn0 = assign25590_e34750_d_n0;
        locals.var_ps0_inia__blk819_dn2 = assign25590_e34750_d_n2;
        locals.var_ps0_inia__blk819_dn6 = assign25590_e34750_d_n6;
        locals.var_ps0_inia__blk819_dn7 = assign25590_e34750_d_n7;
        locals.var_ps0_inia__blk819_dn10 = assign25590_e34750_d_n10;
        locals.var_ps0_inia__blk819_dn11 = assign25590_e34750_d_n11;
        locals.var_ps0_inia__blk819_dn12 = assign25590_e34750_d_n12;
        locals.var_ps0_inia__blk819_dn17 = assign25590_e34750_d_n17;
        locals.var_ps0_inia__blk819_rv = 0.0;

        let (assign25600_e34765, assign25600_e34765_d_n0, assign25600_e34765_d_n2, assign25600_e34765_d_n6, assign25600_e34765_d_n7, assign25600_e34765_d_n10, assign25600_e34765_d_n11, assign25600_e34765_d_n12, assign25600_e34765_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) {
        let assign25600_e34762: f64 = (locals.var_ps0_inia__blk819 + locals.var_vxbgmtcl);
        let assign25600_e34763: f64 = (locals.var_beta * assign25600_e34762);
        (assign25600_e34763, (locals.var_beta * (locals.var_ps0_inia__blk819_dn0 + locals.var_vxbgmtcl_dn0)), (locals.var_beta * (locals.var_ps0_inia__blk819_dn2 + locals.var_vxbgmtcl_dn2)), (locals.var_beta * (locals.var_ps0_inia__blk819_dn6 + locals.var_vxbgmtcl_dn6)), (locals.var_beta * (locals.var_ps0_inia__blk819_dn7 + locals.var_vxbgmtcl_dn7)), ((locals.var_beta_dn10 * assign25600_e34762) + (locals.var_beta * (locals.var_ps0_inia__blk819_dn10 + locals.var_vxbgmtcl_dn10))), (locals.var_beta * (locals.var_ps0_inia__blk819_dn11 + locals.var_vxbgmtcl_dn11)), (locals.var_beta * (locals.var_ps0_inia__blk819_dn12 + locals.var_vxbgmtcl_dn12)), (locals.var_beta * (locals.var_ps0_inia__blk819_dn17 + locals.var_vxbgmtcl_dn17)),)
    } else {
        (locals.var_chi__blk816, locals.var_chi__blk816_dn0, locals.var_chi__blk816_dn2, locals.var_chi__blk816_dn6, locals.var_chi__blk816_dn7, locals.var_chi__blk816_dn10, locals.var_chi__blk816_dn11, locals.var_chi__blk816_dn12, locals.var_chi__blk816_dn17,)
    }
};
        locals.var_chi__blk816 = assign25600_e34765;
        locals.var_chi__blk816_dn0 = assign25600_e34765_d_n0;
        locals.var_chi__blk816_dn2 = assign25600_e34765_d_n2;
        locals.var_chi__blk816_dn6 = assign25600_e34765_d_n6;
        locals.var_chi__blk816_dn7 = assign25600_e34765_d_n7;
        locals.var_chi__blk816_dn10 = assign25600_e34765_d_n10;
        locals.var_chi__blk816_dn11 = assign25600_e34765_d_n11;
        locals.var_chi__blk816_dn12 = assign25600_e34765_d_n12;
        locals.var_chi__blk816_dn17 = assign25600_e34765_d_n17;
        locals.var_chi__blk816_rv = 0.0;

        let (assign25610_e34778, assign25610_e34778_d_n0, assign25610_e34778_d_n2, assign25610_e34778_d_n6, assign25610_e34778_d_n7, assign25610_e34778_d_n10, assign25610_e34778_d_n11, assign25610_e34778_d_n12, assign25610_e34778_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) {
        let assign25610_e34775: f64 = (-locals.var_chi__blk816);
        let assign25610_e34776: f64 = (assign25610_e34775).exp();
        (assign25610_e34776, (assign25610_e34776 * (-locals.var_chi__blk816_dn0)), (assign25610_e34776 * (-locals.var_chi__blk816_dn2)), (assign25610_e34776 * (-locals.var_chi__blk816_dn6)), (assign25610_e34776 * (-locals.var_chi__blk816_dn7)), (assign25610_e34776 * (-locals.var_chi__blk816_dn10)), (assign25610_e34776 * (-locals.var_chi__blk816_dn11)), (assign25610_e34776 * (-locals.var_chi__blk816_dn12)), (assign25610_e34776 * (-locals.var_chi__blk816_dn17)),)
    } else {
        (locals.var_ty__blk780, locals.var_ty__blk780_dn0, locals.var_ty__blk780_dn2, locals.var_ty__blk780_dn6, locals.var_ty__blk780_dn7, locals.var_ty__blk780_dn10, locals.var_ty__blk780_dn11, locals.var_ty__blk780_dn12, locals.var_ty__blk780_dn17,)
    }
};
        locals.var_ty__blk780 = assign25610_e34778;
        locals.var_ty__blk780_dn0 = assign25610_e34778_d_n0;
        locals.var_ty__blk780_dn2 = assign25610_e34778_d_n2;
        locals.var_ty__blk780_dn6 = assign25610_e34778_d_n6;
        locals.var_ty__blk780_dn7 = assign25610_e34778_d_n7;
        locals.var_ty__blk780_dn10 = assign25610_e34778_d_n10;
        locals.var_ty__blk780_dn11 = assign25610_e34778_d_n11;
        locals.var_ty__blk780_dn12 = assign25610_e34778_d_n12;
        locals.var_ty__blk780_dn17 = assign25610_e34778_d_n17;
        locals.var_ty__blk780_rv = 0.0;

        let (assign25620_e34805, assign25620_e34805_d_n0, assign25620_e34805_d_n2, assign25620_e34805_d_n6, assign25620_e34805_d_n7, assign25620_e34805_d_n10, assign25620_e34805_d_n11, assign25620_e34805_d_n12, assign25620_e34805_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) {
        let assign25620_e34792: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign25620_e34793: f64 = (locals.var_beta * assign25620_e34792);
        let assign25620_e34795: f64 = (assign25620_e34793 - 1.0);
        let assign25620_e34797: f64 = (assign25620_e34795 + locals.var_ty__blk780);
        let assign25620_e34798: f64 = (4.0 * assign25620_e34797);
        let assign25620_e34801: f64 = (locals.var_fac1p2__blk803 * locals.var_beta2);
        let assign25620_e34802: f64 = (assign25620_e34798 / assign25620_e34801);
        let assign25620_e34803: f64 = (1.0 + assign25620_e34802);
        (assign25620_e34803, ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn0 + locals.var_vxbgmtcl_dn0)) + locals.var_ty__blk780_dn0)) * assign25620_e34801) - (assign25620_e34798 * (locals.var_fac1p2__blk803_dn0 * locals.var_beta2))) / (assign25620_e34801 * assign25620_e34801)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)) + locals.var_ty__blk780_dn2)) * assign25620_e34801) - (assign25620_e34798 * (locals.var_fac1p2__blk803_dn2 * locals.var_beta2))) / (assign25620_e34801 * assign25620_e34801)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6)) + locals.var_ty__blk780_dn6)) * assign25620_e34801) - (assign25620_e34798 * (locals.var_fac1p2__blk803_dn6 * locals.var_beta2))) / (assign25620_e34801 * assign25620_e34801)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)) + locals.var_ty__blk780_dn7)) * assign25620_e34801) - (assign25620_e34798 * (locals.var_fac1p2__blk803_dn7 * locals.var_beta2))) / (assign25620_e34801 * assign25620_e34801)), ((((4.0 * (((locals.var_beta_dn10 * assign25620_e34792) + (locals.var_beta * (locals.var_vgpld_dn10 + locals.var_vxbgmtcl_dn10))) + locals.var_ty__blk780_dn10)) * assign25620_e34801) - (assign25620_e34798 * ((locals.var_fac1p2__blk803_dn10 * locals.var_beta2) + (locals.var_fac1p2__blk803 * locals.var_beta2_dn10)))) / (assign25620_e34801 * assign25620_e34801)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn11 + locals.var_vxbgmtcl_dn11)) + locals.var_ty__blk780_dn11)) * assign25620_e34801) - (assign25620_e34798 * (locals.var_fac1p2__blk803_dn11 * locals.var_beta2))) / (assign25620_e34801 * assign25620_e34801)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn12 + locals.var_vxbgmtcl_dn12)) + locals.var_ty__blk780_dn12)) * assign25620_e34801) - (assign25620_e34798 * (locals.var_fac1p2__blk803_dn12 * locals.var_beta2))) / (assign25620_e34801 * assign25620_e34801)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn17 + locals.var_vxbgmtcl_dn17)) + locals.var_ty__blk780_dn17)) * assign25620_e34801) - (assign25620_e34798 * (locals.var_fac1p2__blk803_dn17 * locals.var_beta2))) / (assign25620_e34801 * assign25620_e34801)),)
    } else {
        (locals.var_tx__blk779, locals.var_tx__blk779_dn0, locals.var_tx__blk779_dn2, locals.var_tx__blk779_dn6, locals.var_tx__blk779_dn7, locals.var_tx__blk779_dn10, locals.var_tx__blk779_dn11, locals.var_tx__blk779_dn12, locals.var_tx__blk779_dn17,)
    }
};
        locals.var_tx__blk779 = assign25620_e34805;
        locals.var_tx__blk779_dn0 = assign25620_e34805_d_n0;
        locals.var_tx__blk779_dn2 = assign25620_e34805_d_n2;
        locals.var_tx__blk779_dn6 = assign25620_e34805_d_n6;
        locals.var_tx__blk779_dn7 = assign25620_e34805_d_n7;
        locals.var_tx__blk779_dn10 = assign25620_e34805_d_n10;
        locals.var_tx__blk779_dn11 = assign25620_e34805_d_n11;
        locals.var_tx__blk779_dn12 = assign25620_e34805_d_n12;
        locals.var_tx__blk779_dn17 = assign25620_e34805_d_n17;
        locals.var_tx__blk779_rv = 0.0;

        let assign25630_e34809: f64 = (10.0 * 2.220446049250313e-16);
        let assign25630_e34810: f64 = if locals.var_tx__blk779 < assign25630_e34809 { 1.0 } else { 0.0 };
        locals.var_guard858 = assign25630_e34810;
        locals.var_guard858_rv = 0.0;

        let (assign25640_e34825, assign25640_e34825_d_n0, assign25640_e34825_d_n2, assign25640_e34825_d_n6, assign25640_e34825_d_n7, assign25640_e34825_d_n10, assign25640_e34825_d_n11, assign25640_e34825_d_n12, assign25640_e34825_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard858 != 0.0)) {
        let assign25640_e34823: f64 = (10.0 * 2.220446049250313e-16);
        (assign25640_e34823, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tx__blk779, locals.var_tx__blk779_dn0, locals.var_tx__blk779_dn2, locals.var_tx__blk779_dn6, locals.var_tx__blk779_dn7, locals.var_tx__blk779_dn10, locals.var_tx__blk779_dn11, locals.var_tx__blk779_dn12, locals.var_tx__blk779_dn17,)
    }
};
        locals.var_tx__blk779 = assign25640_e34825;
        locals.var_tx__blk779_dn0 = assign25640_e34825_d_n0;
        locals.var_tx__blk779_dn2 = assign25640_e34825_d_n2;
        locals.var_tx__blk779_dn6 = assign25640_e34825_d_n6;
        locals.var_tx__blk779_dn7 = assign25640_e34825_d_n7;
        locals.var_tx__blk779_dn10 = assign25640_e34825_d_n10;
        locals.var_tx__blk779_dn11 = assign25640_e34825_d_n11;
        locals.var_tx__blk779_dn12 = assign25640_e34825_d_n12;
        locals.var_tx__blk779_dn17 = assign25640_e34825_d_n17;
        locals.var_tx__blk779_rv = 0.0;

        let (assign25650_e34847, assign25650_e34847_d_n0, assign25650_e34847_d_n2, assign25650_e34847_d_n6, assign25650_e34847_d_n7, assign25650_e34847_d_n10, assign25650_e34847_d_n11, assign25650_e34847_d_n12, assign25650_e34847_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) {
        let assign25650_e34837: f64 = (locals.var_fac1p2__blk803 * locals.var_beta);
        let assign25650_e34839: f64 = (assign25650_e34837 / 2.0);
        let assign25650_e34842: f64 = (locals.var_tx__blk779).sqrt();
        let assign25650_e34843: f64 = (1.0 - assign25650_e34842);
        let assign25650_e34844: f64 = (assign25650_e34839 * assign25650_e34843);
        let assign25650_e34845: f64 = (locals.var_vgpld + assign25650_e34844);
        (assign25650_e34845, (locals.var_vgpld_dn0 + ((((locals.var_fac1p2__blk803_dn0 * locals.var_beta) / 2.0) * assign25650_e34843) + (assign25650_e34839 * (-(locals.var_tx__blk779_dn0 / (2.0 * assign25650_e34842)))))), (locals.var_vgpld_dn2 + ((((locals.var_fac1p2__blk803_dn2 * locals.var_beta) / 2.0) * assign25650_e34843) + (assign25650_e34839 * (-(locals.var_tx__blk779_dn2 / (2.0 * assign25650_e34842)))))), (locals.var_vgpld_dn6 + ((((locals.var_fac1p2__blk803_dn6 * locals.var_beta) / 2.0) * assign25650_e34843) + (assign25650_e34839 * (-(locals.var_tx__blk779_dn6 / (2.0 * assign25650_e34842)))))), (locals.var_vgpld_dn7 + ((((locals.var_fac1p2__blk803_dn7 * locals.var_beta) / 2.0) * assign25650_e34843) + (assign25650_e34839 * (-(locals.var_tx__blk779_dn7 / (2.0 * assign25650_e34842)))))), (locals.var_vgpld_dn10 + (((((locals.var_fac1p2__blk803_dn10 * locals.var_beta) + (locals.var_fac1p2__blk803 * locals.var_beta_dn10)) / 2.0) * assign25650_e34843) + (assign25650_e34839 * (-(locals.var_tx__blk779_dn10 / (2.0 * assign25650_e34842)))))), (locals.var_vgpld_dn11 + ((((locals.var_fac1p2__blk803_dn11 * locals.var_beta) / 2.0) * assign25650_e34843) + (assign25650_e34839 * (-(locals.var_tx__blk779_dn11 / (2.0 * assign25650_e34842)))))), (locals.var_vgpld_dn12 + ((((locals.var_fac1p2__blk803_dn12 * locals.var_beta) / 2.0) * assign25650_e34843) + (assign25650_e34839 * (-(locals.var_tx__blk779_dn12 / (2.0 * assign25650_e34842)))))), (locals.var_vgpld_dn17 + ((((locals.var_fac1p2__blk803_dn17 * locals.var_beta) / 2.0) * assign25650_e34843) + (assign25650_e34839 * (-(locals.var_tx__blk779_dn17 / (2.0 * assign25650_e34842)))))),)
    } else {
        (locals.var_ps0_inia__blk819, locals.var_ps0_inia__blk819_dn0, locals.var_ps0_inia__blk819_dn2, locals.var_ps0_inia__blk819_dn6, locals.var_ps0_inia__blk819_dn7, locals.var_ps0_inia__blk819_dn10, locals.var_ps0_inia__blk819_dn11, locals.var_ps0_inia__blk819_dn12, locals.var_ps0_inia__blk819_dn17,)
    }
};
        locals.var_ps0_inia__blk819 = assign25650_e34847;
        locals.var_ps0_inia__blk819_dn0 = assign25650_e34847_d_n0;
        locals.var_ps0_inia__blk819_dn2 = assign25650_e34847_d_n2;
        locals.var_ps0_inia__blk819_dn6 = assign25650_e34847_d_n6;
        locals.var_ps0_inia__blk819_dn7 = assign25650_e34847_d_n7;
        locals.var_ps0_inia__blk819_dn10 = assign25650_e34847_d_n10;
        locals.var_ps0_inia__blk819_dn11 = assign25650_e34847_d_n11;
        locals.var_ps0_inia__blk819_dn12 = assign25650_e34847_d_n12;
        locals.var_ps0_inia__blk819_dn17 = assign25650_e34847_d_n17;
        locals.var_ps0_inia__blk819_rv = 0.0;

        let (assign25660_e34862, assign25660_e34862_d_n0, assign25660_e34862_d_n2, assign25660_e34862_d_n6, assign25660_e34862_d_n7, assign25660_e34862_d_n10, assign25660_e34862_d_n11, assign25660_e34862_d_n12, assign25660_e34862_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) {
        let assign25660_e34859: f64 = (locals.var_ps0_inia__blk819 + locals.var_vxbgmtcl);
        let assign25660_e34860: f64 = (locals.var_beta * assign25660_e34859);
        (assign25660_e34860, (locals.var_beta * (locals.var_ps0_inia__blk819_dn0 + locals.var_vxbgmtcl_dn0)), (locals.var_beta * (locals.var_ps0_inia__blk819_dn2 + locals.var_vxbgmtcl_dn2)), (locals.var_beta * (locals.var_ps0_inia__blk819_dn6 + locals.var_vxbgmtcl_dn6)), (locals.var_beta * (locals.var_ps0_inia__blk819_dn7 + locals.var_vxbgmtcl_dn7)), ((locals.var_beta_dn10 * assign25660_e34859) + (locals.var_beta * (locals.var_ps0_inia__blk819_dn10 + locals.var_vxbgmtcl_dn10))), (locals.var_beta * (locals.var_ps0_inia__blk819_dn11 + locals.var_vxbgmtcl_dn11)), (locals.var_beta * (locals.var_ps0_inia__blk819_dn12 + locals.var_vxbgmtcl_dn12)), (locals.var_beta * (locals.var_ps0_inia__blk819_dn17 + locals.var_vxbgmtcl_dn17)),)
    } else {
        (locals.var_chi__blk816, locals.var_chi__blk816_dn0, locals.var_chi__blk816_dn2, locals.var_chi__blk816_dn6, locals.var_chi__blk816_dn7, locals.var_chi__blk816_dn10, locals.var_chi__blk816_dn11, locals.var_chi__blk816_dn12, locals.var_chi__blk816_dn17,)
    }
};
        locals.var_chi__blk816 = assign25660_e34862;
        locals.var_chi__blk816_dn0 = assign25660_e34862_d_n0;
        locals.var_chi__blk816_dn2 = assign25660_e34862_d_n2;
        locals.var_chi__blk816_dn6 = assign25660_e34862_d_n6;
        locals.var_chi__blk816_dn7 = assign25660_e34862_d_n7;
        locals.var_chi__blk816_dn10 = assign25660_e34862_d_n10;
        locals.var_chi__blk816_dn11 = assign25660_e34862_d_n11;
        locals.var_chi__blk816_dn12 = assign25660_e34862_d_n12;
        locals.var_chi__blk816_dn17 = assign25660_e34862_d_n17;
        locals.var_chi__blk816_rv = 0.0;

        let assign25670_e34865: f64 = if locals.var_chi__blk816 < 3.0 { 1.0 } else { 0.0 };
        locals.var_guard859 = assign25670_e34865;
        locals.var_guard859_rv = 0.0;

        let (assign25690_e34908,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard859 != 0.0)) {
        let assign25690_e34892: f64 = (9.0 * 1.414213562373095);
        let assign25690_e34893: f64 = (1.0 / assign25690_e34892);
        let assign25690_e34897: f64 = (7.0 * 0.049787068367863944);
        let assign25690_e34898: f64 = (5.0 + assign25690_e34897);
        let assign25690_e34902: f64 = (2.0 + 0.049787068367863944);
        let assign25690_e34903: f64 = (assign25690_e34902).sqrt();
        let assign25690_e34904: f64 = (54.0 * assign25690_e34903);
        let assign25690_e34905: f64 = (assign25690_e34898 / assign25690_e34904);
        let assign25690_e34906: f64 = (assign25690_e34893 - assign25690_e34905);
        (assign25690_e34906,)
    } else {
        (locals.var_ta,)
    }
};
        locals.var_ta = assign25690_e34908;
        locals.var_ta_rv = 0.0;

        let (assign25700_e34934,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard859 != 0.0)) {
        let assign25700_e34921: f64 = (1.0 + 0.049787068367863944);
        let assign25700_e34925: f64 = (2.0 + 0.049787068367863944);
        let assign25700_e34926: f64 = (assign25700_e34925).sqrt();
        let assign25700_e34927: f64 = (2.0 * assign25700_e34926);
        let assign25700_e34928: f64 = (assign25700_e34921 / assign25700_e34927);
        let assign25700_e34931: f64 = (1.414213562373095 / 3.0);
        let assign25700_e34932: f64 = (assign25700_e34928 - assign25700_e34931);
        (assign25700_e34932,)
    } else {
        (locals.var_tb,)
    }
};
        locals.var_tb = assign25700_e34934;
        locals.var_tb_rv = 0.0;

        let (assign25710_e34955, assign25710_e34955_d_n0, assign25710_e34955_d_n2, assign25710_e34955_d_n6, assign25710_e34955_d_n7, assign25710_e34955_d_n10, assign25710_e34955_d_n11, assign25710_e34955_d_n12, assign25710_e34955_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard859 != 0.0)) {
        let assign25710_e34947: f64 = (1.0 / 1.414213562373095);
        let assign25710_e34951: f64 = (locals.var_beta * locals.var_fac1__blk802);
        let assign25710_e34952: f64 = (1.0 / assign25710_e34951);
        let assign25710_e34953: f64 = (assign25710_e34947 + assign25710_e34952);
        (assign25710_e34953, (-((locals.var_beta * locals.var_fac1__blk802_dn0) / (assign25710_e34951 * assign25710_e34951))), (-((locals.var_beta * locals.var_fac1__blk802_dn2) / (assign25710_e34951 * assign25710_e34951))), (-((locals.var_beta * locals.var_fac1__blk802_dn6) / (assign25710_e34951 * assign25710_e34951))), (-((locals.var_beta * locals.var_fac1__blk802_dn7) / (assign25710_e34951 * assign25710_e34951))), (-(((locals.var_beta_dn10 * locals.var_fac1__blk802) + (locals.var_beta * locals.var_fac1__blk802_dn10)) / (assign25710_e34951 * assign25710_e34951))), (-((locals.var_beta * locals.var_fac1__blk802_dn11) / (assign25710_e34951 * assign25710_e34951))), (-((locals.var_beta * locals.var_fac1__blk802_dn12) / (assign25710_e34951 * assign25710_e34951))), (-((locals.var_beta * locals.var_fac1__blk802_dn17) / (assign25710_e34951 * assign25710_e34951))),)
    } else {
        (locals.var_tc, locals.var_tc_dn0, locals.var_tc_dn2, locals.var_tc_dn6, locals.var_tc_dn7, locals.var_tc_dn10, locals.var_tc_dn11, locals.var_tc_dn12, locals.var_tc_dn17,)
    }
};
        locals.var_tc = assign25710_e34955;
        locals.var_tc_dn0 = assign25710_e34955_d_n0;
        locals.var_tc_dn2 = assign25710_e34955_d_n2;
        locals.var_tc_dn6 = assign25710_e34955_d_n6;
        locals.var_tc_dn7 = assign25710_e34955_d_n7;
        locals.var_tc_dn10 = assign25710_e34955_d_n10;
        locals.var_tc_dn11 = assign25710_e34955_d_n11;
        locals.var_tc_dn12 = assign25710_e34955_d_n12;
        locals.var_tc_dn17 = assign25710_e34955_d_n17;
        locals.var_tc_rv = 0.0;

        let (assign25720_e34973, assign25720_e34973_d_n0, assign25720_e34973_d_n2, assign25720_e34973_d_n6, assign25720_e34973_d_n7, assign25720_e34973_d_n10, assign25720_e34973_d_n11, assign25720_e34973_d_n12, assign25720_e34973_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard859 != 0.0)) {
        let assign25720_e34968: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign25720_e34969: f64 = (-assign25720_e34968);
        let assign25720_e34971: f64 = (assign25720_e34969 / locals.var_fac1__blk802);
        (assign25720_e34971, ((((-(locals.var_vgpld_dn0 + locals.var_vxbgmtcl_dn0)) * locals.var_fac1__blk802) - (assign25720_e34969 * locals.var_fac1__blk802_dn0)) / (locals.var_fac1__blk802 * locals.var_fac1__blk802)), ((((-(locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)) * locals.var_fac1__blk802) - (assign25720_e34969 * locals.var_fac1__blk802_dn2)) / (locals.var_fac1__blk802 * locals.var_fac1__blk802)), ((((-(locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6)) * locals.var_fac1__blk802) - (assign25720_e34969 * locals.var_fac1__blk802_dn6)) / (locals.var_fac1__blk802 * locals.var_fac1__blk802)), ((((-(locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)) * locals.var_fac1__blk802) - (assign25720_e34969 * locals.var_fac1__blk802_dn7)) / (locals.var_fac1__blk802 * locals.var_fac1__blk802)), ((((-(locals.var_vgpld_dn10 + locals.var_vxbgmtcl_dn10)) * locals.var_fac1__blk802) - (assign25720_e34969 * locals.var_fac1__blk802_dn10)) / (locals.var_fac1__blk802 * locals.var_fac1__blk802)), ((((-(locals.var_vgpld_dn11 + locals.var_vxbgmtcl_dn11)) * locals.var_fac1__blk802) - (assign25720_e34969 * locals.var_fac1__blk802_dn11)) / (locals.var_fac1__blk802 * locals.var_fac1__blk802)), ((((-(locals.var_vgpld_dn12 + locals.var_vxbgmtcl_dn12)) * locals.var_fac1__blk802) - (assign25720_e34969 * locals.var_fac1__blk802_dn12)) / (locals.var_fac1__blk802 * locals.var_fac1__blk802)), ((((-(locals.var_vgpld_dn17 + locals.var_vxbgmtcl_dn17)) * locals.var_fac1__blk802) - (assign25720_e34969 * locals.var_fac1__blk802_dn17)) / (locals.var_fac1__blk802 * locals.var_fac1__blk802)),)
    } else {
        (locals.var_td, locals.var_td_dn0, locals.var_td_dn2, locals.var_td_dn6, locals.var_td_dn7, locals.var_td_dn10, locals.var_td_dn11, locals.var_td_dn12, locals.var_td_dn17,)
    }
};
        locals.var_td = assign25720_e34973;
        locals.var_td_dn0 = assign25720_e34973_d_n0;
        locals.var_td_dn2 = assign25720_e34973_d_n2;
        locals.var_td_dn6 = assign25720_e34973_d_n6;
        locals.var_td_dn7 = assign25720_e34973_d_n7;
        locals.var_td_dn10 = assign25720_e34973_d_n10;
        locals.var_td_dn11 = assign25720_e34973_d_n11;
        locals.var_td_dn12 = assign25720_e34973_d_n12;
        locals.var_td_dn17 = assign25720_e34973_d_n17;
        locals.var_td_rv = 0.0;

        let (assign25730_e35014, assign25730_e35014_d_n0, assign25730_e35014_d_n2, assign25730_e35014_d_n6, assign25730_e35014_d_n7, assign25730_e35014_d_n10, assign25730_e35014_d_n11, assign25730_e35014_d_n12, assign25730_e35014_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard859 != 0.0)) {
        let assign25730_e34986: f64 = (locals.var_tb * locals.var_tb);
        let assign25730_e34988: f64 = (assign25730_e34986 * locals.var_tb);
        let assign25730_e34991: f64 = (27.0 * locals.var_ta);
        let assign25730_e34993: f64 = (assign25730_e34991 * locals.var_ta);
        let assign25730_e34995: f64 = (assign25730_e34993 * locals.var_ta);
        let assign25730_e34996: f64 = (assign25730_e34988 / assign25730_e34995);
        let assign25730_e34999: f64 = (locals.var_tb * locals.var_tc);
        let assign25730_e35002: f64 = (6.0 * locals.var_ta);
        let assign25730_e35004: f64 = (assign25730_e35002 * locals.var_ta);
        let assign25730_e35005: f64 = (assign25730_e34999 / assign25730_e35004);
        let assign25730_e35006: f64 = (assign25730_e34996 - assign25730_e35005);
        let assign25730_e35010: f64 = (2.0 * locals.var_ta);
        let assign25730_e35011: f64 = (locals.var_td / assign25730_e35010);
        let assign25730_e35012: f64 = (assign25730_e35006 + assign25730_e35011);
        (assign25730_e35012, ((-((locals.var_tb * locals.var_tc_dn0) / assign25730_e35004)) + (locals.var_td_dn0 / assign25730_e35010)), ((-((locals.var_tb * locals.var_tc_dn2) / assign25730_e35004)) + (locals.var_td_dn2 / assign25730_e35010)), ((-((locals.var_tb * locals.var_tc_dn6) / assign25730_e35004)) + (locals.var_td_dn6 / assign25730_e35010)), ((-((locals.var_tb * locals.var_tc_dn7) / assign25730_e35004)) + (locals.var_td_dn7 / assign25730_e35010)), ((-((locals.var_tb * locals.var_tc_dn10) / assign25730_e35004)) + (locals.var_td_dn10 / assign25730_e35010)), ((-((locals.var_tb * locals.var_tc_dn11) / assign25730_e35004)) + (locals.var_td_dn11 / assign25730_e35010)), ((-((locals.var_tb * locals.var_tc_dn12) / assign25730_e35004)) + (locals.var_td_dn12 / assign25730_e35010)), ((-((locals.var_tb * locals.var_tc_dn17) / assign25730_e35004)) + (locals.var_td_dn17 / assign25730_e35010)),)
    } else {
        (locals.var_tq, locals.var_tq_dn0, locals.var_tq_dn2, locals.var_tq_dn6, locals.var_tq_dn7, locals.var_tq_dn10, locals.var_tq_dn11, locals.var_tq_dn12, locals.var_tq_dn17,)
    }
};
        locals.var_tq = assign25730_e35014;
        locals.var_tq_dn0 = assign25730_e35014_d_n0;
        locals.var_tq_dn2 = assign25730_e35014_d_n2;
        locals.var_tq_dn6 = assign25730_e35014_d_n6;
        locals.var_tq_dn7 = assign25730_e35014_d_n7;
        locals.var_tq_dn10 = assign25730_e35014_d_n10;
        locals.var_tq_dn11 = assign25730_e35014_d_n11;
        locals.var_tq_dn12 = assign25730_e35014_d_n12;
        locals.var_tq_dn17 = assign25730_e35014_d_n17;
        locals.var_tq_rv = 0.0;

        let (assign25740_e35041, assign25740_e35041_d_n0, assign25740_e35041_d_n2, assign25740_e35041_d_n6, assign25740_e35041_d_n7, assign25740_e35041_d_n10, assign25740_e35041_d_n11, assign25740_e35041_d_n12, assign25740_e35041_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard859 != 0.0)) {
        let assign25740_e35027: f64 = (3.0 * locals.var_ta);
        let assign25740_e35029: f64 = (assign25740_e35027 * locals.var_tc);
        let assign25740_e35032: f64 = (locals.var_tb * locals.var_tb);
        let assign25740_e35033: f64 = (assign25740_e35029 - assign25740_e35032);
        let assign25740_e35036: f64 = (9.0 * locals.var_ta);
        let assign25740_e35038: f64 = (assign25740_e35036 * locals.var_ta);
        let assign25740_e35039: f64 = (assign25740_e35033 / assign25740_e35038);
        (assign25740_e35039, ((assign25740_e35027 * locals.var_tc_dn0) / assign25740_e35038), ((assign25740_e35027 * locals.var_tc_dn2) / assign25740_e35038), ((assign25740_e35027 * locals.var_tc_dn6) / assign25740_e35038), ((assign25740_e35027 * locals.var_tc_dn7) / assign25740_e35038), ((assign25740_e35027 * locals.var_tc_dn10) / assign25740_e35038), ((assign25740_e35027 * locals.var_tc_dn11) / assign25740_e35038), ((assign25740_e35027 * locals.var_tc_dn12) / assign25740_e35038), ((assign25740_e35027 * locals.var_tc_dn17) / assign25740_e35038),)
    } else {
        (locals.var_tp, locals.var_tp_dn0, locals.var_tp_dn2, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn10, locals.var_tp_dn11, locals.var_tp_dn12, locals.var_tp_dn17,)
    }
};
        locals.var_tp = assign25740_e35041;
        locals.var_tp_dn0 = assign25740_e35041_d_n0;
        locals.var_tp_dn2 = assign25740_e35041_d_n2;
        locals.var_tp_dn6 = assign25740_e35041_d_n6;
        locals.var_tp_dn7 = assign25740_e35041_d_n7;
        locals.var_tp_dn10 = assign25740_e35041_d_n10;
        locals.var_tp_dn11 = assign25740_e35041_d_n11;
        locals.var_tp_dn12 = assign25740_e35041_d_n12;
        locals.var_tp_dn17 = assign25740_e35041_d_n17;
        locals.var_tp_rv = 0.0;

        let (assign25750_e35063, assign25750_e35063_d_n0, assign25750_e35063_d_n2, assign25750_e35063_d_n6, assign25750_e35063_d_n7, assign25750_e35063_d_n10, assign25750_e35063_d_n11, assign25750_e35063_d_n12, assign25750_e35063_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard859 != 0.0)) {
        let assign25750_e35054: f64 = (locals.var_tq * locals.var_tq);
        let assign25750_e35057: f64 = (locals.var_tp * locals.var_tp);
        let assign25750_e35059: f64 = (assign25750_e35057 * locals.var_tp);
        let assign25750_e35060: f64 = (assign25750_e35054 + assign25750_e35059);
        let assign25750_e35061: f64 = (assign25750_e35060).sqrt();
        (assign25750_e35061, ((((locals.var_tq_dn0 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn0)) + ((((locals.var_tp_dn0 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn0)) * locals.var_tp) + (assign25750_e35057 * locals.var_tp_dn0))) / (2.0 * assign25750_e35061)), ((((locals.var_tq_dn2 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn2)) + ((((locals.var_tp_dn2 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn2)) * locals.var_tp) + (assign25750_e35057 * locals.var_tp_dn2))) / (2.0 * assign25750_e35061)), ((((locals.var_tq_dn6 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn6)) + ((((locals.var_tp_dn6 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn6)) * locals.var_tp) + (assign25750_e35057 * locals.var_tp_dn6))) / (2.0 * assign25750_e35061)), ((((locals.var_tq_dn7 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn7)) + ((((locals.var_tp_dn7 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn7)) * locals.var_tp) + (assign25750_e35057 * locals.var_tp_dn7))) / (2.0 * assign25750_e35061)), ((((locals.var_tq_dn10 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn10)) + ((((locals.var_tp_dn10 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn10)) * locals.var_tp) + (assign25750_e35057 * locals.var_tp_dn10))) / (2.0 * assign25750_e35061)), ((((locals.var_tq_dn11 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn11)) + ((((locals.var_tp_dn11 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn11)) * locals.var_tp) + (assign25750_e35057 * locals.var_tp_dn11))) / (2.0 * assign25750_e35061)), ((((locals.var_tq_dn12 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn12)) + ((((locals.var_tp_dn12 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn12)) * locals.var_tp) + (assign25750_e35057 * locals.var_tp_dn12))) / (2.0 * assign25750_e35061)), ((((locals.var_tq_dn17 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn17)) + ((((locals.var_tp_dn17 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn17)) * locals.var_tp) + (assign25750_e35057 * locals.var_tp_dn17))) / (2.0 * assign25750_e35061)),)
    } else {
        (locals.var_t5__blk776, locals.var_t5__blk776_dn0, locals.var_t5__blk776_dn2, locals.var_t5__blk776_dn6, locals.var_t5__blk776_dn7, locals.var_t5__blk776_dn10, locals.var_t5__blk776_dn11, locals.var_t5__blk776_dn12, locals.var_t5__blk776_dn17,)
    }
};
        locals.var_t5__blk776 = assign25750_e35063;
        locals.var_t5__blk776_dn0 = assign25750_e35063_d_n0;
        locals.var_t5__blk776_dn2 = assign25750_e35063_d_n2;
        locals.var_t5__blk776_dn6 = assign25750_e35063_d_n6;
        locals.var_t5__blk776_dn7 = assign25750_e35063_d_n7;
        locals.var_t5__blk776_dn10 = assign25750_e35063_d_n10;
        locals.var_t5__blk776_dn11 = assign25750_e35063_d_n11;
        locals.var_t5__blk776_dn12 = assign25750_e35063_d_n12;
        locals.var_t5__blk776_dn17 = assign25750_e35063_d_n17;
        locals.var_t5__blk776_rv = 0.0;

        let (assign25760_e35081, assign25760_e35081_d_n0, assign25760_e35081_d_n2, assign25760_e35081_d_n6, assign25760_e35081_d_n7, assign25760_e35081_d_n10, assign25760_e35081_d_n11, assign25760_e35081_d_n12, assign25760_e35081_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard859 != 0.0)) {
        let assign25760_e35075: f64 = (-locals.var_tq);
        let assign25760_e35077: f64 = (assign25760_e35075 + locals.var_t5__blk776);
        let assign25760_e35079: f64 = (assign25760_e35077).powf(0.3333333333333333);
        (assign25760_e35079, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign25760_e35077).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn0) + locals.var_t5__blk776_dn0))) } } else { (assign25760_e35079 * (0.3333333333333333 * (((-locals.var_tq_dn0) + locals.var_t5__blk776_dn0) / assign25760_e35077))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign25760_e35077).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn2) + locals.var_t5__blk776_dn2))) } } else { (assign25760_e35079 * (0.3333333333333333 * (((-locals.var_tq_dn2) + locals.var_t5__blk776_dn2) / assign25760_e35077))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign25760_e35077).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn6) + locals.var_t5__blk776_dn6))) } } else { (assign25760_e35079 * (0.3333333333333333 * (((-locals.var_tq_dn6) + locals.var_t5__blk776_dn6) / assign25760_e35077))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign25760_e35077).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn7) + locals.var_t5__blk776_dn7))) } } else { (assign25760_e35079 * (0.3333333333333333 * (((-locals.var_tq_dn7) + locals.var_t5__blk776_dn7) / assign25760_e35077))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign25760_e35077).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn10) + locals.var_t5__blk776_dn10))) } } else { (assign25760_e35079 * (0.3333333333333333 * (((-locals.var_tq_dn10) + locals.var_t5__blk776_dn10) / assign25760_e35077))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign25760_e35077).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn11) + locals.var_t5__blk776_dn11))) } } else { (assign25760_e35079 * (0.3333333333333333 * (((-locals.var_tq_dn11) + locals.var_t5__blk776_dn11) / assign25760_e35077))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign25760_e35077).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn12) + locals.var_t5__blk776_dn12))) } } else { (assign25760_e35079 * (0.3333333333333333 * (((-locals.var_tq_dn12) + locals.var_t5__blk776_dn12) / assign25760_e35077))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign25760_e35077).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn17) + locals.var_t5__blk776_dn17))) } } else { (assign25760_e35079 * (0.3333333333333333 * (((-locals.var_tq_dn17) + locals.var_t5__blk776_dn17) / assign25760_e35077))) },)
    } else {
        (locals.var_tu, locals.var_tu_dn0, locals.var_tu_dn2, locals.var_tu_dn6, locals.var_tu_dn7, locals.var_tu_dn10, locals.var_tu_dn11, locals.var_tu_dn12, locals.var_tu_dn17,)
    }
};
        locals.var_tu = assign25760_e35081;
        locals.var_tu_dn0 = assign25760_e35081_d_n0;
        locals.var_tu_dn2 = assign25760_e35081_d_n2;
        locals.var_tu_dn6 = assign25760_e35081_d_n6;
        locals.var_tu_dn7 = assign25760_e35081_d_n7;
        locals.var_tu_dn10 = assign25760_e35081_d_n10;
        locals.var_tu_dn11 = assign25760_e35081_d_n11;
        locals.var_tu_dn12 = assign25760_e35081_d_n12;
        locals.var_tu_dn17 = assign25760_e35081_d_n17;
        locals.var_tu_rv = 0.0;

        let (assign25770_e35099, assign25770_e35099_d_n0, assign25770_e35099_d_n2, assign25770_e35099_d_n6, assign25770_e35099_d_n7, assign25770_e35099_d_n10, assign25770_e35099_d_n11, assign25770_e35099_d_n12, assign25770_e35099_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard859 != 0.0)) {
        let assign25770_e35094: f64 = (locals.var_tq + locals.var_t5__blk776);
        let assign25770_e35096: f64 = (assign25770_e35094).powf(0.3333333333333333);
        let assign25770_e35097: f64 = (-assign25770_e35096);
        (assign25770_e35097, (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign25770_e35094).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn0 + locals.var_t5__blk776_dn0))) } } else { (assign25770_e35096 * (0.3333333333333333 * ((locals.var_tq_dn0 + locals.var_t5__blk776_dn0) / assign25770_e35094))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign25770_e35094).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn2 + locals.var_t5__blk776_dn2))) } } else { (assign25770_e35096 * (0.3333333333333333 * ((locals.var_tq_dn2 + locals.var_t5__blk776_dn2) / assign25770_e35094))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign25770_e35094).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn6 + locals.var_t5__blk776_dn6))) } } else { (assign25770_e35096 * (0.3333333333333333 * ((locals.var_tq_dn6 + locals.var_t5__blk776_dn6) / assign25770_e35094))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign25770_e35094).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn7 + locals.var_t5__blk776_dn7))) } } else { (assign25770_e35096 * (0.3333333333333333 * ((locals.var_tq_dn7 + locals.var_t5__blk776_dn7) / assign25770_e35094))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign25770_e35094).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn10 + locals.var_t5__blk776_dn10))) } } else { (assign25770_e35096 * (0.3333333333333333 * ((locals.var_tq_dn10 + locals.var_t5__blk776_dn10) / assign25770_e35094))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign25770_e35094).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn11 + locals.var_t5__blk776_dn11))) } } else { (assign25770_e35096 * (0.3333333333333333 * ((locals.var_tq_dn11 + locals.var_t5__blk776_dn11) / assign25770_e35094))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign25770_e35094).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn12 + locals.var_t5__blk776_dn12))) } } else { (assign25770_e35096 * (0.3333333333333333 * ((locals.var_tq_dn12 + locals.var_t5__blk776_dn12) / assign25770_e35094))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign25770_e35094).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn17 + locals.var_t5__blk776_dn17))) } } else { (assign25770_e35096 * (0.3333333333333333 * ((locals.var_tq_dn17 + locals.var_t5__blk776_dn17) / assign25770_e35094))) }),)
    } else {
        (locals.var_tv, locals.var_tv_dn0, locals.var_tv_dn2, locals.var_tv_dn6, locals.var_tv_dn7, locals.var_tv_dn10, locals.var_tv_dn11, locals.var_tv_dn12, locals.var_tv_dn17,)
    }
};
        locals.var_tv = assign25770_e35099;
        locals.var_tv_dn0 = assign25770_e35099_d_n0;
        locals.var_tv_dn2 = assign25770_e35099_d_n2;
        locals.var_tv_dn6 = assign25770_e35099_d_n6;
        locals.var_tv_dn7 = assign25770_e35099_d_n7;
        locals.var_tv_dn10 = assign25770_e35099_d_n10;
        locals.var_tv_dn11 = assign25770_e35099_d_n11;
        locals.var_tv_dn12 = assign25770_e35099_d_n12;
        locals.var_tv_dn17 = assign25770_e35099_d_n17;
        locals.var_tv_rv = 0.0;

        let (assign25780_e35120, assign25780_e35120_d_n0, assign25780_e35120_d_n2, assign25780_e35120_d_n6, assign25780_e35120_d_n7, assign25780_e35120_d_n10, assign25780_e35120_d_n11, assign25780_e35120_d_n12, assign25780_e35120_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard859 != 0.0)) {
        let assign25780_e35112: f64 = (locals.var_tu + locals.var_tv);
        let assign25780_e35116: f64 = (3.0 * locals.var_ta);
        let assign25780_e35117: f64 = (locals.var_tb / assign25780_e35116);
        let assign25780_e35118: f64 = (assign25780_e35112 - assign25780_e35117);
        (assign25780_e35118, (locals.var_tu_dn0 + locals.var_tv_dn0), (locals.var_tu_dn2 + locals.var_tv_dn2), (locals.var_tu_dn6 + locals.var_tv_dn6), (locals.var_tu_dn7 + locals.var_tv_dn7), (locals.var_tu_dn10 + locals.var_tv_dn10), (locals.var_tu_dn11 + locals.var_tv_dn11), (locals.var_tu_dn12 + locals.var_tv_dn12), (locals.var_tu_dn17 + locals.var_tv_dn17),)
    } else {
        (locals.var_tx__blk779, locals.var_tx__blk779_dn0, locals.var_tx__blk779_dn2, locals.var_tx__blk779_dn6, locals.var_tx__blk779_dn7, locals.var_tx__blk779_dn10, locals.var_tx__blk779_dn11, locals.var_tx__blk779_dn12, locals.var_tx__blk779_dn17,)
    }
};
        locals.var_tx__blk779 = assign25780_e35120;
        locals.var_tx__blk779_dn0 = assign25780_e35120_d_n0;
        locals.var_tx__blk779_dn2 = assign25780_e35120_d_n2;
        locals.var_tx__blk779_dn6 = assign25780_e35120_d_n6;
        locals.var_tx__blk779_dn7 = assign25780_e35120_d_n7;
        locals.var_tx__blk779_dn10 = assign25780_e35120_d_n10;
        locals.var_tx__blk779_dn11 = assign25780_e35120_d_n11;
        locals.var_tx__blk779_dn12 = assign25780_e35120_d_n12;
        locals.var_tx__blk779_dn17 = assign25780_e35120_d_n17;
        locals.var_tx__blk779_rv = 0.0;

        let (assign25790_e35137, assign25790_e35137_d_n0, assign25790_e35137_d_n2, assign25790_e35137_d_n6, assign25790_e35137_d_n7, assign25790_e35137_d_n10, assign25790_e35137_d_n11, assign25790_e35137_d_n12, assign25790_e35137_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard859 != 0.0)) {
        let assign25790_e35133: f64 = (locals.var_tx__blk779 * locals.var_beta_inv);
        let assign25790_e35135: f64 = (assign25790_e35133 - locals.var_vxbgmtcl);
        (assign25790_e35135, ((locals.var_tx__blk779_dn0 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn0), ((locals.var_tx__blk779_dn2 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn2), ((locals.var_tx__blk779_dn6 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn6), ((locals.var_tx__blk779_dn7 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn7), (((locals.var_tx__blk779_dn10 * locals.var_beta_inv) + (locals.var_tx__blk779 * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), ((locals.var_tx__blk779_dn11 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn11), ((locals.var_tx__blk779_dn12 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn12), ((locals.var_tx__blk779_dn17 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_ps0_inia__blk819, locals.var_ps0_inia__blk819_dn0, locals.var_ps0_inia__blk819_dn2, locals.var_ps0_inia__blk819_dn6, locals.var_ps0_inia__blk819_dn7, locals.var_ps0_inia__blk819_dn10, locals.var_ps0_inia__blk819_dn11, locals.var_ps0_inia__blk819_dn12, locals.var_ps0_inia__blk819_dn17,)
    }
};
        locals.var_ps0_inia__blk819 = assign25790_e35137;
        locals.var_ps0_inia__blk819_dn0 = assign25790_e35137_d_n0;
        locals.var_ps0_inia__blk819_dn2 = assign25790_e35137_d_n2;
        locals.var_ps0_inia__blk819_dn6 = assign25790_e35137_d_n6;
        locals.var_ps0_inia__blk819_dn7 = assign25790_e35137_d_n7;
        locals.var_ps0_inia__blk819_dn10 = assign25790_e35137_d_n10;
        locals.var_ps0_inia__blk819_dn11 = assign25790_e35137_d_n11;
        locals.var_ps0_inia__blk819_dn12 = assign25790_e35137_d_n12;
        locals.var_ps0_inia__blk819_dn17 = assign25790_e35137_d_n17;
        locals.var_ps0_inia__blk819_rv = 0.0;

        let (assign25800_e35154, assign25800_e35154_d_n0, assign25800_e35154_d_n2, assign25800_e35154_d_n6, assign25800_e35154_d_n7, assign25800_e35154_d_n10, assign25800_e35154_d_n11, assign25800_e35154_d_n12, assign25800_e35154_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard859 != 0.0)) {
        let assign25800_e35151: f64 = (locals.var_ps0_inia__blk819 + locals.var_vxbgmtcl);
        let assign25800_e35152: f64 = (locals.var_beta * assign25800_e35151);
        (assign25800_e35152, (locals.var_beta * (locals.var_ps0_inia__blk819_dn0 + locals.var_vxbgmtcl_dn0)), (locals.var_beta * (locals.var_ps0_inia__blk819_dn2 + locals.var_vxbgmtcl_dn2)), (locals.var_beta * (locals.var_ps0_inia__blk819_dn6 + locals.var_vxbgmtcl_dn6)), (locals.var_beta * (locals.var_ps0_inia__blk819_dn7 + locals.var_vxbgmtcl_dn7)), ((locals.var_beta_dn10 * assign25800_e35151) + (locals.var_beta * (locals.var_ps0_inia__blk819_dn10 + locals.var_vxbgmtcl_dn10))), (locals.var_beta * (locals.var_ps0_inia__blk819_dn11 + locals.var_vxbgmtcl_dn11)), (locals.var_beta * (locals.var_ps0_inia__blk819_dn12 + locals.var_vxbgmtcl_dn12)), (locals.var_beta * (locals.var_ps0_inia__blk819_dn17 + locals.var_vxbgmtcl_dn17)),)
    } else {
        (locals.var_chi__blk816, locals.var_chi__blk816_dn0, locals.var_chi__blk816_dn2, locals.var_chi__blk816_dn6, locals.var_chi__blk816_dn7, locals.var_chi__blk816_dn10, locals.var_chi__blk816_dn11, locals.var_chi__blk816_dn12, locals.var_chi__blk816_dn17,)
    }
};
        locals.var_chi__blk816 = assign25800_e35154;
        locals.var_chi__blk816_dn0 = assign25800_e35154_d_n0;
        locals.var_chi__blk816_dn2 = assign25800_e35154_d_n2;
        locals.var_chi__blk816_dn6 = assign25800_e35154_d_n6;
        locals.var_chi__blk816_dn7 = assign25800_e35154_d_n7;
        locals.var_chi__blk816_dn10 = assign25800_e35154_d_n10;
        locals.var_chi__blk816_dn11 = assign25800_e35154_d_n11;
        locals.var_chi__blk816_dn12 = assign25800_e35154_d_n12;
        locals.var_chi__blk816_dn17 = assign25800_e35154_d_n17;
        locals.var_chi__blk816_rv = 0.0;

        let (assign25820_e35182, assign25820_e35182_d_n0, assign25820_e35182_d_n2, assign25820_e35182_d_n6, assign25820_e35182_d_n7, assign25820_e35182_d_n10, assign25820_e35182_d_n11, assign25820_e35182_d_n12, assign25820_e35182_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) {
        let assign25820_e35178: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign25820_e35180: f64 = (assign25820_e35178 + 0.1);
        (assign25820_e35180, (locals.var_vgpld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_vgpld_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_vgpld_dn11 + locals.var_vxbgmtcl_dn11), (locals.var_vgpld_dn12 + locals.var_vxbgmtcl_dn12), (locals.var_vgpld_dn17 + locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_vgpld_shift, locals.var_vgpld_shift_dn0, locals.var_vgpld_shift_dn2, locals.var_vgpld_shift_dn6, locals.var_vgpld_shift_dn7, locals.var_vgpld_shift_dn10, locals.var_vgpld_shift_dn11, locals.var_vgpld_shift_dn12, locals.var_vgpld_shift_dn17,)
    }
};
        locals.var_vgpld_shift = assign25820_e35182;
        locals.var_vgpld_shift_dn0 = assign25820_e35182_d_n0;
        locals.var_vgpld_shift_dn2 = assign25820_e35182_d_n2;
        locals.var_vgpld_shift_dn6 = assign25820_e35182_d_n6;
        locals.var_vgpld_shift_dn7 = assign25820_e35182_d_n7;
        locals.var_vgpld_shift_dn10 = assign25820_e35182_d_n10;
        locals.var_vgpld_shift_dn11 = assign25820_e35182_d_n11;
        locals.var_vgpld_shift_dn12 = assign25820_e35182_d_n12;
        locals.var_vgpld_shift_dn17 = assign25820_e35182_d_n17;
        locals.var_vgpld_shift_rv = 0.0;

        let (assign25830_e35199, assign25830_e35199_d_n0, assign25830_e35199_d_n2, assign25830_e35199_d_n6, assign25830_e35199_d_n7, assign25830_e35199_d_n10, assign25830_e35199_d_n11, assign25830_e35199_d_n12, assign25830_e35199_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) {
        let assign25830_e35193: f64 = (-locals.var_vxbgmtcl);
        let assign25830_e35194: f64 = (locals.var_beta * assign25830_e35193);
        let assign25830_e35195: f64 = (assign25830_e35194).exp();
        let assign25830_e35197: f64 = (assign25830_e35195 + 1e-50);
        (assign25830_e35197, (assign25830_e35195 * (locals.var_beta * (-locals.var_vxbgmtcl_dn0))), (assign25830_e35195 * (locals.var_beta * (-locals.var_vxbgmtcl_dn2))), (assign25830_e35195 * (locals.var_beta * (-locals.var_vxbgmtcl_dn6))), (assign25830_e35195 * (locals.var_beta * (-locals.var_vxbgmtcl_dn7))), (assign25830_e35195 * ((locals.var_beta_dn10 * assign25830_e35193) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10)))), (assign25830_e35195 * (locals.var_beta * (-locals.var_vxbgmtcl_dn11))), (assign25830_e35195 * (locals.var_beta * (-locals.var_vxbgmtcl_dn12))), (assign25830_e35195 * (locals.var_beta * (-locals.var_vxbgmtcl_dn17))),)
    } else {
        (locals.var_exp_bvbs__blk835, locals.var_exp_bvbs__blk835_dn0, locals.var_exp_bvbs__blk835_dn2, locals.var_exp_bvbs__blk835_dn6, locals.var_exp_bvbs__blk835_dn7, locals.var_exp_bvbs__blk835_dn10, locals.var_exp_bvbs__blk835_dn11, locals.var_exp_bvbs__blk835_dn12, locals.var_exp_bvbs__blk835_dn17,)
    }
};
        locals.var_exp_bvbs__blk835 = assign25830_e35199;
        locals.var_exp_bvbs__blk835_dn0 = assign25830_e35199_d_n0;
        locals.var_exp_bvbs__blk835_dn2 = assign25830_e35199_d_n2;
        locals.var_exp_bvbs__blk835_dn6 = assign25830_e35199_d_n6;
        locals.var_exp_bvbs__blk835_dn7 = assign25830_e35199_d_n7;
        locals.var_exp_bvbs__blk835_dn10 = assign25830_e35199_d_n10;
        locals.var_exp_bvbs__blk835_dn11 = assign25830_e35199_d_n11;
        locals.var_exp_bvbs__blk835_dn12 = assign25830_e35199_d_n12;
        locals.var_exp_bvbs__blk835_dn17 = assign25830_e35199_d_n17;
        locals.var_exp_bvbs__blk835_rv = 0.0;

        let (assign25840_e35212, assign25840_e35212_d_n0, assign25840_e35212_d_n2, assign25840_e35212_d_n6, assign25840_e35212_d_n7, assign25840_e35212_d_n10, assign25840_e35212_d_n11, assign25840_e35212_d_n12, assign25840_e35212_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) {
        let assign25840_e35210: f64 = (locals.var_nin / locals.var_uc_nsubbttub);
        (assign25840_e35210, (((locals.var_nin_dn0 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn0)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn2 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn2)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn6 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn6)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn7 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn7)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn10 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn10)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn11 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn11)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn12 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn12)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn17 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn17)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)),)
    } else {
        (locals.var_t0__blk772, locals.var_t0__blk772_dn0, locals.var_t0__blk772_dn2, locals.var_t0__blk772_dn6, locals.var_t0__blk772_dn7, locals.var_t0__blk772_dn10, locals.var_t0__blk772_dn11, locals.var_t0__blk772_dn12, locals.var_t0__blk772_dn17,)
    }
};
        locals.var_t0__blk772 = assign25840_e35212;
        locals.var_t0__blk772_dn0 = assign25840_e35212_d_n0;
        locals.var_t0__blk772_dn2 = assign25840_e35212_d_n2;
        locals.var_t0__blk772_dn6 = assign25840_e35212_d_n6;
        locals.var_t0__blk772_dn7 = assign25840_e35212_d_n7;
        locals.var_t0__blk772_dn10 = assign25840_e35212_d_n10;
        locals.var_t0__blk772_dn11 = assign25840_e35212_d_n11;
        locals.var_t0__blk772_dn12 = assign25840_e35212_d_n12;
        locals.var_t0__blk772_dn17 = assign25840_e35212_d_n17;
        locals.var_t0__blk772_rv = 0.0;

        let (assign25850_e35225, assign25850_e35225_d_n0, assign25850_e35225_d_n2, assign25850_e35225_d_n6, assign25850_e35225_d_n7, assign25850_e35225_d_n10, assign25850_e35225_d_n11, assign25850_e35225_d_n12, assign25850_e35225_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) {
        let assign25850_e35223: f64 = (locals.var_t0__blk772 * locals.var_t0__blk772);
        (assign25850_e35223, ((locals.var_t0__blk772_dn0 * locals.var_t0__blk772) + (locals.var_t0__blk772 * locals.var_t0__blk772_dn0)), ((locals.var_t0__blk772_dn2 * locals.var_t0__blk772) + (locals.var_t0__blk772 * locals.var_t0__blk772_dn2)), ((locals.var_t0__blk772_dn6 * locals.var_t0__blk772) + (locals.var_t0__blk772 * locals.var_t0__blk772_dn6)), ((locals.var_t0__blk772_dn7 * locals.var_t0__blk772) + (locals.var_t0__blk772 * locals.var_t0__blk772_dn7)), ((locals.var_t0__blk772_dn10 * locals.var_t0__blk772) + (locals.var_t0__blk772 * locals.var_t0__blk772_dn10)), ((locals.var_t0__blk772_dn11 * locals.var_t0__blk772) + (locals.var_t0__blk772 * locals.var_t0__blk772_dn11)), ((locals.var_t0__blk772_dn12 * locals.var_t0__blk772) + (locals.var_t0__blk772 * locals.var_t0__blk772_dn12)), ((locals.var_t0__blk772_dn17 * locals.var_t0__blk772) + (locals.var_t0__blk772 * locals.var_t0__blk772_dn17)),)
    } else {
        (locals.var_cnst1over, locals.var_cnst1over_dn0, locals.var_cnst1over_dn2, locals.var_cnst1over_dn6, locals.var_cnst1over_dn7, locals.var_cnst1over_dn10, locals.var_cnst1over_dn11, locals.var_cnst1over_dn12, locals.var_cnst1over_dn17,)
    }
};
        locals.var_cnst1over = assign25850_e35225;
        locals.var_cnst1over_dn0 = assign25850_e35225_d_n0;
        locals.var_cnst1over_dn2 = assign25850_e35225_d_n2;
        locals.var_cnst1over_dn6 = assign25850_e35225_d_n6;
        locals.var_cnst1over_dn7 = assign25850_e35225_d_n7;
        locals.var_cnst1over_dn10 = assign25850_e35225_d_n10;
        locals.var_cnst1over_dn11 = assign25850_e35225_d_n11;
        locals.var_cnst1over_dn12 = assign25850_e35225_d_n12;
        locals.var_cnst1over_dn17 = assign25850_e35225_d_n17;
        locals.var_cnst1over_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_91(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign25860_e35238, assign25860_e35238_d_n0, assign25860_e35238_d_n2, assign25860_e35238_d_n6, assign25860_e35238_d_n7, assign25860_e35238_d_n10, assign25860_e35238_d_n11, assign25860_e35238_d_n12, assign25860_e35238_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) {
        let assign25860_e35236: f64 = (locals.var_cnst1over * locals.var_exp_bvbs__blk835);
        (assign25860_e35236, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs__blk835) + (locals.var_cnst1over * locals.var_exp_bvbs__blk835_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs__blk835) + (locals.var_cnst1over * locals.var_exp_bvbs__blk835_dn2)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs__blk835) + (locals.var_cnst1over * locals.var_exp_bvbs__blk835_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs__blk835) + (locals.var_cnst1over * locals.var_exp_bvbs__blk835_dn7)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs__blk835) + (locals.var_cnst1over * locals.var_exp_bvbs__blk835_dn10)), ((locals.var_cnst1over_dn11 * locals.var_exp_bvbs__blk835) + (locals.var_cnst1over * locals.var_exp_bvbs__blk835_dn11)), ((locals.var_cnst1over_dn12 * locals.var_exp_bvbs__blk835) + (locals.var_cnst1over * locals.var_exp_bvbs__blk835_dn12)), ((locals.var_cnst1over_dn17 * locals.var_exp_bvbs__blk835) + (locals.var_cnst1over * locals.var_exp_bvbs__blk835_dn17)),)
    } else {
        (locals.var_gammachi, locals.var_gammachi_dn0, locals.var_gammachi_dn2, locals.var_gammachi_dn6, locals.var_gammachi_dn7, locals.var_gammachi_dn10, locals.var_gammachi_dn11, locals.var_gammachi_dn12, locals.var_gammachi_dn17,)
    }
};
        locals.var_gammachi = assign25860_e35238;
        locals.var_gammachi_dn0 = assign25860_e35238_d_n0;
        locals.var_gammachi_dn2 = assign25860_e35238_d_n2;
        locals.var_gammachi_dn6 = assign25860_e35238_d_n6;
        locals.var_gammachi_dn7 = assign25860_e35238_d_n7;
        locals.var_gammachi_dn10 = assign25860_e35238_d_n10;
        locals.var_gammachi_dn11 = assign25860_e35238_d_n11;
        locals.var_gammachi_dn12 = assign25860_e35238_d_n12;
        locals.var_gammachi_dn17 = assign25860_e35238_d_n17;
        locals.var_gammachi_rv = 0.0;

        let (assign25870_e35251, assign25870_e35251_d_n0, assign25870_e35251_d_n2, assign25870_e35251_d_n6, assign25870_e35251_d_n7, assign25870_e35251_d_n10, assign25870_e35251_d_n11, assign25870_e35251_d_n12, assign25870_e35251_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) {
        let assign25870_e35249: f64 = (locals.var_beta2 * locals.var_fac1p2__blk803);
        (assign25870_e35249, (locals.var_beta2 * locals.var_fac1p2__blk803_dn0), (locals.var_beta2 * locals.var_fac1p2__blk803_dn2), (locals.var_beta2 * locals.var_fac1p2__blk803_dn6), (locals.var_beta2 * locals.var_fac1p2__blk803_dn7), ((locals.var_beta2_dn10 * locals.var_fac1p2__blk803) + (locals.var_beta2 * locals.var_fac1p2__blk803_dn10)), (locals.var_beta2 * locals.var_fac1p2__blk803_dn11), (locals.var_beta2 * locals.var_fac1p2__blk803_dn12), (locals.var_beta2 * locals.var_fac1p2__blk803_dn17),)
    } else {
        (locals.var_t0__blk772, locals.var_t0__blk772_dn0, locals.var_t0__blk772_dn2, locals.var_t0__blk772_dn6, locals.var_t0__blk772_dn7, locals.var_t0__blk772_dn10, locals.var_t0__blk772_dn11, locals.var_t0__blk772_dn12, locals.var_t0__blk772_dn17,)
    }
};
        locals.var_t0__blk772 = assign25870_e35251;
        locals.var_t0__blk772_dn0 = assign25870_e35251_d_n0;
        locals.var_t0__blk772_dn2 = assign25870_e35251_d_n2;
        locals.var_t0__blk772_dn6 = assign25870_e35251_d_n6;
        locals.var_t0__blk772_dn7 = assign25870_e35251_d_n7;
        locals.var_t0__blk772_dn10 = assign25870_e35251_d_n10;
        locals.var_t0__blk772_dn11 = assign25870_e35251_d_n11;
        locals.var_t0__blk772_dn12 = assign25870_e35251_d_n12;
        locals.var_t0__blk772_dn17 = assign25870_e35251_d_n17;
        locals.var_t0__blk772_rv = 0.0;

        let (assign25880_e35264, assign25880_e35264_d_n0, assign25880_e35264_d_n2, assign25880_e35264_d_n6, assign25880_e35264_d_n7, assign25880_e35264_d_n10, assign25880_e35264_d_n11, assign25880_e35264_d_n12, assign25880_e35264_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) {
        let assign25880_e35262: f64 = (locals.var_beta * locals.var_vgpld_shift);
        (assign25880_e35262, (locals.var_beta * locals.var_vgpld_shift_dn0), (locals.var_beta * locals.var_vgpld_shift_dn2), (locals.var_beta * locals.var_vgpld_shift_dn6), (locals.var_beta * locals.var_vgpld_shift_dn7), ((locals.var_beta_dn10 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn10)), (locals.var_beta * locals.var_vgpld_shift_dn11), (locals.var_beta * locals.var_vgpld_shift_dn12), (locals.var_beta * locals.var_vgpld_shift_dn17),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn12, locals.var_psi_dn17,)
    }
};
        locals.var_psi = assign25880_e35264;
        locals.var_psi_dn0 = assign25880_e35264_d_n0;
        locals.var_psi_dn2 = assign25880_e35264_d_n2;
        locals.var_psi_dn6 = assign25880_e35264_d_n6;
        locals.var_psi_dn7 = assign25880_e35264_d_n7;
        locals.var_psi_dn10 = assign25880_e35264_d_n10;
        locals.var_psi_dn11 = assign25880_e35264_d_n11;
        locals.var_psi_dn12 = assign25880_e35264_d_n12;
        locals.var_psi_dn17 = assign25880_e35264_d_n17;
        locals.var_psi_rv = 0.0;

        let (assign25890_e35291, assign25890_e35291_d_n0, assign25890_e35291_d_n2, assign25890_e35291_d_n6, assign25890_e35291_d_n7, assign25890_e35291_d_n10, assign25890_e35291_d_n11, assign25890_e35291_d_n12, assign25890_e35291_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) {
        let assign25890_e35275: f64 = (locals.var_gammachi * locals.var_t0__blk772);
        let assign25890_e35278: f64 = (locals.var_psi * locals.var_psi);
        let assign25890_e35279: f64 = (assign25890_e35275 + assign25890_e35278);
        let assign25890_e35280: f64 = (assign25890_e35279).ln();
        let assign25890_e35283: f64 = (locals.var_cnst1over * locals.var_t0__blk772);
        let assign25890_e35284: f64 = (assign25890_e35283).ln();
        let assign25890_e35285: f64 = (assign25890_e35280 - assign25890_e35284);
        let assign25890_e35288: f64 = (locals.var_beta * locals.var_vxbgmtcl);
        let assign25890_e35289: f64 = (assign25890_e35285 + assign25890_e35288);
        (assign25890_e35289, ((((((locals.var_gammachi_dn0 * locals.var_t0__blk772) + (locals.var_gammachi * locals.var_t0__blk772_dn0)) + ((locals.var_psi_dn0 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn0))) / assign25890_e35279) - (((locals.var_cnst1over_dn0 * locals.var_t0__blk772) + (locals.var_cnst1over * locals.var_t0__blk772_dn0)) / assign25890_e35283)) + (locals.var_beta * locals.var_vxbgmtcl_dn0)), ((((((locals.var_gammachi_dn2 * locals.var_t0__blk772) + (locals.var_gammachi * locals.var_t0__blk772_dn2)) + ((locals.var_psi_dn2 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn2))) / assign25890_e35279) - (((locals.var_cnst1over_dn2 * locals.var_t0__blk772) + (locals.var_cnst1over * locals.var_t0__blk772_dn2)) / assign25890_e35283)) + (locals.var_beta * locals.var_vxbgmtcl_dn2)), ((((((locals.var_gammachi_dn6 * locals.var_t0__blk772) + (locals.var_gammachi * locals.var_t0__blk772_dn6)) + ((locals.var_psi_dn6 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn6))) / assign25890_e35279) - (((locals.var_cnst1over_dn6 * locals.var_t0__blk772) + (locals.var_cnst1over * locals.var_t0__blk772_dn6)) / assign25890_e35283)) + (locals.var_beta * locals.var_vxbgmtcl_dn6)), ((((((locals.var_gammachi_dn7 * locals.var_t0__blk772) + (locals.var_gammachi * locals.var_t0__blk772_dn7)) + ((locals.var_psi_dn7 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn7))) / assign25890_e35279) - (((locals.var_cnst1over_dn7 * locals.var_t0__blk772) + (locals.var_cnst1over * locals.var_t0__blk772_dn7)) / assign25890_e35283)) + (locals.var_beta * locals.var_vxbgmtcl_dn7)), ((((((locals.var_gammachi_dn10 * locals.var_t0__blk772) + (locals.var_gammachi * locals.var_t0__blk772_dn10)) + ((locals.var_psi_dn10 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn10))) / assign25890_e35279) - (((locals.var_cnst1over_dn10 * locals.var_t0__blk772) + (locals.var_cnst1over * locals.var_t0__blk772_dn10)) / assign25890_e35283)) + ((locals.var_beta_dn10 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn10))), ((((((locals.var_gammachi_dn11 * locals.var_t0__blk772) + (locals.var_gammachi * locals.var_t0__blk772_dn11)) + ((locals.var_psi_dn11 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn11))) / assign25890_e35279) - (((locals.var_cnst1over_dn11 * locals.var_t0__blk772) + (locals.var_cnst1over * locals.var_t0__blk772_dn11)) / assign25890_e35283)) + (locals.var_beta * locals.var_vxbgmtcl_dn11)), ((((((locals.var_gammachi_dn12 * locals.var_t0__blk772) + (locals.var_gammachi * locals.var_t0__blk772_dn12)) + ((locals.var_psi_dn12 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn12))) / assign25890_e35279) - (((locals.var_cnst1over_dn12 * locals.var_t0__blk772) + (locals.var_cnst1over * locals.var_t0__blk772_dn12)) / assign25890_e35283)) + (locals.var_beta * locals.var_vxbgmtcl_dn12)), ((((((locals.var_gammachi_dn17 * locals.var_t0__blk772) + (locals.var_gammachi * locals.var_t0__blk772_dn17)) + ((locals.var_psi_dn17 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn17))) / assign25890_e35279) - (((locals.var_cnst1over_dn17 * locals.var_t0__blk772) + (locals.var_cnst1over * locals.var_t0__blk772_dn17)) / assign25890_e35283)) + (locals.var_beta * locals.var_vxbgmtcl_dn17)),)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn12, locals.var_chi_1_dn17,)
    }
};
        locals.var_chi_1 = assign25890_e35291;
        locals.var_chi_1_dn0 = assign25890_e35291_d_n0;
        locals.var_chi_1_dn2 = assign25890_e35291_d_n2;
        locals.var_chi_1_dn6 = assign25890_e35291_d_n6;
        locals.var_chi_1_dn7 = assign25890_e35291_d_n7;
        locals.var_chi_1_dn10 = assign25890_e35291_d_n10;
        locals.var_chi_1_dn11 = assign25890_e35291_d_n11;
        locals.var_chi_1_dn12 = assign25890_e35291_d_n12;
        locals.var_chi_1_dn17 = assign25890_e35291_d_n17;
        locals.var_chi_1_rv = 0.0;

        let (assign25900_e35306, assign25900_e35306_d_n0, assign25900_e35306_d_n2, assign25900_e35306_d_n6, assign25900_e35306_d_n7, assign25900_e35306_d_n10, assign25900_e35306_d_n11, assign25900_e35306_d_n12, assign25900_e35306_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) {
        let assign25900_e35302: f64 = (locals.var_psi - locals.var_chi_1);
        let assign25900_e35304: f64 = (assign25900_e35302 - 1.0);
        (assign25900_e35304, (locals.var_psi_dn0 - locals.var_chi_1_dn0), (locals.var_psi_dn2 - locals.var_chi_1_dn2), (locals.var_psi_dn6 - locals.var_chi_1_dn6), (locals.var_psi_dn7 - locals.var_chi_1_dn7), (locals.var_psi_dn10 - locals.var_chi_1_dn10), (locals.var_psi_dn11 - locals.var_chi_1_dn11), (locals.var_psi_dn12 - locals.var_chi_1_dn12), (locals.var_psi_dn17 - locals.var_chi_1_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign25900_e35306;
        locals.var_tmf1_dn0 = assign25900_e35306_d_n0;
        locals.var_tmf1_dn2 = assign25900_e35306_d_n2;
        locals.var_tmf1_dn6 = assign25900_e35306_d_n6;
        locals.var_tmf1_dn7 = assign25900_e35306_d_n7;
        locals.var_tmf1_dn10 = assign25900_e35306_d_n10;
        locals.var_tmf1_dn11 = assign25900_e35306_d_n11;
        locals.var_tmf1_dn12 = assign25900_e35306_d_n12;
        locals.var_tmf1_dn17 = assign25900_e35306_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign25910_e35321, assign25910_e35321_d_n0, assign25910_e35321_d_n2, assign25910_e35321_d_n6, assign25910_e35321_d_n7, assign25910_e35321_d_n10, assign25910_e35321_d_n11, assign25910_e35321_d_n12, assign25910_e35321_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) {
        let assign25910_e35317: f64 = (4.0 * locals.var_psi);
        let assign25910_e35319: f64 = assign25910_e35317;
        (assign25910_e35319, (4.0 * locals.var_psi_dn0), (4.0 * locals.var_psi_dn2), (4.0 * locals.var_psi_dn6), (4.0 * locals.var_psi_dn7), (4.0 * locals.var_psi_dn10), (4.0 * locals.var_psi_dn11), (4.0 * locals.var_psi_dn12), (4.0 * locals.var_psi_dn17),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign25910_e35321;
        locals.var_tmf2_dn0 = assign25910_e35321_d_n0;
        locals.var_tmf2_dn2 = assign25910_e35321_d_n2;
        locals.var_tmf2_dn6 = assign25910_e35321_d_n6;
        locals.var_tmf2_dn7 = assign25910_e35321_d_n7;
        locals.var_tmf2_dn10 = assign25910_e35321_d_n10;
        locals.var_tmf2_dn11 = assign25910_e35321_d_n11;
        locals.var_tmf2_dn12 = assign25910_e35321_d_n12;
        locals.var_tmf2_dn17 = assign25910_e35321_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign25920_e35338, assign25920_e35338_d_n0, assign25920_e35338_d_n2, assign25920_e35338_d_n6, assign25920_e35338_d_n7, assign25920_e35338_d_n10, assign25920_e35338_d_n11, assign25920_e35338_d_n12, assign25920_e35338_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) {
        let (assign25920_e35336, assign25920_e35336_d_n0, assign25920_e35336_d_n2, assign25920_e35336_d_n6, assign25920_e35336_d_n7, assign25920_e35336_d_n10, assign25920_e35336_d_n11, assign25920_e35336_d_n12, assign25920_e35336_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign25920_e35335: f64 = (-locals.var_tmf2);
                (assign25920_e35335, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign25920_e35336, assign25920_e35336_d_n0, assign25920_e35336_d_n2, assign25920_e35336_d_n6, assign25920_e35336_d_n7, assign25920_e35336_d_n10, assign25920_e35336_d_n11, assign25920_e35336_d_n12, assign25920_e35336_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign25920_e35338;
        locals.var_tmf2_dn0 = assign25920_e35338_d_n0;
        locals.var_tmf2_dn2 = assign25920_e35338_d_n2;
        locals.var_tmf2_dn6 = assign25920_e35338_d_n6;
        locals.var_tmf2_dn7 = assign25920_e35338_d_n7;
        locals.var_tmf2_dn10 = assign25920_e35338_d_n10;
        locals.var_tmf2_dn11 = assign25920_e35338_d_n11;
        locals.var_tmf2_dn12 = assign25920_e35338_d_n12;
        locals.var_tmf2_dn17 = assign25920_e35338_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign25930_e35354, assign25930_e35354_d_n0, assign25930_e35354_d_n2, assign25930_e35354_d_n6, assign25930_e35354_d_n7, assign25930_e35354_d_n10, assign25930_e35354_d_n11, assign25930_e35354_d_n12, assign25930_e35354_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) {
        let assign25930_e35349: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign25930_e35351: f64 = (assign25930_e35349 + locals.var_tmf2);
        let assign25930_e35352: f64 = (assign25930_e35351).sqrt();
        (assign25930_e35352, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign25930_e35352)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign25930_e35352)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign25930_e35352)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign25930_e35352)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign25930_e35352)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign25930_e35352)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign25930_e35352)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign25930_e35352)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign25930_e35354;
        locals.var_tmf2_dn0 = assign25930_e35354_d_n0;
        locals.var_tmf2_dn2 = assign25930_e35354_d_n2;
        locals.var_tmf2_dn6 = assign25930_e35354_d_n6;
        locals.var_tmf2_dn7 = assign25930_e35354_d_n7;
        locals.var_tmf2_dn10 = assign25930_e35354_d_n10;
        locals.var_tmf2_dn11 = assign25930_e35354_d_n11;
        locals.var_tmf2_dn12 = assign25930_e35354_d_n12;
        locals.var_tmf2_dn17 = assign25930_e35354_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign25940_e35371, assign25940_e35371_d_n0, assign25940_e35371_d_n2, assign25940_e35371_d_n6, assign25940_e35371_d_n7, assign25940_e35371_d_n10, assign25940_e35371_d_n11, assign25940_e35371_d_n12, assign25940_e35371_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) {
        let assign25940_e35367: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign25940_e35368: f64 = (1.0 + assign25940_e35367);
        let assign25940_e35369: f64 = (0.5 * assign25940_e35368);
        (assign25940_e35369, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn17 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1__blk773, locals.var_t1__blk773_dn0, locals.var_t1__blk773_dn2, locals.var_t1__blk773_dn6, locals.var_t1__blk773_dn7, locals.var_t1__blk773_dn10, locals.var_t1__blk773_dn11, locals.var_t1__blk773_dn12, locals.var_t1__blk773_dn17,)
    }
};
        locals.var_t1__blk773 = assign25940_e35371;
        locals.var_t1__blk773_dn0 = assign25940_e35371_d_n0;
        locals.var_t1__blk773_dn2 = assign25940_e35371_d_n2;
        locals.var_t1__blk773_dn6 = assign25940_e35371_d_n6;
        locals.var_t1__blk773_dn7 = assign25940_e35371_d_n7;
        locals.var_t1__blk773_dn10 = assign25940_e35371_d_n10;
        locals.var_t1__blk773_dn11 = assign25940_e35371_d_n11;
        locals.var_t1__blk773_dn12 = assign25940_e35371_d_n12;
        locals.var_t1__blk773_dn17 = assign25940_e35371_d_n17;
        locals.var_t1__blk773_rv = 0.0;

        let (assign25950_e35392, assign25950_e35392_d_n0, assign25950_e35392_d_n2, assign25950_e35392_d_n6, assign25950_e35392_d_n7, assign25950_e35392_d_n10, assign25950_e35392_d_n11, assign25950_e35392_d_n12, assign25950_e35392_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) {
        let assign25950_e35385: f64 = 2.0;
        let assign25950_e35386: f64 = (locals.var_tmf1 + assign25950_e35385);
        let assign25950_e35388: f64 = (assign25950_e35386 / locals.var_tmf2);
        let assign25950_e35389: f64 = (1.0 - assign25950_e35388);
        let assign25950_e35390: f64 = (0.5 * assign25950_e35389);
        (assign25950_e35390, (0.5 * (-(((locals.var_tmf1_dn0 * locals.var_tmf2) - (assign25950_e35386 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn2 * locals.var_tmf2) - (assign25950_e35386 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn6 * locals.var_tmf2) - (assign25950_e35386 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn7 * locals.var_tmf2) - (assign25950_e35386 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn10 * locals.var_tmf2) - (assign25950_e35386 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn11 * locals.var_tmf2) - (assign25950_e35386 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn12 * locals.var_tmf2) - (assign25950_e35386 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn17 * locals.var_tmf2) - (assign25950_e35386 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t2__blk774, locals.var_t2__blk774_dn0, locals.var_t2__blk774_dn2, locals.var_t2__blk774_dn6, locals.var_t2__blk774_dn7, locals.var_t2__blk774_dn10, locals.var_t2__blk774_dn11, locals.var_t2__blk774_dn12, locals.var_t2__blk774_dn17,)
    }
};
        locals.var_t2__blk774 = assign25950_e35392;
        locals.var_t2__blk774_dn0 = assign25950_e35392_d_n0;
        locals.var_t2__blk774_dn2 = assign25950_e35392_d_n2;
        locals.var_t2__blk774_dn6 = assign25950_e35392_d_n6;
        locals.var_t2__blk774_dn7 = assign25950_e35392_d_n7;
        locals.var_t2__blk774_dn10 = assign25950_e35392_d_n10;
        locals.var_t2__blk774_dn11 = assign25950_e35392_d_n11;
        locals.var_t2__blk774_dn12 = assign25950_e35392_d_n12;
        locals.var_t2__blk774_dn17 = assign25950_e35392_d_n17;
        locals.var_t2__blk774_rv = 0.0;

        let (assign25960_e35409, assign25960_e35409_d_n0, assign25960_e35409_d_n2, assign25960_e35409_d_n6, assign25960_e35409_d_n7, assign25960_e35409_d_n10, assign25960_e35409_d_n11, assign25960_e35409_d_n12, assign25960_e35409_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) {
        let assign25960_e35405: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign25960_e35406: f64 = (0.5 * assign25960_e35405);
        let assign25960_e35407: f64 = (locals.var_psi - assign25960_e35406);
        (assign25960_e35407, (locals.var_psi_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_psi_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_psi_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_psi_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_psi_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_psi_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_psi_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_psi_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn12, locals.var_chi_1_dn17,)
    }
};
        locals.var_chi_1 = assign25960_e35409;
        locals.var_chi_1_dn0 = assign25960_e35409_d_n0;
        locals.var_chi_1_dn2 = assign25960_e35409_d_n2;
        locals.var_chi_1_dn6 = assign25960_e35409_d_n6;
        locals.var_chi_1_dn7 = assign25960_e35409_d_n7;
        locals.var_chi_1_dn10 = assign25960_e35409_d_n10;
        locals.var_chi_1_dn11 = assign25960_e35409_d_n11;
        locals.var_chi_1_dn12 = assign25960_e35409_d_n12;
        locals.var_chi_1_dn17 = assign25960_e35409_d_n17;
        locals.var_chi_1_rv = 0.0;

        let (assign25970_e35422, assign25970_e35422_d_n0, assign25970_e35422_d_n2, assign25970_e35422_d_n6, assign25970_e35422_d_n7, assign25970_e35422_d_n10, assign25970_e35422_d_n11, assign25970_e35422_d_n12, assign25970_e35422_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) {
        let assign25970_e35420: f64 = (locals.var_psi - locals.var_chi_1);
        (assign25970_e35420, (locals.var_psi_dn0 - locals.var_chi_1_dn0), (locals.var_psi_dn2 - locals.var_chi_1_dn2), (locals.var_psi_dn6 - locals.var_chi_1_dn6), (locals.var_psi_dn7 - locals.var_chi_1_dn7), (locals.var_psi_dn10 - locals.var_chi_1_dn10), (locals.var_psi_dn11 - locals.var_chi_1_dn11), (locals.var_psi_dn12 - locals.var_chi_1_dn12), (locals.var_psi_dn17 - locals.var_chi_1_dn17),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn12, locals.var_psi_dn17,)
    }
};
        locals.var_psi = assign25970_e35422;
        locals.var_psi_dn0 = assign25970_e35422_d_n0;
        locals.var_psi_dn2 = assign25970_e35422_d_n2;
        locals.var_psi_dn6 = assign25970_e35422_d_n6;
        locals.var_psi_dn7 = assign25970_e35422_d_n7;
        locals.var_psi_dn10 = assign25970_e35422_d_n10;
        locals.var_psi_dn11 = assign25970_e35422_d_n11;
        locals.var_psi_dn12 = assign25970_e35422_d_n12;
        locals.var_psi_dn17 = assign25970_e35422_d_n17;
        locals.var_psi_rv = 0.0;

        let (assign25980_e35437, assign25980_e35437_d_n0, assign25980_e35437_d_n2, assign25980_e35437_d_n6, assign25980_e35437_d_n7, assign25980_e35437_d_n10, assign25980_e35437_d_n11, assign25980_e35437_d_n12, assign25980_e35437_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) {
        let assign25980_e35434: f64 = (locals.var_beta * 0.1);
        let assign25980_e35435: f64 = (locals.var_psi + assign25980_e35434);
        (assign25980_e35435, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn6, locals.var_psi_dn7, (locals.var_psi_dn10 + (locals.var_beta_dn10 * 0.1)), locals.var_psi_dn11, locals.var_psi_dn12, locals.var_psi_dn17,)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn12, locals.var_psi_dn17,)
    }
};
        locals.var_psi = assign25980_e35437;
        locals.var_psi_dn0 = assign25980_e35437_d_n0;
        locals.var_psi_dn2 = assign25980_e35437_d_n2;
        locals.var_psi_dn6 = assign25980_e35437_d_n6;
        locals.var_psi_dn7 = assign25980_e35437_d_n7;
        locals.var_psi_dn10 = assign25980_e35437_d_n10;
        locals.var_psi_dn11 = assign25980_e35437_d_n11;
        locals.var_psi_dn12 = assign25980_e35437_d_n12;
        locals.var_psi_dn17 = assign25980_e35437_d_n17;
        locals.var_psi_rv = 0.0;

        let (assign25990_e35464, assign25990_e35464_d_n0, assign25990_e35464_d_n2, assign25990_e35464_d_n6, assign25990_e35464_d_n7, assign25990_e35464_d_n10, assign25990_e35464_d_n11, assign25990_e35464_d_n12, assign25990_e35464_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) {
        let assign25990_e35448: f64 = (locals.var_gammachi * locals.var_t0__blk772);
        let assign25990_e35451: f64 = (locals.var_psi * locals.var_psi);
        let assign25990_e35452: f64 = (assign25990_e35448 + assign25990_e35451);
        let assign25990_e35453: f64 = (assign25990_e35452).ln();
        let assign25990_e35456: f64 = (locals.var_cnst1over * locals.var_t0__blk772);
        let assign25990_e35457: f64 = (assign25990_e35456).ln();
        let assign25990_e35458: f64 = (assign25990_e35453 - assign25990_e35457);
        let assign25990_e35461: f64 = (locals.var_beta * locals.var_vxbgmtcl);
        let assign25990_e35462: f64 = (assign25990_e35458 + assign25990_e35461);
        (assign25990_e35462, ((((((locals.var_gammachi_dn0 * locals.var_t0__blk772) + (locals.var_gammachi * locals.var_t0__blk772_dn0)) + ((locals.var_psi_dn0 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn0))) / assign25990_e35452) - (((locals.var_cnst1over_dn0 * locals.var_t0__blk772) + (locals.var_cnst1over * locals.var_t0__blk772_dn0)) / assign25990_e35456)) + (locals.var_beta * locals.var_vxbgmtcl_dn0)), ((((((locals.var_gammachi_dn2 * locals.var_t0__blk772) + (locals.var_gammachi * locals.var_t0__blk772_dn2)) + ((locals.var_psi_dn2 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn2))) / assign25990_e35452) - (((locals.var_cnst1over_dn2 * locals.var_t0__blk772) + (locals.var_cnst1over * locals.var_t0__blk772_dn2)) / assign25990_e35456)) + (locals.var_beta * locals.var_vxbgmtcl_dn2)), ((((((locals.var_gammachi_dn6 * locals.var_t0__blk772) + (locals.var_gammachi * locals.var_t0__blk772_dn6)) + ((locals.var_psi_dn6 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn6))) / assign25990_e35452) - (((locals.var_cnst1over_dn6 * locals.var_t0__blk772) + (locals.var_cnst1over * locals.var_t0__blk772_dn6)) / assign25990_e35456)) + (locals.var_beta * locals.var_vxbgmtcl_dn6)), ((((((locals.var_gammachi_dn7 * locals.var_t0__blk772) + (locals.var_gammachi * locals.var_t0__blk772_dn7)) + ((locals.var_psi_dn7 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn7))) / assign25990_e35452) - (((locals.var_cnst1over_dn7 * locals.var_t0__blk772) + (locals.var_cnst1over * locals.var_t0__blk772_dn7)) / assign25990_e35456)) + (locals.var_beta * locals.var_vxbgmtcl_dn7)), ((((((locals.var_gammachi_dn10 * locals.var_t0__blk772) + (locals.var_gammachi * locals.var_t0__blk772_dn10)) + ((locals.var_psi_dn10 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn10))) / assign25990_e35452) - (((locals.var_cnst1over_dn10 * locals.var_t0__blk772) + (locals.var_cnst1over * locals.var_t0__blk772_dn10)) / assign25990_e35456)) + ((locals.var_beta_dn10 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn10))), ((((((locals.var_gammachi_dn11 * locals.var_t0__blk772) + (locals.var_gammachi * locals.var_t0__blk772_dn11)) + ((locals.var_psi_dn11 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn11))) / assign25990_e35452) - (((locals.var_cnst1over_dn11 * locals.var_t0__blk772) + (locals.var_cnst1over * locals.var_t0__blk772_dn11)) / assign25990_e35456)) + (locals.var_beta * locals.var_vxbgmtcl_dn11)), ((((((locals.var_gammachi_dn12 * locals.var_t0__blk772) + (locals.var_gammachi * locals.var_t0__blk772_dn12)) + ((locals.var_psi_dn12 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn12))) / assign25990_e35452) - (((locals.var_cnst1over_dn12 * locals.var_t0__blk772) + (locals.var_cnst1over * locals.var_t0__blk772_dn12)) / assign25990_e35456)) + (locals.var_beta * locals.var_vxbgmtcl_dn12)), ((((((locals.var_gammachi_dn17 * locals.var_t0__blk772) + (locals.var_gammachi * locals.var_t0__blk772_dn17)) + ((locals.var_psi_dn17 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn17))) / assign25990_e35452) - (((locals.var_cnst1over_dn17 * locals.var_t0__blk772) + (locals.var_cnst1over * locals.var_t0__blk772_dn17)) / assign25990_e35456)) + (locals.var_beta * locals.var_vxbgmtcl_dn17)),)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn10, locals.var_chi_b_dn11, locals.var_chi_b_dn12, locals.var_chi_b_dn17,)
    }
};
        locals.var_chi_b = assign25990_e35464;
        locals.var_chi_b_dn0 = assign25990_e35464_d_n0;
        locals.var_chi_b_dn2 = assign25990_e35464_d_n2;
        locals.var_chi_b_dn6 = assign25990_e35464_d_n6;
        locals.var_chi_b_dn7 = assign25990_e35464_d_n7;
        locals.var_chi_b_dn10 = assign25990_e35464_d_n10;
        locals.var_chi_b_dn11 = assign25990_e35464_d_n11;
        locals.var_chi_b_dn12 = assign25990_e35464_d_n12;
        locals.var_chi_b_dn17 = assign25990_e35464_d_n17;
        locals.var_chi_b_rv = 0.0;

        let (assign26000_e35475, assign26000_e35475_d_n0, assign26000_e35475_d_n2, assign26000_e35475_d_n6, assign26000_e35475_d_n7, assign26000_e35475_d_n10, assign26000_e35475_d_n11, assign26000_e35475_d_n12, assign26000_e35475_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) {
        (locals.var_chi__blk816, locals.var_chi__blk816_dn0, locals.var_chi__blk816_dn2, locals.var_chi__blk816_dn6, locals.var_chi__blk816_dn7, locals.var_chi__blk816_dn10, locals.var_chi__blk816_dn11, locals.var_chi__blk816_dn12, locals.var_chi__blk816_dn17,)
    } else {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn10, locals.var_chi_a_dn11, locals.var_chi_a_dn12, locals.var_chi_a_dn17,)
    }
};
        locals.var_chi_a = assign26000_e35475;
        locals.var_chi_a_dn0 = assign26000_e35475_d_n0;
        locals.var_chi_a_dn2 = assign26000_e35475_d_n2;
        locals.var_chi_a_dn6 = assign26000_e35475_d_n6;
        locals.var_chi_a_dn7 = assign26000_e35475_d_n7;
        locals.var_chi_a_dn10 = assign26000_e35475_d_n10;
        locals.var_chi_a_dn11 = assign26000_e35475_d_n11;
        locals.var_chi_a_dn12 = assign26000_e35475_d_n12;
        locals.var_chi_a_dn17 = assign26000_e35475_d_n17;
        locals.var_chi_a_rv = 0.0;

        let (assign26010_e35492, assign26010_e35492_d_n0, assign26010_e35492_d_n2, assign26010_e35492_d_n6, assign26010_e35492_d_n7, assign26010_e35492_d_n10, assign26010_e35492_d_n11, assign26010_e35492_d_n12, assign26010_e35492_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) {
        let assign26010_e35486: f64 = (locals.var_chi_b - locals.var_chi_a);
        let assign26010_e35489: f64 = (0.0008 * 75.0);
        let assign26010_e35490: f64 = (assign26010_e35486 - assign26010_e35489);
        (assign26010_e35490, (locals.var_chi_b_dn0 - locals.var_chi_a_dn0), (locals.var_chi_b_dn2 - locals.var_chi_a_dn2), (locals.var_chi_b_dn6 - locals.var_chi_a_dn6), (locals.var_chi_b_dn7 - locals.var_chi_a_dn7), (locals.var_chi_b_dn10 - locals.var_chi_a_dn10), (locals.var_chi_b_dn11 - locals.var_chi_a_dn11), (locals.var_chi_b_dn12 - locals.var_chi_a_dn12), (locals.var_chi_b_dn17 - locals.var_chi_a_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign26010_e35492;
        locals.var_tmf1_dn0 = assign26010_e35492_d_n0;
        locals.var_tmf1_dn2 = assign26010_e35492_d_n2;
        locals.var_tmf1_dn6 = assign26010_e35492_d_n6;
        locals.var_tmf1_dn7 = assign26010_e35492_d_n7;
        locals.var_tmf1_dn10 = assign26010_e35492_d_n10;
        locals.var_tmf1_dn11 = assign26010_e35492_d_n11;
        locals.var_tmf1_dn12 = assign26010_e35492_d_n12;
        locals.var_tmf1_dn17 = assign26010_e35492_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign26020_e35509, assign26020_e35509_d_n0, assign26020_e35509_d_n2, assign26020_e35509_d_n6, assign26020_e35509_d_n7, assign26020_e35509_d_n10, assign26020_e35509_d_n11, assign26020_e35509_d_n12, assign26020_e35509_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) {
        let assign26020_e35503: f64 = (4.0 * locals.var_chi_b);
        let assign26020_e35506: f64 = (0.0008 * 75.0);
        let assign26020_e35507: f64 = (assign26020_e35503 * assign26020_e35506);
        (assign26020_e35507, ((4.0 * locals.var_chi_b_dn0) * assign26020_e35506), ((4.0 * locals.var_chi_b_dn2) * assign26020_e35506), ((4.0 * locals.var_chi_b_dn6) * assign26020_e35506), ((4.0 * locals.var_chi_b_dn7) * assign26020_e35506), ((4.0 * locals.var_chi_b_dn10) * assign26020_e35506), ((4.0 * locals.var_chi_b_dn11) * assign26020_e35506), ((4.0 * locals.var_chi_b_dn12) * assign26020_e35506), ((4.0 * locals.var_chi_b_dn17) * assign26020_e35506),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign26020_e35509;
        locals.var_tmf2_dn0 = assign26020_e35509_d_n0;
        locals.var_tmf2_dn2 = assign26020_e35509_d_n2;
        locals.var_tmf2_dn6 = assign26020_e35509_d_n6;
        locals.var_tmf2_dn7 = assign26020_e35509_d_n7;
        locals.var_tmf2_dn10 = assign26020_e35509_d_n10;
        locals.var_tmf2_dn11 = assign26020_e35509_d_n11;
        locals.var_tmf2_dn12 = assign26020_e35509_d_n12;
        locals.var_tmf2_dn17 = assign26020_e35509_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign26030_e35526, assign26030_e35526_d_n0, assign26030_e35526_d_n2, assign26030_e35526_d_n6, assign26030_e35526_d_n7, assign26030_e35526_d_n10, assign26030_e35526_d_n11, assign26030_e35526_d_n12, assign26030_e35526_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) {
        let (assign26030_e35524, assign26030_e35524_d_n0, assign26030_e35524_d_n2, assign26030_e35524_d_n6, assign26030_e35524_d_n7, assign26030_e35524_d_n10, assign26030_e35524_d_n11, assign26030_e35524_d_n12, assign26030_e35524_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign26030_e35523: f64 = (-locals.var_tmf2);
                (assign26030_e35523, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign26030_e35524, assign26030_e35524_d_n0, assign26030_e35524_d_n2, assign26030_e35524_d_n6, assign26030_e35524_d_n7, assign26030_e35524_d_n10, assign26030_e35524_d_n11, assign26030_e35524_d_n12, assign26030_e35524_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign26030_e35526;
        locals.var_tmf2_dn0 = assign26030_e35526_d_n0;
        locals.var_tmf2_dn2 = assign26030_e35526_d_n2;
        locals.var_tmf2_dn6 = assign26030_e35526_d_n6;
        locals.var_tmf2_dn7 = assign26030_e35526_d_n7;
        locals.var_tmf2_dn10 = assign26030_e35526_d_n10;
        locals.var_tmf2_dn11 = assign26030_e35526_d_n11;
        locals.var_tmf2_dn12 = assign26030_e35526_d_n12;
        locals.var_tmf2_dn17 = assign26030_e35526_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign26040_e35542, assign26040_e35542_d_n0, assign26040_e35542_d_n2, assign26040_e35542_d_n6, assign26040_e35542_d_n7, assign26040_e35542_d_n10, assign26040_e35542_d_n11, assign26040_e35542_d_n12, assign26040_e35542_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) {
        let assign26040_e35537: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign26040_e35539: f64 = (assign26040_e35537 + locals.var_tmf2);
        let assign26040_e35540: f64 = (assign26040_e35539).sqrt();
        (assign26040_e35540, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign26040_e35540)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign26040_e35540)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign26040_e35540)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign26040_e35540)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign26040_e35540)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign26040_e35540)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign26040_e35540)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign26040_e35540)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign26040_e35542;
        locals.var_tmf2_dn0 = assign26040_e35542_d_n0;
        locals.var_tmf2_dn2 = assign26040_e35542_d_n2;
        locals.var_tmf2_dn6 = assign26040_e35542_d_n6;
        locals.var_tmf2_dn7 = assign26040_e35542_d_n7;
        locals.var_tmf2_dn10 = assign26040_e35542_d_n10;
        locals.var_tmf2_dn11 = assign26040_e35542_d_n11;
        locals.var_tmf2_dn12 = assign26040_e35542_d_n12;
        locals.var_tmf2_dn17 = assign26040_e35542_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign26050_e35559, assign26050_e35559_d_n0, assign26050_e35559_d_n2, assign26050_e35559_d_n6, assign26050_e35559_d_n7, assign26050_e35559_d_n10, assign26050_e35559_d_n11, assign26050_e35559_d_n12, assign26050_e35559_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) {
        let assign26050_e35555: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign26050_e35556: f64 = (1.0 + assign26050_e35555);
        let assign26050_e35557: f64 = (0.5 * assign26050_e35556);
        (assign26050_e35557, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn17 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1__blk773, locals.var_t1__blk773_dn0, locals.var_t1__blk773_dn2, locals.var_t1__blk773_dn6, locals.var_t1__blk773_dn7, locals.var_t1__blk773_dn10, locals.var_t1__blk773_dn11, locals.var_t1__blk773_dn12, locals.var_t1__blk773_dn17,)
    }
};
        locals.var_t1__blk773 = assign26050_e35559;
        locals.var_t1__blk773_dn0 = assign26050_e35559_d_n0;
        locals.var_t1__blk773_dn2 = assign26050_e35559_d_n2;
        locals.var_t1__blk773_dn6 = assign26050_e35559_d_n6;
        locals.var_t1__blk773_dn7 = assign26050_e35559_d_n7;
        locals.var_t1__blk773_dn10 = assign26050_e35559_d_n10;
        locals.var_t1__blk773_dn11 = assign26050_e35559_d_n11;
        locals.var_t1__blk773_dn12 = assign26050_e35559_d_n12;
        locals.var_t1__blk773_dn17 = assign26050_e35559_d_n17;
        locals.var_t1__blk773_rv = 0.0;

        let (assign26060_e35582, assign26060_e35582_d_n0, assign26060_e35582_d_n2, assign26060_e35582_d_n6, assign26060_e35582_d_n7, assign26060_e35582_d_n10, assign26060_e35582_d_n11, assign26060_e35582_d_n12, assign26060_e35582_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) {
        let assign26060_e35573: f64 = (2.0 * 0.0008);
        let assign26060_e35575: f64 = (assign26060_e35573 * 75.0);
        let assign26060_e35576: f64 = (locals.var_tmf1 + assign26060_e35575);
        let assign26060_e35578: f64 = (assign26060_e35576 / locals.var_tmf2);
        let assign26060_e35579: f64 = (1.0 - assign26060_e35578);
        let assign26060_e35580: f64 = (0.5 * assign26060_e35579);
        (assign26060_e35580, (0.5 * (-(((locals.var_tmf1_dn0 * locals.var_tmf2) - (assign26060_e35576 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn2 * locals.var_tmf2) - (assign26060_e35576 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn6 * locals.var_tmf2) - (assign26060_e35576 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn7 * locals.var_tmf2) - (assign26060_e35576 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn10 * locals.var_tmf2) - (assign26060_e35576 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn11 * locals.var_tmf2) - (assign26060_e35576 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn12 * locals.var_tmf2) - (assign26060_e35576 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn17 * locals.var_tmf2) - (assign26060_e35576 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t2__blk774, locals.var_t2__blk774_dn0, locals.var_t2__blk774_dn2, locals.var_t2__blk774_dn6, locals.var_t2__blk774_dn7, locals.var_t2__blk774_dn10, locals.var_t2__blk774_dn11, locals.var_t2__blk774_dn12, locals.var_t2__blk774_dn17,)
    }
};
        locals.var_t2__blk774 = assign26060_e35582;
        locals.var_t2__blk774_dn0 = assign26060_e35582_d_n0;
        locals.var_t2__blk774_dn2 = assign26060_e35582_d_n2;
        locals.var_t2__blk774_dn6 = assign26060_e35582_d_n6;
        locals.var_t2__blk774_dn7 = assign26060_e35582_d_n7;
        locals.var_t2__blk774_dn10 = assign26060_e35582_d_n10;
        locals.var_t2__blk774_dn11 = assign26060_e35582_d_n11;
        locals.var_t2__blk774_dn12 = assign26060_e35582_d_n12;
        locals.var_t2__blk774_dn17 = assign26060_e35582_d_n17;
        locals.var_t2__blk774_rv = 0.0;

        let (assign26070_e35599, assign26070_e35599_d_n0, assign26070_e35599_d_n2, assign26070_e35599_d_n6, assign26070_e35599_d_n7, assign26070_e35599_d_n10, assign26070_e35599_d_n11, assign26070_e35599_d_n12, assign26070_e35599_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) {
        let assign26070_e35595: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign26070_e35596: f64 = (0.5 * assign26070_e35595);
        let assign26070_e35597: f64 = (locals.var_chi_b - assign26070_e35596);
        (assign26070_e35597, (locals.var_chi_b_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_chi_b_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_chi_b_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_chi_b_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_chi_b_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_chi_b_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_chi_b_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_chi_b_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_chi__blk816, locals.var_chi__blk816_dn0, locals.var_chi__blk816_dn2, locals.var_chi__blk816_dn6, locals.var_chi__blk816_dn7, locals.var_chi__blk816_dn10, locals.var_chi__blk816_dn11, locals.var_chi__blk816_dn12, locals.var_chi__blk816_dn17,)
    }
};
        locals.var_chi__blk816 = assign26070_e35599;
        locals.var_chi__blk816_dn0 = assign26070_e35599_d_n0;
        locals.var_chi__blk816_dn2 = assign26070_e35599_d_n2;
        locals.var_chi__blk816_dn6 = assign26070_e35599_d_n6;
        locals.var_chi__blk816_dn7 = assign26070_e35599_d_n7;
        locals.var_chi__blk816_dn10 = assign26070_e35599_d_n10;
        locals.var_chi__blk816_dn11 = assign26070_e35599_d_n11;
        locals.var_chi__blk816_dn12 = assign26070_e35599_d_n12;
        locals.var_chi__blk816_dn17 = assign26070_e35599_d_n17;
        locals.var_chi__blk816_rv = 0.0;

        let (assign26080_e35614, assign26080_e35614_d_n0, assign26080_e35614_d_n2, assign26080_e35614_d_n6, assign26080_e35614_d_n7, assign26080_e35614_d_n10, assign26080_e35614_d_n11, assign26080_e35614_d_n12, assign26080_e35614_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) {
        let assign26080_e35610: f64 = (locals.var_chi__blk816 / locals.var_beta);
        let assign26080_e35612: f64 = (assign26080_e35610 - locals.var_vxbgmtcl);
        (assign26080_e35612, ((locals.var_chi__blk816_dn0 / locals.var_beta) - locals.var_vxbgmtcl_dn0), ((locals.var_chi__blk816_dn2 / locals.var_beta) - locals.var_vxbgmtcl_dn2), ((locals.var_chi__blk816_dn6 / locals.var_beta) - locals.var_vxbgmtcl_dn6), ((locals.var_chi__blk816_dn7 / locals.var_beta) - locals.var_vxbgmtcl_dn7), ((((locals.var_chi__blk816_dn10 * locals.var_beta) - (locals.var_chi__blk816 * locals.var_beta_dn10)) / (locals.var_beta * locals.var_beta)) - locals.var_vxbgmtcl_dn10), ((locals.var_chi__blk816_dn11 / locals.var_beta) - locals.var_vxbgmtcl_dn11), ((locals.var_chi__blk816_dn12 / locals.var_beta) - locals.var_vxbgmtcl_dn12), ((locals.var_chi__blk816_dn17 / locals.var_beta) - locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn12, locals.var_ps0ld_dn17,)
    }
};
        locals.var_ps0ld = assign26080_e35614;
        locals.var_ps0ld_dn0 = assign26080_e35614_d_n0;
        locals.var_ps0ld_dn2 = assign26080_e35614_d_n2;
        locals.var_ps0ld_dn6 = assign26080_e35614_d_n6;
        locals.var_ps0ld_dn7 = assign26080_e35614_d_n7;
        locals.var_ps0ld_dn10 = assign26080_e35614_d_n10;
        locals.var_ps0ld_dn11 = assign26080_e35614_d_n11;
        locals.var_ps0ld_dn12 = assign26080_e35614_d_n12;
        locals.var_ps0ld_dn17 = assign26080_e35614_d_n17;
        locals.var_ps0ld_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_92(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign26090_e35631, assign26090_e35631_d_n0, assign26090_e35631_d_n2, assign26090_e35631_d_n6, assign26090_e35631_d_n7, assign26090_e35631_d_n10, assign26090_e35631_d_n11, assign26090_e35631_d_n12, assign26090_e35631_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) {
        let assign26090_e35625: f64 = (locals.var_chi__blk816 - 1.0);
        let assign26090_e35627: f64 = (-locals.var_chi__blk816);
        let assign26090_e35628: f64 = (assign26090_e35627).exp();
        let assign26090_e35629: f64 = (assign26090_e35625 + assign26090_e35628);
        (assign26090_e35629, (locals.var_chi__blk816_dn0 + (assign26090_e35628 * (-locals.var_chi__blk816_dn0))), (locals.var_chi__blk816_dn2 + (assign26090_e35628 * (-locals.var_chi__blk816_dn2))), (locals.var_chi__blk816_dn6 + (assign26090_e35628 * (-locals.var_chi__blk816_dn6))), (locals.var_chi__blk816_dn7 + (assign26090_e35628 * (-locals.var_chi__blk816_dn7))), (locals.var_chi__blk816_dn10 + (assign26090_e35628 * (-locals.var_chi__blk816_dn10))), (locals.var_chi__blk816_dn11 + (assign26090_e35628 * (-locals.var_chi__blk816_dn11))), (locals.var_chi__blk816_dn12 + (assign26090_e35628 * (-locals.var_chi__blk816_dn12))), (locals.var_chi__blk816_dn17 + (assign26090_e35628 * (-locals.var_chi__blk816_dn17))),)
    } else {
        (locals.var_t1__blk773, locals.var_t1__blk773_dn0, locals.var_t1__blk773_dn2, locals.var_t1__blk773_dn6, locals.var_t1__blk773_dn7, locals.var_t1__blk773_dn10, locals.var_t1__blk773_dn11, locals.var_t1__blk773_dn12, locals.var_t1__blk773_dn17,)
    }
};
        locals.var_t1__blk773 = assign26090_e35631;
        locals.var_t1__blk773_dn0 = assign26090_e35631_d_n0;
        locals.var_t1__blk773_dn2 = assign26090_e35631_d_n2;
        locals.var_t1__blk773_dn6 = assign26090_e35631_d_n6;
        locals.var_t1__blk773_dn7 = assign26090_e35631_d_n7;
        locals.var_t1__blk773_dn10 = assign26090_e35631_d_n10;
        locals.var_t1__blk773_dn11 = assign26090_e35631_d_n11;
        locals.var_t1__blk773_dn12 = assign26090_e35631_d_n12;
        locals.var_t1__blk773_dn17 = assign26090_e35631_d_n17;
        locals.var_t1__blk773_rv = 0.0;

        let assign26100_e35635: f64 = (10.0 * 2.220446049250313e-16);
        let assign26100_e35636: f64 = if locals.var_t1__blk773 < assign26100_e35635 { 1.0 } else { 0.0 };
        locals.var_guard860 = assign26100_e35636;
        locals.var_guard860_rv = 0.0;

        let (assign26110_e35651, assign26110_e35651_d_n0, assign26110_e35651_d_n2, assign26110_e35651_d_n6, assign26110_e35651_d_n7, assign26110_e35651_d_n10, assign26110_e35651_d_n11, assign26110_e35651_d_n12, assign26110_e35651_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard860 != 0.0)) {
        let assign26110_e35649: f64 = (10.0 * 2.220446049250313e-16);
        (assign26110_e35649, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk773, locals.var_t1__blk773_dn0, locals.var_t1__blk773_dn2, locals.var_t1__blk773_dn6, locals.var_t1__blk773_dn7, locals.var_t1__blk773_dn10, locals.var_t1__blk773_dn11, locals.var_t1__blk773_dn12, locals.var_t1__blk773_dn17,)
    }
};
        locals.var_t1__blk773 = assign26110_e35651;
        locals.var_t1__blk773_dn0 = assign26110_e35651_d_n0;
        locals.var_t1__blk773_dn2 = assign26110_e35651_d_n2;
        locals.var_t1__blk773_dn6 = assign26110_e35651_d_n6;
        locals.var_t1__blk773_dn7 = assign26110_e35651_d_n7;
        locals.var_t1__blk773_dn10 = assign26110_e35651_d_n10;
        locals.var_t1__blk773_dn11 = assign26110_e35651_d_n11;
        locals.var_t1__blk773_dn12 = assign26110_e35651_d_n12;
        locals.var_t1__blk773_dn17 = assign26110_e35651_d_n17;
        locals.var_t1__blk773_rv = 0.0;

        let (assign26120_e35663, assign26120_e35663_d_n0, assign26120_e35663_d_n2, assign26120_e35663_d_n6, assign26120_e35663_d_n7, assign26120_e35663_d_n10, assign26120_e35663_d_n11, assign26120_e35663_d_n12, assign26120_e35663_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) {
        let assign26120_e35661: f64 = (locals.var_t1__blk773).sqrt();
        (assign26120_e35661, (locals.var_t1__blk773_dn0 / (2.0 * assign26120_e35661)), (locals.var_t1__blk773_dn2 / (2.0 * assign26120_e35661)), (locals.var_t1__blk773_dn6 / (2.0 * assign26120_e35661)), (locals.var_t1__blk773_dn7 / (2.0 * assign26120_e35661)), (locals.var_t1__blk773_dn10 / (2.0 * assign26120_e35661)), (locals.var_t1__blk773_dn11 / (2.0 * assign26120_e35661)), (locals.var_t1__blk773_dn12 / (2.0 * assign26120_e35661)), (locals.var_t1__blk773_dn17 / (2.0 * assign26120_e35661)),)
    } else {
        (locals.var_t2__blk774, locals.var_t2__blk774_dn0, locals.var_t2__blk774_dn2, locals.var_t2__blk774_dn6, locals.var_t2__blk774_dn7, locals.var_t2__blk774_dn10, locals.var_t2__blk774_dn11, locals.var_t2__blk774_dn12, locals.var_t2__blk774_dn17,)
    }
};
        locals.var_t2__blk774 = assign26120_e35663;
        locals.var_t2__blk774_dn0 = assign26120_e35663_d_n0;
        locals.var_t2__blk774_dn2 = assign26120_e35663_d_n2;
        locals.var_t2__blk774_dn6 = assign26120_e35663_d_n6;
        locals.var_t2__blk774_dn7 = assign26120_e35663_d_n7;
        locals.var_t2__blk774_dn10 = assign26120_e35663_d_n10;
        locals.var_t2__blk774_dn11 = assign26120_e35663_d_n11;
        locals.var_t2__blk774_dn12 = assign26120_e35663_d_n12;
        locals.var_t2__blk774_dn17 = assign26120_e35663_d_n17;
        locals.var_t2__blk774_rv = 0.0;

        let (assign26130_e35676, assign26130_e35676_d_n0, assign26130_e35676_d_n2, assign26130_e35676_d_n6, assign26130_e35676_d_n7, assign26130_e35676_d_n10, assign26130_e35676_d_n11, assign26130_e35676_d_n12, assign26130_e35676_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) {
        let assign26130_e35674: f64 = (locals.var_cnst0over * locals.var_t2__blk774);
        (assign26130_e35674, ((locals.var_cnst0over_dn0 * locals.var_t2__blk774) + (locals.var_cnst0over * locals.var_t2__blk774_dn0)), ((locals.var_cnst0over_dn2 * locals.var_t2__blk774) + (locals.var_cnst0over * locals.var_t2__blk774_dn2)), ((locals.var_cnst0over_dn6 * locals.var_t2__blk774) + (locals.var_cnst0over * locals.var_t2__blk774_dn6)), ((locals.var_cnst0over_dn7 * locals.var_t2__blk774) + (locals.var_cnst0over * locals.var_t2__blk774_dn7)), ((locals.var_cnst0over_dn10 * locals.var_t2__blk774) + (locals.var_cnst0over * locals.var_t2__blk774_dn10)), ((locals.var_cnst0over_dn11 * locals.var_t2__blk774) + (locals.var_cnst0over * locals.var_t2__blk774_dn11)), ((locals.var_cnst0over_dn12 * locals.var_t2__blk774) + (locals.var_cnst0over * locals.var_t2__blk774_dn12)), ((locals.var_cnst0over_dn17 * locals.var_t2__blk774) + (locals.var_cnst0over * locals.var_t2__blk774_dn17)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn12, locals.var_qbuld_dn17,)
    }
};
        locals.var_qbuld = assign26130_e35676;
        locals.var_qbuld_dn0 = assign26130_e35676_d_n0;
        locals.var_qbuld_dn2 = assign26130_e35676_d_n2;
        locals.var_qbuld_dn6 = assign26130_e35676_d_n6;
        locals.var_qbuld_dn7 = assign26130_e35676_d_n7;
        locals.var_qbuld_dn10 = assign26130_e35676_d_n10;
        locals.var_qbuld_dn11 = assign26130_e35676_d_n11;
        locals.var_qbuld_dn12 = assign26130_e35676_d_n12;
        locals.var_qbuld_dn17 = assign26130_e35676_d_n17;
        locals.var_qbuld_rv = 0.0;

        let (assign26140_e35691, assign26140_e35691_d_n0, assign26140_e35691_d_n2, assign26140_e35691_d_n6, assign26140_e35691_d_n7, assign26140_e35691_d_n10, assign26140_e35691_d_n11, assign26140_e35691_d_n12, assign26140_e35691_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) {
        let assign26140_e35688: f64 = (locals.var_vgpld - locals.var_ps0ld);
        let assign26140_e35689: f64 = (locals.var_cox0 * assign26140_e35688);
        (assign26140_e35689, (locals.var_cox0 * (locals.var_vgpld_dn0 - locals.var_ps0ld_dn0)), (locals.var_cox0 * (locals.var_vgpld_dn2 - locals.var_ps0ld_dn2)), (locals.var_cox0 * (locals.var_vgpld_dn6 - locals.var_ps0ld_dn6)), (locals.var_cox0 * (locals.var_vgpld_dn7 - locals.var_ps0ld_dn7)), (locals.var_cox0 * (locals.var_vgpld_dn10 - locals.var_ps0ld_dn10)), (locals.var_cox0 * (locals.var_vgpld_dn11 - locals.var_ps0ld_dn11)), (locals.var_cox0 * (locals.var_vgpld_dn12 - locals.var_ps0ld_dn12)), (locals.var_cox0 * (locals.var_vgpld_dn17 - locals.var_ps0ld_dn17)),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17,)
    }
};
        locals.var_qsuld = assign26140_e35691;
        locals.var_qsuld_dn0 = assign26140_e35691_d_n0;
        locals.var_qsuld_dn2 = assign26140_e35691_d_n2;
        locals.var_qsuld_dn6 = assign26140_e35691_d_n6;
        locals.var_qsuld_dn7 = assign26140_e35691_d_n7;
        locals.var_qsuld_dn10 = assign26140_e35691_d_n10;
        locals.var_qsuld_dn11 = assign26140_e35691_d_n11;
        locals.var_qsuld_dn12 = assign26140_e35691_d_n12;
        locals.var_qsuld_dn17 = assign26140_e35691_d_n17;
        locals.var_qsuld_rv = 0.0;

        let assign26150_e35694: f64 = if p.p42 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard861 = assign26150_e35694;
        locals.var_guard861_rv = 0.0;

        let (assign26160_e35711, assign26160_e35711_d_n0, assign26160_e35711_d_n2, assign26160_e35711_d_n6, assign26160_e35711_d_n7, assign26160_e35711_d_n10, assign26160_e35711_d_n11, assign26160_e35711_d_n12, assign26160_e35711_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard861 != 0.0)) {
        let assign26160_e35707: f64 = (-locals.var_vxbgmtcl);
        let assign26160_e35708: f64 = (locals.var_beta * assign26160_e35707);
        let assign26160_e35709: f64 = (assign26160_e35708).exp();
        (assign26160_e35709, (assign26160_e35709 * (locals.var_beta * (-locals.var_vxbgmtcl_dn0))), (assign26160_e35709 * (locals.var_beta * (-locals.var_vxbgmtcl_dn2))), (assign26160_e35709 * (locals.var_beta * (-locals.var_vxbgmtcl_dn6))), (assign26160_e35709 * (locals.var_beta * (-locals.var_vxbgmtcl_dn7))), (assign26160_e35709 * ((locals.var_beta_dn10 * assign26160_e35707) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10)))), (assign26160_e35709 * (locals.var_beta * (-locals.var_vxbgmtcl_dn11))), (assign26160_e35709 * (locals.var_beta * (-locals.var_vxbgmtcl_dn12))), (assign26160_e35709 * (locals.var_beta * (-locals.var_vxbgmtcl_dn17))),)
    } else {
        (locals.var_exp_bvbs__blk835, locals.var_exp_bvbs__blk835_dn0, locals.var_exp_bvbs__blk835_dn2, locals.var_exp_bvbs__blk835_dn6, locals.var_exp_bvbs__blk835_dn7, locals.var_exp_bvbs__blk835_dn10, locals.var_exp_bvbs__blk835_dn11, locals.var_exp_bvbs__blk835_dn12, locals.var_exp_bvbs__blk835_dn17,)
    }
};
        locals.var_exp_bvbs__blk835 = assign26160_e35711;
        locals.var_exp_bvbs__blk835_dn0 = assign26160_e35711_d_n0;
        locals.var_exp_bvbs__blk835_dn2 = assign26160_e35711_d_n2;
        locals.var_exp_bvbs__blk835_dn6 = assign26160_e35711_d_n6;
        locals.var_exp_bvbs__blk835_dn7 = assign26160_e35711_d_n7;
        locals.var_exp_bvbs__blk835_dn10 = assign26160_e35711_d_n10;
        locals.var_exp_bvbs__blk835_dn11 = assign26160_e35711_d_n11;
        locals.var_exp_bvbs__blk835_dn12 = assign26160_e35711_d_n12;
        locals.var_exp_bvbs__blk835_dn17 = assign26160_e35711_d_n17;
        locals.var_exp_bvbs__blk835_rv = 0.0;

        let (assign26170_e35726, assign26170_e35726_d_n0, assign26170_e35726_d_n2, assign26170_e35726_d_n6, assign26170_e35726_d_n7, assign26170_e35726_d_n10, assign26170_e35726_d_n11, assign26170_e35726_d_n12, assign26170_e35726_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard861 != 0.0)) {
        let assign26170_e35724: f64 = (locals.var_nin / locals.var_uc_nsubbttub);
        (assign26170_e35724, (((locals.var_nin_dn0 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn0)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn2 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn2)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn6 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn6)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn7 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn7)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn10 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn10)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn11 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn11)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn12 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn12)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn17 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn17)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)),)
    } else {
        (locals.var_t0__blk772, locals.var_t0__blk772_dn0, locals.var_t0__blk772_dn2, locals.var_t0__blk772_dn6, locals.var_t0__blk772_dn7, locals.var_t0__blk772_dn10, locals.var_t0__blk772_dn11, locals.var_t0__blk772_dn12, locals.var_t0__blk772_dn17,)
    }
};
        locals.var_t0__blk772 = assign26170_e35726;
        locals.var_t0__blk772_dn0 = assign26170_e35726_d_n0;
        locals.var_t0__blk772_dn2 = assign26170_e35726_d_n2;
        locals.var_t0__blk772_dn6 = assign26170_e35726_d_n6;
        locals.var_t0__blk772_dn7 = assign26170_e35726_d_n7;
        locals.var_t0__blk772_dn10 = assign26170_e35726_d_n10;
        locals.var_t0__blk772_dn11 = assign26170_e35726_d_n11;
        locals.var_t0__blk772_dn12 = assign26170_e35726_d_n12;
        locals.var_t0__blk772_dn17 = assign26170_e35726_d_n17;
        locals.var_t0__blk772_rv = 0.0;

        let (assign26180_e35741, assign26180_e35741_d_n0, assign26180_e35741_d_n2, assign26180_e35741_d_n6, assign26180_e35741_d_n7, assign26180_e35741_d_n10, assign26180_e35741_d_n11, assign26180_e35741_d_n12, assign26180_e35741_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard861 != 0.0)) {
        let assign26180_e35739: f64 = (locals.var_t0__blk772 * locals.var_t0__blk772);
        (assign26180_e35739, ((locals.var_t0__blk772_dn0 * locals.var_t0__blk772) + (locals.var_t0__blk772 * locals.var_t0__blk772_dn0)), ((locals.var_t0__blk772_dn2 * locals.var_t0__blk772) + (locals.var_t0__blk772 * locals.var_t0__blk772_dn2)), ((locals.var_t0__blk772_dn6 * locals.var_t0__blk772) + (locals.var_t0__blk772 * locals.var_t0__blk772_dn6)), ((locals.var_t0__blk772_dn7 * locals.var_t0__blk772) + (locals.var_t0__blk772 * locals.var_t0__blk772_dn7)), ((locals.var_t0__blk772_dn10 * locals.var_t0__blk772) + (locals.var_t0__blk772 * locals.var_t0__blk772_dn10)), ((locals.var_t0__blk772_dn11 * locals.var_t0__blk772) + (locals.var_t0__blk772 * locals.var_t0__blk772_dn11)), ((locals.var_t0__blk772_dn12 * locals.var_t0__blk772) + (locals.var_t0__blk772 * locals.var_t0__blk772_dn12)), ((locals.var_t0__blk772_dn17 * locals.var_t0__blk772) + (locals.var_t0__blk772 * locals.var_t0__blk772_dn17)),)
    } else {
        (locals.var_cnst1over, locals.var_cnst1over_dn0, locals.var_cnst1over_dn2, locals.var_cnst1over_dn6, locals.var_cnst1over_dn7, locals.var_cnst1over_dn10, locals.var_cnst1over_dn11, locals.var_cnst1over_dn12, locals.var_cnst1over_dn17,)
    }
};
        locals.var_cnst1over = assign26180_e35741;
        locals.var_cnst1over_dn0 = assign26180_e35741_d_n0;
        locals.var_cnst1over_dn2 = assign26180_e35741_d_n2;
        locals.var_cnst1over_dn6 = assign26180_e35741_d_n6;
        locals.var_cnst1over_dn7 = assign26180_e35741_d_n7;
        locals.var_cnst1over_dn10 = assign26180_e35741_d_n10;
        locals.var_cnst1over_dn11 = assign26180_e35741_d_n11;
        locals.var_cnst1over_dn12 = assign26180_e35741_d_n12;
        locals.var_cnst1over_dn17 = assign26180_e35741_d_n17;
        locals.var_cnst1over_rv = 0.0;

        let (assign26190_e35756, assign26190_e35756_d_n0, assign26190_e35756_d_n2, assign26190_e35756_d_n6, assign26190_e35756_d_n7, assign26190_e35756_d_n10, assign26190_e35756_d_n11, assign26190_e35756_d_n12, assign26190_e35756_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard861 != 0.0)) {
        let assign26190_e35754: f64 = (locals.var_cnst1over * locals.var_exp_bvbs__blk835);
        (assign26190_e35754, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs__blk835) + (locals.var_cnst1over * locals.var_exp_bvbs__blk835_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs__blk835) + (locals.var_cnst1over * locals.var_exp_bvbs__blk835_dn2)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs__blk835) + (locals.var_cnst1over * locals.var_exp_bvbs__blk835_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs__blk835) + (locals.var_cnst1over * locals.var_exp_bvbs__blk835_dn7)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs__blk835) + (locals.var_cnst1over * locals.var_exp_bvbs__blk835_dn10)), ((locals.var_cnst1over_dn11 * locals.var_exp_bvbs__blk835) + (locals.var_cnst1over * locals.var_exp_bvbs__blk835_dn11)), ((locals.var_cnst1over_dn12 * locals.var_exp_bvbs__blk835) + (locals.var_cnst1over * locals.var_exp_bvbs__blk835_dn12)), ((locals.var_cnst1over_dn17 * locals.var_exp_bvbs__blk835) + (locals.var_cnst1over * locals.var_exp_bvbs__blk835_dn17)),)
    } else {
        (locals.var_cfs1__blk844, locals.var_cfs1__blk844_dn0, locals.var_cfs1__blk844_dn2, locals.var_cfs1__blk844_dn6, locals.var_cfs1__blk844_dn7, locals.var_cfs1__blk844_dn10, locals.var_cfs1__blk844_dn11, locals.var_cfs1__blk844_dn12, locals.var_cfs1__blk844_dn17,)
    }
};
        locals.var_cfs1__blk844 = assign26190_e35756;
        locals.var_cfs1__blk844_dn0 = assign26190_e35756_d_n0;
        locals.var_cfs1__blk844_dn2 = assign26190_e35756_d_n2;
        locals.var_cfs1__blk844_dn6 = assign26190_e35756_d_n6;
        locals.var_cfs1__blk844_dn7 = assign26190_e35756_d_n7;
        locals.var_cfs1__blk844_dn10 = assign26190_e35756_d_n10;
        locals.var_cfs1__blk844_dn11 = assign26190_e35756_d_n11;
        locals.var_cfs1__blk844_dn12 = assign26190_e35756_d_n12;
        locals.var_cfs1__blk844_dn17 = assign26190_e35756_d_n17;
        locals.var_cfs1__blk844_rv = 0.0;

        let (assign26200_e35769,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard861 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv__blk789,)
    }
};
        locals.var_flg_conv__blk789 = assign26200_e35769;
        locals.var_flg_conv__blk789_rv = 0.0;

        let (assign26210_e35782,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard861 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign26210_e35782;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_93(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let mut assign26220_loop_guard: usize = 0;
        while {
            let assign26220_cond_e35796: f64 = (2.0 * 20.0);
            let assign26220_cond_e35798: f64 = (assign26220_cond_e35796 + 1.0);
            let assign26220_cond_e35800: f64 = if ((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard861 != 0.0)) && (locals.var_lp_s0 <= assign26220_cond_e35798)) { 1.0 } else { 0.0 };
            assign26220_cond_e35800 != 0.0
        } {
            assign26220_loop_guard += 1;
            assert!(assign26220_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign26220_body0_e35813, assign26220_body0_e35813_d_n0, assign26220_body0_e35813_d_n2, assign26220_body0_e35813_d_n6, assign26220_body0_e35813_d_n7, assign26220_body0_e35813_d_n10, assign26220_body0_e35813_d_n11, assign26220_body0_e35813_d_n12, assign26220_body0_e35813_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard861 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb__blk840, locals.var_fb__blk840_dn0, locals.var_fb__blk840_dn2, locals.var_fb__blk840_dn6, locals.var_fb__blk840_dn7, locals.var_fb__blk840_dn10, locals.var_fb__blk840_dn11, locals.var_fb__blk840_dn12, locals.var_fb__blk840_dn17,)
    }
};
            locals.var_fb__blk840 = assign26220_body0_e35813;
            locals.var_fb__blk840_dn0 = assign26220_body0_e35813_d_n0;
            locals.var_fb__blk840_dn2 = assign26220_body0_e35813_d_n2;
            locals.var_fb__blk840_dn6 = assign26220_body0_e35813_d_n6;
            locals.var_fb__blk840_dn7 = assign26220_body0_e35813_d_n7;
            locals.var_fb__blk840_dn10 = assign26220_body0_e35813_d_n10;
            locals.var_fb__blk840_dn11 = assign26220_body0_e35813_d_n11;
            locals.var_fb__blk840_dn12 = assign26220_body0_e35813_d_n12;
            locals.var_fb__blk840_dn17 = assign26220_body0_e35813_d_n17;
            locals.var_fb__blk840_rv = 0.0;
            let (assign26220_body1_e35830, assign26220_body1_e35830_d_n0, assign26220_body1_e35830_d_n2, assign26220_body1_e35830_d_n6, assign26220_body1_e35830_d_n7, assign26220_body1_e35830_d_n10, assign26220_body1_e35830_d_n11, assign26220_body1_e35830_d_n12, assign26220_body1_e35830_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard861 != 0.0)) {
        let assign26220_body1_e35827: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
        let assign26220_body1_e35828: f64 = (locals.var_beta * assign26220_body1_e35827);
        (assign26220_body1_e35828, (locals.var_beta * (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0)), (locals.var_beta * (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2)), (locals.var_beta * (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6)), (locals.var_beta * (locals.var_ps0ld_dn7 + locals.var_vxbgmtcl_dn7)), ((locals.var_beta_dn10 * assign26220_body1_e35827) + (locals.var_beta * (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10))), (locals.var_beta * (locals.var_ps0ld_dn11 + locals.var_vxbgmtcl_dn11)), (locals.var_beta * (locals.var_ps0ld_dn12 + locals.var_vxbgmtcl_dn12)), (locals.var_beta * (locals.var_ps0ld_dn17 + locals.var_vxbgmtcl_dn17)),)
    } else {
        (locals.var_chi__blk816, locals.var_chi__blk816_dn0, locals.var_chi__blk816_dn2, locals.var_chi__blk816_dn6, locals.var_chi__blk816_dn7, locals.var_chi__blk816_dn10, locals.var_chi__blk816_dn11, locals.var_chi__blk816_dn12, locals.var_chi__blk816_dn17,)
    }
};
            locals.var_chi__blk816 = assign26220_body1_e35830;
            locals.var_chi__blk816_dn0 = assign26220_body1_e35830_d_n0;
            locals.var_chi__blk816_dn2 = assign26220_body1_e35830_d_n2;
            locals.var_chi__blk816_dn6 = assign26220_body1_e35830_d_n6;
            locals.var_chi__blk816_dn7 = assign26220_body1_e35830_d_n7;
            locals.var_chi__blk816_dn10 = assign26220_body1_e35830_d_n10;
            locals.var_chi__blk816_dn11 = assign26220_body1_e35830_d_n11;
            locals.var_chi__blk816_dn12 = assign26220_body1_e35830_d_n12;
            locals.var_chi__blk816_dn17 = assign26220_body1_e35830_d_n17;
            locals.var_chi__blk816_rv = 0.0;
            let assign26220_body2_e35833: f64 = if locals.var_chi__blk816 < 5.0 { 1.0 } else { 0.0 };
            locals.var_guard862 = assign26220_body2_e35833;
            locals.var_guard862_rv = 0.0;
            let (assign26220_body3_e35863, assign26220_body3_e35863_d_n0, assign26220_body3_e35863_d_n2, assign26220_body3_e35863_d_n6, assign26220_body3_e35863_d_n7, assign26220_body3_e35863_d_n10, assign26220_body3_e35863_d_n11, assign26220_body3_e35863_d_n12, assign26220_body3_e35863_d_n17,) = {
    if ((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard861 != 0.0)) && (locals.var_guard862 != 0.0)) {
        let assign26220_body3_e35848: f64 = (locals.var_chi__blk816 * locals.var_chi__blk816);
        let assign26220_body3_e35850: f64 = (assign26220_body3_e35848 * locals.var_chi__blk816);
        let assign26220_body3_e35854: f64 = (-0.07053654284009761);
        let assign26220_body3_e35857: f64 = (locals.var_chi__blk816 * 0.006115288895133179);
        let assign26220_body3_e35858: f64 = (assign26220_body3_e35854 + assign26220_body3_e35857);
        let assign26220_body3_e35859: f64 = (locals.var_chi__blk816 * assign26220_body3_e35858);
        let assign26220_body3_e35860: f64 = (0.29693154855771 + assign26220_body3_e35859);
        let assign26220_body3_e35861: f64 = (assign26220_body3_e35850 * assign26220_body3_e35860);
        (assign26220_body3_e35861, ((((((locals.var_chi__blk816_dn0 * locals.var_chi__blk816) + (locals.var_chi__blk816 * locals.var_chi__blk816_dn0)) * locals.var_chi__blk816) + (assign26220_body3_e35848 * locals.var_chi__blk816_dn0)) * assign26220_body3_e35860) + (assign26220_body3_e35850 * ((locals.var_chi__blk816_dn0 * assign26220_body3_e35858) + (locals.var_chi__blk816 * (locals.var_chi__blk816_dn0 * 0.006115288895133179))))), ((((((locals.var_chi__blk816_dn2 * locals.var_chi__blk816) + (locals.var_chi__blk816 * locals.var_chi__blk816_dn2)) * locals.var_chi__blk816) + (assign26220_body3_e35848 * locals.var_chi__blk816_dn2)) * assign26220_body3_e35860) + (assign26220_body3_e35850 * ((locals.var_chi__blk816_dn2 * assign26220_body3_e35858) + (locals.var_chi__blk816 * (locals.var_chi__blk816_dn2 * 0.006115288895133179))))), ((((((locals.var_chi__blk816_dn6 * locals.var_chi__blk816) + (locals.var_chi__blk816 * locals.var_chi__blk816_dn6)) * locals.var_chi__blk816) + (assign26220_body3_e35848 * locals.var_chi__blk816_dn6)) * assign26220_body3_e35860) + (assign26220_body3_e35850 * ((locals.var_chi__blk816_dn6 * assign26220_body3_e35858) + (locals.var_chi__blk816 * (locals.var_chi__blk816_dn6 * 0.006115288895133179))))), ((((((locals.var_chi__blk816_dn7 * locals.var_chi__blk816) + (locals.var_chi__blk816 * locals.var_chi__blk816_dn7)) * locals.var_chi__blk816) + (assign26220_body3_e35848 * locals.var_chi__blk816_dn7)) * assign26220_body3_e35860) + (assign26220_body3_e35850 * ((locals.var_chi__blk816_dn7 * assign26220_body3_e35858) + (locals.var_chi__blk816 * (locals.var_chi__blk816_dn7 * 0.006115288895133179))))), ((((((locals.var_chi__blk816_dn10 * locals.var_chi__blk816) + (locals.var_chi__blk816 * locals.var_chi__blk816_dn10)) * locals.var_chi__blk816) + (assign26220_body3_e35848 * locals.var_chi__blk816_dn10)) * assign26220_body3_e35860) + (assign26220_body3_e35850 * ((locals.var_chi__blk816_dn10 * assign26220_body3_e35858) + (locals.var_chi__blk816 * (locals.var_chi__blk816_dn10 * 0.006115288895133179))))), ((((((locals.var_chi__blk816_dn11 * locals.var_chi__blk816) + (locals.var_chi__blk816 * locals.var_chi__blk816_dn11)) * locals.var_chi__blk816) + (assign26220_body3_e35848 * locals.var_chi__blk816_dn11)) * assign26220_body3_e35860) + (assign26220_body3_e35850 * ((locals.var_chi__blk816_dn11 * assign26220_body3_e35858) + (locals.var_chi__blk816 * (locals.var_chi__blk816_dn11 * 0.006115288895133179))))), ((((((locals.var_chi__blk816_dn12 * locals.var_chi__blk816) + (locals.var_chi__blk816 * locals.var_chi__blk816_dn12)) * locals.var_chi__blk816) + (assign26220_body3_e35848 * locals.var_chi__blk816_dn12)) * assign26220_body3_e35860) + (assign26220_body3_e35850 * ((locals.var_chi__blk816_dn12 * assign26220_body3_e35858) + (locals.var_chi__blk816 * (locals.var_chi__blk816_dn12 * 0.006115288895133179))))), ((((((locals.var_chi__blk816_dn17 * locals.var_chi__blk816) + (locals.var_chi__blk816 * locals.var_chi__blk816_dn17)) * locals.var_chi__blk816) + (assign26220_body3_e35848 * locals.var_chi__blk816_dn17)) * assign26220_body3_e35860) + (assign26220_body3_e35850 * ((locals.var_chi__blk816_dn17 * assign26220_body3_e35858) + (locals.var_chi__blk816 * (locals.var_chi__blk816_dn17 * 0.006115288895133179))))),)
    } else {
        (locals.var_fi, locals.var_fi_dn0, locals.var_fi_dn2, locals.var_fi_dn6, locals.var_fi_dn7, locals.var_fi_dn10, locals.var_fi_dn11, locals.var_fi_dn12, locals.var_fi_dn17,)
    }
};
            locals.var_fi = assign26220_body3_e35863;
            locals.var_fi_dn0 = assign26220_body3_e35863_d_n0;
            locals.var_fi_dn2 = assign26220_body3_e35863_d_n2;
            locals.var_fi_dn6 = assign26220_body3_e35863_d_n6;
            locals.var_fi_dn7 = assign26220_body3_e35863_d_n7;
            locals.var_fi_dn10 = assign26220_body3_e35863_d_n10;
            locals.var_fi_dn11 = assign26220_body3_e35863_d_n11;
            locals.var_fi_dn12 = assign26220_body3_e35863_d_n12;
            locals.var_fi_dn17 = assign26220_body3_e35863_d_n17;
            locals.var_fi_rv = 0.0;
            let (assign26220_body4_e35897, assign26220_body4_e35897_d_n0, assign26220_body4_e35897_d_n2, assign26220_body4_e35897_d_n6, assign26220_body4_e35897_d_n7, assign26220_body4_e35897_d_n10, assign26220_body4_e35897_d_n11, assign26220_body4_e35897_d_n12, assign26220_body4_e35897_d_n17,) = {
    if ((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard861 != 0.0)) && (locals.var_guard862 != 0.0)) {
        let assign26220_body4_e35878: f64 = (locals.var_chi__blk816 * locals.var_chi__blk816);
        let assign26220_body4_e35881: f64 = (3.0 * 0.29693154855771);
        let assign26220_body4_e35885: f64 = (-0.07053654284009761);
        let assign26220_body4_e35886: f64 = (4.0 * assign26220_body4_e35885);
        let assign26220_body4_e35889: f64 = (locals.var_chi__blk816 * 5.0);
        let assign26220_body4_e35891: f64 = (assign26220_body4_e35889 * 0.006115288895133179);
        let assign26220_body4_e35892: f64 = (assign26220_body4_e35886 + assign26220_body4_e35891);
        let assign26220_body4_e35893: f64 = (locals.var_chi__blk816 * assign26220_body4_e35892);
        let assign26220_body4_e35894: f64 = (assign26220_body4_e35881 + assign26220_body4_e35893);
        let assign26220_body4_e35895: f64 = (assign26220_body4_e35878 * assign26220_body4_e35894);
        (assign26220_body4_e35895, ((((locals.var_chi__blk816_dn0 * locals.var_chi__blk816) + (locals.var_chi__blk816 * locals.var_chi__blk816_dn0)) * assign26220_body4_e35894) + (assign26220_body4_e35878 * ((locals.var_chi__blk816_dn0 * assign26220_body4_e35892) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn0 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk816_dn2 * locals.var_chi__blk816) + (locals.var_chi__blk816 * locals.var_chi__blk816_dn2)) * assign26220_body4_e35894) + (assign26220_body4_e35878 * ((locals.var_chi__blk816_dn2 * assign26220_body4_e35892) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn2 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk816_dn6 * locals.var_chi__blk816) + (locals.var_chi__blk816 * locals.var_chi__blk816_dn6)) * assign26220_body4_e35894) + (assign26220_body4_e35878 * ((locals.var_chi__blk816_dn6 * assign26220_body4_e35892) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn6 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk816_dn7 * locals.var_chi__blk816) + (locals.var_chi__blk816 * locals.var_chi__blk816_dn7)) * assign26220_body4_e35894) + (assign26220_body4_e35878 * ((locals.var_chi__blk816_dn7 * assign26220_body4_e35892) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn7 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk816_dn10 * locals.var_chi__blk816) + (locals.var_chi__blk816 * locals.var_chi__blk816_dn10)) * assign26220_body4_e35894) + (assign26220_body4_e35878 * ((locals.var_chi__blk816_dn10 * assign26220_body4_e35892) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn10 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk816_dn11 * locals.var_chi__blk816) + (locals.var_chi__blk816 * locals.var_chi__blk816_dn11)) * assign26220_body4_e35894) + (assign26220_body4_e35878 * ((locals.var_chi__blk816_dn11 * assign26220_body4_e35892) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn11 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk816_dn12 * locals.var_chi__blk816) + (locals.var_chi__blk816 * locals.var_chi__blk816_dn12)) * assign26220_body4_e35894) + (assign26220_body4_e35878 * ((locals.var_chi__blk816_dn12 * assign26220_body4_e35892) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn12 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk816_dn17 * locals.var_chi__blk816) + (locals.var_chi__blk816 * locals.var_chi__blk816_dn17)) * assign26220_body4_e35894) + (assign26220_body4_e35878 * ((locals.var_chi__blk816_dn17 * assign26220_body4_e35892) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn17 * 5.0) * 0.006115288895133179))))),)
    } else {
        (locals.var_fi_dchi, locals.var_fi_dchi_dn0, locals.var_fi_dchi_dn2, locals.var_fi_dchi_dn6, locals.var_fi_dchi_dn7, locals.var_fi_dchi_dn10, locals.var_fi_dchi_dn11, locals.var_fi_dchi_dn12, locals.var_fi_dchi_dn17,)
    }
};
            locals.var_fi_dchi = assign26220_body4_e35897;
            locals.var_fi_dchi_dn0 = assign26220_body4_e35897_d_n0;
            locals.var_fi_dchi_dn2 = assign26220_body4_e35897_d_n2;
            locals.var_fi_dchi_dn6 = assign26220_body4_e35897_d_n6;
            locals.var_fi_dchi_dn7 = assign26220_body4_e35897_d_n7;
            locals.var_fi_dchi_dn10 = assign26220_body4_e35897_d_n10;
            locals.var_fi_dchi_dn11 = assign26220_body4_e35897_d_n11;
            locals.var_fi_dchi_dn12 = assign26220_body4_e35897_d_n12;
            locals.var_fi_dchi_dn17 = assign26220_body4_e35897_d_n17;
            locals.var_fi_dchi_rv = 0.0;
            let (assign26220_body5_e35916, assign26220_body5_e35916_d_n0, assign26220_body5_e35916_d_n2, assign26220_body5_e35916_d_n6, assign26220_body5_e35916_d_n7, assign26220_body5_e35916_d_n10, assign26220_body5_e35916_d_n11, assign26220_body5_e35916_d_n12, assign26220_body5_e35916_d_n17,) = {
    if ((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard861 != 0.0)) && (locals.var_guard862 != 0.0)) {
        let assign26220_body5_e35912: f64 = (locals.var_cfs1__blk844 * locals.var_fi);
        let assign26220_body5_e35914: f64 = (assign26220_body5_e35912 * locals.var_fi);
        (assign26220_body5_e35914, ((((locals.var_cfs1__blk844_dn0 * locals.var_fi) + (locals.var_cfs1__blk844 * locals.var_fi_dn0)) * locals.var_fi) + (assign26220_body5_e35912 * locals.var_fi_dn0)), ((((locals.var_cfs1__blk844_dn2 * locals.var_fi) + (locals.var_cfs1__blk844 * locals.var_fi_dn2)) * locals.var_fi) + (assign26220_body5_e35912 * locals.var_fi_dn2)), ((((locals.var_cfs1__blk844_dn6 * locals.var_fi) + (locals.var_cfs1__blk844 * locals.var_fi_dn6)) * locals.var_fi) + (assign26220_body5_e35912 * locals.var_fi_dn6)), ((((locals.var_cfs1__blk844_dn7 * locals.var_fi) + (locals.var_cfs1__blk844 * locals.var_fi_dn7)) * locals.var_fi) + (assign26220_body5_e35912 * locals.var_fi_dn7)), ((((locals.var_cfs1__blk844_dn10 * locals.var_fi) + (locals.var_cfs1__blk844 * locals.var_fi_dn10)) * locals.var_fi) + (assign26220_body5_e35912 * locals.var_fi_dn10)), ((((locals.var_cfs1__blk844_dn11 * locals.var_fi) + (locals.var_cfs1__blk844 * locals.var_fi_dn11)) * locals.var_fi) + (assign26220_body5_e35912 * locals.var_fi_dn11)), ((((locals.var_cfs1__blk844_dn12 * locals.var_fi) + (locals.var_cfs1__blk844 * locals.var_fi_dn12)) * locals.var_fi) + (assign26220_body5_e35912 * locals.var_fi_dn12)), ((((locals.var_cfs1__blk844_dn17 * locals.var_fi) + (locals.var_cfs1__blk844 * locals.var_fi_dn17)) * locals.var_fi) + (assign26220_body5_e35912 * locals.var_fi_dn17)),)
    } else {
        (locals.var_fs01__blk838, locals.var_fs01__blk838_dn0, locals.var_fs01__blk838_dn2, locals.var_fs01__blk838_dn6, locals.var_fs01__blk838_dn7, locals.var_fs01__blk838_dn10, locals.var_fs01__blk838_dn11, locals.var_fs01__blk838_dn12, locals.var_fs01__blk838_dn17,)
    }
};
            locals.var_fs01__blk838 = assign26220_body5_e35916;
            locals.var_fs01__blk838_dn0 = assign26220_body5_e35916_d_n0;
            locals.var_fs01__blk838_dn2 = assign26220_body5_e35916_d_n2;
            locals.var_fs01__blk838_dn6 = assign26220_body5_e35916_d_n6;
            locals.var_fs01__blk838_dn7 = assign26220_body5_e35916_d_n7;
            locals.var_fs01__blk838_dn10 = assign26220_body5_e35916_d_n10;
            locals.var_fs01__blk838_dn11 = assign26220_body5_e35916_d_n11;
            locals.var_fs01__blk838_dn12 = assign26220_body5_e35916_d_n12;
            locals.var_fs01__blk838_dn17 = assign26220_body5_e35916_d_n17;
            locals.var_fs01__blk838_rv = 0.0;
            let (assign26220_body6_e35939, assign26220_body6_e35939_d_n0, assign26220_body6_e35939_d_n2, assign26220_body6_e35939_d_n6, assign26220_body6_e35939_d_n7, assign26220_body6_e35939_d_n10, assign26220_body6_e35939_d_n11, assign26220_body6_e35939_d_n12, assign26220_body6_e35939_d_n17,) = {
    if ((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard861 != 0.0)) && (locals.var_guard862 != 0.0)) {
        let assign26220_body6_e35931: f64 = (locals.var_cfs1__blk844 * locals.var_beta);
        let assign26220_body6_e35933: f64 = (assign26220_body6_e35931 * 2.0);
        let assign26220_body6_e35935: f64 = (assign26220_body6_e35933 * locals.var_fi);
        let assign26220_body6_e35937: f64 = (assign26220_body6_e35935 * locals.var_fi_dchi);
        (assign26220_body6_e35937, ((((((locals.var_cfs1__blk844_dn0 * locals.var_beta) * 2.0) * locals.var_fi) + (assign26220_body6_e35933 * locals.var_fi_dn0)) * locals.var_fi_dchi) + (assign26220_body6_e35935 * locals.var_fi_dchi_dn0)), ((((((locals.var_cfs1__blk844_dn2 * locals.var_beta) * 2.0) * locals.var_fi) + (assign26220_body6_e35933 * locals.var_fi_dn2)) * locals.var_fi_dchi) + (assign26220_body6_e35935 * locals.var_fi_dchi_dn2)), ((((((locals.var_cfs1__blk844_dn6 * locals.var_beta) * 2.0) * locals.var_fi) + (assign26220_body6_e35933 * locals.var_fi_dn6)) * locals.var_fi_dchi) + (assign26220_body6_e35935 * locals.var_fi_dchi_dn6)), ((((((locals.var_cfs1__blk844_dn7 * locals.var_beta) * 2.0) * locals.var_fi) + (assign26220_body6_e35933 * locals.var_fi_dn7)) * locals.var_fi_dchi) + (assign26220_body6_e35935 * locals.var_fi_dchi_dn7)), (((((((locals.var_cfs1__blk844_dn10 * locals.var_beta) + (locals.var_cfs1__blk844 * locals.var_beta_dn10)) * 2.0) * locals.var_fi) + (assign26220_body6_e35933 * locals.var_fi_dn10)) * locals.var_fi_dchi) + (assign26220_body6_e35935 * locals.var_fi_dchi_dn10)), ((((((locals.var_cfs1__blk844_dn11 * locals.var_beta) * 2.0) * locals.var_fi) + (assign26220_body6_e35933 * locals.var_fi_dn11)) * locals.var_fi_dchi) + (assign26220_body6_e35935 * locals.var_fi_dchi_dn11)), ((((((locals.var_cfs1__blk844_dn12 * locals.var_beta) * 2.0) * locals.var_fi) + (assign26220_body6_e35933 * locals.var_fi_dn12)) * locals.var_fi_dchi) + (assign26220_body6_e35935 * locals.var_fi_dchi_dn12)), ((((((locals.var_cfs1__blk844_dn17 * locals.var_beta) * 2.0) * locals.var_fi) + (assign26220_body6_e35933 * locals.var_fi_dn17)) * locals.var_fi_dchi) + (assign26220_body6_e35935 * locals.var_fi_dchi_dn17)),)
    } else {
        (locals.var_fs01_dps0__blk839, locals.var_fs01_dps0__blk839_dn0, locals.var_fs01_dps0__blk839_dn2, locals.var_fs01_dps0__blk839_dn6, locals.var_fs01_dps0__blk839_dn7, locals.var_fs01_dps0__blk839_dn10, locals.var_fs01_dps0__blk839_dn11, locals.var_fs01_dps0__blk839_dn12, locals.var_fs01_dps0__blk839_dn17,)
    }
};
            locals.var_fs01_dps0__blk839 = assign26220_body6_e35939;
            locals.var_fs01_dps0__blk839_dn0 = assign26220_body6_e35939_d_n0;
            locals.var_fs01_dps0__blk839_dn2 = assign26220_body6_e35939_d_n2;
            locals.var_fs01_dps0__blk839_dn6 = assign26220_body6_e35939_d_n6;
            locals.var_fs01_dps0__blk839_dn7 = assign26220_body6_e35939_d_n7;
            locals.var_fs01_dps0__blk839_dn10 = assign26220_body6_e35939_d_n10;
            locals.var_fs01_dps0__blk839_dn11 = assign26220_body6_e35939_d_n11;
            locals.var_fs01_dps0__blk839_dn12 = assign26220_body6_e35939_d_n12;
            locals.var_fs01_dps0__blk839_dn17 = assign26220_body6_e35939_d_n17;
            locals.var_fs01_dps0__blk839_rv = 0.0;
            let (assign26220_body7_e35974, assign26220_body7_e35974_d_n0, assign26220_body7_e35974_d_n2, assign26220_body7_e35974_d_n6, assign26220_body7_e35974_d_n7, assign26220_body7_e35974_d_n10, assign26220_body7_e35974_d_n11, assign26220_body7_e35974_d_n12, assign26220_body7_e35974_d_n17,) = {
    if ((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard861 != 0.0)) && (locals.var_guard862 != 0.0)) {
        let assign26220_body7_e35956: f64 = (-0.117851130197758);
        let assign26220_body7_e35961: f64 = (-0.00163730162779191);
        let assign26220_body7_e35964: f64 = (locals.var_chi__blk816 * 6.36964918866352e-5);
        let assign26220_body7_e35965: f64 = (assign26220_body7_e35961 + assign26220_body7_e35964);
        let assign26220_body7_e35966: f64 = (locals.var_chi__blk816 * assign26220_body7_e35965);
        let assign26220_body7_e35967: f64 = (0.0178800506338833 + assign26220_body7_e35966);
        let assign26220_body7_e35968: f64 = (locals.var_chi__blk816 * assign26220_body7_e35967);
        let assign26220_body7_e35969: f64 = (assign26220_body7_e35956 + assign26220_body7_e35968);
        let assign26220_body7_e35970: f64 = (locals.var_chi__blk816 * assign26220_body7_e35969);
        let assign26220_body7_e35971: f64 = (0.707106781186548 + assign26220_body7_e35970);
        let assign26220_body7_e35972: f64 = (locals.var_chi__blk816 * assign26220_body7_e35971);
        (assign26220_body7_e35972, ((locals.var_chi__blk816_dn0 * assign26220_body7_e35971) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn0 * assign26220_body7_e35969) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn0 * assign26220_body7_e35967) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn0 * assign26220_body7_e35965) + (locals.var_chi__blk816 * (locals.var_chi__blk816_dn0 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk816_dn2 * assign26220_body7_e35971) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn2 * assign26220_body7_e35969) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn2 * assign26220_body7_e35967) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn2 * assign26220_body7_e35965) + (locals.var_chi__blk816 * (locals.var_chi__blk816_dn2 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk816_dn6 * assign26220_body7_e35971) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn6 * assign26220_body7_e35969) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn6 * assign26220_body7_e35967) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn6 * assign26220_body7_e35965) + (locals.var_chi__blk816 * (locals.var_chi__blk816_dn6 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk816_dn7 * assign26220_body7_e35971) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn7 * assign26220_body7_e35969) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn7 * assign26220_body7_e35967) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn7 * assign26220_body7_e35965) + (locals.var_chi__blk816 * (locals.var_chi__blk816_dn7 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk816_dn10 * assign26220_body7_e35971) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn10 * assign26220_body7_e35969) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn10 * assign26220_body7_e35967) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn10 * assign26220_body7_e35965) + (locals.var_chi__blk816 * (locals.var_chi__blk816_dn10 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk816_dn11 * assign26220_body7_e35971) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn11 * assign26220_body7_e35969) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn11 * assign26220_body7_e35967) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn11 * assign26220_body7_e35965) + (locals.var_chi__blk816 * (locals.var_chi__blk816_dn11 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk816_dn12 * assign26220_body7_e35971) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn12 * assign26220_body7_e35969) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn12 * assign26220_body7_e35967) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn12 * assign26220_body7_e35965) + (locals.var_chi__blk816 * (locals.var_chi__blk816_dn12 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk816_dn17 * assign26220_body7_e35971) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn17 * assign26220_body7_e35969) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn17 * assign26220_body7_e35967) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn17 * assign26220_body7_e35965) + (locals.var_chi__blk816 * (locals.var_chi__blk816_dn17 * 6.36964918866352e-5))))))))),)
    } else {
        (locals.var_fb__blk840, locals.var_fb__blk840_dn0, locals.var_fb__blk840_dn2, locals.var_fb__blk840_dn6, locals.var_fb__blk840_dn7, locals.var_fb__blk840_dn10, locals.var_fb__blk840_dn11, locals.var_fb__blk840_dn12, locals.var_fb__blk840_dn17,)
    }
};
            locals.var_fb__blk840 = assign26220_body7_e35974;
            locals.var_fb__blk840_dn0 = assign26220_body7_e35974_d_n0;
            locals.var_fb__blk840_dn2 = assign26220_body7_e35974_d_n2;
            locals.var_fb__blk840_dn6 = assign26220_body7_e35974_d_n6;
            locals.var_fb__blk840_dn7 = assign26220_body7_e35974_d_n7;
            locals.var_fb__blk840_dn10 = assign26220_body7_e35974_d_n10;
            locals.var_fb__blk840_dn11 = assign26220_body7_e35974_d_n11;
            locals.var_fb__blk840_dn12 = assign26220_body7_e35974_d_n12;
            locals.var_fb__blk840_dn17 = assign26220_body7_e35974_d_n17;
            locals.var_fb__blk840_rv = 0.0;
            let (assign26220_body8_e36015, assign26220_body8_e36015_d_n0, assign26220_body8_e36015_d_n2, assign26220_body8_e36015_d_n6, assign26220_body8_e36015_d_n7, assign26220_body8_e36015_d_n10, assign26220_body8_e36015_d_n11, assign26220_body8_e36015_d_n12, assign26220_body8_e36015_d_n17,) = {
    if ((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard861 != 0.0)) && (locals.var_guard862 != 0.0)) {
        let assign26220_body8_e35991: f64 = (-0.117851130197758);
        let assign26220_body8_e35992: f64 = (2.0 * assign26220_body8_e35991);
        let assign26220_body8_e35996: f64 = (3.0 * 0.0178800506338833);
        let assign26220_body8_e36000: f64 = (-0.00163730162779191);
        let assign26220_body8_e36001: f64 = (4.0 * assign26220_body8_e36000);
        let assign26220_body8_e36004: f64 = (locals.var_chi__blk816 * 5.0);
        let assign26220_body8_e36006: f64 = (assign26220_body8_e36004 * 6.36964918866352e-5);
        let assign26220_body8_e36007: f64 = (assign26220_body8_e36001 + assign26220_body8_e36006);
        let assign26220_body8_e36008: f64 = (locals.var_chi__blk816 * assign26220_body8_e36007);
        let assign26220_body8_e36009: f64 = (assign26220_body8_e35996 + assign26220_body8_e36008);
        let assign26220_body8_e36010: f64 = (locals.var_chi__blk816 * assign26220_body8_e36009);
        let assign26220_body8_e36011: f64 = (assign26220_body8_e35992 + assign26220_body8_e36010);
        let assign26220_body8_e36012: f64 = (locals.var_chi__blk816 * assign26220_body8_e36011);
        let assign26220_body8_e36013: f64 = (0.707106781186548 + assign26220_body8_e36012);
        (assign26220_body8_e36013, ((locals.var_chi__blk816_dn0 * assign26220_body8_e36011) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn0 * assign26220_body8_e36009) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn0 * assign26220_body8_e36007) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn0 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk816_dn2 * assign26220_body8_e36011) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn2 * assign26220_body8_e36009) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn2 * assign26220_body8_e36007) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn2 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk816_dn6 * assign26220_body8_e36011) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn6 * assign26220_body8_e36009) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn6 * assign26220_body8_e36007) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn6 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk816_dn7 * assign26220_body8_e36011) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn7 * assign26220_body8_e36009) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn7 * assign26220_body8_e36007) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn7 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk816_dn10 * assign26220_body8_e36011) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn10 * assign26220_body8_e36009) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn10 * assign26220_body8_e36007) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn10 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk816_dn11 * assign26220_body8_e36011) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn11 * assign26220_body8_e36009) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn11 * assign26220_body8_e36007) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn11 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk816_dn12 * assign26220_body8_e36011) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn12 * assign26220_body8_e36009) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn12 * assign26220_body8_e36007) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn12 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk816_dn17 * assign26220_body8_e36011) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn17 * assign26220_body8_e36009) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn17 * assign26220_body8_e36007) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn17 * 5.0) * 6.36964918866352e-5))))))),)
    } else {
        (locals.var_fb_dchi, locals.var_fb_dchi_dn0, locals.var_fb_dchi_dn2, locals.var_fb_dchi_dn6, locals.var_fb_dchi_dn7, locals.var_fb_dchi_dn10, locals.var_fb_dchi_dn11, locals.var_fb_dchi_dn12, locals.var_fb_dchi_dn17,)
    }
};
            locals.var_fb_dchi = assign26220_body8_e36015;
            locals.var_fb_dchi_dn0 = assign26220_body8_e36015_d_n0;
            locals.var_fb_dchi_dn2 = assign26220_body8_e36015_d_n2;
            locals.var_fb_dchi_dn6 = assign26220_body8_e36015_d_n6;
            locals.var_fb_dchi_dn7 = assign26220_body8_e36015_d_n7;
            locals.var_fb_dchi_dn10 = assign26220_body8_e36015_d_n10;
            locals.var_fb_dchi_dn11 = assign26220_body8_e36015_d_n11;
            locals.var_fb_dchi_dn12 = assign26220_body8_e36015_d_n12;
            locals.var_fb_dchi_dn17 = assign26220_body8_e36015_d_n17;
            locals.var_fb_dchi_rv = 0.0;
            let (assign26220_body9_e36037, assign26220_body9_e36037_d_n0, assign26220_body9_e36037_d_n2, assign26220_body9_e36037_d_n6, assign26220_body9_e36037_d_n7, assign26220_body9_e36037_d_n10, assign26220_body9_e36037_d_n11, assign26220_body9_e36037_d_n12, assign26220_body9_e36037_d_n17,) = {
    if ((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard861 != 0.0)) && (locals.var_guard862 != 0.0)) {
        let assign26220_body9_e36030: f64 = (locals.var_fb__blk840 * locals.var_fb__blk840);
        let assign26220_body9_e36032: f64 = (assign26220_body9_e36030 + locals.var_fs01__blk838);
        let assign26220_body9_e36034: f64 = (assign26220_body9_e36032 + 1e-50);
        let assign26220_body9_e36035: f64 = (assign26220_body9_e36034).sqrt();
        (assign26220_body9_e36035, ((((locals.var_fb__blk840_dn0 * locals.var_fb__blk840) + (locals.var_fb__blk840 * locals.var_fb__blk840_dn0)) + locals.var_fs01__blk838_dn0) / (2.0 * assign26220_body9_e36035)), ((((locals.var_fb__blk840_dn2 * locals.var_fb__blk840) + (locals.var_fb__blk840 * locals.var_fb__blk840_dn2)) + locals.var_fs01__blk838_dn2) / (2.0 * assign26220_body9_e36035)), ((((locals.var_fb__blk840_dn6 * locals.var_fb__blk840) + (locals.var_fb__blk840 * locals.var_fb__blk840_dn6)) + locals.var_fs01__blk838_dn6) / (2.0 * assign26220_body9_e36035)), ((((locals.var_fb__blk840_dn7 * locals.var_fb__blk840) + (locals.var_fb__blk840 * locals.var_fb__blk840_dn7)) + locals.var_fs01__blk838_dn7) / (2.0 * assign26220_body9_e36035)), ((((locals.var_fb__blk840_dn10 * locals.var_fb__blk840) + (locals.var_fb__blk840 * locals.var_fb__blk840_dn10)) + locals.var_fs01__blk838_dn10) / (2.0 * assign26220_body9_e36035)), ((((locals.var_fb__blk840_dn11 * locals.var_fb__blk840) + (locals.var_fb__blk840 * locals.var_fb__blk840_dn11)) + locals.var_fs01__blk838_dn11) / (2.0 * assign26220_body9_e36035)), ((((locals.var_fb__blk840_dn12 * locals.var_fb__blk840) + (locals.var_fb__blk840 * locals.var_fb__blk840_dn12)) + locals.var_fs01__blk838_dn12) / (2.0 * assign26220_body9_e36035)), ((((locals.var_fb__blk840_dn17 * locals.var_fb__blk840) + (locals.var_fb__blk840 * locals.var_fb__blk840_dn17)) + locals.var_fs01__blk838_dn17) / (2.0 * assign26220_body9_e36035)),)
    } else {
        (locals.var_fs02__blk842, locals.var_fs02__blk842_dn0, locals.var_fs02__blk842_dn2, locals.var_fs02__blk842_dn6, locals.var_fs02__blk842_dn7, locals.var_fs02__blk842_dn10, locals.var_fs02__blk842_dn11, locals.var_fs02__blk842_dn12, locals.var_fs02__blk842_dn17,)
    }
};
            locals.var_fs02__blk842 = assign26220_body9_e36037;
            locals.var_fs02__blk842_dn0 = assign26220_body9_e36037_d_n0;
            locals.var_fs02__blk842_dn2 = assign26220_body9_e36037_d_n2;
            locals.var_fs02__blk842_dn6 = assign26220_body9_e36037_d_n6;
            locals.var_fs02__blk842_dn7 = assign26220_body9_e36037_d_n7;
            locals.var_fs02__blk842_dn10 = assign26220_body9_e36037_d_n10;
            locals.var_fs02__blk842_dn11 = assign26220_body9_e36037_d_n11;
            locals.var_fs02__blk842_dn12 = assign26220_body9_e36037_d_n12;
            locals.var_fs02__blk842_dn17 = assign26220_body9_e36037_d_n17;
            locals.var_fs02__blk842_rv = 0.0;
            let (assign26220_body10_e36064, assign26220_body10_e36064_d_n0, assign26220_body10_e36064_d_n2, assign26220_body10_e36064_d_n6, assign26220_body10_e36064_d_n7, assign26220_body10_e36064_d_n10, assign26220_body10_e36064_d_n11, assign26220_body10_e36064_d_n12, assign26220_body10_e36064_d_n17,) = {
    if ((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard861 != 0.0)) && (locals.var_guard862 != 0.0)) {
        let assign26220_body10_e36052: f64 = (locals.var_beta * locals.var_fb_dchi);
        let assign26220_body10_e36054: f64 = (assign26220_body10_e36052 * 2.0);
        let assign26220_body10_e36056: f64 = (assign26220_body10_e36054 * locals.var_fb__blk840);
        let assign26220_body10_e36058: f64 = (assign26220_body10_e36056 + locals.var_fs01_dps0__blk839);
        let assign26220_body10_e36061: f64 = (locals.var_fs02__blk842 + locals.var_fs02__blk842);
        let assign26220_body10_e36062: f64 = (assign26220_body10_e36058 / assign26220_body10_e36061);
        (assign26220_body10_e36062, ((((((((locals.var_beta * locals.var_fb_dchi_dn0) * 2.0) * locals.var_fb__blk840) + (assign26220_body10_e36054 * locals.var_fb__blk840_dn0)) + locals.var_fs01_dps0__blk839_dn0) * assign26220_body10_e36061) - (assign26220_body10_e36058 * (locals.var_fs02__blk842_dn0 + locals.var_fs02__blk842_dn0))) / (assign26220_body10_e36061 * assign26220_body10_e36061)), ((((((((locals.var_beta * locals.var_fb_dchi_dn2) * 2.0) * locals.var_fb__blk840) + (assign26220_body10_e36054 * locals.var_fb__blk840_dn2)) + locals.var_fs01_dps0__blk839_dn2) * assign26220_body10_e36061) - (assign26220_body10_e36058 * (locals.var_fs02__blk842_dn2 + locals.var_fs02__blk842_dn2))) / (assign26220_body10_e36061 * assign26220_body10_e36061)), ((((((((locals.var_beta * locals.var_fb_dchi_dn6) * 2.0) * locals.var_fb__blk840) + (assign26220_body10_e36054 * locals.var_fb__blk840_dn6)) + locals.var_fs01_dps0__blk839_dn6) * assign26220_body10_e36061) - (assign26220_body10_e36058 * (locals.var_fs02__blk842_dn6 + locals.var_fs02__blk842_dn6))) / (assign26220_body10_e36061 * assign26220_body10_e36061)), ((((((((locals.var_beta * locals.var_fb_dchi_dn7) * 2.0) * locals.var_fb__blk840) + (assign26220_body10_e36054 * locals.var_fb__blk840_dn7)) + locals.var_fs01_dps0__blk839_dn7) * assign26220_body10_e36061) - (assign26220_body10_e36058 * (locals.var_fs02__blk842_dn7 + locals.var_fs02__blk842_dn7))) / (assign26220_body10_e36061 * assign26220_body10_e36061)), (((((((((locals.var_beta_dn10 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn10)) * 2.0) * locals.var_fb__blk840) + (assign26220_body10_e36054 * locals.var_fb__blk840_dn10)) + locals.var_fs01_dps0__blk839_dn10) * assign26220_body10_e36061) - (assign26220_body10_e36058 * (locals.var_fs02__blk842_dn10 + locals.var_fs02__blk842_dn10))) / (assign26220_body10_e36061 * assign26220_body10_e36061)), ((((((((locals.var_beta * locals.var_fb_dchi_dn11) * 2.0) * locals.var_fb__blk840) + (assign26220_body10_e36054 * locals.var_fb__blk840_dn11)) + locals.var_fs01_dps0__blk839_dn11) * assign26220_body10_e36061) - (assign26220_body10_e36058 * (locals.var_fs02__blk842_dn11 + locals.var_fs02__blk842_dn11))) / (assign26220_body10_e36061 * assign26220_body10_e36061)), ((((((((locals.var_beta * locals.var_fb_dchi_dn12) * 2.0) * locals.var_fb__blk840) + (assign26220_body10_e36054 * locals.var_fb__blk840_dn12)) + locals.var_fs01_dps0__blk839_dn12) * assign26220_body10_e36061) - (assign26220_body10_e36058 * (locals.var_fs02__blk842_dn12 + locals.var_fs02__blk842_dn12))) / (assign26220_body10_e36061 * assign26220_body10_e36061)), ((((((((locals.var_beta * locals.var_fb_dchi_dn17) * 2.0) * locals.var_fb__blk840) + (assign26220_body10_e36054 * locals.var_fb__blk840_dn17)) + locals.var_fs01_dps0__blk839_dn17) * assign26220_body10_e36061) - (assign26220_body10_e36058 * (locals.var_fs02__blk842_dn17 + locals.var_fs02__blk842_dn17))) / (assign26220_body10_e36061 * assign26220_body10_e36061)),)
    } else {
        (locals.var_fs02_dps0__blk843, locals.var_fs02_dps0__blk843_dn0, locals.var_fs02_dps0__blk843_dn2, locals.var_fs02_dps0__blk843_dn6, locals.var_fs02_dps0__blk843_dn7, locals.var_fs02_dps0__blk843_dn10, locals.var_fs02_dps0__blk843_dn11, locals.var_fs02_dps0__blk843_dn12, locals.var_fs02_dps0__blk843_dn17,)
    }
};
            locals.var_fs02_dps0__blk843 = assign26220_body10_e36064;
            locals.var_fs02_dps0__blk843_dn0 = assign26220_body10_e36064_d_n0;
            locals.var_fs02_dps0__blk843_dn2 = assign26220_body10_e36064_d_n2;
            locals.var_fs02_dps0__blk843_dn6 = assign26220_body10_e36064_d_n6;
            locals.var_fs02_dps0__blk843_dn7 = assign26220_body10_e36064_d_n7;
            locals.var_fs02_dps0__blk843_dn10 = assign26220_body10_e36064_d_n10;
            locals.var_fs02_dps0__blk843_dn11 = assign26220_body10_e36064_d_n11;
            locals.var_fs02_dps0__blk843_dn12 = assign26220_body10_e36064_d_n12;
            locals.var_fs02_dps0__blk843_dn17 = assign26220_body10_e36064_d_n17;
            locals.var_fs02_dps0__blk843_rv = 0.0;
            let assign26220_body11_e36067: f64 = if locals.var_chi__blk816 < 80.0 { 1.0 } else { 0.0 };
            locals.var_guard863 = assign26220_body11_e36067;
            locals.var_guard863_rv = 0.0;
            let (assign26220_body12_e36086, assign26220_body12_e36086_d_n0, assign26220_body12_e36086_d_n2, assign26220_body12_e36086_d_n6, assign26220_body12_e36086_d_n7, assign26220_body12_e36086_d_n10, assign26220_body12_e36086_d_n11, assign26220_body12_e36086_d_n12, assign26220_body12_e36086_d_n17,) = {
    if (((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard861 != 0.0)) && (locals.var_guard862 == 0.0)) && (locals.var_guard863 != 0.0)) {
        let assign26220_body12_e36084: f64 = (locals.var_chi__blk816).exp();
        (assign26220_body12_e36084, (assign26220_body12_e36084 * locals.var_chi__blk816_dn0), (assign26220_body12_e36084 * locals.var_chi__blk816_dn2), (assign26220_body12_e36084 * locals.var_chi__blk816_dn6), (assign26220_body12_e36084 * locals.var_chi__blk816_dn7), (assign26220_body12_e36084 * locals.var_chi__blk816_dn10), (assign26220_body12_e36084 * locals.var_chi__blk816_dn11), (assign26220_body12_e36084 * locals.var_chi__blk816_dn12), (assign26220_body12_e36084 * locals.var_chi__blk816_dn17),)
    } else {
        (locals.var_exp_chi, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn10, locals.var_exp_chi_dn11, locals.var_exp_chi_dn12, locals.var_exp_chi_dn17,)
    }
};
            locals.var_exp_chi = assign26220_body12_e36086;
            locals.var_exp_chi_dn0 = assign26220_body12_e36086_d_n0;
            locals.var_exp_chi_dn2 = assign26220_body12_e36086_d_n2;
            locals.var_exp_chi_dn6 = assign26220_body12_e36086_d_n6;
            locals.var_exp_chi_dn7 = assign26220_body12_e36086_d_n7;
            locals.var_exp_chi_dn10 = assign26220_body12_e36086_d_n10;
            locals.var_exp_chi_dn11 = assign26220_body12_e36086_d_n11;
            locals.var_exp_chi_dn12 = assign26220_body12_e36086_d_n12;
            locals.var_exp_chi_dn17 = assign26220_body12_e36086_d_n17;
            locals.var_exp_chi_rv = 0.0;
            let (assign26220_body13_e36108, assign26220_body13_e36108_d_n0, assign26220_body13_e36108_d_n2, assign26220_body13_e36108_d_n6, assign26220_body13_e36108_d_n7, assign26220_body13_e36108_d_n10, assign26220_body13_e36108_d_n11, assign26220_body13_e36108_d_n12, assign26220_body13_e36108_d_n17,) = {
    if (((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard861 != 0.0)) && (locals.var_guard862 == 0.0)) && (locals.var_guard863 != 0.0)) {
        let assign26220_body13_e36105: f64 = (locals.var_exp_chi - 1.0);
        let assign26220_body13_e36106: f64 = (locals.var_cfs1__blk844 * assign26220_body13_e36105);
        (assign26220_body13_e36106, ((locals.var_cfs1__blk844_dn0 * assign26220_body13_e36105) + (locals.var_cfs1__blk844 * locals.var_exp_chi_dn0)), ((locals.var_cfs1__blk844_dn2 * assign26220_body13_e36105) + (locals.var_cfs1__blk844 * locals.var_exp_chi_dn2)), ((locals.var_cfs1__blk844_dn6 * assign26220_body13_e36105) + (locals.var_cfs1__blk844 * locals.var_exp_chi_dn6)), ((locals.var_cfs1__blk844_dn7 * assign26220_body13_e36105) + (locals.var_cfs1__blk844 * locals.var_exp_chi_dn7)), ((locals.var_cfs1__blk844_dn10 * assign26220_body13_e36105) + (locals.var_cfs1__blk844 * locals.var_exp_chi_dn10)), ((locals.var_cfs1__blk844_dn11 * assign26220_body13_e36105) + (locals.var_cfs1__blk844 * locals.var_exp_chi_dn11)), ((locals.var_cfs1__blk844_dn12 * assign26220_body13_e36105) + (locals.var_cfs1__blk844 * locals.var_exp_chi_dn12)), ((locals.var_cfs1__blk844_dn17 * assign26220_body13_e36105) + (locals.var_cfs1__blk844 * locals.var_exp_chi_dn17)),)
    } else {
        (locals.var_fs01__blk838, locals.var_fs01__blk838_dn0, locals.var_fs01__blk838_dn2, locals.var_fs01__blk838_dn6, locals.var_fs01__blk838_dn7, locals.var_fs01__blk838_dn10, locals.var_fs01__blk838_dn11, locals.var_fs01__blk838_dn12, locals.var_fs01__blk838_dn17,)
    }
};
            locals.var_fs01__blk838 = assign26220_body13_e36108;
            locals.var_fs01__blk838_dn0 = assign26220_body13_e36108_d_n0;
            locals.var_fs01__blk838_dn2 = assign26220_body13_e36108_d_n2;
            locals.var_fs01__blk838_dn6 = assign26220_body13_e36108_d_n6;
            locals.var_fs01__blk838_dn7 = assign26220_body13_e36108_d_n7;
            locals.var_fs01__blk838_dn10 = assign26220_body13_e36108_d_n10;
            locals.var_fs01__blk838_dn11 = assign26220_body13_e36108_d_n11;
            locals.var_fs01__blk838_dn12 = assign26220_body13_e36108_d_n12;
            locals.var_fs01__blk838_dn17 = assign26220_body13_e36108_d_n17;
            locals.var_fs01__blk838_rv = 0.0;
            let (assign26220_body14_e36130, assign26220_body14_e36130_d_n0, assign26220_body14_e36130_d_n2, assign26220_body14_e36130_d_n6, assign26220_body14_e36130_d_n7, assign26220_body14_e36130_d_n10, assign26220_body14_e36130_d_n11, assign26220_body14_e36130_d_n12, assign26220_body14_e36130_d_n17,) = {
    if (((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard861 != 0.0)) && (locals.var_guard862 == 0.0)) && (locals.var_guard863 != 0.0)) {
        let assign26220_body14_e36126: f64 = (locals.var_cfs1__blk844 * locals.var_beta);
        let assign26220_body14_e36128: f64 = (assign26220_body14_e36126 * locals.var_exp_chi);
        (assign26220_body14_e36128, (((locals.var_cfs1__blk844_dn0 * locals.var_beta) * locals.var_exp_chi) + (assign26220_body14_e36126 * locals.var_exp_chi_dn0)), (((locals.var_cfs1__blk844_dn2 * locals.var_beta) * locals.var_exp_chi) + (assign26220_body14_e36126 * locals.var_exp_chi_dn2)), (((locals.var_cfs1__blk844_dn6 * locals.var_beta) * locals.var_exp_chi) + (assign26220_body14_e36126 * locals.var_exp_chi_dn6)), (((locals.var_cfs1__blk844_dn7 * locals.var_beta) * locals.var_exp_chi) + (assign26220_body14_e36126 * locals.var_exp_chi_dn7)), ((((locals.var_cfs1__blk844_dn10 * locals.var_beta) + (locals.var_cfs1__blk844 * locals.var_beta_dn10)) * locals.var_exp_chi) + (assign26220_body14_e36126 * locals.var_exp_chi_dn10)), (((locals.var_cfs1__blk844_dn11 * locals.var_beta) * locals.var_exp_chi) + (assign26220_body14_e36126 * locals.var_exp_chi_dn11)), (((locals.var_cfs1__blk844_dn12 * locals.var_beta) * locals.var_exp_chi) + (assign26220_body14_e36126 * locals.var_exp_chi_dn12)), (((locals.var_cfs1__blk844_dn17 * locals.var_beta) * locals.var_exp_chi) + (assign26220_body14_e36126 * locals.var_exp_chi_dn17)),)
    } else {
        (locals.var_fs01_dps0__blk839, locals.var_fs01_dps0__blk839_dn0, locals.var_fs01_dps0__blk839_dn2, locals.var_fs01_dps0__blk839_dn6, locals.var_fs01_dps0__blk839_dn7, locals.var_fs01_dps0__blk839_dn10, locals.var_fs01_dps0__blk839_dn11, locals.var_fs01_dps0__blk839_dn12, locals.var_fs01_dps0__blk839_dn17,)
    }
};
            locals.var_fs01_dps0__blk839 = assign26220_body14_e36130;
            locals.var_fs01_dps0__blk839_dn0 = assign26220_body14_e36130_d_n0;
            locals.var_fs01_dps0__blk839_dn2 = assign26220_body14_e36130_d_n2;
            locals.var_fs01_dps0__blk839_dn6 = assign26220_body14_e36130_d_n6;
            locals.var_fs01_dps0__blk839_dn7 = assign26220_body14_e36130_d_n7;
            locals.var_fs01_dps0__blk839_dn10 = assign26220_body14_e36130_d_n10;
            locals.var_fs01_dps0__blk839_dn11 = assign26220_body14_e36130_d_n11;
            locals.var_fs01_dps0__blk839_dn12 = assign26220_body14_e36130_d_n12;
            locals.var_fs01_dps0__blk839_dn17 = assign26220_body14_e36130_d_n17;
            locals.var_fs01_dps0__blk839_rv = 0.0;
            let (assign26220_body15_e36152, assign26220_body15_e36152_d_n0, assign26220_body15_e36152_d_n2, assign26220_body15_e36152_d_n6, assign26220_body15_e36152_d_n7, assign26220_body15_e36152_d_n10, assign26220_body15_e36152_d_n11, assign26220_body15_e36152_d_n12, assign26220_body15_e36152_d_n17,) = {
    if (((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard861 != 0.0)) && (locals.var_guard862 == 0.0)) && (locals.var_guard863 == 0.0)) {
        let assign26220_body15_e36149: f64 = (locals.var_beta * locals.var_ps0ld);
        let assign26220_body15_e36150: f64 = (assign26220_body15_e36149).exp();
        (assign26220_body15_e36150, (assign26220_body15_e36150 * (locals.var_beta * locals.var_ps0ld_dn0)), (assign26220_body15_e36150 * (locals.var_beta * locals.var_ps0ld_dn2)), (assign26220_body15_e36150 * (locals.var_beta * locals.var_ps0ld_dn6)), (assign26220_body15_e36150 * (locals.var_beta * locals.var_ps0ld_dn7)), (assign26220_body15_e36150 * ((locals.var_beta_dn10 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn10))), (assign26220_body15_e36150 * (locals.var_beta * locals.var_ps0ld_dn11)), (assign26220_body15_e36150 * (locals.var_beta * locals.var_ps0ld_dn12)), (assign26220_body15_e36150 * (locals.var_beta * locals.var_ps0ld_dn17)),)
    } else {
        (locals.var_exp_bps0__blk845, locals.var_exp_bps0__blk845_dn0, locals.var_exp_bps0__blk845_dn2, locals.var_exp_bps0__blk845_dn6, locals.var_exp_bps0__blk845_dn7, locals.var_exp_bps0__blk845_dn10, locals.var_exp_bps0__blk845_dn11, locals.var_exp_bps0__blk845_dn12, locals.var_exp_bps0__blk845_dn17,)
    }
};
            locals.var_exp_bps0__blk845 = assign26220_body15_e36152;
            locals.var_exp_bps0__blk845_dn0 = assign26220_body15_e36152_d_n0;
            locals.var_exp_bps0__blk845_dn2 = assign26220_body15_e36152_d_n2;
            locals.var_exp_bps0__blk845_dn6 = assign26220_body15_e36152_d_n6;
            locals.var_exp_bps0__blk845_dn7 = assign26220_body15_e36152_d_n7;
            locals.var_exp_bps0__blk845_dn10 = assign26220_body15_e36152_d_n10;
            locals.var_exp_bps0__blk845_dn11 = assign26220_body15_e36152_d_n11;
            locals.var_exp_bps0__blk845_dn12 = assign26220_body15_e36152_d_n12;
            locals.var_exp_bps0__blk845_dn17 = assign26220_body15_e36152_d_n17;
            locals.var_exp_bps0__blk845_rv = 0.0;
            let (assign26220_body16_e36175, assign26220_body16_e36175_d_n0, assign26220_body16_e36175_d_n2, assign26220_body16_e36175_d_n6, assign26220_body16_e36175_d_n7, assign26220_body16_e36175_d_n10, assign26220_body16_e36175_d_n11, assign26220_body16_e36175_d_n12, assign26220_body16_e36175_d_n17,) = {
    if (((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard861 != 0.0)) && (locals.var_guard862 == 0.0)) && (locals.var_guard863 == 0.0)) {
        let assign26220_body16_e36172: f64 = (locals.var_exp_bps0__blk845 - locals.var_exp_bvbs__blk835);
        let assign26220_body16_e36173: f64 = (locals.var_cnst1over * assign26220_body16_e36172);
        (assign26220_body16_e36173, ((locals.var_cnst1over_dn0 * assign26220_body16_e36172) + (locals.var_cnst1over * (locals.var_exp_bps0__blk845_dn0 - locals.var_exp_bvbs__blk835_dn0))), ((locals.var_cnst1over_dn2 * assign26220_body16_e36172) + (locals.var_cnst1over * (locals.var_exp_bps0__blk845_dn2 - locals.var_exp_bvbs__blk835_dn2))), ((locals.var_cnst1over_dn6 * assign26220_body16_e36172) + (locals.var_cnst1over * (locals.var_exp_bps0__blk845_dn6 - locals.var_exp_bvbs__blk835_dn6))), ((locals.var_cnst1over_dn7 * assign26220_body16_e36172) + (locals.var_cnst1over * (locals.var_exp_bps0__blk845_dn7 - locals.var_exp_bvbs__blk835_dn7))), ((locals.var_cnst1over_dn10 * assign26220_body16_e36172) + (locals.var_cnst1over * (locals.var_exp_bps0__blk845_dn10 - locals.var_exp_bvbs__blk835_dn10))), ((locals.var_cnst1over_dn11 * assign26220_body16_e36172) + (locals.var_cnst1over * (locals.var_exp_bps0__blk845_dn11 - locals.var_exp_bvbs__blk835_dn11))), ((locals.var_cnst1over_dn12 * assign26220_body16_e36172) + (locals.var_cnst1over * (locals.var_exp_bps0__blk845_dn12 - locals.var_exp_bvbs__blk835_dn12))), ((locals.var_cnst1over_dn17 * assign26220_body16_e36172) + (locals.var_cnst1over * (locals.var_exp_bps0__blk845_dn17 - locals.var_exp_bvbs__blk835_dn17))),)
    } else {
        (locals.var_fs01__blk838, locals.var_fs01__blk838_dn0, locals.var_fs01__blk838_dn2, locals.var_fs01__blk838_dn6, locals.var_fs01__blk838_dn7, locals.var_fs01__blk838_dn10, locals.var_fs01__blk838_dn11, locals.var_fs01__blk838_dn12, locals.var_fs01__blk838_dn17,)
    }
};
            locals.var_fs01__blk838 = assign26220_body16_e36175;
            locals.var_fs01__blk838_dn0 = assign26220_body16_e36175_d_n0;
            locals.var_fs01__blk838_dn2 = assign26220_body16_e36175_d_n2;
            locals.var_fs01__blk838_dn6 = assign26220_body16_e36175_d_n6;
            locals.var_fs01__blk838_dn7 = assign26220_body16_e36175_d_n7;
            locals.var_fs01__blk838_dn10 = assign26220_body16_e36175_d_n10;
            locals.var_fs01__blk838_dn11 = assign26220_body16_e36175_d_n11;
            locals.var_fs01__blk838_dn12 = assign26220_body16_e36175_d_n12;
            locals.var_fs01__blk838_dn17 = assign26220_body16_e36175_d_n17;
            locals.var_fs01__blk838_rv = 0.0;
            let (assign26220_body17_e36198, assign26220_body17_e36198_d_n0, assign26220_body17_e36198_d_n2, assign26220_body17_e36198_d_n6, assign26220_body17_e36198_d_n7, assign26220_body17_e36198_d_n10, assign26220_body17_e36198_d_n11, assign26220_body17_e36198_d_n12, assign26220_body17_e36198_d_n17,) = {
    if (((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard861 != 0.0)) && (locals.var_guard862 == 0.0)) && (locals.var_guard863 == 0.0)) {
        let assign26220_body17_e36194: f64 = (locals.var_cnst1over * locals.var_beta);
        let assign26220_body17_e36196: f64 = (assign26220_body17_e36194 * locals.var_exp_bps0__blk845);
        (assign26220_body17_e36196, (((locals.var_cnst1over_dn0 * locals.var_beta) * locals.var_exp_bps0__blk845) + (assign26220_body17_e36194 * locals.var_exp_bps0__blk845_dn0)), (((locals.var_cnst1over_dn2 * locals.var_beta) * locals.var_exp_bps0__blk845) + (assign26220_body17_e36194 * locals.var_exp_bps0__blk845_dn2)), (((locals.var_cnst1over_dn6 * locals.var_beta) * locals.var_exp_bps0__blk845) + (assign26220_body17_e36194 * locals.var_exp_bps0__blk845_dn6)), (((locals.var_cnst1over_dn7 * locals.var_beta) * locals.var_exp_bps0__blk845) + (assign26220_body17_e36194 * locals.var_exp_bps0__blk845_dn7)), ((((locals.var_cnst1over_dn10 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn10)) * locals.var_exp_bps0__blk845) + (assign26220_body17_e36194 * locals.var_exp_bps0__blk845_dn10)), (((locals.var_cnst1over_dn11 * locals.var_beta) * locals.var_exp_bps0__blk845) + (assign26220_body17_e36194 * locals.var_exp_bps0__blk845_dn11)), (((locals.var_cnst1over_dn12 * locals.var_beta) * locals.var_exp_bps0__blk845) + (assign26220_body17_e36194 * locals.var_exp_bps0__blk845_dn12)), (((locals.var_cnst1over_dn17 * locals.var_beta) * locals.var_exp_bps0__blk845) + (assign26220_body17_e36194 * locals.var_exp_bps0__blk845_dn17)),)
    } else {
        (locals.var_fs01_dps0__blk839, locals.var_fs01_dps0__blk839_dn0, locals.var_fs01_dps0__blk839_dn2, locals.var_fs01_dps0__blk839_dn6, locals.var_fs01_dps0__blk839_dn7, locals.var_fs01_dps0__blk839_dn10, locals.var_fs01_dps0__blk839_dn11, locals.var_fs01_dps0__blk839_dn12, locals.var_fs01_dps0__blk839_dn17,)
    }
};
            locals.var_fs01_dps0__blk839 = assign26220_body17_e36198;
            locals.var_fs01_dps0__blk839_dn0 = assign26220_body17_e36198_d_n0;
            locals.var_fs01_dps0__blk839_dn2 = assign26220_body17_e36198_d_n2;
            locals.var_fs01_dps0__blk839_dn6 = assign26220_body17_e36198_d_n6;
            locals.var_fs01_dps0__blk839_dn7 = assign26220_body17_e36198_d_n7;
            locals.var_fs01_dps0__blk839_dn10 = assign26220_body17_e36198_d_n10;
            locals.var_fs01_dps0__blk839_dn11 = assign26220_body17_e36198_d_n11;
            locals.var_fs01_dps0__blk839_dn12 = assign26220_body17_e36198_d_n12;
            locals.var_fs01_dps0__blk839_dn17 = assign26220_body17_e36198_d_n17;
            locals.var_fs01_dps0__blk839_rv = 0.0;
            let (assign26220_body18_e36219, assign26220_body18_e36219_d_n0, assign26220_body18_e36219_d_n2, assign26220_body18_e36219_d_n6, assign26220_body18_e36219_d_n7, assign26220_body18_e36219_d_n10, assign26220_body18_e36219_d_n11, assign26220_body18_e36219_d_n12, assign26220_body18_e36219_d_n17,) = {
    if ((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard861 != 0.0)) && (locals.var_guard862 == 0.0)) {
        let assign26220_body18_e36214: f64 = (locals.var_chi__blk816 - 1.0);
        let assign26220_body18_e36216: f64 = (assign26220_body18_e36214 + locals.var_fs01__blk838);
        let assign26220_body18_e36217: f64 = (assign26220_body18_e36216).sqrt();
        (assign26220_body18_e36217, ((locals.var_chi__blk816_dn0 + locals.var_fs01__blk838_dn0) / (2.0 * assign26220_body18_e36217)), ((locals.var_chi__blk816_dn2 + locals.var_fs01__blk838_dn2) / (2.0 * assign26220_body18_e36217)), ((locals.var_chi__blk816_dn6 + locals.var_fs01__blk838_dn6) / (2.0 * assign26220_body18_e36217)), ((locals.var_chi__blk816_dn7 + locals.var_fs01__blk838_dn7) / (2.0 * assign26220_body18_e36217)), ((locals.var_chi__blk816_dn10 + locals.var_fs01__blk838_dn10) / (2.0 * assign26220_body18_e36217)), ((locals.var_chi__blk816_dn11 + locals.var_fs01__blk838_dn11) / (2.0 * assign26220_body18_e36217)), ((locals.var_chi__blk816_dn12 + locals.var_fs01__blk838_dn12) / (2.0 * assign26220_body18_e36217)), ((locals.var_chi__blk816_dn17 + locals.var_fs01__blk838_dn17) / (2.0 * assign26220_body18_e36217)),)
    } else {
        (locals.var_fs02__blk842, locals.var_fs02__blk842_dn0, locals.var_fs02__blk842_dn2, locals.var_fs02__blk842_dn6, locals.var_fs02__blk842_dn7, locals.var_fs02__blk842_dn10, locals.var_fs02__blk842_dn11, locals.var_fs02__blk842_dn12, locals.var_fs02__blk842_dn17,)
    }
};
            locals.var_fs02__blk842 = assign26220_body18_e36219;
            locals.var_fs02__blk842_dn0 = assign26220_body18_e36219_d_n0;
            locals.var_fs02__blk842_dn2 = assign26220_body18_e36219_d_n2;
            locals.var_fs02__blk842_dn6 = assign26220_body18_e36219_d_n6;
            locals.var_fs02__blk842_dn7 = assign26220_body18_e36219_d_n7;
            locals.var_fs02__blk842_dn10 = assign26220_body18_e36219_d_n10;
            locals.var_fs02__blk842_dn11 = assign26220_body18_e36219_d_n11;
            locals.var_fs02__blk842_dn12 = assign26220_body18_e36219_d_n12;
            locals.var_fs02__blk842_dn17 = assign26220_body18_e36219_d_n17;
            locals.var_fs02__blk842_rv = 0.0;
            let (assign26220_body19_e36241, assign26220_body19_e36241_d_n0, assign26220_body19_e36241_d_n2, assign26220_body19_e36241_d_n6, assign26220_body19_e36241_d_n7, assign26220_body19_e36241_d_n10, assign26220_body19_e36241_d_n11, assign26220_body19_e36241_d_n12, assign26220_body19_e36241_d_n17,) = {
    if ((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard861 != 0.0)) && (locals.var_guard862 == 0.0)) {
        let assign26220_body19_e36235: f64 = (locals.var_beta + locals.var_fs01_dps0__blk839);
        let assign26220_body19_e36237: f64 = (assign26220_body19_e36235 / locals.var_fs02__blk842);
        let assign26220_body19_e36239: f64 = (assign26220_body19_e36237 * 0.5);
        (assign26220_body19_e36239, ((((locals.var_fs01_dps0__blk839_dn0 * locals.var_fs02__blk842) - (assign26220_body19_e36235 * locals.var_fs02__blk842_dn0)) / (locals.var_fs02__blk842 * locals.var_fs02__blk842)) * 0.5), ((((locals.var_fs01_dps0__blk839_dn2 * locals.var_fs02__blk842) - (assign26220_body19_e36235 * locals.var_fs02__blk842_dn2)) / (locals.var_fs02__blk842 * locals.var_fs02__blk842)) * 0.5), ((((locals.var_fs01_dps0__blk839_dn6 * locals.var_fs02__blk842) - (assign26220_body19_e36235 * locals.var_fs02__blk842_dn6)) / (locals.var_fs02__blk842 * locals.var_fs02__blk842)) * 0.5), ((((locals.var_fs01_dps0__blk839_dn7 * locals.var_fs02__blk842) - (assign26220_body19_e36235 * locals.var_fs02__blk842_dn7)) / (locals.var_fs02__blk842 * locals.var_fs02__blk842)) * 0.5), (((((locals.var_beta_dn10 + locals.var_fs01_dps0__blk839_dn10) * locals.var_fs02__blk842) - (assign26220_body19_e36235 * locals.var_fs02__blk842_dn10)) / (locals.var_fs02__blk842 * locals.var_fs02__blk842)) * 0.5), ((((locals.var_fs01_dps0__blk839_dn11 * locals.var_fs02__blk842) - (assign26220_body19_e36235 * locals.var_fs02__blk842_dn11)) / (locals.var_fs02__blk842 * locals.var_fs02__blk842)) * 0.5), ((((locals.var_fs01_dps0__blk839_dn12 * locals.var_fs02__blk842) - (assign26220_body19_e36235 * locals.var_fs02__blk842_dn12)) / (locals.var_fs02__blk842 * locals.var_fs02__blk842)) * 0.5), ((((locals.var_fs01_dps0__blk839_dn17 * locals.var_fs02__blk842) - (assign26220_body19_e36235 * locals.var_fs02__blk842_dn17)) / (locals.var_fs02__blk842 * locals.var_fs02__blk842)) * 0.5),)
    } else {
        (locals.var_fs02_dps0__blk843, locals.var_fs02_dps0__blk843_dn0, locals.var_fs02_dps0__blk843_dn2, locals.var_fs02_dps0__blk843_dn6, locals.var_fs02_dps0__blk843_dn7, locals.var_fs02_dps0__blk843_dn10, locals.var_fs02_dps0__blk843_dn11, locals.var_fs02_dps0__blk843_dn12, locals.var_fs02_dps0__blk843_dn17,)
    }
};
            locals.var_fs02_dps0__blk843 = assign26220_body19_e36241;
            locals.var_fs02_dps0__blk843_dn0 = assign26220_body19_e36241_d_n0;
            locals.var_fs02_dps0__blk843_dn2 = assign26220_body19_e36241_d_n2;
            locals.var_fs02_dps0__blk843_dn6 = assign26220_body19_e36241_d_n6;
            locals.var_fs02_dps0__blk843_dn7 = assign26220_body19_e36241_d_n7;
            locals.var_fs02_dps0__blk843_dn10 = assign26220_body19_e36241_d_n10;
            locals.var_fs02_dps0__blk843_dn11 = assign26220_body19_e36241_d_n11;
            locals.var_fs02_dps0__blk843_dn12 = assign26220_body19_e36241_d_n12;
            locals.var_fs02_dps0__blk843_dn17 = assign26220_body19_e36241_d_n17;
            locals.var_fs02_dps0__blk843_rv = 0.0;
            let (assign26220_body20_e36260, assign26220_body20_e36260_d_n0, assign26220_body20_e36260_d_n2, assign26220_body20_e36260_d_n6, assign26220_body20_e36260_d_n7, assign26220_body20_e36260_d_n10, assign26220_body20_e36260_d_n11, assign26220_body20_e36260_d_n12, assign26220_body20_e36260_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard861 != 0.0)) {
        let assign26220_body20_e36254: f64 = (locals.var_vgpld - locals.var_ps0ld);
        let assign26220_body20_e36257: f64 = (locals.var_fac1__blk802 * locals.var_fs02__blk842);
        let assign26220_body20_e36258: f64 = (assign26220_body20_e36254 - assign26220_body20_e36257);
        (assign26220_body20_e36258, ((locals.var_vgpld_dn0 - locals.var_ps0ld_dn0) - ((locals.var_fac1__blk802_dn0 * locals.var_fs02__blk842) + (locals.var_fac1__blk802 * locals.var_fs02__blk842_dn0))), ((locals.var_vgpld_dn2 - locals.var_ps0ld_dn2) - ((locals.var_fac1__blk802_dn2 * locals.var_fs02__blk842) + (locals.var_fac1__blk802 * locals.var_fs02__blk842_dn2))), ((locals.var_vgpld_dn6 - locals.var_ps0ld_dn6) - ((locals.var_fac1__blk802_dn6 * locals.var_fs02__blk842) + (locals.var_fac1__blk802 * locals.var_fs02__blk842_dn6))), ((locals.var_vgpld_dn7 - locals.var_ps0ld_dn7) - ((locals.var_fac1__blk802_dn7 * locals.var_fs02__blk842) + (locals.var_fac1__blk802 * locals.var_fs02__blk842_dn7))), ((locals.var_vgpld_dn10 - locals.var_ps0ld_dn10) - ((locals.var_fac1__blk802_dn10 * locals.var_fs02__blk842) + (locals.var_fac1__blk802 * locals.var_fs02__blk842_dn10))), ((locals.var_vgpld_dn11 - locals.var_ps0ld_dn11) - ((locals.var_fac1__blk802_dn11 * locals.var_fs02__blk842) + (locals.var_fac1__blk802 * locals.var_fs02__blk842_dn11))), ((locals.var_vgpld_dn12 - locals.var_ps0ld_dn12) - ((locals.var_fac1__blk802_dn12 * locals.var_fs02__blk842) + (locals.var_fac1__blk802 * locals.var_fs02__blk842_dn12))), ((locals.var_vgpld_dn17 - locals.var_ps0ld_dn17) - ((locals.var_fac1__blk802_dn17 * locals.var_fs02__blk842) + (locals.var_fac1__blk802 * locals.var_fs02__blk842_dn17))),)
    } else {
        (locals.var_fs0__blk846, locals.var_fs0__blk846_dn0, locals.var_fs0__blk846_dn2, locals.var_fs0__blk846_dn6, locals.var_fs0__blk846_dn7, locals.var_fs0__blk846_dn10, locals.var_fs0__blk846_dn11, locals.var_fs0__blk846_dn12, locals.var_fs0__blk846_dn17,)
    }
};
            locals.var_fs0__blk846 = assign26220_body20_e36260;
            locals.var_fs0__blk846_dn0 = assign26220_body20_e36260_d_n0;
            locals.var_fs0__blk846_dn2 = assign26220_body20_e36260_d_n2;
            locals.var_fs0__blk846_dn6 = assign26220_body20_e36260_d_n6;
            locals.var_fs0__blk846_dn7 = assign26220_body20_e36260_d_n7;
            locals.var_fs0__blk846_dn10 = assign26220_body20_e36260_d_n10;
            locals.var_fs0__blk846_dn11 = assign26220_body20_e36260_d_n11;
            locals.var_fs0__blk846_dn12 = assign26220_body20_e36260_d_n12;
            locals.var_fs0__blk846_dn17 = assign26220_body20_e36260_d_n17;
            locals.var_fs0__blk846_rv = 0.0;
            let (assign26220_body21_e36278, assign26220_body21_e36278_d_n0, assign26220_body21_e36278_d_n2, assign26220_body21_e36278_d_n6, assign26220_body21_e36278_d_n7, assign26220_body21_e36278_d_n10, assign26220_body21_e36278_d_n11, assign26220_body21_e36278_d_n12, assign26220_body21_e36278_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard861 != 0.0)) {
        let assign26220_body21_e36272: f64 = (-1.0);
        let assign26220_body21_e36275: f64 = (locals.var_fac1__blk802 * locals.var_fs02_dps0__blk843);
        let assign26220_body21_e36276: f64 = (assign26220_body21_e36272 - assign26220_body21_e36275);
        (assign26220_body21_e36276, (-((locals.var_fac1__blk802_dn0 * locals.var_fs02_dps0__blk843) + (locals.var_fac1__blk802 * locals.var_fs02_dps0__blk843_dn0))), (-((locals.var_fac1__blk802_dn2 * locals.var_fs02_dps0__blk843) + (locals.var_fac1__blk802 * locals.var_fs02_dps0__blk843_dn2))), (-((locals.var_fac1__blk802_dn6 * locals.var_fs02_dps0__blk843) + (locals.var_fac1__blk802 * locals.var_fs02_dps0__blk843_dn6))), (-((locals.var_fac1__blk802_dn7 * locals.var_fs02_dps0__blk843) + (locals.var_fac1__blk802 * locals.var_fs02_dps0__blk843_dn7))), (-((locals.var_fac1__blk802_dn10 * locals.var_fs02_dps0__blk843) + (locals.var_fac1__blk802 * locals.var_fs02_dps0__blk843_dn10))), (-((locals.var_fac1__blk802_dn11 * locals.var_fs02_dps0__blk843) + (locals.var_fac1__blk802 * locals.var_fs02_dps0__blk843_dn11))), (-((locals.var_fac1__blk802_dn12 * locals.var_fs02_dps0__blk843) + (locals.var_fac1__blk802 * locals.var_fs02_dps0__blk843_dn12))), (-((locals.var_fac1__blk802_dn17 * locals.var_fs02_dps0__blk843) + (locals.var_fac1__blk802 * locals.var_fs02_dps0__blk843_dn17))),)
    } else {
        (locals.var_fs0_dps0__blk847, locals.var_fs0_dps0__blk847_dn0, locals.var_fs0_dps0__blk847_dn2, locals.var_fs0_dps0__blk847_dn6, locals.var_fs0_dps0__blk847_dn7, locals.var_fs0_dps0__blk847_dn10, locals.var_fs0_dps0__blk847_dn11, locals.var_fs0_dps0__blk847_dn12, locals.var_fs0_dps0__blk847_dn17,)
    }
};
            locals.var_fs0_dps0__blk847 = assign26220_body21_e36278;
            locals.var_fs0_dps0__blk847_dn0 = assign26220_body21_e36278_d_n0;
            locals.var_fs0_dps0__blk847_dn2 = assign26220_body21_e36278_d_n2;
            locals.var_fs0_dps0__blk847_dn6 = assign26220_body21_e36278_d_n6;
            locals.var_fs0_dps0__blk847_dn7 = assign26220_body21_e36278_d_n7;
            locals.var_fs0_dps0__blk847_dn10 = assign26220_body21_e36278_d_n10;
            locals.var_fs0_dps0__blk847_dn11 = assign26220_body21_e36278_d_n11;
            locals.var_fs0_dps0__blk847_dn12 = assign26220_body21_e36278_d_n12;
            locals.var_fs0_dps0__blk847_dn17 = assign26220_body21_e36278_d_n17;
            locals.var_fs0_dps0__blk847_rv = 0.0;
            let assign26220_body22_e36281: f64 = if locals.var_flg_conv__blk789 == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard864 = assign26220_body22_e36281;
            locals.var_guard864_rv = 0.0;
            let (assign26220_body23_e36300,) = {
    if ((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard861 != 0.0)) && (locals.var_guard864 != 0.0)) {
        let assign26220_body23_e36296: f64 = (2.0 * 20.0);
        let assign26220_body23_e36298: f64 = (assign26220_body23_e36296 + 1.0);
        (assign26220_body23_e36298,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign26220_body23_e36300;
            locals.var_lp_s0_rv = 0.0;
            let (assign26220_body24_e36319, assign26220_body24_e36319_d_n0, assign26220_body24_e36319_d_n2, assign26220_body24_e36319_d_n6, assign26220_body24_e36319_d_n7, assign26220_body24_e36319_d_n10, assign26220_body24_e36319_d_n11, assign26220_body24_e36319_d_n12, assign26220_body24_e36319_d_n17,) = {
    if ((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard861 != 0.0)) && (locals.var_guard864 == 0.0)) {
        let assign26220_body24_e36315: f64 = (-locals.var_fs0__blk846);
        let assign26220_body24_e36317: f64 = (assign26220_body24_e36315 / locals.var_fs0_dps0__blk847);
        (assign26220_body24_e36317, ((((-locals.var_fs0__blk846_dn0) * locals.var_fs0_dps0__blk847) - (assign26220_body24_e36315 * locals.var_fs0_dps0__blk847_dn0)) / (locals.var_fs0_dps0__blk847 * locals.var_fs0_dps0__blk847)), ((((-locals.var_fs0__blk846_dn2) * locals.var_fs0_dps0__blk847) - (assign26220_body24_e36315 * locals.var_fs0_dps0__blk847_dn2)) / (locals.var_fs0_dps0__blk847 * locals.var_fs0_dps0__blk847)), ((((-locals.var_fs0__blk846_dn6) * locals.var_fs0_dps0__blk847) - (assign26220_body24_e36315 * locals.var_fs0_dps0__blk847_dn6)) / (locals.var_fs0_dps0__blk847 * locals.var_fs0_dps0__blk847)), ((((-locals.var_fs0__blk846_dn7) * locals.var_fs0_dps0__blk847) - (assign26220_body24_e36315 * locals.var_fs0_dps0__blk847_dn7)) / (locals.var_fs0_dps0__blk847 * locals.var_fs0_dps0__blk847)), ((((-locals.var_fs0__blk846_dn10) * locals.var_fs0_dps0__blk847) - (assign26220_body24_e36315 * locals.var_fs0_dps0__blk847_dn10)) / (locals.var_fs0_dps0__blk847 * locals.var_fs0_dps0__blk847)), ((((-locals.var_fs0__blk846_dn11) * locals.var_fs0_dps0__blk847) - (assign26220_body24_e36315 * locals.var_fs0_dps0__blk847_dn11)) / (locals.var_fs0_dps0__blk847 * locals.var_fs0_dps0__blk847)), ((((-locals.var_fs0__blk846_dn12) * locals.var_fs0_dps0__blk847) - (assign26220_body24_e36315 * locals.var_fs0_dps0__blk847_dn12)) / (locals.var_fs0_dps0__blk847 * locals.var_fs0_dps0__blk847)), ((((-locals.var_fs0__blk846_dn17) * locals.var_fs0_dps0__blk847) - (assign26220_body24_e36315 * locals.var_fs0_dps0__blk847_dn17)) / (locals.var_fs0_dps0__blk847 * locals.var_fs0_dps0__blk847)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn12, locals.var_dps0_dn17,)
    }
};
            locals.var_dps0 = assign26220_body24_e36319;
            locals.var_dps0_dn0 = assign26220_body24_e36319_d_n0;
            locals.var_dps0_dn2 = assign26220_body24_e36319_d_n2;
            locals.var_dps0_dn6 = assign26220_body24_e36319_d_n6;
            locals.var_dps0_dn7 = assign26220_body24_e36319_d_n7;
            locals.var_dps0_dn10 = assign26220_body24_e36319_d_n10;
            locals.var_dps0_dn11 = assign26220_body24_e36319_d_n11;
            locals.var_dps0_dn12 = assign26220_body24_e36319_d_n12;
            locals.var_dps0_dn17 = assign26220_body24_e36319_d_n17;
            locals.var_dps0_rv = 0.0;
            let (assign26220_body25_e36348, assign26220_body25_e36348_d_n0, assign26220_body25_e36348_d_n2, assign26220_body25_e36348_d_n6, assign26220_body25_e36348_d_n7, assign26220_body25_e36348_d_n10, assign26220_body25_e36348_d_n11, assign26220_body25_e36348_d_n12, assign26220_body25_e36348_d_n17,) = {
    if ((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard861 != 0.0)) && (locals.var_guard864 == 0.0)) {
        let assign26220_body25_e36335: f64 = (0.5 * 0.1);
        let assign26220_body25_e36339: f64 = (locals.var_ps0ld).abs();
        let (assign26220_body25_e36344, assign26220_body25_e36344_d_n0, assign26220_body25_e36344_d_n2, assign26220_body25_e36344_d_n6, assign26220_body25_e36344_d_n7, assign26220_body25_e36344_d_n10, assign26220_body25_e36344_d_n11, assign26220_body25_e36344_d_n12, assign26220_body25_e36344_d_n17,) = {
            if (1.0 >= assign26220_body25_e36339) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign26220_body25_e36343: f64 = (locals.var_ps0ld).abs();
                (assign26220_body25_e36343, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn0 } else { (-locals.var_ps0ld_dn0) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn2 } else { (-locals.var_ps0ld_dn2) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn6 } else { (-locals.var_ps0ld_dn6) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn7 } else { (-locals.var_ps0ld_dn7) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn10 } else { (-locals.var_ps0ld_dn10) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn11 } else { (-locals.var_ps0ld_dn11) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn12 } else { (-locals.var_ps0ld_dn12) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn17 } else { (-locals.var_ps0ld_dn17) },)
            }
        };
        let assign26220_body25_e36345: f64 = (1.0 + assign26220_body25_e36344);
        let assign26220_body25_e36346: f64 = (assign26220_body25_e36335 * assign26220_body25_e36345);
        (assign26220_body25_e36346, (assign26220_body25_e36335 * assign26220_body25_e36344_d_n0), (assign26220_body25_e36335 * assign26220_body25_e36344_d_n2), (assign26220_body25_e36335 * assign26220_body25_e36344_d_n6), (assign26220_body25_e36335 * assign26220_body25_e36344_d_n7), (assign26220_body25_e36335 * assign26220_body25_e36344_d_n10), (assign26220_body25_e36335 * assign26220_body25_e36344_d_n11), (assign26220_body25_e36335 * assign26220_body25_e36344_d_n12), (assign26220_body25_e36335 * assign26220_body25_e36344_d_n17),)
    } else {
        (locals.var_dplim__blk848, locals.var_dplim__blk848_dn0, locals.var_dplim__blk848_dn2, locals.var_dplim__blk848_dn6, locals.var_dplim__blk848_dn7, locals.var_dplim__blk848_dn10, locals.var_dplim__blk848_dn11, locals.var_dplim__blk848_dn12, locals.var_dplim__blk848_dn17,)
    }
};
            locals.var_dplim__blk848 = assign26220_body25_e36348;
            locals.var_dplim__blk848_dn0 = assign26220_body25_e36348_d_n0;
            locals.var_dplim__blk848_dn2 = assign26220_body25_e36348_d_n2;
            locals.var_dplim__blk848_dn6 = assign26220_body25_e36348_d_n6;
            locals.var_dplim__blk848_dn7 = assign26220_body25_e36348_d_n7;
            locals.var_dplim__blk848_dn10 = assign26220_body25_e36348_d_n10;
            locals.var_dplim__blk848_dn11 = assign26220_body25_e36348_d_n11;
            locals.var_dplim__blk848_dn12 = assign26220_body25_e36348_d_n12;
            locals.var_dplim__blk848_dn17 = assign26220_body25_e36348_d_n17;
            locals.var_dplim__blk848_rv = 0.0;
            let assign26220_body26_e36350: f64 = (locals.var_dps0).abs();
            let assign26220_body26_e36352: f64 = if assign26220_body26_e36350 > locals.var_dplim__blk848 { 1.0 } else { 0.0 };
            locals.var_guard865 = assign26220_body26_e36352;
            locals.var_guard865_rv = 0.0;
            let (assign26220_body27_e36378, assign26220_body27_e36378_d_n0, assign26220_body27_e36378_d_n2, assign26220_body27_e36378_d_n6, assign26220_body27_e36378_d_n7, assign26220_body27_e36378_d_n10, assign26220_body27_e36378_d_n11, assign26220_body27_e36378_d_n12, assign26220_body27_e36378_d_n17,) = {
    if (((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard861 != 0.0)) && (locals.var_guard864 == 0.0)) && (locals.var_guard865 != 0.0)) {
        let (assign26220_body27_e36375,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign26220_body27_e36374: f64 = (-1.0);
                (assign26220_body27_e36374,)
            }
        };
        let assign26220_body27_e36376: f64 = (locals.var_dplim__blk848 * assign26220_body27_e36375);
        (assign26220_body27_e36376, (locals.var_dplim__blk848_dn0 * assign26220_body27_e36375), (locals.var_dplim__blk848_dn2 * assign26220_body27_e36375), (locals.var_dplim__blk848_dn6 * assign26220_body27_e36375), (locals.var_dplim__blk848_dn7 * assign26220_body27_e36375), (locals.var_dplim__blk848_dn10 * assign26220_body27_e36375), (locals.var_dplim__blk848_dn11 * assign26220_body27_e36375), (locals.var_dplim__blk848_dn12 * assign26220_body27_e36375), (locals.var_dplim__blk848_dn17 * assign26220_body27_e36375),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn12, locals.var_dps0_dn17,)
    }
};
            locals.var_dps0 = assign26220_body27_e36378;
            locals.var_dps0_dn0 = assign26220_body27_e36378_d_n0;
            locals.var_dps0_dn2 = assign26220_body27_e36378_d_n2;
            locals.var_dps0_dn6 = assign26220_body27_e36378_d_n6;
            locals.var_dps0_dn7 = assign26220_body27_e36378_d_n7;
            locals.var_dps0_dn10 = assign26220_body27_e36378_d_n10;
            locals.var_dps0_dn11 = assign26220_body27_e36378_d_n11;
            locals.var_dps0_dn12 = assign26220_body27_e36378_d_n12;
            locals.var_dps0_dn17 = assign26220_body27_e36378_d_n17;
            locals.var_dps0_rv = 0.0;
            let (assign26220_body28_e36396, assign26220_body28_e36396_d_n0, assign26220_body28_e36396_d_n2, assign26220_body28_e36396_d_n6, assign26220_body28_e36396_d_n7, assign26220_body28_e36396_d_n10, assign26220_body28_e36396_d_n11, assign26220_body28_e36396_d_n12, assign26220_body28_e36396_d_n17,) = {
    if ((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard861 != 0.0)) && (locals.var_guard864 == 0.0)) {
        let assign26220_body28_e36394: f64 = (locals.var_ps0ld + locals.var_dps0);
        (assign26220_body28_e36394, (locals.var_ps0ld_dn0 + locals.var_dps0_dn0), (locals.var_ps0ld_dn2 + locals.var_dps0_dn2), (locals.var_ps0ld_dn6 + locals.var_dps0_dn6), (locals.var_ps0ld_dn7 + locals.var_dps0_dn7), (locals.var_ps0ld_dn10 + locals.var_dps0_dn10), (locals.var_ps0ld_dn11 + locals.var_dps0_dn11), (locals.var_ps0ld_dn12 + locals.var_dps0_dn12), (locals.var_ps0ld_dn17 + locals.var_dps0_dn17),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn12, locals.var_ps0ld_dn17,)
    }
};
            locals.var_ps0ld = assign26220_body28_e36396;
            locals.var_ps0ld_dn0 = assign26220_body28_e36396_d_n0;
            locals.var_ps0ld_dn2 = assign26220_body28_e36396_d_n2;
            locals.var_ps0ld_dn6 = assign26220_body28_e36396_d_n6;
            locals.var_ps0ld_dn7 = assign26220_body28_e36396_d_n7;
            locals.var_ps0ld_dn10 = assign26220_body28_e36396_d_n10;
            locals.var_ps0ld_dn11 = assign26220_body28_e36396_d_n11;
            locals.var_ps0ld_dn12 = assign26220_body28_e36396_d_n12;
            locals.var_ps0ld_dn17 = assign26220_body28_e36396_d_n17;
            locals.var_ps0ld_rv = 0.0;
            let assign26220_body29_e36398: f64 = (locals.var_dps0).abs();
            let assign26220_body29_e36402: f64 = (locals.var_fs0__blk846).abs();
            let assign26220_body29_e36405: f64 = if ((assign26220_body29_e36398 <= 5e-12) && (assign26220_body29_e36402 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard866 = assign26220_body29_e36405;
            locals.var_guard866_rv = 0.0;
            let (assign26220_body30_e36423,) = {
    if (((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard861 != 0.0)) && (locals.var_guard864 == 0.0)) && (locals.var_guard866 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv__blk789,)
    }
};
            locals.var_flg_conv__blk789 = assign26220_body30_e36423;
            locals.var_flg_conv__blk789_rv = 0.0;
            let (assign26220_body31_e36438,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard861 != 0.0)) {
        let assign26220_body31_e36436: f64 = (locals.var_lp_s0 + 1.0);
        (assign26220_body31_e36436,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign26220_body31_e36438;
            locals.var_lp_s0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_94(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign26240_e36444: f64 = if locals.var_chi__blk816 < 5.0 { 1.0 } else { 0.0 };
        locals.var_guard868 = assign26240_e36444;
        locals.var_guard868_rv = 0.0;

        let (assign26280_e36503, assign26280_e36503_d_n0, assign26280_e36503_d_n2, assign26280_e36503_d_n6, assign26280_e36503_d_n7, assign26280_e36503_d_n10, assign26280_e36503_d_n11, assign26280_e36503_d_n12, assign26280_e36503_d_n17,) = {
    if ((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard861 != 0.0)) && (locals.var_guard868 != 0.0)) {
        let assign26280_e36497: f64 = (locals.var_fb__blk840 * locals.var_fb__blk840);
        let assign26280_e36500: f64 = (10.0 * 2.220446049250313e-16);
        let assign26280_e36501: f64 = (assign26280_e36497 + assign26280_e36500);
        (assign26280_e36501, ((locals.var_fb__blk840_dn0 * locals.var_fb__blk840) + (locals.var_fb__blk840 * locals.var_fb__blk840_dn0)), ((locals.var_fb__blk840_dn2 * locals.var_fb__blk840) + (locals.var_fb__blk840 * locals.var_fb__blk840_dn2)), ((locals.var_fb__blk840_dn6 * locals.var_fb__blk840) + (locals.var_fb__blk840 * locals.var_fb__blk840_dn6)), ((locals.var_fb__blk840_dn7 * locals.var_fb__blk840) + (locals.var_fb__blk840 * locals.var_fb__blk840_dn7)), ((locals.var_fb__blk840_dn10 * locals.var_fb__blk840) + (locals.var_fb__blk840 * locals.var_fb__blk840_dn10)), ((locals.var_fb__blk840_dn11 * locals.var_fb__blk840) + (locals.var_fb__blk840 * locals.var_fb__blk840_dn11)), ((locals.var_fb__blk840_dn12 * locals.var_fb__blk840) + (locals.var_fb__blk840 * locals.var_fb__blk840_dn12)), ((locals.var_fb__blk840_dn17 * locals.var_fb__blk840) + (locals.var_fb__blk840 * locals.var_fb__blk840_dn17)),)
    } else {
        (locals.var_xi0__blk849, locals.var_xi0__blk849_dn0, locals.var_xi0__blk849_dn2, locals.var_xi0__blk849_dn6, locals.var_xi0__blk849_dn7, locals.var_xi0__blk849_dn10, locals.var_xi0__blk849_dn11, locals.var_xi0__blk849_dn12, locals.var_xi0__blk849_dn17,)
    }
};
        locals.var_xi0__blk849 = assign26280_e36503;
        locals.var_xi0__blk849_dn0 = assign26280_e36503_d_n0;
        locals.var_xi0__blk849_dn2 = assign26280_e36503_d_n2;
        locals.var_xi0__blk849_dn6 = assign26280_e36503_d_n6;
        locals.var_xi0__blk849_dn7 = assign26280_e36503_d_n7;
        locals.var_xi0__blk849_dn10 = assign26280_e36503_d_n10;
        locals.var_xi0__blk849_dn11 = assign26280_e36503_d_n11;
        locals.var_xi0__blk849_dn12 = assign26280_e36503_d_n12;
        locals.var_xi0__blk849_dn17 = assign26280_e36503_d_n17;
        locals.var_xi0__blk849_rv = 0.0;

        let (assign26290_e36522, assign26290_e36522_d_n0, assign26290_e36522_d_n2, assign26290_e36522_d_n6, assign26290_e36522_d_n7, assign26290_e36522_d_n10, assign26290_e36522_d_n11, assign26290_e36522_d_n12, assign26290_e36522_d_n17,) = {
    if ((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard861 != 0.0)) && (locals.var_guard868 != 0.0)) {
        let assign26290_e36519: f64 = (10.0 * 2.220446049250313e-16);
        let assign26290_e36520: f64 = (locals.var_fb__blk840 + assign26290_e36519);
        (assign26290_e36520, locals.var_fb__blk840_dn0, locals.var_fb__blk840_dn2, locals.var_fb__blk840_dn6, locals.var_fb__blk840_dn7, locals.var_fb__blk840_dn10, locals.var_fb__blk840_dn11, locals.var_fb__blk840_dn12, locals.var_fb__blk840_dn17,)
    } else {
        (locals.var_xi0p12__blk850, locals.var_xi0p12__blk850_dn0, locals.var_xi0p12__blk850_dn2, locals.var_xi0p12__blk850_dn6, locals.var_xi0p12__blk850_dn7, locals.var_xi0p12__blk850_dn10, locals.var_xi0p12__blk850_dn11, locals.var_xi0p12__blk850_dn12, locals.var_xi0p12__blk850_dn17,)
    }
};
        locals.var_xi0p12__blk850 = assign26290_e36522;
        locals.var_xi0p12__blk850_dn0 = assign26290_e36522_d_n0;
        locals.var_xi0p12__blk850_dn2 = assign26290_e36522_d_n2;
        locals.var_xi0p12__blk850_dn6 = assign26290_e36522_d_n6;
        locals.var_xi0p12__blk850_dn7 = assign26290_e36522_d_n7;
        locals.var_xi0p12__blk850_dn10 = assign26290_e36522_d_n10;
        locals.var_xi0p12__blk850_dn11 = assign26290_e36522_d_n11;
        locals.var_xi0p12__blk850_dn12 = assign26290_e36522_d_n12;
        locals.var_xi0p12__blk850_dn17 = assign26290_e36522_d_n17;
        locals.var_xi0p12__blk850_rv = 0.0;

        let (assign26310_e36556, assign26310_e36556_d_n0, assign26310_e36556_d_n2, assign26310_e36556_d_n6, assign26310_e36556_d_n7, assign26310_e36556_d_n10, assign26310_e36556_d_n11, assign26310_e36556_d_n12, assign26310_e36556_d_n17,) = {
    if ((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard861 != 0.0)) && (locals.var_guard868 == 0.0)) {
        let assign26310_e36554: f64 = (locals.var_chi__blk816 - 1.0);
        (assign26310_e36554, locals.var_chi__blk816_dn0, locals.var_chi__blk816_dn2, locals.var_chi__blk816_dn6, locals.var_chi__blk816_dn7, locals.var_chi__blk816_dn10, locals.var_chi__blk816_dn11, locals.var_chi__blk816_dn12, locals.var_chi__blk816_dn17,)
    } else {
        (locals.var_xi0__blk849, locals.var_xi0__blk849_dn0, locals.var_xi0__blk849_dn2, locals.var_xi0__blk849_dn6, locals.var_xi0__blk849_dn7, locals.var_xi0__blk849_dn10, locals.var_xi0__blk849_dn11, locals.var_xi0__blk849_dn12, locals.var_xi0__blk849_dn17,)
    }
};
        locals.var_xi0__blk849 = assign26310_e36556;
        locals.var_xi0__blk849_dn0 = assign26310_e36556_d_n0;
        locals.var_xi0__blk849_dn2 = assign26310_e36556_d_n2;
        locals.var_xi0__blk849_dn6 = assign26310_e36556_d_n6;
        locals.var_xi0__blk849_dn7 = assign26310_e36556_d_n7;
        locals.var_xi0__blk849_dn10 = assign26310_e36556_d_n10;
        locals.var_xi0__blk849_dn11 = assign26310_e36556_d_n11;
        locals.var_xi0__blk849_dn12 = assign26310_e36556_d_n12;
        locals.var_xi0__blk849_dn17 = assign26310_e36556_d_n17;
        locals.var_xi0__blk849_rv = 0.0;

        let (assign26320_e36573, assign26320_e36573_d_n0, assign26320_e36573_d_n2, assign26320_e36573_d_n6, assign26320_e36573_d_n7, assign26320_e36573_d_n10, assign26320_e36573_d_n11, assign26320_e36573_d_n12, assign26320_e36573_d_n17,) = {
    if ((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard861 != 0.0)) && (locals.var_guard868 == 0.0)) {
        let assign26320_e36571: f64 = (locals.var_xi0__blk849).sqrt();
        (assign26320_e36571, (locals.var_xi0__blk849_dn0 / (2.0 * assign26320_e36571)), (locals.var_xi0__blk849_dn2 / (2.0 * assign26320_e36571)), (locals.var_xi0__blk849_dn6 / (2.0 * assign26320_e36571)), (locals.var_xi0__blk849_dn7 / (2.0 * assign26320_e36571)), (locals.var_xi0__blk849_dn10 / (2.0 * assign26320_e36571)), (locals.var_xi0__blk849_dn11 / (2.0 * assign26320_e36571)), (locals.var_xi0__blk849_dn12 / (2.0 * assign26320_e36571)), (locals.var_xi0__blk849_dn17 / (2.0 * assign26320_e36571)),)
    } else {
        (locals.var_xi0p12__blk850, locals.var_xi0p12__blk850_dn0, locals.var_xi0p12__blk850_dn2, locals.var_xi0p12__blk850_dn6, locals.var_xi0p12__blk850_dn7, locals.var_xi0p12__blk850_dn10, locals.var_xi0p12__blk850_dn11, locals.var_xi0p12__blk850_dn12, locals.var_xi0p12__blk850_dn17,)
    }
};
        locals.var_xi0p12__blk850 = assign26320_e36573;
        locals.var_xi0p12__blk850_dn0 = assign26320_e36573_d_n0;
        locals.var_xi0p12__blk850_dn2 = assign26320_e36573_d_n2;
        locals.var_xi0p12__blk850_dn6 = assign26320_e36573_d_n6;
        locals.var_xi0p12__blk850_dn7 = assign26320_e36573_d_n7;
        locals.var_xi0p12__blk850_dn10 = assign26320_e36573_d_n10;
        locals.var_xi0p12__blk850_dn11 = assign26320_e36573_d_n11;
        locals.var_xi0p12__blk850_dn12 = assign26320_e36573_d_n12;
        locals.var_xi0p12__blk850_dn17 = assign26320_e36573_d_n17;
        locals.var_xi0p12__blk850_rv = 0.0;

        let (assign26330_e36588, assign26330_e36588_d_n0, assign26330_e36588_d_n2, assign26330_e36588_d_n6, assign26330_e36588_d_n7, assign26330_e36588_d_n10, assign26330_e36588_d_n11, assign26330_e36588_d_n12, assign26330_e36588_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard861 != 0.0)) {
        let assign26330_e36586: f64 = (locals.var_cnst0over * locals.var_xi0p12__blk850);
        (assign26330_e36586, ((locals.var_cnst0over_dn0 * locals.var_xi0p12__blk850) + (locals.var_cnst0over * locals.var_xi0p12__blk850_dn0)), ((locals.var_cnst0over_dn2 * locals.var_xi0p12__blk850) + (locals.var_cnst0over * locals.var_xi0p12__blk850_dn2)), ((locals.var_cnst0over_dn6 * locals.var_xi0p12__blk850) + (locals.var_cnst0over * locals.var_xi0p12__blk850_dn6)), ((locals.var_cnst0over_dn7 * locals.var_xi0p12__blk850) + (locals.var_cnst0over * locals.var_xi0p12__blk850_dn7)), ((locals.var_cnst0over_dn10 * locals.var_xi0p12__blk850) + (locals.var_cnst0over * locals.var_xi0p12__blk850_dn10)), ((locals.var_cnst0over_dn11 * locals.var_xi0p12__blk850) + (locals.var_cnst0over * locals.var_xi0p12__blk850_dn11)), ((locals.var_cnst0over_dn12 * locals.var_xi0p12__blk850) + (locals.var_cnst0over * locals.var_xi0p12__blk850_dn12)), ((locals.var_cnst0over_dn17 * locals.var_xi0p12__blk850) + (locals.var_cnst0over * locals.var_xi0p12__blk850_dn17)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn12, locals.var_qbuld_dn17,)
    }
};
        locals.var_qbuld = assign26330_e36588;
        locals.var_qbuld_dn0 = assign26330_e36588_d_n0;
        locals.var_qbuld_dn2 = assign26330_e36588_d_n2;
        locals.var_qbuld_dn6 = assign26330_e36588_d_n6;
        locals.var_qbuld_dn7 = assign26330_e36588_d_n7;
        locals.var_qbuld_dn10 = assign26330_e36588_d_n10;
        locals.var_qbuld_dn11 = assign26330_e36588_d_n11;
        locals.var_qbuld_dn12 = assign26330_e36588_d_n12;
        locals.var_qbuld_dn17 = assign26330_e36588_d_n17;
        locals.var_qbuld_rv = 0.0;

        let (assign26340_e36605, assign26340_e36605_d_n0, assign26340_e36605_d_n2, assign26340_e36605_d_n6, assign26340_e36605_d_n7, assign26340_e36605_d_n10, assign26340_e36605_d_n11, assign26340_e36605_d_n12, assign26340_e36605_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard861 != 0.0)) {
        let assign26340_e36602: f64 = (locals.var_fs02__blk842 + locals.var_xi0p12__blk850);
        let assign26340_e36603: f64 = (1.0 / assign26340_e36602);
        (assign26340_e36603, (-((locals.var_fs02__blk842_dn0 + locals.var_xi0p12__blk850_dn0) / (assign26340_e36602 * assign26340_e36602))), (-((locals.var_fs02__blk842_dn2 + locals.var_xi0p12__blk850_dn2) / (assign26340_e36602 * assign26340_e36602))), (-((locals.var_fs02__blk842_dn6 + locals.var_xi0p12__blk850_dn6) / (assign26340_e36602 * assign26340_e36602))), (-((locals.var_fs02__blk842_dn7 + locals.var_xi0p12__blk850_dn7) / (assign26340_e36602 * assign26340_e36602))), (-((locals.var_fs02__blk842_dn10 + locals.var_xi0p12__blk850_dn10) / (assign26340_e36602 * assign26340_e36602))), (-((locals.var_fs02__blk842_dn11 + locals.var_xi0p12__blk850_dn11) / (assign26340_e36602 * assign26340_e36602))), (-((locals.var_fs02__blk842_dn12 + locals.var_xi0p12__blk850_dn12) / (assign26340_e36602 * assign26340_e36602))), (-((locals.var_fs02__blk842_dn17 + locals.var_xi0p12__blk850_dn17) / (assign26340_e36602 * assign26340_e36602))),)
    } else {
        (locals.var_t1__blk773, locals.var_t1__blk773_dn0, locals.var_t1__blk773_dn2, locals.var_t1__blk773_dn6, locals.var_t1__blk773_dn7, locals.var_t1__blk773_dn10, locals.var_t1__blk773_dn11, locals.var_t1__blk773_dn12, locals.var_t1__blk773_dn17,)
    }
};
        locals.var_t1__blk773 = assign26340_e36605;
        locals.var_t1__blk773_dn0 = assign26340_e36605_d_n0;
        locals.var_t1__blk773_dn2 = assign26340_e36605_d_n2;
        locals.var_t1__blk773_dn6 = assign26340_e36605_d_n6;
        locals.var_t1__blk773_dn7 = assign26340_e36605_d_n7;
        locals.var_t1__blk773_dn10 = assign26340_e36605_d_n10;
        locals.var_t1__blk773_dn11 = assign26340_e36605_d_n11;
        locals.var_t1__blk773_dn12 = assign26340_e36605_d_n12;
        locals.var_t1__blk773_dn17 = assign26340_e36605_d_n17;
        locals.var_t1__blk773_rv = 0.0;

        let (assign26350_e36622, assign26350_e36622_d_n0, assign26350_e36622_d_n2, assign26350_e36622_d_n6, assign26350_e36622_d_n7, assign26350_e36622_d_n10, assign26350_e36622_d_n11, assign26350_e36622_d_n12, assign26350_e36622_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard861 != 0.0)) {
        let assign26350_e36618: f64 = (locals.var_cnst0over * locals.var_fs01__blk838);
        let assign26350_e36620: f64 = (assign26350_e36618 * locals.var_t1__blk773);
        (assign26350_e36620, ((((locals.var_cnst0over_dn0 * locals.var_fs01__blk838) + (locals.var_cnst0over * locals.var_fs01__blk838_dn0)) * locals.var_t1__blk773) + (assign26350_e36618 * locals.var_t1__blk773_dn0)), ((((locals.var_cnst0over_dn2 * locals.var_fs01__blk838) + (locals.var_cnst0over * locals.var_fs01__blk838_dn2)) * locals.var_t1__blk773) + (assign26350_e36618 * locals.var_t1__blk773_dn2)), ((((locals.var_cnst0over_dn6 * locals.var_fs01__blk838) + (locals.var_cnst0over * locals.var_fs01__blk838_dn6)) * locals.var_t1__blk773) + (assign26350_e36618 * locals.var_t1__blk773_dn6)), ((((locals.var_cnst0over_dn7 * locals.var_fs01__blk838) + (locals.var_cnst0over * locals.var_fs01__blk838_dn7)) * locals.var_t1__blk773) + (assign26350_e36618 * locals.var_t1__blk773_dn7)), ((((locals.var_cnst0over_dn10 * locals.var_fs01__blk838) + (locals.var_cnst0over * locals.var_fs01__blk838_dn10)) * locals.var_t1__blk773) + (assign26350_e36618 * locals.var_t1__blk773_dn10)), ((((locals.var_cnst0over_dn11 * locals.var_fs01__blk838) + (locals.var_cnst0over * locals.var_fs01__blk838_dn11)) * locals.var_t1__blk773) + (assign26350_e36618 * locals.var_t1__blk773_dn11)), ((((locals.var_cnst0over_dn12 * locals.var_fs01__blk838) + (locals.var_cnst0over * locals.var_fs01__blk838_dn12)) * locals.var_t1__blk773) + (assign26350_e36618 * locals.var_t1__blk773_dn12)), ((((locals.var_cnst0over_dn17 * locals.var_fs01__blk838) + (locals.var_cnst0over * locals.var_fs01__blk838_dn17)) * locals.var_t1__blk773) + (assign26350_e36618 * locals.var_t1__blk773_dn17)),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn12, locals.var_qiuld_dn17,)
    }
};
        locals.var_qiuld = assign26350_e36622;
        locals.var_qiuld_dn0 = assign26350_e36622_d_n0;
        locals.var_qiuld_dn2 = assign26350_e36622_d_n2;
        locals.var_qiuld_dn6 = assign26350_e36622_d_n6;
        locals.var_qiuld_dn7 = assign26350_e36622_d_n7;
        locals.var_qiuld_dn10 = assign26350_e36622_d_n10;
        locals.var_qiuld_dn11 = assign26350_e36622_d_n11;
        locals.var_qiuld_dn12 = assign26350_e36622_d_n12;
        locals.var_qiuld_dn17 = assign26350_e36622_d_n17;
        locals.var_qiuld_rv = 0.0;

        let (assign26360_e36637, assign26360_e36637_d_n0, assign26360_e36637_d_n2, assign26360_e36637_d_n6, assign26360_e36637_d_n7, assign26360_e36637_d_n10, assign26360_e36637_d_n11, assign26360_e36637_d_n12, assign26360_e36637_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) && (locals.var_guard861 != 0.0)) {
        let assign26360_e36635: f64 = (locals.var_qbuld + locals.var_qiuld);
        (assign26360_e36635, (locals.var_qbuld_dn0 + locals.var_qiuld_dn0), (locals.var_qbuld_dn2 + locals.var_qiuld_dn2), (locals.var_qbuld_dn6 + locals.var_qiuld_dn6), (locals.var_qbuld_dn7 + locals.var_qiuld_dn7), (locals.var_qbuld_dn10 + locals.var_qiuld_dn10), (locals.var_qbuld_dn11 + locals.var_qiuld_dn11), (locals.var_qbuld_dn12 + locals.var_qiuld_dn12), (locals.var_qbuld_dn17 + locals.var_qiuld_dn17),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17,)
    }
};
        locals.var_qsuld = assign26360_e36637;
        locals.var_qsuld_dn0 = assign26360_e36637_d_n0;
        locals.var_qsuld_dn2 = assign26360_e36637_d_n2;
        locals.var_qsuld_dn6 = assign26360_e36637_d_n6;
        locals.var_qsuld_dn7 = assign26360_e36637_d_n7;
        locals.var_qsuld_dn10 = assign26360_e36637_d_n10;
        locals.var_qsuld_dn11 = assign26360_e36637_d_n11;
        locals.var_qsuld_dn12 = assign26360_e36637_d_n12;
        locals.var_qsuld_dn17 = assign26360_e36637_d_n17;
        locals.var_qsuld_rv = 0.0;

        let (assign26370_e36647, assign26370_e36647_d_n0, assign26370_e36647_d_n2, assign26370_e36647_d_n6, assign26370_e36647_d_n7, assign26370_e36647_d_n10, assign26370_e36647_d_n11, assign26370_e36647_d_n12, assign26370_e36647_d_n17,) = {
    if (((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign26370_e36645: f64 = (locals.var_qsuld - locals.var_qbuld);
        (assign26370_e36645, (locals.var_qsuld_dn0 - locals.var_qbuld_dn0), (locals.var_qsuld_dn2 - locals.var_qbuld_dn2), (locals.var_qsuld_dn6 - locals.var_qbuld_dn6), (locals.var_qsuld_dn7 - locals.var_qbuld_dn7), (locals.var_qsuld_dn10 - locals.var_qbuld_dn10), (locals.var_qsuld_dn11 - locals.var_qbuld_dn11), (locals.var_qsuld_dn12 - locals.var_qbuld_dn12), (locals.var_qsuld_dn17 - locals.var_qbuld_dn17),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn12, locals.var_qiuld_dn17,)
    }
};
        locals.var_qiuld = assign26370_e36647;
        locals.var_qiuld_dn0 = assign26370_e36647_d_n0;
        locals.var_qiuld_dn2 = assign26370_e36647_d_n2;
        locals.var_qiuld_dn6 = assign26370_e36647_d_n6;
        locals.var_qiuld_dn7 = assign26370_e36647_d_n7;
        locals.var_qiuld_dn10 = assign26370_e36647_d_n10;
        locals.var_qiuld_dn11 = assign26370_e36647_d_n11;
        locals.var_qiuld_dn12 = assign26370_e36647_d_n12;
        locals.var_qiuld_dn17 = assign26370_e36647_d_n17;
        locals.var_qiuld_rv = 0.0;

        let assign26380_e36650: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard870 = assign26380_e36650;
        locals.var_guard870_rv = 0.0;

        let assign26390_e36653: f64 = if 1.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard871 = assign26390_e36653;
        locals.var_guard871_rv = 0.0;

        let (assign26400_e36668, assign26400_e36668_d_n0, assign26400_e36668_d_n2, assign26400_e36668_d_n6, assign26400_e36668_d_n7, assign26400_e36668_d_n10, assign26400_e36668_d_n11, assign26400_e36668_d_n12, assign26400_e36668_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard870 != 0.0)) && (locals.var_flg_ovloops != 0.0)) {
        let assign26400_e36664: f64 = (-locals.var_uc_areabt);
        let assign26400_e36666: f64 = (assign26400_e36664 * locals.var_qsuld);
        (assign26400_e36666, (assign26400_e36664 * locals.var_qsuld_dn0), (assign26400_e36664 * locals.var_qsuld_dn2), (assign26400_e36664 * locals.var_qsuld_dn6), (assign26400_e36664 * locals.var_qsuld_dn7), (assign26400_e36664 * locals.var_qsuld_dn10), (assign26400_e36664 * locals.var_qsuld_dn11), (assign26400_e36664 * locals.var_qsuld_dn12), (assign26400_e36664 * locals.var_qsuld_dn17),)
    } else {
        (locals.var_qbody_bt_p_sus, locals.var_qbody_bt_p_sus_dn0, locals.var_qbody_bt_p_sus_dn2, locals.var_qbody_bt_p_sus_dn6, locals.var_qbody_bt_p_sus_dn7, locals.var_qbody_bt_p_sus_dn10, locals.var_qbody_bt_p_sus_dn11, locals.var_qbody_bt_p_sus_dn12, locals.var_qbody_bt_p_sus_dn17,)
    }
};
        locals.var_qbody_bt_p_sus = assign26400_e36668;
        locals.var_qbody_bt_p_sus_dn0 = assign26400_e36668_d_n0;
        locals.var_qbody_bt_p_sus_dn2 = assign26400_e36668_d_n2;
        locals.var_qbody_bt_p_sus_dn6 = assign26400_e36668_d_n6;
        locals.var_qbody_bt_p_sus_dn7 = assign26400_e36668_d_n7;
        locals.var_qbody_bt_p_sus_dn10 = assign26400_e36668_d_n10;
        locals.var_qbody_bt_p_sus_dn11 = assign26400_e36668_d_n11;
        locals.var_qbody_bt_p_sus_dn12 = assign26400_e36668_d_n12;
        locals.var_qbody_bt_p_sus_dn17 = assign26400_e36668_d_n17;
        locals.var_qbody_bt_p_sus_rv = 0.0;

        let (assign26410_e36683, assign26410_e36683_d_n0, assign26410_e36683_d_n2, assign26410_e36683_d_n6, assign26410_e36683_d_n7, assign26410_e36683_d_n10, assign26410_e36683_d_n11, assign26410_e36683_d_n12, assign26410_e36683_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard870 != 0.0)) && (locals.var_flg_ovloops != 0.0)) {
        let assign26410_e36679: f64 = (-locals.var_uc_areabt);
        let assign26410_e36681: f64 = (assign26410_e36679 * locals.var_qiuld);
        (assign26410_e36681, (assign26410_e36679 * locals.var_qiuld_dn0), (assign26410_e36679 * locals.var_qiuld_dn2), (assign26410_e36679 * locals.var_qiuld_dn6), (assign26410_e36679 * locals.var_qiuld_dn7), (assign26410_e36679 * locals.var_qiuld_dn10), (assign26410_e36679 * locals.var_qiuld_dn11), (assign26410_e36679 * locals.var_qiuld_dn12), (assign26410_e36679 * locals.var_qiuld_dn17),)
    } else {
        (locals.var_qbody_bt_p_ius, locals.var_qbody_bt_p_ius_dn0, locals.var_qbody_bt_p_ius_dn2, locals.var_qbody_bt_p_ius_dn6, locals.var_qbody_bt_p_ius_dn7, locals.var_qbody_bt_p_ius_dn10, locals.var_qbody_bt_p_ius_dn11, locals.var_qbody_bt_p_ius_dn12, locals.var_qbody_bt_p_ius_dn17,)
    }
};
        locals.var_qbody_bt_p_ius = assign26410_e36683;
        locals.var_qbody_bt_p_ius_dn0 = assign26410_e36683_d_n0;
        locals.var_qbody_bt_p_ius_dn2 = assign26410_e36683_d_n2;
        locals.var_qbody_bt_p_ius_dn6 = assign26410_e36683_d_n6;
        locals.var_qbody_bt_p_ius_dn7 = assign26410_e36683_d_n7;
        locals.var_qbody_bt_p_ius_dn10 = assign26410_e36683_d_n10;
        locals.var_qbody_bt_p_ius_dn11 = assign26410_e36683_d_n11;
        locals.var_qbody_bt_p_ius_dn12 = assign26410_e36683_d_n12;
        locals.var_qbody_bt_p_ius_dn17 = assign26410_e36683_d_n17;
        locals.var_qbody_bt_p_ius_rv = 0.0;

        let (assign26420_e36698, assign26420_e36698_d_n0, assign26420_e36698_d_n2, assign26420_e36698_d_n6, assign26420_e36698_d_n7, assign26420_e36698_d_n10, assign26420_e36698_d_n11, assign26420_e36698_d_n12, assign26420_e36698_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard870 != 0.0)) && (locals.var_flg_ovloopd != 0.0)) {
        let assign26420_e36694: f64 = (-locals.var_uc_areabt);
        let assign26420_e36696: f64 = (assign26420_e36694 * locals.var_qsuld);
        (assign26420_e36696, (assign26420_e36694 * locals.var_qsuld_dn0), (assign26420_e36694 * locals.var_qsuld_dn2), (assign26420_e36694 * locals.var_qsuld_dn6), (assign26420_e36694 * locals.var_qsuld_dn7), (assign26420_e36694 * locals.var_qsuld_dn10), (assign26420_e36694 * locals.var_qsuld_dn11), (assign26420_e36694 * locals.var_qsuld_dn12), (assign26420_e36694 * locals.var_qsuld_dn17),)
    } else {
        (locals.var_qbody_bt_p_sud, locals.var_qbody_bt_p_sud_dn0, locals.var_qbody_bt_p_sud_dn2, locals.var_qbody_bt_p_sud_dn6, locals.var_qbody_bt_p_sud_dn7, locals.var_qbody_bt_p_sud_dn10, locals.var_qbody_bt_p_sud_dn11, locals.var_qbody_bt_p_sud_dn12, locals.var_qbody_bt_p_sud_dn17,)
    }
};
        locals.var_qbody_bt_p_sud = assign26420_e36698;
        locals.var_qbody_bt_p_sud_dn0 = assign26420_e36698_d_n0;
        locals.var_qbody_bt_p_sud_dn2 = assign26420_e36698_d_n2;
        locals.var_qbody_bt_p_sud_dn6 = assign26420_e36698_d_n6;
        locals.var_qbody_bt_p_sud_dn7 = assign26420_e36698_d_n7;
        locals.var_qbody_bt_p_sud_dn10 = assign26420_e36698_d_n10;
        locals.var_qbody_bt_p_sud_dn11 = assign26420_e36698_d_n11;
        locals.var_qbody_bt_p_sud_dn12 = assign26420_e36698_d_n12;
        locals.var_qbody_bt_p_sud_dn17 = assign26420_e36698_d_n17;
        locals.var_qbody_bt_p_sud_rv = 0.0;

        let (assign26430_e36713, assign26430_e36713_d_n0, assign26430_e36713_d_n2, assign26430_e36713_d_n6, assign26430_e36713_d_n7, assign26430_e36713_d_n10, assign26430_e36713_d_n11, assign26430_e36713_d_n12, assign26430_e36713_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard870 != 0.0)) && (locals.var_flg_ovloopd != 0.0)) {
        let assign26430_e36709: f64 = (-locals.var_uc_areabt);
        let assign26430_e36711: f64 = (assign26430_e36709 * locals.var_qiuld);
        (assign26430_e36711, (assign26430_e36709 * locals.var_qiuld_dn0), (assign26430_e36709 * locals.var_qiuld_dn2), (assign26430_e36709 * locals.var_qiuld_dn6), (assign26430_e36709 * locals.var_qiuld_dn7), (assign26430_e36709 * locals.var_qiuld_dn10), (assign26430_e36709 * locals.var_qiuld_dn11), (assign26430_e36709 * locals.var_qiuld_dn12), (assign26430_e36709 * locals.var_qiuld_dn17),)
    } else {
        (locals.var_qbody_bt_p_iud, locals.var_qbody_bt_p_iud_dn0, locals.var_qbody_bt_p_iud_dn2, locals.var_qbody_bt_p_iud_dn6, locals.var_qbody_bt_p_iud_dn7, locals.var_qbody_bt_p_iud_dn10, locals.var_qbody_bt_p_iud_dn11, locals.var_qbody_bt_p_iud_dn12, locals.var_qbody_bt_p_iud_dn17,)
    }
};
        locals.var_qbody_bt_p_iud = assign26430_e36713;
        locals.var_qbody_bt_p_iud_dn0 = assign26430_e36713_d_n0;
        locals.var_qbody_bt_p_iud_dn2 = assign26430_e36713_d_n2;
        locals.var_qbody_bt_p_iud_dn6 = assign26430_e36713_d_n6;
        locals.var_qbody_bt_p_iud_dn7 = assign26430_e36713_d_n7;
        locals.var_qbody_bt_p_iud_dn10 = assign26430_e36713_d_n10;
        locals.var_qbody_bt_p_iud_dn11 = assign26430_e36713_d_n11;
        locals.var_qbody_bt_p_iud_dn12 = assign26430_e36713_d_n12;
        locals.var_qbody_bt_p_iud_dn17 = assign26430_e36713_d_n17;
        locals.var_qbody_bt_p_iud_rv = 0.0;

        let (assign26440_e36731, assign26440_e36731_d_n0, assign26440_e36731_d_n2, assign26440_e36731_d_n6, assign26440_e36731_d_n7, assign26440_e36731_d_n10, assign26440_e36731_d_n11, assign26440_e36731_d_n12, assign26440_e36731_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && ((locals.var_guard871 != 0.0) && (locals.var_guard870 == 0.0))) && (locals.var_flg_ovloops != 0.0)) {
        let assign26440_e36727: f64 = (-locals.var_uc_areabt);
        let assign26440_e36729: f64 = (assign26440_e36727 * locals.var_qsuld);
        (assign26440_e36729, (assign26440_e36727 * locals.var_qsuld_dn0), (assign26440_e36727 * locals.var_qsuld_dn2), (assign26440_e36727 * locals.var_qsuld_dn6), (assign26440_e36727 * locals.var_qsuld_dn7), (assign26440_e36727 * locals.var_qsuld_dn10), (assign26440_e36727 * locals.var_qsuld_dn11), (assign26440_e36727 * locals.var_qsuld_dn12), (assign26440_e36727 * locals.var_qsuld_dn17),)
    } else {
        (locals.var_qbody_bt_n_sus, locals.var_qbody_bt_n_sus_dn0, locals.var_qbody_bt_n_sus_dn2, locals.var_qbody_bt_n_sus_dn6, locals.var_qbody_bt_n_sus_dn7, locals.var_qbody_bt_n_sus_dn10, locals.var_qbody_bt_n_sus_dn11, locals.var_qbody_bt_n_sus_dn12, locals.var_qbody_bt_n_sus_dn17,)
    }
};
        locals.var_qbody_bt_n_sus = assign26440_e36731;
        locals.var_qbody_bt_n_sus_dn0 = assign26440_e36731_d_n0;
        locals.var_qbody_bt_n_sus_dn2 = assign26440_e36731_d_n2;
        locals.var_qbody_bt_n_sus_dn6 = assign26440_e36731_d_n6;
        locals.var_qbody_bt_n_sus_dn7 = assign26440_e36731_d_n7;
        locals.var_qbody_bt_n_sus_dn10 = assign26440_e36731_d_n10;
        locals.var_qbody_bt_n_sus_dn11 = assign26440_e36731_d_n11;
        locals.var_qbody_bt_n_sus_dn12 = assign26440_e36731_d_n12;
        locals.var_qbody_bt_n_sus_dn17 = assign26440_e36731_d_n17;
        locals.var_qbody_bt_n_sus_rv = 0.0;

        let (assign26450_e36749, assign26450_e36749_d_n0, assign26450_e36749_d_n2, assign26450_e36749_d_n6, assign26450_e36749_d_n7, assign26450_e36749_d_n10, assign26450_e36749_d_n11, assign26450_e36749_d_n12, assign26450_e36749_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && ((locals.var_guard871 != 0.0) && (locals.var_guard870 == 0.0))) && (locals.var_flg_ovloops != 0.0)) {
        let assign26450_e36745: f64 = (-locals.var_uc_areabt);
        let assign26450_e36747: f64 = (assign26450_e36745 * locals.var_qiuld);
        (assign26450_e36747, (assign26450_e36745 * locals.var_qiuld_dn0), (assign26450_e36745 * locals.var_qiuld_dn2), (assign26450_e36745 * locals.var_qiuld_dn6), (assign26450_e36745 * locals.var_qiuld_dn7), (assign26450_e36745 * locals.var_qiuld_dn10), (assign26450_e36745 * locals.var_qiuld_dn11), (assign26450_e36745 * locals.var_qiuld_dn12), (assign26450_e36745 * locals.var_qiuld_dn17),)
    } else {
        (locals.var_qbody_bt_n_ius, locals.var_qbody_bt_n_ius_dn0, locals.var_qbody_bt_n_ius_dn2, locals.var_qbody_bt_n_ius_dn6, locals.var_qbody_bt_n_ius_dn7, locals.var_qbody_bt_n_ius_dn10, locals.var_qbody_bt_n_ius_dn11, locals.var_qbody_bt_n_ius_dn12, locals.var_qbody_bt_n_ius_dn17,)
    }
};
        locals.var_qbody_bt_n_ius = assign26450_e36749;
        locals.var_qbody_bt_n_ius_dn0 = assign26450_e36749_d_n0;
        locals.var_qbody_bt_n_ius_dn2 = assign26450_e36749_d_n2;
        locals.var_qbody_bt_n_ius_dn6 = assign26450_e36749_d_n6;
        locals.var_qbody_bt_n_ius_dn7 = assign26450_e36749_d_n7;
        locals.var_qbody_bt_n_ius_dn10 = assign26450_e36749_d_n10;
        locals.var_qbody_bt_n_ius_dn11 = assign26450_e36749_d_n11;
        locals.var_qbody_bt_n_ius_dn12 = assign26450_e36749_d_n12;
        locals.var_qbody_bt_n_ius_dn17 = assign26450_e36749_d_n17;
        locals.var_qbody_bt_n_ius_rv = 0.0;

        let (assign26460_e36767, assign26460_e36767_d_n0, assign26460_e36767_d_n2, assign26460_e36767_d_n6, assign26460_e36767_d_n7, assign26460_e36767_d_n10, assign26460_e36767_d_n11, assign26460_e36767_d_n12, assign26460_e36767_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && ((locals.var_guard871 != 0.0) && (locals.var_guard870 == 0.0))) && (locals.var_flg_ovloopd != 0.0)) {
        let assign26460_e36763: f64 = (-locals.var_uc_areabt);
        let assign26460_e36765: f64 = (assign26460_e36763 * locals.var_qsuld);
        (assign26460_e36765, (assign26460_e36763 * locals.var_qsuld_dn0), (assign26460_e36763 * locals.var_qsuld_dn2), (assign26460_e36763 * locals.var_qsuld_dn6), (assign26460_e36763 * locals.var_qsuld_dn7), (assign26460_e36763 * locals.var_qsuld_dn10), (assign26460_e36763 * locals.var_qsuld_dn11), (assign26460_e36763 * locals.var_qsuld_dn12), (assign26460_e36763 * locals.var_qsuld_dn17),)
    } else {
        (locals.var_qbody_bt_n_sud, locals.var_qbody_bt_n_sud_dn0, locals.var_qbody_bt_n_sud_dn2, locals.var_qbody_bt_n_sud_dn6, locals.var_qbody_bt_n_sud_dn7, locals.var_qbody_bt_n_sud_dn10, locals.var_qbody_bt_n_sud_dn11, locals.var_qbody_bt_n_sud_dn12, locals.var_qbody_bt_n_sud_dn17,)
    }
};
        locals.var_qbody_bt_n_sud = assign26460_e36767;
        locals.var_qbody_bt_n_sud_dn0 = assign26460_e36767_d_n0;
        locals.var_qbody_bt_n_sud_dn2 = assign26460_e36767_d_n2;
        locals.var_qbody_bt_n_sud_dn6 = assign26460_e36767_d_n6;
        locals.var_qbody_bt_n_sud_dn7 = assign26460_e36767_d_n7;
        locals.var_qbody_bt_n_sud_dn10 = assign26460_e36767_d_n10;
        locals.var_qbody_bt_n_sud_dn11 = assign26460_e36767_d_n11;
        locals.var_qbody_bt_n_sud_dn12 = assign26460_e36767_d_n12;
        locals.var_qbody_bt_n_sud_dn17 = assign26460_e36767_d_n17;
        locals.var_qbody_bt_n_sud_rv = 0.0;

        let (assign26470_e36785, assign26470_e36785_d_n0, assign26470_e36785_d_n2, assign26470_e36785_d_n6, assign26470_e36785_d_n7, assign26470_e36785_d_n10, assign26470_e36785_d_n11, assign26470_e36785_d_n12, assign26470_e36785_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && ((locals.var_guard871 != 0.0) && (locals.var_guard870 == 0.0))) && (locals.var_flg_ovloopd != 0.0)) {
        let assign26470_e36781: f64 = (-locals.var_uc_areabt);
        let assign26470_e36783: f64 = (assign26470_e36781 * locals.var_qiuld);
        (assign26470_e36783, (assign26470_e36781 * locals.var_qiuld_dn0), (assign26470_e36781 * locals.var_qiuld_dn2), (assign26470_e36781 * locals.var_qiuld_dn6), (assign26470_e36781 * locals.var_qiuld_dn7), (assign26470_e36781 * locals.var_qiuld_dn10), (assign26470_e36781 * locals.var_qiuld_dn11), (assign26470_e36781 * locals.var_qiuld_dn12), (assign26470_e36781 * locals.var_qiuld_dn17),)
    } else {
        (locals.var_qbody_bt_n_iud, locals.var_qbody_bt_n_iud_dn0, locals.var_qbody_bt_n_iud_dn2, locals.var_qbody_bt_n_iud_dn6, locals.var_qbody_bt_n_iud_dn7, locals.var_qbody_bt_n_iud_dn10, locals.var_qbody_bt_n_iud_dn11, locals.var_qbody_bt_n_iud_dn12, locals.var_qbody_bt_n_iud_dn17,)
    }
};
        locals.var_qbody_bt_n_iud = assign26470_e36785;
        locals.var_qbody_bt_n_iud_dn0 = assign26470_e36785_d_n0;
        locals.var_qbody_bt_n_iud_dn2 = assign26470_e36785_d_n2;
        locals.var_qbody_bt_n_iud_dn6 = assign26470_e36785_d_n6;
        locals.var_qbody_bt_n_iud_dn7 = assign26470_e36785_d_n7;
        locals.var_qbody_bt_n_iud_dn10 = assign26470_e36785_d_n10;
        locals.var_qbody_bt_n_iud_dn11 = assign26470_e36785_d_n11;
        locals.var_qbody_bt_n_iud_dn12 = assign26470_e36785_d_n12;
        locals.var_qbody_bt_n_iud_dn17 = assign26470_e36785_d_n17;
        locals.var_qbody_bt_n_iud_rv = 0.0;

        let (assign26480_e36797,) = {
    if (((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign26480_e36793: f64 = (1.0 - 1.0);
        let assign26480_e36795: f64 = (assign26480_e36793 / 2.0);
        (assign26480_e36795,)
    } else {
        (locals.var_flg_ovloops,)
    }
};
        locals.var_flg_ovloops = assign26480_e36797;
        locals.var_flg_ovloops_rv = 0.0;

        let (assign26490_e36809,) = {
    if (((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign26490_e36805: f64 = (1.0 + 1.0);
        let assign26490_e36807: f64 = (assign26490_e36805 / 2.0);
        (assign26490_e36807,)
    } else {
        (locals.var_flg_ovloopd,)
    }
};
        locals.var_flg_ovloopd = assign26490_e36809;
        locals.var_flg_ovloopd_rv = 0.0;

        let (assign26500_e36825, assign26500_e36825_d_n0, assign26500_e36825_d_n2, assign26500_e36825_d_n6, assign26500_e36825_d_n7, assign26500_e36825_d_n10, assign26500_e36825_d_n11, assign26500_e36825_d_n12, assign26500_e36825_d_n17,) = {
    if (((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign26500_e36817: f64 = (locals.var_modenml * locals.var_vbs);
        let assign26500_e36821: f64 = (locals.var_vbs - locals.var_vds);
        let assign26500_e36822: f64 = (locals.var_modervs * assign26500_e36821);
        let assign26500_e36823: f64 = (assign26500_e36817 + assign26500_e36822);
        (assign26500_e36823, ((locals.var_modenml * locals.var_vbs_dn0) + (locals.var_modervs * (locals.var_vbs_dn0 - locals.var_vds_dn0))), ((locals.var_modenml * locals.var_vbs_dn2) + (locals.var_modervs * (locals.var_vbs_dn2 - locals.var_vds_dn2))), ((locals.var_modenml * locals.var_vbs_dn6) + (locals.var_modervs * (locals.var_vbs_dn6 - locals.var_vds_dn6))), ((locals.var_modenml * locals.var_vbs_dn7) + (locals.var_modervs * (locals.var_vbs_dn7 - locals.var_vds_dn7))), ((locals.var_modenml * locals.var_vbs_dn10) + (locals.var_modervs * (locals.var_vbs_dn10 - locals.var_vds_dn10))), ((locals.var_modenml * locals.var_vbs_dn11) + (locals.var_modervs * (locals.var_vbs_dn11 - locals.var_vds_dn11))), ((locals.var_modenml * locals.var_vbs_dn12) + (locals.var_modervs * (locals.var_vbs_dn12 - locals.var_vds_dn12))), ((locals.var_modenml * locals.var_vbs_dn17) + (locals.var_modervs * (locals.var_vbs_dn17 - locals.var_vds_dn17))),)
    } else {
        (locals.var_vbsgmt, locals.var_vbsgmt_dn0, locals.var_vbsgmt_dn2, locals.var_vbsgmt_dn6, locals.var_vbsgmt_dn7, locals.var_vbsgmt_dn10, locals.var_vbsgmt_dn11, locals.var_vbsgmt_dn12, locals.var_vbsgmt_dn17,)
    }
};
        locals.var_vbsgmt = assign26500_e36825;
        locals.var_vbsgmt_dn0 = assign26500_e36825_d_n0;
        locals.var_vbsgmt_dn2 = assign26500_e36825_d_n2;
        locals.var_vbsgmt_dn6 = assign26500_e36825_d_n6;
        locals.var_vbsgmt_dn7 = assign26500_e36825_d_n7;
        locals.var_vbsgmt_dn10 = assign26500_e36825_d_n10;
        locals.var_vbsgmt_dn11 = assign26500_e36825_d_n11;
        locals.var_vbsgmt_dn12 = assign26500_e36825_d_n12;
        locals.var_vbsgmt_dn17 = assign26500_e36825_d_n17;
        locals.var_vbsgmt_rv = 0.0;

        let (assign26510_e36840, assign26510_e36840_d_n0, assign26510_e36840_d_n2, assign26510_e36840_d_n6, assign26510_e36840_d_n7, assign26510_e36840_d_n10, assign26510_e36840_d_n11, assign26510_e36840_d_n12, assign26510_e36840_d_n17,) = {
    if (((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign26510_e36833: f64 = (locals.var_modenml * locals.var_vds);
        let assign26510_e36836: f64 = (-locals.var_vds);
        let assign26510_e36837: f64 = (locals.var_modervs * assign26510_e36836);
        let assign26510_e36838: f64 = (assign26510_e36833 + assign26510_e36837);
        (assign26510_e36838, ((locals.var_modenml * locals.var_vds_dn0) + (locals.var_modervs * (-locals.var_vds_dn0))), ((locals.var_modenml * locals.var_vds_dn2) + (locals.var_modervs * (-locals.var_vds_dn2))), ((locals.var_modenml * locals.var_vds_dn6) + (locals.var_modervs * (-locals.var_vds_dn6))), ((locals.var_modenml * locals.var_vds_dn7) + (locals.var_modervs * (-locals.var_vds_dn7))), ((locals.var_modenml * locals.var_vds_dn10) + (locals.var_modervs * (-locals.var_vds_dn10))), ((locals.var_modenml * locals.var_vds_dn11) + (locals.var_modervs * (-locals.var_vds_dn11))), ((locals.var_modenml * locals.var_vds_dn12) + (locals.var_modervs * (-locals.var_vds_dn12))), ((locals.var_modenml * locals.var_vds_dn17) + (locals.var_modervs * (-locals.var_vds_dn17))),)
    } else {
        (locals.var_vdsgmt, locals.var_vdsgmt_dn0, locals.var_vdsgmt_dn2, locals.var_vdsgmt_dn6, locals.var_vdsgmt_dn7, locals.var_vdsgmt_dn10, locals.var_vdsgmt_dn11, locals.var_vdsgmt_dn12, locals.var_vdsgmt_dn17,)
    }
};
        locals.var_vdsgmt = assign26510_e36840;
        locals.var_vdsgmt_dn0 = assign26510_e36840_d_n0;
        locals.var_vdsgmt_dn2 = assign26510_e36840_d_n2;
        locals.var_vdsgmt_dn6 = assign26510_e36840_d_n6;
        locals.var_vdsgmt_dn7 = assign26510_e36840_d_n7;
        locals.var_vdsgmt_dn10 = assign26510_e36840_d_n10;
        locals.var_vdsgmt_dn11 = assign26510_e36840_d_n11;
        locals.var_vdsgmt_dn12 = assign26510_e36840_d_n12;
        locals.var_vdsgmt_dn17 = assign26510_e36840_d_n17;
        locals.var_vdsgmt_rv = 0.0;

        let (assign26520_e36856, assign26520_e36856_d_n0, assign26520_e36856_d_n2, assign26520_e36856_d_n6, assign26520_e36856_d_n7, assign26520_e36856_d_n10, assign26520_e36856_d_n11, assign26520_e36856_d_n12, assign26520_e36856_d_n17,) = {
    if (((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign26520_e36848: f64 = (locals.var_modenml * locals.var_vgs);
        let assign26520_e36852: f64 = (locals.var_vgs - locals.var_vds);
        let assign26520_e36853: f64 = (locals.var_modervs * assign26520_e36852);
        let assign26520_e36854: f64 = (assign26520_e36848 + assign26520_e36853);
        (assign26520_e36854, (locals.var_modervs * (-locals.var_vds_dn0)), (locals.var_modervs * (-locals.var_vds_dn2)), ((locals.var_modenml * locals.var_vgs_dn6) + (locals.var_modervs * (locals.var_vgs_dn6 - locals.var_vds_dn6))), ((locals.var_modenml * locals.var_vgs_dn7) + (locals.var_modervs * (locals.var_vgs_dn7 - locals.var_vds_dn7))), (locals.var_modervs * (-locals.var_vds_dn10)), ((locals.var_modenml * locals.var_vgs_dn11) + (locals.var_modervs * (locals.var_vgs_dn11 - locals.var_vds_dn11))), (locals.var_modervs * (-locals.var_vds_dn12)), (locals.var_modervs * (-locals.var_vds_dn17)),)
    } else {
        (locals.var_vgsgmt, locals.var_vgsgmt_dn0, locals.var_vgsgmt_dn2, locals.var_vgsgmt_dn6, locals.var_vgsgmt_dn7, locals.var_vgsgmt_dn10, locals.var_vgsgmt_dn11, locals.var_vgsgmt_dn12, locals.var_vgsgmt_dn17,)
    }
};
        locals.var_vgsgmt = assign26520_e36856;
        locals.var_vgsgmt_dn0 = assign26520_e36856_d_n0;
        locals.var_vgsgmt_dn2 = assign26520_e36856_d_n2;
        locals.var_vgsgmt_dn6 = assign26520_e36856_d_n6;
        locals.var_vgsgmt_dn7 = assign26520_e36856_d_n7;
        locals.var_vgsgmt_dn10 = assign26520_e36856_d_n10;
        locals.var_vgsgmt_dn11 = assign26520_e36856_d_n11;
        locals.var_vgsgmt_dn12 = assign26520_e36856_d_n12;
        locals.var_vgsgmt_dn17 = assign26520_e36856_d_n17;
        locals.var_vgsgmt_rv = 0.0;

        let (assign26530_e36872, assign26530_e36872_d_n0, assign26530_e36872_d_n2, assign26530_e36872_d_n6, assign26530_e36872_d_n7, assign26530_e36872_d_n10, assign26530_e36872_d_n11, assign26530_e36872_d_n12, assign26530_e36872_d_n17,) = {
    if (((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign26530_e36864: f64 = (locals.var_modervs * locals.var_vgs);
        let assign26530_e36868: f64 = (locals.var_vgs - locals.var_vds);
        let assign26530_e36869: f64 = (locals.var_modenml * assign26530_e36868);
        let assign26530_e36870: f64 = (assign26530_e36864 + assign26530_e36869);
        (assign26530_e36870, (locals.var_modenml * (-locals.var_vds_dn0)), (locals.var_modenml * (-locals.var_vds_dn2)), ((locals.var_modervs * locals.var_vgs_dn6) + (locals.var_modenml * (locals.var_vgs_dn6 - locals.var_vds_dn6))), ((locals.var_modervs * locals.var_vgs_dn7) + (locals.var_modenml * (locals.var_vgs_dn7 - locals.var_vds_dn7))), (locals.var_modenml * (-locals.var_vds_dn10)), ((locals.var_modervs * locals.var_vgs_dn11) + (locals.var_modenml * (locals.var_vgs_dn11 - locals.var_vds_dn11))), (locals.var_modenml * (-locals.var_vds_dn12)), (locals.var_modenml * (-locals.var_vds_dn17)),)
    } else {
        (locals.var_vgdgmt, locals.var_vgdgmt_dn0, locals.var_vgdgmt_dn2, locals.var_vgdgmt_dn6, locals.var_vgdgmt_dn7, locals.var_vgdgmt_dn10, locals.var_vgdgmt_dn11, locals.var_vgdgmt_dn12, locals.var_vgdgmt_dn17,)
    }
};
        locals.var_vgdgmt = assign26530_e36872;
        locals.var_vgdgmt_dn0 = assign26530_e36872_d_n0;
        locals.var_vgdgmt_dn2 = assign26530_e36872_d_n2;
        locals.var_vgdgmt_dn6 = assign26530_e36872_d_n6;
        locals.var_vgdgmt_dn7 = assign26530_e36872_d_n7;
        locals.var_vgdgmt_dn10 = assign26530_e36872_d_n10;
        locals.var_vgdgmt_dn11 = assign26530_e36872_d_n11;
        locals.var_vgdgmt_dn12 = assign26530_e36872_d_n12;
        locals.var_vgdgmt_dn17 = assign26530_e36872_d_n17;
        locals.var_vgdgmt_rv = 0.0;

        let (assign26540_e36882, assign26540_e36882_d_n0, assign26540_e36882_d_n2, assign26540_e36882_d_n6, assign26540_e36882_d_n7, assign26540_e36882_d_n10, assign26540_e36882_d_n11, assign26540_e36882_d_n12, assign26540_e36882_d_n17,) = {
    if (((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign26540_e36880: f64 = (locals.var_vdsgmt - locals.var_vbsgmt);
        (assign26540_e36880, (locals.var_vdsgmt_dn0 - locals.var_vbsgmt_dn0), (locals.var_vdsgmt_dn2 - locals.var_vbsgmt_dn2), (locals.var_vdsgmt_dn6 - locals.var_vbsgmt_dn6), (locals.var_vdsgmt_dn7 - locals.var_vbsgmt_dn7), (locals.var_vdsgmt_dn10 - locals.var_vbsgmt_dn10), (locals.var_vdsgmt_dn11 - locals.var_vbsgmt_dn11), (locals.var_vdsgmt_dn12 - locals.var_vbsgmt_dn12), (locals.var_vdsgmt_dn17 - locals.var_vbsgmt_dn17),)
    } else {
        (locals.var_vdbgmt, locals.var_vdbgmt_dn0, locals.var_vdbgmt_dn2, locals.var_vdbgmt_dn6, locals.var_vdbgmt_dn7, locals.var_vdbgmt_dn10, locals.var_vdbgmt_dn11, locals.var_vdbgmt_dn12, locals.var_vdbgmt_dn17,)
    }
};
        locals.var_vdbgmt = assign26540_e36882;
        locals.var_vdbgmt_dn0 = assign26540_e36882_d_n0;
        locals.var_vdbgmt_dn2 = assign26540_e36882_d_n2;
        locals.var_vdbgmt_dn6 = assign26540_e36882_d_n6;
        locals.var_vdbgmt_dn7 = assign26540_e36882_d_n7;
        locals.var_vdbgmt_dn10 = assign26540_e36882_d_n10;
        locals.var_vdbgmt_dn11 = assign26540_e36882_d_n11;
        locals.var_vdbgmt_dn12 = assign26540_e36882_d_n12;
        locals.var_vdbgmt_dn17 = assign26540_e36882_d_n17;
        locals.var_vdbgmt_rv = 0.0;

        let (assign26550_e36891, assign26550_e36891_d_n0, assign26550_e36891_d_n2, assign26550_e36891_d_n6, assign26550_e36891_d_n7, assign26550_e36891_d_n10, assign26550_e36891_d_n11, assign26550_e36891_d_n12, assign26550_e36891_d_n17,) = {
    if (((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign26550_e36889: f64 = (-locals.var_vbsgmt);
        (assign26550_e36889, (-locals.var_vbsgmt_dn0), (-locals.var_vbsgmt_dn2), (-locals.var_vbsgmt_dn6), (-locals.var_vbsgmt_dn7), (-locals.var_vbsgmt_dn10), (-locals.var_vbsgmt_dn11), (-locals.var_vbsgmt_dn12), (-locals.var_vbsgmt_dn17),)
    } else {
        (locals.var_vsbgmt, locals.var_vsbgmt_dn0, locals.var_vsbgmt_dn2, locals.var_vsbgmt_dn6, locals.var_vsbgmt_dn7, locals.var_vsbgmt_dn10, locals.var_vsbgmt_dn11, locals.var_vsbgmt_dn12, locals.var_vsbgmt_dn17,)
    }
};
        locals.var_vsbgmt = assign26550_e36891;
        locals.var_vsbgmt_dn0 = assign26550_e36891_d_n0;
        locals.var_vsbgmt_dn2 = assign26550_e36891_d_n2;
        locals.var_vsbgmt_dn6 = assign26550_e36891_d_n6;
        locals.var_vsbgmt_dn7 = assign26550_e36891_d_n7;
        locals.var_vsbgmt_dn10 = assign26550_e36891_d_n10;
        locals.var_vsbgmt_dn11 = assign26550_e36891_d_n11;
        locals.var_vsbgmt_dn12 = assign26550_e36891_d_n12;
        locals.var_vsbgmt_dn17 = assign26550_e36891_d_n17;
        locals.var_vsbgmt_rv = 0.0;

        let (assign26560_e36905,) = {
    if (((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign26560_e36899: f64 = (locals.var_flg_ovloops * locals.var_modenml);
        let assign26560_e36902: f64 = (locals.var_flg_ovloopd * locals.var_modervs);
        let assign26560_e36903: f64 = (assign26560_e36899 + assign26560_e36902);
        (assign26560_e36903,)
    } else {
        (locals.var_flg_overs,)
    }
};
        locals.var_flg_overs = assign26560_e36905;
        locals.var_flg_overs_rv = 0.0;

    }
}
