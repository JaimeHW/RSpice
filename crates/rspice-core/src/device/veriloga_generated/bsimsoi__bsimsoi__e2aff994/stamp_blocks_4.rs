#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_64(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign24540_e37850, assign24540_e37850_d_n3, assign24540_e37850_d_n4, assign24540_e37850_d_n5, assign24540_e37850_d_n6, assign24540_e37850_d_n7, assign24540_e37850_d_n8, assign24540_e37850_d_n9, assign24540_e37850_d_n10, assign24540_e37850_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard537 == 0.0)) {
        let assign24540_e37847: f64 = (locals.var_prwg_i * locals.var_qia);
        let assign24540_e37848: f64 = (1.0 + assign24540_e37847);
        (assign24540_e37848, (locals.var_prwg_i * locals.var_qia_dn3), (locals.var_prwg_i * locals.var_qia_dn4), (locals.var_prwg_i * locals.var_qia_dn5), (locals.var_prwg_i * locals.var_qia_dn6), (locals.var_prwg_i * locals.var_qia_dn7), (locals.var_prwg_i * locals.var_qia_dn8), (locals.var_prwg_i * locals.var_qia_dn9), (locals.var_prwg_i * locals.var_qia_dn10), (locals.var_prwg_i * locals.var_qia_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign24540_e37850;
        locals.var_t0_dn3 = assign24540_e37850_d_n3;
        locals.var_t0_dn4 = assign24540_e37850_d_n4;
        locals.var_t0_dn5 = assign24540_e37850_d_n5;
        locals.var_t0_dn6 = assign24540_e37850_d_n6;
        locals.var_t0_dn7 = assign24540_e37850_d_n7;
        locals.var_t0_dn8 = assign24540_e37850_d_n8;
        locals.var_t0_dn9 = assign24540_e37850_d_n9;
        locals.var_t0_dn10 = assign24540_e37850_d_n10;
        locals.var_t0_dn11 = assign24540_e37850_d_n11;

        let (assign24550_e37864, assign24550_e37864_d_n3, assign24550_e37864_d_n4, assign24550_e37864_d_n5, assign24550_e37864_d_n6, assign24550_e37864_d_n7, assign24550_e37864_d_n8, assign24550_e37864_d_n9, assign24550_e37864_d_n10, assign24550_e37864_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard537 == 0.0)) {
        let assign24550_e37861: f64 = (locals.var_sqrtphistvbs - locals.var_sqrtphist);
        let assign24550_e37862: f64 = (locals.var_prwb_i * assign24550_e37861);
        (assign24550_e37862, (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn3 - locals.var_sqrtphist_dn3)), (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn4 - locals.var_sqrtphist_dn4)), (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn5 - locals.var_sqrtphist_dn5)), (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn6 - locals.var_sqrtphist_dn6)), (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn7 - locals.var_sqrtphist_dn7)), (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn8 - locals.var_sqrtphist_dn8)), (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn9 - locals.var_sqrtphist_dn9)), (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn10 - locals.var_sqrtphist_dn10)), (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn11 - locals.var_sqrtphist_dn11)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign24550_e37864;
        locals.var_t1_dn3 = assign24550_e37864_d_n3;
        locals.var_t1_dn4 = assign24550_e37864_d_n4;
        locals.var_t1_dn5 = assign24550_e37864_d_n5;
        locals.var_t1_dn6 = assign24550_e37864_d_n6;
        locals.var_t1_dn7 = assign24550_e37864_d_n7;
        locals.var_t1_dn8 = assign24550_e37864_d_n8;
        locals.var_t1_dn9 = assign24550_e37864_d_n9;
        locals.var_t1_dn10 = assign24550_e37864_d_n10;
        locals.var_t1_dn11 = assign24550_e37864_d_n11;

        let (assign24560_e37878, assign24560_e37878_d_n3, assign24560_e37878_d_n4, assign24560_e37878_d_n5, assign24560_e37878_d_n6, assign24560_e37878_d_n7, assign24560_e37878_d_n8, assign24560_e37878_d_n9, assign24560_e37878_d_n10, assign24560_e37878_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard537 == 0.0)) {
        let assign24560_e37874: f64 = (1.0 / locals.var_t0);
        let assign24560_e37876: f64 = (assign24560_e37874 + locals.var_t1);
        (assign24560_e37876, ((-(locals.var_t0_dn3 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn3), ((-(locals.var_t0_dn4 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn4), ((-(locals.var_t0_dn5 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn5), ((-(locals.var_t0_dn6 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn6), ((-(locals.var_t0_dn7 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn7), ((-(locals.var_t0_dn8 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn8), ((-(locals.var_t0_dn9 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn9), ((-(locals.var_t0_dn10 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn10), ((-(locals.var_t0_dn11 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn11),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign24560_e37878;
        locals.var_t2_dn3 = assign24560_e37878_d_n3;
        locals.var_t2_dn4 = assign24560_e37878_d_n4;
        locals.var_t2_dn5 = assign24560_e37878_d_n5;
        locals.var_t2_dn6 = assign24560_e37878_d_n6;
        locals.var_t2_dn7 = assign24560_e37878_d_n7;
        locals.var_t2_dn8 = assign24560_e37878_d_n8;
        locals.var_t2_dn9 = assign24560_e37878_d_n9;
        locals.var_t2_dn10 = assign24560_e37878_d_n10;
        locals.var_t2_dn11 = assign24560_e37878_d_n11;

        let (assign24570_e37897, assign24570_e37897_d_n3, assign24570_e37897_d_n4, assign24570_e37897_d_n5, assign24570_e37897_d_n6, assign24570_e37897_d_n7, assign24570_e37897_d_n8, assign24570_e37897_d_n9, assign24570_e37897_d_n10, assign24570_e37897_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard537 == 0.0)) {
        let assign24570_e37890: f64 = (locals.var_t2 * locals.var_t2);
        let assign24570_e37892: f64 = (assign24570_e37890 + 0.01);
        let assign24570_e37893: f64 = (assign24570_e37892).sqrt();
        let assign24570_e37894: f64 = (locals.var_t2 + assign24570_e37893);
        let assign24570_e37895: f64 = (0.5 * assign24570_e37894);
        (assign24570_e37895, (0.5 * (locals.var_t2_dn3 + (((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign24570_e37893)))), (0.5 * (locals.var_t2_dn4 + (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign24570_e37893)))), (0.5 * (locals.var_t2_dn5 + (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign24570_e37893)))), (0.5 * (locals.var_t2_dn6 + (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign24570_e37893)))), (0.5 * (locals.var_t2_dn7 + (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign24570_e37893)))), (0.5 * (locals.var_t2_dn8 + (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign24570_e37893)))), (0.5 * (locals.var_t2_dn9 + (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign24570_e37893)))), (0.5 * (locals.var_t2_dn10 + (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign24570_e37893)))), (0.5 * (locals.var_t2_dn11 + (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign24570_e37893)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign24570_e37897;
        locals.var_t3_dn3 = assign24570_e37897_d_n3;
        locals.var_t3_dn4 = assign24570_e37897_d_n4;
        locals.var_t3_dn5 = assign24570_e37897_d_n5;
        locals.var_t3_dn6 = assign24570_e37897_d_n6;
        locals.var_t3_dn7 = assign24570_e37897_d_n7;
        locals.var_t3_dn8 = assign24570_e37897_d_n8;
        locals.var_t3_dn9 = assign24570_e37897_d_n9;
        locals.var_t3_dn10 = assign24570_e37897_d_n10;
        locals.var_t3_dn11 = assign24570_e37897_d_n11;

        let (assign24580_e37917, assign24580_e37917_d_n3, assign24580_e37917_d_n4, assign24580_e37917_d_n5, assign24580_e37917_d_n6, assign24580_e37917_d_n7, assign24580_e37917_d_n8, assign24580_e37917_d_n9, assign24580_e37917_d_n10, assign24580_e37917_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard537 == 0.0)) {
        let assign24580_e37909: f64 = (locals.var_rdsw_i * locals.var_t3);
        let assign24580_e37910: f64 = (locals.var_rdswmin_i + assign24580_e37909);
        let assign24580_e37911: f64 = (locals.var_rdstemp * assign24580_e37910);
        let assign24580_e37913: f64 = (assign24580_e37911 * locals.var_weffwrfactor);
        let assign24580_e37915: f64 = (assign24580_e37913 * p.p2);
        (assign24580_e37915, (((locals.var_rdstemp * (locals.var_rdsw_i * locals.var_t3_dn3)) * locals.var_weffwrfactor) * p.p2), ((((locals.var_rdstemp_dn4 * assign24580_e37910) + (locals.var_rdstemp * (locals.var_rdsw_i * locals.var_t3_dn4))) * locals.var_weffwrfactor) * p.p2), ((((locals.var_rdstemp_dn5 * assign24580_e37910) + (locals.var_rdstemp * (locals.var_rdsw_i * locals.var_t3_dn5))) * locals.var_weffwrfactor) * p.p2), (((locals.var_rdstemp * (locals.var_rdsw_i * locals.var_t3_dn6)) * locals.var_weffwrfactor) * p.p2), (((locals.var_rdstemp * (locals.var_rdsw_i * locals.var_t3_dn7)) * locals.var_weffwrfactor) * p.p2), (((locals.var_rdstemp * (locals.var_rdsw_i * locals.var_t3_dn8)) * locals.var_weffwrfactor) * p.p2), (((locals.var_rdstemp * (locals.var_rdsw_i * locals.var_t3_dn9)) * locals.var_weffwrfactor) * p.p2), (((locals.var_rdstemp * (locals.var_rdsw_i * locals.var_t3_dn10)) * locals.var_weffwrfactor) * p.p2), (((locals.var_rdstemp * (locals.var_rdsw_i * locals.var_t3_dn11)) * locals.var_weffwrfactor) * p.p2),)
    } else {
        (locals.var_rdsi, locals.var_rdsi_dn3, locals.var_rdsi_dn4, locals.var_rdsi_dn5, locals.var_rdsi_dn6, locals.var_rdsi_dn7, locals.var_rdsi_dn8, locals.var_rdsi_dn9, locals.var_rdsi_dn10, locals.var_rdsi_dn11,)
    }
};
        locals.var_rdsi = assign24580_e37917;
        locals.var_rdsi_dn3 = assign24580_e37917_d_n3;
        locals.var_rdsi_dn4 = assign24580_e37917_d_n4;
        locals.var_rdsi_dn5 = assign24580_e37917_d_n5;
        locals.var_rdsi_dn6 = assign24580_e37917_d_n6;
        locals.var_rdsi_dn7 = assign24580_e37917_d_n7;
        locals.var_rdsi_dn8 = assign24580_e37917_d_n8;
        locals.var_rdsi_dn9 = assign24580_e37917_d_n9;
        locals.var_rdsi_dn10 = assign24580_e37917_d_n10;
        locals.var_rdsi_dn11 = assign24580_e37917_d_n11;

        let (assign24590_e37927, assign24590_e37927_d_n3, assign24590_e37927_d_n4, assign24590_e37927_d_n5, assign24590_e37927_d_n6, assign24590_e37927_d_n7, assign24590_e37927_d_n8, assign24590_e37927_d_n9, assign24590_e37927_d_n10, assign24590_e37927_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard537 == 0.0)) {
        (locals.var_rdraingeo, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdrain, locals.var_rdrain_dn3, locals.var_rdrain_dn4, locals.var_rdrain_dn5, locals.var_rdrain_dn6, locals.var_rdrain_dn7, locals.var_rdrain_dn8, locals.var_rdrain_dn9, locals.var_rdrain_dn10, locals.var_rdrain_dn11,)
    }
};
        locals.var_rdrain = assign24590_e37927;
        locals.var_rdrain_dn3 = assign24590_e37927_d_n3;
        locals.var_rdrain_dn4 = assign24590_e37927_d_n4;
        locals.var_rdrain_dn5 = assign24590_e37927_d_n5;
        locals.var_rdrain_dn6 = assign24590_e37927_d_n6;
        locals.var_rdrain_dn7 = assign24590_e37927_d_n7;
        locals.var_rdrain_dn8 = assign24590_e37927_d_n8;
        locals.var_rdrain_dn9 = assign24590_e37927_d_n9;
        locals.var_rdrain_dn10 = assign24590_e37927_d_n10;
        locals.var_rdrain_dn11 = assign24590_e37927_d_n11;

        let (assign24600_e37937, assign24600_e37937_d_n3, assign24600_e37937_d_n4, assign24600_e37937_d_n5, assign24600_e37937_d_n6, assign24600_e37937_d_n7, assign24600_e37937_d_n8, assign24600_e37937_d_n9, assign24600_e37937_d_n10, assign24600_e37937_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard537 == 0.0)) {
        (locals.var_rsourcegeo, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rsource, locals.var_rsource_dn3, locals.var_rsource_dn4, locals.var_rsource_dn5, locals.var_rsource_dn6, locals.var_rsource_dn7, locals.var_rsource_dn8, locals.var_rsource_dn9, locals.var_rsource_dn10, locals.var_rsource_dn11,)
    }
};
        locals.var_rsource = assign24600_e37937;
        locals.var_rsource_dn3 = assign24600_e37937_d_n3;
        locals.var_rsource_dn4 = assign24600_e37937_d_n4;
        locals.var_rsource_dn5 = assign24600_e37937_d_n5;
        locals.var_rsource_dn6 = assign24600_e37937_d_n6;
        locals.var_rsource_dn7 = assign24600_e37937_d_n7;
        locals.var_rsource_dn8 = assign24600_e37937_d_n8;
        locals.var_rsource_dn9 = assign24600_e37937_d_n9;
        locals.var_rsource_dn10 = assign24600_e37937_d_n10;
        locals.var_rsource_dn11 = assign24600_e37937_d_n11;

        let (assign24610_e37961, assign24610_e37961_d_n3, assign24610_e37961_d_n4, assign24610_e37961_d_n5, assign24610_e37961_d_n6, assign24610_e37961_d_n7, assign24610_e37961_d_n8, assign24610_e37961_d_n9, assign24610_e37961_d_n10, assign24610_e37961_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard537 == 0.0)) {
        let assign24610_e37948: f64 = (locals.var_u0_a / locals.var_dvsat);
        let assign24610_e37950: f64 = (assign24610_e37948 * locals.var_cox);
        let assign24610_e37952: f64 = (assign24610_e37950 * locals.var_weff);
        let assign24610_e37954: f64 = (assign24610_e37952 / locals.var_leff);
        let assign24610_e37956: f64 = (assign24610_e37954 * locals.var_qia);
        let assign24610_e37958: f64 = (assign24610_e37956 * locals.var_rdsi);
        let assign24610_e37959: f64 = (1.0 + assign24610_e37958);
        (assign24610_e37959, ((((((((((locals.var_u0_a_dn3 * locals.var_dvsat) - (locals.var_u0_a * locals.var_dvsat_dn3)) / (locals.var_dvsat * locals.var_dvsat)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign24610_e37954 * locals.var_qia_dn3)) * locals.var_rdsi) + (assign24610_e37956 * locals.var_rdsi_dn3)), ((((((((((locals.var_u0_a_dn4 * locals.var_dvsat) - (locals.var_u0_a * locals.var_dvsat_dn4)) / (locals.var_dvsat * locals.var_dvsat)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign24610_e37954 * locals.var_qia_dn4)) * locals.var_rdsi) + (assign24610_e37956 * locals.var_rdsi_dn4)), ((((((((((locals.var_u0_a_dn5 * locals.var_dvsat) - (locals.var_u0_a * locals.var_dvsat_dn5)) / (locals.var_dvsat * locals.var_dvsat)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign24610_e37954 * locals.var_qia_dn5)) * locals.var_rdsi) + (assign24610_e37956 * locals.var_rdsi_dn5)), ((((((((((locals.var_u0_a_dn6 * locals.var_dvsat) - (locals.var_u0_a * locals.var_dvsat_dn6)) / (locals.var_dvsat * locals.var_dvsat)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign24610_e37954 * locals.var_qia_dn6)) * locals.var_rdsi) + (assign24610_e37956 * locals.var_rdsi_dn6)), ((((((((((locals.var_u0_a_dn7 * locals.var_dvsat) - (locals.var_u0_a * locals.var_dvsat_dn7)) / (locals.var_dvsat * locals.var_dvsat)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign24610_e37954 * locals.var_qia_dn7)) * locals.var_rdsi) + (assign24610_e37956 * locals.var_rdsi_dn7)), ((((((((((locals.var_u0_a_dn8 * locals.var_dvsat) - (locals.var_u0_a * locals.var_dvsat_dn8)) / (locals.var_dvsat * locals.var_dvsat)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign24610_e37954 * locals.var_qia_dn8)) * locals.var_rdsi) + (assign24610_e37956 * locals.var_rdsi_dn8)), ((((((((((locals.var_u0_a_dn9 * locals.var_dvsat) - (locals.var_u0_a * locals.var_dvsat_dn9)) / (locals.var_dvsat * locals.var_dvsat)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign24610_e37954 * locals.var_qia_dn9)) * locals.var_rdsi) + (assign24610_e37956 * locals.var_rdsi_dn9)), ((((((((((locals.var_u0_a_dn10 * locals.var_dvsat) - (locals.var_u0_a * locals.var_dvsat_dn10)) / (locals.var_dvsat * locals.var_dvsat)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign24610_e37954 * locals.var_qia_dn10)) * locals.var_rdsi) + (assign24610_e37956 * locals.var_rdsi_dn10)), ((((((((((locals.var_u0_a_dn11 * locals.var_dvsat) - (locals.var_u0_a * locals.var_dvsat_dn11)) / (locals.var_dvsat * locals.var_dvsat)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign24610_e37954 * locals.var_qia_dn11)) * locals.var_rdsi) + (assign24610_e37956 * locals.var_rdsi_dn11)),)
    } else {
        (locals.var_dr, locals.var_dr_dn3, locals.var_dr_dn4, locals.var_dr_dn5, locals.var_dr_dn6, locals.var_dr_dn7, locals.var_dr_dn8, locals.var_dr_dn9, locals.var_dr_dn10, locals.var_dr_dn11,)
    }
};
        locals.var_dr = assign24610_e37961;
        locals.var_dr_dn3 = assign24610_e37961_d_n3;
        locals.var_dr_dn4 = assign24610_e37961_d_n4;
        locals.var_dr_dn5 = assign24610_e37961_d_n5;
        locals.var_dr_dn6 = assign24610_e37961_d_n6;
        locals.var_dr_dn7 = assign24610_e37961_d_n7;
        locals.var_dr_dn8 = assign24610_e37961_d_n8;
        locals.var_dr_dn9 = assign24610_e37961_d_n9;
        locals.var_dr_dn10 = assign24610_e37961_d_n10;
        locals.var_dr_dn11 = assign24610_e37961_d_n11;

        let assign24620_e37964: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard538 = assign24620_e37964;

        let (assign24630_e37990, assign24630_e37990_d_n3, assign24630_e37990_d_n4, assign24630_e37990_d_n5, assign24630_e37990_d_n6, assign24630_e37990_d_n7, assign24630_e37990_d_n8, assign24630_e37990_d_n9, assign24630_e37990_d_n10, assign24630_e37990_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        let assign24630_e37979: f64 = (locals.var_rdsw_i * locals.var_t3);
        let assign24630_e37980: f64 = (locals.var_rdswmin_i + assign24630_e37979);
        let assign24630_e37982: f64 = (assign24630_e37980 * locals.var_weffwrfactor);
        let assign24630_e37984: f64 = (assign24630_e37982 * p.p2);
        let assign24630_e37985: f64 = (locals.var_rsourcegeo + assign24630_e37984);
        let assign24630_e37987: f64 = (assign24630_e37985 + locals.var_rdraingeo);
        let assign24630_e37988: f64 = (locals.var_rdstemp * assign24630_e37987);
        (assign24630_e37988, (locals.var_rdstemp * (((locals.var_rdsw_i * locals.var_t3_dn3) * locals.var_weffwrfactor) * p.p2)), ((locals.var_rdstemp_dn4 * assign24630_e37987) + (locals.var_rdstemp * (((locals.var_rdsw_i * locals.var_t3_dn4) * locals.var_weffwrfactor) * p.p2))), ((locals.var_rdstemp_dn5 * assign24630_e37987) + (locals.var_rdstemp * (((locals.var_rdsw_i * locals.var_t3_dn5) * locals.var_weffwrfactor) * p.p2))), (locals.var_rdstemp * (((locals.var_rdsw_i * locals.var_t3_dn6) * locals.var_weffwrfactor) * p.p2)), (locals.var_rdstemp * (((locals.var_rdsw_i * locals.var_t3_dn7) * locals.var_weffwrfactor) * p.p2)), (locals.var_rdstemp * (((locals.var_rdsw_i * locals.var_t3_dn8) * locals.var_weffwrfactor) * p.p2)), (locals.var_rdstemp * (((locals.var_rdsw_i * locals.var_t3_dn9) * locals.var_weffwrfactor) * p.p2)), (locals.var_rdstemp * (((locals.var_rdsw_i * locals.var_t3_dn10) * locals.var_weffwrfactor) * p.p2)), (locals.var_rdstemp * (((locals.var_rdsw_i * locals.var_t3_dn11) * locals.var_weffwrfactor) * p.p2)),)
    } else {
        (locals.var_rdsi, locals.var_rdsi_dn3, locals.var_rdsi_dn4, locals.var_rdsi_dn5, locals.var_rdsi_dn6, locals.var_rdsi_dn7, locals.var_rdsi_dn8, locals.var_rdsi_dn9, locals.var_rdsi_dn10, locals.var_rdsi_dn11,)
    }
};
        locals.var_rdsi = assign24630_e37990;
        locals.var_rdsi_dn3 = assign24630_e37990_d_n3;
        locals.var_rdsi_dn4 = assign24630_e37990_d_n4;
        locals.var_rdsi_dn5 = assign24630_e37990_d_n5;
        locals.var_rdsi_dn6 = assign24630_e37990_d_n6;
        locals.var_rdsi_dn7 = assign24630_e37990_d_n7;
        locals.var_rdsi_dn8 = assign24630_e37990_d_n8;
        locals.var_rdsi_dn9 = assign24630_e37990_d_n9;
        locals.var_rdsi_dn10 = assign24630_e37990_d_n10;
        locals.var_rdsi_dn11 = assign24630_e37990_d_n11;

        let (assign24640_e38002, assign24640_e38002_d_n3, assign24640_e38002_d_n4, assign24640_e38002_d_n5, assign24640_e38002_d_n6, assign24640_e38002_d_n7, assign24640_e38002_d_n8, assign24640_e38002_d_n9, assign24640_e38002_d_n10, assign24640_e38002_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdrain, locals.var_rdrain_dn3, locals.var_rdrain_dn4, locals.var_rdrain_dn5, locals.var_rdrain_dn6, locals.var_rdrain_dn7, locals.var_rdrain_dn8, locals.var_rdrain_dn9, locals.var_rdrain_dn10, locals.var_rdrain_dn11,)
    }
};
        locals.var_rdrain = assign24640_e38002;
        locals.var_rdrain_dn3 = assign24640_e38002_d_n3;
        locals.var_rdrain_dn4 = assign24640_e38002_d_n4;
        locals.var_rdrain_dn5 = assign24640_e38002_d_n5;
        locals.var_rdrain_dn6 = assign24640_e38002_d_n6;
        locals.var_rdrain_dn7 = assign24640_e38002_d_n7;
        locals.var_rdrain_dn8 = assign24640_e38002_d_n8;
        locals.var_rdrain_dn9 = assign24640_e38002_d_n9;
        locals.var_rdrain_dn10 = assign24640_e38002_d_n10;
        locals.var_rdrain_dn11 = assign24640_e38002_d_n11;

        let (assign24650_e38014, assign24650_e38014_d_n3, assign24650_e38014_d_n4, assign24650_e38014_d_n5, assign24650_e38014_d_n6, assign24650_e38014_d_n7, assign24650_e38014_d_n8, assign24650_e38014_d_n9, assign24650_e38014_d_n10, assign24650_e38014_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rsource, locals.var_rsource_dn3, locals.var_rsource_dn4, locals.var_rsource_dn5, locals.var_rsource_dn6, locals.var_rsource_dn7, locals.var_rsource_dn8, locals.var_rsource_dn9, locals.var_rsource_dn10, locals.var_rsource_dn11,)
    }
};
        locals.var_rsource = assign24650_e38014;
        locals.var_rsource_dn3 = assign24650_e38014_d_n3;
        locals.var_rsource_dn4 = assign24650_e38014_d_n4;
        locals.var_rsource_dn5 = assign24650_e38014_d_n5;
        locals.var_rsource_dn6 = assign24650_e38014_d_n6;
        locals.var_rsource_dn7 = assign24650_e38014_d_n7;
        locals.var_rsource_dn8 = assign24650_e38014_d_n8;
        locals.var_rsource_dn9 = assign24650_e38014_d_n9;
        locals.var_rsource_dn10 = assign24650_e38014_d_n10;
        locals.var_rsource_dn11 = assign24650_e38014_d_n11;

        let (assign24660_e38040, assign24660_e38040_d_n3, assign24660_e38040_d_n4, assign24660_e38040_d_n5, assign24660_e38040_d_n6, assign24660_e38040_d_n7, assign24660_e38040_d_n8, assign24660_e38040_d_n9, assign24660_e38040_d_n10, assign24660_e38040_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        let assign24660_e38027: f64 = (locals.var_u0_a / locals.var_dvsat);
        let assign24660_e38029: f64 = (assign24660_e38027 * locals.var_cox);
        let assign24660_e38031: f64 = (assign24660_e38029 * locals.var_weff);
        let assign24660_e38033: f64 = (assign24660_e38031 / locals.var_leff);
        let assign24660_e38035: f64 = (assign24660_e38033 * locals.var_qia);
        let assign24660_e38037: f64 = (assign24660_e38035 * locals.var_rdsi);
        let assign24660_e38038: f64 = (1.0 + assign24660_e38037);
        (assign24660_e38038, ((((((((((locals.var_u0_a_dn3 * locals.var_dvsat) - (locals.var_u0_a * locals.var_dvsat_dn3)) / (locals.var_dvsat * locals.var_dvsat)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign24660_e38033 * locals.var_qia_dn3)) * locals.var_rdsi) + (assign24660_e38035 * locals.var_rdsi_dn3)), ((((((((((locals.var_u0_a_dn4 * locals.var_dvsat) - (locals.var_u0_a * locals.var_dvsat_dn4)) / (locals.var_dvsat * locals.var_dvsat)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign24660_e38033 * locals.var_qia_dn4)) * locals.var_rdsi) + (assign24660_e38035 * locals.var_rdsi_dn4)), ((((((((((locals.var_u0_a_dn5 * locals.var_dvsat) - (locals.var_u0_a * locals.var_dvsat_dn5)) / (locals.var_dvsat * locals.var_dvsat)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign24660_e38033 * locals.var_qia_dn5)) * locals.var_rdsi) + (assign24660_e38035 * locals.var_rdsi_dn5)), ((((((((((locals.var_u0_a_dn6 * locals.var_dvsat) - (locals.var_u0_a * locals.var_dvsat_dn6)) / (locals.var_dvsat * locals.var_dvsat)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign24660_e38033 * locals.var_qia_dn6)) * locals.var_rdsi) + (assign24660_e38035 * locals.var_rdsi_dn6)), ((((((((((locals.var_u0_a_dn7 * locals.var_dvsat) - (locals.var_u0_a * locals.var_dvsat_dn7)) / (locals.var_dvsat * locals.var_dvsat)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign24660_e38033 * locals.var_qia_dn7)) * locals.var_rdsi) + (assign24660_e38035 * locals.var_rdsi_dn7)), ((((((((((locals.var_u0_a_dn8 * locals.var_dvsat) - (locals.var_u0_a * locals.var_dvsat_dn8)) / (locals.var_dvsat * locals.var_dvsat)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign24660_e38033 * locals.var_qia_dn8)) * locals.var_rdsi) + (assign24660_e38035 * locals.var_rdsi_dn8)), ((((((((((locals.var_u0_a_dn9 * locals.var_dvsat) - (locals.var_u0_a * locals.var_dvsat_dn9)) / (locals.var_dvsat * locals.var_dvsat)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign24660_e38033 * locals.var_qia_dn9)) * locals.var_rdsi) + (assign24660_e38035 * locals.var_rdsi_dn9)), ((((((((((locals.var_u0_a_dn10 * locals.var_dvsat) - (locals.var_u0_a * locals.var_dvsat_dn10)) / (locals.var_dvsat * locals.var_dvsat)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign24660_e38033 * locals.var_qia_dn10)) * locals.var_rdsi) + (assign24660_e38035 * locals.var_rdsi_dn10)), ((((((((((locals.var_u0_a_dn11 * locals.var_dvsat) - (locals.var_u0_a * locals.var_dvsat_dn11)) / (locals.var_dvsat * locals.var_dvsat)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign24660_e38033 * locals.var_qia_dn11)) * locals.var_rdsi) + (assign24660_e38035 * locals.var_rdsi_dn11)),)
    } else {
        (locals.var_dr, locals.var_dr_dn3, locals.var_dr_dn4, locals.var_dr_dn5, locals.var_dr_dn6, locals.var_dr_dn7, locals.var_dr_dn8, locals.var_dr_dn9, locals.var_dr_dn10, locals.var_dr_dn11,)
    }
};
        locals.var_dr = assign24660_e38040;
        locals.var_dr_dn3 = assign24660_e38040_d_n3;
        locals.var_dr_dn4 = assign24660_e38040_d_n4;
        locals.var_dr_dn5 = assign24660_e38040_d_n5;
        locals.var_dr_dn6 = assign24660_e38040_d_n6;
        locals.var_dr_dn7 = assign24660_e38040_d_n7;
        locals.var_dr_dn8 = assign24660_e38040_d_n8;
        locals.var_dr_dn9 = assign24660_e38040_d_n9;
        locals.var_dr_dn10 = assign24660_e38040_d_n10;
        locals.var_dr_dn11 = assign24660_e38040_d_n11;

        let (assign24670_e38057, assign24670_e38057_d_n3, assign24670_e38057_d_n4, assign24670_e38057_d_n5, assign24670_e38057_d_n6, assign24670_e38057_d_n7, assign24670_e38057_d_n8, assign24670_e38057_d_n9, assign24670_e38057_d_n10, assign24670_e38057_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24670_e38050: f64 = (2.0 * locals.var_n);
        let assign24670_e38052: f64 = (assign24670_e38050 * locals.var_vtm);
        let assign24670_e38053: f64 = (locals.var_qia + assign24670_e38052);
        let assign24670_e38054: f64 = (locals.var_a2_t / assign24670_e38053);
        let assign24670_e38055: f64 = (locals.var_a1_t + assign24670_e38054);
        (assign24670_e38055, (-((locals.var_a2_t * (locals.var_qia_dn3 + ((2.0 * locals.var_n_dn3) * locals.var_vtm))) / (assign24670_e38053 * assign24670_e38053))), (locals.var_a1_t_dn4 + (((locals.var_a2_t_dn4 * assign24670_e38053) - (locals.var_a2_t * (locals.var_qia_dn4 + (((2.0 * locals.var_n_dn4) * locals.var_vtm) + (assign24670_e38050 * locals.var_vtm_dn4))))) / (assign24670_e38053 * assign24670_e38053))), (locals.var_a1_t_dn5 + (((locals.var_a2_t_dn5 * assign24670_e38053) - (locals.var_a2_t * (locals.var_qia_dn5 + (((2.0 * locals.var_n_dn5) * locals.var_vtm) + (assign24670_e38050 * locals.var_vtm_dn5))))) / (assign24670_e38053 * assign24670_e38053))), (-((locals.var_a2_t * (locals.var_qia_dn6 + ((2.0 * locals.var_n_dn6) * locals.var_vtm))) / (assign24670_e38053 * assign24670_e38053))), (-((locals.var_a2_t * (locals.var_qia_dn7 + ((2.0 * locals.var_n_dn7) * locals.var_vtm))) / (assign24670_e38053 * assign24670_e38053))), (-((locals.var_a2_t * (locals.var_qia_dn8 + ((2.0 * locals.var_n_dn8) * locals.var_vtm))) / (assign24670_e38053 * assign24670_e38053))), (-((locals.var_a2_t * (locals.var_qia_dn9 + ((2.0 * locals.var_n_dn9) * locals.var_vtm))) / (assign24670_e38053 * assign24670_e38053))), (-((locals.var_a2_t * (locals.var_qia_dn10 + ((2.0 * locals.var_n_dn10) * locals.var_vtm))) / (assign24670_e38053 * assign24670_e38053))), (-((locals.var_a2_t * (locals.var_qia_dn11 + ((2.0 * locals.var_n_dn11) * locals.var_vtm))) / (assign24670_e38053 * assign24670_e38053))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign24670_e38057;
        locals.var_t0_dn3 = assign24670_e38057_d_n3;
        locals.var_t0_dn4 = assign24670_e38057_d_n4;
        locals.var_t0_dn5 = assign24670_e38057_d_n5;
        locals.var_t0_dn6 = assign24670_e38057_d_n6;
        locals.var_t0_dn7 = assign24670_e38057_d_n7;
        locals.var_t0_dn8 = assign24670_e38057_d_n8;
        locals.var_t0_dn9 = assign24670_e38057_d_n9;
        locals.var_t0_dn10 = assign24670_e38057_d_n10;
        locals.var_t0_dn11 = assign24670_e38057_d_n11;

        let (assign24680_e38066, assign24680_e38066_d_n3, assign24680_e38066_d_n4, assign24680_e38066_d_n5, assign24680_e38066_d_n6, assign24680_e38066_d_n7, assign24680_e38066_d_n8, assign24680_e38066_d_n9, assign24680_e38066_d_n10, assign24680_e38066_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24680_e38064: f64 = (locals.var_qis - locals.var_qid);
        (assign24680_e38064, (locals.var_qis_dn3 - locals.var_qid_dn3), (locals.var_qis_dn4 - locals.var_qid_dn4), (locals.var_qis_dn5 - locals.var_qid_dn5), (locals.var_qis_dn6 - locals.var_qid_dn6), (locals.var_qis_dn7 - locals.var_qid_dn7), (locals.var_qis_dn8 - locals.var_qid_dn8), (locals.var_qis_dn9 - locals.var_qid_dn9), (locals.var_qis_dn10 - locals.var_qid_dn10), (locals.var_qis_dn11 - locals.var_qid_dn11),)
    } else {
        (locals.var_dqsd, locals.var_dqsd_dn3, locals.var_dqsd_dn4, locals.var_dqsd_dn5, locals.var_dqsd_dn6, locals.var_dqsd_dn7, locals.var_dqsd_dn8, locals.var_dqsd_dn9, locals.var_dqsd_dn10, locals.var_dqsd_dn11,)
    }
};
        locals.var_dqsd = assign24680_e38066;
        locals.var_dqsd_dn3 = assign24680_e38066_d_n3;
        locals.var_dqsd_dn4 = assign24680_e38066_d_n4;
        locals.var_dqsd_dn5 = assign24680_e38066_d_n5;
        locals.var_dqsd_dn6 = assign24680_e38066_d_n6;
        locals.var_dqsd_dn7 = assign24680_e38066_d_n7;
        locals.var_dqsd_dn8 = assign24680_e38066_d_n8;
        locals.var_dqsd_dn9 = assign24680_e38066_d_n9;
        locals.var_dqsd_dn10 = assign24680_e38066_d_n10;
        locals.var_dqsd_dn11 = assign24680_e38066_d_n11;

        let (assign24690_e38077, assign24690_e38077_d_n3, assign24690_e38077_d_n4, assign24690_e38077_d_n5, assign24690_e38077_d_n6, assign24690_e38077_d_n7, assign24690_e38077_d_n8, assign24690_e38077_d_n9, assign24690_e38077_d_n10, assign24690_e38077_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24690_e38073: f64 = (locals.var_t0 * locals.var_dqsd);
        let assign24690_e38075: f64 = (assign24690_e38073 * locals.var_dqsd);
        (assign24690_e38075, ((((locals.var_t0_dn3 * locals.var_dqsd) + (locals.var_t0 * locals.var_dqsd_dn3)) * locals.var_dqsd) + (assign24690_e38073 * locals.var_dqsd_dn3)), ((((locals.var_t0_dn4 * locals.var_dqsd) + (locals.var_t0 * locals.var_dqsd_dn4)) * locals.var_dqsd) + (assign24690_e38073 * locals.var_dqsd_dn4)), ((((locals.var_t0_dn5 * locals.var_dqsd) + (locals.var_t0 * locals.var_dqsd_dn5)) * locals.var_dqsd) + (assign24690_e38073 * locals.var_dqsd_dn5)), ((((locals.var_t0_dn6 * locals.var_dqsd) + (locals.var_t0 * locals.var_dqsd_dn6)) * locals.var_dqsd) + (assign24690_e38073 * locals.var_dqsd_dn6)), ((((locals.var_t0_dn7 * locals.var_dqsd) + (locals.var_t0 * locals.var_dqsd_dn7)) * locals.var_dqsd) + (assign24690_e38073 * locals.var_dqsd_dn7)), ((((locals.var_t0_dn8 * locals.var_dqsd) + (locals.var_t0 * locals.var_dqsd_dn8)) * locals.var_dqsd) + (assign24690_e38073 * locals.var_dqsd_dn8)), ((((locals.var_t0_dn9 * locals.var_dqsd) + (locals.var_t0 * locals.var_dqsd_dn9)) * locals.var_dqsd) + (assign24690_e38073 * locals.var_dqsd_dn9)), ((((locals.var_t0_dn10 * locals.var_dqsd) + (locals.var_t0 * locals.var_dqsd_dn10)) * locals.var_dqsd) + (assign24690_e38073 * locals.var_dqsd_dn10)), ((((locals.var_t0_dn11 * locals.var_dqsd) + (locals.var_t0 * locals.var_dqsd_dn11)) * locals.var_dqsd) + (assign24690_e38073 * locals.var_dqsd_dn11)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign24690_e38077;
        locals.var_t1_dn3 = assign24690_e38077_d_n3;
        locals.var_t1_dn4 = assign24690_e38077_d_n4;
        locals.var_t1_dn5 = assign24690_e38077_d_n5;
        locals.var_t1_dn6 = assign24690_e38077_d_n6;
        locals.var_t1_dn7 = assign24690_e38077_d_n7;
        locals.var_t1_dn8 = assign24690_e38077_d_n8;
        locals.var_t1_dn9 = assign24690_e38077_d_n9;
        locals.var_t1_dn10 = assign24690_e38077_d_n10;
        locals.var_t1_dn11 = assign24690_e38077_d_n11;

        let (assign24700_e38088, assign24700_e38088_d_n3, assign24700_e38088_d_n4, assign24700_e38088_d_n5, assign24700_e38088_d_n6, assign24700_e38088_d_n7, assign24700_e38088_d_n8, assign24700_e38088_d_n9, assign24700_e38088_d_n10, assign24700_e38088_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24700_e38084: f64 = (locals.var_t1 + 1.0);
        let assign24700_e38086: f64 = (assign24700_e38084 - 0.001);
        (assign24700_e38086, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign24700_e38088;
        locals.var_t2_dn3 = assign24700_e38088_d_n3;
        locals.var_t2_dn4 = assign24700_e38088_d_n4;
        locals.var_t2_dn5 = assign24700_e38088_d_n5;
        locals.var_t2_dn6 = assign24700_e38088_d_n6;
        locals.var_t2_dn7 = assign24700_e38088_d_n7;
        locals.var_t2_dn8 = assign24700_e38088_d_n8;
        locals.var_t2_dn9 = assign24700_e38088_d_n9;
        locals.var_t2_dn10 = assign24700_e38088_d_n10;
        locals.var_t2_dn11 = assign24700_e38088_d_n11;

        let (assign24710_e38107, assign24710_e38107_d_n3, assign24710_e38107_d_n4, assign24710_e38107_d_n5, assign24710_e38107_d_n6, assign24710_e38107_d_n7, assign24710_e38107_d_n8, assign24710_e38107_d_n9, assign24710_e38107_d_n10, assign24710_e38107_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24710_e38094: f64 = (-1.0);
        let assign24710_e38099: f64 = (locals.var_t2 * locals.var_t2);
        let assign24710_e38101: f64 = (assign24710_e38099 + 0.004);
        let assign24710_e38102: f64 = (assign24710_e38101).sqrt();
        let assign24710_e38103: f64 = (locals.var_t2 + assign24710_e38102);
        let assign24710_e38104: f64 = (0.5 * assign24710_e38103);
        let assign24710_e38105: f64 = (assign24710_e38094 + assign24710_e38104);
        (assign24710_e38105, (0.5 * (locals.var_t2_dn3 + (((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign24710_e38102)))), (0.5 * (locals.var_t2_dn4 + (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign24710_e38102)))), (0.5 * (locals.var_t2_dn5 + (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign24710_e38102)))), (0.5 * (locals.var_t2_dn6 + (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign24710_e38102)))), (0.5 * (locals.var_t2_dn7 + (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign24710_e38102)))), (0.5 * (locals.var_t2_dn8 + (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign24710_e38102)))), (0.5 * (locals.var_t2_dn9 + (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign24710_e38102)))), (0.5 * (locals.var_t2_dn10 + (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign24710_e38102)))), (0.5 * (locals.var_t2_dn11 + (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign24710_e38102)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign24710_e38107;
        locals.var_t3_dn3 = assign24710_e38107_d_n3;
        locals.var_t3_dn4 = assign24710_e38107_d_n4;
        locals.var_t3_dn5 = assign24710_e38107_d_n5;
        locals.var_t3_dn6 = assign24710_e38107_d_n6;
        locals.var_t3_dn7 = assign24710_e38107_d_n7;
        locals.var_t3_dn8 = assign24710_e38107_d_n8;
        locals.var_t3_dn9 = assign24710_e38107_d_n9;
        locals.var_t3_dn10 = assign24710_e38107_d_n10;
        locals.var_t3_dn11 = assign24710_e38107_d_n11;

        let (assign24720_e38121, assign24720_e38121_d_n3, assign24720_e38121_d_n4, assign24720_e38121_d_n5, assign24720_e38121_d_n6, assign24720_e38121_d_n7, assign24720_e38121_d_n8, assign24720_e38121_d_n9, assign24720_e38121_d_n10, assign24720_e38121_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24720_e38116: f64 = (1.0 + locals.var_t3);
        let assign24720_e38117: f64 = (assign24720_e38116).sqrt();
        let assign24720_e38118: f64 = (1.0 + assign24720_e38117);
        let assign24720_e38119: f64 = (0.5 * assign24720_e38118);
        (assign24720_e38119, (0.5 * (locals.var_t3_dn3 / (2.0 * assign24720_e38117))), (0.5 * (locals.var_t3_dn4 / (2.0 * assign24720_e38117))), (0.5 * (locals.var_t3_dn5 / (2.0 * assign24720_e38117))), (0.5 * (locals.var_t3_dn6 / (2.0 * assign24720_e38117))), (0.5 * (locals.var_t3_dn7 / (2.0 * assign24720_e38117))), (0.5 * (locals.var_t3_dn8 / (2.0 * assign24720_e38117))), (0.5 * (locals.var_t3_dn9 / (2.0 * assign24720_e38117))), (0.5 * (locals.var_t3_dn10 / (2.0 * assign24720_e38117))), (0.5 * (locals.var_t3_dn11 / (2.0 * assign24720_e38117))),)
    } else {
        (locals.var_nsat, locals.var_nsat_dn3, locals.var_nsat_dn4, locals.var_nsat_dn5, locals.var_nsat_dn6, locals.var_nsat_dn7, locals.var_nsat_dn8, locals.var_nsat_dn9, locals.var_nsat_dn10, locals.var_nsat_dn11,)
    }
};
        locals.var_nsat = assign24720_e38121;
        locals.var_nsat_dn3 = assign24720_e38121_d_n3;
        locals.var_nsat_dn4 = assign24720_e38121_d_n4;
        locals.var_nsat_dn5 = assign24720_e38121_d_n5;
        locals.var_nsat_dn6 = assign24720_e38121_d_n6;
        locals.var_nsat_dn7 = assign24720_e38121_d_n7;
        locals.var_nsat_dn8 = assign24720_e38121_d_n8;
        locals.var_nsat_dn9 = assign24720_e38121_d_n9;
        locals.var_nsat_dn10 = assign24720_e38121_d_n10;
        locals.var_nsat_dn11 = assign24720_e38121_d_n11;

        let (assign24730_e38151, assign24730_e38151_d_n3, assign24730_e38151_d_n4, assign24730_e38151_d_n5, assign24730_e38151_d_n6, assign24730_e38151_d_n7, assign24730_e38151_d_n8, assign24730_e38151_d_n9, assign24730_e38151_d_n10, assign24730_e38151_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24730_e38129: f64 = (locals.var_nsat + 1.0);
        let assign24730_e38132: f64 = (locals.var_nsat - 1.0);
        let assign24730_e38135: f64 = (locals.var_nsat - 1.0);
        let assign24730_e38136: f64 = (assign24730_e38132 * assign24730_e38135);
        let assign24730_e38139: f64 = (0.25 * 0.01);
        let assign24730_e38141: f64 = (assign24730_e38139 * 0.01);
        let assign24730_e38142: f64 = (assign24730_e38136 + assign24730_e38141);
        let assign24730_e38143: f64 = (assign24730_e38142).sqrt();
        let assign24730_e38144: f64 = (assign24730_e38129 - assign24730_e38143);
        let assign24730_e38145: f64 = (0.5 * assign24730_e38144);
        let assign24730_e38148: f64 = (0.25 * 0.01);
        let assign24730_e38149: f64 = (assign24730_e38145 + assign24730_e38148);
        (assign24730_e38149, (0.5 * (locals.var_nsat_dn3 - (((locals.var_nsat_dn3 * assign24730_e38135) + (assign24730_e38132 * locals.var_nsat_dn3)) / (2.0 * assign24730_e38143)))), (0.5 * (locals.var_nsat_dn4 - (((locals.var_nsat_dn4 * assign24730_e38135) + (assign24730_e38132 * locals.var_nsat_dn4)) / (2.0 * assign24730_e38143)))), (0.5 * (locals.var_nsat_dn5 - (((locals.var_nsat_dn5 * assign24730_e38135) + (assign24730_e38132 * locals.var_nsat_dn5)) / (2.0 * assign24730_e38143)))), (0.5 * (locals.var_nsat_dn6 - (((locals.var_nsat_dn6 * assign24730_e38135) + (assign24730_e38132 * locals.var_nsat_dn6)) / (2.0 * assign24730_e38143)))), (0.5 * (locals.var_nsat_dn7 - (((locals.var_nsat_dn7 * assign24730_e38135) + (assign24730_e38132 * locals.var_nsat_dn7)) / (2.0 * assign24730_e38143)))), (0.5 * (locals.var_nsat_dn8 - (((locals.var_nsat_dn8 * assign24730_e38135) + (assign24730_e38132 * locals.var_nsat_dn8)) / (2.0 * assign24730_e38143)))), (0.5 * (locals.var_nsat_dn9 - (((locals.var_nsat_dn9 * assign24730_e38135) + (assign24730_e38132 * locals.var_nsat_dn9)) / (2.0 * assign24730_e38143)))), (0.5 * (locals.var_nsat_dn10 - (((locals.var_nsat_dn10 * assign24730_e38135) + (assign24730_e38132 * locals.var_nsat_dn10)) / (2.0 * assign24730_e38143)))), (0.5 * (locals.var_nsat_dn11 - (((locals.var_nsat_dn11 * assign24730_e38135) + (assign24730_e38132 * locals.var_nsat_dn11)) / (2.0 * assign24730_e38143)))),)
    } else {
        (locals.var_nsat, locals.var_nsat_dn3, locals.var_nsat_dn4, locals.var_nsat_dn5, locals.var_nsat_dn6, locals.var_nsat_dn7, locals.var_nsat_dn8, locals.var_nsat_dn9, locals.var_nsat_dn10, locals.var_nsat_dn11,)
    }
};
        locals.var_nsat = assign24730_e38151;
        locals.var_nsat_dn3 = assign24730_e38151_d_n3;
        locals.var_nsat_dn4 = assign24730_e38151_d_n4;
        locals.var_nsat_dn5 = assign24730_e38151_d_n5;
        locals.var_nsat_dn6 = assign24730_e38151_d_n6;
        locals.var_nsat_dn7 = assign24730_e38151_d_n7;
        locals.var_nsat_dn8 = assign24730_e38151_d_n8;
        locals.var_nsat_dn9 = assign24730_e38151_d_n9;
        locals.var_nsat_dn10 = assign24730_e38151_d_n10;
        locals.var_nsat_dn11 = assign24730_e38151_d_n11;

        let (assign24740_e38160, assign24740_e38160_d_n3, assign24740_e38160_d_n4, assign24740_e38160_d_n5, assign24740_e38160_d_n6, assign24740_e38160_d_n7, assign24740_e38160_d_n8, assign24740_e38160_d_n9, assign24740_e38160_d_n10, assign24740_e38160_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24740_e38158: f64 = (locals.var_qis + locals.var_qid);
        (assign24740_e38158, (locals.var_qis_dn3 + locals.var_qid_dn3), (locals.var_qis_dn4 + locals.var_qid_dn4), (locals.var_qis_dn5 + locals.var_qid_dn5), (locals.var_qis_dn6 + locals.var_qid_dn6), (locals.var_qis_dn7 + locals.var_qid_dn7), (locals.var_qis_dn8 + locals.var_qid_dn8), (locals.var_qis_dn9 + locals.var_qid_dn9), (locals.var_qis_dn10 + locals.var_qid_dn10), (locals.var_qis_dn11 + locals.var_qid_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign24740_e38160;
        locals.var_t0_dn3 = assign24740_e38160_d_n3;
        locals.var_t0_dn4 = assign24740_e38160_d_n4;
        locals.var_t0_dn5 = assign24740_e38160_d_n5;
        locals.var_t0_dn6 = assign24740_e38160_d_n6;
        locals.var_t0_dn7 = assign24740_e38160_d_n7;
        locals.var_t0_dn8 = assign24740_e38160_d_n8;
        locals.var_t0_dn9 = assign24740_e38160_d_n9;
        locals.var_t0_dn10 = assign24740_e38160_d_n10;
        locals.var_t0_dn11 = assign24740_e38160_d_n11;

        let (assign24750_e38169, assign24750_e38169_d_n3, assign24750_e38169_d_n4, assign24750_e38169_d_n5, assign24750_e38169_d_n6, assign24750_e38169_d_n7, assign24750_e38169_d_n8, assign24750_e38169_d_n9, assign24750_e38169_d_n10, assign24750_e38169_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24750_e38167: f64 = (locals.var_qis - locals.var_qid);
        (assign24750_e38167, (locals.var_qis_dn3 - locals.var_qid_dn3), (locals.var_qis_dn4 - locals.var_qid_dn4), (locals.var_qis_dn5 - locals.var_qid_dn5), (locals.var_qis_dn6 - locals.var_qid_dn6), (locals.var_qis_dn7 - locals.var_qid_dn7), (locals.var_qis_dn8 - locals.var_qid_dn8), (locals.var_qis_dn9 - locals.var_qid_dn9), (locals.var_qis_dn10 - locals.var_qid_dn10), (locals.var_qis_dn11 - locals.var_qid_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign24750_e38169;
        locals.var_t1_dn3 = assign24750_e38169_d_n3;
        locals.var_t1_dn4 = assign24750_e38169_d_n4;
        locals.var_t1_dn5 = assign24750_e38169_d_n5;
        locals.var_t1_dn6 = assign24750_e38169_d_n6;
        locals.var_t1_dn7 = assign24750_e38169_d_n7;
        locals.var_t1_dn8 = assign24750_e38169_d_n8;
        locals.var_t1_dn9 = assign24750_e38169_d_n9;
        locals.var_t1_dn10 = assign24750_e38169_d_n10;
        locals.var_t1_dn11 = assign24750_e38169_d_n11;

        let (assign24760_e38180, assign24760_e38180_d_n3, assign24760_e38180_d_n4, assign24760_e38180_d_n5, assign24760_e38180_d_n6, assign24760_e38180_d_n7, assign24760_e38180_d_n8, assign24760_e38180_d_n9, assign24760_e38180_d_n10, assign24760_e38180_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24760_e38177: f64 = (locals.var_t0 + locals.var_m0_t);
        let assign24760_e38178: f64 = (locals.var_t1 / assign24760_e38177);
        (assign24760_e38178, (((locals.var_t1_dn3 * assign24760_e38177) - (locals.var_t1 * locals.var_t0_dn3)) / (assign24760_e38177 * assign24760_e38177)), (((locals.var_t1_dn4 * assign24760_e38177) - (locals.var_t1 * (locals.var_t0_dn4 + locals.var_m0_t_dn4))) / (assign24760_e38177 * assign24760_e38177)), (((locals.var_t1_dn5 * assign24760_e38177) - (locals.var_t1 * (locals.var_t0_dn5 + locals.var_m0_t_dn5))) / (assign24760_e38177 * assign24760_e38177)), (((locals.var_t1_dn6 * assign24760_e38177) - (locals.var_t1 * locals.var_t0_dn6)) / (assign24760_e38177 * assign24760_e38177)), (((locals.var_t1_dn7 * assign24760_e38177) - (locals.var_t1 * locals.var_t0_dn7)) / (assign24760_e38177 * assign24760_e38177)), (((locals.var_t1_dn8 * assign24760_e38177) - (locals.var_t1 * locals.var_t0_dn8)) / (assign24760_e38177 * assign24760_e38177)), (((locals.var_t1_dn9 * assign24760_e38177) - (locals.var_t1 * locals.var_t0_dn9)) / (assign24760_e38177 * assign24760_e38177)), (((locals.var_t1_dn10 * assign24760_e38177) - (locals.var_t1 * locals.var_t0_dn10)) / (assign24760_e38177 * assign24760_e38177)), (((locals.var_t1_dn11 * assign24760_e38177) - (locals.var_t1 * locals.var_t0_dn11)) / (assign24760_e38177 * assign24760_e38177)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign24760_e38180;
        locals.var_t2_dn3 = assign24760_e38180_d_n3;
        locals.var_t2_dn4 = assign24760_e38180_d_n4;
        locals.var_t2_dn5 = assign24760_e38180_d_n5;
        locals.var_t2_dn6 = assign24760_e38180_d_n6;
        locals.var_t2_dn7 = assign24760_e38180_d_n7;
        locals.var_t2_dn8 = assign24760_e38180_d_n8;
        locals.var_t2_dn9 = assign24760_e38180_d_n9;
        locals.var_t2_dn10 = assign24760_e38180_d_n10;
        locals.var_t2_dn11 = assign24760_e38180_d_n11;

        let (assign24770_e38191, assign24770_e38191_d_n3, assign24770_e38191_d_n4, assign24770_e38191_d_n5, assign24770_e38191_d_n6, assign24770_e38191_d_n7, assign24770_e38191_d_n8, assign24770_e38191_d_n9, assign24770_e38191_d_n10, assign24770_e38191_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24770_e38187: f64 = (locals.var_k0_t * locals.var_t2);
        let assign24770_e38189: f64 = (assign24770_e38187 * locals.var_t2);
        (assign24770_e38189, (((locals.var_k0_t * locals.var_t2_dn3) * locals.var_t2) + (assign24770_e38187 * locals.var_t2_dn3)), ((((locals.var_k0_t_dn4 * locals.var_t2) + (locals.var_k0_t * locals.var_t2_dn4)) * locals.var_t2) + (assign24770_e38187 * locals.var_t2_dn4)), ((((locals.var_k0_t_dn5 * locals.var_t2) + (locals.var_k0_t * locals.var_t2_dn5)) * locals.var_t2) + (assign24770_e38187 * locals.var_t2_dn5)), (((locals.var_k0_t * locals.var_t2_dn6) * locals.var_t2) + (assign24770_e38187 * locals.var_t2_dn6)), (((locals.var_k0_t * locals.var_t2_dn7) * locals.var_t2) + (assign24770_e38187 * locals.var_t2_dn7)), (((locals.var_k0_t * locals.var_t2_dn8) * locals.var_t2) + (assign24770_e38187 * locals.var_t2_dn8)), (((locals.var_k0_t * locals.var_t2_dn9) * locals.var_t2) + (assign24770_e38187 * locals.var_t2_dn9)), (((locals.var_k0_t * locals.var_t2_dn10) * locals.var_t2) + (assign24770_e38187 * locals.var_t2_dn10)), (((locals.var_k0_t * locals.var_t2_dn11) * locals.var_t2) + (assign24770_e38187 * locals.var_t2_dn11)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign24770_e38191;
        locals.var_t3_dn3 = assign24770_e38191_d_n3;
        locals.var_t3_dn4 = assign24770_e38191_d_n4;
        locals.var_t3_dn5 = assign24770_e38191_d_n5;
        locals.var_t3_dn6 = assign24770_e38191_d_n6;
        locals.var_t3_dn7 = assign24770_e38191_d_n7;
        locals.var_t3_dn8 = assign24770_e38191_d_n8;
        locals.var_t3_dn9 = assign24770_e38191_d_n9;
        locals.var_t3_dn10 = assign24770_e38191_d_n10;
        locals.var_t3_dn11 = assign24770_e38191_d_n11;

        let (assign24780_e38200, assign24780_e38200_d_n3, assign24780_e38200_d_n4, assign24780_e38200_d_n5, assign24780_e38200_d_n6, assign24780_e38200_d_n7, assign24780_e38200_d_n8, assign24780_e38200_d_n9, assign24780_e38200_d_n10, assign24780_e38200_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24780_e38198: f64 = (1.0 + locals.var_t3);
        (assign24780_e38198, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    } else {
        (locals.var_mnud, locals.var_mnud_dn3, locals.var_mnud_dn4, locals.var_mnud_dn5, locals.var_mnud_dn6, locals.var_mnud_dn7, locals.var_mnud_dn8, locals.var_mnud_dn9, locals.var_mnud_dn10, locals.var_mnud_dn11,)
    }
};
        locals.var_mnud = assign24780_e38200;
        locals.var_mnud_dn3 = assign24780_e38200_d_n3;
        locals.var_mnud_dn4 = assign24780_e38200_d_n4;
        locals.var_mnud_dn5 = assign24780_e38200_d_n5;
        locals.var_mnud_dn6 = assign24780_e38200_d_n6;
        locals.var_mnud_dn7 = assign24780_e38200_d_n7;
        locals.var_mnud_dn8 = assign24780_e38200_d_n8;
        locals.var_mnud_dn9 = assign24780_e38200_d_n9;
        locals.var_mnud_dn10 = assign24780_e38200_d_n10;
        locals.var_mnud_dn11 = assign24780_e38200_d_n11;

    }

    pub(super) fn stamp_transient_block_65(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign24790_e38225, assign24790_e38225_d_n3, assign24790_e38225_d_n4, assign24790_e38225_d_n5, assign24790_e38225_d_n6, assign24790_e38225_d_n7, assign24790_e38225_d_n8, assign24790_e38225_d_n9, assign24790_e38225_d_n10, assign24790_e38225_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24790_e38210: f64 = (locals.var_c0sisat_t * locals.var_t1);
        let assign24790_e38212: f64 = (assign24790_e38210 * locals.var_t1);
        let assign24790_e38213: f64 = (locals.var_c0si_t + assign24790_e38212);
        let assign24790_e38214: f64 = (0.0_f64).max(assign24790_e38213);
        let assign24790_e38216: f64 = (assign24790_e38214 * locals.var_t0);
        let assign24790_e38219: f64 = (2.0 * locals.var_n);
        let assign24790_e38221: f64 = (assign24790_e38219 * locals.var_vtm);
        let assign24790_e38222: f64 = (assign24790_e38216 + assign24790_e38221);
        let assign24790_e38223: f64 = (locals.var_c0_t / assign24790_e38222);
        (assign24790_e38223, (-((locals.var_c0_t * (((if 0.0 >= assign24790_e38213 { 0.0 } else { (((locals.var_c0sisat_t * locals.var_t1_dn3) * locals.var_t1) + (assign24790_e38210 * locals.var_t1_dn3)) } * locals.var_t0) + (assign24790_e38214 * locals.var_t0_dn3)) + ((2.0 * locals.var_n_dn3) * locals.var_vtm))) / (assign24790_e38222 * assign24790_e38222))), (((locals.var_c0_t_dn4 * assign24790_e38222) - (locals.var_c0_t * (((if 0.0 >= assign24790_e38213 { 0.0 } else { (locals.var_c0si_t_dn4 + ((((locals.var_c0sisat_t_dn4 * locals.var_t1) + (locals.var_c0sisat_t * locals.var_t1_dn4)) * locals.var_t1) + (assign24790_e38210 * locals.var_t1_dn4))) } * locals.var_t0) + (assign24790_e38214 * locals.var_t0_dn4)) + (((2.0 * locals.var_n_dn4) * locals.var_vtm) + (assign24790_e38219 * locals.var_vtm_dn4))))) / (assign24790_e38222 * assign24790_e38222)), (((locals.var_c0_t_dn5 * assign24790_e38222) - (locals.var_c0_t * (((if 0.0 >= assign24790_e38213 { 0.0 } else { (locals.var_c0si_t_dn5 + ((((locals.var_c0sisat_t_dn5 * locals.var_t1) + (locals.var_c0sisat_t * locals.var_t1_dn5)) * locals.var_t1) + (assign24790_e38210 * locals.var_t1_dn5))) } * locals.var_t0) + (assign24790_e38214 * locals.var_t0_dn5)) + (((2.0 * locals.var_n_dn5) * locals.var_vtm) + (assign24790_e38219 * locals.var_vtm_dn5))))) / (assign24790_e38222 * assign24790_e38222)), (-((locals.var_c0_t * (((if 0.0 >= assign24790_e38213 { 0.0 } else { (((locals.var_c0sisat_t * locals.var_t1_dn6) * locals.var_t1) + (assign24790_e38210 * locals.var_t1_dn6)) } * locals.var_t0) + (assign24790_e38214 * locals.var_t0_dn6)) + ((2.0 * locals.var_n_dn6) * locals.var_vtm))) / (assign24790_e38222 * assign24790_e38222))), (-((locals.var_c0_t * (((if 0.0 >= assign24790_e38213 { 0.0 } else { (((locals.var_c0sisat_t * locals.var_t1_dn7) * locals.var_t1) + (assign24790_e38210 * locals.var_t1_dn7)) } * locals.var_t0) + (assign24790_e38214 * locals.var_t0_dn7)) + ((2.0 * locals.var_n_dn7) * locals.var_vtm))) / (assign24790_e38222 * assign24790_e38222))), (-((locals.var_c0_t * (((if 0.0 >= assign24790_e38213 { 0.0 } else { (((locals.var_c0sisat_t * locals.var_t1_dn8) * locals.var_t1) + (assign24790_e38210 * locals.var_t1_dn8)) } * locals.var_t0) + (assign24790_e38214 * locals.var_t0_dn8)) + ((2.0 * locals.var_n_dn8) * locals.var_vtm))) / (assign24790_e38222 * assign24790_e38222))), (-((locals.var_c0_t * (((if 0.0 >= assign24790_e38213 { 0.0 } else { (((locals.var_c0sisat_t * locals.var_t1_dn9) * locals.var_t1) + (assign24790_e38210 * locals.var_t1_dn9)) } * locals.var_t0) + (assign24790_e38214 * locals.var_t0_dn9)) + ((2.0 * locals.var_n_dn9) * locals.var_vtm))) / (assign24790_e38222 * assign24790_e38222))), (-((locals.var_c0_t * (((if 0.0 >= assign24790_e38213 { 0.0 } else { (((locals.var_c0sisat_t * locals.var_t1_dn10) * locals.var_t1) + (assign24790_e38210 * locals.var_t1_dn10)) } * locals.var_t0) + (assign24790_e38214 * locals.var_t0_dn10)) + ((2.0 * locals.var_n_dn10) * locals.var_vtm))) / (assign24790_e38222 * assign24790_e38222))), (-((locals.var_c0_t * (((if 0.0 >= assign24790_e38213 { 0.0 } else { (((locals.var_c0sisat_t * locals.var_t1_dn11) * locals.var_t1) + (assign24790_e38210 * locals.var_t1_dn11)) } * locals.var_t0) + (assign24790_e38214 * locals.var_t0_dn11)) + ((2.0 * locals.var_n_dn11) * locals.var_vtm))) / (assign24790_e38222 * assign24790_e38222))),)
    } else {
        (locals.var_t9, locals.var_t9_dn3, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11,)
    }
};
        locals.var_t9 = assign24790_e38225;
        locals.var_t9_dn3 = assign24790_e38225_d_n3;
        locals.var_t9_dn4 = assign24790_e38225_d_n4;
        locals.var_t9_dn5 = assign24790_e38225_d_n5;
        locals.var_t9_dn6 = assign24790_e38225_d_n6;
        locals.var_t9_dn7 = assign24790_e38225_d_n7;
        locals.var_t9_dn8 = assign24790_e38225_d_n8;
        locals.var_t9_dn9 = assign24790_e38225_d_n9;
        locals.var_t9_dn10 = assign24790_e38225_d_n10;
        locals.var_t9_dn11 = assign24790_e38225_d_n11;

        let (assign24800_e38234, assign24800_e38234_d_n3, assign24800_e38234_d_n4, assign24800_e38234_d_n5, assign24800_e38234_d_n6, assign24800_e38234_d_n7, assign24800_e38234_d_n8, assign24800_e38234_d_n9, assign24800_e38234_d_n10, assign24800_e38234_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24800_e38231: f64 = (-locals.var_t9);
        let assign24800_e38232: f64 = { let limited_exp_arg = assign24800_e38231; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign24800_e38232, ({ let limited_exp_arg = assign24800_e38231; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t9_dn3)), ({ let limited_exp_arg = assign24800_e38231; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t9_dn4)), ({ let limited_exp_arg = assign24800_e38231; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t9_dn5)), ({ let limited_exp_arg = assign24800_e38231; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t9_dn6)), ({ let limited_exp_arg = assign24800_e38231; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t9_dn7)), ({ let limited_exp_arg = assign24800_e38231; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t9_dn8)), ({ let limited_exp_arg = assign24800_e38231; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t9_dn9)), ({ let limited_exp_arg = assign24800_e38231; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t9_dn10)), ({ let limited_exp_arg = assign24800_e38231; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t9_dn11)),)
    } else {
        (locals.var_mnud1, locals.var_mnud1_dn3, locals.var_mnud1_dn4, locals.var_mnud1_dn5, locals.var_mnud1_dn6, locals.var_mnud1_dn7, locals.var_mnud1_dn8, locals.var_mnud1_dn9, locals.var_mnud1_dn10, locals.var_mnud1_dn11,)
    }
};
        locals.var_mnud1 = assign24800_e38234;
        locals.var_mnud1_dn3 = assign24800_e38234_d_n3;
        locals.var_mnud1_dn4 = assign24800_e38234_d_n4;
        locals.var_mnud1_dn5 = assign24800_e38234_d_n5;
        locals.var_mnud1_dn6 = assign24800_e38234_d_n6;
        locals.var_mnud1_dn7 = assign24800_e38234_d_n7;
        locals.var_mnud1_dn8 = assign24800_e38234_d_n8;
        locals.var_mnud1_dn9 = assign24800_e38234_d_n9;
        locals.var_mnud1_dn10 = assign24800_e38234_d_n10;
        locals.var_mnud1_dn11 = assign24800_e38234_d_n11;

        let (assign24810_e38243, assign24810_e38243_d_n3, assign24810_e38243_d_n4, assign24810_e38243_d_n5, assign24810_e38243_d_n6, assign24810_e38243_d_n7, assign24810_e38243_d_n8, assign24810_e38243_d_n9, assign24810_e38243_d_n10, assign24810_e38243_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24810_e38241: f64 = (locals.var_dmob_dl * locals.var_dvsatinv);
        (assign24810_e38241, ((locals.var_dmob_dl_dn3 * locals.var_dvsatinv) + (locals.var_dmob_dl * locals.var_dvsatinv_dn3)), ((locals.var_dmob_dl_dn4 * locals.var_dvsatinv) + (locals.var_dmob_dl * locals.var_dvsatinv_dn4)), ((locals.var_dmob_dl_dn5 * locals.var_dvsatinv) + (locals.var_dmob_dl * locals.var_dvsatinv_dn5)), ((locals.var_dmob_dl_dn6 * locals.var_dvsatinv) + (locals.var_dmob_dl * locals.var_dvsatinv_dn6)), ((locals.var_dmob_dl_dn7 * locals.var_dvsatinv) + (locals.var_dmob_dl * locals.var_dvsatinv_dn7)), ((locals.var_dmob_dl_dn8 * locals.var_dvsatinv) + (locals.var_dmob_dl * locals.var_dvsatinv_dn8)), ((locals.var_dmob_dl_dn9 * locals.var_dvsatinv) + (locals.var_dmob_dl * locals.var_dvsatinv_dn9)), ((locals.var_dmob_dl_dn10 * locals.var_dvsatinv) + (locals.var_dmob_dl * locals.var_dvsatinv_dn10)), ((locals.var_dmob_dl_dn11 * locals.var_dvsatinv) + (locals.var_dmob_dl * locals.var_dvsatinv_dn11)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign24810_e38243;
        locals.var_t0_dn3 = assign24810_e38243_d_n3;
        locals.var_t0_dn4 = assign24810_e38243_d_n4;
        locals.var_t0_dn5 = assign24810_e38243_d_n5;
        locals.var_t0_dn6 = assign24810_e38243_d_n6;
        locals.var_t0_dn7 = assign24810_e38243_d_n7;
        locals.var_t0_dn8 = assign24810_e38243_d_n8;
        locals.var_t0_dn9 = assign24810_e38243_d_n9;
        locals.var_t0_dn10 = assign24810_e38243_d_n10;
        locals.var_t0_dn11 = assign24810_e38243_d_n11;

        let (assign24820_e38260, assign24820_e38260_d_n3, assign24820_e38260_d_n4, assign24820_e38260_d_n5, assign24820_e38260_d_n6, assign24820_e38260_d_n7, assign24820_e38260_d_n8, assign24820_e38260_d_n9, assign24820_e38260_d_n10, assign24820_e38260_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24820_e38253: f64 = (locals.var_zsat * locals.var_t0);
        let assign24820_e38255: f64 = (assign24820_e38253 * locals.var_t0);
        let assign24820_e38256: f64 = (0.5 * assign24820_e38255);
        let assign24820_e38257: f64 = (1.0 + assign24820_e38256);
        let assign24820_e38258: f64 = (locals.var_alpha_dd * assign24820_e38257);
        (assign24820_e38258, ((locals.var_alpha_dd_dn3 * assign24820_e38257) + (locals.var_alpha_dd * (0.5 * ((((locals.var_zsat_dn3 * locals.var_t0) + (locals.var_zsat * locals.var_t0_dn3)) * locals.var_t0) + (assign24820_e38253 * locals.var_t0_dn3))))), ((locals.var_alpha_dd_dn4 * assign24820_e38257) + (locals.var_alpha_dd * (0.5 * ((((locals.var_zsat_dn4 * locals.var_t0) + (locals.var_zsat * locals.var_t0_dn4)) * locals.var_t0) + (assign24820_e38253 * locals.var_t0_dn4))))), ((locals.var_alpha_dd_dn5 * assign24820_e38257) + (locals.var_alpha_dd * (0.5 * ((((locals.var_zsat_dn5 * locals.var_t0) + (locals.var_zsat * locals.var_t0_dn5)) * locals.var_t0) + (assign24820_e38253 * locals.var_t0_dn5))))), ((locals.var_alpha_dd_dn6 * assign24820_e38257) + (locals.var_alpha_dd * (0.5 * ((((locals.var_zsat_dn6 * locals.var_t0) + (locals.var_zsat * locals.var_t0_dn6)) * locals.var_t0) + (assign24820_e38253 * locals.var_t0_dn6))))), ((locals.var_alpha_dd_dn7 * assign24820_e38257) + (locals.var_alpha_dd * (0.5 * ((((locals.var_zsat_dn7 * locals.var_t0) + (locals.var_zsat * locals.var_t0_dn7)) * locals.var_t0) + (assign24820_e38253 * locals.var_t0_dn7))))), ((locals.var_alpha_dd_dn8 * assign24820_e38257) + (locals.var_alpha_dd * (0.5 * ((((locals.var_zsat_dn8 * locals.var_t0) + (locals.var_zsat * locals.var_t0_dn8)) * locals.var_t0) + (assign24820_e38253 * locals.var_t0_dn8))))), ((locals.var_alpha_dd_dn9 * assign24820_e38257) + (locals.var_alpha_dd * (0.5 * ((((locals.var_zsat_dn9 * locals.var_t0) + (locals.var_zsat * locals.var_t0_dn9)) * locals.var_t0) + (assign24820_e38253 * locals.var_t0_dn9))))), ((locals.var_alpha_dd_dn10 * assign24820_e38257) + (locals.var_alpha_dd * (0.5 * ((((locals.var_zsat_dn10 * locals.var_t0) + (locals.var_zsat * locals.var_t0_dn10)) * locals.var_t0) + (assign24820_e38253 * locals.var_t0_dn10))))), ((locals.var_alpha_dd_dn11 * assign24820_e38257) + (locals.var_alpha_dd * (0.5 * ((((locals.var_zsat_dn11 * locals.var_t0) + (locals.var_zsat * locals.var_t0_dn11)) * locals.var_t0) + (assign24820_e38253 * locals.var_t0_dn11))))),)
    } else {
        (locals.var_alpha1, locals.var_alpha1_dn3, locals.var_alpha1_dn4, locals.var_alpha1_dn5, locals.var_alpha1_dn6, locals.var_alpha1_dn7, locals.var_alpha1_dn8, locals.var_alpha1_dn9, locals.var_alpha1_dn10, locals.var_alpha1_dn11,)
    }
};
        locals.var_alpha1 = assign24820_e38260;
        locals.var_alpha1_dn3 = assign24820_e38260_d_n3;
        locals.var_alpha1_dn4 = assign24820_e38260_d_n4;
        locals.var_alpha1_dn5 = assign24820_e38260_d_n5;
        locals.var_alpha1_dn6 = assign24820_e38260_d_n6;
        locals.var_alpha1_dn7 = assign24820_e38260_d_n7;
        locals.var_alpha1_dn8 = assign24820_e38260_d_n8;
        locals.var_alpha1_dn9 = assign24820_e38260_d_n9;
        locals.var_alpha1_dn10 = assign24820_e38260_d_n10;
        locals.var_alpha1_dn11 = assign24820_e38260_d_n11;

        let (assign24830_e38288, assign24830_e38288_d_n3, assign24830_e38288_d_n4, assign24830_e38288_d_n5, assign24830_e38288_d_n6, assign24830_e38288_d_n7, assign24830_e38288_d_n8, assign24830_e38288_d_n9, assign24830_e38288_d_n10, assign24830_e38288_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24830_e38268: f64 = (locals.var_alpha1 - 0.001);
        let assign24830_e38271: f64 = (locals.var_alpha1 - 0.001);
        let assign24830_e38274: f64 = (locals.var_alpha1 - 0.001);
        let assign24830_e38275: f64 = (assign24830_e38271 * assign24830_e38274);
        let assign24830_e38278: f64 = (4.0 * 1e-5);
        let assign24830_e38280: f64 = (assign24830_e38278 * 1e-5);
        let assign24830_e38281: f64 = (assign24830_e38275 + assign24830_e38280);
        let assign24830_e38282: f64 = (assign24830_e38281).sqrt();
        let assign24830_e38283: f64 = (assign24830_e38268 + assign24830_e38282);
        let assign24830_e38284: f64 = (0.5 * assign24830_e38283);
        let assign24830_e38286: f64 = (assign24830_e38284 + 0.001);
        (assign24830_e38286, (0.5 * (locals.var_alpha1_dn3 + (((locals.var_alpha1_dn3 * assign24830_e38274) + (assign24830_e38271 * locals.var_alpha1_dn3)) / (2.0 * assign24830_e38282)))), (0.5 * (locals.var_alpha1_dn4 + (((locals.var_alpha1_dn4 * assign24830_e38274) + (assign24830_e38271 * locals.var_alpha1_dn4)) / (2.0 * assign24830_e38282)))), (0.5 * (locals.var_alpha1_dn5 + (((locals.var_alpha1_dn5 * assign24830_e38274) + (assign24830_e38271 * locals.var_alpha1_dn5)) / (2.0 * assign24830_e38282)))), (0.5 * (locals.var_alpha1_dn6 + (((locals.var_alpha1_dn6 * assign24830_e38274) + (assign24830_e38271 * locals.var_alpha1_dn6)) / (2.0 * assign24830_e38282)))), (0.5 * (locals.var_alpha1_dn7 + (((locals.var_alpha1_dn7 * assign24830_e38274) + (assign24830_e38271 * locals.var_alpha1_dn7)) / (2.0 * assign24830_e38282)))), (0.5 * (locals.var_alpha1_dn8 + (((locals.var_alpha1_dn8 * assign24830_e38274) + (assign24830_e38271 * locals.var_alpha1_dn8)) / (2.0 * assign24830_e38282)))), (0.5 * (locals.var_alpha1_dn9 + (((locals.var_alpha1_dn9 * assign24830_e38274) + (assign24830_e38271 * locals.var_alpha1_dn9)) / (2.0 * assign24830_e38282)))), (0.5 * (locals.var_alpha1_dn10 + (((locals.var_alpha1_dn10 * assign24830_e38274) + (assign24830_e38271 * locals.var_alpha1_dn10)) / (2.0 * assign24830_e38282)))), (0.5 * (locals.var_alpha1_dn11 + (((locals.var_alpha1_dn11 * assign24830_e38274) + (assign24830_e38271 * locals.var_alpha1_dn11)) / (2.0 * assign24830_e38282)))),)
    } else {
        (locals.var_alpha1, locals.var_alpha1_dn3, locals.var_alpha1_dn4, locals.var_alpha1_dn5, locals.var_alpha1_dn6, locals.var_alpha1_dn7, locals.var_alpha1_dn8, locals.var_alpha1_dn9, locals.var_alpha1_dn10, locals.var_alpha1_dn11,)
    }
};
        locals.var_alpha1 = assign24830_e38288;
        locals.var_alpha1_dn3 = assign24830_e38288_d_n3;
        locals.var_alpha1_dn4 = assign24830_e38288_d_n4;
        locals.var_alpha1_dn5 = assign24830_e38288_d_n5;
        locals.var_alpha1_dn6 = assign24830_e38288_d_n6;
        locals.var_alpha1_dn7 = assign24830_e38288_d_n7;
        locals.var_alpha1_dn8 = assign24830_e38288_d_n8;
        locals.var_alpha1_dn9 = assign24830_e38288_d_n9;
        locals.var_alpha1_dn10 = assign24830_e38288_d_n10;
        locals.var_alpha1_dn11 = assign24830_e38288_d_n11;

        let (assign24840_e38299, assign24840_e38299_d_n3, assign24840_e38299_d_n4, assign24840_e38299_d_n5, assign24840_e38299_d_n6, assign24840_e38299_d_n7, assign24840_e38299_d_n8, assign24840_e38299_d_n9, assign24840_e38299_d_n10, assign24840_e38299_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24840_e38296: f64 = (locals.var_nvt * locals.var_alpha_dd);
        let assign24840_e38297: f64 = (locals.var_qim + assign24840_e38296);
        (assign24840_e38297, (locals.var_qim_dn3 + ((locals.var_nvt_dn3 * locals.var_alpha_dd) + (locals.var_nvt * locals.var_alpha_dd_dn3))), (locals.var_qim_dn4 + ((locals.var_nvt_dn4 * locals.var_alpha_dd) + (locals.var_nvt * locals.var_alpha_dd_dn4))), (locals.var_qim_dn5 + ((locals.var_nvt_dn5 * locals.var_alpha_dd) + (locals.var_nvt * locals.var_alpha_dd_dn5))), (locals.var_qim_dn6 + ((locals.var_nvt_dn6 * locals.var_alpha_dd) + (locals.var_nvt * locals.var_alpha_dd_dn6))), (locals.var_qim_dn7 + ((locals.var_nvt_dn7 * locals.var_alpha_dd) + (locals.var_nvt * locals.var_alpha_dd_dn7))), (locals.var_qim_dn8 + ((locals.var_nvt_dn8 * locals.var_alpha_dd) + (locals.var_nvt * locals.var_alpha_dd_dn8))), (locals.var_qim_dn9 + ((locals.var_nvt_dn9 * locals.var_alpha_dd) + (locals.var_nvt * locals.var_alpha_dd_dn9))), (locals.var_qim_dn10 + ((locals.var_nvt_dn10 * locals.var_alpha_dd) + (locals.var_nvt * locals.var_alpha_dd_dn10))), (locals.var_qim_dn11 + ((locals.var_nvt_dn11 * locals.var_alpha_dd) + (locals.var_nvt * locals.var_alpha_dd_dn11))),)
    } else {
        (locals.var_qim1, locals.var_qim1_dn3, locals.var_qim1_dn4, locals.var_qim1_dn5, locals.var_qim1_dn6, locals.var_qim1_dn7, locals.var_qim1_dn8, locals.var_qim1_dn9, locals.var_qim1_dn10, locals.var_qim1_dn11,)
    }
};
        locals.var_qim1 = assign24840_e38299;
        locals.var_qim1_dn3 = assign24840_e38299_d_n3;
        locals.var_qim1_dn4 = assign24840_e38299_d_n4;
        locals.var_qim1_dn5 = assign24840_e38299_d_n5;
        locals.var_qim1_dn6 = assign24840_e38299_d_n6;
        locals.var_qim1_dn7 = assign24840_e38299_d_n7;
        locals.var_qim1_dn8 = assign24840_e38299_d_n8;
        locals.var_qim1_dn9 = assign24840_e38299_d_n9;
        locals.var_qim1_dn10 = assign24840_e38299_d_n10;
        locals.var_qim1_dn11 = assign24840_e38299_d_n11;

        let (assign24850_e38312, assign24850_e38312_d_n3, assign24850_e38312_d_n4, assign24850_e38312_d_n5, assign24850_e38312_d_n6, assign24850_e38312_d_n7, assign24850_e38312_d_n8, assign24850_e38312_d_n9, assign24850_e38312_d_n10, assign24850_e38312_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24850_e38306: f64 = (locals.var_qim1 / locals.var_alpha1);
        let assign24850_e38309: f64 = (locals.var_dmob_dl / locals.var_dvsat);
        let assign24850_e38310: f64 = (assign24850_e38306 * assign24850_e38309);
        (assign24850_e38310, (((((locals.var_qim1_dn3 * locals.var_alpha1) - (locals.var_qim1 * locals.var_alpha1_dn3)) / (locals.var_alpha1 * locals.var_alpha1)) * assign24850_e38309) + (assign24850_e38306 * (((locals.var_dmob_dl_dn3 * locals.var_dvsat) - (locals.var_dmob_dl * locals.var_dvsat_dn3)) / (locals.var_dvsat * locals.var_dvsat)))), (((((locals.var_qim1_dn4 * locals.var_alpha1) - (locals.var_qim1 * locals.var_alpha1_dn4)) / (locals.var_alpha1 * locals.var_alpha1)) * assign24850_e38309) + (assign24850_e38306 * (((locals.var_dmob_dl_dn4 * locals.var_dvsat) - (locals.var_dmob_dl * locals.var_dvsat_dn4)) / (locals.var_dvsat * locals.var_dvsat)))), (((((locals.var_qim1_dn5 * locals.var_alpha1) - (locals.var_qim1 * locals.var_alpha1_dn5)) / (locals.var_alpha1 * locals.var_alpha1)) * assign24850_e38309) + (assign24850_e38306 * (((locals.var_dmob_dl_dn5 * locals.var_dvsat) - (locals.var_dmob_dl * locals.var_dvsat_dn5)) / (locals.var_dvsat * locals.var_dvsat)))), (((((locals.var_qim1_dn6 * locals.var_alpha1) - (locals.var_qim1 * locals.var_alpha1_dn6)) / (locals.var_alpha1 * locals.var_alpha1)) * assign24850_e38309) + (assign24850_e38306 * (((locals.var_dmob_dl_dn6 * locals.var_dvsat) - (locals.var_dmob_dl * locals.var_dvsat_dn6)) / (locals.var_dvsat * locals.var_dvsat)))), (((((locals.var_qim1_dn7 * locals.var_alpha1) - (locals.var_qim1 * locals.var_alpha1_dn7)) / (locals.var_alpha1 * locals.var_alpha1)) * assign24850_e38309) + (assign24850_e38306 * (((locals.var_dmob_dl_dn7 * locals.var_dvsat) - (locals.var_dmob_dl * locals.var_dvsat_dn7)) / (locals.var_dvsat * locals.var_dvsat)))), (((((locals.var_qim1_dn8 * locals.var_alpha1) - (locals.var_qim1 * locals.var_alpha1_dn8)) / (locals.var_alpha1 * locals.var_alpha1)) * assign24850_e38309) + (assign24850_e38306 * (((locals.var_dmob_dl_dn8 * locals.var_dvsat) - (locals.var_dmob_dl * locals.var_dvsat_dn8)) / (locals.var_dvsat * locals.var_dvsat)))), (((((locals.var_qim1_dn9 * locals.var_alpha1) - (locals.var_qim1 * locals.var_alpha1_dn9)) / (locals.var_alpha1 * locals.var_alpha1)) * assign24850_e38309) + (assign24850_e38306 * (((locals.var_dmob_dl_dn9 * locals.var_dvsat) - (locals.var_dmob_dl * locals.var_dvsat_dn9)) / (locals.var_dvsat * locals.var_dvsat)))), (((((locals.var_qim1_dn10 * locals.var_alpha1) - (locals.var_qim1 * locals.var_alpha1_dn10)) / (locals.var_alpha1 * locals.var_alpha1)) * assign24850_e38309) + (assign24850_e38306 * (((locals.var_dmob_dl_dn10 * locals.var_dvsat) - (locals.var_dmob_dl * locals.var_dvsat_dn10)) / (locals.var_dvsat * locals.var_dvsat)))), (((((locals.var_qim1_dn11 * locals.var_alpha1) - (locals.var_qim1 * locals.var_alpha1_dn11)) / (locals.var_alpha1 * locals.var_alpha1)) * assign24850_e38309) + (assign24850_e38306 * (((locals.var_dmob_dl_dn11 * locals.var_dvsat) - (locals.var_dmob_dl * locals.var_dvsat_dn11)) / (locals.var_dvsat * locals.var_dvsat)))),)
    } else {
        (locals.var_h_fact, locals.var_h_fact_dn3, locals.var_h_fact_dn4, locals.var_h_fact_dn5, locals.var_h_fact_dn6, locals.var_h_fact_dn7, locals.var_h_fact_dn8, locals.var_h_fact_dn9, locals.var_h_fact_dn10, locals.var_h_fact_dn11,)
    }
};
        locals.var_h_fact = assign24850_e38312;
        locals.var_h_fact_dn3 = assign24850_e38312_d_n3;
        locals.var_h_fact_dn4 = assign24850_e38312_d_n4;
        locals.var_h_fact_dn5 = assign24850_e38312_d_n5;
        locals.var_h_fact_dn6 = assign24850_e38312_d_n6;
        locals.var_h_fact_dn7 = assign24850_e38312_d_n7;
        locals.var_h_fact_dn8 = assign24850_e38312_d_n8;
        locals.var_h_fact_dn9 = assign24850_e38312_d_n9;
        locals.var_h_fact_dn10 = assign24850_e38312_d_n10;
        locals.var_h_fact_dn11 = assign24850_e38312_d_n11;

        let (assign24860_e38342, assign24860_e38342_d_n3, assign24860_e38342_d_n4, assign24860_e38342_d_n5, assign24860_e38342_d_n6, assign24860_e38342_d_n7, assign24860_e38342_d_n8, assign24860_e38342_d_n9, assign24860_e38342_d_n10, assign24860_e38342_d_n11,) = {
    if (locals.var_guard492 != 0.0) {
        let assign24860_e38316: f64 = (p.p2 * locals.var_u0_a);
        let assign24860_e38319: f64 = (locals.var_weff / locals.var_leff);
        let assign24860_e38320: f64 = (assign24860_e38316 * assign24860_e38319);
        let assign24860_e38322: f64 = (assign24860_e38320 * locals.var_cox);
        let assign24860_e38324: f64 = (assign24860_e38322 * locals.var_qim1);
        let assign24860_e38326: f64 = (assign24860_e38324 * locals.var_dps);
        let assign24860_e38329: f64 = (locals.var_ddl * locals.var_dvsatinv);
        let assign24860_e38331: f64 = (assign24860_e38329 / locals.var_dr);
        let assign24860_e38332: f64 = (assign24860_e38326 * assign24860_e38331);
        let assign24860_e38334: f64 = (assign24860_e38332 * locals.var_moc);
        let assign24860_e38336: f64 = (assign24860_e38334 / locals.var_nsat);
        let assign24860_e38338: f64 = (assign24860_e38336 * locals.var_mnud);
        let assign24860_e38340: f64 = (assign24860_e38338 * locals.var_mnud1);
        (assign24860_e38340, ((((((((((((((((((p.p2 * locals.var_u0_a_dn3) * assign24860_e38319) * locals.var_cox) * locals.var_qim1) + (assign24860_e38322 * locals.var_qim1_dn3)) * locals.var_dps) + (assign24860_e38324 * locals.var_dps_dn3)) * assign24860_e38331) + (assign24860_e38326 * (((((locals.var_ddl_dn3 * locals.var_dvsatinv) + (locals.var_ddl * locals.var_dvsatinv_dn3)) * locals.var_dr) - (assign24860_e38329 * locals.var_dr_dn3)) / (locals.var_dr * locals.var_dr)))) * locals.var_moc) + (assign24860_e38332 * locals.var_moc_dn3)) * locals.var_nsat) - (assign24860_e38334 * locals.var_nsat_dn3)) / (locals.var_nsat * locals.var_nsat)) * locals.var_mnud) + (assign24860_e38336 * locals.var_mnud_dn3)) * locals.var_mnud1) + (assign24860_e38338 * locals.var_mnud1_dn3)), ((((((((((((((((((p.p2 * locals.var_u0_a_dn4) * assign24860_e38319) * locals.var_cox) * locals.var_qim1) + (assign24860_e38322 * locals.var_qim1_dn4)) * locals.var_dps) + (assign24860_e38324 * locals.var_dps_dn4)) * assign24860_e38331) + (assign24860_e38326 * (((((locals.var_ddl_dn4 * locals.var_dvsatinv) + (locals.var_ddl * locals.var_dvsatinv_dn4)) * locals.var_dr) - (assign24860_e38329 * locals.var_dr_dn4)) / (locals.var_dr * locals.var_dr)))) * locals.var_moc) + (assign24860_e38332 * locals.var_moc_dn4)) * locals.var_nsat) - (assign24860_e38334 * locals.var_nsat_dn4)) / (locals.var_nsat * locals.var_nsat)) * locals.var_mnud) + (assign24860_e38336 * locals.var_mnud_dn4)) * locals.var_mnud1) + (assign24860_e38338 * locals.var_mnud1_dn4)), ((((((((((((((((((p.p2 * locals.var_u0_a_dn5) * assign24860_e38319) * locals.var_cox) * locals.var_qim1) + (assign24860_e38322 * locals.var_qim1_dn5)) * locals.var_dps) + (assign24860_e38324 * locals.var_dps_dn5)) * assign24860_e38331) + (assign24860_e38326 * (((((locals.var_ddl_dn5 * locals.var_dvsatinv) + (locals.var_ddl * locals.var_dvsatinv_dn5)) * locals.var_dr) - (assign24860_e38329 * locals.var_dr_dn5)) / (locals.var_dr * locals.var_dr)))) * locals.var_moc) + (assign24860_e38332 * locals.var_moc_dn5)) * locals.var_nsat) - (assign24860_e38334 * locals.var_nsat_dn5)) / (locals.var_nsat * locals.var_nsat)) * locals.var_mnud) + (assign24860_e38336 * locals.var_mnud_dn5)) * locals.var_mnud1) + (assign24860_e38338 * locals.var_mnud1_dn5)), ((((((((((((((((((p.p2 * locals.var_u0_a_dn6) * assign24860_e38319) * locals.var_cox) * locals.var_qim1) + (assign24860_e38322 * locals.var_qim1_dn6)) * locals.var_dps) + (assign24860_e38324 * locals.var_dps_dn6)) * assign24860_e38331) + (assign24860_e38326 * (((((locals.var_ddl_dn6 * locals.var_dvsatinv) + (locals.var_ddl * locals.var_dvsatinv_dn6)) * locals.var_dr) - (assign24860_e38329 * locals.var_dr_dn6)) / (locals.var_dr * locals.var_dr)))) * locals.var_moc) + (assign24860_e38332 * locals.var_moc_dn6)) * locals.var_nsat) - (assign24860_e38334 * locals.var_nsat_dn6)) / (locals.var_nsat * locals.var_nsat)) * locals.var_mnud) + (assign24860_e38336 * locals.var_mnud_dn6)) * locals.var_mnud1) + (assign24860_e38338 * locals.var_mnud1_dn6)), ((((((((((((((((((p.p2 * locals.var_u0_a_dn7) * assign24860_e38319) * locals.var_cox) * locals.var_qim1) + (assign24860_e38322 * locals.var_qim1_dn7)) * locals.var_dps) + (assign24860_e38324 * locals.var_dps_dn7)) * assign24860_e38331) + (assign24860_e38326 * (((((locals.var_ddl_dn7 * locals.var_dvsatinv) + (locals.var_ddl * locals.var_dvsatinv_dn7)) * locals.var_dr) - (assign24860_e38329 * locals.var_dr_dn7)) / (locals.var_dr * locals.var_dr)))) * locals.var_moc) + (assign24860_e38332 * locals.var_moc_dn7)) * locals.var_nsat) - (assign24860_e38334 * locals.var_nsat_dn7)) / (locals.var_nsat * locals.var_nsat)) * locals.var_mnud) + (assign24860_e38336 * locals.var_mnud_dn7)) * locals.var_mnud1) + (assign24860_e38338 * locals.var_mnud1_dn7)), ((((((((((((((((((p.p2 * locals.var_u0_a_dn8) * assign24860_e38319) * locals.var_cox) * locals.var_qim1) + (assign24860_e38322 * locals.var_qim1_dn8)) * locals.var_dps) + (assign24860_e38324 * locals.var_dps_dn8)) * assign24860_e38331) + (assign24860_e38326 * (((((locals.var_ddl_dn8 * locals.var_dvsatinv) + (locals.var_ddl * locals.var_dvsatinv_dn8)) * locals.var_dr) - (assign24860_e38329 * locals.var_dr_dn8)) / (locals.var_dr * locals.var_dr)))) * locals.var_moc) + (assign24860_e38332 * locals.var_moc_dn8)) * locals.var_nsat) - (assign24860_e38334 * locals.var_nsat_dn8)) / (locals.var_nsat * locals.var_nsat)) * locals.var_mnud) + (assign24860_e38336 * locals.var_mnud_dn8)) * locals.var_mnud1) + (assign24860_e38338 * locals.var_mnud1_dn8)), ((((((((((((((((((p.p2 * locals.var_u0_a_dn9) * assign24860_e38319) * locals.var_cox) * locals.var_qim1) + (assign24860_e38322 * locals.var_qim1_dn9)) * locals.var_dps) + (assign24860_e38324 * locals.var_dps_dn9)) * assign24860_e38331) + (assign24860_e38326 * (((((locals.var_ddl_dn9 * locals.var_dvsatinv) + (locals.var_ddl * locals.var_dvsatinv_dn9)) * locals.var_dr) - (assign24860_e38329 * locals.var_dr_dn9)) / (locals.var_dr * locals.var_dr)))) * locals.var_moc) + (assign24860_e38332 * locals.var_moc_dn9)) * locals.var_nsat) - (assign24860_e38334 * locals.var_nsat_dn9)) / (locals.var_nsat * locals.var_nsat)) * locals.var_mnud) + (assign24860_e38336 * locals.var_mnud_dn9)) * locals.var_mnud1) + (assign24860_e38338 * locals.var_mnud1_dn9)), ((((((((((((((((((p.p2 * locals.var_u0_a_dn10) * assign24860_e38319) * locals.var_cox) * locals.var_qim1) + (assign24860_e38322 * locals.var_qim1_dn10)) * locals.var_dps) + (assign24860_e38324 * locals.var_dps_dn10)) * assign24860_e38331) + (assign24860_e38326 * (((((locals.var_ddl_dn10 * locals.var_dvsatinv) + (locals.var_ddl * locals.var_dvsatinv_dn10)) * locals.var_dr) - (assign24860_e38329 * locals.var_dr_dn10)) / (locals.var_dr * locals.var_dr)))) * locals.var_moc) + (assign24860_e38332 * locals.var_moc_dn10)) * locals.var_nsat) - (assign24860_e38334 * locals.var_nsat_dn10)) / (locals.var_nsat * locals.var_nsat)) * locals.var_mnud) + (assign24860_e38336 * locals.var_mnud_dn10)) * locals.var_mnud1) + (assign24860_e38338 * locals.var_mnud1_dn10)), ((((((((((((((((((p.p2 * locals.var_u0_a_dn11) * assign24860_e38319) * locals.var_cox) * locals.var_qim1) + (assign24860_e38322 * locals.var_qim1_dn11)) * locals.var_dps) + (assign24860_e38324 * locals.var_dps_dn11)) * assign24860_e38331) + (assign24860_e38326 * (((((locals.var_ddl_dn11 * locals.var_dvsatinv) + (locals.var_ddl * locals.var_dvsatinv_dn11)) * locals.var_dr) - (assign24860_e38329 * locals.var_dr_dn11)) / (locals.var_dr * locals.var_dr)))) * locals.var_moc) + (assign24860_e38332 * locals.var_moc_dn11)) * locals.var_nsat) - (assign24860_e38334 * locals.var_nsat_dn11)) / (locals.var_nsat * locals.var_nsat)) * locals.var_mnud) + (assign24860_e38336 * locals.var_mnud_dn11)) * locals.var_mnud1) + (assign24860_e38338 * locals.var_mnud1_dn11)),)
    } else {
        (locals.var_ids, locals.var_ids_dn3, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn11,)
    }
};
        locals.var_ids = assign24860_e38342;
        locals.var_ids_dn3 = assign24860_e38342_d_n3;
        locals.var_ids_dn4 = assign24860_e38342_d_n4;
        locals.var_ids_dn5 = assign24860_e38342_d_n5;
        locals.var_ids_dn6 = assign24860_e38342_d_n6;
        locals.var_ids_dn7 = assign24860_e38342_d_n7;
        locals.var_ids_dn8 = assign24860_e38342_d_n8;
        locals.var_ids_dn9 = assign24860_e38342_d_n9;
        locals.var_ids_dn10 = assign24860_e38342_d_n10;
        locals.var_ids_dn11 = assign24860_e38342_d_n11;

        let (assign24870_e38350, assign24870_e38350_d_n3, assign24870_e38350_d_n4, assign24870_e38350_d_n5, assign24870_e38350_d_n6, assign24870_e38350_d_n7, assign24870_e38350_d_n8, assign24870_e38350_d_n9, assign24870_e38350_d_n10, assign24870_e38350_d_n11,) = {
    if (locals.var_guard492 != 0.0) {
        let assign24870_e38346: f64 = (locals.var_dmob * locals.var_dvsat);
        let assign24870_e38348: f64 = (assign24870_e38346 * locals.var_dr);
        (assign24870_e38348, ((((locals.var_dmob_dn3 * locals.var_dvsat) + (locals.var_dmob * locals.var_dvsat_dn3)) * locals.var_dr) + (assign24870_e38346 * locals.var_dr_dn3)), ((((locals.var_dmob_dn4 * locals.var_dvsat) + (locals.var_dmob * locals.var_dvsat_dn4)) * locals.var_dr) + (assign24870_e38346 * locals.var_dr_dn4)), ((((locals.var_dmob_dn5 * locals.var_dvsat) + (locals.var_dmob * locals.var_dvsat_dn5)) * locals.var_dr) + (assign24870_e38346 * locals.var_dr_dn5)), ((((locals.var_dmob_dn6 * locals.var_dvsat) + (locals.var_dmob * locals.var_dvsat_dn6)) * locals.var_dr) + (assign24870_e38346 * locals.var_dr_dn6)), ((((locals.var_dmob_dn7 * locals.var_dvsat) + (locals.var_dmob * locals.var_dvsat_dn7)) * locals.var_dr) + (assign24870_e38346 * locals.var_dr_dn7)), ((((locals.var_dmob_dn8 * locals.var_dvsat) + (locals.var_dmob * locals.var_dvsat_dn8)) * locals.var_dr) + (assign24870_e38346 * locals.var_dr_dn8)), ((((locals.var_dmob_dn9 * locals.var_dvsat) + (locals.var_dmob * locals.var_dvsat_dn9)) * locals.var_dr) + (assign24870_e38346 * locals.var_dr_dn9)), ((((locals.var_dmob_dn10 * locals.var_dvsat) + (locals.var_dmob * locals.var_dvsat_dn10)) * locals.var_dr) + (assign24870_e38346 * locals.var_dr_dn10)), ((((locals.var_dmob_dn11 * locals.var_dvsat) + (locals.var_dmob * locals.var_dvsat_dn11)) * locals.var_dr) + (assign24870_e38346 * locals.var_dr_dn11)),)
    } else {
        (locals.var_dtot, locals.var_dtot_dn3, locals.var_dtot_dn4, locals.var_dtot_dn5, locals.var_dtot_dn6, locals.var_dtot_dn7, locals.var_dtot_dn8, locals.var_dtot_dn9, locals.var_dtot_dn10, locals.var_dtot_dn11,)
    }
};
        locals.var_dtot = assign24870_e38350;
        locals.var_dtot_dn3 = assign24870_e38350_d_n3;
        locals.var_dtot_dn4 = assign24870_e38350_d_n4;
        locals.var_dtot_dn5 = assign24870_e38350_d_n5;
        locals.var_dtot_dn6 = assign24870_e38350_d_n6;
        locals.var_dtot_dn7 = assign24870_e38350_d_n7;
        locals.var_dtot_dn8 = assign24870_e38350_d_n8;
        locals.var_dtot_dn9 = assign24870_e38350_d_n9;
        locals.var_dtot_dn10 = assign24870_e38350_d_n10;
        locals.var_dtot_dn11 = assign24870_e38350_d_n11;

        let (assign24880_e38356, assign24880_e38356_d_n3, assign24880_e38356_d_n4, assign24880_e38356_d_n5, assign24880_e38356_d_n6, assign24880_e38356_d_n7, assign24880_e38356_d_n8, assign24880_e38356_d_n9, assign24880_e38356_d_n10, assign24880_e38356_d_n11,) = {
    if (locals.var_guard492 != 0.0) {
        let assign24880_e38354: f64 = (locals.var_u0_a / locals.var_dtot);
        (assign24880_e38354, (((locals.var_u0_a_dn3 * locals.var_dtot) - (locals.var_u0_a * locals.var_dtot_dn3)) / (locals.var_dtot * locals.var_dtot)), (((locals.var_u0_a_dn4 * locals.var_dtot) - (locals.var_u0_a * locals.var_dtot_dn4)) / (locals.var_dtot * locals.var_dtot)), (((locals.var_u0_a_dn5 * locals.var_dtot) - (locals.var_u0_a * locals.var_dtot_dn5)) / (locals.var_dtot * locals.var_dtot)), (((locals.var_u0_a_dn6 * locals.var_dtot) - (locals.var_u0_a * locals.var_dtot_dn6)) / (locals.var_dtot * locals.var_dtot)), (((locals.var_u0_a_dn7 * locals.var_dtot) - (locals.var_u0_a * locals.var_dtot_dn7)) / (locals.var_dtot * locals.var_dtot)), (((locals.var_u0_a_dn8 * locals.var_dtot) - (locals.var_u0_a * locals.var_dtot_dn8)) / (locals.var_dtot * locals.var_dtot)), (((locals.var_u0_a_dn9 * locals.var_dtot) - (locals.var_u0_a * locals.var_dtot_dn9)) / (locals.var_dtot * locals.var_dtot)), (((locals.var_u0_a_dn10 * locals.var_dtot) - (locals.var_u0_a * locals.var_dtot_dn10)) / (locals.var_dtot * locals.var_dtot)), (((locals.var_u0_a_dn11 * locals.var_dtot) - (locals.var_u0_a * locals.var_dtot_dn11)) / (locals.var_dtot * locals.var_dtot)),)
    } else {
        (locals.var_ueff, locals.var_ueff_dn3, locals.var_ueff_dn4, locals.var_ueff_dn5, locals.var_ueff_dn6, locals.var_ueff_dn7, locals.var_ueff_dn8, locals.var_ueff_dn9, locals.var_ueff_dn10, locals.var_ueff_dn11,)
    }
};
        locals.var_ueff = assign24880_e38356;
        locals.var_ueff_dn3 = assign24880_e38356_d_n3;
        locals.var_ueff_dn4 = assign24880_e38356_d_n4;
        locals.var_ueff_dn5 = assign24880_e38356_d_n5;
        locals.var_ueff_dn6 = assign24880_e38356_d_n6;
        locals.var_ueff_dn7 = assign24880_e38356_d_n7;
        locals.var_ueff_dn8 = assign24880_e38356_d_n8;
        locals.var_ueff_dn9 = assign24880_e38356_d_n9;
        locals.var_ueff_dn10 = assign24880_e38356_d_n10;
        locals.var_ueff_dn11 = assign24880_e38356_d_n11;

        let (assign24890_e38360, assign24890_e38360_d_n3, assign24890_e38360_d_n4, assign24890_e38360_d_n5, assign24890_e38360_d_n6, assign24890_e38360_d_n7, assign24890_e38360_d_n8, assign24890_e38360_d_n9, assign24890_e38360_d_n10, assign24890_e38360_d_n11,) = {
    if (locals.var_guard492 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_gcrg, locals.var_gcrg_dn3, locals.var_gcrg_dn4, locals.var_gcrg_dn5, locals.var_gcrg_dn6, locals.var_gcrg_dn7, locals.var_gcrg_dn8, locals.var_gcrg_dn9, locals.var_gcrg_dn10, locals.var_gcrg_dn11,)
    }
};
        locals.var_gcrg = assign24890_e38360;
        locals.var_gcrg_dn3 = assign24890_e38360_d_n3;
        locals.var_gcrg_dn4 = assign24890_e38360_d_n4;
        locals.var_gcrg_dn5 = assign24890_e38360_d_n5;
        locals.var_gcrg_dn6 = assign24890_e38360_d_n6;
        locals.var_gcrg_dn7 = assign24890_e38360_d_n7;
        locals.var_gcrg_dn8 = assign24890_e38360_d_n8;
        locals.var_gcrg_dn9 = assign24890_e38360_d_n9;
        locals.var_gcrg_dn10 = assign24890_e38360_d_n10;
        locals.var_gcrg_dn11 = assign24890_e38360_d_n11;

        let assign24900_e38363: f64 = if p.p7 > 1.0 { 1.0 } else { 0.0 };
        locals.var_guard539 = assign24900_e38363;

        let (assign24910_e38377, assign24910_e38377_d_n3, assign24910_e38377_d_n4, assign24910_e38377_d_n5, assign24910_e38377_d_n6, assign24910_e38377_d_n7, assign24910_e38377_d_n8, assign24910_e38377_d_n9, assign24910_e38377_d_n10, assign24910_e38377_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard539 != 0.0)) {
        let assign24910_e38369: f64 = (locals.var_ueff * locals.var_weff);
        let assign24910_e38371: f64 = (assign24910_e38369 / locals.var_leff);
        let assign24910_e38373: f64 = (assign24910_e38371 * locals.var_cox);
        let assign24910_e38375: f64 = (assign24910_e38373 * locals.var_qia);
        (assign24910_e38375, (((((locals.var_ueff_dn3 * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_qia) + (assign24910_e38373 * locals.var_qia_dn3)), (((((locals.var_ueff_dn4 * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_qia) + (assign24910_e38373 * locals.var_qia_dn4)), (((((locals.var_ueff_dn5 * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_qia) + (assign24910_e38373 * locals.var_qia_dn5)), (((((locals.var_ueff_dn6 * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_qia) + (assign24910_e38373 * locals.var_qia_dn6)), (((((locals.var_ueff_dn7 * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_qia) + (assign24910_e38373 * locals.var_qia_dn7)), (((((locals.var_ueff_dn8 * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_qia) + (assign24910_e38373 * locals.var_qia_dn8)), (((((locals.var_ueff_dn9 * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_qia) + (assign24910_e38373 * locals.var_qia_dn9)), (((((locals.var_ueff_dn10 * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_qia) + (assign24910_e38373 * locals.var_qia_dn10)), (((((locals.var_ueff_dn11 * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_qia) + (assign24910_e38373 * locals.var_qia_dn11)),)
    } else {
        (locals.var_idsovvds, locals.var_idsovvds_dn3, locals.var_idsovvds_dn4, locals.var_idsovvds_dn5, locals.var_idsovvds_dn6, locals.var_idsovvds_dn7, locals.var_idsovvds_dn8, locals.var_idsovvds_dn9, locals.var_idsovvds_dn10, locals.var_idsovvds_dn11,)
    }
};
        locals.var_idsovvds = assign24910_e38377;
        locals.var_idsovvds_dn3 = assign24910_e38377_d_n3;
        locals.var_idsovvds_dn4 = assign24910_e38377_d_n4;
        locals.var_idsovvds_dn5 = assign24910_e38377_d_n5;
        locals.var_idsovvds_dn6 = assign24910_e38377_d_n6;
        locals.var_idsovvds_dn7 = assign24910_e38377_d_n7;
        locals.var_idsovvds_dn8 = assign24910_e38377_d_n8;
        locals.var_idsovvds_dn9 = assign24910_e38377_d_n9;
        locals.var_idsovvds_dn10 = assign24910_e38377_d_n10;
        locals.var_idsovvds_dn11 = assign24910_e38377_d_n11;

        let (assign24920_e38385, assign24920_e38385_d_n3, assign24920_e38385_d_n4, assign24920_e38385_d_n5, assign24920_e38385_d_n6, assign24920_e38385_d_n7, assign24920_e38385_d_n8, assign24920_e38385_d_n9, assign24920_e38385_d_n10, assign24920_e38385_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard539 != 0.0)) {
        let assign24920_e38383: f64 = (p.p1009 * locals.var_vt);
        (assign24920_e38383, 0.0, (p.p1009 * locals.var_vt_dn4), (p.p1009 * locals.var_vt_dn5), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn3, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11,)
    }
};
        locals.var_t9 = assign24920_e38385;
        locals.var_t9_dn3 = assign24920_e38385_d_n3;
        locals.var_t9_dn4 = assign24920_e38385_d_n4;
        locals.var_t9_dn5 = assign24920_e38385_d_n5;
        locals.var_t9_dn6 = assign24920_e38385_d_n6;
        locals.var_t9_dn7 = assign24920_e38385_d_n7;
        locals.var_t9_dn8 = assign24920_e38385_d_n8;
        locals.var_t9_dn9 = assign24920_e38385_d_n9;
        locals.var_t9_dn10 = assign24920_e38385_d_n10;
        locals.var_t9_dn11 = assign24920_e38385_d_n11;

        let (assign24930_e38399, assign24930_e38399_d_n3, assign24930_e38399_d_n4, assign24930_e38399_d_n5, assign24930_e38399_d_n6, assign24930_e38399_d_n7, assign24930_e38399_d_n8, assign24930_e38399_d_n9, assign24930_e38399_d_n10, assign24930_e38399_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard539 != 0.0)) {
        let assign24930_e38391: f64 = (locals.var_t9 * locals.var_ueff);
        let assign24930_e38393: f64 = (assign24930_e38391 * locals.var_weff);
        let assign24930_e38395: f64 = (assign24930_e38393 / locals.var_leff);
        let assign24930_e38397: f64 = (assign24930_e38395 * locals.var_cox);
        (assign24930_e38397, (((((locals.var_t9_dn3 * locals.var_ueff) + (locals.var_t9 * locals.var_ueff_dn3)) * locals.var_weff) / locals.var_leff) * locals.var_cox), (((((locals.var_t9_dn4 * locals.var_ueff) + (locals.var_t9 * locals.var_ueff_dn4)) * locals.var_weff) / locals.var_leff) * locals.var_cox), (((((locals.var_t9_dn5 * locals.var_ueff) + (locals.var_t9 * locals.var_ueff_dn5)) * locals.var_weff) / locals.var_leff) * locals.var_cox), (((((locals.var_t9_dn6 * locals.var_ueff) + (locals.var_t9 * locals.var_ueff_dn6)) * locals.var_weff) / locals.var_leff) * locals.var_cox), (((((locals.var_t9_dn7 * locals.var_ueff) + (locals.var_t9 * locals.var_ueff_dn7)) * locals.var_weff) / locals.var_leff) * locals.var_cox), (((((locals.var_t9_dn8 * locals.var_ueff) + (locals.var_t9 * locals.var_ueff_dn8)) * locals.var_weff) / locals.var_leff) * locals.var_cox), (((((locals.var_t9_dn9 * locals.var_ueff) + (locals.var_t9 * locals.var_ueff_dn9)) * locals.var_weff) / locals.var_leff) * locals.var_cox), (((((locals.var_t9_dn10 * locals.var_ueff) + (locals.var_t9 * locals.var_ueff_dn10)) * locals.var_weff) / locals.var_leff) * locals.var_cox), (((((locals.var_t9_dn11 * locals.var_ueff) + (locals.var_t9 * locals.var_ueff_dn11)) * locals.var_weff) / locals.var_leff) * locals.var_cox),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign24930_e38399;
        locals.var_t0_dn3 = assign24930_e38399_d_n3;
        locals.var_t0_dn4 = assign24930_e38399_d_n4;
        locals.var_t0_dn5 = assign24930_e38399_d_n5;
        locals.var_t0_dn6 = assign24930_e38399_d_n6;
        locals.var_t0_dn7 = assign24930_e38399_d_n7;
        locals.var_t0_dn8 = assign24930_e38399_d_n8;
        locals.var_t0_dn9 = assign24930_e38399_d_n9;
        locals.var_t0_dn10 = assign24930_e38399_d_n10;
        locals.var_t0_dn11 = assign24930_e38399_d_n11;

        let (assign24940_e38411, assign24940_e38411_d_n3, assign24940_e38411_d_n4, assign24940_e38411_d_n5, assign24940_e38411_d_n6, assign24940_e38411_d_n7, assign24940_e38411_d_n8, assign24940_e38411_d_n9, assign24940_e38411_d_n10, assign24940_e38411_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard539 != 0.0)) {
        let assign24940_e38405: f64 = (p.p1008 * p.p2);
        let assign24940_e38408: f64 = (locals.var_t0 + locals.var_idsovvds);
        let assign24940_e38409: f64 = (assign24940_e38405 * assign24940_e38408);
        (assign24940_e38409, (assign24940_e38405 * (locals.var_t0_dn3 + locals.var_idsovvds_dn3)), (assign24940_e38405 * (locals.var_t0_dn4 + locals.var_idsovvds_dn4)), (assign24940_e38405 * (locals.var_t0_dn5 + locals.var_idsovvds_dn5)), (assign24940_e38405 * (locals.var_t0_dn6 + locals.var_idsovvds_dn6)), (assign24940_e38405 * (locals.var_t0_dn7 + locals.var_idsovvds_dn7)), (assign24940_e38405 * (locals.var_t0_dn8 + locals.var_idsovvds_dn8)), (assign24940_e38405 * (locals.var_t0_dn9 + locals.var_idsovvds_dn9)), (assign24940_e38405 * (locals.var_t0_dn10 + locals.var_idsovvds_dn10)), (assign24940_e38405 * (locals.var_t0_dn11 + locals.var_idsovvds_dn11)),)
    } else {
        (locals.var_gcrg, locals.var_gcrg_dn3, locals.var_gcrg_dn4, locals.var_gcrg_dn5, locals.var_gcrg_dn6, locals.var_gcrg_dn7, locals.var_gcrg_dn8, locals.var_gcrg_dn9, locals.var_gcrg_dn10, locals.var_gcrg_dn11,)
    }
};
        locals.var_gcrg = assign24940_e38411;
        locals.var_gcrg_dn3 = assign24940_e38411_d_n3;
        locals.var_gcrg_dn4 = assign24940_e38411_d_n4;
        locals.var_gcrg_dn5 = assign24940_e38411_d_n5;
        locals.var_gcrg_dn6 = assign24940_e38411_d_n6;
        locals.var_gcrg_dn7 = assign24940_e38411_d_n7;
        locals.var_gcrg_dn8 = assign24940_e38411_d_n8;
        locals.var_gcrg_dn9 = assign24940_e38411_d_n9;
        locals.var_gcrg_dn10 = assign24940_e38411_d_n10;
        locals.var_gcrg_dn11 = assign24940_e38411_d_n11;

        let assign24950_e38414: f64 = if p.p7 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard540 = assign24950_e38414;

        let (assign24960_e38424,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard539 != 0.0)) && (locals.var_guard540 != 0.0)) {
        let assign24960_e38422: f64 = (1.0 / locals.var_grgeltd);
        (assign24960_e38422,)
    } else {
        (locals.var_rgeltd,)
    }
};
        locals.var_rgeltd = assign24960_e38424;

        let assign24970_e38427: f64 = if locals.var_rgeltd < p.p1347 { 1.0 } else { 0.0 };
        locals.var_guard541 = assign24970_e38427;

        let (assign24980_e38437,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard539 != 0.0)) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 != 0.0)) {
        (p.p1347,)
    } else {
        (locals.var_rgeltd,)
    }
};
        locals.var_rgeltd = assign24980_e38437;

        let (assign24990_e38449,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard539 != 0.0)) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 != 0.0)) {
        let assign24990_e38447: f64 = (1.0 / locals.var_rgeltd);
        (assign24990_e38447,)
    } else {
        (locals.var_grgeltd,)
    }
};
        locals.var_grgeltd = assign24990_e38449;

        let (assign25000_e38459, assign25000_e38459_d_n3, assign25000_e38459_d_n4, assign25000_e38459_d_n5, assign25000_e38459_d_n6, assign25000_e38459_d_n7, assign25000_e38459_d_n8, assign25000_e38459_d_n9, assign25000_e38459_d_n10, assign25000_e38459_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard539 != 0.0)) && (locals.var_guard540 != 0.0)) {
        let assign25000_e38457: f64 = (locals.var_grgeltd + locals.var_gcrg);
        (assign25000_e38457, locals.var_gcrg_dn3, locals.var_gcrg_dn4, locals.var_gcrg_dn5, locals.var_gcrg_dn6, locals.var_gcrg_dn7, locals.var_gcrg_dn8, locals.var_gcrg_dn9, locals.var_gcrg_dn10, locals.var_gcrg_dn11,)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign25000_e38459;
        locals.var_t11_dn3 = assign25000_e38459_d_n3;
        locals.var_t11_dn4 = assign25000_e38459_d_n4;
        locals.var_t11_dn5 = assign25000_e38459_d_n5;
        locals.var_t11_dn6 = assign25000_e38459_d_n6;
        locals.var_t11_dn7 = assign25000_e38459_d_n7;
        locals.var_t11_dn8 = assign25000_e38459_d_n8;
        locals.var_t11_dn9 = assign25000_e38459_d_n9;
        locals.var_t11_dn10 = assign25000_e38459_d_n10;
        locals.var_t11_dn11 = assign25000_e38459_d_n11;

        let (assign25010_e38471, assign25010_e38471_d_n3, assign25010_e38471_d_n4, assign25010_e38471_d_n5, assign25010_e38471_d_n6, assign25010_e38471_d_n7, assign25010_e38471_d_n8, assign25010_e38471_d_n9, assign25010_e38471_d_n10, assign25010_e38471_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard539 != 0.0)) && (locals.var_guard540 != 0.0)) {
        let assign25010_e38467: f64 = (locals.var_grgeltd * locals.var_gcrg);
        let assign25010_e38469: f64 = (assign25010_e38467 / locals.var_t11);
        (assign25010_e38469, ((((locals.var_grgeltd * locals.var_gcrg_dn3) * locals.var_t11) - (assign25010_e38467 * locals.var_t11_dn3)) / (locals.var_t11 * locals.var_t11)), ((((locals.var_grgeltd * locals.var_gcrg_dn4) * locals.var_t11) - (assign25010_e38467 * locals.var_t11_dn4)) / (locals.var_t11 * locals.var_t11)), ((((locals.var_grgeltd * locals.var_gcrg_dn5) * locals.var_t11) - (assign25010_e38467 * locals.var_t11_dn5)) / (locals.var_t11 * locals.var_t11)), ((((locals.var_grgeltd * locals.var_gcrg_dn6) * locals.var_t11) - (assign25010_e38467 * locals.var_t11_dn6)) / (locals.var_t11 * locals.var_t11)), ((((locals.var_grgeltd * locals.var_gcrg_dn7) * locals.var_t11) - (assign25010_e38467 * locals.var_t11_dn7)) / (locals.var_t11 * locals.var_t11)), ((((locals.var_grgeltd * locals.var_gcrg_dn8) * locals.var_t11) - (assign25010_e38467 * locals.var_t11_dn8)) / (locals.var_t11 * locals.var_t11)), ((((locals.var_grgeltd * locals.var_gcrg_dn9) * locals.var_t11) - (assign25010_e38467 * locals.var_t11_dn9)) / (locals.var_t11 * locals.var_t11)), ((((locals.var_grgeltd * locals.var_gcrg_dn10) * locals.var_t11) - (assign25010_e38467 * locals.var_t11_dn10)) / (locals.var_t11 * locals.var_t11)), ((((locals.var_grgeltd * locals.var_gcrg_dn11) * locals.var_t11) - (assign25010_e38467 * locals.var_t11_dn11)) / (locals.var_t11 * locals.var_t11)),)
    } else {
        (locals.var_gcrg, locals.var_gcrg_dn3, locals.var_gcrg_dn4, locals.var_gcrg_dn5, locals.var_gcrg_dn6, locals.var_gcrg_dn7, locals.var_gcrg_dn8, locals.var_gcrg_dn9, locals.var_gcrg_dn10, locals.var_gcrg_dn11,)
    }
};
        locals.var_gcrg = assign25010_e38471;
        locals.var_gcrg_dn3 = assign25010_e38471_d_n3;
        locals.var_gcrg_dn4 = assign25010_e38471_d_n4;
        locals.var_gcrg_dn5 = assign25010_e38471_d_n5;
        locals.var_gcrg_dn6 = assign25010_e38471_d_n6;
        locals.var_gcrg_dn7 = assign25010_e38471_d_n7;
        locals.var_gcrg_dn8 = assign25010_e38471_d_n8;
        locals.var_gcrg_dn9 = assign25010_e38471_d_n9;
        locals.var_gcrg_dn10 = assign25010_e38471_d_n10;
        locals.var_gcrg_dn11 = assign25010_e38471_d_n11;

        let (assign25020_e38479,) = {
    if (locals.var_guard492 != 0.0) {
        let assign25020_e38475: f64 = (locals.var_weff / p.p1373);
        let assign25020_e38477: f64 = (assign25020_e38475 + p.p1377);
        (assign25020_e38477,)
    } else {
        (locals.var_wdiod,)
    }
};
        locals.var_wdiod = assign25020_e38479;

        let (assign25030_e38487,) = {
    if (locals.var_guard492 != 0.0) {
        let assign25030_e38483: f64 = (locals.var_weff / p.p1373);
        let assign25030_e38485: f64 = (assign25030_e38483 + p.p1378);
        (assign25030_e38485,)
    } else {
        (locals.var_wdios,)
    }
};
        locals.var_wdios = assign25030_e38487;

        let (assign25040_e38493,) = {
    if (locals.var_guard492 != 0.0) {
        let assign25040_e38491: f64 = (locals.var_wdios * p.p74);
        (assign25040_e38491,)
    } else {
        (locals.var_wstsi,)
    }
};
        locals.var_wstsi = assign25040_e38493;

        let (assign25050_e38499,) = {
    if (locals.var_guard492 != 0.0) {
        let assign25050_e38497: f64 = (locals.var_wdiod * p.p74);
        (assign25050_e38497,)
    } else {
        (locals.var_wdtsi,)
    }
};
        locals.var_wdtsi = assign25050_e38499;

        let (assign25060_e38505, assign25060_e38505_d_n4, assign25060_e38505_d_n5,) = {
    if (locals.var_guard492 != 0.0) {
        let assign25060_e38503: f64 = (locals.var_vtm * locals.var_ndiode_i);
        (assign25060_e38503, (locals.var_vtm_dn4 * locals.var_ndiode_i), (locals.var_vtm_dn5 * locals.var_ndiode_i),)
    } else {
        (locals.var_nvtm1, locals.var_nvtm1_dn4, locals.var_nvtm1_dn5,)
    }
};
        locals.var_nvtm1 = assign25060_e38505;
        locals.var_nvtm1_dn4 = assign25060_e38505_d_n4;
        locals.var_nvtm1_dn5 = assign25060_e38505_d_n5;

        let (assign25070_e38511, assign25070_e38511_d_n3, assign25070_e38511_d_n4, assign25070_e38511_d_n5, assign25070_e38511_d_n6, assign25070_e38511_d_n7, assign25070_e38511_d_n8, assign25070_e38511_d_n9, assign25070_e38511_d_n10, assign25070_e38511_d_n11,) = {
    if (locals.var_guard492 != 0.0) {
        let assign25070_e38509: f64 = (locals.var_vbs_jct / locals.var_nvtm1);
        (assign25070_e38509, 0.0, (-((locals.var_vbs_jct * locals.var_nvtm1_dn4) / (locals.var_nvtm1 * locals.var_nvtm1))), (-((locals.var_vbs_jct * locals.var_nvtm1_dn5) / (locals.var_nvtm1 * locals.var_nvtm1))), 0.0, (locals.var_vbs_jct_dn7 / locals.var_nvtm1), 0.0, 0.0, (locals.var_vbs_jct_dn10 / locals.var_nvtm1), 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign25070_e38511;
        locals.var_t0_dn3 = assign25070_e38511_d_n3;
        locals.var_t0_dn4 = assign25070_e38511_d_n4;
        locals.var_t0_dn5 = assign25070_e38511_d_n5;
        locals.var_t0_dn6 = assign25070_e38511_d_n6;
        locals.var_t0_dn7 = assign25070_e38511_d_n7;
        locals.var_t0_dn8 = assign25070_e38511_d_n8;
        locals.var_t0_dn9 = assign25070_e38511_d_n9;
        locals.var_t0_dn10 = assign25070_e38511_d_n10;
        locals.var_t0_dn11 = assign25070_e38511_d_n11;

        let (assign25080_e38516, assign25080_e38516_d_n3, assign25080_e38516_d_n4, assign25080_e38516_d_n5, assign25080_e38516_d_n6, assign25080_e38516_d_n7, assign25080_e38516_d_n8, assign25080_e38516_d_n9, assign25080_e38516_d_n10, assign25080_e38516_d_n11,) = {
    if (locals.var_guard492 != 0.0) {
        let assign25080_e38514: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign25080_e38514, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_expvbsnvtm, locals.var_expvbsnvtm_dn3, locals.var_expvbsnvtm_dn4, locals.var_expvbsnvtm_dn5, locals.var_expvbsnvtm_dn6, locals.var_expvbsnvtm_dn7, locals.var_expvbsnvtm_dn8, locals.var_expvbsnvtm_dn9, locals.var_expvbsnvtm_dn10, locals.var_expvbsnvtm_dn11,)
    }
};
        locals.var_expvbsnvtm = assign25080_e38516;
        locals.var_expvbsnvtm_dn3 = assign25080_e38516_d_n3;
        locals.var_expvbsnvtm_dn4 = assign25080_e38516_d_n4;
        locals.var_expvbsnvtm_dn5 = assign25080_e38516_d_n5;
        locals.var_expvbsnvtm_dn6 = assign25080_e38516_d_n6;
        locals.var_expvbsnvtm_dn7 = assign25080_e38516_d_n7;
        locals.var_expvbsnvtm_dn8 = assign25080_e38516_d_n8;
        locals.var_expvbsnvtm_dn9 = assign25080_e38516_d_n9;
        locals.var_expvbsnvtm_dn10 = assign25080_e38516_d_n10;
        locals.var_expvbsnvtm_dn11 = assign25080_e38516_d_n11;

        let (assign25090_e38522, assign25090_e38522_d_n4, assign25090_e38522_d_n5,) = {
    if (locals.var_guard492 != 0.0) {
        let assign25090_e38520: f64 = (locals.var_vtm * locals.var_ndiode_i);
        (assign25090_e38520, (locals.var_vtm_dn4 * locals.var_ndiode_i), (locals.var_vtm_dn5 * locals.var_ndiode_i),)
    } else {
        (locals.var_nvtm2, locals.var_nvtm2_dn4, locals.var_nvtm2_dn5,)
    }
};
        locals.var_nvtm2 = assign25090_e38522;
        locals.var_nvtm2_dn4 = assign25090_e38522_d_n4;
        locals.var_nvtm2_dn5 = assign25090_e38522_d_n5;

    }

    pub(super) fn stamp_transient_block_66(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign25100_e38528, assign25100_e38528_d_n3, assign25100_e38528_d_n4, assign25100_e38528_d_n5, assign25100_e38528_d_n6, assign25100_e38528_d_n7, assign25100_e38528_d_n8, assign25100_e38528_d_n9, assign25100_e38528_d_n10, assign25100_e38528_d_n11,) = {
    if (locals.var_guard492 != 0.0) {
        let assign25100_e38526: f64 = (locals.var_vbd_jct / locals.var_nvtm2);
        (assign25100_e38526, 0.0, (-((locals.var_vbd_jct * locals.var_nvtm2_dn4) / (locals.var_nvtm2 * locals.var_nvtm2))), (-((locals.var_vbd_jct * locals.var_nvtm2_dn5) / (locals.var_nvtm2 * locals.var_nvtm2))), (locals.var_vbd_jct_dn6 / locals.var_nvtm2), 0.0, 0.0, 0.0, (locals.var_vbd_jct_dn10 / locals.var_nvtm2), 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign25100_e38528;
        locals.var_t0_dn3 = assign25100_e38528_d_n3;
        locals.var_t0_dn4 = assign25100_e38528_d_n4;
        locals.var_t0_dn5 = assign25100_e38528_d_n5;
        locals.var_t0_dn6 = assign25100_e38528_d_n6;
        locals.var_t0_dn7 = assign25100_e38528_d_n7;
        locals.var_t0_dn8 = assign25100_e38528_d_n8;
        locals.var_t0_dn9 = assign25100_e38528_d_n9;
        locals.var_t0_dn10 = assign25100_e38528_d_n10;
        locals.var_t0_dn11 = assign25100_e38528_d_n11;

        let (assign25110_e38533, assign25110_e38533_d_n3, assign25110_e38533_d_n4, assign25110_e38533_d_n5, assign25110_e38533_d_n6, assign25110_e38533_d_n7, assign25110_e38533_d_n8, assign25110_e38533_d_n9, assign25110_e38533_d_n10, assign25110_e38533_d_n11,) = {
    if (locals.var_guard492 != 0.0) {
        let assign25110_e38531: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign25110_e38531, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_expvbdnvtm, locals.var_expvbdnvtm_dn3, locals.var_expvbdnvtm_dn4, locals.var_expvbdnvtm_dn5, locals.var_expvbdnvtm_dn6, locals.var_expvbdnvtm_dn7, locals.var_expvbdnvtm_dn8, locals.var_expvbdnvtm_dn9, locals.var_expvbdnvtm_dn10, locals.var_expvbdnvtm_dn11,)
    }
};
        locals.var_expvbdnvtm = assign25110_e38533;
        locals.var_expvbdnvtm_dn3 = assign25110_e38533_d_n3;
        locals.var_expvbdnvtm_dn4 = assign25110_e38533_d_n4;
        locals.var_expvbdnvtm_dn5 = assign25110_e38533_d_n5;
        locals.var_expvbdnvtm_dn6 = assign25110_e38533_d_n6;
        locals.var_expvbdnvtm_dn7 = assign25110_e38533_d_n7;
        locals.var_expvbdnvtm_dn8 = assign25110_e38533_d_n8;
        locals.var_expvbdnvtm_dn9 = assign25110_e38533_d_n9;
        locals.var_expvbdnvtm_dn10 = assign25110_e38533_d_n10;
        locals.var_expvbdnvtm_dn11 = assign25110_e38533_d_n11;

        let (assign25120_e38543, assign25120_e38543_d_n3, assign25120_e38543_d_n4, assign25120_e38543_d_n5, assign25120_e38543_d_n6, assign25120_e38543_d_n7, assign25120_e38543_d_n8, assign25120_e38543_d_n9, assign25120_e38543_d_n10, assign25120_e38543_d_n11,) = {
    if (locals.var_guard492 != 0.0) {
        let assign25120_e38537: f64 = (1.115 / locals.var_vtm);
        let assign25120_e38540: f64 = (locals.var_tratio - 1.0);
        let assign25120_e38541: f64 = (assign25120_e38537 * assign25120_e38540);
        (assign25120_e38541, 0.0, (((-((1.115 * locals.var_vtm_dn4) / (locals.var_vtm * locals.var_vtm))) * assign25120_e38540) + (assign25120_e38537 * locals.var_tratio_dn4)), (((-((1.115 * locals.var_vtm_dn5) / (locals.var_vtm * locals.var_vtm))) * assign25120_e38540) + (assign25120_e38537 * locals.var_tratio_dn5)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign25120_e38543;
        locals.var_t4_dn3 = assign25120_e38543_d_n3;
        locals.var_t4_dn4 = assign25120_e38543_d_n4;
        locals.var_t4_dn5 = assign25120_e38543_d_n5;
        locals.var_t4_dn6 = assign25120_e38543_d_n6;
        locals.var_t4_dn7 = assign25120_e38543_d_n7;
        locals.var_t4_dn8 = assign25120_e38543_d_n8;
        locals.var_t4_dn9 = assign25120_e38543_d_n9;
        locals.var_t4_dn10 = assign25120_e38543_d_n10;
        locals.var_t4_dn11 = assign25120_e38543_d_n11;

        let assign25130_e38546: f64 = if locals.var_isdif_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard542 = assign25130_e38546;

        let (assign25150_e38563, assign25150_e38563_d_n3, assign25150_e38563_d_n4, assign25150_e38563_d_n5, assign25150_e38563_d_n6, assign25150_e38563_d_n7, assign25150_e38563_d_n8, assign25150_e38563_d_n9, assign25150_e38563_d_n10, assign25150_e38563_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard542 == 0.0)) {
        let assign25150_e38559: f64 = (locals.var_xdif_i * locals.var_t4);
        let assign25150_e38561: f64 = (assign25150_e38559 / locals.var_ndiode_i);
        (assign25150_e38561, ((locals.var_xdif_i * locals.var_t4_dn3) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn4) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn5) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn6) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn7) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn8) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn9) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn10) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn11) / locals.var_ndiode_i),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign25150_e38563;
        locals.var_t7_dn3 = assign25150_e38563_d_n3;
        locals.var_t7_dn4 = assign25150_e38563_d_n4;
        locals.var_t7_dn5 = assign25150_e38563_d_n5;
        locals.var_t7_dn6 = assign25150_e38563_d_n6;
        locals.var_t7_dn7 = assign25150_e38563_d_n7;
        locals.var_t7_dn8 = assign25150_e38563_d_n8;
        locals.var_t7_dn9 = assign25150_e38563_d_n9;
        locals.var_t7_dn10 = assign25150_e38563_d_n10;
        locals.var_t7_dn11 = assign25150_e38563_d_n11;

        let (assign25160_e38571, assign25160_e38571_d_n3, assign25160_e38571_d_n4, assign25160_e38571_d_n5, assign25160_e38571_d_n6, assign25160_e38571_d_n7, assign25160_e38571_d_n8, assign25160_e38571_d_n9, assign25160_e38571_d_n10, assign25160_e38571_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard542 == 0.0)) {
        let assign25160_e38569: f64 = { let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign25160_e38569, ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn3), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn4), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn5), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn6), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn7), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn8), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn9), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn10), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign25160_e38571;
        locals.var_t1_dn3 = assign25160_e38571_d_n3;
        locals.var_t1_dn4 = assign25160_e38571_d_n4;
        locals.var_t1_dn5 = assign25160_e38571_d_n5;
        locals.var_t1_dn6 = assign25160_e38571_d_n6;
        locals.var_t1_dn7 = assign25160_e38571_d_n7;
        locals.var_t1_dn8 = assign25160_e38571_d_n8;
        locals.var_t1_dn9 = assign25160_e38571_d_n9;
        locals.var_t1_dn10 = assign25160_e38571_d_n10;
        locals.var_t1_dn11 = assign25160_e38571_d_n11;

        let (assign25170_e38580, assign25170_e38580_d_n3, assign25170_e38580_d_n4, assign25170_e38580_d_n5, assign25170_e38580_d_n6, assign25170_e38580_d_n7, assign25170_e38580_d_n8, assign25170_e38580_d_n9, assign25170_e38580_d_n10, assign25170_e38580_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard542 == 0.0)) {
        let assign25170_e38578: f64 = (locals.var_isdif_i * locals.var_t1);
        (assign25170_e38578, (locals.var_isdif_i * locals.var_t1_dn3), (locals.var_isdif_i * locals.var_t1_dn4), (locals.var_isdif_i * locals.var_t1_dn5), (locals.var_isdif_i * locals.var_t1_dn6), (locals.var_isdif_i * locals.var_t1_dn7), (locals.var_isdif_i * locals.var_t1_dn8), (locals.var_isdif_i * locals.var_t1_dn9), (locals.var_isdif_i * locals.var_t1_dn10), (locals.var_isdif_i * locals.var_t1_dn11),)
    } else {
        (locals.var_jdifs, locals.var_jdifs_dn3, locals.var_jdifs_dn4, locals.var_jdifs_dn5, locals.var_jdifs_dn6, locals.var_jdifs_dn7, locals.var_jdifs_dn8, locals.var_jdifs_dn9, locals.var_jdifs_dn10, locals.var_jdifs_dn11,)
    }
};
        locals.var_jdifs = assign25170_e38580;
        locals.var_jdifs_dn3 = assign25170_e38580_d_n3;
        locals.var_jdifs_dn4 = assign25170_e38580_d_n4;
        locals.var_jdifs_dn5 = assign25170_e38580_d_n5;
        locals.var_jdifs_dn6 = assign25170_e38580_d_n6;
        locals.var_jdifs_dn7 = assign25170_e38580_d_n7;
        locals.var_jdifs_dn8 = assign25170_e38580_d_n8;
        locals.var_jdifs_dn9 = assign25170_e38580_d_n9;
        locals.var_jdifs_dn10 = assign25170_e38580_d_n10;
        locals.var_jdifs_dn11 = assign25170_e38580_d_n11;

        let (assign25180_e38589, assign25180_e38589_d_n3, assign25180_e38589_d_n4, assign25180_e38589_d_n5, assign25180_e38589_d_n6, assign25180_e38589_d_n7, assign25180_e38589_d_n8, assign25180_e38589_d_n9, assign25180_e38589_d_n10, assign25180_e38589_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard542 == 0.0)) {
        let assign25180_e38587: f64 = (locals.var_wstsi * locals.var_jdifs);
        (assign25180_e38587, (locals.var_wstsi * locals.var_jdifs_dn3), (locals.var_wstsi * locals.var_jdifs_dn4), (locals.var_wstsi * locals.var_jdifs_dn5), (locals.var_wstsi * locals.var_jdifs_dn6), (locals.var_wstsi * locals.var_jdifs_dn7), (locals.var_wstsi * locals.var_jdifs_dn8), (locals.var_wstsi * locals.var_jdifs_dn9), (locals.var_wstsi * locals.var_jdifs_dn10), (locals.var_wstsi * locals.var_jdifs_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign25180_e38589;
        locals.var_t0_dn3 = assign25180_e38589_d_n3;
        locals.var_t0_dn4 = assign25180_e38589_d_n4;
        locals.var_t0_dn5 = assign25180_e38589_d_n5;
        locals.var_t0_dn6 = assign25180_e38589_d_n6;
        locals.var_t0_dn7 = assign25180_e38589_d_n7;
        locals.var_t0_dn8 = assign25180_e38589_d_n8;
        locals.var_t0_dn9 = assign25180_e38589_d_n9;
        locals.var_t0_dn10 = assign25180_e38589_d_n10;
        locals.var_t0_dn11 = assign25180_e38589_d_n11;

        let assign25200_e38603: f64 = if locals.var_iddif_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard543 = assign25200_e38603;

        let (assign25220_e38620, assign25220_e38620_d_n3, assign25220_e38620_d_n4, assign25220_e38620_d_n5, assign25220_e38620_d_n6, assign25220_e38620_d_n7, assign25220_e38620_d_n8, assign25220_e38620_d_n9, assign25220_e38620_d_n10, assign25220_e38620_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard543 == 0.0)) {
        let assign25220_e38616: f64 = (locals.var_xdif_i * locals.var_t4);
        let assign25220_e38618: f64 = (assign25220_e38616 / locals.var_ndiode_i);
        (assign25220_e38618, ((locals.var_xdif_i * locals.var_t4_dn3) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn4) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn5) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn6) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn7) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn8) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn9) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn10) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn11) / locals.var_ndiode_i),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign25220_e38620;
        locals.var_t7_dn3 = assign25220_e38620_d_n3;
        locals.var_t7_dn4 = assign25220_e38620_d_n4;
        locals.var_t7_dn5 = assign25220_e38620_d_n5;
        locals.var_t7_dn6 = assign25220_e38620_d_n6;
        locals.var_t7_dn7 = assign25220_e38620_d_n7;
        locals.var_t7_dn8 = assign25220_e38620_d_n8;
        locals.var_t7_dn9 = assign25220_e38620_d_n9;
        locals.var_t7_dn10 = assign25220_e38620_d_n10;
        locals.var_t7_dn11 = assign25220_e38620_d_n11;

        let (assign25230_e38628, assign25230_e38628_d_n3, assign25230_e38628_d_n4, assign25230_e38628_d_n5, assign25230_e38628_d_n6, assign25230_e38628_d_n7, assign25230_e38628_d_n8, assign25230_e38628_d_n9, assign25230_e38628_d_n10, assign25230_e38628_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard543 == 0.0)) {
        let assign25230_e38626: f64 = { let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign25230_e38626, ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn3), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn4), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn5), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn6), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn7), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn8), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn9), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn10), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign25230_e38628;
        locals.var_t1_dn3 = assign25230_e38628_d_n3;
        locals.var_t1_dn4 = assign25230_e38628_d_n4;
        locals.var_t1_dn5 = assign25230_e38628_d_n5;
        locals.var_t1_dn6 = assign25230_e38628_d_n6;
        locals.var_t1_dn7 = assign25230_e38628_d_n7;
        locals.var_t1_dn8 = assign25230_e38628_d_n8;
        locals.var_t1_dn9 = assign25230_e38628_d_n9;
        locals.var_t1_dn10 = assign25230_e38628_d_n10;
        locals.var_t1_dn11 = assign25230_e38628_d_n11;

        let (assign25240_e38637, assign25240_e38637_d_n3, assign25240_e38637_d_n4, assign25240_e38637_d_n5, assign25240_e38637_d_n6, assign25240_e38637_d_n7, assign25240_e38637_d_n8, assign25240_e38637_d_n9, assign25240_e38637_d_n10, assign25240_e38637_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard543 == 0.0)) {
        let assign25240_e38635: f64 = (locals.var_iddif_i * locals.var_t1);
        (assign25240_e38635, (locals.var_iddif_i * locals.var_t1_dn3), (locals.var_iddif_i * locals.var_t1_dn4), (locals.var_iddif_i * locals.var_t1_dn5), (locals.var_iddif_i * locals.var_t1_dn6), (locals.var_iddif_i * locals.var_t1_dn7), (locals.var_iddif_i * locals.var_t1_dn8), (locals.var_iddif_i * locals.var_t1_dn9), (locals.var_iddif_i * locals.var_t1_dn10), (locals.var_iddif_i * locals.var_t1_dn11),)
    } else {
        (locals.var_jdifd, locals.var_jdifd_dn3, locals.var_jdifd_dn4, locals.var_jdifd_dn5, locals.var_jdifd_dn6, locals.var_jdifd_dn7, locals.var_jdifd_dn8, locals.var_jdifd_dn9, locals.var_jdifd_dn10, locals.var_jdifd_dn11,)
    }
};
        locals.var_jdifd = assign25240_e38637;
        locals.var_jdifd_dn3 = assign25240_e38637_d_n3;
        locals.var_jdifd_dn4 = assign25240_e38637_d_n4;
        locals.var_jdifd_dn5 = assign25240_e38637_d_n5;
        locals.var_jdifd_dn6 = assign25240_e38637_d_n6;
        locals.var_jdifd_dn7 = assign25240_e38637_d_n7;
        locals.var_jdifd_dn8 = assign25240_e38637_d_n8;
        locals.var_jdifd_dn9 = assign25240_e38637_d_n9;
        locals.var_jdifd_dn10 = assign25240_e38637_d_n10;
        locals.var_jdifd_dn11 = assign25240_e38637_d_n11;

        let (assign25250_e38646, assign25250_e38646_d_n3, assign25250_e38646_d_n4, assign25250_e38646_d_n5, assign25250_e38646_d_n6, assign25250_e38646_d_n7, assign25250_e38646_d_n8, assign25250_e38646_d_n9, assign25250_e38646_d_n10, assign25250_e38646_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard543 == 0.0)) {
        let assign25250_e38644: f64 = (locals.var_wdtsi * locals.var_jdifd);
        (assign25250_e38644, (locals.var_wdtsi * locals.var_jdifd_dn3), (locals.var_wdtsi * locals.var_jdifd_dn4), (locals.var_wdtsi * locals.var_jdifd_dn5), (locals.var_wdtsi * locals.var_jdifd_dn6), (locals.var_wdtsi * locals.var_jdifd_dn7), (locals.var_wdtsi * locals.var_jdifd_dn8), (locals.var_wdtsi * locals.var_jdifd_dn9), (locals.var_wdtsi * locals.var_jdifd_dn10), (locals.var_wdtsi * locals.var_jdifd_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign25250_e38646;
        locals.var_t0_dn3 = assign25250_e38646_d_n3;
        locals.var_t0_dn4 = assign25250_e38646_d_n4;
        locals.var_t0_dn5 = assign25250_e38646_d_n5;
        locals.var_t0_dn6 = assign25250_e38646_d_n6;
        locals.var_t0_dn7 = assign25250_e38646_d_n7;
        locals.var_t0_dn8 = assign25250_e38646_d_n8;
        locals.var_t0_dn9 = assign25250_e38646_d_n9;
        locals.var_t0_dn10 = assign25250_e38646_d_n10;
        locals.var_t0_dn11 = assign25250_e38646_d_n11;

        let assign25270_e38660: f64 = if locals.var_isrec_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard544 = assign25270_e38660;

        let (assign25290_e38677, assign25290_e38677_d_n3, assign25290_e38677_d_n4, assign25290_e38677_d_n5, assign25290_e38677_d_n6, assign25290_e38677_d_n7, assign25290_e38677_d_n8, assign25290_e38677_d_n9, assign25290_e38677_d_n10, assign25290_e38677_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard544 == 0.0)) {
        let assign25290_e38673: f64 = (locals.var_xrec_i * locals.var_t4);
        let assign25290_e38675: f64 = (assign25290_e38673 / locals.var_nrecf0_i);
        (assign25290_e38675, ((locals.var_xrec_i * locals.var_t4_dn3) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn4) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn5) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn6) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn7) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn8) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn9) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn10) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn11) / locals.var_nrecf0_i),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign25290_e38677;
        locals.var_t7_dn3 = assign25290_e38677_d_n3;
        locals.var_t7_dn4 = assign25290_e38677_d_n4;
        locals.var_t7_dn5 = assign25290_e38677_d_n5;
        locals.var_t7_dn6 = assign25290_e38677_d_n6;
        locals.var_t7_dn7 = assign25290_e38677_d_n7;
        locals.var_t7_dn8 = assign25290_e38677_d_n8;
        locals.var_t7_dn9 = assign25290_e38677_d_n9;
        locals.var_t7_dn10 = assign25290_e38677_d_n10;
        locals.var_t7_dn11 = assign25290_e38677_d_n11;

        let (assign25300_e38685, assign25300_e38685_d_n3, assign25300_e38685_d_n4, assign25300_e38685_d_n5, assign25300_e38685_d_n6, assign25300_e38685_d_n7, assign25300_e38685_d_n8, assign25300_e38685_d_n9, assign25300_e38685_d_n10, assign25300_e38685_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard544 == 0.0)) {
        let assign25300_e38683: f64 = { let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign25300_e38683, ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn3), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn4), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn5), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn6), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn7), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn8), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn9), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn10), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn11),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign25300_e38685;
        locals.var_t2_dn3 = assign25300_e38685_d_n3;
        locals.var_t2_dn4 = assign25300_e38685_d_n4;
        locals.var_t2_dn5 = assign25300_e38685_d_n5;
        locals.var_t2_dn6 = assign25300_e38685_d_n6;
        locals.var_t2_dn7 = assign25300_e38685_d_n7;
        locals.var_t2_dn8 = assign25300_e38685_d_n8;
        locals.var_t2_dn9 = assign25300_e38685_d_n9;
        locals.var_t2_dn10 = assign25300_e38685_d_n10;
        locals.var_t2_dn11 = assign25300_e38685_d_n11;

        let (assign25310_e38694, assign25310_e38694_d_n3, assign25310_e38694_d_n4, assign25310_e38694_d_n5, assign25310_e38694_d_n6, assign25310_e38694_d_n7, assign25310_e38694_d_n8, assign25310_e38694_d_n9, assign25310_e38694_d_n10, assign25310_e38694_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard544 == 0.0)) {
        let assign25310_e38692: f64 = (locals.var_isrec_i * locals.var_t2);
        (assign25310_e38692, (locals.var_isrec_i * locals.var_t2_dn3), (locals.var_isrec_i * locals.var_t2_dn4), (locals.var_isrec_i * locals.var_t2_dn5), (locals.var_isrec_i * locals.var_t2_dn6), (locals.var_isrec_i * locals.var_t2_dn7), (locals.var_isrec_i * locals.var_t2_dn8), (locals.var_isrec_i * locals.var_t2_dn9), (locals.var_isrec_i * locals.var_t2_dn10), (locals.var_isrec_i * locals.var_t2_dn11),)
    } else {
        (locals.var_jrecs, locals.var_jrecs_dn3, locals.var_jrecs_dn4, locals.var_jrecs_dn5, locals.var_jrecs_dn6, locals.var_jrecs_dn7, locals.var_jrecs_dn8, locals.var_jrecs_dn9, locals.var_jrecs_dn10, locals.var_jrecs_dn11,)
    }
};
        locals.var_jrecs = assign25310_e38694;
        locals.var_jrecs_dn3 = assign25310_e38694_d_n3;
        locals.var_jrecs_dn4 = assign25310_e38694_d_n4;
        locals.var_jrecs_dn5 = assign25310_e38694_d_n5;
        locals.var_jrecs_dn6 = assign25310_e38694_d_n6;
        locals.var_jrecs_dn7 = assign25310_e38694_d_n7;
        locals.var_jrecs_dn8 = assign25310_e38694_d_n8;
        locals.var_jrecs_dn9 = assign25310_e38694_d_n9;
        locals.var_jrecs_dn10 = assign25310_e38694_d_n10;
        locals.var_jrecs_dn11 = assign25310_e38694_d_n11;

        let (assign25320_e38711, assign25320_e38711_d_n4, assign25320_e38711_d_n5,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard544 == 0.0)) {
        let assign25320_e38701: f64 = (p.p925 * locals.var_nrecf0_i);
        let assign25320_e38706: f64 = (locals.var_tratio - 1.0);
        let assign25320_e38707: f64 = (locals.var_ntrecf_i * assign25320_e38706);
        let assign25320_e38708: f64 = (1.0 + assign25320_e38707);
        let assign25320_e38709: f64 = (assign25320_e38701 * assign25320_e38708);
        (assign25320_e38709, (assign25320_e38701 * (locals.var_ntrecf_i * locals.var_tratio_dn4)), (assign25320_e38701 * (locals.var_ntrecf_i * locals.var_tratio_dn5)),)
    } else {
        (locals.var_nvtmf, locals.var_nvtmf_dn4, locals.var_nvtmf_dn5,)
    }
};
        locals.var_nvtmf = assign25320_e38711;
        locals.var_nvtmf_dn4 = assign25320_e38711_d_n4;
        locals.var_nvtmf_dn5 = assign25320_e38711_d_n5;

        let (assign25330_e38728, assign25330_e38728_d_n4, assign25330_e38728_d_n5,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard544 == 0.0)) {
        let assign25330_e38718: f64 = (p.p925 * locals.var_nrecr0_i);
        let assign25330_e38723: f64 = (locals.var_tratio - 1.0);
        let assign25330_e38724: f64 = (locals.var_ntrecr_i * assign25330_e38723);
        let assign25330_e38725: f64 = (1.0 + assign25330_e38724);
        let assign25330_e38726: f64 = (assign25330_e38718 * assign25330_e38725);
        (assign25330_e38726, (assign25330_e38718 * (locals.var_ntrecr_i * locals.var_tratio_dn4)), (assign25330_e38718 * (locals.var_ntrecr_i * locals.var_tratio_dn5)),)
    } else {
        (locals.var_nvtmr, locals.var_nvtmr_dn4, locals.var_nvtmr_dn5,)
    }
};
        locals.var_nvtmr = assign25330_e38728;
        locals.var_nvtmr_dn4 = assign25330_e38728_d_n4;
        locals.var_nvtmr_dn5 = assign25330_e38728_d_n5;

        let (assign25340_e38737, assign25340_e38737_d_n3, assign25340_e38737_d_n4, assign25340_e38737_d_n5, assign25340_e38737_d_n6, assign25340_e38737_d_n7, assign25340_e38737_d_n8, assign25340_e38737_d_n9, assign25340_e38737_d_n10, assign25340_e38737_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard544 == 0.0)) {
        let assign25340_e38735: f64 = (locals.var_vbs_jct / locals.var_nvtmf);
        (assign25340_e38735, 0.0, (-((locals.var_vbs_jct * locals.var_nvtmf_dn4) / (locals.var_nvtmf * locals.var_nvtmf))), (-((locals.var_vbs_jct * locals.var_nvtmf_dn5) / (locals.var_nvtmf * locals.var_nvtmf))), 0.0, (locals.var_vbs_jct_dn7 / locals.var_nvtmf), 0.0, 0.0, (locals.var_vbs_jct_dn10 / locals.var_nvtmf), 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign25340_e38737;
        locals.var_t0_dn3 = assign25340_e38737_d_n3;
        locals.var_t0_dn4 = assign25340_e38737_d_n4;
        locals.var_t0_dn5 = assign25340_e38737_d_n5;
        locals.var_t0_dn6 = assign25340_e38737_d_n6;
        locals.var_t0_dn7 = assign25340_e38737_d_n7;
        locals.var_t0_dn8 = assign25340_e38737_d_n8;
        locals.var_t0_dn9 = assign25340_e38737_d_n9;
        locals.var_t0_dn10 = assign25340_e38737_d_n10;
        locals.var_t0_dn11 = assign25340_e38737_d_n11;

        let (assign25350_e38745, assign25350_e38745_d_n3, assign25350_e38745_d_n4, assign25350_e38745_d_n5, assign25350_e38745_d_n6, assign25350_e38745_d_n7, assign25350_e38745_d_n8, assign25350_e38745_d_n9, assign25350_e38745_d_n10, assign25350_e38745_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard544 == 0.0)) {
        let assign25350_e38743: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign25350_e38743, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_t10, locals.var_t10_dn3, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11,)
    }
};
        locals.var_t10 = assign25350_e38745;
        locals.var_t10_dn3 = assign25350_e38745_d_n3;
        locals.var_t10_dn4 = assign25350_e38745_d_n4;
        locals.var_t10_dn5 = assign25350_e38745_d_n5;
        locals.var_t10_dn6 = assign25350_e38745_d_n6;
        locals.var_t10_dn7 = assign25350_e38745_d_n7;
        locals.var_t10_dn8 = assign25350_e38745_d_n8;
        locals.var_t10_dn9 = assign25350_e38745_d_n9;
        locals.var_t10_dn10 = assign25350_e38745_d_n10;
        locals.var_t10_dn11 = assign25350_e38745_d_n11;

        let assign25360_e38748: f64 = (locals.var_vrec0_i - locals.var_vbs_jct);
        let assign25360_e38750: f64 = if assign25360_e38748 < 0.001 { 1.0 } else { 0.0 };
        locals.var_guard545 = assign25360_e38750;

        let (assign25370_e38759, assign25370_e38759_d_n3, assign25370_e38759_d_n4, assign25370_e38759_d_n5, assign25370_e38759_d_n6, assign25370_e38759_d_n7, assign25370_e38759_d_n8, assign25370_e38759_d_n9, assign25370_e38759_d_n10, assign25370_e38759_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard544 == 0.0)) && (locals.var_guard545 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign25370_e38759;
        locals.var_t1_dn3 = assign25370_e38759_d_n3;
        locals.var_t1_dn4 = assign25370_e38759_d_n4;
        locals.var_t1_dn5 = assign25370_e38759_d_n5;
        locals.var_t1_dn6 = assign25370_e38759_d_n6;
        locals.var_t1_dn7 = assign25370_e38759_d_n7;
        locals.var_t1_dn8 = assign25370_e38759_d_n8;
        locals.var_t1_dn9 = assign25370_e38759_d_n9;
        locals.var_t1_dn10 = assign25370_e38759_d_n10;
        locals.var_t1_dn11 = assign25370_e38759_d_n11;

        let (assign25380_e38775, assign25380_e38775_d_n3, assign25380_e38775_d_n4, assign25380_e38775_d_n5, assign25380_e38775_d_n6, assign25380_e38775_d_n7, assign25380_e38775_d_n8, assign25380_e38775_d_n9, assign25380_e38775_d_n10, assign25380_e38775_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard544 == 0.0)) && (locals.var_guard545 != 0.0)) {
        let assign25380_e38767: f64 = (-locals.var_vbs_jct);
        let assign25380_e38769: f64 = (assign25380_e38767 / locals.var_nvtmr);
        let assign25380_e38771: f64 = (assign25380_e38769 * locals.var_vrec0_i);
        let assign25380_e38773: f64 = (assign25380_e38771 * locals.var_t1);
        (assign25380_e38773, (assign25380_e38771 * locals.var_t1_dn3), ((((-((assign25380_e38767 * locals.var_nvtmr_dn4) / (locals.var_nvtmr * locals.var_nvtmr))) * locals.var_vrec0_i) * locals.var_t1) + (assign25380_e38771 * locals.var_t1_dn4)), ((((-((assign25380_e38767 * locals.var_nvtmr_dn5) / (locals.var_nvtmr * locals.var_nvtmr))) * locals.var_vrec0_i) * locals.var_t1) + (assign25380_e38771 * locals.var_t1_dn5)), (assign25380_e38771 * locals.var_t1_dn6), (((((-locals.var_vbs_jct_dn7) / locals.var_nvtmr) * locals.var_vrec0_i) * locals.var_t1) + (assign25380_e38771 * locals.var_t1_dn7)), (assign25380_e38771 * locals.var_t1_dn8), (assign25380_e38771 * locals.var_t1_dn9), (((((-locals.var_vbs_jct_dn10) / locals.var_nvtmr) * locals.var_vrec0_i) * locals.var_t1) + (assign25380_e38771 * locals.var_t1_dn10)), (assign25380_e38771 * locals.var_t1_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign25380_e38775;
        locals.var_t0_dn3 = assign25380_e38775_d_n3;
        locals.var_t0_dn4 = assign25380_e38775_d_n4;
        locals.var_t0_dn5 = assign25380_e38775_d_n5;
        locals.var_t0_dn6 = assign25380_e38775_d_n6;
        locals.var_t0_dn7 = assign25380_e38775_d_n7;
        locals.var_t0_dn8 = assign25380_e38775_d_n8;
        locals.var_t0_dn9 = assign25380_e38775_d_n9;
        locals.var_t0_dn10 = assign25380_e38775_d_n10;
        locals.var_t0_dn11 = assign25380_e38775_d_n11;

        let (assign25390_e38785, assign25390_e38785_d_n3, assign25390_e38785_d_n4, assign25390_e38785_d_n5, assign25390_e38785_d_n6, assign25390_e38785_d_n7, assign25390_e38785_d_n8, assign25390_e38785_d_n9, assign25390_e38785_d_n10, assign25390_e38785_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard544 == 0.0)) && (locals.var_guard545 != 0.0)) {
        let assign25390_e38783: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign25390_e38783, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign25390_e38785;
        locals.var_t11_dn3 = assign25390_e38785_d_n3;
        locals.var_t11_dn4 = assign25390_e38785_d_n4;
        locals.var_t11_dn5 = assign25390_e38785_d_n5;
        locals.var_t11_dn6 = assign25390_e38785_d_n6;
        locals.var_t11_dn7 = assign25390_e38785_d_n7;
        locals.var_t11_dn8 = assign25390_e38785_d_n8;
        locals.var_t11_dn9 = assign25390_e38785_d_n9;
        locals.var_t11_dn10 = assign25390_e38785_d_n10;
        locals.var_t11_dn11 = assign25390_e38785_d_n11;

        let (assign25400_e38795, assign25400_e38795_d_n3, assign25400_e38795_d_n4, assign25400_e38795_d_n5, assign25400_e38795_d_n6, assign25400_e38795_d_n7, assign25400_e38795_d_n8, assign25400_e38795_d_n9, assign25400_e38795_d_n10, assign25400_e38795_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard544 == 0.0)) && (locals.var_guard545 != 0.0)) {
        let assign25400_e38793: f64 = (-locals.var_t11);
        (assign25400_e38793, (-locals.var_t11_dn3), (-locals.var_t11_dn4), (-locals.var_t11_dn5), (-locals.var_t11_dn6), (-locals.var_t11_dn7), (-locals.var_t11_dn8), (-locals.var_t11_dn9), (-locals.var_t11_dn10), (-locals.var_t11_dn11),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign25400_e38795;
        locals.var_t11_dn3 = assign25400_e38795_d_n3;
        locals.var_t11_dn4 = assign25400_e38795_d_n4;
        locals.var_t11_dn5 = assign25400_e38795_d_n5;
        locals.var_t11_dn6 = assign25400_e38795_d_n6;
        locals.var_t11_dn7 = assign25400_e38795_d_n7;
        locals.var_t11_dn8 = assign25400_e38795_d_n8;
        locals.var_t11_dn9 = assign25400_e38795_d_n9;
        locals.var_t11_dn10 = assign25400_e38795_d_n10;
        locals.var_t11_dn11 = assign25400_e38795_d_n11;

        let (assign25410_e38809, assign25410_e38809_d_n3, assign25410_e38809_d_n4, assign25410_e38809_d_n5, assign25410_e38809_d_n6, assign25410_e38809_d_n7, assign25410_e38809_d_n8, assign25410_e38809_d_n9, assign25410_e38809_d_n10, assign25410_e38809_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard544 == 0.0)) && (locals.var_guard545 == 0.0)) {
        let assign25410_e38806: f64 = (locals.var_vrec0_i - locals.var_vbs_jct);
        let assign25410_e38807: f64 = (1.0 / assign25410_e38806);
        (assign25410_e38807, 0.0, 0.0, 0.0, 0.0, (-((-locals.var_vbs_jct_dn7) / (assign25410_e38806 * assign25410_e38806))), 0.0, 0.0, (-((-locals.var_vbs_jct_dn10) / (assign25410_e38806 * assign25410_e38806))), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign25410_e38809;
        locals.var_t1_dn3 = assign25410_e38809_d_n3;
        locals.var_t1_dn4 = assign25410_e38809_d_n4;
        locals.var_t1_dn5 = assign25410_e38809_d_n5;
        locals.var_t1_dn6 = assign25410_e38809_d_n6;
        locals.var_t1_dn7 = assign25410_e38809_d_n7;
        locals.var_t1_dn8 = assign25410_e38809_d_n8;
        locals.var_t1_dn9 = assign25410_e38809_d_n9;
        locals.var_t1_dn10 = assign25410_e38809_d_n10;
        locals.var_t1_dn11 = assign25410_e38809_d_n11;

        let (assign25420_e38826, assign25420_e38826_d_n3, assign25420_e38826_d_n4, assign25420_e38826_d_n5, assign25420_e38826_d_n6, assign25420_e38826_d_n7, assign25420_e38826_d_n8, assign25420_e38826_d_n9, assign25420_e38826_d_n10, assign25420_e38826_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard544 == 0.0)) && (locals.var_guard545 == 0.0)) {
        let assign25420_e38818: f64 = (-locals.var_vbs_jct);
        let assign25420_e38820: f64 = (assign25420_e38818 / locals.var_nvtmr);
        let assign25420_e38822: f64 = (assign25420_e38820 * locals.var_vrec0_i);
        let assign25420_e38824: f64 = (assign25420_e38822 * locals.var_t1);
        (assign25420_e38824, (assign25420_e38822 * locals.var_t1_dn3), ((((-((assign25420_e38818 * locals.var_nvtmr_dn4) / (locals.var_nvtmr * locals.var_nvtmr))) * locals.var_vrec0_i) * locals.var_t1) + (assign25420_e38822 * locals.var_t1_dn4)), ((((-((assign25420_e38818 * locals.var_nvtmr_dn5) / (locals.var_nvtmr * locals.var_nvtmr))) * locals.var_vrec0_i) * locals.var_t1) + (assign25420_e38822 * locals.var_t1_dn5)), (assign25420_e38822 * locals.var_t1_dn6), (((((-locals.var_vbs_jct_dn7) / locals.var_nvtmr) * locals.var_vrec0_i) * locals.var_t1) + (assign25420_e38822 * locals.var_t1_dn7)), (assign25420_e38822 * locals.var_t1_dn8), (assign25420_e38822 * locals.var_t1_dn9), (((((-locals.var_vbs_jct_dn10) / locals.var_nvtmr) * locals.var_vrec0_i) * locals.var_t1) + (assign25420_e38822 * locals.var_t1_dn10)), (assign25420_e38822 * locals.var_t1_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign25420_e38826;
        locals.var_t0_dn3 = assign25420_e38826_d_n3;
        locals.var_t0_dn4 = assign25420_e38826_d_n4;
        locals.var_t0_dn5 = assign25420_e38826_d_n5;
        locals.var_t0_dn6 = assign25420_e38826_d_n6;
        locals.var_t0_dn7 = assign25420_e38826_d_n7;
        locals.var_t0_dn8 = assign25420_e38826_d_n8;
        locals.var_t0_dn9 = assign25420_e38826_d_n9;
        locals.var_t0_dn10 = assign25420_e38826_d_n10;
        locals.var_t0_dn11 = assign25420_e38826_d_n11;

        let (assign25430_e38837, assign25430_e38837_d_n3, assign25430_e38837_d_n4, assign25430_e38837_d_n5, assign25430_e38837_d_n6, assign25430_e38837_d_n7, assign25430_e38837_d_n8, assign25430_e38837_d_n9, assign25430_e38837_d_n10, assign25430_e38837_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard544 == 0.0)) && (locals.var_guard545 == 0.0)) {
        let assign25430_e38835: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign25430_e38835, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign25430_e38837;
        locals.var_t11_dn3 = assign25430_e38837_d_n3;
        locals.var_t11_dn4 = assign25430_e38837_d_n4;
        locals.var_t11_dn5 = assign25430_e38837_d_n5;
        locals.var_t11_dn6 = assign25430_e38837_d_n6;
        locals.var_t11_dn7 = assign25430_e38837_d_n7;
        locals.var_t11_dn8 = assign25430_e38837_d_n8;
        locals.var_t11_dn9 = assign25430_e38837_d_n9;
        locals.var_t11_dn10 = assign25430_e38837_d_n10;
        locals.var_t11_dn11 = assign25430_e38837_d_n11;

        let (assign25440_e38848, assign25440_e38848_d_n3, assign25440_e38848_d_n4, assign25440_e38848_d_n5, assign25440_e38848_d_n6, assign25440_e38848_d_n7, assign25440_e38848_d_n8, assign25440_e38848_d_n9, assign25440_e38848_d_n10, assign25440_e38848_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard544 == 0.0)) && (locals.var_guard545 == 0.0)) {
        let assign25440_e38846: f64 = (-locals.var_t11);
        (assign25440_e38846, (-locals.var_t11_dn3), (-locals.var_t11_dn4), (-locals.var_t11_dn5), (-locals.var_t11_dn6), (-locals.var_t11_dn7), (-locals.var_t11_dn8), (-locals.var_t11_dn9), (-locals.var_t11_dn10), (-locals.var_t11_dn11),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign25440_e38848;
        locals.var_t11_dn3 = assign25440_e38848_d_n3;
        locals.var_t11_dn4 = assign25440_e38848_d_n4;
        locals.var_t11_dn5 = assign25440_e38848_d_n5;
        locals.var_t11_dn6 = assign25440_e38848_d_n6;
        locals.var_t11_dn7 = assign25440_e38848_d_n7;
        locals.var_t11_dn8 = assign25440_e38848_d_n8;
        locals.var_t11_dn9 = assign25440_e38848_d_n9;
        locals.var_t11_dn10 = assign25440_e38848_d_n10;
        locals.var_t11_dn11 = assign25440_e38848_d_n11;

    }

    pub(super) fn stamp_transient_block_67(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign25450_e38857, assign25450_e38857_d_n3, assign25450_e38857_d_n4, assign25450_e38857_d_n5, assign25450_e38857_d_n6, assign25450_e38857_d_n7, assign25450_e38857_d_n8, assign25450_e38857_d_n9, assign25450_e38857_d_n10, assign25450_e38857_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard544 == 0.0)) {
        let assign25450_e38855: f64 = (locals.var_wstsi * locals.var_jrecs);
        (assign25450_e38855, (locals.var_wstsi * locals.var_jrecs_dn3), (locals.var_wstsi * locals.var_jrecs_dn4), (locals.var_wstsi * locals.var_jrecs_dn5), (locals.var_wstsi * locals.var_jrecs_dn6), (locals.var_wstsi * locals.var_jrecs_dn7), (locals.var_wstsi * locals.var_jrecs_dn8), (locals.var_wstsi * locals.var_jrecs_dn9), (locals.var_wstsi * locals.var_jrecs_dn10), (locals.var_wstsi * locals.var_jrecs_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign25450_e38857;
        locals.var_t3_dn3 = assign25450_e38857_d_n3;
        locals.var_t3_dn4 = assign25450_e38857_d_n4;
        locals.var_t3_dn5 = assign25450_e38857_d_n5;
        locals.var_t3_dn6 = assign25450_e38857_d_n6;
        locals.var_t3_dn7 = assign25450_e38857_d_n7;
        locals.var_t3_dn8 = assign25450_e38857_d_n8;
        locals.var_t3_dn9 = assign25450_e38857_d_n9;
        locals.var_t3_dn10 = assign25450_e38857_d_n10;
        locals.var_t3_dn11 = assign25450_e38857_d_n11;

        let assign25470_e38871: f64 = if locals.var_idrec_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard546 = assign25470_e38871;

        let (assign25490_e38888, assign25490_e38888_d_n3, assign25490_e38888_d_n4, assign25490_e38888_d_n5, assign25490_e38888_d_n6, assign25490_e38888_d_n7, assign25490_e38888_d_n8, assign25490_e38888_d_n9, assign25490_e38888_d_n10, assign25490_e38888_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard546 == 0.0)) {
        let assign25490_e38884: f64 = (locals.var_xrec_i * locals.var_t4);
        let assign25490_e38886: f64 = (assign25490_e38884 / locals.var_nrecf0_i);
        (assign25490_e38886, ((locals.var_xrec_i * locals.var_t4_dn3) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn4) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn5) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn6) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn7) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn8) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn9) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn10) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn11) / locals.var_nrecf0_i),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign25490_e38888;
        locals.var_t7_dn3 = assign25490_e38888_d_n3;
        locals.var_t7_dn4 = assign25490_e38888_d_n4;
        locals.var_t7_dn5 = assign25490_e38888_d_n5;
        locals.var_t7_dn6 = assign25490_e38888_d_n6;
        locals.var_t7_dn7 = assign25490_e38888_d_n7;
        locals.var_t7_dn8 = assign25490_e38888_d_n8;
        locals.var_t7_dn9 = assign25490_e38888_d_n9;
        locals.var_t7_dn10 = assign25490_e38888_d_n10;
        locals.var_t7_dn11 = assign25490_e38888_d_n11;

        let (assign25500_e38896, assign25500_e38896_d_n3, assign25500_e38896_d_n4, assign25500_e38896_d_n5, assign25500_e38896_d_n6, assign25500_e38896_d_n7, assign25500_e38896_d_n8, assign25500_e38896_d_n9, assign25500_e38896_d_n10, assign25500_e38896_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard546 == 0.0)) {
        let assign25500_e38894: f64 = { let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign25500_e38894, ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn3), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn4), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn5), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn6), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn7), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn8), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn9), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn10), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn11),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign25500_e38896;
        locals.var_t2_dn3 = assign25500_e38896_d_n3;
        locals.var_t2_dn4 = assign25500_e38896_d_n4;
        locals.var_t2_dn5 = assign25500_e38896_d_n5;
        locals.var_t2_dn6 = assign25500_e38896_d_n6;
        locals.var_t2_dn7 = assign25500_e38896_d_n7;
        locals.var_t2_dn8 = assign25500_e38896_d_n8;
        locals.var_t2_dn9 = assign25500_e38896_d_n9;
        locals.var_t2_dn10 = assign25500_e38896_d_n10;
        locals.var_t2_dn11 = assign25500_e38896_d_n11;

        let (assign25510_e38905, assign25510_e38905_d_n3, assign25510_e38905_d_n4, assign25510_e38905_d_n5, assign25510_e38905_d_n6, assign25510_e38905_d_n7, assign25510_e38905_d_n8, assign25510_e38905_d_n9, assign25510_e38905_d_n10, assign25510_e38905_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard546 == 0.0)) {
        let assign25510_e38903: f64 = (locals.var_idrec_i * locals.var_t2);
        (assign25510_e38903, (locals.var_idrec_i * locals.var_t2_dn3), (locals.var_idrec_i * locals.var_t2_dn4), (locals.var_idrec_i * locals.var_t2_dn5), (locals.var_idrec_i * locals.var_t2_dn6), (locals.var_idrec_i * locals.var_t2_dn7), (locals.var_idrec_i * locals.var_t2_dn8), (locals.var_idrec_i * locals.var_t2_dn9), (locals.var_idrec_i * locals.var_t2_dn10), (locals.var_idrec_i * locals.var_t2_dn11),)
    } else {
        (locals.var_jrecd, locals.var_jrecd_dn3, locals.var_jrecd_dn4, locals.var_jrecd_dn5, locals.var_jrecd_dn6, locals.var_jrecd_dn7, locals.var_jrecd_dn8, locals.var_jrecd_dn9, locals.var_jrecd_dn10, locals.var_jrecd_dn11,)
    }
};
        locals.var_jrecd = assign25510_e38905;
        locals.var_jrecd_dn3 = assign25510_e38905_d_n3;
        locals.var_jrecd_dn4 = assign25510_e38905_d_n4;
        locals.var_jrecd_dn5 = assign25510_e38905_d_n5;
        locals.var_jrecd_dn6 = assign25510_e38905_d_n6;
        locals.var_jrecd_dn7 = assign25510_e38905_d_n7;
        locals.var_jrecd_dn8 = assign25510_e38905_d_n8;
        locals.var_jrecd_dn9 = assign25510_e38905_d_n9;
        locals.var_jrecd_dn10 = assign25510_e38905_d_n10;
        locals.var_jrecd_dn11 = assign25510_e38905_d_n11;

        let (assign25520_e38922, assign25520_e38922_d_n4, assign25520_e38922_d_n5,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard546 == 0.0)) {
        let assign25520_e38912: f64 = (p.p925 * locals.var_nrecf0_i);
        let assign25520_e38917: f64 = (locals.var_tratio - 1.0);
        let assign25520_e38918: f64 = (locals.var_ntrecf_i * assign25520_e38917);
        let assign25520_e38919: f64 = (1.0 + assign25520_e38918);
        let assign25520_e38920: f64 = (assign25520_e38912 * assign25520_e38919);
        (assign25520_e38920, (assign25520_e38912 * (locals.var_ntrecf_i * locals.var_tratio_dn4)), (assign25520_e38912 * (locals.var_ntrecf_i * locals.var_tratio_dn5)),)
    } else {
        (locals.var_nvtmf, locals.var_nvtmf_dn4, locals.var_nvtmf_dn5,)
    }
};
        locals.var_nvtmf = assign25520_e38922;
        locals.var_nvtmf_dn4 = assign25520_e38922_d_n4;
        locals.var_nvtmf_dn5 = assign25520_e38922_d_n5;

        let (assign25530_e38939, assign25530_e38939_d_n4, assign25530_e38939_d_n5,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard546 == 0.0)) {
        let assign25530_e38929: f64 = (p.p925 * locals.var_nrecr0_i);
        let assign25530_e38934: f64 = (locals.var_tratio - 1.0);
        let assign25530_e38935: f64 = (locals.var_ntrecr_i * assign25530_e38934);
        let assign25530_e38936: f64 = (1.0 + assign25530_e38935);
        let assign25530_e38937: f64 = (assign25530_e38929 * assign25530_e38936);
        (assign25530_e38937, (assign25530_e38929 * (locals.var_ntrecr_i * locals.var_tratio_dn4)), (assign25530_e38929 * (locals.var_ntrecr_i * locals.var_tratio_dn5)),)
    } else {
        (locals.var_nvtmr, locals.var_nvtmr_dn4, locals.var_nvtmr_dn5,)
    }
};
        locals.var_nvtmr = assign25530_e38939;
        locals.var_nvtmr_dn4 = assign25530_e38939_d_n4;
        locals.var_nvtmr_dn5 = assign25530_e38939_d_n5;

        let (assign25540_e38948, assign25540_e38948_d_n3, assign25540_e38948_d_n4, assign25540_e38948_d_n5, assign25540_e38948_d_n6, assign25540_e38948_d_n7, assign25540_e38948_d_n8, assign25540_e38948_d_n9, assign25540_e38948_d_n10, assign25540_e38948_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard546 == 0.0)) {
        let assign25540_e38946: f64 = (locals.var_vbd_jct / locals.var_nvtmf);
        (assign25540_e38946, 0.0, (-((locals.var_vbd_jct * locals.var_nvtmf_dn4) / (locals.var_nvtmf * locals.var_nvtmf))), (-((locals.var_vbd_jct * locals.var_nvtmf_dn5) / (locals.var_nvtmf * locals.var_nvtmf))), (locals.var_vbd_jct_dn6 / locals.var_nvtmf), 0.0, 0.0, 0.0, (locals.var_vbd_jct_dn10 / locals.var_nvtmf), 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign25540_e38948;
        locals.var_t0_dn3 = assign25540_e38948_d_n3;
        locals.var_t0_dn4 = assign25540_e38948_d_n4;
        locals.var_t0_dn5 = assign25540_e38948_d_n5;
        locals.var_t0_dn6 = assign25540_e38948_d_n6;
        locals.var_t0_dn7 = assign25540_e38948_d_n7;
        locals.var_t0_dn8 = assign25540_e38948_d_n8;
        locals.var_t0_dn9 = assign25540_e38948_d_n9;
        locals.var_t0_dn10 = assign25540_e38948_d_n10;
        locals.var_t0_dn11 = assign25540_e38948_d_n11;

        let (assign25550_e38956, assign25550_e38956_d_n3, assign25550_e38956_d_n4, assign25550_e38956_d_n5, assign25550_e38956_d_n6, assign25550_e38956_d_n7, assign25550_e38956_d_n8, assign25550_e38956_d_n9, assign25550_e38956_d_n10, assign25550_e38956_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard546 == 0.0)) {
        let assign25550_e38954: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign25550_e38954, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_t10, locals.var_t10_dn3, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11,)
    }
};
        locals.var_t10 = assign25550_e38956;
        locals.var_t10_dn3 = assign25550_e38956_d_n3;
        locals.var_t10_dn4 = assign25550_e38956_d_n4;
        locals.var_t10_dn5 = assign25550_e38956_d_n5;
        locals.var_t10_dn6 = assign25550_e38956_d_n6;
        locals.var_t10_dn7 = assign25550_e38956_d_n7;
        locals.var_t10_dn8 = assign25550_e38956_d_n8;
        locals.var_t10_dn9 = assign25550_e38956_d_n9;
        locals.var_t10_dn10 = assign25550_e38956_d_n10;
        locals.var_t10_dn11 = assign25550_e38956_d_n11;

        let assign25560_e38959: f64 = (locals.var_vrec0d_i - locals.var_vbd_jct);
        let assign25560_e38961: f64 = if assign25560_e38959 < 0.001 { 1.0 } else { 0.0 };
        locals.var_guard547 = assign25560_e38961;

        let (assign25570_e38970, assign25570_e38970_d_n3, assign25570_e38970_d_n4, assign25570_e38970_d_n5, assign25570_e38970_d_n6, assign25570_e38970_d_n7, assign25570_e38970_d_n8, assign25570_e38970_d_n9, assign25570_e38970_d_n10, assign25570_e38970_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard546 == 0.0)) && (locals.var_guard547 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign25570_e38970;
        locals.var_t1_dn3 = assign25570_e38970_d_n3;
        locals.var_t1_dn4 = assign25570_e38970_d_n4;
        locals.var_t1_dn5 = assign25570_e38970_d_n5;
        locals.var_t1_dn6 = assign25570_e38970_d_n6;
        locals.var_t1_dn7 = assign25570_e38970_d_n7;
        locals.var_t1_dn8 = assign25570_e38970_d_n8;
        locals.var_t1_dn9 = assign25570_e38970_d_n9;
        locals.var_t1_dn10 = assign25570_e38970_d_n10;
        locals.var_t1_dn11 = assign25570_e38970_d_n11;

        let (assign25580_e38986, assign25580_e38986_d_n3, assign25580_e38986_d_n4, assign25580_e38986_d_n5, assign25580_e38986_d_n6, assign25580_e38986_d_n7, assign25580_e38986_d_n8, assign25580_e38986_d_n9, assign25580_e38986_d_n10, assign25580_e38986_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard546 == 0.0)) && (locals.var_guard547 != 0.0)) {
        let assign25580_e38978: f64 = (-locals.var_vbd_jct);
        let assign25580_e38980: f64 = (assign25580_e38978 / locals.var_nvtmr);
        let assign25580_e38982: f64 = (assign25580_e38980 * locals.var_vrec0d_i);
        let assign25580_e38984: f64 = (assign25580_e38982 * locals.var_t1);
        (assign25580_e38984, (assign25580_e38982 * locals.var_t1_dn3), ((((-((assign25580_e38978 * locals.var_nvtmr_dn4) / (locals.var_nvtmr * locals.var_nvtmr))) * locals.var_vrec0d_i) * locals.var_t1) + (assign25580_e38982 * locals.var_t1_dn4)), ((((-((assign25580_e38978 * locals.var_nvtmr_dn5) / (locals.var_nvtmr * locals.var_nvtmr))) * locals.var_vrec0d_i) * locals.var_t1) + (assign25580_e38982 * locals.var_t1_dn5)), (((((-locals.var_vbd_jct_dn6) / locals.var_nvtmr) * locals.var_vrec0d_i) * locals.var_t1) + (assign25580_e38982 * locals.var_t1_dn6)), (assign25580_e38982 * locals.var_t1_dn7), (assign25580_e38982 * locals.var_t1_dn8), (assign25580_e38982 * locals.var_t1_dn9), (((((-locals.var_vbd_jct_dn10) / locals.var_nvtmr) * locals.var_vrec0d_i) * locals.var_t1) + (assign25580_e38982 * locals.var_t1_dn10)), (assign25580_e38982 * locals.var_t1_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign25580_e38986;
        locals.var_t0_dn3 = assign25580_e38986_d_n3;
        locals.var_t0_dn4 = assign25580_e38986_d_n4;
        locals.var_t0_dn5 = assign25580_e38986_d_n5;
        locals.var_t0_dn6 = assign25580_e38986_d_n6;
        locals.var_t0_dn7 = assign25580_e38986_d_n7;
        locals.var_t0_dn8 = assign25580_e38986_d_n8;
        locals.var_t0_dn9 = assign25580_e38986_d_n9;
        locals.var_t0_dn10 = assign25580_e38986_d_n10;
        locals.var_t0_dn11 = assign25580_e38986_d_n11;

        let (assign25590_e38996, assign25590_e38996_d_n3, assign25590_e38996_d_n4, assign25590_e38996_d_n5, assign25590_e38996_d_n6, assign25590_e38996_d_n7, assign25590_e38996_d_n8, assign25590_e38996_d_n9, assign25590_e38996_d_n10, assign25590_e38996_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard546 == 0.0)) && (locals.var_guard547 != 0.0)) {
        let assign25590_e38994: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign25590_e38994, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign25590_e38996;
        locals.var_t11_dn3 = assign25590_e38996_d_n3;
        locals.var_t11_dn4 = assign25590_e38996_d_n4;
        locals.var_t11_dn5 = assign25590_e38996_d_n5;
        locals.var_t11_dn6 = assign25590_e38996_d_n6;
        locals.var_t11_dn7 = assign25590_e38996_d_n7;
        locals.var_t11_dn8 = assign25590_e38996_d_n8;
        locals.var_t11_dn9 = assign25590_e38996_d_n9;
        locals.var_t11_dn10 = assign25590_e38996_d_n10;
        locals.var_t11_dn11 = assign25590_e38996_d_n11;

        let (assign25600_e39006, assign25600_e39006_d_n3, assign25600_e39006_d_n4, assign25600_e39006_d_n5, assign25600_e39006_d_n6, assign25600_e39006_d_n7, assign25600_e39006_d_n8, assign25600_e39006_d_n9, assign25600_e39006_d_n10, assign25600_e39006_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard546 == 0.0)) && (locals.var_guard547 != 0.0)) {
        let assign25600_e39004: f64 = (-locals.var_t11);
        (assign25600_e39004, (-locals.var_t11_dn3), (-locals.var_t11_dn4), (-locals.var_t11_dn5), (-locals.var_t11_dn6), (-locals.var_t11_dn7), (-locals.var_t11_dn8), (-locals.var_t11_dn9), (-locals.var_t11_dn10), (-locals.var_t11_dn11),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign25600_e39006;
        locals.var_t11_dn3 = assign25600_e39006_d_n3;
        locals.var_t11_dn4 = assign25600_e39006_d_n4;
        locals.var_t11_dn5 = assign25600_e39006_d_n5;
        locals.var_t11_dn6 = assign25600_e39006_d_n6;
        locals.var_t11_dn7 = assign25600_e39006_d_n7;
        locals.var_t11_dn8 = assign25600_e39006_d_n8;
        locals.var_t11_dn9 = assign25600_e39006_d_n9;
        locals.var_t11_dn10 = assign25600_e39006_d_n10;
        locals.var_t11_dn11 = assign25600_e39006_d_n11;

        let (assign25610_e39020, assign25610_e39020_d_n3, assign25610_e39020_d_n4, assign25610_e39020_d_n5, assign25610_e39020_d_n6, assign25610_e39020_d_n7, assign25610_e39020_d_n8, assign25610_e39020_d_n9, assign25610_e39020_d_n10, assign25610_e39020_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard546 == 0.0)) && (locals.var_guard547 == 0.0)) {
        let assign25610_e39017: f64 = (locals.var_vrec0d_i - locals.var_vbd_jct);
        let assign25610_e39018: f64 = (1.0 / assign25610_e39017);
        (assign25610_e39018, 0.0, 0.0, 0.0, (-((-locals.var_vbd_jct_dn6) / (assign25610_e39017 * assign25610_e39017))), 0.0, 0.0, 0.0, (-((-locals.var_vbd_jct_dn10) / (assign25610_e39017 * assign25610_e39017))), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign25610_e39020;
        locals.var_t1_dn3 = assign25610_e39020_d_n3;
        locals.var_t1_dn4 = assign25610_e39020_d_n4;
        locals.var_t1_dn5 = assign25610_e39020_d_n5;
        locals.var_t1_dn6 = assign25610_e39020_d_n6;
        locals.var_t1_dn7 = assign25610_e39020_d_n7;
        locals.var_t1_dn8 = assign25610_e39020_d_n8;
        locals.var_t1_dn9 = assign25610_e39020_d_n9;
        locals.var_t1_dn10 = assign25610_e39020_d_n10;
        locals.var_t1_dn11 = assign25610_e39020_d_n11;

        let (assign25620_e39037, assign25620_e39037_d_n3, assign25620_e39037_d_n4, assign25620_e39037_d_n5, assign25620_e39037_d_n6, assign25620_e39037_d_n7, assign25620_e39037_d_n8, assign25620_e39037_d_n9, assign25620_e39037_d_n10, assign25620_e39037_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard546 == 0.0)) && (locals.var_guard547 == 0.0)) {
        let assign25620_e39029: f64 = (-locals.var_vbd_jct);
        let assign25620_e39031: f64 = (assign25620_e39029 / locals.var_nvtmr);
        let assign25620_e39033: f64 = (assign25620_e39031 * locals.var_vrec0d_i);
        let assign25620_e39035: f64 = (assign25620_e39033 * locals.var_t1);
        (assign25620_e39035, (assign25620_e39033 * locals.var_t1_dn3), ((((-((assign25620_e39029 * locals.var_nvtmr_dn4) / (locals.var_nvtmr * locals.var_nvtmr))) * locals.var_vrec0d_i) * locals.var_t1) + (assign25620_e39033 * locals.var_t1_dn4)), ((((-((assign25620_e39029 * locals.var_nvtmr_dn5) / (locals.var_nvtmr * locals.var_nvtmr))) * locals.var_vrec0d_i) * locals.var_t1) + (assign25620_e39033 * locals.var_t1_dn5)), (((((-locals.var_vbd_jct_dn6) / locals.var_nvtmr) * locals.var_vrec0d_i) * locals.var_t1) + (assign25620_e39033 * locals.var_t1_dn6)), (assign25620_e39033 * locals.var_t1_dn7), (assign25620_e39033 * locals.var_t1_dn8), (assign25620_e39033 * locals.var_t1_dn9), (((((-locals.var_vbd_jct_dn10) / locals.var_nvtmr) * locals.var_vrec0d_i) * locals.var_t1) + (assign25620_e39033 * locals.var_t1_dn10)), (assign25620_e39033 * locals.var_t1_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign25620_e39037;
        locals.var_t0_dn3 = assign25620_e39037_d_n3;
        locals.var_t0_dn4 = assign25620_e39037_d_n4;
        locals.var_t0_dn5 = assign25620_e39037_d_n5;
        locals.var_t0_dn6 = assign25620_e39037_d_n6;
        locals.var_t0_dn7 = assign25620_e39037_d_n7;
        locals.var_t0_dn8 = assign25620_e39037_d_n8;
        locals.var_t0_dn9 = assign25620_e39037_d_n9;
        locals.var_t0_dn10 = assign25620_e39037_d_n10;
        locals.var_t0_dn11 = assign25620_e39037_d_n11;

        let (assign25630_e39048, assign25630_e39048_d_n3, assign25630_e39048_d_n4, assign25630_e39048_d_n5, assign25630_e39048_d_n6, assign25630_e39048_d_n7, assign25630_e39048_d_n8, assign25630_e39048_d_n9, assign25630_e39048_d_n10, assign25630_e39048_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard546 == 0.0)) && (locals.var_guard547 == 0.0)) {
        let assign25630_e39046: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign25630_e39046, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign25630_e39048;
        locals.var_t11_dn3 = assign25630_e39048_d_n3;
        locals.var_t11_dn4 = assign25630_e39048_d_n4;
        locals.var_t11_dn5 = assign25630_e39048_d_n5;
        locals.var_t11_dn6 = assign25630_e39048_d_n6;
        locals.var_t11_dn7 = assign25630_e39048_d_n7;
        locals.var_t11_dn8 = assign25630_e39048_d_n8;
        locals.var_t11_dn9 = assign25630_e39048_d_n9;
        locals.var_t11_dn10 = assign25630_e39048_d_n10;
        locals.var_t11_dn11 = assign25630_e39048_d_n11;

        let (assign25640_e39059, assign25640_e39059_d_n3, assign25640_e39059_d_n4, assign25640_e39059_d_n5, assign25640_e39059_d_n6, assign25640_e39059_d_n7, assign25640_e39059_d_n8, assign25640_e39059_d_n9, assign25640_e39059_d_n10, assign25640_e39059_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard546 == 0.0)) && (locals.var_guard547 == 0.0)) {
        let assign25640_e39057: f64 = (-locals.var_t11);
        (assign25640_e39057, (-locals.var_t11_dn3), (-locals.var_t11_dn4), (-locals.var_t11_dn5), (-locals.var_t11_dn6), (-locals.var_t11_dn7), (-locals.var_t11_dn8), (-locals.var_t11_dn9), (-locals.var_t11_dn10), (-locals.var_t11_dn11),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign25640_e39059;
        locals.var_t11_dn3 = assign25640_e39059_d_n3;
        locals.var_t11_dn4 = assign25640_e39059_d_n4;
        locals.var_t11_dn5 = assign25640_e39059_d_n5;
        locals.var_t11_dn6 = assign25640_e39059_d_n6;
        locals.var_t11_dn7 = assign25640_e39059_d_n7;
        locals.var_t11_dn8 = assign25640_e39059_d_n8;
        locals.var_t11_dn9 = assign25640_e39059_d_n9;
        locals.var_t11_dn10 = assign25640_e39059_d_n10;
        locals.var_t11_dn11 = assign25640_e39059_d_n11;

        let (assign25650_e39068, assign25650_e39068_d_n3, assign25650_e39068_d_n4, assign25650_e39068_d_n5, assign25650_e39068_d_n6, assign25650_e39068_d_n7, assign25650_e39068_d_n8, assign25650_e39068_d_n9, assign25650_e39068_d_n10, assign25650_e39068_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard546 == 0.0)) {
        let assign25650_e39066: f64 = (locals.var_wdtsi * locals.var_jrecd);
        (assign25650_e39066, (locals.var_wdtsi * locals.var_jrecd_dn3), (locals.var_wdtsi * locals.var_jrecd_dn4), (locals.var_wdtsi * locals.var_jrecd_dn5), (locals.var_wdtsi * locals.var_jrecd_dn6), (locals.var_wdtsi * locals.var_jrecd_dn7), (locals.var_wdtsi * locals.var_jrecd_dn8), (locals.var_wdtsi * locals.var_jrecd_dn9), (locals.var_wdtsi * locals.var_jrecd_dn10), (locals.var_wdtsi * locals.var_jrecd_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign25650_e39068;
        locals.var_t3_dn3 = assign25650_e39068_d_n3;
        locals.var_t3_dn4 = assign25650_e39068_d_n4;
        locals.var_t3_dn5 = assign25650_e39068_d_n5;
        locals.var_t3_dn6 = assign25650_e39068_d_n6;
        locals.var_t3_dn7 = assign25650_e39068_d_n7;
        locals.var_t3_dn8 = assign25650_e39068_d_n8;
        locals.var_t3_dn9 = assign25650_e39068_d_n9;
        locals.var_t3_dn10 = assign25650_e39068_d_n10;
        locals.var_t3_dn11 = assign25650_e39068_d_n11;

        let (assign25670_e39087,) = {
    if (locals.var_guard492 != 0.0) {
        let assign25670_e39083: f64 = (locals.var_weff / p.p1373);
        let assign25670_e39085: f64 = (assign25670_e39083 * p.p74);
        (assign25670_e39085,)
    } else {
        (locals.var_wtsi,)
    }
};
        locals.var_wtsi = assign25670_e39087;

        let assign25680_e39094: f64 = if ((locals.var_isbjt_i == 0.0) && (locals.var_idbjt_i == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard548 = assign25680_e39094;

        let (assign25720_e39123, assign25720_e39123_d_n3, assign25720_e39123_d_n4, assign25720_e39123_d_n5, assign25720_e39123_d_n6, assign25720_e39123_d_n7, assign25720_e39123_d_n8, assign25720_e39123_d_n9, assign25720_e39123_d_n10, assign25720_e39123_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) {
        let assign25720_e39119: f64 = (locals.var_xbjt_i * locals.var_t4);
        let assign25720_e39121: f64 = (assign25720_e39119 / locals.var_ndiode_i);
        (assign25720_e39121, ((locals.var_xbjt_i * locals.var_t4_dn3) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn4) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn5) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn6) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn7) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn8) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn9) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn10) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn11) / locals.var_ndiode_i),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign25720_e39123;
        locals.var_t7_dn3 = assign25720_e39123_d_n3;
        locals.var_t7_dn4 = assign25720_e39123_d_n4;
        locals.var_t7_dn5 = assign25720_e39123_d_n5;
        locals.var_t7_dn6 = assign25720_e39123_d_n6;
        locals.var_t7_dn7 = assign25720_e39123_d_n7;
        locals.var_t7_dn8 = assign25720_e39123_d_n8;
        locals.var_t7_dn9 = assign25720_e39123_d_n9;
        locals.var_t7_dn10 = assign25720_e39123_d_n10;
        locals.var_t7_dn11 = assign25720_e39123_d_n11;

        let (assign25730_e39131, assign25730_e39131_d_n3, assign25730_e39131_d_n4, assign25730_e39131_d_n5, assign25730_e39131_d_n6, assign25730_e39131_d_n7, assign25730_e39131_d_n8, assign25730_e39131_d_n9, assign25730_e39131_d_n10, assign25730_e39131_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) {
        let assign25730_e39129: f64 = { let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign25730_e39129, ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn3), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn4), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn5), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn6), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn7), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn8), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn9), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn10), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign25730_e39131;
        locals.var_t0_dn3 = assign25730_e39131_d_n3;
        locals.var_t0_dn4 = assign25730_e39131_d_n4;
        locals.var_t0_dn5 = assign25730_e39131_d_n5;
        locals.var_t0_dn6 = assign25730_e39131_d_n6;
        locals.var_t0_dn7 = assign25730_e39131_d_n7;
        locals.var_t0_dn8 = assign25730_e39131_d_n8;
        locals.var_t0_dn9 = assign25730_e39131_d_n9;
        locals.var_t0_dn10 = assign25730_e39131_d_n10;
        locals.var_t0_dn11 = assign25730_e39131_d_n11;

        let (assign25740_e39140, assign25740_e39140_d_n3, assign25740_e39140_d_n4, assign25740_e39140_d_n5, assign25740_e39140_d_n6, assign25740_e39140_d_n7, assign25740_e39140_d_n8, assign25740_e39140_d_n9, assign25740_e39140_d_n10, assign25740_e39140_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) {
        let assign25740_e39138: f64 = (locals.var_ahli_i * locals.var_t0);
        (assign25740_e39138, (locals.var_ahli_i * locals.var_t0_dn3), (locals.var_ahli_i * locals.var_t0_dn4), (locals.var_ahli_i * locals.var_t0_dn5), (locals.var_ahli_i * locals.var_t0_dn6), (locals.var_ahli_i * locals.var_t0_dn7), (locals.var_ahli_i * locals.var_t0_dn8), (locals.var_ahli_i * locals.var_t0_dn9), (locals.var_ahli_i * locals.var_t0_dn10), (locals.var_ahli_i * locals.var_t0_dn11),)
    } else {
        (locals.var_ahlis, locals.var_ahlis_dn3, locals.var_ahlis_dn4, locals.var_ahlis_dn5, locals.var_ahlis_dn6, locals.var_ahlis_dn7, locals.var_ahlis_dn8, locals.var_ahlis_dn9, locals.var_ahlis_dn10, locals.var_ahlis_dn11,)
    }
};
        locals.var_ahlis = assign25740_e39140;
        locals.var_ahlis_dn3 = assign25740_e39140_d_n3;
        locals.var_ahlis_dn4 = assign25740_e39140_d_n4;
        locals.var_ahlis_dn5 = assign25740_e39140_d_n5;
        locals.var_ahlis_dn6 = assign25740_e39140_d_n6;
        locals.var_ahlis_dn7 = assign25740_e39140_d_n7;
        locals.var_ahlis_dn8 = assign25740_e39140_d_n8;
        locals.var_ahlis_dn9 = assign25740_e39140_d_n9;
        locals.var_ahlis_dn10 = assign25740_e39140_d_n10;
        locals.var_ahlis_dn11 = assign25740_e39140_d_n11;

        let (assign25750_e39149, assign25750_e39149_d_n3, assign25750_e39149_d_n4, assign25750_e39149_d_n5, assign25750_e39149_d_n6, assign25750_e39149_d_n7, assign25750_e39149_d_n8, assign25750_e39149_d_n9, assign25750_e39149_d_n10, assign25750_e39149_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) {
        let assign25750_e39147: f64 = (locals.var_isbjt_i * locals.var_t0);
        (assign25750_e39147, (locals.var_isbjt_i * locals.var_t0_dn3), (locals.var_isbjt_i * locals.var_t0_dn4), (locals.var_isbjt_i * locals.var_t0_dn5), (locals.var_isbjt_i * locals.var_t0_dn6), (locals.var_isbjt_i * locals.var_t0_dn7), (locals.var_isbjt_i * locals.var_t0_dn8), (locals.var_isbjt_i * locals.var_t0_dn9), (locals.var_isbjt_i * locals.var_t0_dn10), (locals.var_isbjt_i * locals.var_t0_dn11),)
    } else {
        (locals.var_jbjts, locals.var_jbjts_dn3, locals.var_jbjts_dn4, locals.var_jbjts_dn5, locals.var_jbjts_dn6, locals.var_jbjts_dn7, locals.var_jbjts_dn8, locals.var_jbjts_dn9, locals.var_jbjts_dn10, locals.var_jbjts_dn11,)
    }
};
        locals.var_jbjts = assign25750_e39149;
        locals.var_jbjts_dn3 = assign25750_e39149_d_n3;
        locals.var_jbjts_dn4 = assign25750_e39149_d_n4;
        locals.var_jbjts_dn5 = assign25750_e39149_d_n5;
        locals.var_jbjts_dn6 = assign25750_e39149_d_n6;
        locals.var_jbjts_dn7 = assign25750_e39149_d_n7;
        locals.var_jbjts_dn8 = assign25750_e39149_d_n8;
        locals.var_jbjts_dn9 = assign25750_e39149_d_n9;
        locals.var_jbjts_dn10 = assign25750_e39149_d_n10;
        locals.var_jbjts_dn11 = assign25750_e39149_d_n11;

        let (assign25760_e39160, assign25760_e39160_d_n3, assign25760_e39160_d_n4, assign25760_e39160_d_n5, assign25760_e39160_d_n6, assign25760_e39160_d_n7, assign25760_e39160_d_n8, assign25760_e39160_d_n9, assign25760_e39160_d_n10, assign25760_e39160_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) {
        let assign25760_e39156: f64 = (locals.var_xbjt_i * locals.var_t4);
        let assign25760_e39158: f64 = (assign25760_e39156 / locals.var_ndiode_i);
        (assign25760_e39158, ((locals.var_xbjt_i * locals.var_t4_dn3) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn4) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn5) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn6) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn7) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn8) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn9) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn10) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn11) / locals.var_ndiode_i),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign25760_e39160;
        locals.var_t7_dn3 = assign25760_e39160_d_n3;
        locals.var_t7_dn4 = assign25760_e39160_d_n4;
        locals.var_t7_dn5 = assign25760_e39160_d_n5;
        locals.var_t7_dn6 = assign25760_e39160_d_n6;
        locals.var_t7_dn7 = assign25760_e39160_d_n7;
        locals.var_t7_dn8 = assign25760_e39160_d_n8;
        locals.var_t7_dn9 = assign25760_e39160_d_n9;
        locals.var_t7_dn10 = assign25760_e39160_d_n10;
        locals.var_t7_dn11 = assign25760_e39160_d_n11;

        let (assign25770_e39168, assign25770_e39168_d_n3, assign25770_e39168_d_n4, assign25770_e39168_d_n5, assign25770_e39168_d_n6, assign25770_e39168_d_n7, assign25770_e39168_d_n8, assign25770_e39168_d_n9, assign25770_e39168_d_n10, assign25770_e39168_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) {
        let assign25770_e39166: f64 = { let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign25770_e39166, ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn3), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn4), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn5), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn6), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn7), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn8), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn9), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn10), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign25770_e39168;
        locals.var_t0_dn3 = assign25770_e39168_d_n3;
        locals.var_t0_dn4 = assign25770_e39168_d_n4;
        locals.var_t0_dn5 = assign25770_e39168_d_n5;
        locals.var_t0_dn6 = assign25770_e39168_d_n6;
        locals.var_t0_dn7 = assign25770_e39168_d_n7;
        locals.var_t0_dn8 = assign25770_e39168_d_n8;
        locals.var_t0_dn9 = assign25770_e39168_d_n9;
        locals.var_t0_dn10 = assign25770_e39168_d_n10;
        locals.var_t0_dn11 = assign25770_e39168_d_n11;

        let (assign25780_e39177, assign25780_e39177_d_n3, assign25780_e39177_d_n4, assign25780_e39177_d_n5, assign25780_e39177_d_n6, assign25780_e39177_d_n7, assign25780_e39177_d_n8, assign25780_e39177_d_n9, assign25780_e39177_d_n10, assign25780_e39177_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) {
        let assign25780_e39175: f64 = (locals.var_ahlid_i * locals.var_t0);
        (assign25780_e39175, (locals.var_ahlid_i * locals.var_t0_dn3), (locals.var_ahlid_i * locals.var_t0_dn4), (locals.var_ahlid_i * locals.var_t0_dn5), (locals.var_ahlid_i * locals.var_t0_dn6), (locals.var_ahlid_i * locals.var_t0_dn7), (locals.var_ahlid_i * locals.var_t0_dn8), (locals.var_ahlid_i * locals.var_t0_dn9), (locals.var_ahlid_i * locals.var_t0_dn10), (locals.var_ahlid_i * locals.var_t0_dn11),)
    } else {
        (locals.var_ahlid, locals.var_ahlid_dn3, locals.var_ahlid_dn4, locals.var_ahlid_dn5, locals.var_ahlid_dn6, locals.var_ahlid_dn7, locals.var_ahlid_dn8, locals.var_ahlid_dn9, locals.var_ahlid_dn10, locals.var_ahlid_dn11,)
    }
};
        locals.var_ahlid = assign25780_e39177;
        locals.var_ahlid_dn3 = assign25780_e39177_d_n3;
        locals.var_ahlid_dn4 = assign25780_e39177_d_n4;
        locals.var_ahlid_dn5 = assign25780_e39177_d_n5;
        locals.var_ahlid_dn6 = assign25780_e39177_d_n6;
        locals.var_ahlid_dn7 = assign25780_e39177_d_n7;
        locals.var_ahlid_dn8 = assign25780_e39177_d_n8;
        locals.var_ahlid_dn9 = assign25780_e39177_d_n9;
        locals.var_ahlid_dn10 = assign25780_e39177_d_n10;
        locals.var_ahlid_dn11 = assign25780_e39177_d_n11;

        let (assign25790_e39186, assign25790_e39186_d_n3, assign25790_e39186_d_n4, assign25790_e39186_d_n5, assign25790_e39186_d_n6, assign25790_e39186_d_n7, assign25790_e39186_d_n8, assign25790_e39186_d_n9, assign25790_e39186_d_n10, assign25790_e39186_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) {
        let assign25790_e39184: f64 = (locals.var_idbjt_i * locals.var_t0);
        (assign25790_e39184, (locals.var_idbjt_i * locals.var_t0_dn3), (locals.var_idbjt_i * locals.var_t0_dn4), (locals.var_idbjt_i * locals.var_t0_dn5), (locals.var_idbjt_i * locals.var_t0_dn6), (locals.var_idbjt_i * locals.var_t0_dn7), (locals.var_idbjt_i * locals.var_t0_dn8), (locals.var_idbjt_i * locals.var_t0_dn9), (locals.var_idbjt_i * locals.var_t0_dn10), (locals.var_idbjt_i * locals.var_t0_dn11),)
    } else {
        (locals.var_jbjtd, locals.var_jbjtd_dn3, locals.var_jbjtd_dn4, locals.var_jbjtd_dn5, locals.var_jbjtd_dn6, locals.var_jbjtd_dn7, locals.var_jbjtd_dn8, locals.var_jbjtd_dn9, locals.var_jbjtd_dn10, locals.var_jbjtd_dn11,)
    }
};
        locals.var_jbjtd = assign25790_e39186;
        locals.var_jbjtd_dn3 = assign25790_e39186_d_n3;
        locals.var_jbjtd_dn4 = assign25790_e39186_d_n4;
        locals.var_jbjtd_dn5 = assign25790_e39186_d_n5;
        locals.var_jbjtd_dn6 = assign25790_e39186_d_n6;
        locals.var_jbjtd_dn7 = assign25790_e39186_d_n7;
        locals.var_jbjtd_dn8 = assign25790_e39186_d_n8;
        locals.var_jbjtd_dn9 = assign25790_e39186_d_n9;
        locals.var_jbjtd_dn10 = assign25790_e39186_d_n10;
        locals.var_jbjtd_dn11 = assign25790_e39186_d_n11;

    }

    pub(super) fn stamp_transient_block_68(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign25800_e39197, assign25800_e39197_d_n3, assign25800_e39197_d_n4, assign25800_e39197_d_n5, assign25800_e39197_d_n6, assign25800_e39197_d_n7, assign25800_e39197_d_n8, assign25800_e39197_d_n9, assign25800_e39197_d_n10, assign25800_e39197_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) {
        let assign25800_e39194: f64 = (locals.var_expvbsnvtm - 1.0);
        let assign25800_e39195: f64 = (locals.var_ahlis * assign25800_e39194);
        (assign25800_e39195, ((locals.var_ahlis_dn3 * assign25800_e39194) + (locals.var_ahlis * locals.var_expvbsnvtm_dn3)), ((locals.var_ahlis_dn4 * assign25800_e39194) + (locals.var_ahlis * locals.var_expvbsnvtm_dn4)), ((locals.var_ahlis_dn5 * assign25800_e39194) + (locals.var_ahlis * locals.var_expvbsnvtm_dn5)), ((locals.var_ahlis_dn6 * assign25800_e39194) + (locals.var_ahlis * locals.var_expvbsnvtm_dn6)), ((locals.var_ahlis_dn7 * assign25800_e39194) + (locals.var_ahlis * locals.var_expvbsnvtm_dn7)), ((locals.var_ahlis_dn8 * assign25800_e39194) + (locals.var_ahlis * locals.var_expvbsnvtm_dn8)), ((locals.var_ahlis_dn9 * assign25800_e39194) + (locals.var_ahlis * locals.var_expvbsnvtm_dn9)), ((locals.var_ahlis_dn10 * assign25800_e39194) + (locals.var_ahlis * locals.var_expvbsnvtm_dn10)), ((locals.var_ahlis_dn11 * assign25800_e39194) + (locals.var_ahlis * locals.var_expvbsnvtm_dn11)),)
    } else {
        (locals.var_ehlis, locals.var_ehlis_dn3, locals.var_ehlis_dn4, locals.var_ehlis_dn5, locals.var_ehlis_dn6, locals.var_ehlis_dn7, locals.var_ehlis_dn8, locals.var_ehlis_dn9, locals.var_ehlis_dn10, locals.var_ehlis_dn11,)
    }
};
        locals.var_ehlis = assign25800_e39197;
        locals.var_ehlis_dn3 = assign25800_e39197_d_n3;
        locals.var_ehlis_dn4 = assign25800_e39197_d_n4;
        locals.var_ehlis_dn5 = assign25800_e39197_d_n5;
        locals.var_ehlis_dn6 = assign25800_e39197_d_n6;
        locals.var_ehlis_dn7 = assign25800_e39197_d_n7;
        locals.var_ehlis_dn8 = assign25800_e39197_d_n8;
        locals.var_ehlis_dn9 = assign25800_e39197_d_n9;
        locals.var_ehlis_dn10 = assign25800_e39197_d_n10;
        locals.var_ehlis_dn11 = assign25800_e39197_d_n11;

        let assign25810_e39200: f64 = if locals.var_ehlis < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard549 = assign25810_e39200;

        let (assign25820_e39209, assign25820_e39209_d_n3, assign25820_e39209_d_n4, assign25820_e39209_d_n5, assign25820_e39209_d_n6, assign25820_e39209_d_n7, assign25820_e39209_d_n8, assign25820_e39209_d_n9, assign25820_e39209_d_n10, assign25820_e39209_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) && (locals.var_guard549 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ehlis, locals.var_ehlis_dn3, locals.var_ehlis_dn4, locals.var_ehlis_dn5, locals.var_ehlis_dn6, locals.var_ehlis_dn7, locals.var_ehlis_dn8, locals.var_ehlis_dn9, locals.var_ehlis_dn10, locals.var_ehlis_dn11,)
    }
};
        locals.var_ehlis = assign25820_e39209;
        locals.var_ehlis_dn3 = assign25820_e39209_d_n3;
        locals.var_ehlis_dn4 = assign25820_e39209_d_n4;
        locals.var_ehlis_dn5 = assign25820_e39209_d_n5;
        locals.var_ehlis_dn6 = assign25820_e39209_d_n6;
        locals.var_ehlis_dn7 = assign25820_e39209_d_n7;
        locals.var_ehlis_dn8 = assign25820_e39209_d_n8;
        locals.var_ehlis_dn9 = assign25820_e39209_d_n9;
        locals.var_ehlis_dn10 = assign25820_e39209_d_n10;
        locals.var_ehlis_dn11 = assign25820_e39209_d_n11;

        let (assign25830_e39218, assign25830_e39218_d_n3, assign25830_e39218_d_n4, assign25830_e39218_d_n5, assign25830_e39218_d_n6, assign25830_e39218_d_n7, assign25830_e39218_d_n8, assign25830_e39218_d_n9, assign25830_e39218_d_n10, assign25830_e39218_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) && (locals.var_guard549 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ehlisfactor, locals.var_ehlisfactor_dn3, locals.var_ehlisfactor_dn4, locals.var_ehlisfactor_dn5, locals.var_ehlisfactor_dn6, locals.var_ehlisfactor_dn7, locals.var_ehlisfactor_dn8, locals.var_ehlisfactor_dn9, locals.var_ehlisfactor_dn10, locals.var_ehlisfactor_dn11,)
    }
};
        locals.var_ehlisfactor = assign25830_e39218;
        locals.var_ehlisfactor_dn3 = assign25830_e39218_d_n3;
        locals.var_ehlisfactor_dn4 = assign25830_e39218_d_n4;
        locals.var_ehlisfactor_dn5 = assign25830_e39218_d_n5;
        locals.var_ehlisfactor_dn6 = assign25830_e39218_d_n6;
        locals.var_ehlisfactor_dn7 = assign25830_e39218_d_n7;
        locals.var_ehlisfactor_dn8 = assign25830_e39218_d_n8;
        locals.var_ehlisfactor_dn9 = assign25830_e39218_d_n9;
        locals.var_ehlisfactor_dn10 = assign25830_e39218_d_n10;
        locals.var_ehlisfactor_dn11 = assign25830_e39218_d_n11;

        let (assign25840_e39233, assign25840_e39233_d_n3, assign25840_e39233_d_n4, assign25840_e39233_d_n5, assign25840_e39233_d_n6, assign25840_e39233_d_n7, assign25840_e39233_d_n8, assign25840_e39233_d_n9, assign25840_e39233_d_n10, assign25840_e39233_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) && (locals.var_guard549 == 0.0)) {
        let assign25840_e39229: f64 = (1.0 + locals.var_ehlis);
        let assign25840_e39230: f64 = (assign25840_e39229).sqrt();
        let assign25840_e39231: f64 = (1.0 / assign25840_e39230);
        (assign25840_e39231, (-((locals.var_ehlis_dn3 / (2.0 * assign25840_e39230)) / (assign25840_e39230 * assign25840_e39230))), (-((locals.var_ehlis_dn4 / (2.0 * assign25840_e39230)) / (assign25840_e39230 * assign25840_e39230))), (-((locals.var_ehlis_dn5 / (2.0 * assign25840_e39230)) / (assign25840_e39230 * assign25840_e39230))), (-((locals.var_ehlis_dn6 / (2.0 * assign25840_e39230)) / (assign25840_e39230 * assign25840_e39230))), (-((locals.var_ehlis_dn7 / (2.0 * assign25840_e39230)) / (assign25840_e39230 * assign25840_e39230))), (-((locals.var_ehlis_dn8 / (2.0 * assign25840_e39230)) / (assign25840_e39230 * assign25840_e39230))), (-((locals.var_ehlis_dn9 / (2.0 * assign25840_e39230)) / (assign25840_e39230 * assign25840_e39230))), (-((locals.var_ehlis_dn10 / (2.0 * assign25840_e39230)) / (assign25840_e39230 * assign25840_e39230))), (-((locals.var_ehlis_dn11 / (2.0 * assign25840_e39230)) / (assign25840_e39230 * assign25840_e39230))),)
    } else {
        (locals.var_ehlisfactor, locals.var_ehlisfactor_dn3, locals.var_ehlisfactor_dn4, locals.var_ehlisfactor_dn5, locals.var_ehlisfactor_dn6, locals.var_ehlisfactor_dn7, locals.var_ehlisfactor_dn8, locals.var_ehlisfactor_dn9, locals.var_ehlisfactor_dn10, locals.var_ehlisfactor_dn11,)
    }
};
        locals.var_ehlisfactor = assign25840_e39233;
        locals.var_ehlisfactor_dn3 = assign25840_e39233_d_n3;
        locals.var_ehlisfactor_dn4 = assign25840_e39233_d_n4;
        locals.var_ehlisfactor_dn5 = assign25840_e39233_d_n5;
        locals.var_ehlisfactor_dn6 = assign25840_e39233_d_n6;
        locals.var_ehlisfactor_dn7 = assign25840_e39233_d_n7;
        locals.var_ehlisfactor_dn8 = assign25840_e39233_d_n8;
        locals.var_ehlisfactor_dn9 = assign25840_e39233_d_n9;
        locals.var_ehlisfactor_dn10 = assign25840_e39233_d_n10;
        locals.var_ehlisfactor_dn11 = assign25840_e39233_d_n11;

        let (assign25850_e39244, assign25850_e39244_d_n3, assign25850_e39244_d_n4, assign25850_e39244_d_n5, assign25850_e39244_d_n6, assign25850_e39244_d_n7, assign25850_e39244_d_n8, assign25850_e39244_d_n9, assign25850_e39244_d_n10, assign25850_e39244_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) {
        let assign25850_e39241: f64 = (locals.var_expvbdnvtm - 1.0);
        let assign25850_e39242: f64 = (locals.var_ahlid * assign25850_e39241);
        (assign25850_e39242, ((locals.var_ahlid_dn3 * assign25850_e39241) + (locals.var_ahlid * locals.var_expvbdnvtm_dn3)), ((locals.var_ahlid_dn4 * assign25850_e39241) + (locals.var_ahlid * locals.var_expvbdnvtm_dn4)), ((locals.var_ahlid_dn5 * assign25850_e39241) + (locals.var_ahlid * locals.var_expvbdnvtm_dn5)), ((locals.var_ahlid_dn6 * assign25850_e39241) + (locals.var_ahlid * locals.var_expvbdnvtm_dn6)), ((locals.var_ahlid_dn7 * assign25850_e39241) + (locals.var_ahlid * locals.var_expvbdnvtm_dn7)), ((locals.var_ahlid_dn8 * assign25850_e39241) + (locals.var_ahlid * locals.var_expvbdnvtm_dn8)), ((locals.var_ahlid_dn9 * assign25850_e39241) + (locals.var_ahlid * locals.var_expvbdnvtm_dn9)), ((locals.var_ahlid_dn10 * assign25850_e39241) + (locals.var_ahlid * locals.var_expvbdnvtm_dn10)), ((locals.var_ahlid_dn11 * assign25850_e39241) + (locals.var_ahlid * locals.var_expvbdnvtm_dn11)),)
    } else {
        (locals.var_ehlid, locals.var_ehlid_dn3, locals.var_ehlid_dn4, locals.var_ehlid_dn5, locals.var_ehlid_dn6, locals.var_ehlid_dn7, locals.var_ehlid_dn8, locals.var_ehlid_dn9, locals.var_ehlid_dn10, locals.var_ehlid_dn11,)
    }
};
        locals.var_ehlid = assign25850_e39244;
        locals.var_ehlid_dn3 = assign25850_e39244_d_n3;
        locals.var_ehlid_dn4 = assign25850_e39244_d_n4;
        locals.var_ehlid_dn5 = assign25850_e39244_d_n5;
        locals.var_ehlid_dn6 = assign25850_e39244_d_n6;
        locals.var_ehlid_dn7 = assign25850_e39244_d_n7;
        locals.var_ehlid_dn8 = assign25850_e39244_d_n8;
        locals.var_ehlid_dn9 = assign25850_e39244_d_n9;
        locals.var_ehlid_dn10 = assign25850_e39244_d_n10;
        locals.var_ehlid_dn11 = assign25850_e39244_d_n11;

        let assign25860_e39247: f64 = if locals.var_ehlid < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard550 = assign25860_e39247;

        let (assign25870_e39256, assign25870_e39256_d_n3, assign25870_e39256_d_n4, assign25870_e39256_d_n5, assign25870_e39256_d_n6, assign25870_e39256_d_n7, assign25870_e39256_d_n8, assign25870_e39256_d_n9, assign25870_e39256_d_n10, assign25870_e39256_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) && (locals.var_guard550 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ehlid, locals.var_ehlid_dn3, locals.var_ehlid_dn4, locals.var_ehlid_dn5, locals.var_ehlid_dn6, locals.var_ehlid_dn7, locals.var_ehlid_dn8, locals.var_ehlid_dn9, locals.var_ehlid_dn10, locals.var_ehlid_dn11,)
    }
};
        locals.var_ehlid = assign25870_e39256;
        locals.var_ehlid_dn3 = assign25870_e39256_d_n3;
        locals.var_ehlid_dn4 = assign25870_e39256_d_n4;
        locals.var_ehlid_dn5 = assign25870_e39256_d_n5;
        locals.var_ehlid_dn6 = assign25870_e39256_d_n6;
        locals.var_ehlid_dn7 = assign25870_e39256_d_n7;
        locals.var_ehlid_dn8 = assign25870_e39256_d_n8;
        locals.var_ehlid_dn9 = assign25870_e39256_d_n9;
        locals.var_ehlid_dn10 = assign25870_e39256_d_n10;
        locals.var_ehlid_dn11 = assign25870_e39256_d_n11;

        let (assign25880_e39265, assign25880_e39265_d_n3, assign25880_e39265_d_n4, assign25880_e39265_d_n5, assign25880_e39265_d_n6, assign25880_e39265_d_n7, assign25880_e39265_d_n8, assign25880_e39265_d_n9, assign25880_e39265_d_n10, assign25880_e39265_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) && (locals.var_guard550 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ehlidfactor, locals.var_ehlidfactor_dn3, locals.var_ehlidfactor_dn4, locals.var_ehlidfactor_dn5, locals.var_ehlidfactor_dn6, locals.var_ehlidfactor_dn7, locals.var_ehlidfactor_dn8, locals.var_ehlidfactor_dn9, locals.var_ehlidfactor_dn10, locals.var_ehlidfactor_dn11,)
    }
};
        locals.var_ehlidfactor = assign25880_e39265;
        locals.var_ehlidfactor_dn3 = assign25880_e39265_d_n3;
        locals.var_ehlidfactor_dn4 = assign25880_e39265_d_n4;
        locals.var_ehlidfactor_dn5 = assign25880_e39265_d_n5;
        locals.var_ehlidfactor_dn6 = assign25880_e39265_d_n6;
        locals.var_ehlidfactor_dn7 = assign25880_e39265_d_n7;
        locals.var_ehlidfactor_dn8 = assign25880_e39265_d_n8;
        locals.var_ehlidfactor_dn9 = assign25880_e39265_d_n9;
        locals.var_ehlidfactor_dn10 = assign25880_e39265_d_n10;
        locals.var_ehlidfactor_dn11 = assign25880_e39265_d_n11;

        let (assign25890_e39280, assign25890_e39280_d_n3, assign25890_e39280_d_n4, assign25890_e39280_d_n5, assign25890_e39280_d_n6, assign25890_e39280_d_n7, assign25890_e39280_d_n8, assign25890_e39280_d_n9, assign25890_e39280_d_n10, assign25890_e39280_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) && (locals.var_guard550 == 0.0)) {
        let assign25890_e39276: f64 = (1.0 + locals.var_ehlid);
        let assign25890_e39277: f64 = (assign25890_e39276).sqrt();
        let assign25890_e39278: f64 = (1.0 / assign25890_e39277);
        (assign25890_e39278, (-((locals.var_ehlid_dn3 / (2.0 * assign25890_e39277)) / (assign25890_e39277 * assign25890_e39277))), (-((locals.var_ehlid_dn4 / (2.0 * assign25890_e39277)) / (assign25890_e39277 * assign25890_e39277))), (-((locals.var_ehlid_dn5 / (2.0 * assign25890_e39277)) / (assign25890_e39277 * assign25890_e39277))), (-((locals.var_ehlid_dn6 / (2.0 * assign25890_e39277)) / (assign25890_e39277 * assign25890_e39277))), (-((locals.var_ehlid_dn7 / (2.0 * assign25890_e39277)) / (assign25890_e39277 * assign25890_e39277))), (-((locals.var_ehlid_dn8 / (2.0 * assign25890_e39277)) / (assign25890_e39277 * assign25890_e39277))), (-((locals.var_ehlid_dn9 / (2.0 * assign25890_e39277)) / (assign25890_e39277 * assign25890_e39277))), (-((locals.var_ehlid_dn10 / (2.0 * assign25890_e39277)) / (assign25890_e39277 * assign25890_e39277))), (-((locals.var_ehlid_dn11 / (2.0 * assign25890_e39277)) / (assign25890_e39277 * assign25890_e39277))),)
    } else {
        (locals.var_ehlidfactor, locals.var_ehlidfactor_dn3, locals.var_ehlidfactor_dn4, locals.var_ehlidfactor_dn5, locals.var_ehlidfactor_dn6, locals.var_ehlidfactor_dn7, locals.var_ehlidfactor_dn8, locals.var_ehlidfactor_dn9, locals.var_ehlidfactor_dn10, locals.var_ehlidfactor_dn11,)
    }
};
        locals.var_ehlidfactor = assign25890_e39280;
        locals.var_ehlidfactor_dn3 = assign25890_e39280_d_n3;
        locals.var_ehlidfactor_dn4 = assign25890_e39280_d_n4;
        locals.var_ehlidfactor_dn5 = assign25890_e39280_d_n5;
        locals.var_ehlidfactor_dn6 = assign25890_e39280_d_n6;
        locals.var_ehlidfactor_dn7 = assign25890_e39280_d_n7;
        locals.var_ehlidfactor_dn8 = assign25890_e39280_d_n8;
        locals.var_ehlidfactor_dn9 = assign25890_e39280_d_n9;
        locals.var_ehlidfactor_dn10 = assign25890_e39280_d_n10;
        locals.var_ehlidfactor_dn11 = assign25890_e39280_d_n11;

        let (assign25900_e39296, assign25900_e39296_d_n3, assign25900_e39296_d_n4, assign25900_e39296_d_n5, assign25900_e39296_d_n6, assign25900_e39296_d_n7, assign25900_e39296_d_n8, assign25900_e39296_d_n9, assign25900_e39296_d_n10, assign25900_e39296_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) {
        let assign25900_e39286: f64 = (-0.5);
        let assign25900_e39288: f64 = (assign25900_e39286 * locals.var_leff);
        let assign25900_e39290: f64 = (assign25900_e39288 * locals.var_leff);
        let __rspice_inv_cse_0: f64 = 1.0 / p.p595;
        let assign25900_e39292: f64 = (assign25900_e39290 * __rspice_inv_cse_0);
        let assign25900_e39294: f64 = (assign25900_e39292 * __rspice_inv_cse_0);
        (assign25900_e39294, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign25900_e39296;
        locals.var_t0_dn3 = assign25900_e39296_d_n3;
        locals.var_t0_dn4 = assign25900_e39296_d_n4;
        locals.var_t0_dn5 = assign25900_e39296_d_n5;
        locals.var_t0_dn6 = assign25900_e39296_d_n6;
        locals.var_t0_dn7 = assign25900_e39296_d_n7;
        locals.var_t0_dn8 = assign25900_e39296_d_n8;
        locals.var_t0_dn9 = assign25900_e39296_d_n9;
        locals.var_t0_dn10 = assign25900_e39296_d_n10;
        locals.var_t0_dn11 = assign25900_e39296_d_n11;

        let (assign25910_e39304, assign25910_e39304_d_n3, assign25910_e39304_d_n4, assign25910_e39304_d_n5, assign25910_e39304_d_n6, assign25910_e39304_d_n7, assign25910_e39304_d_n8, assign25910_e39304_d_n9, assign25910_e39304_d_n10, assign25910_e39304_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) {
        let assign25910_e39302: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign25910_e39302, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_alphabjt, locals.var_alphabjt_dn3, locals.var_alphabjt_dn4, locals.var_alphabjt_dn5, locals.var_alphabjt_dn6, locals.var_alphabjt_dn7, locals.var_alphabjt_dn8, locals.var_alphabjt_dn9, locals.var_alphabjt_dn10, locals.var_alphabjt_dn11,)
    }
};
        locals.var_alphabjt = assign25910_e39304;
        locals.var_alphabjt_dn3 = assign25910_e39304_d_n3;
        locals.var_alphabjt_dn4 = assign25910_e39304_d_n4;
        locals.var_alphabjt_dn5 = assign25910_e39304_d_n5;
        locals.var_alphabjt_dn6 = assign25910_e39304_d_n6;
        locals.var_alphabjt_dn7 = assign25910_e39304_d_n7;
        locals.var_alphabjt_dn8 = assign25910_e39304_d_n8;
        locals.var_alphabjt_dn9 = assign25910_e39304_d_n9;
        locals.var_alphabjt_dn10 = assign25910_e39304_d_n10;
        locals.var_alphabjt_dn11 = assign25910_e39304_d_n11;

        let (assign25920_e39313, assign25920_e39313_d_n3, assign25920_e39313_d_n4, assign25920_e39313_d_n5, assign25920_e39313_d_n6, assign25920_e39313_d_n7, assign25920_e39313_d_n8, assign25920_e39313_d_n9, assign25920_e39313_d_n10, assign25920_e39313_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) {
        let assign25920_e39311: f64 = (1.0 - locals.var_alphabjt);
        (assign25920_e39311, (-locals.var_alphabjt_dn3), (-locals.var_alphabjt_dn4), (-locals.var_alphabjt_dn5), (-locals.var_alphabjt_dn6), (-locals.var_alphabjt_dn7), (-locals.var_alphabjt_dn8), (-locals.var_alphabjt_dn9), (-locals.var_alphabjt_dn10), (-locals.var_alphabjt_dn11),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign25920_e39313;
        locals.var_t2_dn3 = assign25920_e39313_d_n3;
        locals.var_t2_dn4 = assign25920_e39313_d_n4;
        locals.var_t2_dn5 = assign25920_e39313_d_n5;
        locals.var_t2_dn6 = assign25920_e39313_d_n6;
        locals.var_t2_dn7 = assign25920_e39313_d_n7;
        locals.var_t2_dn8 = assign25920_e39313_d_n8;
        locals.var_t2_dn9 = assign25920_e39313_d_n9;
        locals.var_t2_dn10 = assign25920_e39313_d_n10;
        locals.var_t2_dn11 = assign25920_e39313_d_n11;

        let (assign25930_e39328, assign25930_e39328_d_n3, assign25930_e39328_d_n4, assign25930_e39328_d_n5, assign25930_e39328_d_n6, assign25930_e39328_d_n7, assign25930_e39328_d_n8, assign25930_e39328_d_n9, assign25930_e39328_d_n10, assign25930_e39328_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) {
        let assign25930_e39321: f64 = (1.0 / locals.var_leff);
        let assign25930_e39324: f64 = (1.0 / p.p595);
        let assign25930_e39325: f64 = (assign25930_e39321 + assign25930_e39324);
        let assign25930_e39326: f64 = (locals.var_lbjt0_i * assign25930_e39325);
        (assign25930_e39326, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign25930_e39328;
        locals.var_t0_dn3 = assign25930_e39328_d_n3;
        locals.var_t0_dn4 = assign25930_e39328_d_n4;
        locals.var_t0_dn5 = assign25930_e39328_d_n5;
        locals.var_t0_dn6 = assign25930_e39328_d_n6;
        locals.var_t0_dn7 = assign25930_e39328_d_n7;
        locals.var_t0_dn8 = assign25930_e39328_d_n8;
        locals.var_t0_dn9 = assign25930_e39328_d_n9;
        locals.var_t0_dn10 = assign25930_e39328_d_n10;
        locals.var_t0_dn11 = assign25930_e39328_d_n11;

        let (assign25940_e39337, assign25940_e39337_d_n3, assign25940_e39337_d_n4, assign25940_e39337_d_n5, assign25940_e39337_d_n6, assign25940_e39337_d_n7, assign25940_e39337_d_n8, assign25940_e39337_d_n9, assign25940_e39337_d_n10, assign25940_e39337_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) {
        let assign25940_e39335: f64 = (locals.var_t0).powf(locals.var_nbjt_i);
        (assign25940_e39335, if 0.0 == 0.0 && ((locals.var_nbjt_i) as f64).is_finite() && ((locals.var_nbjt_i) as f64).fract() == 0.0 { if locals.var_nbjt_i == 0.0 { 0.0 } else { (locals.var_nbjt_i * ((locals.var_t0).powf(locals.var_nbjt_i - 1.0) * locals.var_t0_dn3)) } } else { (assign25940_e39335 * (locals.var_nbjt_i * (locals.var_t0_dn3 / locals.var_t0))) }, if 0.0 == 0.0 && ((locals.var_nbjt_i) as f64).is_finite() && ((locals.var_nbjt_i) as f64).fract() == 0.0 { if locals.var_nbjt_i == 0.0 { 0.0 } else { (locals.var_nbjt_i * ((locals.var_t0).powf(locals.var_nbjt_i - 1.0) * locals.var_t0_dn4)) } } else { (assign25940_e39335 * (locals.var_nbjt_i * (locals.var_t0_dn4 / locals.var_t0))) }, if 0.0 == 0.0 && ((locals.var_nbjt_i) as f64).is_finite() && ((locals.var_nbjt_i) as f64).fract() == 0.0 { if locals.var_nbjt_i == 0.0 { 0.0 } else { (locals.var_nbjt_i * ((locals.var_t0).powf(locals.var_nbjt_i - 1.0) * locals.var_t0_dn5)) } } else { (assign25940_e39335 * (locals.var_nbjt_i * (locals.var_t0_dn5 / locals.var_t0))) }, if 0.0 == 0.0 && ((locals.var_nbjt_i) as f64).is_finite() && ((locals.var_nbjt_i) as f64).fract() == 0.0 { if locals.var_nbjt_i == 0.0 { 0.0 } else { (locals.var_nbjt_i * ((locals.var_t0).powf(locals.var_nbjt_i - 1.0) * locals.var_t0_dn6)) } } else { (assign25940_e39335 * (locals.var_nbjt_i * (locals.var_t0_dn6 / locals.var_t0))) }, if 0.0 == 0.0 && ((locals.var_nbjt_i) as f64).is_finite() && ((locals.var_nbjt_i) as f64).fract() == 0.0 { if locals.var_nbjt_i == 0.0 { 0.0 } else { (locals.var_nbjt_i * ((locals.var_t0).powf(locals.var_nbjt_i - 1.0) * locals.var_t0_dn7)) } } else { (assign25940_e39335 * (locals.var_nbjt_i * (locals.var_t0_dn7 / locals.var_t0))) }, if 0.0 == 0.0 && ((locals.var_nbjt_i) as f64).is_finite() && ((locals.var_nbjt_i) as f64).fract() == 0.0 { if locals.var_nbjt_i == 0.0 { 0.0 } else { (locals.var_nbjt_i * ((locals.var_t0).powf(locals.var_nbjt_i - 1.0) * locals.var_t0_dn8)) } } else { (assign25940_e39335 * (locals.var_nbjt_i * (locals.var_t0_dn8 / locals.var_t0))) }, if 0.0 == 0.0 && ((locals.var_nbjt_i) as f64).is_finite() && ((locals.var_nbjt_i) as f64).fract() == 0.0 { if locals.var_nbjt_i == 0.0 { 0.0 } else { (locals.var_nbjt_i * ((locals.var_t0).powf(locals.var_nbjt_i - 1.0) * locals.var_t0_dn9)) } } else { (assign25940_e39335 * (locals.var_nbjt_i * (locals.var_t0_dn9 / locals.var_t0))) }, if 0.0 == 0.0 && ((locals.var_nbjt_i) as f64).is_finite() && ((locals.var_nbjt_i) as f64).fract() == 0.0 { if locals.var_nbjt_i == 0.0 { 0.0 } else { (locals.var_nbjt_i * ((locals.var_t0).powf(locals.var_nbjt_i - 1.0) * locals.var_t0_dn10)) } } else { (assign25940_e39335 * (locals.var_nbjt_i * (locals.var_t0_dn10 / locals.var_t0))) }, if 0.0 == 0.0 && ((locals.var_nbjt_i) as f64).is_finite() && ((locals.var_nbjt_i) as f64).fract() == 0.0 { if locals.var_nbjt_i == 0.0 { 0.0 } else { (locals.var_nbjt_i * ((locals.var_t0).powf(locals.var_nbjt_i - 1.0) * locals.var_t0_dn11)) } } else { (assign25940_e39335 * (locals.var_nbjt_i * (locals.var_t0_dn11 / locals.var_t0))) },)
    } else {
        (locals.var_lratio, locals.var_lratio_dn3, locals.var_lratio_dn4, locals.var_lratio_dn5, locals.var_lratio_dn6, locals.var_lratio_dn7, locals.var_lratio_dn8, locals.var_lratio_dn9, locals.var_lratio_dn10, locals.var_lratio_dn11,)
    }
};
        locals.var_lratio = assign25940_e39337;
        locals.var_lratio_dn3 = assign25940_e39337_d_n3;
        locals.var_lratio_dn4 = assign25940_e39337_d_n4;
        locals.var_lratio_dn5 = assign25940_e39337_d_n5;
        locals.var_lratio_dn6 = assign25940_e39337_d_n6;
        locals.var_lratio_dn7 = assign25940_e39337_d_n7;
        locals.var_lratio_dn8 = assign25940_e39337_d_n8;
        locals.var_lratio_dn9 = assign25940_e39337_d_n9;
        locals.var_lratio_dn10 = assign25940_e39337_d_n10;
        locals.var_lratio_dn11 = assign25940_e39337_d_n11;

        let (assign25950_e39348, assign25950_e39348_d_n3, assign25950_e39348_d_n4, assign25950_e39348_d_n5, assign25950_e39348_d_n6, assign25950_e39348_d_n7, assign25950_e39348_d_n8, assign25950_e39348_d_n9, assign25950_e39348_d_n10, assign25950_e39348_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) {
        let assign25950_e39344: f64 = (locals.var_wtsi * locals.var_jbjts);
        let assign25950_e39346: f64 = (assign25950_e39344 * locals.var_lratio);
        (assign25950_e39346, (((locals.var_wtsi * locals.var_jbjts_dn3) * locals.var_lratio) + (assign25950_e39344 * locals.var_lratio_dn3)), (((locals.var_wtsi * locals.var_jbjts_dn4) * locals.var_lratio) + (assign25950_e39344 * locals.var_lratio_dn4)), (((locals.var_wtsi * locals.var_jbjts_dn5) * locals.var_lratio) + (assign25950_e39344 * locals.var_lratio_dn5)), (((locals.var_wtsi * locals.var_jbjts_dn6) * locals.var_lratio) + (assign25950_e39344 * locals.var_lratio_dn6)), (((locals.var_wtsi * locals.var_jbjts_dn7) * locals.var_lratio) + (assign25950_e39344 * locals.var_lratio_dn7)), (((locals.var_wtsi * locals.var_jbjts_dn8) * locals.var_lratio) + (assign25950_e39344 * locals.var_lratio_dn8)), (((locals.var_wtsi * locals.var_jbjts_dn9) * locals.var_lratio) + (assign25950_e39344 * locals.var_lratio_dn9)), (((locals.var_wtsi * locals.var_jbjts_dn10) * locals.var_lratio) + (assign25950_e39344 * locals.var_lratio_dn10)), (((locals.var_wtsi * locals.var_jbjts_dn11) * locals.var_lratio) + (assign25950_e39344 * locals.var_lratio_dn11)),)
    } else {
        (locals.var_ien, locals.var_ien_dn3, locals.var_ien_dn4, locals.var_ien_dn5, locals.var_ien_dn6, locals.var_ien_dn7, locals.var_ien_dn8, locals.var_ien_dn9, locals.var_ien_dn10, locals.var_ien_dn11,)
    }
};
        locals.var_ien = assign25950_e39348;
        locals.var_ien_dn3 = assign25950_e39348_d_n3;
        locals.var_ien_dn4 = assign25950_e39348_d_n4;
        locals.var_ien_dn5 = assign25950_e39348_d_n5;
        locals.var_ien_dn6 = assign25950_e39348_d_n6;
        locals.var_ien_dn7 = assign25950_e39348_d_n7;
        locals.var_ien_dn8 = assign25950_e39348_d_n8;
        locals.var_ien_dn9 = assign25950_e39348_d_n9;
        locals.var_ien_dn10 = assign25950_e39348_d_n10;
        locals.var_ien_dn11 = assign25950_e39348_d_n11;

        let (assign25960_e39357, assign25960_e39357_d_n3, assign25960_e39357_d_n4, assign25960_e39357_d_n5, assign25960_e39357_d_n6, assign25960_e39357_d_n7, assign25960_e39357_d_n8, assign25960_e39357_d_n9, assign25960_e39357_d_n10, assign25960_e39357_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) {
        let assign25960_e39355: f64 = (locals.var_t0 * locals.var_ien);
        (assign25960_e39355, ((locals.var_t0_dn3 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn3)), ((locals.var_t0_dn4 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn4)), ((locals.var_t0_dn5 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn5)), ((locals.var_t0_dn6 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn6)), ((locals.var_t0_dn7 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn7)), ((locals.var_t0_dn8 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn8)), ((locals.var_t0_dn9 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn9)), ((locals.var_t0_dn10 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn10)), ((locals.var_t0_dn11 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn11)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign25960_e39357;
        locals.var_t1_dn3 = assign25960_e39357_d_n3;
        locals.var_t1_dn4 = assign25960_e39357_d_n4;
        locals.var_t1_dn5 = assign25960_e39357_d_n5;
        locals.var_t1_dn6 = assign25960_e39357_d_n6;
        locals.var_t1_dn7 = assign25960_e39357_d_n7;
        locals.var_t1_dn8 = assign25960_e39357_d_n8;
        locals.var_t1_dn9 = assign25960_e39357_d_n9;
        locals.var_t1_dn10 = assign25960_e39357_d_n10;
        locals.var_t1_dn11 = assign25960_e39357_d_n11;

        let (assign25980_e39381, assign25980_e39381_d_n3, assign25980_e39381_d_n4, assign25980_e39381_d_n5, assign25980_e39381_d_n6, assign25980_e39381_d_n7, assign25980_e39381_d_n8, assign25980_e39381_d_n9, assign25980_e39381_d_n10, assign25980_e39381_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) {
        let assign25980_e39377: f64 = (locals.var_wtsi * locals.var_jbjtd);
        let assign25980_e39379: f64 = (assign25980_e39377 * locals.var_lratio);
        (assign25980_e39379, (((locals.var_wtsi * locals.var_jbjtd_dn3) * locals.var_lratio) + (assign25980_e39377 * locals.var_lratio_dn3)), (((locals.var_wtsi * locals.var_jbjtd_dn4) * locals.var_lratio) + (assign25980_e39377 * locals.var_lratio_dn4)), (((locals.var_wtsi * locals.var_jbjtd_dn5) * locals.var_lratio) + (assign25980_e39377 * locals.var_lratio_dn5)), (((locals.var_wtsi * locals.var_jbjtd_dn6) * locals.var_lratio) + (assign25980_e39377 * locals.var_lratio_dn6)), (((locals.var_wtsi * locals.var_jbjtd_dn7) * locals.var_lratio) + (assign25980_e39377 * locals.var_lratio_dn7)), (((locals.var_wtsi * locals.var_jbjtd_dn8) * locals.var_lratio) + (assign25980_e39377 * locals.var_lratio_dn8)), (((locals.var_wtsi * locals.var_jbjtd_dn9) * locals.var_lratio) + (assign25980_e39377 * locals.var_lratio_dn9)), (((locals.var_wtsi * locals.var_jbjtd_dn10) * locals.var_lratio) + (assign25980_e39377 * locals.var_lratio_dn10)), (((locals.var_wtsi * locals.var_jbjtd_dn11) * locals.var_lratio) + (assign25980_e39377 * locals.var_lratio_dn11)),)
    } else {
        (locals.var_ien, locals.var_ien_dn3, locals.var_ien_dn4, locals.var_ien_dn5, locals.var_ien_dn6, locals.var_ien_dn7, locals.var_ien_dn8, locals.var_ien_dn9, locals.var_ien_dn10, locals.var_ien_dn11,)
    }
};
        locals.var_ien = assign25980_e39381;
        locals.var_ien_dn3 = assign25980_e39381_d_n3;
        locals.var_ien_dn4 = assign25980_e39381_d_n4;
        locals.var_ien_dn5 = assign25980_e39381_d_n5;
        locals.var_ien_dn6 = assign25980_e39381_d_n6;
        locals.var_ien_dn7 = assign25980_e39381_d_n7;
        locals.var_ien_dn8 = assign25980_e39381_d_n8;
        locals.var_ien_dn9 = assign25980_e39381_d_n9;
        locals.var_ien_dn10 = assign25980_e39381_d_n10;
        locals.var_ien_dn11 = assign25980_e39381_d_n11;

        let (assign25990_e39390, assign25990_e39390_d_n3, assign25990_e39390_d_n4, assign25990_e39390_d_n5, assign25990_e39390_d_n6, assign25990_e39390_d_n7, assign25990_e39390_d_n8, assign25990_e39390_d_n9, assign25990_e39390_d_n10, assign25990_e39390_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) {
        let assign25990_e39388: f64 = (locals.var_t0 * locals.var_ien);
        (assign25990_e39388, ((locals.var_t0_dn3 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn3)), ((locals.var_t0_dn4 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn4)), ((locals.var_t0_dn5 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn5)), ((locals.var_t0_dn6 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn6)), ((locals.var_t0_dn7 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn7)), ((locals.var_t0_dn8 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn8)), ((locals.var_t0_dn9 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn9)), ((locals.var_t0_dn10 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn10)), ((locals.var_t0_dn11 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn11)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign25990_e39390;
        locals.var_t1_dn3 = assign25990_e39390_d_n3;
        locals.var_t1_dn4 = assign25990_e39390_d_n4;
        locals.var_t1_dn5 = assign25990_e39390_d_n5;
        locals.var_t1_dn6 = assign25990_e39390_d_n6;
        locals.var_t1_dn7 = assign25990_e39390_d_n7;
        locals.var_t1_dn8 = assign25990_e39390_d_n8;
        locals.var_t1_dn9 = assign25990_e39390_d_n9;
        locals.var_t1_dn10 = assign25990_e39390_d_n10;
        locals.var_t1_dn11 = assign25990_e39390_d_n11;

        let (assign26010_e39416, assign26010_e39416_d_n3, assign26010_e39416_d_n4, assign26010_e39416_d_n5, assign26010_e39416_d_n6, assign26010_e39416_d_n7, assign26010_e39416_d_n8, assign26010_e39416_d_n9, assign26010_e39416_d_n10, assign26010_e39416_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) {
        let assign26010_e39412: f64 = (locals.var_t0).powf(locals.var_ndif_i);
        let assign26010_e39413: f64 = (p.p920 * assign26010_e39412);
        let assign26010_e39414: f64 = (1.0 + assign26010_e39413);
        (assign26010_e39414, (p.p920 * if 0.0 == 0.0 && ((locals.var_ndif_i) as f64).is_finite() && ((locals.var_ndif_i) as f64).fract() == 0.0 { if locals.var_ndif_i == 0.0 { 0.0 } else { (locals.var_ndif_i * ((locals.var_t0).powf(locals.var_ndif_i - 1.0) * locals.var_t0_dn3)) } } else { (assign26010_e39412 * (locals.var_ndif_i * (locals.var_t0_dn3 / locals.var_t0))) }), (p.p920 * if 0.0 == 0.0 && ((locals.var_ndif_i) as f64).is_finite() && ((locals.var_ndif_i) as f64).fract() == 0.0 { if locals.var_ndif_i == 0.0 { 0.0 } else { (locals.var_ndif_i * ((locals.var_t0).powf(locals.var_ndif_i - 1.0) * locals.var_t0_dn4)) } } else { (assign26010_e39412 * (locals.var_ndif_i * (locals.var_t0_dn4 / locals.var_t0))) }), (p.p920 * if 0.0 == 0.0 && ((locals.var_ndif_i) as f64).is_finite() && ((locals.var_ndif_i) as f64).fract() == 0.0 { if locals.var_ndif_i == 0.0 { 0.0 } else { (locals.var_ndif_i * ((locals.var_t0).powf(locals.var_ndif_i - 1.0) * locals.var_t0_dn5)) } } else { (assign26010_e39412 * (locals.var_ndif_i * (locals.var_t0_dn5 / locals.var_t0))) }), (p.p920 * if 0.0 == 0.0 && ((locals.var_ndif_i) as f64).is_finite() && ((locals.var_ndif_i) as f64).fract() == 0.0 { if locals.var_ndif_i == 0.0 { 0.0 } else { (locals.var_ndif_i * ((locals.var_t0).powf(locals.var_ndif_i - 1.0) * locals.var_t0_dn6)) } } else { (assign26010_e39412 * (locals.var_ndif_i * (locals.var_t0_dn6 / locals.var_t0))) }), (p.p920 * if 0.0 == 0.0 && ((locals.var_ndif_i) as f64).is_finite() && ((locals.var_ndif_i) as f64).fract() == 0.0 { if locals.var_ndif_i == 0.0 { 0.0 } else { (locals.var_ndif_i * ((locals.var_t0).powf(locals.var_ndif_i - 1.0) * locals.var_t0_dn7)) } } else { (assign26010_e39412 * (locals.var_ndif_i * (locals.var_t0_dn7 / locals.var_t0))) }), (p.p920 * if 0.0 == 0.0 && ((locals.var_ndif_i) as f64).is_finite() && ((locals.var_ndif_i) as f64).fract() == 0.0 { if locals.var_ndif_i == 0.0 { 0.0 } else { (locals.var_ndif_i * ((locals.var_t0).powf(locals.var_ndif_i - 1.0) * locals.var_t0_dn8)) } } else { (assign26010_e39412 * (locals.var_ndif_i * (locals.var_t0_dn8 / locals.var_t0))) }), (p.p920 * if 0.0 == 0.0 && ((locals.var_ndif_i) as f64).is_finite() && ((locals.var_ndif_i) as f64).fract() == 0.0 { if locals.var_ndif_i == 0.0 { 0.0 } else { (locals.var_ndif_i * ((locals.var_t0).powf(locals.var_ndif_i - 1.0) * locals.var_t0_dn9)) } } else { (assign26010_e39412 * (locals.var_ndif_i * (locals.var_t0_dn9 / locals.var_t0))) }), (p.p920 * if 0.0 == 0.0 && ((locals.var_ndif_i) as f64).is_finite() && ((locals.var_ndif_i) as f64).fract() == 0.0 { if locals.var_ndif_i == 0.0 { 0.0 } else { (locals.var_ndif_i * ((locals.var_t0).powf(locals.var_ndif_i - 1.0) * locals.var_t0_dn10)) } } else { (assign26010_e39412 * (locals.var_ndif_i * (locals.var_t0_dn10 / locals.var_t0))) }), (p.p920 * if 0.0 == 0.0 && ((locals.var_ndif_i) as f64).is_finite() && ((locals.var_ndif_i) as f64).fract() == 0.0 { if locals.var_ndif_i == 0.0 { 0.0 } else { (locals.var_ndif_i * ((locals.var_t0).powf(locals.var_ndif_i - 1.0) * locals.var_t0_dn11)) } } else { (assign26010_e39412 * (locals.var_ndif_i * (locals.var_t0_dn11 / locals.var_t0))) }),)
    } else {
        (locals.var_lratiodif, locals.var_lratiodif_dn3, locals.var_lratiodif_dn4, locals.var_lratiodif_dn5, locals.var_lratiodif_dn6, locals.var_lratiodif_dn7, locals.var_lratiodif_dn8, locals.var_lratiodif_dn9, locals.var_lratiodif_dn10, locals.var_lratiodif_dn11,)
    }
};
        locals.var_lratiodif = assign26010_e39416;
        locals.var_lratiodif_dn3 = assign26010_e39416_d_n3;
        locals.var_lratiodif_dn4 = assign26010_e39416_d_n4;
        locals.var_lratiodif_dn5 = assign26010_e39416_d_n5;
        locals.var_lratiodif_dn6 = assign26010_e39416_d_n6;
        locals.var_lratiodif_dn7 = assign26010_e39416_d_n7;
        locals.var_lratiodif_dn8 = assign26010_e39416_d_n8;
        locals.var_lratiodif_dn9 = assign26010_e39416_d_n9;
        locals.var_lratiodif_dn10 = assign26010_e39416_d_n10;
        locals.var_lratiodif_dn11 = assign26010_e39416_d_n11;

        let (assign26020_e39427, assign26020_e39427_d_n3, assign26020_e39427_d_n4, assign26020_e39427_d_n5, assign26020_e39427_d_n6, assign26020_e39427_d_n7, assign26020_e39427_d_n8, assign26020_e39427_d_n9, assign26020_e39427_d_n10, assign26020_e39427_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) {
        let assign26020_e39423: f64 = (locals.var_wtsi * locals.var_jbjts);
        let assign26020_e39425: f64 = (assign26020_e39423 * locals.var_lratiodif);
        (assign26020_e39425, (((locals.var_wtsi * locals.var_jbjts_dn3) * locals.var_lratiodif) + (assign26020_e39423 * locals.var_lratiodif_dn3)), (((locals.var_wtsi * locals.var_jbjts_dn4) * locals.var_lratiodif) + (assign26020_e39423 * locals.var_lratiodif_dn4)), (((locals.var_wtsi * locals.var_jbjts_dn5) * locals.var_lratiodif) + (assign26020_e39423 * locals.var_lratiodif_dn5)), (((locals.var_wtsi * locals.var_jbjts_dn6) * locals.var_lratiodif) + (assign26020_e39423 * locals.var_lratiodif_dn6)), (((locals.var_wtsi * locals.var_jbjts_dn7) * locals.var_lratiodif) + (assign26020_e39423 * locals.var_lratiodif_dn7)), (((locals.var_wtsi * locals.var_jbjts_dn8) * locals.var_lratiodif) + (assign26020_e39423 * locals.var_lratiodif_dn8)), (((locals.var_wtsi * locals.var_jbjts_dn9) * locals.var_lratiodif) + (assign26020_e39423 * locals.var_lratiodif_dn9)), (((locals.var_wtsi * locals.var_jbjts_dn10) * locals.var_lratiodif) + (assign26020_e39423 * locals.var_lratiodif_dn10)), (((locals.var_wtsi * locals.var_jbjts_dn11) * locals.var_lratiodif) + (assign26020_e39423 * locals.var_lratiodif_dn11)),)
    } else {
        (locals.var_iendif, locals.var_iendif_dn3, locals.var_iendif_dn4, locals.var_iendif_dn5, locals.var_iendif_dn6, locals.var_iendif_dn7, locals.var_iendif_dn8, locals.var_iendif_dn9, locals.var_iendif_dn10, locals.var_iendif_dn11,)
    }
};
        locals.var_iendif = assign26020_e39427;
        locals.var_iendif_dn3 = assign26020_e39427_d_n3;
        locals.var_iendif_dn4 = assign26020_e39427_d_n4;
        locals.var_iendif_dn5 = assign26020_e39427_d_n5;
        locals.var_iendif_dn6 = assign26020_e39427_d_n6;
        locals.var_iendif_dn7 = assign26020_e39427_d_n7;
        locals.var_iendif_dn8 = assign26020_e39427_d_n8;
        locals.var_iendif_dn9 = assign26020_e39427_d_n9;
        locals.var_iendif_dn10 = assign26020_e39427_d_n10;
        locals.var_iendif_dn11 = assign26020_e39427_d_n11;

        let (assign26030_e39440, assign26030_e39440_d_n3, assign26030_e39440_d_n4, assign26030_e39440_d_n5, assign26030_e39440_d_n6, assign26030_e39440_d_n7, assign26030_e39440_d_n8, assign26030_e39440_d_n9, assign26030_e39440_d_n10, assign26030_e39440_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) {
        let assign26030_e39435: f64 = (locals.var_expvbsnvtm - 1.0);
        let assign26030_e39436: f64 = (locals.var_iendif * assign26030_e39435);
        let assign26030_e39438: f64 = (assign26030_e39436 * locals.var_ehlisfactor);
        (assign26030_e39438, ((((locals.var_iendif_dn3 * assign26030_e39435) + (locals.var_iendif * locals.var_expvbsnvtm_dn3)) * locals.var_ehlisfactor) + (assign26030_e39436 * locals.var_ehlisfactor_dn3)), ((((locals.var_iendif_dn4 * assign26030_e39435) + (locals.var_iendif * locals.var_expvbsnvtm_dn4)) * locals.var_ehlisfactor) + (assign26030_e39436 * locals.var_ehlisfactor_dn4)), ((((locals.var_iendif_dn5 * assign26030_e39435) + (locals.var_iendif * locals.var_expvbsnvtm_dn5)) * locals.var_ehlisfactor) + (assign26030_e39436 * locals.var_ehlisfactor_dn5)), ((((locals.var_iendif_dn6 * assign26030_e39435) + (locals.var_iendif * locals.var_expvbsnvtm_dn6)) * locals.var_ehlisfactor) + (assign26030_e39436 * locals.var_ehlisfactor_dn6)), ((((locals.var_iendif_dn7 * assign26030_e39435) + (locals.var_iendif * locals.var_expvbsnvtm_dn7)) * locals.var_ehlisfactor) + (assign26030_e39436 * locals.var_ehlisfactor_dn7)), ((((locals.var_iendif_dn8 * assign26030_e39435) + (locals.var_iendif * locals.var_expvbsnvtm_dn8)) * locals.var_ehlisfactor) + (assign26030_e39436 * locals.var_ehlisfactor_dn8)), ((((locals.var_iendif_dn9 * assign26030_e39435) + (locals.var_iendif * locals.var_expvbsnvtm_dn9)) * locals.var_ehlisfactor) + (assign26030_e39436 * locals.var_ehlisfactor_dn9)), ((((locals.var_iendif_dn10 * assign26030_e39435) + (locals.var_iendif * locals.var_expvbsnvtm_dn10)) * locals.var_ehlisfactor) + (assign26030_e39436 * locals.var_ehlisfactor_dn10)), ((((locals.var_iendif_dn11 * assign26030_e39435) + (locals.var_iendif * locals.var_expvbsnvtm_dn11)) * locals.var_ehlisfactor) + (assign26030_e39436 * locals.var_ehlisfactor_dn11)),)
    } else {
        (locals.var_ibsdif, locals.var_ibsdif_dn3, locals.var_ibsdif_dn4, locals.var_ibsdif_dn5, locals.var_ibsdif_dn6, locals.var_ibsdif_dn7, locals.var_ibsdif_dn8, locals.var_ibsdif_dn9, locals.var_ibsdif_dn10, locals.var_ibsdif_dn11,)
    }
};
        locals.var_ibsdif = assign26030_e39440;
        locals.var_ibsdif_dn3 = assign26030_e39440_d_n3;
        locals.var_ibsdif_dn4 = assign26030_e39440_d_n4;
        locals.var_ibsdif_dn5 = assign26030_e39440_d_n5;
        locals.var_ibsdif_dn6 = assign26030_e39440_d_n6;
        locals.var_ibsdif_dn7 = assign26030_e39440_d_n7;
        locals.var_ibsdif_dn8 = assign26030_e39440_d_n8;
        locals.var_ibsdif_dn9 = assign26030_e39440_d_n9;
        locals.var_ibsdif_dn10 = assign26030_e39440_d_n10;
        locals.var_ibsdif_dn11 = assign26030_e39440_d_n11;

        let (assign26040_e39451, assign26040_e39451_d_n3, assign26040_e39451_d_n4, assign26040_e39451_d_n5, assign26040_e39451_d_n6, assign26040_e39451_d_n7, assign26040_e39451_d_n8, assign26040_e39451_d_n9, assign26040_e39451_d_n10, assign26040_e39451_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) {
        let assign26040_e39447: f64 = (locals.var_wtsi * locals.var_jbjtd);
        let assign26040_e39449: f64 = (assign26040_e39447 * locals.var_lratiodif);
        (assign26040_e39449, (((locals.var_wtsi * locals.var_jbjtd_dn3) * locals.var_lratiodif) + (assign26040_e39447 * locals.var_lratiodif_dn3)), (((locals.var_wtsi * locals.var_jbjtd_dn4) * locals.var_lratiodif) + (assign26040_e39447 * locals.var_lratiodif_dn4)), (((locals.var_wtsi * locals.var_jbjtd_dn5) * locals.var_lratiodif) + (assign26040_e39447 * locals.var_lratiodif_dn5)), (((locals.var_wtsi * locals.var_jbjtd_dn6) * locals.var_lratiodif) + (assign26040_e39447 * locals.var_lratiodif_dn6)), (((locals.var_wtsi * locals.var_jbjtd_dn7) * locals.var_lratiodif) + (assign26040_e39447 * locals.var_lratiodif_dn7)), (((locals.var_wtsi * locals.var_jbjtd_dn8) * locals.var_lratiodif) + (assign26040_e39447 * locals.var_lratiodif_dn8)), (((locals.var_wtsi * locals.var_jbjtd_dn9) * locals.var_lratiodif) + (assign26040_e39447 * locals.var_lratiodif_dn9)), (((locals.var_wtsi * locals.var_jbjtd_dn10) * locals.var_lratiodif) + (assign26040_e39447 * locals.var_lratiodif_dn10)), (((locals.var_wtsi * locals.var_jbjtd_dn11) * locals.var_lratiodif) + (assign26040_e39447 * locals.var_lratiodif_dn11)),)
    } else {
        (locals.var_iendif, locals.var_iendif_dn3, locals.var_iendif_dn4, locals.var_iendif_dn5, locals.var_iendif_dn6, locals.var_iendif_dn7, locals.var_iendif_dn8, locals.var_iendif_dn9, locals.var_iendif_dn10, locals.var_iendif_dn11,)
    }
};
        locals.var_iendif = assign26040_e39451;
        locals.var_iendif_dn3 = assign26040_e39451_d_n3;
        locals.var_iendif_dn4 = assign26040_e39451_d_n4;
        locals.var_iendif_dn5 = assign26040_e39451_d_n5;
        locals.var_iendif_dn6 = assign26040_e39451_d_n6;
        locals.var_iendif_dn7 = assign26040_e39451_d_n7;
        locals.var_iendif_dn8 = assign26040_e39451_d_n8;
        locals.var_iendif_dn9 = assign26040_e39451_d_n9;
        locals.var_iendif_dn10 = assign26040_e39451_d_n10;
        locals.var_iendif_dn11 = assign26040_e39451_d_n11;

        let (assign26050_e39464, assign26050_e39464_d_n3, assign26050_e39464_d_n4, assign26050_e39464_d_n5, assign26050_e39464_d_n6, assign26050_e39464_d_n7, assign26050_e39464_d_n8, assign26050_e39464_d_n9, assign26050_e39464_d_n10, assign26050_e39464_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) {
        let assign26050_e39459: f64 = (locals.var_expvbdnvtm - 1.0);
        let assign26050_e39460: f64 = (locals.var_iendif * assign26050_e39459);
        let assign26050_e39462: f64 = (assign26050_e39460 * locals.var_ehlidfactor);
        (assign26050_e39462, ((((locals.var_iendif_dn3 * assign26050_e39459) + (locals.var_iendif * locals.var_expvbdnvtm_dn3)) * locals.var_ehlidfactor) + (assign26050_e39460 * locals.var_ehlidfactor_dn3)), ((((locals.var_iendif_dn4 * assign26050_e39459) + (locals.var_iendif * locals.var_expvbdnvtm_dn4)) * locals.var_ehlidfactor) + (assign26050_e39460 * locals.var_ehlidfactor_dn4)), ((((locals.var_iendif_dn5 * assign26050_e39459) + (locals.var_iendif * locals.var_expvbdnvtm_dn5)) * locals.var_ehlidfactor) + (assign26050_e39460 * locals.var_ehlidfactor_dn5)), ((((locals.var_iendif_dn6 * assign26050_e39459) + (locals.var_iendif * locals.var_expvbdnvtm_dn6)) * locals.var_ehlidfactor) + (assign26050_e39460 * locals.var_ehlidfactor_dn6)), ((((locals.var_iendif_dn7 * assign26050_e39459) + (locals.var_iendif * locals.var_expvbdnvtm_dn7)) * locals.var_ehlidfactor) + (assign26050_e39460 * locals.var_ehlidfactor_dn7)), ((((locals.var_iendif_dn8 * assign26050_e39459) + (locals.var_iendif * locals.var_expvbdnvtm_dn8)) * locals.var_ehlidfactor) + (assign26050_e39460 * locals.var_ehlidfactor_dn8)), ((((locals.var_iendif_dn9 * assign26050_e39459) + (locals.var_iendif * locals.var_expvbdnvtm_dn9)) * locals.var_ehlidfactor) + (assign26050_e39460 * locals.var_ehlidfactor_dn9)), ((((locals.var_iendif_dn10 * assign26050_e39459) + (locals.var_iendif * locals.var_expvbdnvtm_dn10)) * locals.var_ehlidfactor) + (assign26050_e39460 * locals.var_ehlidfactor_dn10)), ((((locals.var_iendif_dn11 * assign26050_e39459) + (locals.var_iendif * locals.var_expvbdnvtm_dn11)) * locals.var_ehlidfactor) + (assign26050_e39460 * locals.var_ehlidfactor_dn11)),)
    } else {
        (locals.var_ibddif, locals.var_ibddif_dn3, locals.var_ibddif_dn4, locals.var_ibddif_dn5, locals.var_ibddif_dn6, locals.var_ibddif_dn7, locals.var_ibddif_dn8, locals.var_ibddif_dn9, locals.var_ibddif_dn10, locals.var_ibddif_dn11,)
    }
};
        locals.var_ibddif = assign26050_e39464;
        locals.var_ibddif_dn3 = assign26050_e39464_d_n3;
        locals.var_ibddif_dn4 = assign26050_e39464_d_n4;
        locals.var_ibddif_dn5 = assign26050_e39464_d_n5;
        locals.var_ibddif_dn6 = assign26050_e39464_d_n6;
        locals.var_ibddif_dn7 = assign26050_e39464_d_n7;
        locals.var_ibddif_dn8 = assign26050_e39464_d_n8;
        locals.var_ibddif_dn9 = assign26050_e39464_d_n9;
        locals.var_ibddif_dn10 = assign26050_e39464_d_n10;
        locals.var_ibddif_dn11 = assign26050_e39464_d_n11;

        let (assign26060_e39475,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) {
        let assign26060_e39472: f64 = (locals.var_aely_i * locals.var_leff);
        let assign26060_e39473: f64 = (locals.var_vabjt_i + assign26060_e39472);
        (assign26060_e39473,)
    } else {
        (locals.var_vearly,)
    }
};
        locals.var_vearly = assign26060_e39475;

        let assign26070_e39478: f64 = if locals.var_vearly < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard551 = assign26070_e39478;

        let (assign26080_e39487,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) && (locals.var_guard551 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_vearly,)
    }
};
        locals.var_vearly = assign26080_e39487;

        let assign26090_e39490: f64 = if p.p554 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard552 = assign26090_e39490;

        let (assign26110_e39515, assign26110_e39515_d_n3, assign26110_e39515_d_n4, assign26110_e39515_d_n5, assign26110_e39515_d_n6, assign26110_e39515_d_n7, assign26110_e39515_d_n8, assign26110_e39515_d_n9, assign26110_e39515_d_n10, assign26110_e39515_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) && (locals.var_guard552 == 0.0)) {
        let assign26110_e39510: f64 = (locals.var_vbs_jct + locals.var_vbd_jct);
        let assign26110_e39512: f64 = (assign26110_e39510 / locals.var_vearly);
        let assign26110_e39513: f64 = (1.0 + assign26110_e39512);
        (assign26110_e39513, 0.0, 0.0, 0.0, (locals.var_vbd_jct_dn6 / locals.var_vearly), (locals.var_vbs_jct_dn7 / locals.var_vearly), 0.0, 0.0, ((locals.var_vbs_jct_dn10 + locals.var_vbd_jct_dn10) / locals.var_vearly), 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign26110_e39515;
        locals.var_t0_dn3 = assign26110_e39515_d_n3;
        locals.var_t0_dn4 = assign26110_e39515_d_n4;
        locals.var_t0_dn5 = assign26110_e39515_d_n5;
        locals.var_t0_dn6 = assign26110_e39515_d_n6;
        locals.var_t0_dn7 = assign26110_e39515_d_n7;
        locals.var_t0_dn8 = assign26110_e39515_d_n8;
        locals.var_t0_dn9 = assign26110_e39515_d_n9;
        locals.var_t0_dn10 = assign26110_e39515_d_n10;
        locals.var_t0_dn11 = assign26110_e39515_d_n11;

        let (assign26120_e39527, assign26120_e39527_d_n3, assign26120_e39527_d_n4, assign26120_e39527_d_n5, assign26120_e39527_d_n6, assign26120_e39527_d_n7, assign26120_e39527_d_n8, assign26120_e39527_d_n9, assign26120_e39527_d_n10, assign26120_e39527_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) && (locals.var_guard552 == 0.0)) {
        let assign26120_e39525: f64 = (locals.var_ehlis + locals.var_ehlid);
        (assign26120_e39525, (locals.var_ehlis_dn3 + locals.var_ehlid_dn3), (locals.var_ehlis_dn4 + locals.var_ehlid_dn4), (locals.var_ehlis_dn5 + locals.var_ehlid_dn5), (locals.var_ehlis_dn6 + locals.var_ehlid_dn6), (locals.var_ehlis_dn7 + locals.var_ehlid_dn7), (locals.var_ehlis_dn8 + locals.var_ehlid_dn8), (locals.var_ehlis_dn9 + locals.var_ehlid_dn9), (locals.var_ehlis_dn10 + locals.var_ehlid_dn10), (locals.var_ehlis_dn11 + locals.var_ehlid_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign26120_e39527;
        locals.var_t1_dn3 = assign26120_e39527_d_n3;
        locals.var_t1_dn4 = assign26120_e39527_d_n4;
        locals.var_t1_dn5 = assign26120_e39527_d_n5;
        locals.var_t1_dn6 = assign26120_e39527_d_n6;
        locals.var_t1_dn7 = assign26120_e39527_d_n7;
        locals.var_t1_dn8 = assign26120_e39527_d_n8;
        locals.var_t1_dn9 = assign26120_e39527_d_n9;
        locals.var_t1_dn10 = assign26120_e39527_d_n10;
        locals.var_t1_dn11 = assign26120_e39527_d_n11;

    }

    pub(super) fn stamp_transient_block_69(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign26130_e39544, assign26130_e39544_d_n3, assign26130_e39544_d_n4, assign26130_e39544_d_n5, assign26130_e39544_d_n6, assign26130_e39544_d_n7, assign26130_e39544_d_n8, assign26130_e39544_d_n9, assign26130_e39544_d_n10, assign26130_e39544_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) && (locals.var_guard552 == 0.0)) {
        let assign26130_e39537: f64 = (locals.var_t0 * locals.var_t0);
        let assign26130_e39540: f64 = (4.0 * locals.var_t1);
        let assign26130_e39541: f64 = (assign26130_e39537 + assign26130_e39540);
        let assign26130_e39542: f64 = (assign26130_e39541).sqrt();
        (assign26130_e39542, ((((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) + (4.0 * locals.var_t1_dn3)) / (2.0 * assign26130_e39542)), ((((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) + (4.0 * locals.var_t1_dn4)) / (2.0 * assign26130_e39542)), ((((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) + (4.0 * locals.var_t1_dn5)) / (2.0 * assign26130_e39542)), ((((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) + (4.0 * locals.var_t1_dn6)) / (2.0 * assign26130_e39542)), ((((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) + (4.0 * locals.var_t1_dn7)) / (2.0 * assign26130_e39542)), ((((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) + (4.0 * locals.var_t1_dn8)) / (2.0 * assign26130_e39542)), ((((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) + (4.0 * locals.var_t1_dn9)) / (2.0 * assign26130_e39542)), ((((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) + (4.0 * locals.var_t1_dn10)) / (2.0 * assign26130_e39542)), ((((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) + (4.0 * locals.var_t1_dn11)) / (2.0 * assign26130_e39542)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign26130_e39544;
        locals.var_t3_dn3 = assign26130_e39544_d_n3;
        locals.var_t3_dn4 = assign26130_e39544_d_n4;
        locals.var_t3_dn5 = assign26130_e39544_d_n5;
        locals.var_t3_dn6 = assign26130_e39544_d_n6;
        locals.var_t3_dn7 = assign26130_e39544_d_n7;
        locals.var_t3_dn8 = assign26130_e39544_d_n8;
        locals.var_t3_dn9 = assign26130_e39544_d_n9;
        locals.var_t3_dn10 = assign26130_e39544_d_n10;
        locals.var_t3_dn11 = assign26130_e39544_d_n11;

        let (assign26140_e39558, assign26140_e39558_d_n3, assign26140_e39558_d_n4, assign26140_e39558_d_n5, assign26140_e39558_d_n6, assign26140_e39558_d_n7, assign26140_e39558_d_n8, assign26140_e39558_d_n9, assign26140_e39558_d_n10, assign26140_e39558_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) && (locals.var_guard552 == 0.0)) {
        let assign26140_e39554: f64 = (locals.var_t0 + locals.var_t3);
        let assign26140_e39556: f64 = (assign26140_e39554 / 2.0);
        (assign26140_e39556, ((locals.var_t0_dn3 + locals.var_t3_dn3) / 2.0), ((locals.var_t0_dn4 + locals.var_t3_dn4) / 2.0), ((locals.var_t0_dn5 + locals.var_t3_dn5) / 2.0), ((locals.var_t0_dn6 + locals.var_t3_dn6) / 2.0), ((locals.var_t0_dn7 + locals.var_t3_dn7) / 2.0), ((locals.var_t0_dn8 + locals.var_t3_dn8) / 2.0), ((locals.var_t0_dn9 + locals.var_t3_dn9) / 2.0), ((locals.var_t0_dn10 + locals.var_t3_dn10) / 2.0), ((locals.var_t0_dn11 + locals.var_t3_dn11) / 2.0),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign26140_e39558;
        locals.var_t2_dn3 = assign26140_e39558_d_n3;
        locals.var_t2_dn4 = assign26140_e39558_d_n4;
        locals.var_t2_dn5 = assign26140_e39558_d_n5;
        locals.var_t2_dn6 = assign26140_e39558_d_n6;
        locals.var_t2_dn7 = assign26140_e39558_d_n7;
        locals.var_t2_dn8 = assign26140_e39558_d_n8;
        locals.var_t2_dn9 = assign26140_e39558_d_n9;
        locals.var_t2_dn10 = assign26140_e39558_d_n10;
        locals.var_t2_dn11 = assign26140_e39558_d_n11;

        let (assign26180_e39600, assign26180_e39600_d_n3, assign26180_e39600_d_n4, assign26180_e39600_d_n5, assign26180_e39600_d_n6, assign26180_e39600_d_n7, assign26180_e39600_d_n8, assign26180_e39600_d_n9, assign26180_e39600_d_n10, assign26180_e39600_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) && (locals.var_guard552 == 0.0)) {
        let assign26180_e39598: f64 = (locals.var_alphabjt * locals.var_ien);
        (assign26180_e39598, ((locals.var_alphabjt_dn3 * locals.var_ien) + (locals.var_alphabjt * locals.var_ien_dn3)), ((locals.var_alphabjt_dn4 * locals.var_ien) + (locals.var_alphabjt * locals.var_ien_dn4)), ((locals.var_alphabjt_dn5 * locals.var_ien) + (locals.var_alphabjt * locals.var_ien_dn5)), ((locals.var_alphabjt_dn6 * locals.var_ien) + (locals.var_alphabjt * locals.var_ien_dn6)), ((locals.var_alphabjt_dn7 * locals.var_ien) + (locals.var_alphabjt * locals.var_ien_dn7)), ((locals.var_alphabjt_dn8 * locals.var_ien) + (locals.var_alphabjt * locals.var_ien_dn8)), ((locals.var_alphabjt_dn9 * locals.var_ien) + (locals.var_alphabjt * locals.var_ien_dn9)), ((locals.var_alphabjt_dn10 * locals.var_ien) + (locals.var_alphabjt * locals.var_ien_dn10)), ((locals.var_alphabjt_dn11 * locals.var_ien) + (locals.var_alphabjt * locals.var_ien_dn11)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign26180_e39600;
        locals.var_t0_dn3 = assign26180_e39600_d_n3;
        locals.var_t0_dn4 = assign26180_e39600_d_n4;
        locals.var_t0_dn5 = assign26180_e39600_d_n5;
        locals.var_t0_dn6 = assign26180_e39600_d_n6;
        locals.var_t0_dn7 = assign26180_e39600_d_n7;
        locals.var_t0_dn8 = assign26180_e39600_d_n8;
        locals.var_t0_dn9 = assign26180_e39600_d_n9;
        locals.var_t0_dn10 = assign26180_e39600_d_n10;
        locals.var_t0_dn11 = assign26180_e39600_d_n11;

        let assign26200_e39625: f64 = if ((locals.var_istun_i == 0.0) && (locals.var_idtun_i == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard554 = assign26200_e39625;

        let (assign26230_e39648, assign26230_e39648_d_n3, assign26230_e39648_d_n4, assign26230_e39648_d_n5, assign26230_e39648_d_n6, assign26230_e39648_d_n7, assign26230_e39648_d_n8, assign26230_e39648_d_n9, assign26230_e39648_d_n10, assign26230_e39648_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard554 == 0.0)) {
        let assign26230_e39645: f64 = (locals.var_tratio - 1.0);
        let assign26230_e39646: f64 = (locals.var_xtun_i * assign26230_e39645);
        (assign26230_e39646, 0.0, (locals.var_xtun_i * locals.var_tratio_dn4), (locals.var_xtun_i * locals.var_tratio_dn5), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign26230_e39648;
        locals.var_t7_dn3 = assign26230_e39648_d_n3;
        locals.var_t7_dn4 = assign26230_e39648_d_n4;
        locals.var_t7_dn5 = assign26230_e39648_d_n5;
        locals.var_t7_dn6 = assign26230_e39648_d_n6;
        locals.var_t7_dn7 = assign26230_e39648_d_n7;
        locals.var_t7_dn8 = assign26230_e39648_d_n8;
        locals.var_t7_dn9 = assign26230_e39648_d_n9;
        locals.var_t7_dn10 = assign26230_e39648_d_n10;
        locals.var_t7_dn11 = assign26230_e39648_d_n11;

        let (assign26240_e39656, assign26240_e39656_d_n3, assign26240_e39656_d_n4, assign26240_e39656_d_n5, assign26240_e39656_d_n6, assign26240_e39656_d_n7, assign26240_e39656_d_n8, assign26240_e39656_d_n9, assign26240_e39656_d_n10, assign26240_e39656_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard554 == 0.0)) {
        let assign26240_e39654: f64 = { let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign26240_e39654, ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn3), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn4), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn5), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn6), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn7), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn8), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn9), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn10), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign26240_e39656;
        locals.var_t0_dn3 = assign26240_e39656_d_n3;
        locals.var_t0_dn4 = assign26240_e39656_d_n4;
        locals.var_t0_dn5 = assign26240_e39656_d_n5;
        locals.var_t0_dn6 = assign26240_e39656_d_n6;
        locals.var_t0_dn7 = assign26240_e39656_d_n7;
        locals.var_t0_dn8 = assign26240_e39656_d_n8;
        locals.var_t0_dn9 = assign26240_e39656_d_n9;
        locals.var_t0_dn10 = assign26240_e39656_d_n10;
        locals.var_t0_dn11 = assign26240_e39656_d_n11;

        let (assign26250_e39665, assign26250_e39665_d_n3, assign26250_e39665_d_n4, assign26250_e39665_d_n5, assign26250_e39665_d_n6, assign26250_e39665_d_n7, assign26250_e39665_d_n8, assign26250_e39665_d_n9, assign26250_e39665_d_n10, assign26250_e39665_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard554 == 0.0)) {
        let assign26250_e39663: f64 = (locals.var_istun_i * locals.var_t0);
        (assign26250_e39663, (locals.var_istun_i * locals.var_t0_dn3), (locals.var_istun_i * locals.var_t0_dn4), (locals.var_istun_i * locals.var_t0_dn5), (locals.var_istun_i * locals.var_t0_dn6), (locals.var_istun_i * locals.var_t0_dn7), (locals.var_istun_i * locals.var_t0_dn8), (locals.var_istun_i * locals.var_t0_dn9), (locals.var_istun_i * locals.var_t0_dn10), (locals.var_istun_i * locals.var_t0_dn11),)
    } else {
        (locals.var_jtuns, locals.var_jtuns_dn3, locals.var_jtuns_dn4, locals.var_jtuns_dn5, locals.var_jtuns_dn6, locals.var_jtuns_dn7, locals.var_jtuns_dn8, locals.var_jtuns_dn9, locals.var_jtuns_dn10, locals.var_jtuns_dn11,)
    }
};
        locals.var_jtuns = assign26250_e39665;
        locals.var_jtuns_dn3 = assign26250_e39665_d_n3;
        locals.var_jtuns_dn4 = assign26250_e39665_d_n4;
        locals.var_jtuns_dn5 = assign26250_e39665_d_n5;
        locals.var_jtuns_dn6 = assign26250_e39665_d_n6;
        locals.var_jtuns_dn7 = assign26250_e39665_d_n7;
        locals.var_jtuns_dn8 = assign26250_e39665_d_n8;
        locals.var_jtuns_dn9 = assign26250_e39665_d_n9;
        locals.var_jtuns_dn10 = assign26250_e39665_d_n10;
        locals.var_jtuns_dn11 = assign26250_e39665_d_n11;

        let (assign26260_e39676, assign26260_e39676_d_n3, assign26260_e39676_d_n4, assign26260_e39676_d_n5, assign26260_e39676_d_n6, assign26260_e39676_d_n7, assign26260_e39676_d_n8, assign26260_e39676_d_n9, assign26260_e39676_d_n10, assign26260_e39676_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard554 == 0.0)) {
        let assign26260_e39673: f64 = (locals.var_tratio - 1.0);
        let assign26260_e39674: f64 = (locals.var_xtund_i * assign26260_e39673);
        (assign26260_e39674, 0.0, (locals.var_xtund_i * locals.var_tratio_dn4), (locals.var_xtund_i * locals.var_tratio_dn5), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign26260_e39676;
        locals.var_t7_dn3 = assign26260_e39676_d_n3;
        locals.var_t7_dn4 = assign26260_e39676_d_n4;
        locals.var_t7_dn5 = assign26260_e39676_d_n5;
        locals.var_t7_dn6 = assign26260_e39676_d_n6;
        locals.var_t7_dn7 = assign26260_e39676_d_n7;
        locals.var_t7_dn8 = assign26260_e39676_d_n8;
        locals.var_t7_dn9 = assign26260_e39676_d_n9;
        locals.var_t7_dn10 = assign26260_e39676_d_n10;
        locals.var_t7_dn11 = assign26260_e39676_d_n11;

        let (assign26270_e39684, assign26270_e39684_d_n3, assign26270_e39684_d_n4, assign26270_e39684_d_n5, assign26270_e39684_d_n6, assign26270_e39684_d_n7, assign26270_e39684_d_n8, assign26270_e39684_d_n9, assign26270_e39684_d_n10, assign26270_e39684_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard554 == 0.0)) {
        let assign26270_e39682: f64 = { let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign26270_e39682, ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn3), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn4), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn5), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn6), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn7), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn8), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn9), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn10), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign26270_e39684;
        locals.var_t0_dn3 = assign26270_e39684_d_n3;
        locals.var_t0_dn4 = assign26270_e39684_d_n4;
        locals.var_t0_dn5 = assign26270_e39684_d_n5;
        locals.var_t0_dn6 = assign26270_e39684_d_n6;
        locals.var_t0_dn7 = assign26270_e39684_d_n7;
        locals.var_t0_dn8 = assign26270_e39684_d_n8;
        locals.var_t0_dn9 = assign26270_e39684_d_n9;
        locals.var_t0_dn10 = assign26270_e39684_d_n10;
        locals.var_t0_dn11 = assign26270_e39684_d_n11;

        let (assign26280_e39693, assign26280_e39693_d_n3, assign26280_e39693_d_n4, assign26280_e39693_d_n5, assign26280_e39693_d_n6, assign26280_e39693_d_n7, assign26280_e39693_d_n8, assign26280_e39693_d_n9, assign26280_e39693_d_n10, assign26280_e39693_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard554 == 0.0)) {
        let assign26280_e39691: f64 = (locals.var_idtun_i * locals.var_t0);
        (assign26280_e39691, (locals.var_idtun_i * locals.var_t0_dn3), (locals.var_idtun_i * locals.var_t0_dn4), (locals.var_idtun_i * locals.var_t0_dn5), (locals.var_idtun_i * locals.var_t0_dn6), (locals.var_idtun_i * locals.var_t0_dn7), (locals.var_idtun_i * locals.var_t0_dn8), (locals.var_idtun_i * locals.var_t0_dn9), (locals.var_idtun_i * locals.var_t0_dn10), (locals.var_idtun_i * locals.var_t0_dn11),)
    } else {
        (locals.var_jtund, locals.var_jtund_dn3, locals.var_jtund_dn4, locals.var_jtund_dn5, locals.var_jtund_dn6, locals.var_jtund_dn7, locals.var_jtund_dn8, locals.var_jtund_dn9, locals.var_jtund_dn10, locals.var_jtund_dn11,)
    }
};
        locals.var_jtund = assign26280_e39693;
        locals.var_jtund_dn3 = assign26280_e39693_d_n3;
        locals.var_jtund_dn4 = assign26280_e39693_d_n4;
        locals.var_jtund_dn5 = assign26280_e39693_d_n5;
        locals.var_jtund_dn6 = assign26280_e39693_d_n6;
        locals.var_jtund_dn7 = assign26280_e39693_d_n7;
        locals.var_jtund_dn8 = assign26280_e39693_d_n8;
        locals.var_jtund_dn9 = assign26280_e39693_d_n9;
        locals.var_jtund_dn10 = assign26280_e39693_d_n10;
        locals.var_jtund_dn11 = assign26280_e39693_d_n11;

        let (assign26290_e39702, assign26290_e39702_d_n4, assign26290_e39702_d_n5,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard554 == 0.0)) {
        let assign26290_e39700: f64 = (p.p925 * locals.var_ntun_i);
        (assign26290_e39700, 0.0, 0.0,)
    } else {
        (locals.var_nvtm2, locals.var_nvtm2_dn4, locals.var_nvtm2_dn5,)
    }
};
        locals.var_nvtm2 = assign26290_e39702;
        locals.var_nvtm2_dn4 = assign26290_e39702_d_n4;
        locals.var_nvtm2_dn5 = assign26290_e39702_d_n5;

        let assign26300_e39705: f64 = (locals.var_vtun0_i - locals.var_vbs_jct);
        let assign26300_e39707: f64 = if assign26300_e39705 < 0.001 { 1.0 } else { 0.0 };
        locals.var_guard555 = assign26300_e39707;

        let (assign26310_e39716, assign26310_e39716_d_n3, assign26310_e39716_d_n4, assign26310_e39716_d_n5, assign26310_e39716_d_n6, assign26310_e39716_d_n7, assign26310_e39716_d_n8, assign26310_e39716_d_n9, assign26310_e39716_d_n10, assign26310_e39716_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard554 == 0.0)) && (locals.var_guard555 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign26310_e39716;
        locals.var_t1_dn3 = assign26310_e39716_d_n3;
        locals.var_t1_dn4 = assign26310_e39716_d_n4;
        locals.var_t1_dn5 = assign26310_e39716_d_n5;
        locals.var_t1_dn6 = assign26310_e39716_d_n6;
        locals.var_t1_dn7 = assign26310_e39716_d_n7;
        locals.var_t1_dn8 = assign26310_e39716_d_n8;
        locals.var_t1_dn9 = assign26310_e39716_d_n9;
        locals.var_t1_dn10 = assign26310_e39716_d_n10;
        locals.var_t1_dn11 = assign26310_e39716_d_n11;

        let (assign26320_e39732, assign26320_e39732_d_n3, assign26320_e39732_d_n4, assign26320_e39732_d_n5, assign26320_e39732_d_n6, assign26320_e39732_d_n7, assign26320_e39732_d_n8, assign26320_e39732_d_n9, assign26320_e39732_d_n10, assign26320_e39732_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard554 == 0.0)) && (locals.var_guard555 != 0.0)) {
        let assign26320_e39724: f64 = (-locals.var_vbs_jct);
        let assign26320_e39726: f64 = (assign26320_e39724 / locals.var_nvtm2);
        let assign26320_e39728: f64 = (assign26320_e39726 * locals.var_vtun0_i);
        let assign26320_e39730: f64 = (assign26320_e39728 * locals.var_t1);
        (assign26320_e39730, (assign26320_e39728 * locals.var_t1_dn3), ((((-((assign26320_e39724 * locals.var_nvtm2_dn4) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_vtun0_i) * locals.var_t1) + (assign26320_e39728 * locals.var_t1_dn4)), ((((-((assign26320_e39724 * locals.var_nvtm2_dn5) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_vtun0_i) * locals.var_t1) + (assign26320_e39728 * locals.var_t1_dn5)), (assign26320_e39728 * locals.var_t1_dn6), (((((-locals.var_vbs_jct_dn7) / locals.var_nvtm2) * locals.var_vtun0_i) * locals.var_t1) + (assign26320_e39728 * locals.var_t1_dn7)), (assign26320_e39728 * locals.var_t1_dn8), (assign26320_e39728 * locals.var_t1_dn9), (((((-locals.var_vbs_jct_dn10) / locals.var_nvtm2) * locals.var_vtun0_i) * locals.var_t1) + (assign26320_e39728 * locals.var_t1_dn10)), (assign26320_e39728 * locals.var_t1_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign26320_e39732;
        locals.var_t0_dn3 = assign26320_e39732_d_n3;
        locals.var_t0_dn4 = assign26320_e39732_d_n4;
        locals.var_t0_dn5 = assign26320_e39732_d_n5;
        locals.var_t0_dn6 = assign26320_e39732_d_n6;
        locals.var_t0_dn7 = assign26320_e39732_d_n7;
        locals.var_t0_dn8 = assign26320_e39732_d_n8;
        locals.var_t0_dn9 = assign26320_e39732_d_n9;
        locals.var_t0_dn10 = assign26320_e39732_d_n10;
        locals.var_t0_dn11 = assign26320_e39732_d_n11;

        let (assign26330_e39742, assign26330_e39742_d_n3, assign26330_e39742_d_n4, assign26330_e39742_d_n5, assign26330_e39742_d_n6, assign26330_e39742_d_n7, assign26330_e39742_d_n8, assign26330_e39742_d_n9, assign26330_e39742_d_n10, assign26330_e39742_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard554 == 0.0)) && (locals.var_guard555 != 0.0)) {
        let assign26330_e39740: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign26330_e39740, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign26330_e39742;
        locals.var_t1_dn3 = assign26330_e39742_d_n3;
        locals.var_t1_dn4 = assign26330_e39742_d_n4;
        locals.var_t1_dn5 = assign26330_e39742_d_n5;
        locals.var_t1_dn6 = assign26330_e39742_d_n6;
        locals.var_t1_dn7 = assign26330_e39742_d_n7;
        locals.var_t1_dn8 = assign26330_e39742_d_n8;
        locals.var_t1_dn9 = assign26330_e39742_d_n9;
        locals.var_t1_dn10 = assign26330_e39742_d_n10;
        locals.var_t1_dn11 = assign26330_e39742_d_n11;

        let (assign26340_e39753, assign26340_e39753_d_n3, assign26340_e39753_d_n4, assign26340_e39753_d_n5, assign26340_e39753_d_n6, assign26340_e39753_d_n7, assign26340_e39753_d_n8, assign26340_e39753_d_n9, assign26340_e39753_d_n10, assign26340_e39753_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard554 == 0.0)) && (locals.var_guard555 != 0.0)) {
        let assign26340_e39751: f64 = (locals.var_wstsi * locals.var_jtuns);
        (assign26340_e39751, (locals.var_wstsi * locals.var_jtuns_dn3), (locals.var_wstsi * locals.var_jtuns_dn4), (locals.var_wstsi * locals.var_jtuns_dn5), (locals.var_wstsi * locals.var_jtuns_dn6), (locals.var_wstsi * locals.var_jtuns_dn7), (locals.var_wstsi * locals.var_jtuns_dn8), (locals.var_wstsi * locals.var_jtuns_dn9), (locals.var_wstsi * locals.var_jtuns_dn10), (locals.var_wstsi * locals.var_jtuns_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign26340_e39753;
        locals.var_t3_dn3 = assign26340_e39753_d_n3;
        locals.var_t3_dn4 = assign26340_e39753_d_n4;
        locals.var_t3_dn5 = assign26340_e39753_d_n5;
        locals.var_t3_dn6 = assign26340_e39753_d_n6;
        locals.var_t3_dn7 = assign26340_e39753_d_n7;
        locals.var_t3_dn8 = assign26340_e39753_d_n8;
        locals.var_t3_dn9 = assign26340_e39753_d_n9;
        locals.var_t3_dn10 = assign26340_e39753_d_n10;
        locals.var_t3_dn11 = assign26340_e39753_d_n11;

        let (assign26360_e39780, assign26360_e39780_d_n3, assign26360_e39780_d_n4, assign26360_e39780_d_n5, assign26360_e39780_d_n6, assign26360_e39780_d_n7, assign26360_e39780_d_n8, assign26360_e39780_d_n9, assign26360_e39780_d_n10, assign26360_e39780_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard554 == 0.0)) && (locals.var_guard555 == 0.0)) {
        let assign26360_e39777: f64 = (locals.var_vtun0_i - locals.var_vbs_jct);
        let assign26360_e39778: f64 = (1.0 / assign26360_e39777);
        (assign26360_e39778, 0.0, 0.0, 0.0, 0.0, (-((-locals.var_vbs_jct_dn7) / (assign26360_e39777 * assign26360_e39777))), 0.0, 0.0, (-((-locals.var_vbs_jct_dn10) / (assign26360_e39777 * assign26360_e39777))), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign26360_e39780;
        locals.var_t1_dn3 = assign26360_e39780_d_n3;
        locals.var_t1_dn4 = assign26360_e39780_d_n4;
        locals.var_t1_dn5 = assign26360_e39780_d_n5;
        locals.var_t1_dn6 = assign26360_e39780_d_n6;
        locals.var_t1_dn7 = assign26360_e39780_d_n7;
        locals.var_t1_dn8 = assign26360_e39780_d_n8;
        locals.var_t1_dn9 = assign26360_e39780_d_n9;
        locals.var_t1_dn10 = assign26360_e39780_d_n10;
        locals.var_t1_dn11 = assign26360_e39780_d_n11;

        let (assign26370_e39797, assign26370_e39797_d_n3, assign26370_e39797_d_n4, assign26370_e39797_d_n5, assign26370_e39797_d_n6, assign26370_e39797_d_n7, assign26370_e39797_d_n8, assign26370_e39797_d_n9, assign26370_e39797_d_n10, assign26370_e39797_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard554 == 0.0)) && (locals.var_guard555 == 0.0)) {
        let assign26370_e39789: f64 = (-locals.var_vbs_jct);
        let assign26370_e39791: f64 = (assign26370_e39789 / locals.var_nvtm2);
        let assign26370_e39793: f64 = (assign26370_e39791 * locals.var_vtun0_i);
        let assign26370_e39795: f64 = (assign26370_e39793 * locals.var_t1);
        (assign26370_e39795, (assign26370_e39793 * locals.var_t1_dn3), ((((-((assign26370_e39789 * locals.var_nvtm2_dn4) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_vtun0_i) * locals.var_t1) + (assign26370_e39793 * locals.var_t1_dn4)), ((((-((assign26370_e39789 * locals.var_nvtm2_dn5) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_vtun0_i) * locals.var_t1) + (assign26370_e39793 * locals.var_t1_dn5)), (assign26370_e39793 * locals.var_t1_dn6), (((((-locals.var_vbs_jct_dn7) / locals.var_nvtm2) * locals.var_vtun0_i) * locals.var_t1) + (assign26370_e39793 * locals.var_t1_dn7)), (assign26370_e39793 * locals.var_t1_dn8), (assign26370_e39793 * locals.var_t1_dn9), (((((-locals.var_vbs_jct_dn10) / locals.var_nvtm2) * locals.var_vtun0_i) * locals.var_t1) + (assign26370_e39793 * locals.var_t1_dn10)), (assign26370_e39793 * locals.var_t1_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign26370_e39797;
        locals.var_t0_dn3 = assign26370_e39797_d_n3;
        locals.var_t0_dn4 = assign26370_e39797_d_n4;
        locals.var_t0_dn5 = assign26370_e39797_d_n5;
        locals.var_t0_dn6 = assign26370_e39797_d_n6;
        locals.var_t0_dn7 = assign26370_e39797_d_n7;
        locals.var_t0_dn8 = assign26370_e39797_d_n8;
        locals.var_t0_dn9 = assign26370_e39797_d_n9;
        locals.var_t0_dn10 = assign26370_e39797_d_n10;
        locals.var_t0_dn11 = assign26370_e39797_d_n11;

        let (assign26380_e39808, assign26380_e39808_d_n3, assign26380_e39808_d_n4, assign26380_e39808_d_n5, assign26380_e39808_d_n6, assign26380_e39808_d_n7, assign26380_e39808_d_n8, assign26380_e39808_d_n9, assign26380_e39808_d_n10, assign26380_e39808_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard554 == 0.0)) && (locals.var_guard555 == 0.0)) {
        let assign26380_e39806: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign26380_e39806, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign26380_e39808;
        locals.var_t1_dn3 = assign26380_e39808_d_n3;
        locals.var_t1_dn4 = assign26380_e39808_d_n4;
        locals.var_t1_dn5 = assign26380_e39808_d_n5;
        locals.var_t1_dn6 = assign26380_e39808_d_n6;
        locals.var_t1_dn7 = assign26380_e39808_d_n7;
        locals.var_t1_dn8 = assign26380_e39808_d_n8;
        locals.var_t1_dn9 = assign26380_e39808_d_n9;
        locals.var_t1_dn10 = assign26380_e39808_d_n10;
        locals.var_t1_dn11 = assign26380_e39808_d_n11;

        let (assign26390_e39820, assign26390_e39820_d_n3, assign26390_e39820_d_n4, assign26390_e39820_d_n5, assign26390_e39820_d_n6, assign26390_e39820_d_n7, assign26390_e39820_d_n8, assign26390_e39820_d_n9, assign26390_e39820_d_n10, assign26390_e39820_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard554 == 0.0)) && (locals.var_guard555 == 0.0)) {
        let assign26390_e39818: f64 = (locals.var_wstsi * locals.var_jtuns);
        (assign26390_e39818, (locals.var_wstsi * locals.var_jtuns_dn3), (locals.var_wstsi * locals.var_jtuns_dn4), (locals.var_wstsi * locals.var_jtuns_dn5), (locals.var_wstsi * locals.var_jtuns_dn6), (locals.var_wstsi * locals.var_jtuns_dn7), (locals.var_wstsi * locals.var_jtuns_dn8), (locals.var_wstsi * locals.var_jtuns_dn9), (locals.var_wstsi * locals.var_jtuns_dn10), (locals.var_wstsi * locals.var_jtuns_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign26390_e39820;
        locals.var_t3_dn3 = assign26390_e39820_d_n3;
        locals.var_t3_dn4 = assign26390_e39820_d_n4;
        locals.var_t3_dn5 = assign26390_e39820_d_n5;
        locals.var_t3_dn6 = assign26390_e39820_d_n6;
        locals.var_t3_dn7 = assign26390_e39820_d_n7;
        locals.var_t3_dn8 = assign26390_e39820_d_n8;
        locals.var_t3_dn9 = assign26390_e39820_d_n9;
        locals.var_t3_dn10 = assign26390_e39820_d_n10;
        locals.var_t3_dn11 = assign26390_e39820_d_n11;

        let (assign26410_e39843, assign26410_e39843_d_n4, assign26410_e39843_d_n5,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard554 == 0.0)) {
        let assign26410_e39841: f64 = (p.p925 * locals.var_ntund_i);
        (assign26410_e39841, 0.0, 0.0,)
    } else {
        (locals.var_nvtm2, locals.var_nvtm2_dn4, locals.var_nvtm2_dn5,)
    }
};
        locals.var_nvtm2 = assign26410_e39843;
        locals.var_nvtm2_dn4 = assign26410_e39843_d_n4;
        locals.var_nvtm2_dn5 = assign26410_e39843_d_n5;

        let assign26420_e39846: f64 = (locals.var_vtun0d_i - locals.var_vbd_jct);
        let assign26420_e39848: f64 = if assign26420_e39846 < 0.001 { 1.0 } else { 0.0 };
        locals.var_guard556 = assign26420_e39848;

        let (assign26430_e39857, assign26430_e39857_d_n3, assign26430_e39857_d_n4, assign26430_e39857_d_n5, assign26430_e39857_d_n6, assign26430_e39857_d_n7, assign26430_e39857_d_n8, assign26430_e39857_d_n9, assign26430_e39857_d_n10, assign26430_e39857_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard554 == 0.0)) && (locals.var_guard556 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign26430_e39857;
        locals.var_t1_dn3 = assign26430_e39857_d_n3;
        locals.var_t1_dn4 = assign26430_e39857_d_n4;
        locals.var_t1_dn5 = assign26430_e39857_d_n5;
        locals.var_t1_dn6 = assign26430_e39857_d_n6;
        locals.var_t1_dn7 = assign26430_e39857_d_n7;
        locals.var_t1_dn8 = assign26430_e39857_d_n8;
        locals.var_t1_dn9 = assign26430_e39857_d_n9;
        locals.var_t1_dn10 = assign26430_e39857_d_n10;
        locals.var_t1_dn11 = assign26430_e39857_d_n11;

        let (assign26440_e39873, assign26440_e39873_d_n3, assign26440_e39873_d_n4, assign26440_e39873_d_n5, assign26440_e39873_d_n6, assign26440_e39873_d_n7, assign26440_e39873_d_n8, assign26440_e39873_d_n9, assign26440_e39873_d_n10, assign26440_e39873_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard554 == 0.0)) && (locals.var_guard556 != 0.0)) {
        let assign26440_e39865: f64 = (-locals.var_vbd_jct);
        let assign26440_e39867: f64 = (assign26440_e39865 / locals.var_nvtm2);
        let assign26440_e39869: f64 = (assign26440_e39867 * locals.var_vtun0d_i);
        let assign26440_e39871: f64 = (assign26440_e39869 * locals.var_t1);
        (assign26440_e39871, (assign26440_e39869 * locals.var_t1_dn3), ((((-((assign26440_e39865 * locals.var_nvtm2_dn4) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_vtun0d_i) * locals.var_t1) + (assign26440_e39869 * locals.var_t1_dn4)), ((((-((assign26440_e39865 * locals.var_nvtm2_dn5) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_vtun0d_i) * locals.var_t1) + (assign26440_e39869 * locals.var_t1_dn5)), (((((-locals.var_vbd_jct_dn6) / locals.var_nvtm2) * locals.var_vtun0d_i) * locals.var_t1) + (assign26440_e39869 * locals.var_t1_dn6)), (assign26440_e39869 * locals.var_t1_dn7), (assign26440_e39869 * locals.var_t1_dn8), (assign26440_e39869 * locals.var_t1_dn9), (((((-locals.var_vbd_jct_dn10) / locals.var_nvtm2) * locals.var_vtun0d_i) * locals.var_t1) + (assign26440_e39869 * locals.var_t1_dn10)), (assign26440_e39869 * locals.var_t1_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign26440_e39873;
        locals.var_t0_dn3 = assign26440_e39873_d_n3;
        locals.var_t0_dn4 = assign26440_e39873_d_n4;
        locals.var_t0_dn5 = assign26440_e39873_d_n5;
        locals.var_t0_dn6 = assign26440_e39873_d_n6;
        locals.var_t0_dn7 = assign26440_e39873_d_n7;
        locals.var_t0_dn8 = assign26440_e39873_d_n8;
        locals.var_t0_dn9 = assign26440_e39873_d_n9;
        locals.var_t0_dn10 = assign26440_e39873_d_n10;
        locals.var_t0_dn11 = assign26440_e39873_d_n11;

        let (assign26450_e39883, assign26450_e39883_d_n3, assign26450_e39883_d_n4, assign26450_e39883_d_n5, assign26450_e39883_d_n6, assign26450_e39883_d_n7, assign26450_e39883_d_n8, assign26450_e39883_d_n9, assign26450_e39883_d_n10, assign26450_e39883_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard554 == 0.0)) && (locals.var_guard556 != 0.0)) {
        let assign26450_e39881: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign26450_e39881, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign26450_e39883;
        locals.var_t1_dn3 = assign26450_e39883_d_n3;
        locals.var_t1_dn4 = assign26450_e39883_d_n4;
        locals.var_t1_dn5 = assign26450_e39883_d_n5;
        locals.var_t1_dn6 = assign26450_e39883_d_n6;
        locals.var_t1_dn7 = assign26450_e39883_d_n7;
        locals.var_t1_dn8 = assign26450_e39883_d_n8;
        locals.var_t1_dn9 = assign26450_e39883_d_n9;
        locals.var_t1_dn10 = assign26450_e39883_d_n10;
        locals.var_t1_dn11 = assign26450_e39883_d_n11;

        let (assign26460_e39894, assign26460_e39894_d_n3, assign26460_e39894_d_n4, assign26460_e39894_d_n5, assign26460_e39894_d_n6, assign26460_e39894_d_n7, assign26460_e39894_d_n8, assign26460_e39894_d_n9, assign26460_e39894_d_n10, assign26460_e39894_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard554 == 0.0)) && (locals.var_guard556 != 0.0)) {
        let assign26460_e39892: f64 = (locals.var_wstsi * locals.var_jtund);
        (assign26460_e39892, (locals.var_wstsi * locals.var_jtund_dn3), (locals.var_wstsi * locals.var_jtund_dn4), (locals.var_wstsi * locals.var_jtund_dn5), (locals.var_wstsi * locals.var_jtund_dn6), (locals.var_wstsi * locals.var_jtund_dn7), (locals.var_wstsi * locals.var_jtund_dn8), (locals.var_wstsi * locals.var_jtund_dn9), (locals.var_wstsi * locals.var_jtund_dn10), (locals.var_wstsi * locals.var_jtund_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign26460_e39894;
        locals.var_t3_dn3 = assign26460_e39894_d_n3;
        locals.var_t3_dn4 = assign26460_e39894_d_n4;
        locals.var_t3_dn5 = assign26460_e39894_d_n5;
        locals.var_t3_dn6 = assign26460_e39894_d_n6;
        locals.var_t3_dn7 = assign26460_e39894_d_n7;
        locals.var_t3_dn8 = assign26460_e39894_d_n8;
        locals.var_t3_dn9 = assign26460_e39894_d_n9;
        locals.var_t3_dn10 = assign26460_e39894_d_n10;
        locals.var_t3_dn11 = assign26460_e39894_d_n11;

        let (assign26480_e39921, assign26480_e39921_d_n3, assign26480_e39921_d_n4, assign26480_e39921_d_n5, assign26480_e39921_d_n6, assign26480_e39921_d_n7, assign26480_e39921_d_n8, assign26480_e39921_d_n9, assign26480_e39921_d_n10, assign26480_e39921_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard554 == 0.0)) && (locals.var_guard556 == 0.0)) {
        let assign26480_e39918: f64 = (locals.var_vtun0d_i - locals.var_vbd_jct);
        let assign26480_e39919: f64 = (1.0 / assign26480_e39918);
        (assign26480_e39919, 0.0, 0.0, 0.0, (-((-locals.var_vbd_jct_dn6) / (assign26480_e39918 * assign26480_e39918))), 0.0, 0.0, 0.0, (-((-locals.var_vbd_jct_dn10) / (assign26480_e39918 * assign26480_e39918))), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign26480_e39921;
        locals.var_t1_dn3 = assign26480_e39921_d_n3;
        locals.var_t1_dn4 = assign26480_e39921_d_n4;
        locals.var_t1_dn5 = assign26480_e39921_d_n5;
        locals.var_t1_dn6 = assign26480_e39921_d_n6;
        locals.var_t1_dn7 = assign26480_e39921_d_n7;
        locals.var_t1_dn8 = assign26480_e39921_d_n8;
        locals.var_t1_dn9 = assign26480_e39921_d_n9;
        locals.var_t1_dn10 = assign26480_e39921_d_n10;
        locals.var_t1_dn11 = assign26480_e39921_d_n11;

        let (assign26490_e39938, assign26490_e39938_d_n3, assign26490_e39938_d_n4, assign26490_e39938_d_n5, assign26490_e39938_d_n6, assign26490_e39938_d_n7, assign26490_e39938_d_n8, assign26490_e39938_d_n9, assign26490_e39938_d_n10, assign26490_e39938_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard554 == 0.0)) && (locals.var_guard556 == 0.0)) {
        let assign26490_e39930: f64 = (-locals.var_vbd_jct);
        let assign26490_e39932: f64 = (assign26490_e39930 / locals.var_nvtm2);
        let assign26490_e39934: f64 = (assign26490_e39932 * locals.var_vtun0d_i);
        let assign26490_e39936: f64 = (assign26490_e39934 * locals.var_t1);
        (assign26490_e39936, (assign26490_e39934 * locals.var_t1_dn3), ((((-((assign26490_e39930 * locals.var_nvtm2_dn4) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_vtun0d_i) * locals.var_t1) + (assign26490_e39934 * locals.var_t1_dn4)), ((((-((assign26490_e39930 * locals.var_nvtm2_dn5) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_vtun0d_i) * locals.var_t1) + (assign26490_e39934 * locals.var_t1_dn5)), (((((-locals.var_vbd_jct_dn6) / locals.var_nvtm2) * locals.var_vtun0d_i) * locals.var_t1) + (assign26490_e39934 * locals.var_t1_dn6)), (assign26490_e39934 * locals.var_t1_dn7), (assign26490_e39934 * locals.var_t1_dn8), (assign26490_e39934 * locals.var_t1_dn9), (((((-locals.var_vbd_jct_dn10) / locals.var_nvtm2) * locals.var_vtun0d_i) * locals.var_t1) + (assign26490_e39934 * locals.var_t1_dn10)), (assign26490_e39934 * locals.var_t1_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign26490_e39938;
        locals.var_t0_dn3 = assign26490_e39938_d_n3;
        locals.var_t0_dn4 = assign26490_e39938_d_n4;
        locals.var_t0_dn5 = assign26490_e39938_d_n5;
        locals.var_t0_dn6 = assign26490_e39938_d_n6;
        locals.var_t0_dn7 = assign26490_e39938_d_n7;
        locals.var_t0_dn8 = assign26490_e39938_d_n8;
        locals.var_t0_dn9 = assign26490_e39938_d_n9;
        locals.var_t0_dn10 = assign26490_e39938_d_n10;
        locals.var_t0_dn11 = assign26490_e39938_d_n11;

        let (assign26500_e39949, assign26500_e39949_d_n3, assign26500_e39949_d_n4, assign26500_e39949_d_n5, assign26500_e39949_d_n6, assign26500_e39949_d_n7, assign26500_e39949_d_n8, assign26500_e39949_d_n9, assign26500_e39949_d_n10, assign26500_e39949_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard554 == 0.0)) && (locals.var_guard556 == 0.0)) {
        let assign26500_e39947: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign26500_e39947, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign26500_e39949;
        locals.var_t1_dn3 = assign26500_e39949_d_n3;
        locals.var_t1_dn4 = assign26500_e39949_d_n4;
        locals.var_t1_dn5 = assign26500_e39949_d_n5;
        locals.var_t1_dn6 = assign26500_e39949_d_n6;
        locals.var_t1_dn7 = assign26500_e39949_d_n7;
        locals.var_t1_dn8 = assign26500_e39949_d_n8;
        locals.var_t1_dn9 = assign26500_e39949_d_n9;
        locals.var_t1_dn10 = assign26500_e39949_d_n10;
        locals.var_t1_dn11 = assign26500_e39949_d_n11;

    }

    pub(super) fn stamp_transient_block_70(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign26510_e39961, assign26510_e39961_d_n3, assign26510_e39961_d_n4, assign26510_e39961_d_n5, assign26510_e39961_d_n6, assign26510_e39961_d_n7, assign26510_e39961_d_n8, assign26510_e39961_d_n9, assign26510_e39961_d_n10, assign26510_e39961_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard554 == 0.0)) && (locals.var_guard556 == 0.0)) {
        let assign26510_e39959: f64 = (locals.var_wstsi * locals.var_jtund);
        (assign26510_e39959, (locals.var_wstsi * locals.var_jtund_dn3), (locals.var_wstsi * locals.var_jtund_dn4), (locals.var_wstsi * locals.var_jtund_dn5), (locals.var_wstsi * locals.var_jtund_dn6), (locals.var_wstsi * locals.var_jtund_dn7), (locals.var_wstsi * locals.var_jtund_dn8), (locals.var_wstsi * locals.var_jtund_dn9), (locals.var_wstsi * locals.var_jtund_dn10), (locals.var_wstsi * locals.var_jtund_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign26510_e39961;
        locals.var_t3_dn3 = assign26510_e39961_d_n3;
        locals.var_t3_dn4 = assign26510_e39961_d_n4;
        locals.var_t3_dn5 = assign26510_e39961_d_n5;
        locals.var_t3_dn6 = assign26510_e39961_d_n6;
        locals.var_t3_dn7 = assign26510_e39961_d_n7;
        locals.var_t3_dn8 = assign26510_e39961_d_n8;
        locals.var_t3_dn9 = assign26510_e39961_d_n9;
        locals.var_t3_dn10 = assign26510_e39961_d_n10;
        locals.var_t3_dn11 = assign26510_e39961_d_n11;

        let assign26570_e40010: f64 = if p.p36 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard557 = assign26570_e40010;

        let (assign26580_e40018, assign26580_e40018_d_n3, assign26580_e40018_d_n4, assign26580_e40018_d_n5, assign26580_e40018_d_n6, assign26580_e40018_d_n7, assign26580_e40018_d_n8, assign26580_e40018_d_n9, assign26580_e40018_d_n10, assign26580_e40018_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard557 != 0.0)) {
        let assign26580_e40016: f64 = (locals.var_epsratio * p.p76);
        (assign26580_e40016, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign26580_e40018;
        locals.var_t0_dn3 = assign26580_e40018_d_n3;
        locals.var_t0_dn4 = assign26580_e40018_d_n4;
        locals.var_t0_dn5 = assign26580_e40018_d_n5;
        locals.var_t0_dn6 = assign26580_e40018_d_n6;
        locals.var_t0_dn7 = assign26580_e40018_d_n7;
        locals.var_t0_dn8 = assign26580_e40018_d_n8;
        locals.var_t0_dn9 = assign26580_e40018_d_n9;
        locals.var_t0_dn10 = assign26580_e40018_d_n10;
        locals.var_t0_dn11 = assign26580_e40018_d_n11;

        let assign26590_e40029: f64 = if (((locals.var_agidl_i <= 0.0) || (locals.var_bgidl_t <= 0.0)) || (locals.var_cgidl_i < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard558 = assign26590_e40029;

        let (assign26600_e40037, assign26600_e40037_d_n3, assign26600_e40037_d_n4, assign26600_e40037_d_n5, assign26600_e40037_d_n6, assign26600_e40037_d_n7, assign26600_e40037_d_n8, assign26600_e40037_d_n9, assign26600_e40037_d_n10, assign26600_e40037_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard557 != 0.0)) && (locals.var_guard558 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign26600_e40037;
        locals.var_t6_dn3 = assign26600_e40037_d_n3;
        locals.var_t6_dn4 = assign26600_e40037_d_n4;
        locals.var_t6_dn5 = assign26600_e40037_d_n5;
        locals.var_t6_dn6 = assign26600_e40037_d_n6;
        locals.var_t6_dn7 = assign26600_e40037_d_n7;
        locals.var_t6_dn8 = assign26600_e40037_d_n8;
        locals.var_t6_dn9 = assign26600_e40037_d_n9;
        locals.var_t6_dn10 = assign26600_e40037_d_n10;
        locals.var_t6_dn11 = assign26600_e40037_d_n11;

        let (assign26610_e40053, assign26610_e40053_d_n3, assign26610_e40053_d_n4, assign26610_e40053_d_n5, assign26610_e40053_d_n6, assign26610_e40053_d_n7, assign26610_e40053_d_n8, assign26610_e40053_d_n9, assign26610_e40053_d_n10, assign26610_e40053_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard557 != 0.0)) && (locals.var_guard558 == 0.0)) {
        let assign26610_e40045: f64 = (-locals.var_vgd_noswap);
        let assign26610_e40047: f64 = (assign26610_e40045 - locals.var_egidl_i);
        let assign26610_e40049: f64 = (assign26610_e40047 + locals.var_vfbsdr);
        let assign26610_e40051: f64 = (assign26610_e40049 / locals.var_t0);
        (assign26610_e40051, (-((assign26610_e40049 * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0))), (((locals.var_vfbsdr_dn4 * locals.var_t0) - (assign26610_e40049 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsdr_dn5 * locals.var_t0) - (assign26610_e40049 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgd_noswap_dn6) * locals.var_t0) - (assign26610_e40049 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgd_noswap_dn7) * locals.var_t0) - (assign26610_e40049 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgd_noswap_dn8) * locals.var_t0) - (assign26610_e40049 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (-((assign26610_e40049 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))), ((((-locals.var_vgd_noswap_dn10) * locals.var_t0) - (assign26610_e40049 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (-((assign26610_e40049 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign26610_e40053;
        locals.var_t1_dn3 = assign26610_e40053_d_n3;
        locals.var_t1_dn4 = assign26610_e40053_d_n4;
        locals.var_t1_dn5 = assign26610_e40053_d_n5;
        locals.var_t1_dn6 = assign26610_e40053_d_n6;
        locals.var_t1_dn7 = assign26610_e40053_d_n7;
        locals.var_t1_dn8 = assign26610_e40053_d_n8;
        locals.var_t1_dn9 = assign26610_e40053_d_n9;
        locals.var_t1_dn10 = assign26610_e40053_d_n10;
        locals.var_t1_dn11 = assign26610_e40053_d_n11;

        let (assign26620_e40075, assign26620_e40075_d_n3, assign26620_e40075_d_n4, assign26620_e40075_d_n5, assign26620_e40075_d_n6, assign26620_e40075_d_n7, assign26620_e40075_d_n8, assign26620_e40075_d_n9, assign26620_e40075_d_n10, assign26620_e40075_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard557 != 0.0)) && (locals.var_guard558 == 0.0)) {
        let assign26620_e40064: f64 = (locals.var_t1 * locals.var_t1);
        let assign26620_e40067: f64 = (4.0 * 0.01);
        let assign26620_e40069: f64 = (assign26620_e40067 * 0.01);
        let assign26620_e40070: f64 = (assign26620_e40064 + assign26620_e40069);
        let assign26620_e40071: f64 = (assign26620_e40070).sqrt();
        let assign26620_e40072: f64 = (locals.var_t1 + assign26620_e40071);
        let assign26620_e40073: f64 = (0.5 * assign26620_e40072);
        (assign26620_e40073, (0.5 * (locals.var_t1_dn3 + (((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)) / (2.0 * assign26620_e40071)))), (0.5 * (locals.var_t1_dn4 + (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign26620_e40071)))), (0.5 * (locals.var_t1_dn5 + (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign26620_e40071)))), (0.5 * (locals.var_t1_dn6 + (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign26620_e40071)))), (0.5 * (locals.var_t1_dn7 + (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign26620_e40071)))), (0.5 * (locals.var_t1_dn8 + (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign26620_e40071)))), (0.5 * (locals.var_t1_dn9 + (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign26620_e40071)))), (0.5 * (locals.var_t1_dn10 + (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign26620_e40071)))), (0.5 * (locals.var_t1_dn11 + (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign26620_e40071)))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign26620_e40075;
        locals.var_t1_dn3 = assign26620_e40075_d_n3;
        locals.var_t1_dn4 = assign26620_e40075_d_n4;
        locals.var_t1_dn5 = assign26620_e40075_d_n5;
        locals.var_t1_dn6 = assign26620_e40075_d_n6;
        locals.var_t1_dn7 = assign26620_e40075_d_n7;
        locals.var_t1_dn8 = assign26620_e40075_d_n8;
        locals.var_t1_dn9 = assign26620_e40075_d_n9;
        locals.var_t1_dn10 = assign26620_e40075_d_n10;
        locals.var_t1_dn11 = assign26620_e40075_d_n11;

        let (assign26630_e40088, assign26630_e40088_d_n3, assign26630_e40088_d_n4, assign26630_e40088_d_n5, assign26630_e40088_d_n6, assign26630_e40088_d_n7, assign26630_e40088_d_n8, assign26630_e40088_d_n9, assign26630_e40088_d_n10, assign26630_e40088_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard557 != 0.0)) && (locals.var_guard558 == 0.0)) {
        let assign26630_e40085: f64 = (locals.var_t1 + 0.001);
        let assign26630_e40086: f64 = (locals.var_bgidl_t / assign26630_e40085);
        (assign26630_e40086, (-((locals.var_bgidl_t * locals.var_t1_dn3) / (assign26630_e40085 * assign26630_e40085))), (((locals.var_bgidl_t_dn4 * assign26630_e40085) - (locals.var_bgidl_t * locals.var_t1_dn4)) / (assign26630_e40085 * assign26630_e40085)), (((locals.var_bgidl_t_dn5 * assign26630_e40085) - (locals.var_bgidl_t * locals.var_t1_dn5)) / (assign26630_e40085 * assign26630_e40085)), (-((locals.var_bgidl_t * locals.var_t1_dn6) / (assign26630_e40085 * assign26630_e40085))), (-((locals.var_bgidl_t * locals.var_t1_dn7) / (assign26630_e40085 * assign26630_e40085))), (-((locals.var_bgidl_t * locals.var_t1_dn8) / (assign26630_e40085 * assign26630_e40085))), (-((locals.var_bgidl_t * locals.var_t1_dn9) / (assign26630_e40085 * assign26630_e40085))), (-((locals.var_bgidl_t * locals.var_t1_dn10) / (assign26630_e40085 * assign26630_e40085))), (-((locals.var_bgidl_t * locals.var_t1_dn11) / (assign26630_e40085 * assign26630_e40085))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign26630_e40088;
        locals.var_t2_dn3 = assign26630_e40088_d_n3;
        locals.var_t2_dn4 = assign26630_e40088_d_n4;
        locals.var_t2_dn5 = assign26630_e40088_d_n5;
        locals.var_t2_dn6 = assign26630_e40088_d_n6;
        locals.var_t2_dn7 = assign26630_e40088_d_n7;
        locals.var_t2_dn8 = assign26630_e40088_d_n8;
        locals.var_t2_dn9 = assign26630_e40088_d_n9;
        locals.var_t2_dn10 = assign26630_e40088_d_n10;
        locals.var_t2_dn11 = assign26630_e40088_d_n11;

        let assign26640_e40091: f64 = if locals.var_cgidl_i != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard559 = assign26640_e40091;

        let (assign26650_e40106, assign26650_e40106_d_n3, assign26650_e40106_d_n4, assign26650_e40106_d_n5, assign26650_e40106_d_n6, assign26650_e40106_d_n7, assign26650_e40106_d_n8, assign26650_e40106_d_n9, assign26650_e40106_d_n10, assign26650_e40106_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard557 != 0.0)) && (locals.var_guard558 == 0.0)) && (locals.var_guard559 != 0.0)) {
        let assign26650_e40102: f64 = (locals.var_vdb_noswap * locals.var_vdb_noswap);
        let assign26650_e40104: f64 = (assign26650_e40102 * locals.var_vdb_noswap);
        (assign26650_e40104, 0.0, 0.0, 0.0, ((((locals.var_vdb_noswap_dn6 * locals.var_vdb_noswap) + (locals.var_vdb_noswap * locals.var_vdb_noswap_dn6)) * locals.var_vdb_noswap) + (assign26650_e40102 * locals.var_vdb_noswap_dn6)), ((((locals.var_vdb_noswap_dn7 * locals.var_vdb_noswap) + (locals.var_vdb_noswap * locals.var_vdb_noswap_dn7)) * locals.var_vdb_noswap) + (assign26650_e40102 * locals.var_vdb_noswap_dn7)), 0.0, 0.0, ((((locals.var_vdb_noswap_dn10 * locals.var_vdb_noswap) + (locals.var_vdb_noswap * locals.var_vdb_noswap_dn10)) * locals.var_vdb_noswap) + (assign26650_e40102 * locals.var_vdb_noswap_dn10)), 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign26650_e40106;
        locals.var_t3_dn3 = assign26650_e40106_d_n3;
        locals.var_t3_dn4 = assign26650_e40106_d_n4;
        locals.var_t3_dn5 = assign26650_e40106_d_n5;
        locals.var_t3_dn6 = assign26650_e40106_d_n6;
        locals.var_t3_dn7 = assign26650_e40106_d_n7;
        locals.var_t3_dn8 = assign26650_e40106_d_n8;
        locals.var_t3_dn9 = assign26650_e40106_d_n9;
        locals.var_t3_dn10 = assign26650_e40106_d_n10;
        locals.var_t3_dn11 = assign26650_e40106_d_n11;

        let (assign26660_e40122, assign26660_e40122_d_n3, assign26660_e40122_d_n4, assign26660_e40122_d_n5, assign26660_e40122_d_n6, assign26660_e40122_d_n7, assign26660_e40122_d_n8, assign26660_e40122_d_n9, assign26660_e40122_d_n10, assign26660_e40122_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard557 != 0.0)) && (locals.var_guard558 == 0.0)) && (locals.var_guard559 != 0.0)) {
        let assign26660_e40117: f64 = (locals.var_t3).abs();
        let assign26660_e40118: f64 = (locals.var_cgidl_i + assign26660_e40117);
        let assign26660_e40120: f64 = (assign26660_e40118 + 0.0001);
        (assign26660_e40120, if locals.var_t3 >= 0.0 { locals.var_t3_dn3 } else { (-locals.var_t3_dn3) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn4 } else { (-locals.var_t3_dn4) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn5 } else { (-locals.var_t3_dn5) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn6 } else { (-locals.var_t3_dn6) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn7 } else { (-locals.var_t3_dn7) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn8 } else { (-locals.var_t3_dn8) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn9 } else { (-locals.var_t3_dn9) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn10 } else { (-locals.var_t3_dn10) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn11 } else { (-locals.var_t3_dn11) },)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign26660_e40122;
        locals.var_t4_dn3 = assign26660_e40122_d_n3;
        locals.var_t4_dn4 = assign26660_e40122_d_n4;
        locals.var_t4_dn5 = assign26660_e40122_d_n5;
        locals.var_t4_dn6 = assign26660_e40122_d_n6;
        locals.var_t4_dn7 = assign26660_e40122_d_n7;
        locals.var_t4_dn8 = assign26660_e40122_d_n8;
        locals.var_t4_dn9 = assign26660_e40122_d_n9;
        locals.var_t4_dn10 = assign26660_e40122_d_n10;
        locals.var_t4_dn11 = assign26660_e40122_d_n11;

        let (assign26670_e40154, assign26670_e40154_d_n3, assign26670_e40154_d_n4, assign26670_e40154_d_n5, assign26670_e40154_d_n6, assign26670_e40154_d_n7, assign26670_e40154_d_n8, assign26670_e40154_d_n9, assign26670_e40154_d_n10, assign26670_e40154_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard557 != 0.0)) && (locals.var_guard558 == 0.0)) && (locals.var_guard559 != 0.0)) {
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_t4;
        let assign26670_e40134: f64 = (locals.var_t3 * __rspice_inv_cse_0);
        let assign26670_e40137: f64 = (locals.var_t3 * __rspice_inv_cse_0);
        let assign26670_e40140: f64 = (locals.var_t3 * __rspice_inv_cse_0);
        let assign26670_e40141: f64 = (assign26670_e40137 * assign26670_e40140);
        let assign26670_e40144: f64 = (4.0 * 1e-6);
        let assign26670_e40146: f64 = (assign26670_e40144 * 1e-6);
        let assign26670_e40147: f64 = (assign26670_e40141 + assign26670_e40146);
        let assign26670_e40148: f64 = (assign26670_e40147).sqrt();
        let assign26670_e40149: f64 = (assign26670_e40134 + assign26670_e40148);
        let assign26670_e40150: f64 = (0.5 * assign26670_e40149);
        let assign26670_e40152: f64 = (assign26670_e40150 - 1e-6);
        (assign26670_e40152, (0.5 * ((((locals.var_t3_dn3 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn3 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4)) * assign26670_e40140) + (assign26670_e40137 * (((locals.var_t3_dn3 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign26670_e40148)))), (0.5 * ((((locals.var_t3_dn4 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn4 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)) * assign26670_e40140) + (assign26670_e40137 * (((locals.var_t3_dn4 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign26670_e40148)))), (0.5 * ((((locals.var_t3_dn5 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn5 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)) * assign26670_e40140) + (assign26670_e40137 * (((locals.var_t3_dn5 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign26670_e40148)))), (0.5 * ((((locals.var_t3_dn6 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn6 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)) * assign26670_e40140) + (assign26670_e40137 * (((locals.var_t3_dn6 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign26670_e40148)))), (0.5 * ((((locals.var_t3_dn7 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn7 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)) * assign26670_e40140) + (assign26670_e40137 * (((locals.var_t3_dn7 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign26670_e40148)))), (0.5 * ((((locals.var_t3_dn8 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn8 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)) * assign26670_e40140) + (assign26670_e40137 * (((locals.var_t3_dn8 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign26670_e40148)))), (0.5 * ((((locals.var_t3_dn9 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn9 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)) * assign26670_e40140) + (assign26670_e40137 * (((locals.var_t3_dn9 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign26670_e40148)))), (0.5 * ((((locals.var_t3_dn10 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn10 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)) * assign26670_e40140) + (assign26670_e40137 * (((locals.var_t3_dn10 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign26670_e40148)))), (0.5 * ((((locals.var_t3_dn11 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn11 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)) * assign26670_e40140) + (assign26670_e40137 * (((locals.var_t3_dn11 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign26670_e40148)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign26670_e40154;
        locals.var_t5_dn3 = assign26670_e40154_d_n3;
        locals.var_t5_dn4 = assign26670_e40154_d_n4;
        locals.var_t5_dn5 = assign26670_e40154_d_n5;
        locals.var_t5_dn6 = assign26670_e40154_d_n6;
        locals.var_t5_dn7 = assign26670_e40154_d_n7;
        locals.var_t5_dn8 = assign26670_e40154_d_n8;
        locals.var_t5_dn9 = assign26670_e40154_d_n9;
        locals.var_t5_dn10 = assign26670_e40154_d_n10;
        locals.var_t5_dn11 = assign26670_e40154_d_n11;

        let (assign26680_e40166, assign26680_e40166_d_n3, assign26680_e40166_d_n4, assign26680_e40166_d_n5, assign26680_e40166_d_n6, assign26680_e40166_d_n7, assign26680_e40166_d_n8, assign26680_e40166_d_n9, assign26680_e40166_d_n10, assign26680_e40166_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard557 != 0.0)) && (locals.var_guard558 == 0.0)) && (locals.var_guard559 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign26680_e40166;
        locals.var_t5_dn3 = assign26680_e40166_d_n3;
        locals.var_t5_dn4 = assign26680_e40166_d_n4;
        locals.var_t5_dn5 = assign26680_e40166_d_n5;
        locals.var_t5_dn6 = assign26680_e40166_d_n6;
        locals.var_t5_dn7 = assign26680_e40166_d_n7;
        locals.var_t5_dn8 = assign26680_e40166_d_n8;
        locals.var_t5_dn9 = assign26680_e40166_d_n9;
        locals.var_t5_dn10 = assign26680_e40166_d_n10;
        locals.var_t5_dn11 = assign26680_e40166_d_n11;

        let (assign26690_e40185, assign26690_e40185_d_n3, assign26690_e40185_d_n4, assign26690_e40185_d_n5, assign26690_e40185_d_n6, assign26690_e40185_d_n7, assign26690_e40185_d_n8, assign26690_e40185_d_n9, assign26690_e40185_d_n10, assign26690_e40185_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard557 != 0.0)) && (locals.var_guard558 == 0.0)) {
        let assign26690_e40175: f64 = (locals.var_agidl_i * locals.var_wdiod);
        let assign26690_e40177: f64 = (assign26690_e40175 * locals.var_t1);
        let assign26690_e40179: f64 = (-locals.var_t2);
        let assign26690_e40180: f64 = { let limited_exp_arg = assign26690_e40179; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign26690_e40181: f64 = (assign26690_e40177 * assign26690_e40180);
        let assign26690_e40183: f64 = (assign26690_e40181 * locals.var_t5);
        (assign26690_e40183, (((((assign26690_e40175 * locals.var_t1_dn3) * assign26690_e40180) + (assign26690_e40177 * ({ let limited_exp_arg = assign26690_e40179; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn3)))) * locals.var_t5) + (assign26690_e40181 * locals.var_t5_dn3)), (((((assign26690_e40175 * locals.var_t1_dn4) * assign26690_e40180) + (assign26690_e40177 * ({ let limited_exp_arg = assign26690_e40179; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn4)))) * locals.var_t5) + (assign26690_e40181 * locals.var_t5_dn4)), (((((assign26690_e40175 * locals.var_t1_dn5) * assign26690_e40180) + (assign26690_e40177 * ({ let limited_exp_arg = assign26690_e40179; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn5)))) * locals.var_t5) + (assign26690_e40181 * locals.var_t5_dn5)), (((((assign26690_e40175 * locals.var_t1_dn6) * assign26690_e40180) + (assign26690_e40177 * ({ let limited_exp_arg = assign26690_e40179; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn6)))) * locals.var_t5) + (assign26690_e40181 * locals.var_t5_dn6)), (((((assign26690_e40175 * locals.var_t1_dn7) * assign26690_e40180) + (assign26690_e40177 * ({ let limited_exp_arg = assign26690_e40179; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn7)))) * locals.var_t5) + (assign26690_e40181 * locals.var_t5_dn7)), (((((assign26690_e40175 * locals.var_t1_dn8) * assign26690_e40180) + (assign26690_e40177 * ({ let limited_exp_arg = assign26690_e40179; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn8)))) * locals.var_t5) + (assign26690_e40181 * locals.var_t5_dn8)), (((((assign26690_e40175 * locals.var_t1_dn9) * assign26690_e40180) + (assign26690_e40177 * ({ let limited_exp_arg = assign26690_e40179; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn9)))) * locals.var_t5) + (assign26690_e40181 * locals.var_t5_dn9)), (((((assign26690_e40175 * locals.var_t1_dn10) * assign26690_e40180) + (assign26690_e40177 * ({ let limited_exp_arg = assign26690_e40179; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn10)))) * locals.var_t5) + (assign26690_e40181 * locals.var_t5_dn10)), (((((assign26690_e40175 * locals.var_t1_dn11) * assign26690_e40180) + (assign26690_e40177 * ({ let limited_exp_arg = assign26690_e40179; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn11)))) * locals.var_t5) + (assign26690_e40181 * locals.var_t5_dn11)),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign26690_e40185;
        locals.var_t6_dn3 = assign26690_e40185_d_n3;
        locals.var_t6_dn4 = assign26690_e40185_d_n4;
        locals.var_t6_dn5 = assign26690_e40185_d_n5;
        locals.var_t6_dn6 = assign26690_e40185_d_n6;
        locals.var_t6_dn7 = assign26690_e40185_d_n7;
        locals.var_t6_dn8 = assign26690_e40185_d_n8;
        locals.var_t6_dn9 = assign26690_e40185_d_n9;
        locals.var_t6_dn10 = assign26690_e40185_d_n10;
        locals.var_t6_dn11 = assign26690_e40185_d_n11;

        let assign26710_e40202: f64 = if (((locals.var_agisl_i <= 0.0) || (locals.var_bgisl_t <= 0.0)) || (locals.var_cgisl_i < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard560 = assign26710_e40202;

        let (assign26720_e40210, assign26720_e40210_d_n3, assign26720_e40210_d_n4, assign26720_e40210_d_n5, assign26720_e40210_d_n6, assign26720_e40210_d_n7, assign26720_e40210_d_n8, assign26720_e40210_d_n9, assign26720_e40210_d_n10, assign26720_e40210_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard557 != 0.0)) && (locals.var_guard560 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign26720_e40210;
        locals.var_t6_dn3 = assign26720_e40210_d_n3;
        locals.var_t6_dn4 = assign26720_e40210_d_n4;
        locals.var_t6_dn5 = assign26720_e40210_d_n5;
        locals.var_t6_dn6 = assign26720_e40210_d_n6;
        locals.var_t6_dn7 = assign26720_e40210_d_n7;
        locals.var_t6_dn8 = assign26720_e40210_d_n8;
        locals.var_t6_dn9 = assign26720_e40210_d_n9;
        locals.var_t6_dn10 = assign26720_e40210_d_n10;
        locals.var_t6_dn11 = assign26720_e40210_d_n11;

        let (assign26730_e40226, assign26730_e40226_d_n3, assign26730_e40226_d_n4, assign26730_e40226_d_n5, assign26730_e40226_d_n6, assign26730_e40226_d_n7, assign26730_e40226_d_n8, assign26730_e40226_d_n9, assign26730_e40226_d_n10, assign26730_e40226_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard557 != 0.0)) && (locals.var_guard560 == 0.0)) {
        let assign26730_e40218: f64 = (-locals.var_vgs_noswap);
        let assign26730_e40220: f64 = (assign26730_e40218 - locals.var_egisl_i);
        let assign26730_e40222: f64 = (assign26730_e40220 + locals.var_vfbsdr);
        let assign26730_e40224: f64 = (assign26730_e40222 / locals.var_t0);
        (assign26730_e40224, (-((assign26730_e40222 * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0))), (((locals.var_vfbsdr_dn4 * locals.var_t0) - (assign26730_e40222 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsdr_dn5 * locals.var_t0) - (assign26730_e40222 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgs_noswap_dn6) * locals.var_t0) - (assign26730_e40222 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgs_noswap_dn7) * locals.var_t0) - (assign26730_e40222 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgs_noswap_dn8) * locals.var_t0) - (assign26730_e40222 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (-((assign26730_e40222 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))), ((((-locals.var_vgs_noswap_dn10) * locals.var_t0) - (assign26730_e40222 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (-((assign26730_e40222 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign26730_e40226;
        locals.var_t1_dn3 = assign26730_e40226_d_n3;
        locals.var_t1_dn4 = assign26730_e40226_d_n4;
        locals.var_t1_dn5 = assign26730_e40226_d_n5;
        locals.var_t1_dn6 = assign26730_e40226_d_n6;
        locals.var_t1_dn7 = assign26730_e40226_d_n7;
        locals.var_t1_dn8 = assign26730_e40226_d_n8;
        locals.var_t1_dn9 = assign26730_e40226_d_n9;
        locals.var_t1_dn10 = assign26730_e40226_d_n10;
        locals.var_t1_dn11 = assign26730_e40226_d_n11;

        let (assign26740_e40248, assign26740_e40248_d_n3, assign26740_e40248_d_n4, assign26740_e40248_d_n5, assign26740_e40248_d_n6, assign26740_e40248_d_n7, assign26740_e40248_d_n8, assign26740_e40248_d_n9, assign26740_e40248_d_n10, assign26740_e40248_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard557 != 0.0)) && (locals.var_guard560 == 0.0)) {
        let assign26740_e40237: f64 = (locals.var_t1 * locals.var_t1);
        let assign26740_e40240: f64 = (4.0 * 0.01);
        let assign26740_e40242: f64 = (assign26740_e40240 * 0.01);
        let assign26740_e40243: f64 = (assign26740_e40237 + assign26740_e40242);
        let assign26740_e40244: f64 = (assign26740_e40243).sqrt();
        let assign26740_e40245: f64 = (locals.var_t1 + assign26740_e40244);
        let assign26740_e40246: f64 = (0.5 * assign26740_e40245);
        (assign26740_e40246, (0.5 * (locals.var_t1_dn3 + (((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)) / (2.0 * assign26740_e40244)))), (0.5 * (locals.var_t1_dn4 + (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign26740_e40244)))), (0.5 * (locals.var_t1_dn5 + (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign26740_e40244)))), (0.5 * (locals.var_t1_dn6 + (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign26740_e40244)))), (0.5 * (locals.var_t1_dn7 + (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign26740_e40244)))), (0.5 * (locals.var_t1_dn8 + (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign26740_e40244)))), (0.5 * (locals.var_t1_dn9 + (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign26740_e40244)))), (0.5 * (locals.var_t1_dn10 + (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign26740_e40244)))), (0.5 * (locals.var_t1_dn11 + (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign26740_e40244)))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign26740_e40248;
        locals.var_t1_dn3 = assign26740_e40248_d_n3;
        locals.var_t1_dn4 = assign26740_e40248_d_n4;
        locals.var_t1_dn5 = assign26740_e40248_d_n5;
        locals.var_t1_dn6 = assign26740_e40248_d_n6;
        locals.var_t1_dn7 = assign26740_e40248_d_n7;
        locals.var_t1_dn8 = assign26740_e40248_d_n8;
        locals.var_t1_dn9 = assign26740_e40248_d_n9;
        locals.var_t1_dn10 = assign26740_e40248_d_n10;
        locals.var_t1_dn11 = assign26740_e40248_d_n11;

        let (assign26750_e40261, assign26750_e40261_d_n3, assign26750_e40261_d_n4, assign26750_e40261_d_n5, assign26750_e40261_d_n6, assign26750_e40261_d_n7, assign26750_e40261_d_n8, assign26750_e40261_d_n9, assign26750_e40261_d_n10, assign26750_e40261_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard557 != 0.0)) && (locals.var_guard560 == 0.0)) {
        let assign26750_e40258: f64 = (locals.var_t1 + 0.001);
        let assign26750_e40259: f64 = (locals.var_bgisl_t / assign26750_e40258);
        (assign26750_e40259, (-((locals.var_bgisl_t * locals.var_t1_dn3) / (assign26750_e40258 * assign26750_e40258))), (((locals.var_bgisl_t_dn4 * assign26750_e40258) - (locals.var_bgisl_t * locals.var_t1_dn4)) / (assign26750_e40258 * assign26750_e40258)), (((locals.var_bgisl_t_dn5 * assign26750_e40258) - (locals.var_bgisl_t * locals.var_t1_dn5)) / (assign26750_e40258 * assign26750_e40258)), (-((locals.var_bgisl_t * locals.var_t1_dn6) / (assign26750_e40258 * assign26750_e40258))), (-((locals.var_bgisl_t * locals.var_t1_dn7) / (assign26750_e40258 * assign26750_e40258))), (-((locals.var_bgisl_t * locals.var_t1_dn8) / (assign26750_e40258 * assign26750_e40258))), (-((locals.var_bgisl_t * locals.var_t1_dn9) / (assign26750_e40258 * assign26750_e40258))), (-((locals.var_bgisl_t * locals.var_t1_dn10) / (assign26750_e40258 * assign26750_e40258))), (-((locals.var_bgisl_t * locals.var_t1_dn11) / (assign26750_e40258 * assign26750_e40258))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign26750_e40261;
        locals.var_t2_dn3 = assign26750_e40261_d_n3;
        locals.var_t2_dn4 = assign26750_e40261_d_n4;
        locals.var_t2_dn5 = assign26750_e40261_d_n5;
        locals.var_t2_dn6 = assign26750_e40261_d_n6;
        locals.var_t2_dn7 = assign26750_e40261_d_n7;
        locals.var_t2_dn8 = assign26750_e40261_d_n8;
        locals.var_t2_dn9 = assign26750_e40261_d_n9;
        locals.var_t2_dn10 = assign26750_e40261_d_n10;
        locals.var_t2_dn11 = assign26750_e40261_d_n11;

        let assign26760_e40264: f64 = if locals.var_cgisl_i != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard561 = assign26760_e40264;

        let (assign26770_e40279, assign26770_e40279_d_n3, assign26770_e40279_d_n4, assign26770_e40279_d_n5, assign26770_e40279_d_n6, assign26770_e40279_d_n7, assign26770_e40279_d_n8, assign26770_e40279_d_n9, assign26770_e40279_d_n10, assign26770_e40279_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard557 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard561 != 0.0)) {
        let assign26770_e40275: f64 = (locals.var_vsb_noswap * locals.var_vsb_noswap);
        let assign26770_e40277: f64 = (assign26770_e40275 * locals.var_vsb_noswap);
        (assign26770_e40277, 0.0, 0.0, 0.0, ((((locals.var_vsb_noswap_dn6 * locals.var_vsb_noswap) + (locals.var_vsb_noswap * locals.var_vsb_noswap_dn6)) * locals.var_vsb_noswap) + (assign26770_e40275 * locals.var_vsb_noswap_dn6)), ((((locals.var_vsb_noswap_dn7 * locals.var_vsb_noswap) + (locals.var_vsb_noswap * locals.var_vsb_noswap_dn7)) * locals.var_vsb_noswap) + (assign26770_e40275 * locals.var_vsb_noswap_dn7)), 0.0, 0.0, ((((locals.var_vsb_noswap_dn10 * locals.var_vsb_noswap) + (locals.var_vsb_noswap * locals.var_vsb_noswap_dn10)) * locals.var_vsb_noswap) + (assign26770_e40275 * locals.var_vsb_noswap_dn10)), 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign26770_e40279;
        locals.var_t3_dn3 = assign26770_e40279_d_n3;
        locals.var_t3_dn4 = assign26770_e40279_d_n4;
        locals.var_t3_dn5 = assign26770_e40279_d_n5;
        locals.var_t3_dn6 = assign26770_e40279_d_n6;
        locals.var_t3_dn7 = assign26770_e40279_d_n7;
        locals.var_t3_dn8 = assign26770_e40279_d_n8;
        locals.var_t3_dn9 = assign26770_e40279_d_n9;
        locals.var_t3_dn10 = assign26770_e40279_d_n10;
        locals.var_t3_dn11 = assign26770_e40279_d_n11;

        let (assign26780_e40295, assign26780_e40295_d_n3, assign26780_e40295_d_n4, assign26780_e40295_d_n5, assign26780_e40295_d_n6, assign26780_e40295_d_n7, assign26780_e40295_d_n8, assign26780_e40295_d_n9, assign26780_e40295_d_n10, assign26780_e40295_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard557 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard561 != 0.0)) {
        let assign26780_e40290: f64 = (locals.var_t3).abs();
        let assign26780_e40291: f64 = (locals.var_cgisl_i + assign26780_e40290);
        let assign26780_e40293: f64 = (assign26780_e40291 + 0.0001);
        (assign26780_e40293, if locals.var_t3 >= 0.0 { locals.var_t3_dn3 } else { (-locals.var_t3_dn3) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn4 } else { (-locals.var_t3_dn4) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn5 } else { (-locals.var_t3_dn5) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn6 } else { (-locals.var_t3_dn6) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn7 } else { (-locals.var_t3_dn7) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn8 } else { (-locals.var_t3_dn8) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn9 } else { (-locals.var_t3_dn9) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn10 } else { (-locals.var_t3_dn10) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn11 } else { (-locals.var_t3_dn11) },)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign26780_e40295;
        locals.var_t4_dn3 = assign26780_e40295_d_n3;
        locals.var_t4_dn4 = assign26780_e40295_d_n4;
        locals.var_t4_dn5 = assign26780_e40295_d_n5;
        locals.var_t4_dn6 = assign26780_e40295_d_n6;
        locals.var_t4_dn7 = assign26780_e40295_d_n7;
        locals.var_t4_dn8 = assign26780_e40295_d_n8;
        locals.var_t4_dn9 = assign26780_e40295_d_n9;
        locals.var_t4_dn10 = assign26780_e40295_d_n10;
        locals.var_t4_dn11 = assign26780_e40295_d_n11;

        let (assign26790_e40327, assign26790_e40327_d_n3, assign26790_e40327_d_n4, assign26790_e40327_d_n5, assign26790_e40327_d_n6, assign26790_e40327_d_n7, assign26790_e40327_d_n8, assign26790_e40327_d_n9, assign26790_e40327_d_n10, assign26790_e40327_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard557 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard561 != 0.0)) {
        let __rspice_inv_cse_1: f64 = 1.0 / locals.var_t4;
        let assign26790_e40307: f64 = (locals.var_t3 * __rspice_inv_cse_1);
        let assign26790_e40310: f64 = (locals.var_t3 * __rspice_inv_cse_1);
        let assign26790_e40313: f64 = (locals.var_t3 * __rspice_inv_cse_1);
        let assign26790_e40314: f64 = (assign26790_e40310 * assign26790_e40313);
        let assign26790_e40317: f64 = (4.0 * 1e-6);
        let assign26790_e40319: f64 = (assign26790_e40317 * 1e-6);
        let assign26790_e40320: f64 = (assign26790_e40314 + assign26790_e40319);
        let assign26790_e40321: f64 = (assign26790_e40320).sqrt();
        let assign26790_e40322: f64 = (assign26790_e40307 + assign26790_e40321);
        let assign26790_e40323: f64 = (0.5 * assign26790_e40322);
        let assign26790_e40325: f64 = (assign26790_e40323 - 1e-6);
        (assign26790_e40325, (0.5 * ((((locals.var_t3_dn3 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn3 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4)) * assign26790_e40313) + (assign26790_e40310 * (((locals.var_t3_dn3 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign26790_e40321)))), (0.5 * ((((locals.var_t3_dn4 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn4 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)) * assign26790_e40313) + (assign26790_e40310 * (((locals.var_t3_dn4 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign26790_e40321)))), (0.5 * ((((locals.var_t3_dn5 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn5 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)) * assign26790_e40313) + (assign26790_e40310 * (((locals.var_t3_dn5 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign26790_e40321)))), (0.5 * ((((locals.var_t3_dn6 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn6 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)) * assign26790_e40313) + (assign26790_e40310 * (((locals.var_t3_dn6 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign26790_e40321)))), (0.5 * ((((locals.var_t3_dn7 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn7 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)) * assign26790_e40313) + (assign26790_e40310 * (((locals.var_t3_dn7 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign26790_e40321)))), (0.5 * ((((locals.var_t3_dn8 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn8 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)) * assign26790_e40313) + (assign26790_e40310 * (((locals.var_t3_dn8 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign26790_e40321)))), (0.5 * ((((locals.var_t3_dn9 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn9 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)) * assign26790_e40313) + (assign26790_e40310 * (((locals.var_t3_dn9 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign26790_e40321)))), (0.5 * ((((locals.var_t3_dn10 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn10 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)) * assign26790_e40313) + (assign26790_e40310 * (((locals.var_t3_dn10 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign26790_e40321)))), (0.5 * ((((locals.var_t3_dn11 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn11 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)) * assign26790_e40313) + (assign26790_e40310 * (((locals.var_t3_dn11 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign26790_e40321)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign26790_e40327;
        locals.var_t5_dn3 = assign26790_e40327_d_n3;
        locals.var_t5_dn4 = assign26790_e40327_d_n4;
        locals.var_t5_dn5 = assign26790_e40327_d_n5;
        locals.var_t5_dn6 = assign26790_e40327_d_n6;
        locals.var_t5_dn7 = assign26790_e40327_d_n7;
        locals.var_t5_dn8 = assign26790_e40327_d_n8;
        locals.var_t5_dn9 = assign26790_e40327_d_n9;
        locals.var_t5_dn10 = assign26790_e40327_d_n10;
        locals.var_t5_dn11 = assign26790_e40327_d_n11;

        let (assign26800_e40339, assign26800_e40339_d_n3, assign26800_e40339_d_n4, assign26800_e40339_d_n5, assign26800_e40339_d_n6, assign26800_e40339_d_n7, assign26800_e40339_d_n8, assign26800_e40339_d_n9, assign26800_e40339_d_n10, assign26800_e40339_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard557 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard561 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign26800_e40339;
        locals.var_t5_dn3 = assign26800_e40339_d_n3;
        locals.var_t5_dn4 = assign26800_e40339_d_n4;
        locals.var_t5_dn5 = assign26800_e40339_d_n5;
        locals.var_t5_dn6 = assign26800_e40339_d_n6;
        locals.var_t5_dn7 = assign26800_e40339_d_n7;
        locals.var_t5_dn8 = assign26800_e40339_d_n8;
        locals.var_t5_dn9 = assign26800_e40339_d_n9;
        locals.var_t5_dn10 = assign26800_e40339_d_n10;
        locals.var_t5_dn11 = assign26800_e40339_d_n11;

        let (assign26810_e40358, assign26810_e40358_d_n3, assign26810_e40358_d_n4, assign26810_e40358_d_n5, assign26810_e40358_d_n6, assign26810_e40358_d_n7, assign26810_e40358_d_n8, assign26810_e40358_d_n9, assign26810_e40358_d_n10, assign26810_e40358_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard557 != 0.0)) && (locals.var_guard560 == 0.0)) {
        let assign26810_e40348: f64 = (locals.var_agisl_i * locals.var_wdios);
        let assign26810_e40350: f64 = (assign26810_e40348 * locals.var_t1);
        let assign26810_e40352: f64 = (-locals.var_t2);
        let assign26810_e40353: f64 = { let limited_exp_arg = assign26810_e40352; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign26810_e40354: f64 = (assign26810_e40350 * assign26810_e40353);
        let assign26810_e40356: f64 = (assign26810_e40354 * locals.var_t5);
        (assign26810_e40356, (((((assign26810_e40348 * locals.var_t1_dn3) * assign26810_e40353) + (assign26810_e40350 * ({ let limited_exp_arg = assign26810_e40352; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn3)))) * locals.var_t5) + (assign26810_e40354 * locals.var_t5_dn3)), (((((assign26810_e40348 * locals.var_t1_dn4) * assign26810_e40353) + (assign26810_e40350 * ({ let limited_exp_arg = assign26810_e40352; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn4)))) * locals.var_t5) + (assign26810_e40354 * locals.var_t5_dn4)), (((((assign26810_e40348 * locals.var_t1_dn5) * assign26810_e40353) + (assign26810_e40350 * ({ let limited_exp_arg = assign26810_e40352; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn5)))) * locals.var_t5) + (assign26810_e40354 * locals.var_t5_dn5)), (((((assign26810_e40348 * locals.var_t1_dn6) * assign26810_e40353) + (assign26810_e40350 * ({ let limited_exp_arg = assign26810_e40352; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn6)))) * locals.var_t5) + (assign26810_e40354 * locals.var_t5_dn6)), (((((assign26810_e40348 * locals.var_t1_dn7) * assign26810_e40353) + (assign26810_e40350 * ({ let limited_exp_arg = assign26810_e40352; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn7)))) * locals.var_t5) + (assign26810_e40354 * locals.var_t5_dn7)), (((((assign26810_e40348 * locals.var_t1_dn8) * assign26810_e40353) + (assign26810_e40350 * ({ let limited_exp_arg = assign26810_e40352; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn8)))) * locals.var_t5) + (assign26810_e40354 * locals.var_t5_dn8)), (((((assign26810_e40348 * locals.var_t1_dn9) * assign26810_e40353) + (assign26810_e40350 * ({ let limited_exp_arg = assign26810_e40352; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn9)))) * locals.var_t5) + (assign26810_e40354 * locals.var_t5_dn9)), (((((assign26810_e40348 * locals.var_t1_dn10) * assign26810_e40353) + (assign26810_e40350 * ({ let limited_exp_arg = assign26810_e40352; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn10)))) * locals.var_t5) + (assign26810_e40354 * locals.var_t5_dn10)), (((((assign26810_e40348 * locals.var_t1_dn11) * assign26810_e40353) + (assign26810_e40350 * ({ let limited_exp_arg = assign26810_e40352; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn11)))) * locals.var_t5) + (assign26810_e40354 * locals.var_t5_dn11)),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign26810_e40358;
        locals.var_t6_dn3 = assign26810_e40358_d_n3;
        locals.var_t6_dn4 = assign26810_e40358_d_n4;
        locals.var_t6_dn5 = assign26810_e40358_d_n5;
        locals.var_t6_dn6 = assign26810_e40358_d_n6;
        locals.var_t6_dn7 = assign26810_e40358_d_n7;
        locals.var_t6_dn8 = assign26810_e40358_d_n8;
        locals.var_t6_dn9 = assign26810_e40358_d_n9;
        locals.var_t6_dn10 = assign26810_e40358_d_n10;
        locals.var_t6_dn11 = assign26810_e40358_d_n11;

        let (assign26830_e40373, assign26830_e40373_d_n3, assign26830_e40373_d_n4, assign26830_e40373_d_n5, assign26830_e40373_d_n6, assign26830_e40373_d_n7, assign26830_e40373_d_n8, assign26830_e40373_d_n9, assign26830_e40373_d_n10, assign26830_e40373_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard557 == 0.0)) {
        let assign26830_e40371: f64 = (locals.var_epsratio * p.p76);
        (assign26830_e40371, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign26830_e40373;
        locals.var_t0_dn3 = assign26830_e40373_d_n3;
        locals.var_t0_dn4 = assign26830_e40373_d_n4;
        locals.var_t0_dn5 = assign26830_e40373_d_n5;
        locals.var_t0_dn6 = assign26830_e40373_d_n6;
        locals.var_t0_dn7 = assign26830_e40373_d_n7;
        locals.var_t0_dn8 = assign26830_e40373_d_n8;
        locals.var_t0_dn9 = assign26830_e40373_d_n9;
        locals.var_t0_dn10 = assign26830_e40373_d_n10;
        locals.var_t0_dn11 = assign26830_e40373_d_n11;

        let (assign26840_e40384, assign26840_e40384_d_n6, assign26840_e40384_d_n7, assign26840_e40384_d_n8, assign26840_e40384_d_n10,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard557 == 0.0)) {
        let assign26840_e40380: f64 = (locals.var_rgisl_i * locals.var_vg);
        let assign26840_e40382: f64 = (assign26840_e40380 - locals.var_vd);
        (assign26840_e40382, (-locals.var_vd_dn6), (-locals.var_vd_dn7), (locals.var_rgisl_i * locals.var_vg_dn8), ((locals.var_rgisl_i * locals.var_vg_dn10) - locals.var_vd_dn10),)
    } else {
        (locals.var_vgd_noswap_1, locals.var_vgd_noswap_1_dn6, locals.var_vgd_noswap_1_dn7, locals.var_vgd_noswap_1_dn8, locals.var_vgd_noswap_1_dn10,)
    }
};
        locals.var_vgd_noswap_1 = assign26840_e40384;
        locals.var_vgd_noswap_1_dn6 = assign26840_e40384_d_n6;
        locals.var_vgd_noswap_1_dn7 = assign26840_e40384_d_n7;
        locals.var_vgd_noswap_1_dn8 = assign26840_e40384_d_n8;
        locals.var_vgd_noswap_1_dn10 = assign26840_e40384_d_n10;

        let (assign26850_e40395, assign26850_e40395_d_n6, assign26850_e40395_d_n7, assign26850_e40395_d_n8, assign26850_e40395_d_n10,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard557 == 0.0)) {
        let assign26850_e40391: f64 = (locals.var_rgidl_i * locals.var_vg);
        let assign26850_e40393: f64 = (assign26850_e40391 - locals.var_vs);
        (assign26850_e40393, (-locals.var_vs_dn6), (-locals.var_vs_dn7), (locals.var_rgidl_i * locals.var_vg_dn8), ((locals.var_rgidl_i * locals.var_vg_dn10) - locals.var_vs_dn10),)
    } else {
        (locals.var_vgs_noswap_1, locals.var_vgs_noswap_1_dn6, locals.var_vgs_noswap_1_dn7, locals.var_vgs_noswap_1_dn8, locals.var_vgs_noswap_1_dn10,)
    }
};
        locals.var_vgs_noswap_1 = assign26850_e40395;
        locals.var_vgs_noswap_1_dn6 = assign26850_e40395_d_n6;
        locals.var_vgs_noswap_1_dn7 = assign26850_e40395_d_n7;
        locals.var_vgs_noswap_1_dn8 = assign26850_e40395_d_n8;
        locals.var_vgs_noswap_1_dn10 = assign26850_e40395_d_n10;

    }

    pub(super) fn stamp_transient_block_71(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign26860_e40404, assign26860_e40404_d_n3, assign26860_e40404_d_n4, assign26860_e40404_d_n5, assign26860_e40404_d_n6, assign26860_e40404_d_n7, assign26860_e40404_d_n8, assign26860_e40404_d_n9, assign26860_e40404_d_n10, assign26860_e40404_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard557 == 0.0)) {
        let assign26860_e40402: f64 = (locals.var_vgs_noswap - locals.var_vfbsdr);
        (assign26860_e40402, 0.0, (-locals.var_vfbsdr_dn4), (-locals.var_vfbsdr_dn5), locals.var_vgs_noswap_dn6, locals.var_vgs_noswap_dn7, locals.var_vgs_noswap_dn8, 0.0, locals.var_vgs_noswap_dn10, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign26860_e40404;
        locals.var_t2_dn3 = assign26860_e40404_d_n3;
        locals.var_t2_dn4 = assign26860_e40404_d_n4;
        locals.var_t2_dn5 = assign26860_e40404_d_n5;
        locals.var_t2_dn6 = assign26860_e40404_d_n6;
        locals.var_t2_dn7 = assign26860_e40404_d_n7;
        locals.var_t2_dn8 = assign26860_e40404_d_n8;
        locals.var_t2_dn9 = assign26860_e40404_d_n9;
        locals.var_t2_dn10 = assign26860_e40404_d_n10;
        locals.var_t2_dn11 = assign26860_e40404_d_n11;

        let (assign26870_e40416, assign26870_e40416_d_n3, assign26870_e40416_d_n4, assign26870_e40416_d_n5, assign26870_e40416_d_n6, assign26870_e40416_d_n7, assign26870_e40416_d_n8, assign26870_e40416_d_n9, assign26870_e40416_d_n10, assign26870_e40416_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard557 == 0.0)) {
        let assign26870_e40411: f64 = (locals.var_t2 * locals.var_t2);
        let assign26870_e40413: f64 = (assign26870_e40411 + 0.0001);
        let assign26870_e40414: f64 = (assign26870_e40413).sqrt();
        (assign26870_e40414, (((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign26870_e40414)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign26870_e40414)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign26870_e40414)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign26870_e40414)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign26870_e40414)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign26870_e40414)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign26870_e40414)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign26870_e40414)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign26870_e40414)),)
    } else {
        (locals.var_vgs_eff, locals.var_vgs_eff_dn3, locals.var_vgs_eff_dn4, locals.var_vgs_eff_dn5, locals.var_vgs_eff_dn6, locals.var_vgs_eff_dn7, locals.var_vgs_eff_dn8, locals.var_vgs_eff_dn9, locals.var_vgs_eff_dn10, locals.var_vgs_eff_dn11,)
    }
};
        locals.var_vgs_eff = assign26870_e40416;
        locals.var_vgs_eff_dn3 = assign26870_e40416_d_n3;
        locals.var_vgs_eff_dn4 = assign26870_e40416_d_n4;
        locals.var_vgs_eff_dn5 = assign26870_e40416_d_n5;
        locals.var_vgs_eff_dn6 = assign26870_e40416_d_n6;
        locals.var_vgs_eff_dn7 = assign26870_e40416_d_n7;
        locals.var_vgs_eff_dn8 = assign26870_e40416_d_n8;
        locals.var_vgs_eff_dn9 = assign26870_e40416_d_n9;
        locals.var_vgs_eff_dn10 = assign26870_e40416_d_n10;
        locals.var_vgs_eff_dn11 = assign26870_e40416_d_n11;

        let assign26880_e40423: f64 = if ((locals.var_agidl_i <= 0.0) || (locals.var_bgidl_t <= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard562 = assign26880_e40423;

        let (assign26890_e40432, assign26890_e40432_d_n3, assign26890_e40432_d_n4, assign26890_e40432_d_n5, assign26890_e40432_d_n6, assign26890_e40432_d_n7, assign26890_e40432_d_n8, assign26890_e40432_d_n9, assign26890_e40432_d_n10, assign26890_e40432_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard557 == 0.0)) && (locals.var_guard562 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign26890_e40432;
        locals.var_t6_dn3 = assign26890_e40432_d_n3;
        locals.var_t6_dn4 = assign26890_e40432_d_n4;
        locals.var_t6_dn5 = assign26890_e40432_d_n5;
        locals.var_t6_dn6 = assign26890_e40432_d_n6;
        locals.var_t6_dn7 = assign26890_e40432_d_n7;
        locals.var_t6_dn8 = assign26890_e40432_d_n8;
        locals.var_t6_dn9 = assign26890_e40432_d_n9;
        locals.var_t6_dn10 = assign26890_e40432_d_n10;
        locals.var_t6_dn11 = assign26890_e40432_d_n11;

        let (assign26900_e40449, assign26900_e40449_d_n3, assign26900_e40449_d_n4, assign26900_e40449_d_n5, assign26900_e40449_d_n6, assign26900_e40449_d_n7, assign26900_e40449_d_n8, assign26900_e40449_d_n9, assign26900_e40449_d_n10, assign26900_e40449_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard557 == 0.0)) && (locals.var_guard562 == 0.0)) {
        let assign26900_e40441: f64 = (-locals.var_vgd_noswap_1);
        let assign26900_e40443: f64 = (assign26900_e40441 - locals.var_egidl_i);
        let assign26900_e40445: f64 = (assign26900_e40443 + locals.var_vfbsdr);
        let assign26900_e40447: f64 = (assign26900_e40445 / locals.var_t0);
        (assign26900_e40447, (-((assign26900_e40445 * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0))), (((locals.var_vfbsdr_dn4 * locals.var_t0) - (assign26900_e40445 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsdr_dn5 * locals.var_t0) - (assign26900_e40445 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgd_noswap_1_dn6) * locals.var_t0) - (assign26900_e40445 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgd_noswap_1_dn7) * locals.var_t0) - (assign26900_e40445 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgd_noswap_1_dn8) * locals.var_t0) - (assign26900_e40445 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (-((assign26900_e40445 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))), ((((-locals.var_vgd_noswap_1_dn10) * locals.var_t0) - (assign26900_e40445 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (-((assign26900_e40445 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign26900_e40449;
        locals.var_t1_dn3 = assign26900_e40449_d_n3;
        locals.var_t1_dn4 = assign26900_e40449_d_n4;
        locals.var_t1_dn5 = assign26900_e40449_d_n5;
        locals.var_t1_dn6 = assign26900_e40449_d_n6;
        locals.var_t1_dn7 = assign26900_e40449_d_n7;
        locals.var_t1_dn8 = assign26900_e40449_d_n8;
        locals.var_t1_dn9 = assign26900_e40449_d_n9;
        locals.var_t1_dn10 = assign26900_e40449_d_n10;
        locals.var_t1_dn11 = assign26900_e40449_d_n11;

        let (assign26910_e40472, assign26910_e40472_d_n3, assign26910_e40472_d_n4, assign26910_e40472_d_n5, assign26910_e40472_d_n6, assign26910_e40472_d_n7, assign26910_e40472_d_n8, assign26910_e40472_d_n9, assign26910_e40472_d_n10, assign26910_e40472_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard557 == 0.0)) && (locals.var_guard562 == 0.0)) {
        let assign26910_e40461: f64 = (locals.var_t1 * locals.var_t1);
        let assign26910_e40464: f64 = (4.0 * 0.01);
        let assign26910_e40466: f64 = (assign26910_e40464 * 0.01);
        let assign26910_e40467: f64 = (assign26910_e40461 + assign26910_e40466);
        let assign26910_e40468: f64 = (assign26910_e40467).sqrt();
        let assign26910_e40469: f64 = (locals.var_t1 + assign26910_e40468);
        let assign26910_e40470: f64 = (0.5 * assign26910_e40469);
        (assign26910_e40470, (0.5 * (locals.var_t1_dn3 + (((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)) / (2.0 * assign26910_e40468)))), (0.5 * (locals.var_t1_dn4 + (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign26910_e40468)))), (0.5 * (locals.var_t1_dn5 + (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign26910_e40468)))), (0.5 * (locals.var_t1_dn6 + (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign26910_e40468)))), (0.5 * (locals.var_t1_dn7 + (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign26910_e40468)))), (0.5 * (locals.var_t1_dn8 + (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign26910_e40468)))), (0.5 * (locals.var_t1_dn9 + (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign26910_e40468)))), (0.5 * (locals.var_t1_dn10 + (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign26910_e40468)))), (0.5 * (locals.var_t1_dn11 + (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign26910_e40468)))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign26910_e40472;
        locals.var_t1_dn3 = assign26910_e40472_d_n3;
        locals.var_t1_dn4 = assign26910_e40472_d_n4;
        locals.var_t1_dn5 = assign26910_e40472_d_n5;
        locals.var_t1_dn6 = assign26910_e40472_d_n6;
        locals.var_t1_dn7 = assign26910_e40472_d_n7;
        locals.var_t1_dn8 = assign26910_e40472_d_n8;
        locals.var_t1_dn9 = assign26910_e40472_d_n9;
        locals.var_t1_dn10 = assign26910_e40472_d_n10;
        locals.var_t1_dn11 = assign26910_e40472_d_n11;

        let (assign26920_e40486, assign26920_e40486_d_n3, assign26920_e40486_d_n4, assign26920_e40486_d_n5, assign26920_e40486_d_n6, assign26920_e40486_d_n7, assign26920_e40486_d_n8, assign26920_e40486_d_n9, assign26920_e40486_d_n10, assign26920_e40486_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard557 == 0.0)) && (locals.var_guard562 == 0.0)) {
        let assign26920_e40483: f64 = (locals.var_t1 + 0.001);
        let assign26920_e40484: f64 = (locals.var_bgidl_t / assign26920_e40483);
        (assign26920_e40484, (-((locals.var_bgidl_t * locals.var_t1_dn3) / (assign26920_e40483 * assign26920_e40483))), (((locals.var_bgidl_t_dn4 * assign26920_e40483) - (locals.var_bgidl_t * locals.var_t1_dn4)) / (assign26920_e40483 * assign26920_e40483)), (((locals.var_bgidl_t_dn5 * assign26920_e40483) - (locals.var_bgidl_t * locals.var_t1_dn5)) / (assign26920_e40483 * assign26920_e40483)), (-((locals.var_bgidl_t * locals.var_t1_dn6) / (assign26920_e40483 * assign26920_e40483))), (-((locals.var_bgidl_t * locals.var_t1_dn7) / (assign26920_e40483 * assign26920_e40483))), (-((locals.var_bgidl_t * locals.var_t1_dn8) / (assign26920_e40483 * assign26920_e40483))), (-((locals.var_bgidl_t * locals.var_t1_dn9) / (assign26920_e40483 * assign26920_e40483))), (-((locals.var_bgidl_t * locals.var_t1_dn10) / (assign26920_e40483 * assign26920_e40483))), (-((locals.var_bgidl_t * locals.var_t1_dn11) / (assign26920_e40483 * assign26920_e40483))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign26920_e40486;
        locals.var_t2_dn3 = assign26920_e40486_d_n3;
        locals.var_t2_dn4 = assign26920_e40486_d_n4;
        locals.var_t2_dn5 = assign26920_e40486_d_n5;
        locals.var_t2_dn6 = assign26920_e40486_d_n6;
        locals.var_t2_dn7 = assign26920_e40486_d_n7;
        locals.var_t2_dn8 = assign26920_e40486_d_n8;
        locals.var_t2_dn9 = assign26920_e40486_d_n9;
        locals.var_t2_dn10 = assign26920_e40486_d_n10;
        locals.var_t2_dn11 = assign26920_e40486_d_n11;

        let assign26930_e40489: f64 = if locals.var_kgidl_i != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard563 = assign26930_e40489;

        let (assign26940_e40504, assign26940_e40504_d_n3, assign26940_e40504_d_n4, assign26940_e40504_d_n5, assign26940_e40504_d_n6, assign26940_e40504_d_n7, assign26940_e40504_d_n8, assign26940_e40504_d_n9, assign26940_e40504_d_n10, assign26940_e40504_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard557 == 0.0)) && (locals.var_guard562 == 0.0)) && (locals.var_guard563 != 0.0)) {
        let assign26940_e40500: f64 = (-locals.var_vdb_noswap);
        let assign26940_e40502: f64 = (assign26940_e40500 - locals.var_fgidl_i);
        (assign26940_e40502, 0.0, 0.0, 0.0, (-locals.var_vdb_noswap_dn6), (-locals.var_vdb_noswap_dn7), 0.0, 0.0, (-locals.var_vdb_noswap_dn10), 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign26940_e40504;
        locals.var_t3_dn3 = assign26940_e40504_d_n3;
        locals.var_t3_dn4 = assign26940_e40504_d_n4;
        locals.var_t3_dn5 = assign26940_e40504_d_n5;
        locals.var_t3_dn6 = assign26940_e40504_d_n6;
        locals.var_t3_dn7 = assign26940_e40504_d_n7;
        locals.var_t3_dn8 = assign26940_e40504_d_n8;
        locals.var_t3_dn9 = assign26940_e40504_d_n9;
        locals.var_t3_dn10 = assign26940_e40504_d_n10;
        locals.var_t3_dn11 = assign26940_e40504_d_n11;

        let (assign26950_e40518, assign26950_e40518_d_n3, assign26950_e40518_d_n4, assign26950_e40518_d_n5, assign26950_e40518_d_n6, assign26950_e40518_d_n7, assign26950_e40518_d_n8, assign26950_e40518_d_n9, assign26950_e40518_d_n10, assign26950_e40518_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard557 == 0.0)) && (locals.var_guard562 == 0.0)) && (locals.var_guard563 != 0.0)) {
        let assign26950_e40516: f64 = (locals.var_t3 + 0.0001);
        (assign26950_e40516, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign26950_e40518;
        locals.var_t4_dn3 = assign26950_e40518_d_n3;
        locals.var_t4_dn4 = assign26950_e40518_d_n4;
        locals.var_t4_dn5 = assign26950_e40518_d_n5;
        locals.var_t4_dn6 = assign26950_e40518_d_n6;
        locals.var_t4_dn7 = assign26950_e40518_d_n7;
        locals.var_t4_dn8 = assign26950_e40518_d_n8;
        locals.var_t4_dn9 = assign26950_e40518_d_n9;
        locals.var_t4_dn10 = assign26950_e40518_d_n10;
        locals.var_t4_dn11 = assign26950_e40518_d_n11;

        let (assign26960_e40551, assign26960_e40551_d_n3, assign26960_e40551_d_n4, assign26960_e40551_d_n5, assign26960_e40551_d_n6, assign26960_e40551_d_n7, assign26960_e40551_d_n8, assign26960_e40551_d_n9, assign26960_e40551_d_n10, assign26960_e40551_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard557 == 0.0)) && (locals.var_guard562 == 0.0)) && (locals.var_guard563 != 0.0)) {
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_t4;
        let assign26960_e40531: f64 = (locals.var_kgidl_i * __rspice_inv_cse_0);
        let assign26960_e40534: f64 = (locals.var_kgidl_i * __rspice_inv_cse_0);
        let assign26960_e40537: f64 = (locals.var_kgidl_i * __rspice_inv_cse_0);
        let assign26960_e40538: f64 = (assign26960_e40534 * assign26960_e40537);
        let assign26960_e40541: f64 = (4.0 * 1e-6);
        let assign26960_e40543: f64 = (assign26960_e40541 * 1e-6);
        let assign26960_e40544: f64 = (assign26960_e40538 + assign26960_e40543);
        let assign26960_e40545: f64 = (assign26960_e40544).sqrt();
        let assign26960_e40546: f64 = (assign26960_e40531 + assign26960_e40545);
        let assign26960_e40547: f64 = (0.5 * assign26960_e40546);
        let assign26960_e40549: f64 = (assign26960_e40547 - 1e-6);
        (assign26960_e40549, (0.5 * ((-((locals.var_kgidl_i * locals.var_t4_dn3) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgidl_i * locals.var_t4_dn3) / (locals.var_t4 * locals.var_t4))) * assign26960_e40537) + (assign26960_e40534 * (-((locals.var_kgidl_i * locals.var_t4_dn3) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign26960_e40545)))), (0.5 * ((-((locals.var_kgidl_i * locals.var_t4_dn4) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgidl_i * locals.var_t4_dn4) / (locals.var_t4 * locals.var_t4))) * assign26960_e40537) + (assign26960_e40534 * (-((locals.var_kgidl_i * locals.var_t4_dn4) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign26960_e40545)))), (0.5 * ((-((locals.var_kgidl_i * locals.var_t4_dn5) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgidl_i * locals.var_t4_dn5) / (locals.var_t4 * locals.var_t4))) * assign26960_e40537) + (assign26960_e40534 * (-((locals.var_kgidl_i * locals.var_t4_dn5) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign26960_e40545)))), (0.5 * ((-((locals.var_kgidl_i * locals.var_t4_dn6) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgidl_i * locals.var_t4_dn6) / (locals.var_t4 * locals.var_t4))) * assign26960_e40537) + (assign26960_e40534 * (-((locals.var_kgidl_i * locals.var_t4_dn6) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign26960_e40545)))), (0.5 * ((-((locals.var_kgidl_i * locals.var_t4_dn7) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgidl_i * locals.var_t4_dn7) / (locals.var_t4 * locals.var_t4))) * assign26960_e40537) + (assign26960_e40534 * (-((locals.var_kgidl_i * locals.var_t4_dn7) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign26960_e40545)))), (0.5 * ((-((locals.var_kgidl_i * locals.var_t4_dn8) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgidl_i * locals.var_t4_dn8) / (locals.var_t4 * locals.var_t4))) * assign26960_e40537) + (assign26960_e40534 * (-((locals.var_kgidl_i * locals.var_t4_dn8) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign26960_e40545)))), (0.5 * ((-((locals.var_kgidl_i * locals.var_t4_dn9) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgidl_i * locals.var_t4_dn9) / (locals.var_t4 * locals.var_t4))) * assign26960_e40537) + (assign26960_e40534 * (-((locals.var_kgidl_i * locals.var_t4_dn9) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign26960_e40545)))), (0.5 * ((-((locals.var_kgidl_i * locals.var_t4_dn10) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgidl_i * locals.var_t4_dn10) / (locals.var_t4 * locals.var_t4))) * assign26960_e40537) + (assign26960_e40534 * (-((locals.var_kgidl_i * locals.var_t4_dn10) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign26960_e40545)))), (0.5 * ((-((locals.var_kgidl_i * locals.var_t4_dn11) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgidl_i * locals.var_t4_dn11) / (locals.var_t4 * locals.var_t4))) * assign26960_e40537) + (assign26960_e40534 * (-((locals.var_kgidl_i * locals.var_t4_dn11) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign26960_e40545)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign26960_e40551;
        locals.var_t5_dn3 = assign26960_e40551_d_n3;
        locals.var_t5_dn4 = assign26960_e40551_d_n4;
        locals.var_t5_dn5 = assign26960_e40551_d_n5;
        locals.var_t5_dn6 = assign26960_e40551_d_n6;
        locals.var_t5_dn7 = assign26960_e40551_d_n7;
        locals.var_t5_dn8 = assign26960_e40551_d_n8;
        locals.var_t5_dn9 = assign26960_e40551_d_n9;
        locals.var_t5_dn10 = assign26960_e40551_d_n10;
        locals.var_t5_dn11 = assign26960_e40551_d_n11;

        let (assign26970_e40564, assign26970_e40564_d_n3, assign26970_e40564_d_n4, assign26970_e40564_d_n5, assign26970_e40564_d_n6, assign26970_e40564_d_n7, assign26970_e40564_d_n8, assign26970_e40564_d_n9, assign26970_e40564_d_n10, assign26970_e40564_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard557 == 0.0)) && (locals.var_guard562 == 0.0)) && (locals.var_guard563 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign26970_e40564;
        locals.var_t5_dn3 = assign26970_e40564_d_n3;
        locals.var_t5_dn4 = assign26970_e40564_d_n4;
        locals.var_t5_dn5 = assign26970_e40564_d_n5;
        locals.var_t5_dn6 = assign26970_e40564_d_n6;
        locals.var_t5_dn7 = assign26970_e40564_d_n7;
        locals.var_t5_dn8 = assign26970_e40564_d_n8;
        locals.var_t5_dn9 = assign26970_e40564_d_n9;
        locals.var_t5_dn10 = assign26970_e40564_d_n10;
        locals.var_t5_dn11 = assign26970_e40564_d_n11;

        let (assign26980_e40585, assign26980_e40585_d_n3, assign26980_e40585_d_n4, assign26980_e40585_d_n5, assign26980_e40585_d_n6, assign26980_e40585_d_n7, assign26980_e40585_d_n8, assign26980_e40585_d_n9, assign26980_e40585_d_n10, assign26980_e40585_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard557 == 0.0)) && (locals.var_guard562 == 0.0)) {
        let assign26980_e40574: f64 = (locals.var_agidl_i * locals.var_wdiod);
        let assign26980_e40576: f64 = (assign26980_e40574 * locals.var_t1);
        let assign26980_e40578: f64 = (-locals.var_t2);
        let assign26980_e40579: f64 = { let limited_exp_arg = assign26980_e40578; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign26980_e40580: f64 = (assign26980_e40576 * assign26980_e40579);
        let assign26980_e40582: f64 = { let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign26980_e40583: f64 = (assign26980_e40580 * assign26980_e40582);
        (assign26980_e40583, (((((assign26980_e40574 * locals.var_t1_dn3) * assign26980_e40579) + (assign26980_e40576 * ({ let limited_exp_arg = assign26980_e40578; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn3)))) * assign26980_e40582) + (assign26980_e40580 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn3))), (((((assign26980_e40574 * locals.var_t1_dn4) * assign26980_e40579) + (assign26980_e40576 * ({ let limited_exp_arg = assign26980_e40578; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn4)))) * assign26980_e40582) + (assign26980_e40580 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn4))), (((((assign26980_e40574 * locals.var_t1_dn5) * assign26980_e40579) + (assign26980_e40576 * ({ let limited_exp_arg = assign26980_e40578; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn5)))) * assign26980_e40582) + (assign26980_e40580 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn5))), (((((assign26980_e40574 * locals.var_t1_dn6) * assign26980_e40579) + (assign26980_e40576 * ({ let limited_exp_arg = assign26980_e40578; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn6)))) * assign26980_e40582) + (assign26980_e40580 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn6))), (((((assign26980_e40574 * locals.var_t1_dn7) * assign26980_e40579) + (assign26980_e40576 * ({ let limited_exp_arg = assign26980_e40578; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn7)))) * assign26980_e40582) + (assign26980_e40580 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn7))), (((((assign26980_e40574 * locals.var_t1_dn8) * assign26980_e40579) + (assign26980_e40576 * ({ let limited_exp_arg = assign26980_e40578; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn8)))) * assign26980_e40582) + (assign26980_e40580 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn8))), (((((assign26980_e40574 * locals.var_t1_dn9) * assign26980_e40579) + (assign26980_e40576 * ({ let limited_exp_arg = assign26980_e40578; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn9)))) * assign26980_e40582) + (assign26980_e40580 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn9))), (((((assign26980_e40574 * locals.var_t1_dn10) * assign26980_e40579) + (assign26980_e40576 * ({ let limited_exp_arg = assign26980_e40578; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn10)))) * assign26980_e40582) + (assign26980_e40580 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn10))), (((((assign26980_e40574 * locals.var_t1_dn11) * assign26980_e40579) + (assign26980_e40576 * ({ let limited_exp_arg = assign26980_e40578; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn11)))) * assign26980_e40582) + (assign26980_e40580 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn11))),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign26980_e40585;
        locals.var_t6_dn3 = assign26980_e40585_d_n3;
        locals.var_t6_dn4 = assign26980_e40585_d_n4;
        locals.var_t6_dn5 = assign26980_e40585_d_n5;
        locals.var_t6_dn6 = assign26980_e40585_d_n6;
        locals.var_t6_dn7 = assign26980_e40585_d_n7;
        locals.var_t6_dn8 = assign26980_e40585_d_n8;
        locals.var_t6_dn9 = assign26980_e40585_d_n9;
        locals.var_t6_dn10 = assign26980_e40585_d_n10;
        locals.var_t6_dn11 = assign26980_e40585_d_n11;

        let assign27000_e40599: f64 = if ((locals.var_agisl_i <= 0.0) || (locals.var_bgisl_t <= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard564 = assign27000_e40599;

        let (assign27010_e40608, assign27010_e40608_d_n3, assign27010_e40608_d_n4, assign27010_e40608_d_n5, assign27010_e40608_d_n6, assign27010_e40608_d_n7, assign27010_e40608_d_n8, assign27010_e40608_d_n9, assign27010_e40608_d_n10, assign27010_e40608_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard557 == 0.0)) && (locals.var_guard564 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign27010_e40608;
        locals.var_t6_dn3 = assign27010_e40608_d_n3;
        locals.var_t6_dn4 = assign27010_e40608_d_n4;
        locals.var_t6_dn5 = assign27010_e40608_d_n5;
        locals.var_t6_dn6 = assign27010_e40608_d_n6;
        locals.var_t6_dn7 = assign27010_e40608_d_n7;
        locals.var_t6_dn8 = assign27010_e40608_d_n8;
        locals.var_t6_dn9 = assign27010_e40608_d_n9;
        locals.var_t6_dn10 = assign27010_e40608_d_n10;
        locals.var_t6_dn11 = assign27010_e40608_d_n11;

        let (assign27020_e40625, assign27020_e40625_d_n3, assign27020_e40625_d_n4, assign27020_e40625_d_n5, assign27020_e40625_d_n6, assign27020_e40625_d_n7, assign27020_e40625_d_n8, assign27020_e40625_d_n9, assign27020_e40625_d_n10, assign27020_e40625_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard557 == 0.0)) && (locals.var_guard564 == 0.0)) {
        let assign27020_e40617: f64 = (-locals.var_vgs_noswap_1);
        let assign27020_e40619: f64 = (assign27020_e40617 - locals.var_egisl_i);
        let assign27020_e40621: f64 = (assign27020_e40619 + locals.var_vfbsdr);
        let assign27020_e40623: f64 = (assign27020_e40621 / locals.var_t0);
        (assign27020_e40623, (-((assign27020_e40621 * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0))), (((locals.var_vfbsdr_dn4 * locals.var_t0) - (assign27020_e40621 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsdr_dn5 * locals.var_t0) - (assign27020_e40621 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgs_noswap_1_dn6) * locals.var_t0) - (assign27020_e40621 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgs_noswap_1_dn7) * locals.var_t0) - (assign27020_e40621 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgs_noswap_1_dn8) * locals.var_t0) - (assign27020_e40621 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (-((assign27020_e40621 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))), ((((-locals.var_vgs_noswap_1_dn10) * locals.var_t0) - (assign27020_e40621 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (-((assign27020_e40621 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign27020_e40625;
        locals.var_t1_dn3 = assign27020_e40625_d_n3;
        locals.var_t1_dn4 = assign27020_e40625_d_n4;
        locals.var_t1_dn5 = assign27020_e40625_d_n5;
        locals.var_t1_dn6 = assign27020_e40625_d_n6;
        locals.var_t1_dn7 = assign27020_e40625_d_n7;
        locals.var_t1_dn8 = assign27020_e40625_d_n8;
        locals.var_t1_dn9 = assign27020_e40625_d_n9;
        locals.var_t1_dn10 = assign27020_e40625_d_n10;
        locals.var_t1_dn11 = assign27020_e40625_d_n11;

        let (assign27030_e40648, assign27030_e40648_d_n3, assign27030_e40648_d_n4, assign27030_e40648_d_n5, assign27030_e40648_d_n6, assign27030_e40648_d_n7, assign27030_e40648_d_n8, assign27030_e40648_d_n9, assign27030_e40648_d_n10, assign27030_e40648_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard557 == 0.0)) && (locals.var_guard564 == 0.0)) {
        let assign27030_e40637: f64 = (locals.var_t1 * locals.var_t1);
        let assign27030_e40640: f64 = (4.0 * 0.01);
        let assign27030_e40642: f64 = (assign27030_e40640 * 0.01);
        let assign27030_e40643: f64 = (assign27030_e40637 + assign27030_e40642);
        let assign27030_e40644: f64 = (assign27030_e40643).sqrt();
        let assign27030_e40645: f64 = (locals.var_t1 + assign27030_e40644);
        let assign27030_e40646: f64 = (0.5 * assign27030_e40645);
        (assign27030_e40646, (0.5 * (locals.var_t1_dn3 + (((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)) / (2.0 * assign27030_e40644)))), (0.5 * (locals.var_t1_dn4 + (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign27030_e40644)))), (0.5 * (locals.var_t1_dn5 + (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign27030_e40644)))), (0.5 * (locals.var_t1_dn6 + (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign27030_e40644)))), (0.5 * (locals.var_t1_dn7 + (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign27030_e40644)))), (0.5 * (locals.var_t1_dn8 + (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign27030_e40644)))), (0.5 * (locals.var_t1_dn9 + (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign27030_e40644)))), (0.5 * (locals.var_t1_dn10 + (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign27030_e40644)))), (0.5 * (locals.var_t1_dn11 + (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign27030_e40644)))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign27030_e40648;
        locals.var_t1_dn3 = assign27030_e40648_d_n3;
        locals.var_t1_dn4 = assign27030_e40648_d_n4;
        locals.var_t1_dn5 = assign27030_e40648_d_n5;
        locals.var_t1_dn6 = assign27030_e40648_d_n6;
        locals.var_t1_dn7 = assign27030_e40648_d_n7;
        locals.var_t1_dn8 = assign27030_e40648_d_n8;
        locals.var_t1_dn9 = assign27030_e40648_d_n9;
        locals.var_t1_dn10 = assign27030_e40648_d_n10;
        locals.var_t1_dn11 = assign27030_e40648_d_n11;

        let (assign27040_e40662, assign27040_e40662_d_n3, assign27040_e40662_d_n4, assign27040_e40662_d_n5, assign27040_e40662_d_n6, assign27040_e40662_d_n7, assign27040_e40662_d_n8, assign27040_e40662_d_n9, assign27040_e40662_d_n10, assign27040_e40662_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard557 == 0.0)) && (locals.var_guard564 == 0.0)) {
        let assign27040_e40659: f64 = (locals.var_t1 + 0.001);
        let assign27040_e40660: f64 = (locals.var_bgisl_t / assign27040_e40659);
        (assign27040_e40660, (-((locals.var_bgisl_t * locals.var_t1_dn3) / (assign27040_e40659 * assign27040_e40659))), (((locals.var_bgisl_t_dn4 * assign27040_e40659) - (locals.var_bgisl_t * locals.var_t1_dn4)) / (assign27040_e40659 * assign27040_e40659)), (((locals.var_bgisl_t_dn5 * assign27040_e40659) - (locals.var_bgisl_t * locals.var_t1_dn5)) / (assign27040_e40659 * assign27040_e40659)), (-((locals.var_bgisl_t * locals.var_t1_dn6) / (assign27040_e40659 * assign27040_e40659))), (-((locals.var_bgisl_t * locals.var_t1_dn7) / (assign27040_e40659 * assign27040_e40659))), (-((locals.var_bgisl_t * locals.var_t1_dn8) / (assign27040_e40659 * assign27040_e40659))), (-((locals.var_bgisl_t * locals.var_t1_dn9) / (assign27040_e40659 * assign27040_e40659))), (-((locals.var_bgisl_t * locals.var_t1_dn10) / (assign27040_e40659 * assign27040_e40659))), (-((locals.var_bgisl_t * locals.var_t1_dn11) / (assign27040_e40659 * assign27040_e40659))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign27040_e40662;
        locals.var_t2_dn3 = assign27040_e40662_d_n3;
        locals.var_t2_dn4 = assign27040_e40662_d_n4;
        locals.var_t2_dn5 = assign27040_e40662_d_n5;
        locals.var_t2_dn6 = assign27040_e40662_d_n6;
        locals.var_t2_dn7 = assign27040_e40662_d_n7;
        locals.var_t2_dn8 = assign27040_e40662_d_n8;
        locals.var_t2_dn9 = assign27040_e40662_d_n9;
        locals.var_t2_dn10 = assign27040_e40662_d_n10;
        locals.var_t2_dn11 = assign27040_e40662_d_n11;

        let assign27050_e40665: f64 = if locals.var_kgisl_i != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard565 = assign27050_e40665;

        let (assign27060_e40680, assign27060_e40680_d_n3, assign27060_e40680_d_n4, assign27060_e40680_d_n5, assign27060_e40680_d_n6, assign27060_e40680_d_n7, assign27060_e40680_d_n8, assign27060_e40680_d_n9, assign27060_e40680_d_n10, assign27060_e40680_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard557 == 0.0)) && (locals.var_guard564 == 0.0)) && (locals.var_guard565 != 0.0)) {
        let assign27060_e40676: f64 = (-locals.var_vsb_noswap);
        let assign27060_e40678: f64 = (assign27060_e40676 - locals.var_fgisl_i);
        (assign27060_e40678, 0.0, 0.0, 0.0, (-locals.var_vsb_noswap_dn6), (-locals.var_vsb_noswap_dn7), 0.0, 0.0, (-locals.var_vsb_noswap_dn10), 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign27060_e40680;
        locals.var_t3_dn3 = assign27060_e40680_d_n3;
        locals.var_t3_dn4 = assign27060_e40680_d_n4;
        locals.var_t3_dn5 = assign27060_e40680_d_n5;
        locals.var_t3_dn6 = assign27060_e40680_d_n6;
        locals.var_t3_dn7 = assign27060_e40680_d_n7;
        locals.var_t3_dn8 = assign27060_e40680_d_n8;
        locals.var_t3_dn9 = assign27060_e40680_d_n9;
        locals.var_t3_dn10 = assign27060_e40680_d_n10;
        locals.var_t3_dn11 = assign27060_e40680_d_n11;

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

        let (assign27080_e40727, assign27080_e40727_d_n3, assign27080_e40727_d_n4, assign27080_e40727_d_n5, assign27080_e40727_d_n6, assign27080_e40727_d_n7, assign27080_e40727_d_n8, assign27080_e40727_d_n9, assign27080_e40727_d_n10, assign27080_e40727_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard557 == 0.0)) && (locals.var_guard564 == 0.0)) && (locals.var_guard565 != 0.0)) {
        let __rspice_inv_cse_1: f64 = 1.0 / locals.var_t4;
        let assign27080_e40707: f64 = (locals.var_kgisl_i * __rspice_inv_cse_1);
        let assign27080_e40710: f64 = (locals.var_kgisl_i * __rspice_inv_cse_1);
        let assign27080_e40713: f64 = (locals.var_kgisl_i * __rspice_inv_cse_1);
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

        let assign27140_e40787: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard566 = assign27140_e40787;

        let assign27150_e40794: f64 = if ((locals.var_alpha0_i <= 0.0) || (locals.var_beta0_t <= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard567 = assign27150_e40794;

        let assign27170_e40806: f64 = (locals.var_beta0_t / 80.0);
        let assign27170_e40807: f64 = if locals.var_diffvds > assign27170_e40806 { 1.0 } else { 0.0 };
        locals.var_guard568 = assign27170_e40807;

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

        let assign27210_e40864: f64 = if p.p44 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard569 = assign27210_e40864;

        let assign27220_e40879: f64 = if ((locals.var_alpha0_i <= 0.0) || (((locals.var_beta2_i == 0.0) && (locals.var_beta1_i == 0.0)) && (locals.var_beta0_t == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard570 = assign27220_e40879;

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

    }

    pub(super) fn stamp_transient_block_72(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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

        let assign27380_e41237: f64 = if ((locals.var_alpha0_i <= 0.0) || (((locals.var_beta2_i == 0.0) && (locals.var_beta1_i == 0.0)) && (locals.var_beta0_t == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard571 = assign27380_e41237;

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

    }

    pub(super) fn stamp_transient_block_73(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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

        let assign27560_e41626: f64 = if locals.var_sigvds > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard572 = assign27560_e41626;

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

        let assign27600_e41670: f64 = if locals.var_t1 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard573 = assign27600_e41670;

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

        let assign27710_e41777: f64 = if p.p913 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard574 = assign27710_e41777;

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

        let assign27750_e41819: f64 = if p.p915 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard575 = assign27750_e41819;

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

        let assign27790_e41861: f64 = if p.p917 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard576 = assign27790_e41861;

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

        let assign27820_e41896: f64 = if locals.var_czbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard577 = assign27820_e41896;

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

        let assign27840_e41907: f64 = if locals.var_t1 < 0.9 { 1.0 } else { 0.0 };
        locals.var_guard578 = assign27840_e41907;

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

        let assign27860_e41920: f64 = if p.p913 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard579 = assign27860_e41920;

        let assign27870_e41923: f64 = if p.p913 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard580 = assign27870_e41923;

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

    }

    pub(super) fn stamp_transient_block_74(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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

        let assign27950_e42045: f64 = if locals.var_czbssw > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard581 = assign27950_e42045;

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

        let assign27970_e42056: f64 = if locals.var_t1 < 0.9 { 1.0 } else { 0.0 };
        locals.var_guard582 = assign27970_e42056;

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

        let assign27990_e42069: f64 = if p.p915 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard583 = assign27990_e42069;

        let assign28000_e42072: f64 = if p.p915 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard584 = assign28000_e42072;

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

        let assign28080_e42194: f64 = if locals.var_czbsswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard585 = assign28080_e42194;

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

        let assign28100_e42205: f64 = if locals.var_t1 < 0.9 { 1.0 } else { 0.0 };
        locals.var_guard586 = assign28100_e42205;

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

        let assign28120_e42218: f64 = if p.p917 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard587 = assign28120_e42218;

        let assign28130_e42221: f64 = if p.p917 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard588 = assign28130_e42221;

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

    }

    pub(super) fn stamp_transient_block_75(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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

        let assign28270_e42388: f64 = if p.p914 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard589 = assign28270_e42388;

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

        let assign28310_e42430: f64 = if p.p916 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard590 = assign28310_e42430;

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

        let assign28350_e42472: f64 = if p.p918 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard591 = assign28350_e42472;

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

        let assign28380_e42507: f64 = if locals.var_czbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard592 = assign28380_e42507;

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

        let assign28400_e42518: f64 = if locals.var_t1 < 0.9 { 1.0 } else { 0.0 };
        locals.var_guard593 = assign28400_e42518;

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

        let assign28420_e42531: f64 = if p.p914 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard594 = assign28420_e42531;

        let assign28430_e42534: f64 = if p.p914 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard595 = assign28430_e42534;

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

        let assign28510_e42656: f64 = if locals.var_czbdsw > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard596 = assign28510_e42656;

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

        let assign28530_e42667: f64 = if locals.var_t1 < 0.9 { 1.0 } else { 0.0 };
        locals.var_guard597 = assign28530_e42667;

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

        let assign28550_e42680: f64 = if p.p916 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard598 = assign28550_e42680;

        let assign28560_e42683: f64 = if p.p916 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard599 = assign28560_e42683;

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

    }

    pub(super) fn stamp_transient_block_76(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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

        let assign28640_e42805: f64 = if locals.var_czbdswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard600 = assign28640_e42805;

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

        let assign28660_e42816: f64 = if locals.var_t1 < 0.9 { 1.0 } else { 0.0 };
        locals.var_guard601 = assign28660_e42816;

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

        let assign28680_e42829: f64 = if p.p918 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard602 = assign28680_e42829;

        let assign28690_e42832: f64 = if p.p918 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard603 = assign28690_e42832;

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

        let assign28790_e42972: f64 = if locals.var_x7_s <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard604 = assign28790_e42972;

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

    }

    pub(super) fn stamp_transient_block_77(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
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

        let assign29020_e43255: f64 = if locals.var_sigvds > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard605 = assign29020_e43255;

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

        let assign29080_e43305: f64 = if (!param_given[867]) { 1.0 } else { 0.0 };
        locals.var_guard606 = assign29080_e43305;

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

        let (assign29100_e43334,) = {
    if (locals.var_guard492 != 0.0) {
        let assign29100_e43332: f64 = (p.p872 + locals.var_cf_i);
        (assign29100_e43332,)
    } else {
        (locals.var_cgsof,)
    }
};
        locals.var_cgsof = assign29100_e43334;

        let (assign29110_e43340,) = {
    if (locals.var_guard492 != 0.0) {
        let assign29110_e43338: f64 = (p.p873 + locals.var_cf_i);
        (assign29110_e43338,)
    } else {
        (locals.var_cgdof,)
    }
};
        locals.var_cgdof = assign29110_e43340;

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

        let assign29140_e43359: f64 = if p.p32 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard607 = assign29140_e43359;

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

    }

    pub(super) fn stamp_transient_block_78(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
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

        let assign29310_e43632: f64 = if locals.var_nsub_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard608 = assign29310_e43632;

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

        let (assign29370_e43695,) = {
    if (locals.var_guard492 != 0.0) {
        let assign29370_e43693: f64 = (3.453133e-11 / p.p75);
        (assign29370_e43693,)
    } else {
        (locals.var_cbox_1,)
    }
};
        locals.var_cbox_1 = assign29370_e43695;

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

        let assign29410_e43728: f64 = if p.p47 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard609 = assign29410_e43728;

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

    }

    pub(super) fn stamp_transient_block_79(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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

        let assign29440_e43756: f64 = if locals.var_t1 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard610 = assign29440_e43756;

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

        let assign29480_e43786: f64 = if locals.var_t1 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard611 = assign29480_e43786;

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

        let (assign29510_e43813,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard609 != 0.0)) {
        let assign29510_e43811: f64 = (locals.var_cbox_1 * p.p17);
        (assign29510_e43811,)
    } else {
        (locals.var_csbox,)
    }
};
        locals.var_csbox = assign29510_e43813;

        let (assign29520_e43821,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard609 != 0.0)) {
        let assign29520_e43819: f64 = (p.p1396 * p.p17);
        (assign29520_e43819,)
    } else {
        (locals.var_csmin,)
    }
};
        locals.var_csmin = assign29520_e43821;

        let (assign29530_e43829,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard609 != 0.0)) {
        let assign29530_e43827: f64 = (locals.var_cbox_1 * p.p18);
        (assign29530_e43827,)
    } else {
        (locals.var_cdbox,)
    }
};
        locals.var_cdbox = assign29530_e43829;

        let (assign29540_e43837,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard609 != 0.0)) {
        let assign29540_e43835: f64 = (p.p1396 * p.p18);
        (assign29540_e43835,)
    } else {
        (locals.var_cdmin,)
    }
};
        locals.var_cdmin = assign29540_e43837;

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

        let assign29570_e43858: f64 = if p.p1396 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard612 = assign29570_e43858;

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

        let assign29720_e44033: f64 = if p.p45 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard613 = assign29720_e44033;

        let (assign29730_e44041,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) {
        let assign29730_e44039: f64 = (p.p140 + p.p25);
        (assign29730_e44039,)
    } else {
        (locals.var_vfbagbcp2_i,)
    }
};
        locals.var_vfbagbcp2_i = assign29730_e44041;

    }
}
