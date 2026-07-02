#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_32(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign15920_e10857, assign15920_e10857_d_n0, assign15920_e10857_d_n2, assign15920_e10857_d_n4, assign15920_e10857_d_n5, assign15920_e10857_d_n6, assign15920_e10857_d_n7, assign15920_e10857_d_n8, assign15920_e10857_d_n9, assign15920_e10857_d_n10, assign15920_e10857_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign15920_e10843: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign15920_e10846: f64 = (locals.var_eg * locals.var_beta);
        let assign15920_e10847: f64 = (assign15920_e10843 - assign15920_e10846);
        let assign15920_e10850: f64 = (p.p532 * locals.var_log_tratio);
        let assign15920_e10851: f64 = (assign15920_e10847 + assign15920_e10850);
        let assign15920_e10853: f64 = (assign15920_e10851 / p.p520);
        let assign15920_e10854: f64 = (assign15920_e10853).exp();
        let assign15920_e10855: f64 = (locals.var_uc_js0sws * assign15920_e10854);
        (assign15920_e10855, (locals.var_uc_js0sws * (assign15920_e10854 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p532 * locals.var_log_tratio_dn0)) / p.p520))), (locals.var_uc_js0sws * (assign15920_e10854 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p532 * locals.var_log_tratio_dn2)) / p.p520))), (locals.var_uc_js0sws * (assign15920_e10854 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p532 * locals.var_log_tratio_dn4)) / p.p520))), (locals.var_uc_js0sws * (assign15920_e10854 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p532 * locals.var_log_tratio_dn5)) / p.p520))), (locals.var_uc_js0sws * (assign15920_e10854 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p532 * locals.var_log_tratio_dn6)) / p.p520))), (locals.var_uc_js0sws * (assign15920_e10854 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p532 * locals.var_log_tratio_dn7)) / p.p520))), (locals.var_uc_js0sws * (assign15920_e10854 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p532 * locals.var_log_tratio_dn8)) / p.p520))), (locals.var_uc_js0sws * (assign15920_e10854 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p532 * locals.var_log_tratio_dn9)) / p.p520))), (locals.var_uc_js0sws * (assign15920_e10854 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p532 * locals.var_log_tratio_dn10)) / p.p520))), (locals.var_uc_js0sws * (assign15920_e10854 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p532 * locals.var_log_tratio_dn13)) / p.p520))),)
    } else {
        (locals.var_jssw2, locals.var_jssw2_dn0, locals.var_jssw2_dn2, locals.var_jssw2_dn4, locals.var_jssw2_dn5, locals.var_jssw2_dn6, locals.var_jssw2_dn7, locals.var_jssw2_dn8, locals.var_jssw2_dn9, locals.var_jssw2_dn10, locals.var_jssw2_dn13,)
    }
};
        locals.var_jssw2 = assign15920_e10857;
        locals.var_jssw2_dn0 = assign15920_e10857_d_n0;
        locals.var_jssw2_dn2 = assign15920_e10857_d_n2;
        locals.var_jssw2_dn4 = assign15920_e10857_d_n4;
        locals.var_jssw2_dn5 = assign15920_e10857_d_n5;
        locals.var_jssw2_dn6 = assign15920_e10857_d_n6;
        locals.var_jssw2_dn7 = assign15920_e10857_d_n7;
        locals.var_jssw2_dn8 = assign15920_e10857_d_n8;
        locals.var_jssw2_dn9 = assign15920_e10857_d_n9;
        locals.var_jssw2_dn10 = assign15920_e10857_d_n10;
        locals.var_jssw2_dn13 = assign15920_e10857_d_n13;

        let (assign15930_e10876, assign15930_e10876_d_n0, assign15930_e10876_d_n2, assign15930_e10876_d_n4, assign15930_e10876_d_n5, assign15930_e10876_d_n6, assign15930_e10876_d_n7, assign15930_e10876_d_n8, assign15930_e10876_d_n9, assign15930_e10876_d_n10, assign15930_e10876_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign15930_e10862: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign15930_e10865: f64 = (locals.var_eg * locals.var_beta);
        let assign15930_e10866: f64 = (assign15930_e10862 - assign15930_e10865);
        let assign15930_e10869: f64 = (p.p532 * locals.var_log_tratio);
        let assign15930_e10870: f64 = (assign15930_e10866 + assign15930_e10869);
        let assign15930_e10872: f64 = (assign15930_e10870 / p.p521);
        let assign15930_e10873: f64 = (assign15930_e10872).exp();
        let assign15930_e10874: f64 = (p.p518 * assign15930_e10873);
        (assign15930_e10874, (p.p518 * (assign15930_e10873 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p532 * locals.var_log_tratio_dn0)) / p.p521))), (p.p518 * (assign15930_e10873 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p532 * locals.var_log_tratio_dn2)) / p.p521))), (p.p518 * (assign15930_e10873 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p532 * locals.var_log_tratio_dn4)) / p.p521))), (p.p518 * (assign15930_e10873 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p532 * locals.var_log_tratio_dn5)) / p.p521))), (p.p518 * (assign15930_e10873 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p532 * locals.var_log_tratio_dn6)) / p.p521))), (p.p518 * (assign15930_e10873 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p532 * locals.var_log_tratio_dn7)) / p.p521))), (p.p518 * (assign15930_e10873 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p532 * locals.var_log_tratio_dn8)) / p.p521))), (p.p518 * (assign15930_e10873 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p532 * locals.var_log_tratio_dn9)) / p.p521))), (p.p518 * (assign15930_e10873 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p532 * locals.var_log_tratio_dn10)) / p.p521))), (p.p518 * (assign15930_e10873 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p532 * locals.var_log_tratio_dn13)) / p.p521))),)
    } else {
        (locals.var_jsswg2, locals.var_jsswg2_dn0, locals.var_jsswg2_dn2, locals.var_jsswg2_dn4, locals.var_jsswg2_dn5, locals.var_jsswg2_dn6, locals.var_jsswg2_dn7, locals.var_jsswg2_dn8, locals.var_jsswg2_dn9, locals.var_jsswg2_dn10, locals.var_jsswg2_dn13,)
    }
};
        locals.var_jsswg2 = assign15930_e10876;
        locals.var_jsswg2_dn0 = assign15930_e10876_d_n0;
        locals.var_jsswg2_dn2 = assign15930_e10876_d_n2;
        locals.var_jsswg2_dn4 = assign15930_e10876_d_n4;
        locals.var_jsswg2_dn5 = assign15930_e10876_d_n5;
        locals.var_jsswg2_dn6 = assign15930_e10876_d_n6;
        locals.var_jsswg2_dn7 = assign15930_e10876_d_n7;
        locals.var_jsswg2_dn8 = assign15930_e10876_d_n8;
        locals.var_jsswg2_dn9 = assign15930_e10876_d_n9;
        locals.var_jsswg2_dn10 = assign15930_e10876_d_n10;
        locals.var_jsswg2_dn13 = assign15930_e10876_d_n13;

        let assign15940_e10879: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard332 = assign15940_e10879;

        let assign15950_e10882: f64 = if p.p16 > locals.var_weff_nf { 1.0 } else { 0.0 };
        locals.var_guard333 = assign15950_e10882;

        let (assign15960_e10892, assign15960_e10892_d_n0, assign15960_e10892_d_n2, assign15960_e10892_d_n4, assign15960_e10892_d_n5, assign15960_e10892_d_n6, assign15960_e10892_d_n7, assign15960_e10892_d_n8, assign15960_e10892_d_n9, assign15960_e10892_d_n10, assign15960_e10892_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard332 != 0.0)) && (locals.var_guard333 != 0.0)) {
        let assign15960_e10890: f64 = (p.p14 * locals.var_js);
        (assign15960_e10890, (p.p14 * locals.var_js_dn0), (p.p14 * locals.var_js_dn2), (p.p14 * locals.var_js_dn4), (p.p14 * locals.var_js_dn5), (p.p14 * locals.var_js_dn6), (p.p14 * locals.var_js_dn7), (p.p14 * locals.var_js_dn8), (p.p14 * locals.var_js_dn9), (p.p14 * locals.var_js_dn10), (p.p14 * locals.var_js_dn13),)
    } else {
        (locals.var_isbs_btm, locals.var_isbs_btm_dn0, locals.var_isbs_btm_dn2, locals.var_isbs_btm_dn4, locals.var_isbs_btm_dn5, locals.var_isbs_btm_dn6, locals.var_isbs_btm_dn7, locals.var_isbs_btm_dn8, locals.var_isbs_btm_dn9, locals.var_isbs_btm_dn10, locals.var_isbs_btm_dn13,)
    }
};
        locals.var_isbs_btm = assign15960_e10892;
        locals.var_isbs_btm_dn0 = assign15960_e10892_d_n0;
        locals.var_isbs_btm_dn2 = assign15960_e10892_d_n2;
        locals.var_isbs_btm_dn4 = assign15960_e10892_d_n4;
        locals.var_isbs_btm_dn5 = assign15960_e10892_d_n5;
        locals.var_isbs_btm_dn6 = assign15960_e10892_d_n6;
        locals.var_isbs_btm_dn7 = assign15960_e10892_d_n7;
        locals.var_isbs_btm_dn8 = assign15960_e10892_d_n8;
        locals.var_isbs_btm_dn9 = assign15960_e10892_d_n9;
        locals.var_isbs_btm_dn10 = assign15960_e10892_d_n10;
        locals.var_isbs_btm_dn13 = assign15960_e10892_d_n13;

        let (assign15970_e10902, assign15970_e10902_d_n0, assign15970_e10902_d_n2, assign15970_e10902_d_n4, assign15970_e10902_d_n5, assign15970_e10902_d_n6, assign15970_e10902_d_n7, assign15970_e10902_d_n8, assign15970_e10902_d_n9, assign15970_e10902_d_n10, assign15970_e10902_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard332 != 0.0)) && (locals.var_guard333 != 0.0)) {
        let assign15970_e10900: f64 = (p.p14 * locals.var_js2);
        (assign15970_e10900, (p.p14 * locals.var_js2_dn0), (p.p14 * locals.var_js2_dn2), (p.p14 * locals.var_js2_dn4), (p.p14 * locals.var_js2_dn5), (p.p14 * locals.var_js2_dn6), (p.p14 * locals.var_js2_dn7), (p.p14 * locals.var_js2_dn8), (p.p14 * locals.var_js2_dn9), (p.p14 * locals.var_js2_dn10), (p.p14 * locals.var_js2_dn13),)
    } else {
        (locals.var_isbs2_btm, locals.var_isbs2_btm_dn0, locals.var_isbs2_btm_dn2, locals.var_isbs2_btm_dn4, locals.var_isbs2_btm_dn5, locals.var_isbs2_btm_dn6, locals.var_isbs2_btm_dn7, locals.var_isbs2_btm_dn8, locals.var_isbs2_btm_dn9, locals.var_isbs2_btm_dn10, locals.var_isbs2_btm_dn13,)
    }
};
        locals.var_isbs2_btm = assign15970_e10902;
        locals.var_isbs2_btm_dn0 = assign15970_e10902_d_n0;
        locals.var_isbs2_btm_dn2 = assign15970_e10902_d_n2;
        locals.var_isbs2_btm_dn4 = assign15970_e10902_d_n4;
        locals.var_isbs2_btm_dn5 = assign15970_e10902_d_n5;
        locals.var_isbs2_btm_dn6 = assign15970_e10902_d_n6;
        locals.var_isbs2_btm_dn7 = assign15970_e10902_d_n7;
        locals.var_isbs2_btm_dn8 = assign15970_e10902_d_n8;
        locals.var_isbs2_btm_dn9 = assign15970_e10902_d_n9;
        locals.var_isbs2_btm_dn10 = assign15970_e10902_d_n10;
        locals.var_isbs2_btm_dn13 = assign15970_e10902_d_n13;

        let (assign15980_e10914, assign15980_e10914_d_n0, assign15980_e10914_d_n2, assign15980_e10914_d_n4, assign15980_e10914_d_n5, assign15980_e10914_d_n6, assign15980_e10914_d_n7, assign15980_e10914_d_n8, assign15980_e10914_d_n9, assign15980_e10914_d_n10, assign15980_e10914_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard332 != 0.0)) && (locals.var_guard333 != 0.0)) {
        let assign15980_e10910: f64 = (p.p16 - locals.var_weff_nf);
        let assign15980_e10912: f64 = (assign15980_e10910 * locals.var_jssw);
        (assign15980_e10912, (assign15980_e10910 * locals.var_jssw_dn0), (assign15980_e10910 * locals.var_jssw_dn2), (assign15980_e10910 * locals.var_jssw_dn4), (assign15980_e10910 * locals.var_jssw_dn5), (assign15980_e10910 * locals.var_jssw_dn6), (assign15980_e10910 * locals.var_jssw_dn7), (assign15980_e10910 * locals.var_jssw_dn8), (assign15980_e10910 * locals.var_jssw_dn9), (assign15980_e10910 * locals.var_jssw_dn10), (assign15980_e10910 * locals.var_jssw_dn13),)
    } else {
        (locals.var_isbs_sws, locals.var_isbs_sws_dn0, locals.var_isbs_sws_dn2, locals.var_isbs_sws_dn4, locals.var_isbs_sws_dn5, locals.var_isbs_sws_dn6, locals.var_isbs_sws_dn7, locals.var_isbs_sws_dn8, locals.var_isbs_sws_dn9, locals.var_isbs_sws_dn10, locals.var_isbs_sws_dn13,)
    }
};
        locals.var_isbs_sws = assign15980_e10914;
        locals.var_isbs_sws_dn0 = assign15980_e10914_d_n0;
        locals.var_isbs_sws_dn2 = assign15980_e10914_d_n2;
        locals.var_isbs_sws_dn4 = assign15980_e10914_d_n4;
        locals.var_isbs_sws_dn5 = assign15980_e10914_d_n5;
        locals.var_isbs_sws_dn6 = assign15980_e10914_d_n6;
        locals.var_isbs_sws_dn7 = assign15980_e10914_d_n7;
        locals.var_isbs_sws_dn8 = assign15980_e10914_d_n8;
        locals.var_isbs_sws_dn9 = assign15980_e10914_d_n9;
        locals.var_isbs_sws_dn10 = assign15980_e10914_d_n10;
        locals.var_isbs_sws_dn13 = assign15980_e10914_d_n13;

        let (assign15990_e10926, assign15990_e10926_d_n0, assign15990_e10926_d_n2, assign15990_e10926_d_n4, assign15990_e10926_d_n5, assign15990_e10926_d_n6, assign15990_e10926_d_n7, assign15990_e10926_d_n8, assign15990_e10926_d_n9, assign15990_e10926_d_n10, assign15990_e10926_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard332 != 0.0)) && (locals.var_guard333 != 0.0)) {
        let assign15990_e10922: f64 = (p.p16 - locals.var_weff_nf);
        let assign15990_e10924: f64 = (assign15990_e10922 * locals.var_jssw2);
        (assign15990_e10924, (assign15990_e10922 * locals.var_jssw2_dn0), (assign15990_e10922 * locals.var_jssw2_dn2), (assign15990_e10922 * locals.var_jssw2_dn4), (assign15990_e10922 * locals.var_jssw2_dn5), (assign15990_e10922 * locals.var_jssw2_dn6), (assign15990_e10922 * locals.var_jssw2_dn7), (assign15990_e10922 * locals.var_jssw2_dn8), (assign15990_e10922 * locals.var_jssw2_dn9), (assign15990_e10922 * locals.var_jssw2_dn10), (assign15990_e10922 * locals.var_jssw2_dn13),)
    } else {
        (locals.var_isbs2_sws, locals.var_isbs2_sws_dn0, locals.var_isbs2_sws_dn2, locals.var_isbs2_sws_dn4, locals.var_isbs2_sws_dn5, locals.var_isbs2_sws_dn6, locals.var_isbs2_sws_dn7, locals.var_isbs2_sws_dn8, locals.var_isbs2_sws_dn9, locals.var_isbs2_sws_dn10, locals.var_isbs2_sws_dn13,)
    }
};
        locals.var_isbs2_sws = assign15990_e10926;
        locals.var_isbs2_sws_dn0 = assign15990_e10926_d_n0;
        locals.var_isbs2_sws_dn2 = assign15990_e10926_d_n2;
        locals.var_isbs2_sws_dn4 = assign15990_e10926_d_n4;
        locals.var_isbs2_sws_dn5 = assign15990_e10926_d_n5;
        locals.var_isbs2_sws_dn6 = assign15990_e10926_d_n6;
        locals.var_isbs2_sws_dn7 = assign15990_e10926_d_n7;
        locals.var_isbs2_sws_dn8 = assign15990_e10926_d_n8;
        locals.var_isbs2_sws_dn9 = assign15990_e10926_d_n9;
        locals.var_isbs2_sws_dn10 = assign15990_e10926_d_n10;
        locals.var_isbs2_sws_dn13 = assign15990_e10926_d_n13;

        let (assign16000_e10936, assign16000_e10936_d_n0, assign16000_e10936_d_n2, assign16000_e10936_d_n4, assign16000_e10936_d_n5, assign16000_e10936_d_n6, assign16000_e10936_d_n7, assign16000_e10936_d_n8, assign16000_e10936_d_n9, assign16000_e10936_d_n10, assign16000_e10936_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard332 != 0.0)) && (locals.var_guard333 != 0.0)) {
        let assign16000_e10934: f64 = (locals.var_weff_nf * locals.var_jsswg);
        (assign16000_e10934, (locals.var_weff_nf * locals.var_jsswg_dn0), (locals.var_weff_nf * locals.var_jsswg_dn2), (locals.var_weff_nf * locals.var_jsswg_dn4), (locals.var_weff_nf * locals.var_jsswg_dn5), (locals.var_weff_nf * locals.var_jsswg_dn6), (locals.var_weff_nf * locals.var_jsswg_dn7), (locals.var_weff_nf * locals.var_jsswg_dn8), (locals.var_weff_nf * locals.var_jsswg_dn9), (locals.var_weff_nf * locals.var_jsswg_dn10), (locals.var_weff_nf * locals.var_jsswg_dn13),)
    } else {
        (locals.var_isbs_swg, locals.var_isbs_swg_dn0, locals.var_isbs_swg_dn2, locals.var_isbs_swg_dn4, locals.var_isbs_swg_dn5, locals.var_isbs_swg_dn6, locals.var_isbs_swg_dn7, locals.var_isbs_swg_dn8, locals.var_isbs_swg_dn9, locals.var_isbs_swg_dn10, locals.var_isbs_swg_dn13,)
    }
};
        locals.var_isbs_swg = assign16000_e10936;
        locals.var_isbs_swg_dn0 = assign16000_e10936_d_n0;
        locals.var_isbs_swg_dn2 = assign16000_e10936_d_n2;
        locals.var_isbs_swg_dn4 = assign16000_e10936_d_n4;
        locals.var_isbs_swg_dn5 = assign16000_e10936_d_n5;
        locals.var_isbs_swg_dn6 = assign16000_e10936_d_n6;
        locals.var_isbs_swg_dn7 = assign16000_e10936_d_n7;
        locals.var_isbs_swg_dn8 = assign16000_e10936_d_n8;
        locals.var_isbs_swg_dn9 = assign16000_e10936_d_n9;
        locals.var_isbs_swg_dn10 = assign16000_e10936_d_n10;
        locals.var_isbs_swg_dn13 = assign16000_e10936_d_n13;

        let (assign16010_e10946, assign16010_e10946_d_n0, assign16010_e10946_d_n2, assign16010_e10946_d_n4, assign16010_e10946_d_n5, assign16010_e10946_d_n6, assign16010_e10946_d_n7, assign16010_e10946_d_n8, assign16010_e10946_d_n9, assign16010_e10946_d_n10, assign16010_e10946_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard332 != 0.0)) && (locals.var_guard333 != 0.0)) {
        let assign16010_e10944: f64 = (locals.var_weff_nf * locals.var_jsswg2);
        (assign16010_e10944, (locals.var_weff_nf * locals.var_jsswg2_dn0), (locals.var_weff_nf * locals.var_jsswg2_dn2), (locals.var_weff_nf * locals.var_jsswg2_dn4), (locals.var_weff_nf * locals.var_jsswg2_dn5), (locals.var_weff_nf * locals.var_jsswg2_dn6), (locals.var_weff_nf * locals.var_jsswg2_dn7), (locals.var_weff_nf * locals.var_jsswg2_dn8), (locals.var_weff_nf * locals.var_jsswg2_dn9), (locals.var_weff_nf * locals.var_jsswg2_dn10), (locals.var_weff_nf * locals.var_jsswg2_dn13),)
    } else {
        (locals.var_isbs2_swg, locals.var_isbs2_swg_dn0, locals.var_isbs2_swg_dn2, locals.var_isbs2_swg_dn4, locals.var_isbs2_swg_dn5, locals.var_isbs2_swg_dn6, locals.var_isbs2_swg_dn7, locals.var_isbs2_swg_dn8, locals.var_isbs2_swg_dn9, locals.var_isbs2_swg_dn10, locals.var_isbs2_swg_dn13,)
    }
};
        locals.var_isbs2_swg = assign16010_e10946;
        locals.var_isbs2_swg_dn0 = assign16010_e10946_d_n0;
        locals.var_isbs2_swg_dn2 = assign16010_e10946_d_n2;
        locals.var_isbs2_swg_dn4 = assign16010_e10946_d_n4;
        locals.var_isbs2_swg_dn5 = assign16010_e10946_d_n5;
        locals.var_isbs2_swg_dn6 = assign16010_e10946_d_n6;
        locals.var_isbs2_swg_dn7 = assign16010_e10946_d_n7;
        locals.var_isbs2_swg_dn8 = assign16010_e10946_d_n8;
        locals.var_isbs2_swg_dn9 = assign16010_e10946_d_n9;
        locals.var_isbs2_swg_dn10 = assign16010_e10946_d_n10;
        locals.var_isbs2_swg_dn13 = assign16010_e10946_d_n13;

        let (assign16020_e10957, assign16020_e10957_d_n0, assign16020_e10957_d_n2, assign16020_e10957_d_n4, assign16020_e10957_d_n5, assign16020_e10957_d_n6, assign16020_e10957_d_n7, assign16020_e10957_d_n8, assign16020_e10957_d_n9, assign16020_e10957_d_n10, assign16020_e10957_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard332 != 0.0)) && (locals.var_guard333 == 0.0)) {
        let assign16020_e10955: f64 = (p.p14 * locals.var_js);
        (assign16020_e10955, (p.p14 * locals.var_js_dn0), (p.p14 * locals.var_js_dn2), (p.p14 * locals.var_js_dn4), (p.p14 * locals.var_js_dn5), (p.p14 * locals.var_js_dn6), (p.p14 * locals.var_js_dn7), (p.p14 * locals.var_js_dn8), (p.p14 * locals.var_js_dn9), (p.p14 * locals.var_js_dn10), (p.p14 * locals.var_js_dn13),)
    } else {
        (locals.var_isbs_btm, locals.var_isbs_btm_dn0, locals.var_isbs_btm_dn2, locals.var_isbs_btm_dn4, locals.var_isbs_btm_dn5, locals.var_isbs_btm_dn6, locals.var_isbs_btm_dn7, locals.var_isbs_btm_dn8, locals.var_isbs_btm_dn9, locals.var_isbs_btm_dn10, locals.var_isbs_btm_dn13,)
    }
};
        locals.var_isbs_btm = assign16020_e10957;
        locals.var_isbs_btm_dn0 = assign16020_e10957_d_n0;
        locals.var_isbs_btm_dn2 = assign16020_e10957_d_n2;
        locals.var_isbs_btm_dn4 = assign16020_e10957_d_n4;
        locals.var_isbs_btm_dn5 = assign16020_e10957_d_n5;
        locals.var_isbs_btm_dn6 = assign16020_e10957_d_n6;
        locals.var_isbs_btm_dn7 = assign16020_e10957_d_n7;
        locals.var_isbs_btm_dn8 = assign16020_e10957_d_n8;
        locals.var_isbs_btm_dn9 = assign16020_e10957_d_n9;
        locals.var_isbs_btm_dn10 = assign16020_e10957_d_n10;
        locals.var_isbs_btm_dn13 = assign16020_e10957_d_n13;

        let (assign16030_e10968, assign16030_e10968_d_n0, assign16030_e10968_d_n2, assign16030_e10968_d_n4, assign16030_e10968_d_n5, assign16030_e10968_d_n6, assign16030_e10968_d_n7, assign16030_e10968_d_n8, assign16030_e10968_d_n9, assign16030_e10968_d_n10, assign16030_e10968_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard332 != 0.0)) && (locals.var_guard333 == 0.0)) {
        let assign16030_e10966: f64 = (p.p14 * locals.var_js2);
        (assign16030_e10966, (p.p14 * locals.var_js2_dn0), (p.p14 * locals.var_js2_dn2), (p.p14 * locals.var_js2_dn4), (p.p14 * locals.var_js2_dn5), (p.p14 * locals.var_js2_dn6), (p.p14 * locals.var_js2_dn7), (p.p14 * locals.var_js2_dn8), (p.p14 * locals.var_js2_dn9), (p.p14 * locals.var_js2_dn10), (p.p14 * locals.var_js2_dn13),)
    } else {
        (locals.var_isbs2_btm, locals.var_isbs2_btm_dn0, locals.var_isbs2_btm_dn2, locals.var_isbs2_btm_dn4, locals.var_isbs2_btm_dn5, locals.var_isbs2_btm_dn6, locals.var_isbs2_btm_dn7, locals.var_isbs2_btm_dn8, locals.var_isbs2_btm_dn9, locals.var_isbs2_btm_dn10, locals.var_isbs2_btm_dn13,)
    }
};
        locals.var_isbs2_btm = assign16030_e10968;
        locals.var_isbs2_btm_dn0 = assign16030_e10968_d_n0;
        locals.var_isbs2_btm_dn2 = assign16030_e10968_d_n2;
        locals.var_isbs2_btm_dn4 = assign16030_e10968_d_n4;
        locals.var_isbs2_btm_dn5 = assign16030_e10968_d_n5;
        locals.var_isbs2_btm_dn6 = assign16030_e10968_d_n6;
        locals.var_isbs2_btm_dn7 = assign16030_e10968_d_n7;
        locals.var_isbs2_btm_dn8 = assign16030_e10968_d_n8;
        locals.var_isbs2_btm_dn9 = assign16030_e10968_d_n9;
        locals.var_isbs2_btm_dn10 = assign16030_e10968_d_n10;
        locals.var_isbs2_btm_dn13 = assign16030_e10968_d_n13;

        let (assign16040_e10977, assign16040_e10977_d_n0, assign16040_e10977_d_n2, assign16040_e10977_d_n4, assign16040_e10977_d_n5, assign16040_e10977_d_n6, assign16040_e10977_d_n7, assign16040_e10977_d_n8, assign16040_e10977_d_n9, assign16040_e10977_d_n10, assign16040_e10977_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard332 != 0.0)) && (locals.var_guard333 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbs_sws, locals.var_isbs_sws_dn0, locals.var_isbs_sws_dn2, locals.var_isbs_sws_dn4, locals.var_isbs_sws_dn5, locals.var_isbs_sws_dn6, locals.var_isbs_sws_dn7, locals.var_isbs_sws_dn8, locals.var_isbs_sws_dn9, locals.var_isbs_sws_dn10, locals.var_isbs_sws_dn13,)
    }
};
        locals.var_isbs_sws = assign16040_e10977;
        locals.var_isbs_sws_dn0 = assign16040_e10977_d_n0;
        locals.var_isbs_sws_dn2 = assign16040_e10977_d_n2;
        locals.var_isbs_sws_dn4 = assign16040_e10977_d_n4;
        locals.var_isbs_sws_dn5 = assign16040_e10977_d_n5;
        locals.var_isbs_sws_dn6 = assign16040_e10977_d_n6;
        locals.var_isbs_sws_dn7 = assign16040_e10977_d_n7;
        locals.var_isbs_sws_dn8 = assign16040_e10977_d_n8;
        locals.var_isbs_sws_dn9 = assign16040_e10977_d_n9;
        locals.var_isbs_sws_dn10 = assign16040_e10977_d_n10;
        locals.var_isbs_sws_dn13 = assign16040_e10977_d_n13;

        let (assign16050_e10986, assign16050_e10986_d_n0, assign16050_e10986_d_n2, assign16050_e10986_d_n4, assign16050_e10986_d_n5, assign16050_e10986_d_n6, assign16050_e10986_d_n7, assign16050_e10986_d_n8, assign16050_e10986_d_n9, assign16050_e10986_d_n10, assign16050_e10986_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard332 != 0.0)) && (locals.var_guard333 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbs2_sws, locals.var_isbs2_sws_dn0, locals.var_isbs2_sws_dn2, locals.var_isbs2_sws_dn4, locals.var_isbs2_sws_dn5, locals.var_isbs2_sws_dn6, locals.var_isbs2_sws_dn7, locals.var_isbs2_sws_dn8, locals.var_isbs2_sws_dn9, locals.var_isbs2_sws_dn10, locals.var_isbs2_sws_dn13,)
    }
};
        locals.var_isbs2_sws = assign16050_e10986;
        locals.var_isbs2_sws_dn0 = assign16050_e10986_d_n0;
        locals.var_isbs2_sws_dn2 = assign16050_e10986_d_n2;
        locals.var_isbs2_sws_dn4 = assign16050_e10986_d_n4;
        locals.var_isbs2_sws_dn5 = assign16050_e10986_d_n5;
        locals.var_isbs2_sws_dn6 = assign16050_e10986_d_n6;
        locals.var_isbs2_sws_dn7 = assign16050_e10986_d_n7;
        locals.var_isbs2_sws_dn8 = assign16050_e10986_d_n8;
        locals.var_isbs2_sws_dn9 = assign16050_e10986_d_n9;
        locals.var_isbs2_sws_dn10 = assign16050_e10986_d_n10;
        locals.var_isbs2_sws_dn13 = assign16050_e10986_d_n13;

        let (assign16060_e10997, assign16060_e10997_d_n0, assign16060_e10997_d_n2, assign16060_e10997_d_n4, assign16060_e10997_d_n5, assign16060_e10997_d_n6, assign16060_e10997_d_n7, assign16060_e10997_d_n8, assign16060_e10997_d_n9, assign16060_e10997_d_n10, assign16060_e10997_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard332 != 0.0)) && (locals.var_guard333 == 0.0)) {
        let assign16060_e10995: f64 = (p.p16 * locals.var_jsswg);
        (assign16060_e10995, (p.p16 * locals.var_jsswg_dn0), (p.p16 * locals.var_jsswg_dn2), (p.p16 * locals.var_jsswg_dn4), (p.p16 * locals.var_jsswg_dn5), (p.p16 * locals.var_jsswg_dn6), (p.p16 * locals.var_jsswg_dn7), (p.p16 * locals.var_jsswg_dn8), (p.p16 * locals.var_jsswg_dn9), (p.p16 * locals.var_jsswg_dn10), (p.p16 * locals.var_jsswg_dn13),)
    } else {
        (locals.var_isbs_swg, locals.var_isbs_swg_dn0, locals.var_isbs_swg_dn2, locals.var_isbs_swg_dn4, locals.var_isbs_swg_dn5, locals.var_isbs_swg_dn6, locals.var_isbs_swg_dn7, locals.var_isbs_swg_dn8, locals.var_isbs_swg_dn9, locals.var_isbs_swg_dn10, locals.var_isbs_swg_dn13,)
    }
};
        locals.var_isbs_swg = assign16060_e10997;
        locals.var_isbs_swg_dn0 = assign16060_e10997_d_n0;
        locals.var_isbs_swg_dn2 = assign16060_e10997_d_n2;
        locals.var_isbs_swg_dn4 = assign16060_e10997_d_n4;
        locals.var_isbs_swg_dn5 = assign16060_e10997_d_n5;
        locals.var_isbs_swg_dn6 = assign16060_e10997_d_n6;
        locals.var_isbs_swg_dn7 = assign16060_e10997_d_n7;
        locals.var_isbs_swg_dn8 = assign16060_e10997_d_n8;
        locals.var_isbs_swg_dn9 = assign16060_e10997_d_n9;
        locals.var_isbs_swg_dn10 = assign16060_e10997_d_n10;
        locals.var_isbs_swg_dn13 = assign16060_e10997_d_n13;

        let (assign16070_e11008, assign16070_e11008_d_n0, assign16070_e11008_d_n2, assign16070_e11008_d_n4, assign16070_e11008_d_n5, assign16070_e11008_d_n6, assign16070_e11008_d_n7, assign16070_e11008_d_n8, assign16070_e11008_d_n9, assign16070_e11008_d_n10, assign16070_e11008_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard332 != 0.0)) && (locals.var_guard333 == 0.0)) {
        let assign16070_e11006: f64 = (p.p16 * locals.var_jsswg2);
        (assign16070_e11006, (p.p16 * locals.var_jsswg2_dn0), (p.p16 * locals.var_jsswg2_dn2), (p.p16 * locals.var_jsswg2_dn4), (p.p16 * locals.var_jsswg2_dn5), (p.p16 * locals.var_jsswg2_dn6), (p.p16 * locals.var_jsswg2_dn7), (p.p16 * locals.var_jsswg2_dn8), (p.p16 * locals.var_jsswg2_dn9), (p.p16 * locals.var_jsswg2_dn10), (p.p16 * locals.var_jsswg2_dn13),)
    } else {
        (locals.var_isbs2_swg, locals.var_isbs2_swg_dn0, locals.var_isbs2_swg_dn2, locals.var_isbs2_swg_dn4, locals.var_isbs2_swg_dn5, locals.var_isbs2_swg_dn6, locals.var_isbs2_swg_dn7, locals.var_isbs2_swg_dn8, locals.var_isbs2_swg_dn9, locals.var_isbs2_swg_dn10, locals.var_isbs2_swg_dn13,)
    }
};
        locals.var_isbs2_swg = assign16070_e11008;
        locals.var_isbs2_swg_dn0 = assign16070_e11008_d_n0;
        locals.var_isbs2_swg_dn2 = assign16070_e11008_d_n2;
        locals.var_isbs2_swg_dn4 = assign16070_e11008_d_n4;
        locals.var_isbs2_swg_dn5 = assign16070_e11008_d_n5;
        locals.var_isbs2_swg_dn6 = assign16070_e11008_d_n6;
        locals.var_isbs2_swg_dn7 = assign16070_e11008_d_n7;
        locals.var_isbs2_swg_dn8 = assign16070_e11008_d_n8;
        locals.var_isbs2_swg_dn9 = assign16070_e11008_d_n9;
        locals.var_isbs2_swg_dn10 = assign16070_e11008_d_n10;
        locals.var_isbs2_swg_dn13 = assign16070_e11008_d_n13;

        let (assign16080_e11017, assign16080_e11017_d_n0, assign16080_e11017_d_n2, assign16080_e11017_d_n4, assign16080_e11017_d_n5, assign16080_e11017_d_n6, assign16080_e11017_d_n7, assign16080_e11017_d_n8, assign16080_e11017_d_n9, assign16080_e11017_d_n10, assign16080_e11017_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard332 == 0.0)) {
        let assign16080_e11015: f64 = (p.p14 * locals.var_js);
        (assign16080_e11015, (p.p14 * locals.var_js_dn0), (p.p14 * locals.var_js_dn2), (p.p14 * locals.var_js_dn4), (p.p14 * locals.var_js_dn5), (p.p14 * locals.var_js_dn6), (p.p14 * locals.var_js_dn7), (p.p14 * locals.var_js_dn8), (p.p14 * locals.var_js_dn9), (p.p14 * locals.var_js_dn10), (p.p14 * locals.var_js_dn13),)
    } else {
        (locals.var_isbs_btm, locals.var_isbs_btm_dn0, locals.var_isbs_btm_dn2, locals.var_isbs_btm_dn4, locals.var_isbs_btm_dn5, locals.var_isbs_btm_dn6, locals.var_isbs_btm_dn7, locals.var_isbs_btm_dn8, locals.var_isbs_btm_dn9, locals.var_isbs_btm_dn10, locals.var_isbs_btm_dn13,)
    }
};
        locals.var_isbs_btm = assign16080_e11017;
        locals.var_isbs_btm_dn0 = assign16080_e11017_d_n0;
        locals.var_isbs_btm_dn2 = assign16080_e11017_d_n2;
        locals.var_isbs_btm_dn4 = assign16080_e11017_d_n4;
        locals.var_isbs_btm_dn5 = assign16080_e11017_d_n5;
        locals.var_isbs_btm_dn6 = assign16080_e11017_d_n6;
        locals.var_isbs_btm_dn7 = assign16080_e11017_d_n7;
        locals.var_isbs_btm_dn8 = assign16080_e11017_d_n8;
        locals.var_isbs_btm_dn9 = assign16080_e11017_d_n9;
        locals.var_isbs_btm_dn10 = assign16080_e11017_d_n10;
        locals.var_isbs_btm_dn13 = assign16080_e11017_d_n13;

        let (assign16090_e11026, assign16090_e11026_d_n0, assign16090_e11026_d_n2, assign16090_e11026_d_n4, assign16090_e11026_d_n5, assign16090_e11026_d_n6, assign16090_e11026_d_n7, assign16090_e11026_d_n8, assign16090_e11026_d_n9, assign16090_e11026_d_n10, assign16090_e11026_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard332 == 0.0)) {
        let assign16090_e11024: f64 = (p.p14 * locals.var_js2);
        (assign16090_e11024, (p.p14 * locals.var_js2_dn0), (p.p14 * locals.var_js2_dn2), (p.p14 * locals.var_js2_dn4), (p.p14 * locals.var_js2_dn5), (p.p14 * locals.var_js2_dn6), (p.p14 * locals.var_js2_dn7), (p.p14 * locals.var_js2_dn8), (p.p14 * locals.var_js2_dn9), (p.p14 * locals.var_js2_dn10), (p.p14 * locals.var_js2_dn13),)
    } else {
        (locals.var_isbs2_btm, locals.var_isbs2_btm_dn0, locals.var_isbs2_btm_dn2, locals.var_isbs2_btm_dn4, locals.var_isbs2_btm_dn5, locals.var_isbs2_btm_dn6, locals.var_isbs2_btm_dn7, locals.var_isbs2_btm_dn8, locals.var_isbs2_btm_dn9, locals.var_isbs2_btm_dn10, locals.var_isbs2_btm_dn13,)
    }
};
        locals.var_isbs2_btm = assign16090_e11026;
        locals.var_isbs2_btm_dn0 = assign16090_e11026_d_n0;
        locals.var_isbs2_btm_dn2 = assign16090_e11026_d_n2;
        locals.var_isbs2_btm_dn4 = assign16090_e11026_d_n4;
        locals.var_isbs2_btm_dn5 = assign16090_e11026_d_n5;
        locals.var_isbs2_btm_dn6 = assign16090_e11026_d_n6;
        locals.var_isbs2_btm_dn7 = assign16090_e11026_d_n7;
        locals.var_isbs2_btm_dn8 = assign16090_e11026_d_n8;
        locals.var_isbs2_btm_dn9 = assign16090_e11026_d_n9;
        locals.var_isbs2_btm_dn10 = assign16090_e11026_d_n10;
        locals.var_isbs2_btm_dn13 = assign16090_e11026_d_n13;

        let (assign16100_e11035, assign16100_e11035_d_n0, assign16100_e11035_d_n2, assign16100_e11035_d_n4, assign16100_e11035_d_n5, assign16100_e11035_d_n6, assign16100_e11035_d_n7, assign16100_e11035_d_n8, assign16100_e11035_d_n9, assign16100_e11035_d_n10, assign16100_e11035_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard332 == 0.0)) {
        let assign16100_e11033: f64 = (p.p16 * locals.var_jssw);
        (assign16100_e11033, (p.p16 * locals.var_jssw_dn0), (p.p16 * locals.var_jssw_dn2), (p.p16 * locals.var_jssw_dn4), (p.p16 * locals.var_jssw_dn5), (p.p16 * locals.var_jssw_dn6), (p.p16 * locals.var_jssw_dn7), (p.p16 * locals.var_jssw_dn8), (p.p16 * locals.var_jssw_dn9), (p.p16 * locals.var_jssw_dn10), (p.p16 * locals.var_jssw_dn13),)
    } else {
        (locals.var_isbs_sws, locals.var_isbs_sws_dn0, locals.var_isbs_sws_dn2, locals.var_isbs_sws_dn4, locals.var_isbs_sws_dn5, locals.var_isbs_sws_dn6, locals.var_isbs_sws_dn7, locals.var_isbs_sws_dn8, locals.var_isbs_sws_dn9, locals.var_isbs_sws_dn10, locals.var_isbs_sws_dn13,)
    }
};
        locals.var_isbs_sws = assign16100_e11035;
        locals.var_isbs_sws_dn0 = assign16100_e11035_d_n0;
        locals.var_isbs_sws_dn2 = assign16100_e11035_d_n2;
        locals.var_isbs_sws_dn4 = assign16100_e11035_d_n4;
        locals.var_isbs_sws_dn5 = assign16100_e11035_d_n5;
        locals.var_isbs_sws_dn6 = assign16100_e11035_d_n6;
        locals.var_isbs_sws_dn7 = assign16100_e11035_d_n7;
        locals.var_isbs_sws_dn8 = assign16100_e11035_d_n8;
        locals.var_isbs_sws_dn9 = assign16100_e11035_d_n9;
        locals.var_isbs_sws_dn10 = assign16100_e11035_d_n10;
        locals.var_isbs_sws_dn13 = assign16100_e11035_d_n13;

        let (assign16110_e11044, assign16110_e11044_d_n0, assign16110_e11044_d_n2, assign16110_e11044_d_n4, assign16110_e11044_d_n5, assign16110_e11044_d_n6, assign16110_e11044_d_n7, assign16110_e11044_d_n8, assign16110_e11044_d_n9, assign16110_e11044_d_n10, assign16110_e11044_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard332 == 0.0)) {
        let assign16110_e11042: f64 = (p.p16 * locals.var_jssw2);
        (assign16110_e11042, (p.p16 * locals.var_jssw2_dn0), (p.p16 * locals.var_jssw2_dn2), (p.p16 * locals.var_jssw2_dn4), (p.p16 * locals.var_jssw2_dn5), (p.p16 * locals.var_jssw2_dn6), (p.p16 * locals.var_jssw2_dn7), (p.p16 * locals.var_jssw2_dn8), (p.p16 * locals.var_jssw2_dn9), (p.p16 * locals.var_jssw2_dn10), (p.p16 * locals.var_jssw2_dn13),)
    } else {
        (locals.var_isbs2_sws, locals.var_isbs2_sws_dn0, locals.var_isbs2_sws_dn2, locals.var_isbs2_sws_dn4, locals.var_isbs2_sws_dn5, locals.var_isbs2_sws_dn6, locals.var_isbs2_sws_dn7, locals.var_isbs2_sws_dn8, locals.var_isbs2_sws_dn9, locals.var_isbs2_sws_dn10, locals.var_isbs2_sws_dn13,)
    }
};
        locals.var_isbs2_sws = assign16110_e11044;
        locals.var_isbs2_sws_dn0 = assign16110_e11044_d_n0;
        locals.var_isbs2_sws_dn2 = assign16110_e11044_d_n2;
        locals.var_isbs2_sws_dn4 = assign16110_e11044_d_n4;
        locals.var_isbs2_sws_dn5 = assign16110_e11044_d_n5;
        locals.var_isbs2_sws_dn6 = assign16110_e11044_d_n6;
        locals.var_isbs2_sws_dn7 = assign16110_e11044_d_n7;
        locals.var_isbs2_sws_dn8 = assign16110_e11044_d_n8;
        locals.var_isbs2_sws_dn9 = assign16110_e11044_d_n9;
        locals.var_isbs2_sws_dn10 = assign16110_e11044_d_n10;
        locals.var_isbs2_sws_dn13 = assign16110_e11044_d_n13;

        let (assign16120_e11051, assign16120_e11051_d_n0, assign16120_e11051_d_n2, assign16120_e11051_d_n4, assign16120_e11051_d_n5, assign16120_e11051_d_n6, assign16120_e11051_d_n7, assign16120_e11051_d_n8, assign16120_e11051_d_n9, assign16120_e11051_d_n10, assign16120_e11051_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard332 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbs_swg, locals.var_isbs_swg_dn0, locals.var_isbs_swg_dn2, locals.var_isbs_swg_dn4, locals.var_isbs_swg_dn5, locals.var_isbs_swg_dn6, locals.var_isbs_swg_dn7, locals.var_isbs_swg_dn8, locals.var_isbs_swg_dn9, locals.var_isbs_swg_dn10, locals.var_isbs_swg_dn13,)
    }
};
        locals.var_isbs_swg = assign16120_e11051;
        locals.var_isbs_swg_dn0 = assign16120_e11051_d_n0;
        locals.var_isbs_swg_dn2 = assign16120_e11051_d_n2;
        locals.var_isbs_swg_dn4 = assign16120_e11051_d_n4;
        locals.var_isbs_swg_dn5 = assign16120_e11051_d_n5;
        locals.var_isbs_swg_dn6 = assign16120_e11051_d_n6;
        locals.var_isbs_swg_dn7 = assign16120_e11051_d_n7;
        locals.var_isbs_swg_dn8 = assign16120_e11051_d_n8;
        locals.var_isbs_swg_dn9 = assign16120_e11051_d_n9;
        locals.var_isbs_swg_dn10 = assign16120_e11051_d_n10;
        locals.var_isbs_swg_dn13 = assign16120_e11051_d_n13;

        let (assign16130_e11058, assign16130_e11058_d_n0, assign16130_e11058_d_n2, assign16130_e11058_d_n4, assign16130_e11058_d_n5, assign16130_e11058_d_n6, assign16130_e11058_d_n7, assign16130_e11058_d_n8, assign16130_e11058_d_n9, assign16130_e11058_d_n10, assign16130_e11058_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard332 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbs2_swg, locals.var_isbs2_swg_dn0, locals.var_isbs2_swg_dn2, locals.var_isbs2_swg_dn4, locals.var_isbs2_swg_dn5, locals.var_isbs2_swg_dn6, locals.var_isbs2_swg_dn7, locals.var_isbs2_swg_dn8, locals.var_isbs2_swg_dn9, locals.var_isbs2_swg_dn10, locals.var_isbs2_swg_dn13,)
    }
};
        locals.var_isbs2_swg = assign16130_e11058;
        locals.var_isbs2_swg_dn0 = assign16130_e11058_d_n0;
        locals.var_isbs2_swg_dn2 = assign16130_e11058_d_n2;
        locals.var_isbs2_swg_dn4 = assign16130_e11058_d_n4;
        locals.var_isbs2_swg_dn5 = assign16130_e11058_d_n5;
        locals.var_isbs2_swg_dn6 = assign16130_e11058_d_n6;
        locals.var_isbs2_swg_dn7 = assign16130_e11058_d_n7;
        locals.var_isbs2_swg_dn8 = assign16130_e11058_d_n8;
        locals.var_isbs2_swg_dn9 = assign16130_e11058_d_n9;
        locals.var_isbs2_swg_dn10 = assign16130_e11058_d_n10;
        locals.var_isbs2_swg_dn13 = assign16130_e11058_d_n13;

        let (assign16140_e11066, assign16140_e11066_d_n0, assign16140_e11066_d_n2, assign16140_e11066_d_n4, assign16140_e11066_d_n5, assign16140_e11066_d_n6, assign16140_e11066_d_n7, assign16140_e11066_d_n8, assign16140_e11066_d_n9, assign16140_e11066_d_n10, assign16140_e11066_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign16140_e11062: f64 = (locals.var_isbs_btm + locals.var_isbs_sws);
        let assign16140_e11064: f64 = (assign16140_e11062 + locals.var_isbs_swg);
        (assign16140_e11064, ((locals.var_isbs_btm_dn0 + locals.var_isbs_sws_dn0) + locals.var_isbs_swg_dn0), ((locals.var_isbs_btm_dn2 + locals.var_isbs_sws_dn2) + locals.var_isbs_swg_dn2), ((locals.var_isbs_btm_dn4 + locals.var_isbs_sws_dn4) + locals.var_isbs_swg_dn4), ((locals.var_isbs_btm_dn5 + locals.var_isbs_sws_dn5) + locals.var_isbs_swg_dn5), ((locals.var_isbs_btm_dn6 + locals.var_isbs_sws_dn6) + locals.var_isbs_swg_dn6), ((locals.var_isbs_btm_dn7 + locals.var_isbs_sws_dn7) + locals.var_isbs_swg_dn7), ((locals.var_isbs_btm_dn8 + locals.var_isbs_sws_dn8) + locals.var_isbs_swg_dn8), ((locals.var_isbs_btm_dn9 + locals.var_isbs_sws_dn9) + locals.var_isbs_swg_dn9), ((locals.var_isbs_btm_dn10 + locals.var_isbs_sws_dn10) + locals.var_isbs_swg_dn10), ((locals.var_isbs_btm_dn13 + locals.var_isbs_sws_dn13) + locals.var_isbs_swg_dn13),)
    } else {
        (locals.var_isbs, locals.var_isbs_dn0, locals.var_isbs_dn2, locals.var_isbs_dn4, locals.var_isbs_dn5, locals.var_isbs_dn6, locals.var_isbs_dn7, locals.var_isbs_dn8, locals.var_isbs_dn9, locals.var_isbs_dn10, locals.var_isbs_dn13,)
    }
};
        locals.var_isbs = assign16140_e11066;
        locals.var_isbs_dn0 = assign16140_e11066_d_n0;
        locals.var_isbs_dn2 = assign16140_e11066_d_n2;
        locals.var_isbs_dn4 = assign16140_e11066_d_n4;
        locals.var_isbs_dn5 = assign16140_e11066_d_n5;
        locals.var_isbs_dn6 = assign16140_e11066_d_n6;
        locals.var_isbs_dn7 = assign16140_e11066_d_n7;
        locals.var_isbs_dn8 = assign16140_e11066_d_n8;
        locals.var_isbs_dn9 = assign16140_e11066_d_n9;
        locals.var_isbs_dn10 = assign16140_e11066_d_n10;
        locals.var_isbs_dn13 = assign16140_e11066_d_n13;

        let assign16150_e11069: f64 = if locals.var_isbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard334 = assign16150_e11069;

        let (assign16160_e11077, assign16160_e11077_d_n0, assign16160_e11077_d_n2, assign16160_e11077_d_n4, assign16160_e11077_d_n5, assign16160_e11077_d_n6, assign16160_e11077_d_n7, assign16160_e11077_d_n8, assign16160_e11077_d_n9, assign16160_e11077_d_n10, assign16160_e11077_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard334 != 0.0)) {
        let assign16160_e11075: f64 = (locals.var_isbs + 1e-25);
        (assign16160_e11075, locals.var_isbs_dn0, locals.var_isbs_dn2, locals.var_isbs_dn4, locals.var_isbs_dn5, locals.var_isbs_dn6, locals.var_isbs_dn7, locals.var_isbs_dn8, locals.var_isbs_dn9, locals.var_isbs_dn10, locals.var_isbs_dn13,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign16160_e11077;
        locals.var_t3_dn0 = assign16160_e11077_d_n0;
        locals.var_t3_dn2 = assign16160_e11077_d_n2;
        locals.var_t3_dn4 = assign16160_e11077_d_n4;
        locals.var_t3_dn5 = assign16160_e11077_d_n5;
        locals.var_t3_dn6 = assign16160_e11077_d_n6;
        locals.var_t3_dn7 = assign16160_e11077_d_n7;
        locals.var_t3_dn8 = assign16160_e11077_d_n8;
        locals.var_t3_dn9 = assign16160_e11077_d_n9;
        locals.var_t3_dn10 = assign16160_e11077_d_n10;
        locals.var_t3_dn13 = assign16160_e11077_d_n13;

        let (assign16170_e11094, assign16170_e11094_d_n0, assign16170_e11094_d_n2, assign16170_e11094_d_n4, assign16170_e11094_d_n5, assign16170_e11094_d_n6, assign16170_e11094_d_n7, assign16170_e11094_d_n8, assign16170_e11094_d_n9, assign16170_e11094_d_n10, assign16170_e11094_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard334 != 0.0)) {
        let assign16170_e11083: f64 = (locals.var_uc_njs / locals.var_beta);
        let assign16170_e11086: f64 = (locals.var_uc_vdiffjs * locals.var_t0);
        let assign16170_e11088: f64 = (assign16170_e11086 / locals.var_t3);
        let assign16170_e11090: f64 = (assign16170_e11088 + 1.0);
        let assign16170_e11091: f64 = (assign16170_e11090).ln();
        let assign16170_e11092: f64 = (assign16170_e11083 * assign16170_e11091);
        (assign16170_e11092, (((-((locals.var_uc_njs * locals.var_beta_dn0) / (locals.var_beta * locals.var_beta))) * assign16170_e11091) + (assign16170_e11083 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn0) * locals.var_t3) - (assign16170_e11086 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)) / assign16170_e11090))), (((-((locals.var_uc_njs * locals.var_beta_dn2) / (locals.var_beta * locals.var_beta))) * assign16170_e11091) + (assign16170_e11083 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn2) * locals.var_t3) - (assign16170_e11086 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)) / assign16170_e11090))), (((-((locals.var_uc_njs * locals.var_beta_dn4) / (locals.var_beta * locals.var_beta))) * assign16170_e11091) + (assign16170_e11083 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn4) * locals.var_t3) - (assign16170_e11086 * locals.var_t3_dn4)) / (locals.var_t3 * locals.var_t3)) / assign16170_e11090))), (((-((locals.var_uc_njs * locals.var_beta_dn5) / (locals.var_beta * locals.var_beta))) * assign16170_e11091) + (assign16170_e11083 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn5) * locals.var_t3) - (assign16170_e11086 * locals.var_t3_dn5)) / (locals.var_t3 * locals.var_t3)) / assign16170_e11090))), (((-((locals.var_uc_njs * locals.var_beta_dn6) / (locals.var_beta * locals.var_beta))) * assign16170_e11091) + (assign16170_e11083 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn6) * locals.var_t3) - (assign16170_e11086 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)) / assign16170_e11090))), (((-((locals.var_uc_njs * locals.var_beta_dn7) / (locals.var_beta * locals.var_beta))) * assign16170_e11091) + (assign16170_e11083 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn7) * locals.var_t3) - (assign16170_e11086 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)) / assign16170_e11090))), (((-((locals.var_uc_njs * locals.var_beta_dn8) / (locals.var_beta * locals.var_beta))) * assign16170_e11091) + (assign16170_e11083 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn8) * locals.var_t3) - (assign16170_e11086 * locals.var_t3_dn8)) / (locals.var_t3 * locals.var_t3)) / assign16170_e11090))), (((-((locals.var_uc_njs * locals.var_beta_dn9) / (locals.var_beta * locals.var_beta))) * assign16170_e11091) + (assign16170_e11083 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn9) * locals.var_t3) - (assign16170_e11086 * locals.var_t3_dn9)) / (locals.var_t3 * locals.var_t3)) / assign16170_e11090))), (((-((locals.var_uc_njs * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) * assign16170_e11091) + (assign16170_e11083 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn10) * locals.var_t3) - (assign16170_e11086 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)) / assign16170_e11090))), (((-((locals.var_uc_njs * locals.var_beta_dn13) / (locals.var_beta * locals.var_beta))) * assign16170_e11091) + (assign16170_e11083 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn13) * locals.var_t3) - (assign16170_e11086 * locals.var_t3_dn13)) / (locals.var_t3 * locals.var_t3)) / assign16170_e11090))),)
    } else {
        (locals.var_vbst, locals.var_vbst_dn0, locals.var_vbst_dn2, locals.var_vbst_dn4, locals.var_vbst_dn5, locals.var_vbst_dn6, locals.var_vbst_dn7, locals.var_vbst_dn8, locals.var_vbst_dn9, locals.var_vbst_dn10, locals.var_vbst_dn13,)
    }
};
        locals.var_vbst = assign16170_e11094;
        locals.var_vbst_dn0 = assign16170_e11094_d_n0;
        locals.var_vbst_dn2 = assign16170_e11094_d_n2;
        locals.var_vbst_dn4 = assign16170_e11094_d_n4;
        locals.var_vbst_dn5 = assign16170_e11094_d_n5;
        locals.var_vbst_dn6 = assign16170_e11094_d_n6;
        locals.var_vbst_dn7 = assign16170_e11094_d_n7;
        locals.var_vbst_dn8 = assign16170_e11094_d_n8;
        locals.var_vbst_dn9 = assign16170_e11094_d_n9;
        locals.var_vbst_dn10 = assign16170_e11094_d_n10;
        locals.var_vbst_dn13 = assign16170_e11094_d_n13;

        let (assign16180_e11105, assign16180_e11105_d_n0, assign16180_e11105_d_n2, assign16180_e11105_d_n4, assign16180_e11105_d_n5, assign16180_e11105_d_n6, assign16180_e11105_d_n7, assign16180_e11105_d_n8, assign16180_e11105_d_n9, assign16180_e11105_d_n10, assign16180_e11105_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard334 != 0.0)) {
        let assign16180_e11100: f64 = (locals.var_tratio - 1.0);
        let assign16180_e11102: f64 = (assign16180_e11100 * p.p535);
        let assign16180_e11103: f64 = (assign16180_e11102).exp();
        (assign16180_e11103, (assign16180_e11103 * (locals.var_tratio_dn0 * p.p535)), (assign16180_e11103 * (locals.var_tratio_dn2 * p.p535)), (assign16180_e11103 * (locals.var_tratio_dn4 * p.p535)), (assign16180_e11103 * (locals.var_tratio_dn5 * p.p535)), (assign16180_e11103 * (locals.var_tratio_dn6 * p.p535)), (assign16180_e11103 * (locals.var_tratio_dn7 * p.p535)), (assign16180_e11103 * (locals.var_tratio_dn8 * p.p535)), (assign16180_e11103 * (locals.var_tratio_dn9 * p.p535)), (assign16180_e11103 * (locals.var_tratio_dn10 * p.p535)), (assign16180_e11103 * (locals.var_tratio_dn13 * p.p535)),)
    } else {
        (locals.var_exptemps, locals.var_exptemps_dn0, locals.var_exptemps_dn2, locals.var_exptemps_dn4, locals.var_exptemps_dn5, locals.var_exptemps_dn6, locals.var_exptemps_dn7, locals.var_exptemps_dn8, locals.var_exptemps_dn9, locals.var_exptemps_dn10, locals.var_exptemps_dn13,)
    }
};
        locals.var_exptemps = assign16180_e11105;
        locals.var_exptemps_dn0 = assign16180_e11105_d_n0;
        locals.var_exptemps_dn2 = assign16180_e11105_d_n2;
        locals.var_exptemps_dn4 = assign16180_e11105_d_n4;
        locals.var_exptemps_dn5 = assign16180_e11105_d_n5;
        locals.var_exptemps_dn6 = assign16180_e11105_d_n6;
        locals.var_exptemps_dn7 = assign16180_e11105_d_n7;
        locals.var_exptemps_dn8 = assign16180_e11105_d_n8;
        locals.var_exptemps_dn9 = assign16180_e11105_d_n9;
        locals.var_exptemps_dn10 = assign16180_e11105_d_n10;
        locals.var_exptemps_dn13 = assign16180_e11105_d_n13;

    }

    pub(super) fn stamp_transient_block_33(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign16190_e11115, assign16190_e11115_d_n0, assign16190_e11115_d_n2, assign16190_e11115_d_n4, assign16190_e11115_d_n5, assign16190_e11115_d_n6, assign16190_e11115_d_n7, assign16190_e11115_d_n8, assign16190_e11115_d_n9, assign16190_e11115_d_n10, assign16190_e11115_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard334 != 0.0)) {
        let assign16190_e11112: f64 = (locals.var_uc_njs / locals.var_beta);
        let assign16190_e11113: f64 = (1.0 / assign16190_e11112);
        (assign16190_e11113, (-((-((locals.var_uc_njs * locals.var_beta_dn0) / (locals.var_beta * locals.var_beta))) / (assign16190_e11112 * assign16190_e11112))), (-((-((locals.var_uc_njs * locals.var_beta_dn2) / (locals.var_beta * locals.var_beta))) / (assign16190_e11112 * assign16190_e11112))), (-((-((locals.var_uc_njs * locals.var_beta_dn4) / (locals.var_beta * locals.var_beta))) / (assign16190_e11112 * assign16190_e11112))), (-((-((locals.var_uc_njs * locals.var_beta_dn5) / (locals.var_beta * locals.var_beta))) / (assign16190_e11112 * assign16190_e11112))), (-((-((locals.var_uc_njs * locals.var_beta_dn6) / (locals.var_beta * locals.var_beta))) / (assign16190_e11112 * assign16190_e11112))), (-((-((locals.var_uc_njs * locals.var_beta_dn7) / (locals.var_beta * locals.var_beta))) / (assign16190_e11112 * assign16190_e11112))), (-((-((locals.var_uc_njs * locals.var_beta_dn8) / (locals.var_beta * locals.var_beta))) / (assign16190_e11112 * assign16190_e11112))), (-((-((locals.var_uc_njs * locals.var_beta_dn9) / (locals.var_beta * locals.var_beta))) / (assign16190_e11112 * assign16190_e11112))), (-((-((locals.var_uc_njs * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) / (assign16190_e11112 * assign16190_e11112))), (-((-((locals.var_uc_njs * locals.var_beta_dn13) / (locals.var_beta * locals.var_beta))) / (assign16190_e11112 * assign16190_e11112))),)
    } else {
        (locals.var_jd_nvtm_invs, locals.var_jd_nvtm_invs_dn0, locals.var_jd_nvtm_invs_dn2, locals.var_jd_nvtm_invs_dn4, locals.var_jd_nvtm_invs_dn5, locals.var_jd_nvtm_invs_dn6, locals.var_jd_nvtm_invs_dn7, locals.var_jd_nvtm_invs_dn8, locals.var_jd_nvtm_invs_dn9, locals.var_jd_nvtm_invs_dn10, locals.var_jd_nvtm_invs_dn13,)
    }
};
        locals.var_jd_nvtm_invs = assign16190_e11115;
        locals.var_jd_nvtm_invs_dn0 = assign16190_e11115_d_n0;
        locals.var_jd_nvtm_invs_dn2 = assign16190_e11115_d_n2;
        locals.var_jd_nvtm_invs_dn4 = assign16190_e11115_d_n4;
        locals.var_jd_nvtm_invs_dn5 = assign16190_e11115_d_n5;
        locals.var_jd_nvtm_invs_dn6 = assign16190_e11115_d_n6;
        locals.var_jd_nvtm_invs_dn7 = assign16190_e11115_d_n7;
        locals.var_jd_nvtm_invs_dn8 = assign16190_e11115_d_n8;
        locals.var_jd_nvtm_invs_dn9 = assign16190_e11115_d_n9;
        locals.var_jd_nvtm_invs_dn10 = assign16190_e11115_d_n10;
        locals.var_jd_nvtm_invs_dn13 = assign16190_e11115_d_n13;

        let (assign16200_e11124, assign16200_e11124_d_n0, assign16200_e11124_d_n2, assign16200_e11124_d_n4, assign16200_e11124_d_n5, assign16200_e11124_d_n6, assign16200_e11124_d_n7, assign16200_e11124_d_n8, assign16200_e11124_d_n9, assign16200_e11124_d_n10, assign16200_e11124_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard334 != 0.0)) {
        let assign16200_e11121: f64 = (locals.var_vbst * locals.var_jd_nvtm_invs);
        let assign16200_e11122: f64 = (assign16200_e11121).exp();
        (assign16200_e11122, (assign16200_e11122 * ((locals.var_vbst_dn0 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn0))), (assign16200_e11122 * ((locals.var_vbst_dn2 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn2))), (assign16200_e11122 * ((locals.var_vbst_dn4 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn4))), (assign16200_e11122 * ((locals.var_vbst_dn5 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn5))), (assign16200_e11122 * ((locals.var_vbst_dn6 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn6))), (assign16200_e11122 * ((locals.var_vbst_dn7 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn7))), (assign16200_e11122 * ((locals.var_vbst_dn8 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn8))), (assign16200_e11122 * ((locals.var_vbst_dn9 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn9))), (assign16200_e11122 * ((locals.var_vbst_dn10 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn10))), (assign16200_e11122 * ((locals.var_vbst_dn13 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn13))),)
    } else {
        (locals.var_jd_expcs, locals.var_jd_expcs_dn0, locals.var_jd_expcs_dn2, locals.var_jd_expcs_dn4, locals.var_jd_expcs_dn5, locals.var_jd_expcs_dn6, locals.var_jd_expcs_dn7, locals.var_jd_expcs_dn8, locals.var_jd_expcs_dn9, locals.var_jd_expcs_dn10, locals.var_jd_expcs_dn13,)
    }
};
        locals.var_jd_expcs = assign16200_e11124;
        locals.var_jd_expcs_dn0 = assign16200_e11124_d_n0;
        locals.var_jd_expcs_dn2 = assign16200_e11124_d_n2;
        locals.var_jd_expcs_dn4 = assign16200_e11124_d_n4;
        locals.var_jd_expcs_dn5 = assign16200_e11124_d_n5;
        locals.var_jd_expcs_dn6 = assign16200_e11124_d_n6;
        locals.var_jd_expcs_dn7 = assign16200_e11124_d_n7;
        locals.var_jd_expcs_dn8 = assign16200_e11124_d_n8;
        locals.var_jd_expcs_dn9 = assign16200_e11124_d_n9;
        locals.var_jd_expcs_dn10 = assign16200_e11124_d_n10;
        locals.var_jd_expcs_dn13 = assign16200_e11124_d_n13;

        let (assign16210_e11136, assign16210_e11136_d_n0, assign16210_e11136_d_n2, assign16210_e11136_d_n4, assign16210_e11136_d_n5, assign16210_e11136_d_n6, assign16210_e11136_d_n7, assign16210_e11136_d_n8, assign16210_e11136_d_n9, assign16210_e11136_d_n10, assign16210_e11136_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign16210_e11128: f64 = (p.p500 * p.p13);
        let assign16210_e11132: f64 = (p.p481 * locals.var_tdiff);
        let assign16210_e11133: f64 = (1.0 + assign16210_e11132);
        let assign16210_e11134: f64 = (assign16210_e11128 * assign16210_e11133);
        (assign16210_e11134, (assign16210_e11128 * (p.p481 * locals.var_tdiff_dn0)), (assign16210_e11128 * (p.p481 * locals.var_tdiff_dn2)), (assign16210_e11128 * (p.p481 * locals.var_tdiff_dn4)), (assign16210_e11128 * (p.p481 * locals.var_tdiff_dn5)), (assign16210_e11128 * (p.p481 * locals.var_tdiff_dn6)), (assign16210_e11128 * (p.p481 * locals.var_tdiff_dn7)), (assign16210_e11128 * (p.p481 * locals.var_tdiff_dn8)), (assign16210_e11128 * (p.p481 * locals.var_tdiff_dn9)), (assign16210_e11128 * (p.p481 * locals.var_tdiff_dn10)), (assign16210_e11128 * (p.p481 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_czbd, locals.var_czbd_dn0, locals.var_czbd_dn2, locals.var_czbd_dn4, locals.var_czbd_dn5, locals.var_czbd_dn6, locals.var_czbd_dn7, locals.var_czbd_dn8, locals.var_czbd_dn9, locals.var_czbd_dn10, locals.var_czbd_dn13,)
    }
};
        locals.var_czbd = assign16210_e11136;
        locals.var_czbd_dn0 = assign16210_e11136_d_n0;
        locals.var_czbd_dn2 = assign16210_e11136_d_n2;
        locals.var_czbd_dn4 = assign16210_e11136_d_n4;
        locals.var_czbd_dn5 = assign16210_e11136_d_n5;
        locals.var_czbd_dn6 = assign16210_e11136_d_n6;
        locals.var_czbd_dn7 = assign16210_e11136_d_n7;
        locals.var_czbd_dn8 = assign16210_e11136_d_n8;
        locals.var_czbd_dn9 = assign16210_e11136_d_n9;
        locals.var_czbd_dn10 = assign16210_e11136_d_n10;
        locals.var_czbd_dn13 = assign16210_e11136_d_n13;

        let assign16220_e11139: f64 = if p.p15 > locals.var_weff_nf { 1.0 } else { 0.0 };
        locals.var_guard335 = assign16220_e11139;

        let (assign16230_e11155, assign16230_e11155_d_n0, assign16230_e11155_d_n2, assign16230_e11155_d_n4, assign16230_e11155_d_n5, assign16230_e11155_d_n6, assign16230_e11155_d_n7, assign16230_e11155_d_n8, assign16230_e11155_d_n9, assign16230_e11155_d_n10, assign16230_e11155_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard335 != 0.0)) {
        let assign16230_e11146: f64 = (p.p15 - locals.var_weff_nf);
        let assign16230_e11147: f64 = (p.p501 * assign16230_e11146);
        let assign16230_e11151: f64 = (p.p483 * locals.var_tdiff);
        let assign16230_e11152: f64 = (1.0 + assign16230_e11151);
        let assign16230_e11153: f64 = (assign16230_e11147 * assign16230_e11152);
        (assign16230_e11153, (assign16230_e11147 * (p.p483 * locals.var_tdiff_dn0)), (assign16230_e11147 * (p.p483 * locals.var_tdiff_dn2)), (assign16230_e11147 * (p.p483 * locals.var_tdiff_dn4)), (assign16230_e11147 * (p.p483 * locals.var_tdiff_dn5)), (assign16230_e11147 * (p.p483 * locals.var_tdiff_dn6)), (assign16230_e11147 * (p.p483 * locals.var_tdiff_dn7)), (assign16230_e11147 * (p.p483 * locals.var_tdiff_dn8)), (assign16230_e11147 * (p.p483 * locals.var_tdiff_dn9)), (assign16230_e11147 * (p.p483 * locals.var_tdiff_dn10)), (assign16230_e11147 * (p.p483 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_czbdsw, locals.var_czbdsw_dn0, locals.var_czbdsw_dn2, locals.var_czbdsw_dn4, locals.var_czbdsw_dn5, locals.var_czbdsw_dn6, locals.var_czbdsw_dn7, locals.var_czbdsw_dn8, locals.var_czbdsw_dn9, locals.var_czbdsw_dn10, locals.var_czbdsw_dn13,)
    }
};
        locals.var_czbdsw = assign16230_e11155;
        locals.var_czbdsw_dn0 = assign16230_e11155_d_n0;
        locals.var_czbdsw_dn2 = assign16230_e11155_d_n2;
        locals.var_czbdsw_dn4 = assign16230_e11155_d_n4;
        locals.var_czbdsw_dn5 = assign16230_e11155_d_n5;
        locals.var_czbdsw_dn6 = assign16230_e11155_d_n6;
        locals.var_czbdsw_dn7 = assign16230_e11155_d_n7;
        locals.var_czbdsw_dn8 = assign16230_e11155_d_n8;
        locals.var_czbdsw_dn9 = assign16230_e11155_d_n9;
        locals.var_czbdsw_dn10 = assign16230_e11155_d_n10;
        locals.var_czbdsw_dn13 = assign16230_e11155_d_n13;

        let (assign16240_e11169, assign16240_e11169_d_n0, assign16240_e11169_d_n2, assign16240_e11169_d_n4, assign16240_e11169_d_n5, assign16240_e11169_d_n6, assign16240_e11169_d_n7, assign16240_e11169_d_n8, assign16240_e11169_d_n9, assign16240_e11169_d_n10, assign16240_e11169_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard335 != 0.0)) {
        let assign16240_e11161: f64 = (p.p502 * locals.var_weff_nf);
        let assign16240_e11165: f64 = (p.p485 * locals.var_tdiff);
        let assign16240_e11166: f64 = (1.0 + assign16240_e11165);
        let assign16240_e11167: f64 = (assign16240_e11161 * assign16240_e11166);
        (assign16240_e11167, (assign16240_e11161 * (p.p485 * locals.var_tdiff_dn0)), (assign16240_e11161 * (p.p485 * locals.var_tdiff_dn2)), (assign16240_e11161 * (p.p485 * locals.var_tdiff_dn4)), (assign16240_e11161 * (p.p485 * locals.var_tdiff_dn5)), (assign16240_e11161 * (p.p485 * locals.var_tdiff_dn6)), (assign16240_e11161 * (p.p485 * locals.var_tdiff_dn7)), (assign16240_e11161 * (p.p485 * locals.var_tdiff_dn8)), (assign16240_e11161 * (p.p485 * locals.var_tdiff_dn9)), (assign16240_e11161 * (p.p485 * locals.var_tdiff_dn10)), (assign16240_e11161 * (p.p485 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_czbdswg, locals.var_czbdswg_dn0, locals.var_czbdswg_dn2, locals.var_czbdswg_dn4, locals.var_czbdswg_dn5, locals.var_czbdswg_dn6, locals.var_czbdswg_dn7, locals.var_czbdswg_dn8, locals.var_czbdswg_dn9, locals.var_czbdswg_dn10, locals.var_czbdswg_dn13,)
    }
};
        locals.var_czbdswg = assign16240_e11169;
        locals.var_czbdswg_dn0 = assign16240_e11169_d_n0;
        locals.var_czbdswg_dn2 = assign16240_e11169_d_n2;
        locals.var_czbdswg_dn4 = assign16240_e11169_d_n4;
        locals.var_czbdswg_dn5 = assign16240_e11169_d_n5;
        locals.var_czbdswg_dn6 = assign16240_e11169_d_n6;
        locals.var_czbdswg_dn7 = assign16240_e11169_d_n7;
        locals.var_czbdswg_dn8 = assign16240_e11169_d_n8;
        locals.var_czbdswg_dn9 = assign16240_e11169_d_n9;
        locals.var_czbdswg_dn10 = assign16240_e11169_d_n10;
        locals.var_czbdswg_dn13 = assign16240_e11169_d_n13;

        let (assign16250_e11176, assign16250_e11176_d_n0, assign16250_e11176_d_n2, assign16250_e11176_d_n4, assign16250_e11176_d_n5, assign16250_e11176_d_n6, assign16250_e11176_d_n7, assign16250_e11176_d_n8, assign16250_e11176_d_n9, assign16250_e11176_d_n10, assign16250_e11176_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard335 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbdsw, locals.var_czbdsw_dn0, locals.var_czbdsw_dn2, locals.var_czbdsw_dn4, locals.var_czbdsw_dn5, locals.var_czbdsw_dn6, locals.var_czbdsw_dn7, locals.var_czbdsw_dn8, locals.var_czbdsw_dn9, locals.var_czbdsw_dn10, locals.var_czbdsw_dn13,)
    }
};
        locals.var_czbdsw = assign16250_e11176;
        locals.var_czbdsw_dn0 = assign16250_e11176_d_n0;
        locals.var_czbdsw_dn2 = assign16250_e11176_d_n2;
        locals.var_czbdsw_dn4 = assign16250_e11176_d_n4;
        locals.var_czbdsw_dn5 = assign16250_e11176_d_n5;
        locals.var_czbdsw_dn6 = assign16250_e11176_d_n6;
        locals.var_czbdsw_dn7 = assign16250_e11176_d_n7;
        locals.var_czbdsw_dn8 = assign16250_e11176_d_n8;
        locals.var_czbdsw_dn9 = assign16250_e11176_d_n9;
        locals.var_czbdsw_dn10 = assign16250_e11176_d_n10;
        locals.var_czbdsw_dn13 = assign16250_e11176_d_n13;

        let (assign16260_e11191, assign16260_e11191_d_n0, assign16260_e11191_d_n2, assign16260_e11191_d_n4, assign16260_e11191_d_n5, assign16260_e11191_d_n6, assign16260_e11191_d_n7, assign16260_e11191_d_n8, assign16260_e11191_d_n9, assign16260_e11191_d_n10, assign16260_e11191_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard335 == 0.0)) {
        let assign16260_e11183: f64 = (p.p502 * p.p15);
        let assign16260_e11187: f64 = (p.p485 * locals.var_tdiff);
        let assign16260_e11188: f64 = (1.0 + assign16260_e11187);
        let assign16260_e11189: f64 = (assign16260_e11183 * assign16260_e11188);
        (assign16260_e11189, (assign16260_e11183 * (p.p485 * locals.var_tdiff_dn0)), (assign16260_e11183 * (p.p485 * locals.var_tdiff_dn2)), (assign16260_e11183 * (p.p485 * locals.var_tdiff_dn4)), (assign16260_e11183 * (p.p485 * locals.var_tdiff_dn5)), (assign16260_e11183 * (p.p485 * locals.var_tdiff_dn6)), (assign16260_e11183 * (p.p485 * locals.var_tdiff_dn7)), (assign16260_e11183 * (p.p485 * locals.var_tdiff_dn8)), (assign16260_e11183 * (p.p485 * locals.var_tdiff_dn9)), (assign16260_e11183 * (p.p485 * locals.var_tdiff_dn10)), (assign16260_e11183 * (p.p485 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_czbdswg, locals.var_czbdswg_dn0, locals.var_czbdswg_dn2, locals.var_czbdswg_dn4, locals.var_czbdswg_dn5, locals.var_czbdswg_dn6, locals.var_czbdswg_dn7, locals.var_czbdswg_dn8, locals.var_czbdswg_dn9, locals.var_czbdswg_dn10, locals.var_czbdswg_dn13,)
    }
};
        locals.var_czbdswg = assign16260_e11191;
        locals.var_czbdswg_dn0 = assign16260_e11191_d_n0;
        locals.var_czbdswg_dn2 = assign16260_e11191_d_n2;
        locals.var_czbdswg_dn4 = assign16260_e11191_d_n4;
        locals.var_czbdswg_dn5 = assign16260_e11191_d_n5;
        locals.var_czbdswg_dn6 = assign16260_e11191_d_n6;
        locals.var_czbdswg_dn7 = assign16260_e11191_d_n7;
        locals.var_czbdswg_dn8 = assign16260_e11191_d_n8;
        locals.var_czbdswg_dn9 = assign16260_e11191_d_n9;
        locals.var_czbdswg_dn10 = assign16260_e11191_d_n10;
        locals.var_czbdswg_dn13 = assign16260_e11191_d_n13;

        let assign16270_e11194: f64 = if locals.var_czbd < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard336 = assign16270_e11194;

        let (assign16280_e11200, assign16280_e11200_d_n0, assign16280_e11200_d_n2, assign16280_e11200_d_n4, assign16280_e11200_d_n5, assign16280_e11200_d_n6, assign16280_e11200_d_n7, assign16280_e11200_d_n8, assign16280_e11200_d_n9, assign16280_e11200_d_n10, assign16280_e11200_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard336 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbd, locals.var_czbd_dn0, locals.var_czbd_dn2, locals.var_czbd_dn4, locals.var_czbd_dn5, locals.var_czbd_dn6, locals.var_czbd_dn7, locals.var_czbd_dn8, locals.var_czbd_dn9, locals.var_czbd_dn10, locals.var_czbd_dn13,)
    }
};
        locals.var_czbd = assign16280_e11200;
        locals.var_czbd_dn0 = assign16280_e11200_d_n0;
        locals.var_czbd_dn2 = assign16280_e11200_d_n2;
        locals.var_czbd_dn4 = assign16280_e11200_d_n4;
        locals.var_czbd_dn5 = assign16280_e11200_d_n5;
        locals.var_czbd_dn6 = assign16280_e11200_d_n6;
        locals.var_czbd_dn7 = assign16280_e11200_d_n7;
        locals.var_czbd_dn8 = assign16280_e11200_d_n8;
        locals.var_czbd_dn9 = assign16280_e11200_d_n9;
        locals.var_czbd_dn10 = assign16280_e11200_d_n10;
        locals.var_czbd_dn13 = assign16280_e11200_d_n13;

        let assign16290_e11203: f64 = if locals.var_czbdsw < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard337 = assign16290_e11203;

        let (assign16300_e11209, assign16300_e11209_d_n0, assign16300_e11209_d_n2, assign16300_e11209_d_n4, assign16300_e11209_d_n5, assign16300_e11209_d_n6, assign16300_e11209_d_n7, assign16300_e11209_d_n8, assign16300_e11209_d_n9, assign16300_e11209_d_n10, assign16300_e11209_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard337 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbdsw, locals.var_czbdsw_dn0, locals.var_czbdsw_dn2, locals.var_czbdsw_dn4, locals.var_czbdsw_dn5, locals.var_czbdsw_dn6, locals.var_czbdsw_dn7, locals.var_czbdsw_dn8, locals.var_czbdsw_dn9, locals.var_czbdsw_dn10, locals.var_czbdsw_dn13,)
    }
};
        locals.var_czbdsw = assign16300_e11209;
        locals.var_czbdsw_dn0 = assign16300_e11209_d_n0;
        locals.var_czbdsw_dn2 = assign16300_e11209_d_n2;
        locals.var_czbdsw_dn4 = assign16300_e11209_d_n4;
        locals.var_czbdsw_dn5 = assign16300_e11209_d_n5;
        locals.var_czbdsw_dn6 = assign16300_e11209_d_n6;
        locals.var_czbdsw_dn7 = assign16300_e11209_d_n7;
        locals.var_czbdsw_dn8 = assign16300_e11209_d_n8;
        locals.var_czbdsw_dn9 = assign16300_e11209_d_n9;
        locals.var_czbdsw_dn10 = assign16300_e11209_d_n10;
        locals.var_czbdsw_dn13 = assign16300_e11209_d_n13;

        let assign16310_e11212: f64 = if locals.var_czbdswg < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard338 = assign16310_e11212;

        let (assign16320_e11218, assign16320_e11218_d_n0, assign16320_e11218_d_n2, assign16320_e11218_d_n4, assign16320_e11218_d_n5, assign16320_e11218_d_n6, assign16320_e11218_d_n7, assign16320_e11218_d_n8, assign16320_e11218_d_n9, assign16320_e11218_d_n10, assign16320_e11218_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard338 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbdswg, locals.var_czbdswg_dn0, locals.var_czbdswg_dn2, locals.var_czbdswg_dn4, locals.var_czbdswg_dn5, locals.var_czbdswg_dn6, locals.var_czbdswg_dn7, locals.var_czbdswg_dn8, locals.var_czbdswg_dn9, locals.var_czbdswg_dn10, locals.var_czbdswg_dn13,)
    }
};
        locals.var_czbdswg = assign16320_e11218;
        locals.var_czbdswg_dn0 = assign16320_e11218_d_n0;
        locals.var_czbdswg_dn2 = assign16320_e11218_d_n2;
        locals.var_czbdswg_dn4 = assign16320_e11218_d_n4;
        locals.var_czbdswg_dn5 = assign16320_e11218_d_n5;
        locals.var_czbdswg_dn6 = assign16320_e11218_d_n6;
        locals.var_czbdswg_dn7 = assign16320_e11218_d_n7;
        locals.var_czbdswg_dn8 = assign16320_e11218_d_n8;
        locals.var_czbdswg_dn9 = assign16320_e11218_d_n9;
        locals.var_czbdswg_dn10 = assign16320_e11218_d_n10;
        locals.var_czbdswg_dn13 = assign16320_e11218_d_n13;

        let (assign16330_e11226, assign16330_e11226_d_n0, assign16330_e11226_d_n2, assign16330_e11226_d_n4, assign16330_e11226_d_n5, assign16330_e11226_d_n6, assign16330_e11226_d_n7, assign16330_e11226_d_n8, assign16330_e11226_d_n9, assign16330_e11226_d_n10, assign16330_e11226_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign16330_e11223: f64 = (p.p487 * locals.var_tdiff);
        let assign16330_e11224: f64 = (p.p506 - assign16330_e11223);
        (assign16330_e11224, (-(p.p487 * locals.var_tdiff_dn0)), (-(p.p487 * locals.var_tdiff_dn2)), (-(p.p487 * locals.var_tdiff_dn4)), (-(p.p487 * locals.var_tdiff_dn5)), (-(p.p487 * locals.var_tdiff_dn6)), (-(p.p487 * locals.var_tdiff_dn7)), (-(p.p487 * locals.var_tdiff_dn8)), (-(p.p487 * locals.var_tdiff_dn9)), (-(p.p487 * locals.var_tdiff_dn10)), (-(p.p487 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_pzbd, locals.var_pzbd_dn0, locals.var_pzbd_dn2, locals.var_pzbd_dn4, locals.var_pzbd_dn5, locals.var_pzbd_dn6, locals.var_pzbd_dn7, locals.var_pzbd_dn8, locals.var_pzbd_dn9, locals.var_pzbd_dn10, locals.var_pzbd_dn13,)
    }
};
        locals.var_pzbd = assign16330_e11226;
        locals.var_pzbd_dn0 = assign16330_e11226_d_n0;
        locals.var_pzbd_dn2 = assign16330_e11226_d_n2;
        locals.var_pzbd_dn4 = assign16330_e11226_d_n4;
        locals.var_pzbd_dn5 = assign16330_e11226_d_n5;
        locals.var_pzbd_dn6 = assign16330_e11226_d_n6;
        locals.var_pzbd_dn7 = assign16330_e11226_d_n7;
        locals.var_pzbd_dn8 = assign16330_e11226_d_n8;
        locals.var_pzbd_dn9 = assign16330_e11226_d_n9;
        locals.var_pzbd_dn10 = assign16330_e11226_d_n10;
        locals.var_pzbd_dn13 = assign16330_e11226_d_n13;

        let (assign16340_e11234, assign16340_e11234_d_n0, assign16340_e11234_d_n2, assign16340_e11234_d_n4, assign16340_e11234_d_n5, assign16340_e11234_d_n6, assign16340_e11234_d_n7, assign16340_e11234_d_n8, assign16340_e11234_d_n9, assign16340_e11234_d_n10, assign16340_e11234_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign16340_e11231: f64 = (p.p489 * locals.var_tdiff);
        let assign16340_e11232: f64 = (p.p507 - assign16340_e11231);
        (assign16340_e11232, (-(p.p489 * locals.var_tdiff_dn0)), (-(p.p489 * locals.var_tdiff_dn2)), (-(p.p489 * locals.var_tdiff_dn4)), (-(p.p489 * locals.var_tdiff_dn5)), (-(p.p489 * locals.var_tdiff_dn6)), (-(p.p489 * locals.var_tdiff_dn7)), (-(p.p489 * locals.var_tdiff_dn8)), (-(p.p489 * locals.var_tdiff_dn9)), (-(p.p489 * locals.var_tdiff_dn10)), (-(p.p489 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_pzbdsw, locals.var_pzbdsw_dn0, locals.var_pzbdsw_dn2, locals.var_pzbdsw_dn4, locals.var_pzbdsw_dn5, locals.var_pzbdsw_dn6, locals.var_pzbdsw_dn7, locals.var_pzbdsw_dn8, locals.var_pzbdsw_dn9, locals.var_pzbdsw_dn10, locals.var_pzbdsw_dn13,)
    }
};
        locals.var_pzbdsw = assign16340_e11234;
        locals.var_pzbdsw_dn0 = assign16340_e11234_d_n0;
        locals.var_pzbdsw_dn2 = assign16340_e11234_d_n2;
        locals.var_pzbdsw_dn4 = assign16340_e11234_d_n4;
        locals.var_pzbdsw_dn5 = assign16340_e11234_d_n5;
        locals.var_pzbdsw_dn6 = assign16340_e11234_d_n6;
        locals.var_pzbdsw_dn7 = assign16340_e11234_d_n7;
        locals.var_pzbdsw_dn8 = assign16340_e11234_d_n8;
        locals.var_pzbdsw_dn9 = assign16340_e11234_d_n9;
        locals.var_pzbdsw_dn10 = assign16340_e11234_d_n10;
        locals.var_pzbdsw_dn13 = assign16340_e11234_d_n13;

        let (assign16350_e11242, assign16350_e11242_d_n0, assign16350_e11242_d_n2, assign16350_e11242_d_n4, assign16350_e11242_d_n5, assign16350_e11242_d_n6, assign16350_e11242_d_n7, assign16350_e11242_d_n8, assign16350_e11242_d_n9, assign16350_e11242_d_n10, assign16350_e11242_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign16350_e11239: f64 = (p.p491 * locals.var_tdiff);
        let assign16350_e11240: f64 = (p.p508 - assign16350_e11239);
        (assign16350_e11240, (-(p.p491 * locals.var_tdiff_dn0)), (-(p.p491 * locals.var_tdiff_dn2)), (-(p.p491 * locals.var_tdiff_dn4)), (-(p.p491 * locals.var_tdiff_dn5)), (-(p.p491 * locals.var_tdiff_dn6)), (-(p.p491 * locals.var_tdiff_dn7)), (-(p.p491 * locals.var_tdiff_dn8)), (-(p.p491 * locals.var_tdiff_dn9)), (-(p.p491 * locals.var_tdiff_dn10)), (-(p.p491 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_pzbdswg, locals.var_pzbdswg_dn0, locals.var_pzbdswg_dn2, locals.var_pzbdswg_dn4, locals.var_pzbdswg_dn5, locals.var_pzbdswg_dn6, locals.var_pzbdswg_dn7, locals.var_pzbdswg_dn8, locals.var_pzbdswg_dn9, locals.var_pzbdswg_dn10, locals.var_pzbdswg_dn13,)
    }
};
        locals.var_pzbdswg = assign16350_e11242;
        locals.var_pzbdswg_dn0 = assign16350_e11242_d_n0;
        locals.var_pzbdswg_dn2 = assign16350_e11242_d_n2;
        locals.var_pzbdswg_dn4 = assign16350_e11242_d_n4;
        locals.var_pzbdswg_dn5 = assign16350_e11242_d_n5;
        locals.var_pzbdswg_dn6 = assign16350_e11242_d_n6;
        locals.var_pzbdswg_dn7 = assign16350_e11242_d_n7;
        locals.var_pzbdswg_dn8 = assign16350_e11242_d_n8;
        locals.var_pzbdswg_dn9 = assign16350_e11242_d_n9;
        locals.var_pzbdswg_dn10 = assign16350_e11242_d_n10;
        locals.var_pzbdswg_dn13 = assign16350_e11242_d_n13;

        let assign16360_e11249: f64 = if ((locals.var_pzbd < 0.01) && (p.p13 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard339 = assign16360_e11249;

        let (assign16370_e11255, assign16370_e11255_d_n0, assign16370_e11255_d_n2, assign16370_e11255_d_n4, assign16370_e11255_d_n5, assign16370_e11255_d_n6, assign16370_e11255_d_n7, assign16370_e11255_d_n8, assign16370_e11255_d_n9, assign16370_e11255_d_n10, assign16370_e11255_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard339 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbd, locals.var_pzbd_dn0, locals.var_pzbd_dn2, locals.var_pzbd_dn4, locals.var_pzbd_dn5, locals.var_pzbd_dn6, locals.var_pzbd_dn7, locals.var_pzbd_dn8, locals.var_pzbd_dn9, locals.var_pzbd_dn10, locals.var_pzbd_dn13,)
    }
};
        locals.var_pzbd = assign16370_e11255;
        locals.var_pzbd_dn0 = assign16370_e11255_d_n0;
        locals.var_pzbd_dn2 = assign16370_e11255_d_n2;
        locals.var_pzbd_dn4 = assign16370_e11255_d_n4;
        locals.var_pzbd_dn5 = assign16370_e11255_d_n5;
        locals.var_pzbd_dn6 = assign16370_e11255_d_n6;
        locals.var_pzbd_dn7 = assign16370_e11255_d_n7;
        locals.var_pzbd_dn8 = assign16370_e11255_d_n8;
        locals.var_pzbd_dn9 = assign16370_e11255_d_n9;
        locals.var_pzbd_dn10 = assign16370_e11255_d_n10;
        locals.var_pzbd_dn13 = assign16370_e11255_d_n13;

        let assign16380_e11262: f64 = if ((locals.var_pzbdsw < 0.01) && (p.p15 > locals.var_weff_nf)) { 1.0 } else { 0.0 };
        locals.var_guard340 = assign16380_e11262;

        let (assign16390_e11268, assign16390_e11268_d_n0, assign16390_e11268_d_n2, assign16390_e11268_d_n4, assign16390_e11268_d_n5, assign16390_e11268_d_n6, assign16390_e11268_d_n7, assign16390_e11268_d_n8, assign16390_e11268_d_n9, assign16390_e11268_d_n10, assign16390_e11268_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard340 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbdsw, locals.var_pzbdsw_dn0, locals.var_pzbdsw_dn2, locals.var_pzbdsw_dn4, locals.var_pzbdsw_dn5, locals.var_pzbdsw_dn6, locals.var_pzbdsw_dn7, locals.var_pzbdsw_dn8, locals.var_pzbdsw_dn9, locals.var_pzbdsw_dn10, locals.var_pzbdsw_dn13,)
    }
};
        locals.var_pzbdsw = assign16390_e11268;
        locals.var_pzbdsw_dn0 = assign16390_e11268_d_n0;
        locals.var_pzbdsw_dn2 = assign16390_e11268_d_n2;
        locals.var_pzbdsw_dn4 = assign16390_e11268_d_n4;
        locals.var_pzbdsw_dn5 = assign16390_e11268_d_n5;
        locals.var_pzbdsw_dn6 = assign16390_e11268_d_n6;
        locals.var_pzbdsw_dn7 = assign16390_e11268_d_n7;
        locals.var_pzbdsw_dn8 = assign16390_e11268_d_n8;
        locals.var_pzbdsw_dn9 = assign16390_e11268_d_n9;
        locals.var_pzbdsw_dn10 = assign16390_e11268_d_n10;
        locals.var_pzbdsw_dn13 = assign16390_e11268_d_n13;

        let assign16400_e11275: f64 = if ((locals.var_pzbdswg < 0.01) && (p.p15 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard341 = assign16400_e11275;

        let (assign16410_e11281, assign16410_e11281_d_n0, assign16410_e11281_d_n2, assign16410_e11281_d_n4, assign16410_e11281_d_n5, assign16410_e11281_d_n6, assign16410_e11281_d_n7, assign16410_e11281_d_n8, assign16410_e11281_d_n9, assign16410_e11281_d_n10, assign16410_e11281_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard341 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbdswg, locals.var_pzbdswg_dn0, locals.var_pzbdswg_dn2, locals.var_pzbdswg_dn4, locals.var_pzbdswg_dn5, locals.var_pzbdswg_dn6, locals.var_pzbdswg_dn7, locals.var_pzbdswg_dn8, locals.var_pzbdswg_dn9, locals.var_pzbdswg_dn10, locals.var_pzbdswg_dn13,)
    }
};
        locals.var_pzbdswg = assign16410_e11281;
        locals.var_pzbdswg_dn0 = assign16410_e11281_d_n0;
        locals.var_pzbdswg_dn2 = assign16410_e11281_d_n2;
        locals.var_pzbdswg_dn4 = assign16410_e11281_d_n4;
        locals.var_pzbdswg_dn5 = assign16410_e11281_d_n5;
        locals.var_pzbdswg_dn6 = assign16410_e11281_d_n6;
        locals.var_pzbdswg_dn7 = assign16410_e11281_d_n7;
        locals.var_pzbdswg_dn8 = assign16410_e11281_d_n8;
        locals.var_pzbdswg_dn9 = assign16410_e11281_d_n9;
        locals.var_pzbdswg_dn10 = assign16410_e11281_d_n10;
        locals.var_pzbdswg_dn13 = assign16410_e11281_d_n13;

        let (assign16420_e11293, assign16420_e11293_d_n0, assign16420_e11293_d_n2, assign16420_e11293_d_n4, assign16420_e11293_d_n5, assign16420_e11293_d_n6, assign16420_e11293_d_n7, assign16420_e11293_d_n8, assign16420_e11293_d_n9, assign16420_e11293_d_n10, assign16420_e11293_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign16420_e11285: f64 = (p.p523 * p.p14);
        let assign16420_e11289: f64 = (p.p482 * locals.var_tdiff);
        let assign16420_e11290: f64 = (1.0 + assign16420_e11289);
        let assign16420_e11291: f64 = (assign16420_e11285 * assign16420_e11290);
        (assign16420_e11291, (assign16420_e11285 * (p.p482 * locals.var_tdiff_dn0)), (assign16420_e11285 * (p.p482 * locals.var_tdiff_dn2)), (assign16420_e11285 * (p.p482 * locals.var_tdiff_dn4)), (assign16420_e11285 * (p.p482 * locals.var_tdiff_dn5)), (assign16420_e11285 * (p.p482 * locals.var_tdiff_dn6)), (assign16420_e11285 * (p.p482 * locals.var_tdiff_dn7)), (assign16420_e11285 * (p.p482 * locals.var_tdiff_dn8)), (assign16420_e11285 * (p.p482 * locals.var_tdiff_dn9)), (assign16420_e11285 * (p.p482 * locals.var_tdiff_dn10)), (assign16420_e11285 * (p.p482 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_czbs, locals.var_czbs_dn0, locals.var_czbs_dn2, locals.var_czbs_dn4, locals.var_czbs_dn5, locals.var_czbs_dn6, locals.var_czbs_dn7, locals.var_czbs_dn8, locals.var_czbs_dn9, locals.var_czbs_dn10, locals.var_czbs_dn13,)
    }
};
        locals.var_czbs = assign16420_e11293;
        locals.var_czbs_dn0 = assign16420_e11293_d_n0;
        locals.var_czbs_dn2 = assign16420_e11293_d_n2;
        locals.var_czbs_dn4 = assign16420_e11293_d_n4;
        locals.var_czbs_dn5 = assign16420_e11293_d_n5;
        locals.var_czbs_dn6 = assign16420_e11293_d_n6;
        locals.var_czbs_dn7 = assign16420_e11293_d_n7;
        locals.var_czbs_dn8 = assign16420_e11293_d_n8;
        locals.var_czbs_dn9 = assign16420_e11293_d_n9;
        locals.var_czbs_dn10 = assign16420_e11293_d_n10;
        locals.var_czbs_dn13 = assign16420_e11293_d_n13;

        let assign16430_e11296: f64 = if p.p16 > locals.var_weff_nf { 1.0 } else { 0.0 };
        locals.var_guard342 = assign16430_e11296;

        let (assign16440_e11312, assign16440_e11312_d_n0, assign16440_e11312_d_n2, assign16440_e11312_d_n4, assign16440_e11312_d_n5, assign16440_e11312_d_n6, assign16440_e11312_d_n7, assign16440_e11312_d_n8, assign16440_e11312_d_n9, assign16440_e11312_d_n10, assign16440_e11312_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard342 != 0.0)) {
        let assign16440_e11303: f64 = (p.p16 - locals.var_weff_nf);
        let assign16440_e11304: f64 = (p.p524 * assign16440_e11303);
        let assign16440_e11308: f64 = (p.p484 * locals.var_tdiff);
        let assign16440_e11309: f64 = (1.0 + assign16440_e11308);
        let assign16440_e11310: f64 = (assign16440_e11304 * assign16440_e11309);
        (assign16440_e11310, (assign16440_e11304 * (p.p484 * locals.var_tdiff_dn0)), (assign16440_e11304 * (p.p484 * locals.var_tdiff_dn2)), (assign16440_e11304 * (p.p484 * locals.var_tdiff_dn4)), (assign16440_e11304 * (p.p484 * locals.var_tdiff_dn5)), (assign16440_e11304 * (p.p484 * locals.var_tdiff_dn6)), (assign16440_e11304 * (p.p484 * locals.var_tdiff_dn7)), (assign16440_e11304 * (p.p484 * locals.var_tdiff_dn8)), (assign16440_e11304 * (p.p484 * locals.var_tdiff_dn9)), (assign16440_e11304 * (p.p484 * locals.var_tdiff_dn10)), (assign16440_e11304 * (p.p484 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_czbssw, locals.var_czbssw_dn0, locals.var_czbssw_dn2, locals.var_czbssw_dn4, locals.var_czbssw_dn5, locals.var_czbssw_dn6, locals.var_czbssw_dn7, locals.var_czbssw_dn8, locals.var_czbssw_dn9, locals.var_czbssw_dn10, locals.var_czbssw_dn13,)
    }
};
        locals.var_czbssw = assign16440_e11312;
        locals.var_czbssw_dn0 = assign16440_e11312_d_n0;
        locals.var_czbssw_dn2 = assign16440_e11312_d_n2;
        locals.var_czbssw_dn4 = assign16440_e11312_d_n4;
        locals.var_czbssw_dn5 = assign16440_e11312_d_n5;
        locals.var_czbssw_dn6 = assign16440_e11312_d_n6;
        locals.var_czbssw_dn7 = assign16440_e11312_d_n7;
        locals.var_czbssw_dn8 = assign16440_e11312_d_n8;
        locals.var_czbssw_dn9 = assign16440_e11312_d_n9;
        locals.var_czbssw_dn10 = assign16440_e11312_d_n10;
        locals.var_czbssw_dn13 = assign16440_e11312_d_n13;

        let (assign16450_e11326, assign16450_e11326_d_n0, assign16450_e11326_d_n2, assign16450_e11326_d_n4, assign16450_e11326_d_n5, assign16450_e11326_d_n6, assign16450_e11326_d_n7, assign16450_e11326_d_n8, assign16450_e11326_d_n9, assign16450_e11326_d_n10, assign16450_e11326_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard342 != 0.0)) {
        let assign16450_e11318: f64 = (p.p525 * locals.var_weff_nf);
        let assign16450_e11322: f64 = (p.p486 * locals.var_tdiff);
        let assign16450_e11323: f64 = (1.0 + assign16450_e11322);
        let assign16450_e11324: f64 = (assign16450_e11318 * assign16450_e11323);
        (assign16450_e11324, (assign16450_e11318 * (p.p486 * locals.var_tdiff_dn0)), (assign16450_e11318 * (p.p486 * locals.var_tdiff_dn2)), (assign16450_e11318 * (p.p486 * locals.var_tdiff_dn4)), (assign16450_e11318 * (p.p486 * locals.var_tdiff_dn5)), (assign16450_e11318 * (p.p486 * locals.var_tdiff_dn6)), (assign16450_e11318 * (p.p486 * locals.var_tdiff_dn7)), (assign16450_e11318 * (p.p486 * locals.var_tdiff_dn8)), (assign16450_e11318 * (p.p486 * locals.var_tdiff_dn9)), (assign16450_e11318 * (p.p486 * locals.var_tdiff_dn10)), (assign16450_e11318 * (p.p486 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_czbsswg, locals.var_czbsswg_dn0, locals.var_czbsswg_dn2, locals.var_czbsswg_dn4, locals.var_czbsswg_dn5, locals.var_czbsswg_dn6, locals.var_czbsswg_dn7, locals.var_czbsswg_dn8, locals.var_czbsswg_dn9, locals.var_czbsswg_dn10, locals.var_czbsswg_dn13,)
    }
};
        locals.var_czbsswg = assign16450_e11326;
        locals.var_czbsswg_dn0 = assign16450_e11326_d_n0;
        locals.var_czbsswg_dn2 = assign16450_e11326_d_n2;
        locals.var_czbsswg_dn4 = assign16450_e11326_d_n4;
        locals.var_czbsswg_dn5 = assign16450_e11326_d_n5;
        locals.var_czbsswg_dn6 = assign16450_e11326_d_n6;
        locals.var_czbsswg_dn7 = assign16450_e11326_d_n7;
        locals.var_czbsswg_dn8 = assign16450_e11326_d_n8;
        locals.var_czbsswg_dn9 = assign16450_e11326_d_n9;
        locals.var_czbsswg_dn10 = assign16450_e11326_d_n10;
        locals.var_czbsswg_dn13 = assign16450_e11326_d_n13;

        let (assign16460_e11333, assign16460_e11333_d_n0, assign16460_e11333_d_n2, assign16460_e11333_d_n4, assign16460_e11333_d_n5, assign16460_e11333_d_n6, assign16460_e11333_d_n7, assign16460_e11333_d_n8, assign16460_e11333_d_n9, assign16460_e11333_d_n10, assign16460_e11333_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard342 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbssw, locals.var_czbssw_dn0, locals.var_czbssw_dn2, locals.var_czbssw_dn4, locals.var_czbssw_dn5, locals.var_czbssw_dn6, locals.var_czbssw_dn7, locals.var_czbssw_dn8, locals.var_czbssw_dn9, locals.var_czbssw_dn10, locals.var_czbssw_dn13,)
    }
};
        locals.var_czbssw = assign16460_e11333;
        locals.var_czbssw_dn0 = assign16460_e11333_d_n0;
        locals.var_czbssw_dn2 = assign16460_e11333_d_n2;
        locals.var_czbssw_dn4 = assign16460_e11333_d_n4;
        locals.var_czbssw_dn5 = assign16460_e11333_d_n5;
        locals.var_czbssw_dn6 = assign16460_e11333_d_n6;
        locals.var_czbssw_dn7 = assign16460_e11333_d_n7;
        locals.var_czbssw_dn8 = assign16460_e11333_d_n8;
        locals.var_czbssw_dn9 = assign16460_e11333_d_n9;
        locals.var_czbssw_dn10 = assign16460_e11333_d_n10;
        locals.var_czbssw_dn13 = assign16460_e11333_d_n13;

        let (assign16470_e11348, assign16470_e11348_d_n0, assign16470_e11348_d_n2, assign16470_e11348_d_n4, assign16470_e11348_d_n5, assign16470_e11348_d_n6, assign16470_e11348_d_n7, assign16470_e11348_d_n8, assign16470_e11348_d_n9, assign16470_e11348_d_n10, assign16470_e11348_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard342 == 0.0)) {
        let assign16470_e11340: f64 = (p.p525 * p.p16);
        let assign16470_e11344: f64 = (p.p486 * locals.var_tdiff);
        let assign16470_e11345: f64 = (1.0 + assign16470_e11344);
        let assign16470_e11346: f64 = (assign16470_e11340 * assign16470_e11345);
        (assign16470_e11346, (assign16470_e11340 * (p.p486 * locals.var_tdiff_dn0)), (assign16470_e11340 * (p.p486 * locals.var_tdiff_dn2)), (assign16470_e11340 * (p.p486 * locals.var_tdiff_dn4)), (assign16470_e11340 * (p.p486 * locals.var_tdiff_dn5)), (assign16470_e11340 * (p.p486 * locals.var_tdiff_dn6)), (assign16470_e11340 * (p.p486 * locals.var_tdiff_dn7)), (assign16470_e11340 * (p.p486 * locals.var_tdiff_dn8)), (assign16470_e11340 * (p.p486 * locals.var_tdiff_dn9)), (assign16470_e11340 * (p.p486 * locals.var_tdiff_dn10)), (assign16470_e11340 * (p.p486 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_czbsswg, locals.var_czbsswg_dn0, locals.var_czbsswg_dn2, locals.var_czbsswg_dn4, locals.var_czbsswg_dn5, locals.var_czbsswg_dn6, locals.var_czbsswg_dn7, locals.var_czbsswg_dn8, locals.var_czbsswg_dn9, locals.var_czbsswg_dn10, locals.var_czbsswg_dn13,)
    }
};
        locals.var_czbsswg = assign16470_e11348;
        locals.var_czbsswg_dn0 = assign16470_e11348_d_n0;
        locals.var_czbsswg_dn2 = assign16470_e11348_d_n2;
        locals.var_czbsswg_dn4 = assign16470_e11348_d_n4;
        locals.var_czbsswg_dn5 = assign16470_e11348_d_n5;
        locals.var_czbsswg_dn6 = assign16470_e11348_d_n6;
        locals.var_czbsswg_dn7 = assign16470_e11348_d_n7;
        locals.var_czbsswg_dn8 = assign16470_e11348_d_n8;
        locals.var_czbsswg_dn9 = assign16470_e11348_d_n9;
        locals.var_czbsswg_dn10 = assign16470_e11348_d_n10;
        locals.var_czbsswg_dn13 = assign16470_e11348_d_n13;

        let assign16480_e11351: f64 = if locals.var_czbs < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard343 = assign16480_e11351;

        let (assign16490_e11357, assign16490_e11357_d_n0, assign16490_e11357_d_n2, assign16490_e11357_d_n4, assign16490_e11357_d_n5, assign16490_e11357_d_n6, assign16490_e11357_d_n7, assign16490_e11357_d_n8, assign16490_e11357_d_n9, assign16490_e11357_d_n10, assign16490_e11357_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard343 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbs, locals.var_czbs_dn0, locals.var_czbs_dn2, locals.var_czbs_dn4, locals.var_czbs_dn5, locals.var_czbs_dn6, locals.var_czbs_dn7, locals.var_czbs_dn8, locals.var_czbs_dn9, locals.var_czbs_dn10, locals.var_czbs_dn13,)
    }
};
        locals.var_czbs = assign16490_e11357;
        locals.var_czbs_dn0 = assign16490_e11357_d_n0;
        locals.var_czbs_dn2 = assign16490_e11357_d_n2;
        locals.var_czbs_dn4 = assign16490_e11357_d_n4;
        locals.var_czbs_dn5 = assign16490_e11357_d_n5;
        locals.var_czbs_dn6 = assign16490_e11357_d_n6;
        locals.var_czbs_dn7 = assign16490_e11357_d_n7;
        locals.var_czbs_dn8 = assign16490_e11357_d_n8;
        locals.var_czbs_dn9 = assign16490_e11357_d_n9;
        locals.var_czbs_dn10 = assign16490_e11357_d_n10;
        locals.var_czbs_dn13 = assign16490_e11357_d_n13;

        let assign16500_e11360: f64 = if locals.var_czbssw < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard344 = assign16500_e11360;

        let (assign16510_e11366, assign16510_e11366_d_n0, assign16510_e11366_d_n2, assign16510_e11366_d_n4, assign16510_e11366_d_n5, assign16510_e11366_d_n6, assign16510_e11366_d_n7, assign16510_e11366_d_n8, assign16510_e11366_d_n9, assign16510_e11366_d_n10, assign16510_e11366_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard344 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbssw, locals.var_czbssw_dn0, locals.var_czbssw_dn2, locals.var_czbssw_dn4, locals.var_czbssw_dn5, locals.var_czbssw_dn6, locals.var_czbssw_dn7, locals.var_czbssw_dn8, locals.var_czbssw_dn9, locals.var_czbssw_dn10, locals.var_czbssw_dn13,)
    }
};
        locals.var_czbssw = assign16510_e11366;
        locals.var_czbssw_dn0 = assign16510_e11366_d_n0;
        locals.var_czbssw_dn2 = assign16510_e11366_d_n2;
        locals.var_czbssw_dn4 = assign16510_e11366_d_n4;
        locals.var_czbssw_dn5 = assign16510_e11366_d_n5;
        locals.var_czbssw_dn6 = assign16510_e11366_d_n6;
        locals.var_czbssw_dn7 = assign16510_e11366_d_n7;
        locals.var_czbssw_dn8 = assign16510_e11366_d_n8;
        locals.var_czbssw_dn9 = assign16510_e11366_d_n9;
        locals.var_czbssw_dn10 = assign16510_e11366_d_n10;
        locals.var_czbssw_dn13 = assign16510_e11366_d_n13;

    }

    pub(super) fn stamp_transient_block_34(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let assign16520_e11369: f64 = if locals.var_czbsswg < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard345 = assign16520_e11369;

        let (assign16530_e11375, assign16530_e11375_d_n0, assign16530_e11375_d_n2, assign16530_e11375_d_n4, assign16530_e11375_d_n5, assign16530_e11375_d_n6, assign16530_e11375_d_n7, assign16530_e11375_d_n8, assign16530_e11375_d_n9, assign16530_e11375_d_n10, assign16530_e11375_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard345 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbsswg, locals.var_czbsswg_dn0, locals.var_czbsswg_dn2, locals.var_czbsswg_dn4, locals.var_czbsswg_dn5, locals.var_czbsswg_dn6, locals.var_czbsswg_dn7, locals.var_czbsswg_dn8, locals.var_czbsswg_dn9, locals.var_czbsswg_dn10, locals.var_czbsswg_dn13,)
    }
};
        locals.var_czbsswg = assign16530_e11375;
        locals.var_czbsswg_dn0 = assign16530_e11375_d_n0;
        locals.var_czbsswg_dn2 = assign16530_e11375_d_n2;
        locals.var_czbsswg_dn4 = assign16530_e11375_d_n4;
        locals.var_czbsswg_dn5 = assign16530_e11375_d_n5;
        locals.var_czbsswg_dn6 = assign16530_e11375_d_n6;
        locals.var_czbsswg_dn7 = assign16530_e11375_d_n7;
        locals.var_czbsswg_dn8 = assign16530_e11375_d_n8;
        locals.var_czbsswg_dn9 = assign16530_e11375_d_n9;
        locals.var_czbsswg_dn10 = assign16530_e11375_d_n10;
        locals.var_czbsswg_dn13 = assign16530_e11375_d_n13;

        let (assign16540_e11383, assign16540_e11383_d_n0, assign16540_e11383_d_n2, assign16540_e11383_d_n4, assign16540_e11383_d_n5, assign16540_e11383_d_n6, assign16540_e11383_d_n7, assign16540_e11383_d_n8, assign16540_e11383_d_n9, assign16540_e11383_d_n10, assign16540_e11383_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign16540_e11380: f64 = (p.p488 * locals.var_tdiff);
        let assign16540_e11381: f64 = (p.p529 - assign16540_e11380);
        (assign16540_e11381, (-(p.p488 * locals.var_tdiff_dn0)), (-(p.p488 * locals.var_tdiff_dn2)), (-(p.p488 * locals.var_tdiff_dn4)), (-(p.p488 * locals.var_tdiff_dn5)), (-(p.p488 * locals.var_tdiff_dn6)), (-(p.p488 * locals.var_tdiff_dn7)), (-(p.p488 * locals.var_tdiff_dn8)), (-(p.p488 * locals.var_tdiff_dn9)), (-(p.p488 * locals.var_tdiff_dn10)), (-(p.p488 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_pzbs, locals.var_pzbs_dn0, locals.var_pzbs_dn2, locals.var_pzbs_dn4, locals.var_pzbs_dn5, locals.var_pzbs_dn6, locals.var_pzbs_dn7, locals.var_pzbs_dn8, locals.var_pzbs_dn9, locals.var_pzbs_dn10, locals.var_pzbs_dn13,)
    }
};
        locals.var_pzbs = assign16540_e11383;
        locals.var_pzbs_dn0 = assign16540_e11383_d_n0;
        locals.var_pzbs_dn2 = assign16540_e11383_d_n2;
        locals.var_pzbs_dn4 = assign16540_e11383_d_n4;
        locals.var_pzbs_dn5 = assign16540_e11383_d_n5;
        locals.var_pzbs_dn6 = assign16540_e11383_d_n6;
        locals.var_pzbs_dn7 = assign16540_e11383_d_n7;
        locals.var_pzbs_dn8 = assign16540_e11383_d_n8;
        locals.var_pzbs_dn9 = assign16540_e11383_d_n9;
        locals.var_pzbs_dn10 = assign16540_e11383_d_n10;
        locals.var_pzbs_dn13 = assign16540_e11383_d_n13;

        let (assign16550_e11391, assign16550_e11391_d_n0, assign16550_e11391_d_n2, assign16550_e11391_d_n4, assign16550_e11391_d_n5, assign16550_e11391_d_n6, assign16550_e11391_d_n7, assign16550_e11391_d_n8, assign16550_e11391_d_n9, assign16550_e11391_d_n10, assign16550_e11391_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign16550_e11388: f64 = (p.p490 * locals.var_tdiff);
        let assign16550_e11389: f64 = (p.p530 - assign16550_e11388);
        (assign16550_e11389, (-(p.p490 * locals.var_tdiff_dn0)), (-(p.p490 * locals.var_tdiff_dn2)), (-(p.p490 * locals.var_tdiff_dn4)), (-(p.p490 * locals.var_tdiff_dn5)), (-(p.p490 * locals.var_tdiff_dn6)), (-(p.p490 * locals.var_tdiff_dn7)), (-(p.p490 * locals.var_tdiff_dn8)), (-(p.p490 * locals.var_tdiff_dn9)), (-(p.p490 * locals.var_tdiff_dn10)), (-(p.p490 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_pzbssw, locals.var_pzbssw_dn0, locals.var_pzbssw_dn2, locals.var_pzbssw_dn4, locals.var_pzbssw_dn5, locals.var_pzbssw_dn6, locals.var_pzbssw_dn7, locals.var_pzbssw_dn8, locals.var_pzbssw_dn9, locals.var_pzbssw_dn10, locals.var_pzbssw_dn13,)
    }
};
        locals.var_pzbssw = assign16550_e11391;
        locals.var_pzbssw_dn0 = assign16550_e11391_d_n0;
        locals.var_pzbssw_dn2 = assign16550_e11391_d_n2;
        locals.var_pzbssw_dn4 = assign16550_e11391_d_n4;
        locals.var_pzbssw_dn5 = assign16550_e11391_d_n5;
        locals.var_pzbssw_dn6 = assign16550_e11391_d_n6;
        locals.var_pzbssw_dn7 = assign16550_e11391_d_n7;
        locals.var_pzbssw_dn8 = assign16550_e11391_d_n8;
        locals.var_pzbssw_dn9 = assign16550_e11391_d_n9;
        locals.var_pzbssw_dn10 = assign16550_e11391_d_n10;
        locals.var_pzbssw_dn13 = assign16550_e11391_d_n13;

        let (assign16560_e11399, assign16560_e11399_d_n0, assign16560_e11399_d_n2, assign16560_e11399_d_n4, assign16560_e11399_d_n5, assign16560_e11399_d_n6, assign16560_e11399_d_n7, assign16560_e11399_d_n8, assign16560_e11399_d_n9, assign16560_e11399_d_n10, assign16560_e11399_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign16560_e11396: f64 = (p.p492 * locals.var_tdiff);
        let assign16560_e11397: f64 = (p.p531 - assign16560_e11396);
        (assign16560_e11397, (-(p.p492 * locals.var_tdiff_dn0)), (-(p.p492 * locals.var_tdiff_dn2)), (-(p.p492 * locals.var_tdiff_dn4)), (-(p.p492 * locals.var_tdiff_dn5)), (-(p.p492 * locals.var_tdiff_dn6)), (-(p.p492 * locals.var_tdiff_dn7)), (-(p.p492 * locals.var_tdiff_dn8)), (-(p.p492 * locals.var_tdiff_dn9)), (-(p.p492 * locals.var_tdiff_dn10)), (-(p.p492 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_pzbsswg, locals.var_pzbsswg_dn0, locals.var_pzbsswg_dn2, locals.var_pzbsswg_dn4, locals.var_pzbsswg_dn5, locals.var_pzbsswg_dn6, locals.var_pzbsswg_dn7, locals.var_pzbsswg_dn8, locals.var_pzbsswg_dn9, locals.var_pzbsswg_dn10, locals.var_pzbsswg_dn13,)
    }
};
        locals.var_pzbsswg = assign16560_e11399;
        locals.var_pzbsswg_dn0 = assign16560_e11399_d_n0;
        locals.var_pzbsswg_dn2 = assign16560_e11399_d_n2;
        locals.var_pzbsswg_dn4 = assign16560_e11399_d_n4;
        locals.var_pzbsswg_dn5 = assign16560_e11399_d_n5;
        locals.var_pzbsswg_dn6 = assign16560_e11399_d_n6;
        locals.var_pzbsswg_dn7 = assign16560_e11399_d_n7;
        locals.var_pzbsswg_dn8 = assign16560_e11399_d_n8;
        locals.var_pzbsswg_dn9 = assign16560_e11399_d_n9;
        locals.var_pzbsswg_dn10 = assign16560_e11399_d_n10;
        locals.var_pzbsswg_dn13 = assign16560_e11399_d_n13;

        let assign16570_e11406: f64 = if ((locals.var_pzbs < 0.01) && (p.p14 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard346 = assign16570_e11406;

        let (assign16580_e11412, assign16580_e11412_d_n0, assign16580_e11412_d_n2, assign16580_e11412_d_n4, assign16580_e11412_d_n5, assign16580_e11412_d_n6, assign16580_e11412_d_n7, assign16580_e11412_d_n8, assign16580_e11412_d_n9, assign16580_e11412_d_n10, assign16580_e11412_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard346 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbs, locals.var_pzbs_dn0, locals.var_pzbs_dn2, locals.var_pzbs_dn4, locals.var_pzbs_dn5, locals.var_pzbs_dn6, locals.var_pzbs_dn7, locals.var_pzbs_dn8, locals.var_pzbs_dn9, locals.var_pzbs_dn10, locals.var_pzbs_dn13,)
    }
};
        locals.var_pzbs = assign16580_e11412;
        locals.var_pzbs_dn0 = assign16580_e11412_d_n0;
        locals.var_pzbs_dn2 = assign16580_e11412_d_n2;
        locals.var_pzbs_dn4 = assign16580_e11412_d_n4;
        locals.var_pzbs_dn5 = assign16580_e11412_d_n5;
        locals.var_pzbs_dn6 = assign16580_e11412_d_n6;
        locals.var_pzbs_dn7 = assign16580_e11412_d_n7;
        locals.var_pzbs_dn8 = assign16580_e11412_d_n8;
        locals.var_pzbs_dn9 = assign16580_e11412_d_n9;
        locals.var_pzbs_dn10 = assign16580_e11412_d_n10;
        locals.var_pzbs_dn13 = assign16580_e11412_d_n13;

        let assign16590_e11419: f64 = if ((locals.var_pzbssw < 0.01) && (p.p16 > locals.var_weff_nf)) { 1.0 } else { 0.0 };
        locals.var_guard347 = assign16590_e11419;

        let (assign16600_e11425, assign16600_e11425_d_n0, assign16600_e11425_d_n2, assign16600_e11425_d_n4, assign16600_e11425_d_n5, assign16600_e11425_d_n6, assign16600_e11425_d_n7, assign16600_e11425_d_n8, assign16600_e11425_d_n9, assign16600_e11425_d_n10, assign16600_e11425_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard347 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbssw, locals.var_pzbssw_dn0, locals.var_pzbssw_dn2, locals.var_pzbssw_dn4, locals.var_pzbssw_dn5, locals.var_pzbssw_dn6, locals.var_pzbssw_dn7, locals.var_pzbssw_dn8, locals.var_pzbssw_dn9, locals.var_pzbssw_dn10, locals.var_pzbssw_dn13,)
    }
};
        locals.var_pzbssw = assign16600_e11425;
        locals.var_pzbssw_dn0 = assign16600_e11425_d_n0;
        locals.var_pzbssw_dn2 = assign16600_e11425_d_n2;
        locals.var_pzbssw_dn4 = assign16600_e11425_d_n4;
        locals.var_pzbssw_dn5 = assign16600_e11425_d_n5;
        locals.var_pzbssw_dn6 = assign16600_e11425_d_n6;
        locals.var_pzbssw_dn7 = assign16600_e11425_d_n7;
        locals.var_pzbssw_dn8 = assign16600_e11425_d_n8;
        locals.var_pzbssw_dn9 = assign16600_e11425_d_n9;
        locals.var_pzbssw_dn10 = assign16600_e11425_d_n10;
        locals.var_pzbssw_dn13 = assign16600_e11425_d_n13;

        let assign16610_e11432: f64 = if ((locals.var_pzbsswg < 0.01) && (p.p16 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard348 = assign16610_e11432;

        let (assign16620_e11438, assign16620_e11438_d_n0, assign16620_e11438_d_n2, assign16620_e11438_d_n4, assign16620_e11438_d_n5, assign16620_e11438_d_n6, assign16620_e11438_d_n7, assign16620_e11438_d_n8, assign16620_e11438_d_n9, assign16620_e11438_d_n10, assign16620_e11438_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard348 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbsswg, locals.var_pzbsswg_dn0, locals.var_pzbsswg_dn2, locals.var_pzbsswg_dn4, locals.var_pzbsswg_dn5, locals.var_pzbsswg_dn6, locals.var_pzbsswg_dn7, locals.var_pzbsswg_dn8, locals.var_pzbsswg_dn9, locals.var_pzbsswg_dn10, locals.var_pzbsswg_dn13,)
    }
};
        locals.var_pzbsswg = assign16620_e11438;
        locals.var_pzbsswg_dn0 = assign16620_e11438_d_n0;
        locals.var_pzbsswg_dn2 = assign16620_e11438_d_n2;
        locals.var_pzbsswg_dn4 = assign16620_e11438_d_n4;
        locals.var_pzbsswg_dn5 = assign16620_e11438_d_n5;
        locals.var_pzbsswg_dn6 = assign16620_e11438_d_n6;
        locals.var_pzbsswg_dn7 = assign16620_e11438_d_n7;
        locals.var_pzbsswg_dn8 = assign16620_e11438_d_n8;
        locals.var_pzbsswg_dn9 = assign16620_e11438_d_n9;
        locals.var_pzbsswg_dn10 = assign16620_e11438_d_n10;
        locals.var_pzbsswg_dn13 = assign16620_e11438_d_n13;

        let assign16630_e11441: f64 = (p.p87 * (nv5 - nv7));
        locals.var_vdsi = assign16630_e11441;
        locals.var_vdsi_dn5 = p.p87;
        locals.var_vdsi_dn7 = (-p.p87);

        let assign16640_e11444: f64 = (p.p87 * (nv6 - nv7));
        locals.var_vgsi = assign16640_e11444;
        locals.var_vgsi_dn6 = p.p87;
        locals.var_vgsi_dn7 = (-p.p87);

        let assign16650_e11447: f64 = (p.p87 * (nv8 - nv7));
        locals.var_vbsi = assign16650_e11447;
        locals.var_vbsi_dn7 = (-p.p87);
        locals.var_vbsi_dn8 = p.p87;

        let assign16660_e11450: f64 = (p.p87 * (nv0 - nv2));
        locals.var_vdsei = assign16660_e11450;
        locals.var_vdsei_dn0 = p.p87;
        locals.var_vdsei_dn2 = (-p.p87);

        let assign16670_e11453: f64 = (p.p87 * (nv6 - nv2));
        locals.var_vgsei = assign16670_e11453;
        locals.var_vgsei_dn2 = (-p.p87);
        locals.var_vgsei_dn6 = p.p87;

        let assign16680_e11456: f64 = (p.p87 * (nv8 - nv2));
        locals.var_vbsei = assign16680_e11456;
        locals.var_vbsei_dn2 = (-p.p87);
        locals.var_vbsei_dn8 = p.p87;

        let assign16690_e11459: f64 = (p.p87 * (nv0 - nv5));
        locals.var_vddp = assign16690_e11459;
        locals.var_vddp_dn0 = p.p87;
        locals.var_vddp_dn5 = (-p.p87);

        let assign16700_e11462: f64 = (p.p87 * (nv7 - nv2));
        locals.var_vsps = assign16700_e11462;
        locals.var_vsps_dn2 = (-p.p87);
        locals.var_vsps_dn7 = p.p87;

        let assign16710_e11465: f64 = (p.p87 * (nv10 - nv2));
        locals.var_vsbs = assign16710_e11465;
        locals.var_vsbs_dn2 = (-p.p87);
        locals.var_vsbs_dn10 = p.p87;

        let assign16720_e11468: f64 = (p.p87 * (nv9 - nv0));
        locals.var_vdbd = assign16720_e11468;
        locals.var_vdbd_dn0 = (-p.p87);
        locals.var_vdbd_dn9 = p.p87;

        let assign16730_e11471: f64 = (p.p87 * (nv8 - nv7));
        locals.var_vbpsp = assign16730_e11471;
        locals.var_vbpsp_dn7 = (-p.p87);
        locals.var_vbpsp_dn8 = p.p87;

        let assign16740_e11474: f64 = (p.p87 * (nv8 - nv5));
        locals.var_vbpdp = assign16740_e11474;
        locals.var_vbpdp_dn5 = (-p.p87);
        locals.var_vbpdp_dn8 = p.p87;

        locals.var_vbs_jct = locals.var_vsbs;
        locals.var_vbs_jct_dn2 = locals.var_vsbs_dn2;
        locals.var_vbs_jct_dn10 = locals.var_vsbs_dn10;

        locals.var_vbd_jct = locals.var_vdbd;
        locals.var_vbd_jct_dn0 = locals.var_vdbd_dn0;
        locals.var_vbd_jct_dn9 = locals.var_vdbd_dn9;

        locals.var_vbsi_jct = locals.var_vbpsp;
        locals.var_vbsi_jct_dn7 = locals.var_vbpsp_dn7;
        locals.var_vbsi_jct_dn8 = locals.var_vbpsp_dn8;

        locals.var_vbdi_jct = locals.var_vbpdp;
        locals.var_vbdi_jct_dn5 = locals.var_vbpdp_dn5;
        locals.var_vbdi_jct_dn8 = locals.var_vbpdp_dn8;

        locals.var_vsubs = 0.0;

        let (assign16800_e11483, assign16800_e11483_d_n11,) = {
    if (locals.var_flg_nqs != 0.0) {
        ((nv11 - 0.0), 1.0,)
    } else {
        (locals.var_qi_nqs, locals.var_qi_nqs_dn11,)
    }
};
        locals.var_qi_nqs = assign16800_e11483;
        locals.var_qi_nqs_dn11 = assign16800_e11483_d_n11;

        let (assign16810_e11487, assign16810_e11487_d_n12,) = {
    if (locals.var_flg_nqs != 0.0) {
        ((nv12 - 0.0), 1.0,)
    } else {
        (locals.var_qb_nqs, locals.var_qb_nqs_dn12,)
    }
};
        locals.var_qb_nqs = assign16810_e11487;
        locals.var_qb_nqs_dn12 = assign16810_e11487_d_n12;

        let (assign16820_e11492, assign16820_e11492_d_n11,) = {
    if (locals.var_flg_nqs == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_qi_nqs, locals.var_qi_nqs_dn11,)
    }
};
        locals.var_qi_nqs = assign16820_e11492;
        locals.var_qi_nqs_dn11 = assign16820_e11492_d_n11;

        let (assign16830_e11497, assign16830_e11497_d_n12,) = {
    if (locals.var_flg_nqs == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_qb_nqs, locals.var_qb_nqs_dn12,)
    }
};
        locals.var_qb_nqs = assign16830_e11497;
        locals.var_qb_nqs_dn12 = assign16830_e11497_d_n12;

        let assign16840_e11500: f64 = (locals.var_vgsi - locals.var_vdsi);
        locals.var_vgd = assign16840_e11500;
        locals.var_vgd_dn5 = (-locals.var_vdsi_dn5);
        locals.var_vgd_dn6 = locals.var_vgsi_dn6;
        locals.var_vgd_dn7 = (locals.var_vgsi_dn7 - locals.var_vdsi_dn7);

        let assign16850_e11503: f64 = (locals.var_vbsi - locals.var_vdsi);
        locals.var_vbd = assign16850_e11503;
        locals.var_vbd_dn5 = (-locals.var_vdsi_dn5);
        locals.var_vbd_dn7 = (locals.var_vbsi_dn7 - locals.var_vdsi_dn7);
        locals.var_vbd_dn8 = locals.var_vbsi_dn8;

        let assign16860_e11506: f64 = if locals.var_vdsi >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard349 = assign16860_e11506;

        let (assign16870_e11510,) = {
    if (locals.var_guard349 != 0.0) {
        (1.0,)
    } else {
        (locals.var_mode,)
    }
};
        locals.var_mode = assign16870_e11510;

        let (assign16880_e11514, assign16880_e11514_d_n0, assign16880_e11514_d_n2, assign16880_e11514_d_n4, assign16880_e11514_d_n5, assign16880_e11514_d_n6, assign16880_e11514_d_n7, assign16880_e11514_d_n8, assign16880_e11514_d_n9, assign16880_e11514_d_n10, assign16880_e11514_d_n13,) = {
    if (locals.var_guard349 != 0.0) {
        (locals.var_vdsi, 0.0, 0.0, 0.0, locals.var_vdsi_dn5, 0.0, locals.var_vdsi_dn7, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn13,)
    }
};
        locals.var_vds = assign16880_e11514;
        locals.var_vds_dn0 = assign16880_e11514_d_n0;
        locals.var_vds_dn2 = assign16880_e11514_d_n2;
        locals.var_vds_dn4 = assign16880_e11514_d_n4;
        locals.var_vds_dn5 = assign16880_e11514_d_n5;
        locals.var_vds_dn6 = assign16880_e11514_d_n6;
        locals.var_vds_dn7 = assign16880_e11514_d_n7;
        locals.var_vds_dn8 = assign16880_e11514_d_n8;
        locals.var_vds_dn9 = assign16880_e11514_d_n9;
        locals.var_vds_dn10 = assign16880_e11514_d_n10;
        locals.var_vds_dn13 = assign16880_e11514_d_n13;

        let (assign16890_e11518, assign16890_e11518_d_n5, assign16890_e11518_d_n6, assign16890_e11518_d_n7,) = {
    if (locals.var_guard349 != 0.0) {
        (locals.var_vgsi, 0.0, locals.var_vgsi_dn6, locals.var_vgsi_dn7,)
    } else {
        (locals.var_vgs, locals.var_vgs_dn5, locals.var_vgs_dn6, locals.var_vgs_dn7,)
    }
};
        locals.var_vgs = assign16890_e11518;
        locals.var_vgs_dn5 = assign16890_e11518_d_n5;
        locals.var_vgs_dn6 = assign16890_e11518_d_n6;
        locals.var_vgs_dn7 = assign16890_e11518_d_n7;

        let (assign16900_e11522, assign16900_e11522_d_n5, assign16900_e11522_d_n7, assign16900_e11522_d_n8,) = {
    if (locals.var_guard349 != 0.0) {
        (locals.var_vbsi, 0.0, locals.var_vbsi_dn7, locals.var_vbsi_dn8,)
    } else {
        (locals.var_vbs, locals.var_vbs_dn5, locals.var_vbs_dn7, locals.var_vbs_dn8,)
    }
};
        locals.var_vbs = assign16900_e11522;
        locals.var_vbs_dn5 = assign16900_e11522_d_n5;
        locals.var_vbs_dn7 = assign16900_e11522_d_n7;
        locals.var_vbs_dn8 = assign16900_e11522_d_n8;

        let (assign16910_e11526, assign16910_e11526_d_n0, assign16910_e11526_d_n2,) = {
    if (locals.var_guard349 != 0.0) {
        (locals.var_vdsei, locals.var_vdsei_dn0, locals.var_vdsei_dn2,)
    } else {
        (locals.var_vdse, locals.var_vdse_dn0, locals.var_vdse_dn2,)
    }
};
        locals.var_vdse = assign16910_e11526;
        locals.var_vdse_dn0 = assign16910_e11526_d_n0;
        locals.var_vdse_dn2 = assign16910_e11526_d_n2;

        let (assign16920_e11530, assign16920_e11530_d_n0, assign16920_e11530_d_n2, assign16920_e11530_d_n6,) = {
    if (locals.var_guard349 != 0.0) {
        (locals.var_vgsei, 0.0, locals.var_vgsei_dn2, locals.var_vgsei_dn6,)
    } else {
        (locals.var_vgse, locals.var_vgse_dn0, locals.var_vgse_dn2, locals.var_vgse_dn6,)
    }
};
        locals.var_vgse = assign16920_e11530;
        locals.var_vgse_dn0 = assign16920_e11530_d_n0;
        locals.var_vgse_dn2 = assign16920_e11530_d_n2;
        locals.var_vgse_dn6 = assign16920_e11530_d_n6;

        let (assign16930_e11534, assign16930_e11534_d_n0, assign16930_e11534_d_n2, assign16930_e11534_d_n8,) = {
    if (locals.var_guard349 != 0.0) {
        (locals.var_vbsei, 0.0, locals.var_vbsei_dn2, locals.var_vbsei_dn8,)
    } else {
        (locals.var_vbse, locals.var_vbse_dn0, locals.var_vbse_dn2, locals.var_vbse_dn8,)
    }
};
        locals.var_vbse = assign16930_e11534;
        locals.var_vbse_dn0 = assign16930_e11534_d_n0;
        locals.var_vbse_dn2 = assign16930_e11534_d_n2;
        locals.var_vbse_dn8 = assign16930_e11534_d_n8;

        let (assign16940_e11540,) = {
    if (locals.var_guard349 == 0.0) {
        let assign16940_e11538: f64 = (-1.0);
        (assign16940_e11538,)
    } else {
        (locals.var_mode,)
    }
};
        locals.var_mode = assign16940_e11540;

        let (assign16950_e11546, assign16950_e11546_d_n0, assign16950_e11546_d_n2, assign16950_e11546_d_n4, assign16950_e11546_d_n5, assign16950_e11546_d_n6, assign16950_e11546_d_n7, assign16950_e11546_d_n8, assign16950_e11546_d_n9, assign16950_e11546_d_n10, assign16950_e11546_d_n13,) = {
    if (locals.var_guard349 == 0.0) {
        let assign16950_e11544: f64 = (-locals.var_vdsi);
        (assign16950_e11544, 0.0, 0.0, 0.0, (-locals.var_vdsi_dn5), 0.0, (-locals.var_vdsi_dn7), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn13,)
    }
};
        locals.var_vds = assign16950_e11546;
        locals.var_vds_dn0 = assign16950_e11546_d_n0;
        locals.var_vds_dn2 = assign16950_e11546_d_n2;
        locals.var_vds_dn4 = assign16950_e11546_d_n4;
        locals.var_vds_dn5 = assign16950_e11546_d_n5;
        locals.var_vds_dn6 = assign16950_e11546_d_n6;
        locals.var_vds_dn7 = assign16950_e11546_d_n7;
        locals.var_vds_dn8 = assign16950_e11546_d_n8;
        locals.var_vds_dn9 = assign16950_e11546_d_n9;
        locals.var_vds_dn10 = assign16950_e11546_d_n10;
        locals.var_vds_dn13 = assign16950_e11546_d_n13;

        let (assign16960_e11551, assign16960_e11551_d_n5, assign16960_e11551_d_n6, assign16960_e11551_d_n7,) = {
    if (locals.var_guard349 == 0.0) {
        (locals.var_vgd, locals.var_vgd_dn5, locals.var_vgd_dn6, locals.var_vgd_dn7,)
    } else {
        (locals.var_vgs, locals.var_vgs_dn5, locals.var_vgs_dn6, locals.var_vgs_dn7,)
    }
};
        locals.var_vgs = assign16960_e11551;
        locals.var_vgs_dn5 = assign16960_e11551_d_n5;
        locals.var_vgs_dn6 = assign16960_e11551_d_n6;
        locals.var_vgs_dn7 = assign16960_e11551_d_n7;

        let (assign16970_e11556, assign16970_e11556_d_n5, assign16970_e11556_d_n7, assign16970_e11556_d_n8,) = {
    if (locals.var_guard349 == 0.0) {
        (locals.var_vbd, locals.var_vbd_dn5, locals.var_vbd_dn7, locals.var_vbd_dn8,)
    } else {
        (locals.var_vbs, locals.var_vbs_dn5, locals.var_vbs_dn7, locals.var_vbs_dn8,)
    }
};
        locals.var_vbs = assign16970_e11556;
        locals.var_vbs_dn5 = assign16970_e11556_d_n5;
        locals.var_vbs_dn7 = assign16970_e11556_d_n7;
        locals.var_vbs_dn8 = assign16970_e11556_d_n8;

        let (assign16980_e11562, assign16980_e11562_d_n0, assign16980_e11562_d_n2,) = {
    if (locals.var_guard349 == 0.0) {
        let assign16980_e11560: f64 = (-locals.var_vdsei);
        (assign16980_e11560, (-locals.var_vdsei_dn0), (-locals.var_vdsei_dn2),)
    } else {
        (locals.var_vdse, locals.var_vdse_dn0, locals.var_vdse_dn2,)
    }
};
        locals.var_vdse = assign16980_e11562;
        locals.var_vdse_dn0 = assign16980_e11562_d_n0;
        locals.var_vdse_dn2 = assign16980_e11562_d_n2;

        let (assign16990_e11569, assign16990_e11569_d_n0, assign16990_e11569_d_n2, assign16990_e11569_d_n6,) = {
    if (locals.var_guard349 == 0.0) {
        let assign16990_e11567: f64 = (locals.var_vgsei - locals.var_vdsei);
        (assign16990_e11567, (-locals.var_vdsei_dn0), (locals.var_vgsei_dn2 - locals.var_vdsei_dn2), locals.var_vgsei_dn6,)
    } else {
        (locals.var_vgse, locals.var_vgse_dn0, locals.var_vgse_dn2, locals.var_vgse_dn6,)
    }
};
        locals.var_vgse = assign16990_e11569;
        locals.var_vgse_dn0 = assign16990_e11569_d_n0;
        locals.var_vgse_dn2 = assign16990_e11569_d_n2;
        locals.var_vgse_dn6 = assign16990_e11569_d_n6;

        let (assign17000_e11576, assign17000_e11576_d_n0, assign17000_e11576_d_n2, assign17000_e11576_d_n8,) = {
    if (locals.var_guard349 == 0.0) {
        let assign17000_e11574: f64 = (locals.var_vbsei - locals.var_vdsei);
        (assign17000_e11574, (-locals.var_vdsei_dn0), (locals.var_vbsei_dn2 - locals.var_vdsei_dn2), locals.var_vbsei_dn8,)
    } else {
        (locals.var_vbse, locals.var_vbse_dn0, locals.var_vbse_dn2, locals.var_vbse_dn8,)
    }
};
        locals.var_vbse = assign17000_e11576;
        locals.var_vbse_dn0 = assign17000_e11576_d_n0;
        locals.var_vbse_dn2 = assign17000_e11576_d_n2;
        locals.var_vbse_dn8 = assign17000_e11576_d_n8;

        let assign17030_e11589: f64 = if ((p.p53 > 0.0) && (locals.var_uc_rth0 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard352 = assign17030_e11589;

        let (assign17040_e11593, assign17040_e11593_d_n0, assign17040_e11593_d_n2, assign17040_e11593_d_n4, assign17040_e11593_d_n5, assign17040_e11593_d_n6, assign17040_e11593_d_n7, assign17040_e11593_d_n8, assign17040_e11593_d_n9, assign17040_e11593_d_n10, assign17040_e11593_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        ((nv4 - 0.0), 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_deltemp, locals.var_deltemp_dn0, locals.var_deltemp_dn2, locals.var_deltemp_dn4, locals.var_deltemp_dn5, locals.var_deltemp_dn6, locals.var_deltemp_dn7, locals.var_deltemp_dn8, locals.var_deltemp_dn9, locals.var_deltemp_dn10, locals.var_deltemp_dn13,)
    }
};
        locals.var_deltemp = assign17040_e11593;
        locals.var_deltemp_dn0 = assign17040_e11593_d_n0;
        locals.var_deltemp_dn2 = assign17040_e11593_d_n2;
        locals.var_deltemp_dn4 = assign17040_e11593_d_n4;
        locals.var_deltemp_dn5 = assign17040_e11593_d_n5;
        locals.var_deltemp_dn6 = assign17040_e11593_d_n6;
        locals.var_deltemp_dn7 = assign17040_e11593_d_n7;
        locals.var_deltemp_dn8 = assign17040_e11593_d_n8;
        locals.var_deltemp_dn9 = assign17040_e11593_d_n9;
        locals.var_deltemp_dn10 = assign17040_e11593_d_n10;
        locals.var_deltemp_dn13 = assign17040_e11593_d_n13;

        let assign17050_e11596: f64 = if p.p53 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard353 = assign17050_e11596;

        let (assign17060_e11608, assign17060_e11608_d_n0, assign17060_e11608_d_n2, assign17060_e11608_d_n4, assign17060_e11608_d_n5, assign17060_e11608_d_n6, assign17060_e11608_d_n7, assign17060_e11608_d_n8, assign17060_e11608_d_n9, assign17060_e11608_d_n10, assign17060_e11608_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard353 != 0.0)) {
        let assign17060_e11602: f64 = (p.p433 - locals.var_deltemp);
        let assign17060_e11605: f64 = (p.p337 * 10.0);
        let assign17060_e11606: f64 = (assign17060_e11602 - assign17060_e11605);
        (assign17060_e11606, (-locals.var_deltemp_dn0), (-locals.var_deltemp_dn2), (-locals.var_deltemp_dn4), (-locals.var_deltemp_dn5), (-locals.var_deltemp_dn6), (-locals.var_deltemp_dn7), (-locals.var_deltemp_dn8), (-locals.var_deltemp_dn9), (-locals.var_deltemp_dn10), (-locals.var_deltemp_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign17060_e11608;
        locals.var_tmf1_dn0 = assign17060_e11608_d_n0;
        locals.var_tmf1_dn2 = assign17060_e11608_d_n2;
        locals.var_tmf1_dn4 = assign17060_e11608_d_n4;
        locals.var_tmf1_dn5 = assign17060_e11608_d_n5;
        locals.var_tmf1_dn6 = assign17060_e11608_d_n6;
        locals.var_tmf1_dn7 = assign17060_e11608_d_n7;
        locals.var_tmf1_dn8 = assign17060_e11608_d_n8;
        locals.var_tmf1_dn9 = assign17060_e11608_d_n9;
        locals.var_tmf1_dn10 = assign17060_e11608_d_n10;
        locals.var_tmf1_dn13 = assign17060_e11608_d_n13;

    }

    pub(super) fn stamp_transient_block_35(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let (assign17070_e11620, assign17070_e11620_d_n0, assign17070_e11620_d_n2, assign17070_e11620_d_n4, assign17070_e11620_d_n5, assign17070_e11620_d_n6, assign17070_e11620_d_n7, assign17070_e11620_d_n8, assign17070_e11620_d_n9, assign17070_e11620_d_n10, assign17070_e11620_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard353 != 0.0)) {
        let assign17070_e11614: f64 = (4.0 * p.p433);
        let assign17070_e11617: f64 = (p.p337 * 10.0);
        let assign17070_e11618: f64 = (assign17070_e11614 * assign17070_e11617);
        (assign17070_e11618, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign17070_e11620;
        locals.var_tmf2_dn0 = assign17070_e11620_d_n0;
        locals.var_tmf2_dn2 = assign17070_e11620_d_n2;
        locals.var_tmf2_dn4 = assign17070_e11620_d_n4;
        locals.var_tmf2_dn5 = assign17070_e11620_d_n5;
        locals.var_tmf2_dn6 = assign17070_e11620_d_n6;
        locals.var_tmf2_dn7 = assign17070_e11620_d_n7;
        locals.var_tmf2_dn8 = assign17070_e11620_d_n8;
        locals.var_tmf2_dn9 = assign17070_e11620_d_n9;
        locals.var_tmf2_dn10 = assign17070_e11620_d_n10;
        locals.var_tmf2_dn13 = assign17070_e11620_d_n13;

        let (assign17080_e11632, assign17080_e11632_d_n0, assign17080_e11632_d_n2, assign17080_e11632_d_n4, assign17080_e11632_d_n5, assign17080_e11632_d_n6, assign17080_e11632_d_n7, assign17080_e11632_d_n8, assign17080_e11632_d_n9, assign17080_e11632_d_n10, assign17080_e11632_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard353 != 0.0)) {
        let (assign17080_e11630, assign17080_e11630_d_n0, assign17080_e11630_d_n2, assign17080_e11630_d_n4, assign17080_e11630_d_n5, assign17080_e11630_d_n6, assign17080_e11630_d_n7, assign17080_e11630_d_n8, assign17080_e11630_d_n9, assign17080_e11630_d_n10, assign17080_e11630_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign17080_e11629: f64 = (-locals.var_tmf2);
                (assign17080_e11629, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign17080_e11630, assign17080_e11630_d_n0, assign17080_e11630_d_n2, assign17080_e11630_d_n4, assign17080_e11630_d_n5, assign17080_e11630_d_n6, assign17080_e11630_d_n7, assign17080_e11630_d_n8, assign17080_e11630_d_n9, assign17080_e11630_d_n10, assign17080_e11630_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign17080_e11632;
        locals.var_tmf2_dn0 = assign17080_e11632_d_n0;
        locals.var_tmf2_dn2 = assign17080_e11632_d_n2;
        locals.var_tmf2_dn4 = assign17080_e11632_d_n4;
        locals.var_tmf2_dn5 = assign17080_e11632_d_n5;
        locals.var_tmf2_dn6 = assign17080_e11632_d_n6;
        locals.var_tmf2_dn7 = assign17080_e11632_d_n7;
        locals.var_tmf2_dn8 = assign17080_e11632_d_n8;
        locals.var_tmf2_dn9 = assign17080_e11632_d_n9;
        locals.var_tmf2_dn10 = assign17080_e11632_d_n10;
        locals.var_tmf2_dn13 = assign17080_e11632_d_n13;

        let (assign17090_e11643, assign17090_e11643_d_n0, assign17090_e11643_d_n2, assign17090_e11643_d_n4, assign17090_e11643_d_n5, assign17090_e11643_d_n6, assign17090_e11643_d_n7, assign17090_e11643_d_n8, assign17090_e11643_d_n9, assign17090_e11643_d_n10, assign17090_e11643_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard353 != 0.0)) {
        let assign17090_e11638: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign17090_e11640: f64 = (assign17090_e11638 + locals.var_tmf2);
        let assign17090_e11641: f64 = (assign17090_e11640).sqrt();
        (assign17090_e11641, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign17090_e11641)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign17090_e11641)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign17090_e11641)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign17090_e11641)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign17090_e11641)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign17090_e11641)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign17090_e11641)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign17090_e11641)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign17090_e11641)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign17090_e11641)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign17090_e11643;
        locals.var_tmf2_dn0 = assign17090_e11643_d_n0;
        locals.var_tmf2_dn2 = assign17090_e11643_d_n2;
        locals.var_tmf2_dn4 = assign17090_e11643_d_n4;
        locals.var_tmf2_dn5 = assign17090_e11643_d_n5;
        locals.var_tmf2_dn6 = assign17090_e11643_d_n6;
        locals.var_tmf2_dn7 = assign17090_e11643_d_n7;
        locals.var_tmf2_dn8 = assign17090_e11643_d_n8;
        locals.var_tmf2_dn9 = assign17090_e11643_d_n9;
        locals.var_tmf2_dn10 = assign17090_e11643_d_n10;
        locals.var_tmf2_dn13 = assign17090_e11643_d_n13;

        let (assign17100_e11655, assign17100_e11655_d_n0, assign17100_e11655_d_n2, assign17100_e11655_d_n4, assign17100_e11655_d_n5, assign17100_e11655_d_n6, assign17100_e11655_d_n7, assign17100_e11655_d_n8, assign17100_e11655_d_n9, assign17100_e11655_d_n10, assign17100_e11655_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard353 != 0.0)) {
        let assign17100_e11651: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign17100_e11652: f64 = (1.0 + assign17100_e11651);
        let assign17100_e11653: f64 = (0.5 * assign17100_e11652);
        (assign17100_e11653, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign17100_e11655;
        locals.var_t0_dn0 = assign17100_e11655_d_n0;
        locals.var_t0_dn2 = assign17100_e11655_d_n2;
        locals.var_t0_dn4 = assign17100_e11655_d_n4;
        locals.var_t0_dn5 = assign17100_e11655_d_n5;
        locals.var_t0_dn6 = assign17100_e11655_d_n6;
        locals.var_t0_dn7 = assign17100_e11655_d_n7;
        locals.var_t0_dn8 = assign17100_e11655_d_n8;
        locals.var_t0_dn9 = assign17100_e11655_d_n9;
        locals.var_t0_dn10 = assign17100_e11655_d_n10;
        locals.var_t0_dn13 = assign17100_e11655_d_n13;

        let (assign17110_e11667, assign17110_e11667_d_n0, assign17110_e11667_d_n2, assign17110_e11667_d_n4, assign17110_e11667_d_n5, assign17110_e11667_d_n6, assign17110_e11667_d_n7, assign17110_e11667_d_n8, assign17110_e11667_d_n9, assign17110_e11667_d_n10, assign17110_e11667_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard353 != 0.0)) {
        let assign17110_e11663: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign17110_e11664: f64 = (0.5 * assign17110_e11663);
        let assign17110_e11665: f64 = (p.p433 - assign17110_e11664);
        (assign17110_e11665, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_deltemp, locals.var_deltemp_dn0, locals.var_deltemp_dn2, locals.var_deltemp_dn4, locals.var_deltemp_dn5, locals.var_deltemp_dn6, locals.var_deltemp_dn7, locals.var_deltemp_dn8, locals.var_deltemp_dn9, locals.var_deltemp_dn10, locals.var_deltemp_dn13,)
    }
};
        locals.var_deltemp = assign17110_e11667;
        locals.var_deltemp_dn0 = assign17110_e11667_d_n0;
        locals.var_deltemp_dn2 = assign17110_e11667_d_n2;
        locals.var_deltemp_dn4 = assign17110_e11667_d_n4;
        locals.var_deltemp_dn5 = assign17110_e11667_d_n5;
        locals.var_deltemp_dn6 = assign17110_e11667_d_n6;
        locals.var_deltemp_dn7 = assign17110_e11667_d_n7;
        locals.var_deltemp_dn8 = assign17110_e11667_d_n8;
        locals.var_deltemp_dn9 = assign17110_e11667_d_n9;
        locals.var_deltemp_dn10 = assign17110_e11667_d_n10;
        locals.var_deltemp_dn13 = assign17110_e11667_d_n13;

        let (assign17130_e11676, assign17130_e11676_d_n0, assign17130_e11676_d_n2, assign17130_e11676_d_n4, assign17130_e11676_d_n5, assign17130_e11676_d_n6, assign17130_e11676_d_n7, assign17130_e11676_d_n8, assign17130_e11676_d_n9, assign17130_e11676_d_n10, assign17130_e11676_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17130_e11672: f64 = ctx_temp;
        let assign17130_e11674: f64 = (assign17130_e11672 + p.p11);
        (assign17130_e11674, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ttemp, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn13,)
    }
};
        locals.var_ttemp = assign17130_e11676;
        locals.var_ttemp_dn0 = assign17130_e11676_d_n0;
        locals.var_ttemp_dn2 = assign17130_e11676_d_n2;
        locals.var_ttemp_dn4 = assign17130_e11676_d_n4;
        locals.var_ttemp_dn5 = assign17130_e11676_d_n5;
        locals.var_ttemp_dn6 = assign17130_e11676_d_n6;
        locals.var_ttemp_dn7 = assign17130_e11676_d_n7;
        locals.var_ttemp_dn8 = assign17130_e11676_d_n8;
        locals.var_ttemp_dn9 = assign17130_e11676_d_n9;
        locals.var_ttemp_dn10 = assign17130_e11676_d_n10;
        locals.var_ttemp_dn13 = assign17130_e11676_d_n13;

        let (assign17140_e11680, assign17140_e11680_d_n0, assign17140_e11680_d_n2, assign17140_e11680_d_n4, assign17140_e11680_d_n5, assign17140_e11680_d_n6, assign17140_e11680_d_n7, assign17140_e11680_d_n8, assign17140_e11680_d_n9, assign17140_e11680_d_n10, assign17140_e11680_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        (locals.var_ttemp, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn13,)
    } else {
        (locals.var_ttemp0, locals.var_ttemp0_dn0, locals.var_ttemp0_dn2, locals.var_ttemp0_dn4, locals.var_ttemp0_dn5, locals.var_ttemp0_dn6, locals.var_ttemp0_dn7, locals.var_ttemp0_dn8, locals.var_ttemp0_dn9, locals.var_ttemp0_dn10, locals.var_ttemp0_dn13,)
    }
};
        locals.var_ttemp0 = assign17140_e11680;
        locals.var_ttemp0_dn0 = assign17140_e11680_d_n0;
        locals.var_ttemp0_dn2 = assign17140_e11680_d_n2;
        locals.var_ttemp0_dn4 = assign17140_e11680_d_n4;
        locals.var_ttemp0_dn5 = assign17140_e11680_d_n5;
        locals.var_ttemp0_dn6 = assign17140_e11680_d_n6;
        locals.var_ttemp0_dn7 = assign17140_e11680_d_n7;
        locals.var_ttemp0_dn8 = assign17140_e11680_d_n8;
        locals.var_ttemp0_dn9 = assign17140_e11680_d_n9;
        locals.var_ttemp0_dn10 = assign17140_e11680_d_n10;
        locals.var_ttemp0_dn13 = assign17140_e11680_d_n13;

        let (assign17150_e11686, assign17150_e11686_d_n0, assign17150_e11686_d_n2, assign17150_e11686_d_n4, assign17150_e11686_d_n5, assign17150_e11686_d_n6, assign17150_e11686_d_n7, assign17150_e11686_d_n8, assign17150_e11686_d_n9, assign17150_e11686_d_n10, assign17150_e11686_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17150_e11684: f64 = (locals.var_ttemp + locals.var_deltemp);
        (assign17150_e11684, (locals.var_ttemp_dn0 + locals.var_deltemp_dn0), (locals.var_ttemp_dn2 + locals.var_deltemp_dn2), (locals.var_ttemp_dn4 + locals.var_deltemp_dn4), (locals.var_ttemp_dn5 + locals.var_deltemp_dn5), (locals.var_ttemp_dn6 + locals.var_deltemp_dn6), (locals.var_ttemp_dn7 + locals.var_deltemp_dn7), (locals.var_ttemp_dn8 + locals.var_deltemp_dn8), (locals.var_ttemp_dn9 + locals.var_deltemp_dn9), (locals.var_ttemp_dn10 + locals.var_deltemp_dn10), (locals.var_ttemp_dn13 + locals.var_deltemp_dn13),)
    } else {
        (locals.var_ttemp, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn13,)
    }
};
        locals.var_ttemp = assign17150_e11686;
        locals.var_ttemp_dn0 = assign17150_e11686_d_n0;
        locals.var_ttemp_dn2 = assign17150_e11686_d_n2;
        locals.var_ttemp_dn4 = assign17150_e11686_d_n4;
        locals.var_ttemp_dn5 = assign17150_e11686_d_n5;
        locals.var_ttemp_dn6 = assign17150_e11686_d_n6;
        locals.var_ttemp_dn7 = assign17150_e11686_d_n7;
        locals.var_ttemp_dn8 = assign17150_e11686_d_n8;
        locals.var_ttemp_dn9 = assign17150_e11686_d_n9;
        locals.var_ttemp_dn10 = assign17150_e11686_d_n10;
        locals.var_ttemp_dn13 = assign17150_e11686_d_n13;

        let (assign17160_e11692, assign17160_e11692_d_n0, assign17160_e11692_d_n2, assign17160_e11692_d_n4, assign17160_e11692_d_n5, assign17160_e11692_d_n6, assign17160_e11692_d_n7, assign17160_e11692_d_n8, assign17160_e11692_d_n9, assign17160_e11692_d_n10, assign17160_e11692_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17160_e11690: f64 = (locals.var_ttemp0 - locals.var_ktnom);
        (assign17160_e11690, locals.var_ttemp0_dn0, locals.var_ttemp0_dn2, locals.var_ttemp0_dn4, locals.var_ttemp0_dn5, locals.var_ttemp0_dn6, locals.var_ttemp0_dn7, locals.var_ttemp0_dn8, locals.var_ttemp0_dn9, locals.var_ttemp0_dn10, locals.var_ttemp0_dn13,)
    } else {
        (locals.var_tdiff0, locals.var_tdiff0_dn0, locals.var_tdiff0_dn2, locals.var_tdiff0_dn4, locals.var_tdiff0_dn5, locals.var_tdiff0_dn6, locals.var_tdiff0_dn7, locals.var_tdiff0_dn8, locals.var_tdiff0_dn9, locals.var_tdiff0_dn10, locals.var_tdiff0_dn13,)
    }
};
        locals.var_tdiff0 = assign17160_e11692;
        locals.var_tdiff0_dn0 = assign17160_e11692_d_n0;
        locals.var_tdiff0_dn2 = assign17160_e11692_d_n2;
        locals.var_tdiff0_dn4 = assign17160_e11692_d_n4;
        locals.var_tdiff0_dn5 = assign17160_e11692_d_n5;
        locals.var_tdiff0_dn6 = assign17160_e11692_d_n6;
        locals.var_tdiff0_dn7 = assign17160_e11692_d_n7;
        locals.var_tdiff0_dn8 = assign17160_e11692_d_n8;
        locals.var_tdiff0_dn9 = assign17160_e11692_d_n9;
        locals.var_tdiff0_dn10 = assign17160_e11692_d_n10;
        locals.var_tdiff0_dn13 = assign17160_e11692_d_n13;

        let (assign17170_e11702, assign17170_e11702_d_n0, assign17170_e11702_d_n2, assign17170_e11702_d_n4, assign17170_e11702_d_n5, assign17170_e11702_d_n6, assign17170_e11702_d_n7, assign17170_e11702_d_n8, assign17170_e11702_d_n9, assign17170_e11702_d_n10, assign17170_e11702_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17170_e11696: f64 = (locals.var_ttemp0 * locals.var_ttemp0);
        let assign17170_e11699: f64 = (locals.var_ktnom * locals.var_ktnom);
        let assign17170_e11700: f64 = (assign17170_e11696 - assign17170_e11699);
        (assign17170_e11700, ((locals.var_ttemp0_dn0 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn0)), ((locals.var_ttemp0_dn2 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn2)), ((locals.var_ttemp0_dn4 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn4)), ((locals.var_ttemp0_dn5 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn5)), ((locals.var_ttemp0_dn6 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn6)), ((locals.var_ttemp0_dn7 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn7)), ((locals.var_ttemp0_dn8 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn8)), ((locals.var_ttemp0_dn9 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn9)), ((locals.var_ttemp0_dn10 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn10)), ((locals.var_ttemp0_dn13 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn13)),)
    } else {
        (locals.var_tdiff0_2, locals.var_tdiff0_2_dn0, locals.var_tdiff0_2_dn2, locals.var_tdiff0_2_dn4, locals.var_tdiff0_2_dn5, locals.var_tdiff0_2_dn6, locals.var_tdiff0_2_dn7, locals.var_tdiff0_2_dn8, locals.var_tdiff0_2_dn9, locals.var_tdiff0_2_dn10, locals.var_tdiff0_2_dn13,)
    }
};
        locals.var_tdiff0_2 = assign17170_e11702;
        locals.var_tdiff0_2_dn0 = assign17170_e11702_d_n0;
        locals.var_tdiff0_2_dn2 = assign17170_e11702_d_n2;
        locals.var_tdiff0_2_dn4 = assign17170_e11702_d_n4;
        locals.var_tdiff0_2_dn5 = assign17170_e11702_d_n5;
        locals.var_tdiff0_2_dn6 = assign17170_e11702_d_n6;
        locals.var_tdiff0_2_dn7 = assign17170_e11702_d_n7;
        locals.var_tdiff0_2_dn8 = assign17170_e11702_d_n8;
        locals.var_tdiff0_2_dn9 = assign17170_e11702_d_n9;
        locals.var_tdiff0_2_dn10 = assign17170_e11702_d_n10;
        locals.var_tdiff0_2_dn13 = assign17170_e11702_d_n13;

        let (assign17180_e11708, assign17180_e11708_d_n0, assign17180_e11708_d_n2, assign17180_e11708_d_n4, assign17180_e11708_d_n5, assign17180_e11708_d_n6, assign17180_e11708_d_n7, assign17180_e11708_d_n8, assign17180_e11708_d_n9, assign17180_e11708_d_n10, assign17180_e11708_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17180_e11706: f64 = (locals.var_ttemp - locals.var_ktnom);
        (assign17180_e11706, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn13,)
    } else {
        (locals.var_tdiff, locals.var_tdiff_dn0, locals.var_tdiff_dn2, locals.var_tdiff_dn4, locals.var_tdiff_dn5, locals.var_tdiff_dn6, locals.var_tdiff_dn7, locals.var_tdiff_dn8, locals.var_tdiff_dn9, locals.var_tdiff_dn10, locals.var_tdiff_dn13,)
    }
};
        locals.var_tdiff = assign17180_e11708;
        locals.var_tdiff_dn0 = assign17180_e11708_d_n0;
        locals.var_tdiff_dn2 = assign17180_e11708_d_n2;
        locals.var_tdiff_dn4 = assign17180_e11708_d_n4;
        locals.var_tdiff_dn5 = assign17180_e11708_d_n5;
        locals.var_tdiff_dn6 = assign17180_e11708_d_n6;
        locals.var_tdiff_dn7 = assign17180_e11708_d_n7;
        locals.var_tdiff_dn8 = assign17180_e11708_d_n8;
        locals.var_tdiff_dn9 = assign17180_e11708_d_n9;
        locals.var_tdiff_dn10 = assign17180_e11708_d_n10;
        locals.var_tdiff_dn13 = assign17180_e11708_d_n13;

        let (assign17190_e11718, assign17190_e11718_d_n0, assign17190_e11718_d_n2, assign17190_e11718_d_n4, assign17190_e11718_d_n5, assign17190_e11718_d_n6, assign17190_e11718_d_n7, assign17190_e11718_d_n8, assign17190_e11718_d_n9, assign17190_e11718_d_n10, assign17190_e11718_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17190_e11712: f64 = (locals.var_ttemp * locals.var_ttemp);
        let assign17190_e11715: f64 = (locals.var_ktnom * locals.var_ktnom);
        let assign17190_e11716: f64 = (assign17190_e11712 - assign17190_e11715);
        (assign17190_e11716, ((locals.var_ttemp_dn0 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn0)), ((locals.var_ttemp_dn2 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn2)), ((locals.var_ttemp_dn4 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn4)), ((locals.var_ttemp_dn5 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn5)), ((locals.var_ttemp_dn6 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn6)), ((locals.var_ttemp_dn7 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn7)), ((locals.var_ttemp_dn8 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn8)), ((locals.var_ttemp_dn9 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn9)), ((locals.var_ttemp_dn10 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn10)), ((locals.var_ttemp_dn13 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn13)),)
    } else {
        (locals.var_tdiff_2, locals.var_tdiff_2_dn0, locals.var_tdiff_2_dn2, locals.var_tdiff_2_dn4, locals.var_tdiff_2_dn5, locals.var_tdiff_2_dn6, locals.var_tdiff_2_dn7, locals.var_tdiff_2_dn8, locals.var_tdiff_2_dn9, locals.var_tdiff_2_dn10, locals.var_tdiff_2_dn13,)
    }
};
        locals.var_tdiff_2 = assign17190_e11718;
        locals.var_tdiff_2_dn0 = assign17190_e11718_d_n0;
        locals.var_tdiff_2_dn2 = assign17190_e11718_d_n2;
        locals.var_tdiff_2_dn4 = assign17190_e11718_d_n4;
        locals.var_tdiff_2_dn5 = assign17190_e11718_d_n5;
        locals.var_tdiff_2_dn6 = assign17190_e11718_d_n6;
        locals.var_tdiff_2_dn7 = assign17190_e11718_d_n7;
        locals.var_tdiff_2_dn8 = assign17190_e11718_d_n8;
        locals.var_tdiff_2_dn9 = assign17190_e11718_d_n9;
        locals.var_tdiff_2_dn10 = assign17190_e11718_d_n10;
        locals.var_tdiff_2_dn13 = assign17190_e11718_d_n13;

        let (assign17200_e11724, assign17200_e11724_d_n0, assign17200_e11724_d_n2, assign17200_e11724_d_n4, assign17200_e11724_d_n5, assign17200_e11724_d_n6, assign17200_e11724_d_n7, assign17200_e11724_d_n8, assign17200_e11724_d_n9, assign17200_e11724_d_n10, assign17200_e11724_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17200_e11722: f64 = (locals.var_ttemp / locals.var_ktnom);
        (assign17200_e11722, (locals.var_ttemp_dn0 / locals.var_ktnom), (locals.var_ttemp_dn2 / locals.var_ktnom), (locals.var_ttemp_dn4 / locals.var_ktnom), (locals.var_ttemp_dn5 / locals.var_ktnom), (locals.var_ttemp_dn6 / locals.var_ktnom), (locals.var_ttemp_dn7 / locals.var_ktnom), (locals.var_ttemp_dn8 / locals.var_ktnom), (locals.var_ttemp_dn9 / locals.var_ktnom), (locals.var_ttemp_dn10 / locals.var_ktnom), (locals.var_ttemp_dn13 / locals.var_ktnom),)
    } else {
        (locals.var_tratio, locals.var_tratio_dn0, locals.var_tratio_dn2, locals.var_tratio_dn4, locals.var_tratio_dn5, locals.var_tratio_dn6, locals.var_tratio_dn7, locals.var_tratio_dn8, locals.var_tratio_dn9, locals.var_tratio_dn10, locals.var_tratio_dn13,)
    }
};
        locals.var_tratio = assign17200_e11724;
        locals.var_tratio_dn0 = assign17200_e11724_d_n0;
        locals.var_tratio_dn2 = assign17200_e11724_d_n2;
        locals.var_tratio_dn4 = assign17200_e11724_d_n4;
        locals.var_tratio_dn5 = assign17200_e11724_d_n5;
        locals.var_tratio_dn6 = assign17200_e11724_d_n6;
        locals.var_tratio_dn7 = assign17200_e11724_d_n7;
        locals.var_tratio_dn8 = assign17200_e11724_d_n8;
        locals.var_tratio_dn9 = assign17200_e11724_d_n9;
        locals.var_tratio_dn10 = assign17200_e11724_d_n10;
        locals.var_tratio_dn13 = assign17200_e11724_d_n13;

        let (assign17210_e11729, assign17210_e11729_d_n0, assign17210_e11729_d_n2, assign17210_e11729_d_n4, assign17210_e11729_d_n5, assign17210_e11729_d_n6, assign17210_e11729_d_n7, assign17210_e11729_d_n8, assign17210_e11729_d_n9, assign17210_e11729_d_n10, assign17210_e11729_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17210_e11727: f64 = (locals.var_tratio).ln();
        (assign17210_e11727, (locals.var_tratio_dn0 / locals.var_tratio), (locals.var_tratio_dn2 / locals.var_tratio), (locals.var_tratio_dn4 / locals.var_tratio), (locals.var_tratio_dn5 / locals.var_tratio), (locals.var_tratio_dn6 / locals.var_tratio), (locals.var_tratio_dn7 / locals.var_tratio), (locals.var_tratio_dn8 / locals.var_tratio), (locals.var_tratio_dn9 / locals.var_tratio), (locals.var_tratio_dn10 / locals.var_tratio), (locals.var_tratio_dn13 / locals.var_tratio),)
    } else {
        (locals.var_log_tratio, locals.var_log_tratio_dn0, locals.var_log_tratio_dn2, locals.var_log_tratio_dn4, locals.var_log_tratio_dn5, locals.var_log_tratio_dn6, locals.var_log_tratio_dn7, locals.var_log_tratio_dn8, locals.var_log_tratio_dn9, locals.var_log_tratio_dn10, locals.var_log_tratio_dn13,)
    }
};
        locals.var_log_tratio = assign17210_e11729;
        locals.var_log_tratio_dn0 = assign17210_e11729_d_n0;
        locals.var_log_tratio_dn2 = assign17210_e11729_d_n2;
        locals.var_log_tratio_dn4 = assign17210_e11729_d_n4;
        locals.var_log_tratio_dn5 = assign17210_e11729_d_n5;
        locals.var_log_tratio_dn6 = assign17210_e11729_d_n6;
        locals.var_log_tratio_dn7 = assign17210_e11729_d_n7;
        locals.var_log_tratio_dn8 = assign17210_e11729_d_n8;
        locals.var_log_tratio_dn9 = assign17210_e11729_d_n9;
        locals.var_log_tratio_dn10 = assign17210_e11729_d_n10;
        locals.var_log_tratio_dn13 = assign17210_e11729_d_n13;

        let (assign17220_e11741, assign17220_e11741_d_n0, assign17220_e11741_d_n2, assign17220_e11741_d_n4, assign17220_e11741_d_n5, assign17220_e11741_d_n6, assign17220_e11741_d_n7, assign17220_e11741_d_n8, assign17220_e11741_d_n9, assign17220_e11741_d_n10, assign17220_e11741_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17220_e11734: f64 = (locals.var_uc_bgtmp1 * locals.var_tdiff);
        let assign17220_e11735: f64 = (locals.var_egtnom - assign17220_e11734);
        let assign17220_e11738: f64 = (locals.var_uc_bgtmp2 * locals.var_tdiff_2);
        let assign17220_e11739: f64 = (assign17220_e11735 - assign17220_e11738);
        (assign17220_e11739, ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn0)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn0)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn2)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn2)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn4)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn4)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn5)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn5)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn6)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn6)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn7)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn7)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn8)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn8)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn9)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn9)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn10)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn10)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn13)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn13)),)
    } else {
        (locals.var_eg, locals.var_eg_dn0, locals.var_eg_dn2, locals.var_eg_dn4, locals.var_eg_dn5, locals.var_eg_dn6, locals.var_eg_dn7, locals.var_eg_dn8, locals.var_eg_dn9, locals.var_eg_dn10, locals.var_eg_dn13,)
    }
};
        locals.var_eg = assign17220_e11741;
        locals.var_eg_dn0 = assign17220_e11741_d_n0;
        locals.var_eg_dn2 = assign17220_e11741_d_n2;
        locals.var_eg_dn4 = assign17220_e11741_d_n4;
        locals.var_eg_dn5 = assign17220_e11741_d_n5;
        locals.var_eg_dn6 = assign17220_e11741_d_n6;
        locals.var_eg_dn7 = assign17220_e11741_d_n7;
        locals.var_eg_dn8 = assign17220_e11741_d_n8;
        locals.var_eg_dn9 = assign17220_e11741_d_n9;
        locals.var_eg_dn10 = assign17220_e11741_d_n10;
        locals.var_eg_dn13 = assign17220_e11741_d_n13;

        let (assign17230_e11746, assign17230_e11746_d_n0, assign17230_e11746_d_n2, assign17230_e11746_d_n4, assign17230_e11746_d_n5, assign17230_e11746_d_n6, assign17230_e11746_d_n7, assign17230_e11746_d_n8, assign17230_e11746_d_n9, assign17230_e11746_d_n10, assign17230_e11746_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17230_e11744: f64 = (locals.var_eg).sqrt();
        (assign17230_e11744, (locals.var_eg_dn0 / (2.0 * assign17230_e11744)), (locals.var_eg_dn2 / (2.0 * assign17230_e11744)), (locals.var_eg_dn4 / (2.0 * assign17230_e11744)), (locals.var_eg_dn5 / (2.0 * assign17230_e11744)), (locals.var_eg_dn6 / (2.0 * assign17230_e11744)), (locals.var_eg_dn7 / (2.0 * assign17230_e11744)), (locals.var_eg_dn8 / (2.0 * assign17230_e11744)), (locals.var_eg_dn9 / (2.0 * assign17230_e11744)), (locals.var_eg_dn10 / (2.0 * assign17230_e11744)), (locals.var_eg_dn13 / (2.0 * assign17230_e11744)),)
    } else {
        (locals.var_sqrt_eg, locals.var_sqrt_eg_dn0, locals.var_sqrt_eg_dn2, locals.var_sqrt_eg_dn4, locals.var_sqrt_eg_dn5, locals.var_sqrt_eg_dn6, locals.var_sqrt_eg_dn7, locals.var_sqrt_eg_dn8, locals.var_sqrt_eg_dn9, locals.var_sqrt_eg_dn10, locals.var_sqrt_eg_dn13,)
    }
};
        locals.var_sqrt_eg = assign17230_e11746;
        locals.var_sqrt_eg_dn0 = assign17230_e11746_d_n0;
        locals.var_sqrt_eg_dn2 = assign17230_e11746_d_n2;
        locals.var_sqrt_eg_dn4 = assign17230_e11746_d_n4;
        locals.var_sqrt_eg_dn5 = assign17230_e11746_d_n5;
        locals.var_sqrt_eg_dn6 = assign17230_e11746_d_n6;
        locals.var_sqrt_eg_dn7 = assign17230_e11746_d_n7;
        locals.var_sqrt_eg_dn8 = assign17230_e11746_d_n8;
        locals.var_sqrt_eg_dn9 = assign17230_e11746_d_n9;
        locals.var_sqrt_eg_dn10 = assign17230_e11746_d_n10;
        locals.var_sqrt_eg_dn13 = assign17230_e11746_d_n13;

        let (assign17240_e11752, assign17240_e11752_d_n0, assign17240_e11752_d_n2, assign17240_e11752_d_n4, assign17240_e11752_d_n5, assign17240_e11752_d_n6, assign17240_e11752_d_n7, assign17240_e11752_d_n8, assign17240_e11752_d_n9, assign17240_e11752_d_n10, assign17240_e11752_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17240_e11750: f64 = (1.0 / locals.var_ttemp);
        (assign17240_e11750, (-(locals.var_ttemp_dn0 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn2 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn4 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn5 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn6 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn7 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn8 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn9 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn10 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn13 / (locals.var_ttemp * locals.var_ttemp))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign17240_e11752;
        locals.var_t1_dn0 = assign17240_e11752_d_n0;
        locals.var_t1_dn2 = assign17240_e11752_d_n2;
        locals.var_t1_dn4 = assign17240_e11752_d_n4;
        locals.var_t1_dn5 = assign17240_e11752_d_n5;
        locals.var_t1_dn6 = assign17240_e11752_d_n6;
        locals.var_t1_dn7 = assign17240_e11752_d_n7;
        locals.var_t1_dn8 = assign17240_e11752_d_n8;
        locals.var_t1_dn9 = assign17240_e11752_d_n9;
        locals.var_t1_dn10 = assign17240_e11752_d_n10;
        locals.var_t1_dn13 = assign17240_e11752_d_n13;

        let (assign17250_e11758, assign17250_e11758_d_n0, assign17250_e11758_d_n2, assign17250_e11758_d_n4, assign17250_e11758_d_n5, assign17250_e11758_d_n6, assign17250_e11758_d_n7, assign17250_e11758_d_n8, assign17250_e11758_d_n9, assign17250_e11758_d_n10, assign17250_e11758_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17250_e11756: f64 = (1.0 / locals.var_ktnom);
        (assign17250_e11756, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign17250_e11758;
        locals.var_t2_dn0 = assign17250_e11758_d_n0;
        locals.var_t2_dn2 = assign17250_e11758_d_n2;
        locals.var_t2_dn4 = assign17250_e11758_d_n4;
        locals.var_t2_dn5 = assign17250_e11758_d_n5;
        locals.var_t2_dn6 = assign17250_e11758_d_n6;
        locals.var_t2_dn7 = assign17250_e11758_d_n7;
        locals.var_t2_dn8 = assign17250_e11758_d_n8;
        locals.var_t2_dn9 = assign17250_e11758_d_n9;
        locals.var_t2_dn10 = assign17250_e11758_d_n10;
        locals.var_t2_dn13 = assign17250_e11758_d_n13;

        let (assign17260_e11780, assign17260_e11780_d_n0, assign17260_e11780_d_n2, assign17260_e11780_d_n4, assign17260_e11780_d_n5, assign17260_e11780_d_n6, assign17260_e11780_d_n7, assign17260_e11780_d_n8, assign17260_e11780_d_n9, assign17260_e11780_d_n10, assign17260_e11780_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17260_e11762: f64 = (locals.var_egtnom + p.p259);
        let assign17260_e11766: f64 = (locals.var_t1 - locals.var_t2);
        let assign17260_e11767: f64 = (p.p260 * assign17260_e11766);
        let assign17260_e11768: f64 = (assign17260_e11762 + assign17260_e11767);
        let assign17260_e11772: f64 = (locals.var_t1 * locals.var_t1);
        let assign17260_e11775: f64 = (locals.var_t2 * locals.var_t2);
        let assign17260_e11776: f64 = (assign17260_e11772 - assign17260_e11775);
        let assign17260_e11777: f64 = (p.p261 * assign17260_e11776);
        let assign17260_e11778: f64 = (assign17260_e11768 + assign17260_e11777);
        (assign17260_e11778, ((p.p260 * (locals.var_t1_dn0 - locals.var_t2_dn0)) + (p.p261 * (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) - ((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0))))), ((p.p260 * (locals.var_t1_dn2 - locals.var_t2_dn2)) + (p.p261 * (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) - ((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2))))), ((p.p260 * (locals.var_t1_dn4 - locals.var_t2_dn4)) + (p.p261 * (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) - ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4))))), ((p.p260 * (locals.var_t1_dn5 - locals.var_t2_dn5)) + (p.p261 * (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) - ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5))))), ((p.p260 * (locals.var_t1_dn6 - locals.var_t2_dn6)) + (p.p261 * (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) - ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6))))), ((p.p260 * (locals.var_t1_dn7 - locals.var_t2_dn7)) + (p.p261 * (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) - ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7))))), ((p.p260 * (locals.var_t1_dn8 - locals.var_t2_dn8)) + (p.p261 * (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) - ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8))))), ((p.p260 * (locals.var_t1_dn9 - locals.var_t2_dn9)) + (p.p261 * (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) - ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9))))), ((p.p260 * (locals.var_t1_dn10 - locals.var_t2_dn10)) + (p.p261 * (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) - ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10))))), ((p.p260 * (locals.var_t1_dn13 - locals.var_t2_dn13)) + (p.p261 * (((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)) - ((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign17260_e11780;
        locals.var_t3_dn0 = assign17260_e11780_d_n0;
        locals.var_t3_dn2 = assign17260_e11780_d_n2;
        locals.var_t3_dn4 = assign17260_e11780_d_n4;
        locals.var_t3_dn5 = assign17260_e11780_d_n5;
        locals.var_t3_dn6 = assign17260_e11780_d_n6;
        locals.var_t3_dn7 = assign17260_e11780_d_n7;
        locals.var_t3_dn8 = assign17260_e11780_d_n8;
        locals.var_t3_dn9 = assign17260_e11780_d_n9;
        locals.var_t3_dn10 = assign17260_e11780_d_n10;
        locals.var_t3_dn13 = assign17260_e11780_d_n13;

        let (assign17270_e11785, assign17270_e11785_d_n0, assign17270_e11785_d_n2, assign17270_e11785_d_n4, assign17270_e11785_d_n5, assign17270_e11785_d_n6, assign17270_e11785_d_n7, assign17270_e11785_d_n8, assign17270_e11785_d_n9, assign17270_e11785_d_n10, assign17270_e11785_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17270_e11783: f64 = (locals.var_t3).sqrt();
        (assign17270_e11783, (locals.var_t3_dn0 / (2.0 * assign17270_e11783)), (locals.var_t3_dn2 / (2.0 * assign17270_e11783)), (locals.var_t3_dn4 / (2.0 * assign17270_e11783)), (locals.var_t3_dn5 / (2.0 * assign17270_e11783)), (locals.var_t3_dn6 / (2.0 * assign17270_e11783)), (locals.var_t3_dn7 / (2.0 * assign17270_e11783)), (locals.var_t3_dn8 / (2.0 * assign17270_e11783)), (locals.var_t3_dn9 / (2.0 * assign17270_e11783)), (locals.var_t3_dn10 / (2.0 * assign17270_e11783)), (locals.var_t3_dn13 / (2.0 * assign17270_e11783)),)
    } else {
        (locals.var_egp12, locals.var_egp12_dn0, locals.var_egp12_dn2, locals.var_egp12_dn4, locals.var_egp12_dn5, locals.var_egp12_dn6, locals.var_egp12_dn7, locals.var_egp12_dn8, locals.var_egp12_dn9, locals.var_egp12_dn10, locals.var_egp12_dn13,)
    }
};
        locals.var_egp12 = assign17270_e11785;
        locals.var_egp12_dn0 = assign17270_e11785_d_n0;
        locals.var_egp12_dn2 = assign17270_e11785_d_n2;
        locals.var_egp12_dn4 = assign17270_e11785_d_n4;
        locals.var_egp12_dn5 = assign17270_e11785_d_n5;
        locals.var_egp12_dn6 = assign17270_e11785_d_n6;
        locals.var_egp12_dn7 = assign17270_e11785_d_n7;
        locals.var_egp12_dn8 = assign17270_e11785_d_n8;
        locals.var_egp12_dn9 = assign17270_e11785_d_n9;
        locals.var_egp12_dn10 = assign17270_e11785_d_n10;
        locals.var_egp12_dn13 = assign17270_e11785_d_n13;

        let (assign17280_e11791, assign17280_e11791_d_n0, assign17280_e11791_d_n2, assign17280_e11791_d_n4, assign17280_e11791_d_n5, assign17280_e11791_d_n6, assign17280_e11791_d_n7, assign17280_e11791_d_n8, assign17280_e11791_d_n9, assign17280_e11791_d_n10, assign17280_e11791_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17280_e11789: f64 = (locals.var_t3 * locals.var_egp12);
        (assign17280_e11789, ((locals.var_t3_dn0 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn0)), ((locals.var_t3_dn2 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn2)), ((locals.var_t3_dn4 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn4)), ((locals.var_t3_dn5 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn5)), ((locals.var_t3_dn6 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn6)), ((locals.var_t3_dn7 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn7)), ((locals.var_t3_dn8 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn8)), ((locals.var_t3_dn9 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn9)), ((locals.var_t3_dn10 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn10)), ((locals.var_t3_dn13 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn13)),)
    } else {
        (locals.var_egp32, locals.var_egp32_dn0, locals.var_egp32_dn2, locals.var_egp32_dn4, locals.var_egp32_dn5, locals.var_egp32_dn6, locals.var_egp32_dn7, locals.var_egp32_dn8, locals.var_egp32_dn9, locals.var_egp32_dn10, locals.var_egp32_dn13,)
    }
};
        locals.var_egp32 = assign17280_e11791;
        locals.var_egp32_dn0 = assign17280_e11791_d_n0;
        locals.var_egp32_dn2 = assign17280_e11791_d_n2;
        locals.var_egp32_dn4 = assign17280_e11791_d_n4;
        locals.var_egp32_dn5 = assign17280_e11791_d_n5;
        locals.var_egp32_dn6 = assign17280_e11791_d_n6;
        locals.var_egp32_dn7 = assign17280_e11791_d_n7;
        locals.var_egp32_dn8 = assign17280_e11791_d_n8;
        locals.var_egp32_dn9 = assign17280_e11791_d_n9;
        locals.var_egp32_dn10 = assign17280_e11791_d_n10;
        locals.var_egp32_dn13 = assign17280_e11791_d_n13;

        let (assign17290_e11799, assign17290_e11799_d_n0, assign17290_e11799_d_n2, assign17290_e11799_d_n4, assign17290_e11799_d_n5, assign17290_e11799_d_n6, assign17290_e11799_d_n7, assign17290_e11799_d_n8, assign17290_e11799_d_n9, assign17290_e11799_d_n10, assign17290_e11799_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17290_e11796: f64 = (1.3806226e-23 * locals.var_ttemp);
        let assign17290_e11797: f64 = (1.6021918e-19 / assign17290_e11796);
        (assign17290_e11797, (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn0)) / (assign17290_e11796 * assign17290_e11796))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn2)) / (assign17290_e11796 * assign17290_e11796))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn4)) / (assign17290_e11796 * assign17290_e11796))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn5)) / (assign17290_e11796 * assign17290_e11796))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn6)) / (assign17290_e11796 * assign17290_e11796))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn7)) / (assign17290_e11796 * assign17290_e11796))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn8)) / (assign17290_e11796 * assign17290_e11796))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn9)) / (assign17290_e11796 * assign17290_e11796))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn10)) / (assign17290_e11796 * assign17290_e11796))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn13)) / (assign17290_e11796 * assign17290_e11796))),)
    } else {
        (locals.var_beta, locals.var_beta_dn0, locals.var_beta_dn2, locals.var_beta_dn4, locals.var_beta_dn5, locals.var_beta_dn6, locals.var_beta_dn7, locals.var_beta_dn8, locals.var_beta_dn9, locals.var_beta_dn10, locals.var_beta_dn13,)
    }
};
        locals.var_beta = assign17290_e11799;
        locals.var_beta_dn0 = assign17290_e11799_d_n0;
        locals.var_beta_dn2 = assign17290_e11799_d_n2;
        locals.var_beta_dn4 = assign17290_e11799_d_n4;
        locals.var_beta_dn5 = assign17290_e11799_d_n5;
        locals.var_beta_dn6 = assign17290_e11799_d_n6;
        locals.var_beta_dn7 = assign17290_e11799_d_n7;
        locals.var_beta_dn8 = assign17290_e11799_d_n8;
        locals.var_beta_dn9 = assign17290_e11799_d_n9;
        locals.var_beta_dn10 = assign17290_e11799_d_n10;
        locals.var_beta_dn13 = assign17290_e11799_d_n13;

        let (assign17300_e11805, assign17300_e11805_d_n0, assign17300_e11805_d_n2, assign17300_e11805_d_n4, assign17300_e11805_d_n5, assign17300_e11805_d_n6, assign17300_e11805_d_n7, assign17300_e11805_d_n8, assign17300_e11805_d_n9, assign17300_e11805_d_n10, assign17300_e11805_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17300_e11803: f64 = (1.0 / locals.var_beta);
        (assign17300_e11803, (-(locals.var_beta_dn0 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn2 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn4 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn5 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn6 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn7 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn8 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn9 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn10 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn13 / (locals.var_beta * locals.var_beta))),)
    } else {
        (locals.var_beta_inv, locals.var_beta_inv_dn0, locals.var_beta_inv_dn2, locals.var_beta_inv_dn4, locals.var_beta_inv_dn5, locals.var_beta_inv_dn6, locals.var_beta_inv_dn7, locals.var_beta_inv_dn8, locals.var_beta_inv_dn9, locals.var_beta_inv_dn10, locals.var_beta_inv_dn13,)
    }
};
        locals.var_beta_inv = assign17300_e11805;
        locals.var_beta_inv_dn0 = assign17300_e11805_d_n0;
        locals.var_beta_inv_dn2 = assign17300_e11805_d_n2;
        locals.var_beta_inv_dn4 = assign17300_e11805_d_n4;
        locals.var_beta_inv_dn5 = assign17300_e11805_d_n5;
        locals.var_beta_inv_dn6 = assign17300_e11805_d_n6;
        locals.var_beta_inv_dn7 = assign17300_e11805_d_n7;
        locals.var_beta_inv_dn8 = assign17300_e11805_d_n8;
        locals.var_beta_inv_dn9 = assign17300_e11805_d_n9;
        locals.var_beta_inv_dn10 = assign17300_e11805_d_n10;
        locals.var_beta_inv_dn13 = assign17300_e11805_d_n13;

        let (assign17310_e11811, assign17310_e11811_d_n0, assign17310_e11811_d_n2, assign17310_e11811_d_n4, assign17310_e11811_d_n5, assign17310_e11811_d_n6, assign17310_e11811_d_n7, assign17310_e11811_d_n8, assign17310_e11811_d_n9, assign17310_e11811_d_n10, assign17310_e11811_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17310_e11809: f64 = (locals.var_beta * locals.var_beta);
        (assign17310_e11809, ((locals.var_beta_dn0 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn0)), ((locals.var_beta_dn2 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn2)), ((locals.var_beta_dn4 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn4)), ((locals.var_beta_dn5 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn5)), ((locals.var_beta_dn6 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn6)), ((locals.var_beta_dn7 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn7)), ((locals.var_beta_dn8 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn8)), ((locals.var_beta_dn9 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn9)), ((locals.var_beta_dn10 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn10)), ((locals.var_beta_dn13 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn13)),)
    } else {
        (locals.var_beta2, locals.var_beta2_dn0, locals.var_beta2_dn2, locals.var_beta2_dn4, locals.var_beta2_dn5, locals.var_beta2_dn6, locals.var_beta2_dn7, locals.var_beta2_dn8, locals.var_beta2_dn9, locals.var_beta2_dn10, locals.var_beta2_dn13,)
    }
};
        locals.var_beta2 = assign17310_e11811;
        locals.var_beta2_dn0 = assign17310_e11811_d_n0;
        locals.var_beta2_dn2 = assign17310_e11811_d_n2;
        locals.var_beta2_dn4 = assign17310_e11811_d_n4;
        locals.var_beta2_dn5 = assign17310_e11811_d_n5;
        locals.var_beta2_dn6 = assign17310_e11811_d_n6;
        locals.var_beta2_dn7 = assign17310_e11811_d_n7;
        locals.var_beta2_dn8 = assign17310_e11811_d_n8;
        locals.var_beta2_dn9 = assign17310_e11811_d_n9;
        locals.var_beta2_dn10 = assign17310_e11811_d_n10;
        locals.var_beta2_dn13 = assign17310_e11811_d_n13;

    }

    pub(super) fn stamp_transient_block_36(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign17320_e11819,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17320_e11816: f64 = (1.3806226e-23 * locals.var_ktnom);
        let assign17320_e11817: f64 = (1.6021918e-19 / assign17320_e11816);
        (assign17320_e11817,)
    } else {
        (locals.var_betatnom,)
    }
};
        locals.var_betatnom = assign17320_e11819;

        let (assign17330_e11842, assign17330_e11842_d_n0, assign17330_e11842_d_n2, assign17330_e11842_d_n4, assign17330_e11842_d_n5, assign17330_e11842_d_n6, assign17330_e11842_d_n7, assign17330_e11842_d_n8, assign17330_e11842_d_n9, assign17330_e11842_d_n10, assign17330_e11842_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17330_e11824: f64 = (locals.var_log_tratio * 1.5);
        let assign17330_e11825: f64 = (assign17330_e11824).exp();
        let assign17330_e11826: f64 = (1.04e16 * assign17330_e11825);
        let assign17330_e11828: f64 = (-locals.var_eg);
        let assign17330_e11830: f64 = (assign17330_e11828 / 2.0);
        let assign17330_e11832: f64 = (assign17330_e11830 * locals.var_beta);
        let assign17330_e11835: f64 = (locals.var_egtnom / 2.0);
        let assign17330_e11837: f64 = (assign17330_e11835 * locals.var_betatnom);
        let assign17330_e11838: f64 = (assign17330_e11832 + assign17330_e11837);
        let assign17330_e11839: f64 = (assign17330_e11838).exp();
        let assign17330_e11840: f64 = (assign17330_e11826 * assign17330_e11839);
        (assign17330_e11840, (((1.04e16 * (assign17330_e11825 * (locals.var_log_tratio_dn0 * 1.5))) * assign17330_e11839) + (assign17330_e11826 * (assign17330_e11839 * ((((-locals.var_eg_dn0) / 2.0) * locals.var_beta) + (assign17330_e11830 * locals.var_beta_dn0))))), (((1.04e16 * (assign17330_e11825 * (locals.var_log_tratio_dn2 * 1.5))) * assign17330_e11839) + (assign17330_e11826 * (assign17330_e11839 * ((((-locals.var_eg_dn2) / 2.0) * locals.var_beta) + (assign17330_e11830 * locals.var_beta_dn2))))), (((1.04e16 * (assign17330_e11825 * (locals.var_log_tratio_dn4 * 1.5))) * assign17330_e11839) + (assign17330_e11826 * (assign17330_e11839 * ((((-locals.var_eg_dn4) / 2.0) * locals.var_beta) + (assign17330_e11830 * locals.var_beta_dn4))))), (((1.04e16 * (assign17330_e11825 * (locals.var_log_tratio_dn5 * 1.5))) * assign17330_e11839) + (assign17330_e11826 * (assign17330_e11839 * ((((-locals.var_eg_dn5) / 2.0) * locals.var_beta) + (assign17330_e11830 * locals.var_beta_dn5))))), (((1.04e16 * (assign17330_e11825 * (locals.var_log_tratio_dn6 * 1.5))) * assign17330_e11839) + (assign17330_e11826 * (assign17330_e11839 * ((((-locals.var_eg_dn6) / 2.0) * locals.var_beta) + (assign17330_e11830 * locals.var_beta_dn6))))), (((1.04e16 * (assign17330_e11825 * (locals.var_log_tratio_dn7 * 1.5))) * assign17330_e11839) + (assign17330_e11826 * (assign17330_e11839 * ((((-locals.var_eg_dn7) / 2.0) * locals.var_beta) + (assign17330_e11830 * locals.var_beta_dn7))))), (((1.04e16 * (assign17330_e11825 * (locals.var_log_tratio_dn8 * 1.5))) * assign17330_e11839) + (assign17330_e11826 * (assign17330_e11839 * ((((-locals.var_eg_dn8) / 2.0) * locals.var_beta) + (assign17330_e11830 * locals.var_beta_dn8))))), (((1.04e16 * (assign17330_e11825 * (locals.var_log_tratio_dn9 * 1.5))) * assign17330_e11839) + (assign17330_e11826 * (assign17330_e11839 * ((((-locals.var_eg_dn9) / 2.0) * locals.var_beta) + (assign17330_e11830 * locals.var_beta_dn9))))), (((1.04e16 * (assign17330_e11825 * (locals.var_log_tratio_dn10 * 1.5))) * assign17330_e11839) + (assign17330_e11826 * (assign17330_e11839 * ((((-locals.var_eg_dn10) / 2.0) * locals.var_beta) + (assign17330_e11830 * locals.var_beta_dn10))))), (((1.04e16 * (assign17330_e11825 * (locals.var_log_tratio_dn13 * 1.5))) * assign17330_e11839) + (assign17330_e11826 * (assign17330_e11839 * ((((-locals.var_eg_dn13) / 2.0) * locals.var_beta) + (assign17330_e11830 * locals.var_beta_dn13))))),)
    } else {
        (locals.var_nin, locals.var_nin_dn0, locals.var_nin_dn2, locals.var_nin_dn4, locals.var_nin_dn5, locals.var_nin_dn6, locals.var_nin_dn7, locals.var_nin_dn8, locals.var_nin_dn9, locals.var_nin_dn10, locals.var_nin_dn13,)
    }
};
        locals.var_nin = assign17330_e11842;
        locals.var_nin_dn0 = assign17330_e11842_d_n0;
        locals.var_nin_dn2 = assign17330_e11842_d_n2;
        locals.var_nin_dn4 = assign17330_e11842_d_n4;
        locals.var_nin_dn5 = assign17330_e11842_d_n5;
        locals.var_nin_dn6 = assign17330_e11842_d_n6;
        locals.var_nin_dn7 = assign17330_e11842_d_n7;
        locals.var_nin_dn8 = assign17330_e11842_d_n8;
        locals.var_nin_dn9 = assign17330_e11842_d_n9;
        locals.var_nin_dn10 = assign17330_e11842_d_n10;
        locals.var_nin_dn13 = assign17330_e11842_d_n13;

        let (assign17340_e11849, assign17340_e11849_d_n0, assign17340_e11849_d_n2, assign17340_e11849_d_n4, assign17340_e11849_d_n5, assign17340_e11849_d_n6, assign17340_e11849_d_n7, assign17340_e11849_d_n8, assign17340_e11849_d_n9, assign17340_e11849_d_n10, assign17340_e11849_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17340_e11846: f64 = (locals.var_log_tratio * locals.var_uc_muetmp);
        let assign17340_e11847: f64 = (assign17340_e11846).exp();
        (assign17340_e11847, (assign17340_e11847 * (locals.var_log_tratio_dn0 * locals.var_uc_muetmp)), (assign17340_e11847 * (locals.var_log_tratio_dn2 * locals.var_uc_muetmp)), (assign17340_e11847 * (locals.var_log_tratio_dn4 * locals.var_uc_muetmp)), (assign17340_e11847 * (locals.var_log_tratio_dn5 * locals.var_uc_muetmp)), (assign17340_e11847 * (locals.var_log_tratio_dn6 * locals.var_uc_muetmp)), (assign17340_e11847 * (locals.var_log_tratio_dn7 * locals.var_uc_muetmp)), (assign17340_e11847 * (locals.var_log_tratio_dn8 * locals.var_uc_muetmp)), (assign17340_e11847 * (locals.var_log_tratio_dn9 * locals.var_uc_muetmp)), (assign17340_e11847 * (locals.var_log_tratio_dn10 * locals.var_uc_muetmp)), (assign17340_e11847 * (locals.var_log_tratio_dn13 * locals.var_uc_muetmp)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign17340_e11849;
        locals.var_t1_dn0 = assign17340_e11849_d_n0;
        locals.var_t1_dn2 = assign17340_e11849_d_n2;
        locals.var_t1_dn4 = assign17340_e11849_d_n4;
        locals.var_t1_dn5 = assign17340_e11849_d_n5;
        locals.var_t1_dn6 = assign17340_e11849_d_n6;
        locals.var_t1_dn7 = assign17340_e11849_d_n7;
        locals.var_t1_dn8 = assign17340_e11849_d_n8;
        locals.var_t1_dn9 = assign17340_e11849_d_n9;
        locals.var_t1_dn10 = assign17340_e11849_d_n10;
        locals.var_t1_dn13 = assign17340_e11849_d_n13;

        let (assign17350_e11855, assign17350_e11855_d_n0, assign17350_e11855_d_n2, assign17350_e11855_d_n4, assign17350_e11855_d_n5, assign17350_e11855_d_n6, assign17350_e11855_d_n7, assign17350_e11855_d_n8, assign17350_e11855_d_n9, assign17350_e11855_d_n10, assign17350_e11855_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17350_e11853: f64 = (locals.var_t1 / locals.var_mueph);
        (assign17350_e11853, (((locals.var_t1_dn0 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn0)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn2 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn2)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn4 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn4)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn5 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn5)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn6 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn6)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn7 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn7)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn8 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn8)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn9 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn9)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn10 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn10)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn13 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn13)) / (locals.var_mueph * locals.var_mueph)),)
    } else {
        (locals.var_mphn0, locals.var_mphn0_dn0, locals.var_mphn0_dn2, locals.var_mphn0_dn4, locals.var_mphn0_dn5, locals.var_mphn0_dn6, locals.var_mphn0_dn7, locals.var_mphn0_dn8, locals.var_mphn0_dn9, locals.var_mphn0_dn10, locals.var_mphn0_dn13,)
    }
};
        locals.var_mphn0 = assign17350_e11855;
        locals.var_mphn0_dn0 = assign17350_e11855_d_n0;
        locals.var_mphn0_dn2 = assign17350_e11855_d_n2;
        locals.var_mphn0_dn4 = assign17350_e11855_d_n4;
        locals.var_mphn0_dn5 = assign17350_e11855_d_n5;
        locals.var_mphn0_dn6 = assign17350_e11855_d_n6;
        locals.var_mphn0_dn7 = assign17350_e11855_d_n7;
        locals.var_mphn0_dn8 = assign17350_e11855_d_n8;
        locals.var_mphn0_dn9 = assign17350_e11855_d_n9;
        locals.var_mphn0_dn10 = assign17350_e11855_d_n10;
        locals.var_mphn0_dn13 = assign17350_e11855_d_n13;

        let assign17360_e11862: f64 = if ((locals.var_uc_codep != 0.0) && (locals.var_uc_codep < 3.0)) { 1.0 } else { 0.0 };
        locals.var_guard355 = assign17360_e11862;

        let (assign17370_e11877, assign17370_e11877_d_n0, assign17370_e11877_d_n2, assign17370_e11877_d_n4, assign17370_e11877_d_n5, assign17370_e11877_d_n6, assign17370_e11877_d_n7, assign17370_e11877_d_n8, assign17370_e11877_d_n9, assign17370_e11877_d_n10, assign17370_e11877_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard355 != 0.0)) {
        let assign17370_e11868: f64 = (2.0 * 1.034943e-10);
        let assign17370_e11870: f64 = (assign17370_e11868 * 1.6021918e-19);
        let assign17370_e11872: f64 = (assign17370_e11870 * locals.var_uc_ndepm);
        let assign17370_e11874: f64 = (assign17370_e11872 * locals.var_beta_inv);
        let assign17370_e11875: f64 = (assign17370_e11874).sqrt();
        (assign17370_e11875, ((((assign17370_e11870 * locals.var_uc_ndepm_dn0) * locals.var_beta_inv) + (assign17370_e11872 * locals.var_beta_inv_dn0)) / (2.0 * assign17370_e11875)), ((((assign17370_e11870 * locals.var_uc_ndepm_dn2) * locals.var_beta_inv) + (assign17370_e11872 * locals.var_beta_inv_dn2)) / (2.0 * assign17370_e11875)), ((((assign17370_e11870 * locals.var_uc_ndepm_dn4) * locals.var_beta_inv) + (assign17370_e11872 * locals.var_beta_inv_dn4)) / (2.0 * assign17370_e11875)), ((((assign17370_e11870 * locals.var_uc_ndepm_dn5) * locals.var_beta_inv) + (assign17370_e11872 * locals.var_beta_inv_dn5)) / (2.0 * assign17370_e11875)), ((((assign17370_e11870 * locals.var_uc_ndepm_dn6) * locals.var_beta_inv) + (assign17370_e11872 * locals.var_beta_inv_dn6)) / (2.0 * assign17370_e11875)), ((((assign17370_e11870 * locals.var_uc_ndepm_dn7) * locals.var_beta_inv) + (assign17370_e11872 * locals.var_beta_inv_dn7)) / (2.0 * assign17370_e11875)), ((((assign17370_e11870 * locals.var_uc_ndepm_dn8) * locals.var_beta_inv) + (assign17370_e11872 * locals.var_beta_inv_dn8)) / (2.0 * assign17370_e11875)), ((((assign17370_e11870 * locals.var_uc_ndepm_dn9) * locals.var_beta_inv) + (assign17370_e11872 * locals.var_beta_inv_dn9)) / (2.0 * assign17370_e11875)), ((((assign17370_e11870 * locals.var_uc_ndepm_dn10) * locals.var_beta_inv) + (assign17370_e11872 * locals.var_beta_inv_dn10)) / (2.0 * assign17370_e11875)), ((((assign17370_e11870 * locals.var_uc_ndepm_dn13) * locals.var_beta_inv) + (assign17370_e11872 * locals.var_beta_inv_dn13)) / (2.0 * assign17370_e11875)),)
    } else {
        (locals.var_cnst0, locals.var_cnst0_dn0, locals.var_cnst0_dn2, locals.var_cnst0_dn4, locals.var_cnst0_dn5, locals.var_cnst0_dn6, locals.var_cnst0_dn7, locals.var_cnst0_dn8, locals.var_cnst0_dn9, locals.var_cnst0_dn10, locals.var_cnst0_dn13,)
    }
};
        locals.var_cnst0 = assign17370_e11877;
        locals.var_cnst0_dn0 = assign17370_e11877_d_n0;
        locals.var_cnst0_dn2 = assign17370_e11877_d_n2;
        locals.var_cnst0_dn4 = assign17370_e11877_d_n4;
        locals.var_cnst0_dn5 = assign17370_e11877_d_n5;
        locals.var_cnst0_dn6 = assign17370_e11877_d_n6;
        locals.var_cnst0_dn7 = assign17370_e11877_d_n7;
        locals.var_cnst0_dn8 = assign17370_e11877_d_n8;
        locals.var_cnst0_dn9 = assign17370_e11877_d_n9;
        locals.var_cnst0_dn10 = assign17370_e11877_d_n10;
        locals.var_cnst0_dn13 = assign17370_e11877_d_n13;

        let (assign17380_e11889, assign17380_e11889_d_n0, assign17380_e11889_d_n2, assign17380_e11889_d_n4, assign17380_e11889_d_n5, assign17380_e11889_d_n6, assign17380_e11889_d_n7, assign17380_e11889_d_n8, assign17380_e11889_d_n9, assign17380_e11889_d_n10, assign17380_e11889_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard355 != 0.0)) {
        let assign17380_e11883: f64 = (locals.var_nin * locals.var_nin);
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_uc_ndepm;
        let assign17380_e11885: f64 = (assign17380_e11883 * __rspice_inv_cse_0);
        let assign17380_e11887: f64 = (assign17380_e11885 * __rspice_inv_cse_0);
        (assign17380_e11887, ((((((((locals.var_nin_dn0 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn0)) * locals.var_uc_ndepm) - (assign17380_e11883 * locals.var_uc_ndepm_dn0)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17380_e11885 * locals.var_uc_ndepm_dn0)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn2 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn2)) * locals.var_uc_ndepm) - (assign17380_e11883 * locals.var_uc_ndepm_dn2)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17380_e11885 * locals.var_uc_ndepm_dn2)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn4 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn4)) * locals.var_uc_ndepm) - (assign17380_e11883 * locals.var_uc_ndepm_dn4)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17380_e11885 * locals.var_uc_ndepm_dn4)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn5 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn5)) * locals.var_uc_ndepm) - (assign17380_e11883 * locals.var_uc_ndepm_dn5)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17380_e11885 * locals.var_uc_ndepm_dn5)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn6 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn6)) * locals.var_uc_ndepm) - (assign17380_e11883 * locals.var_uc_ndepm_dn6)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17380_e11885 * locals.var_uc_ndepm_dn6)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn7 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn7)) * locals.var_uc_ndepm) - (assign17380_e11883 * locals.var_uc_ndepm_dn7)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17380_e11885 * locals.var_uc_ndepm_dn7)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn8 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn8)) * locals.var_uc_ndepm) - (assign17380_e11883 * locals.var_uc_ndepm_dn8)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17380_e11885 * locals.var_uc_ndepm_dn8)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn9 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn9)) * locals.var_uc_ndepm) - (assign17380_e11883 * locals.var_uc_ndepm_dn9)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17380_e11885 * locals.var_uc_ndepm_dn9)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn10 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn10)) * locals.var_uc_ndepm) - (assign17380_e11883 * locals.var_uc_ndepm_dn10)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17380_e11885 * locals.var_uc_ndepm_dn10)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn13 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn13)) * locals.var_uc_ndepm) - (assign17380_e11883 * locals.var_uc_ndepm_dn13)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17380_e11885 * locals.var_uc_ndepm_dn13)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)),)
    } else {
        (locals.var_cnst1, locals.var_cnst1_dn0, locals.var_cnst1_dn2, locals.var_cnst1_dn4, locals.var_cnst1_dn5, locals.var_cnst1_dn6, locals.var_cnst1_dn7, locals.var_cnst1_dn8, locals.var_cnst1_dn9, locals.var_cnst1_dn10, locals.var_cnst1_dn13,)
    }
};
        locals.var_cnst1 = assign17380_e11889;
        locals.var_cnst1_dn0 = assign17380_e11889_d_n0;
        locals.var_cnst1_dn2 = assign17380_e11889_d_n2;
        locals.var_cnst1_dn4 = assign17380_e11889_d_n4;
        locals.var_cnst1_dn5 = assign17380_e11889_d_n5;
        locals.var_cnst1_dn6 = assign17380_e11889_d_n6;
        locals.var_cnst1_dn7 = assign17380_e11889_d_n7;
        locals.var_cnst1_dn8 = assign17380_e11889_d_n8;
        locals.var_cnst1_dn9 = assign17380_e11889_d_n9;
        locals.var_cnst1_dn10 = assign17380_e11889_d_n10;
        locals.var_cnst1_dn13 = assign17380_e11889_d_n13;

        let (assign17390_e11902, assign17390_e11902_d_n0, assign17390_e11902_d_n2, assign17390_e11902_d_n4, assign17390_e11902_d_n5, assign17390_e11902_d_n6, assign17390_e11902_d_n7, assign17390_e11902_d_n8, assign17390_e11902_d_n9, assign17390_e11902_d_n10, assign17390_e11902_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard355 != 0.0)) {
        let assign17390_e11895: f64 = (2.0 * locals.var_beta_inv);
        let assign17390_e11898: f64 = (locals.var_uc_ndepm / locals.var_nin);
        let assign17390_e11899: f64 = (assign17390_e11898).ln();
        let assign17390_e11900: f64 = (assign17390_e11895 * assign17390_e11899);
        (assign17390_e11900, (((2.0 * locals.var_beta_inv_dn0) * assign17390_e11899) + (assign17390_e11895 * ((((locals.var_uc_ndepm_dn0 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign17390_e11898))), (((2.0 * locals.var_beta_inv_dn2) * assign17390_e11899) + (assign17390_e11895 * ((((locals.var_uc_ndepm_dn2 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign17390_e11898))), (((2.0 * locals.var_beta_inv_dn4) * assign17390_e11899) + (assign17390_e11895 * ((((locals.var_uc_ndepm_dn4 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign17390_e11898))), (((2.0 * locals.var_beta_inv_dn5) * assign17390_e11899) + (assign17390_e11895 * ((((locals.var_uc_ndepm_dn5 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign17390_e11898))), (((2.0 * locals.var_beta_inv_dn6) * assign17390_e11899) + (assign17390_e11895 * ((((locals.var_uc_ndepm_dn6 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign17390_e11898))), (((2.0 * locals.var_beta_inv_dn7) * assign17390_e11899) + (assign17390_e11895 * ((((locals.var_uc_ndepm_dn7 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign17390_e11898))), (((2.0 * locals.var_beta_inv_dn8) * assign17390_e11899) + (assign17390_e11895 * ((((locals.var_uc_ndepm_dn8 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign17390_e11898))), (((2.0 * locals.var_beta_inv_dn9) * assign17390_e11899) + (assign17390_e11895 * ((((locals.var_uc_ndepm_dn9 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign17390_e11898))), (((2.0 * locals.var_beta_inv_dn10) * assign17390_e11899) + (assign17390_e11895 * ((((locals.var_uc_ndepm_dn10 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign17390_e11898))), (((2.0 * locals.var_beta_inv_dn13) * assign17390_e11899) + (assign17390_e11895 * ((((locals.var_uc_ndepm_dn13 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn13)) / (locals.var_nin * locals.var_nin)) / assign17390_e11898))),)
    } else {
        (locals.var_pb2n, locals.var_pb2n_dn0, locals.var_pb2n_dn2, locals.var_pb2n_dn4, locals.var_pb2n_dn5, locals.var_pb2n_dn6, locals.var_pb2n_dn7, locals.var_pb2n_dn8, locals.var_pb2n_dn9, locals.var_pb2n_dn10, locals.var_pb2n_dn13,)
    }
};
        locals.var_pb2n = assign17390_e11902;
        locals.var_pb2n_dn0 = assign17390_e11902_d_n0;
        locals.var_pb2n_dn2 = assign17390_e11902_d_n2;
        locals.var_pb2n_dn4 = assign17390_e11902_d_n4;
        locals.var_pb2n_dn5 = assign17390_e11902_d_n5;
        locals.var_pb2n_dn6 = assign17390_e11902_d_n6;
        locals.var_pb2n_dn7 = assign17390_e11902_d_n7;
        locals.var_pb2n_dn8 = assign17390_e11902_d_n8;
        locals.var_pb2n_dn9 = assign17390_e11902_d_n9;
        locals.var_pb2n_dn10 = assign17390_e11902_d_n10;
        locals.var_pb2n_dn13 = assign17390_e11902_d_n13;

        let (assign17400_e11917, assign17400_e11917_d_n0, assign17400_e11917_d_n2, assign17400_e11917_d_n4, assign17400_e11917_d_n5, assign17400_e11917_d_n6, assign17400_e11917_d_n7, assign17400_e11917_d_n8, assign17400_e11917_d_n9, assign17400_e11917_d_n10, assign17400_e11917_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard355 != 0.0)) {
        let assign17400_e11909: f64 = (locals.var_uc_ndepm * locals.var_ef_nsubc);
        let __rspice_inv_cse_1: f64 = 1.0 / locals.var_nin;
        let assign17400_e11911: f64 = (assign17400_e11909 * __rspice_inv_cse_1);
        let assign17400_e11913: f64 = (assign17400_e11911 * __rspice_inv_cse_1);
        let assign17400_e11914: f64 = (assign17400_e11913).ln();
        let assign17400_e11915: f64 = (locals.var_beta_inv * assign17400_e11914);
        (assign17400_e11915, ((locals.var_beta_inv_dn0 * assign17400_e11914) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn0 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn0)) * locals.var_nin) - (assign17400_e11909 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17400_e11911 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign17400_e11913))), ((locals.var_beta_inv_dn2 * assign17400_e11914) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn2 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn2)) * locals.var_nin) - (assign17400_e11909 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17400_e11911 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign17400_e11913))), ((locals.var_beta_inv_dn4 * assign17400_e11914) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn4 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn4)) * locals.var_nin) - (assign17400_e11909 * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17400_e11911 * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign17400_e11913))), ((locals.var_beta_inv_dn5 * assign17400_e11914) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn5 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn5)) * locals.var_nin) - (assign17400_e11909 * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17400_e11911 * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign17400_e11913))), ((locals.var_beta_inv_dn6 * assign17400_e11914) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn6 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn6)) * locals.var_nin) - (assign17400_e11909 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17400_e11911 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign17400_e11913))), ((locals.var_beta_inv_dn7 * assign17400_e11914) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn7 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn7)) * locals.var_nin) - (assign17400_e11909 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17400_e11911 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign17400_e11913))), ((locals.var_beta_inv_dn8 * assign17400_e11914) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn8 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn8)) * locals.var_nin) - (assign17400_e11909 * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17400_e11911 * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign17400_e11913))), ((locals.var_beta_inv_dn9 * assign17400_e11914) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn9 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn9)) * locals.var_nin) - (assign17400_e11909 * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17400_e11911 * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign17400_e11913))), ((locals.var_beta_inv_dn10 * assign17400_e11914) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn10 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn10)) * locals.var_nin) - (assign17400_e11909 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17400_e11911 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign17400_e11913))), ((locals.var_beta_inv_dn13 * assign17400_e11914) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn13 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn13)) * locals.var_nin) - (assign17400_e11909 * locals.var_nin_dn13)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17400_e11911 * locals.var_nin_dn13)) / (locals.var_nin * locals.var_nin)) / assign17400_e11913))),)
    } else {
        (locals.var_vbipn, locals.var_vbipn_dn0, locals.var_vbipn_dn2, locals.var_vbipn_dn4, locals.var_vbipn_dn5, locals.var_vbipn_dn6, locals.var_vbipn_dn7, locals.var_vbipn_dn8, locals.var_vbipn_dn9, locals.var_vbipn_dn10, locals.var_vbipn_dn13,)
    }
};
        locals.var_vbipn = assign17400_e11917;
        locals.var_vbipn_dn0 = assign17400_e11917_d_n0;
        locals.var_vbipn_dn2 = assign17400_e11917_d_n2;
        locals.var_vbipn_dn4 = assign17400_e11917_d_n4;
        locals.var_vbipn_dn5 = assign17400_e11917_d_n5;
        locals.var_vbipn_dn6 = assign17400_e11917_d_n6;
        locals.var_vbipn_dn7 = assign17400_e11917_d_n7;
        locals.var_vbipn_dn8 = assign17400_e11917_d_n8;
        locals.var_vbipn_dn9 = assign17400_e11917_d_n9;
        locals.var_vbipn_dn10 = assign17400_e11917_d_n10;
        locals.var_vbipn_dn13 = assign17400_e11917_d_n13;

        let (assign17410_e11926, assign17410_e11926_d_n0, assign17410_e11926_d_n2, assign17410_e11926_d_n4, assign17410_e11926_d_n5, assign17410_e11926_d_n6, assign17410_e11926_d_n7, assign17410_e11926_d_n8, assign17410_e11926_d_n9, assign17410_e11926_d_n10, assign17410_e11926_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard355 != 0.0)) {
        let assign17410_e11923: f64 = (locals.var_log_tratio * p.p380);
        let assign17410_e11924: f64 = (assign17410_e11923).exp();
        (assign17410_e11924, (assign17410_e11924 * (locals.var_log_tratio_dn0 * p.p380)), (assign17410_e11924 * (locals.var_log_tratio_dn2 * p.p380)), (assign17410_e11924 * (locals.var_log_tratio_dn4 * p.p380)), (assign17410_e11924 * (locals.var_log_tratio_dn5 * p.p380)), (assign17410_e11924 * (locals.var_log_tratio_dn6 * p.p380)), (assign17410_e11924 * (locals.var_log_tratio_dn7 * p.p380)), (assign17410_e11924 * (locals.var_log_tratio_dn8 * p.p380)), (assign17410_e11924 * (locals.var_log_tratio_dn9 * p.p380)), (assign17410_e11924 * (locals.var_log_tratio_dn10 * p.p380)), (assign17410_e11924 * (locals.var_log_tratio_dn13 * p.p380)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign17410_e11926;
        locals.var_t1_dn0 = assign17410_e11926_d_n0;
        locals.var_t1_dn2 = assign17410_e11926_d_n2;
        locals.var_t1_dn4 = assign17410_e11926_d_n4;
        locals.var_t1_dn5 = assign17410_e11926_d_n5;
        locals.var_t1_dn6 = assign17410_e11926_d_n6;
        locals.var_t1_dn7 = assign17410_e11926_d_n7;
        locals.var_t1_dn8 = assign17410_e11926_d_n8;
        locals.var_t1_dn9 = assign17410_e11926_d_n9;
        locals.var_t1_dn10 = assign17410_e11926_d_n10;
        locals.var_t1_dn13 = assign17410_e11926_d_n13;

        let (assign17420_e11934, assign17420_e11934_d_n0, assign17420_e11934_d_n2, assign17420_e11934_d_n4, assign17420_e11934_d_n5, assign17420_e11934_d_n6, assign17420_e11934_d_n7, assign17420_e11934_d_n8, assign17420_e11934_d_n9, assign17420_e11934_d_n10, assign17420_e11934_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard355 != 0.0)) {
        let assign17420_e11932: f64 = (locals.var_t1 / locals.var_uc_depmueph1);
        (assign17420_e11932, (locals.var_t1_dn0 / locals.var_uc_depmueph1), (locals.var_t1_dn2 / locals.var_uc_depmueph1), (locals.var_t1_dn4 / locals.var_uc_depmueph1), (locals.var_t1_dn5 / locals.var_uc_depmueph1), (locals.var_t1_dn6 / locals.var_uc_depmueph1), (locals.var_t1_dn7 / locals.var_uc_depmueph1), (locals.var_t1_dn8 / locals.var_uc_depmueph1), (locals.var_t1_dn9 / locals.var_uc_depmueph1), (locals.var_t1_dn10 / locals.var_uc_depmueph1), (locals.var_t1_dn13 / locals.var_uc_depmueph1),)
    } else {
        (locals.var_depmphn0, locals.var_depmphn0_dn0, locals.var_depmphn0_dn2, locals.var_depmphn0_dn4, locals.var_depmphn0_dn5, locals.var_depmphn0_dn6, locals.var_depmphn0_dn7, locals.var_depmphn0_dn8, locals.var_depmphn0_dn9, locals.var_depmphn0_dn10, locals.var_depmphn0_dn13,)
    }
};
        locals.var_depmphn0 = assign17420_e11934;
        locals.var_depmphn0_dn0 = assign17420_e11934_d_n0;
        locals.var_depmphn0_dn2 = assign17420_e11934_d_n2;
        locals.var_depmphn0_dn4 = assign17420_e11934_d_n4;
        locals.var_depmphn0_dn5 = assign17420_e11934_d_n5;
        locals.var_depmphn0_dn6 = assign17420_e11934_d_n6;
        locals.var_depmphn0_dn7 = assign17420_e11934_d_n7;
        locals.var_depmphn0_dn8 = assign17420_e11934_d_n8;
        locals.var_depmphn0_dn9 = assign17420_e11934_d_n9;
        locals.var_depmphn0_dn10 = assign17420_e11934_d_n10;
        locals.var_depmphn0_dn13 = assign17420_e11934_d_n13;

        let (assign17430_e11956, assign17430_e11956_d_n0, assign17430_e11956_d_n2, assign17430_e11956_d_n4, assign17430_e11956_d_n5, assign17430_e11956_d_n6, assign17430_e11956_d_n7, assign17430_e11956_d_n8, assign17430_e11956_d_n9, assign17430_e11956_d_n10, assign17430_e11956_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard355 != 0.0)) {
        let assign17430_e11941: f64 = (0.4 * locals.var_tratio);
        let assign17430_e11942: f64 = (1.8 + assign17430_e11941);
        let assign17430_e11945: f64 = (0.1 * locals.var_tratio);
        let assign17430_e11947: f64 = (assign17430_e11945 * locals.var_tratio);
        let assign17430_e11948: f64 = (assign17430_e11942 + assign17430_e11947);
        let assign17430_e11952: f64 = (1.0 - locals.var_tratio);
        let assign17430_e11953: f64 = (p.p379 * assign17430_e11952);
        let assign17430_e11954: f64 = (assign17430_e11948 - assign17430_e11953);
        (assign17430_e11954, (((0.4 * locals.var_tratio_dn0) + (((0.1 * locals.var_tratio_dn0) * locals.var_tratio) + (assign17430_e11945 * locals.var_tratio_dn0))) - (p.p379 * (-locals.var_tratio_dn0))), (((0.4 * locals.var_tratio_dn2) + (((0.1 * locals.var_tratio_dn2) * locals.var_tratio) + (assign17430_e11945 * locals.var_tratio_dn2))) - (p.p379 * (-locals.var_tratio_dn2))), (((0.4 * locals.var_tratio_dn4) + (((0.1 * locals.var_tratio_dn4) * locals.var_tratio) + (assign17430_e11945 * locals.var_tratio_dn4))) - (p.p379 * (-locals.var_tratio_dn4))), (((0.4 * locals.var_tratio_dn5) + (((0.1 * locals.var_tratio_dn5) * locals.var_tratio) + (assign17430_e11945 * locals.var_tratio_dn5))) - (p.p379 * (-locals.var_tratio_dn5))), (((0.4 * locals.var_tratio_dn6) + (((0.1 * locals.var_tratio_dn6) * locals.var_tratio) + (assign17430_e11945 * locals.var_tratio_dn6))) - (p.p379 * (-locals.var_tratio_dn6))), (((0.4 * locals.var_tratio_dn7) + (((0.1 * locals.var_tratio_dn7) * locals.var_tratio) + (assign17430_e11945 * locals.var_tratio_dn7))) - (p.p379 * (-locals.var_tratio_dn7))), (((0.4 * locals.var_tratio_dn8) + (((0.1 * locals.var_tratio_dn8) * locals.var_tratio) + (assign17430_e11945 * locals.var_tratio_dn8))) - (p.p379 * (-locals.var_tratio_dn8))), (((0.4 * locals.var_tratio_dn9) + (((0.1 * locals.var_tratio_dn9) * locals.var_tratio) + (assign17430_e11945 * locals.var_tratio_dn9))) - (p.p379 * (-locals.var_tratio_dn9))), (((0.4 * locals.var_tratio_dn10) + (((0.1 * locals.var_tratio_dn10) * locals.var_tratio) + (assign17430_e11945 * locals.var_tratio_dn10))) - (p.p379 * (-locals.var_tratio_dn10))), (((0.4 * locals.var_tratio_dn13) + (((0.1 * locals.var_tratio_dn13) * locals.var_tratio) + (assign17430_e11945 * locals.var_tratio_dn13))) - (p.p379 * (-locals.var_tratio_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign17430_e11956;
        locals.var_t0_dn0 = assign17430_e11956_d_n0;
        locals.var_t0_dn2 = assign17430_e11956_d_n2;
        locals.var_t0_dn4 = assign17430_e11956_d_n4;
        locals.var_t0_dn5 = assign17430_e11956_d_n5;
        locals.var_t0_dn6 = assign17430_e11956_d_n6;
        locals.var_t0_dn7 = assign17430_e11956_d_n7;
        locals.var_t0_dn8 = assign17430_e11956_d_n8;
        locals.var_t0_dn9 = assign17430_e11956_d_n9;
        locals.var_t0_dn10 = assign17430_e11956_d_n10;
        locals.var_t0_dn13 = assign17430_e11956_d_n13;

        let (assign17440_e11964, assign17440_e11964_d_n0, assign17440_e11964_d_n2, assign17440_e11964_d_n4, assign17440_e11964_d_n5, assign17440_e11964_d_n6, assign17440_e11964_d_n7, assign17440_e11964_d_n8, assign17440_e11964_d_n9, assign17440_e11964_d_n10, assign17440_e11964_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard355 != 0.0)) {
        let assign17440_e11962: f64 = (locals.var_uc_depvmax / locals.var_t0);
        (assign17440_e11962, (((locals.var_uc_depvmax_dn0 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn0)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn2 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn2)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn4 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn5 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn6 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn7 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn8 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn9 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn10 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn13 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn13)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_uc_depvmax, locals.var_uc_depvmax_dn0, locals.var_uc_depvmax_dn2, locals.var_uc_depvmax_dn4, locals.var_uc_depvmax_dn5, locals.var_uc_depvmax_dn6, locals.var_uc_depvmax_dn7, locals.var_uc_depvmax_dn8, locals.var_uc_depvmax_dn9, locals.var_uc_depvmax_dn10, locals.var_uc_depvmax_dn13,)
    }
};
        locals.var_uc_depvmax = assign17440_e11964;
        locals.var_uc_depvmax_dn0 = assign17440_e11964_d_n0;
        locals.var_uc_depvmax_dn2 = assign17440_e11964_d_n2;
        locals.var_uc_depvmax_dn4 = assign17440_e11964_d_n4;
        locals.var_uc_depvmax_dn5 = assign17440_e11964_d_n5;
        locals.var_uc_depvmax_dn6 = assign17440_e11964_d_n6;
        locals.var_uc_depvmax_dn7 = assign17440_e11964_d_n7;
        locals.var_uc_depvmax_dn8 = assign17440_e11964_d_n8;
        locals.var_uc_depvmax_dn9 = assign17440_e11964_d_n9;
        locals.var_uc_depvmax_dn10 = assign17440_e11964_d_n10;
        locals.var_uc_depvmax_dn13 = assign17440_e11964_d_n13;

        let assign17460_e11972: f64 = if locals.var_uc_depvmax < 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard357 = assign17460_e11972;

        let (assign17470_e11980, assign17470_e11980_d_n0, assign17470_e11980_d_n2, assign17470_e11980_d_n4, assign17470_e11980_d_n5, assign17470_e11980_d_n6, assign17470_e11980_d_n7, assign17470_e11980_d_n8, assign17470_e11980_d_n9, assign17470_e11980_d_n10, assign17470_e11980_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard355 != 0.0)) && (locals.var_guard357 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depvmax, locals.var_uc_depvmax_dn0, locals.var_uc_depvmax_dn2, locals.var_uc_depvmax_dn4, locals.var_uc_depvmax_dn5, locals.var_uc_depvmax_dn6, locals.var_uc_depvmax_dn7, locals.var_uc_depvmax_dn8, locals.var_uc_depvmax_dn9, locals.var_uc_depvmax_dn10, locals.var_uc_depvmax_dn13,)
    }
};
        locals.var_uc_depvmax = assign17470_e11980;
        locals.var_uc_depvmax_dn0 = assign17470_e11980_d_n0;
        locals.var_uc_depvmax_dn2 = assign17470_e11980_d_n2;
        locals.var_uc_depvmax_dn4 = assign17470_e11980_d_n4;
        locals.var_uc_depvmax_dn5 = assign17470_e11980_d_n5;
        locals.var_uc_depvmax_dn6 = assign17470_e11980_d_n6;
        locals.var_uc_depvmax_dn7 = assign17470_e11980_d_n7;
        locals.var_uc_depvmax_dn8 = assign17470_e11980_d_n8;
        locals.var_uc_depvmax_dn9 = assign17470_e11980_d_n9;
        locals.var_uc_depvmax_dn10 = assign17470_e11980_d_n10;
        locals.var_uc_depvmax_dn13 = assign17470_e11980_d_n13;

        let (assign17480_e11990, assign17480_e11990_d_n0, assign17480_e11990_d_n2, assign17480_e11990_d_n4, assign17480_e11990_d_n5, assign17480_e11990_d_n6, assign17480_e11990_d_n7, assign17480_e11990_d_n8, assign17480_e11990_d_n9, assign17480_e11990_d_n10, assign17480_e11990_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard355 != 0.0)) {
        let assign17480_e11987: f64 = (locals.var_tratio).powf(p.p381);
        let assign17480_e11988: f64 = (locals.var_uc_depmue0 / assign17480_e11987);
        (assign17480_e11988, (((locals.var_uc_depmue0_dn0 * assign17480_e11987) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn0)) } } else { (assign17480_e11987 * (p.p381 * (locals.var_tratio_dn0 / locals.var_tratio))) })) / (assign17480_e11987 * assign17480_e11987)), (((locals.var_uc_depmue0_dn2 * assign17480_e11987) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn2)) } } else { (assign17480_e11987 * (p.p381 * (locals.var_tratio_dn2 / locals.var_tratio))) })) / (assign17480_e11987 * assign17480_e11987)), (((locals.var_uc_depmue0_dn4 * assign17480_e11987) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn4)) } } else { (assign17480_e11987 * (p.p381 * (locals.var_tratio_dn4 / locals.var_tratio))) })) / (assign17480_e11987 * assign17480_e11987)), (((locals.var_uc_depmue0_dn5 * assign17480_e11987) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn5)) } } else { (assign17480_e11987 * (p.p381 * (locals.var_tratio_dn5 / locals.var_tratio))) })) / (assign17480_e11987 * assign17480_e11987)), (((locals.var_uc_depmue0_dn6 * assign17480_e11987) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn6)) } } else { (assign17480_e11987 * (p.p381 * (locals.var_tratio_dn6 / locals.var_tratio))) })) / (assign17480_e11987 * assign17480_e11987)), (((locals.var_uc_depmue0_dn7 * assign17480_e11987) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn7)) } } else { (assign17480_e11987 * (p.p381 * (locals.var_tratio_dn7 / locals.var_tratio))) })) / (assign17480_e11987 * assign17480_e11987)), (((locals.var_uc_depmue0_dn8 * assign17480_e11987) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn8)) } } else { (assign17480_e11987 * (p.p381 * (locals.var_tratio_dn8 / locals.var_tratio))) })) / (assign17480_e11987 * assign17480_e11987)), (((locals.var_uc_depmue0_dn9 * assign17480_e11987) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn9)) } } else { (assign17480_e11987 * (p.p381 * (locals.var_tratio_dn9 / locals.var_tratio))) })) / (assign17480_e11987 * assign17480_e11987)), (((locals.var_uc_depmue0_dn10 * assign17480_e11987) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn10)) } } else { (assign17480_e11987 * (p.p381 * (locals.var_tratio_dn10 / locals.var_tratio))) })) / (assign17480_e11987 * assign17480_e11987)), (((locals.var_uc_depmue0_dn13 * assign17480_e11987) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn13)) } } else { (assign17480_e11987 * (p.p381 * (locals.var_tratio_dn13 / locals.var_tratio))) })) / (assign17480_e11987 * assign17480_e11987)),)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn13,)
    }
};
        locals.var_uc_depmue0 = assign17480_e11990;
        locals.var_uc_depmue0_dn0 = assign17480_e11990_d_n0;
        locals.var_uc_depmue0_dn2 = assign17480_e11990_d_n2;
        locals.var_uc_depmue0_dn4 = assign17480_e11990_d_n4;
        locals.var_uc_depmue0_dn5 = assign17480_e11990_d_n5;
        locals.var_uc_depmue0_dn6 = assign17480_e11990_d_n6;
        locals.var_uc_depmue0_dn7 = assign17480_e11990_d_n7;
        locals.var_uc_depmue0_dn8 = assign17480_e11990_d_n8;
        locals.var_uc_depmue0_dn9 = assign17480_e11990_d_n9;
        locals.var_uc_depmue0_dn10 = assign17480_e11990_d_n10;
        locals.var_uc_depmue0_dn13 = assign17480_e11990_d_n13;

        let (assign17490_e12000, assign17490_e12000_d_n0, assign17490_e12000_d_n2, assign17490_e12000_d_n4, assign17490_e12000_d_n5, assign17490_e12000_d_n6, assign17490_e12000_d_n7, assign17490_e12000_d_n8, assign17490_e12000_d_n9, assign17490_e12000_d_n10, assign17490_e12000_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard355 != 0.0)) {
        let assign17490_e11997: f64 = (locals.var_tratio).powf(p.p382);
        let assign17490_e11998: f64 = (locals.var_uc_depmue2 / assign17490_e11997);
        (assign17490_e11998, (((locals.var_uc_depmue2_dn0 * assign17490_e11997) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn0)) } } else { (assign17490_e11997 * (p.p382 * (locals.var_tratio_dn0 / locals.var_tratio))) })) / (assign17490_e11997 * assign17490_e11997)), (((locals.var_uc_depmue2_dn2 * assign17490_e11997) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn2)) } } else { (assign17490_e11997 * (p.p382 * (locals.var_tratio_dn2 / locals.var_tratio))) })) / (assign17490_e11997 * assign17490_e11997)), (((locals.var_uc_depmue2_dn4 * assign17490_e11997) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn4)) } } else { (assign17490_e11997 * (p.p382 * (locals.var_tratio_dn4 / locals.var_tratio))) })) / (assign17490_e11997 * assign17490_e11997)), (((locals.var_uc_depmue2_dn5 * assign17490_e11997) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn5)) } } else { (assign17490_e11997 * (p.p382 * (locals.var_tratio_dn5 / locals.var_tratio))) })) / (assign17490_e11997 * assign17490_e11997)), (((locals.var_uc_depmue2_dn6 * assign17490_e11997) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn6)) } } else { (assign17490_e11997 * (p.p382 * (locals.var_tratio_dn6 / locals.var_tratio))) })) / (assign17490_e11997 * assign17490_e11997)), (((locals.var_uc_depmue2_dn7 * assign17490_e11997) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn7)) } } else { (assign17490_e11997 * (p.p382 * (locals.var_tratio_dn7 / locals.var_tratio))) })) / (assign17490_e11997 * assign17490_e11997)), (((locals.var_uc_depmue2_dn8 * assign17490_e11997) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn8)) } } else { (assign17490_e11997 * (p.p382 * (locals.var_tratio_dn8 / locals.var_tratio))) })) / (assign17490_e11997 * assign17490_e11997)), (((locals.var_uc_depmue2_dn9 * assign17490_e11997) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn9)) } } else { (assign17490_e11997 * (p.p382 * (locals.var_tratio_dn9 / locals.var_tratio))) })) / (assign17490_e11997 * assign17490_e11997)), (((locals.var_uc_depmue2_dn10 * assign17490_e11997) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn10)) } } else { (assign17490_e11997 * (p.p382 * (locals.var_tratio_dn10 / locals.var_tratio))) })) / (assign17490_e11997 * assign17490_e11997)), (((locals.var_uc_depmue2_dn13 * assign17490_e11997) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn13)) } } else { (assign17490_e11997 * (p.p382 * (locals.var_tratio_dn13 / locals.var_tratio))) })) / (assign17490_e11997 * assign17490_e11997)),)
    } else {
        (locals.var_uc_depmue2, locals.var_uc_depmue2_dn0, locals.var_uc_depmue2_dn2, locals.var_uc_depmue2_dn4, locals.var_uc_depmue2_dn5, locals.var_uc_depmue2_dn6, locals.var_uc_depmue2_dn7, locals.var_uc_depmue2_dn8, locals.var_uc_depmue2_dn9, locals.var_uc_depmue2_dn10, locals.var_uc_depmue2_dn13,)
    }
};
        locals.var_uc_depmue2 = assign17490_e12000;
        locals.var_uc_depmue2_dn0 = assign17490_e12000_d_n0;
        locals.var_uc_depmue2_dn2 = assign17490_e12000_d_n2;
        locals.var_uc_depmue2_dn4 = assign17490_e12000_d_n4;
        locals.var_uc_depmue2_dn5 = assign17490_e12000_d_n5;
        locals.var_uc_depmue2_dn6 = assign17490_e12000_d_n6;
        locals.var_uc_depmue2_dn7 = assign17490_e12000_d_n7;
        locals.var_uc_depmue2_dn8 = assign17490_e12000_d_n8;
        locals.var_uc_depmue2_dn9 = assign17490_e12000_d_n9;
        locals.var_uc_depmue2_dn10 = assign17490_e12000_d_n10;
        locals.var_uc_depmue2_dn13 = assign17490_e12000_d_n13;

        let assign17500_e12003: f64 = if locals.var_uc_codep == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard358 = assign17500_e12003;

        let (assign17510_e12021, assign17510_e12021_d_n0, assign17510_e12021_d_n2, assign17510_e12021_d_n4, assign17510_e12021_d_n5, assign17510_e12021_d_n6, assign17510_e12021_d_n7, assign17510_e12021_d_n8, assign17510_e12021_d_n9, assign17510_e12021_d_n10, assign17510_e12021_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard355 == 0.0)) && (locals.var_guard358 != 0.0)) {
        let assign17510_e12012: f64 = (2.0 * 1.034943e-10);
        let assign17510_e12014: f64 = (assign17510_e12012 * 1.6021918e-19);
        let assign17510_e12016: f64 = (assign17510_e12014 * locals.var_uc_ndepm);
        let assign17510_e12018: f64 = (assign17510_e12016 * locals.var_beta_inv);
        let assign17510_e12019: f64 = (assign17510_e12018).sqrt();
        (assign17510_e12019, ((((assign17510_e12014 * locals.var_uc_ndepm_dn0) * locals.var_beta_inv) + (assign17510_e12016 * locals.var_beta_inv_dn0)) / (2.0 * assign17510_e12019)), ((((assign17510_e12014 * locals.var_uc_ndepm_dn2) * locals.var_beta_inv) + (assign17510_e12016 * locals.var_beta_inv_dn2)) / (2.0 * assign17510_e12019)), ((((assign17510_e12014 * locals.var_uc_ndepm_dn4) * locals.var_beta_inv) + (assign17510_e12016 * locals.var_beta_inv_dn4)) / (2.0 * assign17510_e12019)), ((((assign17510_e12014 * locals.var_uc_ndepm_dn5) * locals.var_beta_inv) + (assign17510_e12016 * locals.var_beta_inv_dn5)) / (2.0 * assign17510_e12019)), ((((assign17510_e12014 * locals.var_uc_ndepm_dn6) * locals.var_beta_inv) + (assign17510_e12016 * locals.var_beta_inv_dn6)) / (2.0 * assign17510_e12019)), ((((assign17510_e12014 * locals.var_uc_ndepm_dn7) * locals.var_beta_inv) + (assign17510_e12016 * locals.var_beta_inv_dn7)) / (2.0 * assign17510_e12019)), ((((assign17510_e12014 * locals.var_uc_ndepm_dn8) * locals.var_beta_inv) + (assign17510_e12016 * locals.var_beta_inv_dn8)) / (2.0 * assign17510_e12019)), ((((assign17510_e12014 * locals.var_uc_ndepm_dn9) * locals.var_beta_inv) + (assign17510_e12016 * locals.var_beta_inv_dn9)) / (2.0 * assign17510_e12019)), ((((assign17510_e12014 * locals.var_uc_ndepm_dn10) * locals.var_beta_inv) + (assign17510_e12016 * locals.var_beta_inv_dn10)) / (2.0 * assign17510_e12019)), ((((assign17510_e12014 * locals.var_uc_ndepm_dn13) * locals.var_beta_inv) + (assign17510_e12016 * locals.var_beta_inv_dn13)) / (2.0 * assign17510_e12019)),)
    } else {
        (locals.var_cnst0, locals.var_cnst0_dn0, locals.var_cnst0_dn2, locals.var_cnst0_dn4, locals.var_cnst0_dn5, locals.var_cnst0_dn6, locals.var_cnst0_dn7, locals.var_cnst0_dn8, locals.var_cnst0_dn9, locals.var_cnst0_dn10, locals.var_cnst0_dn13,)
    }
};
        locals.var_cnst0 = assign17510_e12021;
        locals.var_cnst0_dn0 = assign17510_e12021_d_n0;
        locals.var_cnst0_dn2 = assign17510_e12021_d_n2;
        locals.var_cnst0_dn4 = assign17510_e12021_d_n4;
        locals.var_cnst0_dn5 = assign17510_e12021_d_n5;
        locals.var_cnst0_dn6 = assign17510_e12021_d_n6;
        locals.var_cnst0_dn7 = assign17510_e12021_d_n7;
        locals.var_cnst0_dn8 = assign17510_e12021_d_n8;
        locals.var_cnst0_dn9 = assign17510_e12021_d_n9;
        locals.var_cnst0_dn10 = assign17510_e12021_d_n10;
        locals.var_cnst0_dn13 = assign17510_e12021_d_n13;

        let (assign17520_e12036, assign17520_e12036_d_n0, assign17520_e12036_d_n2, assign17520_e12036_d_n4, assign17520_e12036_d_n5, assign17520_e12036_d_n6, assign17520_e12036_d_n7, assign17520_e12036_d_n8, assign17520_e12036_d_n9, assign17520_e12036_d_n10, assign17520_e12036_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard355 == 0.0)) && (locals.var_guard358 != 0.0)) {
        let assign17520_e12030: f64 = (locals.var_nin * locals.var_nin);
        let __rspice_inv_cse_2: f64 = 1.0 / locals.var_uc_ndepm;
        let assign17520_e12032: f64 = (assign17520_e12030 * __rspice_inv_cse_2);
        let assign17520_e12034: f64 = (assign17520_e12032 * __rspice_inv_cse_2);
        (assign17520_e12034, ((((((((locals.var_nin_dn0 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn0)) * locals.var_uc_ndepm) - (assign17520_e12030 * locals.var_uc_ndepm_dn0)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17520_e12032 * locals.var_uc_ndepm_dn0)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn2 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn2)) * locals.var_uc_ndepm) - (assign17520_e12030 * locals.var_uc_ndepm_dn2)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17520_e12032 * locals.var_uc_ndepm_dn2)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn4 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn4)) * locals.var_uc_ndepm) - (assign17520_e12030 * locals.var_uc_ndepm_dn4)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17520_e12032 * locals.var_uc_ndepm_dn4)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn5 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn5)) * locals.var_uc_ndepm) - (assign17520_e12030 * locals.var_uc_ndepm_dn5)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17520_e12032 * locals.var_uc_ndepm_dn5)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn6 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn6)) * locals.var_uc_ndepm) - (assign17520_e12030 * locals.var_uc_ndepm_dn6)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17520_e12032 * locals.var_uc_ndepm_dn6)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn7 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn7)) * locals.var_uc_ndepm) - (assign17520_e12030 * locals.var_uc_ndepm_dn7)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17520_e12032 * locals.var_uc_ndepm_dn7)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn8 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn8)) * locals.var_uc_ndepm) - (assign17520_e12030 * locals.var_uc_ndepm_dn8)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17520_e12032 * locals.var_uc_ndepm_dn8)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn9 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn9)) * locals.var_uc_ndepm) - (assign17520_e12030 * locals.var_uc_ndepm_dn9)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17520_e12032 * locals.var_uc_ndepm_dn9)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn10 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn10)) * locals.var_uc_ndepm) - (assign17520_e12030 * locals.var_uc_ndepm_dn10)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17520_e12032 * locals.var_uc_ndepm_dn10)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn13 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn13)) * locals.var_uc_ndepm) - (assign17520_e12030 * locals.var_uc_ndepm_dn13)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17520_e12032 * locals.var_uc_ndepm_dn13)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)),)
    } else {
        (locals.var_cnst1, locals.var_cnst1_dn0, locals.var_cnst1_dn2, locals.var_cnst1_dn4, locals.var_cnst1_dn5, locals.var_cnst1_dn6, locals.var_cnst1_dn7, locals.var_cnst1_dn8, locals.var_cnst1_dn9, locals.var_cnst1_dn10, locals.var_cnst1_dn13,)
    }
};
        locals.var_cnst1 = assign17520_e12036;
        locals.var_cnst1_dn0 = assign17520_e12036_d_n0;
        locals.var_cnst1_dn2 = assign17520_e12036_d_n2;
        locals.var_cnst1_dn4 = assign17520_e12036_d_n4;
        locals.var_cnst1_dn5 = assign17520_e12036_d_n5;
        locals.var_cnst1_dn6 = assign17520_e12036_d_n6;
        locals.var_cnst1_dn7 = assign17520_e12036_d_n7;
        locals.var_cnst1_dn8 = assign17520_e12036_d_n8;
        locals.var_cnst1_dn9 = assign17520_e12036_d_n9;
        locals.var_cnst1_dn10 = assign17520_e12036_d_n10;
        locals.var_cnst1_dn13 = assign17520_e12036_d_n13;

        let (assign17530_e12052, assign17530_e12052_d_n0, assign17530_e12052_d_n2, assign17530_e12052_d_n4, assign17530_e12052_d_n5, assign17530_e12052_d_n6, assign17530_e12052_d_n7, assign17530_e12052_d_n8, assign17530_e12052_d_n9, assign17530_e12052_d_n10, assign17530_e12052_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard355 == 0.0)) && (locals.var_guard358 != 0.0)) {
        let assign17530_e12045: f64 = (2.0 * locals.var_beta_inv);
        let assign17530_e12048: f64 = (locals.var_uc_ndepm / locals.var_nin);
        let assign17530_e12049: f64 = (assign17530_e12048).ln();
        let assign17530_e12050: f64 = (assign17530_e12045 * assign17530_e12049);
        (assign17530_e12050, (((2.0 * locals.var_beta_inv_dn0) * assign17530_e12049) + (assign17530_e12045 * ((((locals.var_uc_ndepm_dn0 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign17530_e12048))), (((2.0 * locals.var_beta_inv_dn2) * assign17530_e12049) + (assign17530_e12045 * ((((locals.var_uc_ndepm_dn2 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign17530_e12048))), (((2.0 * locals.var_beta_inv_dn4) * assign17530_e12049) + (assign17530_e12045 * ((((locals.var_uc_ndepm_dn4 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign17530_e12048))), (((2.0 * locals.var_beta_inv_dn5) * assign17530_e12049) + (assign17530_e12045 * ((((locals.var_uc_ndepm_dn5 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign17530_e12048))), (((2.0 * locals.var_beta_inv_dn6) * assign17530_e12049) + (assign17530_e12045 * ((((locals.var_uc_ndepm_dn6 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign17530_e12048))), (((2.0 * locals.var_beta_inv_dn7) * assign17530_e12049) + (assign17530_e12045 * ((((locals.var_uc_ndepm_dn7 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign17530_e12048))), (((2.0 * locals.var_beta_inv_dn8) * assign17530_e12049) + (assign17530_e12045 * ((((locals.var_uc_ndepm_dn8 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign17530_e12048))), (((2.0 * locals.var_beta_inv_dn9) * assign17530_e12049) + (assign17530_e12045 * ((((locals.var_uc_ndepm_dn9 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign17530_e12048))), (((2.0 * locals.var_beta_inv_dn10) * assign17530_e12049) + (assign17530_e12045 * ((((locals.var_uc_ndepm_dn10 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign17530_e12048))), (((2.0 * locals.var_beta_inv_dn13) * assign17530_e12049) + (assign17530_e12045 * ((((locals.var_uc_ndepm_dn13 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn13)) / (locals.var_nin * locals.var_nin)) / assign17530_e12048))),)
    } else {
        (locals.var_pb2n, locals.var_pb2n_dn0, locals.var_pb2n_dn2, locals.var_pb2n_dn4, locals.var_pb2n_dn5, locals.var_pb2n_dn6, locals.var_pb2n_dn7, locals.var_pb2n_dn8, locals.var_pb2n_dn9, locals.var_pb2n_dn10, locals.var_pb2n_dn13,)
    }
};
        locals.var_pb2n = assign17530_e12052;
        locals.var_pb2n_dn0 = assign17530_e12052_d_n0;
        locals.var_pb2n_dn2 = assign17530_e12052_d_n2;
        locals.var_pb2n_dn4 = assign17530_e12052_d_n4;
        locals.var_pb2n_dn5 = assign17530_e12052_d_n5;
        locals.var_pb2n_dn6 = assign17530_e12052_d_n6;
        locals.var_pb2n_dn7 = assign17530_e12052_d_n7;
        locals.var_pb2n_dn8 = assign17530_e12052_d_n8;
        locals.var_pb2n_dn9 = assign17530_e12052_d_n9;
        locals.var_pb2n_dn10 = assign17530_e12052_d_n10;
        locals.var_pb2n_dn13 = assign17530_e12052_d_n13;

        let (assign17540_e12070, assign17540_e12070_d_n0, assign17540_e12070_d_n2, assign17540_e12070_d_n4, assign17540_e12070_d_n5, assign17540_e12070_d_n6, assign17540_e12070_d_n7, assign17540_e12070_d_n8, assign17540_e12070_d_n9, assign17540_e12070_d_n10, assign17540_e12070_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard355 == 0.0)) && (locals.var_guard358 != 0.0)) {
        let assign17540_e12062: f64 = (locals.var_uc_ndepm * locals.var_ef_nsubc);
        let __rspice_inv_cse_3: f64 = 1.0 / locals.var_nin;
        let assign17540_e12064: f64 = (assign17540_e12062 * __rspice_inv_cse_3);
        let assign17540_e12066: f64 = (assign17540_e12064 * __rspice_inv_cse_3);
        let assign17540_e12067: f64 = (assign17540_e12066).ln();
        let assign17540_e12068: f64 = (locals.var_beta_inv * assign17540_e12067);
        (assign17540_e12068, ((locals.var_beta_inv_dn0 * assign17540_e12067) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn0 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn0)) * locals.var_nin) - (assign17540_e12062 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17540_e12064 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign17540_e12066))), ((locals.var_beta_inv_dn2 * assign17540_e12067) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn2 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn2)) * locals.var_nin) - (assign17540_e12062 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17540_e12064 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign17540_e12066))), ((locals.var_beta_inv_dn4 * assign17540_e12067) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn4 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn4)) * locals.var_nin) - (assign17540_e12062 * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17540_e12064 * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign17540_e12066))), ((locals.var_beta_inv_dn5 * assign17540_e12067) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn5 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn5)) * locals.var_nin) - (assign17540_e12062 * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17540_e12064 * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign17540_e12066))), ((locals.var_beta_inv_dn6 * assign17540_e12067) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn6 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn6)) * locals.var_nin) - (assign17540_e12062 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17540_e12064 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign17540_e12066))), ((locals.var_beta_inv_dn7 * assign17540_e12067) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn7 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn7)) * locals.var_nin) - (assign17540_e12062 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17540_e12064 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign17540_e12066))), ((locals.var_beta_inv_dn8 * assign17540_e12067) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn8 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn8)) * locals.var_nin) - (assign17540_e12062 * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17540_e12064 * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign17540_e12066))), ((locals.var_beta_inv_dn9 * assign17540_e12067) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn9 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn9)) * locals.var_nin) - (assign17540_e12062 * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17540_e12064 * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign17540_e12066))), ((locals.var_beta_inv_dn10 * assign17540_e12067) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn10 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn10)) * locals.var_nin) - (assign17540_e12062 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17540_e12064 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign17540_e12066))), ((locals.var_beta_inv_dn13 * assign17540_e12067) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn13 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn13)) * locals.var_nin) - (assign17540_e12062 * locals.var_nin_dn13)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17540_e12064 * locals.var_nin_dn13)) / (locals.var_nin * locals.var_nin)) / assign17540_e12066))),)
    } else {
        (locals.var_vbipn, locals.var_vbipn_dn0, locals.var_vbipn_dn2, locals.var_vbipn_dn4, locals.var_vbipn_dn5, locals.var_vbipn_dn6, locals.var_vbipn_dn7, locals.var_vbipn_dn8, locals.var_vbipn_dn9, locals.var_vbipn_dn10, locals.var_vbipn_dn13,)
    }
};
        locals.var_vbipn = assign17540_e12070;
        locals.var_vbipn_dn0 = assign17540_e12070_d_n0;
        locals.var_vbipn_dn2 = assign17540_e12070_d_n2;
        locals.var_vbipn_dn4 = assign17540_e12070_d_n4;
        locals.var_vbipn_dn5 = assign17540_e12070_d_n5;
        locals.var_vbipn_dn6 = assign17540_e12070_d_n6;
        locals.var_vbipn_dn7 = assign17540_e12070_d_n7;
        locals.var_vbipn_dn8 = assign17540_e12070_d_n8;
        locals.var_vbipn_dn9 = assign17540_e12070_d_n9;
        locals.var_vbipn_dn10 = assign17540_e12070_d_n10;
        locals.var_vbipn_dn13 = assign17540_e12070_d_n13;

        let (assign17550_e12082, assign17550_e12082_d_n0, assign17550_e12082_d_n2, assign17550_e12082_d_n4, assign17550_e12082_d_n5, assign17550_e12082_d_n6, assign17550_e12082_d_n7, assign17550_e12082_d_n8, assign17550_e12082_d_n9, assign17550_e12082_d_n10, assign17550_e12082_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard355 == 0.0)) && (locals.var_guard358 != 0.0)) {
        let assign17550_e12079: f64 = (locals.var_log_tratio * p.p380);
        let assign17550_e12080: f64 = (assign17550_e12079).exp();
        (assign17550_e12080, (assign17550_e12080 * (locals.var_log_tratio_dn0 * p.p380)), (assign17550_e12080 * (locals.var_log_tratio_dn2 * p.p380)), (assign17550_e12080 * (locals.var_log_tratio_dn4 * p.p380)), (assign17550_e12080 * (locals.var_log_tratio_dn5 * p.p380)), (assign17550_e12080 * (locals.var_log_tratio_dn6 * p.p380)), (assign17550_e12080 * (locals.var_log_tratio_dn7 * p.p380)), (assign17550_e12080 * (locals.var_log_tratio_dn8 * p.p380)), (assign17550_e12080 * (locals.var_log_tratio_dn9 * p.p380)), (assign17550_e12080 * (locals.var_log_tratio_dn10 * p.p380)), (assign17550_e12080 * (locals.var_log_tratio_dn13 * p.p380)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign17550_e12082;
        locals.var_t1_dn0 = assign17550_e12082_d_n0;
        locals.var_t1_dn2 = assign17550_e12082_d_n2;
        locals.var_t1_dn4 = assign17550_e12082_d_n4;
        locals.var_t1_dn5 = assign17550_e12082_d_n5;
        locals.var_t1_dn6 = assign17550_e12082_d_n6;
        locals.var_t1_dn7 = assign17550_e12082_d_n7;
        locals.var_t1_dn8 = assign17550_e12082_d_n8;
        locals.var_t1_dn9 = assign17550_e12082_d_n9;
        locals.var_t1_dn10 = assign17550_e12082_d_n10;
        locals.var_t1_dn13 = assign17550_e12082_d_n13;

        let (assign17560_e12093, assign17560_e12093_d_n0, assign17560_e12093_d_n2, assign17560_e12093_d_n4, assign17560_e12093_d_n5, assign17560_e12093_d_n6, assign17560_e12093_d_n7, assign17560_e12093_d_n8, assign17560_e12093_d_n9, assign17560_e12093_d_n10, assign17560_e12093_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard355 == 0.0)) && (locals.var_guard358 != 0.0)) {
        let assign17560_e12091: f64 = (locals.var_t1 / locals.var_uc_depmueph1);
        (assign17560_e12091, (locals.var_t1_dn0 / locals.var_uc_depmueph1), (locals.var_t1_dn2 / locals.var_uc_depmueph1), (locals.var_t1_dn4 / locals.var_uc_depmueph1), (locals.var_t1_dn5 / locals.var_uc_depmueph1), (locals.var_t1_dn6 / locals.var_uc_depmueph1), (locals.var_t1_dn7 / locals.var_uc_depmueph1), (locals.var_t1_dn8 / locals.var_uc_depmueph1), (locals.var_t1_dn9 / locals.var_uc_depmueph1), (locals.var_t1_dn10 / locals.var_uc_depmueph1), (locals.var_t1_dn13 / locals.var_uc_depmueph1),)
    } else {
        (locals.var_depmphn0, locals.var_depmphn0_dn0, locals.var_depmphn0_dn2, locals.var_depmphn0_dn4, locals.var_depmphn0_dn5, locals.var_depmphn0_dn6, locals.var_depmphn0_dn7, locals.var_depmphn0_dn8, locals.var_depmphn0_dn9, locals.var_depmphn0_dn10, locals.var_depmphn0_dn13,)
    }
};
        locals.var_depmphn0 = assign17560_e12093;
        locals.var_depmphn0_dn0 = assign17560_e12093_d_n0;
        locals.var_depmphn0_dn2 = assign17560_e12093_d_n2;
        locals.var_depmphn0_dn4 = assign17560_e12093_d_n4;
        locals.var_depmphn0_dn5 = assign17560_e12093_d_n5;
        locals.var_depmphn0_dn6 = assign17560_e12093_d_n6;
        locals.var_depmphn0_dn7 = assign17560_e12093_d_n7;
        locals.var_depmphn0_dn8 = assign17560_e12093_d_n8;
        locals.var_depmphn0_dn9 = assign17560_e12093_d_n9;
        locals.var_depmphn0_dn10 = assign17560_e12093_d_n10;
        locals.var_depmphn0_dn13 = assign17560_e12093_d_n13;

        let (assign17570_e12118, assign17570_e12118_d_n0, assign17570_e12118_d_n2, assign17570_e12118_d_n4, assign17570_e12118_d_n5, assign17570_e12118_d_n6, assign17570_e12118_d_n7, assign17570_e12118_d_n8, assign17570_e12118_d_n9, assign17570_e12118_d_n10, assign17570_e12118_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard355 == 0.0)) && (locals.var_guard358 != 0.0)) {
        let assign17570_e12103: f64 = (0.4 * locals.var_tratio);
        let assign17570_e12104: f64 = (1.8 + assign17570_e12103);
        let assign17570_e12107: f64 = (0.1 * locals.var_tratio);
        let assign17570_e12109: f64 = (assign17570_e12107 * locals.var_tratio);
        let assign17570_e12110: f64 = (assign17570_e12104 + assign17570_e12109);
        let assign17570_e12114: f64 = (1.0 - locals.var_tratio);
        let assign17570_e12115: f64 = (p.p379 * assign17570_e12114);
        let assign17570_e12116: f64 = (assign17570_e12110 - assign17570_e12115);
        (assign17570_e12116, (((0.4 * locals.var_tratio_dn0) + (((0.1 * locals.var_tratio_dn0) * locals.var_tratio) + (assign17570_e12107 * locals.var_tratio_dn0))) - (p.p379 * (-locals.var_tratio_dn0))), (((0.4 * locals.var_tratio_dn2) + (((0.1 * locals.var_tratio_dn2) * locals.var_tratio) + (assign17570_e12107 * locals.var_tratio_dn2))) - (p.p379 * (-locals.var_tratio_dn2))), (((0.4 * locals.var_tratio_dn4) + (((0.1 * locals.var_tratio_dn4) * locals.var_tratio) + (assign17570_e12107 * locals.var_tratio_dn4))) - (p.p379 * (-locals.var_tratio_dn4))), (((0.4 * locals.var_tratio_dn5) + (((0.1 * locals.var_tratio_dn5) * locals.var_tratio) + (assign17570_e12107 * locals.var_tratio_dn5))) - (p.p379 * (-locals.var_tratio_dn5))), (((0.4 * locals.var_tratio_dn6) + (((0.1 * locals.var_tratio_dn6) * locals.var_tratio) + (assign17570_e12107 * locals.var_tratio_dn6))) - (p.p379 * (-locals.var_tratio_dn6))), (((0.4 * locals.var_tratio_dn7) + (((0.1 * locals.var_tratio_dn7) * locals.var_tratio) + (assign17570_e12107 * locals.var_tratio_dn7))) - (p.p379 * (-locals.var_tratio_dn7))), (((0.4 * locals.var_tratio_dn8) + (((0.1 * locals.var_tratio_dn8) * locals.var_tratio) + (assign17570_e12107 * locals.var_tratio_dn8))) - (p.p379 * (-locals.var_tratio_dn8))), (((0.4 * locals.var_tratio_dn9) + (((0.1 * locals.var_tratio_dn9) * locals.var_tratio) + (assign17570_e12107 * locals.var_tratio_dn9))) - (p.p379 * (-locals.var_tratio_dn9))), (((0.4 * locals.var_tratio_dn10) + (((0.1 * locals.var_tratio_dn10) * locals.var_tratio) + (assign17570_e12107 * locals.var_tratio_dn10))) - (p.p379 * (-locals.var_tratio_dn10))), (((0.4 * locals.var_tratio_dn13) + (((0.1 * locals.var_tratio_dn13) * locals.var_tratio) + (assign17570_e12107 * locals.var_tratio_dn13))) - (p.p379 * (-locals.var_tratio_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign17570_e12118;
        locals.var_t0_dn0 = assign17570_e12118_d_n0;
        locals.var_t0_dn2 = assign17570_e12118_d_n2;
        locals.var_t0_dn4 = assign17570_e12118_d_n4;
        locals.var_t0_dn5 = assign17570_e12118_d_n5;
        locals.var_t0_dn6 = assign17570_e12118_d_n6;
        locals.var_t0_dn7 = assign17570_e12118_d_n7;
        locals.var_t0_dn8 = assign17570_e12118_d_n8;
        locals.var_t0_dn9 = assign17570_e12118_d_n9;
        locals.var_t0_dn10 = assign17570_e12118_d_n10;
        locals.var_t0_dn13 = assign17570_e12118_d_n13;

    }

    pub(super) fn stamp_transient_block_37(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign17580_e12129, assign17580_e12129_d_n0, assign17580_e12129_d_n2, assign17580_e12129_d_n4, assign17580_e12129_d_n5, assign17580_e12129_d_n6, assign17580_e12129_d_n7, assign17580_e12129_d_n8, assign17580_e12129_d_n9, assign17580_e12129_d_n10, assign17580_e12129_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard355 == 0.0)) && (locals.var_guard358 != 0.0)) {
        let assign17580_e12127: f64 = (locals.var_uc_depvmax / locals.var_t0);
        (assign17580_e12127, (((locals.var_uc_depvmax_dn0 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn0)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn2 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn2)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn4 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn5 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn6 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn7 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn8 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn9 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn10 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn13 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn13)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_uc_depvmax, locals.var_uc_depvmax_dn0, locals.var_uc_depvmax_dn2, locals.var_uc_depvmax_dn4, locals.var_uc_depvmax_dn5, locals.var_uc_depvmax_dn6, locals.var_uc_depvmax_dn7, locals.var_uc_depvmax_dn8, locals.var_uc_depvmax_dn9, locals.var_uc_depvmax_dn10, locals.var_uc_depvmax_dn13,)
    }
};
        locals.var_uc_depvmax = assign17580_e12129;
        locals.var_uc_depvmax_dn0 = assign17580_e12129_d_n0;
        locals.var_uc_depvmax_dn2 = assign17580_e12129_d_n2;
        locals.var_uc_depvmax_dn4 = assign17580_e12129_d_n4;
        locals.var_uc_depvmax_dn5 = assign17580_e12129_d_n5;
        locals.var_uc_depvmax_dn6 = assign17580_e12129_d_n6;
        locals.var_uc_depvmax_dn7 = assign17580_e12129_d_n7;
        locals.var_uc_depvmax_dn8 = assign17580_e12129_d_n8;
        locals.var_uc_depvmax_dn9 = assign17580_e12129_d_n9;
        locals.var_uc_depvmax_dn10 = assign17580_e12129_d_n10;
        locals.var_uc_depvmax_dn13 = assign17580_e12129_d_n13;

        let assign17600_e12137: f64 = if locals.var_uc_depvmax < 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard360 = assign17600_e12137;

        let (assign17610_e12148, assign17610_e12148_d_n0, assign17610_e12148_d_n2, assign17610_e12148_d_n4, assign17610_e12148_d_n5, assign17610_e12148_d_n6, assign17610_e12148_d_n7, assign17610_e12148_d_n8, assign17610_e12148_d_n9, assign17610_e12148_d_n10, assign17610_e12148_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard355 == 0.0)) && (locals.var_guard358 != 0.0)) && (locals.var_guard360 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depvmax, locals.var_uc_depvmax_dn0, locals.var_uc_depvmax_dn2, locals.var_uc_depvmax_dn4, locals.var_uc_depvmax_dn5, locals.var_uc_depvmax_dn6, locals.var_uc_depvmax_dn7, locals.var_uc_depvmax_dn8, locals.var_uc_depvmax_dn9, locals.var_uc_depvmax_dn10, locals.var_uc_depvmax_dn13,)
    }
};
        locals.var_uc_depvmax = assign17610_e12148;
        locals.var_uc_depvmax_dn0 = assign17610_e12148_d_n0;
        locals.var_uc_depvmax_dn2 = assign17610_e12148_d_n2;
        locals.var_uc_depvmax_dn4 = assign17610_e12148_d_n4;
        locals.var_uc_depvmax_dn5 = assign17610_e12148_d_n5;
        locals.var_uc_depvmax_dn6 = assign17610_e12148_d_n6;
        locals.var_uc_depvmax_dn7 = assign17610_e12148_d_n7;
        locals.var_uc_depvmax_dn8 = assign17610_e12148_d_n8;
        locals.var_uc_depvmax_dn9 = assign17610_e12148_d_n9;
        locals.var_uc_depvmax_dn10 = assign17610_e12148_d_n10;
        locals.var_uc_depvmax_dn13 = assign17610_e12148_d_n13;

        let (assign17620_e12161, assign17620_e12161_d_n0, assign17620_e12161_d_n2, assign17620_e12161_d_n4, assign17620_e12161_d_n5, assign17620_e12161_d_n6, assign17620_e12161_d_n7, assign17620_e12161_d_n8, assign17620_e12161_d_n9, assign17620_e12161_d_n10, assign17620_e12161_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard355 == 0.0)) && (locals.var_guard358 != 0.0)) {
        let assign17620_e12158: f64 = (locals.var_tratio).powf(p.p381);
        let assign17620_e12159: f64 = (locals.var_uc_depmue0 / assign17620_e12158);
        (assign17620_e12159, (((locals.var_uc_depmue0_dn0 * assign17620_e12158) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn0)) } } else { (assign17620_e12158 * (p.p381 * (locals.var_tratio_dn0 / locals.var_tratio))) })) / (assign17620_e12158 * assign17620_e12158)), (((locals.var_uc_depmue0_dn2 * assign17620_e12158) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn2)) } } else { (assign17620_e12158 * (p.p381 * (locals.var_tratio_dn2 / locals.var_tratio))) })) / (assign17620_e12158 * assign17620_e12158)), (((locals.var_uc_depmue0_dn4 * assign17620_e12158) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn4)) } } else { (assign17620_e12158 * (p.p381 * (locals.var_tratio_dn4 / locals.var_tratio))) })) / (assign17620_e12158 * assign17620_e12158)), (((locals.var_uc_depmue0_dn5 * assign17620_e12158) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn5)) } } else { (assign17620_e12158 * (p.p381 * (locals.var_tratio_dn5 / locals.var_tratio))) })) / (assign17620_e12158 * assign17620_e12158)), (((locals.var_uc_depmue0_dn6 * assign17620_e12158) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn6)) } } else { (assign17620_e12158 * (p.p381 * (locals.var_tratio_dn6 / locals.var_tratio))) })) / (assign17620_e12158 * assign17620_e12158)), (((locals.var_uc_depmue0_dn7 * assign17620_e12158) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn7)) } } else { (assign17620_e12158 * (p.p381 * (locals.var_tratio_dn7 / locals.var_tratio))) })) / (assign17620_e12158 * assign17620_e12158)), (((locals.var_uc_depmue0_dn8 * assign17620_e12158) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn8)) } } else { (assign17620_e12158 * (p.p381 * (locals.var_tratio_dn8 / locals.var_tratio))) })) / (assign17620_e12158 * assign17620_e12158)), (((locals.var_uc_depmue0_dn9 * assign17620_e12158) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn9)) } } else { (assign17620_e12158 * (p.p381 * (locals.var_tratio_dn9 / locals.var_tratio))) })) / (assign17620_e12158 * assign17620_e12158)), (((locals.var_uc_depmue0_dn10 * assign17620_e12158) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn10)) } } else { (assign17620_e12158 * (p.p381 * (locals.var_tratio_dn10 / locals.var_tratio))) })) / (assign17620_e12158 * assign17620_e12158)), (((locals.var_uc_depmue0_dn13 * assign17620_e12158) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn13)) } } else { (assign17620_e12158 * (p.p381 * (locals.var_tratio_dn13 / locals.var_tratio))) })) / (assign17620_e12158 * assign17620_e12158)),)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn13,)
    }
};
        locals.var_uc_depmue0 = assign17620_e12161;
        locals.var_uc_depmue0_dn0 = assign17620_e12161_d_n0;
        locals.var_uc_depmue0_dn2 = assign17620_e12161_d_n2;
        locals.var_uc_depmue0_dn4 = assign17620_e12161_d_n4;
        locals.var_uc_depmue0_dn5 = assign17620_e12161_d_n5;
        locals.var_uc_depmue0_dn6 = assign17620_e12161_d_n6;
        locals.var_uc_depmue0_dn7 = assign17620_e12161_d_n7;
        locals.var_uc_depmue0_dn8 = assign17620_e12161_d_n8;
        locals.var_uc_depmue0_dn9 = assign17620_e12161_d_n9;
        locals.var_uc_depmue0_dn10 = assign17620_e12161_d_n10;
        locals.var_uc_depmue0_dn13 = assign17620_e12161_d_n13;

        let (assign17630_e12176, assign17630_e12176_d_n0, assign17630_e12176_d_n2, assign17630_e12176_d_n4, assign17630_e12176_d_n5, assign17630_e12176_d_n6, assign17630_e12176_d_n7, assign17630_e12176_d_n8, assign17630_e12176_d_n9, assign17630_e12176_d_n10, assign17630_e12176_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard355 == 0.0)) && (locals.var_guard358 != 0.0)) {
        let assign17630_e12172: f64 = (locals.var_tratio - 1.0);
        let assign17630_e12173: f64 = (p.p365 * assign17630_e12172);
        let assign17630_e12174: f64 = (p.p364 + assign17630_e12173);
        (assign17630_e12174, (p.p365 * locals.var_tratio_dn0), (p.p365 * locals.var_tratio_dn2), (p.p365 * locals.var_tratio_dn4), (p.p365 * locals.var_tratio_dn5), (p.p365 * locals.var_tratio_dn6), (p.p365 * locals.var_tratio_dn7), (p.p365 * locals.var_tratio_dn8), (p.p365 * locals.var_tratio_dn9), (p.p365 * locals.var_tratio_dn10), (p.p365 * locals.var_tratio_dn13),)
    } else {
        (locals.var_uc_depwlp, locals.var_uc_depwlp_dn0, locals.var_uc_depwlp_dn2, locals.var_uc_depwlp_dn4, locals.var_uc_depwlp_dn5, locals.var_uc_depwlp_dn6, locals.var_uc_depwlp_dn7, locals.var_uc_depwlp_dn8, locals.var_uc_depwlp_dn9, locals.var_uc_depwlp_dn10, locals.var_uc_depwlp_dn13,)
    }
};
        locals.var_uc_depwlp = assign17630_e12176;
        locals.var_uc_depwlp_dn0 = assign17630_e12176_d_n0;
        locals.var_uc_depwlp_dn2 = assign17630_e12176_d_n2;
        locals.var_uc_depwlp_dn4 = assign17630_e12176_d_n4;
        locals.var_uc_depwlp_dn5 = assign17630_e12176_d_n5;
        locals.var_uc_depwlp_dn6 = assign17630_e12176_d_n6;
        locals.var_uc_depwlp_dn7 = assign17630_e12176_d_n7;
        locals.var_uc_depwlp_dn8 = assign17630_e12176_d_n8;
        locals.var_uc_depwlp_dn9 = assign17630_e12176_d_n9;
        locals.var_uc_depwlp_dn10 = assign17630_e12176_d_n10;
        locals.var_uc_depwlp_dn13 = assign17630_e12176_d_n13;

        let (assign17640_e12186, assign17640_e12186_d_n0, assign17640_e12186_d_n2, assign17640_e12186_d_n4, assign17640_e12186_d_n5, assign17640_e12186_d_n6, assign17640_e12186_d_n7, assign17640_e12186_d_n8, assign17640_e12186_d_n9, assign17640_e12186_d_n10, assign17640_e12186_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard355 == 0.0)) && (locals.var_guard358 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pb2n, locals.var_pb2n_dn0, locals.var_pb2n_dn2, locals.var_pb2n_dn4, locals.var_pb2n_dn5, locals.var_pb2n_dn6, locals.var_pb2n_dn7, locals.var_pb2n_dn8, locals.var_pb2n_dn9, locals.var_pb2n_dn10, locals.var_pb2n_dn13,)
    }
};
        locals.var_pb2n = assign17640_e12186;
        locals.var_pb2n_dn0 = assign17640_e12186_d_n0;
        locals.var_pb2n_dn2 = assign17640_e12186_d_n2;
        locals.var_pb2n_dn4 = assign17640_e12186_d_n4;
        locals.var_pb2n_dn5 = assign17640_e12186_d_n5;
        locals.var_pb2n_dn6 = assign17640_e12186_d_n6;
        locals.var_pb2n_dn7 = assign17640_e12186_d_n7;
        locals.var_pb2n_dn8 = assign17640_e12186_d_n8;
        locals.var_pb2n_dn9 = assign17640_e12186_d_n9;
        locals.var_pb2n_dn10 = assign17640_e12186_d_n10;
        locals.var_pb2n_dn13 = assign17640_e12186_d_n13;

        let (assign17650_e12205, assign17650_e12205_d_n0, assign17650_e12205_d_n2, assign17650_e12205_d_n4, assign17650_e12205_d_n5, assign17650_e12205_d_n6, assign17650_e12205_d_n7, assign17650_e12205_d_n8, assign17650_e12205_d_n9, assign17650_e12205_d_n10, assign17650_e12205_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard355 == 0.0)) && (locals.var_guard358 == 0.0)) {
        let assign17650_e12197: f64 = (locals.var_uc_njunc / locals.var_nin);
        let assign17650_e12199: f64 = (assign17650_e12197 * locals.var_nsub);
        let assign17650_e12201: f64 = (assign17650_e12199 / locals.var_nin);
        let assign17650_e12202: f64 = (assign17650_e12201).ln();
        let assign17650_e12203: f64 = (locals.var_beta_inv * assign17650_e12202);
        (assign17650_e12203, ((locals.var_beta_inv_dn0 * assign17650_e12202) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn0) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17650_e12197 * locals.var_nsub_dn0)) * locals.var_nin) - (assign17650_e12199 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign17650_e12201))), ((locals.var_beta_inv_dn2 * assign17650_e12202) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn2) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17650_e12197 * locals.var_nsub_dn2)) * locals.var_nin) - (assign17650_e12199 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign17650_e12201))), ((locals.var_beta_inv_dn4 * assign17650_e12202) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn4) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17650_e12197 * locals.var_nsub_dn4)) * locals.var_nin) - (assign17650_e12199 * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign17650_e12201))), ((locals.var_beta_inv_dn5 * assign17650_e12202) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn5) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17650_e12197 * locals.var_nsub_dn5)) * locals.var_nin) - (assign17650_e12199 * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign17650_e12201))), ((locals.var_beta_inv_dn6 * assign17650_e12202) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn6) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17650_e12197 * locals.var_nsub_dn6)) * locals.var_nin) - (assign17650_e12199 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign17650_e12201))), ((locals.var_beta_inv_dn7 * assign17650_e12202) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn7) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17650_e12197 * locals.var_nsub_dn7)) * locals.var_nin) - (assign17650_e12199 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign17650_e12201))), ((locals.var_beta_inv_dn8 * assign17650_e12202) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn8) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17650_e12197 * locals.var_nsub_dn8)) * locals.var_nin) - (assign17650_e12199 * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign17650_e12201))), ((locals.var_beta_inv_dn9 * assign17650_e12202) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn9) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17650_e12197 * locals.var_nsub_dn9)) * locals.var_nin) - (assign17650_e12199 * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign17650_e12201))), ((locals.var_beta_inv_dn10 * assign17650_e12202) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn10) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17650_e12197 * locals.var_nsub_dn10)) * locals.var_nin) - (assign17650_e12199 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign17650_e12201))), ((locals.var_beta_inv_dn13 * assign17650_e12202) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn13) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17650_e12197 * locals.var_nsub_dn13)) * locals.var_nin) - (assign17650_e12199 * locals.var_nin_dn13)) / (locals.var_nin * locals.var_nin)) / assign17650_e12201))),)
    } else {
        (locals.var_vbipn, locals.var_vbipn_dn0, locals.var_vbipn_dn2, locals.var_vbipn_dn4, locals.var_vbipn_dn5, locals.var_vbipn_dn6, locals.var_vbipn_dn7, locals.var_vbipn_dn8, locals.var_vbipn_dn9, locals.var_vbipn_dn10, locals.var_vbipn_dn13,)
    }
};
        locals.var_vbipn = assign17650_e12205;
        locals.var_vbipn_dn0 = assign17650_e12205_d_n0;
        locals.var_vbipn_dn2 = assign17650_e12205_d_n2;
        locals.var_vbipn_dn4 = assign17650_e12205_d_n4;
        locals.var_vbipn_dn5 = assign17650_e12205_d_n5;
        locals.var_vbipn_dn6 = assign17650_e12205_d_n6;
        locals.var_vbipn_dn7 = assign17650_e12205_d_n7;
        locals.var_vbipn_dn8 = assign17650_e12205_d_n8;
        locals.var_vbipn_dn9 = assign17650_e12205_d_n9;
        locals.var_vbipn_dn10 = assign17650_e12205_d_n10;
        locals.var_vbipn_dn13 = assign17650_e12205_d_n13;

        let (assign17660_e12215, assign17660_e12215_d_n0, assign17660_e12215_d_n2, assign17660_e12215_d_n4, assign17660_e12215_d_n5, assign17660_e12215_d_n6, assign17660_e12215_d_n7, assign17660_e12215_d_n8, assign17660_e12215_d_n9, assign17660_e12215_d_n10, assign17660_e12215_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard355 == 0.0)) && (locals.var_guard358 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_depmphn0, locals.var_depmphn0_dn0, locals.var_depmphn0_dn2, locals.var_depmphn0_dn4, locals.var_depmphn0_dn5, locals.var_depmphn0_dn6, locals.var_depmphn0_dn7, locals.var_depmphn0_dn8, locals.var_depmphn0_dn9, locals.var_depmphn0_dn10, locals.var_depmphn0_dn13,)
    }
};
        locals.var_depmphn0 = assign17660_e12215;
        locals.var_depmphn0_dn0 = assign17660_e12215_d_n0;
        locals.var_depmphn0_dn2 = assign17660_e12215_d_n2;
        locals.var_depmphn0_dn4 = assign17660_e12215_d_n4;
        locals.var_depmphn0_dn5 = assign17660_e12215_d_n5;
        locals.var_depmphn0_dn6 = assign17660_e12215_d_n6;
        locals.var_depmphn0_dn7 = assign17660_e12215_d_n7;
        locals.var_depmphn0_dn8 = assign17660_e12215_d_n8;
        locals.var_depmphn0_dn9 = assign17660_e12215_d_n9;
        locals.var_depmphn0_dn10 = assign17660_e12215_d_n10;
        locals.var_depmphn0_dn13 = assign17660_e12215_d_n13;

        let (assign17670_e12221, assign17670_e12221_d_n0, assign17670_e12221_d_n2, assign17670_e12221_d_n4, assign17670_e12221_d_n5, assign17670_e12221_d_n6, assign17670_e12221_d_n7, assign17670_e12221_d_n8, assign17670_e12221_d_n9, assign17670_e12221_d_n10, assign17670_e12221_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17670_e12219: f64 = (locals.var_ptovr0 * locals.var_beta_inv);
        (assign17670_e12219, ((locals.var_ptovr0_dn0 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn0)), ((locals.var_ptovr0_dn2 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn2)), ((locals.var_ptovr0_dn4 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn4)), ((locals.var_ptovr0_dn5 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn5)), ((locals.var_ptovr0_dn6 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn6)), ((locals.var_ptovr0_dn7 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn7)), ((locals.var_ptovr0_dn8 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn8)), ((locals.var_ptovr0_dn9 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn9)), ((locals.var_ptovr0_dn10 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn10)), ((locals.var_ptovr0_dn13 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn13)),)
    } else {
        (locals.var_ptovr, locals.var_ptovr_dn0, locals.var_ptovr_dn2, locals.var_ptovr_dn4, locals.var_ptovr_dn5, locals.var_ptovr_dn6, locals.var_ptovr_dn7, locals.var_ptovr_dn8, locals.var_ptovr_dn9, locals.var_ptovr_dn10, locals.var_ptovr_dn13,)
    }
};
        locals.var_ptovr = assign17670_e12221;
        locals.var_ptovr_dn0 = assign17670_e12221_d_n0;
        locals.var_ptovr_dn2 = assign17670_e12221_d_n2;
        locals.var_ptovr_dn4 = assign17670_e12221_d_n4;
        locals.var_ptovr_dn5 = assign17670_e12221_d_n5;
        locals.var_ptovr_dn6 = assign17670_e12221_d_n6;
        locals.var_ptovr_dn7 = assign17670_e12221_d_n7;
        locals.var_ptovr_dn8 = assign17670_e12221_d_n8;
        locals.var_ptovr_dn9 = assign17670_e12221_d_n9;
        locals.var_ptovr_dn10 = assign17670_e12221_d_n10;
        locals.var_ptovr_dn13 = assign17670_e12221_d_n13;

        let (assign17680_e12227, assign17680_e12227_d_n0, assign17680_e12227_d_n2, assign17680_e12227_d_n4, assign17680_e12227_d_n5, assign17680_e12227_d_n6, assign17680_e12227_d_n7, assign17680_e12227_d_n8, assign17680_e12227_d_n9, assign17680_e12227_d_n10, assign17680_e12227_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17680_e12225: f64 = (locals.var_ttemp / locals.var_ktnom);
        (assign17680_e12225, (locals.var_ttemp_dn0 / locals.var_ktnom), (locals.var_ttemp_dn2 / locals.var_ktnom), (locals.var_ttemp_dn4 / locals.var_ktnom), (locals.var_ttemp_dn5 / locals.var_ktnom), (locals.var_ttemp_dn6 / locals.var_ktnom), (locals.var_ttemp_dn7 / locals.var_ktnom), (locals.var_ttemp_dn8 / locals.var_ktnom), (locals.var_ttemp_dn9 / locals.var_ktnom), (locals.var_ttemp_dn10 / locals.var_ktnom), (locals.var_ttemp_dn13 / locals.var_ktnom),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign17680_e12227;
        locals.var_t1_dn0 = assign17680_e12227_d_n0;
        locals.var_t1_dn2 = assign17680_e12227_d_n2;
        locals.var_t1_dn4 = assign17680_e12227_d_n4;
        locals.var_t1_dn5 = assign17680_e12227_d_n5;
        locals.var_t1_dn6 = assign17680_e12227_d_n6;
        locals.var_t1_dn7 = assign17680_e12227_d_n7;
        locals.var_t1_dn8 = assign17680_e12227_d_n8;
        locals.var_t1_dn9 = assign17680_e12227_d_n9;
        locals.var_t1_dn10 = assign17680_e12227_d_n10;
        locals.var_t1_dn13 = assign17680_e12227_d_n13;

        let (assign17690_e12247, assign17690_e12247_d_n0, assign17690_e12247_d_n2, assign17690_e12247_d_n4, assign17690_e12247_d_n5, assign17690_e12247_d_n6, assign17690_e12247_d_n7, assign17690_e12247_d_n8, assign17690_e12247_d_n9, assign17690_e12247_d_n10, assign17690_e12247_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17690_e12232: f64 = (0.4 * locals.var_t1);
        let assign17690_e12233: f64 = (1.8 + assign17690_e12232);
        let assign17690_e12236: f64 = (0.1 * locals.var_t1);
        let assign17690_e12238: f64 = (assign17690_e12236 * locals.var_t1);
        let assign17690_e12239: f64 = (assign17690_e12233 + assign17690_e12238);
        let assign17690_e12243: f64 = (1.0 - locals.var_t1);
        let assign17690_e12244: f64 = (locals.var_uc_vtmp * assign17690_e12243);
        let assign17690_e12245: f64 = (assign17690_e12239 - assign17690_e12244);
        (assign17690_e12245, (((0.4 * locals.var_t1_dn0) + (((0.1 * locals.var_t1_dn0) * locals.var_t1) + (assign17690_e12236 * locals.var_t1_dn0))) - (locals.var_uc_vtmp * (-locals.var_t1_dn0))), (((0.4 * locals.var_t1_dn2) + (((0.1 * locals.var_t1_dn2) * locals.var_t1) + (assign17690_e12236 * locals.var_t1_dn2))) - (locals.var_uc_vtmp * (-locals.var_t1_dn2))), (((0.4 * locals.var_t1_dn4) + (((0.1 * locals.var_t1_dn4) * locals.var_t1) + (assign17690_e12236 * locals.var_t1_dn4))) - (locals.var_uc_vtmp * (-locals.var_t1_dn4))), (((0.4 * locals.var_t1_dn5) + (((0.1 * locals.var_t1_dn5) * locals.var_t1) + (assign17690_e12236 * locals.var_t1_dn5))) - (locals.var_uc_vtmp * (-locals.var_t1_dn5))), (((0.4 * locals.var_t1_dn6) + (((0.1 * locals.var_t1_dn6) * locals.var_t1) + (assign17690_e12236 * locals.var_t1_dn6))) - (locals.var_uc_vtmp * (-locals.var_t1_dn6))), (((0.4 * locals.var_t1_dn7) + (((0.1 * locals.var_t1_dn7) * locals.var_t1) + (assign17690_e12236 * locals.var_t1_dn7))) - (locals.var_uc_vtmp * (-locals.var_t1_dn7))), (((0.4 * locals.var_t1_dn8) + (((0.1 * locals.var_t1_dn8) * locals.var_t1) + (assign17690_e12236 * locals.var_t1_dn8))) - (locals.var_uc_vtmp * (-locals.var_t1_dn8))), (((0.4 * locals.var_t1_dn9) + (((0.1 * locals.var_t1_dn9) * locals.var_t1) + (assign17690_e12236 * locals.var_t1_dn9))) - (locals.var_uc_vtmp * (-locals.var_t1_dn9))), (((0.4 * locals.var_t1_dn10) + (((0.1 * locals.var_t1_dn10) * locals.var_t1) + (assign17690_e12236 * locals.var_t1_dn10))) - (locals.var_uc_vtmp * (-locals.var_t1_dn10))), (((0.4 * locals.var_t1_dn13) + (((0.1 * locals.var_t1_dn13) * locals.var_t1) + (assign17690_e12236 * locals.var_t1_dn13))) - (locals.var_uc_vtmp * (-locals.var_t1_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign17690_e12247;
        locals.var_t0_dn0 = assign17690_e12247_d_n0;
        locals.var_t0_dn2 = assign17690_e12247_d_n2;
        locals.var_t0_dn4 = assign17690_e12247_d_n4;
        locals.var_t0_dn5 = assign17690_e12247_d_n5;
        locals.var_t0_dn6 = assign17690_e12247_d_n6;
        locals.var_t0_dn7 = assign17690_e12247_d_n7;
        locals.var_t0_dn8 = assign17690_e12247_d_n8;
        locals.var_t0_dn9 = assign17690_e12247_d_n9;
        locals.var_t0_dn10 = assign17690_e12247_d_n10;
        locals.var_t0_dn13 = assign17690_e12247_d_n13;

        let assign17700_e12250: f64 = if p.p39 != 2.0 { 1.0 } else { 0.0 };
        locals.var_guard361 = assign17700_e12250;

        let (assign17710_e12270, assign17710_e12270_d_n0, assign17710_e12270_d_n2, assign17710_e12270_d_n4, assign17710_e12270_d_n5, assign17710_e12270_d_n6, assign17710_e12270_d_n7, assign17710_e12270_d_n8, assign17710_e12270_d_n9, assign17710_e12270_d_n10, assign17710_e12270_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard361 != 0.0)) {
        let assign17710_e12256: f64 = (locals.var_vmax0 * locals.var_uc_vmax);
        let assign17710_e12258: f64 = (assign17710_e12256 / locals.var_t0);
        let assign17710_e12262: f64 = (p.p90 * locals.var_tdiff0);
        let assign17710_e12263: f64 = (1.0 + assign17710_e12262);
        let assign17710_e12266: f64 = (p.p91 * locals.var_tdiff0_2);
        let assign17710_e12267: f64 = (assign17710_e12263 + assign17710_e12266);
        let assign17710_e12268: f64 = (assign17710_e12258 * assign17710_e12267);
        (assign17710_e12268, (((-((assign17710_e12256 * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))) * assign17710_e12267) + (assign17710_e12258 * ((p.p90 * locals.var_tdiff0_dn0) + (p.p91 * locals.var_tdiff0_2_dn0)))), (((-((assign17710_e12256 * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))) * assign17710_e12267) + (assign17710_e12258 * ((p.p90 * locals.var_tdiff0_dn2) + (p.p91 * locals.var_tdiff0_2_dn2)))), (((-((assign17710_e12256 * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))) * assign17710_e12267) + (assign17710_e12258 * ((p.p90 * locals.var_tdiff0_dn4) + (p.p91 * locals.var_tdiff0_2_dn4)))), (((-((assign17710_e12256 * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))) * assign17710_e12267) + (assign17710_e12258 * ((p.p90 * locals.var_tdiff0_dn5) + (p.p91 * locals.var_tdiff0_2_dn5)))), (((-((assign17710_e12256 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))) * assign17710_e12267) + (assign17710_e12258 * ((p.p90 * locals.var_tdiff0_dn6) + (p.p91 * locals.var_tdiff0_2_dn6)))), (((-((assign17710_e12256 * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))) * assign17710_e12267) + (assign17710_e12258 * ((p.p90 * locals.var_tdiff0_dn7) + (p.p91 * locals.var_tdiff0_2_dn7)))), (((-((assign17710_e12256 * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))) * assign17710_e12267) + (assign17710_e12258 * ((p.p90 * locals.var_tdiff0_dn8) + (p.p91 * locals.var_tdiff0_2_dn8)))), (((-((assign17710_e12256 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))) * assign17710_e12267) + (assign17710_e12258 * ((p.p90 * locals.var_tdiff0_dn9) + (p.p91 * locals.var_tdiff0_2_dn9)))), (((-((assign17710_e12256 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))) * assign17710_e12267) + (assign17710_e12258 * ((p.p90 * locals.var_tdiff0_dn10) + (p.p91 * locals.var_tdiff0_2_dn10)))), (((-((assign17710_e12256 * locals.var_t0_dn13) / (locals.var_t0 * locals.var_t0))) * assign17710_e12267) + (assign17710_e12258 * ((p.p90 * locals.var_tdiff0_dn13) + (p.p91 * locals.var_tdiff0_2_dn13)))),)
    } else {
        (locals.var_vmaxeff, locals.var_vmaxeff_dn0, locals.var_vmaxeff_dn2, locals.var_vmaxeff_dn4, locals.var_vmaxeff_dn5, locals.var_vmaxeff_dn6, locals.var_vmaxeff_dn7, locals.var_vmaxeff_dn8, locals.var_vmaxeff_dn9, locals.var_vmaxeff_dn10, locals.var_vmaxeff_dn13,)
    }
};
        locals.var_vmaxeff = assign17710_e12270;
        locals.var_vmaxeff_dn0 = assign17710_e12270_d_n0;
        locals.var_vmaxeff_dn2 = assign17710_e12270_d_n2;
        locals.var_vmaxeff_dn4 = assign17710_e12270_d_n4;
        locals.var_vmaxeff_dn5 = assign17710_e12270_d_n5;
        locals.var_vmaxeff_dn6 = assign17710_e12270_d_n6;
        locals.var_vmaxeff_dn7 = assign17710_e12270_d_n7;
        locals.var_vmaxeff_dn8 = assign17710_e12270_d_n8;
        locals.var_vmaxeff_dn9 = assign17710_e12270_d_n9;
        locals.var_vmaxeff_dn10 = assign17710_e12270_d_n10;
        locals.var_vmaxeff_dn13 = assign17710_e12270_d_n13;

        let (assign17720_e12291, assign17720_e12291_d_n0, assign17720_e12291_d_n2, assign17720_e12291_d_n4, assign17720_e12291_d_n5, assign17720_e12291_d_n6, assign17720_e12291_d_n7, assign17720_e12291_d_n8, assign17720_e12291_d_n9, assign17720_e12291_d_n10, assign17720_e12291_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard361 == 0.0)) {
        let assign17720_e12277: f64 = (locals.var_vmax0 * locals.var_uc_vmax);
        let assign17720_e12279: f64 = (assign17720_e12277 / locals.var_t0);
        let assign17720_e12283: f64 = (p.p90 * locals.var_tdiff);
        let assign17720_e12284: f64 = (1.0 + assign17720_e12283);
        let assign17720_e12287: f64 = (p.p91 * locals.var_tdiff_2);
        let assign17720_e12288: f64 = (assign17720_e12284 + assign17720_e12287);
        let assign17720_e12289: f64 = (assign17720_e12279 * assign17720_e12288);
        (assign17720_e12289, (((-((assign17720_e12277 * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))) * assign17720_e12288) + (assign17720_e12279 * ((p.p90 * locals.var_tdiff_dn0) + (p.p91 * locals.var_tdiff_2_dn0)))), (((-((assign17720_e12277 * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))) * assign17720_e12288) + (assign17720_e12279 * ((p.p90 * locals.var_tdiff_dn2) + (p.p91 * locals.var_tdiff_2_dn2)))), (((-((assign17720_e12277 * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))) * assign17720_e12288) + (assign17720_e12279 * ((p.p90 * locals.var_tdiff_dn4) + (p.p91 * locals.var_tdiff_2_dn4)))), (((-((assign17720_e12277 * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))) * assign17720_e12288) + (assign17720_e12279 * ((p.p90 * locals.var_tdiff_dn5) + (p.p91 * locals.var_tdiff_2_dn5)))), (((-((assign17720_e12277 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))) * assign17720_e12288) + (assign17720_e12279 * ((p.p90 * locals.var_tdiff_dn6) + (p.p91 * locals.var_tdiff_2_dn6)))), (((-((assign17720_e12277 * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))) * assign17720_e12288) + (assign17720_e12279 * ((p.p90 * locals.var_tdiff_dn7) + (p.p91 * locals.var_tdiff_2_dn7)))), (((-((assign17720_e12277 * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))) * assign17720_e12288) + (assign17720_e12279 * ((p.p90 * locals.var_tdiff_dn8) + (p.p91 * locals.var_tdiff_2_dn8)))), (((-((assign17720_e12277 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))) * assign17720_e12288) + (assign17720_e12279 * ((p.p90 * locals.var_tdiff_dn9) + (p.p91 * locals.var_tdiff_2_dn9)))), (((-((assign17720_e12277 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))) * assign17720_e12288) + (assign17720_e12279 * ((p.p90 * locals.var_tdiff_dn10) + (p.p91 * locals.var_tdiff_2_dn10)))), (((-((assign17720_e12277 * locals.var_t0_dn13) / (locals.var_t0 * locals.var_t0))) * assign17720_e12288) + (assign17720_e12279 * ((p.p90 * locals.var_tdiff_dn13) + (p.p91 * locals.var_tdiff_2_dn13)))),)
    } else {
        (locals.var_vmaxeff, locals.var_vmaxeff_dn0, locals.var_vmaxeff_dn2, locals.var_vmaxeff_dn4, locals.var_vmaxeff_dn5, locals.var_vmaxeff_dn6, locals.var_vmaxeff_dn7, locals.var_vmaxeff_dn8, locals.var_vmaxeff_dn9, locals.var_vmaxeff_dn10, locals.var_vmaxeff_dn13,)
    }
};
        locals.var_vmaxeff = assign17720_e12291;
        locals.var_vmaxeff_dn0 = assign17720_e12291_d_n0;
        locals.var_vmaxeff_dn2 = assign17720_e12291_d_n2;
        locals.var_vmaxeff_dn4 = assign17720_e12291_d_n4;
        locals.var_vmaxeff_dn5 = assign17720_e12291_d_n5;
        locals.var_vmaxeff_dn6 = assign17720_e12291_d_n6;
        locals.var_vmaxeff_dn7 = assign17720_e12291_d_n7;
        locals.var_vmaxeff_dn8 = assign17720_e12291_d_n8;
        locals.var_vmaxeff_dn9 = assign17720_e12291_d_n9;
        locals.var_vmaxeff_dn10 = assign17720_e12291_d_n10;
        locals.var_vmaxeff_dn13 = assign17720_e12291_d_n13;

        let assign17740_e12299: f64 = if p.p39 != 2.0 { 1.0 } else { 0.0 };
        locals.var_guard363 = assign17740_e12299;

        let (assign17750_e12315, assign17750_e12315_d_n0, assign17750_e12315_d_n2, assign17750_e12315_d_n4, assign17750_e12315_d_n5, assign17750_e12315_d_n6, assign17750_e12315_d_n7, assign17750_e12315_d_n8, assign17750_e12315_d_n9, assign17750_e12315_d_n10, assign17750_e12315_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard363 != 0.0)) {
        let assign17750_e12307: f64 = (p.p324 * locals.var_tdiff0);
        let assign17750_e12308: f64 = (1.0 + assign17750_e12307);
        let assign17750_e12311: f64 = (p.p325 * locals.var_tdiff0_2);
        let assign17750_e12312: f64 = (assign17750_e12308 + assign17750_e12311);
        let assign17750_e12313: f64 = (locals.var_ninvd0 * assign17750_e12312);
        (assign17750_e12313, (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn0) + (p.p325 * locals.var_tdiff0_2_dn0))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn2) + (p.p325 * locals.var_tdiff0_2_dn2))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn4) + (p.p325 * locals.var_tdiff0_2_dn4))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn5) + (p.p325 * locals.var_tdiff0_2_dn5))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn6) + (p.p325 * locals.var_tdiff0_2_dn6))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn7) + (p.p325 * locals.var_tdiff0_2_dn7))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn8) + (p.p325 * locals.var_tdiff0_2_dn8))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn9) + (p.p325 * locals.var_tdiff0_2_dn9))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn10) + (p.p325 * locals.var_tdiff0_2_dn10))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn13) + (p.p325 * locals.var_tdiff0_2_dn13))),)
    } else {
        (locals.var_ninvde, locals.var_ninvde_dn0, locals.var_ninvde_dn2, locals.var_ninvde_dn4, locals.var_ninvde_dn5, locals.var_ninvde_dn6, locals.var_ninvde_dn7, locals.var_ninvde_dn8, locals.var_ninvde_dn9, locals.var_ninvde_dn10, locals.var_ninvde_dn13,)
    }
};
        locals.var_ninvde = assign17750_e12315;
        locals.var_ninvde_dn0 = assign17750_e12315_d_n0;
        locals.var_ninvde_dn2 = assign17750_e12315_d_n2;
        locals.var_ninvde_dn4 = assign17750_e12315_d_n4;
        locals.var_ninvde_dn5 = assign17750_e12315_d_n5;
        locals.var_ninvde_dn6 = assign17750_e12315_d_n6;
        locals.var_ninvde_dn7 = assign17750_e12315_d_n7;
        locals.var_ninvde_dn8 = assign17750_e12315_d_n8;
        locals.var_ninvde_dn9 = assign17750_e12315_d_n9;
        locals.var_ninvde_dn10 = assign17750_e12315_d_n10;
        locals.var_ninvde_dn13 = assign17750_e12315_d_n13;

        let (assign17760_e12329, assign17760_e12329_d_n0, assign17760_e12329_d_n2, assign17760_e12329_d_n4, assign17760_e12329_d_n5, assign17760_e12329_d_n6, assign17760_e12329_d_n7, assign17760_e12329_d_n8, assign17760_e12329_d_n9, assign17760_e12329_d_n10, assign17760_e12329_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard363 != 0.0)) {
        let assign17760_e12322: f64 = (p.p390 * locals.var_tdiff0);
        let assign17760_e12323: f64 = (1.0 + assign17760_e12322);
        let assign17760_e12326: f64 = (p.p391 * locals.var_tdiff0_2);
        let assign17760_e12327: f64 = (assign17760_e12323 + assign17760_e12326);
        (assign17760_e12327, ((p.p390 * locals.var_tdiff0_dn0) + (p.p391 * locals.var_tdiff0_2_dn0)), ((p.p390 * locals.var_tdiff0_dn2) + (p.p391 * locals.var_tdiff0_2_dn2)), ((p.p390 * locals.var_tdiff0_dn4) + (p.p391 * locals.var_tdiff0_2_dn4)), ((p.p390 * locals.var_tdiff0_dn5) + (p.p391 * locals.var_tdiff0_2_dn5)), ((p.p390 * locals.var_tdiff0_dn6) + (p.p391 * locals.var_tdiff0_2_dn6)), ((p.p390 * locals.var_tdiff0_dn7) + (p.p391 * locals.var_tdiff0_2_dn7)), ((p.p390 * locals.var_tdiff0_dn8) + (p.p391 * locals.var_tdiff0_2_dn8)), ((p.p390 * locals.var_tdiff0_dn9) + (p.p391 * locals.var_tdiff0_2_dn9)), ((p.p390 * locals.var_tdiff0_dn10) + (p.p391 * locals.var_tdiff0_2_dn10)), ((p.p390 * locals.var_tdiff0_dn13) + (p.p391 * locals.var_tdiff0_2_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign17760_e12329;
        locals.var_t1_dn0 = assign17760_e12329_d_n0;
        locals.var_t1_dn2 = assign17760_e12329_d_n2;
        locals.var_t1_dn4 = assign17760_e12329_d_n4;
        locals.var_t1_dn5 = assign17760_e12329_d_n5;
        locals.var_t1_dn6 = assign17760_e12329_d_n6;
        locals.var_t1_dn7 = assign17760_e12329_d_n7;
        locals.var_t1_dn8 = assign17760_e12329_d_n8;
        locals.var_t1_dn9 = assign17760_e12329_d_n9;
        locals.var_t1_dn10 = assign17760_e12329_d_n10;
        locals.var_t1_dn13 = assign17760_e12329_d_n13;

        let (assign17770_e12337, assign17770_e12337_d_n0, assign17770_e12337_d_n2, assign17770_e12337_d_n4, assign17770_e12337_d_n5, assign17770_e12337_d_n6, assign17770_e12337_d_n7, assign17770_e12337_d_n8, assign17770_e12337_d_n9, assign17770_e12337_d_n10, assign17770_e12337_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard363 != 0.0)) {
        let assign17770_e12335: f64 = (locals.var_ninvd0cres * locals.var_t1);
        (assign17770_e12335, ((locals.var_ninvd0cres_dn0 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn0)), ((locals.var_ninvd0cres_dn2 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn2)), ((locals.var_ninvd0cres_dn4 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn4)), ((locals.var_ninvd0cres_dn5 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn5)), ((locals.var_ninvd0cres_dn6 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn6)), ((locals.var_ninvd0cres_dn7 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn7)), ((locals.var_ninvd0cres_dn8 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn8)), ((locals.var_ninvd0cres_dn9 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn9)), ((locals.var_ninvd0cres_dn10 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn10)), ((locals.var_ninvd0cres_dn13 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn13)),)
    } else {
        (locals.var_ninvdecres, locals.var_ninvdecres_dn0, locals.var_ninvdecres_dn2, locals.var_ninvdecres_dn4, locals.var_ninvdecres_dn5, locals.var_ninvdecres_dn6, locals.var_ninvdecres_dn7, locals.var_ninvdecres_dn8, locals.var_ninvdecres_dn9, locals.var_ninvdecres_dn10, locals.var_ninvdecres_dn13,)
    }
};
        locals.var_ninvdecres = assign17770_e12337;
        locals.var_ninvdecres_dn0 = assign17770_e12337_d_n0;
        locals.var_ninvdecres_dn2 = assign17770_e12337_d_n2;
        locals.var_ninvdecres_dn4 = assign17770_e12337_d_n4;
        locals.var_ninvdecres_dn5 = assign17770_e12337_d_n5;
        locals.var_ninvdecres_dn6 = assign17770_e12337_d_n6;
        locals.var_ninvdecres_dn7 = assign17770_e12337_d_n7;
        locals.var_ninvdecres_dn8 = assign17770_e12337_d_n8;
        locals.var_ninvdecres_dn9 = assign17770_e12337_d_n9;
        locals.var_ninvdecres_dn10 = assign17770_e12337_d_n10;
        locals.var_ninvdecres_dn13 = assign17770_e12337_d_n13;

        let (assign17780_e12345, assign17780_e12345_d_n0, assign17780_e12345_d_n2, assign17780_e12345_d_n4, assign17780_e12345_d_n5, assign17780_e12345_d_n6, assign17780_e12345_d_n7, assign17780_e12345_d_n8, assign17780_e12345_d_n9, assign17780_e12345_d_n10, assign17780_e12345_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard363 != 0.0)) {
        let assign17780_e12343: f64 = (locals.var_ninvd0hres * locals.var_t1);
        (assign17780_e12343, ((locals.var_ninvd0hres_dn0 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn0)), ((locals.var_ninvd0hres_dn2 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn2)), ((locals.var_ninvd0hres_dn4 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn4)), ((locals.var_ninvd0hres_dn5 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn5)), ((locals.var_ninvd0hres_dn6 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn6)), ((locals.var_ninvd0hres_dn7 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn7)), ((locals.var_ninvd0hres_dn8 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn8)), ((locals.var_ninvd0hres_dn9 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn9)), ((locals.var_ninvd0hres_dn10 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn10)), ((locals.var_ninvd0hres_dn13 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn13)),)
    } else {
        (locals.var_ninvdehres, locals.var_ninvdehres_dn0, locals.var_ninvdehres_dn2, locals.var_ninvdehres_dn4, locals.var_ninvdehres_dn5, locals.var_ninvdehres_dn6, locals.var_ninvdehres_dn7, locals.var_ninvdehres_dn8, locals.var_ninvdehres_dn9, locals.var_ninvdehres_dn10, locals.var_ninvdehres_dn13,)
    }
};
        locals.var_ninvdehres = assign17780_e12345;
        locals.var_ninvdehres_dn0 = assign17780_e12345_d_n0;
        locals.var_ninvdehres_dn2 = assign17780_e12345_d_n2;
        locals.var_ninvdehres_dn4 = assign17780_e12345_d_n4;
        locals.var_ninvdehres_dn5 = assign17780_e12345_d_n5;
        locals.var_ninvdehres_dn6 = assign17780_e12345_d_n6;
        locals.var_ninvdehres_dn7 = assign17780_e12345_d_n7;
        locals.var_ninvdehres_dn8 = assign17780_e12345_d_n8;
        locals.var_ninvdehres_dn9 = assign17780_e12345_d_n9;
        locals.var_ninvdehres_dn10 = assign17780_e12345_d_n10;
        locals.var_ninvdehres_dn13 = assign17780_e12345_d_n13;

        let (assign17790_e12362, assign17790_e12362_d_n0, assign17790_e12362_d_n2, assign17790_e12362_d_n4, assign17790_e12362_d_n5, assign17790_e12362_d_n6, assign17790_e12362_d_n7, assign17790_e12362_d_n8, assign17790_e12362_d_n9, assign17790_e12362_d_n10, assign17790_e12362_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard363 == 0.0)) {
        let assign17790_e12354: f64 = (p.p324 * locals.var_tdiff);
        let assign17790_e12355: f64 = (1.0 + assign17790_e12354);
        let assign17790_e12358: f64 = (p.p325 * locals.var_tdiff_2);
        let assign17790_e12359: f64 = (assign17790_e12355 + assign17790_e12358);
        let assign17790_e12360: f64 = (locals.var_ninvd0 * assign17790_e12359);
        (assign17790_e12360, (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn0) + (p.p325 * locals.var_tdiff_2_dn0))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn2) + (p.p325 * locals.var_tdiff_2_dn2))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn4) + (p.p325 * locals.var_tdiff_2_dn4))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn5) + (p.p325 * locals.var_tdiff_2_dn5))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn6) + (p.p325 * locals.var_tdiff_2_dn6))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn7) + (p.p325 * locals.var_tdiff_2_dn7))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn8) + (p.p325 * locals.var_tdiff_2_dn8))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn9) + (p.p325 * locals.var_tdiff_2_dn9))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn10) + (p.p325 * locals.var_tdiff_2_dn10))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn13) + (p.p325 * locals.var_tdiff_2_dn13))),)
    } else {
        (locals.var_ninvde, locals.var_ninvde_dn0, locals.var_ninvde_dn2, locals.var_ninvde_dn4, locals.var_ninvde_dn5, locals.var_ninvde_dn6, locals.var_ninvde_dn7, locals.var_ninvde_dn8, locals.var_ninvde_dn9, locals.var_ninvde_dn10, locals.var_ninvde_dn13,)
    }
};
        locals.var_ninvde = assign17790_e12362;
        locals.var_ninvde_dn0 = assign17790_e12362_d_n0;
        locals.var_ninvde_dn2 = assign17790_e12362_d_n2;
        locals.var_ninvde_dn4 = assign17790_e12362_d_n4;
        locals.var_ninvde_dn5 = assign17790_e12362_d_n5;
        locals.var_ninvde_dn6 = assign17790_e12362_d_n6;
        locals.var_ninvde_dn7 = assign17790_e12362_d_n7;
        locals.var_ninvde_dn8 = assign17790_e12362_d_n8;
        locals.var_ninvde_dn9 = assign17790_e12362_d_n9;
        locals.var_ninvde_dn10 = assign17790_e12362_d_n10;
        locals.var_ninvde_dn13 = assign17790_e12362_d_n13;

        let (assign17800_e12377, assign17800_e12377_d_n0, assign17800_e12377_d_n2, assign17800_e12377_d_n4, assign17800_e12377_d_n5, assign17800_e12377_d_n6, assign17800_e12377_d_n7, assign17800_e12377_d_n8, assign17800_e12377_d_n9, assign17800_e12377_d_n10, assign17800_e12377_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard363 == 0.0)) {
        let assign17800_e12370: f64 = (p.p390 * locals.var_tdiff);
        let assign17800_e12371: f64 = (1.0 + assign17800_e12370);
        let assign17800_e12374: f64 = (p.p391 * locals.var_tdiff_2);
        let assign17800_e12375: f64 = (assign17800_e12371 + assign17800_e12374);
        (assign17800_e12375, ((p.p390 * locals.var_tdiff_dn0) + (p.p391 * locals.var_tdiff_2_dn0)), ((p.p390 * locals.var_tdiff_dn2) + (p.p391 * locals.var_tdiff_2_dn2)), ((p.p390 * locals.var_tdiff_dn4) + (p.p391 * locals.var_tdiff_2_dn4)), ((p.p390 * locals.var_tdiff_dn5) + (p.p391 * locals.var_tdiff_2_dn5)), ((p.p390 * locals.var_tdiff_dn6) + (p.p391 * locals.var_tdiff_2_dn6)), ((p.p390 * locals.var_tdiff_dn7) + (p.p391 * locals.var_tdiff_2_dn7)), ((p.p390 * locals.var_tdiff_dn8) + (p.p391 * locals.var_tdiff_2_dn8)), ((p.p390 * locals.var_tdiff_dn9) + (p.p391 * locals.var_tdiff_2_dn9)), ((p.p390 * locals.var_tdiff_dn10) + (p.p391 * locals.var_tdiff_2_dn10)), ((p.p390 * locals.var_tdiff_dn13) + (p.p391 * locals.var_tdiff_2_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign17800_e12377;
        locals.var_t1_dn0 = assign17800_e12377_d_n0;
        locals.var_t1_dn2 = assign17800_e12377_d_n2;
        locals.var_t1_dn4 = assign17800_e12377_d_n4;
        locals.var_t1_dn5 = assign17800_e12377_d_n5;
        locals.var_t1_dn6 = assign17800_e12377_d_n6;
        locals.var_t1_dn7 = assign17800_e12377_d_n7;
        locals.var_t1_dn8 = assign17800_e12377_d_n8;
        locals.var_t1_dn9 = assign17800_e12377_d_n9;
        locals.var_t1_dn10 = assign17800_e12377_d_n10;
        locals.var_t1_dn13 = assign17800_e12377_d_n13;

        let (assign17810_e12386, assign17810_e12386_d_n0, assign17810_e12386_d_n2, assign17810_e12386_d_n4, assign17810_e12386_d_n5, assign17810_e12386_d_n6, assign17810_e12386_d_n7, assign17810_e12386_d_n8, assign17810_e12386_d_n9, assign17810_e12386_d_n10, assign17810_e12386_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard363 == 0.0)) {
        let assign17810_e12384: f64 = (locals.var_ninvd0cres * locals.var_t1);
        (assign17810_e12384, ((locals.var_ninvd0cres_dn0 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn0)), ((locals.var_ninvd0cres_dn2 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn2)), ((locals.var_ninvd0cres_dn4 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn4)), ((locals.var_ninvd0cres_dn5 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn5)), ((locals.var_ninvd0cres_dn6 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn6)), ((locals.var_ninvd0cres_dn7 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn7)), ((locals.var_ninvd0cres_dn8 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn8)), ((locals.var_ninvd0cres_dn9 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn9)), ((locals.var_ninvd0cres_dn10 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn10)), ((locals.var_ninvd0cres_dn13 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn13)),)
    } else {
        (locals.var_ninvdecres, locals.var_ninvdecres_dn0, locals.var_ninvdecres_dn2, locals.var_ninvdecres_dn4, locals.var_ninvdecres_dn5, locals.var_ninvdecres_dn6, locals.var_ninvdecres_dn7, locals.var_ninvdecres_dn8, locals.var_ninvdecres_dn9, locals.var_ninvdecres_dn10, locals.var_ninvdecres_dn13,)
    }
};
        locals.var_ninvdecres = assign17810_e12386;
        locals.var_ninvdecres_dn0 = assign17810_e12386_d_n0;
        locals.var_ninvdecres_dn2 = assign17810_e12386_d_n2;
        locals.var_ninvdecres_dn4 = assign17810_e12386_d_n4;
        locals.var_ninvdecres_dn5 = assign17810_e12386_d_n5;
        locals.var_ninvdecres_dn6 = assign17810_e12386_d_n6;
        locals.var_ninvdecres_dn7 = assign17810_e12386_d_n7;
        locals.var_ninvdecres_dn8 = assign17810_e12386_d_n8;
        locals.var_ninvdecres_dn9 = assign17810_e12386_d_n9;
        locals.var_ninvdecres_dn10 = assign17810_e12386_d_n10;
        locals.var_ninvdecres_dn13 = assign17810_e12386_d_n13;

        let (assign17820_e12395, assign17820_e12395_d_n0, assign17820_e12395_d_n2, assign17820_e12395_d_n4, assign17820_e12395_d_n5, assign17820_e12395_d_n6, assign17820_e12395_d_n7, assign17820_e12395_d_n8, assign17820_e12395_d_n9, assign17820_e12395_d_n10, assign17820_e12395_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard363 == 0.0)) {
        let assign17820_e12393: f64 = (locals.var_ninvd0hres * locals.var_t1);
        (assign17820_e12393, ((locals.var_ninvd0hres_dn0 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn0)), ((locals.var_ninvd0hres_dn2 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn2)), ((locals.var_ninvd0hres_dn4 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn4)), ((locals.var_ninvd0hres_dn5 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn5)), ((locals.var_ninvd0hres_dn6 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn6)), ((locals.var_ninvd0hres_dn7 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn7)), ((locals.var_ninvd0hres_dn8 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn8)), ((locals.var_ninvd0hres_dn9 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn9)), ((locals.var_ninvd0hres_dn10 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn10)), ((locals.var_ninvd0hres_dn13 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn13)),)
    } else {
        (locals.var_ninvdehres, locals.var_ninvdehres_dn0, locals.var_ninvdehres_dn2, locals.var_ninvdehres_dn4, locals.var_ninvdehres_dn5, locals.var_ninvdehres_dn6, locals.var_ninvdehres_dn7, locals.var_ninvdehres_dn8, locals.var_ninvdehres_dn9, locals.var_ninvdehres_dn10, locals.var_ninvdehres_dn13,)
    }
};
        locals.var_ninvdehres = assign17820_e12395;
        locals.var_ninvdehres_dn0 = assign17820_e12395_d_n0;
        locals.var_ninvdehres_dn2 = assign17820_e12395_d_n2;
        locals.var_ninvdehres_dn4 = assign17820_e12395_d_n4;
        locals.var_ninvdehres_dn5 = assign17820_e12395_d_n5;
        locals.var_ninvdehres_dn6 = assign17820_e12395_d_n6;
        locals.var_ninvdehres_dn7 = assign17820_e12395_d_n7;
        locals.var_ninvdehres_dn8 = assign17820_e12395_d_n8;
        locals.var_ninvdehres_dn9 = assign17820_e12395_d_n9;
        locals.var_ninvdehres_dn10 = assign17820_e12395_d_n10;
        locals.var_ninvdehres_dn13 = assign17820_e12395_d_n13;

        let assign17840_e12403: f64 = if locals.var_ninvde < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard365 = assign17840_e12403;

        let (assign17850_e12409, assign17850_e12409_d_n0, assign17850_e12409_d_n2, assign17850_e12409_d_n4, assign17850_e12409_d_n5, assign17850_e12409_d_n6, assign17850_e12409_d_n7, assign17850_e12409_d_n8, assign17850_e12409_d_n9, assign17850_e12409_d_n10, assign17850_e12409_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard365 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ninvde, locals.var_ninvde_dn0, locals.var_ninvde_dn2, locals.var_ninvde_dn4, locals.var_ninvde_dn5, locals.var_ninvde_dn6, locals.var_ninvde_dn7, locals.var_ninvde_dn8, locals.var_ninvde_dn9, locals.var_ninvde_dn10, locals.var_ninvde_dn13,)
    }
};
        locals.var_ninvde = assign17850_e12409;
        locals.var_ninvde_dn0 = assign17850_e12409_d_n0;
        locals.var_ninvde_dn2 = assign17850_e12409_d_n2;
        locals.var_ninvde_dn4 = assign17850_e12409_d_n4;
        locals.var_ninvde_dn5 = assign17850_e12409_d_n5;
        locals.var_ninvde_dn6 = assign17850_e12409_d_n6;
        locals.var_ninvde_dn7 = assign17850_e12409_d_n7;
        locals.var_ninvde_dn8 = assign17850_e12409_d_n8;
        locals.var_ninvde_dn9 = assign17850_e12409_d_n9;
        locals.var_ninvde_dn10 = assign17850_e12409_d_n10;
        locals.var_ninvde_dn13 = assign17850_e12409_d_n13;

        let assign17870_e12417: f64 = if locals.var_ninvdecres < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard367 = assign17870_e12417;

        let (assign17880_e12423, assign17880_e12423_d_n0, assign17880_e12423_d_n2, assign17880_e12423_d_n4, assign17880_e12423_d_n5, assign17880_e12423_d_n6, assign17880_e12423_d_n7, assign17880_e12423_d_n8, assign17880_e12423_d_n9, assign17880_e12423_d_n10, assign17880_e12423_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard367 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ninvdecres, locals.var_ninvdecres_dn0, locals.var_ninvdecres_dn2, locals.var_ninvdecres_dn4, locals.var_ninvdecres_dn5, locals.var_ninvdecres_dn6, locals.var_ninvdecres_dn7, locals.var_ninvdecres_dn8, locals.var_ninvdecres_dn9, locals.var_ninvdecres_dn10, locals.var_ninvdecres_dn13,)
    }
};
        locals.var_ninvdecres = assign17880_e12423;
        locals.var_ninvdecres_dn0 = assign17880_e12423_d_n0;
        locals.var_ninvdecres_dn2 = assign17880_e12423_d_n2;
        locals.var_ninvdecres_dn4 = assign17880_e12423_d_n4;
        locals.var_ninvdecres_dn5 = assign17880_e12423_d_n5;
        locals.var_ninvdecres_dn6 = assign17880_e12423_d_n6;
        locals.var_ninvdecres_dn7 = assign17880_e12423_d_n7;
        locals.var_ninvdecres_dn8 = assign17880_e12423_d_n8;
        locals.var_ninvdecres_dn9 = assign17880_e12423_d_n9;
        locals.var_ninvdecres_dn10 = assign17880_e12423_d_n10;
        locals.var_ninvdecres_dn13 = assign17880_e12423_d_n13;

        let assign17900_e12431: f64 = if locals.var_ninvdehres < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard369 = assign17900_e12431;

        let (assign17910_e12437, assign17910_e12437_d_n0, assign17910_e12437_d_n2, assign17910_e12437_d_n4, assign17910_e12437_d_n5, assign17910_e12437_d_n6, assign17910_e12437_d_n7, assign17910_e12437_d_n8, assign17910_e12437_d_n9, assign17910_e12437_d_n10, assign17910_e12437_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard369 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ninvdehres, locals.var_ninvdehres_dn0, locals.var_ninvdehres_dn2, locals.var_ninvdehres_dn4, locals.var_ninvdehres_dn5, locals.var_ninvdehres_dn6, locals.var_ninvdehres_dn7, locals.var_ninvdehres_dn8, locals.var_ninvdehres_dn9, locals.var_ninvdehres_dn10, locals.var_ninvdehres_dn13,)
    }
};
        locals.var_ninvdehres = assign17910_e12437;
        locals.var_ninvdehres_dn0 = assign17910_e12437_d_n0;
        locals.var_ninvdehres_dn2 = assign17910_e12437_d_n2;
        locals.var_ninvdehres_dn4 = assign17910_e12437_d_n4;
        locals.var_ninvdehres_dn5 = assign17910_e12437_d_n5;
        locals.var_ninvdehres_dn6 = assign17910_e12437_d_n6;
        locals.var_ninvdehres_dn7 = assign17910_e12437_d_n7;
        locals.var_ninvdehres_dn8 = assign17910_e12437_d_n8;
        locals.var_ninvdehres_dn9 = assign17910_e12437_d_n9;
        locals.var_ninvdehres_dn10 = assign17910_e12437_d_n10;
        locals.var_ninvdehres_dn13 = assign17910_e12437_d_n13;

    }

    pub(super) fn stamp_transient_block_38(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign17920_e12453, assign17920_e12453_d_n0, assign17920_e12453_d_n2, assign17920_e12453_d_n4, assign17920_e12453_d_n5, assign17920_e12453_d_n6, assign17920_e12453_d_n7, assign17920_e12453_d_n8, assign17920_e12453_d_n9, assign17920_e12453_d_n10, assign17920_e12453_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (p.p53 != 0.0)) {
        let assign17920_e12444: f64 = (p.p328 * locals.var_tdiff0);
        let assign17920_e12445: f64 = (locals.var_uc_rth0 + assign17920_e12444);
        let assign17920_e12448: f64 = (p.p329 * locals.var_tdiff0_2);
        let assign17920_e12449: f64 = (assign17920_e12445 + assign17920_e12448);
        let assign17920_e12451: f64 = (assign17920_e12449 * locals.var_rthtemp0);
        (assign17920_e12451, (((p.p328 * locals.var_tdiff0_dn0) + (p.p329 * locals.var_tdiff0_2_dn0)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn2) + (p.p329 * locals.var_tdiff0_2_dn2)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn4) + (p.p329 * locals.var_tdiff0_2_dn4)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn5) + (p.p329 * locals.var_tdiff0_2_dn5)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn6) + (p.p329 * locals.var_tdiff0_2_dn6)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn7) + (p.p329 * locals.var_tdiff0_2_dn7)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn8) + (p.p329 * locals.var_tdiff0_2_dn8)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn9) + (p.p329 * locals.var_tdiff0_2_dn9)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn10) + (p.p329 * locals.var_tdiff0_2_dn10)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn13) + (p.p329 * locals.var_tdiff0_2_dn13)) * locals.var_rthtemp0),)
    } else {
        (locals.var_rth, locals.var_rth_dn0, locals.var_rth_dn2, locals.var_rth_dn4, locals.var_rth_dn5, locals.var_rth_dn6, locals.var_rth_dn7, locals.var_rth_dn8, locals.var_rth_dn9, locals.var_rth_dn10, locals.var_rth_dn13,)
    }
};
        locals.var_rth = assign17920_e12453;
        locals.var_rth_dn0 = assign17920_e12453_d_n0;
        locals.var_rth_dn2 = assign17920_e12453_d_n2;
        locals.var_rth_dn4 = assign17920_e12453_d_n4;
        locals.var_rth_dn5 = assign17920_e12453_d_n5;
        locals.var_rth_dn6 = assign17920_e12453_d_n6;
        locals.var_rth_dn7 = assign17920_e12453_d_n7;
        locals.var_rth_dn8 = assign17920_e12453_d_n8;
        locals.var_rth_dn9 = assign17920_e12453_d_n9;
        locals.var_rth_dn10 = assign17920_e12453_d_n10;
        locals.var_rth_dn13 = assign17920_e12453_d_n13;

        let assign17940_e12461: f64 = if locals.var_rth < 0.0001 { 1.0 } else { 0.0 };
        locals.var_guard371 = assign17940_e12461;

        let (assign17950_e12469, assign17950_e12469_d_n0, assign17950_e12469_d_n2, assign17950_e12469_d_n4, assign17950_e12469_d_n5, assign17950_e12469_d_n6, assign17950_e12469_d_n7, assign17950_e12469_d_n8, assign17950_e12469_d_n9, assign17950_e12469_d_n10, assign17950_e12469_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (p.p53 != 0.0)) && (locals.var_guard371 != 0.0)) {
        (0.0001, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rth, locals.var_rth_dn0, locals.var_rth_dn2, locals.var_rth_dn4, locals.var_rth_dn5, locals.var_rth_dn6, locals.var_rth_dn7, locals.var_rth_dn8, locals.var_rth_dn9, locals.var_rth_dn10, locals.var_rth_dn13,)
    }
};
        locals.var_rth = assign17950_e12469;
        locals.var_rth_dn0 = assign17950_e12469_d_n0;
        locals.var_rth_dn2 = assign17950_e12469_d_n2;
        locals.var_rth_dn4 = assign17950_e12469_d_n4;
        locals.var_rth_dn5 = assign17950_e12469_d_n5;
        locals.var_rth_dn6 = assign17950_e12469_d_n6;
        locals.var_rth_dn7 = assign17950_e12469_d_n7;
        locals.var_rth_dn8 = assign17950_e12469_d_n8;
        locals.var_rth_dn9 = assign17950_e12469_d_n9;
        locals.var_rth_dn10 = assign17950_e12469_d_n10;
        locals.var_rth_dn13 = assign17950_e12469_d_n13;

        let (assign17960_e12481, assign17960_e12481_d_n0, assign17960_e12481_d_n2, assign17960_e12481_d_n4, assign17960_e12481_d_n5, assign17960_e12481_d_n6, assign17960_e12481_d_n7, assign17960_e12481_d_n8, assign17960_e12481_d_n9, assign17960_e12481_d_n10, assign17960_e12481_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17960_e12474: f64 = (p.p330 * locals.var_tdiff0);
        let assign17960_e12475: f64 = (locals.var_uc_powrat + assign17960_e12474);
        let assign17960_e12478: f64 = (p.p331 * locals.var_tdiff0_2);
        let assign17960_e12479: f64 = (assign17960_e12475 + assign17960_e12478);
        (assign17960_e12479, ((p.p330 * locals.var_tdiff0_dn0) + (p.p331 * locals.var_tdiff0_2_dn0)), ((p.p330 * locals.var_tdiff0_dn2) + (p.p331 * locals.var_tdiff0_2_dn2)), ((p.p330 * locals.var_tdiff0_dn4) + (p.p331 * locals.var_tdiff0_2_dn4)), ((p.p330 * locals.var_tdiff0_dn5) + (p.p331 * locals.var_tdiff0_2_dn5)), ((p.p330 * locals.var_tdiff0_dn6) + (p.p331 * locals.var_tdiff0_2_dn6)), ((p.p330 * locals.var_tdiff0_dn7) + (p.p331 * locals.var_tdiff0_2_dn7)), ((p.p330 * locals.var_tdiff0_dn8) + (p.p331 * locals.var_tdiff0_2_dn8)), ((p.p330 * locals.var_tdiff0_dn9) + (p.p331 * locals.var_tdiff0_2_dn9)), ((p.p330 * locals.var_tdiff0_dn10) + (p.p331 * locals.var_tdiff0_2_dn10)), ((p.p330 * locals.var_tdiff0_dn13) + (p.p331 * locals.var_tdiff0_2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign17960_e12481;
        locals.var_t2_dn0 = assign17960_e12481_d_n0;
        locals.var_t2_dn2 = assign17960_e12481_d_n2;
        locals.var_t2_dn4 = assign17960_e12481_d_n4;
        locals.var_t2_dn5 = assign17960_e12481_d_n5;
        locals.var_t2_dn6 = assign17960_e12481_d_n6;
        locals.var_t2_dn7 = assign17960_e12481_d_n7;
        locals.var_t2_dn8 = assign17960_e12481_d_n8;
        locals.var_t2_dn9 = assign17960_e12481_d_n9;
        locals.var_t2_dn10 = assign17960_e12481_d_n10;
        locals.var_t2_dn13 = assign17960_e12481_d_n13;

        let (assign17970_e12489, assign17970_e12489_d_n0, assign17970_e12489_d_n2, assign17970_e12489_d_n4, assign17970_e12489_d_n5, assign17970_e12489_d_n6, assign17970_e12489_d_n7, assign17970_e12489_d_n8, assign17970_e12489_d_n9, assign17970_e12489_d_n10, assign17970_e12489_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17970_e12485: f64 = locals.var_t2;
        let assign17970_e12487: f64 = (assign17970_e12485 - 0.05);
        (assign17970_e12487, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign17970_e12489;
        locals.var_tmf1_dn0 = assign17970_e12489_d_n0;
        locals.var_tmf1_dn2 = assign17970_e12489_d_n2;
        locals.var_tmf1_dn4 = assign17970_e12489_d_n4;
        locals.var_tmf1_dn5 = assign17970_e12489_d_n5;
        locals.var_tmf1_dn6 = assign17970_e12489_d_n6;
        locals.var_tmf1_dn7 = assign17970_e12489_d_n7;
        locals.var_tmf1_dn8 = assign17970_e12489_d_n8;
        locals.var_tmf1_dn9 = assign17970_e12489_d_n9;
        locals.var_tmf1_dn10 = assign17970_e12489_d_n10;
        locals.var_tmf1_dn13 = assign17970_e12489_d_n13;

        let (assign17980_e12497, assign17980_e12497_d_n0, assign17980_e12497_d_n2, assign17980_e12497_d_n4, assign17980_e12497_d_n5, assign17980_e12497_d_n6, assign17980_e12497_d_n7, assign17980_e12497_d_n8, assign17980_e12497_d_n9, assign17980_e12497_d_n10, assign17980_e12497_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign17980_e12497;
        locals.var_tmf2_dn0 = assign17980_e12497_d_n0;
        locals.var_tmf2_dn2 = assign17980_e12497_d_n2;
        locals.var_tmf2_dn4 = assign17980_e12497_d_n4;
        locals.var_tmf2_dn5 = assign17980_e12497_d_n5;
        locals.var_tmf2_dn6 = assign17980_e12497_d_n6;
        locals.var_tmf2_dn7 = assign17980_e12497_d_n7;
        locals.var_tmf2_dn8 = assign17980_e12497_d_n8;
        locals.var_tmf2_dn9 = assign17980_e12497_d_n9;
        locals.var_tmf2_dn10 = assign17980_e12497_d_n10;
        locals.var_tmf2_dn13 = assign17980_e12497_d_n13;

        let (assign17990_e12507, assign17990_e12507_d_n0, assign17990_e12507_d_n2, assign17990_e12507_d_n4, assign17990_e12507_d_n5, assign17990_e12507_d_n6, assign17990_e12507_d_n7, assign17990_e12507_d_n8, assign17990_e12507_d_n9, assign17990_e12507_d_n10, assign17990_e12507_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let (assign17990_e12505, assign17990_e12505_d_n0, assign17990_e12505_d_n2, assign17990_e12505_d_n4, assign17990_e12505_d_n5, assign17990_e12505_d_n6, assign17990_e12505_d_n7, assign17990_e12505_d_n8, assign17990_e12505_d_n9, assign17990_e12505_d_n10, assign17990_e12505_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign17990_e12504: f64 = (-locals.var_tmf2);
                (assign17990_e12504, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign17990_e12505, assign17990_e12505_d_n0, assign17990_e12505_d_n2, assign17990_e12505_d_n4, assign17990_e12505_d_n5, assign17990_e12505_d_n6, assign17990_e12505_d_n7, assign17990_e12505_d_n8, assign17990_e12505_d_n9, assign17990_e12505_d_n10, assign17990_e12505_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign17990_e12507;
        locals.var_tmf2_dn0 = assign17990_e12507_d_n0;
        locals.var_tmf2_dn2 = assign17990_e12507_d_n2;
        locals.var_tmf2_dn4 = assign17990_e12507_d_n4;
        locals.var_tmf2_dn5 = assign17990_e12507_d_n5;
        locals.var_tmf2_dn6 = assign17990_e12507_d_n6;
        locals.var_tmf2_dn7 = assign17990_e12507_d_n7;
        locals.var_tmf2_dn8 = assign17990_e12507_d_n8;
        locals.var_tmf2_dn9 = assign17990_e12507_d_n9;
        locals.var_tmf2_dn10 = assign17990_e12507_d_n10;
        locals.var_tmf2_dn13 = assign17990_e12507_d_n13;

        let (assign18000_e12516, assign18000_e12516_d_n0, assign18000_e12516_d_n2, assign18000_e12516_d_n4, assign18000_e12516_d_n5, assign18000_e12516_d_n6, assign18000_e12516_d_n7, assign18000_e12516_d_n8, assign18000_e12516_d_n9, assign18000_e12516_d_n10, assign18000_e12516_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign18000_e12511: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18000_e12513: f64 = (assign18000_e12511 + locals.var_tmf2);
        let assign18000_e12514: f64 = (assign18000_e12513).sqrt();
        (assign18000_e12514, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18000_e12514)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18000_e12514)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign18000_e12514)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign18000_e12514)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign18000_e12514)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign18000_e12514)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign18000_e12514)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign18000_e12514)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign18000_e12514)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign18000_e12514)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18000_e12516;
        locals.var_tmf2_dn0 = assign18000_e12516_d_n0;
        locals.var_tmf2_dn2 = assign18000_e12516_d_n2;
        locals.var_tmf2_dn4 = assign18000_e12516_d_n4;
        locals.var_tmf2_dn5 = assign18000_e12516_d_n5;
        locals.var_tmf2_dn6 = assign18000_e12516_d_n6;
        locals.var_tmf2_dn7 = assign18000_e12516_d_n7;
        locals.var_tmf2_dn8 = assign18000_e12516_d_n8;
        locals.var_tmf2_dn9 = assign18000_e12516_d_n9;
        locals.var_tmf2_dn10 = assign18000_e12516_d_n10;
        locals.var_tmf2_dn13 = assign18000_e12516_d_n13;

        let (assign18010_e12526, assign18010_e12526_d_n0, assign18010_e12526_d_n2, assign18010_e12526_d_n4, assign18010_e12526_d_n5, assign18010_e12526_d_n6, assign18010_e12526_d_n7, assign18010_e12526_d_n8, assign18010_e12526_d_n9, assign18010_e12526_d_n10, assign18010_e12526_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign18010_e12522: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign18010_e12523: f64 = (1.0 + assign18010_e12522);
        let assign18010_e12524: f64 = (0.5 * assign18010_e12523);
        (assign18010_e12524, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign18010_e12526;
        locals.var_t0_dn0 = assign18010_e12526_d_n0;
        locals.var_t0_dn2 = assign18010_e12526_d_n2;
        locals.var_t0_dn4 = assign18010_e12526_d_n4;
        locals.var_t0_dn5 = assign18010_e12526_d_n5;
        locals.var_t0_dn6 = assign18010_e12526_d_n6;
        locals.var_t0_dn7 = assign18010_e12526_d_n7;
        locals.var_t0_dn8 = assign18010_e12526_d_n8;
        locals.var_t0_dn9 = assign18010_e12526_d_n9;
        locals.var_t0_dn10 = assign18010_e12526_d_n10;
        locals.var_t0_dn13 = assign18010_e12526_d_n13;

        let (assign18020_e12536, assign18020_e12536_d_n0, assign18020_e12536_d_n2, assign18020_e12536_d_n4, assign18020_e12536_d_n5, assign18020_e12536_d_n6, assign18020_e12536_d_n7, assign18020_e12536_d_n8, assign18020_e12536_d_n9, assign18020_e12536_d_n10, assign18020_e12536_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign18020_e12532: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign18020_e12533: f64 = (0.5 * assign18020_e12532);
        let assign18020_e12534: f64 = assign18020_e12533;
        (assign18020_e12534, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign18020_e12536;
        locals.var_t2_dn0 = assign18020_e12536_d_n0;
        locals.var_t2_dn2 = assign18020_e12536_d_n2;
        locals.var_t2_dn4 = assign18020_e12536_d_n4;
        locals.var_t2_dn5 = assign18020_e12536_d_n5;
        locals.var_t2_dn6 = assign18020_e12536_d_n6;
        locals.var_t2_dn7 = assign18020_e12536_d_n7;
        locals.var_t2_dn8 = assign18020_e12536_d_n8;
        locals.var_t2_dn9 = assign18020_e12536_d_n9;
        locals.var_t2_dn10 = assign18020_e12536_d_n10;
        locals.var_t2_dn13 = assign18020_e12536_d_n13;

        let (assign18030_e12544, assign18030_e12544_d_n0, assign18030_e12544_d_n2, assign18030_e12544_d_n4, assign18030_e12544_d_n5, assign18030_e12544_d_n6, assign18030_e12544_d_n7, assign18030_e12544_d_n8, assign18030_e12544_d_n9, assign18030_e12544_d_n10, assign18030_e12544_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign18030_e12540: f64 = (1.0 - locals.var_t2);
        let assign18030_e12542: f64 = (assign18030_e12540 - 0.05);
        (assign18030_e12542, (-locals.var_t2_dn0), (-locals.var_t2_dn2), (-locals.var_t2_dn4), (-locals.var_t2_dn5), (-locals.var_t2_dn6), (-locals.var_t2_dn7), (-locals.var_t2_dn8), (-locals.var_t2_dn9), (-locals.var_t2_dn10), (-locals.var_t2_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign18030_e12544;
        locals.var_tmf1_dn0 = assign18030_e12544_d_n0;
        locals.var_tmf1_dn2 = assign18030_e12544_d_n2;
        locals.var_tmf1_dn4 = assign18030_e12544_d_n4;
        locals.var_tmf1_dn5 = assign18030_e12544_d_n5;
        locals.var_tmf1_dn6 = assign18030_e12544_d_n6;
        locals.var_tmf1_dn7 = assign18030_e12544_d_n7;
        locals.var_tmf1_dn8 = assign18030_e12544_d_n8;
        locals.var_tmf1_dn9 = assign18030_e12544_d_n9;
        locals.var_tmf1_dn10 = assign18030_e12544_d_n10;
        locals.var_tmf1_dn13 = assign18030_e12544_d_n13;

        let (assign18040_e12552, assign18040_e12552_d_n0, assign18040_e12552_d_n2, assign18040_e12552_d_n4, assign18040_e12552_d_n5, assign18040_e12552_d_n6, assign18040_e12552_d_n7, assign18040_e12552_d_n8, assign18040_e12552_d_n9, assign18040_e12552_d_n10, assign18040_e12552_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign18040_e12548: f64 = 4.0;
        let assign18040_e12550: f64 = (assign18040_e12548 * 0.05);
        (assign18040_e12550, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18040_e12552;
        locals.var_tmf2_dn0 = assign18040_e12552_d_n0;
        locals.var_tmf2_dn2 = assign18040_e12552_d_n2;
        locals.var_tmf2_dn4 = assign18040_e12552_d_n4;
        locals.var_tmf2_dn5 = assign18040_e12552_d_n5;
        locals.var_tmf2_dn6 = assign18040_e12552_d_n6;
        locals.var_tmf2_dn7 = assign18040_e12552_d_n7;
        locals.var_tmf2_dn8 = assign18040_e12552_d_n8;
        locals.var_tmf2_dn9 = assign18040_e12552_d_n9;
        locals.var_tmf2_dn10 = assign18040_e12552_d_n10;
        locals.var_tmf2_dn13 = assign18040_e12552_d_n13;

        let (assign18050_e12562, assign18050_e12562_d_n0, assign18050_e12562_d_n2, assign18050_e12562_d_n4, assign18050_e12562_d_n5, assign18050_e12562_d_n6, assign18050_e12562_d_n7, assign18050_e12562_d_n8, assign18050_e12562_d_n9, assign18050_e12562_d_n10, assign18050_e12562_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let (assign18050_e12560, assign18050_e12560_d_n0, assign18050_e12560_d_n2, assign18050_e12560_d_n4, assign18050_e12560_d_n5, assign18050_e12560_d_n6, assign18050_e12560_d_n7, assign18050_e12560_d_n8, assign18050_e12560_d_n9, assign18050_e12560_d_n10, assign18050_e12560_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign18050_e12559: f64 = (-locals.var_tmf2);
                (assign18050_e12559, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign18050_e12560, assign18050_e12560_d_n0, assign18050_e12560_d_n2, assign18050_e12560_d_n4, assign18050_e12560_d_n5, assign18050_e12560_d_n6, assign18050_e12560_d_n7, assign18050_e12560_d_n8, assign18050_e12560_d_n9, assign18050_e12560_d_n10, assign18050_e12560_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18050_e12562;
        locals.var_tmf2_dn0 = assign18050_e12562_d_n0;
        locals.var_tmf2_dn2 = assign18050_e12562_d_n2;
        locals.var_tmf2_dn4 = assign18050_e12562_d_n4;
        locals.var_tmf2_dn5 = assign18050_e12562_d_n5;
        locals.var_tmf2_dn6 = assign18050_e12562_d_n6;
        locals.var_tmf2_dn7 = assign18050_e12562_d_n7;
        locals.var_tmf2_dn8 = assign18050_e12562_d_n8;
        locals.var_tmf2_dn9 = assign18050_e12562_d_n9;
        locals.var_tmf2_dn10 = assign18050_e12562_d_n10;
        locals.var_tmf2_dn13 = assign18050_e12562_d_n13;

        let (assign18060_e12571, assign18060_e12571_d_n0, assign18060_e12571_d_n2, assign18060_e12571_d_n4, assign18060_e12571_d_n5, assign18060_e12571_d_n6, assign18060_e12571_d_n7, assign18060_e12571_d_n8, assign18060_e12571_d_n9, assign18060_e12571_d_n10, assign18060_e12571_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign18060_e12566: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18060_e12568: f64 = (assign18060_e12566 + locals.var_tmf2);
        let assign18060_e12569: f64 = (assign18060_e12568).sqrt();
        (assign18060_e12569, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18060_e12569)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18060_e12569)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign18060_e12569)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign18060_e12569)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign18060_e12569)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign18060_e12569)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign18060_e12569)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign18060_e12569)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign18060_e12569)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign18060_e12569)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18060_e12571;
        locals.var_tmf2_dn0 = assign18060_e12571_d_n0;
        locals.var_tmf2_dn2 = assign18060_e12571_d_n2;
        locals.var_tmf2_dn4 = assign18060_e12571_d_n4;
        locals.var_tmf2_dn5 = assign18060_e12571_d_n5;
        locals.var_tmf2_dn6 = assign18060_e12571_d_n6;
        locals.var_tmf2_dn7 = assign18060_e12571_d_n7;
        locals.var_tmf2_dn8 = assign18060_e12571_d_n8;
        locals.var_tmf2_dn9 = assign18060_e12571_d_n9;
        locals.var_tmf2_dn10 = assign18060_e12571_d_n10;
        locals.var_tmf2_dn13 = assign18060_e12571_d_n13;

        let (assign18070_e12581, assign18070_e12581_d_n0, assign18070_e12581_d_n2, assign18070_e12581_d_n4, assign18070_e12581_d_n5, assign18070_e12581_d_n6, assign18070_e12581_d_n7, assign18070_e12581_d_n8, assign18070_e12581_d_n9, assign18070_e12581_d_n10, assign18070_e12581_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign18070_e12577: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign18070_e12578: f64 = (1.0 + assign18070_e12577);
        let assign18070_e12579: f64 = (0.5 * assign18070_e12578);
        (assign18070_e12579, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign18070_e12581;
        locals.var_t0_dn0 = assign18070_e12581_d_n0;
        locals.var_t0_dn2 = assign18070_e12581_d_n2;
        locals.var_t0_dn4 = assign18070_e12581_d_n4;
        locals.var_t0_dn5 = assign18070_e12581_d_n5;
        locals.var_t0_dn6 = assign18070_e12581_d_n6;
        locals.var_t0_dn7 = assign18070_e12581_d_n7;
        locals.var_t0_dn8 = assign18070_e12581_d_n8;
        locals.var_t0_dn9 = assign18070_e12581_d_n9;
        locals.var_t0_dn10 = assign18070_e12581_d_n10;
        locals.var_t0_dn13 = assign18070_e12581_d_n13;

        let (assign18080_e12591, assign18080_e12591_d_n0, assign18080_e12591_d_n2, assign18080_e12591_d_n4, assign18080_e12591_d_n5, assign18080_e12591_d_n6, assign18080_e12591_d_n7, assign18080_e12591_d_n8, assign18080_e12591_d_n9, assign18080_e12591_d_n10, assign18080_e12591_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign18080_e12587: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign18080_e12588: f64 = (0.5 * assign18080_e12587);
        let assign18080_e12589: f64 = (1.0 - assign18080_e12588);
        (assign18080_e12589, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_powratio, locals.var_powratio_dn0, locals.var_powratio_dn2, locals.var_powratio_dn4, locals.var_powratio_dn5, locals.var_powratio_dn6, locals.var_powratio_dn7, locals.var_powratio_dn8, locals.var_powratio_dn9, locals.var_powratio_dn10, locals.var_powratio_dn13,)
    }
};
        locals.var_powratio = assign18080_e12591;
        locals.var_powratio_dn0 = assign18080_e12591_d_n0;
        locals.var_powratio_dn2 = assign18080_e12591_d_n2;
        locals.var_powratio_dn4 = assign18080_e12591_d_n4;
        locals.var_powratio_dn5 = assign18080_e12591_d_n5;
        locals.var_powratio_dn6 = assign18080_e12591_d_n6;
        locals.var_powratio_dn7 = assign18080_e12591_d_n7;
        locals.var_powratio_dn8 = assign18080_e12591_d_n8;
        locals.var_powratio_dn9 = assign18080_e12591_d_n9;
        locals.var_powratio_dn10 = assign18080_e12591_d_n10;
        locals.var_powratio_dn13 = assign18080_e12591_d_n13;

        let (assign18090_e12602, assign18090_e12602_d_n0, assign18090_e12602_d_n2, assign18090_e12602_d_n4, assign18090_e12602_d_n5, assign18090_e12602_d_n6, assign18090_e12602_d_n7, assign18090_e12602_d_n8, assign18090_e12602_d_n9, assign18090_e12602_d_n10, assign18090_e12602_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign18090_e12595: f64 = (2.0 * locals.var_beta_inv);
        let assign18090_e12598: f64 = (locals.var_nsub / locals.var_nin);
        let assign18090_e12599: f64 = (assign18090_e12598).ln();
        let assign18090_e12600: f64 = (assign18090_e12595 * assign18090_e12599);
        (assign18090_e12600, (((2.0 * locals.var_beta_inv_dn0) * assign18090_e12599) + (assign18090_e12595 * ((((locals.var_nsub_dn0 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign18090_e12598))), (((2.0 * locals.var_beta_inv_dn2) * assign18090_e12599) + (assign18090_e12595 * ((((locals.var_nsub_dn2 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign18090_e12598))), (((2.0 * locals.var_beta_inv_dn4) * assign18090_e12599) + (assign18090_e12595 * ((((locals.var_nsub_dn4 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign18090_e12598))), (((2.0 * locals.var_beta_inv_dn5) * assign18090_e12599) + (assign18090_e12595 * ((((locals.var_nsub_dn5 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign18090_e12598))), (((2.0 * locals.var_beta_inv_dn6) * assign18090_e12599) + (assign18090_e12595 * ((((locals.var_nsub_dn6 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign18090_e12598))), (((2.0 * locals.var_beta_inv_dn7) * assign18090_e12599) + (assign18090_e12595 * ((((locals.var_nsub_dn7 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign18090_e12598))), (((2.0 * locals.var_beta_inv_dn8) * assign18090_e12599) + (assign18090_e12595 * ((((locals.var_nsub_dn8 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign18090_e12598))), (((2.0 * locals.var_beta_inv_dn9) * assign18090_e12599) + (assign18090_e12595 * ((((locals.var_nsub_dn9 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign18090_e12598))), (((2.0 * locals.var_beta_inv_dn10) * assign18090_e12599) + (assign18090_e12595 * ((((locals.var_nsub_dn10 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign18090_e12598))), (((2.0 * locals.var_beta_inv_dn13) * assign18090_e12599) + (assign18090_e12595 * ((((locals.var_nsub_dn13 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn13)) / (locals.var_nin * locals.var_nin)) / assign18090_e12598))),)
    } else {
        (locals.var_pb2, locals.var_pb2_dn0, locals.var_pb2_dn2, locals.var_pb2_dn4, locals.var_pb2_dn5, locals.var_pb2_dn6, locals.var_pb2_dn7, locals.var_pb2_dn8, locals.var_pb2_dn9, locals.var_pb2_dn10, locals.var_pb2_dn13,)
    }
};
        locals.var_pb2 = assign18090_e12602;
        locals.var_pb2_dn0 = assign18090_e12602_d_n0;
        locals.var_pb2_dn2 = assign18090_e12602_d_n2;
        locals.var_pb2_dn4 = assign18090_e12602_d_n4;
        locals.var_pb2_dn5 = assign18090_e12602_d_n5;
        locals.var_pb2_dn6 = assign18090_e12602_d_n6;
        locals.var_pb2_dn7 = assign18090_e12602_d_n7;
        locals.var_pb2_dn8 = assign18090_e12602_d_n8;
        locals.var_pb2_dn9 = assign18090_e12602_d_n9;
        locals.var_pb2_dn10 = assign18090_e12602_d_n10;
        locals.var_pb2_dn13 = assign18090_e12602_d_n13;

        let (assign18100_e12610, assign18100_e12610_d_n0, assign18100_e12610_d_n2, assign18100_e12610_d_n4, assign18100_e12610_d_n5, assign18100_e12610_d_n6, assign18100_e12610_d_n7, assign18100_e12610_d_n8, assign18100_e12610_d_n9, assign18100_e12610_d_n10, assign18100_e12610_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign18100_e12606: f64 = (2.0 * 1.034943e-10);
        let assign18100_e12608: f64 = (assign18100_e12606 / 1.6021918e-19);
        (assign18100_e12608, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign18100_e12610;
        locals.var_t1_dn0 = assign18100_e12610_d_n0;
        locals.var_t1_dn2 = assign18100_e12610_d_n2;
        locals.var_t1_dn4 = assign18100_e12610_d_n4;
        locals.var_t1_dn5 = assign18100_e12610_d_n5;
        locals.var_t1_dn6 = assign18100_e12610_d_n6;
        locals.var_t1_dn7 = assign18100_e12610_d_n7;
        locals.var_t1_dn8 = assign18100_e12610_d_n8;
        locals.var_t1_dn9 = assign18100_e12610_d_n9;
        locals.var_t1_dn10 = assign18100_e12610_d_n10;
        locals.var_t1_dn13 = assign18100_e12610_d_n13;

        let (assign18110_e12617, assign18110_e12617_d_n0, assign18110_e12617_d_n2, assign18110_e12617_d_n4, assign18110_e12617_d_n5, assign18110_e12617_d_n6, assign18110_e12617_d_n7, assign18110_e12617_d_n8, assign18110_e12617_d_n9, assign18110_e12617_d_n10, assign18110_e12617_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign18110_e12614: f64 = (locals.var_t1 / locals.var_nsub);
        let assign18110_e12615: f64 = (assign18110_e12614).sqrt();
        (assign18110_e12615, ((((locals.var_t1_dn0 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn0)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18110_e12615)), ((((locals.var_t1_dn2 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn2)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18110_e12615)), ((((locals.var_t1_dn4 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn4)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18110_e12615)), ((((locals.var_t1_dn5 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn5)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18110_e12615)), ((((locals.var_t1_dn6 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn6)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18110_e12615)), ((((locals.var_t1_dn7 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn7)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18110_e12615)), ((((locals.var_t1_dn8 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn8)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18110_e12615)), ((((locals.var_t1_dn9 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn9)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18110_e12615)), ((((locals.var_t1_dn10 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn10)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18110_e12615)), ((((locals.var_t1_dn13 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn13)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18110_e12615)),)
    } else {
        (locals.var_wdpl, locals.var_wdpl_dn0, locals.var_wdpl_dn2, locals.var_wdpl_dn4, locals.var_wdpl_dn5, locals.var_wdpl_dn6, locals.var_wdpl_dn7, locals.var_wdpl_dn8, locals.var_wdpl_dn9, locals.var_wdpl_dn10, locals.var_wdpl_dn13,)
    }
};
        locals.var_wdpl = assign18110_e12617;
        locals.var_wdpl_dn0 = assign18110_e12617_d_n0;
        locals.var_wdpl_dn2 = assign18110_e12617_d_n2;
        locals.var_wdpl_dn4 = assign18110_e12617_d_n4;
        locals.var_wdpl_dn5 = assign18110_e12617_d_n5;
        locals.var_wdpl_dn6 = assign18110_e12617_d_n6;
        locals.var_wdpl_dn7 = assign18110_e12617_d_n7;
        locals.var_wdpl_dn8 = assign18110_e12617_d_n8;
        locals.var_wdpl_dn9 = assign18110_e12617_d_n9;
        locals.var_wdpl_dn10 = assign18110_e12617_d_n10;
        locals.var_wdpl_dn13 = assign18110_e12617_d_n13;

        let (assign18120_e12624, assign18120_e12624_d_n0, assign18120_e12624_d_n2, assign18120_e12624_d_n4, assign18120_e12624_d_n5, assign18120_e12624_d_n6, assign18120_e12624_d_n7, assign18120_e12624_d_n8, assign18120_e12624_d_n9, assign18120_e12624_d_n10, assign18120_e12624_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign18120_e12621: f64 = (locals.var_t1 / locals.var_ef_nsubp);
        let assign18120_e12622: f64 = (assign18120_e12621).sqrt();
        (assign18120_e12622, ((((locals.var_t1_dn0 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn0)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18120_e12622)), ((((locals.var_t1_dn2 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn2)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18120_e12622)), ((((locals.var_t1_dn4 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn4)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18120_e12622)), ((((locals.var_t1_dn5 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn5)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18120_e12622)), ((((locals.var_t1_dn6 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn6)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18120_e12622)), ((((locals.var_t1_dn7 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn7)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18120_e12622)), ((((locals.var_t1_dn8 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn8)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18120_e12622)), ((((locals.var_t1_dn9 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn9)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18120_e12622)), ((((locals.var_t1_dn10 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn10)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18120_e12622)), ((((locals.var_t1_dn13 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn13)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18120_e12622)),)
    } else {
        (locals.var_wdplp, locals.var_wdplp_dn0, locals.var_wdplp_dn2, locals.var_wdplp_dn4, locals.var_wdplp_dn5, locals.var_wdplp_dn6, locals.var_wdplp_dn7, locals.var_wdplp_dn8, locals.var_wdplp_dn9, locals.var_wdplp_dn10, locals.var_wdplp_dn13,)
    }
};
        locals.var_wdplp = assign18120_e12624;
        locals.var_wdplp_dn0 = assign18120_e12624_d_n0;
        locals.var_wdplp_dn2 = assign18120_e12624_d_n2;
        locals.var_wdplp_dn4 = assign18120_e12624_d_n4;
        locals.var_wdplp_dn5 = assign18120_e12624_d_n5;
        locals.var_wdplp_dn6 = assign18120_e12624_d_n6;
        locals.var_wdplp_dn7 = assign18120_e12624_d_n7;
        locals.var_wdplp_dn8 = assign18120_e12624_d_n8;
        locals.var_wdplp_dn9 = assign18120_e12624_d_n9;
        locals.var_wdplp_dn10 = assign18120_e12624_d_n10;
        locals.var_wdplp_dn13 = assign18120_e12624_d_n13;

        let assign18130_e12627: f64 = if locals.var_uc_codep == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard372 = assign18130_e12627;

        let (assign18140_e12642, assign18140_e12642_d_n0, assign18140_e12642_d_n2, assign18140_e12642_d_n4, assign18140_e12642_d_n5, assign18140_e12642_d_n6, assign18140_e12642_d_n7, assign18140_e12642_d_n8, assign18140_e12642_d_n9, assign18140_e12642_d_n10, assign18140_e12642_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard372 != 0.0)) {
        let assign18140_e12633: f64 = (2.0 * 1.034943e-10);
        let assign18140_e12635: f64 = (assign18140_e12633 * 1.6021918e-19);
        let assign18140_e12637: f64 = (assign18140_e12635 * locals.var_nsub);
        let assign18140_e12639: f64 = (assign18140_e12637 * locals.var_beta_inv);
        let assign18140_e12640: f64 = (assign18140_e12639).sqrt();
        (assign18140_e12640, ((((assign18140_e12635 * locals.var_nsub_dn0) * locals.var_beta_inv) + (assign18140_e12637 * locals.var_beta_inv_dn0)) / (2.0 * assign18140_e12640)), ((((assign18140_e12635 * locals.var_nsub_dn2) * locals.var_beta_inv) + (assign18140_e12637 * locals.var_beta_inv_dn2)) / (2.0 * assign18140_e12640)), ((((assign18140_e12635 * locals.var_nsub_dn4) * locals.var_beta_inv) + (assign18140_e12637 * locals.var_beta_inv_dn4)) / (2.0 * assign18140_e12640)), ((((assign18140_e12635 * locals.var_nsub_dn5) * locals.var_beta_inv) + (assign18140_e12637 * locals.var_beta_inv_dn5)) / (2.0 * assign18140_e12640)), ((((assign18140_e12635 * locals.var_nsub_dn6) * locals.var_beta_inv) + (assign18140_e12637 * locals.var_beta_inv_dn6)) / (2.0 * assign18140_e12640)), ((((assign18140_e12635 * locals.var_nsub_dn7) * locals.var_beta_inv) + (assign18140_e12637 * locals.var_beta_inv_dn7)) / (2.0 * assign18140_e12640)), ((((assign18140_e12635 * locals.var_nsub_dn8) * locals.var_beta_inv) + (assign18140_e12637 * locals.var_beta_inv_dn8)) / (2.0 * assign18140_e12640)), ((((assign18140_e12635 * locals.var_nsub_dn9) * locals.var_beta_inv) + (assign18140_e12637 * locals.var_beta_inv_dn9)) / (2.0 * assign18140_e12640)), ((((assign18140_e12635 * locals.var_nsub_dn10) * locals.var_beta_inv) + (assign18140_e12637 * locals.var_beta_inv_dn10)) / (2.0 * assign18140_e12640)), ((((assign18140_e12635 * locals.var_nsub_dn13) * locals.var_beta_inv) + (assign18140_e12637 * locals.var_beta_inv_dn13)) / (2.0 * assign18140_e12640)),)
    } else {
        (locals.var_cnst0, locals.var_cnst0_dn0, locals.var_cnst0_dn2, locals.var_cnst0_dn4, locals.var_cnst0_dn5, locals.var_cnst0_dn6, locals.var_cnst0_dn7, locals.var_cnst0_dn8, locals.var_cnst0_dn9, locals.var_cnst0_dn10, locals.var_cnst0_dn13,)
    }
};
        locals.var_cnst0 = assign18140_e12642;
        locals.var_cnst0_dn0 = assign18140_e12642_d_n0;
        locals.var_cnst0_dn2 = assign18140_e12642_d_n2;
        locals.var_cnst0_dn4 = assign18140_e12642_d_n4;
        locals.var_cnst0_dn5 = assign18140_e12642_d_n5;
        locals.var_cnst0_dn6 = assign18140_e12642_d_n6;
        locals.var_cnst0_dn7 = assign18140_e12642_d_n7;
        locals.var_cnst0_dn8 = assign18140_e12642_d_n8;
        locals.var_cnst0_dn9 = assign18140_e12642_d_n9;
        locals.var_cnst0_dn10 = assign18140_e12642_d_n10;
        locals.var_cnst0_dn13 = assign18140_e12642_d_n13;

        let (assign18150_e12650, assign18150_e12650_d_n0, assign18150_e12650_d_n2, assign18150_e12650_d_n4, assign18150_e12650_d_n5, assign18150_e12650_d_n6, assign18150_e12650_d_n7, assign18150_e12650_d_n8, assign18150_e12650_d_n9, assign18150_e12650_d_n10, assign18150_e12650_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard372 != 0.0)) {
        let assign18150_e12648: f64 = (locals.var_nin / locals.var_nsub);
        (assign18150_e12648, (((locals.var_nin_dn0 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn0)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn2 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn2)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn4 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn4)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn5 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn5)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn6 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn6)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn7 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn7)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn8 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn8)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn9 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn9)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn10 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn10)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn13 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn13)) / (locals.var_nsub * locals.var_nsub)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign18150_e12650;
        locals.var_t1_dn0 = assign18150_e12650_d_n0;
        locals.var_t1_dn2 = assign18150_e12650_d_n2;
        locals.var_t1_dn4 = assign18150_e12650_d_n4;
        locals.var_t1_dn5 = assign18150_e12650_d_n5;
        locals.var_t1_dn6 = assign18150_e12650_d_n6;
        locals.var_t1_dn7 = assign18150_e12650_d_n7;
        locals.var_t1_dn8 = assign18150_e12650_d_n8;
        locals.var_t1_dn9 = assign18150_e12650_d_n9;
        locals.var_t1_dn10 = assign18150_e12650_d_n10;
        locals.var_t1_dn13 = assign18150_e12650_d_n13;

        let (assign18160_e12658, assign18160_e12658_d_n0, assign18160_e12658_d_n2, assign18160_e12658_d_n4, assign18160_e12658_d_n5, assign18160_e12658_d_n6, assign18160_e12658_d_n7, assign18160_e12658_d_n8, assign18160_e12658_d_n9, assign18160_e12658_d_n10, assign18160_e12658_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard372 != 0.0)) {
        let assign18160_e12656: f64 = (locals.var_t1 * locals.var_t1);
        (assign18160_e12656, ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)), ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)), ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)), ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)), ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)), ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)), ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)), ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)), ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)), ((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)),)
    } else {
        (locals.var_cnst1, locals.var_cnst1_dn0, locals.var_cnst1_dn2, locals.var_cnst1_dn4, locals.var_cnst1_dn5, locals.var_cnst1_dn6, locals.var_cnst1_dn7, locals.var_cnst1_dn8, locals.var_cnst1_dn9, locals.var_cnst1_dn10, locals.var_cnst1_dn13,)
    }
};
        locals.var_cnst1 = assign18160_e12658;
        locals.var_cnst1_dn0 = assign18160_e12658_d_n0;
        locals.var_cnst1_dn2 = assign18160_e12658_d_n2;
        locals.var_cnst1_dn4 = assign18160_e12658_d_n4;
        locals.var_cnst1_dn5 = assign18160_e12658_d_n5;
        locals.var_cnst1_dn6 = assign18160_e12658_d_n6;
        locals.var_cnst1_dn7 = assign18160_e12658_d_n7;
        locals.var_cnst1_dn8 = assign18160_e12658_d_n8;
        locals.var_cnst1_dn9 = assign18160_e12658_d_n9;
        locals.var_cnst1_dn10 = assign18160_e12658_d_n10;
        locals.var_cnst1_dn13 = assign18160_e12658_d_n13;

        let assign18170_e12661: f64 = if locals.var_uc_codep == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard373 = assign18170_e12661;

        let assign18180_e12664: f64 = if locals.var_uc_nover != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard374 = assign18180_e12664;

    }

    pub(super) fn stamp_transient_block_39(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign18190_e12677, assign18190_e12677_d_n0, assign18190_e12677_d_n2, assign18190_e12677_d_n4, assign18190_e12677_d_n5, assign18190_e12677_d_n6, assign18190_e12677_d_n7, assign18190_e12677_d_n8, assign18190_e12677_d_n9, assign18190_e12677_d_n10, assign18190_e12677_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard373 != 0.0)) && (locals.var_guard374 != 0.0)) {
        let assign18190_e12673: f64 = (locals.var_uc_nover / locals.var_nsub);
        let assign18190_e12674: f64 = (assign18190_e12673).sqrt();
        let assign18190_e12675: f64 = (locals.var_cnst0 * assign18190_e12674);
        (assign18190_e12675, ((locals.var_cnst0_dn0 * assign18190_e12674) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn0) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18190_e12674)))), ((locals.var_cnst0_dn2 * assign18190_e12674) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn2) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18190_e12674)))), ((locals.var_cnst0_dn4 * assign18190_e12674) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn4) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18190_e12674)))), ((locals.var_cnst0_dn5 * assign18190_e12674) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn5) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18190_e12674)))), ((locals.var_cnst0_dn6 * assign18190_e12674) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn6) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18190_e12674)))), ((locals.var_cnst0_dn7 * assign18190_e12674) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn7) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18190_e12674)))), ((locals.var_cnst0_dn8 * assign18190_e12674) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn8) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18190_e12674)))), ((locals.var_cnst0_dn9 * assign18190_e12674) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn9) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18190_e12674)))), ((locals.var_cnst0_dn10 * assign18190_e12674) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn10) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18190_e12674)))), ((locals.var_cnst0_dn13 * assign18190_e12674) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn13) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18190_e12674)))),)
    } else {
        (locals.var_cnst0over, locals.var_cnst0over_dn0, locals.var_cnst0over_dn2, locals.var_cnst0over_dn4, locals.var_cnst0over_dn5, locals.var_cnst0over_dn6, locals.var_cnst0over_dn7, locals.var_cnst0over_dn8, locals.var_cnst0over_dn9, locals.var_cnst0over_dn10, locals.var_cnst0over_dn13,)
    }
};
        locals.var_cnst0over = assign18190_e12677;
        locals.var_cnst0over_dn0 = assign18190_e12677_d_n0;
        locals.var_cnst0over_dn2 = assign18190_e12677_d_n2;
        locals.var_cnst0over_dn4 = assign18190_e12677_d_n4;
        locals.var_cnst0over_dn5 = assign18190_e12677_d_n5;
        locals.var_cnst0over_dn6 = assign18190_e12677_d_n6;
        locals.var_cnst0over_dn7 = assign18190_e12677_d_n7;
        locals.var_cnst0over_dn8 = assign18190_e12677_d_n8;
        locals.var_cnst0over_dn9 = assign18190_e12677_d_n9;
        locals.var_cnst0over_dn10 = assign18190_e12677_d_n10;
        locals.var_cnst0over_dn13 = assign18190_e12677_d_n13;

        let assign18200_e12680: f64 = if locals.var_uc_novers != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard375 = assign18200_e12680;

        let (assign18210_e12693, assign18210_e12693_d_n0, assign18210_e12693_d_n2, assign18210_e12693_d_n4, assign18210_e12693_d_n5, assign18210_e12693_d_n6, assign18210_e12693_d_n7, assign18210_e12693_d_n8, assign18210_e12693_d_n9, assign18210_e12693_d_n10, assign18210_e12693_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard373 != 0.0)) && (locals.var_guard375 != 0.0)) {
        let assign18210_e12689: f64 = (locals.var_uc_novers / locals.var_nsub);
        let assign18210_e12690: f64 = (assign18210_e12689).sqrt();
        let assign18210_e12691: f64 = (locals.var_cnst0 * assign18210_e12690);
        (assign18210_e12691, ((locals.var_cnst0_dn0 * assign18210_e12690) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn0) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18210_e12690)))), ((locals.var_cnst0_dn2 * assign18210_e12690) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn2) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18210_e12690)))), ((locals.var_cnst0_dn4 * assign18210_e12690) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn4) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18210_e12690)))), ((locals.var_cnst0_dn5 * assign18210_e12690) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn5) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18210_e12690)))), ((locals.var_cnst0_dn6 * assign18210_e12690) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn6) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18210_e12690)))), ((locals.var_cnst0_dn7 * assign18210_e12690) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn7) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18210_e12690)))), ((locals.var_cnst0_dn8 * assign18210_e12690) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn8) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18210_e12690)))), ((locals.var_cnst0_dn9 * assign18210_e12690) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn9) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18210_e12690)))), ((locals.var_cnst0_dn10 * assign18210_e12690) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn10) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18210_e12690)))), ((locals.var_cnst0_dn13 * assign18210_e12690) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn13) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18210_e12690)))),)
    } else {
        (locals.var_cnst0overs, locals.var_cnst0overs_dn0, locals.var_cnst0overs_dn2, locals.var_cnst0overs_dn4, locals.var_cnst0overs_dn5, locals.var_cnst0overs_dn6, locals.var_cnst0overs_dn7, locals.var_cnst0overs_dn8, locals.var_cnst0overs_dn9, locals.var_cnst0overs_dn10, locals.var_cnst0overs_dn13,)
    }
};
        locals.var_cnst0overs = assign18210_e12693;
        locals.var_cnst0overs_dn0 = assign18210_e12693_d_n0;
        locals.var_cnst0overs_dn2 = assign18210_e12693_d_n2;
        locals.var_cnst0overs_dn4 = assign18210_e12693_d_n4;
        locals.var_cnst0overs_dn5 = assign18210_e12693_d_n5;
        locals.var_cnst0overs_dn6 = assign18210_e12693_d_n6;
        locals.var_cnst0overs_dn7 = assign18210_e12693_d_n7;
        locals.var_cnst0overs_dn8 = assign18210_e12693_d_n8;
        locals.var_cnst0overs_dn9 = assign18210_e12693_d_n9;
        locals.var_cnst0overs_dn10 = assign18210_e12693_d_n10;
        locals.var_cnst0overs_dn13 = assign18210_e12693_d_n13;

        let assign18220_e12696: f64 = if locals.var_uc_nover != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard376 = assign18220_e12696;

        let (assign18230_e12710, assign18230_e12710_d_n0, assign18230_e12710_d_n2, assign18230_e12710_d_n4, assign18230_e12710_d_n5, assign18230_e12710_d_n6, assign18230_e12710_d_n7, assign18230_e12710_d_n8, assign18230_e12710_d_n9, assign18230_e12710_d_n10, assign18230_e12710_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard373 == 0.0)) && (locals.var_guard376 != 0.0)) {
        let assign18230_e12706: f64 = (locals.var_uc_nover / locals.var_uc_ndepm);
        let assign18230_e12707: f64 = (assign18230_e12706).sqrt();
        let assign18230_e12708: f64 = (locals.var_cnst0 * assign18230_e12707);
        (assign18230_e12708, ((locals.var_cnst0_dn0 * assign18230_e12707) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn0) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18230_e12707)))), ((locals.var_cnst0_dn2 * assign18230_e12707) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn2) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18230_e12707)))), ((locals.var_cnst0_dn4 * assign18230_e12707) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn4) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18230_e12707)))), ((locals.var_cnst0_dn5 * assign18230_e12707) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn5) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18230_e12707)))), ((locals.var_cnst0_dn6 * assign18230_e12707) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn6) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18230_e12707)))), ((locals.var_cnst0_dn7 * assign18230_e12707) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn7) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18230_e12707)))), ((locals.var_cnst0_dn8 * assign18230_e12707) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn8) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18230_e12707)))), ((locals.var_cnst0_dn9 * assign18230_e12707) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn9) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18230_e12707)))), ((locals.var_cnst0_dn10 * assign18230_e12707) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn10) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18230_e12707)))), ((locals.var_cnst0_dn13 * assign18230_e12707) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn13) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18230_e12707)))),)
    } else {
        (locals.var_cnst0over, locals.var_cnst0over_dn0, locals.var_cnst0over_dn2, locals.var_cnst0over_dn4, locals.var_cnst0over_dn5, locals.var_cnst0over_dn6, locals.var_cnst0over_dn7, locals.var_cnst0over_dn8, locals.var_cnst0over_dn9, locals.var_cnst0over_dn10, locals.var_cnst0over_dn13,)
    }
};
        locals.var_cnst0over = assign18230_e12710;
        locals.var_cnst0over_dn0 = assign18230_e12710_d_n0;
        locals.var_cnst0over_dn2 = assign18230_e12710_d_n2;
        locals.var_cnst0over_dn4 = assign18230_e12710_d_n4;
        locals.var_cnst0over_dn5 = assign18230_e12710_d_n5;
        locals.var_cnst0over_dn6 = assign18230_e12710_d_n6;
        locals.var_cnst0over_dn7 = assign18230_e12710_d_n7;
        locals.var_cnst0over_dn8 = assign18230_e12710_d_n8;
        locals.var_cnst0over_dn9 = assign18230_e12710_d_n9;
        locals.var_cnst0over_dn10 = assign18230_e12710_d_n10;
        locals.var_cnst0over_dn13 = assign18230_e12710_d_n13;

        let assign18240_e12713: f64 = if locals.var_uc_novers != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard377 = assign18240_e12713;

        let (assign18250_e12727, assign18250_e12727_d_n0, assign18250_e12727_d_n2, assign18250_e12727_d_n4, assign18250_e12727_d_n5, assign18250_e12727_d_n6, assign18250_e12727_d_n7, assign18250_e12727_d_n8, assign18250_e12727_d_n9, assign18250_e12727_d_n10, assign18250_e12727_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard373 == 0.0)) && (locals.var_guard377 != 0.0)) {
        let assign18250_e12723: f64 = (locals.var_uc_novers / locals.var_uc_ndepm);
        let assign18250_e12724: f64 = (assign18250_e12723).sqrt();
        let assign18250_e12725: f64 = (locals.var_cnst0 * assign18250_e12724);
        (assign18250_e12725, ((locals.var_cnst0_dn0 * assign18250_e12724) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn0) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18250_e12724)))), ((locals.var_cnst0_dn2 * assign18250_e12724) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn2) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18250_e12724)))), ((locals.var_cnst0_dn4 * assign18250_e12724) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn4) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18250_e12724)))), ((locals.var_cnst0_dn5 * assign18250_e12724) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn5) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18250_e12724)))), ((locals.var_cnst0_dn6 * assign18250_e12724) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn6) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18250_e12724)))), ((locals.var_cnst0_dn7 * assign18250_e12724) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn7) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18250_e12724)))), ((locals.var_cnst0_dn8 * assign18250_e12724) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn8) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18250_e12724)))), ((locals.var_cnst0_dn9 * assign18250_e12724) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn9) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18250_e12724)))), ((locals.var_cnst0_dn10 * assign18250_e12724) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn10) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18250_e12724)))), ((locals.var_cnst0_dn13 * assign18250_e12724) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn13) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18250_e12724)))),)
    } else {
        (locals.var_cnst0overs, locals.var_cnst0overs_dn0, locals.var_cnst0overs_dn2, locals.var_cnst0overs_dn4, locals.var_cnst0overs_dn5, locals.var_cnst0overs_dn6, locals.var_cnst0overs_dn7, locals.var_cnst0overs_dn8, locals.var_cnst0overs_dn9, locals.var_cnst0overs_dn10, locals.var_cnst0overs_dn13,)
    }
};
        locals.var_cnst0overs = assign18250_e12727;
        locals.var_cnst0overs_dn0 = assign18250_e12727_d_n0;
        locals.var_cnst0overs_dn2 = assign18250_e12727_d_n2;
        locals.var_cnst0overs_dn4 = assign18250_e12727_d_n4;
        locals.var_cnst0overs_dn5 = assign18250_e12727_d_n5;
        locals.var_cnst0overs_dn6 = assign18250_e12727_d_n6;
        locals.var_cnst0overs_dn7 = assign18250_e12727_d_n7;
        locals.var_cnst0overs_dn8 = assign18250_e12727_d_n8;
        locals.var_cnst0overs_dn9 = assign18250_e12727_d_n9;
        locals.var_cnst0overs_dn10 = assign18250_e12727_d_n10;
        locals.var_cnst0overs_dn13 = assign18250_e12727_d_n13;

        let assign18260_e12730: f64 = if locals.var_uc_cordrift == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard378 = assign18260_e12730;

        let assign18270_e12733: f64 = if locals.var_uc_rd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard379 = assign18270_e12733;

        let (assign18280_e12757, assign18280_e12757_d_n0, assign18280_e12757_d_n2, assign18280_e12757_d_n4, assign18280_e12757_d_n5, assign18280_e12757_d_n6, assign18280_e12757_d_n7, assign18280_e12757_d_n8, assign18280_e12757_d_n9, assign18280_e12757_d_n10, assign18280_e12757_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard379 != 0.0)) {
        let assign18280_e12742: f64 = (p.p67 * locals.var_uc_rdslp1);
        let assign18280_e12744: f64 = (assign18280_e12742 * 1000000.0);
        let assign18280_e12746: f64 = (assign18280_e12744 + locals.var_uc_rdict1);
        let assign18280_e12747: f64 = (locals.var_rdtemp0 * assign18280_e12746);
        let assign18280_e12750: f64 = (p.p68 * p.p100);
        let assign18280_e12752: f64 = (assign18280_e12750 * 1000000.0);
        let assign18280_e12754: f64 = (assign18280_e12752 + p.p101);
        let assign18280_e12755: f64 = (assign18280_e12747 * assign18280_e12754);
        (assign18280_e12755, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign18280_e12757;
        locals.var_t2_dn0 = assign18280_e12757_d_n0;
        locals.var_t2_dn2 = assign18280_e12757_d_n2;
        locals.var_t2_dn4 = assign18280_e12757_d_n4;
        locals.var_t2_dn5 = assign18280_e12757_d_n5;
        locals.var_t2_dn6 = assign18280_e12757_d_n6;
        locals.var_t2_dn7 = assign18280_e12757_d_n7;
        locals.var_t2_dn8 = assign18280_e12757_d_n8;
        locals.var_t2_dn9 = assign18280_e12757_d_n9;
        locals.var_t2_dn10 = assign18280_e12757_d_n10;
        locals.var_t2_dn13 = assign18280_e12757_d_n13;

        let assign18290_e12760: f64 = if p.p39 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard380 = assign18290_e12760;

        let (assign18300_e12780, assign18300_e12780_d_n0, assign18300_e12780_d_n2, assign18300_e12780_d_n4, assign18300_e12780_d_n5, assign18300_e12780_d_n6, assign18300_e12780_d_n7, assign18300_e12780_d_n8, assign18300_e12780_d_n9, assign18300_e12780_d_n10, assign18300_e12780_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard379 != 0.0)) && (locals.var_guard380 != 0.0)) {
        let assign18300_e12771: f64 = (locals.var_mks_rdtemp1 * locals.var_tdiff0);
        let assign18300_e12772: f64 = (locals.var_uc_rd + assign18300_e12771);
        let assign18300_e12775: f64 = (locals.var_mks_rdtemp2 * locals.var_tdiff0_2);
        let assign18300_e12776: f64 = (assign18300_e12772 + assign18300_e12775);
        let assign18300_e12778: f64 = (assign18300_e12776 * locals.var_t2);
        (assign18300_e12778, ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn0) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn0)) * locals.var_t2) + (assign18300_e12776 * locals.var_t2_dn0)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn2) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn2)) * locals.var_t2) + (assign18300_e12776 * locals.var_t2_dn2)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn4) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn4)) * locals.var_t2) + (assign18300_e12776 * locals.var_t2_dn4)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn5) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn5)) * locals.var_t2) + (assign18300_e12776 * locals.var_t2_dn5)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn6) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn6)) * locals.var_t2) + (assign18300_e12776 * locals.var_t2_dn6)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn7) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn7)) * locals.var_t2) + (assign18300_e12776 * locals.var_t2_dn7)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn8) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn8)) * locals.var_t2) + (assign18300_e12776 * locals.var_t2_dn8)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn9) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn9)) * locals.var_t2) + (assign18300_e12776 * locals.var_t2_dn9)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn10) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn10)) * locals.var_t2) + (assign18300_e12776 * locals.var_t2_dn10)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn13) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn13)) * locals.var_t2) + (assign18300_e12776 * locals.var_t2_dn13)),)
    } else {
        (locals.var_rde, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn13,)
    }
};
        locals.var_rde = assign18300_e12780;
        locals.var_rde_dn0 = assign18300_e12780_d_n0;
        locals.var_rde_dn2 = assign18300_e12780_d_n2;
        locals.var_rde_dn4 = assign18300_e12780_d_n4;
        locals.var_rde_dn5 = assign18300_e12780_d_n5;
        locals.var_rde_dn6 = assign18300_e12780_d_n6;
        locals.var_rde_dn7 = assign18300_e12780_d_n7;
        locals.var_rde_dn8 = assign18300_e12780_d_n8;
        locals.var_rde_dn9 = assign18300_e12780_d_n9;
        locals.var_rde_dn10 = assign18300_e12780_d_n10;
        locals.var_rde_dn13 = assign18300_e12780_d_n13;

        let (assign18310_e12798, assign18310_e12798_d_n0, assign18310_e12798_d_n2, assign18310_e12798_d_n4, assign18310_e12798_d_n5, assign18310_e12798_d_n6, assign18310_e12798_d_n7, assign18310_e12798_d_n8, assign18310_e12798_d_n9, assign18310_e12798_d_n10, assign18310_e12798_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard379 != 0.0)) && (locals.var_guard380 != 0.0)) {
        let assign18310_e12791: f64 = (0.005 * locals.var_uc_rd);
        let assign18310_e12792: f64 = (locals.var_rde - assign18310_e12791);
        let assign18310_e12795: f64 = (0.01 * locals.var_uc_rd);
        let assign18310_e12796: f64 = (assign18310_e12792 - assign18310_e12795);
        (assign18310_e12796, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign18310_e12798;
        locals.var_tmf1_dn0 = assign18310_e12798_d_n0;
        locals.var_tmf1_dn2 = assign18310_e12798_d_n2;
        locals.var_tmf1_dn4 = assign18310_e12798_d_n4;
        locals.var_tmf1_dn5 = assign18310_e12798_d_n5;
        locals.var_tmf1_dn6 = assign18310_e12798_d_n6;
        locals.var_tmf1_dn7 = assign18310_e12798_d_n7;
        locals.var_tmf1_dn8 = assign18310_e12798_d_n8;
        locals.var_tmf1_dn9 = assign18310_e12798_d_n9;
        locals.var_tmf1_dn10 = assign18310_e12798_d_n10;
        locals.var_tmf1_dn13 = assign18310_e12798_d_n13;

        let (assign18320_e12816, assign18320_e12816_d_n0, assign18320_e12816_d_n2, assign18320_e12816_d_n4, assign18320_e12816_d_n5, assign18320_e12816_d_n6, assign18320_e12816_d_n7, assign18320_e12816_d_n8, assign18320_e12816_d_n9, assign18320_e12816_d_n10, assign18320_e12816_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard379 != 0.0)) && (locals.var_guard380 != 0.0)) {
        let assign18320_e12809: f64 = (0.005 * locals.var_uc_rd);
        let assign18320_e12810: f64 = (4.0 * assign18320_e12809);
        let assign18320_e12813: f64 = (0.01 * locals.var_uc_rd);
        let assign18320_e12814: f64 = (assign18320_e12810 * assign18320_e12813);
        (assign18320_e12814, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18320_e12816;
        locals.var_tmf2_dn0 = assign18320_e12816_d_n0;
        locals.var_tmf2_dn2 = assign18320_e12816_d_n2;
        locals.var_tmf2_dn4 = assign18320_e12816_d_n4;
        locals.var_tmf2_dn5 = assign18320_e12816_d_n5;
        locals.var_tmf2_dn6 = assign18320_e12816_d_n6;
        locals.var_tmf2_dn7 = assign18320_e12816_d_n7;
        locals.var_tmf2_dn8 = assign18320_e12816_d_n8;
        locals.var_tmf2_dn9 = assign18320_e12816_d_n9;
        locals.var_tmf2_dn10 = assign18320_e12816_d_n10;
        locals.var_tmf2_dn13 = assign18320_e12816_d_n13;

        let (assign18330_e12832, assign18330_e12832_d_n0, assign18330_e12832_d_n2, assign18330_e12832_d_n4, assign18330_e12832_d_n5, assign18330_e12832_d_n6, assign18330_e12832_d_n7, assign18330_e12832_d_n8, assign18330_e12832_d_n9, assign18330_e12832_d_n10, assign18330_e12832_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard379 != 0.0)) && (locals.var_guard380 != 0.0)) {
        let (assign18330_e12830, assign18330_e12830_d_n0, assign18330_e12830_d_n2, assign18330_e12830_d_n4, assign18330_e12830_d_n5, assign18330_e12830_d_n6, assign18330_e12830_d_n7, assign18330_e12830_d_n8, assign18330_e12830_d_n9, assign18330_e12830_d_n10, assign18330_e12830_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign18330_e12829: f64 = (-locals.var_tmf2);
                (assign18330_e12829, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign18330_e12830, assign18330_e12830_d_n0, assign18330_e12830_d_n2, assign18330_e12830_d_n4, assign18330_e12830_d_n5, assign18330_e12830_d_n6, assign18330_e12830_d_n7, assign18330_e12830_d_n8, assign18330_e12830_d_n9, assign18330_e12830_d_n10, assign18330_e12830_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18330_e12832;
        locals.var_tmf2_dn0 = assign18330_e12832_d_n0;
        locals.var_tmf2_dn2 = assign18330_e12832_d_n2;
        locals.var_tmf2_dn4 = assign18330_e12832_d_n4;
        locals.var_tmf2_dn5 = assign18330_e12832_d_n5;
        locals.var_tmf2_dn6 = assign18330_e12832_d_n6;
        locals.var_tmf2_dn7 = assign18330_e12832_d_n7;
        locals.var_tmf2_dn8 = assign18330_e12832_d_n8;
        locals.var_tmf2_dn9 = assign18330_e12832_d_n9;
        locals.var_tmf2_dn10 = assign18330_e12832_d_n10;
        locals.var_tmf2_dn13 = assign18330_e12832_d_n13;

        let (assign18340_e12847, assign18340_e12847_d_n0, assign18340_e12847_d_n2, assign18340_e12847_d_n4, assign18340_e12847_d_n5, assign18340_e12847_d_n6, assign18340_e12847_d_n7, assign18340_e12847_d_n8, assign18340_e12847_d_n9, assign18340_e12847_d_n10, assign18340_e12847_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard379 != 0.0)) && (locals.var_guard380 != 0.0)) {
        let assign18340_e12842: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18340_e12844: f64 = (assign18340_e12842 + locals.var_tmf2);
        let assign18340_e12845: f64 = (assign18340_e12844).sqrt();
        (assign18340_e12845, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18340_e12845)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18340_e12845)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign18340_e12845)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign18340_e12845)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign18340_e12845)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign18340_e12845)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign18340_e12845)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign18340_e12845)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign18340_e12845)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign18340_e12845)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18340_e12847;
        locals.var_tmf2_dn0 = assign18340_e12847_d_n0;
        locals.var_tmf2_dn2 = assign18340_e12847_d_n2;
        locals.var_tmf2_dn4 = assign18340_e12847_d_n4;
        locals.var_tmf2_dn5 = assign18340_e12847_d_n5;
        locals.var_tmf2_dn6 = assign18340_e12847_d_n6;
        locals.var_tmf2_dn7 = assign18340_e12847_d_n7;
        locals.var_tmf2_dn8 = assign18340_e12847_d_n8;
        locals.var_tmf2_dn9 = assign18340_e12847_d_n9;
        locals.var_tmf2_dn10 = assign18340_e12847_d_n10;
        locals.var_tmf2_dn13 = assign18340_e12847_d_n13;

        let (assign18350_e12863, assign18350_e12863_d_n0, assign18350_e12863_d_n2, assign18350_e12863_d_n4, assign18350_e12863_d_n5, assign18350_e12863_d_n6, assign18350_e12863_d_n7, assign18350_e12863_d_n8, assign18350_e12863_d_n9, assign18350_e12863_d_n10, assign18350_e12863_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard379 != 0.0)) && (locals.var_guard380 != 0.0)) {
        let assign18350_e12859: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign18350_e12860: f64 = (1.0 + assign18350_e12859);
        let assign18350_e12861: f64 = (0.5 * assign18350_e12860);
        (assign18350_e12861, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign18350_e12863;
        locals.var_t0_dn0 = assign18350_e12863_d_n0;
        locals.var_t0_dn2 = assign18350_e12863_d_n2;
        locals.var_t0_dn4 = assign18350_e12863_d_n4;
        locals.var_t0_dn5 = assign18350_e12863_d_n5;
        locals.var_t0_dn6 = assign18350_e12863_d_n6;
        locals.var_t0_dn7 = assign18350_e12863_d_n7;
        locals.var_t0_dn8 = assign18350_e12863_d_n8;
        locals.var_t0_dn9 = assign18350_e12863_d_n9;
        locals.var_t0_dn10 = assign18350_e12863_d_n10;
        locals.var_t0_dn13 = assign18350_e12863_d_n13;

        let (assign18360_e12881, assign18360_e12881_d_n0, assign18360_e12881_d_n2, assign18360_e12881_d_n4, assign18360_e12881_d_n5, assign18360_e12881_d_n6, assign18360_e12881_d_n7, assign18360_e12881_d_n8, assign18360_e12881_d_n9, assign18360_e12881_d_n10, assign18360_e12881_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard379 != 0.0)) && (locals.var_guard380 != 0.0)) {
        let assign18360_e12873: f64 = (0.005 * locals.var_uc_rd);
        let assign18360_e12877: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign18360_e12878: f64 = (0.5 * assign18360_e12877);
        let assign18360_e12879: f64 = (assign18360_e12873 + assign18360_e12878);
        (assign18360_e12879, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_rde, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn13,)
    }
};
        locals.var_rde = assign18360_e12881;
        locals.var_rde_dn0 = assign18360_e12881_d_n0;
        locals.var_rde_dn2 = assign18360_e12881_d_n2;
        locals.var_rde_dn4 = assign18360_e12881_d_n4;
        locals.var_rde_dn5 = assign18360_e12881_d_n5;
        locals.var_rde_dn6 = assign18360_e12881_d_n6;
        locals.var_rde_dn7 = assign18360_e12881_d_n7;
        locals.var_rde_dn8 = assign18360_e12881_d_n8;
        locals.var_rde_dn9 = assign18360_e12881_d_n9;
        locals.var_rde_dn10 = assign18360_e12881_d_n10;
        locals.var_rde_dn13 = assign18360_e12881_d_n13;

        let (assign18370_e12902, assign18370_e12902_d_n0, assign18370_e12902_d_n2, assign18370_e12902_d_n4, assign18370_e12902_d_n5, assign18370_e12902_d_n6, assign18370_e12902_d_n7, assign18370_e12902_d_n8, assign18370_e12902_d_n9, assign18370_e12902_d_n10, assign18370_e12902_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard379 != 0.0)) && (locals.var_guard380 == 0.0)) {
        let assign18370_e12893: f64 = (locals.var_mks_rdtemp1 * locals.var_tdiff);
        let assign18370_e12894: f64 = (locals.var_uc_rd + assign18370_e12893);
        let assign18370_e12897: f64 = (locals.var_mks_rdtemp2 * locals.var_tdiff_2);
        let assign18370_e12898: f64 = (assign18370_e12894 + assign18370_e12897);
        let assign18370_e12900: f64 = (assign18370_e12898 * locals.var_t2);
        (assign18370_e12900, ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn0) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn0)) * locals.var_t2) + (assign18370_e12898 * locals.var_t2_dn0)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn2) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn2)) * locals.var_t2) + (assign18370_e12898 * locals.var_t2_dn2)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn4) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn4)) * locals.var_t2) + (assign18370_e12898 * locals.var_t2_dn4)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn5) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn5)) * locals.var_t2) + (assign18370_e12898 * locals.var_t2_dn5)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn6) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn6)) * locals.var_t2) + (assign18370_e12898 * locals.var_t2_dn6)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn7) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn7)) * locals.var_t2) + (assign18370_e12898 * locals.var_t2_dn7)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn8) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn8)) * locals.var_t2) + (assign18370_e12898 * locals.var_t2_dn8)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn9) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn9)) * locals.var_t2) + (assign18370_e12898 * locals.var_t2_dn9)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn10) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn10)) * locals.var_t2) + (assign18370_e12898 * locals.var_t2_dn10)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn13) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn13)) * locals.var_t2) + (assign18370_e12898 * locals.var_t2_dn13)),)
    } else {
        (locals.var_rde, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn13,)
    }
};
        locals.var_rde = assign18370_e12902;
        locals.var_rde_dn0 = assign18370_e12902_d_n0;
        locals.var_rde_dn2 = assign18370_e12902_d_n2;
        locals.var_rde_dn4 = assign18370_e12902_d_n4;
        locals.var_rde_dn5 = assign18370_e12902_d_n5;
        locals.var_rde_dn6 = assign18370_e12902_d_n6;
        locals.var_rde_dn7 = assign18370_e12902_d_n7;
        locals.var_rde_dn8 = assign18370_e12902_d_n8;
        locals.var_rde_dn9 = assign18370_e12902_d_n9;
        locals.var_rde_dn10 = assign18370_e12902_d_n10;
        locals.var_rde_dn13 = assign18370_e12902_d_n13;

        let (assign18380_e12921, assign18380_e12921_d_n0, assign18380_e12921_d_n2, assign18380_e12921_d_n4, assign18380_e12921_d_n5, assign18380_e12921_d_n6, assign18380_e12921_d_n7, assign18380_e12921_d_n8, assign18380_e12921_d_n9, assign18380_e12921_d_n10, assign18380_e12921_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard379 != 0.0)) && (locals.var_guard380 == 0.0)) {
        let assign18380_e12914: f64 = (0.005 * locals.var_uc_rd);
        let assign18380_e12915: f64 = (locals.var_rde - assign18380_e12914);
        let assign18380_e12918: f64 = (0.01 * locals.var_uc_rd);
        let assign18380_e12919: f64 = (assign18380_e12915 - assign18380_e12918);
        (assign18380_e12919, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign18380_e12921;
        locals.var_tmf1_dn0 = assign18380_e12921_d_n0;
        locals.var_tmf1_dn2 = assign18380_e12921_d_n2;
        locals.var_tmf1_dn4 = assign18380_e12921_d_n4;
        locals.var_tmf1_dn5 = assign18380_e12921_d_n5;
        locals.var_tmf1_dn6 = assign18380_e12921_d_n6;
        locals.var_tmf1_dn7 = assign18380_e12921_d_n7;
        locals.var_tmf1_dn8 = assign18380_e12921_d_n8;
        locals.var_tmf1_dn9 = assign18380_e12921_d_n9;
        locals.var_tmf1_dn10 = assign18380_e12921_d_n10;
        locals.var_tmf1_dn13 = assign18380_e12921_d_n13;

        let (assign18390_e12940, assign18390_e12940_d_n0, assign18390_e12940_d_n2, assign18390_e12940_d_n4, assign18390_e12940_d_n5, assign18390_e12940_d_n6, assign18390_e12940_d_n7, assign18390_e12940_d_n8, assign18390_e12940_d_n9, assign18390_e12940_d_n10, assign18390_e12940_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard379 != 0.0)) && (locals.var_guard380 == 0.0)) {
        let assign18390_e12933: f64 = (0.005 * locals.var_uc_rd);
        let assign18390_e12934: f64 = (4.0 * assign18390_e12933);
        let assign18390_e12937: f64 = (0.01 * locals.var_uc_rd);
        let assign18390_e12938: f64 = (assign18390_e12934 * assign18390_e12937);
        (assign18390_e12938, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18390_e12940;
        locals.var_tmf2_dn0 = assign18390_e12940_d_n0;
        locals.var_tmf2_dn2 = assign18390_e12940_d_n2;
        locals.var_tmf2_dn4 = assign18390_e12940_d_n4;
        locals.var_tmf2_dn5 = assign18390_e12940_d_n5;
        locals.var_tmf2_dn6 = assign18390_e12940_d_n6;
        locals.var_tmf2_dn7 = assign18390_e12940_d_n7;
        locals.var_tmf2_dn8 = assign18390_e12940_d_n8;
        locals.var_tmf2_dn9 = assign18390_e12940_d_n9;
        locals.var_tmf2_dn10 = assign18390_e12940_d_n10;
        locals.var_tmf2_dn13 = assign18390_e12940_d_n13;

        let (assign18400_e12957, assign18400_e12957_d_n0, assign18400_e12957_d_n2, assign18400_e12957_d_n4, assign18400_e12957_d_n5, assign18400_e12957_d_n6, assign18400_e12957_d_n7, assign18400_e12957_d_n8, assign18400_e12957_d_n9, assign18400_e12957_d_n10, assign18400_e12957_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard379 != 0.0)) && (locals.var_guard380 == 0.0)) {
        let (assign18400_e12955, assign18400_e12955_d_n0, assign18400_e12955_d_n2, assign18400_e12955_d_n4, assign18400_e12955_d_n5, assign18400_e12955_d_n6, assign18400_e12955_d_n7, assign18400_e12955_d_n8, assign18400_e12955_d_n9, assign18400_e12955_d_n10, assign18400_e12955_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign18400_e12954: f64 = (-locals.var_tmf2);
                (assign18400_e12954, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign18400_e12955, assign18400_e12955_d_n0, assign18400_e12955_d_n2, assign18400_e12955_d_n4, assign18400_e12955_d_n5, assign18400_e12955_d_n6, assign18400_e12955_d_n7, assign18400_e12955_d_n8, assign18400_e12955_d_n9, assign18400_e12955_d_n10, assign18400_e12955_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18400_e12957;
        locals.var_tmf2_dn0 = assign18400_e12957_d_n0;
        locals.var_tmf2_dn2 = assign18400_e12957_d_n2;
        locals.var_tmf2_dn4 = assign18400_e12957_d_n4;
        locals.var_tmf2_dn5 = assign18400_e12957_d_n5;
        locals.var_tmf2_dn6 = assign18400_e12957_d_n6;
        locals.var_tmf2_dn7 = assign18400_e12957_d_n7;
        locals.var_tmf2_dn8 = assign18400_e12957_d_n8;
        locals.var_tmf2_dn9 = assign18400_e12957_d_n9;
        locals.var_tmf2_dn10 = assign18400_e12957_d_n10;
        locals.var_tmf2_dn13 = assign18400_e12957_d_n13;

        let (assign18410_e12973, assign18410_e12973_d_n0, assign18410_e12973_d_n2, assign18410_e12973_d_n4, assign18410_e12973_d_n5, assign18410_e12973_d_n6, assign18410_e12973_d_n7, assign18410_e12973_d_n8, assign18410_e12973_d_n9, assign18410_e12973_d_n10, assign18410_e12973_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard379 != 0.0)) && (locals.var_guard380 == 0.0)) {
        let assign18410_e12968: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18410_e12970: f64 = (assign18410_e12968 + locals.var_tmf2);
        let assign18410_e12971: f64 = (assign18410_e12970).sqrt();
        (assign18410_e12971, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18410_e12971)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18410_e12971)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign18410_e12971)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign18410_e12971)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign18410_e12971)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign18410_e12971)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign18410_e12971)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign18410_e12971)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign18410_e12971)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign18410_e12971)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18410_e12973;
        locals.var_tmf2_dn0 = assign18410_e12973_d_n0;
        locals.var_tmf2_dn2 = assign18410_e12973_d_n2;
        locals.var_tmf2_dn4 = assign18410_e12973_d_n4;
        locals.var_tmf2_dn5 = assign18410_e12973_d_n5;
        locals.var_tmf2_dn6 = assign18410_e12973_d_n6;
        locals.var_tmf2_dn7 = assign18410_e12973_d_n7;
        locals.var_tmf2_dn8 = assign18410_e12973_d_n8;
        locals.var_tmf2_dn9 = assign18410_e12973_d_n9;
        locals.var_tmf2_dn10 = assign18410_e12973_d_n10;
        locals.var_tmf2_dn13 = assign18410_e12973_d_n13;

        let (assign18420_e12990, assign18420_e12990_d_n0, assign18420_e12990_d_n2, assign18420_e12990_d_n4, assign18420_e12990_d_n5, assign18420_e12990_d_n6, assign18420_e12990_d_n7, assign18420_e12990_d_n8, assign18420_e12990_d_n9, assign18420_e12990_d_n10, assign18420_e12990_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard379 != 0.0)) && (locals.var_guard380 == 0.0)) {
        let assign18420_e12986: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign18420_e12987: f64 = (1.0 + assign18420_e12986);
        let assign18420_e12988: f64 = (0.5 * assign18420_e12987);
        (assign18420_e12988, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign18420_e12990;
        locals.var_t0_dn0 = assign18420_e12990_d_n0;
        locals.var_t0_dn2 = assign18420_e12990_d_n2;
        locals.var_t0_dn4 = assign18420_e12990_d_n4;
        locals.var_t0_dn5 = assign18420_e12990_d_n5;
        locals.var_t0_dn6 = assign18420_e12990_d_n6;
        locals.var_t0_dn7 = assign18420_e12990_d_n7;
        locals.var_t0_dn8 = assign18420_e12990_d_n8;
        locals.var_t0_dn9 = assign18420_e12990_d_n9;
        locals.var_t0_dn10 = assign18420_e12990_d_n10;
        locals.var_t0_dn13 = assign18420_e12990_d_n13;

        let (assign18430_e13009, assign18430_e13009_d_n0, assign18430_e13009_d_n2, assign18430_e13009_d_n4, assign18430_e13009_d_n5, assign18430_e13009_d_n6, assign18430_e13009_d_n7, assign18430_e13009_d_n8, assign18430_e13009_d_n9, assign18430_e13009_d_n10, assign18430_e13009_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard379 != 0.0)) && (locals.var_guard380 == 0.0)) {
        let assign18430_e13001: f64 = (0.005 * locals.var_uc_rd);
        let assign18430_e13005: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign18430_e13006: f64 = (0.5 * assign18430_e13005);
        let assign18430_e13007: f64 = (assign18430_e13001 + assign18430_e13006);
        (assign18430_e13007, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_rde, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn13,)
    }
};
        locals.var_rde = assign18430_e13009;
        locals.var_rde_dn0 = assign18430_e13009_d_n0;
        locals.var_rde_dn2 = assign18430_e13009_d_n2;
        locals.var_rde_dn4 = assign18430_e13009_d_n4;
        locals.var_rde_dn5 = assign18430_e13009_d_n5;
        locals.var_rde_dn6 = assign18430_e13009_d_n6;
        locals.var_rde_dn7 = assign18430_e13009_d_n7;
        locals.var_rde_dn8 = assign18430_e13009_d_n8;
        locals.var_rde_dn9 = assign18430_e13009_d_n9;
        locals.var_rde_dn10 = assign18430_e13009_d_n10;
        locals.var_rde_dn13 = assign18430_e13009_d_n13;

        let (assign18440_e13018, assign18440_e13018_d_n0, assign18440_e13018_d_n2, assign18440_e13018_d_n4, assign18440_e13018_d_n5, assign18440_e13018_d_n6, assign18440_e13018_d_n7, assign18440_e13018_d_n8, assign18440_e13018_d_n9, assign18440_e13018_d_n10, assign18440_e13018_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard379 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rde, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn13,)
    }
};
        locals.var_rde = assign18440_e13018;
        locals.var_rde_dn0 = assign18440_e13018_d_n0;
        locals.var_rde_dn2 = assign18440_e13018_d_n2;
        locals.var_rde_dn4 = assign18440_e13018_d_n4;
        locals.var_rde_dn5 = assign18440_e13018_d_n5;
        locals.var_rde_dn6 = assign18440_e13018_d_n6;
        locals.var_rde_dn7 = assign18440_e13018_d_n7;
        locals.var_rde_dn8 = assign18440_e13018_d_n8;
        locals.var_rde_dn9 = assign18440_e13018_d_n9;
        locals.var_rde_dn10 = assign18440_e13018_d_n10;
        locals.var_rde_dn13 = assign18440_e13018_d_n13;

        let assign18450_e13021: f64 = if locals.var_uc_rs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard381 = assign18450_e13021;

        let (assign18460_e13045, assign18460_e13045_d_n0, assign18460_e13045_d_n2, assign18460_e13045_d_n4, assign18460_e13045_d_n5, assign18460_e13045_d_n6, assign18460_e13045_d_n7, assign18460_e13045_d_n8, assign18460_e13045_d_n9, assign18460_e13045_d_n10, assign18460_e13045_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard381 != 0.0)) {
        let assign18460_e13030: f64 = (p.p69 * locals.var_uc_rdslp1);
        let assign18460_e13032: f64 = (assign18460_e13030 * 1000000.0);
        let assign18460_e13034: f64 = (assign18460_e13032 + locals.var_uc_rdict1);
        let assign18460_e13035: f64 = (locals.var_rdtemp0 * assign18460_e13034);
        let assign18460_e13038: f64 = (p.p70 * p.p100);
        let assign18460_e13040: f64 = (assign18460_e13038 * 1000000.0);
        let assign18460_e13042: f64 = (assign18460_e13040 + p.p101);
        let assign18460_e13043: f64 = (assign18460_e13035 * assign18460_e13042);
        (assign18460_e13043, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign18460_e13045;
        locals.var_t2_dn0 = assign18460_e13045_d_n0;
        locals.var_t2_dn2 = assign18460_e13045_d_n2;
        locals.var_t2_dn4 = assign18460_e13045_d_n4;
        locals.var_t2_dn5 = assign18460_e13045_d_n5;
        locals.var_t2_dn6 = assign18460_e13045_d_n6;
        locals.var_t2_dn7 = assign18460_e13045_d_n7;
        locals.var_t2_dn8 = assign18460_e13045_d_n8;
        locals.var_t2_dn9 = assign18460_e13045_d_n9;
        locals.var_t2_dn10 = assign18460_e13045_d_n10;
        locals.var_t2_dn13 = assign18460_e13045_d_n13;

    }

    pub(super) fn stamp_transient_block_40(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign18470_e13048: f64 = if p.p39 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard382 = assign18470_e13048;

        let (assign18480_e13068, assign18480_e13068_d_n0, assign18480_e13068_d_n2, assign18480_e13068_d_n4, assign18480_e13068_d_n5, assign18480_e13068_d_n6, assign18480_e13068_d_n7, assign18480_e13068_d_n8, assign18480_e13068_d_n9, assign18480_e13068_d_n10, assign18480_e13068_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard381 != 0.0)) && (locals.var_guard382 != 0.0)) {
        let assign18480_e13059: f64 = (locals.var_mks_rdtemp1 * locals.var_tdiff0);
        let assign18480_e13060: f64 = (locals.var_uc_rs + assign18480_e13059);
        let assign18480_e13063: f64 = (locals.var_mks_rdtemp2 * locals.var_tdiff0_2);
        let assign18480_e13064: f64 = (assign18480_e13060 + assign18480_e13063);
        let assign18480_e13066: f64 = (assign18480_e13064 * locals.var_t2);
        (assign18480_e13066, ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn0) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn0)) * locals.var_t2) + (assign18480_e13064 * locals.var_t2_dn0)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn2) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn2)) * locals.var_t2) + (assign18480_e13064 * locals.var_t2_dn2)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn4) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn4)) * locals.var_t2) + (assign18480_e13064 * locals.var_t2_dn4)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn5) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn5)) * locals.var_t2) + (assign18480_e13064 * locals.var_t2_dn5)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn6) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn6)) * locals.var_t2) + (assign18480_e13064 * locals.var_t2_dn6)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn7) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn7)) * locals.var_t2) + (assign18480_e13064 * locals.var_t2_dn7)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn8) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn8)) * locals.var_t2) + (assign18480_e13064 * locals.var_t2_dn8)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn9) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn9)) * locals.var_t2) + (assign18480_e13064 * locals.var_t2_dn9)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn10) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn10)) * locals.var_t2) + (assign18480_e13064 * locals.var_t2_dn10)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn13) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn13)) * locals.var_t2) + (assign18480_e13064 * locals.var_t2_dn13)),)
    } else {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn13,)
    }
};
        locals.var_rse = assign18480_e13068;
        locals.var_rse_dn0 = assign18480_e13068_d_n0;
        locals.var_rse_dn2 = assign18480_e13068_d_n2;
        locals.var_rse_dn4 = assign18480_e13068_d_n4;
        locals.var_rse_dn5 = assign18480_e13068_d_n5;
        locals.var_rse_dn6 = assign18480_e13068_d_n6;
        locals.var_rse_dn7 = assign18480_e13068_d_n7;
        locals.var_rse_dn8 = assign18480_e13068_d_n8;
        locals.var_rse_dn9 = assign18480_e13068_d_n9;
        locals.var_rse_dn10 = assign18480_e13068_d_n10;
        locals.var_rse_dn13 = assign18480_e13068_d_n13;

        let (assign18490_e13086, assign18490_e13086_d_n0, assign18490_e13086_d_n2, assign18490_e13086_d_n4, assign18490_e13086_d_n5, assign18490_e13086_d_n6, assign18490_e13086_d_n7, assign18490_e13086_d_n8, assign18490_e13086_d_n9, assign18490_e13086_d_n10, assign18490_e13086_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard381 != 0.0)) && (locals.var_guard382 != 0.0)) {
        let assign18490_e13079: f64 = (0.005 * locals.var_uc_rs);
        let assign18490_e13080: f64 = (locals.var_rse - assign18490_e13079);
        let assign18490_e13083: f64 = (0.01 * locals.var_uc_rs);
        let assign18490_e13084: f64 = (assign18490_e13080 - assign18490_e13083);
        (assign18490_e13084, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign18490_e13086;
        locals.var_tmf1_dn0 = assign18490_e13086_d_n0;
        locals.var_tmf1_dn2 = assign18490_e13086_d_n2;
        locals.var_tmf1_dn4 = assign18490_e13086_d_n4;
        locals.var_tmf1_dn5 = assign18490_e13086_d_n5;
        locals.var_tmf1_dn6 = assign18490_e13086_d_n6;
        locals.var_tmf1_dn7 = assign18490_e13086_d_n7;
        locals.var_tmf1_dn8 = assign18490_e13086_d_n8;
        locals.var_tmf1_dn9 = assign18490_e13086_d_n9;
        locals.var_tmf1_dn10 = assign18490_e13086_d_n10;
        locals.var_tmf1_dn13 = assign18490_e13086_d_n13;

        let (assign18500_e13104, assign18500_e13104_d_n0, assign18500_e13104_d_n2, assign18500_e13104_d_n4, assign18500_e13104_d_n5, assign18500_e13104_d_n6, assign18500_e13104_d_n7, assign18500_e13104_d_n8, assign18500_e13104_d_n9, assign18500_e13104_d_n10, assign18500_e13104_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard381 != 0.0)) && (locals.var_guard382 != 0.0)) {
        let assign18500_e13097: f64 = (0.005 * locals.var_uc_rs);
        let assign18500_e13098: f64 = (4.0 * assign18500_e13097);
        let assign18500_e13101: f64 = (0.01 * locals.var_uc_rs);
        let assign18500_e13102: f64 = (assign18500_e13098 * assign18500_e13101);
        (assign18500_e13102, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18500_e13104;
        locals.var_tmf2_dn0 = assign18500_e13104_d_n0;
        locals.var_tmf2_dn2 = assign18500_e13104_d_n2;
        locals.var_tmf2_dn4 = assign18500_e13104_d_n4;
        locals.var_tmf2_dn5 = assign18500_e13104_d_n5;
        locals.var_tmf2_dn6 = assign18500_e13104_d_n6;
        locals.var_tmf2_dn7 = assign18500_e13104_d_n7;
        locals.var_tmf2_dn8 = assign18500_e13104_d_n8;
        locals.var_tmf2_dn9 = assign18500_e13104_d_n9;
        locals.var_tmf2_dn10 = assign18500_e13104_d_n10;
        locals.var_tmf2_dn13 = assign18500_e13104_d_n13;

        let (assign18510_e13120, assign18510_e13120_d_n0, assign18510_e13120_d_n2, assign18510_e13120_d_n4, assign18510_e13120_d_n5, assign18510_e13120_d_n6, assign18510_e13120_d_n7, assign18510_e13120_d_n8, assign18510_e13120_d_n9, assign18510_e13120_d_n10, assign18510_e13120_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard381 != 0.0)) && (locals.var_guard382 != 0.0)) {
        let (assign18510_e13118, assign18510_e13118_d_n0, assign18510_e13118_d_n2, assign18510_e13118_d_n4, assign18510_e13118_d_n5, assign18510_e13118_d_n6, assign18510_e13118_d_n7, assign18510_e13118_d_n8, assign18510_e13118_d_n9, assign18510_e13118_d_n10, assign18510_e13118_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign18510_e13117: f64 = (-locals.var_tmf2);
                (assign18510_e13117, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign18510_e13118, assign18510_e13118_d_n0, assign18510_e13118_d_n2, assign18510_e13118_d_n4, assign18510_e13118_d_n5, assign18510_e13118_d_n6, assign18510_e13118_d_n7, assign18510_e13118_d_n8, assign18510_e13118_d_n9, assign18510_e13118_d_n10, assign18510_e13118_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18510_e13120;
        locals.var_tmf2_dn0 = assign18510_e13120_d_n0;
        locals.var_tmf2_dn2 = assign18510_e13120_d_n2;
        locals.var_tmf2_dn4 = assign18510_e13120_d_n4;
        locals.var_tmf2_dn5 = assign18510_e13120_d_n5;
        locals.var_tmf2_dn6 = assign18510_e13120_d_n6;
        locals.var_tmf2_dn7 = assign18510_e13120_d_n7;
        locals.var_tmf2_dn8 = assign18510_e13120_d_n8;
        locals.var_tmf2_dn9 = assign18510_e13120_d_n9;
        locals.var_tmf2_dn10 = assign18510_e13120_d_n10;
        locals.var_tmf2_dn13 = assign18510_e13120_d_n13;

        let (assign18520_e13135, assign18520_e13135_d_n0, assign18520_e13135_d_n2, assign18520_e13135_d_n4, assign18520_e13135_d_n5, assign18520_e13135_d_n6, assign18520_e13135_d_n7, assign18520_e13135_d_n8, assign18520_e13135_d_n9, assign18520_e13135_d_n10, assign18520_e13135_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard381 != 0.0)) && (locals.var_guard382 != 0.0)) {
        let assign18520_e13130: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18520_e13132: f64 = (assign18520_e13130 + locals.var_tmf2);
        let assign18520_e13133: f64 = (assign18520_e13132).sqrt();
        (assign18520_e13133, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18520_e13133)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18520_e13133)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign18520_e13133)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign18520_e13133)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign18520_e13133)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign18520_e13133)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign18520_e13133)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign18520_e13133)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign18520_e13133)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign18520_e13133)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18520_e13135;
        locals.var_tmf2_dn0 = assign18520_e13135_d_n0;
        locals.var_tmf2_dn2 = assign18520_e13135_d_n2;
        locals.var_tmf2_dn4 = assign18520_e13135_d_n4;
        locals.var_tmf2_dn5 = assign18520_e13135_d_n5;
        locals.var_tmf2_dn6 = assign18520_e13135_d_n6;
        locals.var_tmf2_dn7 = assign18520_e13135_d_n7;
        locals.var_tmf2_dn8 = assign18520_e13135_d_n8;
        locals.var_tmf2_dn9 = assign18520_e13135_d_n9;
        locals.var_tmf2_dn10 = assign18520_e13135_d_n10;
        locals.var_tmf2_dn13 = assign18520_e13135_d_n13;

        let (assign18530_e13151, assign18530_e13151_d_n0, assign18530_e13151_d_n2, assign18530_e13151_d_n4, assign18530_e13151_d_n5, assign18530_e13151_d_n6, assign18530_e13151_d_n7, assign18530_e13151_d_n8, assign18530_e13151_d_n9, assign18530_e13151_d_n10, assign18530_e13151_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard381 != 0.0)) && (locals.var_guard382 != 0.0)) {
        let assign18530_e13147: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign18530_e13148: f64 = (1.0 + assign18530_e13147);
        let assign18530_e13149: f64 = (0.5 * assign18530_e13148);
        (assign18530_e13149, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign18530_e13151;
        locals.var_t0_dn0 = assign18530_e13151_d_n0;
        locals.var_t0_dn2 = assign18530_e13151_d_n2;
        locals.var_t0_dn4 = assign18530_e13151_d_n4;
        locals.var_t0_dn5 = assign18530_e13151_d_n5;
        locals.var_t0_dn6 = assign18530_e13151_d_n6;
        locals.var_t0_dn7 = assign18530_e13151_d_n7;
        locals.var_t0_dn8 = assign18530_e13151_d_n8;
        locals.var_t0_dn9 = assign18530_e13151_d_n9;
        locals.var_t0_dn10 = assign18530_e13151_d_n10;
        locals.var_t0_dn13 = assign18530_e13151_d_n13;

        let (assign18540_e13169, assign18540_e13169_d_n0, assign18540_e13169_d_n2, assign18540_e13169_d_n4, assign18540_e13169_d_n5, assign18540_e13169_d_n6, assign18540_e13169_d_n7, assign18540_e13169_d_n8, assign18540_e13169_d_n9, assign18540_e13169_d_n10, assign18540_e13169_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard381 != 0.0)) && (locals.var_guard382 != 0.0)) {
        let assign18540_e13161: f64 = (0.005 * locals.var_uc_rs);
        let assign18540_e13165: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign18540_e13166: f64 = (0.5 * assign18540_e13165);
        let assign18540_e13167: f64 = (assign18540_e13161 + assign18540_e13166);
        (assign18540_e13167, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn13,)
    }
};
        locals.var_rse = assign18540_e13169;
        locals.var_rse_dn0 = assign18540_e13169_d_n0;
        locals.var_rse_dn2 = assign18540_e13169_d_n2;
        locals.var_rse_dn4 = assign18540_e13169_d_n4;
        locals.var_rse_dn5 = assign18540_e13169_d_n5;
        locals.var_rse_dn6 = assign18540_e13169_d_n6;
        locals.var_rse_dn7 = assign18540_e13169_d_n7;
        locals.var_rse_dn8 = assign18540_e13169_d_n8;
        locals.var_rse_dn9 = assign18540_e13169_d_n9;
        locals.var_rse_dn10 = assign18540_e13169_d_n10;
        locals.var_rse_dn13 = assign18540_e13169_d_n13;

        let (assign18550_e13190, assign18550_e13190_d_n0, assign18550_e13190_d_n2, assign18550_e13190_d_n4, assign18550_e13190_d_n5, assign18550_e13190_d_n6, assign18550_e13190_d_n7, assign18550_e13190_d_n8, assign18550_e13190_d_n9, assign18550_e13190_d_n10, assign18550_e13190_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard381 != 0.0)) && (locals.var_guard382 == 0.0)) {
        let assign18550_e13181: f64 = (locals.var_mks_rdtemp1 * locals.var_tdiff);
        let assign18550_e13182: f64 = (locals.var_uc_rs + assign18550_e13181);
        let assign18550_e13185: f64 = (locals.var_mks_rdtemp2 * locals.var_tdiff_2);
        let assign18550_e13186: f64 = (assign18550_e13182 + assign18550_e13185);
        let assign18550_e13188: f64 = (assign18550_e13186 * locals.var_t2);
        (assign18550_e13188, ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn0) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn0)) * locals.var_t2) + (assign18550_e13186 * locals.var_t2_dn0)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn2) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn2)) * locals.var_t2) + (assign18550_e13186 * locals.var_t2_dn2)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn4) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn4)) * locals.var_t2) + (assign18550_e13186 * locals.var_t2_dn4)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn5) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn5)) * locals.var_t2) + (assign18550_e13186 * locals.var_t2_dn5)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn6) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn6)) * locals.var_t2) + (assign18550_e13186 * locals.var_t2_dn6)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn7) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn7)) * locals.var_t2) + (assign18550_e13186 * locals.var_t2_dn7)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn8) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn8)) * locals.var_t2) + (assign18550_e13186 * locals.var_t2_dn8)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn9) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn9)) * locals.var_t2) + (assign18550_e13186 * locals.var_t2_dn9)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn10) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn10)) * locals.var_t2) + (assign18550_e13186 * locals.var_t2_dn10)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn13) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn13)) * locals.var_t2) + (assign18550_e13186 * locals.var_t2_dn13)),)
    } else {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn13,)
    }
};
        locals.var_rse = assign18550_e13190;
        locals.var_rse_dn0 = assign18550_e13190_d_n0;
        locals.var_rse_dn2 = assign18550_e13190_d_n2;
        locals.var_rse_dn4 = assign18550_e13190_d_n4;
        locals.var_rse_dn5 = assign18550_e13190_d_n5;
        locals.var_rse_dn6 = assign18550_e13190_d_n6;
        locals.var_rse_dn7 = assign18550_e13190_d_n7;
        locals.var_rse_dn8 = assign18550_e13190_d_n8;
        locals.var_rse_dn9 = assign18550_e13190_d_n9;
        locals.var_rse_dn10 = assign18550_e13190_d_n10;
        locals.var_rse_dn13 = assign18550_e13190_d_n13;

        let (assign18560_e13209, assign18560_e13209_d_n0, assign18560_e13209_d_n2, assign18560_e13209_d_n4, assign18560_e13209_d_n5, assign18560_e13209_d_n6, assign18560_e13209_d_n7, assign18560_e13209_d_n8, assign18560_e13209_d_n9, assign18560_e13209_d_n10, assign18560_e13209_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard381 != 0.0)) && (locals.var_guard382 == 0.0)) {
        let assign18560_e13202: f64 = (0.005 * locals.var_uc_rs);
        let assign18560_e13203: f64 = (locals.var_rse - assign18560_e13202);
        let assign18560_e13206: f64 = (0.01 * locals.var_uc_rs);
        let assign18560_e13207: f64 = (assign18560_e13203 - assign18560_e13206);
        (assign18560_e13207, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign18560_e13209;
        locals.var_tmf1_dn0 = assign18560_e13209_d_n0;
        locals.var_tmf1_dn2 = assign18560_e13209_d_n2;
        locals.var_tmf1_dn4 = assign18560_e13209_d_n4;
        locals.var_tmf1_dn5 = assign18560_e13209_d_n5;
        locals.var_tmf1_dn6 = assign18560_e13209_d_n6;
        locals.var_tmf1_dn7 = assign18560_e13209_d_n7;
        locals.var_tmf1_dn8 = assign18560_e13209_d_n8;
        locals.var_tmf1_dn9 = assign18560_e13209_d_n9;
        locals.var_tmf1_dn10 = assign18560_e13209_d_n10;
        locals.var_tmf1_dn13 = assign18560_e13209_d_n13;

        let (assign18570_e13228, assign18570_e13228_d_n0, assign18570_e13228_d_n2, assign18570_e13228_d_n4, assign18570_e13228_d_n5, assign18570_e13228_d_n6, assign18570_e13228_d_n7, assign18570_e13228_d_n8, assign18570_e13228_d_n9, assign18570_e13228_d_n10, assign18570_e13228_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard381 != 0.0)) && (locals.var_guard382 == 0.0)) {
        let assign18570_e13221: f64 = (0.005 * locals.var_uc_rs);
        let assign18570_e13222: f64 = (4.0 * assign18570_e13221);
        let assign18570_e13225: f64 = (0.01 * locals.var_uc_rs);
        let assign18570_e13226: f64 = (assign18570_e13222 * assign18570_e13225);
        (assign18570_e13226, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18570_e13228;
        locals.var_tmf2_dn0 = assign18570_e13228_d_n0;
        locals.var_tmf2_dn2 = assign18570_e13228_d_n2;
        locals.var_tmf2_dn4 = assign18570_e13228_d_n4;
        locals.var_tmf2_dn5 = assign18570_e13228_d_n5;
        locals.var_tmf2_dn6 = assign18570_e13228_d_n6;
        locals.var_tmf2_dn7 = assign18570_e13228_d_n7;
        locals.var_tmf2_dn8 = assign18570_e13228_d_n8;
        locals.var_tmf2_dn9 = assign18570_e13228_d_n9;
        locals.var_tmf2_dn10 = assign18570_e13228_d_n10;
        locals.var_tmf2_dn13 = assign18570_e13228_d_n13;

        let (assign18580_e13245, assign18580_e13245_d_n0, assign18580_e13245_d_n2, assign18580_e13245_d_n4, assign18580_e13245_d_n5, assign18580_e13245_d_n6, assign18580_e13245_d_n7, assign18580_e13245_d_n8, assign18580_e13245_d_n9, assign18580_e13245_d_n10, assign18580_e13245_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard381 != 0.0)) && (locals.var_guard382 == 0.0)) {
        let (assign18580_e13243, assign18580_e13243_d_n0, assign18580_e13243_d_n2, assign18580_e13243_d_n4, assign18580_e13243_d_n5, assign18580_e13243_d_n6, assign18580_e13243_d_n7, assign18580_e13243_d_n8, assign18580_e13243_d_n9, assign18580_e13243_d_n10, assign18580_e13243_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign18580_e13242: f64 = (-locals.var_tmf2);
                (assign18580_e13242, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign18580_e13243, assign18580_e13243_d_n0, assign18580_e13243_d_n2, assign18580_e13243_d_n4, assign18580_e13243_d_n5, assign18580_e13243_d_n6, assign18580_e13243_d_n7, assign18580_e13243_d_n8, assign18580_e13243_d_n9, assign18580_e13243_d_n10, assign18580_e13243_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18580_e13245;
        locals.var_tmf2_dn0 = assign18580_e13245_d_n0;
        locals.var_tmf2_dn2 = assign18580_e13245_d_n2;
        locals.var_tmf2_dn4 = assign18580_e13245_d_n4;
        locals.var_tmf2_dn5 = assign18580_e13245_d_n5;
        locals.var_tmf2_dn6 = assign18580_e13245_d_n6;
        locals.var_tmf2_dn7 = assign18580_e13245_d_n7;
        locals.var_tmf2_dn8 = assign18580_e13245_d_n8;
        locals.var_tmf2_dn9 = assign18580_e13245_d_n9;
        locals.var_tmf2_dn10 = assign18580_e13245_d_n10;
        locals.var_tmf2_dn13 = assign18580_e13245_d_n13;

        let (assign18590_e13261, assign18590_e13261_d_n0, assign18590_e13261_d_n2, assign18590_e13261_d_n4, assign18590_e13261_d_n5, assign18590_e13261_d_n6, assign18590_e13261_d_n7, assign18590_e13261_d_n8, assign18590_e13261_d_n9, assign18590_e13261_d_n10, assign18590_e13261_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard381 != 0.0)) && (locals.var_guard382 == 0.0)) {
        let assign18590_e13256: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18590_e13258: f64 = (assign18590_e13256 + locals.var_tmf2);
        let assign18590_e13259: f64 = (assign18590_e13258).sqrt();
        (assign18590_e13259, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18590_e13259)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18590_e13259)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign18590_e13259)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign18590_e13259)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign18590_e13259)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign18590_e13259)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign18590_e13259)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign18590_e13259)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign18590_e13259)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign18590_e13259)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18590_e13261;
        locals.var_tmf2_dn0 = assign18590_e13261_d_n0;
        locals.var_tmf2_dn2 = assign18590_e13261_d_n2;
        locals.var_tmf2_dn4 = assign18590_e13261_d_n4;
        locals.var_tmf2_dn5 = assign18590_e13261_d_n5;
        locals.var_tmf2_dn6 = assign18590_e13261_d_n6;
        locals.var_tmf2_dn7 = assign18590_e13261_d_n7;
        locals.var_tmf2_dn8 = assign18590_e13261_d_n8;
        locals.var_tmf2_dn9 = assign18590_e13261_d_n9;
        locals.var_tmf2_dn10 = assign18590_e13261_d_n10;
        locals.var_tmf2_dn13 = assign18590_e13261_d_n13;

        let (assign18600_e13278, assign18600_e13278_d_n0, assign18600_e13278_d_n2, assign18600_e13278_d_n4, assign18600_e13278_d_n5, assign18600_e13278_d_n6, assign18600_e13278_d_n7, assign18600_e13278_d_n8, assign18600_e13278_d_n9, assign18600_e13278_d_n10, assign18600_e13278_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard381 != 0.0)) && (locals.var_guard382 == 0.0)) {
        let assign18600_e13274: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign18600_e13275: f64 = (1.0 + assign18600_e13274);
        let assign18600_e13276: f64 = (0.5 * assign18600_e13275);
        (assign18600_e13276, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign18600_e13278;
        locals.var_t0_dn0 = assign18600_e13278_d_n0;
        locals.var_t0_dn2 = assign18600_e13278_d_n2;
        locals.var_t0_dn4 = assign18600_e13278_d_n4;
        locals.var_t0_dn5 = assign18600_e13278_d_n5;
        locals.var_t0_dn6 = assign18600_e13278_d_n6;
        locals.var_t0_dn7 = assign18600_e13278_d_n7;
        locals.var_t0_dn8 = assign18600_e13278_d_n8;
        locals.var_t0_dn9 = assign18600_e13278_d_n9;
        locals.var_t0_dn10 = assign18600_e13278_d_n10;
        locals.var_t0_dn13 = assign18600_e13278_d_n13;

        let (assign18610_e13297, assign18610_e13297_d_n0, assign18610_e13297_d_n2, assign18610_e13297_d_n4, assign18610_e13297_d_n5, assign18610_e13297_d_n6, assign18610_e13297_d_n7, assign18610_e13297_d_n8, assign18610_e13297_d_n9, assign18610_e13297_d_n10, assign18610_e13297_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard381 != 0.0)) && (locals.var_guard382 == 0.0)) {
        let assign18610_e13289: f64 = (0.005 * locals.var_uc_rs);
        let assign18610_e13293: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign18610_e13294: f64 = (0.5 * assign18610_e13293);
        let assign18610_e13295: f64 = (assign18610_e13289 + assign18610_e13294);
        (assign18610_e13295, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn13,)
    }
};
        locals.var_rse = assign18610_e13297;
        locals.var_rse_dn0 = assign18610_e13297_d_n0;
        locals.var_rse_dn2 = assign18610_e13297_d_n2;
        locals.var_rse_dn4 = assign18610_e13297_d_n4;
        locals.var_rse_dn5 = assign18610_e13297_d_n5;
        locals.var_rse_dn6 = assign18610_e13297_d_n6;
        locals.var_rse_dn7 = assign18610_e13297_d_n7;
        locals.var_rse_dn8 = assign18610_e13297_d_n8;
        locals.var_rse_dn9 = assign18610_e13297_d_n9;
        locals.var_rse_dn10 = assign18610_e13297_d_n10;
        locals.var_rse_dn13 = assign18610_e13297_d_n13;

        let (assign18620_e13306, assign18620_e13306_d_n0, assign18620_e13306_d_n2, assign18620_e13306_d_n4, assign18620_e13306_d_n5, assign18620_e13306_d_n6, assign18620_e13306_d_n7, assign18620_e13306_d_n8, assign18620_e13306_d_n9, assign18620_e13306_d_n10, assign18620_e13306_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard381 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn13,)
    }
};
        locals.var_rse = assign18620_e13306;
        locals.var_rse_dn0 = assign18620_e13306_d_n0;
        locals.var_rse_dn2 = assign18620_e13306_d_n2;
        locals.var_rse_dn4 = assign18620_e13306_d_n4;
        locals.var_rse_dn5 = assign18620_e13306_d_n5;
        locals.var_rse_dn6 = assign18620_e13306_d_n6;
        locals.var_rse_dn7 = assign18620_e13306_d_n7;
        locals.var_rse_dn8 = assign18620_e13306_d_n8;
        locals.var_rse_dn9 = assign18620_e13306_d_n9;
        locals.var_rse_dn10 = assign18620_e13306_d_n10;
        locals.var_rse_dn13 = assign18620_e13306_d_n13;

        let assign18630_e13309: f64 = if locals.var_uc_rdvd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard383 = assign18630_e13309;

        let (assign18640_e13333, assign18640_e13333_d_n0, assign18640_e13333_d_n2, assign18640_e13333_d_n4, assign18640_e13333_d_n5, assign18640_e13333_d_n6, assign18640_e13333_d_n7, assign18640_e13333_d_n8, assign18640_e13333_d_n9, assign18640_e13333_d_n10, assign18640_e13333_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign18640_e13318: f64 = (p.p67 * locals.var_uc_rdslp1);
        let assign18640_e13320: f64 = (assign18640_e13318 * 1000000.0);
        let assign18640_e13322: f64 = (assign18640_e13320 + locals.var_uc_rdict1);
        let assign18640_e13323: f64 = (locals.var_rdvdtemp0 * assign18640_e13322);
        let assign18640_e13326: f64 = (p.p68 * p.p100);
        let assign18640_e13328: f64 = (assign18640_e13326 * 1000000.0);
        let assign18640_e13330: f64 = (assign18640_e13328 + p.p101);
        let assign18640_e13331: f64 = (assign18640_e13323 * assign18640_e13330);
        (assign18640_e13331, ((locals.var_rdvdtemp0_dn0 * assign18640_e13322) * assign18640_e13330), ((locals.var_rdvdtemp0_dn2 * assign18640_e13322) * assign18640_e13330), ((locals.var_rdvdtemp0_dn4 * assign18640_e13322) * assign18640_e13330), ((locals.var_rdvdtemp0_dn5 * assign18640_e13322) * assign18640_e13330), ((locals.var_rdvdtemp0_dn6 * assign18640_e13322) * assign18640_e13330), ((locals.var_rdvdtemp0_dn7 * assign18640_e13322) * assign18640_e13330), ((locals.var_rdvdtemp0_dn8 * assign18640_e13322) * assign18640_e13330), ((locals.var_rdvdtemp0_dn9 * assign18640_e13322) * assign18640_e13330), ((locals.var_rdvdtemp0_dn10 * assign18640_e13322) * assign18640_e13330), ((locals.var_rdvdtemp0_dn13 * assign18640_e13322) * assign18640_e13330),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign18640_e13333;
        locals.var_t4_dn0 = assign18640_e13333_d_n0;
        locals.var_t4_dn2 = assign18640_e13333_d_n2;
        locals.var_t4_dn4 = assign18640_e13333_d_n4;
        locals.var_t4_dn5 = assign18640_e13333_d_n5;
        locals.var_t4_dn6 = assign18640_e13333_d_n6;
        locals.var_t4_dn7 = assign18640_e13333_d_n7;
        locals.var_t4_dn8 = assign18640_e13333_d_n8;
        locals.var_t4_dn9 = assign18640_e13333_d_n9;
        locals.var_t4_dn10 = assign18640_e13333_d_n10;
        locals.var_t4_dn13 = assign18640_e13333_d_n13;

        let (assign18650_e13347, assign18650_e13347_d_n0, assign18650_e13347_d_n2, assign18650_e13347_d_n4, assign18650_e13347_d_n5, assign18650_e13347_d_n6, assign18650_e13347_d_n7, assign18650_e13347_d_n8, assign18650_e13347_d_n9, assign18650_e13347_d_n10, assign18650_e13347_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign18650_e13341: f64 = (1.0 - locals.var_uc_rdov13);
        let assign18650_e13343: f64 = (assign18650_e13341 * p.p63);
        let assign18650_e13345: f64 = (assign18650_e13343 * 1000000.0);
        (assign18650_e13345, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign18650_e13347;
        locals.var_t1_dn0 = assign18650_e13347_d_n0;
        locals.var_t1_dn2 = assign18650_e13347_d_n2;
        locals.var_t1_dn4 = assign18650_e13347_d_n4;
        locals.var_t1_dn5 = assign18650_e13347_d_n5;
        locals.var_t1_dn6 = assign18650_e13347_d_n6;
        locals.var_t1_dn7 = assign18650_e13347_d_n7;
        locals.var_t1_dn8 = assign18650_e13347_d_n8;
        locals.var_t1_dn9 = assign18650_e13347_d_n9;
        locals.var_t1_dn10 = assign18650_e13347_d_n10;
        locals.var_t1_dn13 = assign18650_e13347_d_n13;

        let (assign18660_e13368, assign18660_e13368_d_n0, assign18660_e13368_d_n2, assign18660_e13368_d_n4, assign18660_e13368_d_n5, assign18660_e13368_d_n6, assign18660_e13368_d_n7, assign18660_e13368_d_n8, assign18660_e13368_d_n9, assign18660_e13368_d_n10, assign18660_e13368_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign18660_e13355: f64 = (p.p99 * p.p99);
        let assign18660_e13359: f64 = (0.0001 * 0.01);
        let assign18660_e13360: f64 = (4.0 * assign18660_e13359);
        let assign18660_e13363: f64 = (0.0001 * 0.01);
        let assign18660_e13364: f64 = (assign18660_e13360 * assign18660_e13363);
        let assign18660_e13365: f64 = (assign18660_e13355 + assign18660_e13364);
        let assign18660_e13366: f64 = (assign18660_e13365).sqrt();
        (assign18660_e13366, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18660_e13368;
        locals.var_tmf2_dn0 = assign18660_e13368_d_n0;
        locals.var_tmf2_dn2 = assign18660_e13368_d_n2;
        locals.var_tmf2_dn4 = assign18660_e13368_d_n4;
        locals.var_tmf2_dn5 = assign18660_e13368_d_n5;
        locals.var_tmf2_dn6 = assign18660_e13368_d_n6;
        locals.var_tmf2_dn7 = assign18660_e13368_d_n7;
        locals.var_tmf2_dn8 = assign18660_e13368_d_n8;
        locals.var_tmf2_dn9 = assign18660_e13368_d_n9;
        locals.var_tmf2_dn10 = assign18660_e13368_d_n10;
        locals.var_tmf2_dn13 = assign18660_e13368_d_n13;

        let (assign18670_e13382, assign18670_e13382_d_n0, assign18670_e13382_d_n2, assign18670_e13382_d_n4, assign18670_e13382_d_n5, assign18670_e13382_d_n6, assign18670_e13382_d_n7, assign18670_e13382_d_n8, assign18670_e13382_d_n9, assign18670_e13382_d_n10, assign18670_e13382_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign18670_e13378: f64 = (p.p99 / locals.var_tmf2);
        let assign18670_e13379: f64 = (1.0 + assign18670_e13378);
        let assign18670_e13380: f64 = (0.5 * assign18670_e13379);
        (assign18670_e13380, (0.5 * (-((p.p99 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn13) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign18670_e13382;
        locals.var_t0_dn0 = assign18670_e13382_d_n0;
        locals.var_t0_dn2 = assign18670_e13382_d_n2;
        locals.var_t0_dn4 = assign18670_e13382_d_n4;
        locals.var_t0_dn5 = assign18670_e13382_d_n5;
        locals.var_t0_dn6 = assign18670_e13382_d_n6;
        locals.var_t0_dn7 = assign18670_e13382_d_n7;
        locals.var_t0_dn8 = assign18670_e13382_d_n8;
        locals.var_t0_dn9 = assign18670_e13382_d_n9;
        locals.var_t0_dn10 = assign18670_e13382_d_n10;
        locals.var_t0_dn13 = assign18670_e13382_d_n13;

        let (assign18680_e13394, assign18680_e13394_d_n0, assign18680_e13394_d_n2, assign18680_e13394_d_n4, assign18680_e13394_d_n5, assign18680_e13394_d_n6, assign18680_e13394_d_n7, assign18680_e13394_d_n8, assign18680_e13394_d_n9, assign18680_e13394_d_n10, assign18680_e13394_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign18680_e13391: f64 = (p.p99 + locals.var_tmf2);
        let assign18680_e13392: f64 = (0.5 * assign18680_e13391);
        (assign18680_e13392, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * locals.var_tmf2_dn5), (0.5 * locals.var_tmf2_dn6), (0.5 * locals.var_tmf2_dn7), (0.5 * locals.var_tmf2_dn8), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign18680_e13394;
        locals.var_t2_dn0 = assign18680_e13394_d_n0;
        locals.var_t2_dn2 = assign18680_e13394_d_n2;
        locals.var_t2_dn4 = assign18680_e13394_d_n4;
        locals.var_t2_dn5 = assign18680_e13394_d_n5;
        locals.var_t2_dn6 = assign18680_e13394_d_n6;
        locals.var_t2_dn7 = assign18680_e13394_d_n7;
        locals.var_t2_dn8 = assign18680_e13394_d_n8;
        locals.var_t2_dn9 = assign18680_e13394_d_n9;
        locals.var_t2_dn10 = assign18680_e13394_d_n10;
        locals.var_t2_dn13 = assign18680_e13394_d_n13;

        let assign18690_e13397: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard384 = assign18690_e13397;

        let (assign18700_e13407, assign18700_e13407_d_n0, assign18700_e13407_d_n2, assign18700_e13407_d_n4, assign18700_e13407_d_n5, assign18700_e13407_d_n6, assign18700_e13407_d_n7, assign18700_e13407_d_n8, assign18700_e13407_d_n9, assign18700_e13407_d_n10, assign18700_e13407_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign18700_e13407;
        locals.var_t2_dn0 = assign18700_e13407_d_n0;
        locals.var_t2_dn2 = assign18700_e13407_d_n2;
        locals.var_t2_dn4 = assign18700_e13407_d_n4;
        locals.var_t2_dn5 = assign18700_e13407_d_n5;
        locals.var_t2_dn6 = assign18700_e13407_d_n6;
        locals.var_t2_dn7 = assign18700_e13407_d_n7;
        locals.var_t2_dn8 = assign18700_e13407_d_n8;
        locals.var_t2_dn9 = assign18700_e13407_d_n9;
        locals.var_t2_dn10 = assign18700_e13407_d_n10;
        locals.var_t2_dn13 = assign18700_e13407_d_n13;

        let (assign18710_e13417, assign18710_e13417_d_n0, assign18710_e13417_d_n2, assign18710_e13417_d_n4, assign18710_e13417_d_n5, assign18710_e13417_d_n6, assign18710_e13417_d_n7, assign18710_e13417_d_n8, assign18710_e13417_d_n9, assign18710_e13417_d_n10, assign18710_e13417_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign18710_e13417;
        locals.var_t0_dn0 = assign18710_e13417_d_n0;
        locals.var_t0_dn2 = assign18710_e13417_d_n2;
        locals.var_t0_dn4 = assign18710_e13417_d_n4;
        locals.var_t0_dn5 = assign18710_e13417_d_n5;
        locals.var_t0_dn6 = assign18710_e13417_d_n6;
        locals.var_t0_dn7 = assign18710_e13417_d_n7;
        locals.var_t0_dn8 = assign18710_e13417_d_n8;
        locals.var_t0_dn9 = assign18710_e13417_d_n9;
        locals.var_t0_dn10 = assign18710_e13417_d_n10;
        locals.var_t0_dn13 = assign18710_e13417_d_n13;

    }

    pub(super) fn stamp_transient_block_41(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign18720_e13428, assign18720_e13428_d_n0, assign18720_e13428_d_n2, assign18720_e13428_d_n4, assign18720_e13428_d_n5, assign18720_e13428_d_n6, assign18720_e13428_d_n7, assign18720_e13428_d_n8, assign18720_e13428_d_n9, assign18720_e13428_d_n10, assign18720_e13428_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign18720_e13424: f64 = (-p.p98);
        let assign18720_e13426: f64 = (assign18720_e13424 / locals.var_t2);
        (assign18720_e13426, (-((assign18720_e13424 * locals.var_t2_dn0) / (locals.var_t2 * locals.var_t2))), (-((assign18720_e13424 * locals.var_t2_dn2) / (locals.var_t2 * locals.var_t2))), (-((assign18720_e13424 * locals.var_t2_dn4) / (locals.var_t2 * locals.var_t2))), (-((assign18720_e13424 * locals.var_t2_dn5) / (locals.var_t2 * locals.var_t2))), (-((assign18720_e13424 * locals.var_t2_dn6) / (locals.var_t2 * locals.var_t2))), (-((assign18720_e13424 * locals.var_t2_dn7) / (locals.var_t2 * locals.var_t2))), (-((assign18720_e13424 * locals.var_t2_dn8) / (locals.var_t2 * locals.var_t2))), (-((assign18720_e13424 * locals.var_t2_dn9) / (locals.var_t2 * locals.var_t2))), (-((assign18720_e13424 * locals.var_t2_dn10) / (locals.var_t2 * locals.var_t2))), (-((assign18720_e13424 * locals.var_t2_dn13) / (locals.var_t2 * locals.var_t2))),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn13,)
    }
};
        locals.var_t8 = assign18720_e13428;
        locals.var_t8_dn0 = assign18720_e13428_d_n0;
        locals.var_t8_dn2 = assign18720_e13428_d_n2;
        locals.var_t8_dn4 = assign18720_e13428_d_n4;
        locals.var_t8_dn5 = assign18720_e13428_d_n5;
        locals.var_t8_dn6 = assign18720_e13428_d_n6;
        locals.var_t8_dn7 = assign18720_e13428_d_n7;
        locals.var_t8_dn8 = assign18720_e13428_d_n8;
        locals.var_t8_dn9 = assign18720_e13428_d_n9;
        locals.var_t8_dn10 = assign18720_e13428_d_n10;
        locals.var_t8_dn13 = assign18720_e13428_d_n13;

        let (assign18730_e13444, assign18730_e13444_d_n0, assign18730_e13444_d_n2, assign18730_e13444_d_n4, assign18730_e13444_d_n5, assign18730_e13444_d_n6, assign18730_e13444_d_n7, assign18730_e13444_d_n8, assign18730_e13444_d_n9, assign18730_e13444_d_n10, assign18730_e13444_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign18730_e13436: f64 = (locals.var_t8 * p.p63);
        let assign18730_e13438: f64 = (assign18730_e13436 * 1000000.0);
        let assign18730_e13440: f64 = (assign18730_e13438 + 1.0);
        let assign18730_e13442: f64 = (assign18730_e13440 + p.p98);
        (assign18730_e13442, ((locals.var_t8_dn0 * p.p63) * 1000000.0), ((locals.var_t8_dn2 * p.p63) * 1000000.0), ((locals.var_t8_dn4 * p.p63) * 1000000.0), ((locals.var_t8_dn5 * p.p63) * 1000000.0), ((locals.var_t8_dn6 * p.p63) * 1000000.0), ((locals.var_t8_dn7 * p.p63) * 1000000.0), ((locals.var_t8_dn8 * p.p63) * 1000000.0), ((locals.var_t8_dn9 * p.p63) * 1000000.0), ((locals.var_t8_dn10 * p.p63) * 1000000.0), ((locals.var_t8_dn13 * p.p63) * 1000000.0),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign18730_e13444;
        locals.var_t3_dn0 = assign18730_e13444_d_n0;
        locals.var_t3_dn2 = assign18730_e13444_d_n2;
        locals.var_t3_dn4 = assign18730_e13444_d_n4;
        locals.var_t3_dn5 = assign18730_e13444_d_n5;
        locals.var_t3_dn6 = assign18730_e13444_d_n6;
        locals.var_t3_dn7 = assign18730_e13444_d_n7;
        locals.var_t3_dn8 = assign18730_e13444_d_n8;
        locals.var_t3_dn9 = assign18730_e13444_d_n9;
        locals.var_t3_dn10 = assign18730_e13444_d_n10;
        locals.var_t3_dn13 = assign18730_e13444_d_n13;

        let (assign18740_e13458, assign18740_e13458_d_n0, assign18740_e13458_d_n2, assign18740_e13458_d_n4, assign18740_e13458_d_n5, assign18740_e13458_d_n6, assign18740_e13458_d_n7, assign18740_e13458_d_n8, assign18740_e13458_d_n9, assign18740_e13458_d_n10, assign18740_e13458_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign18740_e13452: f64 = (locals.var_t3 * locals.var_t4);
        let assign18740_e13454: f64 = (assign18740_e13452 - locals.var_t4);
        let assign18740_e13456: f64 = (assign18740_e13454 - 0.01);
        (assign18740_e13456, (((locals.var_t3_dn0 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn0)) - locals.var_t4_dn0), (((locals.var_t3_dn2 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn2)) - locals.var_t4_dn2), (((locals.var_t3_dn4 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn4)) - locals.var_t4_dn4), (((locals.var_t3_dn5 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn5)) - locals.var_t4_dn5), (((locals.var_t3_dn6 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn6)) - locals.var_t4_dn6), (((locals.var_t3_dn7 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn7)) - locals.var_t4_dn7), (((locals.var_t3_dn8 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn8)) - locals.var_t4_dn8), (((locals.var_t3_dn9 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn9)) - locals.var_t4_dn9), (((locals.var_t3_dn10 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn10)) - locals.var_t4_dn10), (((locals.var_t3_dn13 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn13)) - locals.var_t4_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign18740_e13458;
        locals.var_tmf1_dn0 = assign18740_e13458_d_n0;
        locals.var_tmf1_dn2 = assign18740_e13458_d_n2;
        locals.var_tmf1_dn4 = assign18740_e13458_d_n4;
        locals.var_tmf1_dn5 = assign18740_e13458_d_n5;
        locals.var_tmf1_dn6 = assign18740_e13458_d_n6;
        locals.var_tmf1_dn7 = assign18740_e13458_d_n7;
        locals.var_tmf1_dn8 = assign18740_e13458_d_n8;
        locals.var_tmf1_dn9 = assign18740_e13458_d_n9;
        locals.var_tmf1_dn10 = assign18740_e13458_d_n10;
        locals.var_tmf1_dn13 = assign18740_e13458_d_n13;

        let (assign18750_e13470, assign18750_e13470_d_n0, assign18750_e13470_d_n2, assign18750_e13470_d_n4, assign18750_e13470_d_n5, assign18750_e13470_d_n6, assign18750_e13470_d_n7, assign18750_e13470_d_n8, assign18750_e13470_d_n9, assign18750_e13470_d_n10, assign18750_e13470_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign18750_e13466: f64 = (4.0 * locals.var_t4);
        let assign18750_e13468: f64 = (assign18750_e13466 * 0.01);
        (assign18750_e13468, ((4.0 * locals.var_t4_dn0) * 0.01), ((4.0 * locals.var_t4_dn2) * 0.01), ((4.0 * locals.var_t4_dn4) * 0.01), ((4.0 * locals.var_t4_dn5) * 0.01), ((4.0 * locals.var_t4_dn6) * 0.01), ((4.0 * locals.var_t4_dn7) * 0.01), ((4.0 * locals.var_t4_dn8) * 0.01), ((4.0 * locals.var_t4_dn9) * 0.01), ((4.0 * locals.var_t4_dn10) * 0.01), ((4.0 * locals.var_t4_dn13) * 0.01),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18750_e13470;
        locals.var_tmf2_dn0 = assign18750_e13470_d_n0;
        locals.var_tmf2_dn2 = assign18750_e13470_d_n2;
        locals.var_tmf2_dn4 = assign18750_e13470_d_n4;
        locals.var_tmf2_dn5 = assign18750_e13470_d_n5;
        locals.var_tmf2_dn6 = assign18750_e13470_d_n6;
        locals.var_tmf2_dn7 = assign18750_e13470_d_n7;
        locals.var_tmf2_dn8 = assign18750_e13470_d_n8;
        locals.var_tmf2_dn9 = assign18750_e13470_d_n9;
        locals.var_tmf2_dn10 = assign18750_e13470_d_n10;
        locals.var_tmf2_dn13 = assign18750_e13470_d_n13;

        let (assign18760_e13484, assign18760_e13484_d_n0, assign18760_e13484_d_n2, assign18760_e13484_d_n4, assign18760_e13484_d_n5, assign18760_e13484_d_n6, assign18760_e13484_d_n7, assign18760_e13484_d_n8, assign18760_e13484_d_n9, assign18760_e13484_d_n10, assign18760_e13484_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let (assign18760_e13482, assign18760_e13482_d_n0, assign18760_e13482_d_n2, assign18760_e13482_d_n4, assign18760_e13482_d_n5, assign18760_e13482_d_n6, assign18760_e13482_d_n7, assign18760_e13482_d_n8, assign18760_e13482_d_n9, assign18760_e13482_d_n10, assign18760_e13482_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign18760_e13481: f64 = (-locals.var_tmf2);
                (assign18760_e13481, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign18760_e13482, assign18760_e13482_d_n0, assign18760_e13482_d_n2, assign18760_e13482_d_n4, assign18760_e13482_d_n5, assign18760_e13482_d_n6, assign18760_e13482_d_n7, assign18760_e13482_d_n8, assign18760_e13482_d_n9, assign18760_e13482_d_n10, assign18760_e13482_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18760_e13484;
        locals.var_tmf2_dn0 = assign18760_e13484_d_n0;
        locals.var_tmf2_dn2 = assign18760_e13484_d_n2;
        locals.var_tmf2_dn4 = assign18760_e13484_d_n4;
        locals.var_tmf2_dn5 = assign18760_e13484_d_n5;
        locals.var_tmf2_dn6 = assign18760_e13484_d_n6;
        locals.var_tmf2_dn7 = assign18760_e13484_d_n7;
        locals.var_tmf2_dn8 = assign18760_e13484_d_n8;
        locals.var_tmf2_dn9 = assign18760_e13484_d_n9;
        locals.var_tmf2_dn10 = assign18760_e13484_d_n10;
        locals.var_tmf2_dn13 = assign18760_e13484_d_n13;

        let (assign18770_e13497, assign18770_e13497_d_n0, assign18770_e13497_d_n2, assign18770_e13497_d_n4, assign18770_e13497_d_n5, assign18770_e13497_d_n6, assign18770_e13497_d_n7, assign18770_e13497_d_n8, assign18770_e13497_d_n9, assign18770_e13497_d_n10, assign18770_e13497_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign18770_e13492: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18770_e13494: f64 = (assign18770_e13492 + locals.var_tmf2);
        let assign18770_e13495: f64 = (assign18770_e13494).sqrt();
        (assign18770_e13495, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18770_e13495)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18770_e13495)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign18770_e13495)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign18770_e13495)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign18770_e13495)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign18770_e13495)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign18770_e13495)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign18770_e13495)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign18770_e13495)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign18770_e13495)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18770_e13497;
        locals.var_tmf2_dn0 = assign18770_e13497_d_n0;
        locals.var_tmf2_dn2 = assign18770_e13497_d_n2;
        locals.var_tmf2_dn4 = assign18770_e13497_d_n4;
        locals.var_tmf2_dn5 = assign18770_e13497_d_n5;
        locals.var_tmf2_dn6 = assign18770_e13497_d_n6;
        locals.var_tmf2_dn7 = assign18770_e13497_d_n7;
        locals.var_tmf2_dn8 = assign18770_e13497_d_n8;
        locals.var_tmf2_dn9 = assign18770_e13497_d_n9;
        locals.var_tmf2_dn10 = assign18770_e13497_d_n10;
        locals.var_tmf2_dn13 = assign18770_e13497_d_n13;

        let (assign18780_e13511, assign18780_e13511_d_n0, assign18780_e13511_d_n2, assign18780_e13511_d_n4, assign18780_e13511_d_n5, assign18780_e13511_d_n6, assign18780_e13511_d_n7, assign18780_e13511_d_n8, assign18780_e13511_d_n9, assign18780_e13511_d_n10, assign18780_e13511_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign18780_e13507: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign18780_e13508: f64 = (1.0 + assign18780_e13507);
        let assign18780_e13509: f64 = (0.5 * assign18780_e13508);
        (assign18780_e13509, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign18780_e13511;
        locals.var_t6_dn0 = assign18780_e13511_d_n0;
        locals.var_t6_dn2 = assign18780_e13511_d_n2;
        locals.var_t6_dn4 = assign18780_e13511_d_n4;
        locals.var_t6_dn5 = assign18780_e13511_d_n5;
        locals.var_t6_dn6 = assign18780_e13511_d_n6;
        locals.var_t6_dn7 = assign18780_e13511_d_n7;
        locals.var_t6_dn8 = assign18780_e13511_d_n8;
        locals.var_t6_dn9 = assign18780_e13511_d_n9;
        locals.var_t6_dn10 = assign18780_e13511_d_n10;
        locals.var_t6_dn13 = assign18780_e13511_d_n13;

        let (assign18790_e13525, assign18790_e13525_d_n0, assign18790_e13525_d_n2, assign18790_e13525_d_n4, assign18790_e13525_d_n5, assign18790_e13525_d_n6, assign18790_e13525_d_n7, assign18790_e13525_d_n8, assign18790_e13525_d_n9, assign18790_e13525_d_n10, assign18790_e13525_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign18790_e13521: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign18790_e13522: f64 = (0.5 * assign18790_e13521);
        let assign18790_e13523: f64 = (locals.var_t4 + assign18790_e13522);
        (assign18790_e13523, (locals.var_t4_dn0 + (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t4_dn2 + (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t4_dn4 + (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t4_dn5 + (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t4_dn6 + (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t4_dn7 + (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t4_dn8 + (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t4_dn9 + (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t4_dn10 + (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t4_dn13 + (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign18790_e13525;
        locals.var_t5_dn0 = assign18790_e13525_d_n0;
        locals.var_t5_dn2 = assign18790_e13525_d_n2;
        locals.var_t5_dn4 = assign18790_e13525_d_n4;
        locals.var_t5_dn5 = assign18790_e13525_d_n5;
        locals.var_t5_dn6 = assign18790_e13525_d_n6;
        locals.var_t5_dn7 = assign18790_e13525_d_n7;
        locals.var_t5_dn8 = assign18790_e13525_d_n8;
        locals.var_t5_dn9 = assign18790_e13525_d_n9;
        locals.var_t5_dn10 = assign18790_e13525_d_n10;
        locals.var_t5_dn13 = assign18790_e13525_d_n13;

        let (assign18800_e13541, assign18800_e13541_d_n0, assign18800_e13541_d_n2, assign18800_e13541_d_n4, assign18800_e13541_d_n5, assign18800_e13541_d_n6, assign18800_e13541_d_n7, assign18800_e13541_d_n8, assign18800_e13541_d_n9, assign18800_e13541_d_n10, assign18800_e13541_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign18800_e13534: f64 = (p.p98 + 1.0);
        let assign18800_e13535: f64 = (locals.var_t4 * assign18800_e13534);
        let assign18800_e13537: f64 = (assign18800_e13535 - locals.var_t5);
        let assign18800_e13539: f64 = (assign18800_e13537 - 5e-5);
        (assign18800_e13539, ((locals.var_t4_dn0 * assign18800_e13534) - locals.var_t5_dn0), ((locals.var_t4_dn2 * assign18800_e13534) - locals.var_t5_dn2), ((locals.var_t4_dn4 * assign18800_e13534) - locals.var_t5_dn4), ((locals.var_t4_dn5 * assign18800_e13534) - locals.var_t5_dn5), ((locals.var_t4_dn6 * assign18800_e13534) - locals.var_t5_dn6), ((locals.var_t4_dn7 * assign18800_e13534) - locals.var_t5_dn7), ((locals.var_t4_dn8 * assign18800_e13534) - locals.var_t5_dn8), ((locals.var_t4_dn9 * assign18800_e13534) - locals.var_t5_dn9), ((locals.var_t4_dn10 * assign18800_e13534) - locals.var_t5_dn10), ((locals.var_t4_dn13 * assign18800_e13534) - locals.var_t5_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign18800_e13541;
        locals.var_tmf1_dn0 = assign18800_e13541_d_n0;
        locals.var_tmf1_dn2 = assign18800_e13541_d_n2;
        locals.var_tmf1_dn4 = assign18800_e13541_d_n4;
        locals.var_tmf1_dn5 = assign18800_e13541_d_n5;
        locals.var_tmf1_dn6 = assign18800_e13541_d_n6;
        locals.var_tmf1_dn7 = assign18800_e13541_d_n7;
        locals.var_tmf1_dn8 = assign18800_e13541_d_n8;
        locals.var_tmf1_dn9 = assign18800_e13541_d_n9;
        locals.var_tmf1_dn10 = assign18800_e13541_d_n10;
        locals.var_tmf1_dn13 = assign18800_e13541_d_n13;

        let (assign18810_e13557, assign18810_e13557_d_n0, assign18810_e13557_d_n2, assign18810_e13557_d_n4, assign18810_e13557_d_n5, assign18810_e13557_d_n6, assign18810_e13557_d_n7, assign18810_e13557_d_n8, assign18810_e13557_d_n9, assign18810_e13557_d_n10, assign18810_e13557_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign18810_e13551: f64 = (p.p98 + 1.0);
        let assign18810_e13552: f64 = (locals.var_t4 * assign18810_e13551);
        let assign18810_e13553: f64 = (4.0 * assign18810_e13552);
        let assign18810_e13555: f64 = (assign18810_e13553 * 5e-5);
        (assign18810_e13555, ((4.0 * (locals.var_t4_dn0 * assign18810_e13551)) * 5e-5), ((4.0 * (locals.var_t4_dn2 * assign18810_e13551)) * 5e-5), ((4.0 * (locals.var_t4_dn4 * assign18810_e13551)) * 5e-5), ((4.0 * (locals.var_t4_dn5 * assign18810_e13551)) * 5e-5), ((4.0 * (locals.var_t4_dn6 * assign18810_e13551)) * 5e-5), ((4.0 * (locals.var_t4_dn7 * assign18810_e13551)) * 5e-5), ((4.0 * (locals.var_t4_dn8 * assign18810_e13551)) * 5e-5), ((4.0 * (locals.var_t4_dn9 * assign18810_e13551)) * 5e-5), ((4.0 * (locals.var_t4_dn10 * assign18810_e13551)) * 5e-5), ((4.0 * (locals.var_t4_dn13 * assign18810_e13551)) * 5e-5),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18810_e13557;
        locals.var_tmf2_dn0 = assign18810_e13557_d_n0;
        locals.var_tmf2_dn2 = assign18810_e13557_d_n2;
        locals.var_tmf2_dn4 = assign18810_e13557_d_n4;
        locals.var_tmf2_dn5 = assign18810_e13557_d_n5;
        locals.var_tmf2_dn6 = assign18810_e13557_d_n6;
        locals.var_tmf2_dn7 = assign18810_e13557_d_n7;
        locals.var_tmf2_dn8 = assign18810_e13557_d_n8;
        locals.var_tmf2_dn9 = assign18810_e13557_d_n9;
        locals.var_tmf2_dn10 = assign18810_e13557_d_n10;
        locals.var_tmf2_dn13 = assign18810_e13557_d_n13;

        let (assign18820_e13571, assign18820_e13571_d_n0, assign18820_e13571_d_n2, assign18820_e13571_d_n4, assign18820_e13571_d_n5, assign18820_e13571_d_n6, assign18820_e13571_d_n7, assign18820_e13571_d_n8, assign18820_e13571_d_n9, assign18820_e13571_d_n10, assign18820_e13571_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let (assign18820_e13569, assign18820_e13569_d_n0, assign18820_e13569_d_n2, assign18820_e13569_d_n4, assign18820_e13569_d_n5, assign18820_e13569_d_n6, assign18820_e13569_d_n7, assign18820_e13569_d_n8, assign18820_e13569_d_n9, assign18820_e13569_d_n10, assign18820_e13569_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign18820_e13568: f64 = (-locals.var_tmf2);
                (assign18820_e13568, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign18820_e13569, assign18820_e13569_d_n0, assign18820_e13569_d_n2, assign18820_e13569_d_n4, assign18820_e13569_d_n5, assign18820_e13569_d_n6, assign18820_e13569_d_n7, assign18820_e13569_d_n8, assign18820_e13569_d_n9, assign18820_e13569_d_n10, assign18820_e13569_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18820_e13571;
        locals.var_tmf2_dn0 = assign18820_e13571_d_n0;
        locals.var_tmf2_dn2 = assign18820_e13571_d_n2;
        locals.var_tmf2_dn4 = assign18820_e13571_d_n4;
        locals.var_tmf2_dn5 = assign18820_e13571_d_n5;
        locals.var_tmf2_dn6 = assign18820_e13571_d_n6;
        locals.var_tmf2_dn7 = assign18820_e13571_d_n7;
        locals.var_tmf2_dn8 = assign18820_e13571_d_n8;
        locals.var_tmf2_dn9 = assign18820_e13571_d_n9;
        locals.var_tmf2_dn10 = assign18820_e13571_d_n10;
        locals.var_tmf2_dn13 = assign18820_e13571_d_n13;

        let (assign18830_e13584, assign18830_e13584_d_n0, assign18830_e13584_d_n2, assign18830_e13584_d_n4, assign18830_e13584_d_n5, assign18830_e13584_d_n6, assign18830_e13584_d_n7, assign18830_e13584_d_n8, assign18830_e13584_d_n9, assign18830_e13584_d_n10, assign18830_e13584_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign18830_e13579: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18830_e13581: f64 = (assign18830_e13579 + locals.var_tmf2);
        let assign18830_e13582: f64 = (assign18830_e13581).sqrt();
        (assign18830_e13582, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18830_e13582)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18830_e13582)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign18830_e13582)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign18830_e13582)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign18830_e13582)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign18830_e13582)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign18830_e13582)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign18830_e13582)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign18830_e13582)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign18830_e13582)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18830_e13584;
        locals.var_tmf2_dn0 = assign18830_e13584_d_n0;
        locals.var_tmf2_dn2 = assign18830_e13584_d_n2;
        locals.var_tmf2_dn4 = assign18830_e13584_d_n4;
        locals.var_tmf2_dn5 = assign18830_e13584_d_n5;
        locals.var_tmf2_dn6 = assign18830_e13584_d_n6;
        locals.var_tmf2_dn7 = assign18830_e13584_d_n7;
        locals.var_tmf2_dn8 = assign18830_e13584_d_n8;
        locals.var_tmf2_dn9 = assign18830_e13584_d_n9;
        locals.var_tmf2_dn10 = assign18830_e13584_d_n10;
        locals.var_tmf2_dn13 = assign18830_e13584_d_n13;

        let (assign18840_e13598, assign18840_e13598_d_n0, assign18840_e13598_d_n2, assign18840_e13598_d_n4, assign18840_e13598_d_n5, assign18840_e13598_d_n6, assign18840_e13598_d_n7, assign18840_e13598_d_n8, assign18840_e13598_d_n9, assign18840_e13598_d_n10, assign18840_e13598_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign18840_e13594: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign18840_e13595: f64 = (1.0 + assign18840_e13594);
        let assign18840_e13596: f64 = (0.5 * assign18840_e13595);
        (assign18840_e13596, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign18840_e13598;
        locals.var_t6_dn0 = assign18840_e13598_d_n0;
        locals.var_t6_dn2 = assign18840_e13598_d_n2;
        locals.var_t6_dn4 = assign18840_e13598_d_n4;
        locals.var_t6_dn5 = assign18840_e13598_d_n5;
        locals.var_t6_dn6 = assign18840_e13598_d_n6;
        locals.var_t6_dn7 = assign18840_e13598_d_n7;
        locals.var_t6_dn8 = assign18840_e13598_d_n8;
        locals.var_t6_dn9 = assign18840_e13598_d_n9;
        locals.var_t6_dn10 = assign18840_e13598_d_n10;
        locals.var_t6_dn13 = assign18840_e13598_d_n13;

        let (assign18850_e13616, assign18850_e13616_d_n0, assign18850_e13616_d_n2, assign18850_e13616_d_n4, assign18850_e13616_d_n5, assign18850_e13616_d_n6, assign18850_e13616_d_n7, assign18850_e13616_d_n8, assign18850_e13616_d_n9, assign18850_e13616_d_n10, assign18850_e13616_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign18850_e13607: f64 = (p.p98 + 1.0);
        let assign18850_e13608: f64 = (locals.var_t4 * assign18850_e13607);
        let assign18850_e13612: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign18850_e13613: f64 = (0.5 * assign18850_e13612);
        let assign18850_e13614: f64 = (assign18850_e13608 - assign18850_e13613);
        (assign18850_e13614, ((locals.var_t4_dn0 * assign18850_e13607) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((locals.var_t4_dn2 * assign18850_e13607) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((locals.var_t4_dn4 * assign18850_e13607) - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), ((locals.var_t4_dn5 * assign18850_e13607) - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), ((locals.var_t4_dn6 * assign18850_e13607) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((locals.var_t4_dn7 * assign18850_e13607) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((locals.var_t4_dn8 * assign18850_e13607) - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), ((locals.var_t4_dn9 * assign18850_e13607) - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), ((locals.var_t4_dn10 * assign18850_e13607) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((locals.var_t4_dn13 * assign18850_e13607) - (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign18850_e13616;
        locals.var_t7_dn0 = assign18850_e13616_d_n0;
        locals.var_t7_dn2 = assign18850_e13616_d_n2;
        locals.var_t7_dn4 = assign18850_e13616_d_n4;
        locals.var_t7_dn5 = assign18850_e13616_d_n5;
        locals.var_t7_dn6 = assign18850_e13616_d_n6;
        locals.var_t7_dn7 = assign18850_e13616_d_n7;
        locals.var_t7_dn8 = assign18850_e13616_d_n8;
        locals.var_t7_dn9 = assign18850_e13616_d_n9;
        locals.var_t7_dn10 = assign18850_e13616_d_n10;
        locals.var_t7_dn13 = assign18850_e13616_d_n13;

        let (assign18860_e13632, assign18860_e13632_d_n0, assign18860_e13632_d_n2, assign18860_e13632_d_n4, assign18860_e13632_d_n5, assign18860_e13632_d_n6, assign18860_e13632_d_n7, assign18860_e13632_d_n8, assign18860_e13632_d_n9, assign18860_e13632_d_n10, assign18860_e13632_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign18860_e13625: f64 = (locals.var_t1 * locals.var_t4);
        let assign18860_e13626: f64 = (locals.var_t7 + assign18860_e13625);
        let assign18860_e13628: f64 = assign18860_e13626;
        let assign18860_e13630: f64 = (assign18860_e13628 - 5e-5);
        (assign18860_e13630, (locals.var_t7_dn0 + ((locals.var_t1_dn0 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn0))), (locals.var_t7_dn2 + ((locals.var_t1_dn2 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn2))), (locals.var_t7_dn4 + ((locals.var_t1_dn4 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn4))), (locals.var_t7_dn5 + ((locals.var_t1_dn5 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn5))), (locals.var_t7_dn6 + ((locals.var_t1_dn6 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn6))), (locals.var_t7_dn7 + ((locals.var_t1_dn7 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn7))), (locals.var_t7_dn8 + ((locals.var_t1_dn8 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn8))), (locals.var_t7_dn9 + ((locals.var_t1_dn9 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn9))), (locals.var_t7_dn10 + ((locals.var_t1_dn10 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn10))), (locals.var_t7_dn13 + ((locals.var_t1_dn13 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn13))),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign18860_e13632;
        locals.var_tmf1_dn0 = assign18860_e13632_d_n0;
        locals.var_tmf1_dn2 = assign18860_e13632_d_n2;
        locals.var_tmf1_dn4 = assign18860_e13632_d_n4;
        locals.var_tmf1_dn5 = assign18860_e13632_d_n5;
        locals.var_tmf1_dn6 = assign18860_e13632_d_n6;
        locals.var_tmf1_dn7 = assign18860_e13632_d_n7;
        locals.var_tmf1_dn8 = assign18860_e13632_d_n8;
        locals.var_tmf1_dn9 = assign18860_e13632_d_n9;
        locals.var_tmf1_dn10 = assign18860_e13632_d_n10;
        locals.var_tmf1_dn13 = assign18860_e13632_d_n13;

        let (assign18870_e13644, assign18870_e13644_d_n0, assign18870_e13644_d_n2, assign18870_e13644_d_n4, assign18870_e13644_d_n5, assign18870_e13644_d_n6, assign18870_e13644_d_n7, assign18870_e13644_d_n8, assign18870_e13644_d_n9, assign18870_e13644_d_n10, assign18870_e13644_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18870_e13644;
        locals.var_tmf2_dn0 = assign18870_e13644_d_n0;
        locals.var_tmf2_dn2 = assign18870_e13644_d_n2;
        locals.var_tmf2_dn4 = assign18870_e13644_d_n4;
        locals.var_tmf2_dn5 = assign18870_e13644_d_n5;
        locals.var_tmf2_dn6 = assign18870_e13644_d_n6;
        locals.var_tmf2_dn7 = assign18870_e13644_d_n7;
        locals.var_tmf2_dn8 = assign18870_e13644_d_n8;
        locals.var_tmf2_dn9 = assign18870_e13644_d_n9;
        locals.var_tmf2_dn10 = assign18870_e13644_d_n10;
        locals.var_tmf2_dn13 = assign18870_e13644_d_n13;

        let (assign18880_e13658, assign18880_e13658_d_n0, assign18880_e13658_d_n2, assign18880_e13658_d_n4, assign18880_e13658_d_n5, assign18880_e13658_d_n6, assign18880_e13658_d_n7, assign18880_e13658_d_n8, assign18880_e13658_d_n9, assign18880_e13658_d_n10, assign18880_e13658_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let (assign18880_e13656, assign18880_e13656_d_n0, assign18880_e13656_d_n2, assign18880_e13656_d_n4, assign18880_e13656_d_n5, assign18880_e13656_d_n6, assign18880_e13656_d_n7, assign18880_e13656_d_n8, assign18880_e13656_d_n9, assign18880_e13656_d_n10, assign18880_e13656_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign18880_e13655: f64 = (-locals.var_tmf2);
                (assign18880_e13655, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign18880_e13656, assign18880_e13656_d_n0, assign18880_e13656_d_n2, assign18880_e13656_d_n4, assign18880_e13656_d_n5, assign18880_e13656_d_n6, assign18880_e13656_d_n7, assign18880_e13656_d_n8, assign18880_e13656_d_n9, assign18880_e13656_d_n10, assign18880_e13656_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18880_e13658;
        locals.var_tmf2_dn0 = assign18880_e13658_d_n0;
        locals.var_tmf2_dn2 = assign18880_e13658_d_n2;
        locals.var_tmf2_dn4 = assign18880_e13658_d_n4;
        locals.var_tmf2_dn5 = assign18880_e13658_d_n5;
        locals.var_tmf2_dn6 = assign18880_e13658_d_n6;
        locals.var_tmf2_dn7 = assign18880_e13658_d_n7;
        locals.var_tmf2_dn8 = assign18880_e13658_d_n8;
        locals.var_tmf2_dn9 = assign18880_e13658_d_n9;
        locals.var_tmf2_dn10 = assign18880_e13658_d_n10;
        locals.var_tmf2_dn13 = assign18880_e13658_d_n13;

        let (assign18890_e13671, assign18890_e13671_d_n0, assign18890_e13671_d_n2, assign18890_e13671_d_n4, assign18890_e13671_d_n5, assign18890_e13671_d_n6, assign18890_e13671_d_n7, assign18890_e13671_d_n8, assign18890_e13671_d_n9, assign18890_e13671_d_n10, assign18890_e13671_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign18890_e13666: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18890_e13668: f64 = (assign18890_e13666 + locals.var_tmf2);
        let assign18890_e13669: f64 = (assign18890_e13668).sqrt();
        (assign18890_e13669, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18890_e13669)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18890_e13669)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign18890_e13669)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign18890_e13669)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign18890_e13669)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign18890_e13669)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign18890_e13669)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign18890_e13669)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign18890_e13669)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign18890_e13669)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18890_e13671;
        locals.var_tmf2_dn0 = assign18890_e13671_d_n0;
        locals.var_tmf2_dn2 = assign18890_e13671_d_n2;
        locals.var_tmf2_dn4 = assign18890_e13671_d_n4;
        locals.var_tmf2_dn5 = assign18890_e13671_d_n5;
        locals.var_tmf2_dn6 = assign18890_e13671_d_n6;
        locals.var_tmf2_dn7 = assign18890_e13671_d_n7;
        locals.var_tmf2_dn8 = assign18890_e13671_d_n8;
        locals.var_tmf2_dn9 = assign18890_e13671_d_n9;
        locals.var_tmf2_dn10 = assign18890_e13671_d_n10;
        locals.var_tmf2_dn13 = assign18890_e13671_d_n13;

        let (assign18900_e13685, assign18900_e13685_d_n0, assign18900_e13685_d_n2, assign18900_e13685_d_n4, assign18900_e13685_d_n5, assign18900_e13685_d_n6, assign18900_e13685_d_n7, assign18900_e13685_d_n8, assign18900_e13685_d_n9, assign18900_e13685_d_n10, assign18900_e13685_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign18900_e13681: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign18900_e13682: f64 = (1.0 + assign18900_e13681);
        let assign18900_e13683: f64 = (0.5 * assign18900_e13682);
        (assign18900_e13683, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign18900_e13685;
        locals.var_t6_dn0 = assign18900_e13685_d_n0;
        locals.var_t6_dn2 = assign18900_e13685_d_n2;
        locals.var_t6_dn4 = assign18900_e13685_d_n4;
        locals.var_t6_dn5 = assign18900_e13685_d_n5;
        locals.var_t6_dn6 = assign18900_e13685_d_n6;
        locals.var_t6_dn7 = assign18900_e13685_d_n7;
        locals.var_t6_dn8 = assign18900_e13685_d_n8;
        locals.var_t6_dn9 = assign18900_e13685_d_n9;
        locals.var_t6_dn10 = assign18900_e13685_d_n10;
        locals.var_t6_dn13 = assign18900_e13685_d_n13;

        let (assign18910_e13699, assign18910_e13699_d_n0, assign18910_e13699_d_n2, assign18910_e13699_d_n4, assign18910_e13699_d_n5, assign18910_e13699_d_n6, assign18910_e13699_d_n7, assign18910_e13699_d_n8, assign18910_e13699_d_n9, assign18910_e13699_d_n10, assign18910_e13699_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign18910_e13695: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign18910_e13696: f64 = (0.5 * assign18910_e13695);
        let assign18910_e13697: f64 = assign18910_e13696;
        (assign18910_e13697, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign18910_e13699;
        locals.var_t2_dn0 = assign18910_e13699_d_n0;
        locals.var_t2_dn2 = assign18910_e13699_d_n2;
        locals.var_t2_dn4 = assign18910_e13699_d_n4;
        locals.var_t2_dn5 = assign18910_e13699_d_n5;
        locals.var_t2_dn6 = assign18910_e13699_d_n6;
        locals.var_t2_dn7 = assign18910_e13699_d_n7;
        locals.var_t2_dn8 = assign18910_e13699_d_n8;
        locals.var_t2_dn9 = assign18910_e13699_d_n9;
        locals.var_t2_dn10 = assign18910_e13699_d_n10;
        locals.var_t2_dn13 = assign18910_e13699_d_n13;

        let assign18920_e13706: f64 = if ((p.p39 == 0.0) || (p.p39 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard385 = assign18920_e13706;

        let (assign18930_e13726, assign18930_e13726_d_n0, assign18930_e13726_d_n2, assign18930_e13726_d_n4, assign18930_e13726_d_n5, assign18930_e13726_d_n6, assign18930_e13726_d_n7, assign18930_e13726_d_n8, assign18930_e13726_d_n9, assign18930_e13726_d_n10, assign18930_e13726_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign18930_e13717: f64 = (locals.var_mks_rdvdtemp1 * locals.var_tdiff0);
        let assign18930_e13718: f64 = (locals.var_uc_rdvd + assign18930_e13717);
        let assign18930_e13721: f64 = (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2);
        let assign18930_e13722: f64 = (assign18930_e13718 + assign18930_e13721);
        let assign18930_e13724: f64 = (assign18930_e13722 * locals.var_t2);
        (assign18930_e13724, ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn0) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn0)) * locals.var_t2) + (assign18930_e13722 * locals.var_t2_dn0)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn2) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn2)) * locals.var_t2) + (assign18930_e13722 * locals.var_t2_dn2)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn4) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn4)) * locals.var_t2) + (assign18930_e13722 * locals.var_t2_dn4)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn5) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn5)) * locals.var_t2) + (assign18930_e13722 * locals.var_t2_dn5)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn6) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn6)) * locals.var_t2) + (assign18930_e13722 * locals.var_t2_dn6)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn7) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn7)) * locals.var_t2) + (assign18930_e13722 * locals.var_t2_dn7)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn8) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn8)) * locals.var_t2) + (assign18930_e13722 * locals.var_t2_dn8)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn9) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn9)) * locals.var_t2) + (assign18930_e13722 * locals.var_t2_dn9)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn10) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn10)) * locals.var_t2) + (assign18930_e13722 * locals.var_t2_dn10)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn13) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn13)) * locals.var_t2) + (assign18930_e13722 * locals.var_t2_dn13)),)
    } else {
        (locals.var_rdvde, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn13,)
    }
};
        locals.var_rdvde = assign18930_e13726;
        locals.var_rdvde_dn0 = assign18930_e13726_d_n0;
        locals.var_rdvde_dn2 = assign18930_e13726_d_n2;
        locals.var_rdvde_dn4 = assign18930_e13726_d_n4;
        locals.var_rdvde_dn5 = assign18930_e13726_d_n5;
        locals.var_rdvde_dn6 = assign18930_e13726_d_n6;
        locals.var_rdvde_dn7 = assign18930_e13726_d_n7;
        locals.var_rdvde_dn8 = assign18930_e13726_d_n8;
        locals.var_rdvde_dn9 = assign18930_e13726_d_n9;
        locals.var_rdvde_dn10 = assign18930_e13726_d_n10;
        locals.var_rdvde_dn13 = assign18930_e13726_d_n13;

        let (assign18940_e13744, assign18940_e13744_d_n0, assign18940_e13744_d_n2, assign18940_e13744_d_n4, assign18940_e13744_d_n5, assign18940_e13744_d_n6, assign18940_e13744_d_n7, assign18940_e13744_d_n8, assign18940_e13744_d_n9, assign18940_e13744_d_n10, assign18940_e13744_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign18940_e13737: f64 = (0.005 * locals.var_uc_rdvd);
        let assign18940_e13738: f64 = (locals.var_rdvde - assign18940_e13737);
        let assign18940_e13741: f64 = (0.01 * locals.var_uc_rdvd);
        let assign18940_e13742: f64 = (assign18940_e13738 - assign18940_e13741);
        (assign18940_e13742, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign18940_e13744;
        locals.var_tmf1_dn0 = assign18940_e13744_d_n0;
        locals.var_tmf1_dn2 = assign18940_e13744_d_n2;
        locals.var_tmf1_dn4 = assign18940_e13744_d_n4;
        locals.var_tmf1_dn5 = assign18940_e13744_d_n5;
        locals.var_tmf1_dn6 = assign18940_e13744_d_n6;
        locals.var_tmf1_dn7 = assign18940_e13744_d_n7;
        locals.var_tmf1_dn8 = assign18940_e13744_d_n8;
        locals.var_tmf1_dn9 = assign18940_e13744_d_n9;
        locals.var_tmf1_dn10 = assign18940_e13744_d_n10;
        locals.var_tmf1_dn13 = assign18940_e13744_d_n13;

    }

    pub(super) fn stamp_transient_block_42(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign18950_e13762, assign18950_e13762_d_n0, assign18950_e13762_d_n2, assign18950_e13762_d_n4, assign18950_e13762_d_n5, assign18950_e13762_d_n6, assign18950_e13762_d_n7, assign18950_e13762_d_n8, assign18950_e13762_d_n9, assign18950_e13762_d_n10, assign18950_e13762_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign18950_e13755: f64 = (0.005 * locals.var_uc_rdvd);
        let assign18950_e13756: f64 = (4.0 * assign18950_e13755);
        let assign18950_e13759: f64 = (0.01 * locals.var_uc_rdvd);
        let assign18950_e13760: f64 = (assign18950_e13756 * assign18950_e13759);
        (assign18950_e13760, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18950_e13762;
        locals.var_tmf2_dn0 = assign18950_e13762_d_n0;
        locals.var_tmf2_dn2 = assign18950_e13762_d_n2;
        locals.var_tmf2_dn4 = assign18950_e13762_d_n4;
        locals.var_tmf2_dn5 = assign18950_e13762_d_n5;
        locals.var_tmf2_dn6 = assign18950_e13762_d_n6;
        locals.var_tmf2_dn7 = assign18950_e13762_d_n7;
        locals.var_tmf2_dn8 = assign18950_e13762_d_n8;
        locals.var_tmf2_dn9 = assign18950_e13762_d_n9;
        locals.var_tmf2_dn10 = assign18950_e13762_d_n10;
        locals.var_tmf2_dn13 = assign18950_e13762_d_n13;

        let (assign18960_e13778, assign18960_e13778_d_n0, assign18960_e13778_d_n2, assign18960_e13778_d_n4, assign18960_e13778_d_n5, assign18960_e13778_d_n6, assign18960_e13778_d_n7, assign18960_e13778_d_n8, assign18960_e13778_d_n9, assign18960_e13778_d_n10, assign18960_e13778_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let (assign18960_e13776, assign18960_e13776_d_n0, assign18960_e13776_d_n2, assign18960_e13776_d_n4, assign18960_e13776_d_n5, assign18960_e13776_d_n6, assign18960_e13776_d_n7, assign18960_e13776_d_n8, assign18960_e13776_d_n9, assign18960_e13776_d_n10, assign18960_e13776_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign18960_e13775: f64 = (-locals.var_tmf2);
                (assign18960_e13775, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign18960_e13776, assign18960_e13776_d_n0, assign18960_e13776_d_n2, assign18960_e13776_d_n4, assign18960_e13776_d_n5, assign18960_e13776_d_n6, assign18960_e13776_d_n7, assign18960_e13776_d_n8, assign18960_e13776_d_n9, assign18960_e13776_d_n10, assign18960_e13776_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18960_e13778;
        locals.var_tmf2_dn0 = assign18960_e13778_d_n0;
        locals.var_tmf2_dn2 = assign18960_e13778_d_n2;
        locals.var_tmf2_dn4 = assign18960_e13778_d_n4;
        locals.var_tmf2_dn5 = assign18960_e13778_d_n5;
        locals.var_tmf2_dn6 = assign18960_e13778_d_n6;
        locals.var_tmf2_dn7 = assign18960_e13778_d_n7;
        locals.var_tmf2_dn8 = assign18960_e13778_d_n8;
        locals.var_tmf2_dn9 = assign18960_e13778_d_n9;
        locals.var_tmf2_dn10 = assign18960_e13778_d_n10;
        locals.var_tmf2_dn13 = assign18960_e13778_d_n13;

        let (assign18970_e13793, assign18970_e13793_d_n0, assign18970_e13793_d_n2, assign18970_e13793_d_n4, assign18970_e13793_d_n5, assign18970_e13793_d_n6, assign18970_e13793_d_n7, assign18970_e13793_d_n8, assign18970_e13793_d_n9, assign18970_e13793_d_n10, assign18970_e13793_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign18970_e13788: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18970_e13790: f64 = (assign18970_e13788 + locals.var_tmf2);
        let assign18970_e13791: f64 = (assign18970_e13790).sqrt();
        (assign18970_e13791, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18970_e13791)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18970_e13791)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign18970_e13791)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign18970_e13791)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign18970_e13791)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign18970_e13791)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign18970_e13791)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign18970_e13791)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign18970_e13791)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign18970_e13791)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18970_e13793;
        locals.var_tmf2_dn0 = assign18970_e13793_d_n0;
        locals.var_tmf2_dn2 = assign18970_e13793_d_n2;
        locals.var_tmf2_dn4 = assign18970_e13793_d_n4;
        locals.var_tmf2_dn5 = assign18970_e13793_d_n5;
        locals.var_tmf2_dn6 = assign18970_e13793_d_n6;
        locals.var_tmf2_dn7 = assign18970_e13793_d_n7;
        locals.var_tmf2_dn8 = assign18970_e13793_d_n8;
        locals.var_tmf2_dn9 = assign18970_e13793_d_n9;
        locals.var_tmf2_dn10 = assign18970_e13793_d_n10;
        locals.var_tmf2_dn13 = assign18970_e13793_d_n13;

        let (assign18980_e13809, assign18980_e13809_d_n0, assign18980_e13809_d_n2, assign18980_e13809_d_n4, assign18980_e13809_d_n5, assign18980_e13809_d_n6, assign18980_e13809_d_n7, assign18980_e13809_d_n8, assign18980_e13809_d_n9, assign18980_e13809_d_n10, assign18980_e13809_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign18980_e13805: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign18980_e13806: f64 = (1.0 + assign18980_e13805);
        let assign18980_e13807: f64 = (0.5 * assign18980_e13806);
        (assign18980_e13807, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign18980_e13809;
        locals.var_t0_dn0 = assign18980_e13809_d_n0;
        locals.var_t0_dn2 = assign18980_e13809_d_n2;
        locals.var_t0_dn4 = assign18980_e13809_d_n4;
        locals.var_t0_dn5 = assign18980_e13809_d_n5;
        locals.var_t0_dn6 = assign18980_e13809_d_n6;
        locals.var_t0_dn7 = assign18980_e13809_d_n7;
        locals.var_t0_dn8 = assign18980_e13809_d_n8;
        locals.var_t0_dn9 = assign18980_e13809_d_n9;
        locals.var_t0_dn10 = assign18980_e13809_d_n10;
        locals.var_t0_dn13 = assign18980_e13809_d_n13;

        let (assign18990_e13827, assign18990_e13827_d_n0, assign18990_e13827_d_n2, assign18990_e13827_d_n4, assign18990_e13827_d_n5, assign18990_e13827_d_n6, assign18990_e13827_d_n7, assign18990_e13827_d_n8, assign18990_e13827_d_n9, assign18990_e13827_d_n10, assign18990_e13827_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign18990_e13819: f64 = (0.005 * locals.var_uc_rdvd);
        let assign18990_e13823: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign18990_e13824: f64 = (0.5 * assign18990_e13823);
        let assign18990_e13825: f64 = (assign18990_e13819 + assign18990_e13824);
        (assign18990_e13825, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_rdvde, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn13,)
    }
};
        locals.var_rdvde = assign18990_e13827;
        locals.var_rdvde_dn0 = assign18990_e13827_d_n0;
        locals.var_rdvde_dn2 = assign18990_e13827_d_n2;
        locals.var_rdvde_dn4 = assign18990_e13827_d_n4;
        locals.var_rdvde_dn5 = assign18990_e13827_d_n5;
        locals.var_rdvde_dn6 = assign18990_e13827_d_n6;
        locals.var_rdvde_dn7 = assign18990_e13827_d_n7;
        locals.var_rdvde_dn8 = assign18990_e13827_d_n8;
        locals.var_rdvde_dn9 = assign18990_e13827_d_n9;
        locals.var_rdvde_dn10 = assign18990_e13827_d_n10;
        locals.var_rdvde_dn13 = assign18990_e13827_d_n13;

        let (assign19000_e13848, assign19000_e13848_d_n0, assign19000_e13848_d_n2, assign19000_e13848_d_n4, assign19000_e13848_d_n5, assign19000_e13848_d_n6, assign19000_e13848_d_n7, assign19000_e13848_d_n8, assign19000_e13848_d_n9, assign19000_e13848_d_n10, assign19000_e13848_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard385 == 0.0)) {
        let assign19000_e13839: f64 = (locals.var_mks_rdvdtemp1 * locals.var_tdiff);
        let assign19000_e13840: f64 = (locals.var_uc_rdvd + assign19000_e13839);
        let assign19000_e13843: f64 = (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2);
        let assign19000_e13844: f64 = (assign19000_e13840 + assign19000_e13843);
        let assign19000_e13846: f64 = (assign19000_e13844 * locals.var_t2);
        (assign19000_e13846, ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn0) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn0)) * locals.var_t2) + (assign19000_e13844 * locals.var_t2_dn0)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn2) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn2)) * locals.var_t2) + (assign19000_e13844 * locals.var_t2_dn2)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn4) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn4)) * locals.var_t2) + (assign19000_e13844 * locals.var_t2_dn4)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn5) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn5)) * locals.var_t2) + (assign19000_e13844 * locals.var_t2_dn5)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn6) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn6)) * locals.var_t2) + (assign19000_e13844 * locals.var_t2_dn6)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn7) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn7)) * locals.var_t2) + (assign19000_e13844 * locals.var_t2_dn7)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn8) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn8)) * locals.var_t2) + (assign19000_e13844 * locals.var_t2_dn8)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn9) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn9)) * locals.var_t2) + (assign19000_e13844 * locals.var_t2_dn9)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn10) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn10)) * locals.var_t2) + (assign19000_e13844 * locals.var_t2_dn10)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn13) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn13)) * locals.var_t2) + (assign19000_e13844 * locals.var_t2_dn13)),)
    } else {
        (locals.var_rdvde, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn13,)
    }
};
        locals.var_rdvde = assign19000_e13848;
        locals.var_rdvde_dn0 = assign19000_e13848_d_n0;
        locals.var_rdvde_dn2 = assign19000_e13848_d_n2;
        locals.var_rdvde_dn4 = assign19000_e13848_d_n4;
        locals.var_rdvde_dn5 = assign19000_e13848_d_n5;
        locals.var_rdvde_dn6 = assign19000_e13848_d_n6;
        locals.var_rdvde_dn7 = assign19000_e13848_d_n7;
        locals.var_rdvde_dn8 = assign19000_e13848_d_n8;
        locals.var_rdvde_dn9 = assign19000_e13848_d_n9;
        locals.var_rdvde_dn10 = assign19000_e13848_d_n10;
        locals.var_rdvde_dn13 = assign19000_e13848_d_n13;

        let (assign19010_e13867, assign19010_e13867_d_n0, assign19010_e13867_d_n2, assign19010_e13867_d_n4, assign19010_e13867_d_n5, assign19010_e13867_d_n6, assign19010_e13867_d_n7, assign19010_e13867_d_n8, assign19010_e13867_d_n9, assign19010_e13867_d_n10, assign19010_e13867_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard385 == 0.0)) {
        let assign19010_e13860: f64 = (0.005 * locals.var_uc_rdvd);
        let assign19010_e13861: f64 = (locals.var_rdvde - assign19010_e13860);
        let assign19010_e13864: f64 = (0.01 * locals.var_uc_rdvd);
        let assign19010_e13865: f64 = (assign19010_e13861 - assign19010_e13864);
        (assign19010_e13865, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign19010_e13867;
        locals.var_tmf1_dn0 = assign19010_e13867_d_n0;
        locals.var_tmf1_dn2 = assign19010_e13867_d_n2;
        locals.var_tmf1_dn4 = assign19010_e13867_d_n4;
        locals.var_tmf1_dn5 = assign19010_e13867_d_n5;
        locals.var_tmf1_dn6 = assign19010_e13867_d_n6;
        locals.var_tmf1_dn7 = assign19010_e13867_d_n7;
        locals.var_tmf1_dn8 = assign19010_e13867_d_n8;
        locals.var_tmf1_dn9 = assign19010_e13867_d_n9;
        locals.var_tmf1_dn10 = assign19010_e13867_d_n10;
        locals.var_tmf1_dn13 = assign19010_e13867_d_n13;

        let (assign19020_e13886, assign19020_e13886_d_n0, assign19020_e13886_d_n2, assign19020_e13886_d_n4, assign19020_e13886_d_n5, assign19020_e13886_d_n6, assign19020_e13886_d_n7, assign19020_e13886_d_n8, assign19020_e13886_d_n9, assign19020_e13886_d_n10, assign19020_e13886_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard385 == 0.0)) {
        let assign19020_e13879: f64 = (0.005 * locals.var_uc_rdvd);
        let assign19020_e13880: f64 = (4.0 * assign19020_e13879);
        let assign19020_e13883: f64 = (0.01 * locals.var_uc_rdvd);
        let assign19020_e13884: f64 = (assign19020_e13880 * assign19020_e13883);
        (assign19020_e13884, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign19020_e13886;
        locals.var_tmf2_dn0 = assign19020_e13886_d_n0;
        locals.var_tmf2_dn2 = assign19020_e13886_d_n2;
        locals.var_tmf2_dn4 = assign19020_e13886_d_n4;
        locals.var_tmf2_dn5 = assign19020_e13886_d_n5;
        locals.var_tmf2_dn6 = assign19020_e13886_d_n6;
        locals.var_tmf2_dn7 = assign19020_e13886_d_n7;
        locals.var_tmf2_dn8 = assign19020_e13886_d_n8;
        locals.var_tmf2_dn9 = assign19020_e13886_d_n9;
        locals.var_tmf2_dn10 = assign19020_e13886_d_n10;
        locals.var_tmf2_dn13 = assign19020_e13886_d_n13;

        let (assign19030_e13903, assign19030_e13903_d_n0, assign19030_e13903_d_n2, assign19030_e13903_d_n4, assign19030_e13903_d_n5, assign19030_e13903_d_n6, assign19030_e13903_d_n7, assign19030_e13903_d_n8, assign19030_e13903_d_n9, assign19030_e13903_d_n10, assign19030_e13903_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard385 == 0.0)) {
        let (assign19030_e13901, assign19030_e13901_d_n0, assign19030_e13901_d_n2, assign19030_e13901_d_n4, assign19030_e13901_d_n5, assign19030_e13901_d_n6, assign19030_e13901_d_n7, assign19030_e13901_d_n8, assign19030_e13901_d_n9, assign19030_e13901_d_n10, assign19030_e13901_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign19030_e13900: f64 = (-locals.var_tmf2);
                (assign19030_e13900, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign19030_e13901, assign19030_e13901_d_n0, assign19030_e13901_d_n2, assign19030_e13901_d_n4, assign19030_e13901_d_n5, assign19030_e13901_d_n6, assign19030_e13901_d_n7, assign19030_e13901_d_n8, assign19030_e13901_d_n9, assign19030_e13901_d_n10, assign19030_e13901_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign19030_e13903;
        locals.var_tmf2_dn0 = assign19030_e13903_d_n0;
        locals.var_tmf2_dn2 = assign19030_e13903_d_n2;
        locals.var_tmf2_dn4 = assign19030_e13903_d_n4;
        locals.var_tmf2_dn5 = assign19030_e13903_d_n5;
        locals.var_tmf2_dn6 = assign19030_e13903_d_n6;
        locals.var_tmf2_dn7 = assign19030_e13903_d_n7;
        locals.var_tmf2_dn8 = assign19030_e13903_d_n8;
        locals.var_tmf2_dn9 = assign19030_e13903_d_n9;
        locals.var_tmf2_dn10 = assign19030_e13903_d_n10;
        locals.var_tmf2_dn13 = assign19030_e13903_d_n13;

        let (assign19040_e13919, assign19040_e13919_d_n0, assign19040_e13919_d_n2, assign19040_e13919_d_n4, assign19040_e13919_d_n5, assign19040_e13919_d_n6, assign19040_e13919_d_n7, assign19040_e13919_d_n8, assign19040_e13919_d_n9, assign19040_e13919_d_n10, assign19040_e13919_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard385 == 0.0)) {
        let assign19040_e13914: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign19040_e13916: f64 = (assign19040_e13914 + locals.var_tmf2);
        let assign19040_e13917: f64 = (assign19040_e13916).sqrt();
        (assign19040_e13917, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign19040_e13917)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign19040_e13917)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign19040_e13917)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign19040_e13917)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign19040_e13917)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign19040_e13917)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign19040_e13917)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign19040_e13917)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign19040_e13917)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign19040_e13917)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign19040_e13919;
        locals.var_tmf2_dn0 = assign19040_e13919_d_n0;
        locals.var_tmf2_dn2 = assign19040_e13919_d_n2;
        locals.var_tmf2_dn4 = assign19040_e13919_d_n4;
        locals.var_tmf2_dn5 = assign19040_e13919_d_n5;
        locals.var_tmf2_dn6 = assign19040_e13919_d_n6;
        locals.var_tmf2_dn7 = assign19040_e13919_d_n7;
        locals.var_tmf2_dn8 = assign19040_e13919_d_n8;
        locals.var_tmf2_dn9 = assign19040_e13919_d_n9;
        locals.var_tmf2_dn10 = assign19040_e13919_d_n10;
        locals.var_tmf2_dn13 = assign19040_e13919_d_n13;

        let (assign19050_e13936, assign19050_e13936_d_n0, assign19050_e13936_d_n2, assign19050_e13936_d_n4, assign19050_e13936_d_n5, assign19050_e13936_d_n6, assign19050_e13936_d_n7, assign19050_e13936_d_n8, assign19050_e13936_d_n9, assign19050_e13936_d_n10, assign19050_e13936_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard385 == 0.0)) {
        let assign19050_e13932: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign19050_e13933: f64 = (1.0 + assign19050_e13932);
        let assign19050_e13934: f64 = (0.5 * assign19050_e13933);
        (assign19050_e13934, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign19050_e13936;
        locals.var_t0_dn0 = assign19050_e13936_d_n0;
        locals.var_t0_dn2 = assign19050_e13936_d_n2;
        locals.var_t0_dn4 = assign19050_e13936_d_n4;
        locals.var_t0_dn5 = assign19050_e13936_d_n5;
        locals.var_t0_dn6 = assign19050_e13936_d_n6;
        locals.var_t0_dn7 = assign19050_e13936_d_n7;
        locals.var_t0_dn8 = assign19050_e13936_d_n8;
        locals.var_t0_dn9 = assign19050_e13936_d_n9;
        locals.var_t0_dn10 = assign19050_e13936_d_n10;
        locals.var_t0_dn13 = assign19050_e13936_d_n13;

        let (assign19060_e13955, assign19060_e13955_d_n0, assign19060_e13955_d_n2, assign19060_e13955_d_n4, assign19060_e13955_d_n5, assign19060_e13955_d_n6, assign19060_e13955_d_n7, assign19060_e13955_d_n8, assign19060_e13955_d_n9, assign19060_e13955_d_n10, assign19060_e13955_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard385 == 0.0)) {
        let assign19060_e13947: f64 = (0.005 * locals.var_uc_rdvd);
        let assign19060_e13951: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign19060_e13952: f64 = (0.5 * assign19060_e13951);
        let assign19060_e13953: f64 = (assign19060_e13947 + assign19060_e13952);
        (assign19060_e13953, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_rdvde, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn13,)
    }
};
        locals.var_rdvde = assign19060_e13955;
        locals.var_rdvde_dn0 = assign19060_e13955_d_n0;
        locals.var_rdvde_dn2 = assign19060_e13955_d_n2;
        locals.var_rdvde_dn4 = assign19060_e13955_d_n4;
        locals.var_rdvde_dn5 = assign19060_e13955_d_n5;
        locals.var_rdvde_dn6 = assign19060_e13955_d_n6;
        locals.var_rdvde_dn7 = assign19060_e13955_d_n7;
        locals.var_rdvde_dn8 = assign19060_e13955_d_n8;
        locals.var_rdvde_dn9 = assign19060_e13955_d_n9;
        locals.var_rdvde_dn10 = assign19060_e13955_d_n10;
        locals.var_rdvde_dn13 = assign19060_e13955_d_n13;

        let (assign19070_e13979, assign19070_e13979_d_n0, assign19070_e13979_d_n2, assign19070_e13979_d_n4, assign19070_e13979_d_n5, assign19070_e13979_d_n6, assign19070_e13979_d_n7, assign19070_e13979_d_n8, assign19070_e13979_d_n9, assign19070_e13979_d_n10, assign19070_e13979_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign19070_e13964: f64 = (p.p69 * locals.var_uc_rdslp1);
        let assign19070_e13966: f64 = (assign19070_e13964 * 1000000.0);
        let assign19070_e13968: f64 = (assign19070_e13966 + locals.var_uc_rdict1);
        let assign19070_e13969: f64 = (locals.var_rdvdtemp0 * assign19070_e13968);
        let assign19070_e13972: f64 = (p.p70 * p.p100);
        let assign19070_e13974: f64 = (assign19070_e13972 * 1000000.0);
        let assign19070_e13976: f64 = (assign19070_e13974 + p.p101);
        let assign19070_e13977: f64 = (assign19070_e13969 * assign19070_e13976);
        (assign19070_e13977, ((locals.var_rdvdtemp0_dn0 * assign19070_e13968) * assign19070_e13976), ((locals.var_rdvdtemp0_dn2 * assign19070_e13968) * assign19070_e13976), ((locals.var_rdvdtemp0_dn4 * assign19070_e13968) * assign19070_e13976), ((locals.var_rdvdtemp0_dn5 * assign19070_e13968) * assign19070_e13976), ((locals.var_rdvdtemp0_dn6 * assign19070_e13968) * assign19070_e13976), ((locals.var_rdvdtemp0_dn7 * assign19070_e13968) * assign19070_e13976), ((locals.var_rdvdtemp0_dn8 * assign19070_e13968) * assign19070_e13976), ((locals.var_rdvdtemp0_dn9 * assign19070_e13968) * assign19070_e13976), ((locals.var_rdvdtemp0_dn10 * assign19070_e13968) * assign19070_e13976), ((locals.var_rdvdtemp0_dn13 * assign19070_e13968) * assign19070_e13976),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign19070_e13979;
        locals.var_t4_dn0 = assign19070_e13979_d_n0;
        locals.var_t4_dn2 = assign19070_e13979_d_n2;
        locals.var_t4_dn4 = assign19070_e13979_d_n4;
        locals.var_t4_dn5 = assign19070_e13979_d_n5;
        locals.var_t4_dn6 = assign19070_e13979_d_n6;
        locals.var_t4_dn7 = assign19070_e13979_d_n7;
        locals.var_t4_dn8 = assign19070_e13979_d_n8;
        locals.var_t4_dn9 = assign19070_e13979_d_n9;
        locals.var_t4_dn10 = assign19070_e13979_d_n10;
        locals.var_t4_dn13 = assign19070_e13979_d_n13;

        let (assign19080_e13993, assign19080_e13993_d_n0, assign19080_e13993_d_n2, assign19080_e13993_d_n4, assign19080_e13993_d_n5, assign19080_e13993_d_n6, assign19080_e13993_d_n7, assign19080_e13993_d_n8, assign19080_e13993_d_n9, assign19080_e13993_d_n10, assign19080_e13993_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign19080_e13987: f64 = (1.0 - locals.var_uc_rdov13);
        let assign19080_e13989: f64 = (assign19080_e13987 * p.p66);
        let assign19080_e13991: f64 = (assign19080_e13989 * 1000000.0);
        (assign19080_e13991, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign19080_e13993;
        locals.var_t1_dn0 = assign19080_e13993_d_n0;
        locals.var_t1_dn2 = assign19080_e13993_d_n2;
        locals.var_t1_dn4 = assign19080_e13993_d_n4;
        locals.var_t1_dn5 = assign19080_e13993_d_n5;
        locals.var_t1_dn6 = assign19080_e13993_d_n6;
        locals.var_t1_dn7 = assign19080_e13993_d_n7;
        locals.var_t1_dn8 = assign19080_e13993_d_n8;
        locals.var_t1_dn9 = assign19080_e13993_d_n9;
        locals.var_t1_dn10 = assign19080_e13993_d_n10;
        locals.var_t1_dn13 = assign19080_e13993_d_n13;

        let (assign19090_e14009, assign19090_e14009_d_n0, assign19090_e14009_d_n2, assign19090_e14009_d_n4, assign19090_e14009_d_n5, assign19090_e14009_d_n6, assign19090_e14009_d_n7, assign19090_e14009_d_n8, assign19090_e14009_d_n9, assign19090_e14009_d_n10, assign19090_e14009_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign19090_e14001: f64 = (locals.var_t8 * p.p66);
        let assign19090_e14003: f64 = (assign19090_e14001 * 1000000.0);
        let assign19090_e14005: f64 = (assign19090_e14003 + 1.0);
        let assign19090_e14007: f64 = (assign19090_e14005 + p.p98);
        (assign19090_e14007, ((locals.var_t8_dn0 * p.p66) * 1000000.0), ((locals.var_t8_dn2 * p.p66) * 1000000.0), ((locals.var_t8_dn4 * p.p66) * 1000000.0), ((locals.var_t8_dn5 * p.p66) * 1000000.0), ((locals.var_t8_dn6 * p.p66) * 1000000.0), ((locals.var_t8_dn7 * p.p66) * 1000000.0), ((locals.var_t8_dn8 * p.p66) * 1000000.0), ((locals.var_t8_dn9 * p.p66) * 1000000.0), ((locals.var_t8_dn10 * p.p66) * 1000000.0), ((locals.var_t8_dn13 * p.p66) * 1000000.0),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign19090_e14009;
        locals.var_t3_dn0 = assign19090_e14009_d_n0;
        locals.var_t3_dn2 = assign19090_e14009_d_n2;
        locals.var_t3_dn4 = assign19090_e14009_d_n4;
        locals.var_t3_dn5 = assign19090_e14009_d_n5;
        locals.var_t3_dn6 = assign19090_e14009_d_n6;
        locals.var_t3_dn7 = assign19090_e14009_d_n7;
        locals.var_t3_dn8 = assign19090_e14009_d_n8;
        locals.var_t3_dn9 = assign19090_e14009_d_n9;
        locals.var_t3_dn10 = assign19090_e14009_d_n10;
        locals.var_t3_dn13 = assign19090_e14009_d_n13;

        let (assign19100_e14023, assign19100_e14023_d_n0, assign19100_e14023_d_n2, assign19100_e14023_d_n4, assign19100_e14023_d_n5, assign19100_e14023_d_n6, assign19100_e14023_d_n7, assign19100_e14023_d_n8, assign19100_e14023_d_n9, assign19100_e14023_d_n10, assign19100_e14023_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign19100_e14017: f64 = (locals.var_t3 * locals.var_t4);
        let assign19100_e14019: f64 = (assign19100_e14017 - locals.var_t4);
        let assign19100_e14021: f64 = (assign19100_e14019 - 0.01);
        (assign19100_e14021, (((locals.var_t3_dn0 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn0)) - locals.var_t4_dn0), (((locals.var_t3_dn2 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn2)) - locals.var_t4_dn2), (((locals.var_t3_dn4 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn4)) - locals.var_t4_dn4), (((locals.var_t3_dn5 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn5)) - locals.var_t4_dn5), (((locals.var_t3_dn6 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn6)) - locals.var_t4_dn6), (((locals.var_t3_dn7 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn7)) - locals.var_t4_dn7), (((locals.var_t3_dn8 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn8)) - locals.var_t4_dn8), (((locals.var_t3_dn9 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn9)) - locals.var_t4_dn9), (((locals.var_t3_dn10 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn10)) - locals.var_t4_dn10), (((locals.var_t3_dn13 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn13)) - locals.var_t4_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign19100_e14023;
        locals.var_tmf1_dn0 = assign19100_e14023_d_n0;
        locals.var_tmf1_dn2 = assign19100_e14023_d_n2;
        locals.var_tmf1_dn4 = assign19100_e14023_d_n4;
        locals.var_tmf1_dn5 = assign19100_e14023_d_n5;
        locals.var_tmf1_dn6 = assign19100_e14023_d_n6;
        locals.var_tmf1_dn7 = assign19100_e14023_d_n7;
        locals.var_tmf1_dn8 = assign19100_e14023_d_n8;
        locals.var_tmf1_dn9 = assign19100_e14023_d_n9;
        locals.var_tmf1_dn10 = assign19100_e14023_d_n10;
        locals.var_tmf1_dn13 = assign19100_e14023_d_n13;

        let (assign19110_e14035, assign19110_e14035_d_n0, assign19110_e14035_d_n2, assign19110_e14035_d_n4, assign19110_e14035_d_n5, assign19110_e14035_d_n6, assign19110_e14035_d_n7, assign19110_e14035_d_n8, assign19110_e14035_d_n9, assign19110_e14035_d_n10, assign19110_e14035_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign19110_e14031: f64 = (4.0 * locals.var_t4);
        let assign19110_e14033: f64 = (assign19110_e14031 * 0.01);
        (assign19110_e14033, ((4.0 * locals.var_t4_dn0) * 0.01), ((4.0 * locals.var_t4_dn2) * 0.01), ((4.0 * locals.var_t4_dn4) * 0.01), ((4.0 * locals.var_t4_dn5) * 0.01), ((4.0 * locals.var_t4_dn6) * 0.01), ((4.0 * locals.var_t4_dn7) * 0.01), ((4.0 * locals.var_t4_dn8) * 0.01), ((4.0 * locals.var_t4_dn9) * 0.01), ((4.0 * locals.var_t4_dn10) * 0.01), ((4.0 * locals.var_t4_dn13) * 0.01),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign19110_e14035;
        locals.var_tmf2_dn0 = assign19110_e14035_d_n0;
        locals.var_tmf2_dn2 = assign19110_e14035_d_n2;
        locals.var_tmf2_dn4 = assign19110_e14035_d_n4;
        locals.var_tmf2_dn5 = assign19110_e14035_d_n5;
        locals.var_tmf2_dn6 = assign19110_e14035_d_n6;
        locals.var_tmf2_dn7 = assign19110_e14035_d_n7;
        locals.var_tmf2_dn8 = assign19110_e14035_d_n8;
        locals.var_tmf2_dn9 = assign19110_e14035_d_n9;
        locals.var_tmf2_dn10 = assign19110_e14035_d_n10;
        locals.var_tmf2_dn13 = assign19110_e14035_d_n13;

        let (assign19120_e14049, assign19120_e14049_d_n0, assign19120_e14049_d_n2, assign19120_e14049_d_n4, assign19120_e14049_d_n5, assign19120_e14049_d_n6, assign19120_e14049_d_n7, assign19120_e14049_d_n8, assign19120_e14049_d_n9, assign19120_e14049_d_n10, assign19120_e14049_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let (assign19120_e14047, assign19120_e14047_d_n0, assign19120_e14047_d_n2, assign19120_e14047_d_n4, assign19120_e14047_d_n5, assign19120_e14047_d_n6, assign19120_e14047_d_n7, assign19120_e14047_d_n8, assign19120_e14047_d_n9, assign19120_e14047_d_n10, assign19120_e14047_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign19120_e14046: f64 = (-locals.var_tmf2);
                (assign19120_e14046, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign19120_e14047, assign19120_e14047_d_n0, assign19120_e14047_d_n2, assign19120_e14047_d_n4, assign19120_e14047_d_n5, assign19120_e14047_d_n6, assign19120_e14047_d_n7, assign19120_e14047_d_n8, assign19120_e14047_d_n9, assign19120_e14047_d_n10, assign19120_e14047_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign19120_e14049;
        locals.var_tmf2_dn0 = assign19120_e14049_d_n0;
        locals.var_tmf2_dn2 = assign19120_e14049_d_n2;
        locals.var_tmf2_dn4 = assign19120_e14049_d_n4;
        locals.var_tmf2_dn5 = assign19120_e14049_d_n5;
        locals.var_tmf2_dn6 = assign19120_e14049_d_n6;
        locals.var_tmf2_dn7 = assign19120_e14049_d_n7;
        locals.var_tmf2_dn8 = assign19120_e14049_d_n8;
        locals.var_tmf2_dn9 = assign19120_e14049_d_n9;
        locals.var_tmf2_dn10 = assign19120_e14049_d_n10;
        locals.var_tmf2_dn13 = assign19120_e14049_d_n13;

        let (assign19130_e14062, assign19130_e14062_d_n0, assign19130_e14062_d_n2, assign19130_e14062_d_n4, assign19130_e14062_d_n5, assign19130_e14062_d_n6, assign19130_e14062_d_n7, assign19130_e14062_d_n8, assign19130_e14062_d_n9, assign19130_e14062_d_n10, assign19130_e14062_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign19130_e14057: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign19130_e14059: f64 = (assign19130_e14057 + locals.var_tmf2);
        let assign19130_e14060: f64 = (assign19130_e14059).sqrt();
        (assign19130_e14060, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign19130_e14060)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign19130_e14060)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign19130_e14060)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign19130_e14060)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign19130_e14060)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign19130_e14060)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign19130_e14060)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign19130_e14060)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign19130_e14060)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign19130_e14060)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign19130_e14062;
        locals.var_tmf2_dn0 = assign19130_e14062_d_n0;
        locals.var_tmf2_dn2 = assign19130_e14062_d_n2;
        locals.var_tmf2_dn4 = assign19130_e14062_d_n4;
        locals.var_tmf2_dn5 = assign19130_e14062_d_n5;
        locals.var_tmf2_dn6 = assign19130_e14062_d_n6;
        locals.var_tmf2_dn7 = assign19130_e14062_d_n7;
        locals.var_tmf2_dn8 = assign19130_e14062_d_n8;
        locals.var_tmf2_dn9 = assign19130_e14062_d_n9;
        locals.var_tmf2_dn10 = assign19130_e14062_d_n10;
        locals.var_tmf2_dn13 = assign19130_e14062_d_n13;

        let (assign19140_e14076, assign19140_e14076_d_n0, assign19140_e14076_d_n2, assign19140_e14076_d_n4, assign19140_e14076_d_n5, assign19140_e14076_d_n6, assign19140_e14076_d_n7, assign19140_e14076_d_n8, assign19140_e14076_d_n9, assign19140_e14076_d_n10, assign19140_e14076_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign19140_e14072: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign19140_e14073: f64 = (1.0 + assign19140_e14072);
        let assign19140_e14074: f64 = (0.5 * assign19140_e14073);
        (assign19140_e14074, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign19140_e14076;
        locals.var_t6_dn0 = assign19140_e14076_d_n0;
        locals.var_t6_dn2 = assign19140_e14076_d_n2;
        locals.var_t6_dn4 = assign19140_e14076_d_n4;
        locals.var_t6_dn5 = assign19140_e14076_d_n5;
        locals.var_t6_dn6 = assign19140_e14076_d_n6;
        locals.var_t6_dn7 = assign19140_e14076_d_n7;
        locals.var_t6_dn8 = assign19140_e14076_d_n8;
        locals.var_t6_dn9 = assign19140_e14076_d_n9;
        locals.var_t6_dn10 = assign19140_e14076_d_n10;
        locals.var_t6_dn13 = assign19140_e14076_d_n13;

        let (assign19150_e14090, assign19150_e14090_d_n0, assign19150_e14090_d_n2, assign19150_e14090_d_n4, assign19150_e14090_d_n5, assign19150_e14090_d_n6, assign19150_e14090_d_n7, assign19150_e14090_d_n8, assign19150_e14090_d_n9, assign19150_e14090_d_n10, assign19150_e14090_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign19150_e14086: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign19150_e14087: f64 = (0.5 * assign19150_e14086);
        let assign19150_e14088: f64 = (locals.var_t4 + assign19150_e14087);
        (assign19150_e14088, (locals.var_t4_dn0 + (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t4_dn2 + (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t4_dn4 + (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t4_dn5 + (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t4_dn6 + (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t4_dn7 + (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t4_dn8 + (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t4_dn9 + (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t4_dn10 + (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t4_dn13 + (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign19150_e14090;
        locals.var_t5_dn0 = assign19150_e14090_d_n0;
        locals.var_t5_dn2 = assign19150_e14090_d_n2;
        locals.var_t5_dn4 = assign19150_e14090_d_n4;
        locals.var_t5_dn5 = assign19150_e14090_d_n5;
        locals.var_t5_dn6 = assign19150_e14090_d_n6;
        locals.var_t5_dn7 = assign19150_e14090_d_n7;
        locals.var_t5_dn8 = assign19150_e14090_d_n8;
        locals.var_t5_dn9 = assign19150_e14090_d_n9;
        locals.var_t5_dn10 = assign19150_e14090_d_n10;
        locals.var_t5_dn13 = assign19150_e14090_d_n13;

        let (assign19160_e14106, assign19160_e14106_d_n0, assign19160_e14106_d_n2, assign19160_e14106_d_n4, assign19160_e14106_d_n5, assign19160_e14106_d_n6, assign19160_e14106_d_n7, assign19160_e14106_d_n8, assign19160_e14106_d_n9, assign19160_e14106_d_n10, assign19160_e14106_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign19160_e14099: f64 = (p.p98 + 1.0);
        let assign19160_e14100: f64 = (locals.var_t4 * assign19160_e14099);
        let assign19160_e14102: f64 = (assign19160_e14100 - locals.var_t5);
        let assign19160_e14104: f64 = (assign19160_e14102 - 5e-5);
        (assign19160_e14104, ((locals.var_t4_dn0 * assign19160_e14099) - locals.var_t5_dn0), ((locals.var_t4_dn2 * assign19160_e14099) - locals.var_t5_dn2), ((locals.var_t4_dn4 * assign19160_e14099) - locals.var_t5_dn4), ((locals.var_t4_dn5 * assign19160_e14099) - locals.var_t5_dn5), ((locals.var_t4_dn6 * assign19160_e14099) - locals.var_t5_dn6), ((locals.var_t4_dn7 * assign19160_e14099) - locals.var_t5_dn7), ((locals.var_t4_dn8 * assign19160_e14099) - locals.var_t5_dn8), ((locals.var_t4_dn9 * assign19160_e14099) - locals.var_t5_dn9), ((locals.var_t4_dn10 * assign19160_e14099) - locals.var_t5_dn10), ((locals.var_t4_dn13 * assign19160_e14099) - locals.var_t5_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign19160_e14106;
        locals.var_tmf1_dn0 = assign19160_e14106_d_n0;
        locals.var_tmf1_dn2 = assign19160_e14106_d_n2;
        locals.var_tmf1_dn4 = assign19160_e14106_d_n4;
        locals.var_tmf1_dn5 = assign19160_e14106_d_n5;
        locals.var_tmf1_dn6 = assign19160_e14106_d_n6;
        locals.var_tmf1_dn7 = assign19160_e14106_d_n7;
        locals.var_tmf1_dn8 = assign19160_e14106_d_n8;
        locals.var_tmf1_dn9 = assign19160_e14106_d_n9;
        locals.var_tmf1_dn10 = assign19160_e14106_d_n10;
        locals.var_tmf1_dn13 = assign19160_e14106_d_n13;

    }

    pub(super) fn stamp_transient_block_43(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign19170_e14122, assign19170_e14122_d_n0, assign19170_e14122_d_n2, assign19170_e14122_d_n4, assign19170_e14122_d_n5, assign19170_e14122_d_n6, assign19170_e14122_d_n7, assign19170_e14122_d_n8, assign19170_e14122_d_n9, assign19170_e14122_d_n10, assign19170_e14122_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign19170_e14116: f64 = (p.p98 + 1.0);
        let assign19170_e14117: f64 = (locals.var_t4 * assign19170_e14116);
        let assign19170_e14118: f64 = (4.0 * assign19170_e14117);
        let assign19170_e14120: f64 = (assign19170_e14118 * 5e-5);
        (assign19170_e14120, ((4.0 * (locals.var_t4_dn0 * assign19170_e14116)) * 5e-5), ((4.0 * (locals.var_t4_dn2 * assign19170_e14116)) * 5e-5), ((4.0 * (locals.var_t4_dn4 * assign19170_e14116)) * 5e-5), ((4.0 * (locals.var_t4_dn5 * assign19170_e14116)) * 5e-5), ((4.0 * (locals.var_t4_dn6 * assign19170_e14116)) * 5e-5), ((4.0 * (locals.var_t4_dn7 * assign19170_e14116)) * 5e-5), ((4.0 * (locals.var_t4_dn8 * assign19170_e14116)) * 5e-5), ((4.0 * (locals.var_t4_dn9 * assign19170_e14116)) * 5e-5), ((4.0 * (locals.var_t4_dn10 * assign19170_e14116)) * 5e-5), ((4.0 * (locals.var_t4_dn13 * assign19170_e14116)) * 5e-5),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign19170_e14122;
        locals.var_tmf2_dn0 = assign19170_e14122_d_n0;
        locals.var_tmf2_dn2 = assign19170_e14122_d_n2;
        locals.var_tmf2_dn4 = assign19170_e14122_d_n4;
        locals.var_tmf2_dn5 = assign19170_e14122_d_n5;
        locals.var_tmf2_dn6 = assign19170_e14122_d_n6;
        locals.var_tmf2_dn7 = assign19170_e14122_d_n7;
        locals.var_tmf2_dn8 = assign19170_e14122_d_n8;
        locals.var_tmf2_dn9 = assign19170_e14122_d_n9;
        locals.var_tmf2_dn10 = assign19170_e14122_d_n10;
        locals.var_tmf2_dn13 = assign19170_e14122_d_n13;

        let (assign19180_e14136, assign19180_e14136_d_n0, assign19180_e14136_d_n2, assign19180_e14136_d_n4, assign19180_e14136_d_n5, assign19180_e14136_d_n6, assign19180_e14136_d_n7, assign19180_e14136_d_n8, assign19180_e14136_d_n9, assign19180_e14136_d_n10, assign19180_e14136_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let (assign19180_e14134, assign19180_e14134_d_n0, assign19180_e14134_d_n2, assign19180_e14134_d_n4, assign19180_e14134_d_n5, assign19180_e14134_d_n6, assign19180_e14134_d_n7, assign19180_e14134_d_n8, assign19180_e14134_d_n9, assign19180_e14134_d_n10, assign19180_e14134_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign19180_e14133: f64 = (-locals.var_tmf2);
                (assign19180_e14133, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign19180_e14134, assign19180_e14134_d_n0, assign19180_e14134_d_n2, assign19180_e14134_d_n4, assign19180_e14134_d_n5, assign19180_e14134_d_n6, assign19180_e14134_d_n7, assign19180_e14134_d_n8, assign19180_e14134_d_n9, assign19180_e14134_d_n10, assign19180_e14134_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign19180_e14136;
        locals.var_tmf2_dn0 = assign19180_e14136_d_n0;
        locals.var_tmf2_dn2 = assign19180_e14136_d_n2;
        locals.var_tmf2_dn4 = assign19180_e14136_d_n4;
        locals.var_tmf2_dn5 = assign19180_e14136_d_n5;
        locals.var_tmf2_dn6 = assign19180_e14136_d_n6;
        locals.var_tmf2_dn7 = assign19180_e14136_d_n7;
        locals.var_tmf2_dn8 = assign19180_e14136_d_n8;
        locals.var_tmf2_dn9 = assign19180_e14136_d_n9;
        locals.var_tmf2_dn10 = assign19180_e14136_d_n10;
        locals.var_tmf2_dn13 = assign19180_e14136_d_n13;

        let (assign19190_e14149, assign19190_e14149_d_n0, assign19190_e14149_d_n2, assign19190_e14149_d_n4, assign19190_e14149_d_n5, assign19190_e14149_d_n6, assign19190_e14149_d_n7, assign19190_e14149_d_n8, assign19190_e14149_d_n9, assign19190_e14149_d_n10, assign19190_e14149_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign19190_e14144: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign19190_e14146: f64 = (assign19190_e14144 + locals.var_tmf2);
        let assign19190_e14147: f64 = (assign19190_e14146).sqrt();
        (assign19190_e14147, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign19190_e14147)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign19190_e14147)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign19190_e14147)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign19190_e14147)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign19190_e14147)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign19190_e14147)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign19190_e14147)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign19190_e14147)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign19190_e14147)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign19190_e14147)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign19190_e14149;
        locals.var_tmf2_dn0 = assign19190_e14149_d_n0;
        locals.var_tmf2_dn2 = assign19190_e14149_d_n2;
        locals.var_tmf2_dn4 = assign19190_e14149_d_n4;
        locals.var_tmf2_dn5 = assign19190_e14149_d_n5;
        locals.var_tmf2_dn6 = assign19190_e14149_d_n6;
        locals.var_tmf2_dn7 = assign19190_e14149_d_n7;
        locals.var_tmf2_dn8 = assign19190_e14149_d_n8;
        locals.var_tmf2_dn9 = assign19190_e14149_d_n9;
        locals.var_tmf2_dn10 = assign19190_e14149_d_n10;
        locals.var_tmf2_dn13 = assign19190_e14149_d_n13;

        let (assign19200_e14163, assign19200_e14163_d_n0, assign19200_e14163_d_n2, assign19200_e14163_d_n4, assign19200_e14163_d_n5, assign19200_e14163_d_n6, assign19200_e14163_d_n7, assign19200_e14163_d_n8, assign19200_e14163_d_n9, assign19200_e14163_d_n10, assign19200_e14163_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign19200_e14159: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign19200_e14160: f64 = (1.0 + assign19200_e14159);
        let assign19200_e14161: f64 = (0.5 * assign19200_e14160);
        (assign19200_e14161, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign19200_e14163;
        locals.var_t6_dn0 = assign19200_e14163_d_n0;
        locals.var_t6_dn2 = assign19200_e14163_d_n2;
        locals.var_t6_dn4 = assign19200_e14163_d_n4;
        locals.var_t6_dn5 = assign19200_e14163_d_n5;
        locals.var_t6_dn6 = assign19200_e14163_d_n6;
        locals.var_t6_dn7 = assign19200_e14163_d_n7;
        locals.var_t6_dn8 = assign19200_e14163_d_n8;
        locals.var_t6_dn9 = assign19200_e14163_d_n9;
        locals.var_t6_dn10 = assign19200_e14163_d_n10;
        locals.var_t6_dn13 = assign19200_e14163_d_n13;

        let (assign19210_e14181, assign19210_e14181_d_n0, assign19210_e14181_d_n2, assign19210_e14181_d_n4, assign19210_e14181_d_n5, assign19210_e14181_d_n6, assign19210_e14181_d_n7, assign19210_e14181_d_n8, assign19210_e14181_d_n9, assign19210_e14181_d_n10, assign19210_e14181_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign19210_e14172: f64 = (p.p98 + 1.0);
        let assign19210_e14173: f64 = (locals.var_t4 * assign19210_e14172);
        let assign19210_e14177: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign19210_e14178: f64 = (0.5 * assign19210_e14177);
        let assign19210_e14179: f64 = (assign19210_e14173 - assign19210_e14178);
        (assign19210_e14179, ((locals.var_t4_dn0 * assign19210_e14172) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((locals.var_t4_dn2 * assign19210_e14172) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((locals.var_t4_dn4 * assign19210_e14172) - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), ((locals.var_t4_dn5 * assign19210_e14172) - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), ((locals.var_t4_dn6 * assign19210_e14172) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((locals.var_t4_dn7 * assign19210_e14172) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((locals.var_t4_dn8 * assign19210_e14172) - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), ((locals.var_t4_dn9 * assign19210_e14172) - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), ((locals.var_t4_dn10 * assign19210_e14172) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((locals.var_t4_dn13 * assign19210_e14172) - (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign19210_e14181;
        locals.var_t7_dn0 = assign19210_e14181_d_n0;
        locals.var_t7_dn2 = assign19210_e14181_d_n2;
        locals.var_t7_dn4 = assign19210_e14181_d_n4;
        locals.var_t7_dn5 = assign19210_e14181_d_n5;
        locals.var_t7_dn6 = assign19210_e14181_d_n6;
        locals.var_t7_dn7 = assign19210_e14181_d_n7;
        locals.var_t7_dn8 = assign19210_e14181_d_n8;
        locals.var_t7_dn9 = assign19210_e14181_d_n9;
        locals.var_t7_dn10 = assign19210_e14181_d_n10;
        locals.var_t7_dn13 = assign19210_e14181_d_n13;

        let (assign19220_e14197, assign19220_e14197_d_n0, assign19220_e14197_d_n2, assign19220_e14197_d_n4, assign19220_e14197_d_n5, assign19220_e14197_d_n6, assign19220_e14197_d_n7, assign19220_e14197_d_n8, assign19220_e14197_d_n9, assign19220_e14197_d_n10, assign19220_e14197_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign19220_e14190: f64 = (locals.var_t1 * locals.var_t4);
        let assign19220_e14191: f64 = (locals.var_t7 + assign19220_e14190);
        let assign19220_e14193: f64 = assign19220_e14191;
        let assign19220_e14195: f64 = (assign19220_e14193 - 5e-5);
        (assign19220_e14195, (locals.var_t7_dn0 + ((locals.var_t1_dn0 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn0))), (locals.var_t7_dn2 + ((locals.var_t1_dn2 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn2))), (locals.var_t7_dn4 + ((locals.var_t1_dn4 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn4))), (locals.var_t7_dn5 + ((locals.var_t1_dn5 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn5))), (locals.var_t7_dn6 + ((locals.var_t1_dn6 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn6))), (locals.var_t7_dn7 + ((locals.var_t1_dn7 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn7))), (locals.var_t7_dn8 + ((locals.var_t1_dn8 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn8))), (locals.var_t7_dn9 + ((locals.var_t1_dn9 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn9))), (locals.var_t7_dn10 + ((locals.var_t1_dn10 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn10))), (locals.var_t7_dn13 + ((locals.var_t1_dn13 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn13))),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign19220_e14197;
        locals.var_tmf1_dn0 = assign19220_e14197_d_n0;
        locals.var_tmf1_dn2 = assign19220_e14197_d_n2;
        locals.var_tmf1_dn4 = assign19220_e14197_d_n4;
        locals.var_tmf1_dn5 = assign19220_e14197_d_n5;
        locals.var_tmf1_dn6 = assign19220_e14197_d_n6;
        locals.var_tmf1_dn7 = assign19220_e14197_d_n7;
        locals.var_tmf1_dn8 = assign19220_e14197_d_n8;
        locals.var_tmf1_dn9 = assign19220_e14197_d_n9;
        locals.var_tmf1_dn10 = assign19220_e14197_d_n10;
        locals.var_tmf1_dn13 = assign19220_e14197_d_n13;

        let (assign19230_e14209, assign19230_e14209_d_n0, assign19230_e14209_d_n2, assign19230_e14209_d_n4, assign19230_e14209_d_n5, assign19230_e14209_d_n6, assign19230_e14209_d_n7, assign19230_e14209_d_n8, assign19230_e14209_d_n9, assign19230_e14209_d_n10, assign19230_e14209_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign19230_e14209;
        locals.var_tmf2_dn0 = assign19230_e14209_d_n0;
        locals.var_tmf2_dn2 = assign19230_e14209_d_n2;
        locals.var_tmf2_dn4 = assign19230_e14209_d_n4;
        locals.var_tmf2_dn5 = assign19230_e14209_d_n5;
        locals.var_tmf2_dn6 = assign19230_e14209_d_n6;
        locals.var_tmf2_dn7 = assign19230_e14209_d_n7;
        locals.var_tmf2_dn8 = assign19230_e14209_d_n8;
        locals.var_tmf2_dn9 = assign19230_e14209_d_n9;
        locals.var_tmf2_dn10 = assign19230_e14209_d_n10;
        locals.var_tmf2_dn13 = assign19230_e14209_d_n13;

        let (assign19240_e14223, assign19240_e14223_d_n0, assign19240_e14223_d_n2, assign19240_e14223_d_n4, assign19240_e14223_d_n5, assign19240_e14223_d_n6, assign19240_e14223_d_n7, assign19240_e14223_d_n8, assign19240_e14223_d_n9, assign19240_e14223_d_n10, assign19240_e14223_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let (assign19240_e14221, assign19240_e14221_d_n0, assign19240_e14221_d_n2, assign19240_e14221_d_n4, assign19240_e14221_d_n5, assign19240_e14221_d_n6, assign19240_e14221_d_n7, assign19240_e14221_d_n8, assign19240_e14221_d_n9, assign19240_e14221_d_n10, assign19240_e14221_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign19240_e14220: f64 = (-locals.var_tmf2);
                (assign19240_e14220, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign19240_e14221, assign19240_e14221_d_n0, assign19240_e14221_d_n2, assign19240_e14221_d_n4, assign19240_e14221_d_n5, assign19240_e14221_d_n6, assign19240_e14221_d_n7, assign19240_e14221_d_n8, assign19240_e14221_d_n9, assign19240_e14221_d_n10, assign19240_e14221_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign19240_e14223;
        locals.var_tmf2_dn0 = assign19240_e14223_d_n0;
        locals.var_tmf2_dn2 = assign19240_e14223_d_n2;
        locals.var_tmf2_dn4 = assign19240_e14223_d_n4;
        locals.var_tmf2_dn5 = assign19240_e14223_d_n5;
        locals.var_tmf2_dn6 = assign19240_e14223_d_n6;
        locals.var_tmf2_dn7 = assign19240_e14223_d_n7;
        locals.var_tmf2_dn8 = assign19240_e14223_d_n8;
        locals.var_tmf2_dn9 = assign19240_e14223_d_n9;
        locals.var_tmf2_dn10 = assign19240_e14223_d_n10;
        locals.var_tmf2_dn13 = assign19240_e14223_d_n13;

        let (assign19250_e14236, assign19250_e14236_d_n0, assign19250_e14236_d_n2, assign19250_e14236_d_n4, assign19250_e14236_d_n5, assign19250_e14236_d_n6, assign19250_e14236_d_n7, assign19250_e14236_d_n8, assign19250_e14236_d_n9, assign19250_e14236_d_n10, assign19250_e14236_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign19250_e14231: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign19250_e14233: f64 = (assign19250_e14231 + locals.var_tmf2);
        let assign19250_e14234: f64 = (assign19250_e14233).sqrt();
        (assign19250_e14234, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign19250_e14234)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign19250_e14234)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign19250_e14234)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign19250_e14234)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign19250_e14234)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign19250_e14234)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign19250_e14234)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign19250_e14234)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign19250_e14234)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign19250_e14234)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign19250_e14236;
        locals.var_tmf2_dn0 = assign19250_e14236_d_n0;
        locals.var_tmf2_dn2 = assign19250_e14236_d_n2;
        locals.var_tmf2_dn4 = assign19250_e14236_d_n4;
        locals.var_tmf2_dn5 = assign19250_e14236_d_n5;
        locals.var_tmf2_dn6 = assign19250_e14236_d_n6;
        locals.var_tmf2_dn7 = assign19250_e14236_d_n7;
        locals.var_tmf2_dn8 = assign19250_e14236_d_n8;
        locals.var_tmf2_dn9 = assign19250_e14236_d_n9;
        locals.var_tmf2_dn10 = assign19250_e14236_d_n10;
        locals.var_tmf2_dn13 = assign19250_e14236_d_n13;

        let (assign19260_e14250, assign19260_e14250_d_n0, assign19260_e14250_d_n2, assign19260_e14250_d_n4, assign19260_e14250_d_n5, assign19260_e14250_d_n6, assign19260_e14250_d_n7, assign19260_e14250_d_n8, assign19260_e14250_d_n9, assign19260_e14250_d_n10, assign19260_e14250_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign19260_e14246: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign19260_e14247: f64 = (1.0 + assign19260_e14246);
        let assign19260_e14248: f64 = (0.5 * assign19260_e14247);
        (assign19260_e14248, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign19260_e14250;
        locals.var_t6_dn0 = assign19260_e14250_d_n0;
        locals.var_t6_dn2 = assign19260_e14250_d_n2;
        locals.var_t6_dn4 = assign19260_e14250_d_n4;
        locals.var_t6_dn5 = assign19260_e14250_d_n5;
        locals.var_t6_dn6 = assign19260_e14250_d_n6;
        locals.var_t6_dn7 = assign19260_e14250_d_n7;
        locals.var_t6_dn8 = assign19260_e14250_d_n8;
        locals.var_t6_dn9 = assign19260_e14250_d_n9;
        locals.var_t6_dn10 = assign19260_e14250_d_n10;
        locals.var_t6_dn13 = assign19260_e14250_d_n13;

        let (assign19270_e14264, assign19270_e14264_d_n0, assign19270_e14264_d_n2, assign19270_e14264_d_n4, assign19270_e14264_d_n5, assign19270_e14264_d_n6, assign19270_e14264_d_n7, assign19270_e14264_d_n8, assign19270_e14264_d_n9, assign19270_e14264_d_n10, assign19270_e14264_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign19270_e14260: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign19270_e14261: f64 = (0.5 * assign19270_e14260);
        let assign19270_e14262: f64 = assign19270_e14261;
        (assign19270_e14262, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign19270_e14264;
        locals.var_t2_dn0 = assign19270_e14264_d_n0;
        locals.var_t2_dn2 = assign19270_e14264_d_n2;
        locals.var_t2_dn4 = assign19270_e14264_d_n4;
        locals.var_t2_dn5 = assign19270_e14264_d_n5;
        locals.var_t2_dn6 = assign19270_e14264_d_n6;
        locals.var_t2_dn7 = assign19270_e14264_d_n7;
        locals.var_t2_dn8 = assign19270_e14264_d_n8;
        locals.var_t2_dn9 = assign19270_e14264_d_n9;
        locals.var_t2_dn10 = assign19270_e14264_d_n10;
        locals.var_t2_dn13 = assign19270_e14264_d_n13;

        let assign19280_e14271: f64 = if ((p.p39 == 0.0) || (p.p39 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard386 = assign19280_e14271;

        let (assign19290_e14291, assign19290_e14291_d_n0, assign19290_e14291_d_n2, assign19290_e14291_d_n4, assign19290_e14291_d_n5, assign19290_e14291_d_n6, assign19290_e14291_d_n7, assign19290_e14291_d_n8, assign19290_e14291_d_n9, assign19290_e14291_d_n10, assign19290_e14291_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard386 != 0.0)) {
        let assign19290_e14282: f64 = (locals.var_mks_rdvdtemp1 * locals.var_tdiff0);
        let assign19290_e14283: f64 = (locals.var_uc_rdvd + assign19290_e14282);
        let assign19290_e14286: f64 = (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2);
        let assign19290_e14287: f64 = (assign19290_e14283 + assign19290_e14286);
        let assign19290_e14289: f64 = (assign19290_e14287 * locals.var_t2);
        (assign19290_e14289, ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn0) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn0)) * locals.var_t2) + (assign19290_e14287 * locals.var_t2_dn0)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn2) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn2)) * locals.var_t2) + (assign19290_e14287 * locals.var_t2_dn2)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn4) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn4)) * locals.var_t2) + (assign19290_e14287 * locals.var_t2_dn4)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn5) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn5)) * locals.var_t2) + (assign19290_e14287 * locals.var_t2_dn5)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn6) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn6)) * locals.var_t2) + (assign19290_e14287 * locals.var_t2_dn6)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn7) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn7)) * locals.var_t2) + (assign19290_e14287 * locals.var_t2_dn7)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn8) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn8)) * locals.var_t2) + (assign19290_e14287 * locals.var_t2_dn8)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn9) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn9)) * locals.var_t2) + (assign19290_e14287 * locals.var_t2_dn9)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn10) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn10)) * locals.var_t2) + (assign19290_e14287 * locals.var_t2_dn10)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn13) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn13)) * locals.var_t2) + (assign19290_e14287 * locals.var_t2_dn13)),)
    } else {
        (locals.var_rsvde, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn13,)
    }
};
        locals.var_rsvde = assign19290_e14291;
        locals.var_rsvde_dn0 = assign19290_e14291_d_n0;
        locals.var_rsvde_dn2 = assign19290_e14291_d_n2;
        locals.var_rsvde_dn4 = assign19290_e14291_d_n4;
        locals.var_rsvde_dn5 = assign19290_e14291_d_n5;
        locals.var_rsvde_dn6 = assign19290_e14291_d_n6;
        locals.var_rsvde_dn7 = assign19290_e14291_d_n7;
        locals.var_rsvde_dn8 = assign19290_e14291_d_n8;
        locals.var_rsvde_dn9 = assign19290_e14291_d_n9;
        locals.var_rsvde_dn10 = assign19290_e14291_d_n10;
        locals.var_rsvde_dn13 = assign19290_e14291_d_n13;

        let (assign19300_e14309, assign19300_e14309_d_n0, assign19300_e14309_d_n2, assign19300_e14309_d_n4, assign19300_e14309_d_n5, assign19300_e14309_d_n6, assign19300_e14309_d_n7, assign19300_e14309_d_n8, assign19300_e14309_d_n9, assign19300_e14309_d_n10, assign19300_e14309_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard386 != 0.0)) {
        let assign19300_e14302: f64 = (0.005 * locals.var_uc_rdvd);
        let assign19300_e14303: f64 = (locals.var_rsvde - assign19300_e14302);
        let assign19300_e14306: f64 = (0.01 * locals.var_uc_rdvd);
        let assign19300_e14307: f64 = (assign19300_e14303 - assign19300_e14306);
        (assign19300_e14307, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign19300_e14309;
        locals.var_tmf1_dn0 = assign19300_e14309_d_n0;
        locals.var_tmf1_dn2 = assign19300_e14309_d_n2;
        locals.var_tmf1_dn4 = assign19300_e14309_d_n4;
        locals.var_tmf1_dn5 = assign19300_e14309_d_n5;
        locals.var_tmf1_dn6 = assign19300_e14309_d_n6;
        locals.var_tmf1_dn7 = assign19300_e14309_d_n7;
        locals.var_tmf1_dn8 = assign19300_e14309_d_n8;
        locals.var_tmf1_dn9 = assign19300_e14309_d_n9;
        locals.var_tmf1_dn10 = assign19300_e14309_d_n10;
        locals.var_tmf1_dn13 = assign19300_e14309_d_n13;

        let (assign19310_e14327, assign19310_e14327_d_n0, assign19310_e14327_d_n2, assign19310_e14327_d_n4, assign19310_e14327_d_n5, assign19310_e14327_d_n6, assign19310_e14327_d_n7, assign19310_e14327_d_n8, assign19310_e14327_d_n9, assign19310_e14327_d_n10, assign19310_e14327_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard386 != 0.0)) {
        let assign19310_e14320: f64 = (0.005 * locals.var_uc_rdvd);
        let assign19310_e14321: f64 = (4.0 * assign19310_e14320);
        let assign19310_e14324: f64 = (0.01 * locals.var_uc_rdvd);
        let assign19310_e14325: f64 = (assign19310_e14321 * assign19310_e14324);
        (assign19310_e14325, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign19310_e14327;
        locals.var_tmf2_dn0 = assign19310_e14327_d_n0;
        locals.var_tmf2_dn2 = assign19310_e14327_d_n2;
        locals.var_tmf2_dn4 = assign19310_e14327_d_n4;
        locals.var_tmf2_dn5 = assign19310_e14327_d_n5;
        locals.var_tmf2_dn6 = assign19310_e14327_d_n6;
        locals.var_tmf2_dn7 = assign19310_e14327_d_n7;
        locals.var_tmf2_dn8 = assign19310_e14327_d_n8;
        locals.var_tmf2_dn9 = assign19310_e14327_d_n9;
        locals.var_tmf2_dn10 = assign19310_e14327_d_n10;
        locals.var_tmf2_dn13 = assign19310_e14327_d_n13;

        let (assign19320_e14343, assign19320_e14343_d_n0, assign19320_e14343_d_n2, assign19320_e14343_d_n4, assign19320_e14343_d_n5, assign19320_e14343_d_n6, assign19320_e14343_d_n7, assign19320_e14343_d_n8, assign19320_e14343_d_n9, assign19320_e14343_d_n10, assign19320_e14343_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard386 != 0.0)) {
        let (assign19320_e14341, assign19320_e14341_d_n0, assign19320_e14341_d_n2, assign19320_e14341_d_n4, assign19320_e14341_d_n5, assign19320_e14341_d_n6, assign19320_e14341_d_n7, assign19320_e14341_d_n8, assign19320_e14341_d_n9, assign19320_e14341_d_n10, assign19320_e14341_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign19320_e14340: f64 = (-locals.var_tmf2);
                (assign19320_e14340, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign19320_e14341, assign19320_e14341_d_n0, assign19320_e14341_d_n2, assign19320_e14341_d_n4, assign19320_e14341_d_n5, assign19320_e14341_d_n6, assign19320_e14341_d_n7, assign19320_e14341_d_n8, assign19320_e14341_d_n9, assign19320_e14341_d_n10, assign19320_e14341_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign19320_e14343;
        locals.var_tmf2_dn0 = assign19320_e14343_d_n0;
        locals.var_tmf2_dn2 = assign19320_e14343_d_n2;
        locals.var_tmf2_dn4 = assign19320_e14343_d_n4;
        locals.var_tmf2_dn5 = assign19320_e14343_d_n5;
        locals.var_tmf2_dn6 = assign19320_e14343_d_n6;
        locals.var_tmf2_dn7 = assign19320_e14343_d_n7;
        locals.var_tmf2_dn8 = assign19320_e14343_d_n8;
        locals.var_tmf2_dn9 = assign19320_e14343_d_n9;
        locals.var_tmf2_dn10 = assign19320_e14343_d_n10;
        locals.var_tmf2_dn13 = assign19320_e14343_d_n13;

        let (assign19330_e14358, assign19330_e14358_d_n0, assign19330_e14358_d_n2, assign19330_e14358_d_n4, assign19330_e14358_d_n5, assign19330_e14358_d_n6, assign19330_e14358_d_n7, assign19330_e14358_d_n8, assign19330_e14358_d_n9, assign19330_e14358_d_n10, assign19330_e14358_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard386 != 0.0)) {
        let assign19330_e14353: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign19330_e14355: f64 = (assign19330_e14353 + locals.var_tmf2);
        let assign19330_e14356: f64 = (assign19330_e14355).sqrt();
        (assign19330_e14356, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign19330_e14356)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign19330_e14356)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign19330_e14356)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign19330_e14356)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign19330_e14356)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign19330_e14356)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign19330_e14356)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign19330_e14356)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign19330_e14356)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign19330_e14356)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign19330_e14358;
        locals.var_tmf2_dn0 = assign19330_e14358_d_n0;
        locals.var_tmf2_dn2 = assign19330_e14358_d_n2;
        locals.var_tmf2_dn4 = assign19330_e14358_d_n4;
        locals.var_tmf2_dn5 = assign19330_e14358_d_n5;
        locals.var_tmf2_dn6 = assign19330_e14358_d_n6;
        locals.var_tmf2_dn7 = assign19330_e14358_d_n7;
        locals.var_tmf2_dn8 = assign19330_e14358_d_n8;
        locals.var_tmf2_dn9 = assign19330_e14358_d_n9;
        locals.var_tmf2_dn10 = assign19330_e14358_d_n10;
        locals.var_tmf2_dn13 = assign19330_e14358_d_n13;

        let (assign19340_e14374, assign19340_e14374_d_n0, assign19340_e14374_d_n2, assign19340_e14374_d_n4, assign19340_e14374_d_n5, assign19340_e14374_d_n6, assign19340_e14374_d_n7, assign19340_e14374_d_n8, assign19340_e14374_d_n9, assign19340_e14374_d_n10, assign19340_e14374_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard386 != 0.0)) {
        let assign19340_e14370: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign19340_e14371: f64 = (1.0 + assign19340_e14370);
        let assign19340_e14372: f64 = (0.5 * assign19340_e14371);
        (assign19340_e14372, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign19340_e14374;
        locals.var_t0_dn0 = assign19340_e14374_d_n0;
        locals.var_t0_dn2 = assign19340_e14374_d_n2;
        locals.var_t0_dn4 = assign19340_e14374_d_n4;
        locals.var_t0_dn5 = assign19340_e14374_d_n5;
        locals.var_t0_dn6 = assign19340_e14374_d_n6;
        locals.var_t0_dn7 = assign19340_e14374_d_n7;
        locals.var_t0_dn8 = assign19340_e14374_d_n8;
        locals.var_t0_dn9 = assign19340_e14374_d_n9;
        locals.var_t0_dn10 = assign19340_e14374_d_n10;
        locals.var_t0_dn13 = assign19340_e14374_d_n13;

        let (assign19350_e14392, assign19350_e14392_d_n0, assign19350_e14392_d_n2, assign19350_e14392_d_n4, assign19350_e14392_d_n5, assign19350_e14392_d_n6, assign19350_e14392_d_n7, assign19350_e14392_d_n8, assign19350_e14392_d_n9, assign19350_e14392_d_n10, assign19350_e14392_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard386 != 0.0)) {
        let assign19350_e14384: f64 = (0.005 * locals.var_uc_rdvd);
        let assign19350_e14388: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign19350_e14389: f64 = (0.5 * assign19350_e14388);
        let assign19350_e14390: f64 = (assign19350_e14384 + assign19350_e14389);
        (assign19350_e14390, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_rsvde, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn13,)
    }
};
        locals.var_rsvde = assign19350_e14392;
        locals.var_rsvde_dn0 = assign19350_e14392_d_n0;
        locals.var_rsvde_dn2 = assign19350_e14392_d_n2;
        locals.var_rsvde_dn4 = assign19350_e14392_d_n4;
        locals.var_rsvde_dn5 = assign19350_e14392_d_n5;
        locals.var_rsvde_dn6 = assign19350_e14392_d_n6;
        locals.var_rsvde_dn7 = assign19350_e14392_d_n7;
        locals.var_rsvde_dn8 = assign19350_e14392_d_n8;
        locals.var_rsvde_dn9 = assign19350_e14392_d_n9;
        locals.var_rsvde_dn10 = assign19350_e14392_d_n10;
        locals.var_rsvde_dn13 = assign19350_e14392_d_n13;

        let (assign19360_e14413, assign19360_e14413_d_n0, assign19360_e14413_d_n2, assign19360_e14413_d_n4, assign19360_e14413_d_n5, assign19360_e14413_d_n6, assign19360_e14413_d_n7, assign19360_e14413_d_n8, assign19360_e14413_d_n9, assign19360_e14413_d_n10, assign19360_e14413_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard386 == 0.0)) {
        let assign19360_e14404: f64 = (locals.var_mks_rdvdtemp1 * locals.var_tdiff);
        let assign19360_e14405: f64 = (locals.var_uc_rdvd + assign19360_e14404);
        let assign19360_e14408: f64 = (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2);
        let assign19360_e14409: f64 = (assign19360_e14405 + assign19360_e14408);
        let assign19360_e14411: f64 = (assign19360_e14409 * locals.var_t2);
        (assign19360_e14411, ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn0) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn0)) * locals.var_t2) + (assign19360_e14409 * locals.var_t2_dn0)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn2) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn2)) * locals.var_t2) + (assign19360_e14409 * locals.var_t2_dn2)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn4) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn4)) * locals.var_t2) + (assign19360_e14409 * locals.var_t2_dn4)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn5) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn5)) * locals.var_t2) + (assign19360_e14409 * locals.var_t2_dn5)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn6) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn6)) * locals.var_t2) + (assign19360_e14409 * locals.var_t2_dn6)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn7) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn7)) * locals.var_t2) + (assign19360_e14409 * locals.var_t2_dn7)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn8) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn8)) * locals.var_t2) + (assign19360_e14409 * locals.var_t2_dn8)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn9) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn9)) * locals.var_t2) + (assign19360_e14409 * locals.var_t2_dn9)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn10) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn10)) * locals.var_t2) + (assign19360_e14409 * locals.var_t2_dn10)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn13) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn13)) * locals.var_t2) + (assign19360_e14409 * locals.var_t2_dn13)),)
    } else {
        (locals.var_rsvde, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn13,)
    }
};
        locals.var_rsvde = assign19360_e14413;
        locals.var_rsvde_dn0 = assign19360_e14413_d_n0;
        locals.var_rsvde_dn2 = assign19360_e14413_d_n2;
        locals.var_rsvde_dn4 = assign19360_e14413_d_n4;
        locals.var_rsvde_dn5 = assign19360_e14413_d_n5;
        locals.var_rsvde_dn6 = assign19360_e14413_d_n6;
        locals.var_rsvde_dn7 = assign19360_e14413_d_n7;
        locals.var_rsvde_dn8 = assign19360_e14413_d_n8;
        locals.var_rsvde_dn9 = assign19360_e14413_d_n9;
        locals.var_rsvde_dn10 = assign19360_e14413_d_n10;
        locals.var_rsvde_dn13 = assign19360_e14413_d_n13;

        let (assign19370_e14432, assign19370_e14432_d_n0, assign19370_e14432_d_n2, assign19370_e14432_d_n4, assign19370_e14432_d_n5, assign19370_e14432_d_n6, assign19370_e14432_d_n7, assign19370_e14432_d_n8, assign19370_e14432_d_n9, assign19370_e14432_d_n10, assign19370_e14432_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard386 == 0.0)) {
        let assign19370_e14425: f64 = (0.005 * locals.var_uc_rdvd);
        let assign19370_e14426: f64 = (locals.var_rsvde - assign19370_e14425);
        let assign19370_e14429: f64 = (0.01 * locals.var_uc_rdvd);
        let assign19370_e14430: f64 = (assign19370_e14426 - assign19370_e14429);
        (assign19370_e14430, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign19370_e14432;
        locals.var_tmf1_dn0 = assign19370_e14432_d_n0;
        locals.var_tmf1_dn2 = assign19370_e14432_d_n2;
        locals.var_tmf1_dn4 = assign19370_e14432_d_n4;
        locals.var_tmf1_dn5 = assign19370_e14432_d_n5;
        locals.var_tmf1_dn6 = assign19370_e14432_d_n6;
        locals.var_tmf1_dn7 = assign19370_e14432_d_n7;
        locals.var_tmf1_dn8 = assign19370_e14432_d_n8;
        locals.var_tmf1_dn9 = assign19370_e14432_d_n9;
        locals.var_tmf1_dn10 = assign19370_e14432_d_n10;
        locals.var_tmf1_dn13 = assign19370_e14432_d_n13;

        let (assign19380_e14451, assign19380_e14451_d_n0, assign19380_e14451_d_n2, assign19380_e14451_d_n4, assign19380_e14451_d_n5, assign19380_e14451_d_n6, assign19380_e14451_d_n7, assign19380_e14451_d_n8, assign19380_e14451_d_n9, assign19380_e14451_d_n10, assign19380_e14451_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard386 == 0.0)) {
        let assign19380_e14444: f64 = (0.005 * locals.var_uc_rdvd);
        let assign19380_e14445: f64 = (4.0 * assign19380_e14444);
        let assign19380_e14448: f64 = (0.01 * locals.var_uc_rdvd);
        let assign19380_e14449: f64 = (assign19380_e14445 * assign19380_e14448);
        (assign19380_e14449, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign19380_e14451;
        locals.var_tmf2_dn0 = assign19380_e14451_d_n0;
        locals.var_tmf2_dn2 = assign19380_e14451_d_n2;
        locals.var_tmf2_dn4 = assign19380_e14451_d_n4;
        locals.var_tmf2_dn5 = assign19380_e14451_d_n5;
        locals.var_tmf2_dn6 = assign19380_e14451_d_n6;
        locals.var_tmf2_dn7 = assign19380_e14451_d_n7;
        locals.var_tmf2_dn8 = assign19380_e14451_d_n8;
        locals.var_tmf2_dn9 = assign19380_e14451_d_n9;
        locals.var_tmf2_dn10 = assign19380_e14451_d_n10;
        locals.var_tmf2_dn13 = assign19380_e14451_d_n13;

    }

    pub(super) fn stamp_transient_block_44(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let (assign19390_e14468, assign19390_e14468_d_n0, assign19390_e14468_d_n2, assign19390_e14468_d_n4, assign19390_e14468_d_n5, assign19390_e14468_d_n6, assign19390_e14468_d_n7, assign19390_e14468_d_n8, assign19390_e14468_d_n9, assign19390_e14468_d_n10, assign19390_e14468_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard386 == 0.0)) {
        let (assign19390_e14466, assign19390_e14466_d_n0, assign19390_e14466_d_n2, assign19390_e14466_d_n4, assign19390_e14466_d_n5, assign19390_e14466_d_n6, assign19390_e14466_d_n7, assign19390_e14466_d_n8, assign19390_e14466_d_n9, assign19390_e14466_d_n10, assign19390_e14466_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign19390_e14465: f64 = (-locals.var_tmf2);
                (assign19390_e14465, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign19390_e14466, assign19390_e14466_d_n0, assign19390_e14466_d_n2, assign19390_e14466_d_n4, assign19390_e14466_d_n5, assign19390_e14466_d_n6, assign19390_e14466_d_n7, assign19390_e14466_d_n8, assign19390_e14466_d_n9, assign19390_e14466_d_n10, assign19390_e14466_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign19390_e14468;
        locals.var_tmf2_dn0 = assign19390_e14468_d_n0;
        locals.var_tmf2_dn2 = assign19390_e14468_d_n2;
        locals.var_tmf2_dn4 = assign19390_e14468_d_n4;
        locals.var_tmf2_dn5 = assign19390_e14468_d_n5;
        locals.var_tmf2_dn6 = assign19390_e14468_d_n6;
        locals.var_tmf2_dn7 = assign19390_e14468_d_n7;
        locals.var_tmf2_dn8 = assign19390_e14468_d_n8;
        locals.var_tmf2_dn9 = assign19390_e14468_d_n9;
        locals.var_tmf2_dn10 = assign19390_e14468_d_n10;
        locals.var_tmf2_dn13 = assign19390_e14468_d_n13;

        let (assign19400_e14484, assign19400_e14484_d_n0, assign19400_e14484_d_n2, assign19400_e14484_d_n4, assign19400_e14484_d_n5, assign19400_e14484_d_n6, assign19400_e14484_d_n7, assign19400_e14484_d_n8, assign19400_e14484_d_n9, assign19400_e14484_d_n10, assign19400_e14484_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard386 == 0.0)) {
        let assign19400_e14479: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign19400_e14481: f64 = (assign19400_e14479 + locals.var_tmf2);
        let assign19400_e14482: f64 = (assign19400_e14481).sqrt();
        (assign19400_e14482, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign19400_e14482)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign19400_e14482)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign19400_e14482)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign19400_e14482)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign19400_e14482)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign19400_e14482)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign19400_e14482)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign19400_e14482)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign19400_e14482)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign19400_e14482)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign19400_e14484;
        locals.var_tmf2_dn0 = assign19400_e14484_d_n0;
        locals.var_tmf2_dn2 = assign19400_e14484_d_n2;
        locals.var_tmf2_dn4 = assign19400_e14484_d_n4;
        locals.var_tmf2_dn5 = assign19400_e14484_d_n5;
        locals.var_tmf2_dn6 = assign19400_e14484_d_n6;
        locals.var_tmf2_dn7 = assign19400_e14484_d_n7;
        locals.var_tmf2_dn8 = assign19400_e14484_d_n8;
        locals.var_tmf2_dn9 = assign19400_e14484_d_n9;
        locals.var_tmf2_dn10 = assign19400_e14484_d_n10;
        locals.var_tmf2_dn13 = assign19400_e14484_d_n13;

        let (assign19410_e14501, assign19410_e14501_d_n0, assign19410_e14501_d_n2, assign19410_e14501_d_n4, assign19410_e14501_d_n5, assign19410_e14501_d_n6, assign19410_e14501_d_n7, assign19410_e14501_d_n8, assign19410_e14501_d_n9, assign19410_e14501_d_n10, assign19410_e14501_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard386 == 0.0)) {
        let assign19410_e14497: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign19410_e14498: f64 = (1.0 + assign19410_e14497);
        let assign19410_e14499: f64 = (0.5 * assign19410_e14498);
        (assign19410_e14499, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign19410_e14501;
        locals.var_t0_dn0 = assign19410_e14501_d_n0;
        locals.var_t0_dn2 = assign19410_e14501_d_n2;
        locals.var_t0_dn4 = assign19410_e14501_d_n4;
        locals.var_t0_dn5 = assign19410_e14501_d_n5;
        locals.var_t0_dn6 = assign19410_e14501_d_n6;
        locals.var_t0_dn7 = assign19410_e14501_d_n7;
        locals.var_t0_dn8 = assign19410_e14501_d_n8;
        locals.var_t0_dn9 = assign19410_e14501_d_n9;
        locals.var_t0_dn10 = assign19410_e14501_d_n10;
        locals.var_t0_dn13 = assign19410_e14501_d_n13;

        let (assign19420_e14520, assign19420_e14520_d_n0, assign19420_e14520_d_n2, assign19420_e14520_d_n4, assign19420_e14520_d_n5, assign19420_e14520_d_n6, assign19420_e14520_d_n7, assign19420_e14520_d_n8, assign19420_e14520_d_n9, assign19420_e14520_d_n10, assign19420_e14520_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard386 == 0.0)) {
        let assign19420_e14512: f64 = (0.005 * locals.var_uc_rdvd);
        let assign19420_e14516: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign19420_e14517: f64 = (0.5 * assign19420_e14516);
        let assign19420_e14518: f64 = (assign19420_e14512 + assign19420_e14517);
        (assign19420_e14518, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_rsvde, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn13,)
    }
};
        locals.var_rsvde = assign19420_e14520;
        locals.var_rsvde_dn0 = assign19420_e14520_d_n0;
        locals.var_rsvde_dn2 = assign19420_e14520_d_n2;
        locals.var_rsvde_dn4 = assign19420_e14520_d_n4;
        locals.var_rsvde_dn5 = assign19420_e14520_d_n5;
        locals.var_rsvde_dn6 = assign19420_e14520_d_n6;
        locals.var_rsvde_dn7 = assign19420_e14520_d_n7;
        locals.var_rsvde_dn8 = assign19420_e14520_d_n8;
        locals.var_rsvde_dn9 = assign19420_e14520_d_n9;
        locals.var_rsvde_dn10 = assign19420_e14520_d_n10;
        locals.var_rsvde_dn13 = assign19420_e14520_d_n13;

        let (assign19430_e14529, assign19430_e14529_d_n0, assign19430_e14529_d_n2, assign19430_e14529_d_n4, assign19430_e14529_d_n5, assign19430_e14529_d_n6, assign19430_e14529_d_n7, assign19430_e14529_d_n8, assign19430_e14529_d_n9, assign19430_e14529_d_n10, assign19430_e14529_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdvde, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn13,)
    }
};
        locals.var_rdvde = assign19430_e14529;
        locals.var_rdvde_dn0 = assign19430_e14529_d_n0;
        locals.var_rdvde_dn2 = assign19430_e14529_d_n2;
        locals.var_rdvde_dn4 = assign19430_e14529_d_n4;
        locals.var_rdvde_dn5 = assign19430_e14529_d_n5;
        locals.var_rdvde_dn6 = assign19430_e14529_d_n6;
        locals.var_rdvde_dn7 = assign19430_e14529_d_n7;
        locals.var_rdvde_dn8 = assign19430_e14529_d_n8;
        locals.var_rdvde_dn9 = assign19430_e14529_d_n9;
        locals.var_rdvde_dn10 = assign19430_e14529_d_n10;
        locals.var_rdvde_dn13 = assign19430_e14529_d_n13;

        let (assign19440_e14538, assign19440_e14538_d_n0, assign19440_e14538_d_n2, assign19440_e14538_d_n4, assign19440_e14538_d_n5, assign19440_e14538_d_n6, assign19440_e14538_d_n7, assign19440_e14538_d_n8, assign19440_e14538_d_n9, assign19440_e14538_d_n10, assign19440_e14538_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rsvde, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn13,)
    }
};
        locals.var_rsvde = assign19440_e14538;
        locals.var_rsvde_dn0 = assign19440_e14538_d_n0;
        locals.var_rsvde_dn2 = assign19440_e14538_d_n2;
        locals.var_rsvde_dn4 = assign19440_e14538_d_n4;
        locals.var_rsvde_dn5 = assign19440_e14538_d_n5;
        locals.var_rsvde_dn6 = assign19440_e14538_d_n6;
        locals.var_rsvde_dn7 = assign19440_e14538_d_n7;
        locals.var_rsvde_dn8 = assign19440_e14538_d_n8;
        locals.var_rsvde_dn9 = assign19440_e14538_d_n9;
        locals.var_rsvde_dn10 = assign19440_e14538_d_n10;
        locals.var_rsvde_dn13 = assign19440_e14538_d_n13;

        let (assign19450_e14545, assign19450_e14545_d_n0, assign19450_e14545_d_n2, assign19450_e14545_d_n4, assign19450_e14545_d_n5, assign19450_e14545_d_n6, assign19450_e14545_d_n7, assign19450_e14545_d_n8, assign19450_e14545_d_n9, assign19450_e14545_d_n10, assign19450_e14545_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign19450_e14542: f64 = (locals.var_beta_inv).sqrt();
        let assign19450_e14543: f64 = (locals.var_costi00 * assign19450_e14542);
        (assign19450_e14543, (locals.var_costi00 * (locals.var_beta_inv_dn0 / (2.0 * assign19450_e14542))), (locals.var_costi00 * (locals.var_beta_inv_dn2 / (2.0 * assign19450_e14542))), (locals.var_costi00 * (locals.var_beta_inv_dn4 / (2.0 * assign19450_e14542))), (locals.var_costi00 * (locals.var_beta_inv_dn5 / (2.0 * assign19450_e14542))), (locals.var_costi00 * (locals.var_beta_inv_dn6 / (2.0 * assign19450_e14542))), (locals.var_costi00 * (locals.var_beta_inv_dn7 / (2.0 * assign19450_e14542))), (locals.var_costi00 * (locals.var_beta_inv_dn8 / (2.0 * assign19450_e14542))), (locals.var_costi00 * (locals.var_beta_inv_dn9 / (2.0 * assign19450_e14542))), (locals.var_costi00 * (locals.var_beta_inv_dn10 / (2.0 * assign19450_e14542))), (locals.var_costi00 * (locals.var_beta_inv_dn13 / (2.0 * assign19450_e14542))),)
    } else {
        (locals.var_costi0, locals.var_costi0_dn0, locals.var_costi0_dn2, locals.var_costi0_dn4, locals.var_costi0_dn5, locals.var_costi0_dn6, locals.var_costi0_dn7, locals.var_costi0_dn8, locals.var_costi0_dn9, locals.var_costi0_dn10, locals.var_costi0_dn13,)
    }
};
        locals.var_costi0 = assign19450_e14545;
        locals.var_costi0_dn0 = assign19450_e14545_d_n0;
        locals.var_costi0_dn2 = assign19450_e14545_d_n2;
        locals.var_costi0_dn4 = assign19450_e14545_d_n4;
        locals.var_costi0_dn5 = assign19450_e14545_d_n5;
        locals.var_costi0_dn6 = assign19450_e14545_d_n6;
        locals.var_costi0_dn7 = assign19450_e14545_d_n7;
        locals.var_costi0_dn8 = assign19450_e14545_d_n8;
        locals.var_costi0_dn9 = assign19450_e14545_d_n9;
        locals.var_costi0_dn10 = assign19450_e14545_d_n10;
        locals.var_costi0_dn13 = assign19450_e14545_d_n13;

        let (assign19460_e14551, assign19460_e14551_d_n0, assign19460_e14551_d_n2, assign19460_e14551_d_n4, assign19460_e14551_d_n5, assign19460_e14551_d_n6, assign19460_e14551_d_n7, assign19460_e14551_d_n8, assign19460_e14551_d_n9, assign19460_e14551_d_n10, assign19460_e14551_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign19460_e14549: f64 = (locals.var_costi0 * locals.var_costi0);
        (assign19460_e14549, ((locals.var_costi0_dn0 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn0)), ((locals.var_costi0_dn2 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn2)), ((locals.var_costi0_dn4 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn4)), ((locals.var_costi0_dn5 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn5)), ((locals.var_costi0_dn6 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn6)), ((locals.var_costi0_dn7 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn7)), ((locals.var_costi0_dn8 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn8)), ((locals.var_costi0_dn9 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn9)), ((locals.var_costi0_dn10 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn10)), ((locals.var_costi0_dn13 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn13)),)
    } else {
        (locals.var_costi0_p2, locals.var_costi0_p2_dn0, locals.var_costi0_p2_dn2, locals.var_costi0_p2_dn4, locals.var_costi0_p2_dn5, locals.var_costi0_p2_dn6, locals.var_costi0_p2_dn7, locals.var_costi0_p2_dn8, locals.var_costi0_p2_dn9, locals.var_costi0_p2_dn10, locals.var_costi0_p2_dn13,)
    }
};
        locals.var_costi0_p2 = assign19460_e14551;
        locals.var_costi0_p2_dn0 = assign19460_e14551_d_n0;
        locals.var_costi0_p2_dn2 = assign19460_e14551_d_n2;
        locals.var_costi0_p2_dn4 = assign19460_e14551_d_n4;
        locals.var_costi0_p2_dn5 = assign19460_e14551_d_n5;
        locals.var_costi0_p2_dn6 = assign19460_e14551_d_n6;
        locals.var_costi0_p2_dn7 = assign19460_e14551_d_n7;
        locals.var_costi0_p2_dn8 = assign19460_e14551_d_n8;
        locals.var_costi0_p2_dn9 = assign19460_e14551_d_n9;
        locals.var_costi0_p2_dn10 = assign19460_e14551_d_n10;
        locals.var_costi0_p2_dn13 = assign19460_e14551_d_n13;

        let (assign19470_e14559, assign19470_e14559_d_n0, assign19470_e14559_d_n2, assign19470_e14559_d_n4, assign19470_e14559_d_n5, assign19470_e14559_d_n6, assign19470_e14559_d_n7, assign19470_e14559_d_n8, assign19470_e14559_d_n9, assign19470_e14559_d_n10, assign19470_e14559_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign19470_e14555: f64 = (locals.var_nin * locals.var_nin);
        let assign19470_e14557: f64 = (assign19470_e14555 * locals.var_nsti_p2);
        (assign19470_e14557, (((locals.var_nin_dn0 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn0)) * locals.var_nsti_p2), (((locals.var_nin_dn2 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn2)) * locals.var_nsti_p2), (((locals.var_nin_dn4 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn4)) * locals.var_nsti_p2), (((locals.var_nin_dn5 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn5)) * locals.var_nsti_p2), (((locals.var_nin_dn6 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn6)) * locals.var_nsti_p2), (((locals.var_nin_dn7 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn7)) * locals.var_nsti_p2), (((locals.var_nin_dn8 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn8)) * locals.var_nsti_p2), (((locals.var_nin_dn9 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn9)) * locals.var_nsti_p2), (((locals.var_nin_dn10 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn10)) * locals.var_nsti_p2), (((locals.var_nin_dn13 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn13)) * locals.var_nsti_p2),)
    } else {
        (locals.var_costi1, locals.var_costi1_dn0, locals.var_costi1_dn2, locals.var_costi1_dn4, locals.var_costi1_dn5, locals.var_costi1_dn6, locals.var_costi1_dn7, locals.var_costi1_dn8, locals.var_costi1_dn9, locals.var_costi1_dn10, locals.var_costi1_dn13,)
    }
};
        locals.var_costi1 = assign19470_e14559;
        locals.var_costi1_dn0 = assign19470_e14559_d_n0;
        locals.var_costi1_dn2 = assign19470_e14559_d_n2;
        locals.var_costi1_dn4 = assign19470_e14559_d_n4;
        locals.var_costi1_dn5 = assign19470_e14559_d_n5;
        locals.var_costi1_dn6 = assign19470_e14559_d_n6;
        locals.var_costi1_dn7 = assign19470_e14559_d_n7;
        locals.var_costi1_dn8 = assign19470_e14559_d_n8;
        locals.var_costi1_dn9 = assign19470_e14559_d_n9;
        locals.var_costi1_dn10 = assign19470_e14559_d_n10;
        locals.var_costi1_dn13 = assign19470_e14559_d_n13;

        let (assign19480_e14567, assign19480_e14567_d_n0, assign19480_e14567_d_n2, assign19480_e14567_d_n4, assign19480_e14567_d_n5, assign19480_e14567_d_n6, assign19480_e14567_d_n7, assign19480_e14567_d_n8, assign19480_e14567_d_n9, assign19480_e14567_d_n10, assign19480_e14567_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign19480_e14564: f64 = (p.p448 * locals.var_tdiff);
        let assign19480_e14565: f64 = (p.p447 + assign19480_e14564);
        (assign19480_e14565, (p.p448 * locals.var_tdiff_dn0), (p.p448 * locals.var_tdiff_dn2), (p.p448 * locals.var_tdiff_dn4), (p.p448 * locals.var_tdiff_dn5), (p.p448 * locals.var_tdiff_dn6), (p.p448 * locals.var_tdiff_dn7), (p.p448 * locals.var_tdiff_dn8), (p.p448 * locals.var_tdiff_dn9), (p.p448 * locals.var_tdiff_dn10), (p.p448 * locals.var_tdiff_dn13),)
    } else {
        (locals.var_hbdceff, locals.var_hbdceff_dn0, locals.var_hbdceff_dn2, locals.var_hbdceff_dn4, locals.var_hbdceff_dn5, locals.var_hbdceff_dn6, locals.var_hbdceff_dn7, locals.var_hbdceff_dn8, locals.var_hbdceff_dn9, locals.var_hbdceff_dn10, locals.var_hbdceff_dn13,)
    }
};
        locals.var_hbdceff = assign19480_e14567;
        locals.var_hbdceff_dn0 = assign19480_e14567_d_n0;
        locals.var_hbdceff_dn2 = assign19480_e14567_d_n2;
        locals.var_hbdceff_dn4 = assign19480_e14567_d_n4;
        locals.var_hbdceff_dn5 = assign19480_e14567_d_n5;
        locals.var_hbdceff_dn6 = assign19480_e14567_d_n6;
        locals.var_hbdceff_dn7 = assign19480_e14567_d_n7;
        locals.var_hbdceff_dn8 = assign19480_e14567_d_n8;
        locals.var_hbdceff_dn9 = assign19480_e14567_d_n9;
        locals.var_hbdceff_dn10 = assign19480_e14567_d_n10;
        locals.var_hbdceff_dn13 = assign19480_e14567_d_n13;

        let (assign19490_e14571,) = {
    if (locals.var_guard352 != 0.0) {
        (p.p193,)
    } else {
        (locals.var_uc_subtmp,)
    }
};
        locals.var_uc_subtmp = assign19490_e14571;

        let assign19520_e14584: f64 = if locals.var_uc_subtmp < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard389 = assign19520_e14584;

        let (assign19530_e14590,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard389 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_uc_subtmp,)
    }
};
        locals.var_uc_subtmp = assign19530_e14590;

        let assign19540_e14593: f64 = if locals.var_uc_subtmp > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard390 = assign19540_e14593;

        let (assign19550_e14599,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard390 != 0.0)) {
        (0.005,)
    } else {
        (locals.var_uc_subtmp,)
    }
};
        locals.var_uc_subtmp = assign19550_e14599;

        let (assign19560_e14606, assign19560_e14606_d_n0, assign19560_e14606_d_n2, assign19560_e14606_d_n4, assign19560_e14606_d_n5, assign19560_e14606_d_n6, assign19560_e14606_d_n7, assign19560_e14606_d_n8, assign19560_e14606_d_n9, assign19560_e14606_d_n10, assign19560_e14606_d_n13,) = {
    if (locals.var_guard352 == 0.0) {
        let assign19560_e14602: f64 = ctx_temp;
        let assign19560_e14604: f64 = (assign19560_e14602 + p.p11);
        (assign19560_e14604, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ttemp, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn13,)
    }
};
        locals.var_ttemp = assign19560_e14606;
        locals.var_ttemp_dn0 = assign19560_e14606_d_n0;
        locals.var_ttemp_dn2 = assign19560_e14606_d_n2;
        locals.var_ttemp_dn4 = assign19560_e14606_d_n4;
        locals.var_ttemp_dn5 = assign19560_e14606_d_n5;
        locals.var_ttemp_dn6 = assign19560_e14606_d_n6;
        locals.var_ttemp_dn7 = assign19560_e14606_d_n7;
        locals.var_ttemp_dn8 = assign19560_e14606_d_n8;
        locals.var_ttemp_dn9 = assign19560_e14606_d_n9;
        locals.var_ttemp_dn10 = assign19560_e14606_d_n10;
        locals.var_ttemp_dn13 = assign19560_e14606_d_n13;

        let assign19570_e14609: f64 = (locals.var_weff_ld * p.p7);
        locals.var_weffld_nf = assign19570_e14609;

        let assign19580_e14612: f64 = (p.p67 + p.p68);
        locals.var_ldrift0 = assign19580_e14612;

        locals.var_vfb = locals.var_uc_vfbc;

        locals.var_vmaxe = locals.var_vmaxeff;
        locals.var_vmaxe_dn0 = locals.var_vmaxeff_dn0;
        locals.var_vmaxe_dn2 = locals.var_vmaxeff_dn2;
        locals.var_vmaxe_dn4 = locals.var_vmaxeff_dn4;
        locals.var_vmaxe_dn5 = locals.var_vmaxeff_dn5;
        locals.var_vmaxe_dn6 = locals.var_vmaxeff_dn6;
        locals.var_vmaxe_dn7 = locals.var_vmaxeff_dn7;
        locals.var_vmaxe_dn8 = locals.var_vmaxeff_dn8;
        locals.var_vmaxe_dn9 = locals.var_vmaxeff_dn9;
        locals.var_vmaxe_dn10 = locals.var_vmaxeff_dn10;
        locals.var_vmaxe_dn13 = locals.var_vmaxeff_dn13;

        locals.var_c_eox = locals.var_cecox;

        locals.var_tox0 = p.p95;

        let assign19630_e14619: f64 = (locals.var_c_eox / locals.var_tox0);
        locals.var_cox0 = assign19630_e14619;

        let assign19640_e14622: f64 = (1.0 / locals.var_cox0);
        locals.var_cox0_inv = assign19640_e14622;

        let assign19650_e14625: f64 = (locals.var_c_eox / locals.var_uc_toxb);
        locals.var_coxb0 = assign19650_e14625;

        let assign19660_e14628: f64 = (p.p87 * p.p434);
        locals.var_vgs_min = assign19660_e14628;

        let assign19670_e14632: f64 = (locals.var_pb2 - p.p262);
        let assign19670_e14633: f64 = (0.8 - assign19670_e14632);
        let assign19670_e14635: f64 = (assign19670_e14633 - 0.1);
        locals.var_tmf1 = assign19670_e14635;
        locals.var_tmf1_dn0 = (-locals.var_pb2_dn0);
        locals.var_tmf1_dn2 = (-locals.var_pb2_dn2);
        locals.var_tmf1_dn4 = (-locals.var_pb2_dn4);
        locals.var_tmf1_dn5 = (-locals.var_pb2_dn5);
        locals.var_tmf1_dn6 = (-locals.var_pb2_dn6);
        locals.var_tmf1_dn7 = (-locals.var_pb2_dn7);
        locals.var_tmf1_dn8 = (-locals.var_pb2_dn8);
        locals.var_tmf1_dn9 = (-locals.var_pb2_dn9);
        locals.var_tmf1_dn10 = (-locals.var_pb2_dn10);
        locals.var_tmf1_dn13 = (-locals.var_pb2_dn13);

        let assign19680_e14638: f64 = (4.0 * 0.8);
        let assign19680_e14640: f64 = (assign19680_e14638 * 0.1);
        locals.var_tmf2 = assign19680_e14640;
        locals.var_tmf2_dn0 = 0.0;
        locals.var_tmf2_dn2 = 0.0;
        locals.var_tmf2_dn4 = 0.0;
        locals.var_tmf2_dn5 = 0.0;
        locals.var_tmf2_dn6 = 0.0;
        locals.var_tmf2_dn7 = 0.0;
        locals.var_tmf2_dn8 = 0.0;
        locals.var_tmf2_dn9 = 0.0;
        locals.var_tmf2_dn10 = 0.0;
        locals.var_tmf2_dn13 = 0.0;

        let (assign19690_e14647, assign19690_e14647_d_n0, assign19690_e14647_d_n2, assign19690_e14647_d_n4, assign19690_e14647_d_n5, assign19690_e14647_d_n6, assign19690_e14647_d_n7, assign19690_e14647_d_n8, assign19690_e14647_d_n9, assign19690_e14647_d_n10, assign19690_e14647_d_n13,) = {
    if (locals.var_tmf2 > 0.0) {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    } else {
        let assign19690_e14646: f64 = (-locals.var_tmf2);
        (assign19690_e14646, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
    }
};
        locals.var_tmf2 = assign19690_e14647;
        locals.var_tmf2_dn0 = assign19690_e14647_d_n0;
        locals.var_tmf2_dn2 = assign19690_e14647_d_n2;
        locals.var_tmf2_dn4 = assign19690_e14647_d_n4;
        locals.var_tmf2_dn5 = assign19690_e14647_d_n5;
        locals.var_tmf2_dn6 = assign19690_e14647_d_n6;
        locals.var_tmf2_dn7 = assign19690_e14647_d_n7;
        locals.var_tmf2_dn8 = assign19690_e14647_d_n8;
        locals.var_tmf2_dn9 = assign19690_e14647_d_n9;
        locals.var_tmf2_dn10 = assign19690_e14647_d_n10;
        locals.var_tmf2_dn13 = assign19690_e14647_d_n13;

        let assign19700_e14650: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign19700_e14652: f64 = (assign19700_e14650 + locals.var_tmf2);
        let assign19700_e14653: f64 = (assign19700_e14652).sqrt();
        locals.var_tmf2 = assign19700_e14653;
        locals.var_tmf2_dn0 = ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign19700_e14653));
        locals.var_tmf2_dn2 = ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign19700_e14653));
        locals.var_tmf2_dn4 = ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign19700_e14653));
        locals.var_tmf2_dn5 = ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign19700_e14653));
        locals.var_tmf2_dn6 = ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign19700_e14653));
        locals.var_tmf2_dn7 = ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign19700_e14653));
        locals.var_tmf2_dn8 = ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign19700_e14653));
        locals.var_tmf2_dn9 = ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign19700_e14653));
        locals.var_tmf2_dn10 = ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign19700_e14653));
        locals.var_tmf2_dn13 = ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign19700_e14653));

        let assign19710_e14658: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign19710_e14659: f64 = (1.0 + assign19710_e14658);
        let assign19710_e14660: f64 = (0.5 * assign19710_e14659);
        locals.var_t0 = assign19710_e14660;
        locals.var_t0_dn0 = (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn2 = (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn4 = (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn5 = (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn6 = (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn7 = (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn8 = (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn9 = (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn10 = (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn13 = (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2)));

        let assign19720_e14665: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign19720_e14666: f64 = (0.5 * assign19720_e14665);
        let assign19720_e14667: f64 = (0.8 - assign19720_e14666);
        locals.var_t1 = assign19720_e14667;
        locals.var_t1_dn0 = (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)));
        locals.var_t1_dn2 = (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)));
        locals.var_t1_dn4 = (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)));
        locals.var_t1_dn5 = (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)));
        locals.var_t1_dn6 = (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)));
        locals.var_t1_dn7 = (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)));
        locals.var_t1_dn8 = (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)));
        locals.var_t1_dn9 = (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)));
        locals.var_t1_dn10 = (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)));
        locals.var_t1_dn13 = (-(0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)));

        locals.var_vbs_max = locals.var_t1;
        locals.var_vbs_max_dn0 = locals.var_t1_dn0;
        locals.var_vbs_max_dn2 = locals.var_t1_dn2;
        locals.var_vbs_max_dn4 = locals.var_t1_dn4;
        locals.var_vbs_max_dn5 = locals.var_t1_dn5;
        locals.var_vbs_max_dn6 = locals.var_t1_dn6;
        locals.var_vbs_max_dn7 = locals.var_t1_dn7;
        locals.var_vbs_max_dn8 = locals.var_t1_dn8;
        locals.var_vbs_max_dn9 = locals.var_t1_dn9;
        locals.var_vbs_max_dn10 = locals.var_t1_dn10;
        locals.var_vbs_max_dn13 = locals.var_t1_dn13;

        let assign19740_e14671: f64 = (locals.var_pb20 - p.p262);
        let assign19740_e14673: f64 = if assign19740_e14671 < locals.var_vbs_max { 1.0 } else { 0.0 };
        locals.var_guard391 = assign19740_e14673;

        let (assign19750_e14679, assign19750_e14679_d_n0, assign19750_e14679_d_n2, assign19750_e14679_d_n4, assign19750_e14679_d_n5, assign19750_e14679_d_n6, assign19750_e14679_d_n7, assign19750_e14679_d_n8, assign19750_e14679_d_n9, assign19750_e14679_d_n10, assign19750_e14679_d_n13,) = {
    if (locals.var_guard391 != 0.0) {
        let assign19750_e14677: f64 = (locals.var_pb20 - p.p262);
        (assign19750_e14677, locals.var_pb20_dn0, locals.var_pb20_dn2, locals.var_pb20_dn4, locals.var_pb20_dn5, locals.var_pb20_dn6, locals.var_pb20_dn7, locals.var_pb20_dn8, locals.var_pb20_dn9, locals.var_pb20_dn10, locals.var_pb20_dn13,)
    } else {
        (locals.var_vbs_max, locals.var_vbs_max_dn0, locals.var_vbs_max_dn2, locals.var_vbs_max_dn4, locals.var_vbs_max_dn5, locals.var_vbs_max_dn6, locals.var_vbs_max_dn7, locals.var_vbs_max_dn8, locals.var_vbs_max_dn9, locals.var_vbs_max_dn10, locals.var_vbs_max_dn13,)
    }
};
        locals.var_vbs_max = assign19750_e14679;
        locals.var_vbs_max_dn0 = assign19750_e14679_d_n0;
        locals.var_vbs_max_dn2 = assign19750_e14679_d_n2;
        locals.var_vbs_max_dn4 = assign19750_e14679_d_n4;
        locals.var_vbs_max_dn5 = assign19750_e14679_d_n5;
        locals.var_vbs_max_dn6 = assign19750_e14679_d_n6;
        locals.var_vbs_max_dn7 = assign19750_e14679_d_n7;
        locals.var_vbs_max_dn8 = assign19750_e14679_d_n8;
        locals.var_vbs_max_dn9 = assign19750_e14679_d_n9;
        locals.var_vbs_max_dn10 = assign19750_e14679_d_n10;
        locals.var_vbs_max_dn13 = assign19750_e14679_d_n13;

        let assign19760_e14682: f64 = (locals.var_pb2c - p.p262);
        let assign19760_e14684: f64 = if assign19760_e14682 < locals.var_vbs_max { 1.0 } else { 0.0 };
        locals.var_guard392 = assign19760_e14684;

        let (assign19770_e14690, assign19770_e14690_d_n0, assign19770_e14690_d_n2, assign19770_e14690_d_n4, assign19770_e14690_d_n5, assign19770_e14690_d_n6, assign19770_e14690_d_n7, assign19770_e14690_d_n8, assign19770_e14690_d_n9, assign19770_e14690_d_n10, assign19770_e14690_d_n13,) = {
    if (locals.var_guard392 != 0.0) {
        let assign19770_e14688: f64 = (locals.var_pb2c - p.p262);
        (assign19770_e14688, locals.var_pb2c_dn0, locals.var_pb2c_dn2, locals.var_pb2c_dn4, locals.var_pb2c_dn5, locals.var_pb2c_dn6, locals.var_pb2c_dn7, locals.var_pb2c_dn8, locals.var_pb2c_dn9, locals.var_pb2c_dn10, locals.var_pb2c_dn13,)
    } else {
        (locals.var_vbs_max, locals.var_vbs_max_dn0, locals.var_vbs_max_dn2, locals.var_vbs_max_dn4, locals.var_vbs_max_dn5, locals.var_vbs_max_dn6, locals.var_vbs_max_dn7, locals.var_vbs_max_dn8, locals.var_vbs_max_dn9, locals.var_vbs_max_dn10, locals.var_vbs_max_dn13,)
    }
};
        locals.var_vbs_max = assign19770_e14690;
        locals.var_vbs_max_dn0 = assign19770_e14690_d_n0;
        locals.var_vbs_max_dn2 = assign19770_e14690_d_n2;
        locals.var_vbs_max_dn4 = assign19770_e14690_d_n4;
        locals.var_vbs_max_dn5 = assign19770_e14690_d_n5;
        locals.var_vbs_max_dn6 = assign19770_e14690_d_n6;
        locals.var_vbs_max_dn7 = assign19770_e14690_d_n7;
        locals.var_vbs_max_dn8 = assign19770_e14690_d_n8;
        locals.var_vbs_max_dn9 = assign19770_e14690_d_n9;
        locals.var_vbs_max_dn10 = assign19770_e14690_d_n10;
        locals.var_vbs_max_dn13 = assign19770_e14690_d_n13;

        let assign19780_e14697: f64 = if ((locals.var_uc_codep > 0.0) && (locals.var_uc_codep <= 3.0)) { 1.0 } else { 0.0 };
        locals.var_guard393 = assign19780_e14697;

        let assign19790_e14700: f64 = (locals.var_pb2n - p.p262);
        let assign19790_e14702: f64 = if assign19790_e14700 < locals.var_vbs_max { 1.0 } else { 0.0 };
        locals.var_guard394 = assign19790_e14702;

        let (assign19800_e14710, assign19800_e14710_d_n0, assign19800_e14710_d_n2, assign19800_e14710_d_n4, assign19800_e14710_d_n5, assign19800_e14710_d_n6, assign19800_e14710_d_n7, assign19800_e14710_d_n8, assign19800_e14710_d_n9, assign19800_e14710_d_n10, assign19800_e14710_d_n13,) = {
    if ((locals.var_guard393 != 0.0) && (locals.var_guard394 != 0.0)) {
        let assign19800_e14708: f64 = (locals.var_pb2n - p.p262);
        (assign19800_e14708, locals.var_pb2n_dn0, locals.var_pb2n_dn2, locals.var_pb2n_dn4, locals.var_pb2n_dn5, locals.var_pb2n_dn6, locals.var_pb2n_dn7, locals.var_pb2n_dn8, locals.var_pb2n_dn9, locals.var_pb2n_dn10, locals.var_pb2n_dn13,)
    } else {
        (locals.var_vbs_max, locals.var_vbs_max_dn0, locals.var_vbs_max_dn2, locals.var_vbs_max_dn4, locals.var_vbs_max_dn5, locals.var_vbs_max_dn6, locals.var_vbs_max_dn7, locals.var_vbs_max_dn8, locals.var_vbs_max_dn9, locals.var_vbs_max_dn10, locals.var_vbs_max_dn13,)
    }
};
        locals.var_vbs_max = assign19800_e14710;
        locals.var_vbs_max_dn0 = assign19800_e14710_d_n0;
        locals.var_vbs_max_dn2 = assign19800_e14710_d_n2;
        locals.var_vbs_max_dn4 = assign19800_e14710_d_n4;
        locals.var_vbs_max_dn5 = assign19800_e14710_d_n5;
        locals.var_vbs_max_dn6 = assign19800_e14710_d_n6;
        locals.var_vbs_max_dn7 = assign19800_e14710_d_n7;
        locals.var_vbs_max_dn8 = assign19800_e14710_d_n8;
        locals.var_vbs_max_dn9 = assign19800_e14710_d_n9;
        locals.var_vbs_max_dn10 = assign19800_e14710_d_n10;
        locals.var_vbs_max_dn13 = assign19800_e14710_d_n13;

        let assign19810_e14713: f64 = (locals.var_vbipn - p.p262);
        let assign19810_e14715: f64 = if assign19810_e14713 < locals.var_vbs_max { 1.0 } else { 0.0 };
        locals.var_guard395 = assign19810_e14715;

        let (assign19820_e14723, assign19820_e14723_d_n0, assign19820_e14723_d_n2, assign19820_e14723_d_n4, assign19820_e14723_d_n5, assign19820_e14723_d_n6, assign19820_e14723_d_n7, assign19820_e14723_d_n8, assign19820_e14723_d_n9, assign19820_e14723_d_n10, assign19820_e14723_d_n13,) = {
    if ((locals.var_guard393 != 0.0) && (locals.var_guard395 != 0.0)) {
        let assign19820_e14721: f64 = (locals.var_vbipn - p.p262);
        (assign19820_e14721, locals.var_vbipn_dn0, locals.var_vbipn_dn2, locals.var_vbipn_dn4, locals.var_vbipn_dn5, locals.var_vbipn_dn6, locals.var_vbipn_dn7, locals.var_vbipn_dn8, locals.var_vbipn_dn9, locals.var_vbipn_dn10, locals.var_vbipn_dn13,)
    } else {
        (locals.var_vbs_max, locals.var_vbs_max_dn0, locals.var_vbs_max_dn2, locals.var_vbs_max_dn4, locals.var_vbs_max_dn5, locals.var_vbs_max_dn6, locals.var_vbs_max_dn7, locals.var_vbs_max_dn8, locals.var_vbs_max_dn9, locals.var_vbs_max_dn10, locals.var_vbs_max_dn13,)
    }
};
        locals.var_vbs_max = assign19820_e14723;
        locals.var_vbs_max_dn0 = assign19820_e14723_d_n0;
        locals.var_vbs_max_dn2 = assign19820_e14723_d_n2;
        locals.var_vbs_max_dn4 = assign19820_e14723_d_n4;
        locals.var_vbs_max_dn5 = assign19820_e14723_d_n5;
        locals.var_vbs_max_dn6 = assign19820_e14723_d_n6;
        locals.var_vbs_max_dn7 = assign19820_e14723_d_n7;
        locals.var_vbs_max_dn8 = assign19820_e14723_d_n8;
        locals.var_vbs_max_dn9 = assign19820_e14723_d_n9;
        locals.var_vbs_max_dn10 = assign19820_e14723_d_n10;
        locals.var_vbs_max_dn13 = assign19820_e14723_d_n13;

    }

    pub(super) fn stamp_transient_block_45(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let assign19830_e14727: f64 = (locals.var_vbs_max * 0.5);
        let assign19830_e14728: f64 = if locals.var_vbs_bnd > assign19830_e14727 { 1.0 } else { 0.0 };
        locals.var_guard396 = assign19830_e14728;

        let (assign19840_e14734, assign19840_e14734_d_n0, assign19840_e14734_d_n2, assign19840_e14734_d_n4, assign19840_e14734_d_n5, assign19840_e14734_d_n6, assign19840_e14734_d_n7, assign19840_e14734_d_n8, assign19840_e14734_d_n9, assign19840_e14734_d_n10, assign19840_e14734_d_n13,) = {
    if (locals.var_guard396 != 0.0) {
        let assign19840_e14732: f64 = (0.5 * locals.var_vbs_max);
        (assign19840_e14732, (0.5 * locals.var_vbs_max_dn0), (0.5 * locals.var_vbs_max_dn2), (0.5 * locals.var_vbs_max_dn4), (0.5 * locals.var_vbs_max_dn5), (0.5 * locals.var_vbs_max_dn6), (0.5 * locals.var_vbs_max_dn7), (0.5 * locals.var_vbs_max_dn8), (0.5 * locals.var_vbs_max_dn9), (0.5 * locals.var_vbs_max_dn10), (0.5 * locals.var_vbs_max_dn13),)
    } else {
        (locals.var_vbs_bnd, locals.var_vbs_bnd_dn0, locals.var_vbs_bnd_dn2, locals.var_vbs_bnd_dn4, locals.var_vbs_bnd_dn5, locals.var_vbs_bnd_dn6, locals.var_vbs_bnd_dn7, locals.var_vbs_bnd_dn8, locals.var_vbs_bnd_dn9, locals.var_vbs_bnd_dn10, locals.var_vbs_bnd_dn13,)
    }
};
        locals.var_vbs_bnd = assign19840_e14734;
        locals.var_vbs_bnd_dn0 = assign19840_e14734_d_n0;
        locals.var_vbs_bnd_dn2 = assign19840_e14734_d_n2;
        locals.var_vbs_bnd_dn4 = assign19840_e14734_d_n4;
        locals.var_vbs_bnd_dn5 = assign19840_e14734_d_n5;
        locals.var_vbs_bnd_dn6 = assign19840_e14734_d_n6;
        locals.var_vbs_bnd_dn7 = assign19840_e14734_d_n7;
        locals.var_vbs_bnd_dn8 = assign19840_e14734_d_n8;
        locals.var_vbs_bnd_dn9 = assign19840_e14734_d_n9;
        locals.var_vbs_bnd_dn10 = assign19840_e14734_d_n10;
        locals.var_vbs_bnd_dn13 = assign19840_e14734_d_n13;

        let assign19850_e14736: f64 = if param_given[338] { 1.0 } else { 0.0 };
        locals.var_guard397 = assign19850_e14736;

        let (assign19860_e14740, assign19860_e14740_d_n0, assign19860_e14740_d_n2, assign19860_e14740_d_n4, assign19860_e14740_d_n5, assign19860_e14740_d_n6, assign19860_e14740_d_n7, assign19860_e14740_d_n8, assign19860_e14740_d_n9, assign19860_e14740_d_n10, assign19860_e14740_d_n13,) = {
    if (locals.var_guard397 != 0.0) {
        (p.p338, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_max_local, locals.var_vbs_max_local_dn0, locals.var_vbs_max_local_dn2, locals.var_vbs_max_local_dn4, locals.var_vbs_max_local_dn5, locals.var_vbs_max_local_dn6, locals.var_vbs_max_local_dn7, locals.var_vbs_max_local_dn8, locals.var_vbs_max_local_dn9, locals.var_vbs_max_local_dn10, locals.var_vbs_max_local_dn13,)
    }
};
        locals.var_vbs_max_local = assign19860_e14740;
        locals.var_vbs_max_local_dn0 = assign19860_e14740_d_n0;
        locals.var_vbs_max_local_dn2 = assign19860_e14740_d_n2;
        locals.var_vbs_max_local_dn4 = assign19860_e14740_d_n4;
        locals.var_vbs_max_local_dn5 = assign19860_e14740_d_n5;
        locals.var_vbs_max_local_dn6 = assign19860_e14740_d_n6;
        locals.var_vbs_max_local_dn7 = assign19860_e14740_d_n7;
        locals.var_vbs_max_local_dn8 = assign19860_e14740_d_n8;
        locals.var_vbs_max_local_dn9 = assign19860_e14740_d_n9;
        locals.var_vbs_max_local_dn10 = assign19860_e14740_d_n10;
        locals.var_vbs_max_local_dn13 = assign19860_e14740_d_n13;

        let (assign19870_e14745, assign19870_e14745_d_n0, assign19870_e14745_d_n2, assign19870_e14745_d_n4, assign19870_e14745_d_n5, assign19870_e14745_d_n6, assign19870_e14745_d_n7, assign19870_e14745_d_n8, assign19870_e14745_d_n9, assign19870_e14745_d_n10, assign19870_e14745_d_n13,) = {
    if (locals.var_guard397 == 0.0) {
        (locals.var_vbs_max, locals.var_vbs_max_dn0, locals.var_vbs_max_dn2, locals.var_vbs_max_dn4, locals.var_vbs_max_dn5, locals.var_vbs_max_dn6, locals.var_vbs_max_dn7, locals.var_vbs_max_dn8, locals.var_vbs_max_dn9, locals.var_vbs_max_dn10, locals.var_vbs_max_dn13,)
    } else {
        (locals.var_vbs_max_local, locals.var_vbs_max_local_dn0, locals.var_vbs_max_local_dn2, locals.var_vbs_max_local_dn4, locals.var_vbs_max_local_dn5, locals.var_vbs_max_local_dn6, locals.var_vbs_max_local_dn7, locals.var_vbs_max_local_dn8, locals.var_vbs_max_local_dn9, locals.var_vbs_max_local_dn10, locals.var_vbs_max_local_dn13,)
    }
};
        locals.var_vbs_max_local = assign19870_e14745;
        locals.var_vbs_max_local_dn0 = assign19870_e14745_d_n0;
        locals.var_vbs_max_local_dn2 = assign19870_e14745_d_n2;
        locals.var_vbs_max_local_dn4 = assign19870_e14745_d_n4;
        locals.var_vbs_max_local_dn5 = assign19870_e14745_d_n5;
        locals.var_vbs_max_local_dn6 = assign19870_e14745_d_n6;
        locals.var_vbs_max_local_dn7 = assign19870_e14745_d_n7;
        locals.var_vbs_max_local_dn8 = assign19870_e14745_d_n8;
        locals.var_vbs_max_local_dn9 = assign19870_e14745_d_n9;
        locals.var_vbs_max_local_dn10 = assign19870_e14745_d_n10;
        locals.var_vbs_max_local_dn13 = assign19870_e14745_d_n13;

        let assign19880_e14747: f64 = if param_given[339] { 1.0 } else { 0.0 };
        locals.var_guard398 = assign19880_e14747;

        let (assign19890_e14751, assign19890_e14751_d_n0, assign19890_e14751_d_n2, assign19890_e14751_d_n4, assign19890_e14751_d_n5, assign19890_e14751_d_n6, assign19890_e14751_d_n7, assign19890_e14751_d_n8, assign19890_e14751_d_n9, assign19890_e14751_d_n10, assign19890_e14751_d_n13,) = {
    if (locals.var_guard398 != 0.0) {
        (p.p339, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_bnd_local, locals.var_vbs_bnd_local_dn0, locals.var_vbs_bnd_local_dn2, locals.var_vbs_bnd_local_dn4, locals.var_vbs_bnd_local_dn5, locals.var_vbs_bnd_local_dn6, locals.var_vbs_bnd_local_dn7, locals.var_vbs_bnd_local_dn8, locals.var_vbs_bnd_local_dn9, locals.var_vbs_bnd_local_dn10, locals.var_vbs_bnd_local_dn13,)
    }
};
        locals.var_vbs_bnd_local = assign19890_e14751;
        locals.var_vbs_bnd_local_dn0 = assign19890_e14751_d_n0;
        locals.var_vbs_bnd_local_dn2 = assign19890_e14751_d_n2;
        locals.var_vbs_bnd_local_dn4 = assign19890_e14751_d_n4;
        locals.var_vbs_bnd_local_dn5 = assign19890_e14751_d_n5;
        locals.var_vbs_bnd_local_dn6 = assign19890_e14751_d_n6;
        locals.var_vbs_bnd_local_dn7 = assign19890_e14751_d_n7;
        locals.var_vbs_bnd_local_dn8 = assign19890_e14751_d_n8;
        locals.var_vbs_bnd_local_dn9 = assign19890_e14751_d_n9;
        locals.var_vbs_bnd_local_dn10 = assign19890_e14751_d_n10;
        locals.var_vbs_bnd_local_dn13 = assign19890_e14751_d_n13;

        let assign19900_e14753: f64 = if param_given[338] { 1.0 } else { 0.0 };
        locals.var_guard399 = assign19900_e14753;

        let (assign19910_e14762, assign19910_e14762_d_n0, assign19910_e14762_d_n2, assign19910_e14762_d_n4, assign19910_e14762_d_n5, assign19910_e14762_d_n6, assign19910_e14762_d_n7, assign19910_e14762_d_n8, assign19910_e14762_d_n9, assign19910_e14762_d_n10, assign19910_e14762_d_n13,) = {
    if ((locals.var_guard398 == 0.0) && (locals.var_guard399 != 0.0)) {
        let assign19910_e14760: f64 = (0.5 * locals.var_vbs_max_local);
        (assign19910_e14760, (0.5 * locals.var_vbs_max_local_dn0), (0.5 * locals.var_vbs_max_local_dn2), (0.5 * locals.var_vbs_max_local_dn4), (0.5 * locals.var_vbs_max_local_dn5), (0.5 * locals.var_vbs_max_local_dn6), (0.5 * locals.var_vbs_max_local_dn7), (0.5 * locals.var_vbs_max_local_dn8), (0.5 * locals.var_vbs_max_local_dn9), (0.5 * locals.var_vbs_max_local_dn10), (0.5 * locals.var_vbs_max_local_dn13),)
    } else {
        (locals.var_vbs_bnd_local, locals.var_vbs_bnd_local_dn0, locals.var_vbs_bnd_local_dn2, locals.var_vbs_bnd_local_dn4, locals.var_vbs_bnd_local_dn5, locals.var_vbs_bnd_local_dn6, locals.var_vbs_bnd_local_dn7, locals.var_vbs_bnd_local_dn8, locals.var_vbs_bnd_local_dn9, locals.var_vbs_bnd_local_dn10, locals.var_vbs_bnd_local_dn13,)
    }
};
        locals.var_vbs_bnd_local = assign19910_e14762;
        locals.var_vbs_bnd_local_dn0 = assign19910_e14762_d_n0;
        locals.var_vbs_bnd_local_dn2 = assign19910_e14762_d_n2;
        locals.var_vbs_bnd_local_dn4 = assign19910_e14762_d_n4;
        locals.var_vbs_bnd_local_dn5 = assign19910_e14762_d_n5;
        locals.var_vbs_bnd_local_dn6 = assign19910_e14762_d_n6;
        locals.var_vbs_bnd_local_dn7 = assign19910_e14762_d_n7;
        locals.var_vbs_bnd_local_dn8 = assign19910_e14762_d_n8;
        locals.var_vbs_bnd_local_dn9 = assign19910_e14762_d_n9;
        locals.var_vbs_bnd_local_dn10 = assign19910_e14762_d_n10;
        locals.var_vbs_bnd_local_dn13 = assign19910_e14762_d_n13;

        let (assign19920_e14770, assign19920_e14770_d_n0, assign19920_e14770_d_n2, assign19920_e14770_d_n4, assign19920_e14770_d_n5, assign19920_e14770_d_n6, assign19920_e14770_d_n7, assign19920_e14770_d_n8, assign19920_e14770_d_n9, assign19920_e14770_d_n10, assign19920_e14770_d_n13,) = {
    if ((locals.var_guard398 == 0.0) && (locals.var_guard399 == 0.0)) {
        (locals.var_vbs_bnd, locals.var_vbs_bnd_dn0, locals.var_vbs_bnd_dn2, locals.var_vbs_bnd_dn4, locals.var_vbs_bnd_dn5, locals.var_vbs_bnd_dn6, locals.var_vbs_bnd_dn7, locals.var_vbs_bnd_dn8, locals.var_vbs_bnd_dn9, locals.var_vbs_bnd_dn10, locals.var_vbs_bnd_dn13,)
    } else {
        (locals.var_vbs_bnd_local, locals.var_vbs_bnd_local_dn0, locals.var_vbs_bnd_local_dn2, locals.var_vbs_bnd_local_dn4, locals.var_vbs_bnd_local_dn5, locals.var_vbs_bnd_local_dn6, locals.var_vbs_bnd_local_dn7, locals.var_vbs_bnd_local_dn8, locals.var_vbs_bnd_local_dn9, locals.var_vbs_bnd_local_dn10, locals.var_vbs_bnd_local_dn13,)
    }
};
        locals.var_vbs_bnd_local = assign19920_e14770;
        locals.var_vbs_bnd_local_dn0 = assign19920_e14770_d_n0;
        locals.var_vbs_bnd_local_dn2 = assign19920_e14770_d_n2;
        locals.var_vbs_bnd_local_dn4 = assign19920_e14770_d_n4;
        locals.var_vbs_bnd_local_dn5 = assign19920_e14770_d_n5;
        locals.var_vbs_bnd_local_dn6 = assign19920_e14770_d_n6;
        locals.var_vbs_bnd_local_dn7 = assign19920_e14770_d_n7;
        locals.var_vbs_bnd_local_dn8 = assign19920_e14770_d_n8;
        locals.var_vbs_bnd_local_dn9 = assign19920_e14770_d_n9;
        locals.var_vbs_bnd_local_dn10 = assign19920_e14770_d_n10;
        locals.var_vbs_bnd_local_dn13 = assign19920_e14770_d_n13;

        let assign19930_e14774: f64 = (locals.var_vbs_max_local * 0.5);
        let assign19930_e14775: f64 = if locals.var_vbs_bnd_local > assign19930_e14774 { 1.0 } else { 0.0 };
        locals.var_guard400 = assign19930_e14775;

        let (assign19940_e14781, assign19940_e14781_d_n0, assign19940_e14781_d_n2, assign19940_e14781_d_n4, assign19940_e14781_d_n5, assign19940_e14781_d_n6, assign19940_e14781_d_n7, assign19940_e14781_d_n8, assign19940_e14781_d_n9, assign19940_e14781_d_n10, assign19940_e14781_d_n13,) = {
    if (locals.var_guard400 != 0.0) {
        let assign19940_e14779: f64 = (0.5 * locals.var_vbs_max_local);
        (assign19940_e14779, (0.5 * locals.var_vbs_max_local_dn0), (0.5 * locals.var_vbs_max_local_dn2), (0.5 * locals.var_vbs_max_local_dn4), (0.5 * locals.var_vbs_max_local_dn5), (0.5 * locals.var_vbs_max_local_dn6), (0.5 * locals.var_vbs_max_local_dn7), (0.5 * locals.var_vbs_max_local_dn8), (0.5 * locals.var_vbs_max_local_dn9), (0.5 * locals.var_vbs_max_local_dn10), (0.5 * locals.var_vbs_max_local_dn13),)
    } else {
        (locals.var_vbs_bnd_local, locals.var_vbs_bnd_local_dn0, locals.var_vbs_bnd_local_dn2, locals.var_vbs_bnd_local_dn4, locals.var_vbs_bnd_local_dn5, locals.var_vbs_bnd_local_dn6, locals.var_vbs_bnd_local_dn7, locals.var_vbs_bnd_local_dn8, locals.var_vbs_bnd_local_dn9, locals.var_vbs_bnd_local_dn10, locals.var_vbs_bnd_local_dn13,)
    }
};
        locals.var_vbs_bnd_local = assign19940_e14781;
        locals.var_vbs_bnd_local_dn0 = assign19940_e14781_d_n0;
        locals.var_vbs_bnd_local_dn2 = assign19940_e14781_d_n2;
        locals.var_vbs_bnd_local_dn4 = assign19940_e14781_d_n4;
        locals.var_vbs_bnd_local_dn5 = assign19940_e14781_d_n5;
        locals.var_vbs_bnd_local_dn6 = assign19940_e14781_d_n6;
        locals.var_vbs_bnd_local_dn7 = assign19940_e14781_d_n7;
        locals.var_vbs_bnd_local_dn8 = assign19940_e14781_d_n8;
        locals.var_vbs_bnd_local_dn9 = assign19940_e14781_d_n9;
        locals.var_vbs_bnd_local_dn10 = assign19940_e14781_d_n10;
        locals.var_vbs_bnd_local_dn13 = assign19940_e14781_d_n13;

        let assign19950_e14788: f64 = if ((locals.var_rse > 0.0) || (locals.var_rde > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard401 = assign19950_e14788;

        let assign19960_e14791: f64 = if locals.var_uc_corsrd == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard402 = assign19960_e14791;

        let (assign19970_e14797,) = {
    if ((locals.var_guard401 != 0.0) && (locals.var_guard402 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_rsrd,)
    }
};
        locals.var_flg_rsrd = assign19970_e14797;

        let assign19980_e14800: f64 = if locals.var_uc_corsrd == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard403 = assign19980_e14800;

        let (assign19990_e14806,) = {
    if ((locals.var_guard401 != 0.0) && (locals.var_guard403 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_flg_rsrd,)
    }
};
        locals.var_flg_rsrd = assign19990_e14806;

        let assign20000_e14809: f64 = if locals.var_uc_corsrd == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard404 = assign20000_e14809;

        let (assign20010_e14815,) = {
    if ((locals.var_guard401 != 0.0) && (locals.var_guard404 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_flg_rsrd,)
    }
};
        locals.var_flg_rsrd = assign20010_e14815;

        locals.var_flg_pprv = 0.0;

        let assign20030_e14827: f64 = (locals.var_mks_nsubsub + locals.var_uc_nover);
        let assign20030_e14828: f64 = (locals.var_uc_nover * assign20030_e14827);
        let assign20030_e14831: f64 = if (((locals.var_uc_cordrift == 1.0) && (p.p54 == 1.0)) && (assign20030_e14828 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard405 = assign20030_e14831;

        let (assign20040_e14835, assign20040_e14835_d_n0, assign20040_e14835_d_n2,) = {
    if (locals.var_guard405 != 0.0) {
        (locals.var_vdsei, locals.var_vdsei_dn0, locals.var_vdsei_dn2,)
    } else {
        (locals.var_vdsegmt, locals.var_vdsegmt_dn0, locals.var_vdsegmt_dn2,)
    }
};
        locals.var_vdsegmt = assign20040_e14835;
        locals.var_vdsegmt_dn0 = assign20040_e14835_d_n0;
        locals.var_vdsegmt_dn2 = assign20040_e14835_d_n2;

        let assign20050_e14838: f64 = if locals.var_vdsegmt >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard406 = assign20050_e14838;

        let (assign20060_e14844, assign20060_e14844_d_n0, assign20060_e14844_d_n2,) = {
    if ((locals.var_guard405 != 0.0) && (locals.var_guard406 != 0.0)) {
        (locals.var_vdsegmt, locals.var_vdsegmt_dn0, locals.var_vdsegmt_dn2,)
    } else {
        (locals.var_vdserev, locals.var_vdserev_dn0, locals.var_vdserev_dn2,)
    }
};
        locals.var_vdserev = assign20060_e14844;
        locals.var_vdserev_dn0 = assign20060_e14844_d_n0;
        locals.var_vdserev_dn2 = assign20060_e14844_d_n2;

        let (assign20070_e14850, assign20070_e14850_d_n0, assign20070_e14850_d_n2,) = {
    if ((locals.var_guard405 != 0.0) && (locals.var_guard406 != 0.0)) {
        (locals.var_vsubs, 0.0, 0.0,)
    } else {
        (locals.var_vsubsrev, locals.var_vsubsrev_dn0, locals.var_vsubsrev_dn2,)
    }
};
        locals.var_vsubsrev = assign20070_e14850;
        locals.var_vsubsrev_dn0 = assign20070_e14850_d_n0;
        locals.var_vsubsrev_dn2 = assign20070_e14850_d_n2;

        let (assign20080_e14858, assign20080_e14858_d_n0, assign20080_e14858_d_n2,) = {
    if ((locals.var_guard405 != 0.0) && (locals.var_guard406 == 0.0)) {
        let assign20080_e14856: f64 = (-locals.var_vdsegmt);
        (assign20080_e14856, (-locals.var_vdsegmt_dn0), (-locals.var_vdsegmt_dn2),)
    } else {
        (locals.var_vdserev, locals.var_vdserev_dn0, locals.var_vdserev_dn2,)
    }
};
        locals.var_vdserev = assign20080_e14858;
        locals.var_vdserev_dn0 = assign20080_e14858_d_n0;
        locals.var_vdserev_dn2 = assign20080_e14858_d_n2;

        let (assign20090_e14867, assign20090_e14867_d_n0, assign20090_e14867_d_n2,) = {
    if ((locals.var_guard405 != 0.0) && (locals.var_guard406 == 0.0)) {
        let assign20090_e14865: f64 = (locals.var_vsubs - locals.var_vdsegmt);
        (assign20090_e14865, (-locals.var_vdsegmt_dn0), (-locals.var_vdsegmt_dn2),)
    } else {
        (locals.var_vsubsrev, locals.var_vsubsrev_dn0, locals.var_vsubsrev_dn2,)
    }
};
        locals.var_vsubsrev = assign20090_e14867;
        locals.var_vsubsrev_dn0 = assign20090_e14867_d_n0;
        locals.var_vsubsrev_dn2 = assign20090_e14867_d_n2;

        let (assign20100_e14877, assign20100_e14877_d_n0, assign20100_e14877_d_n2, assign20100_e14877_d_n4, assign20100_e14877_d_n5, assign20100_e14877_d_n6, assign20100_e14877_d_n7, assign20100_e14877_d_n8, assign20100_e14877_d_n9, assign20100_e14877_d_n10, assign20100_e14877_d_n13,) = {
    if (locals.var_guard405 != 0.0) {
        let assign20100_e14872: f64 = (locals.var_vdserev / 2.0);
        let assign20100_e14873: f64 = (2.0 * assign20100_e14872);
        let assign20100_e14875: f64 = (assign20100_e14873 / p.p262);
        (assign20100_e14875, ((2.0 * (locals.var_vdserev_dn0 / 2.0)) / p.p262), ((2.0 * (locals.var_vdserev_dn2 / 2.0)) / p.p262), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign20100_e14877;
        locals.var_tmf1_dn0 = assign20100_e14877_d_n0;
        locals.var_tmf1_dn2 = assign20100_e14877_d_n2;
        locals.var_tmf1_dn4 = assign20100_e14877_d_n4;
        locals.var_tmf1_dn5 = assign20100_e14877_d_n5;
        locals.var_tmf1_dn6 = assign20100_e14877_d_n6;
        locals.var_tmf1_dn7 = assign20100_e14877_d_n7;
        locals.var_tmf1_dn8 = assign20100_e14877_d_n8;
        locals.var_tmf1_dn9 = assign20100_e14877_d_n9;
        locals.var_tmf1_dn10 = assign20100_e14877_d_n10;
        locals.var_tmf1_dn13 = assign20100_e14877_d_n13;

        let (assign20110_e14917, assign20110_e14917_d_n0, assign20110_e14917_d_n2, assign20110_e14917_d_n4, assign20110_e14917_d_n5, assign20110_e14917_d_n6, assign20110_e14917_d_n7, assign20110_e14917_d_n8, assign20110_e14917_d_n9, assign20110_e14917_d_n10, assign20110_e14917_d_n13,) = {
    if (locals.var_guard405 != 0.0) {
        let assign20110_e14883: f64 = (1.0 / 2.0);
        let assign20110_e14887: f64 = (1.0 / 6.0);
        let assign20110_e14891: f64 = (1.0 / 24.0);
        let assign20110_e14895: f64 = (1.0 / 120.0);
        let assign20110_e14899: f64 = (1.0 / 720.0);
        let assign20110_e14903: f64 = (1.0 / 5040.0);
        let assign20110_e14904: f64 = (locals.var_tmf1 * assign20110_e14903);
        let assign20110_e14905: f64 = (assign20110_e14899 + assign20110_e14904);
        let assign20110_e14906: f64 = (locals.var_tmf1 * assign20110_e14905);
        let assign20110_e14907: f64 = (assign20110_e14895 + assign20110_e14906);
        let assign20110_e14908: f64 = (locals.var_tmf1 * assign20110_e14907);
        let assign20110_e14909: f64 = (assign20110_e14891 + assign20110_e14908);
        let assign20110_e14910: f64 = (locals.var_tmf1 * assign20110_e14909);
        let assign20110_e14911: f64 = (assign20110_e14887 + assign20110_e14910);
        let assign20110_e14912: f64 = (locals.var_tmf1 * assign20110_e14911);
        let assign20110_e14913: f64 = (assign20110_e14883 + assign20110_e14912);
        let assign20110_e14914: f64 = (locals.var_tmf1 * assign20110_e14913);
        let assign20110_e14915: f64 = (1.0 + assign20110_e14914);
        (assign20110_e14915, ((locals.var_tmf1_dn0 * assign20110_e14913) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20110_e14911) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20110_e14909) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20110_e14907) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20110_e14905) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign20110_e14903))))))))))), ((locals.var_tmf1_dn2 * assign20110_e14913) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20110_e14911) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20110_e14909) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20110_e14907) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20110_e14905) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign20110_e14903))))))))))), ((locals.var_tmf1_dn4 * assign20110_e14913) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20110_e14911) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20110_e14909) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20110_e14907) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20110_e14905) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign20110_e14903))))))))))), ((locals.var_tmf1_dn5 * assign20110_e14913) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20110_e14911) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20110_e14909) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20110_e14907) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20110_e14905) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign20110_e14903))))))))))), ((locals.var_tmf1_dn6 * assign20110_e14913) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20110_e14911) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20110_e14909) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20110_e14907) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20110_e14905) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign20110_e14903))))))))))), ((locals.var_tmf1_dn7 * assign20110_e14913) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20110_e14911) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20110_e14909) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20110_e14907) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20110_e14905) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign20110_e14903))))))))))), ((locals.var_tmf1_dn8 * assign20110_e14913) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20110_e14911) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20110_e14909) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20110_e14907) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20110_e14905) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign20110_e14903))))))))))), ((locals.var_tmf1_dn9 * assign20110_e14913) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20110_e14911) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20110_e14909) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20110_e14907) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20110_e14905) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign20110_e14903))))))))))), ((locals.var_tmf1_dn10 * assign20110_e14913) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20110_e14911) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20110_e14909) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20110_e14907) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20110_e14905) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign20110_e14903))))))))))), ((locals.var_tmf1_dn13 * assign20110_e14913) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign20110_e14911) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign20110_e14909) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign20110_e14907) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign20110_e14905) + (locals.var_tmf1 * (locals.var_tmf1_dn13 * assign20110_e14903))))))))))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign20110_e14917;
        locals.var_tmf2_dn0 = assign20110_e14917_d_n0;
        locals.var_tmf2_dn2 = assign20110_e14917_d_n2;
        locals.var_tmf2_dn4 = assign20110_e14917_d_n4;
        locals.var_tmf2_dn5 = assign20110_e14917_d_n5;
        locals.var_tmf2_dn6 = assign20110_e14917_d_n6;
        locals.var_tmf2_dn7 = assign20110_e14917_d_n7;
        locals.var_tmf2_dn8 = assign20110_e14917_d_n8;
        locals.var_tmf2_dn9 = assign20110_e14917_d_n9;
        locals.var_tmf2_dn10 = assign20110_e14917_d_n10;
        locals.var_tmf2_dn13 = assign20110_e14917_d_n13;

        let (assign20120_e14953, assign20120_e14953_d_n0, assign20120_e14953_d_n2, assign20120_e14953_d_n4, assign20120_e14953_d_n5, assign20120_e14953_d_n6, assign20120_e14953_d_n7, assign20120_e14953_d_n8, assign20120_e14953_d_n9, assign20120_e14953_d_n10, assign20120_e14953_d_n13,) = {
    if (locals.var_guard405 != 0.0) {
        let assign20120_e14921: f64 = (1.0 / 2.0);
        let assign20120_e14925: f64 = (1.0 / 3.0);
        let assign20120_e14929: f64 = (1.0 / 8.0);
        let assign20120_e14933: f64 = (1.0 / 30.0);
        let assign20120_e14937: f64 = (1.0 / 144.0);
        let assign20120_e14941: f64 = (1.0 / 840.0);
        let assign20120_e14942: f64 = (locals.var_tmf1 * assign20120_e14941);
        let assign20120_e14943: f64 = (assign20120_e14937 + assign20120_e14942);
        let assign20120_e14944: f64 = (locals.var_tmf1 * assign20120_e14943);
        let assign20120_e14945: f64 = (assign20120_e14933 + assign20120_e14944);
        let assign20120_e14946: f64 = (locals.var_tmf1 * assign20120_e14945);
        let assign20120_e14947: f64 = (assign20120_e14929 + assign20120_e14946);
        let assign20120_e14948: f64 = (locals.var_tmf1 * assign20120_e14947);
        let assign20120_e14949: f64 = (assign20120_e14925 + assign20120_e14948);
        let assign20120_e14950: f64 = (locals.var_tmf1 * assign20120_e14949);
        let assign20120_e14951: f64 = (assign20120_e14921 + assign20120_e14950);
        (assign20120_e14951, ((locals.var_tmf1_dn0 * assign20120_e14949) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20120_e14947) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20120_e14945) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20120_e14943) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign20120_e14941))))))))), ((locals.var_tmf1_dn2 * assign20120_e14949) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20120_e14947) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20120_e14945) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20120_e14943) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign20120_e14941))))))))), ((locals.var_tmf1_dn4 * assign20120_e14949) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20120_e14947) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20120_e14945) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20120_e14943) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign20120_e14941))))))))), ((locals.var_tmf1_dn5 * assign20120_e14949) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20120_e14947) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20120_e14945) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20120_e14943) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign20120_e14941))))))))), ((locals.var_tmf1_dn6 * assign20120_e14949) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20120_e14947) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20120_e14945) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20120_e14943) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign20120_e14941))))))))), ((locals.var_tmf1_dn7 * assign20120_e14949) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20120_e14947) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20120_e14945) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20120_e14943) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign20120_e14941))))))))), ((locals.var_tmf1_dn8 * assign20120_e14949) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20120_e14947) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20120_e14945) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20120_e14943) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign20120_e14941))))))))), ((locals.var_tmf1_dn9 * assign20120_e14949) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20120_e14947) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20120_e14945) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20120_e14943) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign20120_e14941))))))))), ((locals.var_tmf1_dn10 * assign20120_e14949) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20120_e14947) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20120_e14945) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20120_e14943) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign20120_e14941))))))))), ((locals.var_tmf1_dn13 * assign20120_e14949) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign20120_e14947) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign20120_e14945) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign20120_e14943) + (locals.var_tmf1 * (locals.var_tmf1_dn13 * assign20120_e14941))))))))),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn13,)
    }
};
        locals.var_tmf3 = assign20120_e14953;
        locals.var_tmf3_dn0 = assign20120_e14953_d_n0;
        locals.var_tmf3_dn2 = assign20120_e14953_d_n2;
        locals.var_tmf3_dn4 = assign20120_e14953_d_n4;
        locals.var_tmf3_dn5 = assign20120_e14953_d_n5;
        locals.var_tmf3_dn6 = assign20120_e14953_d_n6;
        locals.var_tmf3_dn7 = assign20120_e14953_d_n7;
        locals.var_tmf3_dn8 = assign20120_e14953_d_n8;
        locals.var_tmf3_dn9 = assign20120_e14953_d_n9;
        locals.var_tmf3_dn10 = assign20120_e14953_d_n10;
        locals.var_tmf3_dn13 = assign20120_e14953_d_n13;

        let (assign20130_e14959, assign20130_e14959_d_n0, assign20130_e14959_d_n2, assign20130_e14959_d_n4, assign20130_e14959_d_n5, assign20130_e14959_d_n6, assign20130_e14959_d_n7, assign20130_e14959_d_n8, assign20130_e14959_d_n9, assign20130_e14959_d_n10, assign20130_e14959_d_n13,) = {
    if (locals.var_guard405 != 0.0) {
        let assign20130_e14957: f64 = (p.p262 / locals.var_tmf2);
        (assign20130_e14957, (-((p.p262 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn13) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_vzadd, locals.var_vzadd_dn0, locals.var_vzadd_dn2, locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, locals.var_vzadd_dn7, locals.var_vzadd_dn8, locals.var_vzadd_dn9, locals.var_vzadd_dn10, locals.var_vzadd_dn13,)
    }
};
        locals.var_vzadd = assign20130_e14959;
        locals.var_vzadd_dn0 = assign20130_e14959_d_n0;
        locals.var_vzadd_dn2 = assign20130_e14959_d_n2;
        locals.var_vzadd_dn4 = assign20130_e14959_d_n4;
        locals.var_vzadd_dn5 = assign20130_e14959_d_n5;
        locals.var_vzadd_dn6 = assign20130_e14959_d_n6;
        locals.var_vzadd_dn7 = assign20130_e14959_d_n7;
        locals.var_vzadd_dn8 = assign20130_e14959_d_n8;
        locals.var_vzadd_dn9 = assign20130_e14959_d_n9;
        locals.var_vzadd_dn10 = assign20130_e14959_d_n10;
        locals.var_vzadd_dn13 = assign20130_e14959_d_n13;

        let (assign20140_e14970, assign20140_e14970_d_n0, assign20140_e14970_d_n2, assign20140_e14970_d_n4, assign20140_e14970_d_n5, assign20140_e14970_d_n6, assign20140_e14970_d_n7, assign20140_e14970_d_n8, assign20140_e14970_d_n9, assign20140_e14970_d_n10, assign20140_e14970_d_n13,) = {
    if (locals.var_guard405 != 0.0) {
        let assign20140_e14962: f64 = (-2.0);
        let assign20140_e14964: f64 = (assign20140_e14962 * locals.var_tmf3);
        let assign20140_e14967: f64 = (locals.var_tmf2 * locals.var_tmf2);
        let assign20140_e14968: f64 = (assign20140_e14964 / assign20140_e14967);
        (assign20140_e14968, ((((assign20140_e14962 * locals.var_tmf3_dn0) * assign20140_e14967) - (assign20140_e14964 * ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)))) / (assign20140_e14967 * assign20140_e14967)), ((((assign20140_e14962 * locals.var_tmf3_dn2) * assign20140_e14967) - (assign20140_e14964 * ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)))) / (assign20140_e14967 * assign20140_e14967)), ((((assign20140_e14962 * locals.var_tmf3_dn4) * assign20140_e14967) - (assign20140_e14964 * ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)))) / (assign20140_e14967 * assign20140_e14967)), ((((assign20140_e14962 * locals.var_tmf3_dn5) * assign20140_e14967) - (assign20140_e14964 * ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)))) / (assign20140_e14967 * assign20140_e14967)), ((((assign20140_e14962 * locals.var_tmf3_dn6) * assign20140_e14967) - (assign20140_e14964 * ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)))) / (assign20140_e14967 * assign20140_e14967)), ((((assign20140_e14962 * locals.var_tmf3_dn7) * assign20140_e14967) - (assign20140_e14964 * ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)))) / (assign20140_e14967 * assign20140_e14967)), ((((assign20140_e14962 * locals.var_tmf3_dn8) * assign20140_e14967) - (assign20140_e14964 * ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)))) / (assign20140_e14967 * assign20140_e14967)), ((((assign20140_e14962 * locals.var_tmf3_dn9) * assign20140_e14967) - (assign20140_e14964 * ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)))) / (assign20140_e14967 * assign20140_e14967)), ((((assign20140_e14962 * locals.var_tmf3_dn10) * assign20140_e14967) - (assign20140_e14964 * ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)))) / (assign20140_e14967 * assign20140_e14967)), ((((assign20140_e14962 * locals.var_tmf3_dn13) * assign20140_e14967) - (assign20140_e14964 * ((locals.var_tmf2_dn13 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn13)))) / (assign20140_e14967 * assign20140_e14967)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign20140_e14970;
        locals.var_t2_dn0 = assign20140_e14970_d_n0;
        locals.var_t2_dn2 = assign20140_e14970_d_n2;
        locals.var_t2_dn4 = assign20140_e14970_d_n4;
        locals.var_t2_dn5 = assign20140_e14970_d_n5;
        locals.var_t2_dn6 = assign20140_e14970_d_n6;
        locals.var_t2_dn7 = assign20140_e14970_d_n7;
        locals.var_t2_dn8 = assign20140_e14970_d_n8;
        locals.var_t2_dn9 = assign20140_e14970_d_n9;
        locals.var_t2_dn10 = assign20140_e14970_d_n10;
        locals.var_t2_dn13 = assign20140_e14970_d_n13;

        let assign20150_e14973: f64 = if locals.var_vzadd < 1e-12 { 1.0 } else { 0.0 };
        locals.var_guard407 = assign20150_e14973;

        let (assign20160_e14979, assign20160_e14979_d_n0, assign20160_e14979_d_n2, assign20160_e14979_d_n4, assign20160_e14979_d_n5, assign20160_e14979_d_n6, assign20160_e14979_d_n7, assign20160_e14979_d_n8, assign20160_e14979_d_n9, assign20160_e14979_d_n10, assign20160_e14979_d_n13,) = {
    if ((locals.var_guard405 != 0.0) && (locals.var_guard407 != 0.0)) {
        (1e-12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vzadd, locals.var_vzadd_dn0, locals.var_vzadd_dn2, locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, locals.var_vzadd_dn7, locals.var_vzadd_dn8, locals.var_vzadd_dn9, locals.var_vzadd_dn10, locals.var_vzadd_dn13,)
    }
};
        locals.var_vzadd = assign20160_e14979;
        locals.var_vzadd_dn0 = assign20160_e14979_d_n0;
        locals.var_vzadd_dn2 = assign20160_e14979_d_n2;
        locals.var_vzadd_dn4 = assign20160_e14979_d_n4;
        locals.var_vzadd_dn5 = assign20160_e14979_d_n5;
        locals.var_vzadd_dn6 = assign20160_e14979_d_n6;
        locals.var_vzadd_dn7 = assign20160_e14979_d_n7;
        locals.var_vzadd_dn8 = assign20160_e14979_d_n8;
        locals.var_vzadd_dn9 = assign20160_e14979_d_n9;
        locals.var_vzadd_dn10 = assign20160_e14979_d_n10;
        locals.var_vzadd_dn13 = assign20160_e14979_d_n13;

        let (assign20170_e14987, assign20170_e14987_d_n0, assign20170_e14987_d_n2, assign20170_e14987_d_n4, assign20170_e14987_d_n5, assign20170_e14987_d_n6, assign20170_e14987_d_n7, assign20170_e14987_d_n8, assign20170_e14987_d_n9, assign20170_e14987_d_n10, assign20170_e14987_d_n13,) = {
    if (locals.var_guard405 != 0.0) {
        let assign20170_e14984: f64 = (2.0 * locals.var_vzadd);
        let assign20170_e14985: f64 = (locals.var_vdserev + assign20170_e14984);
        (assign20170_e14985, (locals.var_vdserev_dn0 + (2.0 * locals.var_vzadd_dn0)), (locals.var_vdserev_dn2 + (2.0 * locals.var_vzadd_dn2)), (2.0 * locals.var_vzadd_dn4), (2.0 * locals.var_vzadd_dn5), (2.0 * locals.var_vzadd_dn6), (2.0 * locals.var_vzadd_dn7), (2.0 * locals.var_vzadd_dn8), (2.0 * locals.var_vzadd_dn9), (2.0 * locals.var_vzadd_dn10), (2.0 * locals.var_vzadd_dn13),)
    } else {
        (locals.var_vdserevz, locals.var_vdserevz_dn0, locals.var_vdserevz_dn2, locals.var_vdserevz_dn4, locals.var_vdserevz_dn5, locals.var_vdserevz_dn6, locals.var_vdserevz_dn7, locals.var_vdserevz_dn8, locals.var_vdserevz_dn9, locals.var_vdserevz_dn10, locals.var_vdserevz_dn13,)
    }
};
        locals.var_vdserevz = assign20170_e14987;
        locals.var_vdserevz_dn0 = assign20170_e14987_d_n0;
        locals.var_vdserevz_dn2 = assign20170_e14987_d_n2;
        locals.var_vdserevz_dn4 = assign20170_e14987_d_n4;
        locals.var_vdserevz_dn5 = assign20170_e14987_d_n5;
        locals.var_vdserevz_dn6 = assign20170_e14987_d_n6;
        locals.var_vdserevz_dn7 = assign20170_e14987_d_n7;
        locals.var_vdserevz_dn8 = assign20170_e14987_d_n8;
        locals.var_vdserevz_dn9 = assign20170_e14987_d_n9;
        locals.var_vdserevz_dn10 = assign20170_e14987_d_n10;
        locals.var_vdserevz_dn13 = assign20170_e14987_d_n13;

        let (assign20180_e14999, assign20180_e14999_d_n0, assign20180_e14999_d_n2, assign20180_e14999_d_n4, assign20180_e14999_d_n5, assign20180_e14999_d_n6, assign20180_e14999_d_n7, assign20180_e14999_d_n8, assign20180_e14999_d_n9, assign20180_e14999_d_n10, assign20180_e14999_d_n13,) = {
    if (locals.var_guard405 != 0.0) {
        let assign20180_e14992: f64 = (p.p333 * locals.var_vdserevz);
        let assign20180_e14993: f64 = (p.p335 - assign20180_e14992);
        let assign20180_e14996: f64 = (p.p332 * locals.var_vsubsrev);
        let assign20180_e14997: f64 = (assign20180_e14993 - assign20180_e14996);
        (assign20180_e14997, ((-(p.p333 * locals.var_vdserevz_dn0)) - (p.p332 * locals.var_vsubsrev_dn0)), ((-(p.p333 * locals.var_vdserevz_dn2)) - (p.p332 * locals.var_vsubsrev_dn2)), (-(p.p333 * locals.var_vdserevz_dn4)), (-(p.p333 * locals.var_vdserevz_dn5)), (-(p.p333 * locals.var_vdserevz_dn6)), (-(p.p333 * locals.var_vdserevz_dn7)), (-(p.p333 * locals.var_vdserevz_dn8)), (-(p.p333 * locals.var_vdserevz_dn9)), (-(p.p333 * locals.var_vdserevz_dn10)), (-(p.p333 * locals.var_vdserevz_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign20180_e14999;
        locals.var_t0_dn0 = assign20180_e14999_d_n0;
        locals.var_t0_dn2 = assign20180_e14999_d_n2;
        locals.var_t0_dn4 = assign20180_e14999_d_n4;
        locals.var_t0_dn5 = assign20180_e14999_d_n5;
        locals.var_t0_dn6 = assign20180_e14999_d_n6;
        locals.var_t0_dn7 = assign20180_e14999_d_n7;
        locals.var_t0_dn8 = assign20180_e14999_d_n8;
        locals.var_t0_dn9 = assign20180_e14999_d_n9;
        locals.var_t0_dn10 = assign20180_e14999_d_n10;
        locals.var_t0_dn13 = assign20180_e14999_d_n13;

        let (assign20190_e15012, assign20190_e15012_d_n0, assign20190_e15012_d_n2, assign20190_e15012_d_n4, assign20190_e15012_d_n5, assign20190_e15012_d_n6, assign20190_e15012_d_n7, assign20190_e15012_d_n8, assign20190_e15012_d_n9, assign20190_e15012_d_n10, assign20190_e15012_d_n13,) = {
    if (locals.var_guard405 != 0.0) {
        let assign20190_e15003: f64 = (locals.var_t0 * locals.var_t0);
        let assign20190_e15006: f64 = (4.0 * 10.0);
        let assign20190_e15008: f64 = (assign20190_e15006 * 10.0);
        let assign20190_e15009: f64 = (assign20190_e15003 + assign20190_e15008);
        let assign20190_e15010: f64 = (assign20190_e15009).sqrt();
        (assign20190_e15010, (((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)) / (2.0 * assign20190_e15010)), (((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)) / (2.0 * assign20190_e15010)), (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign20190_e15010)), (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign20190_e15010)), (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign20190_e15010)), (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign20190_e15010)), (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign20190_e15010)), (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign20190_e15010)), (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign20190_e15010)), (((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)) / (2.0 * assign20190_e15010)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign20190_e15012;
        locals.var_tmf2_dn0 = assign20190_e15012_d_n0;
        locals.var_tmf2_dn2 = assign20190_e15012_d_n2;
        locals.var_tmf2_dn4 = assign20190_e15012_d_n4;
        locals.var_tmf2_dn5 = assign20190_e15012_d_n5;
        locals.var_tmf2_dn6 = assign20190_e15012_d_n6;
        locals.var_tmf2_dn7 = assign20190_e15012_d_n7;
        locals.var_tmf2_dn8 = assign20190_e15012_d_n8;
        locals.var_tmf2_dn9 = assign20190_e15012_d_n9;
        locals.var_tmf2_dn10 = assign20190_e15012_d_n10;
        locals.var_tmf2_dn13 = assign20190_e15012_d_n13;

        let (assign20200_e15022, assign20200_e15022_d_n0, assign20200_e15022_d_n2, assign20200_e15022_d_n4, assign20200_e15022_d_n5, assign20200_e15022_d_n6, assign20200_e15022_d_n7, assign20200_e15022_d_n8, assign20200_e15022_d_n9, assign20200_e15022_d_n10, assign20200_e15022_d_n13,) = {
    if (locals.var_guard405 != 0.0) {
        let assign20200_e15018: f64 = (locals.var_t0 / locals.var_tmf2);
        let assign20200_e15019: f64 = (1.0 + assign20200_e15018);
        let assign20200_e15020: f64 = (0.5 * assign20200_e15019);
        (assign20200_e15020, (0.5 * (((locals.var_t0_dn0 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn2 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn4 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn5 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn6 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn7 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn8 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn9 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn10 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn13 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign20200_e15022;
        locals.var_t2_dn0 = assign20200_e15022_d_n0;
        locals.var_t2_dn2 = assign20200_e15022_d_n2;
        locals.var_t2_dn4 = assign20200_e15022_d_n4;
        locals.var_t2_dn5 = assign20200_e15022_d_n5;
        locals.var_t2_dn6 = assign20200_e15022_d_n6;
        locals.var_t2_dn7 = assign20200_e15022_d_n7;
        locals.var_t2_dn8 = assign20200_e15022_d_n8;
        locals.var_t2_dn9 = assign20200_e15022_d_n9;
        locals.var_t2_dn10 = assign20200_e15022_d_n10;
        locals.var_t2_dn13 = assign20200_e15022_d_n13;

    }

    pub(super) fn stamp_transient_block_46(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign20210_e15030, assign20210_e15030_d_n0, assign20210_e15030_d_n2, assign20210_e15030_d_n4, assign20210_e15030_d_n5, assign20210_e15030_d_n6, assign20210_e15030_d_n7, assign20210_e15030_d_n8, assign20210_e15030_d_n9, assign20210_e15030_d_n10, assign20210_e15030_d_n13,) = {
    if (locals.var_guard405 != 0.0) {
        let assign20210_e15027: f64 = (locals.var_t0 + locals.var_tmf2);
        let assign20210_e15028: f64 = (0.5 * assign20210_e15027);
        (assign20210_e15028, (0.5 * (locals.var_t0_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t0_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t0_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t0_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t0_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t0_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t0_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t0_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t0_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t0_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign20210_e15030;
        locals.var_t1_dn0 = assign20210_e15030_d_n0;
        locals.var_t1_dn2 = assign20210_e15030_d_n2;
        locals.var_t1_dn4 = assign20210_e15030_d_n4;
        locals.var_t1_dn5 = assign20210_e15030_d_n5;
        locals.var_t1_dn6 = assign20210_e15030_d_n6;
        locals.var_t1_dn7 = assign20210_e15030_d_n7;
        locals.var_t1_dn8 = assign20210_e15030_d_n8;
        locals.var_t1_dn9 = assign20210_e15030_d_n9;
        locals.var_t1_dn10 = assign20210_e15030_d_n10;
        locals.var_t1_dn13 = assign20210_e15030_d_n13;

        let assign20220_e15033: f64 = if locals.var_t1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard408 = assign20220_e15033;

        let (assign20230_e15039, assign20230_e15039_d_n0, assign20230_e15039_d_n2, assign20230_e15039_d_n4, assign20230_e15039_d_n5, assign20230_e15039_d_n6, assign20230_e15039_d_n7, assign20230_e15039_d_n8, assign20230_e15039_d_n9, assign20230_e15039_d_n10, assign20230_e15039_d_n13,) = {
    if ((locals.var_guard405 != 0.0) && (locals.var_guard408 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign20230_e15039;
        locals.var_t1_dn0 = assign20230_e15039_d_n0;
        locals.var_t1_dn2 = assign20230_e15039_d_n2;
        locals.var_t1_dn4 = assign20230_e15039_d_n4;
        locals.var_t1_dn5 = assign20230_e15039_d_n5;
        locals.var_t1_dn6 = assign20230_e15039_d_n6;
        locals.var_t1_dn7 = assign20230_e15039_d_n7;
        locals.var_t1_dn8 = assign20230_e15039_d_n8;
        locals.var_t1_dn9 = assign20230_e15039_d_n9;
        locals.var_t1_dn10 = assign20230_e15039_d_n10;
        locals.var_t1_dn13 = assign20230_e15039_d_n13;

        let (assign20240_e15045, assign20240_e15045_d_n0, assign20240_e15045_d_n2, assign20240_e15045_d_n4, assign20240_e15045_d_n5, assign20240_e15045_d_n6, assign20240_e15045_d_n7, assign20240_e15045_d_n8, assign20240_e15045_d_n9, assign20240_e15045_d_n10, assign20240_e15045_d_n13,) = {
    if ((locals.var_guard405 != 0.0) && (locals.var_guard408 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign20240_e15045;
        locals.var_t2_dn0 = assign20240_e15045_d_n0;
        locals.var_t2_dn2 = assign20240_e15045_d_n2;
        locals.var_t2_dn4 = assign20240_e15045_d_n4;
        locals.var_t2_dn5 = assign20240_e15045_d_n5;
        locals.var_t2_dn6 = assign20240_e15045_d_n6;
        locals.var_t2_dn7 = assign20240_e15045_d_n7;
        locals.var_t2_dn8 = assign20240_e15045_d_n8;
        locals.var_t2_dn9 = assign20240_e15045_d_n9;
        locals.var_t2_dn10 = assign20240_e15045_d_n10;
        locals.var_t2_dn13 = assign20240_e15045_d_n13;

        let (assign20250_e15053, assign20250_e15053_d_n0, assign20250_e15053_d_n2, assign20250_e15053_d_n4, assign20250_e15053_d_n5, assign20250_e15053_d_n6, assign20250_e15053_d_n7, assign20250_e15053_d_n8, assign20250_e15053_d_n9, assign20250_e15053_d_n10, assign20250_e15053_d_n13,) = {
    if (locals.var_guard405 != 0.0) {
        let assign20250_e15050: f64 = (10.0 * 2.220446049250313e-16);
        let assign20250_e15051: f64 = (locals.var_t1 + assign20250_e15050);
        (assign20250_e15051, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign20250_e15053;
        locals.var_t1_dn0 = assign20250_e15053_d_n0;
        locals.var_t1_dn2 = assign20250_e15053_d_n2;
        locals.var_t1_dn4 = assign20250_e15053_d_n4;
        locals.var_t1_dn5 = assign20250_e15053_d_n5;
        locals.var_t1_dn6 = assign20250_e15053_d_n6;
        locals.var_t1_dn7 = assign20250_e15053_d_n7;
        locals.var_t1_dn8 = assign20250_e15053_d_n8;
        locals.var_t1_dn9 = assign20250_e15053_d_n9;
        locals.var_t1_dn10 = assign20250_e15053_d_n10;
        locals.var_t1_dn13 = assign20250_e15053_d_n13;

        let (assign20260_e15063, assign20260_e15063_d_n0, assign20260_e15063_d_n2, assign20260_e15063_d_n4, assign20260_e15063_d_n5, assign20260_e15063_d_n6, assign20260_e15063_d_n7, assign20260_e15063_d_n8, assign20260_e15063_d_n9, assign20260_e15063_d_n10, assign20260_e15063_d_n13,) = {
    if (locals.var_guard405 != 0.0) {
        let assign20260_e15059: f64 = (locals.var_mks_nsubsub + locals.var_uc_nover);
        let assign20260_e15060: f64 = (locals.var_uc_nover * assign20260_e15059);
        let assign20260_e15061: f64 = (locals.var_mks_nsubsub / assign20260_e15060);
        (assign20260_e15061, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign20260_e15063;
        locals.var_t0_dn0 = assign20260_e15063_d_n0;
        locals.var_t0_dn2 = assign20260_e15063_d_n2;
        locals.var_t0_dn4 = assign20260_e15063_d_n4;
        locals.var_t0_dn5 = assign20260_e15063_d_n5;
        locals.var_t0_dn6 = assign20260_e15063_d_n6;
        locals.var_t0_dn7 = assign20260_e15063_d_n7;
        locals.var_t0_dn8 = assign20260_e15063_d_n8;
        locals.var_t0_dn9 = assign20260_e15063_d_n9;
        locals.var_t0_dn10 = assign20260_e15063_d_n10;
        locals.var_t0_dn13 = assign20260_e15063_d_n13;

        let (assign20270_e15073, assign20270_e15073_d_n0, assign20270_e15073_d_n2, assign20270_e15073_d_n4, assign20270_e15073_d_n5, assign20270_e15073_d_n6, assign20270_e15073_d_n7, assign20270_e15073_d_n8, assign20270_e15073_d_n9, assign20270_e15073_d_n10, assign20270_e15073_d_n13,) = {
    if (locals.var_guard405 != 0.0) {
        let assign20270_e15067: f64 = (2.0 * 1.034943e-10);
        let assign20270_e15069: f64 = (assign20270_e15067 / 1.6021918e-19);
        let assign20270_e15071: f64 = (assign20270_e15069 * locals.var_t0);
        (assign20270_e15071, (assign20270_e15069 * locals.var_t0_dn0), (assign20270_e15069 * locals.var_t0_dn2), (assign20270_e15069 * locals.var_t0_dn4), (assign20270_e15069 * locals.var_t0_dn5), (assign20270_e15069 * locals.var_t0_dn6), (assign20270_e15069 * locals.var_t0_dn7), (assign20270_e15069 * locals.var_t0_dn8), (assign20270_e15069 * locals.var_t0_dn9), (assign20270_e15069 * locals.var_t0_dn10), (assign20270_e15069 * locals.var_t0_dn13),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign20270_e15073;
        locals.var_t4_dn0 = assign20270_e15073_d_n0;
        locals.var_t4_dn2 = assign20270_e15073_d_n2;
        locals.var_t4_dn4 = assign20270_e15073_d_n4;
        locals.var_t4_dn5 = assign20270_e15073_d_n5;
        locals.var_t4_dn6 = assign20270_e15073_d_n6;
        locals.var_t4_dn7 = assign20270_e15073_d_n7;
        locals.var_t4_dn8 = assign20270_e15073_d_n8;
        locals.var_t4_dn9 = assign20270_e15073_d_n9;
        locals.var_t4_dn10 = assign20270_e15073_d_n10;
        locals.var_t4_dn13 = assign20270_e15073_d_n13;

        let (assign20280_e15082, assign20280_e15082_d_n0, assign20280_e15082_d_n2, assign20280_e15082_d_n4, assign20280_e15082_d_n5, assign20280_e15082_d_n6, assign20280_e15082_d_n7, assign20280_e15082_d_n8, assign20280_e15082_d_n9, assign20280_e15082_d_n10, assign20280_e15082_d_n13,) = {
    if (locals.var_guard405 != 0.0) {
        let assign20280_e15077: f64 = (locals.var_t4 * locals.var_t1);
        let assign20280_e15078: f64 = (assign20280_e15077).sqrt();
        let assign20280_e15080: f64 = (assign20280_e15078 + 1e-25);
        (assign20280_e15080, (((locals.var_t4_dn0 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn0)) / (2.0 * assign20280_e15078)), (((locals.var_t4_dn2 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn2)) / (2.0 * assign20280_e15078)), (((locals.var_t4_dn4 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn4)) / (2.0 * assign20280_e15078)), (((locals.var_t4_dn5 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn5)) / (2.0 * assign20280_e15078)), (((locals.var_t4_dn6 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn6)) / (2.0 * assign20280_e15078)), (((locals.var_t4_dn7 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn7)) / (2.0 * assign20280_e15078)), (((locals.var_t4_dn8 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn8)) / (2.0 * assign20280_e15078)), (((locals.var_t4_dn9 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn9)) / (2.0 * assign20280_e15078)), (((locals.var_t4_dn10 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn10)) / (2.0 * assign20280_e15078)), (((locals.var_t4_dn13 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn13)) / (2.0 * assign20280_e15078)),)
    } else {
        (locals.var_wdep, locals.var_wdep_dn0, locals.var_wdep_dn2, locals.var_wdep_dn4, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, locals.var_wdep_dn9, locals.var_wdep_dn10, locals.var_wdep_dn13,)
    }
};
        locals.var_wdep = assign20280_e15082;
        locals.var_wdep_dn0 = assign20280_e15082_d_n0;
        locals.var_wdep_dn2 = assign20280_e15082_d_n2;
        locals.var_wdep_dn4 = assign20280_e15082_d_n4;
        locals.var_wdep_dn5 = assign20280_e15082_d_n5;
        locals.var_wdep_dn6 = assign20280_e15082_d_n6;
        locals.var_wdep_dn7 = assign20280_e15082_d_n7;
        locals.var_wdep_dn8 = assign20280_e15082_d_n8;
        locals.var_wdep_dn9 = assign20280_e15082_d_n9;
        locals.var_wdep_dn10 = assign20280_e15082_d_n10;
        locals.var_wdep_dn13 = assign20280_e15082_d_n13;

        let (assign20290_e15092, assign20290_e15092_d_n0, assign20290_e15092_d_n2, assign20290_e15092_d_n4, assign20290_e15092_d_n5, assign20290_e15092_d_n6, assign20290_e15092_d_n7, assign20290_e15092_d_n8, assign20290_e15092_d_n9, assign20290_e15092_d_n10, assign20290_e15092_d_n13,) = {
    if (locals.var_guard405 != 0.0) {
        let assign20290_e15086: f64 = (p.p334 - locals.var_wdep);
        let assign20290_e15089: f64 = (0.1 * p.p334);
        let assign20290_e15090: f64 = (assign20290_e15086 - assign20290_e15089);
        (assign20290_e15090, (-locals.var_wdep_dn0), (-locals.var_wdep_dn2), (-locals.var_wdep_dn4), (-locals.var_wdep_dn5), (-locals.var_wdep_dn6), (-locals.var_wdep_dn7), (-locals.var_wdep_dn8), (-locals.var_wdep_dn9), (-locals.var_wdep_dn10), (-locals.var_wdep_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign20290_e15092;
        locals.var_tmf1_dn0 = assign20290_e15092_d_n0;
        locals.var_tmf1_dn2 = assign20290_e15092_d_n2;
        locals.var_tmf1_dn4 = assign20290_e15092_d_n4;
        locals.var_tmf1_dn5 = assign20290_e15092_d_n5;
        locals.var_tmf1_dn6 = assign20290_e15092_d_n6;
        locals.var_tmf1_dn7 = assign20290_e15092_d_n7;
        locals.var_tmf1_dn8 = assign20290_e15092_d_n8;
        locals.var_tmf1_dn9 = assign20290_e15092_d_n9;
        locals.var_tmf1_dn10 = assign20290_e15092_d_n10;
        locals.var_tmf1_dn13 = assign20290_e15092_d_n13;

        let (assign20300_e15102, assign20300_e15102_d_n0, assign20300_e15102_d_n2, assign20300_e15102_d_n4, assign20300_e15102_d_n5, assign20300_e15102_d_n6, assign20300_e15102_d_n7, assign20300_e15102_d_n8, assign20300_e15102_d_n9, assign20300_e15102_d_n10, assign20300_e15102_d_n13,) = {
    if (locals.var_guard405 != 0.0) {
        let assign20300_e15096: f64 = (4.0 * p.p334);
        let assign20300_e15099: f64 = (0.1 * p.p334);
        let assign20300_e15100: f64 = (assign20300_e15096 * assign20300_e15099);
        (assign20300_e15100, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign20300_e15102;
        locals.var_tmf2_dn0 = assign20300_e15102_d_n0;
        locals.var_tmf2_dn2 = assign20300_e15102_d_n2;
        locals.var_tmf2_dn4 = assign20300_e15102_d_n4;
        locals.var_tmf2_dn5 = assign20300_e15102_d_n5;
        locals.var_tmf2_dn6 = assign20300_e15102_d_n6;
        locals.var_tmf2_dn7 = assign20300_e15102_d_n7;
        locals.var_tmf2_dn8 = assign20300_e15102_d_n8;
        locals.var_tmf2_dn9 = assign20300_e15102_d_n9;
        locals.var_tmf2_dn10 = assign20300_e15102_d_n10;
        locals.var_tmf2_dn13 = assign20300_e15102_d_n13;

        let (assign20310_e15112, assign20310_e15112_d_n0, assign20310_e15112_d_n2, assign20310_e15112_d_n4, assign20310_e15112_d_n5, assign20310_e15112_d_n6, assign20310_e15112_d_n7, assign20310_e15112_d_n8, assign20310_e15112_d_n9, assign20310_e15112_d_n10, assign20310_e15112_d_n13,) = {
    if (locals.var_guard405 != 0.0) {
        let (assign20310_e15110, assign20310_e15110_d_n0, assign20310_e15110_d_n2, assign20310_e15110_d_n4, assign20310_e15110_d_n5, assign20310_e15110_d_n6, assign20310_e15110_d_n7, assign20310_e15110_d_n8, assign20310_e15110_d_n9, assign20310_e15110_d_n10, assign20310_e15110_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign20310_e15109: f64 = (-locals.var_tmf2);
                (assign20310_e15109, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign20310_e15110, assign20310_e15110_d_n0, assign20310_e15110_d_n2, assign20310_e15110_d_n4, assign20310_e15110_d_n5, assign20310_e15110_d_n6, assign20310_e15110_d_n7, assign20310_e15110_d_n8, assign20310_e15110_d_n9, assign20310_e15110_d_n10, assign20310_e15110_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign20310_e15112;
        locals.var_tmf2_dn0 = assign20310_e15112_d_n0;
        locals.var_tmf2_dn2 = assign20310_e15112_d_n2;
        locals.var_tmf2_dn4 = assign20310_e15112_d_n4;
        locals.var_tmf2_dn5 = assign20310_e15112_d_n5;
        locals.var_tmf2_dn6 = assign20310_e15112_d_n6;
        locals.var_tmf2_dn7 = assign20310_e15112_d_n7;
        locals.var_tmf2_dn8 = assign20310_e15112_d_n8;
        locals.var_tmf2_dn9 = assign20310_e15112_d_n9;
        locals.var_tmf2_dn10 = assign20310_e15112_d_n10;
        locals.var_tmf2_dn13 = assign20310_e15112_d_n13;

        let (assign20320_e15121, assign20320_e15121_d_n0, assign20320_e15121_d_n2, assign20320_e15121_d_n4, assign20320_e15121_d_n5, assign20320_e15121_d_n6, assign20320_e15121_d_n7, assign20320_e15121_d_n8, assign20320_e15121_d_n9, assign20320_e15121_d_n10, assign20320_e15121_d_n13,) = {
    if (locals.var_guard405 != 0.0) {
        let assign20320_e15116: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign20320_e15118: f64 = (assign20320_e15116 + locals.var_tmf2);
        let assign20320_e15119: f64 = (assign20320_e15118).sqrt();
        (assign20320_e15119, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign20320_e15119)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign20320_e15119)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign20320_e15119)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign20320_e15119)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign20320_e15119)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign20320_e15119)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign20320_e15119)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign20320_e15119)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign20320_e15119)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign20320_e15119)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign20320_e15121;
        locals.var_tmf2_dn0 = assign20320_e15121_d_n0;
        locals.var_tmf2_dn2 = assign20320_e15121_d_n2;
        locals.var_tmf2_dn4 = assign20320_e15121_d_n4;
        locals.var_tmf2_dn5 = assign20320_e15121_d_n5;
        locals.var_tmf2_dn6 = assign20320_e15121_d_n6;
        locals.var_tmf2_dn7 = assign20320_e15121_d_n7;
        locals.var_tmf2_dn8 = assign20320_e15121_d_n8;
        locals.var_tmf2_dn9 = assign20320_e15121_d_n9;
        locals.var_tmf2_dn10 = assign20320_e15121_d_n10;
        locals.var_tmf2_dn13 = assign20320_e15121_d_n13;

        let (assign20330_e15131, assign20330_e15131_d_n0, assign20330_e15131_d_n2, assign20330_e15131_d_n4, assign20330_e15131_d_n5, assign20330_e15131_d_n6, assign20330_e15131_d_n7, assign20330_e15131_d_n8, assign20330_e15131_d_n9, assign20330_e15131_d_n10, assign20330_e15131_d_n13,) = {
    if (locals.var_guard405 != 0.0) {
        let assign20330_e15127: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign20330_e15128: f64 = (1.0 + assign20330_e15127);
        let assign20330_e15129: f64 = (0.5 * assign20330_e15128);
        (assign20330_e15129, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign20330_e15131;
        locals.var_t0_dn0 = assign20330_e15131_d_n0;
        locals.var_t0_dn2 = assign20330_e15131_d_n2;
        locals.var_t0_dn4 = assign20330_e15131_d_n4;
        locals.var_t0_dn5 = assign20330_e15131_d_n5;
        locals.var_t0_dn6 = assign20330_e15131_d_n6;
        locals.var_t0_dn7 = assign20330_e15131_d_n7;
        locals.var_t0_dn8 = assign20330_e15131_d_n8;
        locals.var_t0_dn9 = assign20330_e15131_d_n9;
        locals.var_t0_dn10 = assign20330_e15131_d_n10;
        locals.var_t0_dn13 = assign20330_e15131_d_n13;

        let (assign20340_e15141, assign20340_e15141_d_n0, assign20340_e15141_d_n2, assign20340_e15141_d_n4, assign20340_e15141_d_n5, assign20340_e15141_d_n6, assign20340_e15141_d_n7, assign20340_e15141_d_n8, assign20340_e15141_d_n9, assign20340_e15141_d_n10, assign20340_e15141_d_n13,) = {
    if (locals.var_guard405 != 0.0) {
        let assign20340_e15137: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign20340_e15138: f64 = (0.5 * assign20340_e15137);
        let assign20340_e15139: f64 = (p.p334 - assign20340_e15138);
        (assign20340_e15139, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_wdep, locals.var_wdep_dn0, locals.var_wdep_dn2, locals.var_wdep_dn4, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, locals.var_wdep_dn9, locals.var_wdep_dn10, locals.var_wdep_dn13,)
    }
};
        locals.var_wdep = assign20340_e15141;
        locals.var_wdep_dn0 = assign20340_e15141_d_n0;
        locals.var_wdep_dn2 = assign20340_e15141_d_n2;
        locals.var_wdep_dn4 = assign20340_e15141_d_n4;
        locals.var_wdep_dn5 = assign20340_e15141_d_n5;
        locals.var_wdep_dn6 = assign20340_e15141_d_n6;
        locals.var_wdep_dn7 = assign20340_e15141_d_n7;
        locals.var_wdep_dn8 = assign20340_e15141_d_n8;
        locals.var_wdep_dn9 = assign20340_e15141_d_n9;
        locals.var_wdep_dn10 = assign20340_e15141_d_n10;
        locals.var_wdep_dn13 = assign20340_e15141_d_n13;

        let (assign20350_e15146, assign20350_e15146_d_n0, assign20350_e15146_d_n2, assign20350_e15146_d_n4, assign20350_e15146_d_n5, assign20350_e15146_d_n6, assign20350_e15146_d_n7, assign20350_e15146_d_n8, assign20350_e15146_d_n9, assign20350_e15146_d_n10, assign20350_e15146_d_n13,) = {
    if (locals.var_guard405 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_wdep, locals.var_wdep_dn0, locals.var_wdep_dn2, locals.var_wdep_dn4, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, locals.var_wdep_dn9, locals.var_wdep_dn10, locals.var_wdep_dn13,)
    }
};
        locals.var_wdep = assign20350_e15146;
        locals.var_wdep_dn0 = assign20350_e15146_d_n0;
        locals.var_wdep_dn2 = assign20350_e15146_d_n2;
        locals.var_wdep_dn4 = assign20350_e15146_d_n4;
        locals.var_wdep_dn5 = assign20350_e15146_d_n5;
        locals.var_wdep_dn6 = assign20350_e15146_d_n6;
        locals.var_wdep_dn7 = assign20350_e15146_d_n7;
        locals.var_wdep_dn8 = assign20350_e15146_d_n8;
        locals.var_wdep_dn9 = assign20350_e15146_d_n9;
        locals.var_wdep_dn10 = assign20350_e15146_d_n10;
        locals.var_wdep_dn13 = assign20350_e15146_d_n13;

        let assign20360_e15153: f64 = if ((locals.var_flg_rsrd == 1.0) || (locals.var_flg_rsrd == 3.0)) { 1.0 } else { 0.0 };
        locals.var_guard409 = assign20360_e15153;

        let (assign20370_e15157, assign20370_e15157_d_n0, assign20370_e15157_d_n2,) = {
    if (locals.var_guard409 != 0.0) {
        (locals.var_vdsei, locals.var_vdsei_dn0, locals.var_vdsei_dn2,)
    } else {
        (locals.var_vdsegmt, locals.var_vdsegmt_dn0, locals.var_vdsegmt_dn2,)
    }
};
        locals.var_vdsegmt = assign20370_e15157;
        locals.var_vdsegmt_dn0 = assign20370_e15157_d_n0;
        locals.var_vdsegmt_dn2 = assign20370_e15157_d_n2;

        let (assign20380_e15161, assign20380_e15161_d_n2, assign20380_e15161_d_n6,) = {
    if (locals.var_guard409 != 0.0) {
        (locals.var_vgsei, locals.var_vgsei_dn2, locals.var_vgsei_dn6,)
    } else {
        (locals.var_vgsegmt, locals.var_vgsegmt_dn2, locals.var_vgsegmt_dn6,)
    }
};
        locals.var_vgsegmt = assign20380_e15161;
        locals.var_vgsegmt_dn2 = assign20380_e15161_d_n2;
        locals.var_vgsegmt_dn6 = assign20380_e15161_d_n6;

        let (assign20390_e15165, assign20390_e15165_d_n2, assign20390_e15165_d_n8,) = {
    if (locals.var_guard409 != 0.0) {
        (locals.var_vbsei, locals.var_vbsei_dn2, locals.var_vbsei_dn8,)
    } else {
        (locals.var_vbsegmt, locals.var_vbsegmt_dn2, locals.var_vbsegmt_dn8,)
    }
};
        locals.var_vbsegmt = assign20390_e15165;
        locals.var_vbsegmt_dn2 = assign20390_e15165_d_n2;
        locals.var_vbsegmt_dn8 = assign20390_e15165_d_n8;

        let assign20400_e15168: f64 = if locals.var_vdsegmt >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard410 = assign20400_e15168;

        let (assign20410_e15174,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard410 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_vdsemodenml,)
    }
};
        locals.var_vdsemodenml = assign20410_e15174;

        let (assign20420_e15180,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard410 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_vdsemodervs,)
    }
};
        locals.var_vdsemodervs = assign20420_e15180;

        let (assign20430_e15186, assign20430_e15186_d_n0, assign20430_e15186_d_n2,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard410 != 0.0)) {
        (locals.var_vdsegmt, locals.var_vdsegmt_dn0, locals.var_vdsegmt_dn2,)
    } else {
        (locals.var_vdserev, locals.var_vdserev_dn0, locals.var_vdserev_dn2,)
    }
};
        locals.var_vdserev = assign20430_e15186;
        locals.var_vdserev_dn0 = assign20430_e15186_d_n0;
        locals.var_vdserev_dn2 = assign20430_e15186_d_n2;

        let (assign20440_e15192, assign20440_e15192_d_n0, assign20440_e15192_d_n2, assign20440_e15192_d_n6,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard410 != 0.0)) {
        (locals.var_vgsegmt, 0.0, locals.var_vgsegmt_dn2, locals.var_vgsegmt_dn6,)
    } else {
        (locals.var_vgserev, locals.var_vgserev_dn0, locals.var_vgserev_dn2, locals.var_vgserev_dn6,)
    }
};
        locals.var_vgserev = assign20440_e15192;
        locals.var_vgserev_dn0 = assign20440_e15192_d_n0;
        locals.var_vgserev_dn2 = assign20440_e15192_d_n2;
        locals.var_vgserev_dn6 = assign20440_e15192_d_n6;

        let (assign20450_e15198, assign20450_e15198_d_n0, assign20450_e15198_d_n2, assign20450_e15198_d_n8,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard410 != 0.0)) {
        (locals.var_vbsegmt, 0.0, locals.var_vbsegmt_dn2, locals.var_vbsegmt_dn8,)
    } else {
        (locals.var_vbserev, locals.var_vbserev_dn0, locals.var_vbserev_dn2, locals.var_vbserev_dn8,)
    }
};
        locals.var_vbserev = assign20450_e15198;
        locals.var_vbserev_dn0 = assign20450_e15198_d_n0;
        locals.var_vbserev_dn2 = assign20450_e15198_d_n2;
        locals.var_vbserev_dn8 = assign20450_e15198_d_n8;

        let (assign20460_e15204, assign20460_e15204_d_n0, assign20460_e15204_d_n2,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard410 != 0.0)) {
        (locals.var_vsubs, 0.0, 0.0,)
    } else {
        (locals.var_vsubsrev, locals.var_vsubsrev_dn0, locals.var_vsubsrev_dn2,)
    }
};
        locals.var_vsubsrev = assign20460_e15204;
        locals.var_vsubsrev_dn0 = assign20460_e15204_d_n0;
        locals.var_vsubsrev_dn2 = assign20460_e15204_d_n2;

        let (assign20470_e15211,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard410 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_vdsemodenml,)
    }
};
        locals.var_vdsemodenml = assign20470_e15211;

        let (assign20480_e15218,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard410 == 0.0)) {
        (1.0,)
    } else {
        (locals.var_vdsemodervs,)
    }
};
        locals.var_vdsemodervs = assign20480_e15218;

        let (assign20490_e15226, assign20490_e15226_d_n0, assign20490_e15226_d_n2,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard410 == 0.0)) {
        let assign20490_e15224: f64 = (-locals.var_vdsegmt);
        (assign20490_e15224, (-locals.var_vdsegmt_dn0), (-locals.var_vdsegmt_dn2),)
    } else {
        (locals.var_vdserev, locals.var_vdserev_dn0, locals.var_vdserev_dn2,)
    }
};
        locals.var_vdserev = assign20490_e15226;
        locals.var_vdserev_dn0 = assign20490_e15226_d_n0;
        locals.var_vdserev_dn2 = assign20490_e15226_d_n2;

        let (assign20500_e15235, assign20500_e15235_d_n0, assign20500_e15235_d_n2, assign20500_e15235_d_n6,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard410 == 0.0)) {
        let assign20500_e15233: f64 = (locals.var_vgsegmt - locals.var_vdsegmt);
        (assign20500_e15233, (-locals.var_vdsegmt_dn0), (locals.var_vgsegmt_dn2 - locals.var_vdsegmt_dn2), locals.var_vgsegmt_dn6,)
    } else {
        (locals.var_vgserev, locals.var_vgserev_dn0, locals.var_vgserev_dn2, locals.var_vgserev_dn6,)
    }
};
        locals.var_vgserev = assign20500_e15235;
        locals.var_vgserev_dn0 = assign20500_e15235_d_n0;
        locals.var_vgserev_dn2 = assign20500_e15235_d_n2;
        locals.var_vgserev_dn6 = assign20500_e15235_d_n6;

        let (assign20510_e15244, assign20510_e15244_d_n0, assign20510_e15244_d_n2, assign20510_e15244_d_n8,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard410 == 0.0)) {
        let assign20510_e15242: f64 = (locals.var_vbsegmt - locals.var_vdsegmt);
        (assign20510_e15242, (-locals.var_vdsegmt_dn0), (locals.var_vbsegmt_dn2 - locals.var_vdsegmt_dn2), locals.var_vbsegmt_dn8,)
    } else {
        (locals.var_vbserev, locals.var_vbserev_dn0, locals.var_vbserev_dn2, locals.var_vbserev_dn8,)
    }
};
        locals.var_vbserev = assign20510_e15244;
        locals.var_vbserev_dn0 = assign20510_e15244_d_n0;
        locals.var_vbserev_dn2 = assign20510_e15244_d_n2;
        locals.var_vbserev_dn8 = assign20510_e15244_d_n8;

        let (assign20520_e15253, assign20520_e15253_d_n0, assign20520_e15253_d_n2,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard410 == 0.0)) {
        let assign20520_e15251: f64 = (locals.var_vsubs - locals.var_vdsegmt);
        (assign20520_e15251, (-locals.var_vdsegmt_dn0), (-locals.var_vdsegmt_dn2),)
    } else {
        (locals.var_vsubsrev, locals.var_vsubsrev_dn0, locals.var_vsubsrev_dn2,)
    }
};
        locals.var_vsubsrev = assign20520_e15253;
        locals.var_vsubsrev_dn0 = assign20520_e15253_d_n0;
        locals.var_vsubsrev_dn2 = assign20520_e15253_d_n2;

        let assign20530_e15272: f64 = if (((((locals.var_rdvde > 0.0) || (locals.var_rsvde > 0.0)) || (locals.var_uc_rdvg11 > 0.0)) || (locals.var_uc_rdvb > 0.0)) || (p.p54 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard411 = assign20530_e15272;

        let (assign20540_e15284, assign20540_e15284_d_n0, assign20540_e15284_d_n2, assign20540_e15284_d_n4, assign20540_e15284_d_n5, assign20540_e15284_d_n6, assign20540_e15284_d_n7, assign20540_e15284_d_n8, assign20540_e15284_d_n9, assign20540_e15284_d_n10, assign20540_e15284_d_n13,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) {
        let assign20540_e15279: f64 = (locals.var_vdserev / 2.0);
        let assign20540_e15280: f64 = (2.0 * assign20540_e15279);
        let assign20540_e15282: f64 = (assign20540_e15280 / p.p262);
        (assign20540_e15282, ((2.0 * (locals.var_vdserev_dn0 / 2.0)) / p.p262), ((2.0 * (locals.var_vdserev_dn2 / 2.0)) / p.p262), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign20540_e15284;
        locals.var_tmf1_dn0 = assign20540_e15284_d_n0;
        locals.var_tmf1_dn2 = assign20540_e15284_d_n2;
        locals.var_tmf1_dn4 = assign20540_e15284_d_n4;
        locals.var_tmf1_dn5 = assign20540_e15284_d_n5;
        locals.var_tmf1_dn6 = assign20540_e15284_d_n6;
        locals.var_tmf1_dn7 = assign20540_e15284_d_n7;
        locals.var_tmf1_dn8 = assign20540_e15284_d_n8;
        locals.var_tmf1_dn9 = assign20540_e15284_d_n9;
        locals.var_tmf1_dn10 = assign20540_e15284_d_n10;
        locals.var_tmf1_dn13 = assign20540_e15284_d_n13;

    }

    pub(super) fn stamp_transient_block_47(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign20550_e15326, assign20550_e15326_d_n0, assign20550_e15326_d_n2, assign20550_e15326_d_n4, assign20550_e15326_d_n5, assign20550_e15326_d_n6, assign20550_e15326_d_n7, assign20550_e15326_d_n8, assign20550_e15326_d_n9, assign20550_e15326_d_n10, assign20550_e15326_d_n13,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) {
        let assign20550_e15292: f64 = (1.0 / 2.0);
        let assign20550_e15296: f64 = (1.0 / 6.0);
        let assign20550_e15300: f64 = (1.0 / 24.0);
        let assign20550_e15304: f64 = (1.0 / 120.0);
        let assign20550_e15308: f64 = (1.0 / 720.0);
        let assign20550_e15312: f64 = (1.0 / 5040.0);
        let assign20550_e15313: f64 = (locals.var_tmf1 * assign20550_e15312);
        let assign20550_e15314: f64 = (assign20550_e15308 + assign20550_e15313);
        let assign20550_e15315: f64 = (locals.var_tmf1 * assign20550_e15314);
        let assign20550_e15316: f64 = (assign20550_e15304 + assign20550_e15315);
        let assign20550_e15317: f64 = (locals.var_tmf1 * assign20550_e15316);
        let assign20550_e15318: f64 = (assign20550_e15300 + assign20550_e15317);
        let assign20550_e15319: f64 = (locals.var_tmf1 * assign20550_e15318);
        let assign20550_e15320: f64 = (assign20550_e15296 + assign20550_e15319);
        let assign20550_e15321: f64 = (locals.var_tmf1 * assign20550_e15320);
        let assign20550_e15322: f64 = (assign20550_e15292 + assign20550_e15321);
        let assign20550_e15323: f64 = (locals.var_tmf1 * assign20550_e15322);
        let assign20550_e15324: f64 = (1.0 + assign20550_e15323);
        (assign20550_e15324, ((locals.var_tmf1_dn0 * assign20550_e15322) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20550_e15320) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20550_e15318) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20550_e15316) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20550_e15314) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign20550_e15312))))))))))), ((locals.var_tmf1_dn2 * assign20550_e15322) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20550_e15320) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20550_e15318) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20550_e15316) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20550_e15314) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign20550_e15312))))))))))), ((locals.var_tmf1_dn4 * assign20550_e15322) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20550_e15320) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20550_e15318) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20550_e15316) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20550_e15314) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign20550_e15312))))))))))), ((locals.var_tmf1_dn5 * assign20550_e15322) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20550_e15320) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20550_e15318) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20550_e15316) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20550_e15314) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign20550_e15312))))))))))), ((locals.var_tmf1_dn6 * assign20550_e15322) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20550_e15320) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20550_e15318) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20550_e15316) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20550_e15314) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign20550_e15312))))))))))), ((locals.var_tmf1_dn7 * assign20550_e15322) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20550_e15320) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20550_e15318) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20550_e15316) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20550_e15314) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign20550_e15312))))))))))), ((locals.var_tmf1_dn8 * assign20550_e15322) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20550_e15320) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20550_e15318) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20550_e15316) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20550_e15314) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign20550_e15312))))))))))), ((locals.var_tmf1_dn9 * assign20550_e15322) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20550_e15320) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20550_e15318) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20550_e15316) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20550_e15314) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign20550_e15312))))))))))), ((locals.var_tmf1_dn10 * assign20550_e15322) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20550_e15320) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20550_e15318) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20550_e15316) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20550_e15314) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign20550_e15312))))))))))), ((locals.var_tmf1_dn13 * assign20550_e15322) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign20550_e15320) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign20550_e15318) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign20550_e15316) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign20550_e15314) + (locals.var_tmf1 * (locals.var_tmf1_dn13 * assign20550_e15312))))))))))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign20550_e15326;
        locals.var_tmf2_dn0 = assign20550_e15326_d_n0;
        locals.var_tmf2_dn2 = assign20550_e15326_d_n2;
        locals.var_tmf2_dn4 = assign20550_e15326_d_n4;
        locals.var_tmf2_dn5 = assign20550_e15326_d_n5;
        locals.var_tmf2_dn6 = assign20550_e15326_d_n6;
        locals.var_tmf2_dn7 = assign20550_e15326_d_n7;
        locals.var_tmf2_dn8 = assign20550_e15326_d_n8;
        locals.var_tmf2_dn9 = assign20550_e15326_d_n9;
        locals.var_tmf2_dn10 = assign20550_e15326_d_n10;
        locals.var_tmf2_dn13 = assign20550_e15326_d_n13;

        let (assign20560_e15364, assign20560_e15364_d_n0, assign20560_e15364_d_n2, assign20560_e15364_d_n4, assign20560_e15364_d_n5, assign20560_e15364_d_n6, assign20560_e15364_d_n7, assign20560_e15364_d_n8, assign20560_e15364_d_n9, assign20560_e15364_d_n10, assign20560_e15364_d_n13,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) {
        let assign20560_e15332: f64 = (1.0 / 2.0);
        let assign20560_e15336: f64 = (1.0 / 3.0);
        let assign20560_e15340: f64 = (1.0 / 8.0);
        let assign20560_e15344: f64 = (1.0 / 30.0);
        let assign20560_e15348: f64 = (1.0 / 144.0);
        let assign20560_e15352: f64 = (1.0 / 840.0);
        let assign20560_e15353: f64 = (locals.var_tmf1 * assign20560_e15352);
        let assign20560_e15354: f64 = (assign20560_e15348 + assign20560_e15353);
        let assign20560_e15355: f64 = (locals.var_tmf1 * assign20560_e15354);
        let assign20560_e15356: f64 = (assign20560_e15344 + assign20560_e15355);
        let assign20560_e15357: f64 = (locals.var_tmf1 * assign20560_e15356);
        let assign20560_e15358: f64 = (assign20560_e15340 + assign20560_e15357);
        let assign20560_e15359: f64 = (locals.var_tmf1 * assign20560_e15358);
        let assign20560_e15360: f64 = (assign20560_e15336 + assign20560_e15359);
        let assign20560_e15361: f64 = (locals.var_tmf1 * assign20560_e15360);
        let assign20560_e15362: f64 = (assign20560_e15332 + assign20560_e15361);
        (assign20560_e15362, ((locals.var_tmf1_dn0 * assign20560_e15360) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20560_e15358) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20560_e15356) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20560_e15354) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign20560_e15352))))))))), ((locals.var_tmf1_dn2 * assign20560_e15360) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20560_e15358) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20560_e15356) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20560_e15354) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign20560_e15352))))))))), ((locals.var_tmf1_dn4 * assign20560_e15360) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20560_e15358) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20560_e15356) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20560_e15354) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign20560_e15352))))))))), ((locals.var_tmf1_dn5 * assign20560_e15360) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20560_e15358) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20560_e15356) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20560_e15354) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign20560_e15352))))))))), ((locals.var_tmf1_dn6 * assign20560_e15360) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20560_e15358) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20560_e15356) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20560_e15354) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign20560_e15352))))))))), ((locals.var_tmf1_dn7 * assign20560_e15360) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20560_e15358) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20560_e15356) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20560_e15354) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign20560_e15352))))))))), ((locals.var_tmf1_dn8 * assign20560_e15360) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20560_e15358) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20560_e15356) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20560_e15354) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign20560_e15352))))))))), ((locals.var_tmf1_dn9 * assign20560_e15360) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20560_e15358) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20560_e15356) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20560_e15354) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign20560_e15352))))))))), ((locals.var_tmf1_dn10 * assign20560_e15360) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20560_e15358) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20560_e15356) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20560_e15354) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign20560_e15352))))))))), ((locals.var_tmf1_dn13 * assign20560_e15360) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign20560_e15358) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign20560_e15356) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign20560_e15354) + (locals.var_tmf1 * (locals.var_tmf1_dn13 * assign20560_e15352))))))))),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn13,)
    }
};
        locals.var_tmf3 = assign20560_e15364;
        locals.var_tmf3_dn0 = assign20560_e15364_d_n0;
        locals.var_tmf3_dn2 = assign20560_e15364_d_n2;
        locals.var_tmf3_dn4 = assign20560_e15364_d_n4;
        locals.var_tmf3_dn5 = assign20560_e15364_d_n5;
        locals.var_tmf3_dn6 = assign20560_e15364_d_n6;
        locals.var_tmf3_dn7 = assign20560_e15364_d_n7;
        locals.var_tmf3_dn8 = assign20560_e15364_d_n8;
        locals.var_tmf3_dn9 = assign20560_e15364_d_n9;
        locals.var_tmf3_dn10 = assign20560_e15364_d_n10;
        locals.var_tmf3_dn13 = assign20560_e15364_d_n13;

        let (assign20570_e15372, assign20570_e15372_d_n0, assign20570_e15372_d_n2, assign20570_e15372_d_n4, assign20570_e15372_d_n5, assign20570_e15372_d_n6, assign20570_e15372_d_n7, assign20570_e15372_d_n8, assign20570_e15372_d_n9, assign20570_e15372_d_n10, assign20570_e15372_d_n13,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) {
        let assign20570_e15370: f64 = (p.p262 / locals.var_tmf2);
        (assign20570_e15370, (-((p.p262 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn13) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_vzadd, locals.var_vzadd_dn0, locals.var_vzadd_dn2, locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, locals.var_vzadd_dn7, locals.var_vzadd_dn8, locals.var_vzadd_dn9, locals.var_vzadd_dn10, locals.var_vzadd_dn13,)
    }
};
        locals.var_vzadd = assign20570_e15372;
        locals.var_vzadd_dn0 = assign20570_e15372_d_n0;
        locals.var_vzadd_dn2 = assign20570_e15372_d_n2;
        locals.var_vzadd_dn4 = assign20570_e15372_d_n4;
        locals.var_vzadd_dn5 = assign20570_e15372_d_n5;
        locals.var_vzadd_dn6 = assign20570_e15372_d_n6;
        locals.var_vzadd_dn7 = assign20570_e15372_d_n7;
        locals.var_vzadd_dn8 = assign20570_e15372_d_n8;
        locals.var_vzadd_dn9 = assign20570_e15372_d_n9;
        locals.var_vzadd_dn10 = assign20570_e15372_d_n10;
        locals.var_vzadd_dn13 = assign20570_e15372_d_n13;

        let (assign20580_e15385, assign20580_e15385_d_n0, assign20580_e15385_d_n2, assign20580_e15385_d_n4, assign20580_e15385_d_n5, assign20580_e15385_d_n6, assign20580_e15385_d_n7, assign20580_e15385_d_n8, assign20580_e15385_d_n9, assign20580_e15385_d_n10, assign20580_e15385_d_n13,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) {
        let assign20580_e15377: f64 = (-2.0);
        let assign20580_e15379: f64 = (assign20580_e15377 * locals.var_tmf3);
        let assign20580_e15382: f64 = (locals.var_tmf2 * locals.var_tmf2);
        let assign20580_e15383: f64 = (assign20580_e15379 / assign20580_e15382);
        (assign20580_e15383, ((((assign20580_e15377 * locals.var_tmf3_dn0) * assign20580_e15382) - (assign20580_e15379 * ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)))) / (assign20580_e15382 * assign20580_e15382)), ((((assign20580_e15377 * locals.var_tmf3_dn2) * assign20580_e15382) - (assign20580_e15379 * ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)))) / (assign20580_e15382 * assign20580_e15382)), ((((assign20580_e15377 * locals.var_tmf3_dn4) * assign20580_e15382) - (assign20580_e15379 * ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)))) / (assign20580_e15382 * assign20580_e15382)), ((((assign20580_e15377 * locals.var_tmf3_dn5) * assign20580_e15382) - (assign20580_e15379 * ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)))) / (assign20580_e15382 * assign20580_e15382)), ((((assign20580_e15377 * locals.var_tmf3_dn6) * assign20580_e15382) - (assign20580_e15379 * ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)))) / (assign20580_e15382 * assign20580_e15382)), ((((assign20580_e15377 * locals.var_tmf3_dn7) * assign20580_e15382) - (assign20580_e15379 * ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)))) / (assign20580_e15382 * assign20580_e15382)), ((((assign20580_e15377 * locals.var_tmf3_dn8) * assign20580_e15382) - (assign20580_e15379 * ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)))) / (assign20580_e15382 * assign20580_e15382)), ((((assign20580_e15377 * locals.var_tmf3_dn9) * assign20580_e15382) - (assign20580_e15379 * ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)))) / (assign20580_e15382 * assign20580_e15382)), ((((assign20580_e15377 * locals.var_tmf3_dn10) * assign20580_e15382) - (assign20580_e15379 * ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)))) / (assign20580_e15382 * assign20580_e15382)), ((((assign20580_e15377 * locals.var_tmf3_dn13) * assign20580_e15382) - (assign20580_e15379 * ((locals.var_tmf2_dn13 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn13)))) / (assign20580_e15382 * assign20580_e15382)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign20580_e15385;
        locals.var_t2_dn0 = assign20580_e15385_d_n0;
        locals.var_t2_dn2 = assign20580_e15385_d_n2;
        locals.var_t2_dn4 = assign20580_e15385_d_n4;
        locals.var_t2_dn5 = assign20580_e15385_d_n5;
        locals.var_t2_dn6 = assign20580_e15385_d_n6;
        locals.var_t2_dn7 = assign20580_e15385_d_n7;
        locals.var_t2_dn8 = assign20580_e15385_d_n8;
        locals.var_t2_dn9 = assign20580_e15385_d_n9;
        locals.var_t2_dn10 = assign20580_e15385_d_n10;
        locals.var_t2_dn13 = assign20580_e15385_d_n13;

        let assign20590_e15388: f64 = if locals.var_vzadd < 1e-12 { 1.0 } else { 0.0 };
        locals.var_guard412 = assign20590_e15388;

        let (assign20600_e15396, assign20600_e15396_d_n0, assign20600_e15396_d_n2, assign20600_e15396_d_n4, assign20600_e15396_d_n5, assign20600_e15396_d_n6, assign20600_e15396_d_n7, assign20600_e15396_d_n8, assign20600_e15396_d_n9, assign20600_e15396_d_n10, assign20600_e15396_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard412 != 0.0)) {
        (1e-12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vzadd, locals.var_vzadd_dn0, locals.var_vzadd_dn2, locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, locals.var_vzadd_dn7, locals.var_vzadd_dn8, locals.var_vzadd_dn9, locals.var_vzadd_dn10, locals.var_vzadd_dn13,)
    }
};
        locals.var_vzadd = assign20600_e15396;
        locals.var_vzadd_dn0 = assign20600_e15396_d_n0;
        locals.var_vzadd_dn2 = assign20600_e15396_d_n2;
        locals.var_vzadd_dn4 = assign20600_e15396_d_n4;
        locals.var_vzadd_dn5 = assign20600_e15396_d_n5;
        locals.var_vzadd_dn6 = assign20600_e15396_d_n6;
        locals.var_vzadd_dn7 = assign20600_e15396_d_n7;
        locals.var_vzadd_dn8 = assign20600_e15396_d_n8;
        locals.var_vzadd_dn9 = assign20600_e15396_d_n9;
        locals.var_vzadd_dn10 = assign20600_e15396_d_n10;
        locals.var_vzadd_dn13 = assign20600_e15396_d_n13;

        let (assign20610_e15406, assign20610_e15406_d_n0, assign20610_e15406_d_n2, assign20610_e15406_d_n4, assign20610_e15406_d_n5, assign20610_e15406_d_n6, assign20610_e15406_d_n7, assign20610_e15406_d_n8, assign20610_e15406_d_n9, assign20610_e15406_d_n10, assign20610_e15406_d_n13,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) {
        let assign20610_e15403: f64 = (2.0 * locals.var_vzadd);
        let assign20610_e15404: f64 = (locals.var_vdserev + assign20610_e15403);
        (assign20610_e15404, (locals.var_vdserev_dn0 + (2.0 * locals.var_vzadd_dn0)), (locals.var_vdserev_dn2 + (2.0 * locals.var_vzadd_dn2)), (2.0 * locals.var_vzadd_dn4), (2.0 * locals.var_vzadd_dn5), (2.0 * locals.var_vzadd_dn6), (2.0 * locals.var_vzadd_dn7), (2.0 * locals.var_vzadd_dn8), (2.0 * locals.var_vzadd_dn9), (2.0 * locals.var_vzadd_dn10), (2.0 * locals.var_vzadd_dn13),)
    } else {
        (locals.var_vdserevz, locals.var_vdserevz_dn0, locals.var_vdserevz_dn2, locals.var_vdserevz_dn4, locals.var_vdserevz_dn5, locals.var_vdserevz_dn6, locals.var_vdserevz_dn7, locals.var_vdserevz_dn8, locals.var_vdserevz_dn9, locals.var_vdserevz_dn10, locals.var_vdserevz_dn13,)
    }
};
        locals.var_vdserevz = assign20610_e15406;
        locals.var_vdserevz_dn0 = assign20610_e15406_d_n0;
        locals.var_vdserevz_dn2 = assign20610_e15406_d_n2;
        locals.var_vdserevz_dn4 = assign20610_e15406_d_n4;
        locals.var_vdserevz_dn5 = assign20610_e15406_d_n5;
        locals.var_vdserevz_dn6 = assign20610_e15406_d_n6;
        locals.var_vdserevz_dn7 = assign20610_e15406_d_n7;
        locals.var_vdserevz_dn8 = assign20610_e15406_d_n8;
        locals.var_vdserevz_dn9 = assign20610_e15406_d_n9;
        locals.var_vdserevz_dn10 = assign20610_e15406_d_n10;
        locals.var_vdserevz_dn13 = assign20610_e15406_d_n13;

        let (assign20620_e15414, assign20620_e15414_d_n0, assign20620_e15414_d_n2, assign20620_e15414_d_n4, assign20620_e15414_d_n5, assign20620_e15414_d_n6, assign20620_e15414_d_n7, assign20620_e15414_d_n8, assign20620_e15414_d_n9, assign20620_e15414_d_n10, assign20620_e15414_d_n13,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) {
        let assign20620_e15412: f64 = (locals.var_vgserev + locals.var_vzadd);
        (assign20620_e15412, (locals.var_vgserev_dn0 + locals.var_vzadd_dn0), (locals.var_vgserev_dn2 + locals.var_vzadd_dn2), locals.var_vzadd_dn4, locals.var_vzadd_dn5, (locals.var_vgserev_dn6 + locals.var_vzadd_dn6), locals.var_vzadd_dn7, locals.var_vzadd_dn8, locals.var_vzadd_dn9, locals.var_vzadd_dn10, locals.var_vzadd_dn13,)
    } else {
        (locals.var_vgserevz, locals.var_vgserevz_dn0, locals.var_vgserevz_dn2, locals.var_vgserevz_dn4, locals.var_vgserevz_dn5, locals.var_vgserevz_dn6, locals.var_vgserevz_dn7, locals.var_vgserevz_dn8, locals.var_vgserevz_dn9, locals.var_vgserevz_dn10, locals.var_vgserevz_dn13,)
    }
};
        locals.var_vgserevz = assign20620_e15414;
        locals.var_vgserevz_dn0 = assign20620_e15414_d_n0;
        locals.var_vgserevz_dn2 = assign20620_e15414_d_n2;
        locals.var_vgserevz_dn4 = assign20620_e15414_d_n4;
        locals.var_vgserevz_dn5 = assign20620_e15414_d_n5;
        locals.var_vgserevz_dn6 = assign20620_e15414_d_n6;
        locals.var_vgserevz_dn7 = assign20620_e15414_d_n7;
        locals.var_vgserevz_dn8 = assign20620_e15414_d_n8;
        locals.var_vgserevz_dn9 = assign20620_e15414_d_n9;
        locals.var_vgserevz_dn10 = assign20620_e15414_d_n10;
        locals.var_vgserevz_dn13 = assign20620_e15414_d_n13;

        let (assign20630_e15422, assign20630_e15422_d_n0, assign20630_e15422_d_n2, assign20630_e15422_d_n4, assign20630_e15422_d_n5, assign20630_e15422_d_n6, assign20630_e15422_d_n7, assign20630_e15422_d_n8, assign20630_e15422_d_n9, assign20630_e15422_d_n10, assign20630_e15422_d_n13,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) {
        let assign20630_e15420: f64 = (locals.var_vbserev + locals.var_vzadd);
        (assign20630_e15420, (locals.var_vbserev_dn0 + locals.var_vzadd_dn0), (locals.var_vbserev_dn2 + locals.var_vzadd_dn2), locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, locals.var_vzadd_dn7, (locals.var_vbserev_dn8 + locals.var_vzadd_dn8), locals.var_vzadd_dn9, locals.var_vzadd_dn10, locals.var_vzadd_dn13,)
    } else {
        (locals.var_vbserevz, locals.var_vbserevz_dn0, locals.var_vbserevz_dn2, locals.var_vbserevz_dn4, locals.var_vbserevz_dn5, locals.var_vbserevz_dn6, locals.var_vbserevz_dn7, locals.var_vbserevz_dn8, locals.var_vbserevz_dn9, locals.var_vbserevz_dn10, locals.var_vbserevz_dn13,)
    }
};
        locals.var_vbserevz = assign20630_e15422;
        locals.var_vbserevz_dn0 = assign20630_e15422_d_n0;
        locals.var_vbserevz_dn2 = assign20630_e15422_d_n2;
        locals.var_vbserevz_dn4 = assign20630_e15422_d_n4;
        locals.var_vbserevz_dn5 = assign20630_e15422_d_n5;
        locals.var_vbserevz_dn6 = assign20630_e15422_d_n6;
        locals.var_vbserevz_dn7 = assign20630_e15422_d_n7;
        locals.var_vbserevz_dn8 = assign20630_e15422_d_n8;
        locals.var_vbserevz_dn9 = assign20630_e15422_d_n9;
        locals.var_vbserevz_dn10 = assign20630_e15422_d_n10;
        locals.var_vbserevz_dn13 = assign20630_e15422_d_n13;

        let assign20640_e15429: f64 = if ((p.p34 == 1.0) || (locals.var_vdsemodenml == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard413 = assign20640_e15429;

        let (assign20650_e15443, assign20650_e15443_d_n0, assign20650_e15443_d_n2, assign20650_e15443_d_n4, assign20650_e15443_d_n5, assign20650_e15443_d_n6, assign20650_e15443_d_n7, assign20650_e15443_d_n8, assign20650_e15443_d_n9, assign20650_e15443_d_n10, assign20650_e15443_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard413 != 0.0)) {
        let assign20650_e15437: f64 = (locals.var_vdsemodenml * locals.var_rde);
        let assign20650_e15440: f64 = (locals.var_vdsemodervs * locals.var_rse);
        let assign20650_e15441: f64 = (assign20650_e15437 + assign20650_e15440);
        (assign20650_e15441, ((locals.var_vdsemodenml * locals.var_rde_dn0) + (locals.var_vdsemodervs * locals.var_rse_dn0)), ((locals.var_vdsemodenml * locals.var_rde_dn2) + (locals.var_vdsemodervs * locals.var_rse_dn2)), ((locals.var_vdsemodenml * locals.var_rde_dn4) + (locals.var_vdsemodervs * locals.var_rse_dn4)), ((locals.var_vdsemodenml * locals.var_rde_dn5) + (locals.var_vdsemodervs * locals.var_rse_dn5)), ((locals.var_vdsemodenml * locals.var_rde_dn6) + (locals.var_vdsemodervs * locals.var_rse_dn6)), ((locals.var_vdsemodenml * locals.var_rde_dn7) + (locals.var_vdsemodervs * locals.var_rse_dn7)), ((locals.var_vdsemodenml * locals.var_rde_dn8) + (locals.var_vdsemodervs * locals.var_rse_dn8)), ((locals.var_vdsemodenml * locals.var_rde_dn9) + (locals.var_vdsemodervs * locals.var_rse_dn9)), ((locals.var_vdsemodenml * locals.var_rde_dn10) + (locals.var_vdsemodervs * locals.var_rse_dn10)), ((locals.var_vdsemodenml * locals.var_rde_dn13) + (locals.var_vdsemodervs * locals.var_rse_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign20650_e15443;
        locals.var_t1_dn0 = assign20650_e15443_d_n0;
        locals.var_t1_dn2 = assign20650_e15443_d_n2;
        locals.var_t1_dn4 = assign20650_e15443_d_n4;
        locals.var_t1_dn5 = assign20650_e15443_d_n5;
        locals.var_t1_dn6 = assign20650_e15443_d_n6;
        locals.var_t1_dn7 = assign20650_e15443_d_n7;
        locals.var_t1_dn8 = assign20650_e15443_d_n8;
        locals.var_t1_dn9 = assign20650_e15443_d_n9;
        locals.var_t1_dn10 = assign20650_e15443_d_n10;
        locals.var_t1_dn13 = assign20650_e15443_d_n13;

        let (assign20660_e15457, assign20660_e15457_d_n0, assign20660_e15457_d_n2, assign20660_e15457_d_n4, assign20660_e15457_d_n5, assign20660_e15457_d_n6, assign20660_e15457_d_n7, assign20660_e15457_d_n8, assign20660_e15457_d_n9, assign20660_e15457_d_n10, assign20660_e15457_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard413 != 0.0)) {
        let assign20660_e15451: f64 = (locals.var_vdsemodenml * locals.var_rdvde);
        let assign20660_e15454: f64 = (locals.var_vdsemodervs * locals.var_rsvde);
        let assign20660_e15455: f64 = (assign20660_e15451 + assign20660_e15454);
        (assign20660_e15455, ((locals.var_vdsemodenml * locals.var_rdvde_dn0) + (locals.var_vdsemodervs * locals.var_rsvde_dn0)), ((locals.var_vdsemodenml * locals.var_rdvde_dn2) + (locals.var_vdsemodervs * locals.var_rsvde_dn2)), ((locals.var_vdsemodenml * locals.var_rdvde_dn4) + (locals.var_vdsemodervs * locals.var_rsvde_dn4)), ((locals.var_vdsemodenml * locals.var_rdvde_dn5) + (locals.var_vdsemodervs * locals.var_rsvde_dn5)), ((locals.var_vdsemodenml * locals.var_rdvde_dn6) + (locals.var_vdsemodervs * locals.var_rsvde_dn6)), ((locals.var_vdsemodenml * locals.var_rdvde_dn7) + (locals.var_vdsemodervs * locals.var_rsvde_dn7)), ((locals.var_vdsemodenml * locals.var_rdvde_dn8) + (locals.var_vdsemodervs * locals.var_rsvde_dn8)), ((locals.var_vdsemodenml * locals.var_rdvde_dn9) + (locals.var_vdsemodervs * locals.var_rsvde_dn9)), ((locals.var_vdsemodenml * locals.var_rdvde_dn10) + (locals.var_vdsemodervs * locals.var_rsvde_dn10)), ((locals.var_vdsemodenml * locals.var_rdvde_dn13) + (locals.var_vdsemodervs * locals.var_rsvde_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign20660_e15457;
        locals.var_t0_dn0 = assign20660_e15457_d_n0;
        locals.var_t0_dn2 = assign20660_e15457_d_n2;
        locals.var_t0_dn4 = assign20660_e15457_d_n4;
        locals.var_t0_dn5 = assign20660_e15457_d_n5;
        locals.var_t0_dn6 = assign20660_e15457_d_n6;
        locals.var_t0_dn7 = assign20660_e15457_d_n7;
        locals.var_t0_dn8 = assign20660_e15457_d_n8;
        locals.var_t0_dn9 = assign20660_e15457_d_n9;
        locals.var_t0_dn10 = assign20660_e15457_d_n10;
        locals.var_t0_dn13 = assign20660_e15457_d_n13;

        let (assign20670_e15469, assign20670_e15469_d_n0, assign20670_e15469_d_n2, assign20670_e15469_d_n4, assign20670_e15469_d_n5, assign20670_e15469_d_n6, assign20670_e15469_d_n7, assign20670_e15469_d_n8, assign20670_e15469_d_n9, assign20670_e15469_d_n10, assign20670_e15469_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard413 != 0.0)) {
        let assign20670_e15466: f64 = (locals.var_t0 * locals.var_vdserevz);
        let assign20670_e15467: f64 = (locals.var_t1 + assign20670_e15466);
        (assign20670_e15467, (locals.var_t1_dn0 + ((locals.var_t0_dn0 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn0))), (locals.var_t1_dn2 + ((locals.var_t0_dn2 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn2))), (locals.var_t1_dn4 + ((locals.var_t0_dn4 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn4))), (locals.var_t1_dn5 + ((locals.var_t0_dn5 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn5))), (locals.var_t1_dn6 + ((locals.var_t0_dn6 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn6))), (locals.var_t1_dn7 + ((locals.var_t0_dn7 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn7))), (locals.var_t1_dn8 + ((locals.var_t0_dn8 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn8))), (locals.var_t1_dn9 + ((locals.var_t0_dn9 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn9))), (locals.var_t1_dn10 + ((locals.var_t0_dn10 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn10))), (locals.var_t1_dn13 + ((locals.var_t0_dn13 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn13))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign20670_e15469;
        locals.var_t4_dn0 = assign20670_e15469_d_n0;
        locals.var_t4_dn2 = assign20670_e15469_d_n2;
        locals.var_t4_dn4 = assign20670_e15469_d_n4;
        locals.var_t4_dn5 = assign20670_e15469_d_n5;
        locals.var_t4_dn6 = assign20670_e15469_d_n6;
        locals.var_t4_dn7 = assign20670_e15469_d_n7;
        locals.var_t4_dn8 = assign20670_e15469_d_n8;
        locals.var_t4_dn9 = assign20670_e15469_d_n9;
        locals.var_t4_dn10 = assign20670_e15469_d_n10;
        locals.var_t4_dn13 = assign20670_e15469_d_n13;

        let (assign20680_e15490, assign20680_e15490_d_n0, assign20680_e15490_d_n2, assign20680_e15490_d_n4, assign20680_e15490_d_n5, assign20680_e15490_d_n6, assign20680_e15490_d_n7, assign20680_e15490_d_n8, assign20680_e15490_d_n9, assign20680_e15490_d_n10, assign20680_e15490_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard413 != 0.0)) {
        let assign20680_e15477: f64 = (p.p292 * p.p292);
        let assign20680_e15481: f64 = (0.0001 * 0.01);
        let assign20680_e15482: f64 = (4.0 * assign20680_e15481);
        let assign20680_e15485: f64 = (0.0001 * 0.01);
        let assign20680_e15486: f64 = (assign20680_e15482 * assign20680_e15485);
        let assign20680_e15487: f64 = (assign20680_e15477 + assign20680_e15486);
        let assign20680_e15488: f64 = (assign20680_e15487).sqrt();
        (assign20680_e15488, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign20680_e15490;
        locals.var_tmf2_dn0 = assign20680_e15490_d_n0;
        locals.var_tmf2_dn2 = assign20680_e15490_d_n2;
        locals.var_tmf2_dn4 = assign20680_e15490_d_n4;
        locals.var_tmf2_dn5 = assign20680_e15490_d_n5;
        locals.var_tmf2_dn6 = assign20680_e15490_d_n6;
        locals.var_tmf2_dn7 = assign20680_e15490_d_n7;
        locals.var_tmf2_dn8 = assign20680_e15490_d_n8;
        locals.var_tmf2_dn9 = assign20680_e15490_d_n9;
        locals.var_tmf2_dn10 = assign20680_e15490_d_n10;
        locals.var_tmf2_dn13 = assign20680_e15490_d_n13;

        let (assign20690_e15504, assign20690_e15504_d_n0, assign20690_e15504_d_n2, assign20690_e15504_d_n4, assign20690_e15504_d_n5, assign20690_e15504_d_n6, assign20690_e15504_d_n7, assign20690_e15504_d_n8, assign20690_e15504_d_n9, assign20690_e15504_d_n10, assign20690_e15504_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard413 != 0.0)) {
        let assign20690_e15500: f64 = (p.p292 / locals.var_tmf2);
        let assign20690_e15501: f64 = (1.0 + assign20690_e15500);
        let assign20690_e15502: f64 = (0.5 * assign20690_e15501);
        (assign20690_e15502, (0.5 * (-((p.p292 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p292 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p292 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p292 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p292 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p292 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p292 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p292 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p292 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p292 * locals.var_tmf2_dn13) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign20690_e15504;
        locals.var_t0_dn0 = assign20690_e15504_d_n0;
        locals.var_t0_dn2 = assign20690_e15504_d_n2;
        locals.var_t0_dn4 = assign20690_e15504_d_n4;
        locals.var_t0_dn5 = assign20690_e15504_d_n5;
        locals.var_t0_dn6 = assign20690_e15504_d_n6;
        locals.var_t0_dn7 = assign20690_e15504_d_n7;
        locals.var_t0_dn8 = assign20690_e15504_d_n8;
        locals.var_t0_dn9 = assign20690_e15504_d_n9;
        locals.var_t0_dn10 = assign20690_e15504_d_n10;
        locals.var_t0_dn13 = assign20690_e15504_d_n13;

        let (assign20700_e15516, assign20700_e15516_d_n0, assign20700_e15516_d_n2, assign20700_e15516_d_n4, assign20700_e15516_d_n5, assign20700_e15516_d_n6, assign20700_e15516_d_n7, assign20700_e15516_d_n8, assign20700_e15516_d_n9, assign20700_e15516_d_n10, assign20700_e15516_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard413 != 0.0)) {
        let assign20700_e15513: f64 = (p.p292 + locals.var_tmf2);
        let assign20700_e15514: f64 = (0.5 * assign20700_e15513);
        (assign20700_e15514, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * locals.var_tmf2_dn5), (0.5 * locals.var_tmf2_dn6), (0.5 * locals.var_tmf2_dn7), (0.5 * locals.var_tmf2_dn8), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn13),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign20700_e15516;
        locals.var_t10_dn0 = assign20700_e15516_d_n0;
        locals.var_t10_dn2 = assign20700_e15516_d_n2;
        locals.var_t10_dn4 = assign20700_e15516_d_n4;
        locals.var_t10_dn5 = assign20700_e15516_d_n5;
        locals.var_t10_dn6 = assign20700_e15516_d_n6;
        locals.var_t10_dn7 = assign20700_e15516_d_n7;
        locals.var_t10_dn8 = assign20700_e15516_d_n8;
        locals.var_t10_dn9 = assign20700_e15516_d_n9;
        locals.var_t10_dn10 = assign20700_e15516_d_n10;
        locals.var_t10_dn13 = assign20700_e15516_d_n13;

        let assign20710_e15519: f64 = if locals.var_t10 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard414 = assign20710_e15519;

        let (assign20720_e15529, assign20720_e15529_d_n0, assign20720_e15529_d_n2, assign20720_e15529_d_n4, assign20720_e15529_d_n5, assign20720_e15529_d_n6, assign20720_e15529_d_n7, assign20720_e15529_d_n8, assign20720_e15529_d_n9, assign20720_e15529_d_n10, assign20720_e15529_d_n13,) = {
    if ((((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard413 != 0.0)) && (locals.var_guard414 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign20720_e15529;
        locals.var_t10_dn0 = assign20720_e15529_d_n0;
        locals.var_t10_dn2 = assign20720_e15529_d_n2;
        locals.var_t10_dn4 = assign20720_e15529_d_n4;
        locals.var_t10_dn5 = assign20720_e15529_d_n5;
        locals.var_t10_dn6 = assign20720_e15529_d_n6;
        locals.var_t10_dn7 = assign20720_e15529_d_n7;
        locals.var_t10_dn8 = assign20720_e15529_d_n8;
        locals.var_t10_dn9 = assign20720_e15529_d_n9;
        locals.var_t10_dn10 = assign20720_e15529_d_n10;
        locals.var_t10_dn13 = assign20720_e15529_d_n13;

        let (assign20730_e15539, assign20730_e15539_d_n0, assign20730_e15539_d_n2, assign20730_e15539_d_n4, assign20730_e15539_d_n5, assign20730_e15539_d_n6, assign20730_e15539_d_n7, assign20730_e15539_d_n8, assign20730_e15539_d_n9, assign20730_e15539_d_n10, assign20730_e15539_d_n13,) = {
    if ((((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard413 != 0.0)) && (locals.var_guard414 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign20730_e15539;
        locals.var_t0_dn0 = assign20730_e15539_d_n0;
        locals.var_t0_dn2 = assign20730_e15539_d_n2;
        locals.var_t0_dn4 = assign20730_e15539_d_n4;
        locals.var_t0_dn5 = assign20730_e15539_d_n5;
        locals.var_t0_dn6 = assign20730_e15539_d_n6;
        locals.var_t0_dn7 = assign20730_e15539_d_n7;
        locals.var_t0_dn8 = assign20730_e15539_d_n8;
        locals.var_t0_dn9 = assign20730_e15539_d_n9;
        locals.var_t0_dn10 = assign20730_e15539_d_n10;
        locals.var_t0_dn13 = assign20730_e15539_d_n13;

        let (assign20740_e15557, assign20740_e15557_d_n0, assign20740_e15557_d_n2, assign20740_e15557_d_n4, assign20740_e15557_d_n5, assign20740_e15557_d_n6, assign20740_e15557_d_n7, assign20740_e15557_d_n8, assign20740_e15557_d_n9, assign20740_e15557_d_n10, assign20740_e15557_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard413 != 0.0)) {
        let assign20740_e15551: f64 = (locals.var_vgserevz / locals.var_t10);
        let assign20740_e15552: f64 = (1.0 - assign20740_e15551);
        let assign20740_e15553: f64 = (locals.var_uc_rdvg11 * assign20740_e15552);
        let assign20740_e15554: f64 = (1.0 + assign20740_e15553);
        let assign20740_e15555: f64 = (locals.var_t4 * assign20740_e15554);
        (assign20740_e15555, ((locals.var_t4_dn0 * assign20740_e15554) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn0 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn0)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn2 * assign20740_e15554) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn2 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn2)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn4 * assign20740_e15554) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn4 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn4)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn5 * assign20740_e15554) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn5 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn5)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn6 * assign20740_e15554) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn6 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn6)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn7 * assign20740_e15554) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn7 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn7)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn8 * assign20740_e15554) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn8 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn8)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn9 * assign20740_e15554) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn9 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn9)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn10 * assign20740_e15554) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn10 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn10)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn13 * assign20740_e15554) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn13 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn13)) / (locals.var_t10 * locals.var_t10)))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign20740_e15557;
        locals.var_t1_dn0 = assign20740_e15557_d_n0;
        locals.var_t1_dn2 = assign20740_e15557_d_n2;
        locals.var_t1_dn4 = assign20740_e15557_d_n4;
        locals.var_t1_dn5 = assign20740_e15557_d_n5;
        locals.var_t1_dn6 = assign20740_e15557_d_n6;
        locals.var_t1_dn7 = assign20740_e15557_d_n7;
        locals.var_t1_dn8 = assign20740_e15557_d_n8;
        locals.var_t1_dn9 = assign20740_e15557_d_n9;
        locals.var_t1_dn10 = assign20740_e15557_d_n10;
        locals.var_t1_dn13 = assign20740_e15557_d_n13;

        let (assign20750_e15571, assign20750_e15571_d_n0, assign20750_e15571_d_n2, assign20750_e15571_d_n4, assign20750_e15571_d_n5, assign20750_e15571_d_n6, assign20750_e15571_d_n7, assign20750_e15571_d_n8, assign20750_e15571_d_n9, assign20750_e15571_d_n10, assign20750_e15571_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard413 != 0.0)) {
        let assign20750_e15565: f64 = (locals.var_t1 - locals.var_t4);
        let assign20750_e15568: f64 = (0.01 * 0.01);
        let assign20750_e15569: f64 = (assign20750_e15565 - assign20750_e15568);
        (assign20750_e15569, (locals.var_t1_dn0 - locals.var_t4_dn0), (locals.var_t1_dn2 - locals.var_t4_dn2), (locals.var_t1_dn4 - locals.var_t4_dn4), (locals.var_t1_dn5 - locals.var_t4_dn5), (locals.var_t1_dn6 - locals.var_t4_dn6), (locals.var_t1_dn7 - locals.var_t4_dn7), (locals.var_t1_dn8 - locals.var_t4_dn8), (locals.var_t1_dn9 - locals.var_t4_dn9), (locals.var_t1_dn10 - locals.var_t4_dn10), (locals.var_t1_dn13 - locals.var_t4_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign20750_e15571;
        locals.var_tmf1_dn0 = assign20750_e15571_d_n0;
        locals.var_tmf1_dn2 = assign20750_e15571_d_n2;
        locals.var_tmf1_dn4 = assign20750_e15571_d_n4;
        locals.var_tmf1_dn5 = assign20750_e15571_d_n5;
        locals.var_tmf1_dn6 = assign20750_e15571_d_n6;
        locals.var_tmf1_dn7 = assign20750_e15571_d_n7;
        locals.var_tmf1_dn8 = assign20750_e15571_d_n8;
        locals.var_tmf1_dn9 = assign20750_e15571_d_n9;
        locals.var_tmf1_dn10 = assign20750_e15571_d_n10;
        locals.var_tmf1_dn13 = assign20750_e15571_d_n13;

        let (assign20760_e15585, assign20760_e15585_d_n0, assign20760_e15585_d_n2, assign20760_e15585_d_n4, assign20760_e15585_d_n5, assign20760_e15585_d_n6, assign20760_e15585_d_n7, assign20760_e15585_d_n8, assign20760_e15585_d_n9, assign20760_e15585_d_n10, assign20760_e15585_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard413 != 0.0)) {
        let assign20760_e15579: f64 = (4.0 * locals.var_t4);
        let assign20760_e15582: f64 = (0.01 * 0.01);
        let assign20760_e15583: f64 = (assign20760_e15579 * assign20760_e15582);
        (assign20760_e15583, ((4.0 * locals.var_t4_dn0) * assign20760_e15582), ((4.0 * locals.var_t4_dn2) * assign20760_e15582), ((4.0 * locals.var_t4_dn4) * assign20760_e15582), ((4.0 * locals.var_t4_dn5) * assign20760_e15582), ((4.0 * locals.var_t4_dn6) * assign20760_e15582), ((4.0 * locals.var_t4_dn7) * assign20760_e15582), ((4.0 * locals.var_t4_dn8) * assign20760_e15582), ((4.0 * locals.var_t4_dn9) * assign20760_e15582), ((4.0 * locals.var_t4_dn10) * assign20760_e15582), ((4.0 * locals.var_t4_dn13) * assign20760_e15582),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign20760_e15585;
        locals.var_tmf2_dn0 = assign20760_e15585_d_n0;
        locals.var_tmf2_dn2 = assign20760_e15585_d_n2;
        locals.var_tmf2_dn4 = assign20760_e15585_d_n4;
        locals.var_tmf2_dn5 = assign20760_e15585_d_n5;
        locals.var_tmf2_dn6 = assign20760_e15585_d_n6;
        locals.var_tmf2_dn7 = assign20760_e15585_d_n7;
        locals.var_tmf2_dn8 = assign20760_e15585_d_n8;
        locals.var_tmf2_dn9 = assign20760_e15585_d_n9;
        locals.var_tmf2_dn10 = assign20760_e15585_d_n10;
        locals.var_tmf2_dn13 = assign20760_e15585_d_n13;

        let (assign20770_e15599, assign20770_e15599_d_n0, assign20770_e15599_d_n2, assign20770_e15599_d_n4, assign20770_e15599_d_n5, assign20770_e15599_d_n6, assign20770_e15599_d_n7, assign20770_e15599_d_n8, assign20770_e15599_d_n9, assign20770_e15599_d_n10, assign20770_e15599_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard413 != 0.0)) {
        let (assign20770_e15597, assign20770_e15597_d_n0, assign20770_e15597_d_n2, assign20770_e15597_d_n4, assign20770_e15597_d_n5, assign20770_e15597_d_n6, assign20770_e15597_d_n7, assign20770_e15597_d_n8, assign20770_e15597_d_n9, assign20770_e15597_d_n10, assign20770_e15597_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign20770_e15596: f64 = (-locals.var_tmf2);
                (assign20770_e15596, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign20770_e15597, assign20770_e15597_d_n0, assign20770_e15597_d_n2, assign20770_e15597_d_n4, assign20770_e15597_d_n5, assign20770_e15597_d_n6, assign20770_e15597_d_n7, assign20770_e15597_d_n8, assign20770_e15597_d_n9, assign20770_e15597_d_n10, assign20770_e15597_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign20770_e15599;
        locals.var_tmf2_dn0 = assign20770_e15599_d_n0;
        locals.var_tmf2_dn2 = assign20770_e15599_d_n2;
        locals.var_tmf2_dn4 = assign20770_e15599_d_n4;
        locals.var_tmf2_dn5 = assign20770_e15599_d_n5;
        locals.var_tmf2_dn6 = assign20770_e15599_d_n6;
        locals.var_tmf2_dn7 = assign20770_e15599_d_n7;
        locals.var_tmf2_dn8 = assign20770_e15599_d_n8;
        locals.var_tmf2_dn9 = assign20770_e15599_d_n9;
        locals.var_tmf2_dn10 = assign20770_e15599_d_n10;
        locals.var_tmf2_dn13 = assign20770_e15599_d_n13;

        let (assign20780_e15612, assign20780_e15612_d_n0, assign20780_e15612_d_n2, assign20780_e15612_d_n4, assign20780_e15612_d_n5, assign20780_e15612_d_n6, assign20780_e15612_d_n7, assign20780_e15612_d_n8, assign20780_e15612_d_n9, assign20780_e15612_d_n10, assign20780_e15612_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard413 != 0.0)) {
        let assign20780_e15607: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign20780_e15609: f64 = (assign20780_e15607 + locals.var_tmf2);
        let assign20780_e15610: f64 = (assign20780_e15609).sqrt();
        (assign20780_e15610, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign20780_e15610)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign20780_e15610)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign20780_e15610)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign20780_e15610)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign20780_e15610)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign20780_e15610)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign20780_e15610)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign20780_e15610)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign20780_e15610)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign20780_e15610)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign20780_e15612;
        locals.var_tmf2_dn0 = assign20780_e15612_d_n0;
        locals.var_tmf2_dn2 = assign20780_e15612_d_n2;
        locals.var_tmf2_dn4 = assign20780_e15612_d_n4;
        locals.var_tmf2_dn5 = assign20780_e15612_d_n5;
        locals.var_tmf2_dn6 = assign20780_e15612_d_n6;
        locals.var_tmf2_dn7 = assign20780_e15612_d_n7;
        locals.var_tmf2_dn8 = assign20780_e15612_d_n8;
        locals.var_tmf2_dn9 = assign20780_e15612_d_n9;
        locals.var_tmf2_dn10 = assign20780_e15612_d_n10;
        locals.var_tmf2_dn13 = assign20780_e15612_d_n13;

    }
}
