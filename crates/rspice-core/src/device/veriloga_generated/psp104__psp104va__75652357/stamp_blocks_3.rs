#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_48(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign53380_e68587, assign53380_e68587_d_n5, assign53380_e68587_d_n6, assign53380_e68587_d_n7, assign53380_e68587_d_n8,) = {
    if (locals.var_guard1456 == 0.0) {
        (locals.var_thesateff_dc, locals.var_thesateff_dc_dn5, locals.var_thesateff_dc_dn6, locals.var_thesateff_dc_dn7, locals.var_thesateff_dc_dn8,)
    } else {
        (locals.var_thesateff_ac, locals.var_thesateff_ac_dn5, locals.var_thesateff_ac_dn6, locals.var_thesateff_ac_dn7, locals.var_thesateff_ac_dn8,)
    }
};
        locals.var_thesateff_ac = assign53380_e68587;
        locals.var_thesateff_ac_dn5 = assign53380_e68587_d_n5;
        locals.var_thesateff_ac_dn6 = assign53380_e68587_d_n6;
        locals.var_thesateff_ac_dn7 = assign53380_e68587_d_n7;
        locals.var_thesateff_ac_dn8 = assign53380_e68587_d_n8;

        let (assign53390_e68592, assign53390_e68592_d_n5, assign53390_e68592_d_n6, assign53390_e68592_d_n7, assign53390_e68592_d_n8,) = {
    if (locals.var_guard1456 == 0.0) {
        (locals.var_voxm_dc, locals.var_voxm_dc_dn5, locals.var_voxm_dc_dn6, locals.var_voxm_dc_dn7, locals.var_voxm_dc_dn8,)
    } else {
        (locals.var_voxm_ac, locals.var_voxm_ac_dn5, locals.var_voxm_ac_dn6, locals.var_voxm_ac_dn7, locals.var_voxm_ac_dn8,)
    }
};
        locals.var_voxm_ac = assign53390_e68592;
        locals.var_voxm_ac_dn5 = assign53390_e68592_d_n5;
        locals.var_voxm_ac_dn6 = assign53390_e68592_d_n6;
        locals.var_voxm_ac_dn7 = assign53390_e68592_d_n7;
        locals.var_voxm_ac_dn8 = assign53390_e68592_d_n8;

        locals.var_cox_qm = locals.var_cox_i;
        locals.var_cox_qm_dn5 = 0.0;
        locals.var_cox_qm_dn6 = 0.0;
        locals.var_cox_qm_dn7 = 0.0;
        locals.var_cox_qm_dn8 = 0.0;

        let assign53420_e68601: f64 = if locals.var_qq > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1505 = assign53420_e68601;

        let (assign53430_e68620, assign53430_e68620_d_n5, assign53430_e68620_d_n6, assign53430_e68620_d_n7, assign53430_e68620_d_n8,) = {
    if (locals.var_guard1505 != 0.0) {
        let assign53430_e68608: f64 = (locals.var_qeff1_ac * locals.var_qeff1_ac);
        let assign53430_e68610: f64 = (assign53430_e68608 + locals.var_qlim2);
        let assign53430_e68612: f64 = (-1.0);
        let assign53430_e68614: f64 = (assign53430_e68612 * 0.16666666666666666);
        let assign53430_e68615: f64 = (assign53430_e68610).powf(assign53430_e68614);
        let assign53430_e68616: f64 = (locals.var_qq * assign53430_e68615);
        let assign53430_e68617: f64 = (1.0 + assign53430_e68616);
        let assign53430_e68618: f64 = (locals.var_cox_i / assign53430_e68617);
        (assign53430_e68618, (-((locals.var_cox_i * (locals.var_qq * if 0.0 == 0.0 && ((assign53430_e68614) as f64).is_finite() && ((assign53430_e68614) as f64).fract() == 0.0 { if assign53430_e68614 == 0.0 { 0.0 } else { (assign53430_e68614 * ((assign53430_e68610).powf(assign53430_e68614 - 1.0) * ((locals.var_qeff1_ac_dn5 * locals.var_qeff1_ac) + (locals.var_qeff1_ac * locals.var_qeff1_ac_dn5)))) } } else { (assign53430_e68615 * (assign53430_e68614 * (((locals.var_qeff1_ac_dn5 * locals.var_qeff1_ac) + (locals.var_qeff1_ac * locals.var_qeff1_ac_dn5)) / assign53430_e68610))) })) / (assign53430_e68617 * assign53430_e68617))), (-((locals.var_cox_i * (locals.var_qq * if 0.0 == 0.0 && ((assign53430_e68614) as f64).is_finite() && ((assign53430_e68614) as f64).fract() == 0.0 { if assign53430_e68614 == 0.0 { 0.0 } else { (assign53430_e68614 * ((assign53430_e68610).powf(assign53430_e68614 - 1.0) * ((locals.var_qeff1_ac_dn6 * locals.var_qeff1_ac) + (locals.var_qeff1_ac * locals.var_qeff1_ac_dn6)))) } } else { (assign53430_e68615 * (assign53430_e68614 * (((locals.var_qeff1_ac_dn6 * locals.var_qeff1_ac) + (locals.var_qeff1_ac * locals.var_qeff1_ac_dn6)) / assign53430_e68610))) })) / (assign53430_e68617 * assign53430_e68617))), (-((locals.var_cox_i * (locals.var_qq * if 0.0 == 0.0 && ((assign53430_e68614) as f64).is_finite() && ((assign53430_e68614) as f64).fract() == 0.0 { if assign53430_e68614 == 0.0 { 0.0 } else { (assign53430_e68614 * ((assign53430_e68610).powf(assign53430_e68614 - 1.0) * ((locals.var_qeff1_ac_dn7 * locals.var_qeff1_ac) + (locals.var_qeff1_ac * locals.var_qeff1_ac_dn7)))) } } else { (assign53430_e68615 * (assign53430_e68614 * (((locals.var_qeff1_ac_dn7 * locals.var_qeff1_ac) + (locals.var_qeff1_ac * locals.var_qeff1_ac_dn7)) / assign53430_e68610))) })) / (assign53430_e68617 * assign53430_e68617))), (-((locals.var_cox_i * (locals.var_qq * if 0.0 == 0.0 && ((assign53430_e68614) as f64).is_finite() && ((assign53430_e68614) as f64).fract() == 0.0 { if assign53430_e68614 == 0.0 { 0.0 } else { (assign53430_e68614 * ((assign53430_e68610).powf(assign53430_e68614 - 1.0) * ((locals.var_qeff1_ac_dn8 * locals.var_qeff1_ac) + (locals.var_qeff1_ac * locals.var_qeff1_ac_dn8)))) } } else { (assign53430_e68615 * (assign53430_e68614 * (((locals.var_qeff1_ac_dn8 * locals.var_qeff1_ac) + (locals.var_qeff1_ac * locals.var_qeff1_ac_dn8)) / assign53430_e68610))) })) / (assign53430_e68617 * assign53430_e68617))),)
    } else {
        (locals.var_cox_qm, locals.var_cox_qm_dn5, locals.var_cox_qm_dn6, locals.var_cox_qm_dn7, locals.var_cox_qm_dn8,)
    }
};
        locals.var_cox_qm = assign53430_e68620;
        locals.var_cox_qm_dn5 = assign53430_e68620_d_n5;
        locals.var_cox_qm_dn6 = assign53430_e68620_d_n6;
        locals.var_cox_qm_dn7 = assign53430_e68620_d_n7;
        locals.var_cox_qm_dn8 = assign53430_e68620_d_n8;

        locals.var_gdl_ac = 1.0;
        locals.var_gdl_ac_dn5 = 0.0;
        locals.var_gdl_ac_dn6 = 0.0;
        locals.var_gdl_ac_dn7 = 0.0;
        locals.var_gdl_ac_dn8 = 0.0;

        locals.var_gmob_dl_ac = 1.0;
        locals.var_gmob_dl_ac_dn5 = 0.0;
        locals.var_gmob_dl_ac_dn6 = 0.0;
        locals.var_gmob_dl_ac_dn7 = 0.0;
        locals.var_gmob_dl_ac_dn8 = 0.0;

        locals.var_thesat1_ac = 0.0;
        locals.var_thesat1_ac_dn5 = 0.0;
        locals.var_thesat1_ac_dn6 = 0.0;
        locals.var_thesat1_ac_dn7 = 0.0;
        locals.var_thesat1_ac_dn8 = 0.0;

        locals.var_gvsat_ac = 1.0;
        locals.var_gvsat_ac_dn5 = 0.0;
        locals.var_gvsat_ac_dn6 = 0.0;
        locals.var_gvsat_ac_dn7 = 0.0;
        locals.var_gvsat_ac_dn8 = 0.0;

        locals.var_h_ac = 1.0;
        locals.var_h_ac_dn5 = 0.0;
        locals.var_h_ac_dn6 = 0.0;
        locals.var_h_ac_dn7 = 0.0;
        locals.var_h_ac_dn8 = 0.0;

        locals.var_qg_1 = locals.var_voxm_ac;
        locals.var_qg_1_dn5 = locals.var_voxm_ac_dn5;
        locals.var_qg_1_dn6 = locals.var_voxm_ac_dn6;
        locals.var_qg_1_dn7 = locals.var_voxm_ac_dn7;
        locals.var_qg_1_dn8 = locals.var_voxm_ac_dn8;

        locals.var_qi = 0.0;
        locals.var_qi_dn5 = 0.0;
        locals.var_qi_dn6 = 0.0;
        locals.var_qi_dn7 = 0.0;
        locals.var_qi_dn8 = 0.0;

        locals.var_qd_1 = 0.0;
        locals.var_qd_1_dn5 = 0.0;
        locals.var_qd_1_dn6 = 0.0;
        locals.var_qd_1_dn7 = 0.0;
        locals.var_qd_1_dn8 = 0.0;

        locals.var_qb_1 = locals.var_qg_1;
        locals.var_qb_1_dn5 = locals.var_qg_1_dn5;
        locals.var_qb_1_dn6 = locals.var_qg_1_dn6;
        locals.var_qb_1_dn7 = locals.var_qg_1_dn7;
        locals.var_qb_1_dn8 = locals.var_qg_1_dn8;

        let assign53530_e68632: f64 = if locals.var_xg_ac > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1506 = assign53530_e68632;

        let (assign53540_e68646, assign53540_e68646_d_n5, assign53540_e68646_d_n6, assign53540_e68646_d_n7, assign53540_e68646_d_n8,) = {
    if (locals.var_guard1506 != 0.0) {
        let assign53540_e68637: f64 = (locals.var_alp1ac_i / locals.var_qim1_ac);
        let assign53540_e68638: f64 = (locals.var_alpac_i + assign53540_e68637);
        let assign53540_e68640: f64 = (assign53540_e68638 * locals.var_qim_ac);
        let assign53540_e68642: f64 = (assign53540_e68640 / locals.var_qim1_ac);
        let assign53540_e68644: f64 = (assign53540_e68642 * locals.var_s1_ac);
        (assign53540_e68644, ((((((((-((locals.var_alp1ac_i * locals.var_qim1_ac_dn5) / (locals.var_qim1_ac * locals.var_qim1_ac))) * locals.var_qim_ac) + (assign53540_e68638 * locals.var_qim_ac_dn5)) * locals.var_qim1_ac) - (assign53540_e68640 * locals.var_qim1_ac_dn5)) / (locals.var_qim1_ac * locals.var_qim1_ac)) * locals.var_s1_ac) + (assign53540_e68642 * locals.var_s1_ac_dn5)), ((((((((-((locals.var_alp1ac_i * locals.var_qim1_ac_dn6) / (locals.var_qim1_ac * locals.var_qim1_ac))) * locals.var_qim_ac) + (assign53540_e68638 * locals.var_qim_ac_dn6)) * locals.var_qim1_ac) - (assign53540_e68640 * locals.var_qim1_ac_dn6)) / (locals.var_qim1_ac * locals.var_qim1_ac)) * locals.var_s1_ac) + (assign53540_e68642 * locals.var_s1_ac_dn6)), ((((((((-((locals.var_alp1ac_i * locals.var_qim1_ac_dn7) / (locals.var_qim1_ac * locals.var_qim1_ac))) * locals.var_qim_ac) + (assign53540_e68638 * locals.var_qim_ac_dn7)) * locals.var_qim1_ac) - (assign53540_e68640 * locals.var_qim1_ac_dn7)) / (locals.var_qim1_ac * locals.var_qim1_ac)) * locals.var_s1_ac) + (assign53540_e68642 * locals.var_s1_ac_dn7)), ((((((((-((locals.var_alp1ac_i * locals.var_qim1_ac_dn8) / (locals.var_qim1_ac * locals.var_qim1_ac))) * locals.var_qim_ac) + (assign53540_e68638 * locals.var_qim_ac_dn8)) * locals.var_qim1_ac) - (assign53540_e68640 * locals.var_qim1_ac_dn8)) / (locals.var_qim1_ac * locals.var_qim1_ac)) * locals.var_s1_ac) + (assign53540_e68642 * locals.var_s1_ac_dn8)),)
    } else {
        (locals.var_dl__blk1263, locals.var_dl__blk1263_dn5, locals.var_dl__blk1263_dn6, locals.var_dl__blk1263_dn7, locals.var_dl__blk1263_dn8,)
    }
};
        locals.var_dl__blk1263 = assign53540_e68646;
        locals.var_dl__blk1263_dn5 = assign53540_e68646_d_n5;
        locals.var_dl__blk1263_dn6 = assign53540_e68646_d_n6;
        locals.var_dl__blk1263_dn7 = assign53540_e68646_d_n7;
        locals.var_dl__blk1263_dn8 = assign53540_e68646_d_n8;

        let assign53550_e68649: f64 = if locals.var_dl__blk1263 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1507 = assign53550_e68649;

        let (assign53560_e68663, assign53560_e68663_d_n5, assign53560_e68663_d_n6, assign53560_e68663_d_n7, assign53560_e68663_d_n8,) = {
    if ((locals.var_guard1506 != 0.0) && (locals.var_guard1507 != 0.0)) {
        let assign53560_e68656: f64 = (1.0 + locals.var_dl__blk1263);
        let assign53560_e68659: f64 = (locals.var_dl__blk1263 * locals.var_dl__blk1263);
        let assign53560_e68660: f64 = (assign53560_e68656 + assign53560_e68659);
        let assign53560_e68661: f64 = (1.0 / assign53560_e68660);
        (assign53560_e68661, (-((locals.var_dl__blk1263_dn5 + ((locals.var_dl__blk1263_dn5 * locals.var_dl__blk1263) + (locals.var_dl__blk1263 * locals.var_dl__blk1263_dn5))) / (assign53560_e68660 * assign53560_e68660))), (-((locals.var_dl__blk1263_dn6 + ((locals.var_dl__blk1263_dn6 * locals.var_dl__blk1263) + (locals.var_dl__blk1263 * locals.var_dl__blk1263_dn6))) / (assign53560_e68660 * assign53560_e68660))), (-((locals.var_dl__blk1263_dn7 + ((locals.var_dl__blk1263_dn7 * locals.var_dl__blk1263) + (locals.var_dl__blk1263 * locals.var_dl__blk1263_dn7))) / (assign53560_e68660 * assign53560_e68660))), (-((locals.var_dl__blk1263_dn8 + ((locals.var_dl__blk1263_dn8 * locals.var_dl__blk1263) + (locals.var_dl__blk1263 * locals.var_dl__blk1263_dn8))) / (assign53560_e68660 * assign53560_e68660))),)
    } else {
        (locals.var_gdl_ac, locals.var_gdl_ac_dn5, locals.var_gdl_ac_dn6, locals.var_gdl_ac_dn7, locals.var_gdl_ac_dn8,)
    }
};
        locals.var_gdl_ac = assign53560_e68663;
        locals.var_gdl_ac_dn5 = assign53560_e68663_d_n5;
        locals.var_gdl_ac_dn6 = assign53560_e68663_d_n6;
        locals.var_gdl_ac_dn7 = assign53560_e68663_d_n7;
        locals.var_gdl_ac_dn8 = assign53560_e68663_d_n8;

        let (assign53570_e68672, assign53570_e68672_d_n5, assign53570_e68672_d_n6, assign53570_e68672_d_n7, assign53570_e68672_d_n8,) = {
    if ((locals.var_guard1506 != 0.0) && (locals.var_guard1507 == 0.0)) {
        let assign53570_e68670: f64 = (1.0 - locals.var_dl__blk1263);
        (assign53570_e68670, (-locals.var_dl__blk1263_dn5), (-locals.var_dl__blk1263_dn6), (-locals.var_dl__blk1263_dn7), (-locals.var_dl__blk1263_dn8),)
    } else {
        (locals.var_gdl_ac, locals.var_gdl_ac_dn5, locals.var_gdl_ac_dn6, locals.var_gdl_ac_dn7, locals.var_gdl_ac_dn8,)
    }
};
        locals.var_gdl_ac = assign53570_e68672;
        locals.var_gdl_ac_dn5 = assign53570_e68672_d_n5;
        locals.var_gdl_ac_dn6 = assign53570_e68672_d_n6;
        locals.var_gdl_ac_dn7 = assign53570_e68672_d_n7;
        locals.var_gdl_ac_dn8 = assign53570_e68672_d_n8;

        let (assign53580_e68678, assign53580_e68678_d_n5, assign53580_e68678_d_n6, assign53580_e68678_d_n7, assign53580_e68678_d_n8,) = {
    if (locals.var_guard1506 != 0.0) {
        let assign53580_e68676: f64 = (locals.var_gmob_ac * locals.var_gdl_ac);
        (assign53580_e68676, ((locals.var_gmob_ac_dn5 * locals.var_gdl_ac) + (locals.var_gmob_ac * locals.var_gdl_ac_dn5)), ((locals.var_gmob_ac_dn6 * locals.var_gdl_ac) + (locals.var_gmob_ac * locals.var_gdl_ac_dn6)), ((locals.var_gmob_ac_dn7 * locals.var_gdl_ac) + (locals.var_gmob_ac * locals.var_gdl_ac_dn7)), ((locals.var_gmob_ac_dn8 * locals.var_gdl_ac) + (locals.var_gmob_ac * locals.var_gdl_ac_dn8)),)
    } else {
        (locals.var_gmob_dl_ac, locals.var_gmob_dl_ac_dn5, locals.var_gmob_dl_ac_dn6, locals.var_gmob_dl_ac_dn7, locals.var_gmob_dl_ac_dn8,)
    }
};
        locals.var_gmob_dl_ac = assign53580_e68678;
        locals.var_gmob_dl_ac_dn5 = assign53580_e68678_d_n5;
        locals.var_gmob_dl_ac_dn6 = assign53580_e68678_d_n6;
        locals.var_gmob_dl_ac_dn7 = assign53580_e68678_d_n7;
        locals.var_gmob_dl_ac_dn8 = assign53580_e68678_d_n8;

        let (assign53590_e68684, assign53590_e68684_d_n5, assign53590_e68684_d_n6, assign53590_e68684_d_n7, assign53590_e68684_d_n8,) = {
    if (locals.var_guard1506 != 0.0) {
        let assign53590_e68682: f64 = (locals.var_thesateff_ac / locals.var_gmob_dl_ac);
        (assign53590_e68682, (((locals.var_thesateff_ac_dn5 * locals.var_gmob_dl_ac) - (locals.var_thesateff_ac * locals.var_gmob_dl_ac_dn5)) / (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac)), (((locals.var_thesateff_ac_dn6 * locals.var_gmob_dl_ac) - (locals.var_thesateff_ac * locals.var_gmob_dl_ac_dn6)) / (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac)), (((locals.var_thesateff_ac_dn7 * locals.var_gmob_dl_ac) - (locals.var_thesateff_ac * locals.var_gmob_dl_ac_dn7)) / (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac)), (((locals.var_thesateff_ac_dn8 * locals.var_gmob_dl_ac) - (locals.var_thesateff_ac * locals.var_gmob_dl_ac_dn8)) / (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac)),)
    } else {
        (locals.var_thesat1_ac, locals.var_thesat1_ac_dn5, locals.var_thesat1_ac_dn6, locals.var_thesat1_ac_dn7, locals.var_thesat1_ac_dn8,)
    }
};
        locals.var_thesat1_ac = assign53590_e68684;
        locals.var_thesat1_ac_dn5 = assign53590_e68684_d_n5;
        locals.var_thesat1_ac_dn6 = assign53590_e68684_d_n6;
        locals.var_thesat1_ac_dn7 = assign53590_e68684_d_n7;
        locals.var_thesat1_ac_dn8 = assign53590_e68684_d_n8;

        let (assign53600_e68694, assign53600_e68694_d_n5, assign53600_e68694_d_n6, assign53600_e68694_d_n7, assign53600_e68694_d_n8,) = {
    if (locals.var_guard1506 != 0.0) {
        let assign53600_e68688: f64 = (locals.var_thesat1_ac * locals.var_thesat1_ac);
        let assign53600_e68690: f64 = (assign53600_e68688 * locals.var_dps_ac);
        let assign53600_e68692: f64 = (assign53600_e68690 * locals.var_dps_ac);
        (assign53600_e68692, ((((((locals.var_thesat1_ac_dn5 * locals.var_thesat1_ac) + (locals.var_thesat1_ac * locals.var_thesat1_ac_dn5)) * locals.var_dps_ac) + (assign53600_e68688 * locals.var_dps_ac_dn5)) * locals.var_dps_ac) + (assign53600_e68690 * locals.var_dps_ac_dn5)), ((((((locals.var_thesat1_ac_dn6 * locals.var_thesat1_ac) + (locals.var_thesat1_ac * locals.var_thesat1_ac_dn6)) * locals.var_dps_ac) + (assign53600_e68688 * locals.var_dps_ac_dn6)) * locals.var_dps_ac) + (assign53600_e68690 * locals.var_dps_ac_dn6)), ((((((locals.var_thesat1_ac_dn7 * locals.var_thesat1_ac) + (locals.var_thesat1_ac * locals.var_thesat1_ac_dn7)) * locals.var_dps_ac) + (assign53600_e68688 * locals.var_dps_ac_dn7)) * locals.var_dps_ac) + (assign53600_e68690 * locals.var_dps_ac_dn7)), ((((((locals.var_thesat1_ac_dn8 * locals.var_thesat1_ac) + (locals.var_thesat1_ac * locals.var_thesat1_ac_dn8)) * locals.var_dps_ac) + (assign53600_e68688 * locals.var_dps_ac_dn8)) * locals.var_dps_ac) + (assign53600_e68690 * locals.var_dps_ac_dn8)),)
    } else {
        (locals.var_zsat__blk1264, locals.var_zsat__blk1264_dn5, locals.var_zsat__blk1264_dn6, locals.var_zsat__blk1264_dn7, locals.var_zsat__blk1264_dn8,)
    }
};
        locals.var_zsat__blk1264 = assign53600_e68694;
        locals.var_zsat__blk1264_dn5 = assign53600_e68694_d_n5;
        locals.var_zsat__blk1264_dn6 = assign53600_e68694_d_n6;
        locals.var_zsat__blk1264_dn7 = assign53600_e68694_d_n7;
        locals.var_zsat__blk1264_dn8 = assign53600_e68694_d_n8;

        let assign53610_e68697: f64 = (-1.0);
        let assign53610_e68698: f64 = if locals.var_chnl_type == assign53610_e68697 { 1.0 } else { 0.0 };
        locals.var_guard1508 = assign53610_e68698;

        let (assign53620_e68710, assign53620_e68710_d_n5, assign53620_e68710_d_n6, assign53620_e68710_d_n7, assign53620_e68710_d_n8,) = {
    if ((locals.var_guard1506 != 0.0) && (locals.var_guard1508 != 0.0)) {
        let assign53620_e68706: f64 = (locals.var_thesat1_ac * locals.var_dps_ac);
        let assign53620_e68707: f64 = (1.0 + assign53620_e68706);
        let assign53620_e68708: f64 = (locals.var_zsat__blk1264 / assign53620_e68707);
        (assign53620_e68708, (((locals.var_zsat__blk1264_dn5 * assign53620_e68707) - (locals.var_zsat__blk1264 * ((locals.var_thesat1_ac_dn5 * locals.var_dps_ac) + (locals.var_thesat1_ac * locals.var_dps_ac_dn5)))) / (assign53620_e68707 * assign53620_e68707)), (((locals.var_zsat__blk1264_dn6 * assign53620_e68707) - (locals.var_zsat__blk1264 * ((locals.var_thesat1_ac_dn6 * locals.var_dps_ac) + (locals.var_thesat1_ac * locals.var_dps_ac_dn6)))) / (assign53620_e68707 * assign53620_e68707)), (((locals.var_zsat__blk1264_dn7 * assign53620_e68707) - (locals.var_zsat__blk1264 * ((locals.var_thesat1_ac_dn7 * locals.var_dps_ac) + (locals.var_thesat1_ac * locals.var_dps_ac_dn7)))) / (assign53620_e68707 * assign53620_e68707)), (((locals.var_zsat__blk1264_dn8 * assign53620_e68707) - (locals.var_zsat__blk1264 * ((locals.var_thesat1_ac_dn8 * locals.var_dps_ac) + (locals.var_thesat1_ac * locals.var_dps_ac_dn8)))) / (assign53620_e68707 * assign53620_e68707)),)
    } else {
        (locals.var_zsat__blk1264, locals.var_zsat__blk1264_dn5, locals.var_zsat__blk1264_dn6, locals.var_zsat__blk1264_dn7, locals.var_zsat__blk1264_dn8,)
    }
};
        locals.var_zsat__blk1264 = assign53620_e68710;
        locals.var_zsat__blk1264_dn5 = assign53620_e68710_d_n5;
        locals.var_zsat__blk1264_dn6 = assign53620_e68710_d_n6;
        locals.var_zsat__blk1264_dn7 = assign53620_e68710_d_n7;
        locals.var_zsat__blk1264_dn8 = assign53620_e68710_d_n8;

        let (assign53630_e68725, assign53630_e68725_d_n5, assign53630_e68725_d_n6, assign53630_e68725_d_n7, assign53630_e68725_d_n8,) = {
    if (locals.var_guard1506 != 0.0) {
        let assign53630_e68718: f64 = (2.0 * locals.var_zsat__blk1264);
        let assign53630_e68719: f64 = (1.0 + assign53630_e68718);
        let assign53630_e68720: f64 = (assign53630_e68719).sqrt();
        let assign53630_e68721: f64 = (1.0 + assign53630_e68720);
        let assign53630_e68722: f64 = (locals.var_gmob_dl_ac * assign53630_e68721);
        let assign53630_e68723: f64 = (0.5 * assign53630_e68722);
        (assign53630_e68723, (0.5 * ((locals.var_gmob_dl_ac_dn5 * assign53630_e68721) + (locals.var_gmob_dl_ac * ((2.0 * locals.var_zsat__blk1264_dn5) / (2.0 * assign53630_e68720))))), (0.5 * ((locals.var_gmob_dl_ac_dn6 * assign53630_e68721) + (locals.var_gmob_dl_ac * ((2.0 * locals.var_zsat__blk1264_dn6) / (2.0 * assign53630_e68720))))), (0.5 * ((locals.var_gmob_dl_ac_dn7 * assign53630_e68721) + (locals.var_gmob_dl_ac * ((2.0 * locals.var_zsat__blk1264_dn7) / (2.0 * assign53630_e68720))))), (0.5 * ((locals.var_gmob_dl_ac_dn8 * assign53630_e68721) + (locals.var_gmob_dl_ac * ((2.0 * locals.var_zsat__blk1264_dn8) / (2.0 * assign53630_e68720))))),)
    } else {
        (locals.var_gvsat_ac, locals.var_gvsat_ac_dn5, locals.var_gvsat_ac_dn6, locals.var_gvsat_ac_dn7, locals.var_gvsat_ac_dn8,)
    }
};
        locals.var_gvsat_ac = assign53630_e68725;
        locals.var_gvsat_ac_dn5 = assign53630_e68725_d_n5;
        locals.var_gvsat_ac_dn6 = assign53630_e68725_d_n6;
        locals.var_gvsat_ac_dn7 = assign53630_e68725_d_n7;
        locals.var_gvsat_ac_dn8 = assign53630_e68725_d_n8;

        let (assign53640_e68731, assign53640_e68731_d_n5, assign53640_e68731_d_n6, assign53640_e68731_d_n7, assign53640_e68731_d_n8,) = {
    if (locals.var_guard1506 != 0.0) {
        let assign53640_e68729: f64 = (locals.var_gmob_dl_ac / locals.var_gvsat_ac);
        (assign53640_e68729, (((locals.var_gmob_dl_ac_dn5 * locals.var_gvsat_ac) - (locals.var_gmob_dl_ac * locals.var_gvsat_ac_dn5)) / (locals.var_gvsat_ac * locals.var_gvsat_ac)), (((locals.var_gmob_dl_ac_dn6 * locals.var_gvsat_ac) - (locals.var_gmob_dl_ac * locals.var_gvsat_ac_dn6)) / (locals.var_gvsat_ac * locals.var_gvsat_ac)), (((locals.var_gmob_dl_ac_dn7 * locals.var_gvsat_ac) - (locals.var_gmob_dl_ac * locals.var_gvsat_ac_dn7)) / (locals.var_gvsat_ac * locals.var_gvsat_ac)), (((locals.var_gmob_dl_ac_dn8 * locals.var_gvsat_ac) - (locals.var_gmob_dl_ac * locals.var_gvsat_ac_dn8)) / (locals.var_gvsat_ac * locals.var_gvsat_ac)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign53640_e68731;
        locals.var_temp__blk936_dn5 = assign53640_e68731_d_n5;
        locals.var_temp__blk936_dn6 = assign53640_e68731_d_n6;
        locals.var_temp__blk936_dn7 = assign53640_e68731_d_n7;
        locals.var_temp__blk936_dn8 = assign53640_e68731_d_n8;

        let (assign53650_e68745, assign53650_e68745_d_n5, assign53650_e68745_d_n6, assign53650_e68745_d_n7, assign53650_e68745_d_n8,) = {
    if (locals.var_guard1506 != 0.0) {
        let assign53650_e68738: f64 = (locals.var_zsat__blk1264 * locals.var_temp__blk936);
        let assign53650_e68740: f64 = (assign53650_e68738 * locals.var_temp__blk936);
        let assign53650_e68741: f64 = (0.5 * assign53650_e68740);
        let assign53650_e68742: f64 = (1.0 + assign53650_e68741);
        let assign53650_e68743: f64 = (locals.var_alpha_ac * assign53650_e68742);
        (assign53650_e68743, ((locals.var_alpha_ac_dn5 * assign53650_e68742) + (locals.var_alpha_ac * (0.5 * ((((locals.var_zsat__blk1264_dn5 * locals.var_temp__blk936) + (locals.var_zsat__blk1264 * locals.var_temp__blk936_dn5)) * locals.var_temp__blk936) + (assign53650_e68738 * locals.var_temp__blk936_dn5))))), ((locals.var_alpha_ac_dn6 * assign53650_e68742) + (locals.var_alpha_ac * (0.5 * ((((locals.var_zsat__blk1264_dn6 * locals.var_temp__blk936) + (locals.var_zsat__blk1264 * locals.var_temp__blk936_dn6)) * locals.var_temp__blk936) + (assign53650_e68738 * locals.var_temp__blk936_dn6))))), ((locals.var_alpha_ac_dn7 * assign53650_e68742) + (locals.var_alpha_ac * (0.5 * ((((locals.var_zsat__blk1264_dn7 * locals.var_temp__blk936) + (locals.var_zsat__blk1264 * locals.var_temp__blk936_dn7)) * locals.var_temp__blk936) + (assign53650_e68738 * locals.var_temp__blk936_dn7))))), ((locals.var_alpha_ac_dn8 * assign53650_e68742) + (locals.var_alpha_ac * (0.5 * ((((locals.var_zsat__blk1264_dn8 * locals.var_temp__blk936) + (locals.var_zsat__blk1264 * locals.var_temp__blk936_dn8)) * locals.var_temp__blk936) + (assign53650_e68738 * locals.var_temp__blk936_dn8))))),)
    } else {
        (locals.var_alpha1__blk1265, locals.var_alpha1__blk1265_dn5, locals.var_alpha1__blk1265_dn6, locals.var_alpha1__blk1265_dn7, locals.var_alpha1__blk1265_dn8,)
    }
};
        locals.var_alpha1__blk1265 = assign53650_e68745;
        locals.var_alpha1__blk1265_dn5 = assign53650_e68745_d_n5;
        locals.var_alpha1__blk1265_dn6 = assign53650_e68745_d_n6;
        locals.var_alpha1__blk1265_dn7 = assign53650_e68745_d_n7;
        locals.var_alpha1__blk1265_dn8 = assign53650_e68745_d_n8;

        let (assign53660_e68753, assign53660_e68753_d_n5, assign53660_e68753_d_n6, assign53660_e68753_d_n7, assign53660_e68753_d_n8,) = {
    if (locals.var_guard1506 != 0.0) {
        let assign53660_e68749: f64 = (locals.var_temp__blk936 * locals.var_qim1_ac);
        let assign53660_e68751: f64 = (assign53660_e68749 / locals.var_alpha1__blk1265);
        (assign53660_e68751, (((((locals.var_temp__blk936_dn5 * locals.var_qim1_ac) + (locals.var_temp__blk936 * locals.var_qim1_ac_dn5)) * locals.var_alpha1__blk1265) - (assign53660_e68749 * locals.var_alpha1__blk1265_dn5)) / (locals.var_alpha1__blk1265 * locals.var_alpha1__blk1265)), (((((locals.var_temp__blk936_dn6 * locals.var_qim1_ac) + (locals.var_temp__blk936 * locals.var_qim1_ac_dn6)) * locals.var_alpha1__blk1265) - (assign53660_e68749 * locals.var_alpha1__blk1265_dn6)) / (locals.var_alpha1__blk1265 * locals.var_alpha1__blk1265)), (((((locals.var_temp__blk936_dn7 * locals.var_qim1_ac) + (locals.var_temp__blk936 * locals.var_qim1_ac_dn7)) * locals.var_alpha1__blk1265) - (assign53660_e68749 * locals.var_alpha1__blk1265_dn7)) / (locals.var_alpha1__blk1265 * locals.var_alpha1__blk1265)), (((((locals.var_temp__blk936_dn8 * locals.var_qim1_ac) + (locals.var_temp__blk936 * locals.var_qim1_ac_dn8)) * locals.var_alpha1__blk1265) - (assign53660_e68749 * locals.var_alpha1__blk1265_dn8)) / (locals.var_alpha1__blk1265 * locals.var_alpha1__blk1265)),)
    } else {
        (locals.var_h_ac, locals.var_h_ac_dn5, locals.var_h_ac_dn6, locals.var_h_ac_dn7, locals.var_h_ac_dn8,)
    }
};
        locals.var_h_ac = assign53660_e68753;
        locals.var_h_ac_dn5 = assign53660_e68753_d_n5;
        locals.var_h_ac_dn6 = assign53660_e68753_d_n6;
        locals.var_h_ac_dn7 = assign53660_e68753_d_n7;
        locals.var_h_ac_dn8 = assign53660_e68753_d_n8;

        let (assign53670_e68761, assign53670_e68761_d_n5, assign53670_e68761_d_n6, assign53670_e68761_d_n7, assign53670_e68761_d_n8,) = {
    if (locals.var_guard1506 != 0.0) {
        let assign53670_e68758: f64 = (locals.var_dps_ac / locals.var_h_ac);
        let assign53670_e68759: f64 = (0.5 * assign53670_e68758);
        (assign53670_e68759, (0.5 * (((locals.var_dps_ac_dn5 * locals.var_h_ac) - (locals.var_dps_ac * locals.var_h_ac_dn5)) / (locals.var_h_ac * locals.var_h_ac))), (0.5 * (((locals.var_dps_ac_dn6 * locals.var_h_ac) - (locals.var_dps_ac * locals.var_h_ac_dn6)) / (locals.var_h_ac * locals.var_h_ac))), (0.5 * (((locals.var_dps_ac_dn7 * locals.var_h_ac) - (locals.var_dps_ac * locals.var_h_ac_dn7)) / (locals.var_h_ac * locals.var_h_ac))), (0.5 * (((locals.var_dps_ac_dn8 * locals.var_h_ac) - (locals.var_dps_ac * locals.var_h_ac_dn8)) / (locals.var_h_ac * locals.var_h_ac))),)
    } else {
        (locals.var_fj, locals.var_fj_dn5, locals.var_fj_dn6, locals.var_fj_dn7, locals.var_fj_dn8,)
    }
};
        locals.var_fj = assign53670_e68761;
        locals.var_fj_dn5 = assign53670_e68761_d_n5;
        locals.var_fj_dn6 = assign53670_e68761_d_n6;
        locals.var_fj_dn7 = assign53670_e68761_d_n7;
        locals.var_fj_dn8 = assign53670_e68761_d_n8;

        let (assign53680_e68767, assign53680_e68767_d_n5, assign53680_e68767_d_n6, assign53680_e68767_d_n7, assign53680_e68767_d_n8,) = {
    if (locals.var_guard1506 != 0.0) {
        let assign53680_e68765: f64 = (locals.var_fj * locals.var_fj);
        (assign53680_e68765, ((locals.var_fj_dn5 * locals.var_fj) + (locals.var_fj * locals.var_fj_dn5)), ((locals.var_fj_dn6 * locals.var_fj) + (locals.var_fj * locals.var_fj_dn6)), ((locals.var_fj_dn7 * locals.var_fj) + (locals.var_fj * locals.var_fj_dn7)), ((locals.var_fj_dn8 * locals.var_fj) + (locals.var_fj * locals.var_fj_dn8)),)
    } else {
        (locals.var_fj2, locals.var_fj2_dn5, locals.var_fj2_dn6, locals.var_fj2_dn7, locals.var_fj2_dn8,)
    }
};
        locals.var_fj2 = assign53680_e68767;
        locals.var_fj2_dn5 = assign53680_e68767_d_n5;
        locals.var_fj2_dn6 = assign53680_e68767_d_n6;
        locals.var_fj2_dn7 = assign53680_e68767_d_n7;
        locals.var_fj2_dn8 = assign53680_e68767_d_n8;

        let (assign53690_e68787, assign53690_e68787_d_n5, assign53690_e68787_d_n6, assign53690_e68787_d_n7, assign53690_e68787_d_n8,) = {
    if (locals.var_guard1506 != 0.0) {
        let assign53690_e68773: f64 = (locals.var_eta_p_ac * locals.var_dps_ac);
        let assign53690_e68776: f64 = (locals.var_fj * locals.var_gdl_ac);
        let assign53690_e68778: f64 = (assign53690_e68776 * 0.3333333333333333);
        let assign53690_e68780: f64 = (assign53690_e68778 - 1.0);
        let assign53690_e68782: f64 = (assign53690_e68780 + locals.var_gdl_ac);
        let assign53690_e68783: f64 = (assign53690_e68773 * assign53690_e68782);
        let assign53690_e68784: f64 = (0.5 * assign53690_e68783);
        let assign53690_e68785: f64 = (locals.var_voxm_ac + assign53690_e68784);
        (assign53690_e68785, (locals.var_voxm_ac_dn5 + (0.5 * ((((locals.var_eta_p_ac_dn5 * locals.var_dps_ac) + (locals.var_eta_p_ac * locals.var_dps_ac_dn5)) * assign53690_e68782) + (assign53690_e68773 * ((((locals.var_fj_dn5 * locals.var_gdl_ac) + (locals.var_fj * locals.var_gdl_ac_dn5)) * 0.3333333333333333) + locals.var_gdl_ac_dn5))))), (locals.var_voxm_ac_dn6 + (0.5 * ((((locals.var_eta_p_ac_dn6 * locals.var_dps_ac) + (locals.var_eta_p_ac * locals.var_dps_ac_dn6)) * assign53690_e68782) + (assign53690_e68773 * ((((locals.var_fj_dn6 * locals.var_gdl_ac) + (locals.var_fj * locals.var_gdl_ac_dn6)) * 0.3333333333333333) + locals.var_gdl_ac_dn6))))), (locals.var_voxm_ac_dn7 + (0.5 * ((((locals.var_eta_p_ac_dn7 * locals.var_dps_ac) + (locals.var_eta_p_ac * locals.var_dps_ac_dn7)) * assign53690_e68782) + (assign53690_e68773 * ((((locals.var_fj_dn7 * locals.var_gdl_ac) + (locals.var_fj * locals.var_gdl_ac_dn7)) * 0.3333333333333333) + locals.var_gdl_ac_dn7))))), (locals.var_voxm_ac_dn8 + (0.5 * ((((locals.var_eta_p_ac_dn8 * locals.var_dps_ac) + (locals.var_eta_p_ac * locals.var_dps_ac_dn8)) * assign53690_e68782) + (assign53690_e68773 * ((((locals.var_fj_dn8 * locals.var_gdl_ac) + (locals.var_fj * locals.var_gdl_ac_dn8)) * 0.3333333333333333) + locals.var_gdl_ac_dn8))))),)
    } else {
        (locals.var_qg_1, locals.var_qg_1_dn5, locals.var_qg_1_dn6, locals.var_qg_1_dn7, locals.var_qg_1_dn8,)
    }
};
        locals.var_qg_1 = assign53690_e68787;
        locals.var_qg_1_dn5 = assign53690_e68787_d_n5;
        locals.var_qg_1_dn6 = assign53690_e68787_d_n6;
        locals.var_qg_1_dn7 = assign53690_e68787_d_n7;
        locals.var_qg_1_dn8 = assign53690_e68787_d_n8;

        let (assign53700_e68795, assign53700_e68795_d_n5, assign53700_e68795_d_n6, assign53700_e68795_d_n7, assign53700_e68795_d_n8,) = {
    if (locals.var_guard1506 != 0.0) {
        let assign53700_e68791: f64 = (locals.var_alpha_ac * locals.var_dps_ac);
        let assign53700_e68793: f64 = (assign53700_e68791 * 0.16666666666666666);
        (assign53700_e68793, (((locals.var_alpha_ac_dn5 * locals.var_dps_ac) + (locals.var_alpha_ac * locals.var_dps_ac_dn5)) * 0.16666666666666666), (((locals.var_alpha_ac_dn6 * locals.var_dps_ac) + (locals.var_alpha_ac * locals.var_dps_ac_dn6)) * 0.16666666666666666), (((locals.var_alpha_ac_dn7 * locals.var_dps_ac) + (locals.var_alpha_ac * locals.var_dps_ac_dn7)) * 0.16666666666666666), (((locals.var_alpha_ac_dn8 * locals.var_dps_ac) + (locals.var_alpha_ac * locals.var_dps_ac_dn8)) * 0.16666666666666666),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign53700_e68795;
        locals.var_temp__blk936_dn5 = assign53700_e68795_d_n5;
        locals.var_temp__blk936_dn6 = assign53700_e68795_d_n6;
        locals.var_temp__blk936_dn7 = assign53700_e68795_d_n7;
        locals.var_temp__blk936_dn8 = assign53700_e68795_d_n8;

        let assign53710_e68798: f64 = if p.p49 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1509 = assign53710_e68798;

        let (assign53720_e68804, assign53720_e68804_d_n5, assign53720_e68804_d_n6, assign53720_e68804_d_n7, assign53720_e68804_d_n8,) = {
    if ((locals.var_guard1506 != 0.0) && (locals.var_guard1509 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qclm, locals.var_qclm_dn5, locals.var_qclm_dn6, locals.var_qclm_dn7, locals.var_qclm_dn8,)
    }
};
        locals.var_qclm = assign53720_e68804;
        locals.var_qclm_dn5 = assign53720_e68804_d_n5;
        locals.var_qclm_dn6 = assign53720_e68804_d_n6;
        locals.var_qclm_dn7 = assign53720_e68804_d_n7;
        locals.var_qclm_dn8 = assign53720_e68804_d_n8;

        let (assign53730_e68824, assign53730_e68824_d_n5, assign53730_e68824_d_n6, assign53730_e68824_d_n7, assign53730_e68824_d_n8,) = {
    if ((locals.var_guard1506 != 0.0) && (locals.var_guard1509 != 0.0)) {
        let assign53730_e68810: f64 = (0.5 * locals.var_gdl_ac);
        let assign53730_e68812: f64 = (assign53730_e68810 * locals.var_gdl_ac);
        let assign53730_e68816: f64 = (3.0 * locals.var_temp__blk936);
        let assign53730_e68819: f64 = (2.0 - locals.var_fj);
        let assign53730_e68820: f64 = (assign53730_e68816 * assign53730_e68819);
        let assign53730_e68821: f64 = (locals.var_qim_ac - assign53730_e68820);
        let assign53730_e68822: f64 = (assign53730_e68812 * assign53730_e68821);
        (assign53730_e68822, (((((0.5 * locals.var_gdl_ac_dn5) * locals.var_gdl_ac) + (assign53730_e68810 * locals.var_gdl_ac_dn5)) * assign53730_e68821) + (assign53730_e68812 * (locals.var_qim_ac_dn5 - (((3.0 * locals.var_temp__blk936_dn5) * assign53730_e68819) + (assign53730_e68816 * (-locals.var_fj_dn5)))))), (((((0.5 * locals.var_gdl_ac_dn6) * locals.var_gdl_ac) + (assign53730_e68810 * locals.var_gdl_ac_dn6)) * assign53730_e68821) + (assign53730_e68812 * (locals.var_qim_ac_dn6 - (((3.0 * locals.var_temp__blk936_dn6) * assign53730_e68819) + (assign53730_e68816 * (-locals.var_fj_dn6)))))), (((((0.5 * locals.var_gdl_ac_dn7) * locals.var_gdl_ac) + (assign53730_e68810 * locals.var_gdl_ac_dn7)) * assign53730_e68821) + (assign53730_e68812 * (locals.var_qim_ac_dn7 - (((3.0 * locals.var_temp__blk936_dn7) * assign53730_e68819) + (assign53730_e68816 * (-locals.var_fj_dn7)))))), (((((0.5 * locals.var_gdl_ac_dn8) * locals.var_gdl_ac) + (assign53730_e68810 * locals.var_gdl_ac_dn8)) * assign53730_e68821) + (assign53730_e68812 * (locals.var_qim_ac_dn8 - (((3.0 * locals.var_temp__blk936_dn8) * assign53730_e68819) + (assign53730_e68816 * (-locals.var_fj_dn8)))))),)
    } else {
        (locals.var_qd_1, locals.var_qd_1_dn5, locals.var_qd_1_dn6, locals.var_qd_1_dn7, locals.var_qd_1_dn8,)
    }
};
        locals.var_qd_1 = assign53730_e68824;
        locals.var_qd_1_dn5 = assign53730_e68824_d_n5;
        locals.var_qd_1_dn6 = assign53730_e68824_d_n6;
        locals.var_qd_1_dn7 = assign53730_e68824_d_n7;
        locals.var_qd_1_dn8 = assign53730_e68824_d_n8;

        let (assign53740_e68841, assign53740_e68841_d_n5, assign53740_e68841_d_n6, assign53740_e68841_d_n7, assign53740_e68841_d_n8,) = {
    if ((locals.var_guard1506 != 0.0) && (locals.var_guard1509 == 0.0)) {
        let assign53740_e68831: f64 = (1.0 - locals.var_gdl_ac);
        let assign53740_e68836: f64 = (locals.var_alpha_ac * locals.var_dps_ac);
        let assign53740_e68837: f64 = (0.5 * assign53740_e68836);
        let assign53740_e68838: f64 = (locals.var_qim_ac - assign53740_e68837);
        let assign53740_e68839: f64 = (assign53740_e68831 * assign53740_e68838);
        (assign53740_e68839, (((-locals.var_gdl_ac_dn5) * assign53740_e68838) + (assign53740_e68831 * (locals.var_qim_ac_dn5 - (0.5 * ((locals.var_alpha_ac_dn5 * locals.var_dps_ac) + (locals.var_alpha_ac * locals.var_dps_ac_dn5)))))), (((-locals.var_gdl_ac_dn6) * assign53740_e68838) + (assign53740_e68831 * (locals.var_qim_ac_dn6 - (0.5 * ((locals.var_alpha_ac_dn6 * locals.var_dps_ac) + (locals.var_alpha_ac * locals.var_dps_ac_dn6)))))), (((-locals.var_gdl_ac_dn7) * assign53740_e68838) + (assign53740_e68831 * (locals.var_qim_ac_dn7 - (0.5 * ((locals.var_alpha_ac_dn7 * locals.var_dps_ac) + (locals.var_alpha_ac * locals.var_dps_ac_dn7)))))), (((-locals.var_gdl_ac_dn8) * assign53740_e68838) + (assign53740_e68831 * (locals.var_qim_ac_dn8 - (0.5 * ((locals.var_alpha_ac_dn8 * locals.var_dps_ac) + (locals.var_alpha_ac * locals.var_dps_ac_dn8)))))),)
    } else {
        (locals.var_qclm, locals.var_qclm_dn5, locals.var_qclm_dn6, locals.var_qclm_dn7, locals.var_qclm_dn8,)
    }
};
        locals.var_qclm = assign53740_e68841;
        locals.var_qclm_dn5 = assign53740_e68841_d_n5;
        locals.var_qclm_dn6 = assign53740_e68841_d_n6;
        locals.var_qclm_dn7 = assign53740_e68841_d_n7;
        locals.var_qclm_dn8 = assign53740_e68841_d_n8;

        let (assign53750_e68870, assign53750_e68870_d_n5, assign53750_e68870_d_n6, assign53750_e68870_d_n7, assign53750_e68870_d_n8,) = {
    if ((locals.var_guard1506 != 0.0) && (locals.var_guard1509 == 0.0)) {
        let assign53750_e68849: f64 = (locals.var_gdl_ac * locals.var_gdl_ac);
        let assign53750_e68854: f64 = (1.0 - locals.var_fj);
        let assign53750_e68857: f64 = (0.2 * locals.var_fj2);
        let assign53750_e68858: f64 = (assign53750_e68854 - assign53750_e68857);
        let assign53750_e68859: f64 = (locals.var_temp__blk936 * assign53750_e68858);
        let assign53750_e68860: f64 = (locals.var_qim_ac - assign53750_e68859);
        let assign53750_e68861: f64 = (assign53750_e68849 * assign53750_e68860);
        let assign53750_e68865: f64 = (1.0 + locals.var_gdl_ac);
        let assign53750_e68866: f64 = (locals.var_qclm * assign53750_e68865);
        let assign53750_e68867: f64 = (assign53750_e68861 + assign53750_e68866);
        let assign53750_e68868: f64 = (0.5 * assign53750_e68867);
        (assign53750_e68868, (0.5 * (((((locals.var_gdl_ac_dn5 * locals.var_gdl_ac) + (locals.var_gdl_ac * locals.var_gdl_ac_dn5)) * assign53750_e68860) + (assign53750_e68849 * (locals.var_qim_ac_dn5 - ((locals.var_temp__blk936_dn5 * assign53750_e68858) + (locals.var_temp__blk936 * ((-locals.var_fj_dn5) - (0.2 * locals.var_fj2_dn5))))))) + ((locals.var_qclm_dn5 * assign53750_e68865) + (locals.var_qclm * locals.var_gdl_ac_dn5)))), (0.5 * (((((locals.var_gdl_ac_dn6 * locals.var_gdl_ac) + (locals.var_gdl_ac * locals.var_gdl_ac_dn6)) * assign53750_e68860) + (assign53750_e68849 * (locals.var_qim_ac_dn6 - ((locals.var_temp__blk936_dn6 * assign53750_e68858) + (locals.var_temp__blk936 * ((-locals.var_fj_dn6) - (0.2 * locals.var_fj2_dn6))))))) + ((locals.var_qclm_dn6 * assign53750_e68865) + (locals.var_qclm * locals.var_gdl_ac_dn6)))), (0.5 * (((((locals.var_gdl_ac_dn7 * locals.var_gdl_ac) + (locals.var_gdl_ac * locals.var_gdl_ac_dn7)) * assign53750_e68860) + (assign53750_e68849 * (locals.var_qim_ac_dn7 - ((locals.var_temp__blk936_dn7 * assign53750_e68858) + (locals.var_temp__blk936 * ((-locals.var_fj_dn7) - (0.2 * locals.var_fj2_dn7))))))) + ((locals.var_qclm_dn7 * assign53750_e68865) + (locals.var_qclm * locals.var_gdl_ac_dn7)))), (0.5 * (((((locals.var_gdl_ac_dn8 * locals.var_gdl_ac) + (locals.var_gdl_ac * locals.var_gdl_ac_dn8)) * assign53750_e68860) + (assign53750_e68849 * (locals.var_qim_ac_dn8 - ((locals.var_temp__blk936_dn8 * assign53750_e68858) + (locals.var_temp__blk936 * ((-locals.var_fj_dn8) - (0.2 * locals.var_fj2_dn8))))))) + ((locals.var_qclm_dn8 * assign53750_e68865) + (locals.var_qclm * locals.var_gdl_ac_dn8)))),)
    } else {
        (locals.var_qd_1, locals.var_qd_1_dn5, locals.var_qd_1_dn6, locals.var_qd_1_dn7, locals.var_qd_1_dn8,)
    }
};
        locals.var_qd_1 = assign53750_e68870;
        locals.var_qd_1_dn5 = assign53750_e68870_d_n5;
        locals.var_qd_1_dn6 = assign53750_e68870_d_n6;
        locals.var_qd_1_dn7 = assign53750_e68870_d_n7;
        locals.var_qd_1_dn8 = assign53750_e68870_d_n8;

        let (assign53760_e68882, assign53760_e68882_d_n5, assign53760_e68882_d_n6, assign53760_e68882_d_n7, assign53760_e68882_d_n8,) = {
    if (locals.var_guard1506 != 0.0) {
        let assign53760_e68876: f64 = (locals.var_temp__blk936 * locals.var_fj);
        let assign53760_e68877: f64 = (locals.var_qim_ac + assign53760_e68876);
        let assign53760_e68878: f64 = (locals.var_gdl_ac * assign53760_e68877);
        let assign53760_e68880: f64 = (assign53760_e68878 + locals.var_qclm);
        (assign53760_e68880, (((locals.var_gdl_ac_dn5 * assign53760_e68877) + (locals.var_gdl_ac * (locals.var_qim_ac_dn5 + ((locals.var_temp__blk936_dn5 * locals.var_fj) + (locals.var_temp__blk936 * locals.var_fj_dn5))))) + locals.var_qclm_dn5), (((locals.var_gdl_ac_dn6 * assign53760_e68877) + (locals.var_gdl_ac * (locals.var_qim_ac_dn6 + ((locals.var_temp__blk936_dn6 * locals.var_fj) + (locals.var_temp__blk936 * locals.var_fj_dn6))))) + locals.var_qclm_dn6), (((locals.var_gdl_ac_dn7 * assign53760_e68877) + (locals.var_gdl_ac * (locals.var_qim_ac_dn7 + ((locals.var_temp__blk936_dn7 * locals.var_fj) + (locals.var_temp__blk936 * locals.var_fj_dn7))))) + locals.var_qclm_dn7), (((locals.var_gdl_ac_dn8 * assign53760_e68877) + (locals.var_gdl_ac * (locals.var_qim_ac_dn8 + ((locals.var_temp__blk936_dn8 * locals.var_fj) + (locals.var_temp__blk936 * locals.var_fj_dn8))))) + locals.var_qclm_dn8),)
    } else {
        (locals.var_qi, locals.var_qi_dn5, locals.var_qi_dn6, locals.var_qi_dn7, locals.var_qi_dn8,)
    }
};
        locals.var_qi = assign53760_e68882;
        locals.var_qi_dn5 = assign53760_e68882_d_n5;
        locals.var_qi_dn6 = assign53760_e68882_d_n6;
        locals.var_qi_dn7 = assign53760_e68882_d_n7;
        locals.var_qi_dn8 = assign53760_e68882_d_n8;

        let (assign53770_e68888, assign53770_e68888_d_n5, assign53770_e68888_d_n6, assign53770_e68888_d_n7, assign53770_e68888_d_n8,) = {
    if (locals.var_guard1506 != 0.0) {
        let assign53770_e68886: f64 = (locals.var_qg_1 - locals.var_qi);
        (assign53770_e68886, (locals.var_qg_1_dn5 - locals.var_qi_dn5), (locals.var_qg_1_dn6 - locals.var_qi_dn6), (locals.var_qg_1_dn7 - locals.var_qi_dn7), (locals.var_qg_1_dn8 - locals.var_qi_dn8),)
    } else {
        (locals.var_qb_1, locals.var_qb_1_dn5, locals.var_qb_1_dn6, locals.var_qb_1_dn7, locals.var_qb_1_dn8,)
    }
};
        locals.var_qb_1 = assign53770_e68888;
        locals.var_qb_1_dn5 = assign53770_e68888_d_n5;
        locals.var_qb_1_dn6 = assign53770_e68888_d_n6;
        locals.var_qb_1_dn7 = assign53770_e68888_d_n7;
        locals.var_qb_1_dn8 = assign53770_e68888_d_n8;

        let assign53780_e68891: f64 = (locals.var_qg_1 * locals.var_cox_qm);
        locals.var_qg = assign53780_e68891;
        locals.var_qg_dn5 = ((locals.var_qg_1_dn5 * locals.var_cox_qm) + (locals.var_qg_1 * locals.var_cox_qm_dn5));
        locals.var_qg_dn6 = ((locals.var_qg_1_dn6 * locals.var_cox_qm) + (locals.var_qg_1 * locals.var_cox_qm_dn6));
        locals.var_qg_dn7 = ((locals.var_qg_1_dn7 * locals.var_cox_qm) + (locals.var_qg_1 * locals.var_cox_qm_dn7));
        locals.var_qg_dn8 = ((locals.var_qg_1_dn8 * locals.var_cox_qm) + (locals.var_qg_1 * locals.var_cox_qm_dn8));

        let assign53790_e68893: f64 = (-locals.var_qd_1);
        let assign53790_e68895: f64 = (assign53790_e68893 * locals.var_cox_qm);
        locals.var_qd = assign53790_e68895;
        locals.var_qd_dn5 = (((-locals.var_qd_1_dn5) * locals.var_cox_qm) + (assign53790_e68893 * locals.var_cox_qm_dn5));
        locals.var_qd_dn6 = (((-locals.var_qd_1_dn6) * locals.var_cox_qm) + (assign53790_e68893 * locals.var_cox_qm_dn6));
        locals.var_qd_dn7 = (((-locals.var_qd_1_dn7) * locals.var_cox_qm) + (assign53790_e68893 * locals.var_cox_qm_dn7));
        locals.var_qd_dn8 = (((-locals.var_qd_1_dn8) * locals.var_cox_qm) + (assign53790_e68893 * locals.var_cox_qm_dn8));

        let assign53800_e68897: f64 = (-locals.var_qb_1);
        let assign53800_e68899: f64 = (assign53800_e68897 * locals.var_cox_qm);
        locals.var_qb = assign53800_e68899;
        locals.var_qb_dn5 = (((-locals.var_qb_1_dn5) * locals.var_cox_qm) + (assign53800_e68897 * locals.var_cox_qm_dn5));
        locals.var_qb_dn6 = (((-locals.var_qb_1_dn6) * locals.var_cox_qm) + (assign53800_e68897 * locals.var_cox_qm_dn6));
        locals.var_qb_dn7 = (((-locals.var_qb_1_dn7) * locals.var_cox_qm) + (assign53800_e68897 * locals.var_cox_qm_dn7));
        locals.var_qb_dn8 = (((-locals.var_qb_1_dn8) * locals.var_cox_qm) + (assign53800_e68897 * locals.var_cox_qm_dn8));

        locals.var_qsinr = 0.0;
        locals.var_qsinr_dn5 = 0.0;
        locals.var_qsinr_dn6 = 0.0;
        locals.var_qsinr_dn7 = 0.0;
        locals.var_qsinr_dn8 = 0.0;

        locals.var_qdinr = 0.0;
        locals.var_qdinr_dn5 = 0.0;
        locals.var_qdinr_dn6 = 0.0;
        locals.var_qdinr_dn7 = 0.0;
        locals.var_qdinr_dn8 = 0.0;

        locals.var_qginr = 0.0;
        locals.var_qginr_dn5 = 0.0;
        locals.var_qginr_dn6 = 0.0;
        locals.var_qginr_dn7 = 0.0;
        locals.var_qginr_dn8 = 0.0;

    }

    pub(super) fn stamp_transient_block_49(
        locals: &mut StampLocals,
    ) {
        let assign53840_e68909: f64 = if ((locals.var_cinr_i > 0.0) || (locals.var_cinrd_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1510 = assign53840_e68909;

        let (assign53850_e68913, assign53850_e68913_d_n5, assign53850_e68913_d_n6, assign53850_e68913_d_n7, assign53850_e68913_d_n8,) = {
    if (locals.var_guard1510 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_finracc, locals.var_finracc_dn5, locals.var_finracc_dn6, locals.var_finracc_dn7, locals.var_finracc_dn8,)
    }
};
        locals.var_finracc = assign53850_e68913;
        locals.var_finracc_dn5 = assign53850_e68913_d_n5;
        locals.var_finracc_dn6 = assign53850_e68913_d_n6;
        locals.var_finracc_dn7 = assign53850_e68913_d_n7;
        locals.var_finracc_dn8 = assign53850_e68913_d_n8;

        let (assign53860_e68917, assign53860_e68917_d_n5, assign53860_e68917_d_n6, assign53860_e68917_d_n7, assign53860_e68917_d_n8,) = {
    if (locals.var_guard1510 != 0.0) {
        (locals.var_vgb1_ac, locals.var_vgb1_ac_dn5, locals.var_vgb1_ac_dn6, locals.var_vgb1_ac_dn7, locals.var_vgb1_ac_dn8,)
    } else {
        (locals.var_dvinracc, locals.var_dvinracc_dn5, locals.var_dvinracc_dn6, locals.var_dvinracc_dn7, locals.var_dvinracc_dn8,)
    }
};
        locals.var_dvinracc = assign53860_e68917;
        locals.var_dvinracc_dn5 = assign53860_e68917_d_n5;
        locals.var_dvinracc_dn6 = assign53860_e68917_d_n6;
        locals.var_dvinracc_dn7 = assign53860_e68917_d_n7;
        locals.var_dvinracc_dn8 = assign53860_e68917_d_n8;

        let assign53870_e68920: f64 = if locals.var_fcinracc_i > 1e-10 { 1.0 } else { 0.0 };
        locals.var_guard1511 = assign53870_e68920;

        let (assign53880_e68930, assign53880_e68930_d_n5, assign53880_e68930_d_n6, assign53880_e68930_d_n7, assign53880_e68930_d_n8,) = {
    if ((locals.var_guard1510 != 0.0) && (locals.var_guard1511 != 0.0)) {
        let assign53880_e68926: f64 = (locals.var_vgb1_ac - locals.var_dvfbinr_i);
        let assign53880_e68928: f64 = (assign53880_e68926 + locals.var_vinr_max);
        (assign53880_e68928, locals.var_vgb1_ac_dn5, locals.var_vgb1_ac_dn6, locals.var_vgb1_ac_dn7, locals.var_vgb1_ac_dn8,)
    } else {
        (locals.var_vginr, locals.var_vginr_dn5, locals.var_vginr_dn6, locals.var_vginr_dn7, locals.var_vginr_dn8,)
    }
};
        locals.var_vginr = assign53880_e68930;
        locals.var_vginr_dn5 = assign53880_e68930_d_n5;
        locals.var_vginr_dn6 = assign53880_e68930_d_n6;
        locals.var_vginr_dn7 = assign53880_e68930_d_n7;
        locals.var_vginr_dn8 = assign53880_e68930_d_n8;

        let (assign53890_e68951, assign53890_e68951_d_n5, assign53890_e68951_d_n6, assign53890_e68951_d_n7, assign53890_e68951_d_n8,) = {
    if ((locals.var_guard1510 != 0.0) && (locals.var_guard1511 != 0.0)) {
        let assign53890_e68937: f64 = (locals.var_vginr + locals.var_vinr_max);
        let assign53890_e68940: f64 = (locals.var_vginr - locals.var_vinr_max);
        let assign53890_e68943: f64 = (locals.var_vginr - locals.var_vinr_max);
        let assign53890_e68944: f64 = (assign53890_e68940 * assign53890_e68943);
        let assign53890_e68946: f64 = (assign53890_e68944 + locals.var_ainr);
        let assign53890_e68947: f64 = (assign53890_e68946).sqrt();
        let assign53890_e68948: f64 = (assign53890_e68937 + assign53890_e68947);
        let assign53890_e68949: f64 = (0.5 * assign53890_e68948);
        (assign53890_e68949, (0.5 * (locals.var_vginr_dn5 + (((locals.var_vginr_dn5 * assign53890_e68943) + (assign53890_e68940 * locals.var_vginr_dn5)) / (2.0 * assign53890_e68947)))), (0.5 * (locals.var_vginr_dn6 + (((locals.var_vginr_dn6 * assign53890_e68943) + (assign53890_e68940 * locals.var_vginr_dn6)) / (2.0 * assign53890_e68947)))), (0.5 * (locals.var_vginr_dn7 + (((locals.var_vginr_dn7 * assign53890_e68943) + (assign53890_e68940 * locals.var_vginr_dn7)) / (2.0 * assign53890_e68947)))), (0.5 * (locals.var_vginr_dn8 + (((locals.var_vginr_dn8 * assign53890_e68943) + (assign53890_e68940 * locals.var_vginr_dn8)) / (2.0 * assign53890_e68947)))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign53890_e68951;
        locals.var_temp__blk936_dn5 = assign53890_e68951_d_n5;
        locals.var_temp__blk936_dn6 = assign53890_e68951_d_n6;
        locals.var_temp__blk936_dn7 = assign53890_e68951_d_n7;
        locals.var_temp__blk936_dn8 = assign53890_e68951_d_n8;

        let (assign53900_e68965, assign53900_e68965_d_n5, assign53900_e68965_d_n6, assign53900_e68965_d_n7, assign53900_e68965_d_n8,) = {
    if ((locals.var_guard1510 != 0.0) && (locals.var_guard1511 != 0.0)) {
        let assign53900_e68958: f64 = (2.0 * locals.var_temp__blk936);
        let assign53900_e68960: f64 = (assign53900_e68958 - locals.var_vinr_max);
        let assign53900_e68962: f64 = (assign53900_e68960 - locals.var_vginr);
        let assign53900_e68963: f64 = (locals.var_temp__blk936 * assign53900_e68962);
        (assign53900_e68963, ((locals.var_temp__blk936_dn5 * assign53900_e68962) + (locals.var_temp__blk936 * ((2.0 * locals.var_temp__blk936_dn5) - locals.var_vginr_dn5))), ((locals.var_temp__blk936_dn6 * assign53900_e68962) + (locals.var_temp__blk936 * ((2.0 * locals.var_temp__blk936_dn6) - locals.var_vginr_dn6))), ((locals.var_temp__blk936_dn7 * assign53900_e68962) + (locals.var_temp__blk936 * ((2.0 * locals.var_temp__blk936_dn7) - locals.var_vginr_dn7))), ((locals.var_temp__blk936_dn8 * assign53900_e68962) + (locals.var_temp__blk936 * ((2.0 * locals.var_temp__blk936_dn8) - locals.var_vginr_dn8))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign53900_e68965;
        locals.var_temp1_dn5 = assign53900_e68965_d_n5;
        locals.var_temp1_dn6 = assign53900_e68965_d_n6;
        locals.var_temp1_dn7 = assign53900_e68965_d_n7;
        locals.var_temp1_dn8 = assign53900_e68965_d_n8;

        let (assign53910_e68973, assign53910_e68973_d_n5, assign53910_e68973_d_n6, assign53910_e68973_d_n7, assign53910_e68973_d_n8,) = {
    if ((locals.var_guard1510 != 0.0) && (locals.var_guard1511 != 0.0)) {
        let assign53910_e68971: f64 = (locals.var_vinr_max / locals.var_temp__blk936);
        (assign53910_e68971, (-((locals.var_vinr_max * locals.var_temp__blk936_dn5) / (locals.var_temp__blk936 * locals.var_temp__blk936))), (-((locals.var_vinr_max * locals.var_temp__blk936_dn6) / (locals.var_temp__blk936 * locals.var_temp__blk936))), (-((locals.var_vinr_max * locals.var_temp__blk936_dn7) / (locals.var_temp__blk936 * locals.var_temp__blk936))), (-((locals.var_vinr_max * locals.var_temp__blk936_dn8) / (locals.var_temp__blk936 * locals.var_temp__blk936))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8,)
    }
};
        locals.var_temp2 = assign53910_e68973;
        locals.var_temp2_dn5 = assign53910_e68973_d_n5;
        locals.var_temp2_dn6 = assign53910_e68973_d_n6;
        locals.var_temp2_dn7 = assign53910_e68973_d_n7;
        locals.var_temp2_dn8 = assign53910_e68973_d_n8;

        let (assign53920_e68981, assign53920_e68981_d_n5, assign53920_e68981_d_n6, assign53920_e68981_d_n7, assign53920_e68981_d_n8,) = {
    if ((locals.var_guard1510 != 0.0) && (locals.var_guard1511 != 0.0)) {
        let assign53920_e68979: f64 = (locals.var_vginr * locals.var_temp2);
        (assign53920_e68979, ((locals.var_vginr_dn5 * locals.var_temp2) + (locals.var_vginr * locals.var_temp2_dn5)), ((locals.var_vginr_dn6 * locals.var_temp2) + (locals.var_vginr * locals.var_temp2_dn6)), ((locals.var_vginr_dn7 * locals.var_temp2) + (locals.var_vginr * locals.var_temp2_dn7)), ((locals.var_vginr_dn8 * locals.var_temp2) + (locals.var_vginr * locals.var_temp2_dn8)),)
    } else {
        (locals.var_vginreff, locals.var_vginreff_dn5, locals.var_vginreff_dn6, locals.var_vginreff_dn7, locals.var_vginreff_dn8,)
    }
};
        locals.var_vginreff = assign53920_e68981;
        locals.var_vginreff_dn5 = assign53920_e68981_d_n5;
        locals.var_vginreff_dn6 = assign53920_e68981_d_n6;
        locals.var_vginreff_dn7 = assign53920_e68981_d_n7;
        locals.var_vginreff_dn8 = assign53920_e68981_d_n8;

        let (assign53930_e68992, assign53930_e68992_d_n5, assign53930_e68992_d_n6, assign53930_e68992_d_n7, assign53930_e68992_d_n8,) = {
    if ((locals.var_guard1510 != 0.0) && (locals.var_guard1511 != 0.0)) {
        let assign53930_e68988: f64 = (locals.var_vginreff * locals.var_fcinracc_i);
        let assign53930_e68989: f64 = (1.0 - assign53930_e68988);
        let assign53930_e68990: f64 = (assign53930_e68989).sqrt();
        (assign53930_e68990, ((-(locals.var_vginreff_dn5 * locals.var_fcinracc_i)) / (2.0 * assign53930_e68990)), ((-(locals.var_vginreff_dn6 * locals.var_fcinracc_i)) / (2.0 * assign53930_e68990)), ((-(locals.var_vginreff_dn7 * locals.var_fcinracc_i)) / (2.0 * assign53930_e68990)), ((-(locals.var_vginreff_dn8 * locals.var_fcinracc_i)) / (2.0 * assign53930_e68990)),)
    } else {
        (locals.var_fqinr, locals.var_fqinr_dn5, locals.var_fqinr_dn6, locals.var_fqinr_dn7, locals.var_fqinr_dn8,)
    }
};
        locals.var_fqinr = assign53930_e68992;
        locals.var_fqinr_dn5 = assign53930_e68992_d_n5;
        locals.var_fqinr_dn6 = assign53930_e68992_d_n6;
        locals.var_fqinr_dn7 = assign53930_e68992_d_n7;
        locals.var_fqinr_dn8 = assign53930_e68992_d_n8;

        let (assign53940_e69006, assign53940_e69006_d_n5, assign53940_e69006_d_n6, assign53940_e69006_d_n7, assign53940_e69006_d_n8,) = {
    if ((locals.var_guard1510 != 0.0) && (locals.var_guard1511 != 0.0)) {
        let assign53940_e68998: f64 = (1.0 - locals.var_fqinr);
        let assign53940_e69000: f64 = (assign53940_e68998 / locals.var_fcinracc_i);
        let assign53940_e69002: f64 = (assign53940_e69000 + locals.var_vginr);
        let assign53940_e69004: f64 = (assign53940_e69002 - locals.var_vginreff);
        (assign53940_e69004, ((((-locals.var_fqinr_dn5) / locals.var_fcinracc_i) + locals.var_vginr_dn5) - locals.var_vginreff_dn5), ((((-locals.var_fqinr_dn6) / locals.var_fcinracc_i) + locals.var_vginr_dn6) - locals.var_vginreff_dn6), ((((-locals.var_fqinr_dn7) / locals.var_fcinracc_i) + locals.var_vginr_dn7) - locals.var_vginreff_dn7), ((((-locals.var_fqinr_dn8) / locals.var_fcinracc_i) + locals.var_vginr_dn8) - locals.var_vginreff_dn8),)
    } else {
        (locals.var_dvinracc, locals.var_dvinracc_dn5, locals.var_dvinracc_dn6, locals.var_dvinracc_dn7, locals.var_dvinracc_dn8,)
    }
};
        locals.var_dvinracc = assign53940_e69006;
        locals.var_dvinracc_dn5 = assign53940_e69006_d_n5;
        locals.var_dvinracc_dn6 = assign53940_e69006_d_n6;
        locals.var_dvinracc_dn7 = assign53940_e69006_d_n7;
        locals.var_dvinracc_dn8 = assign53940_e69006_d_n8;

        let (assign53950_e69030, assign53950_e69030_d_n5, assign53950_e69030_d_n6, assign53950_e69030_d_n7, assign53950_e69030_d_n8,) = {
    if ((locals.var_guard1510 != 0.0) && (locals.var_guard1511 != 0.0)) {
        let assign53950_e69012: f64 = (0.5 / locals.var_fqinr);
        let assign53950_e69014: f64 = (assign53950_e69012 - 1.0);
        let assign53950_e69019: f64 = (locals.var_vinr_max - locals.var_temp__blk936);
        let assign53950_e69020: f64 = (locals.var_vginr * assign53950_e69019);
        let assign53950_e69021: f64 = (locals.var_temp1 + assign53950_e69020);
        let assign53950_e69022: f64 = (assign53950_e69014 * assign53950_e69021);
        let assign53950_e69024: f64 = (assign53950_e69022 * locals.var_temp2);
        let assign53950_e69026: f64 = (assign53950_e69024 / locals.var_temp1);
        let assign53950_e69028: f64 = (assign53950_e69026 + 1.0);
        (assign53950_e69028, ((((((((-((0.5 * locals.var_fqinr_dn5) / (locals.var_fqinr * locals.var_fqinr))) * assign53950_e69021) + (assign53950_e69014 * (locals.var_temp1_dn5 + ((locals.var_vginr_dn5 * assign53950_e69019) + (locals.var_vginr * (-locals.var_temp__blk936_dn5)))))) * locals.var_temp2) + (assign53950_e69022 * locals.var_temp2_dn5)) * locals.var_temp1) - (assign53950_e69024 * locals.var_temp1_dn5)) / (locals.var_temp1 * locals.var_temp1)), ((((((((-((0.5 * locals.var_fqinr_dn6) / (locals.var_fqinr * locals.var_fqinr))) * assign53950_e69021) + (assign53950_e69014 * (locals.var_temp1_dn6 + ((locals.var_vginr_dn6 * assign53950_e69019) + (locals.var_vginr * (-locals.var_temp__blk936_dn6)))))) * locals.var_temp2) + (assign53950_e69022 * locals.var_temp2_dn6)) * locals.var_temp1) - (assign53950_e69024 * locals.var_temp1_dn6)) / (locals.var_temp1 * locals.var_temp1)), ((((((((-((0.5 * locals.var_fqinr_dn7) / (locals.var_fqinr * locals.var_fqinr))) * assign53950_e69021) + (assign53950_e69014 * (locals.var_temp1_dn7 + ((locals.var_vginr_dn7 * assign53950_e69019) + (locals.var_vginr * (-locals.var_temp__blk936_dn7)))))) * locals.var_temp2) + (assign53950_e69022 * locals.var_temp2_dn7)) * locals.var_temp1) - (assign53950_e69024 * locals.var_temp1_dn7)) / (locals.var_temp1 * locals.var_temp1)), ((((((((-((0.5 * locals.var_fqinr_dn8) / (locals.var_fqinr * locals.var_fqinr))) * assign53950_e69021) + (assign53950_e69014 * (locals.var_temp1_dn8 + ((locals.var_vginr_dn8 * assign53950_e69019) + (locals.var_vginr * (-locals.var_temp__blk936_dn8)))))) * locals.var_temp2) + (assign53950_e69022 * locals.var_temp2_dn8)) * locals.var_temp1) - (assign53950_e69024 * locals.var_temp1_dn8)) / (locals.var_temp1 * locals.var_temp1)),)
    } else {
        (locals.var_finracc, locals.var_finracc_dn5, locals.var_finracc_dn6, locals.var_finracc_dn7, locals.var_finracc_dn8,)
    }
};
        locals.var_finracc = assign53950_e69030;
        locals.var_finracc_dn5 = assign53950_e69030_d_n5;
        locals.var_finracc_dn6 = assign53950_e69030_d_n6;
        locals.var_finracc_dn7 = assign53950_e69030_d_n7;
        locals.var_finracc_dn8 = assign53950_e69030_d_n8;

        let (assign53960_e69034, assign53960_e69034_d_n5, assign53960_e69034_d_n6, assign53960_e69034_d_n7, assign53960_e69034_d_n8,) = {
    if (locals.var_guard1510 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_finrdep, locals.var_finrdep_dn5, locals.var_finrdep_dn6, locals.var_finrdep_dn7, locals.var_finrdep_dn8,)
    }
};
        locals.var_finrdep = assign53960_e69034;
        locals.var_finrdep_dn5 = assign53960_e69034_d_n5;
        locals.var_finrdep_dn6 = assign53960_e69034_d_n6;
        locals.var_finrdep_dn7 = assign53960_e69034_d_n7;
        locals.var_finrdep_dn8 = assign53960_e69034_d_n8;

        let (assign53970_e69038, assign53970_e69038_d_n5, assign53970_e69038_d_n6, assign53970_e69038_d_n7, assign53970_e69038_d_n8,) = {
    if (locals.var_guard1510 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dvinrdep, locals.var_dvinrdep_dn5, locals.var_dvinrdep_dn6, locals.var_dvinrdep_dn7, locals.var_dvinrdep_dn8,)
    }
};
        locals.var_dvinrdep = assign53970_e69038;
        locals.var_dvinrdep_dn5 = assign53970_e69038_d_n5;
        locals.var_dvinrdep_dn6 = assign53970_e69038_d_n6;
        locals.var_dvinrdep_dn7 = assign53970_e69038_d_n7;
        locals.var_dvinrdep_dn8 = assign53970_e69038_d_n8;

        let assign53980_e69041: f64 = if locals.var_fcinrdep_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1512 = assign53980_e69041;

        let (assign53990_e69057, assign53990_e69057_d_n5, assign53990_e69057_d_n6, assign53990_e69057_d_n7, assign53990_e69057_d_n8,) = {
    if ((locals.var_guard1510 != 0.0) && (locals.var_guard1512 != 0.0)) {
        let assign53990_e69047: f64 = (0.5 * locals.var_phib_ac);
        let assign53990_e69052: f64 = (locals.var_gf_ac * 0.7071067811865475);
        let assign53990_e69053: f64 = (1.0 + assign53990_e69052);
        let assign53990_e69054: f64 = (locals.var_phit1_ac * assign53990_e69053);
        let assign53990_e69055: f64 = (assign53990_e69047 + assign53990_e69054);
        (assign53990_e69055, ((locals.var_phit1_ac_dn5 * assign53990_e69053) + (locals.var_phit1_ac * (locals.var_gf_ac_dn5 * 0.7071067811865475))), ((locals.var_phit1_ac_dn6 * assign53990_e69053) + (locals.var_phit1_ac * (locals.var_gf_ac_dn6 * 0.7071067811865475))), ((locals.var_phit1_ac_dn7 * assign53990_e69053) + (locals.var_phit1_ac * (locals.var_gf_ac_dn7 * 0.7071067811865475))), ((locals.var_phit1_ac_dn8 * assign53990_e69053) + (locals.var_phit1_ac * (locals.var_gf_ac_dn8 * 0.7071067811865475))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign53990_e69057;
        locals.var_temp__blk936_dn5 = assign53990_e69057_d_n5;
        locals.var_temp__blk936_dn6 = assign53990_e69057_d_n6;
        locals.var_temp__blk936_dn7 = assign53990_e69057_d_n7;
        locals.var_temp__blk936_dn8 = assign53990_e69057_d_n8;

        let (assign54000_e69065, assign54000_e69065_d_n5, assign54000_e69065_d_n6, assign54000_e69065_d_n7, assign54000_e69065_d_n8,) = {
    if ((locals.var_guard1510 != 0.0) && (locals.var_guard1512 != 0.0)) {
        let assign54000_e69063: f64 = (locals.var_vgb1_ac / locals.var_temp__blk936);
        (assign54000_e69063, (((locals.var_vgb1_ac_dn5 * locals.var_temp__blk936) - (locals.var_vgb1_ac * locals.var_temp__blk936_dn5)) / (locals.var_temp__blk936 * locals.var_temp__blk936)), (((locals.var_vgb1_ac_dn6 * locals.var_temp__blk936) - (locals.var_vgb1_ac * locals.var_temp__blk936_dn6)) / (locals.var_temp__blk936 * locals.var_temp__blk936)), (((locals.var_vgb1_ac_dn7 * locals.var_temp__blk936) - (locals.var_vgb1_ac * locals.var_temp__blk936_dn7)) / (locals.var_temp__blk936 * locals.var_temp__blk936)), (((locals.var_vgb1_ac_dn8 * locals.var_temp__blk936) - (locals.var_vgb1_ac * locals.var_temp__blk936_dn8)) / (locals.var_temp__blk936 * locals.var_temp__blk936)),)
    } else {
        (locals.var_xginrdep, locals.var_xginrdep_dn5, locals.var_xginrdep_dn6, locals.var_xginrdep_dn7, locals.var_xginrdep_dn8,)
    }
};
        locals.var_xginrdep = assign54000_e69065;
        locals.var_xginrdep_dn5 = assign54000_e69065_d_n5;
        locals.var_xginrdep_dn6 = assign54000_e69065_d_n6;
        locals.var_xginrdep_dn7 = assign54000_e69065_d_n7;
        locals.var_xginrdep_dn8 = assign54000_e69065_d_n8;

        let assign54010_e69067: f64 = (locals.var_xginrdep).abs();
        let assign54010_e69069: f64 = if assign54010_e69067 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1513 = assign54010_e69069;

        let (assign54020_e69083, assign54020_e69083_d_n5, assign54020_e69083_d_n6, assign54020_e69083_d_n7, assign54020_e69083_d_n8,) = {
    if (((locals.var_guard1510 != 0.0) && (locals.var_guard1512 != 0.0)) && (locals.var_guard1513 != 0.0)) {
        let assign54020_e69078: f64 = (-locals.var_xginrdep);
        let assign54020_e69079: f64 = (assign54020_e69078).exp();
        let assign54020_e69080: f64 = (1.0 + assign54020_e69079);
        let assign54020_e69081: f64 = (1.0 / assign54020_e69080);
        (assign54020_e69081, (-((assign54020_e69079 * (-locals.var_xginrdep_dn5)) / (assign54020_e69080 * assign54020_e69080))), (-((assign54020_e69079 * (-locals.var_xginrdep_dn6)) / (assign54020_e69080 * assign54020_e69080))), (-((assign54020_e69079 * (-locals.var_xginrdep_dn7)) / (assign54020_e69080 * assign54020_e69080))), (-((assign54020_e69079 * (-locals.var_xginrdep_dn8)) / (assign54020_e69080 * assign54020_e69080))),)
    } else {
        (locals.var_finrdep, locals.var_finrdep_dn5, locals.var_finrdep_dn6, locals.var_finrdep_dn7, locals.var_finrdep_dn8,)
    }
};
        locals.var_finrdep = assign54020_e69083;
        locals.var_finrdep_dn5 = assign54020_e69083_d_n5;
        locals.var_finrdep_dn6 = assign54020_e69083_d_n6;
        locals.var_finrdep_dn7 = assign54020_e69083_d_n7;
        locals.var_finrdep_dn8 = assign54020_e69083_d_n8;

        let assign54030_e69086: f64 = if locals.var_xginrdep < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1514 = assign54030_e69086;

        let (assign54040_e69122, assign54040_e69122_d_n5, assign54040_e69122_d_n6, assign54040_e69122_d_n7, assign54040_e69122_d_n8,) = {
    if ((((locals.var_guard1510 != 0.0) && (locals.var_guard1512 != 0.0)) && (locals.var_guard1513 == 0.0)) && (locals.var_guard1514 != 0.0)) {
        let assign54040_e69098: f64 = (-230.25850929940458);
        let assign54040_e69100: f64 = (assign54040_e69098 + locals.var_xginrdep);
        let assign54040_e69104: f64 = (-230.25850929940458);
        let assign54040_e69106: f64 = (assign54040_e69104 + locals.var_xginrdep);
        let assign54040_e69109: f64 = (-230.25850929940458);
        let assign54040_e69111: f64 = (assign54040_e69109 + locals.var_xginrdep);
        let assign54040_e69113: f64 = (assign54040_e69111 * 0.3333333333333333);
        let assign54040_e69114: f64 = (1.0 + assign54040_e69113);
        let assign54040_e69115: f64 = (assign54040_e69106 * assign54040_e69114);
        let assign54040_e69116: f64 = (0.5 * assign54040_e69115);
        let assign54040_e69117: f64 = (1.0 + assign54040_e69116);
        let assign54040_e69118: f64 = (assign54040_e69100 * assign54040_e69117);
        let assign54040_e69119: f64 = (1.0 + assign54040_e69118);
        let assign54040_e69120: f64 = (1e-100 / assign54040_e69119);
        (assign54040_e69120, (-((1e-100 * ((locals.var_xginrdep_dn5 * assign54040_e69117) + (assign54040_e69100 * (0.5 * ((locals.var_xginrdep_dn5 * assign54040_e69114) + (assign54040_e69106 * (locals.var_xginrdep_dn5 * 0.3333333333333333))))))) / (assign54040_e69119 * assign54040_e69119))), (-((1e-100 * ((locals.var_xginrdep_dn6 * assign54040_e69117) + (assign54040_e69100 * (0.5 * ((locals.var_xginrdep_dn6 * assign54040_e69114) + (assign54040_e69106 * (locals.var_xginrdep_dn6 * 0.3333333333333333))))))) / (assign54040_e69119 * assign54040_e69119))), (-((1e-100 * ((locals.var_xginrdep_dn7 * assign54040_e69117) + (assign54040_e69100 * (0.5 * ((locals.var_xginrdep_dn7 * assign54040_e69114) + (assign54040_e69106 * (locals.var_xginrdep_dn7 * 0.3333333333333333))))))) / (assign54040_e69119 * assign54040_e69119))), (-((1e-100 * ((locals.var_xginrdep_dn8 * assign54040_e69117) + (assign54040_e69100 * (0.5 * ((locals.var_xginrdep_dn8 * assign54040_e69114) + (assign54040_e69106 * (locals.var_xginrdep_dn8 * 0.3333333333333333))))))) / (assign54040_e69119 * assign54040_e69119))),)
    } else {
        (locals.var_finrdep, locals.var_finrdep_dn5, locals.var_finrdep_dn6, locals.var_finrdep_dn7, locals.var_finrdep_dn8,)
    }
};
        locals.var_finrdep = assign54040_e69122;
        locals.var_finrdep_dn5 = assign54040_e69122_d_n5;
        locals.var_finrdep_dn6 = assign54040_e69122_d_n6;
        locals.var_finrdep_dn7 = assign54040_e69122_d_n7;
        locals.var_finrdep_dn8 = assign54040_e69122_d_n8;

        let assign54050_e69125: f64 = if locals.var_xginrdep < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1515 = assign54050_e69125;

        let (assign54060_e69137, assign54060_e69137_d_n5, assign54060_e69137_d_n6, assign54060_e69137_d_n7, assign54060_e69137_d_n8,) = {
    if (((locals.var_guard1510 != 0.0) && (locals.var_guard1512 != 0.0)) && (locals.var_guard1515 != 0.0)) {
        let assign54060_e69133: f64 = (locals.var_xginrdep).exp();
        let assign54060_e69134: f64 = (1.0 + assign54060_e69133);
        let assign54060_e69135: f64 = (assign54060_e69134).ln();
        (assign54060_e69135, ((assign54060_e69133 * locals.var_xginrdep_dn5) / assign54060_e69134), ((assign54060_e69133 * locals.var_xginrdep_dn6) / assign54060_e69134), ((assign54060_e69133 * locals.var_xginrdep_dn7) / assign54060_e69134), ((assign54060_e69133 * locals.var_xginrdep_dn8) / assign54060_e69134),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign54060_e69137;
        locals.var_temp1_dn5 = assign54060_e69137_d_n5;
        locals.var_temp1_dn6 = assign54060_e69137_d_n6;
        locals.var_temp1_dn7 = assign54060_e69137_d_n7;
        locals.var_temp1_dn8 = assign54060_e69137_d_n8;

        let (assign54070_e69146, assign54070_e69146_d_n5, assign54070_e69146_d_n6, assign54070_e69146_d_n7, assign54070_e69146_d_n8,) = {
    if (((locals.var_guard1510 != 0.0) && (locals.var_guard1512 != 0.0)) && (locals.var_guard1515 == 0.0)) {
        (locals.var_xginrdep, locals.var_xginrdep_dn5, locals.var_xginrdep_dn6, locals.var_xginrdep_dn7, locals.var_xginrdep_dn8,)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign54070_e69146;
        locals.var_temp1_dn5 = assign54070_e69146_d_n5;
        locals.var_temp1_dn6 = assign54070_e69146_d_n6;
        locals.var_temp1_dn7 = assign54070_e69146_d_n7;
        locals.var_temp1_dn8 = assign54070_e69146_d_n8;

        let (assign54080_e69154, assign54080_e69154_d_n5, assign54080_e69154_d_n6, assign54080_e69154_d_n7, assign54080_e69154_d_n8,) = {
    if ((locals.var_guard1510 != 0.0) && (locals.var_guard1512 != 0.0)) {
        let assign54080_e69152: f64 = (locals.var_temp__blk936 * locals.var_temp1);
        (assign54080_e69152, ((locals.var_temp__blk936_dn5 * locals.var_temp1) + (locals.var_temp__blk936 * locals.var_temp1_dn5)), ((locals.var_temp__blk936_dn6 * locals.var_temp1) + (locals.var_temp__blk936 * locals.var_temp1_dn6)), ((locals.var_temp__blk936_dn7 * locals.var_temp1) + (locals.var_temp__blk936 * locals.var_temp1_dn7)), ((locals.var_temp__blk936_dn8 * locals.var_temp1) + (locals.var_temp__blk936 * locals.var_temp1_dn8)),)
    } else {
        (locals.var_dvinrdep, locals.var_dvinrdep_dn5, locals.var_dvinrdep_dn6, locals.var_dvinrdep_dn7, locals.var_dvinrdep_dn8,)
    }
};
        locals.var_dvinrdep = assign54080_e69154;
        locals.var_dvinrdep_dn5 = assign54080_e69154_d_n5;
        locals.var_dvinrdep_dn6 = assign54080_e69154_d_n6;
        locals.var_dvinrdep_dn7 = assign54080_e69154_d_n7;
        locals.var_dvinrdep_dn8 = assign54080_e69154_d_n8;

        let (assign54090_e69164, assign54090_e69164_d_n5, assign54090_e69164_d_n6, assign54090_e69164_d_n7, assign54090_e69164_d_n8,) = {
    if (locals.var_guard1510 != 0.0) {
        let assign54090_e69159: f64 = (locals.var_finrdep - locals.var_finracc);
        let assign54090_e69160: f64 = (locals.var_fcinrdep_i * assign54090_e69159);
        let assign54090_e69162: f64 = (assign54090_e69160 + locals.var_finracc);
        (assign54090_e69162, ((locals.var_fcinrdep_i * (locals.var_finrdep_dn5 - locals.var_finracc_dn5)) + locals.var_finracc_dn5), ((locals.var_fcinrdep_i * (locals.var_finrdep_dn6 - locals.var_finracc_dn6)) + locals.var_finracc_dn6), ((locals.var_fcinrdep_i * (locals.var_finrdep_dn7 - locals.var_finracc_dn7)) + locals.var_finracc_dn7), ((locals.var_fcinrdep_i * (locals.var_finrdep_dn8 - locals.var_finracc_dn8)) + locals.var_finracc_dn8),)
    } else {
        (locals.var_finr, locals.var_finr_dn5, locals.var_finr_dn6, locals.var_finr_dn7, locals.var_finr_dn8,)
    }
};
        locals.var_finr = assign54090_e69164;
        locals.var_finr_dn5 = assign54090_e69164_d_n5;
        locals.var_finr_dn6 = assign54090_e69164_d_n6;
        locals.var_finr_dn7 = assign54090_e69164_d_n7;
        locals.var_finr_dn8 = assign54090_e69164_d_n8;

        let (assign54100_e69174, assign54100_e69174_d_n5, assign54100_e69174_d_n6, assign54100_e69174_d_n7, assign54100_e69174_d_n8,) = {
    if (locals.var_guard1510 != 0.0) {
        let assign54100_e69169: f64 = (locals.var_dvinrdep - locals.var_dvinracc);
        let assign54100_e69170: f64 = (locals.var_fcinrdep_i * assign54100_e69169);
        let assign54100_e69172: f64 = (assign54100_e69170 + locals.var_dvinracc);
        (assign54100_e69172, ((locals.var_fcinrdep_i * (locals.var_dvinrdep_dn5 - locals.var_dvinracc_dn5)) + locals.var_dvinracc_dn5), ((locals.var_fcinrdep_i * (locals.var_dvinrdep_dn6 - locals.var_dvinracc_dn6)) + locals.var_dvinracc_dn6), ((locals.var_fcinrdep_i * (locals.var_dvinrdep_dn7 - locals.var_dvinracc_dn7)) + locals.var_dvinracc_dn7), ((locals.var_fcinrdep_i * (locals.var_dvinrdep_dn8 - locals.var_dvinracc_dn8)) + locals.var_dvinracc_dn8),)
    } else {
        (locals.var_dvinr, locals.var_dvinr_dn5, locals.var_dvinr_dn6, locals.var_dvinr_dn7, locals.var_dvinr_dn8,)
    }
};
        locals.var_dvinr = assign54100_e69174;
        locals.var_dvinr_dn5 = assign54100_e69174_d_n5;
        locals.var_dvinr_dn6 = assign54100_e69174_d_n6;
        locals.var_dvinr_dn7 = assign54100_e69174_d_n7;
        locals.var_dvinr_dn8 = assign54100_e69174_d_n8;

        let (assign54110_e69188, assign54110_e69188_d_n5, assign54110_e69188_d_n6, assign54110_e69188_d_n7, assign54110_e69188_d_n8,) = {
    if (locals.var_guard1510 != 0.0) {
        let assign54110_e69179: f64 = (locals.var_phit1_ac * locals.var_xno_s_ac);
        let assign54110_e69180: f64 = (locals.var_vgb1_ac - assign54110_e69179);
        let assign54110_e69182: f64 = (assign54110_e69180 - locals.var_voxm_ac);
        let assign54110_e69185: f64 = (0.5 * locals.var_dps_ac);
        let assign54110_e69186: f64 = (assign54110_e69182 - assign54110_e69185);
        (assign54110_e69186, (((locals.var_vgb1_ac_dn5 - ((locals.var_phit1_ac_dn5 * locals.var_xno_s_ac) + (locals.var_phit1_ac * locals.var_xno_s_ac_dn5))) - locals.var_voxm_ac_dn5) - (0.5 * locals.var_dps_ac_dn5)), (((locals.var_vgb1_ac_dn6 - ((locals.var_phit1_ac_dn6 * locals.var_xno_s_ac) + (locals.var_phit1_ac * locals.var_xno_s_ac_dn6))) - locals.var_voxm_ac_dn6) - (0.5 * locals.var_dps_ac_dn6)), (((locals.var_vgb1_ac_dn7 - ((locals.var_phit1_ac_dn7 * locals.var_xno_s_ac) + (locals.var_phit1_ac * locals.var_xno_s_ac_dn7))) - locals.var_voxm_ac_dn7) - (0.5 * locals.var_dps_ac_dn7)), (((locals.var_vgb1_ac_dn8 - ((locals.var_phit1_ac_dn8 * locals.var_xno_s_ac) + (locals.var_phit1_ac * locals.var_xno_s_ac_dn8))) - locals.var_voxm_ac_dn8) - (0.5 * locals.var_dps_ac_dn8)),)
    } else {
        (locals.var_vgsinr, locals.var_vgsinr_dn5, locals.var_vgsinr_dn6, locals.var_vgsinr_dn7, locals.var_vgsinr_dn8,)
    }
};
        locals.var_vgsinr = assign54110_e69188;
        locals.var_vgsinr_dn5 = assign54110_e69188_d_n5;
        locals.var_vgsinr_dn6 = assign54110_e69188_d_n6;
        locals.var_vgsinr_dn7 = assign54110_e69188_d_n7;
        locals.var_vgsinr_dn8 = assign54110_e69188_d_n8;

        let (assign54120_e69196, assign54120_e69196_d_n5, assign54120_e69196_d_n6, assign54120_e69196_d_n7, assign54120_e69196_d_n8,) = {
    if (locals.var_guard1510 != 0.0) {
        let assign54120_e69192: f64 = (locals.var_vgb1_ac - locals.var_vgsinr);
        let assign54120_e69194: f64 = (assign54120_e69192 - locals.var_qbs_ac);
        (assign54120_e69194, ((locals.var_vgb1_ac_dn5 - locals.var_vgsinr_dn5) - locals.var_qbs_ac_dn5), ((locals.var_vgb1_ac_dn6 - locals.var_vgsinr_dn6) - locals.var_qbs_ac_dn6), ((locals.var_vgb1_ac_dn7 - locals.var_vgsinr_dn7) - locals.var_qbs_ac_dn7), ((locals.var_vgb1_ac_dn8 - locals.var_vgsinr_dn8) - locals.var_qbs_ac_dn8),)
    } else {
        (locals.var_vsginr, locals.var_vsginr_dn5, locals.var_vsginr_dn6, locals.var_vsginr_dn7, locals.var_vsginr_dn8,)
    }
};
        locals.var_vsginr = assign54120_e69196;
        locals.var_vsginr_dn5 = assign54120_e69196_d_n5;
        locals.var_vsginr_dn6 = assign54120_e69196_d_n6;
        locals.var_vsginr_dn7 = assign54120_e69196_d_n7;
        locals.var_vsginr_dn8 = assign54120_e69196_d_n8;

        let (assign54130_e69204, assign54130_e69204_d_n5, assign54130_e69204_d_n6, assign54130_e69204_d_n7, assign54130_e69204_d_n8,) = {
    if (locals.var_guard1510 != 0.0) {
        let assign54130_e69200: f64 = (locals.var_dps_ac + locals.var_vgsinr);
        let assign54130_e69202: f64 = (assign54130_e69200 - locals.var_v_ds);
        (assign54130_e69202, (locals.var_dps_ac_dn5 + locals.var_vgsinr_dn5), ((locals.var_dps_ac_dn6 + locals.var_vgsinr_dn6) - locals.var_v_ds_dn6), ((locals.var_dps_ac_dn7 + locals.var_vgsinr_dn7) - locals.var_v_ds_dn7), (locals.var_dps_ac_dn8 + locals.var_vgsinr_dn8),)
    } else {
        (locals.var_vgdinr, locals.var_vgdinr_dn5, locals.var_vgdinr_dn6, locals.var_vgdinr_dn7, locals.var_vgdinr_dn8,)
    }
};
        locals.var_vgdinr = assign54130_e69204;
        locals.var_vgdinr_dn5 = assign54130_e69204_d_n5;
        locals.var_vgdinr_dn6 = assign54130_e69204_d_n6;
        locals.var_vgdinr_dn7 = assign54130_e69204_d_n7;
        locals.var_vgdinr_dn8 = assign54130_e69204_d_n8;

        let (assign54140_e69212, assign54140_e69212_d_n5, assign54140_e69212_d_n6, assign54140_e69212_d_n7, assign54140_e69212_d_n8,) = {
    if (locals.var_guard1510 != 0.0) {
        let assign54140_e69208: f64 = (locals.var_vgb1_ac - locals.var_vgdinr);
        let assign54140_e69210: f64 = (assign54140_e69208 - locals.var_qbd_ac);
        (assign54140_e69210, ((locals.var_vgb1_ac_dn5 - locals.var_vgdinr_dn5) - locals.var_qbd_ac_dn5), ((locals.var_vgb1_ac_dn6 - locals.var_vgdinr_dn6) - locals.var_qbd_ac_dn6), ((locals.var_vgb1_ac_dn7 - locals.var_vgdinr_dn7) - locals.var_qbd_ac_dn7), ((locals.var_vgb1_ac_dn8 - locals.var_vgdinr_dn8) - locals.var_qbd_ac_dn8),)
    } else {
        (locals.var_vdginr, locals.var_vdginr_dn5, locals.var_vdginr_dn6, locals.var_vdginr_dn7, locals.var_vdginr_dn8,)
    }
};
        locals.var_vdginr = assign54140_e69212;
        locals.var_vdginr_dn5 = assign54140_e69212_d_n5;
        locals.var_vdginr_dn6 = assign54140_e69212_d_n6;
        locals.var_vdginr_dn7 = assign54140_e69212_d_n7;
        locals.var_vdginr_dn8 = assign54140_e69212_d_n8;

        let assign54150_e69215: f64 = if locals.var_sigvds > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1516 = assign54150_e69215;

        let (assign54160_e69229, assign54160_e69229_d_n5, assign54160_e69229_d_n6, assign54160_e69229_d_n7, assign54160_e69229_d_n8,) = {
    if ((locals.var_guard1510 != 0.0) && (locals.var_guard1516 != 0.0)) {
        let assign54160_e69222: f64 = (locals.var_cinrd_i * locals.var_vgdinr);
        let assign54160_e69225: f64 = (locals.var_cinr_i * locals.var_vgsinr);
        let assign54160_e69226: f64 = (assign54160_e69222 + assign54160_e69225);
        let assign54160_e69227: f64 = (locals.var_finr * assign54160_e69226);
        (assign54160_e69227, ((locals.var_finr_dn5 * assign54160_e69226) + (locals.var_finr * ((locals.var_cinrd_i * locals.var_vgdinr_dn5) + (locals.var_cinr_i * locals.var_vgsinr_dn5)))), ((locals.var_finr_dn6 * assign54160_e69226) + (locals.var_finr * ((locals.var_cinrd_i * locals.var_vgdinr_dn6) + (locals.var_cinr_i * locals.var_vgsinr_dn6)))), ((locals.var_finr_dn7 * assign54160_e69226) + (locals.var_finr * ((locals.var_cinrd_i * locals.var_vgdinr_dn7) + (locals.var_cinr_i * locals.var_vgsinr_dn7)))), ((locals.var_finr_dn8 * assign54160_e69226) + (locals.var_finr * ((locals.var_cinrd_i * locals.var_vgdinr_dn8) + (locals.var_cinr_i * locals.var_vgsinr_dn8)))),)
    } else {
        (locals.var_qginr, locals.var_qginr_dn5, locals.var_qginr_dn6, locals.var_qginr_dn7, locals.var_qginr_dn8,)
    }
};
        locals.var_qginr = assign54160_e69229;
        locals.var_qginr_dn5 = assign54160_e69229_d_n5;
        locals.var_qginr_dn6 = assign54160_e69229_d_n6;
        locals.var_qginr_dn7 = assign54160_e69229_d_n7;
        locals.var_qginr_dn8 = assign54160_e69229_d_n8;

        let (assign54170_e69239, assign54170_e69239_d_n5, assign54170_e69239_d_n6, assign54170_e69239_d_n7, assign54170_e69239_d_n8,) = {
    if ((locals.var_guard1510 != 0.0) && (locals.var_guard1516 != 0.0)) {
        let assign54170_e69236: f64 = (locals.var_vsginr - locals.var_dvinr);
        let assign54170_e69237: f64 = (locals.var_cinr_i * assign54170_e69236);
        (assign54170_e69237, (locals.var_cinr_i * (locals.var_vsginr_dn5 - locals.var_dvinr_dn5)), (locals.var_cinr_i * (locals.var_vsginr_dn6 - locals.var_dvinr_dn6)), (locals.var_cinr_i * (locals.var_vsginr_dn7 - locals.var_dvinr_dn7)), (locals.var_cinr_i * (locals.var_vsginr_dn8 - locals.var_dvinr_dn8)),)
    } else {
        (locals.var_qsinr, locals.var_qsinr_dn5, locals.var_qsinr_dn6, locals.var_qsinr_dn7, locals.var_qsinr_dn8,)
    }
};
        locals.var_qsinr = assign54170_e69239;
        locals.var_qsinr_dn5 = assign54170_e69239_d_n5;
        locals.var_qsinr_dn6 = assign54170_e69239_d_n6;
        locals.var_qsinr_dn7 = assign54170_e69239_d_n7;
        locals.var_qsinr_dn8 = assign54170_e69239_d_n8;

        let (assign54180_e69249, assign54180_e69249_d_n5, assign54180_e69249_d_n6, assign54180_e69249_d_n7, assign54180_e69249_d_n8,) = {
    if ((locals.var_guard1510 != 0.0) && (locals.var_guard1516 != 0.0)) {
        let assign54180_e69246: f64 = (locals.var_vdginr - locals.var_dvinr);
        let assign54180_e69247: f64 = (locals.var_cinrd_i * assign54180_e69246);
        (assign54180_e69247, (locals.var_cinrd_i * (locals.var_vdginr_dn5 - locals.var_dvinr_dn5)), (locals.var_cinrd_i * (locals.var_vdginr_dn6 - locals.var_dvinr_dn6)), (locals.var_cinrd_i * (locals.var_vdginr_dn7 - locals.var_dvinr_dn7)), (locals.var_cinrd_i * (locals.var_vdginr_dn8 - locals.var_dvinr_dn8)),)
    } else {
        (locals.var_qdinr, locals.var_qdinr_dn5, locals.var_qdinr_dn6, locals.var_qdinr_dn7, locals.var_qdinr_dn8,)
    }
};
        locals.var_qdinr = assign54180_e69249;
        locals.var_qdinr_dn5 = assign54180_e69249_d_n5;
        locals.var_qdinr_dn6 = assign54180_e69249_d_n6;
        locals.var_qdinr_dn7 = assign54180_e69249_d_n7;
        locals.var_qdinr_dn8 = assign54180_e69249_d_n8;

        let (assign54190_e69264, assign54190_e69264_d_n5, assign54190_e69264_d_n6, assign54190_e69264_d_n7, assign54190_e69264_d_n8,) = {
    if ((locals.var_guard1510 != 0.0) && (locals.var_guard1516 == 0.0)) {
        let assign54190_e69257: f64 = (locals.var_cinr_i * locals.var_vgdinr);
        let assign54190_e69260: f64 = (locals.var_cinrd_i * locals.var_vgsinr);
        let assign54190_e69261: f64 = (assign54190_e69257 + assign54190_e69260);
        let assign54190_e69262: f64 = (locals.var_finr * assign54190_e69261);
        (assign54190_e69262, ((locals.var_finr_dn5 * assign54190_e69261) + (locals.var_finr * ((locals.var_cinr_i * locals.var_vgdinr_dn5) + (locals.var_cinrd_i * locals.var_vgsinr_dn5)))), ((locals.var_finr_dn6 * assign54190_e69261) + (locals.var_finr * ((locals.var_cinr_i * locals.var_vgdinr_dn6) + (locals.var_cinrd_i * locals.var_vgsinr_dn6)))), ((locals.var_finr_dn7 * assign54190_e69261) + (locals.var_finr * ((locals.var_cinr_i * locals.var_vgdinr_dn7) + (locals.var_cinrd_i * locals.var_vgsinr_dn7)))), ((locals.var_finr_dn8 * assign54190_e69261) + (locals.var_finr * ((locals.var_cinr_i * locals.var_vgdinr_dn8) + (locals.var_cinrd_i * locals.var_vgsinr_dn8)))),)
    } else {
        (locals.var_qginr, locals.var_qginr_dn5, locals.var_qginr_dn6, locals.var_qginr_dn7, locals.var_qginr_dn8,)
    }
};
        locals.var_qginr = assign54190_e69264;
        locals.var_qginr_dn5 = assign54190_e69264_d_n5;
        locals.var_qginr_dn6 = assign54190_e69264_d_n6;
        locals.var_qginr_dn7 = assign54190_e69264_d_n7;
        locals.var_qginr_dn8 = assign54190_e69264_d_n8;

        let (assign54200_e69275, assign54200_e69275_d_n5, assign54200_e69275_d_n6, assign54200_e69275_d_n7, assign54200_e69275_d_n8,) = {
    if ((locals.var_guard1510 != 0.0) && (locals.var_guard1516 == 0.0)) {
        let assign54200_e69272: f64 = (locals.var_vsginr - locals.var_dvinr);
        let assign54200_e69273: f64 = (locals.var_cinrd_i * assign54200_e69272);
        (assign54200_e69273, (locals.var_cinrd_i * (locals.var_vsginr_dn5 - locals.var_dvinr_dn5)), (locals.var_cinrd_i * (locals.var_vsginr_dn6 - locals.var_dvinr_dn6)), (locals.var_cinrd_i * (locals.var_vsginr_dn7 - locals.var_dvinr_dn7)), (locals.var_cinrd_i * (locals.var_vsginr_dn8 - locals.var_dvinr_dn8)),)
    } else {
        (locals.var_qsinr, locals.var_qsinr_dn5, locals.var_qsinr_dn6, locals.var_qsinr_dn7, locals.var_qsinr_dn8,)
    }
};
        locals.var_qsinr = assign54200_e69275;
        locals.var_qsinr_dn5 = assign54200_e69275_d_n5;
        locals.var_qsinr_dn6 = assign54200_e69275_d_n6;
        locals.var_qsinr_dn7 = assign54200_e69275_d_n7;
        locals.var_qsinr_dn8 = assign54200_e69275_d_n8;

    }

    pub(super) fn stamp_transient_block_50(
        locals: &mut StampLocals,
    ) {
        let (assign54210_e69286, assign54210_e69286_d_n5, assign54210_e69286_d_n6, assign54210_e69286_d_n7, assign54210_e69286_d_n8,) = {
    if ((locals.var_guard1510 != 0.0) && (locals.var_guard1516 == 0.0)) {
        let assign54210_e69283: f64 = (locals.var_vdginr - locals.var_dvinr);
        let assign54210_e69284: f64 = (locals.var_cinr_i * assign54210_e69283);
        (assign54210_e69284, (locals.var_cinr_i * (locals.var_vdginr_dn5 - locals.var_dvinr_dn5)), (locals.var_cinr_i * (locals.var_vdginr_dn6 - locals.var_dvinr_dn6)), (locals.var_cinr_i * (locals.var_vdginr_dn7 - locals.var_dvinr_dn7)), (locals.var_cinr_i * (locals.var_vdginr_dn8 - locals.var_dvinr_dn8)),)
    } else {
        (locals.var_qdinr, locals.var_qdinr_dn5, locals.var_qdinr_dn6, locals.var_qdinr_dn7, locals.var_qdinr_dn8,)
    }
};
        locals.var_qdinr = assign54210_e69286;
        locals.var_qdinr_dn5 = assign54210_e69286_d_n5;
        locals.var_qdinr_dn6 = assign54210_e69286_d_n6;
        locals.var_qdinr_dn7 = assign54210_e69286_d_n7;
        locals.var_qdinr_dn8 = assign54210_e69286_d_n8;

        let (assign54220_e69292, assign54220_e69292_d_n5, assign54220_e69292_d_n6, assign54220_e69292_d_n7, assign54220_e69292_d_n8,) = {
    if (locals.var_guard1510 != 0.0) {
        let assign54220_e69290: f64 = (locals.var_qg + locals.var_qginr);
        (assign54220_e69290, (locals.var_qg_dn5 + locals.var_qginr_dn5), (locals.var_qg_dn6 + locals.var_qginr_dn6), (locals.var_qg_dn7 + locals.var_qginr_dn7), (locals.var_qg_dn8 + locals.var_qginr_dn8),)
    } else {
        (locals.var_qg, locals.var_qg_dn5, locals.var_qg_dn6, locals.var_qg_dn7, locals.var_qg_dn8,)
    }
};
        locals.var_qg = assign54220_e69292;
        locals.var_qg_dn5 = assign54220_e69292_d_n5;
        locals.var_qg_dn6 = assign54220_e69292_d_n6;
        locals.var_qg_dn7 = assign54220_e69292_d_n7;
        locals.var_qg_dn8 = assign54220_e69292_d_n8;

        let (assign54230_e69298, assign54230_e69298_d_n5, assign54230_e69298_d_n6, assign54230_e69298_d_n7, assign54230_e69298_d_n8,) = {
    if (locals.var_guard1510 != 0.0) {
        let assign54230_e69296: f64 = (locals.var_qd + locals.var_qdinr);
        (assign54230_e69296, (locals.var_qd_dn5 + locals.var_qdinr_dn5), (locals.var_qd_dn6 + locals.var_qdinr_dn6), (locals.var_qd_dn7 + locals.var_qdinr_dn7), (locals.var_qd_dn8 + locals.var_qdinr_dn8),)
    } else {
        (locals.var_qd, locals.var_qd_dn5, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8,)
    }
};
        locals.var_qd = assign54230_e69298;
        locals.var_qd_dn5 = assign54230_e69298_d_n5;
        locals.var_qd_dn6 = assign54230_e69298_d_n6;
        locals.var_qd_dn7 = assign54230_e69298_d_n7;
        locals.var_qd_dn8 = assign54230_e69298_d_n8;

        let (assign54240_e69308, assign54240_e69308_d_n5, assign54240_e69308_d_n6, assign54240_e69308_d_n7, assign54240_e69308_d_n8,) = {
    if (locals.var_guard1510 != 0.0) {
        let assign54240_e69302: f64 = (locals.var_qb - locals.var_qginr);
        let assign54240_e69304: f64 = (assign54240_e69302 - locals.var_qdinr);
        let assign54240_e69306: f64 = (assign54240_e69304 - locals.var_qsinr);
        (assign54240_e69306, (((locals.var_qb_dn5 - locals.var_qginr_dn5) - locals.var_qdinr_dn5) - locals.var_qsinr_dn5), (((locals.var_qb_dn6 - locals.var_qginr_dn6) - locals.var_qdinr_dn6) - locals.var_qsinr_dn6), (((locals.var_qb_dn7 - locals.var_qginr_dn7) - locals.var_qdinr_dn7) - locals.var_qsinr_dn7), (((locals.var_qb_dn8 - locals.var_qginr_dn8) - locals.var_qdinr_dn8) - locals.var_qsinr_dn8),)
    } else {
        (locals.var_qb, locals.var_qb_dn5, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn8,)
    }
};
        locals.var_qb = assign54240_e69308;
        locals.var_qb_dn5 = assign54240_e69308_d_n5;
        locals.var_qb_dn6 = assign54240_e69308_d_n6;
        locals.var_qb_dn7 = assign54240_e69308_d_n7;
        locals.var_qb_dn8 = assign54240_e69308_d_n8;

        locals.var_qg_ov_s = 0.0;
        locals.var_qg_ov_s_dn5 = 0.0;
        locals.var_qg_ov_s_dn6 = 0.0;
        locals.var_qg_ov_s_dn7 = 0.0;
        locals.var_qg_ov_s_dn8 = 0.0;

        locals.var_yb_ov_s = 0.0;
        locals.var_yb_ov_s_dn5 = 0.0;
        locals.var_yb_ov_s_dn6 = 0.0;
        locals.var_yb_ov_s_dn7 = 0.0;
        locals.var_yb_ov_s_dn8 = 0.0;

        let assign54290_e69323: f64 = if ((locals.var_cgov_i > 0.0) && (locals.var_fcgovacc_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1517 = assign54290_e69323;

        let (assign54300_e69333, assign54300_e69333_d_n5, assign54300_e69333_d_n6, assign54300_e69333_d_n7, assign54300_e69333_d_n8,) = {
    if (locals.var_guard1517 != 0.0) {
        let assign54300_e69328: f64 = (0.5 * locals.var_xgb_ov);
        let assign54300_e69330: f64 = (assign54300_e69328 + locals.var_dxgb_ov_s);
        let assign54300_e69331: f64 = (locals.var_cgovaccg_i * assign54300_e69330);
        (assign54300_e69331, (locals.var_cgovaccg_i * (0.5 * locals.var_xgb_ov_dn5)), (locals.var_cgovaccg_i * (0.5 * locals.var_xgb_ov_dn6)), (locals.var_cgovaccg_i * (0.5 * locals.var_xgb_ov_dn7)), (locals.var_cgovaccg_i * (0.5 * locals.var_xgb_ov_dn8)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign54300_e69333;
        locals.var_temp__blk936_dn5 = assign54300_e69333_d_n5;
        locals.var_temp__blk936_dn6 = assign54300_e69333_d_n6;
        locals.var_temp__blk936_dn7 = assign54300_e69333_d_n7;
        locals.var_temp__blk936_dn8 = assign54300_e69333_d_n8;

        let assign54310_e69336: f64 = if locals.var_temp__blk936 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1518 = assign54310_e69336;

        let assign54320_e69339: f64 = (-230.25850929940458);
        let assign54320_e69340: f64 = if locals.var_temp__blk936 > assign54320_e69339 { 1.0 } else { 0.0 };
        locals.var_guard1519 = assign54320_e69340;

        let (assign54330_e69349, assign54330_e69349_d_n5, assign54330_e69349_d_n6, assign54330_e69349_d_n7, assign54330_e69349_d_n8,) = {
    if (((locals.var_guard1517 != 0.0) && (locals.var_guard1518 != 0.0)) && (locals.var_guard1519 != 0.0)) {
        let assign54330_e69347: f64 = (locals.var_temp__blk936).exp();
        (assign54330_e69347, (assign54330_e69347 * locals.var_temp__blk936_dn5), (assign54330_e69347 * locals.var_temp__blk936_dn6), (assign54330_e69347 * locals.var_temp__blk936_dn7), (assign54330_e69347 * locals.var_temp__blk936_dn8),)
    } else {
        (locals.var_yb_ov_s, locals.var_yb_ov_s_dn5, locals.var_yb_ov_s_dn6, locals.var_yb_ov_s_dn7, locals.var_yb_ov_s_dn8,)
    }
};
        locals.var_yb_ov_s = assign54330_e69349;
        locals.var_yb_ov_s_dn5 = assign54330_e69349_d_n5;
        locals.var_yb_ov_s_dn6 = assign54330_e69349_d_n6;
        locals.var_yb_ov_s_dn7 = assign54330_e69349_d_n7;
        locals.var_yb_ov_s_dn8 = assign54330_e69349_d_n8;

        let (assign54340_e69383, assign54340_e69383_d_n5, assign54340_e69383_d_n6, assign54340_e69383_d_n7, assign54340_e69383_d_n8,) = {
    if (((locals.var_guard1517 != 0.0) && (locals.var_guard1518 != 0.0)) && (locals.var_guard1519 == 0.0)) {
        let assign54340_e69359: f64 = (-230.25850929940458);
        let assign54340_e69361: f64 = (assign54340_e69359 - locals.var_temp__blk936);
        let assign54340_e69365: f64 = (-230.25850929940458);
        let assign54340_e69367: f64 = (assign54340_e69365 - locals.var_temp__blk936);
        let assign54340_e69370: f64 = (-230.25850929940458);
        let assign54340_e69372: f64 = (assign54340_e69370 - locals.var_temp__blk936);
        let assign54340_e69374: f64 = (assign54340_e69372 * 0.3333333333333333);
        let assign54340_e69375: f64 = (1.0 + assign54340_e69374);
        let assign54340_e69376: f64 = (assign54340_e69367 * assign54340_e69375);
        let assign54340_e69377: f64 = (0.5 * assign54340_e69376);
        let assign54340_e69378: f64 = (1.0 + assign54340_e69377);
        let assign54340_e69379: f64 = (assign54340_e69361 * assign54340_e69378);
        let assign54340_e69380: f64 = (1.0 + assign54340_e69379);
        let assign54340_e69381: f64 = (1e-100 / assign54340_e69380);
        (assign54340_e69381, (-((1e-100 * (((-locals.var_temp__blk936_dn5) * assign54340_e69378) + (assign54340_e69361 * (0.5 * (((-locals.var_temp__blk936_dn5) * assign54340_e69375) + (assign54340_e69367 * ((-locals.var_temp__blk936_dn5) * 0.3333333333333333))))))) / (assign54340_e69380 * assign54340_e69380))), (-((1e-100 * (((-locals.var_temp__blk936_dn6) * assign54340_e69378) + (assign54340_e69361 * (0.5 * (((-locals.var_temp__blk936_dn6) * assign54340_e69375) + (assign54340_e69367 * ((-locals.var_temp__blk936_dn6) * 0.3333333333333333))))))) / (assign54340_e69380 * assign54340_e69380))), (-((1e-100 * (((-locals.var_temp__blk936_dn7) * assign54340_e69378) + (assign54340_e69361 * (0.5 * (((-locals.var_temp__blk936_dn7) * assign54340_e69375) + (assign54340_e69367 * ((-locals.var_temp__blk936_dn7) * 0.3333333333333333))))))) / (assign54340_e69380 * assign54340_e69380))), (-((1e-100 * (((-locals.var_temp__blk936_dn8) * assign54340_e69378) + (assign54340_e69361 * (0.5 * (((-locals.var_temp__blk936_dn8) * assign54340_e69375) + (assign54340_e69367 * ((-locals.var_temp__blk936_dn8) * 0.3333333333333333))))))) / (assign54340_e69380 * assign54340_e69380))),)
    } else {
        (locals.var_yb_ov_s, locals.var_yb_ov_s_dn5, locals.var_yb_ov_s_dn6, locals.var_yb_ov_s_dn7, locals.var_yb_ov_s_dn8,)
    }
};
        locals.var_yb_ov_s = assign54340_e69383;
        locals.var_yb_ov_s_dn5 = assign54340_e69383_d_n5;
        locals.var_yb_ov_s_dn6 = assign54340_e69383_d_n6;
        locals.var_yb_ov_s_dn7 = assign54340_e69383_d_n7;
        locals.var_yb_ov_s_dn8 = assign54340_e69383_d_n8;

        let assign54350_e69386: f64 = if locals.var_yb_ov_s > 1e-10 { 1.0 } else { 0.0 };
        locals.var_guard1520 = assign54350_e69386;

        let (assign54360_e69397, assign54360_e69397_d_n5, assign54360_e69397_d_n6, assign54360_e69397_d_n7, assign54360_e69397_d_n8,) = {
    if (((locals.var_guard1517 != 0.0) && (locals.var_guard1518 != 0.0)) && (locals.var_guard1520 != 0.0)) {
        let assign54360_e69394: f64 = (1.0 + locals.var_yb_ov_s);
        let assign54360_e69395: f64 = (assign54360_e69394).ln();
        (assign54360_e69395, (locals.var_yb_ov_s_dn5 / assign54360_e69394), (locals.var_yb_ov_s_dn6 / assign54360_e69394), (locals.var_yb_ov_s_dn7 / assign54360_e69394), (locals.var_yb_ov_s_dn8 / assign54360_e69394),)
    } else {
        (locals.var_xgbeff_ov_s, locals.var_xgbeff_ov_s_dn5, locals.var_xgbeff_ov_s_dn6, locals.var_xgbeff_ov_s_dn7, locals.var_xgbeff_ov_s_dn8,)
    }
};
        locals.var_xgbeff_ov_s = assign54360_e69397;
        locals.var_xgbeff_ov_s_dn5 = assign54360_e69397_d_n5;
        locals.var_xgbeff_ov_s_dn6 = assign54360_e69397_d_n6;
        locals.var_xgbeff_ov_s_dn7 = assign54360_e69397_d_n7;
        locals.var_xgbeff_ov_s_dn8 = assign54360_e69397_d_n8;

        let (assign54370_e69416, assign54370_e69416_d_n5, assign54370_e69416_d_n6, assign54370_e69416_d_n7, assign54370_e69416_d_n8,) = {
    if (((locals.var_guard1517 != 0.0) && (locals.var_guard1518 != 0.0)) && (locals.var_guard1520 != 0.0)) {
        let assign54370_e69407: f64 = (1.0 + locals.var_xgbeff_ov_s);
        let assign54370_e69408: f64 = (assign54370_e69407).ln();
        let assign54370_e69411: f64 = (2.0 + locals.var_xgbeff_ov_s);
        let assign54370_e69412: f64 = (assign54370_e69408 / assign54370_e69411);
        let assign54370_e69413: f64 = (1.0 - assign54370_e69412);
        let assign54370_e69414: f64 = (locals.var_xgbeff_ov_s * assign54370_e69413);
        (assign54370_e69414, ((locals.var_xgbeff_ov_s_dn5 * assign54370_e69413) + (locals.var_xgbeff_ov_s * (-((((locals.var_xgbeff_ov_s_dn5 / assign54370_e69407) * assign54370_e69411) - (assign54370_e69408 * locals.var_xgbeff_ov_s_dn5)) / (assign54370_e69411 * assign54370_e69411))))), ((locals.var_xgbeff_ov_s_dn6 * assign54370_e69413) + (locals.var_xgbeff_ov_s * (-((((locals.var_xgbeff_ov_s_dn6 / assign54370_e69407) * assign54370_e69411) - (assign54370_e69408 * locals.var_xgbeff_ov_s_dn6)) / (assign54370_e69411 * assign54370_e69411))))), ((locals.var_xgbeff_ov_s_dn7 * assign54370_e69413) + (locals.var_xgbeff_ov_s * (-((((locals.var_xgbeff_ov_s_dn7 / assign54370_e69407) * assign54370_e69411) - (assign54370_e69408 * locals.var_xgbeff_ov_s_dn7)) / (assign54370_e69411 * assign54370_e69411))))), ((locals.var_xgbeff_ov_s_dn8 * assign54370_e69413) + (locals.var_xgbeff_ov_s * (-((((locals.var_xgbeff_ov_s_dn8 / assign54370_e69407) * assign54370_e69411) - (assign54370_e69408 * locals.var_xgbeff_ov_s_dn8)) / (assign54370_e69411 * assign54370_e69411))))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign54370_e69416;
        locals.var_temp1_dn5 = assign54370_e69416_d_n5;
        locals.var_temp1_dn6 = assign54370_e69416_d_n6;
        locals.var_temp1_dn7 = assign54370_e69416_d_n7;
        locals.var_temp1_dn8 = assign54370_e69416_d_n8;

        let (assign54380_e69425, assign54380_e69425_d_n5, assign54380_e69425_d_n6, assign54380_e69425_d_n7, assign54380_e69425_d_n8,) = {
    if (((locals.var_guard1517 != 0.0) && (locals.var_guard1518 != 0.0)) && (locals.var_guard1520 == 0.0)) {
        (locals.var_yb_ov_s, locals.var_yb_ov_s_dn5, locals.var_yb_ov_s_dn6, locals.var_yb_ov_s_dn7, locals.var_yb_ov_s_dn8,)
    } else {
        (locals.var_xgbeff_ov_s, locals.var_xgbeff_ov_s_dn5, locals.var_xgbeff_ov_s_dn6, locals.var_xgbeff_ov_s_dn7, locals.var_xgbeff_ov_s_dn8,)
    }
};
        locals.var_xgbeff_ov_s = assign54380_e69425;
        locals.var_xgbeff_ov_s_dn5 = assign54380_e69425_d_n5;
        locals.var_xgbeff_ov_s_dn6 = assign54380_e69425_d_n6;
        locals.var_xgbeff_ov_s_dn7 = assign54380_e69425_d_n7;
        locals.var_xgbeff_ov_s_dn8 = assign54380_e69425_d_n8;

        let (assign54390_e69440, assign54390_e69440_d_n5, assign54390_e69440_d_n6, assign54390_e69440_d_n7, assign54390_e69440_d_n8,) = {
    if (((locals.var_guard1517 != 0.0) && (locals.var_guard1518 != 0.0)) && (locals.var_guard1520 == 0.0)) {
        let assign54390_e69434: f64 = (2.0 * locals.var_xgbeff_ov_s);
        let assign54390_e69437: f64 = (2.0 + locals.var_xgbeff_ov_s);
        let assign54390_e69438: f64 = (assign54390_e69434 / assign54390_e69437);
        (assign54390_e69438, ((((2.0 * locals.var_xgbeff_ov_s_dn5) * assign54390_e69437) - (assign54390_e69434 * locals.var_xgbeff_ov_s_dn5)) / (assign54390_e69437 * assign54390_e69437)), ((((2.0 * locals.var_xgbeff_ov_s_dn6) * assign54390_e69437) - (assign54390_e69434 * locals.var_xgbeff_ov_s_dn6)) / (assign54390_e69437 * assign54390_e69437)), ((((2.0 * locals.var_xgbeff_ov_s_dn7) * assign54390_e69437) - (assign54390_e69434 * locals.var_xgbeff_ov_s_dn7)) / (assign54390_e69437 * assign54390_e69437)), ((((2.0 * locals.var_xgbeff_ov_s_dn8) * assign54390_e69437) - (assign54390_e69434 * locals.var_xgbeff_ov_s_dn8)) / (assign54390_e69437 * assign54390_e69437)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign54390_e69440;
        locals.var_temp1_dn5 = assign54390_e69440_d_n5;
        locals.var_temp1_dn6 = assign54390_e69440_d_n6;
        locals.var_temp1_dn7 = assign54390_e69440_d_n7;
        locals.var_temp1_dn8 = assign54390_e69440_d_n8;

        let (assign54400_e69447, assign54400_e69447_d_n5, assign54400_e69447_d_n6, assign54400_e69447_d_n7, assign54400_e69447_d_n8,) = {
    if ((locals.var_guard1517 != 0.0) && (locals.var_guard1518 == 0.0)) {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    } else {
        (locals.var_xgbeff_ov_s, locals.var_xgbeff_ov_s_dn5, locals.var_xgbeff_ov_s_dn6, locals.var_xgbeff_ov_s_dn7, locals.var_xgbeff_ov_s_dn8,)
    }
};
        locals.var_xgbeff_ov_s = assign54400_e69447;
        locals.var_xgbeff_ov_s_dn5 = assign54400_e69447_d_n5;
        locals.var_xgbeff_ov_s_dn6 = assign54400_e69447_d_n6;
        locals.var_xgbeff_ov_s_dn7 = assign54400_e69447_d_n7;
        locals.var_xgbeff_ov_s_dn8 = assign54400_e69447_d_n8;

        let (assign54410_e69465, assign54410_e69465_d_n5, assign54410_e69465_d_n6, assign54410_e69465_d_n7, assign54410_e69465_d_n8,) = {
    if ((locals.var_guard1517 != 0.0) && (locals.var_guard1518 == 0.0)) {
        let assign54410_e69456: f64 = (1.0 + locals.var_xgbeff_ov_s);
        let assign54410_e69457: f64 = (assign54410_e69456).ln();
        let assign54410_e69460: f64 = (2.0 + locals.var_xgbeff_ov_s);
        let assign54410_e69461: f64 = (assign54410_e69457 / assign54410_e69460);
        let assign54410_e69462: f64 = (1.0 - assign54410_e69461);
        let assign54410_e69463: f64 = (locals.var_xgbeff_ov_s * assign54410_e69462);
        (assign54410_e69463, ((locals.var_xgbeff_ov_s_dn5 * assign54410_e69462) + (locals.var_xgbeff_ov_s * (-((((locals.var_xgbeff_ov_s_dn5 / assign54410_e69456) * assign54410_e69460) - (assign54410_e69457 * locals.var_xgbeff_ov_s_dn5)) / (assign54410_e69460 * assign54410_e69460))))), ((locals.var_xgbeff_ov_s_dn6 * assign54410_e69462) + (locals.var_xgbeff_ov_s * (-((((locals.var_xgbeff_ov_s_dn6 / assign54410_e69456) * assign54410_e69460) - (assign54410_e69457 * locals.var_xgbeff_ov_s_dn6)) / (assign54410_e69460 * assign54410_e69460))))), ((locals.var_xgbeff_ov_s_dn7 * assign54410_e69462) + (locals.var_xgbeff_ov_s * (-((((locals.var_xgbeff_ov_s_dn7 / assign54410_e69456) * assign54410_e69460) - (assign54410_e69457 * locals.var_xgbeff_ov_s_dn7)) / (assign54410_e69460 * assign54410_e69460))))), ((locals.var_xgbeff_ov_s_dn8 * assign54410_e69462) + (locals.var_xgbeff_ov_s * (-((((locals.var_xgbeff_ov_s_dn8 / assign54410_e69456) * assign54410_e69460) - (assign54410_e69457 * locals.var_xgbeff_ov_s_dn8)) / (assign54410_e69460 * assign54410_e69460))))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign54410_e69465;
        locals.var_temp1_dn5 = assign54410_e69465_d_n5;
        locals.var_temp1_dn6 = assign54410_e69465_d_n6;
        locals.var_temp1_dn7 = assign54410_e69465_d_n7;
        locals.var_temp1_dn8 = assign54410_e69465_d_n8;

        let (assign54420_e69480, assign54420_e69480_d_n5, assign54420_e69480_d_n6, assign54420_e69480_d_n7, assign54420_e69480_d_n8,) = {
    if (locals.var_guard1517 != 0.0) {
        let assign54420_e69468: f64 = (-2.0);
        let assign54420_e69470: f64 = (assign54420_e69468 * locals.var_fcgovacc_i);
        let assign54420_e69472: f64 = (assign54420_e69470 / locals.var_cgovaccg_i);
        let assign54420_e69474: f64 = (assign54420_e69472 * locals.var_cgov_i);
        let assign54420_e69476: f64 = (assign54420_e69474 * locals.var_phita);
        let assign54420_e69478: f64 = (assign54420_e69476 * locals.var_temp1);
        (assign54420_e69478, (assign54420_e69476 * locals.var_temp1_dn5), (assign54420_e69476 * locals.var_temp1_dn6), (assign54420_e69476 * locals.var_temp1_dn7), (assign54420_e69476 * locals.var_temp1_dn8),)
    } else {
        (locals.var_qg_ov_s, locals.var_qg_ov_s_dn5, locals.var_qg_ov_s_dn6, locals.var_qg_ov_s_dn7, locals.var_qg_ov_s_dn8,)
    }
};
        locals.var_qg_ov_s = assign54420_e69480;
        locals.var_qg_ov_s_dn5 = assign54420_e69480_d_n5;
        locals.var_qg_ov_s_dn6 = assign54420_e69480_d_n6;
        locals.var_qg_ov_s_dn7 = assign54420_e69480_d_n7;
        locals.var_qg_ov_s_dn8 = assign54420_e69480_d_n8;

        locals.var_qg_ov_d = 0.0;
        locals.var_qg_ov_d_dn5 = 0.0;
        locals.var_qg_ov_d_dn6 = 0.0;
        locals.var_qg_ov_d_dn7 = 0.0;
        locals.var_qg_ov_d_dn8 = 0.0;

        locals.var_yb_ov_d = 0.0;
        locals.var_yb_ov_d_dn5 = 0.0;
        locals.var_yb_ov_d_dn6 = 0.0;
        locals.var_yb_ov_d_dn7 = 0.0;
        locals.var_yb_ov_d_dn8 = 0.0;

        let assign54450_e69489: f64 = if ((locals.var_cgovd_i > 0.0) && (locals.var_fcgovaccd_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1521 = assign54450_e69489;

        let (assign54460_e69499, assign54460_e69499_d_n5, assign54460_e69499_d_n6, assign54460_e69499_d_n7, assign54460_e69499_d_n8,) = {
    if (locals.var_guard1521 != 0.0) {
        let assign54460_e69494: f64 = (0.5 * locals.var_xgb_ov);
        let assign54460_e69496: f64 = (assign54460_e69494 + locals.var_dxgb_ov_d);
        let assign54460_e69497: f64 = (locals.var_cgovaccg_i * assign54460_e69496);
        (assign54460_e69497, (locals.var_cgovaccg_i * (0.5 * locals.var_xgb_ov_dn5)), (locals.var_cgovaccg_i * (0.5 * locals.var_xgb_ov_dn6)), (locals.var_cgovaccg_i * (0.5 * locals.var_xgb_ov_dn7)), (locals.var_cgovaccg_i * (0.5 * locals.var_xgb_ov_dn8)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign54460_e69499;
        locals.var_temp__blk936_dn5 = assign54460_e69499_d_n5;
        locals.var_temp__blk936_dn6 = assign54460_e69499_d_n6;
        locals.var_temp__blk936_dn7 = assign54460_e69499_d_n7;
        locals.var_temp__blk936_dn8 = assign54460_e69499_d_n8;

        let assign54470_e69502: f64 = if locals.var_temp__blk936 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1522 = assign54470_e69502;

        let assign54480_e69505: f64 = (-230.25850929940458);
        let assign54480_e69506: f64 = if locals.var_temp__blk936 > assign54480_e69505 { 1.0 } else { 0.0 };
        locals.var_guard1523 = assign54480_e69506;

        let (assign54490_e69515, assign54490_e69515_d_n5, assign54490_e69515_d_n6, assign54490_e69515_d_n7, assign54490_e69515_d_n8,) = {
    if (((locals.var_guard1521 != 0.0) && (locals.var_guard1522 != 0.0)) && (locals.var_guard1523 != 0.0)) {
        let assign54490_e69513: f64 = (locals.var_temp__blk936).exp();
        (assign54490_e69513, (assign54490_e69513 * locals.var_temp__blk936_dn5), (assign54490_e69513 * locals.var_temp__blk936_dn6), (assign54490_e69513 * locals.var_temp__blk936_dn7), (assign54490_e69513 * locals.var_temp__blk936_dn8),)
    } else {
        (locals.var_yb_ov_d, locals.var_yb_ov_d_dn5, locals.var_yb_ov_d_dn6, locals.var_yb_ov_d_dn7, locals.var_yb_ov_d_dn8,)
    }
};
        locals.var_yb_ov_d = assign54490_e69515;
        locals.var_yb_ov_d_dn5 = assign54490_e69515_d_n5;
        locals.var_yb_ov_d_dn6 = assign54490_e69515_d_n6;
        locals.var_yb_ov_d_dn7 = assign54490_e69515_d_n7;
        locals.var_yb_ov_d_dn8 = assign54490_e69515_d_n8;

        let (assign54500_e69549, assign54500_e69549_d_n5, assign54500_e69549_d_n6, assign54500_e69549_d_n7, assign54500_e69549_d_n8,) = {
    if (((locals.var_guard1521 != 0.0) && (locals.var_guard1522 != 0.0)) && (locals.var_guard1523 == 0.0)) {
        let assign54500_e69525: f64 = (-230.25850929940458);
        let assign54500_e69527: f64 = (assign54500_e69525 - locals.var_temp__blk936);
        let assign54500_e69531: f64 = (-230.25850929940458);
        let assign54500_e69533: f64 = (assign54500_e69531 - locals.var_temp__blk936);
        let assign54500_e69536: f64 = (-230.25850929940458);
        let assign54500_e69538: f64 = (assign54500_e69536 - locals.var_temp__blk936);
        let assign54500_e69540: f64 = (assign54500_e69538 * 0.3333333333333333);
        let assign54500_e69541: f64 = (1.0 + assign54500_e69540);
        let assign54500_e69542: f64 = (assign54500_e69533 * assign54500_e69541);
        let assign54500_e69543: f64 = (0.5 * assign54500_e69542);
        let assign54500_e69544: f64 = (1.0 + assign54500_e69543);
        let assign54500_e69545: f64 = (assign54500_e69527 * assign54500_e69544);
        let assign54500_e69546: f64 = (1.0 + assign54500_e69545);
        let assign54500_e69547: f64 = (1e-100 / assign54500_e69546);
        (assign54500_e69547, (-((1e-100 * (((-locals.var_temp__blk936_dn5) * assign54500_e69544) + (assign54500_e69527 * (0.5 * (((-locals.var_temp__blk936_dn5) * assign54500_e69541) + (assign54500_e69533 * ((-locals.var_temp__blk936_dn5) * 0.3333333333333333))))))) / (assign54500_e69546 * assign54500_e69546))), (-((1e-100 * (((-locals.var_temp__blk936_dn6) * assign54500_e69544) + (assign54500_e69527 * (0.5 * (((-locals.var_temp__blk936_dn6) * assign54500_e69541) + (assign54500_e69533 * ((-locals.var_temp__blk936_dn6) * 0.3333333333333333))))))) / (assign54500_e69546 * assign54500_e69546))), (-((1e-100 * (((-locals.var_temp__blk936_dn7) * assign54500_e69544) + (assign54500_e69527 * (0.5 * (((-locals.var_temp__blk936_dn7) * assign54500_e69541) + (assign54500_e69533 * ((-locals.var_temp__blk936_dn7) * 0.3333333333333333))))))) / (assign54500_e69546 * assign54500_e69546))), (-((1e-100 * (((-locals.var_temp__blk936_dn8) * assign54500_e69544) + (assign54500_e69527 * (0.5 * (((-locals.var_temp__blk936_dn8) * assign54500_e69541) + (assign54500_e69533 * ((-locals.var_temp__blk936_dn8) * 0.3333333333333333))))))) / (assign54500_e69546 * assign54500_e69546))),)
    } else {
        (locals.var_yb_ov_d, locals.var_yb_ov_d_dn5, locals.var_yb_ov_d_dn6, locals.var_yb_ov_d_dn7, locals.var_yb_ov_d_dn8,)
    }
};
        locals.var_yb_ov_d = assign54500_e69549;
        locals.var_yb_ov_d_dn5 = assign54500_e69549_d_n5;
        locals.var_yb_ov_d_dn6 = assign54500_e69549_d_n6;
        locals.var_yb_ov_d_dn7 = assign54500_e69549_d_n7;
        locals.var_yb_ov_d_dn8 = assign54500_e69549_d_n8;

        let assign54510_e69552: f64 = if locals.var_yb_ov_d > 1e-10 { 1.0 } else { 0.0 };
        locals.var_guard1524 = assign54510_e69552;

        let (assign54520_e69563, assign54520_e69563_d_n5, assign54520_e69563_d_n6, assign54520_e69563_d_n7, assign54520_e69563_d_n8,) = {
    if (((locals.var_guard1521 != 0.0) && (locals.var_guard1522 != 0.0)) && (locals.var_guard1524 != 0.0)) {
        let assign54520_e69560: f64 = (1.0 + locals.var_yb_ov_d);
        let assign54520_e69561: f64 = (assign54520_e69560).ln();
        (assign54520_e69561, (locals.var_yb_ov_d_dn5 / assign54520_e69560), (locals.var_yb_ov_d_dn6 / assign54520_e69560), (locals.var_yb_ov_d_dn7 / assign54520_e69560), (locals.var_yb_ov_d_dn8 / assign54520_e69560),)
    } else {
        (locals.var_xgbeff_ov_d, locals.var_xgbeff_ov_d_dn5, locals.var_xgbeff_ov_d_dn6, locals.var_xgbeff_ov_d_dn7, locals.var_xgbeff_ov_d_dn8,)
    }
};
        locals.var_xgbeff_ov_d = assign54520_e69563;
        locals.var_xgbeff_ov_d_dn5 = assign54520_e69563_d_n5;
        locals.var_xgbeff_ov_d_dn6 = assign54520_e69563_d_n6;
        locals.var_xgbeff_ov_d_dn7 = assign54520_e69563_d_n7;
        locals.var_xgbeff_ov_d_dn8 = assign54520_e69563_d_n8;

        let (assign54530_e69582, assign54530_e69582_d_n5, assign54530_e69582_d_n6, assign54530_e69582_d_n7, assign54530_e69582_d_n8,) = {
    if (((locals.var_guard1521 != 0.0) && (locals.var_guard1522 != 0.0)) && (locals.var_guard1524 != 0.0)) {
        let assign54530_e69573: f64 = (1.0 + locals.var_xgbeff_ov_d);
        let assign54530_e69574: f64 = (assign54530_e69573).ln();
        let assign54530_e69577: f64 = (2.0 + locals.var_xgbeff_ov_d);
        let assign54530_e69578: f64 = (assign54530_e69574 / assign54530_e69577);
        let assign54530_e69579: f64 = (1.0 - assign54530_e69578);
        let assign54530_e69580: f64 = (locals.var_xgbeff_ov_d * assign54530_e69579);
        (assign54530_e69580, ((locals.var_xgbeff_ov_d_dn5 * assign54530_e69579) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn5 / assign54530_e69573) * assign54530_e69577) - (assign54530_e69574 * locals.var_xgbeff_ov_d_dn5)) / (assign54530_e69577 * assign54530_e69577))))), ((locals.var_xgbeff_ov_d_dn6 * assign54530_e69579) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn6 / assign54530_e69573) * assign54530_e69577) - (assign54530_e69574 * locals.var_xgbeff_ov_d_dn6)) / (assign54530_e69577 * assign54530_e69577))))), ((locals.var_xgbeff_ov_d_dn7 * assign54530_e69579) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn7 / assign54530_e69573) * assign54530_e69577) - (assign54530_e69574 * locals.var_xgbeff_ov_d_dn7)) / (assign54530_e69577 * assign54530_e69577))))), ((locals.var_xgbeff_ov_d_dn8 * assign54530_e69579) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn8 / assign54530_e69573) * assign54530_e69577) - (assign54530_e69574 * locals.var_xgbeff_ov_d_dn8)) / (assign54530_e69577 * assign54530_e69577))))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign54530_e69582;
        locals.var_temp1_dn5 = assign54530_e69582_d_n5;
        locals.var_temp1_dn6 = assign54530_e69582_d_n6;
        locals.var_temp1_dn7 = assign54530_e69582_d_n7;
        locals.var_temp1_dn8 = assign54530_e69582_d_n8;

        let (assign54540_e69591, assign54540_e69591_d_n5, assign54540_e69591_d_n6, assign54540_e69591_d_n7, assign54540_e69591_d_n8,) = {
    if (((locals.var_guard1521 != 0.0) && (locals.var_guard1522 != 0.0)) && (locals.var_guard1524 == 0.0)) {
        (locals.var_yb_ov_d, locals.var_yb_ov_d_dn5, locals.var_yb_ov_d_dn6, locals.var_yb_ov_d_dn7, locals.var_yb_ov_d_dn8,)
    } else {
        (locals.var_xgbeff_ov_d, locals.var_xgbeff_ov_d_dn5, locals.var_xgbeff_ov_d_dn6, locals.var_xgbeff_ov_d_dn7, locals.var_xgbeff_ov_d_dn8,)
    }
};
        locals.var_xgbeff_ov_d = assign54540_e69591;
        locals.var_xgbeff_ov_d_dn5 = assign54540_e69591_d_n5;
        locals.var_xgbeff_ov_d_dn6 = assign54540_e69591_d_n6;
        locals.var_xgbeff_ov_d_dn7 = assign54540_e69591_d_n7;
        locals.var_xgbeff_ov_d_dn8 = assign54540_e69591_d_n8;

        let (assign54550_e69606, assign54550_e69606_d_n5, assign54550_e69606_d_n6, assign54550_e69606_d_n7, assign54550_e69606_d_n8,) = {
    if (((locals.var_guard1521 != 0.0) && (locals.var_guard1522 != 0.0)) && (locals.var_guard1524 == 0.0)) {
        let assign54550_e69600: f64 = (2.0 * locals.var_xgbeff_ov_d);
        let assign54550_e69603: f64 = (2.0 + locals.var_xgbeff_ov_d);
        let assign54550_e69604: f64 = (assign54550_e69600 / assign54550_e69603);
        (assign54550_e69604, ((((2.0 * locals.var_xgbeff_ov_d_dn5) * assign54550_e69603) - (assign54550_e69600 * locals.var_xgbeff_ov_d_dn5)) / (assign54550_e69603 * assign54550_e69603)), ((((2.0 * locals.var_xgbeff_ov_d_dn6) * assign54550_e69603) - (assign54550_e69600 * locals.var_xgbeff_ov_d_dn6)) / (assign54550_e69603 * assign54550_e69603)), ((((2.0 * locals.var_xgbeff_ov_d_dn7) * assign54550_e69603) - (assign54550_e69600 * locals.var_xgbeff_ov_d_dn7)) / (assign54550_e69603 * assign54550_e69603)), ((((2.0 * locals.var_xgbeff_ov_d_dn8) * assign54550_e69603) - (assign54550_e69600 * locals.var_xgbeff_ov_d_dn8)) / (assign54550_e69603 * assign54550_e69603)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign54550_e69606;
        locals.var_temp1_dn5 = assign54550_e69606_d_n5;
        locals.var_temp1_dn6 = assign54550_e69606_d_n6;
        locals.var_temp1_dn7 = assign54550_e69606_d_n7;
        locals.var_temp1_dn8 = assign54550_e69606_d_n8;

        let (assign54560_e69613, assign54560_e69613_d_n5, assign54560_e69613_d_n6, assign54560_e69613_d_n7, assign54560_e69613_d_n8,) = {
    if ((locals.var_guard1521 != 0.0) && (locals.var_guard1522 == 0.0)) {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    } else {
        (locals.var_xgbeff_ov_d, locals.var_xgbeff_ov_d_dn5, locals.var_xgbeff_ov_d_dn6, locals.var_xgbeff_ov_d_dn7, locals.var_xgbeff_ov_d_dn8,)
    }
};
        locals.var_xgbeff_ov_d = assign54560_e69613;
        locals.var_xgbeff_ov_d_dn5 = assign54560_e69613_d_n5;
        locals.var_xgbeff_ov_d_dn6 = assign54560_e69613_d_n6;
        locals.var_xgbeff_ov_d_dn7 = assign54560_e69613_d_n7;
        locals.var_xgbeff_ov_d_dn8 = assign54560_e69613_d_n8;

        let (assign54570_e69631, assign54570_e69631_d_n5, assign54570_e69631_d_n6, assign54570_e69631_d_n7, assign54570_e69631_d_n8,) = {
    if ((locals.var_guard1521 != 0.0) && (locals.var_guard1522 == 0.0)) {
        let assign54570_e69622: f64 = (1.0 + locals.var_xgbeff_ov_d);
        let assign54570_e69623: f64 = (assign54570_e69622).ln();
        let assign54570_e69626: f64 = (2.0 + locals.var_xgbeff_ov_d);
        let assign54570_e69627: f64 = (assign54570_e69623 / assign54570_e69626);
        let assign54570_e69628: f64 = (1.0 - assign54570_e69627);
        let assign54570_e69629: f64 = (locals.var_xgbeff_ov_d * assign54570_e69628);
        (assign54570_e69629, ((locals.var_xgbeff_ov_d_dn5 * assign54570_e69628) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn5 / assign54570_e69622) * assign54570_e69626) - (assign54570_e69623 * locals.var_xgbeff_ov_d_dn5)) / (assign54570_e69626 * assign54570_e69626))))), ((locals.var_xgbeff_ov_d_dn6 * assign54570_e69628) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn6 / assign54570_e69622) * assign54570_e69626) - (assign54570_e69623 * locals.var_xgbeff_ov_d_dn6)) / (assign54570_e69626 * assign54570_e69626))))), ((locals.var_xgbeff_ov_d_dn7 * assign54570_e69628) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn7 / assign54570_e69622) * assign54570_e69626) - (assign54570_e69623 * locals.var_xgbeff_ov_d_dn7)) / (assign54570_e69626 * assign54570_e69626))))), ((locals.var_xgbeff_ov_d_dn8 * assign54570_e69628) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn8 / assign54570_e69622) * assign54570_e69626) - (assign54570_e69623 * locals.var_xgbeff_ov_d_dn8)) / (assign54570_e69626 * assign54570_e69626))))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign54570_e69631;
        locals.var_temp1_dn5 = assign54570_e69631_d_n5;
        locals.var_temp1_dn6 = assign54570_e69631_d_n6;
        locals.var_temp1_dn7 = assign54570_e69631_d_n7;
        locals.var_temp1_dn8 = assign54570_e69631_d_n8;

        let (assign54580_e69646, assign54580_e69646_d_n5, assign54580_e69646_d_n6, assign54580_e69646_d_n7, assign54580_e69646_d_n8,) = {
    if (locals.var_guard1521 != 0.0) {
        let assign54580_e69634: f64 = (-2.0);
        let assign54580_e69636: f64 = (assign54580_e69634 * locals.var_fcgovaccd_i);
        let assign54580_e69638: f64 = (assign54580_e69636 / locals.var_cgovaccg_i);
        let assign54580_e69640: f64 = (assign54580_e69638 * locals.var_cgovd_i);
        let assign54580_e69642: f64 = (assign54580_e69640 * locals.var_phita);
        let assign54580_e69644: f64 = (assign54580_e69642 * locals.var_temp1);
        (assign54580_e69644, (assign54580_e69642 * locals.var_temp1_dn5), (assign54580_e69642 * locals.var_temp1_dn6), (assign54580_e69642 * locals.var_temp1_dn7), (assign54580_e69642 * locals.var_temp1_dn8),)
    } else {
        (locals.var_qg_ov_d, locals.var_qg_ov_d_dn5, locals.var_qg_ov_d_dn6, locals.var_qg_ov_d_dn7, locals.var_qg_ov_d_dn8,)
    }
};
        locals.var_qg_ov_d = assign54580_e69646;
        locals.var_qg_ov_d_dn5 = assign54580_e69646_d_n5;
        locals.var_qg_ov_d_dn6 = assign54580_e69646_d_n6;
        locals.var_qg_ov_d_dn7 = assign54580_e69646_d_n7;
        locals.var_qg_ov_d_dn8 = assign54580_e69646_d_n8;

        let assign54590_e69649: f64 = (locals.var_qg_ov_s + locals.var_qg_ov_d);
        locals.var_qg_ov = assign54590_e69649;
        locals.var_qg_ov_dn5 = (locals.var_qg_ov_s_dn5 + locals.var_qg_ov_d_dn5);
        locals.var_qg_ov_dn6 = (locals.var_qg_ov_s_dn6 + locals.var_qg_ov_d_dn6);
        locals.var_qg_ov_dn7 = (locals.var_qg_ov_s_dn7 + locals.var_qg_ov_d_dn7);
        locals.var_qg_ov_dn8 = (locals.var_qg_ov_s_dn8 + locals.var_qg_ov_d_dn8);

        let assign54600_e69652: f64 = (locals.var_cgbov_i * locals.var_vgb);
        let assign54600_e69654: f64 = (assign54600_e69652 + locals.var_qg_ov);
        locals.var_qgb_ov = assign54600_e69654;
        locals.var_qgb_ov_dn5 = ((locals.var_cgbov_i * locals.var_vgb_dn5) + locals.var_qg_ov_dn5);
        locals.var_qgb_ov_dn6 = ((locals.var_cgbov_i * locals.var_vgb_dn6) + locals.var_qg_ov_dn6);
        locals.var_qgb_ov_dn7 = ((locals.var_cgbov_i * locals.var_vgb_dn7) + locals.var_qg_ov_dn7);
        locals.var_qgb_ov_dn8 = ((locals.var_cgbov_i * locals.var_vgb_dn8) + locals.var_qg_ov_dn8);

        let assign61890_e80509: f64 = if locals.var_sigvds > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1718 = assign61890_e80509;

        let assign61970_e80533: f64 = (locals.var_qg + locals.var_qb);
        let assign61970_e80535: f64 = (assign61970_e80533 + locals.var_qd);
        let assign61970_e80536: f64 = (-assign61970_e80535);
        locals.var_qs = assign61970_e80536;
        locals.var_qs_dn5 = (-((locals.var_qg_dn5 + locals.var_qb_dn5) + locals.var_qd_dn5));
        locals.var_qs_dn6 = (-((locals.var_qg_dn6 + locals.var_qb_dn6) + locals.var_qd_dn6));
        locals.var_qs_dn7 = (-((locals.var_qg_dn7 + locals.var_qb_dn7) + locals.var_qd_dn7));
        locals.var_qs_dn8 = (-((locals.var_qg_dn8 + locals.var_qb_dn8) + locals.var_qd_dn8));

        let assign62020_e80567: f64 = if locals.var_sigvds < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1727 = assign62020_e80567;

        let (assign62030_e80571, assign62030_e80571_d_n5, assign62030_e80571_d_n6, assign62030_e80571_d_n7, assign62030_e80571_d_n8,) = {
    if (locals.var_guard1727 != 0.0) {
        (locals.var_qd, locals.var_qd_dn5, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8,)
    } else {
        (locals.var_temp__blk1726, locals.var_temp__blk1726_dn5, locals.var_temp__blk1726_dn6, locals.var_temp__blk1726_dn7, locals.var_temp__blk1726_dn8,)
    }
};
        locals.var_temp__blk1726 = assign62030_e80571;
        locals.var_temp__blk1726_dn5 = assign62030_e80571_d_n5;
        locals.var_temp__blk1726_dn6 = assign62030_e80571_d_n6;
        locals.var_temp__blk1726_dn7 = assign62030_e80571_d_n7;
        locals.var_temp__blk1726_dn8 = assign62030_e80571_d_n8;

        let (assign62040_e80575, assign62040_e80575_d_n5, assign62040_e80575_d_n6, assign62040_e80575_d_n7, assign62040_e80575_d_n8,) = {
    if (locals.var_guard1727 != 0.0) {
        (locals.var_qs, locals.var_qs_dn5, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8,)
    } else {
        (locals.var_qd, locals.var_qd_dn5, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8,)
    }
};
        locals.var_qd = assign62040_e80575;
        locals.var_qd_dn5 = assign62040_e80575_d_n5;
        locals.var_qd_dn6 = assign62040_e80575_d_n6;
        locals.var_qd_dn7 = assign62040_e80575_d_n7;
        locals.var_qd_dn8 = assign62040_e80575_d_n8;

    }

    pub(super) fn stamp_transient_block_51(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign62050_e80579, assign62050_e80579_d_n5, assign62050_e80579_d_n6, assign62050_e80579_d_n7, assign62050_e80579_d_n8,) = {
    if (locals.var_guard1727 != 0.0) {
        (locals.var_temp__blk1726, locals.var_temp__blk1726_dn5, locals.var_temp__blk1726_dn6, locals.var_temp__blk1726_dn7, locals.var_temp__blk1726_dn8,)
    } else {
        (locals.var_qs, locals.var_qs_dn5, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8,)
    }
};
        locals.var_qs = assign62050_e80579;
        locals.var_qs_dn5 = assign62050_e80579_d_n5;
        locals.var_qs_dn6 = assign62050_e80579_d_n6;
        locals.var_qs_dn7 = assign62050_e80579_d_n7;
        locals.var_qs_dn8 = assign62050_e80579_d_n8;

        locals.var_sidexc = 0.0;
        locals.var_sidexc_dn5 = 0.0;
        locals.var_sidexc_dn6 = 0.0;
        locals.var_sidexc_dn7 = 0.0;
        locals.var_sidexc_dn8 = 0.0;

        locals.var_mid = 0.0;
        locals.var_mid_dn5 = 0.0;
        locals.var_mid_dn6 = 0.0;
        locals.var_mid_dn7 = 0.0;
        locals.var_mid_dn8 = 0.0;

        locals.var_mig = 1e-40;
        locals.var_mig_dn5 = 0.0;
        locals.var_mig_dn6 = 0.0;
        locals.var_mig_dn7 = 0.0;
        locals.var_mig_dn8 = 0.0;

        locals.var_migid = 0.0;
        locals.var_migid_dn5 = 0.0;
        locals.var_migid_dn6 = 0.0;
        locals.var_migid_dn7 = 0.0;
        locals.var_migid_dn8 = 0.0;

        locals.var_c_igid = 0.0;
        locals.var_c_igid_dn5 = 0.0;
        locals.var_c_igid_dn6 = 0.0;
        locals.var_c_igid_dn7 = 0.0;
        locals.var_c_igid_dn8 = 0.0;

        let assign62120_e80588: f64 = (locals.var_cox_qm * locals.var_eta_p_ac);
        locals.var_cgeff = assign62120_e80588;
        locals.var_cgeff_dn5 = ((locals.var_cox_qm_dn5 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn5));
        locals.var_cgeff_dn6 = ((locals.var_cox_qm_dn6 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn6));
        locals.var_cgeff_dn7 = ((locals.var_cox_qm_dn7 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn7));
        locals.var_cgeff_dn8 = ((locals.var_cox_qm_dn8 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn8));

        locals.var_sqid = 0.0;
        locals.var_sqid_dn5 = 0.0;
        locals.var_sqid_dn6 = 0.0;
        locals.var_sqid_dn7 = 0.0;
        locals.var_sqid_dn8 = 0.0;

        locals.var_sqig = 0.0;
        locals.var_sqig_dn5 = 0.0;
        locals.var_sqig_dn6 = 0.0;
        locals.var_sqig_dn7 = 0.0;
        locals.var_sqig_dn8 = 0.0;

        let assign62180_e80600: f64 = if ((locals.var_xg_dc > 0.0) && (locals.var_bet_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1760 = assign62180_e80600;

        let assign62270_e80706: f64 = if p.p32 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1762 = assign62270_e80706;

        let (assign62280_e80714, assign62280_e80714_d_n5, assign62280_e80714_d_n6, assign62280_e80714_d_n7, assign62280_e80714_d_n8,) = {
    if ((locals.var_guard1760 != 0.0) && (locals.var_guard1762 != 0.0)) {
        let assign62280_e80712: f64 = (locals.var_qim1_dc / locals.var_alpha_dc);
        (assign62280_e80712, (((locals.var_qim1_dc_dn5 * locals.var_alpha_dc) - (locals.var_qim1_dc * locals.var_alpha_dc_dn5)) / (locals.var_alpha_dc * locals.var_alpha_dc)), (((locals.var_qim1_dc_dn6 * locals.var_alpha_dc) - (locals.var_qim1_dc * locals.var_alpha_dc_dn6)) / (locals.var_alpha_dc * locals.var_alpha_dc)), (((locals.var_qim1_dc_dn7 * locals.var_alpha_dc) - (locals.var_qim1_dc * locals.var_alpha_dc_dn7)) / (locals.var_alpha_dc * locals.var_alpha_dc)), (((locals.var_qim1_dc_dn8 * locals.var_alpha_dc) - (locals.var_qim1_dc * locals.var_alpha_dc_dn8)) / (locals.var_alpha_dc * locals.var_alpha_dc)),)
    } else {
        (locals.var_h0, locals.var_h0_dn5, locals.var_h0_dn6, locals.var_h0_dn7, locals.var_h0_dn8,)
    }
};
        locals.var_h0 = assign62280_e80714;
        locals.var_h0_dn5 = assign62280_e80714_d_n5;
        locals.var_h0_dn6 = assign62280_e80714_d_n6;
        locals.var_h0_dn7 = assign62280_e80714_d_n7;
        locals.var_h0_dn8 = assign62280_e80714_d_n8;

        let (assign62290_e80722, assign62290_e80722_d_n5, assign62290_e80722_d_n6, assign62290_e80722_d_n7, assign62290_e80722_d_n8,) = {
    if ((locals.var_guard1760 != 0.0) && (locals.var_guard1762 != 0.0)) {
        let assign62290_e80720: f64 = (locals.var_qim_dc / locals.var_qim1_dc);
        (assign62290_e80720, (((locals.var_qim_dc_dn5 * locals.var_qim1_dc) - (locals.var_qim_dc * locals.var_qim1_dc_dn5)) / (locals.var_qim1_dc * locals.var_qim1_dc)), (((locals.var_qim_dc_dn6 * locals.var_qim1_dc) - (locals.var_qim_dc * locals.var_qim1_dc_dn6)) / (locals.var_qim1_dc * locals.var_qim1_dc)), (((locals.var_qim_dc_dn7 * locals.var_qim1_dc) - (locals.var_qim_dc * locals.var_qim1_dc_dn7)) / (locals.var_qim1_dc * locals.var_qim1_dc)), (((locals.var_qim_dc_dn8 * locals.var_qim1_dc) - (locals.var_qim_dc * locals.var_qim1_dc_dn8)) / (locals.var_qim1_dc * locals.var_qim1_dc)),)
    } else {
        (locals.var_t1, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8,)
    }
};
        locals.var_t1 = assign62290_e80722;
        locals.var_t1_dn5 = assign62290_e80722_d_n5;
        locals.var_t1_dn6 = assign62290_e80722_d_n6;
        locals.var_t1_dn7 = assign62290_e80722_d_n7;
        locals.var_t1_dn8 = assign62290_e80722_d_n8;

        let (assign62300_e80734, assign62300_e80734_d_n5, assign62300_e80734_d_n6, assign62300_e80734_d_n7, assign62300_e80734_d_n8,) = {
    if ((locals.var_guard1760 != 0.0) && (locals.var_guard1762 != 0.0)) {
        let assign62300_e80728: f64 = (0.5 * 0.16666666666666666);
        let assign62300_e80731: f64 = (locals.var_dps_dc / locals.var_h0);
        let assign62300_e80732: f64 = (assign62300_e80728 * assign62300_e80731);
        (assign62300_e80732, (assign62300_e80728 * (((locals.var_dps_dc_dn5 * locals.var_h0) - (locals.var_dps_dc * locals.var_h0_dn5)) / (locals.var_h0 * locals.var_h0))), (assign62300_e80728 * (((locals.var_dps_dc_dn6 * locals.var_h0) - (locals.var_dps_dc * locals.var_h0_dn6)) / (locals.var_h0 * locals.var_h0))), (assign62300_e80728 * (((locals.var_dps_dc_dn7 * locals.var_h0) - (locals.var_dps_dc * locals.var_h0_dn7)) / (locals.var_h0 * locals.var_h0))), (assign62300_e80728 * (((locals.var_dps_dc_dn8 * locals.var_h0) - (locals.var_dps_dc * locals.var_h0_dn8)) / (locals.var_h0 * locals.var_h0))),)
    } else {
        (locals.var_sqt2, locals.var_sqt2_dn5, locals.var_sqt2_dn6, locals.var_sqt2_dn7, locals.var_sqt2_dn8,)
    }
};
        locals.var_sqt2 = assign62300_e80734;
        locals.var_sqt2_dn5 = assign62300_e80734_d_n5;
        locals.var_sqt2_dn6 = assign62300_e80734_d_n6;
        locals.var_sqt2_dn7 = assign62300_e80734_d_n7;
        locals.var_sqt2_dn8 = assign62300_e80734_d_n8;

        let (assign62310_e80742, assign62310_e80742_d_n5, assign62310_e80742_d_n6, assign62310_e80742_d_n7, assign62310_e80742_d_n8,) = {
    if ((locals.var_guard1760 != 0.0) && (locals.var_guard1762 != 0.0)) {
        let assign62310_e80740: f64 = (locals.var_sqt2 * locals.var_sqt2);
        (assign62310_e80740, ((locals.var_sqt2_dn5 * locals.var_sqt2) + (locals.var_sqt2 * locals.var_sqt2_dn5)), ((locals.var_sqt2_dn6 * locals.var_sqt2) + (locals.var_sqt2 * locals.var_sqt2_dn6)), ((locals.var_sqt2_dn7 * locals.var_sqt2) + (locals.var_sqt2 * locals.var_sqt2_dn7)), ((locals.var_sqt2_dn8 * locals.var_sqt2) + (locals.var_sqt2 * locals.var_sqt2_dn8)),)
    } else {
        (locals.var_t2, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8,)
    }
};
        locals.var_t2 = assign62310_e80742;
        locals.var_t2_dn5 = assign62310_e80742_d_n5;
        locals.var_t2_dn6 = assign62310_e80742_d_n6;
        locals.var_t2_dn7 = assign62310_e80742_d_n7;
        locals.var_t2_dn8 = assign62310_e80742_d_n8;

        let (assign62320_e80752, assign62320_e80752_d_n5, assign62320_e80752_d_n6, assign62320_e80752_d_n7, assign62320_e80752_d_n8,) = {
    if ((locals.var_guard1760 != 0.0) && (locals.var_guard1762 != 0.0)) {
        let assign62320_e80748: f64 = (locals.var_h0 / locals.var_h_dc);
        let assign62320_e80750: f64 = (assign62320_e80748 - 1.0);
        (assign62320_e80750, (((locals.var_h0_dn5 * locals.var_h_dc) - (locals.var_h0 * locals.var_h_dc_dn5)) / (locals.var_h_dc * locals.var_h_dc)), (((locals.var_h0_dn6 * locals.var_h_dc) - (locals.var_h0 * locals.var_h_dc_dn6)) / (locals.var_h_dc * locals.var_h_dc)), (((locals.var_h0_dn7 * locals.var_h_dc) - (locals.var_h0 * locals.var_h_dc_dn7)) / (locals.var_h_dc * locals.var_h_dc)), (((locals.var_h0_dn8 * locals.var_h_dc) - (locals.var_h0 * locals.var_h_dc_dn8)) / (locals.var_h_dc * locals.var_h_dc)),)
    } else {
        (locals.var_r, locals.var_r_dn5, locals.var_r_dn6, locals.var_r_dn7, locals.var_r_dn8,)
    }
};
        locals.var_r = assign62320_e80752;
        locals.var_r_dn5 = assign62320_e80752_d_n5;
        locals.var_r_dn6 = assign62320_e80752_d_n6;
        locals.var_r_dn7 = assign62320_e80752_d_n7;
        locals.var_r_dn8 = assign62320_e80752_d_n8;

        let (assign62330_e80775, assign62330_e80775_d_n5, assign62330_e80775_d_n6, assign62330_e80775_d_n7, assign62330_e80775_d_n8,) = {
    if ((locals.var_guard1760 != 0.0) && (locals.var_guard1762 != 0.0)) {
        let assign62330_e80760: f64 = (locals.var_r * locals.var_t2);
        let assign62330_e80761: f64 = (12.0 * assign62330_e80760);
        let assign62330_e80762: f64 = (1.0 - assign62330_e80761);
        let (assign62330_e80773, assign62330_e80773_d_n5, assign62330_e80773_d_n6, assign62330_e80773_d_n7, assign62330_e80773_d_n8,) = {
            if (assign62330_e80762 > 1e-20) {
                let assign62330_e80769: f64 = (locals.var_r * locals.var_t2);
                let assign62330_e80770: f64 = (12.0 * assign62330_e80769);
                let assign62330_e80771: f64 = (1.0 - assign62330_e80770);
                (assign62330_e80771, (-(12.0 * ((locals.var_r_dn5 * locals.var_t2) + (locals.var_r * locals.var_t2_dn5)))), (-(12.0 * ((locals.var_r_dn6 * locals.var_t2) + (locals.var_r * locals.var_t2_dn6)))), (-(12.0 * ((locals.var_r_dn7 * locals.var_t2) + (locals.var_r * locals.var_t2_dn7)))), (-(12.0 * ((locals.var_r_dn8 * locals.var_t2) + (locals.var_r * locals.var_t2_dn8)))),)
            } else {
                (1e-20, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign62330_e80773, assign62330_e80773_d_n5, assign62330_e80773_d_n6, assign62330_e80773_d_n7, assign62330_e80773_d_n8,)
    } else {
        (locals.var_lc, locals.var_lc_dn5, locals.var_lc_dn6, locals.var_lc_dn7, locals.var_lc_dn8,)
    }
};
        locals.var_lc = assign62330_e80775;
        locals.var_lc_dn5 = assign62330_e80775_d_n5;
        locals.var_lc_dn6 = assign62330_e80775_d_n6;
        locals.var_lc_dn7 = assign62330_e80775_d_n7;
        locals.var_lc_dn8 = assign62330_e80775_d_n8;

        let (assign62340_e80785, assign62340_e80785_d_n5, assign62340_e80785_d_n6, assign62340_e80785_d_n7, assign62340_e80785_d_n8,) = {
    if ((locals.var_guard1760 != 0.0) && (locals.var_guard1762 != 0.0)) {
        let assign62340_e80782: f64 = (locals.var_lc * locals.var_lc);
        let assign62340_e80783: f64 = (1.0 / assign62340_e80782);
        (assign62340_e80783, (-(((locals.var_lc_dn5 * locals.var_lc) + (locals.var_lc * locals.var_lc_dn5)) / (assign62340_e80782 * assign62340_e80782))), (-(((locals.var_lc_dn6 * locals.var_lc) + (locals.var_lc * locals.var_lc_dn6)) / (assign62340_e80782 * assign62340_e80782))), (-(((locals.var_lc_dn7 * locals.var_lc) + (locals.var_lc * locals.var_lc_dn7)) / (assign62340_e80782 * assign62340_e80782))), (-(((locals.var_lc_dn8 * locals.var_lc) + (locals.var_lc * locals.var_lc_dn8)) / (assign62340_e80782 * assign62340_e80782))),)
    } else {
        (locals.var_lcinv2, locals.var_lcinv2_dn5, locals.var_lcinv2_dn6, locals.var_lcinv2_dn7, locals.var_lcinv2_dn8,)
    }
};
        locals.var_lcinv2 = assign62340_e80785;
        locals.var_lcinv2_dn5 = assign62340_e80785_d_n5;
        locals.var_lcinv2_dn6 = assign62340_e80785_d_n6;
        locals.var_lcinv2_dn7 = assign62340_e80785_d_n7;
        locals.var_lcinv2_dn8 = assign62340_e80785_d_n8;

        let (assign62350_e80795, assign62350_e80795_d_n5, assign62350_e80795_d_n6, assign62350_e80795_d_n7, assign62350_e80795_d_n8,) = {
    if ((locals.var_guard1760 != 0.0) && (locals.var_guard1762 != 0.0)) {
        let assign62350_e80791: f64 = (locals.var_bet_i * locals.var_qim1_dc);
        let assign62350_e80793: f64 = (assign62350_e80791 * locals.var_gvsatinv_dc);
        (assign62350_e80793, (((locals.var_bet_i * locals.var_qim1_dc_dn5) * locals.var_gvsatinv_dc) + (assign62350_e80791 * locals.var_gvsatinv_dc_dn5)), (((locals.var_bet_i * locals.var_qim1_dc_dn6) * locals.var_gvsatinv_dc) + (assign62350_e80791 * locals.var_gvsatinv_dc_dn6)), (((locals.var_bet_i * locals.var_qim1_dc_dn7) * locals.var_gvsatinv_dc) + (assign62350_e80791 * locals.var_gvsatinv_dc_dn7)), (((locals.var_bet_i * locals.var_qim1_dc_dn8) * locals.var_gvsatinv_dc) + (assign62350_e80791 * locals.var_gvsatinv_dc_dn8)),)
    } else {
        (locals.var_g_ideal, locals.var_g_ideal_dn5, locals.var_g_ideal_dn6, locals.var_g_ideal_dn7, locals.var_g_ideal_dn8,)
    }
};
        locals.var_g_ideal = assign62350_e80795;
        locals.var_g_ideal_dn5 = assign62350_e80795_d_n5;
        locals.var_g_ideal_dn6 = assign62350_e80795_d_n6;
        locals.var_g_ideal_dn7 = assign62350_e80795_d_n7;
        locals.var_g_ideal_dn8 = assign62350_e80795_d_n8;

        let (assign62360_e80815, assign62360_e80815_d_n5, assign62360_e80815_d_n6, assign62360_e80815_d_n7, assign62360_e80815_d_n8,) = {
    if ((locals.var_guard1760 != 0.0) && (locals.var_guard1762 != 0.0)) {
        let assign62360_e80802: f64 = (12.0 * locals.var_t2);
        let assign62360_e80803: f64 = (locals.var_t1 + assign62360_e80802);
        let assign62360_e80807: f64 = (1.0 + locals.var_t1);
        let assign62360_e80809: f64 = (assign62360_e80807 * locals.var_t2);
        let assign62360_e80811: f64 = (assign62360_e80809 * locals.var_r);
        let assign62360_e80812: f64 = (24.0 * assign62360_e80811);
        let assign62360_e80813: f64 = (assign62360_e80803 - assign62360_e80812);
        (assign62360_e80813, ((locals.var_t1_dn5 + (12.0 * locals.var_t2_dn5)) - (24.0 * ((((locals.var_t1_dn5 * locals.var_t2) + (assign62360_e80807 * locals.var_t2_dn5)) * locals.var_r) + (assign62360_e80809 * locals.var_r_dn5)))), ((locals.var_t1_dn6 + (12.0 * locals.var_t2_dn6)) - (24.0 * ((((locals.var_t1_dn6 * locals.var_t2) + (assign62360_e80807 * locals.var_t2_dn6)) * locals.var_r) + (assign62360_e80809 * locals.var_r_dn6)))), ((locals.var_t1_dn7 + (12.0 * locals.var_t2_dn7)) - (24.0 * ((((locals.var_t1_dn7 * locals.var_t2) + (assign62360_e80807 * locals.var_t2_dn7)) * locals.var_r) + (assign62360_e80809 * locals.var_r_dn7)))), ((locals.var_t1_dn8 + (12.0 * locals.var_t2_dn8)) - (24.0 * ((((locals.var_t1_dn8 * locals.var_t2) + (assign62360_e80807 * locals.var_t2_dn8)) * locals.var_r) + (assign62360_e80809 * locals.var_r_dn8)))),)
    } else {
        (locals.var_mid, locals.var_mid_dn5, locals.var_mid_dn6, locals.var_mid_dn7, locals.var_mid_dn8,)
    }
};
        locals.var_mid = assign62360_e80815;
        locals.var_mid_dn5 = assign62360_e80815_d_n5;
        locals.var_mid_dn6 = assign62360_e80815_d_n6;
        locals.var_mid_dn7 = assign62360_e80815_d_n7;
        locals.var_mid_dn8 = assign62360_e80815_d_n8;

        let (assign62370_e80826, assign62370_e80826_d_n5, assign62370_e80826_d_n6, assign62370_e80826_d_n7, assign62370_e80826_d_n8,) = {
    if ((locals.var_guard1760 != 0.0) && (locals.var_guard1762 != 0.0)) {
        let (assign62370_e80824, assign62370_e80824_d_n5, assign62370_e80824_d_n6, assign62370_e80824_d_n7, assign62370_e80824_d_n8,) = {
            if (locals.var_mid > 1e-40) {
                (locals.var_mid, locals.var_mid_dn5, locals.var_mid_dn6, locals.var_mid_dn7, locals.var_mid_dn8,)
            } else {
                (1e-40, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign62370_e80824, assign62370_e80824_d_n5, assign62370_e80824_d_n6, assign62370_e80824_d_n7, assign62370_e80824_d_n8,)
    } else {
        (locals.var_mid, locals.var_mid_dn5, locals.var_mid_dn6, locals.var_mid_dn7, locals.var_mid_dn8,)
    }
};
        locals.var_mid = assign62370_e80826;
        locals.var_mid_dn5 = assign62370_e80826_d_n5;
        locals.var_mid_dn6 = assign62370_e80826_d_n6;
        locals.var_mid_dn7 = assign62370_e80826_d_n7;
        locals.var_mid_dn8 = assign62370_e80826_d_n8;

        let (assign62380_e80836, assign62380_e80836_d_n5, assign62380_e80836_d_n6, assign62380_e80836_d_n7, assign62380_e80836_d_n8,) = {
    if ((locals.var_guard1760 != 0.0) && (locals.var_guard1762 != 0.0)) {
        let assign62380_e80832: f64 = (locals.var_g_ideal * locals.var_lcinv2);
        let assign62380_e80834: f64 = (assign62380_e80832 * locals.var_mid);
        (assign62380_e80834, ((((locals.var_g_ideal_dn5 * locals.var_lcinv2) + (locals.var_g_ideal * locals.var_lcinv2_dn5)) * locals.var_mid) + (assign62380_e80832 * locals.var_mid_dn5)), ((((locals.var_g_ideal_dn6 * locals.var_lcinv2) + (locals.var_g_ideal * locals.var_lcinv2_dn6)) * locals.var_mid) + (assign62380_e80832 * locals.var_mid_dn6)), ((((locals.var_g_ideal_dn7 * locals.var_lcinv2) + (locals.var_g_ideal * locals.var_lcinv2_dn7)) * locals.var_mid) + (assign62380_e80832 * locals.var_mid_dn7)), ((((locals.var_g_ideal_dn8 * locals.var_lcinv2) + (locals.var_g_ideal * locals.var_lcinv2_dn8)) * locals.var_mid) + (assign62380_e80832 * locals.var_mid_dn8)),)
    } else {
        (locals.var_mid, locals.var_mid_dn5, locals.var_mid_dn6, locals.var_mid_dn7, locals.var_mid_dn8,)
    }
};
        locals.var_mid = assign62380_e80836;
        locals.var_mid_dn5 = assign62380_e80836_d_n5;
        locals.var_mid_dn6 = assign62380_e80836_d_n6;
        locals.var_mid_dn7 = assign62380_e80836_d_n7;
        locals.var_mid_dn8 = assign62380_e80836_d_n8;

        let assign62390_e80839: f64 = if locals.var_fntexc_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1763 = assign62390_e80839;

        let (assign62400_e80849, assign62400_e80849_d_n5, assign62400_e80849_d_n6, assign62400_e80849_d_n7, assign62400_e80849_d_n8,) = {
    if (((locals.var_guard1760 != 0.0) && (locals.var_guard1762 != 0.0)) && (locals.var_guard1763 != 0.0)) {
        let assign62400_e80847: f64 = (locals.var_thesateff_dc / locals.var_gmob_dc);
        (assign62400_e80847, (((locals.var_thesateff_dc_dn5 * locals.var_gmob_dc) - (locals.var_thesateff_dc * locals.var_gmob_dc_dn5)) / (locals.var_gmob_dc * locals.var_gmob_dc)), (((locals.var_thesateff_dc_dn6 * locals.var_gmob_dc) - (locals.var_thesateff_dc * locals.var_gmob_dc_dn6)) / (locals.var_gmob_dc * locals.var_gmob_dc)), (((locals.var_thesateff_dc_dn7 * locals.var_gmob_dc) - (locals.var_thesateff_dc * locals.var_gmob_dc_dn7)) / (locals.var_gmob_dc * locals.var_gmob_dc)), (((locals.var_thesateff_dc_dn8 * locals.var_gmob_dc) - (locals.var_thesateff_dc * locals.var_gmob_dc_dn8)) / (locals.var_gmob_dc * locals.var_gmob_dc)),)
    } else {
        (locals.var_thesat1_exc, locals.var_thesat1_exc_dn5, locals.var_thesat1_exc_dn6, locals.var_thesat1_exc_dn7, locals.var_thesat1_exc_dn8,)
    }
};
        locals.var_thesat1_exc = assign62400_e80849;
        locals.var_thesat1_exc_dn5 = assign62400_e80849_d_n5;
        locals.var_thesat1_exc_dn6 = assign62400_e80849_d_n6;
        locals.var_thesat1_exc_dn7 = assign62400_e80849_d_n7;
        locals.var_thesat1_exc_dn8 = assign62400_e80849_d_n8;

        let (assign62410_e80863, assign62410_e80863_d_n5, assign62410_e80863_d_n6, assign62410_e80863_d_n7, assign62410_e80863_d_n8,) = {
    if (((locals.var_guard1760 != 0.0) && (locals.var_guard1762 != 0.0)) && (locals.var_guard1763 != 0.0)) {
        let assign62410_e80857: f64 = (locals.var_thesat1_exc * locals.var_thesat1_exc);
        let assign62410_e80859: f64 = (assign62410_e80857 * locals.var_dps_dc);
        let assign62410_e80861: f64 = (assign62410_e80859 * locals.var_dps_dc);
        (assign62410_e80861, ((((((locals.var_thesat1_exc_dn5 * locals.var_thesat1_exc) + (locals.var_thesat1_exc * locals.var_thesat1_exc_dn5)) * locals.var_dps_dc) + (assign62410_e80857 * locals.var_dps_dc_dn5)) * locals.var_dps_dc) + (assign62410_e80859 * locals.var_dps_dc_dn5)), ((((((locals.var_thesat1_exc_dn6 * locals.var_thesat1_exc) + (locals.var_thesat1_exc * locals.var_thesat1_exc_dn6)) * locals.var_dps_dc) + (assign62410_e80857 * locals.var_dps_dc_dn6)) * locals.var_dps_dc) + (assign62410_e80859 * locals.var_dps_dc_dn6)), ((((((locals.var_thesat1_exc_dn7 * locals.var_thesat1_exc) + (locals.var_thesat1_exc * locals.var_thesat1_exc_dn7)) * locals.var_dps_dc) + (assign62410_e80857 * locals.var_dps_dc_dn7)) * locals.var_dps_dc) + (assign62410_e80859 * locals.var_dps_dc_dn7)), ((((((locals.var_thesat1_exc_dn8 * locals.var_thesat1_exc) + (locals.var_thesat1_exc * locals.var_thesat1_exc_dn8)) * locals.var_dps_dc) + (assign62410_e80857 * locals.var_dps_dc_dn8)) * locals.var_dps_dc) + (assign62410_e80859 * locals.var_dps_dc_dn8)),)
    } else {
        (locals.var_zsat_exc, locals.var_zsat_exc_dn5, locals.var_zsat_exc_dn6, locals.var_zsat_exc_dn7, locals.var_zsat_exc_dn8,)
    }
};
        locals.var_zsat_exc = assign62410_e80863;
        locals.var_zsat_exc_dn5 = assign62410_e80863_d_n5;
        locals.var_zsat_exc_dn6 = assign62410_e80863_d_n6;
        locals.var_zsat_exc_dn7 = assign62410_e80863_d_n7;
        locals.var_zsat_exc_dn8 = assign62410_e80863_d_n8;

        let assign62420_e80866: f64 = (-1.0);
        let assign62420_e80867: f64 = if locals.var_chnl_type == assign62420_e80866 { 1.0 } else { 0.0 };
        locals.var_guard1764 = assign62420_e80867;

        let (assign62430_e80883, assign62430_e80883_d_n5, assign62430_e80883_d_n6, assign62430_e80883_d_n7, assign62430_e80883_d_n8,) = {
    if ((((locals.var_guard1760 != 0.0) && (locals.var_guard1762 != 0.0)) && (locals.var_guard1763 != 0.0)) && (locals.var_guard1764 != 0.0)) {
        let assign62430_e80879: f64 = (locals.var_thesat1_exc * locals.var_dps_dc);
        let assign62430_e80880: f64 = (1.0 + assign62430_e80879);
        let assign62430_e80881: f64 = (locals.var_zsat_exc / assign62430_e80880);
        (assign62430_e80881, (((locals.var_zsat_exc_dn5 * assign62430_e80880) - (locals.var_zsat_exc * ((locals.var_thesat1_exc_dn5 * locals.var_dps_dc) + (locals.var_thesat1_exc * locals.var_dps_dc_dn5)))) / (assign62430_e80880 * assign62430_e80880)), (((locals.var_zsat_exc_dn6 * assign62430_e80880) - (locals.var_zsat_exc * ((locals.var_thesat1_exc_dn6 * locals.var_dps_dc) + (locals.var_thesat1_exc * locals.var_dps_dc_dn6)))) / (assign62430_e80880 * assign62430_e80880)), (((locals.var_zsat_exc_dn7 * assign62430_e80880) - (locals.var_zsat_exc * ((locals.var_thesat1_exc_dn7 * locals.var_dps_dc) + (locals.var_thesat1_exc * locals.var_dps_dc_dn7)))) / (assign62430_e80880 * assign62430_e80880)), (((locals.var_zsat_exc_dn8 * assign62430_e80880) - (locals.var_zsat_exc * ((locals.var_thesat1_exc_dn8 * locals.var_dps_dc) + (locals.var_thesat1_exc * locals.var_dps_dc_dn8)))) / (assign62430_e80880 * assign62430_e80880)),)
    } else {
        (locals.var_zsat_exc, locals.var_zsat_exc_dn5, locals.var_zsat_exc_dn6, locals.var_zsat_exc_dn7, locals.var_zsat_exc_dn8,)
    }
};
        locals.var_zsat_exc = assign62430_e80883;
        locals.var_zsat_exc_dn5 = assign62430_e80883_d_n5;
        locals.var_zsat_exc_dn6 = assign62430_e80883_d_n6;
        locals.var_zsat_exc_dn7 = assign62430_e80883_d_n7;
        locals.var_zsat_exc_dn8 = assign62430_e80883_d_n8;

        let (assign62440_e80902, assign62440_e80902_d_n5, assign62440_e80902_d_n6, assign62440_e80902_d_n7, assign62440_e80902_d_n8,) = {
    if (((locals.var_guard1760 != 0.0) && (locals.var_guard1762 != 0.0)) && (locals.var_guard1763 != 0.0)) {
        let assign62440_e80895: f64 = (2.0 * locals.var_zsat_exc);
        let assign62440_e80896: f64 = (1.0 + assign62440_e80895);
        let assign62440_e80897: f64 = (assign62440_e80896).sqrt();
        let assign62440_e80898: f64 = (1.0 + assign62440_e80897);
        let assign62440_e80899: f64 = (locals.var_gmob_dc * assign62440_e80898);
        let assign62440_e80900: f64 = (0.5 * assign62440_e80899);
        (assign62440_e80900, (0.5 * ((locals.var_gmob_dc_dn5 * assign62440_e80898) + (locals.var_gmob_dc * ((2.0 * locals.var_zsat_exc_dn5) / (2.0 * assign62440_e80897))))), (0.5 * ((locals.var_gmob_dc_dn6 * assign62440_e80898) + (locals.var_gmob_dc * ((2.0 * locals.var_zsat_exc_dn6) / (2.0 * assign62440_e80897))))), (0.5 * ((locals.var_gmob_dc_dn7 * assign62440_e80898) + (locals.var_gmob_dc * ((2.0 * locals.var_zsat_exc_dn7) / (2.0 * assign62440_e80897))))), (0.5 * ((locals.var_gmob_dc_dn8 * assign62440_e80898) + (locals.var_gmob_dc * ((2.0 * locals.var_zsat_exc_dn8) / (2.0 * assign62440_e80897))))),)
    } else {
        (locals.var_gvsat_exc, locals.var_gvsat_exc_dn5, locals.var_gvsat_exc_dn6, locals.var_gvsat_exc_dn7, locals.var_gvsat_exc_dn8,)
    }
};
        locals.var_gvsat_exc = assign62440_e80902;
        locals.var_gvsat_exc_dn5 = assign62440_e80902_d_n5;
        locals.var_gvsat_exc_dn6 = assign62440_e80902_d_n6;
        locals.var_gvsat_exc_dn7 = assign62440_e80902_d_n7;
        locals.var_gvsat_exc_dn8 = assign62440_e80902_d_n8;

        let (assign62450_e80914, assign62450_e80914_d_n5, assign62450_e80914_d_n6, assign62450_e80914_d_n7, assign62450_e80914_d_n8,) = {
    if (((locals.var_guard1760 != 0.0) && (locals.var_guard1762 != 0.0)) && (locals.var_guard1763 != 0.0)) {
        let assign62450_e80911: f64 = (locals.var_gvsat_exc * locals.var_lc);
        let assign62450_e80912: f64 = (locals.var_gmob_dc / assign62450_e80911);
        (assign62450_e80912, (((locals.var_gmob_dc_dn5 * assign62450_e80911) - (locals.var_gmob_dc * ((locals.var_gvsat_exc_dn5 * locals.var_lc) + (locals.var_gvsat_exc * locals.var_lc_dn5)))) / (assign62450_e80911 * assign62450_e80911)), (((locals.var_gmob_dc_dn6 * assign62450_e80911) - (locals.var_gmob_dc * ((locals.var_gvsat_exc_dn6 * locals.var_lc) + (locals.var_gvsat_exc * locals.var_lc_dn6)))) / (assign62450_e80911 * assign62450_e80911)), (((locals.var_gmob_dc_dn7 * assign62450_e80911) - (locals.var_gmob_dc * ((locals.var_gvsat_exc_dn7 * locals.var_lc) + (locals.var_gvsat_exc * locals.var_lc_dn7)))) / (assign62450_e80911 * assign62450_e80911)), (((locals.var_gmob_dc_dn8 * assign62450_e80911) - (locals.var_gmob_dc * ((locals.var_gvsat_exc_dn8 * locals.var_lc) + (locals.var_gvsat_exc * locals.var_lc_dn8)))) / (assign62450_e80911 * assign62450_e80911)),)
    } else {
        (locals.var_gfac, locals.var_gfac_dn5, locals.var_gfac_dn6, locals.var_gfac_dn7, locals.var_gfac_dn8,)
    }
};
        locals.var_gfac = assign62450_e80914;
        locals.var_gfac_dn5 = assign62450_e80914_d_n5;
        locals.var_gfac_dn6 = assign62450_e80914_d_n6;
        locals.var_gfac_dn7 = assign62450_e80914_d_n7;
        locals.var_gfac_dn8 = assign62450_e80914_d_n8;

        let (assign62460_e80930, assign62460_e80930_d_n5, assign62460_e80930_d_n6, assign62460_e80930_d_n7, assign62460_e80930_d_n8,) = {
    if (((locals.var_guard1760 != 0.0) && (locals.var_guard1762 != 0.0)) && (locals.var_guard1763 != 0.0)) {
        let assign62460_e80922: f64 = (locals.var_fac_exc * locals.var_i_ds);
        let assign62460_e80924: f64 = (assign62460_e80922 * locals.var_vdse_dc);
        let assign62460_e80926: f64 = (assign62460_e80924 * locals.var_gfac);
        let assign62460_e80928: f64 = (assign62460_e80926 * locals.var_gfac);
        (assign62460_e80928, (((((((locals.var_fac_exc * locals.var_i_ds_dn5) * locals.var_vdse_dc) + (assign62460_e80922 * locals.var_vdse_dc_dn5)) * locals.var_gfac) + (assign62460_e80924 * locals.var_gfac_dn5)) * locals.var_gfac) + (assign62460_e80926 * locals.var_gfac_dn5)), (((((((locals.var_fac_exc * locals.var_i_ds_dn6) * locals.var_vdse_dc) + (assign62460_e80922 * locals.var_vdse_dc_dn6)) * locals.var_gfac) + (assign62460_e80924 * locals.var_gfac_dn6)) * locals.var_gfac) + (assign62460_e80926 * locals.var_gfac_dn6)), (((((((locals.var_fac_exc * locals.var_i_ds_dn7) * locals.var_vdse_dc) + (assign62460_e80922 * locals.var_vdse_dc_dn7)) * locals.var_gfac) + (assign62460_e80924 * locals.var_gfac_dn7)) * locals.var_gfac) + (assign62460_e80926 * locals.var_gfac_dn7)), (((((((locals.var_fac_exc * locals.var_i_ds_dn8) * locals.var_vdse_dc) + (assign62460_e80922 * locals.var_vdse_dc_dn8)) * locals.var_gfac) + (assign62460_e80924 * locals.var_gfac_dn8)) * locals.var_gfac) + (assign62460_e80926 * locals.var_gfac_dn8)),)
    } else {
        (locals.var_sidexc, locals.var_sidexc_dn5, locals.var_sidexc_dn6, locals.var_sidexc_dn7, locals.var_sidexc_dn8,)
    }
};
        locals.var_sidexc = assign62460_e80930;
        locals.var_sidexc_dn5 = assign62460_e80930_d_n5;
        locals.var_sidexc_dn6 = assign62460_e80930_d_n6;
        locals.var_sidexc_dn7 = assign62460_e80930_d_n7;
        locals.var_sidexc_dn8 = assign62460_e80930_d_n8;

        let (assign62470_e80942, assign62470_e80942_d_n5, assign62470_e80942_d_n6, assign62470_e80942_d_n7, assign62470_e80942_d_n8,) = {
    if (((locals.var_guard1760 != 0.0) && (locals.var_guard1762 != 0.0)) && (locals.var_guard1763 != 0.0)) {
        let assign62470_e80939: f64 = (locals.var_sidexc / locals.var_nt0);
        let assign62470_e80940: f64 = (locals.var_mid + assign62470_e80939);
        (assign62470_e80940, (locals.var_mid_dn5 + (locals.var_sidexc_dn5 / locals.var_nt0)), (locals.var_mid_dn6 + (locals.var_sidexc_dn6 / locals.var_nt0)), (locals.var_mid_dn7 + (locals.var_sidexc_dn7 / locals.var_nt0)), (locals.var_mid_dn8 + (locals.var_sidexc_dn8 / locals.var_nt0)),)
    } else {
        (locals.var_mid, locals.var_mid_dn5, locals.var_mid_dn6, locals.var_mid_dn7, locals.var_mid_dn8,)
    }
};
        locals.var_mid = assign62470_e80942;
        locals.var_mid_dn5 = assign62470_e80942_d_n5;
        locals.var_mid_dn6 = assign62470_e80942_d_n6;
        locals.var_mid_dn7 = assign62470_e80942_d_n7;
        locals.var_mid_dn8 = assign62470_e80942_d_n8;

        let (assign62480_e80951, assign62480_e80951_d_n5, assign62480_e80951_d_n6, assign62480_e80951_d_n7, assign62480_e80951_d_n8,) = {
    if ((locals.var_guard1760 != 0.0) && (locals.var_guard1762 != 0.0)) {
        let assign62480_e80948: f64 = (locals.var_nt * locals.var_mid);
        let assign62480_e80949: f64 = (assign62480_e80948).sqrt();
        (assign62480_e80949, ((locals.var_nt * locals.var_mid_dn5) / (2.0 * assign62480_e80949)), ((locals.var_nt * locals.var_mid_dn6) / (2.0 * assign62480_e80949)), ((locals.var_nt * locals.var_mid_dn7) / (2.0 * assign62480_e80949)), ((locals.var_nt * locals.var_mid_dn8) / (2.0 * assign62480_e80949)),)
    } else {
        (locals.var_sqid, locals.var_sqid_dn5, locals.var_sqid_dn6, locals.var_sqid_dn7, locals.var_sqid_dn8,)
    }
};
        locals.var_sqid = assign62480_e80951;
        locals.var_sqid_dn5 = assign62480_e80951_d_n5;
        locals.var_sqid_dn6 = assign62480_e80951_d_n6;
        locals.var_sqid_dn7 = assign62480_e80951_d_n7;
        locals.var_sqid_dn8 = assign62480_e80951_d_n8;

        let assign62490_e80966: f64 = if ((((p.p50 == 1.0) && (locals.var_nt > 0.0)) && (p.p32 > 0.0)) && (p.p33 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1765 = assign62490_e80966;

        let (assign62500_e80998, assign62500_e80998_d_n5, assign62500_e80998_d_n6, assign62500_e80998_d_n7, assign62500_e80998_d_n8,) = {
    if ((locals.var_guard1760 != 0.0) && (locals.var_guard1765 != 0.0)) {
        let assign62500_e80972: f64 = (locals.var_t1 / 12.0);
        let assign62500_e80976: f64 = (locals.var_t1 + 0.2);
        let assign62500_e80979: f64 = (12.0 * locals.var_t2);
        let assign62500_e80980: f64 = (assign62500_e80976 - assign62500_e80979);
        let assign62500_e80981: f64 = (locals.var_t2 * assign62500_e80980);
        let assign62500_e80982: f64 = (assign62500_e80972 - assign62500_e80981);
        let assign62500_e80987: f64 = (locals.var_t1 + 1.0);
        let assign62500_e80990: f64 = (12.0 * locals.var_t2);
        let assign62500_e80991: f64 = (assign62500_e80987 - assign62500_e80990);
        let assign62500_e80992: f64 = (locals.var_t2 * assign62500_e80991);
        let assign62500_e80994: f64 = (assign62500_e80992 * locals.var_r);
        let assign62500_e80995: f64 = (1.6 * assign62500_e80994);
        let assign62500_e80996: f64 = (assign62500_e80982 - assign62500_e80995);
        (assign62500_e80996, (((locals.var_t1_dn5 / 12.0) - ((locals.var_t2_dn5 * assign62500_e80980) + (locals.var_t2 * (locals.var_t1_dn5 - (12.0 * locals.var_t2_dn5))))) - (1.6 * ((((locals.var_t2_dn5 * assign62500_e80991) + (locals.var_t2 * (locals.var_t1_dn5 - (12.0 * locals.var_t2_dn5)))) * locals.var_r) + (assign62500_e80992 * locals.var_r_dn5)))), (((locals.var_t1_dn6 / 12.0) - ((locals.var_t2_dn6 * assign62500_e80980) + (locals.var_t2 * (locals.var_t1_dn6 - (12.0 * locals.var_t2_dn6))))) - (1.6 * ((((locals.var_t2_dn6 * assign62500_e80991) + (locals.var_t2 * (locals.var_t1_dn6 - (12.0 * locals.var_t2_dn6)))) * locals.var_r) + (assign62500_e80992 * locals.var_r_dn6)))), (((locals.var_t1_dn7 / 12.0) - ((locals.var_t2_dn7 * assign62500_e80980) + (locals.var_t2 * (locals.var_t1_dn7 - (12.0 * locals.var_t2_dn7))))) - (1.6 * ((((locals.var_t2_dn7 * assign62500_e80991) + (locals.var_t2 * (locals.var_t1_dn7 - (12.0 * locals.var_t2_dn7)))) * locals.var_r) + (assign62500_e80992 * locals.var_r_dn7)))), (((locals.var_t1_dn8 / 12.0) - ((locals.var_t2_dn8 * assign62500_e80980) + (locals.var_t2 * (locals.var_t1_dn8 - (12.0 * locals.var_t2_dn8))))) - (1.6 * ((((locals.var_t2_dn8 * assign62500_e80991) + (locals.var_t2 * (locals.var_t1_dn8 - (12.0 * locals.var_t2_dn8)))) * locals.var_r) + (assign62500_e80992 * locals.var_r_dn8)))),)
    } else {
        (locals.var_mig, locals.var_mig_dn5, locals.var_mig_dn6, locals.var_mig_dn7, locals.var_mig_dn8,)
    }
};
        locals.var_mig = assign62500_e80998;
        locals.var_mig_dn5 = assign62500_e80998_d_n5;
        locals.var_mig_dn6 = assign62500_e80998_d_n6;
        locals.var_mig_dn7 = assign62500_e80998_d_n7;
        locals.var_mig_dn8 = assign62500_e80998_d_n8;

        let (assign62510_e81009, assign62510_e81009_d_n5, assign62510_e81009_d_n6, assign62510_e81009_d_n7, assign62510_e81009_d_n8,) = {
    if ((locals.var_guard1760 != 0.0) && (locals.var_guard1765 != 0.0)) {
        let (assign62510_e81007, assign62510_e81007_d_n5, assign62510_e81007_d_n6, assign62510_e81007_d_n7, assign62510_e81007_d_n8,) = {
            if (locals.var_mig > 1e-40) {
                (locals.var_mig, locals.var_mig_dn5, locals.var_mig_dn6, locals.var_mig_dn7, locals.var_mig_dn8,)
            } else {
                (1e-40, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign62510_e81007, assign62510_e81007_d_n5, assign62510_e81007_d_n6, assign62510_e81007_d_n7, assign62510_e81007_d_n8,)
    } else {
        (locals.var_mig, locals.var_mig_dn5, locals.var_mig_dn6, locals.var_mig_dn7, locals.var_mig_dn8,)
    }
};
        locals.var_mig = assign62510_e81009;
        locals.var_mig_dn5 = assign62510_e81009_d_n5;
        locals.var_mig_dn6 = assign62510_e81009_d_n6;
        locals.var_mig_dn7 = assign62510_e81009_d_n7;
        locals.var_mig_dn8 = assign62510_e81009_d_n8;

        let (assign62520_e81019, assign62520_e81019_d_n5, assign62520_e81019_d_n6, assign62520_e81019_d_n7, assign62520_e81019_d_n8,) = {
    if ((locals.var_guard1760 != 0.0) && (locals.var_guard1765 != 0.0)) {
        let assign62520_e81015: f64 = (locals.var_lcinv2 / locals.var_g_ideal);
        let assign62520_e81017: f64 = (assign62520_e81015 * locals.var_mig);
        (assign62520_e81017, (((((locals.var_lcinv2_dn5 * locals.var_g_ideal) - (locals.var_lcinv2 * locals.var_g_ideal_dn5)) / (locals.var_g_ideal * locals.var_g_ideal)) * locals.var_mig) + (assign62520_e81015 * locals.var_mig_dn5)), (((((locals.var_lcinv2_dn6 * locals.var_g_ideal) - (locals.var_lcinv2 * locals.var_g_ideal_dn6)) / (locals.var_g_ideal * locals.var_g_ideal)) * locals.var_mig) + (assign62520_e81015 * locals.var_mig_dn6)), (((((locals.var_lcinv2_dn7 * locals.var_g_ideal) - (locals.var_lcinv2 * locals.var_g_ideal_dn7)) / (locals.var_g_ideal * locals.var_g_ideal)) * locals.var_mig) + (assign62520_e81015 * locals.var_mig_dn7)), (((((locals.var_lcinv2_dn8 * locals.var_g_ideal) - (locals.var_lcinv2 * locals.var_g_ideal_dn8)) / (locals.var_g_ideal * locals.var_g_ideal)) * locals.var_mig) + (assign62520_e81015 * locals.var_mig_dn8)),)
    } else {
        (locals.var_mig, locals.var_mig_dn5, locals.var_mig_dn6, locals.var_mig_dn7, locals.var_mig_dn8,)
    }
};
        locals.var_mig = assign62520_e81019;
        locals.var_mig_dn5 = assign62520_e81019_d_n5;
        locals.var_mig_dn6 = assign62520_e81019_d_n6;
        locals.var_mig_dn7 = assign62520_e81019_d_n7;
        locals.var_mig_dn8 = assign62520_e81019_d_n8;

        let (assign62530_e81047, assign62530_e81047_d_n5, assign62530_e81047_d_n6, assign62530_e81047_d_n7, assign62530_e81047_d_n8,) = {
    if ((locals.var_guard1760 != 0.0) && (locals.var_guard1765 != 0.0)) {
        let assign62530_e81025: f64 = (locals.var_lcinv2 * locals.var_sqt2);
        let assign62530_e81029: f64 = (12.0 * locals.var_t2);
        let assign62530_e81030: f64 = (1.0 - assign62530_e81029);
        let assign62530_e81034: f64 = (19.2 * locals.var_t2);
        let assign62530_e81035: f64 = (locals.var_t1 + assign62530_e81034);
        let assign62530_e81039: f64 = (locals.var_t1 * locals.var_t2);
        let assign62530_e81040: f64 = (12.0 * assign62530_e81039);
        let assign62530_e81041: f64 = (assign62530_e81035 - assign62530_e81040);
        let assign62530_e81043: f64 = (assign62530_e81041 * locals.var_r);
        let assign62530_e81044: f64 = (assign62530_e81030 - assign62530_e81043);
        let assign62530_e81045: f64 = (assign62530_e81025 * assign62530_e81044);
        (assign62530_e81045, ((((locals.var_lcinv2_dn5 * locals.var_sqt2) + (locals.var_lcinv2 * locals.var_sqt2_dn5)) * assign62530_e81044) + (assign62530_e81025 * ((-(12.0 * locals.var_t2_dn5)) - ((((locals.var_t1_dn5 + (19.2 * locals.var_t2_dn5)) - (12.0 * ((locals.var_t1_dn5 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn5)))) * locals.var_r) + (assign62530_e81041 * locals.var_r_dn5))))), ((((locals.var_lcinv2_dn6 * locals.var_sqt2) + (locals.var_lcinv2 * locals.var_sqt2_dn6)) * assign62530_e81044) + (assign62530_e81025 * ((-(12.0 * locals.var_t2_dn6)) - ((((locals.var_t1_dn6 + (19.2 * locals.var_t2_dn6)) - (12.0 * ((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6)))) * locals.var_r) + (assign62530_e81041 * locals.var_r_dn6))))), ((((locals.var_lcinv2_dn7 * locals.var_sqt2) + (locals.var_lcinv2 * locals.var_sqt2_dn7)) * assign62530_e81044) + (assign62530_e81025 * ((-(12.0 * locals.var_t2_dn7)) - ((((locals.var_t1_dn7 + (19.2 * locals.var_t2_dn7)) - (12.0 * ((locals.var_t1_dn7 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn7)))) * locals.var_r) + (assign62530_e81041 * locals.var_r_dn7))))), ((((locals.var_lcinv2_dn8 * locals.var_sqt2) + (locals.var_lcinv2 * locals.var_sqt2_dn8)) * assign62530_e81044) + (assign62530_e81025 * ((-(12.0 * locals.var_t2_dn8)) - ((((locals.var_t1_dn8 + (19.2 * locals.var_t2_dn8)) - (12.0 * ((locals.var_t1_dn8 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn8)))) * locals.var_r) + (assign62530_e81041 * locals.var_r_dn8))))),)
    } else {
        (locals.var_migid0, locals.var_migid0_dn5, locals.var_migid0_dn6, locals.var_migid0_dn7, locals.var_migid0_dn8,)
    }
};
        locals.var_migid0 = assign62530_e81047;
        locals.var_migid0_dn5 = assign62530_e81047_d_n5;
        locals.var_migid0_dn6 = assign62530_e81047_d_n6;
        locals.var_migid0_dn7 = assign62530_e81047_d_n7;
        locals.var_migid0_dn8 = assign62530_e81047_d_n8;

        let (assign62540_e81063, assign62540_e81063_d_n5, assign62540_e81063_d_n6, assign62540_e81063_d_n7, assign62540_e81063_d_n8,) = {
    if ((locals.var_guard1760 != 0.0) && (locals.var_guard1765 != 0.0)) {
        let assign62540_e81053: f64 = (locals.var_gvsat_ac * locals.var_gvsat_ac);
        let assign62540_e81055: f64 = (assign62540_e81053 * locals.var_cox_qm);
        let assign62540_e81057: f64 = (assign62540_e81055 * locals.var_eta_p_ac);
        let assign62540_e81060: f64 = (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac);
        let assign62540_e81061: f64 = (assign62540_e81057 / assign62540_e81060);
        (assign62540_e81061, (((((((((locals.var_gvsat_ac_dn5 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn5)) * locals.var_cox_qm) + (assign62540_e81053 * locals.var_cox_qm_dn5)) * locals.var_eta_p_ac) + (assign62540_e81055 * locals.var_eta_p_ac_dn5)) * assign62540_e81060) - (assign62540_e81057 * ((locals.var_gmob_dl_ac_dn5 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn5)))) / (assign62540_e81060 * assign62540_e81060)), (((((((((locals.var_gvsat_ac_dn6 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn6)) * locals.var_cox_qm) + (assign62540_e81053 * locals.var_cox_qm_dn6)) * locals.var_eta_p_ac) + (assign62540_e81055 * locals.var_eta_p_ac_dn6)) * assign62540_e81060) - (assign62540_e81057 * ((locals.var_gmob_dl_ac_dn6 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn6)))) / (assign62540_e81060 * assign62540_e81060)), (((((((((locals.var_gvsat_ac_dn7 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn7)) * locals.var_cox_qm) + (assign62540_e81053 * locals.var_cox_qm_dn7)) * locals.var_eta_p_ac) + (assign62540_e81055 * locals.var_eta_p_ac_dn7)) * assign62540_e81060) - (assign62540_e81057 * ((locals.var_gmob_dl_ac_dn7 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn7)))) / (assign62540_e81060 * assign62540_e81060)), (((((((((locals.var_gvsat_ac_dn8 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn8)) * locals.var_cox_qm) + (assign62540_e81053 * locals.var_cox_qm_dn8)) * locals.var_eta_p_ac) + (assign62540_e81055 * locals.var_eta_p_ac_dn8)) * assign62540_e81060) - (assign62540_e81057 * ((locals.var_gmob_dl_ac_dn8 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn8)))) / (assign62540_e81060 * assign62540_e81060)),)
    } else {
        (locals.var_cgeff, locals.var_cgeff_dn5, locals.var_cgeff_dn6, locals.var_cgeff_dn7, locals.var_cgeff_dn8,)
    }
};
        locals.var_cgeff = assign62540_e81063;
        locals.var_cgeff_dn5 = assign62540_e81063_d_n5;
        locals.var_cgeff_dn6 = assign62540_e81063_d_n6;
        locals.var_cgeff_dn7 = assign62540_e81063_d_n7;
        locals.var_cgeff_dn8 = assign62540_e81063_d_n8;

        let assign62550_e81066: f64 = if locals.var_fntexc_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1766 = assign62550_e81066;

    }

    pub(super) fn stamp_transient_block_52(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign62560_e81090, assign62560_e81090_d_n5, assign62560_e81090_d_n6, assign62560_e81090_d_n7, assign62560_e81090_d_n8,) = {
    if (((locals.var_guard1760 != 0.0) && (locals.var_guard1765 != 0.0)) && (locals.var_guard1766 != 0.0)) {
        let assign62560_e81077: f64 = (12.0 * locals.var_t2);
        let assign62560_e81078: f64 = (1.0 + assign62560_e81077);
        let assign62560_e81079: f64 = (locals.var_sidexc * assign62560_e81078);
        let assign62560_e81082: f64 = (12.0 * locals.var_g_ideal);
        let assign62560_e81084: f64 = (assign62560_e81082 * locals.var_g_ideal);
        let assign62560_e81086: f64 = (assign62560_e81084 * locals.var_nt0);
        let assign62560_e81087: f64 = (assign62560_e81079 / assign62560_e81086);
        let assign62560_e81088: f64 = (locals.var_mig + assign62560_e81087);
        (assign62560_e81088, (locals.var_mig_dn5 + (((((locals.var_sidexc_dn5 * assign62560_e81078) + (locals.var_sidexc * (12.0 * locals.var_t2_dn5))) * assign62560_e81086) - (assign62560_e81079 * ((((12.0 * locals.var_g_ideal_dn5) * locals.var_g_ideal) + (assign62560_e81082 * locals.var_g_ideal_dn5)) * locals.var_nt0))) / (assign62560_e81086 * assign62560_e81086))), (locals.var_mig_dn6 + (((((locals.var_sidexc_dn6 * assign62560_e81078) + (locals.var_sidexc * (12.0 * locals.var_t2_dn6))) * assign62560_e81086) - (assign62560_e81079 * ((((12.0 * locals.var_g_ideal_dn6) * locals.var_g_ideal) + (assign62560_e81082 * locals.var_g_ideal_dn6)) * locals.var_nt0))) / (assign62560_e81086 * assign62560_e81086))), (locals.var_mig_dn7 + (((((locals.var_sidexc_dn7 * assign62560_e81078) + (locals.var_sidexc * (12.0 * locals.var_t2_dn7))) * assign62560_e81086) - (assign62560_e81079 * ((((12.0 * locals.var_g_ideal_dn7) * locals.var_g_ideal) + (assign62560_e81082 * locals.var_g_ideal_dn7)) * locals.var_nt0))) / (assign62560_e81086 * assign62560_e81086))), (locals.var_mig_dn8 + (((((locals.var_sidexc_dn8 * assign62560_e81078) + (locals.var_sidexc * (12.0 * locals.var_t2_dn8))) * assign62560_e81086) - (assign62560_e81079 * ((((12.0 * locals.var_g_ideal_dn8) * locals.var_g_ideal) + (assign62560_e81082 * locals.var_g_ideal_dn8)) * locals.var_nt0))) / (assign62560_e81086 * assign62560_e81086))),)
    } else {
        (locals.var_mig, locals.var_mig_dn5, locals.var_mig_dn6, locals.var_mig_dn7, locals.var_mig_dn8,)
    }
};
        locals.var_mig = assign62560_e81090;
        locals.var_mig_dn5 = assign62560_e81090_d_n5;
        locals.var_mig_dn6 = assign62560_e81090_d_n6;
        locals.var_mig_dn7 = assign62560_e81090_d_n7;
        locals.var_mig_dn8 = assign62560_e81090_d_n8;

        let (assign62570_e81110, assign62570_e81110_d_n5, assign62570_e81110_d_n6, assign62570_e81110_d_n7, assign62570_e81110_d_n8,) = {
    if (((locals.var_guard1760 != 0.0) && (locals.var_guard1765 != 0.0)) && (locals.var_guard1766 != 0.0)) {
        let assign62570_e81099: f64 = (locals.var_sidexc * locals.var_sqt2);
        let assign62570_e81102: f64 = (1.0 + locals.var_r);
        let assign62570_e81103: f64 = (assign62570_e81099 * assign62570_e81102);
        let assign62570_e81106: f64 = (locals.var_g_ideal * locals.var_nt0);
        let assign62570_e81107: f64 = (assign62570_e81103 / assign62570_e81106);
        let assign62570_e81108: f64 = (locals.var_migid0 - assign62570_e81107);
        (assign62570_e81108, (locals.var_migid0_dn5 - (((((((locals.var_sidexc_dn5 * locals.var_sqt2) + (locals.var_sidexc * locals.var_sqt2_dn5)) * assign62570_e81102) + (assign62570_e81099 * locals.var_r_dn5)) * assign62570_e81106) - (assign62570_e81103 * (locals.var_g_ideal_dn5 * locals.var_nt0))) / (assign62570_e81106 * assign62570_e81106))), (locals.var_migid0_dn6 - (((((((locals.var_sidexc_dn6 * locals.var_sqt2) + (locals.var_sidexc * locals.var_sqt2_dn6)) * assign62570_e81102) + (assign62570_e81099 * locals.var_r_dn6)) * assign62570_e81106) - (assign62570_e81103 * (locals.var_g_ideal_dn6 * locals.var_nt0))) / (assign62570_e81106 * assign62570_e81106))), (locals.var_migid0_dn7 - (((((((locals.var_sidexc_dn7 * locals.var_sqt2) + (locals.var_sidexc * locals.var_sqt2_dn7)) * assign62570_e81102) + (assign62570_e81099 * locals.var_r_dn7)) * assign62570_e81106) - (assign62570_e81103 * (locals.var_g_ideal_dn7 * locals.var_nt0))) / (assign62570_e81106 * assign62570_e81106))), (locals.var_migid0_dn8 - (((((((locals.var_sidexc_dn8 * locals.var_sqt2) + (locals.var_sidexc * locals.var_sqt2_dn8)) * assign62570_e81102) + (assign62570_e81099 * locals.var_r_dn8)) * assign62570_e81106) - (assign62570_e81103 * (locals.var_g_ideal_dn8 * locals.var_nt0))) / (assign62570_e81106 * assign62570_e81106))),)
    } else {
        (locals.var_migid0, locals.var_migid0_dn5, locals.var_migid0_dn6, locals.var_migid0_dn7, locals.var_migid0_dn8,)
    }
};
        locals.var_migid0 = assign62570_e81110;
        locals.var_migid0_dn5 = assign62570_e81110_d_n5;
        locals.var_migid0_dn6 = assign62570_e81110_d_n6;
        locals.var_migid0_dn7 = assign62570_e81110_d_n7;
        locals.var_migid0_dn8 = assign62570_e81110_d_n8;

        let (assign62580_e81119, assign62580_e81119_d_n5, assign62580_e81119_d_n6, assign62580_e81119_d_n7, assign62580_e81119_d_n8,) = {
    if ((locals.var_guard1760 != 0.0) && (locals.var_guard1765 != 0.0)) {
        let assign62580_e81116: f64 = (locals.var_nt / locals.var_mig);
        let assign62580_e81117: f64 = (assign62580_e81116).sqrt();
        (assign62580_e81117, ((-((locals.var_nt * locals.var_mig_dn5) / (locals.var_mig * locals.var_mig))) / (2.0 * assign62580_e81117)), ((-((locals.var_nt * locals.var_mig_dn6) / (locals.var_mig * locals.var_mig))) / (2.0 * assign62580_e81117)), ((-((locals.var_nt * locals.var_mig_dn7) / (locals.var_mig * locals.var_mig))) / (2.0 * assign62580_e81117)), ((-((locals.var_nt * locals.var_mig_dn8) / (locals.var_mig * locals.var_mig))) / (2.0 * assign62580_e81117)),)
    } else {
        (locals.var_sqig, locals.var_sqig_dn5, locals.var_sqig_dn6, locals.var_sqig_dn7, locals.var_sqig_dn8,)
    }
};
        locals.var_sqig = assign62580_e81119;
        locals.var_sqig_dn5 = assign62580_e81119_d_n5;
        locals.var_sqig_dn6 = assign62580_e81119_d_n6;
        locals.var_sqig_dn7 = assign62580_e81119_d_n7;
        locals.var_sqig_dn8 = assign62580_e81119_d_n8;

        let assign62590_e81122: f64 = if locals.var_sqid <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1767 = assign62590_e81122;

        let (assign62600_e81130, assign62600_e81130_d_n5, assign62600_e81130_d_n6, assign62600_e81130_d_n7, assign62600_e81130_d_n8,) = {
    if (((locals.var_guard1760 != 0.0) && (locals.var_guard1765 != 0.0)) && (locals.var_guard1767 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_c_igid, locals.var_c_igid_dn5, locals.var_c_igid_dn6, locals.var_c_igid_dn7, locals.var_c_igid_dn8,)
    }
};
        locals.var_c_igid = assign62600_e81130;
        locals.var_c_igid_dn5 = assign62600_e81130_d_n5;
        locals.var_c_igid_dn6 = assign62600_e81130_d_n6;
        locals.var_c_igid_dn7 = assign62600_e81130_d_n7;
        locals.var_c_igid_dn8 = assign62600_e81130_d_n8;

        let (assign62610_e81143, assign62610_e81143_d_n5, assign62610_e81143_d_n6, assign62610_e81143_d_n7, assign62610_e81143_d_n8,) = {
    if (((locals.var_guard1760 != 0.0) && (locals.var_guard1765 != 0.0)) && (locals.var_guard1767 == 0.0)) {
        let assign62610_e81139: f64 = (locals.var_migid0 * locals.var_sqig);
        let assign62610_e81141: f64 = (assign62610_e81139 / locals.var_sqid);
        (assign62610_e81141, (((((locals.var_migid0_dn5 * locals.var_sqig) + (locals.var_migid0 * locals.var_sqig_dn5)) * locals.var_sqid) - (assign62610_e81139 * locals.var_sqid_dn5)) / (locals.var_sqid * locals.var_sqid)), (((((locals.var_migid0_dn6 * locals.var_sqig) + (locals.var_migid0 * locals.var_sqig_dn6)) * locals.var_sqid) - (assign62610_e81139 * locals.var_sqid_dn6)) / (locals.var_sqid * locals.var_sqid)), (((((locals.var_migid0_dn7 * locals.var_sqig) + (locals.var_migid0 * locals.var_sqig_dn7)) * locals.var_sqid) - (assign62610_e81139 * locals.var_sqid_dn7)) / (locals.var_sqid * locals.var_sqid)), (((((locals.var_migid0_dn8 * locals.var_sqig) + (locals.var_migid0 * locals.var_sqig_dn8)) * locals.var_sqid) - (assign62610_e81139 * locals.var_sqid_dn8)) / (locals.var_sqid * locals.var_sqid)),)
    } else {
        (locals.var_c_igid, locals.var_c_igid_dn5, locals.var_c_igid_dn6, locals.var_c_igid_dn7, locals.var_c_igid_dn8,)
    }
};
        locals.var_c_igid = assign62610_e81143;
        locals.var_c_igid_dn5 = assign62610_e81143_d_n5;
        locals.var_c_igid_dn6 = assign62610_e81143_d_n6;
        locals.var_c_igid_dn7 = assign62610_e81143_d_n7;
        locals.var_c_igid_dn8 = assign62610_e81143_d_n8;

        let (assign62620_e81159, assign62620_e81159_d_n5, assign62620_e81159_d_n6, assign62620_e81159_d_n7, assign62620_e81159_d_n8,) = {
    if ((locals.var_guard1760 != 0.0) && (locals.var_guard1765 != 0.0)) {
        let (assign62620_e81157, assign62620_e81157_d_n5, assign62620_e81157_d_n6, assign62620_e81157_d_n7, assign62620_e81157_d_n8,) = {
            if (locals.var_c_igid > 0.0) {
                let (assign62620_e81155, assign62620_e81155_d_n5, assign62620_e81155_d_n6, assign62620_e81155_d_n7, assign62620_e81155_d_n8,) = {
                    if (locals.var_c_igid < 1.0) {
                        (locals.var_c_igid, locals.var_c_igid_dn5, locals.var_c_igid_dn6, locals.var_c_igid_dn7, locals.var_c_igid_dn8,)
                    } else {
                        (1.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign62620_e81155, assign62620_e81155_d_n5, assign62620_e81155_d_n6, assign62620_e81155_d_n7, assign62620_e81155_d_n8,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign62620_e81157, assign62620_e81157_d_n5, assign62620_e81157_d_n6, assign62620_e81157_d_n7, assign62620_e81157_d_n8,)
    } else {
        (locals.var_c_igid, locals.var_c_igid_dn5, locals.var_c_igid_dn6, locals.var_c_igid_dn7, locals.var_c_igid_dn8,)
    }
};
        locals.var_c_igid = assign62620_e81159;
        locals.var_c_igid_dn5 = assign62620_e81159_d_n5;
        locals.var_c_igid_dn6 = assign62620_e81159_d_n6;
        locals.var_c_igid_dn7 = assign62620_e81159_d_n7;
        locals.var_c_igid_dn8 = assign62620_e81159_d_n8;

        let (assign62630_e81169, assign62630_e81169_d_n5, assign62630_e81169_d_n6, assign62630_e81169_d_n7, assign62630_e81169_d_n8,) = {
    if ((locals.var_guard1760 != 0.0) && (locals.var_guard1765 != 0.0)) {
        let assign62630_e81165: f64 = (locals.var_c_igid * locals.var_sqid);
        let assign62630_e81167: f64 = (assign62630_e81165 / locals.var_sqig);
        (assign62630_e81167, (((((locals.var_c_igid_dn5 * locals.var_sqid) + (locals.var_c_igid * locals.var_sqid_dn5)) * locals.var_sqig) - (assign62630_e81165 * locals.var_sqig_dn5)) / (locals.var_sqig * locals.var_sqig)), (((((locals.var_c_igid_dn6 * locals.var_sqid) + (locals.var_c_igid * locals.var_sqid_dn6)) * locals.var_sqig) - (assign62630_e81165 * locals.var_sqig_dn6)) / (locals.var_sqig * locals.var_sqig)), (((((locals.var_c_igid_dn7 * locals.var_sqid) + (locals.var_c_igid * locals.var_sqid_dn7)) * locals.var_sqig) - (assign62630_e81165 * locals.var_sqig_dn7)) / (locals.var_sqig * locals.var_sqig)), (((((locals.var_c_igid_dn8 * locals.var_sqid) + (locals.var_c_igid * locals.var_sqid_dn8)) * locals.var_sqig) - (assign62630_e81165 * locals.var_sqig_dn8)) / (locals.var_sqig * locals.var_sqig)),)
    } else {
        (locals.var_migid, locals.var_migid_dn5, locals.var_migid_dn6, locals.var_migid_dn7, locals.var_migid_dn8,)
    }
};
        locals.var_migid = assign62630_e81169;
        locals.var_migid_dn5 = assign62630_e81169_d_n5;
        locals.var_migid_dn6 = assign62630_e81169_d_n6;
        locals.var_migid_dn7 = assign62630_e81169_d_n7;
        locals.var_migid_dn8 = assign62630_e81169_d_n8;

        let assign62800_e81277: f64 = if (((p.p46 != 0.0) && (locals.var_betnedge_i > 0.0)) && (locals.var_xgedge > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1769 = assign62800_e81277;

        let (assign62810_e81285, assign62810_e81285_d_n5, assign62810_e81285_d_n6, assign62810_e81285_d_n7, assign62810_e81285_d_n8,) = {
    if (locals.var_guard1769 != 0.0) {
        let assign62810_e81281: f64 = (4.0 * locals.var_dsqredge);
        let assign62810_e81283: f64 = (assign62810_e81281 / locals.var_gfedge2);
        (assign62810_e81283, ((4.0 * locals.var_dsqredge_dn5) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn6) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn7) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn8) / locals.var_gfedge2),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign62810_e81285;
        locals.var_temp1_dn5 = assign62810_e81285_d_n5;
        locals.var_temp1_dn6 = assign62810_e81285_d_n6;
        locals.var_temp1_dn7 = assign62810_e81285_d_n7;
        locals.var_temp1_dn8 = assign62810_e81285_d_n8;

        let (assign62830_e81305, assign62830_e81305_d_n5, assign62830_e81305_d_n6, assign62830_e81305_d_n7, assign62830_e81305_d_n8,) = {
    if (locals.var_guard1769 != 0.0) {
        let assign62830_e81303: f64 = (locals.var_cox_over_q * locals.var_phit);
        (assign62830_e81303, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign62830_e81305;
        locals.var_temp1_dn5 = assign62830_e81305_d_n5;
        locals.var_temp1_dn6 = assign62830_e81305_d_n6;
        locals.var_temp1_dn7 = assign62830_e81305_d_n7;
        locals.var_temp1_dn8 = assign62830_e81305_d_n8;

        let (assign62960_e81445, assign62960_e81445_d_n5, assign62960_e81445_d_n6, assign62960_e81445_d_n7, assign62960_e81445_d_n8,) = {
    if (locals.var_guard1769 != 0.0) {
        let assign62960_e81443: f64 = (locals.var_alpha_dc * locals.var_h_dc);
        (assign62960_e81443, ((locals.var_alpha_dc_dn5 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn5)), ((locals.var_alpha_dc_dn6 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn6)), ((locals.var_alpha_dc_dn7 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn7)), ((locals.var_alpha_dc_dn8 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn8)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign62960_e81445;
        locals.var_temp1_dn5 = assign62960_e81445_d_n5;
        locals.var_temp1_dn6 = assign62960_e81445_d_n6;
        locals.var_temp1_dn7 = assign62960_e81445_d_n7;
        locals.var_temp1_dn8 = assign62960_e81445_d_n8;

    }

    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let assign00_e1445: f64 = if p.p37 >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1 = assign00_e1445;
        locals.var_guard1_rv = 0.0;

        let (assign10_e1450,) = {
    if (locals.var_guard1 != 0.0) {
        let assign10_e1448: f64 = 1.0;
        (assign10_e1448,)
    } else {
        (locals.var_chnl_type,)
    }
};
        locals.var_chnl_type = assign10_e1450;
        locals.var_chnl_type_rv = 0.0;

        let (assign20_e1456,) = {
    if (locals.var_guard1 == 0.0) {
        let assign20_e1454: f64 = (-1.0);
        (assign20_e1454,)
    } else {
        (locals.var_chnl_type,)
    }
};
        locals.var_chnl_type = assign20_e1456;
        locals.var_chnl_type_rv = 0.0;

        let assign30_e1459: f64 = (8.8541878176e-12 * 11.8);
        locals.var_epssi = assign30_e1459;
        locals.var_epssi_rv = 0.0;

        let assign40_e1462: f64 = (273.15 + p.p38);
        locals.var_tkr = assign40_e1462;
        locals.var_tkr_rv = 0.0;

        let assign2050_e2493: f64 = ctx_temp;
        let assign2050_e2495: f64 = (assign2050_e2493 + p.p55);
        let assign2050_e2497: f64 = (assign2050_e2495 + p.p35);
        locals.var_tka = assign2050_e2497;
        locals.var_tka_rv = 0.0;

        let assign2060_e2500: f64 = (locals.var_tka / locals.var_tkr);
        locals.var_rta = assign2060_e2500;
        locals.var_rta_rv = 0.0;

        let assign2070_e2503: f64 = (locals.var_tka - locals.var_tkr);
        locals.var_delta = assign2070_e2503;
        locals.var_delta_rv = 0.0;

        let assign2080_e2506: f64 = (locals.var_tka * 1.3806505e-23);
        let assign2080_e2508: f64 = (assign2080_e2506 / 1.6021918e-19);
        locals.var_phita = assign2080_e2508;
        locals.var_phita_rv = 0.0;

        let assign2090_e2511: f64 = (1.0 / locals.var_phita);
        locals.var_inv_phita = assign2090_e2511;
        locals.var_inv_phita_rv = 0.0;

        locals.var_tkd = locals.var_tka;
        locals.var_tkd_rv = 0.0;

        let assign2110_e2515: f64 = (locals.var_tkd * locals.var_tkd);
        locals.var_tkd_sq = assign2110_e2515;
        locals.var_tkd_sq_rv = 0.0;

        let assign2120_e2518: f64 = (locals.var_tkd - locals.var_tkr);
        locals.var_delt = assign2120_e2518;
        locals.var_delt_rv = 0.0;

        let assign2130_e2521: f64 = (locals.var_tkr / locals.var_tkd);
        locals.var_rtn = assign2130_e2521;
        locals.var_rtn_rv = 0.0;

        let assign2140_e2523: f64 = (locals.var_rtn).ln();
        locals.var_ln_rtn = assign2140_e2523;
        locals.var_ln_rtn_rv = 0.0;

        let assign2150_e2526: f64 = (locals.var_tkd * 1.3806505e-23);
        let assign2150_e2528: f64 = (assign2150_e2526 / 1.6021918e-19);
        locals.var_phit = assign2150_e2528;
        locals.var_phit_rv = 0.0;

        let assign2160_e2531: f64 = (1.0 / locals.var_phit);
        locals.var_inv_phit = assign2160_e2531;
        locals.var_inv_phit_rv = 0.0;

        let assign2170_e2535: f64 = (9.025e-5 * locals.var_tkd);
        let assign2170_e2536: f64 = (1.179 - assign2170_e2535);
        let assign2170_e2539: f64 = (3.05e-7 * locals.var_tkd_sq);
        let assign2170_e2540: f64 = (assign2170_e2536 - assign2170_e2539);
        locals.var_eg = assign2170_e2540;
        locals.var_eg_rv = 0.0;

        let assign2180_e2544: f64 = (0.00045 * locals.var_tkd);
        let assign2180_e2545: f64 = (1.045 + assign2180_e2544);
        let assign2180_e2549: f64 = (0.0014 * locals.var_tkd);
        let assign2180_e2550: f64 = (0.523 + assign2180_e2549);
        let assign2180_e2553: f64 = (1.48e-6 * locals.var_tkd_sq);
        let assign2180_e2554: f64 = (assign2180_e2550 - assign2180_e2553);
        let assign2180_e2555: f64 = (assign2180_e2545 * assign2180_e2554);
        let assign2180_e2557: f64 = (assign2180_e2555 * locals.var_tkd_sq);
        let assign2180_e2559: f64 = (assign2180_e2557 / 90000.0);
        locals.var_phibfac = assign2180_e2559;
        locals.var_phibfac_rv = 0.0;

        let (assign2190_e2565,) = {
    if (locals.var_phibfac > 0.001) {
        (locals.var_phibfac,)
    } else {
        (0.001,)
    }
};
        locals.var_phibfac = assign2190_e2565;
        locals.var_phibfac_rv = 0.0;

        locals.var_nf_i = 1.0;
        locals.var_nf_i_rv = 0.0;

        locals.var_invnf = 1.0;
        locals.var_invnf_rv = 0.0;

        locals.var_le = 0.0;
        locals.var_le_rv = 0.0;

        locals.var_we = 0.0;
        locals.var_we_rv = 0.0;

        locals.var_l_i = p.p0;
        locals.var_l_i_rv = 0.0;

        locals.var_w_i = p.p1;
        locals.var_w_i_rv = 0.0;

        locals.var_sa_i = p.p2;
        locals.var_sa_i_rv = 0.0;

        locals.var_sb_i = p.p3;
        locals.var_sb_i_rv = 0.0;

        locals.var_sd_i = p.p4;
        locals.var_sd_i_rv = 0.0;

        locals.var_sc_i = p.p8;
        locals.var_sc_i_rv = 0.0;

        let assign3500_e3418: f64 = if p.p39 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard29 = assign3500_e3418;
        locals.var_guard29_rv = 0.0;

        let (assign3510_e3427,) = {
    if (locals.var_guard29 != 0.0) {
        let (assign3510_e3425,) = {
            if (p.p9 > 1.0) {
                (p.p9,)
            } else {
                (1.0,)
            }
        };
        (assign3510_e3425,)
    } else {
        (locals.var_nf_i,)
    }
};
        locals.var_nf_i = assign3510_e3427;
        locals.var_nf_i_rv = 0.0;

        let (assign3520_e3434,) = {
    if (locals.var_guard29 != 0.0) {
        let assign3520_e3431: f64 = (locals.var_nf_i + 0.5);
        let assign3520_e3432: f64 = (assign3520_e3431).floor();
        (assign3520_e3432,)
    } else {
        (locals.var_nf_i,)
    }
};
        locals.var_nf_i = assign3520_e3434;
        locals.var_nf_i_rv = 0.0;

        let (assign3530_e3440,) = {
    if (locals.var_guard29 != 0.0) {
        let assign3530_e3438: f64 = (1.0 / locals.var_nf_i);
        (assign3530_e3438,)
    } else {
        (locals.var_invnf,)
    }
};
        locals.var_invnf = assign3530_e3440;
        locals.var_invnf_rv = 0.0;

        let assign3540_e3443: f64 = (locals.var_w_i * locals.var_invnf);
        let (assign3540_e3450,) = {
    if (assign3540_e3443 > 1e-9) {
        let assign3540_e3448: f64 = (locals.var_w_i * locals.var_invnf);
        (assign3540_e3448,)
    } else {
        (1e-9,)
    }
};
        locals.var_w_i = assign3540_e3450;
        locals.var_w_i_rv = 0.0;

        locals.var_sca_i = p.p5;
        locals.var_sca_i_rv = 0.0;

        locals.var_scb_i = p.p6;
        locals.var_scb_i_rv = 0.0;

        locals.var_scc_i = p.p7;
        locals.var_scc_i_rv = 0.0;

        let assign3590_e3462: f64 = (1e-6 / locals.var_l_i);
        locals.var_il = assign3590_e3462;
        locals.var_il_rv = 0.0;

        let assign3600_e3465: f64 = (1e-6 / locals.var_w_i);
        locals.var_iw = assign3600_e3465;
        locals.var_iw_rv = 0.0;

        let assign3610_e3470: f64 = (p.p187 * locals.var_il);
        let assign3610_e3471: f64 = (1.0 + assign3610_e3470);
        let assign3610_e3472: f64 = (p.p186 * assign3610_e3471);
        let assign3610_e3476: f64 = (p.p188 * locals.var_iw);
        let assign3610_e3477: f64 = (1.0 + assign3610_e3476);
        let assign3610_e3478: f64 = (assign3610_e3472 * assign3610_e3477);
        locals.var_dellps = assign3610_e3478;
        locals.var_dellps_rv = 0.0;

        let assign3620_e3483: f64 = (p.p191 * locals.var_il);
        let assign3620_e3484: f64 = (1.0 + assign3620_e3483);
        let assign3620_e3485: f64 = (p.p190 * assign3620_e3484);
        let assign3620_e3489: f64 = (p.p192 * locals.var_iw);
        let assign3620_e3490: f64 = (1.0 + assign3620_e3489);
        let assign3620_e3491: f64 = (assign3620_e3485 * assign3620_e3490);
        locals.var_delwod = assign3620_e3491;
        locals.var_delwod_rv = 0.0;

        let assign3630_e3494: f64 = (locals.var_l_i + locals.var_dellps);
        let assign3630_e3497: f64 = (2.0 * p.p189);
        let assign3630_e3498: f64 = (assign3630_e3494 - assign3630_e3497);
        let (assign3630_e3509,) = {
    if (assign3630_e3498 > 1e-9) {
        let assign3630_e3503: f64 = (locals.var_l_i + locals.var_dellps);
        let assign3630_e3506: f64 = (2.0 * p.p189);
        let assign3630_e3507: f64 = (assign3630_e3503 - assign3630_e3506);
        (assign3630_e3507,)
    } else {
        (1e-9,)
    }
};
        locals.var_le = assign3630_e3509;
        locals.var_le_rv = 0.0;

        let assign3640_e3512: f64 = (locals.var_w_i + locals.var_delwod);
        let assign3640_e3515: f64 = (2.0 * p.p193);
        let assign3640_e3516: f64 = (assign3640_e3512 - assign3640_e3515);
        let (assign3640_e3527,) = {
    if (assign3640_e3516 > 1e-9) {
        let assign3640_e3521: f64 = (locals.var_w_i + locals.var_delwod);
        let assign3640_e3524: f64 = (2.0 * p.p193);
        let assign3640_e3525: f64 = (assign3640_e3521 - assign3640_e3524);
        (assign3640_e3525,)
    } else {
        (1e-9,)
    }
};
        locals.var_we = assign3640_e3527;
        locals.var_we_rv = 0.0;

        let assign3650_e3530: f64 = (1e-6 / locals.var_le);
        locals.var_ile = assign3650_e3530;
        locals.var_ile_rv = 0.0;

        let assign3660_e3533: f64 = (locals.var_ile * locals.var_ile);
        locals.var_ile2 = assign3660_e3533;
        locals.var_ile2_rv = 0.0;

        let assign3670_e3536: f64 = (1e-6 / locals.var_we);
        locals.var_iwe = assign3670_e3536;
        locals.var_iwe_rv = 0.0;

        let assign3680_e3539: f64 = (1.0 / locals.var_iwe);
        locals.var_iiwe = assign3680_e3539;
        locals.var_iiwe_rv = 0.0;

        let assign3690_e3542: f64 = (locals.var_ile * locals.var_iwe);
        locals.var_iae = assign3690_e3542;
        locals.var_iae_rv = 0.0;

        let assign3700_e3545: f64 = (1.0 / locals.var_iae);
        locals.var_iiae = assign3700_e3545;
        locals.var_iiae_rv = 0.0;

        let assign3710_e3548: f64 = (locals.var_l_i + locals.var_dellps);
        let assign3710_e3551: f64 = (2.0 * p.p189);
        let assign3710_e3552: f64 = (assign3710_e3548 - assign3710_e3551);
        let assign3710_e3554: f64 = (assign3710_e3552 + p.p194);
        let (assign3710_e3567,) = {
    if (assign3710_e3554 > 1e-9) {
        let assign3710_e3559: f64 = (locals.var_l_i + locals.var_dellps);
        let assign3710_e3562: f64 = (2.0 * p.p189);
        let assign3710_e3563: f64 = (assign3710_e3559 - assign3710_e3562);
        let assign3710_e3565: f64 = (assign3710_e3563 + p.p194);
        (assign3710_e3565,)
    } else {
        (1e-9,)
    }
};
        locals.var_lecv = assign3710_e3567;
        locals.var_lecv_rv = 0.0;

        let assign3720_e3570: f64 = (locals.var_w_i + locals.var_delwod);
        let assign3720_e3573: f64 = (2.0 * p.p193);
        let assign3720_e3574: f64 = (assign3720_e3570 - assign3720_e3573);
        let assign3720_e3576: f64 = (assign3720_e3574 + p.p195);
        let (assign3720_e3589,) = {
    if (assign3720_e3576 > 1e-9) {
        let assign3720_e3581: f64 = (locals.var_w_i + locals.var_delwod);
        let assign3720_e3584: f64 = (2.0 * p.p193);
        let assign3720_e3585: f64 = (assign3720_e3581 - assign3720_e3584);
        let assign3720_e3587: f64 = (assign3720_e3585 + p.p195);
        (assign3720_e3587,)
    } else {
        (1e-9,)
    }
};
        locals.var_wecv = assign3720_e3589;
        locals.var_wecv_rv = 0.0;

        let assign3730_e3592: f64 = (locals.var_wecv / 1e-6);
        locals.var_iiwecv = assign3730_e3592;
        locals.var_iiwecv_rv = 0.0;

        let assign3740_e3595: f64 = (locals.var_l_i + locals.var_dellps);
        let assign3740_e3597: f64 = (assign3740_e3595 + p.p194);
        let (assign3740_e3606,) = {
    if (assign3740_e3597 > 1e-9) {
        let assign3740_e3602: f64 = (locals.var_l_i + locals.var_dellps);
        let assign3740_e3604: f64 = (assign3740_e3602 + p.p194);
        (assign3740_e3604,)
    } else {
        (1e-9,)
    }
};
        locals.var_lcv = assign3740_e3606;
        locals.var_lcv_rv = 0.0;

        let assign3760_e3623: f64 = (locals.var_lcv / 1e-6);
        locals.var_iilcv = assign3760_e3623;
        locals.var_iilcv_rv = 0.0;

        locals.var_vfb_p = p.p56;
        locals.var_vfb_p_rv = 0.0;

        locals.var_stvfb_p = p.p57;
        locals.var_stvfb_p_rv = 0.0;

        locals.var_st2vfb_p = p.p58;
        locals.var_st2vfb_p_rv = 0.0;

        locals.var_tox_p = p.p59;
        locals.var_tox_p_rv = 0.0;

        locals.var_epsrox_p = p.p60;
        locals.var_epsrox_p_rv = 0.0;

        locals.var_neff_p = p.p61;
        locals.var_neff_p_rv = 0.0;

        locals.var_gfacnud_p = p.p62;
        locals.var_gfacnud_p_rv = 0.0;

        locals.var_vsbnud_p = p.p63;
        locals.var_vsbnud_p_rv = 0.0;

        locals.var_dvsbnud_p = p.p64;
        locals.var_dvsbnud_p_rv = 0.0;

        locals.var_dphib_p = p.p65;
        locals.var_dphib_p_rv = 0.0;

        locals.var_np_p = p.p66;
        locals.var_np_p_rv = 0.0;

        locals.var_toxov_p = p.p67;
        locals.var_toxov_p_rv = 0.0;

        locals.var_toxovd_p = p.p68;
        locals.var_toxovd_p_rv = 0.0;

        locals.var_nov_p = p.p69;
        locals.var_nov_p_rv = 0.0;

        locals.var_novd_p = p.p70;
        locals.var_novd_p_rv = 0.0;

        locals.var_ct_p = p.p71;
        locals.var_ct_p_rv = 0.0;

        locals.var_ctg_p = p.p73;
        locals.var_ctg_p_rv = 0.0;

        locals.var_ctb_p = p.p72;
        locals.var_ctb_p_rv = 0.0;

        locals.var_stct_p = p.p74;
        locals.var_stct_p_rv = 0.0;

        locals.var_psce_p = p.p78;
        locals.var_psce_p_rv = 0.0;

        locals.var_psced_p = p.p80;
        locals.var_psced_p_rv = 0.0;

        locals.var_psceb_p = p.p79;
        locals.var_psceb_p_rv = 0.0;

        locals.var_cf_p = p.p75;
        locals.var_cf_p_rv = 0.0;

        locals.var_cfd_p = p.p77;
        locals.var_cfd_p_rv = 0.0;

        locals.var_cfb_p = p.p76;
        locals.var_cfb_p_rv = 0.0;

        locals.var_betn_p = p.p81;
        locals.var_betn_p_rv = 0.0;

        locals.var_stbet_p = p.p82;
        locals.var_stbet_p_rv = 0.0;

        locals.var_mue_p = p.p83;
        locals.var_mue_p_rv = 0.0;

        locals.var_stmue_p = p.p84;
        locals.var_stmue_p_rv = 0.0;

        locals.var_themu_p = p.p85;
        locals.var_themu_p_rv = 0.0;

        locals.var_stthemu_p = p.p86;
        locals.var_stthemu_p_rv = 0.0;

        locals.var_cs_p = p.p87;
        locals.var_cs_p_rv = 0.0;

        locals.var_stcs_p = p.p88;
        locals.var_stcs_p_rv = 0.0;

        locals.var_thecs_p = p.p89;
        locals.var_thecs_p_rv = 0.0;

        locals.var_stthecs_p = p.p90;
        locals.var_stthecs_p_rv = 0.0;

        locals.var_xcor_p = p.p91;
        locals.var_xcor_p_rv = 0.0;

        locals.var_stxcor_p = p.p92;
        locals.var_stxcor_p_rv = 0.0;

        locals.var_feta_p = p.p93;
        locals.var_feta_p_rv = 0.0;

        locals.var_rs_p = p.p94;
        locals.var_rs_p_rv = 0.0;

        locals.var_strs_p = p.p95;
        locals.var_strs_p_rv = 0.0;

        locals.var_rsb_p = p.p96;
        locals.var_rsb_p_rv = 0.0;

        locals.var_rsg_p = p.p97;
        locals.var_rsg_p_rv = 0.0;

        locals.var_thesat_p = p.p98;
        locals.var_thesat_p_rv = 0.0;

        locals.var_stthesat_p = p.p99;
        locals.var_stthesat_p_rv = 0.0;

        locals.var_thesatb_p = p.p100;
        locals.var_thesatb_p_rv = 0.0;

        locals.var_thesatg_p = p.p101;
        locals.var_thesatg_p_rv = 0.0;

        locals.var_thesatt_p = p.p102;
        locals.var_thesatt_p_rv = 0.0;

        locals.var_ax_p = p.p103;
        locals.var_ax_p_rv = 0.0;

        locals.var_alp_p = p.p104;
        locals.var_alp_p_rv = 0.0;

        locals.var_alp1_p = p.p105;
        locals.var_alp1_p_rv = 0.0;

        locals.var_alp2_p = p.p106;
        locals.var_alp2_p_rv = 0.0;

        locals.var_vp_p = p.p107;
        locals.var_vp_p_rv = 0.0;

        locals.var_a1_p = p.p108;
        locals.var_a1_p_rv = 0.0;

        locals.var_a2_p = p.p109;
        locals.var_a2_p_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_1(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        locals.var_sta2_p = p.p110;
        locals.var_sta2_p_rv = 0.0;

        locals.var_a3_p = p.p111;
        locals.var_a3_p_rv = 0.0;

        locals.var_a4_p = p.p112;
        locals.var_a4_p_rv = 0.0;

        locals.var_imaxii_p = p.p113;
        locals.var_imaxii_p_rv = 0.0;

        locals.var_gco_p = p.p114;
        locals.var_gco_p_rv = 0.0;

        locals.var_iginv_p = p.p115;
        locals.var_iginv_p_rv = 0.0;

        locals.var_igov_p = p.p116;
        locals.var_igov_p_rv = 0.0;

        locals.var_igovd_p = p.p117;
        locals.var_igovd_p_rv = 0.0;

        locals.var_stig_p = p.p118;
        locals.var_stig_p_rv = 0.0;

        locals.var_gc2_p = p.p119;
        locals.var_gc2_p_rv = 0.0;

        locals.var_gc3_p = p.p120;
        locals.var_gc3_p_rv = 0.0;

        locals.var_gc2ov_p = p.p119;
        locals.var_gc2ov_p_rv = 0.0;

        let assign4480_e3738: f64 = if param_given[121] { 1.0 } else { 0.0 };
        let assign4480_e3740: f64 = if assign4480_e3738 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard30 = assign4480_e3740;
        locals.var_guard30_rv = 0.0;

        let (assign4490_e3744,) = {
    if (locals.var_guard30 != 0.0) {
        (p.p121,)
    } else {
        (locals.var_gc2ov_p,)
    }
};
        locals.var_gc2ov_p = assign4490_e3744;
        locals.var_gc2ov_p_rv = 0.0;

        locals.var_gc3ov_p = p.p120;
        locals.var_gc3ov_p_rv = 0.0;

        let assign4510_e3747: f64 = if param_given[122] { 1.0 } else { 0.0 };
        let assign4510_e3749: f64 = if assign4510_e3747 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard31 = assign4510_e3749;
        locals.var_guard31_rv = 0.0;

        let (assign4520_e3753,) = {
    if (locals.var_guard31 != 0.0) {
        (p.p122,)
    } else {
        (locals.var_gc3ov_p,)
    }
};
        locals.var_gc3ov_p = assign4520_e3753;
        locals.var_gc3ov_p_rv = 0.0;

        locals.var_gc2ovd_p = locals.var_gc2ov_p;
        locals.var_gc2ovd_p_rv = 0.0;

        let assign4540_e3756: f64 = if param_given[123] { 1.0 } else { 0.0 };
        let assign4540_e3758: f64 = if assign4540_e3756 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard32 = assign4540_e3758;
        locals.var_guard32_rv = 0.0;

        let (assign4550_e3762,) = {
    if (locals.var_guard32 != 0.0) {
        (p.p123,)
    } else {
        (locals.var_gc2ovd_p,)
    }
};
        locals.var_gc2ovd_p = assign4550_e3762;
        locals.var_gc2ovd_p_rv = 0.0;

        locals.var_gc3ovd_p = locals.var_gc3ov_p;
        locals.var_gc3ovd_p_rv = 0.0;

        let assign4570_e3765: f64 = if param_given[124] { 1.0 } else { 0.0 };
        let assign4570_e3767: f64 = if assign4570_e3765 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard33 = assign4570_e3767;
        locals.var_guard33_rv = 0.0;

        let (assign4580_e3771,) = {
    if (locals.var_guard33 != 0.0) {
        (p.p124,)
    } else {
        (locals.var_gc3ovd_p,)
    }
};
        locals.var_gc3ovd_p = assign4580_e3771;
        locals.var_gc3ovd_p_rv = 0.0;

        locals.var_chib_p = p.p125;
        locals.var_chib_p_rv = 0.0;

        locals.var_agidl_p = p.p126;
        locals.var_agidl_p_rv = 0.0;

        locals.var_agidld_p = p.p127;
        locals.var_agidld_p_rv = 0.0;

        locals.var_bgidl_p = p.p128;
        locals.var_bgidl_p_rv = 0.0;

        locals.var_bgidld_p = p.p129;
        locals.var_bgidld_p_rv = 0.0;

        locals.var_stbgidl_p = p.p130;
        locals.var_stbgidl_p_rv = 0.0;

        locals.var_stbgidld_p = p.p131;
        locals.var_stbgidld_p_rv = 0.0;

        locals.var_cgidl_p = p.p132;
        locals.var_cgidl_p_rv = 0.0;

        locals.var_cgidld_p = p.p133;
        locals.var_cgidld_p_rv = 0.0;

        locals.var_cox_p = p.p134;
        locals.var_cox_p_rv = 0.0;

        locals.var_delvtac_p = p.p135;
        locals.var_delvtac_p_rv = 0.0;

        locals.var_facneffac_p = p.p136;
        locals.var_facneffac_p_rv = 0.0;

        locals.var_thesatac_p = p.p98;
        locals.var_thesatac_p_rv = 0.0;

        let assign4720_e3786: f64 = if param_given[137] { 1.0 } else { 0.0 };
        let assign4720_e3788: f64 = if assign4720_e3786 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard34 = assign4720_e3788;
        locals.var_guard34_rv = 0.0;

        let (assign4730_e3792,) = {
    if (locals.var_guard34 != 0.0) {
        (p.p137,)
    } else {
        (locals.var_thesatac_p,)
    }
};
        locals.var_thesatac_p = assign4730_e3792;
        locals.var_thesatac_p_rv = 0.0;

        locals.var_axac_p = p.p103;
        locals.var_axac_p_rv = 0.0;

        let assign4750_e3795: f64 = if param_given[138] { 1.0 } else { 0.0 };
        let assign4750_e3797: f64 = if assign4750_e3795 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard35 = assign4750_e3797;
        locals.var_guard35_rv = 0.0;

        let (assign4760_e3801,) = {
    if (locals.var_guard35 != 0.0) {
        (p.p138,)
    } else {
        (locals.var_axac_p,)
    }
};
        locals.var_axac_p = assign4760_e3801;
        locals.var_axac_p_rv = 0.0;

        locals.var_alpac_p = p.p139;
        locals.var_alpac_p_rv = 0.0;

        locals.var_alp1ac_p = p.p140;
        locals.var_alp1ac_p_rv = 0.0;

        locals.var_cgov_p = p.p141;
        locals.var_cgov_p_rv = 0.0;

        locals.var_cgovd_p = p.p142;
        locals.var_cgovd_p_rv = 0.0;

        locals.var_fcgovacc_p = p.p143;
        locals.var_fcgovacc_p_rv = 0.0;

        locals.var_fcgovaccd_p = p.p144;
        locals.var_fcgovaccd_p_rv = 0.0;

        locals.var_cgovaccg_p = p.p145;
        locals.var_cgovaccg_p_rv = 0.0;

        locals.var_cgbov_p = p.p146;
        locals.var_cgbov_p_rv = 0.0;

        locals.var_cinr_p = p.p147;
        locals.var_cinr_p_rv = 0.0;

        locals.var_cinrd_p = p.p148;
        locals.var_cinrd_p_rv = 0.0;

        locals.var_dvfbinr_p = p.p149;
        locals.var_dvfbinr_p_rv = 0.0;

        locals.var_fcinrdep_p = p.p150;
        locals.var_fcinrdep_p_rv = 0.0;

        locals.var_fcinracc_p = p.p151;
        locals.var_fcinracc_p_rv = 0.0;

        locals.var_axinr_p = p.p152;
        locals.var_axinr_p_rv = 0.0;

        locals.var_fnt_p = p.p155;
        locals.var_fnt_p_rv = 0.0;

        locals.var_vfbedge_p = p.p161;
        locals.var_vfbedge_p_rv = 0.0;

        locals.var_stvfbedge_p = p.p162;
        locals.var_stvfbedge_p_rv = 0.0;

        locals.var_dphibedge_p = p.p163;
        locals.var_dphibedge_p_rv = 0.0;

        locals.var_neffedge_p = p.p164;
        locals.var_neffedge_p_rv = 0.0;

        locals.var_ctedge_p = p.p165;
        locals.var_ctedge_p_rv = 0.0;

        locals.var_betnedge_p = p.p166;
        locals.var_betnedge_p_rv = 0.0;

        locals.var_stbetedge_p = p.p167;
        locals.var_stbetedge_p_rv = 0.0;

        locals.var_psceedge_p = p.p168;
        locals.var_psceedge_p_rv = 0.0;

        locals.var_pscebedge_p = p.p169;
        locals.var_pscebedge_p_rv = 0.0;

        locals.var_pscededge_p = p.p170;
        locals.var_pscededge_p_rv = 0.0;

        locals.var_cfedge_p = p.p171;
        locals.var_cfedge_p_rv = 0.0;

        locals.var_cfdedge_p = p.p173;
        locals.var_cfdedge_p_rv = 0.0;

        locals.var_cfbedge_p = p.p172;
        locals.var_cfbedge_p_rv = 0.0;

        let assign5240_e3851: f64 = if p.p39 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard36 = assign5240_e3851;
        locals.var_guard36_rv = 0.0;

        let (assign5250_e3869,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5250_e3857: f64 = (locals.var_ile).powf(p.p198);
        let assign5250_e3858: f64 = (p.p197 * assign5250_e3857);
        let assign5250_e3859: f64 = (p.p196 + assign5250_e3858);
        let assign5250_e3862: f64 = (p.p199 * locals.var_iwe);
        let assign5250_e3863: f64 = (assign5250_e3859 + assign5250_e3862);
        let assign5250_e3866: f64 = (p.p200 * locals.var_iae);
        let assign5250_e3867: f64 = (assign5250_e3863 + assign5250_e3866);
        (assign5250_e3867,)
    } else {
        (locals.var_vfb_p,)
    }
};
        locals.var_vfb_p = assign5250_e3869;
        locals.var_vfb_p_rv = 0.0;

        let (assign5260_e3885,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5260_e3874: f64 = (p.p202 * locals.var_ile);
        let assign5260_e3875: f64 = (p.p201 + assign5260_e3874);
        let assign5260_e3878: f64 = (p.p203 * locals.var_iwe);
        let assign5260_e3879: f64 = (assign5260_e3875 + assign5260_e3878);
        let assign5260_e3882: f64 = (p.p204 * locals.var_iae);
        let assign5260_e3883: f64 = (assign5260_e3879 + assign5260_e3882);
        (assign5260_e3883,)
    } else {
        (locals.var_stvfb_p,)
    }
};
        locals.var_stvfb_p = assign5260_e3885;
        locals.var_stvfb_p_rv = 0.0;

        let (assign5270_e3889,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p205,)
    } else {
        (locals.var_st2vfb_p,)
    }
};
        locals.var_st2vfb_p = assign5270_e3889;
        locals.var_st2vfb_p_rv = 0.0;

        let (assign5280_e3893,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p206,)
    } else {
        (locals.var_tox_p,)
    }
};
        locals.var_tox_p = assign5280_e3893;
        locals.var_tox_p_rv = 0.0;

        let (assign5290_e3897,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p207,)
    } else {
        (locals.var_epsrox_p,)
    }
};
        locals.var_epsrox_p = assign5290_e3897;
        locals.var_epsrox_p_rv = 0.0;

        let (assign5300_e3930,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5300_e3903: f64 = (p.p209 * locals.var_iwe);
        let assign5300_e3907: f64 = (locals.var_we / p.p210);
        let assign5300_e3908: f64 = (1.0 + assign5300_e3907);
        let assign5300_e3909: f64 = (assign5300_e3908).ln();
        let assign5300_e3910: f64 = (assign5300_e3903 * assign5300_e3909);
        let assign5300_e3911: f64 = (1.0 + assign5300_e3910);
        let (assign5300_e3927,) = {
            if (assign5300_e3911 > 0.001) {
                let assign5300_e3917: f64 = (p.p209 * locals.var_iwe);
                let assign5300_e3921: f64 = (locals.var_we / p.p210);
                let assign5300_e3922: f64 = (1.0 + assign5300_e3921);
                let assign5300_e3923: f64 = (assign5300_e3922).ln();
                let assign5300_e3924: f64 = (assign5300_e3917 * assign5300_e3923);
                let assign5300_e3925: f64 = (1.0 + assign5300_e3924);
                (assign5300_e3925,)
            } else {
                (0.001,)
            }
        };
        let assign5300_e3928: f64 = (p.p208 * assign5300_e3927);
        (assign5300_e3928,)
    } else {
        (locals.var_nsub0e,)
    }
};
        locals.var_nsub0e = assign5300_e3930;
        locals.var_nsub0e_rv = 0.0;

        let (assign5310_e3963,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5310_e3936: f64 = (p.p212 * locals.var_iwe);
        let assign5310_e3940: f64 = (locals.var_we / p.p213);
        let assign5310_e3941: f64 = (1.0 + assign5310_e3940);
        let assign5310_e3942: f64 = (assign5310_e3941).ln();
        let assign5310_e3943: f64 = (assign5310_e3936 * assign5310_e3942);
        let assign5310_e3944: f64 = (1.0 + assign5310_e3943);
        let (assign5310_e3960,) = {
            if (assign5310_e3944 > 0.001) {
                let assign5310_e3950: f64 = (p.p212 * locals.var_iwe);
                let assign5310_e3954: f64 = (locals.var_we / p.p213);
                let assign5310_e3955: f64 = (1.0 + assign5310_e3954);
                let assign5310_e3956: f64 = (assign5310_e3955).ln();
                let assign5310_e3957: f64 = (assign5310_e3950 * assign5310_e3956);
                let assign5310_e3958: f64 = (1.0 + assign5310_e3957);
                (assign5310_e3958,)
            } else {
                (0.001,)
            }
        };
        let assign5310_e3961: f64 = (p.p211 * assign5310_e3960);
        (assign5310_e3961,)
    } else {
        (locals.var_npcke,)
    }
};
        locals.var_npcke = assign5310_e3963;
        locals.var_npcke_rv = 0.0;

        let (assign5320_e3996,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5320_e3969: f64 = (p.p215 * locals.var_iwe);
        let assign5320_e3973: f64 = (locals.var_we / p.p213);
        let assign5320_e3974: f64 = (1.0 + assign5320_e3973);
        let assign5320_e3975: f64 = (assign5320_e3974).ln();
        let assign5320_e3976: f64 = (assign5320_e3969 * assign5320_e3975);
        let assign5320_e3977: f64 = (1.0 + assign5320_e3976);
        let (assign5320_e3993,) = {
            if (assign5320_e3977 > 0.001) {
                let assign5320_e3983: f64 = (p.p215 * locals.var_iwe);
                let assign5320_e3987: f64 = (locals.var_we / p.p213);
                let assign5320_e3988: f64 = (1.0 + assign5320_e3987);
                let assign5320_e3989: f64 = (assign5320_e3988).ln();
                let assign5320_e3990: f64 = (assign5320_e3983 * assign5320_e3989);
                let assign5320_e3991: f64 = (1.0 + assign5320_e3990);
                (assign5320_e3991,)
            } else {
                (0.001,)
            }
        };
        let assign5320_e3994: f64 = (p.p214 * assign5320_e3993);
        (assign5320_e3994,)
    } else {
        (locals.var_lpcke,)
    }
};
        locals.var_lpcke = assign5320_e3996;
        locals.var_lpcke_rv = 0.0;

        let assign5330_e4000: f64 = (2.0 * locals.var_lpcke);
        let assign5330_e4001: f64 = if locals.var_le > assign5330_e4000 { 1.0 } else { 0.0 };
        locals.var_guard37 = assign5330_e4001;
        locals.var_guard37_rv = 0.0;

        let (assign5340_e4007,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard37 != 0.0)) {
        (75000000000.0,)
    } else {
        (locals.var_aa,)
    }
};
        locals.var_aa = assign5340_e4007;
        locals.var_aa_rv = 0.0;

        let (assign5350_e4021,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard37 != 0.0)) {
        let assign5350_e4014: f64 = (0.5 * locals.var_npcke);
        let assign5350_e4015: f64 = (locals.var_nsub0e + assign5350_e4014);
        let assign5350_e4016: f64 = (assign5350_e4015).sqrt();
        let assign5350_e4018: f64 = (locals.var_nsub0e).sqrt();
        let assign5350_e4019: f64 = (assign5350_e4016 - assign5350_e4018);
        (assign5350_e4019,)
    } else {
        (locals.var_bb,)
    }
};
        locals.var_bb = assign5350_e4021;
        locals.var_bb_rv = 0.0;

        let (assign5360_e4046,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard37 != 0.0)) {
        let assign5360_e4026: f64 = (locals.var_nsub0e).sqrt();
        let assign5360_e4031: f64 = (2.0 * locals.var_lpcke);
        let assign5360_e4033: f64 = (assign5360_e4031 / locals.var_le);
        let assign5360_e4036: f64 = (locals.var_bb / locals.var_aa);
        let assign5360_e4037: f64 = (assign5360_e4036).exp();
        let assign5360_e4039: f64 = (assign5360_e4037 - 1.0);
        let assign5360_e4040: f64 = (assign5360_e4033 * assign5360_e4039);
        let assign5360_e4041: f64 = (1.0 + assign5360_e4040);
        let assign5360_e4042: f64 = (assign5360_e4041).ln();
        let assign5360_e4043: f64 = (locals.var_aa * assign5360_e4042);
        let assign5360_e4044: f64 = (assign5360_e4026 + assign5360_e4043);
        (assign5360_e4044,)
    } else {
        (locals.var_nsub,)
    }
};
        locals.var_nsub = assign5360_e4046;
        locals.var_nsub_rv = 0.0;

        let (assign5370_e4054,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard37 != 0.0)) {
        let assign5370_e4052: f64 = (locals.var_nsub * locals.var_nsub);
        (assign5370_e4052,)
    } else {
        (locals.var_nsub,)
    }
};
        locals.var_nsub = assign5370_e4054;
        locals.var_nsub_rv = 0.0;

        let assign5380_e4057: f64 = if locals.var_le >= locals.var_lpcke { 1.0 } else { 0.0 };
        locals.var_guard38 = assign5380_e4057;
        locals.var_guard38_rv = 0.0;

        let (assign5390_e4072,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard37 == 0.0)) && (locals.var_guard38 != 0.0)) {
        let assign5390_e4067: f64 = (locals.var_npcke * locals.var_lpcke);
        let assign5390_e4069: f64 = (assign5390_e4067 / locals.var_le);
        let assign5390_e4070: f64 = (locals.var_nsub0e + assign5390_e4069);
        (assign5390_e4070,)
    } else {
        (locals.var_nsub,)
    }
};
        locals.var_nsub = assign5390_e4072;
        locals.var_nsub_rv = 0.0;

        let (assign5400_e4090,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard37 == 0.0)) && (locals.var_guard38 == 0.0)) {
        let assign5400_e4085: f64 = (locals.var_le / locals.var_lpcke);
        let assign5400_e4086: f64 = (2.0 - assign5400_e4085);
        let assign5400_e4087: f64 = (locals.var_npcke * assign5400_e4086);
        let assign5400_e4088: f64 = (locals.var_nsub0e + assign5400_e4087);
        (assign5400_e4088,)
    } else {
        (locals.var_nsub,)
    }
};
        locals.var_nsub = assign5400_e4090;
        locals.var_nsub_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_2(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign5410_e4104,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5410_e4096: f64 = (p.p216 * locals.var_ile);
        let assign5410_e4097: f64 = (1.0 - assign5410_e4096);
        let assign5410_e4100: f64 = (p.p217 * locals.var_ile2);
        let assign5410_e4101: f64 = (assign5410_e4097 - assign5410_e4100);
        let assign5410_e4102: f64 = (locals.var_nsub * assign5410_e4101);
        (assign5410_e4102,)
    } else {
        (locals.var_neff_p,)
    }
};
        locals.var_neff_p = assign5410_e4104;
        locals.var_neff_p_rv = 0.0;

        let (assign5420_e4122,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5420_e4110: f64 = (locals.var_ile).powf(p.p220);
        let assign5420_e4111: f64 = (p.p219 * assign5420_e4110);
        let assign5420_e4112: f64 = (p.p218 + assign5420_e4111);
        let assign5420_e4115: f64 = (p.p221 * locals.var_iwe);
        let assign5420_e4116: f64 = (assign5420_e4112 + assign5420_e4115);
        let assign5420_e4119: f64 = (p.p222 * locals.var_iae);
        let assign5420_e4120: f64 = (assign5420_e4116 + assign5420_e4119);
        (assign5420_e4120,)
    } else {
        (locals.var_gfacnud_p,)
    }
};
        locals.var_gfacnud_p = assign5420_e4122;
        locals.var_gfacnud_p_rv = 0.0;

        let (assign5430_e4126,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p223,)
    } else {
        (locals.var_vsbnud_p,)
    }
};
        locals.var_vsbnud_p = assign5430_e4126;
        locals.var_vsbnud_p_rv = 0.0;

        let (assign5440_e4130,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p224,)
    } else {
        (locals.var_dvsbnud_p,)
    }
};
        locals.var_dvsbnud_p = assign5440_e4130;
        locals.var_dvsbnud_p_rv = 0.0;

        let (assign5450_e4148,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5450_e4136: f64 = (locals.var_ile).powf(p.p227);
        let assign5450_e4137: f64 = (p.p226 * assign5450_e4136);
        let assign5450_e4138: f64 = (p.p225 + assign5450_e4137);
        let assign5450_e4141: f64 = (p.p228 * locals.var_iwe);
        let assign5450_e4142: f64 = (assign5450_e4138 + assign5450_e4141);
        let assign5450_e4145: f64 = (p.p229 * locals.var_iae);
        let assign5450_e4146: f64 = (assign5450_e4142 + assign5450_e4145);
        (assign5450_e4146,)
    } else {
        (locals.var_dphib_p,)
    }
};
        locals.var_dphib_p = assign5450_e4148;
        locals.var_dphib_p_rv = 0.0;

        let (assign5460_e4167,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5460_e4155: f64 = (p.p231 * locals.var_ile);
        let assign5460_e4156: f64 = (1.0 + assign5460_e4155);
        let (assign5460_e4164,) = {
            if (1e-6 > assign5460_e4156) {
                (1e-6,)
            } else {
                let assign5460_e4162: f64 = (p.p231 * locals.var_ile);
                let assign5460_e4163: f64 = (1.0 + assign5460_e4162);
                (assign5460_e4163,)
            }
        };
        let assign5460_e4165: f64 = (p.p230 * assign5460_e4164);
        (assign5460_e4165,)
    } else {
        (locals.var_np_p,)
    }
};
        locals.var_np_p = assign5460_e4167;
        locals.var_np_p_rv = 0.0;

        let (assign5470_e4171,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p232,)
    } else {
        (locals.var_toxov_p,)
    }
};
        locals.var_toxov_p = assign5470_e4171;
        locals.var_toxov_p_rv = 0.0;

        let (assign5480_e4175,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p233,)
    } else {
        (locals.var_toxovd_p,)
    }
};
        locals.var_toxovd_p = assign5480_e4175;
        locals.var_toxovd_p_rv = 0.0;

        let (assign5490_e4179,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p236,)
    } else {
        (locals.var_nov_p,)
    }
};
        locals.var_nov_p = assign5490_e4179;
        locals.var_nov_p_rv = 0.0;

        let (assign5500_e4183,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p237,)
    } else {
        (locals.var_novd_p,)
    }
};
        locals.var_novd_p = assign5500_e4183;
        locals.var_novd_p_rv = 0.0;

        let (assign5510_e4205,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5510_e4189: f64 = (locals.var_ile).powf(p.p240);
        let assign5510_e4190: f64 = (p.p239 * assign5510_e4189);
        let assign5510_e4191: f64 = (p.p238 + assign5510_e4190);
        let assign5510_e4195: f64 = (p.p241 * locals.var_iwe);
        let assign5510_e4196: f64 = (1.0 + assign5510_e4195);
        let assign5510_e4197: f64 = (assign5510_e4191 * assign5510_e4196);
        let assign5510_e4201: f64 = (p.p242 * locals.var_iae);
        let assign5510_e4202: f64 = (1.0 + assign5510_e4201);
        let assign5510_e4203: f64 = (assign5510_e4197 * assign5510_e4202);
        (assign5510_e4203,)
    } else {
        (locals.var_ct_p,)
    }
};
        locals.var_ct_p = assign5510_e4205;
        locals.var_ct_p_rv = 0.0;

        let (assign5520_e4209,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p244,)
    } else {
        (locals.var_ctg_p,)
    }
};
        locals.var_ctg_p = assign5520_e4209;
        locals.var_ctg_p_rv = 0.0;

        let (assign5530_e4213,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p243,)
    } else {
        (locals.var_ctb_p,)
    }
};
        locals.var_ctb_p = assign5530_e4213;
        locals.var_ctb_p_rv = 0.0;

        let (assign5540_e4217,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p245,)
    } else {
        (locals.var_stct_p,)
    }
};
        locals.var_stct_p = assign5540_e4217;
        locals.var_stct_p_rv = 0.0;

        let (assign5550_e4231,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5550_e4222: f64 = (locals.var_ile).powf(p.p247);
        let assign5550_e4223: f64 = (p.p246 * assign5550_e4222);
        let assign5550_e4227: f64 = (p.p248 * locals.var_iwe);
        let assign5550_e4228: f64 = (1.0 + assign5550_e4227);
        let assign5550_e4229: f64 = (assign5550_e4223 * assign5550_e4228);
        (assign5550_e4229,)
    } else {
        (locals.var_cf_p,)
    }
};
        locals.var_cf_p = assign5550_e4231;
        locals.var_cf_p_rv = 0.0;

        let (assign5560_e4235,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p250,)
    } else {
        (locals.var_cfd_p,)
    }
};
        locals.var_cfd_p = assign5560_e4235;
        locals.var_cfd_p_rv = 0.0;

        let (assign5570_e4239,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p249,)
    } else {
        (locals.var_cfb_p,)
    }
};
        locals.var_cfb_p = assign5570_e4239;
        locals.var_cfb_p_rv = 0.0;

        let (assign5580_e4253,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5580_e4244: f64 = (locals.var_ile).powf(p.p252);
        let assign5580_e4245: f64 = (p.p251 * assign5580_e4244);
        let assign5580_e4249: f64 = (p.p253 * locals.var_iwe);
        let assign5580_e4250: f64 = (1.0 + assign5580_e4249);
        let assign5580_e4251: f64 = (assign5580_e4245 * assign5580_e4250);
        (assign5580_e4251,)
    } else {
        (locals.var_psce_p,)
    }
};
        locals.var_psce_p = assign5580_e4253;
        locals.var_psce_p_rv = 0.0;

        let (assign5590_e4257,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p255,)
    } else {
        (locals.var_psced_p,)
    }
};
        locals.var_psced_p = assign5590_e4257;
        locals.var_psced_p_rv = 0.0;

        let (assign5600_e4261,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p254,)
    } else {
        (locals.var_psceb_p,)
    }
};
        locals.var_psceb_p = assign5600_e4261;
        locals.var_psceb_p_rv = 0.0;

        let (assign5610_e4271,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5610_e4267: f64 = (p.p258 * locals.var_iwe);
        let assign5610_e4268: f64 = (1.0 + assign5610_e4267);
        let assign5610_e4269: f64 = (p.p257 * assign5610_e4268);
        (assign5610_e4269,)
    } else {
        (locals.var_fbet1e,)
    }
};
        locals.var_fbet1e = assign5610_e4271;
        locals.var_fbet1e_rv = 0.0;

        let (assign5620_e4290,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5620_e4277: f64 = (p.p260 * locals.var_iwe);
        let assign5620_e4278: f64 = (1.0 + assign5620_e4277);
        let (assign5620_e4287,) = {
            if (assign5620_e4278 > 0.001) {
                let assign5620_e4284: f64 = (p.p260 * locals.var_iwe);
                let assign5620_e4285: f64 = (1.0 + assign5620_e4284);
                (assign5620_e4285,)
            } else {
                (0.001,)
            }
        };
        let assign5620_e4288: f64 = (p.p259 * assign5620_e4287);
        (assign5620_e4288,)
    } else {
        (locals.var_lp1e,)
    }
};
        locals.var_lp1e = assign5620_e4290;
        locals.var_lp1e_rv = 0.0;

        let (assign5630_e4322,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5630_e4295: f64 = (locals.var_fbet1e * locals.var_lp1e);
        let assign5630_e4297: f64 = (assign5630_e4295 / locals.var_le);
        let assign5630_e4300: f64 = (-locals.var_le);
        let assign5630_e4302: f64 = (assign5630_e4300 / locals.var_lp1e);
        let assign5630_e4303: f64 = (assign5630_e4302).exp();
        let assign5630_e4304: f64 = (1.0 - assign5630_e4303);
        let assign5630_e4305: f64 = (assign5630_e4297 * assign5630_e4304);
        let assign5630_e4306: f64 = (1.0 + assign5630_e4305);
        let assign5630_e4309: f64 = (p.p261 * p.p262);
        let assign5630_e4311: f64 = (assign5630_e4309 / locals.var_le);
        let assign5630_e4314: f64 = (-locals.var_le);
        let assign5630_e4316: f64 = (assign5630_e4314 / p.p262);
        let assign5630_e4317: f64 = (assign5630_e4316).exp();
        let assign5630_e4318: f64 = (1.0 - assign5630_e4317);
        let assign5630_e4319: f64 = (assign5630_e4311 * assign5630_e4318);
        let assign5630_e4320: f64 = (assign5630_e4306 + assign5630_e4319);
        (assign5630_e4320,)
    } else {
        (locals.var_gpe,)
    }
};
        locals.var_gpe = assign5630_e4322;
        locals.var_gpe_rv = 0.0;

        let (assign5640_e4331,) = {
    if (locals.var_guard36 != 0.0) {
        let (assign5640_e4329,) = {
            if (locals.var_gpe > 1e-15) {
                (locals.var_gpe,)
            } else {
                (1e-15,)
            }
        };
        (assign5640_e4329,)
    } else {
        (locals.var_gpe,)
    }
};
        locals.var_gpe = assign5640_e4331;
        locals.var_gpe_rv = 0.0;

        let (assign5650_e4350,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5650_e4336: f64 = (p.p263 * locals.var_iwe);
        let assign5650_e4337: f64 = (1.0 + assign5650_e4336);
        let assign5650_e4340: f64 = (p.p264 * locals.var_iwe);
        let assign5650_e4344: f64 = (locals.var_we / p.p265);
        let assign5650_e4345: f64 = (1.0 + assign5650_e4344);
        let assign5650_e4346: f64 = (assign5650_e4345).ln();
        let assign5650_e4347: f64 = (assign5650_e4340 * assign5650_e4346);
        let assign5650_e4348: f64 = (assign5650_e4337 + assign5650_e4347);
        (assign5650_e4348,)
    } else {
        (locals.var_gwe,)
    }
};
        locals.var_gwe = assign5650_e4350;
        locals.var_gwe_rv = 0.0;

        let (assign5660_e4362,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5660_e4354: f64 = (p.p256 * locals.var_we);
        let assign5660_e4357: f64 = (locals.var_gpe * locals.var_le);
        let assign5660_e4358: f64 = (assign5660_e4354 / assign5660_e4357);
        let assign5660_e4360: f64 = (assign5660_e4358 * locals.var_gwe);
        (assign5660_e4360,)
    } else {
        (locals.var_betn_p,)
    }
};
        locals.var_betn_p = assign5660_e4362;
        locals.var_betn_p_rv = 0.0;

        let (assign5670_e4378,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5670_e4367: f64 = (p.p267 * locals.var_ile);
        let assign5670_e4368: f64 = (p.p266 + assign5670_e4367);
        let assign5670_e4371: f64 = (p.p268 * locals.var_iwe);
        let assign5670_e4372: f64 = (assign5670_e4368 + assign5670_e4371);
        let assign5670_e4375: f64 = (p.p269 * locals.var_iae);
        let assign5670_e4376: f64 = (assign5670_e4372 + assign5670_e4375);
        (assign5670_e4376,)
    } else {
        (locals.var_stbet_p,)
    }
};
        locals.var_stbet_p = assign5670_e4378;
        locals.var_stbet_p_rv = 0.0;

        let (assign5680_e4388,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5680_e4384: f64 = (p.p271 * locals.var_iwe);
        let assign5680_e4385: f64 = (1.0 + assign5680_e4384);
        let assign5680_e4386: f64 = (p.p270 * assign5680_e4385);
        (assign5680_e4386,)
    } else {
        (locals.var_mue_p,)
    }
};
        locals.var_mue_p = assign5680_e4388;
        locals.var_mue_p_rv = 0.0;

        let (assign5690_e4392,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p272,)
    } else {
        (locals.var_stmue_p,)
    }
};
        locals.var_stmue_p = assign5690_e4392;
        locals.var_stmue_p_rv = 0.0;

        let (assign5700_e4396,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p273,)
    } else {
        (locals.var_themu_p,)
    }
};
        locals.var_themu_p = assign5700_e4396;
        locals.var_themu_p_rv = 0.0;

        let (assign5710_e4400,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p274,)
    } else {
        (locals.var_stthemu_p,)
    }
};
        locals.var_stthemu_p = assign5710_e4400;
        locals.var_stthemu_p_rv = 0.0;

        let (assign5720_e4422,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5720_e4406: f64 = (locals.var_ile).powf(p.p277);
        let assign5720_e4407: f64 = (p.p276 * assign5720_e4406);
        let assign5720_e4408: f64 = (p.p275 + assign5720_e4407);
        let assign5720_e4412: f64 = (p.p278 * locals.var_iwe);
        let assign5720_e4413: f64 = (1.0 + assign5720_e4412);
        let assign5720_e4414: f64 = (assign5720_e4408 * assign5720_e4413);
        let assign5720_e4418: f64 = (p.p279 * locals.var_iae);
        let assign5720_e4419: f64 = (1.0 + assign5720_e4418);
        let assign5720_e4420: f64 = (assign5720_e4414 * assign5720_e4419);
        (assign5720_e4420,)
    } else {
        (locals.var_cs_p,)
    }
};
        locals.var_cs_p = assign5720_e4422;
        locals.var_cs_p_rv = 0.0;

        let (assign5730_e4426,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p280,)
    } else {
        (locals.var_stcs_p,)
    }
};
        locals.var_stcs_p = assign5730_e4426;
        locals.var_stcs_p_rv = 0.0;

        let (assign5740_e4430,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p281,)
    } else {
        (locals.var_thecs_p,)
    }
};
        locals.var_thecs_p = assign5740_e4430;
        locals.var_thecs_p_rv = 0.0;

        let (assign5750_e4434,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p282,)
    } else {
        (locals.var_stthecs_p,)
    }
};
        locals.var_stthecs_p = assign5750_e4434;
        locals.var_stthecs_p_rv = 0.0;

        let (assign5760_e4456,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5760_e4440: f64 = (p.p284 * locals.var_ile);
        let assign5760_e4441: f64 = (1.0 + assign5760_e4440);
        let assign5760_e4442: f64 = (p.p283 * assign5760_e4441);
        let assign5760_e4446: f64 = (p.p285 * locals.var_iwe);
        let assign5760_e4447: f64 = (1.0 + assign5760_e4446);
        let assign5760_e4448: f64 = (assign5760_e4442 * assign5760_e4447);
        let assign5760_e4452: f64 = (p.p286 * locals.var_iae);
        let assign5760_e4453: f64 = (1.0 + assign5760_e4452);
        let assign5760_e4454: f64 = (assign5760_e4448 * assign5760_e4453);
        (assign5760_e4454,)
    } else {
        (locals.var_xcor_p,)
    }
};
        locals.var_xcor_p = assign5760_e4456;
        locals.var_xcor_p_rv = 0.0;

        let (assign5770_e4460,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p287,)
    } else {
        (locals.var_stxcor_p,)
    }
};
        locals.var_stxcor_p = assign5770_e4460;
        locals.var_stxcor_p_rv = 0.0;

        let (assign5780_e4464,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p288,)
    } else {
        (locals.var_feta_p,)
    }
};
        locals.var_feta_p = assign5780_e4464;
        locals.var_feta_p_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_3(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign5790_e4476,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5790_e4468: f64 = (p.p289 * locals.var_iwe);
        let assign5790_e4472: f64 = (p.p290 * locals.var_iwe);
        let assign5790_e4473: f64 = (1.0 + assign5790_e4472);
        let assign5790_e4474: f64 = (assign5790_e4468 * assign5790_e4473);
        (assign5790_e4474,)
    } else {
        (locals.var_rs_p,)
    }
};
        locals.var_rs_p = assign5790_e4476;
        locals.var_rs_p_rv = 0.0;

        let (assign5800_e4480,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p291,)
    } else {
        (locals.var_strs_p,)
    }
};
        locals.var_strs_p = assign5800_e4480;
        locals.var_strs_p_rv = 0.0;

        let (assign5810_e4484,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p292,)
    } else {
        (locals.var_rsb_p,)
    }
};
        locals.var_rsb_p = assign5810_e4484;
        locals.var_rsb_p_rv = 0.0;

        let (assign5820_e4488,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p293,)
    } else {
        (locals.var_rsg_p,)
    }
};
        locals.var_rsg_p = assign5820_e4488;
        locals.var_rsg_p_rv = 0.0;

        let (assign5830_e4514,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5830_e4493: f64 = (p.p295 * locals.var_gwe);
        let assign5830_e4495: f64 = (assign5830_e4493 / locals.var_gpe);
        let assign5830_e4498: f64 = (locals.var_ile).powf(p.p296);
        let assign5830_e4499: f64 = (assign5830_e4495 * assign5830_e4498);
        let assign5830_e4500: f64 = (p.p294 + assign5830_e4499);
        let assign5830_e4504: f64 = (p.p297 * locals.var_iwe);
        let assign5830_e4505: f64 = (1.0 + assign5830_e4504);
        let assign5830_e4506: f64 = (assign5830_e4500 * assign5830_e4505);
        let assign5830_e4510: f64 = (p.p298 * locals.var_iae);
        let assign5830_e4511: f64 = (1.0 + assign5830_e4510);
        let assign5830_e4512: f64 = (assign5830_e4506 * assign5830_e4511);
        (assign5830_e4512,)
    } else {
        (locals.var_thesat_p,)
    }
};
        locals.var_thesat_p = assign5830_e4514;
        locals.var_thesat_p_rv = 0.0;

        let (assign5840_e4530,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5840_e4519: f64 = (p.p300 * locals.var_ile);
        let assign5840_e4520: f64 = (p.p299 + assign5840_e4519);
        let assign5840_e4523: f64 = (p.p301 * locals.var_iwe);
        let assign5840_e4524: f64 = (assign5840_e4520 + assign5840_e4523);
        let assign5840_e4527: f64 = (p.p302 * locals.var_iae);
        let assign5840_e4528: f64 = (assign5840_e4524 + assign5840_e4527);
        (assign5840_e4528,)
    } else {
        (locals.var_stthesat_p,)
    }
};
        locals.var_stthesat_p = assign5840_e4530;
        locals.var_stthesat_p_rv = 0.0;

        let (assign5850_e4534,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p303,)
    } else {
        (locals.var_thesatb_p,)
    }
};
        locals.var_thesatb_p = assign5850_e4534;
        locals.var_thesatb_p_rv = 0.0;

        let (assign5860_e4538,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p304,)
    } else {
        (locals.var_thesatg_p,)
    }
};
        locals.var_thesatg_p = assign5860_e4538;
        locals.var_thesatg_p_rv = 0.0;

        let (assign5870_e4542,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p305,)
    } else {
        (locals.var_thesatt_p,)
    }
};
        locals.var_thesatt_p = assign5870_e4542;
        locals.var_thesatt_p_rv = 0.0;

        let (assign5880_e4552,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5880_e4548: f64 = (p.p307 * locals.var_ile);
        let assign5880_e4549: f64 = (1.0 + assign5880_e4548);
        let assign5880_e4550: f64 = (p.p306 / assign5880_e4549);
        (assign5880_e4550,)
    } else {
        (locals.var_ax_p,)
    }
};
        locals.var_ax_p = assign5880_e4552;
        locals.var_ax_p_rv = 0.0;

        let (assign5890_e4566,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5890_e4557: f64 = (locals.var_ile).powf(p.p309);
        let assign5890_e4558: f64 = (p.p308 * assign5890_e4557);
        let assign5890_e4562: f64 = (p.p310 * locals.var_iwe);
        let assign5890_e4563: f64 = (1.0 + assign5890_e4562);
        let assign5890_e4564: f64 = (assign5890_e4558 * assign5890_e4563);
        (assign5890_e4564,)
    } else {
        (locals.var_alp_p,)
    }
};
        locals.var_alp_p = assign5890_e4566;
        locals.var_alp_p_rv = 0.0;

        let (assign5900_e4572,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5900_e4570: f64 = (locals.var_ile).powf(p.p312);
        (assign5900_e4570,)
    } else {
        (locals.var_tmpx,)
    }
};
        locals.var_tmpx = assign5900_e4572;
        locals.var_tmpx_rv = 0.0;

        let (assign5910_e4592,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5910_e4576: f64 = (p.p311 * locals.var_tmpx);
        let assign5910_e4580: f64 = (p.p314 * locals.var_iwe);
        let assign5910_e4581: f64 = (1.0 + assign5910_e4580);
        let assign5910_e4582: f64 = (assign5910_e4576 * assign5910_e4581);
        let assign5910_e4586: f64 = (p.p313 * locals.var_ile);
        let assign5910_e4588: f64 = (assign5910_e4586 * locals.var_tmpx);
        let assign5910_e4589: f64 = (1.0 + assign5910_e4588);
        let assign5910_e4590: f64 = (assign5910_e4582 / assign5910_e4589);
        (assign5910_e4590,)
    } else {
        (locals.var_alp1_p,)
    }
};
        locals.var_alp1_p = assign5910_e4592;
        locals.var_alp1_p_rv = 0.0;

        let (assign5920_e4598,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5920_e4596: f64 = (locals.var_ile).powf(p.p316);
        (assign5920_e4596,)
    } else {
        (locals.var_tmpx,)
    }
};
        locals.var_tmpx = assign5920_e4598;
        locals.var_tmpx_rv = 0.0;

        let (assign5930_e4618,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5930_e4602: f64 = (p.p315 * locals.var_tmpx);
        let assign5930_e4606: f64 = (p.p318 * locals.var_iwe);
        let assign5930_e4607: f64 = (1.0 + assign5930_e4606);
        let assign5930_e4608: f64 = (assign5930_e4602 * assign5930_e4607);
        let assign5930_e4612: f64 = (p.p317 * locals.var_ile);
        let assign5930_e4614: f64 = (assign5930_e4612 * locals.var_tmpx);
        let assign5930_e4615: f64 = (1.0 + assign5930_e4614);
        let assign5930_e4616: f64 = (assign5930_e4608 / assign5930_e4615);
        (assign5930_e4616,)
    } else {
        (locals.var_alp2_p,)
    }
};
        locals.var_alp2_p = assign5930_e4618;
        locals.var_alp2_p_rv = 0.0;

        let (assign5940_e4622,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p319,)
    } else {
        (locals.var_vp_p,)
    }
};
        locals.var_vp_p = assign5940_e4622;
        locals.var_vp_p_rv = 0.0;

        let (assign5950_e4638,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5950_e4628: f64 = (p.p321 * locals.var_ile);
        let assign5950_e4629: f64 = (1.0 + assign5950_e4628);
        let assign5950_e4630: f64 = (p.p320 * assign5950_e4629);
        let assign5950_e4634: f64 = (p.p322 * locals.var_iwe);
        let assign5950_e4635: f64 = (1.0 + assign5950_e4634);
        let assign5950_e4636: f64 = (assign5950_e4630 * assign5950_e4635);
        (assign5950_e4636,)
    } else {
        (locals.var_a1_p,)
    }
};
        locals.var_a1_p = assign5950_e4638;
        locals.var_a1_p_rv = 0.0;

        let (assign5960_e4642,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p323,)
    } else {
        (locals.var_a2_p,)
    }
};
        locals.var_a2_p = assign5960_e4642;
        locals.var_a2_p_rv = 0.0;

        let (assign5970_e4646,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p324,)
    } else {
        (locals.var_sta2_p,)
    }
};
        locals.var_sta2_p = assign5970_e4646;
        locals.var_sta2_p_rv = 0.0;

        let (assign5980_e4662,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5980_e4652: f64 = (p.p326 * locals.var_ile);
        let assign5980_e4653: f64 = (1.0 + assign5980_e4652);
        let assign5980_e4654: f64 = (p.p325 * assign5980_e4653);
        let assign5980_e4658: f64 = (p.p327 * locals.var_iwe);
        let assign5980_e4659: f64 = (1.0 + assign5980_e4658);
        let assign5980_e4660: f64 = (assign5980_e4654 * assign5980_e4659);
        (assign5980_e4660,)
    } else {
        (locals.var_a3_p,)
    }
};
        locals.var_a3_p = assign5980_e4662;
        locals.var_a3_p_rv = 0.0;

        let (assign5990_e4678,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5990_e4668: f64 = (p.p329 * locals.var_ile);
        let assign5990_e4669: f64 = (1.0 + assign5990_e4668);
        let assign5990_e4670: f64 = (p.p328 * assign5990_e4669);
        let assign5990_e4674: f64 = (p.p330 * locals.var_iwe);
        let assign5990_e4675: f64 = (1.0 + assign5990_e4674);
        let assign5990_e4676: f64 = (assign5990_e4670 * assign5990_e4675);
        (assign5990_e4676,)
    } else {
        (locals.var_a4_p,)
    }
};
        locals.var_a4_p = assign5990_e4678;
        locals.var_a4_p_rv = 0.0;

        let (assign6000_e4682,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p331,)
    } else {
        (locals.var_imaxii_p,)
    }
};
        locals.var_imaxii_p = assign6000_e4682;
        locals.var_imaxii_p_rv = 0.0;

        let (assign6010_e4686,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p332,)
    } else {
        (locals.var_gco_p,)
    }
};
        locals.var_gco_p = assign6010_e4686;
        locals.var_gco_p_rv = 0.0;

        let (assign6020_e4692,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6020_e4690: f64 = (p.p333 / locals.var_iae);
        (assign6020_e4690,)
    } else {
        (locals.var_iginv_p,)
    }
};
        locals.var_iginv_p = assign6020_e4692;
        locals.var_iginv_p_rv = 0.0;

        let (assign6030_e4702,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6030_e4696: f64 = (p.p334 * p.p234);
        let assign6030_e4699: f64 = (1e-6 * locals.var_iwe);
        let assign6030_e4700: f64 = (assign6030_e4696 / assign6030_e4699);
        (assign6030_e4700,)
    } else {
        (locals.var_igov_p,)
    }
};
        locals.var_igov_p = assign6030_e4702;
        locals.var_igov_p_rv = 0.0;

        let (assign6040_e4712,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6040_e4706: f64 = (p.p335 * p.p235);
        let assign6040_e4709: f64 = (1e-6 * locals.var_iwe);
        let assign6040_e4710: f64 = (assign6040_e4706 / assign6040_e4709);
        (assign6040_e4710,)
    } else {
        (locals.var_igovd_p,)
    }
};
        locals.var_igovd_p = assign6040_e4712;
        locals.var_igovd_p_rv = 0.0;

        let (assign6050_e4716,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p336,)
    } else {
        (locals.var_stig_p,)
    }
};
        locals.var_stig_p = assign6050_e4716;
        locals.var_stig_p_rv = 0.0;

        let (assign6060_e4720,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p337,)
    } else {
        (locals.var_gc2_p,)
    }
};
        locals.var_gc2_p = assign6060_e4720;
        locals.var_gc2_p_rv = 0.0;

        let (assign6070_e4724,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p338,)
    } else {
        (locals.var_gc3_p,)
    }
};
        locals.var_gc3_p = assign6070_e4724;
        locals.var_gc3_p_rv = 0.0;

        let (assign6080_e4728,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p337,)
    } else {
        (locals.var_gc2ov_p,)
    }
};
        locals.var_gc2ov_p = assign6080_e4728;
        locals.var_gc2ov_p_rv = 0.0;

        let assign6090_e4730: f64 = if param_given[339] { 1.0 } else { 0.0 };
        let assign6090_e4732: f64 = if assign6090_e4730 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard39 = assign6090_e4732;
        locals.var_guard39_rv = 0.0;

        let (assign6100_e4738,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard39 != 0.0)) {
        (p.p339,)
    } else {
        (locals.var_gc2ov_p,)
    }
};
        locals.var_gc2ov_p = assign6100_e4738;
        locals.var_gc2ov_p_rv = 0.0;

        let (assign6110_e4742,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p338,)
    } else {
        (locals.var_gc3ov_p,)
    }
};
        locals.var_gc3ov_p = assign6110_e4742;
        locals.var_gc3ov_p_rv = 0.0;

        let assign6120_e4744: f64 = if param_given[340] { 1.0 } else { 0.0 };
        let assign6120_e4746: f64 = if assign6120_e4744 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard40 = assign6120_e4746;
        locals.var_guard40_rv = 0.0;

        let (assign6130_e4752,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard40 != 0.0)) {
        (p.p340,)
    } else {
        (locals.var_gc3ov_p,)
    }
};
        locals.var_gc3ov_p = assign6130_e4752;
        locals.var_gc3ov_p_rv = 0.0;

        let (assign6140_e4756,) = {
    if (locals.var_guard36 != 0.0) {
        (locals.var_gc2ov_p,)
    } else {
        (locals.var_gc2ovd_p,)
    }
};
        locals.var_gc2ovd_p = assign6140_e4756;
        locals.var_gc2ovd_p_rv = 0.0;

        let assign6150_e4758: f64 = if param_given[341] { 1.0 } else { 0.0 };
        let assign6150_e4760: f64 = if assign6150_e4758 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard41 = assign6150_e4760;
        locals.var_guard41_rv = 0.0;

        let (assign6160_e4766,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard41 != 0.0)) {
        (p.p341,)
    } else {
        (locals.var_gc2ovd_p,)
    }
};
        locals.var_gc2ovd_p = assign6160_e4766;
        locals.var_gc2ovd_p_rv = 0.0;

        let (assign6170_e4770,) = {
    if (locals.var_guard36 != 0.0) {
        (locals.var_gc3ov_p,)
    } else {
        (locals.var_gc3ovd_p,)
    }
};
        locals.var_gc3ovd_p = assign6170_e4770;
        locals.var_gc3ovd_p_rv = 0.0;

        let assign6180_e4772: f64 = if param_given[342] { 1.0 } else { 0.0 };
        let assign6180_e4774: f64 = if assign6180_e4772 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard42 = assign6180_e4774;
        locals.var_guard42_rv = 0.0;

        let (assign6190_e4780,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard42 != 0.0)) {
        (p.p342,)
    } else {
        (locals.var_gc3ovd_p,)
    }
};
        locals.var_gc3ovd_p = assign6190_e4780;
        locals.var_gc3ovd_p_rv = 0.0;

        let (assign6200_e4784,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p343,)
    } else {
        (locals.var_chib_p,)
    }
};
        locals.var_chib_p = assign6200_e4784;
        locals.var_chib_p_rv = 0.0;

        let (assign6210_e4794,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6210_e4788: f64 = (p.p344 * p.p234);
        let assign6210_e4791: f64 = (1e-6 * locals.var_iwe);
        let assign6210_e4792: f64 = (assign6210_e4788 / assign6210_e4791);
        (assign6210_e4792,)
    } else {
        (locals.var_agidl_p,)
    }
};
        locals.var_agidl_p = assign6210_e4794;
        locals.var_agidl_p_rv = 0.0;

        let (assign6220_e4804,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6220_e4798: f64 = (p.p345 * p.p235);
        let assign6220_e4801: f64 = (1e-6 * locals.var_iwe);
        let assign6220_e4802: f64 = (assign6220_e4798 / assign6220_e4801);
        (assign6220_e4802,)
    } else {
        (locals.var_agidld_p,)
    }
};
        locals.var_agidld_p = assign6220_e4804;
        locals.var_agidld_p_rv = 0.0;

        let (assign6230_e4808,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p346,)
    } else {
        (locals.var_bgidl_p,)
    }
};
        locals.var_bgidl_p = assign6230_e4808;
        locals.var_bgidl_p_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_4(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign6240_e4812,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p347,)
    } else {
        (locals.var_bgidld_p,)
    }
};
        locals.var_bgidld_p = assign6240_e4812;
        locals.var_bgidld_p_rv = 0.0;

        let (assign6250_e4816,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p348,)
    } else {
        (locals.var_stbgidl_p,)
    }
};
        locals.var_stbgidl_p = assign6250_e4816;
        locals.var_stbgidl_p_rv = 0.0;

        let (assign6260_e4820,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p349,)
    } else {
        (locals.var_stbgidld_p,)
    }
};
        locals.var_stbgidld_p = assign6260_e4820;
        locals.var_stbgidld_p_rv = 0.0;

        let (assign6270_e4824,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p350,)
    } else {
        (locals.var_cgidl_p,)
    }
};
        locals.var_cgidl_p = assign6270_e4824;
        locals.var_cgidl_p_rv = 0.0;

        let (assign6280_e4828,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p351,)
    } else {
        (locals.var_cgidld_p,)
    }
};
        locals.var_cgidld_p = assign6280_e4828;
        locals.var_cgidld_p_rv = 0.0;

        let (assign6290_e4840,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6290_e4832: f64 = (8.8541878176e-12 * p.p207);
        let assign6290_e4834: f64 = (assign6290_e4832 * locals.var_wecv);
        let assign6290_e4836: f64 = (assign6290_e4834 * locals.var_lecv);
        let assign6290_e4838: f64 = (assign6290_e4836 / p.p206);
        (assign6290_e4838,)
    } else {
        (locals.var_cox_p,)
    }
};
        locals.var_cox_p = assign6290_e4840;
        locals.var_cox_p_rv = 0.0;

        let (assign6300_e4852,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6300_e4844: f64 = (8.8541878176e-12 * p.p207);
        let assign6300_e4846: f64 = (assign6300_e4844 * locals.var_wecv);
        let assign6300_e4848: f64 = (assign6300_e4846 * p.p234);
        let assign6300_e4850: f64 = (assign6300_e4848 / p.p232);
        (assign6300_e4850,)
    } else {
        (locals.var_cgov_p,)
    }
};
        locals.var_cgov_p = assign6300_e4852;
        locals.var_cgov_p_rv = 0.0;

        let (assign6310_e4864,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6310_e4856: f64 = (8.8541878176e-12 * p.p207);
        let assign6310_e4858: f64 = (assign6310_e4856 * locals.var_wecv);
        let assign6310_e4860: f64 = (assign6310_e4858 * p.p235);
        let assign6310_e4862: f64 = (assign6310_e4860 / p.p233);
        (assign6310_e4862,)
    } else {
        (locals.var_cgovd_p,)
    }
};
        locals.var_cgovd_p = assign6310_e4864;
        locals.var_cgovd_p_rv = 0.0;

        let (assign6320_e4882,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6320_e4870: f64 = (locals.var_ile).powf(p.p354);
        let assign6320_e4871: f64 = (p.p353 * assign6320_e4870);
        let assign6320_e4872: f64 = (p.p352 + assign6320_e4871);
        let assign6320_e4875: f64 = (p.p355 * locals.var_iwe);
        let assign6320_e4876: f64 = (assign6320_e4872 + assign6320_e4875);
        let assign6320_e4879: f64 = (p.p356 * locals.var_iae);
        let assign6320_e4880: f64 = (assign6320_e4876 + assign6320_e4879);
        (assign6320_e4880,)
    } else {
        (locals.var_delvtac_p,)
    }
};
        locals.var_delvtac_p = assign6320_e4882;
        locals.var_delvtac_p_rv = 0.0;

        let (assign6330_e4898,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6330_e4887: f64 = (p.p358 * locals.var_ile);
        let assign6330_e4888: f64 = (p.p357 + assign6330_e4887);
        let assign6330_e4891: f64 = (p.p359 * locals.var_iwe);
        let assign6330_e4892: f64 = (assign6330_e4888 + assign6330_e4891);
        let assign6330_e4895: f64 = (p.p360 * locals.var_iae);
        let assign6330_e4896: f64 = (assign6330_e4892 + assign6330_e4895);
        (assign6330_e4896,)
    } else {
        (locals.var_facneffac_p,)
    }
};
        locals.var_facneffac_p = assign6330_e4898;
        locals.var_facneffac_p_rv = 0.0;

        let (assign6340_e4902,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p294,)
    } else {
        (locals.var_thesataco_i,)
    }
};
        locals.var_thesataco_i = assign6340_e4902;
        locals.var_thesataco_i_rv = 0.0;

        let assign6350_e4904: f64 = if param_given[361] { 1.0 } else { 0.0 };
        let assign6350_e4906: f64 = if assign6350_e4904 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard43 = assign6350_e4906;
        locals.var_guard43_rv = 0.0;

        let (assign6360_e4912,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard43 != 0.0)) {
        (p.p361,)
    } else {
        (locals.var_thesataco_i,)
    }
};
        locals.var_thesataco_i = assign6360_e4912;
        locals.var_thesataco_i_rv = 0.0;

        let (assign6370_e4916,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p295,)
    } else {
        (locals.var_thesatacl_i,)
    }
};
        locals.var_thesatacl_i = assign6370_e4916;
        locals.var_thesatacl_i_rv = 0.0;

        let assign6380_e4918: f64 = if param_given[362] { 1.0 } else { 0.0 };
        let assign6380_e4920: f64 = if assign6380_e4918 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard44 = assign6380_e4920;
        locals.var_guard44_rv = 0.0;

        let (assign6390_e4926,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard44 != 0.0)) {
        (p.p362,)
    } else {
        (locals.var_thesatacl_i,)
    }
};
        locals.var_thesatacl_i = assign6390_e4926;
        locals.var_thesatacl_i_rv = 0.0;

        let (assign6400_e4930,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p296,)
    } else {
        (locals.var_thesataclexp_i,)
    }
};
        locals.var_thesataclexp_i = assign6400_e4930;
        locals.var_thesataclexp_i_rv = 0.0;

        let assign6410_e4932: f64 = if param_given[363] { 1.0 } else { 0.0 };
        let assign6410_e4934: f64 = if assign6410_e4932 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard45 = assign6410_e4934;
        locals.var_guard45_rv = 0.0;

        let (assign6420_e4940,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard45 != 0.0)) {
        (p.p363,)
    } else {
        (locals.var_thesataclexp_i,)
    }
};
        locals.var_thesataclexp_i = assign6420_e4940;
        locals.var_thesataclexp_i_rv = 0.0;

        let (assign6430_e4944,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p297,)
    } else {
        (locals.var_thesatacw_i,)
    }
};
        locals.var_thesatacw_i = assign6430_e4944;
        locals.var_thesatacw_i_rv = 0.0;

        let assign6440_e4946: f64 = if param_given[364] { 1.0 } else { 0.0 };
        let assign6440_e4948: f64 = if assign6440_e4946 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard46 = assign6440_e4948;
        locals.var_guard46_rv = 0.0;

        let (assign6450_e4954,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard46 != 0.0)) {
        (p.p364,)
    } else {
        (locals.var_thesatacw_i,)
    }
};
        locals.var_thesatacw_i = assign6450_e4954;
        locals.var_thesatacw_i_rv = 0.0;

        let (assign6460_e4958,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p298,)
    } else {
        (locals.var_thesataclw_i,)
    }
};
        locals.var_thesataclw_i = assign6460_e4958;
        locals.var_thesataclw_i_rv = 0.0;

        let assign6470_e4960: f64 = if param_given[365] { 1.0 } else { 0.0 };
        let assign6470_e4962: f64 = if assign6470_e4960 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard47 = assign6470_e4962;
        locals.var_guard47_rv = 0.0;

        let (assign6480_e4968,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard47 != 0.0)) {
        (p.p365,)
    } else {
        (locals.var_thesataclw_i,)
    }
};
        locals.var_thesataclw_i = assign6480_e4968;
        locals.var_thesataclw_i_rv = 0.0;

        let (assign6490_e4994,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6490_e4973: f64 = (locals.var_thesatacl_i * locals.var_gwe);
        let assign6490_e4975: f64 = (assign6490_e4973 / locals.var_gpe);
        let assign6490_e4978: f64 = (locals.var_ile).powf(locals.var_thesataclexp_i);
        let assign6490_e4979: f64 = (assign6490_e4975 * assign6490_e4978);
        let assign6490_e4980: f64 = (locals.var_thesataco_i + assign6490_e4979);
        let assign6490_e4984: f64 = (locals.var_thesatacw_i * locals.var_iwe);
        let assign6490_e4985: f64 = (1.0 + assign6490_e4984);
        let assign6490_e4986: f64 = (assign6490_e4980 * assign6490_e4985);
        let assign6490_e4990: f64 = (locals.var_thesataclw_i * locals.var_iae);
        let assign6490_e4991: f64 = (1.0 + assign6490_e4990);
        let assign6490_e4992: f64 = (assign6490_e4986 * assign6490_e4991);
        (assign6490_e4992,)
    } else {
        (locals.var_thesatac_p,)
    }
};
        locals.var_thesatac_p = assign6490_e4994;
        locals.var_thesatac_p_rv = 0.0;

        let (assign6500_e4998,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p306,)
    } else {
        (locals.var_axaco_i,)
    }
};
        locals.var_axaco_i = assign6500_e4998;
        locals.var_axaco_i_rv = 0.0;

        let assign6510_e5000: f64 = if param_given[366] { 1.0 } else { 0.0 };
        let assign6510_e5002: f64 = if assign6510_e5000 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard48 = assign6510_e5002;
        locals.var_guard48_rv = 0.0;

        let (assign6520_e5008,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard48 != 0.0)) {
        (p.p366,)
    } else {
        (locals.var_axaco_i,)
    }
};
        locals.var_axaco_i = assign6520_e5008;
        locals.var_axaco_i_rv = 0.0;

        let (assign6530_e5012,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p307,)
    } else {
        (locals.var_axacl_i,)
    }
};
        locals.var_axacl_i = assign6530_e5012;
        locals.var_axacl_i_rv = 0.0;

        let assign6540_e5014: f64 = if param_given[367] { 1.0 } else { 0.0 };
        let assign6540_e5016: f64 = if assign6540_e5014 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard49 = assign6540_e5016;
        locals.var_guard49_rv = 0.0;

        let (assign6550_e5022,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard49 != 0.0)) {
        (p.p367,)
    } else {
        (locals.var_axacl_i,)
    }
};
        locals.var_axacl_i = assign6550_e5022;
        locals.var_axacl_i_rv = 0.0;

        let (assign6560_e5032,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6560_e5028: f64 = (locals.var_axacl_i * locals.var_ile);
        let assign6560_e5029: f64 = (1.0 + assign6560_e5028);
        let assign6560_e5030: f64 = (locals.var_axaco_i / assign6560_e5029);
        (assign6560_e5030,)
    } else {
        (locals.var_axac_p,)
    }
};
        locals.var_axac_p = assign6560_e5032;
        locals.var_axac_p_rv = 0.0;

        let (assign6570_e5046,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6570_e5037: f64 = (locals.var_ile).powf(p.p369);
        let assign6570_e5038: f64 = (p.p368 * assign6570_e5037);
        let assign6570_e5042: f64 = (p.p370 * locals.var_iwe);
        let assign6570_e5043: f64 = (1.0 + assign6570_e5042);
        let assign6570_e5044: f64 = (assign6570_e5038 * assign6570_e5043);
        (assign6570_e5044,)
    } else {
        (locals.var_alpac_p,)
    }
};
        locals.var_alpac_p = assign6570_e5046;
        locals.var_alpac_p_rv = 0.0;

        let (assign6580_e5052,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6580_e5050: f64 = (locals.var_ile).powf(p.p372);
        (assign6580_e5050,)
    } else {
        (locals.var_tmpx,)
    }
};
        locals.var_tmpx = assign6580_e5052;
        locals.var_tmpx_rv = 0.0;

        let (assign6590_e5072,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6590_e5056: f64 = (p.p371 * locals.var_tmpx);
        let assign6590_e5060: f64 = (p.p374 * locals.var_iwe);
        let assign6590_e5061: f64 = (1.0 + assign6590_e5060);
        let assign6590_e5062: f64 = (assign6590_e5056 * assign6590_e5061);
        let assign6590_e5066: f64 = (p.p373 * locals.var_ile);
        let assign6590_e5068: f64 = (assign6590_e5066 * locals.var_tmpx);
        let assign6590_e5069: f64 = (1.0 + assign6590_e5068);
        let assign6590_e5070: f64 = (assign6590_e5062 / assign6590_e5069);
        (assign6590_e5070,)
    } else {
        (locals.var_alp1ac_p,)
    }
};
        locals.var_alp1ac_p = assign6590_e5072;
        locals.var_alp1ac_p_rv = 0.0;

        let (assign6600_e5076,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p375,)
    } else {
        (locals.var_fcgovacc_p,)
    }
};
        locals.var_fcgovacc_p = assign6600_e5076;
        locals.var_fcgovacc_p_rv = 0.0;

        let (assign6610_e5080,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p376,)
    } else {
        (locals.var_fcgovaccd_p,)
    }
};
        locals.var_fcgovaccd_p = assign6610_e5080;
        locals.var_fcgovaccd_p_rv = 0.0;

        let (assign6620_e5084,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p377,)
    } else {
        (locals.var_cgovaccg_p,)
    }
};
        locals.var_cgovaccg_p = assign6620_e5084;
        locals.var_cgovaccg_p_rv = 0.0;

        let (assign6630_e5090,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6630_e5088: f64 = (p.p378 * locals.var_iilcv);
        (assign6630_e5088,)
    } else {
        (locals.var_cgbov_p,)
    }
};
        locals.var_cgbov_p = assign6630_e5090;
        locals.var_cgbov_p_rv = 0.0;

        let (assign6640_e5096,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6640_e5094: f64 = (p.p379 * locals.var_iiwecv);
        (assign6640_e5094,)
    } else {
        (locals.var_cinr_p,)
    }
};
        locals.var_cinr_p = assign6640_e5096;
        locals.var_cinr_p_rv = 0.0;

        let (assign6650_e5102,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6650_e5100: f64 = (p.p380 * locals.var_iiwecv);
        (assign6650_e5100,)
    } else {
        (locals.var_cinrd_p,)
    }
};
        locals.var_cinrd_p = assign6650_e5102;
        locals.var_cinrd_p_rv = 0.0;

        let (assign6660_e5106,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p381,)
    } else {
        (locals.var_dvfbinr_p,)
    }
};
        locals.var_dvfbinr_p = assign6660_e5106;
        locals.var_dvfbinr_p_rv = 0.0;

        let (assign6670_e5110,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p382,)
    } else {
        (locals.var_fcinrdep_p,)
    }
};
        locals.var_fcinrdep_p = assign6670_e5110;
        locals.var_fcinrdep_p_rv = 0.0;

        let (assign6680_e5114,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p383,)
    } else {
        (locals.var_fcinracc_p,)
    }
};
        locals.var_fcinracc_p = assign6680_e5114;
        locals.var_fcinracc_p_rv = 0.0;

        let (assign6690_e5118,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p384,)
    } else {
        (locals.var_axinr_p,)
    }
};
        locals.var_axinr_p = assign6690_e5118;
        locals.var_axinr_p_rv = 0.0;

        let (assign6720_e5140,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6720_e5135: f64 = (2.0 * p.p393);
        let assign6720_e5137: f64 = (assign6720_e5135 / locals.var_le);
        let assign6720_e5138: f64 = (1.0 - assign6720_e5137);
        (assign6720_e5138,)
    } else {
        (locals.var_temp0,)
    }
};
        locals.var_temp0 = assign6720_e5140;
        locals.var_temp0_rv = 0.0;

        let (assign6750_e5161,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p387,)
    } else {
        (locals.var_fnt_p,)
    }
};
        locals.var_fnt_p = assign6750_e5161;
        locals.var_fnt_p_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_5(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign6810_e5211,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6810_e5205: f64 = (2.0 * p.p395);
        let assign6810_e5208: f64 = (p.p396 * locals.var_we);
        let assign6810_e5209: f64 = (assign6810_e5205 + assign6810_e5208);
        (assign6810_e5209,)
    } else {
        (locals.var_we_edge,)
    }
};
        locals.var_we_edge = assign6810_e5211;
        locals.var_we_edge_rv = 0.0;

        let (assign6840_e5227,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p397,)
    } else {
        (locals.var_vfbedge_p,)
    }
};
        locals.var_vfbedge_p = assign6840_e5227;
        locals.var_vfbedge_p_rv = 0.0;

        let (assign6850_e5243,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6850_e5232: f64 = (p.p399 * locals.var_ile);
        let assign6850_e5233: f64 = (p.p398 + assign6850_e5232);
        let assign6850_e5236: f64 = (p.p400 * locals.var_iwe);
        let assign6850_e5237: f64 = (assign6850_e5233 + assign6850_e5236);
        let assign6850_e5240: f64 = (p.p401 * locals.var_iae);
        let assign6850_e5241: f64 = (assign6850_e5237 + assign6850_e5240);
        (assign6850_e5241,)
    } else {
        (locals.var_stvfbedge_p,)
    }
};
        locals.var_stvfbedge_p = assign6850_e5243;
        locals.var_stvfbedge_p_rv = 0.0;

        let (assign6860_e5261,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6860_e5249: f64 = (locals.var_ile).powf(p.p404);
        let assign6860_e5250: f64 = (p.p403 * assign6860_e5249);
        let assign6860_e5251: f64 = (p.p402 + assign6860_e5250);
        let assign6860_e5254: f64 = (p.p405 * locals.var_iwe);
        let assign6860_e5255: f64 = (assign6860_e5251 + assign6860_e5254);
        let assign6860_e5258: f64 = (p.p406 * locals.var_iae);
        let assign6860_e5259: f64 = (assign6860_e5255 + assign6860_e5258);
        (assign6860_e5259,)
    } else {
        (locals.var_dphibedge_p,)
    }
};
        locals.var_dphibedge_p = assign6860_e5261;
        locals.var_dphibedge_p_rv = 0.0;

        let (assign6870_e5285,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6870_e5268: f64 = (locals.var_ile).powf(p.p409);
        let assign6870_e5269: f64 = (p.p408 * assign6870_e5268);
        let assign6870_e5270: f64 = (1.0 + assign6870_e5269);
        let assign6870_e5271: f64 = (p.p407 * assign6870_e5270);
        let assign6870_e5275: f64 = (p.p410 * locals.var_iwe);
        let assign6870_e5276: f64 = (1.0 + assign6870_e5275);
        let assign6870_e5277: f64 = (assign6870_e5271 * assign6870_e5276);
        let assign6870_e5281: f64 = (p.p411 * locals.var_iae);
        let assign6870_e5282: f64 = (1.0 + assign6870_e5281);
        let assign6870_e5283: f64 = (assign6870_e5277 * assign6870_e5282);
        (assign6870_e5283,)
    } else {
        (locals.var_neffedge_p,)
    }
};
        locals.var_neffedge_p = assign6870_e5285;
        locals.var_neffedge_p_rv = 0.0;

        let (assign6880_e5295,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6880_e5291: f64 = (locals.var_ile).powf(p.p414);
        let assign6880_e5292: f64 = (p.p413 * assign6880_e5291);
        let assign6880_e5293: f64 = (p.p412 + assign6880_e5292);
        (assign6880_e5293,)
    } else {
        (locals.var_ctedge_p,)
    }
};
        locals.var_ctedge_p = assign6880_e5295;
        locals.var_ctedge_p_rv = 0.0;

        let (assign6890_e5313,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6890_e5300: f64 = (p.p415 * p.p416);
        let assign6890_e5302: f64 = (assign6890_e5300 / locals.var_le);
        let assign6890_e5305: f64 = (-locals.var_le);
        let assign6890_e5307: f64 = (assign6890_e5305 / p.p416);
        let assign6890_e5308: f64 = (assign6890_e5307).exp();
        let assign6890_e5309: f64 = (1.0 - assign6890_e5308);
        let assign6890_e5310: f64 = (assign6890_e5302 * assign6890_e5309);
        let assign6890_e5311: f64 = (1.0 + assign6890_e5310);
        (assign6890_e5311,)
    } else {
        (locals.var_gpe_edge,)
    }
};
        locals.var_gpe_edge = assign6890_e5313;
        locals.var_gpe_edge_rv = 0.0;

        let (assign6900_e5322,) = {
    if (locals.var_guard36 != 0.0) {
        let (assign6900_e5320,) = {
            if (locals.var_gpe_edge > 1e-15) {
                (locals.var_gpe_edge,)
            } else {
                (1e-15,)
            }
        };
        (assign6900_e5320,)
    } else {
        (locals.var_gpe_edge,)
    }
};
        locals.var_gpe_edge = assign6900_e5322;
        locals.var_gpe_edge_rv = 0.0;

        let (assign6910_e5338,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6910_e5326: f64 = (p.p256 * locals.var_we_edge);
        let assign6910_e5329: f64 = (locals.var_gpe_edge * locals.var_le);
        let assign6910_e5330: f64 = (assign6910_e5326 / assign6910_e5329);
        let assign6910_e5334: f64 = (p.p417 * locals.var_iwe);
        let assign6910_e5335: f64 = (1.0 + assign6910_e5334);
        let assign6910_e5336: f64 = (assign6910_e5330 * assign6910_e5335);
        (assign6910_e5336,)
    } else {
        (locals.var_betnedge_p,)
    }
};
        locals.var_betnedge_p = assign6910_e5338;
        locals.var_betnedge_p_rv = 0.0;

        let (assign6920_e5354,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6920_e5343: f64 = (p.p419 * locals.var_ile);
        let assign6920_e5344: f64 = (p.p418 + assign6920_e5343);
        let assign6920_e5347: f64 = (p.p420 * locals.var_iwe);
        let assign6920_e5348: f64 = (assign6920_e5344 + assign6920_e5347);
        let assign6920_e5351: f64 = (p.p421 * locals.var_iae);
        let assign6920_e5352: f64 = (assign6920_e5348 + assign6920_e5351);
        (assign6920_e5352,)
    } else {
        (locals.var_stbetedge_p,)
    }
};
        locals.var_stbetedge_p = assign6920_e5354;
        locals.var_stbetedge_p_rv = 0.0;

        let (assign6930_e5368,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6930_e5359: f64 = (locals.var_ile).powf(p.p423);
        let assign6930_e5360: f64 = (p.p422 * assign6930_e5359);
        let assign6930_e5364: f64 = (p.p424 * locals.var_iwe);
        let assign6930_e5365: f64 = (1.0 + assign6930_e5364);
        let assign6930_e5366: f64 = (assign6930_e5360 * assign6930_e5365);
        (assign6930_e5366,)
    } else {
        (locals.var_psceedge_p,)
    }
};
        locals.var_psceedge_p = assign6930_e5368;
        locals.var_psceedge_p_rv = 0.0;

        let (assign6940_e5372,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p425,)
    } else {
        (locals.var_pscebedge_p,)
    }
};
        locals.var_pscebedge_p = assign6940_e5372;
        locals.var_pscebedge_p_rv = 0.0;

        let (assign6950_e5376,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p426,)
    } else {
        (locals.var_pscededge_p,)
    }
};
        locals.var_pscededge_p = assign6950_e5376;
        locals.var_pscededge_p_rv = 0.0;

        let (assign6960_e5390,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6960_e5381: f64 = (locals.var_ile).powf(p.p428);
        let assign6960_e5382: f64 = (p.p427 * assign6960_e5381);
        let assign6960_e5386: f64 = (p.p429 * locals.var_iwe);
        let assign6960_e5387: f64 = (1.0 + assign6960_e5386);
        let assign6960_e5388: f64 = (assign6960_e5382 * assign6960_e5387);
        (assign6960_e5388,)
    } else {
        (locals.var_cfedge_p,)
    }
};
        locals.var_cfedge_p = assign6960_e5390;
        locals.var_cfedge_p_rv = 0.0;

        let (assign6970_e5394,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p431,)
    } else {
        (locals.var_cfdedge_p,)
    }
};
        locals.var_cfdedge_p = assign6970_e5394;
        locals.var_cfdedge_p_rv = 0.0;

        let (assign6980_e5398,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p430,)
    } else {
        (locals.var_cfbedge_p,)
    }
};
        locals.var_cfbedge_p = assign6980_e5398;
        locals.var_cfbedge_p_rv = 0.0;

        let (assign7040_e5440,) = {
    if (locals.var_guard36 != 0.0) {
        let assign7040_e5429: f64 = (p.p808 * locals.var_ile);
        let assign7040_e5430: f64 = (p.p807 + assign7040_e5429);
        let assign7040_e5433: f64 = (p.p809 * locals.var_iwe);
        let assign7040_e5434: f64 = (assign7040_e5430 + assign7040_e5433);
        let assign7040_e5437: f64 = (p.p810 * locals.var_iae);
        let assign7040_e5438: f64 = (assign7040_e5434 + assign7040_e5437);
        (assign7040_e5438,)
    } else {
        (locals.var_kvthowe,)
    }
};
        locals.var_kvthowe = assign7040_e5440;
        locals.var_kvthowe_rv = 0.0;

        let (assign7050_e5456,) = {
    if (locals.var_guard36 != 0.0) {
        let assign7050_e5445: f64 = (p.p812 * locals.var_ile);
        let assign7050_e5446: f64 = (p.p811 + assign7050_e5445);
        let assign7050_e5449: f64 = (p.p813 * locals.var_iwe);
        let assign7050_e5450: f64 = (assign7050_e5446 + assign7050_e5449);
        let assign7050_e5453: f64 = (p.p814 * locals.var_iae);
        let assign7050_e5454: f64 = (assign7050_e5450 + assign7050_e5453);
        (assign7050_e5454,)
    } else {
        (locals.var_kuowe,)
    }
};
        locals.var_kuowe = assign7050_e5456;
        locals.var_kuowe_rv = 0.0;

        let assign7170_e5570: f64 = if (((param_given[448] || param_given[449]) || param_given[450]) || param_given[451]) { 1.0 } else { 0.0 };
        locals.var_guard51 = assign7170_e5570;
        locals.var_guard51_rv = 0.0;

        let (assign7180_e5588,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard51 != 0.0)) {
        let assign7180_e5577: f64 = (p.p449 * locals.var_ile);
        let assign7180_e5578: f64 = (p.p448 + assign7180_e5577);
        let assign7180_e5581: f64 = (p.p450 * locals.var_iwe);
        let assign7180_e5582: f64 = (assign7180_e5578 + assign7180_e5581);
        let assign7180_e5585: f64 = (p.p451 * locals.var_iae);
        let assign7180_e5586: f64 = (assign7180_e5582 + assign7180_e5585);
        (assign7180_e5586,)
    } else {
        (locals.var_vfb_p,)
    }
};
        locals.var_vfb_p = assign7180_e5588;
        locals.var_vfb_p_rv = 0.0;

        let assign7190_e5607: f64 = if (((param_given[452] || param_given[453]) || param_given[454]) || param_given[455]) { 1.0 } else { 0.0 };
        locals.var_guard52 = assign7190_e5607;
        locals.var_guard52_rv = 0.0;

        let (assign7200_e5625,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard52 != 0.0)) {
        let assign7200_e5614: f64 = (p.p453 * locals.var_ile);
        let assign7200_e5615: f64 = (p.p452 + assign7200_e5614);
        let assign7200_e5618: f64 = (p.p454 * locals.var_iwe);
        let assign7200_e5619: f64 = (assign7200_e5615 + assign7200_e5618);
        let assign7200_e5622: f64 = (p.p455 * locals.var_iae);
        let assign7200_e5623: f64 = (assign7200_e5619 + assign7200_e5622);
        (assign7200_e5623,)
    } else {
        (locals.var_stvfb_p,)
    }
};
        locals.var_stvfb_p = assign7200_e5625;
        locals.var_stvfb_p_rv = 0.0;

        let assign7210_e5644: f64 = if (((param_given[456] || param_given[457]) || param_given[458]) || param_given[459]) { 1.0 } else { 0.0 };
        locals.var_guard53 = assign7210_e5644;
        locals.var_guard53_rv = 0.0;

        let (assign7220_e5662,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard53 != 0.0)) {
        let assign7220_e5651: f64 = (p.p457 * locals.var_ile);
        let assign7220_e5652: f64 = (p.p456 + assign7220_e5651);
        let assign7220_e5655: f64 = (p.p458 * locals.var_iwe);
        let assign7220_e5656: f64 = (assign7220_e5652 + assign7220_e5655);
        let assign7220_e5659: f64 = (p.p459 * locals.var_iae);
        let assign7220_e5660: f64 = (assign7220_e5656 + assign7220_e5659);
        (assign7220_e5660,)
    } else {
        (locals.var_neff_p,)
    }
};
        locals.var_neff_p = assign7220_e5662;
        locals.var_neff_p_rv = 0.0;

        let assign7230_e5681: f64 = if (((param_given[460] || param_given[461]) || param_given[462]) || param_given[463]) { 1.0 } else { 0.0 };
        locals.var_guard54 = assign7230_e5681;
        locals.var_guard54_rv = 0.0;

        let (assign7240_e5699,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard54 != 0.0)) {
        let assign7240_e5688: f64 = (p.p461 * locals.var_ile);
        let assign7240_e5689: f64 = (p.p460 + assign7240_e5688);
        let assign7240_e5692: f64 = (p.p462 * locals.var_iwe);
        let assign7240_e5693: f64 = (assign7240_e5689 + assign7240_e5692);
        let assign7240_e5696: f64 = (p.p463 * locals.var_iae);
        let assign7240_e5697: f64 = (assign7240_e5693 + assign7240_e5696);
        (assign7240_e5697,)
    } else {
        (locals.var_gfacnud_p,)
    }
};
        locals.var_gfacnud_p = assign7240_e5699;
        locals.var_gfacnud_p_rv = 0.0;

        let assign7250_e5718: f64 = if (((param_given[464] || param_given[465]) || param_given[466]) || param_given[467]) { 1.0 } else { 0.0 };
        locals.var_guard55 = assign7250_e5718;
        locals.var_guard55_rv = 0.0;

        let (assign7260_e5736,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard55 != 0.0)) {
        let assign7260_e5725: f64 = (p.p465 * locals.var_ile);
        let assign7260_e5726: f64 = (p.p464 + assign7260_e5725);
        let assign7260_e5729: f64 = (p.p466 * locals.var_iwe);
        let assign7260_e5730: f64 = (assign7260_e5726 + assign7260_e5729);
        let assign7260_e5733: f64 = (p.p467 * locals.var_iae);
        let assign7260_e5734: f64 = (assign7260_e5730 + assign7260_e5733);
        (assign7260_e5734,)
    } else {
        (locals.var_vsbnud_p,)
    }
};
        locals.var_vsbnud_p = assign7260_e5736;
        locals.var_vsbnud_p_rv = 0.0;

        let assign7270_e5755: f64 = if (((param_given[468] || param_given[469]) || param_given[470]) || param_given[471]) { 1.0 } else { 0.0 };
        locals.var_guard56 = assign7270_e5755;
        locals.var_guard56_rv = 0.0;

        let (assign7280_e5773,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard56 != 0.0)) {
        let assign7280_e5762: f64 = (p.p469 * locals.var_ile);
        let assign7280_e5763: f64 = (p.p468 + assign7280_e5762);
        let assign7280_e5766: f64 = (p.p470 * locals.var_iwe);
        let assign7280_e5767: f64 = (assign7280_e5763 + assign7280_e5766);
        let assign7280_e5770: f64 = (p.p471 * locals.var_iae);
        let assign7280_e5771: f64 = (assign7280_e5767 + assign7280_e5770);
        (assign7280_e5771,)
    } else {
        (locals.var_dphib_p,)
    }
};
        locals.var_dphib_p = assign7280_e5773;
        locals.var_dphib_p_rv = 0.0;

        let assign7290_e5792: f64 = if (((param_given[472] || param_given[473]) || param_given[474]) || param_given[475]) { 1.0 } else { 0.0 };
        locals.var_guard57 = assign7290_e5792;
        locals.var_guard57_rv = 0.0;

        let (assign7300_e5810,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard57 != 0.0)) {
        let assign7300_e5799: f64 = (p.p473 * locals.var_ile);
        let assign7300_e5800: f64 = (p.p472 + assign7300_e5799);
        let assign7300_e5803: f64 = (p.p474 * locals.var_iwe);
        let assign7300_e5804: f64 = (assign7300_e5800 + assign7300_e5803);
        let assign7300_e5807: f64 = (p.p475 * locals.var_iae);
        let assign7300_e5808: f64 = (assign7300_e5804 + assign7300_e5807);
        (assign7300_e5808,)
    } else {
        (locals.var_np_p,)
    }
};
        locals.var_np_p = assign7300_e5810;
        locals.var_np_p_rv = 0.0;

        let assign7310_e5829: f64 = if (((param_given[476] || param_given[477]) || param_given[478]) || param_given[479]) { 1.0 } else { 0.0 };
        locals.var_guard58 = assign7310_e5829;
        locals.var_guard58_rv = 0.0;

        let (assign7320_e5847,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard58 != 0.0)) {
        let assign7320_e5836: f64 = (p.p477 * locals.var_ile);
        let assign7320_e5837: f64 = (p.p476 + assign7320_e5836);
        let assign7320_e5840: f64 = (p.p478 * locals.var_iwe);
        let assign7320_e5841: f64 = (assign7320_e5837 + assign7320_e5840);
        let assign7320_e5844: f64 = (p.p479 * locals.var_iae);
        let assign7320_e5845: f64 = (assign7320_e5841 + assign7320_e5844);
        (assign7320_e5845,)
    } else {
        (locals.var_nov_p,)
    }
};
        locals.var_nov_p = assign7320_e5847;
        locals.var_nov_p_rv = 0.0;

        let assign7330_e5866: f64 = if (((param_given[480] || param_given[481]) || param_given[482]) || param_given[483]) { 1.0 } else { 0.0 };
        locals.var_guard59 = assign7330_e5866;
        locals.var_guard59_rv = 0.0;

        let (assign7340_e5884,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard59 != 0.0)) {
        let assign7340_e5873: f64 = (p.p481 * locals.var_ile);
        let assign7340_e5874: f64 = (p.p480 + assign7340_e5873);
        let assign7340_e5877: f64 = (p.p482 * locals.var_iwe);
        let assign7340_e5878: f64 = (assign7340_e5874 + assign7340_e5877);
        let assign7340_e5881: f64 = (p.p483 * locals.var_iae);
        let assign7340_e5882: f64 = (assign7340_e5878 + assign7340_e5881);
        (assign7340_e5882,)
    } else {
        (locals.var_novd_p,)
    }
};
        locals.var_novd_p = assign7340_e5884;
        locals.var_novd_p_rv = 0.0;

        let assign7350_e5903: f64 = if (((param_given[484] || param_given[485]) || param_given[486]) || param_given[487]) { 1.0 } else { 0.0 };
        locals.var_guard60 = assign7350_e5903;
        locals.var_guard60_rv = 0.0;

        let (assign7360_e5921,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard60 != 0.0)) {
        let assign7360_e5910: f64 = (p.p485 * locals.var_ile);
        let assign7360_e5911: f64 = (p.p484 + assign7360_e5910);
        let assign7360_e5914: f64 = (p.p486 * locals.var_iwe);
        let assign7360_e5915: f64 = (assign7360_e5911 + assign7360_e5914);
        let assign7360_e5918: f64 = (p.p487 * locals.var_iae);
        let assign7360_e5919: f64 = (assign7360_e5915 + assign7360_e5918);
        (assign7360_e5919,)
    } else {
        (locals.var_ct_p,)
    }
};
        locals.var_ct_p = assign7360_e5921;
        locals.var_ct_p_rv = 0.0;

        let assign7370_e5940: f64 = if (((param_given[492] || param_given[493]) || param_given[494]) || param_given[495]) { 1.0 } else { 0.0 };
        locals.var_guard61 = assign7370_e5940;
        locals.var_guard61_rv = 0.0;

        let (assign7380_e5958,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard61 != 0.0)) {
        let assign7380_e5947: f64 = (p.p493 * locals.var_ile);
        let assign7380_e5948: f64 = (p.p492 + assign7380_e5947);
        let assign7380_e5951: f64 = (p.p494 * locals.var_iwe);
        let assign7380_e5952: f64 = (assign7380_e5948 + assign7380_e5951);
        let assign7380_e5955: f64 = (p.p495 * locals.var_iae);
        let assign7380_e5956: f64 = (assign7380_e5952 + assign7380_e5955);
        (assign7380_e5956,)
    } else {
        (locals.var_ctg_p,)
    }
};
        locals.var_ctg_p = assign7380_e5958;
        locals.var_ctg_p_rv = 0.0;

        let assign7390_e5977: f64 = if (((param_given[488] || param_given[489]) || param_given[490]) || param_given[491]) { 1.0 } else { 0.0 };
        locals.var_guard62 = assign7390_e5977;
        locals.var_guard62_rv = 0.0;

        let (assign7400_e5995,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard62 != 0.0)) {
        let assign7400_e5984: f64 = (p.p489 * locals.var_ile);
        let assign7400_e5985: f64 = (p.p488 + assign7400_e5984);
        let assign7400_e5988: f64 = (p.p490 * locals.var_iwe);
        let assign7400_e5989: f64 = (assign7400_e5985 + assign7400_e5988);
        let assign7400_e5992: f64 = (p.p491 * locals.var_iae);
        let assign7400_e5993: f64 = (assign7400_e5989 + assign7400_e5992);
        (assign7400_e5993,)
    } else {
        (locals.var_ctb_p,)
    }
};
        locals.var_ctb_p = assign7400_e5995;
        locals.var_ctb_p_rv = 0.0;

        let assign7410_e6014: f64 = if (((param_given[496] || param_given[497]) || param_given[498]) || param_given[499]) { 1.0 } else { 0.0 };
        locals.var_guard63 = assign7410_e6014;
        locals.var_guard63_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_6(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign7420_e6032,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard63 != 0.0)) {
        let assign7420_e6021: f64 = (p.p497 * locals.var_ile);
        let assign7420_e6022: f64 = (p.p496 + assign7420_e6021);
        let assign7420_e6025: f64 = (p.p498 * locals.var_iwe);
        let assign7420_e6026: f64 = (assign7420_e6022 + assign7420_e6025);
        let assign7420_e6029: f64 = (p.p499 * locals.var_iae);
        let assign7420_e6030: f64 = (assign7420_e6026 + assign7420_e6029);
        (assign7420_e6030,)
    } else {
        (locals.var_stct_p,)
    }
};
        locals.var_stct_p = assign7420_e6032;
        locals.var_stct_p_rv = 0.0;

        let assign7430_e6051: f64 = if (((param_given[500] || param_given[501]) || param_given[502]) || param_given[503]) { 1.0 } else { 0.0 };
        locals.var_guard64 = assign7430_e6051;
        locals.var_guard64_rv = 0.0;

        let (assign7440_e6071,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard64 != 0.0)) {
        let assign7440_e6059: f64 = (p.p501 * locals.var_ile);
        let assign7440_e6060: f64 = (p.p500 + assign7440_e6059);
        let assign7440_e6063: f64 = (p.p502 * locals.var_iwe);
        let assign7440_e6064: f64 = (assign7440_e6060 + assign7440_e6063);
        let assign7440_e6067: f64 = (p.p503 * locals.var_iae);
        let assign7440_e6068: f64 = (assign7440_e6064 + assign7440_e6067);
        let assign7440_e6069: f64 = (locals.var_ile2 * assign7440_e6068);
        (assign7440_e6069,)
    } else {
        (locals.var_cf_p,)
    }
};
        locals.var_cf_p = assign7440_e6071;
        locals.var_cf_p_rv = 0.0;

        let assign7450_e6090: f64 = if (((param_given[508] || param_given[509]) || param_given[510]) || param_given[511]) { 1.0 } else { 0.0 };
        locals.var_guard65 = assign7450_e6090;
        locals.var_guard65_rv = 0.0;

        let (assign7460_e6108,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard65 != 0.0)) {
        let assign7460_e6097: f64 = (p.p509 * locals.var_ile);
        let assign7460_e6098: f64 = (p.p508 + assign7460_e6097);
        let assign7460_e6101: f64 = (p.p510 * locals.var_iwe);
        let assign7460_e6102: f64 = (assign7460_e6098 + assign7460_e6101);
        let assign7460_e6105: f64 = (p.p511 * locals.var_iae);
        let assign7460_e6106: f64 = (assign7460_e6102 + assign7460_e6105);
        (assign7460_e6106,)
    } else {
        (locals.var_cfd_p,)
    }
};
        locals.var_cfd_p = assign7460_e6108;
        locals.var_cfd_p_rv = 0.0;

        let assign7470_e6127: f64 = if (((param_given[504] || param_given[505]) || param_given[506]) || param_given[507]) { 1.0 } else { 0.0 };
        locals.var_guard66 = assign7470_e6127;
        locals.var_guard66_rv = 0.0;

        let (assign7480_e6145,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard66 != 0.0)) {
        let assign7480_e6134: f64 = (p.p505 * locals.var_ile);
        let assign7480_e6135: f64 = (p.p504 + assign7480_e6134);
        let assign7480_e6138: f64 = (p.p506 * locals.var_iwe);
        let assign7480_e6139: f64 = (assign7480_e6135 + assign7480_e6138);
        let assign7480_e6142: f64 = (p.p507 * locals.var_iae);
        let assign7480_e6143: f64 = (assign7480_e6139 + assign7480_e6142);
        (assign7480_e6143,)
    } else {
        (locals.var_cfb_p,)
    }
};
        locals.var_cfb_p = assign7480_e6145;
        locals.var_cfb_p_rv = 0.0;

        let assign7490_e6164: f64 = if (((param_given[512] || param_given[513]) || param_given[514]) || param_given[515]) { 1.0 } else { 0.0 };
        locals.var_guard67 = assign7490_e6164;
        locals.var_guard67_rv = 0.0;

        let (assign7500_e6184,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard67 != 0.0)) {
        let assign7500_e6172: f64 = (p.p513 * locals.var_ile);
        let assign7500_e6173: f64 = (p.p512 + assign7500_e6172);
        let assign7500_e6176: f64 = (p.p514 * locals.var_iwe);
        let assign7500_e6177: f64 = (assign7500_e6173 + assign7500_e6176);
        let assign7500_e6180: f64 = (p.p515 * locals.var_iae);
        let assign7500_e6181: f64 = (assign7500_e6177 + assign7500_e6180);
        let assign7500_e6182: f64 = (locals.var_ile2 * assign7500_e6181);
        (assign7500_e6182,)
    } else {
        (locals.var_psce_p,)
    }
};
        locals.var_psce_p = assign7500_e6184;
        locals.var_psce_p_rv = 0.0;

        let assign7510_e6203: f64 = if (((param_given[520] || param_given[521]) || param_given[522]) || param_given[523]) { 1.0 } else { 0.0 };
        locals.var_guard68 = assign7510_e6203;
        locals.var_guard68_rv = 0.0;

        let (assign7520_e6221,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard68 != 0.0)) {
        let assign7520_e6210: f64 = (p.p521 * locals.var_ile);
        let assign7520_e6211: f64 = (p.p520 + assign7520_e6210);
        let assign7520_e6214: f64 = (p.p522 * locals.var_iwe);
        let assign7520_e6215: f64 = (assign7520_e6211 + assign7520_e6214);
        let assign7520_e6218: f64 = (p.p523 * locals.var_iae);
        let assign7520_e6219: f64 = (assign7520_e6215 + assign7520_e6218);
        (assign7520_e6219,)
    } else {
        (locals.var_psced_p,)
    }
};
        locals.var_psced_p = assign7520_e6221;
        locals.var_psced_p_rv = 0.0;

        let assign7530_e6240: f64 = if (((param_given[516] || param_given[517]) || param_given[518]) || param_given[519]) { 1.0 } else { 0.0 };
        locals.var_guard69 = assign7530_e6240;
        locals.var_guard69_rv = 0.0;

        let (assign7540_e6258,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard69 != 0.0)) {
        let assign7540_e6247: f64 = (p.p517 * locals.var_ile);
        let assign7540_e6248: f64 = (p.p516 + assign7540_e6247);
        let assign7540_e6251: f64 = (p.p518 * locals.var_iwe);
        let assign7540_e6252: f64 = (assign7540_e6248 + assign7540_e6251);
        let assign7540_e6255: f64 = (p.p519 * locals.var_iae);
        let assign7540_e6256: f64 = (assign7540_e6252 + assign7540_e6255);
        (assign7540_e6256,)
    } else {
        (locals.var_psceb_p,)
    }
};
        locals.var_psceb_p = assign7540_e6258;
        locals.var_psceb_p_rv = 0.0;

        let assign7550_e6277: f64 = if (((param_given[524] || param_given[525]) || param_given[526]) || param_given[527]) { 1.0 } else { 0.0 };
        locals.var_guard70 = assign7550_e6277;
        locals.var_guard70_rv = 0.0;

        let (assign7560_e6299,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard70 != 0.0)) {
        let assign7560_e6283: f64 = (locals.var_we / locals.var_le);
        let assign7560_e6287: f64 = (p.p525 * locals.var_ile);
        let assign7560_e6288: f64 = (p.p524 + assign7560_e6287);
        let assign7560_e6291: f64 = (p.p526 * locals.var_iwe);
        let assign7560_e6292: f64 = (assign7560_e6288 + assign7560_e6291);
        let assign7560_e6295: f64 = (p.p527 * locals.var_iae);
        let assign7560_e6296: f64 = (assign7560_e6292 + assign7560_e6295);
        let assign7560_e6297: f64 = (assign7560_e6283 * assign7560_e6296);
        (assign7560_e6297,)
    } else {
        (locals.var_betn_p,)
    }
};
        locals.var_betn_p = assign7560_e6299;
        locals.var_betn_p_rv = 0.0;

        let assign7570_e6318: f64 = if (((param_given[528] || param_given[529]) || param_given[530]) || param_given[531]) { 1.0 } else { 0.0 };
        locals.var_guard71 = assign7570_e6318;
        locals.var_guard71_rv = 0.0;

        let (assign7580_e6336,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard71 != 0.0)) {
        let assign7580_e6325: f64 = (p.p529 * locals.var_ile);
        let assign7580_e6326: f64 = (p.p528 + assign7580_e6325);
        let assign7580_e6329: f64 = (p.p530 * locals.var_iwe);
        let assign7580_e6330: f64 = (assign7580_e6326 + assign7580_e6329);
        let assign7580_e6333: f64 = (p.p531 * locals.var_iae);
        let assign7580_e6334: f64 = (assign7580_e6330 + assign7580_e6333);
        (assign7580_e6334,)
    } else {
        (locals.var_stbet_p,)
    }
};
        locals.var_stbet_p = assign7580_e6336;
        locals.var_stbet_p_rv = 0.0;

        let assign7590_e6355: f64 = if (((param_given[532] || param_given[533]) || param_given[534]) || param_given[535]) { 1.0 } else { 0.0 };
        locals.var_guard72 = assign7590_e6355;
        locals.var_guard72_rv = 0.0;

        let (assign7600_e6373,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard72 != 0.0)) {
        let assign7600_e6362: f64 = (p.p533 * locals.var_ile);
        let assign7600_e6363: f64 = (p.p532 + assign7600_e6362);
        let assign7600_e6366: f64 = (p.p534 * locals.var_iwe);
        let assign7600_e6367: f64 = (assign7600_e6363 + assign7600_e6366);
        let assign7600_e6370: f64 = (p.p535 * locals.var_iae);
        let assign7600_e6371: f64 = (assign7600_e6367 + assign7600_e6370);
        (assign7600_e6371,)
    } else {
        (locals.var_mue_p,)
    }
};
        locals.var_mue_p = assign7600_e6373;
        locals.var_mue_p_rv = 0.0;

        let assign7610_e6392: f64 = if (((param_given[536] || param_given[537]) || param_given[538]) || param_given[539]) { 1.0 } else { 0.0 };
        locals.var_guard73 = assign7610_e6392;
        locals.var_guard73_rv = 0.0;

        let (assign7620_e6410,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard73 != 0.0)) {
        let assign7620_e6399: f64 = (p.p537 * locals.var_ile);
        let assign7620_e6400: f64 = (p.p536 + assign7620_e6399);
        let assign7620_e6403: f64 = (p.p538 * locals.var_iwe);
        let assign7620_e6404: f64 = (assign7620_e6400 + assign7620_e6403);
        let assign7620_e6407: f64 = (p.p539 * locals.var_iae);
        let assign7620_e6408: f64 = (assign7620_e6404 + assign7620_e6407);
        (assign7620_e6408,)
    } else {
        (locals.var_themu_p,)
    }
};
        locals.var_themu_p = assign7620_e6410;
        locals.var_themu_p_rv = 0.0;

        let assign7630_e6429: f64 = if (((param_given[540] || param_given[541]) || param_given[542]) || param_given[543]) { 1.0 } else { 0.0 };
        locals.var_guard74 = assign7630_e6429;
        locals.var_guard74_rv = 0.0;

        let (assign7640_e6447,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard74 != 0.0)) {
        let assign7640_e6436: f64 = (p.p541 * locals.var_ile);
        let assign7640_e6437: f64 = (p.p540 + assign7640_e6436);
        let assign7640_e6440: f64 = (p.p542 * locals.var_iwe);
        let assign7640_e6441: f64 = (assign7640_e6437 + assign7640_e6440);
        let assign7640_e6444: f64 = (p.p543 * locals.var_iae);
        let assign7640_e6445: f64 = (assign7640_e6441 + assign7640_e6444);
        (assign7640_e6445,)
    } else {
        (locals.var_cs_p,)
    }
};
        locals.var_cs_p = assign7640_e6447;
        locals.var_cs_p_rv = 0.0;

        let assign7650_e6466: f64 = if (((param_given[544] || param_given[545]) || param_given[546]) || param_given[547]) { 1.0 } else { 0.0 };
        locals.var_guard75 = assign7650_e6466;
        locals.var_guard75_rv = 0.0;

        let (assign7660_e6484,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard75 != 0.0)) {
        let assign7660_e6473: f64 = (p.p545 * locals.var_ile);
        let assign7660_e6474: f64 = (p.p544 + assign7660_e6473);
        let assign7660_e6477: f64 = (p.p546 * locals.var_iwe);
        let assign7660_e6478: f64 = (assign7660_e6474 + assign7660_e6477);
        let assign7660_e6481: f64 = (p.p547 * locals.var_iae);
        let assign7660_e6482: f64 = (assign7660_e6478 + assign7660_e6481);
        (assign7660_e6482,)
    } else {
        (locals.var_thecs_p,)
    }
};
        locals.var_thecs_p = assign7660_e6484;
        locals.var_thecs_p_rv = 0.0;

        let assign7670_e6503: f64 = if (((param_given[548] || param_given[549]) || param_given[550]) || param_given[551]) { 1.0 } else { 0.0 };
        locals.var_guard76 = assign7670_e6503;
        locals.var_guard76_rv = 0.0;

        let (assign7680_e6521,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard76 != 0.0)) {
        let assign7680_e6510: f64 = (p.p549 * locals.var_ile);
        let assign7680_e6511: f64 = (p.p548 + assign7680_e6510);
        let assign7680_e6514: f64 = (p.p550 * locals.var_iwe);
        let assign7680_e6515: f64 = (assign7680_e6511 + assign7680_e6514);
        let assign7680_e6518: f64 = (p.p551 * locals.var_iae);
        let assign7680_e6519: f64 = (assign7680_e6515 + assign7680_e6518);
        (assign7680_e6519,)
    } else {
        (locals.var_xcor_p,)
    }
};
        locals.var_xcor_p = assign7680_e6521;
        locals.var_xcor_p_rv = 0.0;

        let assign7690_e6540: f64 = if (((param_given[552] || param_given[553]) || param_given[554]) || param_given[555]) { 1.0 } else { 0.0 };
        locals.var_guard77 = assign7690_e6540;
        locals.var_guard77_rv = 0.0;

        let (assign7700_e6560,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard77 != 0.0)) {
        let assign7700_e6548: f64 = (p.p553 * locals.var_ile);
        let assign7700_e6549: f64 = (p.p552 + assign7700_e6548);
        let assign7700_e6552: f64 = (p.p554 * locals.var_iwe);
        let assign7700_e6553: f64 = (assign7700_e6549 + assign7700_e6552);
        let assign7700_e6556: f64 = (p.p555 * locals.var_iae);
        let assign7700_e6557: f64 = (assign7700_e6553 + assign7700_e6556);
        let assign7700_e6558: f64 = (locals.var_iwe * assign7700_e6557);
        (assign7700_e6558,)
    } else {
        (locals.var_rs_p,)
    }
};
        locals.var_rs_p = assign7700_e6560;
        locals.var_rs_p_rv = 0.0;

        let assign7710_e6579: f64 = if (((param_given[556] || param_given[557]) || param_given[558]) || param_given[559]) { 1.0 } else { 0.0 };
        locals.var_guard78 = assign7710_e6579;
        locals.var_guard78_rv = 0.0;

        let (assign7720_e6597,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard78 != 0.0)) {
        let assign7720_e6586: f64 = (p.p557 * locals.var_ile);
        let assign7720_e6587: f64 = (p.p556 + assign7720_e6586);
        let assign7720_e6590: f64 = (p.p558 * locals.var_iwe);
        let assign7720_e6591: f64 = (assign7720_e6587 + assign7720_e6590);
        let assign7720_e6594: f64 = (p.p559 * locals.var_iae);
        let assign7720_e6595: f64 = (assign7720_e6591 + assign7720_e6594);
        (assign7720_e6595,)
    } else {
        (locals.var_strs_p,)
    }
};
        locals.var_strs_p = assign7720_e6597;
        locals.var_strs_p_rv = 0.0;

        let assign7730_e6616: f64 = if (((param_given[560] || param_given[561]) || param_given[562]) || param_given[563]) { 1.0 } else { 0.0 };
        locals.var_guard79 = assign7730_e6616;
        locals.var_guard79_rv = 0.0;

        let (assign7740_e6634,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard79 != 0.0)) {
        let assign7740_e6623: f64 = (p.p561 * locals.var_ile);
        let assign7740_e6624: f64 = (p.p560 + assign7740_e6623);
        let assign7740_e6627: f64 = (p.p562 * locals.var_iwe);
        let assign7740_e6628: f64 = (assign7740_e6624 + assign7740_e6627);
        let assign7740_e6631: f64 = (p.p563 * locals.var_iae);
        let assign7740_e6632: f64 = (assign7740_e6628 + assign7740_e6631);
        (assign7740_e6632,)
    } else {
        (locals.var_rsb_p,)
    }
};
        locals.var_rsb_p = assign7740_e6634;
        locals.var_rsb_p_rv = 0.0;

        let assign7750_e6653: f64 = if (((param_given[564] || param_given[565]) || param_given[566]) || param_given[567]) { 1.0 } else { 0.0 };
        locals.var_guard80 = assign7750_e6653;
        locals.var_guard80_rv = 0.0;

        let (assign7760_e6671,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard80 != 0.0)) {
        let assign7760_e6660: f64 = (p.p565 * locals.var_ile);
        let assign7760_e6661: f64 = (p.p564 + assign7760_e6660);
        let assign7760_e6664: f64 = (p.p566 * locals.var_iwe);
        let assign7760_e6665: f64 = (assign7760_e6661 + assign7760_e6664);
        let assign7760_e6668: f64 = (p.p567 * locals.var_iae);
        let assign7760_e6669: f64 = (assign7760_e6665 + assign7760_e6668);
        (assign7760_e6669,)
    } else {
        (locals.var_rsg_p,)
    }
};
        locals.var_rsg_p = assign7760_e6671;
        locals.var_rsg_p_rv = 0.0;

        let assign7770_e6690: f64 = if (((param_given[568] || param_given[569]) || param_given[570]) || param_given[571]) { 1.0 } else { 0.0 };
        locals.var_guard81 = assign7770_e6690;
        locals.var_guard81_rv = 0.0;

        let (assign7780_e6710,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard81 != 0.0)) {
        let assign7780_e6698: f64 = (p.p569 * locals.var_ile);
        let assign7780_e6699: f64 = (p.p568 + assign7780_e6698);
        let assign7780_e6702: f64 = (p.p570 * locals.var_iwe);
        let assign7780_e6703: f64 = (assign7780_e6699 + assign7780_e6702);
        let assign7780_e6706: f64 = (p.p571 * locals.var_iae);
        let assign7780_e6707: f64 = (assign7780_e6703 + assign7780_e6706);
        let assign7780_e6708: f64 = (locals.var_ile * assign7780_e6707);
        (assign7780_e6708,)
    } else {
        (locals.var_thesat_p,)
    }
};
        locals.var_thesat_p = assign7780_e6710;
        locals.var_thesat_p_rv = 0.0;

        let assign7790_e6729: f64 = if (((param_given[572] || param_given[573]) || param_given[574]) || param_given[575]) { 1.0 } else { 0.0 };
        locals.var_guard82 = assign7790_e6729;
        locals.var_guard82_rv = 0.0;

        let (assign7800_e6747,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard82 != 0.0)) {
        let assign7800_e6736: f64 = (p.p573 * locals.var_ile);
        let assign7800_e6737: f64 = (p.p572 + assign7800_e6736);
        let assign7800_e6740: f64 = (p.p574 * locals.var_iwe);
        let assign7800_e6741: f64 = (assign7800_e6737 + assign7800_e6740);
        let assign7800_e6744: f64 = (p.p575 * locals.var_iae);
        let assign7800_e6745: f64 = (assign7800_e6741 + assign7800_e6744);
        (assign7800_e6745,)
    } else {
        (locals.var_stthesat_p,)
    }
};
        locals.var_stthesat_p = assign7800_e6747;
        locals.var_stthesat_p_rv = 0.0;

        let assign7810_e6766: f64 = if (((param_given[576] || param_given[577]) || param_given[578]) || param_given[579]) { 1.0 } else { 0.0 };
        locals.var_guard83 = assign7810_e6766;
        locals.var_guard83_rv = 0.0;

        let (assign7820_e6784,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard83 != 0.0)) {
        let assign7820_e6773: f64 = (p.p577 * locals.var_ile);
        let assign7820_e6774: f64 = (p.p576 + assign7820_e6773);
        let assign7820_e6777: f64 = (p.p578 * locals.var_iwe);
        let assign7820_e6778: f64 = (assign7820_e6774 + assign7820_e6777);
        let assign7820_e6781: f64 = (p.p579 * locals.var_iae);
        let assign7820_e6782: f64 = (assign7820_e6778 + assign7820_e6781);
        (assign7820_e6782,)
    } else {
        (locals.var_thesatb_p,)
    }
};
        locals.var_thesatb_p = assign7820_e6784;
        locals.var_thesatb_p_rv = 0.0;

        let assign7830_e6803: f64 = if (((param_given[580] || param_given[581]) || param_given[582]) || param_given[583]) { 1.0 } else { 0.0 };
        locals.var_guard84 = assign7830_e6803;
        locals.var_guard84_rv = 0.0;

        let (assign7840_e6821,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard84 != 0.0)) {
        let assign7840_e6810: f64 = (p.p581 * locals.var_ile);
        let assign7840_e6811: f64 = (p.p580 + assign7840_e6810);
        let assign7840_e6814: f64 = (p.p582 * locals.var_iwe);
        let assign7840_e6815: f64 = (assign7840_e6811 + assign7840_e6814);
        let assign7840_e6818: f64 = (p.p583 * locals.var_iae);
        let assign7840_e6819: f64 = (assign7840_e6815 + assign7840_e6818);
        (assign7840_e6819,)
    } else {
        (locals.var_thesatg_p,)
    }
};
        locals.var_thesatg_p = assign7840_e6821;
        locals.var_thesatg_p_rv = 0.0;

        let assign7850_e6840: f64 = if (((param_given[584] || param_given[585]) || param_given[586]) || param_given[587]) { 1.0 } else { 0.0 };
        locals.var_guard85 = assign7850_e6840;
        locals.var_guard85_rv = 0.0;

        let (assign7860_e6858,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard85 != 0.0)) {
        let assign7860_e6847: f64 = (p.p585 * locals.var_ile);
        let assign7860_e6848: f64 = (p.p584 + assign7860_e6847);
        let assign7860_e6851: f64 = (p.p586 * locals.var_iwe);
        let assign7860_e6852: f64 = (assign7860_e6848 + assign7860_e6851);
        let assign7860_e6855: f64 = (p.p587 * locals.var_iae);
        let assign7860_e6856: f64 = (assign7860_e6852 + assign7860_e6855);
        (assign7860_e6856,)
    } else {
        (locals.var_ax_p,)
    }
};
        locals.var_ax_p = assign7860_e6858;
        locals.var_ax_p_rv = 0.0;

        let assign7870_e6877: f64 = if (((param_given[588] || param_given[589]) || param_given[590]) || param_given[591]) { 1.0 } else { 0.0 };
        locals.var_guard86 = assign7870_e6877;
        locals.var_guard86_rv = 0.0;

        let (assign7880_e6897,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard86 != 0.0)) {
        let assign7880_e6885: f64 = (p.p589 * locals.var_ile);
        let assign7880_e6886: f64 = (p.p588 + assign7880_e6885);
        let assign7880_e6889: f64 = (p.p590 * locals.var_iwe);
        let assign7880_e6890: f64 = (assign7880_e6886 + assign7880_e6889);
        let assign7880_e6893: f64 = (p.p591 * locals.var_iae);
        let assign7880_e6894: f64 = (assign7880_e6890 + assign7880_e6893);
        let assign7880_e6895: f64 = (locals.var_ile * assign7880_e6894);
        (assign7880_e6895,)
    } else {
        (locals.var_alp_p,)
    }
};
        locals.var_alp_p = assign7880_e6897;
        locals.var_alp_p_rv = 0.0;

        let assign7890_e6916: f64 = if (((param_given[592] || param_given[593]) || param_given[594]) || param_given[595]) { 1.0 } else { 0.0 };
        locals.var_guard87 = assign7890_e6916;
        locals.var_guard87_rv = 0.0;

        let (assign7900_e6934,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard87 != 0.0)) {
        let assign7900_e6923: f64 = (p.p593 * locals.var_ile);
        let assign7900_e6924: f64 = (p.p592 + assign7900_e6923);
        let assign7900_e6927: f64 = (p.p594 * locals.var_iwe);
        let assign7900_e6928: f64 = (assign7900_e6924 + assign7900_e6927);
        let assign7900_e6931: f64 = (p.p595 * locals.var_iae);
        let assign7900_e6932: f64 = (assign7900_e6928 + assign7900_e6931);
        (assign7900_e6932,)
    } else {
        (locals.var_alp1_p,)
    }
};
        locals.var_alp1_p = assign7900_e6934;
        locals.var_alp1_p_rv = 0.0;

        let assign7910_e6953: f64 = if (((param_given[596] || param_given[597]) || param_given[598]) || param_given[599]) { 1.0 } else { 0.0 };
        locals.var_guard88 = assign7910_e6953;
        locals.var_guard88_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_7(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign7920_e6971,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard88 != 0.0)) {
        let assign7920_e6960: f64 = (p.p597 * locals.var_ile);
        let assign7920_e6961: f64 = (p.p596 + assign7920_e6960);
        let assign7920_e6964: f64 = (p.p598 * locals.var_iwe);
        let assign7920_e6965: f64 = (assign7920_e6961 + assign7920_e6964);
        let assign7920_e6968: f64 = (p.p599 * locals.var_iae);
        let assign7920_e6969: f64 = (assign7920_e6965 + assign7920_e6968);
        (assign7920_e6969,)
    } else {
        (locals.var_alp2_p,)
    }
};
        locals.var_alp2_p = assign7920_e6971;
        locals.var_alp2_p_rv = 0.0;

        let assign7930_e6990: f64 = if (((param_given[600] || param_given[601]) || param_given[602]) || param_given[603]) { 1.0 } else { 0.0 };
        locals.var_guard89 = assign7930_e6990;
        locals.var_guard89_rv = 0.0;

        let (assign7940_e7008,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard89 != 0.0)) {
        let assign7940_e6997: f64 = (p.p601 * locals.var_ile);
        let assign7940_e6998: f64 = (p.p600 + assign7940_e6997);
        let assign7940_e7001: f64 = (p.p602 * locals.var_iwe);
        let assign7940_e7002: f64 = (assign7940_e6998 + assign7940_e7001);
        let assign7940_e7005: f64 = (p.p603 * locals.var_iae);
        let assign7940_e7006: f64 = (assign7940_e7002 + assign7940_e7005);
        (assign7940_e7006,)
    } else {
        (locals.var_a1_p,)
    }
};
        locals.var_a1_p = assign7940_e7008;
        locals.var_a1_p_rv = 0.0;

        let assign7950_e7027: f64 = if (((param_given[604] || param_given[605]) || param_given[606]) || param_given[607]) { 1.0 } else { 0.0 };
        locals.var_guard90 = assign7950_e7027;
        locals.var_guard90_rv = 0.0;

        let (assign7960_e7045,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard90 != 0.0)) {
        let assign7960_e7034: f64 = (p.p605 * locals.var_ile);
        let assign7960_e7035: f64 = (p.p604 + assign7960_e7034);
        let assign7960_e7038: f64 = (p.p606 * locals.var_iwe);
        let assign7960_e7039: f64 = (assign7960_e7035 + assign7960_e7038);
        let assign7960_e7042: f64 = (p.p607 * locals.var_iae);
        let assign7960_e7043: f64 = (assign7960_e7039 + assign7960_e7042);
        (assign7960_e7043,)
    } else {
        (locals.var_sta2_p,)
    }
};
        locals.var_sta2_p = assign7960_e7045;
        locals.var_sta2_p_rv = 0.0;

        let assign7970_e7064: f64 = if (((param_given[608] || param_given[609]) || param_given[610]) || param_given[611]) { 1.0 } else { 0.0 };
        locals.var_guard91 = assign7970_e7064;
        locals.var_guard91_rv = 0.0;

        let (assign7980_e7082,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard91 != 0.0)) {
        let assign7980_e7071: f64 = (p.p609 * locals.var_ile);
        let assign7980_e7072: f64 = (p.p608 + assign7980_e7071);
        let assign7980_e7075: f64 = (p.p610 * locals.var_iwe);
        let assign7980_e7076: f64 = (assign7980_e7072 + assign7980_e7075);
        let assign7980_e7079: f64 = (p.p611 * locals.var_iae);
        let assign7980_e7080: f64 = (assign7980_e7076 + assign7980_e7079);
        (assign7980_e7080,)
    } else {
        (locals.var_a3_p,)
    }
};
        locals.var_a3_p = assign7980_e7082;
        locals.var_a3_p_rv = 0.0;

        let assign7990_e7101: f64 = if (((param_given[612] || param_given[613]) || param_given[614]) || param_given[615]) { 1.0 } else { 0.0 };
        locals.var_guard92 = assign7990_e7101;
        locals.var_guard92_rv = 0.0;

        let (assign8000_e7119,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard92 != 0.0)) {
        let assign8000_e7108: f64 = (p.p613 * locals.var_ile);
        let assign8000_e7109: f64 = (p.p612 + assign8000_e7108);
        let assign8000_e7112: f64 = (p.p614 * locals.var_iwe);
        let assign8000_e7113: f64 = (assign8000_e7109 + assign8000_e7112);
        let assign8000_e7116: f64 = (p.p615 * locals.var_iae);
        let assign8000_e7117: f64 = (assign8000_e7113 + assign8000_e7116);
        (assign8000_e7117,)
    } else {
        (locals.var_a4_p,)
    }
};
        locals.var_a4_p = assign8000_e7119;
        locals.var_a4_p_rv = 0.0;

        let assign8010_e7138: f64 = if (((param_given[616] || param_given[617]) || param_given[618]) || param_given[619]) { 1.0 } else { 0.0 };
        locals.var_guard93 = assign8010_e7138;
        locals.var_guard93_rv = 0.0;

        let (assign8020_e7158,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard93 != 0.0)) {
        let assign8020_e7146: f64 = (p.p617 * locals.var_ile);
        let assign8020_e7147: f64 = (p.p616 + assign8020_e7146);
        let assign8020_e7150: f64 = (p.p618 * locals.var_iwe);
        let assign8020_e7151: f64 = (assign8020_e7147 + assign8020_e7150);
        let assign8020_e7154: f64 = (p.p619 * locals.var_iae);
        let assign8020_e7155: f64 = (assign8020_e7151 + assign8020_e7154);
        let assign8020_e7156: f64 = (locals.var_iiae * assign8020_e7155);
        (assign8020_e7156,)
    } else {
        (locals.var_iginv_p,)
    }
};
        locals.var_iginv_p = assign8020_e7158;
        locals.var_iginv_p_rv = 0.0;

        let assign8030_e7177: f64 = if (((param_given[620] || param_given[621]) || param_given[622]) || param_given[623]) { 1.0 } else { 0.0 };
        locals.var_guard94 = assign8030_e7177;
        locals.var_guard94_rv = 0.0;

        let (assign8040_e7197,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard94 != 0.0)) {
        let assign8040_e7185: f64 = (p.p621 * locals.var_ile);
        let assign8040_e7186: f64 = (p.p620 + assign8040_e7185);
        let assign8040_e7189: f64 = (p.p622 * locals.var_iwe);
        let assign8040_e7190: f64 = (assign8040_e7186 + assign8040_e7189);
        let assign8040_e7193: f64 = (p.p623 * locals.var_iae);
        let assign8040_e7194: f64 = (assign8040_e7190 + assign8040_e7193);
        let assign8040_e7195: f64 = (locals.var_iiwe * assign8040_e7194);
        (assign8040_e7195,)
    } else {
        (locals.var_igov_p,)
    }
};
        locals.var_igov_p = assign8040_e7197;
        locals.var_igov_p_rv = 0.0;

        let assign8050_e7216: f64 = if (((param_given[624] || param_given[625]) || param_given[626]) || param_given[627]) { 1.0 } else { 0.0 };
        locals.var_guard95 = assign8050_e7216;
        locals.var_guard95_rv = 0.0;

        let (assign8060_e7236,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard95 != 0.0)) {
        let assign8060_e7224: f64 = (p.p625 * locals.var_ile);
        let assign8060_e7225: f64 = (p.p624 + assign8060_e7224);
        let assign8060_e7228: f64 = (p.p626 * locals.var_iwe);
        let assign8060_e7229: f64 = (assign8060_e7225 + assign8060_e7228);
        let assign8060_e7232: f64 = (p.p627 * locals.var_iae);
        let assign8060_e7233: f64 = (assign8060_e7229 + assign8060_e7232);
        let assign8060_e7234: f64 = (locals.var_iiwe * assign8060_e7233);
        (assign8060_e7234,)
    } else {
        (locals.var_igovd_p,)
    }
};
        locals.var_igovd_p = assign8060_e7236;
        locals.var_igovd_p_rv = 0.0;

        let assign8070_e7255: f64 = if (((param_given[628] || param_given[629]) || param_given[630]) || param_given[631]) { 1.0 } else { 0.0 };
        locals.var_guard96 = assign8070_e7255;
        locals.var_guard96_rv = 0.0;

        let (assign8080_e7273,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard96 != 0.0)) {
        let assign8080_e7262: f64 = (p.p629 * locals.var_ile);
        let assign8080_e7263: f64 = (p.p628 + assign8080_e7262);
        let assign8080_e7266: f64 = (p.p630 * locals.var_iwe);
        let assign8080_e7267: f64 = (assign8080_e7263 + assign8080_e7266);
        let assign8080_e7270: f64 = (p.p631 * locals.var_iae);
        let assign8080_e7271: f64 = (assign8080_e7267 + assign8080_e7270);
        (assign8080_e7271,)
    } else {
        (locals.var_stig_p,)
    }
};
        locals.var_stig_p = assign8080_e7273;
        locals.var_stig_p_rv = 0.0;

        let assign8090_e7292: f64 = if (((param_given[632] || param_given[633]) || param_given[634]) || param_given[635]) { 1.0 } else { 0.0 };
        locals.var_guard97 = assign8090_e7292;
        locals.var_guard97_rv = 0.0;

        let (assign8100_e7312,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard97 != 0.0)) {
        let assign8100_e7300: f64 = (p.p633 * locals.var_ile);
        let assign8100_e7301: f64 = (p.p632 + assign8100_e7300);
        let assign8100_e7304: f64 = (p.p634 * locals.var_iwe);
        let assign8100_e7305: f64 = (assign8100_e7301 + assign8100_e7304);
        let assign8100_e7308: f64 = (p.p635 * locals.var_iae);
        let assign8100_e7309: f64 = (assign8100_e7305 + assign8100_e7308);
        let assign8100_e7310: f64 = (locals.var_iiwe * assign8100_e7309);
        (assign8100_e7310,)
    } else {
        (locals.var_agidl_p,)
    }
};
        locals.var_agidl_p = assign8100_e7312;
        locals.var_agidl_p_rv = 0.0;

        let assign8110_e7331: f64 = if (((param_given[636] || param_given[637]) || param_given[638]) || param_given[639]) { 1.0 } else { 0.0 };
        locals.var_guard98 = assign8110_e7331;
        locals.var_guard98_rv = 0.0;

        let (assign8120_e7351,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard98 != 0.0)) {
        let assign8120_e7339: f64 = (p.p637 * locals.var_ile);
        let assign8120_e7340: f64 = (p.p636 + assign8120_e7339);
        let assign8120_e7343: f64 = (p.p638 * locals.var_iwe);
        let assign8120_e7344: f64 = (assign8120_e7340 + assign8120_e7343);
        let assign8120_e7347: f64 = (p.p639 * locals.var_iae);
        let assign8120_e7348: f64 = (assign8120_e7344 + assign8120_e7347);
        let assign8120_e7349: f64 = (locals.var_iiwe * assign8120_e7348);
        (assign8120_e7349,)
    } else {
        (locals.var_agidld_p,)
    }
};
        locals.var_agidld_p = assign8120_e7351;
        locals.var_agidld_p_rv = 0.0;

        let assign8130_e7370: f64 = if (((param_given[640] || param_given[641]) || param_given[642]) || param_given[643]) { 1.0 } else { 0.0 };
        locals.var_guard99 = assign8130_e7370;
        locals.var_guard99_rv = 0.0;

        let (assign8140_e7388,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard99 != 0.0)) {
        let assign8140_e7377: f64 = (p.p641 * locals.var_ile);
        let assign8140_e7378: f64 = (p.p640 + assign8140_e7377);
        let assign8140_e7381: f64 = (p.p642 * locals.var_iwe);
        let assign8140_e7382: f64 = (assign8140_e7378 + assign8140_e7381);
        let assign8140_e7385: f64 = (p.p643 * locals.var_iae);
        let assign8140_e7386: f64 = (assign8140_e7382 + assign8140_e7385);
        (assign8140_e7386,)
    } else {
        (locals.var_stbgidl_p,)
    }
};
        locals.var_stbgidl_p = assign8140_e7388;
        locals.var_stbgidl_p_rv = 0.0;

        let assign8150_e7407: f64 = if (((param_given[644] || param_given[645]) || param_given[646]) || param_given[647]) { 1.0 } else { 0.0 };
        locals.var_guard100 = assign8150_e7407;
        locals.var_guard100_rv = 0.0;

        let (assign8160_e7425,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard100 != 0.0)) {
        let assign8160_e7414: f64 = (p.p645 * locals.var_ile);
        let assign8160_e7415: f64 = (p.p644 + assign8160_e7414);
        let assign8160_e7418: f64 = (p.p646 * locals.var_iwe);
        let assign8160_e7419: f64 = (assign8160_e7415 + assign8160_e7418);
        let assign8160_e7422: f64 = (p.p647 * locals.var_iae);
        let assign8160_e7423: f64 = (assign8160_e7419 + assign8160_e7422);
        (assign8160_e7423,)
    } else {
        (locals.var_stbgidld_p,)
    }
};
        locals.var_stbgidld_p = assign8160_e7425;
        locals.var_stbgidld_p_rv = 0.0;

        let assign8170_e7444: f64 = if (((param_given[648] || param_given[649]) || param_given[650]) || param_given[651]) { 1.0 } else { 0.0 };
        locals.var_guard101 = assign8170_e7444;
        locals.var_guard101_rv = 0.0;

        let (assign8180_e7468,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard101 != 0.0)) {
        let assign8180_e7450: f64 = (locals.var_iiwecv * locals.var_lecv);
        let assign8180_e7452: f64 = (assign8180_e7450 / 1e-6);
        let assign8180_e7456: f64 = (p.p649 * locals.var_ile);
        let assign8180_e7457: f64 = (p.p648 + assign8180_e7456);
        let assign8180_e7460: f64 = (p.p650 * locals.var_iwe);
        let assign8180_e7461: f64 = (assign8180_e7457 + assign8180_e7460);
        let assign8180_e7464: f64 = (p.p651 * locals.var_iae);
        let assign8180_e7465: f64 = (assign8180_e7461 + assign8180_e7464);
        let assign8180_e7466: f64 = (assign8180_e7452 * assign8180_e7465);
        (assign8180_e7466,)
    } else {
        (locals.var_cox_p,)
    }
};
        locals.var_cox_p = assign8180_e7468;
        locals.var_cox_p_rv = 0.0;

        let assign8190_e7487: f64 = if (((param_given[652] || param_given[653]) || param_given[654]) || param_given[655]) { 1.0 } else { 0.0 };
        locals.var_guard102 = assign8190_e7487;
        locals.var_guard102_rv = 0.0;

        let (assign8200_e7505,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard102 != 0.0)) {
        let assign8200_e7494: f64 = (p.p653 * locals.var_ile);
        let assign8200_e7495: f64 = (p.p652 + assign8200_e7494);
        let assign8200_e7498: f64 = (p.p654 * locals.var_iwe);
        let assign8200_e7499: f64 = (assign8200_e7495 + assign8200_e7498);
        let assign8200_e7502: f64 = (p.p655 * locals.var_iae);
        let assign8200_e7503: f64 = (assign8200_e7499 + assign8200_e7502);
        (assign8200_e7503,)
    } else {
        (locals.var_delvtac_p,)
    }
};
        locals.var_delvtac_p = assign8200_e7505;
        locals.var_delvtac_p_rv = 0.0;

        let assign8210_e7524: f64 = if (((param_given[656] || param_given[657]) || param_given[658]) || param_given[659]) { 1.0 } else { 0.0 };
        locals.var_guard103 = assign8210_e7524;
        locals.var_guard103_rv = 0.0;

        let (assign8220_e7542,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard103 != 0.0)) {
        let assign8220_e7531: f64 = (p.p657 * locals.var_ile);
        let assign8220_e7532: f64 = (p.p656 + assign8220_e7531);
        let assign8220_e7535: f64 = (p.p658 * locals.var_iwe);
        let assign8220_e7536: f64 = (assign8220_e7532 + assign8220_e7535);
        let assign8220_e7539: f64 = (p.p659 * locals.var_iae);
        let assign8220_e7540: f64 = (assign8220_e7536 + assign8220_e7539);
        (assign8220_e7540,)
    } else {
        (locals.var_facneffac_p,)
    }
};
        locals.var_facneffac_p = assign8220_e7542;
        locals.var_facneffac_p_rv = 0.0;

        let assign8230_e7581: f64 = if (((((((param_given[660] || param_given[661]) || param_given[662]) || param_given[663]) || param_given[568]) || param_given[569]) || param_given[570]) || param_given[571]) { 1.0 } else { 0.0 };
        locals.var_guard104 = assign8230_e7581;
        locals.var_guard104_rv = 0.0;

        let (assign8240_e7587,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard104 != 0.0)) {
        (p.p568,)
    } else {
        (locals.var_poparam_i,)
    }
};
        locals.var_poparam_i = assign8240_e7587;
        locals.var_poparam_i_rv = 0.0;

        let assign8250_e7589: f64 = if param_given[660] { 1.0 } else { 0.0 };
        let assign8250_e7591: f64 = if assign8250_e7589 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard105 = assign8250_e7591;
        locals.var_guard105_rv = 0.0;

        let (assign8260_e7599,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard104 != 0.0)) && (locals.var_guard105 != 0.0)) {
        (p.p660,)
    } else {
        (locals.var_poparam_i,)
    }
};
        locals.var_poparam_i = assign8260_e7599;
        locals.var_poparam_i_rv = 0.0;

        let (assign8270_e7605,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard104 != 0.0)) {
        (p.p569,)
    } else {
        (locals.var_plparam_i,)
    }
};
        locals.var_plparam_i = assign8270_e7605;
        locals.var_plparam_i_rv = 0.0;

        let assign8280_e7607: f64 = if param_given[661] { 1.0 } else { 0.0 };
        let assign8280_e7609: f64 = if assign8280_e7607 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard106 = assign8280_e7609;
        locals.var_guard106_rv = 0.0;

        let (assign8290_e7617,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard104 != 0.0)) && (locals.var_guard106 != 0.0)) {
        (p.p661,)
    } else {
        (locals.var_plparam_i,)
    }
};
        locals.var_plparam_i = assign8290_e7617;
        locals.var_plparam_i_rv = 0.0;

        let (assign8300_e7623,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard104 != 0.0)) {
        (p.p570,)
    } else {
        (locals.var_pwparam_i,)
    }
};
        locals.var_pwparam_i = assign8300_e7623;
        locals.var_pwparam_i_rv = 0.0;

        let assign8310_e7625: f64 = if param_given[662] { 1.0 } else { 0.0 };
        let assign8310_e7627: f64 = if assign8310_e7625 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard107 = assign8310_e7627;
        locals.var_guard107_rv = 0.0;

        let (assign8320_e7635,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard104 != 0.0)) && (locals.var_guard107 != 0.0)) {
        (p.p662,)
    } else {
        (locals.var_pwparam_i,)
    }
};
        locals.var_pwparam_i = assign8320_e7635;
        locals.var_pwparam_i_rv = 0.0;

        let (assign8330_e7641,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard104 != 0.0)) {
        (p.p571,)
    } else {
        (locals.var_plwparam_i,)
    }
};
        locals.var_plwparam_i = assign8330_e7641;
        locals.var_plwparam_i_rv = 0.0;

        let assign8340_e7643: f64 = if param_given[663] { 1.0 } else { 0.0 };
        let assign8340_e7645: f64 = if assign8340_e7643 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard108 = assign8340_e7645;
        locals.var_guard108_rv = 0.0;

        let (assign8350_e7653,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard104 != 0.0)) && (locals.var_guard108 != 0.0)) {
        (p.p663,)
    } else {
        (locals.var_plwparam_i,)
    }
};
        locals.var_plwparam_i = assign8350_e7653;
        locals.var_plwparam_i_rv = 0.0;

        let (assign8360_e7673,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard104 != 0.0)) {
        let assign8360_e7661: f64 = (locals.var_plparam_i * locals.var_ile);
        let assign8360_e7662: f64 = (locals.var_poparam_i + assign8360_e7661);
        let assign8360_e7665: f64 = (locals.var_pwparam_i * locals.var_iwe);
        let assign8360_e7666: f64 = (assign8360_e7662 + assign8360_e7665);
        let assign8360_e7669: f64 = (locals.var_plwparam_i * locals.var_iae);
        let assign8360_e7670: f64 = (assign8360_e7666 + assign8360_e7669);
        let assign8360_e7671: f64 = (locals.var_ile * assign8360_e7670);
        (assign8360_e7671,)
    } else {
        (locals.var_thesatac_p,)
    }
};
        locals.var_thesatac_p = assign8360_e7673;
        locals.var_thesatac_p_rv = 0.0;

        let assign8370_e7712: f64 = if (((((((param_given[664] || param_given[665]) || param_given[666]) || param_given[667]) || param_given[584]) || param_given[585]) || param_given[586]) || param_given[587]) { 1.0 } else { 0.0 };
        locals.var_guard109 = assign8370_e7712;
        locals.var_guard109_rv = 0.0;

        let (assign8380_e7718,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard109 != 0.0)) {
        (p.p584,)
    } else {
        (locals.var_poparam_i,)
    }
};
        locals.var_poparam_i = assign8380_e7718;
        locals.var_poparam_i_rv = 0.0;

        let assign8390_e7720: f64 = if param_given[664] { 1.0 } else { 0.0 };
        let assign8390_e7722: f64 = if assign8390_e7720 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard110 = assign8390_e7722;
        locals.var_guard110_rv = 0.0;

        let (assign8400_e7730,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard109 != 0.0)) && (locals.var_guard110 != 0.0)) {
        (p.p664,)
    } else {
        (locals.var_poparam_i,)
    }
};
        locals.var_poparam_i = assign8400_e7730;
        locals.var_poparam_i_rv = 0.0;

        let (assign8410_e7736,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard109 != 0.0)) {
        (p.p585,)
    } else {
        (locals.var_plparam_i,)
    }
};
        locals.var_plparam_i = assign8410_e7736;
        locals.var_plparam_i_rv = 0.0;

        let assign8420_e7738: f64 = if param_given[665] { 1.0 } else { 0.0 };
        let assign8420_e7740: f64 = if assign8420_e7738 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard111 = assign8420_e7740;
        locals.var_guard111_rv = 0.0;

        let (assign8430_e7748,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard109 != 0.0)) && (locals.var_guard111 != 0.0)) {
        (p.p665,)
    } else {
        (locals.var_plparam_i,)
    }
};
        locals.var_plparam_i = assign8430_e7748;
        locals.var_plparam_i_rv = 0.0;

        let (assign8440_e7754,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard109 != 0.0)) {
        (p.p586,)
    } else {
        (locals.var_pwparam_i,)
    }
};
        locals.var_pwparam_i = assign8440_e7754;
        locals.var_pwparam_i_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_8(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let assign8450_e7756: f64 = if param_given[666] { 1.0 } else { 0.0 };
        let assign8450_e7758: f64 = if assign8450_e7756 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard112 = assign8450_e7758;
        locals.var_guard112_rv = 0.0;

        let (assign8460_e7766,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard109 != 0.0)) && (locals.var_guard112 != 0.0)) {
        (p.p666,)
    } else {
        (locals.var_pwparam_i,)
    }
};
        locals.var_pwparam_i = assign8460_e7766;
        locals.var_pwparam_i_rv = 0.0;

        let (assign8470_e7772,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard109 != 0.0)) {
        (p.p587,)
    } else {
        (locals.var_plwparam_i,)
    }
};
        locals.var_plwparam_i = assign8470_e7772;
        locals.var_plwparam_i_rv = 0.0;

        let assign8480_e7774: f64 = if param_given[667] { 1.0 } else { 0.0 };
        let assign8480_e7776: f64 = if assign8480_e7774 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard113 = assign8480_e7776;
        locals.var_guard113_rv = 0.0;

        let (assign8490_e7784,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard109 != 0.0)) && (locals.var_guard113 != 0.0)) {
        (p.p667,)
    } else {
        (locals.var_plwparam_i,)
    }
};
        locals.var_plwparam_i = assign8490_e7784;
        locals.var_plwparam_i_rv = 0.0;

        let (assign8500_e7804,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard109 != 0.0)) {
        let assign8500_e7792: f64 = (locals.var_plparam_i * locals.var_ile);
        let assign8500_e7793: f64 = (locals.var_poparam_i + assign8500_e7792);
        let assign8500_e7796: f64 = (locals.var_pwparam_i * locals.var_iwe);
        let assign8500_e7797: f64 = (assign8500_e7793 + assign8500_e7796);
        let assign8500_e7800: f64 = (locals.var_plwparam_i * locals.var_iae);
        let assign8500_e7801: f64 = (assign8500_e7797 + assign8500_e7800);
        let assign8500_e7802: f64 = assign8500_e7801;
        (assign8500_e7802,)
    } else {
        (locals.var_axac_p,)
    }
};
        locals.var_axac_p = assign8500_e7804;
        locals.var_axac_p_rv = 0.0;

        let assign8510_e7823: f64 = if (((param_given[668] || param_given[669]) || param_given[670]) || param_given[671]) { 1.0 } else { 0.0 };
        locals.var_guard114 = assign8510_e7823;
        locals.var_guard114_rv = 0.0;

        let (assign8520_e7843,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard114 != 0.0)) {
        let assign8520_e7831: f64 = (p.p669 * locals.var_ile);
        let assign8520_e7832: f64 = (p.p668 + assign8520_e7831);
        let assign8520_e7835: f64 = (p.p670 * locals.var_iwe);
        let assign8520_e7836: f64 = (assign8520_e7832 + assign8520_e7835);
        let assign8520_e7839: f64 = (p.p671 * locals.var_iae);
        let assign8520_e7840: f64 = (assign8520_e7836 + assign8520_e7839);
        let assign8520_e7841: f64 = (locals.var_ile * assign8520_e7840);
        (assign8520_e7841,)
    } else {
        (locals.var_alpac_p,)
    }
};
        locals.var_alpac_p = assign8520_e7843;
        locals.var_alpac_p_rv = 0.0;

        let assign8530_e7862: f64 = if (((param_given[672] || param_given[673]) || param_given[674]) || param_given[675]) { 1.0 } else { 0.0 };
        locals.var_guard115 = assign8530_e7862;
        locals.var_guard115_rv = 0.0;

        let (assign8540_e7882,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard115 != 0.0)) {
        let assign8540_e7870: f64 = (p.p673 * locals.var_ile);
        let assign8540_e7871: f64 = (p.p672 + assign8540_e7870);
        let assign8540_e7874: f64 = (p.p674 * locals.var_iwe);
        let assign8540_e7875: f64 = (assign8540_e7871 + assign8540_e7874);
        let assign8540_e7878: f64 = (p.p675 * locals.var_iae);
        let assign8540_e7879: f64 = (assign8540_e7875 + assign8540_e7878);
        let assign8540_e7880: f64 = (locals.var_ile * assign8540_e7879);
        (assign8540_e7880,)
    } else {
        (locals.var_alp1ac_p,)
    }
};
        locals.var_alp1ac_p = assign8540_e7882;
        locals.var_alp1ac_p_rv = 0.0;

        let assign8550_e7901: f64 = if (((param_given[676] || param_given[677]) || param_given[678]) || param_given[679]) { 1.0 } else { 0.0 };
        locals.var_guard116 = assign8550_e7901;
        locals.var_guard116_rv = 0.0;

        let (assign8560_e7921,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard116 != 0.0)) {
        let assign8560_e7909: f64 = (p.p677 * locals.var_ile);
        let assign8560_e7910: f64 = (p.p676 + assign8560_e7909);
        let assign8560_e7913: f64 = (p.p678 * locals.var_iwe);
        let assign8560_e7914: f64 = (assign8560_e7910 + assign8560_e7913);
        let assign8560_e7917: f64 = (p.p679 * locals.var_iae);
        let assign8560_e7918: f64 = (assign8560_e7914 + assign8560_e7917);
        let assign8560_e7919: f64 = (locals.var_iiwecv * assign8560_e7918);
        (assign8560_e7919,)
    } else {
        (locals.var_cgov_p,)
    }
};
        locals.var_cgov_p = assign8560_e7921;
        locals.var_cgov_p_rv = 0.0;

        let assign8570_e7940: f64 = if (((param_given[680] || param_given[681]) || param_given[682]) || param_given[683]) { 1.0 } else { 0.0 };
        locals.var_guard117 = assign8570_e7940;
        locals.var_guard117_rv = 0.0;

        let (assign8580_e7960,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard117 != 0.0)) {
        let assign8580_e7948: f64 = (p.p681 * locals.var_ile);
        let assign8580_e7949: f64 = (p.p680 + assign8580_e7948);
        let assign8580_e7952: f64 = (p.p682 * locals.var_iwe);
        let assign8580_e7953: f64 = (assign8580_e7949 + assign8580_e7952);
        let assign8580_e7956: f64 = (p.p683 * locals.var_iae);
        let assign8580_e7957: f64 = (assign8580_e7953 + assign8580_e7956);
        let assign8580_e7958: f64 = (locals.var_iiwecv * assign8580_e7957);
        (assign8580_e7958,)
    } else {
        (locals.var_cgovd_p,)
    }
};
        locals.var_cgovd_p = assign8580_e7960;
        locals.var_cgovd_p_rv = 0.0;

        let assign8590_e7979: f64 = if (((param_given[684] || param_given[685]) || param_given[686]) || param_given[687]) { 1.0 } else { 0.0 };
        locals.var_guard118 = assign8590_e7979;
        locals.var_guard118_rv = 0.0;

        let (assign8600_e7999,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard118 != 0.0)) {
        let assign8600_e7987: f64 = (p.p685 * locals.var_ile);
        let assign8600_e7988: f64 = (p.p684 + assign8600_e7987);
        let assign8600_e7991: f64 = (p.p686 * locals.var_iwe);
        let assign8600_e7992: f64 = (assign8600_e7988 + assign8600_e7991);
        let assign8600_e7995: f64 = (p.p687 * locals.var_iae);
        let assign8600_e7996: f64 = (assign8600_e7992 + assign8600_e7995);
        let assign8600_e7997: f64 = (locals.var_iilcv * assign8600_e7996);
        (assign8600_e7997,)
    } else {
        (locals.var_cgbov_p,)
    }
};
        locals.var_cgbov_p = assign8600_e7999;
        locals.var_cgbov_p_rv = 0.0;

        let assign8610_e8018: f64 = if (((param_given[688] || param_given[689]) || param_given[690]) || param_given[691]) { 1.0 } else { 0.0 };
        locals.var_guard119 = assign8610_e8018;
        locals.var_guard119_rv = 0.0;

        let (assign8620_e8038,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard119 != 0.0)) {
        let assign8620_e8026: f64 = (p.p689 * locals.var_ile);
        let assign8620_e8027: f64 = (p.p688 + assign8620_e8026);
        let assign8620_e8030: f64 = (p.p690 * locals.var_iwe);
        let assign8620_e8031: f64 = (assign8620_e8027 + assign8620_e8030);
        let assign8620_e8034: f64 = (p.p691 * locals.var_iae);
        let assign8620_e8035: f64 = (assign8620_e8031 + assign8620_e8034);
        let assign8620_e8036: f64 = (locals.var_iiwecv * assign8620_e8035);
        (assign8620_e8036,)
    } else {
        (locals.var_cinr_p,)
    }
};
        locals.var_cinr_p = assign8620_e8038;
        locals.var_cinr_p_rv = 0.0;

        let assign8630_e8057: f64 = if (((param_given[692] || param_given[693]) || param_given[694]) || param_given[695]) { 1.0 } else { 0.0 };
        locals.var_guard120 = assign8630_e8057;
        locals.var_guard120_rv = 0.0;

        let (assign8640_e8077,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard120 != 0.0)) {
        let assign8640_e8065: f64 = (p.p693 * locals.var_ile);
        let assign8640_e8066: f64 = (p.p692 + assign8640_e8065);
        let assign8640_e8069: f64 = (p.p694 * locals.var_iwe);
        let assign8640_e8070: f64 = (assign8640_e8066 + assign8640_e8069);
        let assign8640_e8073: f64 = (p.p695 * locals.var_iae);
        let assign8640_e8074: f64 = (assign8640_e8070 + assign8640_e8073);
        let assign8640_e8075: f64 = (locals.var_iiwecv * assign8640_e8074);
        (assign8640_e8075,)
    } else {
        (locals.var_cinrd_p,)
    }
};
        locals.var_cinrd_p = assign8640_e8077;
        locals.var_cinrd_p_rv = 0.0;

        let assign8770_e8330: f64 = if (((param_given[720] || param_given[721]) || param_given[722]) || param_given[723]) { 1.0 } else { 0.0 };
        locals.var_guard127 = assign8770_e8330;
        locals.var_guard127_rv = 0.0;

        let (assign8780_e8348,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard127 != 0.0)) {
        let assign8780_e8337: f64 = (p.p721 * locals.var_ile);
        let assign8780_e8338: f64 = (p.p720 + assign8780_e8337);
        let assign8780_e8341: f64 = (p.p722 * locals.var_iwe);
        let assign8780_e8342: f64 = (assign8780_e8338 + assign8780_e8341);
        let assign8780_e8345: f64 = (p.p723 * locals.var_iae);
        let assign8780_e8346: f64 = (assign8780_e8342 + assign8780_e8345);
        (assign8780_e8346,)
    } else {
        (locals.var_vfbedge_p,)
    }
};
        locals.var_vfbedge_p = assign8780_e8348;
        locals.var_vfbedge_p_rv = 0.0;

        let assign8790_e8367: f64 = if (((param_given[724] || param_given[725]) || param_given[726]) || param_given[727]) { 1.0 } else { 0.0 };
        locals.var_guard128 = assign8790_e8367;
        locals.var_guard128_rv = 0.0;

        let (assign8800_e8385,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard128 != 0.0)) {
        let assign8800_e8374: f64 = (p.p725 * locals.var_ile);
        let assign8800_e8375: f64 = (p.p724 + assign8800_e8374);
        let assign8800_e8378: f64 = (p.p726 * locals.var_iwe);
        let assign8800_e8379: f64 = (assign8800_e8375 + assign8800_e8378);
        let assign8800_e8382: f64 = (p.p727 * locals.var_iae);
        let assign8800_e8383: f64 = (assign8800_e8379 + assign8800_e8382);
        (assign8800_e8383,)
    } else {
        (locals.var_stvfbedge_p,)
    }
};
        locals.var_stvfbedge_p = assign8800_e8385;
        locals.var_stvfbedge_p_rv = 0.0;

        let assign8810_e8404: f64 = if (((param_given[728] || param_given[729]) || param_given[730]) || param_given[731]) { 1.0 } else { 0.0 };
        locals.var_guard129 = assign8810_e8404;
        locals.var_guard129_rv = 0.0;

        let (assign8820_e8422,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard129 != 0.0)) {
        let assign8820_e8411: f64 = (p.p729 * locals.var_ile);
        let assign8820_e8412: f64 = (p.p728 + assign8820_e8411);
        let assign8820_e8415: f64 = (p.p730 * locals.var_iwe);
        let assign8820_e8416: f64 = (assign8820_e8412 + assign8820_e8415);
        let assign8820_e8419: f64 = (p.p731 * locals.var_iae);
        let assign8820_e8420: f64 = (assign8820_e8416 + assign8820_e8419);
        (assign8820_e8420,)
    } else {
        (locals.var_dphibedge_p,)
    }
};
        locals.var_dphibedge_p = assign8820_e8422;
        locals.var_dphibedge_p_rv = 0.0;

        let assign8830_e8441: f64 = if (((param_given[732] || param_given[733]) || param_given[734]) || param_given[735]) { 1.0 } else { 0.0 };
        locals.var_guard130 = assign8830_e8441;
        locals.var_guard130_rv = 0.0;

        let (assign8840_e8459,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard130 != 0.0)) {
        let assign8840_e8448: f64 = (p.p733 * locals.var_ile);
        let assign8840_e8449: f64 = (p.p732 + assign8840_e8448);
        let assign8840_e8452: f64 = (p.p734 * locals.var_iwe);
        let assign8840_e8453: f64 = (assign8840_e8449 + assign8840_e8452);
        let assign8840_e8456: f64 = (p.p735 * locals.var_iae);
        let assign8840_e8457: f64 = (assign8840_e8453 + assign8840_e8456);
        (assign8840_e8457,)
    } else {
        (locals.var_neffedge_p,)
    }
};
        locals.var_neffedge_p = assign8840_e8459;
        locals.var_neffedge_p_rv = 0.0;

        let assign8850_e8478: f64 = if (((param_given[736] || param_given[737]) || param_given[738]) || param_given[739]) { 1.0 } else { 0.0 };
        locals.var_guard131 = assign8850_e8478;
        locals.var_guard131_rv = 0.0;

        let (assign8860_e8496,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard131 != 0.0)) {
        let assign8860_e8485: f64 = (p.p737 * locals.var_ile);
        let assign8860_e8486: f64 = (p.p736 + assign8860_e8485);
        let assign8860_e8489: f64 = (p.p738 * locals.var_iwe);
        let assign8860_e8490: f64 = (assign8860_e8486 + assign8860_e8489);
        let assign8860_e8493: f64 = (p.p739 * locals.var_iae);
        let assign8860_e8494: f64 = (assign8860_e8490 + assign8860_e8493);
        (assign8860_e8494,)
    } else {
        (locals.var_ctedge_p,)
    }
};
        locals.var_ctedge_p = assign8860_e8496;
        locals.var_ctedge_p_rv = 0.0;

        let assign8870_e8515: f64 = if (((param_given[740] || param_given[741]) || param_given[742]) || param_given[743]) { 1.0 } else { 0.0 };
        locals.var_guard132 = assign8870_e8515;
        locals.var_guard132_rv = 0.0;

        let (assign8880_e8537,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard132 != 0.0)) {
        let assign8880_e8521: f64 = (locals.var_we_edge / locals.var_le);
        let assign8880_e8525: f64 = (p.p741 * locals.var_ile);
        let assign8880_e8526: f64 = (p.p740 + assign8880_e8525);
        let assign8880_e8529: f64 = (p.p742 * locals.var_iwe);
        let assign8880_e8530: f64 = (assign8880_e8526 + assign8880_e8529);
        let assign8880_e8533: f64 = (p.p743 * locals.var_iae);
        let assign8880_e8534: f64 = (assign8880_e8530 + assign8880_e8533);
        let assign8880_e8535: f64 = (assign8880_e8521 * assign8880_e8534);
        (assign8880_e8535,)
    } else {
        (locals.var_betnedge_p,)
    }
};
        locals.var_betnedge_p = assign8880_e8537;
        locals.var_betnedge_p_rv = 0.0;

        let assign8890_e8556: f64 = if (((param_given[744] || param_given[745]) || param_given[746]) || param_given[747]) { 1.0 } else { 0.0 };
        locals.var_guard133 = assign8890_e8556;
        locals.var_guard133_rv = 0.0;

        let (assign8900_e8574,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard133 != 0.0)) {
        let assign8900_e8563: f64 = (p.p745 * locals.var_ile);
        let assign8900_e8564: f64 = (p.p744 + assign8900_e8563);
        let assign8900_e8567: f64 = (p.p746 * locals.var_iwe);
        let assign8900_e8568: f64 = (assign8900_e8564 + assign8900_e8567);
        let assign8900_e8571: f64 = (p.p747 * locals.var_iae);
        let assign8900_e8572: f64 = (assign8900_e8568 + assign8900_e8571);
        (assign8900_e8572,)
    } else {
        (locals.var_stbetedge_p,)
    }
};
        locals.var_stbetedge_p = assign8900_e8574;
        locals.var_stbetedge_p_rv = 0.0;

        let assign8910_e8593: f64 = if (((param_given[748] || param_given[749]) || param_given[750]) || param_given[751]) { 1.0 } else { 0.0 };
        locals.var_guard134 = assign8910_e8593;
        locals.var_guard134_rv = 0.0;

        let (assign8920_e8613,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard134 != 0.0)) {
        let assign8920_e8601: f64 = (p.p749 * locals.var_ile);
        let assign8920_e8602: f64 = (p.p748 + assign8920_e8601);
        let assign8920_e8605: f64 = (p.p750 * locals.var_iwe);
        let assign8920_e8606: f64 = (assign8920_e8602 + assign8920_e8605);
        let assign8920_e8609: f64 = (p.p751 * locals.var_iae);
        let assign8920_e8610: f64 = (assign8920_e8606 + assign8920_e8609);
        let assign8920_e8611: f64 = (locals.var_ile2 * assign8920_e8610);
        (assign8920_e8611,)
    } else {
        (locals.var_psceedge_p,)
    }
};
        locals.var_psceedge_p = assign8920_e8613;
        locals.var_psceedge_p_rv = 0.0;

        let assign8930_e8632: f64 = if (((param_given[752] || param_given[753]) || param_given[754]) || param_given[755]) { 1.0 } else { 0.0 };
        locals.var_guard135 = assign8930_e8632;
        locals.var_guard135_rv = 0.0;

        let (assign8940_e8650,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard135 != 0.0)) {
        let assign8940_e8639: f64 = (p.p753 * locals.var_ile);
        let assign8940_e8640: f64 = (p.p752 + assign8940_e8639);
        let assign8940_e8643: f64 = (p.p754 * locals.var_iwe);
        let assign8940_e8644: f64 = (assign8940_e8640 + assign8940_e8643);
        let assign8940_e8647: f64 = (p.p755 * locals.var_iae);
        let assign8940_e8648: f64 = (assign8940_e8644 + assign8940_e8647);
        (assign8940_e8648,)
    } else {
        (locals.var_pscebedge_p,)
    }
};
        locals.var_pscebedge_p = assign8940_e8650;
        locals.var_pscebedge_p_rv = 0.0;

        let assign8950_e8669: f64 = if (((param_given[756] || param_given[757]) || param_given[758]) || param_given[759]) { 1.0 } else { 0.0 };
        locals.var_guard136 = assign8950_e8669;
        locals.var_guard136_rv = 0.0;

        let (assign8960_e8687,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard136 != 0.0)) {
        let assign8960_e8676: f64 = (p.p757 * locals.var_ile);
        let assign8960_e8677: f64 = (p.p756 + assign8960_e8676);
        let assign8960_e8680: f64 = (p.p758 * locals.var_iwe);
        let assign8960_e8681: f64 = (assign8960_e8677 + assign8960_e8680);
        let assign8960_e8684: f64 = (p.p759 * locals.var_iae);
        let assign8960_e8685: f64 = (assign8960_e8681 + assign8960_e8684);
        (assign8960_e8685,)
    } else {
        (locals.var_pscededge_p,)
    }
};
        locals.var_pscededge_p = assign8960_e8687;
        locals.var_pscededge_p_rv = 0.0;

        let assign8970_e8706: f64 = if (((param_given[760] || param_given[761]) || param_given[762]) || param_given[763]) { 1.0 } else { 0.0 };
        locals.var_guard137 = assign8970_e8706;
        locals.var_guard137_rv = 0.0;

        let (assign8980_e8726,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard137 != 0.0)) {
        let assign8980_e8714: f64 = (p.p761 * locals.var_ile);
        let assign8980_e8715: f64 = (p.p760 + assign8980_e8714);
        let assign8980_e8718: f64 = (p.p762 * locals.var_iwe);
        let assign8980_e8719: f64 = (assign8980_e8715 + assign8980_e8718);
        let assign8980_e8722: f64 = (p.p763 * locals.var_iae);
        let assign8980_e8723: f64 = (assign8980_e8719 + assign8980_e8722);
        let assign8980_e8724: f64 = (locals.var_ile2 * assign8980_e8723);
        (assign8980_e8724,)
    } else {
        (locals.var_cfedge_p,)
    }
};
        locals.var_cfedge_p = assign8980_e8726;
        locals.var_cfedge_p_rv = 0.0;

        let assign8990_e8745: f64 = if (((param_given[768] || param_given[769]) || param_given[770]) || param_given[771]) { 1.0 } else { 0.0 };
        locals.var_guard138 = assign8990_e8745;
        locals.var_guard138_rv = 0.0;

        let (assign9000_e8763,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard138 != 0.0)) {
        let assign9000_e8752: f64 = (p.p769 * locals.var_ile);
        let assign9000_e8753: f64 = (p.p768 + assign9000_e8752);
        let assign9000_e8756: f64 = (p.p770 * locals.var_iwe);
        let assign9000_e8757: f64 = (assign9000_e8753 + assign9000_e8756);
        let assign9000_e8760: f64 = (p.p771 * locals.var_iae);
        let assign9000_e8761: f64 = (assign9000_e8757 + assign9000_e8760);
        (assign9000_e8761,)
    } else {
        (locals.var_cfdedge_p,)
    }
};
        locals.var_cfdedge_p = assign9000_e8763;
        locals.var_cfdedge_p_rv = 0.0;

        let assign9010_e8782: f64 = if (((param_given[764] || param_given[765]) || param_given[766]) || param_given[767]) { 1.0 } else { 0.0 };
        locals.var_guard139 = assign9010_e8782;
        locals.var_guard139_rv = 0.0;

        let (assign9020_e8800,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard139 != 0.0)) {
        let assign9020_e8789: f64 = (p.p765 * locals.var_ile);
        let assign9020_e8790: f64 = (p.p764 + assign9020_e8789);
        let assign9020_e8793: f64 = (p.p766 * locals.var_iwe);
        let assign9020_e8794: f64 = (assign9020_e8790 + assign9020_e8793);
        let assign9020_e8797: f64 = (p.p767 * locals.var_iae);
        let assign9020_e8798: f64 = (assign9020_e8794 + assign9020_e8797);
        (assign9020_e8798,)
    } else {
        (locals.var_cfbedge_p,)
    }
};
        locals.var_cfbedge_p = assign9020_e8800;
        locals.var_cfbedge_p_rv = 0.0;

        let (assign9090_e8921,) = {
    if (locals.var_guard36 != 0.0) {
        (0.0,)
    } else {
        (locals.var_tmpa,)
    }
};
        locals.var_tmpa = assign9090_e8921;
        locals.var_tmpa_rv = 0.0;

        let (assign9100_e8925,) = {
    if (locals.var_guard36 != 0.0) {
        (0.0,)
    } else {
        (locals.var_tmpb,)
    }
};
        locals.var_tmpb = assign9100_e8925;
        locals.var_tmpb_rv = 0.0;

        let (assign9110_e8929,) = {
    if (locals.var_guard36 != 0.0) {
        (0.0,)
    } else {
        (locals.var_loop_,)
    }
};
        locals.var_loop_ = assign9110_e8929;
        locals.var_loop__rv = 0.0;

        let (assign9120_e8933,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p788,)
    } else {
        (locals.var_kvsatac_i,)
    }
};
        locals.var_kvsatac_i = assign9120_e8933;
        locals.var_kvsatac_i_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_9(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let assign9130_e8935: f64 = if param_given[789] { 1.0 } else { 0.0 };
        let assign9130_e8937: f64 = if assign9130_e8935 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard143 = assign9130_e8937;
        locals.var_guard143_rv = 0.0;

        let (assign9140_e8943,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard143 != 0.0)) {
        (p.p789,)
    } else {
        (locals.var_kvsatac_i,)
    }
};
        locals.var_kvsatac_i = assign9140_e8943;
        locals.var_kvsatac_i_rv = 0.0;

        let assign9150_e8962: f64 = if (((locals.var_sa_i > 0.0) && (locals.var_sb_i > 0.0)) && ((locals.var_nf_i == 1.0) || ((locals.var_nf_i > 1.0) && (locals.var_sd_i > 0.0)))) { 1.0 } else { 0.0 };
        locals.var_guard144 = assign9150_e8962;
        locals.var_guard144_rv = 0.0;

        let mut assign9160_loop_guard: usize = 0;
        while {
            let assign9160_cond_e8969: f64 = (locals.var_nf_i - 0.5);
            let assign9160_cond_e8971: f64 = if (((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_loop_ < assign9160_cond_e8969)) { 1.0 } else { 0.0 };
            assign9160_cond_e8971 != 0.0
        } {
            assign9160_loop_guard += 1;
            assert!(assign9160_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign9160_body0_e8991,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9160_body0_e8980: f64 = (0.5 * locals.var_l_i);
        let assign9160_body0_e8981: f64 = (locals.var_sa_i + assign9160_body0_e8980);
        let assign9160_body0_e8985: f64 = (locals.var_sd_i + locals.var_l_i);
        let assign9160_body0_e8986: f64 = (locals.var_loop_ * assign9160_body0_e8985);
        let assign9160_body0_e8987: f64 = (assign9160_body0_e8981 + assign9160_body0_e8986);
        let assign9160_body0_e8988: f64 = (1.0 / assign9160_body0_e8987);
        let assign9160_body0_e8989: f64 = (locals.var_tmpa + assign9160_body0_e8988);
        (assign9160_body0_e8989,)
    } else {
        (locals.var_tmpa,)
    }
};
            locals.var_tmpa = assign9160_body0_e8991;
            locals.var_tmpa_rv = 0.0;
            let (assign9160_body1_e9011,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9160_body1_e9000: f64 = (0.5 * locals.var_l_i);
        let assign9160_body1_e9001: f64 = (locals.var_sb_i + assign9160_body1_e9000);
        let assign9160_body1_e9005: f64 = (locals.var_sd_i + locals.var_l_i);
        let assign9160_body1_e9006: f64 = (locals.var_loop_ * assign9160_body1_e9005);
        let assign9160_body1_e9007: f64 = (assign9160_body1_e9001 + assign9160_body1_e9006);
        let assign9160_body1_e9008: f64 = (1.0 / assign9160_body1_e9007);
        let assign9160_body1_e9009: f64 = (locals.var_tmpb + assign9160_body1_e9008);
        (assign9160_body1_e9009,)
    } else {
        (locals.var_tmpb,)
    }
};
            locals.var_tmpb = assign9160_body1_e9011;
            locals.var_tmpb_rv = 0.0;
            let (assign9160_body2_e9019,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9160_body2_e9017: f64 = (locals.var_loop_ + 1.0);
        (assign9160_body2_e9017,)
    } else {
        (locals.var_loop_,)
    }
};
            locals.var_loop_ = assign9160_body2_e9019;
            locals.var_loop__rv = 0.0;
        }

        let (assign9170_e9027,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9170_e9025: f64 = (locals.var_tmpa * locals.var_invnf);
        (assign9170_e9025,)
    } else {
        (locals.var_invsa,)
    }
};
        locals.var_invsa = assign9170_e9027;
        locals.var_invsa_rv = 0.0;

        let (assign9180_e9035,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9180_e9033: f64 = (locals.var_tmpb * locals.var_invnf);
        (assign9180_e9033,)
    } else {
        (locals.var_invsb,)
    }
};
        locals.var_invsb = assign9180_e9035;
        locals.var_invsb_rv = 0.0;

        let (assign9190_e9047,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9190_e9043: f64 = (0.5 * locals.var_l_i);
        let assign9190_e9044: f64 = (p.p784 + assign9190_e9043);
        let assign9190_e9045: f64 = (1.0 / assign9190_e9044);
        (assign9190_e9045,)
    } else {
        (locals.var_invsaref,)
    }
};
        locals.var_invsaref = assign9190_e9047;
        locals.var_invsaref_rv = 0.0;

        let (assign9200_e9059,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9200_e9055: f64 = (0.5 * locals.var_l_i);
        let assign9200_e9056: f64 = (p.p785 + assign9200_e9055);
        let assign9200_e9057: f64 = (1.0 / assign9200_e9056);
        (assign9200_e9057,)
    } else {
        (locals.var_invsbref,)
    }
};
        locals.var_invsbref = assign9200_e9059;
        locals.var_invsbref_rv = 0.0;

        let (assign9210_e9074,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9210_e9065: f64 = (locals.var_l_i + locals.var_dellps);
        let (assign9210_e9072,) = {
            if (assign9210_e9065 > 1e-9) {
                let assign9210_e9070: f64 = (locals.var_l_i + locals.var_dellps);
                (assign9210_e9070,)
            } else {
                (1e-9,)
            }
        };
        (assign9210_e9072,)
    } else {
        (locals.var_lx,)
    }
};
        locals.var_lx = assign9210_e9074;
        locals.var_lx_rv = 0.0;

        let (assign9220_e9093,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9220_e9080: f64 = (locals.var_w_i + locals.var_delwod);
        let assign9220_e9082: f64 = (assign9220_e9080 + p.p786);
        let (assign9220_e9091,) = {
            if (assign9220_e9082 > 1e-9) {
                let assign9220_e9087: f64 = (locals.var_w_i + locals.var_delwod);
                let assign9220_e9089: f64 = (assign9220_e9087 + p.p786);
                (assign9220_e9089,)
            } else {
                (1e-9,)
            }
        };
        (assign9220_e9091,)
    } else {
        (locals.var_wx,)
    }
};
        locals.var_wx = assign9220_e9093;
        locals.var_wx_rv = 0.0;

        let (assign9230_e9103,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9230_e9100: f64 = (locals.var_lx).powf(p.p794);
        let assign9230_e9101: f64 = (1.0 / assign9230_e9100);
        (assign9230_e9101,)
    } else {
        (locals.var_templ,)
    }
};
        locals.var_templ = assign9230_e9103;
        locals.var_templ_rv = 0.0;

        let (assign9240_e9113,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9240_e9110: f64 = (locals.var_wx).powf(p.p795);
        let assign9240_e9111: f64 = (1.0 / assign9240_e9110);
        (assign9240_e9111,)
    } else {
        (locals.var_tempw,)
    }
};
        locals.var_tempw = assign9240_e9113;
        locals.var_tempw_rv = 0.0;

        let (assign9250_e9141,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9250_e9120: f64 = (p.p791 * locals.var_templ);
        let assign9250_e9121: f64 = (1.0 + assign9250_e9120);
        let assign9250_e9124: f64 = (p.p792 * locals.var_tempw);
        let assign9250_e9125: f64 = (assign9250_e9121 + assign9250_e9124);
        let assign9250_e9128: f64 = (p.p793 * locals.var_templ);
        let assign9250_e9130: f64 = (assign9250_e9128 * locals.var_tempw);
        let assign9250_e9131: f64 = (assign9250_e9125 + assign9250_e9130);
        let assign9250_e9136: f64 = (locals.var_rta - 1.0);
        let assign9250_e9137: f64 = (p.p790 * assign9250_e9136);
        let assign9250_e9138: f64 = (1.0 + assign9250_e9137);
        let assign9250_e9139: f64 = (assign9250_e9131 * assign9250_e9138);
        (assign9250_e9139,)
    } else {
        (locals.var_kstressu0,)
    }
};
        locals.var_kstressu0 = assign9250_e9141;
        locals.var_kstressu0_rv = 0.0;

        let (assign9260_e9153,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9260_e9148: f64 = (locals.var_invsa + locals.var_invsb);
        let assign9260_e9149: f64 = (p.p787 * assign9260_e9148);
        let assign9260_e9151: f64 = (assign9260_e9149 / locals.var_kstressu0);
        (assign9260_e9151,)
    } else {
        (locals.var_rhobeta,)
    }
};
        locals.var_rhobeta = assign9260_e9153;
        locals.var_rhobeta_rv = 0.0;

        let (assign9270_e9165,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9270_e9160: f64 = (locals.var_invsaref + locals.var_invsbref);
        let assign9270_e9161: f64 = (p.p787 * assign9270_e9160);
        let assign9270_e9163: f64 = (assign9270_e9161 / locals.var_kstressu0);
        (assign9270_e9163,)
    } else {
        (locals.var_rhobetaref,)
    }
};
        locals.var_rhobetaref = assign9270_e9165;
        locals.var_rhobetaref_rv = 0.0;

        let (assign9280_e9175,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9280_e9172: f64 = (locals.var_lx).powf(p.p800);
        let assign9280_e9173: f64 = (1.0 / assign9280_e9172);
        (assign9280_e9173,)
    } else {
        (locals.var_templ,)
    }
};
        locals.var_templ = assign9280_e9175;
        locals.var_templ_rv = 0.0;

        let (assign9290_e9185,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9290_e9182: f64 = (locals.var_wx).powf(p.p801);
        let assign9290_e9183: f64 = (1.0 / assign9290_e9182);
        (assign9290_e9183,)
    } else {
        (locals.var_tempw,)
    }
};
        locals.var_tempw = assign9290_e9185;
        locals.var_tempw_rv = 0.0;

        let (assign9300_e9205,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9300_e9192: f64 = (p.p797 * locals.var_templ);
        let assign9300_e9193: f64 = (1.0 + assign9300_e9192);
        let assign9300_e9196: f64 = (p.p798 * locals.var_tempw);
        let assign9300_e9197: f64 = (assign9300_e9193 + assign9300_e9196);
        let assign9300_e9200: f64 = (p.p799 * locals.var_templ);
        let assign9300_e9202: f64 = (assign9300_e9200 * locals.var_tempw);
        let assign9300_e9203: f64 = (assign9300_e9197 + assign9300_e9202);
        (assign9300_e9203,)
    } else {
        (locals.var_kstressvth0,)
    }
};
        locals.var_kstressvth0 = assign9300_e9205;
        locals.var_kstressvth0_rv = 0.0;

        let (assign9310_e9217,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9310_e9211: f64 = (locals.var_invsa + locals.var_invsb);
        let assign9310_e9213: f64 = (assign9310_e9211 - locals.var_invsaref);
        let assign9310_e9215: f64 = (assign9310_e9213 - locals.var_invsbref);
        (assign9310_e9215,)
    } else {
        (locals.var_temp0,)
    }
};
        locals.var_temp0 = assign9310_e9217;
        locals.var_temp0_rv = 0.0;

        let (assign9320_e9229,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9320_e9223: f64 = (1.0 + locals.var_rhobeta);
        let assign9320_e9226: f64 = (1.0 + locals.var_rhobetaref);
        let assign9320_e9227: f64 = (assign9320_e9223 / assign9320_e9226);
        (assign9320_e9227,)
    } else {
        (locals.var_temp00,)
    }
};
        locals.var_temp00 = assign9320_e9229;
        locals.var_temp00_rv = 0.0;

        let (assign9330_e9237,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9330_e9235: f64 = (locals.var_betn_p * locals.var_temp00);
        (assign9330_e9235,)
    } else {
        (locals.var_betn_p,)
    }
};
        locals.var_betn_p = assign9330_e9237;
        locals.var_betn_p_rv = 0.0;

        let (assign9340_e9257,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9340_e9243: f64 = (locals.var_thesat_p * locals.var_temp00);
        let assign9340_e9247: f64 = (p.p788 * locals.var_rhobetaref);
        let assign9340_e9248: f64 = (1.0 + assign9340_e9247);
        let assign9340_e9249: f64 = (assign9340_e9243 * assign9340_e9248);
        let assign9340_e9253: f64 = (p.p788 * locals.var_rhobeta);
        let assign9340_e9254: f64 = (1.0 + assign9340_e9253);
        let assign9340_e9255: f64 = (assign9340_e9249 / assign9340_e9254);
        (assign9340_e9255,)
    } else {
        (locals.var_thesat_p,)
    }
};
        locals.var_thesat_p = assign9340_e9257;
        locals.var_thesat_p_rv = 0.0;

        let (assign9350_e9277,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9350_e9263: f64 = (locals.var_thesatac_p * locals.var_temp00);
        let assign9350_e9267: f64 = (locals.var_kvsatac_i * locals.var_rhobetaref);
        let assign9350_e9268: f64 = (1.0 + assign9350_e9267);
        let assign9350_e9269: f64 = (assign9350_e9263 * assign9350_e9268);
        let assign9350_e9273: f64 = (locals.var_kvsatac_i * locals.var_rhobeta);
        let assign9350_e9274: f64 = (1.0 + assign9350_e9273);
        let assign9350_e9275: f64 = (assign9350_e9269 / assign9350_e9274);
        (assign9350_e9275,)
    } else {
        (locals.var_thesatac_p,)
    }
};
        locals.var_thesatac_p = assign9350_e9277;
        locals.var_thesatac_p_rv = 0.0;

        let (assign9360_e9285,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9360_e9283: f64 = (locals.var_betnedge_p * locals.var_temp00);
        (assign9360_e9283,)
    } else {
        (locals.var_betnedge_p,)
    }
};
        locals.var_betnedge_p = assign9360_e9285;
        locals.var_betnedge_p_rv = 0.0;

        let (assign9370_e9295,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9370_e9291: f64 = (p.p796 * locals.var_temp0);
        let assign9370_e9293: f64 = (assign9370_e9291 / locals.var_kstressvth0);
        (assign9370_e9293,)
    } else {
        (locals.var_temp00,)
    }
};
        locals.var_temp00 = assign9370_e9295;
        locals.var_temp00_rv = 0.0;

        let (assign9380_e9303,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9380_e9301: f64 = (locals.var_vfb_p + locals.var_temp00);
        (assign9380_e9301,)
    } else {
        (locals.var_vfb_p,)
    }
};
        locals.var_vfb_p = assign9380_e9303;
        locals.var_vfb_p_rv = 0.0;

        let (assign9390_e9311,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9390_e9309: f64 = (locals.var_vfbedge_p + locals.var_temp00);
        (assign9390_e9309,)
    } else {
        (locals.var_vfbedge_p,)
    }
};
        locals.var_vfbedge_p = assign9390_e9311;
        locals.var_vfbedge_p_rv = 0.0;

        let (assign9400_e9323,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9400_e9317: f64 = (p.p802 * locals.var_temp0);
        let assign9400_e9320: f64 = (locals.var_kstressvth0).powf(p.p803);
        let assign9400_e9321: f64 = (assign9400_e9317 / assign9400_e9320);
        (assign9400_e9321,)
    } else {
        (locals.var_temp00,)
    }
};
        locals.var_temp00 = assign9400_e9323;
        locals.var_temp00_rv = 0.0;

        let (assign9410_e9331,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9410_e9329: f64 = (locals.var_cf_p + locals.var_temp00);
        (assign9410_e9329,)
    } else {
        (locals.var_cf_p,)
    }
};
        locals.var_cf_p = assign9410_e9331;
        locals.var_cf_p_rv = 0.0;

        let (assign9420_e9339,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9420_e9337: f64 = (locals.var_cfedge_p + locals.var_temp00);
        (assign9420_e9337,)
    } else {
        (locals.var_cfedge_p,)
    }
};
        locals.var_cfedge_p = assign9420_e9339;
        locals.var_cfedge_p_rv = 0.0;

        let assign9430_e9354: f64 = if ((((locals.var_sca_i > 0.0) || (locals.var_scb_i > 0.0)) || (locals.var_scc_i > 0.0)) || (locals.var_sc_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard145 = assign9430_e9354;
        locals.var_guard145_rv = 0.0;

        let assign9440_e9365: f64 = if (((locals.var_sca_i == 0.0) && (locals.var_scb_i == 0.0)) && (locals.var_scc_i == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard146 = assign9440_e9365;
        locals.var_guard146_rv = 0.0;

        let (assign9450_e9375,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard145 != 0.0)) && (locals.var_guard146 != 0.0)) {
        let assign9450_e9373: f64 = (locals.var_sc_i + locals.var_w_i);
        (assign9450_e9373,)
    } else {
        (locals.var_temp0,)
    }
};
        locals.var_temp0 = assign9450_e9375;
        locals.var_temp0_rv = 0.0;

        let (assign9460_e9385,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard145 != 0.0)) && (locals.var_guard146 != 0.0)) {
        let assign9460_e9383: f64 = (1.0 / p.p804);
        (assign9460_e9383,)
    } else {
        (locals.var_temp00,)
    }
};
        locals.var_temp00 = assign9460_e9385;
        locals.var_temp00_rv = 0.0;

        let (assign9470_e9399,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard145 != 0.0)) && (locals.var_guard146 != 0.0)) {
        let assign9470_e9393: f64 = (p.p804 * p.p804);
        let assign9470_e9396: f64 = (locals.var_sc_i * locals.var_temp0);
        let assign9470_e9397: f64 = (assign9470_e9393 / assign9470_e9396);
        (assign9470_e9397,)
    } else {
        (locals.var_sca_i,)
    }
};
        locals.var_sca_i = assign9470_e9399;
        locals.var_sca_i_rv = 0.0;

        let (assign9480_e9439,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard145 != 0.0)) && (locals.var_guard146 != 0.0)) {
        let assign9480_e9407: f64 = (0.1 * locals.var_sc_i);
        let assign9480_e9410: f64 = (0.01 * p.p804);
        let assign9480_e9411: f64 = (assign9480_e9407 + assign9480_e9410);
        let assign9480_e9413: f64 = (-10.0);
        let assign9480_e9415: f64 = (assign9480_e9413 * locals.var_sc_i);
        let assign9480_e9417: f64 = (assign9480_e9415 * locals.var_temp00);
        let assign9480_e9418: f64 = (assign9480_e9417).exp();
        let assign9480_e9419: f64 = (assign9480_e9411 * assign9480_e9418);
        let assign9480_e9422: f64 = (0.1 * locals.var_temp0);
        let assign9480_e9425: f64 = (0.01 * p.p804);
        let assign9480_e9426: f64 = (assign9480_e9422 + assign9480_e9425);
        let assign9480_e9428: f64 = (-10.0);
        let assign9480_e9430: f64 = (assign9480_e9428 * locals.var_temp0);
        let assign9480_e9432: f64 = (assign9480_e9430 * locals.var_temp00);
        let assign9480_e9433: f64 = (assign9480_e9432).exp();
        let assign9480_e9434: f64 = (assign9480_e9426 * assign9480_e9433);
        let assign9480_e9435: f64 = (assign9480_e9419 - assign9480_e9434);
        let assign9480_e9437: f64 = (assign9480_e9435 / locals.var_w_i);
        (assign9480_e9437,)
    } else {
        (locals.var_scb_i,)
    }
};
        locals.var_scb_i = assign9480_e9439;
        locals.var_scb_i_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_10(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign9490_e9479,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard145 != 0.0)) && (locals.var_guard146 != 0.0)) {
        let assign9490_e9447: f64 = (0.05 * locals.var_sc_i);
        let assign9490_e9450: f64 = (0.0025 * p.p804);
        let assign9490_e9451: f64 = (assign9490_e9447 + assign9490_e9450);
        let assign9490_e9453: f64 = (-20.0);
        let assign9490_e9455: f64 = (assign9490_e9453 * locals.var_sc_i);
        let assign9490_e9457: f64 = (assign9490_e9455 * locals.var_temp00);
        let assign9490_e9458: f64 = (assign9490_e9457).exp();
        let assign9490_e9459: f64 = (assign9490_e9451 * assign9490_e9458);
        let assign9490_e9462: f64 = (0.05 * locals.var_temp0);
        let assign9490_e9465: f64 = (0.0025 * p.p804);
        let assign9490_e9466: f64 = (assign9490_e9462 + assign9490_e9465);
        let assign9490_e9468: f64 = (-20.0);
        let assign9490_e9470: f64 = (assign9490_e9468 * locals.var_temp0);
        let assign9490_e9472: f64 = (assign9490_e9470 * locals.var_temp00);
        let assign9490_e9473: f64 = (assign9490_e9472).exp();
        let assign9490_e9474: f64 = (assign9490_e9466 * assign9490_e9473);
        let assign9490_e9475: f64 = (assign9490_e9459 - assign9490_e9474);
        let assign9490_e9477: f64 = (assign9490_e9475 / locals.var_w_i);
        (assign9490_e9477,)
    } else {
        (locals.var_scc_i,)
    }
};
        locals.var_scc_i = assign9490_e9479;
        locals.var_scc_i_rv = 0.0;

        let (assign9500_e9493,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard145 != 0.0)) {
        let assign9500_e9486: f64 = (p.p805 * locals.var_scb_i);
        let assign9500_e9487: f64 = (locals.var_sca_i + assign9500_e9486);
        let assign9500_e9490: f64 = (p.p806 * locals.var_scc_i);
        let assign9500_e9491: f64 = (assign9500_e9487 + assign9500_e9490);
        (assign9500_e9491,)
    } else {
        (locals.var_temp0,)
    }
};
        locals.var_temp0 = assign9500_e9493;
        locals.var_temp0_rv = 0.0;

        let (assign9510_e9503,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard145 != 0.0)) {
        let assign9510_e9500: f64 = (locals.var_kvthowe * locals.var_temp0);
        let assign9510_e9501: f64 = (locals.var_vfb_p + assign9510_e9500);
        (assign9510_e9501,)
    } else {
        (locals.var_vfb_p,)
    }
};
        locals.var_vfb_p = assign9510_e9503;
        locals.var_vfb_p_rv = 0.0;

        let (assign9520_e9515,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard145 != 0.0)) {
        let assign9520_e9511: f64 = (locals.var_kuowe * locals.var_temp0);
        let assign9520_e9512: f64 = (1.0 + assign9520_e9511);
        let assign9520_e9513: f64 = (locals.var_betn_p * assign9520_e9512);
        (assign9520_e9513,)
    } else {
        (locals.var_betn_p,)
    }
};
        locals.var_betn_p = assign9520_e9515;
        locals.var_betn_p_rv = 0.0;

        let (assign9530_e9525,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard145 != 0.0)) {
        let assign9530_e9522: f64 = (locals.var_kvthowe * locals.var_temp0);
        let assign9530_e9523: f64 = (locals.var_vfbedge_p + assign9530_e9522);
        (assign9530_e9523,)
    } else {
        (locals.var_vfbedge_p,)
    }
};
        locals.var_vfbedge_p = assign9530_e9525;
        locals.var_vfbedge_p_rv = 0.0;

        let (assign9540_e9537,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard145 != 0.0)) {
        let assign9540_e9533: f64 = (locals.var_kuowe * locals.var_temp0);
        let assign9540_e9534: f64 = (1.0 + assign9540_e9533);
        let assign9540_e9535: f64 = (locals.var_betnedge_p * assign9540_e9534);
        (assign9540_e9535,)
    } else {
        (locals.var_betnedge_p,)
    }
};
        locals.var_betnedge_p = assign9540_e9537;
        locals.var_betnedge_p_rv = 0.0;

        locals.var_vfb_i = locals.var_vfb_p;
        locals.var_vfb_i_rv = 0.0;

        locals.var_stvfb_i = locals.var_stvfb_p;
        locals.var_stvfb_i_rv = 0.0;

        locals.var_st2vfb_i = locals.var_st2vfb_p;
        locals.var_st2vfb_i_rv = 0.0;

        locals.var_tox_i = locals.var_tox_p;
        locals.var_tox_i_rv = 0.0;

        locals.var_epsrox_i = locals.var_epsrox_p;
        locals.var_epsrox_i_rv = 0.0;

        let (assign9600_e9553,) = {
    if (locals.var_neff_p > 1e20) {
        let (assign9600_e9551,) = {
            if (locals.var_neff_p < 1e26) {
                (locals.var_neff_p,)
            } else {
                (1e26,)
            }
        };
        (assign9600_e9551,)
    } else {
        (1e20,)
    }
};
        locals.var_neff_i = assign9600_e9553;
        locals.var_neff_i_rv = 0.0;

        let (assign9610_e9559,) = {
    if (locals.var_gfacnud_p > 0.01) {
        (locals.var_gfacnud_p,)
    } else {
        (0.01,)
    }
};
        locals.var_gfacnud_i = assign9610_e9559;
        locals.var_gfacnud_i_rv = 0.0;

        let (assign9620_e9565,) = {
    if (locals.var_vsbnud_p > 0.0) {
        (locals.var_vsbnud_p,)
    } else {
        (0.0,)
    }
};
        locals.var_vsbnud_i = assign9620_e9565;
        locals.var_vsbnud_i_rv = 0.0;

        locals.var_dvsbnud_i = locals.var_dvsbnud_p;
        locals.var_dvsbnud_i_rv = 0.0;

        locals.var_dphib_i = locals.var_dphib_p;
        locals.var_dphib_i_rv = 0.0;

        let (assign9650_e9573,) = {
    if (locals.var_np_p > 0.0) {
        (locals.var_np_p,)
    } else {
        (0.0,)
    }
};
        locals.var_np_i = assign9650_e9573;
        locals.var_np_i_rv = 0.0;

        locals.var_toxov_i = locals.var_toxov_p;
        locals.var_toxov_i_rv = 0.0;

        locals.var_toxovd_i = locals.var_toxovd_p;
        locals.var_toxovd_i_rv = 0.0;

        let (assign9680_e9586,) = {
    if (locals.var_nov_p > 1e23) {
        let (assign9680_e9584,) = {
            if (locals.var_nov_p < 1e27) {
                (locals.var_nov_p,)
            } else {
                (1e27,)
            }
        };
        (assign9680_e9584,)
    } else {
        (1e23,)
    }
};
        locals.var_nov_i = assign9680_e9586;
        locals.var_nov_i_rv = 0.0;

        let (assign9690_e9597,) = {
    if (locals.var_novd_p > 1e23) {
        let (assign9690_e9595,) = {
            if (locals.var_novd_p < 1e27) {
                (locals.var_novd_p,)
            } else {
                (1e27,)
            }
        };
        (assign9690_e9595,)
    } else {
        (1e23,)
    }
};
        locals.var_novd_i = assign9690_e9597;
        locals.var_novd_i_rv = 0.0;

        let (assign9700_e9603,) = {
    if (locals.var_ct_p > 0.0) {
        (locals.var_ct_p,)
    } else {
        (0.0,)
    }
};
        locals.var_ct_i = assign9700_e9603;
        locals.var_ct_i_rv = 0.0;

        let (assign9710_e9614,) = {
    if (locals.var_ctb_p > 0.0) {
        let (assign9710_e9612,) = {
            if (locals.var_ctb_p < 0.5) {
                (locals.var_ctb_p,)
            } else {
                (0.5,)
            }
        };
        (assign9710_e9612,)
    } else {
        (0.0,)
    }
};
        locals.var_ctb_i = assign9710_e9614;
        locals.var_ctb_i_rv = 0.0;

        let (assign9720_e9625,) = {
    if (locals.var_ctg_p > 0.0) {
        let (assign9720_e9623,) = {
            if (locals.var_ctg_p < 1.0) {
                (locals.var_ctg_p,)
            } else {
                (1.0,)
            }
        };
        (assign9720_e9623,)
    } else {
        (0.0,)
    }
};
        locals.var_ctg_i = assign9720_e9625;
        locals.var_ctg_i_rv = 0.0;

        locals.var_stct_i = locals.var_stct_p;
        locals.var_stct_i_rv = 0.0;

        let (assign9740_e9632,) = {
    if (locals.var_cf_p > 0.0) {
        (locals.var_cf_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cf_i = assign9740_e9632;
        locals.var_cf_i_rv = 0.0;

        let (assign9750_e9643,) = {
    if (locals.var_cfb_p > 0.0) {
        let (assign9750_e9641,) = {
            if (locals.var_cfb_p < 1.0) {
                (locals.var_cfb_p,)
            } else {
                (1.0,)
            }
        };
        (assign9750_e9641,)
    } else {
        (0.0,)
    }
};
        locals.var_cfb_i = assign9750_e9643;
        locals.var_cfb_i_rv = 0.0;

        let (assign9760_e9649,) = {
    if (locals.var_cfd_p > 0.0) {
        (locals.var_cfd_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cfd_i = assign9760_e9649;
        locals.var_cfd_i_rv = 0.0;

        let (assign9770_e9655,) = {
    if (locals.var_psce_p > 0.0) {
        (locals.var_psce_p,)
    } else {
        (0.0,)
    }
};
        locals.var_psce_i = assign9770_e9655;
        locals.var_psce_i_rv = 0.0;

        let (assign9780_e9666,) = {
    if (locals.var_psceb_p > 0.0) {
        let (assign9780_e9664,) = {
            if (locals.var_psceb_p < 1.0) {
                (locals.var_psceb_p,)
            } else {
                (1.0,)
            }
        };
        (assign9780_e9664,)
    } else {
        (0.0,)
    }
};
        locals.var_psceb_i = assign9780_e9666;
        locals.var_psceb_i_rv = 0.0;

        let (assign9790_e9672,) = {
    if (locals.var_psced_p > 0.0) {
        (locals.var_psced_p,)
    } else {
        (0.0,)
    }
};
        locals.var_psced_i = assign9790_e9672;
        locals.var_psced_i_rv = 0.0;

        let (assign9800_e9678,) = {
    if (locals.var_betn_p > 0.0) {
        (locals.var_betn_p,)
    } else {
        (0.0,)
    }
};
        locals.var_betn_i = assign9800_e9678;
        locals.var_betn_i_rv = 0.0;

        locals.var_stbet_i = locals.var_stbet_p;
        locals.var_stbet_i_rv = 0.0;

        let (assign9820_e9685,) = {
    if (locals.var_mue_p > 0.0) {
        (locals.var_mue_p,)
    } else {
        (0.0,)
    }
};
        locals.var_mue_i = assign9820_e9685;
        locals.var_mue_i_rv = 0.0;

        locals.var_stmue_i = locals.var_stmue_p;
        locals.var_stmue_i_rv = 0.0;

        let (assign9840_e9692,) = {
    if (locals.var_themu_p > 0.0) {
        (locals.var_themu_p,)
    } else {
        (0.0,)
    }
};
        locals.var_themu_i = assign9840_e9692;
        locals.var_themu_i_rv = 0.0;

        locals.var_stthemu_i = locals.var_stthemu_p;
        locals.var_stthemu_i_rv = 0.0;

        let (assign9860_e9699,) = {
    if (locals.var_cs_p > 0.0) {
        (locals.var_cs_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cs_i = assign9860_e9699;
        locals.var_cs_i_rv = 0.0;

        locals.var_stcs_i = locals.var_stcs_p;
        locals.var_stcs_i_rv = 0.0;

        let (assign9880_e9706,) = {
    if (locals.var_thecs_p > 0.0) {
        (locals.var_thecs_p,)
    } else {
        (0.0,)
    }
};
        locals.var_thecs_i = assign9880_e9706;
        locals.var_thecs_i_rv = 0.0;

        locals.var_stthecs_i = locals.var_stthecs_p;
        locals.var_stthecs_i_rv = 0.0;

        let (assign9900_e9713,) = {
    if (locals.var_xcor_p > 0.0) {
        (locals.var_xcor_p,)
    } else {
        (0.0,)
    }
};
        locals.var_xcor_i = assign9900_e9713;
        locals.var_xcor_i_rv = 0.0;

        locals.var_stxcor_i = locals.var_stxcor_p;
        locals.var_stxcor_i_rv = 0.0;

        locals.var_feta_i = locals.var_feta_p;
        locals.var_feta_i_rv = 0.0;

        let (assign9930_e9721,) = {
    if (locals.var_rs_p > 0.0) {
        (locals.var_rs_p,)
    } else {
        (0.0,)
    }
};
        locals.var_rs_i = assign9930_e9721;
        locals.var_rs_i_rv = 0.0;

        locals.var_strs_i = locals.var_strs_p;
        locals.var_strs_i_rv = 0.0;

        let assign9950_e9725: f64 = (-0.5);
        let (assign9950_e9735,) = {
    if (locals.var_rsb_p > assign9950_e9725) {
        let (assign9950_e9732,) = {
            if (locals.var_rsb_p < 1.0) {
                (locals.var_rsb_p,)
            } else {
                (1.0,)
            }
        };
        (assign9950_e9732,)
    } else {
        let assign9950_e9734: f64 = (-0.5);
        (assign9950_e9734,)
    }
};
        locals.var_rsb_i = assign9950_e9735;
        locals.var_rsb_i_rv = 0.0;

        let assign9960_e9738: f64 = (-0.5);
        let (assign9960_e9743,) = {
    if (locals.var_rsg_p > assign9960_e9738) {
        (locals.var_rsg_p,)
    } else {
        let assign9960_e9742: f64 = (-0.5);
        (assign9960_e9742,)
    }
};
        locals.var_rsg_i = assign9960_e9743;
        locals.var_rsg_i_rv = 0.0;

        let (assign9970_e9749,) = {
    if (locals.var_thesat_p > 0.0) {
        (locals.var_thesat_p,)
    } else {
        (0.0,)
    }
};
        locals.var_thesat_i = assign9970_e9749;
        locals.var_thesat_i_rv = 0.0;

        locals.var_stthesat_i = locals.var_stthesat_p;
        locals.var_stthesat_i_rv = 0.0;

        let assign9990_e9753: f64 = (-0.5);
        let (assign9990_e9763,) = {
    if (locals.var_thesatb_p > assign9990_e9753) {
        let (assign9990_e9760,) = {
            if (locals.var_thesatb_p < 1.0) {
                (locals.var_thesatb_p,)
            } else {
                (1.0,)
            }
        };
        (assign9990_e9760,)
    } else {
        let assign9990_e9762: f64 = (-0.5);
        (assign9990_e9762,)
    }
};
        locals.var_thesatb_i = assign9990_e9763;
        locals.var_thesatb_i_rv = 0.0;

        let assign10000_e9766: f64 = (-0.5);
        let (assign10000_e9771,) = {
    if (locals.var_thesatg_p > assign10000_e9766) {
        (locals.var_thesatg_p,)
    } else {
        let assign10000_e9770: f64 = (-0.5);
        (assign10000_e9770,)
    }
};
        locals.var_thesatg_i = assign10000_e9771;
        locals.var_thesatg_i_rv = 0.0;

        let (assign10010_e9777,) = {
    if (locals.var_thesatt_p > 0.01) {
        (locals.var_thesatt_p,)
    } else {
        (0.01,)
    }
};
        locals.var_thesatt_i = assign10010_e9777;
        locals.var_thesatt_i_rv = 0.0;

        let (assign10020_e9783,) = {
    if (locals.var_ax_p > 2.0) {
        (locals.var_ax_p,)
    } else {
        (2.0,)
    }
};
        locals.var_ax_i = assign10020_e9783;
        locals.var_ax_i_rv = 0.0;

    }
}
