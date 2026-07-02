#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_32(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign15630_e10513, assign15630_e10513_d_n0, assign15630_e10513_d_n2, assign15630_e10513_d_n4, assign15630_e10513_d_n5, assign15630_e10513_d_n6, assign15630_e10513_d_n7, assign15630_e10513_d_n8, assign15630_e10513_d_n9, assign15630_e10513_d_n10, assign15630_e10513_d_n11, assign15630_e10513_d_n14,) = {
    if (locals.var_guard293 != 0.0) {
        let assign15630_e10499: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign15630_e10502: f64 = (locals.var_eg * locals.var_beta);
        let assign15630_e10503: f64 = (assign15630_e10499 - assign15630_e10502);
        let assign15630_e10506: f64 = (p.p509 * locals.var_log_tratio);
        let assign15630_e10507: f64 = (assign15630_e10503 + assign15630_e10506);
        let assign15630_e10509: f64 = (assign15630_e10507 / p.p497);
        let assign15630_e10510: f64 = (assign15630_e10509).exp();
        let assign15630_e10511: f64 = (locals.var_uc_js0swd * assign15630_e10510);
        (assign15630_e10511, (locals.var_uc_js0swd * (assign15630_e10510 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p509 * locals.var_log_tratio_dn0)) / p.p497))), (locals.var_uc_js0swd * (assign15630_e10510 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p509 * locals.var_log_tratio_dn2)) / p.p497))), (locals.var_uc_js0swd * (assign15630_e10510 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p509 * locals.var_log_tratio_dn4)) / p.p497))), (locals.var_uc_js0swd * (assign15630_e10510 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p509 * locals.var_log_tratio_dn5)) / p.p497))), (locals.var_uc_js0swd * (assign15630_e10510 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p509 * locals.var_log_tratio_dn6)) / p.p497))), (locals.var_uc_js0swd * (assign15630_e10510 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p509 * locals.var_log_tratio_dn7)) / p.p497))), (locals.var_uc_js0swd * (assign15630_e10510 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p509 * locals.var_log_tratio_dn8)) / p.p497))), (locals.var_uc_js0swd * (assign15630_e10510 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p509 * locals.var_log_tratio_dn9)) / p.p497))), (locals.var_uc_js0swd * (assign15630_e10510 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p509 * locals.var_log_tratio_dn10)) / p.p497))), (locals.var_uc_js0swd * (assign15630_e10510 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p509 * locals.var_log_tratio_dn11)) / p.p497))), (locals.var_uc_js0swd * (assign15630_e10510 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p509 * locals.var_log_tratio_dn14)) / p.p497))),)
    } else {
        (locals.var_jssw2, locals.var_jssw2_dn0, locals.var_jssw2_dn2, locals.var_jssw2_dn4, locals.var_jssw2_dn5, locals.var_jssw2_dn6, locals.var_jssw2_dn7, locals.var_jssw2_dn8, locals.var_jssw2_dn9, locals.var_jssw2_dn10, locals.var_jssw2_dn11, locals.var_jssw2_dn14,)
    }
};
        locals.var_jssw2 = assign15630_e10513;
        locals.var_jssw2_dn0 = assign15630_e10513_d_n0;
        locals.var_jssw2_dn2 = assign15630_e10513_d_n2;
        locals.var_jssw2_dn4 = assign15630_e10513_d_n4;
        locals.var_jssw2_dn5 = assign15630_e10513_d_n5;
        locals.var_jssw2_dn6 = assign15630_e10513_d_n6;
        locals.var_jssw2_dn7 = assign15630_e10513_d_n7;
        locals.var_jssw2_dn8 = assign15630_e10513_d_n8;
        locals.var_jssw2_dn9 = assign15630_e10513_d_n9;
        locals.var_jssw2_dn10 = assign15630_e10513_d_n10;
        locals.var_jssw2_dn11 = assign15630_e10513_d_n11;
        locals.var_jssw2_dn14 = assign15630_e10513_d_n14;

        let (assign15640_e10532, assign15640_e10532_d_n0, assign15640_e10532_d_n2, assign15640_e10532_d_n4, assign15640_e10532_d_n5, assign15640_e10532_d_n6, assign15640_e10532_d_n7, assign15640_e10532_d_n8, assign15640_e10532_d_n9, assign15640_e10532_d_n10, assign15640_e10532_d_n11, assign15640_e10532_d_n14,) = {
    if (locals.var_guard293 != 0.0) {
        let assign15640_e10518: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign15640_e10521: f64 = (locals.var_eg * locals.var_beta);
        let assign15640_e10522: f64 = (assign15640_e10518 - assign15640_e10521);
        let assign15640_e10525: f64 = (p.p509 * locals.var_log_tratio);
        let assign15640_e10526: f64 = (assign15640_e10522 + assign15640_e10525);
        let assign15640_e10528: f64 = (assign15640_e10526 / p.p498);
        let assign15640_e10529: f64 = (assign15640_e10528).exp();
        let assign15640_e10530: f64 = (p.p495 * assign15640_e10529);
        (assign15640_e10530, (p.p495 * (assign15640_e10529 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p509 * locals.var_log_tratio_dn0)) / p.p498))), (p.p495 * (assign15640_e10529 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p509 * locals.var_log_tratio_dn2)) / p.p498))), (p.p495 * (assign15640_e10529 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p509 * locals.var_log_tratio_dn4)) / p.p498))), (p.p495 * (assign15640_e10529 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p509 * locals.var_log_tratio_dn5)) / p.p498))), (p.p495 * (assign15640_e10529 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p509 * locals.var_log_tratio_dn6)) / p.p498))), (p.p495 * (assign15640_e10529 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p509 * locals.var_log_tratio_dn7)) / p.p498))), (p.p495 * (assign15640_e10529 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p509 * locals.var_log_tratio_dn8)) / p.p498))), (p.p495 * (assign15640_e10529 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p509 * locals.var_log_tratio_dn9)) / p.p498))), (p.p495 * (assign15640_e10529 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p509 * locals.var_log_tratio_dn10)) / p.p498))), (p.p495 * (assign15640_e10529 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p509 * locals.var_log_tratio_dn11)) / p.p498))), (p.p495 * (assign15640_e10529 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p509 * locals.var_log_tratio_dn14)) / p.p498))),)
    } else {
        (locals.var_jsswg2, locals.var_jsswg2_dn0, locals.var_jsswg2_dn2, locals.var_jsswg2_dn4, locals.var_jsswg2_dn5, locals.var_jsswg2_dn6, locals.var_jsswg2_dn7, locals.var_jsswg2_dn8, locals.var_jsswg2_dn9, locals.var_jsswg2_dn10, locals.var_jsswg2_dn11, locals.var_jsswg2_dn14,)
    }
};
        locals.var_jsswg2 = assign15640_e10532;
        locals.var_jsswg2_dn0 = assign15640_e10532_d_n0;
        locals.var_jsswg2_dn2 = assign15640_e10532_d_n2;
        locals.var_jsswg2_dn4 = assign15640_e10532_d_n4;
        locals.var_jsswg2_dn5 = assign15640_e10532_d_n5;
        locals.var_jsswg2_dn6 = assign15640_e10532_d_n6;
        locals.var_jsswg2_dn7 = assign15640_e10532_d_n7;
        locals.var_jsswg2_dn8 = assign15640_e10532_d_n8;
        locals.var_jsswg2_dn9 = assign15640_e10532_d_n9;
        locals.var_jsswg2_dn10 = assign15640_e10532_d_n10;
        locals.var_jsswg2_dn11 = assign15640_e10532_d_n11;
        locals.var_jsswg2_dn14 = assign15640_e10532_d_n14;

        let assign15650_e10535: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard333 = assign15650_e10535;

        let assign15660_e10538: f64 = if p.p15 > locals.var_weff_nf { 1.0 } else { 0.0 };
        locals.var_guard334 = assign15660_e10538;

        let (assign15670_e10548, assign15670_e10548_d_n0, assign15670_e10548_d_n2, assign15670_e10548_d_n4, assign15670_e10548_d_n5, assign15670_e10548_d_n6, assign15670_e10548_d_n7, assign15670_e10548_d_n8, assign15670_e10548_d_n9, assign15670_e10548_d_n10, assign15670_e10548_d_n11, assign15670_e10548_d_n14,) = {
    if (((locals.var_guard293 != 0.0) && (locals.var_guard333 != 0.0)) && (locals.var_guard334 != 0.0)) {
        let assign15670_e10546: f64 = (p.p13 * locals.var_js);
        (assign15670_e10546, (p.p13 * locals.var_js_dn0), (p.p13 * locals.var_js_dn2), (p.p13 * locals.var_js_dn4), (p.p13 * locals.var_js_dn5), (p.p13 * locals.var_js_dn6), (p.p13 * locals.var_js_dn7), (p.p13 * locals.var_js_dn8), (p.p13 * locals.var_js_dn9), (p.p13 * locals.var_js_dn10), (p.p13 * locals.var_js_dn11), (p.p13 * locals.var_js_dn14),)
    } else {
        (locals.var_isbd_btm, locals.var_isbd_btm_dn0, locals.var_isbd_btm_dn2, locals.var_isbd_btm_dn4, locals.var_isbd_btm_dn5, locals.var_isbd_btm_dn6, locals.var_isbd_btm_dn7, locals.var_isbd_btm_dn8, locals.var_isbd_btm_dn9, locals.var_isbd_btm_dn10, locals.var_isbd_btm_dn11, locals.var_isbd_btm_dn14,)
    }
};
        locals.var_isbd_btm = assign15670_e10548;
        locals.var_isbd_btm_dn0 = assign15670_e10548_d_n0;
        locals.var_isbd_btm_dn2 = assign15670_e10548_d_n2;
        locals.var_isbd_btm_dn4 = assign15670_e10548_d_n4;
        locals.var_isbd_btm_dn5 = assign15670_e10548_d_n5;
        locals.var_isbd_btm_dn6 = assign15670_e10548_d_n6;
        locals.var_isbd_btm_dn7 = assign15670_e10548_d_n7;
        locals.var_isbd_btm_dn8 = assign15670_e10548_d_n8;
        locals.var_isbd_btm_dn9 = assign15670_e10548_d_n9;
        locals.var_isbd_btm_dn10 = assign15670_e10548_d_n10;
        locals.var_isbd_btm_dn11 = assign15670_e10548_d_n11;
        locals.var_isbd_btm_dn14 = assign15670_e10548_d_n14;

        let (assign15680_e10558, assign15680_e10558_d_n0, assign15680_e10558_d_n2, assign15680_e10558_d_n4, assign15680_e10558_d_n5, assign15680_e10558_d_n6, assign15680_e10558_d_n7, assign15680_e10558_d_n8, assign15680_e10558_d_n9, assign15680_e10558_d_n10, assign15680_e10558_d_n11, assign15680_e10558_d_n14,) = {
    if (((locals.var_guard293 != 0.0) && (locals.var_guard333 != 0.0)) && (locals.var_guard334 != 0.0)) {
        let assign15680_e10556: f64 = (p.p13 * locals.var_js2);
        (assign15680_e10556, (p.p13 * locals.var_js2_dn0), (p.p13 * locals.var_js2_dn2), (p.p13 * locals.var_js2_dn4), (p.p13 * locals.var_js2_dn5), (p.p13 * locals.var_js2_dn6), (p.p13 * locals.var_js2_dn7), (p.p13 * locals.var_js2_dn8), (p.p13 * locals.var_js2_dn9), (p.p13 * locals.var_js2_dn10), (p.p13 * locals.var_js2_dn11), (p.p13 * locals.var_js2_dn14),)
    } else {
        (locals.var_isbd2_btm, locals.var_isbd2_btm_dn0, locals.var_isbd2_btm_dn2, locals.var_isbd2_btm_dn4, locals.var_isbd2_btm_dn5, locals.var_isbd2_btm_dn6, locals.var_isbd2_btm_dn7, locals.var_isbd2_btm_dn8, locals.var_isbd2_btm_dn9, locals.var_isbd2_btm_dn10, locals.var_isbd2_btm_dn11, locals.var_isbd2_btm_dn14,)
    }
};
        locals.var_isbd2_btm = assign15680_e10558;
        locals.var_isbd2_btm_dn0 = assign15680_e10558_d_n0;
        locals.var_isbd2_btm_dn2 = assign15680_e10558_d_n2;
        locals.var_isbd2_btm_dn4 = assign15680_e10558_d_n4;
        locals.var_isbd2_btm_dn5 = assign15680_e10558_d_n5;
        locals.var_isbd2_btm_dn6 = assign15680_e10558_d_n6;
        locals.var_isbd2_btm_dn7 = assign15680_e10558_d_n7;
        locals.var_isbd2_btm_dn8 = assign15680_e10558_d_n8;
        locals.var_isbd2_btm_dn9 = assign15680_e10558_d_n9;
        locals.var_isbd2_btm_dn10 = assign15680_e10558_d_n10;
        locals.var_isbd2_btm_dn11 = assign15680_e10558_d_n11;
        locals.var_isbd2_btm_dn14 = assign15680_e10558_d_n14;

        let (assign15690_e10570, assign15690_e10570_d_n0, assign15690_e10570_d_n2, assign15690_e10570_d_n4, assign15690_e10570_d_n5, assign15690_e10570_d_n6, assign15690_e10570_d_n7, assign15690_e10570_d_n8, assign15690_e10570_d_n9, assign15690_e10570_d_n10, assign15690_e10570_d_n11, assign15690_e10570_d_n14,) = {
    if (((locals.var_guard293 != 0.0) && (locals.var_guard333 != 0.0)) && (locals.var_guard334 != 0.0)) {
        let assign15690_e10566: f64 = (p.p15 - locals.var_weff_nf);
        let assign15690_e10568: f64 = (assign15690_e10566 * locals.var_jssw);
        (assign15690_e10568, (assign15690_e10566 * locals.var_jssw_dn0), (assign15690_e10566 * locals.var_jssw_dn2), (assign15690_e10566 * locals.var_jssw_dn4), (assign15690_e10566 * locals.var_jssw_dn5), (assign15690_e10566 * locals.var_jssw_dn6), (assign15690_e10566 * locals.var_jssw_dn7), (assign15690_e10566 * locals.var_jssw_dn8), (assign15690_e10566 * locals.var_jssw_dn9), (assign15690_e10566 * locals.var_jssw_dn10), (assign15690_e10566 * locals.var_jssw_dn11), (assign15690_e10566 * locals.var_jssw_dn14),)
    } else {
        (locals.var_isbd_sws, locals.var_isbd_sws_dn0, locals.var_isbd_sws_dn2, locals.var_isbd_sws_dn4, locals.var_isbd_sws_dn5, locals.var_isbd_sws_dn6, locals.var_isbd_sws_dn7, locals.var_isbd_sws_dn8, locals.var_isbd_sws_dn9, locals.var_isbd_sws_dn10, locals.var_isbd_sws_dn11, locals.var_isbd_sws_dn14,)
    }
};
        locals.var_isbd_sws = assign15690_e10570;
        locals.var_isbd_sws_dn0 = assign15690_e10570_d_n0;
        locals.var_isbd_sws_dn2 = assign15690_e10570_d_n2;
        locals.var_isbd_sws_dn4 = assign15690_e10570_d_n4;
        locals.var_isbd_sws_dn5 = assign15690_e10570_d_n5;
        locals.var_isbd_sws_dn6 = assign15690_e10570_d_n6;
        locals.var_isbd_sws_dn7 = assign15690_e10570_d_n7;
        locals.var_isbd_sws_dn8 = assign15690_e10570_d_n8;
        locals.var_isbd_sws_dn9 = assign15690_e10570_d_n9;
        locals.var_isbd_sws_dn10 = assign15690_e10570_d_n10;
        locals.var_isbd_sws_dn11 = assign15690_e10570_d_n11;
        locals.var_isbd_sws_dn14 = assign15690_e10570_d_n14;

        let (assign15700_e10582, assign15700_e10582_d_n0, assign15700_e10582_d_n2, assign15700_e10582_d_n4, assign15700_e10582_d_n5, assign15700_e10582_d_n6, assign15700_e10582_d_n7, assign15700_e10582_d_n8, assign15700_e10582_d_n9, assign15700_e10582_d_n10, assign15700_e10582_d_n11, assign15700_e10582_d_n14,) = {
    if (((locals.var_guard293 != 0.0) && (locals.var_guard333 != 0.0)) && (locals.var_guard334 != 0.0)) {
        let assign15700_e10578: f64 = (p.p15 - locals.var_weff_nf);
        let assign15700_e10580: f64 = (assign15700_e10578 * locals.var_jssw2);
        (assign15700_e10580, (assign15700_e10578 * locals.var_jssw2_dn0), (assign15700_e10578 * locals.var_jssw2_dn2), (assign15700_e10578 * locals.var_jssw2_dn4), (assign15700_e10578 * locals.var_jssw2_dn5), (assign15700_e10578 * locals.var_jssw2_dn6), (assign15700_e10578 * locals.var_jssw2_dn7), (assign15700_e10578 * locals.var_jssw2_dn8), (assign15700_e10578 * locals.var_jssw2_dn9), (assign15700_e10578 * locals.var_jssw2_dn10), (assign15700_e10578 * locals.var_jssw2_dn11), (assign15700_e10578 * locals.var_jssw2_dn14),)
    } else {
        (locals.var_isbd2_sws, locals.var_isbd2_sws_dn0, locals.var_isbd2_sws_dn2, locals.var_isbd2_sws_dn4, locals.var_isbd2_sws_dn5, locals.var_isbd2_sws_dn6, locals.var_isbd2_sws_dn7, locals.var_isbd2_sws_dn8, locals.var_isbd2_sws_dn9, locals.var_isbd2_sws_dn10, locals.var_isbd2_sws_dn11, locals.var_isbd2_sws_dn14,)
    }
};
        locals.var_isbd2_sws = assign15700_e10582;
        locals.var_isbd2_sws_dn0 = assign15700_e10582_d_n0;
        locals.var_isbd2_sws_dn2 = assign15700_e10582_d_n2;
        locals.var_isbd2_sws_dn4 = assign15700_e10582_d_n4;
        locals.var_isbd2_sws_dn5 = assign15700_e10582_d_n5;
        locals.var_isbd2_sws_dn6 = assign15700_e10582_d_n6;
        locals.var_isbd2_sws_dn7 = assign15700_e10582_d_n7;
        locals.var_isbd2_sws_dn8 = assign15700_e10582_d_n8;
        locals.var_isbd2_sws_dn9 = assign15700_e10582_d_n9;
        locals.var_isbd2_sws_dn10 = assign15700_e10582_d_n10;
        locals.var_isbd2_sws_dn11 = assign15700_e10582_d_n11;
        locals.var_isbd2_sws_dn14 = assign15700_e10582_d_n14;

        let (assign15710_e10592, assign15710_e10592_d_n0, assign15710_e10592_d_n2, assign15710_e10592_d_n4, assign15710_e10592_d_n5, assign15710_e10592_d_n6, assign15710_e10592_d_n7, assign15710_e10592_d_n8, assign15710_e10592_d_n9, assign15710_e10592_d_n10, assign15710_e10592_d_n11, assign15710_e10592_d_n14,) = {
    if (((locals.var_guard293 != 0.0) && (locals.var_guard333 != 0.0)) && (locals.var_guard334 != 0.0)) {
        let assign15710_e10590: f64 = (locals.var_weff_nf * locals.var_jsswg);
        (assign15710_e10590, (locals.var_weff_nf * locals.var_jsswg_dn0), (locals.var_weff_nf * locals.var_jsswg_dn2), (locals.var_weff_nf * locals.var_jsswg_dn4), (locals.var_weff_nf * locals.var_jsswg_dn5), (locals.var_weff_nf * locals.var_jsswg_dn6), (locals.var_weff_nf * locals.var_jsswg_dn7), (locals.var_weff_nf * locals.var_jsswg_dn8), (locals.var_weff_nf * locals.var_jsswg_dn9), (locals.var_weff_nf * locals.var_jsswg_dn10), (locals.var_weff_nf * locals.var_jsswg_dn11), (locals.var_weff_nf * locals.var_jsswg_dn14),)
    } else {
        (locals.var_isbd_swg, locals.var_isbd_swg_dn0, locals.var_isbd_swg_dn2, locals.var_isbd_swg_dn4, locals.var_isbd_swg_dn5, locals.var_isbd_swg_dn6, locals.var_isbd_swg_dn7, locals.var_isbd_swg_dn8, locals.var_isbd_swg_dn9, locals.var_isbd_swg_dn10, locals.var_isbd_swg_dn11, locals.var_isbd_swg_dn14,)
    }
};
        locals.var_isbd_swg = assign15710_e10592;
        locals.var_isbd_swg_dn0 = assign15710_e10592_d_n0;
        locals.var_isbd_swg_dn2 = assign15710_e10592_d_n2;
        locals.var_isbd_swg_dn4 = assign15710_e10592_d_n4;
        locals.var_isbd_swg_dn5 = assign15710_e10592_d_n5;
        locals.var_isbd_swg_dn6 = assign15710_e10592_d_n6;
        locals.var_isbd_swg_dn7 = assign15710_e10592_d_n7;
        locals.var_isbd_swg_dn8 = assign15710_e10592_d_n8;
        locals.var_isbd_swg_dn9 = assign15710_e10592_d_n9;
        locals.var_isbd_swg_dn10 = assign15710_e10592_d_n10;
        locals.var_isbd_swg_dn11 = assign15710_e10592_d_n11;
        locals.var_isbd_swg_dn14 = assign15710_e10592_d_n14;

        let (assign15720_e10602, assign15720_e10602_d_n0, assign15720_e10602_d_n2, assign15720_e10602_d_n4, assign15720_e10602_d_n5, assign15720_e10602_d_n6, assign15720_e10602_d_n7, assign15720_e10602_d_n8, assign15720_e10602_d_n9, assign15720_e10602_d_n10, assign15720_e10602_d_n11, assign15720_e10602_d_n14,) = {
    if (((locals.var_guard293 != 0.0) && (locals.var_guard333 != 0.0)) && (locals.var_guard334 != 0.0)) {
        let assign15720_e10600: f64 = (locals.var_weff_nf * locals.var_jsswg2);
        (assign15720_e10600, (locals.var_weff_nf * locals.var_jsswg2_dn0), (locals.var_weff_nf * locals.var_jsswg2_dn2), (locals.var_weff_nf * locals.var_jsswg2_dn4), (locals.var_weff_nf * locals.var_jsswg2_dn5), (locals.var_weff_nf * locals.var_jsswg2_dn6), (locals.var_weff_nf * locals.var_jsswg2_dn7), (locals.var_weff_nf * locals.var_jsswg2_dn8), (locals.var_weff_nf * locals.var_jsswg2_dn9), (locals.var_weff_nf * locals.var_jsswg2_dn10), (locals.var_weff_nf * locals.var_jsswg2_dn11), (locals.var_weff_nf * locals.var_jsswg2_dn14),)
    } else {
        (locals.var_isbd2_swg, locals.var_isbd2_swg_dn0, locals.var_isbd2_swg_dn2, locals.var_isbd2_swg_dn4, locals.var_isbd2_swg_dn5, locals.var_isbd2_swg_dn6, locals.var_isbd2_swg_dn7, locals.var_isbd2_swg_dn8, locals.var_isbd2_swg_dn9, locals.var_isbd2_swg_dn10, locals.var_isbd2_swg_dn11, locals.var_isbd2_swg_dn14,)
    }
};
        locals.var_isbd2_swg = assign15720_e10602;
        locals.var_isbd2_swg_dn0 = assign15720_e10602_d_n0;
        locals.var_isbd2_swg_dn2 = assign15720_e10602_d_n2;
        locals.var_isbd2_swg_dn4 = assign15720_e10602_d_n4;
        locals.var_isbd2_swg_dn5 = assign15720_e10602_d_n5;
        locals.var_isbd2_swg_dn6 = assign15720_e10602_d_n6;
        locals.var_isbd2_swg_dn7 = assign15720_e10602_d_n7;
        locals.var_isbd2_swg_dn8 = assign15720_e10602_d_n8;
        locals.var_isbd2_swg_dn9 = assign15720_e10602_d_n9;
        locals.var_isbd2_swg_dn10 = assign15720_e10602_d_n10;
        locals.var_isbd2_swg_dn11 = assign15720_e10602_d_n11;
        locals.var_isbd2_swg_dn14 = assign15720_e10602_d_n14;

        let (assign15730_e10613, assign15730_e10613_d_n0, assign15730_e10613_d_n2, assign15730_e10613_d_n4, assign15730_e10613_d_n5, assign15730_e10613_d_n6, assign15730_e10613_d_n7, assign15730_e10613_d_n8, assign15730_e10613_d_n9, assign15730_e10613_d_n10, assign15730_e10613_d_n11, assign15730_e10613_d_n14,) = {
    if (((locals.var_guard293 != 0.0) && (locals.var_guard333 != 0.0)) && (locals.var_guard334 == 0.0)) {
        let assign15730_e10611: f64 = (p.p13 * locals.var_js);
        (assign15730_e10611, (p.p13 * locals.var_js_dn0), (p.p13 * locals.var_js_dn2), (p.p13 * locals.var_js_dn4), (p.p13 * locals.var_js_dn5), (p.p13 * locals.var_js_dn6), (p.p13 * locals.var_js_dn7), (p.p13 * locals.var_js_dn8), (p.p13 * locals.var_js_dn9), (p.p13 * locals.var_js_dn10), (p.p13 * locals.var_js_dn11), (p.p13 * locals.var_js_dn14),)
    } else {
        (locals.var_isbd_btm, locals.var_isbd_btm_dn0, locals.var_isbd_btm_dn2, locals.var_isbd_btm_dn4, locals.var_isbd_btm_dn5, locals.var_isbd_btm_dn6, locals.var_isbd_btm_dn7, locals.var_isbd_btm_dn8, locals.var_isbd_btm_dn9, locals.var_isbd_btm_dn10, locals.var_isbd_btm_dn11, locals.var_isbd_btm_dn14,)
    }
};
        locals.var_isbd_btm = assign15730_e10613;
        locals.var_isbd_btm_dn0 = assign15730_e10613_d_n0;
        locals.var_isbd_btm_dn2 = assign15730_e10613_d_n2;
        locals.var_isbd_btm_dn4 = assign15730_e10613_d_n4;
        locals.var_isbd_btm_dn5 = assign15730_e10613_d_n5;
        locals.var_isbd_btm_dn6 = assign15730_e10613_d_n6;
        locals.var_isbd_btm_dn7 = assign15730_e10613_d_n7;
        locals.var_isbd_btm_dn8 = assign15730_e10613_d_n8;
        locals.var_isbd_btm_dn9 = assign15730_e10613_d_n9;
        locals.var_isbd_btm_dn10 = assign15730_e10613_d_n10;
        locals.var_isbd_btm_dn11 = assign15730_e10613_d_n11;
        locals.var_isbd_btm_dn14 = assign15730_e10613_d_n14;

        let (assign15740_e10624, assign15740_e10624_d_n0, assign15740_e10624_d_n2, assign15740_e10624_d_n4, assign15740_e10624_d_n5, assign15740_e10624_d_n6, assign15740_e10624_d_n7, assign15740_e10624_d_n8, assign15740_e10624_d_n9, assign15740_e10624_d_n10, assign15740_e10624_d_n11, assign15740_e10624_d_n14,) = {
    if (((locals.var_guard293 != 0.0) && (locals.var_guard333 != 0.0)) && (locals.var_guard334 == 0.0)) {
        let assign15740_e10622: f64 = (p.p13 * locals.var_js2);
        (assign15740_e10622, (p.p13 * locals.var_js2_dn0), (p.p13 * locals.var_js2_dn2), (p.p13 * locals.var_js2_dn4), (p.p13 * locals.var_js2_dn5), (p.p13 * locals.var_js2_dn6), (p.p13 * locals.var_js2_dn7), (p.p13 * locals.var_js2_dn8), (p.p13 * locals.var_js2_dn9), (p.p13 * locals.var_js2_dn10), (p.p13 * locals.var_js2_dn11), (p.p13 * locals.var_js2_dn14),)
    } else {
        (locals.var_isbd2_btm, locals.var_isbd2_btm_dn0, locals.var_isbd2_btm_dn2, locals.var_isbd2_btm_dn4, locals.var_isbd2_btm_dn5, locals.var_isbd2_btm_dn6, locals.var_isbd2_btm_dn7, locals.var_isbd2_btm_dn8, locals.var_isbd2_btm_dn9, locals.var_isbd2_btm_dn10, locals.var_isbd2_btm_dn11, locals.var_isbd2_btm_dn14,)
    }
};
        locals.var_isbd2_btm = assign15740_e10624;
        locals.var_isbd2_btm_dn0 = assign15740_e10624_d_n0;
        locals.var_isbd2_btm_dn2 = assign15740_e10624_d_n2;
        locals.var_isbd2_btm_dn4 = assign15740_e10624_d_n4;
        locals.var_isbd2_btm_dn5 = assign15740_e10624_d_n5;
        locals.var_isbd2_btm_dn6 = assign15740_e10624_d_n6;
        locals.var_isbd2_btm_dn7 = assign15740_e10624_d_n7;
        locals.var_isbd2_btm_dn8 = assign15740_e10624_d_n8;
        locals.var_isbd2_btm_dn9 = assign15740_e10624_d_n9;
        locals.var_isbd2_btm_dn10 = assign15740_e10624_d_n10;
        locals.var_isbd2_btm_dn11 = assign15740_e10624_d_n11;
        locals.var_isbd2_btm_dn14 = assign15740_e10624_d_n14;

        let (assign15750_e10633, assign15750_e10633_d_n0, assign15750_e10633_d_n2, assign15750_e10633_d_n4, assign15750_e10633_d_n5, assign15750_e10633_d_n6, assign15750_e10633_d_n7, assign15750_e10633_d_n8, assign15750_e10633_d_n9, assign15750_e10633_d_n10, assign15750_e10633_d_n11, assign15750_e10633_d_n14,) = {
    if (((locals.var_guard293 != 0.0) && (locals.var_guard333 != 0.0)) && (locals.var_guard334 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbd_sws, locals.var_isbd_sws_dn0, locals.var_isbd_sws_dn2, locals.var_isbd_sws_dn4, locals.var_isbd_sws_dn5, locals.var_isbd_sws_dn6, locals.var_isbd_sws_dn7, locals.var_isbd_sws_dn8, locals.var_isbd_sws_dn9, locals.var_isbd_sws_dn10, locals.var_isbd_sws_dn11, locals.var_isbd_sws_dn14,)
    }
};
        locals.var_isbd_sws = assign15750_e10633;
        locals.var_isbd_sws_dn0 = assign15750_e10633_d_n0;
        locals.var_isbd_sws_dn2 = assign15750_e10633_d_n2;
        locals.var_isbd_sws_dn4 = assign15750_e10633_d_n4;
        locals.var_isbd_sws_dn5 = assign15750_e10633_d_n5;
        locals.var_isbd_sws_dn6 = assign15750_e10633_d_n6;
        locals.var_isbd_sws_dn7 = assign15750_e10633_d_n7;
        locals.var_isbd_sws_dn8 = assign15750_e10633_d_n8;
        locals.var_isbd_sws_dn9 = assign15750_e10633_d_n9;
        locals.var_isbd_sws_dn10 = assign15750_e10633_d_n10;
        locals.var_isbd_sws_dn11 = assign15750_e10633_d_n11;
        locals.var_isbd_sws_dn14 = assign15750_e10633_d_n14;

        let (assign15760_e10642, assign15760_e10642_d_n0, assign15760_e10642_d_n2, assign15760_e10642_d_n4, assign15760_e10642_d_n5, assign15760_e10642_d_n6, assign15760_e10642_d_n7, assign15760_e10642_d_n8, assign15760_e10642_d_n9, assign15760_e10642_d_n10, assign15760_e10642_d_n11, assign15760_e10642_d_n14,) = {
    if (((locals.var_guard293 != 0.0) && (locals.var_guard333 != 0.0)) && (locals.var_guard334 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbd2_sws, locals.var_isbd2_sws_dn0, locals.var_isbd2_sws_dn2, locals.var_isbd2_sws_dn4, locals.var_isbd2_sws_dn5, locals.var_isbd2_sws_dn6, locals.var_isbd2_sws_dn7, locals.var_isbd2_sws_dn8, locals.var_isbd2_sws_dn9, locals.var_isbd2_sws_dn10, locals.var_isbd2_sws_dn11, locals.var_isbd2_sws_dn14,)
    }
};
        locals.var_isbd2_sws = assign15760_e10642;
        locals.var_isbd2_sws_dn0 = assign15760_e10642_d_n0;
        locals.var_isbd2_sws_dn2 = assign15760_e10642_d_n2;
        locals.var_isbd2_sws_dn4 = assign15760_e10642_d_n4;
        locals.var_isbd2_sws_dn5 = assign15760_e10642_d_n5;
        locals.var_isbd2_sws_dn6 = assign15760_e10642_d_n6;
        locals.var_isbd2_sws_dn7 = assign15760_e10642_d_n7;
        locals.var_isbd2_sws_dn8 = assign15760_e10642_d_n8;
        locals.var_isbd2_sws_dn9 = assign15760_e10642_d_n9;
        locals.var_isbd2_sws_dn10 = assign15760_e10642_d_n10;
        locals.var_isbd2_sws_dn11 = assign15760_e10642_d_n11;
        locals.var_isbd2_sws_dn14 = assign15760_e10642_d_n14;

        let (assign15770_e10653, assign15770_e10653_d_n0, assign15770_e10653_d_n2, assign15770_e10653_d_n4, assign15770_e10653_d_n5, assign15770_e10653_d_n6, assign15770_e10653_d_n7, assign15770_e10653_d_n8, assign15770_e10653_d_n9, assign15770_e10653_d_n10, assign15770_e10653_d_n11, assign15770_e10653_d_n14,) = {
    if (((locals.var_guard293 != 0.0) && (locals.var_guard333 != 0.0)) && (locals.var_guard334 == 0.0)) {
        let assign15770_e10651: f64 = (p.p15 * locals.var_jsswg);
        (assign15770_e10651, (p.p15 * locals.var_jsswg_dn0), (p.p15 * locals.var_jsswg_dn2), (p.p15 * locals.var_jsswg_dn4), (p.p15 * locals.var_jsswg_dn5), (p.p15 * locals.var_jsswg_dn6), (p.p15 * locals.var_jsswg_dn7), (p.p15 * locals.var_jsswg_dn8), (p.p15 * locals.var_jsswg_dn9), (p.p15 * locals.var_jsswg_dn10), (p.p15 * locals.var_jsswg_dn11), (p.p15 * locals.var_jsswg_dn14),)
    } else {
        (locals.var_isbd_swg, locals.var_isbd_swg_dn0, locals.var_isbd_swg_dn2, locals.var_isbd_swg_dn4, locals.var_isbd_swg_dn5, locals.var_isbd_swg_dn6, locals.var_isbd_swg_dn7, locals.var_isbd_swg_dn8, locals.var_isbd_swg_dn9, locals.var_isbd_swg_dn10, locals.var_isbd_swg_dn11, locals.var_isbd_swg_dn14,)
    }
};
        locals.var_isbd_swg = assign15770_e10653;
        locals.var_isbd_swg_dn0 = assign15770_e10653_d_n0;
        locals.var_isbd_swg_dn2 = assign15770_e10653_d_n2;
        locals.var_isbd_swg_dn4 = assign15770_e10653_d_n4;
        locals.var_isbd_swg_dn5 = assign15770_e10653_d_n5;
        locals.var_isbd_swg_dn6 = assign15770_e10653_d_n6;
        locals.var_isbd_swg_dn7 = assign15770_e10653_d_n7;
        locals.var_isbd_swg_dn8 = assign15770_e10653_d_n8;
        locals.var_isbd_swg_dn9 = assign15770_e10653_d_n9;
        locals.var_isbd_swg_dn10 = assign15770_e10653_d_n10;
        locals.var_isbd_swg_dn11 = assign15770_e10653_d_n11;
        locals.var_isbd_swg_dn14 = assign15770_e10653_d_n14;

        let (assign15780_e10664, assign15780_e10664_d_n0, assign15780_e10664_d_n2, assign15780_e10664_d_n4, assign15780_e10664_d_n5, assign15780_e10664_d_n6, assign15780_e10664_d_n7, assign15780_e10664_d_n8, assign15780_e10664_d_n9, assign15780_e10664_d_n10, assign15780_e10664_d_n11, assign15780_e10664_d_n14,) = {
    if (((locals.var_guard293 != 0.0) && (locals.var_guard333 != 0.0)) && (locals.var_guard334 == 0.0)) {
        let assign15780_e10662: f64 = (p.p15 * locals.var_jsswg2);
        (assign15780_e10662, (p.p15 * locals.var_jsswg2_dn0), (p.p15 * locals.var_jsswg2_dn2), (p.p15 * locals.var_jsswg2_dn4), (p.p15 * locals.var_jsswg2_dn5), (p.p15 * locals.var_jsswg2_dn6), (p.p15 * locals.var_jsswg2_dn7), (p.p15 * locals.var_jsswg2_dn8), (p.p15 * locals.var_jsswg2_dn9), (p.p15 * locals.var_jsswg2_dn10), (p.p15 * locals.var_jsswg2_dn11), (p.p15 * locals.var_jsswg2_dn14),)
    } else {
        (locals.var_isbd2_swg, locals.var_isbd2_swg_dn0, locals.var_isbd2_swg_dn2, locals.var_isbd2_swg_dn4, locals.var_isbd2_swg_dn5, locals.var_isbd2_swg_dn6, locals.var_isbd2_swg_dn7, locals.var_isbd2_swg_dn8, locals.var_isbd2_swg_dn9, locals.var_isbd2_swg_dn10, locals.var_isbd2_swg_dn11, locals.var_isbd2_swg_dn14,)
    }
};
        locals.var_isbd2_swg = assign15780_e10664;
        locals.var_isbd2_swg_dn0 = assign15780_e10664_d_n0;
        locals.var_isbd2_swg_dn2 = assign15780_e10664_d_n2;
        locals.var_isbd2_swg_dn4 = assign15780_e10664_d_n4;
        locals.var_isbd2_swg_dn5 = assign15780_e10664_d_n5;
        locals.var_isbd2_swg_dn6 = assign15780_e10664_d_n6;
        locals.var_isbd2_swg_dn7 = assign15780_e10664_d_n7;
        locals.var_isbd2_swg_dn8 = assign15780_e10664_d_n8;
        locals.var_isbd2_swg_dn9 = assign15780_e10664_d_n9;
        locals.var_isbd2_swg_dn10 = assign15780_e10664_d_n10;
        locals.var_isbd2_swg_dn11 = assign15780_e10664_d_n11;
        locals.var_isbd2_swg_dn14 = assign15780_e10664_d_n14;

        let (assign15790_e10673, assign15790_e10673_d_n0, assign15790_e10673_d_n2, assign15790_e10673_d_n4, assign15790_e10673_d_n5, assign15790_e10673_d_n6, assign15790_e10673_d_n7, assign15790_e10673_d_n8, assign15790_e10673_d_n9, assign15790_e10673_d_n10, assign15790_e10673_d_n11, assign15790_e10673_d_n14,) = {
    if ((locals.var_guard293 != 0.0) && (locals.var_guard333 == 0.0)) {
        let assign15790_e10671: f64 = (p.p13 * locals.var_js);
        (assign15790_e10671, (p.p13 * locals.var_js_dn0), (p.p13 * locals.var_js_dn2), (p.p13 * locals.var_js_dn4), (p.p13 * locals.var_js_dn5), (p.p13 * locals.var_js_dn6), (p.p13 * locals.var_js_dn7), (p.p13 * locals.var_js_dn8), (p.p13 * locals.var_js_dn9), (p.p13 * locals.var_js_dn10), (p.p13 * locals.var_js_dn11), (p.p13 * locals.var_js_dn14),)
    } else {
        (locals.var_isbd_btm, locals.var_isbd_btm_dn0, locals.var_isbd_btm_dn2, locals.var_isbd_btm_dn4, locals.var_isbd_btm_dn5, locals.var_isbd_btm_dn6, locals.var_isbd_btm_dn7, locals.var_isbd_btm_dn8, locals.var_isbd_btm_dn9, locals.var_isbd_btm_dn10, locals.var_isbd_btm_dn11, locals.var_isbd_btm_dn14,)
    }
};
        locals.var_isbd_btm = assign15790_e10673;
        locals.var_isbd_btm_dn0 = assign15790_e10673_d_n0;
        locals.var_isbd_btm_dn2 = assign15790_e10673_d_n2;
        locals.var_isbd_btm_dn4 = assign15790_e10673_d_n4;
        locals.var_isbd_btm_dn5 = assign15790_e10673_d_n5;
        locals.var_isbd_btm_dn6 = assign15790_e10673_d_n6;
        locals.var_isbd_btm_dn7 = assign15790_e10673_d_n7;
        locals.var_isbd_btm_dn8 = assign15790_e10673_d_n8;
        locals.var_isbd_btm_dn9 = assign15790_e10673_d_n9;
        locals.var_isbd_btm_dn10 = assign15790_e10673_d_n10;
        locals.var_isbd_btm_dn11 = assign15790_e10673_d_n11;
        locals.var_isbd_btm_dn14 = assign15790_e10673_d_n14;

        let (assign15800_e10682, assign15800_e10682_d_n0, assign15800_e10682_d_n2, assign15800_e10682_d_n4, assign15800_e10682_d_n5, assign15800_e10682_d_n6, assign15800_e10682_d_n7, assign15800_e10682_d_n8, assign15800_e10682_d_n9, assign15800_e10682_d_n10, assign15800_e10682_d_n11, assign15800_e10682_d_n14,) = {
    if ((locals.var_guard293 != 0.0) && (locals.var_guard333 == 0.0)) {
        let assign15800_e10680: f64 = (p.p13 * locals.var_js2);
        (assign15800_e10680, (p.p13 * locals.var_js2_dn0), (p.p13 * locals.var_js2_dn2), (p.p13 * locals.var_js2_dn4), (p.p13 * locals.var_js2_dn5), (p.p13 * locals.var_js2_dn6), (p.p13 * locals.var_js2_dn7), (p.p13 * locals.var_js2_dn8), (p.p13 * locals.var_js2_dn9), (p.p13 * locals.var_js2_dn10), (p.p13 * locals.var_js2_dn11), (p.p13 * locals.var_js2_dn14),)
    } else {
        (locals.var_isbd2_btm, locals.var_isbd2_btm_dn0, locals.var_isbd2_btm_dn2, locals.var_isbd2_btm_dn4, locals.var_isbd2_btm_dn5, locals.var_isbd2_btm_dn6, locals.var_isbd2_btm_dn7, locals.var_isbd2_btm_dn8, locals.var_isbd2_btm_dn9, locals.var_isbd2_btm_dn10, locals.var_isbd2_btm_dn11, locals.var_isbd2_btm_dn14,)
    }
};
        locals.var_isbd2_btm = assign15800_e10682;
        locals.var_isbd2_btm_dn0 = assign15800_e10682_d_n0;
        locals.var_isbd2_btm_dn2 = assign15800_e10682_d_n2;
        locals.var_isbd2_btm_dn4 = assign15800_e10682_d_n4;
        locals.var_isbd2_btm_dn5 = assign15800_e10682_d_n5;
        locals.var_isbd2_btm_dn6 = assign15800_e10682_d_n6;
        locals.var_isbd2_btm_dn7 = assign15800_e10682_d_n7;
        locals.var_isbd2_btm_dn8 = assign15800_e10682_d_n8;
        locals.var_isbd2_btm_dn9 = assign15800_e10682_d_n9;
        locals.var_isbd2_btm_dn10 = assign15800_e10682_d_n10;
        locals.var_isbd2_btm_dn11 = assign15800_e10682_d_n11;
        locals.var_isbd2_btm_dn14 = assign15800_e10682_d_n14;

        let (assign15810_e10691, assign15810_e10691_d_n0, assign15810_e10691_d_n2, assign15810_e10691_d_n4, assign15810_e10691_d_n5, assign15810_e10691_d_n6, assign15810_e10691_d_n7, assign15810_e10691_d_n8, assign15810_e10691_d_n9, assign15810_e10691_d_n10, assign15810_e10691_d_n11, assign15810_e10691_d_n14,) = {
    if ((locals.var_guard293 != 0.0) && (locals.var_guard333 == 0.0)) {
        let assign15810_e10689: f64 = (p.p15 * locals.var_jssw);
        (assign15810_e10689, (p.p15 * locals.var_jssw_dn0), (p.p15 * locals.var_jssw_dn2), (p.p15 * locals.var_jssw_dn4), (p.p15 * locals.var_jssw_dn5), (p.p15 * locals.var_jssw_dn6), (p.p15 * locals.var_jssw_dn7), (p.p15 * locals.var_jssw_dn8), (p.p15 * locals.var_jssw_dn9), (p.p15 * locals.var_jssw_dn10), (p.p15 * locals.var_jssw_dn11), (p.p15 * locals.var_jssw_dn14),)
    } else {
        (locals.var_isbd_sws, locals.var_isbd_sws_dn0, locals.var_isbd_sws_dn2, locals.var_isbd_sws_dn4, locals.var_isbd_sws_dn5, locals.var_isbd_sws_dn6, locals.var_isbd_sws_dn7, locals.var_isbd_sws_dn8, locals.var_isbd_sws_dn9, locals.var_isbd_sws_dn10, locals.var_isbd_sws_dn11, locals.var_isbd_sws_dn14,)
    }
};
        locals.var_isbd_sws = assign15810_e10691;
        locals.var_isbd_sws_dn0 = assign15810_e10691_d_n0;
        locals.var_isbd_sws_dn2 = assign15810_e10691_d_n2;
        locals.var_isbd_sws_dn4 = assign15810_e10691_d_n4;
        locals.var_isbd_sws_dn5 = assign15810_e10691_d_n5;
        locals.var_isbd_sws_dn6 = assign15810_e10691_d_n6;
        locals.var_isbd_sws_dn7 = assign15810_e10691_d_n7;
        locals.var_isbd_sws_dn8 = assign15810_e10691_d_n8;
        locals.var_isbd_sws_dn9 = assign15810_e10691_d_n9;
        locals.var_isbd_sws_dn10 = assign15810_e10691_d_n10;
        locals.var_isbd_sws_dn11 = assign15810_e10691_d_n11;
        locals.var_isbd_sws_dn14 = assign15810_e10691_d_n14;

        let (assign15820_e10700, assign15820_e10700_d_n0, assign15820_e10700_d_n2, assign15820_e10700_d_n4, assign15820_e10700_d_n5, assign15820_e10700_d_n6, assign15820_e10700_d_n7, assign15820_e10700_d_n8, assign15820_e10700_d_n9, assign15820_e10700_d_n10, assign15820_e10700_d_n11, assign15820_e10700_d_n14,) = {
    if ((locals.var_guard293 != 0.0) && (locals.var_guard333 == 0.0)) {
        let assign15820_e10698: f64 = (p.p15 * locals.var_jssw2);
        (assign15820_e10698, (p.p15 * locals.var_jssw2_dn0), (p.p15 * locals.var_jssw2_dn2), (p.p15 * locals.var_jssw2_dn4), (p.p15 * locals.var_jssw2_dn5), (p.p15 * locals.var_jssw2_dn6), (p.p15 * locals.var_jssw2_dn7), (p.p15 * locals.var_jssw2_dn8), (p.p15 * locals.var_jssw2_dn9), (p.p15 * locals.var_jssw2_dn10), (p.p15 * locals.var_jssw2_dn11), (p.p15 * locals.var_jssw2_dn14),)
    } else {
        (locals.var_isbd2_sws, locals.var_isbd2_sws_dn0, locals.var_isbd2_sws_dn2, locals.var_isbd2_sws_dn4, locals.var_isbd2_sws_dn5, locals.var_isbd2_sws_dn6, locals.var_isbd2_sws_dn7, locals.var_isbd2_sws_dn8, locals.var_isbd2_sws_dn9, locals.var_isbd2_sws_dn10, locals.var_isbd2_sws_dn11, locals.var_isbd2_sws_dn14,)
    }
};
        locals.var_isbd2_sws = assign15820_e10700;
        locals.var_isbd2_sws_dn0 = assign15820_e10700_d_n0;
        locals.var_isbd2_sws_dn2 = assign15820_e10700_d_n2;
        locals.var_isbd2_sws_dn4 = assign15820_e10700_d_n4;
        locals.var_isbd2_sws_dn5 = assign15820_e10700_d_n5;
        locals.var_isbd2_sws_dn6 = assign15820_e10700_d_n6;
        locals.var_isbd2_sws_dn7 = assign15820_e10700_d_n7;
        locals.var_isbd2_sws_dn8 = assign15820_e10700_d_n8;
        locals.var_isbd2_sws_dn9 = assign15820_e10700_d_n9;
        locals.var_isbd2_sws_dn10 = assign15820_e10700_d_n10;
        locals.var_isbd2_sws_dn11 = assign15820_e10700_d_n11;
        locals.var_isbd2_sws_dn14 = assign15820_e10700_d_n14;

        let (assign15830_e10707, assign15830_e10707_d_n0, assign15830_e10707_d_n2, assign15830_e10707_d_n4, assign15830_e10707_d_n5, assign15830_e10707_d_n6, assign15830_e10707_d_n7, assign15830_e10707_d_n8, assign15830_e10707_d_n9, assign15830_e10707_d_n10, assign15830_e10707_d_n11, assign15830_e10707_d_n14,) = {
    if ((locals.var_guard293 != 0.0) && (locals.var_guard333 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbd_swg, locals.var_isbd_swg_dn0, locals.var_isbd_swg_dn2, locals.var_isbd_swg_dn4, locals.var_isbd_swg_dn5, locals.var_isbd_swg_dn6, locals.var_isbd_swg_dn7, locals.var_isbd_swg_dn8, locals.var_isbd_swg_dn9, locals.var_isbd_swg_dn10, locals.var_isbd_swg_dn11, locals.var_isbd_swg_dn14,)
    }
};
        locals.var_isbd_swg = assign15830_e10707;
        locals.var_isbd_swg_dn0 = assign15830_e10707_d_n0;
        locals.var_isbd_swg_dn2 = assign15830_e10707_d_n2;
        locals.var_isbd_swg_dn4 = assign15830_e10707_d_n4;
        locals.var_isbd_swg_dn5 = assign15830_e10707_d_n5;
        locals.var_isbd_swg_dn6 = assign15830_e10707_d_n6;
        locals.var_isbd_swg_dn7 = assign15830_e10707_d_n7;
        locals.var_isbd_swg_dn8 = assign15830_e10707_d_n8;
        locals.var_isbd_swg_dn9 = assign15830_e10707_d_n9;
        locals.var_isbd_swg_dn10 = assign15830_e10707_d_n10;
        locals.var_isbd_swg_dn11 = assign15830_e10707_d_n11;
        locals.var_isbd_swg_dn14 = assign15830_e10707_d_n14;

        let (assign15840_e10714, assign15840_e10714_d_n0, assign15840_e10714_d_n2, assign15840_e10714_d_n4, assign15840_e10714_d_n5, assign15840_e10714_d_n6, assign15840_e10714_d_n7, assign15840_e10714_d_n8, assign15840_e10714_d_n9, assign15840_e10714_d_n10, assign15840_e10714_d_n11, assign15840_e10714_d_n14,) = {
    if ((locals.var_guard293 != 0.0) && (locals.var_guard333 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbd2_swg, locals.var_isbd2_swg_dn0, locals.var_isbd2_swg_dn2, locals.var_isbd2_swg_dn4, locals.var_isbd2_swg_dn5, locals.var_isbd2_swg_dn6, locals.var_isbd2_swg_dn7, locals.var_isbd2_swg_dn8, locals.var_isbd2_swg_dn9, locals.var_isbd2_swg_dn10, locals.var_isbd2_swg_dn11, locals.var_isbd2_swg_dn14,)
    }
};
        locals.var_isbd2_swg = assign15840_e10714;
        locals.var_isbd2_swg_dn0 = assign15840_e10714_d_n0;
        locals.var_isbd2_swg_dn2 = assign15840_e10714_d_n2;
        locals.var_isbd2_swg_dn4 = assign15840_e10714_d_n4;
        locals.var_isbd2_swg_dn5 = assign15840_e10714_d_n5;
        locals.var_isbd2_swg_dn6 = assign15840_e10714_d_n6;
        locals.var_isbd2_swg_dn7 = assign15840_e10714_d_n7;
        locals.var_isbd2_swg_dn8 = assign15840_e10714_d_n8;
        locals.var_isbd2_swg_dn9 = assign15840_e10714_d_n9;
        locals.var_isbd2_swg_dn10 = assign15840_e10714_d_n10;
        locals.var_isbd2_swg_dn11 = assign15840_e10714_d_n11;
        locals.var_isbd2_swg_dn14 = assign15840_e10714_d_n14;

        let (assign15850_e10722, assign15850_e10722_d_n0, assign15850_e10722_d_n2, assign15850_e10722_d_n4, assign15850_e10722_d_n5, assign15850_e10722_d_n6, assign15850_e10722_d_n7, assign15850_e10722_d_n8, assign15850_e10722_d_n9, assign15850_e10722_d_n10, assign15850_e10722_d_n11, assign15850_e10722_d_n14,) = {
    if (locals.var_guard293 != 0.0) {
        let assign15850_e10718: f64 = (locals.var_isbd_btm + locals.var_isbd_sws);
        let assign15850_e10720: f64 = (assign15850_e10718 + locals.var_isbd_swg);
        (assign15850_e10720, ((locals.var_isbd_btm_dn0 + locals.var_isbd_sws_dn0) + locals.var_isbd_swg_dn0), ((locals.var_isbd_btm_dn2 + locals.var_isbd_sws_dn2) + locals.var_isbd_swg_dn2), ((locals.var_isbd_btm_dn4 + locals.var_isbd_sws_dn4) + locals.var_isbd_swg_dn4), ((locals.var_isbd_btm_dn5 + locals.var_isbd_sws_dn5) + locals.var_isbd_swg_dn5), ((locals.var_isbd_btm_dn6 + locals.var_isbd_sws_dn6) + locals.var_isbd_swg_dn6), ((locals.var_isbd_btm_dn7 + locals.var_isbd_sws_dn7) + locals.var_isbd_swg_dn7), ((locals.var_isbd_btm_dn8 + locals.var_isbd_sws_dn8) + locals.var_isbd_swg_dn8), ((locals.var_isbd_btm_dn9 + locals.var_isbd_sws_dn9) + locals.var_isbd_swg_dn9), ((locals.var_isbd_btm_dn10 + locals.var_isbd_sws_dn10) + locals.var_isbd_swg_dn10), ((locals.var_isbd_btm_dn11 + locals.var_isbd_sws_dn11) + locals.var_isbd_swg_dn11), ((locals.var_isbd_btm_dn14 + locals.var_isbd_sws_dn14) + locals.var_isbd_swg_dn14),)
    } else {
        (locals.var_isbd, locals.var_isbd_dn0, locals.var_isbd_dn2, locals.var_isbd_dn4, locals.var_isbd_dn5, locals.var_isbd_dn6, locals.var_isbd_dn7, locals.var_isbd_dn8, locals.var_isbd_dn9, locals.var_isbd_dn10, locals.var_isbd_dn11, locals.var_isbd_dn14,)
    }
};
        locals.var_isbd = assign15850_e10722;
        locals.var_isbd_dn0 = assign15850_e10722_d_n0;
        locals.var_isbd_dn2 = assign15850_e10722_d_n2;
        locals.var_isbd_dn4 = assign15850_e10722_d_n4;
        locals.var_isbd_dn5 = assign15850_e10722_d_n5;
        locals.var_isbd_dn6 = assign15850_e10722_d_n6;
        locals.var_isbd_dn7 = assign15850_e10722_d_n7;
        locals.var_isbd_dn8 = assign15850_e10722_d_n8;
        locals.var_isbd_dn9 = assign15850_e10722_d_n9;
        locals.var_isbd_dn10 = assign15850_e10722_d_n10;
        locals.var_isbd_dn11 = assign15850_e10722_d_n11;
        locals.var_isbd_dn14 = assign15850_e10722_d_n14;

        let assign15860_e10725: f64 = if locals.var_isbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard335 = assign15860_e10725;

        let (assign15870_e10733, assign15870_e10733_d_n0, assign15870_e10733_d_n2, assign15870_e10733_d_n4, assign15870_e10733_d_n5, assign15870_e10733_d_n6, assign15870_e10733_d_n7, assign15870_e10733_d_n8, assign15870_e10733_d_n9, assign15870_e10733_d_n10, assign15870_e10733_d_n11, assign15870_e10733_d_n14,) = {
    if ((locals.var_guard293 != 0.0) && (locals.var_guard335 != 0.0)) {
        let assign15870_e10731: f64 = (locals.var_isbd + 1e-25);
        (assign15870_e10731, locals.var_isbd_dn0, locals.var_isbd_dn2, locals.var_isbd_dn4, locals.var_isbd_dn5, locals.var_isbd_dn6, locals.var_isbd_dn7, locals.var_isbd_dn8, locals.var_isbd_dn9, locals.var_isbd_dn10, locals.var_isbd_dn11, locals.var_isbd_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign15870_e10733;
        locals.var_t2_dn0 = assign15870_e10733_d_n0;
        locals.var_t2_dn2 = assign15870_e10733_d_n2;
        locals.var_t2_dn4 = assign15870_e10733_d_n4;
        locals.var_t2_dn5 = assign15870_e10733_d_n5;
        locals.var_t2_dn6 = assign15870_e10733_d_n6;
        locals.var_t2_dn7 = assign15870_e10733_d_n7;
        locals.var_t2_dn8 = assign15870_e10733_d_n8;
        locals.var_t2_dn9 = assign15870_e10733_d_n9;
        locals.var_t2_dn10 = assign15870_e10733_d_n10;
        locals.var_t2_dn11 = assign15870_e10733_d_n11;
        locals.var_t2_dn14 = assign15870_e10733_d_n14;

        let (assign15880_e10750, assign15880_e10750_d_n0, assign15880_e10750_d_n2, assign15880_e10750_d_n4, assign15880_e10750_d_n5, assign15880_e10750_d_n6, assign15880_e10750_d_n7, assign15880_e10750_d_n8, assign15880_e10750_d_n9, assign15880_e10750_d_n10, assign15880_e10750_d_n11, assign15880_e10750_d_n14,) = {
    if ((locals.var_guard293 != 0.0) && (locals.var_guard335 != 0.0)) {
        let assign15880_e10739: f64 = (locals.var_uc_njd / locals.var_beta);
        let assign15880_e10742: f64 = (locals.var_uc_vdiffjd * locals.var_t0);
        let assign15880_e10744: f64 = (assign15880_e10742 / locals.var_t2);
        let assign15880_e10746: f64 = (assign15880_e10744 + 1.0);
        let assign15880_e10747: f64 = (assign15880_e10746).ln();
        let assign15880_e10748: f64 = (assign15880_e10739 * assign15880_e10747);
        (assign15880_e10748, (((-((locals.var_uc_njd * locals.var_beta_dn0) / (locals.var_beta * locals.var_beta))) * assign15880_e10747) + (assign15880_e10739 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn0) * locals.var_t2) - (assign15880_e10742 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)) / assign15880_e10746))), (((-((locals.var_uc_njd * locals.var_beta_dn2) / (locals.var_beta * locals.var_beta))) * assign15880_e10747) + (assign15880_e10739 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn2) * locals.var_t2) - (assign15880_e10742 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)) / assign15880_e10746))), (((-((locals.var_uc_njd * locals.var_beta_dn4) / (locals.var_beta * locals.var_beta))) * assign15880_e10747) + (assign15880_e10739 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn4) * locals.var_t2) - (assign15880_e10742 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)) / assign15880_e10746))), (((-((locals.var_uc_njd * locals.var_beta_dn5) / (locals.var_beta * locals.var_beta))) * assign15880_e10747) + (assign15880_e10739 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn5) * locals.var_t2) - (assign15880_e10742 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)) / assign15880_e10746))), (((-((locals.var_uc_njd * locals.var_beta_dn6) / (locals.var_beta * locals.var_beta))) * assign15880_e10747) + (assign15880_e10739 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn6) * locals.var_t2) - (assign15880_e10742 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)) / assign15880_e10746))), (((-((locals.var_uc_njd * locals.var_beta_dn7) / (locals.var_beta * locals.var_beta))) * assign15880_e10747) + (assign15880_e10739 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn7) * locals.var_t2) - (assign15880_e10742 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)) / assign15880_e10746))), (((-((locals.var_uc_njd * locals.var_beta_dn8) / (locals.var_beta * locals.var_beta))) * assign15880_e10747) + (assign15880_e10739 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn8) * locals.var_t2) - (assign15880_e10742 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)) / assign15880_e10746))), (((-((locals.var_uc_njd * locals.var_beta_dn9) / (locals.var_beta * locals.var_beta))) * assign15880_e10747) + (assign15880_e10739 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn9) * locals.var_t2) - (assign15880_e10742 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)) / assign15880_e10746))), (((-((locals.var_uc_njd * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) * assign15880_e10747) + (assign15880_e10739 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn10) * locals.var_t2) - (assign15880_e10742 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)) / assign15880_e10746))), (((-((locals.var_uc_njd * locals.var_beta_dn11) / (locals.var_beta * locals.var_beta))) * assign15880_e10747) + (assign15880_e10739 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn11) * locals.var_t2) - (assign15880_e10742 * locals.var_t2_dn11)) / (locals.var_t2 * locals.var_t2)) / assign15880_e10746))), (((-((locals.var_uc_njd * locals.var_beta_dn14) / (locals.var_beta * locals.var_beta))) * assign15880_e10747) + (assign15880_e10739 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn14) * locals.var_t2) - (assign15880_e10742 * locals.var_t2_dn14)) / (locals.var_t2 * locals.var_t2)) / assign15880_e10746))),)
    } else {
        (locals.var_vbdt, locals.var_vbdt_dn0, locals.var_vbdt_dn2, locals.var_vbdt_dn4, locals.var_vbdt_dn5, locals.var_vbdt_dn6, locals.var_vbdt_dn7, locals.var_vbdt_dn8, locals.var_vbdt_dn9, locals.var_vbdt_dn10, locals.var_vbdt_dn11, locals.var_vbdt_dn14,)
    }
};
        locals.var_vbdt = assign15880_e10750;
        locals.var_vbdt_dn0 = assign15880_e10750_d_n0;
        locals.var_vbdt_dn2 = assign15880_e10750_d_n2;
        locals.var_vbdt_dn4 = assign15880_e10750_d_n4;
        locals.var_vbdt_dn5 = assign15880_e10750_d_n5;
        locals.var_vbdt_dn6 = assign15880_e10750_d_n6;
        locals.var_vbdt_dn7 = assign15880_e10750_d_n7;
        locals.var_vbdt_dn8 = assign15880_e10750_d_n8;
        locals.var_vbdt_dn9 = assign15880_e10750_d_n9;
        locals.var_vbdt_dn10 = assign15880_e10750_d_n10;
        locals.var_vbdt_dn11 = assign15880_e10750_d_n11;
        locals.var_vbdt_dn14 = assign15880_e10750_d_n14;

    }

    pub(super) fn stamp_transient_block_33(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign15890_e10761, assign15890_e10761_d_n0, assign15890_e10761_d_n2, assign15890_e10761_d_n4, assign15890_e10761_d_n5, assign15890_e10761_d_n6, assign15890_e10761_d_n7, assign15890_e10761_d_n8, assign15890_e10761_d_n9, assign15890_e10761_d_n10, assign15890_e10761_d_n11, assign15890_e10761_d_n14,) = {
    if ((locals.var_guard293 != 0.0) && (locals.var_guard335 != 0.0)) {
        let assign15890_e10756: f64 = (locals.var_tratio - 1.0);
        let assign15890_e10758: f64 = (assign15890_e10756 * p.p512);
        let assign15890_e10759: f64 = (assign15890_e10758).exp();
        (assign15890_e10759, (assign15890_e10759 * (locals.var_tratio_dn0 * p.p512)), (assign15890_e10759 * (locals.var_tratio_dn2 * p.p512)), (assign15890_e10759 * (locals.var_tratio_dn4 * p.p512)), (assign15890_e10759 * (locals.var_tratio_dn5 * p.p512)), (assign15890_e10759 * (locals.var_tratio_dn6 * p.p512)), (assign15890_e10759 * (locals.var_tratio_dn7 * p.p512)), (assign15890_e10759 * (locals.var_tratio_dn8 * p.p512)), (assign15890_e10759 * (locals.var_tratio_dn9 * p.p512)), (assign15890_e10759 * (locals.var_tratio_dn10 * p.p512)), (assign15890_e10759 * (locals.var_tratio_dn11 * p.p512)), (assign15890_e10759 * (locals.var_tratio_dn14 * p.p512)),)
    } else {
        (locals.var_exptempd, locals.var_exptempd_dn0, locals.var_exptempd_dn2, locals.var_exptempd_dn4, locals.var_exptempd_dn5, locals.var_exptempd_dn6, locals.var_exptempd_dn7, locals.var_exptempd_dn8, locals.var_exptempd_dn9, locals.var_exptempd_dn10, locals.var_exptempd_dn11, locals.var_exptempd_dn14,)
    }
};
        locals.var_exptempd = assign15890_e10761;
        locals.var_exptempd_dn0 = assign15890_e10761_d_n0;
        locals.var_exptempd_dn2 = assign15890_e10761_d_n2;
        locals.var_exptempd_dn4 = assign15890_e10761_d_n4;
        locals.var_exptempd_dn5 = assign15890_e10761_d_n5;
        locals.var_exptempd_dn6 = assign15890_e10761_d_n6;
        locals.var_exptempd_dn7 = assign15890_e10761_d_n7;
        locals.var_exptempd_dn8 = assign15890_e10761_d_n8;
        locals.var_exptempd_dn9 = assign15890_e10761_d_n9;
        locals.var_exptempd_dn10 = assign15890_e10761_d_n10;
        locals.var_exptempd_dn11 = assign15890_e10761_d_n11;
        locals.var_exptempd_dn14 = assign15890_e10761_d_n14;

        let (assign15900_e10771, assign15900_e10771_d_n0, assign15900_e10771_d_n2, assign15900_e10771_d_n4, assign15900_e10771_d_n5, assign15900_e10771_d_n6, assign15900_e10771_d_n7, assign15900_e10771_d_n8, assign15900_e10771_d_n9, assign15900_e10771_d_n10, assign15900_e10771_d_n11, assign15900_e10771_d_n14,) = {
    if ((locals.var_guard293 != 0.0) && (locals.var_guard335 != 0.0)) {
        let assign15900_e10768: f64 = (locals.var_uc_njd / locals.var_beta);
        let assign15900_e10769: f64 = (1.0 / assign15900_e10768);
        (assign15900_e10769, (-((-((locals.var_uc_njd * locals.var_beta_dn0) / (locals.var_beta * locals.var_beta))) / (assign15900_e10768 * assign15900_e10768))), (-((-((locals.var_uc_njd * locals.var_beta_dn2) / (locals.var_beta * locals.var_beta))) / (assign15900_e10768 * assign15900_e10768))), (-((-((locals.var_uc_njd * locals.var_beta_dn4) / (locals.var_beta * locals.var_beta))) / (assign15900_e10768 * assign15900_e10768))), (-((-((locals.var_uc_njd * locals.var_beta_dn5) / (locals.var_beta * locals.var_beta))) / (assign15900_e10768 * assign15900_e10768))), (-((-((locals.var_uc_njd * locals.var_beta_dn6) / (locals.var_beta * locals.var_beta))) / (assign15900_e10768 * assign15900_e10768))), (-((-((locals.var_uc_njd * locals.var_beta_dn7) / (locals.var_beta * locals.var_beta))) / (assign15900_e10768 * assign15900_e10768))), (-((-((locals.var_uc_njd * locals.var_beta_dn8) / (locals.var_beta * locals.var_beta))) / (assign15900_e10768 * assign15900_e10768))), (-((-((locals.var_uc_njd * locals.var_beta_dn9) / (locals.var_beta * locals.var_beta))) / (assign15900_e10768 * assign15900_e10768))), (-((-((locals.var_uc_njd * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) / (assign15900_e10768 * assign15900_e10768))), (-((-((locals.var_uc_njd * locals.var_beta_dn11) / (locals.var_beta * locals.var_beta))) / (assign15900_e10768 * assign15900_e10768))), (-((-((locals.var_uc_njd * locals.var_beta_dn14) / (locals.var_beta * locals.var_beta))) / (assign15900_e10768 * assign15900_e10768))),)
    } else {
        (locals.var_jd_nvtm_invd, locals.var_jd_nvtm_invd_dn0, locals.var_jd_nvtm_invd_dn2, locals.var_jd_nvtm_invd_dn4, locals.var_jd_nvtm_invd_dn5, locals.var_jd_nvtm_invd_dn6, locals.var_jd_nvtm_invd_dn7, locals.var_jd_nvtm_invd_dn8, locals.var_jd_nvtm_invd_dn9, locals.var_jd_nvtm_invd_dn10, locals.var_jd_nvtm_invd_dn11, locals.var_jd_nvtm_invd_dn14,)
    }
};
        locals.var_jd_nvtm_invd = assign15900_e10771;
        locals.var_jd_nvtm_invd_dn0 = assign15900_e10771_d_n0;
        locals.var_jd_nvtm_invd_dn2 = assign15900_e10771_d_n2;
        locals.var_jd_nvtm_invd_dn4 = assign15900_e10771_d_n4;
        locals.var_jd_nvtm_invd_dn5 = assign15900_e10771_d_n5;
        locals.var_jd_nvtm_invd_dn6 = assign15900_e10771_d_n6;
        locals.var_jd_nvtm_invd_dn7 = assign15900_e10771_d_n7;
        locals.var_jd_nvtm_invd_dn8 = assign15900_e10771_d_n8;
        locals.var_jd_nvtm_invd_dn9 = assign15900_e10771_d_n9;
        locals.var_jd_nvtm_invd_dn10 = assign15900_e10771_d_n10;
        locals.var_jd_nvtm_invd_dn11 = assign15900_e10771_d_n11;
        locals.var_jd_nvtm_invd_dn14 = assign15900_e10771_d_n14;

        let (assign15910_e10780, assign15910_e10780_d_n0, assign15910_e10780_d_n2, assign15910_e10780_d_n4, assign15910_e10780_d_n5, assign15910_e10780_d_n6, assign15910_e10780_d_n7, assign15910_e10780_d_n8, assign15910_e10780_d_n9, assign15910_e10780_d_n10, assign15910_e10780_d_n11, assign15910_e10780_d_n14,) = {
    if ((locals.var_guard293 != 0.0) && (locals.var_guard335 != 0.0)) {
        let assign15910_e10777: f64 = (locals.var_vbdt * locals.var_jd_nvtm_invd);
        let assign15910_e10778: f64 = (assign15910_e10777).exp();
        (assign15910_e10778, (assign15910_e10778 * ((locals.var_vbdt_dn0 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn0))), (assign15910_e10778 * ((locals.var_vbdt_dn2 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn2))), (assign15910_e10778 * ((locals.var_vbdt_dn4 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn4))), (assign15910_e10778 * ((locals.var_vbdt_dn5 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn5))), (assign15910_e10778 * ((locals.var_vbdt_dn6 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn6))), (assign15910_e10778 * ((locals.var_vbdt_dn7 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn7))), (assign15910_e10778 * ((locals.var_vbdt_dn8 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn8))), (assign15910_e10778 * ((locals.var_vbdt_dn9 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn9))), (assign15910_e10778 * ((locals.var_vbdt_dn10 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn10))), (assign15910_e10778 * ((locals.var_vbdt_dn11 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn11))), (assign15910_e10778 * ((locals.var_vbdt_dn14 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn14))),)
    } else {
        (locals.var_jd_expcd, locals.var_jd_expcd_dn0, locals.var_jd_expcd_dn2, locals.var_jd_expcd_dn4, locals.var_jd_expcd_dn5, locals.var_jd_expcd_dn6, locals.var_jd_expcd_dn7, locals.var_jd_expcd_dn8, locals.var_jd_expcd_dn9, locals.var_jd_expcd_dn10, locals.var_jd_expcd_dn11, locals.var_jd_expcd_dn14,)
    }
};
        locals.var_jd_expcd = assign15910_e10780;
        locals.var_jd_expcd_dn0 = assign15910_e10780_d_n0;
        locals.var_jd_expcd_dn2 = assign15910_e10780_d_n2;
        locals.var_jd_expcd_dn4 = assign15910_e10780_d_n4;
        locals.var_jd_expcd_dn5 = assign15910_e10780_d_n5;
        locals.var_jd_expcd_dn6 = assign15910_e10780_d_n6;
        locals.var_jd_expcd_dn7 = assign15910_e10780_d_n7;
        locals.var_jd_expcd_dn8 = assign15910_e10780_d_n8;
        locals.var_jd_expcd_dn9 = assign15910_e10780_d_n9;
        locals.var_jd_expcd_dn10 = assign15910_e10780_d_n10;
        locals.var_jd_expcd_dn11 = assign15910_e10780_d_n11;
        locals.var_jd_expcd_dn14 = assign15910_e10780_d_n14;

        let (assign15920_e10799, assign15920_e10799_d_n0, assign15920_e10799_d_n2, assign15920_e10799_d_n4, assign15920_e10799_d_n5, assign15920_e10799_d_n6, assign15920_e10799_d_n7, assign15920_e10799_d_n8, assign15920_e10799_d_n9, assign15920_e10799_d_n10, assign15920_e10799_d_n11, assign15920_e10799_d_n14,) = {
    if (locals.var_guard293 != 0.0) {
        let assign15920_e10785: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign15920_e10788: f64 = (locals.var_eg * locals.var_beta);
        let assign15920_e10789: f64 = (assign15920_e10785 - assign15920_e10788);
        let assign15920_e10792: f64 = (p.p522 * locals.var_log_tratio);
        let assign15920_e10793: f64 = (assign15920_e10789 + assign15920_e10792);
        let assign15920_e10795: f64 = (assign15920_e10793 / locals.var_uc_njs);
        let assign15920_e10796: f64 = (assign15920_e10795).exp();
        let assign15920_e10797: f64 = (locals.var_uc_js0s * assign15920_e10796);
        (assign15920_e10797, (locals.var_uc_js0s * (assign15920_e10796 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p522 * locals.var_log_tratio_dn0)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15920_e10796 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p522 * locals.var_log_tratio_dn2)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15920_e10796 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p522 * locals.var_log_tratio_dn4)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15920_e10796 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p522 * locals.var_log_tratio_dn5)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15920_e10796 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p522 * locals.var_log_tratio_dn6)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15920_e10796 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p522 * locals.var_log_tratio_dn7)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15920_e10796 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p522 * locals.var_log_tratio_dn8)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15920_e10796 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p522 * locals.var_log_tratio_dn9)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15920_e10796 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p522 * locals.var_log_tratio_dn10)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15920_e10796 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p522 * locals.var_log_tratio_dn11)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15920_e10796 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p522 * locals.var_log_tratio_dn14)) / locals.var_uc_njs))),)
    } else {
        (locals.var_js, locals.var_js_dn0, locals.var_js_dn2, locals.var_js_dn4, locals.var_js_dn5, locals.var_js_dn6, locals.var_js_dn7, locals.var_js_dn8, locals.var_js_dn9, locals.var_js_dn10, locals.var_js_dn11, locals.var_js_dn14,)
    }
};
        locals.var_js = assign15920_e10799;
        locals.var_js_dn0 = assign15920_e10799_d_n0;
        locals.var_js_dn2 = assign15920_e10799_d_n2;
        locals.var_js_dn4 = assign15920_e10799_d_n4;
        locals.var_js_dn5 = assign15920_e10799_d_n5;
        locals.var_js_dn6 = assign15920_e10799_d_n6;
        locals.var_js_dn7 = assign15920_e10799_d_n7;
        locals.var_js_dn8 = assign15920_e10799_d_n8;
        locals.var_js_dn9 = assign15920_e10799_d_n9;
        locals.var_js_dn10 = assign15920_e10799_d_n10;
        locals.var_js_dn11 = assign15920_e10799_d_n11;
        locals.var_js_dn14 = assign15920_e10799_d_n14;

        let (assign15930_e10818, assign15930_e10818_d_n0, assign15930_e10818_d_n2, assign15930_e10818_d_n4, assign15930_e10818_d_n5, assign15930_e10818_d_n6, assign15930_e10818_d_n7, assign15930_e10818_d_n8, assign15930_e10818_d_n9, assign15930_e10818_d_n10, assign15930_e10818_d_n11, assign15930_e10818_d_n14,) = {
    if (locals.var_guard293 != 0.0) {
        let assign15930_e10804: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign15930_e10807: f64 = (locals.var_eg * locals.var_beta);
        let assign15930_e10808: f64 = (assign15930_e10804 - assign15930_e10807);
        let assign15930_e10811: f64 = (p.p522 * locals.var_log_tratio);
        let assign15930_e10812: f64 = (assign15930_e10808 + assign15930_e10811);
        let assign15930_e10814: f64 = (assign15930_e10812 / p.p520);
        let assign15930_e10815: f64 = (assign15930_e10814).exp();
        let assign15930_e10816: f64 = (locals.var_uc_js0sws * assign15930_e10815);
        (assign15930_e10816, (locals.var_uc_js0sws * (assign15930_e10815 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p522 * locals.var_log_tratio_dn0)) / p.p520))), (locals.var_uc_js0sws * (assign15930_e10815 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p522 * locals.var_log_tratio_dn2)) / p.p520))), (locals.var_uc_js0sws * (assign15930_e10815 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p522 * locals.var_log_tratio_dn4)) / p.p520))), (locals.var_uc_js0sws * (assign15930_e10815 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p522 * locals.var_log_tratio_dn5)) / p.p520))), (locals.var_uc_js0sws * (assign15930_e10815 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p522 * locals.var_log_tratio_dn6)) / p.p520))), (locals.var_uc_js0sws * (assign15930_e10815 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p522 * locals.var_log_tratio_dn7)) / p.p520))), (locals.var_uc_js0sws * (assign15930_e10815 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p522 * locals.var_log_tratio_dn8)) / p.p520))), (locals.var_uc_js0sws * (assign15930_e10815 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p522 * locals.var_log_tratio_dn9)) / p.p520))), (locals.var_uc_js0sws * (assign15930_e10815 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p522 * locals.var_log_tratio_dn10)) / p.p520))), (locals.var_uc_js0sws * (assign15930_e10815 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p522 * locals.var_log_tratio_dn11)) / p.p520))), (locals.var_uc_js0sws * (assign15930_e10815 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p522 * locals.var_log_tratio_dn14)) / p.p520))),)
    } else {
        (locals.var_jssw, locals.var_jssw_dn0, locals.var_jssw_dn2, locals.var_jssw_dn4, locals.var_jssw_dn5, locals.var_jssw_dn6, locals.var_jssw_dn7, locals.var_jssw_dn8, locals.var_jssw_dn9, locals.var_jssw_dn10, locals.var_jssw_dn11, locals.var_jssw_dn14,)
    }
};
        locals.var_jssw = assign15930_e10818;
        locals.var_jssw_dn0 = assign15930_e10818_d_n0;
        locals.var_jssw_dn2 = assign15930_e10818_d_n2;
        locals.var_jssw_dn4 = assign15930_e10818_d_n4;
        locals.var_jssw_dn5 = assign15930_e10818_d_n5;
        locals.var_jssw_dn6 = assign15930_e10818_d_n6;
        locals.var_jssw_dn7 = assign15930_e10818_d_n7;
        locals.var_jssw_dn8 = assign15930_e10818_d_n8;
        locals.var_jssw_dn9 = assign15930_e10818_d_n9;
        locals.var_jssw_dn10 = assign15930_e10818_d_n10;
        locals.var_jssw_dn11 = assign15930_e10818_d_n11;
        locals.var_jssw_dn14 = assign15930_e10818_d_n14;

        let (assign15940_e10837, assign15940_e10837_d_n0, assign15940_e10837_d_n2, assign15940_e10837_d_n4, assign15940_e10837_d_n5, assign15940_e10837_d_n6, assign15940_e10837_d_n7, assign15940_e10837_d_n8, assign15940_e10837_d_n9, assign15940_e10837_d_n10, assign15940_e10837_d_n11, assign15940_e10837_d_n14,) = {
    if (locals.var_guard293 != 0.0) {
        let assign15940_e10823: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign15940_e10826: f64 = (locals.var_eg * locals.var_beta);
        let assign15940_e10827: f64 = (assign15940_e10823 - assign15940_e10826);
        let assign15940_e10830: f64 = (p.p522 * locals.var_log_tratio);
        let assign15940_e10831: f64 = (assign15940_e10827 + assign15940_e10830);
        let assign15940_e10833: f64 = (assign15940_e10831 / p.p521);
        let assign15940_e10834: f64 = (assign15940_e10833).exp();
        let assign15940_e10835: f64 = (p.p518 * assign15940_e10834);
        (assign15940_e10835, (p.p518 * (assign15940_e10834 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p522 * locals.var_log_tratio_dn0)) / p.p521))), (p.p518 * (assign15940_e10834 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p522 * locals.var_log_tratio_dn2)) / p.p521))), (p.p518 * (assign15940_e10834 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p522 * locals.var_log_tratio_dn4)) / p.p521))), (p.p518 * (assign15940_e10834 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p522 * locals.var_log_tratio_dn5)) / p.p521))), (p.p518 * (assign15940_e10834 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p522 * locals.var_log_tratio_dn6)) / p.p521))), (p.p518 * (assign15940_e10834 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p522 * locals.var_log_tratio_dn7)) / p.p521))), (p.p518 * (assign15940_e10834 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p522 * locals.var_log_tratio_dn8)) / p.p521))), (p.p518 * (assign15940_e10834 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p522 * locals.var_log_tratio_dn9)) / p.p521))), (p.p518 * (assign15940_e10834 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p522 * locals.var_log_tratio_dn10)) / p.p521))), (p.p518 * (assign15940_e10834 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p522 * locals.var_log_tratio_dn11)) / p.p521))), (p.p518 * (assign15940_e10834 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p522 * locals.var_log_tratio_dn14)) / p.p521))),)
    } else {
        (locals.var_jsswg, locals.var_jsswg_dn0, locals.var_jsswg_dn2, locals.var_jsswg_dn4, locals.var_jsswg_dn5, locals.var_jsswg_dn6, locals.var_jsswg_dn7, locals.var_jsswg_dn8, locals.var_jsswg_dn9, locals.var_jsswg_dn10, locals.var_jsswg_dn11, locals.var_jsswg_dn14,)
    }
};
        locals.var_jsswg = assign15940_e10837;
        locals.var_jsswg_dn0 = assign15940_e10837_d_n0;
        locals.var_jsswg_dn2 = assign15940_e10837_d_n2;
        locals.var_jsswg_dn4 = assign15940_e10837_d_n4;
        locals.var_jsswg_dn5 = assign15940_e10837_d_n5;
        locals.var_jsswg_dn6 = assign15940_e10837_d_n6;
        locals.var_jsswg_dn7 = assign15940_e10837_d_n7;
        locals.var_jsswg_dn8 = assign15940_e10837_d_n8;
        locals.var_jsswg_dn9 = assign15940_e10837_d_n9;
        locals.var_jsswg_dn10 = assign15940_e10837_d_n10;
        locals.var_jsswg_dn11 = assign15940_e10837_d_n11;
        locals.var_jsswg_dn14 = assign15940_e10837_d_n14;

        let (assign15950_e10856, assign15950_e10856_d_n0, assign15950_e10856_d_n2, assign15950_e10856_d_n4, assign15950_e10856_d_n5, assign15950_e10856_d_n6, assign15950_e10856_d_n7, assign15950_e10856_d_n8, assign15950_e10856_d_n9, assign15950_e10856_d_n10, assign15950_e10856_d_n11, assign15950_e10856_d_n14,) = {
    if (locals.var_guard293 != 0.0) {
        let assign15950_e10842: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign15950_e10845: f64 = (locals.var_eg * locals.var_beta);
        let assign15950_e10846: f64 = (assign15950_e10842 - assign15950_e10845);
        let assign15950_e10849: f64 = (p.p532 * locals.var_log_tratio);
        let assign15950_e10850: f64 = (assign15950_e10846 + assign15950_e10849);
        let assign15950_e10852: f64 = (assign15950_e10850 / locals.var_uc_njs);
        let assign15950_e10853: f64 = (assign15950_e10852).exp();
        let assign15950_e10854: f64 = (locals.var_uc_js0s * assign15950_e10853);
        (assign15950_e10854, (locals.var_uc_js0s * (assign15950_e10853 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p532 * locals.var_log_tratio_dn0)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15950_e10853 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p532 * locals.var_log_tratio_dn2)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15950_e10853 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p532 * locals.var_log_tratio_dn4)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15950_e10853 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p532 * locals.var_log_tratio_dn5)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15950_e10853 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p532 * locals.var_log_tratio_dn6)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15950_e10853 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p532 * locals.var_log_tratio_dn7)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15950_e10853 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p532 * locals.var_log_tratio_dn8)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15950_e10853 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p532 * locals.var_log_tratio_dn9)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15950_e10853 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p532 * locals.var_log_tratio_dn10)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15950_e10853 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p532 * locals.var_log_tratio_dn11)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign15950_e10853 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p532 * locals.var_log_tratio_dn14)) / locals.var_uc_njs))),)
    } else {
        (locals.var_js2, locals.var_js2_dn0, locals.var_js2_dn2, locals.var_js2_dn4, locals.var_js2_dn5, locals.var_js2_dn6, locals.var_js2_dn7, locals.var_js2_dn8, locals.var_js2_dn9, locals.var_js2_dn10, locals.var_js2_dn11, locals.var_js2_dn14,)
    }
};
        locals.var_js2 = assign15950_e10856;
        locals.var_js2_dn0 = assign15950_e10856_d_n0;
        locals.var_js2_dn2 = assign15950_e10856_d_n2;
        locals.var_js2_dn4 = assign15950_e10856_d_n4;
        locals.var_js2_dn5 = assign15950_e10856_d_n5;
        locals.var_js2_dn6 = assign15950_e10856_d_n6;
        locals.var_js2_dn7 = assign15950_e10856_d_n7;
        locals.var_js2_dn8 = assign15950_e10856_d_n8;
        locals.var_js2_dn9 = assign15950_e10856_d_n9;
        locals.var_js2_dn10 = assign15950_e10856_d_n10;
        locals.var_js2_dn11 = assign15950_e10856_d_n11;
        locals.var_js2_dn14 = assign15950_e10856_d_n14;

        let (assign15960_e10875, assign15960_e10875_d_n0, assign15960_e10875_d_n2, assign15960_e10875_d_n4, assign15960_e10875_d_n5, assign15960_e10875_d_n6, assign15960_e10875_d_n7, assign15960_e10875_d_n8, assign15960_e10875_d_n9, assign15960_e10875_d_n10, assign15960_e10875_d_n11, assign15960_e10875_d_n14,) = {
    if (locals.var_guard293 != 0.0) {
        let assign15960_e10861: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign15960_e10864: f64 = (locals.var_eg * locals.var_beta);
        let assign15960_e10865: f64 = (assign15960_e10861 - assign15960_e10864);
        let assign15960_e10868: f64 = (p.p532 * locals.var_log_tratio);
        let assign15960_e10869: f64 = (assign15960_e10865 + assign15960_e10868);
        let assign15960_e10871: f64 = (assign15960_e10869 / p.p520);
        let assign15960_e10872: f64 = (assign15960_e10871).exp();
        let assign15960_e10873: f64 = (locals.var_uc_js0sws * assign15960_e10872);
        (assign15960_e10873, (locals.var_uc_js0sws * (assign15960_e10872 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p532 * locals.var_log_tratio_dn0)) / p.p520))), (locals.var_uc_js0sws * (assign15960_e10872 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p532 * locals.var_log_tratio_dn2)) / p.p520))), (locals.var_uc_js0sws * (assign15960_e10872 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p532 * locals.var_log_tratio_dn4)) / p.p520))), (locals.var_uc_js0sws * (assign15960_e10872 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p532 * locals.var_log_tratio_dn5)) / p.p520))), (locals.var_uc_js0sws * (assign15960_e10872 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p532 * locals.var_log_tratio_dn6)) / p.p520))), (locals.var_uc_js0sws * (assign15960_e10872 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p532 * locals.var_log_tratio_dn7)) / p.p520))), (locals.var_uc_js0sws * (assign15960_e10872 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p532 * locals.var_log_tratio_dn8)) / p.p520))), (locals.var_uc_js0sws * (assign15960_e10872 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p532 * locals.var_log_tratio_dn9)) / p.p520))), (locals.var_uc_js0sws * (assign15960_e10872 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p532 * locals.var_log_tratio_dn10)) / p.p520))), (locals.var_uc_js0sws * (assign15960_e10872 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p532 * locals.var_log_tratio_dn11)) / p.p520))), (locals.var_uc_js0sws * (assign15960_e10872 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p532 * locals.var_log_tratio_dn14)) / p.p520))),)
    } else {
        (locals.var_jssw2, locals.var_jssw2_dn0, locals.var_jssw2_dn2, locals.var_jssw2_dn4, locals.var_jssw2_dn5, locals.var_jssw2_dn6, locals.var_jssw2_dn7, locals.var_jssw2_dn8, locals.var_jssw2_dn9, locals.var_jssw2_dn10, locals.var_jssw2_dn11, locals.var_jssw2_dn14,)
    }
};
        locals.var_jssw2 = assign15960_e10875;
        locals.var_jssw2_dn0 = assign15960_e10875_d_n0;
        locals.var_jssw2_dn2 = assign15960_e10875_d_n2;
        locals.var_jssw2_dn4 = assign15960_e10875_d_n4;
        locals.var_jssw2_dn5 = assign15960_e10875_d_n5;
        locals.var_jssw2_dn6 = assign15960_e10875_d_n6;
        locals.var_jssw2_dn7 = assign15960_e10875_d_n7;
        locals.var_jssw2_dn8 = assign15960_e10875_d_n8;
        locals.var_jssw2_dn9 = assign15960_e10875_d_n9;
        locals.var_jssw2_dn10 = assign15960_e10875_d_n10;
        locals.var_jssw2_dn11 = assign15960_e10875_d_n11;
        locals.var_jssw2_dn14 = assign15960_e10875_d_n14;

        let (assign15970_e10894, assign15970_e10894_d_n0, assign15970_e10894_d_n2, assign15970_e10894_d_n4, assign15970_e10894_d_n5, assign15970_e10894_d_n6, assign15970_e10894_d_n7, assign15970_e10894_d_n8, assign15970_e10894_d_n9, assign15970_e10894_d_n10, assign15970_e10894_d_n11, assign15970_e10894_d_n14,) = {
    if (locals.var_guard293 != 0.0) {
        let assign15970_e10880: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign15970_e10883: f64 = (locals.var_eg * locals.var_beta);
        let assign15970_e10884: f64 = (assign15970_e10880 - assign15970_e10883);
        let assign15970_e10887: f64 = (p.p532 * locals.var_log_tratio);
        let assign15970_e10888: f64 = (assign15970_e10884 + assign15970_e10887);
        let assign15970_e10890: f64 = (assign15970_e10888 / p.p521);
        let assign15970_e10891: f64 = (assign15970_e10890).exp();
        let assign15970_e10892: f64 = (p.p518 * assign15970_e10891);
        (assign15970_e10892, (p.p518 * (assign15970_e10891 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p532 * locals.var_log_tratio_dn0)) / p.p521))), (p.p518 * (assign15970_e10891 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p532 * locals.var_log_tratio_dn2)) / p.p521))), (p.p518 * (assign15970_e10891 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p532 * locals.var_log_tratio_dn4)) / p.p521))), (p.p518 * (assign15970_e10891 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p532 * locals.var_log_tratio_dn5)) / p.p521))), (p.p518 * (assign15970_e10891 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p532 * locals.var_log_tratio_dn6)) / p.p521))), (p.p518 * (assign15970_e10891 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p532 * locals.var_log_tratio_dn7)) / p.p521))), (p.p518 * (assign15970_e10891 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p532 * locals.var_log_tratio_dn8)) / p.p521))), (p.p518 * (assign15970_e10891 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p532 * locals.var_log_tratio_dn9)) / p.p521))), (p.p518 * (assign15970_e10891 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p532 * locals.var_log_tratio_dn10)) / p.p521))), (p.p518 * (assign15970_e10891 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p532 * locals.var_log_tratio_dn11)) / p.p521))), (p.p518 * (assign15970_e10891 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p532 * locals.var_log_tratio_dn14)) / p.p521))),)
    } else {
        (locals.var_jsswg2, locals.var_jsswg2_dn0, locals.var_jsswg2_dn2, locals.var_jsswg2_dn4, locals.var_jsswg2_dn5, locals.var_jsswg2_dn6, locals.var_jsswg2_dn7, locals.var_jsswg2_dn8, locals.var_jsswg2_dn9, locals.var_jsswg2_dn10, locals.var_jsswg2_dn11, locals.var_jsswg2_dn14,)
    }
};
        locals.var_jsswg2 = assign15970_e10894;
        locals.var_jsswg2_dn0 = assign15970_e10894_d_n0;
        locals.var_jsswg2_dn2 = assign15970_e10894_d_n2;
        locals.var_jsswg2_dn4 = assign15970_e10894_d_n4;
        locals.var_jsswg2_dn5 = assign15970_e10894_d_n5;
        locals.var_jsswg2_dn6 = assign15970_e10894_d_n6;
        locals.var_jsswg2_dn7 = assign15970_e10894_d_n7;
        locals.var_jsswg2_dn8 = assign15970_e10894_d_n8;
        locals.var_jsswg2_dn9 = assign15970_e10894_d_n9;
        locals.var_jsswg2_dn10 = assign15970_e10894_d_n10;
        locals.var_jsswg2_dn11 = assign15970_e10894_d_n11;
        locals.var_jsswg2_dn14 = assign15970_e10894_d_n14;

        let assign15980_e10897: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard336 = assign15980_e10897;

        let assign15990_e10900: f64 = if p.p16 > locals.var_weff_nf { 1.0 } else { 0.0 };
        locals.var_guard337 = assign15990_e10900;

        let (assign16000_e10910, assign16000_e10910_d_n0, assign16000_e10910_d_n2, assign16000_e10910_d_n4, assign16000_e10910_d_n5, assign16000_e10910_d_n6, assign16000_e10910_d_n7, assign16000_e10910_d_n8, assign16000_e10910_d_n9, assign16000_e10910_d_n10, assign16000_e10910_d_n11, assign16000_e10910_d_n14,) = {
    if (((locals.var_guard293 != 0.0) && (locals.var_guard336 != 0.0)) && (locals.var_guard337 != 0.0)) {
        let assign16000_e10908: f64 = (p.p14 * locals.var_js);
        (assign16000_e10908, (p.p14 * locals.var_js_dn0), (p.p14 * locals.var_js_dn2), (p.p14 * locals.var_js_dn4), (p.p14 * locals.var_js_dn5), (p.p14 * locals.var_js_dn6), (p.p14 * locals.var_js_dn7), (p.p14 * locals.var_js_dn8), (p.p14 * locals.var_js_dn9), (p.p14 * locals.var_js_dn10), (p.p14 * locals.var_js_dn11), (p.p14 * locals.var_js_dn14),)
    } else {
        (locals.var_isbs_btm, locals.var_isbs_btm_dn0, locals.var_isbs_btm_dn2, locals.var_isbs_btm_dn4, locals.var_isbs_btm_dn5, locals.var_isbs_btm_dn6, locals.var_isbs_btm_dn7, locals.var_isbs_btm_dn8, locals.var_isbs_btm_dn9, locals.var_isbs_btm_dn10, locals.var_isbs_btm_dn11, locals.var_isbs_btm_dn14,)
    }
};
        locals.var_isbs_btm = assign16000_e10910;
        locals.var_isbs_btm_dn0 = assign16000_e10910_d_n0;
        locals.var_isbs_btm_dn2 = assign16000_e10910_d_n2;
        locals.var_isbs_btm_dn4 = assign16000_e10910_d_n4;
        locals.var_isbs_btm_dn5 = assign16000_e10910_d_n5;
        locals.var_isbs_btm_dn6 = assign16000_e10910_d_n6;
        locals.var_isbs_btm_dn7 = assign16000_e10910_d_n7;
        locals.var_isbs_btm_dn8 = assign16000_e10910_d_n8;
        locals.var_isbs_btm_dn9 = assign16000_e10910_d_n9;
        locals.var_isbs_btm_dn10 = assign16000_e10910_d_n10;
        locals.var_isbs_btm_dn11 = assign16000_e10910_d_n11;
        locals.var_isbs_btm_dn14 = assign16000_e10910_d_n14;

        let (assign16010_e10920, assign16010_e10920_d_n0, assign16010_e10920_d_n2, assign16010_e10920_d_n4, assign16010_e10920_d_n5, assign16010_e10920_d_n6, assign16010_e10920_d_n7, assign16010_e10920_d_n8, assign16010_e10920_d_n9, assign16010_e10920_d_n10, assign16010_e10920_d_n11, assign16010_e10920_d_n14,) = {
    if (((locals.var_guard293 != 0.0) && (locals.var_guard336 != 0.0)) && (locals.var_guard337 != 0.0)) {
        let assign16010_e10918: f64 = (p.p14 * locals.var_js2);
        (assign16010_e10918, (p.p14 * locals.var_js2_dn0), (p.p14 * locals.var_js2_dn2), (p.p14 * locals.var_js2_dn4), (p.p14 * locals.var_js2_dn5), (p.p14 * locals.var_js2_dn6), (p.p14 * locals.var_js2_dn7), (p.p14 * locals.var_js2_dn8), (p.p14 * locals.var_js2_dn9), (p.p14 * locals.var_js2_dn10), (p.p14 * locals.var_js2_dn11), (p.p14 * locals.var_js2_dn14),)
    } else {
        (locals.var_isbs2_btm, locals.var_isbs2_btm_dn0, locals.var_isbs2_btm_dn2, locals.var_isbs2_btm_dn4, locals.var_isbs2_btm_dn5, locals.var_isbs2_btm_dn6, locals.var_isbs2_btm_dn7, locals.var_isbs2_btm_dn8, locals.var_isbs2_btm_dn9, locals.var_isbs2_btm_dn10, locals.var_isbs2_btm_dn11, locals.var_isbs2_btm_dn14,)
    }
};
        locals.var_isbs2_btm = assign16010_e10920;
        locals.var_isbs2_btm_dn0 = assign16010_e10920_d_n0;
        locals.var_isbs2_btm_dn2 = assign16010_e10920_d_n2;
        locals.var_isbs2_btm_dn4 = assign16010_e10920_d_n4;
        locals.var_isbs2_btm_dn5 = assign16010_e10920_d_n5;
        locals.var_isbs2_btm_dn6 = assign16010_e10920_d_n6;
        locals.var_isbs2_btm_dn7 = assign16010_e10920_d_n7;
        locals.var_isbs2_btm_dn8 = assign16010_e10920_d_n8;
        locals.var_isbs2_btm_dn9 = assign16010_e10920_d_n9;
        locals.var_isbs2_btm_dn10 = assign16010_e10920_d_n10;
        locals.var_isbs2_btm_dn11 = assign16010_e10920_d_n11;
        locals.var_isbs2_btm_dn14 = assign16010_e10920_d_n14;

        let (assign16020_e10932, assign16020_e10932_d_n0, assign16020_e10932_d_n2, assign16020_e10932_d_n4, assign16020_e10932_d_n5, assign16020_e10932_d_n6, assign16020_e10932_d_n7, assign16020_e10932_d_n8, assign16020_e10932_d_n9, assign16020_e10932_d_n10, assign16020_e10932_d_n11, assign16020_e10932_d_n14,) = {
    if (((locals.var_guard293 != 0.0) && (locals.var_guard336 != 0.0)) && (locals.var_guard337 != 0.0)) {
        let assign16020_e10928: f64 = (p.p16 - locals.var_weff_nf);
        let assign16020_e10930: f64 = (assign16020_e10928 * locals.var_jssw);
        (assign16020_e10930, (assign16020_e10928 * locals.var_jssw_dn0), (assign16020_e10928 * locals.var_jssw_dn2), (assign16020_e10928 * locals.var_jssw_dn4), (assign16020_e10928 * locals.var_jssw_dn5), (assign16020_e10928 * locals.var_jssw_dn6), (assign16020_e10928 * locals.var_jssw_dn7), (assign16020_e10928 * locals.var_jssw_dn8), (assign16020_e10928 * locals.var_jssw_dn9), (assign16020_e10928 * locals.var_jssw_dn10), (assign16020_e10928 * locals.var_jssw_dn11), (assign16020_e10928 * locals.var_jssw_dn14),)
    } else {
        (locals.var_isbs_sws, locals.var_isbs_sws_dn0, locals.var_isbs_sws_dn2, locals.var_isbs_sws_dn4, locals.var_isbs_sws_dn5, locals.var_isbs_sws_dn6, locals.var_isbs_sws_dn7, locals.var_isbs_sws_dn8, locals.var_isbs_sws_dn9, locals.var_isbs_sws_dn10, locals.var_isbs_sws_dn11, locals.var_isbs_sws_dn14,)
    }
};
        locals.var_isbs_sws = assign16020_e10932;
        locals.var_isbs_sws_dn0 = assign16020_e10932_d_n0;
        locals.var_isbs_sws_dn2 = assign16020_e10932_d_n2;
        locals.var_isbs_sws_dn4 = assign16020_e10932_d_n4;
        locals.var_isbs_sws_dn5 = assign16020_e10932_d_n5;
        locals.var_isbs_sws_dn6 = assign16020_e10932_d_n6;
        locals.var_isbs_sws_dn7 = assign16020_e10932_d_n7;
        locals.var_isbs_sws_dn8 = assign16020_e10932_d_n8;
        locals.var_isbs_sws_dn9 = assign16020_e10932_d_n9;
        locals.var_isbs_sws_dn10 = assign16020_e10932_d_n10;
        locals.var_isbs_sws_dn11 = assign16020_e10932_d_n11;
        locals.var_isbs_sws_dn14 = assign16020_e10932_d_n14;

        let (assign16030_e10944, assign16030_e10944_d_n0, assign16030_e10944_d_n2, assign16030_e10944_d_n4, assign16030_e10944_d_n5, assign16030_e10944_d_n6, assign16030_e10944_d_n7, assign16030_e10944_d_n8, assign16030_e10944_d_n9, assign16030_e10944_d_n10, assign16030_e10944_d_n11, assign16030_e10944_d_n14,) = {
    if (((locals.var_guard293 != 0.0) && (locals.var_guard336 != 0.0)) && (locals.var_guard337 != 0.0)) {
        let assign16030_e10940: f64 = (p.p16 - locals.var_weff_nf);
        let assign16030_e10942: f64 = (assign16030_e10940 * locals.var_jssw2);
        (assign16030_e10942, (assign16030_e10940 * locals.var_jssw2_dn0), (assign16030_e10940 * locals.var_jssw2_dn2), (assign16030_e10940 * locals.var_jssw2_dn4), (assign16030_e10940 * locals.var_jssw2_dn5), (assign16030_e10940 * locals.var_jssw2_dn6), (assign16030_e10940 * locals.var_jssw2_dn7), (assign16030_e10940 * locals.var_jssw2_dn8), (assign16030_e10940 * locals.var_jssw2_dn9), (assign16030_e10940 * locals.var_jssw2_dn10), (assign16030_e10940 * locals.var_jssw2_dn11), (assign16030_e10940 * locals.var_jssw2_dn14),)
    } else {
        (locals.var_isbs2_sws, locals.var_isbs2_sws_dn0, locals.var_isbs2_sws_dn2, locals.var_isbs2_sws_dn4, locals.var_isbs2_sws_dn5, locals.var_isbs2_sws_dn6, locals.var_isbs2_sws_dn7, locals.var_isbs2_sws_dn8, locals.var_isbs2_sws_dn9, locals.var_isbs2_sws_dn10, locals.var_isbs2_sws_dn11, locals.var_isbs2_sws_dn14,)
    }
};
        locals.var_isbs2_sws = assign16030_e10944;
        locals.var_isbs2_sws_dn0 = assign16030_e10944_d_n0;
        locals.var_isbs2_sws_dn2 = assign16030_e10944_d_n2;
        locals.var_isbs2_sws_dn4 = assign16030_e10944_d_n4;
        locals.var_isbs2_sws_dn5 = assign16030_e10944_d_n5;
        locals.var_isbs2_sws_dn6 = assign16030_e10944_d_n6;
        locals.var_isbs2_sws_dn7 = assign16030_e10944_d_n7;
        locals.var_isbs2_sws_dn8 = assign16030_e10944_d_n8;
        locals.var_isbs2_sws_dn9 = assign16030_e10944_d_n9;
        locals.var_isbs2_sws_dn10 = assign16030_e10944_d_n10;
        locals.var_isbs2_sws_dn11 = assign16030_e10944_d_n11;
        locals.var_isbs2_sws_dn14 = assign16030_e10944_d_n14;

        let (assign16040_e10954, assign16040_e10954_d_n0, assign16040_e10954_d_n2, assign16040_e10954_d_n4, assign16040_e10954_d_n5, assign16040_e10954_d_n6, assign16040_e10954_d_n7, assign16040_e10954_d_n8, assign16040_e10954_d_n9, assign16040_e10954_d_n10, assign16040_e10954_d_n11, assign16040_e10954_d_n14,) = {
    if (((locals.var_guard293 != 0.0) && (locals.var_guard336 != 0.0)) && (locals.var_guard337 != 0.0)) {
        let assign16040_e10952: f64 = (locals.var_weff_nf * locals.var_jsswg);
        (assign16040_e10952, (locals.var_weff_nf * locals.var_jsswg_dn0), (locals.var_weff_nf * locals.var_jsswg_dn2), (locals.var_weff_nf * locals.var_jsswg_dn4), (locals.var_weff_nf * locals.var_jsswg_dn5), (locals.var_weff_nf * locals.var_jsswg_dn6), (locals.var_weff_nf * locals.var_jsswg_dn7), (locals.var_weff_nf * locals.var_jsswg_dn8), (locals.var_weff_nf * locals.var_jsswg_dn9), (locals.var_weff_nf * locals.var_jsswg_dn10), (locals.var_weff_nf * locals.var_jsswg_dn11), (locals.var_weff_nf * locals.var_jsswg_dn14),)
    } else {
        (locals.var_isbs_swg, locals.var_isbs_swg_dn0, locals.var_isbs_swg_dn2, locals.var_isbs_swg_dn4, locals.var_isbs_swg_dn5, locals.var_isbs_swg_dn6, locals.var_isbs_swg_dn7, locals.var_isbs_swg_dn8, locals.var_isbs_swg_dn9, locals.var_isbs_swg_dn10, locals.var_isbs_swg_dn11, locals.var_isbs_swg_dn14,)
    }
};
        locals.var_isbs_swg = assign16040_e10954;
        locals.var_isbs_swg_dn0 = assign16040_e10954_d_n0;
        locals.var_isbs_swg_dn2 = assign16040_e10954_d_n2;
        locals.var_isbs_swg_dn4 = assign16040_e10954_d_n4;
        locals.var_isbs_swg_dn5 = assign16040_e10954_d_n5;
        locals.var_isbs_swg_dn6 = assign16040_e10954_d_n6;
        locals.var_isbs_swg_dn7 = assign16040_e10954_d_n7;
        locals.var_isbs_swg_dn8 = assign16040_e10954_d_n8;
        locals.var_isbs_swg_dn9 = assign16040_e10954_d_n9;
        locals.var_isbs_swg_dn10 = assign16040_e10954_d_n10;
        locals.var_isbs_swg_dn11 = assign16040_e10954_d_n11;
        locals.var_isbs_swg_dn14 = assign16040_e10954_d_n14;

        let (assign16050_e10964, assign16050_e10964_d_n0, assign16050_e10964_d_n2, assign16050_e10964_d_n4, assign16050_e10964_d_n5, assign16050_e10964_d_n6, assign16050_e10964_d_n7, assign16050_e10964_d_n8, assign16050_e10964_d_n9, assign16050_e10964_d_n10, assign16050_e10964_d_n11, assign16050_e10964_d_n14,) = {
    if (((locals.var_guard293 != 0.0) && (locals.var_guard336 != 0.0)) && (locals.var_guard337 != 0.0)) {
        let assign16050_e10962: f64 = (locals.var_weff_nf * locals.var_jsswg2);
        (assign16050_e10962, (locals.var_weff_nf * locals.var_jsswg2_dn0), (locals.var_weff_nf * locals.var_jsswg2_dn2), (locals.var_weff_nf * locals.var_jsswg2_dn4), (locals.var_weff_nf * locals.var_jsswg2_dn5), (locals.var_weff_nf * locals.var_jsswg2_dn6), (locals.var_weff_nf * locals.var_jsswg2_dn7), (locals.var_weff_nf * locals.var_jsswg2_dn8), (locals.var_weff_nf * locals.var_jsswg2_dn9), (locals.var_weff_nf * locals.var_jsswg2_dn10), (locals.var_weff_nf * locals.var_jsswg2_dn11), (locals.var_weff_nf * locals.var_jsswg2_dn14),)
    } else {
        (locals.var_isbs2_swg, locals.var_isbs2_swg_dn0, locals.var_isbs2_swg_dn2, locals.var_isbs2_swg_dn4, locals.var_isbs2_swg_dn5, locals.var_isbs2_swg_dn6, locals.var_isbs2_swg_dn7, locals.var_isbs2_swg_dn8, locals.var_isbs2_swg_dn9, locals.var_isbs2_swg_dn10, locals.var_isbs2_swg_dn11, locals.var_isbs2_swg_dn14,)
    }
};
        locals.var_isbs2_swg = assign16050_e10964;
        locals.var_isbs2_swg_dn0 = assign16050_e10964_d_n0;
        locals.var_isbs2_swg_dn2 = assign16050_e10964_d_n2;
        locals.var_isbs2_swg_dn4 = assign16050_e10964_d_n4;
        locals.var_isbs2_swg_dn5 = assign16050_e10964_d_n5;
        locals.var_isbs2_swg_dn6 = assign16050_e10964_d_n6;
        locals.var_isbs2_swg_dn7 = assign16050_e10964_d_n7;
        locals.var_isbs2_swg_dn8 = assign16050_e10964_d_n8;
        locals.var_isbs2_swg_dn9 = assign16050_e10964_d_n9;
        locals.var_isbs2_swg_dn10 = assign16050_e10964_d_n10;
        locals.var_isbs2_swg_dn11 = assign16050_e10964_d_n11;
        locals.var_isbs2_swg_dn14 = assign16050_e10964_d_n14;

        let (assign16060_e10975, assign16060_e10975_d_n0, assign16060_e10975_d_n2, assign16060_e10975_d_n4, assign16060_e10975_d_n5, assign16060_e10975_d_n6, assign16060_e10975_d_n7, assign16060_e10975_d_n8, assign16060_e10975_d_n9, assign16060_e10975_d_n10, assign16060_e10975_d_n11, assign16060_e10975_d_n14,) = {
    if (((locals.var_guard293 != 0.0) && (locals.var_guard336 != 0.0)) && (locals.var_guard337 == 0.0)) {
        let assign16060_e10973: f64 = (p.p14 * locals.var_js);
        (assign16060_e10973, (p.p14 * locals.var_js_dn0), (p.p14 * locals.var_js_dn2), (p.p14 * locals.var_js_dn4), (p.p14 * locals.var_js_dn5), (p.p14 * locals.var_js_dn6), (p.p14 * locals.var_js_dn7), (p.p14 * locals.var_js_dn8), (p.p14 * locals.var_js_dn9), (p.p14 * locals.var_js_dn10), (p.p14 * locals.var_js_dn11), (p.p14 * locals.var_js_dn14),)
    } else {
        (locals.var_isbs_btm, locals.var_isbs_btm_dn0, locals.var_isbs_btm_dn2, locals.var_isbs_btm_dn4, locals.var_isbs_btm_dn5, locals.var_isbs_btm_dn6, locals.var_isbs_btm_dn7, locals.var_isbs_btm_dn8, locals.var_isbs_btm_dn9, locals.var_isbs_btm_dn10, locals.var_isbs_btm_dn11, locals.var_isbs_btm_dn14,)
    }
};
        locals.var_isbs_btm = assign16060_e10975;
        locals.var_isbs_btm_dn0 = assign16060_e10975_d_n0;
        locals.var_isbs_btm_dn2 = assign16060_e10975_d_n2;
        locals.var_isbs_btm_dn4 = assign16060_e10975_d_n4;
        locals.var_isbs_btm_dn5 = assign16060_e10975_d_n5;
        locals.var_isbs_btm_dn6 = assign16060_e10975_d_n6;
        locals.var_isbs_btm_dn7 = assign16060_e10975_d_n7;
        locals.var_isbs_btm_dn8 = assign16060_e10975_d_n8;
        locals.var_isbs_btm_dn9 = assign16060_e10975_d_n9;
        locals.var_isbs_btm_dn10 = assign16060_e10975_d_n10;
        locals.var_isbs_btm_dn11 = assign16060_e10975_d_n11;
        locals.var_isbs_btm_dn14 = assign16060_e10975_d_n14;

        let (assign16070_e10986, assign16070_e10986_d_n0, assign16070_e10986_d_n2, assign16070_e10986_d_n4, assign16070_e10986_d_n5, assign16070_e10986_d_n6, assign16070_e10986_d_n7, assign16070_e10986_d_n8, assign16070_e10986_d_n9, assign16070_e10986_d_n10, assign16070_e10986_d_n11, assign16070_e10986_d_n14,) = {
    if (((locals.var_guard293 != 0.0) && (locals.var_guard336 != 0.0)) && (locals.var_guard337 == 0.0)) {
        let assign16070_e10984: f64 = (p.p14 * locals.var_js2);
        (assign16070_e10984, (p.p14 * locals.var_js2_dn0), (p.p14 * locals.var_js2_dn2), (p.p14 * locals.var_js2_dn4), (p.p14 * locals.var_js2_dn5), (p.p14 * locals.var_js2_dn6), (p.p14 * locals.var_js2_dn7), (p.p14 * locals.var_js2_dn8), (p.p14 * locals.var_js2_dn9), (p.p14 * locals.var_js2_dn10), (p.p14 * locals.var_js2_dn11), (p.p14 * locals.var_js2_dn14),)
    } else {
        (locals.var_isbs2_btm, locals.var_isbs2_btm_dn0, locals.var_isbs2_btm_dn2, locals.var_isbs2_btm_dn4, locals.var_isbs2_btm_dn5, locals.var_isbs2_btm_dn6, locals.var_isbs2_btm_dn7, locals.var_isbs2_btm_dn8, locals.var_isbs2_btm_dn9, locals.var_isbs2_btm_dn10, locals.var_isbs2_btm_dn11, locals.var_isbs2_btm_dn14,)
    }
};
        locals.var_isbs2_btm = assign16070_e10986;
        locals.var_isbs2_btm_dn0 = assign16070_e10986_d_n0;
        locals.var_isbs2_btm_dn2 = assign16070_e10986_d_n2;
        locals.var_isbs2_btm_dn4 = assign16070_e10986_d_n4;
        locals.var_isbs2_btm_dn5 = assign16070_e10986_d_n5;
        locals.var_isbs2_btm_dn6 = assign16070_e10986_d_n6;
        locals.var_isbs2_btm_dn7 = assign16070_e10986_d_n7;
        locals.var_isbs2_btm_dn8 = assign16070_e10986_d_n8;
        locals.var_isbs2_btm_dn9 = assign16070_e10986_d_n9;
        locals.var_isbs2_btm_dn10 = assign16070_e10986_d_n10;
        locals.var_isbs2_btm_dn11 = assign16070_e10986_d_n11;
        locals.var_isbs2_btm_dn14 = assign16070_e10986_d_n14;

        let (assign16080_e10995, assign16080_e10995_d_n0, assign16080_e10995_d_n2, assign16080_e10995_d_n4, assign16080_e10995_d_n5, assign16080_e10995_d_n6, assign16080_e10995_d_n7, assign16080_e10995_d_n8, assign16080_e10995_d_n9, assign16080_e10995_d_n10, assign16080_e10995_d_n11, assign16080_e10995_d_n14,) = {
    if (((locals.var_guard293 != 0.0) && (locals.var_guard336 != 0.0)) && (locals.var_guard337 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbs_sws, locals.var_isbs_sws_dn0, locals.var_isbs_sws_dn2, locals.var_isbs_sws_dn4, locals.var_isbs_sws_dn5, locals.var_isbs_sws_dn6, locals.var_isbs_sws_dn7, locals.var_isbs_sws_dn8, locals.var_isbs_sws_dn9, locals.var_isbs_sws_dn10, locals.var_isbs_sws_dn11, locals.var_isbs_sws_dn14,)
    }
};
        locals.var_isbs_sws = assign16080_e10995;
        locals.var_isbs_sws_dn0 = assign16080_e10995_d_n0;
        locals.var_isbs_sws_dn2 = assign16080_e10995_d_n2;
        locals.var_isbs_sws_dn4 = assign16080_e10995_d_n4;
        locals.var_isbs_sws_dn5 = assign16080_e10995_d_n5;
        locals.var_isbs_sws_dn6 = assign16080_e10995_d_n6;
        locals.var_isbs_sws_dn7 = assign16080_e10995_d_n7;
        locals.var_isbs_sws_dn8 = assign16080_e10995_d_n8;
        locals.var_isbs_sws_dn9 = assign16080_e10995_d_n9;
        locals.var_isbs_sws_dn10 = assign16080_e10995_d_n10;
        locals.var_isbs_sws_dn11 = assign16080_e10995_d_n11;
        locals.var_isbs_sws_dn14 = assign16080_e10995_d_n14;

        let (assign16090_e11004, assign16090_e11004_d_n0, assign16090_e11004_d_n2, assign16090_e11004_d_n4, assign16090_e11004_d_n5, assign16090_e11004_d_n6, assign16090_e11004_d_n7, assign16090_e11004_d_n8, assign16090_e11004_d_n9, assign16090_e11004_d_n10, assign16090_e11004_d_n11, assign16090_e11004_d_n14,) = {
    if (((locals.var_guard293 != 0.0) && (locals.var_guard336 != 0.0)) && (locals.var_guard337 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbs2_sws, locals.var_isbs2_sws_dn0, locals.var_isbs2_sws_dn2, locals.var_isbs2_sws_dn4, locals.var_isbs2_sws_dn5, locals.var_isbs2_sws_dn6, locals.var_isbs2_sws_dn7, locals.var_isbs2_sws_dn8, locals.var_isbs2_sws_dn9, locals.var_isbs2_sws_dn10, locals.var_isbs2_sws_dn11, locals.var_isbs2_sws_dn14,)
    }
};
        locals.var_isbs2_sws = assign16090_e11004;
        locals.var_isbs2_sws_dn0 = assign16090_e11004_d_n0;
        locals.var_isbs2_sws_dn2 = assign16090_e11004_d_n2;
        locals.var_isbs2_sws_dn4 = assign16090_e11004_d_n4;
        locals.var_isbs2_sws_dn5 = assign16090_e11004_d_n5;
        locals.var_isbs2_sws_dn6 = assign16090_e11004_d_n6;
        locals.var_isbs2_sws_dn7 = assign16090_e11004_d_n7;
        locals.var_isbs2_sws_dn8 = assign16090_e11004_d_n8;
        locals.var_isbs2_sws_dn9 = assign16090_e11004_d_n9;
        locals.var_isbs2_sws_dn10 = assign16090_e11004_d_n10;
        locals.var_isbs2_sws_dn11 = assign16090_e11004_d_n11;
        locals.var_isbs2_sws_dn14 = assign16090_e11004_d_n14;

        let (assign16100_e11015, assign16100_e11015_d_n0, assign16100_e11015_d_n2, assign16100_e11015_d_n4, assign16100_e11015_d_n5, assign16100_e11015_d_n6, assign16100_e11015_d_n7, assign16100_e11015_d_n8, assign16100_e11015_d_n9, assign16100_e11015_d_n10, assign16100_e11015_d_n11, assign16100_e11015_d_n14,) = {
    if (((locals.var_guard293 != 0.0) && (locals.var_guard336 != 0.0)) && (locals.var_guard337 == 0.0)) {
        let assign16100_e11013: f64 = (p.p16 * locals.var_jsswg);
        (assign16100_e11013, (p.p16 * locals.var_jsswg_dn0), (p.p16 * locals.var_jsswg_dn2), (p.p16 * locals.var_jsswg_dn4), (p.p16 * locals.var_jsswg_dn5), (p.p16 * locals.var_jsswg_dn6), (p.p16 * locals.var_jsswg_dn7), (p.p16 * locals.var_jsswg_dn8), (p.p16 * locals.var_jsswg_dn9), (p.p16 * locals.var_jsswg_dn10), (p.p16 * locals.var_jsswg_dn11), (p.p16 * locals.var_jsswg_dn14),)
    } else {
        (locals.var_isbs_swg, locals.var_isbs_swg_dn0, locals.var_isbs_swg_dn2, locals.var_isbs_swg_dn4, locals.var_isbs_swg_dn5, locals.var_isbs_swg_dn6, locals.var_isbs_swg_dn7, locals.var_isbs_swg_dn8, locals.var_isbs_swg_dn9, locals.var_isbs_swg_dn10, locals.var_isbs_swg_dn11, locals.var_isbs_swg_dn14,)
    }
};
        locals.var_isbs_swg = assign16100_e11015;
        locals.var_isbs_swg_dn0 = assign16100_e11015_d_n0;
        locals.var_isbs_swg_dn2 = assign16100_e11015_d_n2;
        locals.var_isbs_swg_dn4 = assign16100_e11015_d_n4;
        locals.var_isbs_swg_dn5 = assign16100_e11015_d_n5;
        locals.var_isbs_swg_dn6 = assign16100_e11015_d_n6;
        locals.var_isbs_swg_dn7 = assign16100_e11015_d_n7;
        locals.var_isbs_swg_dn8 = assign16100_e11015_d_n8;
        locals.var_isbs_swg_dn9 = assign16100_e11015_d_n9;
        locals.var_isbs_swg_dn10 = assign16100_e11015_d_n10;
        locals.var_isbs_swg_dn11 = assign16100_e11015_d_n11;
        locals.var_isbs_swg_dn14 = assign16100_e11015_d_n14;

        let (assign16110_e11026, assign16110_e11026_d_n0, assign16110_e11026_d_n2, assign16110_e11026_d_n4, assign16110_e11026_d_n5, assign16110_e11026_d_n6, assign16110_e11026_d_n7, assign16110_e11026_d_n8, assign16110_e11026_d_n9, assign16110_e11026_d_n10, assign16110_e11026_d_n11, assign16110_e11026_d_n14,) = {
    if (((locals.var_guard293 != 0.0) && (locals.var_guard336 != 0.0)) && (locals.var_guard337 == 0.0)) {
        let assign16110_e11024: f64 = (p.p16 * locals.var_jsswg2);
        (assign16110_e11024, (p.p16 * locals.var_jsswg2_dn0), (p.p16 * locals.var_jsswg2_dn2), (p.p16 * locals.var_jsswg2_dn4), (p.p16 * locals.var_jsswg2_dn5), (p.p16 * locals.var_jsswg2_dn6), (p.p16 * locals.var_jsswg2_dn7), (p.p16 * locals.var_jsswg2_dn8), (p.p16 * locals.var_jsswg2_dn9), (p.p16 * locals.var_jsswg2_dn10), (p.p16 * locals.var_jsswg2_dn11), (p.p16 * locals.var_jsswg2_dn14),)
    } else {
        (locals.var_isbs2_swg, locals.var_isbs2_swg_dn0, locals.var_isbs2_swg_dn2, locals.var_isbs2_swg_dn4, locals.var_isbs2_swg_dn5, locals.var_isbs2_swg_dn6, locals.var_isbs2_swg_dn7, locals.var_isbs2_swg_dn8, locals.var_isbs2_swg_dn9, locals.var_isbs2_swg_dn10, locals.var_isbs2_swg_dn11, locals.var_isbs2_swg_dn14,)
    }
};
        locals.var_isbs2_swg = assign16110_e11026;
        locals.var_isbs2_swg_dn0 = assign16110_e11026_d_n0;
        locals.var_isbs2_swg_dn2 = assign16110_e11026_d_n2;
        locals.var_isbs2_swg_dn4 = assign16110_e11026_d_n4;
        locals.var_isbs2_swg_dn5 = assign16110_e11026_d_n5;
        locals.var_isbs2_swg_dn6 = assign16110_e11026_d_n6;
        locals.var_isbs2_swg_dn7 = assign16110_e11026_d_n7;
        locals.var_isbs2_swg_dn8 = assign16110_e11026_d_n8;
        locals.var_isbs2_swg_dn9 = assign16110_e11026_d_n9;
        locals.var_isbs2_swg_dn10 = assign16110_e11026_d_n10;
        locals.var_isbs2_swg_dn11 = assign16110_e11026_d_n11;
        locals.var_isbs2_swg_dn14 = assign16110_e11026_d_n14;

    }

    pub(super) fn stamp_transient_block_34(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign16120_e11035, assign16120_e11035_d_n0, assign16120_e11035_d_n2, assign16120_e11035_d_n4, assign16120_e11035_d_n5, assign16120_e11035_d_n6, assign16120_e11035_d_n7, assign16120_e11035_d_n8, assign16120_e11035_d_n9, assign16120_e11035_d_n10, assign16120_e11035_d_n11, assign16120_e11035_d_n14,) = {
    if ((locals.var_guard293 != 0.0) && (locals.var_guard336 == 0.0)) {
        let assign16120_e11033: f64 = (p.p14 * locals.var_js);
        (assign16120_e11033, (p.p14 * locals.var_js_dn0), (p.p14 * locals.var_js_dn2), (p.p14 * locals.var_js_dn4), (p.p14 * locals.var_js_dn5), (p.p14 * locals.var_js_dn6), (p.p14 * locals.var_js_dn7), (p.p14 * locals.var_js_dn8), (p.p14 * locals.var_js_dn9), (p.p14 * locals.var_js_dn10), (p.p14 * locals.var_js_dn11), (p.p14 * locals.var_js_dn14),)
    } else {
        (locals.var_isbs_btm, locals.var_isbs_btm_dn0, locals.var_isbs_btm_dn2, locals.var_isbs_btm_dn4, locals.var_isbs_btm_dn5, locals.var_isbs_btm_dn6, locals.var_isbs_btm_dn7, locals.var_isbs_btm_dn8, locals.var_isbs_btm_dn9, locals.var_isbs_btm_dn10, locals.var_isbs_btm_dn11, locals.var_isbs_btm_dn14,)
    }
};
        locals.var_isbs_btm = assign16120_e11035;
        locals.var_isbs_btm_dn0 = assign16120_e11035_d_n0;
        locals.var_isbs_btm_dn2 = assign16120_e11035_d_n2;
        locals.var_isbs_btm_dn4 = assign16120_e11035_d_n4;
        locals.var_isbs_btm_dn5 = assign16120_e11035_d_n5;
        locals.var_isbs_btm_dn6 = assign16120_e11035_d_n6;
        locals.var_isbs_btm_dn7 = assign16120_e11035_d_n7;
        locals.var_isbs_btm_dn8 = assign16120_e11035_d_n8;
        locals.var_isbs_btm_dn9 = assign16120_e11035_d_n9;
        locals.var_isbs_btm_dn10 = assign16120_e11035_d_n10;
        locals.var_isbs_btm_dn11 = assign16120_e11035_d_n11;
        locals.var_isbs_btm_dn14 = assign16120_e11035_d_n14;

        let (assign16130_e11044, assign16130_e11044_d_n0, assign16130_e11044_d_n2, assign16130_e11044_d_n4, assign16130_e11044_d_n5, assign16130_e11044_d_n6, assign16130_e11044_d_n7, assign16130_e11044_d_n8, assign16130_e11044_d_n9, assign16130_e11044_d_n10, assign16130_e11044_d_n11, assign16130_e11044_d_n14,) = {
    if ((locals.var_guard293 != 0.0) && (locals.var_guard336 == 0.0)) {
        let assign16130_e11042: f64 = (p.p14 * locals.var_js2);
        (assign16130_e11042, (p.p14 * locals.var_js2_dn0), (p.p14 * locals.var_js2_dn2), (p.p14 * locals.var_js2_dn4), (p.p14 * locals.var_js2_dn5), (p.p14 * locals.var_js2_dn6), (p.p14 * locals.var_js2_dn7), (p.p14 * locals.var_js2_dn8), (p.p14 * locals.var_js2_dn9), (p.p14 * locals.var_js2_dn10), (p.p14 * locals.var_js2_dn11), (p.p14 * locals.var_js2_dn14),)
    } else {
        (locals.var_isbs2_btm, locals.var_isbs2_btm_dn0, locals.var_isbs2_btm_dn2, locals.var_isbs2_btm_dn4, locals.var_isbs2_btm_dn5, locals.var_isbs2_btm_dn6, locals.var_isbs2_btm_dn7, locals.var_isbs2_btm_dn8, locals.var_isbs2_btm_dn9, locals.var_isbs2_btm_dn10, locals.var_isbs2_btm_dn11, locals.var_isbs2_btm_dn14,)
    }
};
        locals.var_isbs2_btm = assign16130_e11044;
        locals.var_isbs2_btm_dn0 = assign16130_e11044_d_n0;
        locals.var_isbs2_btm_dn2 = assign16130_e11044_d_n2;
        locals.var_isbs2_btm_dn4 = assign16130_e11044_d_n4;
        locals.var_isbs2_btm_dn5 = assign16130_e11044_d_n5;
        locals.var_isbs2_btm_dn6 = assign16130_e11044_d_n6;
        locals.var_isbs2_btm_dn7 = assign16130_e11044_d_n7;
        locals.var_isbs2_btm_dn8 = assign16130_e11044_d_n8;
        locals.var_isbs2_btm_dn9 = assign16130_e11044_d_n9;
        locals.var_isbs2_btm_dn10 = assign16130_e11044_d_n10;
        locals.var_isbs2_btm_dn11 = assign16130_e11044_d_n11;
        locals.var_isbs2_btm_dn14 = assign16130_e11044_d_n14;

        let (assign16140_e11053, assign16140_e11053_d_n0, assign16140_e11053_d_n2, assign16140_e11053_d_n4, assign16140_e11053_d_n5, assign16140_e11053_d_n6, assign16140_e11053_d_n7, assign16140_e11053_d_n8, assign16140_e11053_d_n9, assign16140_e11053_d_n10, assign16140_e11053_d_n11, assign16140_e11053_d_n14,) = {
    if ((locals.var_guard293 != 0.0) && (locals.var_guard336 == 0.0)) {
        let assign16140_e11051: f64 = (p.p16 * locals.var_jssw);
        (assign16140_e11051, (p.p16 * locals.var_jssw_dn0), (p.p16 * locals.var_jssw_dn2), (p.p16 * locals.var_jssw_dn4), (p.p16 * locals.var_jssw_dn5), (p.p16 * locals.var_jssw_dn6), (p.p16 * locals.var_jssw_dn7), (p.p16 * locals.var_jssw_dn8), (p.p16 * locals.var_jssw_dn9), (p.p16 * locals.var_jssw_dn10), (p.p16 * locals.var_jssw_dn11), (p.p16 * locals.var_jssw_dn14),)
    } else {
        (locals.var_isbs_sws, locals.var_isbs_sws_dn0, locals.var_isbs_sws_dn2, locals.var_isbs_sws_dn4, locals.var_isbs_sws_dn5, locals.var_isbs_sws_dn6, locals.var_isbs_sws_dn7, locals.var_isbs_sws_dn8, locals.var_isbs_sws_dn9, locals.var_isbs_sws_dn10, locals.var_isbs_sws_dn11, locals.var_isbs_sws_dn14,)
    }
};
        locals.var_isbs_sws = assign16140_e11053;
        locals.var_isbs_sws_dn0 = assign16140_e11053_d_n0;
        locals.var_isbs_sws_dn2 = assign16140_e11053_d_n2;
        locals.var_isbs_sws_dn4 = assign16140_e11053_d_n4;
        locals.var_isbs_sws_dn5 = assign16140_e11053_d_n5;
        locals.var_isbs_sws_dn6 = assign16140_e11053_d_n6;
        locals.var_isbs_sws_dn7 = assign16140_e11053_d_n7;
        locals.var_isbs_sws_dn8 = assign16140_e11053_d_n8;
        locals.var_isbs_sws_dn9 = assign16140_e11053_d_n9;
        locals.var_isbs_sws_dn10 = assign16140_e11053_d_n10;
        locals.var_isbs_sws_dn11 = assign16140_e11053_d_n11;
        locals.var_isbs_sws_dn14 = assign16140_e11053_d_n14;

        let (assign16150_e11062, assign16150_e11062_d_n0, assign16150_e11062_d_n2, assign16150_e11062_d_n4, assign16150_e11062_d_n5, assign16150_e11062_d_n6, assign16150_e11062_d_n7, assign16150_e11062_d_n8, assign16150_e11062_d_n9, assign16150_e11062_d_n10, assign16150_e11062_d_n11, assign16150_e11062_d_n14,) = {
    if ((locals.var_guard293 != 0.0) && (locals.var_guard336 == 0.0)) {
        let assign16150_e11060: f64 = (p.p16 * locals.var_jssw2);
        (assign16150_e11060, (p.p16 * locals.var_jssw2_dn0), (p.p16 * locals.var_jssw2_dn2), (p.p16 * locals.var_jssw2_dn4), (p.p16 * locals.var_jssw2_dn5), (p.p16 * locals.var_jssw2_dn6), (p.p16 * locals.var_jssw2_dn7), (p.p16 * locals.var_jssw2_dn8), (p.p16 * locals.var_jssw2_dn9), (p.p16 * locals.var_jssw2_dn10), (p.p16 * locals.var_jssw2_dn11), (p.p16 * locals.var_jssw2_dn14),)
    } else {
        (locals.var_isbs2_sws, locals.var_isbs2_sws_dn0, locals.var_isbs2_sws_dn2, locals.var_isbs2_sws_dn4, locals.var_isbs2_sws_dn5, locals.var_isbs2_sws_dn6, locals.var_isbs2_sws_dn7, locals.var_isbs2_sws_dn8, locals.var_isbs2_sws_dn9, locals.var_isbs2_sws_dn10, locals.var_isbs2_sws_dn11, locals.var_isbs2_sws_dn14,)
    }
};
        locals.var_isbs2_sws = assign16150_e11062;
        locals.var_isbs2_sws_dn0 = assign16150_e11062_d_n0;
        locals.var_isbs2_sws_dn2 = assign16150_e11062_d_n2;
        locals.var_isbs2_sws_dn4 = assign16150_e11062_d_n4;
        locals.var_isbs2_sws_dn5 = assign16150_e11062_d_n5;
        locals.var_isbs2_sws_dn6 = assign16150_e11062_d_n6;
        locals.var_isbs2_sws_dn7 = assign16150_e11062_d_n7;
        locals.var_isbs2_sws_dn8 = assign16150_e11062_d_n8;
        locals.var_isbs2_sws_dn9 = assign16150_e11062_d_n9;
        locals.var_isbs2_sws_dn10 = assign16150_e11062_d_n10;
        locals.var_isbs2_sws_dn11 = assign16150_e11062_d_n11;
        locals.var_isbs2_sws_dn14 = assign16150_e11062_d_n14;

        let (assign16160_e11069, assign16160_e11069_d_n0, assign16160_e11069_d_n2, assign16160_e11069_d_n4, assign16160_e11069_d_n5, assign16160_e11069_d_n6, assign16160_e11069_d_n7, assign16160_e11069_d_n8, assign16160_e11069_d_n9, assign16160_e11069_d_n10, assign16160_e11069_d_n11, assign16160_e11069_d_n14,) = {
    if ((locals.var_guard293 != 0.0) && (locals.var_guard336 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbs_swg, locals.var_isbs_swg_dn0, locals.var_isbs_swg_dn2, locals.var_isbs_swg_dn4, locals.var_isbs_swg_dn5, locals.var_isbs_swg_dn6, locals.var_isbs_swg_dn7, locals.var_isbs_swg_dn8, locals.var_isbs_swg_dn9, locals.var_isbs_swg_dn10, locals.var_isbs_swg_dn11, locals.var_isbs_swg_dn14,)
    }
};
        locals.var_isbs_swg = assign16160_e11069;
        locals.var_isbs_swg_dn0 = assign16160_e11069_d_n0;
        locals.var_isbs_swg_dn2 = assign16160_e11069_d_n2;
        locals.var_isbs_swg_dn4 = assign16160_e11069_d_n4;
        locals.var_isbs_swg_dn5 = assign16160_e11069_d_n5;
        locals.var_isbs_swg_dn6 = assign16160_e11069_d_n6;
        locals.var_isbs_swg_dn7 = assign16160_e11069_d_n7;
        locals.var_isbs_swg_dn8 = assign16160_e11069_d_n8;
        locals.var_isbs_swg_dn9 = assign16160_e11069_d_n9;
        locals.var_isbs_swg_dn10 = assign16160_e11069_d_n10;
        locals.var_isbs_swg_dn11 = assign16160_e11069_d_n11;
        locals.var_isbs_swg_dn14 = assign16160_e11069_d_n14;

        let (assign16170_e11076, assign16170_e11076_d_n0, assign16170_e11076_d_n2, assign16170_e11076_d_n4, assign16170_e11076_d_n5, assign16170_e11076_d_n6, assign16170_e11076_d_n7, assign16170_e11076_d_n8, assign16170_e11076_d_n9, assign16170_e11076_d_n10, assign16170_e11076_d_n11, assign16170_e11076_d_n14,) = {
    if ((locals.var_guard293 != 0.0) && (locals.var_guard336 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbs2_swg, locals.var_isbs2_swg_dn0, locals.var_isbs2_swg_dn2, locals.var_isbs2_swg_dn4, locals.var_isbs2_swg_dn5, locals.var_isbs2_swg_dn6, locals.var_isbs2_swg_dn7, locals.var_isbs2_swg_dn8, locals.var_isbs2_swg_dn9, locals.var_isbs2_swg_dn10, locals.var_isbs2_swg_dn11, locals.var_isbs2_swg_dn14,)
    }
};
        locals.var_isbs2_swg = assign16170_e11076;
        locals.var_isbs2_swg_dn0 = assign16170_e11076_d_n0;
        locals.var_isbs2_swg_dn2 = assign16170_e11076_d_n2;
        locals.var_isbs2_swg_dn4 = assign16170_e11076_d_n4;
        locals.var_isbs2_swg_dn5 = assign16170_e11076_d_n5;
        locals.var_isbs2_swg_dn6 = assign16170_e11076_d_n6;
        locals.var_isbs2_swg_dn7 = assign16170_e11076_d_n7;
        locals.var_isbs2_swg_dn8 = assign16170_e11076_d_n8;
        locals.var_isbs2_swg_dn9 = assign16170_e11076_d_n9;
        locals.var_isbs2_swg_dn10 = assign16170_e11076_d_n10;
        locals.var_isbs2_swg_dn11 = assign16170_e11076_d_n11;
        locals.var_isbs2_swg_dn14 = assign16170_e11076_d_n14;

        let (assign16180_e11084, assign16180_e11084_d_n0, assign16180_e11084_d_n2, assign16180_e11084_d_n4, assign16180_e11084_d_n5, assign16180_e11084_d_n6, assign16180_e11084_d_n7, assign16180_e11084_d_n8, assign16180_e11084_d_n9, assign16180_e11084_d_n10, assign16180_e11084_d_n11, assign16180_e11084_d_n14,) = {
    if (locals.var_guard293 != 0.0) {
        let assign16180_e11080: f64 = (locals.var_isbs_btm + locals.var_isbs_sws);
        let assign16180_e11082: f64 = (assign16180_e11080 + locals.var_isbs_swg);
        (assign16180_e11082, ((locals.var_isbs_btm_dn0 + locals.var_isbs_sws_dn0) + locals.var_isbs_swg_dn0), ((locals.var_isbs_btm_dn2 + locals.var_isbs_sws_dn2) + locals.var_isbs_swg_dn2), ((locals.var_isbs_btm_dn4 + locals.var_isbs_sws_dn4) + locals.var_isbs_swg_dn4), ((locals.var_isbs_btm_dn5 + locals.var_isbs_sws_dn5) + locals.var_isbs_swg_dn5), ((locals.var_isbs_btm_dn6 + locals.var_isbs_sws_dn6) + locals.var_isbs_swg_dn6), ((locals.var_isbs_btm_dn7 + locals.var_isbs_sws_dn7) + locals.var_isbs_swg_dn7), ((locals.var_isbs_btm_dn8 + locals.var_isbs_sws_dn8) + locals.var_isbs_swg_dn8), ((locals.var_isbs_btm_dn9 + locals.var_isbs_sws_dn9) + locals.var_isbs_swg_dn9), ((locals.var_isbs_btm_dn10 + locals.var_isbs_sws_dn10) + locals.var_isbs_swg_dn10), ((locals.var_isbs_btm_dn11 + locals.var_isbs_sws_dn11) + locals.var_isbs_swg_dn11), ((locals.var_isbs_btm_dn14 + locals.var_isbs_sws_dn14) + locals.var_isbs_swg_dn14),)
    } else {
        (locals.var_isbs, locals.var_isbs_dn0, locals.var_isbs_dn2, locals.var_isbs_dn4, locals.var_isbs_dn5, locals.var_isbs_dn6, locals.var_isbs_dn7, locals.var_isbs_dn8, locals.var_isbs_dn9, locals.var_isbs_dn10, locals.var_isbs_dn11, locals.var_isbs_dn14,)
    }
};
        locals.var_isbs = assign16180_e11084;
        locals.var_isbs_dn0 = assign16180_e11084_d_n0;
        locals.var_isbs_dn2 = assign16180_e11084_d_n2;
        locals.var_isbs_dn4 = assign16180_e11084_d_n4;
        locals.var_isbs_dn5 = assign16180_e11084_d_n5;
        locals.var_isbs_dn6 = assign16180_e11084_d_n6;
        locals.var_isbs_dn7 = assign16180_e11084_d_n7;
        locals.var_isbs_dn8 = assign16180_e11084_d_n8;
        locals.var_isbs_dn9 = assign16180_e11084_d_n9;
        locals.var_isbs_dn10 = assign16180_e11084_d_n10;
        locals.var_isbs_dn11 = assign16180_e11084_d_n11;
        locals.var_isbs_dn14 = assign16180_e11084_d_n14;

        let assign16190_e11087: f64 = if locals.var_isbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard338 = assign16190_e11087;

        let (assign16200_e11095, assign16200_e11095_d_n0, assign16200_e11095_d_n2, assign16200_e11095_d_n4, assign16200_e11095_d_n5, assign16200_e11095_d_n6, assign16200_e11095_d_n7, assign16200_e11095_d_n8, assign16200_e11095_d_n9, assign16200_e11095_d_n10, assign16200_e11095_d_n11, assign16200_e11095_d_n14,) = {
    if ((locals.var_guard293 != 0.0) && (locals.var_guard338 != 0.0)) {
        let assign16200_e11093: f64 = (locals.var_isbs + 1e-25);
        (assign16200_e11093, locals.var_isbs_dn0, locals.var_isbs_dn2, locals.var_isbs_dn4, locals.var_isbs_dn5, locals.var_isbs_dn6, locals.var_isbs_dn7, locals.var_isbs_dn8, locals.var_isbs_dn9, locals.var_isbs_dn10, locals.var_isbs_dn11, locals.var_isbs_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign16200_e11095;
        locals.var_t3_dn0 = assign16200_e11095_d_n0;
        locals.var_t3_dn2 = assign16200_e11095_d_n2;
        locals.var_t3_dn4 = assign16200_e11095_d_n4;
        locals.var_t3_dn5 = assign16200_e11095_d_n5;
        locals.var_t3_dn6 = assign16200_e11095_d_n6;
        locals.var_t3_dn7 = assign16200_e11095_d_n7;
        locals.var_t3_dn8 = assign16200_e11095_d_n8;
        locals.var_t3_dn9 = assign16200_e11095_d_n9;
        locals.var_t3_dn10 = assign16200_e11095_d_n10;
        locals.var_t3_dn11 = assign16200_e11095_d_n11;
        locals.var_t3_dn14 = assign16200_e11095_d_n14;

        let (assign16210_e11112, assign16210_e11112_d_n0, assign16210_e11112_d_n2, assign16210_e11112_d_n4, assign16210_e11112_d_n5, assign16210_e11112_d_n6, assign16210_e11112_d_n7, assign16210_e11112_d_n8, assign16210_e11112_d_n9, assign16210_e11112_d_n10, assign16210_e11112_d_n11, assign16210_e11112_d_n14,) = {
    if ((locals.var_guard293 != 0.0) && (locals.var_guard338 != 0.0)) {
        let assign16210_e11101: f64 = (locals.var_uc_njs / locals.var_beta);
        let assign16210_e11104: f64 = (locals.var_uc_vdiffjs * locals.var_t0);
        let assign16210_e11106: f64 = (assign16210_e11104 / locals.var_t3);
        let assign16210_e11108: f64 = (assign16210_e11106 + 1.0);
        let assign16210_e11109: f64 = (assign16210_e11108).ln();
        let assign16210_e11110: f64 = (assign16210_e11101 * assign16210_e11109);
        (assign16210_e11110, (((-((locals.var_uc_njs * locals.var_beta_dn0) / (locals.var_beta * locals.var_beta))) * assign16210_e11109) + (assign16210_e11101 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn0) * locals.var_t3) - (assign16210_e11104 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)) / assign16210_e11108))), (((-((locals.var_uc_njs * locals.var_beta_dn2) / (locals.var_beta * locals.var_beta))) * assign16210_e11109) + (assign16210_e11101 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn2) * locals.var_t3) - (assign16210_e11104 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)) / assign16210_e11108))), (((-((locals.var_uc_njs * locals.var_beta_dn4) / (locals.var_beta * locals.var_beta))) * assign16210_e11109) + (assign16210_e11101 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn4) * locals.var_t3) - (assign16210_e11104 * locals.var_t3_dn4)) / (locals.var_t3 * locals.var_t3)) / assign16210_e11108))), (((-((locals.var_uc_njs * locals.var_beta_dn5) / (locals.var_beta * locals.var_beta))) * assign16210_e11109) + (assign16210_e11101 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn5) * locals.var_t3) - (assign16210_e11104 * locals.var_t3_dn5)) / (locals.var_t3 * locals.var_t3)) / assign16210_e11108))), (((-((locals.var_uc_njs * locals.var_beta_dn6) / (locals.var_beta * locals.var_beta))) * assign16210_e11109) + (assign16210_e11101 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn6) * locals.var_t3) - (assign16210_e11104 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)) / assign16210_e11108))), (((-((locals.var_uc_njs * locals.var_beta_dn7) / (locals.var_beta * locals.var_beta))) * assign16210_e11109) + (assign16210_e11101 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn7) * locals.var_t3) - (assign16210_e11104 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)) / assign16210_e11108))), (((-((locals.var_uc_njs * locals.var_beta_dn8) / (locals.var_beta * locals.var_beta))) * assign16210_e11109) + (assign16210_e11101 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn8) * locals.var_t3) - (assign16210_e11104 * locals.var_t3_dn8)) / (locals.var_t3 * locals.var_t3)) / assign16210_e11108))), (((-((locals.var_uc_njs * locals.var_beta_dn9) / (locals.var_beta * locals.var_beta))) * assign16210_e11109) + (assign16210_e11101 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn9) * locals.var_t3) - (assign16210_e11104 * locals.var_t3_dn9)) / (locals.var_t3 * locals.var_t3)) / assign16210_e11108))), (((-((locals.var_uc_njs * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) * assign16210_e11109) + (assign16210_e11101 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn10) * locals.var_t3) - (assign16210_e11104 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)) / assign16210_e11108))), (((-((locals.var_uc_njs * locals.var_beta_dn11) / (locals.var_beta * locals.var_beta))) * assign16210_e11109) + (assign16210_e11101 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn11) * locals.var_t3) - (assign16210_e11104 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)) / assign16210_e11108))), (((-((locals.var_uc_njs * locals.var_beta_dn14) / (locals.var_beta * locals.var_beta))) * assign16210_e11109) + (assign16210_e11101 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn14) * locals.var_t3) - (assign16210_e11104 * locals.var_t3_dn14)) / (locals.var_t3 * locals.var_t3)) / assign16210_e11108))),)
    } else {
        (locals.var_vbst, locals.var_vbst_dn0, locals.var_vbst_dn2, locals.var_vbst_dn4, locals.var_vbst_dn5, locals.var_vbst_dn6, locals.var_vbst_dn7, locals.var_vbst_dn8, locals.var_vbst_dn9, locals.var_vbst_dn10, locals.var_vbst_dn11, locals.var_vbst_dn14,)
    }
};
        locals.var_vbst = assign16210_e11112;
        locals.var_vbst_dn0 = assign16210_e11112_d_n0;
        locals.var_vbst_dn2 = assign16210_e11112_d_n2;
        locals.var_vbst_dn4 = assign16210_e11112_d_n4;
        locals.var_vbst_dn5 = assign16210_e11112_d_n5;
        locals.var_vbst_dn6 = assign16210_e11112_d_n6;
        locals.var_vbst_dn7 = assign16210_e11112_d_n7;
        locals.var_vbst_dn8 = assign16210_e11112_d_n8;
        locals.var_vbst_dn9 = assign16210_e11112_d_n9;
        locals.var_vbst_dn10 = assign16210_e11112_d_n10;
        locals.var_vbst_dn11 = assign16210_e11112_d_n11;
        locals.var_vbst_dn14 = assign16210_e11112_d_n14;

        let (assign16220_e11123, assign16220_e11123_d_n0, assign16220_e11123_d_n2, assign16220_e11123_d_n4, assign16220_e11123_d_n5, assign16220_e11123_d_n6, assign16220_e11123_d_n7, assign16220_e11123_d_n8, assign16220_e11123_d_n9, assign16220_e11123_d_n10, assign16220_e11123_d_n11, assign16220_e11123_d_n14,) = {
    if ((locals.var_guard293 != 0.0) && (locals.var_guard338 != 0.0)) {
        let assign16220_e11118: f64 = (locals.var_tratio - 1.0);
        let assign16220_e11120: f64 = (assign16220_e11118 * p.p535);
        let assign16220_e11121: f64 = (assign16220_e11120).exp();
        (assign16220_e11121, (assign16220_e11121 * (locals.var_tratio_dn0 * p.p535)), (assign16220_e11121 * (locals.var_tratio_dn2 * p.p535)), (assign16220_e11121 * (locals.var_tratio_dn4 * p.p535)), (assign16220_e11121 * (locals.var_tratio_dn5 * p.p535)), (assign16220_e11121 * (locals.var_tratio_dn6 * p.p535)), (assign16220_e11121 * (locals.var_tratio_dn7 * p.p535)), (assign16220_e11121 * (locals.var_tratio_dn8 * p.p535)), (assign16220_e11121 * (locals.var_tratio_dn9 * p.p535)), (assign16220_e11121 * (locals.var_tratio_dn10 * p.p535)), (assign16220_e11121 * (locals.var_tratio_dn11 * p.p535)), (assign16220_e11121 * (locals.var_tratio_dn14 * p.p535)),)
    } else {
        (locals.var_exptemps, locals.var_exptemps_dn0, locals.var_exptemps_dn2, locals.var_exptemps_dn4, locals.var_exptemps_dn5, locals.var_exptemps_dn6, locals.var_exptemps_dn7, locals.var_exptemps_dn8, locals.var_exptemps_dn9, locals.var_exptemps_dn10, locals.var_exptemps_dn11, locals.var_exptemps_dn14,)
    }
};
        locals.var_exptemps = assign16220_e11123;
        locals.var_exptemps_dn0 = assign16220_e11123_d_n0;
        locals.var_exptemps_dn2 = assign16220_e11123_d_n2;
        locals.var_exptemps_dn4 = assign16220_e11123_d_n4;
        locals.var_exptemps_dn5 = assign16220_e11123_d_n5;
        locals.var_exptemps_dn6 = assign16220_e11123_d_n6;
        locals.var_exptemps_dn7 = assign16220_e11123_d_n7;
        locals.var_exptemps_dn8 = assign16220_e11123_d_n8;
        locals.var_exptemps_dn9 = assign16220_e11123_d_n9;
        locals.var_exptemps_dn10 = assign16220_e11123_d_n10;
        locals.var_exptemps_dn11 = assign16220_e11123_d_n11;
        locals.var_exptemps_dn14 = assign16220_e11123_d_n14;

        let (assign16230_e11133, assign16230_e11133_d_n0, assign16230_e11133_d_n2, assign16230_e11133_d_n4, assign16230_e11133_d_n5, assign16230_e11133_d_n6, assign16230_e11133_d_n7, assign16230_e11133_d_n8, assign16230_e11133_d_n9, assign16230_e11133_d_n10, assign16230_e11133_d_n11, assign16230_e11133_d_n14,) = {
    if ((locals.var_guard293 != 0.0) && (locals.var_guard338 != 0.0)) {
        let assign16230_e11130: f64 = (locals.var_uc_njs / locals.var_beta);
        let assign16230_e11131: f64 = (1.0 / assign16230_e11130);
        (assign16230_e11131, (-((-((locals.var_uc_njs * locals.var_beta_dn0) / (locals.var_beta * locals.var_beta))) / (assign16230_e11130 * assign16230_e11130))), (-((-((locals.var_uc_njs * locals.var_beta_dn2) / (locals.var_beta * locals.var_beta))) / (assign16230_e11130 * assign16230_e11130))), (-((-((locals.var_uc_njs * locals.var_beta_dn4) / (locals.var_beta * locals.var_beta))) / (assign16230_e11130 * assign16230_e11130))), (-((-((locals.var_uc_njs * locals.var_beta_dn5) / (locals.var_beta * locals.var_beta))) / (assign16230_e11130 * assign16230_e11130))), (-((-((locals.var_uc_njs * locals.var_beta_dn6) / (locals.var_beta * locals.var_beta))) / (assign16230_e11130 * assign16230_e11130))), (-((-((locals.var_uc_njs * locals.var_beta_dn7) / (locals.var_beta * locals.var_beta))) / (assign16230_e11130 * assign16230_e11130))), (-((-((locals.var_uc_njs * locals.var_beta_dn8) / (locals.var_beta * locals.var_beta))) / (assign16230_e11130 * assign16230_e11130))), (-((-((locals.var_uc_njs * locals.var_beta_dn9) / (locals.var_beta * locals.var_beta))) / (assign16230_e11130 * assign16230_e11130))), (-((-((locals.var_uc_njs * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) / (assign16230_e11130 * assign16230_e11130))), (-((-((locals.var_uc_njs * locals.var_beta_dn11) / (locals.var_beta * locals.var_beta))) / (assign16230_e11130 * assign16230_e11130))), (-((-((locals.var_uc_njs * locals.var_beta_dn14) / (locals.var_beta * locals.var_beta))) / (assign16230_e11130 * assign16230_e11130))),)
    } else {
        (locals.var_jd_nvtm_invs, locals.var_jd_nvtm_invs_dn0, locals.var_jd_nvtm_invs_dn2, locals.var_jd_nvtm_invs_dn4, locals.var_jd_nvtm_invs_dn5, locals.var_jd_nvtm_invs_dn6, locals.var_jd_nvtm_invs_dn7, locals.var_jd_nvtm_invs_dn8, locals.var_jd_nvtm_invs_dn9, locals.var_jd_nvtm_invs_dn10, locals.var_jd_nvtm_invs_dn11, locals.var_jd_nvtm_invs_dn14,)
    }
};
        locals.var_jd_nvtm_invs = assign16230_e11133;
        locals.var_jd_nvtm_invs_dn0 = assign16230_e11133_d_n0;
        locals.var_jd_nvtm_invs_dn2 = assign16230_e11133_d_n2;
        locals.var_jd_nvtm_invs_dn4 = assign16230_e11133_d_n4;
        locals.var_jd_nvtm_invs_dn5 = assign16230_e11133_d_n5;
        locals.var_jd_nvtm_invs_dn6 = assign16230_e11133_d_n6;
        locals.var_jd_nvtm_invs_dn7 = assign16230_e11133_d_n7;
        locals.var_jd_nvtm_invs_dn8 = assign16230_e11133_d_n8;
        locals.var_jd_nvtm_invs_dn9 = assign16230_e11133_d_n9;
        locals.var_jd_nvtm_invs_dn10 = assign16230_e11133_d_n10;
        locals.var_jd_nvtm_invs_dn11 = assign16230_e11133_d_n11;
        locals.var_jd_nvtm_invs_dn14 = assign16230_e11133_d_n14;

        let (assign16240_e11142, assign16240_e11142_d_n0, assign16240_e11142_d_n2, assign16240_e11142_d_n4, assign16240_e11142_d_n5, assign16240_e11142_d_n6, assign16240_e11142_d_n7, assign16240_e11142_d_n8, assign16240_e11142_d_n9, assign16240_e11142_d_n10, assign16240_e11142_d_n11, assign16240_e11142_d_n14,) = {
    if ((locals.var_guard293 != 0.0) && (locals.var_guard338 != 0.0)) {
        let assign16240_e11139: f64 = (locals.var_vbst * locals.var_jd_nvtm_invs);
        let assign16240_e11140: f64 = (assign16240_e11139).exp();
        (assign16240_e11140, (assign16240_e11140 * ((locals.var_vbst_dn0 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn0))), (assign16240_e11140 * ((locals.var_vbst_dn2 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn2))), (assign16240_e11140 * ((locals.var_vbst_dn4 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn4))), (assign16240_e11140 * ((locals.var_vbst_dn5 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn5))), (assign16240_e11140 * ((locals.var_vbst_dn6 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn6))), (assign16240_e11140 * ((locals.var_vbst_dn7 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn7))), (assign16240_e11140 * ((locals.var_vbst_dn8 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn8))), (assign16240_e11140 * ((locals.var_vbst_dn9 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn9))), (assign16240_e11140 * ((locals.var_vbst_dn10 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn10))), (assign16240_e11140 * ((locals.var_vbst_dn11 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn11))), (assign16240_e11140 * ((locals.var_vbst_dn14 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn14))),)
    } else {
        (locals.var_jd_expcs, locals.var_jd_expcs_dn0, locals.var_jd_expcs_dn2, locals.var_jd_expcs_dn4, locals.var_jd_expcs_dn5, locals.var_jd_expcs_dn6, locals.var_jd_expcs_dn7, locals.var_jd_expcs_dn8, locals.var_jd_expcs_dn9, locals.var_jd_expcs_dn10, locals.var_jd_expcs_dn11, locals.var_jd_expcs_dn14,)
    }
};
        locals.var_jd_expcs = assign16240_e11142;
        locals.var_jd_expcs_dn0 = assign16240_e11142_d_n0;
        locals.var_jd_expcs_dn2 = assign16240_e11142_d_n2;
        locals.var_jd_expcs_dn4 = assign16240_e11142_d_n4;
        locals.var_jd_expcs_dn5 = assign16240_e11142_d_n5;
        locals.var_jd_expcs_dn6 = assign16240_e11142_d_n6;
        locals.var_jd_expcs_dn7 = assign16240_e11142_d_n7;
        locals.var_jd_expcs_dn8 = assign16240_e11142_d_n8;
        locals.var_jd_expcs_dn9 = assign16240_e11142_d_n9;
        locals.var_jd_expcs_dn10 = assign16240_e11142_d_n10;
        locals.var_jd_expcs_dn11 = assign16240_e11142_d_n11;
        locals.var_jd_expcs_dn14 = assign16240_e11142_d_n14;

        let (assign16250_e11154, assign16250_e11154_d_n0, assign16250_e11154_d_n2, assign16250_e11154_d_n4, assign16250_e11154_d_n5, assign16250_e11154_d_n6, assign16250_e11154_d_n7, assign16250_e11154_d_n8, assign16250_e11154_d_n9, assign16250_e11154_d_n10, assign16250_e11154_d_n11, assign16250_e11154_d_n14,) = {
    if (locals.var_guard293 != 0.0) {
        let assign16250_e11146: f64 = (p.p500 * p.p13);
        let assign16250_e11150: f64 = (p.p481 * locals.var_tdiff);
        let assign16250_e11151: f64 = (1.0 + assign16250_e11150);
        let assign16250_e11152: f64 = (assign16250_e11146 * assign16250_e11151);
        (assign16250_e11152, (assign16250_e11146 * (p.p481 * locals.var_tdiff_dn0)), (assign16250_e11146 * (p.p481 * locals.var_tdiff_dn2)), (assign16250_e11146 * (p.p481 * locals.var_tdiff_dn4)), (assign16250_e11146 * (p.p481 * locals.var_tdiff_dn5)), (assign16250_e11146 * (p.p481 * locals.var_tdiff_dn6)), (assign16250_e11146 * (p.p481 * locals.var_tdiff_dn7)), (assign16250_e11146 * (p.p481 * locals.var_tdiff_dn8)), (assign16250_e11146 * (p.p481 * locals.var_tdiff_dn9)), (assign16250_e11146 * (p.p481 * locals.var_tdiff_dn10)), (assign16250_e11146 * (p.p481 * locals.var_tdiff_dn11)), (assign16250_e11146 * (p.p481 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_czbd, locals.var_czbd_dn0, locals.var_czbd_dn2, locals.var_czbd_dn4, locals.var_czbd_dn5, locals.var_czbd_dn6, locals.var_czbd_dn7, locals.var_czbd_dn8, locals.var_czbd_dn9, locals.var_czbd_dn10, locals.var_czbd_dn11, locals.var_czbd_dn14,)
    }
};
        locals.var_czbd = assign16250_e11154;
        locals.var_czbd_dn0 = assign16250_e11154_d_n0;
        locals.var_czbd_dn2 = assign16250_e11154_d_n2;
        locals.var_czbd_dn4 = assign16250_e11154_d_n4;
        locals.var_czbd_dn5 = assign16250_e11154_d_n5;
        locals.var_czbd_dn6 = assign16250_e11154_d_n6;
        locals.var_czbd_dn7 = assign16250_e11154_d_n7;
        locals.var_czbd_dn8 = assign16250_e11154_d_n8;
        locals.var_czbd_dn9 = assign16250_e11154_d_n9;
        locals.var_czbd_dn10 = assign16250_e11154_d_n10;
        locals.var_czbd_dn11 = assign16250_e11154_d_n11;
        locals.var_czbd_dn14 = assign16250_e11154_d_n14;

        let assign16260_e11157: f64 = if p.p15 > locals.var_weff_nf { 1.0 } else { 0.0 };
        locals.var_guard339 = assign16260_e11157;

        let (assign16270_e11173, assign16270_e11173_d_n0, assign16270_e11173_d_n2, assign16270_e11173_d_n4, assign16270_e11173_d_n5, assign16270_e11173_d_n6, assign16270_e11173_d_n7, assign16270_e11173_d_n8, assign16270_e11173_d_n9, assign16270_e11173_d_n10, assign16270_e11173_d_n11, assign16270_e11173_d_n14,) = {
    if ((locals.var_guard293 != 0.0) && (locals.var_guard339 != 0.0)) {
        let assign16270_e11164: f64 = (p.p15 - locals.var_weff_nf);
        let assign16270_e11165: f64 = (p.p501 * assign16270_e11164);
        let assign16270_e11169: f64 = (p.p483 * locals.var_tdiff);
        let assign16270_e11170: f64 = (1.0 + assign16270_e11169);
        let assign16270_e11171: f64 = (assign16270_e11165 * assign16270_e11170);
        (assign16270_e11171, (assign16270_e11165 * (p.p483 * locals.var_tdiff_dn0)), (assign16270_e11165 * (p.p483 * locals.var_tdiff_dn2)), (assign16270_e11165 * (p.p483 * locals.var_tdiff_dn4)), (assign16270_e11165 * (p.p483 * locals.var_tdiff_dn5)), (assign16270_e11165 * (p.p483 * locals.var_tdiff_dn6)), (assign16270_e11165 * (p.p483 * locals.var_tdiff_dn7)), (assign16270_e11165 * (p.p483 * locals.var_tdiff_dn8)), (assign16270_e11165 * (p.p483 * locals.var_tdiff_dn9)), (assign16270_e11165 * (p.p483 * locals.var_tdiff_dn10)), (assign16270_e11165 * (p.p483 * locals.var_tdiff_dn11)), (assign16270_e11165 * (p.p483 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_czbdsw, locals.var_czbdsw_dn0, locals.var_czbdsw_dn2, locals.var_czbdsw_dn4, locals.var_czbdsw_dn5, locals.var_czbdsw_dn6, locals.var_czbdsw_dn7, locals.var_czbdsw_dn8, locals.var_czbdsw_dn9, locals.var_czbdsw_dn10, locals.var_czbdsw_dn11, locals.var_czbdsw_dn14,)
    }
};
        locals.var_czbdsw = assign16270_e11173;
        locals.var_czbdsw_dn0 = assign16270_e11173_d_n0;
        locals.var_czbdsw_dn2 = assign16270_e11173_d_n2;
        locals.var_czbdsw_dn4 = assign16270_e11173_d_n4;
        locals.var_czbdsw_dn5 = assign16270_e11173_d_n5;
        locals.var_czbdsw_dn6 = assign16270_e11173_d_n6;
        locals.var_czbdsw_dn7 = assign16270_e11173_d_n7;
        locals.var_czbdsw_dn8 = assign16270_e11173_d_n8;
        locals.var_czbdsw_dn9 = assign16270_e11173_d_n9;
        locals.var_czbdsw_dn10 = assign16270_e11173_d_n10;
        locals.var_czbdsw_dn11 = assign16270_e11173_d_n11;
        locals.var_czbdsw_dn14 = assign16270_e11173_d_n14;

        let (assign16280_e11187, assign16280_e11187_d_n0, assign16280_e11187_d_n2, assign16280_e11187_d_n4, assign16280_e11187_d_n5, assign16280_e11187_d_n6, assign16280_e11187_d_n7, assign16280_e11187_d_n8, assign16280_e11187_d_n9, assign16280_e11187_d_n10, assign16280_e11187_d_n11, assign16280_e11187_d_n14,) = {
    if ((locals.var_guard293 != 0.0) && (locals.var_guard339 != 0.0)) {
        let assign16280_e11179: f64 = (p.p502 * locals.var_weff_nf);
        let assign16280_e11183: f64 = (p.p485 * locals.var_tdiff);
        let assign16280_e11184: f64 = (1.0 + assign16280_e11183);
        let assign16280_e11185: f64 = (assign16280_e11179 * assign16280_e11184);
        (assign16280_e11185, (assign16280_e11179 * (p.p485 * locals.var_tdiff_dn0)), (assign16280_e11179 * (p.p485 * locals.var_tdiff_dn2)), (assign16280_e11179 * (p.p485 * locals.var_tdiff_dn4)), (assign16280_e11179 * (p.p485 * locals.var_tdiff_dn5)), (assign16280_e11179 * (p.p485 * locals.var_tdiff_dn6)), (assign16280_e11179 * (p.p485 * locals.var_tdiff_dn7)), (assign16280_e11179 * (p.p485 * locals.var_tdiff_dn8)), (assign16280_e11179 * (p.p485 * locals.var_tdiff_dn9)), (assign16280_e11179 * (p.p485 * locals.var_tdiff_dn10)), (assign16280_e11179 * (p.p485 * locals.var_tdiff_dn11)), (assign16280_e11179 * (p.p485 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_czbdswg, locals.var_czbdswg_dn0, locals.var_czbdswg_dn2, locals.var_czbdswg_dn4, locals.var_czbdswg_dn5, locals.var_czbdswg_dn6, locals.var_czbdswg_dn7, locals.var_czbdswg_dn8, locals.var_czbdswg_dn9, locals.var_czbdswg_dn10, locals.var_czbdswg_dn11, locals.var_czbdswg_dn14,)
    }
};
        locals.var_czbdswg = assign16280_e11187;
        locals.var_czbdswg_dn0 = assign16280_e11187_d_n0;
        locals.var_czbdswg_dn2 = assign16280_e11187_d_n2;
        locals.var_czbdswg_dn4 = assign16280_e11187_d_n4;
        locals.var_czbdswg_dn5 = assign16280_e11187_d_n5;
        locals.var_czbdswg_dn6 = assign16280_e11187_d_n6;
        locals.var_czbdswg_dn7 = assign16280_e11187_d_n7;
        locals.var_czbdswg_dn8 = assign16280_e11187_d_n8;
        locals.var_czbdswg_dn9 = assign16280_e11187_d_n9;
        locals.var_czbdswg_dn10 = assign16280_e11187_d_n10;
        locals.var_czbdswg_dn11 = assign16280_e11187_d_n11;
        locals.var_czbdswg_dn14 = assign16280_e11187_d_n14;

        let (assign16290_e11194, assign16290_e11194_d_n0, assign16290_e11194_d_n2, assign16290_e11194_d_n4, assign16290_e11194_d_n5, assign16290_e11194_d_n6, assign16290_e11194_d_n7, assign16290_e11194_d_n8, assign16290_e11194_d_n9, assign16290_e11194_d_n10, assign16290_e11194_d_n11, assign16290_e11194_d_n14,) = {
    if ((locals.var_guard293 != 0.0) && (locals.var_guard339 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbdsw, locals.var_czbdsw_dn0, locals.var_czbdsw_dn2, locals.var_czbdsw_dn4, locals.var_czbdsw_dn5, locals.var_czbdsw_dn6, locals.var_czbdsw_dn7, locals.var_czbdsw_dn8, locals.var_czbdsw_dn9, locals.var_czbdsw_dn10, locals.var_czbdsw_dn11, locals.var_czbdsw_dn14,)
    }
};
        locals.var_czbdsw = assign16290_e11194;
        locals.var_czbdsw_dn0 = assign16290_e11194_d_n0;
        locals.var_czbdsw_dn2 = assign16290_e11194_d_n2;
        locals.var_czbdsw_dn4 = assign16290_e11194_d_n4;
        locals.var_czbdsw_dn5 = assign16290_e11194_d_n5;
        locals.var_czbdsw_dn6 = assign16290_e11194_d_n6;
        locals.var_czbdsw_dn7 = assign16290_e11194_d_n7;
        locals.var_czbdsw_dn8 = assign16290_e11194_d_n8;
        locals.var_czbdsw_dn9 = assign16290_e11194_d_n9;
        locals.var_czbdsw_dn10 = assign16290_e11194_d_n10;
        locals.var_czbdsw_dn11 = assign16290_e11194_d_n11;
        locals.var_czbdsw_dn14 = assign16290_e11194_d_n14;

        let (assign16300_e11209, assign16300_e11209_d_n0, assign16300_e11209_d_n2, assign16300_e11209_d_n4, assign16300_e11209_d_n5, assign16300_e11209_d_n6, assign16300_e11209_d_n7, assign16300_e11209_d_n8, assign16300_e11209_d_n9, assign16300_e11209_d_n10, assign16300_e11209_d_n11, assign16300_e11209_d_n14,) = {
    if ((locals.var_guard293 != 0.0) && (locals.var_guard339 == 0.0)) {
        let assign16300_e11201: f64 = (p.p502 * p.p15);
        let assign16300_e11205: f64 = (p.p485 * locals.var_tdiff);
        let assign16300_e11206: f64 = (1.0 + assign16300_e11205);
        let assign16300_e11207: f64 = (assign16300_e11201 * assign16300_e11206);
        (assign16300_e11207, (assign16300_e11201 * (p.p485 * locals.var_tdiff_dn0)), (assign16300_e11201 * (p.p485 * locals.var_tdiff_dn2)), (assign16300_e11201 * (p.p485 * locals.var_tdiff_dn4)), (assign16300_e11201 * (p.p485 * locals.var_tdiff_dn5)), (assign16300_e11201 * (p.p485 * locals.var_tdiff_dn6)), (assign16300_e11201 * (p.p485 * locals.var_tdiff_dn7)), (assign16300_e11201 * (p.p485 * locals.var_tdiff_dn8)), (assign16300_e11201 * (p.p485 * locals.var_tdiff_dn9)), (assign16300_e11201 * (p.p485 * locals.var_tdiff_dn10)), (assign16300_e11201 * (p.p485 * locals.var_tdiff_dn11)), (assign16300_e11201 * (p.p485 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_czbdswg, locals.var_czbdswg_dn0, locals.var_czbdswg_dn2, locals.var_czbdswg_dn4, locals.var_czbdswg_dn5, locals.var_czbdswg_dn6, locals.var_czbdswg_dn7, locals.var_czbdswg_dn8, locals.var_czbdswg_dn9, locals.var_czbdswg_dn10, locals.var_czbdswg_dn11, locals.var_czbdswg_dn14,)
    }
};
        locals.var_czbdswg = assign16300_e11209;
        locals.var_czbdswg_dn0 = assign16300_e11209_d_n0;
        locals.var_czbdswg_dn2 = assign16300_e11209_d_n2;
        locals.var_czbdswg_dn4 = assign16300_e11209_d_n4;
        locals.var_czbdswg_dn5 = assign16300_e11209_d_n5;
        locals.var_czbdswg_dn6 = assign16300_e11209_d_n6;
        locals.var_czbdswg_dn7 = assign16300_e11209_d_n7;
        locals.var_czbdswg_dn8 = assign16300_e11209_d_n8;
        locals.var_czbdswg_dn9 = assign16300_e11209_d_n9;
        locals.var_czbdswg_dn10 = assign16300_e11209_d_n10;
        locals.var_czbdswg_dn11 = assign16300_e11209_d_n11;
        locals.var_czbdswg_dn14 = assign16300_e11209_d_n14;

        let assign16310_e11212: f64 = if locals.var_czbd < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard340 = assign16310_e11212;

        let (assign16320_e11218, assign16320_e11218_d_n0, assign16320_e11218_d_n2, assign16320_e11218_d_n4, assign16320_e11218_d_n5, assign16320_e11218_d_n6, assign16320_e11218_d_n7, assign16320_e11218_d_n8, assign16320_e11218_d_n9, assign16320_e11218_d_n10, assign16320_e11218_d_n11, assign16320_e11218_d_n14,) = {
    if ((locals.var_guard293 != 0.0) && (locals.var_guard340 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbd, locals.var_czbd_dn0, locals.var_czbd_dn2, locals.var_czbd_dn4, locals.var_czbd_dn5, locals.var_czbd_dn6, locals.var_czbd_dn7, locals.var_czbd_dn8, locals.var_czbd_dn9, locals.var_czbd_dn10, locals.var_czbd_dn11, locals.var_czbd_dn14,)
    }
};
        locals.var_czbd = assign16320_e11218;
        locals.var_czbd_dn0 = assign16320_e11218_d_n0;
        locals.var_czbd_dn2 = assign16320_e11218_d_n2;
        locals.var_czbd_dn4 = assign16320_e11218_d_n4;
        locals.var_czbd_dn5 = assign16320_e11218_d_n5;
        locals.var_czbd_dn6 = assign16320_e11218_d_n6;
        locals.var_czbd_dn7 = assign16320_e11218_d_n7;
        locals.var_czbd_dn8 = assign16320_e11218_d_n8;
        locals.var_czbd_dn9 = assign16320_e11218_d_n9;
        locals.var_czbd_dn10 = assign16320_e11218_d_n10;
        locals.var_czbd_dn11 = assign16320_e11218_d_n11;
        locals.var_czbd_dn14 = assign16320_e11218_d_n14;

        let assign16330_e11221: f64 = if locals.var_czbdsw < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard341 = assign16330_e11221;

        let (assign16340_e11227, assign16340_e11227_d_n0, assign16340_e11227_d_n2, assign16340_e11227_d_n4, assign16340_e11227_d_n5, assign16340_e11227_d_n6, assign16340_e11227_d_n7, assign16340_e11227_d_n8, assign16340_e11227_d_n9, assign16340_e11227_d_n10, assign16340_e11227_d_n11, assign16340_e11227_d_n14,) = {
    if ((locals.var_guard293 != 0.0) && (locals.var_guard341 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbdsw, locals.var_czbdsw_dn0, locals.var_czbdsw_dn2, locals.var_czbdsw_dn4, locals.var_czbdsw_dn5, locals.var_czbdsw_dn6, locals.var_czbdsw_dn7, locals.var_czbdsw_dn8, locals.var_czbdsw_dn9, locals.var_czbdsw_dn10, locals.var_czbdsw_dn11, locals.var_czbdsw_dn14,)
    }
};
        locals.var_czbdsw = assign16340_e11227;
        locals.var_czbdsw_dn0 = assign16340_e11227_d_n0;
        locals.var_czbdsw_dn2 = assign16340_e11227_d_n2;
        locals.var_czbdsw_dn4 = assign16340_e11227_d_n4;
        locals.var_czbdsw_dn5 = assign16340_e11227_d_n5;
        locals.var_czbdsw_dn6 = assign16340_e11227_d_n6;
        locals.var_czbdsw_dn7 = assign16340_e11227_d_n7;
        locals.var_czbdsw_dn8 = assign16340_e11227_d_n8;
        locals.var_czbdsw_dn9 = assign16340_e11227_d_n9;
        locals.var_czbdsw_dn10 = assign16340_e11227_d_n10;
        locals.var_czbdsw_dn11 = assign16340_e11227_d_n11;
        locals.var_czbdsw_dn14 = assign16340_e11227_d_n14;

        let assign16350_e11230: f64 = if locals.var_czbdswg < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard342 = assign16350_e11230;

        let (assign16360_e11236, assign16360_e11236_d_n0, assign16360_e11236_d_n2, assign16360_e11236_d_n4, assign16360_e11236_d_n5, assign16360_e11236_d_n6, assign16360_e11236_d_n7, assign16360_e11236_d_n8, assign16360_e11236_d_n9, assign16360_e11236_d_n10, assign16360_e11236_d_n11, assign16360_e11236_d_n14,) = {
    if ((locals.var_guard293 != 0.0) && (locals.var_guard342 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbdswg, locals.var_czbdswg_dn0, locals.var_czbdswg_dn2, locals.var_czbdswg_dn4, locals.var_czbdswg_dn5, locals.var_czbdswg_dn6, locals.var_czbdswg_dn7, locals.var_czbdswg_dn8, locals.var_czbdswg_dn9, locals.var_czbdswg_dn10, locals.var_czbdswg_dn11, locals.var_czbdswg_dn14,)
    }
};
        locals.var_czbdswg = assign16360_e11236;
        locals.var_czbdswg_dn0 = assign16360_e11236_d_n0;
        locals.var_czbdswg_dn2 = assign16360_e11236_d_n2;
        locals.var_czbdswg_dn4 = assign16360_e11236_d_n4;
        locals.var_czbdswg_dn5 = assign16360_e11236_d_n5;
        locals.var_czbdswg_dn6 = assign16360_e11236_d_n6;
        locals.var_czbdswg_dn7 = assign16360_e11236_d_n7;
        locals.var_czbdswg_dn8 = assign16360_e11236_d_n8;
        locals.var_czbdswg_dn9 = assign16360_e11236_d_n9;
        locals.var_czbdswg_dn10 = assign16360_e11236_d_n10;
        locals.var_czbdswg_dn11 = assign16360_e11236_d_n11;
        locals.var_czbdswg_dn14 = assign16360_e11236_d_n14;

        let (assign16370_e11244, assign16370_e11244_d_n0, assign16370_e11244_d_n2, assign16370_e11244_d_n4, assign16370_e11244_d_n5, assign16370_e11244_d_n6, assign16370_e11244_d_n7, assign16370_e11244_d_n8, assign16370_e11244_d_n9, assign16370_e11244_d_n10, assign16370_e11244_d_n11, assign16370_e11244_d_n14,) = {
    if (locals.var_guard293 != 0.0) {
        let assign16370_e11241: f64 = (p.p487 * locals.var_tdiff);
        let assign16370_e11242: f64 = (p.p506 - assign16370_e11241);
        (assign16370_e11242, (-(p.p487 * locals.var_tdiff_dn0)), (-(p.p487 * locals.var_tdiff_dn2)), (-(p.p487 * locals.var_tdiff_dn4)), (-(p.p487 * locals.var_tdiff_dn5)), (-(p.p487 * locals.var_tdiff_dn6)), (-(p.p487 * locals.var_tdiff_dn7)), (-(p.p487 * locals.var_tdiff_dn8)), (-(p.p487 * locals.var_tdiff_dn9)), (-(p.p487 * locals.var_tdiff_dn10)), (-(p.p487 * locals.var_tdiff_dn11)), (-(p.p487 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_pzbd, locals.var_pzbd_dn0, locals.var_pzbd_dn2, locals.var_pzbd_dn4, locals.var_pzbd_dn5, locals.var_pzbd_dn6, locals.var_pzbd_dn7, locals.var_pzbd_dn8, locals.var_pzbd_dn9, locals.var_pzbd_dn10, locals.var_pzbd_dn11, locals.var_pzbd_dn14,)
    }
};
        locals.var_pzbd = assign16370_e11244;
        locals.var_pzbd_dn0 = assign16370_e11244_d_n0;
        locals.var_pzbd_dn2 = assign16370_e11244_d_n2;
        locals.var_pzbd_dn4 = assign16370_e11244_d_n4;
        locals.var_pzbd_dn5 = assign16370_e11244_d_n5;
        locals.var_pzbd_dn6 = assign16370_e11244_d_n6;
        locals.var_pzbd_dn7 = assign16370_e11244_d_n7;
        locals.var_pzbd_dn8 = assign16370_e11244_d_n8;
        locals.var_pzbd_dn9 = assign16370_e11244_d_n9;
        locals.var_pzbd_dn10 = assign16370_e11244_d_n10;
        locals.var_pzbd_dn11 = assign16370_e11244_d_n11;
        locals.var_pzbd_dn14 = assign16370_e11244_d_n14;

        let (assign16380_e11252, assign16380_e11252_d_n0, assign16380_e11252_d_n2, assign16380_e11252_d_n4, assign16380_e11252_d_n5, assign16380_e11252_d_n6, assign16380_e11252_d_n7, assign16380_e11252_d_n8, assign16380_e11252_d_n9, assign16380_e11252_d_n10, assign16380_e11252_d_n11, assign16380_e11252_d_n14,) = {
    if (locals.var_guard293 != 0.0) {
        let assign16380_e11249: f64 = (p.p489 * locals.var_tdiff);
        let assign16380_e11250: f64 = (p.p507 - assign16380_e11249);
        (assign16380_e11250, (-(p.p489 * locals.var_tdiff_dn0)), (-(p.p489 * locals.var_tdiff_dn2)), (-(p.p489 * locals.var_tdiff_dn4)), (-(p.p489 * locals.var_tdiff_dn5)), (-(p.p489 * locals.var_tdiff_dn6)), (-(p.p489 * locals.var_tdiff_dn7)), (-(p.p489 * locals.var_tdiff_dn8)), (-(p.p489 * locals.var_tdiff_dn9)), (-(p.p489 * locals.var_tdiff_dn10)), (-(p.p489 * locals.var_tdiff_dn11)), (-(p.p489 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_pzbdsw, locals.var_pzbdsw_dn0, locals.var_pzbdsw_dn2, locals.var_pzbdsw_dn4, locals.var_pzbdsw_dn5, locals.var_pzbdsw_dn6, locals.var_pzbdsw_dn7, locals.var_pzbdsw_dn8, locals.var_pzbdsw_dn9, locals.var_pzbdsw_dn10, locals.var_pzbdsw_dn11, locals.var_pzbdsw_dn14,)
    }
};
        locals.var_pzbdsw = assign16380_e11252;
        locals.var_pzbdsw_dn0 = assign16380_e11252_d_n0;
        locals.var_pzbdsw_dn2 = assign16380_e11252_d_n2;
        locals.var_pzbdsw_dn4 = assign16380_e11252_d_n4;
        locals.var_pzbdsw_dn5 = assign16380_e11252_d_n5;
        locals.var_pzbdsw_dn6 = assign16380_e11252_d_n6;
        locals.var_pzbdsw_dn7 = assign16380_e11252_d_n7;
        locals.var_pzbdsw_dn8 = assign16380_e11252_d_n8;
        locals.var_pzbdsw_dn9 = assign16380_e11252_d_n9;
        locals.var_pzbdsw_dn10 = assign16380_e11252_d_n10;
        locals.var_pzbdsw_dn11 = assign16380_e11252_d_n11;
        locals.var_pzbdsw_dn14 = assign16380_e11252_d_n14;

    }

    pub(super) fn stamp_transient_block_35(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let (assign16390_e11260, assign16390_e11260_d_n0, assign16390_e11260_d_n2, assign16390_e11260_d_n4, assign16390_e11260_d_n5, assign16390_e11260_d_n6, assign16390_e11260_d_n7, assign16390_e11260_d_n8, assign16390_e11260_d_n9, assign16390_e11260_d_n10, assign16390_e11260_d_n11, assign16390_e11260_d_n14,) = {
    if (locals.var_guard293 != 0.0) {
        let assign16390_e11257: f64 = (p.p491 * locals.var_tdiff);
        let assign16390_e11258: f64 = (p.p508 - assign16390_e11257);
        (assign16390_e11258, (-(p.p491 * locals.var_tdiff_dn0)), (-(p.p491 * locals.var_tdiff_dn2)), (-(p.p491 * locals.var_tdiff_dn4)), (-(p.p491 * locals.var_tdiff_dn5)), (-(p.p491 * locals.var_tdiff_dn6)), (-(p.p491 * locals.var_tdiff_dn7)), (-(p.p491 * locals.var_tdiff_dn8)), (-(p.p491 * locals.var_tdiff_dn9)), (-(p.p491 * locals.var_tdiff_dn10)), (-(p.p491 * locals.var_tdiff_dn11)), (-(p.p491 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_pzbdswg, locals.var_pzbdswg_dn0, locals.var_pzbdswg_dn2, locals.var_pzbdswg_dn4, locals.var_pzbdswg_dn5, locals.var_pzbdswg_dn6, locals.var_pzbdswg_dn7, locals.var_pzbdswg_dn8, locals.var_pzbdswg_dn9, locals.var_pzbdswg_dn10, locals.var_pzbdswg_dn11, locals.var_pzbdswg_dn14,)
    }
};
        locals.var_pzbdswg = assign16390_e11260;
        locals.var_pzbdswg_dn0 = assign16390_e11260_d_n0;
        locals.var_pzbdswg_dn2 = assign16390_e11260_d_n2;
        locals.var_pzbdswg_dn4 = assign16390_e11260_d_n4;
        locals.var_pzbdswg_dn5 = assign16390_e11260_d_n5;
        locals.var_pzbdswg_dn6 = assign16390_e11260_d_n6;
        locals.var_pzbdswg_dn7 = assign16390_e11260_d_n7;
        locals.var_pzbdswg_dn8 = assign16390_e11260_d_n8;
        locals.var_pzbdswg_dn9 = assign16390_e11260_d_n9;
        locals.var_pzbdswg_dn10 = assign16390_e11260_d_n10;
        locals.var_pzbdswg_dn11 = assign16390_e11260_d_n11;
        locals.var_pzbdswg_dn14 = assign16390_e11260_d_n14;

        let assign16400_e11267: f64 = if ((locals.var_pzbd < 0.01) && (p.p13 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard343 = assign16400_e11267;

        let (assign16410_e11273, assign16410_e11273_d_n0, assign16410_e11273_d_n2, assign16410_e11273_d_n4, assign16410_e11273_d_n5, assign16410_e11273_d_n6, assign16410_e11273_d_n7, assign16410_e11273_d_n8, assign16410_e11273_d_n9, assign16410_e11273_d_n10, assign16410_e11273_d_n11, assign16410_e11273_d_n14,) = {
    if ((locals.var_guard293 != 0.0) && (locals.var_guard343 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbd, locals.var_pzbd_dn0, locals.var_pzbd_dn2, locals.var_pzbd_dn4, locals.var_pzbd_dn5, locals.var_pzbd_dn6, locals.var_pzbd_dn7, locals.var_pzbd_dn8, locals.var_pzbd_dn9, locals.var_pzbd_dn10, locals.var_pzbd_dn11, locals.var_pzbd_dn14,)
    }
};
        locals.var_pzbd = assign16410_e11273;
        locals.var_pzbd_dn0 = assign16410_e11273_d_n0;
        locals.var_pzbd_dn2 = assign16410_e11273_d_n2;
        locals.var_pzbd_dn4 = assign16410_e11273_d_n4;
        locals.var_pzbd_dn5 = assign16410_e11273_d_n5;
        locals.var_pzbd_dn6 = assign16410_e11273_d_n6;
        locals.var_pzbd_dn7 = assign16410_e11273_d_n7;
        locals.var_pzbd_dn8 = assign16410_e11273_d_n8;
        locals.var_pzbd_dn9 = assign16410_e11273_d_n9;
        locals.var_pzbd_dn10 = assign16410_e11273_d_n10;
        locals.var_pzbd_dn11 = assign16410_e11273_d_n11;
        locals.var_pzbd_dn14 = assign16410_e11273_d_n14;

        let assign16420_e11280: f64 = if ((locals.var_pzbdsw < 0.01) && (p.p15 > locals.var_weff_nf)) { 1.0 } else { 0.0 };
        locals.var_guard344 = assign16420_e11280;

        let (assign16430_e11286, assign16430_e11286_d_n0, assign16430_e11286_d_n2, assign16430_e11286_d_n4, assign16430_e11286_d_n5, assign16430_e11286_d_n6, assign16430_e11286_d_n7, assign16430_e11286_d_n8, assign16430_e11286_d_n9, assign16430_e11286_d_n10, assign16430_e11286_d_n11, assign16430_e11286_d_n14,) = {
    if ((locals.var_guard293 != 0.0) && (locals.var_guard344 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbdsw, locals.var_pzbdsw_dn0, locals.var_pzbdsw_dn2, locals.var_pzbdsw_dn4, locals.var_pzbdsw_dn5, locals.var_pzbdsw_dn6, locals.var_pzbdsw_dn7, locals.var_pzbdsw_dn8, locals.var_pzbdsw_dn9, locals.var_pzbdsw_dn10, locals.var_pzbdsw_dn11, locals.var_pzbdsw_dn14,)
    }
};
        locals.var_pzbdsw = assign16430_e11286;
        locals.var_pzbdsw_dn0 = assign16430_e11286_d_n0;
        locals.var_pzbdsw_dn2 = assign16430_e11286_d_n2;
        locals.var_pzbdsw_dn4 = assign16430_e11286_d_n4;
        locals.var_pzbdsw_dn5 = assign16430_e11286_d_n5;
        locals.var_pzbdsw_dn6 = assign16430_e11286_d_n6;
        locals.var_pzbdsw_dn7 = assign16430_e11286_d_n7;
        locals.var_pzbdsw_dn8 = assign16430_e11286_d_n8;
        locals.var_pzbdsw_dn9 = assign16430_e11286_d_n9;
        locals.var_pzbdsw_dn10 = assign16430_e11286_d_n10;
        locals.var_pzbdsw_dn11 = assign16430_e11286_d_n11;
        locals.var_pzbdsw_dn14 = assign16430_e11286_d_n14;

        let assign16440_e11293: f64 = if ((locals.var_pzbdswg < 0.01) && (p.p15 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard345 = assign16440_e11293;

        let (assign16450_e11299, assign16450_e11299_d_n0, assign16450_e11299_d_n2, assign16450_e11299_d_n4, assign16450_e11299_d_n5, assign16450_e11299_d_n6, assign16450_e11299_d_n7, assign16450_e11299_d_n8, assign16450_e11299_d_n9, assign16450_e11299_d_n10, assign16450_e11299_d_n11, assign16450_e11299_d_n14,) = {
    if ((locals.var_guard293 != 0.0) && (locals.var_guard345 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbdswg, locals.var_pzbdswg_dn0, locals.var_pzbdswg_dn2, locals.var_pzbdswg_dn4, locals.var_pzbdswg_dn5, locals.var_pzbdswg_dn6, locals.var_pzbdswg_dn7, locals.var_pzbdswg_dn8, locals.var_pzbdswg_dn9, locals.var_pzbdswg_dn10, locals.var_pzbdswg_dn11, locals.var_pzbdswg_dn14,)
    }
};
        locals.var_pzbdswg = assign16450_e11299;
        locals.var_pzbdswg_dn0 = assign16450_e11299_d_n0;
        locals.var_pzbdswg_dn2 = assign16450_e11299_d_n2;
        locals.var_pzbdswg_dn4 = assign16450_e11299_d_n4;
        locals.var_pzbdswg_dn5 = assign16450_e11299_d_n5;
        locals.var_pzbdswg_dn6 = assign16450_e11299_d_n6;
        locals.var_pzbdswg_dn7 = assign16450_e11299_d_n7;
        locals.var_pzbdswg_dn8 = assign16450_e11299_d_n8;
        locals.var_pzbdswg_dn9 = assign16450_e11299_d_n9;
        locals.var_pzbdswg_dn10 = assign16450_e11299_d_n10;
        locals.var_pzbdswg_dn11 = assign16450_e11299_d_n11;
        locals.var_pzbdswg_dn14 = assign16450_e11299_d_n14;

        let (assign16460_e11311, assign16460_e11311_d_n0, assign16460_e11311_d_n2, assign16460_e11311_d_n4, assign16460_e11311_d_n5, assign16460_e11311_d_n6, assign16460_e11311_d_n7, assign16460_e11311_d_n8, assign16460_e11311_d_n9, assign16460_e11311_d_n10, assign16460_e11311_d_n11, assign16460_e11311_d_n14,) = {
    if (locals.var_guard293 != 0.0) {
        let assign16460_e11303: f64 = (p.p523 * p.p14);
        let assign16460_e11307: f64 = (p.p482 * locals.var_tdiff);
        let assign16460_e11308: f64 = (1.0 + assign16460_e11307);
        let assign16460_e11309: f64 = (assign16460_e11303 * assign16460_e11308);
        (assign16460_e11309, (assign16460_e11303 * (p.p482 * locals.var_tdiff_dn0)), (assign16460_e11303 * (p.p482 * locals.var_tdiff_dn2)), (assign16460_e11303 * (p.p482 * locals.var_tdiff_dn4)), (assign16460_e11303 * (p.p482 * locals.var_tdiff_dn5)), (assign16460_e11303 * (p.p482 * locals.var_tdiff_dn6)), (assign16460_e11303 * (p.p482 * locals.var_tdiff_dn7)), (assign16460_e11303 * (p.p482 * locals.var_tdiff_dn8)), (assign16460_e11303 * (p.p482 * locals.var_tdiff_dn9)), (assign16460_e11303 * (p.p482 * locals.var_tdiff_dn10)), (assign16460_e11303 * (p.p482 * locals.var_tdiff_dn11)), (assign16460_e11303 * (p.p482 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_czbs, locals.var_czbs_dn0, locals.var_czbs_dn2, locals.var_czbs_dn4, locals.var_czbs_dn5, locals.var_czbs_dn6, locals.var_czbs_dn7, locals.var_czbs_dn8, locals.var_czbs_dn9, locals.var_czbs_dn10, locals.var_czbs_dn11, locals.var_czbs_dn14,)
    }
};
        locals.var_czbs = assign16460_e11311;
        locals.var_czbs_dn0 = assign16460_e11311_d_n0;
        locals.var_czbs_dn2 = assign16460_e11311_d_n2;
        locals.var_czbs_dn4 = assign16460_e11311_d_n4;
        locals.var_czbs_dn5 = assign16460_e11311_d_n5;
        locals.var_czbs_dn6 = assign16460_e11311_d_n6;
        locals.var_czbs_dn7 = assign16460_e11311_d_n7;
        locals.var_czbs_dn8 = assign16460_e11311_d_n8;
        locals.var_czbs_dn9 = assign16460_e11311_d_n9;
        locals.var_czbs_dn10 = assign16460_e11311_d_n10;
        locals.var_czbs_dn11 = assign16460_e11311_d_n11;
        locals.var_czbs_dn14 = assign16460_e11311_d_n14;

        let assign16470_e11314: f64 = if p.p16 > locals.var_weff_nf { 1.0 } else { 0.0 };
        locals.var_guard346 = assign16470_e11314;

        let (assign16480_e11330, assign16480_e11330_d_n0, assign16480_e11330_d_n2, assign16480_e11330_d_n4, assign16480_e11330_d_n5, assign16480_e11330_d_n6, assign16480_e11330_d_n7, assign16480_e11330_d_n8, assign16480_e11330_d_n9, assign16480_e11330_d_n10, assign16480_e11330_d_n11, assign16480_e11330_d_n14,) = {
    if ((locals.var_guard293 != 0.0) && (locals.var_guard346 != 0.0)) {
        let assign16480_e11321: f64 = (p.p16 - locals.var_weff_nf);
        let assign16480_e11322: f64 = (p.p524 * assign16480_e11321);
        let assign16480_e11326: f64 = (p.p484 * locals.var_tdiff);
        let assign16480_e11327: f64 = (1.0 + assign16480_e11326);
        let assign16480_e11328: f64 = (assign16480_e11322 * assign16480_e11327);
        (assign16480_e11328, (assign16480_e11322 * (p.p484 * locals.var_tdiff_dn0)), (assign16480_e11322 * (p.p484 * locals.var_tdiff_dn2)), (assign16480_e11322 * (p.p484 * locals.var_tdiff_dn4)), (assign16480_e11322 * (p.p484 * locals.var_tdiff_dn5)), (assign16480_e11322 * (p.p484 * locals.var_tdiff_dn6)), (assign16480_e11322 * (p.p484 * locals.var_tdiff_dn7)), (assign16480_e11322 * (p.p484 * locals.var_tdiff_dn8)), (assign16480_e11322 * (p.p484 * locals.var_tdiff_dn9)), (assign16480_e11322 * (p.p484 * locals.var_tdiff_dn10)), (assign16480_e11322 * (p.p484 * locals.var_tdiff_dn11)), (assign16480_e11322 * (p.p484 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_czbssw, locals.var_czbssw_dn0, locals.var_czbssw_dn2, locals.var_czbssw_dn4, locals.var_czbssw_dn5, locals.var_czbssw_dn6, locals.var_czbssw_dn7, locals.var_czbssw_dn8, locals.var_czbssw_dn9, locals.var_czbssw_dn10, locals.var_czbssw_dn11, locals.var_czbssw_dn14,)
    }
};
        locals.var_czbssw = assign16480_e11330;
        locals.var_czbssw_dn0 = assign16480_e11330_d_n0;
        locals.var_czbssw_dn2 = assign16480_e11330_d_n2;
        locals.var_czbssw_dn4 = assign16480_e11330_d_n4;
        locals.var_czbssw_dn5 = assign16480_e11330_d_n5;
        locals.var_czbssw_dn6 = assign16480_e11330_d_n6;
        locals.var_czbssw_dn7 = assign16480_e11330_d_n7;
        locals.var_czbssw_dn8 = assign16480_e11330_d_n8;
        locals.var_czbssw_dn9 = assign16480_e11330_d_n9;
        locals.var_czbssw_dn10 = assign16480_e11330_d_n10;
        locals.var_czbssw_dn11 = assign16480_e11330_d_n11;
        locals.var_czbssw_dn14 = assign16480_e11330_d_n14;

        let (assign16490_e11344, assign16490_e11344_d_n0, assign16490_e11344_d_n2, assign16490_e11344_d_n4, assign16490_e11344_d_n5, assign16490_e11344_d_n6, assign16490_e11344_d_n7, assign16490_e11344_d_n8, assign16490_e11344_d_n9, assign16490_e11344_d_n10, assign16490_e11344_d_n11, assign16490_e11344_d_n14,) = {
    if ((locals.var_guard293 != 0.0) && (locals.var_guard346 != 0.0)) {
        let assign16490_e11336: f64 = (p.p525 * locals.var_weff_nf);
        let assign16490_e11340: f64 = (p.p486 * locals.var_tdiff);
        let assign16490_e11341: f64 = (1.0 + assign16490_e11340);
        let assign16490_e11342: f64 = (assign16490_e11336 * assign16490_e11341);
        (assign16490_e11342, (assign16490_e11336 * (p.p486 * locals.var_tdiff_dn0)), (assign16490_e11336 * (p.p486 * locals.var_tdiff_dn2)), (assign16490_e11336 * (p.p486 * locals.var_tdiff_dn4)), (assign16490_e11336 * (p.p486 * locals.var_tdiff_dn5)), (assign16490_e11336 * (p.p486 * locals.var_tdiff_dn6)), (assign16490_e11336 * (p.p486 * locals.var_tdiff_dn7)), (assign16490_e11336 * (p.p486 * locals.var_tdiff_dn8)), (assign16490_e11336 * (p.p486 * locals.var_tdiff_dn9)), (assign16490_e11336 * (p.p486 * locals.var_tdiff_dn10)), (assign16490_e11336 * (p.p486 * locals.var_tdiff_dn11)), (assign16490_e11336 * (p.p486 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_czbsswg, locals.var_czbsswg_dn0, locals.var_czbsswg_dn2, locals.var_czbsswg_dn4, locals.var_czbsswg_dn5, locals.var_czbsswg_dn6, locals.var_czbsswg_dn7, locals.var_czbsswg_dn8, locals.var_czbsswg_dn9, locals.var_czbsswg_dn10, locals.var_czbsswg_dn11, locals.var_czbsswg_dn14,)
    }
};
        locals.var_czbsswg = assign16490_e11344;
        locals.var_czbsswg_dn0 = assign16490_e11344_d_n0;
        locals.var_czbsswg_dn2 = assign16490_e11344_d_n2;
        locals.var_czbsswg_dn4 = assign16490_e11344_d_n4;
        locals.var_czbsswg_dn5 = assign16490_e11344_d_n5;
        locals.var_czbsswg_dn6 = assign16490_e11344_d_n6;
        locals.var_czbsswg_dn7 = assign16490_e11344_d_n7;
        locals.var_czbsswg_dn8 = assign16490_e11344_d_n8;
        locals.var_czbsswg_dn9 = assign16490_e11344_d_n9;
        locals.var_czbsswg_dn10 = assign16490_e11344_d_n10;
        locals.var_czbsswg_dn11 = assign16490_e11344_d_n11;
        locals.var_czbsswg_dn14 = assign16490_e11344_d_n14;

        let (assign16500_e11351, assign16500_e11351_d_n0, assign16500_e11351_d_n2, assign16500_e11351_d_n4, assign16500_e11351_d_n5, assign16500_e11351_d_n6, assign16500_e11351_d_n7, assign16500_e11351_d_n8, assign16500_e11351_d_n9, assign16500_e11351_d_n10, assign16500_e11351_d_n11, assign16500_e11351_d_n14,) = {
    if ((locals.var_guard293 != 0.0) && (locals.var_guard346 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbssw, locals.var_czbssw_dn0, locals.var_czbssw_dn2, locals.var_czbssw_dn4, locals.var_czbssw_dn5, locals.var_czbssw_dn6, locals.var_czbssw_dn7, locals.var_czbssw_dn8, locals.var_czbssw_dn9, locals.var_czbssw_dn10, locals.var_czbssw_dn11, locals.var_czbssw_dn14,)
    }
};
        locals.var_czbssw = assign16500_e11351;
        locals.var_czbssw_dn0 = assign16500_e11351_d_n0;
        locals.var_czbssw_dn2 = assign16500_e11351_d_n2;
        locals.var_czbssw_dn4 = assign16500_e11351_d_n4;
        locals.var_czbssw_dn5 = assign16500_e11351_d_n5;
        locals.var_czbssw_dn6 = assign16500_e11351_d_n6;
        locals.var_czbssw_dn7 = assign16500_e11351_d_n7;
        locals.var_czbssw_dn8 = assign16500_e11351_d_n8;
        locals.var_czbssw_dn9 = assign16500_e11351_d_n9;
        locals.var_czbssw_dn10 = assign16500_e11351_d_n10;
        locals.var_czbssw_dn11 = assign16500_e11351_d_n11;
        locals.var_czbssw_dn14 = assign16500_e11351_d_n14;

        let (assign16510_e11366, assign16510_e11366_d_n0, assign16510_e11366_d_n2, assign16510_e11366_d_n4, assign16510_e11366_d_n5, assign16510_e11366_d_n6, assign16510_e11366_d_n7, assign16510_e11366_d_n8, assign16510_e11366_d_n9, assign16510_e11366_d_n10, assign16510_e11366_d_n11, assign16510_e11366_d_n14,) = {
    if ((locals.var_guard293 != 0.0) && (locals.var_guard346 == 0.0)) {
        let assign16510_e11358: f64 = (p.p525 * p.p16);
        let assign16510_e11362: f64 = (p.p486 * locals.var_tdiff);
        let assign16510_e11363: f64 = (1.0 + assign16510_e11362);
        let assign16510_e11364: f64 = (assign16510_e11358 * assign16510_e11363);
        (assign16510_e11364, (assign16510_e11358 * (p.p486 * locals.var_tdiff_dn0)), (assign16510_e11358 * (p.p486 * locals.var_tdiff_dn2)), (assign16510_e11358 * (p.p486 * locals.var_tdiff_dn4)), (assign16510_e11358 * (p.p486 * locals.var_tdiff_dn5)), (assign16510_e11358 * (p.p486 * locals.var_tdiff_dn6)), (assign16510_e11358 * (p.p486 * locals.var_tdiff_dn7)), (assign16510_e11358 * (p.p486 * locals.var_tdiff_dn8)), (assign16510_e11358 * (p.p486 * locals.var_tdiff_dn9)), (assign16510_e11358 * (p.p486 * locals.var_tdiff_dn10)), (assign16510_e11358 * (p.p486 * locals.var_tdiff_dn11)), (assign16510_e11358 * (p.p486 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_czbsswg, locals.var_czbsswg_dn0, locals.var_czbsswg_dn2, locals.var_czbsswg_dn4, locals.var_czbsswg_dn5, locals.var_czbsswg_dn6, locals.var_czbsswg_dn7, locals.var_czbsswg_dn8, locals.var_czbsswg_dn9, locals.var_czbsswg_dn10, locals.var_czbsswg_dn11, locals.var_czbsswg_dn14,)
    }
};
        locals.var_czbsswg = assign16510_e11366;
        locals.var_czbsswg_dn0 = assign16510_e11366_d_n0;
        locals.var_czbsswg_dn2 = assign16510_e11366_d_n2;
        locals.var_czbsswg_dn4 = assign16510_e11366_d_n4;
        locals.var_czbsswg_dn5 = assign16510_e11366_d_n5;
        locals.var_czbsswg_dn6 = assign16510_e11366_d_n6;
        locals.var_czbsswg_dn7 = assign16510_e11366_d_n7;
        locals.var_czbsswg_dn8 = assign16510_e11366_d_n8;
        locals.var_czbsswg_dn9 = assign16510_e11366_d_n9;
        locals.var_czbsswg_dn10 = assign16510_e11366_d_n10;
        locals.var_czbsswg_dn11 = assign16510_e11366_d_n11;
        locals.var_czbsswg_dn14 = assign16510_e11366_d_n14;

        let assign16520_e11369: f64 = if locals.var_czbs < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard347 = assign16520_e11369;

        let (assign16530_e11375, assign16530_e11375_d_n0, assign16530_e11375_d_n2, assign16530_e11375_d_n4, assign16530_e11375_d_n5, assign16530_e11375_d_n6, assign16530_e11375_d_n7, assign16530_e11375_d_n8, assign16530_e11375_d_n9, assign16530_e11375_d_n10, assign16530_e11375_d_n11, assign16530_e11375_d_n14,) = {
    if ((locals.var_guard293 != 0.0) && (locals.var_guard347 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbs, locals.var_czbs_dn0, locals.var_czbs_dn2, locals.var_czbs_dn4, locals.var_czbs_dn5, locals.var_czbs_dn6, locals.var_czbs_dn7, locals.var_czbs_dn8, locals.var_czbs_dn9, locals.var_czbs_dn10, locals.var_czbs_dn11, locals.var_czbs_dn14,)
    }
};
        locals.var_czbs = assign16530_e11375;
        locals.var_czbs_dn0 = assign16530_e11375_d_n0;
        locals.var_czbs_dn2 = assign16530_e11375_d_n2;
        locals.var_czbs_dn4 = assign16530_e11375_d_n4;
        locals.var_czbs_dn5 = assign16530_e11375_d_n5;
        locals.var_czbs_dn6 = assign16530_e11375_d_n6;
        locals.var_czbs_dn7 = assign16530_e11375_d_n7;
        locals.var_czbs_dn8 = assign16530_e11375_d_n8;
        locals.var_czbs_dn9 = assign16530_e11375_d_n9;
        locals.var_czbs_dn10 = assign16530_e11375_d_n10;
        locals.var_czbs_dn11 = assign16530_e11375_d_n11;
        locals.var_czbs_dn14 = assign16530_e11375_d_n14;

        let assign16540_e11378: f64 = if locals.var_czbssw < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard348 = assign16540_e11378;

        let (assign16550_e11384, assign16550_e11384_d_n0, assign16550_e11384_d_n2, assign16550_e11384_d_n4, assign16550_e11384_d_n5, assign16550_e11384_d_n6, assign16550_e11384_d_n7, assign16550_e11384_d_n8, assign16550_e11384_d_n9, assign16550_e11384_d_n10, assign16550_e11384_d_n11, assign16550_e11384_d_n14,) = {
    if ((locals.var_guard293 != 0.0) && (locals.var_guard348 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbssw, locals.var_czbssw_dn0, locals.var_czbssw_dn2, locals.var_czbssw_dn4, locals.var_czbssw_dn5, locals.var_czbssw_dn6, locals.var_czbssw_dn7, locals.var_czbssw_dn8, locals.var_czbssw_dn9, locals.var_czbssw_dn10, locals.var_czbssw_dn11, locals.var_czbssw_dn14,)
    }
};
        locals.var_czbssw = assign16550_e11384;
        locals.var_czbssw_dn0 = assign16550_e11384_d_n0;
        locals.var_czbssw_dn2 = assign16550_e11384_d_n2;
        locals.var_czbssw_dn4 = assign16550_e11384_d_n4;
        locals.var_czbssw_dn5 = assign16550_e11384_d_n5;
        locals.var_czbssw_dn6 = assign16550_e11384_d_n6;
        locals.var_czbssw_dn7 = assign16550_e11384_d_n7;
        locals.var_czbssw_dn8 = assign16550_e11384_d_n8;
        locals.var_czbssw_dn9 = assign16550_e11384_d_n9;
        locals.var_czbssw_dn10 = assign16550_e11384_d_n10;
        locals.var_czbssw_dn11 = assign16550_e11384_d_n11;
        locals.var_czbssw_dn14 = assign16550_e11384_d_n14;

        let assign16560_e11387: f64 = if locals.var_czbsswg < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard349 = assign16560_e11387;

        let (assign16570_e11393, assign16570_e11393_d_n0, assign16570_e11393_d_n2, assign16570_e11393_d_n4, assign16570_e11393_d_n5, assign16570_e11393_d_n6, assign16570_e11393_d_n7, assign16570_e11393_d_n8, assign16570_e11393_d_n9, assign16570_e11393_d_n10, assign16570_e11393_d_n11, assign16570_e11393_d_n14,) = {
    if ((locals.var_guard293 != 0.0) && (locals.var_guard349 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbsswg, locals.var_czbsswg_dn0, locals.var_czbsswg_dn2, locals.var_czbsswg_dn4, locals.var_czbsswg_dn5, locals.var_czbsswg_dn6, locals.var_czbsswg_dn7, locals.var_czbsswg_dn8, locals.var_czbsswg_dn9, locals.var_czbsswg_dn10, locals.var_czbsswg_dn11, locals.var_czbsswg_dn14,)
    }
};
        locals.var_czbsswg = assign16570_e11393;
        locals.var_czbsswg_dn0 = assign16570_e11393_d_n0;
        locals.var_czbsswg_dn2 = assign16570_e11393_d_n2;
        locals.var_czbsswg_dn4 = assign16570_e11393_d_n4;
        locals.var_czbsswg_dn5 = assign16570_e11393_d_n5;
        locals.var_czbsswg_dn6 = assign16570_e11393_d_n6;
        locals.var_czbsswg_dn7 = assign16570_e11393_d_n7;
        locals.var_czbsswg_dn8 = assign16570_e11393_d_n8;
        locals.var_czbsswg_dn9 = assign16570_e11393_d_n9;
        locals.var_czbsswg_dn10 = assign16570_e11393_d_n10;
        locals.var_czbsswg_dn11 = assign16570_e11393_d_n11;
        locals.var_czbsswg_dn14 = assign16570_e11393_d_n14;

        let (assign16580_e11401, assign16580_e11401_d_n0, assign16580_e11401_d_n2, assign16580_e11401_d_n4, assign16580_e11401_d_n5, assign16580_e11401_d_n6, assign16580_e11401_d_n7, assign16580_e11401_d_n8, assign16580_e11401_d_n9, assign16580_e11401_d_n10, assign16580_e11401_d_n11, assign16580_e11401_d_n14,) = {
    if (locals.var_guard293 != 0.0) {
        let assign16580_e11398: f64 = (p.p488 * locals.var_tdiff);
        let assign16580_e11399: f64 = (p.p529 - assign16580_e11398);
        (assign16580_e11399, (-(p.p488 * locals.var_tdiff_dn0)), (-(p.p488 * locals.var_tdiff_dn2)), (-(p.p488 * locals.var_tdiff_dn4)), (-(p.p488 * locals.var_tdiff_dn5)), (-(p.p488 * locals.var_tdiff_dn6)), (-(p.p488 * locals.var_tdiff_dn7)), (-(p.p488 * locals.var_tdiff_dn8)), (-(p.p488 * locals.var_tdiff_dn9)), (-(p.p488 * locals.var_tdiff_dn10)), (-(p.p488 * locals.var_tdiff_dn11)), (-(p.p488 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_pzbs, locals.var_pzbs_dn0, locals.var_pzbs_dn2, locals.var_pzbs_dn4, locals.var_pzbs_dn5, locals.var_pzbs_dn6, locals.var_pzbs_dn7, locals.var_pzbs_dn8, locals.var_pzbs_dn9, locals.var_pzbs_dn10, locals.var_pzbs_dn11, locals.var_pzbs_dn14,)
    }
};
        locals.var_pzbs = assign16580_e11401;
        locals.var_pzbs_dn0 = assign16580_e11401_d_n0;
        locals.var_pzbs_dn2 = assign16580_e11401_d_n2;
        locals.var_pzbs_dn4 = assign16580_e11401_d_n4;
        locals.var_pzbs_dn5 = assign16580_e11401_d_n5;
        locals.var_pzbs_dn6 = assign16580_e11401_d_n6;
        locals.var_pzbs_dn7 = assign16580_e11401_d_n7;
        locals.var_pzbs_dn8 = assign16580_e11401_d_n8;
        locals.var_pzbs_dn9 = assign16580_e11401_d_n9;
        locals.var_pzbs_dn10 = assign16580_e11401_d_n10;
        locals.var_pzbs_dn11 = assign16580_e11401_d_n11;
        locals.var_pzbs_dn14 = assign16580_e11401_d_n14;

        let (assign16590_e11409, assign16590_e11409_d_n0, assign16590_e11409_d_n2, assign16590_e11409_d_n4, assign16590_e11409_d_n5, assign16590_e11409_d_n6, assign16590_e11409_d_n7, assign16590_e11409_d_n8, assign16590_e11409_d_n9, assign16590_e11409_d_n10, assign16590_e11409_d_n11, assign16590_e11409_d_n14,) = {
    if (locals.var_guard293 != 0.0) {
        let assign16590_e11406: f64 = (p.p490 * locals.var_tdiff);
        let assign16590_e11407: f64 = (p.p530 - assign16590_e11406);
        (assign16590_e11407, (-(p.p490 * locals.var_tdiff_dn0)), (-(p.p490 * locals.var_tdiff_dn2)), (-(p.p490 * locals.var_tdiff_dn4)), (-(p.p490 * locals.var_tdiff_dn5)), (-(p.p490 * locals.var_tdiff_dn6)), (-(p.p490 * locals.var_tdiff_dn7)), (-(p.p490 * locals.var_tdiff_dn8)), (-(p.p490 * locals.var_tdiff_dn9)), (-(p.p490 * locals.var_tdiff_dn10)), (-(p.p490 * locals.var_tdiff_dn11)), (-(p.p490 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_pzbssw, locals.var_pzbssw_dn0, locals.var_pzbssw_dn2, locals.var_pzbssw_dn4, locals.var_pzbssw_dn5, locals.var_pzbssw_dn6, locals.var_pzbssw_dn7, locals.var_pzbssw_dn8, locals.var_pzbssw_dn9, locals.var_pzbssw_dn10, locals.var_pzbssw_dn11, locals.var_pzbssw_dn14,)
    }
};
        locals.var_pzbssw = assign16590_e11409;
        locals.var_pzbssw_dn0 = assign16590_e11409_d_n0;
        locals.var_pzbssw_dn2 = assign16590_e11409_d_n2;
        locals.var_pzbssw_dn4 = assign16590_e11409_d_n4;
        locals.var_pzbssw_dn5 = assign16590_e11409_d_n5;
        locals.var_pzbssw_dn6 = assign16590_e11409_d_n6;
        locals.var_pzbssw_dn7 = assign16590_e11409_d_n7;
        locals.var_pzbssw_dn8 = assign16590_e11409_d_n8;
        locals.var_pzbssw_dn9 = assign16590_e11409_d_n9;
        locals.var_pzbssw_dn10 = assign16590_e11409_d_n10;
        locals.var_pzbssw_dn11 = assign16590_e11409_d_n11;
        locals.var_pzbssw_dn14 = assign16590_e11409_d_n14;

        let (assign16600_e11417, assign16600_e11417_d_n0, assign16600_e11417_d_n2, assign16600_e11417_d_n4, assign16600_e11417_d_n5, assign16600_e11417_d_n6, assign16600_e11417_d_n7, assign16600_e11417_d_n8, assign16600_e11417_d_n9, assign16600_e11417_d_n10, assign16600_e11417_d_n11, assign16600_e11417_d_n14,) = {
    if (locals.var_guard293 != 0.0) {
        let assign16600_e11414: f64 = (p.p492 * locals.var_tdiff);
        let assign16600_e11415: f64 = (p.p531 - assign16600_e11414);
        (assign16600_e11415, (-(p.p492 * locals.var_tdiff_dn0)), (-(p.p492 * locals.var_tdiff_dn2)), (-(p.p492 * locals.var_tdiff_dn4)), (-(p.p492 * locals.var_tdiff_dn5)), (-(p.p492 * locals.var_tdiff_dn6)), (-(p.p492 * locals.var_tdiff_dn7)), (-(p.p492 * locals.var_tdiff_dn8)), (-(p.p492 * locals.var_tdiff_dn9)), (-(p.p492 * locals.var_tdiff_dn10)), (-(p.p492 * locals.var_tdiff_dn11)), (-(p.p492 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_pzbsswg, locals.var_pzbsswg_dn0, locals.var_pzbsswg_dn2, locals.var_pzbsswg_dn4, locals.var_pzbsswg_dn5, locals.var_pzbsswg_dn6, locals.var_pzbsswg_dn7, locals.var_pzbsswg_dn8, locals.var_pzbsswg_dn9, locals.var_pzbsswg_dn10, locals.var_pzbsswg_dn11, locals.var_pzbsswg_dn14,)
    }
};
        locals.var_pzbsswg = assign16600_e11417;
        locals.var_pzbsswg_dn0 = assign16600_e11417_d_n0;
        locals.var_pzbsswg_dn2 = assign16600_e11417_d_n2;
        locals.var_pzbsswg_dn4 = assign16600_e11417_d_n4;
        locals.var_pzbsswg_dn5 = assign16600_e11417_d_n5;
        locals.var_pzbsswg_dn6 = assign16600_e11417_d_n6;
        locals.var_pzbsswg_dn7 = assign16600_e11417_d_n7;
        locals.var_pzbsswg_dn8 = assign16600_e11417_d_n8;
        locals.var_pzbsswg_dn9 = assign16600_e11417_d_n9;
        locals.var_pzbsswg_dn10 = assign16600_e11417_d_n10;
        locals.var_pzbsswg_dn11 = assign16600_e11417_d_n11;
        locals.var_pzbsswg_dn14 = assign16600_e11417_d_n14;

        let assign16610_e11424: f64 = if ((locals.var_pzbs < 0.01) && (p.p14 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard350 = assign16610_e11424;

        let (assign16620_e11430, assign16620_e11430_d_n0, assign16620_e11430_d_n2, assign16620_e11430_d_n4, assign16620_e11430_d_n5, assign16620_e11430_d_n6, assign16620_e11430_d_n7, assign16620_e11430_d_n8, assign16620_e11430_d_n9, assign16620_e11430_d_n10, assign16620_e11430_d_n11, assign16620_e11430_d_n14,) = {
    if ((locals.var_guard293 != 0.0) && (locals.var_guard350 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbs, locals.var_pzbs_dn0, locals.var_pzbs_dn2, locals.var_pzbs_dn4, locals.var_pzbs_dn5, locals.var_pzbs_dn6, locals.var_pzbs_dn7, locals.var_pzbs_dn8, locals.var_pzbs_dn9, locals.var_pzbs_dn10, locals.var_pzbs_dn11, locals.var_pzbs_dn14,)
    }
};
        locals.var_pzbs = assign16620_e11430;
        locals.var_pzbs_dn0 = assign16620_e11430_d_n0;
        locals.var_pzbs_dn2 = assign16620_e11430_d_n2;
        locals.var_pzbs_dn4 = assign16620_e11430_d_n4;
        locals.var_pzbs_dn5 = assign16620_e11430_d_n5;
        locals.var_pzbs_dn6 = assign16620_e11430_d_n6;
        locals.var_pzbs_dn7 = assign16620_e11430_d_n7;
        locals.var_pzbs_dn8 = assign16620_e11430_d_n8;
        locals.var_pzbs_dn9 = assign16620_e11430_d_n9;
        locals.var_pzbs_dn10 = assign16620_e11430_d_n10;
        locals.var_pzbs_dn11 = assign16620_e11430_d_n11;
        locals.var_pzbs_dn14 = assign16620_e11430_d_n14;

        let assign16630_e11437: f64 = if ((locals.var_pzbssw < 0.01) && (p.p16 > locals.var_weff_nf)) { 1.0 } else { 0.0 };
        locals.var_guard351 = assign16630_e11437;

        let (assign16640_e11443, assign16640_e11443_d_n0, assign16640_e11443_d_n2, assign16640_e11443_d_n4, assign16640_e11443_d_n5, assign16640_e11443_d_n6, assign16640_e11443_d_n7, assign16640_e11443_d_n8, assign16640_e11443_d_n9, assign16640_e11443_d_n10, assign16640_e11443_d_n11, assign16640_e11443_d_n14,) = {
    if ((locals.var_guard293 != 0.0) && (locals.var_guard351 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbssw, locals.var_pzbssw_dn0, locals.var_pzbssw_dn2, locals.var_pzbssw_dn4, locals.var_pzbssw_dn5, locals.var_pzbssw_dn6, locals.var_pzbssw_dn7, locals.var_pzbssw_dn8, locals.var_pzbssw_dn9, locals.var_pzbssw_dn10, locals.var_pzbssw_dn11, locals.var_pzbssw_dn14,)
    }
};
        locals.var_pzbssw = assign16640_e11443;
        locals.var_pzbssw_dn0 = assign16640_e11443_d_n0;
        locals.var_pzbssw_dn2 = assign16640_e11443_d_n2;
        locals.var_pzbssw_dn4 = assign16640_e11443_d_n4;
        locals.var_pzbssw_dn5 = assign16640_e11443_d_n5;
        locals.var_pzbssw_dn6 = assign16640_e11443_d_n6;
        locals.var_pzbssw_dn7 = assign16640_e11443_d_n7;
        locals.var_pzbssw_dn8 = assign16640_e11443_d_n8;
        locals.var_pzbssw_dn9 = assign16640_e11443_d_n9;
        locals.var_pzbssw_dn10 = assign16640_e11443_d_n10;
        locals.var_pzbssw_dn11 = assign16640_e11443_d_n11;
        locals.var_pzbssw_dn14 = assign16640_e11443_d_n14;

        let assign16650_e11450: f64 = if ((locals.var_pzbsswg < 0.01) && (p.p16 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard352 = assign16650_e11450;

        let (assign16660_e11456, assign16660_e11456_d_n0, assign16660_e11456_d_n2, assign16660_e11456_d_n4, assign16660_e11456_d_n5, assign16660_e11456_d_n6, assign16660_e11456_d_n7, assign16660_e11456_d_n8, assign16660_e11456_d_n9, assign16660_e11456_d_n10, assign16660_e11456_d_n11, assign16660_e11456_d_n14,) = {
    if ((locals.var_guard293 != 0.0) && (locals.var_guard352 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbsswg, locals.var_pzbsswg_dn0, locals.var_pzbsswg_dn2, locals.var_pzbsswg_dn4, locals.var_pzbsswg_dn5, locals.var_pzbsswg_dn6, locals.var_pzbsswg_dn7, locals.var_pzbsswg_dn8, locals.var_pzbsswg_dn9, locals.var_pzbsswg_dn10, locals.var_pzbsswg_dn11, locals.var_pzbsswg_dn14,)
    }
};
        locals.var_pzbsswg = assign16660_e11456;
        locals.var_pzbsswg_dn0 = assign16660_e11456_d_n0;
        locals.var_pzbsswg_dn2 = assign16660_e11456_d_n2;
        locals.var_pzbsswg_dn4 = assign16660_e11456_d_n4;
        locals.var_pzbsswg_dn5 = assign16660_e11456_d_n5;
        locals.var_pzbsswg_dn6 = assign16660_e11456_d_n6;
        locals.var_pzbsswg_dn7 = assign16660_e11456_d_n7;
        locals.var_pzbsswg_dn8 = assign16660_e11456_d_n8;
        locals.var_pzbsswg_dn9 = assign16660_e11456_d_n9;
        locals.var_pzbsswg_dn10 = assign16660_e11456_d_n10;
        locals.var_pzbsswg_dn11 = assign16660_e11456_d_n11;
        locals.var_pzbsswg_dn14 = assign16660_e11456_d_n14;

        let assign16670_e11459: f64 = (p.p87 * (nv6 - nv8));
        locals.var_vdsi = assign16670_e11459;
        locals.var_vdsi_dn6 = p.p87;
        locals.var_vdsi_dn8 = (-p.p87);

        let assign16680_e11462: f64 = (p.p87 * (nv7 - nv8));
        locals.var_vgsi = assign16680_e11462;
        locals.var_vgsi_dn7 = p.p87;
        locals.var_vgsi_dn8 = (-p.p87);

        let assign16690_e11465: f64 = (p.p87 * (nv9 - nv8));
        locals.var_vbsi = assign16690_e11465;
        locals.var_vbsi_dn8 = (-p.p87);
        locals.var_vbsi_dn9 = p.p87;

        let assign16700_e11468: f64 = (p.p87 * (nv0 - nv2));
        locals.var_vdsei = assign16700_e11468;
        locals.var_vdsei_dn0 = p.p87;
        locals.var_vdsei_dn2 = (-p.p87);

        let assign16710_e11471: f64 = (p.p87 * (nv7 - nv2));
        locals.var_vgsei = assign16710_e11471;
        locals.var_vgsei_dn2 = (-p.p87);
        locals.var_vgsei_dn7 = p.p87;

        let assign16720_e11474: f64 = (p.p87 * (nv9 - nv2));
        locals.var_vbsei = assign16720_e11474;
        locals.var_vbsei_dn2 = (-p.p87);
        locals.var_vbsei_dn9 = p.p87;

        let assign16730_e11477: f64 = (p.p87 * (nv0 - nv6));
        locals.var_vddp = assign16730_e11477;
        locals.var_vddp_dn0 = p.p87;
        locals.var_vddp_dn6 = (-p.p87);

        let assign16740_e11480: f64 = (p.p87 * (nv8 - nv2));
        locals.var_vsps = assign16740_e11480;
        locals.var_vsps_dn2 = (-p.p87);
        locals.var_vsps_dn8 = p.p87;

        let assign16750_e11483: f64 = (p.p87 * (nv11 - nv2));
        locals.var_vsbs = assign16750_e11483;
        locals.var_vsbs_dn2 = (-p.p87);
        locals.var_vsbs_dn11 = p.p87;

        let assign16760_e11486: f64 = (p.p87 * (nv10 - nv0));
        locals.var_vdbd = assign16760_e11486;
        locals.var_vdbd_dn0 = (-p.p87);
        locals.var_vdbd_dn10 = p.p87;

        let assign16770_e11489: f64 = (p.p87 * (nv9 - nv8));
        locals.var_vbpsp = assign16770_e11489;
        locals.var_vbpsp_dn8 = (-p.p87);
        locals.var_vbpsp_dn9 = p.p87;

        let assign16780_e11492: f64 = (p.p87 * (nv9 - nv6));
        locals.var_vbpdp = assign16780_e11492;
        locals.var_vbpdp_dn6 = (-p.p87);
        locals.var_vbpdp_dn9 = p.p87;

        locals.var_vbs_jct = locals.var_vsbs;
        locals.var_vbs_jct_dn2 = locals.var_vsbs_dn2;
        locals.var_vbs_jct_dn11 = locals.var_vsbs_dn11;

        locals.var_vbd_jct = locals.var_vdbd;
        locals.var_vbd_jct_dn0 = locals.var_vdbd_dn0;
        locals.var_vbd_jct_dn10 = locals.var_vdbd_dn10;

        locals.var_vbsi_jct = locals.var_vbpsp;
        locals.var_vbsi_jct_dn8 = locals.var_vbpsp_dn8;
        locals.var_vbsi_jct_dn9 = locals.var_vbpsp_dn9;

        locals.var_vbdi_jct = locals.var_vbpdp;
        locals.var_vbdi_jct_dn6 = locals.var_vbpdp_dn6;
        locals.var_vbdi_jct_dn9 = locals.var_vbpdp_dn9;

        let assign16830_e11499: f64 = (p.p87 * (nv4 - nv2));
        locals.var_vsubs = assign16830_e11499;
        locals.var_vsubs_dn2 = (-p.p87);
        locals.var_vsubs_dn4 = p.p87;

        let (assign16840_e11503, assign16840_e11503_d_n12,) = {
    if (locals.var_flg_nqs != 0.0) {
        ((nv12 - 0.0), 1.0,)
    } else {
        (locals.var_qi_nqs, locals.var_qi_nqs_dn12,)
    }
};
        locals.var_qi_nqs = assign16840_e11503;
        locals.var_qi_nqs_dn12 = assign16840_e11503_d_n12;

    }

    pub(super) fn stamp_transient_block_36(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let (assign16850_e11507, assign16850_e11507_d_n13,) = {
    if (locals.var_flg_nqs != 0.0) {
        ((nv13 - 0.0), 1.0,)
    } else {
        (locals.var_qb_nqs, locals.var_qb_nqs_dn13,)
    }
};
        locals.var_qb_nqs = assign16850_e11507;
        locals.var_qb_nqs_dn13 = assign16850_e11507_d_n13;

        let (assign16860_e11512, assign16860_e11512_d_n12,) = {
    if (locals.var_flg_nqs == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_qi_nqs, locals.var_qi_nqs_dn12,)
    }
};
        locals.var_qi_nqs = assign16860_e11512;
        locals.var_qi_nqs_dn12 = assign16860_e11512_d_n12;

        let (assign16870_e11517, assign16870_e11517_d_n13,) = {
    if (locals.var_flg_nqs == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_qb_nqs, locals.var_qb_nqs_dn13,)
    }
};
        locals.var_qb_nqs = assign16870_e11517;
        locals.var_qb_nqs_dn13 = assign16870_e11517_d_n13;

        let assign16880_e11520: f64 = (locals.var_vgsi - locals.var_vdsi);
        locals.var_vgd = assign16880_e11520;
        locals.var_vgd_dn6 = (-locals.var_vdsi_dn6);
        locals.var_vgd_dn7 = locals.var_vgsi_dn7;
        locals.var_vgd_dn8 = (locals.var_vgsi_dn8 - locals.var_vdsi_dn8);

        let assign16890_e11523: f64 = (locals.var_vbsi - locals.var_vdsi);
        locals.var_vbd = assign16890_e11523;
        locals.var_vbd_dn6 = (-locals.var_vdsi_dn6);
        locals.var_vbd_dn8 = (locals.var_vbsi_dn8 - locals.var_vdsi_dn8);
        locals.var_vbd_dn9 = locals.var_vbsi_dn9;

        let assign16900_e11526: f64 = if locals.var_vdsi >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard353 = assign16900_e11526;

        let (assign16910_e11530,) = {
    if (locals.var_guard353 != 0.0) {
        (1.0,)
    } else {
        (locals.var_mode,)
    }
};
        locals.var_mode = assign16910_e11530;

        let (assign16920_e11534, assign16920_e11534_d_n0, assign16920_e11534_d_n2, assign16920_e11534_d_n4, assign16920_e11534_d_n5, assign16920_e11534_d_n6, assign16920_e11534_d_n7, assign16920_e11534_d_n8, assign16920_e11534_d_n9, assign16920_e11534_d_n10, assign16920_e11534_d_n11, assign16920_e11534_d_n14,) = {
    if (locals.var_guard353 != 0.0) {
        (locals.var_vdsi, 0.0, 0.0, 0.0, 0.0, locals.var_vdsi_dn6, 0.0, locals.var_vdsi_dn8, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn14,)
    }
};
        locals.var_vds = assign16920_e11534;
        locals.var_vds_dn0 = assign16920_e11534_d_n0;
        locals.var_vds_dn2 = assign16920_e11534_d_n2;
        locals.var_vds_dn4 = assign16920_e11534_d_n4;
        locals.var_vds_dn5 = assign16920_e11534_d_n5;
        locals.var_vds_dn6 = assign16920_e11534_d_n6;
        locals.var_vds_dn7 = assign16920_e11534_d_n7;
        locals.var_vds_dn8 = assign16920_e11534_d_n8;
        locals.var_vds_dn9 = assign16920_e11534_d_n9;
        locals.var_vds_dn10 = assign16920_e11534_d_n10;
        locals.var_vds_dn11 = assign16920_e11534_d_n11;
        locals.var_vds_dn14 = assign16920_e11534_d_n14;

        let (assign16930_e11538, assign16930_e11538_d_n6, assign16930_e11538_d_n7, assign16930_e11538_d_n8,) = {
    if (locals.var_guard353 != 0.0) {
        (locals.var_vgsi, 0.0, locals.var_vgsi_dn7, locals.var_vgsi_dn8,)
    } else {
        (locals.var_vgs, locals.var_vgs_dn6, locals.var_vgs_dn7, locals.var_vgs_dn8,)
    }
};
        locals.var_vgs = assign16930_e11538;
        locals.var_vgs_dn6 = assign16930_e11538_d_n6;
        locals.var_vgs_dn7 = assign16930_e11538_d_n7;
        locals.var_vgs_dn8 = assign16930_e11538_d_n8;

        let (assign16940_e11542, assign16940_e11542_d_n6, assign16940_e11542_d_n8, assign16940_e11542_d_n9,) = {
    if (locals.var_guard353 != 0.0) {
        (locals.var_vbsi, 0.0, locals.var_vbsi_dn8, locals.var_vbsi_dn9,)
    } else {
        (locals.var_vbs, locals.var_vbs_dn6, locals.var_vbs_dn8, locals.var_vbs_dn9,)
    }
};
        locals.var_vbs = assign16940_e11542;
        locals.var_vbs_dn6 = assign16940_e11542_d_n6;
        locals.var_vbs_dn8 = assign16940_e11542_d_n8;
        locals.var_vbs_dn9 = assign16940_e11542_d_n9;

        let (assign16950_e11546, assign16950_e11546_d_n0, assign16950_e11546_d_n2,) = {
    if (locals.var_guard353 != 0.0) {
        (locals.var_vdsei, locals.var_vdsei_dn0, locals.var_vdsei_dn2,)
    } else {
        (locals.var_vdse, locals.var_vdse_dn0, locals.var_vdse_dn2,)
    }
};
        locals.var_vdse = assign16950_e11546;
        locals.var_vdse_dn0 = assign16950_e11546_d_n0;
        locals.var_vdse_dn2 = assign16950_e11546_d_n2;

        let (assign16960_e11550, assign16960_e11550_d_n0, assign16960_e11550_d_n2, assign16960_e11550_d_n7,) = {
    if (locals.var_guard353 != 0.0) {
        (locals.var_vgsei, 0.0, locals.var_vgsei_dn2, locals.var_vgsei_dn7,)
    } else {
        (locals.var_vgse, locals.var_vgse_dn0, locals.var_vgse_dn2, locals.var_vgse_dn7,)
    }
};
        locals.var_vgse = assign16960_e11550;
        locals.var_vgse_dn0 = assign16960_e11550_d_n0;
        locals.var_vgse_dn2 = assign16960_e11550_d_n2;
        locals.var_vgse_dn7 = assign16960_e11550_d_n7;

        let (assign16970_e11554, assign16970_e11554_d_n0, assign16970_e11554_d_n2, assign16970_e11554_d_n9,) = {
    if (locals.var_guard353 != 0.0) {
        (locals.var_vbsei, 0.0, locals.var_vbsei_dn2, locals.var_vbsei_dn9,)
    } else {
        (locals.var_vbse, locals.var_vbse_dn0, locals.var_vbse_dn2, locals.var_vbse_dn9,)
    }
};
        locals.var_vbse = assign16970_e11554;
        locals.var_vbse_dn0 = assign16970_e11554_d_n0;
        locals.var_vbse_dn2 = assign16970_e11554_d_n2;
        locals.var_vbse_dn9 = assign16970_e11554_d_n9;

        let (assign16980_e11560,) = {
    if (locals.var_guard353 == 0.0) {
        let assign16980_e11558: f64 = (-1.0);
        (assign16980_e11558,)
    } else {
        (locals.var_mode,)
    }
};
        locals.var_mode = assign16980_e11560;

        let (assign16990_e11566, assign16990_e11566_d_n0, assign16990_e11566_d_n2, assign16990_e11566_d_n4, assign16990_e11566_d_n5, assign16990_e11566_d_n6, assign16990_e11566_d_n7, assign16990_e11566_d_n8, assign16990_e11566_d_n9, assign16990_e11566_d_n10, assign16990_e11566_d_n11, assign16990_e11566_d_n14,) = {
    if (locals.var_guard353 == 0.0) {
        let assign16990_e11564: f64 = (-locals.var_vdsi);
        (assign16990_e11564, 0.0, 0.0, 0.0, 0.0, (-locals.var_vdsi_dn6), 0.0, (-locals.var_vdsi_dn8), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn14,)
    }
};
        locals.var_vds = assign16990_e11566;
        locals.var_vds_dn0 = assign16990_e11566_d_n0;
        locals.var_vds_dn2 = assign16990_e11566_d_n2;
        locals.var_vds_dn4 = assign16990_e11566_d_n4;
        locals.var_vds_dn5 = assign16990_e11566_d_n5;
        locals.var_vds_dn6 = assign16990_e11566_d_n6;
        locals.var_vds_dn7 = assign16990_e11566_d_n7;
        locals.var_vds_dn8 = assign16990_e11566_d_n8;
        locals.var_vds_dn9 = assign16990_e11566_d_n9;
        locals.var_vds_dn10 = assign16990_e11566_d_n10;
        locals.var_vds_dn11 = assign16990_e11566_d_n11;
        locals.var_vds_dn14 = assign16990_e11566_d_n14;

        let (assign17000_e11571, assign17000_e11571_d_n6, assign17000_e11571_d_n7, assign17000_e11571_d_n8,) = {
    if (locals.var_guard353 == 0.0) {
        (locals.var_vgd, locals.var_vgd_dn6, locals.var_vgd_dn7, locals.var_vgd_dn8,)
    } else {
        (locals.var_vgs, locals.var_vgs_dn6, locals.var_vgs_dn7, locals.var_vgs_dn8,)
    }
};
        locals.var_vgs = assign17000_e11571;
        locals.var_vgs_dn6 = assign17000_e11571_d_n6;
        locals.var_vgs_dn7 = assign17000_e11571_d_n7;
        locals.var_vgs_dn8 = assign17000_e11571_d_n8;

        let (assign17010_e11576, assign17010_e11576_d_n6, assign17010_e11576_d_n8, assign17010_e11576_d_n9,) = {
    if (locals.var_guard353 == 0.0) {
        (locals.var_vbd, locals.var_vbd_dn6, locals.var_vbd_dn8, locals.var_vbd_dn9,)
    } else {
        (locals.var_vbs, locals.var_vbs_dn6, locals.var_vbs_dn8, locals.var_vbs_dn9,)
    }
};
        locals.var_vbs = assign17010_e11576;
        locals.var_vbs_dn6 = assign17010_e11576_d_n6;
        locals.var_vbs_dn8 = assign17010_e11576_d_n8;
        locals.var_vbs_dn9 = assign17010_e11576_d_n9;

        let (assign17020_e11582, assign17020_e11582_d_n0, assign17020_e11582_d_n2,) = {
    if (locals.var_guard353 == 0.0) {
        let assign17020_e11580: f64 = (-locals.var_vdsei);
        (assign17020_e11580, (-locals.var_vdsei_dn0), (-locals.var_vdsei_dn2),)
    } else {
        (locals.var_vdse, locals.var_vdse_dn0, locals.var_vdse_dn2,)
    }
};
        locals.var_vdse = assign17020_e11582;
        locals.var_vdse_dn0 = assign17020_e11582_d_n0;
        locals.var_vdse_dn2 = assign17020_e11582_d_n2;

        let (assign17030_e11589, assign17030_e11589_d_n0, assign17030_e11589_d_n2, assign17030_e11589_d_n7,) = {
    if (locals.var_guard353 == 0.0) {
        let assign17030_e11587: f64 = (locals.var_vgsei - locals.var_vdsei);
        (assign17030_e11587, (-locals.var_vdsei_dn0), (locals.var_vgsei_dn2 - locals.var_vdsei_dn2), locals.var_vgsei_dn7,)
    } else {
        (locals.var_vgse, locals.var_vgse_dn0, locals.var_vgse_dn2, locals.var_vgse_dn7,)
    }
};
        locals.var_vgse = assign17030_e11589;
        locals.var_vgse_dn0 = assign17030_e11589_d_n0;
        locals.var_vgse_dn2 = assign17030_e11589_d_n2;
        locals.var_vgse_dn7 = assign17030_e11589_d_n7;

        let (assign17040_e11596, assign17040_e11596_d_n0, assign17040_e11596_d_n2, assign17040_e11596_d_n9,) = {
    if (locals.var_guard353 == 0.0) {
        let assign17040_e11594: f64 = (locals.var_vbsei - locals.var_vdsei);
        (assign17040_e11594, (-locals.var_vdsei_dn0), (locals.var_vbsei_dn2 - locals.var_vdsei_dn2), locals.var_vbsei_dn9,)
    } else {
        (locals.var_vbse, locals.var_vbse_dn0, locals.var_vbse_dn2, locals.var_vbse_dn9,)
    }
};
        locals.var_vbse = assign17040_e11596;
        locals.var_vbse_dn0 = assign17040_e11596_d_n0;
        locals.var_vbse_dn2 = assign17040_e11596_d_n2;
        locals.var_vbse_dn9 = assign17040_e11596_d_n9;

        let assign17070_e11609: f64 = if ((p.p53 > 0.0) && (locals.var_uc_rth0 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard356 = assign17070_e11609;

        let (assign17080_e11613, assign17080_e11613_d_n0, assign17080_e11613_d_n2, assign17080_e11613_d_n4, assign17080_e11613_d_n5, assign17080_e11613_d_n6, assign17080_e11613_d_n7, assign17080_e11613_d_n8, assign17080_e11613_d_n9, assign17080_e11613_d_n10, assign17080_e11613_d_n11, assign17080_e11613_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        ((nv5 - 0.0), 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_deltemp, locals.var_deltemp_dn0, locals.var_deltemp_dn2, locals.var_deltemp_dn4, locals.var_deltemp_dn5, locals.var_deltemp_dn6, locals.var_deltemp_dn7, locals.var_deltemp_dn8, locals.var_deltemp_dn9, locals.var_deltemp_dn10, locals.var_deltemp_dn11, locals.var_deltemp_dn14,)
    }
};
        locals.var_deltemp = assign17080_e11613;
        locals.var_deltemp_dn0 = assign17080_e11613_d_n0;
        locals.var_deltemp_dn2 = assign17080_e11613_d_n2;
        locals.var_deltemp_dn4 = assign17080_e11613_d_n4;
        locals.var_deltemp_dn5 = assign17080_e11613_d_n5;
        locals.var_deltemp_dn6 = assign17080_e11613_d_n6;
        locals.var_deltemp_dn7 = assign17080_e11613_d_n7;
        locals.var_deltemp_dn8 = assign17080_e11613_d_n8;
        locals.var_deltemp_dn9 = assign17080_e11613_d_n9;
        locals.var_deltemp_dn10 = assign17080_e11613_d_n10;
        locals.var_deltemp_dn11 = assign17080_e11613_d_n11;
        locals.var_deltemp_dn14 = assign17080_e11613_d_n14;

        let assign17090_e11616: f64 = if p.p53 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard357 = assign17090_e11616;

        let (assign17100_e11628, assign17100_e11628_d_n0, assign17100_e11628_d_n2, assign17100_e11628_d_n4, assign17100_e11628_d_n5, assign17100_e11628_d_n6, assign17100_e11628_d_n7, assign17100_e11628_d_n8, assign17100_e11628_d_n9, assign17100_e11628_d_n10, assign17100_e11628_d_n11, assign17100_e11628_d_n14,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard357 != 0.0)) {
        let assign17100_e11622: f64 = (p.p433 - locals.var_deltemp);
        let assign17100_e11625: f64 = (p.p337 * 10.0);
        let assign17100_e11626: f64 = (assign17100_e11622 - assign17100_e11625);
        (assign17100_e11626, (-locals.var_deltemp_dn0), (-locals.var_deltemp_dn2), (-locals.var_deltemp_dn4), (-locals.var_deltemp_dn5), (-locals.var_deltemp_dn6), (-locals.var_deltemp_dn7), (-locals.var_deltemp_dn8), (-locals.var_deltemp_dn9), (-locals.var_deltemp_dn10), (-locals.var_deltemp_dn11), (-locals.var_deltemp_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign17100_e11628;
        locals.var_tmf1_dn0 = assign17100_e11628_d_n0;
        locals.var_tmf1_dn2 = assign17100_e11628_d_n2;
        locals.var_tmf1_dn4 = assign17100_e11628_d_n4;
        locals.var_tmf1_dn5 = assign17100_e11628_d_n5;
        locals.var_tmf1_dn6 = assign17100_e11628_d_n6;
        locals.var_tmf1_dn7 = assign17100_e11628_d_n7;
        locals.var_tmf1_dn8 = assign17100_e11628_d_n8;
        locals.var_tmf1_dn9 = assign17100_e11628_d_n9;
        locals.var_tmf1_dn10 = assign17100_e11628_d_n10;
        locals.var_tmf1_dn11 = assign17100_e11628_d_n11;
        locals.var_tmf1_dn14 = assign17100_e11628_d_n14;

        let (assign17110_e11640, assign17110_e11640_d_n0, assign17110_e11640_d_n2, assign17110_e11640_d_n4, assign17110_e11640_d_n5, assign17110_e11640_d_n6, assign17110_e11640_d_n7, assign17110_e11640_d_n8, assign17110_e11640_d_n9, assign17110_e11640_d_n10, assign17110_e11640_d_n11, assign17110_e11640_d_n14,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard357 != 0.0)) {
        let assign17110_e11634: f64 = (4.0 * p.p433);
        let assign17110_e11637: f64 = (p.p337 * 10.0);
        let assign17110_e11638: f64 = (assign17110_e11634 * assign17110_e11637);
        (assign17110_e11638, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign17110_e11640;
        locals.var_tmf2_dn0 = assign17110_e11640_d_n0;
        locals.var_tmf2_dn2 = assign17110_e11640_d_n2;
        locals.var_tmf2_dn4 = assign17110_e11640_d_n4;
        locals.var_tmf2_dn5 = assign17110_e11640_d_n5;
        locals.var_tmf2_dn6 = assign17110_e11640_d_n6;
        locals.var_tmf2_dn7 = assign17110_e11640_d_n7;
        locals.var_tmf2_dn8 = assign17110_e11640_d_n8;
        locals.var_tmf2_dn9 = assign17110_e11640_d_n9;
        locals.var_tmf2_dn10 = assign17110_e11640_d_n10;
        locals.var_tmf2_dn11 = assign17110_e11640_d_n11;
        locals.var_tmf2_dn14 = assign17110_e11640_d_n14;

        let (assign17120_e11652, assign17120_e11652_d_n0, assign17120_e11652_d_n2, assign17120_e11652_d_n4, assign17120_e11652_d_n5, assign17120_e11652_d_n6, assign17120_e11652_d_n7, assign17120_e11652_d_n8, assign17120_e11652_d_n9, assign17120_e11652_d_n10, assign17120_e11652_d_n11, assign17120_e11652_d_n14,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard357 != 0.0)) {
        let (assign17120_e11650, assign17120_e11650_d_n0, assign17120_e11650_d_n2, assign17120_e11650_d_n4, assign17120_e11650_d_n5, assign17120_e11650_d_n6, assign17120_e11650_d_n7, assign17120_e11650_d_n8, assign17120_e11650_d_n9, assign17120_e11650_d_n10, assign17120_e11650_d_n11, assign17120_e11650_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign17120_e11649: f64 = (-locals.var_tmf2);
                (assign17120_e11649, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign17120_e11650, assign17120_e11650_d_n0, assign17120_e11650_d_n2, assign17120_e11650_d_n4, assign17120_e11650_d_n5, assign17120_e11650_d_n6, assign17120_e11650_d_n7, assign17120_e11650_d_n8, assign17120_e11650_d_n9, assign17120_e11650_d_n10, assign17120_e11650_d_n11, assign17120_e11650_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign17120_e11652;
        locals.var_tmf2_dn0 = assign17120_e11652_d_n0;
        locals.var_tmf2_dn2 = assign17120_e11652_d_n2;
        locals.var_tmf2_dn4 = assign17120_e11652_d_n4;
        locals.var_tmf2_dn5 = assign17120_e11652_d_n5;
        locals.var_tmf2_dn6 = assign17120_e11652_d_n6;
        locals.var_tmf2_dn7 = assign17120_e11652_d_n7;
        locals.var_tmf2_dn8 = assign17120_e11652_d_n8;
        locals.var_tmf2_dn9 = assign17120_e11652_d_n9;
        locals.var_tmf2_dn10 = assign17120_e11652_d_n10;
        locals.var_tmf2_dn11 = assign17120_e11652_d_n11;
        locals.var_tmf2_dn14 = assign17120_e11652_d_n14;

        let (assign17130_e11663, assign17130_e11663_d_n0, assign17130_e11663_d_n2, assign17130_e11663_d_n4, assign17130_e11663_d_n5, assign17130_e11663_d_n6, assign17130_e11663_d_n7, assign17130_e11663_d_n8, assign17130_e11663_d_n9, assign17130_e11663_d_n10, assign17130_e11663_d_n11, assign17130_e11663_d_n14,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard357 != 0.0)) {
        let assign17130_e11658: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign17130_e11660: f64 = (assign17130_e11658 + locals.var_tmf2);
        let assign17130_e11661: f64 = (assign17130_e11660).sqrt();
        (assign17130_e11661, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign17130_e11661)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign17130_e11661)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign17130_e11661)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign17130_e11661)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign17130_e11661)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign17130_e11661)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign17130_e11661)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign17130_e11661)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign17130_e11661)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign17130_e11661)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign17130_e11661)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign17130_e11663;
        locals.var_tmf2_dn0 = assign17130_e11663_d_n0;
        locals.var_tmf2_dn2 = assign17130_e11663_d_n2;
        locals.var_tmf2_dn4 = assign17130_e11663_d_n4;
        locals.var_tmf2_dn5 = assign17130_e11663_d_n5;
        locals.var_tmf2_dn6 = assign17130_e11663_d_n6;
        locals.var_tmf2_dn7 = assign17130_e11663_d_n7;
        locals.var_tmf2_dn8 = assign17130_e11663_d_n8;
        locals.var_tmf2_dn9 = assign17130_e11663_d_n9;
        locals.var_tmf2_dn10 = assign17130_e11663_d_n10;
        locals.var_tmf2_dn11 = assign17130_e11663_d_n11;
        locals.var_tmf2_dn14 = assign17130_e11663_d_n14;

        let (assign17140_e11675, assign17140_e11675_d_n0, assign17140_e11675_d_n2, assign17140_e11675_d_n4, assign17140_e11675_d_n5, assign17140_e11675_d_n6, assign17140_e11675_d_n7, assign17140_e11675_d_n8, assign17140_e11675_d_n9, assign17140_e11675_d_n10, assign17140_e11675_d_n11, assign17140_e11675_d_n14,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard357 != 0.0)) {
        let assign17140_e11671: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign17140_e11672: f64 = (1.0 + assign17140_e11671);
        let assign17140_e11673: f64 = (0.5 * assign17140_e11672);
        (assign17140_e11673, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign17140_e11675;
        locals.var_t0_dn0 = assign17140_e11675_d_n0;
        locals.var_t0_dn2 = assign17140_e11675_d_n2;
        locals.var_t0_dn4 = assign17140_e11675_d_n4;
        locals.var_t0_dn5 = assign17140_e11675_d_n5;
        locals.var_t0_dn6 = assign17140_e11675_d_n6;
        locals.var_t0_dn7 = assign17140_e11675_d_n7;
        locals.var_t0_dn8 = assign17140_e11675_d_n8;
        locals.var_t0_dn9 = assign17140_e11675_d_n9;
        locals.var_t0_dn10 = assign17140_e11675_d_n10;
        locals.var_t0_dn11 = assign17140_e11675_d_n11;
        locals.var_t0_dn14 = assign17140_e11675_d_n14;

        let (assign17150_e11687, assign17150_e11687_d_n0, assign17150_e11687_d_n2, assign17150_e11687_d_n4, assign17150_e11687_d_n5, assign17150_e11687_d_n6, assign17150_e11687_d_n7, assign17150_e11687_d_n8, assign17150_e11687_d_n9, assign17150_e11687_d_n10, assign17150_e11687_d_n11, assign17150_e11687_d_n14,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard357 != 0.0)) {
        let assign17150_e11683: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign17150_e11684: f64 = (0.5 * assign17150_e11683);
        let assign17150_e11685: f64 = (p.p433 - assign17150_e11684);
        (assign17150_e11685, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (-(0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_deltemp, locals.var_deltemp_dn0, locals.var_deltemp_dn2, locals.var_deltemp_dn4, locals.var_deltemp_dn5, locals.var_deltemp_dn6, locals.var_deltemp_dn7, locals.var_deltemp_dn8, locals.var_deltemp_dn9, locals.var_deltemp_dn10, locals.var_deltemp_dn11, locals.var_deltemp_dn14,)
    }
};
        locals.var_deltemp = assign17150_e11687;
        locals.var_deltemp_dn0 = assign17150_e11687_d_n0;
        locals.var_deltemp_dn2 = assign17150_e11687_d_n2;
        locals.var_deltemp_dn4 = assign17150_e11687_d_n4;
        locals.var_deltemp_dn5 = assign17150_e11687_d_n5;
        locals.var_deltemp_dn6 = assign17150_e11687_d_n6;
        locals.var_deltemp_dn7 = assign17150_e11687_d_n7;
        locals.var_deltemp_dn8 = assign17150_e11687_d_n8;
        locals.var_deltemp_dn9 = assign17150_e11687_d_n9;
        locals.var_deltemp_dn10 = assign17150_e11687_d_n10;
        locals.var_deltemp_dn11 = assign17150_e11687_d_n11;
        locals.var_deltemp_dn14 = assign17150_e11687_d_n14;

        let (assign17170_e11696, assign17170_e11696_d_n0, assign17170_e11696_d_n2, assign17170_e11696_d_n4, assign17170_e11696_d_n5, assign17170_e11696_d_n6, assign17170_e11696_d_n7, assign17170_e11696_d_n8, assign17170_e11696_d_n9, assign17170_e11696_d_n10, assign17170_e11696_d_n11, assign17170_e11696_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign17170_e11692: f64 = ctx_temp;
        let assign17170_e11694: f64 = (assign17170_e11692 + p.p11);
        (assign17170_e11694, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ttemp, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn11, locals.var_ttemp_dn14,)
    }
};
        locals.var_ttemp = assign17170_e11696;
        locals.var_ttemp_dn0 = assign17170_e11696_d_n0;
        locals.var_ttemp_dn2 = assign17170_e11696_d_n2;
        locals.var_ttemp_dn4 = assign17170_e11696_d_n4;
        locals.var_ttemp_dn5 = assign17170_e11696_d_n5;
        locals.var_ttemp_dn6 = assign17170_e11696_d_n6;
        locals.var_ttemp_dn7 = assign17170_e11696_d_n7;
        locals.var_ttemp_dn8 = assign17170_e11696_d_n8;
        locals.var_ttemp_dn9 = assign17170_e11696_d_n9;
        locals.var_ttemp_dn10 = assign17170_e11696_d_n10;
        locals.var_ttemp_dn11 = assign17170_e11696_d_n11;
        locals.var_ttemp_dn14 = assign17170_e11696_d_n14;

        let (assign17180_e11700, assign17180_e11700_d_n0, assign17180_e11700_d_n2, assign17180_e11700_d_n4, assign17180_e11700_d_n5, assign17180_e11700_d_n6, assign17180_e11700_d_n7, assign17180_e11700_d_n8, assign17180_e11700_d_n9, assign17180_e11700_d_n10, assign17180_e11700_d_n11, assign17180_e11700_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        (locals.var_ttemp, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn11, locals.var_ttemp_dn14,)
    } else {
        (locals.var_ttemp0, locals.var_ttemp0_dn0, locals.var_ttemp0_dn2, locals.var_ttemp0_dn4, locals.var_ttemp0_dn5, locals.var_ttemp0_dn6, locals.var_ttemp0_dn7, locals.var_ttemp0_dn8, locals.var_ttemp0_dn9, locals.var_ttemp0_dn10, locals.var_ttemp0_dn11, locals.var_ttemp0_dn14,)
    }
};
        locals.var_ttemp0 = assign17180_e11700;
        locals.var_ttemp0_dn0 = assign17180_e11700_d_n0;
        locals.var_ttemp0_dn2 = assign17180_e11700_d_n2;
        locals.var_ttemp0_dn4 = assign17180_e11700_d_n4;
        locals.var_ttemp0_dn5 = assign17180_e11700_d_n5;
        locals.var_ttemp0_dn6 = assign17180_e11700_d_n6;
        locals.var_ttemp0_dn7 = assign17180_e11700_d_n7;
        locals.var_ttemp0_dn8 = assign17180_e11700_d_n8;
        locals.var_ttemp0_dn9 = assign17180_e11700_d_n9;
        locals.var_ttemp0_dn10 = assign17180_e11700_d_n10;
        locals.var_ttemp0_dn11 = assign17180_e11700_d_n11;
        locals.var_ttemp0_dn14 = assign17180_e11700_d_n14;

        let (assign17190_e11706, assign17190_e11706_d_n0, assign17190_e11706_d_n2, assign17190_e11706_d_n4, assign17190_e11706_d_n5, assign17190_e11706_d_n6, assign17190_e11706_d_n7, assign17190_e11706_d_n8, assign17190_e11706_d_n9, assign17190_e11706_d_n10, assign17190_e11706_d_n11, assign17190_e11706_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign17190_e11704: f64 = (locals.var_ttemp + locals.var_deltemp);
        (assign17190_e11704, (locals.var_ttemp_dn0 + locals.var_deltemp_dn0), (locals.var_ttemp_dn2 + locals.var_deltemp_dn2), (locals.var_ttemp_dn4 + locals.var_deltemp_dn4), (locals.var_ttemp_dn5 + locals.var_deltemp_dn5), (locals.var_ttemp_dn6 + locals.var_deltemp_dn6), (locals.var_ttemp_dn7 + locals.var_deltemp_dn7), (locals.var_ttemp_dn8 + locals.var_deltemp_dn8), (locals.var_ttemp_dn9 + locals.var_deltemp_dn9), (locals.var_ttemp_dn10 + locals.var_deltemp_dn10), (locals.var_ttemp_dn11 + locals.var_deltemp_dn11), (locals.var_ttemp_dn14 + locals.var_deltemp_dn14),)
    } else {
        (locals.var_ttemp, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn11, locals.var_ttemp_dn14,)
    }
};
        locals.var_ttemp = assign17190_e11706;
        locals.var_ttemp_dn0 = assign17190_e11706_d_n0;
        locals.var_ttemp_dn2 = assign17190_e11706_d_n2;
        locals.var_ttemp_dn4 = assign17190_e11706_d_n4;
        locals.var_ttemp_dn5 = assign17190_e11706_d_n5;
        locals.var_ttemp_dn6 = assign17190_e11706_d_n6;
        locals.var_ttemp_dn7 = assign17190_e11706_d_n7;
        locals.var_ttemp_dn8 = assign17190_e11706_d_n8;
        locals.var_ttemp_dn9 = assign17190_e11706_d_n9;
        locals.var_ttemp_dn10 = assign17190_e11706_d_n10;
        locals.var_ttemp_dn11 = assign17190_e11706_d_n11;
        locals.var_ttemp_dn14 = assign17190_e11706_d_n14;

        let (assign17200_e11712, assign17200_e11712_d_n0, assign17200_e11712_d_n2, assign17200_e11712_d_n4, assign17200_e11712_d_n5, assign17200_e11712_d_n6, assign17200_e11712_d_n7, assign17200_e11712_d_n8, assign17200_e11712_d_n9, assign17200_e11712_d_n10, assign17200_e11712_d_n11, assign17200_e11712_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign17200_e11710: f64 = (locals.var_ttemp0 - locals.var_ktnom);
        (assign17200_e11710, locals.var_ttemp0_dn0, locals.var_ttemp0_dn2, locals.var_ttemp0_dn4, locals.var_ttemp0_dn5, locals.var_ttemp0_dn6, locals.var_ttemp0_dn7, locals.var_ttemp0_dn8, locals.var_ttemp0_dn9, locals.var_ttemp0_dn10, locals.var_ttemp0_dn11, locals.var_ttemp0_dn14,)
    } else {
        (locals.var_tdiff0, locals.var_tdiff0_dn0, locals.var_tdiff0_dn2, locals.var_tdiff0_dn4, locals.var_tdiff0_dn5, locals.var_tdiff0_dn6, locals.var_tdiff0_dn7, locals.var_tdiff0_dn8, locals.var_tdiff0_dn9, locals.var_tdiff0_dn10, locals.var_tdiff0_dn11, locals.var_tdiff0_dn14,)
    }
};
        locals.var_tdiff0 = assign17200_e11712;
        locals.var_tdiff0_dn0 = assign17200_e11712_d_n0;
        locals.var_tdiff0_dn2 = assign17200_e11712_d_n2;
        locals.var_tdiff0_dn4 = assign17200_e11712_d_n4;
        locals.var_tdiff0_dn5 = assign17200_e11712_d_n5;
        locals.var_tdiff0_dn6 = assign17200_e11712_d_n6;
        locals.var_tdiff0_dn7 = assign17200_e11712_d_n7;
        locals.var_tdiff0_dn8 = assign17200_e11712_d_n8;
        locals.var_tdiff0_dn9 = assign17200_e11712_d_n9;
        locals.var_tdiff0_dn10 = assign17200_e11712_d_n10;
        locals.var_tdiff0_dn11 = assign17200_e11712_d_n11;
        locals.var_tdiff0_dn14 = assign17200_e11712_d_n14;

        let (assign17210_e11722, assign17210_e11722_d_n0, assign17210_e11722_d_n2, assign17210_e11722_d_n4, assign17210_e11722_d_n5, assign17210_e11722_d_n6, assign17210_e11722_d_n7, assign17210_e11722_d_n8, assign17210_e11722_d_n9, assign17210_e11722_d_n10, assign17210_e11722_d_n11, assign17210_e11722_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign17210_e11716: f64 = (locals.var_ttemp0 * locals.var_ttemp0);
        let assign17210_e11719: f64 = (locals.var_ktnom * locals.var_ktnom);
        let assign17210_e11720: f64 = (assign17210_e11716 - assign17210_e11719);
        (assign17210_e11720, ((locals.var_ttemp0_dn0 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn0)), ((locals.var_ttemp0_dn2 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn2)), ((locals.var_ttemp0_dn4 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn4)), ((locals.var_ttemp0_dn5 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn5)), ((locals.var_ttemp0_dn6 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn6)), ((locals.var_ttemp0_dn7 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn7)), ((locals.var_ttemp0_dn8 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn8)), ((locals.var_ttemp0_dn9 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn9)), ((locals.var_ttemp0_dn10 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn10)), ((locals.var_ttemp0_dn11 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn11)), ((locals.var_ttemp0_dn14 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn14)),)
    } else {
        (locals.var_tdiff0_2, locals.var_tdiff0_2_dn0, locals.var_tdiff0_2_dn2, locals.var_tdiff0_2_dn4, locals.var_tdiff0_2_dn5, locals.var_tdiff0_2_dn6, locals.var_tdiff0_2_dn7, locals.var_tdiff0_2_dn8, locals.var_tdiff0_2_dn9, locals.var_tdiff0_2_dn10, locals.var_tdiff0_2_dn11, locals.var_tdiff0_2_dn14,)
    }
};
        locals.var_tdiff0_2 = assign17210_e11722;
        locals.var_tdiff0_2_dn0 = assign17210_e11722_d_n0;
        locals.var_tdiff0_2_dn2 = assign17210_e11722_d_n2;
        locals.var_tdiff0_2_dn4 = assign17210_e11722_d_n4;
        locals.var_tdiff0_2_dn5 = assign17210_e11722_d_n5;
        locals.var_tdiff0_2_dn6 = assign17210_e11722_d_n6;
        locals.var_tdiff0_2_dn7 = assign17210_e11722_d_n7;
        locals.var_tdiff0_2_dn8 = assign17210_e11722_d_n8;
        locals.var_tdiff0_2_dn9 = assign17210_e11722_d_n9;
        locals.var_tdiff0_2_dn10 = assign17210_e11722_d_n10;
        locals.var_tdiff0_2_dn11 = assign17210_e11722_d_n11;
        locals.var_tdiff0_2_dn14 = assign17210_e11722_d_n14;

    }

    pub(super) fn stamp_transient_block_37(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign17220_e11728, assign17220_e11728_d_n0, assign17220_e11728_d_n2, assign17220_e11728_d_n4, assign17220_e11728_d_n5, assign17220_e11728_d_n6, assign17220_e11728_d_n7, assign17220_e11728_d_n8, assign17220_e11728_d_n9, assign17220_e11728_d_n10, assign17220_e11728_d_n11, assign17220_e11728_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign17220_e11726: f64 = (locals.var_ttemp - locals.var_ktnom);
        (assign17220_e11726, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn11, locals.var_ttemp_dn14,)
    } else {
        (locals.var_tdiff, locals.var_tdiff_dn0, locals.var_tdiff_dn2, locals.var_tdiff_dn4, locals.var_tdiff_dn5, locals.var_tdiff_dn6, locals.var_tdiff_dn7, locals.var_tdiff_dn8, locals.var_tdiff_dn9, locals.var_tdiff_dn10, locals.var_tdiff_dn11, locals.var_tdiff_dn14,)
    }
};
        locals.var_tdiff = assign17220_e11728;
        locals.var_tdiff_dn0 = assign17220_e11728_d_n0;
        locals.var_tdiff_dn2 = assign17220_e11728_d_n2;
        locals.var_tdiff_dn4 = assign17220_e11728_d_n4;
        locals.var_tdiff_dn5 = assign17220_e11728_d_n5;
        locals.var_tdiff_dn6 = assign17220_e11728_d_n6;
        locals.var_tdiff_dn7 = assign17220_e11728_d_n7;
        locals.var_tdiff_dn8 = assign17220_e11728_d_n8;
        locals.var_tdiff_dn9 = assign17220_e11728_d_n9;
        locals.var_tdiff_dn10 = assign17220_e11728_d_n10;
        locals.var_tdiff_dn11 = assign17220_e11728_d_n11;
        locals.var_tdiff_dn14 = assign17220_e11728_d_n14;

        let (assign17230_e11738, assign17230_e11738_d_n0, assign17230_e11738_d_n2, assign17230_e11738_d_n4, assign17230_e11738_d_n5, assign17230_e11738_d_n6, assign17230_e11738_d_n7, assign17230_e11738_d_n8, assign17230_e11738_d_n9, assign17230_e11738_d_n10, assign17230_e11738_d_n11, assign17230_e11738_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign17230_e11732: f64 = (locals.var_ttemp * locals.var_ttemp);
        let assign17230_e11735: f64 = (locals.var_ktnom * locals.var_ktnom);
        let assign17230_e11736: f64 = (assign17230_e11732 - assign17230_e11735);
        (assign17230_e11736, ((locals.var_ttemp_dn0 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn0)), ((locals.var_ttemp_dn2 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn2)), ((locals.var_ttemp_dn4 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn4)), ((locals.var_ttemp_dn5 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn5)), ((locals.var_ttemp_dn6 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn6)), ((locals.var_ttemp_dn7 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn7)), ((locals.var_ttemp_dn8 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn8)), ((locals.var_ttemp_dn9 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn9)), ((locals.var_ttemp_dn10 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn10)), ((locals.var_ttemp_dn11 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn11)), ((locals.var_ttemp_dn14 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn14)),)
    } else {
        (locals.var_tdiff_2, locals.var_tdiff_2_dn0, locals.var_tdiff_2_dn2, locals.var_tdiff_2_dn4, locals.var_tdiff_2_dn5, locals.var_tdiff_2_dn6, locals.var_tdiff_2_dn7, locals.var_tdiff_2_dn8, locals.var_tdiff_2_dn9, locals.var_tdiff_2_dn10, locals.var_tdiff_2_dn11, locals.var_tdiff_2_dn14,)
    }
};
        locals.var_tdiff_2 = assign17230_e11738;
        locals.var_tdiff_2_dn0 = assign17230_e11738_d_n0;
        locals.var_tdiff_2_dn2 = assign17230_e11738_d_n2;
        locals.var_tdiff_2_dn4 = assign17230_e11738_d_n4;
        locals.var_tdiff_2_dn5 = assign17230_e11738_d_n5;
        locals.var_tdiff_2_dn6 = assign17230_e11738_d_n6;
        locals.var_tdiff_2_dn7 = assign17230_e11738_d_n7;
        locals.var_tdiff_2_dn8 = assign17230_e11738_d_n8;
        locals.var_tdiff_2_dn9 = assign17230_e11738_d_n9;
        locals.var_tdiff_2_dn10 = assign17230_e11738_d_n10;
        locals.var_tdiff_2_dn11 = assign17230_e11738_d_n11;
        locals.var_tdiff_2_dn14 = assign17230_e11738_d_n14;

        let (assign17240_e11744, assign17240_e11744_d_n0, assign17240_e11744_d_n2, assign17240_e11744_d_n4, assign17240_e11744_d_n5, assign17240_e11744_d_n6, assign17240_e11744_d_n7, assign17240_e11744_d_n8, assign17240_e11744_d_n9, assign17240_e11744_d_n10, assign17240_e11744_d_n11, assign17240_e11744_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign17240_e11742: f64 = (locals.var_ttemp / locals.var_ktnom);
        (assign17240_e11742, (locals.var_ttemp_dn0 / locals.var_ktnom), (locals.var_ttemp_dn2 / locals.var_ktnom), (locals.var_ttemp_dn4 / locals.var_ktnom), (locals.var_ttemp_dn5 / locals.var_ktnom), (locals.var_ttemp_dn6 / locals.var_ktnom), (locals.var_ttemp_dn7 / locals.var_ktnom), (locals.var_ttemp_dn8 / locals.var_ktnom), (locals.var_ttemp_dn9 / locals.var_ktnom), (locals.var_ttemp_dn10 / locals.var_ktnom), (locals.var_ttemp_dn11 / locals.var_ktnom), (locals.var_ttemp_dn14 / locals.var_ktnom),)
    } else {
        (locals.var_tratio, locals.var_tratio_dn0, locals.var_tratio_dn2, locals.var_tratio_dn4, locals.var_tratio_dn5, locals.var_tratio_dn6, locals.var_tratio_dn7, locals.var_tratio_dn8, locals.var_tratio_dn9, locals.var_tratio_dn10, locals.var_tratio_dn11, locals.var_tratio_dn14,)
    }
};
        locals.var_tratio = assign17240_e11744;
        locals.var_tratio_dn0 = assign17240_e11744_d_n0;
        locals.var_tratio_dn2 = assign17240_e11744_d_n2;
        locals.var_tratio_dn4 = assign17240_e11744_d_n4;
        locals.var_tratio_dn5 = assign17240_e11744_d_n5;
        locals.var_tratio_dn6 = assign17240_e11744_d_n6;
        locals.var_tratio_dn7 = assign17240_e11744_d_n7;
        locals.var_tratio_dn8 = assign17240_e11744_d_n8;
        locals.var_tratio_dn9 = assign17240_e11744_d_n9;
        locals.var_tratio_dn10 = assign17240_e11744_d_n10;
        locals.var_tratio_dn11 = assign17240_e11744_d_n11;
        locals.var_tratio_dn14 = assign17240_e11744_d_n14;

        let (assign17250_e11749, assign17250_e11749_d_n0, assign17250_e11749_d_n2, assign17250_e11749_d_n4, assign17250_e11749_d_n5, assign17250_e11749_d_n6, assign17250_e11749_d_n7, assign17250_e11749_d_n8, assign17250_e11749_d_n9, assign17250_e11749_d_n10, assign17250_e11749_d_n11, assign17250_e11749_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign17250_e11747: f64 = (locals.var_tratio).ln();
        (assign17250_e11747, (locals.var_tratio_dn0 / locals.var_tratio), (locals.var_tratio_dn2 / locals.var_tratio), (locals.var_tratio_dn4 / locals.var_tratio), (locals.var_tratio_dn5 / locals.var_tratio), (locals.var_tratio_dn6 / locals.var_tratio), (locals.var_tratio_dn7 / locals.var_tratio), (locals.var_tratio_dn8 / locals.var_tratio), (locals.var_tratio_dn9 / locals.var_tratio), (locals.var_tratio_dn10 / locals.var_tratio), (locals.var_tratio_dn11 / locals.var_tratio), (locals.var_tratio_dn14 / locals.var_tratio),)
    } else {
        (locals.var_log_tratio, locals.var_log_tratio_dn0, locals.var_log_tratio_dn2, locals.var_log_tratio_dn4, locals.var_log_tratio_dn5, locals.var_log_tratio_dn6, locals.var_log_tratio_dn7, locals.var_log_tratio_dn8, locals.var_log_tratio_dn9, locals.var_log_tratio_dn10, locals.var_log_tratio_dn11, locals.var_log_tratio_dn14,)
    }
};
        locals.var_log_tratio = assign17250_e11749;
        locals.var_log_tratio_dn0 = assign17250_e11749_d_n0;
        locals.var_log_tratio_dn2 = assign17250_e11749_d_n2;
        locals.var_log_tratio_dn4 = assign17250_e11749_d_n4;
        locals.var_log_tratio_dn5 = assign17250_e11749_d_n5;
        locals.var_log_tratio_dn6 = assign17250_e11749_d_n6;
        locals.var_log_tratio_dn7 = assign17250_e11749_d_n7;
        locals.var_log_tratio_dn8 = assign17250_e11749_d_n8;
        locals.var_log_tratio_dn9 = assign17250_e11749_d_n9;
        locals.var_log_tratio_dn10 = assign17250_e11749_d_n10;
        locals.var_log_tratio_dn11 = assign17250_e11749_d_n11;
        locals.var_log_tratio_dn14 = assign17250_e11749_d_n14;

        let (assign17260_e11761, assign17260_e11761_d_n0, assign17260_e11761_d_n2, assign17260_e11761_d_n4, assign17260_e11761_d_n5, assign17260_e11761_d_n6, assign17260_e11761_d_n7, assign17260_e11761_d_n8, assign17260_e11761_d_n9, assign17260_e11761_d_n10, assign17260_e11761_d_n11, assign17260_e11761_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign17260_e11754: f64 = (locals.var_uc_bgtmp1 * locals.var_tdiff);
        let assign17260_e11755: f64 = (locals.var_egtnom - assign17260_e11754);
        let assign17260_e11758: f64 = (locals.var_uc_bgtmp2 * locals.var_tdiff_2);
        let assign17260_e11759: f64 = (assign17260_e11755 - assign17260_e11758);
        (assign17260_e11759, ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn0)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn0)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn2)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn2)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn4)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn4)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn5)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn5)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn6)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn6)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn7)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn7)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn8)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn8)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn9)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn9)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn10)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn10)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn11)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn11)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn14)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn14)),)
    } else {
        (locals.var_eg, locals.var_eg_dn0, locals.var_eg_dn2, locals.var_eg_dn4, locals.var_eg_dn5, locals.var_eg_dn6, locals.var_eg_dn7, locals.var_eg_dn8, locals.var_eg_dn9, locals.var_eg_dn10, locals.var_eg_dn11, locals.var_eg_dn14,)
    }
};
        locals.var_eg = assign17260_e11761;
        locals.var_eg_dn0 = assign17260_e11761_d_n0;
        locals.var_eg_dn2 = assign17260_e11761_d_n2;
        locals.var_eg_dn4 = assign17260_e11761_d_n4;
        locals.var_eg_dn5 = assign17260_e11761_d_n5;
        locals.var_eg_dn6 = assign17260_e11761_d_n6;
        locals.var_eg_dn7 = assign17260_e11761_d_n7;
        locals.var_eg_dn8 = assign17260_e11761_d_n8;
        locals.var_eg_dn9 = assign17260_e11761_d_n9;
        locals.var_eg_dn10 = assign17260_e11761_d_n10;
        locals.var_eg_dn11 = assign17260_e11761_d_n11;
        locals.var_eg_dn14 = assign17260_e11761_d_n14;

        let (assign17270_e11766, assign17270_e11766_d_n0, assign17270_e11766_d_n2, assign17270_e11766_d_n4, assign17270_e11766_d_n5, assign17270_e11766_d_n6, assign17270_e11766_d_n7, assign17270_e11766_d_n8, assign17270_e11766_d_n9, assign17270_e11766_d_n10, assign17270_e11766_d_n11, assign17270_e11766_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign17270_e11764: f64 = (locals.var_eg).sqrt();
        (assign17270_e11764, (locals.var_eg_dn0 / (2.0 * assign17270_e11764)), (locals.var_eg_dn2 / (2.0 * assign17270_e11764)), (locals.var_eg_dn4 / (2.0 * assign17270_e11764)), (locals.var_eg_dn5 / (2.0 * assign17270_e11764)), (locals.var_eg_dn6 / (2.0 * assign17270_e11764)), (locals.var_eg_dn7 / (2.0 * assign17270_e11764)), (locals.var_eg_dn8 / (2.0 * assign17270_e11764)), (locals.var_eg_dn9 / (2.0 * assign17270_e11764)), (locals.var_eg_dn10 / (2.0 * assign17270_e11764)), (locals.var_eg_dn11 / (2.0 * assign17270_e11764)), (locals.var_eg_dn14 / (2.0 * assign17270_e11764)),)
    } else {
        (locals.var_sqrt_eg, locals.var_sqrt_eg_dn0, locals.var_sqrt_eg_dn2, locals.var_sqrt_eg_dn4, locals.var_sqrt_eg_dn5, locals.var_sqrt_eg_dn6, locals.var_sqrt_eg_dn7, locals.var_sqrt_eg_dn8, locals.var_sqrt_eg_dn9, locals.var_sqrt_eg_dn10, locals.var_sqrt_eg_dn11, locals.var_sqrt_eg_dn14,)
    }
};
        locals.var_sqrt_eg = assign17270_e11766;
        locals.var_sqrt_eg_dn0 = assign17270_e11766_d_n0;
        locals.var_sqrt_eg_dn2 = assign17270_e11766_d_n2;
        locals.var_sqrt_eg_dn4 = assign17270_e11766_d_n4;
        locals.var_sqrt_eg_dn5 = assign17270_e11766_d_n5;
        locals.var_sqrt_eg_dn6 = assign17270_e11766_d_n6;
        locals.var_sqrt_eg_dn7 = assign17270_e11766_d_n7;
        locals.var_sqrt_eg_dn8 = assign17270_e11766_d_n8;
        locals.var_sqrt_eg_dn9 = assign17270_e11766_d_n9;
        locals.var_sqrt_eg_dn10 = assign17270_e11766_d_n10;
        locals.var_sqrt_eg_dn11 = assign17270_e11766_d_n11;
        locals.var_sqrt_eg_dn14 = assign17270_e11766_d_n14;

        let (assign17280_e11772, assign17280_e11772_d_n0, assign17280_e11772_d_n2, assign17280_e11772_d_n4, assign17280_e11772_d_n5, assign17280_e11772_d_n6, assign17280_e11772_d_n7, assign17280_e11772_d_n8, assign17280_e11772_d_n9, assign17280_e11772_d_n10, assign17280_e11772_d_n11, assign17280_e11772_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign17280_e11770: f64 = (1.0 / locals.var_ttemp);
        (assign17280_e11770, (-(locals.var_ttemp_dn0 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn2 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn4 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn5 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn6 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn7 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn8 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn9 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn10 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn11 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn14 / (locals.var_ttemp * locals.var_ttemp))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign17280_e11772;
        locals.var_t1_dn0 = assign17280_e11772_d_n0;
        locals.var_t1_dn2 = assign17280_e11772_d_n2;
        locals.var_t1_dn4 = assign17280_e11772_d_n4;
        locals.var_t1_dn5 = assign17280_e11772_d_n5;
        locals.var_t1_dn6 = assign17280_e11772_d_n6;
        locals.var_t1_dn7 = assign17280_e11772_d_n7;
        locals.var_t1_dn8 = assign17280_e11772_d_n8;
        locals.var_t1_dn9 = assign17280_e11772_d_n9;
        locals.var_t1_dn10 = assign17280_e11772_d_n10;
        locals.var_t1_dn11 = assign17280_e11772_d_n11;
        locals.var_t1_dn14 = assign17280_e11772_d_n14;

        let (assign17290_e11778, assign17290_e11778_d_n0, assign17290_e11778_d_n2, assign17290_e11778_d_n4, assign17290_e11778_d_n5, assign17290_e11778_d_n6, assign17290_e11778_d_n7, assign17290_e11778_d_n8, assign17290_e11778_d_n9, assign17290_e11778_d_n10, assign17290_e11778_d_n11, assign17290_e11778_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign17290_e11776: f64 = (1.0 / locals.var_ktnom);
        (assign17290_e11776, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign17290_e11778;
        locals.var_t2_dn0 = assign17290_e11778_d_n0;
        locals.var_t2_dn2 = assign17290_e11778_d_n2;
        locals.var_t2_dn4 = assign17290_e11778_d_n4;
        locals.var_t2_dn5 = assign17290_e11778_d_n5;
        locals.var_t2_dn6 = assign17290_e11778_d_n6;
        locals.var_t2_dn7 = assign17290_e11778_d_n7;
        locals.var_t2_dn8 = assign17290_e11778_d_n8;
        locals.var_t2_dn9 = assign17290_e11778_d_n9;
        locals.var_t2_dn10 = assign17290_e11778_d_n10;
        locals.var_t2_dn11 = assign17290_e11778_d_n11;
        locals.var_t2_dn14 = assign17290_e11778_d_n14;

        let (assign17300_e11800, assign17300_e11800_d_n0, assign17300_e11800_d_n2, assign17300_e11800_d_n4, assign17300_e11800_d_n5, assign17300_e11800_d_n6, assign17300_e11800_d_n7, assign17300_e11800_d_n8, assign17300_e11800_d_n9, assign17300_e11800_d_n10, assign17300_e11800_d_n11, assign17300_e11800_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign17300_e11782: f64 = (locals.var_egtnom + p.p259);
        let assign17300_e11786: f64 = (locals.var_t1 - locals.var_t2);
        let assign17300_e11787: f64 = (p.p260 * assign17300_e11786);
        let assign17300_e11788: f64 = (assign17300_e11782 + assign17300_e11787);
        let assign17300_e11792: f64 = (locals.var_t1 * locals.var_t1);
        let assign17300_e11795: f64 = (locals.var_t2 * locals.var_t2);
        let assign17300_e11796: f64 = (assign17300_e11792 - assign17300_e11795);
        let assign17300_e11797: f64 = (p.p261 * assign17300_e11796);
        let assign17300_e11798: f64 = (assign17300_e11788 + assign17300_e11797);
        (assign17300_e11798, ((p.p260 * (locals.var_t1_dn0 - locals.var_t2_dn0)) + (p.p261 * (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) - ((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0))))), ((p.p260 * (locals.var_t1_dn2 - locals.var_t2_dn2)) + (p.p261 * (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) - ((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2))))), ((p.p260 * (locals.var_t1_dn4 - locals.var_t2_dn4)) + (p.p261 * (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) - ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4))))), ((p.p260 * (locals.var_t1_dn5 - locals.var_t2_dn5)) + (p.p261 * (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) - ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5))))), ((p.p260 * (locals.var_t1_dn6 - locals.var_t2_dn6)) + (p.p261 * (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) - ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6))))), ((p.p260 * (locals.var_t1_dn7 - locals.var_t2_dn7)) + (p.p261 * (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) - ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7))))), ((p.p260 * (locals.var_t1_dn8 - locals.var_t2_dn8)) + (p.p261 * (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) - ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8))))), ((p.p260 * (locals.var_t1_dn9 - locals.var_t2_dn9)) + (p.p261 * (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) - ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9))))), ((p.p260 * (locals.var_t1_dn10 - locals.var_t2_dn10)) + (p.p261 * (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) - ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10))))), ((p.p260 * (locals.var_t1_dn11 - locals.var_t2_dn11)) + (p.p261 * (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) - ((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11))))), ((p.p260 * (locals.var_t1_dn14 - locals.var_t2_dn14)) + (p.p261 * (((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) - ((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign17300_e11800;
        locals.var_t3_dn0 = assign17300_e11800_d_n0;
        locals.var_t3_dn2 = assign17300_e11800_d_n2;
        locals.var_t3_dn4 = assign17300_e11800_d_n4;
        locals.var_t3_dn5 = assign17300_e11800_d_n5;
        locals.var_t3_dn6 = assign17300_e11800_d_n6;
        locals.var_t3_dn7 = assign17300_e11800_d_n7;
        locals.var_t3_dn8 = assign17300_e11800_d_n8;
        locals.var_t3_dn9 = assign17300_e11800_d_n9;
        locals.var_t3_dn10 = assign17300_e11800_d_n10;
        locals.var_t3_dn11 = assign17300_e11800_d_n11;
        locals.var_t3_dn14 = assign17300_e11800_d_n14;

        let (assign17310_e11805, assign17310_e11805_d_n0, assign17310_e11805_d_n2, assign17310_e11805_d_n4, assign17310_e11805_d_n5, assign17310_e11805_d_n6, assign17310_e11805_d_n7, assign17310_e11805_d_n8, assign17310_e11805_d_n9, assign17310_e11805_d_n10, assign17310_e11805_d_n11, assign17310_e11805_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign17310_e11803: f64 = (locals.var_t3).sqrt();
        (assign17310_e11803, (locals.var_t3_dn0 / (2.0 * assign17310_e11803)), (locals.var_t3_dn2 / (2.0 * assign17310_e11803)), (locals.var_t3_dn4 / (2.0 * assign17310_e11803)), (locals.var_t3_dn5 / (2.0 * assign17310_e11803)), (locals.var_t3_dn6 / (2.0 * assign17310_e11803)), (locals.var_t3_dn7 / (2.0 * assign17310_e11803)), (locals.var_t3_dn8 / (2.0 * assign17310_e11803)), (locals.var_t3_dn9 / (2.0 * assign17310_e11803)), (locals.var_t3_dn10 / (2.0 * assign17310_e11803)), (locals.var_t3_dn11 / (2.0 * assign17310_e11803)), (locals.var_t3_dn14 / (2.0 * assign17310_e11803)),)
    } else {
        (locals.var_egp12, locals.var_egp12_dn0, locals.var_egp12_dn2, locals.var_egp12_dn4, locals.var_egp12_dn5, locals.var_egp12_dn6, locals.var_egp12_dn7, locals.var_egp12_dn8, locals.var_egp12_dn9, locals.var_egp12_dn10, locals.var_egp12_dn11, locals.var_egp12_dn14,)
    }
};
        locals.var_egp12 = assign17310_e11805;
        locals.var_egp12_dn0 = assign17310_e11805_d_n0;
        locals.var_egp12_dn2 = assign17310_e11805_d_n2;
        locals.var_egp12_dn4 = assign17310_e11805_d_n4;
        locals.var_egp12_dn5 = assign17310_e11805_d_n5;
        locals.var_egp12_dn6 = assign17310_e11805_d_n6;
        locals.var_egp12_dn7 = assign17310_e11805_d_n7;
        locals.var_egp12_dn8 = assign17310_e11805_d_n8;
        locals.var_egp12_dn9 = assign17310_e11805_d_n9;
        locals.var_egp12_dn10 = assign17310_e11805_d_n10;
        locals.var_egp12_dn11 = assign17310_e11805_d_n11;
        locals.var_egp12_dn14 = assign17310_e11805_d_n14;

        let (assign17320_e11811, assign17320_e11811_d_n0, assign17320_e11811_d_n2, assign17320_e11811_d_n4, assign17320_e11811_d_n5, assign17320_e11811_d_n6, assign17320_e11811_d_n7, assign17320_e11811_d_n8, assign17320_e11811_d_n9, assign17320_e11811_d_n10, assign17320_e11811_d_n11, assign17320_e11811_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign17320_e11809: f64 = (locals.var_t3 * locals.var_egp12);
        (assign17320_e11809, ((locals.var_t3_dn0 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn0)), ((locals.var_t3_dn2 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn2)), ((locals.var_t3_dn4 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn4)), ((locals.var_t3_dn5 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn5)), ((locals.var_t3_dn6 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn6)), ((locals.var_t3_dn7 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn7)), ((locals.var_t3_dn8 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn8)), ((locals.var_t3_dn9 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn9)), ((locals.var_t3_dn10 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn10)), ((locals.var_t3_dn11 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn11)), ((locals.var_t3_dn14 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn14)),)
    } else {
        (locals.var_egp32, locals.var_egp32_dn0, locals.var_egp32_dn2, locals.var_egp32_dn4, locals.var_egp32_dn5, locals.var_egp32_dn6, locals.var_egp32_dn7, locals.var_egp32_dn8, locals.var_egp32_dn9, locals.var_egp32_dn10, locals.var_egp32_dn11, locals.var_egp32_dn14,)
    }
};
        locals.var_egp32 = assign17320_e11811;
        locals.var_egp32_dn0 = assign17320_e11811_d_n0;
        locals.var_egp32_dn2 = assign17320_e11811_d_n2;
        locals.var_egp32_dn4 = assign17320_e11811_d_n4;
        locals.var_egp32_dn5 = assign17320_e11811_d_n5;
        locals.var_egp32_dn6 = assign17320_e11811_d_n6;
        locals.var_egp32_dn7 = assign17320_e11811_d_n7;
        locals.var_egp32_dn8 = assign17320_e11811_d_n8;
        locals.var_egp32_dn9 = assign17320_e11811_d_n9;
        locals.var_egp32_dn10 = assign17320_e11811_d_n10;
        locals.var_egp32_dn11 = assign17320_e11811_d_n11;
        locals.var_egp32_dn14 = assign17320_e11811_d_n14;

        let (assign17330_e11819, assign17330_e11819_d_n0, assign17330_e11819_d_n2, assign17330_e11819_d_n4, assign17330_e11819_d_n5, assign17330_e11819_d_n6, assign17330_e11819_d_n7, assign17330_e11819_d_n8, assign17330_e11819_d_n9, assign17330_e11819_d_n10, assign17330_e11819_d_n11, assign17330_e11819_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign17330_e11816: f64 = (1.3806226e-23 * locals.var_ttemp);
        let assign17330_e11817: f64 = (1.6021918e-19 / assign17330_e11816);
        (assign17330_e11817, (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn0)) / (assign17330_e11816 * assign17330_e11816))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn2)) / (assign17330_e11816 * assign17330_e11816))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn4)) / (assign17330_e11816 * assign17330_e11816))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn5)) / (assign17330_e11816 * assign17330_e11816))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn6)) / (assign17330_e11816 * assign17330_e11816))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn7)) / (assign17330_e11816 * assign17330_e11816))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn8)) / (assign17330_e11816 * assign17330_e11816))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn9)) / (assign17330_e11816 * assign17330_e11816))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn10)) / (assign17330_e11816 * assign17330_e11816))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn11)) / (assign17330_e11816 * assign17330_e11816))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn14)) / (assign17330_e11816 * assign17330_e11816))),)
    } else {
        (locals.var_beta, locals.var_beta_dn0, locals.var_beta_dn2, locals.var_beta_dn4, locals.var_beta_dn5, locals.var_beta_dn6, locals.var_beta_dn7, locals.var_beta_dn8, locals.var_beta_dn9, locals.var_beta_dn10, locals.var_beta_dn11, locals.var_beta_dn14,)
    }
};
        locals.var_beta = assign17330_e11819;
        locals.var_beta_dn0 = assign17330_e11819_d_n0;
        locals.var_beta_dn2 = assign17330_e11819_d_n2;
        locals.var_beta_dn4 = assign17330_e11819_d_n4;
        locals.var_beta_dn5 = assign17330_e11819_d_n5;
        locals.var_beta_dn6 = assign17330_e11819_d_n6;
        locals.var_beta_dn7 = assign17330_e11819_d_n7;
        locals.var_beta_dn8 = assign17330_e11819_d_n8;
        locals.var_beta_dn9 = assign17330_e11819_d_n9;
        locals.var_beta_dn10 = assign17330_e11819_d_n10;
        locals.var_beta_dn11 = assign17330_e11819_d_n11;
        locals.var_beta_dn14 = assign17330_e11819_d_n14;

        let (assign17340_e11825, assign17340_e11825_d_n0, assign17340_e11825_d_n2, assign17340_e11825_d_n4, assign17340_e11825_d_n5, assign17340_e11825_d_n6, assign17340_e11825_d_n7, assign17340_e11825_d_n8, assign17340_e11825_d_n9, assign17340_e11825_d_n10, assign17340_e11825_d_n11, assign17340_e11825_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign17340_e11823: f64 = (1.0 / locals.var_beta);
        (assign17340_e11823, (-(locals.var_beta_dn0 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn2 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn4 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn5 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn6 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn7 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn8 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn9 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn10 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn11 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn14 / (locals.var_beta * locals.var_beta))),)
    } else {
        (locals.var_beta_inv, locals.var_beta_inv_dn0, locals.var_beta_inv_dn2, locals.var_beta_inv_dn4, locals.var_beta_inv_dn5, locals.var_beta_inv_dn6, locals.var_beta_inv_dn7, locals.var_beta_inv_dn8, locals.var_beta_inv_dn9, locals.var_beta_inv_dn10, locals.var_beta_inv_dn11, locals.var_beta_inv_dn14,)
    }
};
        locals.var_beta_inv = assign17340_e11825;
        locals.var_beta_inv_dn0 = assign17340_e11825_d_n0;
        locals.var_beta_inv_dn2 = assign17340_e11825_d_n2;
        locals.var_beta_inv_dn4 = assign17340_e11825_d_n4;
        locals.var_beta_inv_dn5 = assign17340_e11825_d_n5;
        locals.var_beta_inv_dn6 = assign17340_e11825_d_n6;
        locals.var_beta_inv_dn7 = assign17340_e11825_d_n7;
        locals.var_beta_inv_dn8 = assign17340_e11825_d_n8;
        locals.var_beta_inv_dn9 = assign17340_e11825_d_n9;
        locals.var_beta_inv_dn10 = assign17340_e11825_d_n10;
        locals.var_beta_inv_dn11 = assign17340_e11825_d_n11;
        locals.var_beta_inv_dn14 = assign17340_e11825_d_n14;

        let (assign17350_e11831, assign17350_e11831_d_n0, assign17350_e11831_d_n2, assign17350_e11831_d_n4, assign17350_e11831_d_n5, assign17350_e11831_d_n6, assign17350_e11831_d_n7, assign17350_e11831_d_n8, assign17350_e11831_d_n9, assign17350_e11831_d_n10, assign17350_e11831_d_n11, assign17350_e11831_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign17350_e11829: f64 = (locals.var_beta * locals.var_beta);
        (assign17350_e11829, ((locals.var_beta_dn0 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn0)), ((locals.var_beta_dn2 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn2)), ((locals.var_beta_dn4 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn4)), ((locals.var_beta_dn5 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn5)), ((locals.var_beta_dn6 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn6)), ((locals.var_beta_dn7 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn7)), ((locals.var_beta_dn8 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn8)), ((locals.var_beta_dn9 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn9)), ((locals.var_beta_dn10 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn10)), ((locals.var_beta_dn11 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn11)), ((locals.var_beta_dn14 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn14)),)
    } else {
        (locals.var_beta2, locals.var_beta2_dn0, locals.var_beta2_dn2, locals.var_beta2_dn4, locals.var_beta2_dn5, locals.var_beta2_dn6, locals.var_beta2_dn7, locals.var_beta2_dn8, locals.var_beta2_dn9, locals.var_beta2_dn10, locals.var_beta2_dn11, locals.var_beta2_dn14,)
    }
};
        locals.var_beta2 = assign17350_e11831;
        locals.var_beta2_dn0 = assign17350_e11831_d_n0;
        locals.var_beta2_dn2 = assign17350_e11831_d_n2;
        locals.var_beta2_dn4 = assign17350_e11831_d_n4;
        locals.var_beta2_dn5 = assign17350_e11831_d_n5;
        locals.var_beta2_dn6 = assign17350_e11831_d_n6;
        locals.var_beta2_dn7 = assign17350_e11831_d_n7;
        locals.var_beta2_dn8 = assign17350_e11831_d_n8;
        locals.var_beta2_dn9 = assign17350_e11831_d_n9;
        locals.var_beta2_dn10 = assign17350_e11831_d_n10;
        locals.var_beta2_dn11 = assign17350_e11831_d_n11;
        locals.var_beta2_dn14 = assign17350_e11831_d_n14;

        let (assign17360_e11839,) = {
    if (locals.var_guard356 != 0.0) {
        let assign17360_e11836: f64 = (1.3806226e-23 * locals.var_ktnom);
        let assign17360_e11837: f64 = (1.6021918e-19 / assign17360_e11836);
        (assign17360_e11837,)
    } else {
        (locals.var_betatnom,)
    }
};
        locals.var_betatnom = assign17360_e11839;

        let (assign17370_e11862, assign17370_e11862_d_n0, assign17370_e11862_d_n2, assign17370_e11862_d_n4, assign17370_e11862_d_n5, assign17370_e11862_d_n6, assign17370_e11862_d_n7, assign17370_e11862_d_n8, assign17370_e11862_d_n9, assign17370_e11862_d_n10, assign17370_e11862_d_n11, assign17370_e11862_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign17370_e11844: f64 = (locals.var_log_tratio * 1.5);
        let assign17370_e11845: f64 = (assign17370_e11844).exp();
        let assign17370_e11846: f64 = (1.04e16 * assign17370_e11845);
        let assign17370_e11848: f64 = (-locals.var_eg);
        let assign17370_e11850: f64 = (assign17370_e11848 / 2.0);
        let assign17370_e11852: f64 = (assign17370_e11850 * locals.var_beta);
        let assign17370_e11855: f64 = (locals.var_egtnom / 2.0);
        let assign17370_e11857: f64 = (assign17370_e11855 * locals.var_betatnom);
        let assign17370_e11858: f64 = (assign17370_e11852 + assign17370_e11857);
        let assign17370_e11859: f64 = (assign17370_e11858).exp();
        let assign17370_e11860: f64 = (assign17370_e11846 * assign17370_e11859);
        (assign17370_e11860, (((1.04e16 * (assign17370_e11845 * (locals.var_log_tratio_dn0 * 1.5))) * assign17370_e11859) + (assign17370_e11846 * (assign17370_e11859 * ((((-locals.var_eg_dn0) / 2.0) * locals.var_beta) + (assign17370_e11850 * locals.var_beta_dn0))))), (((1.04e16 * (assign17370_e11845 * (locals.var_log_tratio_dn2 * 1.5))) * assign17370_e11859) + (assign17370_e11846 * (assign17370_e11859 * ((((-locals.var_eg_dn2) / 2.0) * locals.var_beta) + (assign17370_e11850 * locals.var_beta_dn2))))), (((1.04e16 * (assign17370_e11845 * (locals.var_log_tratio_dn4 * 1.5))) * assign17370_e11859) + (assign17370_e11846 * (assign17370_e11859 * ((((-locals.var_eg_dn4) / 2.0) * locals.var_beta) + (assign17370_e11850 * locals.var_beta_dn4))))), (((1.04e16 * (assign17370_e11845 * (locals.var_log_tratio_dn5 * 1.5))) * assign17370_e11859) + (assign17370_e11846 * (assign17370_e11859 * ((((-locals.var_eg_dn5) / 2.0) * locals.var_beta) + (assign17370_e11850 * locals.var_beta_dn5))))), (((1.04e16 * (assign17370_e11845 * (locals.var_log_tratio_dn6 * 1.5))) * assign17370_e11859) + (assign17370_e11846 * (assign17370_e11859 * ((((-locals.var_eg_dn6) / 2.0) * locals.var_beta) + (assign17370_e11850 * locals.var_beta_dn6))))), (((1.04e16 * (assign17370_e11845 * (locals.var_log_tratio_dn7 * 1.5))) * assign17370_e11859) + (assign17370_e11846 * (assign17370_e11859 * ((((-locals.var_eg_dn7) / 2.0) * locals.var_beta) + (assign17370_e11850 * locals.var_beta_dn7))))), (((1.04e16 * (assign17370_e11845 * (locals.var_log_tratio_dn8 * 1.5))) * assign17370_e11859) + (assign17370_e11846 * (assign17370_e11859 * ((((-locals.var_eg_dn8) / 2.0) * locals.var_beta) + (assign17370_e11850 * locals.var_beta_dn8))))), (((1.04e16 * (assign17370_e11845 * (locals.var_log_tratio_dn9 * 1.5))) * assign17370_e11859) + (assign17370_e11846 * (assign17370_e11859 * ((((-locals.var_eg_dn9) / 2.0) * locals.var_beta) + (assign17370_e11850 * locals.var_beta_dn9))))), (((1.04e16 * (assign17370_e11845 * (locals.var_log_tratio_dn10 * 1.5))) * assign17370_e11859) + (assign17370_e11846 * (assign17370_e11859 * ((((-locals.var_eg_dn10) / 2.0) * locals.var_beta) + (assign17370_e11850 * locals.var_beta_dn10))))), (((1.04e16 * (assign17370_e11845 * (locals.var_log_tratio_dn11 * 1.5))) * assign17370_e11859) + (assign17370_e11846 * (assign17370_e11859 * ((((-locals.var_eg_dn11) / 2.0) * locals.var_beta) + (assign17370_e11850 * locals.var_beta_dn11))))), (((1.04e16 * (assign17370_e11845 * (locals.var_log_tratio_dn14 * 1.5))) * assign17370_e11859) + (assign17370_e11846 * (assign17370_e11859 * ((((-locals.var_eg_dn14) / 2.0) * locals.var_beta) + (assign17370_e11850 * locals.var_beta_dn14))))),)
    } else {
        (locals.var_nin, locals.var_nin_dn0, locals.var_nin_dn2, locals.var_nin_dn4, locals.var_nin_dn5, locals.var_nin_dn6, locals.var_nin_dn7, locals.var_nin_dn8, locals.var_nin_dn9, locals.var_nin_dn10, locals.var_nin_dn11, locals.var_nin_dn14,)
    }
};
        locals.var_nin = assign17370_e11862;
        locals.var_nin_dn0 = assign17370_e11862_d_n0;
        locals.var_nin_dn2 = assign17370_e11862_d_n2;
        locals.var_nin_dn4 = assign17370_e11862_d_n4;
        locals.var_nin_dn5 = assign17370_e11862_d_n5;
        locals.var_nin_dn6 = assign17370_e11862_d_n6;
        locals.var_nin_dn7 = assign17370_e11862_d_n7;
        locals.var_nin_dn8 = assign17370_e11862_d_n8;
        locals.var_nin_dn9 = assign17370_e11862_d_n9;
        locals.var_nin_dn10 = assign17370_e11862_d_n10;
        locals.var_nin_dn11 = assign17370_e11862_d_n11;
        locals.var_nin_dn14 = assign17370_e11862_d_n14;

        let (assign17380_e11869, assign17380_e11869_d_n0, assign17380_e11869_d_n2, assign17380_e11869_d_n4, assign17380_e11869_d_n5, assign17380_e11869_d_n6, assign17380_e11869_d_n7, assign17380_e11869_d_n8, assign17380_e11869_d_n9, assign17380_e11869_d_n10, assign17380_e11869_d_n11, assign17380_e11869_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign17380_e11866: f64 = (locals.var_log_tratio * locals.var_uc_muetmp);
        let assign17380_e11867: f64 = (assign17380_e11866).exp();
        (assign17380_e11867, (assign17380_e11867 * (locals.var_log_tratio_dn0 * locals.var_uc_muetmp)), (assign17380_e11867 * (locals.var_log_tratio_dn2 * locals.var_uc_muetmp)), (assign17380_e11867 * (locals.var_log_tratio_dn4 * locals.var_uc_muetmp)), (assign17380_e11867 * (locals.var_log_tratio_dn5 * locals.var_uc_muetmp)), (assign17380_e11867 * (locals.var_log_tratio_dn6 * locals.var_uc_muetmp)), (assign17380_e11867 * (locals.var_log_tratio_dn7 * locals.var_uc_muetmp)), (assign17380_e11867 * (locals.var_log_tratio_dn8 * locals.var_uc_muetmp)), (assign17380_e11867 * (locals.var_log_tratio_dn9 * locals.var_uc_muetmp)), (assign17380_e11867 * (locals.var_log_tratio_dn10 * locals.var_uc_muetmp)), (assign17380_e11867 * (locals.var_log_tratio_dn11 * locals.var_uc_muetmp)), (assign17380_e11867 * (locals.var_log_tratio_dn14 * locals.var_uc_muetmp)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign17380_e11869;
        locals.var_t1_dn0 = assign17380_e11869_d_n0;
        locals.var_t1_dn2 = assign17380_e11869_d_n2;
        locals.var_t1_dn4 = assign17380_e11869_d_n4;
        locals.var_t1_dn5 = assign17380_e11869_d_n5;
        locals.var_t1_dn6 = assign17380_e11869_d_n6;
        locals.var_t1_dn7 = assign17380_e11869_d_n7;
        locals.var_t1_dn8 = assign17380_e11869_d_n8;
        locals.var_t1_dn9 = assign17380_e11869_d_n9;
        locals.var_t1_dn10 = assign17380_e11869_d_n10;
        locals.var_t1_dn11 = assign17380_e11869_d_n11;
        locals.var_t1_dn14 = assign17380_e11869_d_n14;

        let (assign17390_e11875, assign17390_e11875_d_n0, assign17390_e11875_d_n2, assign17390_e11875_d_n4, assign17390_e11875_d_n5, assign17390_e11875_d_n6, assign17390_e11875_d_n7, assign17390_e11875_d_n8, assign17390_e11875_d_n9, assign17390_e11875_d_n10, assign17390_e11875_d_n11, assign17390_e11875_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign17390_e11873: f64 = (locals.var_t1 / locals.var_mueph);
        (assign17390_e11873, (((locals.var_t1_dn0 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn0)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn2 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn2)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn4 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn4)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn5 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn5)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn6 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn6)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn7 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn7)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn8 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn8)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn9 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn9)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn10 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn10)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn11 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn11)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn14 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn14)) / (locals.var_mueph * locals.var_mueph)),)
    } else {
        (locals.var_mphn0, locals.var_mphn0_dn0, locals.var_mphn0_dn2, locals.var_mphn0_dn4, locals.var_mphn0_dn5, locals.var_mphn0_dn6, locals.var_mphn0_dn7, locals.var_mphn0_dn8, locals.var_mphn0_dn9, locals.var_mphn0_dn10, locals.var_mphn0_dn11, locals.var_mphn0_dn14,)
    }
};
        locals.var_mphn0 = assign17390_e11875;
        locals.var_mphn0_dn0 = assign17390_e11875_d_n0;
        locals.var_mphn0_dn2 = assign17390_e11875_d_n2;
        locals.var_mphn0_dn4 = assign17390_e11875_d_n4;
        locals.var_mphn0_dn5 = assign17390_e11875_d_n5;
        locals.var_mphn0_dn6 = assign17390_e11875_d_n6;
        locals.var_mphn0_dn7 = assign17390_e11875_d_n7;
        locals.var_mphn0_dn8 = assign17390_e11875_d_n8;
        locals.var_mphn0_dn9 = assign17390_e11875_d_n9;
        locals.var_mphn0_dn10 = assign17390_e11875_d_n10;
        locals.var_mphn0_dn11 = assign17390_e11875_d_n11;
        locals.var_mphn0_dn14 = assign17390_e11875_d_n14;

        let assign17400_e11882: f64 = if ((locals.var_uc_codep != 0.0) && (locals.var_uc_codep < 3.0)) { 1.0 } else { 0.0 };
        locals.var_guard359 = assign17400_e11882;

        let (assign17410_e11897, assign17410_e11897_d_n0, assign17410_e11897_d_n2, assign17410_e11897_d_n4, assign17410_e11897_d_n5, assign17410_e11897_d_n6, assign17410_e11897_d_n7, assign17410_e11897_d_n8, assign17410_e11897_d_n9, assign17410_e11897_d_n10, assign17410_e11897_d_n11, assign17410_e11897_d_n14,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard359 != 0.0)) {
        let assign17410_e11888: f64 = (2.0 * 1.034943e-10);
        let assign17410_e11890: f64 = (assign17410_e11888 * 1.6021918e-19);
        let assign17410_e11892: f64 = (assign17410_e11890 * locals.var_uc_ndepm);
        let assign17410_e11894: f64 = (assign17410_e11892 * locals.var_beta_inv);
        let assign17410_e11895: f64 = (assign17410_e11894).sqrt();
        (assign17410_e11895, ((((assign17410_e11890 * locals.var_uc_ndepm_dn0) * locals.var_beta_inv) + (assign17410_e11892 * locals.var_beta_inv_dn0)) / (2.0 * assign17410_e11895)), ((((assign17410_e11890 * locals.var_uc_ndepm_dn2) * locals.var_beta_inv) + (assign17410_e11892 * locals.var_beta_inv_dn2)) / (2.0 * assign17410_e11895)), ((((assign17410_e11890 * locals.var_uc_ndepm_dn4) * locals.var_beta_inv) + (assign17410_e11892 * locals.var_beta_inv_dn4)) / (2.0 * assign17410_e11895)), ((((assign17410_e11890 * locals.var_uc_ndepm_dn5) * locals.var_beta_inv) + (assign17410_e11892 * locals.var_beta_inv_dn5)) / (2.0 * assign17410_e11895)), ((((assign17410_e11890 * locals.var_uc_ndepm_dn6) * locals.var_beta_inv) + (assign17410_e11892 * locals.var_beta_inv_dn6)) / (2.0 * assign17410_e11895)), ((((assign17410_e11890 * locals.var_uc_ndepm_dn7) * locals.var_beta_inv) + (assign17410_e11892 * locals.var_beta_inv_dn7)) / (2.0 * assign17410_e11895)), ((((assign17410_e11890 * locals.var_uc_ndepm_dn8) * locals.var_beta_inv) + (assign17410_e11892 * locals.var_beta_inv_dn8)) / (2.0 * assign17410_e11895)), ((((assign17410_e11890 * locals.var_uc_ndepm_dn9) * locals.var_beta_inv) + (assign17410_e11892 * locals.var_beta_inv_dn9)) / (2.0 * assign17410_e11895)), ((((assign17410_e11890 * locals.var_uc_ndepm_dn10) * locals.var_beta_inv) + (assign17410_e11892 * locals.var_beta_inv_dn10)) / (2.0 * assign17410_e11895)), ((((assign17410_e11890 * locals.var_uc_ndepm_dn11) * locals.var_beta_inv) + (assign17410_e11892 * locals.var_beta_inv_dn11)) / (2.0 * assign17410_e11895)), ((((assign17410_e11890 * locals.var_uc_ndepm_dn14) * locals.var_beta_inv) + (assign17410_e11892 * locals.var_beta_inv_dn14)) / (2.0 * assign17410_e11895)),)
    } else {
        (locals.var_cnst0, locals.var_cnst0_dn0, locals.var_cnst0_dn2, locals.var_cnst0_dn4, locals.var_cnst0_dn5, locals.var_cnst0_dn6, locals.var_cnst0_dn7, locals.var_cnst0_dn8, locals.var_cnst0_dn9, locals.var_cnst0_dn10, locals.var_cnst0_dn11, locals.var_cnst0_dn14,)
    }
};
        locals.var_cnst0 = assign17410_e11897;
        locals.var_cnst0_dn0 = assign17410_e11897_d_n0;
        locals.var_cnst0_dn2 = assign17410_e11897_d_n2;
        locals.var_cnst0_dn4 = assign17410_e11897_d_n4;
        locals.var_cnst0_dn5 = assign17410_e11897_d_n5;
        locals.var_cnst0_dn6 = assign17410_e11897_d_n6;
        locals.var_cnst0_dn7 = assign17410_e11897_d_n7;
        locals.var_cnst0_dn8 = assign17410_e11897_d_n8;
        locals.var_cnst0_dn9 = assign17410_e11897_d_n9;
        locals.var_cnst0_dn10 = assign17410_e11897_d_n10;
        locals.var_cnst0_dn11 = assign17410_e11897_d_n11;
        locals.var_cnst0_dn14 = assign17410_e11897_d_n14;

        let (assign17420_e11909, assign17420_e11909_d_n0, assign17420_e11909_d_n2, assign17420_e11909_d_n4, assign17420_e11909_d_n5, assign17420_e11909_d_n6, assign17420_e11909_d_n7, assign17420_e11909_d_n8, assign17420_e11909_d_n9, assign17420_e11909_d_n10, assign17420_e11909_d_n11, assign17420_e11909_d_n14,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard359 != 0.0)) {
        let assign17420_e11903: f64 = (locals.var_nin * locals.var_nin);
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_uc_ndepm;
        let assign17420_e11905: f64 = (assign17420_e11903 * __rspice_inv_cse_0);
        let assign17420_e11907: f64 = (assign17420_e11905 * __rspice_inv_cse_0);
        (assign17420_e11907, ((((((((locals.var_nin_dn0 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn0)) * locals.var_uc_ndepm) - (assign17420_e11903 * locals.var_uc_ndepm_dn0)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17420_e11905 * locals.var_uc_ndepm_dn0)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn2 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn2)) * locals.var_uc_ndepm) - (assign17420_e11903 * locals.var_uc_ndepm_dn2)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17420_e11905 * locals.var_uc_ndepm_dn2)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn4 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn4)) * locals.var_uc_ndepm) - (assign17420_e11903 * locals.var_uc_ndepm_dn4)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17420_e11905 * locals.var_uc_ndepm_dn4)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn5 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn5)) * locals.var_uc_ndepm) - (assign17420_e11903 * locals.var_uc_ndepm_dn5)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17420_e11905 * locals.var_uc_ndepm_dn5)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn6 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn6)) * locals.var_uc_ndepm) - (assign17420_e11903 * locals.var_uc_ndepm_dn6)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17420_e11905 * locals.var_uc_ndepm_dn6)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn7 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn7)) * locals.var_uc_ndepm) - (assign17420_e11903 * locals.var_uc_ndepm_dn7)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17420_e11905 * locals.var_uc_ndepm_dn7)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn8 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn8)) * locals.var_uc_ndepm) - (assign17420_e11903 * locals.var_uc_ndepm_dn8)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17420_e11905 * locals.var_uc_ndepm_dn8)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn9 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn9)) * locals.var_uc_ndepm) - (assign17420_e11903 * locals.var_uc_ndepm_dn9)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17420_e11905 * locals.var_uc_ndepm_dn9)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn10 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn10)) * locals.var_uc_ndepm) - (assign17420_e11903 * locals.var_uc_ndepm_dn10)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17420_e11905 * locals.var_uc_ndepm_dn10)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn11 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn11)) * locals.var_uc_ndepm) - (assign17420_e11903 * locals.var_uc_ndepm_dn11)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17420_e11905 * locals.var_uc_ndepm_dn11)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn14 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn14)) * locals.var_uc_ndepm) - (assign17420_e11903 * locals.var_uc_ndepm_dn14)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17420_e11905 * locals.var_uc_ndepm_dn14)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)),)
    } else {
        (locals.var_cnst1, locals.var_cnst1_dn0, locals.var_cnst1_dn2, locals.var_cnst1_dn4, locals.var_cnst1_dn5, locals.var_cnst1_dn6, locals.var_cnst1_dn7, locals.var_cnst1_dn8, locals.var_cnst1_dn9, locals.var_cnst1_dn10, locals.var_cnst1_dn11, locals.var_cnst1_dn14,)
    }
};
        locals.var_cnst1 = assign17420_e11909;
        locals.var_cnst1_dn0 = assign17420_e11909_d_n0;
        locals.var_cnst1_dn2 = assign17420_e11909_d_n2;
        locals.var_cnst1_dn4 = assign17420_e11909_d_n4;
        locals.var_cnst1_dn5 = assign17420_e11909_d_n5;
        locals.var_cnst1_dn6 = assign17420_e11909_d_n6;
        locals.var_cnst1_dn7 = assign17420_e11909_d_n7;
        locals.var_cnst1_dn8 = assign17420_e11909_d_n8;
        locals.var_cnst1_dn9 = assign17420_e11909_d_n9;
        locals.var_cnst1_dn10 = assign17420_e11909_d_n10;
        locals.var_cnst1_dn11 = assign17420_e11909_d_n11;
        locals.var_cnst1_dn14 = assign17420_e11909_d_n14;

        let (assign17430_e11922, assign17430_e11922_d_n0, assign17430_e11922_d_n2, assign17430_e11922_d_n4, assign17430_e11922_d_n5, assign17430_e11922_d_n6, assign17430_e11922_d_n7, assign17430_e11922_d_n8, assign17430_e11922_d_n9, assign17430_e11922_d_n10, assign17430_e11922_d_n11, assign17430_e11922_d_n14,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard359 != 0.0)) {
        let assign17430_e11915: f64 = (2.0 * locals.var_beta_inv);
        let assign17430_e11918: f64 = (locals.var_uc_ndepm / locals.var_nin);
        let assign17430_e11919: f64 = (assign17430_e11918).ln();
        let assign17430_e11920: f64 = (assign17430_e11915 * assign17430_e11919);
        (assign17430_e11920, (((2.0 * locals.var_beta_inv_dn0) * assign17430_e11919) + (assign17430_e11915 * ((((locals.var_uc_ndepm_dn0 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign17430_e11918))), (((2.0 * locals.var_beta_inv_dn2) * assign17430_e11919) + (assign17430_e11915 * ((((locals.var_uc_ndepm_dn2 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign17430_e11918))), (((2.0 * locals.var_beta_inv_dn4) * assign17430_e11919) + (assign17430_e11915 * ((((locals.var_uc_ndepm_dn4 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign17430_e11918))), (((2.0 * locals.var_beta_inv_dn5) * assign17430_e11919) + (assign17430_e11915 * ((((locals.var_uc_ndepm_dn5 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign17430_e11918))), (((2.0 * locals.var_beta_inv_dn6) * assign17430_e11919) + (assign17430_e11915 * ((((locals.var_uc_ndepm_dn6 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign17430_e11918))), (((2.0 * locals.var_beta_inv_dn7) * assign17430_e11919) + (assign17430_e11915 * ((((locals.var_uc_ndepm_dn7 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign17430_e11918))), (((2.0 * locals.var_beta_inv_dn8) * assign17430_e11919) + (assign17430_e11915 * ((((locals.var_uc_ndepm_dn8 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign17430_e11918))), (((2.0 * locals.var_beta_inv_dn9) * assign17430_e11919) + (assign17430_e11915 * ((((locals.var_uc_ndepm_dn9 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign17430_e11918))), (((2.0 * locals.var_beta_inv_dn10) * assign17430_e11919) + (assign17430_e11915 * ((((locals.var_uc_ndepm_dn10 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign17430_e11918))), (((2.0 * locals.var_beta_inv_dn11) * assign17430_e11919) + (assign17430_e11915 * ((((locals.var_uc_ndepm_dn11 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)) / assign17430_e11918))), (((2.0 * locals.var_beta_inv_dn14) * assign17430_e11919) + (assign17430_e11915 * ((((locals.var_uc_ndepm_dn14 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn14)) / (locals.var_nin * locals.var_nin)) / assign17430_e11918))),)
    } else {
        (locals.var_pb2n, locals.var_pb2n_dn0, locals.var_pb2n_dn2, locals.var_pb2n_dn4, locals.var_pb2n_dn5, locals.var_pb2n_dn6, locals.var_pb2n_dn7, locals.var_pb2n_dn8, locals.var_pb2n_dn9, locals.var_pb2n_dn10, locals.var_pb2n_dn11, locals.var_pb2n_dn14,)
    }
};
        locals.var_pb2n = assign17430_e11922;
        locals.var_pb2n_dn0 = assign17430_e11922_d_n0;
        locals.var_pb2n_dn2 = assign17430_e11922_d_n2;
        locals.var_pb2n_dn4 = assign17430_e11922_d_n4;
        locals.var_pb2n_dn5 = assign17430_e11922_d_n5;
        locals.var_pb2n_dn6 = assign17430_e11922_d_n6;
        locals.var_pb2n_dn7 = assign17430_e11922_d_n7;
        locals.var_pb2n_dn8 = assign17430_e11922_d_n8;
        locals.var_pb2n_dn9 = assign17430_e11922_d_n9;
        locals.var_pb2n_dn10 = assign17430_e11922_d_n10;
        locals.var_pb2n_dn11 = assign17430_e11922_d_n11;
        locals.var_pb2n_dn14 = assign17430_e11922_d_n14;

        let (assign17440_e11937, assign17440_e11937_d_n0, assign17440_e11937_d_n2, assign17440_e11937_d_n4, assign17440_e11937_d_n5, assign17440_e11937_d_n6, assign17440_e11937_d_n7, assign17440_e11937_d_n8, assign17440_e11937_d_n9, assign17440_e11937_d_n10, assign17440_e11937_d_n11, assign17440_e11937_d_n14,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard359 != 0.0)) {
        let assign17440_e11929: f64 = (locals.var_uc_ndepm * locals.var_ef_nsubc);
        let __rspice_inv_cse_1: f64 = 1.0 / locals.var_nin;
        let assign17440_e11931: f64 = (assign17440_e11929 * __rspice_inv_cse_1);
        let assign17440_e11933: f64 = (assign17440_e11931 * __rspice_inv_cse_1);
        let assign17440_e11934: f64 = (assign17440_e11933).ln();
        let assign17440_e11935: f64 = (locals.var_beta_inv * assign17440_e11934);
        (assign17440_e11935, ((locals.var_beta_inv_dn0 * assign17440_e11934) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn0 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn0)) * locals.var_nin) - (assign17440_e11929 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17440_e11931 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign17440_e11933))), ((locals.var_beta_inv_dn2 * assign17440_e11934) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn2 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn2)) * locals.var_nin) - (assign17440_e11929 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17440_e11931 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign17440_e11933))), ((locals.var_beta_inv_dn4 * assign17440_e11934) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn4 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn4)) * locals.var_nin) - (assign17440_e11929 * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17440_e11931 * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign17440_e11933))), ((locals.var_beta_inv_dn5 * assign17440_e11934) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn5 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn5)) * locals.var_nin) - (assign17440_e11929 * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17440_e11931 * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign17440_e11933))), ((locals.var_beta_inv_dn6 * assign17440_e11934) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn6 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn6)) * locals.var_nin) - (assign17440_e11929 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17440_e11931 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign17440_e11933))), ((locals.var_beta_inv_dn7 * assign17440_e11934) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn7 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn7)) * locals.var_nin) - (assign17440_e11929 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17440_e11931 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign17440_e11933))), ((locals.var_beta_inv_dn8 * assign17440_e11934) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn8 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn8)) * locals.var_nin) - (assign17440_e11929 * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17440_e11931 * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign17440_e11933))), ((locals.var_beta_inv_dn9 * assign17440_e11934) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn9 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn9)) * locals.var_nin) - (assign17440_e11929 * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17440_e11931 * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign17440_e11933))), ((locals.var_beta_inv_dn10 * assign17440_e11934) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn10 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn10)) * locals.var_nin) - (assign17440_e11929 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17440_e11931 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign17440_e11933))), ((locals.var_beta_inv_dn11 * assign17440_e11934) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn11 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn11)) * locals.var_nin) - (assign17440_e11929 * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17440_e11931 * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)) / assign17440_e11933))), ((locals.var_beta_inv_dn14 * assign17440_e11934) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn14 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn14)) * locals.var_nin) - (assign17440_e11929 * locals.var_nin_dn14)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17440_e11931 * locals.var_nin_dn14)) / (locals.var_nin * locals.var_nin)) / assign17440_e11933))),)
    } else {
        (locals.var_vbipn, locals.var_vbipn_dn0, locals.var_vbipn_dn2, locals.var_vbipn_dn4, locals.var_vbipn_dn5, locals.var_vbipn_dn6, locals.var_vbipn_dn7, locals.var_vbipn_dn8, locals.var_vbipn_dn9, locals.var_vbipn_dn10, locals.var_vbipn_dn11, locals.var_vbipn_dn14,)
    }
};
        locals.var_vbipn = assign17440_e11937;
        locals.var_vbipn_dn0 = assign17440_e11937_d_n0;
        locals.var_vbipn_dn2 = assign17440_e11937_d_n2;
        locals.var_vbipn_dn4 = assign17440_e11937_d_n4;
        locals.var_vbipn_dn5 = assign17440_e11937_d_n5;
        locals.var_vbipn_dn6 = assign17440_e11937_d_n6;
        locals.var_vbipn_dn7 = assign17440_e11937_d_n7;
        locals.var_vbipn_dn8 = assign17440_e11937_d_n8;
        locals.var_vbipn_dn9 = assign17440_e11937_d_n9;
        locals.var_vbipn_dn10 = assign17440_e11937_d_n10;
        locals.var_vbipn_dn11 = assign17440_e11937_d_n11;
        locals.var_vbipn_dn14 = assign17440_e11937_d_n14;

    }

    pub(super) fn stamp_transient_block_38(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign17450_e11946, assign17450_e11946_d_n0, assign17450_e11946_d_n2, assign17450_e11946_d_n4, assign17450_e11946_d_n5, assign17450_e11946_d_n6, assign17450_e11946_d_n7, assign17450_e11946_d_n8, assign17450_e11946_d_n9, assign17450_e11946_d_n10, assign17450_e11946_d_n11, assign17450_e11946_d_n14,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard359 != 0.0)) {
        let assign17450_e11943: f64 = (locals.var_log_tratio * p.p380);
        let assign17450_e11944: f64 = (assign17450_e11943).exp();
        (assign17450_e11944, (assign17450_e11944 * (locals.var_log_tratio_dn0 * p.p380)), (assign17450_e11944 * (locals.var_log_tratio_dn2 * p.p380)), (assign17450_e11944 * (locals.var_log_tratio_dn4 * p.p380)), (assign17450_e11944 * (locals.var_log_tratio_dn5 * p.p380)), (assign17450_e11944 * (locals.var_log_tratio_dn6 * p.p380)), (assign17450_e11944 * (locals.var_log_tratio_dn7 * p.p380)), (assign17450_e11944 * (locals.var_log_tratio_dn8 * p.p380)), (assign17450_e11944 * (locals.var_log_tratio_dn9 * p.p380)), (assign17450_e11944 * (locals.var_log_tratio_dn10 * p.p380)), (assign17450_e11944 * (locals.var_log_tratio_dn11 * p.p380)), (assign17450_e11944 * (locals.var_log_tratio_dn14 * p.p380)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign17450_e11946;
        locals.var_t1_dn0 = assign17450_e11946_d_n0;
        locals.var_t1_dn2 = assign17450_e11946_d_n2;
        locals.var_t1_dn4 = assign17450_e11946_d_n4;
        locals.var_t1_dn5 = assign17450_e11946_d_n5;
        locals.var_t1_dn6 = assign17450_e11946_d_n6;
        locals.var_t1_dn7 = assign17450_e11946_d_n7;
        locals.var_t1_dn8 = assign17450_e11946_d_n8;
        locals.var_t1_dn9 = assign17450_e11946_d_n9;
        locals.var_t1_dn10 = assign17450_e11946_d_n10;
        locals.var_t1_dn11 = assign17450_e11946_d_n11;
        locals.var_t1_dn14 = assign17450_e11946_d_n14;

        let (assign17460_e11954, assign17460_e11954_d_n0, assign17460_e11954_d_n2, assign17460_e11954_d_n4, assign17460_e11954_d_n5, assign17460_e11954_d_n6, assign17460_e11954_d_n7, assign17460_e11954_d_n8, assign17460_e11954_d_n9, assign17460_e11954_d_n10, assign17460_e11954_d_n11, assign17460_e11954_d_n14,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard359 != 0.0)) {
        let assign17460_e11952: f64 = (locals.var_t1 / locals.var_uc_depmueph1);
        (assign17460_e11952, (locals.var_t1_dn0 / locals.var_uc_depmueph1), (locals.var_t1_dn2 / locals.var_uc_depmueph1), (locals.var_t1_dn4 / locals.var_uc_depmueph1), (locals.var_t1_dn5 / locals.var_uc_depmueph1), (locals.var_t1_dn6 / locals.var_uc_depmueph1), (locals.var_t1_dn7 / locals.var_uc_depmueph1), (locals.var_t1_dn8 / locals.var_uc_depmueph1), (locals.var_t1_dn9 / locals.var_uc_depmueph1), (locals.var_t1_dn10 / locals.var_uc_depmueph1), (locals.var_t1_dn11 / locals.var_uc_depmueph1), (locals.var_t1_dn14 / locals.var_uc_depmueph1),)
    } else {
        (locals.var_depmphn0, locals.var_depmphn0_dn0, locals.var_depmphn0_dn2, locals.var_depmphn0_dn4, locals.var_depmphn0_dn5, locals.var_depmphn0_dn6, locals.var_depmphn0_dn7, locals.var_depmphn0_dn8, locals.var_depmphn0_dn9, locals.var_depmphn0_dn10, locals.var_depmphn0_dn11, locals.var_depmphn0_dn14,)
    }
};
        locals.var_depmphn0 = assign17460_e11954;
        locals.var_depmphn0_dn0 = assign17460_e11954_d_n0;
        locals.var_depmphn0_dn2 = assign17460_e11954_d_n2;
        locals.var_depmphn0_dn4 = assign17460_e11954_d_n4;
        locals.var_depmphn0_dn5 = assign17460_e11954_d_n5;
        locals.var_depmphn0_dn6 = assign17460_e11954_d_n6;
        locals.var_depmphn0_dn7 = assign17460_e11954_d_n7;
        locals.var_depmphn0_dn8 = assign17460_e11954_d_n8;
        locals.var_depmphn0_dn9 = assign17460_e11954_d_n9;
        locals.var_depmphn0_dn10 = assign17460_e11954_d_n10;
        locals.var_depmphn0_dn11 = assign17460_e11954_d_n11;
        locals.var_depmphn0_dn14 = assign17460_e11954_d_n14;

        let (assign17470_e11976, assign17470_e11976_d_n0, assign17470_e11976_d_n2, assign17470_e11976_d_n4, assign17470_e11976_d_n5, assign17470_e11976_d_n6, assign17470_e11976_d_n7, assign17470_e11976_d_n8, assign17470_e11976_d_n9, assign17470_e11976_d_n10, assign17470_e11976_d_n11, assign17470_e11976_d_n14,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard359 != 0.0)) {
        let assign17470_e11961: f64 = (0.4 * locals.var_tratio);
        let assign17470_e11962: f64 = (1.8 + assign17470_e11961);
        let assign17470_e11965: f64 = (0.1 * locals.var_tratio);
        let assign17470_e11967: f64 = (assign17470_e11965 * locals.var_tratio);
        let assign17470_e11968: f64 = (assign17470_e11962 + assign17470_e11967);
        let assign17470_e11972: f64 = (1.0 - locals.var_tratio);
        let assign17470_e11973: f64 = (p.p379 * assign17470_e11972);
        let assign17470_e11974: f64 = (assign17470_e11968 - assign17470_e11973);
        (assign17470_e11974, (((0.4 * locals.var_tratio_dn0) + (((0.1 * locals.var_tratio_dn0) * locals.var_tratio) + (assign17470_e11965 * locals.var_tratio_dn0))) - (p.p379 * (-locals.var_tratio_dn0))), (((0.4 * locals.var_tratio_dn2) + (((0.1 * locals.var_tratio_dn2) * locals.var_tratio) + (assign17470_e11965 * locals.var_tratio_dn2))) - (p.p379 * (-locals.var_tratio_dn2))), (((0.4 * locals.var_tratio_dn4) + (((0.1 * locals.var_tratio_dn4) * locals.var_tratio) + (assign17470_e11965 * locals.var_tratio_dn4))) - (p.p379 * (-locals.var_tratio_dn4))), (((0.4 * locals.var_tratio_dn5) + (((0.1 * locals.var_tratio_dn5) * locals.var_tratio) + (assign17470_e11965 * locals.var_tratio_dn5))) - (p.p379 * (-locals.var_tratio_dn5))), (((0.4 * locals.var_tratio_dn6) + (((0.1 * locals.var_tratio_dn6) * locals.var_tratio) + (assign17470_e11965 * locals.var_tratio_dn6))) - (p.p379 * (-locals.var_tratio_dn6))), (((0.4 * locals.var_tratio_dn7) + (((0.1 * locals.var_tratio_dn7) * locals.var_tratio) + (assign17470_e11965 * locals.var_tratio_dn7))) - (p.p379 * (-locals.var_tratio_dn7))), (((0.4 * locals.var_tratio_dn8) + (((0.1 * locals.var_tratio_dn8) * locals.var_tratio) + (assign17470_e11965 * locals.var_tratio_dn8))) - (p.p379 * (-locals.var_tratio_dn8))), (((0.4 * locals.var_tratio_dn9) + (((0.1 * locals.var_tratio_dn9) * locals.var_tratio) + (assign17470_e11965 * locals.var_tratio_dn9))) - (p.p379 * (-locals.var_tratio_dn9))), (((0.4 * locals.var_tratio_dn10) + (((0.1 * locals.var_tratio_dn10) * locals.var_tratio) + (assign17470_e11965 * locals.var_tratio_dn10))) - (p.p379 * (-locals.var_tratio_dn10))), (((0.4 * locals.var_tratio_dn11) + (((0.1 * locals.var_tratio_dn11) * locals.var_tratio) + (assign17470_e11965 * locals.var_tratio_dn11))) - (p.p379 * (-locals.var_tratio_dn11))), (((0.4 * locals.var_tratio_dn14) + (((0.1 * locals.var_tratio_dn14) * locals.var_tratio) + (assign17470_e11965 * locals.var_tratio_dn14))) - (p.p379 * (-locals.var_tratio_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign17470_e11976;
        locals.var_t0_dn0 = assign17470_e11976_d_n0;
        locals.var_t0_dn2 = assign17470_e11976_d_n2;
        locals.var_t0_dn4 = assign17470_e11976_d_n4;
        locals.var_t0_dn5 = assign17470_e11976_d_n5;
        locals.var_t0_dn6 = assign17470_e11976_d_n6;
        locals.var_t0_dn7 = assign17470_e11976_d_n7;
        locals.var_t0_dn8 = assign17470_e11976_d_n8;
        locals.var_t0_dn9 = assign17470_e11976_d_n9;
        locals.var_t0_dn10 = assign17470_e11976_d_n10;
        locals.var_t0_dn11 = assign17470_e11976_d_n11;
        locals.var_t0_dn14 = assign17470_e11976_d_n14;

        let (assign17480_e11984, assign17480_e11984_d_n0, assign17480_e11984_d_n2, assign17480_e11984_d_n4, assign17480_e11984_d_n5, assign17480_e11984_d_n6, assign17480_e11984_d_n7, assign17480_e11984_d_n8, assign17480_e11984_d_n9, assign17480_e11984_d_n10, assign17480_e11984_d_n11, assign17480_e11984_d_n14,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard359 != 0.0)) {
        let assign17480_e11982: f64 = (locals.var_uc_depvmax / locals.var_t0);
        (assign17480_e11982, (((locals.var_uc_depvmax_dn0 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn0)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn2 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn2)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn4 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn5 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn6 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn7 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn8 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn9 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn10 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn11 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn14 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn14)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_uc_depvmax, locals.var_uc_depvmax_dn0, locals.var_uc_depvmax_dn2, locals.var_uc_depvmax_dn4, locals.var_uc_depvmax_dn5, locals.var_uc_depvmax_dn6, locals.var_uc_depvmax_dn7, locals.var_uc_depvmax_dn8, locals.var_uc_depvmax_dn9, locals.var_uc_depvmax_dn10, locals.var_uc_depvmax_dn11, locals.var_uc_depvmax_dn14,)
    }
};
        locals.var_uc_depvmax = assign17480_e11984;
        locals.var_uc_depvmax_dn0 = assign17480_e11984_d_n0;
        locals.var_uc_depvmax_dn2 = assign17480_e11984_d_n2;
        locals.var_uc_depvmax_dn4 = assign17480_e11984_d_n4;
        locals.var_uc_depvmax_dn5 = assign17480_e11984_d_n5;
        locals.var_uc_depvmax_dn6 = assign17480_e11984_d_n6;
        locals.var_uc_depvmax_dn7 = assign17480_e11984_d_n7;
        locals.var_uc_depvmax_dn8 = assign17480_e11984_d_n8;
        locals.var_uc_depvmax_dn9 = assign17480_e11984_d_n9;
        locals.var_uc_depvmax_dn10 = assign17480_e11984_d_n10;
        locals.var_uc_depvmax_dn11 = assign17480_e11984_d_n11;
        locals.var_uc_depvmax_dn14 = assign17480_e11984_d_n14;

        let assign17500_e11992: f64 = if locals.var_uc_depvmax < 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard361 = assign17500_e11992;

        let (assign17510_e12000, assign17510_e12000_d_n0, assign17510_e12000_d_n2, assign17510_e12000_d_n4, assign17510_e12000_d_n5, assign17510_e12000_d_n6, assign17510_e12000_d_n7, assign17510_e12000_d_n8, assign17510_e12000_d_n9, assign17510_e12000_d_n10, assign17510_e12000_d_n11, assign17510_e12000_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard359 != 0.0)) && (locals.var_guard361 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depvmax, locals.var_uc_depvmax_dn0, locals.var_uc_depvmax_dn2, locals.var_uc_depvmax_dn4, locals.var_uc_depvmax_dn5, locals.var_uc_depvmax_dn6, locals.var_uc_depvmax_dn7, locals.var_uc_depvmax_dn8, locals.var_uc_depvmax_dn9, locals.var_uc_depvmax_dn10, locals.var_uc_depvmax_dn11, locals.var_uc_depvmax_dn14,)
    }
};
        locals.var_uc_depvmax = assign17510_e12000;
        locals.var_uc_depvmax_dn0 = assign17510_e12000_d_n0;
        locals.var_uc_depvmax_dn2 = assign17510_e12000_d_n2;
        locals.var_uc_depvmax_dn4 = assign17510_e12000_d_n4;
        locals.var_uc_depvmax_dn5 = assign17510_e12000_d_n5;
        locals.var_uc_depvmax_dn6 = assign17510_e12000_d_n6;
        locals.var_uc_depvmax_dn7 = assign17510_e12000_d_n7;
        locals.var_uc_depvmax_dn8 = assign17510_e12000_d_n8;
        locals.var_uc_depvmax_dn9 = assign17510_e12000_d_n9;
        locals.var_uc_depvmax_dn10 = assign17510_e12000_d_n10;
        locals.var_uc_depvmax_dn11 = assign17510_e12000_d_n11;
        locals.var_uc_depvmax_dn14 = assign17510_e12000_d_n14;

        let (assign17520_e12010, assign17520_e12010_d_n0, assign17520_e12010_d_n2, assign17520_e12010_d_n4, assign17520_e12010_d_n5, assign17520_e12010_d_n6, assign17520_e12010_d_n7, assign17520_e12010_d_n8, assign17520_e12010_d_n9, assign17520_e12010_d_n10, assign17520_e12010_d_n11, assign17520_e12010_d_n14,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard359 != 0.0)) {
        let assign17520_e12007: f64 = (locals.var_tratio).powf(p.p381);
        let assign17520_e12008: f64 = (locals.var_uc_depmue0 / assign17520_e12007);
        (assign17520_e12008, (((locals.var_uc_depmue0_dn0 * assign17520_e12007) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn0)) } } else { (assign17520_e12007 * (p.p381 * (locals.var_tratio_dn0 / locals.var_tratio))) })) / (assign17520_e12007 * assign17520_e12007)), (((locals.var_uc_depmue0_dn2 * assign17520_e12007) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn2)) } } else { (assign17520_e12007 * (p.p381 * (locals.var_tratio_dn2 / locals.var_tratio))) })) / (assign17520_e12007 * assign17520_e12007)), (((locals.var_uc_depmue0_dn4 * assign17520_e12007) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn4)) } } else { (assign17520_e12007 * (p.p381 * (locals.var_tratio_dn4 / locals.var_tratio))) })) / (assign17520_e12007 * assign17520_e12007)), (((locals.var_uc_depmue0_dn5 * assign17520_e12007) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn5)) } } else { (assign17520_e12007 * (p.p381 * (locals.var_tratio_dn5 / locals.var_tratio))) })) / (assign17520_e12007 * assign17520_e12007)), (((locals.var_uc_depmue0_dn6 * assign17520_e12007) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn6)) } } else { (assign17520_e12007 * (p.p381 * (locals.var_tratio_dn6 / locals.var_tratio))) })) / (assign17520_e12007 * assign17520_e12007)), (((locals.var_uc_depmue0_dn7 * assign17520_e12007) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn7)) } } else { (assign17520_e12007 * (p.p381 * (locals.var_tratio_dn7 / locals.var_tratio))) })) / (assign17520_e12007 * assign17520_e12007)), (((locals.var_uc_depmue0_dn8 * assign17520_e12007) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn8)) } } else { (assign17520_e12007 * (p.p381 * (locals.var_tratio_dn8 / locals.var_tratio))) })) / (assign17520_e12007 * assign17520_e12007)), (((locals.var_uc_depmue0_dn9 * assign17520_e12007) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn9)) } } else { (assign17520_e12007 * (p.p381 * (locals.var_tratio_dn9 / locals.var_tratio))) })) / (assign17520_e12007 * assign17520_e12007)), (((locals.var_uc_depmue0_dn10 * assign17520_e12007) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn10)) } } else { (assign17520_e12007 * (p.p381 * (locals.var_tratio_dn10 / locals.var_tratio))) })) / (assign17520_e12007 * assign17520_e12007)), (((locals.var_uc_depmue0_dn11 * assign17520_e12007) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn11)) } } else { (assign17520_e12007 * (p.p381 * (locals.var_tratio_dn11 / locals.var_tratio))) })) / (assign17520_e12007 * assign17520_e12007)), (((locals.var_uc_depmue0_dn14 * assign17520_e12007) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn14)) } } else { (assign17520_e12007 * (p.p381 * (locals.var_tratio_dn14 / locals.var_tratio))) })) / (assign17520_e12007 * assign17520_e12007)),)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn11, locals.var_uc_depmue0_dn14,)
    }
};
        locals.var_uc_depmue0 = assign17520_e12010;
        locals.var_uc_depmue0_dn0 = assign17520_e12010_d_n0;
        locals.var_uc_depmue0_dn2 = assign17520_e12010_d_n2;
        locals.var_uc_depmue0_dn4 = assign17520_e12010_d_n4;
        locals.var_uc_depmue0_dn5 = assign17520_e12010_d_n5;
        locals.var_uc_depmue0_dn6 = assign17520_e12010_d_n6;
        locals.var_uc_depmue0_dn7 = assign17520_e12010_d_n7;
        locals.var_uc_depmue0_dn8 = assign17520_e12010_d_n8;
        locals.var_uc_depmue0_dn9 = assign17520_e12010_d_n9;
        locals.var_uc_depmue0_dn10 = assign17520_e12010_d_n10;
        locals.var_uc_depmue0_dn11 = assign17520_e12010_d_n11;
        locals.var_uc_depmue0_dn14 = assign17520_e12010_d_n14;

        let (assign17530_e12020, assign17530_e12020_d_n0, assign17530_e12020_d_n2, assign17530_e12020_d_n4, assign17530_e12020_d_n5, assign17530_e12020_d_n6, assign17530_e12020_d_n7, assign17530_e12020_d_n8, assign17530_e12020_d_n9, assign17530_e12020_d_n10, assign17530_e12020_d_n11, assign17530_e12020_d_n14,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard359 != 0.0)) {
        let assign17530_e12017: f64 = (locals.var_tratio).powf(p.p382);
        let assign17530_e12018: f64 = (locals.var_uc_depmue2 / assign17530_e12017);
        (assign17530_e12018, (((locals.var_uc_depmue2_dn0 * assign17530_e12017) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn0)) } } else { (assign17530_e12017 * (p.p382 * (locals.var_tratio_dn0 / locals.var_tratio))) })) / (assign17530_e12017 * assign17530_e12017)), (((locals.var_uc_depmue2_dn2 * assign17530_e12017) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn2)) } } else { (assign17530_e12017 * (p.p382 * (locals.var_tratio_dn2 / locals.var_tratio))) })) / (assign17530_e12017 * assign17530_e12017)), (((locals.var_uc_depmue2_dn4 * assign17530_e12017) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn4)) } } else { (assign17530_e12017 * (p.p382 * (locals.var_tratio_dn4 / locals.var_tratio))) })) / (assign17530_e12017 * assign17530_e12017)), (((locals.var_uc_depmue2_dn5 * assign17530_e12017) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn5)) } } else { (assign17530_e12017 * (p.p382 * (locals.var_tratio_dn5 / locals.var_tratio))) })) / (assign17530_e12017 * assign17530_e12017)), (((locals.var_uc_depmue2_dn6 * assign17530_e12017) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn6)) } } else { (assign17530_e12017 * (p.p382 * (locals.var_tratio_dn6 / locals.var_tratio))) })) / (assign17530_e12017 * assign17530_e12017)), (((locals.var_uc_depmue2_dn7 * assign17530_e12017) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn7)) } } else { (assign17530_e12017 * (p.p382 * (locals.var_tratio_dn7 / locals.var_tratio))) })) / (assign17530_e12017 * assign17530_e12017)), (((locals.var_uc_depmue2_dn8 * assign17530_e12017) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn8)) } } else { (assign17530_e12017 * (p.p382 * (locals.var_tratio_dn8 / locals.var_tratio))) })) / (assign17530_e12017 * assign17530_e12017)), (((locals.var_uc_depmue2_dn9 * assign17530_e12017) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn9)) } } else { (assign17530_e12017 * (p.p382 * (locals.var_tratio_dn9 / locals.var_tratio))) })) / (assign17530_e12017 * assign17530_e12017)), (((locals.var_uc_depmue2_dn10 * assign17530_e12017) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn10)) } } else { (assign17530_e12017 * (p.p382 * (locals.var_tratio_dn10 / locals.var_tratio))) })) / (assign17530_e12017 * assign17530_e12017)), (((locals.var_uc_depmue2_dn11 * assign17530_e12017) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn11)) } } else { (assign17530_e12017 * (p.p382 * (locals.var_tratio_dn11 / locals.var_tratio))) })) / (assign17530_e12017 * assign17530_e12017)), (((locals.var_uc_depmue2_dn14 * assign17530_e12017) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn14)) } } else { (assign17530_e12017 * (p.p382 * (locals.var_tratio_dn14 / locals.var_tratio))) })) / (assign17530_e12017 * assign17530_e12017)),)
    } else {
        (locals.var_uc_depmue2, locals.var_uc_depmue2_dn0, locals.var_uc_depmue2_dn2, locals.var_uc_depmue2_dn4, locals.var_uc_depmue2_dn5, locals.var_uc_depmue2_dn6, locals.var_uc_depmue2_dn7, locals.var_uc_depmue2_dn8, locals.var_uc_depmue2_dn9, locals.var_uc_depmue2_dn10, locals.var_uc_depmue2_dn11, locals.var_uc_depmue2_dn14,)
    }
};
        locals.var_uc_depmue2 = assign17530_e12020;
        locals.var_uc_depmue2_dn0 = assign17530_e12020_d_n0;
        locals.var_uc_depmue2_dn2 = assign17530_e12020_d_n2;
        locals.var_uc_depmue2_dn4 = assign17530_e12020_d_n4;
        locals.var_uc_depmue2_dn5 = assign17530_e12020_d_n5;
        locals.var_uc_depmue2_dn6 = assign17530_e12020_d_n6;
        locals.var_uc_depmue2_dn7 = assign17530_e12020_d_n7;
        locals.var_uc_depmue2_dn8 = assign17530_e12020_d_n8;
        locals.var_uc_depmue2_dn9 = assign17530_e12020_d_n9;
        locals.var_uc_depmue2_dn10 = assign17530_e12020_d_n10;
        locals.var_uc_depmue2_dn11 = assign17530_e12020_d_n11;
        locals.var_uc_depmue2_dn14 = assign17530_e12020_d_n14;

        let assign17540_e12023: f64 = if locals.var_uc_codep == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard362 = assign17540_e12023;

        let (assign17550_e12041, assign17550_e12041_d_n0, assign17550_e12041_d_n2, assign17550_e12041_d_n4, assign17550_e12041_d_n5, assign17550_e12041_d_n6, assign17550_e12041_d_n7, assign17550_e12041_d_n8, assign17550_e12041_d_n9, assign17550_e12041_d_n10, assign17550_e12041_d_n11, assign17550_e12041_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard359 == 0.0)) && (locals.var_guard362 != 0.0)) {
        let assign17550_e12032: f64 = (2.0 * 1.034943e-10);
        let assign17550_e12034: f64 = (assign17550_e12032 * 1.6021918e-19);
        let assign17550_e12036: f64 = (assign17550_e12034 * locals.var_uc_ndepm);
        let assign17550_e12038: f64 = (assign17550_e12036 * locals.var_beta_inv);
        let assign17550_e12039: f64 = (assign17550_e12038).sqrt();
        (assign17550_e12039, ((((assign17550_e12034 * locals.var_uc_ndepm_dn0) * locals.var_beta_inv) + (assign17550_e12036 * locals.var_beta_inv_dn0)) / (2.0 * assign17550_e12039)), ((((assign17550_e12034 * locals.var_uc_ndepm_dn2) * locals.var_beta_inv) + (assign17550_e12036 * locals.var_beta_inv_dn2)) / (2.0 * assign17550_e12039)), ((((assign17550_e12034 * locals.var_uc_ndepm_dn4) * locals.var_beta_inv) + (assign17550_e12036 * locals.var_beta_inv_dn4)) / (2.0 * assign17550_e12039)), ((((assign17550_e12034 * locals.var_uc_ndepm_dn5) * locals.var_beta_inv) + (assign17550_e12036 * locals.var_beta_inv_dn5)) / (2.0 * assign17550_e12039)), ((((assign17550_e12034 * locals.var_uc_ndepm_dn6) * locals.var_beta_inv) + (assign17550_e12036 * locals.var_beta_inv_dn6)) / (2.0 * assign17550_e12039)), ((((assign17550_e12034 * locals.var_uc_ndepm_dn7) * locals.var_beta_inv) + (assign17550_e12036 * locals.var_beta_inv_dn7)) / (2.0 * assign17550_e12039)), ((((assign17550_e12034 * locals.var_uc_ndepm_dn8) * locals.var_beta_inv) + (assign17550_e12036 * locals.var_beta_inv_dn8)) / (2.0 * assign17550_e12039)), ((((assign17550_e12034 * locals.var_uc_ndepm_dn9) * locals.var_beta_inv) + (assign17550_e12036 * locals.var_beta_inv_dn9)) / (2.0 * assign17550_e12039)), ((((assign17550_e12034 * locals.var_uc_ndepm_dn10) * locals.var_beta_inv) + (assign17550_e12036 * locals.var_beta_inv_dn10)) / (2.0 * assign17550_e12039)), ((((assign17550_e12034 * locals.var_uc_ndepm_dn11) * locals.var_beta_inv) + (assign17550_e12036 * locals.var_beta_inv_dn11)) / (2.0 * assign17550_e12039)), ((((assign17550_e12034 * locals.var_uc_ndepm_dn14) * locals.var_beta_inv) + (assign17550_e12036 * locals.var_beta_inv_dn14)) / (2.0 * assign17550_e12039)),)
    } else {
        (locals.var_cnst0, locals.var_cnst0_dn0, locals.var_cnst0_dn2, locals.var_cnst0_dn4, locals.var_cnst0_dn5, locals.var_cnst0_dn6, locals.var_cnst0_dn7, locals.var_cnst0_dn8, locals.var_cnst0_dn9, locals.var_cnst0_dn10, locals.var_cnst0_dn11, locals.var_cnst0_dn14,)
    }
};
        locals.var_cnst0 = assign17550_e12041;
        locals.var_cnst0_dn0 = assign17550_e12041_d_n0;
        locals.var_cnst0_dn2 = assign17550_e12041_d_n2;
        locals.var_cnst0_dn4 = assign17550_e12041_d_n4;
        locals.var_cnst0_dn5 = assign17550_e12041_d_n5;
        locals.var_cnst0_dn6 = assign17550_e12041_d_n6;
        locals.var_cnst0_dn7 = assign17550_e12041_d_n7;
        locals.var_cnst0_dn8 = assign17550_e12041_d_n8;
        locals.var_cnst0_dn9 = assign17550_e12041_d_n9;
        locals.var_cnst0_dn10 = assign17550_e12041_d_n10;
        locals.var_cnst0_dn11 = assign17550_e12041_d_n11;
        locals.var_cnst0_dn14 = assign17550_e12041_d_n14;

        let (assign17560_e12056, assign17560_e12056_d_n0, assign17560_e12056_d_n2, assign17560_e12056_d_n4, assign17560_e12056_d_n5, assign17560_e12056_d_n6, assign17560_e12056_d_n7, assign17560_e12056_d_n8, assign17560_e12056_d_n9, assign17560_e12056_d_n10, assign17560_e12056_d_n11, assign17560_e12056_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard359 == 0.0)) && (locals.var_guard362 != 0.0)) {
        let assign17560_e12050: f64 = (locals.var_nin * locals.var_nin);
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_uc_ndepm;
        let assign17560_e12052: f64 = (assign17560_e12050 * __rspice_inv_cse_0);
        let assign17560_e12054: f64 = (assign17560_e12052 * __rspice_inv_cse_0);
        (assign17560_e12054, ((((((((locals.var_nin_dn0 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn0)) * locals.var_uc_ndepm) - (assign17560_e12050 * locals.var_uc_ndepm_dn0)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17560_e12052 * locals.var_uc_ndepm_dn0)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn2 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn2)) * locals.var_uc_ndepm) - (assign17560_e12050 * locals.var_uc_ndepm_dn2)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17560_e12052 * locals.var_uc_ndepm_dn2)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn4 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn4)) * locals.var_uc_ndepm) - (assign17560_e12050 * locals.var_uc_ndepm_dn4)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17560_e12052 * locals.var_uc_ndepm_dn4)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn5 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn5)) * locals.var_uc_ndepm) - (assign17560_e12050 * locals.var_uc_ndepm_dn5)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17560_e12052 * locals.var_uc_ndepm_dn5)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn6 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn6)) * locals.var_uc_ndepm) - (assign17560_e12050 * locals.var_uc_ndepm_dn6)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17560_e12052 * locals.var_uc_ndepm_dn6)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn7 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn7)) * locals.var_uc_ndepm) - (assign17560_e12050 * locals.var_uc_ndepm_dn7)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17560_e12052 * locals.var_uc_ndepm_dn7)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn8 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn8)) * locals.var_uc_ndepm) - (assign17560_e12050 * locals.var_uc_ndepm_dn8)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17560_e12052 * locals.var_uc_ndepm_dn8)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn9 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn9)) * locals.var_uc_ndepm) - (assign17560_e12050 * locals.var_uc_ndepm_dn9)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17560_e12052 * locals.var_uc_ndepm_dn9)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn10 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn10)) * locals.var_uc_ndepm) - (assign17560_e12050 * locals.var_uc_ndepm_dn10)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17560_e12052 * locals.var_uc_ndepm_dn10)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn11 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn11)) * locals.var_uc_ndepm) - (assign17560_e12050 * locals.var_uc_ndepm_dn11)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17560_e12052 * locals.var_uc_ndepm_dn11)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn14 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn14)) * locals.var_uc_ndepm) - (assign17560_e12050 * locals.var_uc_ndepm_dn14)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17560_e12052 * locals.var_uc_ndepm_dn14)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)),)
    } else {
        (locals.var_cnst1, locals.var_cnst1_dn0, locals.var_cnst1_dn2, locals.var_cnst1_dn4, locals.var_cnst1_dn5, locals.var_cnst1_dn6, locals.var_cnst1_dn7, locals.var_cnst1_dn8, locals.var_cnst1_dn9, locals.var_cnst1_dn10, locals.var_cnst1_dn11, locals.var_cnst1_dn14,)
    }
};
        locals.var_cnst1 = assign17560_e12056;
        locals.var_cnst1_dn0 = assign17560_e12056_d_n0;
        locals.var_cnst1_dn2 = assign17560_e12056_d_n2;
        locals.var_cnst1_dn4 = assign17560_e12056_d_n4;
        locals.var_cnst1_dn5 = assign17560_e12056_d_n5;
        locals.var_cnst1_dn6 = assign17560_e12056_d_n6;
        locals.var_cnst1_dn7 = assign17560_e12056_d_n7;
        locals.var_cnst1_dn8 = assign17560_e12056_d_n8;
        locals.var_cnst1_dn9 = assign17560_e12056_d_n9;
        locals.var_cnst1_dn10 = assign17560_e12056_d_n10;
        locals.var_cnst1_dn11 = assign17560_e12056_d_n11;
        locals.var_cnst1_dn14 = assign17560_e12056_d_n14;

        let (assign17570_e12072, assign17570_e12072_d_n0, assign17570_e12072_d_n2, assign17570_e12072_d_n4, assign17570_e12072_d_n5, assign17570_e12072_d_n6, assign17570_e12072_d_n7, assign17570_e12072_d_n8, assign17570_e12072_d_n9, assign17570_e12072_d_n10, assign17570_e12072_d_n11, assign17570_e12072_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard359 == 0.0)) && (locals.var_guard362 != 0.0)) {
        let assign17570_e12065: f64 = (2.0 * locals.var_beta_inv);
        let assign17570_e12068: f64 = (locals.var_uc_ndepm / locals.var_nin);
        let assign17570_e12069: f64 = (assign17570_e12068).ln();
        let assign17570_e12070: f64 = (assign17570_e12065 * assign17570_e12069);
        (assign17570_e12070, (((2.0 * locals.var_beta_inv_dn0) * assign17570_e12069) + (assign17570_e12065 * ((((locals.var_uc_ndepm_dn0 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign17570_e12068))), (((2.0 * locals.var_beta_inv_dn2) * assign17570_e12069) + (assign17570_e12065 * ((((locals.var_uc_ndepm_dn2 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign17570_e12068))), (((2.0 * locals.var_beta_inv_dn4) * assign17570_e12069) + (assign17570_e12065 * ((((locals.var_uc_ndepm_dn4 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign17570_e12068))), (((2.0 * locals.var_beta_inv_dn5) * assign17570_e12069) + (assign17570_e12065 * ((((locals.var_uc_ndepm_dn5 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign17570_e12068))), (((2.0 * locals.var_beta_inv_dn6) * assign17570_e12069) + (assign17570_e12065 * ((((locals.var_uc_ndepm_dn6 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign17570_e12068))), (((2.0 * locals.var_beta_inv_dn7) * assign17570_e12069) + (assign17570_e12065 * ((((locals.var_uc_ndepm_dn7 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign17570_e12068))), (((2.0 * locals.var_beta_inv_dn8) * assign17570_e12069) + (assign17570_e12065 * ((((locals.var_uc_ndepm_dn8 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign17570_e12068))), (((2.0 * locals.var_beta_inv_dn9) * assign17570_e12069) + (assign17570_e12065 * ((((locals.var_uc_ndepm_dn9 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign17570_e12068))), (((2.0 * locals.var_beta_inv_dn10) * assign17570_e12069) + (assign17570_e12065 * ((((locals.var_uc_ndepm_dn10 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign17570_e12068))), (((2.0 * locals.var_beta_inv_dn11) * assign17570_e12069) + (assign17570_e12065 * ((((locals.var_uc_ndepm_dn11 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)) / assign17570_e12068))), (((2.0 * locals.var_beta_inv_dn14) * assign17570_e12069) + (assign17570_e12065 * ((((locals.var_uc_ndepm_dn14 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn14)) / (locals.var_nin * locals.var_nin)) / assign17570_e12068))),)
    } else {
        (locals.var_pb2n, locals.var_pb2n_dn0, locals.var_pb2n_dn2, locals.var_pb2n_dn4, locals.var_pb2n_dn5, locals.var_pb2n_dn6, locals.var_pb2n_dn7, locals.var_pb2n_dn8, locals.var_pb2n_dn9, locals.var_pb2n_dn10, locals.var_pb2n_dn11, locals.var_pb2n_dn14,)
    }
};
        locals.var_pb2n = assign17570_e12072;
        locals.var_pb2n_dn0 = assign17570_e12072_d_n0;
        locals.var_pb2n_dn2 = assign17570_e12072_d_n2;
        locals.var_pb2n_dn4 = assign17570_e12072_d_n4;
        locals.var_pb2n_dn5 = assign17570_e12072_d_n5;
        locals.var_pb2n_dn6 = assign17570_e12072_d_n6;
        locals.var_pb2n_dn7 = assign17570_e12072_d_n7;
        locals.var_pb2n_dn8 = assign17570_e12072_d_n8;
        locals.var_pb2n_dn9 = assign17570_e12072_d_n9;
        locals.var_pb2n_dn10 = assign17570_e12072_d_n10;
        locals.var_pb2n_dn11 = assign17570_e12072_d_n11;
        locals.var_pb2n_dn14 = assign17570_e12072_d_n14;

        let (assign17580_e12090, assign17580_e12090_d_n0, assign17580_e12090_d_n2, assign17580_e12090_d_n4, assign17580_e12090_d_n5, assign17580_e12090_d_n6, assign17580_e12090_d_n7, assign17580_e12090_d_n8, assign17580_e12090_d_n9, assign17580_e12090_d_n10, assign17580_e12090_d_n11, assign17580_e12090_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard359 == 0.0)) && (locals.var_guard362 != 0.0)) {
        let assign17580_e12082: f64 = (locals.var_uc_ndepm * locals.var_ef_nsubc);
        let __rspice_inv_cse_1: f64 = 1.0 / locals.var_nin;
        let assign17580_e12084: f64 = (assign17580_e12082 * __rspice_inv_cse_1);
        let assign17580_e12086: f64 = (assign17580_e12084 * __rspice_inv_cse_1);
        let assign17580_e12087: f64 = (assign17580_e12086).ln();
        let assign17580_e12088: f64 = (locals.var_beta_inv * assign17580_e12087);
        (assign17580_e12088, ((locals.var_beta_inv_dn0 * assign17580_e12087) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn0 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn0)) * locals.var_nin) - (assign17580_e12082 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17580_e12084 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign17580_e12086))), ((locals.var_beta_inv_dn2 * assign17580_e12087) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn2 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn2)) * locals.var_nin) - (assign17580_e12082 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17580_e12084 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign17580_e12086))), ((locals.var_beta_inv_dn4 * assign17580_e12087) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn4 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn4)) * locals.var_nin) - (assign17580_e12082 * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17580_e12084 * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign17580_e12086))), ((locals.var_beta_inv_dn5 * assign17580_e12087) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn5 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn5)) * locals.var_nin) - (assign17580_e12082 * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17580_e12084 * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign17580_e12086))), ((locals.var_beta_inv_dn6 * assign17580_e12087) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn6 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn6)) * locals.var_nin) - (assign17580_e12082 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17580_e12084 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign17580_e12086))), ((locals.var_beta_inv_dn7 * assign17580_e12087) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn7 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn7)) * locals.var_nin) - (assign17580_e12082 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17580_e12084 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign17580_e12086))), ((locals.var_beta_inv_dn8 * assign17580_e12087) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn8 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn8)) * locals.var_nin) - (assign17580_e12082 * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17580_e12084 * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign17580_e12086))), ((locals.var_beta_inv_dn9 * assign17580_e12087) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn9 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn9)) * locals.var_nin) - (assign17580_e12082 * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17580_e12084 * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign17580_e12086))), ((locals.var_beta_inv_dn10 * assign17580_e12087) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn10 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn10)) * locals.var_nin) - (assign17580_e12082 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17580_e12084 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign17580_e12086))), ((locals.var_beta_inv_dn11 * assign17580_e12087) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn11 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn11)) * locals.var_nin) - (assign17580_e12082 * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17580_e12084 * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)) / assign17580_e12086))), ((locals.var_beta_inv_dn14 * assign17580_e12087) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn14 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn14)) * locals.var_nin) - (assign17580_e12082 * locals.var_nin_dn14)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17580_e12084 * locals.var_nin_dn14)) / (locals.var_nin * locals.var_nin)) / assign17580_e12086))),)
    } else {
        (locals.var_vbipn, locals.var_vbipn_dn0, locals.var_vbipn_dn2, locals.var_vbipn_dn4, locals.var_vbipn_dn5, locals.var_vbipn_dn6, locals.var_vbipn_dn7, locals.var_vbipn_dn8, locals.var_vbipn_dn9, locals.var_vbipn_dn10, locals.var_vbipn_dn11, locals.var_vbipn_dn14,)
    }
};
        locals.var_vbipn = assign17580_e12090;
        locals.var_vbipn_dn0 = assign17580_e12090_d_n0;
        locals.var_vbipn_dn2 = assign17580_e12090_d_n2;
        locals.var_vbipn_dn4 = assign17580_e12090_d_n4;
        locals.var_vbipn_dn5 = assign17580_e12090_d_n5;
        locals.var_vbipn_dn6 = assign17580_e12090_d_n6;
        locals.var_vbipn_dn7 = assign17580_e12090_d_n7;
        locals.var_vbipn_dn8 = assign17580_e12090_d_n8;
        locals.var_vbipn_dn9 = assign17580_e12090_d_n9;
        locals.var_vbipn_dn10 = assign17580_e12090_d_n10;
        locals.var_vbipn_dn11 = assign17580_e12090_d_n11;
        locals.var_vbipn_dn14 = assign17580_e12090_d_n14;

        let (assign17590_e12102, assign17590_e12102_d_n0, assign17590_e12102_d_n2, assign17590_e12102_d_n4, assign17590_e12102_d_n5, assign17590_e12102_d_n6, assign17590_e12102_d_n7, assign17590_e12102_d_n8, assign17590_e12102_d_n9, assign17590_e12102_d_n10, assign17590_e12102_d_n11, assign17590_e12102_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard359 == 0.0)) && (locals.var_guard362 != 0.0)) {
        let assign17590_e12099: f64 = (locals.var_log_tratio * p.p380);
        let assign17590_e12100: f64 = (assign17590_e12099).exp();
        (assign17590_e12100, (assign17590_e12100 * (locals.var_log_tratio_dn0 * p.p380)), (assign17590_e12100 * (locals.var_log_tratio_dn2 * p.p380)), (assign17590_e12100 * (locals.var_log_tratio_dn4 * p.p380)), (assign17590_e12100 * (locals.var_log_tratio_dn5 * p.p380)), (assign17590_e12100 * (locals.var_log_tratio_dn6 * p.p380)), (assign17590_e12100 * (locals.var_log_tratio_dn7 * p.p380)), (assign17590_e12100 * (locals.var_log_tratio_dn8 * p.p380)), (assign17590_e12100 * (locals.var_log_tratio_dn9 * p.p380)), (assign17590_e12100 * (locals.var_log_tratio_dn10 * p.p380)), (assign17590_e12100 * (locals.var_log_tratio_dn11 * p.p380)), (assign17590_e12100 * (locals.var_log_tratio_dn14 * p.p380)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign17590_e12102;
        locals.var_t1_dn0 = assign17590_e12102_d_n0;
        locals.var_t1_dn2 = assign17590_e12102_d_n2;
        locals.var_t1_dn4 = assign17590_e12102_d_n4;
        locals.var_t1_dn5 = assign17590_e12102_d_n5;
        locals.var_t1_dn6 = assign17590_e12102_d_n6;
        locals.var_t1_dn7 = assign17590_e12102_d_n7;
        locals.var_t1_dn8 = assign17590_e12102_d_n8;
        locals.var_t1_dn9 = assign17590_e12102_d_n9;
        locals.var_t1_dn10 = assign17590_e12102_d_n10;
        locals.var_t1_dn11 = assign17590_e12102_d_n11;
        locals.var_t1_dn14 = assign17590_e12102_d_n14;

        let (assign17600_e12113, assign17600_e12113_d_n0, assign17600_e12113_d_n2, assign17600_e12113_d_n4, assign17600_e12113_d_n5, assign17600_e12113_d_n6, assign17600_e12113_d_n7, assign17600_e12113_d_n8, assign17600_e12113_d_n9, assign17600_e12113_d_n10, assign17600_e12113_d_n11, assign17600_e12113_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard359 == 0.0)) && (locals.var_guard362 != 0.0)) {
        let assign17600_e12111: f64 = (locals.var_t1 / locals.var_uc_depmueph1);
        (assign17600_e12111, (locals.var_t1_dn0 / locals.var_uc_depmueph1), (locals.var_t1_dn2 / locals.var_uc_depmueph1), (locals.var_t1_dn4 / locals.var_uc_depmueph1), (locals.var_t1_dn5 / locals.var_uc_depmueph1), (locals.var_t1_dn6 / locals.var_uc_depmueph1), (locals.var_t1_dn7 / locals.var_uc_depmueph1), (locals.var_t1_dn8 / locals.var_uc_depmueph1), (locals.var_t1_dn9 / locals.var_uc_depmueph1), (locals.var_t1_dn10 / locals.var_uc_depmueph1), (locals.var_t1_dn11 / locals.var_uc_depmueph1), (locals.var_t1_dn14 / locals.var_uc_depmueph1),)
    } else {
        (locals.var_depmphn0, locals.var_depmphn0_dn0, locals.var_depmphn0_dn2, locals.var_depmphn0_dn4, locals.var_depmphn0_dn5, locals.var_depmphn0_dn6, locals.var_depmphn0_dn7, locals.var_depmphn0_dn8, locals.var_depmphn0_dn9, locals.var_depmphn0_dn10, locals.var_depmphn0_dn11, locals.var_depmphn0_dn14,)
    }
};
        locals.var_depmphn0 = assign17600_e12113;
        locals.var_depmphn0_dn0 = assign17600_e12113_d_n0;
        locals.var_depmphn0_dn2 = assign17600_e12113_d_n2;
        locals.var_depmphn0_dn4 = assign17600_e12113_d_n4;
        locals.var_depmphn0_dn5 = assign17600_e12113_d_n5;
        locals.var_depmphn0_dn6 = assign17600_e12113_d_n6;
        locals.var_depmphn0_dn7 = assign17600_e12113_d_n7;
        locals.var_depmphn0_dn8 = assign17600_e12113_d_n8;
        locals.var_depmphn0_dn9 = assign17600_e12113_d_n9;
        locals.var_depmphn0_dn10 = assign17600_e12113_d_n10;
        locals.var_depmphn0_dn11 = assign17600_e12113_d_n11;
        locals.var_depmphn0_dn14 = assign17600_e12113_d_n14;

        let (assign17610_e12138, assign17610_e12138_d_n0, assign17610_e12138_d_n2, assign17610_e12138_d_n4, assign17610_e12138_d_n5, assign17610_e12138_d_n6, assign17610_e12138_d_n7, assign17610_e12138_d_n8, assign17610_e12138_d_n9, assign17610_e12138_d_n10, assign17610_e12138_d_n11, assign17610_e12138_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard359 == 0.0)) && (locals.var_guard362 != 0.0)) {
        let assign17610_e12123: f64 = (0.4 * locals.var_tratio);
        let assign17610_e12124: f64 = (1.8 + assign17610_e12123);
        let assign17610_e12127: f64 = (0.1 * locals.var_tratio);
        let assign17610_e12129: f64 = (assign17610_e12127 * locals.var_tratio);
        let assign17610_e12130: f64 = (assign17610_e12124 + assign17610_e12129);
        let assign17610_e12134: f64 = (1.0 - locals.var_tratio);
        let assign17610_e12135: f64 = (p.p379 * assign17610_e12134);
        let assign17610_e12136: f64 = (assign17610_e12130 - assign17610_e12135);
        (assign17610_e12136, (((0.4 * locals.var_tratio_dn0) + (((0.1 * locals.var_tratio_dn0) * locals.var_tratio) + (assign17610_e12127 * locals.var_tratio_dn0))) - (p.p379 * (-locals.var_tratio_dn0))), (((0.4 * locals.var_tratio_dn2) + (((0.1 * locals.var_tratio_dn2) * locals.var_tratio) + (assign17610_e12127 * locals.var_tratio_dn2))) - (p.p379 * (-locals.var_tratio_dn2))), (((0.4 * locals.var_tratio_dn4) + (((0.1 * locals.var_tratio_dn4) * locals.var_tratio) + (assign17610_e12127 * locals.var_tratio_dn4))) - (p.p379 * (-locals.var_tratio_dn4))), (((0.4 * locals.var_tratio_dn5) + (((0.1 * locals.var_tratio_dn5) * locals.var_tratio) + (assign17610_e12127 * locals.var_tratio_dn5))) - (p.p379 * (-locals.var_tratio_dn5))), (((0.4 * locals.var_tratio_dn6) + (((0.1 * locals.var_tratio_dn6) * locals.var_tratio) + (assign17610_e12127 * locals.var_tratio_dn6))) - (p.p379 * (-locals.var_tratio_dn6))), (((0.4 * locals.var_tratio_dn7) + (((0.1 * locals.var_tratio_dn7) * locals.var_tratio) + (assign17610_e12127 * locals.var_tratio_dn7))) - (p.p379 * (-locals.var_tratio_dn7))), (((0.4 * locals.var_tratio_dn8) + (((0.1 * locals.var_tratio_dn8) * locals.var_tratio) + (assign17610_e12127 * locals.var_tratio_dn8))) - (p.p379 * (-locals.var_tratio_dn8))), (((0.4 * locals.var_tratio_dn9) + (((0.1 * locals.var_tratio_dn9) * locals.var_tratio) + (assign17610_e12127 * locals.var_tratio_dn9))) - (p.p379 * (-locals.var_tratio_dn9))), (((0.4 * locals.var_tratio_dn10) + (((0.1 * locals.var_tratio_dn10) * locals.var_tratio) + (assign17610_e12127 * locals.var_tratio_dn10))) - (p.p379 * (-locals.var_tratio_dn10))), (((0.4 * locals.var_tratio_dn11) + (((0.1 * locals.var_tratio_dn11) * locals.var_tratio) + (assign17610_e12127 * locals.var_tratio_dn11))) - (p.p379 * (-locals.var_tratio_dn11))), (((0.4 * locals.var_tratio_dn14) + (((0.1 * locals.var_tratio_dn14) * locals.var_tratio) + (assign17610_e12127 * locals.var_tratio_dn14))) - (p.p379 * (-locals.var_tratio_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign17610_e12138;
        locals.var_t0_dn0 = assign17610_e12138_d_n0;
        locals.var_t0_dn2 = assign17610_e12138_d_n2;
        locals.var_t0_dn4 = assign17610_e12138_d_n4;
        locals.var_t0_dn5 = assign17610_e12138_d_n5;
        locals.var_t0_dn6 = assign17610_e12138_d_n6;
        locals.var_t0_dn7 = assign17610_e12138_d_n7;
        locals.var_t0_dn8 = assign17610_e12138_d_n8;
        locals.var_t0_dn9 = assign17610_e12138_d_n9;
        locals.var_t0_dn10 = assign17610_e12138_d_n10;
        locals.var_t0_dn11 = assign17610_e12138_d_n11;
        locals.var_t0_dn14 = assign17610_e12138_d_n14;

        let (assign17620_e12149, assign17620_e12149_d_n0, assign17620_e12149_d_n2, assign17620_e12149_d_n4, assign17620_e12149_d_n5, assign17620_e12149_d_n6, assign17620_e12149_d_n7, assign17620_e12149_d_n8, assign17620_e12149_d_n9, assign17620_e12149_d_n10, assign17620_e12149_d_n11, assign17620_e12149_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard359 == 0.0)) && (locals.var_guard362 != 0.0)) {
        let assign17620_e12147: f64 = (locals.var_uc_depvmax / locals.var_t0);
        (assign17620_e12147, (((locals.var_uc_depvmax_dn0 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn0)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn2 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn2)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn4 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn5 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn6 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn7 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn8 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn9 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn10 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn11 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn14 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn14)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_uc_depvmax, locals.var_uc_depvmax_dn0, locals.var_uc_depvmax_dn2, locals.var_uc_depvmax_dn4, locals.var_uc_depvmax_dn5, locals.var_uc_depvmax_dn6, locals.var_uc_depvmax_dn7, locals.var_uc_depvmax_dn8, locals.var_uc_depvmax_dn9, locals.var_uc_depvmax_dn10, locals.var_uc_depvmax_dn11, locals.var_uc_depvmax_dn14,)
    }
};
        locals.var_uc_depvmax = assign17620_e12149;
        locals.var_uc_depvmax_dn0 = assign17620_e12149_d_n0;
        locals.var_uc_depvmax_dn2 = assign17620_e12149_d_n2;
        locals.var_uc_depvmax_dn4 = assign17620_e12149_d_n4;
        locals.var_uc_depvmax_dn5 = assign17620_e12149_d_n5;
        locals.var_uc_depvmax_dn6 = assign17620_e12149_d_n6;
        locals.var_uc_depvmax_dn7 = assign17620_e12149_d_n7;
        locals.var_uc_depvmax_dn8 = assign17620_e12149_d_n8;
        locals.var_uc_depvmax_dn9 = assign17620_e12149_d_n9;
        locals.var_uc_depvmax_dn10 = assign17620_e12149_d_n10;
        locals.var_uc_depvmax_dn11 = assign17620_e12149_d_n11;
        locals.var_uc_depvmax_dn14 = assign17620_e12149_d_n14;

        let assign17640_e12157: f64 = if locals.var_uc_depvmax < 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard364 = assign17640_e12157;

        let (assign17650_e12168, assign17650_e12168_d_n0, assign17650_e12168_d_n2, assign17650_e12168_d_n4, assign17650_e12168_d_n5, assign17650_e12168_d_n6, assign17650_e12168_d_n7, assign17650_e12168_d_n8, assign17650_e12168_d_n9, assign17650_e12168_d_n10, assign17650_e12168_d_n11, assign17650_e12168_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard359 == 0.0)) && (locals.var_guard362 != 0.0)) && (locals.var_guard364 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depvmax, locals.var_uc_depvmax_dn0, locals.var_uc_depvmax_dn2, locals.var_uc_depvmax_dn4, locals.var_uc_depvmax_dn5, locals.var_uc_depvmax_dn6, locals.var_uc_depvmax_dn7, locals.var_uc_depvmax_dn8, locals.var_uc_depvmax_dn9, locals.var_uc_depvmax_dn10, locals.var_uc_depvmax_dn11, locals.var_uc_depvmax_dn14,)
    }
};
        locals.var_uc_depvmax = assign17650_e12168;
        locals.var_uc_depvmax_dn0 = assign17650_e12168_d_n0;
        locals.var_uc_depvmax_dn2 = assign17650_e12168_d_n2;
        locals.var_uc_depvmax_dn4 = assign17650_e12168_d_n4;
        locals.var_uc_depvmax_dn5 = assign17650_e12168_d_n5;
        locals.var_uc_depvmax_dn6 = assign17650_e12168_d_n6;
        locals.var_uc_depvmax_dn7 = assign17650_e12168_d_n7;
        locals.var_uc_depvmax_dn8 = assign17650_e12168_d_n8;
        locals.var_uc_depvmax_dn9 = assign17650_e12168_d_n9;
        locals.var_uc_depvmax_dn10 = assign17650_e12168_d_n10;
        locals.var_uc_depvmax_dn11 = assign17650_e12168_d_n11;
        locals.var_uc_depvmax_dn14 = assign17650_e12168_d_n14;

        let (assign17660_e12181, assign17660_e12181_d_n0, assign17660_e12181_d_n2, assign17660_e12181_d_n4, assign17660_e12181_d_n5, assign17660_e12181_d_n6, assign17660_e12181_d_n7, assign17660_e12181_d_n8, assign17660_e12181_d_n9, assign17660_e12181_d_n10, assign17660_e12181_d_n11, assign17660_e12181_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard359 == 0.0)) && (locals.var_guard362 != 0.0)) {
        let assign17660_e12178: f64 = (locals.var_tratio).powf(p.p381);
        let assign17660_e12179: f64 = (locals.var_uc_depmue0 / assign17660_e12178);
        (assign17660_e12179, (((locals.var_uc_depmue0_dn0 * assign17660_e12178) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn0)) } } else { (assign17660_e12178 * (p.p381 * (locals.var_tratio_dn0 / locals.var_tratio))) })) / (assign17660_e12178 * assign17660_e12178)), (((locals.var_uc_depmue0_dn2 * assign17660_e12178) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn2)) } } else { (assign17660_e12178 * (p.p381 * (locals.var_tratio_dn2 / locals.var_tratio))) })) / (assign17660_e12178 * assign17660_e12178)), (((locals.var_uc_depmue0_dn4 * assign17660_e12178) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn4)) } } else { (assign17660_e12178 * (p.p381 * (locals.var_tratio_dn4 / locals.var_tratio))) })) / (assign17660_e12178 * assign17660_e12178)), (((locals.var_uc_depmue0_dn5 * assign17660_e12178) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn5)) } } else { (assign17660_e12178 * (p.p381 * (locals.var_tratio_dn5 / locals.var_tratio))) })) / (assign17660_e12178 * assign17660_e12178)), (((locals.var_uc_depmue0_dn6 * assign17660_e12178) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn6)) } } else { (assign17660_e12178 * (p.p381 * (locals.var_tratio_dn6 / locals.var_tratio))) })) / (assign17660_e12178 * assign17660_e12178)), (((locals.var_uc_depmue0_dn7 * assign17660_e12178) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn7)) } } else { (assign17660_e12178 * (p.p381 * (locals.var_tratio_dn7 / locals.var_tratio))) })) / (assign17660_e12178 * assign17660_e12178)), (((locals.var_uc_depmue0_dn8 * assign17660_e12178) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn8)) } } else { (assign17660_e12178 * (p.p381 * (locals.var_tratio_dn8 / locals.var_tratio))) })) / (assign17660_e12178 * assign17660_e12178)), (((locals.var_uc_depmue0_dn9 * assign17660_e12178) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn9)) } } else { (assign17660_e12178 * (p.p381 * (locals.var_tratio_dn9 / locals.var_tratio))) })) / (assign17660_e12178 * assign17660_e12178)), (((locals.var_uc_depmue0_dn10 * assign17660_e12178) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn10)) } } else { (assign17660_e12178 * (p.p381 * (locals.var_tratio_dn10 / locals.var_tratio))) })) / (assign17660_e12178 * assign17660_e12178)), (((locals.var_uc_depmue0_dn11 * assign17660_e12178) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn11)) } } else { (assign17660_e12178 * (p.p381 * (locals.var_tratio_dn11 / locals.var_tratio))) })) / (assign17660_e12178 * assign17660_e12178)), (((locals.var_uc_depmue0_dn14 * assign17660_e12178) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn14)) } } else { (assign17660_e12178 * (p.p381 * (locals.var_tratio_dn14 / locals.var_tratio))) })) / (assign17660_e12178 * assign17660_e12178)),)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn11, locals.var_uc_depmue0_dn14,)
    }
};
        locals.var_uc_depmue0 = assign17660_e12181;
        locals.var_uc_depmue0_dn0 = assign17660_e12181_d_n0;
        locals.var_uc_depmue0_dn2 = assign17660_e12181_d_n2;
        locals.var_uc_depmue0_dn4 = assign17660_e12181_d_n4;
        locals.var_uc_depmue0_dn5 = assign17660_e12181_d_n5;
        locals.var_uc_depmue0_dn6 = assign17660_e12181_d_n6;
        locals.var_uc_depmue0_dn7 = assign17660_e12181_d_n7;
        locals.var_uc_depmue0_dn8 = assign17660_e12181_d_n8;
        locals.var_uc_depmue0_dn9 = assign17660_e12181_d_n9;
        locals.var_uc_depmue0_dn10 = assign17660_e12181_d_n10;
        locals.var_uc_depmue0_dn11 = assign17660_e12181_d_n11;
        locals.var_uc_depmue0_dn14 = assign17660_e12181_d_n14;

        let (assign17670_e12196, assign17670_e12196_d_n0, assign17670_e12196_d_n2, assign17670_e12196_d_n4, assign17670_e12196_d_n5, assign17670_e12196_d_n6, assign17670_e12196_d_n7, assign17670_e12196_d_n8, assign17670_e12196_d_n9, assign17670_e12196_d_n10, assign17670_e12196_d_n11, assign17670_e12196_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard359 == 0.0)) && (locals.var_guard362 != 0.0)) {
        let assign17670_e12192: f64 = (locals.var_tratio - 1.0);
        let assign17670_e12193: f64 = (p.p365 * assign17670_e12192);
        let assign17670_e12194: f64 = (p.p364 + assign17670_e12193);
        (assign17670_e12194, (p.p365 * locals.var_tratio_dn0), (p.p365 * locals.var_tratio_dn2), (p.p365 * locals.var_tratio_dn4), (p.p365 * locals.var_tratio_dn5), (p.p365 * locals.var_tratio_dn6), (p.p365 * locals.var_tratio_dn7), (p.p365 * locals.var_tratio_dn8), (p.p365 * locals.var_tratio_dn9), (p.p365 * locals.var_tratio_dn10), (p.p365 * locals.var_tratio_dn11), (p.p365 * locals.var_tratio_dn14),)
    } else {
        (locals.var_uc_depwlp, locals.var_uc_depwlp_dn0, locals.var_uc_depwlp_dn2, locals.var_uc_depwlp_dn4, locals.var_uc_depwlp_dn5, locals.var_uc_depwlp_dn6, locals.var_uc_depwlp_dn7, locals.var_uc_depwlp_dn8, locals.var_uc_depwlp_dn9, locals.var_uc_depwlp_dn10, locals.var_uc_depwlp_dn11, locals.var_uc_depwlp_dn14,)
    }
};
        locals.var_uc_depwlp = assign17670_e12196;
        locals.var_uc_depwlp_dn0 = assign17670_e12196_d_n0;
        locals.var_uc_depwlp_dn2 = assign17670_e12196_d_n2;
        locals.var_uc_depwlp_dn4 = assign17670_e12196_d_n4;
        locals.var_uc_depwlp_dn5 = assign17670_e12196_d_n5;
        locals.var_uc_depwlp_dn6 = assign17670_e12196_d_n6;
        locals.var_uc_depwlp_dn7 = assign17670_e12196_d_n7;
        locals.var_uc_depwlp_dn8 = assign17670_e12196_d_n8;
        locals.var_uc_depwlp_dn9 = assign17670_e12196_d_n9;
        locals.var_uc_depwlp_dn10 = assign17670_e12196_d_n10;
        locals.var_uc_depwlp_dn11 = assign17670_e12196_d_n11;
        locals.var_uc_depwlp_dn14 = assign17670_e12196_d_n14;

        let (assign17680_e12206, assign17680_e12206_d_n0, assign17680_e12206_d_n2, assign17680_e12206_d_n4, assign17680_e12206_d_n5, assign17680_e12206_d_n6, assign17680_e12206_d_n7, assign17680_e12206_d_n8, assign17680_e12206_d_n9, assign17680_e12206_d_n10, assign17680_e12206_d_n11, assign17680_e12206_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard359 == 0.0)) && (locals.var_guard362 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pb2n, locals.var_pb2n_dn0, locals.var_pb2n_dn2, locals.var_pb2n_dn4, locals.var_pb2n_dn5, locals.var_pb2n_dn6, locals.var_pb2n_dn7, locals.var_pb2n_dn8, locals.var_pb2n_dn9, locals.var_pb2n_dn10, locals.var_pb2n_dn11, locals.var_pb2n_dn14,)
    }
};
        locals.var_pb2n = assign17680_e12206;
        locals.var_pb2n_dn0 = assign17680_e12206_d_n0;
        locals.var_pb2n_dn2 = assign17680_e12206_d_n2;
        locals.var_pb2n_dn4 = assign17680_e12206_d_n4;
        locals.var_pb2n_dn5 = assign17680_e12206_d_n5;
        locals.var_pb2n_dn6 = assign17680_e12206_d_n6;
        locals.var_pb2n_dn7 = assign17680_e12206_d_n7;
        locals.var_pb2n_dn8 = assign17680_e12206_d_n8;
        locals.var_pb2n_dn9 = assign17680_e12206_d_n9;
        locals.var_pb2n_dn10 = assign17680_e12206_d_n10;
        locals.var_pb2n_dn11 = assign17680_e12206_d_n11;
        locals.var_pb2n_dn14 = assign17680_e12206_d_n14;

        let (assign17690_e12225, assign17690_e12225_d_n0, assign17690_e12225_d_n2, assign17690_e12225_d_n4, assign17690_e12225_d_n5, assign17690_e12225_d_n6, assign17690_e12225_d_n7, assign17690_e12225_d_n8, assign17690_e12225_d_n9, assign17690_e12225_d_n10, assign17690_e12225_d_n11, assign17690_e12225_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard359 == 0.0)) && (locals.var_guard362 == 0.0)) {
        let assign17690_e12217: f64 = (locals.var_uc_njunc / locals.var_nin);
        let assign17690_e12219: f64 = (assign17690_e12217 * locals.var_nsub);
        let assign17690_e12221: f64 = (assign17690_e12219 / locals.var_nin);
        let assign17690_e12222: f64 = (assign17690_e12221).ln();
        let assign17690_e12223: f64 = (locals.var_beta_inv * assign17690_e12222);
        (assign17690_e12223, ((locals.var_beta_inv_dn0 * assign17690_e12222) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn0) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17690_e12217 * locals.var_nsub_dn0)) * locals.var_nin) - (assign17690_e12219 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign17690_e12221))), ((locals.var_beta_inv_dn2 * assign17690_e12222) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn2) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17690_e12217 * locals.var_nsub_dn2)) * locals.var_nin) - (assign17690_e12219 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign17690_e12221))), ((locals.var_beta_inv_dn4 * assign17690_e12222) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn4) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17690_e12217 * locals.var_nsub_dn4)) * locals.var_nin) - (assign17690_e12219 * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign17690_e12221))), ((locals.var_beta_inv_dn5 * assign17690_e12222) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn5) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17690_e12217 * locals.var_nsub_dn5)) * locals.var_nin) - (assign17690_e12219 * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign17690_e12221))), ((locals.var_beta_inv_dn6 * assign17690_e12222) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn6) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17690_e12217 * locals.var_nsub_dn6)) * locals.var_nin) - (assign17690_e12219 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign17690_e12221))), ((locals.var_beta_inv_dn7 * assign17690_e12222) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn7) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17690_e12217 * locals.var_nsub_dn7)) * locals.var_nin) - (assign17690_e12219 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign17690_e12221))), ((locals.var_beta_inv_dn8 * assign17690_e12222) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn8) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17690_e12217 * locals.var_nsub_dn8)) * locals.var_nin) - (assign17690_e12219 * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign17690_e12221))), ((locals.var_beta_inv_dn9 * assign17690_e12222) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn9) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17690_e12217 * locals.var_nsub_dn9)) * locals.var_nin) - (assign17690_e12219 * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign17690_e12221))), ((locals.var_beta_inv_dn10 * assign17690_e12222) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn10) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17690_e12217 * locals.var_nsub_dn10)) * locals.var_nin) - (assign17690_e12219 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign17690_e12221))), ((locals.var_beta_inv_dn11 * assign17690_e12222) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn11) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17690_e12217 * locals.var_nsub_dn11)) * locals.var_nin) - (assign17690_e12219 * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)) / assign17690_e12221))), ((locals.var_beta_inv_dn14 * assign17690_e12222) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn14) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17690_e12217 * locals.var_nsub_dn14)) * locals.var_nin) - (assign17690_e12219 * locals.var_nin_dn14)) / (locals.var_nin * locals.var_nin)) / assign17690_e12221))),)
    } else {
        (locals.var_vbipn, locals.var_vbipn_dn0, locals.var_vbipn_dn2, locals.var_vbipn_dn4, locals.var_vbipn_dn5, locals.var_vbipn_dn6, locals.var_vbipn_dn7, locals.var_vbipn_dn8, locals.var_vbipn_dn9, locals.var_vbipn_dn10, locals.var_vbipn_dn11, locals.var_vbipn_dn14,)
    }
};
        locals.var_vbipn = assign17690_e12225;
        locals.var_vbipn_dn0 = assign17690_e12225_d_n0;
        locals.var_vbipn_dn2 = assign17690_e12225_d_n2;
        locals.var_vbipn_dn4 = assign17690_e12225_d_n4;
        locals.var_vbipn_dn5 = assign17690_e12225_d_n5;
        locals.var_vbipn_dn6 = assign17690_e12225_d_n6;
        locals.var_vbipn_dn7 = assign17690_e12225_d_n7;
        locals.var_vbipn_dn8 = assign17690_e12225_d_n8;
        locals.var_vbipn_dn9 = assign17690_e12225_d_n9;
        locals.var_vbipn_dn10 = assign17690_e12225_d_n10;
        locals.var_vbipn_dn11 = assign17690_e12225_d_n11;
        locals.var_vbipn_dn14 = assign17690_e12225_d_n14;

        let (assign17700_e12235, assign17700_e12235_d_n0, assign17700_e12235_d_n2, assign17700_e12235_d_n4, assign17700_e12235_d_n5, assign17700_e12235_d_n6, assign17700_e12235_d_n7, assign17700_e12235_d_n8, assign17700_e12235_d_n9, assign17700_e12235_d_n10, assign17700_e12235_d_n11, assign17700_e12235_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard359 == 0.0)) && (locals.var_guard362 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_depmphn0, locals.var_depmphn0_dn0, locals.var_depmphn0_dn2, locals.var_depmphn0_dn4, locals.var_depmphn0_dn5, locals.var_depmphn0_dn6, locals.var_depmphn0_dn7, locals.var_depmphn0_dn8, locals.var_depmphn0_dn9, locals.var_depmphn0_dn10, locals.var_depmphn0_dn11, locals.var_depmphn0_dn14,)
    }
};
        locals.var_depmphn0 = assign17700_e12235;
        locals.var_depmphn0_dn0 = assign17700_e12235_d_n0;
        locals.var_depmphn0_dn2 = assign17700_e12235_d_n2;
        locals.var_depmphn0_dn4 = assign17700_e12235_d_n4;
        locals.var_depmphn0_dn5 = assign17700_e12235_d_n5;
        locals.var_depmphn0_dn6 = assign17700_e12235_d_n6;
        locals.var_depmphn0_dn7 = assign17700_e12235_d_n7;
        locals.var_depmphn0_dn8 = assign17700_e12235_d_n8;
        locals.var_depmphn0_dn9 = assign17700_e12235_d_n9;
        locals.var_depmphn0_dn10 = assign17700_e12235_d_n10;
        locals.var_depmphn0_dn11 = assign17700_e12235_d_n11;
        locals.var_depmphn0_dn14 = assign17700_e12235_d_n14;

        let (assign17710_e12241, assign17710_e12241_d_n0, assign17710_e12241_d_n2, assign17710_e12241_d_n4, assign17710_e12241_d_n5, assign17710_e12241_d_n6, assign17710_e12241_d_n7, assign17710_e12241_d_n8, assign17710_e12241_d_n9, assign17710_e12241_d_n10, assign17710_e12241_d_n11, assign17710_e12241_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign17710_e12239: f64 = (locals.var_ptovr0 * locals.var_beta_inv);
        (assign17710_e12239, ((locals.var_ptovr0_dn0 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn0)), ((locals.var_ptovr0_dn2 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn2)), ((locals.var_ptovr0_dn4 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn4)), ((locals.var_ptovr0_dn5 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn5)), ((locals.var_ptovr0_dn6 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn6)), ((locals.var_ptovr0_dn7 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn7)), ((locals.var_ptovr0_dn8 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn8)), ((locals.var_ptovr0_dn9 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn9)), ((locals.var_ptovr0_dn10 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn10)), ((locals.var_ptovr0_dn11 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn11)), ((locals.var_ptovr0_dn14 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn14)),)
    } else {
        (locals.var_ptovr, locals.var_ptovr_dn0, locals.var_ptovr_dn2, locals.var_ptovr_dn4, locals.var_ptovr_dn5, locals.var_ptovr_dn6, locals.var_ptovr_dn7, locals.var_ptovr_dn8, locals.var_ptovr_dn9, locals.var_ptovr_dn10, locals.var_ptovr_dn11, locals.var_ptovr_dn14,)
    }
};
        locals.var_ptovr = assign17710_e12241;
        locals.var_ptovr_dn0 = assign17710_e12241_d_n0;
        locals.var_ptovr_dn2 = assign17710_e12241_d_n2;
        locals.var_ptovr_dn4 = assign17710_e12241_d_n4;
        locals.var_ptovr_dn5 = assign17710_e12241_d_n5;
        locals.var_ptovr_dn6 = assign17710_e12241_d_n6;
        locals.var_ptovr_dn7 = assign17710_e12241_d_n7;
        locals.var_ptovr_dn8 = assign17710_e12241_d_n8;
        locals.var_ptovr_dn9 = assign17710_e12241_d_n9;
        locals.var_ptovr_dn10 = assign17710_e12241_d_n10;
        locals.var_ptovr_dn11 = assign17710_e12241_d_n11;
        locals.var_ptovr_dn14 = assign17710_e12241_d_n14;

    }

    pub(super) fn stamp_transient_block_39(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign17720_e12247, assign17720_e12247_d_n0, assign17720_e12247_d_n2, assign17720_e12247_d_n4, assign17720_e12247_d_n5, assign17720_e12247_d_n6, assign17720_e12247_d_n7, assign17720_e12247_d_n8, assign17720_e12247_d_n9, assign17720_e12247_d_n10, assign17720_e12247_d_n11, assign17720_e12247_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign17720_e12245: f64 = (locals.var_ttemp / locals.var_ktnom);
        (assign17720_e12245, (locals.var_ttemp_dn0 / locals.var_ktnom), (locals.var_ttemp_dn2 / locals.var_ktnom), (locals.var_ttemp_dn4 / locals.var_ktnom), (locals.var_ttemp_dn5 / locals.var_ktnom), (locals.var_ttemp_dn6 / locals.var_ktnom), (locals.var_ttemp_dn7 / locals.var_ktnom), (locals.var_ttemp_dn8 / locals.var_ktnom), (locals.var_ttemp_dn9 / locals.var_ktnom), (locals.var_ttemp_dn10 / locals.var_ktnom), (locals.var_ttemp_dn11 / locals.var_ktnom), (locals.var_ttemp_dn14 / locals.var_ktnom),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign17720_e12247;
        locals.var_t1_dn0 = assign17720_e12247_d_n0;
        locals.var_t1_dn2 = assign17720_e12247_d_n2;
        locals.var_t1_dn4 = assign17720_e12247_d_n4;
        locals.var_t1_dn5 = assign17720_e12247_d_n5;
        locals.var_t1_dn6 = assign17720_e12247_d_n6;
        locals.var_t1_dn7 = assign17720_e12247_d_n7;
        locals.var_t1_dn8 = assign17720_e12247_d_n8;
        locals.var_t1_dn9 = assign17720_e12247_d_n9;
        locals.var_t1_dn10 = assign17720_e12247_d_n10;
        locals.var_t1_dn11 = assign17720_e12247_d_n11;
        locals.var_t1_dn14 = assign17720_e12247_d_n14;

        let (assign17730_e12267, assign17730_e12267_d_n0, assign17730_e12267_d_n2, assign17730_e12267_d_n4, assign17730_e12267_d_n5, assign17730_e12267_d_n6, assign17730_e12267_d_n7, assign17730_e12267_d_n8, assign17730_e12267_d_n9, assign17730_e12267_d_n10, assign17730_e12267_d_n11, assign17730_e12267_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign17730_e12252: f64 = (0.4 * locals.var_t1);
        let assign17730_e12253: f64 = (1.8 + assign17730_e12252);
        let assign17730_e12256: f64 = (0.1 * locals.var_t1);
        let assign17730_e12258: f64 = (assign17730_e12256 * locals.var_t1);
        let assign17730_e12259: f64 = (assign17730_e12253 + assign17730_e12258);
        let assign17730_e12263: f64 = (1.0 - locals.var_t1);
        let assign17730_e12264: f64 = (locals.var_uc_vtmp * assign17730_e12263);
        let assign17730_e12265: f64 = (assign17730_e12259 - assign17730_e12264);
        (assign17730_e12265, (((0.4 * locals.var_t1_dn0) + (((0.1 * locals.var_t1_dn0) * locals.var_t1) + (assign17730_e12256 * locals.var_t1_dn0))) - (locals.var_uc_vtmp * (-locals.var_t1_dn0))), (((0.4 * locals.var_t1_dn2) + (((0.1 * locals.var_t1_dn2) * locals.var_t1) + (assign17730_e12256 * locals.var_t1_dn2))) - (locals.var_uc_vtmp * (-locals.var_t1_dn2))), (((0.4 * locals.var_t1_dn4) + (((0.1 * locals.var_t1_dn4) * locals.var_t1) + (assign17730_e12256 * locals.var_t1_dn4))) - (locals.var_uc_vtmp * (-locals.var_t1_dn4))), (((0.4 * locals.var_t1_dn5) + (((0.1 * locals.var_t1_dn5) * locals.var_t1) + (assign17730_e12256 * locals.var_t1_dn5))) - (locals.var_uc_vtmp * (-locals.var_t1_dn5))), (((0.4 * locals.var_t1_dn6) + (((0.1 * locals.var_t1_dn6) * locals.var_t1) + (assign17730_e12256 * locals.var_t1_dn6))) - (locals.var_uc_vtmp * (-locals.var_t1_dn6))), (((0.4 * locals.var_t1_dn7) + (((0.1 * locals.var_t1_dn7) * locals.var_t1) + (assign17730_e12256 * locals.var_t1_dn7))) - (locals.var_uc_vtmp * (-locals.var_t1_dn7))), (((0.4 * locals.var_t1_dn8) + (((0.1 * locals.var_t1_dn8) * locals.var_t1) + (assign17730_e12256 * locals.var_t1_dn8))) - (locals.var_uc_vtmp * (-locals.var_t1_dn8))), (((0.4 * locals.var_t1_dn9) + (((0.1 * locals.var_t1_dn9) * locals.var_t1) + (assign17730_e12256 * locals.var_t1_dn9))) - (locals.var_uc_vtmp * (-locals.var_t1_dn9))), (((0.4 * locals.var_t1_dn10) + (((0.1 * locals.var_t1_dn10) * locals.var_t1) + (assign17730_e12256 * locals.var_t1_dn10))) - (locals.var_uc_vtmp * (-locals.var_t1_dn10))), (((0.4 * locals.var_t1_dn11) + (((0.1 * locals.var_t1_dn11) * locals.var_t1) + (assign17730_e12256 * locals.var_t1_dn11))) - (locals.var_uc_vtmp * (-locals.var_t1_dn11))), (((0.4 * locals.var_t1_dn14) + (((0.1 * locals.var_t1_dn14) * locals.var_t1) + (assign17730_e12256 * locals.var_t1_dn14))) - (locals.var_uc_vtmp * (-locals.var_t1_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign17730_e12267;
        locals.var_t0_dn0 = assign17730_e12267_d_n0;
        locals.var_t0_dn2 = assign17730_e12267_d_n2;
        locals.var_t0_dn4 = assign17730_e12267_d_n4;
        locals.var_t0_dn5 = assign17730_e12267_d_n5;
        locals.var_t0_dn6 = assign17730_e12267_d_n6;
        locals.var_t0_dn7 = assign17730_e12267_d_n7;
        locals.var_t0_dn8 = assign17730_e12267_d_n8;
        locals.var_t0_dn9 = assign17730_e12267_d_n9;
        locals.var_t0_dn10 = assign17730_e12267_d_n10;
        locals.var_t0_dn11 = assign17730_e12267_d_n11;
        locals.var_t0_dn14 = assign17730_e12267_d_n14;

        let assign17740_e12270: f64 = if p.p39 != 2.0 { 1.0 } else { 0.0 };
        locals.var_guard365 = assign17740_e12270;

        let (assign17750_e12290, assign17750_e12290_d_n0, assign17750_e12290_d_n2, assign17750_e12290_d_n4, assign17750_e12290_d_n5, assign17750_e12290_d_n6, assign17750_e12290_d_n7, assign17750_e12290_d_n8, assign17750_e12290_d_n9, assign17750_e12290_d_n10, assign17750_e12290_d_n11, assign17750_e12290_d_n14,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard365 != 0.0)) {
        let assign17750_e12276: f64 = (locals.var_vmax0 * locals.var_uc_vmax);
        let assign17750_e12278: f64 = (assign17750_e12276 / locals.var_t0);
        let assign17750_e12282: f64 = (p.p90 * locals.var_tdiff0);
        let assign17750_e12283: f64 = (1.0 + assign17750_e12282);
        let assign17750_e12286: f64 = (p.p91 * locals.var_tdiff0_2);
        let assign17750_e12287: f64 = (assign17750_e12283 + assign17750_e12286);
        let assign17750_e12288: f64 = (assign17750_e12278 * assign17750_e12287);
        (assign17750_e12288, (((-((assign17750_e12276 * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))) * assign17750_e12287) + (assign17750_e12278 * ((p.p90 * locals.var_tdiff0_dn0) + (p.p91 * locals.var_tdiff0_2_dn0)))), (((-((assign17750_e12276 * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))) * assign17750_e12287) + (assign17750_e12278 * ((p.p90 * locals.var_tdiff0_dn2) + (p.p91 * locals.var_tdiff0_2_dn2)))), (((-((assign17750_e12276 * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))) * assign17750_e12287) + (assign17750_e12278 * ((p.p90 * locals.var_tdiff0_dn4) + (p.p91 * locals.var_tdiff0_2_dn4)))), (((-((assign17750_e12276 * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))) * assign17750_e12287) + (assign17750_e12278 * ((p.p90 * locals.var_tdiff0_dn5) + (p.p91 * locals.var_tdiff0_2_dn5)))), (((-((assign17750_e12276 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))) * assign17750_e12287) + (assign17750_e12278 * ((p.p90 * locals.var_tdiff0_dn6) + (p.p91 * locals.var_tdiff0_2_dn6)))), (((-((assign17750_e12276 * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))) * assign17750_e12287) + (assign17750_e12278 * ((p.p90 * locals.var_tdiff0_dn7) + (p.p91 * locals.var_tdiff0_2_dn7)))), (((-((assign17750_e12276 * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))) * assign17750_e12287) + (assign17750_e12278 * ((p.p90 * locals.var_tdiff0_dn8) + (p.p91 * locals.var_tdiff0_2_dn8)))), (((-((assign17750_e12276 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))) * assign17750_e12287) + (assign17750_e12278 * ((p.p90 * locals.var_tdiff0_dn9) + (p.p91 * locals.var_tdiff0_2_dn9)))), (((-((assign17750_e12276 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))) * assign17750_e12287) + (assign17750_e12278 * ((p.p90 * locals.var_tdiff0_dn10) + (p.p91 * locals.var_tdiff0_2_dn10)))), (((-((assign17750_e12276 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))) * assign17750_e12287) + (assign17750_e12278 * ((p.p90 * locals.var_tdiff0_dn11) + (p.p91 * locals.var_tdiff0_2_dn11)))), (((-((assign17750_e12276 * locals.var_t0_dn14) / (locals.var_t0 * locals.var_t0))) * assign17750_e12287) + (assign17750_e12278 * ((p.p90 * locals.var_tdiff0_dn14) + (p.p91 * locals.var_tdiff0_2_dn14)))),)
    } else {
        (locals.var_vmaxeff, locals.var_vmaxeff_dn0, locals.var_vmaxeff_dn2, locals.var_vmaxeff_dn4, locals.var_vmaxeff_dn5, locals.var_vmaxeff_dn6, locals.var_vmaxeff_dn7, locals.var_vmaxeff_dn8, locals.var_vmaxeff_dn9, locals.var_vmaxeff_dn10, locals.var_vmaxeff_dn11, locals.var_vmaxeff_dn14,)
    }
};
        locals.var_vmaxeff = assign17750_e12290;
        locals.var_vmaxeff_dn0 = assign17750_e12290_d_n0;
        locals.var_vmaxeff_dn2 = assign17750_e12290_d_n2;
        locals.var_vmaxeff_dn4 = assign17750_e12290_d_n4;
        locals.var_vmaxeff_dn5 = assign17750_e12290_d_n5;
        locals.var_vmaxeff_dn6 = assign17750_e12290_d_n6;
        locals.var_vmaxeff_dn7 = assign17750_e12290_d_n7;
        locals.var_vmaxeff_dn8 = assign17750_e12290_d_n8;
        locals.var_vmaxeff_dn9 = assign17750_e12290_d_n9;
        locals.var_vmaxeff_dn10 = assign17750_e12290_d_n10;
        locals.var_vmaxeff_dn11 = assign17750_e12290_d_n11;
        locals.var_vmaxeff_dn14 = assign17750_e12290_d_n14;

        let (assign17760_e12311, assign17760_e12311_d_n0, assign17760_e12311_d_n2, assign17760_e12311_d_n4, assign17760_e12311_d_n5, assign17760_e12311_d_n6, assign17760_e12311_d_n7, assign17760_e12311_d_n8, assign17760_e12311_d_n9, assign17760_e12311_d_n10, assign17760_e12311_d_n11, assign17760_e12311_d_n14,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard365 == 0.0)) {
        let assign17760_e12297: f64 = (locals.var_vmax0 * locals.var_uc_vmax);
        let assign17760_e12299: f64 = (assign17760_e12297 / locals.var_t0);
        let assign17760_e12303: f64 = (p.p90 * locals.var_tdiff);
        let assign17760_e12304: f64 = (1.0 + assign17760_e12303);
        let assign17760_e12307: f64 = (p.p91 * locals.var_tdiff_2);
        let assign17760_e12308: f64 = (assign17760_e12304 + assign17760_e12307);
        let assign17760_e12309: f64 = (assign17760_e12299 * assign17760_e12308);
        (assign17760_e12309, (((-((assign17760_e12297 * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))) * assign17760_e12308) + (assign17760_e12299 * ((p.p90 * locals.var_tdiff_dn0) + (p.p91 * locals.var_tdiff_2_dn0)))), (((-((assign17760_e12297 * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))) * assign17760_e12308) + (assign17760_e12299 * ((p.p90 * locals.var_tdiff_dn2) + (p.p91 * locals.var_tdiff_2_dn2)))), (((-((assign17760_e12297 * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))) * assign17760_e12308) + (assign17760_e12299 * ((p.p90 * locals.var_tdiff_dn4) + (p.p91 * locals.var_tdiff_2_dn4)))), (((-((assign17760_e12297 * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))) * assign17760_e12308) + (assign17760_e12299 * ((p.p90 * locals.var_tdiff_dn5) + (p.p91 * locals.var_tdiff_2_dn5)))), (((-((assign17760_e12297 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))) * assign17760_e12308) + (assign17760_e12299 * ((p.p90 * locals.var_tdiff_dn6) + (p.p91 * locals.var_tdiff_2_dn6)))), (((-((assign17760_e12297 * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))) * assign17760_e12308) + (assign17760_e12299 * ((p.p90 * locals.var_tdiff_dn7) + (p.p91 * locals.var_tdiff_2_dn7)))), (((-((assign17760_e12297 * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))) * assign17760_e12308) + (assign17760_e12299 * ((p.p90 * locals.var_tdiff_dn8) + (p.p91 * locals.var_tdiff_2_dn8)))), (((-((assign17760_e12297 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))) * assign17760_e12308) + (assign17760_e12299 * ((p.p90 * locals.var_tdiff_dn9) + (p.p91 * locals.var_tdiff_2_dn9)))), (((-((assign17760_e12297 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))) * assign17760_e12308) + (assign17760_e12299 * ((p.p90 * locals.var_tdiff_dn10) + (p.p91 * locals.var_tdiff_2_dn10)))), (((-((assign17760_e12297 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))) * assign17760_e12308) + (assign17760_e12299 * ((p.p90 * locals.var_tdiff_dn11) + (p.p91 * locals.var_tdiff_2_dn11)))), (((-((assign17760_e12297 * locals.var_t0_dn14) / (locals.var_t0 * locals.var_t0))) * assign17760_e12308) + (assign17760_e12299 * ((p.p90 * locals.var_tdiff_dn14) + (p.p91 * locals.var_tdiff_2_dn14)))),)
    } else {
        (locals.var_vmaxeff, locals.var_vmaxeff_dn0, locals.var_vmaxeff_dn2, locals.var_vmaxeff_dn4, locals.var_vmaxeff_dn5, locals.var_vmaxeff_dn6, locals.var_vmaxeff_dn7, locals.var_vmaxeff_dn8, locals.var_vmaxeff_dn9, locals.var_vmaxeff_dn10, locals.var_vmaxeff_dn11, locals.var_vmaxeff_dn14,)
    }
};
        locals.var_vmaxeff = assign17760_e12311;
        locals.var_vmaxeff_dn0 = assign17760_e12311_d_n0;
        locals.var_vmaxeff_dn2 = assign17760_e12311_d_n2;
        locals.var_vmaxeff_dn4 = assign17760_e12311_d_n4;
        locals.var_vmaxeff_dn5 = assign17760_e12311_d_n5;
        locals.var_vmaxeff_dn6 = assign17760_e12311_d_n6;
        locals.var_vmaxeff_dn7 = assign17760_e12311_d_n7;
        locals.var_vmaxeff_dn8 = assign17760_e12311_d_n8;
        locals.var_vmaxeff_dn9 = assign17760_e12311_d_n9;
        locals.var_vmaxeff_dn10 = assign17760_e12311_d_n10;
        locals.var_vmaxeff_dn11 = assign17760_e12311_d_n11;
        locals.var_vmaxeff_dn14 = assign17760_e12311_d_n14;

        let assign17780_e12319: f64 = if p.p39 != 2.0 { 1.0 } else { 0.0 };
        locals.var_guard367 = assign17780_e12319;

        let (assign17790_e12335, assign17790_e12335_d_n0, assign17790_e12335_d_n2, assign17790_e12335_d_n4, assign17790_e12335_d_n5, assign17790_e12335_d_n6, assign17790_e12335_d_n7, assign17790_e12335_d_n8, assign17790_e12335_d_n9, assign17790_e12335_d_n10, assign17790_e12335_d_n11, assign17790_e12335_d_n14,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard367 != 0.0)) {
        let assign17790_e12327: f64 = (p.p324 * locals.var_tdiff0);
        let assign17790_e12328: f64 = (1.0 + assign17790_e12327);
        let assign17790_e12331: f64 = (p.p325 * locals.var_tdiff0_2);
        let assign17790_e12332: f64 = (assign17790_e12328 + assign17790_e12331);
        let assign17790_e12333: f64 = (locals.var_ninvd0 * assign17790_e12332);
        (assign17790_e12333, (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn0) + (p.p325 * locals.var_tdiff0_2_dn0))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn2) + (p.p325 * locals.var_tdiff0_2_dn2))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn4) + (p.p325 * locals.var_tdiff0_2_dn4))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn5) + (p.p325 * locals.var_tdiff0_2_dn5))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn6) + (p.p325 * locals.var_tdiff0_2_dn6))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn7) + (p.p325 * locals.var_tdiff0_2_dn7))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn8) + (p.p325 * locals.var_tdiff0_2_dn8))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn9) + (p.p325 * locals.var_tdiff0_2_dn9))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn10) + (p.p325 * locals.var_tdiff0_2_dn10))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn11) + (p.p325 * locals.var_tdiff0_2_dn11))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn14) + (p.p325 * locals.var_tdiff0_2_dn14))),)
    } else {
        (locals.var_ninvde, locals.var_ninvde_dn0, locals.var_ninvde_dn2, locals.var_ninvde_dn4, locals.var_ninvde_dn5, locals.var_ninvde_dn6, locals.var_ninvde_dn7, locals.var_ninvde_dn8, locals.var_ninvde_dn9, locals.var_ninvde_dn10, locals.var_ninvde_dn11, locals.var_ninvde_dn14,)
    }
};
        locals.var_ninvde = assign17790_e12335;
        locals.var_ninvde_dn0 = assign17790_e12335_d_n0;
        locals.var_ninvde_dn2 = assign17790_e12335_d_n2;
        locals.var_ninvde_dn4 = assign17790_e12335_d_n4;
        locals.var_ninvde_dn5 = assign17790_e12335_d_n5;
        locals.var_ninvde_dn6 = assign17790_e12335_d_n6;
        locals.var_ninvde_dn7 = assign17790_e12335_d_n7;
        locals.var_ninvde_dn8 = assign17790_e12335_d_n8;
        locals.var_ninvde_dn9 = assign17790_e12335_d_n9;
        locals.var_ninvde_dn10 = assign17790_e12335_d_n10;
        locals.var_ninvde_dn11 = assign17790_e12335_d_n11;
        locals.var_ninvde_dn14 = assign17790_e12335_d_n14;

        let (assign17800_e12349, assign17800_e12349_d_n0, assign17800_e12349_d_n2, assign17800_e12349_d_n4, assign17800_e12349_d_n5, assign17800_e12349_d_n6, assign17800_e12349_d_n7, assign17800_e12349_d_n8, assign17800_e12349_d_n9, assign17800_e12349_d_n10, assign17800_e12349_d_n11, assign17800_e12349_d_n14,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard367 != 0.0)) {
        let assign17800_e12342: f64 = (p.p390 * locals.var_tdiff0);
        let assign17800_e12343: f64 = (1.0 + assign17800_e12342);
        let assign17800_e12346: f64 = (p.p391 * locals.var_tdiff0_2);
        let assign17800_e12347: f64 = (assign17800_e12343 + assign17800_e12346);
        (assign17800_e12347, ((p.p390 * locals.var_tdiff0_dn0) + (p.p391 * locals.var_tdiff0_2_dn0)), ((p.p390 * locals.var_tdiff0_dn2) + (p.p391 * locals.var_tdiff0_2_dn2)), ((p.p390 * locals.var_tdiff0_dn4) + (p.p391 * locals.var_tdiff0_2_dn4)), ((p.p390 * locals.var_tdiff0_dn5) + (p.p391 * locals.var_tdiff0_2_dn5)), ((p.p390 * locals.var_tdiff0_dn6) + (p.p391 * locals.var_tdiff0_2_dn6)), ((p.p390 * locals.var_tdiff0_dn7) + (p.p391 * locals.var_tdiff0_2_dn7)), ((p.p390 * locals.var_tdiff0_dn8) + (p.p391 * locals.var_tdiff0_2_dn8)), ((p.p390 * locals.var_tdiff0_dn9) + (p.p391 * locals.var_tdiff0_2_dn9)), ((p.p390 * locals.var_tdiff0_dn10) + (p.p391 * locals.var_tdiff0_2_dn10)), ((p.p390 * locals.var_tdiff0_dn11) + (p.p391 * locals.var_tdiff0_2_dn11)), ((p.p390 * locals.var_tdiff0_dn14) + (p.p391 * locals.var_tdiff0_2_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign17800_e12349;
        locals.var_t1_dn0 = assign17800_e12349_d_n0;
        locals.var_t1_dn2 = assign17800_e12349_d_n2;
        locals.var_t1_dn4 = assign17800_e12349_d_n4;
        locals.var_t1_dn5 = assign17800_e12349_d_n5;
        locals.var_t1_dn6 = assign17800_e12349_d_n6;
        locals.var_t1_dn7 = assign17800_e12349_d_n7;
        locals.var_t1_dn8 = assign17800_e12349_d_n8;
        locals.var_t1_dn9 = assign17800_e12349_d_n9;
        locals.var_t1_dn10 = assign17800_e12349_d_n10;
        locals.var_t1_dn11 = assign17800_e12349_d_n11;
        locals.var_t1_dn14 = assign17800_e12349_d_n14;

        let (assign17810_e12357, assign17810_e12357_d_n0, assign17810_e12357_d_n2, assign17810_e12357_d_n4, assign17810_e12357_d_n5, assign17810_e12357_d_n6, assign17810_e12357_d_n7, assign17810_e12357_d_n8, assign17810_e12357_d_n9, assign17810_e12357_d_n10, assign17810_e12357_d_n11, assign17810_e12357_d_n14,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard367 != 0.0)) {
        let assign17810_e12355: f64 = (locals.var_ninvd0cres * locals.var_t1);
        (assign17810_e12355, ((locals.var_ninvd0cres_dn0 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn0)), ((locals.var_ninvd0cres_dn2 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn2)), ((locals.var_ninvd0cres_dn4 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn4)), ((locals.var_ninvd0cres_dn5 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn5)), ((locals.var_ninvd0cres_dn6 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn6)), ((locals.var_ninvd0cres_dn7 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn7)), ((locals.var_ninvd0cres_dn8 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn8)), ((locals.var_ninvd0cres_dn9 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn9)), ((locals.var_ninvd0cres_dn10 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn10)), ((locals.var_ninvd0cres_dn11 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn11)), ((locals.var_ninvd0cres_dn14 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn14)),)
    } else {
        (locals.var_ninvdecres, locals.var_ninvdecres_dn0, locals.var_ninvdecres_dn2, locals.var_ninvdecres_dn4, locals.var_ninvdecres_dn5, locals.var_ninvdecres_dn6, locals.var_ninvdecres_dn7, locals.var_ninvdecres_dn8, locals.var_ninvdecres_dn9, locals.var_ninvdecres_dn10, locals.var_ninvdecres_dn11, locals.var_ninvdecres_dn14,)
    }
};
        locals.var_ninvdecres = assign17810_e12357;
        locals.var_ninvdecres_dn0 = assign17810_e12357_d_n0;
        locals.var_ninvdecres_dn2 = assign17810_e12357_d_n2;
        locals.var_ninvdecres_dn4 = assign17810_e12357_d_n4;
        locals.var_ninvdecres_dn5 = assign17810_e12357_d_n5;
        locals.var_ninvdecres_dn6 = assign17810_e12357_d_n6;
        locals.var_ninvdecres_dn7 = assign17810_e12357_d_n7;
        locals.var_ninvdecres_dn8 = assign17810_e12357_d_n8;
        locals.var_ninvdecres_dn9 = assign17810_e12357_d_n9;
        locals.var_ninvdecres_dn10 = assign17810_e12357_d_n10;
        locals.var_ninvdecres_dn11 = assign17810_e12357_d_n11;
        locals.var_ninvdecres_dn14 = assign17810_e12357_d_n14;

        let (assign17820_e12365, assign17820_e12365_d_n0, assign17820_e12365_d_n2, assign17820_e12365_d_n4, assign17820_e12365_d_n5, assign17820_e12365_d_n6, assign17820_e12365_d_n7, assign17820_e12365_d_n8, assign17820_e12365_d_n9, assign17820_e12365_d_n10, assign17820_e12365_d_n11, assign17820_e12365_d_n14,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard367 != 0.0)) {
        let assign17820_e12363: f64 = (locals.var_ninvd0hres * locals.var_t1);
        (assign17820_e12363, ((locals.var_ninvd0hres_dn0 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn0)), ((locals.var_ninvd0hres_dn2 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn2)), ((locals.var_ninvd0hres_dn4 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn4)), ((locals.var_ninvd0hres_dn5 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn5)), ((locals.var_ninvd0hres_dn6 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn6)), ((locals.var_ninvd0hres_dn7 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn7)), ((locals.var_ninvd0hres_dn8 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn8)), ((locals.var_ninvd0hres_dn9 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn9)), ((locals.var_ninvd0hres_dn10 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn10)), ((locals.var_ninvd0hres_dn11 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn11)), ((locals.var_ninvd0hres_dn14 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn14)),)
    } else {
        (locals.var_ninvdehres, locals.var_ninvdehres_dn0, locals.var_ninvdehres_dn2, locals.var_ninvdehres_dn4, locals.var_ninvdehres_dn5, locals.var_ninvdehres_dn6, locals.var_ninvdehres_dn7, locals.var_ninvdehres_dn8, locals.var_ninvdehres_dn9, locals.var_ninvdehres_dn10, locals.var_ninvdehres_dn11, locals.var_ninvdehres_dn14,)
    }
};
        locals.var_ninvdehres = assign17820_e12365;
        locals.var_ninvdehres_dn0 = assign17820_e12365_d_n0;
        locals.var_ninvdehres_dn2 = assign17820_e12365_d_n2;
        locals.var_ninvdehres_dn4 = assign17820_e12365_d_n4;
        locals.var_ninvdehres_dn5 = assign17820_e12365_d_n5;
        locals.var_ninvdehres_dn6 = assign17820_e12365_d_n6;
        locals.var_ninvdehres_dn7 = assign17820_e12365_d_n7;
        locals.var_ninvdehres_dn8 = assign17820_e12365_d_n8;
        locals.var_ninvdehres_dn9 = assign17820_e12365_d_n9;
        locals.var_ninvdehres_dn10 = assign17820_e12365_d_n10;
        locals.var_ninvdehres_dn11 = assign17820_e12365_d_n11;
        locals.var_ninvdehres_dn14 = assign17820_e12365_d_n14;

        let (assign17830_e12382, assign17830_e12382_d_n0, assign17830_e12382_d_n2, assign17830_e12382_d_n4, assign17830_e12382_d_n5, assign17830_e12382_d_n6, assign17830_e12382_d_n7, assign17830_e12382_d_n8, assign17830_e12382_d_n9, assign17830_e12382_d_n10, assign17830_e12382_d_n11, assign17830_e12382_d_n14,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard367 == 0.0)) {
        let assign17830_e12374: f64 = (p.p324 * locals.var_tdiff);
        let assign17830_e12375: f64 = (1.0 + assign17830_e12374);
        let assign17830_e12378: f64 = (p.p325 * locals.var_tdiff_2);
        let assign17830_e12379: f64 = (assign17830_e12375 + assign17830_e12378);
        let assign17830_e12380: f64 = (locals.var_ninvd0 * assign17830_e12379);
        (assign17830_e12380, (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn0) + (p.p325 * locals.var_tdiff_2_dn0))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn2) + (p.p325 * locals.var_tdiff_2_dn2))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn4) + (p.p325 * locals.var_tdiff_2_dn4))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn5) + (p.p325 * locals.var_tdiff_2_dn5))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn6) + (p.p325 * locals.var_tdiff_2_dn6))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn7) + (p.p325 * locals.var_tdiff_2_dn7))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn8) + (p.p325 * locals.var_tdiff_2_dn8))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn9) + (p.p325 * locals.var_tdiff_2_dn9))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn10) + (p.p325 * locals.var_tdiff_2_dn10))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn11) + (p.p325 * locals.var_tdiff_2_dn11))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn14) + (p.p325 * locals.var_tdiff_2_dn14))),)
    } else {
        (locals.var_ninvde, locals.var_ninvde_dn0, locals.var_ninvde_dn2, locals.var_ninvde_dn4, locals.var_ninvde_dn5, locals.var_ninvde_dn6, locals.var_ninvde_dn7, locals.var_ninvde_dn8, locals.var_ninvde_dn9, locals.var_ninvde_dn10, locals.var_ninvde_dn11, locals.var_ninvde_dn14,)
    }
};
        locals.var_ninvde = assign17830_e12382;
        locals.var_ninvde_dn0 = assign17830_e12382_d_n0;
        locals.var_ninvde_dn2 = assign17830_e12382_d_n2;
        locals.var_ninvde_dn4 = assign17830_e12382_d_n4;
        locals.var_ninvde_dn5 = assign17830_e12382_d_n5;
        locals.var_ninvde_dn6 = assign17830_e12382_d_n6;
        locals.var_ninvde_dn7 = assign17830_e12382_d_n7;
        locals.var_ninvde_dn8 = assign17830_e12382_d_n8;
        locals.var_ninvde_dn9 = assign17830_e12382_d_n9;
        locals.var_ninvde_dn10 = assign17830_e12382_d_n10;
        locals.var_ninvde_dn11 = assign17830_e12382_d_n11;
        locals.var_ninvde_dn14 = assign17830_e12382_d_n14;

        let (assign17840_e12397, assign17840_e12397_d_n0, assign17840_e12397_d_n2, assign17840_e12397_d_n4, assign17840_e12397_d_n5, assign17840_e12397_d_n6, assign17840_e12397_d_n7, assign17840_e12397_d_n8, assign17840_e12397_d_n9, assign17840_e12397_d_n10, assign17840_e12397_d_n11, assign17840_e12397_d_n14,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard367 == 0.0)) {
        let assign17840_e12390: f64 = (p.p390 * locals.var_tdiff);
        let assign17840_e12391: f64 = (1.0 + assign17840_e12390);
        let assign17840_e12394: f64 = (p.p391 * locals.var_tdiff_2);
        let assign17840_e12395: f64 = (assign17840_e12391 + assign17840_e12394);
        (assign17840_e12395, ((p.p390 * locals.var_tdiff_dn0) + (p.p391 * locals.var_tdiff_2_dn0)), ((p.p390 * locals.var_tdiff_dn2) + (p.p391 * locals.var_tdiff_2_dn2)), ((p.p390 * locals.var_tdiff_dn4) + (p.p391 * locals.var_tdiff_2_dn4)), ((p.p390 * locals.var_tdiff_dn5) + (p.p391 * locals.var_tdiff_2_dn5)), ((p.p390 * locals.var_tdiff_dn6) + (p.p391 * locals.var_tdiff_2_dn6)), ((p.p390 * locals.var_tdiff_dn7) + (p.p391 * locals.var_tdiff_2_dn7)), ((p.p390 * locals.var_tdiff_dn8) + (p.p391 * locals.var_tdiff_2_dn8)), ((p.p390 * locals.var_tdiff_dn9) + (p.p391 * locals.var_tdiff_2_dn9)), ((p.p390 * locals.var_tdiff_dn10) + (p.p391 * locals.var_tdiff_2_dn10)), ((p.p390 * locals.var_tdiff_dn11) + (p.p391 * locals.var_tdiff_2_dn11)), ((p.p390 * locals.var_tdiff_dn14) + (p.p391 * locals.var_tdiff_2_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign17840_e12397;
        locals.var_t1_dn0 = assign17840_e12397_d_n0;
        locals.var_t1_dn2 = assign17840_e12397_d_n2;
        locals.var_t1_dn4 = assign17840_e12397_d_n4;
        locals.var_t1_dn5 = assign17840_e12397_d_n5;
        locals.var_t1_dn6 = assign17840_e12397_d_n6;
        locals.var_t1_dn7 = assign17840_e12397_d_n7;
        locals.var_t1_dn8 = assign17840_e12397_d_n8;
        locals.var_t1_dn9 = assign17840_e12397_d_n9;
        locals.var_t1_dn10 = assign17840_e12397_d_n10;
        locals.var_t1_dn11 = assign17840_e12397_d_n11;
        locals.var_t1_dn14 = assign17840_e12397_d_n14;

        let (assign17850_e12406, assign17850_e12406_d_n0, assign17850_e12406_d_n2, assign17850_e12406_d_n4, assign17850_e12406_d_n5, assign17850_e12406_d_n6, assign17850_e12406_d_n7, assign17850_e12406_d_n8, assign17850_e12406_d_n9, assign17850_e12406_d_n10, assign17850_e12406_d_n11, assign17850_e12406_d_n14,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard367 == 0.0)) {
        let assign17850_e12404: f64 = (locals.var_ninvd0cres * locals.var_t1);
        (assign17850_e12404, ((locals.var_ninvd0cres_dn0 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn0)), ((locals.var_ninvd0cres_dn2 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn2)), ((locals.var_ninvd0cres_dn4 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn4)), ((locals.var_ninvd0cres_dn5 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn5)), ((locals.var_ninvd0cres_dn6 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn6)), ((locals.var_ninvd0cres_dn7 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn7)), ((locals.var_ninvd0cres_dn8 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn8)), ((locals.var_ninvd0cres_dn9 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn9)), ((locals.var_ninvd0cres_dn10 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn10)), ((locals.var_ninvd0cres_dn11 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn11)), ((locals.var_ninvd0cres_dn14 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn14)),)
    } else {
        (locals.var_ninvdecres, locals.var_ninvdecres_dn0, locals.var_ninvdecres_dn2, locals.var_ninvdecres_dn4, locals.var_ninvdecres_dn5, locals.var_ninvdecres_dn6, locals.var_ninvdecres_dn7, locals.var_ninvdecres_dn8, locals.var_ninvdecres_dn9, locals.var_ninvdecres_dn10, locals.var_ninvdecres_dn11, locals.var_ninvdecres_dn14,)
    }
};
        locals.var_ninvdecres = assign17850_e12406;
        locals.var_ninvdecres_dn0 = assign17850_e12406_d_n0;
        locals.var_ninvdecres_dn2 = assign17850_e12406_d_n2;
        locals.var_ninvdecres_dn4 = assign17850_e12406_d_n4;
        locals.var_ninvdecres_dn5 = assign17850_e12406_d_n5;
        locals.var_ninvdecres_dn6 = assign17850_e12406_d_n6;
        locals.var_ninvdecres_dn7 = assign17850_e12406_d_n7;
        locals.var_ninvdecres_dn8 = assign17850_e12406_d_n8;
        locals.var_ninvdecres_dn9 = assign17850_e12406_d_n9;
        locals.var_ninvdecres_dn10 = assign17850_e12406_d_n10;
        locals.var_ninvdecres_dn11 = assign17850_e12406_d_n11;
        locals.var_ninvdecres_dn14 = assign17850_e12406_d_n14;

        let (assign17860_e12415, assign17860_e12415_d_n0, assign17860_e12415_d_n2, assign17860_e12415_d_n4, assign17860_e12415_d_n5, assign17860_e12415_d_n6, assign17860_e12415_d_n7, assign17860_e12415_d_n8, assign17860_e12415_d_n9, assign17860_e12415_d_n10, assign17860_e12415_d_n11, assign17860_e12415_d_n14,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard367 == 0.0)) {
        let assign17860_e12413: f64 = (locals.var_ninvd0hres * locals.var_t1);
        (assign17860_e12413, ((locals.var_ninvd0hres_dn0 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn0)), ((locals.var_ninvd0hres_dn2 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn2)), ((locals.var_ninvd0hres_dn4 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn4)), ((locals.var_ninvd0hres_dn5 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn5)), ((locals.var_ninvd0hres_dn6 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn6)), ((locals.var_ninvd0hres_dn7 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn7)), ((locals.var_ninvd0hres_dn8 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn8)), ((locals.var_ninvd0hres_dn9 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn9)), ((locals.var_ninvd0hres_dn10 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn10)), ((locals.var_ninvd0hres_dn11 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn11)), ((locals.var_ninvd0hres_dn14 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn14)),)
    } else {
        (locals.var_ninvdehres, locals.var_ninvdehres_dn0, locals.var_ninvdehres_dn2, locals.var_ninvdehres_dn4, locals.var_ninvdehres_dn5, locals.var_ninvdehres_dn6, locals.var_ninvdehres_dn7, locals.var_ninvdehres_dn8, locals.var_ninvdehres_dn9, locals.var_ninvdehres_dn10, locals.var_ninvdehres_dn11, locals.var_ninvdehres_dn14,)
    }
};
        locals.var_ninvdehres = assign17860_e12415;
        locals.var_ninvdehres_dn0 = assign17860_e12415_d_n0;
        locals.var_ninvdehres_dn2 = assign17860_e12415_d_n2;
        locals.var_ninvdehres_dn4 = assign17860_e12415_d_n4;
        locals.var_ninvdehres_dn5 = assign17860_e12415_d_n5;
        locals.var_ninvdehres_dn6 = assign17860_e12415_d_n6;
        locals.var_ninvdehres_dn7 = assign17860_e12415_d_n7;
        locals.var_ninvdehres_dn8 = assign17860_e12415_d_n8;
        locals.var_ninvdehres_dn9 = assign17860_e12415_d_n9;
        locals.var_ninvdehres_dn10 = assign17860_e12415_d_n10;
        locals.var_ninvdehres_dn11 = assign17860_e12415_d_n11;
        locals.var_ninvdehres_dn14 = assign17860_e12415_d_n14;

        let assign17880_e12423: f64 = if locals.var_ninvde < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard369 = assign17880_e12423;

        let (assign17890_e12429, assign17890_e12429_d_n0, assign17890_e12429_d_n2, assign17890_e12429_d_n4, assign17890_e12429_d_n5, assign17890_e12429_d_n6, assign17890_e12429_d_n7, assign17890_e12429_d_n8, assign17890_e12429_d_n9, assign17890_e12429_d_n10, assign17890_e12429_d_n11, assign17890_e12429_d_n14,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard369 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ninvde, locals.var_ninvde_dn0, locals.var_ninvde_dn2, locals.var_ninvde_dn4, locals.var_ninvde_dn5, locals.var_ninvde_dn6, locals.var_ninvde_dn7, locals.var_ninvde_dn8, locals.var_ninvde_dn9, locals.var_ninvde_dn10, locals.var_ninvde_dn11, locals.var_ninvde_dn14,)
    }
};
        locals.var_ninvde = assign17890_e12429;
        locals.var_ninvde_dn0 = assign17890_e12429_d_n0;
        locals.var_ninvde_dn2 = assign17890_e12429_d_n2;
        locals.var_ninvde_dn4 = assign17890_e12429_d_n4;
        locals.var_ninvde_dn5 = assign17890_e12429_d_n5;
        locals.var_ninvde_dn6 = assign17890_e12429_d_n6;
        locals.var_ninvde_dn7 = assign17890_e12429_d_n7;
        locals.var_ninvde_dn8 = assign17890_e12429_d_n8;
        locals.var_ninvde_dn9 = assign17890_e12429_d_n9;
        locals.var_ninvde_dn10 = assign17890_e12429_d_n10;
        locals.var_ninvde_dn11 = assign17890_e12429_d_n11;
        locals.var_ninvde_dn14 = assign17890_e12429_d_n14;

        let assign17910_e12437: f64 = if locals.var_ninvdecres < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard371 = assign17910_e12437;

        let (assign17920_e12443, assign17920_e12443_d_n0, assign17920_e12443_d_n2, assign17920_e12443_d_n4, assign17920_e12443_d_n5, assign17920_e12443_d_n6, assign17920_e12443_d_n7, assign17920_e12443_d_n8, assign17920_e12443_d_n9, assign17920_e12443_d_n10, assign17920_e12443_d_n11, assign17920_e12443_d_n14,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard371 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ninvdecres, locals.var_ninvdecres_dn0, locals.var_ninvdecres_dn2, locals.var_ninvdecres_dn4, locals.var_ninvdecres_dn5, locals.var_ninvdecres_dn6, locals.var_ninvdecres_dn7, locals.var_ninvdecres_dn8, locals.var_ninvdecres_dn9, locals.var_ninvdecres_dn10, locals.var_ninvdecres_dn11, locals.var_ninvdecres_dn14,)
    }
};
        locals.var_ninvdecres = assign17920_e12443;
        locals.var_ninvdecres_dn0 = assign17920_e12443_d_n0;
        locals.var_ninvdecres_dn2 = assign17920_e12443_d_n2;
        locals.var_ninvdecres_dn4 = assign17920_e12443_d_n4;
        locals.var_ninvdecres_dn5 = assign17920_e12443_d_n5;
        locals.var_ninvdecres_dn6 = assign17920_e12443_d_n6;
        locals.var_ninvdecres_dn7 = assign17920_e12443_d_n7;
        locals.var_ninvdecres_dn8 = assign17920_e12443_d_n8;
        locals.var_ninvdecres_dn9 = assign17920_e12443_d_n9;
        locals.var_ninvdecres_dn10 = assign17920_e12443_d_n10;
        locals.var_ninvdecres_dn11 = assign17920_e12443_d_n11;
        locals.var_ninvdecres_dn14 = assign17920_e12443_d_n14;

        let assign17940_e12451: f64 = if locals.var_ninvdehres < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard373 = assign17940_e12451;

        let (assign17950_e12457, assign17950_e12457_d_n0, assign17950_e12457_d_n2, assign17950_e12457_d_n4, assign17950_e12457_d_n5, assign17950_e12457_d_n6, assign17950_e12457_d_n7, assign17950_e12457_d_n8, assign17950_e12457_d_n9, assign17950_e12457_d_n10, assign17950_e12457_d_n11, assign17950_e12457_d_n14,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard373 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ninvdehres, locals.var_ninvdehres_dn0, locals.var_ninvdehres_dn2, locals.var_ninvdehres_dn4, locals.var_ninvdehres_dn5, locals.var_ninvdehres_dn6, locals.var_ninvdehres_dn7, locals.var_ninvdehres_dn8, locals.var_ninvdehres_dn9, locals.var_ninvdehres_dn10, locals.var_ninvdehres_dn11, locals.var_ninvdehres_dn14,)
    }
};
        locals.var_ninvdehres = assign17950_e12457;
        locals.var_ninvdehres_dn0 = assign17950_e12457_d_n0;
        locals.var_ninvdehres_dn2 = assign17950_e12457_d_n2;
        locals.var_ninvdehres_dn4 = assign17950_e12457_d_n4;
        locals.var_ninvdehres_dn5 = assign17950_e12457_d_n5;
        locals.var_ninvdehres_dn6 = assign17950_e12457_d_n6;
        locals.var_ninvdehres_dn7 = assign17950_e12457_d_n7;
        locals.var_ninvdehres_dn8 = assign17950_e12457_d_n8;
        locals.var_ninvdehres_dn9 = assign17950_e12457_d_n9;
        locals.var_ninvdehres_dn10 = assign17950_e12457_d_n10;
        locals.var_ninvdehres_dn11 = assign17950_e12457_d_n11;
        locals.var_ninvdehres_dn14 = assign17950_e12457_d_n14;

        let (assign17960_e12473, assign17960_e12473_d_n0, assign17960_e12473_d_n2, assign17960_e12473_d_n4, assign17960_e12473_d_n5, assign17960_e12473_d_n6, assign17960_e12473_d_n7, assign17960_e12473_d_n8, assign17960_e12473_d_n9, assign17960_e12473_d_n10, assign17960_e12473_d_n11, assign17960_e12473_d_n14,) = {
    if ((locals.var_guard356 != 0.0) && (p.p53 != 0.0)) {
        let assign17960_e12464: f64 = (p.p328 * locals.var_tdiff0);
        let assign17960_e12465: f64 = (locals.var_uc_rth0 + assign17960_e12464);
        let assign17960_e12468: f64 = (p.p329 * locals.var_tdiff0_2);
        let assign17960_e12469: f64 = (assign17960_e12465 + assign17960_e12468);
        let assign17960_e12471: f64 = (assign17960_e12469 * locals.var_rthtemp0);
        (assign17960_e12471, (((p.p328 * locals.var_tdiff0_dn0) + (p.p329 * locals.var_tdiff0_2_dn0)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn2) + (p.p329 * locals.var_tdiff0_2_dn2)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn4) + (p.p329 * locals.var_tdiff0_2_dn4)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn5) + (p.p329 * locals.var_tdiff0_2_dn5)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn6) + (p.p329 * locals.var_tdiff0_2_dn6)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn7) + (p.p329 * locals.var_tdiff0_2_dn7)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn8) + (p.p329 * locals.var_tdiff0_2_dn8)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn9) + (p.p329 * locals.var_tdiff0_2_dn9)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn10) + (p.p329 * locals.var_tdiff0_2_dn10)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn11) + (p.p329 * locals.var_tdiff0_2_dn11)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn14) + (p.p329 * locals.var_tdiff0_2_dn14)) * locals.var_rthtemp0),)
    } else {
        (locals.var_rth, locals.var_rth_dn0, locals.var_rth_dn2, locals.var_rth_dn4, locals.var_rth_dn5, locals.var_rth_dn6, locals.var_rth_dn7, locals.var_rth_dn8, locals.var_rth_dn9, locals.var_rth_dn10, locals.var_rth_dn11, locals.var_rth_dn14,)
    }
};
        locals.var_rth = assign17960_e12473;
        locals.var_rth_dn0 = assign17960_e12473_d_n0;
        locals.var_rth_dn2 = assign17960_e12473_d_n2;
        locals.var_rth_dn4 = assign17960_e12473_d_n4;
        locals.var_rth_dn5 = assign17960_e12473_d_n5;
        locals.var_rth_dn6 = assign17960_e12473_d_n6;
        locals.var_rth_dn7 = assign17960_e12473_d_n7;
        locals.var_rth_dn8 = assign17960_e12473_d_n8;
        locals.var_rth_dn9 = assign17960_e12473_d_n9;
        locals.var_rth_dn10 = assign17960_e12473_d_n10;
        locals.var_rth_dn11 = assign17960_e12473_d_n11;
        locals.var_rth_dn14 = assign17960_e12473_d_n14;

        let assign17980_e12481: f64 = if locals.var_rth < 0.0001 { 1.0 } else { 0.0 };
        locals.var_guard375 = assign17980_e12481;

        let (assign17990_e12489, assign17990_e12489_d_n0, assign17990_e12489_d_n2, assign17990_e12489_d_n4, assign17990_e12489_d_n5, assign17990_e12489_d_n6, assign17990_e12489_d_n7, assign17990_e12489_d_n8, assign17990_e12489_d_n9, assign17990_e12489_d_n10, assign17990_e12489_d_n11, assign17990_e12489_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (p.p53 != 0.0)) && (locals.var_guard375 != 0.0)) {
        (0.0001, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rth, locals.var_rth_dn0, locals.var_rth_dn2, locals.var_rth_dn4, locals.var_rth_dn5, locals.var_rth_dn6, locals.var_rth_dn7, locals.var_rth_dn8, locals.var_rth_dn9, locals.var_rth_dn10, locals.var_rth_dn11, locals.var_rth_dn14,)
    }
};
        locals.var_rth = assign17990_e12489;
        locals.var_rth_dn0 = assign17990_e12489_d_n0;
        locals.var_rth_dn2 = assign17990_e12489_d_n2;
        locals.var_rth_dn4 = assign17990_e12489_d_n4;
        locals.var_rth_dn5 = assign17990_e12489_d_n5;
        locals.var_rth_dn6 = assign17990_e12489_d_n6;
        locals.var_rth_dn7 = assign17990_e12489_d_n7;
        locals.var_rth_dn8 = assign17990_e12489_d_n8;
        locals.var_rth_dn9 = assign17990_e12489_d_n9;
        locals.var_rth_dn10 = assign17990_e12489_d_n10;
        locals.var_rth_dn11 = assign17990_e12489_d_n11;
        locals.var_rth_dn14 = assign17990_e12489_d_n14;

        let (assign18000_e12501, assign18000_e12501_d_n0, assign18000_e12501_d_n2, assign18000_e12501_d_n4, assign18000_e12501_d_n5, assign18000_e12501_d_n6, assign18000_e12501_d_n7, assign18000_e12501_d_n8, assign18000_e12501_d_n9, assign18000_e12501_d_n10, assign18000_e12501_d_n11, assign18000_e12501_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign18000_e12494: f64 = (p.p330 * locals.var_tdiff0);
        let assign18000_e12495: f64 = (locals.var_uc_powrat + assign18000_e12494);
        let assign18000_e12498: f64 = (p.p331 * locals.var_tdiff0_2);
        let assign18000_e12499: f64 = (assign18000_e12495 + assign18000_e12498);
        (assign18000_e12499, ((p.p330 * locals.var_tdiff0_dn0) + (p.p331 * locals.var_tdiff0_2_dn0)), ((p.p330 * locals.var_tdiff0_dn2) + (p.p331 * locals.var_tdiff0_2_dn2)), ((p.p330 * locals.var_tdiff0_dn4) + (p.p331 * locals.var_tdiff0_2_dn4)), ((p.p330 * locals.var_tdiff0_dn5) + (p.p331 * locals.var_tdiff0_2_dn5)), ((p.p330 * locals.var_tdiff0_dn6) + (p.p331 * locals.var_tdiff0_2_dn6)), ((p.p330 * locals.var_tdiff0_dn7) + (p.p331 * locals.var_tdiff0_2_dn7)), ((p.p330 * locals.var_tdiff0_dn8) + (p.p331 * locals.var_tdiff0_2_dn8)), ((p.p330 * locals.var_tdiff0_dn9) + (p.p331 * locals.var_tdiff0_2_dn9)), ((p.p330 * locals.var_tdiff0_dn10) + (p.p331 * locals.var_tdiff0_2_dn10)), ((p.p330 * locals.var_tdiff0_dn11) + (p.p331 * locals.var_tdiff0_2_dn11)), ((p.p330 * locals.var_tdiff0_dn14) + (p.p331 * locals.var_tdiff0_2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign18000_e12501;
        locals.var_t2_dn0 = assign18000_e12501_d_n0;
        locals.var_t2_dn2 = assign18000_e12501_d_n2;
        locals.var_t2_dn4 = assign18000_e12501_d_n4;
        locals.var_t2_dn5 = assign18000_e12501_d_n5;
        locals.var_t2_dn6 = assign18000_e12501_d_n6;
        locals.var_t2_dn7 = assign18000_e12501_d_n7;
        locals.var_t2_dn8 = assign18000_e12501_d_n8;
        locals.var_t2_dn9 = assign18000_e12501_d_n9;
        locals.var_t2_dn10 = assign18000_e12501_d_n10;
        locals.var_t2_dn11 = assign18000_e12501_d_n11;
        locals.var_t2_dn14 = assign18000_e12501_d_n14;

        let (assign18010_e12509, assign18010_e12509_d_n0, assign18010_e12509_d_n2, assign18010_e12509_d_n4, assign18010_e12509_d_n5, assign18010_e12509_d_n6, assign18010_e12509_d_n7, assign18010_e12509_d_n8, assign18010_e12509_d_n9, assign18010_e12509_d_n10, assign18010_e12509_d_n11, assign18010_e12509_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign18010_e12505: f64 = locals.var_t2;
        let assign18010_e12507: f64 = (assign18010_e12505 - 0.05);
        (assign18010_e12507, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign18010_e12509;
        locals.var_tmf1_dn0 = assign18010_e12509_d_n0;
        locals.var_tmf1_dn2 = assign18010_e12509_d_n2;
        locals.var_tmf1_dn4 = assign18010_e12509_d_n4;
        locals.var_tmf1_dn5 = assign18010_e12509_d_n5;
        locals.var_tmf1_dn6 = assign18010_e12509_d_n6;
        locals.var_tmf1_dn7 = assign18010_e12509_d_n7;
        locals.var_tmf1_dn8 = assign18010_e12509_d_n8;
        locals.var_tmf1_dn9 = assign18010_e12509_d_n9;
        locals.var_tmf1_dn10 = assign18010_e12509_d_n10;
        locals.var_tmf1_dn11 = assign18010_e12509_d_n11;
        locals.var_tmf1_dn14 = assign18010_e12509_d_n14;

        let (assign18020_e12517, assign18020_e12517_d_n0, assign18020_e12517_d_n2, assign18020_e12517_d_n4, assign18020_e12517_d_n5, assign18020_e12517_d_n6, assign18020_e12517_d_n7, assign18020_e12517_d_n8, assign18020_e12517_d_n9, assign18020_e12517_d_n10, assign18020_e12517_d_n11, assign18020_e12517_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18020_e12517;
        locals.var_tmf2_dn0 = assign18020_e12517_d_n0;
        locals.var_tmf2_dn2 = assign18020_e12517_d_n2;
        locals.var_tmf2_dn4 = assign18020_e12517_d_n4;
        locals.var_tmf2_dn5 = assign18020_e12517_d_n5;
        locals.var_tmf2_dn6 = assign18020_e12517_d_n6;
        locals.var_tmf2_dn7 = assign18020_e12517_d_n7;
        locals.var_tmf2_dn8 = assign18020_e12517_d_n8;
        locals.var_tmf2_dn9 = assign18020_e12517_d_n9;
        locals.var_tmf2_dn10 = assign18020_e12517_d_n10;
        locals.var_tmf2_dn11 = assign18020_e12517_d_n11;
        locals.var_tmf2_dn14 = assign18020_e12517_d_n14;

        let (assign18030_e12527, assign18030_e12527_d_n0, assign18030_e12527_d_n2, assign18030_e12527_d_n4, assign18030_e12527_d_n5, assign18030_e12527_d_n6, assign18030_e12527_d_n7, assign18030_e12527_d_n8, assign18030_e12527_d_n9, assign18030_e12527_d_n10, assign18030_e12527_d_n11, assign18030_e12527_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let (assign18030_e12525, assign18030_e12525_d_n0, assign18030_e12525_d_n2, assign18030_e12525_d_n4, assign18030_e12525_d_n5, assign18030_e12525_d_n6, assign18030_e12525_d_n7, assign18030_e12525_d_n8, assign18030_e12525_d_n9, assign18030_e12525_d_n10, assign18030_e12525_d_n11, assign18030_e12525_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign18030_e12524: f64 = (-locals.var_tmf2);
                (assign18030_e12524, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign18030_e12525, assign18030_e12525_d_n0, assign18030_e12525_d_n2, assign18030_e12525_d_n4, assign18030_e12525_d_n5, assign18030_e12525_d_n6, assign18030_e12525_d_n7, assign18030_e12525_d_n8, assign18030_e12525_d_n9, assign18030_e12525_d_n10, assign18030_e12525_d_n11, assign18030_e12525_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18030_e12527;
        locals.var_tmf2_dn0 = assign18030_e12527_d_n0;
        locals.var_tmf2_dn2 = assign18030_e12527_d_n2;
        locals.var_tmf2_dn4 = assign18030_e12527_d_n4;
        locals.var_tmf2_dn5 = assign18030_e12527_d_n5;
        locals.var_tmf2_dn6 = assign18030_e12527_d_n6;
        locals.var_tmf2_dn7 = assign18030_e12527_d_n7;
        locals.var_tmf2_dn8 = assign18030_e12527_d_n8;
        locals.var_tmf2_dn9 = assign18030_e12527_d_n9;
        locals.var_tmf2_dn10 = assign18030_e12527_d_n10;
        locals.var_tmf2_dn11 = assign18030_e12527_d_n11;
        locals.var_tmf2_dn14 = assign18030_e12527_d_n14;

    }

    pub(super) fn stamp_transient_block_40(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign18040_e12536, assign18040_e12536_d_n0, assign18040_e12536_d_n2, assign18040_e12536_d_n4, assign18040_e12536_d_n5, assign18040_e12536_d_n6, assign18040_e12536_d_n7, assign18040_e12536_d_n8, assign18040_e12536_d_n9, assign18040_e12536_d_n10, assign18040_e12536_d_n11, assign18040_e12536_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign18040_e12531: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18040_e12533: f64 = (assign18040_e12531 + locals.var_tmf2);
        let assign18040_e12534: f64 = (assign18040_e12533).sqrt();
        (assign18040_e12534, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18040_e12534)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18040_e12534)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign18040_e12534)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign18040_e12534)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign18040_e12534)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign18040_e12534)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign18040_e12534)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign18040_e12534)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign18040_e12534)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign18040_e12534)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign18040_e12534)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18040_e12536;
        locals.var_tmf2_dn0 = assign18040_e12536_d_n0;
        locals.var_tmf2_dn2 = assign18040_e12536_d_n2;
        locals.var_tmf2_dn4 = assign18040_e12536_d_n4;
        locals.var_tmf2_dn5 = assign18040_e12536_d_n5;
        locals.var_tmf2_dn6 = assign18040_e12536_d_n6;
        locals.var_tmf2_dn7 = assign18040_e12536_d_n7;
        locals.var_tmf2_dn8 = assign18040_e12536_d_n8;
        locals.var_tmf2_dn9 = assign18040_e12536_d_n9;
        locals.var_tmf2_dn10 = assign18040_e12536_d_n10;
        locals.var_tmf2_dn11 = assign18040_e12536_d_n11;
        locals.var_tmf2_dn14 = assign18040_e12536_d_n14;

        let (assign18050_e12546, assign18050_e12546_d_n0, assign18050_e12546_d_n2, assign18050_e12546_d_n4, assign18050_e12546_d_n5, assign18050_e12546_d_n6, assign18050_e12546_d_n7, assign18050_e12546_d_n8, assign18050_e12546_d_n9, assign18050_e12546_d_n10, assign18050_e12546_d_n11, assign18050_e12546_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign18050_e12542: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign18050_e12543: f64 = (1.0 + assign18050_e12542);
        let assign18050_e12544: f64 = (0.5 * assign18050_e12543);
        (assign18050_e12544, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign18050_e12546;
        locals.var_t0_dn0 = assign18050_e12546_d_n0;
        locals.var_t0_dn2 = assign18050_e12546_d_n2;
        locals.var_t0_dn4 = assign18050_e12546_d_n4;
        locals.var_t0_dn5 = assign18050_e12546_d_n5;
        locals.var_t0_dn6 = assign18050_e12546_d_n6;
        locals.var_t0_dn7 = assign18050_e12546_d_n7;
        locals.var_t0_dn8 = assign18050_e12546_d_n8;
        locals.var_t0_dn9 = assign18050_e12546_d_n9;
        locals.var_t0_dn10 = assign18050_e12546_d_n10;
        locals.var_t0_dn11 = assign18050_e12546_d_n11;
        locals.var_t0_dn14 = assign18050_e12546_d_n14;

        let (assign18060_e12556, assign18060_e12556_d_n0, assign18060_e12556_d_n2, assign18060_e12556_d_n4, assign18060_e12556_d_n5, assign18060_e12556_d_n6, assign18060_e12556_d_n7, assign18060_e12556_d_n8, assign18060_e12556_d_n9, assign18060_e12556_d_n10, assign18060_e12556_d_n11, assign18060_e12556_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign18060_e12552: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign18060_e12553: f64 = (0.5 * assign18060_e12552);
        let assign18060_e12554: f64 = assign18060_e12553;
        (assign18060_e12554, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign18060_e12556;
        locals.var_t2_dn0 = assign18060_e12556_d_n0;
        locals.var_t2_dn2 = assign18060_e12556_d_n2;
        locals.var_t2_dn4 = assign18060_e12556_d_n4;
        locals.var_t2_dn5 = assign18060_e12556_d_n5;
        locals.var_t2_dn6 = assign18060_e12556_d_n6;
        locals.var_t2_dn7 = assign18060_e12556_d_n7;
        locals.var_t2_dn8 = assign18060_e12556_d_n8;
        locals.var_t2_dn9 = assign18060_e12556_d_n9;
        locals.var_t2_dn10 = assign18060_e12556_d_n10;
        locals.var_t2_dn11 = assign18060_e12556_d_n11;
        locals.var_t2_dn14 = assign18060_e12556_d_n14;

        let (assign18070_e12564, assign18070_e12564_d_n0, assign18070_e12564_d_n2, assign18070_e12564_d_n4, assign18070_e12564_d_n5, assign18070_e12564_d_n6, assign18070_e12564_d_n7, assign18070_e12564_d_n8, assign18070_e12564_d_n9, assign18070_e12564_d_n10, assign18070_e12564_d_n11, assign18070_e12564_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign18070_e12560: f64 = (1.0 - locals.var_t2);
        let assign18070_e12562: f64 = (assign18070_e12560 - 0.05);
        (assign18070_e12562, (-locals.var_t2_dn0), (-locals.var_t2_dn2), (-locals.var_t2_dn4), (-locals.var_t2_dn5), (-locals.var_t2_dn6), (-locals.var_t2_dn7), (-locals.var_t2_dn8), (-locals.var_t2_dn9), (-locals.var_t2_dn10), (-locals.var_t2_dn11), (-locals.var_t2_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign18070_e12564;
        locals.var_tmf1_dn0 = assign18070_e12564_d_n0;
        locals.var_tmf1_dn2 = assign18070_e12564_d_n2;
        locals.var_tmf1_dn4 = assign18070_e12564_d_n4;
        locals.var_tmf1_dn5 = assign18070_e12564_d_n5;
        locals.var_tmf1_dn6 = assign18070_e12564_d_n6;
        locals.var_tmf1_dn7 = assign18070_e12564_d_n7;
        locals.var_tmf1_dn8 = assign18070_e12564_d_n8;
        locals.var_tmf1_dn9 = assign18070_e12564_d_n9;
        locals.var_tmf1_dn10 = assign18070_e12564_d_n10;
        locals.var_tmf1_dn11 = assign18070_e12564_d_n11;
        locals.var_tmf1_dn14 = assign18070_e12564_d_n14;

        let (assign18080_e12572, assign18080_e12572_d_n0, assign18080_e12572_d_n2, assign18080_e12572_d_n4, assign18080_e12572_d_n5, assign18080_e12572_d_n6, assign18080_e12572_d_n7, assign18080_e12572_d_n8, assign18080_e12572_d_n9, assign18080_e12572_d_n10, assign18080_e12572_d_n11, assign18080_e12572_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign18080_e12568: f64 = 4.0;
        let assign18080_e12570: f64 = (assign18080_e12568 * 0.05);
        (assign18080_e12570, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18080_e12572;
        locals.var_tmf2_dn0 = assign18080_e12572_d_n0;
        locals.var_tmf2_dn2 = assign18080_e12572_d_n2;
        locals.var_tmf2_dn4 = assign18080_e12572_d_n4;
        locals.var_tmf2_dn5 = assign18080_e12572_d_n5;
        locals.var_tmf2_dn6 = assign18080_e12572_d_n6;
        locals.var_tmf2_dn7 = assign18080_e12572_d_n7;
        locals.var_tmf2_dn8 = assign18080_e12572_d_n8;
        locals.var_tmf2_dn9 = assign18080_e12572_d_n9;
        locals.var_tmf2_dn10 = assign18080_e12572_d_n10;
        locals.var_tmf2_dn11 = assign18080_e12572_d_n11;
        locals.var_tmf2_dn14 = assign18080_e12572_d_n14;

        let (assign18090_e12582, assign18090_e12582_d_n0, assign18090_e12582_d_n2, assign18090_e12582_d_n4, assign18090_e12582_d_n5, assign18090_e12582_d_n6, assign18090_e12582_d_n7, assign18090_e12582_d_n8, assign18090_e12582_d_n9, assign18090_e12582_d_n10, assign18090_e12582_d_n11, assign18090_e12582_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let (assign18090_e12580, assign18090_e12580_d_n0, assign18090_e12580_d_n2, assign18090_e12580_d_n4, assign18090_e12580_d_n5, assign18090_e12580_d_n6, assign18090_e12580_d_n7, assign18090_e12580_d_n8, assign18090_e12580_d_n9, assign18090_e12580_d_n10, assign18090_e12580_d_n11, assign18090_e12580_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign18090_e12579: f64 = (-locals.var_tmf2);
                (assign18090_e12579, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign18090_e12580, assign18090_e12580_d_n0, assign18090_e12580_d_n2, assign18090_e12580_d_n4, assign18090_e12580_d_n5, assign18090_e12580_d_n6, assign18090_e12580_d_n7, assign18090_e12580_d_n8, assign18090_e12580_d_n9, assign18090_e12580_d_n10, assign18090_e12580_d_n11, assign18090_e12580_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18090_e12582;
        locals.var_tmf2_dn0 = assign18090_e12582_d_n0;
        locals.var_tmf2_dn2 = assign18090_e12582_d_n2;
        locals.var_tmf2_dn4 = assign18090_e12582_d_n4;
        locals.var_tmf2_dn5 = assign18090_e12582_d_n5;
        locals.var_tmf2_dn6 = assign18090_e12582_d_n6;
        locals.var_tmf2_dn7 = assign18090_e12582_d_n7;
        locals.var_tmf2_dn8 = assign18090_e12582_d_n8;
        locals.var_tmf2_dn9 = assign18090_e12582_d_n9;
        locals.var_tmf2_dn10 = assign18090_e12582_d_n10;
        locals.var_tmf2_dn11 = assign18090_e12582_d_n11;
        locals.var_tmf2_dn14 = assign18090_e12582_d_n14;

        let (assign18100_e12591, assign18100_e12591_d_n0, assign18100_e12591_d_n2, assign18100_e12591_d_n4, assign18100_e12591_d_n5, assign18100_e12591_d_n6, assign18100_e12591_d_n7, assign18100_e12591_d_n8, assign18100_e12591_d_n9, assign18100_e12591_d_n10, assign18100_e12591_d_n11, assign18100_e12591_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign18100_e12586: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18100_e12588: f64 = (assign18100_e12586 + locals.var_tmf2);
        let assign18100_e12589: f64 = (assign18100_e12588).sqrt();
        (assign18100_e12589, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18100_e12589)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18100_e12589)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign18100_e12589)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign18100_e12589)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign18100_e12589)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign18100_e12589)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign18100_e12589)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign18100_e12589)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign18100_e12589)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign18100_e12589)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign18100_e12589)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18100_e12591;
        locals.var_tmf2_dn0 = assign18100_e12591_d_n0;
        locals.var_tmf2_dn2 = assign18100_e12591_d_n2;
        locals.var_tmf2_dn4 = assign18100_e12591_d_n4;
        locals.var_tmf2_dn5 = assign18100_e12591_d_n5;
        locals.var_tmf2_dn6 = assign18100_e12591_d_n6;
        locals.var_tmf2_dn7 = assign18100_e12591_d_n7;
        locals.var_tmf2_dn8 = assign18100_e12591_d_n8;
        locals.var_tmf2_dn9 = assign18100_e12591_d_n9;
        locals.var_tmf2_dn10 = assign18100_e12591_d_n10;
        locals.var_tmf2_dn11 = assign18100_e12591_d_n11;
        locals.var_tmf2_dn14 = assign18100_e12591_d_n14;

        let (assign18110_e12601, assign18110_e12601_d_n0, assign18110_e12601_d_n2, assign18110_e12601_d_n4, assign18110_e12601_d_n5, assign18110_e12601_d_n6, assign18110_e12601_d_n7, assign18110_e12601_d_n8, assign18110_e12601_d_n9, assign18110_e12601_d_n10, assign18110_e12601_d_n11, assign18110_e12601_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign18110_e12597: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign18110_e12598: f64 = (1.0 + assign18110_e12597);
        let assign18110_e12599: f64 = (0.5 * assign18110_e12598);
        (assign18110_e12599, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign18110_e12601;
        locals.var_t0_dn0 = assign18110_e12601_d_n0;
        locals.var_t0_dn2 = assign18110_e12601_d_n2;
        locals.var_t0_dn4 = assign18110_e12601_d_n4;
        locals.var_t0_dn5 = assign18110_e12601_d_n5;
        locals.var_t0_dn6 = assign18110_e12601_d_n6;
        locals.var_t0_dn7 = assign18110_e12601_d_n7;
        locals.var_t0_dn8 = assign18110_e12601_d_n8;
        locals.var_t0_dn9 = assign18110_e12601_d_n9;
        locals.var_t0_dn10 = assign18110_e12601_d_n10;
        locals.var_t0_dn11 = assign18110_e12601_d_n11;
        locals.var_t0_dn14 = assign18110_e12601_d_n14;

        let (assign18120_e12611, assign18120_e12611_d_n0, assign18120_e12611_d_n2, assign18120_e12611_d_n4, assign18120_e12611_d_n5, assign18120_e12611_d_n6, assign18120_e12611_d_n7, assign18120_e12611_d_n8, assign18120_e12611_d_n9, assign18120_e12611_d_n10, assign18120_e12611_d_n11, assign18120_e12611_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign18120_e12607: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign18120_e12608: f64 = (0.5 * assign18120_e12607);
        let assign18120_e12609: f64 = (1.0 - assign18120_e12608);
        (assign18120_e12609, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (-(0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_powratio, locals.var_powratio_dn0, locals.var_powratio_dn2, locals.var_powratio_dn4, locals.var_powratio_dn5, locals.var_powratio_dn6, locals.var_powratio_dn7, locals.var_powratio_dn8, locals.var_powratio_dn9, locals.var_powratio_dn10, locals.var_powratio_dn11, locals.var_powratio_dn14,)
    }
};
        locals.var_powratio = assign18120_e12611;
        locals.var_powratio_dn0 = assign18120_e12611_d_n0;
        locals.var_powratio_dn2 = assign18120_e12611_d_n2;
        locals.var_powratio_dn4 = assign18120_e12611_d_n4;
        locals.var_powratio_dn5 = assign18120_e12611_d_n5;
        locals.var_powratio_dn6 = assign18120_e12611_d_n6;
        locals.var_powratio_dn7 = assign18120_e12611_d_n7;
        locals.var_powratio_dn8 = assign18120_e12611_d_n8;
        locals.var_powratio_dn9 = assign18120_e12611_d_n9;
        locals.var_powratio_dn10 = assign18120_e12611_d_n10;
        locals.var_powratio_dn11 = assign18120_e12611_d_n11;
        locals.var_powratio_dn14 = assign18120_e12611_d_n14;

        let (assign18130_e12622, assign18130_e12622_d_n0, assign18130_e12622_d_n2, assign18130_e12622_d_n4, assign18130_e12622_d_n5, assign18130_e12622_d_n6, assign18130_e12622_d_n7, assign18130_e12622_d_n8, assign18130_e12622_d_n9, assign18130_e12622_d_n10, assign18130_e12622_d_n11, assign18130_e12622_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign18130_e12615: f64 = (2.0 * locals.var_beta_inv);
        let assign18130_e12618: f64 = (locals.var_nsub / locals.var_nin);
        let assign18130_e12619: f64 = (assign18130_e12618).ln();
        let assign18130_e12620: f64 = (assign18130_e12615 * assign18130_e12619);
        (assign18130_e12620, (((2.0 * locals.var_beta_inv_dn0) * assign18130_e12619) + (assign18130_e12615 * ((((locals.var_nsub_dn0 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign18130_e12618))), (((2.0 * locals.var_beta_inv_dn2) * assign18130_e12619) + (assign18130_e12615 * ((((locals.var_nsub_dn2 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign18130_e12618))), (((2.0 * locals.var_beta_inv_dn4) * assign18130_e12619) + (assign18130_e12615 * ((((locals.var_nsub_dn4 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign18130_e12618))), (((2.0 * locals.var_beta_inv_dn5) * assign18130_e12619) + (assign18130_e12615 * ((((locals.var_nsub_dn5 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign18130_e12618))), (((2.0 * locals.var_beta_inv_dn6) * assign18130_e12619) + (assign18130_e12615 * ((((locals.var_nsub_dn6 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign18130_e12618))), (((2.0 * locals.var_beta_inv_dn7) * assign18130_e12619) + (assign18130_e12615 * ((((locals.var_nsub_dn7 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign18130_e12618))), (((2.0 * locals.var_beta_inv_dn8) * assign18130_e12619) + (assign18130_e12615 * ((((locals.var_nsub_dn8 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign18130_e12618))), (((2.0 * locals.var_beta_inv_dn9) * assign18130_e12619) + (assign18130_e12615 * ((((locals.var_nsub_dn9 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign18130_e12618))), (((2.0 * locals.var_beta_inv_dn10) * assign18130_e12619) + (assign18130_e12615 * ((((locals.var_nsub_dn10 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign18130_e12618))), (((2.0 * locals.var_beta_inv_dn11) * assign18130_e12619) + (assign18130_e12615 * ((((locals.var_nsub_dn11 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)) / assign18130_e12618))), (((2.0 * locals.var_beta_inv_dn14) * assign18130_e12619) + (assign18130_e12615 * ((((locals.var_nsub_dn14 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn14)) / (locals.var_nin * locals.var_nin)) / assign18130_e12618))),)
    } else {
        (locals.var_pb2, locals.var_pb2_dn0, locals.var_pb2_dn2, locals.var_pb2_dn4, locals.var_pb2_dn5, locals.var_pb2_dn6, locals.var_pb2_dn7, locals.var_pb2_dn8, locals.var_pb2_dn9, locals.var_pb2_dn10, locals.var_pb2_dn11, locals.var_pb2_dn14,)
    }
};
        locals.var_pb2 = assign18130_e12622;
        locals.var_pb2_dn0 = assign18130_e12622_d_n0;
        locals.var_pb2_dn2 = assign18130_e12622_d_n2;
        locals.var_pb2_dn4 = assign18130_e12622_d_n4;
        locals.var_pb2_dn5 = assign18130_e12622_d_n5;
        locals.var_pb2_dn6 = assign18130_e12622_d_n6;
        locals.var_pb2_dn7 = assign18130_e12622_d_n7;
        locals.var_pb2_dn8 = assign18130_e12622_d_n8;
        locals.var_pb2_dn9 = assign18130_e12622_d_n9;
        locals.var_pb2_dn10 = assign18130_e12622_d_n10;
        locals.var_pb2_dn11 = assign18130_e12622_d_n11;
        locals.var_pb2_dn14 = assign18130_e12622_d_n14;

        let (assign18140_e12630, assign18140_e12630_d_n0, assign18140_e12630_d_n2, assign18140_e12630_d_n4, assign18140_e12630_d_n5, assign18140_e12630_d_n6, assign18140_e12630_d_n7, assign18140_e12630_d_n8, assign18140_e12630_d_n9, assign18140_e12630_d_n10, assign18140_e12630_d_n11, assign18140_e12630_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign18140_e12626: f64 = (2.0 * 1.034943e-10);
        let assign18140_e12628: f64 = (assign18140_e12626 / 1.6021918e-19);
        (assign18140_e12628, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign18140_e12630;
        locals.var_t1_dn0 = assign18140_e12630_d_n0;
        locals.var_t1_dn2 = assign18140_e12630_d_n2;
        locals.var_t1_dn4 = assign18140_e12630_d_n4;
        locals.var_t1_dn5 = assign18140_e12630_d_n5;
        locals.var_t1_dn6 = assign18140_e12630_d_n6;
        locals.var_t1_dn7 = assign18140_e12630_d_n7;
        locals.var_t1_dn8 = assign18140_e12630_d_n8;
        locals.var_t1_dn9 = assign18140_e12630_d_n9;
        locals.var_t1_dn10 = assign18140_e12630_d_n10;
        locals.var_t1_dn11 = assign18140_e12630_d_n11;
        locals.var_t1_dn14 = assign18140_e12630_d_n14;

        let (assign18150_e12637, assign18150_e12637_d_n0, assign18150_e12637_d_n2, assign18150_e12637_d_n4, assign18150_e12637_d_n5, assign18150_e12637_d_n6, assign18150_e12637_d_n7, assign18150_e12637_d_n8, assign18150_e12637_d_n9, assign18150_e12637_d_n10, assign18150_e12637_d_n11, assign18150_e12637_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign18150_e12634: f64 = (locals.var_t1 / locals.var_nsub);
        let assign18150_e12635: f64 = (assign18150_e12634).sqrt();
        (assign18150_e12635, ((((locals.var_t1_dn0 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn0)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18150_e12635)), ((((locals.var_t1_dn2 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn2)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18150_e12635)), ((((locals.var_t1_dn4 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn4)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18150_e12635)), ((((locals.var_t1_dn5 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn5)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18150_e12635)), ((((locals.var_t1_dn6 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn6)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18150_e12635)), ((((locals.var_t1_dn7 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn7)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18150_e12635)), ((((locals.var_t1_dn8 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn8)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18150_e12635)), ((((locals.var_t1_dn9 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn9)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18150_e12635)), ((((locals.var_t1_dn10 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn10)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18150_e12635)), ((((locals.var_t1_dn11 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn11)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18150_e12635)), ((((locals.var_t1_dn14 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn14)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18150_e12635)),)
    } else {
        (locals.var_wdpl, locals.var_wdpl_dn0, locals.var_wdpl_dn2, locals.var_wdpl_dn4, locals.var_wdpl_dn5, locals.var_wdpl_dn6, locals.var_wdpl_dn7, locals.var_wdpl_dn8, locals.var_wdpl_dn9, locals.var_wdpl_dn10, locals.var_wdpl_dn11, locals.var_wdpl_dn14,)
    }
};
        locals.var_wdpl = assign18150_e12637;
        locals.var_wdpl_dn0 = assign18150_e12637_d_n0;
        locals.var_wdpl_dn2 = assign18150_e12637_d_n2;
        locals.var_wdpl_dn4 = assign18150_e12637_d_n4;
        locals.var_wdpl_dn5 = assign18150_e12637_d_n5;
        locals.var_wdpl_dn6 = assign18150_e12637_d_n6;
        locals.var_wdpl_dn7 = assign18150_e12637_d_n7;
        locals.var_wdpl_dn8 = assign18150_e12637_d_n8;
        locals.var_wdpl_dn9 = assign18150_e12637_d_n9;
        locals.var_wdpl_dn10 = assign18150_e12637_d_n10;
        locals.var_wdpl_dn11 = assign18150_e12637_d_n11;
        locals.var_wdpl_dn14 = assign18150_e12637_d_n14;

        let (assign18160_e12644, assign18160_e12644_d_n0, assign18160_e12644_d_n2, assign18160_e12644_d_n4, assign18160_e12644_d_n5, assign18160_e12644_d_n6, assign18160_e12644_d_n7, assign18160_e12644_d_n8, assign18160_e12644_d_n9, assign18160_e12644_d_n10, assign18160_e12644_d_n11, assign18160_e12644_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign18160_e12641: f64 = (locals.var_t1 / locals.var_ef_nsubp);
        let assign18160_e12642: f64 = (assign18160_e12641).sqrt();
        (assign18160_e12642, ((((locals.var_t1_dn0 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn0)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18160_e12642)), ((((locals.var_t1_dn2 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn2)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18160_e12642)), ((((locals.var_t1_dn4 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn4)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18160_e12642)), ((((locals.var_t1_dn5 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn5)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18160_e12642)), ((((locals.var_t1_dn6 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn6)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18160_e12642)), ((((locals.var_t1_dn7 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn7)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18160_e12642)), ((((locals.var_t1_dn8 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn8)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18160_e12642)), ((((locals.var_t1_dn9 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn9)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18160_e12642)), ((((locals.var_t1_dn10 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn10)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18160_e12642)), ((((locals.var_t1_dn11 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn11)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18160_e12642)), ((((locals.var_t1_dn14 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn14)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18160_e12642)),)
    } else {
        (locals.var_wdplp, locals.var_wdplp_dn0, locals.var_wdplp_dn2, locals.var_wdplp_dn4, locals.var_wdplp_dn5, locals.var_wdplp_dn6, locals.var_wdplp_dn7, locals.var_wdplp_dn8, locals.var_wdplp_dn9, locals.var_wdplp_dn10, locals.var_wdplp_dn11, locals.var_wdplp_dn14,)
    }
};
        locals.var_wdplp = assign18160_e12644;
        locals.var_wdplp_dn0 = assign18160_e12644_d_n0;
        locals.var_wdplp_dn2 = assign18160_e12644_d_n2;
        locals.var_wdplp_dn4 = assign18160_e12644_d_n4;
        locals.var_wdplp_dn5 = assign18160_e12644_d_n5;
        locals.var_wdplp_dn6 = assign18160_e12644_d_n6;
        locals.var_wdplp_dn7 = assign18160_e12644_d_n7;
        locals.var_wdplp_dn8 = assign18160_e12644_d_n8;
        locals.var_wdplp_dn9 = assign18160_e12644_d_n9;
        locals.var_wdplp_dn10 = assign18160_e12644_d_n10;
        locals.var_wdplp_dn11 = assign18160_e12644_d_n11;
        locals.var_wdplp_dn14 = assign18160_e12644_d_n14;

        let assign18170_e12647: f64 = if locals.var_uc_codep == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard376 = assign18170_e12647;

        let (assign18180_e12662, assign18180_e12662_d_n0, assign18180_e12662_d_n2, assign18180_e12662_d_n4, assign18180_e12662_d_n5, assign18180_e12662_d_n6, assign18180_e12662_d_n7, assign18180_e12662_d_n8, assign18180_e12662_d_n9, assign18180_e12662_d_n10, assign18180_e12662_d_n11, assign18180_e12662_d_n14,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard376 != 0.0)) {
        let assign18180_e12653: f64 = (2.0 * 1.034943e-10);
        let assign18180_e12655: f64 = (assign18180_e12653 * 1.6021918e-19);
        let assign18180_e12657: f64 = (assign18180_e12655 * locals.var_nsub);
        let assign18180_e12659: f64 = (assign18180_e12657 * locals.var_beta_inv);
        let assign18180_e12660: f64 = (assign18180_e12659).sqrt();
        (assign18180_e12660, ((((assign18180_e12655 * locals.var_nsub_dn0) * locals.var_beta_inv) + (assign18180_e12657 * locals.var_beta_inv_dn0)) / (2.0 * assign18180_e12660)), ((((assign18180_e12655 * locals.var_nsub_dn2) * locals.var_beta_inv) + (assign18180_e12657 * locals.var_beta_inv_dn2)) / (2.0 * assign18180_e12660)), ((((assign18180_e12655 * locals.var_nsub_dn4) * locals.var_beta_inv) + (assign18180_e12657 * locals.var_beta_inv_dn4)) / (2.0 * assign18180_e12660)), ((((assign18180_e12655 * locals.var_nsub_dn5) * locals.var_beta_inv) + (assign18180_e12657 * locals.var_beta_inv_dn5)) / (2.0 * assign18180_e12660)), ((((assign18180_e12655 * locals.var_nsub_dn6) * locals.var_beta_inv) + (assign18180_e12657 * locals.var_beta_inv_dn6)) / (2.0 * assign18180_e12660)), ((((assign18180_e12655 * locals.var_nsub_dn7) * locals.var_beta_inv) + (assign18180_e12657 * locals.var_beta_inv_dn7)) / (2.0 * assign18180_e12660)), ((((assign18180_e12655 * locals.var_nsub_dn8) * locals.var_beta_inv) + (assign18180_e12657 * locals.var_beta_inv_dn8)) / (2.0 * assign18180_e12660)), ((((assign18180_e12655 * locals.var_nsub_dn9) * locals.var_beta_inv) + (assign18180_e12657 * locals.var_beta_inv_dn9)) / (2.0 * assign18180_e12660)), ((((assign18180_e12655 * locals.var_nsub_dn10) * locals.var_beta_inv) + (assign18180_e12657 * locals.var_beta_inv_dn10)) / (2.0 * assign18180_e12660)), ((((assign18180_e12655 * locals.var_nsub_dn11) * locals.var_beta_inv) + (assign18180_e12657 * locals.var_beta_inv_dn11)) / (2.0 * assign18180_e12660)), ((((assign18180_e12655 * locals.var_nsub_dn14) * locals.var_beta_inv) + (assign18180_e12657 * locals.var_beta_inv_dn14)) / (2.0 * assign18180_e12660)),)
    } else {
        (locals.var_cnst0, locals.var_cnst0_dn0, locals.var_cnst0_dn2, locals.var_cnst0_dn4, locals.var_cnst0_dn5, locals.var_cnst0_dn6, locals.var_cnst0_dn7, locals.var_cnst0_dn8, locals.var_cnst0_dn9, locals.var_cnst0_dn10, locals.var_cnst0_dn11, locals.var_cnst0_dn14,)
    }
};
        locals.var_cnst0 = assign18180_e12662;
        locals.var_cnst0_dn0 = assign18180_e12662_d_n0;
        locals.var_cnst0_dn2 = assign18180_e12662_d_n2;
        locals.var_cnst0_dn4 = assign18180_e12662_d_n4;
        locals.var_cnst0_dn5 = assign18180_e12662_d_n5;
        locals.var_cnst0_dn6 = assign18180_e12662_d_n6;
        locals.var_cnst0_dn7 = assign18180_e12662_d_n7;
        locals.var_cnst0_dn8 = assign18180_e12662_d_n8;
        locals.var_cnst0_dn9 = assign18180_e12662_d_n9;
        locals.var_cnst0_dn10 = assign18180_e12662_d_n10;
        locals.var_cnst0_dn11 = assign18180_e12662_d_n11;
        locals.var_cnst0_dn14 = assign18180_e12662_d_n14;

        let (assign18190_e12670, assign18190_e12670_d_n0, assign18190_e12670_d_n2, assign18190_e12670_d_n4, assign18190_e12670_d_n5, assign18190_e12670_d_n6, assign18190_e12670_d_n7, assign18190_e12670_d_n8, assign18190_e12670_d_n9, assign18190_e12670_d_n10, assign18190_e12670_d_n11, assign18190_e12670_d_n14,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard376 != 0.0)) {
        let assign18190_e12668: f64 = (locals.var_nin / locals.var_nsub);
        (assign18190_e12668, (((locals.var_nin_dn0 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn0)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn2 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn2)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn4 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn4)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn5 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn5)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn6 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn6)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn7 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn7)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn8 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn8)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn9 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn9)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn10 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn10)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn11 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn11)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn14 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn14)) / (locals.var_nsub * locals.var_nsub)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign18190_e12670;
        locals.var_t1_dn0 = assign18190_e12670_d_n0;
        locals.var_t1_dn2 = assign18190_e12670_d_n2;
        locals.var_t1_dn4 = assign18190_e12670_d_n4;
        locals.var_t1_dn5 = assign18190_e12670_d_n5;
        locals.var_t1_dn6 = assign18190_e12670_d_n6;
        locals.var_t1_dn7 = assign18190_e12670_d_n7;
        locals.var_t1_dn8 = assign18190_e12670_d_n8;
        locals.var_t1_dn9 = assign18190_e12670_d_n9;
        locals.var_t1_dn10 = assign18190_e12670_d_n10;
        locals.var_t1_dn11 = assign18190_e12670_d_n11;
        locals.var_t1_dn14 = assign18190_e12670_d_n14;

        let (assign18200_e12678, assign18200_e12678_d_n0, assign18200_e12678_d_n2, assign18200_e12678_d_n4, assign18200_e12678_d_n5, assign18200_e12678_d_n6, assign18200_e12678_d_n7, assign18200_e12678_d_n8, assign18200_e12678_d_n9, assign18200_e12678_d_n10, assign18200_e12678_d_n11, assign18200_e12678_d_n14,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard376 != 0.0)) {
        let assign18200_e12676: f64 = (locals.var_t1 * locals.var_t1);
        (assign18200_e12676, ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)), ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)), ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)), ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)), ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)), ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)), ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)), ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)), ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)), ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)), ((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)),)
    } else {
        (locals.var_cnst1, locals.var_cnst1_dn0, locals.var_cnst1_dn2, locals.var_cnst1_dn4, locals.var_cnst1_dn5, locals.var_cnst1_dn6, locals.var_cnst1_dn7, locals.var_cnst1_dn8, locals.var_cnst1_dn9, locals.var_cnst1_dn10, locals.var_cnst1_dn11, locals.var_cnst1_dn14,)
    }
};
        locals.var_cnst1 = assign18200_e12678;
        locals.var_cnst1_dn0 = assign18200_e12678_d_n0;
        locals.var_cnst1_dn2 = assign18200_e12678_d_n2;
        locals.var_cnst1_dn4 = assign18200_e12678_d_n4;
        locals.var_cnst1_dn5 = assign18200_e12678_d_n5;
        locals.var_cnst1_dn6 = assign18200_e12678_d_n6;
        locals.var_cnst1_dn7 = assign18200_e12678_d_n7;
        locals.var_cnst1_dn8 = assign18200_e12678_d_n8;
        locals.var_cnst1_dn9 = assign18200_e12678_d_n9;
        locals.var_cnst1_dn10 = assign18200_e12678_d_n10;
        locals.var_cnst1_dn11 = assign18200_e12678_d_n11;
        locals.var_cnst1_dn14 = assign18200_e12678_d_n14;

        let assign18210_e12681: f64 = if locals.var_uc_codep == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard377 = assign18210_e12681;

        let assign18220_e12684: f64 = if locals.var_uc_nover != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard378 = assign18220_e12684;

        let (assign18230_e12697, assign18230_e12697_d_n0, assign18230_e12697_d_n2, assign18230_e12697_d_n4, assign18230_e12697_d_n5, assign18230_e12697_d_n6, assign18230_e12697_d_n7, assign18230_e12697_d_n8, assign18230_e12697_d_n9, assign18230_e12697_d_n10, assign18230_e12697_d_n11, assign18230_e12697_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard377 != 0.0)) && (locals.var_guard378 != 0.0)) {
        let assign18230_e12693: f64 = (locals.var_uc_nover / locals.var_nsub);
        let assign18230_e12694: f64 = (assign18230_e12693).sqrt();
        let assign18230_e12695: f64 = (locals.var_cnst0 * assign18230_e12694);
        (assign18230_e12695, ((locals.var_cnst0_dn0 * assign18230_e12694) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn0) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18230_e12694)))), ((locals.var_cnst0_dn2 * assign18230_e12694) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn2) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18230_e12694)))), ((locals.var_cnst0_dn4 * assign18230_e12694) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn4) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18230_e12694)))), ((locals.var_cnst0_dn5 * assign18230_e12694) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn5) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18230_e12694)))), ((locals.var_cnst0_dn6 * assign18230_e12694) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn6) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18230_e12694)))), ((locals.var_cnst0_dn7 * assign18230_e12694) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn7) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18230_e12694)))), ((locals.var_cnst0_dn8 * assign18230_e12694) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn8) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18230_e12694)))), ((locals.var_cnst0_dn9 * assign18230_e12694) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn9) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18230_e12694)))), ((locals.var_cnst0_dn10 * assign18230_e12694) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn10) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18230_e12694)))), ((locals.var_cnst0_dn11 * assign18230_e12694) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn11) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18230_e12694)))), ((locals.var_cnst0_dn14 * assign18230_e12694) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn14) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18230_e12694)))),)
    } else {
        (locals.var_cnst0over, locals.var_cnst0over_dn0, locals.var_cnst0over_dn2, locals.var_cnst0over_dn4, locals.var_cnst0over_dn5, locals.var_cnst0over_dn6, locals.var_cnst0over_dn7, locals.var_cnst0over_dn8, locals.var_cnst0over_dn9, locals.var_cnst0over_dn10, locals.var_cnst0over_dn11, locals.var_cnst0over_dn14,)
    }
};
        locals.var_cnst0over = assign18230_e12697;
        locals.var_cnst0over_dn0 = assign18230_e12697_d_n0;
        locals.var_cnst0over_dn2 = assign18230_e12697_d_n2;
        locals.var_cnst0over_dn4 = assign18230_e12697_d_n4;
        locals.var_cnst0over_dn5 = assign18230_e12697_d_n5;
        locals.var_cnst0over_dn6 = assign18230_e12697_d_n6;
        locals.var_cnst0over_dn7 = assign18230_e12697_d_n7;
        locals.var_cnst0over_dn8 = assign18230_e12697_d_n8;
        locals.var_cnst0over_dn9 = assign18230_e12697_d_n9;
        locals.var_cnst0over_dn10 = assign18230_e12697_d_n10;
        locals.var_cnst0over_dn11 = assign18230_e12697_d_n11;
        locals.var_cnst0over_dn14 = assign18230_e12697_d_n14;

        let assign18240_e12700: f64 = if locals.var_uc_novers != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard379 = assign18240_e12700;

        let (assign18250_e12713, assign18250_e12713_d_n0, assign18250_e12713_d_n2, assign18250_e12713_d_n4, assign18250_e12713_d_n5, assign18250_e12713_d_n6, assign18250_e12713_d_n7, assign18250_e12713_d_n8, assign18250_e12713_d_n9, assign18250_e12713_d_n10, assign18250_e12713_d_n11, assign18250_e12713_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard377 != 0.0)) && (locals.var_guard379 != 0.0)) {
        let assign18250_e12709: f64 = (locals.var_uc_novers / locals.var_nsub);
        let assign18250_e12710: f64 = (assign18250_e12709).sqrt();
        let assign18250_e12711: f64 = (locals.var_cnst0 * assign18250_e12710);
        (assign18250_e12711, ((locals.var_cnst0_dn0 * assign18250_e12710) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn0) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18250_e12710)))), ((locals.var_cnst0_dn2 * assign18250_e12710) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn2) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18250_e12710)))), ((locals.var_cnst0_dn4 * assign18250_e12710) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn4) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18250_e12710)))), ((locals.var_cnst0_dn5 * assign18250_e12710) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn5) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18250_e12710)))), ((locals.var_cnst0_dn6 * assign18250_e12710) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn6) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18250_e12710)))), ((locals.var_cnst0_dn7 * assign18250_e12710) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn7) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18250_e12710)))), ((locals.var_cnst0_dn8 * assign18250_e12710) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn8) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18250_e12710)))), ((locals.var_cnst0_dn9 * assign18250_e12710) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn9) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18250_e12710)))), ((locals.var_cnst0_dn10 * assign18250_e12710) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn10) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18250_e12710)))), ((locals.var_cnst0_dn11 * assign18250_e12710) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn11) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18250_e12710)))), ((locals.var_cnst0_dn14 * assign18250_e12710) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn14) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18250_e12710)))),)
    } else {
        (locals.var_cnst0overs, locals.var_cnst0overs_dn0, locals.var_cnst0overs_dn2, locals.var_cnst0overs_dn4, locals.var_cnst0overs_dn5, locals.var_cnst0overs_dn6, locals.var_cnst0overs_dn7, locals.var_cnst0overs_dn8, locals.var_cnst0overs_dn9, locals.var_cnst0overs_dn10, locals.var_cnst0overs_dn11, locals.var_cnst0overs_dn14,)
    }
};
        locals.var_cnst0overs = assign18250_e12713;
        locals.var_cnst0overs_dn0 = assign18250_e12713_d_n0;
        locals.var_cnst0overs_dn2 = assign18250_e12713_d_n2;
        locals.var_cnst0overs_dn4 = assign18250_e12713_d_n4;
        locals.var_cnst0overs_dn5 = assign18250_e12713_d_n5;
        locals.var_cnst0overs_dn6 = assign18250_e12713_d_n6;
        locals.var_cnst0overs_dn7 = assign18250_e12713_d_n7;
        locals.var_cnst0overs_dn8 = assign18250_e12713_d_n8;
        locals.var_cnst0overs_dn9 = assign18250_e12713_d_n9;
        locals.var_cnst0overs_dn10 = assign18250_e12713_d_n10;
        locals.var_cnst0overs_dn11 = assign18250_e12713_d_n11;
        locals.var_cnst0overs_dn14 = assign18250_e12713_d_n14;

        let assign18260_e12716: f64 = if locals.var_uc_nover != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard380 = assign18260_e12716;

        let (assign18270_e12730, assign18270_e12730_d_n0, assign18270_e12730_d_n2, assign18270_e12730_d_n4, assign18270_e12730_d_n5, assign18270_e12730_d_n6, assign18270_e12730_d_n7, assign18270_e12730_d_n8, assign18270_e12730_d_n9, assign18270_e12730_d_n10, assign18270_e12730_d_n11, assign18270_e12730_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard377 == 0.0)) && (locals.var_guard380 != 0.0)) {
        let assign18270_e12726: f64 = (locals.var_uc_nover / locals.var_uc_ndepm);
        let assign18270_e12727: f64 = (assign18270_e12726).sqrt();
        let assign18270_e12728: f64 = (locals.var_cnst0 * assign18270_e12727);
        (assign18270_e12728, ((locals.var_cnst0_dn0 * assign18270_e12727) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn0) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18270_e12727)))), ((locals.var_cnst0_dn2 * assign18270_e12727) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn2) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18270_e12727)))), ((locals.var_cnst0_dn4 * assign18270_e12727) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn4) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18270_e12727)))), ((locals.var_cnst0_dn5 * assign18270_e12727) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn5) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18270_e12727)))), ((locals.var_cnst0_dn6 * assign18270_e12727) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn6) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18270_e12727)))), ((locals.var_cnst0_dn7 * assign18270_e12727) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn7) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18270_e12727)))), ((locals.var_cnst0_dn8 * assign18270_e12727) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn8) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18270_e12727)))), ((locals.var_cnst0_dn9 * assign18270_e12727) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn9) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18270_e12727)))), ((locals.var_cnst0_dn10 * assign18270_e12727) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn10) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18270_e12727)))), ((locals.var_cnst0_dn11 * assign18270_e12727) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn11) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18270_e12727)))), ((locals.var_cnst0_dn14 * assign18270_e12727) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn14) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18270_e12727)))),)
    } else {
        (locals.var_cnst0over, locals.var_cnst0over_dn0, locals.var_cnst0over_dn2, locals.var_cnst0over_dn4, locals.var_cnst0over_dn5, locals.var_cnst0over_dn6, locals.var_cnst0over_dn7, locals.var_cnst0over_dn8, locals.var_cnst0over_dn9, locals.var_cnst0over_dn10, locals.var_cnst0over_dn11, locals.var_cnst0over_dn14,)
    }
};
        locals.var_cnst0over = assign18270_e12730;
        locals.var_cnst0over_dn0 = assign18270_e12730_d_n0;
        locals.var_cnst0over_dn2 = assign18270_e12730_d_n2;
        locals.var_cnst0over_dn4 = assign18270_e12730_d_n4;
        locals.var_cnst0over_dn5 = assign18270_e12730_d_n5;
        locals.var_cnst0over_dn6 = assign18270_e12730_d_n6;
        locals.var_cnst0over_dn7 = assign18270_e12730_d_n7;
        locals.var_cnst0over_dn8 = assign18270_e12730_d_n8;
        locals.var_cnst0over_dn9 = assign18270_e12730_d_n9;
        locals.var_cnst0over_dn10 = assign18270_e12730_d_n10;
        locals.var_cnst0over_dn11 = assign18270_e12730_d_n11;
        locals.var_cnst0over_dn14 = assign18270_e12730_d_n14;

        let assign18280_e12733: f64 = if locals.var_uc_novers != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard381 = assign18280_e12733;

        let (assign18290_e12747, assign18290_e12747_d_n0, assign18290_e12747_d_n2, assign18290_e12747_d_n4, assign18290_e12747_d_n5, assign18290_e12747_d_n6, assign18290_e12747_d_n7, assign18290_e12747_d_n8, assign18290_e12747_d_n9, assign18290_e12747_d_n10, assign18290_e12747_d_n11, assign18290_e12747_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard377 == 0.0)) && (locals.var_guard381 != 0.0)) {
        let assign18290_e12743: f64 = (locals.var_uc_novers / locals.var_uc_ndepm);
        let assign18290_e12744: f64 = (assign18290_e12743).sqrt();
        let assign18290_e12745: f64 = (locals.var_cnst0 * assign18290_e12744);
        (assign18290_e12745, ((locals.var_cnst0_dn0 * assign18290_e12744) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn0) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18290_e12744)))), ((locals.var_cnst0_dn2 * assign18290_e12744) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn2) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18290_e12744)))), ((locals.var_cnst0_dn4 * assign18290_e12744) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn4) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18290_e12744)))), ((locals.var_cnst0_dn5 * assign18290_e12744) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn5) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18290_e12744)))), ((locals.var_cnst0_dn6 * assign18290_e12744) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn6) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18290_e12744)))), ((locals.var_cnst0_dn7 * assign18290_e12744) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn7) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18290_e12744)))), ((locals.var_cnst0_dn8 * assign18290_e12744) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn8) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18290_e12744)))), ((locals.var_cnst0_dn9 * assign18290_e12744) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn9) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18290_e12744)))), ((locals.var_cnst0_dn10 * assign18290_e12744) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn10) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18290_e12744)))), ((locals.var_cnst0_dn11 * assign18290_e12744) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn11) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18290_e12744)))), ((locals.var_cnst0_dn14 * assign18290_e12744) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn14) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18290_e12744)))),)
    } else {
        (locals.var_cnst0overs, locals.var_cnst0overs_dn0, locals.var_cnst0overs_dn2, locals.var_cnst0overs_dn4, locals.var_cnst0overs_dn5, locals.var_cnst0overs_dn6, locals.var_cnst0overs_dn7, locals.var_cnst0overs_dn8, locals.var_cnst0overs_dn9, locals.var_cnst0overs_dn10, locals.var_cnst0overs_dn11, locals.var_cnst0overs_dn14,)
    }
};
        locals.var_cnst0overs = assign18290_e12747;
        locals.var_cnst0overs_dn0 = assign18290_e12747_d_n0;
        locals.var_cnst0overs_dn2 = assign18290_e12747_d_n2;
        locals.var_cnst0overs_dn4 = assign18290_e12747_d_n4;
        locals.var_cnst0overs_dn5 = assign18290_e12747_d_n5;
        locals.var_cnst0overs_dn6 = assign18290_e12747_d_n6;
        locals.var_cnst0overs_dn7 = assign18290_e12747_d_n7;
        locals.var_cnst0overs_dn8 = assign18290_e12747_d_n8;
        locals.var_cnst0overs_dn9 = assign18290_e12747_d_n9;
        locals.var_cnst0overs_dn10 = assign18290_e12747_d_n10;
        locals.var_cnst0overs_dn11 = assign18290_e12747_d_n11;
        locals.var_cnst0overs_dn14 = assign18290_e12747_d_n14;

        let assign18300_e12750: f64 = if locals.var_uc_cordrift == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard382 = assign18300_e12750;

        let assign18310_e12753: f64 = if locals.var_uc_rd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard383 = assign18310_e12753;

        let (assign18320_e12777, assign18320_e12777_d_n0, assign18320_e12777_d_n2, assign18320_e12777_d_n4, assign18320_e12777_d_n5, assign18320_e12777_d_n6, assign18320_e12777_d_n7, assign18320_e12777_d_n8, assign18320_e12777_d_n9, assign18320_e12777_d_n10, assign18320_e12777_d_n11, assign18320_e12777_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign18320_e12762: f64 = (p.p67 * locals.var_uc_rdslp1);
        let assign18320_e12764: f64 = (assign18320_e12762 * 1000000.0);
        let assign18320_e12766: f64 = (assign18320_e12764 + locals.var_uc_rdict1);
        let assign18320_e12767: f64 = (locals.var_rdtemp0 * assign18320_e12766);
        let assign18320_e12770: f64 = (p.p68 * p.p100);
        let assign18320_e12772: f64 = (assign18320_e12770 * 1000000.0);
        let assign18320_e12774: f64 = (assign18320_e12772 + p.p101);
        let assign18320_e12775: f64 = (assign18320_e12767 * assign18320_e12774);
        (assign18320_e12775, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign18320_e12777;
        locals.var_t2_dn0 = assign18320_e12777_d_n0;
        locals.var_t2_dn2 = assign18320_e12777_d_n2;
        locals.var_t2_dn4 = assign18320_e12777_d_n4;
        locals.var_t2_dn5 = assign18320_e12777_d_n5;
        locals.var_t2_dn6 = assign18320_e12777_d_n6;
        locals.var_t2_dn7 = assign18320_e12777_d_n7;
        locals.var_t2_dn8 = assign18320_e12777_d_n8;
        locals.var_t2_dn9 = assign18320_e12777_d_n9;
        locals.var_t2_dn10 = assign18320_e12777_d_n10;
        locals.var_t2_dn11 = assign18320_e12777_d_n11;
        locals.var_t2_dn14 = assign18320_e12777_d_n14;

    }

    pub(super) fn stamp_transient_block_41(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign18330_e12780: f64 = if p.p39 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard384 = assign18330_e12780;

        let (assign18340_e12800, assign18340_e12800_d_n0, assign18340_e12800_d_n2, assign18340_e12800_d_n4, assign18340_e12800_d_n5, assign18340_e12800_d_n6, assign18340_e12800_d_n7, assign18340_e12800_d_n8, assign18340_e12800_d_n9, assign18340_e12800_d_n10, assign18340_e12800_d_n11, assign18340_e12800_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 != 0.0)) {
        let assign18340_e12791: f64 = (locals.var_mks_rdtemp1 * locals.var_tdiff0);
        let assign18340_e12792: f64 = (locals.var_uc_rd + assign18340_e12791);
        let assign18340_e12795: f64 = (locals.var_mks_rdtemp2 * locals.var_tdiff0_2);
        let assign18340_e12796: f64 = (assign18340_e12792 + assign18340_e12795);
        let assign18340_e12798: f64 = (assign18340_e12796 * locals.var_t2);
        (assign18340_e12798, ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn0) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn0)) * locals.var_t2) + (assign18340_e12796 * locals.var_t2_dn0)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn2) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn2)) * locals.var_t2) + (assign18340_e12796 * locals.var_t2_dn2)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn4) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn4)) * locals.var_t2) + (assign18340_e12796 * locals.var_t2_dn4)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn5) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn5)) * locals.var_t2) + (assign18340_e12796 * locals.var_t2_dn5)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn6) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn6)) * locals.var_t2) + (assign18340_e12796 * locals.var_t2_dn6)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn7) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn7)) * locals.var_t2) + (assign18340_e12796 * locals.var_t2_dn7)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn8) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn8)) * locals.var_t2) + (assign18340_e12796 * locals.var_t2_dn8)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn9) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn9)) * locals.var_t2) + (assign18340_e12796 * locals.var_t2_dn9)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn10) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn10)) * locals.var_t2) + (assign18340_e12796 * locals.var_t2_dn10)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn11) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn11)) * locals.var_t2) + (assign18340_e12796 * locals.var_t2_dn11)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn14) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn14)) * locals.var_t2) + (assign18340_e12796 * locals.var_t2_dn14)),)
    } else {
        (locals.var_rde, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn11, locals.var_rde_dn14,)
    }
};
        locals.var_rde = assign18340_e12800;
        locals.var_rde_dn0 = assign18340_e12800_d_n0;
        locals.var_rde_dn2 = assign18340_e12800_d_n2;
        locals.var_rde_dn4 = assign18340_e12800_d_n4;
        locals.var_rde_dn5 = assign18340_e12800_d_n5;
        locals.var_rde_dn6 = assign18340_e12800_d_n6;
        locals.var_rde_dn7 = assign18340_e12800_d_n7;
        locals.var_rde_dn8 = assign18340_e12800_d_n8;
        locals.var_rde_dn9 = assign18340_e12800_d_n9;
        locals.var_rde_dn10 = assign18340_e12800_d_n10;
        locals.var_rde_dn11 = assign18340_e12800_d_n11;
        locals.var_rde_dn14 = assign18340_e12800_d_n14;

        let (assign18350_e12818, assign18350_e12818_d_n0, assign18350_e12818_d_n2, assign18350_e12818_d_n4, assign18350_e12818_d_n5, assign18350_e12818_d_n6, assign18350_e12818_d_n7, assign18350_e12818_d_n8, assign18350_e12818_d_n9, assign18350_e12818_d_n10, assign18350_e12818_d_n11, assign18350_e12818_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 != 0.0)) {
        let assign18350_e12811: f64 = (0.005 * locals.var_uc_rd);
        let assign18350_e12812: f64 = (locals.var_rde - assign18350_e12811);
        let assign18350_e12815: f64 = (0.01 * locals.var_uc_rd);
        let assign18350_e12816: f64 = (assign18350_e12812 - assign18350_e12815);
        (assign18350_e12816, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn11, locals.var_rde_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign18350_e12818;
        locals.var_tmf1_dn0 = assign18350_e12818_d_n0;
        locals.var_tmf1_dn2 = assign18350_e12818_d_n2;
        locals.var_tmf1_dn4 = assign18350_e12818_d_n4;
        locals.var_tmf1_dn5 = assign18350_e12818_d_n5;
        locals.var_tmf1_dn6 = assign18350_e12818_d_n6;
        locals.var_tmf1_dn7 = assign18350_e12818_d_n7;
        locals.var_tmf1_dn8 = assign18350_e12818_d_n8;
        locals.var_tmf1_dn9 = assign18350_e12818_d_n9;
        locals.var_tmf1_dn10 = assign18350_e12818_d_n10;
        locals.var_tmf1_dn11 = assign18350_e12818_d_n11;
        locals.var_tmf1_dn14 = assign18350_e12818_d_n14;

        let (assign18360_e12836, assign18360_e12836_d_n0, assign18360_e12836_d_n2, assign18360_e12836_d_n4, assign18360_e12836_d_n5, assign18360_e12836_d_n6, assign18360_e12836_d_n7, assign18360_e12836_d_n8, assign18360_e12836_d_n9, assign18360_e12836_d_n10, assign18360_e12836_d_n11, assign18360_e12836_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 != 0.0)) {
        let assign18360_e12829: f64 = (0.005 * locals.var_uc_rd);
        let assign18360_e12830: f64 = (4.0 * assign18360_e12829);
        let assign18360_e12833: f64 = (0.01 * locals.var_uc_rd);
        let assign18360_e12834: f64 = (assign18360_e12830 * assign18360_e12833);
        (assign18360_e12834, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18360_e12836;
        locals.var_tmf2_dn0 = assign18360_e12836_d_n0;
        locals.var_tmf2_dn2 = assign18360_e12836_d_n2;
        locals.var_tmf2_dn4 = assign18360_e12836_d_n4;
        locals.var_tmf2_dn5 = assign18360_e12836_d_n5;
        locals.var_tmf2_dn6 = assign18360_e12836_d_n6;
        locals.var_tmf2_dn7 = assign18360_e12836_d_n7;
        locals.var_tmf2_dn8 = assign18360_e12836_d_n8;
        locals.var_tmf2_dn9 = assign18360_e12836_d_n9;
        locals.var_tmf2_dn10 = assign18360_e12836_d_n10;
        locals.var_tmf2_dn11 = assign18360_e12836_d_n11;
        locals.var_tmf2_dn14 = assign18360_e12836_d_n14;

        let (assign18370_e12852, assign18370_e12852_d_n0, assign18370_e12852_d_n2, assign18370_e12852_d_n4, assign18370_e12852_d_n5, assign18370_e12852_d_n6, assign18370_e12852_d_n7, assign18370_e12852_d_n8, assign18370_e12852_d_n9, assign18370_e12852_d_n10, assign18370_e12852_d_n11, assign18370_e12852_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 != 0.0)) {
        let (assign18370_e12850, assign18370_e12850_d_n0, assign18370_e12850_d_n2, assign18370_e12850_d_n4, assign18370_e12850_d_n5, assign18370_e12850_d_n6, assign18370_e12850_d_n7, assign18370_e12850_d_n8, assign18370_e12850_d_n9, assign18370_e12850_d_n10, assign18370_e12850_d_n11, assign18370_e12850_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign18370_e12849: f64 = (-locals.var_tmf2);
                (assign18370_e12849, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign18370_e12850, assign18370_e12850_d_n0, assign18370_e12850_d_n2, assign18370_e12850_d_n4, assign18370_e12850_d_n5, assign18370_e12850_d_n6, assign18370_e12850_d_n7, assign18370_e12850_d_n8, assign18370_e12850_d_n9, assign18370_e12850_d_n10, assign18370_e12850_d_n11, assign18370_e12850_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18370_e12852;
        locals.var_tmf2_dn0 = assign18370_e12852_d_n0;
        locals.var_tmf2_dn2 = assign18370_e12852_d_n2;
        locals.var_tmf2_dn4 = assign18370_e12852_d_n4;
        locals.var_tmf2_dn5 = assign18370_e12852_d_n5;
        locals.var_tmf2_dn6 = assign18370_e12852_d_n6;
        locals.var_tmf2_dn7 = assign18370_e12852_d_n7;
        locals.var_tmf2_dn8 = assign18370_e12852_d_n8;
        locals.var_tmf2_dn9 = assign18370_e12852_d_n9;
        locals.var_tmf2_dn10 = assign18370_e12852_d_n10;
        locals.var_tmf2_dn11 = assign18370_e12852_d_n11;
        locals.var_tmf2_dn14 = assign18370_e12852_d_n14;

        let (assign18380_e12867, assign18380_e12867_d_n0, assign18380_e12867_d_n2, assign18380_e12867_d_n4, assign18380_e12867_d_n5, assign18380_e12867_d_n6, assign18380_e12867_d_n7, assign18380_e12867_d_n8, assign18380_e12867_d_n9, assign18380_e12867_d_n10, assign18380_e12867_d_n11, assign18380_e12867_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 != 0.0)) {
        let assign18380_e12862: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18380_e12864: f64 = (assign18380_e12862 + locals.var_tmf2);
        let assign18380_e12865: f64 = (assign18380_e12864).sqrt();
        (assign18380_e12865, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18380_e12865)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18380_e12865)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign18380_e12865)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign18380_e12865)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign18380_e12865)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign18380_e12865)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign18380_e12865)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign18380_e12865)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign18380_e12865)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign18380_e12865)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign18380_e12865)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18380_e12867;
        locals.var_tmf2_dn0 = assign18380_e12867_d_n0;
        locals.var_tmf2_dn2 = assign18380_e12867_d_n2;
        locals.var_tmf2_dn4 = assign18380_e12867_d_n4;
        locals.var_tmf2_dn5 = assign18380_e12867_d_n5;
        locals.var_tmf2_dn6 = assign18380_e12867_d_n6;
        locals.var_tmf2_dn7 = assign18380_e12867_d_n7;
        locals.var_tmf2_dn8 = assign18380_e12867_d_n8;
        locals.var_tmf2_dn9 = assign18380_e12867_d_n9;
        locals.var_tmf2_dn10 = assign18380_e12867_d_n10;
        locals.var_tmf2_dn11 = assign18380_e12867_d_n11;
        locals.var_tmf2_dn14 = assign18380_e12867_d_n14;

        let (assign18390_e12883, assign18390_e12883_d_n0, assign18390_e12883_d_n2, assign18390_e12883_d_n4, assign18390_e12883_d_n5, assign18390_e12883_d_n6, assign18390_e12883_d_n7, assign18390_e12883_d_n8, assign18390_e12883_d_n9, assign18390_e12883_d_n10, assign18390_e12883_d_n11, assign18390_e12883_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 != 0.0)) {
        let assign18390_e12879: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign18390_e12880: f64 = (1.0 + assign18390_e12879);
        let assign18390_e12881: f64 = (0.5 * assign18390_e12880);
        (assign18390_e12881, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign18390_e12883;
        locals.var_t0_dn0 = assign18390_e12883_d_n0;
        locals.var_t0_dn2 = assign18390_e12883_d_n2;
        locals.var_t0_dn4 = assign18390_e12883_d_n4;
        locals.var_t0_dn5 = assign18390_e12883_d_n5;
        locals.var_t0_dn6 = assign18390_e12883_d_n6;
        locals.var_t0_dn7 = assign18390_e12883_d_n7;
        locals.var_t0_dn8 = assign18390_e12883_d_n8;
        locals.var_t0_dn9 = assign18390_e12883_d_n9;
        locals.var_t0_dn10 = assign18390_e12883_d_n10;
        locals.var_t0_dn11 = assign18390_e12883_d_n11;
        locals.var_t0_dn14 = assign18390_e12883_d_n14;

        let (assign18400_e12901, assign18400_e12901_d_n0, assign18400_e12901_d_n2, assign18400_e12901_d_n4, assign18400_e12901_d_n5, assign18400_e12901_d_n6, assign18400_e12901_d_n7, assign18400_e12901_d_n8, assign18400_e12901_d_n9, assign18400_e12901_d_n10, assign18400_e12901_d_n11, assign18400_e12901_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 != 0.0)) {
        let assign18400_e12893: f64 = (0.005 * locals.var_uc_rd);
        let assign18400_e12897: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign18400_e12898: f64 = (0.5 * assign18400_e12897);
        let assign18400_e12899: f64 = (assign18400_e12893 + assign18400_e12898);
        (assign18400_e12899, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_rde, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn11, locals.var_rde_dn14,)
    }
};
        locals.var_rde = assign18400_e12901;
        locals.var_rde_dn0 = assign18400_e12901_d_n0;
        locals.var_rde_dn2 = assign18400_e12901_d_n2;
        locals.var_rde_dn4 = assign18400_e12901_d_n4;
        locals.var_rde_dn5 = assign18400_e12901_d_n5;
        locals.var_rde_dn6 = assign18400_e12901_d_n6;
        locals.var_rde_dn7 = assign18400_e12901_d_n7;
        locals.var_rde_dn8 = assign18400_e12901_d_n8;
        locals.var_rde_dn9 = assign18400_e12901_d_n9;
        locals.var_rde_dn10 = assign18400_e12901_d_n10;
        locals.var_rde_dn11 = assign18400_e12901_d_n11;
        locals.var_rde_dn14 = assign18400_e12901_d_n14;

        let (assign18410_e12922, assign18410_e12922_d_n0, assign18410_e12922_d_n2, assign18410_e12922_d_n4, assign18410_e12922_d_n5, assign18410_e12922_d_n6, assign18410_e12922_d_n7, assign18410_e12922_d_n8, assign18410_e12922_d_n9, assign18410_e12922_d_n10, assign18410_e12922_d_n11, assign18410_e12922_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 == 0.0)) {
        let assign18410_e12913: f64 = (locals.var_mks_rdtemp1 * locals.var_tdiff);
        let assign18410_e12914: f64 = (locals.var_uc_rd + assign18410_e12913);
        let assign18410_e12917: f64 = (locals.var_mks_rdtemp2 * locals.var_tdiff_2);
        let assign18410_e12918: f64 = (assign18410_e12914 + assign18410_e12917);
        let assign18410_e12920: f64 = (assign18410_e12918 * locals.var_t2);
        (assign18410_e12920, ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn0) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn0)) * locals.var_t2) + (assign18410_e12918 * locals.var_t2_dn0)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn2) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn2)) * locals.var_t2) + (assign18410_e12918 * locals.var_t2_dn2)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn4) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn4)) * locals.var_t2) + (assign18410_e12918 * locals.var_t2_dn4)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn5) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn5)) * locals.var_t2) + (assign18410_e12918 * locals.var_t2_dn5)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn6) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn6)) * locals.var_t2) + (assign18410_e12918 * locals.var_t2_dn6)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn7) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn7)) * locals.var_t2) + (assign18410_e12918 * locals.var_t2_dn7)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn8) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn8)) * locals.var_t2) + (assign18410_e12918 * locals.var_t2_dn8)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn9) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn9)) * locals.var_t2) + (assign18410_e12918 * locals.var_t2_dn9)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn10) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn10)) * locals.var_t2) + (assign18410_e12918 * locals.var_t2_dn10)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn11) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn11)) * locals.var_t2) + (assign18410_e12918 * locals.var_t2_dn11)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn14) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn14)) * locals.var_t2) + (assign18410_e12918 * locals.var_t2_dn14)),)
    } else {
        (locals.var_rde, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn11, locals.var_rde_dn14,)
    }
};
        locals.var_rde = assign18410_e12922;
        locals.var_rde_dn0 = assign18410_e12922_d_n0;
        locals.var_rde_dn2 = assign18410_e12922_d_n2;
        locals.var_rde_dn4 = assign18410_e12922_d_n4;
        locals.var_rde_dn5 = assign18410_e12922_d_n5;
        locals.var_rde_dn6 = assign18410_e12922_d_n6;
        locals.var_rde_dn7 = assign18410_e12922_d_n7;
        locals.var_rde_dn8 = assign18410_e12922_d_n8;
        locals.var_rde_dn9 = assign18410_e12922_d_n9;
        locals.var_rde_dn10 = assign18410_e12922_d_n10;
        locals.var_rde_dn11 = assign18410_e12922_d_n11;
        locals.var_rde_dn14 = assign18410_e12922_d_n14;

        let (assign18420_e12941, assign18420_e12941_d_n0, assign18420_e12941_d_n2, assign18420_e12941_d_n4, assign18420_e12941_d_n5, assign18420_e12941_d_n6, assign18420_e12941_d_n7, assign18420_e12941_d_n8, assign18420_e12941_d_n9, assign18420_e12941_d_n10, assign18420_e12941_d_n11, assign18420_e12941_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 == 0.0)) {
        let assign18420_e12934: f64 = (0.005 * locals.var_uc_rd);
        let assign18420_e12935: f64 = (locals.var_rde - assign18420_e12934);
        let assign18420_e12938: f64 = (0.01 * locals.var_uc_rd);
        let assign18420_e12939: f64 = (assign18420_e12935 - assign18420_e12938);
        (assign18420_e12939, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn11, locals.var_rde_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign18420_e12941;
        locals.var_tmf1_dn0 = assign18420_e12941_d_n0;
        locals.var_tmf1_dn2 = assign18420_e12941_d_n2;
        locals.var_tmf1_dn4 = assign18420_e12941_d_n4;
        locals.var_tmf1_dn5 = assign18420_e12941_d_n5;
        locals.var_tmf1_dn6 = assign18420_e12941_d_n6;
        locals.var_tmf1_dn7 = assign18420_e12941_d_n7;
        locals.var_tmf1_dn8 = assign18420_e12941_d_n8;
        locals.var_tmf1_dn9 = assign18420_e12941_d_n9;
        locals.var_tmf1_dn10 = assign18420_e12941_d_n10;
        locals.var_tmf1_dn11 = assign18420_e12941_d_n11;
        locals.var_tmf1_dn14 = assign18420_e12941_d_n14;

        let (assign18430_e12960, assign18430_e12960_d_n0, assign18430_e12960_d_n2, assign18430_e12960_d_n4, assign18430_e12960_d_n5, assign18430_e12960_d_n6, assign18430_e12960_d_n7, assign18430_e12960_d_n8, assign18430_e12960_d_n9, assign18430_e12960_d_n10, assign18430_e12960_d_n11, assign18430_e12960_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 == 0.0)) {
        let assign18430_e12953: f64 = (0.005 * locals.var_uc_rd);
        let assign18430_e12954: f64 = (4.0 * assign18430_e12953);
        let assign18430_e12957: f64 = (0.01 * locals.var_uc_rd);
        let assign18430_e12958: f64 = (assign18430_e12954 * assign18430_e12957);
        (assign18430_e12958, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18430_e12960;
        locals.var_tmf2_dn0 = assign18430_e12960_d_n0;
        locals.var_tmf2_dn2 = assign18430_e12960_d_n2;
        locals.var_tmf2_dn4 = assign18430_e12960_d_n4;
        locals.var_tmf2_dn5 = assign18430_e12960_d_n5;
        locals.var_tmf2_dn6 = assign18430_e12960_d_n6;
        locals.var_tmf2_dn7 = assign18430_e12960_d_n7;
        locals.var_tmf2_dn8 = assign18430_e12960_d_n8;
        locals.var_tmf2_dn9 = assign18430_e12960_d_n9;
        locals.var_tmf2_dn10 = assign18430_e12960_d_n10;
        locals.var_tmf2_dn11 = assign18430_e12960_d_n11;
        locals.var_tmf2_dn14 = assign18430_e12960_d_n14;

        let (assign18440_e12977, assign18440_e12977_d_n0, assign18440_e12977_d_n2, assign18440_e12977_d_n4, assign18440_e12977_d_n5, assign18440_e12977_d_n6, assign18440_e12977_d_n7, assign18440_e12977_d_n8, assign18440_e12977_d_n9, assign18440_e12977_d_n10, assign18440_e12977_d_n11, assign18440_e12977_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 == 0.0)) {
        let (assign18440_e12975, assign18440_e12975_d_n0, assign18440_e12975_d_n2, assign18440_e12975_d_n4, assign18440_e12975_d_n5, assign18440_e12975_d_n6, assign18440_e12975_d_n7, assign18440_e12975_d_n8, assign18440_e12975_d_n9, assign18440_e12975_d_n10, assign18440_e12975_d_n11, assign18440_e12975_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign18440_e12974: f64 = (-locals.var_tmf2);
                (assign18440_e12974, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign18440_e12975, assign18440_e12975_d_n0, assign18440_e12975_d_n2, assign18440_e12975_d_n4, assign18440_e12975_d_n5, assign18440_e12975_d_n6, assign18440_e12975_d_n7, assign18440_e12975_d_n8, assign18440_e12975_d_n9, assign18440_e12975_d_n10, assign18440_e12975_d_n11, assign18440_e12975_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18440_e12977;
        locals.var_tmf2_dn0 = assign18440_e12977_d_n0;
        locals.var_tmf2_dn2 = assign18440_e12977_d_n2;
        locals.var_tmf2_dn4 = assign18440_e12977_d_n4;
        locals.var_tmf2_dn5 = assign18440_e12977_d_n5;
        locals.var_tmf2_dn6 = assign18440_e12977_d_n6;
        locals.var_tmf2_dn7 = assign18440_e12977_d_n7;
        locals.var_tmf2_dn8 = assign18440_e12977_d_n8;
        locals.var_tmf2_dn9 = assign18440_e12977_d_n9;
        locals.var_tmf2_dn10 = assign18440_e12977_d_n10;
        locals.var_tmf2_dn11 = assign18440_e12977_d_n11;
        locals.var_tmf2_dn14 = assign18440_e12977_d_n14;

        let (assign18450_e12993, assign18450_e12993_d_n0, assign18450_e12993_d_n2, assign18450_e12993_d_n4, assign18450_e12993_d_n5, assign18450_e12993_d_n6, assign18450_e12993_d_n7, assign18450_e12993_d_n8, assign18450_e12993_d_n9, assign18450_e12993_d_n10, assign18450_e12993_d_n11, assign18450_e12993_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 == 0.0)) {
        let assign18450_e12988: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18450_e12990: f64 = (assign18450_e12988 + locals.var_tmf2);
        let assign18450_e12991: f64 = (assign18450_e12990).sqrt();
        (assign18450_e12991, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18450_e12991)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18450_e12991)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign18450_e12991)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign18450_e12991)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign18450_e12991)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign18450_e12991)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign18450_e12991)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign18450_e12991)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign18450_e12991)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign18450_e12991)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign18450_e12991)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18450_e12993;
        locals.var_tmf2_dn0 = assign18450_e12993_d_n0;
        locals.var_tmf2_dn2 = assign18450_e12993_d_n2;
        locals.var_tmf2_dn4 = assign18450_e12993_d_n4;
        locals.var_tmf2_dn5 = assign18450_e12993_d_n5;
        locals.var_tmf2_dn6 = assign18450_e12993_d_n6;
        locals.var_tmf2_dn7 = assign18450_e12993_d_n7;
        locals.var_tmf2_dn8 = assign18450_e12993_d_n8;
        locals.var_tmf2_dn9 = assign18450_e12993_d_n9;
        locals.var_tmf2_dn10 = assign18450_e12993_d_n10;
        locals.var_tmf2_dn11 = assign18450_e12993_d_n11;
        locals.var_tmf2_dn14 = assign18450_e12993_d_n14;

        let (assign18460_e13010, assign18460_e13010_d_n0, assign18460_e13010_d_n2, assign18460_e13010_d_n4, assign18460_e13010_d_n5, assign18460_e13010_d_n6, assign18460_e13010_d_n7, assign18460_e13010_d_n8, assign18460_e13010_d_n9, assign18460_e13010_d_n10, assign18460_e13010_d_n11, assign18460_e13010_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 == 0.0)) {
        let assign18460_e13006: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign18460_e13007: f64 = (1.0 + assign18460_e13006);
        let assign18460_e13008: f64 = (0.5 * assign18460_e13007);
        (assign18460_e13008, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign18460_e13010;
        locals.var_t0_dn0 = assign18460_e13010_d_n0;
        locals.var_t0_dn2 = assign18460_e13010_d_n2;
        locals.var_t0_dn4 = assign18460_e13010_d_n4;
        locals.var_t0_dn5 = assign18460_e13010_d_n5;
        locals.var_t0_dn6 = assign18460_e13010_d_n6;
        locals.var_t0_dn7 = assign18460_e13010_d_n7;
        locals.var_t0_dn8 = assign18460_e13010_d_n8;
        locals.var_t0_dn9 = assign18460_e13010_d_n9;
        locals.var_t0_dn10 = assign18460_e13010_d_n10;
        locals.var_t0_dn11 = assign18460_e13010_d_n11;
        locals.var_t0_dn14 = assign18460_e13010_d_n14;

        let (assign18470_e13029, assign18470_e13029_d_n0, assign18470_e13029_d_n2, assign18470_e13029_d_n4, assign18470_e13029_d_n5, assign18470_e13029_d_n6, assign18470_e13029_d_n7, assign18470_e13029_d_n8, assign18470_e13029_d_n9, assign18470_e13029_d_n10, assign18470_e13029_d_n11, assign18470_e13029_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 == 0.0)) {
        let assign18470_e13021: f64 = (0.005 * locals.var_uc_rd);
        let assign18470_e13025: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign18470_e13026: f64 = (0.5 * assign18470_e13025);
        let assign18470_e13027: f64 = (assign18470_e13021 + assign18470_e13026);
        (assign18470_e13027, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_rde, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn11, locals.var_rde_dn14,)
    }
};
        locals.var_rde = assign18470_e13029;
        locals.var_rde_dn0 = assign18470_e13029_d_n0;
        locals.var_rde_dn2 = assign18470_e13029_d_n2;
        locals.var_rde_dn4 = assign18470_e13029_d_n4;
        locals.var_rde_dn5 = assign18470_e13029_d_n5;
        locals.var_rde_dn6 = assign18470_e13029_d_n6;
        locals.var_rde_dn7 = assign18470_e13029_d_n7;
        locals.var_rde_dn8 = assign18470_e13029_d_n8;
        locals.var_rde_dn9 = assign18470_e13029_d_n9;
        locals.var_rde_dn10 = assign18470_e13029_d_n10;
        locals.var_rde_dn11 = assign18470_e13029_d_n11;
        locals.var_rde_dn14 = assign18470_e13029_d_n14;

        let (assign18480_e13038, assign18480_e13038_d_n0, assign18480_e13038_d_n2, assign18480_e13038_d_n4, assign18480_e13038_d_n5, assign18480_e13038_d_n6, assign18480_e13038_d_n7, assign18480_e13038_d_n8, assign18480_e13038_d_n9, assign18480_e13038_d_n10, assign18480_e13038_d_n11, assign18480_e13038_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard383 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rde, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn11, locals.var_rde_dn14,)
    }
};
        locals.var_rde = assign18480_e13038;
        locals.var_rde_dn0 = assign18480_e13038_d_n0;
        locals.var_rde_dn2 = assign18480_e13038_d_n2;
        locals.var_rde_dn4 = assign18480_e13038_d_n4;
        locals.var_rde_dn5 = assign18480_e13038_d_n5;
        locals.var_rde_dn6 = assign18480_e13038_d_n6;
        locals.var_rde_dn7 = assign18480_e13038_d_n7;
        locals.var_rde_dn8 = assign18480_e13038_d_n8;
        locals.var_rde_dn9 = assign18480_e13038_d_n9;
        locals.var_rde_dn10 = assign18480_e13038_d_n10;
        locals.var_rde_dn11 = assign18480_e13038_d_n11;
        locals.var_rde_dn14 = assign18480_e13038_d_n14;

        let assign18490_e13041: f64 = if locals.var_uc_rs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard385 = assign18490_e13041;

        let (assign18500_e13065, assign18500_e13065_d_n0, assign18500_e13065_d_n2, assign18500_e13065_d_n4, assign18500_e13065_d_n5, assign18500_e13065_d_n6, assign18500_e13065_d_n7, assign18500_e13065_d_n8, assign18500_e13065_d_n9, assign18500_e13065_d_n10, assign18500_e13065_d_n11, assign18500_e13065_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign18500_e13050: f64 = (p.p69 * locals.var_uc_rdslp1);
        let assign18500_e13052: f64 = (assign18500_e13050 * 1000000.0);
        let assign18500_e13054: f64 = (assign18500_e13052 + locals.var_uc_rdict1);
        let assign18500_e13055: f64 = (locals.var_rdtemp0 * assign18500_e13054);
        let assign18500_e13058: f64 = (p.p70 * p.p100);
        let assign18500_e13060: f64 = (assign18500_e13058 * 1000000.0);
        let assign18500_e13062: f64 = (assign18500_e13060 + p.p101);
        let assign18500_e13063: f64 = (assign18500_e13055 * assign18500_e13062);
        (assign18500_e13063, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign18500_e13065;
        locals.var_t2_dn0 = assign18500_e13065_d_n0;
        locals.var_t2_dn2 = assign18500_e13065_d_n2;
        locals.var_t2_dn4 = assign18500_e13065_d_n4;
        locals.var_t2_dn5 = assign18500_e13065_d_n5;
        locals.var_t2_dn6 = assign18500_e13065_d_n6;
        locals.var_t2_dn7 = assign18500_e13065_d_n7;
        locals.var_t2_dn8 = assign18500_e13065_d_n8;
        locals.var_t2_dn9 = assign18500_e13065_d_n9;
        locals.var_t2_dn10 = assign18500_e13065_d_n10;
        locals.var_t2_dn11 = assign18500_e13065_d_n11;
        locals.var_t2_dn14 = assign18500_e13065_d_n14;

        let assign18510_e13068: f64 = if p.p39 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard386 = assign18510_e13068;

        let (assign18520_e13088, assign18520_e13088_d_n0, assign18520_e13088_d_n2, assign18520_e13088_d_n4, assign18520_e13088_d_n5, assign18520_e13088_d_n6, assign18520_e13088_d_n7, assign18520_e13088_d_n8, assign18520_e13088_d_n9, assign18520_e13088_d_n10, assign18520_e13088_d_n11, assign18520_e13088_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard386 != 0.0)) {
        let assign18520_e13079: f64 = (locals.var_mks_rdtemp1 * locals.var_tdiff0);
        let assign18520_e13080: f64 = (locals.var_uc_rs + assign18520_e13079);
        let assign18520_e13083: f64 = (locals.var_mks_rdtemp2 * locals.var_tdiff0_2);
        let assign18520_e13084: f64 = (assign18520_e13080 + assign18520_e13083);
        let assign18520_e13086: f64 = (assign18520_e13084 * locals.var_t2);
        (assign18520_e13086, ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn0) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn0)) * locals.var_t2) + (assign18520_e13084 * locals.var_t2_dn0)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn2) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn2)) * locals.var_t2) + (assign18520_e13084 * locals.var_t2_dn2)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn4) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn4)) * locals.var_t2) + (assign18520_e13084 * locals.var_t2_dn4)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn5) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn5)) * locals.var_t2) + (assign18520_e13084 * locals.var_t2_dn5)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn6) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn6)) * locals.var_t2) + (assign18520_e13084 * locals.var_t2_dn6)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn7) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn7)) * locals.var_t2) + (assign18520_e13084 * locals.var_t2_dn7)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn8) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn8)) * locals.var_t2) + (assign18520_e13084 * locals.var_t2_dn8)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn9) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn9)) * locals.var_t2) + (assign18520_e13084 * locals.var_t2_dn9)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn10) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn10)) * locals.var_t2) + (assign18520_e13084 * locals.var_t2_dn10)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn11) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn11)) * locals.var_t2) + (assign18520_e13084 * locals.var_t2_dn11)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn14) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn14)) * locals.var_t2) + (assign18520_e13084 * locals.var_t2_dn14)),)
    } else {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn11, locals.var_rse_dn14,)
    }
};
        locals.var_rse = assign18520_e13088;
        locals.var_rse_dn0 = assign18520_e13088_d_n0;
        locals.var_rse_dn2 = assign18520_e13088_d_n2;
        locals.var_rse_dn4 = assign18520_e13088_d_n4;
        locals.var_rse_dn5 = assign18520_e13088_d_n5;
        locals.var_rse_dn6 = assign18520_e13088_d_n6;
        locals.var_rse_dn7 = assign18520_e13088_d_n7;
        locals.var_rse_dn8 = assign18520_e13088_d_n8;
        locals.var_rse_dn9 = assign18520_e13088_d_n9;
        locals.var_rse_dn10 = assign18520_e13088_d_n10;
        locals.var_rse_dn11 = assign18520_e13088_d_n11;
        locals.var_rse_dn14 = assign18520_e13088_d_n14;

        let (assign18530_e13106, assign18530_e13106_d_n0, assign18530_e13106_d_n2, assign18530_e13106_d_n4, assign18530_e13106_d_n5, assign18530_e13106_d_n6, assign18530_e13106_d_n7, assign18530_e13106_d_n8, assign18530_e13106_d_n9, assign18530_e13106_d_n10, assign18530_e13106_d_n11, assign18530_e13106_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard386 != 0.0)) {
        let assign18530_e13099: f64 = (0.005 * locals.var_uc_rs);
        let assign18530_e13100: f64 = (locals.var_rse - assign18530_e13099);
        let assign18530_e13103: f64 = (0.01 * locals.var_uc_rs);
        let assign18530_e13104: f64 = (assign18530_e13100 - assign18530_e13103);
        (assign18530_e13104, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn11, locals.var_rse_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign18530_e13106;
        locals.var_tmf1_dn0 = assign18530_e13106_d_n0;
        locals.var_tmf1_dn2 = assign18530_e13106_d_n2;
        locals.var_tmf1_dn4 = assign18530_e13106_d_n4;
        locals.var_tmf1_dn5 = assign18530_e13106_d_n5;
        locals.var_tmf1_dn6 = assign18530_e13106_d_n6;
        locals.var_tmf1_dn7 = assign18530_e13106_d_n7;
        locals.var_tmf1_dn8 = assign18530_e13106_d_n8;
        locals.var_tmf1_dn9 = assign18530_e13106_d_n9;
        locals.var_tmf1_dn10 = assign18530_e13106_d_n10;
        locals.var_tmf1_dn11 = assign18530_e13106_d_n11;
        locals.var_tmf1_dn14 = assign18530_e13106_d_n14;

        let (assign18540_e13124, assign18540_e13124_d_n0, assign18540_e13124_d_n2, assign18540_e13124_d_n4, assign18540_e13124_d_n5, assign18540_e13124_d_n6, assign18540_e13124_d_n7, assign18540_e13124_d_n8, assign18540_e13124_d_n9, assign18540_e13124_d_n10, assign18540_e13124_d_n11, assign18540_e13124_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard386 != 0.0)) {
        let assign18540_e13117: f64 = (0.005 * locals.var_uc_rs);
        let assign18540_e13118: f64 = (4.0 * assign18540_e13117);
        let assign18540_e13121: f64 = (0.01 * locals.var_uc_rs);
        let assign18540_e13122: f64 = (assign18540_e13118 * assign18540_e13121);
        (assign18540_e13122, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18540_e13124;
        locals.var_tmf2_dn0 = assign18540_e13124_d_n0;
        locals.var_tmf2_dn2 = assign18540_e13124_d_n2;
        locals.var_tmf2_dn4 = assign18540_e13124_d_n4;
        locals.var_tmf2_dn5 = assign18540_e13124_d_n5;
        locals.var_tmf2_dn6 = assign18540_e13124_d_n6;
        locals.var_tmf2_dn7 = assign18540_e13124_d_n7;
        locals.var_tmf2_dn8 = assign18540_e13124_d_n8;
        locals.var_tmf2_dn9 = assign18540_e13124_d_n9;
        locals.var_tmf2_dn10 = assign18540_e13124_d_n10;
        locals.var_tmf2_dn11 = assign18540_e13124_d_n11;
        locals.var_tmf2_dn14 = assign18540_e13124_d_n14;

        let (assign18550_e13140, assign18550_e13140_d_n0, assign18550_e13140_d_n2, assign18550_e13140_d_n4, assign18550_e13140_d_n5, assign18550_e13140_d_n6, assign18550_e13140_d_n7, assign18550_e13140_d_n8, assign18550_e13140_d_n9, assign18550_e13140_d_n10, assign18550_e13140_d_n11, assign18550_e13140_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard386 != 0.0)) {
        let (assign18550_e13138, assign18550_e13138_d_n0, assign18550_e13138_d_n2, assign18550_e13138_d_n4, assign18550_e13138_d_n5, assign18550_e13138_d_n6, assign18550_e13138_d_n7, assign18550_e13138_d_n8, assign18550_e13138_d_n9, assign18550_e13138_d_n10, assign18550_e13138_d_n11, assign18550_e13138_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign18550_e13137: f64 = (-locals.var_tmf2);
                (assign18550_e13137, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign18550_e13138, assign18550_e13138_d_n0, assign18550_e13138_d_n2, assign18550_e13138_d_n4, assign18550_e13138_d_n5, assign18550_e13138_d_n6, assign18550_e13138_d_n7, assign18550_e13138_d_n8, assign18550_e13138_d_n9, assign18550_e13138_d_n10, assign18550_e13138_d_n11, assign18550_e13138_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18550_e13140;
        locals.var_tmf2_dn0 = assign18550_e13140_d_n0;
        locals.var_tmf2_dn2 = assign18550_e13140_d_n2;
        locals.var_tmf2_dn4 = assign18550_e13140_d_n4;
        locals.var_tmf2_dn5 = assign18550_e13140_d_n5;
        locals.var_tmf2_dn6 = assign18550_e13140_d_n6;
        locals.var_tmf2_dn7 = assign18550_e13140_d_n7;
        locals.var_tmf2_dn8 = assign18550_e13140_d_n8;
        locals.var_tmf2_dn9 = assign18550_e13140_d_n9;
        locals.var_tmf2_dn10 = assign18550_e13140_d_n10;
        locals.var_tmf2_dn11 = assign18550_e13140_d_n11;
        locals.var_tmf2_dn14 = assign18550_e13140_d_n14;

    }

    pub(super) fn stamp_transient_block_42(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign18560_e13155, assign18560_e13155_d_n0, assign18560_e13155_d_n2, assign18560_e13155_d_n4, assign18560_e13155_d_n5, assign18560_e13155_d_n6, assign18560_e13155_d_n7, assign18560_e13155_d_n8, assign18560_e13155_d_n9, assign18560_e13155_d_n10, assign18560_e13155_d_n11, assign18560_e13155_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard386 != 0.0)) {
        let assign18560_e13150: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18560_e13152: f64 = (assign18560_e13150 + locals.var_tmf2);
        let assign18560_e13153: f64 = (assign18560_e13152).sqrt();
        (assign18560_e13153, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18560_e13153)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18560_e13153)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign18560_e13153)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign18560_e13153)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign18560_e13153)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign18560_e13153)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign18560_e13153)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign18560_e13153)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign18560_e13153)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign18560_e13153)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign18560_e13153)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18560_e13155;
        locals.var_tmf2_dn0 = assign18560_e13155_d_n0;
        locals.var_tmf2_dn2 = assign18560_e13155_d_n2;
        locals.var_tmf2_dn4 = assign18560_e13155_d_n4;
        locals.var_tmf2_dn5 = assign18560_e13155_d_n5;
        locals.var_tmf2_dn6 = assign18560_e13155_d_n6;
        locals.var_tmf2_dn7 = assign18560_e13155_d_n7;
        locals.var_tmf2_dn8 = assign18560_e13155_d_n8;
        locals.var_tmf2_dn9 = assign18560_e13155_d_n9;
        locals.var_tmf2_dn10 = assign18560_e13155_d_n10;
        locals.var_tmf2_dn11 = assign18560_e13155_d_n11;
        locals.var_tmf2_dn14 = assign18560_e13155_d_n14;

        let (assign18570_e13171, assign18570_e13171_d_n0, assign18570_e13171_d_n2, assign18570_e13171_d_n4, assign18570_e13171_d_n5, assign18570_e13171_d_n6, assign18570_e13171_d_n7, assign18570_e13171_d_n8, assign18570_e13171_d_n9, assign18570_e13171_d_n10, assign18570_e13171_d_n11, assign18570_e13171_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard386 != 0.0)) {
        let assign18570_e13167: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign18570_e13168: f64 = (1.0 + assign18570_e13167);
        let assign18570_e13169: f64 = (0.5 * assign18570_e13168);
        (assign18570_e13169, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign18570_e13171;
        locals.var_t0_dn0 = assign18570_e13171_d_n0;
        locals.var_t0_dn2 = assign18570_e13171_d_n2;
        locals.var_t0_dn4 = assign18570_e13171_d_n4;
        locals.var_t0_dn5 = assign18570_e13171_d_n5;
        locals.var_t0_dn6 = assign18570_e13171_d_n6;
        locals.var_t0_dn7 = assign18570_e13171_d_n7;
        locals.var_t0_dn8 = assign18570_e13171_d_n8;
        locals.var_t0_dn9 = assign18570_e13171_d_n9;
        locals.var_t0_dn10 = assign18570_e13171_d_n10;
        locals.var_t0_dn11 = assign18570_e13171_d_n11;
        locals.var_t0_dn14 = assign18570_e13171_d_n14;

        let (assign18580_e13189, assign18580_e13189_d_n0, assign18580_e13189_d_n2, assign18580_e13189_d_n4, assign18580_e13189_d_n5, assign18580_e13189_d_n6, assign18580_e13189_d_n7, assign18580_e13189_d_n8, assign18580_e13189_d_n9, assign18580_e13189_d_n10, assign18580_e13189_d_n11, assign18580_e13189_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard386 != 0.0)) {
        let assign18580_e13181: f64 = (0.005 * locals.var_uc_rs);
        let assign18580_e13185: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign18580_e13186: f64 = (0.5 * assign18580_e13185);
        let assign18580_e13187: f64 = (assign18580_e13181 + assign18580_e13186);
        (assign18580_e13187, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn11, locals.var_rse_dn14,)
    }
};
        locals.var_rse = assign18580_e13189;
        locals.var_rse_dn0 = assign18580_e13189_d_n0;
        locals.var_rse_dn2 = assign18580_e13189_d_n2;
        locals.var_rse_dn4 = assign18580_e13189_d_n4;
        locals.var_rse_dn5 = assign18580_e13189_d_n5;
        locals.var_rse_dn6 = assign18580_e13189_d_n6;
        locals.var_rse_dn7 = assign18580_e13189_d_n7;
        locals.var_rse_dn8 = assign18580_e13189_d_n8;
        locals.var_rse_dn9 = assign18580_e13189_d_n9;
        locals.var_rse_dn10 = assign18580_e13189_d_n10;
        locals.var_rse_dn11 = assign18580_e13189_d_n11;
        locals.var_rse_dn14 = assign18580_e13189_d_n14;

        let (assign18590_e13210, assign18590_e13210_d_n0, assign18590_e13210_d_n2, assign18590_e13210_d_n4, assign18590_e13210_d_n5, assign18590_e13210_d_n6, assign18590_e13210_d_n7, assign18590_e13210_d_n8, assign18590_e13210_d_n9, assign18590_e13210_d_n10, assign18590_e13210_d_n11, assign18590_e13210_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard386 == 0.0)) {
        let assign18590_e13201: f64 = (locals.var_mks_rdtemp1 * locals.var_tdiff);
        let assign18590_e13202: f64 = (locals.var_uc_rs + assign18590_e13201);
        let assign18590_e13205: f64 = (locals.var_mks_rdtemp2 * locals.var_tdiff_2);
        let assign18590_e13206: f64 = (assign18590_e13202 + assign18590_e13205);
        let assign18590_e13208: f64 = (assign18590_e13206 * locals.var_t2);
        (assign18590_e13208, ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn0) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn0)) * locals.var_t2) + (assign18590_e13206 * locals.var_t2_dn0)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn2) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn2)) * locals.var_t2) + (assign18590_e13206 * locals.var_t2_dn2)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn4) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn4)) * locals.var_t2) + (assign18590_e13206 * locals.var_t2_dn4)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn5) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn5)) * locals.var_t2) + (assign18590_e13206 * locals.var_t2_dn5)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn6) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn6)) * locals.var_t2) + (assign18590_e13206 * locals.var_t2_dn6)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn7) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn7)) * locals.var_t2) + (assign18590_e13206 * locals.var_t2_dn7)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn8) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn8)) * locals.var_t2) + (assign18590_e13206 * locals.var_t2_dn8)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn9) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn9)) * locals.var_t2) + (assign18590_e13206 * locals.var_t2_dn9)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn10) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn10)) * locals.var_t2) + (assign18590_e13206 * locals.var_t2_dn10)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn11) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn11)) * locals.var_t2) + (assign18590_e13206 * locals.var_t2_dn11)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn14) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn14)) * locals.var_t2) + (assign18590_e13206 * locals.var_t2_dn14)),)
    } else {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn11, locals.var_rse_dn14,)
    }
};
        locals.var_rse = assign18590_e13210;
        locals.var_rse_dn0 = assign18590_e13210_d_n0;
        locals.var_rse_dn2 = assign18590_e13210_d_n2;
        locals.var_rse_dn4 = assign18590_e13210_d_n4;
        locals.var_rse_dn5 = assign18590_e13210_d_n5;
        locals.var_rse_dn6 = assign18590_e13210_d_n6;
        locals.var_rse_dn7 = assign18590_e13210_d_n7;
        locals.var_rse_dn8 = assign18590_e13210_d_n8;
        locals.var_rse_dn9 = assign18590_e13210_d_n9;
        locals.var_rse_dn10 = assign18590_e13210_d_n10;
        locals.var_rse_dn11 = assign18590_e13210_d_n11;
        locals.var_rse_dn14 = assign18590_e13210_d_n14;

        let (assign18600_e13229, assign18600_e13229_d_n0, assign18600_e13229_d_n2, assign18600_e13229_d_n4, assign18600_e13229_d_n5, assign18600_e13229_d_n6, assign18600_e13229_d_n7, assign18600_e13229_d_n8, assign18600_e13229_d_n9, assign18600_e13229_d_n10, assign18600_e13229_d_n11, assign18600_e13229_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard386 == 0.0)) {
        let assign18600_e13222: f64 = (0.005 * locals.var_uc_rs);
        let assign18600_e13223: f64 = (locals.var_rse - assign18600_e13222);
        let assign18600_e13226: f64 = (0.01 * locals.var_uc_rs);
        let assign18600_e13227: f64 = (assign18600_e13223 - assign18600_e13226);
        (assign18600_e13227, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn11, locals.var_rse_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign18600_e13229;
        locals.var_tmf1_dn0 = assign18600_e13229_d_n0;
        locals.var_tmf1_dn2 = assign18600_e13229_d_n2;
        locals.var_tmf1_dn4 = assign18600_e13229_d_n4;
        locals.var_tmf1_dn5 = assign18600_e13229_d_n5;
        locals.var_tmf1_dn6 = assign18600_e13229_d_n6;
        locals.var_tmf1_dn7 = assign18600_e13229_d_n7;
        locals.var_tmf1_dn8 = assign18600_e13229_d_n8;
        locals.var_tmf1_dn9 = assign18600_e13229_d_n9;
        locals.var_tmf1_dn10 = assign18600_e13229_d_n10;
        locals.var_tmf1_dn11 = assign18600_e13229_d_n11;
        locals.var_tmf1_dn14 = assign18600_e13229_d_n14;

        let (assign18610_e13248, assign18610_e13248_d_n0, assign18610_e13248_d_n2, assign18610_e13248_d_n4, assign18610_e13248_d_n5, assign18610_e13248_d_n6, assign18610_e13248_d_n7, assign18610_e13248_d_n8, assign18610_e13248_d_n9, assign18610_e13248_d_n10, assign18610_e13248_d_n11, assign18610_e13248_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard386 == 0.0)) {
        let assign18610_e13241: f64 = (0.005 * locals.var_uc_rs);
        let assign18610_e13242: f64 = (4.0 * assign18610_e13241);
        let assign18610_e13245: f64 = (0.01 * locals.var_uc_rs);
        let assign18610_e13246: f64 = (assign18610_e13242 * assign18610_e13245);
        (assign18610_e13246, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18610_e13248;
        locals.var_tmf2_dn0 = assign18610_e13248_d_n0;
        locals.var_tmf2_dn2 = assign18610_e13248_d_n2;
        locals.var_tmf2_dn4 = assign18610_e13248_d_n4;
        locals.var_tmf2_dn5 = assign18610_e13248_d_n5;
        locals.var_tmf2_dn6 = assign18610_e13248_d_n6;
        locals.var_tmf2_dn7 = assign18610_e13248_d_n7;
        locals.var_tmf2_dn8 = assign18610_e13248_d_n8;
        locals.var_tmf2_dn9 = assign18610_e13248_d_n9;
        locals.var_tmf2_dn10 = assign18610_e13248_d_n10;
        locals.var_tmf2_dn11 = assign18610_e13248_d_n11;
        locals.var_tmf2_dn14 = assign18610_e13248_d_n14;

        let (assign18620_e13265, assign18620_e13265_d_n0, assign18620_e13265_d_n2, assign18620_e13265_d_n4, assign18620_e13265_d_n5, assign18620_e13265_d_n6, assign18620_e13265_d_n7, assign18620_e13265_d_n8, assign18620_e13265_d_n9, assign18620_e13265_d_n10, assign18620_e13265_d_n11, assign18620_e13265_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard386 == 0.0)) {
        let (assign18620_e13263, assign18620_e13263_d_n0, assign18620_e13263_d_n2, assign18620_e13263_d_n4, assign18620_e13263_d_n5, assign18620_e13263_d_n6, assign18620_e13263_d_n7, assign18620_e13263_d_n8, assign18620_e13263_d_n9, assign18620_e13263_d_n10, assign18620_e13263_d_n11, assign18620_e13263_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign18620_e13262: f64 = (-locals.var_tmf2);
                (assign18620_e13262, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign18620_e13263, assign18620_e13263_d_n0, assign18620_e13263_d_n2, assign18620_e13263_d_n4, assign18620_e13263_d_n5, assign18620_e13263_d_n6, assign18620_e13263_d_n7, assign18620_e13263_d_n8, assign18620_e13263_d_n9, assign18620_e13263_d_n10, assign18620_e13263_d_n11, assign18620_e13263_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18620_e13265;
        locals.var_tmf2_dn0 = assign18620_e13265_d_n0;
        locals.var_tmf2_dn2 = assign18620_e13265_d_n2;
        locals.var_tmf2_dn4 = assign18620_e13265_d_n4;
        locals.var_tmf2_dn5 = assign18620_e13265_d_n5;
        locals.var_tmf2_dn6 = assign18620_e13265_d_n6;
        locals.var_tmf2_dn7 = assign18620_e13265_d_n7;
        locals.var_tmf2_dn8 = assign18620_e13265_d_n8;
        locals.var_tmf2_dn9 = assign18620_e13265_d_n9;
        locals.var_tmf2_dn10 = assign18620_e13265_d_n10;
        locals.var_tmf2_dn11 = assign18620_e13265_d_n11;
        locals.var_tmf2_dn14 = assign18620_e13265_d_n14;

        let (assign18630_e13281, assign18630_e13281_d_n0, assign18630_e13281_d_n2, assign18630_e13281_d_n4, assign18630_e13281_d_n5, assign18630_e13281_d_n6, assign18630_e13281_d_n7, assign18630_e13281_d_n8, assign18630_e13281_d_n9, assign18630_e13281_d_n10, assign18630_e13281_d_n11, assign18630_e13281_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard386 == 0.0)) {
        let assign18630_e13276: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18630_e13278: f64 = (assign18630_e13276 + locals.var_tmf2);
        let assign18630_e13279: f64 = (assign18630_e13278).sqrt();
        (assign18630_e13279, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18630_e13279)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18630_e13279)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign18630_e13279)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign18630_e13279)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign18630_e13279)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign18630_e13279)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign18630_e13279)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign18630_e13279)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign18630_e13279)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign18630_e13279)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign18630_e13279)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18630_e13281;
        locals.var_tmf2_dn0 = assign18630_e13281_d_n0;
        locals.var_tmf2_dn2 = assign18630_e13281_d_n2;
        locals.var_tmf2_dn4 = assign18630_e13281_d_n4;
        locals.var_tmf2_dn5 = assign18630_e13281_d_n5;
        locals.var_tmf2_dn6 = assign18630_e13281_d_n6;
        locals.var_tmf2_dn7 = assign18630_e13281_d_n7;
        locals.var_tmf2_dn8 = assign18630_e13281_d_n8;
        locals.var_tmf2_dn9 = assign18630_e13281_d_n9;
        locals.var_tmf2_dn10 = assign18630_e13281_d_n10;
        locals.var_tmf2_dn11 = assign18630_e13281_d_n11;
        locals.var_tmf2_dn14 = assign18630_e13281_d_n14;

        let (assign18640_e13298, assign18640_e13298_d_n0, assign18640_e13298_d_n2, assign18640_e13298_d_n4, assign18640_e13298_d_n5, assign18640_e13298_d_n6, assign18640_e13298_d_n7, assign18640_e13298_d_n8, assign18640_e13298_d_n9, assign18640_e13298_d_n10, assign18640_e13298_d_n11, assign18640_e13298_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard386 == 0.0)) {
        let assign18640_e13294: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign18640_e13295: f64 = (1.0 + assign18640_e13294);
        let assign18640_e13296: f64 = (0.5 * assign18640_e13295);
        (assign18640_e13296, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign18640_e13298;
        locals.var_t0_dn0 = assign18640_e13298_d_n0;
        locals.var_t0_dn2 = assign18640_e13298_d_n2;
        locals.var_t0_dn4 = assign18640_e13298_d_n4;
        locals.var_t0_dn5 = assign18640_e13298_d_n5;
        locals.var_t0_dn6 = assign18640_e13298_d_n6;
        locals.var_t0_dn7 = assign18640_e13298_d_n7;
        locals.var_t0_dn8 = assign18640_e13298_d_n8;
        locals.var_t0_dn9 = assign18640_e13298_d_n9;
        locals.var_t0_dn10 = assign18640_e13298_d_n10;
        locals.var_t0_dn11 = assign18640_e13298_d_n11;
        locals.var_t0_dn14 = assign18640_e13298_d_n14;

        let (assign18650_e13317, assign18650_e13317_d_n0, assign18650_e13317_d_n2, assign18650_e13317_d_n4, assign18650_e13317_d_n5, assign18650_e13317_d_n6, assign18650_e13317_d_n7, assign18650_e13317_d_n8, assign18650_e13317_d_n9, assign18650_e13317_d_n10, assign18650_e13317_d_n11, assign18650_e13317_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard386 == 0.0)) {
        let assign18650_e13309: f64 = (0.005 * locals.var_uc_rs);
        let assign18650_e13313: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign18650_e13314: f64 = (0.5 * assign18650_e13313);
        let assign18650_e13315: f64 = (assign18650_e13309 + assign18650_e13314);
        (assign18650_e13315, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn11, locals.var_rse_dn14,)
    }
};
        locals.var_rse = assign18650_e13317;
        locals.var_rse_dn0 = assign18650_e13317_d_n0;
        locals.var_rse_dn2 = assign18650_e13317_d_n2;
        locals.var_rse_dn4 = assign18650_e13317_d_n4;
        locals.var_rse_dn5 = assign18650_e13317_d_n5;
        locals.var_rse_dn6 = assign18650_e13317_d_n6;
        locals.var_rse_dn7 = assign18650_e13317_d_n7;
        locals.var_rse_dn8 = assign18650_e13317_d_n8;
        locals.var_rse_dn9 = assign18650_e13317_d_n9;
        locals.var_rse_dn10 = assign18650_e13317_d_n10;
        locals.var_rse_dn11 = assign18650_e13317_d_n11;
        locals.var_rse_dn14 = assign18650_e13317_d_n14;

        let (assign18660_e13326, assign18660_e13326_d_n0, assign18660_e13326_d_n2, assign18660_e13326_d_n4, assign18660_e13326_d_n5, assign18660_e13326_d_n6, assign18660_e13326_d_n7, assign18660_e13326_d_n8, assign18660_e13326_d_n9, assign18660_e13326_d_n10, assign18660_e13326_d_n11, assign18660_e13326_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard385 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn11, locals.var_rse_dn14,)
    }
};
        locals.var_rse = assign18660_e13326;
        locals.var_rse_dn0 = assign18660_e13326_d_n0;
        locals.var_rse_dn2 = assign18660_e13326_d_n2;
        locals.var_rse_dn4 = assign18660_e13326_d_n4;
        locals.var_rse_dn5 = assign18660_e13326_d_n5;
        locals.var_rse_dn6 = assign18660_e13326_d_n6;
        locals.var_rse_dn7 = assign18660_e13326_d_n7;
        locals.var_rse_dn8 = assign18660_e13326_d_n8;
        locals.var_rse_dn9 = assign18660_e13326_d_n9;
        locals.var_rse_dn10 = assign18660_e13326_d_n10;
        locals.var_rse_dn11 = assign18660_e13326_d_n11;
        locals.var_rse_dn14 = assign18660_e13326_d_n14;

        let assign18670_e13329: f64 = if locals.var_uc_rdvd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard387 = assign18670_e13329;

        let (assign18680_e13353, assign18680_e13353_d_n0, assign18680_e13353_d_n2, assign18680_e13353_d_n4, assign18680_e13353_d_n5, assign18680_e13353_d_n6, assign18680_e13353_d_n7, assign18680_e13353_d_n8, assign18680_e13353_d_n9, assign18680_e13353_d_n10, assign18680_e13353_d_n11, assign18680_e13353_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign18680_e13338: f64 = (p.p67 * locals.var_uc_rdslp1);
        let assign18680_e13340: f64 = (assign18680_e13338 * 1000000.0);
        let assign18680_e13342: f64 = (assign18680_e13340 + locals.var_uc_rdict1);
        let assign18680_e13343: f64 = (locals.var_rdvdtemp0 * assign18680_e13342);
        let assign18680_e13346: f64 = (p.p68 * p.p100);
        let assign18680_e13348: f64 = (assign18680_e13346 * 1000000.0);
        let assign18680_e13350: f64 = (assign18680_e13348 + p.p101);
        let assign18680_e13351: f64 = (assign18680_e13343 * assign18680_e13350);
        (assign18680_e13351, ((locals.var_rdvdtemp0_dn0 * assign18680_e13342) * assign18680_e13350), ((locals.var_rdvdtemp0_dn2 * assign18680_e13342) * assign18680_e13350), ((locals.var_rdvdtemp0_dn4 * assign18680_e13342) * assign18680_e13350), ((locals.var_rdvdtemp0_dn5 * assign18680_e13342) * assign18680_e13350), ((locals.var_rdvdtemp0_dn6 * assign18680_e13342) * assign18680_e13350), ((locals.var_rdvdtemp0_dn7 * assign18680_e13342) * assign18680_e13350), ((locals.var_rdvdtemp0_dn8 * assign18680_e13342) * assign18680_e13350), ((locals.var_rdvdtemp0_dn9 * assign18680_e13342) * assign18680_e13350), ((locals.var_rdvdtemp0_dn10 * assign18680_e13342) * assign18680_e13350), ((locals.var_rdvdtemp0_dn11 * assign18680_e13342) * assign18680_e13350), ((locals.var_rdvdtemp0_dn14 * assign18680_e13342) * assign18680_e13350),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign18680_e13353;
        locals.var_t4_dn0 = assign18680_e13353_d_n0;
        locals.var_t4_dn2 = assign18680_e13353_d_n2;
        locals.var_t4_dn4 = assign18680_e13353_d_n4;
        locals.var_t4_dn5 = assign18680_e13353_d_n5;
        locals.var_t4_dn6 = assign18680_e13353_d_n6;
        locals.var_t4_dn7 = assign18680_e13353_d_n7;
        locals.var_t4_dn8 = assign18680_e13353_d_n8;
        locals.var_t4_dn9 = assign18680_e13353_d_n9;
        locals.var_t4_dn10 = assign18680_e13353_d_n10;
        locals.var_t4_dn11 = assign18680_e13353_d_n11;
        locals.var_t4_dn14 = assign18680_e13353_d_n14;

        let (assign18690_e13367, assign18690_e13367_d_n0, assign18690_e13367_d_n2, assign18690_e13367_d_n4, assign18690_e13367_d_n5, assign18690_e13367_d_n6, assign18690_e13367_d_n7, assign18690_e13367_d_n8, assign18690_e13367_d_n9, assign18690_e13367_d_n10, assign18690_e13367_d_n11, assign18690_e13367_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign18690_e13361: f64 = (1.0 - locals.var_uc_rdov13);
        let assign18690_e13363: f64 = (assign18690_e13361 * p.p63);
        let assign18690_e13365: f64 = (assign18690_e13363 * 1000000.0);
        (assign18690_e13365, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign18690_e13367;
        locals.var_t1_dn0 = assign18690_e13367_d_n0;
        locals.var_t1_dn2 = assign18690_e13367_d_n2;
        locals.var_t1_dn4 = assign18690_e13367_d_n4;
        locals.var_t1_dn5 = assign18690_e13367_d_n5;
        locals.var_t1_dn6 = assign18690_e13367_d_n6;
        locals.var_t1_dn7 = assign18690_e13367_d_n7;
        locals.var_t1_dn8 = assign18690_e13367_d_n8;
        locals.var_t1_dn9 = assign18690_e13367_d_n9;
        locals.var_t1_dn10 = assign18690_e13367_d_n10;
        locals.var_t1_dn11 = assign18690_e13367_d_n11;
        locals.var_t1_dn14 = assign18690_e13367_d_n14;

        let (assign18700_e13388, assign18700_e13388_d_n0, assign18700_e13388_d_n2, assign18700_e13388_d_n4, assign18700_e13388_d_n5, assign18700_e13388_d_n6, assign18700_e13388_d_n7, assign18700_e13388_d_n8, assign18700_e13388_d_n9, assign18700_e13388_d_n10, assign18700_e13388_d_n11, assign18700_e13388_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign18700_e13375: f64 = (p.p99 * p.p99);
        let assign18700_e13379: f64 = (0.0001 * 0.01);
        let assign18700_e13380: f64 = (4.0 * assign18700_e13379);
        let assign18700_e13383: f64 = (0.0001 * 0.01);
        let assign18700_e13384: f64 = (assign18700_e13380 * assign18700_e13383);
        let assign18700_e13385: f64 = (assign18700_e13375 + assign18700_e13384);
        let assign18700_e13386: f64 = (assign18700_e13385).sqrt();
        (assign18700_e13386, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18700_e13388;
        locals.var_tmf2_dn0 = assign18700_e13388_d_n0;
        locals.var_tmf2_dn2 = assign18700_e13388_d_n2;
        locals.var_tmf2_dn4 = assign18700_e13388_d_n4;
        locals.var_tmf2_dn5 = assign18700_e13388_d_n5;
        locals.var_tmf2_dn6 = assign18700_e13388_d_n6;
        locals.var_tmf2_dn7 = assign18700_e13388_d_n7;
        locals.var_tmf2_dn8 = assign18700_e13388_d_n8;
        locals.var_tmf2_dn9 = assign18700_e13388_d_n9;
        locals.var_tmf2_dn10 = assign18700_e13388_d_n10;
        locals.var_tmf2_dn11 = assign18700_e13388_d_n11;
        locals.var_tmf2_dn14 = assign18700_e13388_d_n14;

        let (assign18710_e13402, assign18710_e13402_d_n0, assign18710_e13402_d_n2, assign18710_e13402_d_n4, assign18710_e13402_d_n5, assign18710_e13402_d_n6, assign18710_e13402_d_n7, assign18710_e13402_d_n8, assign18710_e13402_d_n9, assign18710_e13402_d_n10, assign18710_e13402_d_n11, assign18710_e13402_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign18710_e13398: f64 = (p.p99 / locals.var_tmf2);
        let assign18710_e13399: f64 = (1.0 + assign18710_e13398);
        let assign18710_e13400: f64 = (0.5 * assign18710_e13399);
        (assign18710_e13400, (0.5 * (-((p.p99 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign18710_e13402;
        locals.var_t0_dn0 = assign18710_e13402_d_n0;
        locals.var_t0_dn2 = assign18710_e13402_d_n2;
        locals.var_t0_dn4 = assign18710_e13402_d_n4;
        locals.var_t0_dn5 = assign18710_e13402_d_n5;
        locals.var_t0_dn6 = assign18710_e13402_d_n6;
        locals.var_t0_dn7 = assign18710_e13402_d_n7;
        locals.var_t0_dn8 = assign18710_e13402_d_n8;
        locals.var_t0_dn9 = assign18710_e13402_d_n9;
        locals.var_t0_dn10 = assign18710_e13402_d_n10;
        locals.var_t0_dn11 = assign18710_e13402_d_n11;
        locals.var_t0_dn14 = assign18710_e13402_d_n14;

        let (assign18720_e13414, assign18720_e13414_d_n0, assign18720_e13414_d_n2, assign18720_e13414_d_n4, assign18720_e13414_d_n5, assign18720_e13414_d_n6, assign18720_e13414_d_n7, assign18720_e13414_d_n8, assign18720_e13414_d_n9, assign18720_e13414_d_n10, assign18720_e13414_d_n11, assign18720_e13414_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign18720_e13411: f64 = (p.p99 + locals.var_tmf2);
        let assign18720_e13412: f64 = (0.5 * assign18720_e13411);
        (assign18720_e13412, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * locals.var_tmf2_dn5), (0.5 * locals.var_tmf2_dn6), (0.5 * locals.var_tmf2_dn7), (0.5 * locals.var_tmf2_dn8), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn11), (0.5 * locals.var_tmf2_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign18720_e13414;
        locals.var_t2_dn0 = assign18720_e13414_d_n0;
        locals.var_t2_dn2 = assign18720_e13414_d_n2;
        locals.var_t2_dn4 = assign18720_e13414_d_n4;
        locals.var_t2_dn5 = assign18720_e13414_d_n5;
        locals.var_t2_dn6 = assign18720_e13414_d_n6;
        locals.var_t2_dn7 = assign18720_e13414_d_n7;
        locals.var_t2_dn8 = assign18720_e13414_d_n8;
        locals.var_t2_dn9 = assign18720_e13414_d_n9;
        locals.var_t2_dn10 = assign18720_e13414_d_n10;
        locals.var_t2_dn11 = assign18720_e13414_d_n11;
        locals.var_t2_dn14 = assign18720_e13414_d_n14;

        let assign18730_e13417: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard388 = assign18730_e13417;

        let (assign18740_e13427, assign18740_e13427_d_n0, assign18740_e13427_d_n2, assign18740_e13427_d_n4, assign18740_e13427_d_n5, assign18740_e13427_d_n6, assign18740_e13427_d_n7, assign18740_e13427_d_n8, assign18740_e13427_d_n9, assign18740_e13427_d_n10, assign18740_e13427_d_n11, assign18740_e13427_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard388 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign18740_e13427;
        locals.var_t2_dn0 = assign18740_e13427_d_n0;
        locals.var_t2_dn2 = assign18740_e13427_d_n2;
        locals.var_t2_dn4 = assign18740_e13427_d_n4;
        locals.var_t2_dn5 = assign18740_e13427_d_n5;
        locals.var_t2_dn6 = assign18740_e13427_d_n6;
        locals.var_t2_dn7 = assign18740_e13427_d_n7;
        locals.var_t2_dn8 = assign18740_e13427_d_n8;
        locals.var_t2_dn9 = assign18740_e13427_d_n9;
        locals.var_t2_dn10 = assign18740_e13427_d_n10;
        locals.var_t2_dn11 = assign18740_e13427_d_n11;
        locals.var_t2_dn14 = assign18740_e13427_d_n14;

        let (assign18750_e13437, assign18750_e13437_d_n0, assign18750_e13437_d_n2, assign18750_e13437_d_n4, assign18750_e13437_d_n5, assign18750_e13437_d_n6, assign18750_e13437_d_n7, assign18750_e13437_d_n8, assign18750_e13437_d_n9, assign18750_e13437_d_n10, assign18750_e13437_d_n11, assign18750_e13437_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard388 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign18750_e13437;
        locals.var_t0_dn0 = assign18750_e13437_d_n0;
        locals.var_t0_dn2 = assign18750_e13437_d_n2;
        locals.var_t0_dn4 = assign18750_e13437_d_n4;
        locals.var_t0_dn5 = assign18750_e13437_d_n5;
        locals.var_t0_dn6 = assign18750_e13437_d_n6;
        locals.var_t0_dn7 = assign18750_e13437_d_n7;
        locals.var_t0_dn8 = assign18750_e13437_d_n8;
        locals.var_t0_dn9 = assign18750_e13437_d_n9;
        locals.var_t0_dn10 = assign18750_e13437_d_n10;
        locals.var_t0_dn11 = assign18750_e13437_d_n11;
        locals.var_t0_dn14 = assign18750_e13437_d_n14;

        let (assign18760_e13448, assign18760_e13448_d_n0, assign18760_e13448_d_n2, assign18760_e13448_d_n4, assign18760_e13448_d_n5, assign18760_e13448_d_n6, assign18760_e13448_d_n7, assign18760_e13448_d_n8, assign18760_e13448_d_n9, assign18760_e13448_d_n10, assign18760_e13448_d_n11, assign18760_e13448_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign18760_e13444: f64 = (-p.p98);
        let assign18760_e13446: f64 = (assign18760_e13444 / locals.var_t2);
        (assign18760_e13446, (-((assign18760_e13444 * locals.var_t2_dn0) / (locals.var_t2 * locals.var_t2))), (-((assign18760_e13444 * locals.var_t2_dn2) / (locals.var_t2 * locals.var_t2))), (-((assign18760_e13444 * locals.var_t2_dn4) / (locals.var_t2 * locals.var_t2))), (-((assign18760_e13444 * locals.var_t2_dn5) / (locals.var_t2 * locals.var_t2))), (-((assign18760_e13444 * locals.var_t2_dn6) / (locals.var_t2 * locals.var_t2))), (-((assign18760_e13444 * locals.var_t2_dn7) / (locals.var_t2 * locals.var_t2))), (-((assign18760_e13444 * locals.var_t2_dn8) / (locals.var_t2 * locals.var_t2))), (-((assign18760_e13444 * locals.var_t2_dn9) / (locals.var_t2 * locals.var_t2))), (-((assign18760_e13444 * locals.var_t2_dn10) / (locals.var_t2 * locals.var_t2))), (-((assign18760_e13444 * locals.var_t2_dn11) / (locals.var_t2 * locals.var_t2))), (-((assign18760_e13444 * locals.var_t2_dn14) / (locals.var_t2 * locals.var_t2))),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign18760_e13448;
        locals.var_t8_dn0 = assign18760_e13448_d_n0;
        locals.var_t8_dn2 = assign18760_e13448_d_n2;
        locals.var_t8_dn4 = assign18760_e13448_d_n4;
        locals.var_t8_dn5 = assign18760_e13448_d_n5;
        locals.var_t8_dn6 = assign18760_e13448_d_n6;
        locals.var_t8_dn7 = assign18760_e13448_d_n7;
        locals.var_t8_dn8 = assign18760_e13448_d_n8;
        locals.var_t8_dn9 = assign18760_e13448_d_n9;
        locals.var_t8_dn10 = assign18760_e13448_d_n10;
        locals.var_t8_dn11 = assign18760_e13448_d_n11;
        locals.var_t8_dn14 = assign18760_e13448_d_n14;

        let (assign18770_e13464, assign18770_e13464_d_n0, assign18770_e13464_d_n2, assign18770_e13464_d_n4, assign18770_e13464_d_n5, assign18770_e13464_d_n6, assign18770_e13464_d_n7, assign18770_e13464_d_n8, assign18770_e13464_d_n9, assign18770_e13464_d_n10, assign18770_e13464_d_n11, assign18770_e13464_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign18770_e13456: f64 = (locals.var_t8 * p.p63);
        let assign18770_e13458: f64 = (assign18770_e13456 * 1000000.0);
        let assign18770_e13460: f64 = (assign18770_e13458 + 1.0);
        let assign18770_e13462: f64 = (assign18770_e13460 + p.p98);
        (assign18770_e13462, ((locals.var_t8_dn0 * p.p63) * 1000000.0), ((locals.var_t8_dn2 * p.p63) * 1000000.0), ((locals.var_t8_dn4 * p.p63) * 1000000.0), ((locals.var_t8_dn5 * p.p63) * 1000000.0), ((locals.var_t8_dn6 * p.p63) * 1000000.0), ((locals.var_t8_dn7 * p.p63) * 1000000.0), ((locals.var_t8_dn8 * p.p63) * 1000000.0), ((locals.var_t8_dn9 * p.p63) * 1000000.0), ((locals.var_t8_dn10 * p.p63) * 1000000.0), ((locals.var_t8_dn11 * p.p63) * 1000000.0), ((locals.var_t8_dn14 * p.p63) * 1000000.0),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign18770_e13464;
        locals.var_t3_dn0 = assign18770_e13464_d_n0;
        locals.var_t3_dn2 = assign18770_e13464_d_n2;
        locals.var_t3_dn4 = assign18770_e13464_d_n4;
        locals.var_t3_dn5 = assign18770_e13464_d_n5;
        locals.var_t3_dn6 = assign18770_e13464_d_n6;
        locals.var_t3_dn7 = assign18770_e13464_d_n7;
        locals.var_t3_dn8 = assign18770_e13464_d_n8;
        locals.var_t3_dn9 = assign18770_e13464_d_n9;
        locals.var_t3_dn10 = assign18770_e13464_d_n10;
        locals.var_t3_dn11 = assign18770_e13464_d_n11;
        locals.var_t3_dn14 = assign18770_e13464_d_n14;

        let (assign18780_e13478, assign18780_e13478_d_n0, assign18780_e13478_d_n2, assign18780_e13478_d_n4, assign18780_e13478_d_n5, assign18780_e13478_d_n6, assign18780_e13478_d_n7, assign18780_e13478_d_n8, assign18780_e13478_d_n9, assign18780_e13478_d_n10, assign18780_e13478_d_n11, assign18780_e13478_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign18780_e13472: f64 = (locals.var_t3 * locals.var_t4);
        let assign18780_e13474: f64 = (assign18780_e13472 - locals.var_t4);
        let assign18780_e13476: f64 = (assign18780_e13474 - 0.01);
        (assign18780_e13476, (((locals.var_t3_dn0 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn0)) - locals.var_t4_dn0), (((locals.var_t3_dn2 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn2)) - locals.var_t4_dn2), (((locals.var_t3_dn4 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn4)) - locals.var_t4_dn4), (((locals.var_t3_dn5 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn5)) - locals.var_t4_dn5), (((locals.var_t3_dn6 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn6)) - locals.var_t4_dn6), (((locals.var_t3_dn7 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn7)) - locals.var_t4_dn7), (((locals.var_t3_dn8 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn8)) - locals.var_t4_dn8), (((locals.var_t3_dn9 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn9)) - locals.var_t4_dn9), (((locals.var_t3_dn10 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn10)) - locals.var_t4_dn10), (((locals.var_t3_dn11 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn11)) - locals.var_t4_dn11), (((locals.var_t3_dn14 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn14)) - locals.var_t4_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign18780_e13478;
        locals.var_tmf1_dn0 = assign18780_e13478_d_n0;
        locals.var_tmf1_dn2 = assign18780_e13478_d_n2;
        locals.var_tmf1_dn4 = assign18780_e13478_d_n4;
        locals.var_tmf1_dn5 = assign18780_e13478_d_n5;
        locals.var_tmf1_dn6 = assign18780_e13478_d_n6;
        locals.var_tmf1_dn7 = assign18780_e13478_d_n7;
        locals.var_tmf1_dn8 = assign18780_e13478_d_n8;
        locals.var_tmf1_dn9 = assign18780_e13478_d_n9;
        locals.var_tmf1_dn10 = assign18780_e13478_d_n10;
        locals.var_tmf1_dn11 = assign18780_e13478_d_n11;
        locals.var_tmf1_dn14 = assign18780_e13478_d_n14;

    }

    pub(super) fn stamp_transient_block_43(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign18790_e13490, assign18790_e13490_d_n0, assign18790_e13490_d_n2, assign18790_e13490_d_n4, assign18790_e13490_d_n5, assign18790_e13490_d_n6, assign18790_e13490_d_n7, assign18790_e13490_d_n8, assign18790_e13490_d_n9, assign18790_e13490_d_n10, assign18790_e13490_d_n11, assign18790_e13490_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign18790_e13486: f64 = (4.0 * locals.var_t4);
        let assign18790_e13488: f64 = (assign18790_e13486 * 0.01);
        (assign18790_e13488, ((4.0 * locals.var_t4_dn0) * 0.01), ((4.0 * locals.var_t4_dn2) * 0.01), ((4.0 * locals.var_t4_dn4) * 0.01), ((4.0 * locals.var_t4_dn5) * 0.01), ((4.0 * locals.var_t4_dn6) * 0.01), ((4.0 * locals.var_t4_dn7) * 0.01), ((4.0 * locals.var_t4_dn8) * 0.01), ((4.0 * locals.var_t4_dn9) * 0.01), ((4.0 * locals.var_t4_dn10) * 0.01), ((4.0 * locals.var_t4_dn11) * 0.01), ((4.0 * locals.var_t4_dn14) * 0.01),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18790_e13490;
        locals.var_tmf2_dn0 = assign18790_e13490_d_n0;
        locals.var_tmf2_dn2 = assign18790_e13490_d_n2;
        locals.var_tmf2_dn4 = assign18790_e13490_d_n4;
        locals.var_tmf2_dn5 = assign18790_e13490_d_n5;
        locals.var_tmf2_dn6 = assign18790_e13490_d_n6;
        locals.var_tmf2_dn7 = assign18790_e13490_d_n7;
        locals.var_tmf2_dn8 = assign18790_e13490_d_n8;
        locals.var_tmf2_dn9 = assign18790_e13490_d_n9;
        locals.var_tmf2_dn10 = assign18790_e13490_d_n10;
        locals.var_tmf2_dn11 = assign18790_e13490_d_n11;
        locals.var_tmf2_dn14 = assign18790_e13490_d_n14;

        let (assign18800_e13504, assign18800_e13504_d_n0, assign18800_e13504_d_n2, assign18800_e13504_d_n4, assign18800_e13504_d_n5, assign18800_e13504_d_n6, assign18800_e13504_d_n7, assign18800_e13504_d_n8, assign18800_e13504_d_n9, assign18800_e13504_d_n10, assign18800_e13504_d_n11, assign18800_e13504_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let (assign18800_e13502, assign18800_e13502_d_n0, assign18800_e13502_d_n2, assign18800_e13502_d_n4, assign18800_e13502_d_n5, assign18800_e13502_d_n6, assign18800_e13502_d_n7, assign18800_e13502_d_n8, assign18800_e13502_d_n9, assign18800_e13502_d_n10, assign18800_e13502_d_n11, assign18800_e13502_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign18800_e13501: f64 = (-locals.var_tmf2);
                (assign18800_e13501, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign18800_e13502, assign18800_e13502_d_n0, assign18800_e13502_d_n2, assign18800_e13502_d_n4, assign18800_e13502_d_n5, assign18800_e13502_d_n6, assign18800_e13502_d_n7, assign18800_e13502_d_n8, assign18800_e13502_d_n9, assign18800_e13502_d_n10, assign18800_e13502_d_n11, assign18800_e13502_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18800_e13504;
        locals.var_tmf2_dn0 = assign18800_e13504_d_n0;
        locals.var_tmf2_dn2 = assign18800_e13504_d_n2;
        locals.var_tmf2_dn4 = assign18800_e13504_d_n4;
        locals.var_tmf2_dn5 = assign18800_e13504_d_n5;
        locals.var_tmf2_dn6 = assign18800_e13504_d_n6;
        locals.var_tmf2_dn7 = assign18800_e13504_d_n7;
        locals.var_tmf2_dn8 = assign18800_e13504_d_n8;
        locals.var_tmf2_dn9 = assign18800_e13504_d_n9;
        locals.var_tmf2_dn10 = assign18800_e13504_d_n10;
        locals.var_tmf2_dn11 = assign18800_e13504_d_n11;
        locals.var_tmf2_dn14 = assign18800_e13504_d_n14;

        let (assign18810_e13517, assign18810_e13517_d_n0, assign18810_e13517_d_n2, assign18810_e13517_d_n4, assign18810_e13517_d_n5, assign18810_e13517_d_n6, assign18810_e13517_d_n7, assign18810_e13517_d_n8, assign18810_e13517_d_n9, assign18810_e13517_d_n10, assign18810_e13517_d_n11, assign18810_e13517_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign18810_e13512: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18810_e13514: f64 = (assign18810_e13512 + locals.var_tmf2);
        let assign18810_e13515: f64 = (assign18810_e13514).sqrt();
        (assign18810_e13515, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18810_e13515)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18810_e13515)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign18810_e13515)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign18810_e13515)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign18810_e13515)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign18810_e13515)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign18810_e13515)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign18810_e13515)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign18810_e13515)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign18810_e13515)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign18810_e13515)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18810_e13517;
        locals.var_tmf2_dn0 = assign18810_e13517_d_n0;
        locals.var_tmf2_dn2 = assign18810_e13517_d_n2;
        locals.var_tmf2_dn4 = assign18810_e13517_d_n4;
        locals.var_tmf2_dn5 = assign18810_e13517_d_n5;
        locals.var_tmf2_dn6 = assign18810_e13517_d_n6;
        locals.var_tmf2_dn7 = assign18810_e13517_d_n7;
        locals.var_tmf2_dn8 = assign18810_e13517_d_n8;
        locals.var_tmf2_dn9 = assign18810_e13517_d_n9;
        locals.var_tmf2_dn10 = assign18810_e13517_d_n10;
        locals.var_tmf2_dn11 = assign18810_e13517_d_n11;
        locals.var_tmf2_dn14 = assign18810_e13517_d_n14;

        let (assign18820_e13531, assign18820_e13531_d_n0, assign18820_e13531_d_n2, assign18820_e13531_d_n4, assign18820_e13531_d_n5, assign18820_e13531_d_n6, assign18820_e13531_d_n7, assign18820_e13531_d_n8, assign18820_e13531_d_n9, assign18820_e13531_d_n10, assign18820_e13531_d_n11, assign18820_e13531_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign18820_e13527: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign18820_e13528: f64 = (1.0 + assign18820_e13527);
        let assign18820_e13529: f64 = (0.5 * assign18820_e13528);
        (assign18820_e13529, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign18820_e13531;
        locals.var_t6_dn0 = assign18820_e13531_d_n0;
        locals.var_t6_dn2 = assign18820_e13531_d_n2;
        locals.var_t6_dn4 = assign18820_e13531_d_n4;
        locals.var_t6_dn5 = assign18820_e13531_d_n5;
        locals.var_t6_dn6 = assign18820_e13531_d_n6;
        locals.var_t6_dn7 = assign18820_e13531_d_n7;
        locals.var_t6_dn8 = assign18820_e13531_d_n8;
        locals.var_t6_dn9 = assign18820_e13531_d_n9;
        locals.var_t6_dn10 = assign18820_e13531_d_n10;
        locals.var_t6_dn11 = assign18820_e13531_d_n11;
        locals.var_t6_dn14 = assign18820_e13531_d_n14;

        let (assign18830_e13545, assign18830_e13545_d_n0, assign18830_e13545_d_n2, assign18830_e13545_d_n4, assign18830_e13545_d_n5, assign18830_e13545_d_n6, assign18830_e13545_d_n7, assign18830_e13545_d_n8, assign18830_e13545_d_n9, assign18830_e13545_d_n10, assign18830_e13545_d_n11, assign18830_e13545_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign18830_e13541: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign18830_e13542: f64 = (0.5 * assign18830_e13541);
        let assign18830_e13543: f64 = (locals.var_t4 + assign18830_e13542);
        (assign18830_e13543, (locals.var_t4_dn0 + (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t4_dn2 + (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t4_dn4 + (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t4_dn5 + (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t4_dn6 + (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t4_dn7 + (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t4_dn8 + (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t4_dn9 + (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t4_dn10 + (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t4_dn11 + (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_t4_dn14 + (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign18830_e13545;
        locals.var_t5_dn0 = assign18830_e13545_d_n0;
        locals.var_t5_dn2 = assign18830_e13545_d_n2;
        locals.var_t5_dn4 = assign18830_e13545_d_n4;
        locals.var_t5_dn5 = assign18830_e13545_d_n5;
        locals.var_t5_dn6 = assign18830_e13545_d_n6;
        locals.var_t5_dn7 = assign18830_e13545_d_n7;
        locals.var_t5_dn8 = assign18830_e13545_d_n8;
        locals.var_t5_dn9 = assign18830_e13545_d_n9;
        locals.var_t5_dn10 = assign18830_e13545_d_n10;
        locals.var_t5_dn11 = assign18830_e13545_d_n11;
        locals.var_t5_dn14 = assign18830_e13545_d_n14;

        let (assign18840_e13561, assign18840_e13561_d_n0, assign18840_e13561_d_n2, assign18840_e13561_d_n4, assign18840_e13561_d_n5, assign18840_e13561_d_n6, assign18840_e13561_d_n7, assign18840_e13561_d_n8, assign18840_e13561_d_n9, assign18840_e13561_d_n10, assign18840_e13561_d_n11, assign18840_e13561_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign18840_e13554: f64 = (p.p98 + 1.0);
        let assign18840_e13555: f64 = (locals.var_t4 * assign18840_e13554);
        let assign18840_e13557: f64 = (assign18840_e13555 - locals.var_t5);
        let assign18840_e13559: f64 = (assign18840_e13557 - 5e-5);
        (assign18840_e13559, ((locals.var_t4_dn0 * assign18840_e13554) - locals.var_t5_dn0), ((locals.var_t4_dn2 * assign18840_e13554) - locals.var_t5_dn2), ((locals.var_t4_dn4 * assign18840_e13554) - locals.var_t5_dn4), ((locals.var_t4_dn5 * assign18840_e13554) - locals.var_t5_dn5), ((locals.var_t4_dn6 * assign18840_e13554) - locals.var_t5_dn6), ((locals.var_t4_dn7 * assign18840_e13554) - locals.var_t5_dn7), ((locals.var_t4_dn8 * assign18840_e13554) - locals.var_t5_dn8), ((locals.var_t4_dn9 * assign18840_e13554) - locals.var_t5_dn9), ((locals.var_t4_dn10 * assign18840_e13554) - locals.var_t5_dn10), ((locals.var_t4_dn11 * assign18840_e13554) - locals.var_t5_dn11), ((locals.var_t4_dn14 * assign18840_e13554) - locals.var_t5_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign18840_e13561;
        locals.var_tmf1_dn0 = assign18840_e13561_d_n0;
        locals.var_tmf1_dn2 = assign18840_e13561_d_n2;
        locals.var_tmf1_dn4 = assign18840_e13561_d_n4;
        locals.var_tmf1_dn5 = assign18840_e13561_d_n5;
        locals.var_tmf1_dn6 = assign18840_e13561_d_n6;
        locals.var_tmf1_dn7 = assign18840_e13561_d_n7;
        locals.var_tmf1_dn8 = assign18840_e13561_d_n8;
        locals.var_tmf1_dn9 = assign18840_e13561_d_n9;
        locals.var_tmf1_dn10 = assign18840_e13561_d_n10;
        locals.var_tmf1_dn11 = assign18840_e13561_d_n11;
        locals.var_tmf1_dn14 = assign18840_e13561_d_n14;

        let (assign18850_e13577, assign18850_e13577_d_n0, assign18850_e13577_d_n2, assign18850_e13577_d_n4, assign18850_e13577_d_n5, assign18850_e13577_d_n6, assign18850_e13577_d_n7, assign18850_e13577_d_n8, assign18850_e13577_d_n9, assign18850_e13577_d_n10, assign18850_e13577_d_n11, assign18850_e13577_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign18850_e13571: f64 = (p.p98 + 1.0);
        let assign18850_e13572: f64 = (locals.var_t4 * assign18850_e13571);
        let assign18850_e13573: f64 = (4.0 * assign18850_e13572);
        let assign18850_e13575: f64 = (assign18850_e13573 * 5e-5);
        (assign18850_e13575, ((4.0 * (locals.var_t4_dn0 * assign18850_e13571)) * 5e-5), ((4.0 * (locals.var_t4_dn2 * assign18850_e13571)) * 5e-5), ((4.0 * (locals.var_t4_dn4 * assign18850_e13571)) * 5e-5), ((4.0 * (locals.var_t4_dn5 * assign18850_e13571)) * 5e-5), ((4.0 * (locals.var_t4_dn6 * assign18850_e13571)) * 5e-5), ((4.0 * (locals.var_t4_dn7 * assign18850_e13571)) * 5e-5), ((4.0 * (locals.var_t4_dn8 * assign18850_e13571)) * 5e-5), ((4.0 * (locals.var_t4_dn9 * assign18850_e13571)) * 5e-5), ((4.0 * (locals.var_t4_dn10 * assign18850_e13571)) * 5e-5), ((4.0 * (locals.var_t4_dn11 * assign18850_e13571)) * 5e-5), ((4.0 * (locals.var_t4_dn14 * assign18850_e13571)) * 5e-5),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18850_e13577;
        locals.var_tmf2_dn0 = assign18850_e13577_d_n0;
        locals.var_tmf2_dn2 = assign18850_e13577_d_n2;
        locals.var_tmf2_dn4 = assign18850_e13577_d_n4;
        locals.var_tmf2_dn5 = assign18850_e13577_d_n5;
        locals.var_tmf2_dn6 = assign18850_e13577_d_n6;
        locals.var_tmf2_dn7 = assign18850_e13577_d_n7;
        locals.var_tmf2_dn8 = assign18850_e13577_d_n8;
        locals.var_tmf2_dn9 = assign18850_e13577_d_n9;
        locals.var_tmf2_dn10 = assign18850_e13577_d_n10;
        locals.var_tmf2_dn11 = assign18850_e13577_d_n11;
        locals.var_tmf2_dn14 = assign18850_e13577_d_n14;

        let (assign18860_e13591, assign18860_e13591_d_n0, assign18860_e13591_d_n2, assign18860_e13591_d_n4, assign18860_e13591_d_n5, assign18860_e13591_d_n6, assign18860_e13591_d_n7, assign18860_e13591_d_n8, assign18860_e13591_d_n9, assign18860_e13591_d_n10, assign18860_e13591_d_n11, assign18860_e13591_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let (assign18860_e13589, assign18860_e13589_d_n0, assign18860_e13589_d_n2, assign18860_e13589_d_n4, assign18860_e13589_d_n5, assign18860_e13589_d_n6, assign18860_e13589_d_n7, assign18860_e13589_d_n8, assign18860_e13589_d_n9, assign18860_e13589_d_n10, assign18860_e13589_d_n11, assign18860_e13589_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign18860_e13588: f64 = (-locals.var_tmf2);
                (assign18860_e13588, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign18860_e13589, assign18860_e13589_d_n0, assign18860_e13589_d_n2, assign18860_e13589_d_n4, assign18860_e13589_d_n5, assign18860_e13589_d_n6, assign18860_e13589_d_n7, assign18860_e13589_d_n8, assign18860_e13589_d_n9, assign18860_e13589_d_n10, assign18860_e13589_d_n11, assign18860_e13589_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18860_e13591;
        locals.var_tmf2_dn0 = assign18860_e13591_d_n0;
        locals.var_tmf2_dn2 = assign18860_e13591_d_n2;
        locals.var_tmf2_dn4 = assign18860_e13591_d_n4;
        locals.var_tmf2_dn5 = assign18860_e13591_d_n5;
        locals.var_tmf2_dn6 = assign18860_e13591_d_n6;
        locals.var_tmf2_dn7 = assign18860_e13591_d_n7;
        locals.var_tmf2_dn8 = assign18860_e13591_d_n8;
        locals.var_tmf2_dn9 = assign18860_e13591_d_n9;
        locals.var_tmf2_dn10 = assign18860_e13591_d_n10;
        locals.var_tmf2_dn11 = assign18860_e13591_d_n11;
        locals.var_tmf2_dn14 = assign18860_e13591_d_n14;

        let (assign18870_e13604, assign18870_e13604_d_n0, assign18870_e13604_d_n2, assign18870_e13604_d_n4, assign18870_e13604_d_n5, assign18870_e13604_d_n6, assign18870_e13604_d_n7, assign18870_e13604_d_n8, assign18870_e13604_d_n9, assign18870_e13604_d_n10, assign18870_e13604_d_n11, assign18870_e13604_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign18870_e13599: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18870_e13601: f64 = (assign18870_e13599 + locals.var_tmf2);
        let assign18870_e13602: f64 = (assign18870_e13601).sqrt();
        (assign18870_e13602, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18870_e13602)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18870_e13602)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign18870_e13602)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign18870_e13602)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign18870_e13602)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign18870_e13602)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign18870_e13602)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign18870_e13602)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign18870_e13602)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign18870_e13602)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign18870_e13602)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18870_e13604;
        locals.var_tmf2_dn0 = assign18870_e13604_d_n0;
        locals.var_tmf2_dn2 = assign18870_e13604_d_n2;
        locals.var_tmf2_dn4 = assign18870_e13604_d_n4;
        locals.var_tmf2_dn5 = assign18870_e13604_d_n5;
        locals.var_tmf2_dn6 = assign18870_e13604_d_n6;
        locals.var_tmf2_dn7 = assign18870_e13604_d_n7;
        locals.var_tmf2_dn8 = assign18870_e13604_d_n8;
        locals.var_tmf2_dn9 = assign18870_e13604_d_n9;
        locals.var_tmf2_dn10 = assign18870_e13604_d_n10;
        locals.var_tmf2_dn11 = assign18870_e13604_d_n11;
        locals.var_tmf2_dn14 = assign18870_e13604_d_n14;

        let (assign18880_e13618, assign18880_e13618_d_n0, assign18880_e13618_d_n2, assign18880_e13618_d_n4, assign18880_e13618_d_n5, assign18880_e13618_d_n6, assign18880_e13618_d_n7, assign18880_e13618_d_n8, assign18880_e13618_d_n9, assign18880_e13618_d_n10, assign18880_e13618_d_n11, assign18880_e13618_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign18880_e13614: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign18880_e13615: f64 = (1.0 + assign18880_e13614);
        let assign18880_e13616: f64 = (0.5 * assign18880_e13615);
        (assign18880_e13616, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign18880_e13618;
        locals.var_t6_dn0 = assign18880_e13618_d_n0;
        locals.var_t6_dn2 = assign18880_e13618_d_n2;
        locals.var_t6_dn4 = assign18880_e13618_d_n4;
        locals.var_t6_dn5 = assign18880_e13618_d_n5;
        locals.var_t6_dn6 = assign18880_e13618_d_n6;
        locals.var_t6_dn7 = assign18880_e13618_d_n7;
        locals.var_t6_dn8 = assign18880_e13618_d_n8;
        locals.var_t6_dn9 = assign18880_e13618_d_n9;
        locals.var_t6_dn10 = assign18880_e13618_d_n10;
        locals.var_t6_dn11 = assign18880_e13618_d_n11;
        locals.var_t6_dn14 = assign18880_e13618_d_n14;

        let (assign18890_e13636, assign18890_e13636_d_n0, assign18890_e13636_d_n2, assign18890_e13636_d_n4, assign18890_e13636_d_n5, assign18890_e13636_d_n6, assign18890_e13636_d_n7, assign18890_e13636_d_n8, assign18890_e13636_d_n9, assign18890_e13636_d_n10, assign18890_e13636_d_n11, assign18890_e13636_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign18890_e13627: f64 = (p.p98 + 1.0);
        let assign18890_e13628: f64 = (locals.var_t4 * assign18890_e13627);
        let assign18890_e13632: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign18890_e13633: f64 = (0.5 * assign18890_e13632);
        let assign18890_e13634: f64 = (assign18890_e13628 - assign18890_e13633);
        (assign18890_e13634, ((locals.var_t4_dn0 * assign18890_e13627) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((locals.var_t4_dn2 * assign18890_e13627) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((locals.var_t4_dn4 * assign18890_e13627) - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), ((locals.var_t4_dn5 * assign18890_e13627) - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), ((locals.var_t4_dn6 * assign18890_e13627) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((locals.var_t4_dn7 * assign18890_e13627) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((locals.var_t4_dn8 * assign18890_e13627) - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), ((locals.var_t4_dn9 * assign18890_e13627) - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), ((locals.var_t4_dn10 * assign18890_e13627) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((locals.var_t4_dn11 * assign18890_e13627) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((locals.var_t4_dn14 * assign18890_e13627) - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign18890_e13636;
        locals.var_t7_dn0 = assign18890_e13636_d_n0;
        locals.var_t7_dn2 = assign18890_e13636_d_n2;
        locals.var_t7_dn4 = assign18890_e13636_d_n4;
        locals.var_t7_dn5 = assign18890_e13636_d_n5;
        locals.var_t7_dn6 = assign18890_e13636_d_n6;
        locals.var_t7_dn7 = assign18890_e13636_d_n7;
        locals.var_t7_dn8 = assign18890_e13636_d_n8;
        locals.var_t7_dn9 = assign18890_e13636_d_n9;
        locals.var_t7_dn10 = assign18890_e13636_d_n10;
        locals.var_t7_dn11 = assign18890_e13636_d_n11;
        locals.var_t7_dn14 = assign18890_e13636_d_n14;

        let (assign18900_e13652, assign18900_e13652_d_n0, assign18900_e13652_d_n2, assign18900_e13652_d_n4, assign18900_e13652_d_n5, assign18900_e13652_d_n6, assign18900_e13652_d_n7, assign18900_e13652_d_n8, assign18900_e13652_d_n9, assign18900_e13652_d_n10, assign18900_e13652_d_n11, assign18900_e13652_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign18900_e13645: f64 = (locals.var_t1 * locals.var_t4);
        let assign18900_e13646: f64 = (locals.var_t7 + assign18900_e13645);
        let assign18900_e13648: f64 = assign18900_e13646;
        let assign18900_e13650: f64 = (assign18900_e13648 - 5e-5);
        (assign18900_e13650, (locals.var_t7_dn0 + ((locals.var_t1_dn0 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn0))), (locals.var_t7_dn2 + ((locals.var_t1_dn2 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn2))), (locals.var_t7_dn4 + ((locals.var_t1_dn4 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn4))), (locals.var_t7_dn5 + ((locals.var_t1_dn5 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn5))), (locals.var_t7_dn6 + ((locals.var_t1_dn6 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn6))), (locals.var_t7_dn7 + ((locals.var_t1_dn7 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn7))), (locals.var_t7_dn8 + ((locals.var_t1_dn8 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn8))), (locals.var_t7_dn9 + ((locals.var_t1_dn9 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn9))), (locals.var_t7_dn10 + ((locals.var_t1_dn10 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn10))), (locals.var_t7_dn11 + ((locals.var_t1_dn11 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn11))), (locals.var_t7_dn14 + ((locals.var_t1_dn14 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn14))),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign18900_e13652;
        locals.var_tmf1_dn0 = assign18900_e13652_d_n0;
        locals.var_tmf1_dn2 = assign18900_e13652_d_n2;
        locals.var_tmf1_dn4 = assign18900_e13652_d_n4;
        locals.var_tmf1_dn5 = assign18900_e13652_d_n5;
        locals.var_tmf1_dn6 = assign18900_e13652_d_n6;
        locals.var_tmf1_dn7 = assign18900_e13652_d_n7;
        locals.var_tmf1_dn8 = assign18900_e13652_d_n8;
        locals.var_tmf1_dn9 = assign18900_e13652_d_n9;
        locals.var_tmf1_dn10 = assign18900_e13652_d_n10;
        locals.var_tmf1_dn11 = assign18900_e13652_d_n11;
        locals.var_tmf1_dn14 = assign18900_e13652_d_n14;

        let (assign18910_e13664, assign18910_e13664_d_n0, assign18910_e13664_d_n2, assign18910_e13664_d_n4, assign18910_e13664_d_n5, assign18910_e13664_d_n6, assign18910_e13664_d_n7, assign18910_e13664_d_n8, assign18910_e13664_d_n9, assign18910_e13664_d_n10, assign18910_e13664_d_n11, assign18910_e13664_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18910_e13664;
        locals.var_tmf2_dn0 = assign18910_e13664_d_n0;
        locals.var_tmf2_dn2 = assign18910_e13664_d_n2;
        locals.var_tmf2_dn4 = assign18910_e13664_d_n4;
        locals.var_tmf2_dn5 = assign18910_e13664_d_n5;
        locals.var_tmf2_dn6 = assign18910_e13664_d_n6;
        locals.var_tmf2_dn7 = assign18910_e13664_d_n7;
        locals.var_tmf2_dn8 = assign18910_e13664_d_n8;
        locals.var_tmf2_dn9 = assign18910_e13664_d_n9;
        locals.var_tmf2_dn10 = assign18910_e13664_d_n10;
        locals.var_tmf2_dn11 = assign18910_e13664_d_n11;
        locals.var_tmf2_dn14 = assign18910_e13664_d_n14;

        let (assign18920_e13678, assign18920_e13678_d_n0, assign18920_e13678_d_n2, assign18920_e13678_d_n4, assign18920_e13678_d_n5, assign18920_e13678_d_n6, assign18920_e13678_d_n7, assign18920_e13678_d_n8, assign18920_e13678_d_n9, assign18920_e13678_d_n10, assign18920_e13678_d_n11, assign18920_e13678_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let (assign18920_e13676, assign18920_e13676_d_n0, assign18920_e13676_d_n2, assign18920_e13676_d_n4, assign18920_e13676_d_n5, assign18920_e13676_d_n6, assign18920_e13676_d_n7, assign18920_e13676_d_n8, assign18920_e13676_d_n9, assign18920_e13676_d_n10, assign18920_e13676_d_n11, assign18920_e13676_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign18920_e13675: f64 = (-locals.var_tmf2);
                (assign18920_e13675, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign18920_e13676, assign18920_e13676_d_n0, assign18920_e13676_d_n2, assign18920_e13676_d_n4, assign18920_e13676_d_n5, assign18920_e13676_d_n6, assign18920_e13676_d_n7, assign18920_e13676_d_n8, assign18920_e13676_d_n9, assign18920_e13676_d_n10, assign18920_e13676_d_n11, assign18920_e13676_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18920_e13678;
        locals.var_tmf2_dn0 = assign18920_e13678_d_n0;
        locals.var_tmf2_dn2 = assign18920_e13678_d_n2;
        locals.var_tmf2_dn4 = assign18920_e13678_d_n4;
        locals.var_tmf2_dn5 = assign18920_e13678_d_n5;
        locals.var_tmf2_dn6 = assign18920_e13678_d_n6;
        locals.var_tmf2_dn7 = assign18920_e13678_d_n7;
        locals.var_tmf2_dn8 = assign18920_e13678_d_n8;
        locals.var_tmf2_dn9 = assign18920_e13678_d_n9;
        locals.var_tmf2_dn10 = assign18920_e13678_d_n10;
        locals.var_tmf2_dn11 = assign18920_e13678_d_n11;
        locals.var_tmf2_dn14 = assign18920_e13678_d_n14;

        let (assign18930_e13691, assign18930_e13691_d_n0, assign18930_e13691_d_n2, assign18930_e13691_d_n4, assign18930_e13691_d_n5, assign18930_e13691_d_n6, assign18930_e13691_d_n7, assign18930_e13691_d_n8, assign18930_e13691_d_n9, assign18930_e13691_d_n10, assign18930_e13691_d_n11, assign18930_e13691_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign18930_e13686: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18930_e13688: f64 = (assign18930_e13686 + locals.var_tmf2);
        let assign18930_e13689: f64 = (assign18930_e13688).sqrt();
        (assign18930_e13689, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18930_e13689)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18930_e13689)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign18930_e13689)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign18930_e13689)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign18930_e13689)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign18930_e13689)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign18930_e13689)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign18930_e13689)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign18930_e13689)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign18930_e13689)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign18930_e13689)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18930_e13691;
        locals.var_tmf2_dn0 = assign18930_e13691_d_n0;
        locals.var_tmf2_dn2 = assign18930_e13691_d_n2;
        locals.var_tmf2_dn4 = assign18930_e13691_d_n4;
        locals.var_tmf2_dn5 = assign18930_e13691_d_n5;
        locals.var_tmf2_dn6 = assign18930_e13691_d_n6;
        locals.var_tmf2_dn7 = assign18930_e13691_d_n7;
        locals.var_tmf2_dn8 = assign18930_e13691_d_n8;
        locals.var_tmf2_dn9 = assign18930_e13691_d_n9;
        locals.var_tmf2_dn10 = assign18930_e13691_d_n10;
        locals.var_tmf2_dn11 = assign18930_e13691_d_n11;
        locals.var_tmf2_dn14 = assign18930_e13691_d_n14;

        let (assign18940_e13705, assign18940_e13705_d_n0, assign18940_e13705_d_n2, assign18940_e13705_d_n4, assign18940_e13705_d_n5, assign18940_e13705_d_n6, assign18940_e13705_d_n7, assign18940_e13705_d_n8, assign18940_e13705_d_n9, assign18940_e13705_d_n10, assign18940_e13705_d_n11, assign18940_e13705_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign18940_e13701: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign18940_e13702: f64 = (1.0 + assign18940_e13701);
        let assign18940_e13703: f64 = (0.5 * assign18940_e13702);
        (assign18940_e13703, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign18940_e13705;
        locals.var_t6_dn0 = assign18940_e13705_d_n0;
        locals.var_t6_dn2 = assign18940_e13705_d_n2;
        locals.var_t6_dn4 = assign18940_e13705_d_n4;
        locals.var_t6_dn5 = assign18940_e13705_d_n5;
        locals.var_t6_dn6 = assign18940_e13705_d_n6;
        locals.var_t6_dn7 = assign18940_e13705_d_n7;
        locals.var_t6_dn8 = assign18940_e13705_d_n8;
        locals.var_t6_dn9 = assign18940_e13705_d_n9;
        locals.var_t6_dn10 = assign18940_e13705_d_n10;
        locals.var_t6_dn11 = assign18940_e13705_d_n11;
        locals.var_t6_dn14 = assign18940_e13705_d_n14;

        let (assign18950_e13719, assign18950_e13719_d_n0, assign18950_e13719_d_n2, assign18950_e13719_d_n4, assign18950_e13719_d_n5, assign18950_e13719_d_n6, assign18950_e13719_d_n7, assign18950_e13719_d_n8, assign18950_e13719_d_n9, assign18950_e13719_d_n10, assign18950_e13719_d_n11, assign18950_e13719_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign18950_e13715: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign18950_e13716: f64 = (0.5 * assign18950_e13715);
        let assign18950_e13717: f64 = assign18950_e13716;
        (assign18950_e13717, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign18950_e13719;
        locals.var_t2_dn0 = assign18950_e13719_d_n0;
        locals.var_t2_dn2 = assign18950_e13719_d_n2;
        locals.var_t2_dn4 = assign18950_e13719_d_n4;
        locals.var_t2_dn5 = assign18950_e13719_d_n5;
        locals.var_t2_dn6 = assign18950_e13719_d_n6;
        locals.var_t2_dn7 = assign18950_e13719_d_n7;
        locals.var_t2_dn8 = assign18950_e13719_d_n8;
        locals.var_t2_dn9 = assign18950_e13719_d_n9;
        locals.var_t2_dn10 = assign18950_e13719_d_n10;
        locals.var_t2_dn11 = assign18950_e13719_d_n11;
        locals.var_t2_dn14 = assign18950_e13719_d_n14;

        let assign18960_e13726: f64 = if ((p.p39 == 0.0) || (p.p39 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard389 = assign18960_e13726;

        let (assign18970_e13746, assign18970_e13746_d_n0, assign18970_e13746_d_n2, assign18970_e13746_d_n4, assign18970_e13746_d_n5, assign18970_e13746_d_n6, assign18970_e13746_d_n7, assign18970_e13746_d_n8, assign18970_e13746_d_n9, assign18970_e13746_d_n10, assign18970_e13746_d_n11, assign18970_e13746_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard389 != 0.0)) {
        let assign18970_e13737: f64 = (locals.var_mks_rdvdtemp1 * locals.var_tdiff0);
        let assign18970_e13738: f64 = (locals.var_uc_rdvd + assign18970_e13737);
        let assign18970_e13741: f64 = (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2);
        let assign18970_e13742: f64 = (assign18970_e13738 + assign18970_e13741);
        let assign18970_e13744: f64 = (assign18970_e13742 * locals.var_t2);
        (assign18970_e13744, ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn0) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn0)) * locals.var_t2) + (assign18970_e13742 * locals.var_t2_dn0)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn2) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn2)) * locals.var_t2) + (assign18970_e13742 * locals.var_t2_dn2)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn4) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn4)) * locals.var_t2) + (assign18970_e13742 * locals.var_t2_dn4)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn5) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn5)) * locals.var_t2) + (assign18970_e13742 * locals.var_t2_dn5)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn6) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn6)) * locals.var_t2) + (assign18970_e13742 * locals.var_t2_dn6)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn7) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn7)) * locals.var_t2) + (assign18970_e13742 * locals.var_t2_dn7)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn8) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn8)) * locals.var_t2) + (assign18970_e13742 * locals.var_t2_dn8)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn9) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn9)) * locals.var_t2) + (assign18970_e13742 * locals.var_t2_dn9)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn10) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn10)) * locals.var_t2) + (assign18970_e13742 * locals.var_t2_dn10)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn11) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn11)) * locals.var_t2) + (assign18970_e13742 * locals.var_t2_dn11)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn14) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn14)) * locals.var_t2) + (assign18970_e13742 * locals.var_t2_dn14)),)
    } else {
        (locals.var_rdvde, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn11, locals.var_rdvde_dn14,)
    }
};
        locals.var_rdvde = assign18970_e13746;
        locals.var_rdvde_dn0 = assign18970_e13746_d_n0;
        locals.var_rdvde_dn2 = assign18970_e13746_d_n2;
        locals.var_rdvde_dn4 = assign18970_e13746_d_n4;
        locals.var_rdvde_dn5 = assign18970_e13746_d_n5;
        locals.var_rdvde_dn6 = assign18970_e13746_d_n6;
        locals.var_rdvde_dn7 = assign18970_e13746_d_n7;
        locals.var_rdvde_dn8 = assign18970_e13746_d_n8;
        locals.var_rdvde_dn9 = assign18970_e13746_d_n9;
        locals.var_rdvde_dn10 = assign18970_e13746_d_n10;
        locals.var_rdvde_dn11 = assign18970_e13746_d_n11;
        locals.var_rdvde_dn14 = assign18970_e13746_d_n14;

        let (assign18980_e13764, assign18980_e13764_d_n0, assign18980_e13764_d_n2, assign18980_e13764_d_n4, assign18980_e13764_d_n5, assign18980_e13764_d_n6, assign18980_e13764_d_n7, assign18980_e13764_d_n8, assign18980_e13764_d_n9, assign18980_e13764_d_n10, assign18980_e13764_d_n11, assign18980_e13764_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard389 != 0.0)) {
        let assign18980_e13757: f64 = (0.005 * locals.var_uc_rdvd);
        let assign18980_e13758: f64 = (locals.var_rdvde - assign18980_e13757);
        let assign18980_e13761: f64 = (0.01 * locals.var_uc_rdvd);
        let assign18980_e13762: f64 = (assign18980_e13758 - assign18980_e13761);
        (assign18980_e13762, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn11, locals.var_rdvde_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign18980_e13764;
        locals.var_tmf1_dn0 = assign18980_e13764_d_n0;
        locals.var_tmf1_dn2 = assign18980_e13764_d_n2;
        locals.var_tmf1_dn4 = assign18980_e13764_d_n4;
        locals.var_tmf1_dn5 = assign18980_e13764_d_n5;
        locals.var_tmf1_dn6 = assign18980_e13764_d_n6;
        locals.var_tmf1_dn7 = assign18980_e13764_d_n7;
        locals.var_tmf1_dn8 = assign18980_e13764_d_n8;
        locals.var_tmf1_dn9 = assign18980_e13764_d_n9;
        locals.var_tmf1_dn10 = assign18980_e13764_d_n10;
        locals.var_tmf1_dn11 = assign18980_e13764_d_n11;
        locals.var_tmf1_dn14 = assign18980_e13764_d_n14;

        let (assign18990_e13782, assign18990_e13782_d_n0, assign18990_e13782_d_n2, assign18990_e13782_d_n4, assign18990_e13782_d_n5, assign18990_e13782_d_n6, assign18990_e13782_d_n7, assign18990_e13782_d_n8, assign18990_e13782_d_n9, assign18990_e13782_d_n10, assign18990_e13782_d_n11, assign18990_e13782_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard389 != 0.0)) {
        let assign18990_e13775: f64 = (0.005 * locals.var_uc_rdvd);
        let assign18990_e13776: f64 = (4.0 * assign18990_e13775);
        let assign18990_e13779: f64 = (0.01 * locals.var_uc_rdvd);
        let assign18990_e13780: f64 = (assign18990_e13776 * assign18990_e13779);
        (assign18990_e13780, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18990_e13782;
        locals.var_tmf2_dn0 = assign18990_e13782_d_n0;
        locals.var_tmf2_dn2 = assign18990_e13782_d_n2;
        locals.var_tmf2_dn4 = assign18990_e13782_d_n4;
        locals.var_tmf2_dn5 = assign18990_e13782_d_n5;
        locals.var_tmf2_dn6 = assign18990_e13782_d_n6;
        locals.var_tmf2_dn7 = assign18990_e13782_d_n7;
        locals.var_tmf2_dn8 = assign18990_e13782_d_n8;
        locals.var_tmf2_dn9 = assign18990_e13782_d_n9;
        locals.var_tmf2_dn10 = assign18990_e13782_d_n10;
        locals.var_tmf2_dn11 = assign18990_e13782_d_n11;
        locals.var_tmf2_dn14 = assign18990_e13782_d_n14;

        let (assign19000_e13798, assign19000_e13798_d_n0, assign19000_e13798_d_n2, assign19000_e13798_d_n4, assign19000_e13798_d_n5, assign19000_e13798_d_n6, assign19000_e13798_d_n7, assign19000_e13798_d_n8, assign19000_e13798_d_n9, assign19000_e13798_d_n10, assign19000_e13798_d_n11, assign19000_e13798_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard389 != 0.0)) {
        let (assign19000_e13796, assign19000_e13796_d_n0, assign19000_e13796_d_n2, assign19000_e13796_d_n4, assign19000_e13796_d_n5, assign19000_e13796_d_n6, assign19000_e13796_d_n7, assign19000_e13796_d_n8, assign19000_e13796_d_n9, assign19000_e13796_d_n10, assign19000_e13796_d_n11, assign19000_e13796_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign19000_e13795: f64 = (-locals.var_tmf2);
                (assign19000_e13795, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign19000_e13796, assign19000_e13796_d_n0, assign19000_e13796_d_n2, assign19000_e13796_d_n4, assign19000_e13796_d_n5, assign19000_e13796_d_n6, assign19000_e13796_d_n7, assign19000_e13796_d_n8, assign19000_e13796_d_n9, assign19000_e13796_d_n10, assign19000_e13796_d_n11, assign19000_e13796_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19000_e13798;
        locals.var_tmf2_dn0 = assign19000_e13798_d_n0;
        locals.var_tmf2_dn2 = assign19000_e13798_d_n2;
        locals.var_tmf2_dn4 = assign19000_e13798_d_n4;
        locals.var_tmf2_dn5 = assign19000_e13798_d_n5;
        locals.var_tmf2_dn6 = assign19000_e13798_d_n6;
        locals.var_tmf2_dn7 = assign19000_e13798_d_n7;
        locals.var_tmf2_dn8 = assign19000_e13798_d_n8;
        locals.var_tmf2_dn9 = assign19000_e13798_d_n9;
        locals.var_tmf2_dn10 = assign19000_e13798_d_n10;
        locals.var_tmf2_dn11 = assign19000_e13798_d_n11;
        locals.var_tmf2_dn14 = assign19000_e13798_d_n14;

    }

    pub(super) fn stamp_transient_block_44(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign19010_e13813, assign19010_e13813_d_n0, assign19010_e13813_d_n2, assign19010_e13813_d_n4, assign19010_e13813_d_n5, assign19010_e13813_d_n6, assign19010_e13813_d_n7, assign19010_e13813_d_n8, assign19010_e13813_d_n9, assign19010_e13813_d_n10, assign19010_e13813_d_n11, assign19010_e13813_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard389 != 0.0)) {
        let assign19010_e13808: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign19010_e13810: f64 = (assign19010_e13808 + locals.var_tmf2);
        let assign19010_e13811: f64 = (assign19010_e13810).sqrt();
        (assign19010_e13811, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign19010_e13811)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign19010_e13811)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign19010_e13811)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign19010_e13811)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign19010_e13811)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign19010_e13811)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign19010_e13811)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign19010_e13811)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign19010_e13811)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign19010_e13811)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign19010_e13811)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19010_e13813;
        locals.var_tmf2_dn0 = assign19010_e13813_d_n0;
        locals.var_tmf2_dn2 = assign19010_e13813_d_n2;
        locals.var_tmf2_dn4 = assign19010_e13813_d_n4;
        locals.var_tmf2_dn5 = assign19010_e13813_d_n5;
        locals.var_tmf2_dn6 = assign19010_e13813_d_n6;
        locals.var_tmf2_dn7 = assign19010_e13813_d_n7;
        locals.var_tmf2_dn8 = assign19010_e13813_d_n8;
        locals.var_tmf2_dn9 = assign19010_e13813_d_n9;
        locals.var_tmf2_dn10 = assign19010_e13813_d_n10;
        locals.var_tmf2_dn11 = assign19010_e13813_d_n11;
        locals.var_tmf2_dn14 = assign19010_e13813_d_n14;

        let (assign19020_e13829, assign19020_e13829_d_n0, assign19020_e13829_d_n2, assign19020_e13829_d_n4, assign19020_e13829_d_n5, assign19020_e13829_d_n6, assign19020_e13829_d_n7, assign19020_e13829_d_n8, assign19020_e13829_d_n9, assign19020_e13829_d_n10, assign19020_e13829_d_n11, assign19020_e13829_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard389 != 0.0)) {
        let assign19020_e13825: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign19020_e13826: f64 = (1.0 + assign19020_e13825);
        let assign19020_e13827: f64 = (0.5 * assign19020_e13826);
        (assign19020_e13827, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign19020_e13829;
        locals.var_t0_dn0 = assign19020_e13829_d_n0;
        locals.var_t0_dn2 = assign19020_e13829_d_n2;
        locals.var_t0_dn4 = assign19020_e13829_d_n4;
        locals.var_t0_dn5 = assign19020_e13829_d_n5;
        locals.var_t0_dn6 = assign19020_e13829_d_n6;
        locals.var_t0_dn7 = assign19020_e13829_d_n7;
        locals.var_t0_dn8 = assign19020_e13829_d_n8;
        locals.var_t0_dn9 = assign19020_e13829_d_n9;
        locals.var_t0_dn10 = assign19020_e13829_d_n10;
        locals.var_t0_dn11 = assign19020_e13829_d_n11;
        locals.var_t0_dn14 = assign19020_e13829_d_n14;

        let (assign19030_e13847, assign19030_e13847_d_n0, assign19030_e13847_d_n2, assign19030_e13847_d_n4, assign19030_e13847_d_n5, assign19030_e13847_d_n6, assign19030_e13847_d_n7, assign19030_e13847_d_n8, assign19030_e13847_d_n9, assign19030_e13847_d_n10, assign19030_e13847_d_n11, assign19030_e13847_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard389 != 0.0)) {
        let assign19030_e13839: f64 = (0.005 * locals.var_uc_rdvd);
        let assign19030_e13843: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign19030_e13844: f64 = (0.5 * assign19030_e13843);
        let assign19030_e13845: f64 = (assign19030_e13839 + assign19030_e13844);
        (assign19030_e13845, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_rdvde, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn11, locals.var_rdvde_dn14,)
    }
};
        locals.var_rdvde = assign19030_e13847;
        locals.var_rdvde_dn0 = assign19030_e13847_d_n0;
        locals.var_rdvde_dn2 = assign19030_e13847_d_n2;
        locals.var_rdvde_dn4 = assign19030_e13847_d_n4;
        locals.var_rdvde_dn5 = assign19030_e13847_d_n5;
        locals.var_rdvde_dn6 = assign19030_e13847_d_n6;
        locals.var_rdvde_dn7 = assign19030_e13847_d_n7;
        locals.var_rdvde_dn8 = assign19030_e13847_d_n8;
        locals.var_rdvde_dn9 = assign19030_e13847_d_n9;
        locals.var_rdvde_dn10 = assign19030_e13847_d_n10;
        locals.var_rdvde_dn11 = assign19030_e13847_d_n11;
        locals.var_rdvde_dn14 = assign19030_e13847_d_n14;

        let (assign19040_e13868, assign19040_e13868_d_n0, assign19040_e13868_d_n2, assign19040_e13868_d_n4, assign19040_e13868_d_n5, assign19040_e13868_d_n6, assign19040_e13868_d_n7, assign19040_e13868_d_n8, assign19040_e13868_d_n9, assign19040_e13868_d_n10, assign19040_e13868_d_n11, assign19040_e13868_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard389 == 0.0)) {
        let assign19040_e13859: f64 = (locals.var_mks_rdvdtemp1 * locals.var_tdiff);
        let assign19040_e13860: f64 = (locals.var_uc_rdvd + assign19040_e13859);
        let assign19040_e13863: f64 = (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2);
        let assign19040_e13864: f64 = (assign19040_e13860 + assign19040_e13863);
        let assign19040_e13866: f64 = (assign19040_e13864 * locals.var_t2);
        (assign19040_e13866, ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn0) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn0)) * locals.var_t2) + (assign19040_e13864 * locals.var_t2_dn0)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn2) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn2)) * locals.var_t2) + (assign19040_e13864 * locals.var_t2_dn2)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn4) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn4)) * locals.var_t2) + (assign19040_e13864 * locals.var_t2_dn4)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn5) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn5)) * locals.var_t2) + (assign19040_e13864 * locals.var_t2_dn5)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn6) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn6)) * locals.var_t2) + (assign19040_e13864 * locals.var_t2_dn6)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn7) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn7)) * locals.var_t2) + (assign19040_e13864 * locals.var_t2_dn7)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn8) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn8)) * locals.var_t2) + (assign19040_e13864 * locals.var_t2_dn8)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn9) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn9)) * locals.var_t2) + (assign19040_e13864 * locals.var_t2_dn9)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn10) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn10)) * locals.var_t2) + (assign19040_e13864 * locals.var_t2_dn10)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn11) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn11)) * locals.var_t2) + (assign19040_e13864 * locals.var_t2_dn11)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn14) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn14)) * locals.var_t2) + (assign19040_e13864 * locals.var_t2_dn14)),)
    } else {
        (locals.var_rdvde, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn11, locals.var_rdvde_dn14,)
    }
};
        locals.var_rdvde = assign19040_e13868;
        locals.var_rdvde_dn0 = assign19040_e13868_d_n0;
        locals.var_rdvde_dn2 = assign19040_e13868_d_n2;
        locals.var_rdvde_dn4 = assign19040_e13868_d_n4;
        locals.var_rdvde_dn5 = assign19040_e13868_d_n5;
        locals.var_rdvde_dn6 = assign19040_e13868_d_n6;
        locals.var_rdvde_dn7 = assign19040_e13868_d_n7;
        locals.var_rdvde_dn8 = assign19040_e13868_d_n8;
        locals.var_rdvde_dn9 = assign19040_e13868_d_n9;
        locals.var_rdvde_dn10 = assign19040_e13868_d_n10;
        locals.var_rdvde_dn11 = assign19040_e13868_d_n11;
        locals.var_rdvde_dn14 = assign19040_e13868_d_n14;

        let (assign19050_e13887, assign19050_e13887_d_n0, assign19050_e13887_d_n2, assign19050_e13887_d_n4, assign19050_e13887_d_n5, assign19050_e13887_d_n6, assign19050_e13887_d_n7, assign19050_e13887_d_n8, assign19050_e13887_d_n9, assign19050_e13887_d_n10, assign19050_e13887_d_n11, assign19050_e13887_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard389 == 0.0)) {
        let assign19050_e13880: f64 = (0.005 * locals.var_uc_rdvd);
        let assign19050_e13881: f64 = (locals.var_rdvde - assign19050_e13880);
        let assign19050_e13884: f64 = (0.01 * locals.var_uc_rdvd);
        let assign19050_e13885: f64 = (assign19050_e13881 - assign19050_e13884);
        (assign19050_e13885, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn11, locals.var_rdvde_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign19050_e13887;
        locals.var_tmf1_dn0 = assign19050_e13887_d_n0;
        locals.var_tmf1_dn2 = assign19050_e13887_d_n2;
        locals.var_tmf1_dn4 = assign19050_e13887_d_n4;
        locals.var_tmf1_dn5 = assign19050_e13887_d_n5;
        locals.var_tmf1_dn6 = assign19050_e13887_d_n6;
        locals.var_tmf1_dn7 = assign19050_e13887_d_n7;
        locals.var_tmf1_dn8 = assign19050_e13887_d_n8;
        locals.var_tmf1_dn9 = assign19050_e13887_d_n9;
        locals.var_tmf1_dn10 = assign19050_e13887_d_n10;
        locals.var_tmf1_dn11 = assign19050_e13887_d_n11;
        locals.var_tmf1_dn14 = assign19050_e13887_d_n14;

        let (assign19060_e13906, assign19060_e13906_d_n0, assign19060_e13906_d_n2, assign19060_e13906_d_n4, assign19060_e13906_d_n5, assign19060_e13906_d_n6, assign19060_e13906_d_n7, assign19060_e13906_d_n8, assign19060_e13906_d_n9, assign19060_e13906_d_n10, assign19060_e13906_d_n11, assign19060_e13906_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard389 == 0.0)) {
        let assign19060_e13899: f64 = (0.005 * locals.var_uc_rdvd);
        let assign19060_e13900: f64 = (4.0 * assign19060_e13899);
        let assign19060_e13903: f64 = (0.01 * locals.var_uc_rdvd);
        let assign19060_e13904: f64 = (assign19060_e13900 * assign19060_e13903);
        (assign19060_e13904, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19060_e13906;
        locals.var_tmf2_dn0 = assign19060_e13906_d_n0;
        locals.var_tmf2_dn2 = assign19060_e13906_d_n2;
        locals.var_tmf2_dn4 = assign19060_e13906_d_n4;
        locals.var_tmf2_dn5 = assign19060_e13906_d_n5;
        locals.var_tmf2_dn6 = assign19060_e13906_d_n6;
        locals.var_tmf2_dn7 = assign19060_e13906_d_n7;
        locals.var_tmf2_dn8 = assign19060_e13906_d_n8;
        locals.var_tmf2_dn9 = assign19060_e13906_d_n9;
        locals.var_tmf2_dn10 = assign19060_e13906_d_n10;
        locals.var_tmf2_dn11 = assign19060_e13906_d_n11;
        locals.var_tmf2_dn14 = assign19060_e13906_d_n14;

        let (assign19070_e13923, assign19070_e13923_d_n0, assign19070_e13923_d_n2, assign19070_e13923_d_n4, assign19070_e13923_d_n5, assign19070_e13923_d_n6, assign19070_e13923_d_n7, assign19070_e13923_d_n8, assign19070_e13923_d_n9, assign19070_e13923_d_n10, assign19070_e13923_d_n11, assign19070_e13923_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard389 == 0.0)) {
        let (assign19070_e13921, assign19070_e13921_d_n0, assign19070_e13921_d_n2, assign19070_e13921_d_n4, assign19070_e13921_d_n5, assign19070_e13921_d_n6, assign19070_e13921_d_n7, assign19070_e13921_d_n8, assign19070_e13921_d_n9, assign19070_e13921_d_n10, assign19070_e13921_d_n11, assign19070_e13921_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign19070_e13920: f64 = (-locals.var_tmf2);
                (assign19070_e13920, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign19070_e13921, assign19070_e13921_d_n0, assign19070_e13921_d_n2, assign19070_e13921_d_n4, assign19070_e13921_d_n5, assign19070_e13921_d_n6, assign19070_e13921_d_n7, assign19070_e13921_d_n8, assign19070_e13921_d_n9, assign19070_e13921_d_n10, assign19070_e13921_d_n11, assign19070_e13921_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19070_e13923;
        locals.var_tmf2_dn0 = assign19070_e13923_d_n0;
        locals.var_tmf2_dn2 = assign19070_e13923_d_n2;
        locals.var_tmf2_dn4 = assign19070_e13923_d_n4;
        locals.var_tmf2_dn5 = assign19070_e13923_d_n5;
        locals.var_tmf2_dn6 = assign19070_e13923_d_n6;
        locals.var_tmf2_dn7 = assign19070_e13923_d_n7;
        locals.var_tmf2_dn8 = assign19070_e13923_d_n8;
        locals.var_tmf2_dn9 = assign19070_e13923_d_n9;
        locals.var_tmf2_dn10 = assign19070_e13923_d_n10;
        locals.var_tmf2_dn11 = assign19070_e13923_d_n11;
        locals.var_tmf2_dn14 = assign19070_e13923_d_n14;

        let (assign19080_e13939, assign19080_e13939_d_n0, assign19080_e13939_d_n2, assign19080_e13939_d_n4, assign19080_e13939_d_n5, assign19080_e13939_d_n6, assign19080_e13939_d_n7, assign19080_e13939_d_n8, assign19080_e13939_d_n9, assign19080_e13939_d_n10, assign19080_e13939_d_n11, assign19080_e13939_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard389 == 0.0)) {
        let assign19080_e13934: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign19080_e13936: f64 = (assign19080_e13934 + locals.var_tmf2);
        let assign19080_e13937: f64 = (assign19080_e13936).sqrt();
        (assign19080_e13937, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign19080_e13937)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign19080_e13937)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign19080_e13937)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign19080_e13937)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign19080_e13937)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign19080_e13937)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign19080_e13937)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign19080_e13937)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign19080_e13937)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign19080_e13937)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign19080_e13937)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19080_e13939;
        locals.var_tmf2_dn0 = assign19080_e13939_d_n0;
        locals.var_tmf2_dn2 = assign19080_e13939_d_n2;
        locals.var_tmf2_dn4 = assign19080_e13939_d_n4;
        locals.var_tmf2_dn5 = assign19080_e13939_d_n5;
        locals.var_tmf2_dn6 = assign19080_e13939_d_n6;
        locals.var_tmf2_dn7 = assign19080_e13939_d_n7;
        locals.var_tmf2_dn8 = assign19080_e13939_d_n8;
        locals.var_tmf2_dn9 = assign19080_e13939_d_n9;
        locals.var_tmf2_dn10 = assign19080_e13939_d_n10;
        locals.var_tmf2_dn11 = assign19080_e13939_d_n11;
        locals.var_tmf2_dn14 = assign19080_e13939_d_n14;

        let (assign19090_e13956, assign19090_e13956_d_n0, assign19090_e13956_d_n2, assign19090_e13956_d_n4, assign19090_e13956_d_n5, assign19090_e13956_d_n6, assign19090_e13956_d_n7, assign19090_e13956_d_n8, assign19090_e13956_d_n9, assign19090_e13956_d_n10, assign19090_e13956_d_n11, assign19090_e13956_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard389 == 0.0)) {
        let assign19090_e13952: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign19090_e13953: f64 = (1.0 + assign19090_e13952);
        let assign19090_e13954: f64 = (0.5 * assign19090_e13953);
        (assign19090_e13954, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign19090_e13956;
        locals.var_t0_dn0 = assign19090_e13956_d_n0;
        locals.var_t0_dn2 = assign19090_e13956_d_n2;
        locals.var_t0_dn4 = assign19090_e13956_d_n4;
        locals.var_t0_dn5 = assign19090_e13956_d_n5;
        locals.var_t0_dn6 = assign19090_e13956_d_n6;
        locals.var_t0_dn7 = assign19090_e13956_d_n7;
        locals.var_t0_dn8 = assign19090_e13956_d_n8;
        locals.var_t0_dn9 = assign19090_e13956_d_n9;
        locals.var_t0_dn10 = assign19090_e13956_d_n10;
        locals.var_t0_dn11 = assign19090_e13956_d_n11;
        locals.var_t0_dn14 = assign19090_e13956_d_n14;

        let (assign19100_e13975, assign19100_e13975_d_n0, assign19100_e13975_d_n2, assign19100_e13975_d_n4, assign19100_e13975_d_n5, assign19100_e13975_d_n6, assign19100_e13975_d_n7, assign19100_e13975_d_n8, assign19100_e13975_d_n9, assign19100_e13975_d_n10, assign19100_e13975_d_n11, assign19100_e13975_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard389 == 0.0)) {
        let assign19100_e13967: f64 = (0.005 * locals.var_uc_rdvd);
        let assign19100_e13971: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign19100_e13972: f64 = (0.5 * assign19100_e13971);
        let assign19100_e13973: f64 = (assign19100_e13967 + assign19100_e13972);
        (assign19100_e13973, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_rdvde, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn11, locals.var_rdvde_dn14,)
    }
};
        locals.var_rdvde = assign19100_e13975;
        locals.var_rdvde_dn0 = assign19100_e13975_d_n0;
        locals.var_rdvde_dn2 = assign19100_e13975_d_n2;
        locals.var_rdvde_dn4 = assign19100_e13975_d_n4;
        locals.var_rdvde_dn5 = assign19100_e13975_d_n5;
        locals.var_rdvde_dn6 = assign19100_e13975_d_n6;
        locals.var_rdvde_dn7 = assign19100_e13975_d_n7;
        locals.var_rdvde_dn8 = assign19100_e13975_d_n8;
        locals.var_rdvde_dn9 = assign19100_e13975_d_n9;
        locals.var_rdvde_dn10 = assign19100_e13975_d_n10;
        locals.var_rdvde_dn11 = assign19100_e13975_d_n11;
        locals.var_rdvde_dn14 = assign19100_e13975_d_n14;

        let (assign19110_e13999, assign19110_e13999_d_n0, assign19110_e13999_d_n2, assign19110_e13999_d_n4, assign19110_e13999_d_n5, assign19110_e13999_d_n6, assign19110_e13999_d_n7, assign19110_e13999_d_n8, assign19110_e13999_d_n9, assign19110_e13999_d_n10, assign19110_e13999_d_n11, assign19110_e13999_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign19110_e13984: f64 = (p.p69 * locals.var_uc_rdslp1);
        let assign19110_e13986: f64 = (assign19110_e13984 * 1000000.0);
        let assign19110_e13988: f64 = (assign19110_e13986 + locals.var_uc_rdict1);
        let assign19110_e13989: f64 = (locals.var_rdvdtemp0 * assign19110_e13988);
        let assign19110_e13992: f64 = (p.p70 * p.p100);
        let assign19110_e13994: f64 = (assign19110_e13992 * 1000000.0);
        let assign19110_e13996: f64 = (assign19110_e13994 + p.p101);
        let assign19110_e13997: f64 = (assign19110_e13989 * assign19110_e13996);
        (assign19110_e13997, ((locals.var_rdvdtemp0_dn0 * assign19110_e13988) * assign19110_e13996), ((locals.var_rdvdtemp0_dn2 * assign19110_e13988) * assign19110_e13996), ((locals.var_rdvdtemp0_dn4 * assign19110_e13988) * assign19110_e13996), ((locals.var_rdvdtemp0_dn5 * assign19110_e13988) * assign19110_e13996), ((locals.var_rdvdtemp0_dn6 * assign19110_e13988) * assign19110_e13996), ((locals.var_rdvdtemp0_dn7 * assign19110_e13988) * assign19110_e13996), ((locals.var_rdvdtemp0_dn8 * assign19110_e13988) * assign19110_e13996), ((locals.var_rdvdtemp0_dn9 * assign19110_e13988) * assign19110_e13996), ((locals.var_rdvdtemp0_dn10 * assign19110_e13988) * assign19110_e13996), ((locals.var_rdvdtemp0_dn11 * assign19110_e13988) * assign19110_e13996), ((locals.var_rdvdtemp0_dn14 * assign19110_e13988) * assign19110_e13996),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign19110_e13999;
        locals.var_t4_dn0 = assign19110_e13999_d_n0;
        locals.var_t4_dn2 = assign19110_e13999_d_n2;
        locals.var_t4_dn4 = assign19110_e13999_d_n4;
        locals.var_t4_dn5 = assign19110_e13999_d_n5;
        locals.var_t4_dn6 = assign19110_e13999_d_n6;
        locals.var_t4_dn7 = assign19110_e13999_d_n7;
        locals.var_t4_dn8 = assign19110_e13999_d_n8;
        locals.var_t4_dn9 = assign19110_e13999_d_n9;
        locals.var_t4_dn10 = assign19110_e13999_d_n10;
        locals.var_t4_dn11 = assign19110_e13999_d_n11;
        locals.var_t4_dn14 = assign19110_e13999_d_n14;

        let (assign19120_e14013, assign19120_e14013_d_n0, assign19120_e14013_d_n2, assign19120_e14013_d_n4, assign19120_e14013_d_n5, assign19120_e14013_d_n6, assign19120_e14013_d_n7, assign19120_e14013_d_n8, assign19120_e14013_d_n9, assign19120_e14013_d_n10, assign19120_e14013_d_n11, assign19120_e14013_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign19120_e14007: f64 = (1.0 - locals.var_uc_rdov13);
        let assign19120_e14009: f64 = (assign19120_e14007 * p.p66);
        let assign19120_e14011: f64 = (assign19120_e14009 * 1000000.0);
        (assign19120_e14011, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign19120_e14013;
        locals.var_t1_dn0 = assign19120_e14013_d_n0;
        locals.var_t1_dn2 = assign19120_e14013_d_n2;
        locals.var_t1_dn4 = assign19120_e14013_d_n4;
        locals.var_t1_dn5 = assign19120_e14013_d_n5;
        locals.var_t1_dn6 = assign19120_e14013_d_n6;
        locals.var_t1_dn7 = assign19120_e14013_d_n7;
        locals.var_t1_dn8 = assign19120_e14013_d_n8;
        locals.var_t1_dn9 = assign19120_e14013_d_n9;
        locals.var_t1_dn10 = assign19120_e14013_d_n10;
        locals.var_t1_dn11 = assign19120_e14013_d_n11;
        locals.var_t1_dn14 = assign19120_e14013_d_n14;

        let (assign19130_e14029, assign19130_e14029_d_n0, assign19130_e14029_d_n2, assign19130_e14029_d_n4, assign19130_e14029_d_n5, assign19130_e14029_d_n6, assign19130_e14029_d_n7, assign19130_e14029_d_n8, assign19130_e14029_d_n9, assign19130_e14029_d_n10, assign19130_e14029_d_n11, assign19130_e14029_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign19130_e14021: f64 = (locals.var_t8 * p.p66);
        let assign19130_e14023: f64 = (assign19130_e14021 * 1000000.0);
        let assign19130_e14025: f64 = (assign19130_e14023 + 1.0);
        let assign19130_e14027: f64 = (assign19130_e14025 + p.p98);
        (assign19130_e14027, ((locals.var_t8_dn0 * p.p66) * 1000000.0), ((locals.var_t8_dn2 * p.p66) * 1000000.0), ((locals.var_t8_dn4 * p.p66) * 1000000.0), ((locals.var_t8_dn5 * p.p66) * 1000000.0), ((locals.var_t8_dn6 * p.p66) * 1000000.0), ((locals.var_t8_dn7 * p.p66) * 1000000.0), ((locals.var_t8_dn8 * p.p66) * 1000000.0), ((locals.var_t8_dn9 * p.p66) * 1000000.0), ((locals.var_t8_dn10 * p.p66) * 1000000.0), ((locals.var_t8_dn11 * p.p66) * 1000000.0), ((locals.var_t8_dn14 * p.p66) * 1000000.0),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign19130_e14029;
        locals.var_t3_dn0 = assign19130_e14029_d_n0;
        locals.var_t3_dn2 = assign19130_e14029_d_n2;
        locals.var_t3_dn4 = assign19130_e14029_d_n4;
        locals.var_t3_dn5 = assign19130_e14029_d_n5;
        locals.var_t3_dn6 = assign19130_e14029_d_n6;
        locals.var_t3_dn7 = assign19130_e14029_d_n7;
        locals.var_t3_dn8 = assign19130_e14029_d_n8;
        locals.var_t3_dn9 = assign19130_e14029_d_n9;
        locals.var_t3_dn10 = assign19130_e14029_d_n10;
        locals.var_t3_dn11 = assign19130_e14029_d_n11;
        locals.var_t3_dn14 = assign19130_e14029_d_n14;

        let (assign19140_e14043, assign19140_e14043_d_n0, assign19140_e14043_d_n2, assign19140_e14043_d_n4, assign19140_e14043_d_n5, assign19140_e14043_d_n6, assign19140_e14043_d_n7, assign19140_e14043_d_n8, assign19140_e14043_d_n9, assign19140_e14043_d_n10, assign19140_e14043_d_n11, assign19140_e14043_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign19140_e14037: f64 = (locals.var_t3 * locals.var_t4);
        let assign19140_e14039: f64 = (assign19140_e14037 - locals.var_t4);
        let assign19140_e14041: f64 = (assign19140_e14039 - 0.01);
        (assign19140_e14041, (((locals.var_t3_dn0 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn0)) - locals.var_t4_dn0), (((locals.var_t3_dn2 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn2)) - locals.var_t4_dn2), (((locals.var_t3_dn4 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn4)) - locals.var_t4_dn4), (((locals.var_t3_dn5 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn5)) - locals.var_t4_dn5), (((locals.var_t3_dn6 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn6)) - locals.var_t4_dn6), (((locals.var_t3_dn7 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn7)) - locals.var_t4_dn7), (((locals.var_t3_dn8 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn8)) - locals.var_t4_dn8), (((locals.var_t3_dn9 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn9)) - locals.var_t4_dn9), (((locals.var_t3_dn10 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn10)) - locals.var_t4_dn10), (((locals.var_t3_dn11 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn11)) - locals.var_t4_dn11), (((locals.var_t3_dn14 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn14)) - locals.var_t4_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign19140_e14043;
        locals.var_tmf1_dn0 = assign19140_e14043_d_n0;
        locals.var_tmf1_dn2 = assign19140_e14043_d_n2;
        locals.var_tmf1_dn4 = assign19140_e14043_d_n4;
        locals.var_tmf1_dn5 = assign19140_e14043_d_n5;
        locals.var_tmf1_dn6 = assign19140_e14043_d_n6;
        locals.var_tmf1_dn7 = assign19140_e14043_d_n7;
        locals.var_tmf1_dn8 = assign19140_e14043_d_n8;
        locals.var_tmf1_dn9 = assign19140_e14043_d_n9;
        locals.var_tmf1_dn10 = assign19140_e14043_d_n10;
        locals.var_tmf1_dn11 = assign19140_e14043_d_n11;
        locals.var_tmf1_dn14 = assign19140_e14043_d_n14;

        let (assign19150_e14055, assign19150_e14055_d_n0, assign19150_e14055_d_n2, assign19150_e14055_d_n4, assign19150_e14055_d_n5, assign19150_e14055_d_n6, assign19150_e14055_d_n7, assign19150_e14055_d_n8, assign19150_e14055_d_n9, assign19150_e14055_d_n10, assign19150_e14055_d_n11, assign19150_e14055_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign19150_e14051: f64 = (4.0 * locals.var_t4);
        let assign19150_e14053: f64 = (assign19150_e14051 * 0.01);
        (assign19150_e14053, ((4.0 * locals.var_t4_dn0) * 0.01), ((4.0 * locals.var_t4_dn2) * 0.01), ((4.0 * locals.var_t4_dn4) * 0.01), ((4.0 * locals.var_t4_dn5) * 0.01), ((4.0 * locals.var_t4_dn6) * 0.01), ((4.0 * locals.var_t4_dn7) * 0.01), ((4.0 * locals.var_t4_dn8) * 0.01), ((4.0 * locals.var_t4_dn9) * 0.01), ((4.0 * locals.var_t4_dn10) * 0.01), ((4.0 * locals.var_t4_dn11) * 0.01), ((4.0 * locals.var_t4_dn14) * 0.01),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19150_e14055;
        locals.var_tmf2_dn0 = assign19150_e14055_d_n0;
        locals.var_tmf2_dn2 = assign19150_e14055_d_n2;
        locals.var_tmf2_dn4 = assign19150_e14055_d_n4;
        locals.var_tmf2_dn5 = assign19150_e14055_d_n5;
        locals.var_tmf2_dn6 = assign19150_e14055_d_n6;
        locals.var_tmf2_dn7 = assign19150_e14055_d_n7;
        locals.var_tmf2_dn8 = assign19150_e14055_d_n8;
        locals.var_tmf2_dn9 = assign19150_e14055_d_n9;
        locals.var_tmf2_dn10 = assign19150_e14055_d_n10;
        locals.var_tmf2_dn11 = assign19150_e14055_d_n11;
        locals.var_tmf2_dn14 = assign19150_e14055_d_n14;

        let (assign19160_e14069, assign19160_e14069_d_n0, assign19160_e14069_d_n2, assign19160_e14069_d_n4, assign19160_e14069_d_n5, assign19160_e14069_d_n6, assign19160_e14069_d_n7, assign19160_e14069_d_n8, assign19160_e14069_d_n9, assign19160_e14069_d_n10, assign19160_e14069_d_n11, assign19160_e14069_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let (assign19160_e14067, assign19160_e14067_d_n0, assign19160_e14067_d_n2, assign19160_e14067_d_n4, assign19160_e14067_d_n5, assign19160_e14067_d_n6, assign19160_e14067_d_n7, assign19160_e14067_d_n8, assign19160_e14067_d_n9, assign19160_e14067_d_n10, assign19160_e14067_d_n11, assign19160_e14067_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign19160_e14066: f64 = (-locals.var_tmf2);
                (assign19160_e14066, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign19160_e14067, assign19160_e14067_d_n0, assign19160_e14067_d_n2, assign19160_e14067_d_n4, assign19160_e14067_d_n5, assign19160_e14067_d_n6, assign19160_e14067_d_n7, assign19160_e14067_d_n8, assign19160_e14067_d_n9, assign19160_e14067_d_n10, assign19160_e14067_d_n11, assign19160_e14067_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19160_e14069;
        locals.var_tmf2_dn0 = assign19160_e14069_d_n0;
        locals.var_tmf2_dn2 = assign19160_e14069_d_n2;
        locals.var_tmf2_dn4 = assign19160_e14069_d_n4;
        locals.var_tmf2_dn5 = assign19160_e14069_d_n5;
        locals.var_tmf2_dn6 = assign19160_e14069_d_n6;
        locals.var_tmf2_dn7 = assign19160_e14069_d_n7;
        locals.var_tmf2_dn8 = assign19160_e14069_d_n8;
        locals.var_tmf2_dn9 = assign19160_e14069_d_n9;
        locals.var_tmf2_dn10 = assign19160_e14069_d_n10;
        locals.var_tmf2_dn11 = assign19160_e14069_d_n11;
        locals.var_tmf2_dn14 = assign19160_e14069_d_n14;

        let (assign19170_e14082, assign19170_e14082_d_n0, assign19170_e14082_d_n2, assign19170_e14082_d_n4, assign19170_e14082_d_n5, assign19170_e14082_d_n6, assign19170_e14082_d_n7, assign19170_e14082_d_n8, assign19170_e14082_d_n9, assign19170_e14082_d_n10, assign19170_e14082_d_n11, assign19170_e14082_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign19170_e14077: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign19170_e14079: f64 = (assign19170_e14077 + locals.var_tmf2);
        let assign19170_e14080: f64 = (assign19170_e14079).sqrt();
        (assign19170_e14080, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign19170_e14080)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign19170_e14080)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign19170_e14080)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign19170_e14080)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign19170_e14080)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign19170_e14080)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign19170_e14080)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign19170_e14080)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign19170_e14080)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign19170_e14080)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign19170_e14080)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19170_e14082;
        locals.var_tmf2_dn0 = assign19170_e14082_d_n0;
        locals.var_tmf2_dn2 = assign19170_e14082_d_n2;
        locals.var_tmf2_dn4 = assign19170_e14082_d_n4;
        locals.var_tmf2_dn5 = assign19170_e14082_d_n5;
        locals.var_tmf2_dn6 = assign19170_e14082_d_n6;
        locals.var_tmf2_dn7 = assign19170_e14082_d_n7;
        locals.var_tmf2_dn8 = assign19170_e14082_d_n8;
        locals.var_tmf2_dn9 = assign19170_e14082_d_n9;
        locals.var_tmf2_dn10 = assign19170_e14082_d_n10;
        locals.var_tmf2_dn11 = assign19170_e14082_d_n11;
        locals.var_tmf2_dn14 = assign19170_e14082_d_n14;

        let (assign19180_e14096, assign19180_e14096_d_n0, assign19180_e14096_d_n2, assign19180_e14096_d_n4, assign19180_e14096_d_n5, assign19180_e14096_d_n6, assign19180_e14096_d_n7, assign19180_e14096_d_n8, assign19180_e14096_d_n9, assign19180_e14096_d_n10, assign19180_e14096_d_n11, assign19180_e14096_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign19180_e14092: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign19180_e14093: f64 = (1.0 + assign19180_e14092);
        let assign19180_e14094: f64 = (0.5 * assign19180_e14093);
        (assign19180_e14094, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign19180_e14096;
        locals.var_t6_dn0 = assign19180_e14096_d_n0;
        locals.var_t6_dn2 = assign19180_e14096_d_n2;
        locals.var_t6_dn4 = assign19180_e14096_d_n4;
        locals.var_t6_dn5 = assign19180_e14096_d_n5;
        locals.var_t6_dn6 = assign19180_e14096_d_n6;
        locals.var_t6_dn7 = assign19180_e14096_d_n7;
        locals.var_t6_dn8 = assign19180_e14096_d_n8;
        locals.var_t6_dn9 = assign19180_e14096_d_n9;
        locals.var_t6_dn10 = assign19180_e14096_d_n10;
        locals.var_t6_dn11 = assign19180_e14096_d_n11;
        locals.var_t6_dn14 = assign19180_e14096_d_n14;

        let (assign19190_e14110, assign19190_e14110_d_n0, assign19190_e14110_d_n2, assign19190_e14110_d_n4, assign19190_e14110_d_n5, assign19190_e14110_d_n6, assign19190_e14110_d_n7, assign19190_e14110_d_n8, assign19190_e14110_d_n9, assign19190_e14110_d_n10, assign19190_e14110_d_n11, assign19190_e14110_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign19190_e14106: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign19190_e14107: f64 = (0.5 * assign19190_e14106);
        let assign19190_e14108: f64 = (locals.var_t4 + assign19190_e14107);
        (assign19190_e14108, (locals.var_t4_dn0 + (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t4_dn2 + (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t4_dn4 + (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t4_dn5 + (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t4_dn6 + (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t4_dn7 + (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t4_dn8 + (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t4_dn9 + (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t4_dn10 + (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t4_dn11 + (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_t4_dn14 + (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign19190_e14110;
        locals.var_t5_dn0 = assign19190_e14110_d_n0;
        locals.var_t5_dn2 = assign19190_e14110_d_n2;
        locals.var_t5_dn4 = assign19190_e14110_d_n4;
        locals.var_t5_dn5 = assign19190_e14110_d_n5;
        locals.var_t5_dn6 = assign19190_e14110_d_n6;
        locals.var_t5_dn7 = assign19190_e14110_d_n7;
        locals.var_t5_dn8 = assign19190_e14110_d_n8;
        locals.var_t5_dn9 = assign19190_e14110_d_n9;
        locals.var_t5_dn10 = assign19190_e14110_d_n10;
        locals.var_t5_dn11 = assign19190_e14110_d_n11;
        locals.var_t5_dn14 = assign19190_e14110_d_n14;

        let (assign19200_e14126, assign19200_e14126_d_n0, assign19200_e14126_d_n2, assign19200_e14126_d_n4, assign19200_e14126_d_n5, assign19200_e14126_d_n6, assign19200_e14126_d_n7, assign19200_e14126_d_n8, assign19200_e14126_d_n9, assign19200_e14126_d_n10, assign19200_e14126_d_n11, assign19200_e14126_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign19200_e14119: f64 = (p.p98 + 1.0);
        let assign19200_e14120: f64 = (locals.var_t4 * assign19200_e14119);
        let assign19200_e14122: f64 = (assign19200_e14120 - locals.var_t5);
        let assign19200_e14124: f64 = (assign19200_e14122 - 5e-5);
        (assign19200_e14124, ((locals.var_t4_dn0 * assign19200_e14119) - locals.var_t5_dn0), ((locals.var_t4_dn2 * assign19200_e14119) - locals.var_t5_dn2), ((locals.var_t4_dn4 * assign19200_e14119) - locals.var_t5_dn4), ((locals.var_t4_dn5 * assign19200_e14119) - locals.var_t5_dn5), ((locals.var_t4_dn6 * assign19200_e14119) - locals.var_t5_dn6), ((locals.var_t4_dn7 * assign19200_e14119) - locals.var_t5_dn7), ((locals.var_t4_dn8 * assign19200_e14119) - locals.var_t5_dn8), ((locals.var_t4_dn9 * assign19200_e14119) - locals.var_t5_dn9), ((locals.var_t4_dn10 * assign19200_e14119) - locals.var_t5_dn10), ((locals.var_t4_dn11 * assign19200_e14119) - locals.var_t5_dn11), ((locals.var_t4_dn14 * assign19200_e14119) - locals.var_t5_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign19200_e14126;
        locals.var_tmf1_dn0 = assign19200_e14126_d_n0;
        locals.var_tmf1_dn2 = assign19200_e14126_d_n2;
        locals.var_tmf1_dn4 = assign19200_e14126_d_n4;
        locals.var_tmf1_dn5 = assign19200_e14126_d_n5;
        locals.var_tmf1_dn6 = assign19200_e14126_d_n6;
        locals.var_tmf1_dn7 = assign19200_e14126_d_n7;
        locals.var_tmf1_dn8 = assign19200_e14126_d_n8;
        locals.var_tmf1_dn9 = assign19200_e14126_d_n9;
        locals.var_tmf1_dn10 = assign19200_e14126_d_n10;
        locals.var_tmf1_dn11 = assign19200_e14126_d_n11;
        locals.var_tmf1_dn14 = assign19200_e14126_d_n14;

        let (assign19210_e14142, assign19210_e14142_d_n0, assign19210_e14142_d_n2, assign19210_e14142_d_n4, assign19210_e14142_d_n5, assign19210_e14142_d_n6, assign19210_e14142_d_n7, assign19210_e14142_d_n8, assign19210_e14142_d_n9, assign19210_e14142_d_n10, assign19210_e14142_d_n11, assign19210_e14142_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign19210_e14136: f64 = (p.p98 + 1.0);
        let assign19210_e14137: f64 = (locals.var_t4 * assign19210_e14136);
        let assign19210_e14138: f64 = (4.0 * assign19210_e14137);
        let assign19210_e14140: f64 = (assign19210_e14138 * 5e-5);
        (assign19210_e14140, ((4.0 * (locals.var_t4_dn0 * assign19210_e14136)) * 5e-5), ((4.0 * (locals.var_t4_dn2 * assign19210_e14136)) * 5e-5), ((4.0 * (locals.var_t4_dn4 * assign19210_e14136)) * 5e-5), ((4.0 * (locals.var_t4_dn5 * assign19210_e14136)) * 5e-5), ((4.0 * (locals.var_t4_dn6 * assign19210_e14136)) * 5e-5), ((4.0 * (locals.var_t4_dn7 * assign19210_e14136)) * 5e-5), ((4.0 * (locals.var_t4_dn8 * assign19210_e14136)) * 5e-5), ((4.0 * (locals.var_t4_dn9 * assign19210_e14136)) * 5e-5), ((4.0 * (locals.var_t4_dn10 * assign19210_e14136)) * 5e-5), ((4.0 * (locals.var_t4_dn11 * assign19210_e14136)) * 5e-5), ((4.0 * (locals.var_t4_dn14 * assign19210_e14136)) * 5e-5),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19210_e14142;
        locals.var_tmf2_dn0 = assign19210_e14142_d_n0;
        locals.var_tmf2_dn2 = assign19210_e14142_d_n2;
        locals.var_tmf2_dn4 = assign19210_e14142_d_n4;
        locals.var_tmf2_dn5 = assign19210_e14142_d_n5;
        locals.var_tmf2_dn6 = assign19210_e14142_d_n6;
        locals.var_tmf2_dn7 = assign19210_e14142_d_n7;
        locals.var_tmf2_dn8 = assign19210_e14142_d_n8;
        locals.var_tmf2_dn9 = assign19210_e14142_d_n9;
        locals.var_tmf2_dn10 = assign19210_e14142_d_n10;
        locals.var_tmf2_dn11 = assign19210_e14142_d_n11;
        locals.var_tmf2_dn14 = assign19210_e14142_d_n14;

    }

    pub(super) fn stamp_transient_block_45(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign19220_e14156, assign19220_e14156_d_n0, assign19220_e14156_d_n2, assign19220_e14156_d_n4, assign19220_e14156_d_n5, assign19220_e14156_d_n6, assign19220_e14156_d_n7, assign19220_e14156_d_n8, assign19220_e14156_d_n9, assign19220_e14156_d_n10, assign19220_e14156_d_n11, assign19220_e14156_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let (assign19220_e14154, assign19220_e14154_d_n0, assign19220_e14154_d_n2, assign19220_e14154_d_n4, assign19220_e14154_d_n5, assign19220_e14154_d_n6, assign19220_e14154_d_n7, assign19220_e14154_d_n8, assign19220_e14154_d_n9, assign19220_e14154_d_n10, assign19220_e14154_d_n11, assign19220_e14154_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign19220_e14153: f64 = (-locals.var_tmf2);
                (assign19220_e14153, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign19220_e14154, assign19220_e14154_d_n0, assign19220_e14154_d_n2, assign19220_e14154_d_n4, assign19220_e14154_d_n5, assign19220_e14154_d_n6, assign19220_e14154_d_n7, assign19220_e14154_d_n8, assign19220_e14154_d_n9, assign19220_e14154_d_n10, assign19220_e14154_d_n11, assign19220_e14154_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19220_e14156;
        locals.var_tmf2_dn0 = assign19220_e14156_d_n0;
        locals.var_tmf2_dn2 = assign19220_e14156_d_n2;
        locals.var_tmf2_dn4 = assign19220_e14156_d_n4;
        locals.var_tmf2_dn5 = assign19220_e14156_d_n5;
        locals.var_tmf2_dn6 = assign19220_e14156_d_n6;
        locals.var_tmf2_dn7 = assign19220_e14156_d_n7;
        locals.var_tmf2_dn8 = assign19220_e14156_d_n8;
        locals.var_tmf2_dn9 = assign19220_e14156_d_n9;
        locals.var_tmf2_dn10 = assign19220_e14156_d_n10;
        locals.var_tmf2_dn11 = assign19220_e14156_d_n11;
        locals.var_tmf2_dn14 = assign19220_e14156_d_n14;

        let (assign19230_e14169, assign19230_e14169_d_n0, assign19230_e14169_d_n2, assign19230_e14169_d_n4, assign19230_e14169_d_n5, assign19230_e14169_d_n6, assign19230_e14169_d_n7, assign19230_e14169_d_n8, assign19230_e14169_d_n9, assign19230_e14169_d_n10, assign19230_e14169_d_n11, assign19230_e14169_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign19230_e14164: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign19230_e14166: f64 = (assign19230_e14164 + locals.var_tmf2);
        let assign19230_e14167: f64 = (assign19230_e14166).sqrt();
        (assign19230_e14167, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign19230_e14167)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign19230_e14167)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign19230_e14167)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign19230_e14167)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign19230_e14167)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign19230_e14167)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign19230_e14167)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign19230_e14167)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign19230_e14167)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign19230_e14167)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign19230_e14167)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19230_e14169;
        locals.var_tmf2_dn0 = assign19230_e14169_d_n0;
        locals.var_tmf2_dn2 = assign19230_e14169_d_n2;
        locals.var_tmf2_dn4 = assign19230_e14169_d_n4;
        locals.var_tmf2_dn5 = assign19230_e14169_d_n5;
        locals.var_tmf2_dn6 = assign19230_e14169_d_n6;
        locals.var_tmf2_dn7 = assign19230_e14169_d_n7;
        locals.var_tmf2_dn8 = assign19230_e14169_d_n8;
        locals.var_tmf2_dn9 = assign19230_e14169_d_n9;
        locals.var_tmf2_dn10 = assign19230_e14169_d_n10;
        locals.var_tmf2_dn11 = assign19230_e14169_d_n11;
        locals.var_tmf2_dn14 = assign19230_e14169_d_n14;

        let (assign19240_e14183, assign19240_e14183_d_n0, assign19240_e14183_d_n2, assign19240_e14183_d_n4, assign19240_e14183_d_n5, assign19240_e14183_d_n6, assign19240_e14183_d_n7, assign19240_e14183_d_n8, assign19240_e14183_d_n9, assign19240_e14183_d_n10, assign19240_e14183_d_n11, assign19240_e14183_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign19240_e14179: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign19240_e14180: f64 = (1.0 + assign19240_e14179);
        let assign19240_e14181: f64 = (0.5 * assign19240_e14180);
        (assign19240_e14181, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign19240_e14183;
        locals.var_t6_dn0 = assign19240_e14183_d_n0;
        locals.var_t6_dn2 = assign19240_e14183_d_n2;
        locals.var_t6_dn4 = assign19240_e14183_d_n4;
        locals.var_t6_dn5 = assign19240_e14183_d_n5;
        locals.var_t6_dn6 = assign19240_e14183_d_n6;
        locals.var_t6_dn7 = assign19240_e14183_d_n7;
        locals.var_t6_dn8 = assign19240_e14183_d_n8;
        locals.var_t6_dn9 = assign19240_e14183_d_n9;
        locals.var_t6_dn10 = assign19240_e14183_d_n10;
        locals.var_t6_dn11 = assign19240_e14183_d_n11;
        locals.var_t6_dn14 = assign19240_e14183_d_n14;

        let (assign19250_e14201, assign19250_e14201_d_n0, assign19250_e14201_d_n2, assign19250_e14201_d_n4, assign19250_e14201_d_n5, assign19250_e14201_d_n6, assign19250_e14201_d_n7, assign19250_e14201_d_n8, assign19250_e14201_d_n9, assign19250_e14201_d_n10, assign19250_e14201_d_n11, assign19250_e14201_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign19250_e14192: f64 = (p.p98 + 1.0);
        let assign19250_e14193: f64 = (locals.var_t4 * assign19250_e14192);
        let assign19250_e14197: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign19250_e14198: f64 = (0.5 * assign19250_e14197);
        let assign19250_e14199: f64 = (assign19250_e14193 - assign19250_e14198);
        (assign19250_e14199, ((locals.var_t4_dn0 * assign19250_e14192) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((locals.var_t4_dn2 * assign19250_e14192) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((locals.var_t4_dn4 * assign19250_e14192) - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), ((locals.var_t4_dn5 * assign19250_e14192) - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), ((locals.var_t4_dn6 * assign19250_e14192) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((locals.var_t4_dn7 * assign19250_e14192) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((locals.var_t4_dn8 * assign19250_e14192) - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), ((locals.var_t4_dn9 * assign19250_e14192) - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), ((locals.var_t4_dn10 * assign19250_e14192) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((locals.var_t4_dn11 * assign19250_e14192) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((locals.var_t4_dn14 * assign19250_e14192) - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign19250_e14201;
        locals.var_t7_dn0 = assign19250_e14201_d_n0;
        locals.var_t7_dn2 = assign19250_e14201_d_n2;
        locals.var_t7_dn4 = assign19250_e14201_d_n4;
        locals.var_t7_dn5 = assign19250_e14201_d_n5;
        locals.var_t7_dn6 = assign19250_e14201_d_n6;
        locals.var_t7_dn7 = assign19250_e14201_d_n7;
        locals.var_t7_dn8 = assign19250_e14201_d_n8;
        locals.var_t7_dn9 = assign19250_e14201_d_n9;
        locals.var_t7_dn10 = assign19250_e14201_d_n10;
        locals.var_t7_dn11 = assign19250_e14201_d_n11;
        locals.var_t7_dn14 = assign19250_e14201_d_n14;

        let (assign19260_e14217, assign19260_e14217_d_n0, assign19260_e14217_d_n2, assign19260_e14217_d_n4, assign19260_e14217_d_n5, assign19260_e14217_d_n6, assign19260_e14217_d_n7, assign19260_e14217_d_n8, assign19260_e14217_d_n9, assign19260_e14217_d_n10, assign19260_e14217_d_n11, assign19260_e14217_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign19260_e14210: f64 = (locals.var_t1 * locals.var_t4);
        let assign19260_e14211: f64 = (locals.var_t7 + assign19260_e14210);
        let assign19260_e14213: f64 = assign19260_e14211;
        let assign19260_e14215: f64 = (assign19260_e14213 - 5e-5);
        (assign19260_e14215, (locals.var_t7_dn0 + ((locals.var_t1_dn0 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn0))), (locals.var_t7_dn2 + ((locals.var_t1_dn2 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn2))), (locals.var_t7_dn4 + ((locals.var_t1_dn4 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn4))), (locals.var_t7_dn5 + ((locals.var_t1_dn5 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn5))), (locals.var_t7_dn6 + ((locals.var_t1_dn6 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn6))), (locals.var_t7_dn7 + ((locals.var_t1_dn7 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn7))), (locals.var_t7_dn8 + ((locals.var_t1_dn8 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn8))), (locals.var_t7_dn9 + ((locals.var_t1_dn9 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn9))), (locals.var_t7_dn10 + ((locals.var_t1_dn10 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn10))), (locals.var_t7_dn11 + ((locals.var_t1_dn11 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn11))), (locals.var_t7_dn14 + ((locals.var_t1_dn14 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn14))),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign19260_e14217;
        locals.var_tmf1_dn0 = assign19260_e14217_d_n0;
        locals.var_tmf1_dn2 = assign19260_e14217_d_n2;
        locals.var_tmf1_dn4 = assign19260_e14217_d_n4;
        locals.var_tmf1_dn5 = assign19260_e14217_d_n5;
        locals.var_tmf1_dn6 = assign19260_e14217_d_n6;
        locals.var_tmf1_dn7 = assign19260_e14217_d_n7;
        locals.var_tmf1_dn8 = assign19260_e14217_d_n8;
        locals.var_tmf1_dn9 = assign19260_e14217_d_n9;
        locals.var_tmf1_dn10 = assign19260_e14217_d_n10;
        locals.var_tmf1_dn11 = assign19260_e14217_d_n11;
        locals.var_tmf1_dn14 = assign19260_e14217_d_n14;

        let (assign19270_e14229, assign19270_e14229_d_n0, assign19270_e14229_d_n2, assign19270_e14229_d_n4, assign19270_e14229_d_n5, assign19270_e14229_d_n6, assign19270_e14229_d_n7, assign19270_e14229_d_n8, assign19270_e14229_d_n9, assign19270_e14229_d_n10, assign19270_e14229_d_n11, assign19270_e14229_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19270_e14229;
        locals.var_tmf2_dn0 = assign19270_e14229_d_n0;
        locals.var_tmf2_dn2 = assign19270_e14229_d_n2;
        locals.var_tmf2_dn4 = assign19270_e14229_d_n4;
        locals.var_tmf2_dn5 = assign19270_e14229_d_n5;
        locals.var_tmf2_dn6 = assign19270_e14229_d_n6;
        locals.var_tmf2_dn7 = assign19270_e14229_d_n7;
        locals.var_tmf2_dn8 = assign19270_e14229_d_n8;
        locals.var_tmf2_dn9 = assign19270_e14229_d_n9;
        locals.var_tmf2_dn10 = assign19270_e14229_d_n10;
        locals.var_tmf2_dn11 = assign19270_e14229_d_n11;
        locals.var_tmf2_dn14 = assign19270_e14229_d_n14;

        let (assign19280_e14243, assign19280_e14243_d_n0, assign19280_e14243_d_n2, assign19280_e14243_d_n4, assign19280_e14243_d_n5, assign19280_e14243_d_n6, assign19280_e14243_d_n7, assign19280_e14243_d_n8, assign19280_e14243_d_n9, assign19280_e14243_d_n10, assign19280_e14243_d_n11, assign19280_e14243_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let (assign19280_e14241, assign19280_e14241_d_n0, assign19280_e14241_d_n2, assign19280_e14241_d_n4, assign19280_e14241_d_n5, assign19280_e14241_d_n6, assign19280_e14241_d_n7, assign19280_e14241_d_n8, assign19280_e14241_d_n9, assign19280_e14241_d_n10, assign19280_e14241_d_n11, assign19280_e14241_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign19280_e14240: f64 = (-locals.var_tmf2);
                (assign19280_e14240, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign19280_e14241, assign19280_e14241_d_n0, assign19280_e14241_d_n2, assign19280_e14241_d_n4, assign19280_e14241_d_n5, assign19280_e14241_d_n6, assign19280_e14241_d_n7, assign19280_e14241_d_n8, assign19280_e14241_d_n9, assign19280_e14241_d_n10, assign19280_e14241_d_n11, assign19280_e14241_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19280_e14243;
        locals.var_tmf2_dn0 = assign19280_e14243_d_n0;
        locals.var_tmf2_dn2 = assign19280_e14243_d_n2;
        locals.var_tmf2_dn4 = assign19280_e14243_d_n4;
        locals.var_tmf2_dn5 = assign19280_e14243_d_n5;
        locals.var_tmf2_dn6 = assign19280_e14243_d_n6;
        locals.var_tmf2_dn7 = assign19280_e14243_d_n7;
        locals.var_tmf2_dn8 = assign19280_e14243_d_n8;
        locals.var_tmf2_dn9 = assign19280_e14243_d_n9;
        locals.var_tmf2_dn10 = assign19280_e14243_d_n10;
        locals.var_tmf2_dn11 = assign19280_e14243_d_n11;
        locals.var_tmf2_dn14 = assign19280_e14243_d_n14;

        let (assign19290_e14256, assign19290_e14256_d_n0, assign19290_e14256_d_n2, assign19290_e14256_d_n4, assign19290_e14256_d_n5, assign19290_e14256_d_n6, assign19290_e14256_d_n7, assign19290_e14256_d_n8, assign19290_e14256_d_n9, assign19290_e14256_d_n10, assign19290_e14256_d_n11, assign19290_e14256_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign19290_e14251: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign19290_e14253: f64 = (assign19290_e14251 + locals.var_tmf2);
        let assign19290_e14254: f64 = (assign19290_e14253).sqrt();
        (assign19290_e14254, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign19290_e14254)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign19290_e14254)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign19290_e14254)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign19290_e14254)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign19290_e14254)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign19290_e14254)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign19290_e14254)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign19290_e14254)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign19290_e14254)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign19290_e14254)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign19290_e14254)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19290_e14256;
        locals.var_tmf2_dn0 = assign19290_e14256_d_n0;
        locals.var_tmf2_dn2 = assign19290_e14256_d_n2;
        locals.var_tmf2_dn4 = assign19290_e14256_d_n4;
        locals.var_tmf2_dn5 = assign19290_e14256_d_n5;
        locals.var_tmf2_dn6 = assign19290_e14256_d_n6;
        locals.var_tmf2_dn7 = assign19290_e14256_d_n7;
        locals.var_tmf2_dn8 = assign19290_e14256_d_n8;
        locals.var_tmf2_dn9 = assign19290_e14256_d_n9;
        locals.var_tmf2_dn10 = assign19290_e14256_d_n10;
        locals.var_tmf2_dn11 = assign19290_e14256_d_n11;
        locals.var_tmf2_dn14 = assign19290_e14256_d_n14;

        let (assign19300_e14270, assign19300_e14270_d_n0, assign19300_e14270_d_n2, assign19300_e14270_d_n4, assign19300_e14270_d_n5, assign19300_e14270_d_n6, assign19300_e14270_d_n7, assign19300_e14270_d_n8, assign19300_e14270_d_n9, assign19300_e14270_d_n10, assign19300_e14270_d_n11, assign19300_e14270_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign19300_e14266: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign19300_e14267: f64 = (1.0 + assign19300_e14266);
        let assign19300_e14268: f64 = (0.5 * assign19300_e14267);
        (assign19300_e14268, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign19300_e14270;
        locals.var_t6_dn0 = assign19300_e14270_d_n0;
        locals.var_t6_dn2 = assign19300_e14270_d_n2;
        locals.var_t6_dn4 = assign19300_e14270_d_n4;
        locals.var_t6_dn5 = assign19300_e14270_d_n5;
        locals.var_t6_dn6 = assign19300_e14270_d_n6;
        locals.var_t6_dn7 = assign19300_e14270_d_n7;
        locals.var_t6_dn8 = assign19300_e14270_d_n8;
        locals.var_t6_dn9 = assign19300_e14270_d_n9;
        locals.var_t6_dn10 = assign19300_e14270_d_n10;
        locals.var_t6_dn11 = assign19300_e14270_d_n11;
        locals.var_t6_dn14 = assign19300_e14270_d_n14;

        let (assign19310_e14284, assign19310_e14284_d_n0, assign19310_e14284_d_n2, assign19310_e14284_d_n4, assign19310_e14284_d_n5, assign19310_e14284_d_n6, assign19310_e14284_d_n7, assign19310_e14284_d_n8, assign19310_e14284_d_n9, assign19310_e14284_d_n10, assign19310_e14284_d_n11, assign19310_e14284_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign19310_e14280: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign19310_e14281: f64 = (0.5 * assign19310_e14280);
        let assign19310_e14282: f64 = assign19310_e14281;
        (assign19310_e14282, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign19310_e14284;
        locals.var_t2_dn0 = assign19310_e14284_d_n0;
        locals.var_t2_dn2 = assign19310_e14284_d_n2;
        locals.var_t2_dn4 = assign19310_e14284_d_n4;
        locals.var_t2_dn5 = assign19310_e14284_d_n5;
        locals.var_t2_dn6 = assign19310_e14284_d_n6;
        locals.var_t2_dn7 = assign19310_e14284_d_n7;
        locals.var_t2_dn8 = assign19310_e14284_d_n8;
        locals.var_t2_dn9 = assign19310_e14284_d_n9;
        locals.var_t2_dn10 = assign19310_e14284_d_n10;
        locals.var_t2_dn11 = assign19310_e14284_d_n11;
        locals.var_t2_dn14 = assign19310_e14284_d_n14;

        let assign19320_e14291: f64 = if ((p.p39 == 0.0) || (p.p39 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard390 = assign19320_e14291;

        let (assign19330_e14311, assign19330_e14311_d_n0, assign19330_e14311_d_n2, assign19330_e14311_d_n4, assign19330_e14311_d_n5, assign19330_e14311_d_n6, assign19330_e14311_d_n7, assign19330_e14311_d_n8, assign19330_e14311_d_n9, assign19330_e14311_d_n10, assign19330_e14311_d_n11, assign19330_e14311_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard390 != 0.0)) {
        let assign19330_e14302: f64 = (locals.var_mks_rdvdtemp1 * locals.var_tdiff0);
        let assign19330_e14303: f64 = (locals.var_uc_rdvd + assign19330_e14302);
        let assign19330_e14306: f64 = (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2);
        let assign19330_e14307: f64 = (assign19330_e14303 + assign19330_e14306);
        let assign19330_e14309: f64 = (assign19330_e14307 * locals.var_t2);
        (assign19330_e14309, ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn0) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn0)) * locals.var_t2) + (assign19330_e14307 * locals.var_t2_dn0)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn2) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn2)) * locals.var_t2) + (assign19330_e14307 * locals.var_t2_dn2)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn4) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn4)) * locals.var_t2) + (assign19330_e14307 * locals.var_t2_dn4)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn5) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn5)) * locals.var_t2) + (assign19330_e14307 * locals.var_t2_dn5)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn6) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn6)) * locals.var_t2) + (assign19330_e14307 * locals.var_t2_dn6)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn7) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn7)) * locals.var_t2) + (assign19330_e14307 * locals.var_t2_dn7)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn8) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn8)) * locals.var_t2) + (assign19330_e14307 * locals.var_t2_dn8)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn9) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn9)) * locals.var_t2) + (assign19330_e14307 * locals.var_t2_dn9)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn10) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn10)) * locals.var_t2) + (assign19330_e14307 * locals.var_t2_dn10)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn11) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn11)) * locals.var_t2) + (assign19330_e14307 * locals.var_t2_dn11)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn14) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn14)) * locals.var_t2) + (assign19330_e14307 * locals.var_t2_dn14)),)
    } else {
        (locals.var_rsvde, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn11, locals.var_rsvde_dn14,)
    }
};
        locals.var_rsvde = assign19330_e14311;
        locals.var_rsvde_dn0 = assign19330_e14311_d_n0;
        locals.var_rsvde_dn2 = assign19330_e14311_d_n2;
        locals.var_rsvde_dn4 = assign19330_e14311_d_n4;
        locals.var_rsvde_dn5 = assign19330_e14311_d_n5;
        locals.var_rsvde_dn6 = assign19330_e14311_d_n6;
        locals.var_rsvde_dn7 = assign19330_e14311_d_n7;
        locals.var_rsvde_dn8 = assign19330_e14311_d_n8;
        locals.var_rsvde_dn9 = assign19330_e14311_d_n9;
        locals.var_rsvde_dn10 = assign19330_e14311_d_n10;
        locals.var_rsvde_dn11 = assign19330_e14311_d_n11;
        locals.var_rsvde_dn14 = assign19330_e14311_d_n14;

        let (assign19340_e14329, assign19340_e14329_d_n0, assign19340_e14329_d_n2, assign19340_e14329_d_n4, assign19340_e14329_d_n5, assign19340_e14329_d_n6, assign19340_e14329_d_n7, assign19340_e14329_d_n8, assign19340_e14329_d_n9, assign19340_e14329_d_n10, assign19340_e14329_d_n11, assign19340_e14329_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard390 != 0.0)) {
        let assign19340_e14322: f64 = (0.005 * locals.var_uc_rdvd);
        let assign19340_e14323: f64 = (locals.var_rsvde - assign19340_e14322);
        let assign19340_e14326: f64 = (0.01 * locals.var_uc_rdvd);
        let assign19340_e14327: f64 = (assign19340_e14323 - assign19340_e14326);
        (assign19340_e14327, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn11, locals.var_rsvde_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign19340_e14329;
        locals.var_tmf1_dn0 = assign19340_e14329_d_n0;
        locals.var_tmf1_dn2 = assign19340_e14329_d_n2;
        locals.var_tmf1_dn4 = assign19340_e14329_d_n4;
        locals.var_tmf1_dn5 = assign19340_e14329_d_n5;
        locals.var_tmf1_dn6 = assign19340_e14329_d_n6;
        locals.var_tmf1_dn7 = assign19340_e14329_d_n7;
        locals.var_tmf1_dn8 = assign19340_e14329_d_n8;
        locals.var_tmf1_dn9 = assign19340_e14329_d_n9;
        locals.var_tmf1_dn10 = assign19340_e14329_d_n10;
        locals.var_tmf1_dn11 = assign19340_e14329_d_n11;
        locals.var_tmf1_dn14 = assign19340_e14329_d_n14;

        let (assign19350_e14347, assign19350_e14347_d_n0, assign19350_e14347_d_n2, assign19350_e14347_d_n4, assign19350_e14347_d_n5, assign19350_e14347_d_n6, assign19350_e14347_d_n7, assign19350_e14347_d_n8, assign19350_e14347_d_n9, assign19350_e14347_d_n10, assign19350_e14347_d_n11, assign19350_e14347_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard390 != 0.0)) {
        let assign19350_e14340: f64 = (0.005 * locals.var_uc_rdvd);
        let assign19350_e14341: f64 = (4.0 * assign19350_e14340);
        let assign19350_e14344: f64 = (0.01 * locals.var_uc_rdvd);
        let assign19350_e14345: f64 = (assign19350_e14341 * assign19350_e14344);
        (assign19350_e14345, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19350_e14347;
        locals.var_tmf2_dn0 = assign19350_e14347_d_n0;
        locals.var_tmf2_dn2 = assign19350_e14347_d_n2;
        locals.var_tmf2_dn4 = assign19350_e14347_d_n4;
        locals.var_tmf2_dn5 = assign19350_e14347_d_n5;
        locals.var_tmf2_dn6 = assign19350_e14347_d_n6;
        locals.var_tmf2_dn7 = assign19350_e14347_d_n7;
        locals.var_tmf2_dn8 = assign19350_e14347_d_n8;
        locals.var_tmf2_dn9 = assign19350_e14347_d_n9;
        locals.var_tmf2_dn10 = assign19350_e14347_d_n10;
        locals.var_tmf2_dn11 = assign19350_e14347_d_n11;
        locals.var_tmf2_dn14 = assign19350_e14347_d_n14;

        let (assign19360_e14363, assign19360_e14363_d_n0, assign19360_e14363_d_n2, assign19360_e14363_d_n4, assign19360_e14363_d_n5, assign19360_e14363_d_n6, assign19360_e14363_d_n7, assign19360_e14363_d_n8, assign19360_e14363_d_n9, assign19360_e14363_d_n10, assign19360_e14363_d_n11, assign19360_e14363_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard390 != 0.0)) {
        let (assign19360_e14361, assign19360_e14361_d_n0, assign19360_e14361_d_n2, assign19360_e14361_d_n4, assign19360_e14361_d_n5, assign19360_e14361_d_n6, assign19360_e14361_d_n7, assign19360_e14361_d_n8, assign19360_e14361_d_n9, assign19360_e14361_d_n10, assign19360_e14361_d_n11, assign19360_e14361_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign19360_e14360: f64 = (-locals.var_tmf2);
                (assign19360_e14360, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign19360_e14361, assign19360_e14361_d_n0, assign19360_e14361_d_n2, assign19360_e14361_d_n4, assign19360_e14361_d_n5, assign19360_e14361_d_n6, assign19360_e14361_d_n7, assign19360_e14361_d_n8, assign19360_e14361_d_n9, assign19360_e14361_d_n10, assign19360_e14361_d_n11, assign19360_e14361_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19360_e14363;
        locals.var_tmf2_dn0 = assign19360_e14363_d_n0;
        locals.var_tmf2_dn2 = assign19360_e14363_d_n2;
        locals.var_tmf2_dn4 = assign19360_e14363_d_n4;
        locals.var_tmf2_dn5 = assign19360_e14363_d_n5;
        locals.var_tmf2_dn6 = assign19360_e14363_d_n6;
        locals.var_tmf2_dn7 = assign19360_e14363_d_n7;
        locals.var_tmf2_dn8 = assign19360_e14363_d_n8;
        locals.var_tmf2_dn9 = assign19360_e14363_d_n9;
        locals.var_tmf2_dn10 = assign19360_e14363_d_n10;
        locals.var_tmf2_dn11 = assign19360_e14363_d_n11;
        locals.var_tmf2_dn14 = assign19360_e14363_d_n14;

        let (assign19370_e14378, assign19370_e14378_d_n0, assign19370_e14378_d_n2, assign19370_e14378_d_n4, assign19370_e14378_d_n5, assign19370_e14378_d_n6, assign19370_e14378_d_n7, assign19370_e14378_d_n8, assign19370_e14378_d_n9, assign19370_e14378_d_n10, assign19370_e14378_d_n11, assign19370_e14378_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard390 != 0.0)) {
        let assign19370_e14373: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign19370_e14375: f64 = (assign19370_e14373 + locals.var_tmf2);
        let assign19370_e14376: f64 = (assign19370_e14375).sqrt();
        (assign19370_e14376, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign19370_e14376)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign19370_e14376)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign19370_e14376)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign19370_e14376)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign19370_e14376)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign19370_e14376)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign19370_e14376)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign19370_e14376)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign19370_e14376)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign19370_e14376)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign19370_e14376)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19370_e14378;
        locals.var_tmf2_dn0 = assign19370_e14378_d_n0;
        locals.var_tmf2_dn2 = assign19370_e14378_d_n2;
        locals.var_tmf2_dn4 = assign19370_e14378_d_n4;
        locals.var_tmf2_dn5 = assign19370_e14378_d_n5;
        locals.var_tmf2_dn6 = assign19370_e14378_d_n6;
        locals.var_tmf2_dn7 = assign19370_e14378_d_n7;
        locals.var_tmf2_dn8 = assign19370_e14378_d_n8;
        locals.var_tmf2_dn9 = assign19370_e14378_d_n9;
        locals.var_tmf2_dn10 = assign19370_e14378_d_n10;
        locals.var_tmf2_dn11 = assign19370_e14378_d_n11;
        locals.var_tmf2_dn14 = assign19370_e14378_d_n14;

        let (assign19380_e14394, assign19380_e14394_d_n0, assign19380_e14394_d_n2, assign19380_e14394_d_n4, assign19380_e14394_d_n5, assign19380_e14394_d_n6, assign19380_e14394_d_n7, assign19380_e14394_d_n8, assign19380_e14394_d_n9, assign19380_e14394_d_n10, assign19380_e14394_d_n11, assign19380_e14394_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard390 != 0.0)) {
        let assign19380_e14390: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign19380_e14391: f64 = (1.0 + assign19380_e14390);
        let assign19380_e14392: f64 = (0.5 * assign19380_e14391);
        (assign19380_e14392, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign19380_e14394;
        locals.var_t0_dn0 = assign19380_e14394_d_n0;
        locals.var_t0_dn2 = assign19380_e14394_d_n2;
        locals.var_t0_dn4 = assign19380_e14394_d_n4;
        locals.var_t0_dn5 = assign19380_e14394_d_n5;
        locals.var_t0_dn6 = assign19380_e14394_d_n6;
        locals.var_t0_dn7 = assign19380_e14394_d_n7;
        locals.var_t0_dn8 = assign19380_e14394_d_n8;
        locals.var_t0_dn9 = assign19380_e14394_d_n9;
        locals.var_t0_dn10 = assign19380_e14394_d_n10;
        locals.var_t0_dn11 = assign19380_e14394_d_n11;
        locals.var_t0_dn14 = assign19380_e14394_d_n14;

        let (assign19390_e14412, assign19390_e14412_d_n0, assign19390_e14412_d_n2, assign19390_e14412_d_n4, assign19390_e14412_d_n5, assign19390_e14412_d_n6, assign19390_e14412_d_n7, assign19390_e14412_d_n8, assign19390_e14412_d_n9, assign19390_e14412_d_n10, assign19390_e14412_d_n11, assign19390_e14412_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard390 != 0.0)) {
        let assign19390_e14404: f64 = (0.005 * locals.var_uc_rdvd);
        let assign19390_e14408: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign19390_e14409: f64 = (0.5 * assign19390_e14408);
        let assign19390_e14410: f64 = (assign19390_e14404 + assign19390_e14409);
        (assign19390_e14410, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_rsvde, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn11, locals.var_rsvde_dn14,)
    }
};
        locals.var_rsvde = assign19390_e14412;
        locals.var_rsvde_dn0 = assign19390_e14412_d_n0;
        locals.var_rsvde_dn2 = assign19390_e14412_d_n2;
        locals.var_rsvde_dn4 = assign19390_e14412_d_n4;
        locals.var_rsvde_dn5 = assign19390_e14412_d_n5;
        locals.var_rsvde_dn6 = assign19390_e14412_d_n6;
        locals.var_rsvde_dn7 = assign19390_e14412_d_n7;
        locals.var_rsvde_dn8 = assign19390_e14412_d_n8;
        locals.var_rsvde_dn9 = assign19390_e14412_d_n9;
        locals.var_rsvde_dn10 = assign19390_e14412_d_n10;
        locals.var_rsvde_dn11 = assign19390_e14412_d_n11;
        locals.var_rsvde_dn14 = assign19390_e14412_d_n14;

        let (assign19400_e14433, assign19400_e14433_d_n0, assign19400_e14433_d_n2, assign19400_e14433_d_n4, assign19400_e14433_d_n5, assign19400_e14433_d_n6, assign19400_e14433_d_n7, assign19400_e14433_d_n8, assign19400_e14433_d_n9, assign19400_e14433_d_n10, assign19400_e14433_d_n11, assign19400_e14433_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard390 == 0.0)) {
        let assign19400_e14424: f64 = (locals.var_mks_rdvdtemp1 * locals.var_tdiff);
        let assign19400_e14425: f64 = (locals.var_uc_rdvd + assign19400_e14424);
        let assign19400_e14428: f64 = (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2);
        let assign19400_e14429: f64 = (assign19400_e14425 + assign19400_e14428);
        let assign19400_e14431: f64 = (assign19400_e14429 * locals.var_t2);
        (assign19400_e14431, ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn0) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn0)) * locals.var_t2) + (assign19400_e14429 * locals.var_t2_dn0)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn2) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn2)) * locals.var_t2) + (assign19400_e14429 * locals.var_t2_dn2)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn4) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn4)) * locals.var_t2) + (assign19400_e14429 * locals.var_t2_dn4)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn5) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn5)) * locals.var_t2) + (assign19400_e14429 * locals.var_t2_dn5)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn6) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn6)) * locals.var_t2) + (assign19400_e14429 * locals.var_t2_dn6)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn7) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn7)) * locals.var_t2) + (assign19400_e14429 * locals.var_t2_dn7)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn8) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn8)) * locals.var_t2) + (assign19400_e14429 * locals.var_t2_dn8)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn9) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn9)) * locals.var_t2) + (assign19400_e14429 * locals.var_t2_dn9)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn10) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn10)) * locals.var_t2) + (assign19400_e14429 * locals.var_t2_dn10)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn11) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn11)) * locals.var_t2) + (assign19400_e14429 * locals.var_t2_dn11)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn14) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn14)) * locals.var_t2) + (assign19400_e14429 * locals.var_t2_dn14)),)
    } else {
        (locals.var_rsvde, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn11, locals.var_rsvde_dn14,)
    }
};
        locals.var_rsvde = assign19400_e14433;
        locals.var_rsvde_dn0 = assign19400_e14433_d_n0;
        locals.var_rsvde_dn2 = assign19400_e14433_d_n2;
        locals.var_rsvde_dn4 = assign19400_e14433_d_n4;
        locals.var_rsvde_dn5 = assign19400_e14433_d_n5;
        locals.var_rsvde_dn6 = assign19400_e14433_d_n6;
        locals.var_rsvde_dn7 = assign19400_e14433_d_n7;
        locals.var_rsvde_dn8 = assign19400_e14433_d_n8;
        locals.var_rsvde_dn9 = assign19400_e14433_d_n9;
        locals.var_rsvde_dn10 = assign19400_e14433_d_n10;
        locals.var_rsvde_dn11 = assign19400_e14433_d_n11;
        locals.var_rsvde_dn14 = assign19400_e14433_d_n14;

        let (assign19410_e14452, assign19410_e14452_d_n0, assign19410_e14452_d_n2, assign19410_e14452_d_n4, assign19410_e14452_d_n5, assign19410_e14452_d_n6, assign19410_e14452_d_n7, assign19410_e14452_d_n8, assign19410_e14452_d_n9, assign19410_e14452_d_n10, assign19410_e14452_d_n11, assign19410_e14452_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard390 == 0.0)) {
        let assign19410_e14445: f64 = (0.005 * locals.var_uc_rdvd);
        let assign19410_e14446: f64 = (locals.var_rsvde - assign19410_e14445);
        let assign19410_e14449: f64 = (0.01 * locals.var_uc_rdvd);
        let assign19410_e14450: f64 = (assign19410_e14446 - assign19410_e14449);
        (assign19410_e14450, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn11, locals.var_rsvde_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign19410_e14452;
        locals.var_tmf1_dn0 = assign19410_e14452_d_n0;
        locals.var_tmf1_dn2 = assign19410_e14452_d_n2;
        locals.var_tmf1_dn4 = assign19410_e14452_d_n4;
        locals.var_tmf1_dn5 = assign19410_e14452_d_n5;
        locals.var_tmf1_dn6 = assign19410_e14452_d_n6;
        locals.var_tmf1_dn7 = assign19410_e14452_d_n7;
        locals.var_tmf1_dn8 = assign19410_e14452_d_n8;
        locals.var_tmf1_dn9 = assign19410_e14452_d_n9;
        locals.var_tmf1_dn10 = assign19410_e14452_d_n10;
        locals.var_tmf1_dn11 = assign19410_e14452_d_n11;
        locals.var_tmf1_dn14 = assign19410_e14452_d_n14;

        let (assign19420_e14471, assign19420_e14471_d_n0, assign19420_e14471_d_n2, assign19420_e14471_d_n4, assign19420_e14471_d_n5, assign19420_e14471_d_n6, assign19420_e14471_d_n7, assign19420_e14471_d_n8, assign19420_e14471_d_n9, assign19420_e14471_d_n10, assign19420_e14471_d_n11, assign19420_e14471_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard390 == 0.0)) {
        let assign19420_e14464: f64 = (0.005 * locals.var_uc_rdvd);
        let assign19420_e14465: f64 = (4.0 * assign19420_e14464);
        let assign19420_e14468: f64 = (0.01 * locals.var_uc_rdvd);
        let assign19420_e14469: f64 = (assign19420_e14465 * assign19420_e14468);
        (assign19420_e14469, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19420_e14471;
        locals.var_tmf2_dn0 = assign19420_e14471_d_n0;
        locals.var_tmf2_dn2 = assign19420_e14471_d_n2;
        locals.var_tmf2_dn4 = assign19420_e14471_d_n4;
        locals.var_tmf2_dn5 = assign19420_e14471_d_n5;
        locals.var_tmf2_dn6 = assign19420_e14471_d_n6;
        locals.var_tmf2_dn7 = assign19420_e14471_d_n7;
        locals.var_tmf2_dn8 = assign19420_e14471_d_n8;
        locals.var_tmf2_dn9 = assign19420_e14471_d_n9;
        locals.var_tmf2_dn10 = assign19420_e14471_d_n10;
        locals.var_tmf2_dn11 = assign19420_e14471_d_n11;
        locals.var_tmf2_dn14 = assign19420_e14471_d_n14;

    }

    pub(super) fn stamp_transient_block_46(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let (assign19430_e14488, assign19430_e14488_d_n0, assign19430_e14488_d_n2, assign19430_e14488_d_n4, assign19430_e14488_d_n5, assign19430_e14488_d_n6, assign19430_e14488_d_n7, assign19430_e14488_d_n8, assign19430_e14488_d_n9, assign19430_e14488_d_n10, assign19430_e14488_d_n11, assign19430_e14488_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard390 == 0.0)) {
        let (assign19430_e14486, assign19430_e14486_d_n0, assign19430_e14486_d_n2, assign19430_e14486_d_n4, assign19430_e14486_d_n5, assign19430_e14486_d_n6, assign19430_e14486_d_n7, assign19430_e14486_d_n8, assign19430_e14486_d_n9, assign19430_e14486_d_n10, assign19430_e14486_d_n11, assign19430_e14486_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign19430_e14485: f64 = (-locals.var_tmf2);
                (assign19430_e14485, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign19430_e14486, assign19430_e14486_d_n0, assign19430_e14486_d_n2, assign19430_e14486_d_n4, assign19430_e14486_d_n5, assign19430_e14486_d_n6, assign19430_e14486_d_n7, assign19430_e14486_d_n8, assign19430_e14486_d_n9, assign19430_e14486_d_n10, assign19430_e14486_d_n11, assign19430_e14486_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19430_e14488;
        locals.var_tmf2_dn0 = assign19430_e14488_d_n0;
        locals.var_tmf2_dn2 = assign19430_e14488_d_n2;
        locals.var_tmf2_dn4 = assign19430_e14488_d_n4;
        locals.var_tmf2_dn5 = assign19430_e14488_d_n5;
        locals.var_tmf2_dn6 = assign19430_e14488_d_n6;
        locals.var_tmf2_dn7 = assign19430_e14488_d_n7;
        locals.var_tmf2_dn8 = assign19430_e14488_d_n8;
        locals.var_tmf2_dn9 = assign19430_e14488_d_n9;
        locals.var_tmf2_dn10 = assign19430_e14488_d_n10;
        locals.var_tmf2_dn11 = assign19430_e14488_d_n11;
        locals.var_tmf2_dn14 = assign19430_e14488_d_n14;

        let (assign19440_e14504, assign19440_e14504_d_n0, assign19440_e14504_d_n2, assign19440_e14504_d_n4, assign19440_e14504_d_n5, assign19440_e14504_d_n6, assign19440_e14504_d_n7, assign19440_e14504_d_n8, assign19440_e14504_d_n9, assign19440_e14504_d_n10, assign19440_e14504_d_n11, assign19440_e14504_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard390 == 0.0)) {
        let assign19440_e14499: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign19440_e14501: f64 = (assign19440_e14499 + locals.var_tmf2);
        let assign19440_e14502: f64 = (assign19440_e14501).sqrt();
        (assign19440_e14502, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign19440_e14502)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign19440_e14502)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign19440_e14502)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign19440_e14502)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign19440_e14502)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign19440_e14502)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign19440_e14502)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign19440_e14502)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign19440_e14502)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign19440_e14502)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign19440_e14502)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19440_e14504;
        locals.var_tmf2_dn0 = assign19440_e14504_d_n0;
        locals.var_tmf2_dn2 = assign19440_e14504_d_n2;
        locals.var_tmf2_dn4 = assign19440_e14504_d_n4;
        locals.var_tmf2_dn5 = assign19440_e14504_d_n5;
        locals.var_tmf2_dn6 = assign19440_e14504_d_n6;
        locals.var_tmf2_dn7 = assign19440_e14504_d_n7;
        locals.var_tmf2_dn8 = assign19440_e14504_d_n8;
        locals.var_tmf2_dn9 = assign19440_e14504_d_n9;
        locals.var_tmf2_dn10 = assign19440_e14504_d_n10;
        locals.var_tmf2_dn11 = assign19440_e14504_d_n11;
        locals.var_tmf2_dn14 = assign19440_e14504_d_n14;

        let (assign19450_e14521, assign19450_e14521_d_n0, assign19450_e14521_d_n2, assign19450_e14521_d_n4, assign19450_e14521_d_n5, assign19450_e14521_d_n6, assign19450_e14521_d_n7, assign19450_e14521_d_n8, assign19450_e14521_d_n9, assign19450_e14521_d_n10, assign19450_e14521_d_n11, assign19450_e14521_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard390 == 0.0)) {
        let assign19450_e14517: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign19450_e14518: f64 = (1.0 + assign19450_e14517);
        let assign19450_e14519: f64 = (0.5 * assign19450_e14518);
        (assign19450_e14519, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign19450_e14521;
        locals.var_t0_dn0 = assign19450_e14521_d_n0;
        locals.var_t0_dn2 = assign19450_e14521_d_n2;
        locals.var_t0_dn4 = assign19450_e14521_d_n4;
        locals.var_t0_dn5 = assign19450_e14521_d_n5;
        locals.var_t0_dn6 = assign19450_e14521_d_n6;
        locals.var_t0_dn7 = assign19450_e14521_d_n7;
        locals.var_t0_dn8 = assign19450_e14521_d_n8;
        locals.var_t0_dn9 = assign19450_e14521_d_n9;
        locals.var_t0_dn10 = assign19450_e14521_d_n10;
        locals.var_t0_dn11 = assign19450_e14521_d_n11;
        locals.var_t0_dn14 = assign19450_e14521_d_n14;

        let (assign19460_e14540, assign19460_e14540_d_n0, assign19460_e14540_d_n2, assign19460_e14540_d_n4, assign19460_e14540_d_n5, assign19460_e14540_d_n6, assign19460_e14540_d_n7, assign19460_e14540_d_n8, assign19460_e14540_d_n9, assign19460_e14540_d_n10, assign19460_e14540_d_n11, assign19460_e14540_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard390 == 0.0)) {
        let assign19460_e14532: f64 = (0.005 * locals.var_uc_rdvd);
        let assign19460_e14536: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign19460_e14537: f64 = (0.5 * assign19460_e14536);
        let assign19460_e14538: f64 = (assign19460_e14532 + assign19460_e14537);
        (assign19460_e14538, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_rsvde, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn11, locals.var_rsvde_dn14,)
    }
};
        locals.var_rsvde = assign19460_e14540;
        locals.var_rsvde_dn0 = assign19460_e14540_d_n0;
        locals.var_rsvde_dn2 = assign19460_e14540_d_n2;
        locals.var_rsvde_dn4 = assign19460_e14540_d_n4;
        locals.var_rsvde_dn5 = assign19460_e14540_d_n5;
        locals.var_rsvde_dn6 = assign19460_e14540_d_n6;
        locals.var_rsvde_dn7 = assign19460_e14540_d_n7;
        locals.var_rsvde_dn8 = assign19460_e14540_d_n8;
        locals.var_rsvde_dn9 = assign19460_e14540_d_n9;
        locals.var_rsvde_dn10 = assign19460_e14540_d_n10;
        locals.var_rsvde_dn11 = assign19460_e14540_d_n11;
        locals.var_rsvde_dn14 = assign19460_e14540_d_n14;

        let (assign19470_e14549, assign19470_e14549_d_n0, assign19470_e14549_d_n2, assign19470_e14549_d_n4, assign19470_e14549_d_n5, assign19470_e14549_d_n6, assign19470_e14549_d_n7, assign19470_e14549_d_n8, assign19470_e14549_d_n9, assign19470_e14549_d_n10, assign19470_e14549_d_n11, assign19470_e14549_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdvde, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn11, locals.var_rdvde_dn14,)
    }
};
        locals.var_rdvde = assign19470_e14549;
        locals.var_rdvde_dn0 = assign19470_e14549_d_n0;
        locals.var_rdvde_dn2 = assign19470_e14549_d_n2;
        locals.var_rdvde_dn4 = assign19470_e14549_d_n4;
        locals.var_rdvde_dn5 = assign19470_e14549_d_n5;
        locals.var_rdvde_dn6 = assign19470_e14549_d_n6;
        locals.var_rdvde_dn7 = assign19470_e14549_d_n7;
        locals.var_rdvde_dn8 = assign19470_e14549_d_n8;
        locals.var_rdvde_dn9 = assign19470_e14549_d_n9;
        locals.var_rdvde_dn10 = assign19470_e14549_d_n10;
        locals.var_rdvde_dn11 = assign19470_e14549_d_n11;
        locals.var_rdvde_dn14 = assign19470_e14549_d_n14;

        let (assign19480_e14558, assign19480_e14558_d_n0, assign19480_e14558_d_n2, assign19480_e14558_d_n4, assign19480_e14558_d_n5, assign19480_e14558_d_n6, assign19480_e14558_d_n7, assign19480_e14558_d_n8, assign19480_e14558_d_n9, assign19480_e14558_d_n10, assign19480_e14558_d_n11, assign19480_e14558_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rsvde, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn11, locals.var_rsvde_dn14,)
    }
};
        locals.var_rsvde = assign19480_e14558;
        locals.var_rsvde_dn0 = assign19480_e14558_d_n0;
        locals.var_rsvde_dn2 = assign19480_e14558_d_n2;
        locals.var_rsvde_dn4 = assign19480_e14558_d_n4;
        locals.var_rsvde_dn5 = assign19480_e14558_d_n5;
        locals.var_rsvde_dn6 = assign19480_e14558_d_n6;
        locals.var_rsvde_dn7 = assign19480_e14558_d_n7;
        locals.var_rsvde_dn8 = assign19480_e14558_d_n8;
        locals.var_rsvde_dn9 = assign19480_e14558_d_n9;
        locals.var_rsvde_dn10 = assign19480_e14558_d_n10;
        locals.var_rsvde_dn11 = assign19480_e14558_d_n11;
        locals.var_rsvde_dn14 = assign19480_e14558_d_n14;

        let (assign19490_e14565, assign19490_e14565_d_n0, assign19490_e14565_d_n2, assign19490_e14565_d_n4, assign19490_e14565_d_n5, assign19490_e14565_d_n6, assign19490_e14565_d_n7, assign19490_e14565_d_n8, assign19490_e14565_d_n9, assign19490_e14565_d_n10, assign19490_e14565_d_n11, assign19490_e14565_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign19490_e14562: f64 = (locals.var_beta_inv).sqrt();
        let assign19490_e14563: f64 = (locals.var_costi00 * assign19490_e14562);
        (assign19490_e14563, (locals.var_costi00 * (locals.var_beta_inv_dn0 / (2.0 * assign19490_e14562))), (locals.var_costi00 * (locals.var_beta_inv_dn2 / (2.0 * assign19490_e14562))), (locals.var_costi00 * (locals.var_beta_inv_dn4 / (2.0 * assign19490_e14562))), (locals.var_costi00 * (locals.var_beta_inv_dn5 / (2.0 * assign19490_e14562))), (locals.var_costi00 * (locals.var_beta_inv_dn6 / (2.0 * assign19490_e14562))), (locals.var_costi00 * (locals.var_beta_inv_dn7 / (2.0 * assign19490_e14562))), (locals.var_costi00 * (locals.var_beta_inv_dn8 / (2.0 * assign19490_e14562))), (locals.var_costi00 * (locals.var_beta_inv_dn9 / (2.0 * assign19490_e14562))), (locals.var_costi00 * (locals.var_beta_inv_dn10 / (2.0 * assign19490_e14562))), (locals.var_costi00 * (locals.var_beta_inv_dn11 / (2.0 * assign19490_e14562))), (locals.var_costi00 * (locals.var_beta_inv_dn14 / (2.0 * assign19490_e14562))),)
    } else {
        (locals.var_costi0, locals.var_costi0_dn0, locals.var_costi0_dn2, locals.var_costi0_dn4, locals.var_costi0_dn5, locals.var_costi0_dn6, locals.var_costi0_dn7, locals.var_costi0_dn8, locals.var_costi0_dn9, locals.var_costi0_dn10, locals.var_costi0_dn11, locals.var_costi0_dn14,)
    }
};
        locals.var_costi0 = assign19490_e14565;
        locals.var_costi0_dn0 = assign19490_e14565_d_n0;
        locals.var_costi0_dn2 = assign19490_e14565_d_n2;
        locals.var_costi0_dn4 = assign19490_e14565_d_n4;
        locals.var_costi0_dn5 = assign19490_e14565_d_n5;
        locals.var_costi0_dn6 = assign19490_e14565_d_n6;
        locals.var_costi0_dn7 = assign19490_e14565_d_n7;
        locals.var_costi0_dn8 = assign19490_e14565_d_n8;
        locals.var_costi0_dn9 = assign19490_e14565_d_n9;
        locals.var_costi0_dn10 = assign19490_e14565_d_n10;
        locals.var_costi0_dn11 = assign19490_e14565_d_n11;
        locals.var_costi0_dn14 = assign19490_e14565_d_n14;

        let (assign19500_e14571, assign19500_e14571_d_n0, assign19500_e14571_d_n2, assign19500_e14571_d_n4, assign19500_e14571_d_n5, assign19500_e14571_d_n6, assign19500_e14571_d_n7, assign19500_e14571_d_n8, assign19500_e14571_d_n9, assign19500_e14571_d_n10, assign19500_e14571_d_n11, assign19500_e14571_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign19500_e14569: f64 = (locals.var_costi0 * locals.var_costi0);
        (assign19500_e14569, ((locals.var_costi0_dn0 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn0)), ((locals.var_costi0_dn2 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn2)), ((locals.var_costi0_dn4 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn4)), ((locals.var_costi0_dn5 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn5)), ((locals.var_costi0_dn6 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn6)), ((locals.var_costi0_dn7 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn7)), ((locals.var_costi0_dn8 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn8)), ((locals.var_costi0_dn9 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn9)), ((locals.var_costi0_dn10 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn10)), ((locals.var_costi0_dn11 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn11)), ((locals.var_costi0_dn14 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn14)),)
    } else {
        (locals.var_costi0_p2, locals.var_costi0_p2_dn0, locals.var_costi0_p2_dn2, locals.var_costi0_p2_dn4, locals.var_costi0_p2_dn5, locals.var_costi0_p2_dn6, locals.var_costi0_p2_dn7, locals.var_costi0_p2_dn8, locals.var_costi0_p2_dn9, locals.var_costi0_p2_dn10, locals.var_costi0_p2_dn11, locals.var_costi0_p2_dn14,)
    }
};
        locals.var_costi0_p2 = assign19500_e14571;
        locals.var_costi0_p2_dn0 = assign19500_e14571_d_n0;
        locals.var_costi0_p2_dn2 = assign19500_e14571_d_n2;
        locals.var_costi0_p2_dn4 = assign19500_e14571_d_n4;
        locals.var_costi0_p2_dn5 = assign19500_e14571_d_n5;
        locals.var_costi0_p2_dn6 = assign19500_e14571_d_n6;
        locals.var_costi0_p2_dn7 = assign19500_e14571_d_n7;
        locals.var_costi0_p2_dn8 = assign19500_e14571_d_n8;
        locals.var_costi0_p2_dn9 = assign19500_e14571_d_n9;
        locals.var_costi0_p2_dn10 = assign19500_e14571_d_n10;
        locals.var_costi0_p2_dn11 = assign19500_e14571_d_n11;
        locals.var_costi0_p2_dn14 = assign19500_e14571_d_n14;

        let (assign19510_e14579, assign19510_e14579_d_n0, assign19510_e14579_d_n2, assign19510_e14579_d_n4, assign19510_e14579_d_n5, assign19510_e14579_d_n6, assign19510_e14579_d_n7, assign19510_e14579_d_n8, assign19510_e14579_d_n9, assign19510_e14579_d_n10, assign19510_e14579_d_n11, assign19510_e14579_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign19510_e14575: f64 = (locals.var_nin * locals.var_nin);
        let assign19510_e14577: f64 = (assign19510_e14575 * locals.var_nsti_p2);
        (assign19510_e14577, (((locals.var_nin_dn0 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn0)) * locals.var_nsti_p2), (((locals.var_nin_dn2 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn2)) * locals.var_nsti_p2), (((locals.var_nin_dn4 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn4)) * locals.var_nsti_p2), (((locals.var_nin_dn5 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn5)) * locals.var_nsti_p2), (((locals.var_nin_dn6 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn6)) * locals.var_nsti_p2), (((locals.var_nin_dn7 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn7)) * locals.var_nsti_p2), (((locals.var_nin_dn8 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn8)) * locals.var_nsti_p2), (((locals.var_nin_dn9 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn9)) * locals.var_nsti_p2), (((locals.var_nin_dn10 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn10)) * locals.var_nsti_p2), (((locals.var_nin_dn11 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn11)) * locals.var_nsti_p2), (((locals.var_nin_dn14 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn14)) * locals.var_nsti_p2),)
    } else {
        (locals.var_costi1, locals.var_costi1_dn0, locals.var_costi1_dn2, locals.var_costi1_dn4, locals.var_costi1_dn5, locals.var_costi1_dn6, locals.var_costi1_dn7, locals.var_costi1_dn8, locals.var_costi1_dn9, locals.var_costi1_dn10, locals.var_costi1_dn11, locals.var_costi1_dn14,)
    }
};
        locals.var_costi1 = assign19510_e14579;
        locals.var_costi1_dn0 = assign19510_e14579_d_n0;
        locals.var_costi1_dn2 = assign19510_e14579_d_n2;
        locals.var_costi1_dn4 = assign19510_e14579_d_n4;
        locals.var_costi1_dn5 = assign19510_e14579_d_n5;
        locals.var_costi1_dn6 = assign19510_e14579_d_n6;
        locals.var_costi1_dn7 = assign19510_e14579_d_n7;
        locals.var_costi1_dn8 = assign19510_e14579_d_n8;
        locals.var_costi1_dn9 = assign19510_e14579_d_n9;
        locals.var_costi1_dn10 = assign19510_e14579_d_n10;
        locals.var_costi1_dn11 = assign19510_e14579_d_n11;
        locals.var_costi1_dn14 = assign19510_e14579_d_n14;

        let (assign19520_e14587, assign19520_e14587_d_n0, assign19520_e14587_d_n2, assign19520_e14587_d_n4, assign19520_e14587_d_n5, assign19520_e14587_d_n6, assign19520_e14587_d_n7, assign19520_e14587_d_n8, assign19520_e14587_d_n9, assign19520_e14587_d_n10, assign19520_e14587_d_n11, assign19520_e14587_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign19520_e14584: f64 = (p.p448 * locals.var_tdiff);
        let assign19520_e14585: f64 = (p.p447 + assign19520_e14584);
        (assign19520_e14585, (p.p448 * locals.var_tdiff_dn0), (p.p448 * locals.var_tdiff_dn2), (p.p448 * locals.var_tdiff_dn4), (p.p448 * locals.var_tdiff_dn5), (p.p448 * locals.var_tdiff_dn6), (p.p448 * locals.var_tdiff_dn7), (p.p448 * locals.var_tdiff_dn8), (p.p448 * locals.var_tdiff_dn9), (p.p448 * locals.var_tdiff_dn10), (p.p448 * locals.var_tdiff_dn11), (p.p448 * locals.var_tdiff_dn14),)
    } else {
        (locals.var_hbdceff, locals.var_hbdceff_dn0, locals.var_hbdceff_dn2, locals.var_hbdceff_dn4, locals.var_hbdceff_dn5, locals.var_hbdceff_dn6, locals.var_hbdceff_dn7, locals.var_hbdceff_dn8, locals.var_hbdceff_dn9, locals.var_hbdceff_dn10, locals.var_hbdceff_dn11, locals.var_hbdceff_dn14,)
    }
};
        locals.var_hbdceff = assign19520_e14587;
        locals.var_hbdceff_dn0 = assign19520_e14587_d_n0;
        locals.var_hbdceff_dn2 = assign19520_e14587_d_n2;
        locals.var_hbdceff_dn4 = assign19520_e14587_d_n4;
        locals.var_hbdceff_dn5 = assign19520_e14587_d_n5;
        locals.var_hbdceff_dn6 = assign19520_e14587_d_n6;
        locals.var_hbdceff_dn7 = assign19520_e14587_d_n7;
        locals.var_hbdceff_dn8 = assign19520_e14587_d_n8;
        locals.var_hbdceff_dn9 = assign19520_e14587_d_n9;
        locals.var_hbdceff_dn10 = assign19520_e14587_d_n10;
        locals.var_hbdceff_dn11 = assign19520_e14587_d_n11;
        locals.var_hbdceff_dn14 = assign19520_e14587_d_n14;

        let (assign19530_e14591,) = {
    if (locals.var_guard356 != 0.0) {
        (p.p193,)
    } else {
        (locals.var_uc_subtmp,)
    }
};
        locals.var_uc_subtmp = assign19530_e14591;

        let assign19560_e14604: f64 = if locals.var_uc_subtmp < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard393 = assign19560_e14604;

        let (assign19570_e14610,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard393 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_uc_subtmp,)
    }
};
        locals.var_uc_subtmp = assign19570_e14610;

        let assign19580_e14613: f64 = if locals.var_uc_subtmp > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard394 = assign19580_e14613;

        let (assign19590_e14619,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard394 != 0.0)) {
        (0.005,)
    } else {
        (locals.var_uc_subtmp,)
    }
};
        locals.var_uc_subtmp = assign19590_e14619;

        let (assign19600_e14626, assign19600_e14626_d_n0, assign19600_e14626_d_n2, assign19600_e14626_d_n4, assign19600_e14626_d_n5, assign19600_e14626_d_n6, assign19600_e14626_d_n7, assign19600_e14626_d_n8, assign19600_e14626_d_n9, assign19600_e14626_d_n10, assign19600_e14626_d_n11, assign19600_e14626_d_n14,) = {
    if (locals.var_guard356 == 0.0) {
        let assign19600_e14622: f64 = ctx_temp;
        let assign19600_e14624: f64 = (assign19600_e14622 + p.p11);
        (assign19600_e14624, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ttemp, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn11, locals.var_ttemp_dn14,)
    }
};
        locals.var_ttemp = assign19600_e14626;
        locals.var_ttemp_dn0 = assign19600_e14626_d_n0;
        locals.var_ttemp_dn2 = assign19600_e14626_d_n2;
        locals.var_ttemp_dn4 = assign19600_e14626_d_n4;
        locals.var_ttemp_dn5 = assign19600_e14626_d_n5;
        locals.var_ttemp_dn6 = assign19600_e14626_d_n6;
        locals.var_ttemp_dn7 = assign19600_e14626_d_n7;
        locals.var_ttemp_dn8 = assign19600_e14626_d_n8;
        locals.var_ttemp_dn9 = assign19600_e14626_d_n9;
        locals.var_ttemp_dn10 = assign19600_e14626_d_n10;
        locals.var_ttemp_dn11 = assign19600_e14626_d_n11;
        locals.var_ttemp_dn14 = assign19600_e14626_d_n14;

        let assign19610_e14629: f64 = (locals.var_weff_ld * p.p7);
        locals.var_weffld_nf = assign19610_e14629;

        let assign19620_e14632: f64 = (p.p67 + p.p68);
        locals.var_ldrift0 = assign19620_e14632;

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
        locals.var_vmaxe_dn11 = locals.var_vmaxeff_dn11;
        locals.var_vmaxe_dn14 = locals.var_vmaxeff_dn14;

        locals.var_c_eox = locals.var_cecox;

        locals.var_tox0 = p.p95;

        let assign19670_e14639: f64 = (locals.var_c_eox / locals.var_tox0);
        locals.var_cox0 = assign19670_e14639;

        let assign19680_e14642: f64 = (1.0 / locals.var_cox0);
        locals.var_cox0_inv = assign19680_e14642;

        let assign19690_e14645: f64 = (locals.var_c_eox / locals.var_uc_toxb);
        locals.var_coxb0 = assign19690_e14645;

        let assign19700_e14648: f64 = (p.p87 * p.p434);
        locals.var_vgs_min = assign19700_e14648;

        let assign19710_e14652: f64 = (locals.var_pb2 - p.p262);
        let assign19710_e14653: f64 = (0.8 - assign19710_e14652);
        let assign19710_e14655: f64 = (assign19710_e14653 - 0.1);
        locals.var_tmf1 = assign19710_e14655;
        locals.var_tmf1_dn0 = (-locals.var_pb2_dn0);
        locals.var_tmf1_dn2 = (-locals.var_pb2_dn2);
        locals.var_tmf1_dn4 = (-locals.var_pb2_dn4);
        locals.var_tmf1_dn5 = (-locals.var_pb2_dn5);
        locals.var_tmf1_dn6 = (-locals.var_pb2_dn6);
        locals.var_tmf1_dn7 = (-locals.var_pb2_dn7);
        locals.var_tmf1_dn8 = (-locals.var_pb2_dn8);
        locals.var_tmf1_dn9 = (-locals.var_pb2_dn9);
        locals.var_tmf1_dn10 = (-locals.var_pb2_dn10);
        locals.var_tmf1_dn11 = (-locals.var_pb2_dn11);
        locals.var_tmf1_dn14 = (-locals.var_pb2_dn14);

        let assign19720_e14658: f64 = (4.0 * 0.8);
        let assign19720_e14660: f64 = (assign19720_e14658 * 0.1);
        locals.var_tmf2 = assign19720_e14660;
        locals.var_tmf2_dn0 = 0.0;
        locals.var_tmf2_dn2 = 0.0;
        locals.var_tmf2_dn4 = 0.0;
        locals.var_tmf2_dn5 = 0.0;
        locals.var_tmf2_dn6 = 0.0;
        locals.var_tmf2_dn7 = 0.0;
        locals.var_tmf2_dn8 = 0.0;
        locals.var_tmf2_dn9 = 0.0;
        locals.var_tmf2_dn10 = 0.0;
        locals.var_tmf2_dn11 = 0.0;
        locals.var_tmf2_dn14 = 0.0;

        let (assign19730_e14667, assign19730_e14667_d_n0, assign19730_e14667_d_n2, assign19730_e14667_d_n4, assign19730_e14667_d_n5, assign19730_e14667_d_n6, assign19730_e14667_d_n7, assign19730_e14667_d_n8, assign19730_e14667_d_n9, assign19730_e14667_d_n10, assign19730_e14667_d_n11, assign19730_e14667_d_n14,) = {
    if (locals.var_tmf2 > 0.0) {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    } else {
        let assign19730_e14666: f64 = (-locals.var_tmf2);
        (assign19730_e14666, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
    }
};
        locals.var_tmf2 = assign19730_e14667;
        locals.var_tmf2_dn0 = assign19730_e14667_d_n0;
        locals.var_tmf2_dn2 = assign19730_e14667_d_n2;
        locals.var_tmf2_dn4 = assign19730_e14667_d_n4;
        locals.var_tmf2_dn5 = assign19730_e14667_d_n5;
        locals.var_tmf2_dn6 = assign19730_e14667_d_n6;
        locals.var_tmf2_dn7 = assign19730_e14667_d_n7;
        locals.var_tmf2_dn8 = assign19730_e14667_d_n8;
        locals.var_tmf2_dn9 = assign19730_e14667_d_n9;
        locals.var_tmf2_dn10 = assign19730_e14667_d_n10;
        locals.var_tmf2_dn11 = assign19730_e14667_d_n11;
        locals.var_tmf2_dn14 = assign19730_e14667_d_n14;

        let assign19740_e14670: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign19740_e14672: f64 = (assign19740_e14670 + locals.var_tmf2);
        let assign19740_e14673: f64 = (assign19740_e14672).sqrt();
        locals.var_tmf2 = assign19740_e14673;
        locals.var_tmf2_dn0 = ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign19740_e14673));
        locals.var_tmf2_dn2 = ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign19740_e14673));
        locals.var_tmf2_dn4 = ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign19740_e14673));
        locals.var_tmf2_dn5 = ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign19740_e14673));
        locals.var_tmf2_dn6 = ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign19740_e14673));
        locals.var_tmf2_dn7 = ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign19740_e14673));
        locals.var_tmf2_dn8 = ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign19740_e14673));
        locals.var_tmf2_dn9 = ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign19740_e14673));
        locals.var_tmf2_dn10 = ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign19740_e14673));
        locals.var_tmf2_dn11 = ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign19740_e14673));
        locals.var_tmf2_dn14 = ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign19740_e14673));

        let assign19750_e14678: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign19750_e14679: f64 = (1.0 + assign19750_e14678);
        let assign19750_e14680: f64 = (0.5 * assign19750_e14679);
        locals.var_t0 = assign19750_e14680;
        locals.var_t0_dn0 = (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn2 = (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn4 = (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn5 = (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn6 = (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn7 = (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn8 = (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn9 = (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn10 = (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn11 = (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn14 = (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2)));

        let assign19760_e14685: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign19760_e14686: f64 = (0.5 * assign19760_e14685);
        let assign19760_e14687: f64 = (0.8 - assign19760_e14686);
        locals.var_t1 = assign19760_e14687;
        locals.var_t1_dn0 = (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)));
        locals.var_t1_dn2 = (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)));
        locals.var_t1_dn4 = (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)));
        locals.var_t1_dn5 = (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)));
        locals.var_t1_dn6 = (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)));
        locals.var_t1_dn7 = (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)));
        locals.var_t1_dn8 = (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)));
        locals.var_t1_dn9 = (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)));
        locals.var_t1_dn10 = (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)));
        locals.var_t1_dn11 = (-(0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)));
        locals.var_t1_dn14 = (-(0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)));

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
        locals.var_vbs_max_dn11 = locals.var_t1_dn11;
        locals.var_vbs_max_dn14 = locals.var_t1_dn14;

        let assign19780_e14691: f64 = (locals.var_pb20 - p.p262);
        let assign19780_e14693: f64 = if assign19780_e14691 < locals.var_vbs_max { 1.0 } else { 0.0 };
        locals.var_guard395 = assign19780_e14693;

        let (assign19790_e14699, assign19790_e14699_d_n0, assign19790_e14699_d_n2, assign19790_e14699_d_n4, assign19790_e14699_d_n5, assign19790_e14699_d_n6, assign19790_e14699_d_n7, assign19790_e14699_d_n8, assign19790_e14699_d_n9, assign19790_e14699_d_n10, assign19790_e14699_d_n11, assign19790_e14699_d_n14,) = {
    if (locals.var_guard395 != 0.0) {
        let assign19790_e14697: f64 = (locals.var_pb20 - p.p262);
        (assign19790_e14697, locals.var_pb20_dn0, locals.var_pb20_dn2, locals.var_pb20_dn4, locals.var_pb20_dn5, locals.var_pb20_dn6, locals.var_pb20_dn7, locals.var_pb20_dn8, locals.var_pb20_dn9, locals.var_pb20_dn10, locals.var_pb20_dn11, locals.var_pb20_dn14,)
    } else {
        (locals.var_vbs_max, locals.var_vbs_max_dn0, locals.var_vbs_max_dn2, locals.var_vbs_max_dn4, locals.var_vbs_max_dn5, locals.var_vbs_max_dn6, locals.var_vbs_max_dn7, locals.var_vbs_max_dn8, locals.var_vbs_max_dn9, locals.var_vbs_max_dn10, locals.var_vbs_max_dn11, locals.var_vbs_max_dn14,)
    }
};
        locals.var_vbs_max = assign19790_e14699;
        locals.var_vbs_max_dn0 = assign19790_e14699_d_n0;
        locals.var_vbs_max_dn2 = assign19790_e14699_d_n2;
        locals.var_vbs_max_dn4 = assign19790_e14699_d_n4;
        locals.var_vbs_max_dn5 = assign19790_e14699_d_n5;
        locals.var_vbs_max_dn6 = assign19790_e14699_d_n6;
        locals.var_vbs_max_dn7 = assign19790_e14699_d_n7;
        locals.var_vbs_max_dn8 = assign19790_e14699_d_n8;
        locals.var_vbs_max_dn9 = assign19790_e14699_d_n9;
        locals.var_vbs_max_dn10 = assign19790_e14699_d_n10;
        locals.var_vbs_max_dn11 = assign19790_e14699_d_n11;
        locals.var_vbs_max_dn14 = assign19790_e14699_d_n14;

        let assign19800_e14702: f64 = (locals.var_pb2c - p.p262);
        let assign19800_e14704: f64 = if assign19800_e14702 < locals.var_vbs_max { 1.0 } else { 0.0 };
        locals.var_guard396 = assign19800_e14704;

        let (assign19810_e14710, assign19810_e14710_d_n0, assign19810_e14710_d_n2, assign19810_e14710_d_n4, assign19810_e14710_d_n5, assign19810_e14710_d_n6, assign19810_e14710_d_n7, assign19810_e14710_d_n8, assign19810_e14710_d_n9, assign19810_e14710_d_n10, assign19810_e14710_d_n11, assign19810_e14710_d_n14,) = {
    if (locals.var_guard396 != 0.0) {
        let assign19810_e14708: f64 = (locals.var_pb2c - p.p262);
        (assign19810_e14708, locals.var_pb2c_dn0, locals.var_pb2c_dn2, locals.var_pb2c_dn4, locals.var_pb2c_dn5, locals.var_pb2c_dn6, locals.var_pb2c_dn7, locals.var_pb2c_dn8, locals.var_pb2c_dn9, locals.var_pb2c_dn10, locals.var_pb2c_dn11, locals.var_pb2c_dn14,)
    } else {
        (locals.var_vbs_max, locals.var_vbs_max_dn0, locals.var_vbs_max_dn2, locals.var_vbs_max_dn4, locals.var_vbs_max_dn5, locals.var_vbs_max_dn6, locals.var_vbs_max_dn7, locals.var_vbs_max_dn8, locals.var_vbs_max_dn9, locals.var_vbs_max_dn10, locals.var_vbs_max_dn11, locals.var_vbs_max_dn14,)
    }
};
        locals.var_vbs_max = assign19810_e14710;
        locals.var_vbs_max_dn0 = assign19810_e14710_d_n0;
        locals.var_vbs_max_dn2 = assign19810_e14710_d_n2;
        locals.var_vbs_max_dn4 = assign19810_e14710_d_n4;
        locals.var_vbs_max_dn5 = assign19810_e14710_d_n5;
        locals.var_vbs_max_dn6 = assign19810_e14710_d_n6;
        locals.var_vbs_max_dn7 = assign19810_e14710_d_n7;
        locals.var_vbs_max_dn8 = assign19810_e14710_d_n8;
        locals.var_vbs_max_dn9 = assign19810_e14710_d_n9;
        locals.var_vbs_max_dn10 = assign19810_e14710_d_n10;
        locals.var_vbs_max_dn11 = assign19810_e14710_d_n11;
        locals.var_vbs_max_dn14 = assign19810_e14710_d_n14;

        let assign19820_e14717: f64 = if ((locals.var_uc_codep > 0.0) && (locals.var_uc_codep <= 3.0)) { 1.0 } else { 0.0 };
        locals.var_guard397 = assign19820_e14717;

        let assign19830_e14720: f64 = (locals.var_pb2n - p.p262);
        let assign19830_e14722: f64 = if assign19830_e14720 < locals.var_vbs_max { 1.0 } else { 0.0 };
        locals.var_guard398 = assign19830_e14722;

        let (assign19840_e14730, assign19840_e14730_d_n0, assign19840_e14730_d_n2, assign19840_e14730_d_n4, assign19840_e14730_d_n5, assign19840_e14730_d_n6, assign19840_e14730_d_n7, assign19840_e14730_d_n8, assign19840_e14730_d_n9, assign19840_e14730_d_n10, assign19840_e14730_d_n11, assign19840_e14730_d_n14,) = {
    if ((locals.var_guard397 != 0.0) && (locals.var_guard398 != 0.0)) {
        let assign19840_e14728: f64 = (locals.var_pb2n - p.p262);
        (assign19840_e14728, locals.var_pb2n_dn0, locals.var_pb2n_dn2, locals.var_pb2n_dn4, locals.var_pb2n_dn5, locals.var_pb2n_dn6, locals.var_pb2n_dn7, locals.var_pb2n_dn8, locals.var_pb2n_dn9, locals.var_pb2n_dn10, locals.var_pb2n_dn11, locals.var_pb2n_dn14,)
    } else {
        (locals.var_vbs_max, locals.var_vbs_max_dn0, locals.var_vbs_max_dn2, locals.var_vbs_max_dn4, locals.var_vbs_max_dn5, locals.var_vbs_max_dn6, locals.var_vbs_max_dn7, locals.var_vbs_max_dn8, locals.var_vbs_max_dn9, locals.var_vbs_max_dn10, locals.var_vbs_max_dn11, locals.var_vbs_max_dn14,)
    }
};
        locals.var_vbs_max = assign19840_e14730;
        locals.var_vbs_max_dn0 = assign19840_e14730_d_n0;
        locals.var_vbs_max_dn2 = assign19840_e14730_d_n2;
        locals.var_vbs_max_dn4 = assign19840_e14730_d_n4;
        locals.var_vbs_max_dn5 = assign19840_e14730_d_n5;
        locals.var_vbs_max_dn6 = assign19840_e14730_d_n6;
        locals.var_vbs_max_dn7 = assign19840_e14730_d_n7;
        locals.var_vbs_max_dn8 = assign19840_e14730_d_n8;
        locals.var_vbs_max_dn9 = assign19840_e14730_d_n9;
        locals.var_vbs_max_dn10 = assign19840_e14730_d_n10;
        locals.var_vbs_max_dn11 = assign19840_e14730_d_n11;
        locals.var_vbs_max_dn14 = assign19840_e14730_d_n14;

        let assign19850_e14733: f64 = (locals.var_vbipn - p.p262);
        let assign19850_e14735: f64 = if assign19850_e14733 < locals.var_vbs_max { 1.0 } else { 0.0 };
        locals.var_guard399 = assign19850_e14735;

    }

    pub(super) fn stamp_transient_block_47(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign19860_e14743, assign19860_e14743_d_n0, assign19860_e14743_d_n2, assign19860_e14743_d_n4, assign19860_e14743_d_n5, assign19860_e14743_d_n6, assign19860_e14743_d_n7, assign19860_e14743_d_n8, assign19860_e14743_d_n9, assign19860_e14743_d_n10, assign19860_e14743_d_n11, assign19860_e14743_d_n14,) = {
    if ((locals.var_guard397 != 0.0) && (locals.var_guard399 != 0.0)) {
        let assign19860_e14741: f64 = (locals.var_vbipn - p.p262);
        (assign19860_e14741, locals.var_vbipn_dn0, locals.var_vbipn_dn2, locals.var_vbipn_dn4, locals.var_vbipn_dn5, locals.var_vbipn_dn6, locals.var_vbipn_dn7, locals.var_vbipn_dn8, locals.var_vbipn_dn9, locals.var_vbipn_dn10, locals.var_vbipn_dn11, locals.var_vbipn_dn14,)
    } else {
        (locals.var_vbs_max, locals.var_vbs_max_dn0, locals.var_vbs_max_dn2, locals.var_vbs_max_dn4, locals.var_vbs_max_dn5, locals.var_vbs_max_dn6, locals.var_vbs_max_dn7, locals.var_vbs_max_dn8, locals.var_vbs_max_dn9, locals.var_vbs_max_dn10, locals.var_vbs_max_dn11, locals.var_vbs_max_dn14,)
    }
};
        locals.var_vbs_max = assign19860_e14743;
        locals.var_vbs_max_dn0 = assign19860_e14743_d_n0;
        locals.var_vbs_max_dn2 = assign19860_e14743_d_n2;
        locals.var_vbs_max_dn4 = assign19860_e14743_d_n4;
        locals.var_vbs_max_dn5 = assign19860_e14743_d_n5;
        locals.var_vbs_max_dn6 = assign19860_e14743_d_n6;
        locals.var_vbs_max_dn7 = assign19860_e14743_d_n7;
        locals.var_vbs_max_dn8 = assign19860_e14743_d_n8;
        locals.var_vbs_max_dn9 = assign19860_e14743_d_n9;
        locals.var_vbs_max_dn10 = assign19860_e14743_d_n10;
        locals.var_vbs_max_dn11 = assign19860_e14743_d_n11;
        locals.var_vbs_max_dn14 = assign19860_e14743_d_n14;

        let assign19870_e14747: f64 = (locals.var_vbs_max * 0.5);
        let assign19870_e14748: f64 = if locals.var_vbs_bnd > assign19870_e14747 { 1.0 } else { 0.0 };
        locals.var_guard400 = assign19870_e14748;

        let (assign19880_e14754, assign19880_e14754_d_n0, assign19880_e14754_d_n2, assign19880_e14754_d_n4, assign19880_e14754_d_n5, assign19880_e14754_d_n6, assign19880_e14754_d_n7, assign19880_e14754_d_n8, assign19880_e14754_d_n9, assign19880_e14754_d_n10, assign19880_e14754_d_n11, assign19880_e14754_d_n14,) = {
    if (locals.var_guard400 != 0.0) {
        let assign19880_e14752: f64 = (0.5 * locals.var_vbs_max);
        (assign19880_e14752, (0.5 * locals.var_vbs_max_dn0), (0.5 * locals.var_vbs_max_dn2), (0.5 * locals.var_vbs_max_dn4), (0.5 * locals.var_vbs_max_dn5), (0.5 * locals.var_vbs_max_dn6), (0.5 * locals.var_vbs_max_dn7), (0.5 * locals.var_vbs_max_dn8), (0.5 * locals.var_vbs_max_dn9), (0.5 * locals.var_vbs_max_dn10), (0.5 * locals.var_vbs_max_dn11), (0.5 * locals.var_vbs_max_dn14),)
    } else {
        (locals.var_vbs_bnd, locals.var_vbs_bnd_dn0, locals.var_vbs_bnd_dn2, locals.var_vbs_bnd_dn4, locals.var_vbs_bnd_dn5, locals.var_vbs_bnd_dn6, locals.var_vbs_bnd_dn7, locals.var_vbs_bnd_dn8, locals.var_vbs_bnd_dn9, locals.var_vbs_bnd_dn10, locals.var_vbs_bnd_dn11, locals.var_vbs_bnd_dn14,)
    }
};
        locals.var_vbs_bnd = assign19880_e14754;
        locals.var_vbs_bnd_dn0 = assign19880_e14754_d_n0;
        locals.var_vbs_bnd_dn2 = assign19880_e14754_d_n2;
        locals.var_vbs_bnd_dn4 = assign19880_e14754_d_n4;
        locals.var_vbs_bnd_dn5 = assign19880_e14754_d_n5;
        locals.var_vbs_bnd_dn6 = assign19880_e14754_d_n6;
        locals.var_vbs_bnd_dn7 = assign19880_e14754_d_n7;
        locals.var_vbs_bnd_dn8 = assign19880_e14754_d_n8;
        locals.var_vbs_bnd_dn9 = assign19880_e14754_d_n9;
        locals.var_vbs_bnd_dn10 = assign19880_e14754_d_n10;
        locals.var_vbs_bnd_dn11 = assign19880_e14754_d_n11;
        locals.var_vbs_bnd_dn14 = assign19880_e14754_d_n14;

        let assign19890_e14756: f64 = if param_given[338] { 1.0 } else { 0.0 };
        locals.var_guard401 = assign19890_e14756;

        let (assign19900_e14760, assign19900_e14760_d_n0, assign19900_e14760_d_n2, assign19900_e14760_d_n4, assign19900_e14760_d_n5, assign19900_e14760_d_n6, assign19900_e14760_d_n7, assign19900_e14760_d_n8, assign19900_e14760_d_n9, assign19900_e14760_d_n10, assign19900_e14760_d_n11, assign19900_e14760_d_n14,) = {
    if (locals.var_guard401 != 0.0) {
        (p.p338, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_max_local, locals.var_vbs_max_local_dn0, locals.var_vbs_max_local_dn2, locals.var_vbs_max_local_dn4, locals.var_vbs_max_local_dn5, locals.var_vbs_max_local_dn6, locals.var_vbs_max_local_dn7, locals.var_vbs_max_local_dn8, locals.var_vbs_max_local_dn9, locals.var_vbs_max_local_dn10, locals.var_vbs_max_local_dn11, locals.var_vbs_max_local_dn14,)
    }
};
        locals.var_vbs_max_local = assign19900_e14760;
        locals.var_vbs_max_local_dn0 = assign19900_e14760_d_n0;
        locals.var_vbs_max_local_dn2 = assign19900_e14760_d_n2;
        locals.var_vbs_max_local_dn4 = assign19900_e14760_d_n4;
        locals.var_vbs_max_local_dn5 = assign19900_e14760_d_n5;
        locals.var_vbs_max_local_dn6 = assign19900_e14760_d_n6;
        locals.var_vbs_max_local_dn7 = assign19900_e14760_d_n7;
        locals.var_vbs_max_local_dn8 = assign19900_e14760_d_n8;
        locals.var_vbs_max_local_dn9 = assign19900_e14760_d_n9;
        locals.var_vbs_max_local_dn10 = assign19900_e14760_d_n10;
        locals.var_vbs_max_local_dn11 = assign19900_e14760_d_n11;
        locals.var_vbs_max_local_dn14 = assign19900_e14760_d_n14;

        let (assign19910_e14765, assign19910_e14765_d_n0, assign19910_e14765_d_n2, assign19910_e14765_d_n4, assign19910_e14765_d_n5, assign19910_e14765_d_n6, assign19910_e14765_d_n7, assign19910_e14765_d_n8, assign19910_e14765_d_n9, assign19910_e14765_d_n10, assign19910_e14765_d_n11, assign19910_e14765_d_n14,) = {
    if (locals.var_guard401 == 0.0) {
        (locals.var_vbs_max, locals.var_vbs_max_dn0, locals.var_vbs_max_dn2, locals.var_vbs_max_dn4, locals.var_vbs_max_dn5, locals.var_vbs_max_dn6, locals.var_vbs_max_dn7, locals.var_vbs_max_dn8, locals.var_vbs_max_dn9, locals.var_vbs_max_dn10, locals.var_vbs_max_dn11, locals.var_vbs_max_dn14,)
    } else {
        (locals.var_vbs_max_local, locals.var_vbs_max_local_dn0, locals.var_vbs_max_local_dn2, locals.var_vbs_max_local_dn4, locals.var_vbs_max_local_dn5, locals.var_vbs_max_local_dn6, locals.var_vbs_max_local_dn7, locals.var_vbs_max_local_dn8, locals.var_vbs_max_local_dn9, locals.var_vbs_max_local_dn10, locals.var_vbs_max_local_dn11, locals.var_vbs_max_local_dn14,)
    }
};
        locals.var_vbs_max_local = assign19910_e14765;
        locals.var_vbs_max_local_dn0 = assign19910_e14765_d_n0;
        locals.var_vbs_max_local_dn2 = assign19910_e14765_d_n2;
        locals.var_vbs_max_local_dn4 = assign19910_e14765_d_n4;
        locals.var_vbs_max_local_dn5 = assign19910_e14765_d_n5;
        locals.var_vbs_max_local_dn6 = assign19910_e14765_d_n6;
        locals.var_vbs_max_local_dn7 = assign19910_e14765_d_n7;
        locals.var_vbs_max_local_dn8 = assign19910_e14765_d_n8;
        locals.var_vbs_max_local_dn9 = assign19910_e14765_d_n9;
        locals.var_vbs_max_local_dn10 = assign19910_e14765_d_n10;
        locals.var_vbs_max_local_dn11 = assign19910_e14765_d_n11;
        locals.var_vbs_max_local_dn14 = assign19910_e14765_d_n14;

        let assign19920_e14767: f64 = if param_given[339] { 1.0 } else { 0.0 };
        locals.var_guard402 = assign19920_e14767;

        let (assign19930_e14771, assign19930_e14771_d_n0, assign19930_e14771_d_n2, assign19930_e14771_d_n4, assign19930_e14771_d_n5, assign19930_e14771_d_n6, assign19930_e14771_d_n7, assign19930_e14771_d_n8, assign19930_e14771_d_n9, assign19930_e14771_d_n10, assign19930_e14771_d_n11, assign19930_e14771_d_n14,) = {
    if (locals.var_guard402 != 0.0) {
        (p.p339, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_bnd_local, locals.var_vbs_bnd_local_dn0, locals.var_vbs_bnd_local_dn2, locals.var_vbs_bnd_local_dn4, locals.var_vbs_bnd_local_dn5, locals.var_vbs_bnd_local_dn6, locals.var_vbs_bnd_local_dn7, locals.var_vbs_bnd_local_dn8, locals.var_vbs_bnd_local_dn9, locals.var_vbs_bnd_local_dn10, locals.var_vbs_bnd_local_dn11, locals.var_vbs_bnd_local_dn14,)
    }
};
        locals.var_vbs_bnd_local = assign19930_e14771;
        locals.var_vbs_bnd_local_dn0 = assign19930_e14771_d_n0;
        locals.var_vbs_bnd_local_dn2 = assign19930_e14771_d_n2;
        locals.var_vbs_bnd_local_dn4 = assign19930_e14771_d_n4;
        locals.var_vbs_bnd_local_dn5 = assign19930_e14771_d_n5;
        locals.var_vbs_bnd_local_dn6 = assign19930_e14771_d_n6;
        locals.var_vbs_bnd_local_dn7 = assign19930_e14771_d_n7;
        locals.var_vbs_bnd_local_dn8 = assign19930_e14771_d_n8;
        locals.var_vbs_bnd_local_dn9 = assign19930_e14771_d_n9;
        locals.var_vbs_bnd_local_dn10 = assign19930_e14771_d_n10;
        locals.var_vbs_bnd_local_dn11 = assign19930_e14771_d_n11;
        locals.var_vbs_bnd_local_dn14 = assign19930_e14771_d_n14;

        let assign19940_e14773: f64 = if param_given[338] { 1.0 } else { 0.0 };
        locals.var_guard403 = assign19940_e14773;

        let (assign19950_e14782, assign19950_e14782_d_n0, assign19950_e14782_d_n2, assign19950_e14782_d_n4, assign19950_e14782_d_n5, assign19950_e14782_d_n6, assign19950_e14782_d_n7, assign19950_e14782_d_n8, assign19950_e14782_d_n9, assign19950_e14782_d_n10, assign19950_e14782_d_n11, assign19950_e14782_d_n14,) = {
    if ((locals.var_guard402 == 0.0) && (locals.var_guard403 != 0.0)) {
        let assign19950_e14780: f64 = (0.5 * locals.var_vbs_max_local);
        (assign19950_e14780, (0.5 * locals.var_vbs_max_local_dn0), (0.5 * locals.var_vbs_max_local_dn2), (0.5 * locals.var_vbs_max_local_dn4), (0.5 * locals.var_vbs_max_local_dn5), (0.5 * locals.var_vbs_max_local_dn6), (0.5 * locals.var_vbs_max_local_dn7), (0.5 * locals.var_vbs_max_local_dn8), (0.5 * locals.var_vbs_max_local_dn9), (0.5 * locals.var_vbs_max_local_dn10), (0.5 * locals.var_vbs_max_local_dn11), (0.5 * locals.var_vbs_max_local_dn14),)
    } else {
        (locals.var_vbs_bnd_local, locals.var_vbs_bnd_local_dn0, locals.var_vbs_bnd_local_dn2, locals.var_vbs_bnd_local_dn4, locals.var_vbs_bnd_local_dn5, locals.var_vbs_bnd_local_dn6, locals.var_vbs_bnd_local_dn7, locals.var_vbs_bnd_local_dn8, locals.var_vbs_bnd_local_dn9, locals.var_vbs_bnd_local_dn10, locals.var_vbs_bnd_local_dn11, locals.var_vbs_bnd_local_dn14,)
    }
};
        locals.var_vbs_bnd_local = assign19950_e14782;
        locals.var_vbs_bnd_local_dn0 = assign19950_e14782_d_n0;
        locals.var_vbs_bnd_local_dn2 = assign19950_e14782_d_n2;
        locals.var_vbs_bnd_local_dn4 = assign19950_e14782_d_n4;
        locals.var_vbs_bnd_local_dn5 = assign19950_e14782_d_n5;
        locals.var_vbs_bnd_local_dn6 = assign19950_e14782_d_n6;
        locals.var_vbs_bnd_local_dn7 = assign19950_e14782_d_n7;
        locals.var_vbs_bnd_local_dn8 = assign19950_e14782_d_n8;
        locals.var_vbs_bnd_local_dn9 = assign19950_e14782_d_n9;
        locals.var_vbs_bnd_local_dn10 = assign19950_e14782_d_n10;
        locals.var_vbs_bnd_local_dn11 = assign19950_e14782_d_n11;
        locals.var_vbs_bnd_local_dn14 = assign19950_e14782_d_n14;

        let (assign19960_e14790, assign19960_e14790_d_n0, assign19960_e14790_d_n2, assign19960_e14790_d_n4, assign19960_e14790_d_n5, assign19960_e14790_d_n6, assign19960_e14790_d_n7, assign19960_e14790_d_n8, assign19960_e14790_d_n9, assign19960_e14790_d_n10, assign19960_e14790_d_n11, assign19960_e14790_d_n14,) = {
    if ((locals.var_guard402 == 0.0) && (locals.var_guard403 == 0.0)) {
        (locals.var_vbs_bnd, locals.var_vbs_bnd_dn0, locals.var_vbs_bnd_dn2, locals.var_vbs_bnd_dn4, locals.var_vbs_bnd_dn5, locals.var_vbs_bnd_dn6, locals.var_vbs_bnd_dn7, locals.var_vbs_bnd_dn8, locals.var_vbs_bnd_dn9, locals.var_vbs_bnd_dn10, locals.var_vbs_bnd_dn11, locals.var_vbs_bnd_dn14,)
    } else {
        (locals.var_vbs_bnd_local, locals.var_vbs_bnd_local_dn0, locals.var_vbs_bnd_local_dn2, locals.var_vbs_bnd_local_dn4, locals.var_vbs_bnd_local_dn5, locals.var_vbs_bnd_local_dn6, locals.var_vbs_bnd_local_dn7, locals.var_vbs_bnd_local_dn8, locals.var_vbs_bnd_local_dn9, locals.var_vbs_bnd_local_dn10, locals.var_vbs_bnd_local_dn11, locals.var_vbs_bnd_local_dn14,)
    }
};
        locals.var_vbs_bnd_local = assign19960_e14790;
        locals.var_vbs_bnd_local_dn0 = assign19960_e14790_d_n0;
        locals.var_vbs_bnd_local_dn2 = assign19960_e14790_d_n2;
        locals.var_vbs_bnd_local_dn4 = assign19960_e14790_d_n4;
        locals.var_vbs_bnd_local_dn5 = assign19960_e14790_d_n5;
        locals.var_vbs_bnd_local_dn6 = assign19960_e14790_d_n6;
        locals.var_vbs_bnd_local_dn7 = assign19960_e14790_d_n7;
        locals.var_vbs_bnd_local_dn8 = assign19960_e14790_d_n8;
        locals.var_vbs_bnd_local_dn9 = assign19960_e14790_d_n9;
        locals.var_vbs_bnd_local_dn10 = assign19960_e14790_d_n10;
        locals.var_vbs_bnd_local_dn11 = assign19960_e14790_d_n11;
        locals.var_vbs_bnd_local_dn14 = assign19960_e14790_d_n14;

        let assign19970_e14794: f64 = (locals.var_vbs_max_local * 0.5);
        let assign19970_e14795: f64 = if locals.var_vbs_bnd_local > assign19970_e14794 { 1.0 } else { 0.0 };
        locals.var_guard404 = assign19970_e14795;

        let (assign19980_e14801, assign19980_e14801_d_n0, assign19980_e14801_d_n2, assign19980_e14801_d_n4, assign19980_e14801_d_n5, assign19980_e14801_d_n6, assign19980_e14801_d_n7, assign19980_e14801_d_n8, assign19980_e14801_d_n9, assign19980_e14801_d_n10, assign19980_e14801_d_n11, assign19980_e14801_d_n14,) = {
    if (locals.var_guard404 != 0.0) {
        let assign19980_e14799: f64 = (0.5 * locals.var_vbs_max_local);
        (assign19980_e14799, (0.5 * locals.var_vbs_max_local_dn0), (0.5 * locals.var_vbs_max_local_dn2), (0.5 * locals.var_vbs_max_local_dn4), (0.5 * locals.var_vbs_max_local_dn5), (0.5 * locals.var_vbs_max_local_dn6), (0.5 * locals.var_vbs_max_local_dn7), (0.5 * locals.var_vbs_max_local_dn8), (0.5 * locals.var_vbs_max_local_dn9), (0.5 * locals.var_vbs_max_local_dn10), (0.5 * locals.var_vbs_max_local_dn11), (0.5 * locals.var_vbs_max_local_dn14),)
    } else {
        (locals.var_vbs_bnd_local, locals.var_vbs_bnd_local_dn0, locals.var_vbs_bnd_local_dn2, locals.var_vbs_bnd_local_dn4, locals.var_vbs_bnd_local_dn5, locals.var_vbs_bnd_local_dn6, locals.var_vbs_bnd_local_dn7, locals.var_vbs_bnd_local_dn8, locals.var_vbs_bnd_local_dn9, locals.var_vbs_bnd_local_dn10, locals.var_vbs_bnd_local_dn11, locals.var_vbs_bnd_local_dn14,)
    }
};
        locals.var_vbs_bnd_local = assign19980_e14801;
        locals.var_vbs_bnd_local_dn0 = assign19980_e14801_d_n0;
        locals.var_vbs_bnd_local_dn2 = assign19980_e14801_d_n2;
        locals.var_vbs_bnd_local_dn4 = assign19980_e14801_d_n4;
        locals.var_vbs_bnd_local_dn5 = assign19980_e14801_d_n5;
        locals.var_vbs_bnd_local_dn6 = assign19980_e14801_d_n6;
        locals.var_vbs_bnd_local_dn7 = assign19980_e14801_d_n7;
        locals.var_vbs_bnd_local_dn8 = assign19980_e14801_d_n8;
        locals.var_vbs_bnd_local_dn9 = assign19980_e14801_d_n9;
        locals.var_vbs_bnd_local_dn10 = assign19980_e14801_d_n10;
        locals.var_vbs_bnd_local_dn11 = assign19980_e14801_d_n11;
        locals.var_vbs_bnd_local_dn14 = assign19980_e14801_d_n14;

        let assign19990_e14808: f64 = if ((locals.var_rse > 0.0) || (locals.var_rde > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard405 = assign19990_e14808;

        let assign20000_e14811: f64 = if locals.var_uc_corsrd == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard406 = assign20000_e14811;

        let (assign20010_e14817,) = {
    if ((locals.var_guard405 != 0.0) && (locals.var_guard406 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_rsrd,)
    }
};
        locals.var_flg_rsrd = assign20010_e14817;

        let assign20020_e14820: f64 = if locals.var_uc_corsrd == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard407 = assign20020_e14820;

        let (assign20030_e14826,) = {
    if ((locals.var_guard405 != 0.0) && (locals.var_guard407 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_flg_rsrd,)
    }
};
        locals.var_flg_rsrd = assign20030_e14826;

        let assign20040_e14829: f64 = if locals.var_uc_corsrd == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard408 = assign20040_e14829;

        let (assign20050_e14835,) = {
    if ((locals.var_guard405 != 0.0) && (locals.var_guard408 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_flg_rsrd,)
    }
};
        locals.var_flg_rsrd = assign20050_e14835;

        locals.var_flg_pprv = 0.0;

        let assign20070_e14847: f64 = (locals.var_mks_nsubsub + locals.var_uc_nover);
        let assign20070_e14848: f64 = (locals.var_uc_nover * assign20070_e14847);
        let assign20070_e14851: f64 = if (((locals.var_uc_cordrift == 1.0) && (p.p54 == 1.0)) && (assign20070_e14848 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard409 = assign20070_e14851;

        let (assign20080_e14855, assign20080_e14855_d_n0, assign20080_e14855_d_n2,) = {
    if (locals.var_guard409 != 0.0) {
        (locals.var_vdsei, locals.var_vdsei_dn0, locals.var_vdsei_dn2,)
    } else {
        (locals.var_vdsegmt, locals.var_vdsegmt_dn0, locals.var_vdsegmt_dn2,)
    }
};
        locals.var_vdsegmt = assign20080_e14855;
        locals.var_vdsegmt_dn0 = assign20080_e14855_d_n0;
        locals.var_vdsegmt_dn2 = assign20080_e14855_d_n2;

        let assign20090_e14858: f64 = if locals.var_vdsegmt >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard410 = assign20090_e14858;

        let (assign20100_e14864, assign20100_e14864_d_n0, assign20100_e14864_d_n2,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard410 != 0.0)) {
        (locals.var_vdsegmt, locals.var_vdsegmt_dn0, locals.var_vdsegmt_dn2,)
    } else {
        (locals.var_vdserev, locals.var_vdserev_dn0, locals.var_vdserev_dn2,)
    }
};
        locals.var_vdserev = assign20100_e14864;
        locals.var_vdserev_dn0 = assign20100_e14864_d_n0;
        locals.var_vdserev_dn2 = assign20100_e14864_d_n2;

        let (assign20110_e14870, assign20110_e14870_d_n0, assign20110_e14870_d_n2, assign20110_e14870_d_n4,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard410 != 0.0)) {
        (locals.var_vsubs, 0.0, locals.var_vsubs_dn2, locals.var_vsubs_dn4,)
    } else {
        (locals.var_vsubsrev, locals.var_vsubsrev_dn0, locals.var_vsubsrev_dn2, locals.var_vsubsrev_dn4,)
    }
};
        locals.var_vsubsrev = assign20110_e14870;
        locals.var_vsubsrev_dn0 = assign20110_e14870_d_n0;
        locals.var_vsubsrev_dn2 = assign20110_e14870_d_n2;
        locals.var_vsubsrev_dn4 = assign20110_e14870_d_n4;

        let (assign20120_e14878, assign20120_e14878_d_n0, assign20120_e14878_d_n2,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard410 == 0.0)) {
        let assign20120_e14876: f64 = (-locals.var_vdsegmt);
        (assign20120_e14876, (-locals.var_vdsegmt_dn0), (-locals.var_vdsegmt_dn2),)
    } else {
        (locals.var_vdserev, locals.var_vdserev_dn0, locals.var_vdserev_dn2,)
    }
};
        locals.var_vdserev = assign20120_e14878;
        locals.var_vdserev_dn0 = assign20120_e14878_d_n0;
        locals.var_vdserev_dn2 = assign20120_e14878_d_n2;

        let (assign20130_e14887, assign20130_e14887_d_n0, assign20130_e14887_d_n2, assign20130_e14887_d_n4,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard410 == 0.0)) {
        let assign20130_e14885: f64 = (locals.var_vsubs - locals.var_vdsegmt);
        (assign20130_e14885, (-locals.var_vdsegmt_dn0), (locals.var_vsubs_dn2 - locals.var_vdsegmt_dn2), locals.var_vsubs_dn4,)
    } else {
        (locals.var_vsubsrev, locals.var_vsubsrev_dn0, locals.var_vsubsrev_dn2, locals.var_vsubsrev_dn4,)
    }
};
        locals.var_vsubsrev = assign20130_e14887;
        locals.var_vsubsrev_dn0 = assign20130_e14887_d_n0;
        locals.var_vsubsrev_dn2 = assign20130_e14887_d_n2;
        locals.var_vsubsrev_dn4 = assign20130_e14887_d_n4;

        let (assign20140_e14897, assign20140_e14897_d_n0, assign20140_e14897_d_n2, assign20140_e14897_d_n4, assign20140_e14897_d_n5, assign20140_e14897_d_n6, assign20140_e14897_d_n7, assign20140_e14897_d_n8, assign20140_e14897_d_n9, assign20140_e14897_d_n10, assign20140_e14897_d_n11, assign20140_e14897_d_n14,) = {
    if (locals.var_guard409 != 0.0) {
        let assign20140_e14892: f64 = (locals.var_vdserev / 2.0);
        let assign20140_e14893: f64 = (2.0 * assign20140_e14892);
        let assign20140_e14895: f64 = (assign20140_e14893 / p.p262);
        (assign20140_e14895, ((2.0 * (locals.var_vdserev_dn0 / 2.0)) / p.p262), ((2.0 * (locals.var_vdserev_dn2 / 2.0)) / p.p262), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign20140_e14897;
        locals.var_tmf1_dn0 = assign20140_e14897_d_n0;
        locals.var_tmf1_dn2 = assign20140_e14897_d_n2;
        locals.var_tmf1_dn4 = assign20140_e14897_d_n4;
        locals.var_tmf1_dn5 = assign20140_e14897_d_n5;
        locals.var_tmf1_dn6 = assign20140_e14897_d_n6;
        locals.var_tmf1_dn7 = assign20140_e14897_d_n7;
        locals.var_tmf1_dn8 = assign20140_e14897_d_n8;
        locals.var_tmf1_dn9 = assign20140_e14897_d_n9;
        locals.var_tmf1_dn10 = assign20140_e14897_d_n10;
        locals.var_tmf1_dn11 = assign20140_e14897_d_n11;
        locals.var_tmf1_dn14 = assign20140_e14897_d_n14;

        let (assign20150_e14937, assign20150_e14937_d_n0, assign20150_e14937_d_n2, assign20150_e14937_d_n4, assign20150_e14937_d_n5, assign20150_e14937_d_n6, assign20150_e14937_d_n7, assign20150_e14937_d_n8, assign20150_e14937_d_n9, assign20150_e14937_d_n10, assign20150_e14937_d_n11, assign20150_e14937_d_n14,) = {
    if (locals.var_guard409 != 0.0) {
        let assign20150_e14903: f64 = (1.0 / 2.0);
        let assign20150_e14907: f64 = (1.0 / 6.0);
        let assign20150_e14911: f64 = (1.0 / 24.0);
        let assign20150_e14915: f64 = (1.0 / 120.0);
        let assign20150_e14919: f64 = (1.0 / 720.0);
        let assign20150_e14923: f64 = (1.0 / 5040.0);
        let assign20150_e14924: f64 = (locals.var_tmf1 * assign20150_e14923);
        let assign20150_e14925: f64 = (assign20150_e14919 + assign20150_e14924);
        let assign20150_e14926: f64 = (locals.var_tmf1 * assign20150_e14925);
        let assign20150_e14927: f64 = (assign20150_e14915 + assign20150_e14926);
        let assign20150_e14928: f64 = (locals.var_tmf1 * assign20150_e14927);
        let assign20150_e14929: f64 = (assign20150_e14911 + assign20150_e14928);
        let assign20150_e14930: f64 = (locals.var_tmf1 * assign20150_e14929);
        let assign20150_e14931: f64 = (assign20150_e14907 + assign20150_e14930);
        let assign20150_e14932: f64 = (locals.var_tmf1 * assign20150_e14931);
        let assign20150_e14933: f64 = (assign20150_e14903 + assign20150_e14932);
        let assign20150_e14934: f64 = (locals.var_tmf1 * assign20150_e14933);
        let assign20150_e14935: f64 = (1.0 + assign20150_e14934);
        (assign20150_e14935, ((locals.var_tmf1_dn0 * assign20150_e14933) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20150_e14931) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20150_e14929) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20150_e14927) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20150_e14925) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign20150_e14923))))))))))), ((locals.var_tmf1_dn2 * assign20150_e14933) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20150_e14931) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20150_e14929) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20150_e14927) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20150_e14925) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign20150_e14923))))))))))), ((locals.var_tmf1_dn4 * assign20150_e14933) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20150_e14931) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20150_e14929) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20150_e14927) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20150_e14925) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign20150_e14923))))))))))), ((locals.var_tmf1_dn5 * assign20150_e14933) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20150_e14931) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20150_e14929) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20150_e14927) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20150_e14925) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign20150_e14923))))))))))), ((locals.var_tmf1_dn6 * assign20150_e14933) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20150_e14931) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20150_e14929) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20150_e14927) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20150_e14925) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign20150_e14923))))))))))), ((locals.var_tmf1_dn7 * assign20150_e14933) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20150_e14931) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20150_e14929) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20150_e14927) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20150_e14925) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign20150_e14923))))))))))), ((locals.var_tmf1_dn8 * assign20150_e14933) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20150_e14931) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20150_e14929) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20150_e14927) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20150_e14925) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign20150_e14923))))))))))), ((locals.var_tmf1_dn9 * assign20150_e14933) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20150_e14931) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20150_e14929) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20150_e14927) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20150_e14925) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign20150_e14923))))))))))), ((locals.var_tmf1_dn10 * assign20150_e14933) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20150_e14931) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20150_e14929) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20150_e14927) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20150_e14925) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign20150_e14923))))))))))), ((locals.var_tmf1_dn11 * assign20150_e14933) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign20150_e14931) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign20150_e14929) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign20150_e14927) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign20150_e14925) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign20150_e14923))))))))))), ((locals.var_tmf1_dn14 * assign20150_e14933) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign20150_e14931) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign20150_e14929) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign20150_e14927) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign20150_e14925) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign20150_e14923))))))))))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign20150_e14937;
        locals.var_tmf2_dn0 = assign20150_e14937_d_n0;
        locals.var_tmf2_dn2 = assign20150_e14937_d_n2;
        locals.var_tmf2_dn4 = assign20150_e14937_d_n4;
        locals.var_tmf2_dn5 = assign20150_e14937_d_n5;
        locals.var_tmf2_dn6 = assign20150_e14937_d_n6;
        locals.var_tmf2_dn7 = assign20150_e14937_d_n7;
        locals.var_tmf2_dn8 = assign20150_e14937_d_n8;
        locals.var_tmf2_dn9 = assign20150_e14937_d_n9;
        locals.var_tmf2_dn10 = assign20150_e14937_d_n10;
        locals.var_tmf2_dn11 = assign20150_e14937_d_n11;
        locals.var_tmf2_dn14 = assign20150_e14937_d_n14;

        let (assign20160_e14973, assign20160_e14973_d_n0, assign20160_e14973_d_n2, assign20160_e14973_d_n4, assign20160_e14973_d_n5, assign20160_e14973_d_n6, assign20160_e14973_d_n7, assign20160_e14973_d_n8, assign20160_e14973_d_n9, assign20160_e14973_d_n10, assign20160_e14973_d_n11, assign20160_e14973_d_n14,) = {
    if (locals.var_guard409 != 0.0) {
        let assign20160_e14941: f64 = (1.0 / 2.0);
        let assign20160_e14945: f64 = (1.0 / 3.0);
        let assign20160_e14949: f64 = (1.0 / 8.0);
        let assign20160_e14953: f64 = (1.0 / 30.0);
        let assign20160_e14957: f64 = (1.0 / 144.0);
        let assign20160_e14961: f64 = (1.0 / 840.0);
        let assign20160_e14962: f64 = (locals.var_tmf1 * assign20160_e14961);
        let assign20160_e14963: f64 = (assign20160_e14957 + assign20160_e14962);
        let assign20160_e14964: f64 = (locals.var_tmf1 * assign20160_e14963);
        let assign20160_e14965: f64 = (assign20160_e14953 + assign20160_e14964);
        let assign20160_e14966: f64 = (locals.var_tmf1 * assign20160_e14965);
        let assign20160_e14967: f64 = (assign20160_e14949 + assign20160_e14966);
        let assign20160_e14968: f64 = (locals.var_tmf1 * assign20160_e14967);
        let assign20160_e14969: f64 = (assign20160_e14945 + assign20160_e14968);
        let assign20160_e14970: f64 = (locals.var_tmf1 * assign20160_e14969);
        let assign20160_e14971: f64 = (assign20160_e14941 + assign20160_e14970);
        (assign20160_e14971, ((locals.var_tmf1_dn0 * assign20160_e14969) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20160_e14967) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20160_e14965) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20160_e14963) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign20160_e14961))))))))), ((locals.var_tmf1_dn2 * assign20160_e14969) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20160_e14967) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20160_e14965) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20160_e14963) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign20160_e14961))))))))), ((locals.var_tmf1_dn4 * assign20160_e14969) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20160_e14967) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20160_e14965) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20160_e14963) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign20160_e14961))))))))), ((locals.var_tmf1_dn5 * assign20160_e14969) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20160_e14967) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20160_e14965) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20160_e14963) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign20160_e14961))))))))), ((locals.var_tmf1_dn6 * assign20160_e14969) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20160_e14967) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20160_e14965) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20160_e14963) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign20160_e14961))))))))), ((locals.var_tmf1_dn7 * assign20160_e14969) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20160_e14967) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20160_e14965) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20160_e14963) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign20160_e14961))))))))), ((locals.var_tmf1_dn8 * assign20160_e14969) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20160_e14967) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20160_e14965) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20160_e14963) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign20160_e14961))))))))), ((locals.var_tmf1_dn9 * assign20160_e14969) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20160_e14967) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20160_e14965) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20160_e14963) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign20160_e14961))))))))), ((locals.var_tmf1_dn10 * assign20160_e14969) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20160_e14967) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20160_e14965) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20160_e14963) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign20160_e14961))))))))), ((locals.var_tmf1_dn11 * assign20160_e14969) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign20160_e14967) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign20160_e14965) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign20160_e14963) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign20160_e14961))))))))), ((locals.var_tmf1_dn14 * assign20160_e14969) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign20160_e14967) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign20160_e14965) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign20160_e14963) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign20160_e14961))))))))),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn14,)
    }
};
        locals.var_tmf3 = assign20160_e14973;
        locals.var_tmf3_dn0 = assign20160_e14973_d_n0;
        locals.var_tmf3_dn2 = assign20160_e14973_d_n2;
        locals.var_tmf3_dn4 = assign20160_e14973_d_n4;
        locals.var_tmf3_dn5 = assign20160_e14973_d_n5;
        locals.var_tmf3_dn6 = assign20160_e14973_d_n6;
        locals.var_tmf3_dn7 = assign20160_e14973_d_n7;
        locals.var_tmf3_dn8 = assign20160_e14973_d_n8;
        locals.var_tmf3_dn9 = assign20160_e14973_d_n9;
        locals.var_tmf3_dn10 = assign20160_e14973_d_n10;
        locals.var_tmf3_dn11 = assign20160_e14973_d_n11;
        locals.var_tmf3_dn14 = assign20160_e14973_d_n14;

        let (assign20170_e14979, assign20170_e14979_d_n0, assign20170_e14979_d_n2, assign20170_e14979_d_n4, assign20170_e14979_d_n5, assign20170_e14979_d_n6, assign20170_e14979_d_n7, assign20170_e14979_d_n8, assign20170_e14979_d_n9, assign20170_e14979_d_n10, assign20170_e14979_d_n11, assign20170_e14979_d_n14,) = {
    if (locals.var_guard409 != 0.0) {
        let assign20170_e14977: f64 = (p.p262 / locals.var_tmf2);
        (assign20170_e14977, (-((p.p262 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_vzadd, locals.var_vzadd_dn0, locals.var_vzadd_dn2, locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, locals.var_vzadd_dn7, locals.var_vzadd_dn8, locals.var_vzadd_dn9, locals.var_vzadd_dn10, locals.var_vzadd_dn11, locals.var_vzadd_dn14,)
    }
};
        locals.var_vzadd = assign20170_e14979;
        locals.var_vzadd_dn0 = assign20170_e14979_d_n0;
        locals.var_vzadd_dn2 = assign20170_e14979_d_n2;
        locals.var_vzadd_dn4 = assign20170_e14979_d_n4;
        locals.var_vzadd_dn5 = assign20170_e14979_d_n5;
        locals.var_vzadd_dn6 = assign20170_e14979_d_n6;
        locals.var_vzadd_dn7 = assign20170_e14979_d_n7;
        locals.var_vzadd_dn8 = assign20170_e14979_d_n8;
        locals.var_vzadd_dn9 = assign20170_e14979_d_n9;
        locals.var_vzadd_dn10 = assign20170_e14979_d_n10;
        locals.var_vzadd_dn11 = assign20170_e14979_d_n11;
        locals.var_vzadd_dn14 = assign20170_e14979_d_n14;

        let (assign20180_e14990, assign20180_e14990_d_n0, assign20180_e14990_d_n2, assign20180_e14990_d_n4, assign20180_e14990_d_n5, assign20180_e14990_d_n6, assign20180_e14990_d_n7, assign20180_e14990_d_n8, assign20180_e14990_d_n9, assign20180_e14990_d_n10, assign20180_e14990_d_n11, assign20180_e14990_d_n14,) = {
    if (locals.var_guard409 != 0.0) {
        let assign20180_e14982: f64 = (-2.0);
        let assign20180_e14984: f64 = (assign20180_e14982 * locals.var_tmf3);
        let assign20180_e14987: f64 = (locals.var_tmf2 * locals.var_tmf2);
        let assign20180_e14988: f64 = (assign20180_e14984 / assign20180_e14987);
        (assign20180_e14988, ((((assign20180_e14982 * locals.var_tmf3_dn0) * assign20180_e14987) - (assign20180_e14984 * ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)))) / (assign20180_e14987 * assign20180_e14987)), ((((assign20180_e14982 * locals.var_tmf3_dn2) * assign20180_e14987) - (assign20180_e14984 * ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)))) / (assign20180_e14987 * assign20180_e14987)), ((((assign20180_e14982 * locals.var_tmf3_dn4) * assign20180_e14987) - (assign20180_e14984 * ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)))) / (assign20180_e14987 * assign20180_e14987)), ((((assign20180_e14982 * locals.var_tmf3_dn5) * assign20180_e14987) - (assign20180_e14984 * ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)))) / (assign20180_e14987 * assign20180_e14987)), ((((assign20180_e14982 * locals.var_tmf3_dn6) * assign20180_e14987) - (assign20180_e14984 * ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)))) / (assign20180_e14987 * assign20180_e14987)), ((((assign20180_e14982 * locals.var_tmf3_dn7) * assign20180_e14987) - (assign20180_e14984 * ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)))) / (assign20180_e14987 * assign20180_e14987)), ((((assign20180_e14982 * locals.var_tmf3_dn8) * assign20180_e14987) - (assign20180_e14984 * ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)))) / (assign20180_e14987 * assign20180_e14987)), ((((assign20180_e14982 * locals.var_tmf3_dn9) * assign20180_e14987) - (assign20180_e14984 * ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)))) / (assign20180_e14987 * assign20180_e14987)), ((((assign20180_e14982 * locals.var_tmf3_dn10) * assign20180_e14987) - (assign20180_e14984 * ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)))) / (assign20180_e14987 * assign20180_e14987)), ((((assign20180_e14982 * locals.var_tmf3_dn11) * assign20180_e14987) - (assign20180_e14984 * ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)))) / (assign20180_e14987 * assign20180_e14987)), ((((assign20180_e14982 * locals.var_tmf3_dn14) * assign20180_e14987) - (assign20180_e14984 * ((locals.var_tmf2_dn14 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn14)))) / (assign20180_e14987 * assign20180_e14987)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign20180_e14990;
        locals.var_t2_dn0 = assign20180_e14990_d_n0;
        locals.var_t2_dn2 = assign20180_e14990_d_n2;
        locals.var_t2_dn4 = assign20180_e14990_d_n4;
        locals.var_t2_dn5 = assign20180_e14990_d_n5;
        locals.var_t2_dn6 = assign20180_e14990_d_n6;
        locals.var_t2_dn7 = assign20180_e14990_d_n7;
        locals.var_t2_dn8 = assign20180_e14990_d_n8;
        locals.var_t2_dn9 = assign20180_e14990_d_n9;
        locals.var_t2_dn10 = assign20180_e14990_d_n10;
        locals.var_t2_dn11 = assign20180_e14990_d_n11;
        locals.var_t2_dn14 = assign20180_e14990_d_n14;

        let assign20190_e14993: f64 = if locals.var_vzadd < 1e-12 { 1.0 } else { 0.0 };
        locals.var_guard411 = assign20190_e14993;

        let (assign20200_e14999, assign20200_e14999_d_n0, assign20200_e14999_d_n2, assign20200_e14999_d_n4, assign20200_e14999_d_n5, assign20200_e14999_d_n6, assign20200_e14999_d_n7, assign20200_e14999_d_n8, assign20200_e14999_d_n9, assign20200_e14999_d_n10, assign20200_e14999_d_n11, assign20200_e14999_d_n14,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) {
        (1e-12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vzadd, locals.var_vzadd_dn0, locals.var_vzadd_dn2, locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, locals.var_vzadd_dn7, locals.var_vzadd_dn8, locals.var_vzadd_dn9, locals.var_vzadd_dn10, locals.var_vzadd_dn11, locals.var_vzadd_dn14,)
    }
};
        locals.var_vzadd = assign20200_e14999;
        locals.var_vzadd_dn0 = assign20200_e14999_d_n0;
        locals.var_vzadd_dn2 = assign20200_e14999_d_n2;
        locals.var_vzadd_dn4 = assign20200_e14999_d_n4;
        locals.var_vzadd_dn5 = assign20200_e14999_d_n5;
        locals.var_vzadd_dn6 = assign20200_e14999_d_n6;
        locals.var_vzadd_dn7 = assign20200_e14999_d_n7;
        locals.var_vzadd_dn8 = assign20200_e14999_d_n8;
        locals.var_vzadd_dn9 = assign20200_e14999_d_n9;
        locals.var_vzadd_dn10 = assign20200_e14999_d_n10;
        locals.var_vzadd_dn11 = assign20200_e14999_d_n11;
        locals.var_vzadd_dn14 = assign20200_e14999_d_n14;

        let (assign20210_e15007, assign20210_e15007_d_n0, assign20210_e15007_d_n2, assign20210_e15007_d_n4, assign20210_e15007_d_n5, assign20210_e15007_d_n6, assign20210_e15007_d_n7, assign20210_e15007_d_n8, assign20210_e15007_d_n9, assign20210_e15007_d_n10, assign20210_e15007_d_n11, assign20210_e15007_d_n14,) = {
    if (locals.var_guard409 != 0.0) {
        let assign20210_e15004: f64 = (2.0 * locals.var_vzadd);
        let assign20210_e15005: f64 = (locals.var_vdserev + assign20210_e15004);
        (assign20210_e15005, (locals.var_vdserev_dn0 + (2.0 * locals.var_vzadd_dn0)), (locals.var_vdserev_dn2 + (2.0 * locals.var_vzadd_dn2)), (2.0 * locals.var_vzadd_dn4), (2.0 * locals.var_vzadd_dn5), (2.0 * locals.var_vzadd_dn6), (2.0 * locals.var_vzadd_dn7), (2.0 * locals.var_vzadd_dn8), (2.0 * locals.var_vzadd_dn9), (2.0 * locals.var_vzadd_dn10), (2.0 * locals.var_vzadd_dn11), (2.0 * locals.var_vzadd_dn14),)
    } else {
        (locals.var_vdserevz, locals.var_vdserevz_dn0, locals.var_vdserevz_dn2, locals.var_vdserevz_dn4, locals.var_vdserevz_dn5, locals.var_vdserevz_dn6, locals.var_vdserevz_dn7, locals.var_vdserevz_dn8, locals.var_vdserevz_dn9, locals.var_vdserevz_dn10, locals.var_vdserevz_dn11, locals.var_vdserevz_dn14,)
    }
};
        locals.var_vdserevz = assign20210_e15007;
        locals.var_vdserevz_dn0 = assign20210_e15007_d_n0;
        locals.var_vdserevz_dn2 = assign20210_e15007_d_n2;
        locals.var_vdserevz_dn4 = assign20210_e15007_d_n4;
        locals.var_vdserevz_dn5 = assign20210_e15007_d_n5;
        locals.var_vdserevz_dn6 = assign20210_e15007_d_n6;
        locals.var_vdserevz_dn7 = assign20210_e15007_d_n7;
        locals.var_vdserevz_dn8 = assign20210_e15007_d_n8;
        locals.var_vdserevz_dn9 = assign20210_e15007_d_n9;
        locals.var_vdserevz_dn10 = assign20210_e15007_d_n10;
        locals.var_vdserevz_dn11 = assign20210_e15007_d_n11;
        locals.var_vdserevz_dn14 = assign20210_e15007_d_n14;

        let (assign20220_e15019, assign20220_e15019_d_n0, assign20220_e15019_d_n2, assign20220_e15019_d_n4, assign20220_e15019_d_n5, assign20220_e15019_d_n6, assign20220_e15019_d_n7, assign20220_e15019_d_n8, assign20220_e15019_d_n9, assign20220_e15019_d_n10, assign20220_e15019_d_n11, assign20220_e15019_d_n14,) = {
    if (locals.var_guard409 != 0.0) {
        let assign20220_e15012: f64 = (p.p333 * locals.var_vdserevz);
        let assign20220_e15013: f64 = (p.p335 - assign20220_e15012);
        let assign20220_e15016: f64 = (p.p332 * locals.var_vsubsrev);
        let assign20220_e15017: f64 = (assign20220_e15013 - assign20220_e15016);
        (assign20220_e15017, ((-(p.p333 * locals.var_vdserevz_dn0)) - (p.p332 * locals.var_vsubsrev_dn0)), ((-(p.p333 * locals.var_vdserevz_dn2)) - (p.p332 * locals.var_vsubsrev_dn2)), ((-(p.p333 * locals.var_vdserevz_dn4)) - (p.p332 * locals.var_vsubsrev_dn4)), (-(p.p333 * locals.var_vdserevz_dn5)), (-(p.p333 * locals.var_vdserevz_dn6)), (-(p.p333 * locals.var_vdserevz_dn7)), (-(p.p333 * locals.var_vdserevz_dn8)), (-(p.p333 * locals.var_vdserevz_dn9)), (-(p.p333 * locals.var_vdserevz_dn10)), (-(p.p333 * locals.var_vdserevz_dn11)), (-(p.p333 * locals.var_vdserevz_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign20220_e15019;
        locals.var_t0_dn0 = assign20220_e15019_d_n0;
        locals.var_t0_dn2 = assign20220_e15019_d_n2;
        locals.var_t0_dn4 = assign20220_e15019_d_n4;
        locals.var_t0_dn5 = assign20220_e15019_d_n5;
        locals.var_t0_dn6 = assign20220_e15019_d_n6;
        locals.var_t0_dn7 = assign20220_e15019_d_n7;
        locals.var_t0_dn8 = assign20220_e15019_d_n8;
        locals.var_t0_dn9 = assign20220_e15019_d_n9;
        locals.var_t0_dn10 = assign20220_e15019_d_n10;
        locals.var_t0_dn11 = assign20220_e15019_d_n11;
        locals.var_t0_dn14 = assign20220_e15019_d_n14;

    }
}
