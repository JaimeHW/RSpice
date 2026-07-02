#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_176(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign51780_e85922, assign51780_e85922_d_n3, assign51780_e85922_d_n4, assign51780_e85922_d_n5, assign51780_e85922_d_n6, assign51780_e85922_d_n7, assign51780_e85922_d_n8, assign51780_e85922_d_n9, assign51780_e85922_d_n10, assign51780_e85922_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51780_e85918: f64 = (locals.var_bechvbedge * locals.var_t1);
        let assign51780_e85920: f64 = (assign51780_e85918 * locals.var_t2);
        (assign51780_e85920, (((locals.var_bechvbedge * locals.var_t1_dn3) * locals.var_t2) + (assign51780_e85918 * locals.var_t2_dn3)), (((locals.var_bechvbedge * locals.var_t1_dn4) * locals.var_t2) + (assign51780_e85918 * locals.var_t2_dn4)), (((locals.var_bechvbedge * locals.var_t1_dn5) * locals.var_t2) + (assign51780_e85918 * locals.var_t2_dn5)), (((locals.var_bechvbedge * locals.var_t1_dn6) * locals.var_t2) + (assign51780_e85918 * locals.var_t2_dn6)), (((locals.var_bechvbedge * locals.var_t1_dn7) * locals.var_t2) + (assign51780_e85918 * locals.var_t2_dn7)), (((locals.var_bechvbedge * locals.var_t1_dn8) * locals.var_t2) + (assign51780_e85918 * locals.var_t2_dn8)), (((locals.var_bechvbedge * locals.var_t1_dn9) * locals.var_t2) + (assign51780_e85918 * locals.var_t2_dn9)), (((locals.var_bechvbedge * locals.var_t1_dn10) * locals.var_t2) + (assign51780_e85918 * locals.var_t2_dn10)), (((locals.var_bechvbedge * locals.var_t1_dn11) * locals.var_t2) + (assign51780_e85918 * locals.var_t2_dn11)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign51780_e85922;
        locals.var_t3_dn3 = assign51780_e85922_d_n3;
        locals.var_t3_dn4 = assign51780_e85922_d_n4;
        locals.var_t3_dn5 = assign51780_e85922_d_n5;
        locals.var_t3_dn6 = assign51780_e85922_d_n6;
        locals.var_t3_dn7 = assign51780_e85922_d_n7;
        locals.var_t3_dn8 = assign51780_e85922_d_n8;
        locals.var_t3_dn9 = assign51780_e85922_d_n9;
        locals.var_t3_dn10 = assign51780_e85922_d_n10;
        locals.var_t3_dn11 = assign51780_e85922_d_n11;

        let (assign51790_e85932, assign51790_e85932_d_n3, assign51790_e85932_d_n4, assign51790_e85932_d_n5, assign51790_e85932_d_n6, assign51790_e85932_d_n7, assign51790_e85932_d_n8, assign51790_e85932_d_n9, assign51790_e85932_d_n10, assign51790_e85932_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51790_e85930: f64 = { let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign51790_e85930, ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn3), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn4), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn5), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn6), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn7), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn8), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn9), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn10), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn11),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign51790_e85932;
        locals.var_t4_dn3 = assign51790_e85932_d_n3;
        locals.var_t4_dn4 = assign51790_e85932_d_n4;
        locals.var_t4_dn5 = assign51790_e85932_d_n5;
        locals.var_t4_dn6 = assign51790_e85932_d_n6;
        locals.var_t4_dn7 = assign51790_e85932_d_n7;
        locals.var_t4_dn8 = assign51790_e85932_d_n8;
        locals.var_t4_dn9 = assign51790_e85932_d_n9;
        locals.var_t4_dn10 = assign51790_e85932_d_n10;
        locals.var_t4_dn11 = assign51790_e85932_d_n11;

        let (assign51800_e85947, assign51800_e85947_d_n3, assign51800_e85947_d_n4, assign51800_e85947_d_n5, assign51800_e85947_d_n6, assign51800_e85947_d_n7, assign51800_e85947_d_n8, assign51800_e85947_d_n9, assign51800_e85947_d_n10, assign51800_e85947_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51800_e85941: f64 = (locals.var_igtemp * p.p2);
        let assign51800_e85943: f64 = (assign51800_e85941 * locals.var_aechvbedges);
        let assign51800_e85945: f64 = (assign51800_e85943 * locals.var_dlcig_i);
        (assign51800_e85945, ((assign51800_e85941 * locals.var_aechvbedges_dn3) * locals.var_dlcig_i), ((((locals.var_igtemp_dn4 * p.p2) * locals.var_aechvbedges) + (assign51800_e85941 * locals.var_aechvbedges_dn4)) * locals.var_dlcig_i), ((((locals.var_igtemp_dn5 * p.p2) * locals.var_aechvbedges) + (assign51800_e85941 * locals.var_aechvbedges_dn5)) * locals.var_dlcig_i), ((assign51800_e85941 * locals.var_aechvbedges_dn6) * locals.var_dlcig_i), ((assign51800_e85941 * locals.var_aechvbedges_dn7) * locals.var_dlcig_i), ((assign51800_e85941 * locals.var_aechvbedges_dn8) * locals.var_dlcig_i), ((assign51800_e85941 * locals.var_aechvbedges_dn9) * locals.var_dlcig_i), ((assign51800_e85941 * locals.var_aechvbedges_dn10) * locals.var_dlcig_i), ((assign51800_e85941 * locals.var_aechvbedges_dn11) * locals.var_dlcig_i),)
    } else {
        (locals.var_igs_mult, locals.var_igs_mult_dn3, locals.var_igs_mult_dn4, locals.var_igs_mult_dn5, locals.var_igs_mult_dn6, locals.var_igs_mult_dn7, locals.var_igs_mult_dn8, locals.var_igs_mult_dn9, locals.var_igs_mult_dn10, locals.var_igs_mult_dn11,)
    }
};
        locals.var_igs_mult = assign51800_e85947;
        locals.var_igs_mult_dn3 = assign51800_e85947_d_n3;
        locals.var_igs_mult_dn4 = assign51800_e85947_d_n4;
        locals.var_igs_mult_dn5 = assign51800_e85947_d_n5;
        locals.var_igs_mult_dn6 = assign51800_e85947_d_n6;
        locals.var_igs_mult_dn7 = assign51800_e85947_d_n7;
        locals.var_igs_mult_dn8 = assign51800_e85947_d_n8;
        locals.var_igs_mult_dn9 = assign51800_e85947_d_n9;
        locals.var_igs_mult_dn10 = assign51800_e85947_d_n10;
        locals.var_igs_mult_dn11 = assign51800_e85947_d_n11;

        let (assign51810_e85962, assign51810_e85962_d_n3, assign51810_e85962_d_n4, assign51810_e85962_d_n5, assign51810_e85962_d_n6, assign51810_e85962_d_n7, assign51810_e85962_d_n8, assign51810_e85962_d_n9, assign51810_e85962_d_n10, assign51810_e85962_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51810_e85956: f64 = (locals.var_igs_mult * locals.var_vgs_noswap);
        let assign51810_e85958: f64 = (assign51810_e85956 * locals.var_vgs_eff);
        let assign51810_e85960: f64 = (assign51810_e85958 * locals.var_t4);
        (assign51810_e85960, (((((locals.var_igs_mult_dn3 * locals.var_vgs_noswap) * locals.var_vgs_eff) + (assign51810_e85956 * locals.var_vgs_eff_dn3)) * locals.var_t4) + (assign51810_e85958 * locals.var_t4_dn3)), (((((locals.var_igs_mult_dn4 * locals.var_vgs_noswap) * locals.var_vgs_eff) + (assign51810_e85956 * locals.var_vgs_eff_dn4)) * locals.var_t4) + (assign51810_e85958 * locals.var_t4_dn4)), (((((locals.var_igs_mult_dn5 * locals.var_vgs_noswap) * locals.var_vgs_eff) + (assign51810_e85956 * locals.var_vgs_eff_dn5)) * locals.var_t4) + (assign51810_e85958 * locals.var_t4_dn5)), ((((((locals.var_igs_mult_dn6 * locals.var_vgs_noswap) + (locals.var_igs_mult * locals.var_vgs_noswap_dn6)) * locals.var_vgs_eff) + (assign51810_e85956 * locals.var_vgs_eff_dn6)) * locals.var_t4) + (assign51810_e85958 * locals.var_t4_dn6)), ((((((locals.var_igs_mult_dn7 * locals.var_vgs_noswap) + (locals.var_igs_mult * locals.var_vgs_noswap_dn7)) * locals.var_vgs_eff) + (assign51810_e85956 * locals.var_vgs_eff_dn7)) * locals.var_t4) + (assign51810_e85958 * locals.var_t4_dn7)), ((((((locals.var_igs_mult_dn8 * locals.var_vgs_noswap) + (locals.var_igs_mult * locals.var_vgs_noswap_dn8)) * locals.var_vgs_eff) + (assign51810_e85956 * locals.var_vgs_eff_dn8)) * locals.var_t4) + (assign51810_e85958 * locals.var_t4_dn8)), (((((locals.var_igs_mult_dn9 * locals.var_vgs_noswap) * locals.var_vgs_eff) + (assign51810_e85956 * locals.var_vgs_eff_dn9)) * locals.var_t4) + (assign51810_e85958 * locals.var_t4_dn9)), ((((((locals.var_igs_mult_dn10 * locals.var_vgs_noswap) + (locals.var_igs_mult * locals.var_vgs_noswap_dn10)) * locals.var_vgs_eff) + (assign51810_e85956 * locals.var_vgs_eff_dn10)) * locals.var_t4) + (assign51810_e85958 * locals.var_t4_dn10)), (((((locals.var_igs_mult_dn11 * locals.var_vgs_noswap) * locals.var_vgs_eff) + (assign51810_e85956 * locals.var_vgs_eff_dn11)) * locals.var_t4) + (assign51810_e85958 * locals.var_t4_dn11)),)
    } else {
        (locals.var_igs, locals.var_igs_dn3, locals.var_igs_dn4, locals.var_igs_dn5, locals.var_igs_dn6, locals.var_igs_dn7, locals.var_igs_dn8, locals.var_igs_dn9, locals.var_igs_dn10, locals.var_igs_dn11,)
    }
};
        locals.var_igs = assign51810_e85962;
        locals.var_igs_dn3 = assign51810_e85962_d_n3;
        locals.var_igs_dn4 = assign51810_e85962_d_n4;
        locals.var_igs_dn5 = assign51810_e85962_d_n5;
        locals.var_igs_dn6 = assign51810_e85962_d_n6;
        locals.var_igs_dn7 = assign51810_e85962_d_n7;
        locals.var_igs_dn8 = assign51810_e85962_d_n8;
        locals.var_igs_dn9 = assign51810_e85962_d_n9;
        locals.var_igs_dn10 = assign51810_e85962_d_n10;
        locals.var_igs_dn11 = assign51810_e85962_d_n11;

        let (assign51820_e85973, assign51820_e85973_d_n3, assign51820_e85973_d_n4, assign51820_e85973_d_n5, assign51820_e85973_d_n6, assign51820_e85973_d_n7, assign51820_e85973_d_n8, assign51820_e85973_d_n9, assign51820_e85973_d_n10, assign51820_e85973_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51820_e85971: f64 = (locals.var_vgd_noswap - locals.var_vfbsdr);
        (assign51820_e85971, 0.0, (-locals.var_vfbsdr_dn4), (-locals.var_vfbsdr_dn5), locals.var_vgd_noswap_dn6, locals.var_vgd_noswap_dn7, locals.var_vgd_noswap_dn8, 0.0, locals.var_vgd_noswap_dn10, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign51820_e85973;
        locals.var_t2_dn3 = assign51820_e85973_d_n3;
        locals.var_t2_dn4 = assign51820_e85973_d_n4;
        locals.var_t2_dn5 = assign51820_e85973_d_n5;
        locals.var_t2_dn6 = assign51820_e85973_d_n6;
        locals.var_t2_dn7 = assign51820_e85973_d_n7;
        locals.var_t2_dn8 = assign51820_e85973_d_n8;
        locals.var_t2_dn9 = assign51820_e85973_d_n9;
        locals.var_t2_dn10 = assign51820_e85973_d_n10;
        locals.var_t2_dn11 = assign51820_e85973_d_n11;

        let (assign51830_e85987, assign51830_e85987_d_n3, assign51830_e85987_d_n4, assign51830_e85987_d_n5, assign51830_e85987_d_n6, assign51830_e85987_d_n7, assign51830_e85987_d_n8, assign51830_e85987_d_n9, assign51830_e85987_d_n10, assign51830_e85987_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51830_e85982: f64 = (locals.var_t2 * locals.var_t2);
        let assign51830_e85984: f64 = (assign51830_e85982 + 0.0001);
        let assign51830_e85985: f64 = (assign51830_e85984).sqrt();
        (assign51830_e85985, (((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign51830_e85985)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign51830_e85985)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign51830_e85985)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign51830_e85985)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign51830_e85985)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign51830_e85985)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign51830_e85985)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign51830_e85985)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign51830_e85985)),)
    } else {
        (locals.var_vgd_eff, locals.var_vgd_eff_dn3, locals.var_vgd_eff_dn4, locals.var_vgd_eff_dn5, locals.var_vgd_eff_dn6, locals.var_vgd_eff_dn7, locals.var_vgd_eff_dn8, locals.var_vgd_eff_dn9, locals.var_vgd_eff_dn10, locals.var_vgd_eff_dn11,)
    }
};
        locals.var_vgd_eff = assign51830_e85987;
        locals.var_vgd_eff_dn3 = assign51830_e85987_d_n3;
        locals.var_vgd_eff_dn4 = assign51830_e85987_d_n4;
        locals.var_vgd_eff_dn5 = assign51830_e85987_d_n5;
        locals.var_vgd_eff_dn6 = assign51830_e85987_d_n6;
        locals.var_vgd_eff_dn7 = assign51830_e85987_d_n7;
        locals.var_vgd_eff_dn8 = assign51830_e85987_d_n8;
        locals.var_vgd_eff_dn9 = assign51830_e85987_d_n9;
        locals.var_vgd_eff_dn10 = assign51830_e85987_d_n10;
        locals.var_vgd_eff_dn11 = assign51830_e85987_d_n11;

        let assign51840_e85990: f64 = if p.p1295 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard787 = assign51840_e85990;

        let (assign51850_e86026, assign51850_e86026_d_n3, assign51850_e86026_d_n4, assign51850_e86026_d_n5, assign51850_e86026_d_n6, assign51850_e86026_d_n7, assign51850_e86026_d_n8, assign51850_e86026_d_n9, assign51850_e86026_d_n10, assign51850_e86026_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) && (locals.var_guard787 != 0.0)) {
        let assign51850_e86003: f64 = (locals.var_bigd_i * locals.var_vgd_eff);
        let assign51850_e86004: f64 = (locals.var_aigd_i - assign51850_e86003);
        let assign51850_e86008: f64 = (locals.var_bigd_i * locals.var_vgd_eff);
        let assign51850_e86009: f64 = (locals.var_aigd_i - assign51850_e86008);
        let assign51850_e86013: f64 = (locals.var_bigd_i * locals.var_vgd_eff);
        let assign51850_e86014: f64 = (locals.var_aigd_i - assign51850_e86013);
        let assign51850_e86015: f64 = (assign51850_e86009 * assign51850_e86014);
        let assign51850_e86018: f64 = (4.0 * 1e-6);
        let assign51850_e86020: f64 = (assign51850_e86018 * 1e-6);
        let assign51850_e86021: f64 = (assign51850_e86015 + assign51850_e86020);
        let assign51850_e86022: f64 = (assign51850_e86021).sqrt();
        let assign51850_e86023: f64 = (assign51850_e86004 + assign51850_e86022);
        let assign51850_e86024: f64 = (0.5 * assign51850_e86023);
        (assign51850_e86024, (0.5 * ((-(locals.var_bigd_i * locals.var_vgd_eff_dn3)) + ((((-(locals.var_bigd_i * locals.var_vgd_eff_dn3)) * assign51850_e86014) + (assign51850_e86009 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn3)))) / (2.0 * assign51850_e86022)))), (0.5 * ((locals.var_aigd_i_dn4 - (locals.var_bigd_i * locals.var_vgd_eff_dn4)) + ((((locals.var_aigd_i_dn4 - (locals.var_bigd_i * locals.var_vgd_eff_dn4)) * assign51850_e86014) + (assign51850_e86009 * (locals.var_aigd_i_dn4 - (locals.var_bigd_i * locals.var_vgd_eff_dn4)))) / (2.0 * assign51850_e86022)))), (0.5 * ((locals.var_aigd_i_dn5 - (locals.var_bigd_i * locals.var_vgd_eff_dn5)) + ((((locals.var_aigd_i_dn5 - (locals.var_bigd_i * locals.var_vgd_eff_dn5)) * assign51850_e86014) + (assign51850_e86009 * (locals.var_aigd_i_dn5 - (locals.var_bigd_i * locals.var_vgd_eff_dn5)))) / (2.0 * assign51850_e86022)))), (0.5 * ((-(locals.var_bigd_i * locals.var_vgd_eff_dn6)) + ((((-(locals.var_bigd_i * locals.var_vgd_eff_dn6)) * assign51850_e86014) + (assign51850_e86009 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn6)))) / (2.0 * assign51850_e86022)))), (0.5 * ((-(locals.var_bigd_i * locals.var_vgd_eff_dn7)) + ((((-(locals.var_bigd_i * locals.var_vgd_eff_dn7)) * assign51850_e86014) + (assign51850_e86009 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn7)))) / (2.0 * assign51850_e86022)))), (0.5 * ((-(locals.var_bigd_i * locals.var_vgd_eff_dn8)) + ((((-(locals.var_bigd_i * locals.var_vgd_eff_dn8)) * assign51850_e86014) + (assign51850_e86009 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn8)))) / (2.0 * assign51850_e86022)))), (0.5 * ((-(locals.var_bigd_i * locals.var_vgd_eff_dn9)) + ((((-(locals.var_bigd_i * locals.var_vgd_eff_dn9)) * assign51850_e86014) + (assign51850_e86009 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn9)))) / (2.0 * assign51850_e86022)))), (0.5 * ((-(locals.var_bigd_i * locals.var_vgd_eff_dn10)) + ((((-(locals.var_bigd_i * locals.var_vgd_eff_dn10)) * assign51850_e86014) + (assign51850_e86009 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn10)))) / (2.0 * assign51850_e86022)))), (0.5 * ((-(locals.var_bigd_i * locals.var_vgd_eff_dn11)) + ((((-(locals.var_bigd_i * locals.var_vgd_eff_dn11)) * assign51850_e86014) + (assign51850_e86009 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn11)))) / (2.0 * assign51850_e86022)))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign51850_e86026;
        locals.var_t1_dn3 = assign51850_e86026_d_n3;
        locals.var_t1_dn4 = assign51850_e86026_d_n4;
        locals.var_t1_dn5 = assign51850_e86026_d_n5;
        locals.var_t1_dn6 = assign51850_e86026_d_n6;
        locals.var_t1_dn7 = assign51850_e86026_d_n7;
        locals.var_t1_dn8 = assign51850_e86026_d_n8;
        locals.var_t1_dn9 = assign51850_e86026_d_n9;
        locals.var_t1_dn10 = assign51850_e86026_d_n10;
        locals.var_t1_dn11 = assign51850_e86026_d_n11;

        let assign51860_e86029: f64 = if locals.var_cigd_i < 0.01 { 1.0 } else { 0.0 };
        locals.var_guard788 = assign51860_e86029;

        let (assign51870_e86042,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) && (locals.var_guard787 != 0.0)) && (locals.var_guard788 != 0.0)) {
        (0.01,)
    } else {
        (locals.var_cigd_i,)
    }
};
        locals.var_cigd_i = assign51870_e86042;

        let (assign51880_e86058, assign51880_e86058_d_n3, assign51880_e86058_d_n4, assign51880_e86058_d_n5, assign51880_e86058_d_n6, assign51880_e86058_d_n7, assign51880_e86058_d_n8, assign51880_e86058_d_n9, assign51880_e86058_d_n10, assign51880_e86058_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) && (locals.var_guard787 == 0.0)) {
        let assign51880_e86055: f64 = (locals.var_bigd_i * locals.var_vgd_eff);
        let assign51880_e86056: f64 = (locals.var_aigd_i - assign51880_e86055);
        (assign51880_e86056, (-(locals.var_bigd_i * locals.var_vgd_eff_dn3)), (locals.var_aigd_i_dn4 - (locals.var_bigd_i * locals.var_vgd_eff_dn4)), (locals.var_aigd_i_dn5 - (locals.var_bigd_i * locals.var_vgd_eff_dn5)), (-(locals.var_bigd_i * locals.var_vgd_eff_dn6)), (-(locals.var_bigd_i * locals.var_vgd_eff_dn7)), (-(locals.var_bigd_i * locals.var_vgd_eff_dn8)), (-(locals.var_bigd_i * locals.var_vgd_eff_dn9)), (-(locals.var_bigd_i * locals.var_vgd_eff_dn10)), (-(locals.var_bigd_i * locals.var_vgd_eff_dn11)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign51880_e86058;
        locals.var_t1_dn3 = assign51880_e86058_d_n3;
        locals.var_t1_dn4 = assign51880_e86058_d_n4;
        locals.var_t1_dn5 = assign51880_e86058_d_n5;
        locals.var_t1_dn6 = assign51880_e86058_d_n6;
        locals.var_t1_dn7 = assign51880_e86058_d_n7;
        locals.var_t1_dn8 = assign51880_e86058_d_n8;
        locals.var_t1_dn9 = assign51880_e86058_d_n9;
        locals.var_t1_dn10 = assign51880_e86058_d_n10;
        locals.var_t1_dn11 = assign51880_e86058_d_n11;

        let (assign51890_e86071, assign51890_e86071_d_n3, assign51890_e86071_d_n4, assign51890_e86071_d_n5, assign51890_e86071_d_n6, assign51890_e86071_d_n7, assign51890_e86071_d_n8, assign51890_e86071_d_n9, assign51890_e86071_d_n10, assign51890_e86071_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51890_e86068: f64 = (locals.var_cigd_i * locals.var_vgd_eff);
        let assign51890_e86069: f64 = (1.0 + assign51890_e86068);
        (assign51890_e86069, (locals.var_cigd_i * locals.var_vgd_eff_dn3), (locals.var_cigd_i * locals.var_vgd_eff_dn4), (locals.var_cigd_i * locals.var_vgd_eff_dn5), (locals.var_cigd_i * locals.var_vgd_eff_dn6), (locals.var_cigd_i * locals.var_vgd_eff_dn7), (locals.var_cigd_i * locals.var_vgd_eff_dn8), (locals.var_cigd_i * locals.var_vgd_eff_dn9), (locals.var_cigd_i * locals.var_vgd_eff_dn10), (locals.var_cigd_i * locals.var_vgd_eff_dn11),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign51890_e86071;
        locals.var_t2_dn3 = assign51890_e86071_d_n3;
        locals.var_t2_dn4 = assign51890_e86071_d_n4;
        locals.var_t2_dn5 = assign51890_e86071_d_n5;
        locals.var_t2_dn6 = assign51890_e86071_d_n6;
        locals.var_t2_dn7 = assign51890_e86071_d_n7;
        locals.var_t2_dn8 = assign51890_e86071_d_n8;
        locals.var_t2_dn9 = assign51890_e86071_d_n9;
        locals.var_t2_dn10 = assign51890_e86071_d_n10;
        locals.var_t2_dn11 = assign51890_e86071_d_n11;

        let (assign51900_e86084, assign51900_e86084_d_n3, assign51900_e86084_d_n4, assign51900_e86084_d_n5, assign51900_e86084_d_n6, assign51900_e86084_d_n7, assign51900_e86084_d_n8, assign51900_e86084_d_n9, assign51900_e86084_d_n10, assign51900_e86084_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51900_e86080: f64 = (locals.var_bechvbedge * locals.var_t1);
        let assign51900_e86082: f64 = (assign51900_e86080 * locals.var_t2);
        (assign51900_e86082, (((locals.var_bechvbedge * locals.var_t1_dn3) * locals.var_t2) + (assign51900_e86080 * locals.var_t2_dn3)), (((locals.var_bechvbedge * locals.var_t1_dn4) * locals.var_t2) + (assign51900_e86080 * locals.var_t2_dn4)), (((locals.var_bechvbedge * locals.var_t1_dn5) * locals.var_t2) + (assign51900_e86080 * locals.var_t2_dn5)), (((locals.var_bechvbedge * locals.var_t1_dn6) * locals.var_t2) + (assign51900_e86080 * locals.var_t2_dn6)), (((locals.var_bechvbedge * locals.var_t1_dn7) * locals.var_t2) + (assign51900_e86080 * locals.var_t2_dn7)), (((locals.var_bechvbedge * locals.var_t1_dn8) * locals.var_t2) + (assign51900_e86080 * locals.var_t2_dn8)), (((locals.var_bechvbedge * locals.var_t1_dn9) * locals.var_t2) + (assign51900_e86080 * locals.var_t2_dn9)), (((locals.var_bechvbedge * locals.var_t1_dn10) * locals.var_t2) + (assign51900_e86080 * locals.var_t2_dn10)), (((locals.var_bechvbedge * locals.var_t1_dn11) * locals.var_t2) + (assign51900_e86080 * locals.var_t2_dn11)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign51900_e86084;
        locals.var_t3_dn3 = assign51900_e86084_d_n3;
        locals.var_t3_dn4 = assign51900_e86084_d_n4;
        locals.var_t3_dn5 = assign51900_e86084_d_n5;
        locals.var_t3_dn6 = assign51900_e86084_d_n6;
        locals.var_t3_dn7 = assign51900_e86084_d_n7;
        locals.var_t3_dn8 = assign51900_e86084_d_n8;
        locals.var_t3_dn9 = assign51900_e86084_d_n9;
        locals.var_t3_dn10 = assign51900_e86084_d_n10;
        locals.var_t3_dn11 = assign51900_e86084_d_n11;

        let (assign51910_e86094, assign51910_e86094_d_n3, assign51910_e86094_d_n4, assign51910_e86094_d_n5, assign51910_e86094_d_n6, assign51910_e86094_d_n7, assign51910_e86094_d_n8, assign51910_e86094_d_n9, assign51910_e86094_d_n10, assign51910_e86094_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51910_e86092: f64 = { let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign51910_e86092, ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn3), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn4), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn5), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn6), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn7), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn8), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn9), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn10), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn11),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign51910_e86094;
        locals.var_t4_dn3 = assign51910_e86094_d_n3;
        locals.var_t4_dn4 = assign51910_e86094_d_n4;
        locals.var_t4_dn5 = assign51910_e86094_d_n5;
        locals.var_t4_dn6 = assign51910_e86094_d_n6;
        locals.var_t4_dn7 = assign51910_e86094_d_n7;
        locals.var_t4_dn8 = assign51910_e86094_d_n8;
        locals.var_t4_dn9 = assign51910_e86094_d_n9;
        locals.var_t4_dn10 = assign51910_e86094_d_n10;
        locals.var_t4_dn11 = assign51910_e86094_d_n11;

        let (assign51920_e86109, assign51920_e86109_d_n3, assign51920_e86109_d_n4, assign51920_e86109_d_n5, assign51920_e86109_d_n6, assign51920_e86109_d_n7, assign51920_e86109_d_n8, assign51920_e86109_d_n9, assign51920_e86109_d_n10, assign51920_e86109_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51920_e86103: f64 = (locals.var_igtemp * p.p2);
        let assign51920_e86105: f64 = (assign51920_e86103 * locals.var_aechvbedged);
        let assign51920_e86107: f64 = (assign51920_e86105 * locals.var_dlcigd_i);
        (assign51920_e86107, ((assign51920_e86103 * locals.var_aechvbedged_dn3) * locals.var_dlcigd_i), ((((locals.var_igtemp_dn4 * p.p2) * locals.var_aechvbedged) + (assign51920_e86103 * locals.var_aechvbedged_dn4)) * locals.var_dlcigd_i), ((((locals.var_igtemp_dn5 * p.p2) * locals.var_aechvbedged) + (assign51920_e86103 * locals.var_aechvbedged_dn5)) * locals.var_dlcigd_i), ((assign51920_e86103 * locals.var_aechvbedged_dn6) * locals.var_dlcigd_i), ((assign51920_e86103 * locals.var_aechvbedged_dn7) * locals.var_dlcigd_i), ((assign51920_e86103 * locals.var_aechvbedged_dn8) * locals.var_dlcigd_i), ((assign51920_e86103 * locals.var_aechvbedged_dn9) * locals.var_dlcigd_i), ((assign51920_e86103 * locals.var_aechvbedged_dn10) * locals.var_dlcigd_i), ((assign51920_e86103 * locals.var_aechvbedged_dn11) * locals.var_dlcigd_i),)
    } else {
        (locals.var_igd_mult, locals.var_igd_mult_dn3, locals.var_igd_mult_dn4, locals.var_igd_mult_dn5, locals.var_igd_mult_dn6, locals.var_igd_mult_dn7, locals.var_igd_mult_dn8, locals.var_igd_mult_dn9, locals.var_igd_mult_dn10, locals.var_igd_mult_dn11,)
    }
};
        locals.var_igd_mult = assign51920_e86109;
        locals.var_igd_mult_dn3 = assign51920_e86109_d_n3;
        locals.var_igd_mult_dn4 = assign51920_e86109_d_n4;
        locals.var_igd_mult_dn5 = assign51920_e86109_d_n5;
        locals.var_igd_mult_dn6 = assign51920_e86109_d_n6;
        locals.var_igd_mult_dn7 = assign51920_e86109_d_n7;
        locals.var_igd_mult_dn8 = assign51920_e86109_d_n8;
        locals.var_igd_mult_dn9 = assign51920_e86109_d_n9;
        locals.var_igd_mult_dn10 = assign51920_e86109_d_n10;
        locals.var_igd_mult_dn11 = assign51920_e86109_d_n11;

        let (assign51930_e86124, assign51930_e86124_d_n3, assign51930_e86124_d_n4, assign51930_e86124_d_n5, assign51930_e86124_d_n6, assign51930_e86124_d_n7, assign51930_e86124_d_n8, assign51930_e86124_d_n9, assign51930_e86124_d_n10, assign51930_e86124_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51930_e86118: f64 = (locals.var_igd_mult * locals.var_vgd_noswap);
        let assign51930_e86120: f64 = (assign51930_e86118 * locals.var_vgd_eff);
        let assign51930_e86122: f64 = (assign51930_e86120 * locals.var_t4);
        (assign51930_e86122, (((((locals.var_igd_mult_dn3 * locals.var_vgd_noswap) * locals.var_vgd_eff) + (assign51930_e86118 * locals.var_vgd_eff_dn3)) * locals.var_t4) + (assign51930_e86120 * locals.var_t4_dn3)), (((((locals.var_igd_mult_dn4 * locals.var_vgd_noswap) * locals.var_vgd_eff) + (assign51930_e86118 * locals.var_vgd_eff_dn4)) * locals.var_t4) + (assign51930_e86120 * locals.var_t4_dn4)), (((((locals.var_igd_mult_dn5 * locals.var_vgd_noswap) * locals.var_vgd_eff) + (assign51930_e86118 * locals.var_vgd_eff_dn5)) * locals.var_t4) + (assign51930_e86120 * locals.var_t4_dn5)), ((((((locals.var_igd_mult_dn6 * locals.var_vgd_noswap) + (locals.var_igd_mult * locals.var_vgd_noswap_dn6)) * locals.var_vgd_eff) + (assign51930_e86118 * locals.var_vgd_eff_dn6)) * locals.var_t4) + (assign51930_e86120 * locals.var_t4_dn6)), ((((((locals.var_igd_mult_dn7 * locals.var_vgd_noswap) + (locals.var_igd_mult * locals.var_vgd_noswap_dn7)) * locals.var_vgd_eff) + (assign51930_e86118 * locals.var_vgd_eff_dn7)) * locals.var_t4) + (assign51930_e86120 * locals.var_t4_dn7)), ((((((locals.var_igd_mult_dn8 * locals.var_vgd_noswap) + (locals.var_igd_mult * locals.var_vgd_noswap_dn8)) * locals.var_vgd_eff) + (assign51930_e86118 * locals.var_vgd_eff_dn8)) * locals.var_t4) + (assign51930_e86120 * locals.var_t4_dn8)), (((((locals.var_igd_mult_dn9 * locals.var_vgd_noswap) * locals.var_vgd_eff) + (assign51930_e86118 * locals.var_vgd_eff_dn9)) * locals.var_t4) + (assign51930_e86120 * locals.var_t4_dn9)), ((((((locals.var_igd_mult_dn10 * locals.var_vgd_noswap) + (locals.var_igd_mult * locals.var_vgd_noswap_dn10)) * locals.var_vgd_eff) + (assign51930_e86118 * locals.var_vgd_eff_dn10)) * locals.var_t4) + (assign51930_e86120 * locals.var_t4_dn10)), (((((locals.var_igd_mult_dn11 * locals.var_vgd_noswap) * locals.var_vgd_eff) + (assign51930_e86118 * locals.var_vgd_eff_dn11)) * locals.var_t4) + (assign51930_e86120 * locals.var_t4_dn11)),)
    } else {
        (locals.var_igd, locals.var_igd_dn3, locals.var_igd_dn4, locals.var_igd_dn5, locals.var_igd_dn6, locals.var_igd_dn7, locals.var_igd_dn8, locals.var_igd_dn9, locals.var_igd_dn10, locals.var_igd_dn11,)
    }
};
        locals.var_igd = assign51930_e86124;
        locals.var_igd_dn3 = assign51930_e86124_d_n3;
        locals.var_igd_dn4 = assign51930_e86124_d_n4;
        locals.var_igd_dn5 = assign51930_e86124_d_n5;
        locals.var_igd_dn6 = assign51930_e86124_d_n6;
        locals.var_igd_dn7 = assign51930_e86124_d_n7;
        locals.var_igd_dn8 = assign51930_e86124_d_n8;
        locals.var_igd_dn9 = assign51930_e86124_d_n9;
        locals.var_igd_dn10 = assign51930_e86124_d_n10;
        locals.var_igd_dn11 = assign51930_e86124_d_n11;

        let (assign51940_e86131, assign51940_e86131_d_n3, assign51940_e86131_d_n4, assign51940_e86131_d_n5, assign51940_e86131_d_n6, assign51940_e86131_d_n7, assign51940_e86131_d_n8, assign51940_e86131_d_n9, assign51940_e86131_d_n10, assign51940_e86131_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign51940_e86129: f64 = (locals.var_devsign * locals.var_igs);
        (assign51940_e86129, (locals.var_devsign * locals.var_igs_dn3), (locals.var_devsign * locals.var_igs_dn4), (locals.var_devsign * locals.var_igs_dn5), (locals.var_devsign * locals.var_igs_dn6), (locals.var_devsign * locals.var_igs_dn7), (locals.var_devsign * locals.var_igs_dn8), (locals.var_devsign * locals.var_igs_dn9), (locals.var_devsign * locals.var_igs_dn10), (locals.var_devsign * locals.var_igs_dn11),)
    } else {
        (locals.var_igs_1, locals.var_igs_1_dn3, locals.var_igs_1_dn4, locals.var_igs_1_dn5, locals.var_igs_1_dn6, locals.var_igs_1_dn7, locals.var_igs_1_dn8, locals.var_igs_1_dn9, locals.var_igs_1_dn10, locals.var_igs_1_dn11,)
    }
};
        locals.var_igs_1 = assign51940_e86131;
        locals.var_igs_1_dn3 = assign51940_e86131_d_n3;
        locals.var_igs_1_dn4 = assign51940_e86131_d_n4;
        locals.var_igs_1_dn5 = assign51940_e86131_d_n5;
        locals.var_igs_1_dn6 = assign51940_e86131_d_n6;
        locals.var_igs_1_dn7 = assign51940_e86131_d_n7;
        locals.var_igs_1_dn8 = assign51940_e86131_d_n8;
        locals.var_igs_1_dn9 = assign51940_e86131_d_n9;
        locals.var_igs_1_dn10 = assign51940_e86131_d_n10;
        locals.var_igs_1_dn11 = assign51940_e86131_d_n11;

        let (assign51950_e86138, assign51950_e86138_d_n3, assign51950_e86138_d_n4, assign51950_e86138_d_n5, assign51950_e86138_d_n6, assign51950_e86138_d_n7, assign51950_e86138_d_n8, assign51950_e86138_d_n9, assign51950_e86138_d_n10, assign51950_e86138_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign51950_e86136: f64 = (locals.var_devsign * locals.var_igd);
        (assign51950_e86136, (locals.var_devsign * locals.var_igd_dn3), (locals.var_devsign * locals.var_igd_dn4), (locals.var_devsign * locals.var_igd_dn5), (locals.var_devsign * locals.var_igd_dn6), (locals.var_devsign * locals.var_igd_dn7), (locals.var_devsign * locals.var_igd_dn8), (locals.var_devsign * locals.var_igd_dn9), (locals.var_devsign * locals.var_igd_dn10), (locals.var_devsign * locals.var_igd_dn11),)
    } else {
        (locals.var_igd_1, locals.var_igd_1_dn3, locals.var_igd_1_dn4, locals.var_igd_1_dn5, locals.var_igd_1_dn6, locals.var_igd_1_dn7, locals.var_igd_1_dn8, locals.var_igd_1_dn9, locals.var_igd_1_dn10, locals.var_igd_1_dn11,)
    }
};
        locals.var_igd_1 = assign51950_e86138;
        locals.var_igd_1_dn3 = assign51950_e86138_d_n3;
        locals.var_igd_1_dn4 = assign51950_e86138_d_n4;
        locals.var_igd_1_dn5 = assign51950_e86138_d_n5;
        locals.var_igd_1_dn6 = assign51950_e86138_d_n6;
        locals.var_igd_1_dn7 = assign51950_e86138_d_n7;
        locals.var_igd_1_dn8 = assign51950_e86138_d_n8;
        locals.var_igd_1_dn9 = assign51950_e86138_d_n9;
        locals.var_igd_1_dn10 = assign51950_e86138_d_n10;
        locals.var_igd_1_dn11 = assign51950_e86138_d_n11;

        let (assign51960_e86145, assign51960_e86145_d_n3, assign51960_e86145_d_n4, assign51960_e86145_d_n5, assign51960_e86145_d_n6, assign51960_e86145_d_n7, assign51960_e86145_d_n8, assign51960_e86145_d_n9, assign51960_e86145_d_n10, assign51960_e86145_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign51960_e86143: f64 = (locals.var_devsign * locals.var_igb);
        (assign51960_e86143, (locals.var_devsign * locals.var_igb_dn3), (locals.var_devsign * locals.var_igb_dn4), (locals.var_devsign * locals.var_igb_dn5), (locals.var_devsign * locals.var_igb_dn6), (locals.var_devsign * locals.var_igb_dn7), (locals.var_devsign * locals.var_igb_dn8), (locals.var_devsign * locals.var_igb_dn9), (locals.var_devsign * locals.var_igb_dn10), (locals.var_devsign * locals.var_igb_dn11),)
    } else {
        (locals.var_igb_1, locals.var_igb_1_dn3, locals.var_igb_1_dn4, locals.var_igb_1_dn5, locals.var_igb_1_dn6, locals.var_igb_1_dn7, locals.var_igb_1_dn8, locals.var_igb_1_dn9, locals.var_igb_1_dn10, locals.var_igb_1_dn11,)
    }
};
        locals.var_igb_1 = assign51960_e86145;
        locals.var_igb_1_dn3 = assign51960_e86145_d_n3;
        locals.var_igb_1_dn4 = assign51960_e86145_d_n4;
        locals.var_igb_1_dn5 = assign51960_e86145_d_n5;
        locals.var_igb_1_dn6 = assign51960_e86145_d_n6;
        locals.var_igb_1_dn7 = assign51960_e86145_d_n7;
        locals.var_igb_1_dn8 = assign51960_e86145_d_n8;
        locals.var_igb_1_dn9 = assign51960_e86145_d_n9;
        locals.var_igb_1_dn10 = assign51960_e86145_d_n10;
        locals.var_igb_1_dn11 = assign51960_e86145_d_n11;

        let (assign51970_e86152, assign51970_e86152_d_n3, assign51970_e86152_d_n4, assign51970_e86152_d_n5, assign51970_e86152_d_n6, assign51970_e86152_d_n7, assign51970_e86152_d_n8, assign51970_e86152_d_n9, assign51970_e86152_d_n10, assign51970_e86152_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign51970_e86150: f64 = (locals.var_devsign * locals.var_igcs);
        (assign51970_e86150, (locals.var_devsign * locals.var_igcs_dn3), (locals.var_devsign * locals.var_igcs_dn4), (locals.var_devsign * locals.var_igcs_dn5), (locals.var_devsign * locals.var_igcs_dn6), (locals.var_devsign * locals.var_igcs_dn7), (locals.var_devsign * locals.var_igcs_dn8), (locals.var_devsign * locals.var_igcs_dn9), (locals.var_devsign * locals.var_igcs_dn10), (locals.var_devsign * locals.var_igcs_dn11),)
    } else {
        (locals.var_igcs_1, locals.var_igcs_1_dn3, locals.var_igcs_1_dn4, locals.var_igcs_1_dn5, locals.var_igcs_1_dn6, locals.var_igcs_1_dn7, locals.var_igcs_1_dn8, locals.var_igcs_1_dn9, locals.var_igcs_1_dn10, locals.var_igcs_1_dn11,)
    }
};
        locals.var_igcs_1 = assign51970_e86152;
        locals.var_igcs_1_dn3 = assign51970_e86152_d_n3;
        locals.var_igcs_1_dn4 = assign51970_e86152_d_n4;
        locals.var_igcs_1_dn5 = assign51970_e86152_d_n5;
        locals.var_igcs_1_dn6 = assign51970_e86152_d_n6;
        locals.var_igcs_1_dn7 = assign51970_e86152_d_n7;
        locals.var_igcs_1_dn8 = assign51970_e86152_d_n8;
        locals.var_igcs_1_dn9 = assign51970_e86152_d_n9;
        locals.var_igcs_1_dn10 = assign51970_e86152_d_n10;
        locals.var_igcs_1_dn11 = assign51970_e86152_d_n11;

        let (assign51980_e86159, assign51980_e86159_d_n3, assign51980_e86159_d_n4, assign51980_e86159_d_n5, assign51980_e86159_d_n6, assign51980_e86159_d_n7, assign51980_e86159_d_n8, assign51980_e86159_d_n9, assign51980_e86159_d_n10, assign51980_e86159_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign51980_e86157: f64 = (locals.var_devsign * locals.var_igcd);
        (assign51980_e86157, (locals.var_devsign * locals.var_igcd_dn3), (locals.var_devsign * locals.var_igcd_dn4), (locals.var_devsign * locals.var_igcd_dn5), (locals.var_devsign * locals.var_igcd_dn6), (locals.var_devsign * locals.var_igcd_dn7), (locals.var_devsign * locals.var_igcd_dn8), (locals.var_devsign * locals.var_igcd_dn9), (locals.var_devsign * locals.var_igcd_dn10), (locals.var_devsign * locals.var_igcd_dn11),)
    } else {
        (locals.var_igcd_1, locals.var_igcd_1_dn3, locals.var_igcd_1_dn4, locals.var_igcd_1_dn5, locals.var_igcd_1_dn6, locals.var_igcd_1_dn7, locals.var_igcd_1_dn8, locals.var_igcd_1_dn9, locals.var_igcd_1_dn10, locals.var_igcd_1_dn11,)
    }
};
        locals.var_igcd_1 = assign51980_e86159;
        locals.var_igcd_1_dn3 = assign51980_e86159_d_n3;
        locals.var_igcd_1_dn4 = assign51980_e86159_d_n4;
        locals.var_igcd_1_dn5 = assign51980_e86159_d_n5;
        locals.var_igcd_1_dn6 = assign51980_e86159_d_n6;
        locals.var_igcd_1_dn7 = assign51980_e86159_d_n7;
        locals.var_igcd_1_dn8 = assign51980_e86159_d_n8;
        locals.var_igcd_1_dn9 = assign51980_e86159_d_n9;
        locals.var_igcd_1_dn10 = assign51980_e86159_d_n10;
        locals.var_igcd_1_dn11 = assign51980_e86159_d_n11;

        let (assign51990_e86166, assign51990_e86166_d_n3, assign51990_e86166_d_n4, assign51990_e86166_d_n5, assign51990_e86166_d_n6, assign51990_e86166_d_n7, assign51990_e86166_d_n8, assign51990_e86166_d_n9, assign51990_e86166_d_n10, assign51990_e86166_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign51990_e86164: f64 = (locals.var_cjs_t * locals.var_aseff);
        (assign51990_e86164, (locals.var_cjs_t * locals.var_aseff_dn3), ((locals.var_cjs_t_dn4 * locals.var_aseff) + (locals.var_cjs_t * locals.var_aseff_dn4)), ((locals.var_cjs_t_dn5 * locals.var_aseff) + (locals.var_cjs_t * locals.var_aseff_dn5)), (locals.var_cjs_t * locals.var_aseff_dn6), (locals.var_cjs_t * locals.var_aseff_dn7), (locals.var_cjs_t * locals.var_aseff_dn8), (locals.var_cjs_t * locals.var_aseff_dn9), (locals.var_cjs_t * locals.var_aseff_dn10), (locals.var_cjs_t * locals.var_aseff_dn11),)
    } else {
        (locals.var_czbs, locals.var_czbs_dn3, locals.var_czbs_dn4, locals.var_czbs_dn5, locals.var_czbs_dn6, locals.var_czbs_dn7, locals.var_czbs_dn8, locals.var_czbs_dn9, locals.var_czbs_dn10, locals.var_czbs_dn11,)
    }
};
        locals.var_czbs = assign51990_e86166;
        locals.var_czbs_dn3 = assign51990_e86166_d_n3;
        locals.var_czbs_dn4 = assign51990_e86166_d_n4;
        locals.var_czbs_dn5 = assign51990_e86166_d_n5;
        locals.var_czbs_dn6 = assign51990_e86166_d_n6;
        locals.var_czbs_dn7 = assign51990_e86166_d_n7;
        locals.var_czbs_dn8 = assign51990_e86166_d_n8;
        locals.var_czbs_dn9 = assign51990_e86166_d_n9;
        locals.var_czbs_dn10 = assign51990_e86166_d_n10;
        locals.var_czbs_dn11 = assign51990_e86166_d_n11;

        let (assign52000_e86173, assign52000_e86173_d_n3, assign52000_e86173_d_n4, assign52000_e86173_d_n5, assign52000_e86173_d_n6, assign52000_e86173_d_n7, assign52000_e86173_d_n8, assign52000_e86173_d_n9, assign52000_e86173_d_n10, assign52000_e86173_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign52000_e86171: f64 = (locals.var_cjsws_t * locals.var_pseff);
        (assign52000_e86171, (locals.var_cjsws_t * locals.var_pseff_dn3), ((locals.var_cjsws_t_dn4 * locals.var_pseff) + (locals.var_cjsws_t * locals.var_pseff_dn4)), ((locals.var_cjsws_t_dn5 * locals.var_pseff) + (locals.var_cjsws_t * locals.var_pseff_dn5)), (locals.var_cjsws_t * locals.var_pseff_dn6), (locals.var_cjsws_t * locals.var_pseff_dn7), (locals.var_cjsws_t * locals.var_pseff_dn8), (locals.var_cjsws_t * locals.var_pseff_dn9), (locals.var_cjsws_t * locals.var_pseff_dn10), (locals.var_cjsws_t * locals.var_pseff_dn11),)
    } else {
        (locals.var_czbssw, locals.var_czbssw_dn3, locals.var_czbssw_dn4, locals.var_czbssw_dn5, locals.var_czbssw_dn6, locals.var_czbssw_dn7, locals.var_czbssw_dn8, locals.var_czbssw_dn9, locals.var_czbssw_dn10, locals.var_czbssw_dn11,)
    }
};
        locals.var_czbssw = assign52000_e86173;
        locals.var_czbssw_dn3 = assign52000_e86173_d_n3;
        locals.var_czbssw_dn4 = assign52000_e86173_d_n4;
        locals.var_czbssw_dn5 = assign52000_e86173_d_n5;
        locals.var_czbssw_dn6 = assign52000_e86173_d_n6;
        locals.var_czbssw_dn7 = assign52000_e86173_d_n7;
        locals.var_czbssw_dn8 = assign52000_e86173_d_n8;
        locals.var_czbssw_dn9 = assign52000_e86173_d_n9;
        locals.var_czbssw_dn10 = assign52000_e86173_d_n10;
        locals.var_czbssw_dn11 = assign52000_e86173_d_n11;

        let (assign52010_e86182, assign52010_e86182_d_n4, assign52010_e86182_d_n5,) = {
    if (locals.var_guard492 == 0.0) {
        let assign52010_e86178: f64 = (locals.var_cjswgs_t * locals.var_weffcj);
        let assign52010_e86180: f64 = (assign52010_e86178 * p.p2);
        (assign52010_e86180, ((locals.var_cjswgs_t_dn4 * locals.var_weffcj) * p.p2), ((locals.var_cjswgs_t_dn5 * locals.var_weffcj) * p.p2),)
    } else {
        (locals.var_czbsswg, locals.var_czbsswg_dn4, locals.var_czbsswg_dn5,)
    }
};
        locals.var_czbsswg = assign52010_e86182;
        locals.var_czbsswg_dn4 = assign52010_e86182_d_n4;
        locals.var_czbsswg_dn5 = assign52010_e86182_d_n5;

        let (assign52020_e86190,) = {
    if (locals.var_guard492 == 0.0) {
        let assign52020_e86187: f64 = (-p.p913);
        let assign52020_e86188: f64 = (0.1_f64).powf(assign52020_e86187);
        (assign52020_e86188,)
    } else {
        (locals.var_czbs_p1,)
    }
};
        locals.var_czbs_p1 = assign52020_e86190;

        let assign52030_e86193: f64 = if p.p913 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard789 = assign52030_e86193;

        let (assign52040_e86203,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard789 != 0.0)) {
        let assign52040_e86200: f64 = (0.1_f64).ln();
        let assign52040_e86201: f64 = (1.5 - assign52040_e86200);
        (assign52040_e86201,)
    } else {
        (locals.var_czbs_p2,)
    }
};
        locals.var_czbs_p2 = assign52040_e86203;

        let (assign52050_e86227,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard789 == 0.0)) {
        let assign52050_e86212: f64 = (1.0 - p.p913);
        let assign52050_e86213: f64 = (1.0 / assign52050_e86212);
        let assign52050_e86217: f64 = (0.05 * p.p913);
        let assign52050_e86220: f64 = (1.0 + p.p913);
        let assign52050_e86221: f64 = (assign52050_e86217 * assign52050_e86220);
        let assign52050_e86223: f64 = (assign52050_e86221 * locals.var_czbs_p1);
        let assign52050_e86224: f64 = (1.0 - assign52050_e86223);
        let assign52050_e86225: f64 = (assign52050_e86213 * assign52050_e86224);
        (assign52050_e86225,)
    } else {
        (locals.var_czbs_p2,)
    }
};
        locals.var_czbs_p2 = assign52050_e86227;

        let (assign52060_e86235,) = {
    if (locals.var_guard492 == 0.0) {
        let assign52060_e86232: f64 = (-p.p915);
        let assign52060_e86233: f64 = (0.1_f64).powf(assign52060_e86232);
        (assign52060_e86233,)
    } else {
        (locals.var_czbssw_p1,)
    }
};
        locals.var_czbssw_p1 = assign52060_e86235;

        let assign52070_e86238: f64 = if p.p915 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard790 = assign52070_e86238;

        let (assign52080_e86248,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard790 != 0.0)) {
        let assign52080_e86245: f64 = (0.1_f64).ln();
        let assign52080_e86246: f64 = (1.5 - assign52080_e86245);
        (assign52080_e86246,)
    } else {
        (locals.var_czbssw_p2,)
    }
};
        locals.var_czbssw_p2 = assign52080_e86248;

    }

    pub(super) fn stamp_transient_block_177(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign52090_e86272,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard790 == 0.0)) {
        let assign52090_e86257: f64 = (1.0 - p.p915);
        let assign52090_e86258: f64 = (1.0 / assign52090_e86257);
        let assign52090_e86262: f64 = (0.05 * p.p915);
        let assign52090_e86265: f64 = (1.0 + p.p915);
        let assign52090_e86266: f64 = (assign52090_e86262 * assign52090_e86265);
        let assign52090_e86268: f64 = (assign52090_e86266 * locals.var_czbssw_p1);
        let assign52090_e86269: f64 = (1.0 - assign52090_e86268);
        let assign52090_e86270: f64 = (assign52090_e86258 * assign52090_e86269);
        (assign52090_e86270,)
    } else {
        (locals.var_czbssw_p2,)
    }
};
        locals.var_czbssw_p2 = assign52090_e86272;

        let (assign52100_e86280,) = {
    if (locals.var_guard492 == 0.0) {
        let assign52100_e86277: f64 = (-p.p917);
        let assign52100_e86278: f64 = (0.1_f64).powf(assign52100_e86277);
        (assign52100_e86278,)
    } else {
        (locals.var_czbsswg_p1,)
    }
};
        locals.var_czbsswg_p1 = assign52100_e86280;

        let assign52110_e86283: f64 = if p.p917 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard791 = assign52110_e86283;

        let (assign52120_e86293,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard791 != 0.0)) {
        let assign52120_e86290: f64 = (0.1_f64).ln();
        let assign52120_e86291: f64 = (1.5 - assign52120_e86290);
        (assign52120_e86291,)
    } else {
        (locals.var_czbsswg_p2,)
    }
};
        locals.var_czbsswg_p2 = assign52120_e86293;

        let (assign52130_e86317,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard791 == 0.0)) {
        let assign52130_e86302: f64 = (1.0 - p.p917);
        let assign52130_e86303: f64 = (1.0 / assign52130_e86302);
        let assign52130_e86307: f64 = (0.05 * p.p917);
        let assign52130_e86310: f64 = (1.0 + p.p917);
        let assign52130_e86311: f64 = (assign52130_e86307 * assign52130_e86310);
        let assign52130_e86313: f64 = (assign52130_e86311 * locals.var_czbsswg_p1);
        let assign52130_e86314: f64 = (1.0 - assign52130_e86313);
        let assign52130_e86315: f64 = (assign52130_e86303 * assign52130_e86314);
        (assign52130_e86315,)
    } else {
        (locals.var_czbsswg_p2,)
    }
};
        locals.var_czbsswg_p2 = assign52130_e86317;

        let assign52140_e86320: f64 = if locals.var_czbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard792 = assign52140_e86320;

        let (assign52150_e86329, assign52150_e86329_d_n3, assign52150_e86329_d_n4, assign52150_e86329_d_n5, assign52150_e86329_d_n6, assign52150_e86329_d_n7, assign52150_e86329_d_n8, assign52150_e86329_d_n9, assign52150_e86329_d_n10, assign52150_e86329_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard792 != 0.0)) {
        let assign52150_e86327: f64 = (locals.var_vbs_jct / locals.var_pbs_t);
        (assign52150_e86327, 0.0, (-((locals.var_vbs_jct * locals.var_pbs_t_dn4) / (locals.var_pbs_t * locals.var_pbs_t))), (-((locals.var_vbs_jct * locals.var_pbs_t_dn5) / (locals.var_pbs_t * locals.var_pbs_t))), 0.0, (locals.var_vbs_jct_dn7 / locals.var_pbs_t), 0.0, 0.0, (locals.var_vbs_jct_dn10 / locals.var_pbs_t), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign52150_e86329;
        locals.var_t1_dn3 = assign52150_e86329_d_n3;
        locals.var_t1_dn4 = assign52150_e86329_d_n4;
        locals.var_t1_dn5 = assign52150_e86329_d_n5;
        locals.var_t1_dn6 = assign52150_e86329_d_n6;
        locals.var_t1_dn7 = assign52150_e86329_d_n7;
        locals.var_t1_dn8 = assign52150_e86329_d_n8;
        locals.var_t1_dn9 = assign52150_e86329_d_n9;
        locals.var_t1_dn10 = assign52150_e86329_d_n10;
        locals.var_t1_dn11 = assign52150_e86329_d_n11;

        let assign52160_e86332: f64 = if locals.var_t1 < 0.9 { 1.0 } else { 0.0 };
        locals.var_guard793 = assign52160_e86332;

        let (assign52170_e86343, assign52170_e86343_d_n3, assign52170_e86343_d_n4, assign52170_e86343_d_n5, assign52170_e86343_d_n6, assign52170_e86343_d_n7, assign52170_e86343_d_n8, assign52170_e86343_d_n9, assign52170_e86343_d_n10, assign52170_e86343_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard792 != 0.0)) && (locals.var_guard793 != 0.0)) {
        let assign52170_e86341: f64 = (1.0 - locals.var_t1);
        (assign52170_e86341, (-locals.var_t1_dn3), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn11),)
    } else {
        (locals.var_arg, locals.var_arg_dn3, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11,)
    }
};
        locals.var_arg = assign52170_e86343;
        locals.var_arg_dn3 = assign52170_e86343_d_n3;
        locals.var_arg_dn4 = assign52170_e86343_d_n4;
        locals.var_arg_dn5 = assign52170_e86343_d_n5;
        locals.var_arg_dn6 = assign52170_e86343_d_n6;
        locals.var_arg_dn7 = assign52170_e86343_d_n7;
        locals.var_arg_dn8 = assign52170_e86343_d_n8;
        locals.var_arg_dn9 = assign52170_e86343_d_n9;
        locals.var_arg_dn10 = assign52170_e86343_d_n10;
        locals.var_arg_dn11 = assign52170_e86343_d_n11;

        let assign52180_e86346: f64 = if p.p913 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard794 = assign52180_e86346;

        let assign52190_e86349: f64 = if p.p913 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard795 = assign52190_e86349;

        let (assign52200_e86365, assign52200_e86365_d_n3, assign52200_e86365_d_n4, assign52200_e86365_d_n5, assign52200_e86365_d_n6, assign52200_e86365_d_n7, assign52200_e86365_d_n8, assign52200_e86365_d_n9, assign52200_e86365_d_n10, assign52200_e86365_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard792 != 0.0)) && (locals.var_guard793 != 0.0)) && (locals.var_guard794 != 0.0)) && (locals.var_guard795 != 0.0)) {
        let assign52200_e86362: f64 = (locals.var_arg).sqrt();
        let assign52200_e86363: f64 = (1.0 / assign52200_e86362);
        (assign52200_e86363, (-((locals.var_arg_dn3 / (2.0 * assign52200_e86362)) / (assign52200_e86362 * assign52200_e86362))), (-((locals.var_arg_dn4 / (2.0 * assign52200_e86362)) / (assign52200_e86362 * assign52200_e86362))), (-((locals.var_arg_dn5 / (2.0 * assign52200_e86362)) / (assign52200_e86362 * assign52200_e86362))), (-((locals.var_arg_dn6 / (2.0 * assign52200_e86362)) / (assign52200_e86362 * assign52200_e86362))), (-((locals.var_arg_dn7 / (2.0 * assign52200_e86362)) / (assign52200_e86362 * assign52200_e86362))), (-((locals.var_arg_dn8 / (2.0 * assign52200_e86362)) / (assign52200_e86362 * assign52200_e86362))), (-((locals.var_arg_dn9 / (2.0 * assign52200_e86362)) / (assign52200_e86362 * assign52200_e86362))), (-((locals.var_arg_dn10 / (2.0 * assign52200_e86362)) / (assign52200_e86362 * assign52200_e86362))), (-((locals.var_arg_dn11 / (2.0 * assign52200_e86362)) / (assign52200_e86362 * assign52200_e86362))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11,)
    }
};
        locals.var_sarg = assign52200_e86365;
        locals.var_sarg_dn3 = assign52200_e86365_d_n3;
        locals.var_sarg_dn4 = assign52200_e86365_d_n4;
        locals.var_sarg_dn5 = assign52200_e86365_d_n5;
        locals.var_sarg_dn6 = assign52200_e86365_d_n6;
        locals.var_sarg_dn7 = assign52200_e86365_d_n7;
        locals.var_sarg_dn8 = assign52200_e86365_d_n8;
        locals.var_sarg_dn9 = assign52200_e86365_d_n9;
        locals.var_sarg_dn10 = assign52200_e86365_d_n10;
        locals.var_sarg_dn11 = assign52200_e86365_d_n11;

        let (assign52210_e86384, assign52210_e86384_d_n3, assign52210_e86384_d_n4, assign52210_e86384_d_n5, assign52210_e86384_d_n6, assign52210_e86384_d_n7, assign52210_e86384_d_n8, assign52210_e86384_d_n9, assign52210_e86384_d_n10, assign52210_e86384_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard792 != 0.0)) && (locals.var_guard793 != 0.0)) && (locals.var_guard794 != 0.0)) && (locals.var_guard795 == 0.0)) {
        let assign52210_e86378: f64 = (-p.p913);
        let assign52210_e86380: f64 = (locals.var_arg).ln();
        let assign52210_e86381: f64 = (assign52210_e86378 * assign52210_e86380);
        let assign52210_e86382: f64 = { let limited_exp_arg = assign52210_e86381; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign52210_e86382, ({ let limited_exp_arg = assign52210_e86381; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52210_e86378 * (locals.var_arg_dn3 / locals.var_arg))), ({ let limited_exp_arg = assign52210_e86381; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52210_e86378 * (locals.var_arg_dn4 / locals.var_arg))), ({ let limited_exp_arg = assign52210_e86381; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52210_e86378 * (locals.var_arg_dn5 / locals.var_arg))), ({ let limited_exp_arg = assign52210_e86381; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52210_e86378 * (locals.var_arg_dn6 / locals.var_arg))), ({ let limited_exp_arg = assign52210_e86381; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52210_e86378 * (locals.var_arg_dn7 / locals.var_arg))), ({ let limited_exp_arg = assign52210_e86381; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52210_e86378 * (locals.var_arg_dn8 / locals.var_arg))), ({ let limited_exp_arg = assign52210_e86381; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52210_e86378 * (locals.var_arg_dn9 / locals.var_arg))), ({ let limited_exp_arg = assign52210_e86381; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52210_e86378 * (locals.var_arg_dn10 / locals.var_arg))), ({ let limited_exp_arg = assign52210_e86381; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52210_e86378 * (locals.var_arg_dn11 / locals.var_arg))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11,)
    }
};
        locals.var_sarg = assign52210_e86384;
        locals.var_sarg_dn3 = assign52210_e86384_d_n3;
        locals.var_sarg_dn4 = assign52210_e86384_d_n4;
        locals.var_sarg_dn5 = assign52210_e86384_d_n5;
        locals.var_sarg_dn6 = assign52210_e86384_d_n6;
        locals.var_sarg_dn7 = assign52210_e86384_d_n7;
        locals.var_sarg_dn8 = assign52210_e86384_d_n8;
        locals.var_sarg_dn9 = assign52210_e86384_d_n9;
        locals.var_sarg_dn10 = assign52210_e86384_d_n10;
        locals.var_sarg_dn11 = assign52210_e86384_d_n11;

        let (assign52220_e86407, assign52220_e86407_d_n3, assign52220_e86407_d_n4, assign52220_e86407_d_n5, assign52220_e86407_d_n6, assign52220_e86407_d_n7, assign52220_e86407_d_n8, assign52220_e86407_d_n9, assign52220_e86407_d_n10, assign52220_e86407_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard792 != 0.0)) && (locals.var_guard793 != 0.0)) && (locals.var_guard794 != 0.0)) {
        let assign52220_e86395: f64 = (locals.var_pbs_t * locals.var_czbs);
        let assign52220_e86399: f64 = (locals.var_arg * locals.var_sarg);
        let assign52220_e86400: f64 = (1.0 - assign52220_e86399);
        let assign52220_e86401: f64 = (assign52220_e86395 * assign52220_e86400);
        let assign52220_e86404: f64 = (1.0 - p.p913);
        let assign52220_e86405: f64 = (assign52220_e86401 / assign52220_e86404);
        (assign52220_e86405, ((((locals.var_pbs_t * locals.var_czbs_dn3) * assign52220_e86400) + (assign52220_e86395 * (-((locals.var_arg_dn3 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn3))))) / assign52220_e86404), (((((locals.var_pbs_t_dn4 * locals.var_czbs) + (locals.var_pbs_t * locals.var_czbs_dn4)) * assign52220_e86400) + (assign52220_e86395 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign52220_e86404), (((((locals.var_pbs_t_dn5 * locals.var_czbs) + (locals.var_pbs_t * locals.var_czbs_dn5)) * assign52220_e86400) + (assign52220_e86395 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign52220_e86404), ((((locals.var_pbs_t * locals.var_czbs_dn6) * assign52220_e86400) + (assign52220_e86395 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign52220_e86404), ((((locals.var_pbs_t * locals.var_czbs_dn7) * assign52220_e86400) + (assign52220_e86395 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign52220_e86404), ((((locals.var_pbs_t * locals.var_czbs_dn8) * assign52220_e86400) + (assign52220_e86395 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign52220_e86404), ((((locals.var_pbs_t * locals.var_czbs_dn9) * assign52220_e86400) + (assign52220_e86395 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign52220_e86404), ((((locals.var_pbs_t * locals.var_czbs_dn10) * assign52220_e86400) + (assign52220_e86395 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign52220_e86404), ((((locals.var_pbs_t * locals.var_czbs_dn11) * assign52220_e86400) + (assign52220_e86395 * (-((locals.var_arg_dn11 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn11))))) / assign52220_e86404),)
    } else {
        (locals.var_qbsj1, locals.var_qbsj1_dn3, locals.var_qbsj1_dn4, locals.var_qbsj1_dn5, locals.var_qbsj1_dn6, locals.var_qbsj1_dn7, locals.var_qbsj1_dn8, locals.var_qbsj1_dn9, locals.var_qbsj1_dn10, locals.var_qbsj1_dn11,)
    }
};
        locals.var_qbsj1 = assign52220_e86407;
        locals.var_qbsj1_dn3 = assign52220_e86407_d_n3;
        locals.var_qbsj1_dn4 = assign52220_e86407_d_n4;
        locals.var_qbsj1_dn5 = assign52220_e86407_d_n5;
        locals.var_qbsj1_dn6 = assign52220_e86407_d_n6;
        locals.var_qbsj1_dn7 = assign52220_e86407_d_n7;
        locals.var_qbsj1_dn8 = assign52220_e86407_d_n8;
        locals.var_qbsj1_dn9 = assign52220_e86407_d_n9;
        locals.var_qbsj1_dn10 = assign52220_e86407_d_n10;
        locals.var_qbsj1_dn11 = assign52220_e86407_d_n11;

        let (assign52230_e86425, assign52230_e86425_d_n3, assign52230_e86425_d_n4, assign52230_e86425_d_n5, assign52230_e86425_d_n6, assign52230_e86425_d_n7, assign52230_e86425_d_n8, assign52230_e86425_d_n9, assign52230_e86425_d_n10, assign52230_e86425_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard792 != 0.0)) && (locals.var_guard793 != 0.0)) && (locals.var_guard794 == 0.0)) {
        let assign52230_e86419: f64 = (locals.var_pbs_t * locals.var_czbs);
        let assign52230_e86421: f64 = (locals.var_arg).ln();
        let assign52230_e86422: f64 = (-assign52230_e86421);
        let assign52230_e86423: f64 = (assign52230_e86419 * assign52230_e86422);
        (assign52230_e86423, (((locals.var_pbs_t * locals.var_czbs_dn3) * assign52230_e86422) + (assign52230_e86419 * (-(locals.var_arg_dn3 / locals.var_arg)))), ((((locals.var_pbs_t_dn4 * locals.var_czbs) + (locals.var_pbs_t * locals.var_czbs_dn4)) * assign52230_e86422) + (assign52230_e86419 * (-(locals.var_arg_dn4 / locals.var_arg)))), ((((locals.var_pbs_t_dn5 * locals.var_czbs) + (locals.var_pbs_t * locals.var_czbs_dn5)) * assign52230_e86422) + (assign52230_e86419 * (-(locals.var_arg_dn5 / locals.var_arg)))), (((locals.var_pbs_t * locals.var_czbs_dn6) * assign52230_e86422) + (assign52230_e86419 * (-(locals.var_arg_dn6 / locals.var_arg)))), (((locals.var_pbs_t * locals.var_czbs_dn7) * assign52230_e86422) + (assign52230_e86419 * (-(locals.var_arg_dn7 / locals.var_arg)))), (((locals.var_pbs_t * locals.var_czbs_dn8) * assign52230_e86422) + (assign52230_e86419 * (-(locals.var_arg_dn8 / locals.var_arg)))), (((locals.var_pbs_t * locals.var_czbs_dn9) * assign52230_e86422) + (assign52230_e86419 * (-(locals.var_arg_dn9 / locals.var_arg)))), (((locals.var_pbs_t * locals.var_czbs_dn10) * assign52230_e86422) + (assign52230_e86419 * (-(locals.var_arg_dn10 / locals.var_arg)))), (((locals.var_pbs_t * locals.var_czbs_dn11) * assign52230_e86422) + (assign52230_e86419 * (-(locals.var_arg_dn11 / locals.var_arg)))),)
    } else {
        (locals.var_qbsj1, locals.var_qbsj1_dn3, locals.var_qbsj1_dn4, locals.var_qbsj1_dn5, locals.var_qbsj1_dn6, locals.var_qbsj1_dn7, locals.var_qbsj1_dn8, locals.var_qbsj1_dn9, locals.var_qbsj1_dn10, locals.var_qbsj1_dn11,)
    }
};
        locals.var_qbsj1 = assign52230_e86425;
        locals.var_qbsj1_dn3 = assign52230_e86425_d_n3;
        locals.var_qbsj1_dn4 = assign52230_e86425_d_n4;
        locals.var_qbsj1_dn5 = assign52230_e86425_d_n5;
        locals.var_qbsj1_dn6 = assign52230_e86425_d_n6;
        locals.var_qbsj1_dn7 = assign52230_e86425_d_n7;
        locals.var_qbsj1_dn8 = assign52230_e86425_d_n8;
        locals.var_qbsj1_dn9 = assign52230_e86425_d_n9;
        locals.var_qbsj1_dn10 = assign52230_e86425_d_n10;
        locals.var_qbsj1_dn11 = assign52230_e86425_d_n11;

        let (assign52240_e86451, assign52240_e86451_d_n3, assign52240_e86451_d_n4, assign52240_e86451_d_n5, assign52240_e86451_d_n6, assign52240_e86451_d_n7, assign52240_e86451_d_n8, assign52240_e86451_d_n9, assign52240_e86451_d_n10, assign52240_e86451_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard792 != 0.0)) && (locals.var_guard793 == 0.0)) {
        let assign52240_e86436: f64 = (locals.var_t1 - 1.0);
        let assign52240_e86437: f64 = (locals.var_czbs_p1 * assign52240_e86436);
        let assign52240_e86440: f64 = (5.0 * p.p913);
        let assign52240_e86443: f64 = (locals.var_t1 - 1.0);
        let assign52240_e86444: f64 = (assign52240_e86440 * assign52240_e86443);
        let assign52240_e86447: f64 = (1.0 + p.p913);
        let assign52240_e86448: f64 = (assign52240_e86444 + assign52240_e86447);
        let assign52240_e86449: f64 = (assign52240_e86437 * assign52240_e86448);
        (assign52240_e86449, (((locals.var_czbs_p1 * locals.var_t1_dn3) * assign52240_e86448) + (assign52240_e86437 * (assign52240_e86440 * locals.var_t1_dn3))), (((locals.var_czbs_p1 * locals.var_t1_dn4) * assign52240_e86448) + (assign52240_e86437 * (assign52240_e86440 * locals.var_t1_dn4))), (((locals.var_czbs_p1 * locals.var_t1_dn5) * assign52240_e86448) + (assign52240_e86437 * (assign52240_e86440 * locals.var_t1_dn5))), (((locals.var_czbs_p1 * locals.var_t1_dn6) * assign52240_e86448) + (assign52240_e86437 * (assign52240_e86440 * locals.var_t1_dn6))), (((locals.var_czbs_p1 * locals.var_t1_dn7) * assign52240_e86448) + (assign52240_e86437 * (assign52240_e86440 * locals.var_t1_dn7))), (((locals.var_czbs_p1 * locals.var_t1_dn8) * assign52240_e86448) + (assign52240_e86437 * (assign52240_e86440 * locals.var_t1_dn8))), (((locals.var_czbs_p1 * locals.var_t1_dn9) * assign52240_e86448) + (assign52240_e86437 * (assign52240_e86440 * locals.var_t1_dn9))), (((locals.var_czbs_p1 * locals.var_t1_dn10) * assign52240_e86448) + (assign52240_e86437 * (assign52240_e86440 * locals.var_t1_dn10))), (((locals.var_czbs_p1 * locals.var_t1_dn11) * assign52240_e86448) + (assign52240_e86437 * (assign52240_e86440 * locals.var_t1_dn11))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign52240_e86451;
        locals.var_t2_dn3 = assign52240_e86451_d_n3;
        locals.var_t2_dn4 = assign52240_e86451_d_n4;
        locals.var_t2_dn5 = assign52240_e86451_d_n5;
        locals.var_t2_dn6 = assign52240_e86451_d_n6;
        locals.var_t2_dn7 = assign52240_e86451_d_n7;
        locals.var_t2_dn8 = assign52240_e86451_d_n8;
        locals.var_t2_dn9 = assign52240_e86451_d_n9;
        locals.var_t2_dn10 = assign52240_e86451_d_n10;
        locals.var_t2_dn11 = assign52240_e86451_d_n11;

        let (assign52250_e86467, assign52250_e86467_d_n3, assign52250_e86467_d_n4, assign52250_e86467_d_n5, assign52250_e86467_d_n6, assign52250_e86467_d_n7, assign52250_e86467_d_n8, assign52250_e86467_d_n9, assign52250_e86467_d_n10, assign52250_e86467_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard792 != 0.0)) && (locals.var_guard793 == 0.0)) {
        let assign52250_e86461: f64 = (locals.var_pbs_t * locals.var_czbs);
        let assign52250_e86464: f64 = (locals.var_t2 + locals.var_czbs_p2);
        let assign52250_e86465: f64 = (assign52250_e86461 * assign52250_e86464);
        (assign52250_e86465, (((locals.var_pbs_t * locals.var_czbs_dn3) * assign52250_e86464) + (assign52250_e86461 * locals.var_t2_dn3)), ((((locals.var_pbs_t_dn4 * locals.var_czbs) + (locals.var_pbs_t * locals.var_czbs_dn4)) * assign52250_e86464) + (assign52250_e86461 * locals.var_t2_dn4)), ((((locals.var_pbs_t_dn5 * locals.var_czbs) + (locals.var_pbs_t * locals.var_czbs_dn5)) * assign52250_e86464) + (assign52250_e86461 * locals.var_t2_dn5)), (((locals.var_pbs_t * locals.var_czbs_dn6) * assign52250_e86464) + (assign52250_e86461 * locals.var_t2_dn6)), (((locals.var_pbs_t * locals.var_czbs_dn7) * assign52250_e86464) + (assign52250_e86461 * locals.var_t2_dn7)), (((locals.var_pbs_t * locals.var_czbs_dn8) * assign52250_e86464) + (assign52250_e86461 * locals.var_t2_dn8)), (((locals.var_pbs_t * locals.var_czbs_dn9) * assign52250_e86464) + (assign52250_e86461 * locals.var_t2_dn9)), (((locals.var_pbs_t * locals.var_czbs_dn10) * assign52250_e86464) + (assign52250_e86461 * locals.var_t2_dn10)), (((locals.var_pbs_t * locals.var_czbs_dn11) * assign52250_e86464) + (assign52250_e86461 * locals.var_t2_dn11)),)
    } else {
        (locals.var_qbsj1, locals.var_qbsj1_dn3, locals.var_qbsj1_dn4, locals.var_qbsj1_dn5, locals.var_qbsj1_dn6, locals.var_qbsj1_dn7, locals.var_qbsj1_dn8, locals.var_qbsj1_dn9, locals.var_qbsj1_dn10, locals.var_qbsj1_dn11,)
    }
};
        locals.var_qbsj1 = assign52250_e86467;
        locals.var_qbsj1_dn3 = assign52250_e86467_d_n3;
        locals.var_qbsj1_dn4 = assign52250_e86467_d_n4;
        locals.var_qbsj1_dn5 = assign52250_e86467_d_n5;
        locals.var_qbsj1_dn6 = assign52250_e86467_d_n6;
        locals.var_qbsj1_dn7 = assign52250_e86467_d_n7;
        locals.var_qbsj1_dn8 = assign52250_e86467_d_n8;
        locals.var_qbsj1_dn9 = assign52250_e86467_d_n9;
        locals.var_qbsj1_dn10 = assign52250_e86467_d_n10;
        locals.var_qbsj1_dn11 = assign52250_e86467_d_n11;

        let (assign52260_e86475, assign52260_e86475_d_n3, assign52260_e86475_d_n4, assign52260_e86475_d_n5, assign52260_e86475_d_n6, assign52260_e86475_d_n7, assign52260_e86475_d_n8, assign52260_e86475_d_n9, assign52260_e86475_d_n10, assign52260_e86475_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard792 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbsj1, locals.var_qbsj1_dn3, locals.var_qbsj1_dn4, locals.var_qbsj1_dn5, locals.var_qbsj1_dn6, locals.var_qbsj1_dn7, locals.var_qbsj1_dn8, locals.var_qbsj1_dn9, locals.var_qbsj1_dn10, locals.var_qbsj1_dn11,)
    }
};
        locals.var_qbsj1 = assign52260_e86475;
        locals.var_qbsj1_dn3 = assign52260_e86475_d_n3;
        locals.var_qbsj1_dn4 = assign52260_e86475_d_n4;
        locals.var_qbsj1_dn5 = assign52260_e86475_d_n5;
        locals.var_qbsj1_dn6 = assign52260_e86475_d_n6;
        locals.var_qbsj1_dn7 = assign52260_e86475_d_n7;
        locals.var_qbsj1_dn8 = assign52260_e86475_d_n8;
        locals.var_qbsj1_dn9 = assign52260_e86475_d_n9;
        locals.var_qbsj1_dn10 = assign52260_e86475_d_n10;
        locals.var_qbsj1_dn11 = assign52260_e86475_d_n11;

        let assign52270_e86478: f64 = if locals.var_czbssw > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard796 = assign52270_e86478;

        let (assign52280_e86487, assign52280_e86487_d_n3, assign52280_e86487_d_n4, assign52280_e86487_d_n5, assign52280_e86487_d_n6, assign52280_e86487_d_n7, assign52280_e86487_d_n8, assign52280_e86487_d_n9, assign52280_e86487_d_n10, assign52280_e86487_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard796 != 0.0)) {
        let assign52280_e86485: f64 = (locals.var_vbs_jct / locals.var_pbsws_t);
        (assign52280_e86485, 0.0, (-((locals.var_vbs_jct * locals.var_pbsws_t_dn4) / (locals.var_pbsws_t * locals.var_pbsws_t))), (-((locals.var_vbs_jct * locals.var_pbsws_t_dn5) / (locals.var_pbsws_t * locals.var_pbsws_t))), 0.0, (locals.var_vbs_jct_dn7 / locals.var_pbsws_t), 0.0, 0.0, (locals.var_vbs_jct_dn10 / locals.var_pbsws_t), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign52280_e86487;
        locals.var_t1_dn3 = assign52280_e86487_d_n3;
        locals.var_t1_dn4 = assign52280_e86487_d_n4;
        locals.var_t1_dn5 = assign52280_e86487_d_n5;
        locals.var_t1_dn6 = assign52280_e86487_d_n6;
        locals.var_t1_dn7 = assign52280_e86487_d_n7;
        locals.var_t1_dn8 = assign52280_e86487_d_n8;
        locals.var_t1_dn9 = assign52280_e86487_d_n9;
        locals.var_t1_dn10 = assign52280_e86487_d_n10;
        locals.var_t1_dn11 = assign52280_e86487_d_n11;

        let assign52290_e86490: f64 = if locals.var_t1 < 0.9 { 1.0 } else { 0.0 };
        locals.var_guard797 = assign52290_e86490;

        let (assign52300_e86501, assign52300_e86501_d_n3, assign52300_e86501_d_n4, assign52300_e86501_d_n5, assign52300_e86501_d_n6, assign52300_e86501_d_n7, assign52300_e86501_d_n8, assign52300_e86501_d_n9, assign52300_e86501_d_n10, assign52300_e86501_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard796 != 0.0)) && (locals.var_guard797 != 0.0)) {
        let assign52300_e86499: f64 = (1.0 - locals.var_t1);
        (assign52300_e86499, (-locals.var_t1_dn3), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn11),)
    } else {
        (locals.var_arg, locals.var_arg_dn3, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11,)
    }
};
        locals.var_arg = assign52300_e86501;
        locals.var_arg_dn3 = assign52300_e86501_d_n3;
        locals.var_arg_dn4 = assign52300_e86501_d_n4;
        locals.var_arg_dn5 = assign52300_e86501_d_n5;
        locals.var_arg_dn6 = assign52300_e86501_d_n6;
        locals.var_arg_dn7 = assign52300_e86501_d_n7;
        locals.var_arg_dn8 = assign52300_e86501_d_n8;
        locals.var_arg_dn9 = assign52300_e86501_d_n9;
        locals.var_arg_dn10 = assign52300_e86501_d_n10;
        locals.var_arg_dn11 = assign52300_e86501_d_n11;

        let assign52310_e86504: f64 = if p.p915 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard798 = assign52310_e86504;

        let assign52320_e86507: f64 = if p.p915 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard799 = assign52320_e86507;

        let (assign52330_e86523, assign52330_e86523_d_n3, assign52330_e86523_d_n4, assign52330_e86523_d_n5, assign52330_e86523_d_n6, assign52330_e86523_d_n7, assign52330_e86523_d_n8, assign52330_e86523_d_n9, assign52330_e86523_d_n10, assign52330_e86523_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard796 != 0.0)) && (locals.var_guard797 != 0.0)) && (locals.var_guard798 != 0.0)) && (locals.var_guard799 != 0.0)) {
        let assign52330_e86520: f64 = (locals.var_arg).sqrt();
        let assign52330_e86521: f64 = (1.0 / assign52330_e86520);
        (assign52330_e86521, (-((locals.var_arg_dn3 / (2.0 * assign52330_e86520)) / (assign52330_e86520 * assign52330_e86520))), (-((locals.var_arg_dn4 / (2.0 * assign52330_e86520)) / (assign52330_e86520 * assign52330_e86520))), (-((locals.var_arg_dn5 / (2.0 * assign52330_e86520)) / (assign52330_e86520 * assign52330_e86520))), (-((locals.var_arg_dn6 / (2.0 * assign52330_e86520)) / (assign52330_e86520 * assign52330_e86520))), (-((locals.var_arg_dn7 / (2.0 * assign52330_e86520)) / (assign52330_e86520 * assign52330_e86520))), (-((locals.var_arg_dn8 / (2.0 * assign52330_e86520)) / (assign52330_e86520 * assign52330_e86520))), (-((locals.var_arg_dn9 / (2.0 * assign52330_e86520)) / (assign52330_e86520 * assign52330_e86520))), (-((locals.var_arg_dn10 / (2.0 * assign52330_e86520)) / (assign52330_e86520 * assign52330_e86520))), (-((locals.var_arg_dn11 / (2.0 * assign52330_e86520)) / (assign52330_e86520 * assign52330_e86520))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11,)
    }
};
        locals.var_sarg = assign52330_e86523;
        locals.var_sarg_dn3 = assign52330_e86523_d_n3;
        locals.var_sarg_dn4 = assign52330_e86523_d_n4;
        locals.var_sarg_dn5 = assign52330_e86523_d_n5;
        locals.var_sarg_dn6 = assign52330_e86523_d_n6;
        locals.var_sarg_dn7 = assign52330_e86523_d_n7;
        locals.var_sarg_dn8 = assign52330_e86523_d_n8;
        locals.var_sarg_dn9 = assign52330_e86523_d_n9;
        locals.var_sarg_dn10 = assign52330_e86523_d_n10;
        locals.var_sarg_dn11 = assign52330_e86523_d_n11;

        let (assign52340_e86542, assign52340_e86542_d_n3, assign52340_e86542_d_n4, assign52340_e86542_d_n5, assign52340_e86542_d_n6, assign52340_e86542_d_n7, assign52340_e86542_d_n8, assign52340_e86542_d_n9, assign52340_e86542_d_n10, assign52340_e86542_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard796 != 0.0)) && (locals.var_guard797 != 0.0)) && (locals.var_guard798 != 0.0)) && (locals.var_guard799 == 0.0)) {
        let assign52340_e86536: f64 = (-p.p915);
        let assign52340_e86538: f64 = (locals.var_arg).ln();
        let assign52340_e86539: f64 = (assign52340_e86536 * assign52340_e86538);
        let assign52340_e86540: f64 = { let limited_exp_arg = assign52340_e86539; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign52340_e86540, ({ let limited_exp_arg = assign52340_e86539; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52340_e86536 * (locals.var_arg_dn3 / locals.var_arg))), ({ let limited_exp_arg = assign52340_e86539; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52340_e86536 * (locals.var_arg_dn4 / locals.var_arg))), ({ let limited_exp_arg = assign52340_e86539; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52340_e86536 * (locals.var_arg_dn5 / locals.var_arg))), ({ let limited_exp_arg = assign52340_e86539; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52340_e86536 * (locals.var_arg_dn6 / locals.var_arg))), ({ let limited_exp_arg = assign52340_e86539; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52340_e86536 * (locals.var_arg_dn7 / locals.var_arg))), ({ let limited_exp_arg = assign52340_e86539; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52340_e86536 * (locals.var_arg_dn8 / locals.var_arg))), ({ let limited_exp_arg = assign52340_e86539; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52340_e86536 * (locals.var_arg_dn9 / locals.var_arg))), ({ let limited_exp_arg = assign52340_e86539; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52340_e86536 * (locals.var_arg_dn10 / locals.var_arg))), ({ let limited_exp_arg = assign52340_e86539; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52340_e86536 * (locals.var_arg_dn11 / locals.var_arg))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11,)
    }
};
        locals.var_sarg = assign52340_e86542;
        locals.var_sarg_dn3 = assign52340_e86542_d_n3;
        locals.var_sarg_dn4 = assign52340_e86542_d_n4;
        locals.var_sarg_dn5 = assign52340_e86542_d_n5;
        locals.var_sarg_dn6 = assign52340_e86542_d_n6;
        locals.var_sarg_dn7 = assign52340_e86542_d_n7;
        locals.var_sarg_dn8 = assign52340_e86542_d_n8;
        locals.var_sarg_dn9 = assign52340_e86542_d_n9;
        locals.var_sarg_dn10 = assign52340_e86542_d_n10;
        locals.var_sarg_dn11 = assign52340_e86542_d_n11;

        let (assign52350_e86565, assign52350_e86565_d_n3, assign52350_e86565_d_n4, assign52350_e86565_d_n5, assign52350_e86565_d_n6, assign52350_e86565_d_n7, assign52350_e86565_d_n8, assign52350_e86565_d_n9, assign52350_e86565_d_n10, assign52350_e86565_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard796 != 0.0)) && (locals.var_guard797 != 0.0)) && (locals.var_guard798 != 0.0)) {
        let assign52350_e86553: f64 = (locals.var_pbsws_t * locals.var_czbssw);
        let assign52350_e86557: f64 = (locals.var_arg * locals.var_sarg);
        let assign52350_e86558: f64 = (1.0 - assign52350_e86557);
        let assign52350_e86559: f64 = (assign52350_e86553 * assign52350_e86558);
        let assign52350_e86562: f64 = (1.0 - p.p915);
        let assign52350_e86563: f64 = (assign52350_e86559 / assign52350_e86562);
        (assign52350_e86563, ((((locals.var_pbsws_t * locals.var_czbssw_dn3) * assign52350_e86558) + (assign52350_e86553 * (-((locals.var_arg_dn3 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn3))))) / assign52350_e86562), (((((locals.var_pbsws_t_dn4 * locals.var_czbssw) + (locals.var_pbsws_t * locals.var_czbssw_dn4)) * assign52350_e86558) + (assign52350_e86553 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign52350_e86562), (((((locals.var_pbsws_t_dn5 * locals.var_czbssw) + (locals.var_pbsws_t * locals.var_czbssw_dn5)) * assign52350_e86558) + (assign52350_e86553 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign52350_e86562), ((((locals.var_pbsws_t * locals.var_czbssw_dn6) * assign52350_e86558) + (assign52350_e86553 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign52350_e86562), ((((locals.var_pbsws_t * locals.var_czbssw_dn7) * assign52350_e86558) + (assign52350_e86553 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign52350_e86562), ((((locals.var_pbsws_t * locals.var_czbssw_dn8) * assign52350_e86558) + (assign52350_e86553 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign52350_e86562), ((((locals.var_pbsws_t * locals.var_czbssw_dn9) * assign52350_e86558) + (assign52350_e86553 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign52350_e86562), ((((locals.var_pbsws_t * locals.var_czbssw_dn10) * assign52350_e86558) + (assign52350_e86553 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign52350_e86562), ((((locals.var_pbsws_t * locals.var_czbssw_dn11) * assign52350_e86558) + (assign52350_e86553 * (-((locals.var_arg_dn11 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn11))))) / assign52350_e86562),)
    } else {
        (locals.var_qbsj2, locals.var_qbsj2_dn3, locals.var_qbsj2_dn4, locals.var_qbsj2_dn5, locals.var_qbsj2_dn6, locals.var_qbsj2_dn7, locals.var_qbsj2_dn8, locals.var_qbsj2_dn9, locals.var_qbsj2_dn10, locals.var_qbsj2_dn11,)
    }
};
        locals.var_qbsj2 = assign52350_e86565;
        locals.var_qbsj2_dn3 = assign52350_e86565_d_n3;
        locals.var_qbsj2_dn4 = assign52350_e86565_d_n4;
        locals.var_qbsj2_dn5 = assign52350_e86565_d_n5;
        locals.var_qbsj2_dn6 = assign52350_e86565_d_n6;
        locals.var_qbsj2_dn7 = assign52350_e86565_d_n7;
        locals.var_qbsj2_dn8 = assign52350_e86565_d_n8;
        locals.var_qbsj2_dn9 = assign52350_e86565_d_n9;
        locals.var_qbsj2_dn10 = assign52350_e86565_d_n10;
        locals.var_qbsj2_dn11 = assign52350_e86565_d_n11;

        let (assign52360_e86583, assign52360_e86583_d_n3, assign52360_e86583_d_n4, assign52360_e86583_d_n5, assign52360_e86583_d_n6, assign52360_e86583_d_n7, assign52360_e86583_d_n8, assign52360_e86583_d_n9, assign52360_e86583_d_n10, assign52360_e86583_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard796 != 0.0)) && (locals.var_guard797 != 0.0)) && (locals.var_guard798 == 0.0)) {
        let assign52360_e86577: f64 = (locals.var_pbsws_t * locals.var_czbssw);
        let assign52360_e86579: f64 = (locals.var_arg).ln();
        let assign52360_e86580: f64 = (-assign52360_e86579);
        let assign52360_e86581: f64 = (assign52360_e86577 * assign52360_e86580);
        (assign52360_e86581, (((locals.var_pbsws_t * locals.var_czbssw_dn3) * assign52360_e86580) + (assign52360_e86577 * (-(locals.var_arg_dn3 / locals.var_arg)))), ((((locals.var_pbsws_t_dn4 * locals.var_czbssw) + (locals.var_pbsws_t * locals.var_czbssw_dn4)) * assign52360_e86580) + (assign52360_e86577 * (-(locals.var_arg_dn4 / locals.var_arg)))), ((((locals.var_pbsws_t_dn5 * locals.var_czbssw) + (locals.var_pbsws_t * locals.var_czbssw_dn5)) * assign52360_e86580) + (assign52360_e86577 * (-(locals.var_arg_dn5 / locals.var_arg)))), (((locals.var_pbsws_t * locals.var_czbssw_dn6) * assign52360_e86580) + (assign52360_e86577 * (-(locals.var_arg_dn6 / locals.var_arg)))), (((locals.var_pbsws_t * locals.var_czbssw_dn7) * assign52360_e86580) + (assign52360_e86577 * (-(locals.var_arg_dn7 / locals.var_arg)))), (((locals.var_pbsws_t * locals.var_czbssw_dn8) * assign52360_e86580) + (assign52360_e86577 * (-(locals.var_arg_dn8 / locals.var_arg)))), (((locals.var_pbsws_t * locals.var_czbssw_dn9) * assign52360_e86580) + (assign52360_e86577 * (-(locals.var_arg_dn9 / locals.var_arg)))), (((locals.var_pbsws_t * locals.var_czbssw_dn10) * assign52360_e86580) + (assign52360_e86577 * (-(locals.var_arg_dn10 / locals.var_arg)))), (((locals.var_pbsws_t * locals.var_czbssw_dn11) * assign52360_e86580) + (assign52360_e86577 * (-(locals.var_arg_dn11 / locals.var_arg)))),)
    } else {
        (locals.var_qbsj2, locals.var_qbsj2_dn3, locals.var_qbsj2_dn4, locals.var_qbsj2_dn5, locals.var_qbsj2_dn6, locals.var_qbsj2_dn7, locals.var_qbsj2_dn8, locals.var_qbsj2_dn9, locals.var_qbsj2_dn10, locals.var_qbsj2_dn11,)
    }
};
        locals.var_qbsj2 = assign52360_e86583;
        locals.var_qbsj2_dn3 = assign52360_e86583_d_n3;
        locals.var_qbsj2_dn4 = assign52360_e86583_d_n4;
        locals.var_qbsj2_dn5 = assign52360_e86583_d_n5;
        locals.var_qbsj2_dn6 = assign52360_e86583_d_n6;
        locals.var_qbsj2_dn7 = assign52360_e86583_d_n7;
        locals.var_qbsj2_dn8 = assign52360_e86583_d_n8;
        locals.var_qbsj2_dn9 = assign52360_e86583_d_n9;
        locals.var_qbsj2_dn10 = assign52360_e86583_d_n10;
        locals.var_qbsj2_dn11 = assign52360_e86583_d_n11;

        let (assign52370_e86609, assign52370_e86609_d_n3, assign52370_e86609_d_n4, assign52370_e86609_d_n5, assign52370_e86609_d_n6, assign52370_e86609_d_n7, assign52370_e86609_d_n8, assign52370_e86609_d_n9, assign52370_e86609_d_n10, assign52370_e86609_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard796 != 0.0)) && (locals.var_guard797 == 0.0)) {
        let assign52370_e86594: f64 = (locals.var_t1 - 1.0);
        let assign52370_e86595: f64 = (locals.var_czbssw_p1 * assign52370_e86594);
        let assign52370_e86598: f64 = (5.0 * p.p915);
        let assign52370_e86601: f64 = (locals.var_t1 - 1.0);
        let assign52370_e86602: f64 = (assign52370_e86598 * assign52370_e86601);
        let assign52370_e86605: f64 = (1.0 + p.p915);
        let assign52370_e86606: f64 = (assign52370_e86602 + assign52370_e86605);
        let assign52370_e86607: f64 = (assign52370_e86595 * assign52370_e86606);
        (assign52370_e86607, (((locals.var_czbssw_p1 * locals.var_t1_dn3) * assign52370_e86606) + (assign52370_e86595 * (assign52370_e86598 * locals.var_t1_dn3))), (((locals.var_czbssw_p1 * locals.var_t1_dn4) * assign52370_e86606) + (assign52370_e86595 * (assign52370_e86598 * locals.var_t1_dn4))), (((locals.var_czbssw_p1 * locals.var_t1_dn5) * assign52370_e86606) + (assign52370_e86595 * (assign52370_e86598 * locals.var_t1_dn5))), (((locals.var_czbssw_p1 * locals.var_t1_dn6) * assign52370_e86606) + (assign52370_e86595 * (assign52370_e86598 * locals.var_t1_dn6))), (((locals.var_czbssw_p1 * locals.var_t1_dn7) * assign52370_e86606) + (assign52370_e86595 * (assign52370_e86598 * locals.var_t1_dn7))), (((locals.var_czbssw_p1 * locals.var_t1_dn8) * assign52370_e86606) + (assign52370_e86595 * (assign52370_e86598 * locals.var_t1_dn8))), (((locals.var_czbssw_p1 * locals.var_t1_dn9) * assign52370_e86606) + (assign52370_e86595 * (assign52370_e86598 * locals.var_t1_dn9))), (((locals.var_czbssw_p1 * locals.var_t1_dn10) * assign52370_e86606) + (assign52370_e86595 * (assign52370_e86598 * locals.var_t1_dn10))), (((locals.var_czbssw_p1 * locals.var_t1_dn11) * assign52370_e86606) + (assign52370_e86595 * (assign52370_e86598 * locals.var_t1_dn11))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign52370_e86609;
        locals.var_t2_dn3 = assign52370_e86609_d_n3;
        locals.var_t2_dn4 = assign52370_e86609_d_n4;
        locals.var_t2_dn5 = assign52370_e86609_d_n5;
        locals.var_t2_dn6 = assign52370_e86609_d_n6;
        locals.var_t2_dn7 = assign52370_e86609_d_n7;
        locals.var_t2_dn8 = assign52370_e86609_d_n8;
        locals.var_t2_dn9 = assign52370_e86609_d_n9;
        locals.var_t2_dn10 = assign52370_e86609_d_n10;
        locals.var_t2_dn11 = assign52370_e86609_d_n11;

        let (assign52380_e86625, assign52380_e86625_d_n3, assign52380_e86625_d_n4, assign52380_e86625_d_n5, assign52380_e86625_d_n6, assign52380_e86625_d_n7, assign52380_e86625_d_n8, assign52380_e86625_d_n9, assign52380_e86625_d_n10, assign52380_e86625_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard796 != 0.0)) && (locals.var_guard797 == 0.0)) {
        let assign52380_e86619: f64 = (locals.var_pbsws_t * locals.var_czbssw);
        let assign52380_e86622: f64 = (locals.var_t2 + locals.var_czbssw_p2);
        let assign52380_e86623: f64 = (assign52380_e86619 * assign52380_e86622);
        (assign52380_e86623, (((locals.var_pbsws_t * locals.var_czbssw_dn3) * assign52380_e86622) + (assign52380_e86619 * locals.var_t2_dn3)), ((((locals.var_pbsws_t_dn4 * locals.var_czbssw) + (locals.var_pbsws_t * locals.var_czbssw_dn4)) * assign52380_e86622) + (assign52380_e86619 * locals.var_t2_dn4)), ((((locals.var_pbsws_t_dn5 * locals.var_czbssw) + (locals.var_pbsws_t * locals.var_czbssw_dn5)) * assign52380_e86622) + (assign52380_e86619 * locals.var_t2_dn5)), (((locals.var_pbsws_t * locals.var_czbssw_dn6) * assign52380_e86622) + (assign52380_e86619 * locals.var_t2_dn6)), (((locals.var_pbsws_t * locals.var_czbssw_dn7) * assign52380_e86622) + (assign52380_e86619 * locals.var_t2_dn7)), (((locals.var_pbsws_t * locals.var_czbssw_dn8) * assign52380_e86622) + (assign52380_e86619 * locals.var_t2_dn8)), (((locals.var_pbsws_t * locals.var_czbssw_dn9) * assign52380_e86622) + (assign52380_e86619 * locals.var_t2_dn9)), (((locals.var_pbsws_t * locals.var_czbssw_dn10) * assign52380_e86622) + (assign52380_e86619 * locals.var_t2_dn10)), (((locals.var_pbsws_t * locals.var_czbssw_dn11) * assign52380_e86622) + (assign52380_e86619 * locals.var_t2_dn11)),)
    } else {
        (locals.var_qbsj2, locals.var_qbsj2_dn3, locals.var_qbsj2_dn4, locals.var_qbsj2_dn5, locals.var_qbsj2_dn6, locals.var_qbsj2_dn7, locals.var_qbsj2_dn8, locals.var_qbsj2_dn9, locals.var_qbsj2_dn10, locals.var_qbsj2_dn11,)
    }
};
        locals.var_qbsj2 = assign52380_e86625;
        locals.var_qbsj2_dn3 = assign52380_e86625_d_n3;
        locals.var_qbsj2_dn4 = assign52380_e86625_d_n4;
        locals.var_qbsj2_dn5 = assign52380_e86625_d_n5;
        locals.var_qbsj2_dn6 = assign52380_e86625_d_n6;
        locals.var_qbsj2_dn7 = assign52380_e86625_d_n7;
        locals.var_qbsj2_dn8 = assign52380_e86625_d_n8;
        locals.var_qbsj2_dn9 = assign52380_e86625_d_n9;
        locals.var_qbsj2_dn10 = assign52380_e86625_d_n10;
        locals.var_qbsj2_dn11 = assign52380_e86625_d_n11;

        let (assign52390_e86633, assign52390_e86633_d_n3, assign52390_e86633_d_n4, assign52390_e86633_d_n5, assign52390_e86633_d_n6, assign52390_e86633_d_n7, assign52390_e86633_d_n8, assign52390_e86633_d_n9, assign52390_e86633_d_n10, assign52390_e86633_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard796 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbsj2, locals.var_qbsj2_dn3, locals.var_qbsj2_dn4, locals.var_qbsj2_dn5, locals.var_qbsj2_dn6, locals.var_qbsj2_dn7, locals.var_qbsj2_dn8, locals.var_qbsj2_dn9, locals.var_qbsj2_dn10, locals.var_qbsj2_dn11,)
    }
};
        locals.var_qbsj2 = assign52390_e86633;
        locals.var_qbsj2_dn3 = assign52390_e86633_d_n3;
        locals.var_qbsj2_dn4 = assign52390_e86633_d_n4;
        locals.var_qbsj2_dn5 = assign52390_e86633_d_n5;
        locals.var_qbsj2_dn6 = assign52390_e86633_d_n6;
        locals.var_qbsj2_dn7 = assign52390_e86633_d_n7;
        locals.var_qbsj2_dn8 = assign52390_e86633_d_n8;
        locals.var_qbsj2_dn9 = assign52390_e86633_d_n9;
        locals.var_qbsj2_dn10 = assign52390_e86633_d_n10;
        locals.var_qbsj2_dn11 = assign52390_e86633_d_n11;

        let assign52400_e86636: f64 = if locals.var_czbsswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard800 = assign52400_e86636;

        let (assign52410_e86645, assign52410_e86645_d_n3, assign52410_e86645_d_n4, assign52410_e86645_d_n5, assign52410_e86645_d_n6, assign52410_e86645_d_n7, assign52410_e86645_d_n8, assign52410_e86645_d_n9, assign52410_e86645_d_n10, assign52410_e86645_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard800 != 0.0)) {
        let assign52410_e86643: f64 = (locals.var_vbs_jct / locals.var_pbswgs_t);
        (assign52410_e86643, 0.0, (-((locals.var_vbs_jct * locals.var_pbswgs_t_dn4) / (locals.var_pbswgs_t * locals.var_pbswgs_t))), (-((locals.var_vbs_jct * locals.var_pbswgs_t_dn5) / (locals.var_pbswgs_t * locals.var_pbswgs_t))), 0.0, (locals.var_vbs_jct_dn7 / locals.var_pbswgs_t), 0.0, 0.0, (locals.var_vbs_jct_dn10 / locals.var_pbswgs_t), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign52410_e86645;
        locals.var_t1_dn3 = assign52410_e86645_d_n3;
        locals.var_t1_dn4 = assign52410_e86645_d_n4;
        locals.var_t1_dn5 = assign52410_e86645_d_n5;
        locals.var_t1_dn6 = assign52410_e86645_d_n6;
        locals.var_t1_dn7 = assign52410_e86645_d_n7;
        locals.var_t1_dn8 = assign52410_e86645_d_n8;
        locals.var_t1_dn9 = assign52410_e86645_d_n9;
        locals.var_t1_dn10 = assign52410_e86645_d_n10;
        locals.var_t1_dn11 = assign52410_e86645_d_n11;

        let assign52420_e86648: f64 = if locals.var_t1 < 0.9 { 1.0 } else { 0.0 };
        locals.var_guard801 = assign52420_e86648;

        let (assign52430_e86659, assign52430_e86659_d_n3, assign52430_e86659_d_n4, assign52430_e86659_d_n5, assign52430_e86659_d_n6, assign52430_e86659_d_n7, assign52430_e86659_d_n8, assign52430_e86659_d_n9, assign52430_e86659_d_n10, assign52430_e86659_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard800 != 0.0)) && (locals.var_guard801 != 0.0)) {
        let assign52430_e86657: f64 = (1.0 - locals.var_t1);
        (assign52430_e86657, (-locals.var_t1_dn3), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn11),)
    } else {
        (locals.var_arg, locals.var_arg_dn3, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11,)
    }
};
        locals.var_arg = assign52430_e86659;
        locals.var_arg_dn3 = assign52430_e86659_d_n3;
        locals.var_arg_dn4 = assign52430_e86659_d_n4;
        locals.var_arg_dn5 = assign52430_e86659_d_n5;
        locals.var_arg_dn6 = assign52430_e86659_d_n6;
        locals.var_arg_dn7 = assign52430_e86659_d_n7;
        locals.var_arg_dn8 = assign52430_e86659_d_n8;
        locals.var_arg_dn9 = assign52430_e86659_d_n9;
        locals.var_arg_dn10 = assign52430_e86659_d_n10;
        locals.var_arg_dn11 = assign52430_e86659_d_n11;

        let assign52440_e86662: f64 = if p.p917 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard802 = assign52440_e86662;

    }

    pub(super) fn stamp_transient_block_178(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign52450_e86665: f64 = if p.p917 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard803 = assign52450_e86665;

        let (assign52460_e86681, assign52460_e86681_d_n3, assign52460_e86681_d_n4, assign52460_e86681_d_n5, assign52460_e86681_d_n6, assign52460_e86681_d_n7, assign52460_e86681_d_n8, assign52460_e86681_d_n9, assign52460_e86681_d_n10, assign52460_e86681_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard800 != 0.0)) && (locals.var_guard801 != 0.0)) && (locals.var_guard802 != 0.0)) && (locals.var_guard803 != 0.0)) {
        let assign52460_e86678: f64 = (locals.var_arg).sqrt();
        let assign52460_e86679: f64 = (1.0 / assign52460_e86678);
        (assign52460_e86679, (-((locals.var_arg_dn3 / (2.0 * assign52460_e86678)) / (assign52460_e86678 * assign52460_e86678))), (-((locals.var_arg_dn4 / (2.0 * assign52460_e86678)) / (assign52460_e86678 * assign52460_e86678))), (-((locals.var_arg_dn5 / (2.0 * assign52460_e86678)) / (assign52460_e86678 * assign52460_e86678))), (-((locals.var_arg_dn6 / (2.0 * assign52460_e86678)) / (assign52460_e86678 * assign52460_e86678))), (-((locals.var_arg_dn7 / (2.0 * assign52460_e86678)) / (assign52460_e86678 * assign52460_e86678))), (-((locals.var_arg_dn8 / (2.0 * assign52460_e86678)) / (assign52460_e86678 * assign52460_e86678))), (-((locals.var_arg_dn9 / (2.0 * assign52460_e86678)) / (assign52460_e86678 * assign52460_e86678))), (-((locals.var_arg_dn10 / (2.0 * assign52460_e86678)) / (assign52460_e86678 * assign52460_e86678))), (-((locals.var_arg_dn11 / (2.0 * assign52460_e86678)) / (assign52460_e86678 * assign52460_e86678))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11,)
    }
};
        locals.var_sarg = assign52460_e86681;
        locals.var_sarg_dn3 = assign52460_e86681_d_n3;
        locals.var_sarg_dn4 = assign52460_e86681_d_n4;
        locals.var_sarg_dn5 = assign52460_e86681_d_n5;
        locals.var_sarg_dn6 = assign52460_e86681_d_n6;
        locals.var_sarg_dn7 = assign52460_e86681_d_n7;
        locals.var_sarg_dn8 = assign52460_e86681_d_n8;
        locals.var_sarg_dn9 = assign52460_e86681_d_n9;
        locals.var_sarg_dn10 = assign52460_e86681_d_n10;
        locals.var_sarg_dn11 = assign52460_e86681_d_n11;

        let (assign52470_e86700, assign52470_e86700_d_n3, assign52470_e86700_d_n4, assign52470_e86700_d_n5, assign52470_e86700_d_n6, assign52470_e86700_d_n7, assign52470_e86700_d_n8, assign52470_e86700_d_n9, assign52470_e86700_d_n10, assign52470_e86700_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard800 != 0.0)) && (locals.var_guard801 != 0.0)) && (locals.var_guard802 != 0.0)) && (locals.var_guard803 == 0.0)) {
        let assign52470_e86694: f64 = (-p.p917);
        let assign52470_e86696: f64 = (locals.var_arg).ln();
        let assign52470_e86697: f64 = (assign52470_e86694 * assign52470_e86696);
        let assign52470_e86698: f64 = { let limited_exp_arg = assign52470_e86697; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign52470_e86698, ({ let limited_exp_arg = assign52470_e86697; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52470_e86694 * (locals.var_arg_dn3 / locals.var_arg))), ({ let limited_exp_arg = assign52470_e86697; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52470_e86694 * (locals.var_arg_dn4 / locals.var_arg))), ({ let limited_exp_arg = assign52470_e86697; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52470_e86694 * (locals.var_arg_dn5 / locals.var_arg))), ({ let limited_exp_arg = assign52470_e86697; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52470_e86694 * (locals.var_arg_dn6 / locals.var_arg))), ({ let limited_exp_arg = assign52470_e86697; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52470_e86694 * (locals.var_arg_dn7 / locals.var_arg))), ({ let limited_exp_arg = assign52470_e86697; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52470_e86694 * (locals.var_arg_dn8 / locals.var_arg))), ({ let limited_exp_arg = assign52470_e86697; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52470_e86694 * (locals.var_arg_dn9 / locals.var_arg))), ({ let limited_exp_arg = assign52470_e86697; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52470_e86694 * (locals.var_arg_dn10 / locals.var_arg))), ({ let limited_exp_arg = assign52470_e86697; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52470_e86694 * (locals.var_arg_dn11 / locals.var_arg))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11,)
    }
};
        locals.var_sarg = assign52470_e86700;
        locals.var_sarg_dn3 = assign52470_e86700_d_n3;
        locals.var_sarg_dn4 = assign52470_e86700_d_n4;
        locals.var_sarg_dn5 = assign52470_e86700_d_n5;
        locals.var_sarg_dn6 = assign52470_e86700_d_n6;
        locals.var_sarg_dn7 = assign52470_e86700_d_n7;
        locals.var_sarg_dn8 = assign52470_e86700_d_n8;
        locals.var_sarg_dn9 = assign52470_e86700_d_n9;
        locals.var_sarg_dn10 = assign52470_e86700_d_n10;
        locals.var_sarg_dn11 = assign52470_e86700_d_n11;

        let (assign52480_e86723, assign52480_e86723_d_n3, assign52480_e86723_d_n4, assign52480_e86723_d_n5, assign52480_e86723_d_n6, assign52480_e86723_d_n7, assign52480_e86723_d_n8, assign52480_e86723_d_n9, assign52480_e86723_d_n10, assign52480_e86723_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard800 != 0.0)) && (locals.var_guard801 != 0.0)) && (locals.var_guard802 != 0.0)) {
        let assign52480_e86711: f64 = (locals.var_pbswgs_t * locals.var_czbsswg);
        let assign52480_e86715: f64 = (locals.var_arg * locals.var_sarg);
        let assign52480_e86716: f64 = (1.0 - assign52480_e86715);
        let assign52480_e86717: f64 = (assign52480_e86711 * assign52480_e86716);
        let assign52480_e86720: f64 = (1.0 - p.p917);
        let assign52480_e86721: f64 = (assign52480_e86717 / assign52480_e86720);
        (assign52480_e86721, ((assign52480_e86711 * (-((locals.var_arg_dn3 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn3)))) / assign52480_e86720), (((((locals.var_pbswgs_t_dn4 * locals.var_czbsswg) + (locals.var_pbswgs_t * locals.var_czbsswg_dn4)) * assign52480_e86716) + (assign52480_e86711 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign52480_e86720), (((((locals.var_pbswgs_t_dn5 * locals.var_czbsswg) + (locals.var_pbswgs_t * locals.var_czbsswg_dn5)) * assign52480_e86716) + (assign52480_e86711 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign52480_e86720), ((assign52480_e86711 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6)))) / assign52480_e86720), ((assign52480_e86711 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7)))) / assign52480_e86720), ((assign52480_e86711 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8)))) / assign52480_e86720), ((assign52480_e86711 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9)))) / assign52480_e86720), ((assign52480_e86711 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10)))) / assign52480_e86720), ((assign52480_e86711 * (-((locals.var_arg_dn11 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn11)))) / assign52480_e86720),)
    } else {
        (locals.var_qbsj3, locals.var_qbsj3_dn3, locals.var_qbsj3_dn4, locals.var_qbsj3_dn5, locals.var_qbsj3_dn6, locals.var_qbsj3_dn7, locals.var_qbsj3_dn8, locals.var_qbsj3_dn9, locals.var_qbsj3_dn10, locals.var_qbsj3_dn11,)
    }
};
        locals.var_qbsj3 = assign52480_e86723;
        locals.var_qbsj3_dn3 = assign52480_e86723_d_n3;
        locals.var_qbsj3_dn4 = assign52480_e86723_d_n4;
        locals.var_qbsj3_dn5 = assign52480_e86723_d_n5;
        locals.var_qbsj3_dn6 = assign52480_e86723_d_n6;
        locals.var_qbsj3_dn7 = assign52480_e86723_d_n7;
        locals.var_qbsj3_dn8 = assign52480_e86723_d_n8;
        locals.var_qbsj3_dn9 = assign52480_e86723_d_n9;
        locals.var_qbsj3_dn10 = assign52480_e86723_d_n10;
        locals.var_qbsj3_dn11 = assign52480_e86723_d_n11;

        let (assign52490_e86741, assign52490_e86741_d_n3, assign52490_e86741_d_n4, assign52490_e86741_d_n5, assign52490_e86741_d_n6, assign52490_e86741_d_n7, assign52490_e86741_d_n8, assign52490_e86741_d_n9, assign52490_e86741_d_n10, assign52490_e86741_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard800 != 0.0)) && (locals.var_guard801 != 0.0)) && (locals.var_guard802 == 0.0)) {
        let assign52490_e86735: f64 = (locals.var_pbswgs_t * locals.var_czbsswg);
        let assign52490_e86737: f64 = (locals.var_arg).ln();
        let assign52490_e86738: f64 = (-assign52490_e86737);
        let assign52490_e86739: f64 = (assign52490_e86735 * assign52490_e86738);
        (assign52490_e86739, (assign52490_e86735 * (-(locals.var_arg_dn3 / locals.var_arg))), ((((locals.var_pbswgs_t_dn4 * locals.var_czbsswg) + (locals.var_pbswgs_t * locals.var_czbsswg_dn4)) * assign52490_e86738) + (assign52490_e86735 * (-(locals.var_arg_dn4 / locals.var_arg)))), ((((locals.var_pbswgs_t_dn5 * locals.var_czbsswg) + (locals.var_pbswgs_t * locals.var_czbsswg_dn5)) * assign52490_e86738) + (assign52490_e86735 * (-(locals.var_arg_dn5 / locals.var_arg)))), (assign52490_e86735 * (-(locals.var_arg_dn6 / locals.var_arg))), (assign52490_e86735 * (-(locals.var_arg_dn7 / locals.var_arg))), (assign52490_e86735 * (-(locals.var_arg_dn8 / locals.var_arg))), (assign52490_e86735 * (-(locals.var_arg_dn9 / locals.var_arg))), (assign52490_e86735 * (-(locals.var_arg_dn10 / locals.var_arg))), (assign52490_e86735 * (-(locals.var_arg_dn11 / locals.var_arg))),)
    } else {
        (locals.var_qbsj3, locals.var_qbsj3_dn3, locals.var_qbsj3_dn4, locals.var_qbsj3_dn5, locals.var_qbsj3_dn6, locals.var_qbsj3_dn7, locals.var_qbsj3_dn8, locals.var_qbsj3_dn9, locals.var_qbsj3_dn10, locals.var_qbsj3_dn11,)
    }
};
        locals.var_qbsj3 = assign52490_e86741;
        locals.var_qbsj3_dn3 = assign52490_e86741_d_n3;
        locals.var_qbsj3_dn4 = assign52490_e86741_d_n4;
        locals.var_qbsj3_dn5 = assign52490_e86741_d_n5;
        locals.var_qbsj3_dn6 = assign52490_e86741_d_n6;
        locals.var_qbsj3_dn7 = assign52490_e86741_d_n7;
        locals.var_qbsj3_dn8 = assign52490_e86741_d_n8;
        locals.var_qbsj3_dn9 = assign52490_e86741_d_n9;
        locals.var_qbsj3_dn10 = assign52490_e86741_d_n10;
        locals.var_qbsj3_dn11 = assign52490_e86741_d_n11;

        let (assign52500_e86767, assign52500_e86767_d_n3, assign52500_e86767_d_n4, assign52500_e86767_d_n5, assign52500_e86767_d_n6, assign52500_e86767_d_n7, assign52500_e86767_d_n8, assign52500_e86767_d_n9, assign52500_e86767_d_n10, assign52500_e86767_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard800 != 0.0)) && (locals.var_guard801 == 0.0)) {
        let assign52500_e86752: f64 = (locals.var_t1 - 1.0);
        let assign52500_e86753: f64 = (locals.var_czbsswg_p1 * assign52500_e86752);
        let assign52500_e86756: f64 = (5.0 * p.p917);
        let assign52500_e86759: f64 = (locals.var_t1 - 1.0);
        let assign52500_e86760: f64 = (assign52500_e86756 * assign52500_e86759);
        let assign52500_e86763: f64 = (1.0 + p.p917);
        let assign52500_e86764: f64 = (assign52500_e86760 + assign52500_e86763);
        let assign52500_e86765: f64 = (assign52500_e86753 * assign52500_e86764);
        (assign52500_e86765, (((locals.var_czbsswg_p1 * locals.var_t1_dn3) * assign52500_e86764) + (assign52500_e86753 * (assign52500_e86756 * locals.var_t1_dn3))), (((locals.var_czbsswg_p1 * locals.var_t1_dn4) * assign52500_e86764) + (assign52500_e86753 * (assign52500_e86756 * locals.var_t1_dn4))), (((locals.var_czbsswg_p1 * locals.var_t1_dn5) * assign52500_e86764) + (assign52500_e86753 * (assign52500_e86756 * locals.var_t1_dn5))), (((locals.var_czbsswg_p1 * locals.var_t1_dn6) * assign52500_e86764) + (assign52500_e86753 * (assign52500_e86756 * locals.var_t1_dn6))), (((locals.var_czbsswg_p1 * locals.var_t1_dn7) * assign52500_e86764) + (assign52500_e86753 * (assign52500_e86756 * locals.var_t1_dn7))), (((locals.var_czbsswg_p1 * locals.var_t1_dn8) * assign52500_e86764) + (assign52500_e86753 * (assign52500_e86756 * locals.var_t1_dn8))), (((locals.var_czbsswg_p1 * locals.var_t1_dn9) * assign52500_e86764) + (assign52500_e86753 * (assign52500_e86756 * locals.var_t1_dn9))), (((locals.var_czbsswg_p1 * locals.var_t1_dn10) * assign52500_e86764) + (assign52500_e86753 * (assign52500_e86756 * locals.var_t1_dn10))), (((locals.var_czbsswg_p1 * locals.var_t1_dn11) * assign52500_e86764) + (assign52500_e86753 * (assign52500_e86756 * locals.var_t1_dn11))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign52500_e86767;
        locals.var_t2_dn3 = assign52500_e86767_d_n3;
        locals.var_t2_dn4 = assign52500_e86767_d_n4;
        locals.var_t2_dn5 = assign52500_e86767_d_n5;
        locals.var_t2_dn6 = assign52500_e86767_d_n6;
        locals.var_t2_dn7 = assign52500_e86767_d_n7;
        locals.var_t2_dn8 = assign52500_e86767_d_n8;
        locals.var_t2_dn9 = assign52500_e86767_d_n9;
        locals.var_t2_dn10 = assign52500_e86767_d_n10;
        locals.var_t2_dn11 = assign52500_e86767_d_n11;

        let (assign52510_e86783, assign52510_e86783_d_n3, assign52510_e86783_d_n4, assign52510_e86783_d_n5, assign52510_e86783_d_n6, assign52510_e86783_d_n7, assign52510_e86783_d_n8, assign52510_e86783_d_n9, assign52510_e86783_d_n10, assign52510_e86783_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard800 != 0.0)) && (locals.var_guard801 == 0.0)) {
        let assign52510_e86777: f64 = (locals.var_pbswgs_t * locals.var_czbsswg);
        let assign52510_e86780: f64 = (locals.var_t2 + locals.var_czbsswg_p2);
        let assign52510_e86781: f64 = (assign52510_e86777 * assign52510_e86780);
        (assign52510_e86781, (assign52510_e86777 * locals.var_t2_dn3), ((((locals.var_pbswgs_t_dn4 * locals.var_czbsswg) + (locals.var_pbswgs_t * locals.var_czbsswg_dn4)) * assign52510_e86780) + (assign52510_e86777 * locals.var_t2_dn4)), ((((locals.var_pbswgs_t_dn5 * locals.var_czbsswg) + (locals.var_pbswgs_t * locals.var_czbsswg_dn5)) * assign52510_e86780) + (assign52510_e86777 * locals.var_t2_dn5)), (assign52510_e86777 * locals.var_t2_dn6), (assign52510_e86777 * locals.var_t2_dn7), (assign52510_e86777 * locals.var_t2_dn8), (assign52510_e86777 * locals.var_t2_dn9), (assign52510_e86777 * locals.var_t2_dn10), (assign52510_e86777 * locals.var_t2_dn11),)
    } else {
        (locals.var_qbsj3, locals.var_qbsj3_dn3, locals.var_qbsj3_dn4, locals.var_qbsj3_dn5, locals.var_qbsj3_dn6, locals.var_qbsj3_dn7, locals.var_qbsj3_dn8, locals.var_qbsj3_dn9, locals.var_qbsj3_dn10, locals.var_qbsj3_dn11,)
    }
};
        locals.var_qbsj3 = assign52510_e86783;
        locals.var_qbsj3_dn3 = assign52510_e86783_d_n3;
        locals.var_qbsj3_dn4 = assign52510_e86783_d_n4;
        locals.var_qbsj3_dn5 = assign52510_e86783_d_n5;
        locals.var_qbsj3_dn6 = assign52510_e86783_d_n6;
        locals.var_qbsj3_dn7 = assign52510_e86783_d_n7;
        locals.var_qbsj3_dn8 = assign52510_e86783_d_n8;
        locals.var_qbsj3_dn9 = assign52510_e86783_d_n9;
        locals.var_qbsj3_dn10 = assign52510_e86783_d_n10;
        locals.var_qbsj3_dn11 = assign52510_e86783_d_n11;

        let (assign52520_e86791, assign52520_e86791_d_n3, assign52520_e86791_d_n4, assign52520_e86791_d_n5, assign52520_e86791_d_n6, assign52520_e86791_d_n7, assign52520_e86791_d_n8, assign52520_e86791_d_n9, assign52520_e86791_d_n10, assign52520_e86791_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard800 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbsj3, locals.var_qbsj3_dn3, locals.var_qbsj3_dn4, locals.var_qbsj3_dn5, locals.var_qbsj3_dn6, locals.var_qbsj3_dn7, locals.var_qbsj3_dn8, locals.var_qbsj3_dn9, locals.var_qbsj3_dn10, locals.var_qbsj3_dn11,)
    }
};
        locals.var_qbsj3 = assign52520_e86791;
        locals.var_qbsj3_dn3 = assign52520_e86791_d_n3;
        locals.var_qbsj3_dn4 = assign52520_e86791_d_n4;
        locals.var_qbsj3_dn5 = assign52520_e86791_d_n5;
        locals.var_qbsj3_dn6 = assign52520_e86791_d_n6;
        locals.var_qbsj3_dn7 = assign52520_e86791_d_n7;
        locals.var_qbsj3_dn8 = assign52520_e86791_d_n8;
        locals.var_qbsj3_dn9 = assign52520_e86791_d_n9;
        locals.var_qbsj3_dn10 = assign52520_e86791_d_n10;
        locals.var_qbsj3_dn11 = assign52520_e86791_d_n11;

        let (assign52530_e86800, assign52530_e86800_d_n3, assign52530_e86800_d_n4, assign52530_e86800_d_n5, assign52530_e86800_d_n6, assign52530_e86800_d_n7, assign52530_e86800_d_n8, assign52530_e86800_d_n9, assign52530_e86800_d_n10, assign52530_e86800_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign52530_e86796: f64 = (p.p919 * locals.var_ibsdif);
        let assign52530_e86798: f64 = (assign52530_e86796 * p.p2);
        (assign52530_e86798, ((p.p919 * locals.var_ibsdif_dn3) * p.p2), ((p.p919 * locals.var_ibsdif_dn4) * p.p2), ((p.p919 * locals.var_ibsdif_dn5) * p.p2), ((p.p919 * locals.var_ibsdif_dn6) * p.p2), ((p.p919 * locals.var_ibsdif_dn7) * p.p2), ((p.p919 * locals.var_ibsdif_dn8) * p.p2), ((p.p919 * locals.var_ibsdif_dn9) * p.p2), ((p.p919 * locals.var_ibsdif_dn10) * p.p2), ((p.p919 * locals.var_ibsdif_dn11) * p.p2),)
    } else {
        (locals.var_qbsj4, locals.var_qbsj4_dn3, locals.var_qbsj4_dn4, locals.var_qbsj4_dn5, locals.var_qbsj4_dn6, locals.var_qbsj4_dn7, locals.var_qbsj4_dn8, locals.var_qbsj4_dn9, locals.var_qbsj4_dn10, locals.var_qbsj4_dn11,)
    }
};
        locals.var_qbsj4 = assign52530_e86800;
        locals.var_qbsj4_dn3 = assign52530_e86800_d_n3;
        locals.var_qbsj4_dn4 = assign52530_e86800_d_n4;
        locals.var_qbsj4_dn5 = assign52530_e86800_d_n5;
        locals.var_qbsj4_dn6 = assign52530_e86800_d_n6;
        locals.var_qbsj4_dn7 = assign52530_e86800_d_n7;
        locals.var_qbsj4_dn8 = assign52530_e86800_d_n8;
        locals.var_qbsj4_dn9 = assign52530_e86800_d_n9;
        locals.var_qbsj4_dn10 = assign52530_e86800_d_n10;
        locals.var_qbsj4_dn11 = assign52530_e86800_d_n11;

        let (assign52540_e86811, assign52540_e86811_d_n3, assign52540_e86811_d_n4, assign52540_e86811_d_n5, assign52540_e86811_d_n6, assign52540_e86811_d_n7, assign52540_e86811_d_n8, assign52540_e86811_d_n9, assign52540_e86811_d_n10, assign52540_e86811_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign52540_e86805: f64 = (locals.var_qbsj1 + locals.var_qbsj2);
        let assign52540_e86807: f64 = (assign52540_e86805 + locals.var_qbsj3);
        let assign52540_e86809: f64 = (assign52540_e86807 + locals.var_qbsj4);
        (assign52540_e86809, (((locals.var_qbsj1_dn3 + locals.var_qbsj2_dn3) + locals.var_qbsj3_dn3) + locals.var_qbsj4_dn3), (((locals.var_qbsj1_dn4 + locals.var_qbsj2_dn4) + locals.var_qbsj3_dn4) + locals.var_qbsj4_dn4), (((locals.var_qbsj1_dn5 + locals.var_qbsj2_dn5) + locals.var_qbsj3_dn5) + locals.var_qbsj4_dn5), (((locals.var_qbsj1_dn6 + locals.var_qbsj2_dn6) + locals.var_qbsj3_dn6) + locals.var_qbsj4_dn6), (((locals.var_qbsj1_dn7 + locals.var_qbsj2_dn7) + locals.var_qbsj3_dn7) + locals.var_qbsj4_dn7), (((locals.var_qbsj1_dn8 + locals.var_qbsj2_dn8) + locals.var_qbsj3_dn8) + locals.var_qbsj4_dn8), (((locals.var_qbsj1_dn9 + locals.var_qbsj2_dn9) + locals.var_qbsj3_dn9) + locals.var_qbsj4_dn9), (((locals.var_qbsj1_dn10 + locals.var_qbsj2_dn10) + locals.var_qbsj3_dn10) + locals.var_qbsj4_dn10), (((locals.var_qbsj1_dn11 + locals.var_qbsj2_dn11) + locals.var_qbsj3_dn11) + locals.var_qbsj4_dn11),)
    } else {
        (locals.var_qbsj, locals.var_qbsj_dn3, locals.var_qbsj_dn4, locals.var_qbsj_dn5, locals.var_qbsj_dn6, locals.var_qbsj_dn7, locals.var_qbsj_dn8, locals.var_qbsj_dn9, locals.var_qbsj_dn10, locals.var_qbsj_dn11,)
    }
};
        locals.var_qbsj = assign52540_e86811;
        locals.var_qbsj_dn3 = assign52540_e86811_d_n3;
        locals.var_qbsj_dn4 = assign52540_e86811_d_n4;
        locals.var_qbsj_dn5 = assign52540_e86811_d_n5;
        locals.var_qbsj_dn6 = assign52540_e86811_d_n6;
        locals.var_qbsj_dn7 = assign52540_e86811_d_n7;
        locals.var_qbsj_dn8 = assign52540_e86811_d_n8;
        locals.var_qbsj_dn9 = assign52540_e86811_d_n9;
        locals.var_qbsj_dn10 = assign52540_e86811_d_n10;
        locals.var_qbsj_dn11 = assign52540_e86811_d_n11;

        let (assign52550_e86818, assign52550_e86818_d_n3, assign52550_e86818_d_n4, assign52550_e86818_d_n5, assign52550_e86818_d_n6, assign52550_e86818_d_n7, assign52550_e86818_d_n8, assign52550_e86818_d_n9, assign52550_e86818_d_n10, assign52550_e86818_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign52550_e86816: f64 = (locals.var_cjd_t * locals.var_adeff);
        (assign52550_e86816, (locals.var_cjd_t * locals.var_adeff_dn3), ((locals.var_cjd_t_dn4 * locals.var_adeff) + (locals.var_cjd_t * locals.var_adeff_dn4)), ((locals.var_cjd_t_dn5 * locals.var_adeff) + (locals.var_cjd_t * locals.var_adeff_dn5)), (locals.var_cjd_t * locals.var_adeff_dn6), (locals.var_cjd_t * locals.var_adeff_dn7), (locals.var_cjd_t * locals.var_adeff_dn8), (locals.var_cjd_t * locals.var_adeff_dn9), (locals.var_cjd_t * locals.var_adeff_dn10), (locals.var_cjd_t * locals.var_adeff_dn11),)
    } else {
        (locals.var_czbd, locals.var_czbd_dn3, locals.var_czbd_dn4, locals.var_czbd_dn5, locals.var_czbd_dn6, locals.var_czbd_dn7, locals.var_czbd_dn8, locals.var_czbd_dn9, locals.var_czbd_dn10, locals.var_czbd_dn11,)
    }
};
        locals.var_czbd = assign52550_e86818;
        locals.var_czbd_dn3 = assign52550_e86818_d_n3;
        locals.var_czbd_dn4 = assign52550_e86818_d_n4;
        locals.var_czbd_dn5 = assign52550_e86818_d_n5;
        locals.var_czbd_dn6 = assign52550_e86818_d_n6;
        locals.var_czbd_dn7 = assign52550_e86818_d_n7;
        locals.var_czbd_dn8 = assign52550_e86818_d_n8;
        locals.var_czbd_dn9 = assign52550_e86818_d_n9;
        locals.var_czbd_dn10 = assign52550_e86818_d_n10;
        locals.var_czbd_dn11 = assign52550_e86818_d_n11;

        let (assign52560_e86825, assign52560_e86825_d_n3, assign52560_e86825_d_n4, assign52560_e86825_d_n5, assign52560_e86825_d_n6, assign52560_e86825_d_n7, assign52560_e86825_d_n8, assign52560_e86825_d_n9, assign52560_e86825_d_n10, assign52560_e86825_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign52560_e86823: f64 = (locals.var_cjswd_t * locals.var_pdeff);
        (assign52560_e86823, (locals.var_cjswd_t * locals.var_pdeff_dn3), ((locals.var_cjswd_t_dn4 * locals.var_pdeff) + (locals.var_cjswd_t * locals.var_pdeff_dn4)), ((locals.var_cjswd_t_dn5 * locals.var_pdeff) + (locals.var_cjswd_t * locals.var_pdeff_dn5)), (locals.var_cjswd_t * locals.var_pdeff_dn6), (locals.var_cjswd_t * locals.var_pdeff_dn7), (locals.var_cjswd_t * locals.var_pdeff_dn8), (locals.var_cjswd_t * locals.var_pdeff_dn9), (locals.var_cjswd_t * locals.var_pdeff_dn10), (locals.var_cjswd_t * locals.var_pdeff_dn11),)
    } else {
        (locals.var_czbdsw, locals.var_czbdsw_dn3, locals.var_czbdsw_dn4, locals.var_czbdsw_dn5, locals.var_czbdsw_dn6, locals.var_czbdsw_dn7, locals.var_czbdsw_dn8, locals.var_czbdsw_dn9, locals.var_czbdsw_dn10, locals.var_czbdsw_dn11,)
    }
};
        locals.var_czbdsw = assign52560_e86825;
        locals.var_czbdsw_dn3 = assign52560_e86825_d_n3;
        locals.var_czbdsw_dn4 = assign52560_e86825_d_n4;
        locals.var_czbdsw_dn5 = assign52560_e86825_d_n5;
        locals.var_czbdsw_dn6 = assign52560_e86825_d_n6;
        locals.var_czbdsw_dn7 = assign52560_e86825_d_n7;
        locals.var_czbdsw_dn8 = assign52560_e86825_d_n8;
        locals.var_czbdsw_dn9 = assign52560_e86825_d_n9;
        locals.var_czbdsw_dn10 = assign52560_e86825_d_n10;
        locals.var_czbdsw_dn11 = assign52560_e86825_d_n11;

        let (assign52570_e86834, assign52570_e86834_d_n4, assign52570_e86834_d_n5,) = {
    if (locals.var_guard492 == 0.0) {
        let assign52570_e86830: f64 = (locals.var_cjswgd_t * locals.var_weffcj);
        let assign52570_e86832: f64 = (assign52570_e86830 * p.p2);
        (assign52570_e86832, ((locals.var_cjswgd_t_dn4 * locals.var_weffcj) * p.p2), ((locals.var_cjswgd_t_dn5 * locals.var_weffcj) * p.p2),)
    } else {
        (locals.var_czbdswg, locals.var_czbdswg_dn4, locals.var_czbdswg_dn5,)
    }
};
        locals.var_czbdswg = assign52570_e86834;
        locals.var_czbdswg_dn4 = assign52570_e86834_d_n4;
        locals.var_czbdswg_dn5 = assign52570_e86834_d_n5;

        let (assign52580_e86842,) = {
    if (locals.var_guard492 == 0.0) {
        let assign52580_e86839: f64 = (-p.p914);
        let assign52580_e86840: f64 = (0.1_f64).powf(assign52580_e86839);
        (assign52580_e86840,)
    } else {
        (locals.var_czbd_p1,)
    }
};
        locals.var_czbd_p1 = assign52580_e86842;

        let assign52590_e86845: f64 = if p.p914 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard804 = assign52590_e86845;

        let (assign52600_e86855,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard804 != 0.0)) {
        let assign52600_e86852: f64 = (0.1_f64).ln();
        let assign52600_e86853: f64 = (1.5 - assign52600_e86852);
        (assign52600_e86853,)
    } else {
        (locals.var_czbd_p2,)
    }
};
        locals.var_czbd_p2 = assign52600_e86855;

        let (assign52610_e86879,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard804 == 0.0)) {
        let assign52610_e86864: f64 = (1.0 - p.p914);
        let assign52610_e86865: f64 = (1.0 / assign52610_e86864);
        let assign52610_e86869: f64 = (0.05 * p.p914);
        let assign52610_e86872: f64 = (1.0 + p.p914);
        let assign52610_e86873: f64 = (assign52610_e86869 * assign52610_e86872);
        let assign52610_e86875: f64 = (assign52610_e86873 * locals.var_czbd_p1);
        let assign52610_e86876: f64 = (1.0 - assign52610_e86875);
        let assign52610_e86877: f64 = (assign52610_e86865 * assign52610_e86876);
        (assign52610_e86877,)
    } else {
        (locals.var_czbd_p2,)
    }
};
        locals.var_czbd_p2 = assign52610_e86879;

        let (assign52620_e86887,) = {
    if (locals.var_guard492 == 0.0) {
        let assign52620_e86884: f64 = (-p.p916);
        let assign52620_e86885: f64 = (0.1_f64).powf(assign52620_e86884);
        (assign52620_e86885,)
    } else {
        (locals.var_czbdsw_p1,)
    }
};
        locals.var_czbdsw_p1 = assign52620_e86887;

        let assign52630_e86890: f64 = if p.p916 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard805 = assign52630_e86890;

        let (assign52640_e86900,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard805 != 0.0)) {
        let assign52640_e86897: f64 = (0.1_f64).ln();
        let assign52640_e86898: f64 = (1.5 - assign52640_e86897);
        (assign52640_e86898,)
    } else {
        (locals.var_czbdsw_p2,)
    }
};
        locals.var_czbdsw_p2 = assign52640_e86900;

        let (assign52650_e86924,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard805 == 0.0)) {
        let assign52650_e86909: f64 = (1.0 - p.p916);
        let assign52650_e86910: f64 = (1.0 / assign52650_e86909);
        let assign52650_e86914: f64 = (0.05 * p.p916);
        let assign52650_e86917: f64 = (1.0 + p.p916);
        let assign52650_e86918: f64 = (assign52650_e86914 * assign52650_e86917);
        let assign52650_e86920: f64 = (assign52650_e86918 * locals.var_czbdsw_p1);
        let assign52650_e86921: f64 = (1.0 - assign52650_e86920);
        let assign52650_e86922: f64 = (assign52650_e86910 * assign52650_e86921);
        (assign52650_e86922,)
    } else {
        (locals.var_czbdsw_p2,)
    }
};
        locals.var_czbdsw_p2 = assign52650_e86924;

        let (assign52660_e86932,) = {
    if (locals.var_guard492 == 0.0) {
        let assign52660_e86929: f64 = (-p.p918);
        let assign52660_e86930: f64 = (0.1_f64).powf(assign52660_e86929);
        (assign52660_e86930,)
    } else {
        (locals.var_czbdswg_p1,)
    }
};
        locals.var_czbdswg_p1 = assign52660_e86932;

        let assign52670_e86935: f64 = if p.p918 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard806 = assign52670_e86935;

        let (assign52680_e86945,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard806 != 0.0)) {
        let assign52680_e86942: f64 = (0.1_f64).ln();
        let assign52680_e86943: f64 = (1.5 - assign52680_e86942);
        (assign52680_e86943,)
    } else {
        (locals.var_czbdswg_p2,)
    }
};
        locals.var_czbdswg_p2 = assign52680_e86945;

        let (assign52690_e86969,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard806 == 0.0)) {
        let assign52690_e86954: f64 = (1.0 - p.p918);
        let assign52690_e86955: f64 = (1.0 / assign52690_e86954);
        let assign52690_e86959: f64 = (0.05 * p.p918);
        let assign52690_e86962: f64 = (1.0 + p.p918);
        let assign52690_e86963: f64 = (assign52690_e86959 * assign52690_e86962);
        let assign52690_e86965: f64 = (assign52690_e86963 * locals.var_czbdswg_p1);
        let assign52690_e86966: f64 = (1.0 - assign52690_e86965);
        let assign52690_e86967: f64 = (assign52690_e86955 * assign52690_e86966);
        (assign52690_e86967,)
    } else {
        (locals.var_czbdswg_p2,)
    }
};
        locals.var_czbdswg_p2 = assign52690_e86969;

        let assign52700_e86972: f64 = if locals.var_czbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard807 = assign52700_e86972;

        let (assign52710_e86981, assign52710_e86981_d_n3, assign52710_e86981_d_n4, assign52710_e86981_d_n5, assign52710_e86981_d_n6, assign52710_e86981_d_n7, assign52710_e86981_d_n8, assign52710_e86981_d_n9, assign52710_e86981_d_n10, assign52710_e86981_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard807 != 0.0)) {
        let assign52710_e86979: f64 = (locals.var_vbd_jct / locals.var_pbd_t);
        (assign52710_e86979, 0.0, (-((locals.var_vbd_jct * locals.var_pbd_t_dn4) / (locals.var_pbd_t * locals.var_pbd_t))), (-((locals.var_vbd_jct * locals.var_pbd_t_dn5) / (locals.var_pbd_t * locals.var_pbd_t))), (locals.var_vbd_jct_dn6 / locals.var_pbd_t), 0.0, 0.0, 0.0, (locals.var_vbd_jct_dn10 / locals.var_pbd_t), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign52710_e86981;
        locals.var_t1_dn3 = assign52710_e86981_d_n3;
        locals.var_t1_dn4 = assign52710_e86981_d_n4;
        locals.var_t1_dn5 = assign52710_e86981_d_n5;
        locals.var_t1_dn6 = assign52710_e86981_d_n6;
        locals.var_t1_dn7 = assign52710_e86981_d_n7;
        locals.var_t1_dn8 = assign52710_e86981_d_n8;
        locals.var_t1_dn9 = assign52710_e86981_d_n9;
        locals.var_t1_dn10 = assign52710_e86981_d_n10;
        locals.var_t1_dn11 = assign52710_e86981_d_n11;

        let assign52720_e86984: f64 = if locals.var_t1 < 0.9 { 1.0 } else { 0.0 };
        locals.var_guard808 = assign52720_e86984;

        let (assign52730_e86995, assign52730_e86995_d_n3, assign52730_e86995_d_n4, assign52730_e86995_d_n5, assign52730_e86995_d_n6, assign52730_e86995_d_n7, assign52730_e86995_d_n8, assign52730_e86995_d_n9, assign52730_e86995_d_n10, assign52730_e86995_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard807 != 0.0)) && (locals.var_guard808 != 0.0)) {
        let assign52730_e86993: f64 = (1.0 - locals.var_t1);
        (assign52730_e86993, (-locals.var_t1_dn3), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn11),)
    } else {
        (locals.var_arg, locals.var_arg_dn3, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11,)
    }
};
        locals.var_arg = assign52730_e86995;
        locals.var_arg_dn3 = assign52730_e86995_d_n3;
        locals.var_arg_dn4 = assign52730_e86995_d_n4;
        locals.var_arg_dn5 = assign52730_e86995_d_n5;
        locals.var_arg_dn6 = assign52730_e86995_d_n6;
        locals.var_arg_dn7 = assign52730_e86995_d_n7;
        locals.var_arg_dn8 = assign52730_e86995_d_n8;
        locals.var_arg_dn9 = assign52730_e86995_d_n9;
        locals.var_arg_dn10 = assign52730_e86995_d_n10;
        locals.var_arg_dn11 = assign52730_e86995_d_n11;

        let assign52740_e86998: f64 = if p.p914 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard809 = assign52740_e86998;

        let assign52750_e87001: f64 = if p.p914 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard810 = assign52750_e87001;

        let (assign52760_e87017, assign52760_e87017_d_n3, assign52760_e87017_d_n4, assign52760_e87017_d_n5, assign52760_e87017_d_n6, assign52760_e87017_d_n7, assign52760_e87017_d_n8, assign52760_e87017_d_n9, assign52760_e87017_d_n10, assign52760_e87017_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard807 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard809 != 0.0)) && (locals.var_guard810 != 0.0)) {
        let assign52760_e87014: f64 = (locals.var_arg).sqrt();
        let assign52760_e87015: f64 = (1.0 / assign52760_e87014);
        (assign52760_e87015, (-((locals.var_arg_dn3 / (2.0 * assign52760_e87014)) / (assign52760_e87014 * assign52760_e87014))), (-((locals.var_arg_dn4 / (2.0 * assign52760_e87014)) / (assign52760_e87014 * assign52760_e87014))), (-((locals.var_arg_dn5 / (2.0 * assign52760_e87014)) / (assign52760_e87014 * assign52760_e87014))), (-((locals.var_arg_dn6 / (2.0 * assign52760_e87014)) / (assign52760_e87014 * assign52760_e87014))), (-((locals.var_arg_dn7 / (2.0 * assign52760_e87014)) / (assign52760_e87014 * assign52760_e87014))), (-((locals.var_arg_dn8 / (2.0 * assign52760_e87014)) / (assign52760_e87014 * assign52760_e87014))), (-((locals.var_arg_dn9 / (2.0 * assign52760_e87014)) / (assign52760_e87014 * assign52760_e87014))), (-((locals.var_arg_dn10 / (2.0 * assign52760_e87014)) / (assign52760_e87014 * assign52760_e87014))), (-((locals.var_arg_dn11 / (2.0 * assign52760_e87014)) / (assign52760_e87014 * assign52760_e87014))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11,)
    }
};
        locals.var_sarg = assign52760_e87017;
        locals.var_sarg_dn3 = assign52760_e87017_d_n3;
        locals.var_sarg_dn4 = assign52760_e87017_d_n4;
        locals.var_sarg_dn5 = assign52760_e87017_d_n5;
        locals.var_sarg_dn6 = assign52760_e87017_d_n6;
        locals.var_sarg_dn7 = assign52760_e87017_d_n7;
        locals.var_sarg_dn8 = assign52760_e87017_d_n8;
        locals.var_sarg_dn9 = assign52760_e87017_d_n9;
        locals.var_sarg_dn10 = assign52760_e87017_d_n10;
        locals.var_sarg_dn11 = assign52760_e87017_d_n11;

        let (assign52770_e87036, assign52770_e87036_d_n3, assign52770_e87036_d_n4, assign52770_e87036_d_n5, assign52770_e87036_d_n6, assign52770_e87036_d_n7, assign52770_e87036_d_n8, assign52770_e87036_d_n9, assign52770_e87036_d_n10, assign52770_e87036_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard807 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard809 != 0.0)) && (locals.var_guard810 == 0.0)) {
        let assign52770_e87030: f64 = (-p.p914);
        let assign52770_e87032: f64 = (locals.var_arg).ln();
        let assign52770_e87033: f64 = (assign52770_e87030 * assign52770_e87032);
        let assign52770_e87034: f64 = { let limited_exp_arg = assign52770_e87033; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign52770_e87034, ({ let limited_exp_arg = assign52770_e87033; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52770_e87030 * (locals.var_arg_dn3 / locals.var_arg))), ({ let limited_exp_arg = assign52770_e87033; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52770_e87030 * (locals.var_arg_dn4 / locals.var_arg))), ({ let limited_exp_arg = assign52770_e87033; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52770_e87030 * (locals.var_arg_dn5 / locals.var_arg))), ({ let limited_exp_arg = assign52770_e87033; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52770_e87030 * (locals.var_arg_dn6 / locals.var_arg))), ({ let limited_exp_arg = assign52770_e87033; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52770_e87030 * (locals.var_arg_dn7 / locals.var_arg))), ({ let limited_exp_arg = assign52770_e87033; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52770_e87030 * (locals.var_arg_dn8 / locals.var_arg))), ({ let limited_exp_arg = assign52770_e87033; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52770_e87030 * (locals.var_arg_dn9 / locals.var_arg))), ({ let limited_exp_arg = assign52770_e87033; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52770_e87030 * (locals.var_arg_dn10 / locals.var_arg))), ({ let limited_exp_arg = assign52770_e87033; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52770_e87030 * (locals.var_arg_dn11 / locals.var_arg))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11,)
    }
};
        locals.var_sarg = assign52770_e87036;
        locals.var_sarg_dn3 = assign52770_e87036_d_n3;
        locals.var_sarg_dn4 = assign52770_e87036_d_n4;
        locals.var_sarg_dn5 = assign52770_e87036_d_n5;
        locals.var_sarg_dn6 = assign52770_e87036_d_n6;
        locals.var_sarg_dn7 = assign52770_e87036_d_n7;
        locals.var_sarg_dn8 = assign52770_e87036_d_n8;
        locals.var_sarg_dn9 = assign52770_e87036_d_n9;
        locals.var_sarg_dn10 = assign52770_e87036_d_n10;
        locals.var_sarg_dn11 = assign52770_e87036_d_n11;

        let (assign52780_e87059, assign52780_e87059_d_n3, assign52780_e87059_d_n4, assign52780_e87059_d_n5, assign52780_e87059_d_n6, assign52780_e87059_d_n7, assign52780_e87059_d_n8, assign52780_e87059_d_n9, assign52780_e87059_d_n10, assign52780_e87059_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard807 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard809 != 0.0)) {
        let assign52780_e87047: f64 = (locals.var_pbd_t * locals.var_czbd);
        let assign52780_e87051: f64 = (locals.var_arg * locals.var_sarg);
        let assign52780_e87052: f64 = (1.0 - assign52780_e87051);
        let assign52780_e87053: f64 = (assign52780_e87047 * assign52780_e87052);
        let assign52780_e87056: f64 = (1.0 - p.p914);
        let assign52780_e87057: f64 = (assign52780_e87053 / assign52780_e87056);
        (assign52780_e87057, ((((locals.var_pbd_t * locals.var_czbd_dn3) * assign52780_e87052) + (assign52780_e87047 * (-((locals.var_arg_dn3 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn3))))) / assign52780_e87056), (((((locals.var_pbd_t_dn4 * locals.var_czbd) + (locals.var_pbd_t * locals.var_czbd_dn4)) * assign52780_e87052) + (assign52780_e87047 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign52780_e87056), (((((locals.var_pbd_t_dn5 * locals.var_czbd) + (locals.var_pbd_t * locals.var_czbd_dn5)) * assign52780_e87052) + (assign52780_e87047 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign52780_e87056), ((((locals.var_pbd_t * locals.var_czbd_dn6) * assign52780_e87052) + (assign52780_e87047 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign52780_e87056), ((((locals.var_pbd_t * locals.var_czbd_dn7) * assign52780_e87052) + (assign52780_e87047 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign52780_e87056), ((((locals.var_pbd_t * locals.var_czbd_dn8) * assign52780_e87052) + (assign52780_e87047 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign52780_e87056), ((((locals.var_pbd_t * locals.var_czbd_dn9) * assign52780_e87052) + (assign52780_e87047 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign52780_e87056), ((((locals.var_pbd_t * locals.var_czbd_dn10) * assign52780_e87052) + (assign52780_e87047 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign52780_e87056), ((((locals.var_pbd_t * locals.var_czbd_dn11) * assign52780_e87052) + (assign52780_e87047 * (-((locals.var_arg_dn11 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn11))))) / assign52780_e87056),)
    } else {
        (locals.var_qbdj1, locals.var_qbdj1_dn3, locals.var_qbdj1_dn4, locals.var_qbdj1_dn5, locals.var_qbdj1_dn6, locals.var_qbdj1_dn7, locals.var_qbdj1_dn8, locals.var_qbdj1_dn9, locals.var_qbdj1_dn10, locals.var_qbdj1_dn11,)
    }
};
        locals.var_qbdj1 = assign52780_e87059;
        locals.var_qbdj1_dn3 = assign52780_e87059_d_n3;
        locals.var_qbdj1_dn4 = assign52780_e87059_d_n4;
        locals.var_qbdj1_dn5 = assign52780_e87059_d_n5;
        locals.var_qbdj1_dn6 = assign52780_e87059_d_n6;
        locals.var_qbdj1_dn7 = assign52780_e87059_d_n7;
        locals.var_qbdj1_dn8 = assign52780_e87059_d_n8;
        locals.var_qbdj1_dn9 = assign52780_e87059_d_n9;
        locals.var_qbdj1_dn10 = assign52780_e87059_d_n10;
        locals.var_qbdj1_dn11 = assign52780_e87059_d_n11;

        let (assign52790_e87077, assign52790_e87077_d_n3, assign52790_e87077_d_n4, assign52790_e87077_d_n5, assign52790_e87077_d_n6, assign52790_e87077_d_n7, assign52790_e87077_d_n8, assign52790_e87077_d_n9, assign52790_e87077_d_n10, assign52790_e87077_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard807 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard809 == 0.0)) {
        let assign52790_e87071: f64 = (locals.var_pbd_t * locals.var_czbd);
        let assign52790_e87073: f64 = (locals.var_arg).ln();
        let assign52790_e87074: f64 = (-assign52790_e87073);
        let assign52790_e87075: f64 = (assign52790_e87071 * assign52790_e87074);
        (assign52790_e87075, (((locals.var_pbd_t * locals.var_czbd_dn3) * assign52790_e87074) + (assign52790_e87071 * (-(locals.var_arg_dn3 / locals.var_arg)))), ((((locals.var_pbd_t_dn4 * locals.var_czbd) + (locals.var_pbd_t * locals.var_czbd_dn4)) * assign52790_e87074) + (assign52790_e87071 * (-(locals.var_arg_dn4 / locals.var_arg)))), ((((locals.var_pbd_t_dn5 * locals.var_czbd) + (locals.var_pbd_t * locals.var_czbd_dn5)) * assign52790_e87074) + (assign52790_e87071 * (-(locals.var_arg_dn5 / locals.var_arg)))), (((locals.var_pbd_t * locals.var_czbd_dn6) * assign52790_e87074) + (assign52790_e87071 * (-(locals.var_arg_dn6 / locals.var_arg)))), (((locals.var_pbd_t * locals.var_czbd_dn7) * assign52790_e87074) + (assign52790_e87071 * (-(locals.var_arg_dn7 / locals.var_arg)))), (((locals.var_pbd_t * locals.var_czbd_dn8) * assign52790_e87074) + (assign52790_e87071 * (-(locals.var_arg_dn8 / locals.var_arg)))), (((locals.var_pbd_t * locals.var_czbd_dn9) * assign52790_e87074) + (assign52790_e87071 * (-(locals.var_arg_dn9 / locals.var_arg)))), (((locals.var_pbd_t * locals.var_czbd_dn10) * assign52790_e87074) + (assign52790_e87071 * (-(locals.var_arg_dn10 / locals.var_arg)))), (((locals.var_pbd_t * locals.var_czbd_dn11) * assign52790_e87074) + (assign52790_e87071 * (-(locals.var_arg_dn11 / locals.var_arg)))),)
    } else {
        (locals.var_qbdj1, locals.var_qbdj1_dn3, locals.var_qbdj1_dn4, locals.var_qbdj1_dn5, locals.var_qbdj1_dn6, locals.var_qbdj1_dn7, locals.var_qbdj1_dn8, locals.var_qbdj1_dn9, locals.var_qbdj1_dn10, locals.var_qbdj1_dn11,)
    }
};
        locals.var_qbdj1 = assign52790_e87077;
        locals.var_qbdj1_dn3 = assign52790_e87077_d_n3;
        locals.var_qbdj1_dn4 = assign52790_e87077_d_n4;
        locals.var_qbdj1_dn5 = assign52790_e87077_d_n5;
        locals.var_qbdj1_dn6 = assign52790_e87077_d_n6;
        locals.var_qbdj1_dn7 = assign52790_e87077_d_n7;
        locals.var_qbdj1_dn8 = assign52790_e87077_d_n8;
        locals.var_qbdj1_dn9 = assign52790_e87077_d_n9;
        locals.var_qbdj1_dn10 = assign52790_e87077_d_n10;
        locals.var_qbdj1_dn11 = assign52790_e87077_d_n11;

    }

    pub(super) fn stamp_transient_block_179(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign52800_e87103, assign52800_e87103_d_n3, assign52800_e87103_d_n4, assign52800_e87103_d_n5, assign52800_e87103_d_n6, assign52800_e87103_d_n7, assign52800_e87103_d_n8, assign52800_e87103_d_n9, assign52800_e87103_d_n10, assign52800_e87103_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard807 != 0.0)) && (locals.var_guard808 == 0.0)) {
        let assign52800_e87088: f64 = (locals.var_t1 - 1.0);
        let assign52800_e87089: f64 = (locals.var_czbd_p1 * assign52800_e87088);
        let assign52800_e87092: f64 = (5.0 * p.p914);
        let assign52800_e87095: f64 = (locals.var_t1 - 1.0);
        let assign52800_e87096: f64 = (assign52800_e87092 * assign52800_e87095);
        let assign52800_e87099: f64 = (1.0 + p.p914);
        let assign52800_e87100: f64 = (assign52800_e87096 + assign52800_e87099);
        let assign52800_e87101: f64 = (assign52800_e87089 * assign52800_e87100);
        (assign52800_e87101, (((locals.var_czbd_p1 * locals.var_t1_dn3) * assign52800_e87100) + (assign52800_e87089 * (assign52800_e87092 * locals.var_t1_dn3))), (((locals.var_czbd_p1 * locals.var_t1_dn4) * assign52800_e87100) + (assign52800_e87089 * (assign52800_e87092 * locals.var_t1_dn4))), (((locals.var_czbd_p1 * locals.var_t1_dn5) * assign52800_e87100) + (assign52800_e87089 * (assign52800_e87092 * locals.var_t1_dn5))), (((locals.var_czbd_p1 * locals.var_t1_dn6) * assign52800_e87100) + (assign52800_e87089 * (assign52800_e87092 * locals.var_t1_dn6))), (((locals.var_czbd_p1 * locals.var_t1_dn7) * assign52800_e87100) + (assign52800_e87089 * (assign52800_e87092 * locals.var_t1_dn7))), (((locals.var_czbd_p1 * locals.var_t1_dn8) * assign52800_e87100) + (assign52800_e87089 * (assign52800_e87092 * locals.var_t1_dn8))), (((locals.var_czbd_p1 * locals.var_t1_dn9) * assign52800_e87100) + (assign52800_e87089 * (assign52800_e87092 * locals.var_t1_dn9))), (((locals.var_czbd_p1 * locals.var_t1_dn10) * assign52800_e87100) + (assign52800_e87089 * (assign52800_e87092 * locals.var_t1_dn10))), (((locals.var_czbd_p1 * locals.var_t1_dn11) * assign52800_e87100) + (assign52800_e87089 * (assign52800_e87092 * locals.var_t1_dn11))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign52800_e87103;
        locals.var_t2_dn3 = assign52800_e87103_d_n3;
        locals.var_t2_dn4 = assign52800_e87103_d_n4;
        locals.var_t2_dn5 = assign52800_e87103_d_n5;
        locals.var_t2_dn6 = assign52800_e87103_d_n6;
        locals.var_t2_dn7 = assign52800_e87103_d_n7;
        locals.var_t2_dn8 = assign52800_e87103_d_n8;
        locals.var_t2_dn9 = assign52800_e87103_d_n9;
        locals.var_t2_dn10 = assign52800_e87103_d_n10;
        locals.var_t2_dn11 = assign52800_e87103_d_n11;

        let (assign52810_e87119, assign52810_e87119_d_n3, assign52810_e87119_d_n4, assign52810_e87119_d_n5, assign52810_e87119_d_n6, assign52810_e87119_d_n7, assign52810_e87119_d_n8, assign52810_e87119_d_n9, assign52810_e87119_d_n10, assign52810_e87119_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard807 != 0.0)) && (locals.var_guard808 == 0.0)) {
        let assign52810_e87113: f64 = (locals.var_pbd_t * locals.var_czbd);
        let assign52810_e87116: f64 = (locals.var_t2 + locals.var_czbd_p2);
        let assign52810_e87117: f64 = (assign52810_e87113 * assign52810_e87116);
        (assign52810_e87117, (((locals.var_pbd_t * locals.var_czbd_dn3) * assign52810_e87116) + (assign52810_e87113 * locals.var_t2_dn3)), ((((locals.var_pbd_t_dn4 * locals.var_czbd) + (locals.var_pbd_t * locals.var_czbd_dn4)) * assign52810_e87116) + (assign52810_e87113 * locals.var_t2_dn4)), ((((locals.var_pbd_t_dn5 * locals.var_czbd) + (locals.var_pbd_t * locals.var_czbd_dn5)) * assign52810_e87116) + (assign52810_e87113 * locals.var_t2_dn5)), (((locals.var_pbd_t * locals.var_czbd_dn6) * assign52810_e87116) + (assign52810_e87113 * locals.var_t2_dn6)), (((locals.var_pbd_t * locals.var_czbd_dn7) * assign52810_e87116) + (assign52810_e87113 * locals.var_t2_dn7)), (((locals.var_pbd_t * locals.var_czbd_dn8) * assign52810_e87116) + (assign52810_e87113 * locals.var_t2_dn8)), (((locals.var_pbd_t * locals.var_czbd_dn9) * assign52810_e87116) + (assign52810_e87113 * locals.var_t2_dn9)), (((locals.var_pbd_t * locals.var_czbd_dn10) * assign52810_e87116) + (assign52810_e87113 * locals.var_t2_dn10)), (((locals.var_pbd_t * locals.var_czbd_dn11) * assign52810_e87116) + (assign52810_e87113 * locals.var_t2_dn11)),)
    } else {
        (locals.var_qbdj1, locals.var_qbdj1_dn3, locals.var_qbdj1_dn4, locals.var_qbdj1_dn5, locals.var_qbdj1_dn6, locals.var_qbdj1_dn7, locals.var_qbdj1_dn8, locals.var_qbdj1_dn9, locals.var_qbdj1_dn10, locals.var_qbdj1_dn11,)
    }
};
        locals.var_qbdj1 = assign52810_e87119;
        locals.var_qbdj1_dn3 = assign52810_e87119_d_n3;
        locals.var_qbdj1_dn4 = assign52810_e87119_d_n4;
        locals.var_qbdj1_dn5 = assign52810_e87119_d_n5;
        locals.var_qbdj1_dn6 = assign52810_e87119_d_n6;
        locals.var_qbdj1_dn7 = assign52810_e87119_d_n7;
        locals.var_qbdj1_dn8 = assign52810_e87119_d_n8;
        locals.var_qbdj1_dn9 = assign52810_e87119_d_n9;
        locals.var_qbdj1_dn10 = assign52810_e87119_d_n10;
        locals.var_qbdj1_dn11 = assign52810_e87119_d_n11;

        let (assign52820_e87127, assign52820_e87127_d_n3, assign52820_e87127_d_n4, assign52820_e87127_d_n5, assign52820_e87127_d_n6, assign52820_e87127_d_n7, assign52820_e87127_d_n8, assign52820_e87127_d_n9, assign52820_e87127_d_n10, assign52820_e87127_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard807 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbdj1, locals.var_qbdj1_dn3, locals.var_qbdj1_dn4, locals.var_qbdj1_dn5, locals.var_qbdj1_dn6, locals.var_qbdj1_dn7, locals.var_qbdj1_dn8, locals.var_qbdj1_dn9, locals.var_qbdj1_dn10, locals.var_qbdj1_dn11,)
    }
};
        locals.var_qbdj1 = assign52820_e87127;
        locals.var_qbdj1_dn3 = assign52820_e87127_d_n3;
        locals.var_qbdj1_dn4 = assign52820_e87127_d_n4;
        locals.var_qbdj1_dn5 = assign52820_e87127_d_n5;
        locals.var_qbdj1_dn6 = assign52820_e87127_d_n6;
        locals.var_qbdj1_dn7 = assign52820_e87127_d_n7;
        locals.var_qbdj1_dn8 = assign52820_e87127_d_n8;
        locals.var_qbdj1_dn9 = assign52820_e87127_d_n9;
        locals.var_qbdj1_dn10 = assign52820_e87127_d_n10;
        locals.var_qbdj1_dn11 = assign52820_e87127_d_n11;

        let assign52830_e87130: f64 = if locals.var_czbdsw > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard811 = assign52830_e87130;

        let (assign52840_e87139, assign52840_e87139_d_n3, assign52840_e87139_d_n4, assign52840_e87139_d_n5, assign52840_e87139_d_n6, assign52840_e87139_d_n7, assign52840_e87139_d_n8, assign52840_e87139_d_n9, assign52840_e87139_d_n10, assign52840_e87139_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard811 != 0.0)) {
        let assign52840_e87137: f64 = (locals.var_vbd_jct / locals.var_pbswd_t);
        (assign52840_e87137, 0.0, (-((locals.var_vbd_jct * locals.var_pbswd_t_dn4) / (locals.var_pbswd_t * locals.var_pbswd_t))), (-((locals.var_vbd_jct * locals.var_pbswd_t_dn5) / (locals.var_pbswd_t * locals.var_pbswd_t))), (locals.var_vbd_jct_dn6 / locals.var_pbswd_t), 0.0, 0.0, 0.0, (locals.var_vbd_jct_dn10 / locals.var_pbswd_t), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign52840_e87139;
        locals.var_t1_dn3 = assign52840_e87139_d_n3;
        locals.var_t1_dn4 = assign52840_e87139_d_n4;
        locals.var_t1_dn5 = assign52840_e87139_d_n5;
        locals.var_t1_dn6 = assign52840_e87139_d_n6;
        locals.var_t1_dn7 = assign52840_e87139_d_n7;
        locals.var_t1_dn8 = assign52840_e87139_d_n8;
        locals.var_t1_dn9 = assign52840_e87139_d_n9;
        locals.var_t1_dn10 = assign52840_e87139_d_n10;
        locals.var_t1_dn11 = assign52840_e87139_d_n11;

        let assign52850_e87142: f64 = if locals.var_t1 < 0.9 { 1.0 } else { 0.0 };
        locals.var_guard812 = assign52850_e87142;

        let (assign52860_e87153, assign52860_e87153_d_n3, assign52860_e87153_d_n4, assign52860_e87153_d_n5, assign52860_e87153_d_n6, assign52860_e87153_d_n7, assign52860_e87153_d_n8, assign52860_e87153_d_n9, assign52860_e87153_d_n10, assign52860_e87153_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard811 != 0.0)) && (locals.var_guard812 != 0.0)) {
        let assign52860_e87151: f64 = (1.0 - locals.var_t1);
        (assign52860_e87151, (-locals.var_t1_dn3), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn11),)
    } else {
        (locals.var_arg, locals.var_arg_dn3, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11,)
    }
};
        locals.var_arg = assign52860_e87153;
        locals.var_arg_dn3 = assign52860_e87153_d_n3;
        locals.var_arg_dn4 = assign52860_e87153_d_n4;
        locals.var_arg_dn5 = assign52860_e87153_d_n5;
        locals.var_arg_dn6 = assign52860_e87153_d_n6;
        locals.var_arg_dn7 = assign52860_e87153_d_n7;
        locals.var_arg_dn8 = assign52860_e87153_d_n8;
        locals.var_arg_dn9 = assign52860_e87153_d_n9;
        locals.var_arg_dn10 = assign52860_e87153_d_n10;
        locals.var_arg_dn11 = assign52860_e87153_d_n11;

        let assign52870_e87156: f64 = if p.p916 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard813 = assign52870_e87156;

        let assign52880_e87159: f64 = if p.p916 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard814 = assign52880_e87159;

        let (assign52890_e87175, assign52890_e87175_d_n3, assign52890_e87175_d_n4, assign52890_e87175_d_n5, assign52890_e87175_d_n6, assign52890_e87175_d_n7, assign52890_e87175_d_n8, assign52890_e87175_d_n9, assign52890_e87175_d_n10, assign52890_e87175_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard811 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard813 != 0.0)) && (locals.var_guard814 != 0.0)) {
        let assign52890_e87172: f64 = (locals.var_arg).sqrt();
        let assign52890_e87173: f64 = (1.0 / assign52890_e87172);
        (assign52890_e87173, (-((locals.var_arg_dn3 / (2.0 * assign52890_e87172)) / (assign52890_e87172 * assign52890_e87172))), (-((locals.var_arg_dn4 / (2.0 * assign52890_e87172)) / (assign52890_e87172 * assign52890_e87172))), (-((locals.var_arg_dn5 / (2.0 * assign52890_e87172)) / (assign52890_e87172 * assign52890_e87172))), (-((locals.var_arg_dn6 / (2.0 * assign52890_e87172)) / (assign52890_e87172 * assign52890_e87172))), (-((locals.var_arg_dn7 / (2.0 * assign52890_e87172)) / (assign52890_e87172 * assign52890_e87172))), (-((locals.var_arg_dn8 / (2.0 * assign52890_e87172)) / (assign52890_e87172 * assign52890_e87172))), (-((locals.var_arg_dn9 / (2.0 * assign52890_e87172)) / (assign52890_e87172 * assign52890_e87172))), (-((locals.var_arg_dn10 / (2.0 * assign52890_e87172)) / (assign52890_e87172 * assign52890_e87172))), (-((locals.var_arg_dn11 / (2.0 * assign52890_e87172)) / (assign52890_e87172 * assign52890_e87172))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11,)
    }
};
        locals.var_sarg = assign52890_e87175;
        locals.var_sarg_dn3 = assign52890_e87175_d_n3;
        locals.var_sarg_dn4 = assign52890_e87175_d_n4;
        locals.var_sarg_dn5 = assign52890_e87175_d_n5;
        locals.var_sarg_dn6 = assign52890_e87175_d_n6;
        locals.var_sarg_dn7 = assign52890_e87175_d_n7;
        locals.var_sarg_dn8 = assign52890_e87175_d_n8;
        locals.var_sarg_dn9 = assign52890_e87175_d_n9;
        locals.var_sarg_dn10 = assign52890_e87175_d_n10;
        locals.var_sarg_dn11 = assign52890_e87175_d_n11;

        let (assign52900_e87194, assign52900_e87194_d_n3, assign52900_e87194_d_n4, assign52900_e87194_d_n5, assign52900_e87194_d_n6, assign52900_e87194_d_n7, assign52900_e87194_d_n8, assign52900_e87194_d_n9, assign52900_e87194_d_n10, assign52900_e87194_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard811 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard813 != 0.0)) && (locals.var_guard814 == 0.0)) {
        let assign52900_e87188: f64 = (-p.p916);
        let assign52900_e87190: f64 = (locals.var_arg).ln();
        let assign52900_e87191: f64 = (assign52900_e87188 * assign52900_e87190);
        let assign52900_e87192: f64 = { let limited_exp_arg = assign52900_e87191; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign52900_e87192, ({ let limited_exp_arg = assign52900_e87191; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52900_e87188 * (locals.var_arg_dn3 / locals.var_arg))), ({ let limited_exp_arg = assign52900_e87191; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52900_e87188 * (locals.var_arg_dn4 / locals.var_arg))), ({ let limited_exp_arg = assign52900_e87191; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52900_e87188 * (locals.var_arg_dn5 / locals.var_arg))), ({ let limited_exp_arg = assign52900_e87191; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52900_e87188 * (locals.var_arg_dn6 / locals.var_arg))), ({ let limited_exp_arg = assign52900_e87191; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52900_e87188 * (locals.var_arg_dn7 / locals.var_arg))), ({ let limited_exp_arg = assign52900_e87191; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52900_e87188 * (locals.var_arg_dn8 / locals.var_arg))), ({ let limited_exp_arg = assign52900_e87191; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52900_e87188 * (locals.var_arg_dn9 / locals.var_arg))), ({ let limited_exp_arg = assign52900_e87191; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52900_e87188 * (locals.var_arg_dn10 / locals.var_arg))), ({ let limited_exp_arg = assign52900_e87191; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52900_e87188 * (locals.var_arg_dn11 / locals.var_arg))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11,)
    }
};
        locals.var_sarg = assign52900_e87194;
        locals.var_sarg_dn3 = assign52900_e87194_d_n3;
        locals.var_sarg_dn4 = assign52900_e87194_d_n4;
        locals.var_sarg_dn5 = assign52900_e87194_d_n5;
        locals.var_sarg_dn6 = assign52900_e87194_d_n6;
        locals.var_sarg_dn7 = assign52900_e87194_d_n7;
        locals.var_sarg_dn8 = assign52900_e87194_d_n8;
        locals.var_sarg_dn9 = assign52900_e87194_d_n9;
        locals.var_sarg_dn10 = assign52900_e87194_d_n10;
        locals.var_sarg_dn11 = assign52900_e87194_d_n11;

        let (assign52910_e87217, assign52910_e87217_d_n3, assign52910_e87217_d_n4, assign52910_e87217_d_n5, assign52910_e87217_d_n6, assign52910_e87217_d_n7, assign52910_e87217_d_n8, assign52910_e87217_d_n9, assign52910_e87217_d_n10, assign52910_e87217_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard811 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard813 != 0.0)) {
        let assign52910_e87205: f64 = (locals.var_pbswd_t * locals.var_czbdsw);
        let assign52910_e87209: f64 = (locals.var_arg * locals.var_sarg);
        let assign52910_e87210: f64 = (1.0 - assign52910_e87209);
        let assign52910_e87211: f64 = (assign52910_e87205 * assign52910_e87210);
        let assign52910_e87214: f64 = (1.0 - p.p916);
        let assign52910_e87215: f64 = (assign52910_e87211 / assign52910_e87214);
        (assign52910_e87215, ((((locals.var_pbswd_t * locals.var_czbdsw_dn3) * assign52910_e87210) + (assign52910_e87205 * (-((locals.var_arg_dn3 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn3))))) / assign52910_e87214), (((((locals.var_pbswd_t_dn4 * locals.var_czbdsw) + (locals.var_pbswd_t * locals.var_czbdsw_dn4)) * assign52910_e87210) + (assign52910_e87205 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign52910_e87214), (((((locals.var_pbswd_t_dn5 * locals.var_czbdsw) + (locals.var_pbswd_t * locals.var_czbdsw_dn5)) * assign52910_e87210) + (assign52910_e87205 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign52910_e87214), ((((locals.var_pbswd_t * locals.var_czbdsw_dn6) * assign52910_e87210) + (assign52910_e87205 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign52910_e87214), ((((locals.var_pbswd_t * locals.var_czbdsw_dn7) * assign52910_e87210) + (assign52910_e87205 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign52910_e87214), ((((locals.var_pbswd_t * locals.var_czbdsw_dn8) * assign52910_e87210) + (assign52910_e87205 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign52910_e87214), ((((locals.var_pbswd_t * locals.var_czbdsw_dn9) * assign52910_e87210) + (assign52910_e87205 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign52910_e87214), ((((locals.var_pbswd_t * locals.var_czbdsw_dn10) * assign52910_e87210) + (assign52910_e87205 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign52910_e87214), ((((locals.var_pbswd_t * locals.var_czbdsw_dn11) * assign52910_e87210) + (assign52910_e87205 * (-((locals.var_arg_dn11 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn11))))) / assign52910_e87214),)
    } else {
        (locals.var_qbdj2, locals.var_qbdj2_dn3, locals.var_qbdj2_dn4, locals.var_qbdj2_dn5, locals.var_qbdj2_dn6, locals.var_qbdj2_dn7, locals.var_qbdj2_dn8, locals.var_qbdj2_dn9, locals.var_qbdj2_dn10, locals.var_qbdj2_dn11,)
    }
};
        locals.var_qbdj2 = assign52910_e87217;
        locals.var_qbdj2_dn3 = assign52910_e87217_d_n3;
        locals.var_qbdj2_dn4 = assign52910_e87217_d_n4;
        locals.var_qbdj2_dn5 = assign52910_e87217_d_n5;
        locals.var_qbdj2_dn6 = assign52910_e87217_d_n6;
        locals.var_qbdj2_dn7 = assign52910_e87217_d_n7;
        locals.var_qbdj2_dn8 = assign52910_e87217_d_n8;
        locals.var_qbdj2_dn9 = assign52910_e87217_d_n9;
        locals.var_qbdj2_dn10 = assign52910_e87217_d_n10;
        locals.var_qbdj2_dn11 = assign52910_e87217_d_n11;

        let (assign52920_e87235, assign52920_e87235_d_n3, assign52920_e87235_d_n4, assign52920_e87235_d_n5, assign52920_e87235_d_n6, assign52920_e87235_d_n7, assign52920_e87235_d_n8, assign52920_e87235_d_n9, assign52920_e87235_d_n10, assign52920_e87235_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard811 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard813 == 0.0)) {
        let assign52920_e87229: f64 = (locals.var_pbswd_t * locals.var_czbdsw);
        let assign52920_e87231: f64 = (locals.var_arg).ln();
        let assign52920_e87232: f64 = (-assign52920_e87231);
        let assign52920_e87233: f64 = (assign52920_e87229 * assign52920_e87232);
        (assign52920_e87233, (((locals.var_pbswd_t * locals.var_czbdsw_dn3) * assign52920_e87232) + (assign52920_e87229 * (-(locals.var_arg_dn3 / locals.var_arg)))), ((((locals.var_pbswd_t_dn4 * locals.var_czbdsw) + (locals.var_pbswd_t * locals.var_czbdsw_dn4)) * assign52920_e87232) + (assign52920_e87229 * (-(locals.var_arg_dn4 / locals.var_arg)))), ((((locals.var_pbswd_t_dn5 * locals.var_czbdsw) + (locals.var_pbswd_t * locals.var_czbdsw_dn5)) * assign52920_e87232) + (assign52920_e87229 * (-(locals.var_arg_dn5 / locals.var_arg)))), (((locals.var_pbswd_t * locals.var_czbdsw_dn6) * assign52920_e87232) + (assign52920_e87229 * (-(locals.var_arg_dn6 / locals.var_arg)))), (((locals.var_pbswd_t * locals.var_czbdsw_dn7) * assign52920_e87232) + (assign52920_e87229 * (-(locals.var_arg_dn7 / locals.var_arg)))), (((locals.var_pbswd_t * locals.var_czbdsw_dn8) * assign52920_e87232) + (assign52920_e87229 * (-(locals.var_arg_dn8 / locals.var_arg)))), (((locals.var_pbswd_t * locals.var_czbdsw_dn9) * assign52920_e87232) + (assign52920_e87229 * (-(locals.var_arg_dn9 / locals.var_arg)))), (((locals.var_pbswd_t * locals.var_czbdsw_dn10) * assign52920_e87232) + (assign52920_e87229 * (-(locals.var_arg_dn10 / locals.var_arg)))), (((locals.var_pbswd_t * locals.var_czbdsw_dn11) * assign52920_e87232) + (assign52920_e87229 * (-(locals.var_arg_dn11 / locals.var_arg)))),)
    } else {
        (locals.var_qbdj2, locals.var_qbdj2_dn3, locals.var_qbdj2_dn4, locals.var_qbdj2_dn5, locals.var_qbdj2_dn6, locals.var_qbdj2_dn7, locals.var_qbdj2_dn8, locals.var_qbdj2_dn9, locals.var_qbdj2_dn10, locals.var_qbdj2_dn11,)
    }
};
        locals.var_qbdj2 = assign52920_e87235;
        locals.var_qbdj2_dn3 = assign52920_e87235_d_n3;
        locals.var_qbdj2_dn4 = assign52920_e87235_d_n4;
        locals.var_qbdj2_dn5 = assign52920_e87235_d_n5;
        locals.var_qbdj2_dn6 = assign52920_e87235_d_n6;
        locals.var_qbdj2_dn7 = assign52920_e87235_d_n7;
        locals.var_qbdj2_dn8 = assign52920_e87235_d_n8;
        locals.var_qbdj2_dn9 = assign52920_e87235_d_n9;
        locals.var_qbdj2_dn10 = assign52920_e87235_d_n10;
        locals.var_qbdj2_dn11 = assign52920_e87235_d_n11;

        let (assign52930_e87261, assign52930_e87261_d_n3, assign52930_e87261_d_n4, assign52930_e87261_d_n5, assign52930_e87261_d_n6, assign52930_e87261_d_n7, assign52930_e87261_d_n8, assign52930_e87261_d_n9, assign52930_e87261_d_n10, assign52930_e87261_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard811 != 0.0)) && (locals.var_guard812 == 0.0)) {
        let assign52930_e87246: f64 = (locals.var_t1 - 1.0);
        let assign52930_e87247: f64 = (locals.var_czbdsw_p1 * assign52930_e87246);
        let assign52930_e87250: f64 = (5.0 * p.p916);
        let assign52930_e87253: f64 = (locals.var_t1 - 1.0);
        let assign52930_e87254: f64 = (assign52930_e87250 * assign52930_e87253);
        let assign52930_e87257: f64 = (1.0 + p.p916);
        let assign52930_e87258: f64 = (assign52930_e87254 + assign52930_e87257);
        let assign52930_e87259: f64 = (assign52930_e87247 * assign52930_e87258);
        (assign52930_e87259, (((locals.var_czbdsw_p1 * locals.var_t1_dn3) * assign52930_e87258) + (assign52930_e87247 * (assign52930_e87250 * locals.var_t1_dn3))), (((locals.var_czbdsw_p1 * locals.var_t1_dn4) * assign52930_e87258) + (assign52930_e87247 * (assign52930_e87250 * locals.var_t1_dn4))), (((locals.var_czbdsw_p1 * locals.var_t1_dn5) * assign52930_e87258) + (assign52930_e87247 * (assign52930_e87250 * locals.var_t1_dn5))), (((locals.var_czbdsw_p1 * locals.var_t1_dn6) * assign52930_e87258) + (assign52930_e87247 * (assign52930_e87250 * locals.var_t1_dn6))), (((locals.var_czbdsw_p1 * locals.var_t1_dn7) * assign52930_e87258) + (assign52930_e87247 * (assign52930_e87250 * locals.var_t1_dn7))), (((locals.var_czbdsw_p1 * locals.var_t1_dn8) * assign52930_e87258) + (assign52930_e87247 * (assign52930_e87250 * locals.var_t1_dn8))), (((locals.var_czbdsw_p1 * locals.var_t1_dn9) * assign52930_e87258) + (assign52930_e87247 * (assign52930_e87250 * locals.var_t1_dn9))), (((locals.var_czbdsw_p1 * locals.var_t1_dn10) * assign52930_e87258) + (assign52930_e87247 * (assign52930_e87250 * locals.var_t1_dn10))), (((locals.var_czbdsw_p1 * locals.var_t1_dn11) * assign52930_e87258) + (assign52930_e87247 * (assign52930_e87250 * locals.var_t1_dn11))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign52930_e87261;
        locals.var_t2_dn3 = assign52930_e87261_d_n3;
        locals.var_t2_dn4 = assign52930_e87261_d_n4;
        locals.var_t2_dn5 = assign52930_e87261_d_n5;
        locals.var_t2_dn6 = assign52930_e87261_d_n6;
        locals.var_t2_dn7 = assign52930_e87261_d_n7;
        locals.var_t2_dn8 = assign52930_e87261_d_n8;
        locals.var_t2_dn9 = assign52930_e87261_d_n9;
        locals.var_t2_dn10 = assign52930_e87261_d_n10;
        locals.var_t2_dn11 = assign52930_e87261_d_n11;

        let (assign52940_e87277, assign52940_e87277_d_n3, assign52940_e87277_d_n4, assign52940_e87277_d_n5, assign52940_e87277_d_n6, assign52940_e87277_d_n7, assign52940_e87277_d_n8, assign52940_e87277_d_n9, assign52940_e87277_d_n10, assign52940_e87277_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard811 != 0.0)) && (locals.var_guard812 == 0.0)) {
        let assign52940_e87271: f64 = (locals.var_pbswd_t * locals.var_czbdsw);
        let assign52940_e87274: f64 = (locals.var_t2 + locals.var_czbdsw_p2);
        let assign52940_e87275: f64 = (assign52940_e87271 * assign52940_e87274);
        (assign52940_e87275, (((locals.var_pbswd_t * locals.var_czbdsw_dn3) * assign52940_e87274) + (assign52940_e87271 * locals.var_t2_dn3)), ((((locals.var_pbswd_t_dn4 * locals.var_czbdsw) + (locals.var_pbswd_t * locals.var_czbdsw_dn4)) * assign52940_e87274) + (assign52940_e87271 * locals.var_t2_dn4)), ((((locals.var_pbswd_t_dn5 * locals.var_czbdsw) + (locals.var_pbswd_t * locals.var_czbdsw_dn5)) * assign52940_e87274) + (assign52940_e87271 * locals.var_t2_dn5)), (((locals.var_pbswd_t * locals.var_czbdsw_dn6) * assign52940_e87274) + (assign52940_e87271 * locals.var_t2_dn6)), (((locals.var_pbswd_t * locals.var_czbdsw_dn7) * assign52940_e87274) + (assign52940_e87271 * locals.var_t2_dn7)), (((locals.var_pbswd_t * locals.var_czbdsw_dn8) * assign52940_e87274) + (assign52940_e87271 * locals.var_t2_dn8)), (((locals.var_pbswd_t * locals.var_czbdsw_dn9) * assign52940_e87274) + (assign52940_e87271 * locals.var_t2_dn9)), (((locals.var_pbswd_t * locals.var_czbdsw_dn10) * assign52940_e87274) + (assign52940_e87271 * locals.var_t2_dn10)), (((locals.var_pbswd_t * locals.var_czbdsw_dn11) * assign52940_e87274) + (assign52940_e87271 * locals.var_t2_dn11)),)
    } else {
        (locals.var_qbdj2, locals.var_qbdj2_dn3, locals.var_qbdj2_dn4, locals.var_qbdj2_dn5, locals.var_qbdj2_dn6, locals.var_qbdj2_dn7, locals.var_qbdj2_dn8, locals.var_qbdj2_dn9, locals.var_qbdj2_dn10, locals.var_qbdj2_dn11,)
    }
};
        locals.var_qbdj2 = assign52940_e87277;
        locals.var_qbdj2_dn3 = assign52940_e87277_d_n3;
        locals.var_qbdj2_dn4 = assign52940_e87277_d_n4;
        locals.var_qbdj2_dn5 = assign52940_e87277_d_n5;
        locals.var_qbdj2_dn6 = assign52940_e87277_d_n6;
        locals.var_qbdj2_dn7 = assign52940_e87277_d_n7;
        locals.var_qbdj2_dn8 = assign52940_e87277_d_n8;
        locals.var_qbdj2_dn9 = assign52940_e87277_d_n9;
        locals.var_qbdj2_dn10 = assign52940_e87277_d_n10;
        locals.var_qbdj2_dn11 = assign52940_e87277_d_n11;

        let (assign52950_e87285, assign52950_e87285_d_n3, assign52950_e87285_d_n4, assign52950_e87285_d_n5, assign52950_e87285_d_n6, assign52950_e87285_d_n7, assign52950_e87285_d_n8, assign52950_e87285_d_n9, assign52950_e87285_d_n10, assign52950_e87285_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard811 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbdj2, locals.var_qbdj2_dn3, locals.var_qbdj2_dn4, locals.var_qbdj2_dn5, locals.var_qbdj2_dn6, locals.var_qbdj2_dn7, locals.var_qbdj2_dn8, locals.var_qbdj2_dn9, locals.var_qbdj2_dn10, locals.var_qbdj2_dn11,)
    }
};
        locals.var_qbdj2 = assign52950_e87285;
        locals.var_qbdj2_dn3 = assign52950_e87285_d_n3;
        locals.var_qbdj2_dn4 = assign52950_e87285_d_n4;
        locals.var_qbdj2_dn5 = assign52950_e87285_d_n5;
        locals.var_qbdj2_dn6 = assign52950_e87285_d_n6;
        locals.var_qbdj2_dn7 = assign52950_e87285_d_n7;
        locals.var_qbdj2_dn8 = assign52950_e87285_d_n8;
        locals.var_qbdj2_dn9 = assign52950_e87285_d_n9;
        locals.var_qbdj2_dn10 = assign52950_e87285_d_n10;
        locals.var_qbdj2_dn11 = assign52950_e87285_d_n11;

        let assign52960_e87288: f64 = if locals.var_czbdswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard815 = assign52960_e87288;

        let (assign52970_e87297, assign52970_e87297_d_n3, assign52970_e87297_d_n4, assign52970_e87297_d_n5, assign52970_e87297_d_n6, assign52970_e87297_d_n7, assign52970_e87297_d_n8, assign52970_e87297_d_n9, assign52970_e87297_d_n10, assign52970_e87297_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard815 != 0.0)) {
        let assign52970_e87295: f64 = (locals.var_vbd_jct / locals.var_pbswgd_t);
        (assign52970_e87295, 0.0, (-((locals.var_vbd_jct * locals.var_pbswgd_t_dn4) / (locals.var_pbswgd_t * locals.var_pbswgd_t))), (-((locals.var_vbd_jct * locals.var_pbswgd_t_dn5) / (locals.var_pbswgd_t * locals.var_pbswgd_t))), (locals.var_vbd_jct_dn6 / locals.var_pbswgd_t), 0.0, 0.0, 0.0, (locals.var_vbd_jct_dn10 / locals.var_pbswgd_t), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign52970_e87297;
        locals.var_t1_dn3 = assign52970_e87297_d_n3;
        locals.var_t1_dn4 = assign52970_e87297_d_n4;
        locals.var_t1_dn5 = assign52970_e87297_d_n5;
        locals.var_t1_dn6 = assign52970_e87297_d_n6;
        locals.var_t1_dn7 = assign52970_e87297_d_n7;
        locals.var_t1_dn8 = assign52970_e87297_d_n8;
        locals.var_t1_dn9 = assign52970_e87297_d_n9;
        locals.var_t1_dn10 = assign52970_e87297_d_n10;
        locals.var_t1_dn11 = assign52970_e87297_d_n11;

        let assign52980_e87300: f64 = if locals.var_t1 < 0.9 { 1.0 } else { 0.0 };
        locals.var_guard816 = assign52980_e87300;

        let (assign52990_e87311, assign52990_e87311_d_n3, assign52990_e87311_d_n4, assign52990_e87311_d_n5, assign52990_e87311_d_n6, assign52990_e87311_d_n7, assign52990_e87311_d_n8, assign52990_e87311_d_n9, assign52990_e87311_d_n10, assign52990_e87311_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard815 != 0.0)) && (locals.var_guard816 != 0.0)) {
        let assign52990_e87309: f64 = (1.0 - locals.var_t1);
        (assign52990_e87309, (-locals.var_t1_dn3), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn11),)
    } else {
        (locals.var_arg, locals.var_arg_dn3, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11,)
    }
};
        locals.var_arg = assign52990_e87311;
        locals.var_arg_dn3 = assign52990_e87311_d_n3;
        locals.var_arg_dn4 = assign52990_e87311_d_n4;
        locals.var_arg_dn5 = assign52990_e87311_d_n5;
        locals.var_arg_dn6 = assign52990_e87311_d_n6;
        locals.var_arg_dn7 = assign52990_e87311_d_n7;
        locals.var_arg_dn8 = assign52990_e87311_d_n8;
        locals.var_arg_dn9 = assign52990_e87311_d_n9;
        locals.var_arg_dn10 = assign52990_e87311_d_n10;
        locals.var_arg_dn11 = assign52990_e87311_d_n11;

        let assign53000_e87314: f64 = if p.p918 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard817 = assign53000_e87314;

        let assign53010_e87317: f64 = if p.p918 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard818 = assign53010_e87317;

        let (assign53020_e87333, assign53020_e87333_d_n3, assign53020_e87333_d_n4, assign53020_e87333_d_n5, assign53020_e87333_d_n6, assign53020_e87333_d_n7, assign53020_e87333_d_n8, assign53020_e87333_d_n9, assign53020_e87333_d_n10, assign53020_e87333_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard815 != 0.0)) && (locals.var_guard816 != 0.0)) && (locals.var_guard817 != 0.0)) && (locals.var_guard818 != 0.0)) {
        let assign53020_e87330: f64 = (locals.var_arg).sqrt();
        let assign53020_e87331: f64 = (1.0 / assign53020_e87330);
        (assign53020_e87331, (-((locals.var_arg_dn3 / (2.0 * assign53020_e87330)) / (assign53020_e87330 * assign53020_e87330))), (-((locals.var_arg_dn4 / (2.0 * assign53020_e87330)) / (assign53020_e87330 * assign53020_e87330))), (-((locals.var_arg_dn5 / (2.0 * assign53020_e87330)) / (assign53020_e87330 * assign53020_e87330))), (-((locals.var_arg_dn6 / (2.0 * assign53020_e87330)) / (assign53020_e87330 * assign53020_e87330))), (-((locals.var_arg_dn7 / (2.0 * assign53020_e87330)) / (assign53020_e87330 * assign53020_e87330))), (-((locals.var_arg_dn8 / (2.0 * assign53020_e87330)) / (assign53020_e87330 * assign53020_e87330))), (-((locals.var_arg_dn9 / (2.0 * assign53020_e87330)) / (assign53020_e87330 * assign53020_e87330))), (-((locals.var_arg_dn10 / (2.0 * assign53020_e87330)) / (assign53020_e87330 * assign53020_e87330))), (-((locals.var_arg_dn11 / (2.0 * assign53020_e87330)) / (assign53020_e87330 * assign53020_e87330))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11,)
    }
};
        locals.var_sarg = assign53020_e87333;
        locals.var_sarg_dn3 = assign53020_e87333_d_n3;
        locals.var_sarg_dn4 = assign53020_e87333_d_n4;
        locals.var_sarg_dn5 = assign53020_e87333_d_n5;
        locals.var_sarg_dn6 = assign53020_e87333_d_n6;
        locals.var_sarg_dn7 = assign53020_e87333_d_n7;
        locals.var_sarg_dn8 = assign53020_e87333_d_n8;
        locals.var_sarg_dn9 = assign53020_e87333_d_n9;
        locals.var_sarg_dn10 = assign53020_e87333_d_n10;
        locals.var_sarg_dn11 = assign53020_e87333_d_n11;

        let (assign53030_e87352, assign53030_e87352_d_n3, assign53030_e87352_d_n4, assign53030_e87352_d_n5, assign53030_e87352_d_n6, assign53030_e87352_d_n7, assign53030_e87352_d_n8, assign53030_e87352_d_n9, assign53030_e87352_d_n10, assign53030_e87352_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard815 != 0.0)) && (locals.var_guard816 != 0.0)) && (locals.var_guard817 != 0.0)) && (locals.var_guard818 == 0.0)) {
        let assign53030_e87346: f64 = (-p.p918);
        let assign53030_e87348: f64 = (locals.var_arg).ln();
        let assign53030_e87349: f64 = (assign53030_e87346 * assign53030_e87348);
        let assign53030_e87350: f64 = { let limited_exp_arg = assign53030_e87349; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign53030_e87350, ({ let limited_exp_arg = assign53030_e87349; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign53030_e87346 * (locals.var_arg_dn3 / locals.var_arg))), ({ let limited_exp_arg = assign53030_e87349; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign53030_e87346 * (locals.var_arg_dn4 / locals.var_arg))), ({ let limited_exp_arg = assign53030_e87349; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign53030_e87346 * (locals.var_arg_dn5 / locals.var_arg))), ({ let limited_exp_arg = assign53030_e87349; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign53030_e87346 * (locals.var_arg_dn6 / locals.var_arg))), ({ let limited_exp_arg = assign53030_e87349; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign53030_e87346 * (locals.var_arg_dn7 / locals.var_arg))), ({ let limited_exp_arg = assign53030_e87349; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign53030_e87346 * (locals.var_arg_dn8 / locals.var_arg))), ({ let limited_exp_arg = assign53030_e87349; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign53030_e87346 * (locals.var_arg_dn9 / locals.var_arg))), ({ let limited_exp_arg = assign53030_e87349; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign53030_e87346 * (locals.var_arg_dn10 / locals.var_arg))), ({ let limited_exp_arg = assign53030_e87349; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign53030_e87346 * (locals.var_arg_dn11 / locals.var_arg))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11,)
    }
};
        locals.var_sarg = assign53030_e87352;
        locals.var_sarg_dn3 = assign53030_e87352_d_n3;
        locals.var_sarg_dn4 = assign53030_e87352_d_n4;
        locals.var_sarg_dn5 = assign53030_e87352_d_n5;
        locals.var_sarg_dn6 = assign53030_e87352_d_n6;
        locals.var_sarg_dn7 = assign53030_e87352_d_n7;
        locals.var_sarg_dn8 = assign53030_e87352_d_n8;
        locals.var_sarg_dn9 = assign53030_e87352_d_n9;
        locals.var_sarg_dn10 = assign53030_e87352_d_n10;
        locals.var_sarg_dn11 = assign53030_e87352_d_n11;

        let (assign53040_e87375, assign53040_e87375_d_n3, assign53040_e87375_d_n4, assign53040_e87375_d_n5, assign53040_e87375_d_n6, assign53040_e87375_d_n7, assign53040_e87375_d_n8, assign53040_e87375_d_n9, assign53040_e87375_d_n10, assign53040_e87375_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard815 != 0.0)) && (locals.var_guard816 != 0.0)) && (locals.var_guard817 != 0.0)) {
        let assign53040_e87363: f64 = (locals.var_pbswgd_t * locals.var_czbdswg);
        let assign53040_e87367: f64 = (locals.var_arg * locals.var_sarg);
        let assign53040_e87368: f64 = (1.0 - assign53040_e87367);
        let assign53040_e87369: f64 = (assign53040_e87363 * assign53040_e87368);
        let assign53040_e87372: f64 = (1.0 - p.p918);
        let assign53040_e87373: f64 = (assign53040_e87369 / assign53040_e87372);
        (assign53040_e87373, ((assign53040_e87363 * (-((locals.var_arg_dn3 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn3)))) / assign53040_e87372), (((((locals.var_pbswgd_t_dn4 * locals.var_czbdswg) + (locals.var_pbswgd_t * locals.var_czbdswg_dn4)) * assign53040_e87368) + (assign53040_e87363 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign53040_e87372), (((((locals.var_pbswgd_t_dn5 * locals.var_czbdswg) + (locals.var_pbswgd_t * locals.var_czbdswg_dn5)) * assign53040_e87368) + (assign53040_e87363 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign53040_e87372), ((assign53040_e87363 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6)))) / assign53040_e87372), ((assign53040_e87363 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7)))) / assign53040_e87372), ((assign53040_e87363 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8)))) / assign53040_e87372), ((assign53040_e87363 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9)))) / assign53040_e87372), ((assign53040_e87363 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10)))) / assign53040_e87372), ((assign53040_e87363 * (-((locals.var_arg_dn11 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn11)))) / assign53040_e87372),)
    } else {
        (locals.var_qbdj3, locals.var_qbdj3_dn3, locals.var_qbdj3_dn4, locals.var_qbdj3_dn5, locals.var_qbdj3_dn6, locals.var_qbdj3_dn7, locals.var_qbdj3_dn8, locals.var_qbdj3_dn9, locals.var_qbdj3_dn10, locals.var_qbdj3_dn11,)
    }
};
        locals.var_qbdj3 = assign53040_e87375;
        locals.var_qbdj3_dn3 = assign53040_e87375_d_n3;
        locals.var_qbdj3_dn4 = assign53040_e87375_d_n4;
        locals.var_qbdj3_dn5 = assign53040_e87375_d_n5;
        locals.var_qbdj3_dn6 = assign53040_e87375_d_n6;
        locals.var_qbdj3_dn7 = assign53040_e87375_d_n7;
        locals.var_qbdj3_dn8 = assign53040_e87375_d_n8;
        locals.var_qbdj3_dn9 = assign53040_e87375_d_n9;
        locals.var_qbdj3_dn10 = assign53040_e87375_d_n10;
        locals.var_qbdj3_dn11 = assign53040_e87375_d_n11;

        let (assign53050_e87393, assign53050_e87393_d_n3, assign53050_e87393_d_n4, assign53050_e87393_d_n5, assign53050_e87393_d_n6, assign53050_e87393_d_n7, assign53050_e87393_d_n8, assign53050_e87393_d_n9, assign53050_e87393_d_n10, assign53050_e87393_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard815 != 0.0)) && (locals.var_guard816 != 0.0)) && (locals.var_guard817 == 0.0)) {
        let assign53050_e87387: f64 = (locals.var_pbswgd_t * locals.var_czbdswg);
        let assign53050_e87389: f64 = (locals.var_arg).ln();
        let assign53050_e87390: f64 = (-assign53050_e87389);
        let assign53050_e87391: f64 = (assign53050_e87387 * assign53050_e87390);
        (assign53050_e87391, (assign53050_e87387 * (-(locals.var_arg_dn3 / locals.var_arg))), ((((locals.var_pbswgd_t_dn4 * locals.var_czbdswg) + (locals.var_pbswgd_t * locals.var_czbdswg_dn4)) * assign53050_e87390) + (assign53050_e87387 * (-(locals.var_arg_dn4 / locals.var_arg)))), ((((locals.var_pbswgd_t_dn5 * locals.var_czbdswg) + (locals.var_pbswgd_t * locals.var_czbdswg_dn5)) * assign53050_e87390) + (assign53050_e87387 * (-(locals.var_arg_dn5 / locals.var_arg)))), (assign53050_e87387 * (-(locals.var_arg_dn6 / locals.var_arg))), (assign53050_e87387 * (-(locals.var_arg_dn7 / locals.var_arg))), (assign53050_e87387 * (-(locals.var_arg_dn8 / locals.var_arg))), (assign53050_e87387 * (-(locals.var_arg_dn9 / locals.var_arg))), (assign53050_e87387 * (-(locals.var_arg_dn10 / locals.var_arg))), (assign53050_e87387 * (-(locals.var_arg_dn11 / locals.var_arg))),)
    } else {
        (locals.var_qbdj3, locals.var_qbdj3_dn3, locals.var_qbdj3_dn4, locals.var_qbdj3_dn5, locals.var_qbdj3_dn6, locals.var_qbdj3_dn7, locals.var_qbdj3_dn8, locals.var_qbdj3_dn9, locals.var_qbdj3_dn10, locals.var_qbdj3_dn11,)
    }
};
        locals.var_qbdj3 = assign53050_e87393;
        locals.var_qbdj3_dn3 = assign53050_e87393_d_n3;
        locals.var_qbdj3_dn4 = assign53050_e87393_d_n4;
        locals.var_qbdj3_dn5 = assign53050_e87393_d_n5;
        locals.var_qbdj3_dn6 = assign53050_e87393_d_n6;
        locals.var_qbdj3_dn7 = assign53050_e87393_d_n7;
        locals.var_qbdj3_dn8 = assign53050_e87393_d_n8;
        locals.var_qbdj3_dn9 = assign53050_e87393_d_n9;
        locals.var_qbdj3_dn10 = assign53050_e87393_d_n10;
        locals.var_qbdj3_dn11 = assign53050_e87393_d_n11;

        let (assign53060_e87419, assign53060_e87419_d_n3, assign53060_e87419_d_n4, assign53060_e87419_d_n5, assign53060_e87419_d_n6, assign53060_e87419_d_n7, assign53060_e87419_d_n8, assign53060_e87419_d_n9, assign53060_e87419_d_n10, assign53060_e87419_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard815 != 0.0)) && (locals.var_guard816 == 0.0)) {
        let assign53060_e87404: f64 = (locals.var_t1 - 1.0);
        let assign53060_e87405: f64 = (locals.var_czbdswg_p1 * assign53060_e87404);
        let assign53060_e87408: f64 = (5.0 * p.p918);
        let assign53060_e87411: f64 = (locals.var_t1 - 1.0);
        let assign53060_e87412: f64 = (assign53060_e87408 * assign53060_e87411);
        let assign53060_e87415: f64 = (1.0 + p.p918);
        let assign53060_e87416: f64 = (assign53060_e87412 + assign53060_e87415);
        let assign53060_e87417: f64 = (assign53060_e87405 * assign53060_e87416);
        (assign53060_e87417, (((locals.var_czbdswg_p1 * locals.var_t1_dn3) * assign53060_e87416) + (assign53060_e87405 * (assign53060_e87408 * locals.var_t1_dn3))), (((locals.var_czbdswg_p1 * locals.var_t1_dn4) * assign53060_e87416) + (assign53060_e87405 * (assign53060_e87408 * locals.var_t1_dn4))), (((locals.var_czbdswg_p1 * locals.var_t1_dn5) * assign53060_e87416) + (assign53060_e87405 * (assign53060_e87408 * locals.var_t1_dn5))), (((locals.var_czbdswg_p1 * locals.var_t1_dn6) * assign53060_e87416) + (assign53060_e87405 * (assign53060_e87408 * locals.var_t1_dn6))), (((locals.var_czbdswg_p1 * locals.var_t1_dn7) * assign53060_e87416) + (assign53060_e87405 * (assign53060_e87408 * locals.var_t1_dn7))), (((locals.var_czbdswg_p1 * locals.var_t1_dn8) * assign53060_e87416) + (assign53060_e87405 * (assign53060_e87408 * locals.var_t1_dn8))), (((locals.var_czbdswg_p1 * locals.var_t1_dn9) * assign53060_e87416) + (assign53060_e87405 * (assign53060_e87408 * locals.var_t1_dn9))), (((locals.var_czbdswg_p1 * locals.var_t1_dn10) * assign53060_e87416) + (assign53060_e87405 * (assign53060_e87408 * locals.var_t1_dn10))), (((locals.var_czbdswg_p1 * locals.var_t1_dn11) * assign53060_e87416) + (assign53060_e87405 * (assign53060_e87408 * locals.var_t1_dn11))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign53060_e87419;
        locals.var_t2_dn3 = assign53060_e87419_d_n3;
        locals.var_t2_dn4 = assign53060_e87419_d_n4;
        locals.var_t2_dn5 = assign53060_e87419_d_n5;
        locals.var_t2_dn6 = assign53060_e87419_d_n6;
        locals.var_t2_dn7 = assign53060_e87419_d_n7;
        locals.var_t2_dn8 = assign53060_e87419_d_n8;
        locals.var_t2_dn9 = assign53060_e87419_d_n9;
        locals.var_t2_dn10 = assign53060_e87419_d_n10;
        locals.var_t2_dn11 = assign53060_e87419_d_n11;

        let (assign53070_e87435, assign53070_e87435_d_n3, assign53070_e87435_d_n4, assign53070_e87435_d_n5, assign53070_e87435_d_n6, assign53070_e87435_d_n7, assign53070_e87435_d_n8, assign53070_e87435_d_n9, assign53070_e87435_d_n10, assign53070_e87435_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard815 != 0.0)) && (locals.var_guard816 == 0.0)) {
        let assign53070_e87429: f64 = (locals.var_pbswgd_t * locals.var_czbdswg);
        let assign53070_e87432: f64 = (locals.var_t2 + locals.var_czbdswg_p2);
        let assign53070_e87433: f64 = (assign53070_e87429 * assign53070_e87432);
        (assign53070_e87433, (assign53070_e87429 * locals.var_t2_dn3), ((((locals.var_pbswgd_t_dn4 * locals.var_czbdswg) + (locals.var_pbswgd_t * locals.var_czbdswg_dn4)) * assign53070_e87432) + (assign53070_e87429 * locals.var_t2_dn4)), ((((locals.var_pbswgd_t_dn5 * locals.var_czbdswg) + (locals.var_pbswgd_t * locals.var_czbdswg_dn5)) * assign53070_e87432) + (assign53070_e87429 * locals.var_t2_dn5)), (assign53070_e87429 * locals.var_t2_dn6), (assign53070_e87429 * locals.var_t2_dn7), (assign53070_e87429 * locals.var_t2_dn8), (assign53070_e87429 * locals.var_t2_dn9), (assign53070_e87429 * locals.var_t2_dn10), (assign53070_e87429 * locals.var_t2_dn11),)
    } else {
        (locals.var_qbdj3, locals.var_qbdj3_dn3, locals.var_qbdj3_dn4, locals.var_qbdj3_dn5, locals.var_qbdj3_dn6, locals.var_qbdj3_dn7, locals.var_qbdj3_dn8, locals.var_qbdj3_dn9, locals.var_qbdj3_dn10, locals.var_qbdj3_dn11,)
    }
};
        locals.var_qbdj3 = assign53070_e87435;
        locals.var_qbdj3_dn3 = assign53070_e87435_d_n3;
        locals.var_qbdj3_dn4 = assign53070_e87435_d_n4;
        locals.var_qbdj3_dn5 = assign53070_e87435_d_n5;
        locals.var_qbdj3_dn6 = assign53070_e87435_d_n6;
        locals.var_qbdj3_dn7 = assign53070_e87435_d_n7;
        locals.var_qbdj3_dn8 = assign53070_e87435_d_n8;
        locals.var_qbdj3_dn9 = assign53070_e87435_d_n9;
        locals.var_qbdj3_dn10 = assign53070_e87435_d_n10;
        locals.var_qbdj3_dn11 = assign53070_e87435_d_n11;

        let (assign53080_e87443, assign53080_e87443_d_n3, assign53080_e87443_d_n4, assign53080_e87443_d_n5, assign53080_e87443_d_n6, assign53080_e87443_d_n7, assign53080_e87443_d_n8, assign53080_e87443_d_n9, assign53080_e87443_d_n10, assign53080_e87443_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard815 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbdj3, locals.var_qbdj3_dn3, locals.var_qbdj3_dn4, locals.var_qbdj3_dn5, locals.var_qbdj3_dn6, locals.var_qbdj3_dn7, locals.var_qbdj3_dn8, locals.var_qbdj3_dn9, locals.var_qbdj3_dn10, locals.var_qbdj3_dn11,)
    }
};
        locals.var_qbdj3 = assign53080_e87443;
        locals.var_qbdj3_dn3 = assign53080_e87443_d_n3;
        locals.var_qbdj3_dn4 = assign53080_e87443_d_n4;
        locals.var_qbdj3_dn5 = assign53080_e87443_d_n5;
        locals.var_qbdj3_dn6 = assign53080_e87443_d_n6;
        locals.var_qbdj3_dn7 = assign53080_e87443_d_n7;
        locals.var_qbdj3_dn8 = assign53080_e87443_d_n8;
        locals.var_qbdj3_dn9 = assign53080_e87443_d_n9;
        locals.var_qbdj3_dn10 = assign53080_e87443_d_n10;
        locals.var_qbdj3_dn11 = assign53080_e87443_d_n11;

        let (assign53090_e87452, assign53090_e87452_d_n3, assign53090_e87452_d_n4, assign53090_e87452_d_n5, assign53090_e87452_d_n6, assign53090_e87452_d_n7, assign53090_e87452_d_n8, assign53090_e87452_d_n9, assign53090_e87452_d_n10, assign53090_e87452_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign53090_e87448: f64 = (p.p919 * locals.var_ibddif);
        let assign53090_e87450: f64 = (assign53090_e87448 * p.p2);
        (assign53090_e87450, ((p.p919 * locals.var_ibddif_dn3) * p.p2), ((p.p919 * locals.var_ibddif_dn4) * p.p2), ((p.p919 * locals.var_ibddif_dn5) * p.p2), ((p.p919 * locals.var_ibddif_dn6) * p.p2), ((p.p919 * locals.var_ibddif_dn7) * p.p2), ((p.p919 * locals.var_ibddif_dn8) * p.p2), ((p.p919 * locals.var_ibddif_dn9) * p.p2), ((p.p919 * locals.var_ibddif_dn10) * p.p2), ((p.p919 * locals.var_ibddif_dn11) * p.p2),)
    } else {
        (locals.var_qbdj4, locals.var_qbdj4_dn3, locals.var_qbdj4_dn4, locals.var_qbdj4_dn5, locals.var_qbdj4_dn6, locals.var_qbdj4_dn7, locals.var_qbdj4_dn8, locals.var_qbdj4_dn9, locals.var_qbdj4_dn10, locals.var_qbdj4_dn11,)
    }
};
        locals.var_qbdj4 = assign53090_e87452;
        locals.var_qbdj4_dn3 = assign53090_e87452_d_n3;
        locals.var_qbdj4_dn4 = assign53090_e87452_d_n4;
        locals.var_qbdj4_dn5 = assign53090_e87452_d_n5;
        locals.var_qbdj4_dn6 = assign53090_e87452_d_n6;
        locals.var_qbdj4_dn7 = assign53090_e87452_d_n7;
        locals.var_qbdj4_dn8 = assign53090_e87452_d_n8;
        locals.var_qbdj4_dn9 = assign53090_e87452_d_n9;
        locals.var_qbdj4_dn10 = assign53090_e87452_d_n10;
        locals.var_qbdj4_dn11 = assign53090_e87452_d_n11;

        let (assign53100_e87463, assign53100_e87463_d_n3, assign53100_e87463_d_n4, assign53100_e87463_d_n5, assign53100_e87463_d_n6, assign53100_e87463_d_n7, assign53100_e87463_d_n8, assign53100_e87463_d_n9, assign53100_e87463_d_n10, assign53100_e87463_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign53100_e87457: f64 = (locals.var_qbdj1 + locals.var_qbdj2);
        let assign53100_e87459: f64 = (assign53100_e87457 + locals.var_qbdj3);
        let assign53100_e87461: f64 = (assign53100_e87459 + locals.var_qbdj4);
        (assign53100_e87461, (((locals.var_qbdj1_dn3 + locals.var_qbdj2_dn3) + locals.var_qbdj3_dn3) + locals.var_qbdj4_dn3), (((locals.var_qbdj1_dn4 + locals.var_qbdj2_dn4) + locals.var_qbdj3_dn4) + locals.var_qbdj4_dn4), (((locals.var_qbdj1_dn5 + locals.var_qbdj2_dn5) + locals.var_qbdj3_dn5) + locals.var_qbdj4_dn5), (((locals.var_qbdj1_dn6 + locals.var_qbdj2_dn6) + locals.var_qbdj3_dn6) + locals.var_qbdj4_dn6), (((locals.var_qbdj1_dn7 + locals.var_qbdj2_dn7) + locals.var_qbdj3_dn7) + locals.var_qbdj4_dn7), (((locals.var_qbdj1_dn8 + locals.var_qbdj2_dn8) + locals.var_qbdj3_dn8) + locals.var_qbdj4_dn8), (((locals.var_qbdj1_dn9 + locals.var_qbdj2_dn9) + locals.var_qbdj3_dn9) + locals.var_qbdj4_dn9), (((locals.var_qbdj1_dn10 + locals.var_qbdj2_dn10) + locals.var_qbdj3_dn10) + locals.var_qbdj4_dn10), (((locals.var_qbdj1_dn11 + locals.var_qbdj2_dn11) + locals.var_qbdj3_dn11) + locals.var_qbdj4_dn11),)
    } else {
        (locals.var_qbdj, locals.var_qbdj_dn3, locals.var_qbdj_dn4, locals.var_qbdj_dn5, locals.var_qbdj_dn6, locals.var_qbdj_dn7, locals.var_qbdj_dn8, locals.var_qbdj_dn9, locals.var_qbdj_dn10, locals.var_qbdj_dn11,)
    }
};
        locals.var_qbdj = assign53100_e87463;
        locals.var_qbdj_dn3 = assign53100_e87463_d_n3;
        locals.var_qbdj_dn4 = assign53100_e87463_d_n4;
        locals.var_qbdj_dn5 = assign53100_e87463_d_n5;
        locals.var_qbdj_dn6 = assign53100_e87463_d_n6;
        locals.var_qbdj_dn7 = assign53100_e87463_d_n7;
        locals.var_qbdj_dn8 = assign53100_e87463_d_n8;
        locals.var_qbdj_dn9 = assign53100_e87463_d_n9;
        locals.var_qbdj_dn10 = assign53100_e87463_d_n10;
        locals.var_qbdj_dn11 = assign53100_e87463_d_n11;

    }

    pub(super) fn stamp_transient_block_180(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let assign53110_e87466: f64 = if p.p28 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard819 = assign53110_e87466;

        let (assign53120_e87477, assign53120_e87477_d_n3, assign53120_e87477_d_n4, assign53120_e87477_d_n5, assign53120_e87477_d_n6, assign53120_e87477_d_n7, assign53120_e87477_d_n8, assign53120_e87477_d_n9, assign53120_e87477_d_n10, assign53120_e87477_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard819 != 0.0)) {
        let assign53120_e87473: f64 = (locals.var_ndep_i / 1e23);
        let assign53120_e87475: f64 = (assign53120_e87473).powf(p.p1144);
        (assign53120_e87475, if 0.0 == 0.0 && ((p.p1144) as f64).is_finite() && ((p.p1144) as f64).fract() == 0.0 { if p.p1144 == 0.0 { 0.0 } else { (p.p1144 * ((assign53120_e87473).powf(p.p1144 - 1.0) * (locals.var_ndep_i_dn3 / 1e23))) } } else { (assign53120_e87475 * (p.p1144 * ((locals.var_ndep_i_dn3 / 1e23) / assign53120_e87473))) }, if 0.0 == 0.0 && ((p.p1144) as f64).is_finite() && ((p.p1144) as f64).fract() == 0.0 { if p.p1144 == 0.0 { 0.0 } else { (p.p1144 * ((assign53120_e87473).powf(p.p1144 - 1.0) * (locals.var_ndep_i_dn4 / 1e23))) } } else { (assign53120_e87475 * (p.p1144 * ((locals.var_ndep_i_dn4 / 1e23) / assign53120_e87473))) }, if 0.0 == 0.0 && ((p.p1144) as f64).is_finite() && ((p.p1144) as f64).fract() == 0.0 { if p.p1144 == 0.0 { 0.0 } else { (p.p1144 * ((assign53120_e87473).powf(p.p1144 - 1.0) * (locals.var_ndep_i_dn5 / 1e23))) } } else { (assign53120_e87475 * (p.p1144 * ((locals.var_ndep_i_dn5 / 1e23) / assign53120_e87473))) }, if 0.0 == 0.0 && ((p.p1144) as f64).is_finite() && ((p.p1144) as f64).fract() == 0.0 { if p.p1144 == 0.0 { 0.0 } else { (p.p1144 * ((assign53120_e87473).powf(p.p1144 - 1.0) * (locals.var_ndep_i_dn6 / 1e23))) } } else { (assign53120_e87475 * (p.p1144 * ((locals.var_ndep_i_dn6 / 1e23) / assign53120_e87473))) }, if 0.0 == 0.0 && ((p.p1144) as f64).is_finite() && ((p.p1144) as f64).fract() == 0.0 { if p.p1144 == 0.0 { 0.0 } else { (p.p1144 * ((assign53120_e87473).powf(p.p1144 - 1.0) * (locals.var_ndep_i_dn7 / 1e23))) } } else { (assign53120_e87475 * (p.p1144 * ((locals.var_ndep_i_dn7 / 1e23) / assign53120_e87473))) }, if 0.0 == 0.0 && ((p.p1144) as f64).is_finite() && ((p.p1144) as f64).fract() == 0.0 { if p.p1144 == 0.0 { 0.0 } else { (p.p1144 * ((assign53120_e87473).powf(p.p1144 - 1.0) * (locals.var_ndep_i_dn8 / 1e23))) } } else { (assign53120_e87475 * (p.p1144 * ((locals.var_ndep_i_dn8 / 1e23) / assign53120_e87473))) }, if 0.0 == 0.0 && ((p.p1144) as f64).is_finite() && ((p.p1144) as f64).fract() == 0.0 { if p.p1144 == 0.0 { 0.0 } else { (p.p1144 * ((assign53120_e87473).powf(p.p1144 - 1.0) * (locals.var_ndep_i_dn9 / 1e23))) } } else { (assign53120_e87475 * (p.p1144 * ((locals.var_ndep_i_dn9 / 1e23) / assign53120_e87473))) }, if 0.0 == 0.0 && ((p.p1144) as f64).is_finite() && ((p.p1144) as f64).fract() == 0.0 { if p.p1144 == 0.0 { 0.0 } else { (p.p1144 * ((assign53120_e87473).powf(p.p1144 - 1.0) * (locals.var_ndep_i_dn10 / 1e23))) } } else { (assign53120_e87475 * (p.p1144 * ((locals.var_ndep_i_dn10 / 1e23) / assign53120_e87473))) }, if 0.0 == 0.0 && ((p.p1144) as f64).is_finite() && ((p.p1144) as f64).fract() == 0.0 { if p.p1144 == 0.0 { 0.0 } else { (p.p1144 * ((assign53120_e87473).powf(p.p1144 - 1.0) * (locals.var_ndep_i_dn11 / 1e23))) } } else { (assign53120_e87475 * (p.p1144 * ((locals.var_ndep_i_dn11 / 1e23) / assign53120_e87473))) },)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign53120_e87477;
        locals.var_t1_dn3 = assign53120_e87477_d_n3;
        locals.var_t1_dn4 = assign53120_e87477_d_n4;
        locals.var_t1_dn5 = assign53120_e87477_d_n5;
        locals.var_t1_dn6 = assign53120_e87477_d_n6;
        locals.var_t1_dn7 = assign53120_e87477_d_n7;
        locals.var_t1_dn8 = assign53120_e87477_d_n8;
        locals.var_t1_dn9 = assign53120_e87477_d_n9;
        locals.var_t1_dn10 = assign53120_e87477_d_n10;
        locals.var_t1_dn11 = assign53120_e87477_d_n11;

        let (assign53130_e87488, assign53130_e87488_d_n3, assign53130_e87488_d_n4, assign53130_e87488_d_n5, assign53130_e87488_d_n6, assign53130_e87488_d_n7, assign53130_e87488_d_n8, assign53130_e87488_d_n9, assign53130_e87488_d_n10, assign53130_e87488_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard819 != 0.0)) {
        let assign53130_e87484: f64 = (300.0 / locals.var_devtemp);
        let assign53130_e87486: f64 = (assign53130_e87484).powf(p.p1145);
        (assign53130_e87486, 0.0, if 0.0 == 0.0 && ((p.p1145) as f64).is_finite() && ((p.p1145) as f64).fract() == 0.0 { if p.p1145 == 0.0 { 0.0 } else { (p.p1145 * ((assign53130_e87484).powf(p.p1145 - 1.0) * (-((300.0 * locals.var_devtemp_dn4) / (locals.var_devtemp * locals.var_devtemp))))) } } else { (assign53130_e87486 * (p.p1145 * ((-((300.0 * locals.var_devtemp_dn4) / (locals.var_devtemp * locals.var_devtemp))) / assign53130_e87484))) }, if 0.0 == 0.0 && ((p.p1145) as f64).is_finite() && ((p.p1145) as f64).fract() == 0.0 { if p.p1145 == 0.0 { 0.0 } else { (p.p1145 * ((assign53130_e87484).powf(p.p1145 - 1.0) * (-((300.0 * locals.var_devtemp_dn5) / (locals.var_devtemp * locals.var_devtemp))))) } } else { (assign53130_e87486 * (p.p1145 * ((-((300.0 * locals.var_devtemp_dn5) / (locals.var_devtemp * locals.var_devtemp))) / assign53130_e87484))) }, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign53130_e87488;
        locals.var_t2_dn3 = assign53130_e87488_d_n3;
        locals.var_t2_dn4 = assign53130_e87488_d_n4;
        locals.var_t2_dn5 = assign53130_e87488_d_n5;
        locals.var_t2_dn6 = assign53130_e87488_d_n6;
        locals.var_t2_dn7 = assign53130_e87488_d_n7;
        locals.var_t2_dn8 = assign53130_e87488_d_n8;
        locals.var_t2_dn9 = assign53130_e87488_d_n9;
        locals.var_t2_dn10 = assign53130_e87488_d_n10;
        locals.var_t2_dn11 = assign53130_e87488_d_n11;

        let (assign53140_e87501, assign53140_e87501_d_n3, assign53140_e87501_d_n4, assign53140_e87501_d_n5, assign53140_e87501_d_n6, assign53140_e87501_d_n7, assign53140_e87501_d_n8, assign53140_e87501_d_n9, assign53140_e87501_d_n10, assign53140_e87501_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard819 != 0.0)) {
        let assign53140_e87495: f64 = (locals.var_devsign * p.p1143);
        let assign53140_e87497: f64 = (assign53140_e87495 * (nv10 - nv7));
        let assign53140_e87499: f64 = (assign53140_e87497 / locals.var_vt);
        (assign53140_e87499, 0.0, (-((assign53140_e87497 * locals.var_vt_dn4) / (locals.var_vt * locals.var_vt))), (-((assign53140_e87497 * locals.var_vt_dn5) / (locals.var_vt * locals.var_vt))), 0.0, ((-assign53140_e87495) / locals.var_vt), 0.0, 0.0, (assign53140_e87495 / locals.var_vt), 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign53140_e87501;
        locals.var_t3_dn3 = assign53140_e87501_d_n3;
        locals.var_t3_dn4 = assign53140_e87501_d_n4;
        locals.var_t3_dn5 = assign53140_e87501_d_n5;
        locals.var_t3_dn6 = assign53140_e87501_d_n6;
        locals.var_t3_dn7 = assign53140_e87501_d_n7;
        locals.var_t3_dn8 = assign53140_e87501_d_n8;
        locals.var_t3_dn9 = assign53140_e87501_d_n9;
        locals.var_t3_dn10 = assign53140_e87501_d_n10;
        locals.var_t3_dn11 = assign53140_e87501_d_n11;

        let (assign53150_e87514, assign53150_e87514_d_n3, assign53150_e87514_d_n4, assign53150_e87514_d_n5, assign53150_e87514_d_n6, assign53150_e87514_d_n7, assign53150_e87514_d_n8, assign53150_e87514_d_n9, assign53150_e87514_d_n10, assign53150_e87514_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard819 != 0.0)) {
        let assign53150_e87508: f64 = (-locals.var_t1);
        let assign53150_e87510: f64 = (assign53150_e87508 * locals.var_t2);
        let assign53150_e87511: f64 = { let limited_exp_arg = assign53150_e87510; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign53150_e87512: f64 = (p.p1138 * assign53150_e87511);
        (assign53150_e87512, (p.p1138 * ({ let limited_exp_arg = assign53150_e87510; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-locals.var_t1_dn3) * locals.var_t2) + (assign53150_e87508 * locals.var_t2_dn3)))), (p.p1138 * ({ let limited_exp_arg = assign53150_e87510; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-locals.var_t1_dn4) * locals.var_t2) + (assign53150_e87508 * locals.var_t2_dn4)))), (p.p1138 * ({ let limited_exp_arg = assign53150_e87510; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-locals.var_t1_dn5) * locals.var_t2) + (assign53150_e87508 * locals.var_t2_dn5)))), (p.p1138 * ({ let limited_exp_arg = assign53150_e87510; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-locals.var_t1_dn6) * locals.var_t2) + (assign53150_e87508 * locals.var_t2_dn6)))), (p.p1138 * ({ let limited_exp_arg = assign53150_e87510; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-locals.var_t1_dn7) * locals.var_t2) + (assign53150_e87508 * locals.var_t2_dn7)))), (p.p1138 * ({ let limited_exp_arg = assign53150_e87510; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-locals.var_t1_dn8) * locals.var_t2) + (assign53150_e87508 * locals.var_t2_dn8)))), (p.p1138 * ({ let limited_exp_arg = assign53150_e87510; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-locals.var_t1_dn9) * locals.var_t2) + (assign53150_e87508 * locals.var_t2_dn9)))), (p.p1138 * ({ let limited_exp_arg = assign53150_e87510; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-locals.var_t1_dn10) * locals.var_t2) + (assign53150_e87508 * locals.var_t2_dn10)))), (p.p1138 * ({ let limited_exp_arg = assign53150_e87510; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-locals.var_t1_dn11) * locals.var_t2) + (assign53150_e87508 * locals.var_t2_dn11)))),)
    } else {
        (locals.var_ssl0_nt, locals.var_ssl0_nt_dn3, locals.var_ssl0_nt_dn4, locals.var_ssl0_nt_dn5, locals.var_ssl0_nt_dn6, locals.var_ssl0_nt_dn7, locals.var_ssl0_nt_dn8, locals.var_ssl0_nt_dn9, locals.var_ssl0_nt_dn10, locals.var_ssl0_nt_dn11,)
    }
};
        locals.var_ssl0_nt = assign53150_e87514;
        locals.var_ssl0_nt_dn3 = assign53150_e87514_d_n3;
        locals.var_ssl0_nt_dn4 = assign53150_e87514_d_n4;
        locals.var_ssl0_nt_dn5 = assign53150_e87514_d_n5;
        locals.var_ssl0_nt_dn6 = assign53150_e87514_d_n6;
        locals.var_ssl0_nt_dn7 = assign53150_e87514_d_n7;
        locals.var_ssl0_nt_dn8 = assign53150_e87514_d_n8;
        locals.var_ssl0_nt_dn9 = assign53150_e87514_d_n9;
        locals.var_ssl0_nt_dn10 = assign53150_e87514_d_n10;
        locals.var_ssl0_nt_dn11 = assign53150_e87514_d_n11;

        let (assign53160_e87525, assign53160_e87525_d_n3, assign53160_e87525_d_n4, assign53160_e87525_d_n5, assign53160_e87525_d_n6, assign53160_e87525_d_n7, assign53160_e87525_d_n8, assign53160_e87525_d_n9, assign53160_e87525_d_n10, assign53160_e87525_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard819 != 0.0)) {
        let assign53160_e87521: f64 = (p.p1139 * locals.var_t2);
        let assign53160_e87523: f64 = (assign53160_e87521 * locals.var_t1);
        (assign53160_e87523, (((p.p1139 * locals.var_t2_dn3) * locals.var_t1) + (assign53160_e87521 * locals.var_t1_dn3)), (((p.p1139 * locals.var_t2_dn4) * locals.var_t1) + (assign53160_e87521 * locals.var_t1_dn4)), (((p.p1139 * locals.var_t2_dn5) * locals.var_t1) + (assign53160_e87521 * locals.var_t1_dn5)), (((p.p1139 * locals.var_t2_dn6) * locals.var_t1) + (assign53160_e87521 * locals.var_t1_dn6)), (((p.p1139 * locals.var_t2_dn7) * locals.var_t1) + (assign53160_e87521 * locals.var_t1_dn7)), (((p.p1139 * locals.var_t2_dn8) * locals.var_t1) + (assign53160_e87521 * locals.var_t1_dn8)), (((p.p1139 * locals.var_t2_dn9) * locals.var_t1) + (assign53160_e87521 * locals.var_t1_dn9)), (((p.p1139 * locals.var_t2_dn10) * locals.var_t1) + (assign53160_e87521 * locals.var_t1_dn10)), (((p.p1139 * locals.var_t2_dn11) * locals.var_t1) + (assign53160_e87521 * locals.var_t1_dn11)),)
    } else {
        (locals.var_ssl1_nt, locals.var_ssl1_nt_dn3, locals.var_ssl1_nt_dn4, locals.var_ssl1_nt_dn5, locals.var_ssl1_nt_dn6, locals.var_ssl1_nt_dn7, locals.var_ssl1_nt_dn8, locals.var_ssl1_nt_dn9, locals.var_ssl1_nt_dn10, locals.var_ssl1_nt_dn11,)
    }
};
        locals.var_ssl1_nt = assign53160_e87525;
        locals.var_ssl1_nt_dn3 = assign53160_e87525_d_n3;
        locals.var_ssl1_nt_dn4 = assign53160_e87525_d_n4;
        locals.var_ssl1_nt_dn5 = assign53160_e87525_d_n5;
        locals.var_ssl1_nt_dn6 = assign53160_e87525_d_n6;
        locals.var_ssl1_nt_dn7 = assign53160_e87525_d_n7;
        locals.var_ssl1_nt_dn8 = assign53160_e87525_d_n8;
        locals.var_ssl1_nt_dn9 = assign53160_e87525_d_n9;
        locals.var_ssl1_nt_dn10 = assign53160_e87525_d_n10;
        locals.var_ssl1_nt_dn11 = assign53160_e87525_d_n11;

        let (assign53170_e87544, assign53170_e87544_d_n3, assign53170_e87544_d_n4, assign53170_e87544_d_n5, assign53170_e87544_d_n6, assign53170_e87544_d_n7, assign53170_e87544_d_n8, assign53170_e87544_d_n9, assign53170_e87544_d_n10, assign53170_e87544_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard819 != 0.0)) {
        let assign53170_e87533: f64 = (locals.var_devsign * p.p1142);
        let assign53170_e87536: f64 = ((nv8 - nv10) - locals.var_vth);
        let assign53170_e87538: f64 = (assign53170_e87536 - (nv7 - nv10));
        let assign53170_e87539: f64 = (assign53170_e87533 * assign53170_e87538);
        let assign53170_e87540: f64 = { let limited_exp_arg = assign53170_e87539; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign53170_e87541: f64 = (assign53170_e87540).tanh();
        let assign53170_e87542: f64 = (p.p1141 * assign53170_e87541);
        (assign53170_e87542, (p.p1141 * (({ let limited_exp_arg = assign53170_e87539; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign53170_e87533 * (-locals.var_vth_dn3))) / ((assign53170_e87540).cosh() * (assign53170_e87540).cosh()))), (p.p1141 * (({ let limited_exp_arg = assign53170_e87539; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign53170_e87533 * (-locals.var_vth_dn4))) / ((assign53170_e87540).cosh() * (assign53170_e87540).cosh()))), (p.p1141 * (({ let limited_exp_arg = assign53170_e87539; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign53170_e87533 * (-locals.var_vth_dn5))) / ((assign53170_e87540).cosh() * (assign53170_e87540).cosh()))), (p.p1141 * (({ let limited_exp_arg = assign53170_e87539; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign53170_e87533 * (-locals.var_vth_dn6))) / ((assign53170_e87540).cosh() * (assign53170_e87540).cosh()))), (p.p1141 * (({ let limited_exp_arg = assign53170_e87539; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign53170_e87533 * ((-locals.var_vth_dn7) - 1.0))) / ((assign53170_e87540).cosh() * (assign53170_e87540).cosh()))), (p.p1141 * (({ let limited_exp_arg = assign53170_e87539; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign53170_e87533 * (1.0 - locals.var_vth_dn8))) / ((assign53170_e87540).cosh() * (assign53170_e87540).cosh()))), (p.p1141 * (({ let limited_exp_arg = assign53170_e87539; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign53170_e87533 * (-locals.var_vth_dn9))) / ((assign53170_e87540).cosh() * (assign53170_e87540).cosh()))), (p.p1141 * (({ let limited_exp_arg = assign53170_e87539; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign53170_e87533 * ((-1.0 - locals.var_vth_dn10) - -1.0))) / ((assign53170_e87540).cosh() * (assign53170_e87540).cosh()))), (p.p1141 * (({ let limited_exp_arg = assign53170_e87539; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign53170_e87533 * (-locals.var_vth_dn11))) / ((assign53170_e87540).cosh() * (assign53170_e87540).cosh()))),)
    } else {
        (locals.var_phib_ssl, locals.var_phib_ssl_dn3, locals.var_phib_ssl_dn4, locals.var_phib_ssl_dn5, locals.var_phib_ssl_dn6, locals.var_phib_ssl_dn7, locals.var_phib_ssl_dn8, locals.var_phib_ssl_dn9, locals.var_phib_ssl_dn10, locals.var_phib_ssl_dn11,)
    }
};
        locals.var_phib_ssl = assign53170_e87544;
        locals.var_phib_ssl_dn3 = assign53170_e87544_d_n3;
        locals.var_phib_ssl_dn4 = assign53170_e87544_d_n4;
        locals.var_phib_ssl_dn5 = assign53170_e87544_d_n5;
        locals.var_phib_ssl_dn6 = assign53170_e87544_d_n6;
        locals.var_phib_ssl_dn7 = assign53170_e87544_d_n7;
        locals.var_phib_ssl_dn8 = assign53170_e87544_d_n8;
        locals.var_phib_ssl_dn9 = assign53170_e87544_d_n9;
        locals.var_phib_ssl_dn10 = assign53170_e87544_d_n10;
        locals.var_phib_ssl_dn11 = assign53170_e87544_d_n11;

        let (assign53180_e87580, assign53180_e87580_d_n3, assign53180_e87580_d_n4, assign53180_e87580_d_n5, assign53180_e87580_d_n6, assign53180_e87580_d_n7, assign53180_e87580_d_n8, assign53180_e87580_d_n9, assign53180_e87580_d_n10, assign53180_e87580_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard819 != 0.0)) {
        let assign53180_e87551: f64 = (locals.var_sigvds * p.p2);
        let assign53180_e87553: f64 = (assign53180_e87551 * locals.var_weff);
        let assign53180_e87555: f64 = (assign53180_e87553 * locals.var_ssl0_nt);
        let assign53180_e87557: f64 = { let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign53180_e87558: f64 = (assign53180_e87555 * assign53180_e87557);
        let assign53180_e87560: f64 = (-locals.var_ssl1_nt);
        let assign53180_e87562: f64 = (assign53180_e87560 * locals.var_leff);
        let assign53180_e87563: f64 = { let limited_exp_arg = assign53180_e87562; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign53180_e87564: f64 = (assign53180_e87558 * assign53180_e87563);
        let assign53180_e87567: f64 = (locals.var_phib_ssl / locals.var_vt);
        let assign53180_e87568: f64 = { let limited_exp_arg = assign53180_e87567; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign53180_e87569: f64 = (assign53180_e87564 * assign53180_e87568);
        let assign53180_e87572: f64 = (p.p1140 * locals.var_vdsx);
        let assign53180_e87574: f64 = (assign53180_e87572 / locals.var_vt);
        let assign53180_e87575: f64 = { let limited_exp_arg = assign53180_e87574; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign53180_e87577: f64 = (assign53180_e87575 - 1.0);
        let assign53180_e87578: f64 = (assign53180_e87569 * assign53180_e87577);
        (assign53180_e87578, (((((((((assign53180_e87553 * locals.var_ssl0_nt_dn3) * assign53180_e87557) + (assign53180_e87555 * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn3))) * assign53180_e87563) + (assign53180_e87558 * ({ let limited_exp_arg = assign53180_e87562; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_ssl1_nt_dn3) * locals.var_leff)))) * assign53180_e87568) + (assign53180_e87564 * ({ let limited_exp_arg = assign53180_e87567; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_phib_ssl_dn3 / locals.var_vt)))) * assign53180_e87577) + (assign53180_e87569 * ({ let limited_exp_arg = assign53180_e87574; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((p.p1140 * locals.var_vdsx_dn3) / locals.var_vt)))), (((((((((assign53180_e87553 * locals.var_ssl0_nt_dn4) * assign53180_e87557) + (assign53180_e87555 * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn4))) * assign53180_e87563) + (assign53180_e87558 * ({ let limited_exp_arg = assign53180_e87562; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_ssl1_nt_dn4) * locals.var_leff)))) * assign53180_e87568) + (assign53180_e87564 * ({ let limited_exp_arg = assign53180_e87567; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_phib_ssl_dn4 * locals.var_vt) - (locals.var_phib_ssl * locals.var_vt_dn4)) / (locals.var_vt * locals.var_vt))))) * assign53180_e87577) + (assign53180_e87569 * ({ let limited_exp_arg = assign53180_e87574; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((p.p1140 * locals.var_vdsx_dn4) * locals.var_vt) - (assign53180_e87572 * locals.var_vt_dn4)) / (locals.var_vt * locals.var_vt))))), (((((((((assign53180_e87553 * locals.var_ssl0_nt_dn5) * assign53180_e87557) + (assign53180_e87555 * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn5))) * assign53180_e87563) + (assign53180_e87558 * ({ let limited_exp_arg = assign53180_e87562; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_ssl1_nt_dn5) * locals.var_leff)))) * assign53180_e87568) + (assign53180_e87564 * ({ let limited_exp_arg = assign53180_e87567; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_phib_ssl_dn5 * locals.var_vt) - (locals.var_phib_ssl * locals.var_vt_dn5)) / (locals.var_vt * locals.var_vt))))) * assign53180_e87577) + (assign53180_e87569 * ({ let limited_exp_arg = assign53180_e87574; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((p.p1140 * locals.var_vdsx_dn5) * locals.var_vt) - (assign53180_e87572 * locals.var_vt_dn5)) / (locals.var_vt * locals.var_vt))))), (((((((((assign53180_e87553 * locals.var_ssl0_nt_dn6) * assign53180_e87557) + (assign53180_e87555 * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn6))) * assign53180_e87563) + (assign53180_e87558 * ({ let limited_exp_arg = assign53180_e87562; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_ssl1_nt_dn6) * locals.var_leff)))) * assign53180_e87568) + (assign53180_e87564 * ({ let limited_exp_arg = assign53180_e87567; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_phib_ssl_dn6 / locals.var_vt)))) * assign53180_e87577) + (assign53180_e87569 * ({ let limited_exp_arg = assign53180_e87574; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((p.p1140 * locals.var_vdsx_dn6) / locals.var_vt)))), (((((((((assign53180_e87553 * locals.var_ssl0_nt_dn7) * assign53180_e87557) + (assign53180_e87555 * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn7))) * assign53180_e87563) + (assign53180_e87558 * ({ let limited_exp_arg = assign53180_e87562; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_ssl1_nt_dn7) * locals.var_leff)))) * assign53180_e87568) + (assign53180_e87564 * ({ let limited_exp_arg = assign53180_e87567; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_phib_ssl_dn7 / locals.var_vt)))) * assign53180_e87577) + (assign53180_e87569 * ({ let limited_exp_arg = assign53180_e87574; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((p.p1140 * locals.var_vdsx_dn7) / locals.var_vt)))), (((((((((assign53180_e87553 * locals.var_ssl0_nt_dn8) * assign53180_e87557) + (assign53180_e87555 * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn8))) * assign53180_e87563) + (assign53180_e87558 * ({ let limited_exp_arg = assign53180_e87562; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_ssl1_nt_dn8) * locals.var_leff)))) * assign53180_e87568) + (assign53180_e87564 * ({ let limited_exp_arg = assign53180_e87567; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_phib_ssl_dn8 / locals.var_vt)))) * assign53180_e87577) + (assign53180_e87569 * ({ let limited_exp_arg = assign53180_e87574; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((p.p1140 * locals.var_vdsx_dn8) / locals.var_vt)))), (((((((((assign53180_e87553 * locals.var_ssl0_nt_dn9) * assign53180_e87557) + (assign53180_e87555 * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn9))) * assign53180_e87563) + (assign53180_e87558 * ({ let limited_exp_arg = assign53180_e87562; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_ssl1_nt_dn9) * locals.var_leff)))) * assign53180_e87568) + (assign53180_e87564 * ({ let limited_exp_arg = assign53180_e87567; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_phib_ssl_dn9 / locals.var_vt)))) * assign53180_e87577) + (assign53180_e87569 * ({ let limited_exp_arg = assign53180_e87574; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((p.p1140 * locals.var_vdsx_dn9) / locals.var_vt)))), (((((((((assign53180_e87553 * locals.var_ssl0_nt_dn10) * assign53180_e87557) + (assign53180_e87555 * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn10))) * assign53180_e87563) + (assign53180_e87558 * ({ let limited_exp_arg = assign53180_e87562; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_ssl1_nt_dn10) * locals.var_leff)))) * assign53180_e87568) + (assign53180_e87564 * ({ let limited_exp_arg = assign53180_e87567; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_phib_ssl_dn10 / locals.var_vt)))) * assign53180_e87577) + (assign53180_e87569 * ({ let limited_exp_arg = assign53180_e87574; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((p.p1140 * locals.var_vdsx_dn10) / locals.var_vt)))), (((((((((assign53180_e87553 * locals.var_ssl0_nt_dn11) * assign53180_e87557) + (assign53180_e87555 * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn11))) * assign53180_e87563) + (assign53180_e87558 * ({ let limited_exp_arg = assign53180_e87562; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_ssl1_nt_dn11) * locals.var_leff)))) * assign53180_e87568) + (assign53180_e87564 * ({ let limited_exp_arg = assign53180_e87567; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_phib_ssl_dn11 / locals.var_vt)))) * assign53180_e87577) + (assign53180_e87569 * ({ let limited_exp_arg = assign53180_e87574; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((p.p1140 * locals.var_vdsx_dn11) / locals.var_vt)))),)
    } else {
        (locals.var_issl, locals.var_issl_dn3, locals.var_issl_dn4, locals.var_issl_dn5, locals.var_issl_dn6, locals.var_issl_dn7, locals.var_issl_dn8, locals.var_issl_dn9, locals.var_issl_dn10, locals.var_issl_dn11,)
    }
};
        locals.var_issl = assign53180_e87580;
        locals.var_issl_dn3 = assign53180_e87580_d_n3;
        locals.var_issl_dn4 = assign53180_e87580_d_n4;
        locals.var_issl_dn5 = assign53180_e87580_d_n5;
        locals.var_issl_dn6 = assign53180_e87580_d_n6;
        locals.var_issl_dn7 = assign53180_e87580_d_n7;
        locals.var_issl_dn8 = assign53180_e87580_d_n8;
        locals.var_issl_dn9 = assign53180_e87580_d_n9;
        locals.var_issl_dn10 = assign53180_e87580_d_n10;
        locals.var_issl_dn11 = assign53180_e87580_d_n11;

        let (assign53190_e87589, assign53190_e87589_d_n4, assign53190_e87589_d_n5,) = {
    if (locals.var_guard492 == 0.0) {
        let assign53190_e87585: f64 = (4.0 * locals.var_vt);
        let assign53190_e87587: f64 = (assign53190_e87585 * 1.602176462e-19);
        (assign53190_e87587, ((4.0 * locals.var_vt_dn4) * 1.602176462e-19), ((4.0 * locals.var_vt_dn5) * 1.602176462e-19),)
    } else {
        (locals.var_nt, locals.var_nt_dn4, locals.var_nt_dn5,)
    }
};
        locals.var_nt = assign53190_e87589;
        locals.var_nt_dn4 = assign53190_e87589_d_n4;
        locals.var_nt_dn5 = assign53190_e87589_d_n5;

        let (assign53200_e87598, assign53200_e87598_d_n3, assign53200_e87598_d_n4, assign53200_e87598_d_n5, assign53200_e87598_d_n6, assign53200_e87598_d_n7, assign53200_e87598_d_n8, assign53200_e87598_d_n9, assign53200_e87598_d_n10, assign53200_e87598_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign53200_e87594: f64 = (2.0 * locals.var_vsat_a);
        let assign53200_e87596: f64 = (assign53200_e87594 / locals.var_ueff);
        (assign53200_e87596, ((((2.0 * locals.var_vsat_a_dn3) * locals.var_ueff) - (assign53200_e87594 * locals.var_ueff_dn3)) / (locals.var_ueff * locals.var_ueff)), ((((2.0 * locals.var_vsat_a_dn4) * locals.var_ueff) - (assign53200_e87594 * locals.var_ueff_dn4)) / (locals.var_ueff * locals.var_ueff)), ((((2.0 * locals.var_vsat_a_dn5) * locals.var_ueff) - (assign53200_e87594 * locals.var_ueff_dn5)) / (locals.var_ueff * locals.var_ueff)), ((((2.0 * locals.var_vsat_a_dn6) * locals.var_ueff) - (assign53200_e87594 * locals.var_ueff_dn6)) / (locals.var_ueff * locals.var_ueff)), ((((2.0 * locals.var_vsat_a_dn7) * locals.var_ueff) - (assign53200_e87594 * locals.var_ueff_dn7)) / (locals.var_ueff * locals.var_ueff)), ((((2.0 * locals.var_vsat_a_dn8) * locals.var_ueff) - (assign53200_e87594 * locals.var_ueff_dn8)) / (locals.var_ueff * locals.var_ueff)), ((((2.0 * locals.var_vsat_a_dn9) * locals.var_ueff) - (assign53200_e87594 * locals.var_ueff_dn9)) / (locals.var_ueff * locals.var_ueff)), ((((2.0 * locals.var_vsat_a_dn10) * locals.var_ueff) - (assign53200_e87594 * locals.var_ueff_dn10)) / (locals.var_ueff * locals.var_ueff)), ((((2.0 * locals.var_vsat_a_dn11) * locals.var_ueff) - (assign53200_e87594 * locals.var_ueff_dn11)) / (locals.var_ueff * locals.var_ueff)),)
    } else {
        (locals.var_esatnoi, locals.var_esatnoi_dn3, locals.var_esatnoi_dn4, locals.var_esatnoi_dn5, locals.var_esatnoi_dn6, locals.var_esatnoi_dn7, locals.var_esatnoi_dn8, locals.var_esatnoi_dn9, locals.var_esatnoi_dn10, locals.var_esatnoi_dn11,)
    }
};
        locals.var_esatnoi = assign53200_e87598;
        locals.var_esatnoi_dn3 = assign53200_e87598_d_n3;
        locals.var_esatnoi_dn4 = assign53200_e87598_d_n4;
        locals.var_esatnoi_dn5 = assign53200_e87598_d_n5;
        locals.var_esatnoi_dn6 = assign53200_e87598_d_n6;
        locals.var_esatnoi_dn7 = assign53200_e87598_d_n7;
        locals.var_esatnoi_dn8 = assign53200_e87598_d_n8;
        locals.var_esatnoi_dn9 = assign53200_e87598_d_n9;
        locals.var_esatnoi_dn10 = assign53200_e87598_d_n10;
        locals.var_esatnoi_dn11 = assign53200_e87598_d_n11;

        let assign53210_e87601: f64 = if p.p1011 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard820 = assign53210_e87601;

        let (assign53220_e87608, assign53220_e87608_d_n3, assign53220_e87608_d_n4, assign53220_e87608_d_n5, assign53220_e87608_d_n6, assign53220_e87608_d_n7, assign53220_e87608_d_n8, assign53220_e87608_d_n9, assign53220_e87608_d_n10, assign53220_e87608_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard820 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_delclm, locals.var_delclm_dn3, locals.var_delclm_dn4, locals.var_delclm_dn5, locals.var_delclm_dn6, locals.var_delclm_dn7, locals.var_delclm_dn8, locals.var_delclm_dn9, locals.var_delclm_dn10, locals.var_delclm_dn11,)
    }
};
        locals.var_delclm = assign53220_e87608;
        locals.var_delclm_dn3 = assign53220_e87608_d_n3;
        locals.var_delclm_dn4 = assign53220_e87608_d_n4;
        locals.var_delclm_dn5 = assign53220_e87608_d_n5;
        locals.var_delclm_dn6 = assign53220_e87608_d_n6;
        locals.var_delclm_dn7 = assign53220_e87608_d_n7;
        locals.var_delclm_dn8 = assign53220_e87608_d_n8;
        locals.var_delclm_dn9 = assign53220_e87608_d_n9;
        locals.var_delclm_dn10 = assign53220_e87608_d_n10;
        locals.var_delclm_dn11 = assign53220_e87608_d_n11;

        let (assign53230_e87622, assign53230_e87622_d_n3, assign53230_e87622_d_n4, assign53230_e87622_d_n5, assign53230_e87622_d_n6, assign53230_e87622_d_n7, assign53230_e87622_d_n8, assign53230_e87622_d_n9, assign53230_e87622_d_n10, assign53230_e87622_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard820 == 0.0)) {
        let assign53230_e87616: f64 = (locals.var_diffvds / locals.var_litl);
        let assign53230_e87618: f64 = (assign53230_e87616 + p.p1011);
        let assign53230_e87620: f64 = (assign53230_e87618 / locals.var_esatnoi);
        (assign53230_e87620, ((((locals.var_diffvds_dn3 / locals.var_litl) * locals.var_esatnoi) - (assign53230_e87618 * locals.var_esatnoi_dn3)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn4 / locals.var_litl) * locals.var_esatnoi) - (assign53230_e87618 * locals.var_esatnoi_dn4)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn5 / locals.var_litl) * locals.var_esatnoi) - (assign53230_e87618 * locals.var_esatnoi_dn5)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn6 / locals.var_litl) * locals.var_esatnoi) - (assign53230_e87618 * locals.var_esatnoi_dn6)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn7 / locals.var_litl) * locals.var_esatnoi) - (assign53230_e87618 * locals.var_esatnoi_dn7)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn8 / locals.var_litl) * locals.var_esatnoi) - (assign53230_e87618 * locals.var_esatnoi_dn8)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn9 / locals.var_litl) * locals.var_esatnoi) - (assign53230_e87618 * locals.var_esatnoi_dn9)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn10 / locals.var_litl) * locals.var_esatnoi) - (assign53230_e87618 * locals.var_esatnoi_dn10)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn11 / locals.var_litl) * locals.var_esatnoi) - (assign53230_e87618 * locals.var_esatnoi_dn11)) / (locals.var_esatnoi * locals.var_esatnoi)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign53230_e87622;
        locals.var_t0_dn3 = assign53230_e87622_d_n3;
        locals.var_t0_dn4 = assign53230_e87622_d_n4;
        locals.var_t0_dn5 = assign53230_e87622_d_n5;
        locals.var_t0_dn6 = assign53230_e87622_d_n6;
        locals.var_t0_dn7 = assign53230_e87622_d_n7;
        locals.var_t0_dn8 = assign53230_e87622_d_n8;
        locals.var_t0_dn9 = assign53230_e87622_d_n9;
        locals.var_t0_dn10 = assign53230_e87622_d_n10;
        locals.var_t0_dn11 = assign53230_e87622_d_n11;

        let (assign53240_e87635, assign53240_e87635_d_n3, assign53240_e87635_d_n4, assign53240_e87635_d_n5, assign53240_e87635_d_n6, assign53240_e87635_d_n7, assign53240_e87635_d_n8, assign53240_e87635_d_n9, assign53240_e87635_d_n10, assign53240_e87635_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard820 == 0.0)) {
        let assign53240_e87631: f64 = (locals.var_t0).max(1e-38);
        let assign53240_e87632: f64 = (assign53240_e87631).ln();
        let assign53240_e87633: f64 = (locals.var_litl * assign53240_e87632);
        (assign53240_e87633, (locals.var_litl * (if locals.var_t0 >= 1e-38 { locals.var_t0_dn3 } else { 0.0 } / assign53240_e87631)), (locals.var_litl * (if locals.var_t0 >= 1e-38 { locals.var_t0_dn4 } else { 0.0 } / assign53240_e87631)), (locals.var_litl * (if locals.var_t0 >= 1e-38 { locals.var_t0_dn5 } else { 0.0 } / assign53240_e87631)), (locals.var_litl * (if locals.var_t0 >= 1e-38 { locals.var_t0_dn6 } else { 0.0 } / assign53240_e87631)), (locals.var_litl * (if locals.var_t0 >= 1e-38 { locals.var_t0_dn7 } else { 0.0 } / assign53240_e87631)), (locals.var_litl * (if locals.var_t0 >= 1e-38 { locals.var_t0_dn8 } else { 0.0 } / assign53240_e87631)), (locals.var_litl * (if locals.var_t0 >= 1e-38 { locals.var_t0_dn9 } else { 0.0 } / assign53240_e87631)), (locals.var_litl * (if locals.var_t0 >= 1e-38 { locals.var_t0_dn10 } else { 0.0 } / assign53240_e87631)), (locals.var_litl * (if locals.var_t0 >= 1e-38 { locals.var_t0_dn11 } else { 0.0 } / assign53240_e87631)),)
    } else {
        (locals.var_delclm, locals.var_delclm_dn3, locals.var_delclm_dn4, locals.var_delclm_dn5, locals.var_delclm_dn6, locals.var_delclm_dn7, locals.var_delclm_dn8, locals.var_delclm_dn9, locals.var_delclm_dn10, locals.var_delclm_dn11,)
    }
};
        locals.var_delclm = assign53240_e87635;
        locals.var_delclm_dn3 = assign53240_e87635_d_n3;
        locals.var_delclm_dn4 = assign53240_e87635_d_n4;
        locals.var_delclm_dn5 = assign53240_e87635_d_n5;
        locals.var_delclm_dn6 = assign53240_e87635_d_n6;
        locals.var_delclm_dn7 = assign53240_e87635_d_n7;
        locals.var_delclm_dn8 = assign53240_e87635_d_n8;
        locals.var_delclm_dn9 = assign53240_e87635_d_n9;
        locals.var_delclm_dn10 = assign53240_e87635_d_n10;
        locals.var_delclm_dn11 = assign53240_e87635_d_n11;

        let assign53250_e87638: f64 = if locals.var_delclm < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard821 = assign53250_e87638;

        let (assign53260_e87648, assign53260_e87648_d_n3, assign53260_e87648_d_n4, assign53260_e87648_d_n5, assign53260_e87648_d_n6, assign53260_e87648_d_n7, assign53260_e87648_d_n8, assign53260_e87648_d_n9, assign53260_e87648_d_n10, assign53260_e87648_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard820 == 0.0)) && (locals.var_guard821 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_delclm, locals.var_delclm_dn3, locals.var_delclm_dn4, locals.var_delclm_dn5, locals.var_delclm_dn6, locals.var_delclm_dn7, locals.var_delclm_dn8, locals.var_delclm_dn9, locals.var_delclm_dn10, locals.var_delclm_dn11,)
    }
};
        locals.var_delclm = assign53260_e87648;
        locals.var_delclm_dn3 = assign53260_e87648_d_n3;
        locals.var_delclm_dn4 = assign53260_e87648_d_n4;
        locals.var_delclm_dn5 = assign53260_e87648_d_n5;
        locals.var_delclm_dn6 = assign53260_e87648_d_n6;
        locals.var_delclm_dn7 = assign53260_e87648_d_n7;
        locals.var_delclm_dn8 = assign53260_e87648_d_n8;
        locals.var_delclm_dn9 = assign53260_e87648_d_n9;
        locals.var_delclm_dn10 = assign53260_e87648_d_n10;
        locals.var_delclm_dn11 = assign53260_e87648_d_n11;

        let (assign53270_e87661, assign53270_e87661_d_n3, assign53270_e87661_d_n4, assign53270_e87661_d_n5, assign53270_e87661_d_n6, assign53270_e87661_d_n7, assign53270_e87661_d_n8, assign53270_e87661_d_n9, assign53270_e87661_d_n10, assign53270_e87661_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign53270_e87653: f64 = (locals.var_vt / 1.602176462e-19);
        let assign53270_e87656: f64 = (locals.var_cox + locals.var_cdep);
        let assign53270_e87658: f64 = (assign53270_e87656 + locals.var_cit_i);
        let assign53270_e87659: f64 = (assign53270_e87653 * assign53270_e87658);
        (assign53270_e87659, (assign53270_e87653 * locals.var_cdep_dn3), (((locals.var_vt_dn4 / 1.602176462e-19) * assign53270_e87658) + (assign53270_e87653 * locals.var_cdep_dn4)), (((locals.var_vt_dn5 / 1.602176462e-19) * assign53270_e87658) + (assign53270_e87653 * locals.var_cdep_dn5)), (assign53270_e87653 * locals.var_cdep_dn6), (assign53270_e87653 * locals.var_cdep_dn7), (assign53270_e87653 * locals.var_cdep_dn8), (assign53270_e87653 * locals.var_cdep_dn9), (assign53270_e87653 * locals.var_cdep_dn10), (assign53270_e87653 * locals.var_cdep_dn11),)
    } else {
        (locals.var_nstar, locals.var_nstar_dn3, locals.var_nstar_dn4, locals.var_nstar_dn5, locals.var_nstar_dn6, locals.var_nstar_dn7, locals.var_nstar_dn8, locals.var_nstar_dn9, locals.var_nstar_dn10, locals.var_nstar_dn11,)
    }
};
        locals.var_nstar = assign53270_e87661;
        locals.var_nstar_dn3 = assign53270_e87661_d_n3;
        locals.var_nstar_dn4 = assign53270_e87661_d_n4;
        locals.var_nstar_dn5 = assign53270_e87661_d_n5;
        locals.var_nstar_dn6 = assign53270_e87661_d_n6;
        locals.var_nstar_dn7 = assign53270_e87661_d_n7;
        locals.var_nstar_dn8 = assign53270_e87661_d_n8;
        locals.var_nstar_dn9 = assign53270_e87661_d_n9;
        locals.var_nstar_dn10 = assign53270_e87661_d_n10;
        locals.var_nstar_dn11 = assign53270_e87661_d_n11;

        let (assign53280_e87680, assign53280_e87680_d_n3, assign53280_e87680_d_n4, assign53280_e87680_d_n5, assign53280_e87680_d_n6, assign53280_e87680_d_n7, assign53280_e87680_d_n8, assign53280_e87680_d_n9, assign53280_e87680_d_n10, assign53280_e87680_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign53280_e87666: f64 = (2.0 * locals.var_nq);
        let assign53280_e87668: f64 = (assign53280_e87666 * locals.var_cox);
        let assign53280_e87670: f64 = (assign53280_e87668 * locals.var_vt);
        let assign53280_e87672: f64 = (assign53280_e87670 * locals.var_qdeff);
        let assign53280_e87674: f64 = (assign53280_e87672 * locals.var_mnud1);
        let assign53280_e87676: f64 = (assign53280_e87674 * locals.var_mnud);
        let assign53280_e87678: f64 = (assign53280_e87676 / 1.602176462e-19);
        (assign53280_e87678, ((((((((((2.0 * locals.var_nq_dn3) * locals.var_cox) * locals.var_vt) * locals.var_qdeff) + (assign53280_e87670 * locals.var_qdeff_dn3)) * locals.var_mnud1) + (assign53280_e87672 * locals.var_mnud1_dn3)) * locals.var_mnud) + (assign53280_e87674 * locals.var_mnud_dn3)) / 1.602176462e-19), (((((((((((2.0 * locals.var_nq_dn4) * locals.var_cox) * locals.var_vt) + (assign53280_e87668 * locals.var_vt_dn4)) * locals.var_qdeff) + (assign53280_e87670 * locals.var_qdeff_dn4)) * locals.var_mnud1) + (assign53280_e87672 * locals.var_mnud1_dn4)) * locals.var_mnud) + (assign53280_e87674 * locals.var_mnud_dn4)) / 1.602176462e-19), (((((((((((2.0 * locals.var_nq_dn5) * locals.var_cox) * locals.var_vt) + (assign53280_e87668 * locals.var_vt_dn5)) * locals.var_qdeff) + (assign53280_e87670 * locals.var_qdeff_dn5)) * locals.var_mnud1) + (assign53280_e87672 * locals.var_mnud1_dn5)) * locals.var_mnud) + (assign53280_e87674 * locals.var_mnud_dn5)) / 1.602176462e-19), ((((((((((2.0 * locals.var_nq_dn6) * locals.var_cox) * locals.var_vt) * locals.var_qdeff) + (assign53280_e87670 * locals.var_qdeff_dn6)) * locals.var_mnud1) + (assign53280_e87672 * locals.var_mnud1_dn6)) * locals.var_mnud) + (assign53280_e87674 * locals.var_mnud_dn6)) / 1.602176462e-19), ((((((((((2.0 * locals.var_nq_dn7) * locals.var_cox) * locals.var_vt) * locals.var_qdeff) + (assign53280_e87670 * locals.var_qdeff_dn7)) * locals.var_mnud1) + (assign53280_e87672 * locals.var_mnud1_dn7)) * locals.var_mnud) + (assign53280_e87674 * locals.var_mnud_dn7)) / 1.602176462e-19), ((((((((((2.0 * locals.var_nq_dn8) * locals.var_cox) * locals.var_vt) * locals.var_qdeff) + (assign53280_e87670 * locals.var_qdeff_dn8)) * locals.var_mnud1) + (assign53280_e87672 * locals.var_mnud1_dn8)) * locals.var_mnud) + (assign53280_e87674 * locals.var_mnud_dn8)) / 1.602176462e-19), ((((((((((2.0 * locals.var_nq_dn9) * locals.var_cox) * locals.var_vt) * locals.var_qdeff) + (assign53280_e87670 * locals.var_qdeff_dn9)) * locals.var_mnud1) + (assign53280_e87672 * locals.var_mnud1_dn9)) * locals.var_mnud) + (assign53280_e87674 * locals.var_mnud_dn9)) / 1.602176462e-19), ((((((((((2.0 * locals.var_nq_dn10) * locals.var_cox) * locals.var_vt) * locals.var_qdeff) + (assign53280_e87670 * locals.var_qdeff_dn10)) * locals.var_mnud1) + (assign53280_e87672 * locals.var_mnud1_dn10)) * locals.var_mnud) + (assign53280_e87674 * locals.var_mnud_dn10)) / 1.602176462e-19), ((((((((((2.0 * locals.var_nq_dn11) * locals.var_cox) * locals.var_vt) * locals.var_qdeff) + (assign53280_e87670 * locals.var_qdeff_dn11)) * locals.var_mnud1) + (assign53280_e87672 * locals.var_mnud1_dn11)) * locals.var_mnud) + (assign53280_e87674 * locals.var_mnud_dn11)) / 1.602176462e-19),)
    } else {
        (locals.var_nl, locals.var_nl_dn3, locals.var_nl_dn4, locals.var_nl_dn5, locals.var_nl_dn6, locals.var_nl_dn7, locals.var_nl_dn8, locals.var_nl_dn9, locals.var_nl_dn10, locals.var_nl_dn11,)
    }
};
        locals.var_nl = assign53280_e87680;
        locals.var_nl_dn3 = assign53280_e87680_d_n3;
        locals.var_nl_dn4 = assign53280_e87680_d_n4;
        locals.var_nl_dn5 = assign53280_e87680_d_n5;
        locals.var_nl_dn6 = assign53280_e87680_d_n6;
        locals.var_nl_dn7 = assign53280_e87680_d_n7;
        locals.var_nl_dn8 = assign53280_e87680_d_n8;
        locals.var_nl_dn9 = assign53280_e87680_d_n9;
        locals.var_nl_dn10 = assign53280_e87680_d_n10;
        locals.var_nl_dn11 = assign53280_e87680_d_n11;

        let (assign53290_e87696, assign53290_e87696_d_n3, assign53290_e87696_d_n4, assign53290_e87696_d_n5, assign53290_e87696_d_n6, assign53290_e87696_d_n7, assign53290_e87696_d_n8, assign53290_e87696_d_n9, assign53290_e87696_d_n10, assign53290_e87696_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign53290_e87685: f64 = (1.602176462e-19 * 1.602176462e-19);
        let assign53290_e87687: f64 = (assign53290_e87685 * 1.602176462e-19);
        let assign53290_e87689: f64 = (assign53290_e87687 * locals.var_vt);
        let assign53290_e87691: f64 = (locals.var_ids).abs();
        let assign53290_e87692: f64 = (assign53290_e87689 * assign53290_e87691);
        let assign53290_e87694: f64 = (assign53290_e87692 * locals.var_ueff);
        (assign53290_e87694, (((assign53290_e87689 * if locals.var_ids >= 0.0 { locals.var_ids_dn3 } else { (-locals.var_ids_dn3) }) * locals.var_ueff) + (assign53290_e87692 * locals.var_ueff_dn3)), (((((assign53290_e87687 * locals.var_vt_dn4) * assign53290_e87691) + (assign53290_e87689 * if locals.var_ids >= 0.0 { locals.var_ids_dn4 } else { (-locals.var_ids_dn4) })) * locals.var_ueff) + (assign53290_e87692 * locals.var_ueff_dn4)), (((((assign53290_e87687 * locals.var_vt_dn5) * assign53290_e87691) + (assign53290_e87689 * if locals.var_ids >= 0.0 { locals.var_ids_dn5 } else { (-locals.var_ids_dn5) })) * locals.var_ueff) + (assign53290_e87692 * locals.var_ueff_dn5)), (((assign53290_e87689 * if locals.var_ids >= 0.0 { locals.var_ids_dn6 } else { (-locals.var_ids_dn6) }) * locals.var_ueff) + (assign53290_e87692 * locals.var_ueff_dn6)), (((assign53290_e87689 * if locals.var_ids >= 0.0 { locals.var_ids_dn7 } else { (-locals.var_ids_dn7) }) * locals.var_ueff) + (assign53290_e87692 * locals.var_ueff_dn7)), (((assign53290_e87689 * if locals.var_ids >= 0.0 { locals.var_ids_dn8 } else { (-locals.var_ids_dn8) }) * locals.var_ueff) + (assign53290_e87692 * locals.var_ueff_dn8)), (((assign53290_e87689 * if locals.var_ids >= 0.0 { locals.var_ids_dn9 } else { (-locals.var_ids_dn9) }) * locals.var_ueff) + (assign53290_e87692 * locals.var_ueff_dn9)), (((assign53290_e87689 * if locals.var_ids >= 0.0 { locals.var_ids_dn10 } else { (-locals.var_ids_dn10) }) * locals.var_ueff) + (assign53290_e87692 * locals.var_ueff_dn10)), (((assign53290_e87689 * if locals.var_ids >= 0.0 { locals.var_ids_dn11 } else { (-locals.var_ids_dn11) }) * locals.var_ueff) + (assign53290_e87692 * locals.var_ueff_dn11)),)
    } else {
        (locals.var_t0a, locals.var_t0a_dn3, locals.var_t0a_dn4, locals.var_t0a_dn5, locals.var_t0a_dn6, locals.var_t0a_dn7, locals.var_t0a_dn8, locals.var_t0a_dn9, locals.var_t0a_dn10, locals.var_t0a_dn11,)
    }
};
        locals.var_t0a = assign53290_e87696;
        locals.var_t0a_dn3 = assign53290_e87696_d_n3;
        locals.var_t0a_dn4 = assign53290_e87696_d_n4;
        locals.var_t0a_dn5 = assign53290_e87696_d_n5;
        locals.var_t0a_dn6 = assign53290_e87696_d_n6;
        locals.var_t0a_dn7 = assign53290_e87696_d_n7;
        locals.var_t0a_dn8 = assign53290_e87696_d_n8;
        locals.var_t0a_dn9 = assign53290_e87696_d_n9;
        locals.var_t0a_dn10 = assign53290_e87696_d_n10;
        locals.var_t0a_dn11 = assign53290_e87696_d_n11;

        let (assign53300_e87707, assign53300_e87707_d_n3, assign53300_e87707_d_n4, assign53300_e87707_d_n5, assign53300_e87707_d_n6, assign53300_e87707_d_n7, assign53300_e87707_d_n8, assign53300_e87707_d_n9, assign53300_e87707_d_n10, assign53300_e87707_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign53300_e87701: f64 = (1.602176462e-19 * locals.var_vt);
        let assign53300_e87703: f64 = (assign53300_e87701 * locals.var_ids);
        let assign53300_e87705: f64 = (assign53300_e87703 * locals.var_ids);
        (assign53300_e87705, (((assign53300_e87701 * locals.var_ids_dn3) * locals.var_ids) + (assign53300_e87703 * locals.var_ids_dn3)), (((((1.602176462e-19 * locals.var_vt_dn4) * locals.var_ids) + (assign53300_e87701 * locals.var_ids_dn4)) * locals.var_ids) + (assign53300_e87703 * locals.var_ids_dn4)), (((((1.602176462e-19 * locals.var_vt_dn5) * locals.var_ids) + (assign53300_e87701 * locals.var_ids_dn5)) * locals.var_ids) + (assign53300_e87703 * locals.var_ids_dn5)), (((assign53300_e87701 * locals.var_ids_dn6) * locals.var_ids) + (assign53300_e87703 * locals.var_ids_dn6)), (((assign53300_e87701 * locals.var_ids_dn7) * locals.var_ids) + (assign53300_e87703 * locals.var_ids_dn7)), (((assign53300_e87701 * locals.var_ids_dn8) * locals.var_ids) + (assign53300_e87703 * locals.var_ids_dn8)), (((assign53300_e87701 * locals.var_ids_dn9) * locals.var_ids) + (assign53300_e87703 * locals.var_ids_dn9)), (((assign53300_e87701 * locals.var_ids_dn10) * locals.var_ids) + (assign53300_e87703 * locals.var_ids_dn10)), (((assign53300_e87701 * locals.var_ids_dn11) * locals.var_ids) + (assign53300_e87703 * locals.var_ids_dn11)),)
    } else {
        (locals.var_t0b, locals.var_t0b_dn3, locals.var_t0b_dn4, locals.var_t0b_dn5, locals.var_t0b_dn6, locals.var_t0b_dn7, locals.var_t0b_dn8, locals.var_t0b_dn9, locals.var_t0b_dn10, locals.var_t0b_dn11,)
    }
};
        locals.var_t0b = assign53300_e87707;
        locals.var_t0b_dn3 = assign53300_e87707_d_n3;
        locals.var_t0b_dn4 = assign53300_e87707_d_n4;
        locals.var_t0b_dn5 = assign53300_e87707_d_n5;
        locals.var_t0b_dn6 = assign53300_e87707_d_n6;
        locals.var_t0b_dn7 = assign53300_e87707_d_n7;
        locals.var_t0b_dn8 = assign53300_e87707_d_n8;
        locals.var_t0b_dn9 = assign53300_e87707_d_n9;
        locals.var_t0b_dn10 = assign53300_e87707_d_n10;
        locals.var_t0b_dn11 = assign53300_e87707_d_n11;

        let (assign53310_e87722, assign53310_e87722_d_n3, assign53310_e87722_d_n4, assign53310_e87722_d_n5, assign53310_e87722_d_n6, assign53310_e87722_d_n7, assign53310_e87722_d_n8, assign53310_e87722_d_n9, assign53310_e87722_d_n10, assign53310_e87722_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign53310_e87713: f64 = (p.p1013 * locals.var_nl);
        let assign53310_e87714: f64 = (p.p1012 + assign53310_e87713);
        let assign53310_e87717: f64 = (p.p1014 * locals.var_nl);
        let assign53310_e87719: f64 = (assign53310_e87717 * locals.var_nl);
        let assign53310_e87720: f64 = (assign53310_e87714 + assign53310_e87719);
        (assign53310_e87720, ((p.p1013 * locals.var_nl_dn3) + (((p.p1014 * locals.var_nl_dn3) * locals.var_nl) + (assign53310_e87717 * locals.var_nl_dn3))), ((p.p1013 * locals.var_nl_dn4) + (((p.p1014 * locals.var_nl_dn4) * locals.var_nl) + (assign53310_e87717 * locals.var_nl_dn4))), ((p.p1013 * locals.var_nl_dn5) + (((p.p1014 * locals.var_nl_dn5) * locals.var_nl) + (assign53310_e87717 * locals.var_nl_dn5))), ((p.p1013 * locals.var_nl_dn6) + (((p.p1014 * locals.var_nl_dn6) * locals.var_nl) + (assign53310_e87717 * locals.var_nl_dn6))), ((p.p1013 * locals.var_nl_dn7) + (((p.p1014 * locals.var_nl_dn7) * locals.var_nl) + (assign53310_e87717 * locals.var_nl_dn7))), ((p.p1013 * locals.var_nl_dn8) + (((p.p1014 * locals.var_nl_dn8) * locals.var_nl) + (assign53310_e87717 * locals.var_nl_dn8))), ((p.p1013 * locals.var_nl_dn9) + (((p.p1014 * locals.var_nl_dn9) * locals.var_nl) + (assign53310_e87717 * locals.var_nl_dn9))), ((p.p1013 * locals.var_nl_dn10) + (((p.p1014 * locals.var_nl_dn10) * locals.var_nl) + (assign53310_e87717 * locals.var_nl_dn10))), ((p.p1013 * locals.var_nl_dn11) + (((p.p1014 * locals.var_nl_dn11) * locals.var_nl) + (assign53310_e87717 * locals.var_nl_dn11))),)
    } else {
        (locals.var_t0c, locals.var_t0c_dn3, locals.var_t0c_dn4, locals.var_t0c_dn5, locals.var_t0c_dn6, locals.var_t0c_dn7, locals.var_t0c_dn8, locals.var_t0c_dn9, locals.var_t0c_dn10, locals.var_t0c_dn11,)
    }
};
        locals.var_t0c = assign53310_e87722;
        locals.var_t0c_dn3 = assign53310_e87722_d_n3;
        locals.var_t0c_dn4 = assign53310_e87722_d_n4;
        locals.var_t0c_dn5 = assign53310_e87722_d_n5;
        locals.var_t0c_dn6 = assign53310_e87722_d_n6;
        locals.var_t0c_dn7 = assign53310_e87722_d_n7;
        locals.var_t0c_dn8 = assign53310_e87722_d_n8;
        locals.var_t0c_dn9 = assign53310_e87722_d_n9;
        locals.var_t0c_dn10 = assign53310_e87722_d_n10;
        locals.var_t0c_dn11 = assign53310_e87722_d_n11;

        let (assign53320_e87733, assign53320_e87733_d_n3, assign53320_e87733_d_n4, assign53320_e87733_d_n5, assign53320_e87733_d_n6, assign53320_e87733_d_n7, assign53320_e87733_d_n8, assign53320_e87733_d_n9, assign53320_e87733_d_n10, assign53320_e87733_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign53320_e87727: f64 = (locals.var_nl + locals.var_nstar);
        let assign53320_e87730: f64 = (locals.var_nl + locals.var_nstar);
        let assign53320_e87731: f64 = (assign53320_e87727 * assign53320_e87730);
        (assign53320_e87731, (((locals.var_nl_dn3 + locals.var_nstar_dn3) * assign53320_e87730) + (assign53320_e87727 * (locals.var_nl_dn3 + locals.var_nstar_dn3))), (((locals.var_nl_dn4 + locals.var_nstar_dn4) * assign53320_e87730) + (assign53320_e87727 * (locals.var_nl_dn4 + locals.var_nstar_dn4))), (((locals.var_nl_dn5 + locals.var_nstar_dn5) * assign53320_e87730) + (assign53320_e87727 * (locals.var_nl_dn5 + locals.var_nstar_dn5))), (((locals.var_nl_dn6 + locals.var_nstar_dn6) * assign53320_e87730) + (assign53320_e87727 * (locals.var_nl_dn6 + locals.var_nstar_dn6))), (((locals.var_nl_dn7 + locals.var_nstar_dn7) * assign53320_e87730) + (assign53320_e87727 * (locals.var_nl_dn7 + locals.var_nstar_dn7))), (((locals.var_nl_dn8 + locals.var_nstar_dn8) * assign53320_e87730) + (assign53320_e87727 * (locals.var_nl_dn8 + locals.var_nstar_dn8))), (((locals.var_nl_dn9 + locals.var_nstar_dn9) * assign53320_e87730) + (assign53320_e87727 * (locals.var_nl_dn9 + locals.var_nstar_dn9))), (((locals.var_nl_dn10 + locals.var_nstar_dn10) * assign53320_e87730) + (assign53320_e87727 * (locals.var_nl_dn10 + locals.var_nstar_dn10))), (((locals.var_nl_dn11 + locals.var_nstar_dn11) * assign53320_e87730) + (assign53320_e87727 * (locals.var_nl_dn11 + locals.var_nstar_dn11))),)
    } else {
        (locals.var_t0d, locals.var_t0d_dn3, locals.var_t0d_dn4, locals.var_t0d_dn5, locals.var_t0d_dn6, locals.var_t0d_dn7, locals.var_t0d_dn8, locals.var_t0d_dn9, locals.var_t0d_dn10, locals.var_t0d_dn11,)
    }
};
        locals.var_t0d = assign53320_e87733;
        locals.var_t0d_dn3 = assign53320_e87733_d_n3;
        locals.var_t0d_dn4 = assign53320_e87733_d_n4;
        locals.var_t0d_dn5 = assign53320_e87733_d_n5;
        locals.var_t0d_dn6 = assign53320_e87733_d_n6;
        locals.var_t0d_dn7 = assign53320_e87733_d_n7;
        locals.var_t0d_dn8 = assign53320_e87733_d_n8;
        locals.var_t0d_dn9 = assign53320_e87733_d_n9;
        locals.var_t0d_dn10 = assign53320_e87733_d_n10;
        locals.var_t0d_dn11 = assign53320_e87733_d_n11;

        let (assign53330_e87742, assign53330_e87742_d_n4, assign53330_e87742_d_n5,) = {
    if (locals.var_guard492 == 0.0) {
        let assign53330_e87738: f64 = (p.p1012 * 1.602176462e-19);
        let assign53330_e87740: f64 = (assign53330_e87738 * locals.var_vt);
        (assign53330_e87740, (assign53330_e87738 * locals.var_vt_dn4), (assign53330_e87738 * locals.var_vt_dn5),)
    } else {
        (locals.var_t0e, locals.var_t0e_dn4, locals.var_t0e_dn5,)
    }
};
        locals.var_t0e = assign53330_e87742;
        locals.var_t0e_dn4 = assign53330_e87742_d_n4;
        locals.var_t0e_dn5 = assign53330_e87742_d_n5;

        let assign53340_e87745: f64 = if p.p1319 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard822 = assign53340_e87745;

        let (assign53350_e87752,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        (p.p1320,)
    } else {
        (locals.var_lh1,)
    }
};
        locals.var_lh1 = assign53350_e87752;

        let assign53360_e87755: f64 = if locals.var_leff > locals.var_lh1 { 1.0 } else { 0.0 };
        locals.var_guard823 = assign53360_e87755;

        let (assign53370_e87766, assign53370_e87766_d_n3, assign53370_e87766_d_n4, assign53370_e87766_d_n5, assign53370_e87766_d_n6, assign53370_e87766_d_n7, assign53370_e87766_d_n8, assign53370_e87766_d_n9, assign53370_e87766_d_n10, assign53370_e87766_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard823 != 0.0)) {
        let assign53370_e87764: f64 = (locals.var_leff - locals.var_lh1);
        (assign53370_e87764, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign53370_e87766;
        locals.var_t0_dn3 = assign53370_e87766_d_n3;
        locals.var_t0_dn4 = assign53370_e87766_d_n4;
        locals.var_t0_dn5 = assign53370_e87766_d_n5;
        locals.var_t0_dn6 = assign53370_e87766_d_n6;
        locals.var_t0_dn7 = assign53370_e87766_d_n7;
        locals.var_t0_dn8 = assign53370_e87766_d_n8;
        locals.var_t0_dn9 = assign53370_e87766_d_n9;
        locals.var_t0_dn10 = assign53370_e87766_d_n10;
        locals.var_t0_dn11 = assign53370_e87766_d_n11;

        let (assign53380_e87776,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard823 == 0.0)) {
        (locals.var_leff,)
    } else {
        (locals.var_lh1,)
    }
};
        locals.var_lh1 = assign53380_e87776;

        let (assign53390_e87786, assign53390_e87786_d_n3, assign53390_e87786_d_n4, assign53390_e87786_d_n5, assign53390_e87786_d_n6, assign53390_e87786_d_n7, assign53390_e87786_d_n8, assign53390_e87786_d_n9, assign53390_e87786_d_n10, assign53390_e87786_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard823 == 0.0)) {
        (locals.var_lh1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign53390_e87786;
        locals.var_t0_dn3 = assign53390_e87786_d_n3;
        locals.var_t0_dn4 = assign53390_e87786_d_n4;
        locals.var_t0_dn5 = assign53390_e87786_d_n5;
        locals.var_t0_dn6 = assign53390_e87786_d_n6;
        locals.var_t0_dn7 = assign53390_e87786_d_n7;
        locals.var_t0_dn8 = assign53390_e87786_d_n8;
        locals.var_t0_dn9 = assign53390_e87786_d_n9;
        locals.var_t0_dn10 = assign53390_e87786_d_n10;
        locals.var_t0_dn11 = assign53390_e87786_d_n11;

        let assign53400_e87790: f64 = (locals.var_t0 / 2.0);
        let assign53400_e87791: f64 = if p.p1015 >= assign53400_e87790 { 1.0 } else { 0.0 };
        locals.var_guard824 = assign53400_e87791;

        let (assign53410_e87800,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard824 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_lintnoi_i,)
    }
};
        locals.var_lintnoi_i = assign53410_e87800;

    }

    pub(super) fn stamp_transient_block_181(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign53420_e87810,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard824 == 0.0)) {
        (p.p1015,)
    } else {
        (locals.var_lintnoi_i,)
    }
};
        locals.var_lintnoi_i = assign53420_e87810;

        let (assign53430_e87817,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        (locals.var_leff,)
    } else {
        (locals.var_leffnoih,)
    }
};
        locals.var_leffnoih = assign53430_e87817;

        let (assign53440_e87828, assign53440_e87828_d_n3, assign53440_e87828_d_n4, assign53440_e87828_d_n5, assign53440_e87828_d_n6, assign53440_e87828_d_n7, assign53440_e87828_d_n8, assign53440_e87828_d_n9, assign53440_e87828_d_n10, assign53440_e87828_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        let assign53440_e87824: f64 = (locals.var_vg - locals.var_vfb_i);
        let assign53440_e87826: f64 = (assign53440_e87824 / locals.var_vt);
        (assign53440_e87826, ((-locals.var_vfb_i_dn3) / locals.var_vt), ((((-locals.var_vfb_i_dn4) * locals.var_vt) - (assign53440_e87824 * locals.var_vt_dn4)) / (locals.var_vt * locals.var_vt)), ((((-locals.var_vfb_i_dn5) * locals.var_vt) - (assign53440_e87824 * locals.var_vt_dn5)) / (locals.var_vt * locals.var_vt)), ((-locals.var_vfb_i_dn6) / locals.var_vt), ((-locals.var_vfb_i_dn7) / locals.var_vt), ((locals.var_vg_dn8 - locals.var_vfb_i_dn8) / locals.var_vt), ((-locals.var_vfb_i_dn9) / locals.var_vt), ((locals.var_vg_dn10 - locals.var_vfb_i_dn10) / locals.var_vt), ((-locals.var_vfb_i_dn11) / locals.var_vt),)
    } else {
        (locals.var_vgfbh, locals.var_vgfbh_dn3, locals.var_vgfbh_dn4, locals.var_vgfbh_dn5, locals.var_vgfbh_dn6, locals.var_vgfbh_dn7, locals.var_vgfbh_dn8, locals.var_vgfbh_dn9, locals.var_vgfbh_dn10, locals.var_vgfbh_dn11,)
    }
};
        locals.var_vgfbh = assign53440_e87828;
        locals.var_vgfbh_dn3 = assign53440_e87828_d_n3;
        locals.var_vgfbh_dn4 = assign53440_e87828_d_n4;
        locals.var_vgfbh_dn5 = assign53440_e87828_d_n5;
        locals.var_vgfbh_dn6 = assign53440_e87828_d_n6;
        locals.var_vgfbh_dn7 = assign53440_e87828_d_n7;
        locals.var_vgfbh_dn8 = assign53440_e87828_d_n8;
        locals.var_vgfbh_dn9 = assign53440_e87828_d_n9;
        locals.var_vgfbh_dn10 = assign53440_e87828_d_n10;
        locals.var_vgfbh_dn11 = assign53440_e87828_d_n11;

        let (assign53450_e87846, assign53450_e87846_d_n4, assign53450_e87846_d_n5,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        let assign53450_e87835: f64 = (2.0 * 1.602176462e-19);
        let assign53450_e87837: f64 = (assign53450_e87835 * locals.var_epssi);
        let assign53450_e87839: f64 = (assign53450_e87837 * p.p1322);
        let assign53450_e87841: f64 = (assign53450_e87839 / locals.var_vt);
        let assign53450_e87842: f64 = (assign53450_e87841).sqrt();
        let assign53450_e87844: f64 = (assign53450_e87842 / locals.var_cox);
        (assign53450_e87844, (((-((assign53450_e87839 * locals.var_vt_dn4) / (locals.var_vt * locals.var_vt))) / (2.0 * assign53450_e87842)) / locals.var_cox), (((-((assign53450_e87839 * locals.var_vt_dn5) / (locals.var_vt * locals.var_vt))) / (2.0 * assign53450_e87842)) / locals.var_cox),)
    } else {
        (locals.var_gam_h, locals.var_gam_h_dn4, locals.var_gam_h_dn5,)
    }
};
        locals.var_gam_h = assign53450_e87846;
        locals.var_gam_h_dn4 = assign53450_e87846_d_n4;
        locals.var_gam_h_dn5 = assign53450_e87846_d_n5;

        let (assign53460_e87856, assign53460_e87856_d_n3, assign53460_e87856_d_n4, assign53460_e87856_d_n5, assign53460_e87856_d_n6, assign53460_e87856_d_n7, assign53460_e87856_d_n8, assign53460_e87856_d_n9, assign53460_e87856_d_n10, assign53460_e87856_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        let assign53460_e87853: f64 = (p.p1322 / locals.var_ni);
        let assign53460_e87854: f64 = (assign53460_e87853).ln();
        (assign53460_e87854, ((-((p.p1322 * locals.var_ni_dn3) / (locals.var_ni * locals.var_ni))) / assign53460_e87853), ((-((p.p1322 * locals.var_ni_dn4) / (locals.var_ni * locals.var_ni))) / assign53460_e87853), ((-((p.p1322 * locals.var_ni_dn5) / (locals.var_ni * locals.var_ni))) / assign53460_e87853), ((-((p.p1322 * locals.var_ni_dn6) / (locals.var_ni * locals.var_ni))) / assign53460_e87853), ((-((p.p1322 * locals.var_ni_dn7) / (locals.var_ni * locals.var_ni))) / assign53460_e87853), ((-((p.p1322 * locals.var_ni_dn8) / (locals.var_ni * locals.var_ni))) / assign53460_e87853), ((-((p.p1322 * locals.var_ni_dn9) / (locals.var_ni * locals.var_ni))) / assign53460_e87853), ((-((p.p1322 * locals.var_ni_dn10) / (locals.var_ni * locals.var_ni))) / assign53460_e87853), ((-((p.p1322 * locals.var_ni_dn11) / (locals.var_ni * locals.var_ni))) / assign53460_e87853),)
    } else {
        (locals.var_phib_h, locals.var_phib_h_dn3, locals.var_phib_h_dn4, locals.var_phib_h_dn5, locals.var_phib_h_dn6, locals.var_phib_h_dn7, locals.var_phib_h_dn8, locals.var_phib_h_dn9, locals.var_phib_h_dn10, locals.var_phib_h_dn11,)
    }
};
        locals.var_phib_h = assign53460_e87856;
        locals.var_phib_h_dn3 = assign53460_e87856_d_n3;
        locals.var_phib_h_dn4 = assign53460_e87856_d_n4;
        locals.var_phib_h_dn5 = assign53460_e87856_d_n5;
        locals.var_phib_h_dn6 = assign53460_e87856_d_n6;
        locals.var_phib_h_dn7 = assign53460_e87856_d_n7;
        locals.var_phib_h_dn8 = assign53460_e87856_d_n8;
        locals.var_phib_h_dn9 = assign53460_e87856_d_n9;
        locals.var_phib_h_dn10 = assign53460_e87856_d_n10;
        locals.var_phib_h_dn11 = assign53460_e87856_d_n11;

        let (assign53470_e87865, assign53470_e87865_d_n3, assign53470_e87865_d_n4, assign53470_e87865_d_n5, assign53470_e87865_d_n6, assign53470_e87865_d_n7, assign53470_e87865_d_n8, assign53470_e87865_d_n9, assign53470_e87865_d_n10, assign53470_e87865_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        let assign53470_e87863: f64 = 1.0;
        (assign53470_e87863, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign53470_e87865;
        locals.var_t1_dn3 = assign53470_e87865_d_n3;
        locals.var_t1_dn4 = assign53470_e87865_d_n4;
        locals.var_t1_dn5 = assign53470_e87865_d_n5;
        locals.var_t1_dn6 = assign53470_e87865_d_n6;
        locals.var_t1_dn7 = assign53470_e87865_d_n7;
        locals.var_t1_dn8 = assign53470_e87865_d_n8;
        locals.var_t1_dn9 = assign53470_e87865_d_n9;
        locals.var_t1_dn10 = assign53470_e87865_d_n10;
        locals.var_t1_dn11 = assign53470_e87865_d_n11;

        let (assign53480_e87874, assign53480_e87874_d_n3, assign53480_e87874_d_n4, assign53480_e87874_d_n5, assign53480_e87874_d_n6, assign53480_e87874_d_n7, assign53480_e87874_d_n8, assign53480_e87874_d_n9, assign53480_e87874_d_n10, assign53480_e87874_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        let assign53480_e87872: f64 = (locals.var_vgfbh / locals.var_t1);
        (assign53480_e87872, (((locals.var_vgfbh_dn3 * locals.var_t1) - (locals.var_vgfbh * locals.var_t1_dn3)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfbh_dn4 * locals.var_t1) - (locals.var_vgfbh * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfbh_dn5 * locals.var_t1) - (locals.var_vgfbh * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfbh_dn6 * locals.var_t1) - (locals.var_vgfbh * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfbh_dn7 * locals.var_t1) - (locals.var_vgfbh * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfbh_dn8 * locals.var_t1) - (locals.var_vgfbh * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfbh_dn9 * locals.var_t1) - (locals.var_vgfbh * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfbh_dn10 * locals.var_t1) - (locals.var_vgfbh * locals.var_t1_dn10)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfbh_dn11 * locals.var_t1) - (locals.var_vgfbh * locals.var_t1_dn11)) / (locals.var_t1 * locals.var_t1)),)
    } else {
        (locals.var_vgfbpd, locals.var_vgfbpd_dn3, locals.var_vgfbpd_dn4, locals.var_vgfbpd_dn5, locals.var_vgfbpd_dn6, locals.var_vgfbpd_dn7, locals.var_vgfbpd_dn8, locals.var_vgfbpd_dn9, locals.var_vgfbpd_dn10, locals.var_vgfbpd_dn11,)
    }
};
        locals.var_vgfbpd = assign53480_e87874;
        locals.var_vgfbpd_dn3 = assign53480_e87874_d_n3;
        locals.var_vgfbpd_dn4 = assign53480_e87874_d_n4;
        locals.var_vgfbpd_dn5 = assign53480_e87874_d_n5;
        locals.var_vgfbpd_dn6 = assign53480_e87874_d_n6;
        locals.var_vgfbpd_dn7 = assign53480_e87874_d_n7;
        locals.var_vgfbpd_dn8 = assign53480_e87874_d_n8;
        locals.var_vgfbpd_dn9 = assign53480_e87874_d_n9;
        locals.var_vgfbpd_dn10 = assign53480_e87874_d_n10;
        locals.var_vgfbpd_dn11 = assign53480_e87874_d_n11;

        let (assign53490_e87883, assign53490_e87883_d_n3, assign53490_e87883_d_n4, assign53490_e87883_d_n5, assign53490_e87883_d_n6, assign53490_e87883_d_n7, assign53490_e87883_d_n8, assign53490_e87883_d_n9, assign53490_e87883_d_n10, assign53490_e87883_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        let assign53490_e87881: f64 = (locals.var_gam_h / locals.var_t1);
        (assign53490_e87881, (-((locals.var_gam_h * locals.var_t1_dn3) / (locals.var_t1 * locals.var_t1))), (((locals.var_gam_h_dn4 * locals.var_t1) - (locals.var_gam_h * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_h_dn5 * locals.var_t1) - (locals.var_gam_h * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)), (-((locals.var_gam_h * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((locals.var_gam_h * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((locals.var_gam_h * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1))), (-((locals.var_gam_h * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1))), (-((locals.var_gam_h * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((locals.var_gam_h * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_gammapd, locals.var_gammapd_dn3, locals.var_gammapd_dn4, locals.var_gammapd_dn5, locals.var_gammapd_dn6, locals.var_gammapd_dn7, locals.var_gammapd_dn8, locals.var_gammapd_dn9, locals.var_gammapd_dn10, locals.var_gammapd_dn11,)
    }
};
        locals.var_gammapd = assign53490_e87883;
        locals.var_gammapd_dn3 = assign53490_e87883_d_n3;
        locals.var_gammapd_dn4 = assign53490_e87883_d_n4;
        locals.var_gammapd_dn5 = assign53490_e87883_d_n5;
        locals.var_gammapd_dn6 = assign53490_e87883_d_n6;
        locals.var_gammapd_dn7 = assign53490_e87883_d_n7;
        locals.var_gammapd_dn8 = assign53490_e87883_d_n8;
        locals.var_gammapd_dn9 = assign53490_e87883_d_n9;
        locals.var_gammapd_dn10 = assign53490_e87883_d_n10;
        locals.var_gammapd_dn11 = assign53490_e87883_d_n11;

        let (assign53500_e87900, assign53500_e87900_d_n3, assign53500_e87900_d_n4, assign53500_e87900_d_n5, assign53500_e87900_d_n6, assign53500_e87900_d_n7, assign53500_e87900_d_n8, assign53500_e87900_d_n9, assign53500_e87900_d_n10, assign53500_e87900_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        let assign53500_e87890: f64 = (0.5 * locals.var_vgfbpd);
        let assign53500_e87895: f64 = (locals.var_gammapd / 1.4142135623730951);
        let assign53500_e87896: f64 = (1.0 + assign53500_e87895);
        let assign53500_e87897: f64 = (3.0 * assign53500_e87896);
        let assign53500_e87898: f64 = (assign53500_e87890 - assign53500_e87897);
        (assign53500_e87898, ((0.5 * locals.var_vgfbpd_dn3) - (3.0 * (locals.var_gammapd_dn3 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn4) - (3.0 * (locals.var_gammapd_dn4 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn5) - (3.0 * (locals.var_gammapd_dn5 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn6) - (3.0 * (locals.var_gammapd_dn6 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn7) - (3.0 * (locals.var_gammapd_dn7 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn8) - (3.0 * (locals.var_gammapd_dn8 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn9) - (3.0 * (locals.var_gammapd_dn9 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn10) - (3.0 * (locals.var_gammapd_dn10 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn11) - (3.0 * (locals.var_gammapd_dn11 / 1.4142135623730951))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign53500_e87900;
        locals.var_t1_dn3 = assign53500_e87900_d_n3;
        locals.var_t1_dn4 = assign53500_e87900_d_n4;
        locals.var_t1_dn5 = assign53500_e87900_d_n5;
        locals.var_t1_dn6 = assign53500_e87900_d_n6;
        locals.var_t1_dn7 = assign53500_e87900_d_n7;
        locals.var_t1_dn8 = assign53500_e87900_d_n8;
        locals.var_t1_dn9 = assign53500_e87900_d_n9;
        locals.var_t1_dn10 = assign53500_e87900_d_n10;
        locals.var_t1_dn11 = assign53500_e87900_d_n11;

        let (assign53510_e87916, assign53510_e87916_d_n3, assign53510_e87916_d_n4, assign53510_e87916_d_n5, assign53510_e87916_d_n6, assign53510_e87916_d_n7, assign53510_e87916_d_n8, assign53510_e87916_d_n9, assign53510_e87916_d_n10, assign53510_e87916_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        let assign53510_e87908: f64 = (locals.var_t1 * locals.var_t1);
        let assign53510_e87911: f64 = (6.0 * locals.var_vgfbpd);
        let assign53510_e87912: f64 = (assign53510_e87908 + assign53510_e87911);
        let assign53510_e87913: f64 = (assign53510_e87912).sqrt();
        let assign53510_e87914: f64 = (locals.var_t1 + assign53510_e87913);
        (assign53510_e87914, (locals.var_t1_dn3 + ((((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)) + (6.0 * locals.var_vgfbpd_dn3)) / (2.0 * assign53510_e87913))), (locals.var_t1_dn4 + ((((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) + (6.0 * locals.var_vgfbpd_dn4)) / (2.0 * assign53510_e87913))), (locals.var_t1_dn5 + ((((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) + (6.0 * locals.var_vgfbpd_dn5)) / (2.0 * assign53510_e87913))), (locals.var_t1_dn6 + ((((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) + (6.0 * locals.var_vgfbpd_dn6)) / (2.0 * assign53510_e87913))), (locals.var_t1_dn7 + ((((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) + (6.0 * locals.var_vgfbpd_dn7)) / (2.0 * assign53510_e87913))), (locals.var_t1_dn8 + ((((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) + (6.0 * locals.var_vgfbpd_dn8)) / (2.0 * assign53510_e87913))), (locals.var_t1_dn9 + ((((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) + (6.0 * locals.var_vgfbpd_dn9)) / (2.0 * assign53510_e87913))), (locals.var_t1_dn10 + ((((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) + (6.0 * locals.var_vgfbpd_dn10)) / (2.0 * assign53510_e87913))), (locals.var_t1_dn11 + ((((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) + (6.0 * locals.var_vgfbpd_dn11)) / (2.0 * assign53510_e87913))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign53510_e87916;
        locals.var_t2_dn3 = assign53510_e87916_d_n3;
        locals.var_t2_dn4 = assign53510_e87916_d_n4;
        locals.var_t2_dn5 = assign53510_e87916_d_n5;
        locals.var_t2_dn6 = assign53510_e87916_d_n6;
        locals.var_t2_dn7 = assign53510_e87916_d_n7;
        locals.var_t2_dn8 = assign53510_e87916_d_n8;
        locals.var_t2_dn9 = assign53510_e87916_d_n9;
        locals.var_t2_dn10 = assign53510_e87916_d_n10;
        locals.var_t2_dn11 = assign53510_e87916_d_n11;

        let assign53520_e87919: f64 = if locals.var_vgfbpd < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard825 = assign53520_e87919;

        let (assign53530_e87932, assign53530_e87932_d_n3, assign53530_e87932_d_n4, assign53530_e87932_d_n5, assign53530_e87932_d_n6, assign53530_e87932_d_n7, assign53530_e87932_d_n8, assign53530_e87932_d_n9, assign53530_e87932_d_n10, assign53530_e87932_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard825 != 0.0)) {
        let assign53530_e87928: f64 = (locals.var_vgfbpd - locals.var_t2);
        let assign53530_e87930: f64 = (assign53530_e87928 / locals.var_gammapd);
        (assign53530_e87930, ((((locals.var_vgfbpd_dn3 - locals.var_t2_dn3) * locals.var_gammapd) - (assign53530_e87928 * locals.var_gammapd_dn3)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn4 - locals.var_t2_dn4) * locals.var_gammapd) - (assign53530_e87928 * locals.var_gammapd_dn4)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn5 - locals.var_t2_dn5) * locals.var_gammapd) - (assign53530_e87928 * locals.var_gammapd_dn5)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn6 - locals.var_t2_dn6) * locals.var_gammapd) - (assign53530_e87928 * locals.var_gammapd_dn6)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn7 - locals.var_t2_dn7) * locals.var_gammapd) - (assign53530_e87928 * locals.var_gammapd_dn7)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn8 - locals.var_t2_dn8) * locals.var_gammapd) - (assign53530_e87928 * locals.var_gammapd_dn8)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn9 - locals.var_t2_dn9) * locals.var_gammapd) - (assign53530_e87928 * locals.var_gammapd_dn9)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn10 - locals.var_t2_dn10) * locals.var_gammapd) - (assign53530_e87928 * locals.var_gammapd_dn10)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn11 - locals.var_t2_dn11) * locals.var_gammapd) - (assign53530_e87928 * locals.var_gammapd_dn11)) / (locals.var_gammapd * locals.var_gammapd)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign53530_e87932;
        locals.var_t3_dn3 = assign53530_e87932_d_n3;
        locals.var_t3_dn4 = assign53530_e87932_d_n4;
        locals.var_t3_dn5 = assign53530_e87932_d_n5;
        locals.var_t3_dn6 = assign53530_e87932_d_n6;
        locals.var_t3_dn7 = assign53530_e87932_d_n7;
        locals.var_t3_dn8 = assign53530_e87932_d_n8;
        locals.var_t3_dn9 = assign53530_e87932_d_n9;
        locals.var_t3_dn10 = assign53530_e87932_d_n10;
        locals.var_t3_dn11 = assign53530_e87932_d_n11;

        let (assign53540_e87951, assign53540_e87951_d_n3, assign53540_e87951_d_n4, assign53540_e87951_d_n5, assign53540_e87951_d_n6, assign53540_e87951_d_n7, assign53540_e87951_d_n8, assign53540_e87951_d_n9, assign53540_e87951_d_n10, assign53540_e87951_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard825 != 0.0)) {
        let assign53540_e87941: f64 = (1.0 - locals.var_t2);
        let assign53540_e87944: f64 = (locals.var_t3 * locals.var_t3);
        let assign53540_e87945: f64 = (assign53540_e87941 + assign53540_e87944);
        let assign53540_e87947: f64 = (assign53540_e87945).max(1e-38);
        let assign53540_e87948: f64 = (assign53540_e87947).ln();
        let assign53540_e87949: f64 = (-assign53540_e87948);
        (assign53540_e87949, (-(if assign53540_e87945 >= 1e-38 { ((-locals.var_t2_dn3) + ((locals.var_t3_dn3 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn3))) } else { 0.0 } / assign53540_e87947)), (-(if assign53540_e87945 >= 1e-38 { ((-locals.var_t2_dn4) + ((locals.var_t3_dn4 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn4))) } else { 0.0 } / assign53540_e87947)), (-(if assign53540_e87945 >= 1e-38 { ((-locals.var_t2_dn5) + ((locals.var_t3_dn5 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn5))) } else { 0.0 } / assign53540_e87947)), (-(if assign53540_e87945 >= 1e-38 { ((-locals.var_t2_dn6) + ((locals.var_t3_dn6 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn6))) } else { 0.0 } / assign53540_e87947)), (-(if assign53540_e87945 >= 1e-38 { ((-locals.var_t2_dn7) + ((locals.var_t3_dn7 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn7))) } else { 0.0 } / assign53540_e87947)), (-(if assign53540_e87945 >= 1e-38 { ((-locals.var_t2_dn8) + ((locals.var_t3_dn8 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn8))) } else { 0.0 } / assign53540_e87947)), (-(if assign53540_e87945 >= 1e-38 { ((-locals.var_t2_dn9) + ((locals.var_t3_dn9 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn9))) } else { 0.0 } / assign53540_e87947)), (-(if assign53540_e87945 >= 1e-38 { ((-locals.var_t2_dn10) + ((locals.var_t3_dn10 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn10))) } else { 0.0 } / assign53540_e87947)), (-(if assign53540_e87945 >= 1e-38 { ((-locals.var_t2_dn11) + ((locals.var_t3_dn11 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn11))) } else { 0.0 } / assign53540_e87947)),)
    } else {
        (locals.var_psiph, locals.var_psiph_dn3, locals.var_psiph_dn4, locals.var_psiph_dn5, locals.var_psiph_dn6, locals.var_psiph_dn7, locals.var_psiph_dn8, locals.var_psiph_dn9, locals.var_psiph_dn10, locals.var_psiph_dn11,)
    }
};
        locals.var_psiph = assign53540_e87951;
        locals.var_psiph_dn3 = assign53540_e87951_d_n3;
        locals.var_psiph_dn4 = assign53540_e87951_d_n4;
        locals.var_psiph_dn5 = assign53540_e87951_d_n5;
        locals.var_psiph_dn6 = assign53540_e87951_d_n6;
        locals.var_psiph_dn7 = assign53540_e87951_d_n7;
        locals.var_psiph_dn8 = assign53540_e87951_d_n8;
        locals.var_psiph_dn9 = assign53540_e87951_d_n9;
        locals.var_psiph_dn10 = assign53540_e87951_d_n10;
        locals.var_psiph_dn11 = assign53540_e87951_d_n11;

        let (assign53550_e87963, assign53550_e87963_d_n3, assign53550_e87963_d_n4, assign53550_e87963_d_n5, assign53550_e87963_d_n6, assign53550_e87963_d_n7, assign53550_e87963_d_n8, assign53550_e87963_d_n9, assign53550_e87963_d_n10, assign53550_e87963_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard825 == 0.0)) {
        let assign53550_e87960: f64 = (-locals.var_t2);
        let assign53550_e87961: f64 = { let limited_exp_arg = assign53550_e87960; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign53550_e87961, ({ let limited_exp_arg = assign53550_e87960; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn3)), ({ let limited_exp_arg = assign53550_e87960; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn4)), ({ let limited_exp_arg = assign53550_e87960; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn5)), ({ let limited_exp_arg = assign53550_e87960; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn6)), ({ let limited_exp_arg = assign53550_e87960; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn7)), ({ let limited_exp_arg = assign53550_e87960; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn8)), ({ let limited_exp_arg = assign53550_e87960; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn9)), ({ let limited_exp_arg = assign53550_e87960; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn10)), ({ let limited_exp_arg = assign53550_e87960; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn11)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign53550_e87963;
        locals.var_t3_dn3 = assign53550_e87963_d_n3;
        locals.var_t3_dn4 = assign53550_e87963_d_n4;
        locals.var_t3_dn5 = assign53550_e87963_d_n5;
        locals.var_t3_dn6 = assign53550_e87963_d_n6;
        locals.var_t3_dn7 = assign53550_e87963_d_n7;
        locals.var_t3_dn8 = assign53550_e87963_d_n8;
        locals.var_t3_dn9 = assign53550_e87963_d_n9;
        locals.var_t3_dn10 = assign53550_e87963_d_n10;
        locals.var_t3_dn11 = assign53550_e87963_d_n11;

        let (assign53560_e87975, assign53560_e87975_d_n3, assign53560_e87975_d_n4, assign53560_e87975_d_n5, assign53560_e87975_d_n6, assign53560_e87975_d_n7, assign53560_e87975_d_n8, assign53560_e87975_d_n9, assign53560_e87975_d_n10, assign53560_e87975_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard825 == 0.0)) {
        let assign53560_e87973: f64 = (0.5 * locals.var_gammapd);
        (assign53560_e87973, (0.5 * locals.var_gammapd_dn3), (0.5 * locals.var_gammapd_dn4), (0.5 * locals.var_gammapd_dn5), (0.5 * locals.var_gammapd_dn6), (0.5 * locals.var_gammapd_dn7), (0.5 * locals.var_gammapd_dn8), (0.5 * locals.var_gammapd_dn9), (0.5 * locals.var_gammapd_dn10), (0.5 * locals.var_gammapd_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign53560_e87975;
        locals.var_t1_dn3 = assign53560_e87975_d_n3;
        locals.var_t1_dn4 = assign53560_e87975_d_n4;
        locals.var_t1_dn5 = assign53560_e87975_d_n5;
        locals.var_t1_dn6 = assign53560_e87975_d_n6;
        locals.var_t1_dn7 = assign53560_e87975_d_n7;
        locals.var_t1_dn8 = assign53560_e87975_d_n8;
        locals.var_t1_dn9 = assign53560_e87975_d_n9;
        locals.var_t1_dn10 = assign53560_e87975_d_n10;
        locals.var_t1_dn11 = assign53560_e87975_d_n11;

        let (assign53570_e87996, assign53570_e87996_d_n3, assign53570_e87996_d_n4, assign53570_e87996_d_n5, assign53570_e87996_d_n6, assign53570_e87996_d_n7, assign53570_e87996_d_n8, assign53570_e87996_d_n9, assign53570_e87996_d_n10, assign53570_e87996_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard825 == 0.0)) {
        let assign53570_e87985: f64 = (locals.var_vgfbpd - 1.0);
        let assign53570_e87987: f64 = (assign53570_e87985 + locals.var_t3);
        let assign53570_e87990: f64 = (locals.var_t1 * locals.var_t1);
        let assign53570_e87991: f64 = (assign53570_e87987 + assign53570_e87990);
        let assign53570_e87992: f64 = (assign53570_e87991).sqrt();
        let assign53570_e87994: f64 = (assign53570_e87992 - locals.var_t1);
        (assign53570_e87994, ((((locals.var_vgfbpd_dn3 + locals.var_t3_dn3) + ((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3))) / (2.0 * assign53570_e87992)) - locals.var_t1_dn3), ((((locals.var_vgfbpd_dn4 + locals.var_t3_dn4) + ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4))) / (2.0 * assign53570_e87992)) - locals.var_t1_dn4), ((((locals.var_vgfbpd_dn5 + locals.var_t3_dn5) + ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5))) / (2.0 * assign53570_e87992)) - locals.var_t1_dn5), ((((locals.var_vgfbpd_dn6 + locals.var_t3_dn6) + ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6))) / (2.0 * assign53570_e87992)) - locals.var_t1_dn6), ((((locals.var_vgfbpd_dn7 + locals.var_t3_dn7) + ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7))) / (2.0 * assign53570_e87992)) - locals.var_t1_dn7), ((((locals.var_vgfbpd_dn8 + locals.var_t3_dn8) + ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8))) / (2.0 * assign53570_e87992)) - locals.var_t1_dn8), ((((locals.var_vgfbpd_dn9 + locals.var_t3_dn9) + ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9))) / (2.0 * assign53570_e87992)) - locals.var_t1_dn9), ((((locals.var_vgfbpd_dn10 + locals.var_t3_dn10) + ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10))) / (2.0 * assign53570_e87992)) - locals.var_t1_dn10), ((((locals.var_vgfbpd_dn11 + locals.var_t3_dn11) + ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11))) / (2.0 * assign53570_e87992)) - locals.var_t1_dn11),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign53570_e87996;
        locals.var_t2_dn3 = assign53570_e87996_d_n3;
        locals.var_t2_dn4 = assign53570_e87996_d_n4;
        locals.var_t2_dn5 = assign53570_e87996_d_n5;
        locals.var_t2_dn6 = assign53570_e87996_d_n6;
        locals.var_t2_dn7 = assign53570_e87996_d_n7;
        locals.var_t2_dn8 = assign53570_e87996_d_n8;
        locals.var_t2_dn9 = assign53570_e87996_d_n9;
        locals.var_t2_dn10 = assign53570_e87996_d_n10;
        locals.var_t2_dn11 = assign53570_e87996_d_n11;

        let (assign53580_e88012, assign53580_e88012_d_n3, assign53580_e88012_d_n4, assign53580_e88012_d_n5, assign53580_e88012_d_n6, assign53580_e88012_d_n7, assign53580_e88012_d_n8, assign53580_e88012_d_n9, assign53580_e88012_d_n10, assign53580_e88012_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard825 == 0.0)) {
        let assign53580_e88006: f64 = (locals.var_t2 * locals.var_t2);
        let assign53580_e88008: f64 = (assign53580_e88006 + 1.0);
        let assign53580_e88010: f64 = (assign53580_e88008 - locals.var_t3);
        (assign53580_e88010, (((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)) - locals.var_t3_dn3), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) - locals.var_t3_dn4), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) - locals.var_t3_dn5), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) - locals.var_t3_dn6), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) - locals.var_t3_dn7), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) - locals.var_t3_dn8), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) - locals.var_t3_dn9), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) - locals.var_t3_dn10), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) - locals.var_t3_dn11),)
    } else {
        (locals.var_psiph, locals.var_psiph_dn3, locals.var_psiph_dn4, locals.var_psiph_dn5, locals.var_psiph_dn6, locals.var_psiph_dn7, locals.var_psiph_dn8, locals.var_psiph_dn9, locals.var_psiph_dn10, locals.var_psiph_dn11,)
    }
};
        locals.var_psiph = assign53580_e88012;
        locals.var_psiph_dn3 = assign53580_e88012_d_n3;
        locals.var_psiph_dn4 = assign53580_e88012_d_n4;
        locals.var_psiph_dn5 = assign53580_e88012_d_n5;
        locals.var_psiph_dn6 = assign53580_e88012_d_n6;
        locals.var_psiph_dn7 = assign53580_e88012_d_n7;
        locals.var_psiph_dn8 = assign53580_e88012_d_n8;
        locals.var_psiph_dn9 = assign53580_e88012_d_n9;
        locals.var_psiph_dn10 = assign53580_e88012_d_n10;
        locals.var_psiph_dn11 = assign53580_e88012_d_n11;

        let (assign53590_e88038, assign53590_e88038_d_n3, assign53590_e88038_d_n4, assign53590_e88038_d_n5, assign53590_e88038_d_n6, assign53590_e88038_d_n7, assign53590_e88038_d_n8, assign53590_e88038_d_n9, assign53590_e88038_d_n10, assign53590_e88038_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        let assign53590_e88020: f64 = (locals.var_psiph + 1.0);
        let assign53590_e88023: f64 = (locals.var_psiph - 1.0);
        let assign53590_e88026: f64 = (locals.var_psiph - 1.0);
        let assign53590_e88027: f64 = (assign53590_e88023 * assign53590_e88026);
        let assign53590_e88030: f64 = (0.25 * 2.0);
        let assign53590_e88032: f64 = (assign53590_e88030 * 2.0);
        let assign53590_e88033: f64 = (assign53590_e88027 + assign53590_e88032);
        let assign53590_e88034: f64 = (assign53590_e88033).sqrt();
        let assign53590_e88035: f64 = (assign53590_e88020 + assign53590_e88034);
        let assign53590_e88036: f64 = (0.5 * assign53590_e88035);
        (assign53590_e88036, (0.5 * (locals.var_psiph_dn3 + (((locals.var_psiph_dn3 * assign53590_e88026) + (assign53590_e88023 * locals.var_psiph_dn3)) / (2.0 * assign53590_e88034)))), (0.5 * (locals.var_psiph_dn4 + (((locals.var_psiph_dn4 * assign53590_e88026) + (assign53590_e88023 * locals.var_psiph_dn4)) / (2.0 * assign53590_e88034)))), (0.5 * (locals.var_psiph_dn5 + (((locals.var_psiph_dn5 * assign53590_e88026) + (assign53590_e88023 * locals.var_psiph_dn5)) / (2.0 * assign53590_e88034)))), (0.5 * (locals.var_psiph_dn6 + (((locals.var_psiph_dn6 * assign53590_e88026) + (assign53590_e88023 * locals.var_psiph_dn6)) / (2.0 * assign53590_e88034)))), (0.5 * (locals.var_psiph_dn7 + (((locals.var_psiph_dn7 * assign53590_e88026) + (assign53590_e88023 * locals.var_psiph_dn7)) / (2.0 * assign53590_e88034)))), (0.5 * (locals.var_psiph_dn8 + (((locals.var_psiph_dn8 * assign53590_e88026) + (assign53590_e88023 * locals.var_psiph_dn8)) / (2.0 * assign53590_e88034)))), (0.5 * (locals.var_psiph_dn9 + (((locals.var_psiph_dn9 * assign53590_e88026) + (assign53590_e88023 * locals.var_psiph_dn9)) / (2.0 * assign53590_e88034)))), (0.5 * (locals.var_psiph_dn10 + (((locals.var_psiph_dn10 * assign53590_e88026) + (assign53590_e88023 * locals.var_psiph_dn10)) / (2.0 * assign53590_e88034)))), (0.5 * (locals.var_psiph_dn11 + (((locals.var_psiph_dn11 * assign53590_e88026) + (assign53590_e88023 * locals.var_psiph_dn11)) / (2.0 * assign53590_e88034)))),)
    } else {
        (locals.var_t8, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11,)
    }
};
        locals.var_t8 = assign53590_e88038;
        locals.var_t8_dn3 = assign53590_e88038_d_n3;
        locals.var_t8_dn4 = assign53590_e88038_d_n4;
        locals.var_t8_dn5 = assign53590_e88038_d_n5;
        locals.var_t8_dn6 = assign53590_e88038_d_n6;
        locals.var_t8_dn7 = assign53590_e88038_d_n7;
        locals.var_t8_dn8 = assign53590_e88038_d_n8;
        locals.var_t8_dn9 = assign53590_e88038_d_n9;
        locals.var_t8_dn10 = assign53590_e88038_d_n10;
        locals.var_t8_dn11 = assign53590_e88038_d_n11;

        let (assign53600_e88046, assign53600_e88046_d_n3, assign53600_e88046_d_n4, assign53600_e88046_d_n5, assign53600_e88046_d_n6, assign53600_e88046_d_n7, assign53600_e88046_d_n8, assign53600_e88046_d_n9, assign53600_e88046_d_n10, assign53600_e88046_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        let assign53600_e88044: f64 = (locals.var_t8).sqrt();
        (assign53600_e88044, (locals.var_t8_dn3 / (2.0 * assign53600_e88044)), (locals.var_t8_dn4 / (2.0 * assign53600_e88044)), (locals.var_t8_dn5 / (2.0 * assign53600_e88044)), (locals.var_t8_dn6 / (2.0 * assign53600_e88044)), (locals.var_t8_dn7 / (2.0 * assign53600_e88044)), (locals.var_t8_dn8 / (2.0 * assign53600_e88044)), (locals.var_t8_dn9 / (2.0 * assign53600_e88044)), (locals.var_t8_dn10 / (2.0 * assign53600_e88044)), (locals.var_t8_dn11 / (2.0 * assign53600_e88044)),)
    } else {
        (locals.var_sqrtpsip, locals.var_sqrtpsip_dn3, locals.var_sqrtpsip_dn4, locals.var_sqrtpsip_dn5, locals.var_sqrtpsip_dn6, locals.var_sqrtpsip_dn7, locals.var_sqrtpsip_dn8, locals.var_sqrtpsip_dn9, locals.var_sqrtpsip_dn10, locals.var_sqrtpsip_dn11,)
    }
};
        locals.var_sqrtpsip = assign53600_e88046;
        locals.var_sqrtpsip_dn3 = assign53600_e88046_d_n3;
        locals.var_sqrtpsip_dn4 = assign53600_e88046_d_n4;
        locals.var_sqrtpsip_dn5 = assign53600_e88046_d_n5;
        locals.var_sqrtpsip_dn6 = assign53600_e88046_d_n6;
        locals.var_sqrtpsip_dn7 = assign53600_e88046_d_n7;
        locals.var_sqrtpsip_dn8 = assign53600_e88046_d_n8;
        locals.var_sqrtpsip_dn9 = assign53600_e88046_d_n9;
        locals.var_sqrtpsip_dn10 = assign53600_e88046_d_n10;
        locals.var_sqrtpsip_dn11 = assign53600_e88046_d_n11;

        let (assign53610_e88061, assign53610_e88061_d_n3, assign53610_e88061_d_n4, assign53610_e88061_d_n5, assign53610_e88061_d_n6, assign53610_e88061_d_n7, assign53610_e88061_d_n8, assign53610_e88061_d_n9, assign53610_e88061_d_n10, assign53610_e88061_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        let assign53610_e88055: f64 = (2.0 * locals.var_sqrtpsip);
        let assign53610_e88056: f64 = (locals.var_gam_h / assign53610_e88055);
        let assign53610_e88057: f64 = (1.0 + assign53610_e88056);
        let assign53610_e88059: f64 = (assign53610_e88057 / locals.var_gam_h);
        (assign53610_e88059, ((-((locals.var_gam_h * (2.0 * locals.var_sqrtpsip_dn3)) / (assign53610_e88055 * assign53610_e88055))) / locals.var_gam_h), ((((((locals.var_gam_h_dn4 * assign53610_e88055) - (locals.var_gam_h * (2.0 * locals.var_sqrtpsip_dn4))) / (assign53610_e88055 * assign53610_e88055)) * locals.var_gam_h) - (assign53610_e88057 * locals.var_gam_h_dn4)) / (locals.var_gam_h * locals.var_gam_h)), ((((((locals.var_gam_h_dn5 * assign53610_e88055) - (locals.var_gam_h * (2.0 * locals.var_sqrtpsip_dn5))) / (assign53610_e88055 * assign53610_e88055)) * locals.var_gam_h) - (assign53610_e88057 * locals.var_gam_h_dn5)) / (locals.var_gam_h * locals.var_gam_h)), ((-((locals.var_gam_h * (2.0 * locals.var_sqrtpsip_dn6)) / (assign53610_e88055 * assign53610_e88055))) / locals.var_gam_h), ((-((locals.var_gam_h * (2.0 * locals.var_sqrtpsip_dn7)) / (assign53610_e88055 * assign53610_e88055))) / locals.var_gam_h), ((-((locals.var_gam_h * (2.0 * locals.var_sqrtpsip_dn8)) / (assign53610_e88055 * assign53610_e88055))) / locals.var_gam_h), ((-((locals.var_gam_h * (2.0 * locals.var_sqrtpsip_dn9)) / (assign53610_e88055 * assign53610_e88055))) / locals.var_gam_h), ((-((locals.var_gam_h * (2.0 * locals.var_sqrtpsip_dn10)) / (assign53610_e88055 * assign53610_e88055))) / locals.var_gam_h), ((-((locals.var_gam_h * (2.0 * locals.var_sqrtpsip_dn11)) / (assign53610_e88055 * assign53610_e88055))) / locals.var_gam_h),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign53610_e88061;
        locals.var_t0_dn3 = assign53610_e88061_d_n3;
        locals.var_t0_dn4 = assign53610_e88061_d_n4;
        locals.var_t0_dn5 = assign53610_e88061_d_n5;
        locals.var_t0_dn6 = assign53610_e88061_d_n6;
        locals.var_t0_dn7 = assign53610_e88061_d_n7;
        locals.var_t0_dn8 = assign53610_e88061_d_n8;
        locals.var_t0_dn9 = assign53610_e88061_d_n9;
        locals.var_t0_dn10 = assign53610_e88061_d_n10;
        locals.var_t0_dn11 = assign53610_e88061_d_n11;

        let (assign53620_e88074, assign53620_e88074_d_n3, assign53620_e88074_d_n4, assign53620_e88074_d_n5, assign53620_e88074_d_n6, assign53620_e88074_d_n7, assign53620_e88074_d_n8, assign53620_e88074_d_n9, assign53620_e88074_d_n10, assign53620_e88074_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        let assign53620_e88069: f64 = (2.0 * locals.var_phib_h);
        let assign53620_e88070: f64 = (locals.var_psiph - assign53620_e88069);
        let assign53620_e88072: f64 = (assign53620_e88070 - locals.var_vs_1);
        (assign53620_e88072, ((locals.var_psiph_dn3 - (2.0 * locals.var_phib_h_dn3)) - locals.var_vs_1_dn3), ((locals.var_psiph_dn4 - (2.0 * locals.var_phib_h_dn4)) - locals.var_vs_1_dn4), ((locals.var_psiph_dn5 - (2.0 * locals.var_phib_h_dn5)) - locals.var_vs_1_dn5), ((locals.var_psiph_dn6 - (2.0 * locals.var_phib_h_dn6)) - locals.var_vs_1_dn6), ((locals.var_psiph_dn7 - (2.0 * locals.var_phib_h_dn7)) - locals.var_vs_1_dn7), ((locals.var_psiph_dn8 - (2.0 * locals.var_phib_h_dn8)) - locals.var_vs_1_dn8), ((locals.var_psiph_dn9 - (2.0 * locals.var_phib_h_dn9)) - locals.var_vs_1_dn9), ((locals.var_psiph_dn10 - (2.0 * locals.var_phib_h_dn10)) - locals.var_vs_1_dn10), ((locals.var_psiph_dn11 - (2.0 * locals.var_phib_h_dn11)) - locals.var_vs_1_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign53620_e88074;
        locals.var_t1_dn3 = assign53620_e88074_d_n3;
        locals.var_t1_dn4 = assign53620_e88074_d_n4;
        locals.var_t1_dn5 = assign53620_e88074_d_n5;
        locals.var_t1_dn6 = assign53620_e88074_d_n6;
        locals.var_t1_dn7 = assign53620_e88074_d_n7;
        locals.var_t1_dn8 = assign53620_e88074_d_n8;
        locals.var_t1_dn9 = assign53620_e88074_d_n9;
        locals.var_t1_dn10 = assign53620_e88074_d_n10;
        locals.var_t1_dn11 = assign53620_e88074_d_n11;

        let (assign53630_e88090, assign53630_e88090_d_n3, assign53630_e88090_d_n4, assign53630_e88090_d_n5, assign53630_e88090_d_n6, assign53630_e88090_d_n7, assign53630_e88090_d_n8, assign53630_e88090_d_n9, assign53630_e88090_d_n10, assign53630_e88090_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        let assign53630_e88082: f64 = (4.0 * locals.var_t0);
        let assign53630_e88084: f64 = (assign53630_e88082 * locals.var_sqrtpsip);
        let assign53630_e88086: f64 = (assign53630_e88084).max(1e-38);
        let assign53630_e88087: f64 = (assign53630_e88086).ln();
        let assign53630_e88088: f64 = (locals.var_t1 - assign53630_e88087);
        (assign53630_e88088, (locals.var_t1_dn3 - (if assign53630_e88084 >= 1e-38 { (((4.0 * locals.var_t0_dn3) * locals.var_sqrtpsip) + (assign53630_e88082 * locals.var_sqrtpsip_dn3)) } else { 0.0 } / assign53630_e88086)), (locals.var_t1_dn4 - (if assign53630_e88084 >= 1e-38 { (((4.0 * locals.var_t0_dn4) * locals.var_sqrtpsip) + (assign53630_e88082 * locals.var_sqrtpsip_dn4)) } else { 0.0 } / assign53630_e88086)), (locals.var_t1_dn5 - (if assign53630_e88084 >= 1e-38 { (((4.0 * locals.var_t0_dn5) * locals.var_sqrtpsip) + (assign53630_e88082 * locals.var_sqrtpsip_dn5)) } else { 0.0 } / assign53630_e88086)), (locals.var_t1_dn6 - (if assign53630_e88084 >= 1e-38 { (((4.0 * locals.var_t0_dn6) * locals.var_sqrtpsip) + (assign53630_e88082 * locals.var_sqrtpsip_dn6)) } else { 0.0 } / assign53630_e88086)), (locals.var_t1_dn7 - (if assign53630_e88084 >= 1e-38 { (((4.0 * locals.var_t0_dn7) * locals.var_sqrtpsip) + (assign53630_e88082 * locals.var_sqrtpsip_dn7)) } else { 0.0 } / assign53630_e88086)), (locals.var_t1_dn8 - (if assign53630_e88084 >= 1e-38 { (((4.0 * locals.var_t0_dn8) * locals.var_sqrtpsip) + (assign53630_e88082 * locals.var_sqrtpsip_dn8)) } else { 0.0 } / assign53630_e88086)), (locals.var_t1_dn9 - (if assign53630_e88084 >= 1e-38 { (((4.0 * locals.var_t0_dn9) * locals.var_sqrtpsip) + (assign53630_e88082 * locals.var_sqrtpsip_dn9)) } else { 0.0 } / assign53630_e88086)), (locals.var_t1_dn10 - (if assign53630_e88084 >= 1e-38 { (((4.0 * locals.var_t0_dn10) * locals.var_sqrtpsip) + (assign53630_e88082 * locals.var_sqrtpsip_dn10)) } else { 0.0 } / assign53630_e88086)), (locals.var_t1_dn11 - (if assign53630_e88084 >= 1e-38 { (((4.0 * locals.var_t0_dn11) * locals.var_sqrtpsip) + (assign53630_e88082 * locals.var_sqrtpsip_dn11)) } else { 0.0 } / assign53630_e88086)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign53630_e88090;
        locals.var_t2_dn3 = assign53630_e88090_d_n3;
        locals.var_t2_dn4 = assign53630_e88090_d_n4;
        locals.var_t2_dn5 = assign53630_e88090_d_n5;
        locals.var_t2_dn6 = assign53630_e88090_d_n6;
        locals.var_t2_dn7 = assign53630_e88090_d_n7;
        locals.var_t2_dn8 = assign53630_e88090_d_n8;
        locals.var_t2_dn9 = assign53630_e88090_d_n9;
        locals.var_t2_dn10 = assign53630_e88090_d_n10;
        locals.var_t2_dn11 = assign53630_e88090_d_n11;

        let (assign53640_e88110, assign53640_e88110_d_n3, assign53640_e88110_d_n4, assign53640_e88110_d_n5, assign53640_e88110_d_n6, assign53640_e88110_d_n7, assign53640_e88110_d_n8, assign53640_e88110_d_n9, assign53640_e88110_d_n10, assign53640_e88110_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        let assign53640_e88098: f64 = (locals.var_t2 - 0.201491);
        let assign53640_e88102: f64 = (locals.var_t2 + 0.402982);
        let assign53640_e88103: f64 = (locals.var_t2 * assign53640_e88102);
        let assign53640_e88105: f64 = (assign53640_e88103 + 2.446562);
        let assign53640_e88106: f64 = (assign53640_e88105).sqrt();
        let assign53640_e88107: f64 = (assign53640_e88098 - assign53640_e88106);
        let assign53640_e88108: f64 = (0.5 * assign53640_e88107);
        (assign53640_e88108, (0.5 * (locals.var_t2_dn3 - (((locals.var_t2_dn3 * assign53640_e88102) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign53640_e88106)))), (0.5 * (locals.var_t2_dn4 - (((locals.var_t2_dn4 * assign53640_e88102) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign53640_e88106)))), (0.5 * (locals.var_t2_dn5 - (((locals.var_t2_dn5 * assign53640_e88102) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign53640_e88106)))), (0.5 * (locals.var_t2_dn6 - (((locals.var_t2_dn6 * assign53640_e88102) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign53640_e88106)))), (0.5 * (locals.var_t2_dn7 - (((locals.var_t2_dn7 * assign53640_e88102) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign53640_e88106)))), (0.5 * (locals.var_t2_dn8 - (((locals.var_t2_dn8 * assign53640_e88102) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign53640_e88106)))), (0.5 * (locals.var_t2_dn9 - (((locals.var_t2_dn9 * assign53640_e88102) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign53640_e88106)))), (0.5 * (locals.var_t2_dn10 - (((locals.var_t2_dn10 * assign53640_e88102) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign53640_e88106)))), (0.5 * (locals.var_t2_dn11 - (((locals.var_t2_dn11 * assign53640_e88102) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign53640_e88106)))),)
    } else {
        (locals.var_t8, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11,)
    }
};
        locals.var_t8 = assign53640_e88110;
        locals.var_t8_dn3 = assign53640_e88110_d_n3;
        locals.var_t8_dn4 = assign53640_e88110_d_n4;
        locals.var_t8_dn5 = assign53640_e88110_d_n5;
        locals.var_t8_dn6 = assign53640_e88110_d_n6;
        locals.var_t8_dn7 = assign53640_e88110_d_n7;
        locals.var_t8_dn8 = assign53640_e88110_d_n8;
        locals.var_t8_dn9 = assign53640_e88110_d_n9;
        locals.var_t8_dn10 = assign53640_e88110_d_n10;
        locals.var_t8_dn11 = assign53640_e88110_d_n11;

        let (assign53650_e88117, assign53650_e88117_d_n3, assign53650_e88117_d_n4, assign53650_e88117_d_n5, assign53650_e88117_d_n6, assign53650_e88117_d_n7, assign53650_e88117_d_n8, assign53650_e88117_d_n9, assign53650_e88117_d_n10, assign53650_e88117_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        (locals.var_sqrtpsip, locals.var_sqrtpsip_dn3, locals.var_sqrtpsip_dn4, locals.var_sqrtpsip_dn5, locals.var_sqrtpsip_dn6, locals.var_sqrtpsip_dn7, locals.var_sqrtpsip_dn8, locals.var_sqrtpsip_dn9, locals.var_sqrtpsip_dn10, locals.var_sqrtpsip_dn11,)
    } else {
        (locals.var_sqrtpsisa, locals.var_sqrtpsisa_dn3, locals.var_sqrtpsisa_dn4, locals.var_sqrtpsisa_dn5, locals.var_sqrtpsisa_dn6, locals.var_sqrtpsisa_dn7, locals.var_sqrtpsisa_dn8, locals.var_sqrtpsisa_dn9, locals.var_sqrtpsisa_dn10, locals.var_sqrtpsisa_dn11,)
    }
};
        locals.var_sqrtpsisa = assign53650_e88117;
        locals.var_sqrtpsisa_dn3 = assign53650_e88117_d_n3;
        locals.var_sqrtpsisa_dn4 = assign53650_e88117_d_n4;
        locals.var_sqrtpsisa_dn5 = assign53650_e88117_d_n5;
        locals.var_sqrtpsisa_dn6 = assign53650_e88117_d_n6;
        locals.var_sqrtpsisa_dn7 = assign53650_e88117_d_n7;
        locals.var_sqrtpsisa_dn8 = assign53650_e88117_d_n8;
        locals.var_sqrtpsisa_dn9 = assign53650_e88117_d_n9;
        locals.var_sqrtpsisa_dn10 = assign53650_e88117_d_n10;
        locals.var_sqrtpsisa_dn11 = assign53650_e88117_d_n11;

        let assign53660_e88120: f64 = (-68.0);
        let assign53660_e88121: f64 = if locals.var_t8 <= assign53660_e88120 { 1.0 } else { 0.0 };
        locals.var_guard826 = assign53660_e88121;

        let (assign53670_e88131, assign53670_e88131_d_n3, assign53670_e88131_d_n4, assign53670_e88131_d_n5, assign53670_e88131_d_n6, assign53670_e88131_d_n7, assign53670_e88131_d_n8, assign53670_e88131_d_n9, assign53670_e88131_d_n10, assign53670_e88131_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard826 != 0.0)) {
        let assign53670_e88129: f64 = (-100.0);
        (assign53670_e88129, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign53670_e88131;
        locals.var_t4_dn3 = assign53670_e88131_d_n3;
        locals.var_t4_dn4 = assign53670_e88131_d_n4;
        locals.var_t4_dn5 = assign53670_e88131_d_n5;
        locals.var_t4_dn6 = assign53670_e88131_d_n6;
        locals.var_t4_dn7 = assign53670_e88131_d_n7;
        locals.var_t4_dn8 = assign53670_e88131_d_n8;
        locals.var_t4_dn9 = assign53670_e88131_d_n9;
        locals.var_t4_dn10 = assign53670_e88131_d_n10;
        locals.var_t4_dn11 = assign53670_e88131_d_n11;

        let (assign53680_e88140, assign53680_e88140_d_n3, assign53680_e88140_d_n4, assign53680_e88140_d_n5, assign53680_e88140_d_n6, assign53680_e88140_d_n7, assign53680_e88140_d_n8, assign53680_e88140_d_n9, assign53680_e88140_d_n10, assign53680_e88140_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard826 != 0.0)) {
        (20.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign53680_e88140;
        locals.var_t5_dn3 = assign53680_e88140_d_n3;
        locals.var_t5_dn4 = assign53680_e88140_d_n4;
        locals.var_t5_dn5 = assign53680_e88140_d_n5;
        locals.var_t5_dn6 = assign53680_e88140_d_n6;
        locals.var_t5_dn7 = assign53680_e88140_d_n7;
        locals.var_t5_dn8 = assign53680_e88140_d_n8;
        locals.var_t5_dn9 = assign53680_e88140_d_n9;
        locals.var_t5_dn10 = assign53680_e88140_d_n10;
        locals.var_t5_dn11 = assign53680_e88140_d_n11;

        let assign53690_e88145: f64 = (0.5 * locals.var_t5);
        let assign53690_e88146: f64 = (locals.var_t4 - assign53690_e88145);
        let assign53690_e88147: f64 = if locals.var_t8 < assign53690_e88146 { 1.0 } else { 0.0 };
        locals.var_guard827 = assign53690_e88147;

    }

    pub(super) fn stamp_transient_block_182(
        locals: &mut StampLocals,
    ) {
        let (assign53700_e88159, assign53700_e88159_d_n3, assign53700_e88159_d_n4, assign53700_e88159_d_n5, assign53700_e88159_d_n6, assign53700_e88159_d_n7, assign53700_e88159_d_n8, assign53700_e88159_d_n9, assign53700_e88159_d_n10, assign53700_e88159_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard826 != 0.0)) && (locals.var_guard827 != 0.0)) {
        let assign53700_e88157: f64 = { let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign53700_e88157, ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn3), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn4), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn5), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn6), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn7), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn8), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn9), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn10), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign53700_e88159;
        locals.var_t3_dn3 = assign53700_e88159_d_n3;
        locals.var_t3_dn4 = assign53700_e88159_d_n4;
        locals.var_t3_dn5 = assign53700_e88159_d_n5;
        locals.var_t3_dn6 = assign53700_e88159_d_n6;
        locals.var_t3_dn7 = assign53700_e88159_d_n7;
        locals.var_t3_dn8 = assign53700_e88159_d_n8;
        locals.var_t3_dn9 = assign53700_e88159_d_n9;
        locals.var_t3_dn10 = assign53700_e88159_d_n10;
        locals.var_t3_dn11 = assign53700_e88159_d_n11;

        let assign53710_e88164: f64 = (0.5 * locals.var_t5);
        let assign53710_e88165: f64 = (locals.var_t4 + assign53710_e88164);
        let assign53710_e88166: f64 = if locals.var_t8 > assign53710_e88165 { 1.0 } else { 0.0 };
        locals.var_guard828 = assign53710_e88166;

        let (assign53720_e88181, assign53720_e88181_d_n3, assign53720_e88181_d_n4, assign53720_e88181_d_n5, assign53720_e88181_d_n6, assign53720_e88181_d_n7, assign53720_e88181_d_n8, assign53720_e88181_d_n9, assign53720_e88181_d_n10, assign53720_e88181_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard826 != 0.0)) && (locals.var_guard827 == 0.0)) && (locals.var_guard828 != 0.0)) {
        let assign53720_e88179: f64 = { let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign53720_e88179, ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn3), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn4), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn5), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn6), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn7), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn8), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn9), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn10), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign53720_e88181;
        locals.var_t3_dn3 = assign53720_e88181_d_n3;
        locals.var_t3_dn4 = assign53720_e88181_d_n4;
        locals.var_t3_dn5 = assign53720_e88181_d_n5;
        locals.var_t3_dn6 = assign53720_e88181_d_n6;
        locals.var_t3_dn7 = assign53720_e88181_d_n7;
        locals.var_t3_dn8 = assign53720_e88181_d_n8;
        locals.var_t3_dn9 = assign53720_e88181_d_n9;
        locals.var_t3_dn10 = assign53720_e88181_d_n10;
        locals.var_t3_dn11 = assign53720_e88181_d_n11;

        let (assign53730_e88200, assign53730_e88200_d_n3, assign53730_e88200_d_n4, assign53730_e88200_d_n5, assign53730_e88200_d_n6, assign53730_e88200_d_n7, assign53730_e88200_d_n8, assign53730_e88200_d_n9, assign53730_e88200_d_n10, assign53730_e88200_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard826 != 0.0)) && (locals.var_guard827 == 0.0)) && (locals.var_guard828 == 0.0)) {
        let assign53730_e88196: f64 = (locals.var_t8 - locals.var_t4);
        let assign53730_e88198: f64 = (assign53730_e88196 / locals.var_t5);
        (assign53730_e88198, ((((locals.var_t8_dn3 - locals.var_t4_dn3) * locals.var_t5) - (assign53730_e88196 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn4 - locals.var_t4_dn4) * locals.var_t5) - (assign53730_e88196 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn5 - locals.var_t4_dn5) * locals.var_t5) - (assign53730_e88196 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn6 - locals.var_t4_dn6) * locals.var_t5) - (assign53730_e88196 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn7 - locals.var_t4_dn7) * locals.var_t5) - (assign53730_e88196 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn8 - locals.var_t4_dn8) * locals.var_t5) - (assign53730_e88196 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn9 - locals.var_t4_dn9) * locals.var_t5) - (assign53730_e88196 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn10 - locals.var_t4_dn10) * locals.var_t5) - (assign53730_e88196 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn11 - locals.var_t4_dn11) * locals.var_t5) - (assign53730_e88196 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign53730_e88200;
        locals.var_t2_dn3 = assign53730_e88200_d_n3;
        locals.var_t2_dn4 = assign53730_e88200_d_n4;
        locals.var_t2_dn5 = assign53730_e88200_d_n5;
        locals.var_t2_dn6 = assign53730_e88200_d_n6;
        locals.var_t2_dn7 = assign53730_e88200_d_n7;
        locals.var_t2_dn8 = assign53730_e88200_d_n8;
        locals.var_t2_dn9 = assign53730_e88200_d_n9;
        locals.var_t2_dn10 = assign53730_e88200_d_n10;
        locals.var_t2_dn11 = assign53730_e88200_d_n11;

        let (assign53740_e88217, assign53740_e88217_d_n3, assign53740_e88217_d_n4, assign53740_e88217_d_n5, assign53740_e88217_d_n6, assign53740_e88217_d_n7, assign53740_e88217_d_n8, assign53740_e88217_d_n9, assign53740_e88217_d_n10, assign53740_e88217_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard826 != 0.0)) && (locals.var_guard827 == 0.0)) && (locals.var_guard828 == 0.0)) {
        let assign53740_e88215: f64 = (locals.var_t2 * locals.var_t2);
        (assign53740_e88215, ((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)), ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)), ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)), ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)), ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)), ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)), ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)), ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)), ((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign53740_e88217;
        locals.var_t6_dn3 = assign53740_e88217_d_n3;
        locals.var_t6_dn4 = assign53740_e88217_d_n4;
        locals.var_t6_dn5 = assign53740_e88217_d_n5;
        locals.var_t6_dn6 = assign53740_e88217_d_n6;
        locals.var_t6_dn7 = assign53740_e88217_d_n7;
        locals.var_t6_dn8 = assign53740_e88217_d_n8;
        locals.var_t6_dn9 = assign53740_e88217_d_n9;
        locals.var_t6_dn10 = assign53740_e88217_d_n10;
        locals.var_t6_dn11 = assign53740_e88217_d_n11;

        let (assign53750_e88255, assign53750_e88255_d_n3, assign53750_e88255_d_n4, assign53750_e88255_d_n5, assign53750_e88255_d_n6, assign53750_e88255_d_n7, assign53750_e88255_d_n8, assign53750_e88255_d_n9, assign53750_e88255_d_n10, assign53750_e88255_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard826 != 0.0)) && (locals.var_guard827 == 0.0)) && (locals.var_guard828 == 0.0)) {
        let assign53750_e88234: f64 = (5.0 / 64.0);
        let assign53750_e88237: f64 = (0.5 * locals.var_t2);
        let assign53750_e88238: f64 = (assign53750_e88234 + assign53750_e88237);
        let assign53750_e88242: f64 = (15.0 / 16.0);
        let assign53750_e88246: f64 = (1.25 - locals.var_t6);
        let assign53750_e88247: f64 = (locals.var_t6 * assign53750_e88246);
        let assign53750_e88248: f64 = (assign53750_e88242 - assign53750_e88247);
        let assign53750_e88249: f64 = (locals.var_t6 * assign53750_e88248);
        let assign53750_e88250: f64 = (assign53750_e88238 + assign53750_e88249);
        let assign53750_e88251: f64 = (locals.var_t5 * assign53750_e88250);
        let assign53750_e88252: f64 = (locals.var_t4 + assign53750_e88251);
        let assign53750_e88253: f64 = { let limited_exp_arg = assign53750_e88252; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign53750_e88253, ({ let limited_exp_arg = assign53750_e88252; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn3 + ((locals.var_t5_dn3 * assign53750_e88250) + (locals.var_t5 * ((0.5 * locals.var_t2_dn3) + ((locals.var_t6_dn3 * assign53750_e88248) + (locals.var_t6 * (-((locals.var_t6_dn3 * assign53750_e88246) + (locals.var_t6 * (-locals.var_t6_dn3))))))))))), ({ let limited_exp_arg = assign53750_e88252; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn4 + ((locals.var_t5_dn4 * assign53750_e88250) + (locals.var_t5 * ((0.5 * locals.var_t2_dn4) + ((locals.var_t6_dn4 * assign53750_e88248) + (locals.var_t6 * (-((locals.var_t6_dn4 * assign53750_e88246) + (locals.var_t6 * (-locals.var_t6_dn4))))))))))), ({ let limited_exp_arg = assign53750_e88252; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn5 + ((locals.var_t5_dn5 * assign53750_e88250) + (locals.var_t5 * ((0.5 * locals.var_t2_dn5) + ((locals.var_t6_dn5 * assign53750_e88248) + (locals.var_t6 * (-((locals.var_t6_dn5 * assign53750_e88246) + (locals.var_t6 * (-locals.var_t6_dn5))))))))))), ({ let limited_exp_arg = assign53750_e88252; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn6 + ((locals.var_t5_dn6 * assign53750_e88250) + (locals.var_t5 * ((0.5 * locals.var_t2_dn6) + ((locals.var_t6_dn6 * assign53750_e88248) + (locals.var_t6 * (-((locals.var_t6_dn6 * assign53750_e88246) + (locals.var_t6 * (-locals.var_t6_dn6))))))))))), ({ let limited_exp_arg = assign53750_e88252; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn7 + ((locals.var_t5_dn7 * assign53750_e88250) + (locals.var_t5 * ((0.5 * locals.var_t2_dn7) + ((locals.var_t6_dn7 * assign53750_e88248) + (locals.var_t6 * (-((locals.var_t6_dn7 * assign53750_e88246) + (locals.var_t6 * (-locals.var_t6_dn7))))))))))), ({ let limited_exp_arg = assign53750_e88252; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn8 + ((locals.var_t5_dn8 * assign53750_e88250) + (locals.var_t5 * ((0.5 * locals.var_t2_dn8) + ((locals.var_t6_dn8 * assign53750_e88248) + (locals.var_t6 * (-((locals.var_t6_dn8 * assign53750_e88246) + (locals.var_t6 * (-locals.var_t6_dn8))))))))))), ({ let limited_exp_arg = assign53750_e88252; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn9 + ((locals.var_t5_dn9 * assign53750_e88250) + (locals.var_t5 * ((0.5 * locals.var_t2_dn9) + ((locals.var_t6_dn9 * assign53750_e88248) + (locals.var_t6 * (-((locals.var_t6_dn9 * assign53750_e88246) + (locals.var_t6 * (-locals.var_t6_dn9))))))))))), ({ let limited_exp_arg = assign53750_e88252; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn10 + ((locals.var_t5_dn10 * assign53750_e88250) + (locals.var_t5 * ((0.5 * locals.var_t2_dn10) + ((locals.var_t6_dn10 * assign53750_e88248) + (locals.var_t6 * (-((locals.var_t6_dn10 * assign53750_e88246) + (locals.var_t6 * (-locals.var_t6_dn10))))))))))), ({ let limited_exp_arg = assign53750_e88252; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn11 + ((locals.var_t5_dn11 * assign53750_e88250) + (locals.var_t5 * ((0.5 * locals.var_t2_dn11) + ((locals.var_t6_dn11 * assign53750_e88248) + (locals.var_t6 * (-((locals.var_t6_dn11 * assign53750_e88246) + (locals.var_t6 * (-locals.var_t6_dn11))))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign53750_e88255;
        locals.var_t3_dn3 = assign53750_e88255_d_n3;
        locals.var_t3_dn4 = assign53750_e88255_d_n4;
        locals.var_t3_dn5 = assign53750_e88255_d_n5;
        locals.var_t3_dn6 = assign53750_e88255_d_n6;
        locals.var_t3_dn7 = assign53750_e88255_d_n7;
        locals.var_t3_dn8 = assign53750_e88255_d_n8;
        locals.var_t3_dn9 = assign53750_e88255_d_n9;
        locals.var_t3_dn10 = assign53750_e88255_d_n10;
        locals.var_t3_dn11 = assign53750_e88255_d_n11;

        let (assign53760_e88287, assign53760_e88287_d_n3, assign53760_e88287_d_n4, assign53760_e88287_d_n5, assign53760_e88287_d_n6, assign53760_e88287_d_n7, assign53760_e88287_d_n8, assign53760_e88287_d_n9, assign53760_e88287_d_n10, assign53760_e88287_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard826 != 0.0)) {
        let assign53760_e88265: f64 = (1.0 + locals.var_t1);
        let assign53760_e88267: f64 = (assign53760_e88265 - locals.var_t8);
        let assign53760_e88270: f64 = (2.0 * locals.var_t0);
        let assign53760_e88273: f64 = (locals.var_t3 * 2.0);
        let assign53760_e88275: f64 = (assign53760_e88273 * locals.var_t0);
        let assign53760_e88278: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign53760_e88279: f64 = (assign53760_e88275 + assign53760_e88278);
        let assign53760_e88280: f64 = (assign53760_e88270 * assign53760_e88279);
        let assign53760_e88282: f64 = (assign53760_e88280).max(1e-38);
        let assign53760_e88283: f64 = (assign53760_e88282).ln();
        let assign53760_e88284: f64 = (assign53760_e88267 - assign53760_e88283);
        let assign53760_e88285: f64 = (locals.var_t3 * assign53760_e88284);
        (assign53760_e88285, ((locals.var_t3_dn3 * assign53760_e88284) + (locals.var_t3 * ((locals.var_t1_dn3 - locals.var_t8_dn3) - (if assign53760_e88280 >= 1e-38 { (((2.0 * locals.var_t0_dn3) * assign53760_e88279) + (assign53760_e88270 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign53760_e88273 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign53760_e88282)))), ((locals.var_t3_dn4 * assign53760_e88284) + (locals.var_t3 * ((locals.var_t1_dn4 - locals.var_t8_dn4) - (if assign53760_e88280 >= 1e-38 { (((2.0 * locals.var_t0_dn4) * assign53760_e88279) + (assign53760_e88270 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign53760_e88273 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign53760_e88282)))), ((locals.var_t3_dn5 * assign53760_e88284) + (locals.var_t3 * ((locals.var_t1_dn5 - locals.var_t8_dn5) - (if assign53760_e88280 >= 1e-38 { (((2.0 * locals.var_t0_dn5) * assign53760_e88279) + (assign53760_e88270 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign53760_e88273 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign53760_e88282)))), ((locals.var_t3_dn6 * assign53760_e88284) + (locals.var_t3 * ((locals.var_t1_dn6 - locals.var_t8_dn6) - (if assign53760_e88280 >= 1e-38 { (((2.0 * locals.var_t0_dn6) * assign53760_e88279) + (assign53760_e88270 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign53760_e88273 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign53760_e88282)))), ((locals.var_t3_dn7 * assign53760_e88284) + (locals.var_t3 * ((locals.var_t1_dn7 - locals.var_t8_dn7) - (if assign53760_e88280 >= 1e-38 { (((2.0 * locals.var_t0_dn7) * assign53760_e88279) + (assign53760_e88270 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign53760_e88273 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign53760_e88282)))), ((locals.var_t3_dn8 * assign53760_e88284) + (locals.var_t3 * ((locals.var_t1_dn8 - locals.var_t8_dn8) - (if assign53760_e88280 >= 1e-38 { (((2.0 * locals.var_t0_dn8) * assign53760_e88279) + (assign53760_e88270 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign53760_e88273 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign53760_e88282)))), ((locals.var_t3_dn9 * assign53760_e88284) + (locals.var_t3 * ((locals.var_t1_dn9 - locals.var_t8_dn9) - (if assign53760_e88280 >= 1e-38 { (((2.0 * locals.var_t0_dn9) * assign53760_e88279) + (assign53760_e88270 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign53760_e88273 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign53760_e88282)))), ((locals.var_t3_dn10 * assign53760_e88284) + (locals.var_t3 * ((locals.var_t1_dn10 - locals.var_t8_dn10) - (if assign53760_e88280 >= 1e-38 { (((2.0 * locals.var_t0_dn10) * assign53760_e88279) + (assign53760_e88270 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign53760_e88273 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign53760_e88282)))), ((locals.var_t3_dn11 * assign53760_e88284) + (locals.var_t3 * ((locals.var_t1_dn11 - locals.var_t8_dn11) - (if assign53760_e88280 >= 1e-38 { (((2.0 * locals.var_t0_dn11) * assign53760_e88279) + (assign53760_e88270 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign53760_e88273 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign53760_e88282)))),)
    } else {
        (locals.var_qsh, locals.var_qsh_dn3, locals.var_qsh_dn4, locals.var_qsh_dn5, locals.var_qsh_dn6, locals.var_qsh_dn7, locals.var_qsh_dn8, locals.var_qsh_dn9, locals.var_qsh_dn10, locals.var_qsh_dn11,)
    }
};
        locals.var_qsh = assign53760_e88287;
        locals.var_qsh_dn3 = assign53760_e88287_d_n3;
        locals.var_qsh_dn4 = assign53760_e88287_d_n4;
        locals.var_qsh_dn5 = assign53760_e88287_d_n5;
        locals.var_qsh_dn6 = assign53760_e88287_d_n6;
        locals.var_qsh_dn7 = assign53760_e88287_d_n7;
        locals.var_qsh_dn8 = assign53760_e88287_d_n8;
        locals.var_qsh_dn9 = assign53760_e88287_d_n9;
        locals.var_qsh_dn10 = assign53760_e88287_d_n10;
        locals.var_qsh_dn11 = assign53760_e88287_d_n11;

        let (assign53770_e88298, assign53770_e88298_d_n3, assign53770_e88298_d_n4, assign53770_e88298_d_n5, assign53770_e88298_d_n6, assign53770_e88298_d_n7, assign53770_e88298_d_n8, assign53770_e88298_d_n9, assign53770_e88298_d_n10, assign53770_e88298_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard826 == 0.0)) {
        let assign53770_e88296: f64 = { let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign53770_e88296, ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn3), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn4), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn5), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn6), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn7), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn8), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn9), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn10), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign53770_e88298;
        locals.var_t3_dn3 = assign53770_e88298_d_n3;
        locals.var_t3_dn4 = assign53770_e88298_d_n4;
        locals.var_t3_dn5 = assign53770_e88298_d_n5;
        locals.var_t3_dn6 = assign53770_e88298_d_n6;
        locals.var_t3_dn7 = assign53770_e88298_d_n7;
        locals.var_t3_dn8 = assign53770_e88298_d_n8;
        locals.var_t3_dn9 = assign53770_e88298_d_n9;
        locals.var_t3_dn10 = assign53770_e88298_d_n10;
        locals.var_t3_dn11 = assign53770_e88298_d_n11;

        let (assign53780_e88310, assign53780_e88310_d_n3, assign53780_e88310_d_n4, assign53780_e88310_d_n5, assign53780_e88310_d_n6, assign53780_e88310_d_n7, assign53780_e88310_d_n8, assign53780_e88310_d_n9, assign53780_e88310_d_n10, assign53780_e88310_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard826 == 0.0)) {
        let assign53780_e88308: f64 = (1.0 / locals.var_sqrtpsisa);
        (assign53780_e88308, (-(locals.var_sqrtpsisa_dn3 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn4 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn5 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn6 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn7 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn8 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn9 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn10 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn11 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))),)
    } else {
        (locals.var_sqrtpsisainv, locals.var_sqrtpsisainv_dn3, locals.var_sqrtpsisainv_dn4, locals.var_sqrtpsisainv_dn5, locals.var_sqrtpsisainv_dn6, locals.var_sqrtpsisainv_dn7, locals.var_sqrtpsisainv_dn8, locals.var_sqrtpsisainv_dn9, locals.var_sqrtpsisainv_dn10, locals.var_sqrtpsisainv_dn11,)
    }
};
        locals.var_sqrtpsisainv = assign53780_e88310;
        locals.var_sqrtpsisainv_dn3 = assign53780_e88310_d_n3;
        locals.var_sqrtpsisainv_dn4 = assign53780_e88310_d_n4;
        locals.var_sqrtpsisainv_dn5 = assign53780_e88310_d_n5;
        locals.var_sqrtpsisainv_dn6 = assign53780_e88310_d_n6;
        locals.var_sqrtpsisainv_dn7 = assign53780_e88310_d_n7;
        locals.var_sqrtpsisainv_dn8 = assign53780_e88310_d_n8;
        locals.var_sqrtpsisainv_dn9 = assign53780_e88310_d_n9;
        locals.var_sqrtpsisainv_dn10 = assign53780_e88310_d_n10;
        locals.var_sqrtpsisainv_dn11 = assign53780_e88310_d_n11;

        let (assign53790_e88343, assign53790_e88343_d_n3, assign53790_e88343_d_n4, assign53790_e88343_d_n5, assign53790_e88343_d_n6, assign53790_e88343_d_n7, assign53790_e88343_d_n8, assign53790_e88343_d_n9, assign53790_e88343_d_n10, assign53790_e88343_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard826 == 0.0)) {
        let assign53790_e88320: f64 = (2.0 * locals.var_t3);
        let assign53790_e88323: f64 = (locals.var_t3 * 2.0);
        let assign53790_e88325: f64 = (assign53790_e88323 * locals.var_t0);
        let assign53790_e88328: f64 = (locals.var_t3 * 2.0);
        let assign53790_e88330: f64 = (assign53790_e88328 * locals.var_t0);
        let assign53790_e88333: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign53790_e88334: f64 = (assign53790_e88330 + assign53790_e88333);
        let assign53790_e88335: f64 = (assign53790_e88325 * assign53790_e88334);
        let assign53790_e88337: f64 = (assign53790_e88335).max(1e-38);
        let assign53790_e88338: f64 = (assign53790_e88337).ln();
        let assign53790_e88339: f64 = (assign53790_e88320 + assign53790_e88338);
        let assign53790_e88341: f64 = (assign53790_e88339 - locals.var_t1);
        (assign53790_e88341, (((2.0 * locals.var_t3_dn3) + (if assign53790_e88335 >= 1e-38 { (((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign53790_e88323 * locals.var_t0_dn3)) * assign53790_e88334) + (assign53790_e88325 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign53790_e88328 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign53790_e88337)) - locals.var_t1_dn3), (((2.0 * locals.var_t3_dn4) + (if assign53790_e88335 >= 1e-38 { (((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign53790_e88323 * locals.var_t0_dn4)) * assign53790_e88334) + (assign53790_e88325 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign53790_e88328 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign53790_e88337)) - locals.var_t1_dn4), (((2.0 * locals.var_t3_dn5) + (if assign53790_e88335 >= 1e-38 { (((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign53790_e88323 * locals.var_t0_dn5)) * assign53790_e88334) + (assign53790_e88325 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign53790_e88328 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign53790_e88337)) - locals.var_t1_dn5), (((2.0 * locals.var_t3_dn6) + (if assign53790_e88335 >= 1e-38 { (((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign53790_e88323 * locals.var_t0_dn6)) * assign53790_e88334) + (assign53790_e88325 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign53790_e88328 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign53790_e88337)) - locals.var_t1_dn6), (((2.0 * locals.var_t3_dn7) + (if assign53790_e88335 >= 1e-38 { (((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign53790_e88323 * locals.var_t0_dn7)) * assign53790_e88334) + (assign53790_e88325 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign53790_e88328 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign53790_e88337)) - locals.var_t1_dn7), (((2.0 * locals.var_t3_dn8) + (if assign53790_e88335 >= 1e-38 { (((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign53790_e88323 * locals.var_t0_dn8)) * assign53790_e88334) + (assign53790_e88325 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign53790_e88328 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign53790_e88337)) - locals.var_t1_dn8), (((2.0 * locals.var_t3_dn9) + (if assign53790_e88335 >= 1e-38 { (((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign53790_e88323 * locals.var_t0_dn9)) * assign53790_e88334) + (assign53790_e88325 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign53790_e88328 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign53790_e88337)) - locals.var_t1_dn9), (((2.0 * locals.var_t3_dn10) + (if assign53790_e88335 >= 1e-38 { (((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign53790_e88323 * locals.var_t0_dn10)) * assign53790_e88334) + (assign53790_e88325 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign53790_e88328 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign53790_e88337)) - locals.var_t1_dn10), (((2.0 * locals.var_t3_dn11) + (if assign53790_e88335 >= 1e-38 { (((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign53790_e88323 * locals.var_t0_dn11)) * assign53790_e88334) + (assign53790_e88325 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign53790_e88328 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign53790_e88337)) - locals.var_t1_dn11),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign53790_e88343;
        locals.var_t4_dn3 = assign53790_e88343_d_n3;
        locals.var_t4_dn4 = assign53790_e88343_d_n4;
        locals.var_t4_dn5 = assign53790_e88343_d_n5;
        locals.var_t4_dn6 = assign53790_e88343_d_n6;
        locals.var_t4_dn7 = assign53790_e88343_d_n7;
        locals.var_t4_dn8 = assign53790_e88343_d_n8;
        locals.var_t4_dn9 = assign53790_e88343_d_n9;
        locals.var_t4_dn10 = assign53790_e88343_d_n10;
        locals.var_t4_dn11 = assign53790_e88343_d_n11;

        let (assign53800_e88367, assign53800_e88367_d_n3, assign53800_e88367_d_n4, assign53800_e88367_d_n5, assign53800_e88367_d_n6, assign53800_e88367_d_n7, assign53800_e88367_d_n8, assign53800_e88367_d_n9, assign53800_e88367_d_n10, assign53800_e88367_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard826 == 0.0)) {
        let assign53800_e88354: f64 = (1.0 / locals.var_t3);
        let assign53800_e88355: f64 = (2.0 + assign53800_e88354);
        let assign53800_e88358: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign53800_e88361: f64 = (locals.var_t0 * locals.var_t3);
        let assign53800_e88363: f64 = (assign53800_e88361 + locals.var_sqrtpsisa);
        let assign53800_e88364: f64 = (assign53800_e88358 / assign53800_e88363);
        let assign53800_e88365: f64 = (assign53800_e88355 + assign53800_e88364);
        (assign53800_e88365, ((-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign53800_e88363) - (assign53800_e88358 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign53800_e88363 * assign53800_e88363))), ((-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign53800_e88363) - (assign53800_e88358 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign53800_e88363 * assign53800_e88363))), ((-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign53800_e88363) - (assign53800_e88358 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign53800_e88363 * assign53800_e88363))), ((-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign53800_e88363) - (assign53800_e88358 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign53800_e88363 * assign53800_e88363))), ((-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign53800_e88363) - (assign53800_e88358 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign53800_e88363 * assign53800_e88363))), ((-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign53800_e88363) - (assign53800_e88358 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign53800_e88363 * assign53800_e88363))), ((-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign53800_e88363) - (assign53800_e88358 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign53800_e88363 * assign53800_e88363))), ((-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign53800_e88363) - (assign53800_e88358 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign53800_e88363 * assign53800_e88363))), ((-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign53800_e88363) - (assign53800_e88358 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign53800_e88363 * assign53800_e88363))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign53800_e88367;
        locals.var_t5_dn3 = assign53800_e88367_d_n3;
        locals.var_t5_dn4 = assign53800_e88367_d_n4;
        locals.var_t5_dn5 = assign53800_e88367_d_n5;
        locals.var_t5_dn6 = assign53800_e88367_d_n6;
        locals.var_t5_dn7 = assign53800_e88367_d_n7;
        locals.var_t5_dn8 = assign53800_e88367_d_n8;
        locals.var_t5_dn9 = assign53800_e88367_d_n9;
        locals.var_t5_dn10 = assign53800_e88367_d_n10;
        locals.var_t5_dn11 = assign53800_e88367_d_n11;

        let (assign53810_e88381, assign53810_e88381_d_n3, assign53810_e88381_d_n4, assign53810_e88381_d_n5, assign53810_e88381_d_n6, assign53810_e88381_d_n7, assign53810_e88381_d_n8, assign53810_e88381_d_n9, assign53810_e88381_d_n10, assign53810_e88381_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard826 == 0.0)) {
        let assign53810_e88378: f64 = (locals.var_t4 / locals.var_t5);
        let assign53810_e88379: f64 = (locals.var_t3 - assign53810_e88378);
        (assign53810_e88379, (locals.var_t3_dn3 - (((locals.var_t4_dn3 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn4 - (((locals.var_t4_dn4 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn5 - (((locals.var_t4_dn5 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn6 - (((locals.var_t4_dn6 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn7 - (((locals.var_t4_dn7 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn8 - (((locals.var_t4_dn8 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn9 - (((locals.var_t4_dn9 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn10 - (((locals.var_t4_dn10 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn11 - (((locals.var_t4_dn11 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign53810_e88381;
        locals.var_t3_dn3 = assign53810_e88381_d_n3;
        locals.var_t3_dn4 = assign53810_e88381_d_n4;
        locals.var_t3_dn5 = assign53810_e88381_d_n5;
        locals.var_t3_dn6 = assign53810_e88381_d_n6;
        locals.var_t3_dn7 = assign53810_e88381_d_n7;
        locals.var_t3_dn8 = assign53810_e88381_d_n8;
        locals.var_t3_dn9 = assign53810_e88381_d_n9;
        locals.var_t3_dn10 = assign53810_e88381_d_n10;
        locals.var_t3_dn11 = assign53810_e88381_d_n11;

        let (assign53820_e88414, assign53820_e88414_d_n3, assign53820_e88414_d_n4, assign53820_e88414_d_n5, assign53820_e88414_d_n6, assign53820_e88414_d_n7, assign53820_e88414_d_n8, assign53820_e88414_d_n9, assign53820_e88414_d_n10, assign53820_e88414_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard826 == 0.0)) {
        let assign53820_e88391: f64 = (2.0 * locals.var_t3);
        let assign53820_e88394: f64 = (locals.var_t3 * 2.0);
        let assign53820_e88396: f64 = (assign53820_e88394 * locals.var_t0);
        let assign53820_e88399: f64 = (locals.var_t3 * 2.0);
        let assign53820_e88401: f64 = (assign53820_e88399 * locals.var_t0);
        let assign53820_e88404: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign53820_e88405: f64 = (assign53820_e88401 + assign53820_e88404);
        let assign53820_e88406: f64 = (assign53820_e88396 * assign53820_e88405);
        let assign53820_e88408: f64 = (assign53820_e88406).max(1e-38);
        let assign53820_e88409: f64 = (assign53820_e88408).ln();
        let assign53820_e88410: f64 = (assign53820_e88391 + assign53820_e88409);
        let assign53820_e88412: f64 = (assign53820_e88410 - locals.var_t1);
        (assign53820_e88412, (((2.0 * locals.var_t3_dn3) + (if assign53820_e88406 >= 1e-38 { (((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign53820_e88394 * locals.var_t0_dn3)) * assign53820_e88405) + (assign53820_e88396 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign53820_e88399 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign53820_e88408)) - locals.var_t1_dn3), (((2.0 * locals.var_t3_dn4) + (if assign53820_e88406 >= 1e-38 { (((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign53820_e88394 * locals.var_t0_dn4)) * assign53820_e88405) + (assign53820_e88396 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign53820_e88399 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign53820_e88408)) - locals.var_t1_dn4), (((2.0 * locals.var_t3_dn5) + (if assign53820_e88406 >= 1e-38 { (((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign53820_e88394 * locals.var_t0_dn5)) * assign53820_e88405) + (assign53820_e88396 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign53820_e88399 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign53820_e88408)) - locals.var_t1_dn5), (((2.0 * locals.var_t3_dn6) + (if assign53820_e88406 >= 1e-38 { (((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign53820_e88394 * locals.var_t0_dn6)) * assign53820_e88405) + (assign53820_e88396 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign53820_e88399 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign53820_e88408)) - locals.var_t1_dn6), (((2.0 * locals.var_t3_dn7) + (if assign53820_e88406 >= 1e-38 { (((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign53820_e88394 * locals.var_t0_dn7)) * assign53820_e88405) + (assign53820_e88396 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign53820_e88399 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign53820_e88408)) - locals.var_t1_dn7), (((2.0 * locals.var_t3_dn8) + (if assign53820_e88406 >= 1e-38 { (((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign53820_e88394 * locals.var_t0_dn8)) * assign53820_e88405) + (assign53820_e88396 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign53820_e88399 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign53820_e88408)) - locals.var_t1_dn8), (((2.0 * locals.var_t3_dn9) + (if assign53820_e88406 >= 1e-38 { (((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign53820_e88394 * locals.var_t0_dn9)) * assign53820_e88405) + (assign53820_e88396 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign53820_e88399 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign53820_e88408)) - locals.var_t1_dn9), (((2.0 * locals.var_t3_dn10) + (if assign53820_e88406 >= 1e-38 { (((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign53820_e88394 * locals.var_t0_dn10)) * assign53820_e88405) + (assign53820_e88396 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign53820_e88399 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign53820_e88408)) - locals.var_t1_dn10), (((2.0 * locals.var_t3_dn11) + (if assign53820_e88406 >= 1e-38 { (((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign53820_e88394 * locals.var_t0_dn11)) * assign53820_e88405) + (assign53820_e88396 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign53820_e88399 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign53820_e88408)) - locals.var_t1_dn11),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign53820_e88414;
        locals.var_t4_dn3 = assign53820_e88414_d_n3;
        locals.var_t4_dn4 = assign53820_e88414_d_n4;
        locals.var_t4_dn5 = assign53820_e88414_d_n5;
        locals.var_t4_dn6 = assign53820_e88414_d_n6;
        locals.var_t4_dn7 = assign53820_e88414_d_n7;
        locals.var_t4_dn8 = assign53820_e88414_d_n8;
        locals.var_t4_dn9 = assign53820_e88414_d_n9;
        locals.var_t4_dn10 = assign53820_e88414_d_n10;
        locals.var_t4_dn11 = assign53820_e88414_d_n11;

        let (assign53830_e88438, assign53830_e88438_d_n3, assign53830_e88438_d_n4, assign53830_e88438_d_n5, assign53830_e88438_d_n6, assign53830_e88438_d_n7, assign53830_e88438_d_n8, assign53830_e88438_d_n9, assign53830_e88438_d_n10, assign53830_e88438_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard826 == 0.0)) {
        let assign53830_e88425: f64 = (1.0 / locals.var_t3);
        let assign53830_e88426: f64 = (2.0 + assign53830_e88425);
        let assign53830_e88429: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign53830_e88432: f64 = (locals.var_t0 * locals.var_t3);
        let assign53830_e88434: f64 = (assign53830_e88432 + locals.var_sqrtpsisa);
        let assign53830_e88435: f64 = (assign53830_e88429 / assign53830_e88434);
        let assign53830_e88436: f64 = (assign53830_e88426 + assign53830_e88435);
        (assign53830_e88436, ((-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign53830_e88434) - (assign53830_e88429 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign53830_e88434 * assign53830_e88434))), ((-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign53830_e88434) - (assign53830_e88429 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign53830_e88434 * assign53830_e88434))), ((-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign53830_e88434) - (assign53830_e88429 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign53830_e88434 * assign53830_e88434))), ((-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign53830_e88434) - (assign53830_e88429 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign53830_e88434 * assign53830_e88434))), ((-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign53830_e88434) - (assign53830_e88429 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign53830_e88434 * assign53830_e88434))), ((-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign53830_e88434) - (assign53830_e88429 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign53830_e88434 * assign53830_e88434))), ((-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign53830_e88434) - (assign53830_e88429 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign53830_e88434 * assign53830_e88434))), ((-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign53830_e88434) - (assign53830_e88429 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign53830_e88434 * assign53830_e88434))), ((-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign53830_e88434) - (assign53830_e88429 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign53830_e88434 * assign53830_e88434))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign53830_e88438;
        locals.var_t5_dn3 = assign53830_e88438_d_n3;
        locals.var_t5_dn4 = assign53830_e88438_d_n4;
        locals.var_t5_dn5 = assign53830_e88438_d_n5;
        locals.var_t5_dn6 = assign53830_e88438_d_n6;
        locals.var_t5_dn7 = assign53830_e88438_d_n7;
        locals.var_t5_dn8 = assign53830_e88438_d_n8;
        locals.var_t5_dn9 = assign53830_e88438_d_n9;
        locals.var_t5_dn10 = assign53830_e88438_d_n10;
        locals.var_t5_dn11 = assign53830_e88438_d_n11;

        let (assign53840_e88466, assign53840_e88466_d_n3, assign53840_e88466_d_n4, assign53840_e88466_d_n5, assign53840_e88466_d_n6, assign53840_e88466_d_n7, assign53840_e88466_d_n8, assign53840_e88466_d_n9, assign53840_e88466_d_n10, assign53840_e88466_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard826 == 0.0)) {
        let assign53840_e88448: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign53840_e88451: f64 = (locals.var_t0 * locals.var_t3);
        let assign53840_e88453: f64 = (assign53840_e88451 + locals.var_sqrtpsisa);
        let assign53840_e88454: f64 = (assign53840_e88448 / assign53840_e88453);
        let assign53840_e88457: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign53840_e88460: f64 = (locals.var_t0 * locals.var_t3);
        let assign53840_e88462: f64 = (assign53840_e88460 + locals.var_sqrtpsisa);
        let assign53840_e88463: f64 = (assign53840_e88457 / assign53840_e88462);
        let assign53840_e88464: f64 = (assign53840_e88454 * assign53840_e88463);
        (assign53840_e88464, ((((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign53840_e88453) - (assign53840_e88448 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign53840_e88453 * assign53840_e88453)) * assign53840_e88463) + (assign53840_e88454 * ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign53840_e88462) - (assign53840_e88457 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign53840_e88462 * assign53840_e88462)))), ((((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign53840_e88453) - (assign53840_e88448 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign53840_e88453 * assign53840_e88453)) * assign53840_e88463) + (assign53840_e88454 * ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign53840_e88462) - (assign53840_e88457 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign53840_e88462 * assign53840_e88462)))), ((((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign53840_e88453) - (assign53840_e88448 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign53840_e88453 * assign53840_e88453)) * assign53840_e88463) + (assign53840_e88454 * ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign53840_e88462) - (assign53840_e88457 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign53840_e88462 * assign53840_e88462)))), ((((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign53840_e88453) - (assign53840_e88448 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign53840_e88453 * assign53840_e88453)) * assign53840_e88463) + (assign53840_e88454 * ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign53840_e88462) - (assign53840_e88457 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign53840_e88462 * assign53840_e88462)))), ((((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign53840_e88453) - (assign53840_e88448 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign53840_e88453 * assign53840_e88453)) * assign53840_e88463) + (assign53840_e88454 * ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign53840_e88462) - (assign53840_e88457 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign53840_e88462 * assign53840_e88462)))), ((((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign53840_e88453) - (assign53840_e88448 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign53840_e88453 * assign53840_e88453)) * assign53840_e88463) + (assign53840_e88454 * ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign53840_e88462) - (assign53840_e88457 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign53840_e88462 * assign53840_e88462)))), ((((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign53840_e88453) - (assign53840_e88448 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign53840_e88453 * assign53840_e88453)) * assign53840_e88463) + (assign53840_e88454 * ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign53840_e88462) - (assign53840_e88457 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign53840_e88462 * assign53840_e88462)))), ((((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign53840_e88453) - (assign53840_e88448 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign53840_e88453 * assign53840_e88453)) * assign53840_e88463) + (assign53840_e88454 * ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign53840_e88462) - (assign53840_e88457 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign53840_e88462 * assign53840_e88462)))), ((((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign53840_e88453) - (assign53840_e88448 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign53840_e88453 * assign53840_e88453)) * assign53840_e88463) + (assign53840_e88454 * ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign53840_e88462) - (assign53840_e88457 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign53840_e88462 * assign53840_e88462)))),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign53840_e88466;
        locals.var_t6_dn3 = assign53840_e88466_d_n3;
        locals.var_t6_dn4 = assign53840_e88466_d_n4;
        locals.var_t6_dn5 = assign53840_e88466_d_n5;
        locals.var_t6_dn6 = assign53840_e88466_d_n6;
        locals.var_t6_dn7 = assign53840_e88466_d_n7;
        locals.var_t6_dn8 = assign53840_e88466_d_n8;
        locals.var_t6_dn9 = assign53840_e88466_d_n9;
        locals.var_t6_dn10 = assign53840_e88466_d_n10;
        locals.var_t6_dn11 = assign53840_e88466_d_n11;

        let (assign53850_e88499, assign53850_e88499_d_n3, assign53850_e88499_d_n4, assign53850_e88499_d_n5, assign53850_e88499_d_n6, assign53850_e88499_d_n7, assign53850_e88499_d_n8, assign53850_e88499_d_n9, assign53850_e88499_d_n10, assign53850_e88499_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard826 == 0.0)) {
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_t3;
        let assign53850_e88476: f64 = (1.0 * __rspice_inv_cse_0);
        let assign53850_e88479: f64 = (1.0 * __rspice_inv_cse_0);
        let assign53850_e88480: f64 = (assign53850_e88476 * assign53850_e88479);
        let assign53850_e88481: f64 = (-assign53850_e88480);
        let assign53850_e88485: f64 = (locals.var_sqrtpsisa * locals.var_sqrtpsisa);
        let assign53850_e88487: f64 = (assign53850_e88485 * locals.var_sqrtpsisa);
        let assign53850_e88490: f64 = (locals.var_t0 * locals.var_t3);
        let assign53850_e88492: f64 = (assign53850_e88490 + locals.var_sqrtpsisa);
        let assign53850_e88493: f64 = (assign53850_e88487 * assign53850_e88492);
        let assign53850_e88494: f64 = (1.0 / assign53850_e88493);
        let assign53850_e88495: f64 = (assign53850_e88481 - assign53850_e88494);
        let assign53850_e88497: f64 = (assign53850_e88495 - locals.var_t6);
        (assign53850_e88497, (((-(((-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3))) * assign53850_e88479) + (assign53850_e88476 * (-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn3 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn3)) * locals.var_sqrtpsisa) + (assign53850_e88485 * locals.var_sqrtpsisa_dn3)) * assign53850_e88492) + (assign53850_e88487 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign53850_e88493 * assign53850_e88493)))) - locals.var_t6_dn3), (((-(((-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))) * assign53850_e88479) + (assign53850_e88476 * (-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn4 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn4)) * locals.var_sqrtpsisa) + (assign53850_e88485 * locals.var_sqrtpsisa_dn4)) * assign53850_e88492) + (assign53850_e88487 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign53850_e88493 * assign53850_e88493)))) - locals.var_t6_dn4), (((-(((-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))) * assign53850_e88479) + (assign53850_e88476 * (-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn5 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn5)) * locals.var_sqrtpsisa) + (assign53850_e88485 * locals.var_sqrtpsisa_dn5)) * assign53850_e88492) + (assign53850_e88487 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign53850_e88493 * assign53850_e88493)))) - locals.var_t6_dn5), (((-(((-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))) * assign53850_e88479) + (assign53850_e88476 * (-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn6 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn6)) * locals.var_sqrtpsisa) + (assign53850_e88485 * locals.var_sqrtpsisa_dn6)) * assign53850_e88492) + (assign53850_e88487 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign53850_e88493 * assign53850_e88493)))) - locals.var_t6_dn6), (((-(((-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))) * assign53850_e88479) + (assign53850_e88476 * (-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn7 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn7)) * locals.var_sqrtpsisa) + (assign53850_e88485 * locals.var_sqrtpsisa_dn7)) * assign53850_e88492) + (assign53850_e88487 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign53850_e88493 * assign53850_e88493)))) - locals.var_t6_dn7), (((-(((-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))) * assign53850_e88479) + (assign53850_e88476 * (-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn8 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn8)) * locals.var_sqrtpsisa) + (assign53850_e88485 * locals.var_sqrtpsisa_dn8)) * assign53850_e88492) + (assign53850_e88487 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign53850_e88493 * assign53850_e88493)))) - locals.var_t6_dn8), (((-(((-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))) * assign53850_e88479) + (assign53850_e88476 * (-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn9 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn9)) * locals.var_sqrtpsisa) + (assign53850_e88485 * locals.var_sqrtpsisa_dn9)) * assign53850_e88492) + (assign53850_e88487 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign53850_e88493 * assign53850_e88493)))) - locals.var_t6_dn9), (((-(((-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))) * assign53850_e88479) + (assign53850_e88476 * (-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn10 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn10)) * locals.var_sqrtpsisa) + (assign53850_e88485 * locals.var_sqrtpsisa_dn10)) * assign53850_e88492) + (assign53850_e88487 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign53850_e88493 * assign53850_e88493)))) - locals.var_t6_dn10), (((-(((-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))) * assign53850_e88479) + (assign53850_e88476 * (-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn11 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn11)) * locals.var_sqrtpsisa) + (assign53850_e88485 * locals.var_sqrtpsisa_dn11)) * assign53850_e88492) + (assign53850_e88487 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign53850_e88493 * assign53850_e88493)))) - locals.var_t6_dn11),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign53850_e88499;
        locals.var_t7_dn3 = assign53850_e88499_d_n3;
        locals.var_t7_dn4 = assign53850_e88499_d_n4;
        locals.var_t7_dn5 = assign53850_e88499_d_n5;
        locals.var_t7_dn6 = assign53850_e88499_d_n6;
        locals.var_t7_dn7 = assign53850_e88499_d_n7;
        locals.var_t7_dn8 = assign53850_e88499_d_n8;
        locals.var_t7_dn9 = assign53850_e88499_d_n9;
        locals.var_t7_dn10 = assign53850_e88499_d_n10;
        locals.var_t7_dn11 = assign53850_e88499_d_n11;

        let (assign53860_e88525, assign53860_e88525_d_n3, assign53860_e88525_d_n4, assign53860_e88525_d_n5, assign53860_e88525_d_n6, assign53860_e88525_d_n7, assign53860_e88525_d_n8, assign53860_e88525_d_n9, assign53860_e88525_d_n10, assign53860_e88525_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard826 == 0.0)) {
        let assign53860_e88510: f64 = (locals.var_t4 / locals.var_t5);
        let assign53860_e88514: f64 = (locals.var_t4 * locals.var_t7);
        let assign53860_e88517: f64 = (2.0 * locals.var_t5);
        let assign53860_e88519: f64 = (assign53860_e88517 * locals.var_t5);
        let assign53860_e88520: f64 = (assign53860_e88514 / assign53860_e88519);
        let assign53860_e88521: f64 = (1.0 + assign53860_e88520);
        let assign53860_e88522: f64 = (assign53860_e88510 * assign53860_e88521);
        let assign53860_e88523: f64 = (locals.var_t3 - assign53860_e88522);
        (assign53860_e88523, (locals.var_t3_dn3 - (((((locals.var_t4_dn3 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5)) * assign53860_e88521) + (assign53860_e88510 * (((((locals.var_t4_dn3 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn3)) * assign53860_e88519) - (assign53860_e88514 * (((2.0 * locals.var_t5_dn3) * locals.var_t5) + (assign53860_e88517 * locals.var_t5_dn3)))) / (assign53860_e88519 * assign53860_e88519))))), (locals.var_t3_dn4 - (((((locals.var_t4_dn4 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)) * assign53860_e88521) + (assign53860_e88510 * (((((locals.var_t4_dn4 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn4)) * assign53860_e88519) - (assign53860_e88514 * (((2.0 * locals.var_t5_dn4) * locals.var_t5) + (assign53860_e88517 * locals.var_t5_dn4)))) / (assign53860_e88519 * assign53860_e88519))))), (locals.var_t3_dn5 - (((((locals.var_t4_dn5 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)) * assign53860_e88521) + (assign53860_e88510 * (((((locals.var_t4_dn5 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn5)) * assign53860_e88519) - (assign53860_e88514 * (((2.0 * locals.var_t5_dn5) * locals.var_t5) + (assign53860_e88517 * locals.var_t5_dn5)))) / (assign53860_e88519 * assign53860_e88519))))), (locals.var_t3_dn6 - (((((locals.var_t4_dn6 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5)) * assign53860_e88521) + (assign53860_e88510 * (((((locals.var_t4_dn6 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn6)) * assign53860_e88519) - (assign53860_e88514 * (((2.0 * locals.var_t5_dn6) * locals.var_t5) + (assign53860_e88517 * locals.var_t5_dn6)))) / (assign53860_e88519 * assign53860_e88519))))), (locals.var_t3_dn7 - (((((locals.var_t4_dn7 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5)) * assign53860_e88521) + (assign53860_e88510 * (((((locals.var_t4_dn7 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn7)) * assign53860_e88519) - (assign53860_e88514 * (((2.0 * locals.var_t5_dn7) * locals.var_t5) + (assign53860_e88517 * locals.var_t5_dn7)))) / (assign53860_e88519 * assign53860_e88519))))), (locals.var_t3_dn8 - (((((locals.var_t4_dn8 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5)) * assign53860_e88521) + (assign53860_e88510 * (((((locals.var_t4_dn8 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn8)) * assign53860_e88519) - (assign53860_e88514 * (((2.0 * locals.var_t5_dn8) * locals.var_t5) + (assign53860_e88517 * locals.var_t5_dn8)))) / (assign53860_e88519 * assign53860_e88519))))), (locals.var_t3_dn9 - (((((locals.var_t4_dn9 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5)) * assign53860_e88521) + (assign53860_e88510 * (((((locals.var_t4_dn9 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn9)) * assign53860_e88519) - (assign53860_e88514 * (((2.0 * locals.var_t5_dn9) * locals.var_t5) + (assign53860_e88517 * locals.var_t5_dn9)))) / (assign53860_e88519 * assign53860_e88519))))), (locals.var_t3_dn10 - (((((locals.var_t4_dn10 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5)) * assign53860_e88521) + (assign53860_e88510 * (((((locals.var_t4_dn10 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn10)) * assign53860_e88519) - (assign53860_e88514 * (((2.0 * locals.var_t5_dn10) * locals.var_t5) + (assign53860_e88517 * locals.var_t5_dn10)))) / (assign53860_e88519 * assign53860_e88519))))), (locals.var_t3_dn11 - (((((locals.var_t4_dn11 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5)) * assign53860_e88521) + (assign53860_e88510 * (((((locals.var_t4_dn11 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn11)) * assign53860_e88519) - (assign53860_e88514 * (((2.0 * locals.var_t5_dn11) * locals.var_t5) + (assign53860_e88517 * locals.var_t5_dn11)))) / (assign53860_e88519 * assign53860_e88519))))),)
    } else {
        (locals.var_qsh, locals.var_qsh_dn3, locals.var_qsh_dn4, locals.var_qsh_dn5, locals.var_qsh_dn6, locals.var_qsh_dn7, locals.var_qsh_dn8, locals.var_qsh_dn9, locals.var_qsh_dn10, locals.var_qsh_dn11,)
    }
};
        locals.var_qsh = assign53860_e88525;
        locals.var_qsh_dn3 = assign53860_e88525_d_n3;
        locals.var_qsh_dn4 = assign53860_e88525_d_n4;
        locals.var_qsh_dn5 = assign53860_e88525_d_n5;
        locals.var_qsh_dn6 = assign53860_e88525_d_n6;
        locals.var_qsh_dn7 = assign53860_e88525_d_n7;
        locals.var_qsh_dn8 = assign53860_e88525_d_n8;
        locals.var_qsh_dn9 = assign53860_e88525_d_n9;
        locals.var_qsh_dn10 = assign53860_e88525_d_n10;
        locals.var_qsh_dn11 = assign53860_e88525_d_n11;

        let (assign53870_e88551, assign53870_e88551_d_n3, assign53870_e88551_d_n4, assign53870_e88551_d_n5, assign53870_e88551_d_n6, assign53870_e88551_d_n7, assign53870_e88551_d_n8, assign53870_e88551_d_n9, assign53870_e88551_d_n10, assign53870_e88551_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        let assign53870_e88533: f64 = (locals.var_psiph + 1.0);
        let assign53870_e88536: f64 = (locals.var_psiph - 1.0);
        let assign53870_e88539: f64 = (locals.var_psiph - 1.0);
        let assign53870_e88540: f64 = (assign53870_e88536 * assign53870_e88539);
        let assign53870_e88543: f64 = (0.25 * 2.0);
        let assign53870_e88545: f64 = (assign53870_e88543 * 2.0);
        let assign53870_e88546: f64 = (assign53870_e88540 + assign53870_e88545);
        let assign53870_e88547: f64 = (assign53870_e88546).sqrt();
        let assign53870_e88548: f64 = (assign53870_e88533 + assign53870_e88547);
        let assign53870_e88549: f64 = (0.5 * assign53870_e88548);
        (assign53870_e88549, (0.5 * (locals.var_psiph_dn3 + (((locals.var_psiph_dn3 * assign53870_e88539) + (assign53870_e88536 * locals.var_psiph_dn3)) / (2.0 * assign53870_e88547)))), (0.5 * (locals.var_psiph_dn4 + (((locals.var_psiph_dn4 * assign53870_e88539) + (assign53870_e88536 * locals.var_psiph_dn4)) / (2.0 * assign53870_e88547)))), (0.5 * (locals.var_psiph_dn5 + (((locals.var_psiph_dn5 * assign53870_e88539) + (assign53870_e88536 * locals.var_psiph_dn5)) / (2.0 * assign53870_e88547)))), (0.5 * (locals.var_psiph_dn6 + (((locals.var_psiph_dn6 * assign53870_e88539) + (assign53870_e88536 * locals.var_psiph_dn6)) / (2.0 * assign53870_e88547)))), (0.5 * (locals.var_psiph_dn7 + (((locals.var_psiph_dn7 * assign53870_e88539) + (assign53870_e88536 * locals.var_psiph_dn7)) / (2.0 * assign53870_e88547)))), (0.5 * (locals.var_psiph_dn8 + (((locals.var_psiph_dn8 * assign53870_e88539) + (assign53870_e88536 * locals.var_psiph_dn8)) / (2.0 * assign53870_e88547)))), (0.5 * (locals.var_psiph_dn9 + (((locals.var_psiph_dn9 * assign53870_e88539) + (assign53870_e88536 * locals.var_psiph_dn9)) / (2.0 * assign53870_e88547)))), (0.5 * (locals.var_psiph_dn10 + (((locals.var_psiph_dn10 * assign53870_e88539) + (assign53870_e88536 * locals.var_psiph_dn10)) / (2.0 * assign53870_e88547)))), (0.5 * (locals.var_psiph_dn11 + (((locals.var_psiph_dn11 * assign53870_e88539) + (assign53870_e88536 * locals.var_psiph_dn11)) / (2.0 * assign53870_e88547)))),)
    } else {
        (locals.var_psiphclamp, locals.var_psiphclamp_dn3, locals.var_psiphclamp_dn4, locals.var_psiphclamp_dn5, locals.var_psiphclamp_dn6, locals.var_psiphclamp_dn7, locals.var_psiphclamp_dn8, locals.var_psiphclamp_dn9, locals.var_psiphclamp_dn10, locals.var_psiphclamp_dn11,)
    }
};
        locals.var_psiphclamp = assign53870_e88551;
        locals.var_psiphclamp_dn3 = assign53870_e88551_d_n3;
        locals.var_psiphclamp_dn4 = assign53870_e88551_d_n4;
        locals.var_psiphclamp_dn5 = assign53870_e88551_d_n5;
        locals.var_psiphclamp_dn6 = assign53870_e88551_d_n6;
        locals.var_psiphclamp_dn7 = assign53870_e88551_d_n7;
        locals.var_psiphclamp_dn8 = assign53870_e88551_d_n8;
        locals.var_psiphclamp_dn9 = assign53870_e88551_d_n9;
        locals.var_psiphclamp_dn10 = assign53870_e88551_d_n10;
        locals.var_psiphclamp_dn11 = assign53870_e88551_d_n11;

        let (assign53880_e88565, assign53880_e88565_d_n3, assign53880_e88565_d_n4, assign53880_e88565_d_n5, assign53880_e88565_d_n6, assign53880_e88565_d_n7, assign53880_e88565_d_n8, assign53880_e88565_d_n9, assign53880_e88565_d_n10, assign53880_e88565_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        let assign53880_e88560: f64 = (locals.var_psiphclamp).sqrt();
        let assign53880_e88561: f64 = (2.0 * assign53880_e88560);
        let assign53880_e88562: f64 = (locals.var_gam_h / assign53880_e88561);
        let assign53880_e88563: f64 = (1.0 + assign53880_e88562);
        (assign53880_e88563, (-((locals.var_gam_h * (2.0 * (locals.var_psiphclamp_dn3 / (2.0 * assign53880_e88560)))) / (assign53880_e88561 * assign53880_e88561))), (((locals.var_gam_h_dn4 * assign53880_e88561) - (locals.var_gam_h * (2.0 * (locals.var_psiphclamp_dn4 / (2.0 * assign53880_e88560))))) / (assign53880_e88561 * assign53880_e88561)), (((locals.var_gam_h_dn5 * assign53880_e88561) - (locals.var_gam_h * (2.0 * (locals.var_psiphclamp_dn5 / (2.0 * assign53880_e88560))))) / (assign53880_e88561 * assign53880_e88561)), (-((locals.var_gam_h * (2.0 * (locals.var_psiphclamp_dn6 / (2.0 * assign53880_e88560)))) / (assign53880_e88561 * assign53880_e88561))), (-((locals.var_gam_h * (2.0 * (locals.var_psiphclamp_dn7 / (2.0 * assign53880_e88560)))) / (assign53880_e88561 * assign53880_e88561))), (-((locals.var_gam_h * (2.0 * (locals.var_psiphclamp_dn8 / (2.0 * assign53880_e88560)))) / (assign53880_e88561 * assign53880_e88561))), (-((locals.var_gam_h * (2.0 * (locals.var_psiphclamp_dn9 / (2.0 * assign53880_e88560)))) / (assign53880_e88561 * assign53880_e88561))), (-((locals.var_gam_h * (2.0 * (locals.var_psiphclamp_dn10 / (2.0 * assign53880_e88560)))) / (assign53880_e88561 * assign53880_e88561))), (-((locals.var_gam_h * (2.0 * (locals.var_psiphclamp_dn11 / (2.0 * assign53880_e88560)))) / (assign53880_e88561 * assign53880_e88561))),)
    } else {
        (locals.var_nq_h, locals.var_nq_h_dn3, locals.var_nq_h_dn4, locals.var_nq_h_dn5, locals.var_nq_h_dn6, locals.var_nq_h_dn7, locals.var_nq_h_dn8, locals.var_nq_h_dn9, locals.var_nq_h_dn10, locals.var_nq_h_dn11,)
    }
};
        locals.var_nq_h = assign53880_e88565;
        locals.var_nq_h_dn3 = assign53880_e88565_d_n3;
        locals.var_nq_h_dn4 = assign53880_e88565_d_n4;
        locals.var_nq_h_dn5 = assign53880_e88565_d_n5;
        locals.var_nq_h_dn6 = assign53880_e88565_d_n6;
        locals.var_nq_h_dn7 = assign53880_e88565_d_n7;
        locals.var_nq_h_dn8 = assign53880_e88565_d_n8;
        locals.var_nq_h_dn9 = assign53880_e88565_d_n9;
        locals.var_nq_h_dn10 = assign53880_e88565_d_n10;
        locals.var_nq_h_dn11 = assign53880_e88565_d_n11;

        let (assign53890_e88572, assign53890_e88572_d_n3, assign53890_e88572_d_n4, assign53890_e88572_d_n5, assign53890_e88572_d_n6, assign53890_e88572_d_n7, assign53890_e88572_d_n8, assign53890_e88572_d_n9, assign53890_e88572_d_n10, assign53890_e88572_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        (locals.var_ueff, locals.var_ueff_dn3, locals.var_ueff_dn4, locals.var_ueff_dn5, locals.var_ueff_dn6, locals.var_ueff_dn7, locals.var_ueff_dn8, locals.var_ueff_dn9, locals.var_ueff_dn10, locals.var_ueff_dn11,)
    } else {
        (locals.var_u0_i_h, locals.var_u0_i_h_dn3, locals.var_u0_i_h_dn4, locals.var_u0_i_h_dn5, locals.var_u0_i_h_dn6, locals.var_u0_i_h_dn7, locals.var_u0_i_h_dn8, locals.var_u0_i_h_dn9, locals.var_u0_i_h_dn10, locals.var_u0_i_h_dn11,)
    }
};
        locals.var_u0_i_h = assign53890_e88572;
        locals.var_u0_i_h_dn3 = assign53890_e88572_d_n3;
        locals.var_u0_i_h_dn4 = assign53890_e88572_d_n4;
        locals.var_u0_i_h_dn5 = assign53890_e88572_d_n5;
        locals.var_u0_i_h_dn6 = assign53890_e88572_d_n6;
        locals.var_u0_i_h_dn7 = assign53890_e88572_d_n7;
        locals.var_u0_i_h_dn8 = assign53890_e88572_d_n8;
        locals.var_u0_i_h_dn9 = assign53890_e88572_d_n9;
        locals.var_u0_i_h_dn10 = assign53890_e88572_d_n10;
        locals.var_u0_i_h_dn11 = assign53890_e88572_d_n11;

        let (assign53900_e88583, assign53900_e88583_d_n3, assign53900_e88583_d_n4, assign53900_e88583_d_n5, assign53900_e88583_d_n6, assign53900_e88583_d_n7, assign53900_e88583_d_n8, assign53900_e88583_d_n9, assign53900_e88583_d_n10, assign53900_e88583_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        let assign53900_e88579: f64 = (locals.var_u0_i_h * locals.var_cox);
        let assign53900_e88581: f64 = (assign53900_e88579 * locals.var_weff);
        (assign53900_e88581, ((locals.var_u0_i_h_dn3 * locals.var_cox) * locals.var_weff), ((locals.var_u0_i_h_dn4 * locals.var_cox) * locals.var_weff), ((locals.var_u0_i_h_dn5 * locals.var_cox) * locals.var_weff), ((locals.var_u0_i_h_dn6 * locals.var_cox) * locals.var_weff), ((locals.var_u0_i_h_dn7 * locals.var_cox) * locals.var_weff), ((locals.var_u0_i_h_dn8 * locals.var_cox) * locals.var_weff), ((locals.var_u0_i_h_dn9 * locals.var_cox) * locals.var_weff), ((locals.var_u0_i_h_dn10 * locals.var_cox) * locals.var_weff), ((locals.var_u0_i_h_dn11 * locals.var_cox) * locals.var_weff),)
    } else {
        (locals.var_beta_h, locals.var_beta_h_dn3, locals.var_beta_h_dn4, locals.var_beta_h_dn5, locals.var_beta_h_dn6, locals.var_beta_h_dn7, locals.var_beta_h_dn8, locals.var_beta_h_dn9, locals.var_beta_h_dn10, locals.var_beta_h_dn11,)
    }
};
        locals.var_beta_h = assign53900_e88583;
        locals.var_beta_h_dn3 = assign53900_e88583_d_n3;
        locals.var_beta_h_dn4 = assign53900_e88583_d_n4;
        locals.var_beta_h_dn5 = assign53900_e88583_d_n5;
        locals.var_beta_h_dn6 = assign53900_e88583_d_n6;
        locals.var_beta_h_dn7 = assign53900_e88583_d_n7;
        locals.var_beta_h_dn8 = assign53900_e88583_d_n8;
        locals.var_beta_h_dn9 = assign53900_e88583_d_n9;
        locals.var_beta_h_dn10 = assign53900_e88583_d_n10;
        locals.var_beta_h_dn11 = assign53900_e88583_d_n11;

        let (assign53910_e88594, assign53910_e88594_d_n3, assign53910_e88594_d_n4, assign53910_e88594_d_n5, assign53910_e88594_d_n6, assign53910_e88594_d_n7, assign53910_e88594_d_n8, assign53910_e88594_d_n9, assign53910_e88594_d_n10, assign53910_e88594_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        let assign53910_e88590: f64 = (locals.var_ueff * locals.var_cox);
        let assign53910_e88592: f64 = (assign53910_e88590 * locals.var_weff);
        (assign53910_e88592, ((locals.var_ueff_dn3 * locals.var_cox) * locals.var_weff), ((locals.var_ueff_dn4 * locals.var_cox) * locals.var_weff), ((locals.var_ueff_dn5 * locals.var_cox) * locals.var_weff), ((locals.var_ueff_dn6 * locals.var_cox) * locals.var_weff), ((locals.var_ueff_dn7 * locals.var_cox) * locals.var_weff), ((locals.var_ueff_dn8 * locals.var_cox) * locals.var_weff), ((locals.var_ueff_dn9 * locals.var_cox) * locals.var_weff), ((locals.var_ueff_dn10 * locals.var_cox) * locals.var_weff), ((locals.var_ueff_dn11 * locals.var_cox) * locals.var_weff),)
    } else {
        (locals.var_beta_ch, locals.var_beta_ch_dn3, locals.var_beta_ch_dn4, locals.var_beta_ch_dn5, locals.var_beta_ch_dn6, locals.var_beta_ch_dn7, locals.var_beta_ch_dn8, locals.var_beta_ch_dn9, locals.var_beta_ch_dn10, locals.var_beta_ch_dn11,)
    }
};
        locals.var_beta_ch = assign53910_e88594;
        locals.var_beta_ch_dn3 = assign53910_e88594_d_n3;
        locals.var_beta_ch_dn4 = assign53910_e88594_d_n4;
        locals.var_beta_ch_dn5 = assign53910_e88594_d_n5;
        locals.var_beta_ch_dn6 = assign53910_e88594_d_n6;
        locals.var_beta_ch_dn7 = assign53910_e88594_d_n7;
        locals.var_beta_ch_dn8 = assign53910_e88594_d_n8;
        locals.var_beta_ch_dn9 = assign53910_e88594_d_n9;
        locals.var_beta_ch_dn10 = assign53910_e88594_d_n10;
        locals.var_beta_ch_dn11 = assign53910_e88594_d_n11;

    }

    pub(super) fn stamp_transient_block_183(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign53920_e88613, assign53920_e88613_d_n3, assign53920_e88613_d_n4, assign53920_e88613_d_n5, assign53920_e88613_d_n6, assign53920_e88613_d_n7, assign53920_e88613_d_n8, assign53920_e88613_d_n9, assign53920_e88613_d_n10, assign53920_e88613_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        let assign53920_e88601: f64 = (locals.var_ids * locals.var_lh1);
        let assign53920_e88604: f64 = (2.0 * locals.var_nq_h);
        let assign53920_e88606: f64 = (assign53920_e88604 * locals.var_beta_h);
        let assign53920_e88608: f64 = (assign53920_e88606 * locals.var_vt);
        let assign53920_e88610: f64 = (assign53920_e88608 * locals.var_vt);
        let assign53920_e88611: f64 = (assign53920_e88601 / assign53920_e88610);
        (assign53920_e88611, ((((locals.var_ids_dn3 * locals.var_lh1) * assign53920_e88610) - (assign53920_e88601 * (((((2.0 * locals.var_nq_h_dn3) * locals.var_beta_h) + (assign53920_e88604 * locals.var_beta_h_dn3)) * locals.var_vt) * locals.var_vt))) / (assign53920_e88610 * assign53920_e88610)), ((((locals.var_ids_dn4 * locals.var_lh1) * assign53920_e88610) - (assign53920_e88601 * (((((((2.0 * locals.var_nq_h_dn4) * locals.var_beta_h) + (assign53920_e88604 * locals.var_beta_h_dn4)) * locals.var_vt) + (assign53920_e88606 * locals.var_vt_dn4)) * locals.var_vt) + (assign53920_e88608 * locals.var_vt_dn4)))) / (assign53920_e88610 * assign53920_e88610)), ((((locals.var_ids_dn5 * locals.var_lh1) * assign53920_e88610) - (assign53920_e88601 * (((((((2.0 * locals.var_nq_h_dn5) * locals.var_beta_h) + (assign53920_e88604 * locals.var_beta_h_dn5)) * locals.var_vt) + (assign53920_e88606 * locals.var_vt_dn5)) * locals.var_vt) + (assign53920_e88608 * locals.var_vt_dn5)))) / (assign53920_e88610 * assign53920_e88610)), ((((locals.var_ids_dn6 * locals.var_lh1) * assign53920_e88610) - (assign53920_e88601 * (((((2.0 * locals.var_nq_h_dn6) * locals.var_beta_h) + (assign53920_e88604 * locals.var_beta_h_dn6)) * locals.var_vt) * locals.var_vt))) / (assign53920_e88610 * assign53920_e88610)), ((((locals.var_ids_dn7 * locals.var_lh1) * assign53920_e88610) - (assign53920_e88601 * (((((2.0 * locals.var_nq_h_dn7) * locals.var_beta_h) + (assign53920_e88604 * locals.var_beta_h_dn7)) * locals.var_vt) * locals.var_vt))) / (assign53920_e88610 * assign53920_e88610)), ((((locals.var_ids_dn8 * locals.var_lh1) * assign53920_e88610) - (assign53920_e88601 * (((((2.0 * locals.var_nq_h_dn8) * locals.var_beta_h) + (assign53920_e88604 * locals.var_beta_h_dn8)) * locals.var_vt) * locals.var_vt))) / (assign53920_e88610 * assign53920_e88610)), ((((locals.var_ids_dn9 * locals.var_lh1) * assign53920_e88610) - (assign53920_e88601 * (((((2.0 * locals.var_nq_h_dn9) * locals.var_beta_h) + (assign53920_e88604 * locals.var_beta_h_dn9)) * locals.var_vt) * locals.var_vt))) / (assign53920_e88610 * assign53920_e88610)), ((((locals.var_ids_dn10 * locals.var_lh1) * assign53920_e88610) - (assign53920_e88601 * (((((2.0 * locals.var_nq_h_dn10) * locals.var_beta_h) + (assign53920_e88604 * locals.var_beta_h_dn10)) * locals.var_vt) * locals.var_vt))) / (assign53920_e88610 * assign53920_e88610)), ((((locals.var_ids_dn11 * locals.var_lh1) * assign53920_e88610) - (assign53920_e88601 * (((((2.0 * locals.var_nq_h_dn11) * locals.var_beta_h) + (assign53920_e88604 * locals.var_beta_h_dn11)) * locals.var_vt) * locals.var_vt))) / (assign53920_e88610 * assign53920_e88610)),)
    } else {
        (locals.var_i1, locals.var_i1_dn3, locals.var_i1_dn4, locals.var_i1_dn5, locals.var_i1_dn6, locals.var_i1_dn7, locals.var_i1_dn8, locals.var_i1_dn9, locals.var_i1_dn10, locals.var_i1_dn11,)
    }
};
        locals.var_i1 = assign53920_e88613;
        locals.var_i1_dn3 = assign53920_e88613_d_n3;
        locals.var_i1_dn4 = assign53920_e88613_d_n4;
        locals.var_i1_dn5 = assign53920_e88613_d_n5;
        locals.var_i1_dn6 = assign53920_e88613_d_n6;
        locals.var_i1_dn7 = assign53920_e88613_d_n7;
        locals.var_i1_dn8 = assign53920_e88613_d_n8;
        locals.var_i1_dn9 = assign53920_e88613_d_n9;
        locals.var_i1_dn10 = assign53920_e88613_d_n10;
        locals.var_i1_dn11 = assign53920_e88613_d_n11;

        let (assign53930_e88634, assign53930_e88634_d_n3, assign53930_e88634_d_n4, assign53930_e88634_d_n5, assign53930_e88634_d_n6, assign53930_e88634_d_n7, assign53930_e88634_d_n8, assign53930_e88634_d_n9, assign53930_e88634_d_n10, assign53930_e88634_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        let assign53930_e88621: f64 = (locals.var_leffnoih - locals.var_lh1);
        let assign53930_e88622: f64 = (locals.var_ids * assign53930_e88621);
        let assign53930_e88625: f64 = (2.0 * locals.var_nq);
        let assign53930_e88627: f64 = (assign53930_e88625 * locals.var_beta_ch);
        let assign53930_e88629: f64 = (assign53930_e88627 * locals.var_nvt);
        let assign53930_e88631: f64 = (assign53930_e88629 * locals.var_nvt);
        let assign53930_e88632: f64 = (assign53930_e88622 / assign53930_e88631);
        (assign53930_e88632, ((((locals.var_ids_dn3 * assign53930_e88621) * assign53930_e88631) - (assign53930_e88622 * (((((((2.0 * locals.var_nq_dn3) * locals.var_beta_ch) + (assign53930_e88625 * locals.var_beta_ch_dn3)) * locals.var_nvt) + (assign53930_e88627 * locals.var_nvt_dn3)) * locals.var_nvt) + (assign53930_e88629 * locals.var_nvt_dn3)))) / (assign53930_e88631 * assign53930_e88631)), ((((locals.var_ids_dn4 * assign53930_e88621) * assign53930_e88631) - (assign53930_e88622 * (((((((2.0 * locals.var_nq_dn4) * locals.var_beta_ch) + (assign53930_e88625 * locals.var_beta_ch_dn4)) * locals.var_nvt) + (assign53930_e88627 * locals.var_nvt_dn4)) * locals.var_nvt) + (assign53930_e88629 * locals.var_nvt_dn4)))) / (assign53930_e88631 * assign53930_e88631)), ((((locals.var_ids_dn5 * assign53930_e88621) * assign53930_e88631) - (assign53930_e88622 * (((((((2.0 * locals.var_nq_dn5) * locals.var_beta_ch) + (assign53930_e88625 * locals.var_beta_ch_dn5)) * locals.var_nvt) + (assign53930_e88627 * locals.var_nvt_dn5)) * locals.var_nvt) + (assign53930_e88629 * locals.var_nvt_dn5)))) / (assign53930_e88631 * assign53930_e88631)), ((((locals.var_ids_dn6 * assign53930_e88621) * assign53930_e88631) - (assign53930_e88622 * (((((((2.0 * locals.var_nq_dn6) * locals.var_beta_ch) + (assign53930_e88625 * locals.var_beta_ch_dn6)) * locals.var_nvt) + (assign53930_e88627 * locals.var_nvt_dn6)) * locals.var_nvt) + (assign53930_e88629 * locals.var_nvt_dn6)))) / (assign53930_e88631 * assign53930_e88631)), ((((locals.var_ids_dn7 * assign53930_e88621) * assign53930_e88631) - (assign53930_e88622 * (((((((2.0 * locals.var_nq_dn7) * locals.var_beta_ch) + (assign53930_e88625 * locals.var_beta_ch_dn7)) * locals.var_nvt) + (assign53930_e88627 * locals.var_nvt_dn7)) * locals.var_nvt) + (assign53930_e88629 * locals.var_nvt_dn7)))) / (assign53930_e88631 * assign53930_e88631)), ((((locals.var_ids_dn8 * assign53930_e88621) * assign53930_e88631) - (assign53930_e88622 * (((((((2.0 * locals.var_nq_dn8) * locals.var_beta_ch) + (assign53930_e88625 * locals.var_beta_ch_dn8)) * locals.var_nvt) + (assign53930_e88627 * locals.var_nvt_dn8)) * locals.var_nvt) + (assign53930_e88629 * locals.var_nvt_dn8)))) / (assign53930_e88631 * assign53930_e88631)), ((((locals.var_ids_dn9 * assign53930_e88621) * assign53930_e88631) - (assign53930_e88622 * (((((((2.0 * locals.var_nq_dn9) * locals.var_beta_ch) + (assign53930_e88625 * locals.var_beta_ch_dn9)) * locals.var_nvt) + (assign53930_e88627 * locals.var_nvt_dn9)) * locals.var_nvt) + (assign53930_e88629 * locals.var_nvt_dn9)))) / (assign53930_e88631 * assign53930_e88631)), ((((locals.var_ids_dn10 * assign53930_e88621) * assign53930_e88631) - (assign53930_e88622 * (((((((2.0 * locals.var_nq_dn10) * locals.var_beta_ch) + (assign53930_e88625 * locals.var_beta_ch_dn10)) * locals.var_nvt) + (assign53930_e88627 * locals.var_nvt_dn10)) * locals.var_nvt) + (assign53930_e88629 * locals.var_nvt_dn10)))) / (assign53930_e88631 * assign53930_e88631)), ((((locals.var_ids_dn11 * assign53930_e88621) * assign53930_e88631) - (assign53930_e88622 * (((((((2.0 * locals.var_nq_dn11) * locals.var_beta_ch) + (assign53930_e88625 * locals.var_beta_ch_dn11)) * locals.var_nvt) + (assign53930_e88627 * locals.var_nvt_dn11)) * locals.var_nvt) + (assign53930_e88629 * locals.var_nvt_dn11)))) / (assign53930_e88631 * assign53930_e88631)),)
    } else {
        (locals.var_i2, locals.var_i2_dn3, locals.var_i2_dn4, locals.var_i2_dn5, locals.var_i2_dn6, locals.var_i2_dn7, locals.var_i2_dn8, locals.var_i2_dn9, locals.var_i2_dn10, locals.var_i2_dn11,)
    }
};
        locals.var_i2 = assign53930_e88634;
        locals.var_i2_dn3 = assign53930_e88634_d_n3;
        locals.var_i2_dn4 = assign53930_e88634_d_n4;
        locals.var_i2_dn5 = assign53930_e88634_d_n5;
        locals.var_i2_dn6 = assign53930_e88634_d_n6;
        locals.var_i2_dn7 = assign53930_e88634_d_n7;
        locals.var_i2_dn8 = assign53930_e88634_d_n8;
        locals.var_i2_dn9 = assign53930_e88634_d_n9;
        locals.var_i2_dn10 = assign53930_e88634_d_n10;
        locals.var_i2_dn11 = assign53930_e88634_d_n11;

        let (assign53940_e88651, assign53940_e88651_d_n3, assign53940_e88651_d_n4, assign53940_e88651_d_n5, assign53940_e88651_d_n6, assign53940_e88651_d_n7, assign53940_e88651_d_n8, assign53940_e88651_d_n9, assign53940_e88651_d_n10, assign53940_e88651_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        let assign53940_e88643: f64 = (locals.var_qsh * locals.var_qsh);
        let assign53940_e88645: f64 = (assign53940_e88643 + locals.var_qsh);
        let assign53940_e88647: f64 = (assign53940_e88645 - locals.var_i1);
        let assign53940_e88648: f64 = (4.0 * assign53940_e88647);
        let assign53940_e88649: f64 = (1.0 + assign53940_e88648);
        (assign53940_e88649, (4.0 * ((((locals.var_qsh_dn3 * locals.var_qsh) + (locals.var_qsh * locals.var_qsh_dn3)) + locals.var_qsh_dn3) - locals.var_i1_dn3)), (4.0 * ((((locals.var_qsh_dn4 * locals.var_qsh) + (locals.var_qsh * locals.var_qsh_dn4)) + locals.var_qsh_dn4) - locals.var_i1_dn4)), (4.0 * ((((locals.var_qsh_dn5 * locals.var_qsh) + (locals.var_qsh * locals.var_qsh_dn5)) + locals.var_qsh_dn5) - locals.var_i1_dn5)), (4.0 * ((((locals.var_qsh_dn6 * locals.var_qsh) + (locals.var_qsh * locals.var_qsh_dn6)) + locals.var_qsh_dn6) - locals.var_i1_dn6)), (4.0 * ((((locals.var_qsh_dn7 * locals.var_qsh) + (locals.var_qsh * locals.var_qsh_dn7)) + locals.var_qsh_dn7) - locals.var_i1_dn7)), (4.0 * ((((locals.var_qsh_dn8 * locals.var_qsh) + (locals.var_qsh * locals.var_qsh_dn8)) + locals.var_qsh_dn8) - locals.var_i1_dn8)), (4.0 * ((((locals.var_qsh_dn9 * locals.var_qsh) + (locals.var_qsh * locals.var_qsh_dn9)) + locals.var_qsh_dn9) - locals.var_i1_dn9)), (4.0 * ((((locals.var_qsh_dn10 * locals.var_qsh) + (locals.var_qsh * locals.var_qsh_dn10)) + locals.var_qsh_dn10) - locals.var_i1_dn10)), (4.0 * ((((locals.var_qsh_dn11 * locals.var_qsh) + (locals.var_qsh * locals.var_qsh_dn11)) + locals.var_qsh_dn11) - locals.var_i1_dn11)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign53940_e88651;
        locals.var_t0_dn3 = assign53940_e88651_d_n3;
        locals.var_t0_dn4 = assign53940_e88651_d_n4;
        locals.var_t0_dn5 = assign53940_e88651_d_n5;
        locals.var_t0_dn6 = assign53940_e88651_d_n6;
        locals.var_t0_dn7 = assign53940_e88651_d_n7;
        locals.var_t0_dn8 = assign53940_e88651_d_n8;
        locals.var_t0_dn9 = assign53940_e88651_d_n9;
        locals.var_t0_dn10 = assign53940_e88651_d_n10;
        locals.var_t0_dn11 = assign53940_e88651_d_n11;

        let (assign53980_e88702, assign53980_e88702_d_n3, assign53980_e88702_d_n4, assign53980_e88702_d_n5, assign53980_e88702_d_n6, assign53980_e88702_d_n7, assign53980_e88702_d_n8, assign53980_e88702_d_n9, assign53980_e88702_d_n10, assign53980_e88702_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        let assign53980_e88685: f64 = (-0.5);
        let assign53980_e88691: f64 = (locals.var_qdeff * locals.var_qdeff);
        let assign53980_e88693: f64 = (assign53980_e88691 + locals.var_qdeff);
        let assign53980_e88695: f64 = (assign53980_e88693 + locals.var_i2);
        let assign53980_e88696: f64 = (4.0 * assign53980_e88695);
        let assign53980_e88697: f64 = (1.0 + assign53980_e88696);
        let assign53980_e88698: f64 = (assign53980_e88697).sqrt();
        let assign53980_e88699: f64 = (0.5 * assign53980_e88698);
        let assign53980_e88700: f64 = (assign53980_e88685 + assign53980_e88699);
        (assign53980_e88700, (0.5 * ((4.0 * ((((locals.var_qdeff_dn3 * locals.var_qdeff) + (locals.var_qdeff * locals.var_qdeff_dn3)) + locals.var_qdeff_dn3) + locals.var_i2_dn3)) / (2.0 * assign53980_e88698))), (0.5 * ((4.0 * ((((locals.var_qdeff_dn4 * locals.var_qdeff) + (locals.var_qdeff * locals.var_qdeff_dn4)) + locals.var_qdeff_dn4) + locals.var_i2_dn4)) / (2.0 * assign53980_e88698))), (0.5 * ((4.0 * ((((locals.var_qdeff_dn5 * locals.var_qdeff) + (locals.var_qdeff * locals.var_qdeff_dn5)) + locals.var_qdeff_dn5) + locals.var_i2_dn5)) / (2.0 * assign53980_e88698))), (0.5 * ((4.0 * ((((locals.var_qdeff_dn6 * locals.var_qdeff) + (locals.var_qdeff * locals.var_qdeff_dn6)) + locals.var_qdeff_dn6) + locals.var_i2_dn6)) / (2.0 * assign53980_e88698))), (0.5 * ((4.0 * ((((locals.var_qdeff_dn7 * locals.var_qdeff) + (locals.var_qdeff * locals.var_qdeff_dn7)) + locals.var_qdeff_dn7) + locals.var_i2_dn7)) / (2.0 * assign53980_e88698))), (0.5 * ((4.0 * ((((locals.var_qdeff_dn8 * locals.var_qdeff) + (locals.var_qdeff * locals.var_qdeff_dn8)) + locals.var_qdeff_dn8) + locals.var_i2_dn8)) / (2.0 * assign53980_e88698))), (0.5 * ((4.0 * ((((locals.var_qdeff_dn9 * locals.var_qdeff) + (locals.var_qdeff * locals.var_qdeff_dn9)) + locals.var_qdeff_dn9) + locals.var_i2_dn9)) / (2.0 * assign53980_e88698))), (0.5 * ((4.0 * ((((locals.var_qdeff_dn10 * locals.var_qdeff) + (locals.var_qdeff * locals.var_qdeff_dn10)) + locals.var_qdeff_dn10) + locals.var_i2_dn10)) / (2.0 * assign53980_e88698))), (0.5 * ((4.0 * ((((locals.var_qdeff_dn11 * locals.var_qdeff) + (locals.var_qdeff * locals.var_qdeff_dn11)) + locals.var_qdeff_dn11) + locals.var_i2_dn11)) / (2.0 * assign53980_e88698))),)
    } else {
        (locals.var_qsch, locals.var_qsch_dn3, locals.var_qsch_dn4, locals.var_qsch_dn5, locals.var_qsch_dn6, locals.var_qsch_dn7, locals.var_qsch_dn8, locals.var_qsch_dn9, locals.var_qsch_dn10, locals.var_qsch_dn11,)
    }
};
        locals.var_qsch = assign53980_e88702;
        locals.var_qsch_dn3 = assign53980_e88702_d_n3;
        locals.var_qsch_dn4 = assign53980_e88702_d_n4;
        locals.var_qsch_dn5 = assign53980_e88702_d_n5;
        locals.var_qsch_dn6 = assign53980_e88702_d_n6;
        locals.var_qsch_dn7 = assign53980_e88702_d_n7;
        locals.var_qsch_dn8 = assign53980_e88702_d_n8;
        locals.var_qsch_dn9 = assign53980_e88702_d_n9;
        locals.var_qsch_dn10 = assign53980_e88702_d_n10;
        locals.var_qsch_dn11 = assign53980_e88702_d_n11;

        let assign54070_e88811: f64 = if locals.var_leff != locals.var_lh1 { 1.0 } else { 0.0 };
        locals.var_guard830 = assign54070_e88811;

        let (assign54080_e88830, assign54080_e88830_d_n3, assign54080_e88830_d_n4, assign54080_e88830_d_n5, assign54080_e88830_d_n6, assign54080_e88830_d_n7, assign54080_e88830_d_n8, assign54080_e88830_d_n9, assign54080_e88830_d_n10, assign54080_e88830_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard830 != 0.0)) {
        let assign54080_e88820: f64 = (2.0 * locals.var_nq);
        let assign54080_e88822: f64 = (assign54080_e88820 * locals.var_cox);
        let assign54080_e88824: f64 = (assign54080_e88822 * locals.var_vt);
        let assign54080_e88826: f64 = (assign54080_e88824 * locals.var_qsch);
        let assign54080_e88828: f64 = (assign54080_e88826 / 1.602176462e-19);
        (assign54080_e88828, ((((((2.0 * locals.var_nq_dn3) * locals.var_cox) * locals.var_vt) * locals.var_qsch) + (assign54080_e88824 * locals.var_qsch_dn3)) / 1.602176462e-19), (((((((2.0 * locals.var_nq_dn4) * locals.var_cox) * locals.var_vt) + (assign54080_e88822 * locals.var_vt_dn4)) * locals.var_qsch) + (assign54080_e88824 * locals.var_qsch_dn4)) / 1.602176462e-19), (((((((2.0 * locals.var_nq_dn5) * locals.var_cox) * locals.var_vt) + (assign54080_e88822 * locals.var_vt_dn5)) * locals.var_qsch) + (assign54080_e88824 * locals.var_qsch_dn5)) / 1.602176462e-19), ((((((2.0 * locals.var_nq_dn6) * locals.var_cox) * locals.var_vt) * locals.var_qsch) + (assign54080_e88824 * locals.var_qsch_dn6)) / 1.602176462e-19), ((((((2.0 * locals.var_nq_dn7) * locals.var_cox) * locals.var_vt) * locals.var_qsch) + (assign54080_e88824 * locals.var_qsch_dn7)) / 1.602176462e-19), ((((((2.0 * locals.var_nq_dn8) * locals.var_cox) * locals.var_vt) * locals.var_qsch) + (assign54080_e88824 * locals.var_qsch_dn8)) / 1.602176462e-19), ((((((2.0 * locals.var_nq_dn9) * locals.var_cox) * locals.var_vt) * locals.var_qsch) + (assign54080_e88824 * locals.var_qsch_dn9)) / 1.602176462e-19), ((((((2.0 * locals.var_nq_dn10) * locals.var_cox) * locals.var_vt) * locals.var_qsch) + (assign54080_e88824 * locals.var_qsch_dn10)) / 1.602176462e-19), ((((((2.0 * locals.var_nq_dn11) * locals.var_cox) * locals.var_vt) * locals.var_qsch) + (assign54080_e88824 * locals.var_qsch_dn11)) / 1.602176462e-19),)
    } else {
        (locals.var_np2, locals.var_np2_dn3, locals.var_np2_dn4, locals.var_np2_dn5, locals.var_np2_dn6, locals.var_np2_dn7, locals.var_np2_dn8, locals.var_np2_dn9, locals.var_np2_dn10, locals.var_np2_dn11,)
    }
};
        locals.var_np2 = assign54080_e88830;
        locals.var_np2_dn3 = assign54080_e88830_d_n3;
        locals.var_np2_dn4 = assign54080_e88830_d_n4;
        locals.var_np2_dn5 = assign54080_e88830_d_n5;
        locals.var_np2_dn6 = assign54080_e88830_d_n6;
        locals.var_np2_dn7 = assign54080_e88830_d_n7;
        locals.var_np2_dn8 = assign54080_e88830_d_n8;
        locals.var_np2_dn9 = assign54080_e88830_d_n9;
        locals.var_np2_dn10 = assign54080_e88830_d_n10;
        locals.var_np2_dn11 = assign54080_e88830_d_n11;

        let (assign54090_e88845,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard830 != 0.0)) {
        let assign54090_e88840: f64 = (2.0 * locals.var_lintnoi_i);
        let assign54090_e88841: f64 = (locals.var_leffnoih - assign54090_e88840);
        let assign54090_e88843: f64 = (assign54090_e88841 - locals.var_lh1);
        (assign54090_e88843,)
    } else {
        (locals.var_leffnoi,)
    }
};
        locals.var_leffnoi = assign54090_e88845;

        let (assign54100_e88856,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard830 != 0.0)) {
        let assign54100_e88854: f64 = (locals.var_leffnoi * locals.var_leffnoi);
        (assign54100_e88854,)
    } else {
        (locals.var_leffnoisq,)
    }
};
        locals.var_leffnoisq = assign54100_e88856;

        let (assign54110_e88869, assign54110_e88869_d_n3, assign54110_e88869_d_n4, assign54110_e88869_d_n5, assign54110_e88869_d_n6, assign54110_e88869_d_n7, assign54110_e88869_d_n8, assign54110_e88869_d_n9, assign54110_e88869_d_n10, assign54110_e88869_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard830 != 0.0)) {
        let assign54110_e88865: f64 = (10000000000.0 * locals.var_cox);
        let assign54110_e88867: f64 = (assign54110_e88865 * locals.var_leffnoisq);
        (assign54110_e88867, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign54110_e88869;
        locals.var_t1_dn3 = assign54110_e88869_d_n3;
        locals.var_t1_dn4 = assign54110_e88869_d_n4;
        locals.var_t1_dn5 = assign54110_e88869_d_n5;
        locals.var_t1_dn6 = assign54110_e88869_d_n6;
        locals.var_t1_dn7 = assign54110_e88869_d_n7;
        locals.var_t1_dn8 = assign54110_e88869_d_n8;
        locals.var_t1_dn9 = assign54110_e88869_d_n9;
        locals.var_t1_dn10 = assign54110_e88869_d_n10;
        locals.var_t1_dn11 = assign54110_e88869_d_n11;

        let (assign54120_e88889, assign54120_e88889_d_n3, assign54120_e88889_d_n4, assign54120_e88889_d_n5, assign54120_e88889_d_n6, assign54120_e88889_d_n7, assign54120_e88889_d_n8, assign54120_e88889_d_n9, assign54120_e88889_d_n10, assign54120_e88889_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard830 != 0.0)) {
        let assign54120_e88879: f64 = (locals.var_np2 + locals.var_nstar);
        let assign54120_e88882: f64 = (locals.var_nl + locals.var_nstar);
        let assign54120_e88883: f64 = (assign54120_e88879 / assign54120_e88882);
        let assign54120_e88885: f64 = (assign54120_e88883).max(1e-38);
        let assign54120_e88886: f64 = (assign54120_e88885).ln();
        let assign54120_e88887: f64 = (p.p1012 * assign54120_e88886);
        (assign54120_e88887, (p.p1012 * (if assign54120_e88883 >= 1e-38 { ((((locals.var_np2_dn3 + locals.var_nstar_dn3) * assign54120_e88882) - (assign54120_e88879 * (locals.var_nl_dn3 + locals.var_nstar_dn3))) / (assign54120_e88882 * assign54120_e88882)) } else { 0.0 } / assign54120_e88885)), (p.p1012 * (if assign54120_e88883 >= 1e-38 { ((((locals.var_np2_dn4 + locals.var_nstar_dn4) * assign54120_e88882) - (assign54120_e88879 * (locals.var_nl_dn4 + locals.var_nstar_dn4))) / (assign54120_e88882 * assign54120_e88882)) } else { 0.0 } / assign54120_e88885)), (p.p1012 * (if assign54120_e88883 >= 1e-38 { ((((locals.var_np2_dn5 + locals.var_nstar_dn5) * assign54120_e88882) - (assign54120_e88879 * (locals.var_nl_dn5 + locals.var_nstar_dn5))) / (assign54120_e88882 * assign54120_e88882)) } else { 0.0 } / assign54120_e88885)), (p.p1012 * (if assign54120_e88883 >= 1e-38 { ((((locals.var_np2_dn6 + locals.var_nstar_dn6) * assign54120_e88882) - (assign54120_e88879 * (locals.var_nl_dn6 + locals.var_nstar_dn6))) / (assign54120_e88882 * assign54120_e88882)) } else { 0.0 } / assign54120_e88885)), (p.p1012 * (if assign54120_e88883 >= 1e-38 { ((((locals.var_np2_dn7 + locals.var_nstar_dn7) * assign54120_e88882) - (assign54120_e88879 * (locals.var_nl_dn7 + locals.var_nstar_dn7))) / (assign54120_e88882 * assign54120_e88882)) } else { 0.0 } / assign54120_e88885)), (p.p1012 * (if assign54120_e88883 >= 1e-38 { ((((locals.var_np2_dn8 + locals.var_nstar_dn8) * assign54120_e88882) - (assign54120_e88879 * (locals.var_nl_dn8 + locals.var_nstar_dn8))) / (assign54120_e88882 * assign54120_e88882)) } else { 0.0 } / assign54120_e88885)), (p.p1012 * (if assign54120_e88883 >= 1e-38 { ((((locals.var_np2_dn9 + locals.var_nstar_dn9) * assign54120_e88882) - (assign54120_e88879 * (locals.var_nl_dn9 + locals.var_nstar_dn9))) / (assign54120_e88882 * assign54120_e88882)) } else { 0.0 } / assign54120_e88885)), (p.p1012 * (if assign54120_e88883 >= 1e-38 { ((((locals.var_np2_dn10 + locals.var_nstar_dn10) * assign54120_e88882) - (assign54120_e88879 * (locals.var_nl_dn10 + locals.var_nstar_dn10))) / (assign54120_e88882 * assign54120_e88882)) } else { 0.0 } / assign54120_e88885)), (p.p1012 * (if assign54120_e88883 >= 1e-38 { ((((locals.var_np2_dn11 + locals.var_nstar_dn11) * assign54120_e88882) - (assign54120_e88879 * (locals.var_nl_dn11 + locals.var_nstar_dn11))) / (assign54120_e88882 * assign54120_e88882)) } else { 0.0 } / assign54120_e88885)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign54120_e88889;
        locals.var_t2_dn3 = assign54120_e88889_d_n3;
        locals.var_t2_dn4 = assign54120_e88889_d_n4;
        locals.var_t2_dn5 = assign54120_e88889_d_n5;
        locals.var_t2_dn6 = assign54120_e88889_d_n6;
        locals.var_t2_dn7 = assign54120_e88889_d_n7;
        locals.var_t2_dn8 = assign54120_e88889_d_n8;
        locals.var_t2_dn9 = assign54120_e88889_d_n9;
        locals.var_t2_dn10 = assign54120_e88889_d_n10;
        locals.var_t2_dn11 = assign54120_e88889_d_n11;

        let (assign54130_e88902, assign54130_e88902_d_n3, assign54130_e88902_d_n4, assign54130_e88902_d_n5, assign54130_e88902_d_n6, assign54130_e88902_d_n7, assign54130_e88902_d_n8, assign54130_e88902_d_n9, assign54130_e88902_d_n10, assign54130_e88902_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard830 != 0.0)) {
        let assign54130_e88899: f64 = (locals.var_np2 - locals.var_nl);
        let assign54130_e88900: f64 = (p.p1013 * assign54130_e88899);
        (assign54130_e88900, (p.p1013 * (locals.var_np2_dn3 - locals.var_nl_dn3)), (p.p1013 * (locals.var_np2_dn4 - locals.var_nl_dn4)), (p.p1013 * (locals.var_np2_dn5 - locals.var_nl_dn5)), (p.p1013 * (locals.var_np2_dn6 - locals.var_nl_dn6)), (p.p1013 * (locals.var_np2_dn7 - locals.var_nl_dn7)), (p.p1013 * (locals.var_np2_dn8 - locals.var_nl_dn8)), (p.p1013 * (locals.var_np2_dn9 - locals.var_nl_dn9)), (p.p1013 * (locals.var_np2_dn10 - locals.var_nl_dn10)), (p.p1013 * (locals.var_np2_dn11 - locals.var_nl_dn11)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign54130_e88902;
        locals.var_t3_dn3 = assign54130_e88902_d_n3;
        locals.var_t3_dn4 = assign54130_e88902_d_n4;
        locals.var_t3_dn5 = assign54130_e88902_d_n5;
        locals.var_t3_dn6 = assign54130_e88902_d_n6;
        locals.var_t3_dn7 = assign54130_e88902_d_n7;
        locals.var_t3_dn8 = assign54130_e88902_d_n8;
        locals.var_t3_dn9 = assign54130_e88902_d_n9;
        locals.var_t3_dn10 = assign54130_e88902_d_n10;
        locals.var_t3_dn11 = assign54130_e88902_d_n11;

        let (assign54140_e88921, assign54140_e88921_d_n3, assign54140_e88921_d_n4, assign54140_e88921_d_n5, assign54140_e88921_d_n6, assign54140_e88921_d_n7, assign54140_e88921_d_n8, assign54140_e88921_d_n9, assign54140_e88921_d_n10, assign54140_e88921_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard830 != 0.0)) {
        let assign54140_e88911: f64 = (0.5 * p.p1014);
        let assign54140_e88914: f64 = (locals.var_np2 * locals.var_np2);
        let assign54140_e88917: f64 = (locals.var_nl * locals.var_nl);
        let assign54140_e88918: f64 = (assign54140_e88914 - assign54140_e88917);
        let assign54140_e88919: f64 = (assign54140_e88911 * assign54140_e88918);
        (assign54140_e88919, (assign54140_e88911 * (((locals.var_np2_dn3 * locals.var_np2) + (locals.var_np2 * locals.var_np2_dn3)) - ((locals.var_nl_dn3 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn3)))), (assign54140_e88911 * (((locals.var_np2_dn4 * locals.var_np2) + (locals.var_np2 * locals.var_np2_dn4)) - ((locals.var_nl_dn4 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn4)))), (assign54140_e88911 * (((locals.var_np2_dn5 * locals.var_np2) + (locals.var_np2 * locals.var_np2_dn5)) - ((locals.var_nl_dn5 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn5)))), (assign54140_e88911 * (((locals.var_np2_dn6 * locals.var_np2) + (locals.var_np2 * locals.var_np2_dn6)) - ((locals.var_nl_dn6 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn6)))), (assign54140_e88911 * (((locals.var_np2_dn7 * locals.var_np2) + (locals.var_np2 * locals.var_np2_dn7)) - ((locals.var_nl_dn7 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn7)))), (assign54140_e88911 * (((locals.var_np2_dn8 * locals.var_np2) + (locals.var_np2 * locals.var_np2_dn8)) - ((locals.var_nl_dn8 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn8)))), (assign54140_e88911 * (((locals.var_np2_dn9 * locals.var_np2) + (locals.var_np2 * locals.var_np2_dn9)) - ((locals.var_nl_dn9 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn9)))), (assign54140_e88911 * (((locals.var_np2_dn10 * locals.var_np2) + (locals.var_np2 * locals.var_np2_dn10)) - ((locals.var_nl_dn10 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn10)))), (assign54140_e88911 * (((locals.var_np2_dn11 * locals.var_np2) + (locals.var_np2 * locals.var_np2_dn11)) - ((locals.var_nl_dn11 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn11)))),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign54140_e88921;
        locals.var_t4_dn3 = assign54140_e88921_d_n3;
        locals.var_t4_dn4 = assign54140_e88921_d_n4;
        locals.var_t4_dn5 = assign54140_e88921_d_n5;
        locals.var_t4_dn6 = assign54140_e88921_d_n6;
        locals.var_t4_dn7 = assign54140_e88921_d_n7;
        locals.var_t4_dn8 = assign54140_e88921_d_n8;
        locals.var_t4_dn9 = assign54140_e88921_d_n9;
        locals.var_t4_dn10 = assign54140_e88921_d_n10;
        locals.var_t4_dn11 = assign54140_e88921_d_n11;

        let (assign54150_e88936, assign54150_e88936_d_n3, assign54150_e88936_d_n4, assign54150_e88936_d_n5, assign54150_e88936_d_n6, assign54150_e88936_d_n7, assign54150_e88936_d_n8, assign54150_e88936_d_n9, assign54150_e88936_d_n10, assign54150_e88936_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard830 != 0.0)) {
        let assign54150_e88930: f64 = (10000000000.0 * locals.var_leffnoisq);
        let assign54150_e88932: f64 = (assign54150_e88930 * locals.var_weff);
        let assign54150_e88934: f64 = (assign54150_e88932 * p.p2);
        (assign54150_e88934, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign54150_e88936;
        locals.var_t5_dn3 = assign54150_e88936_d_n3;
        locals.var_t5_dn4 = assign54150_e88936_d_n4;
        locals.var_t5_dn5 = assign54150_e88936_d_n5;
        locals.var_t5_dn6 = assign54150_e88936_d_n6;
        locals.var_t5_dn7 = assign54150_e88936_d_n7;
        locals.var_t5_dn8 = assign54150_e88936_d_n8;
        locals.var_t5_dn9 = assign54150_e88936_d_n9;
        locals.var_t5_dn10 = assign54150_e88936_d_n10;
        locals.var_t5_dn11 = assign54150_e88936_d_n11;

        let (assign54160_e88963, assign54160_e88963_d_n3, assign54160_e88963_d_n4, assign54160_e88963_d_n5, assign54160_e88963_d_n6, assign54160_e88963_d_n7, assign54160_e88963_d_n8, assign54160_e88963_d_n9, assign54160_e88963_d_n10, assign54160_e88963_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard830 != 0.0)) {
        let assign54160_e88945: f64 = (locals.var_t0a / locals.var_t1);
        let assign54160_e88948: f64 = (locals.var_t2 + locals.var_t3);
        let assign54160_e88950: f64 = (assign54160_e88948 + locals.var_t4);
        let assign54160_e88951: f64 = (assign54160_e88945 * assign54160_e88950);
        let assign54160_e88954: f64 = (locals.var_t0b / locals.var_t5);
        let assign54160_e88956: f64 = (assign54160_e88954 * locals.var_delclm);
        let assign54160_e88958: f64 = (assign54160_e88956 * locals.var_t0c);
        let assign54160_e88960: f64 = (assign54160_e88958 / locals.var_t0d);
        let assign54160_e88961: f64 = (assign54160_e88951 + assign54160_e88960);
        (assign54160_e88961, ((((((locals.var_t0a_dn3 * locals.var_t1) - (locals.var_t0a * locals.var_t1_dn3)) / (locals.var_t1 * locals.var_t1)) * assign54160_e88950) + (assign54160_e88945 * ((locals.var_t2_dn3 + locals.var_t3_dn3) + locals.var_t4_dn3))) + ((((((((((locals.var_t0b_dn3 * locals.var_t5) - (locals.var_t0b * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5)) * locals.var_delclm) + (assign54160_e88954 * locals.var_delclm_dn3)) * locals.var_t0c) + (assign54160_e88956 * locals.var_t0c_dn3)) * locals.var_t0d) - (assign54160_e88958 * locals.var_t0d_dn3)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn4 * locals.var_t1) - (locals.var_t0a * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)) * assign54160_e88950) + (assign54160_e88945 * ((locals.var_t2_dn4 + locals.var_t3_dn4) + locals.var_t4_dn4))) + ((((((((((locals.var_t0b_dn4 * locals.var_t5) - (locals.var_t0b * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)) * locals.var_delclm) + (assign54160_e88954 * locals.var_delclm_dn4)) * locals.var_t0c) + (assign54160_e88956 * locals.var_t0c_dn4)) * locals.var_t0d) - (assign54160_e88958 * locals.var_t0d_dn4)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn5 * locals.var_t1) - (locals.var_t0a * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)) * assign54160_e88950) + (assign54160_e88945 * ((locals.var_t2_dn5 + locals.var_t3_dn5) + locals.var_t4_dn5))) + ((((((((((locals.var_t0b_dn5 * locals.var_t5) - (locals.var_t0b * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)) * locals.var_delclm) + (assign54160_e88954 * locals.var_delclm_dn5)) * locals.var_t0c) + (assign54160_e88956 * locals.var_t0c_dn5)) * locals.var_t0d) - (assign54160_e88958 * locals.var_t0d_dn5)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn6 * locals.var_t1) - (locals.var_t0a * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1)) * assign54160_e88950) + (assign54160_e88945 * ((locals.var_t2_dn6 + locals.var_t3_dn6) + locals.var_t4_dn6))) + ((((((((((locals.var_t0b_dn6 * locals.var_t5) - (locals.var_t0b * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5)) * locals.var_delclm) + (assign54160_e88954 * locals.var_delclm_dn6)) * locals.var_t0c) + (assign54160_e88956 * locals.var_t0c_dn6)) * locals.var_t0d) - (assign54160_e88958 * locals.var_t0d_dn6)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn7 * locals.var_t1) - (locals.var_t0a * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1)) * assign54160_e88950) + (assign54160_e88945 * ((locals.var_t2_dn7 + locals.var_t3_dn7) + locals.var_t4_dn7))) + ((((((((((locals.var_t0b_dn7 * locals.var_t5) - (locals.var_t0b * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5)) * locals.var_delclm) + (assign54160_e88954 * locals.var_delclm_dn7)) * locals.var_t0c) + (assign54160_e88956 * locals.var_t0c_dn7)) * locals.var_t0d) - (assign54160_e88958 * locals.var_t0d_dn7)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn8 * locals.var_t1) - (locals.var_t0a * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1)) * assign54160_e88950) + (assign54160_e88945 * ((locals.var_t2_dn8 + locals.var_t3_dn8) + locals.var_t4_dn8))) + ((((((((((locals.var_t0b_dn8 * locals.var_t5) - (locals.var_t0b * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5)) * locals.var_delclm) + (assign54160_e88954 * locals.var_delclm_dn8)) * locals.var_t0c) + (assign54160_e88956 * locals.var_t0c_dn8)) * locals.var_t0d) - (assign54160_e88958 * locals.var_t0d_dn8)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn9 * locals.var_t1) - (locals.var_t0a * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1)) * assign54160_e88950) + (assign54160_e88945 * ((locals.var_t2_dn9 + locals.var_t3_dn9) + locals.var_t4_dn9))) + ((((((((((locals.var_t0b_dn9 * locals.var_t5) - (locals.var_t0b * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5)) * locals.var_delclm) + (assign54160_e88954 * locals.var_delclm_dn9)) * locals.var_t0c) + (assign54160_e88956 * locals.var_t0c_dn9)) * locals.var_t0d) - (assign54160_e88958 * locals.var_t0d_dn9)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn10 * locals.var_t1) - (locals.var_t0a * locals.var_t1_dn10)) / (locals.var_t1 * locals.var_t1)) * assign54160_e88950) + (assign54160_e88945 * ((locals.var_t2_dn10 + locals.var_t3_dn10) + locals.var_t4_dn10))) + ((((((((((locals.var_t0b_dn10 * locals.var_t5) - (locals.var_t0b * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5)) * locals.var_delclm) + (assign54160_e88954 * locals.var_delclm_dn10)) * locals.var_t0c) + (assign54160_e88956 * locals.var_t0c_dn10)) * locals.var_t0d) - (assign54160_e88958 * locals.var_t0d_dn10)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn11 * locals.var_t1) - (locals.var_t0a * locals.var_t1_dn11)) / (locals.var_t1 * locals.var_t1)) * assign54160_e88950) + (assign54160_e88945 * ((locals.var_t2_dn11 + locals.var_t3_dn11) + locals.var_t4_dn11))) + ((((((((((locals.var_t0b_dn11 * locals.var_t5) - (locals.var_t0b * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5)) * locals.var_delclm) + (assign54160_e88954 * locals.var_delclm_dn11)) * locals.var_t0c) + (assign54160_e88956 * locals.var_t0c_dn11)) * locals.var_t0d) - (assign54160_e88958 * locals.var_t0d_dn11)) / (locals.var_t0d * locals.var_t0d))),)
    } else {
        (locals.var_ssi_ch, locals.var_ssi_ch_dn3, locals.var_ssi_ch_dn4, locals.var_ssi_ch_dn5, locals.var_ssi_ch_dn6, locals.var_ssi_ch_dn7, locals.var_ssi_ch_dn8, locals.var_ssi_ch_dn9, locals.var_ssi_ch_dn10, locals.var_ssi_ch_dn11,)
    }
};
        locals.var_ssi_ch = assign54160_e88963;
        locals.var_ssi_ch_dn3 = assign54160_e88963_d_n3;
        locals.var_ssi_ch_dn4 = assign54160_e88963_d_n4;
        locals.var_ssi_ch_dn5 = assign54160_e88963_d_n5;
        locals.var_ssi_ch_dn6 = assign54160_e88963_d_n6;
        locals.var_ssi_ch_dn7 = assign54160_e88963_d_n7;
        locals.var_ssi_ch_dn8 = assign54160_e88963_d_n8;
        locals.var_ssi_ch_dn9 = assign54160_e88963_d_n9;
        locals.var_ssi_ch_dn10 = assign54160_e88963_d_n10;
        locals.var_ssi_ch_dn11 = assign54160_e88963_d_n11;

        let (assign54170_e88982, assign54170_e88982_d_n3, assign54170_e88982_d_n4, assign54170_e88982_d_n5, assign54170_e88982_d_n6, assign54170_e88982_d_n7, assign54170_e88982_d_n8, assign54170_e88982_d_n9, assign54170_e88982_d_n10, assign54170_e88982_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard830 != 0.0)) {
        let assign54170_e88972: f64 = (locals.var_weff * p.p2);
        let assign54170_e88974: f64 = (assign54170_e88972 * locals.var_leffnoi);
        let assign54170_e88976: f64 = (assign54170_e88974 * 10000000000.0);
        let assign54170_e88978: f64 = (assign54170_e88976 * locals.var_nstar);
        let assign54170_e88980: f64 = (assign54170_e88978 * locals.var_nstar);
        (assign54170_e88980, (((assign54170_e88976 * locals.var_nstar_dn3) * locals.var_nstar) + (assign54170_e88978 * locals.var_nstar_dn3)), (((assign54170_e88976 * locals.var_nstar_dn4) * locals.var_nstar) + (assign54170_e88978 * locals.var_nstar_dn4)), (((assign54170_e88976 * locals.var_nstar_dn5) * locals.var_nstar) + (assign54170_e88978 * locals.var_nstar_dn5)), (((assign54170_e88976 * locals.var_nstar_dn6) * locals.var_nstar) + (assign54170_e88978 * locals.var_nstar_dn6)), (((assign54170_e88976 * locals.var_nstar_dn7) * locals.var_nstar) + (assign54170_e88978 * locals.var_nstar_dn7)), (((assign54170_e88976 * locals.var_nstar_dn8) * locals.var_nstar) + (assign54170_e88978 * locals.var_nstar_dn8)), (((assign54170_e88976 * locals.var_nstar_dn9) * locals.var_nstar) + (assign54170_e88978 * locals.var_nstar_dn9)), (((assign54170_e88976 * locals.var_nstar_dn10) * locals.var_nstar) + (assign54170_e88978 * locals.var_nstar_dn10)), (((assign54170_e88976 * locals.var_nstar_dn11) * locals.var_nstar) + (assign54170_e88978 * locals.var_nstar_dn11)),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign54170_e88982;
        locals.var_t6_dn3 = assign54170_e88982_d_n3;
        locals.var_t6_dn4 = assign54170_e88982_d_n4;
        locals.var_t6_dn5 = assign54170_e88982_d_n5;
        locals.var_t6_dn6 = assign54170_e88982_d_n6;
        locals.var_t6_dn7 = assign54170_e88982_d_n7;
        locals.var_t6_dn8 = assign54170_e88982_d_n8;
        locals.var_t6_dn9 = assign54170_e88982_d_n9;
        locals.var_t6_dn10 = assign54170_e88982_d_n10;
        locals.var_t6_dn11 = assign54170_e88982_d_n11;

        let (assign54180_e88997, assign54180_e88997_d_n3, assign54180_e88997_d_n4, assign54180_e88997_d_n5, assign54180_e88997_d_n6, assign54180_e88997_d_n7, assign54180_e88997_d_n8, assign54180_e88997_d_n9, assign54180_e88997_d_n10, assign54180_e88997_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard830 != 0.0)) {
        let assign54180_e88991: f64 = (locals.var_t0e / locals.var_t6);
        let assign54180_e88993: f64 = (assign54180_e88991 * locals.var_ids);
        let assign54180_e88995: f64 = (assign54180_e88993 * locals.var_ids);
        (assign54180_e88995, (((((-((locals.var_t0e * locals.var_t6_dn3) / (locals.var_t6 * locals.var_t6))) * locals.var_ids) + (assign54180_e88991 * locals.var_ids_dn3)) * locals.var_ids) + (assign54180_e88993 * locals.var_ids_dn3)), (((((((locals.var_t0e_dn4 * locals.var_t6) - (locals.var_t0e * locals.var_t6_dn4)) / (locals.var_t6 * locals.var_t6)) * locals.var_ids) + (assign54180_e88991 * locals.var_ids_dn4)) * locals.var_ids) + (assign54180_e88993 * locals.var_ids_dn4)), (((((((locals.var_t0e_dn5 * locals.var_t6) - (locals.var_t0e * locals.var_t6_dn5)) / (locals.var_t6 * locals.var_t6)) * locals.var_ids) + (assign54180_e88991 * locals.var_ids_dn5)) * locals.var_ids) + (assign54180_e88993 * locals.var_ids_dn5)), (((((-((locals.var_t0e * locals.var_t6_dn6) / (locals.var_t6 * locals.var_t6))) * locals.var_ids) + (assign54180_e88991 * locals.var_ids_dn6)) * locals.var_ids) + (assign54180_e88993 * locals.var_ids_dn6)), (((((-((locals.var_t0e * locals.var_t6_dn7) / (locals.var_t6 * locals.var_t6))) * locals.var_ids) + (assign54180_e88991 * locals.var_ids_dn7)) * locals.var_ids) + (assign54180_e88993 * locals.var_ids_dn7)), (((((-((locals.var_t0e * locals.var_t6_dn8) / (locals.var_t6 * locals.var_t6))) * locals.var_ids) + (assign54180_e88991 * locals.var_ids_dn8)) * locals.var_ids) + (assign54180_e88993 * locals.var_ids_dn8)), (((((-((locals.var_t0e * locals.var_t6_dn9) / (locals.var_t6 * locals.var_t6))) * locals.var_ids) + (assign54180_e88991 * locals.var_ids_dn9)) * locals.var_ids) + (assign54180_e88993 * locals.var_ids_dn9)), (((((-((locals.var_t0e * locals.var_t6_dn10) / (locals.var_t6 * locals.var_t6))) * locals.var_ids) + (assign54180_e88991 * locals.var_ids_dn10)) * locals.var_ids) + (assign54180_e88993 * locals.var_ids_dn10)), (((((-((locals.var_t0e * locals.var_t6_dn11) / (locals.var_t6 * locals.var_t6))) * locals.var_ids) + (assign54180_e88991 * locals.var_ids_dn11)) * locals.var_ids) + (assign54180_e88993 * locals.var_ids_dn11)),)
    } else {
        (locals.var_swi_ch, locals.var_swi_ch_dn3, locals.var_swi_ch_dn4, locals.var_swi_ch_dn5, locals.var_swi_ch_dn6, locals.var_swi_ch_dn7, locals.var_swi_ch_dn8, locals.var_swi_ch_dn9, locals.var_swi_ch_dn10, locals.var_swi_ch_dn11,)
    }
};
        locals.var_swi_ch = assign54180_e88997;
        locals.var_swi_ch_dn3 = assign54180_e88997_d_n3;
        locals.var_swi_ch_dn4 = assign54180_e88997_d_n4;
        locals.var_swi_ch_dn5 = assign54180_e88997_d_n5;
        locals.var_swi_ch_dn6 = assign54180_e88997_d_n6;
        locals.var_swi_ch_dn7 = assign54180_e88997_d_n7;
        locals.var_swi_ch_dn8 = assign54180_e88997_d_n8;
        locals.var_swi_ch_dn9 = assign54180_e88997_d_n9;
        locals.var_swi_ch_dn10 = assign54180_e88997_d_n10;
        locals.var_swi_ch_dn11 = assign54180_e88997_d_n11;

        let (assign54190_e89008, assign54190_e89008_d_n3, assign54190_e89008_d_n4, assign54190_e89008_d_n5, assign54190_e89008_d_n6, assign54190_e89008_d_n7, assign54190_e89008_d_n8, assign54190_e89008_d_n9, assign54190_e89008_d_n10, assign54190_e89008_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) && (locals.var_guard830 != 0.0)) {
        let assign54190_e89006: f64 = (locals.var_swi_ch + locals.var_ssi_ch);
        (assign54190_e89006, (locals.var_swi_ch_dn3 + locals.var_ssi_ch_dn3), (locals.var_swi_ch_dn4 + locals.var_ssi_ch_dn4), (locals.var_swi_ch_dn5 + locals.var_ssi_ch_dn5), (locals.var_swi_ch_dn6 + locals.var_ssi_ch_dn6), (locals.var_swi_ch_dn7 + locals.var_ssi_ch_dn7), (locals.var_swi_ch_dn8 + locals.var_ssi_ch_dn8), (locals.var_swi_ch_dn9 + locals.var_ssi_ch_dn9), (locals.var_swi_ch_dn10 + locals.var_ssi_ch_dn10), (locals.var_swi_ch_dn11 + locals.var_ssi_ch_dn11),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign54190_e89008;
        locals.var_t7_dn3 = assign54190_e89008_d_n3;
        locals.var_t7_dn4 = assign54190_e89008_d_n4;
        locals.var_t7_dn5 = assign54190_e89008_d_n5;
        locals.var_t7_dn6 = assign54190_e89008_d_n6;
        locals.var_t7_dn7 = assign54190_e89008_d_n7;
        locals.var_t7_dn8 = assign54190_e89008_d_n8;
        locals.var_t7_dn9 = assign54190_e89008_d_n9;
        locals.var_t7_dn10 = assign54190_e89008_d_n10;
        locals.var_t7_dn11 = assign54190_e89008_d_n11;

        let (assign54240_e89059, assign54240_e89059_d_n3, assign54240_e89059_d_n4, assign54240_e89059_d_n5, assign54240_e89059_d_n6, assign54240_e89059_d_n7, assign54240_e89059_d_n8, assign54240_e89059_d_n9, assign54240_e89059_d_n10, assign54240_e89059_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        let assign54240_e89055: f64 = (p.p1321 * 1.602176462e-19);
        let assign54240_e89057: f64 = (assign54240_e89055 * locals.var_vt);
        (assign54240_e89057, 0.0, (assign54240_e89055 * locals.var_vt_dn4), (assign54240_e89055 * locals.var_vt_dn5), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t8, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11,)
    }
};
        locals.var_t8 = assign54240_e89059;
        locals.var_t8_dn3 = assign54240_e89059_d_n3;
        locals.var_t8_dn4 = assign54240_e89059_d_n4;
        locals.var_t8_dn5 = assign54240_e89059_d_n5;
        locals.var_t8_dn6 = assign54240_e89059_d_n6;
        locals.var_t8_dn7 = assign54240_e89059_d_n7;
        locals.var_t8_dn8 = assign54240_e89059_d_n8;
        locals.var_t8_dn9 = assign54240_e89059_d_n9;
        locals.var_t8_dn10 = assign54240_e89059_d_n10;
        locals.var_t8_dn11 = assign54240_e89059_d_n11;

        let (assign54250_e89076, assign54250_e89076_d_n3, assign54250_e89076_d_n4, assign54250_e89076_d_n5, assign54250_e89076_d_n6, assign54250_e89076_d_n7, assign54250_e89076_d_n8, assign54250_e89076_d_n9, assign54250_e89076_d_n10, assign54250_e89076_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        let assign54250_e89066: f64 = (locals.var_weff * p.p2);
        let assign54250_e89068: f64 = (assign54250_e89066 * locals.var_lh1);
        let assign54250_e89070: f64 = (assign54250_e89068 * 10000000000.0);
        let assign54250_e89072: f64 = (assign54250_e89070 * locals.var_nstar);
        let assign54250_e89074: f64 = (assign54250_e89072 * locals.var_nstar);
        (assign54250_e89074, (((assign54250_e89070 * locals.var_nstar_dn3) * locals.var_nstar) + (assign54250_e89072 * locals.var_nstar_dn3)), (((assign54250_e89070 * locals.var_nstar_dn4) * locals.var_nstar) + (assign54250_e89072 * locals.var_nstar_dn4)), (((assign54250_e89070 * locals.var_nstar_dn5) * locals.var_nstar) + (assign54250_e89072 * locals.var_nstar_dn5)), (((assign54250_e89070 * locals.var_nstar_dn6) * locals.var_nstar) + (assign54250_e89072 * locals.var_nstar_dn6)), (((assign54250_e89070 * locals.var_nstar_dn7) * locals.var_nstar) + (assign54250_e89072 * locals.var_nstar_dn7)), (((assign54250_e89070 * locals.var_nstar_dn8) * locals.var_nstar) + (assign54250_e89072 * locals.var_nstar_dn8)), (((assign54250_e89070 * locals.var_nstar_dn9) * locals.var_nstar) + (assign54250_e89072 * locals.var_nstar_dn9)), (((assign54250_e89070 * locals.var_nstar_dn10) * locals.var_nstar) + (assign54250_e89072 * locals.var_nstar_dn10)), (((assign54250_e89070 * locals.var_nstar_dn11) * locals.var_nstar) + (assign54250_e89072 * locals.var_nstar_dn11)),)
    } else {
        (locals.var_t9, locals.var_t9_dn3, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11,)
    }
};
        locals.var_t9 = assign54250_e89076;
        locals.var_t9_dn3 = assign54250_e89076_d_n3;
        locals.var_t9_dn4 = assign54250_e89076_d_n4;
        locals.var_t9_dn5 = assign54250_e89076_d_n5;
        locals.var_t9_dn6 = assign54250_e89076_d_n6;
        locals.var_t9_dn7 = assign54250_e89076_d_n7;
        locals.var_t9_dn8 = assign54250_e89076_d_n8;
        locals.var_t9_dn9 = assign54250_e89076_d_n9;
        locals.var_t9_dn10 = assign54250_e89076_d_n10;
        locals.var_t9_dn11 = assign54250_e89076_d_n11;

        let (assign54260_e89089, assign54260_e89089_d_n3, assign54260_e89089_d_n4, assign54260_e89089_d_n5, assign54260_e89089_d_n6, assign54260_e89089_d_n7, assign54260_e89089_d_n8, assign54260_e89089_d_n9, assign54260_e89089_d_n10, assign54260_e89089_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        let assign54260_e89083: f64 = (locals.var_t8 / locals.var_t9);
        let assign54260_e89085: f64 = (assign54260_e89083 * locals.var_ids);
        let assign54260_e89087: f64 = (assign54260_e89085 * locals.var_ids);
        (assign54260_e89087, (((((((locals.var_t8_dn3 * locals.var_t9) - (locals.var_t8 * locals.var_t9_dn3)) / (locals.var_t9 * locals.var_t9)) * locals.var_ids) + (assign54260_e89083 * locals.var_ids_dn3)) * locals.var_ids) + (assign54260_e89085 * locals.var_ids_dn3)), (((((((locals.var_t8_dn4 * locals.var_t9) - (locals.var_t8 * locals.var_t9_dn4)) / (locals.var_t9 * locals.var_t9)) * locals.var_ids) + (assign54260_e89083 * locals.var_ids_dn4)) * locals.var_ids) + (assign54260_e89085 * locals.var_ids_dn4)), (((((((locals.var_t8_dn5 * locals.var_t9) - (locals.var_t8 * locals.var_t9_dn5)) / (locals.var_t9 * locals.var_t9)) * locals.var_ids) + (assign54260_e89083 * locals.var_ids_dn5)) * locals.var_ids) + (assign54260_e89085 * locals.var_ids_dn5)), (((((((locals.var_t8_dn6 * locals.var_t9) - (locals.var_t8 * locals.var_t9_dn6)) / (locals.var_t9 * locals.var_t9)) * locals.var_ids) + (assign54260_e89083 * locals.var_ids_dn6)) * locals.var_ids) + (assign54260_e89085 * locals.var_ids_dn6)), (((((((locals.var_t8_dn7 * locals.var_t9) - (locals.var_t8 * locals.var_t9_dn7)) / (locals.var_t9 * locals.var_t9)) * locals.var_ids) + (assign54260_e89083 * locals.var_ids_dn7)) * locals.var_ids) + (assign54260_e89085 * locals.var_ids_dn7)), (((((((locals.var_t8_dn8 * locals.var_t9) - (locals.var_t8 * locals.var_t9_dn8)) / (locals.var_t9 * locals.var_t9)) * locals.var_ids) + (assign54260_e89083 * locals.var_ids_dn8)) * locals.var_ids) + (assign54260_e89085 * locals.var_ids_dn8)), (((((((locals.var_t8_dn9 * locals.var_t9) - (locals.var_t8 * locals.var_t9_dn9)) / (locals.var_t9 * locals.var_t9)) * locals.var_ids) + (assign54260_e89083 * locals.var_ids_dn9)) * locals.var_ids) + (assign54260_e89085 * locals.var_ids_dn9)), (((((((locals.var_t8_dn10 * locals.var_t9) - (locals.var_t8 * locals.var_t9_dn10)) / (locals.var_t9 * locals.var_t9)) * locals.var_ids) + (assign54260_e89083 * locals.var_ids_dn10)) * locals.var_ids) + (assign54260_e89085 * locals.var_ids_dn10)), (((((((locals.var_t8_dn11 * locals.var_t9) - (locals.var_t8 * locals.var_t9_dn11)) / (locals.var_t9 * locals.var_t9)) * locals.var_ids) + (assign54260_e89083 * locals.var_ids_dn11)) * locals.var_ids) + (assign54260_e89085 * locals.var_ids_dn11)),)
    } else {
        (locals.var_swi_h, locals.var_swi_h_dn3, locals.var_swi_h_dn4, locals.var_swi_h_dn5, locals.var_swi_h_dn6, locals.var_swi_h_dn7, locals.var_swi_h_dn8, locals.var_swi_h_dn9, locals.var_swi_h_dn10, locals.var_swi_h_dn11,)
    }
};
        locals.var_swi_h = assign54260_e89089;
        locals.var_swi_h_dn3 = assign54260_e89089_d_n3;
        locals.var_swi_h_dn4 = assign54260_e89089_d_n4;
        locals.var_swi_h_dn5 = assign54260_e89089_d_n5;
        locals.var_swi_h_dn6 = assign54260_e89089_d_n6;
        locals.var_swi_h_dn7 = assign54260_e89089_d_n7;
        locals.var_swi_h_dn8 = assign54260_e89089_d_n8;
        locals.var_swi_h_dn9 = assign54260_e89089_d_n9;
        locals.var_swi_h_dn10 = assign54260_e89089_d_n10;
        locals.var_swi_h_dn11 = assign54260_e89089_d_n11;

        let (assign54270_e89096, assign54270_e89096_d_n3, assign54270_e89096_d_n4, assign54270_e89096_d_n5, assign54270_e89096_d_n6, assign54270_e89096_d_n7, assign54270_e89096_d_n8, assign54270_e89096_d_n9, assign54270_e89096_d_n10, assign54270_e89096_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard822 != 0.0)) {
        (locals.var_swi_h, locals.var_swi_h_dn3, locals.var_swi_h_dn4, locals.var_swi_h_dn5, locals.var_swi_h_dn6, locals.var_swi_h_dn7, locals.var_swi_h_dn8, locals.var_swi_h_dn9, locals.var_swi_h_dn10, locals.var_swi_h_dn11,)
    } else {
        (locals.var_t10, locals.var_t10_dn3, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11,)
    }
};
        locals.var_t10 = assign54270_e89096;
        locals.var_t10_dn3 = assign54270_e89096_d_n3;
        locals.var_t10_dn4 = assign54270_e89096_d_n4;
        locals.var_t10_dn5 = assign54270_e89096_d_n5;
        locals.var_t10_dn6 = assign54270_e89096_d_n6;
        locals.var_t10_dn7 = assign54270_e89096_d_n7;
        locals.var_t10_dn8 = assign54270_e89096_d_n8;
        locals.var_t10_dn9 = assign54270_e89096_d_n9;
        locals.var_t10_dn10 = assign54270_e89096_d_n10;
        locals.var_t10_dn11 = assign54270_e89096_d_n11;

        let assign54320_e89135: f64 = (locals.var_leff / 2.0);
        let assign54320_e89136: f64 = if p.p1015 >= assign54320_e89135 { 1.0 } else { 0.0 };
        locals.var_guard833 = assign54320_e89136;

        let (assign54330_e89146,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 == 0.0)) && (locals.var_guard833 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_lintnoi_i,)
    }
};
        locals.var_lintnoi_i = assign54330_e89146;

        let (assign54340_e89157,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 == 0.0)) && (locals.var_guard833 == 0.0)) {
        (p.p1015,)
    } else {
        (locals.var_lintnoi_i,)
    }
};
        locals.var_lintnoi_i = assign54340_e89157;

        let assign54350_e89168: f64 = if (((p.p1012 > 0.0) || (p.p1013 > 0.0)) || (p.p1014 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard834 = assign54350_e89168;

        let (assign54360_e89182,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 == 0.0)) && (locals.var_guard834 != 0.0)) {
        let assign54360_e89179: f64 = (2.0 * locals.var_lintnoi_i);
        let assign54360_e89180: f64 = (locals.var_leff - assign54360_e89179);
        (assign54360_e89180,)
    } else {
        (locals.var_leffnoi,)
    }
};
        locals.var_leffnoi = assign54360_e89182;

        let (assign54370_e89194,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 == 0.0)) && (locals.var_guard834 != 0.0)) {
        let assign54370_e89192: f64 = (locals.var_leffnoi * locals.var_leffnoi);
        (assign54370_e89192,)
    } else {
        (locals.var_leffnoisq,)
    }
};
        locals.var_leffnoisq = assign54370_e89194;

        let (assign54380_e89208, assign54380_e89208_d_n3, assign54380_e89208_d_n4, assign54380_e89208_d_n5, assign54380_e89208_d_n6, assign54380_e89208_d_n7, assign54380_e89208_d_n8, assign54380_e89208_d_n9, assign54380_e89208_d_n10, assign54380_e89208_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 == 0.0)) && (locals.var_guard834 != 0.0)) {
        let assign54380_e89204: f64 = (10000000000.0 * locals.var_cox);
        let assign54380_e89206: f64 = (assign54380_e89204 * locals.var_leffnoisq);
        (assign54380_e89206, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign54380_e89208;
        locals.var_t0_dn3 = assign54380_e89208_d_n3;
        locals.var_t0_dn4 = assign54380_e89208_d_n4;
        locals.var_t0_dn5 = assign54380_e89208_d_n5;
        locals.var_t0_dn6 = assign54380_e89208_d_n6;
        locals.var_t0_dn7 = assign54380_e89208_d_n7;
        locals.var_t0_dn8 = assign54380_e89208_d_n8;
        locals.var_t0_dn9 = assign54380_e89208_d_n9;
        locals.var_t0_dn10 = assign54380_e89208_d_n10;
        locals.var_t0_dn11 = assign54380_e89208_d_n11;

    }

    pub(super) fn stamp_transient_block_184(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign54390_e89232, assign54390_e89232_d_n3, assign54390_e89232_d_n4, assign54390_e89232_d_n5, assign54390_e89232_d_n6, assign54390_e89232_d_n7, assign54390_e89232_d_n8, assign54390_e89232_d_n9, assign54390_e89232_d_n10, assign54390_e89232_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 == 0.0)) && (locals.var_guard834 != 0.0)) {
        let assign54390_e89218: f64 = (2.0 * locals.var_nq);
        let assign54390_e89220: f64 = (assign54390_e89218 * locals.var_cox);
        let assign54390_e89222: f64 = (assign54390_e89220 * locals.var_vt);
        let assign54390_e89224: f64 = (assign54390_e89222 * locals.var_qs_1);
        let assign54390_e89226: f64 = (assign54390_e89224 * locals.var_mnud1);
        let assign54390_e89228: f64 = (assign54390_e89226 * locals.var_mnud);
        let assign54390_e89230: f64 = (assign54390_e89228 / 1.602176462e-19);
        (assign54390_e89230, ((((((((((2.0 * locals.var_nq_dn3) * locals.var_cox) * locals.var_vt) * locals.var_qs_1) + (assign54390_e89222 * locals.var_qs_1_dn3)) * locals.var_mnud1) + (assign54390_e89224 * locals.var_mnud1_dn3)) * locals.var_mnud) + (assign54390_e89226 * locals.var_mnud_dn3)) / 1.602176462e-19), (((((((((((2.0 * locals.var_nq_dn4) * locals.var_cox) * locals.var_vt) + (assign54390_e89220 * locals.var_vt_dn4)) * locals.var_qs_1) + (assign54390_e89222 * locals.var_qs_1_dn4)) * locals.var_mnud1) + (assign54390_e89224 * locals.var_mnud1_dn4)) * locals.var_mnud) + (assign54390_e89226 * locals.var_mnud_dn4)) / 1.602176462e-19), (((((((((((2.0 * locals.var_nq_dn5) * locals.var_cox) * locals.var_vt) + (assign54390_e89220 * locals.var_vt_dn5)) * locals.var_qs_1) + (assign54390_e89222 * locals.var_qs_1_dn5)) * locals.var_mnud1) + (assign54390_e89224 * locals.var_mnud1_dn5)) * locals.var_mnud) + (assign54390_e89226 * locals.var_mnud_dn5)) / 1.602176462e-19), ((((((((((2.0 * locals.var_nq_dn6) * locals.var_cox) * locals.var_vt) * locals.var_qs_1) + (assign54390_e89222 * locals.var_qs_1_dn6)) * locals.var_mnud1) + (assign54390_e89224 * locals.var_mnud1_dn6)) * locals.var_mnud) + (assign54390_e89226 * locals.var_mnud_dn6)) / 1.602176462e-19), ((((((((((2.0 * locals.var_nq_dn7) * locals.var_cox) * locals.var_vt) * locals.var_qs_1) + (assign54390_e89222 * locals.var_qs_1_dn7)) * locals.var_mnud1) + (assign54390_e89224 * locals.var_mnud1_dn7)) * locals.var_mnud) + (assign54390_e89226 * locals.var_mnud_dn7)) / 1.602176462e-19), ((((((((((2.0 * locals.var_nq_dn8) * locals.var_cox) * locals.var_vt) * locals.var_qs_1) + (assign54390_e89222 * locals.var_qs_1_dn8)) * locals.var_mnud1) + (assign54390_e89224 * locals.var_mnud1_dn8)) * locals.var_mnud) + (assign54390_e89226 * locals.var_mnud_dn8)) / 1.602176462e-19), ((((((((((2.0 * locals.var_nq_dn9) * locals.var_cox) * locals.var_vt) * locals.var_qs_1) + (assign54390_e89222 * locals.var_qs_1_dn9)) * locals.var_mnud1) + (assign54390_e89224 * locals.var_mnud1_dn9)) * locals.var_mnud) + (assign54390_e89226 * locals.var_mnud_dn9)) / 1.602176462e-19), ((((((((((2.0 * locals.var_nq_dn10) * locals.var_cox) * locals.var_vt) * locals.var_qs_1) + (assign54390_e89222 * locals.var_qs_1_dn10)) * locals.var_mnud1) + (assign54390_e89224 * locals.var_mnud1_dn10)) * locals.var_mnud) + (assign54390_e89226 * locals.var_mnud_dn10)) / 1.602176462e-19), ((((((((((2.0 * locals.var_nq_dn11) * locals.var_cox) * locals.var_vt) * locals.var_qs_1) + (assign54390_e89222 * locals.var_qs_1_dn11)) * locals.var_mnud1) + (assign54390_e89224 * locals.var_mnud1_dn11)) * locals.var_mnud) + (assign54390_e89226 * locals.var_mnud_dn11)) / 1.602176462e-19),)
    } else {
        (locals.var_n0, locals.var_n0_dn3, locals.var_n0_dn4, locals.var_n0_dn5, locals.var_n0_dn6, locals.var_n0_dn7, locals.var_n0_dn8, locals.var_n0_dn9, locals.var_n0_dn10, locals.var_n0_dn11,)
    }
};
        locals.var_n0 = assign54390_e89232;
        locals.var_n0_dn3 = assign54390_e89232_d_n3;
        locals.var_n0_dn4 = assign54390_e89232_d_n4;
        locals.var_n0_dn5 = assign54390_e89232_d_n5;
        locals.var_n0_dn6 = assign54390_e89232_d_n6;
        locals.var_n0_dn7 = assign54390_e89232_d_n7;
        locals.var_n0_dn8 = assign54390_e89232_d_n8;
        locals.var_n0_dn9 = assign54390_e89232_d_n9;
        locals.var_n0_dn10 = assign54390_e89232_d_n10;
        locals.var_n0_dn11 = assign54390_e89232_d_n11;

        let (assign54400_e89253, assign54400_e89253_d_n3, assign54400_e89253_d_n4, assign54400_e89253_d_n5, assign54400_e89253_d_n6, assign54400_e89253_d_n7, assign54400_e89253_d_n8, assign54400_e89253_d_n9, assign54400_e89253_d_n10, assign54400_e89253_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 == 0.0)) && (locals.var_guard834 != 0.0)) {
        let assign54400_e89243: f64 = (locals.var_n0 + locals.var_nstar);
        let assign54400_e89246: f64 = (locals.var_nl + locals.var_nstar);
        let assign54400_e89247: f64 = (assign54400_e89243 / assign54400_e89246);
        let assign54400_e89249: f64 = (assign54400_e89247).max(1e-38);
        let assign54400_e89250: f64 = (assign54400_e89249).ln();
        let assign54400_e89251: f64 = (p.p1012 * assign54400_e89250);
        (assign54400_e89251, (p.p1012 * (if assign54400_e89247 >= 1e-38 { ((((locals.var_n0_dn3 + locals.var_nstar_dn3) * assign54400_e89246) - (assign54400_e89243 * (locals.var_nl_dn3 + locals.var_nstar_dn3))) / (assign54400_e89246 * assign54400_e89246)) } else { 0.0 } / assign54400_e89249)), (p.p1012 * (if assign54400_e89247 >= 1e-38 { ((((locals.var_n0_dn4 + locals.var_nstar_dn4) * assign54400_e89246) - (assign54400_e89243 * (locals.var_nl_dn4 + locals.var_nstar_dn4))) / (assign54400_e89246 * assign54400_e89246)) } else { 0.0 } / assign54400_e89249)), (p.p1012 * (if assign54400_e89247 >= 1e-38 { ((((locals.var_n0_dn5 + locals.var_nstar_dn5) * assign54400_e89246) - (assign54400_e89243 * (locals.var_nl_dn5 + locals.var_nstar_dn5))) / (assign54400_e89246 * assign54400_e89246)) } else { 0.0 } / assign54400_e89249)), (p.p1012 * (if assign54400_e89247 >= 1e-38 { ((((locals.var_n0_dn6 + locals.var_nstar_dn6) * assign54400_e89246) - (assign54400_e89243 * (locals.var_nl_dn6 + locals.var_nstar_dn6))) / (assign54400_e89246 * assign54400_e89246)) } else { 0.0 } / assign54400_e89249)), (p.p1012 * (if assign54400_e89247 >= 1e-38 { ((((locals.var_n0_dn7 + locals.var_nstar_dn7) * assign54400_e89246) - (assign54400_e89243 * (locals.var_nl_dn7 + locals.var_nstar_dn7))) / (assign54400_e89246 * assign54400_e89246)) } else { 0.0 } / assign54400_e89249)), (p.p1012 * (if assign54400_e89247 >= 1e-38 { ((((locals.var_n0_dn8 + locals.var_nstar_dn8) * assign54400_e89246) - (assign54400_e89243 * (locals.var_nl_dn8 + locals.var_nstar_dn8))) / (assign54400_e89246 * assign54400_e89246)) } else { 0.0 } / assign54400_e89249)), (p.p1012 * (if assign54400_e89247 >= 1e-38 { ((((locals.var_n0_dn9 + locals.var_nstar_dn9) * assign54400_e89246) - (assign54400_e89243 * (locals.var_nl_dn9 + locals.var_nstar_dn9))) / (assign54400_e89246 * assign54400_e89246)) } else { 0.0 } / assign54400_e89249)), (p.p1012 * (if assign54400_e89247 >= 1e-38 { ((((locals.var_n0_dn10 + locals.var_nstar_dn10) * assign54400_e89246) - (assign54400_e89243 * (locals.var_nl_dn10 + locals.var_nstar_dn10))) / (assign54400_e89246 * assign54400_e89246)) } else { 0.0 } / assign54400_e89249)), (p.p1012 * (if assign54400_e89247 >= 1e-38 { ((((locals.var_n0_dn11 + locals.var_nstar_dn11) * assign54400_e89246) - (assign54400_e89243 * (locals.var_nl_dn11 + locals.var_nstar_dn11))) / (assign54400_e89246 * assign54400_e89246)) } else { 0.0 } / assign54400_e89249)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign54400_e89253;
        locals.var_t1_dn3 = assign54400_e89253_d_n3;
        locals.var_t1_dn4 = assign54400_e89253_d_n4;
        locals.var_t1_dn5 = assign54400_e89253_d_n5;
        locals.var_t1_dn6 = assign54400_e89253_d_n6;
        locals.var_t1_dn7 = assign54400_e89253_d_n7;
        locals.var_t1_dn8 = assign54400_e89253_d_n8;
        locals.var_t1_dn9 = assign54400_e89253_d_n9;
        locals.var_t1_dn10 = assign54400_e89253_d_n10;
        locals.var_t1_dn11 = assign54400_e89253_d_n11;

        let (assign54410_e89267, assign54410_e89267_d_n3, assign54410_e89267_d_n4, assign54410_e89267_d_n5, assign54410_e89267_d_n6, assign54410_e89267_d_n7, assign54410_e89267_d_n8, assign54410_e89267_d_n9, assign54410_e89267_d_n10, assign54410_e89267_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 == 0.0)) && (locals.var_guard834 != 0.0)) {
        let assign54410_e89264: f64 = (locals.var_n0 - locals.var_nl);
        let assign54410_e89265: f64 = (p.p1013 * assign54410_e89264);
        (assign54410_e89265, (p.p1013 * (locals.var_n0_dn3 - locals.var_nl_dn3)), (p.p1013 * (locals.var_n0_dn4 - locals.var_nl_dn4)), (p.p1013 * (locals.var_n0_dn5 - locals.var_nl_dn5)), (p.p1013 * (locals.var_n0_dn6 - locals.var_nl_dn6)), (p.p1013 * (locals.var_n0_dn7 - locals.var_nl_dn7)), (p.p1013 * (locals.var_n0_dn8 - locals.var_nl_dn8)), (p.p1013 * (locals.var_n0_dn9 - locals.var_nl_dn9)), (p.p1013 * (locals.var_n0_dn10 - locals.var_nl_dn10)), (p.p1013 * (locals.var_n0_dn11 - locals.var_nl_dn11)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign54410_e89267;
        locals.var_t2_dn3 = assign54410_e89267_d_n3;
        locals.var_t2_dn4 = assign54410_e89267_d_n4;
        locals.var_t2_dn5 = assign54410_e89267_d_n5;
        locals.var_t2_dn6 = assign54410_e89267_d_n6;
        locals.var_t2_dn7 = assign54410_e89267_d_n7;
        locals.var_t2_dn8 = assign54410_e89267_d_n8;
        locals.var_t2_dn9 = assign54410_e89267_d_n9;
        locals.var_t2_dn10 = assign54410_e89267_d_n10;
        locals.var_t2_dn11 = assign54410_e89267_d_n11;

        let (assign54420_e89287, assign54420_e89287_d_n3, assign54420_e89287_d_n4, assign54420_e89287_d_n5, assign54420_e89287_d_n6, assign54420_e89287_d_n7, assign54420_e89287_d_n8, assign54420_e89287_d_n9, assign54420_e89287_d_n10, assign54420_e89287_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 == 0.0)) && (locals.var_guard834 != 0.0)) {
        let assign54420_e89277: f64 = (0.5 * p.p1014);
        let assign54420_e89280: f64 = (locals.var_n0 * locals.var_n0);
        let assign54420_e89283: f64 = (locals.var_nl * locals.var_nl);
        let assign54420_e89284: f64 = (assign54420_e89280 - assign54420_e89283);
        let assign54420_e89285: f64 = (assign54420_e89277 * assign54420_e89284);
        (assign54420_e89285, (assign54420_e89277 * (((locals.var_n0_dn3 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn3)) - ((locals.var_nl_dn3 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn3)))), (assign54420_e89277 * (((locals.var_n0_dn4 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn4)) - ((locals.var_nl_dn4 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn4)))), (assign54420_e89277 * (((locals.var_n0_dn5 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn5)) - ((locals.var_nl_dn5 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn5)))), (assign54420_e89277 * (((locals.var_n0_dn6 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn6)) - ((locals.var_nl_dn6 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn6)))), (assign54420_e89277 * (((locals.var_n0_dn7 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn7)) - ((locals.var_nl_dn7 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn7)))), (assign54420_e89277 * (((locals.var_n0_dn8 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn8)) - ((locals.var_nl_dn8 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn8)))), (assign54420_e89277 * (((locals.var_n0_dn9 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn9)) - ((locals.var_nl_dn9 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn9)))), (assign54420_e89277 * (((locals.var_n0_dn10 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn10)) - ((locals.var_nl_dn10 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn10)))), (assign54420_e89277 * (((locals.var_n0_dn11 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn11)) - ((locals.var_nl_dn11 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn11)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign54420_e89287;
        locals.var_t3_dn3 = assign54420_e89287_d_n3;
        locals.var_t3_dn4 = assign54420_e89287_d_n4;
        locals.var_t3_dn5 = assign54420_e89287_d_n5;
        locals.var_t3_dn6 = assign54420_e89287_d_n6;
        locals.var_t3_dn7 = assign54420_e89287_d_n7;
        locals.var_t3_dn8 = assign54420_e89287_d_n8;
        locals.var_t3_dn9 = assign54420_e89287_d_n9;
        locals.var_t3_dn10 = assign54420_e89287_d_n10;
        locals.var_t3_dn11 = assign54420_e89287_d_n11;

        let (assign54430_e89303, assign54430_e89303_d_n3, assign54430_e89303_d_n4, assign54430_e89303_d_n5, assign54430_e89303_d_n6, assign54430_e89303_d_n7, assign54430_e89303_d_n8, assign54430_e89303_d_n9, assign54430_e89303_d_n10, assign54430_e89303_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 == 0.0)) && (locals.var_guard834 != 0.0)) {
        let assign54430_e89297: f64 = (10000000000.0 * locals.var_leffnoisq);
        let assign54430_e89299: f64 = (assign54430_e89297 * locals.var_weff);
        let assign54430_e89301: f64 = (assign54430_e89299 * p.p2);
        (assign54430_e89301, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign54430_e89303;
        locals.var_t4_dn3 = assign54430_e89303_d_n3;
        locals.var_t4_dn4 = assign54430_e89303_d_n4;
        locals.var_t4_dn5 = assign54430_e89303_d_n5;
        locals.var_t4_dn6 = assign54430_e89303_d_n6;
        locals.var_t4_dn7 = assign54430_e89303_d_n7;
        locals.var_t4_dn8 = assign54430_e89303_d_n8;
        locals.var_t4_dn9 = assign54430_e89303_d_n9;
        locals.var_t4_dn10 = assign54430_e89303_d_n10;
        locals.var_t4_dn11 = assign54430_e89303_d_n11;

        let (assign54440_e89331, assign54440_e89331_d_n3, assign54440_e89331_d_n4, assign54440_e89331_d_n5, assign54440_e89331_d_n6, assign54440_e89331_d_n7, assign54440_e89331_d_n8, assign54440_e89331_d_n9, assign54440_e89331_d_n10, assign54440_e89331_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 == 0.0)) && (locals.var_guard834 != 0.0)) {
        let assign54440_e89313: f64 = (locals.var_t0a / locals.var_t0);
        let assign54440_e89316: f64 = (locals.var_t1 + locals.var_t2);
        let assign54440_e89318: f64 = (assign54440_e89316 + locals.var_t3);
        let assign54440_e89319: f64 = (assign54440_e89313 * assign54440_e89318);
        let assign54440_e89322: f64 = (locals.var_t0b / locals.var_t4);
        let assign54440_e89324: f64 = (assign54440_e89322 * locals.var_delclm);
        let assign54440_e89326: f64 = (assign54440_e89324 * locals.var_t0c);
        let assign54440_e89328: f64 = (assign54440_e89326 / locals.var_t0d);
        let assign54440_e89329: f64 = (assign54440_e89319 + assign54440_e89328);
        (assign54440_e89329, ((((((locals.var_t0a_dn3 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn3)) / (locals.var_t0 * locals.var_t0)) * assign54440_e89318) + (assign54440_e89313 * ((locals.var_t1_dn3 + locals.var_t2_dn3) + locals.var_t3_dn3))) + ((((((((((locals.var_t0b_dn3 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign54440_e89322 * locals.var_delclm_dn3)) * locals.var_t0c) + (assign54440_e89324 * locals.var_t0c_dn3)) * locals.var_t0d) - (assign54440_e89326 * locals.var_t0d_dn3)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn4 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)) * assign54440_e89318) + (assign54440_e89313 * ((locals.var_t1_dn4 + locals.var_t2_dn4) + locals.var_t3_dn4))) + ((((((((((locals.var_t0b_dn4 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign54440_e89322 * locals.var_delclm_dn4)) * locals.var_t0c) + (assign54440_e89324 * locals.var_t0c_dn4)) * locals.var_t0d) - (assign54440_e89326 * locals.var_t0d_dn4)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn5 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)) * assign54440_e89318) + (assign54440_e89313 * ((locals.var_t1_dn5 + locals.var_t2_dn5) + locals.var_t3_dn5))) + ((((((((((locals.var_t0b_dn5 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign54440_e89322 * locals.var_delclm_dn5)) * locals.var_t0c) + (assign54440_e89324 * locals.var_t0c_dn5)) * locals.var_t0d) - (assign54440_e89326 * locals.var_t0d_dn5)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn6 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)) * assign54440_e89318) + (assign54440_e89313 * ((locals.var_t1_dn6 + locals.var_t2_dn6) + locals.var_t3_dn6))) + ((((((((((locals.var_t0b_dn6 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign54440_e89322 * locals.var_delclm_dn6)) * locals.var_t0c) + (assign54440_e89324 * locals.var_t0c_dn6)) * locals.var_t0d) - (assign54440_e89326 * locals.var_t0d_dn6)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn7 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)) * assign54440_e89318) + (assign54440_e89313 * ((locals.var_t1_dn7 + locals.var_t2_dn7) + locals.var_t3_dn7))) + ((((((((((locals.var_t0b_dn7 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign54440_e89322 * locals.var_delclm_dn7)) * locals.var_t0c) + (assign54440_e89324 * locals.var_t0c_dn7)) * locals.var_t0d) - (assign54440_e89326 * locals.var_t0d_dn7)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn8 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)) * assign54440_e89318) + (assign54440_e89313 * ((locals.var_t1_dn8 + locals.var_t2_dn8) + locals.var_t3_dn8))) + ((((((((((locals.var_t0b_dn8 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign54440_e89322 * locals.var_delclm_dn8)) * locals.var_t0c) + (assign54440_e89324 * locals.var_t0c_dn8)) * locals.var_t0d) - (assign54440_e89326 * locals.var_t0d_dn8)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn9 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)) * assign54440_e89318) + (assign54440_e89313 * ((locals.var_t1_dn9 + locals.var_t2_dn9) + locals.var_t3_dn9))) + ((((((((((locals.var_t0b_dn9 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign54440_e89322 * locals.var_delclm_dn9)) * locals.var_t0c) + (assign54440_e89324 * locals.var_t0c_dn9)) * locals.var_t0d) - (assign54440_e89326 * locals.var_t0d_dn9)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn10 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)) * assign54440_e89318) + (assign54440_e89313 * ((locals.var_t1_dn10 + locals.var_t2_dn10) + locals.var_t3_dn10))) + ((((((((((locals.var_t0b_dn10 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign54440_e89322 * locals.var_delclm_dn10)) * locals.var_t0c) + (assign54440_e89324 * locals.var_t0c_dn10)) * locals.var_t0d) - (assign54440_e89326 * locals.var_t0d_dn10)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn11 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)) * assign54440_e89318) + (assign54440_e89313 * ((locals.var_t1_dn11 + locals.var_t2_dn11) + locals.var_t3_dn11))) + ((((((((((locals.var_t0b_dn11 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign54440_e89322 * locals.var_delclm_dn11)) * locals.var_t0c) + (assign54440_e89324 * locals.var_t0c_dn11)) * locals.var_t0d) - (assign54440_e89326 * locals.var_t0d_dn11)) / (locals.var_t0d * locals.var_t0d))),)
    } else {
        (locals.var_ssi, locals.var_ssi_dn3, locals.var_ssi_dn4, locals.var_ssi_dn5, locals.var_ssi_dn6, locals.var_ssi_dn7, locals.var_ssi_dn8, locals.var_ssi_dn9, locals.var_ssi_dn10, locals.var_ssi_dn11,)
    }
};
        locals.var_ssi = assign54440_e89331;
        locals.var_ssi_dn3 = assign54440_e89331_d_n3;
        locals.var_ssi_dn4 = assign54440_e89331_d_n4;
        locals.var_ssi_dn5 = assign54440_e89331_d_n5;
        locals.var_ssi_dn6 = assign54440_e89331_d_n6;
        locals.var_ssi_dn7 = assign54440_e89331_d_n7;
        locals.var_ssi_dn8 = assign54440_e89331_d_n8;
        locals.var_ssi_dn9 = assign54440_e89331_d_n9;
        locals.var_ssi_dn10 = assign54440_e89331_d_n10;
        locals.var_ssi_dn11 = assign54440_e89331_d_n11;

        let (assign54450_e89351, assign54450_e89351_d_n3, assign54450_e89351_d_n4, assign54450_e89351_d_n5, assign54450_e89351_d_n6, assign54450_e89351_d_n7, assign54450_e89351_d_n8, assign54450_e89351_d_n9, assign54450_e89351_d_n10, assign54450_e89351_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 == 0.0)) && (locals.var_guard834 != 0.0)) {
        let assign54450_e89341: f64 = (locals.var_weff * p.p2);
        let assign54450_e89343: f64 = (assign54450_e89341 * locals.var_leffnoi);
        let assign54450_e89345: f64 = (assign54450_e89343 * 10000000000.0);
        let assign54450_e89347: f64 = (assign54450_e89345 * locals.var_nstar);
        let assign54450_e89349: f64 = (assign54450_e89347 * locals.var_nstar);
        (assign54450_e89349, (((assign54450_e89345 * locals.var_nstar_dn3) * locals.var_nstar) + (assign54450_e89347 * locals.var_nstar_dn3)), (((assign54450_e89345 * locals.var_nstar_dn4) * locals.var_nstar) + (assign54450_e89347 * locals.var_nstar_dn4)), (((assign54450_e89345 * locals.var_nstar_dn5) * locals.var_nstar) + (assign54450_e89347 * locals.var_nstar_dn5)), (((assign54450_e89345 * locals.var_nstar_dn6) * locals.var_nstar) + (assign54450_e89347 * locals.var_nstar_dn6)), (((assign54450_e89345 * locals.var_nstar_dn7) * locals.var_nstar) + (assign54450_e89347 * locals.var_nstar_dn7)), (((assign54450_e89345 * locals.var_nstar_dn8) * locals.var_nstar) + (assign54450_e89347 * locals.var_nstar_dn8)), (((assign54450_e89345 * locals.var_nstar_dn9) * locals.var_nstar) + (assign54450_e89347 * locals.var_nstar_dn9)), (((assign54450_e89345 * locals.var_nstar_dn10) * locals.var_nstar) + (assign54450_e89347 * locals.var_nstar_dn10)), (((assign54450_e89345 * locals.var_nstar_dn11) * locals.var_nstar) + (assign54450_e89347 * locals.var_nstar_dn11)),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign54450_e89351;
        locals.var_t5_dn3 = assign54450_e89351_d_n3;
        locals.var_t5_dn4 = assign54450_e89351_d_n4;
        locals.var_t5_dn5 = assign54450_e89351_d_n5;
        locals.var_t5_dn6 = assign54450_e89351_d_n6;
        locals.var_t5_dn7 = assign54450_e89351_d_n7;
        locals.var_t5_dn8 = assign54450_e89351_d_n8;
        locals.var_t5_dn9 = assign54450_e89351_d_n9;
        locals.var_t5_dn10 = assign54450_e89351_d_n10;
        locals.var_t5_dn11 = assign54450_e89351_d_n11;

        let (assign54460_e89367, assign54460_e89367_d_n3, assign54460_e89367_d_n4, assign54460_e89367_d_n5, assign54460_e89367_d_n6, assign54460_e89367_d_n7, assign54460_e89367_d_n8, assign54460_e89367_d_n9, assign54460_e89367_d_n10, assign54460_e89367_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 == 0.0)) && (locals.var_guard834 != 0.0)) {
        let assign54460_e89361: f64 = (locals.var_t0e / locals.var_t5);
        let assign54460_e89363: f64 = (assign54460_e89361 * locals.var_ids);
        let assign54460_e89365: f64 = (assign54460_e89363 * locals.var_ids);
        (assign54460_e89365, (((((-((locals.var_t0e * locals.var_t5_dn3) / (locals.var_t5 * locals.var_t5))) * locals.var_ids) + (assign54460_e89361 * locals.var_ids_dn3)) * locals.var_ids) + (assign54460_e89363 * locals.var_ids_dn3)), (((((((locals.var_t0e_dn4 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids) + (assign54460_e89361 * locals.var_ids_dn4)) * locals.var_ids) + (assign54460_e89363 * locals.var_ids_dn4)), (((((((locals.var_t0e_dn5 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids) + (assign54460_e89361 * locals.var_ids_dn5)) * locals.var_ids) + (assign54460_e89363 * locals.var_ids_dn5)), (((((-((locals.var_t0e * locals.var_t5_dn6) / (locals.var_t5 * locals.var_t5))) * locals.var_ids) + (assign54460_e89361 * locals.var_ids_dn6)) * locals.var_ids) + (assign54460_e89363 * locals.var_ids_dn6)), (((((-((locals.var_t0e * locals.var_t5_dn7) / (locals.var_t5 * locals.var_t5))) * locals.var_ids) + (assign54460_e89361 * locals.var_ids_dn7)) * locals.var_ids) + (assign54460_e89363 * locals.var_ids_dn7)), (((((-((locals.var_t0e * locals.var_t5_dn8) / (locals.var_t5 * locals.var_t5))) * locals.var_ids) + (assign54460_e89361 * locals.var_ids_dn8)) * locals.var_ids) + (assign54460_e89363 * locals.var_ids_dn8)), (((((-((locals.var_t0e * locals.var_t5_dn9) / (locals.var_t5 * locals.var_t5))) * locals.var_ids) + (assign54460_e89361 * locals.var_ids_dn9)) * locals.var_ids) + (assign54460_e89363 * locals.var_ids_dn9)), (((((-((locals.var_t0e * locals.var_t5_dn10) / (locals.var_t5 * locals.var_t5))) * locals.var_ids) + (assign54460_e89361 * locals.var_ids_dn10)) * locals.var_ids) + (assign54460_e89363 * locals.var_ids_dn10)), (((((-((locals.var_t0e * locals.var_t5_dn11) / (locals.var_t5 * locals.var_t5))) * locals.var_ids) + (assign54460_e89361 * locals.var_ids_dn11)) * locals.var_ids) + (assign54460_e89363 * locals.var_ids_dn11)),)
    } else {
        (locals.var_swi, locals.var_swi_dn3, locals.var_swi_dn4, locals.var_swi_dn5, locals.var_swi_dn6, locals.var_swi_dn7, locals.var_swi_dn8, locals.var_swi_dn9, locals.var_swi_dn10, locals.var_swi_dn11,)
    }
};
        locals.var_swi = assign54460_e89367;
        locals.var_swi_dn3 = assign54460_e89367_d_n3;
        locals.var_swi_dn4 = assign54460_e89367_d_n4;
        locals.var_swi_dn5 = assign54460_e89367_d_n5;
        locals.var_swi_dn6 = assign54460_e89367_d_n6;
        locals.var_swi_dn7 = assign54460_e89367_d_n7;
        locals.var_swi_dn8 = assign54460_e89367_d_n8;
        locals.var_swi_dn9 = assign54460_e89367_d_n9;
        locals.var_swi_dn10 = assign54460_e89367_d_n10;
        locals.var_swi_dn11 = assign54460_e89367_d_n11;

        let (assign54470_e89379, assign54470_e89379_d_n3, assign54470_e89379_d_n4, assign54470_e89379_d_n5, assign54470_e89379_d_n6, assign54470_e89379_d_n7, assign54470_e89379_d_n8, assign54470_e89379_d_n9, assign54470_e89379_d_n10, assign54470_e89379_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard822 == 0.0)) && (locals.var_guard834 != 0.0)) {
        let assign54470_e89377: f64 = (locals.var_swi + locals.var_ssi);
        (assign54470_e89377, (locals.var_swi_dn3 + locals.var_ssi_dn3), (locals.var_swi_dn4 + locals.var_ssi_dn4), (locals.var_swi_dn5 + locals.var_ssi_dn5), (locals.var_swi_dn6 + locals.var_ssi_dn6), (locals.var_swi_dn7 + locals.var_ssi_dn7), (locals.var_swi_dn8 + locals.var_ssi_dn8), (locals.var_swi_dn9 + locals.var_ssi_dn9), (locals.var_swi_dn10 + locals.var_ssi_dn10), (locals.var_swi_dn11 + locals.var_ssi_dn11),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign54470_e89379;
        locals.var_t6_dn3 = assign54470_e89379_d_n3;
        locals.var_t6_dn4 = assign54470_e89379_d_n4;
        locals.var_t6_dn5 = assign54470_e89379_d_n5;
        locals.var_t6_dn6 = assign54470_e89379_d_n6;
        locals.var_t6_dn7 = assign54470_e89379_d_n7;
        locals.var_t6_dn8 = assign54470_e89379_d_n8;
        locals.var_t6_dn9 = assign54470_e89379_d_n9;
        locals.var_t6_dn10 = assign54470_e89379_d_n10;
        locals.var_t6_dn11 = assign54470_e89379_d_n11;

        let (assign54520_e89441, assign54520_e89441_d_n3, assign54520_e89441_d_n4, assign54520_e89441_d_n5, assign54520_e89441_d_n6, assign54520_e89441_d_n7, assign54520_e89441_d_n8, assign54520_e89441_d_n9, assign54520_e89441_d_n10, assign54520_e89441_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign54520_e89437: f64 = (locals.var_qia / locals.var_esatnoi);
        let assign54520_e89439: f64 = (assign54520_e89437 / locals.var_leff);
        (assign54520_e89439, ((((locals.var_qia_dn3 * locals.var_esatnoi) - (locals.var_qia * locals.var_esatnoi_dn3)) / (locals.var_esatnoi * locals.var_esatnoi)) / locals.var_leff), ((((locals.var_qia_dn4 * locals.var_esatnoi) - (locals.var_qia * locals.var_esatnoi_dn4)) / (locals.var_esatnoi * locals.var_esatnoi)) / locals.var_leff), ((((locals.var_qia_dn5 * locals.var_esatnoi) - (locals.var_qia * locals.var_esatnoi_dn5)) / (locals.var_esatnoi * locals.var_esatnoi)) / locals.var_leff), ((((locals.var_qia_dn6 * locals.var_esatnoi) - (locals.var_qia * locals.var_esatnoi_dn6)) / (locals.var_esatnoi * locals.var_esatnoi)) / locals.var_leff), ((((locals.var_qia_dn7 * locals.var_esatnoi) - (locals.var_qia * locals.var_esatnoi_dn7)) / (locals.var_esatnoi * locals.var_esatnoi)) / locals.var_leff), ((((locals.var_qia_dn8 * locals.var_esatnoi) - (locals.var_qia * locals.var_esatnoi_dn8)) / (locals.var_esatnoi * locals.var_esatnoi)) / locals.var_leff), ((((locals.var_qia_dn9 * locals.var_esatnoi) - (locals.var_qia * locals.var_esatnoi_dn9)) / (locals.var_esatnoi * locals.var_esatnoi)) / locals.var_leff), ((((locals.var_qia_dn10 * locals.var_esatnoi) - (locals.var_qia * locals.var_esatnoi_dn10)) / (locals.var_esatnoi * locals.var_esatnoi)) / locals.var_leff), ((((locals.var_qia_dn11 * locals.var_esatnoi) - (locals.var_qia * locals.var_esatnoi_dn11)) / (locals.var_esatnoi * locals.var_esatnoi)) / locals.var_leff),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign54520_e89441;
        locals.var_t0_dn3 = assign54520_e89441_d_n3;
        locals.var_t0_dn4 = assign54520_e89441_d_n4;
        locals.var_t0_dn5 = assign54520_e89441_d_n5;
        locals.var_t0_dn6 = assign54520_e89441_d_n6;
        locals.var_t0_dn7 = assign54520_e89441_d_n7;
        locals.var_t0_dn8 = assign54520_e89441_d_n8;
        locals.var_t0_dn9 = assign54520_e89441_d_n9;
        locals.var_t0_dn10 = assign54520_e89441_d_n10;
        locals.var_t0_dn11 = assign54520_e89441_d_n11;

        let (assign54530_e89448, assign54530_e89448_d_n3, assign54530_e89448_d_n4, assign54530_e89448_d_n5, assign54530_e89448_d_n6, assign54530_e89448_d_n7, assign54530_e89448_d_n8, assign54530_e89448_d_n9, assign54530_e89448_d_n10, assign54530_e89448_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign54530_e89446: f64 = (locals.var_t0 * locals.var_t0);
        (assign54530_e89446, ((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)), ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)), ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)), ((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)), ((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)), ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)), ((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)), ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)), ((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign54530_e89448;
        locals.var_t1_dn3 = assign54530_e89448_d_n3;
        locals.var_t1_dn4 = assign54530_e89448_d_n4;
        locals.var_t1_dn5 = assign54530_e89448_d_n5;
        locals.var_t1_dn6 = assign54530_e89448_d_n6;
        locals.var_t1_dn7 = assign54530_e89448_d_n7;
        locals.var_t1_dn8 = assign54530_e89448_d_n8;
        locals.var_t1_dn9 = assign54530_e89448_d_n9;
        locals.var_t1_dn10 = assign54530_e89448_d_n10;
        locals.var_t1_dn11 = assign54530_e89448_d_n11;

        let (assign54540_e89461, assign54540_e89461_d_n3, assign54540_e89461_d_n4, assign54540_e89461_d_n5, assign54540_e89461_d_n6, assign54540_e89461_d_n7, assign54540_e89461_d_n8, assign54540_e89461_d_n9, assign54540_e89461_d_n10, assign54540_e89461_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign54540_e89455: f64 = (p.p1022 * locals.var_leff);
        let assign54540_e89457: f64 = (assign54540_e89455 * locals.var_t1);
        let assign54540_e89458: f64 = (1.0 + assign54540_e89457);
        let assign54540_e89459: f64 = (p.p1019 * assign54540_e89458);
        (assign54540_e89459, (p.p1019 * (assign54540_e89455 * locals.var_t1_dn3)), (p.p1019 * (assign54540_e89455 * locals.var_t1_dn4)), (p.p1019 * (assign54540_e89455 * locals.var_t1_dn5)), (p.p1019 * (assign54540_e89455 * locals.var_t1_dn6)), (p.p1019 * (assign54540_e89455 * locals.var_t1_dn7)), (p.p1019 * (assign54540_e89455 * locals.var_t1_dn8)), (p.p1019 * (assign54540_e89455 * locals.var_t1_dn9)), (p.p1019 * (assign54540_e89455 * locals.var_t1_dn10)), (p.p1019 * (assign54540_e89455 * locals.var_t1_dn11)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign54540_e89461;
        locals.var_t3_dn3 = assign54540_e89461_d_n3;
        locals.var_t3_dn4 = assign54540_e89461_d_n4;
        locals.var_t3_dn5 = assign54540_e89461_d_n5;
        locals.var_t3_dn6 = assign54540_e89461_d_n6;
        locals.var_t3_dn7 = assign54540_e89461_d_n7;
        locals.var_t3_dn8 = assign54540_e89461_d_n8;
        locals.var_t3_dn9 = assign54540_e89461_d_n9;
        locals.var_t3_dn10 = assign54540_e89461_d_n10;
        locals.var_t3_dn11 = assign54540_e89461_d_n11;

        let (assign54550_e89474, assign54550_e89474_d_n3, assign54550_e89474_d_n4, assign54550_e89474_d_n5, assign54550_e89474_d_n6, assign54550_e89474_d_n7, assign54550_e89474_d_n8, assign54550_e89474_d_n9, assign54550_e89474_d_n10, assign54550_e89474_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign54550_e89468: f64 = (p.p1023 * locals.var_leff);
        let assign54550_e89470: f64 = (assign54550_e89468 * locals.var_t1);
        let assign54550_e89471: f64 = (1.0 + assign54550_e89470);
        let assign54550_e89472: f64 = (p.p1020 * assign54550_e89471);
        (assign54550_e89472, (p.p1020 * (assign54550_e89468 * locals.var_t1_dn3)), (p.p1020 * (assign54550_e89468 * locals.var_t1_dn4)), (p.p1020 * (assign54550_e89468 * locals.var_t1_dn5)), (p.p1020 * (assign54550_e89468 * locals.var_t1_dn6)), (p.p1020 * (assign54550_e89468 * locals.var_t1_dn7)), (p.p1020 * (assign54550_e89468 * locals.var_t1_dn8)), (p.p1020 * (assign54550_e89468 * locals.var_t1_dn9)), (p.p1020 * (assign54550_e89468 * locals.var_t1_dn10)), (p.p1020 * (assign54550_e89468 * locals.var_t1_dn11)),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign54550_e89474;
        locals.var_t4_dn3 = assign54550_e89474_d_n3;
        locals.var_t4_dn4 = assign54550_e89474_d_n4;
        locals.var_t4_dn5 = assign54550_e89474_d_n5;
        locals.var_t4_dn6 = assign54550_e89474_d_n6;
        locals.var_t4_dn7 = assign54550_e89474_d_n7;
        locals.var_t4_dn8 = assign54550_e89474_d_n8;
        locals.var_t4_dn9 = assign54550_e89474_d_n9;
        locals.var_t4_dn10 = assign54550_e89474_d_n10;
        locals.var_t4_dn11 = assign54550_e89474_d_n11;

        let (assign54560_e89487, assign54560_e89487_d_n3, assign54560_e89487_d_n4, assign54560_e89487_d_n5, assign54560_e89487_d_n6, assign54560_e89487_d_n7, assign54560_e89487_d_n8, assign54560_e89487_d_n9, assign54560_e89487_d_n10, assign54560_e89487_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign54560_e89481: f64 = (p.p1298 * locals.var_leff);
        let assign54560_e89483: f64 = (assign54560_e89481 * locals.var_t1);
        let assign54560_e89484: f64 = (1.0 + assign54560_e89483);
        let assign54560_e89485: f64 = (p.p1297 * assign54560_e89484);
        (assign54560_e89485, (p.p1297 * (assign54560_e89481 * locals.var_t1_dn3)), (p.p1297 * (assign54560_e89481 * locals.var_t1_dn4)), (p.p1297 * (assign54560_e89481 * locals.var_t1_dn5)), (p.p1297 * (assign54560_e89481 * locals.var_t1_dn6)), (p.p1297 * (assign54560_e89481 * locals.var_t1_dn7)), (p.p1297 * (assign54560_e89481 * locals.var_t1_dn8)), (p.p1297 * (assign54560_e89481 * locals.var_t1_dn9)), (p.p1297 * (assign54560_e89481 * locals.var_t1_dn10)), (p.p1297 * (assign54560_e89481 * locals.var_t1_dn11)),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign54560_e89487;
        locals.var_t5_dn3 = assign54560_e89487_d_n3;
        locals.var_t5_dn4 = assign54560_e89487_d_n4;
        locals.var_t5_dn5 = assign54560_e89487_d_n5;
        locals.var_t5_dn6 = assign54560_e89487_d_n6;
        locals.var_t5_dn7 = assign54560_e89487_d_n7;
        locals.var_t5_dn8 = assign54560_e89487_d_n8;
        locals.var_t5_dn9 = assign54560_e89487_d_n9;
        locals.var_t5_dn10 = assign54560_e89487_d_n10;
        locals.var_t5_dn11 = assign54560_e89487_d_n11;

        let (assign54580_e89509, assign54580_e89509_d_n3, assign54580_e89509_d_n4, assign54580_e89509_d_n5, assign54580_e89509_d_n6, assign54580_e89509_d_n7, assign54580_e89509_d_n8, assign54580_e89509_d_n9, assign54580_e89509_d_n10, assign54580_e89509_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign54580_e89505: f64 = (3.0 * locals.var_t3);
        let assign54580_e89507: f64 = (assign54580_e89505 * locals.var_t3);
        (assign54580_e89507, (((3.0 * locals.var_t3_dn3) * locals.var_t3) + (assign54580_e89505 * locals.var_t3_dn3)), (((3.0 * locals.var_t3_dn4) * locals.var_t3) + (assign54580_e89505 * locals.var_t3_dn4)), (((3.0 * locals.var_t3_dn5) * locals.var_t3) + (assign54580_e89505 * locals.var_t3_dn5)), (((3.0 * locals.var_t3_dn6) * locals.var_t3) + (assign54580_e89505 * locals.var_t3_dn6)), (((3.0 * locals.var_t3_dn7) * locals.var_t3) + (assign54580_e89505 * locals.var_t3_dn7)), (((3.0 * locals.var_t3_dn8) * locals.var_t3) + (assign54580_e89505 * locals.var_t3_dn8)), (((3.0 * locals.var_t3_dn9) * locals.var_t3) + (assign54580_e89505 * locals.var_t3_dn9)), (((3.0 * locals.var_t3_dn10) * locals.var_t3) + (assign54580_e89505 * locals.var_t3_dn10)), (((3.0 * locals.var_t3_dn11) * locals.var_t3) + (assign54580_e89505 * locals.var_t3_dn11)),)
    } else {
        (locals.var_betanoisq, locals.var_betanoisq_dn3, locals.var_betanoisq_dn4, locals.var_betanoisq_dn5, locals.var_betanoisq_dn6, locals.var_betanoisq_dn7, locals.var_betanoisq_dn8, locals.var_betanoisq_dn9, locals.var_betanoisq_dn10, locals.var_betanoisq_dn11,)
    }
};
        locals.var_betanoisq = assign54580_e89509;
        locals.var_betanoisq_dn3 = assign54580_e89509_d_n3;
        locals.var_betanoisq_dn4 = assign54580_e89509_d_n4;
        locals.var_betanoisq_dn5 = assign54580_e89509_d_n5;
        locals.var_betanoisq_dn6 = assign54580_e89509_d_n6;
        locals.var_betanoisq_dn7 = assign54580_e89509_d_n7;
        locals.var_betanoisq_dn8 = assign54580_e89509_d_n8;
        locals.var_betanoisq_dn9 = assign54580_e89509_d_n9;
        locals.var_betanoisq_dn10 = assign54580_e89509_d_n10;
        locals.var_betanoisq_dn11 = assign54580_e89509_d_n11;

        let (assign54590_e89524, assign54590_e89524_d_n3, assign54590_e89524_d_n4, assign54590_e89524_d_n5, assign54590_e89524_d_n6, assign54590_e89524_d_n7, assign54590_e89524_d_n8, assign54590_e89524_d_n9, assign54590_e89524_d_n10, assign54590_e89524_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign54590_e89514: f64 = (locals.var_betanoisq - 1.0);
        let assign54590_e89516: f64 = (-locals.var_leff);
        let assign54590_e89518: f64 = (assign54590_e89516 / p.p1296);
        let assign54590_e89519: f64 = { let limited_exp_arg = assign54590_e89518; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign54590_e89520: f64 = (assign54590_e89514 * assign54590_e89519);
        let assign54590_e89522: f64 = (assign54590_e89520 + 1.0);
        (assign54590_e89522, (locals.var_betanoisq_dn3 * assign54590_e89519), (locals.var_betanoisq_dn4 * assign54590_e89519), (locals.var_betanoisq_dn5 * assign54590_e89519), (locals.var_betanoisq_dn6 * assign54590_e89519), (locals.var_betanoisq_dn7 * assign54590_e89519), (locals.var_betanoisq_dn8 * assign54590_e89519), (locals.var_betanoisq_dn9 * assign54590_e89519), (locals.var_betanoisq_dn10 * assign54590_e89519), (locals.var_betanoisq_dn11 * assign54590_e89519),)
    } else {
        (locals.var_betanoisq, locals.var_betanoisq_dn3, locals.var_betanoisq_dn4, locals.var_betanoisq_dn5, locals.var_betanoisq_dn6, locals.var_betanoisq_dn7, locals.var_betanoisq_dn8, locals.var_betanoisq_dn9, locals.var_betanoisq_dn10, locals.var_betanoisq_dn11,)
    }
};
        locals.var_betanoisq = assign54590_e89524;
        locals.var_betanoisq_dn3 = assign54590_e89524_d_n3;
        locals.var_betanoisq_dn4 = assign54590_e89524_d_n4;
        locals.var_betanoisq_dn5 = assign54590_e89524_d_n5;
        locals.var_betanoisq_dn6 = assign54590_e89524_d_n6;
        locals.var_betanoisq_dn7 = assign54590_e89524_d_n7;
        locals.var_betanoisq_dn8 = assign54590_e89524_d_n8;
        locals.var_betanoisq_dn9 = assign54590_e89524_d_n9;
        locals.var_betanoisq_dn10 = assign54590_e89524_d_n10;
        locals.var_betanoisq_dn11 = assign54590_e89524_d_n11;

        let (assign54600_e89531, assign54600_e89531_d_n3, assign54600_e89531_d_n4, assign54600_e89531_d_n5, assign54600_e89531_d_n6, assign54600_e89531_d_n7, assign54600_e89531_d_n8, assign54600_e89531_d_n9, assign54600_e89531_d_n10, assign54600_e89531_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign54600_e89529: f64 = (locals.var_t5 * locals.var_t5);
        (assign54600_e89529, ((locals.var_t5_dn3 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn3)), ((locals.var_t5_dn4 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn4)), ((locals.var_t5_dn5 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn5)), ((locals.var_t5_dn6 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn6)), ((locals.var_t5_dn7 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn7)), ((locals.var_t5_dn8 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn8)), ((locals.var_t5_dn9 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn9)), ((locals.var_t5_dn10 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn10)), ((locals.var_t5_dn11 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn11)),)
    } else {
        (locals.var_betalowid, locals.var_betalowid_dn3, locals.var_betalowid_dn4, locals.var_betalowid_dn5, locals.var_betalowid_dn6, locals.var_betalowid_dn7, locals.var_betalowid_dn8, locals.var_betalowid_dn9, locals.var_betalowid_dn10, locals.var_betalowid_dn11,)
    }
};
        locals.var_betalowid = assign54600_e89531;
        locals.var_betalowid_dn3 = assign54600_e89531_d_n3;
        locals.var_betalowid_dn4 = assign54600_e89531_d_n4;
        locals.var_betalowid_dn5 = assign54600_e89531_d_n5;
        locals.var_betalowid_dn6 = assign54600_e89531_d_n6;
        locals.var_betalowid_dn7 = assign54600_e89531_d_n7;
        locals.var_betalowid_dn8 = assign54600_e89531_d_n8;
        locals.var_betalowid_dn9 = assign54600_e89531_d_n9;
        locals.var_betalowid_dn10 = assign54600_e89531_d_n10;
        locals.var_betalowid_dn11 = assign54600_e89531_d_n11;

        let (assign54610_e89538, assign54610_e89538_d_n3, assign54610_e89538_d_n4, assign54610_e89538_d_n5, assign54610_e89538_d_n6, assign54610_e89538_d_n7, assign54610_e89538_d_n8, assign54610_e89538_d_n9, assign54610_e89538_d_n10, assign54610_e89538_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign54610_e89536: f64 = (locals.var_t4 * locals.var_t4);
        (assign54610_e89536, ((locals.var_t4_dn3 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn3)), ((locals.var_t4_dn4 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn4)), ((locals.var_t4_dn5 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn5)), ((locals.var_t4_dn6 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn6)), ((locals.var_t4_dn7 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn7)), ((locals.var_t4_dn8 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn8)), ((locals.var_t4_dn9 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn9)), ((locals.var_t4_dn10 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn10)), ((locals.var_t4_dn11 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn11)),)
    } else {
        (locals.var_thetanoisq, locals.var_thetanoisq_dn3, locals.var_thetanoisq_dn4, locals.var_thetanoisq_dn5, locals.var_thetanoisq_dn6, locals.var_thetanoisq_dn7, locals.var_thetanoisq_dn8, locals.var_thetanoisq_dn9, locals.var_thetanoisq_dn10, locals.var_thetanoisq_dn11,)
    }
};
        locals.var_thetanoisq = assign54610_e89538;
        locals.var_thetanoisq_dn3 = assign54610_e89538_d_n3;
        locals.var_thetanoisq_dn4 = assign54610_e89538_d_n4;
        locals.var_thetanoisq_dn5 = assign54610_e89538_d_n5;
        locals.var_thetanoisq_dn6 = assign54610_e89538_d_n6;
        locals.var_thetanoisq_dn7 = assign54610_e89538_d_n7;
        locals.var_thetanoisq_dn8 = assign54610_e89538_d_n8;
        locals.var_thetanoisq_dn9 = assign54610_e89538_d_n9;
        locals.var_thetanoisq_dn10 = assign54610_e89538_d_n10;
        locals.var_thetanoisq_dn11 = assign54610_e89538_d_n11;

        let assign54630_e89546: f64 = if p.p39 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard836 = assign54630_e89546;

        let assign54640_e89549: f64 = if p.p39 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard837 = assign54640_e89549;

        let (assign54650_e89567, assign54650_e89567_d_n3, assign54650_e89567_d_n4, assign54650_e89567_d_n5, assign54650_e89567_d_n6, assign54650_e89567_d_n7, assign54650_e89567_d_n8, assign54650_e89567_d_n9, assign54650_e89567_d_n10, assign54650_e89567_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard836 != 0.0)) {
        let assign54650_e89555: f64 = (-p.p2);
        let assign54650_e89557: f64 = (assign54650_e89555 * locals.var_weff);
        let assign54650_e89559: f64 = (assign54650_e89557 * locals.var_leff);
        let assign54650_e89561: f64 = (assign54650_e89559 * locals.var_cox);
        let assign54650_e89563: f64 = (assign54650_e89561 * locals.var_vt);
        let assign54650_e89565: f64 = (assign54650_e89563 * locals.var_qs);
        (assign54650_e89565, (assign54650_e89563 * locals.var_qs_dn3), (((assign54650_e89561 * locals.var_vt_dn4) * locals.var_qs) + (assign54650_e89563 * locals.var_qs_dn4)), (((assign54650_e89561 * locals.var_vt_dn5) * locals.var_qs) + (assign54650_e89563 * locals.var_qs_dn5)), (assign54650_e89563 * locals.var_qs_dn6), (assign54650_e89563 * locals.var_qs_dn7), (assign54650_e89563 * locals.var_qs_dn8), (assign54650_e89563 * locals.var_qs_dn9), (assign54650_e89563 * locals.var_qs_dn10), (assign54650_e89563 * locals.var_qs_dn11),)
    } else {
        (locals.var_qsi, locals.var_qsi_dn3, locals.var_qsi_dn4, locals.var_qsi_dn5, locals.var_qsi_dn6, locals.var_qsi_dn7, locals.var_qsi_dn8, locals.var_qsi_dn9, locals.var_qsi_dn10, locals.var_qsi_dn11,)
    }
};
        locals.var_qsi = assign54650_e89567;
        locals.var_qsi_dn3 = assign54650_e89567_d_n3;
        locals.var_qsi_dn4 = assign54650_e89567_d_n4;
        locals.var_qsi_dn5 = assign54650_e89567_d_n5;
        locals.var_qsi_dn6 = assign54650_e89567_d_n6;
        locals.var_qsi_dn7 = assign54650_e89567_d_n7;
        locals.var_qsi_dn8 = assign54650_e89567_d_n8;
        locals.var_qsi_dn9 = assign54650_e89567_d_n9;
        locals.var_qsi_dn10 = assign54650_e89567_d_n10;
        locals.var_qsi_dn11 = assign54650_e89567_d_n11;

        let (assign54660_e89585, assign54660_e89585_d_n3, assign54660_e89585_d_n4, assign54660_e89585_d_n5, assign54660_e89585_d_n6, assign54660_e89585_d_n7, assign54660_e89585_d_n8, assign54660_e89585_d_n9, assign54660_e89585_d_n10, assign54660_e89585_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard836 != 0.0)) {
        let assign54660_e89573: f64 = (-p.p2);
        let assign54660_e89575: f64 = (assign54660_e89573 * locals.var_weff);
        let assign54660_e89577: f64 = (assign54660_e89575 * locals.var_leff);
        let assign54660_e89579: f64 = (assign54660_e89577 * locals.var_cox);
        let assign54660_e89581: f64 = (assign54660_e89579 * locals.var_vt);
        let assign54660_e89583: f64 = (assign54660_e89581 * locals.var_qd);
        (assign54660_e89583, (assign54660_e89581 * locals.var_qd_dn3), (((assign54660_e89579 * locals.var_vt_dn4) * locals.var_qd) + (assign54660_e89581 * locals.var_qd_dn4)), (((assign54660_e89579 * locals.var_vt_dn5) * locals.var_qd) + (assign54660_e89581 * locals.var_qd_dn5)), (assign54660_e89581 * locals.var_qd_dn6), (assign54660_e89581 * locals.var_qd_dn7), (assign54660_e89581 * locals.var_qd_dn8), (assign54660_e89581 * locals.var_qd_dn9), (assign54660_e89581 * locals.var_qd_dn10), (assign54660_e89581 * locals.var_qd_dn11),)
    } else {
        (locals.var_qdi, locals.var_qdi_dn3, locals.var_qdi_dn4, locals.var_qdi_dn5, locals.var_qdi_dn6, locals.var_qdi_dn7, locals.var_qdi_dn8, locals.var_qdi_dn9, locals.var_qdi_dn10, locals.var_qdi_dn11,)
    }
};
        locals.var_qdi = assign54660_e89585;
        locals.var_qdi_dn3 = assign54660_e89585_d_n3;
        locals.var_qdi_dn4 = assign54660_e89585_d_n4;
        locals.var_qdi_dn5 = assign54660_e89585_d_n5;
        locals.var_qdi_dn6 = assign54660_e89585_d_n6;
        locals.var_qdi_dn7 = assign54660_e89585_d_n7;
        locals.var_qdi_dn8 = assign54660_e89585_d_n8;
        locals.var_qdi_dn9 = assign54660_e89585_d_n9;
        locals.var_qdi_dn10 = assign54660_e89585_d_n10;
        locals.var_qdi_dn11 = assign54660_e89585_d_n11;

        let (assign54670_e89597, assign54670_e89597_d_n3, assign54670_e89597_d_n4, assign54670_e89597_d_n5, assign54670_e89597_d_n6, assign54670_e89597_d_n7, assign54670_e89597_d_n8, assign54670_e89597_d_n9, assign54670_e89597_d_n10, assign54670_e89597_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard836 != 0.0)) {
        let assign54670_e89593: f64 = (locals.var_qsi + locals.var_qdi);
        let assign54670_e89594: f64 = (assign54670_e89593).abs();
        let assign54670_e89595: f64 = (locals.var_ueff * assign54670_e89594);
        (assign54670_e89595, ((locals.var_ueff_dn3 * assign54670_e89594) + (locals.var_ueff * if assign54670_e89593 >= 0.0 { (locals.var_qsi_dn3 + locals.var_qdi_dn3) } else { (-(locals.var_qsi_dn3 + locals.var_qdi_dn3)) })), ((locals.var_ueff_dn4 * assign54670_e89594) + (locals.var_ueff * if assign54670_e89593 >= 0.0 { (locals.var_qsi_dn4 + locals.var_qdi_dn4) } else { (-(locals.var_qsi_dn4 + locals.var_qdi_dn4)) })), ((locals.var_ueff_dn5 * assign54670_e89594) + (locals.var_ueff * if assign54670_e89593 >= 0.0 { (locals.var_qsi_dn5 + locals.var_qdi_dn5) } else { (-(locals.var_qsi_dn5 + locals.var_qdi_dn5)) })), ((locals.var_ueff_dn6 * assign54670_e89594) + (locals.var_ueff * if assign54670_e89593 >= 0.0 { (locals.var_qsi_dn6 + locals.var_qdi_dn6) } else { (-(locals.var_qsi_dn6 + locals.var_qdi_dn6)) })), ((locals.var_ueff_dn7 * assign54670_e89594) + (locals.var_ueff * if assign54670_e89593 >= 0.0 { (locals.var_qsi_dn7 + locals.var_qdi_dn7) } else { (-(locals.var_qsi_dn7 + locals.var_qdi_dn7)) })), ((locals.var_ueff_dn8 * assign54670_e89594) + (locals.var_ueff * if assign54670_e89593 >= 0.0 { (locals.var_qsi_dn8 + locals.var_qdi_dn8) } else { (-(locals.var_qsi_dn8 + locals.var_qdi_dn8)) })), ((locals.var_ueff_dn9 * assign54670_e89594) + (locals.var_ueff * if assign54670_e89593 >= 0.0 { (locals.var_qsi_dn9 + locals.var_qdi_dn9) } else { (-(locals.var_qsi_dn9 + locals.var_qdi_dn9)) })), ((locals.var_ueff_dn10 * assign54670_e89594) + (locals.var_ueff * if assign54670_e89593 >= 0.0 { (locals.var_qsi_dn10 + locals.var_qdi_dn10) } else { (-(locals.var_qsi_dn10 + locals.var_qdi_dn10)) })), ((locals.var_ueff_dn11 * assign54670_e89594) + (locals.var_ueff * if assign54670_e89593 >= 0.0 { (locals.var_qsi_dn11 + locals.var_qdi_dn11) } else { (-(locals.var_qsi_dn11 + locals.var_qdi_dn11)) })),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign54670_e89597;
        locals.var_t0_dn3 = assign54670_e89597_d_n3;
        locals.var_t0_dn4 = assign54670_e89597_d_n4;
        locals.var_t0_dn5 = assign54670_e89597_d_n5;
        locals.var_t0_dn6 = assign54670_e89597_d_n6;
        locals.var_t0_dn7 = assign54670_e89597_d_n7;
        locals.var_t0_dn8 = assign54670_e89597_d_n8;
        locals.var_t0_dn9 = assign54670_e89597_d_n9;
        locals.var_t0_dn10 = assign54670_e89597_d_n10;
        locals.var_t0_dn11 = assign54670_e89597_d_n11;

        let (assign54680_e89610, assign54680_e89610_d_n3, assign54680_e89610_d_n4, assign54680_e89610_d_n5, assign54680_e89610_d_n6, assign54680_e89610_d_n7, assign54680_e89610_d_n8, assign54680_e89610_d_n9, assign54680_e89610_d_n10, assign54680_e89610_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard836 != 0.0)) {
        let assign54680_e89604: f64 = (locals.var_t0 * locals.var_rdsi);
        let assign54680_e89607: f64 = (locals.var_leff * locals.var_leff);
        let assign54680_e89608: f64 = (assign54680_e89604 + assign54680_e89607);
        (assign54680_e89608, ((locals.var_t0_dn3 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn3)), ((locals.var_t0_dn4 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn4)), ((locals.var_t0_dn5 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn5)), ((locals.var_t0_dn6 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn6)), ((locals.var_t0_dn7 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn7)), ((locals.var_t0_dn8 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn8)), ((locals.var_t0_dn9 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn9)), ((locals.var_t0_dn10 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn10)), ((locals.var_t0_dn11 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn11)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign54680_e89610;
        locals.var_t1_dn3 = assign54680_e89610_d_n3;
        locals.var_t1_dn4 = assign54680_e89610_d_n4;
        locals.var_t1_dn5 = assign54680_e89610_d_n5;
        locals.var_t1_dn6 = assign54680_e89610_d_n6;
        locals.var_t1_dn7 = assign54680_e89610_d_n7;
        locals.var_t1_dn8 = assign54680_e89610_d_n8;
        locals.var_t1_dn9 = assign54680_e89610_d_n9;
        locals.var_t1_dn10 = assign54680_e89610_d_n10;
        locals.var_t1_dn11 = assign54680_e89610_d_n11;

        let (assign54710_e89644, assign54710_e89644_d_n3, assign54710_e89644_d_n4, assign54710_e89644_d_n5, assign54710_e89644_d_n6, assign54710_e89644_d_n7, assign54710_e89644_d_n8, assign54710_e89644_d_n9, assign54710_e89644_d_n10, assign54710_e89644_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && ((locals.var_guard837 != 0.0) && (locals.var_guard836 == 0.0))) {
        let assign54710_e89640: f64 = (2.0 * locals.var_nq);
        let assign54710_e89642: f64 = (assign54710_e89640 * locals.var_nvt);
        (assign54710_e89642, (((2.0 * locals.var_nq_dn3) * locals.var_nvt) + (assign54710_e89640 * locals.var_nvt_dn3)), (((2.0 * locals.var_nq_dn4) * locals.var_nvt) + (assign54710_e89640 * locals.var_nvt_dn4)), (((2.0 * locals.var_nq_dn5) * locals.var_nvt) + (assign54710_e89640 * locals.var_nvt_dn5)), (((2.0 * locals.var_nq_dn6) * locals.var_nvt) + (assign54710_e89640 * locals.var_nvt_dn6)), (((2.0 * locals.var_nq_dn7) * locals.var_nvt) + (assign54710_e89640 * locals.var_nvt_dn7)), (((2.0 * locals.var_nq_dn8) * locals.var_nvt) + (assign54710_e89640 * locals.var_nvt_dn8)), (((2.0 * locals.var_nq_dn9) * locals.var_nvt) + (assign54710_e89640 * locals.var_nvt_dn9)), (((2.0 * locals.var_nq_dn10) * locals.var_nvt) + (assign54710_e89640 * locals.var_nvt_dn10)), (((2.0 * locals.var_nq_dn11) * locals.var_nvt) + (assign54710_e89640 * locals.var_nvt_dn11)),)
    } else {
        (locals.var_vtn, locals.var_vtn_dn3, locals.var_vtn_dn4, locals.var_vtn_dn5, locals.var_vtn_dn6, locals.var_vtn_dn7, locals.var_vtn_dn8, locals.var_vtn_dn9, locals.var_vtn_dn10, locals.var_vtn_dn11,)
    }
};
        locals.var_vtn = assign54710_e89644;
        locals.var_vtn_dn3 = assign54710_e89644_d_n3;
        locals.var_vtn_dn4 = assign54710_e89644_d_n4;
        locals.var_vtn_dn5 = assign54710_e89644_d_n5;
        locals.var_vtn_dn6 = assign54710_e89644_d_n6;
        locals.var_vtn_dn7 = assign54710_e89644_d_n7;
        locals.var_vtn_dn8 = assign54710_e89644_d_n8;
        locals.var_vtn_dn9 = assign54710_e89644_d_n9;
        locals.var_vtn_dn10 = assign54710_e89644_d_n10;
        locals.var_vtn_dn11 = assign54710_e89644_d_n11;

    }

    pub(super) fn stamp_transient_block_185(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign54720_e89662, assign54720_e89662_d_n3, assign54720_e89662_d_n4, assign54720_e89662_d_n5, assign54720_e89662_d_n6, assign54720_e89662_d_n7, assign54720_e89662_d_n8, assign54720_e89662_d_n9, assign54720_e89662_d_n10, assign54720_e89662_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && ((locals.var_guard837 != 0.0) && (locals.var_guard836 == 0.0))) {
        let assign54720_e89654: f64 = (locals.var_ueff * locals.var_dptwg);
        let assign54720_e89656: f64 = (assign54720_e89654 * locals.var_moc);
        let assign54720_e89658: f64 = (assign54720_e89656 * locals.var_cox);
        let assign54720_e89660: f64 = (assign54720_e89658 * locals.var_vtn);
        (assign54720_e89660, (((((((locals.var_ueff_dn3 * locals.var_dptwg) + (locals.var_ueff * locals.var_dptwg_dn3)) * locals.var_moc) + (assign54720_e89654 * locals.var_moc_dn3)) * locals.var_cox) * locals.var_vtn) + (assign54720_e89658 * locals.var_vtn_dn3)), (((((((locals.var_ueff_dn4 * locals.var_dptwg) + (locals.var_ueff * locals.var_dptwg_dn4)) * locals.var_moc) + (assign54720_e89654 * locals.var_moc_dn4)) * locals.var_cox) * locals.var_vtn) + (assign54720_e89658 * locals.var_vtn_dn4)), (((((((locals.var_ueff_dn5 * locals.var_dptwg) + (locals.var_ueff * locals.var_dptwg_dn5)) * locals.var_moc) + (assign54720_e89654 * locals.var_moc_dn5)) * locals.var_cox) * locals.var_vtn) + (assign54720_e89658 * locals.var_vtn_dn5)), (((((((locals.var_ueff_dn6 * locals.var_dptwg) + (locals.var_ueff * locals.var_dptwg_dn6)) * locals.var_moc) + (assign54720_e89654 * locals.var_moc_dn6)) * locals.var_cox) * locals.var_vtn) + (assign54720_e89658 * locals.var_vtn_dn6)), (((((((locals.var_ueff_dn7 * locals.var_dptwg) + (locals.var_ueff * locals.var_dptwg_dn7)) * locals.var_moc) + (assign54720_e89654 * locals.var_moc_dn7)) * locals.var_cox) * locals.var_vtn) + (assign54720_e89658 * locals.var_vtn_dn7)), (((((((locals.var_ueff_dn8 * locals.var_dptwg) + (locals.var_ueff * locals.var_dptwg_dn8)) * locals.var_moc) + (assign54720_e89654 * locals.var_moc_dn8)) * locals.var_cox) * locals.var_vtn) + (assign54720_e89658 * locals.var_vtn_dn8)), (((((((locals.var_ueff_dn9 * locals.var_dptwg) + (locals.var_ueff * locals.var_dptwg_dn9)) * locals.var_moc) + (assign54720_e89654 * locals.var_moc_dn9)) * locals.var_cox) * locals.var_vtn) + (assign54720_e89658 * locals.var_vtn_dn9)), (((((((locals.var_ueff_dn10 * locals.var_dptwg) + (locals.var_ueff * locals.var_dptwg_dn10)) * locals.var_moc) + (assign54720_e89654 * locals.var_moc_dn10)) * locals.var_cox) * locals.var_vtn) + (assign54720_e89658 * locals.var_vtn_dn10)), (((((((locals.var_ueff_dn11 * locals.var_dptwg) + (locals.var_ueff * locals.var_dptwg_dn11)) * locals.var_moc) + (assign54720_e89654 * locals.var_moc_dn11)) * locals.var_cox) * locals.var_vtn) + (assign54720_e89658 * locals.var_vtn_dn11)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign54720_e89662;
        locals.var_t0_dn3 = assign54720_e89662_d_n3;
        locals.var_t0_dn4 = assign54720_e89662_d_n4;
        locals.var_t0_dn5 = assign54720_e89662_d_n5;
        locals.var_t0_dn6 = assign54720_e89662_d_n6;
        locals.var_t0_dn7 = assign54720_e89662_d_n7;
        locals.var_t0_dn8 = assign54720_e89662_d_n8;
        locals.var_t0_dn9 = assign54720_e89662_d_n9;
        locals.var_t0_dn10 = assign54720_e89662_d_n10;
        locals.var_t0_dn11 = assign54720_e89662_d_n11;

        let (assign54730_e89676, assign54730_e89676_d_n3, assign54730_e89676_d_n4, assign54730_e89676_d_n5, assign54730_e89676_d_n6, assign54730_e89676_d_n7, assign54730_e89676_d_n8, assign54730_e89676_d_n9, assign54730_e89676_d_n10, assign54730_e89676_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && ((locals.var_guard837 != 0.0) && (locals.var_guard836 == 0.0))) {
        let assign54730_e89673: f64 = (locals.var_qs_1 + locals.var_qdeff);
        let assign54730_e89674: f64 = (0.5 * assign54730_e89673);
        (assign54730_e89674, (0.5 * (locals.var_qs_1_dn3 + locals.var_qdeff_dn3)), (0.5 * (locals.var_qs_1_dn4 + locals.var_qdeff_dn4)), (0.5 * (locals.var_qs_1_dn5 + locals.var_qdeff_dn5)), (0.5 * (locals.var_qs_1_dn6 + locals.var_qdeff_dn6)), (0.5 * (locals.var_qs_1_dn7 + locals.var_qdeff_dn7)), (0.5 * (locals.var_qs_1_dn8 + locals.var_qdeff_dn8)), (0.5 * (locals.var_qs_1_dn9 + locals.var_qdeff_dn9)), (0.5 * (locals.var_qs_1_dn10 + locals.var_qdeff_dn10)), (0.5 * (locals.var_qs_1_dn11 + locals.var_qdeff_dn11)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign54730_e89676;
        locals.var_t1_dn3 = assign54730_e89676_d_n3;
        locals.var_t1_dn4 = assign54730_e89676_d_n4;
        locals.var_t1_dn5 = assign54730_e89676_d_n5;
        locals.var_t1_dn6 = assign54730_e89676_d_n6;
        locals.var_t1_dn7 = assign54730_e89676_d_n7;
        locals.var_t1_dn8 = assign54730_e89676_d_n8;
        locals.var_t1_dn9 = assign54730_e89676_d_n9;
        locals.var_t1_dn10 = assign54730_e89676_d_n10;
        locals.var_t1_dn11 = assign54730_e89676_d_n11;

        let (assign54740_e89688, assign54740_e89688_d_n3, assign54740_e89688_d_n4, assign54740_e89688_d_n5, assign54740_e89688_d_n6, assign54740_e89688_d_n7, assign54740_e89688_d_n8, assign54740_e89688_d_n9, assign54740_e89688_d_n10, assign54740_e89688_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && ((locals.var_guard837 != 0.0) && (locals.var_guard836 == 0.0))) {
        let assign54740_e89686: f64 = (locals.var_t1 + 0.5);
        (assign54740_e89686, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign54740_e89688;
        locals.var_t3_dn3 = assign54740_e89688_d_n3;
        locals.var_t3_dn4 = assign54740_e89688_d_n4;
        locals.var_t3_dn5 = assign54740_e89688_d_n5;
        locals.var_t3_dn6 = assign54740_e89688_d_n6;
        locals.var_t3_dn7 = assign54740_e89688_d_n7;
        locals.var_t3_dn8 = assign54740_e89688_d_n8;
        locals.var_t3_dn9 = assign54740_e89688_d_n9;
        locals.var_t3_dn10 = assign54740_e89688_d_n10;
        locals.var_t3_dn11 = assign54740_e89688_d_n11;

        let (assign54750_e89700, assign54750_e89700_d_n3, assign54750_e89700_d_n4, assign54750_e89700_d_n5, assign54750_e89700_d_n6, assign54750_e89700_d_n7, assign54750_e89700_d_n8, assign54750_e89700_d_n9, assign54750_e89700_d_n10, assign54750_e89700_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && ((locals.var_guard837 != 0.0) && (locals.var_guard836 == 0.0))) {
        let assign54750_e89698: f64 = (locals.var_t3 * locals.var_t3);
        (assign54750_e89698, ((locals.var_t3_dn3 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn3)), ((locals.var_t3_dn4 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn4)), ((locals.var_t3_dn5 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn5)), ((locals.var_t3_dn6 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn6)), ((locals.var_t3_dn7 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn7)), ((locals.var_t3_dn8 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn8)), ((locals.var_t3_dn9 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn9)), ((locals.var_t3_dn10 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn10)), ((locals.var_t3_dn11 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn11)),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign54750_e89700;
        locals.var_t4_dn3 = assign54750_e89700_d_n3;
        locals.var_t4_dn4 = assign54750_e89700_d_n4;
        locals.var_t4_dn5 = assign54750_e89700_d_n5;
        locals.var_t4_dn6 = assign54750_e89700_d_n6;
        locals.var_t4_dn7 = assign54750_e89700_d_n7;
        locals.var_t4_dn8 = assign54750_e89700_d_n8;
        locals.var_t4_dn9 = assign54750_e89700_d_n9;
        locals.var_t4_dn10 = assign54750_e89700_d_n10;
        locals.var_t4_dn11 = assign54750_e89700_d_n11;

        let (assign54760_e89712, assign54760_e89712_d_n3, assign54760_e89712_d_n4, assign54760_e89712_d_n5, assign54760_e89712_d_n6, assign54760_e89712_d_n7, assign54760_e89712_d_n8, assign54760_e89712_d_n9, assign54760_e89712_d_n10, assign54760_e89712_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && ((locals.var_guard837 != 0.0) && (locals.var_guard836 == 0.0))) {
        let assign54760_e89710: f64 = (locals.var_t4 * locals.var_t3);
        (assign54760_e89710, ((locals.var_t4_dn3 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn3)), ((locals.var_t4_dn4 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn4)), ((locals.var_t4_dn5 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn5)), ((locals.var_t4_dn6 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn6)), ((locals.var_t4_dn7 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn7)), ((locals.var_t4_dn8 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn8)), ((locals.var_t4_dn9 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn9)), ((locals.var_t4_dn10 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn10)), ((locals.var_t4_dn11 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn11)),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign54760_e89712;
        locals.var_t5_dn3 = assign54760_e89712_d_n3;
        locals.var_t5_dn4 = assign54760_e89712_d_n4;
        locals.var_t5_dn5 = assign54760_e89712_d_n5;
        locals.var_t5_dn6 = assign54760_e89712_d_n6;
        locals.var_t5_dn7 = assign54760_e89712_d_n7;
        locals.var_t5_dn8 = assign54760_e89712_d_n8;
        locals.var_t5_dn9 = assign54760_e89712_d_n9;
        locals.var_t5_dn10 = assign54760_e89712_d_n10;
        locals.var_t5_dn11 = assign54760_e89712_d_n11;

        let (assign54770_e89724, assign54770_e89724_d_n3, assign54770_e89724_d_n4, assign54770_e89724_d_n5, assign54770_e89724_d_n6, assign54770_e89724_d_n7, assign54770_e89724_d_n8, assign54770_e89724_d_n9, assign54770_e89724_d_n10, assign54770_e89724_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && ((locals.var_guard837 != 0.0) && (locals.var_guard836 == 0.0))) {
        let assign54770_e89722: f64 = (locals.var_qs_1 - locals.var_qdeff);
        (assign54770_e89722, (locals.var_qs_1_dn3 - locals.var_qdeff_dn3), (locals.var_qs_1_dn4 - locals.var_qdeff_dn4), (locals.var_qs_1_dn5 - locals.var_qdeff_dn5), (locals.var_qs_1_dn6 - locals.var_qdeff_dn6), (locals.var_qs_1_dn7 - locals.var_qdeff_dn7), (locals.var_qs_1_dn8 - locals.var_qdeff_dn8), (locals.var_qs_1_dn9 - locals.var_qdeff_dn9), (locals.var_qs_1_dn10 - locals.var_qdeff_dn10), (locals.var_qs_1_dn11 - locals.var_qdeff_dn11),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign54770_e89724;
        locals.var_t6_dn3 = assign54770_e89724_d_n3;
        locals.var_t6_dn4 = assign54770_e89724_d_n4;
        locals.var_t6_dn5 = assign54770_e89724_d_n5;
        locals.var_t6_dn6 = assign54770_e89724_d_n6;
        locals.var_t6_dn7 = assign54770_e89724_d_n7;
        locals.var_t6_dn8 = assign54770_e89724_d_n8;
        locals.var_t6_dn9 = assign54770_e89724_d_n9;
        locals.var_t6_dn10 = assign54770_e89724_d_n10;
        locals.var_t6_dn11 = assign54770_e89724_d_n11;

        let (assign54780_e89736, assign54780_e89736_d_n3, assign54780_e89736_d_n4, assign54780_e89736_d_n5, assign54780_e89736_d_n6, assign54780_e89736_d_n7, assign54780_e89736_d_n8, assign54780_e89736_d_n9, assign54780_e89736_d_n10, assign54780_e89736_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && ((locals.var_guard837 != 0.0) && (locals.var_guard836 == 0.0))) {
        let assign54780_e89734: f64 = (locals.var_t6 * locals.var_t6);
        (assign54780_e89734, ((locals.var_t6_dn3 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn3)), ((locals.var_t6_dn4 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn4)), ((locals.var_t6_dn5 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn5)), ((locals.var_t6_dn6 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn6)), ((locals.var_t6_dn7 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn7)), ((locals.var_t6_dn8 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn8)), ((locals.var_t6_dn9 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn9)), ((locals.var_t6_dn10 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn10)), ((locals.var_t6_dn11 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn11)),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign54780_e89736;
        locals.var_t7_dn3 = assign54780_e89736_d_n3;
        locals.var_t7_dn4 = assign54780_e89736_d_n4;
        locals.var_t7_dn5 = assign54780_e89736_d_n5;
        locals.var_t7_dn6 = assign54780_e89736_d_n6;
        locals.var_t7_dn7 = assign54780_e89736_d_n7;
        locals.var_t7_dn8 = assign54780_e89736_d_n8;
        locals.var_t7_dn9 = assign54780_e89736_d_n9;
        locals.var_t7_dn10 = assign54780_e89736_d_n10;
        locals.var_t7_dn11 = assign54780_e89736_d_n11;

        let (assign54790_e89748, assign54790_e89748_d_n3, assign54790_e89748_d_n4, assign54790_e89748_d_n5, assign54790_e89748_d_n6, assign54790_e89748_d_n7, assign54790_e89748_d_n8, assign54790_e89748_d_n9, assign54790_e89748_d_n10, assign54790_e89748_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && ((locals.var_guard837 != 0.0) && (locals.var_guard836 == 0.0))) {
        let assign54790_e89746: f64 = (locals.var_t7 * locals.var_t6);
        (assign54790_e89746, ((locals.var_t7_dn3 * locals.var_t6) + (locals.var_t7 * locals.var_t6_dn3)), ((locals.var_t7_dn4 * locals.var_t6) + (locals.var_t7 * locals.var_t6_dn4)), ((locals.var_t7_dn5 * locals.var_t6) + (locals.var_t7 * locals.var_t6_dn5)), ((locals.var_t7_dn6 * locals.var_t6) + (locals.var_t7 * locals.var_t6_dn6)), ((locals.var_t7_dn7 * locals.var_t6) + (locals.var_t7 * locals.var_t6_dn7)), ((locals.var_t7_dn8 * locals.var_t6) + (locals.var_t7 * locals.var_t6_dn8)), ((locals.var_t7_dn9 * locals.var_t6) + (locals.var_t7 * locals.var_t6_dn9)), ((locals.var_t7_dn10 * locals.var_t6) + (locals.var_t7 * locals.var_t6_dn10)), ((locals.var_t7_dn11 * locals.var_t6) + (locals.var_t7 * locals.var_t6_dn11)),)
    } else {
        (locals.var_t8, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11,)
    }
};
        locals.var_t8 = assign54790_e89748;
        locals.var_t8_dn3 = assign54790_e89748_d_n3;
        locals.var_t8_dn4 = assign54790_e89748_d_n4;
        locals.var_t8_dn5 = assign54790_e89748_d_n5;
        locals.var_t8_dn6 = assign54790_e89748_d_n6;
        locals.var_t8_dn7 = assign54790_e89748_d_n7;
        locals.var_t8_dn8 = assign54790_e89748_d_n8;
        locals.var_t8_dn9 = assign54790_e89748_d_n9;
        locals.var_t8_dn10 = assign54790_e89748_d_n10;
        locals.var_t8_dn11 = assign54790_e89748_d_n11;

        let (assign54800_e89764, assign54800_e89764_d_n3, assign54800_e89764_d_n4, assign54800_e89764_d_n5, assign54800_e89764_d_n6, assign54800_e89764_d_n7, assign54800_e89764_d_n8, assign54800_e89764_d_n9, assign54800_e89764_d_n10, assign54800_e89764_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && ((locals.var_guard837 != 0.0) && (locals.var_guard836 == 0.0))) {
        let assign54800_e89758: f64 = (6.0 * locals.var_t1);
        let assign54800_e89760: f64 = (assign54800_e89758 + 0.5);
        let assign54800_e89762: f64 = (assign54800_e89760 * locals.var_t7);
        (assign54800_e89762, (((6.0 * locals.var_t1_dn3) * locals.var_t7) + (assign54800_e89760 * locals.var_t7_dn3)), (((6.0 * locals.var_t1_dn4) * locals.var_t7) + (assign54800_e89760 * locals.var_t7_dn4)), (((6.0 * locals.var_t1_dn5) * locals.var_t7) + (assign54800_e89760 * locals.var_t7_dn5)), (((6.0 * locals.var_t1_dn6) * locals.var_t7) + (assign54800_e89760 * locals.var_t7_dn6)), (((6.0 * locals.var_t1_dn7) * locals.var_t7) + (assign54800_e89760 * locals.var_t7_dn7)), (((6.0 * locals.var_t1_dn8) * locals.var_t7) + (assign54800_e89760 * locals.var_t7_dn8)), (((6.0 * locals.var_t1_dn9) * locals.var_t7) + (assign54800_e89760 * locals.var_t7_dn9)), (((6.0 * locals.var_t1_dn10) * locals.var_t7) + (assign54800_e89760 * locals.var_t7_dn10)), (((6.0 * locals.var_t1_dn11) * locals.var_t7) + (assign54800_e89760 * locals.var_t7_dn11)),)
    } else {
        (locals.var_t9, locals.var_t9_dn3, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11,)
    }
};
        locals.var_t9 = assign54800_e89764;
        locals.var_t9_dn3 = assign54800_e89764_d_n3;
        locals.var_t9_dn4 = assign54800_e89764_d_n4;
        locals.var_t9_dn5 = assign54800_e89764_d_n5;
        locals.var_t9_dn6 = assign54800_e89764_d_n6;
        locals.var_t9_dn7 = assign54800_e89764_d_n7;
        locals.var_t9_dn8 = assign54800_e89764_d_n8;
        locals.var_t9_dn9 = assign54800_e89764_d_n9;
        locals.var_t9_dn10 = assign54800_e89764_d_n10;
        locals.var_t9_dn11 = assign54800_e89764_d_n11;

        let (assign54810_e89776, assign54810_e89776_d_n3, assign54810_e89776_d_n4, assign54810_e89776_d_n5, assign54810_e89776_d_n6, assign54810_e89776_d_n7, assign54810_e89776_d_n8, assign54810_e89776_d_n9, assign54810_e89776_d_n10, assign54810_e89776_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && ((locals.var_guard837 != 0.0) && (locals.var_guard836 == 0.0))) {
        let assign54810_e89774: f64 = (locals.var_leff * locals.var_dptwg);
        (assign54810_e89774, (locals.var_leff * locals.var_dptwg_dn3), (locals.var_leff * locals.var_dptwg_dn4), (locals.var_leff * locals.var_dptwg_dn5), (locals.var_leff * locals.var_dptwg_dn6), (locals.var_leff * locals.var_dptwg_dn7), (locals.var_leff * locals.var_dptwg_dn8), (locals.var_leff * locals.var_dptwg_dn9), (locals.var_leff * locals.var_dptwg_dn10), (locals.var_leff * locals.var_dptwg_dn11),)
    } else {
        (locals.var_lvsat, locals.var_lvsat_dn3, locals.var_lvsat_dn4, locals.var_lvsat_dn5, locals.var_lvsat_dn6, locals.var_lvsat_dn7, locals.var_lvsat_dn8, locals.var_lvsat_dn9, locals.var_lvsat_dn10, locals.var_lvsat_dn11,)
    }
};
        locals.var_lvsat = assign54810_e89776;
        locals.var_lvsat_dn3 = assign54810_e89776_d_n3;
        locals.var_lvsat_dn4 = assign54810_e89776_d_n4;
        locals.var_lvsat_dn5 = assign54810_e89776_d_n5;
        locals.var_lvsat_dn6 = assign54810_e89776_d_n6;
        locals.var_lvsat_dn7 = assign54810_e89776_d_n7;
        locals.var_lvsat_dn8 = assign54810_e89776_d_n8;
        locals.var_lvsat_dn9 = assign54810_e89776_d_n9;
        locals.var_lvsat_dn10 = assign54810_e89776_d_n10;
        locals.var_lvsat_dn11 = assign54810_e89776_d_n11;

        let (assign54820_e89788, assign54820_e89788_d_n3, assign54820_e89788_d_n4, assign54820_e89788_d_n5, assign54820_e89788_d_n6, assign54820_e89788_d_n7, assign54820_e89788_d_n8, assign54820_e89788_d_n9, assign54820_e89788_d_n10, assign54820_e89788_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && ((locals.var_guard837 != 0.0) && (locals.var_guard836 == 0.0))) {
        let assign54820_e89786: f64 = (locals.var_lvsat / locals.var_leff);
        (assign54820_e89786, (locals.var_lvsat_dn3 / locals.var_leff), (locals.var_lvsat_dn4 / locals.var_leff), (locals.var_lvsat_dn5 / locals.var_leff), (locals.var_lvsat_dn6 / locals.var_leff), (locals.var_lvsat_dn7 / locals.var_leff), (locals.var_lvsat_dn8 / locals.var_leff), (locals.var_lvsat_dn9 / locals.var_leff), (locals.var_lvsat_dn10 / locals.var_leff), (locals.var_lvsat_dn11 / locals.var_leff),)
    } else {
        (locals.var_t10, locals.var_t10_dn3, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11,)
    }
};
        locals.var_t10 = assign54820_e89788;
        locals.var_t10_dn3 = assign54820_e89788_d_n3;
        locals.var_t10_dn4 = assign54820_e89788_d_n4;
        locals.var_t10_dn5 = assign54820_e89788_d_n5;
        locals.var_t10_dn6 = assign54820_e89788_d_n6;
        locals.var_t10_dn7 = assign54820_e89788_d_n7;
        locals.var_t10_dn8 = assign54820_e89788_d_n8;
        locals.var_t10_dn9 = assign54820_e89788_d_n9;
        locals.var_t10_dn10 = assign54820_e89788_d_n10;
        locals.var_t10_dn11 = assign54820_e89788_d_n11;

        let (assign54830_e89808, assign54830_e89808_d_n3, assign54830_e89808_d_n4, assign54830_e89808_d_n5, assign54830_e89808_d_n6, assign54830_e89808_d_n7, assign54830_e89808_d_n8, assign54830_e89808_d_n9, assign54830_e89808_d_n10, assign54830_e89808_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && ((locals.var_guard837 != 0.0) && (locals.var_guard836 == 0.0))) {
        let assign54830_e89800: f64 = (locals.var_vdseff / locals.var_vdssat);
        let assign54830_e89801: f64 = (locals.var_betalowid * assign54830_e89800);
        let assign54830_e89804: f64 = (p.p1299 + locals.var_qia);
        let assign54830_e89805: f64 = (assign54830_e89801 / assign54830_e89804);
        let assign54830_e89806: f64 = (1.0 + assign54830_e89805);
        (assign54830_e89806, (((((locals.var_betalowid_dn3 * assign54830_e89800) + (locals.var_betalowid * (((locals.var_vdseff_dn3 * locals.var_vdssat) - (locals.var_vdseff * locals.var_vdssat_dn3)) / (locals.var_vdssat * locals.var_vdssat)))) * assign54830_e89804) - (assign54830_e89801 * locals.var_qia_dn3)) / (assign54830_e89804 * assign54830_e89804)), (((((locals.var_betalowid_dn4 * assign54830_e89800) + (locals.var_betalowid * (((locals.var_vdseff_dn4 * locals.var_vdssat) - (locals.var_vdseff * locals.var_vdssat_dn4)) / (locals.var_vdssat * locals.var_vdssat)))) * assign54830_e89804) - (assign54830_e89801 * locals.var_qia_dn4)) / (assign54830_e89804 * assign54830_e89804)), (((((locals.var_betalowid_dn5 * assign54830_e89800) + (locals.var_betalowid * (((locals.var_vdseff_dn5 * locals.var_vdssat) - (locals.var_vdseff * locals.var_vdssat_dn5)) / (locals.var_vdssat * locals.var_vdssat)))) * assign54830_e89804) - (assign54830_e89801 * locals.var_qia_dn5)) / (assign54830_e89804 * assign54830_e89804)), (((((locals.var_betalowid_dn6 * assign54830_e89800) + (locals.var_betalowid * (((locals.var_vdseff_dn6 * locals.var_vdssat) - (locals.var_vdseff * locals.var_vdssat_dn6)) / (locals.var_vdssat * locals.var_vdssat)))) * assign54830_e89804) - (assign54830_e89801 * locals.var_qia_dn6)) / (assign54830_e89804 * assign54830_e89804)), (((((locals.var_betalowid_dn7 * assign54830_e89800) + (locals.var_betalowid * (((locals.var_vdseff_dn7 * locals.var_vdssat) - (locals.var_vdseff * locals.var_vdssat_dn7)) / (locals.var_vdssat * locals.var_vdssat)))) * assign54830_e89804) - (assign54830_e89801 * locals.var_qia_dn7)) / (assign54830_e89804 * assign54830_e89804)), (((((locals.var_betalowid_dn8 * assign54830_e89800) + (locals.var_betalowid * (((locals.var_vdseff_dn8 * locals.var_vdssat) - (locals.var_vdseff * locals.var_vdssat_dn8)) / (locals.var_vdssat * locals.var_vdssat)))) * assign54830_e89804) - (assign54830_e89801 * locals.var_qia_dn8)) / (assign54830_e89804 * assign54830_e89804)), (((((locals.var_betalowid_dn9 * assign54830_e89800) + (locals.var_betalowid * (((locals.var_vdseff_dn9 * locals.var_vdssat) - (locals.var_vdseff * locals.var_vdssat_dn9)) / (locals.var_vdssat * locals.var_vdssat)))) * assign54830_e89804) - (assign54830_e89801 * locals.var_qia_dn9)) / (assign54830_e89804 * assign54830_e89804)), (((((locals.var_betalowid_dn10 * assign54830_e89800) + (locals.var_betalowid * (((locals.var_vdseff_dn10 * locals.var_vdssat) - (locals.var_vdseff * locals.var_vdssat_dn10)) / (locals.var_vdssat * locals.var_vdssat)))) * assign54830_e89804) - (assign54830_e89801 * locals.var_qia_dn10)) / (assign54830_e89804 * assign54830_e89804)), (((((locals.var_betalowid_dn11 * assign54830_e89800) + (locals.var_betalowid * (((locals.var_vdseff_dn11 * locals.var_vdssat) - (locals.var_vdseff * locals.var_vdssat_dn11)) / (locals.var_vdssat * locals.var_vdssat)))) * assign54830_e89804) - (assign54830_e89801 * locals.var_qia_dn11)) / (assign54830_e89804 * assign54830_e89804)),)
    } else {
        (locals.var_t12, locals.var_t12_dn3, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn11,)
    }
};
        locals.var_t12 = assign54830_e89808;
        locals.var_t12_dn3 = assign54830_e89808_d_n3;
        locals.var_t12_dn4 = assign54830_e89808_d_n4;
        locals.var_t12_dn5 = assign54830_e89808_d_n5;
        locals.var_t12_dn6 = assign54830_e89808_d_n6;
        locals.var_t12_dn7 = assign54830_e89808_d_n7;
        locals.var_t12_dn8 = assign54830_e89808_d_n8;
        locals.var_t12_dn9 = assign54830_e89808_d_n9;
        locals.var_t12_dn10 = assign54830_e89808_d_n10;
        locals.var_t12_dn11 = assign54830_e89808_d_n11;

        let (assign54840_e89828, assign54840_e89828_d_n3, assign54840_e89828_d_n4, assign54840_e89828_d_n5, assign54840_e89828_d_n6, assign54840_e89828_d_n7, assign54840_e89828_d_n8, assign54840_e89828_d_n9, assign54840_e89828_d_n10, assign54840_e89828_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && ((locals.var_guard837 != 0.0) && (locals.var_guard836 == 0.0))) {
        let assign54840_e89818: f64 = (locals.var_t12 - 1.0);
        let assign54840_e89820: f64 = (-locals.var_leff);
        let assign54840_e89822: f64 = (assign54840_e89820 / p.p1296);
        let assign54840_e89823: f64 = { let limited_exp_arg = assign54840_e89822; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign54840_e89824: f64 = (assign54840_e89818 * assign54840_e89823);
        let assign54840_e89826: f64 = (assign54840_e89824 + 1.0);
        (assign54840_e89826, (locals.var_t12_dn3 * assign54840_e89823), (locals.var_t12_dn4 * assign54840_e89823), (locals.var_t12_dn5 * assign54840_e89823), (locals.var_t12_dn6 * assign54840_e89823), (locals.var_t12_dn7 * assign54840_e89823), (locals.var_t12_dn8 * assign54840_e89823), (locals.var_t12_dn9 * assign54840_e89823), (locals.var_t12_dn10 * assign54840_e89823), (locals.var_t12_dn11 * assign54840_e89823),)
    } else {
        (locals.var_t12, locals.var_t12_dn3, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn11,)
    }
};
        locals.var_t12 = assign54840_e89828;
        locals.var_t12_dn3 = assign54840_e89828_d_n3;
        locals.var_t12_dn4 = assign54840_e89828_d_n4;
        locals.var_t12_dn5 = assign54840_e89828_d_n5;
        locals.var_t12_dn6 = assign54840_e89828_d_n6;
        locals.var_t12_dn7 = assign54840_e89828_d_n7;
        locals.var_t12_dn8 = assign54840_e89828_d_n8;
        locals.var_t12_dn9 = assign54840_e89828_d_n9;
        locals.var_t12_dn10 = assign54840_e89828_d_n10;
        locals.var_t12_dn11 = assign54840_e89828_d_n11;

        let (assign54850_e89857, assign54850_e89857_d_n3, assign54850_e89857_d_n4, assign54850_e89857_d_n5, assign54850_e89857_d_n6, assign54850_e89857_d_n7, assign54850_e89857_d_n8, assign54850_e89857_d_n9, assign54850_e89857_d_n10, assign54850_e89857_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && ((locals.var_guard837 != 0.0) && (locals.var_guard836 == 0.0))) {
        let assign54850_e89839: f64 = locals.var_t12;
        let assign54850_e89842: f64 = locals.var_t12;
        let assign54850_e89845: f64 = locals.var_t12;
        let assign54850_e89846: f64 = (assign54850_e89842 * assign54850_e89845);
        let assign54850_e89849: f64 = (0.25 * 0.1);
        let assign54850_e89851: f64 = (assign54850_e89849 * 0.1);
        let assign54850_e89852: f64 = (assign54850_e89846 + assign54850_e89851);
        let assign54850_e89853: f64 = (assign54850_e89852).sqrt();
        let assign54850_e89854: f64 = (assign54850_e89839 + assign54850_e89853);
        let assign54850_e89855: f64 = (0.5 * assign54850_e89854);
        (assign54850_e89855, (0.5 * (locals.var_t12_dn3 + (((locals.var_t12_dn3 * assign54850_e89845) + (assign54850_e89842 * locals.var_t12_dn3)) / (2.0 * assign54850_e89853)))), (0.5 * (locals.var_t12_dn4 + (((locals.var_t12_dn4 * assign54850_e89845) + (assign54850_e89842 * locals.var_t12_dn4)) / (2.0 * assign54850_e89853)))), (0.5 * (locals.var_t12_dn5 + (((locals.var_t12_dn5 * assign54850_e89845) + (assign54850_e89842 * locals.var_t12_dn5)) / (2.0 * assign54850_e89853)))), (0.5 * (locals.var_t12_dn6 + (((locals.var_t12_dn6 * assign54850_e89845) + (assign54850_e89842 * locals.var_t12_dn6)) / (2.0 * assign54850_e89853)))), (0.5 * (locals.var_t12_dn7 + (((locals.var_t12_dn7 * assign54850_e89845) + (assign54850_e89842 * locals.var_t12_dn7)) / (2.0 * assign54850_e89853)))), (0.5 * (locals.var_t12_dn8 + (((locals.var_t12_dn8 * assign54850_e89845) + (assign54850_e89842 * locals.var_t12_dn8)) / (2.0 * assign54850_e89853)))), (0.5 * (locals.var_t12_dn9 + (((locals.var_t12_dn9 * assign54850_e89845) + (assign54850_e89842 * locals.var_t12_dn9)) / (2.0 * assign54850_e89853)))), (0.5 * (locals.var_t12_dn10 + (((locals.var_t12_dn10 * assign54850_e89845) + (assign54850_e89842 * locals.var_t12_dn10)) / (2.0 * assign54850_e89853)))), (0.5 * (locals.var_t12_dn11 + (((locals.var_t12_dn11 * assign54850_e89845) + (assign54850_e89842 * locals.var_t12_dn11)) / (2.0 * assign54850_e89853)))),)
    } else {
        (locals.var_t12, locals.var_t12_dn3, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn11,)
    }
};
        locals.var_t12 = assign54850_e89857;
        locals.var_t12_dn3 = assign54850_e89857_d_n3;
        locals.var_t12_dn4 = assign54850_e89857_d_n4;
        locals.var_t12_dn5 = assign54850_e89857_d_n5;
        locals.var_t12_dn6 = assign54850_e89857_d_n6;
        locals.var_t12_dn7 = assign54850_e89857_d_n7;
        locals.var_t12_dn8 = assign54850_e89857_d_n8;
        locals.var_t12_dn9 = assign54850_e89857_d_n9;
        locals.var_t12_dn10 = assign54850_e89857_d_n10;
        locals.var_t12_dn11 = assign54850_e89857_d_n11;

        let (assign54860_e89885, assign54860_e89885_d_n3, assign54860_e89885_d_n4, assign54860_e89885_d_n5, assign54860_e89885_d_n6, assign54860_e89885_d_n7, assign54860_e89885_d_n8, assign54860_e89885_d_n9, assign54860_e89885_d_n10, assign54860_e89885_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && ((locals.var_guard837 != 0.0) && (locals.var_guard836 == 0.0))) {
        let assign54860_e89867: f64 = (locals.var_t0 * p.p2);
        let assign54860_e89869: f64 = (assign54860_e89867 * locals.var_weff);
        let assign54860_e89871: f64 = (assign54860_e89869 / locals.var_lvsat);
        let assign54860_e89874: f64 = (locals.var_t1 * locals.var_t12);
        let assign54860_e89877: f64 = (locals.var_t7 * locals.var_betanoisq);
        let assign54860_e89880: f64 = (12.0 * locals.var_t3);
        let assign54860_e89881: f64 = (assign54860_e89877 / assign54860_e89880);
        let assign54860_e89882: f64 = (assign54860_e89874 + assign54860_e89881);
        let assign54860_e89883: f64 = (assign54860_e89871 * assign54860_e89882);
        (assign54860_e89883, (((((((locals.var_t0_dn3 * p.p2) * locals.var_weff) * locals.var_lvsat) - (assign54860_e89869 * locals.var_lvsat_dn3)) / (locals.var_lvsat * locals.var_lvsat)) * assign54860_e89882) + (assign54860_e89871 * (((locals.var_t1_dn3 * locals.var_t12) + (locals.var_t1 * locals.var_t12_dn3)) + (((((locals.var_t7_dn3 * locals.var_betanoisq) + (locals.var_t7 * locals.var_betanoisq_dn3)) * assign54860_e89880) - (assign54860_e89877 * (12.0 * locals.var_t3_dn3))) / (assign54860_e89880 * assign54860_e89880))))), (((((((locals.var_t0_dn4 * p.p2) * locals.var_weff) * locals.var_lvsat) - (assign54860_e89869 * locals.var_lvsat_dn4)) / (locals.var_lvsat * locals.var_lvsat)) * assign54860_e89882) + (assign54860_e89871 * (((locals.var_t1_dn4 * locals.var_t12) + (locals.var_t1 * locals.var_t12_dn4)) + (((((locals.var_t7_dn4 * locals.var_betanoisq) + (locals.var_t7 * locals.var_betanoisq_dn4)) * assign54860_e89880) - (assign54860_e89877 * (12.0 * locals.var_t3_dn4))) / (assign54860_e89880 * assign54860_e89880))))), (((((((locals.var_t0_dn5 * p.p2) * locals.var_weff) * locals.var_lvsat) - (assign54860_e89869 * locals.var_lvsat_dn5)) / (locals.var_lvsat * locals.var_lvsat)) * assign54860_e89882) + (assign54860_e89871 * (((locals.var_t1_dn5 * locals.var_t12) + (locals.var_t1 * locals.var_t12_dn5)) + (((((locals.var_t7_dn5 * locals.var_betanoisq) + (locals.var_t7 * locals.var_betanoisq_dn5)) * assign54860_e89880) - (assign54860_e89877 * (12.0 * locals.var_t3_dn5))) / (assign54860_e89880 * assign54860_e89880))))), (((((((locals.var_t0_dn6 * p.p2) * locals.var_weff) * locals.var_lvsat) - (assign54860_e89869 * locals.var_lvsat_dn6)) / (locals.var_lvsat * locals.var_lvsat)) * assign54860_e89882) + (assign54860_e89871 * (((locals.var_t1_dn6 * locals.var_t12) + (locals.var_t1 * locals.var_t12_dn6)) + (((((locals.var_t7_dn6 * locals.var_betanoisq) + (locals.var_t7 * locals.var_betanoisq_dn6)) * assign54860_e89880) - (assign54860_e89877 * (12.0 * locals.var_t3_dn6))) / (assign54860_e89880 * assign54860_e89880))))), (((((((locals.var_t0_dn7 * p.p2) * locals.var_weff) * locals.var_lvsat) - (assign54860_e89869 * locals.var_lvsat_dn7)) / (locals.var_lvsat * locals.var_lvsat)) * assign54860_e89882) + (assign54860_e89871 * (((locals.var_t1_dn7 * locals.var_t12) + (locals.var_t1 * locals.var_t12_dn7)) + (((((locals.var_t7_dn7 * locals.var_betanoisq) + (locals.var_t7 * locals.var_betanoisq_dn7)) * assign54860_e89880) - (assign54860_e89877 * (12.0 * locals.var_t3_dn7))) / (assign54860_e89880 * assign54860_e89880))))), (((((((locals.var_t0_dn8 * p.p2) * locals.var_weff) * locals.var_lvsat) - (assign54860_e89869 * locals.var_lvsat_dn8)) / (locals.var_lvsat * locals.var_lvsat)) * assign54860_e89882) + (assign54860_e89871 * (((locals.var_t1_dn8 * locals.var_t12) + (locals.var_t1 * locals.var_t12_dn8)) + (((((locals.var_t7_dn8 * locals.var_betanoisq) + (locals.var_t7 * locals.var_betanoisq_dn8)) * assign54860_e89880) - (assign54860_e89877 * (12.0 * locals.var_t3_dn8))) / (assign54860_e89880 * assign54860_e89880))))), (((((((locals.var_t0_dn9 * p.p2) * locals.var_weff) * locals.var_lvsat) - (assign54860_e89869 * locals.var_lvsat_dn9)) / (locals.var_lvsat * locals.var_lvsat)) * assign54860_e89882) + (assign54860_e89871 * (((locals.var_t1_dn9 * locals.var_t12) + (locals.var_t1 * locals.var_t12_dn9)) + (((((locals.var_t7_dn9 * locals.var_betanoisq) + (locals.var_t7 * locals.var_betanoisq_dn9)) * assign54860_e89880) - (assign54860_e89877 * (12.0 * locals.var_t3_dn9))) / (assign54860_e89880 * assign54860_e89880))))), (((((((locals.var_t0_dn10 * p.p2) * locals.var_weff) * locals.var_lvsat) - (assign54860_e89869 * locals.var_lvsat_dn10)) / (locals.var_lvsat * locals.var_lvsat)) * assign54860_e89882) + (assign54860_e89871 * (((locals.var_t1_dn10 * locals.var_t12) + (locals.var_t1 * locals.var_t12_dn10)) + (((((locals.var_t7_dn10 * locals.var_betanoisq) + (locals.var_t7 * locals.var_betanoisq_dn10)) * assign54860_e89880) - (assign54860_e89877 * (12.0 * locals.var_t3_dn10))) / (assign54860_e89880 * assign54860_e89880))))), (((((((locals.var_t0_dn11 * p.p2) * locals.var_weff) * locals.var_lvsat) - (assign54860_e89869 * locals.var_lvsat_dn11)) / (locals.var_lvsat * locals.var_lvsat)) * assign54860_e89882) + (assign54860_e89871 * (((locals.var_t1_dn11 * locals.var_t12) + (locals.var_t1 * locals.var_t12_dn11)) + (((((locals.var_t7_dn11 * locals.var_betanoisq) + (locals.var_t7 * locals.var_betanoisq_dn11)) * assign54860_e89880) - (assign54860_e89877 * (12.0 * locals.var_t3_dn11))) / (assign54860_e89880 * assign54860_e89880))))),)
    } else {
        (locals.var_mid, locals.var_mid_dn3, locals.var_mid_dn4, locals.var_mid_dn5, locals.var_mid_dn6, locals.var_mid_dn7, locals.var_mid_dn8, locals.var_mid_dn9, locals.var_mid_dn10, locals.var_mid_dn11,)
    }
};
        locals.var_mid = assign54860_e89885;
        locals.var_mid_dn3 = assign54860_e89885_d_n3;
        locals.var_mid_dn4 = assign54860_e89885_d_n4;
        locals.var_mid_dn5 = assign54860_e89885_d_n5;
        locals.var_mid_dn6 = assign54860_e89885_d_n6;
        locals.var_mid_dn7 = assign54860_e89885_d_n7;
        locals.var_mid_dn8 = assign54860_e89885_d_n8;
        locals.var_mid_dn9 = assign54860_e89885_d_n9;
        locals.var_mid_dn10 = assign54860_e89885_d_n10;
        locals.var_mid_dn11 = assign54860_e89885_d_n11;

        let (assign54870_e89935, assign54870_e89935_d_n3, assign54870_e89935_d_n4, assign54870_e89935_d_n5, assign54870_e89935_d_n6, assign54870_e89935_d_n7, assign54870_e89935_d_n8, assign54870_e89935_d_n9, assign54870_e89935_d_n10, assign54870_e89935_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && ((locals.var_guard837 != 0.0) && (locals.var_guard836 == 0.0))) {
        let assign54870_e89895: f64 = (locals.var_lvsat * locals.var_t10);
        let assign54870_e89897: f64 = (assign54870_e89895 * locals.var_t10);
        let assign54870_e89900: f64 = (locals.var_t1 / locals.var_t4);
        let assign54870_e89904: f64 = (60.0 * locals.var_t4);
        let assign54870_e89906: f64 = (assign54870_e89904 * locals.var_t4);
        let assign54870_e89907: f64 = (locals.var_t9 / assign54870_e89906);
        let assign54870_e89908: f64 = (assign54870_e89900 - assign54870_e89907);
        let assign54870_e89911: f64 = (locals.var_t7 * locals.var_t7);
        let assign54870_e89914: f64 = (144.0 * locals.var_t4);
        let assign54870_e89916: f64 = (assign54870_e89914 * locals.var_t5);
        let assign54870_e89917: f64 = (assign54870_e89911 / assign54870_e89916);
        let assign54870_e89918: f64 = (assign54870_e89908 + assign54870_e89917);
        let assign54870_e89919: f64 = (assign54870_e89897 * assign54870_e89918);
        let assign54870_e89921: f64 = (assign54870_e89919 * 15.0);
        let assign54870_e89923: f64 = (assign54870_e89921 / 4.0);
        let assign54870_e89925: f64 = (assign54870_e89923 * locals.var_thetanoisq);
        let assign54870_e89928: f64 = (p.p2 * locals.var_weff);
        let assign54870_e89930: f64 = (assign54870_e89928 * 12.0);
        let assign54870_e89932: f64 = (assign54870_e89930 * locals.var_t0);
        let assign54870_e89933: f64 = (assign54870_e89925 / assign54870_e89932);
        (assign54870_e89933, (((((((((((((locals.var_lvsat_dn3 * locals.var_t10) + (locals.var_lvsat * locals.var_t10_dn3)) * locals.var_t10) + (assign54870_e89895 * locals.var_t10_dn3)) * assign54870_e89918) + (assign54870_e89897 * (((((locals.var_t1_dn3 * locals.var_t4) - (locals.var_t1 * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4)) - (((locals.var_t9_dn3 * assign54870_e89906) - (locals.var_t9 * (((60.0 * locals.var_t4_dn3) * locals.var_t4) + (assign54870_e89904 * locals.var_t4_dn3)))) / (assign54870_e89906 * assign54870_e89906))) + (((((locals.var_t7_dn3 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn3)) * assign54870_e89916) - (assign54870_e89911 * (((144.0 * locals.var_t4_dn3) * locals.var_t5) + (assign54870_e89914 * locals.var_t5_dn3)))) / (assign54870_e89916 * assign54870_e89916))))) * 15.0) / 4.0) * locals.var_thetanoisq) + (assign54870_e89923 * locals.var_thetanoisq_dn3)) * assign54870_e89932) - (assign54870_e89925 * (assign54870_e89930 * locals.var_t0_dn3))) / (assign54870_e89932 * assign54870_e89932)), (((((((((((((locals.var_lvsat_dn4 * locals.var_t10) + (locals.var_lvsat * locals.var_t10_dn4)) * locals.var_t10) + (assign54870_e89895 * locals.var_t10_dn4)) * assign54870_e89918) + (assign54870_e89897 * (((((locals.var_t1_dn4 * locals.var_t4) - (locals.var_t1 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)) - (((locals.var_t9_dn4 * assign54870_e89906) - (locals.var_t9 * (((60.0 * locals.var_t4_dn4) * locals.var_t4) + (assign54870_e89904 * locals.var_t4_dn4)))) / (assign54870_e89906 * assign54870_e89906))) + (((((locals.var_t7_dn4 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn4)) * assign54870_e89916) - (assign54870_e89911 * (((144.0 * locals.var_t4_dn4) * locals.var_t5) + (assign54870_e89914 * locals.var_t5_dn4)))) / (assign54870_e89916 * assign54870_e89916))))) * 15.0) / 4.0) * locals.var_thetanoisq) + (assign54870_e89923 * locals.var_thetanoisq_dn4)) * assign54870_e89932) - (assign54870_e89925 * (assign54870_e89930 * locals.var_t0_dn4))) / (assign54870_e89932 * assign54870_e89932)), (((((((((((((locals.var_lvsat_dn5 * locals.var_t10) + (locals.var_lvsat * locals.var_t10_dn5)) * locals.var_t10) + (assign54870_e89895 * locals.var_t10_dn5)) * assign54870_e89918) + (assign54870_e89897 * (((((locals.var_t1_dn5 * locals.var_t4) - (locals.var_t1 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)) - (((locals.var_t9_dn5 * assign54870_e89906) - (locals.var_t9 * (((60.0 * locals.var_t4_dn5) * locals.var_t4) + (assign54870_e89904 * locals.var_t4_dn5)))) / (assign54870_e89906 * assign54870_e89906))) + (((((locals.var_t7_dn5 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn5)) * assign54870_e89916) - (assign54870_e89911 * (((144.0 * locals.var_t4_dn5) * locals.var_t5) + (assign54870_e89914 * locals.var_t5_dn5)))) / (assign54870_e89916 * assign54870_e89916))))) * 15.0) / 4.0) * locals.var_thetanoisq) + (assign54870_e89923 * locals.var_thetanoisq_dn5)) * assign54870_e89932) - (assign54870_e89925 * (assign54870_e89930 * locals.var_t0_dn5))) / (assign54870_e89932 * assign54870_e89932)), (((((((((((((locals.var_lvsat_dn6 * locals.var_t10) + (locals.var_lvsat * locals.var_t10_dn6)) * locals.var_t10) + (assign54870_e89895 * locals.var_t10_dn6)) * assign54870_e89918) + (assign54870_e89897 * (((((locals.var_t1_dn6 * locals.var_t4) - (locals.var_t1 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)) - (((locals.var_t9_dn6 * assign54870_e89906) - (locals.var_t9 * (((60.0 * locals.var_t4_dn6) * locals.var_t4) + (assign54870_e89904 * locals.var_t4_dn6)))) / (assign54870_e89906 * assign54870_e89906))) + (((((locals.var_t7_dn6 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn6)) * assign54870_e89916) - (assign54870_e89911 * (((144.0 * locals.var_t4_dn6) * locals.var_t5) + (assign54870_e89914 * locals.var_t5_dn6)))) / (assign54870_e89916 * assign54870_e89916))))) * 15.0) / 4.0) * locals.var_thetanoisq) + (assign54870_e89923 * locals.var_thetanoisq_dn6)) * assign54870_e89932) - (assign54870_e89925 * (assign54870_e89930 * locals.var_t0_dn6))) / (assign54870_e89932 * assign54870_e89932)), (((((((((((((locals.var_lvsat_dn7 * locals.var_t10) + (locals.var_lvsat * locals.var_t10_dn7)) * locals.var_t10) + (assign54870_e89895 * locals.var_t10_dn7)) * assign54870_e89918) + (assign54870_e89897 * (((((locals.var_t1_dn7 * locals.var_t4) - (locals.var_t1 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)) - (((locals.var_t9_dn7 * assign54870_e89906) - (locals.var_t9 * (((60.0 * locals.var_t4_dn7) * locals.var_t4) + (assign54870_e89904 * locals.var_t4_dn7)))) / (assign54870_e89906 * assign54870_e89906))) + (((((locals.var_t7_dn7 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn7)) * assign54870_e89916) - (assign54870_e89911 * (((144.0 * locals.var_t4_dn7) * locals.var_t5) + (assign54870_e89914 * locals.var_t5_dn7)))) / (assign54870_e89916 * assign54870_e89916))))) * 15.0) / 4.0) * locals.var_thetanoisq) + (assign54870_e89923 * locals.var_thetanoisq_dn7)) * assign54870_e89932) - (assign54870_e89925 * (assign54870_e89930 * locals.var_t0_dn7))) / (assign54870_e89932 * assign54870_e89932)), (((((((((((((locals.var_lvsat_dn8 * locals.var_t10) + (locals.var_lvsat * locals.var_t10_dn8)) * locals.var_t10) + (assign54870_e89895 * locals.var_t10_dn8)) * assign54870_e89918) + (assign54870_e89897 * (((((locals.var_t1_dn8 * locals.var_t4) - (locals.var_t1 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)) - (((locals.var_t9_dn8 * assign54870_e89906) - (locals.var_t9 * (((60.0 * locals.var_t4_dn8) * locals.var_t4) + (assign54870_e89904 * locals.var_t4_dn8)))) / (assign54870_e89906 * assign54870_e89906))) + (((((locals.var_t7_dn8 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn8)) * assign54870_e89916) - (assign54870_e89911 * (((144.0 * locals.var_t4_dn8) * locals.var_t5) + (assign54870_e89914 * locals.var_t5_dn8)))) / (assign54870_e89916 * assign54870_e89916))))) * 15.0) / 4.0) * locals.var_thetanoisq) + (assign54870_e89923 * locals.var_thetanoisq_dn8)) * assign54870_e89932) - (assign54870_e89925 * (assign54870_e89930 * locals.var_t0_dn8))) / (assign54870_e89932 * assign54870_e89932)), (((((((((((((locals.var_lvsat_dn9 * locals.var_t10) + (locals.var_lvsat * locals.var_t10_dn9)) * locals.var_t10) + (assign54870_e89895 * locals.var_t10_dn9)) * assign54870_e89918) + (assign54870_e89897 * (((((locals.var_t1_dn9 * locals.var_t4) - (locals.var_t1 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)) - (((locals.var_t9_dn9 * assign54870_e89906) - (locals.var_t9 * (((60.0 * locals.var_t4_dn9) * locals.var_t4) + (assign54870_e89904 * locals.var_t4_dn9)))) / (assign54870_e89906 * assign54870_e89906))) + (((((locals.var_t7_dn9 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn9)) * assign54870_e89916) - (assign54870_e89911 * (((144.0 * locals.var_t4_dn9) * locals.var_t5) + (assign54870_e89914 * locals.var_t5_dn9)))) / (assign54870_e89916 * assign54870_e89916))))) * 15.0) / 4.0) * locals.var_thetanoisq) + (assign54870_e89923 * locals.var_thetanoisq_dn9)) * assign54870_e89932) - (assign54870_e89925 * (assign54870_e89930 * locals.var_t0_dn9))) / (assign54870_e89932 * assign54870_e89932)), (((((((((((((locals.var_lvsat_dn10 * locals.var_t10) + (locals.var_lvsat * locals.var_t10_dn10)) * locals.var_t10) + (assign54870_e89895 * locals.var_t10_dn10)) * assign54870_e89918) + (assign54870_e89897 * (((((locals.var_t1_dn10 * locals.var_t4) - (locals.var_t1 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)) - (((locals.var_t9_dn10 * assign54870_e89906) - (locals.var_t9 * (((60.0 * locals.var_t4_dn10) * locals.var_t4) + (assign54870_e89904 * locals.var_t4_dn10)))) / (assign54870_e89906 * assign54870_e89906))) + (((((locals.var_t7_dn10 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn10)) * assign54870_e89916) - (assign54870_e89911 * (((144.0 * locals.var_t4_dn10) * locals.var_t5) + (assign54870_e89914 * locals.var_t5_dn10)))) / (assign54870_e89916 * assign54870_e89916))))) * 15.0) / 4.0) * locals.var_thetanoisq) + (assign54870_e89923 * locals.var_thetanoisq_dn10)) * assign54870_e89932) - (assign54870_e89925 * (assign54870_e89930 * locals.var_t0_dn10))) / (assign54870_e89932 * assign54870_e89932)), (((((((((((((locals.var_lvsat_dn11 * locals.var_t10) + (locals.var_lvsat * locals.var_t10_dn11)) * locals.var_t10) + (assign54870_e89895 * locals.var_t10_dn11)) * assign54870_e89918) + (assign54870_e89897 * (((((locals.var_t1_dn11 * locals.var_t4) - (locals.var_t1 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)) - (((locals.var_t9_dn11 * assign54870_e89906) - (locals.var_t9 * (((60.0 * locals.var_t4_dn11) * locals.var_t4) + (assign54870_e89904 * locals.var_t4_dn11)))) / (assign54870_e89906 * assign54870_e89906))) + (((((locals.var_t7_dn11 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn11)) * assign54870_e89916) - (assign54870_e89911 * (((144.0 * locals.var_t4_dn11) * locals.var_t5) + (assign54870_e89914 * locals.var_t5_dn11)))) / (assign54870_e89916 * assign54870_e89916))))) * 15.0) / 4.0) * locals.var_thetanoisq) + (assign54870_e89923 * locals.var_thetanoisq_dn11)) * assign54870_e89932) - (assign54870_e89925 * (assign54870_e89930 * locals.var_t0_dn11))) / (assign54870_e89932 * assign54870_e89932)),)
    } else {
        (locals.var_mig, locals.var_mig_dn3, locals.var_mig_dn4, locals.var_mig_dn5, locals.var_mig_dn6, locals.var_mig_dn7, locals.var_mig_dn8, locals.var_mig_dn9, locals.var_mig_dn10, locals.var_mig_dn11,)
    }
};
        locals.var_mig = assign54870_e89935;
        locals.var_mig_dn3 = assign54870_e89935_d_n3;
        locals.var_mig_dn4 = assign54870_e89935_d_n4;
        locals.var_mig_dn5 = assign54870_e89935_d_n5;
        locals.var_mig_dn6 = assign54870_e89935_d_n6;
        locals.var_mig_dn7 = assign54870_e89935_d_n7;
        locals.var_mig_dn8 = assign54870_e89935_d_n8;
        locals.var_mig_dn9 = assign54870_e89935_d_n9;
        locals.var_mig_dn10 = assign54870_e89935_d_n10;
        locals.var_mig_dn11 = assign54870_e89935_d_n11;

        let (assign54890_e89974, assign54890_e89974_d_n3, assign54890_e89974_d_n4, assign54890_e89974_d_n5, assign54890_e89974_d_n6, assign54890_e89974_d_n7, assign54890_e89974_d_n8, assign54890_e89974_d_n9, assign54890_e89974_d_n10, assign54890_e89974_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && ((locals.var_guard837 != 0.0) && (locals.var_guard836 == 0.0))) {
        let assign54890_e89971: f64 = (locals.var_nt * locals.var_mid);
        let assign54890_e89972: f64 = (assign54890_e89971).sqrt();
        (assign54890_e89972, ((locals.var_nt * locals.var_mid_dn3) / (2.0 * assign54890_e89972)), (((locals.var_nt_dn4 * locals.var_mid) + (locals.var_nt * locals.var_mid_dn4)) / (2.0 * assign54890_e89972)), (((locals.var_nt_dn5 * locals.var_mid) + (locals.var_nt * locals.var_mid_dn5)) / (2.0 * assign54890_e89972)), ((locals.var_nt * locals.var_mid_dn6) / (2.0 * assign54890_e89972)), ((locals.var_nt * locals.var_mid_dn7) / (2.0 * assign54890_e89972)), ((locals.var_nt * locals.var_mid_dn8) / (2.0 * assign54890_e89972)), ((locals.var_nt * locals.var_mid_dn9) / (2.0 * assign54890_e89972)), ((locals.var_nt * locals.var_mid_dn10) / (2.0 * assign54890_e89972)), ((locals.var_nt * locals.var_mid_dn11) / (2.0 * assign54890_e89972)),)
    } else {
        (locals.var_sqid, locals.var_sqid_dn3, locals.var_sqid_dn4, locals.var_sqid_dn5, locals.var_sqid_dn6, locals.var_sqid_dn7, locals.var_sqid_dn8, locals.var_sqid_dn9, locals.var_sqid_dn10, locals.var_sqid_dn11,)
    }
};
        locals.var_sqid = assign54890_e89974;
        locals.var_sqid_dn3 = assign54890_e89974_d_n3;
        locals.var_sqid_dn4 = assign54890_e89974_d_n4;
        locals.var_sqid_dn5 = assign54890_e89974_d_n5;
        locals.var_sqid_dn6 = assign54890_e89974_d_n6;
        locals.var_sqid_dn7 = assign54890_e89974_d_n7;
        locals.var_sqid_dn8 = assign54890_e89974_d_n8;
        locals.var_sqid_dn9 = assign54890_e89974_d_n9;
        locals.var_sqid_dn10 = assign54890_e89974_d_n10;
        locals.var_sqid_dn11 = assign54890_e89974_d_n11;

        let assign54900_e89977: f64 = if locals.var_mig > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard838 = assign54900_e89977;

        let (assign54910_e89992, assign54910_e89992_d_n3, assign54910_e89992_d_n4, assign54910_e89992_d_n5, assign54910_e89992_d_n6, assign54910_e89992_d_n7, assign54910_e89992_d_n8, assign54910_e89992_d_n9, assign54910_e89992_d_n10, assign54910_e89992_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && ((locals.var_guard837 != 0.0) && (locals.var_guard836 == 0.0))) && (locals.var_guard838 != 0.0)) {
        let assign54910_e89989: f64 = (locals.var_nt / locals.var_mig);
        let assign54910_e89990: f64 = (assign54910_e89989).sqrt();
        (assign54910_e89990, ((-((locals.var_nt * locals.var_mig_dn3) / (locals.var_mig * locals.var_mig))) / (2.0 * assign54910_e89990)), ((((locals.var_nt_dn4 * locals.var_mig) - (locals.var_nt * locals.var_mig_dn4)) / (locals.var_mig * locals.var_mig)) / (2.0 * assign54910_e89990)), ((((locals.var_nt_dn5 * locals.var_mig) - (locals.var_nt * locals.var_mig_dn5)) / (locals.var_mig * locals.var_mig)) / (2.0 * assign54910_e89990)), ((-((locals.var_nt * locals.var_mig_dn6) / (locals.var_mig * locals.var_mig))) / (2.0 * assign54910_e89990)), ((-((locals.var_nt * locals.var_mig_dn7) / (locals.var_mig * locals.var_mig))) / (2.0 * assign54910_e89990)), ((-((locals.var_nt * locals.var_mig_dn8) / (locals.var_mig * locals.var_mig))) / (2.0 * assign54910_e89990)), ((-((locals.var_nt * locals.var_mig_dn9) / (locals.var_mig * locals.var_mig))) / (2.0 * assign54910_e89990)), ((-((locals.var_nt * locals.var_mig_dn10) / (locals.var_mig * locals.var_mig))) / (2.0 * assign54910_e89990)), ((-((locals.var_nt * locals.var_mig_dn11) / (locals.var_mig * locals.var_mig))) / (2.0 * assign54910_e89990)),)
    } else {
        (locals.var_sqig, locals.var_sqig_dn3, locals.var_sqig_dn4, locals.var_sqig_dn5, locals.var_sqig_dn6, locals.var_sqig_dn7, locals.var_sqig_dn8, locals.var_sqig_dn9, locals.var_sqig_dn10, locals.var_sqig_dn11,)
    }
};
        locals.var_sqig = assign54910_e89992;
        locals.var_sqig_dn3 = assign54910_e89992_d_n3;
        locals.var_sqig_dn4 = assign54910_e89992_d_n4;
        locals.var_sqig_dn5 = assign54910_e89992_d_n5;
        locals.var_sqig_dn6 = assign54910_e89992_d_n6;
        locals.var_sqig_dn7 = assign54910_e89992_d_n7;
        locals.var_sqig_dn8 = assign54910_e89992_d_n8;
        locals.var_sqig_dn9 = assign54910_e89992_d_n9;
        locals.var_sqig_dn10 = assign54910_e89992_d_n10;
        locals.var_sqig_dn11 = assign54910_e89992_d_n11;

        let (assign54950_e90041, assign54950_e90041_d_n3, assign54950_e90041_d_n4, assign54950_e90041_d_n5, assign54950_e90041_d_n6, assign54950_e90041_d_n7, assign54950_e90041_d_n8, assign54950_e90041_d_n9, assign54950_e90041_d_n10, assign54950_e90041_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && ((locals.var_guard837 != 0.0) && (locals.var_guard836 == 0.0))) && (locals.var_guard838 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_sqig, locals.var_sqig_dn3, locals.var_sqig_dn4, locals.var_sqig_dn5, locals.var_sqig_dn6, locals.var_sqig_dn7, locals.var_sqig_dn8, locals.var_sqig_dn9, locals.var_sqig_dn10, locals.var_sqig_dn11,)
    }
};
        locals.var_sqig = assign54950_e90041;
        locals.var_sqig_dn3 = assign54950_e90041_d_n3;
        locals.var_sqig_dn4 = assign54950_e90041_d_n4;
        locals.var_sqig_dn5 = assign54950_e90041_d_n5;
        locals.var_sqig_dn6 = assign54950_e90041_d_n6;
        locals.var_sqig_dn7 = assign54950_e90041_d_n7;
        locals.var_sqig_dn8 = assign54950_e90041_d_n8;
        locals.var_sqig_dn9 = assign54950_e90041_d_n9;
        locals.var_sqig_dn10 = assign54950_e90041_d_n10;
        locals.var_sqig_dn11 = assign54950_e90041_d_n11;

        let (assign54990_e90065, assign54990_e90065_d_n3, assign54990_e90065_d_n4, assign54990_e90065_d_n5, assign54990_e90065_d_n6, assign54990_e90065_d_n7, assign54990_e90065_d_n8, assign54990_e90065_d_n9, assign54990_e90065_d_n10, assign54990_e90065_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        (locals.var_vgfb, locals.var_vgfb_dn3, locals.var_vgfb_dn4, locals.var_vgfb_dn5, locals.var_vgfb_dn6, locals.var_vgfb_dn7, locals.var_vgfb_dn8, locals.var_vgfb_dn9, locals.var_vgfb_dn10, locals.var_vgfb_dn11,)
    } else {
        (locals.var_vgfbcv, locals.var_vgfbcv_dn3, locals.var_vgfbcv_dn4, locals.var_vgfbcv_dn5, locals.var_vgfbcv_dn6, locals.var_vgfbcv_dn7, locals.var_vgfbcv_dn8, locals.var_vgfbcv_dn9, locals.var_vgfbcv_dn10, locals.var_vgfbcv_dn11,)
    }
};
        locals.var_vgfbcv = assign54990_e90065;
        locals.var_vgfbcv_dn3 = assign54990_e90065_d_n3;
        locals.var_vgfbcv_dn4 = assign54990_e90065_d_n4;
        locals.var_vgfbcv_dn5 = assign54990_e90065_d_n5;
        locals.var_vgfbcv_dn6 = assign54990_e90065_d_n6;
        locals.var_vgfbcv_dn7 = assign54990_e90065_d_n7;
        locals.var_vgfbcv_dn8 = assign54990_e90065_d_n8;
        locals.var_vgfbcv_dn9 = assign54990_e90065_d_n9;
        locals.var_vgfbcv_dn10 = assign54990_e90065_d_n10;
        locals.var_vgfbcv_dn11 = assign54990_e90065_d_n11;

        let (assign55000_e90070, assign55000_e90070_d_n4, assign55000_e90070_d_n5,) = {
    if (locals.var_guard492 == 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_invgamg2, locals.var_invgamg2_dn4, locals.var_invgamg2_dn5,)
    }
};
        locals.var_invgamg2 = assign55000_e90070;
        locals.var_invgamg2_dn4 = assign55000_e90070_d_n4;
        locals.var_invgamg2_dn5 = assign55000_e90070_d_n5;

        let assign55010_e90073: f64 = if p.p31 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard842 = assign55010_e90073;

        let (assign55020_e90082, assign55020_e90082_d_n3, assign55020_e90082_d_n4, assign55020_e90082_d_n5, assign55020_e90082_d_n6, assign55020_e90082_d_n7, assign55020_e90082_d_n8, assign55020_e90082_d_n9, assign55020_e90082_d_n10, assign55020_e90082_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55020_e90080: f64 = (locals.var_vfbcv_i + p.p25);
        (assign55020_e90080, locals.var_vfbcv_i_dn3, locals.var_vfbcv_i_dn4, locals.var_vfbcv_i_dn5, locals.var_vfbcv_i_dn6, locals.var_vfbcv_i_dn7, locals.var_vfbcv_i_dn8, locals.var_vfbcv_i_dn9, locals.var_vfbcv_i_dn10, locals.var_vfbcv_i_dn11,)
    } else {
        (locals.var_vfbcv_i, locals.var_vfbcv_i_dn3, locals.var_vfbcv_i_dn4, locals.var_vfbcv_i_dn5, locals.var_vfbcv_i_dn6, locals.var_vfbcv_i_dn7, locals.var_vfbcv_i_dn8, locals.var_vfbcv_i_dn9, locals.var_vfbcv_i_dn10, locals.var_vfbcv_i_dn11,)
    }
};
        locals.var_vfbcv_i = assign55020_e90082;
        locals.var_vfbcv_i_dn3 = assign55020_e90082_d_n3;
        locals.var_vfbcv_i_dn4 = assign55020_e90082_d_n4;
        locals.var_vfbcv_i_dn5 = assign55020_e90082_d_n5;
        locals.var_vfbcv_i_dn6 = assign55020_e90082_d_n6;
        locals.var_vfbcv_i_dn7 = assign55020_e90082_d_n7;
        locals.var_vfbcv_i_dn8 = assign55020_e90082_d_n8;
        locals.var_vfbcv_i_dn9 = assign55020_e90082_d_n9;
        locals.var_vfbcv_i_dn10 = assign55020_e90082_d_n10;
        locals.var_vfbcv_i_dn11 = assign55020_e90082_d_n11;

        let (assign55030_e90091, assign55030_e90091_d_n3, assign55030_e90091_d_n4, assign55030_e90091_d_n5, assign55030_e90091_d_n6, assign55030_e90091_d_n7, assign55030_e90091_d_n8, assign55030_e90091_d_n9, assign55030_e90091_d_n10, assign55030_e90091_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55030_e90089: f64 = (locals.var_vg * locals.var_inv_vt);
        (assign55030_e90089, 0.0, (locals.var_vg * locals.var_inv_vt_dn4), (locals.var_vg * locals.var_inv_vt_dn5), 0.0, 0.0, (locals.var_vg_dn8 * locals.var_inv_vt), 0.0, (locals.var_vg_dn10 * locals.var_inv_vt), 0.0,)
    } else {
        (locals.var_vg_1, locals.var_vg_1_dn3, locals.var_vg_1_dn4, locals.var_vg_1_dn5, locals.var_vg_1_dn6, locals.var_vg_1_dn7, locals.var_vg_1_dn8, locals.var_vg_1_dn9, locals.var_vg_1_dn10, locals.var_vg_1_dn11,)
    }
};
        locals.var_vg_1 = assign55030_e90091;
        locals.var_vg_1_dn3 = assign55030_e90091_d_n3;
        locals.var_vg_1_dn4 = assign55030_e90091_d_n4;
        locals.var_vg_1_dn5 = assign55030_e90091_d_n5;
        locals.var_vg_1_dn6 = assign55030_e90091_d_n6;
        locals.var_vg_1_dn7 = assign55030_e90091_d_n7;
        locals.var_vg_1_dn8 = assign55030_e90091_d_n8;
        locals.var_vg_1_dn9 = assign55030_e90091_d_n9;
        locals.var_vg_1_dn10 = assign55030_e90091_d_n10;
        locals.var_vg_1_dn11 = assign55030_e90091_d_n11;

        let (assign55040_e90100, assign55040_e90100_d_n3, assign55040_e90100_d_n4, assign55040_e90100_d_n5, assign55040_e90100_d_n6, assign55040_e90100_d_n7, assign55040_e90100_d_n8, assign55040_e90100_d_n9, assign55040_e90100_d_n10, assign55040_e90100_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55040_e90098: f64 = (locals.var_vs * locals.var_inv_vt);
        (assign55040_e90098, 0.0, (locals.var_vs * locals.var_inv_vt_dn4), (locals.var_vs * locals.var_inv_vt_dn5), (locals.var_vs_dn6 * locals.var_inv_vt), (locals.var_vs_dn7 * locals.var_inv_vt), 0.0, 0.0, (locals.var_vs_dn10 * locals.var_inv_vt), 0.0,)
    } else {
        (locals.var_vs_1, locals.var_vs_1_dn3, locals.var_vs_1_dn4, locals.var_vs_1_dn5, locals.var_vs_1_dn6, locals.var_vs_1_dn7, locals.var_vs_1_dn8, locals.var_vs_1_dn9, locals.var_vs_1_dn10, locals.var_vs_1_dn11,)
    }
};
        locals.var_vs_1 = assign55040_e90100;
        locals.var_vs_1_dn3 = assign55040_e90100_d_n3;
        locals.var_vs_1_dn4 = assign55040_e90100_d_n4;
        locals.var_vs_1_dn5 = assign55040_e90100_d_n5;
        locals.var_vs_1_dn6 = assign55040_e90100_d_n6;
        locals.var_vs_1_dn7 = assign55040_e90100_d_n7;
        locals.var_vs_1_dn8 = assign55040_e90100_d_n8;
        locals.var_vs_1_dn9 = assign55040_e90100_d_n9;
        locals.var_vs_1_dn10 = assign55040_e90100_d_n10;
        locals.var_vs_1_dn11 = assign55040_e90100_d_n11;

    }

    pub(super) fn stamp_transient_block_186(
        locals: &mut StampLocals,
    ) {
        let (assign55050_e90109, assign55050_e90109_d_n3, assign55050_e90109_d_n4, assign55050_e90109_d_n5, assign55050_e90109_d_n6, assign55050_e90109_d_n7, assign55050_e90109_d_n8, assign55050_e90109_d_n9, assign55050_e90109_d_n10, assign55050_e90109_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55050_e90107: f64 = (locals.var_vfbcv_i * locals.var_inv_vt);
        (assign55050_e90107, (locals.var_vfbcv_i_dn3 * locals.var_inv_vt), ((locals.var_vfbcv_i_dn4 * locals.var_inv_vt) + (locals.var_vfbcv_i * locals.var_inv_vt_dn4)), ((locals.var_vfbcv_i_dn5 * locals.var_inv_vt) + (locals.var_vfbcv_i * locals.var_inv_vt_dn5)), (locals.var_vfbcv_i_dn6 * locals.var_inv_vt), (locals.var_vfbcv_i_dn7 * locals.var_inv_vt), (locals.var_vfbcv_i_dn8 * locals.var_inv_vt), (locals.var_vfbcv_i_dn9 * locals.var_inv_vt), (locals.var_vfbcv_i_dn10 * locals.var_inv_vt), (locals.var_vfbcv_i_dn11 * locals.var_inv_vt),)
    } else {
        (locals.var_vfb, locals.var_vfb_dn3, locals.var_vfb_dn4, locals.var_vfb_dn5, locals.var_vfb_dn6, locals.var_vfb_dn7, locals.var_vfb_dn8, locals.var_vfb_dn9, locals.var_vfb_dn10, locals.var_vfb_dn11,)
    }
};
        locals.var_vfb = assign55050_e90109;
        locals.var_vfb_dn3 = assign55050_e90109_d_n3;
        locals.var_vfb_dn4 = assign55050_e90109_d_n4;
        locals.var_vfb_dn5 = assign55050_e90109_d_n5;
        locals.var_vfb_dn6 = assign55050_e90109_d_n6;
        locals.var_vfb_dn7 = assign55050_e90109_d_n7;
        locals.var_vfb_dn8 = assign55050_e90109_d_n8;
        locals.var_vfb_dn9 = assign55050_e90109_d_n9;
        locals.var_vfb_dn10 = assign55050_e90109_d_n10;
        locals.var_vfb_dn11 = assign55050_e90109_d_n11;

        let (assign55060_e90118, assign55060_e90118_d_n3, assign55060_e90118_d_n4, assign55060_e90118_d_n5, assign55060_e90118_d_n6, assign55060_e90118_d_n7, assign55060_e90118_d_n8, assign55060_e90118_d_n9, assign55060_e90118_d_n10, assign55060_e90118_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55060_e90116: f64 = (locals.var_vg_1 - locals.var_vfb);
        (assign55060_e90116, (locals.var_vg_1_dn3 - locals.var_vfb_dn3), (locals.var_vg_1_dn4 - locals.var_vfb_dn4), (locals.var_vg_1_dn5 - locals.var_vfb_dn5), (locals.var_vg_1_dn6 - locals.var_vfb_dn6), (locals.var_vg_1_dn7 - locals.var_vfb_dn7), (locals.var_vg_1_dn8 - locals.var_vfb_dn8), (locals.var_vg_1_dn9 - locals.var_vfb_dn9), (locals.var_vg_1_dn10 - locals.var_vfb_dn10), (locals.var_vg_1_dn11 - locals.var_vfb_dn11),)
    } else {
        (locals.var_vgfbcv, locals.var_vgfbcv_dn3, locals.var_vgfbcv_dn4, locals.var_vgfbcv_dn5, locals.var_vgfbcv_dn6, locals.var_vgfbcv_dn7, locals.var_vgfbcv_dn8, locals.var_vgfbcv_dn9, locals.var_vgfbcv_dn10, locals.var_vgfbcv_dn11,)
    }
};
        locals.var_vgfbcv = assign55060_e90118;
        locals.var_vgfbcv_dn3 = assign55060_e90118_d_n3;
        locals.var_vgfbcv_dn4 = assign55060_e90118_d_n4;
        locals.var_vgfbcv_dn5 = assign55060_e90118_d_n5;
        locals.var_vgfbcv_dn6 = assign55060_e90118_d_n6;
        locals.var_vgfbcv_dn7 = assign55060_e90118_d_n7;
        locals.var_vgfbcv_dn8 = assign55060_e90118_d_n8;
        locals.var_vgfbcv_dn9 = assign55060_e90118_d_n9;
        locals.var_vgfbcv_dn10 = assign55060_e90118_d_n10;
        locals.var_vgfbcv_dn11 = assign55060_e90118_d_n11;

        let (assign55070_e90130, assign55070_e90130_d_n3, assign55070_e90130_d_n4, assign55070_e90130_d_n5, assign55070_e90130_d_n6, assign55070_e90130_d_n7, assign55070_e90130_d_n8, assign55070_e90130_d_n9, assign55070_e90130_d_n10, assign55070_e90130_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55070_e90125: f64 = (locals.var_ndepcv_i / locals.var_ni);
        let assign55070_e90127: f64 = (assign55070_e90125).max(1e-38);
        let assign55070_e90128: f64 = (assign55070_e90127).ln();
        (assign55070_e90128, (if assign55070_e90125 >= 1e-38 { (((locals.var_ndepcv_i_dn3 * locals.var_ni) - (locals.var_ndepcv_i * locals.var_ni_dn3)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign55070_e90127), (if assign55070_e90125 >= 1e-38 { (((locals.var_ndepcv_i_dn4 * locals.var_ni) - (locals.var_ndepcv_i * locals.var_ni_dn4)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign55070_e90127), (if assign55070_e90125 >= 1e-38 { (((locals.var_ndepcv_i_dn5 * locals.var_ni) - (locals.var_ndepcv_i * locals.var_ni_dn5)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign55070_e90127), (if assign55070_e90125 >= 1e-38 { (((locals.var_ndepcv_i_dn6 * locals.var_ni) - (locals.var_ndepcv_i * locals.var_ni_dn6)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign55070_e90127), (if assign55070_e90125 >= 1e-38 { (((locals.var_ndepcv_i_dn7 * locals.var_ni) - (locals.var_ndepcv_i * locals.var_ni_dn7)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign55070_e90127), (if assign55070_e90125 >= 1e-38 { (((locals.var_ndepcv_i_dn8 * locals.var_ni) - (locals.var_ndepcv_i * locals.var_ni_dn8)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign55070_e90127), (if assign55070_e90125 >= 1e-38 { (((locals.var_ndepcv_i_dn9 * locals.var_ni) - (locals.var_ndepcv_i * locals.var_ni_dn9)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign55070_e90127), (if assign55070_e90125 >= 1e-38 { (((locals.var_ndepcv_i_dn10 * locals.var_ni) - (locals.var_ndepcv_i * locals.var_ni_dn10)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign55070_e90127), (if assign55070_e90125 >= 1e-38 { (((locals.var_ndepcv_i_dn11 * locals.var_ni) - (locals.var_ndepcv_i * locals.var_ni_dn11)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign55070_e90127),)
    } else {
        (locals.var_phibcv, locals.var_phibcv_dn3, locals.var_phibcv_dn4, locals.var_phibcv_dn5, locals.var_phibcv_dn6, locals.var_phibcv_dn7, locals.var_phibcv_dn8, locals.var_phibcv_dn9, locals.var_phibcv_dn10, locals.var_phibcv_dn11,)
    }
};
        locals.var_phibcv = assign55070_e90130;
        locals.var_phibcv_dn3 = assign55070_e90130_d_n3;
        locals.var_phibcv_dn4 = assign55070_e90130_d_n4;
        locals.var_phibcv_dn5 = assign55070_e90130_d_n5;
        locals.var_phibcv_dn6 = assign55070_e90130_d_n6;
        locals.var_phibcv_dn7 = assign55070_e90130_d_n7;
        locals.var_phibcv_dn8 = assign55070_e90130_d_n8;
        locals.var_phibcv_dn9 = assign55070_e90130_d_n9;
        locals.var_phibcv_dn10 = assign55070_e90130_d_n10;
        locals.var_phibcv_dn11 = assign55070_e90130_d_n11;

        let (assign55080_e90148, assign55080_e90148_d_n3, assign55080_e90148_d_n4, assign55080_e90148_d_n5, assign55080_e90148_d_n6, assign55080_e90148_d_n7, assign55080_e90148_d_n8, assign55080_e90148_d_n9, assign55080_e90148_d_n10, assign55080_e90148_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55080_e90137: f64 = (2.0 * 1.602176462e-19);
        let assign55080_e90139: f64 = (assign55080_e90137 * locals.var_epssi);
        let assign55080_e90141: f64 = (assign55080_e90139 * locals.var_ndepcv_i);
        let assign55080_e90143: f64 = (assign55080_e90141 * locals.var_inv_vt);
        let assign55080_e90144: f64 = (assign55080_e90143).sqrt();
        let assign55080_e90146: f64 = (assign55080_e90144 / locals.var_cox);
        (assign55080_e90146, ((((assign55080_e90139 * locals.var_ndepcv_i_dn3) * locals.var_inv_vt) / (2.0 * assign55080_e90144)) / locals.var_cox), (((((assign55080_e90139 * locals.var_ndepcv_i_dn4) * locals.var_inv_vt) + (assign55080_e90141 * locals.var_inv_vt_dn4)) / (2.0 * assign55080_e90144)) / locals.var_cox), (((((assign55080_e90139 * locals.var_ndepcv_i_dn5) * locals.var_inv_vt) + (assign55080_e90141 * locals.var_inv_vt_dn5)) / (2.0 * assign55080_e90144)) / locals.var_cox), ((((assign55080_e90139 * locals.var_ndepcv_i_dn6) * locals.var_inv_vt) / (2.0 * assign55080_e90144)) / locals.var_cox), ((((assign55080_e90139 * locals.var_ndepcv_i_dn7) * locals.var_inv_vt) / (2.0 * assign55080_e90144)) / locals.var_cox), ((((assign55080_e90139 * locals.var_ndepcv_i_dn8) * locals.var_inv_vt) / (2.0 * assign55080_e90144)) / locals.var_cox), ((((assign55080_e90139 * locals.var_ndepcv_i_dn9) * locals.var_inv_vt) / (2.0 * assign55080_e90144)) / locals.var_cox), ((((assign55080_e90139 * locals.var_ndepcv_i_dn10) * locals.var_inv_vt) / (2.0 * assign55080_e90144)) / locals.var_cox), ((((assign55080_e90139 * locals.var_ndepcv_i_dn11) * locals.var_inv_vt) / (2.0 * assign55080_e90144)) / locals.var_cox),)
    } else {
        (locals.var_gamcv, locals.var_gamcv_dn3, locals.var_gamcv_dn4, locals.var_gamcv_dn5, locals.var_gamcv_dn6, locals.var_gamcv_dn7, locals.var_gamcv_dn8, locals.var_gamcv_dn9, locals.var_gamcv_dn10, locals.var_gamcv_dn11,)
    }
};
        locals.var_gamcv = assign55080_e90148;
        locals.var_gamcv_dn3 = assign55080_e90148_d_n3;
        locals.var_gamcv_dn4 = assign55080_e90148_d_n4;
        locals.var_gamcv_dn5 = assign55080_e90148_d_n5;
        locals.var_gamcv_dn6 = assign55080_e90148_d_n6;
        locals.var_gamcv_dn7 = assign55080_e90148_d_n7;
        locals.var_gamcv_dn8 = assign55080_e90148_d_n8;
        locals.var_gamcv_dn9 = assign55080_e90148_d_n9;
        locals.var_gamcv_dn10 = assign55080_e90148_d_n10;
        locals.var_gamcv_dn11 = assign55080_e90148_d_n11;

        let (assign55090_e90157, assign55090_e90157_d_n3, assign55090_e90157_d_n4, assign55090_e90157_d_n5, assign55090_e90157_d_n6, assign55090_e90157_d_n7, assign55090_e90157_d_n8, assign55090_e90157_d_n9, assign55090_e90157_d_n10, assign55090_e90157_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55090_e90155: f64 = (1.0 / locals.var_gamcv);
        (assign55090_e90155, (-(locals.var_gamcv_dn3 / (locals.var_gamcv * locals.var_gamcv))), (-(locals.var_gamcv_dn4 / (locals.var_gamcv * locals.var_gamcv))), (-(locals.var_gamcv_dn5 / (locals.var_gamcv * locals.var_gamcv))), (-(locals.var_gamcv_dn6 / (locals.var_gamcv * locals.var_gamcv))), (-(locals.var_gamcv_dn7 / (locals.var_gamcv * locals.var_gamcv))), (-(locals.var_gamcv_dn8 / (locals.var_gamcv * locals.var_gamcv))), (-(locals.var_gamcv_dn9 / (locals.var_gamcv * locals.var_gamcv))), (-(locals.var_gamcv_dn10 / (locals.var_gamcv * locals.var_gamcv))), (-(locals.var_gamcv_dn11 / (locals.var_gamcv * locals.var_gamcv))),)
    } else {
        (locals.var_inv_gam, locals.var_inv_gam_dn3, locals.var_inv_gam_dn4, locals.var_inv_gam_dn5, locals.var_inv_gam_dn6, locals.var_inv_gam_dn7, locals.var_inv_gam_dn8, locals.var_inv_gam_dn9, locals.var_inv_gam_dn10, locals.var_inv_gam_dn11,)
    }
};
        locals.var_inv_gam = assign55090_e90157;
        locals.var_inv_gam_dn3 = assign55090_e90157_d_n3;
        locals.var_inv_gam_dn4 = assign55090_e90157_d_n4;
        locals.var_inv_gam_dn5 = assign55090_e90157_d_n5;
        locals.var_inv_gam_dn6 = assign55090_e90157_d_n6;
        locals.var_inv_gam_dn7 = assign55090_e90157_d_n7;
        locals.var_inv_gam_dn8 = assign55090_e90157_d_n8;
        locals.var_inv_gam_dn9 = assign55090_e90157_d_n9;
        locals.var_inv_gam_dn10 = assign55090_e90157_d_n10;
        locals.var_inv_gam_dn11 = assign55090_e90157_d_n11;

        let (assign55100_e90176, assign55100_e90176_d_n4, assign55100_e90176_d_n5,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55100_e90164: f64 = (2.0 * 1.602176462e-19);
        let assign55100_e90166: f64 = (assign55100_e90164 * locals.var_epssi);
        let assign55100_e90168: f64 = (assign55100_e90166 * locals.var_ngate_i);
        let assign55100_e90171: f64 = (locals.var_cox * locals.var_cox);
        let assign55100_e90173: f64 = (assign55100_e90171 * locals.var_vt);
        let assign55100_e90174: f64 = (assign55100_e90168 / assign55100_e90173);
        (assign55100_e90174, (-((assign55100_e90168 * (assign55100_e90171 * locals.var_vt_dn4)) / (assign55100_e90173 * assign55100_e90173))), (-((assign55100_e90168 * (assign55100_e90171 * locals.var_vt_dn5)) / (assign55100_e90173 * assign55100_e90173))),)
    } else {
        (locals.var_gamg2, locals.var_gamg2_dn4, locals.var_gamg2_dn5,)
    }
};
        locals.var_gamg2 = assign55100_e90176;
        locals.var_gamg2_dn4 = assign55100_e90176_d_n4;
        locals.var_gamg2_dn5 = assign55100_e90176_d_n5;

        let (assign55110_e90190, assign55110_e90190_d_n4, assign55110_e90190_d_n5,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let (assign55110_e90188, assign55110_e90188_d_n4, assign55110_e90188_d_n5,) = {
            if (locals.var_ngate_i > 0.0) {
                let assign55110_e90186: f64 = (1.0 / locals.var_gamg2);
                (assign55110_e90186, (-(locals.var_gamg2_dn4 / (locals.var_gamg2 * locals.var_gamg2))), (-(locals.var_gamg2_dn5 / (locals.var_gamg2 * locals.var_gamg2))),)
            } else {
                (0.0, 0.0, 0.0,)
            }
        };
        (assign55110_e90188, assign55110_e90188_d_n4, assign55110_e90188_d_n5,)
    } else {
        (locals.var_invgamg2, locals.var_invgamg2_dn4, locals.var_invgamg2_dn5,)
    }
};
        locals.var_invgamg2 = assign55110_e90190;
        locals.var_invgamg2_dn4 = assign55110_e90190_d_n4;
        locals.var_invgamg2_dn5 = assign55110_e90190_d_n5;

        let (assign55120_e90204, assign55120_e90204_d_n3, assign55120_e90204_d_n4, assign55120_e90204_d_n5, assign55120_e90204_d_n6, assign55120_e90204_d_n7, assign55120_e90204_d_n8, assign55120_e90204_d_n9, assign55120_e90204_d_n10, assign55120_e90204_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let (assign55120_e90202, assign55120_e90202_d_n3, assign55120_e90202_d_n4, assign55120_e90202_d_n5, assign55120_e90202_d_n6, assign55120_e90202_d_n7, assign55120_e90202_d_n8, assign55120_e90202_d_n9, assign55120_e90202_d_n10, assign55120_e90202_d_n11,) = {
            if (locals.var_ngate_i > 0.0) {
                let assign55120_e90200: f64 = (locals.var_ndepcv_i / locals.var_ngate_i);
                (assign55120_e90200, (locals.var_ndepcv_i_dn3 / locals.var_ngate_i), (locals.var_ndepcv_i_dn4 / locals.var_ngate_i), (locals.var_ndepcv_i_dn5 / locals.var_ngate_i), (locals.var_ndepcv_i_dn6 / locals.var_ngate_i), (locals.var_ndepcv_i_dn7 / locals.var_ngate_i), (locals.var_ndepcv_i_dn8 / locals.var_ngate_i), (locals.var_ndepcv_i_dn9 / locals.var_ngate_i), (locals.var_ndepcv_i_dn10 / locals.var_ngate_i), (locals.var_ndepcv_i_dn11 / locals.var_ngate_i),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign55120_e90202, assign55120_e90202_d_n3, assign55120_e90202_d_n4, assign55120_e90202_d_n5, assign55120_e90202_d_n6, assign55120_e90202_d_n7, assign55120_e90202_d_n8, assign55120_e90202_d_n9, assign55120_e90202_d_n10, assign55120_e90202_d_n11,)
    } else {
        (locals.var_dpd, locals.var_dpd_dn3, locals.var_dpd_dn4, locals.var_dpd_dn5, locals.var_dpd_dn6, locals.var_dpd_dn7, locals.var_dpd_dn8, locals.var_dpd_dn9, locals.var_dpd_dn10, locals.var_dpd_dn11,)
    }
};
        locals.var_dpd = assign55120_e90204;
        locals.var_dpd_dn3 = assign55120_e90204_d_n3;
        locals.var_dpd_dn4 = assign55120_e90204_d_n4;
        locals.var_dpd_dn5 = assign55120_e90204_d_n5;
        locals.var_dpd_dn6 = assign55120_e90204_d_n6;
        locals.var_dpd_dn7 = assign55120_e90204_d_n7;
        locals.var_dpd_dn8 = assign55120_e90204_d_n8;
        locals.var_dpd_dn9 = assign55120_e90204_d_n9;
        locals.var_dpd_dn10 = assign55120_e90204_d_n10;
        locals.var_dpd_dn11 = assign55120_e90204_d_n11;

        let (assign55130_e90213, assign55130_e90213_d_n3, assign55130_e90213_d_n4, assign55130_e90213_d_n5, assign55130_e90213_d_n6, assign55130_e90213_d_n7, assign55130_e90213_d_n8, assign55130_e90213_d_n9, assign55130_e90213_d_n10, assign55130_e90213_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55130_e90211: f64 = (1.0 + locals.var_dpd);
        (assign55130_e90211, locals.var_dpd_dn3, locals.var_dpd_dn4, locals.var_dpd_dn5, locals.var_dpd_dn6, locals.var_dpd_dn7, locals.var_dpd_dn8, locals.var_dpd_dn9, locals.var_dpd_dn10, locals.var_dpd_dn11,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign55130_e90213;
        locals.var_t1_dn3 = assign55130_e90213_d_n3;
        locals.var_t1_dn4 = assign55130_e90213_d_n4;
        locals.var_t1_dn5 = assign55130_e90213_d_n5;
        locals.var_t1_dn6 = assign55130_e90213_d_n6;
        locals.var_t1_dn7 = assign55130_e90213_d_n7;
        locals.var_t1_dn8 = assign55130_e90213_d_n8;
        locals.var_t1_dn9 = assign55130_e90213_d_n9;
        locals.var_t1_dn10 = assign55130_e90213_d_n10;
        locals.var_t1_dn11 = assign55130_e90213_d_n11;

        let (assign55140_e90222, assign55140_e90222_d_n3, assign55140_e90222_d_n4, assign55140_e90222_d_n5, assign55140_e90222_d_n6, assign55140_e90222_d_n7, assign55140_e90222_d_n8, assign55140_e90222_d_n9, assign55140_e90222_d_n10, assign55140_e90222_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55140_e90220: f64 = (locals.var_vgfbcv / locals.var_t1);
        (assign55140_e90220, (((locals.var_vgfbcv_dn3 * locals.var_t1) - (locals.var_vgfbcv * locals.var_t1_dn3)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfbcv_dn4 * locals.var_t1) - (locals.var_vgfbcv * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfbcv_dn5 * locals.var_t1) - (locals.var_vgfbcv * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfbcv_dn6 * locals.var_t1) - (locals.var_vgfbcv * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfbcv_dn7 * locals.var_t1) - (locals.var_vgfbcv * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfbcv_dn8 * locals.var_t1) - (locals.var_vgfbcv * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfbcv_dn9 * locals.var_t1) - (locals.var_vgfbcv * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfbcv_dn10 * locals.var_t1) - (locals.var_vgfbcv * locals.var_t1_dn10)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfbcv_dn11 * locals.var_t1) - (locals.var_vgfbcv * locals.var_t1_dn11)) / (locals.var_t1 * locals.var_t1)),)
    } else {
        (locals.var_vgfbpd, locals.var_vgfbpd_dn3, locals.var_vgfbpd_dn4, locals.var_vgfbpd_dn5, locals.var_vgfbpd_dn6, locals.var_vgfbpd_dn7, locals.var_vgfbpd_dn8, locals.var_vgfbpd_dn9, locals.var_vgfbpd_dn10, locals.var_vgfbpd_dn11,)
    }
};
        locals.var_vgfbpd = assign55140_e90222;
        locals.var_vgfbpd_dn3 = assign55140_e90222_d_n3;
        locals.var_vgfbpd_dn4 = assign55140_e90222_d_n4;
        locals.var_vgfbpd_dn5 = assign55140_e90222_d_n5;
        locals.var_vgfbpd_dn6 = assign55140_e90222_d_n6;
        locals.var_vgfbpd_dn7 = assign55140_e90222_d_n7;
        locals.var_vgfbpd_dn8 = assign55140_e90222_d_n8;
        locals.var_vgfbpd_dn9 = assign55140_e90222_d_n9;
        locals.var_vgfbpd_dn10 = assign55140_e90222_d_n10;
        locals.var_vgfbpd_dn11 = assign55140_e90222_d_n11;

        let (assign55150_e90231, assign55150_e90231_d_n3, assign55150_e90231_d_n4, assign55150_e90231_d_n5, assign55150_e90231_d_n6, assign55150_e90231_d_n7, assign55150_e90231_d_n8, assign55150_e90231_d_n9, assign55150_e90231_d_n10, assign55150_e90231_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55150_e90229: f64 = (locals.var_gamcv / locals.var_t1);
        (assign55150_e90229, (((locals.var_gamcv_dn3 * locals.var_t1) - (locals.var_gamcv * locals.var_t1_dn3)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gamcv_dn4 * locals.var_t1) - (locals.var_gamcv * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gamcv_dn5 * locals.var_t1) - (locals.var_gamcv * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gamcv_dn6 * locals.var_t1) - (locals.var_gamcv * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gamcv_dn7 * locals.var_t1) - (locals.var_gamcv * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gamcv_dn8 * locals.var_t1) - (locals.var_gamcv * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gamcv_dn9 * locals.var_t1) - (locals.var_gamcv * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gamcv_dn10 * locals.var_t1) - (locals.var_gamcv * locals.var_t1_dn10)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gamcv_dn11 * locals.var_t1) - (locals.var_gamcv * locals.var_t1_dn11)) / (locals.var_t1 * locals.var_t1)),)
    } else {
        (locals.var_gammapd, locals.var_gammapd_dn3, locals.var_gammapd_dn4, locals.var_gammapd_dn5, locals.var_gammapd_dn6, locals.var_gammapd_dn7, locals.var_gammapd_dn8, locals.var_gammapd_dn9, locals.var_gammapd_dn10, locals.var_gammapd_dn11,)
    }
};
        locals.var_gammapd = assign55150_e90231;
        locals.var_gammapd_dn3 = assign55150_e90231_d_n3;
        locals.var_gammapd_dn4 = assign55150_e90231_d_n4;
        locals.var_gammapd_dn5 = assign55150_e90231_d_n5;
        locals.var_gammapd_dn6 = assign55150_e90231_d_n6;
        locals.var_gammapd_dn7 = assign55150_e90231_d_n7;
        locals.var_gammapd_dn8 = assign55150_e90231_d_n8;
        locals.var_gammapd_dn9 = assign55150_e90231_d_n9;
        locals.var_gammapd_dn10 = assign55150_e90231_d_n10;
        locals.var_gammapd_dn11 = assign55150_e90231_d_n11;

        let (assign55160_e90248, assign55160_e90248_d_n3, assign55160_e90248_d_n4, assign55160_e90248_d_n5, assign55160_e90248_d_n6, assign55160_e90248_d_n7, assign55160_e90248_d_n8, assign55160_e90248_d_n9, assign55160_e90248_d_n10, assign55160_e90248_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55160_e90238: f64 = (0.5 * locals.var_vgfbpd);
        let assign55160_e90243: f64 = (locals.var_gammapd / 1.4142135623730951);
        let assign55160_e90244: f64 = (1.0 + assign55160_e90243);
        let assign55160_e90245: f64 = (3.0 * assign55160_e90244);
        let assign55160_e90246: f64 = (assign55160_e90238 - assign55160_e90245);
        (assign55160_e90246, ((0.5 * locals.var_vgfbpd_dn3) - (3.0 * (locals.var_gammapd_dn3 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn4) - (3.0 * (locals.var_gammapd_dn4 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn5) - (3.0 * (locals.var_gammapd_dn5 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn6) - (3.0 * (locals.var_gammapd_dn6 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn7) - (3.0 * (locals.var_gammapd_dn7 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn8) - (3.0 * (locals.var_gammapd_dn8 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn9) - (3.0 * (locals.var_gammapd_dn9 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn10) - (3.0 * (locals.var_gammapd_dn10 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn11) - (3.0 * (locals.var_gammapd_dn11 / 1.4142135623730951))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign55160_e90248;
        locals.var_t1_dn3 = assign55160_e90248_d_n3;
        locals.var_t1_dn4 = assign55160_e90248_d_n4;
        locals.var_t1_dn5 = assign55160_e90248_d_n5;
        locals.var_t1_dn6 = assign55160_e90248_d_n6;
        locals.var_t1_dn7 = assign55160_e90248_d_n7;
        locals.var_t1_dn8 = assign55160_e90248_d_n8;
        locals.var_t1_dn9 = assign55160_e90248_d_n9;
        locals.var_t1_dn10 = assign55160_e90248_d_n10;
        locals.var_t1_dn11 = assign55160_e90248_d_n11;

        let (assign55170_e90264, assign55170_e90264_d_n3, assign55170_e90264_d_n4, assign55170_e90264_d_n5, assign55170_e90264_d_n6, assign55170_e90264_d_n7, assign55170_e90264_d_n8, assign55170_e90264_d_n9, assign55170_e90264_d_n10, assign55170_e90264_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55170_e90256: f64 = (locals.var_t1 * locals.var_t1);
        let assign55170_e90259: f64 = (6.0 * locals.var_vgfbpd);
        let assign55170_e90260: f64 = (assign55170_e90256 + assign55170_e90259);
        let assign55170_e90261: f64 = (assign55170_e90260).sqrt();
        let assign55170_e90262: f64 = (locals.var_t1 + assign55170_e90261);
        (assign55170_e90262, (locals.var_t1_dn3 + ((((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)) + (6.0 * locals.var_vgfbpd_dn3)) / (2.0 * assign55170_e90261))), (locals.var_t1_dn4 + ((((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) + (6.0 * locals.var_vgfbpd_dn4)) / (2.0 * assign55170_e90261))), (locals.var_t1_dn5 + ((((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) + (6.0 * locals.var_vgfbpd_dn5)) / (2.0 * assign55170_e90261))), (locals.var_t1_dn6 + ((((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) + (6.0 * locals.var_vgfbpd_dn6)) / (2.0 * assign55170_e90261))), (locals.var_t1_dn7 + ((((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) + (6.0 * locals.var_vgfbpd_dn7)) / (2.0 * assign55170_e90261))), (locals.var_t1_dn8 + ((((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) + (6.0 * locals.var_vgfbpd_dn8)) / (2.0 * assign55170_e90261))), (locals.var_t1_dn9 + ((((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) + (6.0 * locals.var_vgfbpd_dn9)) / (2.0 * assign55170_e90261))), (locals.var_t1_dn10 + ((((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) + (6.0 * locals.var_vgfbpd_dn10)) / (2.0 * assign55170_e90261))), (locals.var_t1_dn11 + ((((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) + (6.0 * locals.var_vgfbpd_dn11)) / (2.0 * assign55170_e90261))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign55170_e90264;
        locals.var_t2_dn3 = assign55170_e90264_d_n3;
        locals.var_t2_dn4 = assign55170_e90264_d_n4;
        locals.var_t2_dn5 = assign55170_e90264_d_n5;
        locals.var_t2_dn6 = assign55170_e90264_d_n6;
        locals.var_t2_dn7 = assign55170_e90264_d_n7;
        locals.var_t2_dn8 = assign55170_e90264_d_n8;
        locals.var_t2_dn9 = assign55170_e90264_d_n9;
        locals.var_t2_dn10 = assign55170_e90264_d_n10;
        locals.var_t2_dn11 = assign55170_e90264_d_n11;

        let assign55180_e90267: f64 = if locals.var_vgfbpd < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard843 = assign55180_e90267;

        let (assign55190_e90280, assign55190_e90280_d_n3, assign55190_e90280_d_n4, assign55190_e90280_d_n5, assign55190_e90280_d_n6, assign55190_e90280_d_n7, assign55190_e90280_d_n8, assign55190_e90280_d_n9, assign55190_e90280_d_n10, assign55190_e90280_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard843 != 0.0)) {
        let assign55190_e90276: f64 = (locals.var_vgfbpd - locals.var_t2);
        let assign55190_e90278: f64 = (assign55190_e90276 / locals.var_gammapd);
        (assign55190_e90278, ((((locals.var_vgfbpd_dn3 - locals.var_t2_dn3) * locals.var_gammapd) - (assign55190_e90276 * locals.var_gammapd_dn3)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn4 - locals.var_t2_dn4) * locals.var_gammapd) - (assign55190_e90276 * locals.var_gammapd_dn4)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn5 - locals.var_t2_dn5) * locals.var_gammapd) - (assign55190_e90276 * locals.var_gammapd_dn5)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn6 - locals.var_t2_dn6) * locals.var_gammapd) - (assign55190_e90276 * locals.var_gammapd_dn6)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn7 - locals.var_t2_dn7) * locals.var_gammapd) - (assign55190_e90276 * locals.var_gammapd_dn7)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn8 - locals.var_t2_dn8) * locals.var_gammapd) - (assign55190_e90276 * locals.var_gammapd_dn8)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn9 - locals.var_t2_dn9) * locals.var_gammapd) - (assign55190_e90276 * locals.var_gammapd_dn9)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn10 - locals.var_t2_dn10) * locals.var_gammapd) - (assign55190_e90276 * locals.var_gammapd_dn10)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn11 - locals.var_t2_dn11) * locals.var_gammapd) - (assign55190_e90276 * locals.var_gammapd_dn11)) / (locals.var_gammapd * locals.var_gammapd)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign55190_e90280;
        locals.var_t3_dn3 = assign55190_e90280_d_n3;
        locals.var_t3_dn4 = assign55190_e90280_d_n4;
        locals.var_t3_dn5 = assign55190_e90280_d_n5;
        locals.var_t3_dn6 = assign55190_e90280_d_n6;
        locals.var_t3_dn7 = assign55190_e90280_d_n7;
        locals.var_t3_dn8 = assign55190_e90280_d_n8;
        locals.var_t3_dn9 = assign55190_e90280_d_n9;
        locals.var_t3_dn10 = assign55190_e90280_d_n10;
        locals.var_t3_dn11 = assign55190_e90280_d_n11;

        let (assign55200_e90299, assign55200_e90299_d_n3, assign55200_e90299_d_n4, assign55200_e90299_d_n5, assign55200_e90299_d_n6, assign55200_e90299_d_n7, assign55200_e90299_d_n8, assign55200_e90299_d_n9, assign55200_e90299_d_n10, assign55200_e90299_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard843 != 0.0)) {
        let assign55200_e90289: f64 = (1.0 - locals.var_t2);
        let assign55200_e90292: f64 = (locals.var_t3 * locals.var_t3);
        let assign55200_e90293: f64 = (assign55200_e90289 + assign55200_e90292);
        let assign55200_e90295: f64 = (assign55200_e90293).max(1e-38);
        let assign55200_e90296: f64 = (assign55200_e90295).ln();
        let assign55200_e90297: f64 = (-assign55200_e90296);
        (assign55200_e90297, (-(if assign55200_e90293 >= 1e-38 { ((-locals.var_t2_dn3) + ((locals.var_t3_dn3 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn3))) } else { 0.0 } / assign55200_e90295)), (-(if assign55200_e90293 >= 1e-38 { ((-locals.var_t2_dn4) + ((locals.var_t3_dn4 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn4))) } else { 0.0 } / assign55200_e90295)), (-(if assign55200_e90293 >= 1e-38 { ((-locals.var_t2_dn5) + ((locals.var_t3_dn5 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn5))) } else { 0.0 } / assign55200_e90295)), (-(if assign55200_e90293 >= 1e-38 { ((-locals.var_t2_dn6) + ((locals.var_t3_dn6 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn6))) } else { 0.0 } / assign55200_e90295)), (-(if assign55200_e90293 >= 1e-38 { ((-locals.var_t2_dn7) + ((locals.var_t3_dn7 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn7))) } else { 0.0 } / assign55200_e90295)), (-(if assign55200_e90293 >= 1e-38 { ((-locals.var_t2_dn8) + ((locals.var_t3_dn8 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn8))) } else { 0.0 } / assign55200_e90295)), (-(if assign55200_e90293 >= 1e-38 { ((-locals.var_t2_dn9) + ((locals.var_t3_dn9 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn9))) } else { 0.0 } / assign55200_e90295)), (-(if assign55200_e90293 >= 1e-38 { ((-locals.var_t2_dn10) + ((locals.var_t3_dn10 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn10))) } else { 0.0 } / assign55200_e90295)), (-(if assign55200_e90293 >= 1e-38 { ((-locals.var_t2_dn11) + ((locals.var_t3_dn11 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn11))) } else { 0.0 } / assign55200_e90295)),)
    } else {
        (locals.var_psip, locals.var_psip_dn3, locals.var_psip_dn4, locals.var_psip_dn5, locals.var_psip_dn6, locals.var_psip_dn7, locals.var_psip_dn8, locals.var_psip_dn9, locals.var_psip_dn10, locals.var_psip_dn11,)
    }
};
        locals.var_psip = assign55200_e90299;
        locals.var_psip_dn3 = assign55200_e90299_d_n3;
        locals.var_psip_dn4 = assign55200_e90299_d_n4;
        locals.var_psip_dn5 = assign55200_e90299_d_n5;
        locals.var_psip_dn6 = assign55200_e90299_d_n6;
        locals.var_psip_dn7 = assign55200_e90299_d_n7;
        locals.var_psip_dn8 = assign55200_e90299_d_n8;
        locals.var_psip_dn9 = assign55200_e90299_d_n9;
        locals.var_psip_dn10 = assign55200_e90299_d_n10;
        locals.var_psip_dn11 = assign55200_e90299_d_n11;

        let (assign55210_e90311, assign55210_e90311_d_n3, assign55210_e90311_d_n4, assign55210_e90311_d_n5, assign55210_e90311_d_n6, assign55210_e90311_d_n7, assign55210_e90311_d_n8, assign55210_e90311_d_n9, assign55210_e90311_d_n10, assign55210_e90311_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard843 == 0.0)) {
        let assign55210_e90308: f64 = (-locals.var_t2);
        let assign55210_e90309: f64 = { let limited_exp_arg = assign55210_e90308; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign55210_e90309, ({ let limited_exp_arg = assign55210_e90308; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn3)), ({ let limited_exp_arg = assign55210_e90308; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn4)), ({ let limited_exp_arg = assign55210_e90308; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn5)), ({ let limited_exp_arg = assign55210_e90308; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn6)), ({ let limited_exp_arg = assign55210_e90308; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn7)), ({ let limited_exp_arg = assign55210_e90308; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn8)), ({ let limited_exp_arg = assign55210_e90308; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn9)), ({ let limited_exp_arg = assign55210_e90308; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn10)), ({ let limited_exp_arg = assign55210_e90308; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn11)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign55210_e90311;
        locals.var_t3_dn3 = assign55210_e90311_d_n3;
        locals.var_t3_dn4 = assign55210_e90311_d_n4;
        locals.var_t3_dn5 = assign55210_e90311_d_n5;
        locals.var_t3_dn6 = assign55210_e90311_d_n6;
        locals.var_t3_dn7 = assign55210_e90311_d_n7;
        locals.var_t3_dn8 = assign55210_e90311_d_n8;
        locals.var_t3_dn9 = assign55210_e90311_d_n9;
        locals.var_t3_dn10 = assign55210_e90311_d_n10;
        locals.var_t3_dn11 = assign55210_e90311_d_n11;

        let (assign55220_e90323, assign55220_e90323_d_n3, assign55220_e90323_d_n4, assign55220_e90323_d_n5, assign55220_e90323_d_n6, assign55220_e90323_d_n7, assign55220_e90323_d_n8, assign55220_e90323_d_n9, assign55220_e90323_d_n10, assign55220_e90323_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard843 == 0.0)) {
        let assign55220_e90321: f64 = (0.5 * locals.var_gammapd);
        (assign55220_e90321, (0.5 * locals.var_gammapd_dn3), (0.5 * locals.var_gammapd_dn4), (0.5 * locals.var_gammapd_dn5), (0.5 * locals.var_gammapd_dn6), (0.5 * locals.var_gammapd_dn7), (0.5 * locals.var_gammapd_dn8), (0.5 * locals.var_gammapd_dn9), (0.5 * locals.var_gammapd_dn10), (0.5 * locals.var_gammapd_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign55220_e90323;
        locals.var_t1_dn3 = assign55220_e90323_d_n3;
        locals.var_t1_dn4 = assign55220_e90323_d_n4;
        locals.var_t1_dn5 = assign55220_e90323_d_n5;
        locals.var_t1_dn6 = assign55220_e90323_d_n6;
        locals.var_t1_dn7 = assign55220_e90323_d_n7;
        locals.var_t1_dn8 = assign55220_e90323_d_n8;
        locals.var_t1_dn9 = assign55220_e90323_d_n9;
        locals.var_t1_dn10 = assign55220_e90323_d_n10;
        locals.var_t1_dn11 = assign55220_e90323_d_n11;

        let (assign55230_e90344, assign55230_e90344_d_n3, assign55230_e90344_d_n4, assign55230_e90344_d_n5, assign55230_e90344_d_n6, assign55230_e90344_d_n7, assign55230_e90344_d_n8, assign55230_e90344_d_n9, assign55230_e90344_d_n10, assign55230_e90344_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard843 == 0.0)) {
        let assign55230_e90333: f64 = (locals.var_vgfbpd - 1.0);
        let assign55230_e90335: f64 = (assign55230_e90333 + locals.var_t3);
        let assign55230_e90338: f64 = (locals.var_t1 * locals.var_t1);
        let assign55230_e90339: f64 = (assign55230_e90335 + assign55230_e90338);
        let assign55230_e90340: f64 = (assign55230_e90339).sqrt();
        let assign55230_e90342: f64 = (assign55230_e90340 - locals.var_t1);
        (assign55230_e90342, ((((locals.var_vgfbpd_dn3 + locals.var_t3_dn3) + ((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3))) / (2.0 * assign55230_e90340)) - locals.var_t1_dn3), ((((locals.var_vgfbpd_dn4 + locals.var_t3_dn4) + ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4))) / (2.0 * assign55230_e90340)) - locals.var_t1_dn4), ((((locals.var_vgfbpd_dn5 + locals.var_t3_dn5) + ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5))) / (2.0 * assign55230_e90340)) - locals.var_t1_dn5), ((((locals.var_vgfbpd_dn6 + locals.var_t3_dn6) + ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6))) / (2.0 * assign55230_e90340)) - locals.var_t1_dn6), ((((locals.var_vgfbpd_dn7 + locals.var_t3_dn7) + ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7))) / (2.0 * assign55230_e90340)) - locals.var_t1_dn7), ((((locals.var_vgfbpd_dn8 + locals.var_t3_dn8) + ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8))) / (2.0 * assign55230_e90340)) - locals.var_t1_dn8), ((((locals.var_vgfbpd_dn9 + locals.var_t3_dn9) + ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9))) / (2.0 * assign55230_e90340)) - locals.var_t1_dn9), ((((locals.var_vgfbpd_dn10 + locals.var_t3_dn10) + ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10))) / (2.0 * assign55230_e90340)) - locals.var_t1_dn10), ((((locals.var_vgfbpd_dn11 + locals.var_t3_dn11) + ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11))) / (2.0 * assign55230_e90340)) - locals.var_t1_dn11),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign55230_e90344;
        locals.var_t2_dn3 = assign55230_e90344_d_n3;
        locals.var_t2_dn4 = assign55230_e90344_d_n4;
        locals.var_t2_dn5 = assign55230_e90344_d_n5;
        locals.var_t2_dn6 = assign55230_e90344_d_n6;
        locals.var_t2_dn7 = assign55230_e90344_d_n7;
        locals.var_t2_dn8 = assign55230_e90344_d_n8;
        locals.var_t2_dn9 = assign55230_e90344_d_n9;
        locals.var_t2_dn10 = assign55230_e90344_d_n10;
        locals.var_t2_dn11 = assign55230_e90344_d_n11;

        let (assign55240_e90360, assign55240_e90360_d_n3, assign55240_e90360_d_n4, assign55240_e90360_d_n5, assign55240_e90360_d_n6, assign55240_e90360_d_n7, assign55240_e90360_d_n8, assign55240_e90360_d_n9, assign55240_e90360_d_n10, assign55240_e90360_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard843 == 0.0)) {
        let assign55240_e90354: f64 = (locals.var_t2 * locals.var_t2);
        let assign55240_e90356: f64 = (assign55240_e90354 + 1.0);
        let assign55240_e90358: f64 = (assign55240_e90356 - locals.var_t3);
        (assign55240_e90358, (((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)) - locals.var_t3_dn3), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) - locals.var_t3_dn4), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) - locals.var_t3_dn5), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) - locals.var_t3_dn6), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) - locals.var_t3_dn7), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) - locals.var_t3_dn8), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) - locals.var_t3_dn9), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) - locals.var_t3_dn10), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) - locals.var_t3_dn11),)
    } else {
        (locals.var_psip, locals.var_psip_dn3, locals.var_psip_dn4, locals.var_psip_dn5, locals.var_psip_dn6, locals.var_psip_dn7, locals.var_psip_dn8, locals.var_psip_dn9, locals.var_psip_dn10, locals.var_psip_dn11,)
    }
};
        locals.var_psip = assign55240_e90360;
        locals.var_psip_dn3 = assign55240_e90360_d_n3;
        locals.var_psip_dn4 = assign55240_e90360_d_n4;
        locals.var_psip_dn5 = assign55240_e90360_d_n5;
        locals.var_psip_dn6 = assign55240_e90360_d_n6;
        locals.var_psip_dn7 = assign55240_e90360_d_n7;
        locals.var_psip_dn8 = assign55240_e90360_d_n8;
        locals.var_psip_dn9 = assign55240_e90360_d_n9;
        locals.var_psip_dn10 = assign55240_e90360_d_n10;
        locals.var_psip_dn11 = assign55240_e90360_d_n11;

        let (assign55250_e90386, assign55250_e90386_d_n3, assign55250_e90386_d_n4, assign55250_e90386_d_n5, assign55250_e90386_d_n6, assign55250_e90386_d_n7, assign55250_e90386_d_n8, assign55250_e90386_d_n9, assign55250_e90386_d_n10, assign55250_e90386_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55250_e90368: f64 = (locals.var_psip + 1.0);
        let assign55250_e90371: f64 = (locals.var_psip - 1.0);
        let assign55250_e90374: f64 = (locals.var_psip - 1.0);
        let assign55250_e90375: f64 = (assign55250_e90371 * assign55250_e90374);
        let assign55250_e90378: f64 = (0.25 * 2.0);
        let assign55250_e90380: f64 = (assign55250_e90378 * 2.0);
        let assign55250_e90381: f64 = (assign55250_e90375 + assign55250_e90380);
        let assign55250_e90382: f64 = (assign55250_e90381).sqrt();
        let assign55250_e90383: f64 = (assign55250_e90368 + assign55250_e90382);
        let assign55250_e90384: f64 = (0.5 * assign55250_e90383);
        (assign55250_e90384, (0.5 * (locals.var_psip_dn3 + (((locals.var_psip_dn3 * assign55250_e90374) + (assign55250_e90371 * locals.var_psip_dn3)) / (2.0 * assign55250_e90382)))), (0.5 * (locals.var_psip_dn4 + (((locals.var_psip_dn4 * assign55250_e90374) + (assign55250_e90371 * locals.var_psip_dn4)) / (2.0 * assign55250_e90382)))), (0.5 * (locals.var_psip_dn5 + (((locals.var_psip_dn5 * assign55250_e90374) + (assign55250_e90371 * locals.var_psip_dn5)) / (2.0 * assign55250_e90382)))), (0.5 * (locals.var_psip_dn6 + (((locals.var_psip_dn6 * assign55250_e90374) + (assign55250_e90371 * locals.var_psip_dn6)) / (2.0 * assign55250_e90382)))), (0.5 * (locals.var_psip_dn7 + (((locals.var_psip_dn7 * assign55250_e90374) + (assign55250_e90371 * locals.var_psip_dn7)) / (2.0 * assign55250_e90382)))), (0.5 * (locals.var_psip_dn8 + (((locals.var_psip_dn8 * assign55250_e90374) + (assign55250_e90371 * locals.var_psip_dn8)) / (2.0 * assign55250_e90382)))), (0.5 * (locals.var_psip_dn9 + (((locals.var_psip_dn9 * assign55250_e90374) + (assign55250_e90371 * locals.var_psip_dn9)) / (2.0 * assign55250_e90382)))), (0.5 * (locals.var_psip_dn10 + (((locals.var_psip_dn10 * assign55250_e90374) + (assign55250_e90371 * locals.var_psip_dn10)) / (2.0 * assign55250_e90382)))), (0.5 * (locals.var_psip_dn11 + (((locals.var_psip_dn11 * assign55250_e90374) + (assign55250_e90371 * locals.var_psip_dn11)) / (2.0 * assign55250_e90382)))),)
    } else {
        (locals.var_t8, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11,)
    }
};
        locals.var_t8 = assign55250_e90386;
        locals.var_t8_dn3 = assign55250_e90386_d_n3;
        locals.var_t8_dn4 = assign55250_e90386_d_n4;
        locals.var_t8_dn5 = assign55250_e90386_d_n5;
        locals.var_t8_dn6 = assign55250_e90386_d_n6;
        locals.var_t8_dn7 = assign55250_e90386_d_n7;
        locals.var_t8_dn8 = assign55250_e90386_d_n8;
        locals.var_t8_dn9 = assign55250_e90386_d_n9;
        locals.var_t8_dn10 = assign55250_e90386_d_n10;
        locals.var_t8_dn11 = assign55250_e90386_d_n11;

        let (assign55260_e90394, assign55260_e90394_d_n3, assign55260_e90394_d_n4, assign55260_e90394_d_n5, assign55260_e90394_d_n6, assign55260_e90394_d_n7, assign55260_e90394_d_n8, assign55260_e90394_d_n9, assign55260_e90394_d_n10, assign55260_e90394_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55260_e90392: f64 = (locals.var_t8).sqrt();
        (assign55260_e90392, (locals.var_t8_dn3 / (2.0 * assign55260_e90392)), (locals.var_t8_dn4 / (2.0 * assign55260_e90392)), (locals.var_t8_dn5 / (2.0 * assign55260_e90392)), (locals.var_t8_dn6 / (2.0 * assign55260_e90392)), (locals.var_t8_dn7 / (2.0 * assign55260_e90392)), (locals.var_t8_dn8 / (2.0 * assign55260_e90392)), (locals.var_t8_dn9 / (2.0 * assign55260_e90392)), (locals.var_t8_dn10 / (2.0 * assign55260_e90392)), (locals.var_t8_dn11 / (2.0 * assign55260_e90392)),)
    } else {
        (locals.var_sqrtpsip, locals.var_sqrtpsip_dn3, locals.var_sqrtpsip_dn4, locals.var_sqrtpsip_dn5, locals.var_sqrtpsip_dn6, locals.var_sqrtpsip_dn7, locals.var_sqrtpsip_dn8, locals.var_sqrtpsip_dn9, locals.var_sqrtpsip_dn10, locals.var_sqrtpsip_dn11,)
    }
};
        locals.var_sqrtpsip = assign55260_e90394;
        locals.var_sqrtpsip_dn3 = assign55260_e90394_d_n3;
        locals.var_sqrtpsip_dn4 = assign55260_e90394_d_n4;
        locals.var_sqrtpsip_dn5 = assign55260_e90394_d_n5;
        locals.var_sqrtpsip_dn6 = assign55260_e90394_d_n6;
        locals.var_sqrtpsip_dn7 = assign55260_e90394_d_n7;
        locals.var_sqrtpsip_dn8 = assign55260_e90394_d_n8;
        locals.var_sqrtpsip_dn9 = assign55260_e90394_d_n9;
        locals.var_sqrtpsip_dn10 = assign55260_e90394_d_n10;
        locals.var_sqrtpsip_dn11 = assign55260_e90394_d_n11;

        let (assign55270_e90409, assign55270_e90409_d_n3, assign55270_e90409_d_n4, assign55270_e90409_d_n5, assign55270_e90409_d_n6, assign55270_e90409_d_n7, assign55270_e90409_d_n8, assign55270_e90409_d_n9, assign55270_e90409_d_n10, assign55270_e90409_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55270_e90403: f64 = (2.0 * locals.var_sqrtpsip);
        let assign55270_e90404: f64 = (locals.var_gamcv / assign55270_e90403);
        let assign55270_e90405: f64 = (1.0 + assign55270_e90404);
        let assign55270_e90407: f64 = (assign55270_e90405 / locals.var_gamcv);
        (assign55270_e90407, ((((((locals.var_gamcv_dn3 * assign55270_e90403) - (locals.var_gamcv * (2.0 * locals.var_sqrtpsip_dn3))) / (assign55270_e90403 * assign55270_e90403)) * locals.var_gamcv) - (assign55270_e90405 * locals.var_gamcv_dn3)) / (locals.var_gamcv * locals.var_gamcv)), ((((((locals.var_gamcv_dn4 * assign55270_e90403) - (locals.var_gamcv * (2.0 * locals.var_sqrtpsip_dn4))) / (assign55270_e90403 * assign55270_e90403)) * locals.var_gamcv) - (assign55270_e90405 * locals.var_gamcv_dn4)) / (locals.var_gamcv * locals.var_gamcv)), ((((((locals.var_gamcv_dn5 * assign55270_e90403) - (locals.var_gamcv * (2.0 * locals.var_sqrtpsip_dn5))) / (assign55270_e90403 * assign55270_e90403)) * locals.var_gamcv) - (assign55270_e90405 * locals.var_gamcv_dn5)) / (locals.var_gamcv * locals.var_gamcv)), ((((((locals.var_gamcv_dn6 * assign55270_e90403) - (locals.var_gamcv * (2.0 * locals.var_sqrtpsip_dn6))) / (assign55270_e90403 * assign55270_e90403)) * locals.var_gamcv) - (assign55270_e90405 * locals.var_gamcv_dn6)) / (locals.var_gamcv * locals.var_gamcv)), ((((((locals.var_gamcv_dn7 * assign55270_e90403) - (locals.var_gamcv * (2.0 * locals.var_sqrtpsip_dn7))) / (assign55270_e90403 * assign55270_e90403)) * locals.var_gamcv) - (assign55270_e90405 * locals.var_gamcv_dn7)) / (locals.var_gamcv * locals.var_gamcv)), ((((((locals.var_gamcv_dn8 * assign55270_e90403) - (locals.var_gamcv * (2.0 * locals.var_sqrtpsip_dn8))) / (assign55270_e90403 * assign55270_e90403)) * locals.var_gamcv) - (assign55270_e90405 * locals.var_gamcv_dn8)) / (locals.var_gamcv * locals.var_gamcv)), ((((((locals.var_gamcv_dn9 * assign55270_e90403) - (locals.var_gamcv * (2.0 * locals.var_sqrtpsip_dn9))) / (assign55270_e90403 * assign55270_e90403)) * locals.var_gamcv) - (assign55270_e90405 * locals.var_gamcv_dn9)) / (locals.var_gamcv * locals.var_gamcv)), ((((((locals.var_gamcv_dn10 * assign55270_e90403) - (locals.var_gamcv * (2.0 * locals.var_sqrtpsip_dn10))) / (assign55270_e90403 * assign55270_e90403)) * locals.var_gamcv) - (assign55270_e90405 * locals.var_gamcv_dn10)) / (locals.var_gamcv * locals.var_gamcv)), ((((((locals.var_gamcv_dn11 * assign55270_e90403) - (locals.var_gamcv * (2.0 * locals.var_sqrtpsip_dn11))) / (assign55270_e90403 * assign55270_e90403)) * locals.var_gamcv) - (assign55270_e90405 * locals.var_gamcv_dn11)) / (locals.var_gamcv * locals.var_gamcv)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign55270_e90409;
        locals.var_t0_dn3 = assign55270_e90409_d_n3;
        locals.var_t0_dn4 = assign55270_e90409_d_n4;
        locals.var_t0_dn5 = assign55270_e90409_d_n5;
        locals.var_t0_dn6 = assign55270_e90409_d_n6;
        locals.var_t0_dn7 = assign55270_e90409_d_n7;
        locals.var_t0_dn8 = assign55270_e90409_d_n8;
        locals.var_t0_dn9 = assign55270_e90409_d_n9;
        locals.var_t0_dn10 = assign55270_e90409_d_n10;
        locals.var_t0_dn11 = assign55270_e90409_d_n11;

        let (assign55280_e90422, assign55280_e90422_d_n3, assign55280_e90422_d_n4, assign55280_e90422_d_n5, assign55280_e90422_d_n6, assign55280_e90422_d_n7, assign55280_e90422_d_n8, assign55280_e90422_d_n9, assign55280_e90422_d_n10, assign55280_e90422_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55280_e90417: f64 = (2.0 * locals.var_phibcv);
        let assign55280_e90418: f64 = (locals.var_psip - assign55280_e90417);
        let assign55280_e90420: f64 = (assign55280_e90418 - locals.var_vs_1);
        (assign55280_e90420, ((locals.var_psip_dn3 - (2.0 * locals.var_phibcv_dn3)) - locals.var_vs_1_dn3), ((locals.var_psip_dn4 - (2.0 * locals.var_phibcv_dn4)) - locals.var_vs_1_dn4), ((locals.var_psip_dn5 - (2.0 * locals.var_phibcv_dn5)) - locals.var_vs_1_dn5), ((locals.var_psip_dn6 - (2.0 * locals.var_phibcv_dn6)) - locals.var_vs_1_dn6), ((locals.var_psip_dn7 - (2.0 * locals.var_phibcv_dn7)) - locals.var_vs_1_dn7), ((locals.var_psip_dn8 - (2.0 * locals.var_phibcv_dn8)) - locals.var_vs_1_dn8), ((locals.var_psip_dn9 - (2.0 * locals.var_phibcv_dn9)) - locals.var_vs_1_dn9), ((locals.var_psip_dn10 - (2.0 * locals.var_phibcv_dn10)) - locals.var_vs_1_dn10), ((locals.var_psip_dn11 - (2.0 * locals.var_phibcv_dn11)) - locals.var_vs_1_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign55280_e90422;
        locals.var_t1_dn3 = assign55280_e90422_d_n3;
        locals.var_t1_dn4 = assign55280_e90422_d_n4;
        locals.var_t1_dn5 = assign55280_e90422_d_n5;
        locals.var_t1_dn6 = assign55280_e90422_d_n6;
        locals.var_t1_dn7 = assign55280_e90422_d_n7;
        locals.var_t1_dn8 = assign55280_e90422_d_n8;
        locals.var_t1_dn9 = assign55280_e90422_d_n9;
        locals.var_t1_dn10 = assign55280_e90422_d_n10;
        locals.var_t1_dn11 = assign55280_e90422_d_n11;

        let (assign55290_e90438, assign55290_e90438_d_n3, assign55290_e90438_d_n4, assign55290_e90438_d_n5, assign55290_e90438_d_n6, assign55290_e90438_d_n7, assign55290_e90438_d_n8, assign55290_e90438_d_n9, assign55290_e90438_d_n10, assign55290_e90438_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55290_e90430: f64 = (4.0 * locals.var_t0);
        let assign55290_e90432: f64 = (assign55290_e90430 * locals.var_sqrtpsip);
        let assign55290_e90434: f64 = (assign55290_e90432).max(1e-38);
        let assign55290_e90435: f64 = (assign55290_e90434).ln();
        let assign55290_e90436: f64 = (locals.var_t1 - assign55290_e90435);
        (assign55290_e90436, (locals.var_t1_dn3 - (if assign55290_e90432 >= 1e-38 { (((4.0 * locals.var_t0_dn3) * locals.var_sqrtpsip) + (assign55290_e90430 * locals.var_sqrtpsip_dn3)) } else { 0.0 } / assign55290_e90434)), (locals.var_t1_dn4 - (if assign55290_e90432 >= 1e-38 { (((4.0 * locals.var_t0_dn4) * locals.var_sqrtpsip) + (assign55290_e90430 * locals.var_sqrtpsip_dn4)) } else { 0.0 } / assign55290_e90434)), (locals.var_t1_dn5 - (if assign55290_e90432 >= 1e-38 { (((4.0 * locals.var_t0_dn5) * locals.var_sqrtpsip) + (assign55290_e90430 * locals.var_sqrtpsip_dn5)) } else { 0.0 } / assign55290_e90434)), (locals.var_t1_dn6 - (if assign55290_e90432 >= 1e-38 { (((4.0 * locals.var_t0_dn6) * locals.var_sqrtpsip) + (assign55290_e90430 * locals.var_sqrtpsip_dn6)) } else { 0.0 } / assign55290_e90434)), (locals.var_t1_dn7 - (if assign55290_e90432 >= 1e-38 { (((4.0 * locals.var_t0_dn7) * locals.var_sqrtpsip) + (assign55290_e90430 * locals.var_sqrtpsip_dn7)) } else { 0.0 } / assign55290_e90434)), (locals.var_t1_dn8 - (if assign55290_e90432 >= 1e-38 { (((4.0 * locals.var_t0_dn8) * locals.var_sqrtpsip) + (assign55290_e90430 * locals.var_sqrtpsip_dn8)) } else { 0.0 } / assign55290_e90434)), (locals.var_t1_dn9 - (if assign55290_e90432 >= 1e-38 { (((4.0 * locals.var_t0_dn9) * locals.var_sqrtpsip) + (assign55290_e90430 * locals.var_sqrtpsip_dn9)) } else { 0.0 } / assign55290_e90434)), (locals.var_t1_dn10 - (if assign55290_e90432 >= 1e-38 { (((4.0 * locals.var_t0_dn10) * locals.var_sqrtpsip) + (assign55290_e90430 * locals.var_sqrtpsip_dn10)) } else { 0.0 } / assign55290_e90434)), (locals.var_t1_dn11 - (if assign55290_e90432 >= 1e-38 { (((4.0 * locals.var_t0_dn11) * locals.var_sqrtpsip) + (assign55290_e90430 * locals.var_sqrtpsip_dn11)) } else { 0.0 } / assign55290_e90434)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign55290_e90438;
        locals.var_t2_dn3 = assign55290_e90438_d_n3;
        locals.var_t2_dn4 = assign55290_e90438_d_n4;
        locals.var_t2_dn5 = assign55290_e90438_d_n5;
        locals.var_t2_dn6 = assign55290_e90438_d_n6;
        locals.var_t2_dn7 = assign55290_e90438_d_n7;
        locals.var_t2_dn8 = assign55290_e90438_d_n8;
        locals.var_t2_dn9 = assign55290_e90438_d_n9;
        locals.var_t2_dn10 = assign55290_e90438_d_n10;
        locals.var_t2_dn11 = assign55290_e90438_d_n11;

    }

    pub(super) fn stamp_transient_block_187(
        locals: &mut StampLocals,
    ) {
        let (assign55300_e90458, assign55300_e90458_d_n3, assign55300_e90458_d_n4, assign55300_e90458_d_n5, assign55300_e90458_d_n6, assign55300_e90458_d_n7, assign55300_e90458_d_n8, assign55300_e90458_d_n9, assign55300_e90458_d_n10, assign55300_e90458_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55300_e90446: f64 = (locals.var_t2 - 0.201491);
        let assign55300_e90450: f64 = (locals.var_t2 + 0.402982);
        let assign55300_e90451: f64 = (locals.var_t2 * assign55300_e90450);
        let assign55300_e90453: f64 = (assign55300_e90451 + 2.446562);
        let assign55300_e90454: f64 = (assign55300_e90453).sqrt();
        let assign55300_e90455: f64 = (assign55300_e90446 - assign55300_e90454);
        let assign55300_e90456: f64 = (0.5 * assign55300_e90455);
        (assign55300_e90456, (0.5 * (locals.var_t2_dn3 - (((locals.var_t2_dn3 * assign55300_e90450) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign55300_e90454)))), (0.5 * (locals.var_t2_dn4 - (((locals.var_t2_dn4 * assign55300_e90450) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign55300_e90454)))), (0.5 * (locals.var_t2_dn5 - (((locals.var_t2_dn5 * assign55300_e90450) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign55300_e90454)))), (0.5 * (locals.var_t2_dn6 - (((locals.var_t2_dn6 * assign55300_e90450) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign55300_e90454)))), (0.5 * (locals.var_t2_dn7 - (((locals.var_t2_dn7 * assign55300_e90450) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign55300_e90454)))), (0.5 * (locals.var_t2_dn8 - (((locals.var_t2_dn8 * assign55300_e90450) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign55300_e90454)))), (0.5 * (locals.var_t2_dn9 - (((locals.var_t2_dn9 * assign55300_e90450) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign55300_e90454)))), (0.5 * (locals.var_t2_dn10 - (((locals.var_t2_dn10 * assign55300_e90450) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign55300_e90454)))), (0.5 * (locals.var_t2_dn11 - (((locals.var_t2_dn11 * assign55300_e90450) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign55300_e90454)))),)
    } else {
        (locals.var_t8, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11,)
    }
};
        locals.var_t8 = assign55300_e90458;
        locals.var_t8_dn3 = assign55300_e90458_d_n3;
        locals.var_t8_dn4 = assign55300_e90458_d_n4;
        locals.var_t8_dn5 = assign55300_e90458_d_n5;
        locals.var_t8_dn6 = assign55300_e90458_d_n6;
        locals.var_t8_dn7 = assign55300_e90458_d_n7;
        locals.var_t8_dn8 = assign55300_e90458_d_n8;
        locals.var_t8_dn9 = assign55300_e90458_d_n9;
        locals.var_t8_dn10 = assign55300_e90458_d_n10;
        locals.var_t8_dn11 = assign55300_e90458_d_n11;

        let (assign55310_e90465, assign55310_e90465_d_n3, assign55310_e90465_d_n4, assign55310_e90465_d_n5, assign55310_e90465_d_n6, assign55310_e90465_d_n7, assign55310_e90465_d_n8, assign55310_e90465_d_n9, assign55310_e90465_d_n10, assign55310_e90465_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        (locals.var_sqrtpsip, locals.var_sqrtpsip_dn3, locals.var_sqrtpsip_dn4, locals.var_sqrtpsip_dn5, locals.var_sqrtpsip_dn6, locals.var_sqrtpsip_dn7, locals.var_sqrtpsip_dn8, locals.var_sqrtpsip_dn9, locals.var_sqrtpsip_dn10, locals.var_sqrtpsip_dn11,)
    } else {
        (locals.var_sqrtpsisa, locals.var_sqrtpsisa_dn3, locals.var_sqrtpsisa_dn4, locals.var_sqrtpsisa_dn5, locals.var_sqrtpsisa_dn6, locals.var_sqrtpsisa_dn7, locals.var_sqrtpsisa_dn8, locals.var_sqrtpsisa_dn9, locals.var_sqrtpsisa_dn10, locals.var_sqrtpsisa_dn11,)
    }
};
        locals.var_sqrtpsisa = assign55310_e90465;
        locals.var_sqrtpsisa_dn3 = assign55310_e90465_d_n3;
        locals.var_sqrtpsisa_dn4 = assign55310_e90465_d_n4;
        locals.var_sqrtpsisa_dn5 = assign55310_e90465_d_n5;
        locals.var_sqrtpsisa_dn6 = assign55310_e90465_d_n6;
        locals.var_sqrtpsisa_dn7 = assign55310_e90465_d_n7;
        locals.var_sqrtpsisa_dn8 = assign55310_e90465_d_n8;
        locals.var_sqrtpsisa_dn9 = assign55310_e90465_d_n9;
        locals.var_sqrtpsisa_dn10 = assign55310_e90465_d_n10;
        locals.var_sqrtpsisa_dn11 = assign55310_e90465_d_n11;

        let assign55320_e90468: f64 = (-68.0);
        let assign55320_e90469: f64 = if locals.var_t8 <= assign55320_e90468 { 1.0 } else { 0.0 };
        locals.var_guard844 = assign55320_e90469;

        let (assign55330_e90479, assign55330_e90479_d_n3, assign55330_e90479_d_n4, assign55330_e90479_d_n5, assign55330_e90479_d_n6, assign55330_e90479_d_n7, assign55330_e90479_d_n8, assign55330_e90479_d_n9, assign55330_e90479_d_n10, assign55330_e90479_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard844 != 0.0)) {
        let assign55330_e90477: f64 = (-100.0);
        (assign55330_e90477, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign55330_e90479;
        locals.var_t4_dn3 = assign55330_e90479_d_n3;
        locals.var_t4_dn4 = assign55330_e90479_d_n4;
        locals.var_t4_dn5 = assign55330_e90479_d_n5;
        locals.var_t4_dn6 = assign55330_e90479_d_n6;
        locals.var_t4_dn7 = assign55330_e90479_d_n7;
        locals.var_t4_dn8 = assign55330_e90479_d_n8;
        locals.var_t4_dn9 = assign55330_e90479_d_n9;
        locals.var_t4_dn10 = assign55330_e90479_d_n10;
        locals.var_t4_dn11 = assign55330_e90479_d_n11;

        let (assign55340_e90488, assign55340_e90488_d_n3, assign55340_e90488_d_n4, assign55340_e90488_d_n5, assign55340_e90488_d_n6, assign55340_e90488_d_n7, assign55340_e90488_d_n8, assign55340_e90488_d_n9, assign55340_e90488_d_n10, assign55340_e90488_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard844 != 0.0)) {
        (20.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign55340_e90488;
        locals.var_t5_dn3 = assign55340_e90488_d_n3;
        locals.var_t5_dn4 = assign55340_e90488_d_n4;
        locals.var_t5_dn5 = assign55340_e90488_d_n5;
        locals.var_t5_dn6 = assign55340_e90488_d_n6;
        locals.var_t5_dn7 = assign55340_e90488_d_n7;
        locals.var_t5_dn8 = assign55340_e90488_d_n8;
        locals.var_t5_dn9 = assign55340_e90488_d_n9;
        locals.var_t5_dn10 = assign55340_e90488_d_n10;
        locals.var_t5_dn11 = assign55340_e90488_d_n11;

        let assign55350_e90493: f64 = (0.5 * locals.var_t5);
        let assign55350_e90494: f64 = (locals.var_t4 - assign55350_e90493);
        let assign55350_e90495: f64 = if locals.var_t8 < assign55350_e90494 { 1.0 } else { 0.0 };
        locals.var_guard845 = assign55350_e90495;

        let (assign55360_e90507, assign55360_e90507_d_n3, assign55360_e90507_d_n4, assign55360_e90507_d_n5, assign55360_e90507_d_n6, assign55360_e90507_d_n7, assign55360_e90507_d_n8, assign55360_e90507_d_n9, assign55360_e90507_d_n10, assign55360_e90507_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard844 != 0.0)) && (locals.var_guard845 != 0.0)) {
        let assign55360_e90505: f64 = { let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign55360_e90505, ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn3), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn4), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn5), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn6), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn7), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn8), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn9), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn10), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign55360_e90507;
        locals.var_t3_dn3 = assign55360_e90507_d_n3;
        locals.var_t3_dn4 = assign55360_e90507_d_n4;
        locals.var_t3_dn5 = assign55360_e90507_d_n5;
        locals.var_t3_dn6 = assign55360_e90507_d_n6;
        locals.var_t3_dn7 = assign55360_e90507_d_n7;
        locals.var_t3_dn8 = assign55360_e90507_d_n8;
        locals.var_t3_dn9 = assign55360_e90507_d_n9;
        locals.var_t3_dn10 = assign55360_e90507_d_n10;
        locals.var_t3_dn11 = assign55360_e90507_d_n11;

        let assign55370_e90512: f64 = (0.5 * locals.var_t5);
        let assign55370_e90513: f64 = (locals.var_t4 + assign55370_e90512);
        let assign55370_e90514: f64 = if locals.var_t8 > assign55370_e90513 { 1.0 } else { 0.0 };
        locals.var_guard846 = assign55370_e90514;

        let (assign55380_e90529, assign55380_e90529_d_n3, assign55380_e90529_d_n4, assign55380_e90529_d_n5, assign55380_e90529_d_n6, assign55380_e90529_d_n7, assign55380_e90529_d_n8, assign55380_e90529_d_n9, assign55380_e90529_d_n10, assign55380_e90529_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard844 != 0.0)) && (locals.var_guard845 == 0.0)) && (locals.var_guard846 != 0.0)) {
        let assign55380_e90527: f64 = { let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign55380_e90527, ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn3), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn4), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn5), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn6), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn7), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn8), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn9), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn10), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign55380_e90529;
        locals.var_t3_dn3 = assign55380_e90529_d_n3;
        locals.var_t3_dn4 = assign55380_e90529_d_n4;
        locals.var_t3_dn5 = assign55380_e90529_d_n5;
        locals.var_t3_dn6 = assign55380_e90529_d_n6;
        locals.var_t3_dn7 = assign55380_e90529_d_n7;
        locals.var_t3_dn8 = assign55380_e90529_d_n8;
        locals.var_t3_dn9 = assign55380_e90529_d_n9;
        locals.var_t3_dn10 = assign55380_e90529_d_n10;
        locals.var_t3_dn11 = assign55380_e90529_d_n11;

        let (assign55390_e90548, assign55390_e90548_d_n3, assign55390_e90548_d_n4, assign55390_e90548_d_n5, assign55390_e90548_d_n6, assign55390_e90548_d_n7, assign55390_e90548_d_n8, assign55390_e90548_d_n9, assign55390_e90548_d_n10, assign55390_e90548_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard844 != 0.0)) && (locals.var_guard845 == 0.0)) && (locals.var_guard846 == 0.0)) {
        let assign55390_e90544: f64 = (locals.var_t8 - locals.var_t4);
        let assign55390_e90546: f64 = (assign55390_e90544 / locals.var_t5);
        (assign55390_e90546, ((((locals.var_t8_dn3 - locals.var_t4_dn3) * locals.var_t5) - (assign55390_e90544 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn4 - locals.var_t4_dn4) * locals.var_t5) - (assign55390_e90544 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn5 - locals.var_t4_dn5) * locals.var_t5) - (assign55390_e90544 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn6 - locals.var_t4_dn6) * locals.var_t5) - (assign55390_e90544 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn7 - locals.var_t4_dn7) * locals.var_t5) - (assign55390_e90544 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn8 - locals.var_t4_dn8) * locals.var_t5) - (assign55390_e90544 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn9 - locals.var_t4_dn9) * locals.var_t5) - (assign55390_e90544 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn10 - locals.var_t4_dn10) * locals.var_t5) - (assign55390_e90544 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn11 - locals.var_t4_dn11) * locals.var_t5) - (assign55390_e90544 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign55390_e90548;
        locals.var_t2_dn3 = assign55390_e90548_d_n3;
        locals.var_t2_dn4 = assign55390_e90548_d_n4;
        locals.var_t2_dn5 = assign55390_e90548_d_n5;
        locals.var_t2_dn6 = assign55390_e90548_d_n6;
        locals.var_t2_dn7 = assign55390_e90548_d_n7;
        locals.var_t2_dn8 = assign55390_e90548_d_n8;
        locals.var_t2_dn9 = assign55390_e90548_d_n9;
        locals.var_t2_dn10 = assign55390_e90548_d_n10;
        locals.var_t2_dn11 = assign55390_e90548_d_n11;

        let (assign55400_e90565, assign55400_e90565_d_n3, assign55400_e90565_d_n4, assign55400_e90565_d_n5, assign55400_e90565_d_n6, assign55400_e90565_d_n7, assign55400_e90565_d_n8, assign55400_e90565_d_n9, assign55400_e90565_d_n10, assign55400_e90565_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard844 != 0.0)) && (locals.var_guard845 == 0.0)) && (locals.var_guard846 == 0.0)) {
        let assign55400_e90563: f64 = (locals.var_t2 * locals.var_t2);
        (assign55400_e90563, ((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)), ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)), ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)), ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)), ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)), ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)), ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)), ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)), ((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign55400_e90565;
        locals.var_t6_dn3 = assign55400_e90565_d_n3;
        locals.var_t6_dn4 = assign55400_e90565_d_n4;
        locals.var_t6_dn5 = assign55400_e90565_d_n5;
        locals.var_t6_dn6 = assign55400_e90565_d_n6;
        locals.var_t6_dn7 = assign55400_e90565_d_n7;
        locals.var_t6_dn8 = assign55400_e90565_d_n8;
        locals.var_t6_dn9 = assign55400_e90565_d_n9;
        locals.var_t6_dn10 = assign55400_e90565_d_n10;
        locals.var_t6_dn11 = assign55400_e90565_d_n11;

        let (assign55410_e90603, assign55410_e90603_d_n3, assign55410_e90603_d_n4, assign55410_e90603_d_n5, assign55410_e90603_d_n6, assign55410_e90603_d_n7, assign55410_e90603_d_n8, assign55410_e90603_d_n9, assign55410_e90603_d_n10, assign55410_e90603_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard844 != 0.0)) && (locals.var_guard845 == 0.0)) && (locals.var_guard846 == 0.0)) {
        let assign55410_e90582: f64 = (5.0 / 64.0);
        let assign55410_e90585: f64 = (0.5 * locals.var_t2);
        let assign55410_e90586: f64 = (assign55410_e90582 + assign55410_e90585);
        let assign55410_e90590: f64 = (15.0 / 16.0);
        let assign55410_e90594: f64 = (1.25 - locals.var_t6);
        let assign55410_e90595: f64 = (locals.var_t6 * assign55410_e90594);
        let assign55410_e90596: f64 = (assign55410_e90590 - assign55410_e90595);
        let assign55410_e90597: f64 = (locals.var_t6 * assign55410_e90596);
        let assign55410_e90598: f64 = (assign55410_e90586 + assign55410_e90597);
        let assign55410_e90599: f64 = (locals.var_t5 * assign55410_e90598);
        let assign55410_e90600: f64 = (locals.var_t4 + assign55410_e90599);
        let assign55410_e90601: f64 = { let limited_exp_arg = assign55410_e90600; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign55410_e90601, ({ let limited_exp_arg = assign55410_e90600; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn3 + ((locals.var_t5_dn3 * assign55410_e90598) + (locals.var_t5 * ((0.5 * locals.var_t2_dn3) + ((locals.var_t6_dn3 * assign55410_e90596) + (locals.var_t6 * (-((locals.var_t6_dn3 * assign55410_e90594) + (locals.var_t6 * (-locals.var_t6_dn3))))))))))), ({ let limited_exp_arg = assign55410_e90600; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn4 + ((locals.var_t5_dn4 * assign55410_e90598) + (locals.var_t5 * ((0.5 * locals.var_t2_dn4) + ((locals.var_t6_dn4 * assign55410_e90596) + (locals.var_t6 * (-((locals.var_t6_dn4 * assign55410_e90594) + (locals.var_t6 * (-locals.var_t6_dn4))))))))))), ({ let limited_exp_arg = assign55410_e90600; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn5 + ((locals.var_t5_dn5 * assign55410_e90598) + (locals.var_t5 * ((0.5 * locals.var_t2_dn5) + ((locals.var_t6_dn5 * assign55410_e90596) + (locals.var_t6 * (-((locals.var_t6_dn5 * assign55410_e90594) + (locals.var_t6 * (-locals.var_t6_dn5))))))))))), ({ let limited_exp_arg = assign55410_e90600; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn6 + ((locals.var_t5_dn6 * assign55410_e90598) + (locals.var_t5 * ((0.5 * locals.var_t2_dn6) + ((locals.var_t6_dn6 * assign55410_e90596) + (locals.var_t6 * (-((locals.var_t6_dn6 * assign55410_e90594) + (locals.var_t6 * (-locals.var_t6_dn6))))))))))), ({ let limited_exp_arg = assign55410_e90600; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn7 + ((locals.var_t5_dn7 * assign55410_e90598) + (locals.var_t5 * ((0.5 * locals.var_t2_dn7) + ((locals.var_t6_dn7 * assign55410_e90596) + (locals.var_t6 * (-((locals.var_t6_dn7 * assign55410_e90594) + (locals.var_t6 * (-locals.var_t6_dn7))))))))))), ({ let limited_exp_arg = assign55410_e90600; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn8 + ((locals.var_t5_dn8 * assign55410_e90598) + (locals.var_t5 * ((0.5 * locals.var_t2_dn8) + ((locals.var_t6_dn8 * assign55410_e90596) + (locals.var_t6 * (-((locals.var_t6_dn8 * assign55410_e90594) + (locals.var_t6 * (-locals.var_t6_dn8))))))))))), ({ let limited_exp_arg = assign55410_e90600; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn9 + ((locals.var_t5_dn9 * assign55410_e90598) + (locals.var_t5 * ((0.5 * locals.var_t2_dn9) + ((locals.var_t6_dn9 * assign55410_e90596) + (locals.var_t6 * (-((locals.var_t6_dn9 * assign55410_e90594) + (locals.var_t6 * (-locals.var_t6_dn9))))))))))), ({ let limited_exp_arg = assign55410_e90600; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn10 + ((locals.var_t5_dn10 * assign55410_e90598) + (locals.var_t5 * ((0.5 * locals.var_t2_dn10) + ((locals.var_t6_dn10 * assign55410_e90596) + (locals.var_t6 * (-((locals.var_t6_dn10 * assign55410_e90594) + (locals.var_t6 * (-locals.var_t6_dn10))))))))))), ({ let limited_exp_arg = assign55410_e90600; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn11 + ((locals.var_t5_dn11 * assign55410_e90598) + (locals.var_t5 * ((0.5 * locals.var_t2_dn11) + ((locals.var_t6_dn11 * assign55410_e90596) + (locals.var_t6 * (-((locals.var_t6_dn11 * assign55410_e90594) + (locals.var_t6 * (-locals.var_t6_dn11))))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign55410_e90603;
        locals.var_t3_dn3 = assign55410_e90603_d_n3;
        locals.var_t3_dn4 = assign55410_e90603_d_n4;
        locals.var_t3_dn5 = assign55410_e90603_d_n5;
        locals.var_t3_dn6 = assign55410_e90603_d_n6;
        locals.var_t3_dn7 = assign55410_e90603_d_n7;
        locals.var_t3_dn8 = assign55410_e90603_d_n8;
        locals.var_t3_dn9 = assign55410_e90603_d_n9;
        locals.var_t3_dn10 = assign55410_e90603_d_n10;
        locals.var_t3_dn11 = assign55410_e90603_d_n11;

        let (assign55420_e90635, assign55420_e90635_d_n3, assign55420_e90635_d_n4, assign55420_e90635_d_n5, assign55420_e90635_d_n6, assign55420_e90635_d_n7, assign55420_e90635_d_n8, assign55420_e90635_d_n9, assign55420_e90635_d_n10, assign55420_e90635_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard844 != 0.0)) {
        let assign55420_e90613: f64 = (1.0 + locals.var_t1);
        let assign55420_e90615: f64 = (assign55420_e90613 - locals.var_t8);
        let assign55420_e90618: f64 = (2.0 * locals.var_t0);
        let assign55420_e90621: f64 = (locals.var_t3 * 2.0);
        let assign55420_e90623: f64 = (assign55420_e90621 * locals.var_t0);
        let assign55420_e90626: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign55420_e90627: f64 = (assign55420_e90623 + assign55420_e90626);
        let assign55420_e90628: f64 = (assign55420_e90618 * assign55420_e90627);
        let assign55420_e90630: f64 = (assign55420_e90628).max(1e-38);
        let assign55420_e90631: f64 = (assign55420_e90630).ln();
        let assign55420_e90632: f64 = (assign55420_e90615 - assign55420_e90631);
        let assign55420_e90633: f64 = (locals.var_t3 * assign55420_e90632);
        (assign55420_e90633, ((locals.var_t3_dn3 * assign55420_e90632) + (locals.var_t3 * ((locals.var_t1_dn3 - locals.var_t8_dn3) - (if assign55420_e90628 >= 1e-38 { (((2.0 * locals.var_t0_dn3) * assign55420_e90627) + (assign55420_e90618 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign55420_e90621 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign55420_e90630)))), ((locals.var_t3_dn4 * assign55420_e90632) + (locals.var_t3 * ((locals.var_t1_dn4 - locals.var_t8_dn4) - (if assign55420_e90628 >= 1e-38 { (((2.0 * locals.var_t0_dn4) * assign55420_e90627) + (assign55420_e90618 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign55420_e90621 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign55420_e90630)))), ((locals.var_t3_dn5 * assign55420_e90632) + (locals.var_t3 * ((locals.var_t1_dn5 - locals.var_t8_dn5) - (if assign55420_e90628 >= 1e-38 { (((2.0 * locals.var_t0_dn5) * assign55420_e90627) + (assign55420_e90618 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign55420_e90621 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign55420_e90630)))), ((locals.var_t3_dn6 * assign55420_e90632) + (locals.var_t3 * ((locals.var_t1_dn6 - locals.var_t8_dn6) - (if assign55420_e90628 >= 1e-38 { (((2.0 * locals.var_t0_dn6) * assign55420_e90627) + (assign55420_e90618 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign55420_e90621 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign55420_e90630)))), ((locals.var_t3_dn7 * assign55420_e90632) + (locals.var_t3 * ((locals.var_t1_dn7 - locals.var_t8_dn7) - (if assign55420_e90628 >= 1e-38 { (((2.0 * locals.var_t0_dn7) * assign55420_e90627) + (assign55420_e90618 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign55420_e90621 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign55420_e90630)))), ((locals.var_t3_dn8 * assign55420_e90632) + (locals.var_t3 * ((locals.var_t1_dn8 - locals.var_t8_dn8) - (if assign55420_e90628 >= 1e-38 { (((2.0 * locals.var_t0_dn8) * assign55420_e90627) + (assign55420_e90618 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign55420_e90621 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign55420_e90630)))), ((locals.var_t3_dn9 * assign55420_e90632) + (locals.var_t3 * ((locals.var_t1_dn9 - locals.var_t8_dn9) - (if assign55420_e90628 >= 1e-38 { (((2.0 * locals.var_t0_dn9) * assign55420_e90627) + (assign55420_e90618 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign55420_e90621 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign55420_e90630)))), ((locals.var_t3_dn10 * assign55420_e90632) + (locals.var_t3 * ((locals.var_t1_dn10 - locals.var_t8_dn10) - (if assign55420_e90628 >= 1e-38 { (((2.0 * locals.var_t0_dn10) * assign55420_e90627) + (assign55420_e90618 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign55420_e90621 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign55420_e90630)))), ((locals.var_t3_dn11 * assign55420_e90632) + (locals.var_t3 * ((locals.var_t1_dn11 - locals.var_t8_dn11) - (if assign55420_e90628 >= 1e-38 { (((2.0 * locals.var_t0_dn11) * assign55420_e90627) + (assign55420_e90618 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign55420_e90621 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign55420_e90630)))),)
    } else {
        (locals.var_qs_1, locals.var_qs_1_dn3, locals.var_qs_1_dn4, locals.var_qs_1_dn5, locals.var_qs_1_dn6, locals.var_qs_1_dn7, locals.var_qs_1_dn8, locals.var_qs_1_dn9, locals.var_qs_1_dn10, locals.var_qs_1_dn11,)
    }
};
        locals.var_qs_1 = assign55420_e90635;
        locals.var_qs_1_dn3 = assign55420_e90635_d_n3;
        locals.var_qs_1_dn4 = assign55420_e90635_d_n4;
        locals.var_qs_1_dn5 = assign55420_e90635_d_n5;
        locals.var_qs_1_dn6 = assign55420_e90635_d_n6;
        locals.var_qs_1_dn7 = assign55420_e90635_d_n7;
        locals.var_qs_1_dn8 = assign55420_e90635_d_n8;
        locals.var_qs_1_dn9 = assign55420_e90635_d_n9;
        locals.var_qs_1_dn10 = assign55420_e90635_d_n10;
        locals.var_qs_1_dn11 = assign55420_e90635_d_n11;

        let (assign55430_e90646, assign55430_e90646_d_n3, assign55430_e90646_d_n4, assign55430_e90646_d_n5, assign55430_e90646_d_n6, assign55430_e90646_d_n7, assign55430_e90646_d_n8, assign55430_e90646_d_n9, assign55430_e90646_d_n10, assign55430_e90646_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard844 == 0.0)) {
        let assign55430_e90644: f64 = { let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign55430_e90644, ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn3), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn4), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn5), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn6), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn7), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn8), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn9), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn10), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign55430_e90646;
        locals.var_t3_dn3 = assign55430_e90646_d_n3;
        locals.var_t3_dn4 = assign55430_e90646_d_n4;
        locals.var_t3_dn5 = assign55430_e90646_d_n5;
        locals.var_t3_dn6 = assign55430_e90646_d_n6;
        locals.var_t3_dn7 = assign55430_e90646_d_n7;
        locals.var_t3_dn8 = assign55430_e90646_d_n8;
        locals.var_t3_dn9 = assign55430_e90646_d_n9;
        locals.var_t3_dn10 = assign55430_e90646_d_n10;
        locals.var_t3_dn11 = assign55430_e90646_d_n11;

        let (assign55440_e90658, assign55440_e90658_d_n3, assign55440_e90658_d_n4, assign55440_e90658_d_n5, assign55440_e90658_d_n6, assign55440_e90658_d_n7, assign55440_e90658_d_n8, assign55440_e90658_d_n9, assign55440_e90658_d_n10, assign55440_e90658_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard844 == 0.0)) {
        let assign55440_e90656: f64 = (1.0 / locals.var_sqrtpsisa);
        (assign55440_e90656, (-(locals.var_sqrtpsisa_dn3 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn4 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn5 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn6 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn7 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn8 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn9 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn10 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn11 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))),)
    } else {
        (locals.var_sqrtpsisainv, locals.var_sqrtpsisainv_dn3, locals.var_sqrtpsisainv_dn4, locals.var_sqrtpsisainv_dn5, locals.var_sqrtpsisainv_dn6, locals.var_sqrtpsisainv_dn7, locals.var_sqrtpsisainv_dn8, locals.var_sqrtpsisainv_dn9, locals.var_sqrtpsisainv_dn10, locals.var_sqrtpsisainv_dn11,)
    }
};
        locals.var_sqrtpsisainv = assign55440_e90658;
        locals.var_sqrtpsisainv_dn3 = assign55440_e90658_d_n3;
        locals.var_sqrtpsisainv_dn4 = assign55440_e90658_d_n4;
        locals.var_sqrtpsisainv_dn5 = assign55440_e90658_d_n5;
        locals.var_sqrtpsisainv_dn6 = assign55440_e90658_d_n6;
        locals.var_sqrtpsisainv_dn7 = assign55440_e90658_d_n7;
        locals.var_sqrtpsisainv_dn8 = assign55440_e90658_d_n8;
        locals.var_sqrtpsisainv_dn9 = assign55440_e90658_d_n9;
        locals.var_sqrtpsisainv_dn10 = assign55440_e90658_d_n10;
        locals.var_sqrtpsisainv_dn11 = assign55440_e90658_d_n11;

        let (assign55450_e90691, assign55450_e90691_d_n3, assign55450_e90691_d_n4, assign55450_e90691_d_n5, assign55450_e90691_d_n6, assign55450_e90691_d_n7, assign55450_e90691_d_n8, assign55450_e90691_d_n9, assign55450_e90691_d_n10, assign55450_e90691_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard844 == 0.0)) {
        let assign55450_e90668: f64 = (2.0 * locals.var_t3);
        let assign55450_e90671: f64 = (locals.var_t3 * 2.0);
        let assign55450_e90673: f64 = (assign55450_e90671 * locals.var_t0);
        let assign55450_e90676: f64 = (locals.var_t3 * 2.0);
        let assign55450_e90678: f64 = (assign55450_e90676 * locals.var_t0);
        let assign55450_e90681: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign55450_e90682: f64 = (assign55450_e90678 + assign55450_e90681);
        let assign55450_e90683: f64 = (assign55450_e90673 * assign55450_e90682);
        let assign55450_e90685: f64 = (assign55450_e90683).max(1e-38);
        let assign55450_e90686: f64 = (assign55450_e90685).ln();
        let assign55450_e90687: f64 = (assign55450_e90668 + assign55450_e90686);
        let assign55450_e90689: f64 = (assign55450_e90687 - locals.var_t1);
        (assign55450_e90689, (((2.0 * locals.var_t3_dn3) + (if assign55450_e90683 >= 1e-38 { (((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign55450_e90671 * locals.var_t0_dn3)) * assign55450_e90682) + (assign55450_e90673 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign55450_e90676 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign55450_e90685)) - locals.var_t1_dn3), (((2.0 * locals.var_t3_dn4) + (if assign55450_e90683 >= 1e-38 { (((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign55450_e90671 * locals.var_t0_dn4)) * assign55450_e90682) + (assign55450_e90673 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign55450_e90676 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign55450_e90685)) - locals.var_t1_dn4), (((2.0 * locals.var_t3_dn5) + (if assign55450_e90683 >= 1e-38 { (((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign55450_e90671 * locals.var_t0_dn5)) * assign55450_e90682) + (assign55450_e90673 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign55450_e90676 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign55450_e90685)) - locals.var_t1_dn5), (((2.0 * locals.var_t3_dn6) + (if assign55450_e90683 >= 1e-38 { (((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign55450_e90671 * locals.var_t0_dn6)) * assign55450_e90682) + (assign55450_e90673 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign55450_e90676 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign55450_e90685)) - locals.var_t1_dn6), (((2.0 * locals.var_t3_dn7) + (if assign55450_e90683 >= 1e-38 { (((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign55450_e90671 * locals.var_t0_dn7)) * assign55450_e90682) + (assign55450_e90673 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign55450_e90676 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign55450_e90685)) - locals.var_t1_dn7), (((2.0 * locals.var_t3_dn8) + (if assign55450_e90683 >= 1e-38 { (((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign55450_e90671 * locals.var_t0_dn8)) * assign55450_e90682) + (assign55450_e90673 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign55450_e90676 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign55450_e90685)) - locals.var_t1_dn8), (((2.0 * locals.var_t3_dn9) + (if assign55450_e90683 >= 1e-38 { (((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign55450_e90671 * locals.var_t0_dn9)) * assign55450_e90682) + (assign55450_e90673 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign55450_e90676 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign55450_e90685)) - locals.var_t1_dn9), (((2.0 * locals.var_t3_dn10) + (if assign55450_e90683 >= 1e-38 { (((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign55450_e90671 * locals.var_t0_dn10)) * assign55450_e90682) + (assign55450_e90673 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign55450_e90676 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign55450_e90685)) - locals.var_t1_dn10), (((2.0 * locals.var_t3_dn11) + (if assign55450_e90683 >= 1e-38 { (((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign55450_e90671 * locals.var_t0_dn11)) * assign55450_e90682) + (assign55450_e90673 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign55450_e90676 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign55450_e90685)) - locals.var_t1_dn11),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign55450_e90691;
        locals.var_t4_dn3 = assign55450_e90691_d_n3;
        locals.var_t4_dn4 = assign55450_e90691_d_n4;
        locals.var_t4_dn5 = assign55450_e90691_d_n5;
        locals.var_t4_dn6 = assign55450_e90691_d_n6;
        locals.var_t4_dn7 = assign55450_e90691_d_n7;
        locals.var_t4_dn8 = assign55450_e90691_d_n8;
        locals.var_t4_dn9 = assign55450_e90691_d_n9;
        locals.var_t4_dn10 = assign55450_e90691_d_n10;
        locals.var_t4_dn11 = assign55450_e90691_d_n11;

        let (assign55460_e90715, assign55460_e90715_d_n3, assign55460_e90715_d_n4, assign55460_e90715_d_n5, assign55460_e90715_d_n6, assign55460_e90715_d_n7, assign55460_e90715_d_n8, assign55460_e90715_d_n9, assign55460_e90715_d_n10, assign55460_e90715_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard844 == 0.0)) {
        let assign55460_e90702: f64 = (1.0 / locals.var_t3);
        let assign55460_e90703: f64 = (2.0 + assign55460_e90702);
        let assign55460_e90706: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign55460_e90709: f64 = (locals.var_t0 * locals.var_t3);
        let assign55460_e90711: f64 = (assign55460_e90709 + locals.var_sqrtpsisa);
        let assign55460_e90712: f64 = (assign55460_e90706 / assign55460_e90711);
        let assign55460_e90713: f64 = (assign55460_e90703 + assign55460_e90712);
        (assign55460_e90713, ((-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign55460_e90711) - (assign55460_e90706 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign55460_e90711 * assign55460_e90711))), ((-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign55460_e90711) - (assign55460_e90706 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign55460_e90711 * assign55460_e90711))), ((-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign55460_e90711) - (assign55460_e90706 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign55460_e90711 * assign55460_e90711))), ((-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign55460_e90711) - (assign55460_e90706 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign55460_e90711 * assign55460_e90711))), ((-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign55460_e90711) - (assign55460_e90706 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign55460_e90711 * assign55460_e90711))), ((-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign55460_e90711) - (assign55460_e90706 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign55460_e90711 * assign55460_e90711))), ((-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign55460_e90711) - (assign55460_e90706 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign55460_e90711 * assign55460_e90711))), ((-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign55460_e90711) - (assign55460_e90706 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign55460_e90711 * assign55460_e90711))), ((-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign55460_e90711) - (assign55460_e90706 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign55460_e90711 * assign55460_e90711))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign55460_e90715;
        locals.var_t5_dn3 = assign55460_e90715_d_n3;
        locals.var_t5_dn4 = assign55460_e90715_d_n4;
        locals.var_t5_dn5 = assign55460_e90715_d_n5;
        locals.var_t5_dn6 = assign55460_e90715_d_n6;
        locals.var_t5_dn7 = assign55460_e90715_d_n7;
        locals.var_t5_dn8 = assign55460_e90715_d_n8;
        locals.var_t5_dn9 = assign55460_e90715_d_n9;
        locals.var_t5_dn10 = assign55460_e90715_d_n10;
        locals.var_t5_dn11 = assign55460_e90715_d_n11;

        let (assign55470_e90729, assign55470_e90729_d_n3, assign55470_e90729_d_n4, assign55470_e90729_d_n5, assign55470_e90729_d_n6, assign55470_e90729_d_n7, assign55470_e90729_d_n8, assign55470_e90729_d_n9, assign55470_e90729_d_n10, assign55470_e90729_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard844 == 0.0)) {
        let assign55470_e90726: f64 = (locals.var_t4 / locals.var_t5);
        let assign55470_e90727: f64 = (locals.var_t3 - assign55470_e90726);
        (assign55470_e90727, (locals.var_t3_dn3 - (((locals.var_t4_dn3 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn4 - (((locals.var_t4_dn4 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn5 - (((locals.var_t4_dn5 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn6 - (((locals.var_t4_dn6 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn7 - (((locals.var_t4_dn7 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn8 - (((locals.var_t4_dn8 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn9 - (((locals.var_t4_dn9 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn10 - (((locals.var_t4_dn10 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn11 - (((locals.var_t4_dn11 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign55470_e90729;
        locals.var_t3_dn3 = assign55470_e90729_d_n3;
        locals.var_t3_dn4 = assign55470_e90729_d_n4;
        locals.var_t3_dn5 = assign55470_e90729_d_n5;
        locals.var_t3_dn6 = assign55470_e90729_d_n6;
        locals.var_t3_dn7 = assign55470_e90729_d_n7;
        locals.var_t3_dn8 = assign55470_e90729_d_n8;
        locals.var_t3_dn9 = assign55470_e90729_d_n9;
        locals.var_t3_dn10 = assign55470_e90729_d_n10;
        locals.var_t3_dn11 = assign55470_e90729_d_n11;

        let (assign55480_e90762, assign55480_e90762_d_n3, assign55480_e90762_d_n4, assign55480_e90762_d_n5, assign55480_e90762_d_n6, assign55480_e90762_d_n7, assign55480_e90762_d_n8, assign55480_e90762_d_n9, assign55480_e90762_d_n10, assign55480_e90762_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard844 == 0.0)) {
        let assign55480_e90739: f64 = (2.0 * locals.var_t3);
        let assign55480_e90742: f64 = (locals.var_t3 * 2.0);
        let assign55480_e90744: f64 = (assign55480_e90742 * locals.var_t0);
        let assign55480_e90747: f64 = (locals.var_t3 * 2.0);
        let assign55480_e90749: f64 = (assign55480_e90747 * locals.var_t0);
        let assign55480_e90752: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign55480_e90753: f64 = (assign55480_e90749 + assign55480_e90752);
        let assign55480_e90754: f64 = (assign55480_e90744 * assign55480_e90753);
        let assign55480_e90756: f64 = (assign55480_e90754).max(1e-38);
        let assign55480_e90757: f64 = (assign55480_e90756).ln();
        let assign55480_e90758: f64 = (assign55480_e90739 + assign55480_e90757);
        let assign55480_e90760: f64 = (assign55480_e90758 - locals.var_t1);
        (assign55480_e90760, (((2.0 * locals.var_t3_dn3) + (if assign55480_e90754 >= 1e-38 { (((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign55480_e90742 * locals.var_t0_dn3)) * assign55480_e90753) + (assign55480_e90744 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign55480_e90747 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign55480_e90756)) - locals.var_t1_dn3), (((2.0 * locals.var_t3_dn4) + (if assign55480_e90754 >= 1e-38 { (((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign55480_e90742 * locals.var_t0_dn4)) * assign55480_e90753) + (assign55480_e90744 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign55480_e90747 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign55480_e90756)) - locals.var_t1_dn4), (((2.0 * locals.var_t3_dn5) + (if assign55480_e90754 >= 1e-38 { (((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign55480_e90742 * locals.var_t0_dn5)) * assign55480_e90753) + (assign55480_e90744 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign55480_e90747 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign55480_e90756)) - locals.var_t1_dn5), (((2.0 * locals.var_t3_dn6) + (if assign55480_e90754 >= 1e-38 { (((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign55480_e90742 * locals.var_t0_dn6)) * assign55480_e90753) + (assign55480_e90744 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign55480_e90747 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign55480_e90756)) - locals.var_t1_dn6), (((2.0 * locals.var_t3_dn7) + (if assign55480_e90754 >= 1e-38 { (((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign55480_e90742 * locals.var_t0_dn7)) * assign55480_e90753) + (assign55480_e90744 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign55480_e90747 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign55480_e90756)) - locals.var_t1_dn7), (((2.0 * locals.var_t3_dn8) + (if assign55480_e90754 >= 1e-38 { (((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign55480_e90742 * locals.var_t0_dn8)) * assign55480_e90753) + (assign55480_e90744 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign55480_e90747 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign55480_e90756)) - locals.var_t1_dn8), (((2.0 * locals.var_t3_dn9) + (if assign55480_e90754 >= 1e-38 { (((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign55480_e90742 * locals.var_t0_dn9)) * assign55480_e90753) + (assign55480_e90744 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign55480_e90747 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign55480_e90756)) - locals.var_t1_dn9), (((2.0 * locals.var_t3_dn10) + (if assign55480_e90754 >= 1e-38 { (((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign55480_e90742 * locals.var_t0_dn10)) * assign55480_e90753) + (assign55480_e90744 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign55480_e90747 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign55480_e90756)) - locals.var_t1_dn10), (((2.0 * locals.var_t3_dn11) + (if assign55480_e90754 >= 1e-38 { (((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign55480_e90742 * locals.var_t0_dn11)) * assign55480_e90753) + (assign55480_e90744 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign55480_e90747 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign55480_e90756)) - locals.var_t1_dn11),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign55480_e90762;
        locals.var_t4_dn3 = assign55480_e90762_d_n3;
        locals.var_t4_dn4 = assign55480_e90762_d_n4;
        locals.var_t4_dn5 = assign55480_e90762_d_n5;
        locals.var_t4_dn6 = assign55480_e90762_d_n6;
        locals.var_t4_dn7 = assign55480_e90762_d_n7;
        locals.var_t4_dn8 = assign55480_e90762_d_n8;
        locals.var_t4_dn9 = assign55480_e90762_d_n9;
        locals.var_t4_dn10 = assign55480_e90762_d_n10;
        locals.var_t4_dn11 = assign55480_e90762_d_n11;

        let (assign55490_e90786, assign55490_e90786_d_n3, assign55490_e90786_d_n4, assign55490_e90786_d_n5, assign55490_e90786_d_n6, assign55490_e90786_d_n7, assign55490_e90786_d_n8, assign55490_e90786_d_n9, assign55490_e90786_d_n10, assign55490_e90786_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard844 == 0.0)) {
        let assign55490_e90773: f64 = (1.0 / locals.var_t3);
        let assign55490_e90774: f64 = (2.0 + assign55490_e90773);
        let assign55490_e90777: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign55490_e90780: f64 = (locals.var_t0 * locals.var_t3);
        let assign55490_e90782: f64 = (assign55490_e90780 + locals.var_sqrtpsisa);
        let assign55490_e90783: f64 = (assign55490_e90777 / assign55490_e90782);
        let assign55490_e90784: f64 = (assign55490_e90774 + assign55490_e90783);
        (assign55490_e90784, ((-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign55490_e90782) - (assign55490_e90777 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign55490_e90782 * assign55490_e90782))), ((-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign55490_e90782) - (assign55490_e90777 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign55490_e90782 * assign55490_e90782))), ((-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign55490_e90782) - (assign55490_e90777 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign55490_e90782 * assign55490_e90782))), ((-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign55490_e90782) - (assign55490_e90777 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign55490_e90782 * assign55490_e90782))), ((-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign55490_e90782) - (assign55490_e90777 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign55490_e90782 * assign55490_e90782))), ((-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign55490_e90782) - (assign55490_e90777 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign55490_e90782 * assign55490_e90782))), ((-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign55490_e90782) - (assign55490_e90777 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign55490_e90782 * assign55490_e90782))), ((-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign55490_e90782) - (assign55490_e90777 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign55490_e90782 * assign55490_e90782))), ((-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign55490_e90782) - (assign55490_e90777 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign55490_e90782 * assign55490_e90782))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign55490_e90786;
        locals.var_t5_dn3 = assign55490_e90786_d_n3;
        locals.var_t5_dn4 = assign55490_e90786_d_n4;
        locals.var_t5_dn5 = assign55490_e90786_d_n5;
        locals.var_t5_dn6 = assign55490_e90786_d_n6;
        locals.var_t5_dn7 = assign55490_e90786_d_n7;
        locals.var_t5_dn8 = assign55490_e90786_d_n8;
        locals.var_t5_dn9 = assign55490_e90786_d_n9;
        locals.var_t5_dn10 = assign55490_e90786_d_n10;
        locals.var_t5_dn11 = assign55490_e90786_d_n11;

        let (assign55500_e90814, assign55500_e90814_d_n3, assign55500_e90814_d_n4, assign55500_e90814_d_n5, assign55500_e90814_d_n6, assign55500_e90814_d_n7, assign55500_e90814_d_n8, assign55500_e90814_d_n9, assign55500_e90814_d_n10, assign55500_e90814_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard844 == 0.0)) {
        let assign55500_e90796: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign55500_e90799: f64 = (locals.var_t0 * locals.var_t3);
        let assign55500_e90801: f64 = (assign55500_e90799 + locals.var_sqrtpsisa);
        let assign55500_e90802: f64 = (assign55500_e90796 / assign55500_e90801);
        let assign55500_e90805: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign55500_e90808: f64 = (locals.var_t0 * locals.var_t3);
        let assign55500_e90810: f64 = (assign55500_e90808 + locals.var_sqrtpsisa);
        let assign55500_e90811: f64 = (assign55500_e90805 / assign55500_e90810);
        let assign55500_e90812: f64 = (assign55500_e90802 * assign55500_e90811);
        (assign55500_e90812, ((((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign55500_e90801) - (assign55500_e90796 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign55500_e90801 * assign55500_e90801)) * assign55500_e90811) + (assign55500_e90802 * ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign55500_e90810) - (assign55500_e90805 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign55500_e90810 * assign55500_e90810)))), ((((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign55500_e90801) - (assign55500_e90796 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign55500_e90801 * assign55500_e90801)) * assign55500_e90811) + (assign55500_e90802 * ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign55500_e90810) - (assign55500_e90805 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign55500_e90810 * assign55500_e90810)))), ((((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign55500_e90801) - (assign55500_e90796 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign55500_e90801 * assign55500_e90801)) * assign55500_e90811) + (assign55500_e90802 * ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign55500_e90810) - (assign55500_e90805 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign55500_e90810 * assign55500_e90810)))), ((((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign55500_e90801) - (assign55500_e90796 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign55500_e90801 * assign55500_e90801)) * assign55500_e90811) + (assign55500_e90802 * ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign55500_e90810) - (assign55500_e90805 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign55500_e90810 * assign55500_e90810)))), ((((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign55500_e90801) - (assign55500_e90796 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign55500_e90801 * assign55500_e90801)) * assign55500_e90811) + (assign55500_e90802 * ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign55500_e90810) - (assign55500_e90805 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign55500_e90810 * assign55500_e90810)))), ((((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign55500_e90801) - (assign55500_e90796 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign55500_e90801 * assign55500_e90801)) * assign55500_e90811) + (assign55500_e90802 * ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign55500_e90810) - (assign55500_e90805 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign55500_e90810 * assign55500_e90810)))), ((((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign55500_e90801) - (assign55500_e90796 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign55500_e90801 * assign55500_e90801)) * assign55500_e90811) + (assign55500_e90802 * ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign55500_e90810) - (assign55500_e90805 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign55500_e90810 * assign55500_e90810)))), ((((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign55500_e90801) - (assign55500_e90796 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign55500_e90801 * assign55500_e90801)) * assign55500_e90811) + (assign55500_e90802 * ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign55500_e90810) - (assign55500_e90805 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign55500_e90810 * assign55500_e90810)))), ((((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign55500_e90801) - (assign55500_e90796 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign55500_e90801 * assign55500_e90801)) * assign55500_e90811) + (assign55500_e90802 * ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign55500_e90810) - (assign55500_e90805 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign55500_e90810 * assign55500_e90810)))),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign55500_e90814;
        locals.var_t6_dn3 = assign55500_e90814_d_n3;
        locals.var_t6_dn4 = assign55500_e90814_d_n4;
        locals.var_t6_dn5 = assign55500_e90814_d_n5;
        locals.var_t6_dn6 = assign55500_e90814_d_n6;
        locals.var_t6_dn7 = assign55500_e90814_d_n7;
        locals.var_t6_dn8 = assign55500_e90814_d_n8;
        locals.var_t6_dn9 = assign55500_e90814_d_n9;
        locals.var_t6_dn10 = assign55500_e90814_d_n10;
        locals.var_t6_dn11 = assign55500_e90814_d_n11;

        let (assign55510_e90847, assign55510_e90847_d_n3, assign55510_e90847_d_n4, assign55510_e90847_d_n5, assign55510_e90847_d_n6, assign55510_e90847_d_n7, assign55510_e90847_d_n8, assign55510_e90847_d_n9, assign55510_e90847_d_n10, assign55510_e90847_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard844 == 0.0)) {
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_t3;
        let assign55510_e90824: f64 = (1.0 * __rspice_inv_cse_0);
        let assign55510_e90827: f64 = (1.0 * __rspice_inv_cse_0);
        let assign55510_e90828: f64 = (assign55510_e90824 * assign55510_e90827);
        let assign55510_e90829: f64 = (-assign55510_e90828);
        let assign55510_e90833: f64 = (locals.var_sqrtpsisa * locals.var_sqrtpsisa);
        let assign55510_e90835: f64 = (assign55510_e90833 * locals.var_sqrtpsisa);
        let assign55510_e90838: f64 = (locals.var_t0 * locals.var_t3);
        let assign55510_e90840: f64 = (assign55510_e90838 + locals.var_sqrtpsisa);
        let assign55510_e90841: f64 = (assign55510_e90835 * assign55510_e90840);
        let assign55510_e90842: f64 = (1.0 / assign55510_e90841);
        let assign55510_e90843: f64 = (assign55510_e90829 - assign55510_e90842);
        let assign55510_e90845: f64 = (assign55510_e90843 - locals.var_t6);
        (assign55510_e90845, (((-(((-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3))) * assign55510_e90827) + (assign55510_e90824 * (-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn3 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn3)) * locals.var_sqrtpsisa) + (assign55510_e90833 * locals.var_sqrtpsisa_dn3)) * assign55510_e90840) + (assign55510_e90835 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign55510_e90841 * assign55510_e90841)))) - locals.var_t6_dn3), (((-(((-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))) * assign55510_e90827) + (assign55510_e90824 * (-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn4 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn4)) * locals.var_sqrtpsisa) + (assign55510_e90833 * locals.var_sqrtpsisa_dn4)) * assign55510_e90840) + (assign55510_e90835 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign55510_e90841 * assign55510_e90841)))) - locals.var_t6_dn4), (((-(((-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))) * assign55510_e90827) + (assign55510_e90824 * (-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn5 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn5)) * locals.var_sqrtpsisa) + (assign55510_e90833 * locals.var_sqrtpsisa_dn5)) * assign55510_e90840) + (assign55510_e90835 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign55510_e90841 * assign55510_e90841)))) - locals.var_t6_dn5), (((-(((-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))) * assign55510_e90827) + (assign55510_e90824 * (-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn6 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn6)) * locals.var_sqrtpsisa) + (assign55510_e90833 * locals.var_sqrtpsisa_dn6)) * assign55510_e90840) + (assign55510_e90835 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign55510_e90841 * assign55510_e90841)))) - locals.var_t6_dn6), (((-(((-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))) * assign55510_e90827) + (assign55510_e90824 * (-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn7 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn7)) * locals.var_sqrtpsisa) + (assign55510_e90833 * locals.var_sqrtpsisa_dn7)) * assign55510_e90840) + (assign55510_e90835 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign55510_e90841 * assign55510_e90841)))) - locals.var_t6_dn7), (((-(((-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))) * assign55510_e90827) + (assign55510_e90824 * (-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn8 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn8)) * locals.var_sqrtpsisa) + (assign55510_e90833 * locals.var_sqrtpsisa_dn8)) * assign55510_e90840) + (assign55510_e90835 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign55510_e90841 * assign55510_e90841)))) - locals.var_t6_dn8), (((-(((-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))) * assign55510_e90827) + (assign55510_e90824 * (-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn9 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn9)) * locals.var_sqrtpsisa) + (assign55510_e90833 * locals.var_sqrtpsisa_dn9)) * assign55510_e90840) + (assign55510_e90835 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign55510_e90841 * assign55510_e90841)))) - locals.var_t6_dn9), (((-(((-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))) * assign55510_e90827) + (assign55510_e90824 * (-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn10 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn10)) * locals.var_sqrtpsisa) + (assign55510_e90833 * locals.var_sqrtpsisa_dn10)) * assign55510_e90840) + (assign55510_e90835 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign55510_e90841 * assign55510_e90841)))) - locals.var_t6_dn10), (((-(((-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))) * assign55510_e90827) + (assign55510_e90824 * (-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn11 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn11)) * locals.var_sqrtpsisa) + (assign55510_e90833 * locals.var_sqrtpsisa_dn11)) * assign55510_e90840) + (assign55510_e90835 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign55510_e90841 * assign55510_e90841)))) - locals.var_t6_dn11),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign55510_e90847;
        locals.var_t7_dn3 = assign55510_e90847_d_n3;
        locals.var_t7_dn4 = assign55510_e90847_d_n4;
        locals.var_t7_dn5 = assign55510_e90847_d_n5;
        locals.var_t7_dn6 = assign55510_e90847_d_n6;
        locals.var_t7_dn7 = assign55510_e90847_d_n7;
        locals.var_t7_dn8 = assign55510_e90847_d_n8;
        locals.var_t7_dn9 = assign55510_e90847_d_n9;
        locals.var_t7_dn10 = assign55510_e90847_d_n10;
        locals.var_t7_dn11 = assign55510_e90847_d_n11;

        let (assign55520_e90873, assign55520_e90873_d_n3, assign55520_e90873_d_n4, assign55520_e90873_d_n5, assign55520_e90873_d_n6, assign55520_e90873_d_n7, assign55520_e90873_d_n8, assign55520_e90873_d_n9, assign55520_e90873_d_n10, assign55520_e90873_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard844 == 0.0)) {
        let assign55520_e90858: f64 = (locals.var_t4 / locals.var_t5);
        let assign55520_e90862: f64 = (locals.var_t4 * locals.var_t7);
        let assign55520_e90865: f64 = (2.0 * locals.var_t5);
        let assign55520_e90867: f64 = (assign55520_e90865 * locals.var_t5);
        let assign55520_e90868: f64 = (assign55520_e90862 / assign55520_e90867);
        let assign55520_e90869: f64 = (1.0 + assign55520_e90868);
        let assign55520_e90870: f64 = (assign55520_e90858 * assign55520_e90869);
        let assign55520_e90871: f64 = (locals.var_t3 - assign55520_e90870);
        (assign55520_e90871, (locals.var_t3_dn3 - (((((locals.var_t4_dn3 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5)) * assign55520_e90869) + (assign55520_e90858 * (((((locals.var_t4_dn3 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn3)) * assign55520_e90867) - (assign55520_e90862 * (((2.0 * locals.var_t5_dn3) * locals.var_t5) + (assign55520_e90865 * locals.var_t5_dn3)))) / (assign55520_e90867 * assign55520_e90867))))), (locals.var_t3_dn4 - (((((locals.var_t4_dn4 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)) * assign55520_e90869) + (assign55520_e90858 * (((((locals.var_t4_dn4 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn4)) * assign55520_e90867) - (assign55520_e90862 * (((2.0 * locals.var_t5_dn4) * locals.var_t5) + (assign55520_e90865 * locals.var_t5_dn4)))) / (assign55520_e90867 * assign55520_e90867))))), (locals.var_t3_dn5 - (((((locals.var_t4_dn5 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)) * assign55520_e90869) + (assign55520_e90858 * (((((locals.var_t4_dn5 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn5)) * assign55520_e90867) - (assign55520_e90862 * (((2.0 * locals.var_t5_dn5) * locals.var_t5) + (assign55520_e90865 * locals.var_t5_dn5)))) / (assign55520_e90867 * assign55520_e90867))))), (locals.var_t3_dn6 - (((((locals.var_t4_dn6 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5)) * assign55520_e90869) + (assign55520_e90858 * (((((locals.var_t4_dn6 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn6)) * assign55520_e90867) - (assign55520_e90862 * (((2.0 * locals.var_t5_dn6) * locals.var_t5) + (assign55520_e90865 * locals.var_t5_dn6)))) / (assign55520_e90867 * assign55520_e90867))))), (locals.var_t3_dn7 - (((((locals.var_t4_dn7 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5)) * assign55520_e90869) + (assign55520_e90858 * (((((locals.var_t4_dn7 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn7)) * assign55520_e90867) - (assign55520_e90862 * (((2.0 * locals.var_t5_dn7) * locals.var_t5) + (assign55520_e90865 * locals.var_t5_dn7)))) / (assign55520_e90867 * assign55520_e90867))))), (locals.var_t3_dn8 - (((((locals.var_t4_dn8 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5)) * assign55520_e90869) + (assign55520_e90858 * (((((locals.var_t4_dn8 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn8)) * assign55520_e90867) - (assign55520_e90862 * (((2.0 * locals.var_t5_dn8) * locals.var_t5) + (assign55520_e90865 * locals.var_t5_dn8)))) / (assign55520_e90867 * assign55520_e90867))))), (locals.var_t3_dn9 - (((((locals.var_t4_dn9 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5)) * assign55520_e90869) + (assign55520_e90858 * (((((locals.var_t4_dn9 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn9)) * assign55520_e90867) - (assign55520_e90862 * (((2.0 * locals.var_t5_dn9) * locals.var_t5) + (assign55520_e90865 * locals.var_t5_dn9)))) / (assign55520_e90867 * assign55520_e90867))))), (locals.var_t3_dn10 - (((((locals.var_t4_dn10 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5)) * assign55520_e90869) + (assign55520_e90858 * (((((locals.var_t4_dn10 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn10)) * assign55520_e90867) - (assign55520_e90862 * (((2.0 * locals.var_t5_dn10) * locals.var_t5) + (assign55520_e90865 * locals.var_t5_dn10)))) / (assign55520_e90867 * assign55520_e90867))))), (locals.var_t3_dn11 - (((((locals.var_t4_dn11 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5)) * assign55520_e90869) + (assign55520_e90858 * (((((locals.var_t4_dn11 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn11)) * assign55520_e90867) - (assign55520_e90862 * (((2.0 * locals.var_t5_dn11) * locals.var_t5) + (assign55520_e90865 * locals.var_t5_dn11)))) / (assign55520_e90867 * assign55520_e90867))))),)
    } else {
        (locals.var_qs_1, locals.var_qs_1_dn3, locals.var_qs_1_dn4, locals.var_qs_1_dn5, locals.var_qs_1_dn6, locals.var_qs_1_dn7, locals.var_qs_1_dn8, locals.var_qs_1_dn9, locals.var_qs_1_dn10, locals.var_qs_1_dn11,)
    }
};
        locals.var_qs_1 = assign55520_e90873;
        locals.var_qs_1_dn3 = assign55520_e90873_d_n3;
        locals.var_qs_1_dn4 = assign55520_e90873_d_n4;
        locals.var_qs_1_dn5 = assign55520_e90873_d_n5;
        locals.var_qs_1_dn6 = assign55520_e90873_d_n6;
        locals.var_qs_1_dn7 = assign55520_e90873_d_n7;
        locals.var_qs_1_dn8 = assign55520_e90873_d_n8;
        locals.var_qs_1_dn9 = assign55520_e90873_d_n9;
        locals.var_qs_1_dn10 = assign55520_e90873_d_n10;
        locals.var_qs_1_dn11 = assign55520_e90873_d_n11;

        let (assign55530_e90899, assign55530_e90899_d_n3, assign55530_e90899_d_n4, assign55530_e90899_d_n5, assign55530_e90899_d_n6, assign55530_e90899_d_n7, assign55530_e90899_d_n8, assign55530_e90899_d_n9, assign55530_e90899_d_n10, assign55530_e90899_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55530_e90881: f64 = (locals.var_psip + 1.0);
        let assign55530_e90884: f64 = (locals.var_psip - 1.0);
        let assign55530_e90887: f64 = (locals.var_psip - 1.0);
        let assign55530_e90888: f64 = (assign55530_e90884 * assign55530_e90887);
        let assign55530_e90891: f64 = (0.25 * 2.0);
        let assign55530_e90893: f64 = (assign55530_e90891 * 2.0);
        let assign55530_e90894: f64 = (assign55530_e90888 + assign55530_e90893);
        let assign55530_e90895: f64 = (assign55530_e90894).sqrt();
        let assign55530_e90896: f64 = (assign55530_e90881 + assign55530_e90895);
        let assign55530_e90897: f64 = (0.5 * assign55530_e90896);
        (assign55530_e90897, (0.5 * (locals.var_psip_dn3 + (((locals.var_psip_dn3 * assign55530_e90887) + (assign55530_e90884 * locals.var_psip_dn3)) / (2.0 * assign55530_e90895)))), (0.5 * (locals.var_psip_dn4 + (((locals.var_psip_dn4 * assign55530_e90887) + (assign55530_e90884 * locals.var_psip_dn4)) / (2.0 * assign55530_e90895)))), (0.5 * (locals.var_psip_dn5 + (((locals.var_psip_dn5 * assign55530_e90887) + (assign55530_e90884 * locals.var_psip_dn5)) / (2.0 * assign55530_e90895)))), (0.5 * (locals.var_psip_dn6 + (((locals.var_psip_dn6 * assign55530_e90887) + (assign55530_e90884 * locals.var_psip_dn6)) / (2.0 * assign55530_e90895)))), (0.5 * (locals.var_psip_dn7 + (((locals.var_psip_dn7 * assign55530_e90887) + (assign55530_e90884 * locals.var_psip_dn7)) / (2.0 * assign55530_e90895)))), (0.5 * (locals.var_psip_dn8 + (((locals.var_psip_dn8 * assign55530_e90887) + (assign55530_e90884 * locals.var_psip_dn8)) / (2.0 * assign55530_e90895)))), (0.5 * (locals.var_psip_dn9 + (((locals.var_psip_dn9 * assign55530_e90887) + (assign55530_e90884 * locals.var_psip_dn9)) / (2.0 * assign55530_e90895)))), (0.5 * (locals.var_psip_dn10 + (((locals.var_psip_dn10 * assign55530_e90887) + (assign55530_e90884 * locals.var_psip_dn10)) / (2.0 * assign55530_e90895)))), (0.5 * (locals.var_psip_dn11 + (((locals.var_psip_dn11 * assign55530_e90887) + (assign55530_e90884 * locals.var_psip_dn11)) / (2.0 * assign55530_e90895)))),)
    } else {
        (locals.var_psipclamp, locals.var_psipclamp_dn3, locals.var_psipclamp_dn4, locals.var_psipclamp_dn5, locals.var_psipclamp_dn6, locals.var_psipclamp_dn7, locals.var_psipclamp_dn8, locals.var_psipclamp_dn9, locals.var_psipclamp_dn10, locals.var_psipclamp_dn11,)
    }
};
        locals.var_psipclamp = assign55530_e90899;
        locals.var_psipclamp_dn3 = assign55530_e90899_d_n3;
        locals.var_psipclamp_dn4 = assign55530_e90899_d_n4;
        locals.var_psipclamp_dn5 = assign55530_e90899_d_n5;
        locals.var_psipclamp_dn6 = assign55530_e90899_d_n6;
        locals.var_psipclamp_dn7 = assign55530_e90899_d_n7;
        locals.var_psipclamp_dn8 = assign55530_e90899_d_n8;
        locals.var_psipclamp_dn9 = assign55530_e90899_d_n9;
        locals.var_psipclamp_dn10 = assign55530_e90899_d_n10;
        locals.var_psipclamp_dn11 = assign55530_e90899_d_n11;

    }

    pub(super) fn stamp_transient_block_188(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign55540_e90907, assign55540_e90907_d_n3, assign55540_e90907_d_n4, assign55540_e90907_d_n5, assign55540_e90907_d_n6, assign55540_e90907_d_n7, assign55540_e90907_d_n8, assign55540_e90907_d_n9, assign55540_e90907_d_n10, assign55540_e90907_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55540_e90905: f64 = (locals.var_psipclamp).sqrt();
        (assign55540_e90905, (locals.var_psipclamp_dn3 / (2.0 * assign55540_e90905)), (locals.var_psipclamp_dn4 / (2.0 * assign55540_e90905)), (locals.var_psipclamp_dn5 / (2.0 * assign55540_e90905)), (locals.var_psipclamp_dn6 / (2.0 * assign55540_e90905)), (locals.var_psipclamp_dn7 / (2.0 * assign55540_e90905)), (locals.var_psipclamp_dn8 / (2.0 * assign55540_e90905)), (locals.var_psipclamp_dn9 / (2.0 * assign55540_e90905)), (locals.var_psipclamp_dn10 / (2.0 * assign55540_e90905)), (locals.var_psipclamp_dn11 / (2.0 * assign55540_e90905)),)
    } else {
        (locals.var_sqrtpsip, locals.var_sqrtpsip_dn3, locals.var_sqrtpsip_dn4, locals.var_sqrtpsip_dn5, locals.var_sqrtpsip_dn6, locals.var_sqrtpsip_dn7, locals.var_sqrtpsip_dn8, locals.var_sqrtpsip_dn9, locals.var_sqrtpsip_dn10, locals.var_sqrtpsip_dn11,)
    }
};
        locals.var_sqrtpsip = assign55540_e90907;
        locals.var_sqrtpsip_dn3 = assign55540_e90907_d_n3;
        locals.var_sqrtpsip_dn4 = assign55540_e90907_d_n4;
        locals.var_sqrtpsip_dn5 = assign55540_e90907_d_n5;
        locals.var_sqrtpsip_dn6 = assign55540_e90907_d_n6;
        locals.var_sqrtpsip_dn7 = assign55540_e90907_d_n7;
        locals.var_sqrtpsip_dn8 = assign55540_e90907_d_n8;
        locals.var_sqrtpsip_dn9 = assign55540_e90907_d_n9;
        locals.var_sqrtpsip_dn10 = assign55540_e90907_d_n10;
        locals.var_sqrtpsip_dn11 = assign55540_e90907_d_n11;

        let (assign55550_e90918, assign55550_e90918_d_n3, assign55550_e90918_d_n4, assign55550_e90918_d_n5, assign55550_e90918_d_n6, assign55550_e90918_d_n7, assign55550_e90918_d_n8, assign55550_e90918_d_n9, assign55550_e90918_d_n10, assign55550_e90918_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55550_e90915: f64 = (2.0 * locals.var_qs_1);
        let assign55550_e90916: f64 = (locals.var_psip - assign55550_e90915);
        (assign55550_e90916, (locals.var_psip_dn3 - (2.0 * locals.var_qs_1_dn3)), (locals.var_psip_dn4 - (2.0 * locals.var_qs_1_dn4)), (locals.var_psip_dn5 - (2.0 * locals.var_qs_1_dn5)), (locals.var_psip_dn6 - (2.0 * locals.var_qs_1_dn6)), (locals.var_psip_dn7 - (2.0 * locals.var_qs_1_dn7)), (locals.var_psip_dn8 - (2.0 * locals.var_qs_1_dn8)), (locals.var_psip_dn9 - (2.0 * locals.var_qs_1_dn9)), (locals.var_psip_dn10 - (2.0 * locals.var_qs_1_dn10)), (locals.var_psip_dn11 - (2.0 * locals.var_qs_1_dn11)),)
    } else {
        (locals.var_psiavg, locals.var_psiavg_dn3, locals.var_psiavg_dn4, locals.var_psiavg_dn5, locals.var_psiavg_dn6, locals.var_psiavg_dn7, locals.var_psiavg_dn8, locals.var_psiavg_dn9, locals.var_psiavg_dn10, locals.var_psiavg_dn11,)
    }
};
        locals.var_psiavg = assign55550_e90918;
        locals.var_psiavg_dn3 = assign55550_e90918_d_n3;
        locals.var_psiavg_dn4 = assign55550_e90918_d_n4;
        locals.var_psiavg_dn5 = assign55550_e90918_d_n5;
        locals.var_psiavg_dn6 = assign55550_e90918_d_n6;
        locals.var_psiavg_dn7 = assign55550_e90918_d_n7;
        locals.var_psiavg_dn8 = assign55550_e90918_d_n8;
        locals.var_psiavg_dn9 = assign55550_e90918_d_n9;
        locals.var_psiavg_dn10 = assign55550_e90918_d_n10;
        locals.var_psiavg_dn11 = assign55550_e90918_d_n11;

        let (assign55560_e90944, assign55560_e90944_d_n3, assign55560_e90944_d_n4, assign55560_e90944_d_n5, assign55560_e90944_d_n6, assign55560_e90944_d_n7, assign55560_e90944_d_n8, assign55560_e90944_d_n9, assign55560_e90944_d_n10, assign55560_e90944_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55560_e90926: f64 = (locals.var_psiavg + 1.0);
        let assign55560_e90929: f64 = (locals.var_psiavg - 1.0);
        let assign55560_e90932: f64 = (locals.var_psiavg - 1.0);
        let assign55560_e90933: f64 = (assign55560_e90929 * assign55560_e90932);
        let assign55560_e90936: f64 = (0.25 * 2.0);
        let assign55560_e90938: f64 = (assign55560_e90936 * 2.0);
        let assign55560_e90939: f64 = (assign55560_e90933 + assign55560_e90938);
        let assign55560_e90940: f64 = (assign55560_e90939).sqrt();
        let assign55560_e90941: f64 = (assign55560_e90926 + assign55560_e90940);
        let assign55560_e90942: f64 = (0.5 * assign55560_e90941);
        (assign55560_e90942, (0.5 * (locals.var_psiavg_dn3 + (((locals.var_psiavg_dn3 * assign55560_e90932) + (assign55560_e90929 * locals.var_psiavg_dn3)) / (2.0 * assign55560_e90940)))), (0.5 * (locals.var_psiavg_dn4 + (((locals.var_psiavg_dn4 * assign55560_e90932) + (assign55560_e90929 * locals.var_psiavg_dn4)) / (2.0 * assign55560_e90940)))), (0.5 * (locals.var_psiavg_dn5 + (((locals.var_psiavg_dn5 * assign55560_e90932) + (assign55560_e90929 * locals.var_psiavg_dn5)) / (2.0 * assign55560_e90940)))), (0.5 * (locals.var_psiavg_dn6 + (((locals.var_psiavg_dn6 * assign55560_e90932) + (assign55560_e90929 * locals.var_psiavg_dn6)) / (2.0 * assign55560_e90940)))), (0.5 * (locals.var_psiavg_dn7 + (((locals.var_psiavg_dn7 * assign55560_e90932) + (assign55560_e90929 * locals.var_psiavg_dn7)) / (2.0 * assign55560_e90940)))), (0.5 * (locals.var_psiavg_dn8 + (((locals.var_psiavg_dn8 * assign55560_e90932) + (assign55560_e90929 * locals.var_psiavg_dn8)) / (2.0 * assign55560_e90940)))), (0.5 * (locals.var_psiavg_dn9 + (((locals.var_psiavg_dn9 * assign55560_e90932) + (assign55560_e90929 * locals.var_psiavg_dn9)) / (2.0 * assign55560_e90940)))), (0.5 * (locals.var_psiavg_dn10 + (((locals.var_psiavg_dn10 * assign55560_e90932) + (assign55560_e90929 * locals.var_psiavg_dn10)) / (2.0 * assign55560_e90940)))), (0.5 * (locals.var_psiavg_dn11 + (((locals.var_psiavg_dn11 * assign55560_e90932) + (assign55560_e90929 * locals.var_psiavg_dn11)) / (2.0 * assign55560_e90940)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign55560_e90944;
        locals.var_t0_dn3 = assign55560_e90944_d_n3;
        locals.var_t0_dn4 = assign55560_e90944_d_n4;
        locals.var_t0_dn5 = assign55560_e90944_d_n5;
        locals.var_t0_dn6 = assign55560_e90944_d_n6;
        locals.var_t0_dn7 = assign55560_e90944_d_n7;
        locals.var_t0_dn8 = assign55560_e90944_d_n8;
        locals.var_t0_dn9 = assign55560_e90944_d_n9;
        locals.var_t0_dn10 = assign55560_e90944_d_n10;
        locals.var_t0_dn11 = assign55560_e90944_d_n11;

        let (assign55570_e90958, assign55570_e90958_d_n3, assign55570_e90958_d_n4, assign55570_e90958_d_n5, assign55570_e90958_d_n6, assign55570_e90958_d_n7, assign55570_e90958_d_n8, assign55570_e90958_d_n9, assign55570_e90958_d_n10, assign55570_e90958_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55570_e90953: f64 = (locals.var_t0).sqrt();
        let assign55570_e90954: f64 = (locals.var_sqrtpsip + assign55570_e90953);
        let assign55570_e90955: f64 = (locals.var_gamcv / assign55570_e90954);
        let assign55570_e90956: f64 = (1.0 + assign55570_e90955);
        (assign55570_e90956, (((locals.var_gamcv_dn3 * assign55570_e90954) - (locals.var_gamcv * (locals.var_sqrtpsip_dn3 + (locals.var_t0_dn3 / (2.0 * assign55570_e90953))))) / (assign55570_e90954 * assign55570_e90954)), (((locals.var_gamcv_dn4 * assign55570_e90954) - (locals.var_gamcv * (locals.var_sqrtpsip_dn4 + (locals.var_t0_dn4 / (2.0 * assign55570_e90953))))) / (assign55570_e90954 * assign55570_e90954)), (((locals.var_gamcv_dn5 * assign55570_e90954) - (locals.var_gamcv * (locals.var_sqrtpsip_dn5 + (locals.var_t0_dn5 / (2.0 * assign55570_e90953))))) / (assign55570_e90954 * assign55570_e90954)), (((locals.var_gamcv_dn6 * assign55570_e90954) - (locals.var_gamcv * (locals.var_sqrtpsip_dn6 + (locals.var_t0_dn6 / (2.0 * assign55570_e90953))))) / (assign55570_e90954 * assign55570_e90954)), (((locals.var_gamcv_dn7 * assign55570_e90954) - (locals.var_gamcv * (locals.var_sqrtpsip_dn7 + (locals.var_t0_dn7 / (2.0 * assign55570_e90953))))) / (assign55570_e90954 * assign55570_e90954)), (((locals.var_gamcv_dn8 * assign55570_e90954) - (locals.var_gamcv * (locals.var_sqrtpsip_dn8 + (locals.var_t0_dn8 / (2.0 * assign55570_e90953))))) / (assign55570_e90954 * assign55570_e90954)), (((locals.var_gamcv_dn9 * assign55570_e90954) - (locals.var_gamcv * (locals.var_sqrtpsip_dn9 + (locals.var_t0_dn9 / (2.0 * assign55570_e90953))))) / (assign55570_e90954 * assign55570_e90954)), (((locals.var_gamcv_dn10 * assign55570_e90954) - (locals.var_gamcv * (locals.var_sqrtpsip_dn10 + (locals.var_t0_dn10 / (2.0 * assign55570_e90953))))) / (assign55570_e90954 * assign55570_e90954)), (((locals.var_gamcv_dn11 * assign55570_e90954) - (locals.var_gamcv * (locals.var_sqrtpsip_dn11 + (locals.var_t0_dn11 / (2.0 * assign55570_e90953))))) / (assign55570_e90954 * assign55570_e90954)),)
    } else {
        (locals.var_nq, locals.var_nq_dn3, locals.var_nq_dn4, locals.var_nq_dn5, locals.var_nq_dn6, locals.var_nq_dn7, locals.var_nq_dn8, locals.var_nq_dn9, locals.var_nq_dn10, locals.var_nq_dn11,)
    }
};
        locals.var_nq = assign55570_e90958;
        locals.var_nq_dn3 = assign55570_e90958_d_n3;
        locals.var_nq_dn4 = assign55570_e90958_d_n4;
        locals.var_nq_dn5 = assign55570_e90958_d_n5;
        locals.var_nq_dn6 = assign55570_e90958_d_n6;
        locals.var_nq_dn7 = assign55570_e90958_d_n7;
        locals.var_nq_dn8 = assign55570_e90958_d_n8;
        locals.var_nq_dn9 = assign55570_e90958_d_n9;
        locals.var_nq_dn10 = assign55570_e90958_d_n10;
        locals.var_nq_dn11 = assign55570_e90958_d_n11;

        let (assign55580_e90977, assign55580_e90977_d_n3, assign55580_e90977_d_n4, assign55580_e90977_d_n5, assign55580_e90977_d_n6, assign55580_e90977_d_n7, assign55580_e90977_d_n8, assign55580_e90977_d_n9, assign55580_e90977_d_n10, assign55580_e90977_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55580_e90966: f64 = (locals.var_vgfbcv - locals.var_psip);
        let assign55580_e90969: f64 = (2.0 * locals.var_qs_1);
        let assign55580_e90972: f64 = (locals.var_nq - 1.0);
        let assign55580_e90973: f64 = (assign55580_e90969 * assign55580_e90972);
        let assign55580_e90974: f64 = (assign55580_e90966 - assign55580_e90973);
        let assign55580_e90975: f64 = (locals.var_vt * assign55580_e90974);
        (assign55580_e90975, (locals.var_vt * ((locals.var_vgfbcv_dn3 - locals.var_psip_dn3) - (((2.0 * locals.var_qs_1_dn3) * assign55580_e90972) + (assign55580_e90969 * locals.var_nq_dn3)))), ((locals.var_vt_dn4 * assign55580_e90974) + (locals.var_vt * ((locals.var_vgfbcv_dn4 - locals.var_psip_dn4) - (((2.0 * locals.var_qs_1_dn4) * assign55580_e90972) + (assign55580_e90969 * locals.var_nq_dn4))))), ((locals.var_vt_dn5 * assign55580_e90974) + (locals.var_vt * ((locals.var_vgfbcv_dn5 - locals.var_psip_dn5) - (((2.0 * locals.var_qs_1_dn5) * assign55580_e90972) + (assign55580_e90969 * locals.var_nq_dn5))))), (locals.var_vt * ((locals.var_vgfbcv_dn6 - locals.var_psip_dn6) - (((2.0 * locals.var_qs_1_dn6) * assign55580_e90972) + (assign55580_e90969 * locals.var_nq_dn6)))), (locals.var_vt * ((locals.var_vgfbcv_dn7 - locals.var_psip_dn7) - (((2.0 * locals.var_qs_1_dn7) * assign55580_e90972) + (assign55580_e90969 * locals.var_nq_dn7)))), (locals.var_vt * ((locals.var_vgfbcv_dn8 - locals.var_psip_dn8) - (((2.0 * locals.var_qs_1_dn8) * assign55580_e90972) + (assign55580_e90969 * locals.var_nq_dn8)))), (locals.var_vt * ((locals.var_vgfbcv_dn9 - locals.var_psip_dn9) - (((2.0 * locals.var_qs_1_dn9) * assign55580_e90972) + (assign55580_e90969 * locals.var_nq_dn9)))), (locals.var_vt * ((locals.var_vgfbcv_dn10 - locals.var_psip_dn10) - (((2.0 * locals.var_qs_1_dn10) * assign55580_e90972) + (assign55580_e90969 * locals.var_nq_dn10)))), (locals.var_vt * ((locals.var_vgfbcv_dn11 - locals.var_psip_dn11) - (((2.0 * locals.var_qs_1_dn11) * assign55580_e90972) + (assign55580_e90969 * locals.var_nq_dn11)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign55580_e90977;
        locals.var_t0_dn3 = assign55580_e90977_d_n3;
        locals.var_t0_dn4 = assign55580_e90977_d_n4;
        locals.var_t0_dn5 = assign55580_e90977_d_n5;
        locals.var_t0_dn6 = assign55580_e90977_d_n6;
        locals.var_t0_dn7 = assign55580_e90977_d_n7;
        locals.var_t0_dn8 = assign55580_e90977_d_n8;
        locals.var_t0_dn9 = assign55580_e90977_d_n9;
        locals.var_t0_dn10 = assign55580_e90977_d_n10;
        locals.var_t0_dn11 = assign55580_e90977_d_n11;

        let (assign55590_e91003, assign55590_e91003_d_n3, assign55590_e91003_d_n4, assign55590_e91003_d_n5, assign55590_e91003_d_n6, assign55590_e91003_d_n7, assign55590_e91003_d_n8, assign55590_e91003_d_n9, assign55590_e91003_d_n10, assign55590_e91003_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55590_e90985: f64 = locals.var_t0;
        let assign55590_e90988: f64 = locals.var_t0;
        let assign55590_e90991: f64 = locals.var_t0;
        let assign55590_e90992: f64 = (assign55590_e90988 * assign55590_e90991);
        let assign55590_e90995: f64 = (0.25 * 0.1);
        let assign55590_e90997: f64 = (assign55590_e90995 * 0.1);
        let assign55590_e90998: f64 = (assign55590_e90992 + assign55590_e90997);
        let assign55590_e90999: f64 = (assign55590_e90998).sqrt();
        let assign55590_e91000: f64 = (assign55590_e90985 + assign55590_e90999);
        let assign55590_e91001: f64 = (0.5 * assign55590_e91000);
        (assign55590_e91001, (0.5 * (locals.var_t0_dn3 + (((locals.var_t0_dn3 * assign55590_e90991) + (assign55590_e90988 * locals.var_t0_dn3)) / (2.0 * assign55590_e90999)))), (0.5 * (locals.var_t0_dn4 + (((locals.var_t0_dn4 * assign55590_e90991) + (assign55590_e90988 * locals.var_t0_dn4)) / (2.0 * assign55590_e90999)))), (0.5 * (locals.var_t0_dn5 + (((locals.var_t0_dn5 * assign55590_e90991) + (assign55590_e90988 * locals.var_t0_dn5)) / (2.0 * assign55590_e90999)))), (0.5 * (locals.var_t0_dn6 + (((locals.var_t0_dn6 * assign55590_e90991) + (assign55590_e90988 * locals.var_t0_dn6)) / (2.0 * assign55590_e90999)))), (0.5 * (locals.var_t0_dn7 + (((locals.var_t0_dn7 * assign55590_e90991) + (assign55590_e90988 * locals.var_t0_dn7)) / (2.0 * assign55590_e90999)))), (0.5 * (locals.var_t0_dn8 + (((locals.var_t0_dn8 * assign55590_e90991) + (assign55590_e90988 * locals.var_t0_dn8)) / (2.0 * assign55590_e90999)))), (0.5 * (locals.var_t0_dn9 + (((locals.var_t0_dn9 * assign55590_e90991) + (assign55590_e90988 * locals.var_t0_dn9)) / (2.0 * assign55590_e90999)))), (0.5 * (locals.var_t0_dn10 + (((locals.var_t0_dn10 * assign55590_e90991) + (assign55590_e90988 * locals.var_t0_dn10)) / (2.0 * assign55590_e90999)))), (0.5 * (locals.var_t0_dn11 + (((locals.var_t0_dn11 * assign55590_e90991) + (assign55590_e90988 * locals.var_t0_dn11)) / (2.0 * assign55590_e90999)))),)
    } else {
        (locals.var_qbs, locals.var_qbs_dn3, locals.var_qbs_dn4, locals.var_qbs_dn5, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn8, locals.var_qbs_dn9, locals.var_qbs_dn10, locals.var_qbs_dn11,)
    }
};
        locals.var_qbs = assign55590_e91003;
        locals.var_qbs_dn3 = assign55590_e91003_d_n3;
        locals.var_qbs_dn4 = assign55590_e91003_d_n4;
        locals.var_qbs_dn5 = assign55590_e91003_d_n5;
        locals.var_qbs_dn6 = assign55590_e91003_d_n6;
        locals.var_qbs_dn7 = assign55590_e91003_d_n7;
        locals.var_qbs_dn8 = assign55590_e91003_d_n8;
        locals.var_qbs_dn9 = assign55590_e91003_d_n9;
        locals.var_qbs_dn10 = assign55590_e91003_d_n10;
        locals.var_qbs_dn11 = assign55590_e91003_d_n11;

        let (assign55600_e91016, assign55600_e91016_d_n3, assign55600_e91016_d_n4, assign55600_e91016_d_n5, assign55600_e91016_d_n6, assign55600_e91016_d_n7, assign55600_e91016_d_n8, assign55600_e91016_d_n9, assign55600_e91016_d_n10, assign55600_e91016_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55600_e91010: f64 = (2.0 * locals.var_nq);
        let assign55600_e91012: f64 = (assign55600_e91010 * locals.var_vt);
        let assign55600_e91014: f64 = (assign55600_e91012 * locals.var_qs_1);
        (assign55600_e91014, ((((2.0 * locals.var_nq_dn3) * locals.var_vt) * locals.var_qs_1) + (assign55600_e91012 * locals.var_qs_1_dn3)), (((((2.0 * locals.var_nq_dn4) * locals.var_vt) + (assign55600_e91010 * locals.var_vt_dn4)) * locals.var_qs_1) + (assign55600_e91012 * locals.var_qs_1_dn4)), (((((2.0 * locals.var_nq_dn5) * locals.var_vt) + (assign55600_e91010 * locals.var_vt_dn5)) * locals.var_qs_1) + (assign55600_e91012 * locals.var_qs_1_dn5)), ((((2.0 * locals.var_nq_dn6) * locals.var_vt) * locals.var_qs_1) + (assign55600_e91012 * locals.var_qs_1_dn6)), ((((2.0 * locals.var_nq_dn7) * locals.var_vt) * locals.var_qs_1) + (assign55600_e91012 * locals.var_qs_1_dn7)), ((((2.0 * locals.var_nq_dn8) * locals.var_vt) * locals.var_qs_1) + (assign55600_e91012 * locals.var_qs_1_dn8)), ((((2.0 * locals.var_nq_dn9) * locals.var_vt) * locals.var_qs_1) + (assign55600_e91012 * locals.var_qs_1_dn9)), ((((2.0 * locals.var_nq_dn10) * locals.var_vt) * locals.var_qs_1) + (assign55600_e91012 * locals.var_qs_1_dn10)), ((((2.0 * locals.var_nq_dn11) * locals.var_vt) * locals.var_qs_1) + (assign55600_e91012 * locals.var_qs_1_dn11)),)
    } else {
        (locals.var_qis, locals.var_qis_dn3, locals.var_qis_dn4, locals.var_qis_dn5, locals.var_qis_dn6, locals.var_qis_dn7, locals.var_qis_dn8, locals.var_qis_dn9, locals.var_qis_dn10, locals.var_qis_dn11,)
    }
};
        locals.var_qis = assign55600_e91016;
        locals.var_qis_dn3 = assign55600_e91016_d_n3;
        locals.var_qis_dn4 = assign55600_e91016_d_n4;
        locals.var_qis_dn5 = assign55600_e91016_d_n5;
        locals.var_qis_dn6 = assign55600_e91016_d_n6;
        locals.var_qis_dn7 = assign55600_e91016_d_n7;
        locals.var_qis_dn8 = assign55600_e91016_d_n8;
        locals.var_qis_dn9 = assign55600_e91016_d_n9;
        locals.var_qis_dn10 = assign55600_e91016_d_n10;
        locals.var_qis_dn11 = assign55600_e91016_d_n11;

        let (assign55610_e91029, assign55610_e91029_d_n3, assign55610_e91029_d_n4, assign55610_e91029_d_n5, assign55610_e91029_d_n6, assign55610_e91029_d_n7, assign55610_e91029_d_n8, assign55610_e91029_d_n9, assign55610_e91029_d_n10, assign55610_e91029_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55610_e91025: f64 = (locals.var_eta_mu * locals.var_qis);
        let assign55610_e91026: f64 = (locals.var_qbs + assign55610_e91025);
        let assign55610_e91027: f64 = (locals.var_eefffactor * assign55610_e91026);
        (assign55610_e91027, (locals.var_eefffactor * (locals.var_qbs_dn3 + (locals.var_eta_mu * locals.var_qis_dn3))), (locals.var_eefffactor * (locals.var_qbs_dn4 + (locals.var_eta_mu * locals.var_qis_dn4))), (locals.var_eefffactor * (locals.var_qbs_dn5 + (locals.var_eta_mu * locals.var_qis_dn5))), (locals.var_eefffactor * (locals.var_qbs_dn6 + (locals.var_eta_mu * locals.var_qis_dn6))), (locals.var_eefffactor * (locals.var_qbs_dn7 + (locals.var_eta_mu * locals.var_qis_dn7))), (locals.var_eefffactor * (locals.var_qbs_dn8 + (locals.var_eta_mu * locals.var_qis_dn8))), (locals.var_eefffactor * (locals.var_qbs_dn9 + (locals.var_eta_mu * locals.var_qis_dn9))), (locals.var_eefffactor * (locals.var_qbs_dn10 + (locals.var_eta_mu * locals.var_qis_dn10))), (locals.var_eefffactor * (locals.var_qbs_dn11 + (locals.var_eta_mu * locals.var_qis_dn11))),)
    } else {
        (locals.var_eeffs, locals.var_eeffs_dn3, locals.var_eeffs_dn4, locals.var_eeffs_dn5, locals.var_eeffs_dn6, locals.var_eeffs_dn7, locals.var_eeffs_dn8, locals.var_eeffs_dn9, locals.var_eeffs_dn10, locals.var_eeffs_dn11,)
    }
};
        locals.var_eeffs = assign55610_e91029;
        locals.var_eeffs_dn3 = assign55610_e91029_d_n3;
        locals.var_eeffs_dn4 = assign55610_e91029_d_n4;
        locals.var_eeffs_dn5 = assign55610_e91029_d_n5;
        locals.var_eeffs_dn6 = assign55610_e91029_d_n6;
        locals.var_eeffs_dn7 = assign55610_e91029_d_n7;
        locals.var_eeffs_dn8 = assign55610_e91029_d_n8;
        locals.var_eeffs_dn9 = assign55610_e91029_d_n9;
        locals.var_eeffs_dn10 = assign55610_e91029_d_n10;
        locals.var_eeffs_dn11 = assign55610_e91029_d_n11;

        let (assign55620_e91044, assign55620_e91044_d_n3, assign55620_e91044_d_n4, assign55620_e91044_d_n5, assign55620_e91044_d_n6, assign55620_e91044_d_n7, assign55620_e91044_d_n8, assign55620_e91044_d_n9, assign55620_e91044_d_n10, assign55620_e91044_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55620_e91037: f64 = (locals.var_uc_a * locals.var_vbsx);
        let assign55620_e91038: f64 = (locals.var_ua_a + assign55620_e91037);
        let assign55620_e91041: f64 = (locals.var_eeffs).powf(locals.var_eu_t);
        let assign55620_e91042: f64 = (assign55620_e91038 * assign55620_e91041);
        (assign55620_e91042, (((locals.var_ua_a_dn3 + ((locals.var_uc_a_dn3 * locals.var_vbsx) + (locals.var_uc_a * locals.var_vbsx_dn3))) * assign55620_e91041) + (assign55620_e91038 * if locals.var_eu_t_dn3 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffs).powf(locals.var_eu_t - 1.0) * locals.var_eeffs_dn3)) } } else { (assign55620_e91041 * ((locals.var_eu_t_dn3 * (locals.var_eeffs).ln()) + (locals.var_eu_t * (locals.var_eeffs_dn3 / locals.var_eeffs)))) })), (((locals.var_ua_a_dn4 + ((locals.var_uc_a_dn4 * locals.var_vbsx) + (locals.var_uc_a * locals.var_vbsx_dn4))) * assign55620_e91041) + (assign55620_e91038 * if locals.var_eu_t_dn4 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffs).powf(locals.var_eu_t - 1.0) * locals.var_eeffs_dn4)) } } else { (assign55620_e91041 * ((locals.var_eu_t_dn4 * (locals.var_eeffs).ln()) + (locals.var_eu_t * (locals.var_eeffs_dn4 / locals.var_eeffs)))) })), (((locals.var_ua_a_dn5 + ((locals.var_uc_a_dn5 * locals.var_vbsx) + (locals.var_uc_a * locals.var_vbsx_dn5))) * assign55620_e91041) + (assign55620_e91038 * if locals.var_eu_t_dn5 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffs).powf(locals.var_eu_t - 1.0) * locals.var_eeffs_dn5)) } } else { (assign55620_e91041 * ((locals.var_eu_t_dn5 * (locals.var_eeffs).ln()) + (locals.var_eu_t * (locals.var_eeffs_dn5 / locals.var_eeffs)))) })), (((locals.var_ua_a_dn6 + ((locals.var_uc_a_dn6 * locals.var_vbsx) + (locals.var_uc_a * locals.var_vbsx_dn6))) * assign55620_e91041) + (assign55620_e91038 * if locals.var_eu_t_dn6 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffs).powf(locals.var_eu_t - 1.0) * locals.var_eeffs_dn6)) } } else { (assign55620_e91041 * ((locals.var_eu_t_dn6 * (locals.var_eeffs).ln()) + (locals.var_eu_t * (locals.var_eeffs_dn6 / locals.var_eeffs)))) })), (((locals.var_ua_a_dn7 + ((locals.var_uc_a_dn7 * locals.var_vbsx) + (locals.var_uc_a * locals.var_vbsx_dn7))) * assign55620_e91041) + (assign55620_e91038 * if locals.var_eu_t_dn7 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffs).powf(locals.var_eu_t - 1.0) * locals.var_eeffs_dn7)) } } else { (assign55620_e91041 * ((locals.var_eu_t_dn7 * (locals.var_eeffs).ln()) + (locals.var_eu_t * (locals.var_eeffs_dn7 / locals.var_eeffs)))) })), (((locals.var_ua_a_dn8 + ((locals.var_uc_a_dn8 * locals.var_vbsx) + (locals.var_uc_a * locals.var_vbsx_dn8))) * assign55620_e91041) + (assign55620_e91038 * if locals.var_eu_t_dn8 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffs).powf(locals.var_eu_t - 1.0) * locals.var_eeffs_dn8)) } } else { (assign55620_e91041 * ((locals.var_eu_t_dn8 * (locals.var_eeffs).ln()) + (locals.var_eu_t * (locals.var_eeffs_dn8 / locals.var_eeffs)))) })), (((locals.var_ua_a_dn9 + ((locals.var_uc_a_dn9 * locals.var_vbsx) + (locals.var_uc_a * locals.var_vbsx_dn9))) * assign55620_e91041) + (assign55620_e91038 * if locals.var_eu_t_dn9 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffs).powf(locals.var_eu_t - 1.0) * locals.var_eeffs_dn9)) } } else { (assign55620_e91041 * ((locals.var_eu_t_dn9 * (locals.var_eeffs).ln()) + (locals.var_eu_t * (locals.var_eeffs_dn9 / locals.var_eeffs)))) })), (((locals.var_ua_a_dn10 + ((locals.var_uc_a_dn10 * locals.var_vbsx) + (locals.var_uc_a * locals.var_vbsx_dn10))) * assign55620_e91041) + (assign55620_e91038 * if locals.var_eu_t_dn10 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffs).powf(locals.var_eu_t - 1.0) * locals.var_eeffs_dn10)) } } else { (assign55620_e91041 * ((locals.var_eu_t_dn10 * (locals.var_eeffs).ln()) + (locals.var_eu_t * (locals.var_eeffs_dn10 / locals.var_eeffs)))) })), (((locals.var_ua_a_dn11 + ((locals.var_uc_a_dn11 * locals.var_vbsx) + (locals.var_uc_a * locals.var_vbsx_dn11))) * assign55620_e91041) + (assign55620_e91038 * if locals.var_eu_t_dn11 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffs).powf(locals.var_eu_t - 1.0) * locals.var_eeffs_dn11)) } } else { (assign55620_e91041 * ((locals.var_eu_t_dn11 * (locals.var_eeffs).ln()) + (locals.var_eu_t * (locals.var_eeffs_dn11 / locals.var_eeffs)))) })),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign55620_e91044;
        locals.var_t3_dn3 = assign55620_e91044_d_n3;
        locals.var_t3_dn4 = assign55620_e91044_d_n4;
        locals.var_t3_dn5 = assign55620_e91044_d_n5;
        locals.var_t3_dn6 = assign55620_e91044_d_n6;
        locals.var_t3_dn7 = assign55620_e91044_d_n7;
        locals.var_t3_dn8 = assign55620_e91044_d_n8;
        locals.var_t3_dn9 = assign55620_e91044_d_n9;
        locals.var_t3_dn10 = assign55620_e91044_d_n10;
        locals.var_t3_dn11 = assign55620_e91044_d_n11;

        let (assign55630_e91053, assign55630_e91053_d_n3, assign55630_e91053_d_n4, assign55630_e91053_d_n5, assign55630_e91053_d_n6, assign55630_e91053_d_n7, assign55630_e91053_d_n8, assign55630_e91053_d_n9, assign55630_e91053_d_n10, assign55630_e91053_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55630_e91051: f64 = (1.0 + locals.var_t3);
        (assign55630_e91051, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign55630_e91053;
        locals.var_t4_dn3 = assign55630_e91053_d_n3;
        locals.var_t4_dn4 = assign55630_e91053_d_n4;
        locals.var_t4_dn5 = assign55630_e91053_d_n5;
        locals.var_t4_dn6 = assign55630_e91053_d_n6;
        locals.var_t4_dn7 = assign55630_e91053_d_n7;
        locals.var_t4_dn8 = assign55630_e91053_d_n8;
        locals.var_t4_dn9 = assign55630_e91053_d_n9;
        locals.var_t4_dn10 = assign55630_e91053_d_n10;
        locals.var_t4_dn11 = assign55630_e91053_d_n11;

        let (assign55640_e91079, assign55640_e91079_d_n3, assign55640_e91079_d_n4, assign55640_e91079_d_n5, assign55640_e91079_d_n6, assign55640_e91079_d_n7, assign55640_e91079_d_n8, assign55640_e91079_d_n9, assign55640_e91079_d_n10, assign55640_e91079_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55640_e91061: f64 = (locals.var_t4 + 1.0);
        let assign55640_e91064: f64 = (locals.var_t4 - 1.0);
        let assign55640_e91067: f64 = (locals.var_t4 - 1.0);
        let assign55640_e91068: f64 = (assign55640_e91064 * assign55640_e91067);
        let assign55640_e91071: f64 = (0.25 * 0.0015);
        let assign55640_e91073: f64 = (assign55640_e91071 * 0.0015);
        let assign55640_e91074: f64 = (assign55640_e91068 + assign55640_e91073);
        let assign55640_e91075: f64 = (assign55640_e91074).sqrt();
        let assign55640_e91076: f64 = (assign55640_e91061 + assign55640_e91075);
        let assign55640_e91077: f64 = (0.5 * assign55640_e91076);
        (assign55640_e91077, (0.5 * (locals.var_t4_dn3 + (((locals.var_t4_dn3 * assign55640_e91067) + (assign55640_e91064 * locals.var_t4_dn3)) / (2.0 * assign55640_e91075)))), (0.5 * (locals.var_t4_dn4 + (((locals.var_t4_dn4 * assign55640_e91067) + (assign55640_e91064 * locals.var_t4_dn4)) / (2.0 * assign55640_e91075)))), (0.5 * (locals.var_t4_dn5 + (((locals.var_t4_dn5 * assign55640_e91067) + (assign55640_e91064 * locals.var_t4_dn5)) / (2.0 * assign55640_e91075)))), (0.5 * (locals.var_t4_dn6 + (((locals.var_t4_dn6 * assign55640_e91067) + (assign55640_e91064 * locals.var_t4_dn6)) / (2.0 * assign55640_e91075)))), (0.5 * (locals.var_t4_dn7 + (((locals.var_t4_dn7 * assign55640_e91067) + (assign55640_e91064 * locals.var_t4_dn7)) / (2.0 * assign55640_e91075)))), (0.5 * (locals.var_t4_dn8 + (((locals.var_t4_dn8 * assign55640_e91067) + (assign55640_e91064 * locals.var_t4_dn8)) / (2.0 * assign55640_e91075)))), (0.5 * (locals.var_t4_dn9 + (((locals.var_t4_dn9 * assign55640_e91067) + (assign55640_e91064 * locals.var_t4_dn9)) / (2.0 * assign55640_e91075)))), (0.5 * (locals.var_t4_dn10 + (((locals.var_t4_dn10 * assign55640_e91067) + (assign55640_e91064 * locals.var_t4_dn10)) / (2.0 * assign55640_e91075)))), (0.5 * (locals.var_t4_dn11 + (((locals.var_t4_dn11 * assign55640_e91067) + (assign55640_e91064 * locals.var_t4_dn11)) / (2.0 * assign55640_e91075)))),)
    } else {
        (locals.var_dmobs, locals.var_dmobs_dn3, locals.var_dmobs_dn4, locals.var_dmobs_dn5, locals.var_dmobs_dn6, locals.var_dmobs_dn7, locals.var_dmobs_dn8, locals.var_dmobs_dn9, locals.var_dmobs_dn10, locals.var_dmobs_dn11,)
    }
};
        locals.var_dmobs = assign55640_e91079;
        locals.var_dmobs_dn3 = assign55640_e91079_d_n3;
        locals.var_dmobs_dn4 = assign55640_e91079_d_n4;
        locals.var_dmobs_dn5 = assign55640_e91079_d_n5;
        locals.var_dmobs_dn6 = assign55640_e91079_d_n6;
        locals.var_dmobs_dn7 = assign55640_e91079_d_n7;
        locals.var_dmobs_dn8 = assign55640_e91079_d_n8;
        locals.var_dmobs_dn9 = assign55640_e91079_d_n9;
        locals.var_dmobs_dn10 = assign55640_e91079_d_n10;
        locals.var_dmobs_dn11 = assign55640_e91079_d_n11;

        let (assign55650_e91094, assign55650_e91094_d_n3, assign55650_e91094_d_n4, assign55650_e91094_d_n5, assign55650_e91094_d_n6, assign55650_e91094_d_n7, assign55650_e91094_d_n8, assign55650_e91094_d_n9, assign55650_e91094_d_n10, assign55650_e91094_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55650_e91086: f64 = (locals.var_u0_a / locals.var_dmobs);
        let assign55650_e91088: f64 = (assign55650_e91086 * locals.var_vt);
        let assign55650_e91091: f64 = (locals.var_vsatcv_t * locals.var_lact);
        let assign55650_e91092: f64 = (assign55650_e91088 / assign55650_e91091);
        (assign55650_e91092, (((((((locals.var_u0_a_dn3 * locals.var_dmobs) - (locals.var_u0_a * locals.var_dmobs_dn3)) / (locals.var_dmobs * locals.var_dmobs)) * locals.var_vt) * assign55650_e91091) - (assign55650_e91088 * (locals.var_vsatcv_t_dn3 * locals.var_lact))) / (assign55650_e91091 * assign55650_e91091)), ((((((((locals.var_u0_a_dn4 * locals.var_dmobs) - (locals.var_u0_a * locals.var_dmobs_dn4)) / (locals.var_dmobs * locals.var_dmobs)) * locals.var_vt) + (assign55650_e91086 * locals.var_vt_dn4)) * assign55650_e91091) - (assign55650_e91088 * (locals.var_vsatcv_t_dn4 * locals.var_lact))) / (assign55650_e91091 * assign55650_e91091)), ((((((((locals.var_u0_a_dn5 * locals.var_dmobs) - (locals.var_u0_a * locals.var_dmobs_dn5)) / (locals.var_dmobs * locals.var_dmobs)) * locals.var_vt) + (assign55650_e91086 * locals.var_vt_dn5)) * assign55650_e91091) - (assign55650_e91088 * (locals.var_vsatcv_t_dn5 * locals.var_lact))) / (assign55650_e91091 * assign55650_e91091)), (((((((locals.var_u0_a_dn6 * locals.var_dmobs) - (locals.var_u0_a * locals.var_dmobs_dn6)) / (locals.var_dmobs * locals.var_dmobs)) * locals.var_vt) * assign55650_e91091) - (assign55650_e91088 * (locals.var_vsatcv_t_dn6 * locals.var_lact))) / (assign55650_e91091 * assign55650_e91091)), (((((((locals.var_u0_a_dn7 * locals.var_dmobs) - (locals.var_u0_a * locals.var_dmobs_dn7)) / (locals.var_dmobs * locals.var_dmobs)) * locals.var_vt) * assign55650_e91091) - (assign55650_e91088 * (locals.var_vsatcv_t_dn7 * locals.var_lact))) / (assign55650_e91091 * assign55650_e91091)), (((((((locals.var_u0_a_dn8 * locals.var_dmobs) - (locals.var_u0_a * locals.var_dmobs_dn8)) / (locals.var_dmobs * locals.var_dmobs)) * locals.var_vt) * assign55650_e91091) - (assign55650_e91088 * (locals.var_vsatcv_t_dn8 * locals.var_lact))) / (assign55650_e91091 * assign55650_e91091)), (((((((locals.var_u0_a_dn9 * locals.var_dmobs) - (locals.var_u0_a * locals.var_dmobs_dn9)) / (locals.var_dmobs * locals.var_dmobs)) * locals.var_vt) * assign55650_e91091) - (assign55650_e91088 * (locals.var_vsatcv_t_dn9 * locals.var_lact))) / (assign55650_e91091 * assign55650_e91091)), (((((((locals.var_u0_a_dn10 * locals.var_dmobs) - (locals.var_u0_a * locals.var_dmobs_dn10)) / (locals.var_dmobs * locals.var_dmobs)) * locals.var_vt) * assign55650_e91091) - (assign55650_e91088 * (locals.var_vsatcv_t_dn10 * locals.var_lact))) / (assign55650_e91091 * assign55650_e91091)), (((((((locals.var_u0_a_dn11 * locals.var_dmobs) - (locals.var_u0_a * locals.var_dmobs_dn11)) / (locals.var_dmobs * locals.var_dmobs)) * locals.var_vt) * assign55650_e91091) - (assign55650_e91088 * (locals.var_vsatcv_t_dn11 * locals.var_lact))) / (assign55650_e91091 * assign55650_e91091)),)
    } else {
        (locals.var_lambdac_by2, locals.var_lambdac_by2_dn3, locals.var_lambdac_by2_dn4, locals.var_lambdac_by2_dn5, locals.var_lambdac_by2_dn6, locals.var_lambdac_by2_dn7, locals.var_lambdac_by2_dn8, locals.var_lambdac_by2_dn9, locals.var_lambdac_by2_dn10, locals.var_lambdac_by2_dn11,)
    }
};
        locals.var_lambdac_by2 = assign55650_e91094;
        locals.var_lambdac_by2_dn3 = assign55650_e91094_d_n3;
        locals.var_lambdac_by2_dn4 = assign55650_e91094_d_n4;
        locals.var_lambdac_by2_dn5 = assign55650_e91094_d_n5;
        locals.var_lambdac_by2_dn6 = assign55650_e91094_d_n6;
        locals.var_lambdac_by2_dn7 = assign55650_e91094_d_n7;
        locals.var_lambdac_by2_dn8 = assign55650_e91094_d_n8;
        locals.var_lambdac_by2_dn9 = assign55650_e91094_d_n9;
        locals.var_lambdac_by2_dn10 = assign55650_e91094_d_n10;
        locals.var_lambdac_by2_dn11 = assign55650_e91094_d_n11;

        let (assign55660_e91115, assign55660_e91115_d_n3, assign55660_e91115_d_n4, assign55660_e91115_d_n5, assign55660_e91115_d_n6, assign55660_e91115_d_n7, assign55660_e91115_d_n8, assign55660_e91115_d_n9, assign55660_e91115_d_n10, assign55660_e91115_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55660_e91102: f64 = (locals.var_qs_1 * locals.var_qs_1);
        let assign55660_e91104: f64 = (assign55660_e91102 + locals.var_qs_1);
        let assign55660_e91105: f64 = (locals.var_lambdac_by2 * assign55660_e91104);
        let assign55660_e91110: f64 = (1.0 + locals.var_qs_1);
        let assign55660_e91111: f64 = (locals.var_lambdac_by2 * assign55660_e91110);
        let assign55660_e91112: f64 = (1.0 + assign55660_e91111);
        let assign55660_e91113: f64 = (assign55660_e91105 / assign55660_e91112);
        (assign55660_e91113, (((((locals.var_lambdac_by2_dn3 * assign55660_e91104) + (locals.var_lambdac_by2 * (((locals.var_qs_1_dn3 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn3)) + locals.var_qs_1_dn3))) * assign55660_e91112) - (assign55660_e91105 * ((locals.var_lambdac_by2_dn3 * assign55660_e91110) + (locals.var_lambdac_by2 * locals.var_qs_1_dn3)))) / (assign55660_e91112 * assign55660_e91112)), (((((locals.var_lambdac_by2_dn4 * assign55660_e91104) + (locals.var_lambdac_by2 * (((locals.var_qs_1_dn4 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn4)) + locals.var_qs_1_dn4))) * assign55660_e91112) - (assign55660_e91105 * ((locals.var_lambdac_by2_dn4 * assign55660_e91110) + (locals.var_lambdac_by2 * locals.var_qs_1_dn4)))) / (assign55660_e91112 * assign55660_e91112)), (((((locals.var_lambdac_by2_dn5 * assign55660_e91104) + (locals.var_lambdac_by2 * (((locals.var_qs_1_dn5 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn5)) + locals.var_qs_1_dn5))) * assign55660_e91112) - (assign55660_e91105 * ((locals.var_lambdac_by2_dn5 * assign55660_e91110) + (locals.var_lambdac_by2 * locals.var_qs_1_dn5)))) / (assign55660_e91112 * assign55660_e91112)), (((((locals.var_lambdac_by2_dn6 * assign55660_e91104) + (locals.var_lambdac_by2 * (((locals.var_qs_1_dn6 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn6)) + locals.var_qs_1_dn6))) * assign55660_e91112) - (assign55660_e91105 * ((locals.var_lambdac_by2_dn6 * assign55660_e91110) + (locals.var_lambdac_by2 * locals.var_qs_1_dn6)))) / (assign55660_e91112 * assign55660_e91112)), (((((locals.var_lambdac_by2_dn7 * assign55660_e91104) + (locals.var_lambdac_by2 * (((locals.var_qs_1_dn7 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn7)) + locals.var_qs_1_dn7))) * assign55660_e91112) - (assign55660_e91105 * ((locals.var_lambdac_by2_dn7 * assign55660_e91110) + (locals.var_lambdac_by2 * locals.var_qs_1_dn7)))) / (assign55660_e91112 * assign55660_e91112)), (((((locals.var_lambdac_by2_dn8 * assign55660_e91104) + (locals.var_lambdac_by2 * (((locals.var_qs_1_dn8 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn8)) + locals.var_qs_1_dn8))) * assign55660_e91112) - (assign55660_e91105 * ((locals.var_lambdac_by2_dn8 * assign55660_e91110) + (locals.var_lambdac_by2 * locals.var_qs_1_dn8)))) / (assign55660_e91112 * assign55660_e91112)), (((((locals.var_lambdac_by2_dn9 * assign55660_e91104) + (locals.var_lambdac_by2 * (((locals.var_qs_1_dn9 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn9)) + locals.var_qs_1_dn9))) * assign55660_e91112) - (assign55660_e91105 * ((locals.var_lambdac_by2_dn9 * assign55660_e91110) + (locals.var_lambdac_by2 * locals.var_qs_1_dn9)))) / (assign55660_e91112 * assign55660_e91112)), (((((locals.var_lambdac_by2_dn10 * assign55660_e91104) + (locals.var_lambdac_by2 * (((locals.var_qs_1_dn10 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn10)) + locals.var_qs_1_dn10))) * assign55660_e91112) - (assign55660_e91105 * ((locals.var_lambdac_by2_dn10 * assign55660_e91110) + (locals.var_lambdac_by2 * locals.var_qs_1_dn10)))) / (assign55660_e91112 * assign55660_e91112)), (((((locals.var_lambdac_by2_dn11 * assign55660_e91104) + (locals.var_lambdac_by2 * (((locals.var_qs_1_dn11 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn11)) + locals.var_qs_1_dn11))) * assign55660_e91112) - (assign55660_e91105 * ((locals.var_lambdac_by2_dn11 * assign55660_e91110) + (locals.var_lambdac_by2 * locals.var_qs_1_dn11)))) / (assign55660_e91112 * assign55660_e91112)),)
    } else {
        (locals.var_qdsat, locals.var_qdsat_dn3, locals.var_qdsat_dn4, locals.var_qdsat_dn5, locals.var_qdsat_dn6, locals.var_qdsat_dn7, locals.var_qdsat_dn8, locals.var_qdsat_dn9, locals.var_qdsat_dn10, locals.var_qdsat_dn11,)
    }
};
        locals.var_qdsat = assign55660_e91115;
        locals.var_qdsat_dn3 = assign55660_e91115_d_n3;
        locals.var_qdsat_dn4 = assign55660_e91115_d_n4;
        locals.var_qdsat_dn5 = assign55660_e91115_d_n5;
        locals.var_qdsat_dn6 = assign55660_e91115_d_n6;
        locals.var_qdsat_dn7 = assign55660_e91115_d_n7;
        locals.var_qdsat_dn8 = assign55660_e91115_d_n8;
        locals.var_qdsat_dn9 = assign55660_e91115_d_n9;
        locals.var_qdsat_dn10 = assign55660_e91115_d_n10;
        locals.var_qdsat_dn11 = assign55660_e91115_d_n11;

        let (assign55670_e91155, assign55670_e91155_d_n3, assign55670_e91155_d_n4, assign55670_e91155_d_n5, assign55670_e91155_d_n6, assign55670_e91155_d_n7, assign55670_e91155_d_n8, assign55670_e91155_d_n9, assign55670_e91155_d_n10, assign55670_e91155_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55670_e91123: f64 = (2.0 * locals.var_phibcv);
        let assign55670_e91124: f64 = (locals.var_psip - assign55670_e91123);
        let assign55670_e91127: f64 = (2.0 * locals.var_qdsat);
        let assign55670_e91130: f64 = (locals.var_qdsat * 2.0);
        let assign55670_e91132: f64 = (assign55670_e91130 * locals.var_nq);
        let assign55670_e91134: f64 = (assign55670_e91132 * locals.var_inv_gam);
        let assign55670_e91137: f64 = (locals.var_qdsat * 2.0);
        let assign55670_e91139: f64 = (assign55670_e91137 * locals.var_nq);
        let assign55670_e91141: f64 = (assign55670_e91139 * locals.var_inv_gam);
        let assign55670_e91145: f64 = (locals.var_nq - 1.0);
        let assign55670_e91146: f64 = (locals.var_gamcv / assign55670_e91145);
        let assign55670_e91147: f64 = (assign55670_e91141 + assign55670_e91146);
        let assign55670_e91148: f64 = (assign55670_e91134 * assign55670_e91147);
        let assign55670_e91150: f64 = (assign55670_e91148).max(1e-38);
        let assign55670_e91151: f64 = (assign55670_e91150).ln();
        let assign55670_e91152: f64 = (assign55670_e91127 + assign55670_e91151);
        let assign55670_e91153: f64 = (assign55670_e91124 - assign55670_e91152);
        (assign55670_e91153, ((locals.var_psip_dn3 - (2.0 * locals.var_phibcv_dn3)) - ((2.0 * locals.var_qdsat_dn3) + (if assign55670_e91148 >= 1e-38 { (((((((locals.var_qdsat_dn3 * 2.0) * locals.var_nq) + (assign55670_e91130 * locals.var_nq_dn3)) * locals.var_inv_gam) + (assign55670_e91132 * locals.var_inv_gam_dn3)) * assign55670_e91147) + (assign55670_e91134 * ((((((locals.var_qdsat_dn3 * 2.0) * locals.var_nq) + (assign55670_e91137 * locals.var_nq_dn3)) * locals.var_inv_gam) + (assign55670_e91139 * locals.var_inv_gam_dn3)) + (((locals.var_gamcv_dn3 * assign55670_e91145) - (locals.var_gamcv * locals.var_nq_dn3)) / (assign55670_e91145 * assign55670_e91145))))) } else { 0.0 } / assign55670_e91150))), ((locals.var_psip_dn4 - (2.0 * locals.var_phibcv_dn4)) - ((2.0 * locals.var_qdsat_dn4) + (if assign55670_e91148 >= 1e-38 { (((((((locals.var_qdsat_dn4 * 2.0) * locals.var_nq) + (assign55670_e91130 * locals.var_nq_dn4)) * locals.var_inv_gam) + (assign55670_e91132 * locals.var_inv_gam_dn4)) * assign55670_e91147) + (assign55670_e91134 * ((((((locals.var_qdsat_dn4 * 2.0) * locals.var_nq) + (assign55670_e91137 * locals.var_nq_dn4)) * locals.var_inv_gam) + (assign55670_e91139 * locals.var_inv_gam_dn4)) + (((locals.var_gamcv_dn4 * assign55670_e91145) - (locals.var_gamcv * locals.var_nq_dn4)) / (assign55670_e91145 * assign55670_e91145))))) } else { 0.0 } / assign55670_e91150))), ((locals.var_psip_dn5 - (2.0 * locals.var_phibcv_dn5)) - ((2.0 * locals.var_qdsat_dn5) + (if assign55670_e91148 >= 1e-38 { (((((((locals.var_qdsat_dn5 * 2.0) * locals.var_nq) + (assign55670_e91130 * locals.var_nq_dn5)) * locals.var_inv_gam) + (assign55670_e91132 * locals.var_inv_gam_dn5)) * assign55670_e91147) + (assign55670_e91134 * ((((((locals.var_qdsat_dn5 * 2.0) * locals.var_nq) + (assign55670_e91137 * locals.var_nq_dn5)) * locals.var_inv_gam) + (assign55670_e91139 * locals.var_inv_gam_dn5)) + (((locals.var_gamcv_dn5 * assign55670_e91145) - (locals.var_gamcv * locals.var_nq_dn5)) / (assign55670_e91145 * assign55670_e91145))))) } else { 0.0 } / assign55670_e91150))), ((locals.var_psip_dn6 - (2.0 * locals.var_phibcv_dn6)) - ((2.0 * locals.var_qdsat_dn6) + (if assign55670_e91148 >= 1e-38 { (((((((locals.var_qdsat_dn6 * 2.0) * locals.var_nq) + (assign55670_e91130 * locals.var_nq_dn6)) * locals.var_inv_gam) + (assign55670_e91132 * locals.var_inv_gam_dn6)) * assign55670_e91147) + (assign55670_e91134 * ((((((locals.var_qdsat_dn6 * 2.0) * locals.var_nq) + (assign55670_e91137 * locals.var_nq_dn6)) * locals.var_inv_gam) + (assign55670_e91139 * locals.var_inv_gam_dn6)) + (((locals.var_gamcv_dn6 * assign55670_e91145) - (locals.var_gamcv * locals.var_nq_dn6)) / (assign55670_e91145 * assign55670_e91145))))) } else { 0.0 } / assign55670_e91150))), ((locals.var_psip_dn7 - (2.0 * locals.var_phibcv_dn7)) - ((2.0 * locals.var_qdsat_dn7) + (if assign55670_e91148 >= 1e-38 { (((((((locals.var_qdsat_dn7 * 2.0) * locals.var_nq) + (assign55670_e91130 * locals.var_nq_dn7)) * locals.var_inv_gam) + (assign55670_e91132 * locals.var_inv_gam_dn7)) * assign55670_e91147) + (assign55670_e91134 * ((((((locals.var_qdsat_dn7 * 2.0) * locals.var_nq) + (assign55670_e91137 * locals.var_nq_dn7)) * locals.var_inv_gam) + (assign55670_e91139 * locals.var_inv_gam_dn7)) + (((locals.var_gamcv_dn7 * assign55670_e91145) - (locals.var_gamcv * locals.var_nq_dn7)) / (assign55670_e91145 * assign55670_e91145))))) } else { 0.0 } / assign55670_e91150))), ((locals.var_psip_dn8 - (2.0 * locals.var_phibcv_dn8)) - ((2.0 * locals.var_qdsat_dn8) + (if assign55670_e91148 >= 1e-38 { (((((((locals.var_qdsat_dn8 * 2.0) * locals.var_nq) + (assign55670_e91130 * locals.var_nq_dn8)) * locals.var_inv_gam) + (assign55670_e91132 * locals.var_inv_gam_dn8)) * assign55670_e91147) + (assign55670_e91134 * ((((((locals.var_qdsat_dn8 * 2.0) * locals.var_nq) + (assign55670_e91137 * locals.var_nq_dn8)) * locals.var_inv_gam) + (assign55670_e91139 * locals.var_inv_gam_dn8)) + (((locals.var_gamcv_dn8 * assign55670_e91145) - (locals.var_gamcv * locals.var_nq_dn8)) / (assign55670_e91145 * assign55670_e91145))))) } else { 0.0 } / assign55670_e91150))), ((locals.var_psip_dn9 - (2.0 * locals.var_phibcv_dn9)) - ((2.0 * locals.var_qdsat_dn9) + (if assign55670_e91148 >= 1e-38 { (((((((locals.var_qdsat_dn9 * 2.0) * locals.var_nq) + (assign55670_e91130 * locals.var_nq_dn9)) * locals.var_inv_gam) + (assign55670_e91132 * locals.var_inv_gam_dn9)) * assign55670_e91147) + (assign55670_e91134 * ((((((locals.var_qdsat_dn9 * 2.0) * locals.var_nq) + (assign55670_e91137 * locals.var_nq_dn9)) * locals.var_inv_gam) + (assign55670_e91139 * locals.var_inv_gam_dn9)) + (((locals.var_gamcv_dn9 * assign55670_e91145) - (locals.var_gamcv * locals.var_nq_dn9)) / (assign55670_e91145 * assign55670_e91145))))) } else { 0.0 } / assign55670_e91150))), ((locals.var_psip_dn10 - (2.0 * locals.var_phibcv_dn10)) - ((2.0 * locals.var_qdsat_dn10) + (if assign55670_e91148 >= 1e-38 { (((((((locals.var_qdsat_dn10 * 2.0) * locals.var_nq) + (assign55670_e91130 * locals.var_nq_dn10)) * locals.var_inv_gam) + (assign55670_e91132 * locals.var_inv_gam_dn10)) * assign55670_e91147) + (assign55670_e91134 * ((((((locals.var_qdsat_dn10 * 2.0) * locals.var_nq) + (assign55670_e91137 * locals.var_nq_dn10)) * locals.var_inv_gam) + (assign55670_e91139 * locals.var_inv_gam_dn10)) + (((locals.var_gamcv_dn10 * assign55670_e91145) - (locals.var_gamcv * locals.var_nq_dn10)) / (assign55670_e91145 * assign55670_e91145))))) } else { 0.0 } / assign55670_e91150))), ((locals.var_psip_dn11 - (2.0 * locals.var_phibcv_dn11)) - ((2.0 * locals.var_qdsat_dn11) + (if assign55670_e91148 >= 1e-38 { (((((((locals.var_qdsat_dn11 * 2.0) * locals.var_nq) + (assign55670_e91130 * locals.var_nq_dn11)) * locals.var_inv_gam) + (assign55670_e91132 * locals.var_inv_gam_dn11)) * assign55670_e91147) + (assign55670_e91134 * ((((((locals.var_qdsat_dn11 * 2.0) * locals.var_nq) + (assign55670_e91137 * locals.var_nq_dn11)) * locals.var_inv_gam) + (assign55670_e91139 * locals.var_inv_gam_dn11)) + (((locals.var_gamcv_dn11 * assign55670_e91145) - (locals.var_gamcv * locals.var_nq_dn11)) / (assign55670_e91145 * assign55670_e91145))))) } else { 0.0 } / assign55670_e91150))),)
    } else {
        (locals.var_vdsatcv, locals.var_vdsatcv_dn3, locals.var_vdsatcv_dn4, locals.var_vdsatcv_dn5, locals.var_vdsatcv_dn6, locals.var_vdsatcv_dn7, locals.var_vdsatcv_dn8, locals.var_vdsatcv_dn9, locals.var_vdsatcv_dn10, locals.var_vdsatcv_dn11,)
    }
};
        locals.var_vdsatcv = assign55670_e91155;
        locals.var_vdsatcv_dn3 = assign55670_e91155_d_n3;
        locals.var_vdsatcv_dn4 = assign55670_e91155_d_n4;
        locals.var_vdsatcv_dn5 = assign55670_e91155_d_n5;
        locals.var_vdsatcv_dn6 = assign55670_e91155_d_n6;
        locals.var_vdsatcv_dn7 = assign55670_e91155_d_n7;
        locals.var_vdsatcv_dn8 = assign55670_e91155_d_n8;
        locals.var_vdsatcv_dn9 = assign55670_e91155_d_n9;
        locals.var_vdsatcv_dn10 = assign55670_e91155_d_n10;
        locals.var_vdsatcv_dn11 = assign55670_e91155_d_n11;

        let (assign55680_e91164, assign55680_e91164_d_n3, assign55680_e91164_d_n4, assign55680_e91164_d_n5, assign55680_e91164_d_n6, assign55680_e91164_d_n7, assign55680_e91164_d_n8, assign55680_e91164_d_n9, assign55680_e91164_d_n10, assign55680_e91164_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55680_e91162: f64 = (locals.var_vdsatcv * locals.var_vt);
        (assign55680_e91162, (locals.var_vdsatcv_dn3 * locals.var_vt), ((locals.var_vdsatcv_dn4 * locals.var_vt) + (locals.var_vdsatcv * locals.var_vt_dn4)), ((locals.var_vdsatcv_dn5 * locals.var_vt) + (locals.var_vdsatcv * locals.var_vt_dn5)), (locals.var_vdsatcv_dn6 * locals.var_vt), (locals.var_vdsatcv_dn7 * locals.var_vt), (locals.var_vdsatcv_dn8 * locals.var_vt), (locals.var_vdsatcv_dn9 * locals.var_vt), (locals.var_vdsatcv_dn10 * locals.var_vt), (locals.var_vdsatcv_dn11 * locals.var_vt),)
    } else {
        (locals.var_vdsatcv_1, locals.var_vdsatcv_1_dn3, locals.var_vdsatcv_1_dn4, locals.var_vdsatcv_1_dn5, locals.var_vdsatcv_1_dn6, locals.var_vdsatcv_1_dn7, locals.var_vdsatcv_1_dn8, locals.var_vdsatcv_1_dn9, locals.var_vdsatcv_1_dn10, locals.var_vdsatcv_1_dn11,)
    }
};
        locals.var_vdsatcv_1 = assign55680_e91164;
        locals.var_vdsatcv_1_dn3 = assign55680_e91164_d_n3;
        locals.var_vdsatcv_1_dn4 = assign55680_e91164_d_n4;
        locals.var_vdsatcv_1_dn5 = assign55680_e91164_d_n5;
        locals.var_vdsatcv_1_dn6 = assign55680_e91164_d_n6;
        locals.var_vdsatcv_1_dn7 = assign55680_e91164_d_n7;
        locals.var_vdsatcv_1_dn8 = assign55680_e91164_d_n8;
        locals.var_vdsatcv_1_dn9 = assign55680_e91164_d_n9;
        locals.var_vdsatcv_1_dn10 = assign55680_e91164_d_n10;
        locals.var_vdsatcv_1_dn11 = assign55680_e91164_d_n11;

        let (assign55690_e91196, assign55690_e91196_d_n3, assign55690_e91196_d_n4, assign55690_e91196_d_n5, assign55690_e91196_d_n6, assign55690_e91196_d_n7, assign55690_e91196_d_n8, assign55690_e91196_d_n9, assign55690_e91196_d_n10, assign55690_e91196_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55690_e91172: f64 = (locals.var_vdsatcv_1 - locals.var_vs);
        let assign55690_e91174: f64 = assign55690_e91172;
        let assign55690_e91177: f64 = (locals.var_vdsatcv_1 - locals.var_vs);
        let assign55690_e91179: f64 = assign55690_e91177;
        let assign55690_e91182: f64 = (locals.var_vdsatcv_1 - locals.var_vs);
        let assign55690_e91184: f64 = assign55690_e91182;
        let assign55690_e91185: f64 = (assign55690_e91179 * assign55690_e91184);
        let assign55690_e91188: f64 = (0.25 * 0.001);
        let assign55690_e91190: f64 = (assign55690_e91188 * 0.001);
        let assign55690_e91191: f64 = (assign55690_e91185 + assign55690_e91190);
        let assign55690_e91192: f64 = (assign55690_e91191).sqrt();
        let assign55690_e91193: f64 = (assign55690_e91174 + assign55690_e91192);
        let assign55690_e91194: f64 = (0.5 * assign55690_e91193);
        (assign55690_e91194, (0.5 * (locals.var_vdsatcv_1_dn3 + (((locals.var_vdsatcv_1_dn3 * assign55690_e91184) + (assign55690_e91179 * locals.var_vdsatcv_1_dn3)) / (2.0 * assign55690_e91192)))), (0.5 * (locals.var_vdsatcv_1_dn4 + (((locals.var_vdsatcv_1_dn4 * assign55690_e91184) + (assign55690_e91179 * locals.var_vdsatcv_1_dn4)) / (2.0 * assign55690_e91192)))), (0.5 * (locals.var_vdsatcv_1_dn5 + (((locals.var_vdsatcv_1_dn5 * assign55690_e91184) + (assign55690_e91179 * locals.var_vdsatcv_1_dn5)) / (2.0 * assign55690_e91192)))), (0.5 * ((locals.var_vdsatcv_1_dn6 - locals.var_vs_dn6) + ((((locals.var_vdsatcv_1_dn6 - locals.var_vs_dn6) * assign55690_e91184) + (assign55690_e91179 * (locals.var_vdsatcv_1_dn6 - locals.var_vs_dn6))) / (2.0 * assign55690_e91192)))), (0.5 * ((locals.var_vdsatcv_1_dn7 - locals.var_vs_dn7) + ((((locals.var_vdsatcv_1_dn7 - locals.var_vs_dn7) * assign55690_e91184) + (assign55690_e91179 * (locals.var_vdsatcv_1_dn7 - locals.var_vs_dn7))) / (2.0 * assign55690_e91192)))), (0.5 * (locals.var_vdsatcv_1_dn8 + (((locals.var_vdsatcv_1_dn8 * assign55690_e91184) + (assign55690_e91179 * locals.var_vdsatcv_1_dn8)) / (2.0 * assign55690_e91192)))), (0.5 * (locals.var_vdsatcv_1_dn9 + (((locals.var_vdsatcv_1_dn9 * assign55690_e91184) + (assign55690_e91179 * locals.var_vdsatcv_1_dn9)) / (2.0 * assign55690_e91192)))), (0.5 * ((locals.var_vdsatcv_1_dn10 - locals.var_vs_dn10) + ((((locals.var_vdsatcv_1_dn10 - locals.var_vs_dn10) * assign55690_e91184) + (assign55690_e91179 * (locals.var_vdsatcv_1_dn10 - locals.var_vs_dn10))) / (2.0 * assign55690_e91192)))), (0.5 * (locals.var_vdsatcv_1_dn11 + (((locals.var_vdsatcv_1_dn11 * assign55690_e91184) + (assign55690_e91179 * locals.var_vdsatcv_1_dn11)) / (2.0 * assign55690_e91192)))),)
    } else {
        (locals.var_vdssatcv, locals.var_vdssatcv_dn3, locals.var_vdssatcv_dn4, locals.var_vdssatcv_dn5, locals.var_vdssatcv_dn6, locals.var_vdssatcv_dn7, locals.var_vdssatcv_dn8, locals.var_vdssatcv_dn9, locals.var_vdssatcv_dn10, locals.var_vdssatcv_dn11,)
    }
};
        locals.var_vdssatcv = assign55690_e91196;
        locals.var_vdssatcv_dn3 = assign55690_e91196_d_n3;
        locals.var_vdssatcv_dn4 = assign55690_e91196_d_n4;
        locals.var_vdssatcv_dn5 = assign55690_e91196_d_n5;
        locals.var_vdssatcv_dn6 = assign55690_e91196_d_n6;
        locals.var_vdssatcv_dn7 = assign55690_e91196_d_n7;
        locals.var_vdssatcv_dn8 = assign55690_e91196_d_n8;
        locals.var_vdssatcv_dn9 = assign55690_e91196_d_n9;
        locals.var_vdssatcv_dn10 = assign55690_e91196_d_n10;
        locals.var_vdssatcv_dn11 = assign55690_e91196_d_n11;

        let assign55700_e91203: f64 = if ((p.p1353 == 0.0) && (p.p1354 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard847 = assign55700_e91203;

        let (assign55710_e91212, assign55710_e91212_d_n3, assign55710_e91212_d_n4, assign55710_e91212_d_n5, assign55710_e91212_d_n6, assign55710_e91212_d_n7, assign55710_e91212_d_n8, assign55710_e91212_d_n9, assign55710_e91212_d_n10, assign55710_e91212_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard847 != 0.0)) {
        (p.p1348, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_abulkcv, locals.var_abulkcv_dn3, locals.var_abulkcv_dn4, locals.var_abulkcv_dn5, locals.var_abulkcv_dn6, locals.var_abulkcv_dn7, locals.var_abulkcv_dn8, locals.var_abulkcv_dn9, locals.var_abulkcv_dn10, locals.var_abulkcv_dn11,)
    }
};
        locals.var_abulkcv = assign55710_e91212;
        locals.var_abulkcv_dn3 = assign55710_e91212_d_n3;
        locals.var_abulkcv_dn4 = assign55710_e91212_d_n4;
        locals.var_abulkcv_dn5 = assign55710_e91212_d_n5;
        locals.var_abulkcv_dn6 = assign55710_e91212_d_n6;
        locals.var_abulkcv_dn7 = assign55710_e91212_d_n7;
        locals.var_abulkcv_dn8 = assign55710_e91212_d_n8;
        locals.var_abulkcv_dn9 = assign55710_e91212_d_n9;
        locals.var_abulkcv_dn10 = assign55710_e91212_d_n10;
        locals.var_abulkcv_dn11 = assign55710_e91212_d_n11;

        let (assign55720_e91229, assign55720_e91229_d_n3, assign55720_e91229_d_n4, assign55720_e91229_d_n5, assign55720_e91229_d_n6, assign55720_e91229_d_n7, assign55720_e91229_d_n8, assign55720_e91229_d_n9, assign55720_e91229_d_n10, assign55720_e91229_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard847 == 0.0)) {
        let assign55720_e91224: f64 = (locals.var_xj_i * locals.var_xdep);
        let assign55720_e91225: f64 = (assign55720_e91224).sqrt();
        let assign55720_e91226: f64 = (locals.var_leff + assign55720_e91225);
        let assign55720_e91227: f64 = (locals.var_leff / assign55720_e91226);
        (assign55720_e91227, (-((locals.var_leff * ((locals.var_xj_i * locals.var_xdep_dn3) / (2.0 * assign55720_e91225))) / (assign55720_e91226 * assign55720_e91226))), (-((locals.var_leff * ((locals.var_xj_i * locals.var_xdep_dn4) / (2.0 * assign55720_e91225))) / (assign55720_e91226 * assign55720_e91226))), (-((locals.var_leff * ((locals.var_xj_i * locals.var_xdep_dn5) / (2.0 * assign55720_e91225))) / (assign55720_e91226 * assign55720_e91226))), (-((locals.var_leff * ((locals.var_xj_i * locals.var_xdep_dn6) / (2.0 * assign55720_e91225))) / (assign55720_e91226 * assign55720_e91226))), (-((locals.var_leff * ((locals.var_xj_i * locals.var_xdep_dn7) / (2.0 * assign55720_e91225))) / (assign55720_e91226 * assign55720_e91226))), (-((locals.var_leff * ((locals.var_xj_i * locals.var_xdep_dn8) / (2.0 * assign55720_e91225))) / (assign55720_e91226 * assign55720_e91226))), (-((locals.var_leff * ((locals.var_xj_i * locals.var_xdep_dn9) / (2.0 * assign55720_e91225))) / (assign55720_e91226 * assign55720_e91226))), (-((locals.var_leff * ((locals.var_xj_i * locals.var_xdep_dn10) / (2.0 * assign55720_e91225))) / (assign55720_e91226 * assign55720_e91226))), (-((locals.var_leff * ((locals.var_xj_i * locals.var_xdep_dn11) / (2.0 * assign55720_e91225))) / (assign55720_e91226 * assign55720_e91226))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign55720_e91229;
        locals.var_t1_dn3 = assign55720_e91229_d_n3;
        locals.var_t1_dn4 = assign55720_e91229_d_n4;
        locals.var_t1_dn5 = assign55720_e91229_d_n5;
        locals.var_t1_dn6 = assign55720_e91229_d_n6;
        locals.var_t1_dn7 = assign55720_e91229_d_n7;
        locals.var_t1_dn8 = assign55720_e91229_d_n8;
        locals.var_t1_dn9 = assign55720_e91229_d_n9;
        locals.var_t1_dn10 = assign55720_e91229_d_n10;
        locals.var_t1_dn11 = assign55720_e91229_d_n11;

        let (assign55730_e91257, assign55730_e91257_d_n3, assign55730_e91257_d_n4, assign55730_e91257_d_n5, assign55730_e91257_d_n6, assign55730_e91257_d_n7, assign55730_e91257_d_n8, assign55730_e91257_d_n9, assign55730_e91257_d_n10, assign55730_e91257_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard847 == 0.0)) {
        let assign55730_e91240: f64 = (p.p1353 * locals.var_t1);
        let assign55730_e91243: f64 = (p.p1354 * locals.var_t1);
        let assign55730_e91245: f64 = (assign55730_e91243 * locals.var_qs_1);
        let assign55730_e91247: f64 = (assign55730_e91245 * locals.var_nvt);
        let assign55730_e91248: f64 = (assign55730_e91240 - assign55730_e91247);
        let assign55730_e91252: f64 = (p.p1355 * locals.var_vbsx);
        let assign55730_e91253: f64 = (1.0 + assign55730_e91252);
        let assign55730_e91254: f64 = (assign55730_e91248 / assign55730_e91253);
        let assign55730_e91255: f64 = (1.0 + assign55730_e91254);
        (assign55730_e91255, (((((p.p1353 * locals.var_t1_dn3) - (((((p.p1354 * locals.var_t1_dn3) * locals.var_qs_1) + (assign55730_e91243 * locals.var_qs_1_dn3)) * locals.var_nvt) + (assign55730_e91245 * locals.var_nvt_dn3))) * assign55730_e91253) - (assign55730_e91248 * (p.p1355 * locals.var_vbsx_dn3))) / (assign55730_e91253 * assign55730_e91253)), (((((p.p1353 * locals.var_t1_dn4) - (((((p.p1354 * locals.var_t1_dn4) * locals.var_qs_1) + (assign55730_e91243 * locals.var_qs_1_dn4)) * locals.var_nvt) + (assign55730_e91245 * locals.var_nvt_dn4))) * assign55730_e91253) - (assign55730_e91248 * (p.p1355 * locals.var_vbsx_dn4))) / (assign55730_e91253 * assign55730_e91253)), (((((p.p1353 * locals.var_t1_dn5) - (((((p.p1354 * locals.var_t1_dn5) * locals.var_qs_1) + (assign55730_e91243 * locals.var_qs_1_dn5)) * locals.var_nvt) + (assign55730_e91245 * locals.var_nvt_dn5))) * assign55730_e91253) - (assign55730_e91248 * (p.p1355 * locals.var_vbsx_dn5))) / (assign55730_e91253 * assign55730_e91253)), (((((p.p1353 * locals.var_t1_dn6) - (((((p.p1354 * locals.var_t1_dn6) * locals.var_qs_1) + (assign55730_e91243 * locals.var_qs_1_dn6)) * locals.var_nvt) + (assign55730_e91245 * locals.var_nvt_dn6))) * assign55730_e91253) - (assign55730_e91248 * (p.p1355 * locals.var_vbsx_dn6))) / (assign55730_e91253 * assign55730_e91253)), (((((p.p1353 * locals.var_t1_dn7) - (((((p.p1354 * locals.var_t1_dn7) * locals.var_qs_1) + (assign55730_e91243 * locals.var_qs_1_dn7)) * locals.var_nvt) + (assign55730_e91245 * locals.var_nvt_dn7))) * assign55730_e91253) - (assign55730_e91248 * (p.p1355 * locals.var_vbsx_dn7))) / (assign55730_e91253 * assign55730_e91253)), (((((p.p1353 * locals.var_t1_dn8) - (((((p.p1354 * locals.var_t1_dn8) * locals.var_qs_1) + (assign55730_e91243 * locals.var_qs_1_dn8)) * locals.var_nvt) + (assign55730_e91245 * locals.var_nvt_dn8))) * assign55730_e91253) - (assign55730_e91248 * (p.p1355 * locals.var_vbsx_dn8))) / (assign55730_e91253 * assign55730_e91253)), (((((p.p1353 * locals.var_t1_dn9) - (((((p.p1354 * locals.var_t1_dn9) * locals.var_qs_1) + (assign55730_e91243 * locals.var_qs_1_dn9)) * locals.var_nvt) + (assign55730_e91245 * locals.var_nvt_dn9))) * assign55730_e91253) - (assign55730_e91248 * (p.p1355 * locals.var_vbsx_dn9))) / (assign55730_e91253 * assign55730_e91253)), (((((p.p1353 * locals.var_t1_dn10) - (((((p.p1354 * locals.var_t1_dn10) * locals.var_qs_1) + (assign55730_e91243 * locals.var_qs_1_dn10)) * locals.var_nvt) + (assign55730_e91245 * locals.var_nvt_dn10))) * assign55730_e91253) - (assign55730_e91248 * (p.p1355 * locals.var_vbsx_dn10))) / (assign55730_e91253 * assign55730_e91253)), (((((p.p1353 * locals.var_t1_dn11) - (((((p.p1354 * locals.var_t1_dn11) * locals.var_qs_1) + (assign55730_e91243 * locals.var_qs_1_dn11)) * locals.var_nvt) + (assign55730_e91245 * locals.var_nvt_dn11))) * assign55730_e91253) - (assign55730_e91248 * (p.p1355 * locals.var_vbsx_dn11))) / (assign55730_e91253 * assign55730_e91253)),)
    } else {
        (locals.var_abulkcv, locals.var_abulkcv_dn3, locals.var_abulkcv_dn4, locals.var_abulkcv_dn5, locals.var_abulkcv_dn6, locals.var_abulkcv_dn7, locals.var_abulkcv_dn8, locals.var_abulkcv_dn9, locals.var_abulkcv_dn10, locals.var_abulkcv_dn11,)
    }
};
        locals.var_abulkcv = assign55730_e91257;
        locals.var_abulkcv_dn3 = assign55730_e91257_d_n3;
        locals.var_abulkcv_dn4 = assign55730_e91257_d_n4;
        locals.var_abulkcv_dn5 = assign55730_e91257_d_n5;
        locals.var_abulkcv_dn6 = assign55730_e91257_d_n6;
        locals.var_abulkcv_dn7 = assign55730_e91257_d_n7;
        locals.var_abulkcv_dn8 = assign55730_e91257_d_n8;
        locals.var_abulkcv_dn9 = assign55730_e91257_d_n9;
        locals.var_abulkcv_dn10 = assign55730_e91257_d_n10;
        locals.var_abulkcv_dn11 = assign55730_e91257_d_n11;

        let (assign55740_e91286, assign55740_e91286_d_n3, assign55740_e91286_d_n4, assign55740_e91286_d_n5, assign55740_e91286_d_n6, assign55740_e91286_d_n7, assign55740_e91286_d_n8, assign55740_e91286_d_n9, assign55740_e91286_d_n10, assign55740_e91286_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard847 == 0.0)) {
        let assign55740_e91268: f64 = (locals.var_abulkcv + 0.1);
        let assign55740_e91271: f64 = (locals.var_abulkcv - 0.1);
        let assign55740_e91274: f64 = (locals.var_abulkcv - 0.1);
        let assign55740_e91275: f64 = (assign55740_e91271 * assign55740_e91274);
        let assign55740_e91278: f64 = (0.25 * 0.0005);
        let assign55740_e91280: f64 = (assign55740_e91278 * 0.0005);
        let assign55740_e91281: f64 = (assign55740_e91275 + assign55740_e91280);
        let assign55740_e91282: f64 = (assign55740_e91281).sqrt();
        let assign55740_e91283: f64 = (assign55740_e91268 + assign55740_e91282);
        let assign55740_e91284: f64 = (0.5 * assign55740_e91283);
        (assign55740_e91284, (0.5 * (locals.var_abulkcv_dn3 + (((locals.var_abulkcv_dn3 * assign55740_e91274) + (assign55740_e91271 * locals.var_abulkcv_dn3)) / (2.0 * assign55740_e91282)))), (0.5 * (locals.var_abulkcv_dn4 + (((locals.var_abulkcv_dn4 * assign55740_e91274) + (assign55740_e91271 * locals.var_abulkcv_dn4)) / (2.0 * assign55740_e91282)))), (0.5 * (locals.var_abulkcv_dn5 + (((locals.var_abulkcv_dn5 * assign55740_e91274) + (assign55740_e91271 * locals.var_abulkcv_dn5)) / (2.0 * assign55740_e91282)))), (0.5 * (locals.var_abulkcv_dn6 + (((locals.var_abulkcv_dn6 * assign55740_e91274) + (assign55740_e91271 * locals.var_abulkcv_dn6)) / (2.0 * assign55740_e91282)))), (0.5 * (locals.var_abulkcv_dn7 + (((locals.var_abulkcv_dn7 * assign55740_e91274) + (assign55740_e91271 * locals.var_abulkcv_dn7)) / (2.0 * assign55740_e91282)))), (0.5 * (locals.var_abulkcv_dn8 + (((locals.var_abulkcv_dn8 * assign55740_e91274) + (assign55740_e91271 * locals.var_abulkcv_dn8)) / (2.0 * assign55740_e91282)))), (0.5 * (locals.var_abulkcv_dn9 + (((locals.var_abulkcv_dn9 * assign55740_e91274) + (assign55740_e91271 * locals.var_abulkcv_dn9)) / (2.0 * assign55740_e91282)))), (0.5 * (locals.var_abulkcv_dn10 + (((locals.var_abulkcv_dn10 * assign55740_e91274) + (assign55740_e91271 * locals.var_abulkcv_dn10)) / (2.0 * assign55740_e91282)))), (0.5 * (locals.var_abulkcv_dn11 + (((locals.var_abulkcv_dn11 * assign55740_e91274) + (assign55740_e91271 * locals.var_abulkcv_dn11)) / (2.0 * assign55740_e91282)))),)
    } else {
        (locals.var_abulkcv, locals.var_abulkcv_dn3, locals.var_abulkcv_dn4, locals.var_abulkcv_dn5, locals.var_abulkcv_dn6, locals.var_abulkcv_dn7, locals.var_abulkcv_dn8, locals.var_abulkcv_dn9, locals.var_abulkcv_dn10, locals.var_abulkcv_dn11,)
    }
};
        locals.var_abulkcv = assign55740_e91286;
        locals.var_abulkcv_dn3 = assign55740_e91286_d_n3;
        locals.var_abulkcv_dn4 = assign55740_e91286_d_n4;
        locals.var_abulkcv_dn5 = assign55740_e91286_d_n5;
        locals.var_abulkcv_dn6 = assign55740_e91286_d_n6;
        locals.var_abulkcv_dn7 = assign55740_e91286_d_n7;
        locals.var_abulkcv_dn8 = assign55740_e91286_d_n8;
        locals.var_abulkcv_dn9 = assign55740_e91286_d_n9;
        locals.var_abulkcv_dn10 = assign55740_e91286_d_n10;
        locals.var_abulkcv_dn11 = assign55740_e91286_d_n11;

        let (assign55750_e91295, assign55750_e91295_d_n3, assign55750_e91295_d_n4, assign55750_e91295_d_n5, assign55750_e91295_d_n6, assign55750_e91295_d_n7, assign55750_e91295_d_n8, assign55750_e91295_d_n9, assign55750_e91295_d_n10, assign55750_e91295_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55750_e91293: f64 = (locals.var_vdssatcv / locals.var_abulkcv);
        (assign55750_e91293, (((locals.var_vdssatcv_dn3 * locals.var_abulkcv) - (locals.var_vdssatcv * locals.var_abulkcv_dn3)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_vdssatcv_dn4 * locals.var_abulkcv) - (locals.var_vdssatcv * locals.var_abulkcv_dn4)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_vdssatcv_dn5 * locals.var_abulkcv) - (locals.var_vdssatcv * locals.var_abulkcv_dn5)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_vdssatcv_dn6 * locals.var_abulkcv) - (locals.var_vdssatcv * locals.var_abulkcv_dn6)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_vdssatcv_dn7 * locals.var_abulkcv) - (locals.var_vdssatcv * locals.var_abulkcv_dn7)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_vdssatcv_dn8 * locals.var_abulkcv) - (locals.var_vdssatcv * locals.var_abulkcv_dn8)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_vdssatcv_dn9 * locals.var_abulkcv) - (locals.var_vdssatcv * locals.var_abulkcv_dn9)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_vdssatcv_dn10 * locals.var_abulkcv) - (locals.var_vdssatcv * locals.var_abulkcv_dn10)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_vdssatcv_dn11 * locals.var_abulkcv) - (locals.var_vdssatcv * locals.var_abulkcv_dn11)) / (locals.var_abulkcv * locals.var_abulkcv)),)
    } else {
        (locals.var_vdssatcv, locals.var_vdssatcv_dn3, locals.var_vdssatcv_dn4, locals.var_vdssatcv_dn5, locals.var_vdssatcv_dn6, locals.var_vdssatcv_dn7, locals.var_vdssatcv_dn8, locals.var_vdssatcv_dn9, locals.var_vdssatcv_dn10, locals.var_vdssatcv_dn11,)
    }
};
        locals.var_vdssatcv = assign55750_e91295;
        locals.var_vdssatcv_dn3 = assign55750_e91295_d_n3;
        locals.var_vdssatcv_dn4 = assign55750_e91295_d_n4;
        locals.var_vdssatcv_dn5 = assign55750_e91295_d_n5;
        locals.var_vdssatcv_dn6 = assign55750_e91295_d_n6;
        locals.var_vdssatcv_dn7 = assign55750_e91295_d_n7;
        locals.var_vdssatcv_dn8 = assign55750_e91295_d_n8;
        locals.var_vdssatcv_dn9 = assign55750_e91295_d_n9;
        locals.var_vdssatcv_dn10 = assign55750_e91295_d_n10;
        locals.var_vdssatcv_dn11 = assign55750_e91295_d_n11;

    }

    pub(super) fn stamp_transient_block_189(
        locals: &mut StampLocals,
    ) {
        let (assign55760_e91310, assign55760_e91310_d_n3, assign55760_e91310_d_n4, assign55760_e91310_d_n5, assign55760_e91310_d_n6, assign55760_e91310_d_n7, assign55760_e91310_d_n8, assign55760_e91310_d_n9, assign55760_e91310_d_n10, assign55760_e91310_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55760_e91302: f64 = (locals.var_vds / locals.var_vdssatcv);
        let assign55760_e91304: f64 = (assign55760_e91302 + 1e-6);
        let assign55760_e91307: f64 = (1.0 / locals.var_delta_t);
        let assign55760_e91308: f64 = (assign55760_e91304).powf(assign55760_e91307);
        (assign55760_e91308, if (-(locals.var_delta_t_dn3 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign55760_e91307) as f64).is_finite() && ((assign55760_e91307) as f64).fract() == 0.0 { if assign55760_e91307 == 0.0 { 0.0 } else { (assign55760_e91307 * ((assign55760_e91304).powf(assign55760_e91307 - 1.0) * (-((locals.var_vds * locals.var_vdssatcv_dn3) / (locals.var_vdssatcv * locals.var_vdssatcv))))) } } else { (assign55760_e91308 * (((-(locals.var_delta_t_dn3 / (locals.var_delta_t * locals.var_delta_t))) * (assign55760_e91304).ln()) + (assign55760_e91307 * ((-((locals.var_vds * locals.var_vdssatcv_dn3) / (locals.var_vdssatcv * locals.var_vdssatcv))) / assign55760_e91304)))) }, if (-(locals.var_delta_t_dn4 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign55760_e91307) as f64).is_finite() && ((assign55760_e91307) as f64).fract() == 0.0 { if assign55760_e91307 == 0.0 { 0.0 } else { (assign55760_e91307 * ((assign55760_e91304).powf(assign55760_e91307 - 1.0) * (-((locals.var_vds * locals.var_vdssatcv_dn4) / (locals.var_vdssatcv * locals.var_vdssatcv))))) } } else { (assign55760_e91308 * (((-(locals.var_delta_t_dn4 / (locals.var_delta_t * locals.var_delta_t))) * (assign55760_e91304).ln()) + (assign55760_e91307 * ((-((locals.var_vds * locals.var_vdssatcv_dn4) / (locals.var_vdssatcv * locals.var_vdssatcv))) / assign55760_e91304)))) }, if (-(locals.var_delta_t_dn5 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign55760_e91307) as f64).is_finite() && ((assign55760_e91307) as f64).fract() == 0.0 { if assign55760_e91307 == 0.0 { 0.0 } else { (assign55760_e91307 * ((assign55760_e91304).powf(assign55760_e91307 - 1.0) * (-((locals.var_vds * locals.var_vdssatcv_dn5) / (locals.var_vdssatcv * locals.var_vdssatcv))))) } } else { (assign55760_e91308 * (((-(locals.var_delta_t_dn5 / (locals.var_delta_t * locals.var_delta_t))) * (assign55760_e91304).ln()) + (assign55760_e91307 * ((-((locals.var_vds * locals.var_vdssatcv_dn5) / (locals.var_vdssatcv * locals.var_vdssatcv))) / assign55760_e91304)))) }, if (-(locals.var_delta_t_dn6 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign55760_e91307) as f64).is_finite() && ((assign55760_e91307) as f64).fract() == 0.0 { if assign55760_e91307 == 0.0 { 0.0 } else { (assign55760_e91307 * ((assign55760_e91304).powf(assign55760_e91307 - 1.0) * (((locals.var_vds_dn6 * locals.var_vdssatcv) - (locals.var_vds * locals.var_vdssatcv_dn6)) / (locals.var_vdssatcv * locals.var_vdssatcv)))) } } else { (assign55760_e91308 * (((-(locals.var_delta_t_dn6 / (locals.var_delta_t * locals.var_delta_t))) * (assign55760_e91304).ln()) + (assign55760_e91307 * ((((locals.var_vds_dn6 * locals.var_vdssatcv) - (locals.var_vds * locals.var_vdssatcv_dn6)) / (locals.var_vdssatcv * locals.var_vdssatcv)) / assign55760_e91304)))) }, if (-(locals.var_delta_t_dn7 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign55760_e91307) as f64).is_finite() && ((assign55760_e91307) as f64).fract() == 0.0 { if assign55760_e91307 == 0.0 { 0.0 } else { (assign55760_e91307 * ((assign55760_e91304).powf(assign55760_e91307 - 1.0) * (((locals.var_vds_dn7 * locals.var_vdssatcv) - (locals.var_vds * locals.var_vdssatcv_dn7)) / (locals.var_vdssatcv * locals.var_vdssatcv)))) } } else { (assign55760_e91308 * (((-(locals.var_delta_t_dn7 / (locals.var_delta_t * locals.var_delta_t))) * (assign55760_e91304).ln()) + (assign55760_e91307 * ((((locals.var_vds_dn7 * locals.var_vdssatcv) - (locals.var_vds * locals.var_vdssatcv_dn7)) / (locals.var_vdssatcv * locals.var_vdssatcv)) / assign55760_e91304)))) }, if (-(locals.var_delta_t_dn8 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign55760_e91307) as f64).is_finite() && ((assign55760_e91307) as f64).fract() == 0.0 { if assign55760_e91307 == 0.0 { 0.0 } else { (assign55760_e91307 * ((assign55760_e91304).powf(assign55760_e91307 - 1.0) * (-((locals.var_vds * locals.var_vdssatcv_dn8) / (locals.var_vdssatcv * locals.var_vdssatcv))))) } } else { (assign55760_e91308 * (((-(locals.var_delta_t_dn8 / (locals.var_delta_t * locals.var_delta_t))) * (assign55760_e91304).ln()) + (assign55760_e91307 * ((-((locals.var_vds * locals.var_vdssatcv_dn8) / (locals.var_vdssatcv * locals.var_vdssatcv))) / assign55760_e91304)))) }, if (-(locals.var_delta_t_dn9 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign55760_e91307) as f64).is_finite() && ((assign55760_e91307) as f64).fract() == 0.0 { if assign55760_e91307 == 0.0 { 0.0 } else { (assign55760_e91307 * ((assign55760_e91304).powf(assign55760_e91307 - 1.0) * (-((locals.var_vds * locals.var_vdssatcv_dn9) / (locals.var_vdssatcv * locals.var_vdssatcv))))) } } else { (assign55760_e91308 * (((-(locals.var_delta_t_dn9 / (locals.var_delta_t * locals.var_delta_t))) * (assign55760_e91304).ln()) + (assign55760_e91307 * ((-((locals.var_vds * locals.var_vdssatcv_dn9) / (locals.var_vdssatcv * locals.var_vdssatcv))) / assign55760_e91304)))) }, if (-(locals.var_delta_t_dn10 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign55760_e91307) as f64).is_finite() && ((assign55760_e91307) as f64).fract() == 0.0 { if assign55760_e91307 == 0.0 { 0.0 } else { (assign55760_e91307 * ((assign55760_e91304).powf(assign55760_e91307 - 1.0) * (((locals.var_vds_dn10 * locals.var_vdssatcv) - (locals.var_vds * locals.var_vdssatcv_dn10)) / (locals.var_vdssatcv * locals.var_vdssatcv)))) } } else { (assign55760_e91308 * (((-(locals.var_delta_t_dn10 / (locals.var_delta_t * locals.var_delta_t))) * (assign55760_e91304).ln()) + (assign55760_e91307 * ((((locals.var_vds_dn10 * locals.var_vdssatcv) - (locals.var_vds * locals.var_vdssatcv_dn10)) / (locals.var_vdssatcv * locals.var_vdssatcv)) / assign55760_e91304)))) }, if (-(locals.var_delta_t_dn11 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign55760_e91307) as f64).is_finite() && ((assign55760_e91307) as f64).fract() == 0.0 { if assign55760_e91307 == 0.0 { 0.0 } else { (assign55760_e91307 * ((assign55760_e91304).powf(assign55760_e91307 - 1.0) * (-((locals.var_vds * locals.var_vdssatcv_dn11) / (locals.var_vdssatcv * locals.var_vdssatcv))))) } } else { (assign55760_e91308 * (((-(locals.var_delta_t_dn11 / (locals.var_delta_t * locals.var_delta_t))) * (assign55760_e91304).ln()) + (assign55760_e91307 * ((-((locals.var_vds * locals.var_vdssatcv_dn11) / (locals.var_vdssatcv * locals.var_vdssatcv))) / assign55760_e91304)))) },)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign55760_e91310;
        locals.var_t7_dn3 = assign55760_e91310_d_n3;
        locals.var_t7_dn4 = assign55760_e91310_d_n4;
        locals.var_t7_dn5 = assign55760_e91310_d_n5;
        locals.var_t7_dn6 = assign55760_e91310_d_n6;
        locals.var_t7_dn7 = assign55760_e91310_d_n7;
        locals.var_t7_dn8 = assign55760_e91310_d_n8;
        locals.var_t7_dn9 = assign55760_e91310_d_n9;
        locals.var_t7_dn10 = assign55760_e91310_d_n10;
        locals.var_t7_dn11 = assign55760_e91310_d_n11;

        let (assign55770_e91322, assign55770_e91322_d_n3, assign55770_e91322_d_n4, assign55770_e91322_d_n5, assign55770_e91322_d_n6, assign55770_e91322_d_n7, assign55770_e91322_d_n8, assign55770_e91322_d_n9, assign55770_e91322_d_n10, assign55770_e91322_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55770_e91317: f64 = (1.0 + locals.var_t7);
        let assign55770_e91319: f64 = (-locals.var_delta_t);
        let assign55770_e91320: f64 = (assign55770_e91317).powf(assign55770_e91319);
        (assign55770_e91320, if (-locals.var_delta_t_dn3) == 0.0 && ((assign55770_e91319) as f64).is_finite() && ((assign55770_e91319) as f64).fract() == 0.0 { if assign55770_e91319 == 0.0 { 0.0 } else { (assign55770_e91319 * ((assign55770_e91317).powf(assign55770_e91319 - 1.0) * locals.var_t7_dn3)) } } else { (assign55770_e91320 * (((-locals.var_delta_t_dn3) * (assign55770_e91317).ln()) + (assign55770_e91319 * (locals.var_t7_dn3 / assign55770_e91317)))) }, if (-locals.var_delta_t_dn4) == 0.0 && ((assign55770_e91319) as f64).is_finite() && ((assign55770_e91319) as f64).fract() == 0.0 { if assign55770_e91319 == 0.0 { 0.0 } else { (assign55770_e91319 * ((assign55770_e91317).powf(assign55770_e91319 - 1.0) * locals.var_t7_dn4)) } } else { (assign55770_e91320 * (((-locals.var_delta_t_dn4) * (assign55770_e91317).ln()) + (assign55770_e91319 * (locals.var_t7_dn4 / assign55770_e91317)))) }, if (-locals.var_delta_t_dn5) == 0.0 && ((assign55770_e91319) as f64).is_finite() && ((assign55770_e91319) as f64).fract() == 0.0 { if assign55770_e91319 == 0.0 { 0.0 } else { (assign55770_e91319 * ((assign55770_e91317).powf(assign55770_e91319 - 1.0) * locals.var_t7_dn5)) } } else { (assign55770_e91320 * (((-locals.var_delta_t_dn5) * (assign55770_e91317).ln()) + (assign55770_e91319 * (locals.var_t7_dn5 / assign55770_e91317)))) }, if (-locals.var_delta_t_dn6) == 0.0 && ((assign55770_e91319) as f64).is_finite() && ((assign55770_e91319) as f64).fract() == 0.0 { if assign55770_e91319 == 0.0 { 0.0 } else { (assign55770_e91319 * ((assign55770_e91317).powf(assign55770_e91319 - 1.0) * locals.var_t7_dn6)) } } else { (assign55770_e91320 * (((-locals.var_delta_t_dn6) * (assign55770_e91317).ln()) + (assign55770_e91319 * (locals.var_t7_dn6 / assign55770_e91317)))) }, if (-locals.var_delta_t_dn7) == 0.0 && ((assign55770_e91319) as f64).is_finite() && ((assign55770_e91319) as f64).fract() == 0.0 { if assign55770_e91319 == 0.0 { 0.0 } else { (assign55770_e91319 * ((assign55770_e91317).powf(assign55770_e91319 - 1.0) * locals.var_t7_dn7)) } } else { (assign55770_e91320 * (((-locals.var_delta_t_dn7) * (assign55770_e91317).ln()) + (assign55770_e91319 * (locals.var_t7_dn7 / assign55770_e91317)))) }, if (-locals.var_delta_t_dn8) == 0.0 && ((assign55770_e91319) as f64).is_finite() && ((assign55770_e91319) as f64).fract() == 0.0 { if assign55770_e91319 == 0.0 { 0.0 } else { (assign55770_e91319 * ((assign55770_e91317).powf(assign55770_e91319 - 1.0) * locals.var_t7_dn8)) } } else { (assign55770_e91320 * (((-locals.var_delta_t_dn8) * (assign55770_e91317).ln()) + (assign55770_e91319 * (locals.var_t7_dn8 / assign55770_e91317)))) }, if (-locals.var_delta_t_dn9) == 0.0 && ((assign55770_e91319) as f64).is_finite() && ((assign55770_e91319) as f64).fract() == 0.0 { if assign55770_e91319 == 0.0 { 0.0 } else { (assign55770_e91319 * ((assign55770_e91317).powf(assign55770_e91319 - 1.0) * locals.var_t7_dn9)) } } else { (assign55770_e91320 * (((-locals.var_delta_t_dn9) * (assign55770_e91317).ln()) + (assign55770_e91319 * (locals.var_t7_dn9 / assign55770_e91317)))) }, if (-locals.var_delta_t_dn10) == 0.0 && ((assign55770_e91319) as f64).is_finite() && ((assign55770_e91319) as f64).fract() == 0.0 { if assign55770_e91319 == 0.0 { 0.0 } else { (assign55770_e91319 * ((assign55770_e91317).powf(assign55770_e91319 - 1.0) * locals.var_t7_dn10)) } } else { (assign55770_e91320 * (((-locals.var_delta_t_dn10) * (assign55770_e91317).ln()) + (assign55770_e91319 * (locals.var_t7_dn10 / assign55770_e91317)))) }, if (-locals.var_delta_t_dn11) == 0.0 && ((assign55770_e91319) as f64).is_finite() && ((assign55770_e91319) as f64).fract() == 0.0 { if assign55770_e91319 == 0.0 { 0.0 } else { (assign55770_e91319 * ((assign55770_e91317).powf(assign55770_e91319 - 1.0) * locals.var_t7_dn11)) } } else { (assign55770_e91320 * (((-locals.var_delta_t_dn11) * (assign55770_e91317).ln()) + (assign55770_e91319 * (locals.var_t7_dn11 / assign55770_e91317)))) },)
    } else {
        (locals.var_t8, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11,)
    }
};
        locals.var_t8 = assign55770_e91322;
        locals.var_t8_dn3 = assign55770_e91322_d_n3;
        locals.var_t8_dn4 = assign55770_e91322_d_n4;
        locals.var_t8_dn5 = assign55770_e91322_d_n5;
        locals.var_t8_dn6 = assign55770_e91322_d_n6;
        locals.var_t8_dn7 = assign55770_e91322_d_n7;
        locals.var_t8_dn8 = assign55770_e91322_d_n8;
        locals.var_t8_dn9 = assign55770_e91322_d_n9;
        locals.var_t8_dn10 = assign55770_e91322_d_n10;
        locals.var_t8_dn11 = assign55770_e91322_d_n11;

        let (assign55780_e91331, assign55780_e91331_d_n3, assign55780_e91331_d_n4, assign55780_e91331_d_n5, assign55780_e91331_d_n6, assign55780_e91331_d_n7, assign55780_e91331_d_n8, assign55780_e91331_d_n9, assign55780_e91331_d_n10, assign55780_e91331_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55780_e91329: f64 = (locals.var_vds * locals.var_t8);
        (assign55780_e91329, (locals.var_vds * locals.var_t8_dn3), (locals.var_vds * locals.var_t8_dn4), (locals.var_vds * locals.var_t8_dn5), ((locals.var_vds_dn6 * locals.var_t8) + (locals.var_vds * locals.var_t8_dn6)), ((locals.var_vds_dn7 * locals.var_t8) + (locals.var_vds * locals.var_t8_dn7)), (locals.var_vds * locals.var_t8_dn8), (locals.var_vds * locals.var_t8_dn9), ((locals.var_vds_dn10 * locals.var_t8) + (locals.var_vds * locals.var_t8_dn10)), (locals.var_vds * locals.var_t8_dn11),)
    } else {
        (locals.var_vdseff, locals.var_vdseff_dn3, locals.var_vdseff_dn4, locals.var_vdseff_dn5, locals.var_vdseff_dn6, locals.var_vdseff_dn7, locals.var_vdseff_dn8, locals.var_vdseff_dn9, locals.var_vdseff_dn10, locals.var_vdseff_dn11,)
    }
};
        locals.var_vdseff = assign55780_e91331;
        locals.var_vdseff_dn3 = assign55780_e91331_d_n3;
        locals.var_vdseff_dn4 = assign55780_e91331_d_n4;
        locals.var_vdseff_dn5 = assign55780_e91331_d_n5;
        locals.var_vdseff_dn6 = assign55780_e91331_d_n6;
        locals.var_vdseff_dn7 = assign55780_e91331_d_n7;
        locals.var_vdseff_dn8 = assign55780_e91331_d_n8;
        locals.var_vdseff_dn9 = assign55780_e91331_d_n9;
        locals.var_vdseff_dn10 = assign55780_e91331_d_n10;
        locals.var_vdseff_dn11 = assign55780_e91331_d_n11;

        let (assign55790_e91342, assign55790_e91342_d_n3, assign55790_e91342_d_n4, assign55790_e91342_d_n5, assign55790_e91342_d_n6, assign55790_e91342_d_n7, assign55790_e91342_d_n8, assign55790_e91342_d_n9, assign55790_e91342_d_n10, assign55790_e91342_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55790_e91338: f64 = (locals.var_vdseff + locals.var_vs);
        let assign55790_e91340: f64 = (assign55790_e91338 * locals.var_inv_vt);
        (assign55790_e91340, (locals.var_vdseff_dn3 * locals.var_inv_vt), ((locals.var_vdseff_dn4 * locals.var_inv_vt) + (assign55790_e91338 * locals.var_inv_vt_dn4)), ((locals.var_vdseff_dn5 * locals.var_inv_vt) + (assign55790_e91338 * locals.var_inv_vt_dn5)), ((locals.var_vdseff_dn6 + locals.var_vs_dn6) * locals.var_inv_vt), ((locals.var_vdseff_dn7 + locals.var_vs_dn7) * locals.var_inv_vt), (locals.var_vdseff_dn8 * locals.var_inv_vt), (locals.var_vdseff_dn9 * locals.var_inv_vt), ((locals.var_vdseff_dn10 + locals.var_vs_dn10) * locals.var_inv_vt), (locals.var_vdseff_dn11 * locals.var_inv_vt),)
    } else {
        (locals.var_vdeff, locals.var_vdeff_dn3, locals.var_vdeff_dn4, locals.var_vdeff_dn5, locals.var_vdeff_dn6, locals.var_vdeff_dn7, locals.var_vdeff_dn8, locals.var_vdeff_dn9, locals.var_vdeff_dn10, locals.var_vdeff_dn11,)
    }
};
        locals.var_vdeff = assign55790_e91342;
        locals.var_vdeff_dn3 = assign55790_e91342_d_n3;
        locals.var_vdeff_dn4 = assign55790_e91342_d_n4;
        locals.var_vdeff_dn5 = assign55790_e91342_d_n5;
        locals.var_vdeff_dn6 = assign55790_e91342_d_n6;
        locals.var_vdeff_dn7 = assign55790_e91342_d_n7;
        locals.var_vdeff_dn8 = assign55790_e91342_d_n8;
        locals.var_vdeff_dn9 = assign55790_e91342_d_n9;
        locals.var_vdeff_dn10 = assign55790_e91342_d_n10;
        locals.var_vdeff_dn11 = assign55790_e91342_d_n11;

        let (assign55800_e91368, assign55800_e91368_d_n3, assign55800_e91368_d_n4, assign55800_e91368_d_n5, assign55800_e91368_d_n6, assign55800_e91368_d_n7, assign55800_e91368_d_n8, assign55800_e91368_d_n9, assign55800_e91368_d_n10, assign55800_e91368_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55800_e91350: f64 = (locals.var_psip + 1.0);
        let assign55800_e91353: f64 = (locals.var_psip - 1.0);
        let assign55800_e91356: f64 = (locals.var_psip - 1.0);
        let assign55800_e91357: f64 = (assign55800_e91353 * assign55800_e91356);
        let assign55800_e91360: f64 = (0.25 * 2.0);
        let assign55800_e91362: f64 = (assign55800_e91360 * 2.0);
        let assign55800_e91363: f64 = (assign55800_e91357 + assign55800_e91362);
        let assign55800_e91364: f64 = (assign55800_e91363).sqrt();
        let assign55800_e91365: f64 = (assign55800_e91350 + assign55800_e91364);
        let assign55800_e91366: f64 = (0.5 * assign55800_e91365);
        (assign55800_e91366, (0.5 * (locals.var_psip_dn3 + (((locals.var_psip_dn3 * assign55800_e91356) + (assign55800_e91353 * locals.var_psip_dn3)) / (2.0 * assign55800_e91364)))), (0.5 * (locals.var_psip_dn4 + (((locals.var_psip_dn4 * assign55800_e91356) + (assign55800_e91353 * locals.var_psip_dn4)) / (2.0 * assign55800_e91364)))), (0.5 * (locals.var_psip_dn5 + (((locals.var_psip_dn5 * assign55800_e91356) + (assign55800_e91353 * locals.var_psip_dn5)) / (2.0 * assign55800_e91364)))), (0.5 * (locals.var_psip_dn6 + (((locals.var_psip_dn6 * assign55800_e91356) + (assign55800_e91353 * locals.var_psip_dn6)) / (2.0 * assign55800_e91364)))), (0.5 * (locals.var_psip_dn7 + (((locals.var_psip_dn7 * assign55800_e91356) + (assign55800_e91353 * locals.var_psip_dn7)) / (2.0 * assign55800_e91364)))), (0.5 * (locals.var_psip_dn8 + (((locals.var_psip_dn8 * assign55800_e91356) + (assign55800_e91353 * locals.var_psip_dn8)) / (2.0 * assign55800_e91364)))), (0.5 * (locals.var_psip_dn9 + (((locals.var_psip_dn9 * assign55800_e91356) + (assign55800_e91353 * locals.var_psip_dn9)) / (2.0 * assign55800_e91364)))), (0.5 * (locals.var_psip_dn10 + (((locals.var_psip_dn10 * assign55800_e91356) + (assign55800_e91353 * locals.var_psip_dn10)) / (2.0 * assign55800_e91364)))), (0.5 * (locals.var_psip_dn11 + (((locals.var_psip_dn11 * assign55800_e91356) + (assign55800_e91353 * locals.var_psip_dn11)) / (2.0 * assign55800_e91364)))),)
    } else {
        (locals.var_t8, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11,)
    }
};
        locals.var_t8 = assign55800_e91368;
        locals.var_t8_dn3 = assign55800_e91368_d_n3;
        locals.var_t8_dn4 = assign55800_e91368_d_n4;
        locals.var_t8_dn5 = assign55800_e91368_d_n5;
        locals.var_t8_dn6 = assign55800_e91368_d_n6;
        locals.var_t8_dn7 = assign55800_e91368_d_n7;
        locals.var_t8_dn8 = assign55800_e91368_d_n8;
        locals.var_t8_dn9 = assign55800_e91368_d_n9;
        locals.var_t8_dn10 = assign55800_e91368_d_n10;
        locals.var_t8_dn11 = assign55800_e91368_d_n11;

        let (assign55810_e91376, assign55810_e91376_d_n3, assign55810_e91376_d_n4, assign55810_e91376_d_n5, assign55810_e91376_d_n6, assign55810_e91376_d_n7, assign55810_e91376_d_n8, assign55810_e91376_d_n9, assign55810_e91376_d_n10, assign55810_e91376_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55810_e91374: f64 = (locals.var_t8).sqrt();
        (assign55810_e91374, (locals.var_t8_dn3 / (2.0 * assign55810_e91374)), (locals.var_t8_dn4 / (2.0 * assign55810_e91374)), (locals.var_t8_dn5 / (2.0 * assign55810_e91374)), (locals.var_t8_dn6 / (2.0 * assign55810_e91374)), (locals.var_t8_dn7 / (2.0 * assign55810_e91374)), (locals.var_t8_dn8 / (2.0 * assign55810_e91374)), (locals.var_t8_dn9 / (2.0 * assign55810_e91374)), (locals.var_t8_dn10 / (2.0 * assign55810_e91374)), (locals.var_t8_dn11 / (2.0 * assign55810_e91374)),)
    } else {
        (locals.var_sqrtpsip, locals.var_sqrtpsip_dn3, locals.var_sqrtpsip_dn4, locals.var_sqrtpsip_dn5, locals.var_sqrtpsip_dn6, locals.var_sqrtpsip_dn7, locals.var_sqrtpsip_dn8, locals.var_sqrtpsip_dn9, locals.var_sqrtpsip_dn10, locals.var_sqrtpsip_dn11,)
    }
};
        locals.var_sqrtpsip = assign55810_e91376;
        locals.var_sqrtpsip_dn3 = assign55810_e91376_d_n3;
        locals.var_sqrtpsip_dn4 = assign55810_e91376_d_n4;
        locals.var_sqrtpsip_dn5 = assign55810_e91376_d_n5;
        locals.var_sqrtpsip_dn6 = assign55810_e91376_d_n6;
        locals.var_sqrtpsip_dn7 = assign55810_e91376_d_n7;
        locals.var_sqrtpsip_dn8 = assign55810_e91376_d_n8;
        locals.var_sqrtpsip_dn9 = assign55810_e91376_d_n9;
        locals.var_sqrtpsip_dn10 = assign55810_e91376_d_n10;
        locals.var_sqrtpsip_dn11 = assign55810_e91376_d_n11;

        let (assign55820_e91391, assign55820_e91391_d_n3, assign55820_e91391_d_n4, assign55820_e91391_d_n5, assign55820_e91391_d_n6, assign55820_e91391_d_n7, assign55820_e91391_d_n8, assign55820_e91391_d_n9, assign55820_e91391_d_n10, assign55820_e91391_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55820_e91385: f64 = (2.0 * locals.var_sqrtpsip);
        let assign55820_e91386: f64 = (locals.var_gamcv / assign55820_e91385);
        let assign55820_e91387: f64 = (1.0 + assign55820_e91386);
        let assign55820_e91389: f64 = (assign55820_e91387 / locals.var_gamcv);
        (assign55820_e91389, ((((((locals.var_gamcv_dn3 * assign55820_e91385) - (locals.var_gamcv * (2.0 * locals.var_sqrtpsip_dn3))) / (assign55820_e91385 * assign55820_e91385)) * locals.var_gamcv) - (assign55820_e91387 * locals.var_gamcv_dn3)) / (locals.var_gamcv * locals.var_gamcv)), ((((((locals.var_gamcv_dn4 * assign55820_e91385) - (locals.var_gamcv * (2.0 * locals.var_sqrtpsip_dn4))) / (assign55820_e91385 * assign55820_e91385)) * locals.var_gamcv) - (assign55820_e91387 * locals.var_gamcv_dn4)) / (locals.var_gamcv * locals.var_gamcv)), ((((((locals.var_gamcv_dn5 * assign55820_e91385) - (locals.var_gamcv * (2.0 * locals.var_sqrtpsip_dn5))) / (assign55820_e91385 * assign55820_e91385)) * locals.var_gamcv) - (assign55820_e91387 * locals.var_gamcv_dn5)) / (locals.var_gamcv * locals.var_gamcv)), ((((((locals.var_gamcv_dn6 * assign55820_e91385) - (locals.var_gamcv * (2.0 * locals.var_sqrtpsip_dn6))) / (assign55820_e91385 * assign55820_e91385)) * locals.var_gamcv) - (assign55820_e91387 * locals.var_gamcv_dn6)) / (locals.var_gamcv * locals.var_gamcv)), ((((((locals.var_gamcv_dn7 * assign55820_e91385) - (locals.var_gamcv * (2.0 * locals.var_sqrtpsip_dn7))) / (assign55820_e91385 * assign55820_e91385)) * locals.var_gamcv) - (assign55820_e91387 * locals.var_gamcv_dn7)) / (locals.var_gamcv * locals.var_gamcv)), ((((((locals.var_gamcv_dn8 * assign55820_e91385) - (locals.var_gamcv * (2.0 * locals.var_sqrtpsip_dn8))) / (assign55820_e91385 * assign55820_e91385)) * locals.var_gamcv) - (assign55820_e91387 * locals.var_gamcv_dn8)) / (locals.var_gamcv * locals.var_gamcv)), ((((((locals.var_gamcv_dn9 * assign55820_e91385) - (locals.var_gamcv * (2.0 * locals.var_sqrtpsip_dn9))) / (assign55820_e91385 * assign55820_e91385)) * locals.var_gamcv) - (assign55820_e91387 * locals.var_gamcv_dn9)) / (locals.var_gamcv * locals.var_gamcv)), ((((((locals.var_gamcv_dn10 * assign55820_e91385) - (locals.var_gamcv * (2.0 * locals.var_sqrtpsip_dn10))) / (assign55820_e91385 * assign55820_e91385)) * locals.var_gamcv) - (assign55820_e91387 * locals.var_gamcv_dn10)) / (locals.var_gamcv * locals.var_gamcv)), ((((((locals.var_gamcv_dn11 * assign55820_e91385) - (locals.var_gamcv * (2.0 * locals.var_sqrtpsip_dn11))) / (assign55820_e91385 * assign55820_e91385)) * locals.var_gamcv) - (assign55820_e91387 * locals.var_gamcv_dn11)) / (locals.var_gamcv * locals.var_gamcv)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign55820_e91391;
        locals.var_t0_dn3 = assign55820_e91391_d_n3;
        locals.var_t0_dn4 = assign55820_e91391_d_n4;
        locals.var_t0_dn5 = assign55820_e91391_d_n5;
        locals.var_t0_dn6 = assign55820_e91391_d_n6;
        locals.var_t0_dn7 = assign55820_e91391_d_n7;
        locals.var_t0_dn8 = assign55820_e91391_d_n8;
        locals.var_t0_dn9 = assign55820_e91391_d_n9;
        locals.var_t0_dn10 = assign55820_e91391_d_n10;
        locals.var_t0_dn11 = assign55820_e91391_d_n11;

        let (assign55830_e91404, assign55830_e91404_d_n3, assign55830_e91404_d_n4, assign55830_e91404_d_n5, assign55830_e91404_d_n6, assign55830_e91404_d_n7, assign55830_e91404_d_n8, assign55830_e91404_d_n9, assign55830_e91404_d_n10, assign55830_e91404_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55830_e91399: f64 = (2.0 * locals.var_phibcv);
        let assign55830_e91400: f64 = (locals.var_psip - assign55830_e91399);
        let assign55830_e91402: f64 = (assign55830_e91400 - locals.var_vdeff);
        (assign55830_e91402, ((locals.var_psip_dn3 - (2.0 * locals.var_phibcv_dn3)) - locals.var_vdeff_dn3), ((locals.var_psip_dn4 - (2.0 * locals.var_phibcv_dn4)) - locals.var_vdeff_dn4), ((locals.var_psip_dn5 - (2.0 * locals.var_phibcv_dn5)) - locals.var_vdeff_dn5), ((locals.var_psip_dn6 - (2.0 * locals.var_phibcv_dn6)) - locals.var_vdeff_dn6), ((locals.var_psip_dn7 - (2.0 * locals.var_phibcv_dn7)) - locals.var_vdeff_dn7), ((locals.var_psip_dn8 - (2.0 * locals.var_phibcv_dn8)) - locals.var_vdeff_dn8), ((locals.var_psip_dn9 - (2.0 * locals.var_phibcv_dn9)) - locals.var_vdeff_dn9), ((locals.var_psip_dn10 - (2.0 * locals.var_phibcv_dn10)) - locals.var_vdeff_dn10), ((locals.var_psip_dn11 - (2.0 * locals.var_phibcv_dn11)) - locals.var_vdeff_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign55830_e91404;
        locals.var_t1_dn3 = assign55830_e91404_d_n3;
        locals.var_t1_dn4 = assign55830_e91404_d_n4;
        locals.var_t1_dn5 = assign55830_e91404_d_n5;
        locals.var_t1_dn6 = assign55830_e91404_d_n6;
        locals.var_t1_dn7 = assign55830_e91404_d_n7;
        locals.var_t1_dn8 = assign55830_e91404_d_n8;
        locals.var_t1_dn9 = assign55830_e91404_d_n9;
        locals.var_t1_dn10 = assign55830_e91404_d_n10;
        locals.var_t1_dn11 = assign55830_e91404_d_n11;

        let (assign55840_e91420, assign55840_e91420_d_n3, assign55840_e91420_d_n4, assign55840_e91420_d_n5, assign55840_e91420_d_n6, assign55840_e91420_d_n7, assign55840_e91420_d_n8, assign55840_e91420_d_n9, assign55840_e91420_d_n10, assign55840_e91420_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55840_e91412: f64 = (4.0 * locals.var_t0);
        let assign55840_e91414: f64 = (assign55840_e91412 * locals.var_sqrtpsip);
        let assign55840_e91416: f64 = (assign55840_e91414).max(1e-38);
        let assign55840_e91417: f64 = (assign55840_e91416).ln();
        let assign55840_e91418: f64 = (locals.var_t1 - assign55840_e91417);
        (assign55840_e91418, (locals.var_t1_dn3 - (if assign55840_e91414 >= 1e-38 { (((4.0 * locals.var_t0_dn3) * locals.var_sqrtpsip) + (assign55840_e91412 * locals.var_sqrtpsip_dn3)) } else { 0.0 } / assign55840_e91416)), (locals.var_t1_dn4 - (if assign55840_e91414 >= 1e-38 { (((4.0 * locals.var_t0_dn4) * locals.var_sqrtpsip) + (assign55840_e91412 * locals.var_sqrtpsip_dn4)) } else { 0.0 } / assign55840_e91416)), (locals.var_t1_dn5 - (if assign55840_e91414 >= 1e-38 { (((4.0 * locals.var_t0_dn5) * locals.var_sqrtpsip) + (assign55840_e91412 * locals.var_sqrtpsip_dn5)) } else { 0.0 } / assign55840_e91416)), (locals.var_t1_dn6 - (if assign55840_e91414 >= 1e-38 { (((4.0 * locals.var_t0_dn6) * locals.var_sqrtpsip) + (assign55840_e91412 * locals.var_sqrtpsip_dn6)) } else { 0.0 } / assign55840_e91416)), (locals.var_t1_dn7 - (if assign55840_e91414 >= 1e-38 { (((4.0 * locals.var_t0_dn7) * locals.var_sqrtpsip) + (assign55840_e91412 * locals.var_sqrtpsip_dn7)) } else { 0.0 } / assign55840_e91416)), (locals.var_t1_dn8 - (if assign55840_e91414 >= 1e-38 { (((4.0 * locals.var_t0_dn8) * locals.var_sqrtpsip) + (assign55840_e91412 * locals.var_sqrtpsip_dn8)) } else { 0.0 } / assign55840_e91416)), (locals.var_t1_dn9 - (if assign55840_e91414 >= 1e-38 { (((4.0 * locals.var_t0_dn9) * locals.var_sqrtpsip) + (assign55840_e91412 * locals.var_sqrtpsip_dn9)) } else { 0.0 } / assign55840_e91416)), (locals.var_t1_dn10 - (if assign55840_e91414 >= 1e-38 { (((4.0 * locals.var_t0_dn10) * locals.var_sqrtpsip) + (assign55840_e91412 * locals.var_sqrtpsip_dn10)) } else { 0.0 } / assign55840_e91416)), (locals.var_t1_dn11 - (if assign55840_e91414 >= 1e-38 { (((4.0 * locals.var_t0_dn11) * locals.var_sqrtpsip) + (assign55840_e91412 * locals.var_sqrtpsip_dn11)) } else { 0.0 } / assign55840_e91416)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign55840_e91420;
        locals.var_t2_dn3 = assign55840_e91420_d_n3;
        locals.var_t2_dn4 = assign55840_e91420_d_n4;
        locals.var_t2_dn5 = assign55840_e91420_d_n5;
        locals.var_t2_dn6 = assign55840_e91420_d_n6;
        locals.var_t2_dn7 = assign55840_e91420_d_n7;
        locals.var_t2_dn8 = assign55840_e91420_d_n8;
        locals.var_t2_dn9 = assign55840_e91420_d_n9;
        locals.var_t2_dn10 = assign55840_e91420_d_n10;
        locals.var_t2_dn11 = assign55840_e91420_d_n11;

        let (assign55850_e91440, assign55850_e91440_d_n3, assign55850_e91440_d_n4, assign55850_e91440_d_n5, assign55850_e91440_d_n6, assign55850_e91440_d_n7, assign55850_e91440_d_n8, assign55850_e91440_d_n9, assign55850_e91440_d_n10, assign55850_e91440_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign55850_e91428: f64 = (locals.var_t2 - 0.201491);
        let assign55850_e91432: f64 = (locals.var_t2 + 0.402982);
        let assign55850_e91433: f64 = (locals.var_t2 * assign55850_e91432);
        let assign55850_e91435: f64 = (assign55850_e91433 + 2.446562);
        let assign55850_e91436: f64 = (assign55850_e91435).sqrt();
        let assign55850_e91437: f64 = (assign55850_e91428 - assign55850_e91436);
        let assign55850_e91438: f64 = (0.5 * assign55850_e91437);
        (assign55850_e91438, (0.5 * (locals.var_t2_dn3 - (((locals.var_t2_dn3 * assign55850_e91432) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign55850_e91436)))), (0.5 * (locals.var_t2_dn4 - (((locals.var_t2_dn4 * assign55850_e91432) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign55850_e91436)))), (0.5 * (locals.var_t2_dn5 - (((locals.var_t2_dn5 * assign55850_e91432) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign55850_e91436)))), (0.5 * (locals.var_t2_dn6 - (((locals.var_t2_dn6 * assign55850_e91432) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign55850_e91436)))), (0.5 * (locals.var_t2_dn7 - (((locals.var_t2_dn7 * assign55850_e91432) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign55850_e91436)))), (0.5 * (locals.var_t2_dn8 - (((locals.var_t2_dn8 * assign55850_e91432) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign55850_e91436)))), (0.5 * (locals.var_t2_dn9 - (((locals.var_t2_dn9 * assign55850_e91432) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign55850_e91436)))), (0.5 * (locals.var_t2_dn10 - (((locals.var_t2_dn10 * assign55850_e91432) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign55850_e91436)))), (0.5 * (locals.var_t2_dn11 - (((locals.var_t2_dn11 * assign55850_e91432) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign55850_e91436)))),)
    } else {
        (locals.var_t8, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11,)
    }
};
        locals.var_t8 = assign55850_e91440;
        locals.var_t8_dn3 = assign55850_e91440_d_n3;
        locals.var_t8_dn4 = assign55850_e91440_d_n4;
        locals.var_t8_dn5 = assign55850_e91440_d_n5;
        locals.var_t8_dn6 = assign55850_e91440_d_n6;
        locals.var_t8_dn7 = assign55850_e91440_d_n7;
        locals.var_t8_dn8 = assign55850_e91440_d_n8;
        locals.var_t8_dn9 = assign55850_e91440_d_n9;
        locals.var_t8_dn10 = assign55850_e91440_d_n10;
        locals.var_t8_dn11 = assign55850_e91440_d_n11;

        let (assign55860_e91447, assign55860_e91447_d_n3, assign55860_e91447_d_n4, assign55860_e91447_d_n5, assign55860_e91447_d_n6, assign55860_e91447_d_n7, assign55860_e91447_d_n8, assign55860_e91447_d_n9, assign55860_e91447_d_n10, assign55860_e91447_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        (locals.var_sqrtpsip, locals.var_sqrtpsip_dn3, locals.var_sqrtpsip_dn4, locals.var_sqrtpsip_dn5, locals.var_sqrtpsip_dn6, locals.var_sqrtpsip_dn7, locals.var_sqrtpsip_dn8, locals.var_sqrtpsip_dn9, locals.var_sqrtpsip_dn10, locals.var_sqrtpsip_dn11,)
    } else {
        (locals.var_sqrtpsisa, locals.var_sqrtpsisa_dn3, locals.var_sqrtpsisa_dn4, locals.var_sqrtpsisa_dn5, locals.var_sqrtpsisa_dn6, locals.var_sqrtpsisa_dn7, locals.var_sqrtpsisa_dn8, locals.var_sqrtpsisa_dn9, locals.var_sqrtpsisa_dn10, locals.var_sqrtpsisa_dn11,)
    }
};
        locals.var_sqrtpsisa = assign55860_e91447;
        locals.var_sqrtpsisa_dn3 = assign55860_e91447_d_n3;
        locals.var_sqrtpsisa_dn4 = assign55860_e91447_d_n4;
        locals.var_sqrtpsisa_dn5 = assign55860_e91447_d_n5;
        locals.var_sqrtpsisa_dn6 = assign55860_e91447_d_n6;
        locals.var_sqrtpsisa_dn7 = assign55860_e91447_d_n7;
        locals.var_sqrtpsisa_dn8 = assign55860_e91447_d_n8;
        locals.var_sqrtpsisa_dn9 = assign55860_e91447_d_n9;
        locals.var_sqrtpsisa_dn10 = assign55860_e91447_d_n10;
        locals.var_sqrtpsisa_dn11 = assign55860_e91447_d_n11;

        let assign55870_e91450: f64 = (-68.0);
        let assign55870_e91451: f64 = if locals.var_t8 <= assign55870_e91450 { 1.0 } else { 0.0 };
        locals.var_guard848 = assign55870_e91451;

        let (assign55880_e91461, assign55880_e91461_d_n3, assign55880_e91461_d_n4, assign55880_e91461_d_n5, assign55880_e91461_d_n6, assign55880_e91461_d_n7, assign55880_e91461_d_n8, assign55880_e91461_d_n9, assign55880_e91461_d_n10, assign55880_e91461_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard848 != 0.0)) {
        let assign55880_e91459: f64 = (-100.0);
        (assign55880_e91459, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign55880_e91461;
        locals.var_t4_dn3 = assign55880_e91461_d_n3;
        locals.var_t4_dn4 = assign55880_e91461_d_n4;
        locals.var_t4_dn5 = assign55880_e91461_d_n5;
        locals.var_t4_dn6 = assign55880_e91461_d_n6;
        locals.var_t4_dn7 = assign55880_e91461_d_n7;
        locals.var_t4_dn8 = assign55880_e91461_d_n8;
        locals.var_t4_dn9 = assign55880_e91461_d_n9;
        locals.var_t4_dn10 = assign55880_e91461_d_n10;
        locals.var_t4_dn11 = assign55880_e91461_d_n11;

        let (assign55890_e91470, assign55890_e91470_d_n3, assign55890_e91470_d_n4, assign55890_e91470_d_n5, assign55890_e91470_d_n6, assign55890_e91470_d_n7, assign55890_e91470_d_n8, assign55890_e91470_d_n9, assign55890_e91470_d_n10, assign55890_e91470_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard848 != 0.0)) {
        (20.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign55890_e91470;
        locals.var_t5_dn3 = assign55890_e91470_d_n3;
        locals.var_t5_dn4 = assign55890_e91470_d_n4;
        locals.var_t5_dn5 = assign55890_e91470_d_n5;
        locals.var_t5_dn6 = assign55890_e91470_d_n6;
        locals.var_t5_dn7 = assign55890_e91470_d_n7;
        locals.var_t5_dn8 = assign55890_e91470_d_n8;
        locals.var_t5_dn9 = assign55890_e91470_d_n9;
        locals.var_t5_dn10 = assign55890_e91470_d_n10;
        locals.var_t5_dn11 = assign55890_e91470_d_n11;

        let assign55900_e91475: f64 = (0.5 * locals.var_t5);
        let assign55900_e91476: f64 = (locals.var_t4 - assign55900_e91475);
        let assign55900_e91477: f64 = if locals.var_t8 < assign55900_e91476 { 1.0 } else { 0.0 };
        locals.var_guard849 = assign55900_e91477;

        let (assign55910_e91489, assign55910_e91489_d_n3, assign55910_e91489_d_n4, assign55910_e91489_d_n5, assign55910_e91489_d_n6, assign55910_e91489_d_n7, assign55910_e91489_d_n8, assign55910_e91489_d_n9, assign55910_e91489_d_n10, assign55910_e91489_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard848 != 0.0)) && (locals.var_guard849 != 0.0)) {
        let assign55910_e91487: f64 = { let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign55910_e91487, ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn3), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn4), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn5), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn6), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn7), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn8), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn9), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn10), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign55910_e91489;
        locals.var_t3_dn3 = assign55910_e91489_d_n3;
        locals.var_t3_dn4 = assign55910_e91489_d_n4;
        locals.var_t3_dn5 = assign55910_e91489_d_n5;
        locals.var_t3_dn6 = assign55910_e91489_d_n6;
        locals.var_t3_dn7 = assign55910_e91489_d_n7;
        locals.var_t3_dn8 = assign55910_e91489_d_n8;
        locals.var_t3_dn9 = assign55910_e91489_d_n9;
        locals.var_t3_dn10 = assign55910_e91489_d_n10;
        locals.var_t3_dn11 = assign55910_e91489_d_n11;

        let assign55920_e91494: f64 = (0.5 * locals.var_t5);
        let assign55920_e91495: f64 = (locals.var_t4 + assign55920_e91494);
        let assign55920_e91496: f64 = if locals.var_t8 > assign55920_e91495 { 1.0 } else { 0.0 };
        locals.var_guard850 = assign55920_e91496;

        let (assign55930_e91511, assign55930_e91511_d_n3, assign55930_e91511_d_n4, assign55930_e91511_d_n5, assign55930_e91511_d_n6, assign55930_e91511_d_n7, assign55930_e91511_d_n8, assign55930_e91511_d_n9, assign55930_e91511_d_n10, assign55930_e91511_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard848 != 0.0)) && (locals.var_guard849 == 0.0)) && (locals.var_guard850 != 0.0)) {
        let assign55930_e91509: f64 = { let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign55930_e91509, ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn3), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn4), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn5), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn6), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn7), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn8), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn9), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn10), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign55930_e91511;
        locals.var_t3_dn3 = assign55930_e91511_d_n3;
        locals.var_t3_dn4 = assign55930_e91511_d_n4;
        locals.var_t3_dn5 = assign55930_e91511_d_n5;
        locals.var_t3_dn6 = assign55930_e91511_d_n6;
        locals.var_t3_dn7 = assign55930_e91511_d_n7;
        locals.var_t3_dn8 = assign55930_e91511_d_n8;
        locals.var_t3_dn9 = assign55930_e91511_d_n9;
        locals.var_t3_dn10 = assign55930_e91511_d_n10;
        locals.var_t3_dn11 = assign55930_e91511_d_n11;

        let (assign55940_e91530, assign55940_e91530_d_n3, assign55940_e91530_d_n4, assign55940_e91530_d_n5, assign55940_e91530_d_n6, assign55940_e91530_d_n7, assign55940_e91530_d_n8, assign55940_e91530_d_n9, assign55940_e91530_d_n10, assign55940_e91530_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard848 != 0.0)) && (locals.var_guard849 == 0.0)) && (locals.var_guard850 == 0.0)) {
        let assign55940_e91526: f64 = (locals.var_t8 - locals.var_t4);
        let assign55940_e91528: f64 = (assign55940_e91526 / locals.var_t5);
        (assign55940_e91528, ((((locals.var_t8_dn3 - locals.var_t4_dn3) * locals.var_t5) - (assign55940_e91526 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn4 - locals.var_t4_dn4) * locals.var_t5) - (assign55940_e91526 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn5 - locals.var_t4_dn5) * locals.var_t5) - (assign55940_e91526 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn6 - locals.var_t4_dn6) * locals.var_t5) - (assign55940_e91526 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn7 - locals.var_t4_dn7) * locals.var_t5) - (assign55940_e91526 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn8 - locals.var_t4_dn8) * locals.var_t5) - (assign55940_e91526 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn9 - locals.var_t4_dn9) * locals.var_t5) - (assign55940_e91526 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn10 - locals.var_t4_dn10) * locals.var_t5) - (assign55940_e91526 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn11 - locals.var_t4_dn11) * locals.var_t5) - (assign55940_e91526 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign55940_e91530;
        locals.var_t2_dn3 = assign55940_e91530_d_n3;
        locals.var_t2_dn4 = assign55940_e91530_d_n4;
        locals.var_t2_dn5 = assign55940_e91530_d_n5;
        locals.var_t2_dn6 = assign55940_e91530_d_n6;
        locals.var_t2_dn7 = assign55940_e91530_d_n7;
        locals.var_t2_dn8 = assign55940_e91530_d_n8;
        locals.var_t2_dn9 = assign55940_e91530_d_n9;
        locals.var_t2_dn10 = assign55940_e91530_d_n10;
        locals.var_t2_dn11 = assign55940_e91530_d_n11;

        let (assign55950_e91547, assign55950_e91547_d_n3, assign55950_e91547_d_n4, assign55950_e91547_d_n5, assign55950_e91547_d_n6, assign55950_e91547_d_n7, assign55950_e91547_d_n8, assign55950_e91547_d_n9, assign55950_e91547_d_n10, assign55950_e91547_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard848 != 0.0)) && (locals.var_guard849 == 0.0)) && (locals.var_guard850 == 0.0)) {
        let assign55950_e91545: f64 = (locals.var_t2 * locals.var_t2);
        (assign55950_e91545, ((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)), ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)), ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)), ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)), ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)), ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)), ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)), ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)), ((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign55950_e91547;
        locals.var_t6_dn3 = assign55950_e91547_d_n3;
        locals.var_t6_dn4 = assign55950_e91547_d_n4;
        locals.var_t6_dn5 = assign55950_e91547_d_n5;
        locals.var_t6_dn6 = assign55950_e91547_d_n6;
        locals.var_t6_dn7 = assign55950_e91547_d_n7;
        locals.var_t6_dn8 = assign55950_e91547_d_n8;
        locals.var_t6_dn9 = assign55950_e91547_d_n9;
        locals.var_t6_dn10 = assign55950_e91547_d_n10;
        locals.var_t6_dn11 = assign55950_e91547_d_n11;

        let (assign55960_e91585, assign55960_e91585_d_n3, assign55960_e91585_d_n4, assign55960_e91585_d_n5, assign55960_e91585_d_n6, assign55960_e91585_d_n7, assign55960_e91585_d_n8, assign55960_e91585_d_n9, assign55960_e91585_d_n10, assign55960_e91585_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard848 != 0.0)) && (locals.var_guard849 == 0.0)) && (locals.var_guard850 == 0.0)) {
        let assign55960_e91564: f64 = (5.0 / 64.0);
        let assign55960_e91567: f64 = (0.5 * locals.var_t2);
        let assign55960_e91568: f64 = (assign55960_e91564 + assign55960_e91567);
        let assign55960_e91572: f64 = (15.0 / 16.0);
        let assign55960_e91576: f64 = (1.25 - locals.var_t6);
        let assign55960_e91577: f64 = (locals.var_t6 * assign55960_e91576);
        let assign55960_e91578: f64 = (assign55960_e91572 - assign55960_e91577);
        let assign55960_e91579: f64 = (locals.var_t6 * assign55960_e91578);
        let assign55960_e91580: f64 = (assign55960_e91568 + assign55960_e91579);
        let assign55960_e91581: f64 = (locals.var_t5 * assign55960_e91580);
        let assign55960_e91582: f64 = (locals.var_t4 + assign55960_e91581);
        let assign55960_e91583: f64 = { let limited_exp_arg = assign55960_e91582; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign55960_e91583, ({ let limited_exp_arg = assign55960_e91582; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn3 + ((locals.var_t5_dn3 * assign55960_e91580) + (locals.var_t5 * ((0.5 * locals.var_t2_dn3) + ((locals.var_t6_dn3 * assign55960_e91578) + (locals.var_t6 * (-((locals.var_t6_dn3 * assign55960_e91576) + (locals.var_t6 * (-locals.var_t6_dn3))))))))))), ({ let limited_exp_arg = assign55960_e91582; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn4 + ((locals.var_t5_dn4 * assign55960_e91580) + (locals.var_t5 * ((0.5 * locals.var_t2_dn4) + ((locals.var_t6_dn4 * assign55960_e91578) + (locals.var_t6 * (-((locals.var_t6_dn4 * assign55960_e91576) + (locals.var_t6 * (-locals.var_t6_dn4))))))))))), ({ let limited_exp_arg = assign55960_e91582; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn5 + ((locals.var_t5_dn5 * assign55960_e91580) + (locals.var_t5 * ((0.5 * locals.var_t2_dn5) + ((locals.var_t6_dn5 * assign55960_e91578) + (locals.var_t6 * (-((locals.var_t6_dn5 * assign55960_e91576) + (locals.var_t6 * (-locals.var_t6_dn5))))))))))), ({ let limited_exp_arg = assign55960_e91582; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn6 + ((locals.var_t5_dn6 * assign55960_e91580) + (locals.var_t5 * ((0.5 * locals.var_t2_dn6) + ((locals.var_t6_dn6 * assign55960_e91578) + (locals.var_t6 * (-((locals.var_t6_dn6 * assign55960_e91576) + (locals.var_t6 * (-locals.var_t6_dn6))))))))))), ({ let limited_exp_arg = assign55960_e91582; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn7 + ((locals.var_t5_dn7 * assign55960_e91580) + (locals.var_t5 * ((0.5 * locals.var_t2_dn7) + ((locals.var_t6_dn7 * assign55960_e91578) + (locals.var_t6 * (-((locals.var_t6_dn7 * assign55960_e91576) + (locals.var_t6 * (-locals.var_t6_dn7))))))))))), ({ let limited_exp_arg = assign55960_e91582; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn8 + ((locals.var_t5_dn8 * assign55960_e91580) + (locals.var_t5 * ((0.5 * locals.var_t2_dn8) + ((locals.var_t6_dn8 * assign55960_e91578) + (locals.var_t6 * (-((locals.var_t6_dn8 * assign55960_e91576) + (locals.var_t6 * (-locals.var_t6_dn8))))))))))), ({ let limited_exp_arg = assign55960_e91582; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn9 + ((locals.var_t5_dn9 * assign55960_e91580) + (locals.var_t5 * ((0.5 * locals.var_t2_dn9) + ((locals.var_t6_dn9 * assign55960_e91578) + (locals.var_t6 * (-((locals.var_t6_dn9 * assign55960_e91576) + (locals.var_t6 * (-locals.var_t6_dn9))))))))))), ({ let limited_exp_arg = assign55960_e91582; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn10 + ((locals.var_t5_dn10 * assign55960_e91580) + (locals.var_t5 * ((0.5 * locals.var_t2_dn10) + ((locals.var_t6_dn10 * assign55960_e91578) + (locals.var_t6 * (-((locals.var_t6_dn10 * assign55960_e91576) + (locals.var_t6 * (-locals.var_t6_dn10))))))))))), ({ let limited_exp_arg = assign55960_e91582; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn11 + ((locals.var_t5_dn11 * assign55960_e91580) + (locals.var_t5 * ((0.5 * locals.var_t2_dn11) + ((locals.var_t6_dn11 * assign55960_e91578) + (locals.var_t6 * (-((locals.var_t6_dn11 * assign55960_e91576) + (locals.var_t6 * (-locals.var_t6_dn11))))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign55960_e91585;
        locals.var_t3_dn3 = assign55960_e91585_d_n3;
        locals.var_t3_dn4 = assign55960_e91585_d_n4;
        locals.var_t3_dn5 = assign55960_e91585_d_n5;
        locals.var_t3_dn6 = assign55960_e91585_d_n6;
        locals.var_t3_dn7 = assign55960_e91585_d_n7;
        locals.var_t3_dn8 = assign55960_e91585_d_n8;
        locals.var_t3_dn9 = assign55960_e91585_d_n9;
        locals.var_t3_dn10 = assign55960_e91585_d_n10;
        locals.var_t3_dn11 = assign55960_e91585_d_n11;

        let (assign55970_e91617, assign55970_e91617_d_n3, assign55970_e91617_d_n4, assign55970_e91617_d_n5, assign55970_e91617_d_n6, assign55970_e91617_d_n7, assign55970_e91617_d_n8, assign55970_e91617_d_n9, assign55970_e91617_d_n10, assign55970_e91617_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard848 != 0.0)) {
        let assign55970_e91595: f64 = (1.0 + locals.var_t1);
        let assign55970_e91597: f64 = (assign55970_e91595 - locals.var_t8);
        let assign55970_e91600: f64 = (2.0 * locals.var_t0);
        let assign55970_e91603: f64 = (locals.var_t3 * 2.0);
        let assign55970_e91605: f64 = (assign55970_e91603 * locals.var_t0);
        let assign55970_e91608: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign55970_e91609: f64 = (assign55970_e91605 + assign55970_e91608);
        let assign55970_e91610: f64 = (assign55970_e91600 * assign55970_e91609);
        let assign55970_e91612: f64 = (assign55970_e91610).max(1e-38);
        let assign55970_e91613: f64 = (assign55970_e91612).ln();
        let assign55970_e91614: f64 = (assign55970_e91597 - assign55970_e91613);
        let assign55970_e91615: f64 = (locals.var_t3 * assign55970_e91614);
        (assign55970_e91615, ((locals.var_t3_dn3 * assign55970_e91614) + (locals.var_t3 * ((locals.var_t1_dn3 - locals.var_t8_dn3) - (if assign55970_e91610 >= 1e-38 { (((2.0 * locals.var_t0_dn3) * assign55970_e91609) + (assign55970_e91600 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign55970_e91603 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign55970_e91612)))), ((locals.var_t3_dn4 * assign55970_e91614) + (locals.var_t3 * ((locals.var_t1_dn4 - locals.var_t8_dn4) - (if assign55970_e91610 >= 1e-38 { (((2.0 * locals.var_t0_dn4) * assign55970_e91609) + (assign55970_e91600 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign55970_e91603 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign55970_e91612)))), ((locals.var_t3_dn5 * assign55970_e91614) + (locals.var_t3 * ((locals.var_t1_dn5 - locals.var_t8_dn5) - (if assign55970_e91610 >= 1e-38 { (((2.0 * locals.var_t0_dn5) * assign55970_e91609) + (assign55970_e91600 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign55970_e91603 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign55970_e91612)))), ((locals.var_t3_dn6 * assign55970_e91614) + (locals.var_t3 * ((locals.var_t1_dn6 - locals.var_t8_dn6) - (if assign55970_e91610 >= 1e-38 { (((2.0 * locals.var_t0_dn6) * assign55970_e91609) + (assign55970_e91600 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign55970_e91603 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign55970_e91612)))), ((locals.var_t3_dn7 * assign55970_e91614) + (locals.var_t3 * ((locals.var_t1_dn7 - locals.var_t8_dn7) - (if assign55970_e91610 >= 1e-38 { (((2.0 * locals.var_t0_dn7) * assign55970_e91609) + (assign55970_e91600 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign55970_e91603 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign55970_e91612)))), ((locals.var_t3_dn8 * assign55970_e91614) + (locals.var_t3 * ((locals.var_t1_dn8 - locals.var_t8_dn8) - (if assign55970_e91610 >= 1e-38 { (((2.0 * locals.var_t0_dn8) * assign55970_e91609) + (assign55970_e91600 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign55970_e91603 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign55970_e91612)))), ((locals.var_t3_dn9 * assign55970_e91614) + (locals.var_t3 * ((locals.var_t1_dn9 - locals.var_t8_dn9) - (if assign55970_e91610 >= 1e-38 { (((2.0 * locals.var_t0_dn9) * assign55970_e91609) + (assign55970_e91600 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign55970_e91603 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign55970_e91612)))), ((locals.var_t3_dn10 * assign55970_e91614) + (locals.var_t3 * ((locals.var_t1_dn10 - locals.var_t8_dn10) - (if assign55970_e91610 >= 1e-38 { (((2.0 * locals.var_t0_dn10) * assign55970_e91609) + (assign55970_e91600 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign55970_e91603 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign55970_e91612)))), ((locals.var_t3_dn11 * assign55970_e91614) + (locals.var_t3 * ((locals.var_t1_dn11 - locals.var_t8_dn11) - (if assign55970_e91610 >= 1e-38 { (((2.0 * locals.var_t0_dn11) * assign55970_e91609) + (assign55970_e91600 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign55970_e91603 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign55970_e91612)))),)
    } else {
        (locals.var_qdeff, locals.var_qdeff_dn3, locals.var_qdeff_dn4, locals.var_qdeff_dn5, locals.var_qdeff_dn6, locals.var_qdeff_dn7, locals.var_qdeff_dn8, locals.var_qdeff_dn9, locals.var_qdeff_dn10, locals.var_qdeff_dn11,)
    }
};
        locals.var_qdeff = assign55970_e91617;
        locals.var_qdeff_dn3 = assign55970_e91617_d_n3;
        locals.var_qdeff_dn4 = assign55970_e91617_d_n4;
        locals.var_qdeff_dn5 = assign55970_e91617_d_n5;
        locals.var_qdeff_dn6 = assign55970_e91617_d_n6;
        locals.var_qdeff_dn7 = assign55970_e91617_d_n7;
        locals.var_qdeff_dn8 = assign55970_e91617_d_n8;
        locals.var_qdeff_dn9 = assign55970_e91617_d_n9;
        locals.var_qdeff_dn10 = assign55970_e91617_d_n10;
        locals.var_qdeff_dn11 = assign55970_e91617_d_n11;

        let (assign55980_e91628, assign55980_e91628_d_n3, assign55980_e91628_d_n4, assign55980_e91628_d_n5, assign55980_e91628_d_n6, assign55980_e91628_d_n7, assign55980_e91628_d_n8, assign55980_e91628_d_n9, assign55980_e91628_d_n10, assign55980_e91628_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard848 == 0.0)) {
        let assign55980_e91626: f64 = { let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign55980_e91626, ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn3), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn4), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn5), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn6), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn7), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn8), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn9), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn10), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign55980_e91628;
        locals.var_t3_dn3 = assign55980_e91628_d_n3;
        locals.var_t3_dn4 = assign55980_e91628_d_n4;
        locals.var_t3_dn5 = assign55980_e91628_d_n5;
        locals.var_t3_dn6 = assign55980_e91628_d_n6;
        locals.var_t3_dn7 = assign55980_e91628_d_n7;
        locals.var_t3_dn8 = assign55980_e91628_d_n8;
        locals.var_t3_dn9 = assign55980_e91628_d_n9;
        locals.var_t3_dn10 = assign55980_e91628_d_n10;
        locals.var_t3_dn11 = assign55980_e91628_d_n11;

        let (assign55990_e91640, assign55990_e91640_d_n3, assign55990_e91640_d_n4, assign55990_e91640_d_n5, assign55990_e91640_d_n6, assign55990_e91640_d_n7, assign55990_e91640_d_n8, assign55990_e91640_d_n9, assign55990_e91640_d_n10, assign55990_e91640_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard848 == 0.0)) {
        let assign55990_e91638: f64 = (1.0 / locals.var_sqrtpsisa);
        (assign55990_e91638, (-(locals.var_sqrtpsisa_dn3 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn4 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn5 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn6 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn7 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn8 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn9 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn10 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn11 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))),)
    } else {
        (locals.var_sqrtpsisainv, locals.var_sqrtpsisainv_dn3, locals.var_sqrtpsisainv_dn4, locals.var_sqrtpsisainv_dn5, locals.var_sqrtpsisainv_dn6, locals.var_sqrtpsisainv_dn7, locals.var_sqrtpsisainv_dn8, locals.var_sqrtpsisainv_dn9, locals.var_sqrtpsisainv_dn10, locals.var_sqrtpsisainv_dn11,)
    }
};
        locals.var_sqrtpsisainv = assign55990_e91640;
        locals.var_sqrtpsisainv_dn3 = assign55990_e91640_d_n3;
        locals.var_sqrtpsisainv_dn4 = assign55990_e91640_d_n4;
        locals.var_sqrtpsisainv_dn5 = assign55990_e91640_d_n5;
        locals.var_sqrtpsisainv_dn6 = assign55990_e91640_d_n6;
        locals.var_sqrtpsisainv_dn7 = assign55990_e91640_d_n7;
        locals.var_sqrtpsisainv_dn8 = assign55990_e91640_d_n8;
        locals.var_sqrtpsisainv_dn9 = assign55990_e91640_d_n9;
        locals.var_sqrtpsisainv_dn10 = assign55990_e91640_d_n10;
        locals.var_sqrtpsisainv_dn11 = assign55990_e91640_d_n11;

        let (assign56000_e91673, assign56000_e91673_d_n3, assign56000_e91673_d_n4, assign56000_e91673_d_n5, assign56000_e91673_d_n6, assign56000_e91673_d_n7, assign56000_e91673_d_n8, assign56000_e91673_d_n9, assign56000_e91673_d_n10, assign56000_e91673_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard848 == 0.0)) {
        let assign56000_e91650: f64 = (2.0 * locals.var_t3);
        let assign56000_e91653: f64 = (locals.var_t3 * 2.0);
        let assign56000_e91655: f64 = (assign56000_e91653 * locals.var_t0);
        let assign56000_e91658: f64 = (locals.var_t3 * 2.0);
        let assign56000_e91660: f64 = (assign56000_e91658 * locals.var_t0);
        let assign56000_e91663: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign56000_e91664: f64 = (assign56000_e91660 + assign56000_e91663);
        let assign56000_e91665: f64 = (assign56000_e91655 * assign56000_e91664);
        let assign56000_e91667: f64 = (assign56000_e91665).max(1e-38);
        let assign56000_e91668: f64 = (assign56000_e91667).ln();
        let assign56000_e91669: f64 = (assign56000_e91650 + assign56000_e91668);
        let assign56000_e91671: f64 = (assign56000_e91669 - locals.var_t1);
        (assign56000_e91671, (((2.0 * locals.var_t3_dn3) + (if assign56000_e91665 >= 1e-38 { (((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign56000_e91653 * locals.var_t0_dn3)) * assign56000_e91664) + (assign56000_e91655 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign56000_e91658 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign56000_e91667)) - locals.var_t1_dn3), (((2.0 * locals.var_t3_dn4) + (if assign56000_e91665 >= 1e-38 { (((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign56000_e91653 * locals.var_t0_dn4)) * assign56000_e91664) + (assign56000_e91655 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign56000_e91658 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign56000_e91667)) - locals.var_t1_dn4), (((2.0 * locals.var_t3_dn5) + (if assign56000_e91665 >= 1e-38 { (((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign56000_e91653 * locals.var_t0_dn5)) * assign56000_e91664) + (assign56000_e91655 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign56000_e91658 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign56000_e91667)) - locals.var_t1_dn5), (((2.0 * locals.var_t3_dn6) + (if assign56000_e91665 >= 1e-38 { (((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign56000_e91653 * locals.var_t0_dn6)) * assign56000_e91664) + (assign56000_e91655 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign56000_e91658 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign56000_e91667)) - locals.var_t1_dn6), (((2.0 * locals.var_t3_dn7) + (if assign56000_e91665 >= 1e-38 { (((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign56000_e91653 * locals.var_t0_dn7)) * assign56000_e91664) + (assign56000_e91655 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign56000_e91658 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign56000_e91667)) - locals.var_t1_dn7), (((2.0 * locals.var_t3_dn8) + (if assign56000_e91665 >= 1e-38 { (((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign56000_e91653 * locals.var_t0_dn8)) * assign56000_e91664) + (assign56000_e91655 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign56000_e91658 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign56000_e91667)) - locals.var_t1_dn8), (((2.0 * locals.var_t3_dn9) + (if assign56000_e91665 >= 1e-38 { (((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign56000_e91653 * locals.var_t0_dn9)) * assign56000_e91664) + (assign56000_e91655 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign56000_e91658 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign56000_e91667)) - locals.var_t1_dn9), (((2.0 * locals.var_t3_dn10) + (if assign56000_e91665 >= 1e-38 { (((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign56000_e91653 * locals.var_t0_dn10)) * assign56000_e91664) + (assign56000_e91655 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign56000_e91658 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign56000_e91667)) - locals.var_t1_dn10), (((2.0 * locals.var_t3_dn11) + (if assign56000_e91665 >= 1e-38 { (((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign56000_e91653 * locals.var_t0_dn11)) * assign56000_e91664) + (assign56000_e91655 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign56000_e91658 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign56000_e91667)) - locals.var_t1_dn11),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign56000_e91673;
        locals.var_t4_dn3 = assign56000_e91673_d_n3;
        locals.var_t4_dn4 = assign56000_e91673_d_n4;
        locals.var_t4_dn5 = assign56000_e91673_d_n5;
        locals.var_t4_dn6 = assign56000_e91673_d_n6;
        locals.var_t4_dn7 = assign56000_e91673_d_n7;
        locals.var_t4_dn8 = assign56000_e91673_d_n8;
        locals.var_t4_dn9 = assign56000_e91673_d_n9;
        locals.var_t4_dn10 = assign56000_e91673_d_n10;
        locals.var_t4_dn11 = assign56000_e91673_d_n11;

    }

    pub(super) fn stamp_transient_block_190(
        locals: &mut StampLocals,
    ) {
        let (assign56010_e91697, assign56010_e91697_d_n3, assign56010_e91697_d_n4, assign56010_e91697_d_n5, assign56010_e91697_d_n6, assign56010_e91697_d_n7, assign56010_e91697_d_n8, assign56010_e91697_d_n9, assign56010_e91697_d_n10, assign56010_e91697_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard848 == 0.0)) {
        let assign56010_e91684: f64 = (1.0 / locals.var_t3);
        let assign56010_e91685: f64 = (2.0 + assign56010_e91684);
        let assign56010_e91688: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign56010_e91691: f64 = (locals.var_t0 * locals.var_t3);
        let assign56010_e91693: f64 = (assign56010_e91691 + locals.var_sqrtpsisa);
        let assign56010_e91694: f64 = (assign56010_e91688 / assign56010_e91693);
        let assign56010_e91695: f64 = (assign56010_e91685 + assign56010_e91694);
        (assign56010_e91695, ((-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign56010_e91693) - (assign56010_e91688 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign56010_e91693 * assign56010_e91693))), ((-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign56010_e91693) - (assign56010_e91688 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign56010_e91693 * assign56010_e91693))), ((-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign56010_e91693) - (assign56010_e91688 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign56010_e91693 * assign56010_e91693))), ((-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign56010_e91693) - (assign56010_e91688 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign56010_e91693 * assign56010_e91693))), ((-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign56010_e91693) - (assign56010_e91688 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign56010_e91693 * assign56010_e91693))), ((-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign56010_e91693) - (assign56010_e91688 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign56010_e91693 * assign56010_e91693))), ((-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign56010_e91693) - (assign56010_e91688 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign56010_e91693 * assign56010_e91693))), ((-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign56010_e91693) - (assign56010_e91688 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign56010_e91693 * assign56010_e91693))), ((-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign56010_e91693) - (assign56010_e91688 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign56010_e91693 * assign56010_e91693))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign56010_e91697;
        locals.var_t5_dn3 = assign56010_e91697_d_n3;
        locals.var_t5_dn4 = assign56010_e91697_d_n4;
        locals.var_t5_dn5 = assign56010_e91697_d_n5;
        locals.var_t5_dn6 = assign56010_e91697_d_n6;
        locals.var_t5_dn7 = assign56010_e91697_d_n7;
        locals.var_t5_dn8 = assign56010_e91697_d_n8;
        locals.var_t5_dn9 = assign56010_e91697_d_n9;
        locals.var_t5_dn10 = assign56010_e91697_d_n10;
        locals.var_t5_dn11 = assign56010_e91697_d_n11;

        let (assign56020_e91711, assign56020_e91711_d_n3, assign56020_e91711_d_n4, assign56020_e91711_d_n5, assign56020_e91711_d_n6, assign56020_e91711_d_n7, assign56020_e91711_d_n8, assign56020_e91711_d_n9, assign56020_e91711_d_n10, assign56020_e91711_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard848 == 0.0)) {
        let assign56020_e91708: f64 = (locals.var_t4 / locals.var_t5);
        let assign56020_e91709: f64 = (locals.var_t3 - assign56020_e91708);
        (assign56020_e91709, (locals.var_t3_dn3 - (((locals.var_t4_dn3 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn4 - (((locals.var_t4_dn4 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn5 - (((locals.var_t4_dn5 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn6 - (((locals.var_t4_dn6 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn7 - (((locals.var_t4_dn7 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn8 - (((locals.var_t4_dn8 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn9 - (((locals.var_t4_dn9 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn10 - (((locals.var_t4_dn10 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn11 - (((locals.var_t4_dn11 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign56020_e91711;
        locals.var_t3_dn3 = assign56020_e91711_d_n3;
        locals.var_t3_dn4 = assign56020_e91711_d_n4;
        locals.var_t3_dn5 = assign56020_e91711_d_n5;
        locals.var_t3_dn6 = assign56020_e91711_d_n6;
        locals.var_t3_dn7 = assign56020_e91711_d_n7;
        locals.var_t3_dn8 = assign56020_e91711_d_n8;
        locals.var_t3_dn9 = assign56020_e91711_d_n9;
        locals.var_t3_dn10 = assign56020_e91711_d_n10;
        locals.var_t3_dn11 = assign56020_e91711_d_n11;

        let (assign56030_e91744, assign56030_e91744_d_n3, assign56030_e91744_d_n4, assign56030_e91744_d_n5, assign56030_e91744_d_n6, assign56030_e91744_d_n7, assign56030_e91744_d_n8, assign56030_e91744_d_n9, assign56030_e91744_d_n10, assign56030_e91744_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard848 == 0.0)) {
        let assign56030_e91721: f64 = (2.0 * locals.var_t3);
        let assign56030_e91724: f64 = (locals.var_t3 * 2.0);
        let assign56030_e91726: f64 = (assign56030_e91724 * locals.var_t0);
        let assign56030_e91729: f64 = (locals.var_t3 * 2.0);
        let assign56030_e91731: f64 = (assign56030_e91729 * locals.var_t0);
        let assign56030_e91734: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign56030_e91735: f64 = (assign56030_e91731 + assign56030_e91734);
        let assign56030_e91736: f64 = (assign56030_e91726 * assign56030_e91735);
        let assign56030_e91738: f64 = (assign56030_e91736).max(1e-38);
        let assign56030_e91739: f64 = (assign56030_e91738).ln();
        let assign56030_e91740: f64 = (assign56030_e91721 + assign56030_e91739);
        let assign56030_e91742: f64 = (assign56030_e91740 - locals.var_t1);
        (assign56030_e91742, (((2.0 * locals.var_t3_dn3) + (if assign56030_e91736 >= 1e-38 { (((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign56030_e91724 * locals.var_t0_dn3)) * assign56030_e91735) + (assign56030_e91726 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign56030_e91729 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign56030_e91738)) - locals.var_t1_dn3), (((2.0 * locals.var_t3_dn4) + (if assign56030_e91736 >= 1e-38 { (((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign56030_e91724 * locals.var_t0_dn4)) * assign56030_e91735) + (assign56030_e91726 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign56030_e91729 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign56030_e91738)) - locals.var_t1_dn4), (((2.0 * locals.var_t3_dn5) + (if assign56030_e91736 >= 1e-38 { (((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign56030_e91724 * locals.var_t0_dn5)) * assign56030_e91735) + (assign56030_e91726 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign56030_e91729 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign56030_e91738)) - locals.var_t1_dn5), (((2.0 * locals.var_t3_dn6) + (if assign56030_e91736 >= 1e-38 { (((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign56030_e91724 * locals.var_t0_dn6)) * assign56030_e91735) + (assign56030_e91726 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign56030_e91729 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign56030_e91738)) - locals.var_t1_dn6), (((2.0 * locals.var_t3_dn7) + (if assign56030_e91736 >= 1e-38 { (((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign56030_e91724 * locals.var_t0_dn7)) * assign56030_e91735) + (assign56030_e91726 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign56030_e91729 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign56030_e91738)) - locals.var_t1_dn7), (((2.0 * locals.var_t3_dn8) + (if assign56030_e91736 >= 1e-38 { (((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign56030_e91724 * locals.var_t0_dn8)) * assign56030_e91735) + (assign56030_e91726 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign56030_e91729 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign56030_e91738)) - locals.var_t1_dn8), (((2.0 * locals.var_t3_dn9) + (if assign56030_e91736 >= 1e-38 { (((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign56030_e91724 * locals.var_t0_dn9)) * assign56030_e91735) + (assign56030_e91726 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign56030_e91729 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign56030_e91738)) - locals.var_t1_dn9), (((2.0 * locals.var_t3_dn10) + (if assign56030_e91736 >= 1e-38 { (((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign56030_e91724 * locals.var_t0_dn10)) * assign56030_e91735) + (assign56030_e91726 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign56030_e91729 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign56030_e91738)) - locals.var_t1_dn10), (((2.0 * locals.var_t3_dn11) + (if assign56030_e91736 >= 1e-38 { (((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign56030_e91724 * locals.var_t0_dn11)) * assign56030_e91735) + (assign56030_e91726 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign56030_e91729 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign56030_e91738)) - locals.var_t1_dn11),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign56030_e91744;
        locals.var_t4_dn3 = assign56030_e91744_d_n3;
        locals.var_t4_dn4 = assign56030_e91744_d_n4;
        locals.var_t4_dn5 = assign56030_e91744_d_n5;
        locals.var_t4_dn6 = assign56030_e91744_d_n6;
        locals.var_t4_dn7 = assign56030_e91744_d_n7;
        locals.var_t4_dn8 = assign56030_e91744_d_n8;
        locals.var_t4_dn9 = assign56030_e91744_d_n9;
        locals.var_t4_dn10 = assign56030_e91744_d_n10;
        locals.var_t4_dn11 = assign56030_e91744_d_n11;

        let (assign56040_e91768, assign56040_e91768_d_n3, assign56040_e91768_d_n4, assign56040_e91768_d_n5, assign56040_e91768_d_n6, assign56040_e91768_d_n7, assign56040_e91768_d_n8, assign56040_e91768_d_n9, assign56040_e91768_d_n10, assign56040_e91768_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard848 == 0.0)) {
        let assign56040_e91755: f64 = (1.0 / locals.var_t3);
        let assign56040_e91756: f64 = (2.0 + assign56040_e91755);
        let assign56040_e91759: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign56040_e91762: f64 = (locals.var_t0 * locals.var_t3);
        let assign56040_e91764: f64 = (assign56040_e91762 + locals.var_sqrtpsisa);
        let assign56040_e91765: f64 = (assign56040_e91759 / assign56040_e91764);
        let assign56040_e91766: f64 = (assign56040_e91756 + assign56040_e91765);
        (assign56040_e91766, ((-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign56040_e91764) - (assign56040_e91759 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign56040_e91764 * assign56040_e91764))), ((-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign56040_e91764) - (assign56040_e91759 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign56040_e91764 * assign56040_e91764))), ((-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign56040_e91764) - (assign56040_e91759 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign56040_e91764 * assign56040_e91764))), ((-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign56040_e91764) - (assign56040_e91759 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign56040_e91764 * assign56040_e91764))), ((-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign56040_e91764) - (assign56040_e91759 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign56040_e91764 * assign56040_e91764))), ((-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign56040_e91764) - (assign56040_e91759 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign56040_e91764 * assign56040_e91764))), ((-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign56040_e91764) - (assign56040_e91759 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign56040_e91764 * assign56040_e91764))), ((-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign56040_e91764) - (assign56040_e91759 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign56040_e91764 * assign56040_e91764))), ((-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign56040_e91764) - (assign56040_e91759 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign56040_e91764 * assign56040_e91764))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign56040_e91768;
        locals.var_t5_dn3 = assign56040_e91768_d_n3;
        locals.var_t5_dn4 = assign56040_e91768_d_n4;
        locals.var_t5_dn5 = assign56040_e91768_d_n5;
        locals.var_t5_dn6 = assign56040_e91768_d_n6;
        locals.var_t5_dn7 = assign56040_e91768_d_n7;
        locals.var_t5_dn8 = assign56040_e91768_d_n8;
        locals.var_t5_dn9 = assign56040_e91768_d_n9;
        locals.var_t5_dn10 = assign56040_e91768_d_n10;
        locals.var_t5_dn11 = assign56040_e91768_d_n11;

        let (assign56050_e91796, assign56050_e91796_d_n3, assign56050_e91796_d_n4, assign56050_e91796_d_n5, assign56050_e91796_d_n6, assign56050_e91796_d_n7, assign56050_e91796_d_n8, assign56050_e91796_d_n9, assign56050_e91796_d_n10, assign56050_e91796_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard848 == 0.0)) {
        let assign56050_e91778: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign56050_e91781: f64 = (locals.var_t0 * locals.var_t3);
        let assign56050_e91783: f64 = (assign56050_e91781 + locals.var_sqrtpsisa);
        let assign56050_e91784: f64 = (assign56050_e91778 / assign56050_e91783);
        let assign56050_e91787: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign56050_e91790: f64 = (locals.var_t0 * locals.var_t3);
        let assign56050_e91792: f64 = (assign56050_e91790 + locals.var_sqrtpsisa);
        let assign56050_e91793: f64 = (assign56050_e91787 / assign56050_e91792);
        let assign56050_e91794: f64 = (assign56050_e91784 * assign56050_e91793);
        (assign56050_e91794, ((((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign56050_e91783) - (assign56050_e91778 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign56050_e91783 * assign56050_e91783)) * assign56050_e91793) + (assign56050_e91784 * ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign56050_e91792) - (assign56050_e91787 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign56050_e91792 * assign56050_e91792)))), ((((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign56050_e91783) - (assign56050_e91778 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign56050_e91783 * assign56050_e91783)) * assign56050_e91793) + (assign56050_e91784 * ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign56050_e91792) - (assign56050_e91787 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign56050_e91792 * assign56050_e91792)))), ((((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign56050_e91783) - (assign56050_e91778 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign56050_e91783 * assign56050_e91783)) * assign56050_e91793) + (assign56050_e91784 * ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign56050_e91792) - (assign56050_e91787 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign56050_e91792 * assign56050_e91792)))), ((((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign56050_e91783) - (assign56050_e91778 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign56050_e91783 * assign56050_e91783)) * assign56050_e91793) + (assign56050_e91784 * ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign56050_e91792) - (assign56050_e91787 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign56050_e91792 * assign56050_e91792)))), ((((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign56050_e91783) - (assign56050_e91778 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign56050_e91783 * assign56050_e91783)) * assign56050_e91793) + (assign56050_e91784 * ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign56050_e91792) - (assign56050_e91787 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign56050_e91792 * assign56050_e91792)))), ((((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign56050_e91783) - (assign56050_e91778 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign56050_e91783 * assign56050_e91783)) * assign56050_e91793) + (assign56050_e91784 * ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign56050_e91792) - (assign56050_e91787 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign56050_e91792 * assign56050_e91792)))), ((((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign56050_e91783) - (assign56050_e91778 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign56050_e91783 * assign56050_e91783)) * assign56050_e91793) + (assign56050_e91784 * ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign56050_e91792) - (assign56050_e91787 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign56050_e91792 * assign56050_e91792)))), ((((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign56050_e91783) - (assign56050_e91778 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign56050_e91783 * assign56050_e91783)) * assign56050_e91793) + (assign56050_e91784 * ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign56050_e91792) - (assign56050_e91787 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign56050_e91792 * assign56050_e91792)))), ((((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign56050_e91783) - (assign56050_e91778 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign56050_e91783 * assign56050_e91783)) * assign56050_e91793) + (assign56050_e91784 * ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign56050_e91792) - (assign56050_e91787 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign56050_e91792 * assign56050_e91792)))),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign56050_e91796;
        locals.var_t6_dn3 = assign56050_e91796_d_n3;
        locals.var_t6_dn4 = assign56050_e91796_d_n4;
        locals.var_t6_dn5 = assign56050_e91796_d_n5;
        locals.var_t6_dn6 = assign56050_e91796_d_n6;
        locals.var_t6_dn7 = assign56050_e91796_d_n7;
        locals.var_t6_dn8 = assign56050_e91796_d_n8;
        locals.var_t6_dn9 = assign56050_e91796_d_n9;
        locals.var_t6_dn10 = assign56050_e91796_d_n10;
        locals.var_t6_dn11 = assign56050_e91796_d_n11;

        let (assign56060_e91829, assign56060_e91829_d_n3, assign56060_e91829_d_n4, assign56060_e91829_d_n5, assign56060_e91829_d_n6, assign56060_e91829_d_n7, assign56060_e91829_d_n8, assign56060_e91829_d_n9, assign56060_e91829_d_n10, assign56060_e91829_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard848 == 0.0)) {
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_t3;
        let assign56060_e91806: f64 = (1.0 * __rspice_inv_cse_0);
        let assign56060_e91809: f64 = (1.0 * __rspice_inv_cse_0);
        let assign56060_e91810: f64 = (assign56060_e91806 * assign56060_e91809);
        let assign56060_e91811: f64 = (-assign56060_e91810);
        let assign56060_e91815: f64 = (locals.var_sqrtpsisa * locals.var_sqrtpsisa);
        let assign56060_e91817: f64 = (assign56060_e91815 * locals.var_sqrtpsisa);
        let assign56060_e91820: f64 = (locals.var_t0 * locals.var_t3);
        let assign56060_e91822: f64 = (assign56060_e91820 + locals.var_sqrtpsisa);
        let assign56060_e91823: f64 = (assign56060_e91817 * assign56060_e91822);
        let assign56060_e91824: f64 = (1.0 / assign56060_e91823);
        let assign56060_e91825: f64 = (assign56060_e91811 - assign56060_e91824);
        let assign56060_e91827: f64 = (assign56060_e91825 - locals.var_t6);
        (assign56060_e91827, (((-(((-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3))) * assign56060_e91809) + (assign56060_e91806 * (-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn3 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn3)) * locals.var_sqrtpsisa) + (assign56060_e91815 * locals.var_sqrtpsisa_dn3)) * assign56060_e91822) + (assign56060_e91817 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign56060_e91823 * assign56060_e91823)))) - locals.var_t6_dn3), (((-(((-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))) * assign56060_e91809) + (assign56060_e91806 * (-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn4 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn4)) * locals.var_sqrtpsisa) + (assign56060_e91815 * locals.var_sqrtpsisa_dn4)) * assign56060_e91822) + (assign56060_e91817 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign56060_e91823 * assign56060_e91823)))) - locals.var_t6_dn4), (((-(((-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))) * assign56060_e91809) + (assign56060_e91806 * (-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn5 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn5)) * locals.var_sqrtpsisa) + (assign56060_e91815 * locals.var_sqrtpsisa_dn5)) * assign56060_e91822) + (assign56060_e91817 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign56060_e91823 * assign56060_e91823)))) - locals.var_t6_dn5), (((-(((-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))) * assign56060_e91809) + (assign56060_e91806 * (-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn6 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn6)) * locals.var_sqrtpsisa) + (assign56060_e91815 * locals.var_sqrtpsisa_dn6)) * assign56060_e91822) + (assign56060_e91817 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign56060_e91823 * assign56060_e91823)))) - locals.var_t6_dn6), (((-(((-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))) * assign56060_e91809) + (assign56060_e91806 * (-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn7 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn7)) * locals.var_sqrtpsisa) + (assign56060_e91815 * locals.var_sqrtpsisa_dn7)) * assign56060_e91822) + (assign56060_e91817 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign56060_e91823 * assign56060_e91823)))) - locals.var_t6_dn7), (((-(((-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))) * assign56060_e91809) + (assign56060_e91806 * (-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn8 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn8)) * locals.var_sqrtpsisa) + (assign56060_e91815 * locals.var_sqrtpsisa_dn8)) * assign56060_e91822) + (assign56060_e91817 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign56060_e91823 * assign56060_e91823)))) - locals.var_t6_dn8), (((-(((-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))) * assign56060_e91809) + (assign56060_e91806 * (-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn9 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn9)) * locals.var_sqrtpsisa) + (assign56060_e91815 * locals.var_sqrtpsisa_dn9)) * assign56060_e91822) + (assign56060_e91817 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign56060_e91823 * assign56060_e91823)))) - locals.var_t6_dn9), (((-(((-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))) * assign56060_e91809) + (assign56060_e91806 * (-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn10 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn10)) * locals.var_sqrtpsisa) + (assign56060_e91815 * locals.var_sqrtpsisa_dn10)) * assign56060_e91822) + (assign56060_e91817 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign56060_e91823 * assign56060_e91823)))) - locals.var_t6_dn10), (((-(((-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))) * assign56060_e91809) + (assign56060_e91806 * (-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn11 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn11)) * locals.var_sqrtpsisa) + (assign56060_e91815 * locals.var_sqrtpsisa_dn11)) * assign56060_e91822) + (assign56060_e91817 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign56060_e91823 * assign56060_e91823)))) - locals.var_t6_dn11),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign56060_e91829;
        locals.var_t7_dn3 = assign56060_e91829_d_n3;
        locals.var_t7_dn4 = assign56060_e91829_d_n4;
        locals.var_t7_dn5 = assign56060_e91829_d_n5;
        locals.var_t7_dn6 = assign56060_e91829_d_n6;
        locals.var_t7_dn7 = assign56060_e91829_d_n7;
        locals.var_t7_dn8 = assign56060_e91829_d_n8;
        locals.var_t7_dn9 = assign56060_e91829_d_n9;
        locals.var_t7_dn10 = assign56060_e91829_d_n10;
        locals.var_t7_dn11 = assign56060_e91829_d_n11;

        let (assign56070_e91855, assign56070_e91855_d_n3, assign56070_e91855_d_n4, assign56070_e91855_d_n5, assign56070_e91855_d_n6, assign56070_e91855_d_n7, assign56070_e91855_d_n8, assign56070_e91855_d_n9, assign56070_e91855_d_n10, assign56070_e91855_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard848 == 0.0)) {
        let assign56070_e91840: f64 = (locals.var_t4 / locals.var_t5);
        let assign56070_e91844: f64 = (locals.var_t4 * locals.var_t7);
        let assign56070_e91847: f64 = (2.0 * locals.var_t5);
        let assign56070_e91849: f64 = (assign56070_e91847 * locals.var_t5);
        let assign56070_e91850: f64 = (assign56070_e91844 / assign56070_e91849);
        let assign56070_e91851: f64 = (1.0 + assign56070_e91850);
        let assign56070_e91852: f64 = (assign56070_e91840 * assign56070_e91851);
        let assign56070_e91853: f64 = (locals.var_t3 - assign56070_e91852);
        (assign56070_e91853, (locals.var_t3_dn3 - (((((locals.var_t4_dn3 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5)) * assign56070_e91851) + (assign56070_e91840 * (((((locals.var_t4_dn3 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn3)) * assign56070_e91849) - (assign56070_e91844 * (((2.0 * locals.var_t5_dn3) * locals.var_t5) + (assign56070_e91847 * locals.var_t5_dn3)))) / (assign56070_e91849 * assign56070_e91849))))), (locals.var_t3_dn4 - (((((locals.var_t4_dn4 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)) * assign56070_e91851) + (assign56070_e91840 * (((((locals.var_t4_dn4 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn4)) * assign56070_e91849) - (assign56070_e91844 * (((2.0 * locals.var_t5_dn4) * locals.var_t5) + (assign56070_e91847 * locals.var_t5_dn4)))) / (assign56070_e91849 * assign56070_e91849))))), (locals.var_t3_dn5 - (((((locals.var_t4_dn5 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)) * assign56070_e91851) + (assign56070_e91840 * (((((locals.var_t4_dn5 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn5)) * assign56070_e91849) - (assign56070_e91844 * (((2.0 * locals.var_t5_dn5) * locals.var_t5) + (assign56070_e91847 * locals.var_t5_dn5)))) / (assign56070_e91849 * assign56070_e91849))))), (locals.var_t3_dn6 - (((((locals.var_t4_dn6 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5)) * assign56070_e91851) + (assign56070_e91840 * (((((locals.var_t4_dn6 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn6)) * assign56070_e91849) - (assign56070_e91844 * (((2.0 * locals.var_t5_dn6) * locals.var_t5) + (assign56070_e91847 * locals.var_t5_dn6)))) / (assign56070_e91849 * assign56070_e91849))))), (locals.var_t3_dn7 - (((((locals.var_t4_dn7 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5)) * assign56070_e91851) + (assign56070_e91840 * (((((locals.var_t4_dn7 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn7)) * assign56070_e91849) - (assign56070_e91844 * (((2.0 * locals.var_t5_dn7) * locals.var_t5) + (assign56070_e91847 * locals.var_t5_dn7)))) / (assign56070_e91849 * assign56070_e91849))))), (locals.var_t3_dn8 - (((((locals.var_t4_dn8 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5)) * assign56070_e91851) + (assign56070_e91840 * (((((locals.var_t4_dn8 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn8)) * assign56070_e91849) - (assign56070_e91844 * (((2.0 * locals.var_t5_dn8) * locals.var_t5) + (assign56070_e91847 * locals.var_t5_dn8)))) / (assign56070_e91849 * assign56070_e91849))))), (locals.var_t3_dn9 - (((((locals.var_t4_dn9 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5)) * assign56070_e91851) + (assign56070_e91840 * (((((locals.var_t4_dn9 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn9)) * assign56070_e91849) - (assign56070_e91844 * (((2.0 * locals.var_t5_dn9) * locals.var_t5) + (assign56070_e91847 * locals.var_t5_dn9)))) / (assign56070_e91849 * assign56070_e91849))))), (locals.var_t3_dn10 - (((((locals.var_t4_dn10 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5)) * assign56070_e91851) + (assign56070_e91840 * (((((locals.var_t4_dn10 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn10)) * assign56070_e91849) - (assign56070_e91844 * (((2.0 * locals.var_t5_dn10) * locals.var_t5) + (assign56070_e91847 * locals.var_t5_dn10)))) / (assign56070_e91849 * assign56070_e91849))))), (locals.var_t3_dn11 - (((((locals.var_t4_dn11 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5)) * assign56070_e91851) + (assign56070_e91840 * (((((locals.var_t4_dn11 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn11)) * assign56070_e91849) - (assign56070_e91844 * (((2.0 * locals.var_t5_dn11) * locals.var_t5) + (assign56070_e91847 * locals.var_t5_dn11)))) / (assign56070_e91849 * assign56070_e91849))))),)
    } else {
        (locals.var_qdeff, locals.var_qdeff_dn3, locals.var_qdeff_dn4, locals.var_qdeff_dn5, locals.var_qdeff_dn6, locals.var_qdeff_dn7, locals.var_qdeff_dn8, locals.var_qdeff_dn9, locals.var_qdeff_dn10, locals.var_qdeff_dn11,)
    }
};
        locals.var_qdeff = assign56070_e91855;
        locals.var_qdeff_dn3 = assign56070_e91855_d_n3;
        locals.var_qdeff_dn4 = assign56070_e91855_d_n4;
        locals.var_qdeff_dn5 = assign56070_e91855_d_n5;
        locals.var_qdeff_dn6 = assign56070_e91855_d_n6;
        locals.var_qdeff_dn7 = assign56070_e91855_d_n7;
        locals.var_qdeff_dn8 = assign56070_e91855_d_n8;
        locals.var_qdeff_dn9 = assign56070_e91855_d_n9;
        locals.var_qdeff_dn10 = assign56070_e91855_d_n10;
        locals.var_qdeff_dn11 = assign56070_e91855_d_n11;

        let (assign56080_e91868, assign56080_e91868_d_n3, assign56080_e91868_d_n4, assign56080_e91868_d_n5, assign56080_e91868_d_n6, assign56080_e91868_d_n7, assign56080_e91868_d_n8, assign56080_e91868_d_n9, assign56080_e91868_d_n10, assign56080_e91868_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign56080_e91862: f64 = (locals.var_psip - locals.var_qs_1);
        let assign56080_e91864: f64 = (assign56080_e91862 - locals.var_qdeff);
        let assign56080_e91866: f64 = (assign56080_e91864 - 1.0);
        (assign56080_e91866, ((locals.var_psip_dn3 - locals.var_qs_1_dn3) - locals.var_qdeff_dn3), ((locals.var_psip_dn4 - locals.var_qs_1_dn4) - locals.var_qdeff_dn4), ((locals.var_psip_dn5 - locals.var_qs_1_dn5) - locals.var_qdeff_dn5), ((locals.var_psip_dn6 - locals.var_qs_1_dn6) - locals.var_qdeff_dn6), ((locals.var_psip_dn7 - locals.var_qs_1_dn7) - locals.var_qdeff_dn7), ((locals.var_psip_dn8 - locals.var_qs_1_dn8) - locals.var_qdeff_dn8), ((locals.var_psip_dn9 - locals.var_qs_1_dn9) - locals.var_qdeff_dn9), ((locals.var_psip_dn10 - locals.var_qs_1_dn10) - locals.var_qdeff_dn10), ((locals.var_psip_dn11 - locals.var_qs_1_dn11) - locals.var_qdeff_dn11),)
    } else {
        (locals.var_psiavg, locals.var_psiavg_dn3, locals.var_psiavg_dn4, locals.var_psiavg_dn5, locals.var_psiavg_dn6, locals.var_psiavg_dn7, locals.var_psiavg_dn8, locals.var_psiavg_dn9, locals.var_psiavg_dn10, locals.var_psiavg_dn11,)
    }
};
        locals.var_psiavg = assign56080_e91868;
        locals.var_psiavg_dn3 = assign56080_e91868_d_n3;
        locals.var_psiavg_dn4 = assign56080_e91868_d_n4;
        locals.var_psiavg_dn5 = assign56080_e91868_d_n5;
        locals.var_psiavg_dn6 = assign56080_e91868_d_n6;
        locals.var_psiavg_dn7 = assign56080_e91868_d_n7;
        locals.var_psiavg_dn8 = assign56080_e91868_d_n8;
        locals.var_psiavg_dn9 = assign56080_e91868_d_n9;
        locals.var_psiavg_dn10 = assign56080_e91868_d_n10;
        locals.var_psiavg_dn11 = assign56080_e91868_d_n11;

        let (assign56090_e91894, assign56090_e91894_d_n3, assign56090_e91894_d_n4, assign56090_e91894_d_n5, assign56090_e91894_d_n6, assign56090_e91894_d_n7, assign56090_e91894_d_n8, assign56090_e91894_d_n9, assign56090_e91894_d_n10, assign56090_e91894_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign56090_e91876: f64 = (locals.var_psiavg + 1.0);
        let assign56090_e91879: f64 = (locals.var_psiavg - 1.0);
        let assign56090_e91882: f64 = (locals.var_psiavg - 1.0);
        let assign56090_e91883: f64 = (assign56090_e91879 * assign56090_e91882);
        let assign56090_e91886: f64 = (0.25 * 2.0);
        let assign56090_e91888: f64 = (assign56090_e91886 * 2.0);
        let assign56090_e91889: f64 = (assign56090_e91883 + assign56090_e91888);
        let assign56090_e91890: f64 = (assign56090_e91889).sqrt();
        let assign56090_e91891: f64 = (assign56090_e91876 + assign56090_e91890);
        let assign56090_e91892: f64 = (0.5 * assign56090_e91891);
        (assign56090_e91892, (0.5 * (locals.var_psiavg_dn3 + (((locals.var_psiavg_dn3 * assign56090_e91882) + (assign56090_e91879 * locals.var_psiavg_dn3)) / (2.0 * assign56090_e91890)))), (0.5 * (locals.var_psiavg_dn4 + (((locals.var_psiavg_dn4 * assign56090_e91882) + (assign56090_e91879 * locals.var_psiavg_dn4)) / (2.0 * assign56090_e91890)))), (0.5 * (locals.var_psiavg_dn5 + (((locals.var_psiavg_dn5 * assign56090_e91882) + (assign56090_e91879 * locals.var_psiavg_dn5)) / (2.0 * assign56090_e91890)))), (0.5 * (locals.var_psiavg_dn6 + (((locals.var_psiavg_dn6 * assign56090_e91882) + (assign56090_e91879 * locals.var_psiavg_dn6)) / (2.0 * assign56090_e91890)))), (0.5 * (locals.var_psiavg_dn7 + (((locals.var_psiavg_dn7 * assign56090_e91882) + (assign56090_e91879 * locals.var_psiavg_dn7)) / (2.0 * assign56090_e91890)))), (0.5 * (locals.var_psiavg_dn8 + (((locals.var_psiavg_dn8 * assign56090_e91882) + (assign56090_e91879 * locals.var_psiavg_dn8)) / (2.0 * assign56090_e91890)))), (0.5 * (locals.var_psiavg_dn9 + (((locals.var_psiavg_dn9 * assign56090_e91882) + (assign56090_e91879 * locals.var_psiavg_dn9)) / (2.0 * assign56090_e91890)))), (0.5 * (locals.var_psiavg_dn10 + (((locals.var_psiavg_dn10 * assign56090_e91882) + (assign56090_e91879 * locals.var_psiavg_dn10)) / (2.0 * assign56090_e91890)))), (0.5 * (locals.var_psiavg_dn11 + (((locals.var_psiavg_dn11 * assign56090_e91882) + (assign56090_e91879 * locals.var_psiavg_dn11)) / (2.0 * assign56090_e91890)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign56090_e91894;
        locals.var_t0_dn3 = assign56090_e91894_d_n3;
        locals.var_t0_dn4 = assign56090_e91894_d_n4;
        locals.var_t0_dn5 = assign56090_e91894_d_n5;
        locals.var_t0_dn6 = assign56090_e91894_d_n6;
        locals.var_t0_dn7 = assign56090_e91894_d_n7;
        locals.var_t0_dn8 = assign56090_e91894_d_n8;
        locals.var_t0_dn9 = assign56090_e91894_d_n9;
        locals.var_t0_dn10 = assign56090_e91894_d_n10;
        locals.var_t0_dn11 = assign56090_e91894_d_n11;

        let (assign56100_e91902, assign56100_e91902_d_n3, assign56100_e91902_d_n4, assign56100_e91902_d_n5, assign56100_e91902_d_n6, assign56100_e91902_d_n7, assign56100_e91902_d_n8, assign56100_e91902_d_n9, assign56100_e91902_d_n10, assign56100_e91902_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign56100_e91900: f64 = (locals.var_t0).sqrt();
        (assign56100_e91900, (locals.var_t0_dn3 / (2.0 * assign56100_e91900)), (locals.var_t0_dn4 / (2.0 * assign56100_e91900)), (locals.var_t0_dn5 / (2.0 * assign56100_e91900)), (locals.var_t0_dn6 / (2.0 * assign56100_e91900)), (locals.var_t0_dn7 / (2.0 * assign56100_e91900)), (locals.var_t0_dn8 / (2.0 * assign56100_e91900)), (locals.var_t0_dn9 / (2.0 * assign56100_e91900)), (locals.var_t0_dn10 / (2.0 * assign56100_e91900)), (locals.var_t0_dn11 / (2.0 * assign56100_e91900)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign56100_e91902;
        locals.var_t2_dn3 = assign56100_e91902_d_n3;
        locals.var_t2_dn4 = assign56100_e91902_d_n4;
        locals.var_t2_dn5 = assign56100_e91902_d_n5;
        locals.var_t2_dn6 = assign56100_e91902_d_n6;
        locals.var_t2_dn7 = assign56100_e91902_d_n7;
        locals.var_t2_dn8 = assign56100_e91902_d_n8;
        locals.var_t2_dn9 = assign56100_e91902_d_n9;
        locals.var_t2_dn10 = assign56100_e91902_d_n10;
        locals.var_t2_dn11 = assign56100_e91902_d_n11;

        let (assign56110_e91917, assign56110_e91917_d_n3, assign56110_e91917_d_n4, assign56110_e91917_d_n5, assign56110_e91917_d_n6, assign56110_e91917_d_n7, assign56110_e91917_d_n8, assign56110_e91917_d_n9, assign56110_e91917_d_n10, assign56110_e91917_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign56110_e91909: f64 = (1.0 + locals.var_dpd);
        let assign56110_e91913: f64 = (locals.var_sqrtpsip + locals.var_t2);
        let assign56110_e91914: f64 = (locals.var_gamcv / assign56110_e91913);
        let assign56110_e91915: f64 = (assign56110_e91909 + assign56110_e91914);
        (assign56110_e91915, (locals.var_dpd_dn3 + (((locals.var_gamcv_dn3 * assign56110_e91913) - (locals.var_gamcv * (locals.var_sqrtpsip_dn3 + locals.var_t2_dn3))) / (assign56110_e91913 * assign56110_e91913))), (locals.var_dpd_dn4 + (((locals.var_gamcv_dn4 * assign56110_e91913) - (locals.var_gamcv * (locals.var_sqrtpsip_dn4 + locals.var_t2_dn4))) / (assign56110_e91913 * assign56110_e91913))), (locals.var_dpd_dn5 + (((locals.var_gamcv_dn5 * assign56110_e91913) - (locals.var_gamcv * (locals.var_sqrtpsip_dn5 + locals.var_t2_dn5))) / (assign56110_e91913 * assign56110_e91913))), (locals.var_dpd_dn6 + (((locals.var_gamcv_dn6 * assign56110_e91913) - (locals.var_gamcv * (locals.var_sqrtpsip_dn6 + locals.var_t2_dn6))) / (assign56110_e91913 * assign56110_e91913))), (locals.var_dpd_dn7 + (((locals.var_gamcv_dn7 * assign56110_e91913) - (locals.var_gamcv * (locals.var_sqrtpsip_dn7 + locals.var_t2_dn7))) / (assign56110_e91913 * assign56110_e91913))), (locals.var_dpd_dn8 + (((locals.var_gamcv_dn8 * assign56110_e91913) - (locals.var_gamcv * (locals.var_sqrtpsip_dn8 + locals.var_t2_dn8))) / (assign56110_e91913 * assign56110_e91913))), (locals.var_dpd_dn9 + (((locals.var_gamcv_dn9 * assign56110_e91913) - (locals.var_gamcv * (locals.var_sqrtpsip_dn9 + locals.var_t2_dn9))) / (assign56110_e91913 * assign56110_e91913))), (locals.var_dpd_dn10 + (((locals.var_gamcv_dn10 * assign56110_e91913) - (locals.var_gamcv * (locals.var_sqrtpsip_dn10 + locals.var_t2_dn10))) / (assign56110_e91913 * assign56110_e91913))), (locals.var_dpd_dn11 + (((locals.var_gamcv_dn11 * assign56110_e91913) - (locals.var_gamcv * (locals.var_sqrtpsip_dn11 + locals.var_t2_dn11))) / (assign56110_e91913 * assign56110_e91913))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign56110_e91917;
        locals.var_t3_dn3 = assign56110_e91917_d_n3;
        locals.var_t3_dn4 = assign56110_e91917_d_n4;
        locals.var_t3_dn5 = assign56110_e91917_d_n5;
        locals.var_t3_dn6 = assign56110_e91917_d_n6;
        locals.var_t3_dn7 = assign56110_e91917_d_n7;
        locals.var_t3_dn8 = assign56110_e91917_d_n8;
        locals.var_t3_dn9 = assign56110_e91917_d_n9;
        locals.var_t3_dn10 = assign56110_e91917_d_n10;
        locals.var_t3_dn11 = assign56110_e91917_d_n11;

        let (assign56120_e91930, assign56120_e91930_d_n3, assign56120_e91930_d_n4, assign56120_e91930_d_n5, assign56120_e91930_d_n6, assign56120_e91930_d_n7, assign56120_e91930_d_n8, assign56120_e91930_d_n9, assign56120_e91930_d_n10, assign56120_e91930_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign56120_e91925: f64 = (locals.var_dpd * locals.var_t2);
        let assign56120_e91927: f64 = (assign56120_e91925 * locals.var_inv_gam);
        let assign56120_e91928: f64 = (0.5 + assign56120_e91927);
        (assign56120_e91928, ((((locals.var_dpd_dn3 * locals.var_t2) + (locals.var_dpd * locals.var_t2_dn3)) * locals.var_inv_gam) + (assign56120_e91925 * locals.var_inv_gam_dn3)), ((((locals.var_dpd_dn4 * locals.var_t2) + (locals.var_dpd * locals.var_t2_dn4)) * locals.var_inv_gam) + (assign56120_e91925 * locals.var_inv_gam_dn4)), ((((locals.var_dpd_dn5 * locals.var_t2) + (locals.var_dpd * locals.var_t2_dn5)) * locals.var_inv_gam) + (assign56120_e91925 * locals.var_inv_gam_dn5)), ((((locals.var_dpd_dn6 * locals.var_t2) + (locals.var_dpd * locals.var_t2_dn6)) * locals.var_inv_gam) + (assign56120_e91925 * locals.var_inv_gam_dn6)), ((((locals.var_dpd_dn7 * locals.var_t2) + (locals.var_dpd * locals.var_t2_dn7)) * locals.var_inv_gam) + (assign56120_e91925 * locals.var_inv_gam_dn7)), ((((locals.var_dpd_dn8 * locals.var_t2) + (locals.var_dpd * locals.var_t2_dn8)) * locals.var_inv_gam) + (assign56120_e91925 * locals.var_inv_gam_dn8)), ((((locals.var_dpd_dn9 * locals.var_t2) + (locals.var_dpd * locals.var_t2_dn9)) * locals.var_inv_gam) + (assign56120_e91925 * locals.var_inv_gam_dn9)), ((((locals.var_dpd_dn10 * locals.var_t2) + (locals.var_dpd * locals.var_t2_dn10)) * locals.var_inv_gam) + (assign56120_e91925 * locals.var_inv_gam_dn10)), ((((locals.var_dpd_dn11 * locals.var_t2) + (locals.var_dpd * locals.var_t2_dn11)) * locals.var_inv_gam) + (assign56120_e91925 * locals.var_inv_gam_dn11)),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign56120_e91930;
        locals.var_t4_dn3 = assign56120_e91930_d_n3;
        locals.var_t4_dn4 = assign56120_e91930_d_n4;
        locals.var_t4_dn5 = assign56120_e91930_d_n5;
        locals.var_t4_dn6 = assign56120_e91930_d_n6;
        locals.var_t4_dn7 = assign56120_e91930_d_n7;
        locals.var_t4_dn8 = assign56120_e91930_d_n8;
        locals.var_t4_dn9 = assign56120_e91930_d_n9;
        locals.var_t4_dn10 = assign56120_e91930_d_n10;
        locals.var_t4_dn11 = assign56120_e91930_d_n11;

        let (assign56130_e91948, assign56130_e91948_d_n3, assign56130_e91948_d_n4, assign56130_e91948_d_n5, assign56130_e91948_d_n6, assign56130_e91948_d_n7, assign56130_e91948_d_n8, assign56130_e91948_d_n9, assign56130_e91948_d_n10, assign56130_e91948_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign56130_e91937: f64 = (locals.var_t4 * locals.var_t4);
        let assign56130_e91941: f64 = (locals.var_qs_1 + locals.var_qdeff);
        let assign56130_e91942: f64 = (locals.var_t3 * assign56130_e91941);
        let assign56130_e91944: f64 = (assign56130_e91942 * locals.var_invgamg2);
        let assign56130_e91945: f64 = (assign56130_e91937 + assign56130_e91944);
        let assign56130_e91946: f64 = (assign56130_e91945).sqrt();
        (assign56130_e91946, ((((locals.var_t4_dn3 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn3)) + (((locals.var_t3_dn3 * assign56130_e91941) + (locals.var_t3 * (locals.var_qs_1_dn3 + locals.var_qdeff_dn3))) * locals.var_invgamg2)) / (2.0 * assign56130_e91946)), ((((locals.var_t4_dn4 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn4)) + ((((locals.var_t3_dn4 * assign56130_e91941) + (locals.var_t3 * (locals.var_qs_1_dn4 + locals.var_qdeff_dn4))) * locals.var_invgamg2) + (assign56130_e91942 * locals.var_invgamg2_dn4))) / (2.0 * assign56130_e91946)), ((((locals.var_t4_dn5 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn5)) + ((((locals.var_t3_dn5 * assign56130_e91941) + (locals.var_t3 * (locals.var_qs_1_dn5 + locals.var_qdeff_dn5))) * locals.var_invgamg2) + (assign56130_e91942 * locals.var_invgamg2_dn5))) / (2.0 * assign56130_e91946)), ((((locals.var_t4_dn6 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn6)) + (((locals.var_t3_dn6 * assign56130_e91941) + (locals.var_t3 * (locals.var_qs_1_dn6 + locals.var_qdeff_dn6))) * locals.var_invgamg2)) / (2.0 * assign56130_e91946)), ((((locals.var_t4_dn7 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn7)) + (((locals.var_t3_dn7 * assign56130_e91941) + (locals.var_t3 * (locals.var_qs_1_dn7 + locals.var_qdeff_dn7))) * locals.var_invgamg2)) / (2.0 * assign56130_e91946)), ((((locals.var_t4_dn8 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn8)) + (((locals.var_t3_dn8 * assign56130_e91941) + (locals.var_t3 * (locals.var_qs_1_dn8 + locals.var_qdeff_dn8))) * locals.var_invgamg2)) / (2.0 * assign56130_e91946)), ((((locals.var_t4_dn9 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn9)) + (((locals.var_t3_dn9 * assign56130_e91941) + (locals.var_t3 * (locals.var_qs_1_dn9 + locals.var_qdeff_dn9))) * locals.var_invgamg2)) / (2.0 * assign56130_e91946)), ((((locals.var_t4_dn10 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn10)) + (((locals.var_t3_dn10 * assign56130_e91941) + (locals.var_t3 * (locals.var_qs_1_dn10 + locals.var_qdeff_dn10))) * locals.var_invgamg2)) / (2.0 * assign56130_e91946)), ((((locals.var_t4_dn11 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn11)) + (((locals.var_t3_dn11 * assign56130_e91941) + (locals.var_t3 * (locals.var_qs_1_dn11 + locals.var_qdeff_dn11))) * locals.var_invgamg2)) / (2.0 * assign56130_e91946)),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign56130_e91948;
        locals.var_t5_dn3 = assign56130_e91948_d_n3;
        locals.var_t5_dn4 = assign56130_e91948_d_n4;
        locals.var_t5_dn5 = assign56130_e91948_d_n5;
        locals.var_t5_dn6 = assign56130_e91948_d_n6;
        locals.var_t5_dn7 = assign56130_e91948_d_n7;
        locals.var_t5_dn8 = assign56130_e91948_d_n8;
        locals.var_t5_dn9 = assign56130_e91948_d_n9;
        locals.var_t5_dn10 = assign56130_e91948_d_n10;
        locals.var_t5_dn11 = assign56130_e91948_d_n11;

        let (assign56140_e91959, assign56140_e91959_d_n3, assign56140_e91959_d_n4, assign56140_e91959_d_n5, assign56140_e91959_d_n6, assign56140_e91959_d_n7, assign56140_e91959_d_n8, assign56140_e91959_d_n9, assign56140_e91959_d_n10, assign56140_e91959_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign56140_e91956: f64 = (locals.var_t4 + locals.var_t5);
        let assign56140_e91957: f64 = (locals.var_t3 / assign56140_e91956);
        (assign56140_e91957, (((locals.var_t3_dn3 * assign56140_e91956) - (locals.var_t3 * (locals.var_t4_dn3 + locals.var_t5_dn3))) / (assign56140_e91956 * assign56140_e91956)), (((locals.var_t3_dn4 * assign56140_e91956) - (locals.var_t3 * (locals.var_t4_dn4 + locals.var_t5_dn4))) / (assign56140_e91956 * assign56140_e91956)), (((locals.var_t3_dn5 * assign56140_e91956) - (locals.var_t3 * (locals.var_t4_dn5 + locals.var_t5_dn5))) / (assign56140_e91956 * assign56140_e91956)), (((locals.var_t3_dn6 * assign56140_e91956) - (locals.var_t3 * (locals.var_t4_dn6 + locals.var_t5_dn6))) / (assign56140_e91956 * assign56140_e91956)), (((locals.var_t3_dn7 * assign56140_e91956) - (locals.var_t3 * (locals.var_t4_dn7 + locals.var_t5_dn7))) / (assign56140_e91956 * assign56140_e91956)), (((locals.var_t3_dn8 * assign56140_e91956) - (locals.var_t3 * (locals.var_t4_dn8 + locals.var_t5_dn8))) / (assign56140_e91956 * assign56140_e91956)), (((locals.var_t3_dn9 * assign56140_e91956) - (locals.var_t3 * (locals.var_t4_dn9 + locals.var_t5_dn9))) / (assign56140_e91956 * assign56140_e91956)), (((locals.var_t3_dn10 * assign56140_e91956) - (locals.var_t3 * (locals.var_t4_dn10 + locals.var_t5_dn10))) / (assign56140_e91956 * assign56140_e91956)), (((locals.var_t3_dn11 * assign56140_e91956) - (locals.var_t3 * (locals.var_t4_dn11 + locals.var_t5_dn11))) / (assign56140_e91956 * assign56140_e91956)),)
    } else {
        (locals.var_nq, locals.var_nq_dn3, locals.var_nq_dn4, locals.var_nq_dn5, locals.var_nq_dn6, locals.var_nq_dn7, locals.var_nq_dn8, locals.var_nq_dn9, locals.var_nq_dn10, locals.var_nq_dn11,)
    }
};
        locals.var_nq = assign56140_e91959;
        locals.var_nq_dn3 = assign56140_e91959_d_n3;
        locals.var_nq_dn4 = assign56140_e91959_d_n4;
        locals.var_nq_dn5 = assign56140_e91959_d_n5;
        locals.var_nq_dn6 = assign56140_e91959_d_n6;
        locals.var_nq_dn7 = assign56140_e91959_d_n7;
        locals.var_nq_dn8 = assign56140_e91959_d_n8;
        locals.var_nq_dn9 = assign56140_e91959_d_n9;
        locals.var_nq_dn10 = assign56140_e91959_d_n10;
        locals.var_nq_dn11 = assign56140_e91959_d_n11;

        let (assign56150_e91978, assign56150_e91978_d_n3, assign56150_e91978_d_n4, assign56150_e91978_d_n5, assign56150_e91978_d_n6, assign56150_e91978_d_n7, assign56150_e91978_d_n8, assign56150_e91978_d_n9, assign56150_e91978_d_n10, assign56150_e91978_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign56150_e91967: f64 = (locals.var_vgfbcv - locals.var_psip);
        let assign56150_e91970: f64 = (2.0 * locals.var_qs_1);
        let assign56150_e91973: f64 = (locals.var_nq - 1.0);
        let assign56150_e91974: f64 = (assign56150_e91970 * assign56150_e91973);
        let assign56150_e91975: f64 = (assign56150_e91967 - assign56150_e91974);
        let assign56150_e91976: f64 = (locals.var_vt * assign56150_e91975);
        (assign56150_e91976, (locals.var_vt * ((locals.var_vgfbcv_dn3 - locals.var_psip_dn3) - (((2.0 * locals.var_qs_1_dn3) * assign56150_e91973) + (assign56150_e91970 * locals.var_nq_dn3)))), ((locals.var_vt_dn4 * assign56150_e91975) + (locals.var_vt * ((locals.var_vgfbcv_dn4 - locals.var_psip_dn4) - (((2.0 * locals.var_qs_1_dn4) * assign56150_e91973) + (assign56150_e91970 * locals.var_nq_dn4))))), ((locals.var_vt_dn5 * assign56150_e91975) + (locals.var_vt * ((locals.var_vgfbcv_dn5 - locals.var_psip_dn5) - (((2.0 * locals.var_qs_1_dn5) * assign56150_e91973) + (assign56150_e91970 * locals.var_nq_dn5))))), (locals.var_vt * ((locals.var_vgfbcv_dn6 - locals.var_psip_dn6) - (((2.0 * locals.var_qs_1_dn6) * assign56150_e91973) + (assign56150_e91970 * locals.var_nq_dn6)))), (locals.var_vt * ((locals.var_vgfbcv_dn7 - locals.var_psip_dn7) - (((2.0 * locals.var_qs_1_dn7) * assign56150_e91973) + (assign56150_e91970 * locals.var_nq_dn7)))), (locals.var_vt * ((locals.var_vgfbcv_dn8 - locals.var_psip_dn8) - (((2.0 * locals.var_qs_1_dn8) * assign56150_e91973) + (assign56150_e91970 * locals.var_nq_dn8)))), (locals.var_vt * ((locals.var_vgfbcv_dn9 - locals.var_psip_dn9) - (((2.0 * locals.var_qs_1_dn9) * assign56150_e91973) + (assign56150_e91970 * locals.var_nq_dn9)))), (locals.var_vt * ((locals.var_vgfbcv_dn10 - locals.var_psip_dn10) - (((2.0 * locals.var_qs_1_dn10) * assign56150_e91973) + (assign56150_e91970 * locals.var_nq_dn10)))), (locals.var_vt * ((locals.var_vgfbcv_dn11 - locals.var_psip_dn11) - (((2.0 * locals.var_qs_1_dn11) * assign56150_e91973) + (assign56150_e91970 * locals.var_nq_dn11)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign56150_e91978;
        locals.var_t0_dn3 = assign56150_e91978_d_n3;
        locals.var_t0_dn4 = assign56150_e91978_d_n4;
        locals.var_t0_dn5 = assign56150_e91978_d_n5;
        locals.var_t0_dn6 = assign56150_e91978_d_n6;
        locals.var_t0_dn7 = assign56150_e91978_d_n7;
        locals.var_t0_dn8 = assign56150_e91978_d_n8;
        locals.var_t0_dn9 = assign56150_e91978_d_n9;
        locals.var_t0_dn10 = assign56150_e91978_d_n10;
        locals.var_t0_dn11 = assign56150_e91978_d_n11;

        let (assign56160_e92004, assign56160_e92004_d_n3, assign56160_e92004_d_n4, assign56160_e92004_d_n5, assign56160_e92004_d_n6, assign56160_e92004_d_n7, assign56160_e92004_d_n8, assign56160_e92004_d_n9, assign56160_e92004_d_n10, assign56160_e92004_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign56160_e91986: f64 = locals.var_t0;
        let assign56160_e91989: f64 = locals.var_t0;
        let assign56160_e91992: f64 = locals.var_t0;
        let assign56160_e91993: f64 = (assign56160_e91989 * assign56160_e91992);
        let assign56160_e91996: f64 = (0.25 * 0.1);
        let assign56160_e91998: f64 = (assign56160_e91996 * 0.1);
        let assign56160_e91999: f64 = (assign56160_e91993 + assign56160_e91998);
        let assign56160_e92000: f64 = (assign56160_e91999).sqrt();
        let assign56160_e92001: f64 = (assign56160_e91986 + assign56160_e92000);
        let assign56160_e92002: f64 = (0.5 * assign56160_e92001);
        (assign56160_e92002, (0.5 * (locals.var_t0_dn3 + (((locals.var_t0_dn3 * assign56160_e91992) + (assign56160_e91989 * locals.var_t0_dn3)) / (2.0 * assign56160_e92000)))), (0.5 * (locals.var_t0_dn4 + (((locals.var_t0_dn4 * assign56160_e91992) + (assign56160_e91989 * locals.var_t0_dn4)) / (2.0 * assign56160_e92000)))), (0.5 * (locals.var_t0_dn5 + (((locals.var_t0_dn5 * assign56160_e91992) + (assign56160_e91989 * locals.var_t0_dn5)) / (2.0 * assign56160_e92000)))), (0.5 * (locals.var_t0_dn6 + (((locals.var_t0_dn6 * assign56160_e91992) + (assign56160_e91989 * locals.var_t0_dn6)) / (2.0 * assign56160_e92000)))), (0.5 * (locals.var_t0_dn7 + (((locals.var_t0_dn7 * assign56160_e91992) + (assign56160_e91989 * locals.var_t0_dn7)) / (2.0 * assign56160_e92000)))), (0.5 * (locals.var_t0_dn8 + (((locals.var_t0_dn8 * assign56160_e91992) + (assign56160_e91989 * locals.var_t0_dn8)) / (2.0 * assign56160_e92000)))), (0.5 * (locals.var_t0_dn9 + (((locals.var_t0_dn9 * assign56160_e91992) + (assign56160_e91989 * locals.var_t0_dn9)) / (2.0 * assign56160_e92000)))), (0.5 * (locals.var_t0_dn10 + (((locals.var_t0_dn10 * assign56160_e91992) + (assign56160_e91989 * locals.var_t0_dn10)) / (2.0 * assign56160_e92000)))), (0.5 * (locals.var_t0_dn11 + (((locals.var_t0_dn11 * assign56160_e91992) + (assign56160_e91989 * locals.var_t0_dn11)) / (2.0 * assign56160_e92000)))),)
    } else {
        (locals.var_qbs, locals.var_qbs_dn3, locals.var_qbs_dn4, locals.var_qbs_dn5, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn8, locals.var_qbs_dn9, locals.var_qbs_dn10, locals.var_qbs_dn11,)
    }
};
        locals.var_qbs = assign56160_e92004;
        locals.var_qbs_dn3 = assign56160_e92004_d_n3;
        locals.var_qbs_dn4 = assign56160_e92004_d_n4;
        locals.var_qbs_dn5 = assign56160_e92004_d_n5;
        locals.var_qbs_dn6 = assign56160_e92004_d_n6;
        locals.var_qbs_dn7 = assign56160_e92004_d_n7;
        locals.var_qbs_dn8 = assign56160_e92004_d_n8;
        locals.var_qbs_dn9 = assign56160_e92004_d_n9;
        locals.var_qbs_dn10 = assign56160_e92004_d_n10;
        locals.var_qbs_dn11 = assign56160_e92004_d_n11;

        let (assign56170_e92023, assign56170_e92023_d_n3, assign56170_e92023_d_n4, assign56170_e92023_d_n5, assign56170_e92023_d_n6, assign56170_e92023_d_n7, assign56170_e92023_d_n8, assign56170_e92023_d_n9, assign56170_e92023_d_n10, assign56170_e92023_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign56170_e92012: f64 = (locals.var_vgfbcv - locals.var_psip);
        let assign56170_e92015: f64 = (2.0 * locals.var_qdeff);
        let assign56170_e92018: f64 = (locals.var_nq - 1.0);
        let assign56170_e92019: f64 = (assign56170_e92015 * assign56170_e92018);
        let assign56170_e92020: f64 = (assign56170_e92012 - assign56170_e92019);
        let assign56170_e92021: f64 = (locals.var_vt * assign56170_e92020);
        (assign56170_e92021, (locals.var_vt * ((locals.var_vgfbcv_dn3 - locals.var_psip_dn3) - (((2.0 * locals.var_qdeff_dn3) * assign56170_e92018) + (assign56170_e92015 * locals.var_nq_dn3)))), ((locals.var_vt_dn4 * assign56170_e92020) + (locals.var_vt * ((locals.var_vgfbcv_dn4 - locals.var_psip_dn4) - (((2.0 * locals.var_qdeff_dn4) * assign56170_e92018) + (assign56170_e92015 * locals.var_nq_dn4))))), ((locals.var_vt_dn5 * assign56170_e92020) + (locals.var_vt * ((locals.var_vgfbcv_dn5 - locals.var_psip_dn5) - (((2.0 * locals.var_qdeff_dn5) * assign56170_e92018) + (assign56170_e92015 * locals.var_nq_dn5))))), (locals.var_vt * ((locals.var_vgfbcv_dn6 - locals.var_psip_dn6) - (((2.0 * locals.var_qdeff_dn6) * assign56170_e92018) + (assign56170_e92015 * locals.var_nq_dn6)))), (locals.var_vt * ((locals.var_vgfbcv_dn7 - locals.var_psip_dn7) - (((2.0 * locals.var_qdeff_dn7) * assign56170_e92018) + (assign56170_e92015 * locals.var_nq_dn7)))), (locals.var_vt * ((locals.var_vgfbcv_dn8 - locals.var_psip_dn8) - (((2.0 * locals.var_qdeff_dn8) * assign56170_e92018) + (assign56170_e92015 * locals.var_nq_dn8)))), (locals.var_vt * ((locals.var_vgfbcv_dn9 - locals.var_psip_dn9) - (((2.0 * locals.var_qdeff_dn9) * assign56170_e92018) + (assign56170_e92015 * locals.var_nq_dn9)))), (locals.var_vt * ((locals.var_vgfbcv_dn10 - locals.var_psip_dn10) - (((2.0 * locals.var_qdeff_dn10) * assign56170_e92018) + (assign56170_e92015 * locals.var_nq_dn10)))), (locals.var_vt * ((locals.var_vgfbcv_dn11 - locals.var_psip_dn11) - (((2.0 * locals.var_qdeff_dn11) * assign56170_e92018) + (assign56170_e92015 * locals.var_nq_dn11)))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign56170_e92023;
        locals.var_t1_dn3 = assign56170_e92023_d_n3;
        locals.var_t1_dn4 = assign56170_e92023_d_n4;
        locals.var_t1_dn5 = assign56170_e92023_d_n5;
        locals.var_t1_dn6 = assign56170_e92023_d_n6;
        locals.var_t1_dn7 = assign56170_e92023_d_n7;
        locals.var_t1_dn8 = assign56170_e92023_d_n8;
        locals.var_t1_dn9 = assign56170_e92023_d_n9;
        locals.var_t1_dn10 = assign56170_e92023_d_n10;
        locals.var_t1_dn11 = assign56170_e92023_d_n11;

        let (assign56180_e92049, assign56180_e92049_d_n3, assign56180_e92049_d_n4, assign56180_e92049_d_n5, assign56180_e92049_d_n6, assign56180_e92049_d_n7, assign56180_e92049_d_n8, assign56180_e92049_d_n9, assign56180_e92049_d_n10, assign56180_e92049_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign56180_e92031: f64 = locals.var_t1;
        let assign56180_e92034: f64 = locals.var_t1;
        let assign56180_e92037: f64 = locals.var_t1;
        let assign56180_e92038: f64 = (assign56180_e92034 * assign56180_e92037);
        let assign56180_e92041: f64 = (0.25 * 0.1);
        let assign56180_e92043: f64 = (assign56180_e92041 * 0.1);
        let assign56180_e92044: f64 = (assign56180_e92038 + assign56180_e92043);
        let assign56180_e92045: f64 = (assign56180_e92044).sqrt();
        let assign56180_e92046: f64 = (assign56180_e92031 + assign56180_e92045);
        let assign56180_e92047: f64 = (0.5 * assign56180_e92046);
        (assign56180_e92047, (0.5 * (locals.var_t1_dn3 + (((locals.var_t1_dn3 * assign56180_e92037) + (assign56180_e92034 * locals.var_t1_dn3)) / (2.0 * assign56180_e92045)))), (0.5 * (locals.var_t1_dn4 + (((locals.var_t1_dn4 * assign56180_e92037) + (assign56180_e92034 * locals.var_t1_dn4)) / (2.0 * assign56180_e92045)))), (0.5 * (locals.var_t1_dn5 + (((locals.var_t1_dn5 * assign56180_e92037) + (assign56180_e92034 * locals.var_t1_dn5)) / (2.0 * assign56180_e92045)))), (0.5 * (locals.var_t1_dn6 + (((locals.var_t1_dn6 * assign56180_e92037) + (assign56180_e92034 * locals.var_t1_dn6)) / (2.0 * assign56180_e92045)))), (0.5 * (locals.var_t1_dn7 + (((locals.var_t1_dn7 * assign56180_e92037) + (assign56180_e92034 * locals.var_t1_dn7)) / (2.0 * assign56180_e92045)))), (0.5 * (locals.var_t1_dn8 + (((locals.var_t1_dn8 * assign56180_e92037) + (assign56180_e92034 * locals.var_t1_dn8)) / (2.0 * assign56180_e92045)))), (0.5 * (locals.var_t1_dn9 + (((locals.var_t1_dn9 * assign56180_e92037) + (assign56180_e92034 * locals.var_t1_dn9)) / (2.0 * assign56180_e92045)))), (0.5 * (locals.var_t1_dn10 + (((locals.var_t1_dn10 * assign56180_e92037) + (assign56180_e92034 * locals.var_t1_dn10)) / (2.0 * assign56180_e92045)))), (0.5 * (locals.var_t1_dn11 + (((locals.var_t1_dn11 * assign56180_e92037) + (assign56180_e92034 * locals.var_t1_dn11)) / (2.0 * assign56180_e92045)))),)
    } else {
        (locals.var_qbd, locals.var_qbd_dn3, locals.var_qbd_dn4, locals.var_qbd_dn5, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn8, locals.var_qbd_dn9, locals.var_qbd_dn10, locals.var_qbd_dn11,)
    }
};
        locals.var_qbd = assign56180_e92049;
        locals.var_qbd_dn3 = assign56180_e92049_d_n3;
        locals.var_qbd_dn4 = assign56180_e92049_d_n4;
        locals.var_qbd_dn5 = assign56180_e92049_d_n5;
        locals.var_qbd_dn6 = assign56180_e92049_d_n6;
        locals.var_qbd_dn7 = assign56180_e92049_d_n7;
        locals.var_qbd_dn8 = assign56180_e92049_d_n8;
        locals.var_qbd_dn9 = assign56180_e92049_d_n9;
        locals.var_qbd_dn10 = assign56180_e92049_d_n10;
        locals.var_qbd_dn11 = assign56180_e92049_d_n11;

        let (assign56190_e92060, assign56190_e92060_d_n3, assign56190_e92060_d_n4, assign56190_e92060_d_n5, assign56190_e92060_d_n6, assign56190_e92060_d_n7, assign56190_e92060_d_n8, assign56190_e92060_d_n9, assign56190_e92060_d_n10, assign56190_e92060_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign56190_e92057: f64 = (locals.var_qbs + locals.var_qbd);
        let assign56190_e92058: f64 = (0.5 * assign56190_e92057);
        (assign56190_e92058, (0.5 * (locals.var_qbs_dn3 + locals.var_qbd_dn3)), (0.5 * (locals.var_qbs_dn4 + locals.var_qbd_dn4)), (0.5 * (locals.var_qbs_dn5 + locals.var_qbd_dn5)), (0.5 * (locals.var_qbs_dn6 + locals.var_qbd_dn6)), (0.5 * (locals.var_qbs_dn7 + locals.var_qbd_dn7)), (0.5 * (locals.var_qbs_dn8 + locals.var_qbd_dn8)), (0.5 * (locals.var_qbs_dn9 + locals.var_qbd_dn9)), (0.5 * (locals.var_qbs_dn10 + locals.var_qbd_dn10)), (0.5 * (locals.var_qbs_dn11 + locals.var_qbd_dn11)),)
    } else {
        (locals.var_qb, locals.var_qb_dn3, locals.var_qb_dn4, locals.var_qb_dn5, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn8, locals.var_qb_dn9, locals.var_qb_dn10, locals.var_qb_dn11,)
    }
};
        locals.var_qb = assign56190_e92060;
        locals.var_qb_dn3 = assign56190_e92060_d_n3;
        locals.var_qb_dn4 = assign56190_e92060_d_n4;
        locals.var_qb_dn5 = assign56190_e92060_d_n5;
        locals.var_qb_dn6 = assign56190_e92060_d_n6;
        locals.var_qb_dn7 = assign56190_e92060_d_n7;
        locals.var_qb_dn8 = assign56190_e92060_d_n8;
        locals.var_qb_dn9 = assign56190_e92060_d_n9;
        locals.var_qb_dn10 = assign56190_e92060_d_n10;
        locals.var_qb_dn11 = assign56190_e92060_d_n11;

        let (assign56200_e92073, assign56200_e92073_d_n3, assign56200_e92073_d_n4, assign56200_e92073_d_n5, assign56200_e92073_d_n6, assign56200_e92073_d_n7, assign56200_e92073_d_n8, assign56200_e92073_d_n9, assign56200_e92073_d_n10, assign56200_e92073_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign56200_e92067: f64 = (locals.var_nq * locals.var_vt);
        let assign56200_e92070: f64 = (locals.var_qs_1 + locals.var_qdeff);
        let assign56200_e92071: f64 = (assign56200_e92067 * assign56200_e92070);
        (assign56200_e92071, (((locals.var_nq_dn3 * locals.var_vt) * assign56200_e92070) + (assign56200_e92067 * (locals.var_qs_1_dn3 + locals.var_qdeff_dn3))), ((((locals.var_nq_dn4 * locals.var_vt) + (locals.var_nq * locals.var_vt_dn4)) * assign56200_e92070) + (assign56200_e92067 * (locals.var_qs_1_dn4 + locals.var_qdeff_dn4))), ((((locals.var_nq_dn5 * locals.var_vt) + (locals.var_nq * locals.var_vt_dn5)) * assign56200_e92070) + (assign56200_e92067 * (locals.var_qs_1_dn5 + locals.var_qdeff_dn5))), (((locals.var_nq_dn6 * locals.var_vt) * assign56200_e92070) + (assign56200_e92067 * (locals.var_qs_1_dn6 + locals.var_qdeff_dn6))), (((locals.var_nq_dn7 * locals.var_vt) * assign56200_e92070) + (assign56200_e92067 * (locals.var_qs_1_dn7 + locals.var_qdeff_dn7))), (((locals.var_nq_dn8 * locals.var_vt) * assign56200_e92070) + (assign56200_e92067 * (locals.var_qs_1_dn8 + locals.var_qdeff_dn8))), (((locals.var_nq_dn9 * locals.var_vt) * assign56200_e92070) + (assign56200_e92067 * (locals.var_qs_1_dn9 + locals.var_qdeff_dn9))), (((locals.var_nq_dn10 * locals.var_vt) * assign56200_e92070) + (assign56200_e92067 * (locals.var_qs_1_dn10 + locals.var_qdeff_dn10))), (((locals.var_nq_dn11 * locals.var_vt) * assign56200_e92070) + (assign56200_e92067 * (locals.var_qs_1_dn11 + locals.var_qdeff_dn11))),)
    } else {
        (locals.var_qia, locals.var_qia_dn3, locals.var_qia_dn4, locals.var_qia_dn5, locals.var_qia_dn6, locals.var_qia_dn7, locals.var_qia_dn8, locals.var_qia_dn9, locals.var_qia_dn10, locals.var_qia_dn11,)
    }
};
        locals.var_qia = assign56200_e92073;
        locals.var_qia_dn3 = assign56200_e92073_d_n3;
        locals.var_qia_dn4 = assign56200_e92073_d_n4;
        locals.var_qia_dn5 = assign56200_e92073_d_n5;
        locals.var_qia_dn6 = assign56200_e92073_d_n6;
        locals.var_qia_dn7 = assign56200_e92073_d_n7;
        locals.var_qia_dn8 = assign56200_e92073_d_n8;
        locals.var_qia_dn9 = assign56200_e92073_d_n9;
        locals.var_qia_dn10 = assign56200_e92073_d_n10;
        locals.var_qia_dn11 = assign56200_e92073_d_n11;

        let (assign56210_e92086, assign56210_e92086_d_n3, assign56210_e92086_d_n4, assign56210_e92086_d_n5, assign56210_e92086_d_n6, assign56210_e92086_d_n7, assign56210_e92086_d_n8, assign56210_e92086_d_n9, assign56210_e92086_d_n10, assign56210_e92086_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign56210_e92082: f64 = (locals.var_eta_mu * locals.var_qia);
        let assign56210_e92083: f64 = (locals.var_qb + assign56210_e92082);
        let assign56210_e92084: f64 = (locals.var_eefffactor * assign56210_e92083);
        (assign56210_e92084, (locals.var_eefffactor * (locals.var_qb_dn3 + (locals.var_eta_mu * locals.var_qia_dn3))), (locals.var_eefffactor * (locals.var_qb_dn4 + (locals.var_eta_mu * locals.var_qia_dn4))), (locals.var_eefffactor * (locals.var_qb_dn5 + (locals.var_eta_mu * locals.var_qia_dn5))), (locals.var_eefffactor * (locals.var_qb_dn6 + (locals.var_eta_mu * locals.var_qia_dn6))), (locals.var_eefffactor * (locals.var_qb_dn7 + (locals.var_eta_mu * locals.var_qia_dn7))), (locals.var_eefffactor * (locals.var_qb_dn8 + (locals.var_eta_mu * locals.var_qia_dn8))), (locals.var_eefffactor * (locals.var_qb_dn9 + (locals.var_eta_mu * locals.var_qia_dn9))), (locals.var_eefffactor * (locals.var_qb_dn10 + (locals.var_eta_mu * locals.var_qia_dn10))), (locals.var_eefffactor * (locals.var_qb_dn11 + (locals.var_eta_mu * locals.var_qia_dn11))),)
    } else {
        (locals.var_eeffm, locals.var_eeffm_dn3, locals.var_eeffm_dn4, locals.var_eeffm_dn5, locals.var_eeffm_dn6, locals.var_eeffm_dn7, locals.var_eeffm_dn8, locals.var_eeffm_dn9, locals.var_eeffm_dn10, locals.var_eeffm_dn11,)
    }
};
        locals.var_eeffm = assign56210_e92086;
        locals.var_eeffm_dn3 = assign56210_e92086_d_n3;
        locals.var_eeffm_dn4 = assign56210_e92086_d_n4;
        locals.var_eeffm_dn5 = assign56210_e92086_d_n5;
        locals.var_eeffm_dn6 = assign56210_e92086_d_n6;
        locals.var_eeffm_dn7 = assign56210_e92086_d_n7;
        locals.var_eeffm_dn8 = assign56210_e92086_d_n8;
        locals.var_eeffm_dn9 = assign56210_e92086_d_n9;
        locals.var_eeffm_dn10 = assign56210_e92086_d_n10;
        locals.var_eeffm_dn11 = assign56210_e92086_d_n11;

    }

    pub(super) fn stamp_transient_block_191(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign56220_e92095, assign56220_e92095_d_n3, assign56220_e92095_d_n4, assign56220_e92095_d_n5, assign56220_e92095_d_n6, assign56220_e92095_d_n7, assign56220_e92095_d_n8, assign56220_e92095_d_n9, assign56220_e92095_d_n10, assign56220_e92095_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign56220_e92093: f64 = (1.0 + locals.var_dpd);
        (assign56220_e92093, locals.var_dpd_dn3, locals.var_dpd_dn4, locals.var_dpd_dn5, locals.var_dpd_dn6, locals.var_dpd_dn7, locals.var_dpd_dn8, locals.var_dpd_dn9, locals.var_dpd_dn10, locals.var_dpd_dn11,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign56220_e92095;
        locals.var_t1_dn3 = assign56220_e92095_d_n3;
        locals.var_t1_dn4 = assign56220_e92095_d_n4;
        locals.var_t1_dn5 = assign56220_e92095_d_n5;
        locals.var_t1_dn6 = assign56220_e92095_d_n6;
        locals.var_t1_dn7 = assign56220_e92095_d_n7;
        locals.var_t1_dn8 = assign56220_e92095_d_n8;
        locals.var_t1_dn9 = assign56220_e92095_d_n9;
        locals.var_t1_dn10 = assign56220_e92095_d_n10;
        locals.var_t1_dn11 = assign56220_e92095_d_n11;

        let (assign56230_e92108, assign56230_e92108_d_n3, assign56230_e92108_d_n4, assign56230_e92108_d_n5, assign56230_e92108_d_n6, assign56230_e92108_d_n7, assign56230_e92108_d_n8, assign56230_e92108_d_n9, assign56230_e92108_d_n10, assign56230_e92108_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign56230_e92103: f64 = (p.p139 * locals.var_inv_vt);
        let assign56230_e92104: f64 = (locals.var_vgfbcv + assign56230_e92103);
        let assign56230_e92106: f64 = (assign56230_e92104 / locals.var_t1);
        (assign56230_e92106, (((locals.var_vgfbcv_dn3 * locals.var_t1) - (assign56230_e92104 * locals.var_t1_dn3)) / (locals.var_t1 * locals.var_t1)), ((((locals.var_vgfbcv_dn4 + (p.p139 * locals.var_inv_vt_dn4)) * locals.var_t1) - (assign56230_e92104 * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)), ((((locals.var_vgfbcv_dn5 + (p.p139 * locals.var_inv_vt_dn5)) * locals.var_t1) - (assign56230_e92104 * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfbcv_dn6 * locals.var_t1) - (assign56230_e92104 * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfbcv_dn7 * locals.var_t1) - (assign56230_e92104 * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfbcv_dn8 * locals.var_t1) - (assign56230_e92104 * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfbcv_dn9 * locals.var_t1) - (assign56230_e92104 * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfbcv_dn10 * locals.var_t1) - (assign56230_e92104 * locals.var_t1_dn10)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfbcv_dn11 * locals.var_t1) - (assign56230_e92104 * locals.var_t1_dn11)) / (locals.var_t1 * locals.var_t1)),)
    } else {
        (locals.var_vgfbpd, locals.var_vgfbpd_dn3, locals.var_vgfbpd_dn4, locals.var_vgfbpd_dn5, locals.var_vgfbpd_dn6, locals.var_vgfbpd_dn7, locals.var_vgfbpd_dn8, locals.var_vgfbpd_dn9, locals.var_vgfbpd_dn10, locals.var_vgfbpd_dn11,)
    }
};
        locals.var_vgfbpd = assign56230_e92108;
        locals.var_vgfbpd_dn3 = assign56230_e92108_d_n3;
        locals.var_vgfbpd_dn4 = assign56230_e92108_d_n4;
        locals.var_vgfbpd_dn5 = assign56230_e92108_d_n5;
        locals.var_vgfbpd_dn6 = assign56230_e92108_d_n6;
        locals.var_vgfbpd_dn7 = assign56230_e92108_d_n7;
        locals.var_vgfbpd_dn8 = assign56230_e92108_d_n8;
        locals.var_vgfbpd_dn9 = assign56230_e92108_d_n9;
        locals.var_vgfbpd_dn10 = assign56230_e92108_d_n10;
        locals.var_vgfbpd_dn11 = assign56230_e92108_d_n11;

        let (assign56240_e92117, assign56240_e92117_d_n3, assign56240_e92117_d_n4, assign56240_e92117_d_n5, assign56240_e92117_d_n6, assign56240_e92117_d_n7, assign56240_e92117_d_n8, assign56240_e92117_d_n9, assign56240_e92117_d_n10, assign56240_e92117_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign56240_e92115: f64 = (locals.var_gamcv / locals.var_t1);
        (assign56240_e92115, (((locals.var_gamcv_dn3 * locals.var_t1) - (locals.var_gamcv * locals.var_t1_dn3)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gamcv_dn4 * locals.var_t1) - (locals.var_gamcv * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gamcv_dn5 * locals.var_t1) - (locals.var_gamcv * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gamcv_dn6 * locals.var_t1) - (locals.var_gamcv * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gamcv_dn7 * locals.var_t1) - (locals.var_gamcv * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gamcv_dn8 * locals.var_t1) - (locals.var_gamcv * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gamcv_dn9 * locals.var_t1) - (locals.var_gamcv * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gamcv_dn10 * locals.var_t1) - (locals.var_gamcv * locals.var_t1_dn10)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gamcv_dn11 * locals.var_t1) - (locals.var_gamcv * locals.var_t1_dn11)) / (locals.var_t1 * locals.var_t1)),)
    } else {
        (locals.var_gammapd, locals.var_gammapd_dn3, locals.var_gammapd_dn4, locals.var_gammapd_dn5, locals.var_gammapd_dn6, locals.var_gammapd_dn7, locals.var_gammapd_dn8, locals.var_gammapd_dn9, locals.var_gammapd_dn10, locals.var_gammapd_dn11,)
    }
};
        locals.var_gammapd = assign56240_e92117;
        locals.var_gammapd_dn3 = assign56240_e92117_d_n3;
        locals.var_gammapd_dn4 = assign56240_e92117_d_n4;
        locals.var_gammapd_dn5 = assign56240_e92117_d_n5;
        locals.var_gammapd_dn6 = assign56240_e92117_d_n6;
        locals.var_gammapd_dn7 = assign56240_e92117_d_n7;
        locals.var_gammapd_dn8 = assign56240_e92117_d_n8;
        locals.var_gammapd_dn9 = assign56240_e92117_d_n9;
        locals.var_gammapd_dn10 = assign56240_e92117_d_n10;
        locals.var_gammapd_dn11 = assign56240_e92117_d_n11;

        let (assign56250_e92134, assign56250_e92134_d_n3, assign56250_e92134_d_n4, assign56250_e92134_d_n5, assign56250_e92134_d_n6, assign56250_e92134_d_n7, assign56250_e92134_d_n8, assign56250_e92134_d_n9, assign56250_e92134_d_n10, assign56250_e92134_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign56250_e92124: f64 = (0.5 * locals.var_vgfbpd);
        let assign56250_e92129: f64 = (locals.var_gammapd / 1.4142135623730951);
        let assign56250_e92130: f64 = (1.0 + assign56250_e92129);
        let assign56250_e92131: f64 = (3.0 * assign56250_e92130);
        let assign56250_e92132: f64 = (assign56250_e92124 - assign56250_e92131);
        (assign56250_e92132, ((0.5 * locals.var_vgfbpd_dn3) - (3.0 * (locals.var_gammapd_dn3 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn4) - (3.0 * (locals.var_gammapd_dn4 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn5) - (3.0 * (locals.var_gammapd_dn5 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn6) - (3.0 * (locals.var_gammapd_dn6 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn7) - (3.0 * (locals.var_gammapd_dn7 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn8) - (3.0 * (locals.var_gammapd_dn8 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn9) - (3.0 * (locals.var_gammapd_dn9 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn10) - (3.0 * (locals.var_gammapd_dn10 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn11) - (3.0 * (locals.var_gammapd_dn11 / 1.4142135623730951))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign56250_e92134;
        locals.var_t1_dn3 = assign56250_e92134_d_n3;
        locals.var_t1_dn4 = assign56250_e92134_d_n4;
        locals.var_t1_dn5 = assign56250_e92134_d_n5;
        locals.var_t1_dn6 = assign56250_e92134_d_n6;
        locals.var_t1_dn7 = assign56250_e92134_d_n7;
        locals.var_t1_dn8 = assign56250_e92134_d_n8;
        locals.var_t1_dn9 = assign56250_e92134_d_n9;
        locals.var_t1_dn10 = assign56250_e92134_d_n10;
        locals.var_t1_dn11 = assign56250_e92134_d_n11;

        let (assign56260_e92150, assign56260_e92150_d_n3, assign56260_e92150_d_n4, assign56260_e92150_d_n5, assign56260_e92150_d_n6, assign56260_e92150_d_n7, assign56260_e92150_d_n8, assign56260_e92150_d_n9, assign56260_e92150_d_n10, assign56260_e92150_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign56260_e92142: f64 = (locals.var_t1 * locals.var_t1);
        let assign56260_e92145: f64 = (6.0 * locals.var_vgfbpd);
        let assign56260_e92146: f64 = (assign56260_e92142 + assign56260_e92145);
        let assign56260_e92147: f64 = (assign56260_e92146).sqrt();
        let assign56260_e92148: f64 = (locals.var_t1 + assign56260_e92147);
        (assign56260_e92148, (locals.var_t1_dn3 + ((((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)) + (6.0 * locals.var_vgfbpd_dn3)) / (2.0 * assign56260_e92147))), (locals.var_t1_dn4 + ((((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) + (6.0 * locals.var_vgfbpd_dn4)) / (2.0 * assign56260_e92147))), (locals.var_t1_dn5 + ((((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) + (6.0 * locals.var_vgfbpd_dn5)) / (2.0 * assign56260_e92147))), (locals.var_t1_dn6 + ((((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) + (6.0 * locals.var_vgfbpd_dn6)) / (2.0 * assign56260_e92147))), (locals.var_t1_dn7 + ((((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) + (6.0 * locals.var_vgfbpd_dn7)) / (2.0 * assign56260_e92147))), (locals.var_t1_dn8 + ((((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) + (6.0 * locals.var_vgfbpd_dn8)) / (2.0 * assign56260_e92147))), (locals.var_t1_dn9 + ((((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) + (6.0 * locals.var_vgfbpd_dn9)) / (2.0 * assign56260_e92147))), (locals.var_t1_dn10 + ((((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) + (6.0 * locals.var_vgfbpd_dn10)) / (2.0 * assign56260_e92147))), (locals.var_t1_dn11 + ((((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) + (6.0 * locals.var_vgfbpd_dn11)) / (2.0 * assign56260_e92147))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign56260_e92150;
        locals.var_t2_dn3 = assign56260_e92150_d_n3;
        locals.var_t2_dn4 = assign56260_e92150_d_n4;
        locals.var_t2_dn5 = assign56260_e92150_d_n5;
        locals.var_t2_dn6 = assign56260_e92150_d_n6;
        locals.var_t2_dn7 = assign56260_e92150_d_n7;
        locals.var_t2_dn8 = assign56260_e92150_d_n8;
        locals.var_t2_dn9 = assign56260_e92150_d_n9;
        locals.var_t2_dn10 = assign56260_e92150_d_n10;
        locals.var_t2_dn11 = assign56260_e92150_d_n11;

        let assign56270_e92153: f64 = if locals.var_vgfbpd < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard851 = assign56270_e92153;

        let (assign56280_e92166, assign56280_e92166_d_n3, assign56280_e92166_d_n4, assign56280_e92166_d_n5, assign56280_e92166_d_n6, assign56280_e92166_d_n7, assign56280_e92166_d_n8, assign56280_e92166_d_n9, assign56280_e92166_d_n10, assign56280_e92166_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard851 != 0.0)) {
        let assign56280_e92162: f64 = (locals.var_vgfbpd - locals.var_t2);
        let assign56280_e92164: f64 = (assign56280_e92162 / locals.var_gammapd);
        (assign56280_e92164, ((((locals.var_vgfbpd_dn3 - locals.var_t2_dn3) * locals.var_gammapd) - (assign56280_e92162 * locals.var_gammapd_dn3)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn4 - locals.var_t2_dn4) * locals.var_gammapd) - (assign56280_e92162 * locals.var_gammapd_dn4)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn5 - locals.var_t2_dn5) * locals.var_gammapd) - (assign56280_e92162 * locals.var_gammapd_dn5)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn6 - locals.var_t2_dn6) * locals.var_gammapd) - (assign56280_e92162 * locals.var_gammapd_dn6)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn7 - locals.var_t2_dn7) * locals.var_gammapd) - (assign56280_e92162 * locals.var_gammapd_dn7)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn8 - locals.var_t2_dn8) * locals.var_gammapd) - (assign56280_e92162 * locals.var_gammapd_dn8)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn9 - locals.var_t2_dn9) * locals.var_gammapd) - (assign56280_e92162 * locals.var_gammapd_dn9)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn10 - locals.var_t2_dn10) * locals.var_gammapd) - (assign56280_e92162 * locals.var_gammapd_dn10)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn11 - locals.var_t2_dn11) * locals.var_gammapd) - (assign56280_e92162 * locals.var_gammapd_dn11)) / (locals.var_gammapd * locals.var_gammapd)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign56280_e92166;
        locals.var_t3_dn3 = assign56280_e92166_d_n3;
        locals.var_t3_dn4 = assign56280_e92166_d_n4;
        locals.var_t3_dn5 = assign56280_e92166_d_n5;
        locals.var_t3_dn6 = assign56280_e92166_d_n6;
        locals.var_t3_dn7 = assign56280_e92166_d_n7;
        locals.var_t3_dn8 = assign56280_e92166_d_n8;
        locals.var_t3_dn9 = assign56280_e92166_d_n9;
        locals.var_t3_dn10 = assign56280_e92166_d_n10;
        locals.var_t3_dn11 = assign56280_e92166_d_n11;

        let (assign56290_e92185, assign56290_e92185_d_n3, assign56290_e92185_d_n4, assign56290_e92185_d_n5, assign56290_e92185_d_n6, assign56290_e92185_d_n7, assign56290_e92185_d_n8, assign56290_e92185_d_n9, assign56290_e92185_d_n10, assign56290_e92185_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard851 != 0.0)) {
        let assign56290_e92175: f64 = (1.0 - locals.var_t2);
        let assign56290_e92178: f64 = (locals.var_t3 * locals.var_t3);
        let assign56290_e92179: f64 = (assign56290_e92175 + assign56290_e92178);
        let assign56290_e92181: f64 = (assign56290_e92179).max(1e-38);
        let assign56290_e92182: f64 = (assign56290_e92181).ln();
        let assign56290_e92183: f64 = (-assign56290_e92182);
        (assign56290_e92183, (-(if assign56290_e92179 >= 1e-38 { ((-locals.var_t2_dn3) + ((locals.var_t3_dn3 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn3))) } else { 0.0 } / assign56290_e92181)), (-(if assign56290_e92179 >= 1e-38 { ((-locals.var_t2_dn4) + ((locals.var_t3_dn4 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn4))) } else { 0.0 } / assign56290_e92181)), (-(if assign56290_e92179 >= 1e-38 { ((-locals.var_t2_dn5) + ((locals.var_t3_dn5 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn5))) } else { 0.0 } / assign56290_e92181)), (-(if assign56290_e92179 >= 1e-38 { ((-locals.var_t2_dn6) + ((locals.var_t3_dn6 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn6))) } else { 0.0 } / assign56290_e92181)), (-(if assign56290_e92179 >= 1e-38 { ((-locals.var_t2_dn7) + ((locals.var_t3_dn7 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn7))) } else { 0.0 } / assign56290_e92181)), (-(if assign56290_e92179 >= 1e-38 { ((-locals.var_t2_dn8) + ((locals.var_t3_dn8 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn8))) } else { 0.0 } / assign56290_e92181)), (-(if assign56290_e92179 >= 1e-38 { ((-locals.var_t2_dn9) + ((locals.var_t3_dn9 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn9))) } else { 0.0 } / assign56290_e92181)), (-(if assign56290_e92179 >= 1e-38 { ((-locals.var_t2_dn10) + ((locals.var_t3_dn10 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn10))) } else { 0.0 } / assign56290_e92181)), (-(if assign56290_e92179 >= 1e-38 { ((-locals.var_t2_dn11) + ((locals.var_t3_dn11 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn11))) } else { 0.0 } / assign56290_e92181)),)
    } else {
        (locals.var_psip, locals.var_psip_dn3, locals.var_psip_dn4, locals.var_psip_dn5, locals.var_psip_dn6, locals.var_psip_dn7, locals.var_psip_dn8, locals.var_psip_dn9, locals.var_psip_dn10, locals.var_psip_dn11,)
    }
};
        locals.var_psip = assign56290_e92185;
        locals.var_psip_dn3 = assign56290_e92185_d_n3;
        locals.var_psip_dn4 = assign56290_e92185_d_n4;
        locals.var_psip_dn5 = assign56290_e92185_d_n5;
        locals.var_psip_dn6 = assign56290_e92185_d_n6;
        locals.var_psip_dn7 = assign56290_e92185_d_n7;
        locals.var_psip_dn8 = assign56290_e92185_d_n8;
        locals.var_psip_dn9 = assign56290_e92185_d_n9;
        locals.var_psip_dn10 = assign56290_e92185_d_n10;
        locals.var_psip_dn11 = assign56290_e92185_d_n11;

        let (assign56300_e92197, assign56300_e92197_d_n3, assign56300_e92197_d_n4, assign56300_e92197_d_n5, assign56300_e92197_d_n6, assign56300_e92197_d_n7, assign56300_e92197_d_n8, assign56300_e92197_d_n9, assign56300_e92197_d_n10, assign56300_e92197_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard851 == 0.0)) {
        let assign56300_e92194: f64 = (-locals.var_t2);
        let assign56300_e92195: f64 = { let limited_exp_arg = assign56300_e92194; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign56300_e92195, ({ let limited_exp_arg = assign56300_e92194; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn3)), ({ let limited_exp_arg = assign56300_e92194; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn4)), ({ let limited_exp_arg = assign56300_e92194; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn5)), ({ let limited_exp_arg = assign56300_e92194; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn6)), ({ let limited_exp_arg = assign56300_e92194; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn7)), ({ let limited_exp_arg = assign56300_e92194; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn8)), ({ let limited_exp_arg = assign56300_e92194; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn9)), ({ let limited_exp_arg = assign56300_e92194; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn10)), ({ let limited_exp_arg = assign56300_e92194; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn11)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign56300_e92197;
        locals.var_t3_dn3 = assign56300_e92197_d_n3;
        locals.var_t3_dn4 = assign56300_e92197_d_n4;
        locals.var_t3_dn5 = assign56300_e92197_d_n5;
        locals.var_t3_dn6 = assign56300_e92197_d_n6;
        locals.var_t3_dn7 = assign56300_e92197_d_n7;
        locals.var_t3_dn8 = assign56300_e92197_d_n8;
        locals.var_t3_dn9 = assign56300_e92197_d_n9;
        locals.var_t3_dn10 = assign56300_e92197_d_n10;
        locals.var_t3_dn11 = assign56300_e92197_d_n11;

        let (assign56310_e92209, assign56310_e92209_d_n3, assign56310_e92209_d_n4, assign56310_e92209_d_n5, assign56310_e92209_d_n6, assign56310_e92209_d_n7, assign56310_e92209_d_n8, assign56310_e92209_d_n9, assign56310_e92209_d_n10, assign56310_e92209_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard851 == 0.0)) {
        let assign56310_e92207: f64 = (0.5 * locals.var_gammapd);
        (assign56310_e92207, (0.5 * locals.var_gammapd_dn3), (0.5 * locals.var_gammapd_dn4), (0.5 * locals.var_gammapd_dn5), (0.5 * locals.var_gammapd_dn6), (0.5 * locals.var_gammapd_dn7), (0.5 * locals.var_gammapd_dn8), (0.5 * locals.var_gammapd_dn9), (0.5 * locals.var_gammapd_dn10), (0.5 * locals.var_gammapd_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign56310_e92209;
        locals.var_t1_dn3 = assign56310_e92209_d_n3;
        locals.var_t1_dn4 = assign56310_e92209_d_n4;
        locals.var_t1_dn5 = assign56310_e92209_d_n5;
        locals.var_t1_dn6 = assign56310_e92209_d_n6;
        locals.var_t1_dn7 = assign56310_e92209_d_n7;
        locals.var_t1_dn8 = assign56310_e92209_d_n8;
        locals.var_t1_dn9 = assign56310_e92209_d_n9;
        locals.var_t1_dn10 = assign56310_e92209_d_n10;
        locals.var_t1_dn11 = assign56310_e92209_d_n11;

        let (assign56320_e92230, assign56320_e92230_d_n3, assign56320_e92230_d_n4, assign56320_e92230_d_n5, assign56320_e92230_d_n6, assign56320_e92230_d_n7, assign56320_e92230_d_n8, assign56320_e92230_d_n9, assign56320_e92230_d_n10, assign56320_e92230_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard851 == 0.0)) {
        let assign56320_e92219: f64 = (locals.var_vgfbpd - 1.0);
        let assign56320_e92221: f64 = (assign56320_e92219 + locals.var_t3);
        let assign56320_e92224: f64 = (locals.var_t1 * locals.var_t1);
        let assign56320_e92225: f64 = (assign56320_e92221 + assign56320_e92224);
        let assign56320_e92226: f64 = (assign56320_e92225).sqrt();
        let assign56320_e92228: f64 = (assign56320_e92226 - locals.var_t1);
        (assign56320_e92228, ((((locals.var_vgfbpd_dn3 + locals.var_t3_dn3) + ((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3))) / (2.0 * assign56320_e92226)) - locals.var_t1_dn3), ((((locals.var_vgfbpd_dn4 + locals.var_t3_dn4) + ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4))) / (2.0 * assign56320_e92226)) - locals.var_t1_dn4), ((((locals.var_vgfbpd_dn5 + locals.var_t3_dn5) + ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5))) / (2.0 * assign56320_e92226)) - locals.var_t1_dn5), ((((locals.var_vgfbpd_dn6 + locals.var_t3_dn6) + ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6))) / (2.0 * assign56320_e92226)) - locals.var_t1_dn6), ((((locals.var_vgfbpd_dn7 + locals.var_t3_dn7) + ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7))) / (2.0 * assign56320_e92226)) - locals.var_t1_dn7), ((((locals.var_vgfbpd_dn8 + locals.var_t3_dn8) + ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8))) / (2.0 * assign56320_e92226)) - locals.var_t1_dn8), ((((locals.var_vgfbpd_dn9 + locals.var_t3_dn9) + ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9))) / (2.0 * assign56320_e92226)) - locals.var_t1_dn9), ((((locals.var_vgfbpd_dn10 + locals.var_t3_dn10) + ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10))) / (2.0 * assign56320_e92226)) - locals.var_t1_dn10), ((((locals.var_vgfbpd_dn11 + locals.var_t3_dn11) + ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11))) / (2.0 * assign56320_e92226)) - locals.var_t1_dn11),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign56320_e92230;
        locals.var_t2_dn3 = assign56320_e92230_d_n3;
        locals.var_t2_dn4 = assign56320_e92230_d_n4;
        locals.var_t2_dn5 = assign56320_e92230_d_n5;
        locals.var_t2_dn6 = assign56320_e92230_d_n6;
        locals.var_t2_dn7 = assign56320_e92230_d_n7;
        locals.var_t2_dn8 = assign56320_e92230_d_n8;
        locals.var_t2_dn9 = assign56320_e92230_d_n9;
        locals.var_t2_dn10 = assign56320_e92230_d_n10;
        locals.var_t2_dn11 = assign56320_e92230_d_n11;

        let (assign56330_e92246, assign56330_e92246_d_n3, assign56330_e92246_d_n4, assign56330_e92246_d_n5, assign56330_e92246_d_n6, assign56330_e92246_d_n7, assign56330_e92246_d_n8, assign56330_e92246_d_n9, assign56330_e92246_d_n10, assign56330_e92246_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) && (locals.var_guard851 == 0.0)) {
        let assign56330_e92240: f64 = (locals.var_t2 * locals.var_t2);
        let assign56330_e92242: f64 = (assign56330_e92240 + 1.0);
        let assign56330_e92244: f64 = (assign56330_e92242 - locals.var_t3);
        (assign56330_e92244, (((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)) - locals.var_t3_dn3), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) - locals.var_t3_dn4), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) - locals.var_t3_dn5), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) - locals.var_t3_dn6), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) - locals.var_t3_dn7), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) - locals.var_t3_dn8), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) - locals.var_t3_dn9), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) - locals.var_t3_dn10), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) - locals.var_t3_dn11),)
    } else {
        (locals.var_psip, locals.var_psip_dn3, locals.var_psip_dn4, locals.var_psip_dn5, locals.var_psip_dn6, locals.var_psip_dn7, locals.var_psip_dn8, locals.var_psip_dn9, locals.var_psip_dn10, locals.var_psip_dn11,)
    }
};
        locals.var_psip = assign56330_e92246;
        locals.var_psip_dn3 = assign56330_e92246_d_n3;
        locals.var_psip_dn4 = assign56330_e92246_d_n4;
        locals.var_psip_dn5 = assign56330_e92246_d_n5;
        locals.var_psip_dn6 = assign56330_e92246_d_n6;
        locals.var_psip_dn7 = assign56330_e92246_d_n7;
        locals.var_psip_dn8 = assign56330_e92246_d_n8;
        locals.var_psip_dn9 = assign56330_e92246_d_n9;
        locals.var_psip_dn10 = assign56330_e92246_d_n10;
        locals.var_psip_dn11 = assign56330_e92246_d_n11;

        let (assign56340_e92261, assign56340_e92261_d_n3, assign56340_e92261_d_n4, assign56340_e92261_d_n5, assign56340_e92261_d_n6, assign56340_e92261_d_n7, assign56340_e92261_d_n8, assign56340_e92261_d_n9, assign56340_e92261_d_n10, assign56340_e92261_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign56340_e92254: f64 = (locals.var_uc_a * locals.var_vbsx);
        let assign56340_e92255: f64 = (locals.var_ua_a + assign56340_e92254);
        let assign56340_e92258: f64 = (locals.var_eeffm).powf(locals.var_eu_t);
        let assign56340_e92259: f64 = (assign56340_e92255 * assign56340_e92258);
        (assign56340_e92259, (((locals.var_ua_a_dn3 + ((locals.var_uc_a_dn3 * locals.var_vbsx) + (locals.var_uc_a * locals.var_vbsx_dn3))) * assign56340_e92258) + (assign56340_e92255 * if locals.var_eu_t_dn3 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffm).powf(locals.var_eu_t - 1.0) * locals.var_eeffm_dn3)) } } else { (assign56340_e92258 * ((locals.var_eu_t_dn3 * (locals.var_eeffm).ln()) + (locals.var_eu_t * (locals.var_eeffm_dn3 / locals.var_eeffm)))) })), (((locals.var_ua_a_dn4 + ((locals.var_uc_a_dn4 * locals.var_vbsx) + (locals.var_uc_a * locals.var_vbsx_dn4))) * assign56340_e92258) + (assign56340_e92255 * if locals.var_eu_t_dn4 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffm).powf(locals.var_eu_t - 1.0) * locals.var_eeffm_dn4)) } } else { (assign56340_e92258 * ((locals.var_eu_t_dn4 * (locals.var_eeffm).ln()) + (locals.var_eu_t * (locals.var_eeffm_dn4 / locals.var_eeffm)))) })), (((locals.var_ua_a_dn5 + ((locals.var_uc_a_dn5 * locals.var_vbsx) + (locals.var_uc_a * locals.var_vbsx_dn5))) * assign56340_e92258) + (assign56340_e92255 * if locals.var_eu_t_dn5 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffm).powf(locals.var_eu_t - 1.0) * locals.var_eeffm_dn5)) } } else { (assign56340_e92258 * ((locals.var_eu_t_dn5 * (locals.var_eeffm).ln()) + (locals.var_eu_t * (locals.var_eeffm_dn5 / locals.var_eeffm)))) })), (((locals.var_ua_a_dn6 + ((locals.var_uc_a_dn6 * locals.var_vbsx) + (locals.var_uc_a * locals.var_vbsx_dn6))) * assign56340_e92258) + (assign56340_e92255 * if locals.var_eu_t_dn6 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffm).powf(locals.var_eu_t - 1.0) * locals.var_eeffm_dn6)) } } else { (assign56340_e92258 * ((locals.var_eu_t_dn6 * (locals.var_eeffm).ln()) + (locals.var_eu_t * (locals.var_eeffm_dn6 / locals.var_eeffm)))) })), (((locals.var_ua_a_dn7 + ((locals.var_uc_a_dn7 * locals.var_vbsx) + (locals.var_uc_a * locals.var_vbsx_dn7))) * assign56340_e92258) + (assign56340_e92255 * if locals.var_eu_t_dn7 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffm).powf(locals.var_eu_t - 1.0) * locals.var_eeffm_dn7)) } } else { (assign56340_e92258 * ((locals.var_eu_t_dn7 * (locals.var_eeffm).ln()) + (locals.var_eu_t * (locals.var_eeffm_dn7 / locals.var_eeffm)))) })), (((locals.var_ua_a_dn8 + ((locals.var_uc_a_dn8 * locals.var_vbsx) + (locals.var_uc_a * locals.var_vbsx_dn8))) * assign56340_e92258) + (assign56340_e92255 * if locals.var_eu_t_dn8 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffm).powf(locals.var_eu_t - 1.0) * locals.var_eeffm_dn8)) } } else { (assign56340_e92258 * ((locals.var_eu_t_dn8 * (locals.var_eeffm).ln()) + (locals.var_eu_t * (locals.var_eeffm_dn8 / locals.var_eeffm)))) })), (((locals.var_ua_a_dn9 + ((locals.var_uc_a_dn9 * locals.var_vbsx) + (locals.var_uc_a * locals.var_vbsx_dn9))) * assign56340_e92258) + (assign56340_e92255 * if locals.var_eu_t_dn9 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffm).powf(locals.var_eu_t - 1.0) * locals.var_eeffm_dn9)) } } else { (assign56340_e92258 * ((locals.var_eu_t_dn9 * (locals.var_eeffm).ln()) + (locals.var_eu_t * (locals.var_eeffm_dn9 / locals.var_eeffm)))) })), (((locals.var_ua_a_dn10 + ((locals.var_uc_a_dn10 * locals.var_vbsx) + (locals.var_uc_a * locals.var_vbsx_dn10))) * assign56340_e92258) + (assign56340_e92255 * if locals.var_eu_t_dn10 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffm).powf(locals.var_eu_t - 1.0) * locals.var_eeffm_dn10)) } } else { (assign56340_e92258 * ((locals.var_eu_t_dn10 * (locals.var_eeffm).ln()) + (locals.var_eu_t * (locals.var_eeffm_dn10 / locals.var_eeffm)))) })), (((locals.var_ua_a_dn11 + ((locals.var_uc_a_dn11 * locals.var_vbsx) + (locals.var_uc_a * locals.var_vbsx_dn11))) * assign56340_e92258) + (assign56340_e92255 * if locals.var_eu_t_dn11 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffm).powf(locals.var_eu_t - 1.0) * locals.var_eeffm_dn11)) } } else { (assign56340_e92258 * ((locals.var_eu_t_dn11 * (locals.var_eeffm).ln()) + (locals.var_eu_t * (locals.var_eeffm_dn11 / locals.var_eeffm)))) })),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign56340_e92261;
        locals.var_t3_dn3 = assign56340_e92261_d_n3;
        locals.var_t3_dn4 = assign56340_e92261_d_n4;
        locals.var_t3_dn5 = assign56340_e92261_d_n5;
        locals.var_t3_dn6 = assign56340_e92261_d_n6;
        locals.var_t3_dn7 = assign56340_e92261_d_n7;
        locals.var_t3_dn8 = assign56340_e92261_d_n8;
        locals.var_t3_dn9 = assign56340_e92261_d_n9;
        locals.var_t3_dn10 = assign56340_e92261_d_n10;
        locals.var_t3_dn11 = assign56340_e92261_d_n11;

        let (assign56350_e92270, assign56350_e92270_d_n3, assign56350_e92270_d_n4, assign56350_e92270_d_n5, assign56350_e92270_d_n6, assign56350_e92270_d_n7, assign56350_e92270_d_n8, assign56350_e92270_d_n9, assign56350_e92270_d_n10, assign56350_e92270_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign56350_e92268: f64 = (1.0 + locals.var_t3);
        (assign56350_e92268, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign56350_e92270;
        locals.var_t4_dn3 = assign56350_e92270_d_n3;
        locals.var_t4_dn4 = assign56350_e92270_d_n4;
        locals.var_t4_dn5 = assign56350_e92270_d_n5;
        locals.var_t4_dn6 = assign56350_e92270_d_n6;
        locals.var_t4_dn7 = assign56350_e92270_d_n7;
        locals.var_t4_dn8 = assign56350_e92270_d_n8;
        locals.var_t4_dn9 = assign56350_e92270_d_n9;
        locals.var_t4_dn10 = assign56350_e92270_d_n10;
        locals.var_t4_dn11 = assign56350_e92270_d_n11;

        let (assign56360_e92296, assign56360_e92296_d_n3, assign56360_e92296_d_n4, assign56360_e92296_d_n5, assign56360_e92296_d_n6, assign56360_e92296_d_n7, assign56360_e92296_d_n8, assign56360_e92296_d_n9, assign56360_e92296_d_n10, assign56360_e92296_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign56360_e92278: f64 = (locals.var_t4 + 1.0);
        let assign56360_e92281: f64 = (locals.var_t4 - 1.0);
        let assign56360_e92284: f64 = (locals.var_t4 - 1.0);
        let assign56360_e92285: f64 = (assign56360_e92281 * assign56360_e92284);
        let assign56360_e92288: f64 = (0.25 * 0.0015);
        let assign56360_e92290: f64 = (assign56360_e92288 * 0.0015);
        let assign56360_e92291: f64 = (assign56360_e92285 + assign56360_e92290);
        let assign56360_e92292: f64 = (assign56360_e92291).sqrt();
        let assign56360_e92293: f64 = (assign56360_e92278 + assign56360_e92292);
        let assign56360_e92294: f64 = (0.5 * assign56360_e92293);
        (assign56360_e92294, (0.5 * (locals.var_t4_dn3 + (((locals.var_t4_dn3 * assign56360_e92284) + (assign56360_e92281 * locals.var_t4_dn3)) / (2.0 * assign56360_e92292)))), (0.5 * (locals.var_t4_dn4 + (((locals.var_t4_dn4 * assign56360_e92284) + (assign56360_e92281 * locals.var_t4_dn4)) / (2.0 * assign56360_e92292)))), (0.5 * (locals.var_t4_dn5 + (((locals.var_t4_dn5 * assign56360_e92284) + (assign56360_e92281 * locals.var_t4_dn5)) / (2.0 * assign56360_e92292)))), (0.5 * (locals.var_t4_dn6 + (((locals.var_t4_dn6 * assign56360_e92284) + (assign56360_e92281 * locals.var_t4_dn6)) / (2.0 * assign56360_e92292)))), (0.5 * (locals.var_t4_dn7 + (((locals.var_t4_dn7 * assign56360_e92284) + (assign56360_e92281 * locals.var_t4_dn7)) / (2.0 * assign56360_e92292)))), (0.5 * (locals.var_t4_dn8 + (((locals.var_t4_dn8 * assign56360_e92284) + (assign56360_e92281 * locals.var_t4_dn8)) / (2.0 * assign56360_e92292)))), (0.5 * (locals.var_t4_dn9 + (((locals.var_t4_dn9 * assign56360_e92284) + (assign56360_e92281 * locals.var_t4_dn9)) / (2.0 * assign56360_e92292)))), (0.5 * (locals.var_t4_dn10 + (((locals.var_t4_dn10 * assign56360_e92284) + (assign56360_e92281 * locals.var_t4_dn10)) / (2.0 * assign56360_e92292)))), (0.5 * (locals.var_t4_dn11 + (((locals.var_t4_dn11 * assign56360_e92284) + (assign56360_e92281 * locals.var_t4_dn11)) / (2.0 * assign56360_e92292)))),)
    } else {
        (locals.var_dmob, locals.var_dmob_dn3, locals.var_dmob_dn4, locals.var_dmob_dn5, locals.var_dmob_dn6, locals.var_dmob_dn7, locals.var_dmob_dn8, locals.var_dmob_dn9, locals.var_dmob_dn10, locals.var_dmob_dn11,)
    }
};
        locals.var_dmob = assign56360_e92296;
        locals.var_dmob_dn3 = assign56360_e92296_d_n3;
        locals.var_dmob_dn4 = assign56360_e92296_d_n4;
        locals.var_dmob_dn5 = assign56360_e92296_d_n5;
        locals.var_dmob_dn6 = assign56360_e92296_d_n6;
        locals.var_dmob_dn7 = assign56360_e92296_d_n7;
        locals.var_dmob_dn8 = assign56360_e92296_d_n8;
        locals.var_dmob_dn9 = assign56360_e92296_d_n9;
        locals.var_dmob_dn10 = assign56360_e92296_d_n10;
        locals.var_dmob_dn11 = assign56360_e92296_d_n11;

        let (assign56370_e92313, assign56370_e92313_d_n3, assign56370_e92313_d_n4, assign56370_e92313_d_n5, assign56370_e92313_d_n6, assign56370_e92313_d_n7, assign56370_e92313_d_n8, assign56370_e92313_d_n9, assign56370_e92313_d_n10, assign56370_e92313_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign56370_e92304: f64 = (locals.var_u0_a / locals.var_dmob);
        let assign56370_e92305: f64 = (2.0 * assign56370_e92304);
        let assign56370_e92307: f64 = (assign56370_e92305 * locals.var_vt);
        let assign56370_e92310: f64 = (locals.var_vsatcv_t * locals.var_lact);
        let assign56370_e92311: f64 = (assign56370_e92307 / assign56370_e92310);
        (assign56370_e92311, (((((2.0 * (((locals.var_u0_a_dn3 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn3)) / (locals.var_dmob * locals.var_dmob))) * locals.var_vt) * assign56370_e92310) - (assign56370_e92307 * (locals.var_vsatcv_t_dn3 * locals.var_lact))) / (assign56370_e92310 * assign56370_e92310)), ((((((2.0 * (((locals.var_u0_a_dn4 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn4)) / (locals.var_dmob * locals.var_dmob))) * locals.var_vt) + (assign56370_e92305 * locals.var_vt_dn4)) * assign56370_e92310) - (assign56370_e92307 * (locals.var_vsatcv_t_dn4 * locals.var_lact))) / (assign56370_e92310 * assign56370_e92310)), ((((((2.0 * (((locals.var_u0_a_dn5 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn5)) / (locals.var_dmob * locals.var_dmob))) * locals.var_vt) + (assign56370_e92305 * locals.var_vt_dn5)) * assign56370_e92310) - (assign56370_e92307 * (locals.var_vsatcv_t_dn5 * locals.var_lact))) / (assign56370_e92310 * assign56370_e92310)), (((((2.0 * (((locals.var_u0_a_dn6 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn6)) / (locals.var_dmob * locals.var_dmob))) * locals.var_vt) * assign56370_e92310) - (assign56370_e92307 * (locals.var_vsatcv_t_dn6 * locals.var_lact))) / (assign56370_e92310 * assign56370_e92310)), (((((2.0 * (((locals.var_u0_a_dn7 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn7)) / (locals.var_dmob * locals.var_dmob))) * locals.var_vt) * assign56370_e92310) - (assign56370_e92307 * (locals.var_vsatcv_t_dn7 * locals.var_lact))) / (assign56370_e92310 * assign56370_e92310)), (((((2.0 * (((locals.var_u0_a_dn8 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn8)) / (locals.var_dmob * locals.var_dmob))) * locals.var_vt) * assign56370_e92310) - (assign56370_e92307 * (locals.var_vsatcv_t_dn8 * locals.var_lact))) / (assign56370_e92310 * assign56370_e92310)), (((((2.0 * (((locals.var_u0_a_dn9 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn9)) / (locals.var_dmob * locals.var_dmob))) * locals.var_vt) * assign56370_e92310) - (assign56370_e92307 * (locals.var_vsatcv_t_dn9 * locals.var_lact))) / (assign56370_e92310 * assign56370_e92310)), (((((2.0 * (((locals.var_u0_a_dn10 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn10)) / (locals.var_dmob * locals.var_dmob))) * locals.var_vt) * assign56370_e92310) - (assign56370_e92307 * (locals.var_vsatcv_t_dn10 * locals.var_lact))) / (assign56370_e92310 * assign56370_e92310)), (((((2.0 * (((locals.var_u0_a_dn11 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn11)) / (locals.var_dmob * locals.var_dmob))) * locals.var_vt) * assign56370_e92310) - (assign56370_e92307 * (locals.var_vsatcv_t_dn11 * locals.var_lact))) / (assign56370_e92310 * assign56370_e92310)),)
    } else {
        (locals.var_lambdac, locals.var_lambdac_dn3, locals.var_lambdac_dn4, locals.var_lambdac_dn5, locals.var_lambdac_dn6, locals.var_lambdac_dn7, locals.var_lambdac_dn8, locals.var_lambdac_dn9, locals.var_lambdac_dn10, locals.var_lambdac_dn11,)
    }
};
        locals.var_lambdac = assign56370_e92313;
        locals.var_lambdac_dn3 = assign56370_e92313_d_n3;
        locals.var_lambdac_dn4 = assign56370_e92313_d_n4;
        locals.var_lambdac_dn5 = assign56370_e92313_d_n5;
        locals.var_lambdac_dn6 = assign56370_e92313_d_n6;
        locals.var_lambdac_dn7 = assign56370_e92313_d_n7;
        locals.var_lambdac_dn8 = assign56370_e92313_d_n8;
        locals.var_lambdac_dn9 = assign56370_e92313_d_n9;
        locals.var_lambdac_dn10 = assign56370_e92313_d_n10;
        locals.var_lambdac_dn11 = assign56370_e92313_d_n11;

        let (assign56380_e92322, assign56380_e92322_d_n3, assign56380_e92322_d_n4, assign56380_e92322_d_n5, assign56380_e92322_d_n6, assign56380_e92322_d_n7, assign56380_e92322_d_n8, assign56380_e92322_d_n9, assign56380_e92322_d_n10, assign56380_e92322_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign56380_e92320: f64 = (locals.var_qs_1 - locals.var_qdeff);
        (assign56380_e92320, (locals.var_qs_1_dn3 - locals.var_qdeff_dn3), (locals.var_qs_1_dn4 - locals.var_qdeff_dn4), (locals.var_qs_1_dn5 - locals.var_qdeff_dn5), (locals.var_qs_1_dn6 - locals.var_qdeff_dn6), (locals.var_qs_1_dn7 - locals.var_qdeff_dn7), (locals.var_qs_1_dn8 - locals.var_qdeff_dn8), (locals.var_qs_1_dn9 - locals.var_qdeff_dn9), (locals.var_qs_1_dn10 - locals.var_qdeff_dn10), (locals.var_qs_1_dn11 - locals.var_qdeff_dn11),)
    } else {
        (locals.var_dps, locals.var_dps_dn3, locals.var_dps_dn4, locals.var_dps_dn5, locals.var_dps_dn6, locals.var_dps_dn7, locals.var_dps_dn8, locals.var_dps_dn9, locals.var_dps_dn10, locals.var_dps_dn11,)
    }
};
        locals.var_dps = assign56380_e92322;
        locals.var_dps_dn3 = assign56380_e92322_d_n3;
        locals.var_dps_dn4 = assign56380_e92322_d_n4;
        locals.var_dps_dn5 = assign56380_e92322_d_n5;
        locals.var_dps_dn6 = assign56380_e92322_d_n6;
        locals.var_dps_dn7 = assign56380_e92322_d_n7;
        locals.var_dps_dn8 = assign56380_e92322_d_n8;
        locals.var_dps_dn9 = assign56380_e92322_d_n9;
        locals.var_dps_dn10 = assign56380_e92322_d_n10;
        locals.var_dps_dn11 = assign56380_e92322_d_n11;

        let (assign56390_e92337, assign56390_e92337_d_n3, assign56390_e92337_d_n4, assign56390_e92337_d_n5, assign56390_e92337_d_n6, assign56390_e92337_d_n7, assign56390_e92337_d_n8, assign56390_e92337_d_n9, assign56390_e92337_d_n10, assign56390_e92337_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign56390_e92330: f64 = (locals.var_lambdac * locals.var_dps);
        let assign56390_e92331: f64 = (2.0 * assign56390_e92330);
        let assign56390_e92334: f64 = (locals.var_lambdac * locals.var_dps);
        let assign56390_e92335: f64 = (assign56390_e92331 * assign56390_e92334);
        (assign56390_e92335, (((2.0 * ((locals.var_lambdac_dn3 * locals.var_dps) + (locals.var_lambdac * locals.var_dps_dn3))) * assign56390_e92334) + (assign56390_e92331 * ((locals.var_lambdac_dn3 * locals.var_dps) + (locals.var_lambdac * locals.var_dps_dn3)))), (((2.0 * ((locals.var_lambdac_dn4 * locals.var_dps) + (locals.var_lambdac * locals.var_dps_dn4))) * assign56390_e92334) + (assign56390_e92331 * ((locals.var_lambdac_dn4 * locals.var_dps) + (locals.var_lambdac * locals.var_dps_dn4)))), (((2.0 * ((locals.var_lambdac_dn5 * locals.var_dps) + (locals.var_lambdac * locals.var_dps_dn5))) * assign56390_e92334) + (assign56390_e92331 * ((locals.var_lambdac_dn5 * locals.var_dps) + (locals.var_lambdac * locals.var_dps_dn5)))), (((2.0 * ((locals.var_lambdac_dn6 * locals.var_dps) + (locals.var_lambdac * locals.var_dps_dn6))) * assign56390_e92334) + (assign56390_e92331 * ((locals.var_lambdac_dn6 * locals.var_dps) + (locals.var_lambdac * locals.var_dps_dn6)))), (((2.0 * ((locals.var_lambdac_dn7 * locals.var_dps) + (locals.var_lambdac * locals.var_dps_dn7))) * assign56390_e92334) + (assign56390_e92331 * ((locals.var_lambdac_dn7 * locals.var_dps) + (locals.var_lambdac * locals.var_dps_dn7)))), (((2.0 * ((locals.var_lambdac_dn8 * locals.var_dps) + (locals.var_lambdac * locals.var_dps_dn8))) * assign56390_e92334) + (assign56390_e92331 * ((locals.var_lambdac_dn8 * locals.var_dps) + (locals.var_lambdac * locals.var_dps_dn8)))), (((2.0 * ((locals.var_lambdac_dn9 * locals.var_dps) + (locals.var_lambdac * locals.var_dps_dn9))) * assign56390_e92334) + (assign56390_e92331 * ((locals.var_lambdac_dn9 * locals.var_dps) + (locals.var_lambdac * locals.var_dps_dn9)))), (((2.0 * ((locals.var_lambdac_dn10 * locals.var_dps) + (locals.var_lambdac * locals.var_dps_dn10))) * assign56390_e92334) + (assign56390_e92331 * ((locals.var_lambdac_dn10 * locals.var_dps) + (locals.var_lambdac * locals.var_dps_dn10)))), (((2.0 * ((locals.var_lambdac_dn11 * locals.var_dps) + (locals.var_lambdac * locals.var_dps_dn11))) * assign56390_e92334) + (assign56390_e92331 * ((locals.var_lambdac_dn11 * locals.var_dps) + (locals.var_lambdac * locals.var_dps_dn11)))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign56390_e92337;
        locals.var_t1_dn3 = assign56390_e92337_d_n3;
        locals.var_t1_dn4 = assign56390_e92337_d_n4;
        locals.var_t1_dn5 = assign56390_e92337_d_n5;
        locals.var_t1_dn6 = assign56390_e92337_d_n6;
        locals.var_t1_dn7 = assign56390_e92337_d_n7;
        locals.var_t1_dn8 = assign56390_e92337_d_n8;
        locals.var_t1_dn9 = assign56390_e92337_d_n9;
        locals.var_t1_dn10 = assign56390_e92337_d_n10;
        locals.var_t1_dn11 = assign56390_e92337_d_n11;

        let (assign56400_e92347, assign56400_e92347_d_n3, assign56400_e92347_d_n4, assign56400_e92347_d_n5, assign56400_e92347_d_n6, assign56400_e92347_d_n7, assign56400_e92347_d_n8, assign56400_e92347_d_n9, assign56400_e92347_d_n10, assign56400_e92347_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign56400_e92344: f64 = (1.0 + locals.var_t1);
        let assign56400_e92345: f64 = (assign56400_e92344).sqrt();
        (assign56400_e92345, (locals.var_t1_dn3 / (2.0 * assign56400_e92345)), (locals.var_t1_dn4 / (2.0 * assign56400_e92345)), (locals.var_t1_dn5 / (2.0 * assign56400_e92345)), (locals.var_t1_dn6 / (2.0 * assign56400_e92345)), (locals.var_t1_dn7 / (2.0 * assign56400_e92345)), (locals.var_t1_dn8 / (2.0 * assign56400_e92345)), (locals.var_t1_dn9 / (2.0 * assign56400_e92345)), (locals.var_t1_dn10 / (2.0 * assign56400_e92345)), (locals.var_t1_dn11 / (2.0 * assign56400_e92345)),)
    } else {
        (locals.var_zsat, locals.var_zsat_dn3, locals.var_zsat_dn4, locals.var_zsat_dn5, locals.var_zsat_dn6, locals.var_zsat_dn7, locals.var_zsat_dn8, locals.var_zsat_dn9, locals.var_zsat_dn10, locals.var_zsat_dn11,)
    }
};
        locals.var_zsat = assign56400_e92347;
        locals.var_zsat_dn3 = assign56400_e92347_d_n3;
        locals.var_zsat_dn4 = assign56400_e92347_d_n4;
        locals.var_zsat_dn5 = assign56400_e92347_d_n5;
        locals.var_zsat_dn6 = assign56400_e92347_d_n6;
        locals.var_zsat_dn7 = assign56400_e92347_d_n7;
        locals.var_zsat_dn8 = assign56400_e92347_d_n8;
        locals.var_zsat_dn9 = assign56400_e92347_d_n9;
        locals.var_zsat_dn10 = assign56400_e92347_d_n10;
        locals.var_zsat_dn11 = assign56400_e92347_d_n11;

        let (assign56410_e92358, assign56410_e92358_d_n3, assign56410_e92358_d_n4, assign56410_e92358_d_n5, assign56410_e92358_d_n6, assign56410_e92358_d_n7, assign56410_e92358_d_n8, assign56410_e92358_d_n9, assign56410_e92358_d_n10, assign56410_e92358_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign56410_e92355: f64 = (1.0 + locals.var_zsat);
        let assign56410_e92356: f64 = (0.5 * assign56410_e92355);
        (assign56410_e92356, (0.5 * locals.var_zsat_dn3), (0.5 * locals.var_zsat_dn4), (0.5 * locals.var_zsat_dn5), (0.5 * locals.var_zsat_dn6), (0.5 * locals.var_zsat_dn7), (0.5 * locals.var_zsat_dn8), (0.5 * locals.var_zsat_dn9), (0.5 * locals.var_zsat_dn10), (0.5 * locals.var_zsat_dn11),)
    } else {
        (locals.var_dvsat, locals.var_dvsat_dn3, locals.var_dvsat_dn4, locals.var_dvsat_dn5, locals.var_dvsat_dn6, locals.var_dvsat_dn7, locals.var_dvsat_dn8, locals.var_dvsat_dn9, locals.var_dvsat_dn10, locals.var_dvsat_dn11,)
    }
};
        locals.var_dvsat = assign56410_e92358;
        locals.var_dvsat_dn3 = assign56410_e92358_d_n3;
        locals.var_dvsat_dn4 = assign56410_e92358_d_n4;
        locals.var_dvsat_dn5 = assign56410_e92358_d_n5;
        locals.var_dvsat_dn6 = assign56410_e92358_d_n6;
        locals.var_dvsat_dn7 = assign56410_e92358_d_n7;
        locals.var_dvsat_dn8 = assign56410_e92358_d_n8;
        locals.var_dvsat_dn9 = assign56410_e92358_d_n9;
        locals.var_dvsat_dn10 = assign56410_e92358_d_n10;
        locals.var_dvsat_dn11 = assign56410_e92358_d_n11;

        let (assign56420_e92371, assign56420_e92371_d_n3, assign56420_e92371_d_n4, assign56420_e92371_d_n5, assign56420_e92371_d_n6, assign56420_e92371_d_n7, assign56420_e92371_d_n8, assign56420_e92371_d_n9, assign56420_e92371_d_n10, assign56420_e92371_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign56420_e92365: f64 = (2.0 * locals.var_vsatcv_t);
        let assign56420_e92368: f64 = (locals.var_u0_a / locals.var_dmob);
        let assign56420_e92369: f64 = (assign56420_e92365 / assign56420_e92368);
        (assign56420_e92369, ((((2.0 * locals.var_vsatcv_t_dn3) * assign56420_e92368) - (assign56420_e92365 * (((locals.var_u0_a_dn3 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn3)) / (locals.var_dmob * locals.var_dmob)))) / (assign56420_e92368 * assign56420_e92368)), ((((2.0 * locals.var_vsatcv_t_dn4) * assign56420_e92368) - (assign56420_e92365 * (((locals.var_u0_a_dn4 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn4)) / (locals.var_dmob * locals.var_dmob)))) / (assign56420_e92368 * assign56420_e92368)), ((((2.0 * locals.var_vsatcv_t_dn5) * assign56420_e92368) - (assign56420_e92365 * (((locals.var_u0_a_dn5 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn5)) / (locals.var_dmob * locals.var_dmob)))) / (assign56420_e92368 * assign56420_e92368)), ((((2.0 * locals.var_vsatcv_t_dn6) * assign56420_e92368) - (assign56420_e92365 * (((locals.var_u0_a_dn6 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn6)) / (locals.var_dmob * locals.var_dmob)))) / (assign56420_e92368 * assign56420_e92368)), ((((2.0 * locals.var_vsatcv_t_dn7) * assign56420_e92368) - (assign56420_e92365 * (((locals.var_u0_a_dn7 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn7)) / (locals.var_dmob * locals.var_dmob)))) / (assign56420_e92368 * assign56420_e92368)), ((((2.0 * locals.var_vsatcv_t_dn8) * assign56420_e92368) - (assign56420_e92365 * (((locals.var_u0_a_dn8 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn8)) / (locals.var_dmob * locals.var_dmob)))) / (assign56420_e92368 * assign56420_e92368)), ((((2.0 * locals.var_vsatcv_t_dn9) * assign56420_e92368) - (assign56420_e92365 * (((locals.var_u0_a_dn9 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn9)) / (locals.var_dmob * locals.var_dmob)))) / (assign56420_e92368 * assign56420_e92368)), ((((2.0 * locals.var_vsatcv_t_dn10) * assign56420_e92368) - (assign56420_e92365 * (((locals.var_u0_a_dn10 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn10)) / (locals.var_dmob * locals.var_dmob)))) / (assign56420_e92368 * assign56420_e92368)), ((((2.0 * locals.var_vsatcv_t_dn11) * assign56420_e92368) - (assign56420_e92365 * (((locals.var_u0_a_dn11 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn11)) / (locals.var_dmob * locals.var_dmob)))) / (assign56420_e92368 * assign56420_e92368)),)
    } else {
        (locals.var_esat, locals.var_esat_dn3, locals.var_esat_dn4, locals.var_esat_dn5, locals.var_esat_dn6, locals.var_esat_dn7, locals.var_esat_dn8, locals.var_esat_dn9, locals.var_esat_dn10, locals.var_esat_dn11,)
    }
};
        locals.var_esat = assign56420_e92371;
        locals.var_esat_dn3 = assign56420_e92371_d_n3;
        locals.var_esat_dn4 = assign56420_e92371_d_n4;
        locals.var_esat_dn5 = assign56420_e92371_d_n5;
        locals.var_esat_dn6 = assign56420_e92371_d_n6;
        locals.var_esat_dn7 = assign56420_e92371_d_n7;
        locals.var_esat_dn8 = assign56420_e92371_d_n8;
        locals.var_esat_dn9 = assign56420_e92371_d_n9;
        locals.var_esat_dn10 = assign56420_e92371_d_n10;
        locals.var_esat_dn11 = assign56420_e92371_d_n11;

        let (assign56430_e92380, assign56430_e92380_d_n3, assign56430_e92380_d_n4, assign56430_e92380_d_n5, assign56430_e92380_d_n6, assign56430_e92380_d_n7, assign56430_e92380_d_n8, assign56430_e92380_d_n9, assign56430_e92380_d_n10, assign56430_e92380_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign56430_e92378: f64 = (locals.var_esat * locals.var_lact);
        (assign56430_e92378, (locals.var_esat_dn3 * locals.var_lact), (locals.var_esat_dn4 * locals.var_lact), (locals.var_esat_dn5 * locals.var_lact), (locals.var_esat_dn6 * locals.var_lact), (locals.var_esat_dn7 * locals.var_lact), (locals.var_esat_dn8 * locals.var_lact), (locals.var_esat_dn9 * locals.var_lact), (locals.var_esat_dn10 * locals.var_lact), (locals.var_esat_dn11 * locals.var_lact),)
    } else {
        (locals.var_esatl, locals.var_esatl_dn3, locals.var_esatl_dn4, locals.var_esatl_dn5, locals.var_esatl_dn6, locals.var_esatl_dn7, locals.var_esatl_dn8, locals.var_esatl_dn9, locals.var_esatl_dn10, locals.var_esatl_dn11,)
    }
};
        locals.var_esatl = assign56430_e92380;
        locals.var_esatl_dn3 = assign56430_e92380_d_n3;
        locals.var_esatl_dn4 = assign56430_e92380_d_n4;
        locals.var_esatl_dn5 = assign56430_e92380_d_n5;
        locals.var_esatl_dn6 = assign56430_e92380_d_n6;
        locals.var_esatl_dn7 = assign56430_e92380_d_n7;
        locals.var_esatl_dn8 = assign56430_e92380_d_n8;
        locals.var_esatl_dn9 = assign56430_e92380_d_n9;
        locals.var_esatl_dn10 = assign56430_e92380_d_n10;
        locals.var_esatl_dn11 = assign56430_e92380_d_n11;

        let (assign56440_e92389, assign56440_e92389_d_n3, assign56440_e92389_d_n4, assign56440_e92389_d_n5, assign56440_e92389_d_n6, assign56440_e92389_d_n7, assign56440_e92389_d_n8, assign56440_e92389_d_n9, assign56440_e92389_d_n10, assign56440_e92389_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign56440_e92387: f64 = (locals.var_vdssatcv + locals.var_esatl);
        (assign56440_e92387, (locals.var_vdssatcv_dn3 + locals.var_esatl_dn3), (locals.var_vdssatcv_dn4 + locals.var_esatl_dn4), (locals.var_vdssatcv_dn5 + locals.var_esatl_dn5), (locals.var_vdssatcv_dn6 + locals.var_esatl_dn6), (locals.var_vdssatcv_dn7 + locals.var_esatl_dn7), (locals.var_vdssatcv_dn8 + locals.var_esatl_dn8), (locals.var_vdssatcv_dn9 + locals.var_esatl_dn9), (locals.var_vdssatcv_dn10 + locals.var_esatl_dn10), (locals.var_vdssatcv_dn11 + locals.var_esatl_dn11),)
    } else {
        (locals.var_vasat, locals.var_vasat_dn3, locals.var_vasat_dn4, locals.var_vasat_dn5, locals.var_vasat_dn6, locals.var_vasat_dn7, locals.var_vasat_dn8, locals.var_vasat_dn9, locals.var_vasat_dn10, locals.var_vasat_dn11,)
    }
};
        locals.var_vasat = assign56440_e92389;
        locals.var_vasat_dn3 = assign56440_e92389_d_n3;
        locals.var_vasat_dn4 = assign56440_e92389_d_n4;
        locals.var_vasat_dn5 = assign56440_e92389_d_n5;
        locals.var_vasat_dn6 = assign56440_e92389_d_n6;
        locals.var_vasat_dn7 = assign56440_e92389_d_n7;
        locals.var_vasat_dn8 = assign56440_e92389_d_n8;
        locals.var_vasat_dn9 = assign56440_e92389_d_n9;
        locals.var_vasat_dn10 = assign56440_e92389_d_n10;
        locals.var_vasat_dn11 = assign56440_e92389_d_n11;

        let (assign56450_e92398, assign56450_e92398_d_n3, assign56450_e92398_d_n4, assign56450_e92398_d_n5, assign56450_e92398_d_n6, assign56450_e92398_d_n7, assign56450_e92398_d_n8, assign56450_e92398_d_n9, assign56450_e92398_d_n10, assign56450_e92398_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard842 != 0.0)) {
        let assign56450_e92396: f64 = (locals.var_vds - locals.var_vdseff);
        (assign56450_e92396, (-locals.var_vdseff_dn3), (-locals.var_vdseff_dn4), (-locals.var_vdseff_dn5), (locals.var_vds_dn6 - locals.var_vdseff_dn6), (locals.var_vds_dn7 - locals.var_vdseff_dn7), (-locals.var_vdseff_dn8), (-locals.var_vdseff_dn9), (locals.var_vds_dn10 - locals.var_vdseff_dn10), (-locals.var_vdseff_dn11),)
    } else {
        (locals.var_diffvds, locals.var_diffvds_dn3, locals.var_diffvds_dn4, locals.var_diffvds_dn5, locals.var_diffvds_dn6, locals.var_diffvds_dn7, locals.var_diffvds_dn8, locals.var_diffvds_dn9, locals.var_diffvds_dn10, locals.var_diffvds_dn11,)
    }
};
        locals.var_diffvds = assign56450_e92398;
        locals.var_diffvds_dn3 = assign56450_e92398_d_n3;
        locals.var_diffvds_dn4 = assign56450_e92398_d_n4;
        locals.var_diffvds_dn5 = assign56450_e92398_d_n5;
        locals.var_diffvds_dn6 = assign56450_e92398_d_n6;
        locals.var_diffvds_dn7 = assign56450_e92398_d_n7;
        locals.var_diffvds_dn8 = assign56450_e92398_d_n8;
        locals.var_diffvds_dn9 = assign56450_e92398_d_n9;
        locals.var_diffvds_dn10 = assign56450_e92398_d_n10;
        locals.var_diffvds_dn11 = assign56450_e92398_d_n11;

        let assign56460_e92401: f64 = if locals.var_pclmcv_i != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard852 = assign56460_e92401;

    }
}
