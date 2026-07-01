#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_32(
        p: &Parameters,
        var_alphaav: f64,
        var_btatpartgat: f64,
        var_fbbtsti: f64,
        var_fstopsti: f64,
        var_ftdgat: f64,
        var_guard176: f64,
        var_guard193: f64,
        var_guard320: f64,
        var_idmult: f64,
        var_idsatgat: f64,
        var_lgsource_i: f64,
        var_one_minus_pgat: f64,
        var_one_over_one_minus_psti: f64,
        var_slopesti: f64,
        var_two_psistar: f64,
        var_v2: f64,
        var_vav: f64,
        var_vbbt: f64,
        var_vbigat: f64,
        var_vbirgatinv: f64,
        var_vbirstiinv: f64,
        var_vbrinvsti: f64,
        var_vjsrh: f64,
        var_wdepnulrgat: f64,
        var_wdepnulrinvsti: f64,
        var_zinv: f64,
        var_asrh_slot: &mut f64,
        var_asrh_dn5_slot: &mut f64,
        var_asrh_dn6_slot: &mut f64,
        var_asrh_dn7_slot: &mut f64,
        var_asrh_dn8_slot: &mut f64,
        var_btat_slot: &mut f64,
        var_btat_dn5_slot: &mut f64,
        var_btat_dn6_slot: &mut f64,
        var_btat_dn7_slot: &mut f64,
        var_btat_dn8_slot: &mut f64,
        var_dwsrh_slot: &mut f64,
        var_fbreakdown_slot: &mut f64,
        var_fbreakdown_dn5_slot: &mut f64,
        var_fbreakdown_dn6_slot: &mut f64,
        var_fbreakdown_dn7_slot: &mut f64,
        var_fbreakdown_dn8_slot: &mut f64,
        var_fmaxr_slot: &mut f64,
        var_fmaxr_dn5_slot: &mut f64,
        var_fmaxr_dn6_slot: &mut f64,
        var_fmaxr_dn7_slot: &mut f64,
        var_fmaxr_dn8_slot: &mut f64,
        var_guard330_slot: &mut f64,
        var_guard331_slot: &mut f64,
        var_guard332_slot: &mut f64,
        var_guard333_slot: &mut f64,
        var_guard334_slot: &mut f64,
        var_guard335_slot: &mut f64,
        var_guard336_slot: &mut f64,
        var_guard337_slot: &mut f64,
        var_guard338_slot: &mut f64,
        var_guard339_slot: &mut f64,
        var_guard340_slot: &mut f64,
        var_guard341_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn5_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
        var_id__blk213_slot: &mut f64,
        var_ijungat_slot: &mut f64,
        var_ijungat_dn5_slot: &mut f64,
        var_ijungat_dn6_slot: &mut f64,
        var_ijungat_dn7_slot: &mut f64,
        var_ijungat_dn8_slot: &mut f64,
        var_ijunsti_slot: &mut f64,
        var_ijunsti_dn5_slot: &mut f64,
        var_ijunsti_dn6_slot: &mut f64,
        var_ijunsti_dn7_slot: &mut f64,
        var_ijunsti_dn8_slot: &mut f64,
        var_isrh_slot: &mut f64,
        var_isrh_dn5_slot: &mut f64,
        var_isrh_dn6_slot: &mut f64,
        var_isrh_dn7_slot: &mut f64,
        var_isrh_dn8_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn5_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn5_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_vbi_minus_vjsrh_slot: &mut f64,
        var_wdep_slot: &mut f64,
        var_wdep_dn5_slot: &mut f64,
        var_wdep_dn6_slot: &mut f64,
        var_wdep_dn7_slot: &mut f64,
        var_wdep_dn8_slot: &mut f64,
        var_wsrh_slot: &mut f64,
        var_wsrhstep_slot: &mut f64,
    ) {
        let mut var_asrh: f64 = *var_asrh_slot;
        let mut var_asrh_dn5: f64 = *var_asrh_dn5_slot;
        let mut var_asrh_dn6: f64 = *var_asrh_dn6_slot;
        let mut var_asrh_dn7: f64 = *var_asrh_dn7_slot;
        let mut var_asrh_dn8: f64 = *var_asrh_dn8_slot;
        let mut var_btat: f64 = *var_btat_slot;
        let mut var_btat_dn5: f64 = *var_btat_dn5_slot;
        let mut var_btat_dn6: f64 = *var_btat_dn6_slot;
        let mut var_btat_dn7: f64 = *var_btat_dn7_slot;
        let mut var_btat_dn8: f64 = *var_btat_dn8_slot;
        let mut var_dwsrh: f64 = *var_dwsrh_slot;
        let mut var_fbreakdown: f64 = *var_fbreakdown_slot;
        let mut var_fbreakdown_dn5: f64 = *var_fbreakdown_dn5_slot;
        let mut var_fbreakdown_dn6: f64 = *var_fbreakdown_dn6_slot;
        let mut var_fbreakdown_dn7: f64 = *var_fbreakdown_dn7_slot;
        let mut var_fbreakdown_dn8: f64 = *var_fbreakdown_dn8_slot;
        let mut var_fmaxr: f64 = *var_fmaxr_slot;
        let mut var_fmaxr_dn5: f64 = *var_fmaxr_dn5_slot;
        let mut var_fmaxr_dn6: f64 = *var_fmaxr_dn6_slot;
        let mut var_fmaxr_dn7: f64 = *var_fmaxr_dn7_slot;
        let mut var_fmaxr_dn8: f64 = *var_fmaxr_dn8_slot;
        let mut var_guard330: f64 = *var_guard330_slot;
        let mut var_guard331: f64 = *var_guard331_slot;
        let mut var_guard332: f64 = *var_guard332_slot;
        let mut var_guard333: f64 = *var_guard333_slot;
        let mut var_guard334: f64 = *var_guard334_slot;
        let mut var_guard335: f64 = *var_guard335_slot;
        let mut var_guard336: f64 = *var_guard336_slot;
        let mut var_guard337: f64 = *var_guard337_slot;
        let mut var_guard338: f64 = *var_guard338_slot;
        let mut var_guard339: f64 = *var_guard339_slot;
        let mut var_guard340: f64 = *var_guard340_slot;
        let mut var_guard341: f64 = *var_guard341_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn5: f64 = *var_ibbt_dn5_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
        let mut var_id__blk213: f64 = *var_id__blk213_slot;
        let mut var_ijungat: f64 = *var_ijungat_slot;
        let mut var_ijungat_dn5: f64 = *var_ijungat_dn5_slot;
        let mut var_ijungat_dn6: f64 = *var_ijungat_dn6_slot;
        let mut var_ijungat_dn7: f64 = *var_ijungat_dn7_slot;
        let mut var_ijungat_dn8: f64 = *var_ijungat_dn8_slot;
        let mut var_ijunsti: f64 = *var_ijunsti_slot;
        let mut var_ijunsti_dn5: f64 = *var_ijunsti_dn5_slot;
        let mut var_ijunsti_dn6: f64 = *var_ijunsti_dn6_slot;
        let mut var_ijunsti_dn7: f64 = *var_ijunsti_dn7_slot;
        let mut var_ijunsti_dn8: f64 = *var_ijunsti_dn8_slot;
        let mut var_isrh: f64 = *var_isrh_slot;
        let mut var_isrh_dn5: f64 = *var_isrh_dn5_slot;
        let mut var_isrh_dn6: f64 = *var_isrh_dn6_slot;
        let mut var_isrh_dn7: f64 = *var_isrh_dn7_slot;
        let mut var_isrh_dn8: f64 = *var_isrh_dn8_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn5: f64 = *var_itat_dn5_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn5: f64 = *var_tmp_dn5_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_vbi_minus_vjsrh: f64 = *var_vbi_minus_vjsrh_slot;
        let mut var_wdep: f64 = *var_wdep_slot;
        let mut var_wdep_dn5: f64 = *var_wdep_dn5_slot;
        let mut var_wdep_dn6: f64 = *var_wdep_dn6_slot;
        let mut var_wdep_dn7: f64 = *var_wdep_dn7_slot;
        let mut var_wdep_dn8: f64 = *var_wdep_dn8_slot;
        let mut var_wsrh: f64 = *var_wsrh_slot;
        let mut var_wsrhstep: f64 = *var_wsrhstep_slot;

        let assign19190_e19061: f64 = if p.p845 == 0.0 { 1.0 } else { 0.0 };
        var_guard330 = assign19190_e19061;

        let (assign19200_e19072, assign19200_e19072_d_n5, assign19200_e19072_d_n6, assign19200_e19072_d_n7, assign19200_e19072_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard320 == 0.0)) && (var_guard330 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign19200_e19072;
        var_ibbt_dn5 = assign19200_e19072_d_n5;
        var_ibbt_dn6 = assign19200_e19072_d_n6;
        var_ibbt_dn7 = assign19200_e19072_d_n7;
        var_ibbt_dn8 = assign19200_e19072_d_n8;

        let assign19210_e19075: f64 = if p.p825 == 0.5 { 1.0 } else { 0.0 };
        var_guard331 = assign19210_e19075;

        let (assign19220_e19094, assign19220_e19094_d_n5, assign19220_e19094_d_n6, assign19220_e19094_d_n7, assign19220_e19094_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard320 == 0.0)) && (var_guard330 == 0.0)) && (var_guard331 != 0.0)) {
        let assign19220_e19089: f64 = (p.p822 - var_vbbt);
        let assign19220_e19091: f64 = (assign19220_e19089 * var_vbirstiinv);
        let assign19220_e19092: f64 = (assign19220_e19091).sqrt();
        (assign19220_e19092, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign19220_e19094;
        var_tmp_dn5 = assign19220_e19094_d_n5;
        var_tmp_dn6 = assign19220_e19094_d_n6;
        var_tmp_dn7 = assign19220_e19094_d_n7;
        var_tmp_dn8 = assign19220_e19094_d_n8;

        let (assign19230_e19115, assign19230_e19115_d_n5, assign19230_e19115_d_n6, assign19230_e19115_d_n7, assign19230_e19115_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard320 == 0.0)) && (var_guard330 == 0.0)) && (var_guard331 == 0.0)) {
        let assign19230_e19109: f64 = (p.p822 - var_vbbt);
        let assign19230_e19111: f64 = (assign19230_e19109 * var_vbirstiinv);
        let assign19230_e19113: f64 = (assign19230_e19111).powf(p.p825);
        (assign19230_e19113, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign19230_e19115;
        var_tmp_dn5 = assign19230_e19115_d_n5;
        var_tmp_dn6 = assign19230_e19115_d_n6;
        var_tmp_dn7 = assign19230_e19115_d_n7;
        var_tmp_dn8 = assign19230_e19115_d_n8;

        let (assign19240_e19135, assign19240_e19135_d_n5, assign19240_e19135_d_n6, assign19240_e19135_d_n7, assign19240_e19135_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard320 == 0.0)) && (var_guard330 == 0.0)) {
        let assign19240_e19128: f64 = (p.p822 - var_vbbt);
        let assign19240_e19130: f64 = (assign19240_e19128 * var_wdepnulrinvsti);
        let assign19240_e19132: f64 = (assign19240_e19130 / var_tmp);
        let assign19240_e19133: f64 = (var_one_over_one_minus_psti * assign19240_e19132);
        (assign19240_e19133, (var_one_over_one_minus_psti * (-((assign19240_e19130 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti * (-((assign19240_e19130 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti * (-((assign19240_e19130 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti * (-((assign19240_e19130 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign19240_e19135;
        var_fmaxr_dn5 = assign19240_e19135_d_n5;
        var_fmaxr_dn6 = assign19240_e19135_d_n6;
        var_fmaxr_dn7 = assign19240_e19135_d_n7;
        var_fmaxr_dn8 = assign19240_e19135_d_n8;

        let assign19250_e19137: f64 = (-var_fbbtsti);
        let assign19250_e19139: f64 = (assign19250_e19137 / var_fmaxr);
        let assign19250_e19140: f64 = (assign19250_e19139).abs();
        let assign19250_e19142: f64 = if assign19250_e19140 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard332 = assign19250_e19142;

        let (assign19260_e19160, assign19260_e19160_d_n5, assign19260_e19160_d_n6, assign19260_e19160_d_n7, assign19260_e19160_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard320 == 0.0)) && (var_guard330 == 0.0)) && (var_guard332 != 0.0)) {
        let assign19260_e19155: f64 = (-var_fbbtsti);
        let assign19260_e19157: f64 = (assign19260_e19155 / var_fmaxr);
        let assign19260_e19158: f64 = (assign19260_e19157).exp();
        (assign19260_e19158, (assign19260_e19158 * (-((assign19260_e19155 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))), (assign19260_e19158 * (-((assign19260_e19155 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign19260_e19158 * (-((assign19260_e19155 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign19260_e19158 * (-((assign19260_e19155 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign19260_e19160;
        var_tmp_dn5 = assign19260_e19160_d_n5;
        var_tmp_dn6 = assign19260_e19160_d_n6;
        var_tmp_dn7 = assign19260_e19160_d_n7;
        var_tmp_dn8 = assign19260_e19160_d_n8;

        let assign19270_e19162: f64 = (-var_fbbtsti);
        let assign19270_e19164: f64 = (assign19270_e19162 / var_fmaxr);
        let assign19270_e19166: f64 = if assign19270_e19164 < 0.0 { 1.0 } else { 0.0 };
        var_guard333 = assign19270_e19166;

        let (assign19280_e19217, assign19280_e19217_d_n5, assign19280_e19217_d_n6, assign19280_e19217_d_n7, assign19280_e19217_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard320 == 0.0)) && (var_guard330 == 0.0)) && (var_guard332 == 0.0)) && (var_guard333 != 0.0)) {
        let assign19280_e19184: f64 = (-230.25850929940458);
        let assign19280_e19186: f64 = (-var_fbbtsti);
        let assign19280_e19188: f64 = (assign19280_e19186 / var_fmaxr);
        let assign19280_e19189: f64 = (assign19280_e19184 - assign19280_e19188);
        let assign19280_e19193: f64 = (-230.25850929940458);
        let assign19280_e19195: f64 = (-var_fbbtsti);
        let assign19280_e19197: f64 = (assign19280_e19195 / var_fmaxr);
        let assign19280_e19198: f64 = (assign19280_e19193 - assign19280_e19197);
        let assign19280_e19201: f64 = (-230.25850929940458);
        let assign19280_e19203: f64 = (-var_fbbtsti);
        let assign19280_e19205: f64 = (assign19280_e19203 / var_fmaxr);
        let assign19280_e19206: f64 = (assign19280_e19201 - assign19280_e19205);
        let assign19280_e19208: f64 = (assign19280_e19206 * 0.3333333333333333);
        let assign19280_e19209: f64 = (1.0 + assign19280_e19208);
        let assign19280_e19210: f64 = (assign19280_e19198 * assign19280_e19209);
        let assign19280_e19211: f64 = (0.5 * assign19280_e19210);
        let assign19280_e19212: f64 = (1.0 + assign19280_e19211);
        let assign19280_e19213: f64 = (assign19280_e19189 * assign19280_e19212);
        let assign19280_e19214: f64 = (1.0 + assign19280_e19213);
        let assign19280_e19215: f64 = (1e-100 / assign19280_e19214);
        (assign19280_e19215, (-((1e-100 * (((-(-((assign19280_e19186 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign19280_e19212) + (assign19280_e19189 * (0.5 * (((-(-((assign19280_e19195 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign19280_e19209) + (assign19280_e19198 * ((-(-((assign19280_e19203 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign19280_e19214 * assign19280_e19214))), (-((1e-100 * (((-(-((assign19280_e19186 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign19280_e19212) + (assign19280_e19189 * (0.5 * (((-(-((assign19280_e19195 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign19280_e19209) + (assign19280_e19198 * ((-(-((assign19280_e19203 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign19280_e19214 * assign19280_e19214))), (-((1e-100 * (((-(-((assign19280_e19186 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign19280_e19212) + (assign19280_e19189 * (0.5 * (((-(-((assign19280_e19195 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign19280_e19209) + (assign19280_e19198 * ((-(-((assign19280_e19203 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign19280_e19214 * assign19280_e19214))), (-((1e-100 * (((-(-((assign19280_e19186 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign19280_e19212) + (assign19280_e19189 * (0.5 * (((-(-((assign19280_e19195 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign19280_e19209) + (assign19280_e19198 * ((-(-((assign19280_e19203 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign19280_e19214 * assign19280_e19214))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign19280_e19217;
        var_tmp_dn5 = assign19280_e19217_d_n5;
        var_tmp_dn6 = assign19280_e19217_d_n6;
        var_tmp_dn7 = assign19280_e19217_d_n7;
        var_tmp_dn8 = assign19280_e19217_d_n8;

        let (assign19290_e19266, assign19290_e19266_d_n5, assign19290_e19266_d_n6, assign19290_e19266_d_n7, assign19290_e19266_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard320 == 0.0)) && (var_guard330 == 0.0)) && (var_guard332 == 0.0)) && (var_guard333 == 0.0)) {
        let assign19290_e19236: f64 = (-var_fbbtsti);
        let assign19290_e19238: f64 = (assign19290_e19236 / var_fmaxr);
        let assign19290_e19240: f64 = (assign19290_e19238 - 230.25850929940458);
        let assign19290_e19244: f64 = (-var_fbbtsti);
        let assign19290_e19246: f64 = (assign19290_e19244 / var_fmaxr);
        let assign19290_e19248: f64 = (assign19290_e19246 - 230.25850929940458);
        let assign19290_e19251: f64 = (-var_fbbtsti);
        let assign19290_e19253: f64 = (assign19290_e19251 / var_fmaxr);
        let assign19290_e19255: f64 = (assign19290_e19253 - 230.25850929940458);
        let assign19290_e19257: f64 = (assign19290_e19255 * 0.3333333333333333);
        let assign19290_e19258: f64 = (1.0 + assign19290_e19257);
        let assign19290_e19259: f64 = (assign19290_e19248 * assign19290_e19258);
        let assign19290_e19260: f64 = (0.5 * assign19290_e19259);
        let assign19290_e19261: f64 = (1.0 + assign19290_e19260);
        let assign19290_e19262: f64 = (assign19290_e19240 * assign19290_e19261);
        let assign19290_e19263: f64 = (1.0 + assign19290_e19262);
        let assign19290_e19264: f64 = (1e100 * assign19290_e19263);
        (assign19290_e19264, (1e100 * (((-((assign19290_e19236 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign19290_e19261) + (assign19290_e19240 * (0.5 * (((-((assign19290_e19244 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign19290_e19258) + (assign19290_e19248 * ((-((assign19290_e19251 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign19290_e19236 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign19290_e19261) + (assign19290_e19240 * (0.5 * (((-((assign19290_e19244 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign19290_e19258) + (assign19290_e19248 * ((-((assign19290_e19251 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign19290_e19236 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign19290_e19261) + (assign19290_e19240 * (0.5 * (((-((assign19290_e19244 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign19290_e19258) + (assign19290_e19248 * ((-((assign19290_e19251 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign19290_e19236 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign19290_e19261) + (assign19290_e19240 * (0.5 * (((-((assign19290_e19244 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign19290_e19258) + (assign19290_e19248 * ((-((assign19290_e19251 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign19290_e19266;
        var_tmp_dn5 = assign19290_e19266_d_n5;
        var_tmp_dn6 = assign19290_e19266_d_n6;
        var_tmp_dn7 = assign19290_e19266_d_n7;
        var_tmp_dn8 = assign19290_e19266_d_n8;

        let (assign19300_e19286, assign19300_e19286_d_n5, assign19300_e19286_d_n6, assign19300_e19286_d_n7, assign19300_e19286_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard320 == 0.0)) && (var_guard330 == 0.0)) {
        let assign19300_e19279: f64 = (var_v2 * var_fmaxr);
        let assign19300_e19281: f64 = (assign19300_e19279 * var_fmaxr);
        let assign19300_e19283: f64 = (assign19300_e19281 * var_tmp);
        let assign19300_e19284: f64 = (p.p845 * assign19300_e19283);
        (assign19300_e19284, (p.p845 * (((((var_v2 * var_fmaxr_dn5) * var_fmaxr) + (assign19300_e19279 * var_fmaxr_dn5)) * var_tmp) + (assign19300_e19281 * var_tmp_dn5))), (p.p845 * (((((var_v2 * var_fmaxr_dn6) * var_fmaxr) + (assign19300_e19279 * var_fmaxr_dn6)) * var_tmp) + (assign19300_e19281 * var_tmp_dn6))), (p.p845 * (((((var_v2 * var_fmaxr_dn7) * var_fmaxr) + (assign19300_e19279 * var_fmaxr_dn7)) * var_tmp) + (assign19300_e19281 * var_tmp_dn7))), (p.p845 * (((((var_v2 * var_fmaxr_dn8) * var_fmaxr) + (assign19300_e19279 * var_fmaxr_dn8)) * var_tmp) + (assign19300_e19281 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign19300_e19286;
        var_ibbt_dn5 = assign19300_e19286_d_n5;
        var_ibbt_dn6 = assign19300_e19286_d_n6;
        var_ibbt_dn7 = assign19300_e19286_d_n7;
        var_ibbt_dn8 = assign19300_e19286_d_n8;

        let assign19310_e19289: f64 = if p.p854 > 1000.0 { 1.0 } else { 0.0 };
        var_guard334 = assign19310_e19289;

        let (assign19320_e19300, assign19320_e19300_d_n5, assign19320_e19300_d_n6, assign19320_e19300_d_n7, assign19320_e19300_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard320 == 0.0)) && (var_guard334 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign19320_e19300;
        var_fbreakdown_dn5 = assign19320_e19300_d_n5;
        var_fbreakdown_dn6 = assign19320_e19300_d_n6;
        var_fbreakdown_dn7 = assign19320_e19300_d_n7;
        var_fbreakdown_dn8 = assign19320_e19300_d_n8;

        let assign19330_e19303: f64 = (-var_alphaav);
        let assign19330_e19305: f64 = (assign19330_e19303 * p.p854);
        let assign19330_e19306: f64 = if var_vav > assign19330_e19305 { 1.0 } else { 0.0 };
        var_guard335 = assign19330_e19306;

        let assign19340_e19309: f64 = if p.p857 == 4.0 { 1.0 } else { 0.0 };
        var_guard336 = assign19340_e19309;

        let (assign19350_e19339, assign19350_e19339_d_n5, assign19350_e19339_d_n6, assign19350_e19339_d_n7, assign19350_e19339_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard320 == 0.0)) && (var_guard334 == 0.0)) && (var_guard335 != 0.0)) && (var_guard336 != 0.0)) {
        let assign19350_e19325: f64 = (var_vav * var_vbrinvsti);
        let assign19350_e19328: f64 = (var_vav * var_vbrinvsti);
        let assign19350_e19329: f64 = (assign19350_e19325 * assign19350_e19328);
        let assign19350_e19332: f64 = (var_vav * var_vbrinvsti);
        let assign19350_e19333: f64 = (assign19350_e19329 * assign19350_e19332);
        let assign19350_e19336: f64 = (var_vav * var_vbrinvsti);
        let assign19350_e19337: f64 = (assign19350_e19333 * assign19350_e19336);
        (assign19350_e19337, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign19350_e19339;
        var_tmp_dn5 = assign19350_e19339_d_n5;
        var_tmp_dn6 = assign19350_e19339_d_n6;
        var_tmp_dn7 = assign19350_e19339_d_n7;
        var_tmp_dn8 = assign19350_e19339_d_n8;

        let (assign19360_e19361, assign19360_e19361_d_n5, assign19360_e19361_d_n6, assign19360_e19361_d_n7, assign19360_e19361_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard320 == 0.0)) && (var_guard334 == 0.0)) && (var_guard335 != 0.0)) && (var_guard336 == 0.0)) {
        let assign19360_e19356: f64 = (var_vav * var_vbrinvsti);
        let assign19360_e19357: f64 = (assign19360_e19356).abs();
        let assign19360_e19359: f64 = (assign19360_e19357).powf(p.p857);
        (assign19360_e19359, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign19360_e19361;
        var_tmp_dn5 = assign19360_e19361_d_n5;
        var_tmp_dn6 = assign19360_e19361_d_n6;
        var_tmp_dn7 = assign19360_e19361_d_n7;
        var_tmp_dn8 = assign19360_e19361_d_n8;

        let (assign19370_e19379, assign19370_e19379_d_n5, assign19370_e19379_d_n6, assign19370_e19379_d_n7, assign19370_e19379_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard320 == 0.0)) && (var_guard334 == 0.0)) && (var_guard335 != 0.0)) {
        let assign19370_e19376: f64 = (1.0 - var_tmp);
        let assign19370_e19377: f64 = (1.0 / assign19370_e19376);
        (assign19370_e19377, (-((-var_tmp_dn5) / (assign19370_e19376 * assign19370_e19376))), (-((-var_tmp_dn6) / (assign19370_e19376 * assign19370_e19376))), (-((-var_tmp_dn7) / (assign19370_e19376 * assign19370_e19376))), (-((-var_tmp_dn8) / (assign19370_e19376 * assign19370_e19376))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign19370_e19379;
        var_fbreakdown_dn5 = assign19370_e19379_d_n5;
        var_fbreakdown_dn6 = assign19370_e19379_d_n6;
        var_fbreakdown_dn7 = assign19370_e19379_d_n7;
        var_fbreakdown_dn8 = assign19370_e19379_d_n8;

        let (assign19380_e19402, assign19380_e19402_d_n5, assign19380_e19402_d_n6, assign19380_e19402_d_n7, assign19380_e19402_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard320 == 0.0)) && (var_guard334 == 0.0)) && (var_guard335 == 0.0)) {
        let assign19380_e19396: f64 = (var_alphaav * p.p854);
        let assign19380_e19397: f64 = (var_vav + assign19380_e19396);
        let assign19380_e19399: f64 = (assign19380_e19397 * var_slopesti);
        let assign19380_e19400: f64 = (var_fstopsti + assign19380_e19399);
        (assign19380_e19400, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign19380_e19402;
        var_fbreakdown_dn5 = assign19380_e19402_d_n5;
        var_fbreakdown_dn6 = assign19380_e19402_d_n6;
        var_fbreakdown_dn7 = assign19380_e19402_d_n7;
        var_fbreakdown_dn8 = assign19380_e19402_d_n8;

        let (assign19390_e19421, assign19390_e19421_d_n5, assign19390_e19421_d_n6, assign19390_e19421_d_n7, assign19390_e19421_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard320 == 0.0)) {
        let assign19390_e19412: f64 = (var_id__blk213 + var_isrh);
        let assign19390_e19414: f64 = (assign19390_e19412 + var_itat);
        let assign19390_e19416: f64 = (assign19390_e19414 + var_ibbt);
        let assign19390_e19417: f64 = (p.p29 * assign19390_e19416);
        let assign19390_e19419: f64 = (assign19390_e19417 * var_fbreakdown);
        (assign19390_e19419, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign19390_e19417 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign19390_e19417 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign19390_e19417 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign19390_e19417 * var_fbreakdown_dn8)),)
    } else {
        (var_ijunsti, var_ijunsti_dn5, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8,)
    }
};
        var_ijunsti = assign19390_e19421;
        var_ijunsti_dn5 = assign19390_e19421_d_n5;
        var_ijunsti_dn6 = assign19390_e19421_d_n6;
        var_ijunsti_dn7 = assign19390_e19421_d_n7;
        var_ijunsti_dn8 = assign19390_e19421_d_n8;

        let assign19400_e19424: f64 = if var_lgsource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard337 = assign19400_e19424;

        let (assign19410_e19432, assign19410_e19432_d_n5, assign19410_e19432_d_n6, assign19410_e19432_d_n7, assign19410_e19432_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijungat, var_ijungat_dn5, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8,)
    }
};
        var_ijungat = assign19410_e19432;
        var_ijungat_dn5 = assign19410_e19432_d_n5;
        var_ijungat_dn6 = assign19410_e19432_d_n6;
        var_ijungat_dn7 = assign19410_e19432_d_n7;
        var_ijungat_dn8 = assign19410_e19432_d_n8;

        let (assign19420_e19443,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) {
        let assign19420_e19441: f64 = (var_idsatgat * var_idmult);
        (assign19420_e19441,)
    } else {
        (var_id__blk213,)
    }
};
        var_id__blk213 = assign19420_e19443;

        let assign19430_e19450: f64 = if ((p.p835 == 0.0) && (p.p840 == 0.0)) { 1.0 } else { 0.0 };
        var_guard338 = assign19430_e19450;

        let (assign19440_e19461, assign19440_e19461_d_n5, assign19440_e19461_d_n6, assign19440_e19461_d_n7, assign19440_e19461_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) && (var_guard338 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign19440_e19461;
        var_isrh_dn5 = assign19440_e19461_d_n5;
        var_isrh_dn6 = assign19440_e19461_d_n6;
        var_isrh_dn7 = assign19440_e19461_d_n7;
        var_isrh_dn8 = assign19440_e19461_d_n8;

        let (assign19450_e19475,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) && (var_guard338 == 0.0)) {
        let assign19450_e19473: f64 = (var_vbigat - var_vjsrh);
        (assign19450_e19473,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign19450_e19475;

        let (assign19460_e19494,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) && (var_guard338 == 0.0)) {
        let assign19460_e19489: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign19460_e19490: f64 = (1.0 - assign19460_e19489);
        let assign19460_e19491: f64 = (assign19460_e19490).sqrt();
        let assign19460_e19492: f64 = (1.0 - assign19460_e19491);
        (assign19460_e19492,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign19460_e19494;

        let assign19470_e19497: f64 = if p.p826 == 0.5 { 1.0 } else { 0.0 };
        var_guard339 = assign19470_e19497;

        let (assign19480_e19511,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) && (var_guard338 == 0.0)) && (var_guard339 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign19480_e19511;

        let (assign19490_e19543,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) && (var_guard338 == 0.0)) && (var_guard339 == 0.0)) {
        let assign19490_e19526: f64 = (var_wsrhstep * var_wsrhstep);
        let assign19490_e19528: f64 = (var_wsrhstep).ln();
        let assign19490_e19529: f64 = (assign19490_e19526 * assign19490_e19528);
        let assign19490_e19532: f64 = (1.0 - var_wsrhstep);
        let assign19490_e19533: f64 = (assign19490_e19529 / assign19490_e19532);
        let assign19490_e19535: f64 = (assign19490_e19533 + var_wsrhstep);
        let assign19490_e19539: f64 = (2.0 * p.p826);
        let assign19490_e19540: f64 = (1.0 - assign19490_e19539);
        let assign19490_e19541: f64 = (assign19490_e19535 * assign19490_e19540);
        (assign19490_e19541,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign19490_e19543;

        let (assign19500_e19557,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) && (var_guard338 == 0.0)) {
        let assign19500_e19555: f64 = (var_wsrhstep + var_dwsrh);
        (assign19500_e19555,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign19500_e19557;

        let assign19510_e19560: f64 = if p.p826 == 0.5 { 1.0 } else { 0.0 };
        var_guard340 = assign19510_e19560;

        let (assign19520_e19577, assign19520_e19577_d_n5, assign19520_e19577_d_n6, assign19520_e19577_d_n7, assign19520_e19577_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) && (var_guard338 == 0.0)) && (var_guard340 != 0.0)) {
        let assign19520_e19574: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv);
        let assign19520_e19575: f64 = (assign19520_e19574).sqrt();
        (assign19520_e19575, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign19520_e19577;
        var_tmp_dn5 = assign19520_e19577_d_n5;
        var_tmp_dn6 = assign19520_e19577_d_n6;
        var_tmp_dn7 = assign19520_e19577_d_n7;
        var_tmp_dn8 = assign19520_e19577_d_n8;

        let (assign19530_e19596, assign19530_e19596_d_n5, assign19530_e19596_d_n6, assign19530_e19596_d_n7, assign19530_e19596_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) && (var_guard338 == 0.0)) && (var_guard340 == 0.0)) {
        let assign19530_e19592: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv);
        let assign19530_e19594: f64 = (assign19530_e19592).powf(p.p826);
        (assign19530_e19594, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign19530_e19596;
        var_tmp_dn5 = assign19530_e19596_d_n5;
        var_tmp_dn6 = assign19530_e19596_d_n6;
        var_tmp_dn7 = assign19530_e19596_d_n7;
        var_tmp_dn8 = assign19530_e19596_d_n8;

        let (assign19540_e19610, assign19540_e19610_d_n5, assign19540_e19610_d_n6, assign19540_e19610_d_n7, assign19540_e19610_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) && (var_guard338 == 0.0)) {
        let assign19540_e19608: f64 = (var_wdepnulrgat * var_tmp);
        (assign19540_e19608, (var_wdepnulrgat * var_tmp_dn5), (var_wdepnulrgat * var_tmp_dn6), (var_wdepnulrgat * var_tmp_dn7), (var_wdepnulrgat * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign19540_e19610;
        var_wdep_dn5 = assign19540_e19610_d_n5;
        var_wdep_dn6 = assign19540_e19610_d_n6;
        var_wdep_dn7 = assign19540_e19610_d_n7;
        var_wdep_dn8 = assign19540_e19610_d_n8;

        let (assign19550_e19628, assign19550_e19628_d_n5, assign19550_e19628_d_n6, assign19550_e19628_d_n7, assign19550_e19628_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) && (var_guard338 == 0.0)) {
        let assign19550_e19623: f64 = (var_zinv - 1.0);
        let assign19550_e19625: f64 = (assign19550_e19623 * var_wdep);
        let assign19550_e19626: f64 = (var_ftdgat * assign19550_e19625);
        (assign19550_e19626, (var_ftdgat * (assign19550_e19623 * var_wdep_dn5)), (var_ftdgat * (assign19550_e19623 * var_wdep_dn6)), (var_ftdgat * (assign19550_e19623 * var_wdep_dn7)), (var_ftdgat * (assign19550_e19623 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign19550_e19628;
        var_asrh_dn5 = assign19550_e19628_d_n5;
        var_asrh_dn6 = assign19550_e19628_d_n6;
        var_asrh_dn7 = assign19550_e19628_d_n7;
        var_asrh_dn8 = assign19550_e19628_d_n8;

        let (assign19560_e19644, assign19560_e19644_d_n5, assign19560_e19644_d_n6, assign19560_e19644_d_n7, assign19560_e19644_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) && (var_guard338 == 0.0)) {
        let assign19560_e19641: f64 = (var_asrh * var_wsrh);
        let assign19560_e19642: f64 = (p.p835 * assign19560_e19641);
        (assign19560_e19642, (p.p835 * (var_asrh_dn5 * var_wsrh)), (p.p835 * (var_asrh_dn6 * var_wsrh)), (p.p835 * (var_asrh_dn7 * var_wsrh)), (p.p835 * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign19560_e19644;
        var_isrh_dn5 = assign19560_e19644_d_n5;
        var_isrh_dn6 = assign19560_e19644_d_n6;
        var_isrh_dn7 = assign19560_e19644_d_n7;
        var_isrh_dn8 = assign19560_e19644_d_n8;

        let assign19570_e19647: f64 = if p.p840 == 0.0 { 1.0 } else { 0.0 };
        var_guard341 = assign19570_e19647;

        let (assign19580_e19658, assign19580_e19658_d_n5, assign19580_e19658_d_n6, assign19580_e19658_d_n7, assign19580_e19658_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) && (var_guard341 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign19580_e19658;
        var_itat_dn5 = assign19580_e19658_d_n5;
        var_itat_dn6 = assign19580_e19658_d_n6;
        var_itat_dn7 = assign19580_e19658_d_n7;
        var_itat_dn8 = assign19580_e19658_d_n8;

        let (assign19590_e19676, assign19590_e19676_d_n5, assign19590_e19676_d_n6, assign19590_e19676_d_n7, assign19590_e19676_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) && (var_guard341 == 0.0)) {
        let assign19590_e19671: f64 = (var_wdep * var_one_minus_pgat);
        let assign19590_e19673: f64 = (assign19590_e19671 / var_vbi_minus_vjsrh);
        let assign19590_e19674: f64 = (var_btatpartgat * assign19590_e19673);
        (assign19590_e19674, (var_btatpartgat * ((var_wdep_dn5 * var_one_minus_pgat) / var_vbi_minus_vjsrh)), (var_btatpartgat * ((var_wdep_dn6 * var_one_minus_pgat) / var_vbi_minus_vjsrh)), (var_btatpartgat * ((var_wdep_dn7 * var_one_minus_pgat) / var_vbi_minus_vjsrh)), (var_btatpartgat * ((var_wdep_dn8 * var_one_minus_pgat) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign19590_e19676;
        var_btat_dn5 = assign19590_e19676_d_n5;
        var_btat_dn6 = assign19590_e19676_d_n6;
        var_btat_dn7 = assign19590_e19676_d_n7;
        var_btat_dn8 = assign19590_e19676_d_n8;

        *var_asrh_slot = var_asrh;
        *var_asrh_dn5_slot = var_asrh_dn5;
        *var_asrh_dn6_slot = var_asrh_dn6;
        *var_asrh_dn7_slot = var_asrh_dn7;
        *var_asrh_dn8_slot = var_asrh_dn8;
        *var_btat_slot = var_btat;
        *var_btat_dn5_slot = var_btat_dn5;
        *var_btat_dn6_slot = var_btat_dn6;
        *var_btat_dn7_slot = var_btat_dn7;
        *var_btat_dn8_slot = var_btat_dn8;
        *var_dwsrh_slot = var_dwsrh;
        *var_fbreakdown_slot = var_fbreakdown;
        *var_fbreakdown_dn5_slot = var_fbreakdown_dn5;
        *var_fbreakdown_dn6_slot = var_fbreakdown_dn6;
        *var_fbreakdown_dn7_slot = var_fbreakdown_dn7;
        *var_fbreakdown_dn8_slot = var_fbreakdown_dn8;
        *var_fmaxr_slot = var_fmaxr;
        *var_fmaxr_dn5_slot = var_fmaxr_dn5;
        *var_fmaxr_dn6_slot = var_fmaxr_dn6;
        *var_fmaxr_dn7_slot = var_fmaxr_dn7;
        *var_fmaxr_dn8_slot = var_fmaxr_dn8;
        *var_guard330_slot = var_guard330;
        *var_guard331_slot = var_guard331;
        *var_guard332_slot = var_guard332;
        *var_guard333_slot = var_guard333;
        *var_guard334_slot = var_guard334;
        *var_guard335_slot = var_guard335;
        *var_guard336_slot = var_guard336;
        *var_guard337_slot = var_guard337;
        *var_guard338_slot = var_guard338;
        *var_guard339_slot = var_guard339;
        *var_guard340_slot = var_guard340;
        *var_guard341_slot = var_guard341;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn5_slot = var_ibbt_dn5;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
        *var_id__blk213_slot = var_id__blk213;
        *var_ijungat_slot = var_ijungat;
        *var_ijungat_dn5_slot = var_ijungat_dn5;
        *var_ijungat_dn6_slot = var_ijungat_dn6;
        *var_ijungat_dn7_slot = var_ijungat_dn7;
        *var_ijungat_dn8_slot = var_ijungat_dn8;
        *var_ijunsti_slot = var_ijunsti;
        *var_ijunsti_dn5_slot = var_ijunsti_dn5;
        *var_ijunsti_dn6_slot = var_ijunsti_dn6;
        *var_ijunsti_dn7_slot = var_ijunsti_dn7;
        *var_ijunsti_dn8_slot = var_ijunsti_dn8;
        *var_isrh_slot = var_isrh;
        *var_isrh_dn5_slot = var_isrh_dn5;
        *var_isrh_dn6_slot = var_isrh_dn6;
        *var_isrh_dn7_slot = var_isrh_dn7;
        *var_isrh_dn8_slot = var_isrh_dn8;
        *var_itat_slot = var_itat;
        *var_itat_dn5_slot = var_itat_dn5;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn5_slot = var_tmp_dn5;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_vbi_minus_vjsrh_slot = var_vbi_minus_vjsrh;
        *var_wdep_slot = var_wdep;
        *var_wdep_dn5_slot = var_wdep_dn5;
        *var_wdep_dn6_slot = var_wdep_dn6;
        *var_wdep_dn7_slot = var_wdep_dn7;
        *var_wdep_dn8_slot = var_wdep_dn8;
        *var_wsrh_slot = var_wsrh;
        *var_wsrhstep_slot = var_wsrhstep;
    }

    pub(super) fn stamp_transient_block_33(
        p: &Parameters,
        var_asrh: f64,
        var_asrh_dn5: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_atatgat: f64,
        var_berfc: f64,
        var_btat: f64,
        var_btat_dn5: f64,
        var_btat_dn6: f64,
        var_btat_dn7: f64,
        var_btat_dn8: f64,
        var_cerfc: f64,
        var_fbbtgat: f64,
        var_guard176: f64,
        var_guard193: f64,
        var_guard337: f64,
        var_guard341: f64,
        var_one_over_one_minus_pgat: f64,
        var_perfc: f64,
        var_vbbt: f64,
        var_vbirgatinv: f64,
        var_wdepnulrinvgat: f64,
        var_wsrh: f64,
        var_erfcpos_slot: &mut f64,
        var_erfcpos_dn5_slot: &mut f64,
        var_erfcpos_dn6_slot: &mut f64,
        var_erfcpos_dn7_slot: &mut f64,
        var_erfcpos_dn8_slot: &mut f64,
        var_erfctimesexpmtat_slot: &mut f64,
        var_erfctimesexpmtat_dn5_slot: &mut f64,
        var_erfctimesexpmtat_dn6_slot: &mut f64,
        var_erfctimesexpmtat_dn7_slot: &mut f64,
        var_erfctimesexpmtat_dn8_slot: &mut f64,
        var_fmaxr_slot: &mut f64,
        var_fmaxr_dn5_slot: &mut f64,
        var_fmaxr_dn6_slot: &mut f64,
        var_fmaxr_dn7_slot: &mut f64,
        var_fmaxr_dn8_slot: &mut f64,
        var_gammamax_slot: &mut f64,
        var_gammamax_dn5_slot: &mut f64,
        var_gammamax_dn6_slot: &mut f64,
        var_gammamax_dn7_slot: &mut f64,
        var_gammamax_dn8_slot: &mut f64,
        var_guard342_slot: &mut f64,
        var_guard343_slot: &mut f64,
        var_guard344_slot: &mut f64,
        var_guard345_slot: &mut f64,
        var_guard346_slot: &mut f64,
        var_guard347_slot: &mut f64,
        var_guard348_slot: &mut f64,
        var_guard349_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn5_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn5_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_ktat_slot: &mut f64,
        var_ktat_dn5_slot: &mut f64,
        var_ktat_dn6_slot: &mut f64,
        var_ktat_dn7_slot: &mut f64,
        var_ktat_dn8_slot: &mut f64,
        var_ltat_slot: &mut f64,
        var_ltat_dn5_slot: &mut f64,
        var_ltat_dn6_slot: &mut f64,
        var_ltat_dn7_slot: &mut f64,
        var_ltat_dn8_slot: &mut f64,
        var_mtat_slot: &mut f64,
        var_mtat_dn5_slot: &mut f64,
        var_mtat_dn6_slot: &mut f64,
        var_mtat_dn7_slot: &mut f64,
        var_mtat_dn8_slot: &mut f64,
        var_sqrtumax_slot: &mut f64,
        var_sqrtumax_dn5_slot: &mut f64,
        var_sqrtumax_dn6_slot: &mut f64,
        var_sqrtumax_dn7_slot: &mut f64,
        var_sqrtumax_dn8_slot: &mut f64,
        var_terfc_slot: &mut f64,
        var_terfc_dn5_slot: &mut f64,
        var_terfc_dn6_slot: &mut f64,
        var_terfc_dn7_slot: &mut f64,
        var_terfc_dn8_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn5_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_twoatatoverthreebtat_slot: &mut f64,
        var_twoatatoverthreebtat_dn5_slot: &mut f64,
        var_twoatatoverthreebtat_dn6_slot: &mut f64,
        var_twoatatoverthreebtat_dn7_slot: &mut f64,
        var_twoatatoverthreebtat_dn8_slot: &mut f64,
        var_umax_slot: &mut f64,
        var_umax_dn5_slot: &mut f64,
        var_umax_dn6_slot: &mut f64,
        var_umax_dn7_slot: &mut f64,
        var_umax_dn8_slot: &mut f64,
        var_umaxbeforelimiting_slot: &mut f64,
        var_umaxbeforelimiting_dn5_slot: &mut f64,
        var_umaxbeforelimiting_dn6_slot: &mut f64,
        var_umaxbeforelimiting_dn7_slot: &mut f64,
        var_umaxbeforelimiting_dn8_slot: &mut f64,
        var_umaxpoweronepointfive_slot: &mut f64,
        var_umaxpoweronepointfive_dn5_slot: &mut f64,
        var_umaxpoweronepointfive_dn6_slot: &mut f64,
        var_umaxpoweronepointfive_dn7_slot: &mut f64,
        var_umaxpoweronepointfive_dn8_slot: &mut f64,
        var_wgamma_slot: &mut f64,
        var_wgamma_dn5_slot: &mut f64,
        var_wgamma_dn6_slot: &mut f64,
        var_wgamma_dn7_slot: &mut f64,
        var_wgamma_dn8_slot: &mut f64,
        var_wtat_slot: &mut f64,
        var_wtat_dn5_slot: &mut f64,
        var_wtat_dn6_slot: &mut f64,
        var_wtat_dn7_slot: &mut f64,
        var_wtat_dn8_slot: &mut f64,
        var_xerfc_slot: &mut f64,
        var_xerfc_dn5_slot: &mut f64,
        var_xerfc_dn6_slot: &mut f64,
        var_xerfc_dn7_slot: &mut f64,
        var_xerfc_dn8_slot: &mut f64,
        var_ysq_slot: &mut f64,
        var_ysq_dn5_slot: &mut f64,
        var_ysq_dn6_slot: &mut f64,
        var_ysq_dn7_slot: &mut f64,
        var_ysq_dn8_slot: &mut f64,
    ) {
        let mut var_erfcpos: f64 = *var_erfcpos_slot;
        let mut var_erfcpos_dn5: f64 = *var_erfcpos_dn5_slot;
        let mut var_erfcpos_dn6: f64 = *var_erfcpos_dn6_slot;
        let mut var_erfcpos_dn7: f64 = *var_erfcpos_dn7_slot;
        let mut var_erfcpos_dn8: f64 = *var_erfcpos_dn8_slot;
        let mut var_erfctimesexpmtat: f64 = *var_erfctimesexpmtat_slot;
        let mut var_erfctimesexpmtat_dn5: f64 = *var_erfctimesexpmtat_dn5_slot;
        let mut var_erfctimesexpmtat_dn6: f64 = *var_erfctimesexpmtat_dn6_slot;
        let mut var_erfctimesexpmtat_dn7: f64 = *var_erfctimesexpmtat_dn7_slot;
        let mut var_erfctimesexpmtat_dn8: f64 = *var_erfctimesexpmtat_dn8_slot;
        let mut var_fmaxr: f64 = *var_fmaxr_slot;
        let mut var_fmaxr_dn5: f64 = *var_fmaxr_dn5_slot;
        let mut var_fmaxr_dn6: f64 = *var_fmaxr_dn6_slot;
        let mut var_fmaxr_dn7: f64 = *var_fmaxr_dn7_slot;
        let mut var_fmaxr_dn8: f64 = *var_fmaxr_dn8_slot;
        let mut var_gammamax: f64 = *var_gammamax_slot;
        let mut var_gammamax_dn5: f64 = *var_gammamax_dn5_slot;
        let mut var_gammamax_dn6: f64 = *var_gammamax_dn6_slot;
        let mut var_gammamax_dn7: f64 = *var_gammamax_dn7_slot;
        let mut var_gammamax_dn8: f64 = *var_gammamax_dn8_slot;
        let mut var_guard342: f64 = *var_guard342_slot;
        let mut var_guard343: f64 = *var_guard343_slot;
        let mut var_guard344: f64 = *var_guard344_slot;
        let mut var_guard345: f64 = *var_guard345_slot;
        let mut var_guard346: f64 = *var_guard346_slot;
        let mut var_guard347: f64 = *var_guard347_slot;
        let mut var_guard348: f64 = *var_guard348_slot;
        let mut var_guard349: f64 = *var_guard349_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn5: f64 = *var_ibbt_dn5_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn5: f64 = *var_itat_dn5_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_ktat: f64 = *var_ktat_slot;
        let mut var_ktat_dn5: f64 = *var_ktat_dn5_slot;
        let mut var_ktat_dn6: f64 = *var_ktat_dn6_slot;
        let mut var_ktat_dn7: f64 = *var_ktat_dn7_slot;
        let mut var_ktat_dn8: f64 = *var_ktat_dn8_slot;
        let mut var_ltat: f64 = *var_ltat_slot;
        let mut var_ltat_dn5: f64 = *var_ltat_dn5_slot;
        let mut var_ltat_dn6: f64 = *var_ltat_dn6_slot;
        let mut var_ltat_dn7: f64 = *var_ltat_dn7_slot;
        let mut var_ltat_dn8: f64 = *var_ltat_dn8_slot;
        let mut var_mtat: f64 = *var_mtat_slot;
        let mut var_mtat_dn5: f64 = *var_mtat_dn5_slot;
        let mut var_mtat_dn6: f64 = *var_mtat_dn6_slot;
        let mut var_mtat_dn7: f64 = *var_mtat_dn7_slot;
        let mut var_mtat_dn8: f64 = *var_mtat_dn8_slot;
        let mut var_sqrtumax: f64 = *var_sqrtumax_slot;
        let mut var_sqrtumax_dn5: f64 = *var_sqrtumax_dn5_slot;
        let mut var_sqrtumax_dn6: f64 = *var_sqrtumax_dn6_slot;
        let mut var_sqrtumax_dn7: f64 = *var_sqrtumax_dn7_slot;
        let mut var_sqrtumax_dn8: f64 = *var_sqrtumax_dn8_slot;
        let mut var_terfc: f64 = *var_terfc_slot;
        let mut var_terfc_dn5: f64 = *var_terfc_dn5_slot;
        let mut var_terfc_dn6: f64 = *var_terfc_dn6_slot;
        let mut var_terfc_dn7: f64 = *var_terfc_dn7_slot;
        let mut var_terfc_dn8: f64 = *var_terfc_dn8_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn5: f64 = *var_tmp_dn5_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_twoatatoverthreebtat: f64 = *var_twoatatoverthreebtat_slot;
        let mut var_twoatatoverthreebtat_dn5: f64 = *var_twoatatoverthreebtat_dn5_slot;
        let mut var_twoatatoverthreebtat_dn6: f64 = *var_twoatatoverthreebtat_dn6_slot;
        let mut var_twoatatoverthreebtat_dn7: f64 = *var_twoatatoverthreebtat_dn7_slot;
        let mut var_twoatatoverthreebtat_dn8: f64 = *var_twoatatoverthreebtat_dn8_slot;
        let mut var_umax: f64 = *var_umax_slot;
        let mut var_umax_dn5: f64 = *var_umax_dn5_slot;
        let mut var_umax_dn6: f64 = *var_umax_dn6_slot;
        let mut var_umax_dn7: f64 = *var_umax_dn7_slot;
        let mut var_umax_dn8: f64 = *var_umax_dn8_slot;
        let mut var_umaxbeforelimiting: f64 = *var_umaxbeforelimiting_slot;
        let mut var_umaxbeforelimiting_dn5: f64 = *var_umaxbeforelimiting_dn5_slot;
        let mut var_umaxbeforelimiting_dn6: f64 = *var_umaxbeforelimiting_dn6_slot;
        let mut var_umaxbeforelimiting_dn7: f64 = *var_umaxbeforelimiting_dn7_slot;
        let mut var_umaxbeforelimiting_dn8: f64 = *var_umaxbeforelimiting_dn8_slot;
        let mut var_umaxpoweronepointfive: f64 = *var_umaxpoweronepointfive_slot;
        let mut var_umaxpoweronepointfive_dn5: f64 = *var_umaxpoweronepointfive_dn5_slot;
        let mut var_umaxpoweronepointfive_dn6: f64 = *var_umaxpoweronepointfive_dn6_slot;
        let mut var_umaxpoweronepointfive_dn7: f64 = *var_umaxpoweronepointfive_dn7_slot;
        let mut var_umaxpoweronepointfive_dn8: f64 = *var_umaxpoweronepointfive_dn8_slot;
        let mut var_wgamma: f64 = *var_wgamma_slot;
        let mut var_wgamma_dn5: f64 = *var_wgamma_dn5_slot;
        let mut var_wgamma_dn6: f64 = *var_wgamma_dn6_slot;
        let mut var_wgamma_dn7: f64 = *var_wgamma_dn7_slot;
        let mut var_wgamma_dn8: f64 = *var_wgamma_dn8_slot;
        let mut var_wtat: f64 = *var_wtat_slot;
        let mut var_wtat_dn5: f64 = *var_wtat_dn5_slot;
        let mut var_wtat_dn6: f64 = *var_wtat_dn6_slot;
        let mut var_wtat_dn7: f64 = *var_wtat_dn7_slot;
        let mut var_wtat_dn8: f64 = *var_wtat_dn8_slot;
        let mut var_xerfc: f64 = *var_xerfc_slot;
        let mut var_xerfc_dn5: f64 = *var_xerfc_dn5_slot;
        let mut var_xerfc_dn6: f64 = *var_xerfc_dn6_slot;
        let mut var_xerfc_dn7: f64 = *var_xerfc_dn7_slot;
        let mut var_xerfc_dn8: f64 = *var_xerfc_dn8_slot;
        let mut var_ysq: f64 = *var_ysq_slot;
        let mut var_ysq_dn5: f64 = *var_ysq_dn5_slot;
        let mut var_ysq_dn6: f64 = *var_ysq_dn6_slot;
        let mut var_ysq_dn7: f64 = *var_ysq_dn7_slot;
        let mut var_ysq_dn8: f64 = *var_ysq_dn8_slot;

        let (assign19600_e19692, assign19600_e19692_d_n5, assign19600_e19692_d_n6, assign19600_e19692_d_n7, assign19600_e19692_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) && (var_guard341 == 0.0)) {
        let assign19600_e19688: f64 = (0.666666666666667 * var_atatgat);
        let assign19600_e19690: f64 = (assign19600_e19688 / var_btat);
        (assign19600_e19690, (-((assign19600_e19688 * var_btat_dn5) / (var_btat * var_btat))), (-((assign19600_e19688 * var_btat_dn6) / (var_btat * var_btat))), (-((assign19600_e19688 * var_btat_dn7) / (var_btat * var_btat))), (-((assign19600_e19688 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign19600_e19692;
        var_twoatatoverthreebtat_dn5 = assign19600_e19692_d_n5;
        var_twoatatoverthreebtat_dn6 = assign19600_e19692_d_n6;
        var_twoatatoverthreebtat_dn7 = assign19600_e19692_d_n7;
        var_twoatatoverthreebtat_dn8 = assign19600_e19692_d_n8;

        let (assign19610_e19706, assign19610_e19706_d_n5, assign19610_e19706_d_n6, assign19610_e19706_d_n7, assign19610_e19706_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) && (var_guard341 == 0.0)) {
        let assign19610_e19704: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign19610_e19704, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign19610_e19706;
        var_umaxbeforelimiting_dn5 = assign19610_e19706_d_n5;
        var_umaxbeforelimiting_dn6 = assign19610_e19706_d_n6;
        var_umaxbeforelimiting_dn7 = assign19610_e19706_d_n7;
        var_umaxbeforelimiting_dn8 = assign19610_e19706_d_n8;

        let (assign19620_e19727, assign19620_e19727_d_n5, assign19620_e19727_d_n6, assign19620_e19727_d_n7, assign19620_e19727_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) && (var_guard341 == 0.0)) {
        let assign19620_e19718: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign19620_e19721: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign19620_e19723: f64 = (assign19620_e19721 + 1.0);
        let assign19620_e19724: f64 = (assign19620_e19718 / assign19620_e19723);
        let assign19620_e19725: f64 = (assign19620_e19724).sqrt();
        (assign19620_e19725, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign19620_e19723) - (assign19620_e19718 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign19620_e19723 * assign19620_e19723)) / (2.0 * assign19620_e19725)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign19620_e19723) - (assign19620_e19718 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign19620_e19723 * assign19620_e19723)) / (2.0 * assign19620_e19725)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign19620_e19723) - (assign19620_e19718 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign19620_e19723 * assign19620_e19723)) / (2.0 * assign19620_e19725)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign19620_e19723) - (assign19620_e19718 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign19620_e19723 * assign19620_e19723)) / (2.0 * assign19620_e19725)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign19620_e19727;
        var_umax_dn5 = assign19620_e19727_d_n5;
        var_umax_dn6 = assign19620_e19727_d_n6;
        var_umax_dn7 = assign19620_e19727_d_n7;
        var_umax_dn8 = assign19620_e19727_d_n8;

        let (assign19630_e19740, assign19630_e19740_d_n5, assign19630_e19740_d_n6, assign19630_e19740_d_n7, assign19630_e19740_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) && (var_guard341 == 0.0)) {
        let assign19630_e19738: f64 = (var_umax).sqrt();
        (assign19630_e19738, (var_umax_dn5 / (2.0 * assign19630_e19738)), (var_umax_dn6 / (2.0 * assign19630_e19738)), (var_umax_dn7 / (2.0 * assign19630_e19738)), (var_umax_dn8 / (2.0 * assign19630_e19738)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign19630_e19740;
        var_sqrtumax_dn5 = assign19630_e19740_d_n5;
        var_sqrtumax_dn6 = assign19630_e19740_d_n6;
        var_sqrtumax_dn7 = assign19630_e19740_d_n7;
        var_sqrtumax_dn8 = assign19630_e19740_d_n8;

        let (assign19640_e19754, assign19640_e19754_d_n5, assign19640_e19754_d_n6, assign19640_e19754_d_n7, assign19640_e19754_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) && (var_guard341 == 0.0)) {
        let assign19640_e19752: f64 = (var_umax * var_sqrtumax);
        (assign19640_e19752, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign19640_e19754;
        var_umaxpoweronepointfive_dn5 = assign19640_e19754_d_n5;
        var_umaxpoweronepointfive_dn6 = assign19640_e19754_d_n6;
        var_umaxpoweronepointfive_dn7 = assign19640_e19754_d_n7;
        var_umaxpoweronepointfive_dn8 = assign19640_e19754_d_n8;

        let assign19650_e19756: f64 = (-p.p826);
        let assign19650_e19758: f64 = (assign19650_e19756 * var_one_over_one_minus_pgat);
        let assign19650_e19760: f64 = (-1.0);
        let assign19650_e19761: f64 = if assign19650_e19758 == assign19650_e19760 { 1.0 } else { 0.0 };
        var_guard342 = assign19650_e19761;

        let (assign19660_e19781, assign19660_e19781_d_n5, assign19660_e19781_d_n6, assign19660_e19781_d_n7, assign19660_e19781_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) && (var_guard341 == 0.0)) && (var_guard342 != 0.0)) {
        let assign19660_e19777: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign19660_e19778: f64 = (1.0 + assign19660_e19777);
        let assign19660_e19779: f64 = (1.0 / assign19660_e19778);
        (assign19660_e19779, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign19660_e19778 * assign19660_e19778))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign19660_e19778 * assign19660_e19778))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign19660_e19778 * assign19660_e19778))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign19660_e19778 * assign19660_e19778))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign19660_e19781;
        var_wgamma_dn5 = assign19660_e19781_d_n5;
        var_wgamma_dn6 = assign19660_e19781_d_n6;
        var_wgamma_dn7 = assign19660_e19781_d_n7;
        var_wgamma_dn8 = assign19660_e19781_d_n8;

        let (assign19670_e19805, assign19670_e19805_d_n5, assign19670_e19805_d_n6, assign19670_e19805_d_n7, assign19670_e19805_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) && (var_guard341 == 0.0)) && (var_guard342 == 0.0)) {
        let assign19670_e19797: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign19670_e19798: f64 = (1.0 + assign19670_e19797);
        let assign19670_e19800: f64 = (-p.p826);
        let assign19670_e19802: f64 = (assign19670_e19800 * var_one_over_one_minus_pgat);
        let assign19670_e19803: f64 = (assign19670_e19798).powf(assign19670_e19802);
        (assign19670_e19803, if 0.0 == 0.0 && ((assign19670_e19802) as f64).is_finite() && ((assign19670_e19802) as f64).fract() == 0.0 { if assign19670_e19802 == 0.0 { 0.0 } else { (assign19670_e19802 * ((assign19670_e19798).powf(assign19670_e19802 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign19670_e19803 * (assign19670_e19802 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign19670_e19798))) }, if 0.0 == 0.0 && ((assign19670_e19802) as f64).is_finite() && ((assign19670_e19802) as f64).fract() == 0.0 { if assign19670_e19802 == 0.0 { 0.0 } else { (assign19670_e19802 * ((assign19670_e19798).powf(assign19670_e19802 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign19670_e19803 * (assign19670_e19802 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign19670_e19798))) }, if 0.0 == 0.0 && ((assign19670_e19802) as f64).is_finite() && ((assign19670_e19802) as f64).fract() == 0.0 { if assign19670_e19802 == 0.0 { 0.0 } else { (assign19670_e19802 * ((assign19670_e19798).powf(assign19670_e19802 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign19670_e19803 * (assign19670_e19802 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign19670_e19798))) }, if 0.0 == 0.0 && ((assign19670_e19802) as f64).is_finite() && ((assign19670_e19802) as f64).fract() == 0.0 { if assign19670_e19802 == 0.0 { 0.0 } else { (assign19670_e19802 * ((assign19670_e19798).powf(assign19670_e19802 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign19670_e19803 * (assign19670_e19802 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign19670_e19798))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign19670_e19805;
        var_wgamma_dn5 = assign19670_e19805_d_n5;
        var_wgamma_dn6 = assign19670_e19805_d_n6;
        var_wgamma_dn7 = assign19670_e19805_d_n7;
        var_wgamma_dn8 = assign19670_e19805_d_n8;

        let (assign19680_e19823, assign19680_e19823_d_n5, assign19680_e19823_d_n6, assign19680_e19823_d_n7, assign19680_e19823_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) && (var_guard341 == 0.0)) {
        let assign19680_e19817: f64 = (var_wsrh * var_wgamma);
        let assign19680_e19820: f64 = (var_wsrh + var_wgamma);
        let assign19680_e19821: f64 = (assign19680_e19817 / assign19680_e19820);
        (assign19680_e19821, ((((var_wsrh * var_wgamma_dn5) * assign19680_e19820) - (assign19680_e19817 * var_wgamma_dn5)) / (assign19680_e19820 * assign19680_e19820)), ((((var_wsrh * var_wgamma_dn6) * assign19680_e19820) - (assign19680_e19817 * var_wgamma_dn6)) / (assign19680_e19820 * assign19680_e19820)), ((((var_wsrh * var_wgamma_dn7) * assign19680_e19820) - (assign19680_e19817 * var_wgamma_dn7)) / (assign19680_e19820 * assign19680_e19820)), ((((var_wsrh * var_wgamma_dn8) * assign19680_e19820) - (assign19680_e19817 * var_wgamma_dn8)) / (assign19680_e19820 * assign19680_e19820)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign19680_e19823;
        var_wtat_dn5 = assign19680_e19823_d_n5;
        var_wtat_dn6 = assign19680_e19823_d_n6;
        var_wtat_dn7 = assign19680_e19823_d_n7;
        var_wtat_dn8 = assign19680_e19823_d_n8;

        let (assign19690_e19840, assign19690_e19840_d_n5, assign19690_e19840_d_n6, assign19690_e19840_d_n7, assign19690_e19840_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) && (var_guard341 == 0.0)) {
        let assign19690_e19836: f64 = (var_btat / var_sqrtumax);
        let assign19690_e19837: f64 = (0.375 * assign19690_e19836);
        let assign19690_e19838: f64 = (assign19690_e19837).sqrt();
        (assign19690_e19838, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign19690_e19838)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign19690_e19838)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign19690_e19838)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign19690_e19838)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign19690_e19840;
        var_ktat_dn5 = assign19690_e19840_d_n5;
        var_ktat_dn6 = assign19690_e19840_d_n6;
        var_ktat_dn7 = assign19690_e19840_d_n7;
        var_ktat_dn8 = assign19690_e19840_d_n8;

        let (assign19700_e19858, assign19700_e19858_d_n5, assign19700_e19858_d_n6, assign19700_e19858_d_n7, assign19700_e19858_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) && (var_guard341 == 0.0)) {
        let assign19700_e19853: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign19700_e19854: f64 = (2.0 * assign19700_e19853);
        let assign19700_e19856: f64 = (assign19700_e19854 - var_umax);
        (assign19700_e19856, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign19700_e19858;
        var_ltat_dn5 = assign19700_e19858_d_n5;
        var_ltat_dn6 = assign19700_e19858_d_n6;
        var_ltat_dn7 = assign19700_e19858_d_n7;
        var_ltat_dn8 = assign19700_e19858_d_n8;

        let (assign19710_e19884, assign19710_e19884_d_n5, assign19710_e19884_d_n6, assign19710_e19884_d_n7, assign19710_e19884_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) && (var_guard341 == 0.0)) {
        let assign19710_e19870: f64 = (var_atatgat * var_twoatatoverthreebtat);
        let assign19710_e19872: f64 = (assign19710_e19870 * var_sqrtumax);
        let assign19710_e19875: f64 = (var_atatgat * var_umax);
        let assign19710_e19876: f64 = (assign19710_e19872 - assign19710_e19875);
        let assign19710_e19880: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign19710_e19881: f64 = (0.5 * assign19710_e19880);
        let assign19710_e19882: f64 = (assign19710_e19876 + assign19710_e19881);
        (assign19710_e19882, (((((var_atatgat * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign19710_e19870 * var_sqrtumax_dn5)) - (var_atatgat * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatgat * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign19710_e19870 * var_sqrtumax_dn6)) - (var_atatgat * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatgat * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign19710_e19870 * var_sqrtumax_dn7)) - (var_atatgat * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatgat * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign19710_e19870 * var_sqrtumax_dn8)) - (var_atatgat * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign19710_e19884;
        var_mtat_dn5 = assign19710_e19884_d_n5;
        var_mtat_dn6 = assign19710_e19884_d_n6;
        var_mtat_dn7 = assign19710_e19884_d_n7;
        var_mtat_dn8 = assign19710_e19884_d_n8;

        let (assign19720_e19900, assign19720_e19900_d_n5, assign19720_e19900_d_n6, assign19720_e19900_d_n7, assign19720_e19900_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) && (var_guard341 == 0.0)) {
        let assign19720_e19896: f64 = (var_ltat - 1.0);
        let assign19720_e19898: f64 = (assign19720_e19896 * var_ktat);
        (assign19720_e19898, ((var_ltat_dn5 * var_ktat) + (assign19720_e19896 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign19720_e19896 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign19720_e19896 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign19720_e19896 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign19720_e19900;
        var_xerfc_dn5 = assign19720_e19900_d_n5;
        var_xerfc_dn6 = assign19720_e19900_d_n6;
        var_xerfc_dn7 = assign19720_e19900_d_n7;
        var_xerfc_dn8 = assign19720_e19900_d_n8;

        let (assign19730_e19914, assign19730_e19914_d_n5, assign19730_e19914_d_n6, assign19730_e19914_d_n7, assign19730_e19914_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) && (var_guard341 == 0.0)) {
        let assign19730_e19912: f64 = (var_xerfc * var_xerfc);
        (assign19730_e19912, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign19730_e19914;
        var_ysq_dn5 = assign19730_e19914_d_n5;
        var_ysq_dn6 = assign19730_e19914_d_n6;
        var_ysq_dn7 = assign19730_e19914_d_n7;
        var_ysq_dn8 = assign19730_e19914_d_n8;

        let assign19740_e19917: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard343 = assign19740_e19917;

        let (assign19750_e19937, assign19750_e19937_d_n5, assign19750_e19937_d_n6, assign19750_e19937_d_n7, assign19750_e19937_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) && (var_guard341 == 0.0)) && (var_guard343 != 0.0)) {
        let assign19750_e19933: f64 = (var_perfc * var_xerfc);
        let assign19750_e19934: f64 = (1.0 + assign19750_e19933);
        let assign19750_e19935: f64 = (1.0 / assign19750_e19934);
        (assign19750_e19935, (-((var_perfc * var_xerfc_dn5) / (assign19750_e19934 * assign19750_e19934))), (-((var_perfc * var_xerfc_dn6) / (assign19750_e19934 * assign19750_e19934))), (-((var_perfc * var_xerfc_dn7) / (assign19750_e19934 * assign19750_e19934))), (-((var_perfc * var_xerfc_dn8) / (assign19750_e19934 * assign19750_e19934))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign19750_e19937;
        var_terfc_dn5 = assign19750_e19937_d_n5;
        var_terfc_dn6 = assign19750_e19937_d_n6;
        var_terfc_dn7 = assign19750_e19937_d_n7;
        var_terfc_dn8 = assign19750_e19937_d_n8;

        let (assign19760_e19958, assign19760_e19958_d_n5, assign19760_e19958_d_n6, assign19760_e19958_d_n7, assign19760_e19958_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) && (var_guard341 == 0.0)) && (var_guard343 == 0.0)) {
        let assign19760_e19954: f64 = (var_perfc * var_xerfc);
        let assign19760_e19955: f64 = (1.0 - assign19760_e19954);
        let assign19760_e19956: f64 = (1.0 / assign19760_e19955);
        (assign19760_e19956, (-((-(var_perfc * var_xerfc_dn5)) / (assign19760_e19955 * assign19760_e19955))), (-((-(var_perfc * var_xerfc_dn6)) / (assign19760_e19955 * assign19760_e19955))), (-((-(var_perfc * var_xerfc_dn7)) / (assign19760_e19955 * assign19760_e19955))), (-((-(var_perfc * var_xerfc_dn8)) / (assign19760_e19955 * assign19760_e19955))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign19760_e19958;
        var_terfc_dn5 = assign19760_e19958_d_n5;
        var_terfc_dn6 = assign19760_e19958_d_n6;
        var_terfc_dn7 = assign19760_e19958_d_n7;
        var_terfc_dn8 = assign19760_e19958_d_n8;

        let assign19770_e19960: f64 = (-var_ysq);
        let assign19770_e19962: f64 = (assign19770_e19960 + var_mtat);
        let assign19770_e19964: f64 = (-230.25850929940458);
        let assign19770_e19965: f64 = if assign19770_e19962 > assign19770_e19964 { 1.0 } else { 0.0 };
        var_guard344 = assign19770_e19965;

        let (assign19780_e19983, assign19780_e19983_d_n5, assign19780_e19983_d_n6, assign19780_e19983_d_n7, assign19780_e19983_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) && (var_guard341 == 0.0)) && (var_guard344 != 0.0)) {
        let assign19780_e19978: f64 = (-var_ysq);
        let assign19780_e19980: f64 = (assign19780_e19978 + var_mtat);
        let assign19780_e19981: f64 = (assign19780_e19980).exp();
        (assign19780_e19981, (assign19780_e19981 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign19780_e19981 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign19780_e19981 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign19780_e19981 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign19780_e19983;
        var_tmp_dn5 = assign19780_e19983_d_n5;
        var_tmp_dn6 = assign19780_e19983_d_n6;
        var_tmp_dn7 = assign19780_e19983_d_n7;
        var_tmp_dn8 = assign19780_e19983_d_n8;

        let (assign19790_e20032, assign19790_e20032_d_n5, assign19790_e20032_d_n6, assign19790_e20032_d_n7, assign19790_e20032_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) && (var_guard341 == 0.0)) && (var_guard344 == 0.0)) {
        let assign19790_e19999: f64 = (-230.25850929940458);
        let assign19790_e20001: f64 = (-var_ysq);
        let assign19790_e20003: f64 = (assign19790_e20001 + var_mtat);
        let assign19790_e20004: f64 = (assign19790_e19999 - assign19790_e20003);
        let assign19790_e20008: f64 = (-230.25850929940458);
        let assign19790_e20010: f64 = (-var_ysq);
        let assign19790_e20012: f64 = (assign19790_e20010 + var_mtat);
        let assign19790_e20013: f64 = (assign19790_e20008 - assign19790_e20012);
        let assign19790_e20016: f64 = (-230.25850929940458);
        let assign19790_e20018: f64 = (-var_ysq);
        let assign19790_e20020: f64 = (assign19790_e20018 + var_mtat);
        let assign19790_e20021: f64 = (assign19790_e20016 - assign19790_e20020);
        let assign19790_e20023: f64 = (assign19790_e20021 * 0.3333333333333333);
        let assign19790_e20024: f64 = (1.0 + assign19790_e20023);
        let assign19790_e20025: f64 = (assign19790_e20013 * assign19790_e20024);
        let assign19790_e20026: f64 = (0.5 * assign19790_e20025);
        let assign19790_e20027: f64 = (1.0 + assign19790_e20026);
        let assign19790_e20028: f64 = (assign19790_e20004 * assign19790_e20027);
        let assign19790_e20029: f64 = (1.0 + assign19790_e20028);
        let assign19790_e20030: f64 = (1e-100 / assign19790_e20029);
        (assign19790_e20030, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign19790_e20027) + (assign19790_e20004 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign19790_e20024) + (assign19790_e20013 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign19790_e20029 * assign19790_e20029))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign19790_e20027) + (assign19790_e20004 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign19790_e20024) + (assign19790_e20013 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign19790_e20029 * assign19790_e20029))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign19790_e20027) + (assign19790_e20004 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign19790_e20024) + (assign19790_e20013 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign19790_e20029 * assign19790_e20029))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign19790_e20027) + (assign19790_e20004 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign19790_e20024) + (assign19790_e20013 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign19790_e20029 * assign19790_e20029))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign19790_e20032;
        var_tmp_dn5 = assign19790_e20032_d_n5;
        var_tmp_dn6 = assign19790_e20032_d_n6;
        var_tmp_dn7 = assign19790_e20032_d_n7;
        var_tmp_dn8 = assign19790_e20032_d_n8;

        let (assign19800_e20062, assign19800_e20062_d_n5, assign19800_e20062_d_n6, assign19800_e20062_d_n7, assign19800_e20062_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) && (var_guard341 == 0.0)) {
        let assign19800_e20044: f64 = (0.29214664 * var_terfc);
        let assign19800_e20048: f64 = (var_terfc * var_terfc);
        let assign19800_e20049: f64 = (var_berfc * assign19800_e20048);
        let assign19800_e20050: f64 = (assign19800_e20044 + assign19800_e20049);
        let assign19800_e20054: f64 = (var_terfc * var_terfc);
        let assign19800_e20056: f64 = (assign19800_e20054 * var_terfc);
        let assign19800_e20057: f64 = (var_cerfc * assign19800_e20056);
        let assign19800_e20058: f64 = (assign19800_e20050 + assign19800_e20057);
        let assign19800_e20060: f64 = (assign19800_e20058 * var_tmp);
        (assign19800_e20060, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign19800_e20054 * var_terfc_dn5)))) * var_tmp) + (assign19800_e20058 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign19800_e20054 * var_terfc_dn6)))) * var_tmp) + (assign19800_e20058 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign19800_e20054 * var_terfc_dn7)))) * var_tmp) + (assign19800_e20058 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign19800_e20054 * var_terfc_dn8)))) * var_tmp) + (assign19800_e20058 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign19800_e20062;
        var_erfcpos_dn5 = assign19800_e20062_d_n5;
        var_erfcpos_dn6 = assign19800_e20062_d_n6;
        var_erfcpos_dn7 = assign19800_e20062_d_n7;
        var_erfcpos_dn8 = assign19800_e20062_d_n8;

        let assign19810_e20065: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard345 = assign19810_e20065;

        let (assign19820_e20079, assign19820_e20079_d_n5, assign19820_e20079_d_n6, assign19820_e20079_d_n7, assign19820_e20079_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) && (var_guard341 == 0.0)) && (var_guard345 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign19820_e20079;
        var_erfctimesexpmtat_dn5 = assign19820_e20079_d_n5;
        var_erfctimesexpmtat_dn6 = assign19820_e20079_d_n6;
        var_erfctimesexpmtat_dn7 = assign19820_e20079_d_n7;
        var_erfctimesexpmtat_dn8 = assign19820_e20079_d_n8;

        let assign19830_e20082: f64 = (-230.25850929940458);
        let assign19830_e20083: f64 = if var_mtat > assign19830_e20082 { 1.0 } else { 0.0 };
        var_guard346 = assign19830_e20083;

        let (assign19840_e20101, assign19840_e20101_d_n5, assign19840_e20101_d_n6, assign19840_e20101_d_n7, assign19840_e20101_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) && (var_guard341 == 0.0)) && (var_guard345 == 0.0)) && (var_guard346 != 0.0)) {
        let assign19840_e20099: f64 = (var_mtat).exp();
        (assign19840_e20099, (assign19840_e20099 * var_mtat_dn5), (assign19840_e20099 * var_mtat_dn6), (assign19840_e20099 * var_mtat_dn7), (assign19840_e20099 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign19840_e20101;
        var_tmp_dn5 = assign19840_e20101_d_n5;
        var_tmp_dn6 = assign19840_e20101_d_n6;
        var_tmp_dn7 = assign19840_e20101_d_n7;
        var_tmp_dn8 = assign19840_e20101_d_n8;

        let (assign19850_e20144, assign19850_e20144_d_n5, assign19850_e20144_d_n6, assign19850_e20144_d_n7, assign19850_e20144_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) && (var_guard341 == 0.0)) && (var_guard345 == 0.0)) && (var_guard346 == 0.0)) {
        let assign19850_e20120: f64 = (-230.25850929940458);
        let assign19850_e20122: f64 = (assign19850_e20120 - var_mtat);
        let assign19850_e20126: f64 = (-230.25850929940458);
        let assign19850_e20128: f64 = (assign19850_e20126 - var_mtat);
        let assign19850_e20131: f64 = (-230.25850929940458);
        let assign19850_e20133: f64 = (assign19850_e20131 - var_mtat);
        let assign19850_e20135: f64 = (assign19850_e20133 * 0.3333333333333333);
        let assign19850_e20136: f64 = (1.0 + assign19850_e20135);
        let assign19850_e20137: f64 = (assign19850_e20128 * assign19850_e20136);
        let assign19850_e20138: f64 = (0.5 * assign19850_e20137);
        let assign19850_e20139: f64 = (1.0 + assign19850_e20138);
        let assign19850_e20140: f64 = (assign19850_e20122 * assign19850_e20139);
        let assign19850_e20141: f64 = (1.0 + assign19850_e20140);
        let assign19850_e20142: f64 = (1e-100 / assign19850_e20141);
        (assign19850_e20142, (-((1e-100 * (((-var_mtat_dn5) * assign19850_e20139) + (assign19850_e20122 * (0.5 * (((-var_mtat_dn5) * assign19850_e20136) + (assign19850_e20128 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign19850_e20141 * assign19850_e20141))), (-((1e-100 * (((-var_mtat_dn6) * assign19850_e20139) + (assign19850_e20122 * (0.5 * (((-var_mtat_dn6) * assign19850_e20136) + (assign19850_e20128 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign19850_e20141 * assign19850_e20141))), (-((1e-100 * (((-var_mtat_dn7) * assign19850_e20139) + (assign19850_e20122 * (0.5 * (((-var_mtat_dn7) * assign19850_e20136) + (assign19850_e20128 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign19850_e20141 * assign19850_e20141))), (-((1e-100 * (((-var_mtat_dn8) * assign19850_e20139) + (assign19850_e20122 * (0.5 * (((-var_mtat_dn8) * assign19850_e20136) + (assign19850_e20128 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign19850_e20141 * assign19850_e20141))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign19850_e20144;
        var_tmp_dn5 = assign19850_e20144_d_n5;
        var_tmp_dn6 = assign19850_e20144_d_n6;
        var_tmp_dn7 = assign19850_e20144_d_n7;
        var_tmp_dn8 = assign19850_e20144_d_n8;

        let (assign19860_e20163, assign19860_e20163_d_n5, assign19860_e20163_d_n6, assign19860_e20163_d_n7, assign19860_e20163_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) && (var_guard341 == 0.0)) && (var_guard345 == 0.0)) {
        let assign19860_e20159: f64 = (2.0 * var_tmp);
        let assign19860_e20161: f64 = (assign19860_e20159 - var_erfcpos);
        (assign19860_e20161, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign19860_e20163;
        var_erfctimesexpmtat_dn5 = assign19860_e20163_d_n5;
        var_erfctimesexpmtat_dn6 = assign19860_e20163_d_n6;
        var_erfctimesexpmtat_dn7 = assign19860_e20163_d_n7;
        var_erfctimesexpmtat_dn8 = assign19860_e20163_d_n8;

        let (assign19870_e20183, assign19870_e20183_d_n5, assign19870_e20183_d_n6, assign19870_e20183_d_n7, assign19870_e20183_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) && (var_guard341 == 0.0)) {
        let assign19870_e20175: f64 = (1.772453850905516 * 0.5);
        let assign19870_e20178: f64 = (var_atatgat * var_erfctimesexpmtat);
        let assign19870_e20180: f64 = (assign19870_e20178 / var_ktat);
        let assign19870_e20181: f64 = (assign19870_e20175 * assign19870_e20180);
        (assign19870_e20181, (assign19870_e20175 * ((((var_atatgat * var_erfctimesexpmtat_dn5) * var_ktat) - (assign19870_e20178 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign19870_e20175 * ((((var_atatgat * var_erfctimesexpmtat_dn6) * var_ktat) - (assign19870_e20178 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign19870_e20175 * ((((var_atatgat * var_erfctimesexpmtat_dn7) * var_ktat) - (assign19870_e20178 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign19870_e20175 * ((((var_atatgat * var_erfctimesexpmtat_dn8) * var_ktat) - (assign19870_e20178 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign19870_e20183;
        var_gammamax_dn5 = assign19870_e20183_d_n5;
        var_gammamax_dn6 = assign19870_e20183_d_n6;
        var_gammamax_dn7 = assign19870_e20183_d_n7;
        var_gammamax_dn8 = assign19870_e20183_d_n8;

        let (assign19880_e20201, assign19880_e20201_d_n5, assign19880_e20201_d_n6, assign19880_e20201_d_n7, assign19880_e20201_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) && (var_guard341 == 0.0)) {
        let assign19880_e20196: f64 = (var_asrh * var_gammamax);
        let assign19880_e20198: f64 = (assign19880_e20196 * var_wtat);
        let assign19880_e20199: f64 = (p.p840 * assign19880_e20198);
        (assign19880_e20199, (p.p840 * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign19880_e20196 * var_wtat_dn5))), (p.p840 * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign19880_e20196 * var_wtat_dn6))), (p.p840 * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign19880_e20196 * var_wtat_dn7))), (p.p840 * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign19880_e20196 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign19880_e20201;
        var_itat_dn5 = assign19880_e20201_d_n5;
        var_itat_dn6 = assign19880_e20201_d_n6;
        var_itat_dn7 = assign19880_e20201_d_n7;
        var_itat_dn8 = assign19880_e20201_d_n8;

        let assign19890_e20204: f64 = if p.p846 == 0.0 { 1.0 } else { 0.0 };
        var_guard347 = assign19890_e20204;

        let (assign19900_e20215, assign19900_e20215_d_n5, assign19900_e20215_d_n6, assign19900_e20215_d_n7, assign19900_e20215_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) && (var_guard347 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign19900_e20215;
        var_ibbt_dn5 = assign19900_e20215_d_n5;
        var_ibbt_dn6 = assign19900_e20215_d_n6;
        var_ibbt_dn7 = assign19900_e20215_d_n7;
        var_ibbt_dn8 = assign19900_e20215_d_n8;

        let assign19910_e20218: f64 = if p.p826 == 0.5 { 1.0 } else { 0.0 };
        var_guard348 = assign19910_e20218;

        let (assign19920_e20237, assign19920_e20237_d_n5, assign19920_e20237_d_n6, assign19920_e20237_d_n7, assign19920_e20237_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) && (var_guard347 == 0.0)) && (var_guard348 != 0.0)) {
        let assign19920_e20232: f64 = (p.p823 - var_vbbt);
        let assign19920_e20234: f64 = (assign19920_e20232 * var_vbirgatinv);
        let assign19920_e20235: f64 = (assign19920_e20234).sqrt();
        (assign19920_e20235, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign19920_e20237;
        var_tmp_dn5 = assign19920_e20237_d_n5;
        var_tmp_dn6 = assign19920_e20237_d_n6;
        var_tmp_dn7 = assign19920_e20237_d_n7;
        var_tmp_dn8 = assign19920_e20237_d_n8;

        let (assign19930_e20258, assign19930_e20258_d_n5, assign19930_e20258_d_n6, assign19930_e20258_d_n7, assign19930_e20258_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) && (var_guard347 == 0.0)) && (var_guard348 == 0.0)) {
        let assign19930_e20252: f64 = (p.p823 - var_vbbt);
        let assign19930_e20254: f64 = (assign19930_e20252 * var_vbirgatinv);
        let assign19930_e20256: f64 = (assign19930_e20254).powf(p.p826);
        (assign19930_e20256, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign19930_e20258;
        var_tmp_dn5 = assign19930_e20258_d_n5;
        var_tmp_dn6 = assign19930_e20258_d_n6;
        var_tmp_dn7 = assign19930_e20258_d_n7;
        var_tmp_dn8 = assign19930_e20258_d_n8;

        let (assign19940_e20278, assign19940_e20278_d_n5, assign19940_e20278_d_n6, assign19940_e20278_d_n7, assign19940_e20278_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) && (var_guard347 == 0.0)) {
        let assign19940_e20271: f64 = (p.p823 - var_vbbt);
        let assign19940_e20273: f64 = (assign19940_e20271 * var_wdepnulrinvgat);
        let assign19940_e20275: f64 = (assign19940_e20273 / var_tmp);
        let assign19940_e20276: f64 = (var_one_over_one_minus_pgat * assign19940_e20275);
        (assign19940_e20276, (var_one_over_one_minus_pgat * (-((assign19940_e20273 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat * (-((assign19940_e20273 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat * (-((assign19940_e20273 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat * (-((assign19940_e20273 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign19940_e20278;
        var_fmaxr_dn5 = assign19940_e20278_d_n5;
        var_fmaxr_dn6 = assign19940_e20278_d_n6;
        var_fmaxr_dn7 = assign19940_e20278_d_n7;
        var_fmaxr_dn8 = assign19940_e20278_d_n8;

        let assign19950_e20280: f64 = (-var_fbbtgat);
        let assign19950_e20282: f64 = (assign19950_e20280 / var_fmaxr);
        let assign19950_e20283: f64 = (assign19950_e20282).abs();
        let assign19950_e20285: f64 = if assign19950_e20283 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard349 = assign19950_e20285;

        *var_erfcpos_slot = var_erfcpos;
        *var_erfcpos_dn5_slot = var_erfcpos_dn5;
        *var_erfcpos_dn6_slot = var_erfcpos_dn6;
        *var_erfcpos_dn7_slot = var_erfcpos_dn7;
        *var_erfcpos_dn8_slot = var_erfcpos_dn8;
        *var_erfctimesexpmtat_slot = var_erfctimesexpmtat;
        *var_erfctimesexpmtat_dn5_slot = var_erfctimesexpmtat_dn5;
        *var_erfctimesexpmtat_dn6_slot = var_erfctimesexpmtat_dn6;
        *var_erfctimesexpmtat_dn7_slot = var_erfctimesexpmtat_dn7;
        *var_erfctimesexpmtat_dn8_slot = var_erfctimesexpmtat_dn8;
        *var_fmaxr_slot = var_fmaxr;
        *var_fmaxr_dn5_slot = var_fmaxr_dn5;
        *var_fmaxr_dn6_slot = var_fmaxr_dn6;
        *var_fmaxr_dn7_slot = var_fmaxr_dn7;
        *var_fmaxr_dn8_slot = var_fmaxr_dn8;
        *var_gammamax_slot = var_gammamax;
        *var_gammamax_dn5_slot = var_gammamax_dn5;
        *var_gammamax_dn6_slot = var_gammamax_dn6;
        *var_gammamax_dn7_slot = var_gammamax_dn7;
        *var_gammamax_dn8_slot = var_gammamax_dn8;
        *var_guard342_slot = var_guard342;
        *var_guard343_slot = var_guard343;
        *var_guard344_slot = var_guard344;
        *var_guard345_slot = var_guard345;
        *var_guard346_slot = var_guard346;
        *var_guard347_slot = var_guard347;
        *var_guard348_slot = var_guard348;
        *var_guard349_slot = var_guard349;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn5_slot = var_ibbt_dn5;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
        *var_itat_slot = var_itat;
        *var_itat_dn5_slot = var_itat_dn5;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_ktat_slot = var_ktat;
        *var_ktat_dn5_slot = var_ktat_dn5;
        *var_ktat_dn6_slot = var_ktat_dn6;
        *var_ktat_dn7_slot = var_ktat_dn7;
        *var_ktat_dn8_slot = var_ktat_dn8;
        *var_ltat_slot = var_ltat;
        *var_ltat_dn5_slot = var_ltat_dn5;
        *var_ltat_dn6_slot = var_ltat_dn6;
        *var_ltat_dn7_slot = var_ltat_dn7;
        *var_ltat_dn8_slot = var_ltat_dn8;
        *var_mtat_slot = var_mtat;
        *var_mtat_dn5_slot = var_mtat_dn5;
        *var_mtat_dn6_slot = var_mtat_dn6;
        *var_mtat_dn7_slot = var_mtat_dn7;
        *var_mtat_dn8_slot = var_mtat_dn8;
        *var_sqrtumax_slot = var_sqrtumax;
        *var_sqrtumax_dn5_slot = var_sqrtumax_dn5;
        *var_sqrtumax_dn6_slot = var_sqrtumax_dn6;
        *var_sqrtumax_dn7_slot = var_sqrtumax_dn7;
        *var_sqrtumax_dn8_slot = var_sqrtumax_dn8;
        *var_terfc_slot = var_terfc;
        *var_terfc_dn5_slot = var_terfc_dn5;
        *var_terfc_dn6_slot = var_terfc_dn6;
        *var_terfc_dn7_slot = var_terfc_dn7;
        *var_terfc_dn8_slot = var_terfc_dn8;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn5_slot = var_tmp_dn5;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_twoatatoverthreebtat_slot = var_twoatatoverthreebtat;
        *var_twoatatoverthreebtat_dn5_slot = var_twoatatoverthreebtat_dn5;
        *var_twoatatoverthreebtat_dn6_slot = var_twoatatoverthreebtat_dn6;
        *var_twoatatoverthreebtat_dn7_slot = var_twoatatoverthreebtat_dn7;
        *var_twoatatoverthreebtat_dn8_slot = var_twoatatoverthreebtat_dn8;
        *var_umax_slot = var_umax;
        *var_umax_dn5_slot = var_umax_dn5;
        *var_umax_dn6_slot = var_umax_dn6;
        *var_umax_dn7_slot = var_umax_dn7;
        *var_umax_dn8_slot = var_umax_dn8;
        *var_umaxbeforelimiting_slot = var_umaxbeforelimiting;
        *var_umaxbeforelimiting_dn5_slot = var_umaxbeforelimiting_dn5;
        *var_umaxbeforelimiting_dn6_slot = var_umaxbeforelimiting_dn6;
        *var_umaxbeforelimiting_dn7_slot = var_umaxbeforelimiting_dn7;
        *var_umaxbeforelimiting_dn8_slot = var_umaxbeforelimiting_dn8;
        *var_umaxpoweronepointfive_slot = var_umaxpoweronepointfive;
        *var_umaxpoweronepointfive_dn5_slot = var_umaxpoweronepointfive_dn5;
        *var_umaxpoweronepointfive_dn6_slot = var_umaxpoweronepointfive_dn6;
        *var_umaxpoweronepointfive_dn7_slot = var_umaxpoweronepointfive_dn7;
        *var_umaxpoweronepointfive_dn8_slot = var_umaxpoweronepointfive_dn8;
        *var_wgamma_slot = var_wgamma;
        *var_wgamma_dn5_slot = var_wgamma_dn5;
        *var_wgamma_dn6_slot = var_wgamma_dn6;
        *var_wgamma_dn7_slot = var_wgamma_dn7;
        *var_wgamma_dn8_slot = var_wgamma_dn8;
        *var_wtat_slot = var_wtat;
        *var_wtat_dn5_slot = var_wtat_dn5;
        *var_wtat_dn6_slot = var_wtat_dn6;
        *var_wtat_dn7_slot = var_wtat_dn7;
        *var_wtat_dn8_slot = var_wtat_dn8;
        *var_xerfc_slot = var_xerfc;
        *var_xerfc_dn5_slot = var_xerfc_dn5;
        *var_xerfc_dn6_slot = var_xerfc_dn6;
        *var_xerfc_dn7_slot = var_xerfc_dn7;
        *var_xerfc_dn8_slot = var_xerfc_dn8;
        *var_ysq_slot = var_ysq;
        *var_ysq_dn5_slot = var_ysq_dn5;
        *var_ysq_dn6_slot = var_ysq_dn6;
        *var_ysq_dn7_slot = var_ysq_dn7;
        *var_ysq_dn8_slot = var_ysq_dn8;
    }

    pub(super) fn stamp_transient_block_34(
        p: &Parameters,
        var_absource_i: f64,
        var_alphaav: f64,
        var_exp_vmax_over_phitd_s: f64,
        var_fbbtgat: f64,
        var_fbbtgat_dn5: f64,
        var_fbbtgat_dn6: f64,
        var_fbbtgat_dn7: f64,
        var_fbbtgat_dn8: f64,
        var_fmaxr: f64,
        var_fmaxr_dn5: f64,
        var_fmaxr_dn6: f64,
        var_fmaxr_dn7: f64,
        var_fmaxr_dn8: f64,
        var_fstopgat: f64,
        var_guard176: f64,
        var_guard193: f64,
        var_guard337: f64,
        var_guard347: f64,
        var_guard349: f64,
        var_id__blk213: f64,
        var_ijunbot: f64,
        var_ijunbot_dn5: f64,
        var_ijunbot_dn6: f64,
        var_ijunbot_dn7: f64,
        var_ijunbot_dn8: f64,
        var_ijunsti: f64,
        var_ijunsti_dn5: f64,
        var_ijunsti_dn6: f64,
        var_ijunsti_dn7: f64,
        var_ijunsti_dn8: f64,
        var_isrh: f64,
        var_isrh_dn5: f64,
        var_isrh_dn6: f64,
        var_isrh_dn7: f64,
        var_isrh_dn8: f64,
        var_itat: f64,
        var_itat_dn5: f64,
        var_itat_dn6: f64,
        var_itat_dn7: f64,
        var_itat_dn8: f64,
        var_lgsource_i: f64,
        var_lssource_i: f64,
        var_phitd: f64,
        var_phitdinv: f64,
        var_phitr: f64,
        var_slopegat: f64,
        var_slopegat_dn5: f64,
        var_slopegat_dn6: f64,
        var_slopegat_dn7: f64,
        var_slopegat_dn8: f64,
        var_v2: f64,
        var_v3: f64,
        var_vav: f64,
        var_vbbtlim_s: f64,
        var_vbimin_s: f64,
        var_vbrinvgat: f64,
        var_vbrinvgat_dn5: f64,
        var_vbrinvgat_dn6: f64,
        var_vbrinvgat_dn7: f64,
        var_vbrinvgat_dn8: f64,
        var_vmax_s: f64,
        var_fbreakdown_slot: &mut f64,
        var_fbreakdown_dn5_slot: &mut f64,
        var_fbreakdown_dn6_slot: &mut f64,
        var_fbreakdown_dn7_slot: &mut f64,
        var_fbreakdown_dn8_slot: &mut f64,
        var_guard350_slot: &mut f64,
        var_guard351_slot: &mut f64,
        var_guard352_slot: &mut f64,
        var_guard353_slot: &mut f64,
        var_guard354_slot: &mut f64,
        var_guard355_slot: &mut f64,
        var_guard356_slot: &mut f64,
        var_guard357_slot: &mut f64,
        var_guard358_slot: &mut f64,
        var_i2_slot: &mut f64,
        var_i2_dn5_slot: &mut f64,
        var_i2_dn6_slot: &mut f64,
        var_i2_dn7_slot: &mut f64,
        var_i2_dn8_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn5_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
        var_idmult_slot: &mut f64,
        var_ijungat_slot: &mut f64,
        var_ijungat_dn5_slot: &mut f64,
        var_ijungat_dn6_slot: &mut f64,
        var_ijungat_dn7_slot: &mut f64,
        var_ijungat_dn8_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn5_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_two_psistar_slot: &mut f64,
        var_vbbt_slot: &mut f64,
        var_vjlim_slot: &mut f64,
        var_vjsrh_slot: &mut f64,
        var_z_slot: &mut f64,
        var_zinv_slot: &mut f64,
    ) {
        let mut var_fbreakdown: f64 = *var_fbreakdown_slot;
        let mut var_fbreakdown_dn5: f64 = *var_fbreakdown_dn5_slot;
        let mut var_fbreakdown_dn6: f64 = *var_fbreakdown_dn6_slot;
        let mut var_fbreakdown_dn7: f64 = *var_fbreakdown_dn7_slot;
        let mut var_fbreakdown_dn8: f64 = *var_fbreakdown_dn8_slot;
        let mut var_guard350: f64 = *var_guard350_slot;
        let mut var_guard351: f64 = *var_guard351_slot;
        let mut var_guard352: f64 = *var_guard352_slot;
        let mut var_guard353: f64 = *var_guard353_slot;
        let mut var_guard354: f64 = *var_guard354_slot;
        let mut var_guard355: f64 = *var_guard355_slot;
        let mut var_guard356: f64 = *var_guard356_slot;
        let mut var_guard357: f64 = *var_guard357_slot;
        let mut var_guard358: f64 = *var_guard358_slot;
        let mut var_i2: f64 = *var_i2_slot;
        let mut var_i2_dn5: f64 = *var_i2_dn5_slot;
        let mut var_i2_dn6: f64 = *var_i2_dn6_slot;
        let mut var_i2_dn7: f64 = *var_i2_dn7_slot;
        let mut var_i2_dn8: f64 = *var_i2_dn8_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn5: f64 = *var_ibbt_dn5_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
        let mut var_idmult: f64 = *var_idmult_slot;
        let mut var_ijungat: f64 = *var_ijungat_slot;
        let mut var_ijungat_dn5: f64 = *var_ijungat_dn5_slot;
        let mut var_ijungat_dn6: f64 = *var_ijungat_dn6_slot;
        let mut var_ijungat_dn7: f64 = *var_ijungat_dn7_slot;
        let mut var_ijungat_dn8: f64 = *var_ijungat_dn8_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn5: f64 = *var_tmp_dn5_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_two_psistar: f64 = *var_two_psistar_slot;
        let mut var_vbbt: f64 = *var_vbbt_slot;
        let mut var_vjlim: f64 = *var_vjlim_slot;
        let mut var_vjsrh: f64 = *var_vjsrh_slot;
        let mut var_z: f64 = *var_z_slot;
        let mut var_zinv: f64 = *var_zinv_slot;

        let (assign19960_e20303, assign19960_e20303_d_n5, assign19960_e20303_d_n6, assign19960_e20303_d_n7, assign19960_e20303_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) && (var_guard347 == 0.0)) && (var_guard349 != 0.0)) {
        let assign19960_e20298: f64 = (-var_fbbtgat);
        let assign19960_e20300: f64 = (assign19960_e20298 / var_fmaxr);
        let assign19960_e20301: f64 = (assign19960_e20300).exp();
        (assign19960_e20301, (assign19960_e20301 * ((((-var_fbbtgat_dn5) * var_fmaxr) - (assign19960_e20298 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))), (assign19960_e20301 * ((((-var_fbbtgat_dn6) * var_fmaxr) - (assign19960_e20298 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))), (assign19960_e20301 * ((((-var_fbbtgat_dn7) * var_fmaxr) - (assign19960_e20298 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))), (assign19960_e20301 * ((((-var_fbbtgat_dn8) * var_fmaxr) - (assign19960_e20298 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign19960_e20303;
        var_tmp_dn5 = assign19960_e20303_d_n5;
        var_tmp_dn6 = assign19960_e20303_d_n6;
        var_tmp_dn7 = assign19960_e20303_d_n7;
        var_tmp_dn8 = assign19960_e20303_d_n8;

        let assign19970_e20305: f64 = (-var_fbbtgat);
        let assign19970_e20307: f64 = (assign19970_e20305 / var_fmaxr);
        let assign19970_e20309: f64 = if assign19970_e20307 < 0.0 { 1.0 } else { 0.0 };
        var_guard350 = assign19970_e20309;

        let (assign19980_e20360, assign19980_e20360_d_n5, assign19980_e20360_d_n6, assign19980_e20360_d_n7, assign19980_e20360_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) && (var_guard347 == 0.0)) && (var_guard349 == 0.0)) && (var_guard350 != 0.0)) {
        let assign19980_e20327: f64 = (-230.25850929940458);
        let assign19980_e20329: f64 = (-var_fbbtgat);
        let assign19980_e20331: f64 = (assign19980_e20329 / var_fmaxr);
        let assign19980_e20332: f64 = (assign19980_e20327 - assign19980_e20331);
        let assign19980_e20336: f64 = (-230.25850929940458);
        let assign19980_e20338: f64 = (-var_fbbtgat);
        let assign19980_e20340: f64 = (assign19980_e20338 / var_fmaxr);
        let assign19980_e20341: f64 = (assign19980_e20336 - assign19980_e20340);
        let assign19980_e20344: f64 = (-230.25850929940458);
        let assign19980_e20346: f64 = (-var_fbbtgat);
        let assign19980_e20348: f64 = (assign19980_e20346 / var_fmaxr);
        let assign19980_e20349: f64 = (assign19980_e20344 - assign19980_e20348);
        let assign19980_e20351: f64 = (assign19980_e20349 * 0.3333333333333333);
        let assign19980_e20352: f64 = (1.0 + assign19980_e20351);
        let assign19980_e20353: f64 = (assign19980_e20341 * assign19980_e20352);
        let assign19980_e20354: f64 = (0.5 * assign19980_e20353);
        let assign19980_e20355: f64 = (1.0 + assign19980_e20354);
        let assign19980_e20356: f64 = (assign19980_e20332 * assign19980_e20355);
        let assign19980_e20357: f64 = (1.0 + assign19980_e20356);
        let assign19980_e20358: f64 = (1e-100 / assign19980_e20357);
        (assign19980_e20358, (-((1e-100 * (((-((((-var_fbbtgat_dn5) * var_fmaxr) - (assign19980_e20329 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * assign19980_e20355) + (assign19980_e20332 * (0.5 * (((-((((-var_fbbtgat_dn5) * var_fmaxr) - (assign19980_e20338 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * assign19980_e20352) + (assign19980_e20341 * ((-((((-var_fbbtgat_dn5) * var_fmaxr) - (assign19980_e20346 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign19980_e20357 * assign19980_e20357))), (-((1e-100 * (((-((((-var_fbbtgat_dn6) * var_fmaxr) - (assign19980_e20329 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign19980_e20355) + (assign19980_e20332 * (0.5 * (((-((((-var_fbbtgat_dn6) * var_fmaxr) - (assign19980_e20338 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign19980_e20352) + (assign19980_e20341 * ((-((((-var_fbbtgat_dn6) * var_fmaxr) - (assign19980_e20346 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign19980_e20357 * assign19980_e20357))), (-((1e-100 * (((-((((-var_fbbtgat_dn7) * var_fmaxr) - (assign19980_e20329 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign19980_e20355) + (assign19980_e20332 * (0.5 * (((-((((-var_fbbtgat_dn7) * var_fmaxr) - (assign19980_e20338 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign19980_e20352) + (assign19980_e20341 * ((-((((-var_fbbtgat_dn7) * var_fmaxr) - (assign19980_e20346 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign19980_e20357 * assign19980_e20357))), (-((1e-100 * (((-((((-var_fbbtgat_dn8) * var_fmaxr) - (assign19980_e20329 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign19980_e20355) + (assign19980_e20332 * (0.5 * (((-((((-var_fbbtgat_dn8) * var_fmaxr) - (assign19980_e20338 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign19980_e20352) + (assign19980_e20341 * ((-((((-var_fbbtgat_dn8) * var_fmaxr) - (assign19980_e20346 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign19980_e20357 * assign19980_e20357))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign19980_e20360;
        var_tmp_dn5 = assign19980_e20360_d_n5;
        var_tmp_dn6 = assign19980_e20360_d_n6;
        var_tmp_dn7 = assign19980_e20360_d_n7;
        var_tmp_dn8 = assign19980_e20360_d_n8;

        let (assign19990_e20409, assign19990_e20409_d_n5, assign19990_e20409_d_n6, assign19990_e20409_d_n7, assign19990_e20409_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) && (var_guard347 == 0.0)) && (var_guard349 == 0.0)) && (var_guard350 == 0.0)) {
        let assign19990_e20379: f64 = (-var_fbbtgat);
        let assign19990_e20381: f64 = (assign19990_e20379 / var_fmaxr);
        let assign19990_e20383: f64 = (assign19990_e20381 - 230.25850929940458);
        let assign19990_e20387: f64 = (-var_fbbtgat);
        let assign19990_e20389: f64 = (assign19990_e20387 / var_fmaxr);
        let assign19990_e20391: f64 = (assign19990_e20389 - 230.25850929940458);
        let assign19990_e20394: f64 = (-var_fbbtgat);
        let assign19990_e20396: f64 = (assign19990_e20394 / var_fmaxr);
        let assign19990_e20398: f64 = (assign19990_e20396 - 230.25850929940458);
        let assign19990_e20400: f64 = (assign19990_e20398 * 0.3333333333333333);
        let assign19990_e20401: f64 = (1.0 + assign19990_e20400);
        let assign19990_e20402: f64 = (assign19990_e20391 * assign19990_e20401);
        let assign19990_e20403: f64 = (0.5 * assign19990_e20402);
        let assign19990_e20404: f64 = (1.0 + assign19990_e20403);
        let assign19990_e20405: f64 = (assign19990_e20383 * assign19990_e20404);
        let assign19990_e20406: f64 = (1.0 + assign19990_e20405);
        let assign19990_e20407: f64 = (1e100 * assign19990_e20406);
        (assign19990_e20407, (1e100 * ((((((-var_fbbtgat_dn5) * var_fmaxr) - (assign19990_e20379 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * assign19990_e20404) + (assign19990_e20383 * (0.5 * ((((((-var_fbbtgat_dn5) * var_fmaxr) - (assign19990_e20387 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * assign19990_e20401) + (assign19990_e20391 * (((((-var_fbbtgat_dn5) * var_fmaxr) - (assign19990_e20394 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_dn6) * var_fmaxr) - (assign19990_e20379 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign19990_e20404) + (assign19990_e20383 * (0.5 * ((((((-var_fbbtgat_dn6) * var_fmaxr) - (assign19990_e20387 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign19990_e20401) + (assign19990_e20391 * (((((-var_fbbtgat_dn6) * var_fmaxr) - (assign19990_e20394 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_dn7) * var_fmaxr) - (assign19990_e20379 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign19990_e20404) + (assign19990_e20383 * (0.5 * ((((((-var_fbbtgat_dn7) * var_fmaxr) - (assign19990_e20387 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign19990_e20401) + (assign19990_e20391 * (((((-var_fbbtgat_dn7) * var_fmaxr) - (assign19990_e20394 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_dn8) * var_fmaxr) - (assign19990_e20379 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign19990_e20404) + (assign19990_e20383 * (0.5 * ((((((-var_fbbtgat_dn8) * var_fmaxr) - (assign19990_e20387 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign19990_e20401) + (assign19990_e20391 * (((((-var_fbbtgat_dn8) * var_fmaxr) - (assign19990_e20394 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign19990_e20409;
        var_tmp_dn5 = assign19990_e20409_d_n5;
        var_tmp_dn6 = assign19990_e20409_d_n6;
        var_tmp_dn7 = assign19990_e20409_d_n7;
        var_tmp_dn8 = assign19990_e20409_d_n8;

        let (assign20000_e20429, assign20000_e20429_d_n5, assign20000_e20429_d_n6, assign20000_e20429_d_n7, assign20000_e20429_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) && (var_guard347 == 0.0)) {
        let assign20000_e20422: f64 = (var_v2 * var_fmaxr);
        let assign20000_e20424: f64 = (assign20000_e20422 * var_fmaxr);
        let assign20000_e20426: f64 = (assign20000_e20424 * var_tmp);
        let assign20000_e20427: f64 = (p.p846 * assign20000_e20426);
        (assign20000_e20427, (p.p846 * (((((var_v2 * var_fmaxr_dn5) * var_fmaxr) + (assign20000_e20422 * var_fmaxr_dn5)) * var_tmp) + (assign20000_e20424 * var_tmp_dn5))), (p.p846 * (((((var_v2 * var_fmaxr_dn6) * var_fmaxr) + (assign20000_e20422 * var_fmaxr_dn6)) * var_tmp) + (assign20000_e20424 * var_tmp_dn6))), (p.p846 * (((((var_v2 * var_fmaxr_dn7) * var_fmaxr) + (assign20000_e20422 * var_fmaxr_dn7)) * var_tmp) + (assign20000_e20424 * var_tmp_dn7))), (p.p846 * (((((var_v2 * var_fmaxr_dn8) * var_fmaxr) + (assign20000_e20422 * var_fmaxr_dn8)) * var_tmp) + (assign20000_e20424 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign20000_e20429;
        var_ibbt_dn5 = assign20000_e20429_d_n5;
        var_ibbt_dn6 = assign20000_e20429_d_n6;
        var_ibbt_dn7 = assign20000_e20429_d_n7;
        var_ibbt_dn8 = assign20000_e20429_d_n8;

        let assign20010_e20432: f64 = if p.p855 > 1000.0 { 1.0 } else { 0.0 };
        var_guard351 = assign20010_e20432;

        let (assign20020_e20443, assign20020_e20443_d_n5, assign20020_e20443_d_n6, assign20020_e20443_d_n7, assign20020_e20443_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) && (var_guard351 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign20020_e20443;
        var_fbreakdown_dn5 = assign20020_e20443_d_n5;
        var_fbreakdown_dn6 = assign20020_e20443_d_n6;
        var_fbreakdown_dn7 = assign20020_e20443_d_n7;
        var_fbreakdown_dn8 = assign20020_e20443_d_n8;

        let assign20030_e20446: f64 = (-var_alphaav);
        let assign20030_e20448: f64 = (assign20030_e20446 * p.p855);
        let assign20030_e20449: f64 = if var_vav > assign20030_e20448 { 1.0 } else { 0.0 };
        var_guard352 = assign20030_e20449;

        let assign20040_e20452: f64 = if p.p858 == 4.0 { 1.0 } else { 0.0 };
        var_guard353 = assign20040_e20452;

        let (assign20050_e20482, assign20050_e20482_d_n5, assign20050_e20482_d_n6, assign20050_e20482_d_n7, assign20050_e20482_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) && (var_guard351 == 0.0)) && (var_guard352 != 0.0)) && (var_guard353 != 0.0)) {
        let assign20050_e20468: f64 = (var_vav * var_vbrinvgat);
        let assign20050_e20471: f64 = (var_vav * var_vbrinvgat);
        let assign20050_e20472: f64 = (assign20050_e20468 * assign20050_e20471);
        let assign20050_e20475: f64 = (var_vav * var_vbrinvgat);
        let assign20050_e20476: f64 = (assign20050_e20472 * assign20050_e20475);
        let assign20050_e20479: f64 = (var_vav * var_vbrinvgat);
        let assign20050_e20480: f64 = (assign20050_e20476 * assign20050_e20479);
        (assign20050_e20480, (((((((var_vav * var_vbrinvgat_dn5) * assign20050_e20471) + (assign20050_e20468 * (var_vav * var_vbrinvgat_dn5))) * assign20050_e20475) + (assign20050_e20472 * (var_vav * var_vbrinvgat_dn5))) * assign20050_e20479) + (assign20050_e20476 * (var_vav * var_vbrinvgat_dn5))), (((((((var_vav * var_vbrinvgat_dn6) * assign20050_e20471) + (assign20050_e20468 * (var_vav * var_vbrinvgat_dn6))) * assign20050_e20475) + (assign20050_e20472 * (var_vav * var_vbrinvgat_dn6))) * assign20050_e20479) + (assign20050_e20476 * (var_vav * var_vbrinvgat_dn6))), (((((((var_vav * var_vbrinvgat_dn7) * assign20050_e20471) + (assign20050_e20468 * (var_vav * var_vbrinvgat_dn7))) * assign20050_e20475) + (assign20050_e20472 * (var_vav * var_vbrinvgat_dn7))) * assign20050_e20479) + (assign20050_e20476 * (var_vav * var_vbrinvgat_dn7))), (((((((var_vav * var_vbrinvgat_dn8) * assign20050_e20471) + (assign20050_e20468 * (var_vav * var_vbrinvgat_dn8))) * assign20050_e20475) + (assign20050_e20472 * (var_vav * var_vbrinvgat_dn8))) * assign20050_e20479) + (assign20050_e20476 * (var_vav * var_vbrinvgat_dn8))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign20050_e20482;
        var_tmp_dn5 = assign20050_e20482_d_n5;
        var_tmp_dn6 = assign20050_e20482_d_n6;
        var_tmp_dn7 = assign20050_e20482_d_n7;
        var_tmp_dn8 = assign20050_e20482_d_n8;

        let (assign20060_e20504, assign20060_e20504_d_n5, assign20060_e20504_d_n6, assign20060_e20504_d_n7, assign20060_e20504_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) && (var_guard351 == 0.0)) && (var_guard352 != 0.0)) && (var_guard353 == 0.0)) {
        let assign20060_e20499: f64 = (var_vav * var_vbrinvgat);
        let assign20060_e20500: f64 = (assign20060_e20499).abs();
        let assign20060_e20502: f64 = (assign20060_e20500).powf(p.p858);
        (assign20060_e20502, if 0.0 == 0.0 && ((p.p858) as f64).is_finite() && ((p.p858) as f64).fract() == 0.0 { if p.p858 == 0.0 { 0.0 } else { (p.p858 * ((assign20060_e20500).powf(p.p858 - 1.0) * if assign20060_e20499 >= 0.0 { (var_vav * var_vbrinvgat_dn5) } else { (-(var_vav * var_vbrinvgat_dn5)) })) } } else { (assign20060_e20502 * (p.p858 * (if assign20060_e20499 >= 0.0 { (var_vav * var_vbrinvgat_dn5) } else { (-(var_vav * var_vbrinvgat_dn5)) } / assign20060_e20500))) }, if 0.0 == 0.0 && ((p.p858) as f64).is_finite() && ((p.p858) as f64).fract() == 0.0 { if p.p858 == 0.0 { 0.0 } else { (p.p858 * ((assign20060_e20500).powf(p.p858 - 1.0) * if assign20060_e20499 >= 0.0 { (var_vav * var_vbrinvgat_dn6) } else { (-(var_vav * var_vbrinvgat_dn6)) })) } } else { (assign20060_e20502 * (p.p858 * (if assign20060_e20499 >= 0.0 { (var_vav * var_vbrinvgat_dn6) } else { (-(var_vav * var_vbrinvgat_dn6)) } / assign20060_e20500))) }, if 0.0 == 0.0 && ((p.p858) as f64).is_finite() && ((p.p858) as f64).fract() == 0.0 { if p.p858 == 0.0 { 0.0 } else { (p.p858 * ((assign20060_e20500).powf(p.p858 - 1.0) * if assign20060_e20499 >= 0.0 { (var_vav * var_vbrinvgat_dn7) } else { (-(var_vav * var_vbrinvgat_dn7)) })) } } else { (assign20060_e20502 * (p.p858 * (if assign20060_e20499 >= 0.0 { (var_vav * var_vbrinvgat_dn7) } else { (-(var_vav * var_vbrinvgat_dn7)) } / assign20060_e20500))) }, if 0.0 == 0.0 && ((p.p858) as f64).is_finite() && ((p.p858) as f64).fract() == 0.0 { if p.p858 == 0.0 { 0.0 } else { (p.p858 * ((assign20060_e20500).powf(p.p858 - 1.0) * if assign20060_e20499 >= 0.0 { (var_vav * var_vbrinvgat_dn8) } else { (-(var_vav * var_vbrinvgat_dn8)) })) } } else { (assign20060_e20502 * (p.p858 * (if assign20060_e20499 >= 0.0 { (var_vav * var_vbrinvgat_dn8) } else { (-(var_vav * var_vbrinvgat_dn8)) } / assign20060_e20500))) },)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign20060_e20504;
        var_tmp_dn5 = assign20060_e20504_d_n5;
        var_tmp_dn6 = assign20060_e20504_d_n6;
        var_tmp_dn7 = assign20060_e20504_d_n7;
        var_tmp_dn8 = assign20060_e20504_d_n8;

        let (assign20070_e20522, assign20070_e20522_d_n5, assign20070_e20522_d_n6, assign20070_e20522_d_n7, assign20070_e20522_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) && (var_guard351 == 0.0)) && (var_guard352 != 0.0)) {
        let assign20070_e20519: f64 = (1.0 - var_tmp);
        let assign20070_e20520: f64 = (1.0 / assign20070_e20519);
        (assign20070_e20520, (-((-var_tmp_dn5) / (assign20070_e20519 * assign20070_e20519))), (-((-var_tmp_dn6) / (assign20070_e20519 * assign20070_e20519))), (-((-var_tmp_dn7) / (assign20070_e20519 * assign20070_e20519))), (-((-var_tmp_dn8) / (assign20070_e20519 * assign20070_e20519))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign20070_e20522;
        var_fbreakdown_dn5 = assign20070_e20522_d_n5;
        var_fbreakdown_dn6 = assign20070_e20522_d_n6;
        var_fbreakdown_dn7 = assign20070_e20522_d_n7;
        var_fbreakdown_dn8 = assign20070_e20522_d_n8;

        let (assign20080_e20545, assign20080_e20545_d_n5, assign20080_e20545_d_n6, assign20080_e20545_d_n7, assign20080_e20545_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) && (var_guard351 == 0.0)) && (var_guard352 == 0.0)) {
        let assign20080_e20539: f64 = (var_alphaav * p.p855);
        let assign20080_e20540: f64 = (var_vav + assign20080_e20539);
        let assign20080_e20542: f64 = (assign20080_e20540 * var_slopegat);
        let assign20080_e20543: f64 = (var_fstopgat + assign20080_e20542);
        (assign20080_e20543, (assign20080_e20540 * var_slopegat_dn5), (assign20080_e20540 * var_slopegat_dn6), (assign20080_e20540 * var_slopegat_dn7), (assign20080_e20540 * var_slopegat_dn8),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign20080_e20545;
        var_fbreakdown_dn5 = assign20080_e20545_d_n5;
        var_fbreakdown_dn6 = assign20080_e20545_d_n6;
        var_fbreakdown_dn7 = assign20080_e20545_d_n7;
        var_fbreakdown_dn8 = assign20080_e20545_d_n8;

        let (assign20090_e20564, assign20090_e20564_d_n5, assign20090_e20564_d_n6, assign20090_e20564_d_n7, assign20090_e20564_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard337 == 0.0)) {
        let assign20090_e20555: f64 = (var_id__blk213 + var_isrh);
        let assign20090_e20557: f64 = (assign20090_e20555 + var_itat);
        let assign20090_e20559: f64 = (assign20090_e20557 + var_ibbt);
        let assign20090_e20560: f64 = (p.p29 * assign20090_e20559);
        let assign20090_e20562: f64 = (assign20090_e20560 * var_fbreakdown);
        (assign20090_e20562, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign20090_e20560 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign20090_e20560 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign20090_e20560 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign20090_e20560 * var_fbreakdown_dn8)),)
    } else {
        (var_ijungat, var_ijungat_dn5, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8,)
    }
};
        var_ijungat = assign20090_e20564;
        var_ijungat_dn5 = assign20090_e20564_d_n5;
        var_ijungat_dn6 = assign20090_e20564_d_n6;
        var_ijungat_dn7 = assign20090_e20564_d_n7;
        var_ijungat_dn8 = assign20090_e20564_d_n8;

        let (assign20100_e20580, assign20100_e20580_d_n5, assign20100_e20580_d_n6, assign20100_e20580_d_n7, assign20100_e20580_d_n8,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        let assign20100_e20570: f64 = (var_absource_i * var_ijunbot);
        let assign20100_e20573: f64 = (var_lssource_i * var_ijunsti);
        let assign20100_e20574: f64 = (assign20100_e20570 + assign20100_e20573);
        let assign20100_e20577: f64 = (var_lgsource_i * var_ijungat);
        let assign20100_e20578: f64 = (assign20100_e20574 + assign20100_e20577);
        (assign20100_e20578, (((var_absource_i * var_ijunbot_dn5) + (var_lssource_i * var_ijunsti_dn5)) + (var_lgsource_i * var_ijungat_dn5)), (((var_absource_i * var_ijunbot_dn6) + (var_lssource_i * var_ijunsti_dn6)) + (var_lgsource_i * var_ijungat_dn6)), (((var_absource_i * var_ijunbot_dn7) + (var_lssource_i * var_ijunsti_dn7)) + (var_lgsource_i * var_ijungat_dn7)), (((var_absource_i * var_ijunbot_dn8) + (var_lssource_i * var_ijunsti_dn8)) + (var_lgsource_i * var_ijungat_dn8)),)
    } else {
        (var_i2, var_i2_dn5, var_i2_dn6, var_i2_dn7, var_i2_dn8,)
    }
};
        var_i2 = assign20100_e20580;
        var_i2_dn5 = assign20100_e20580_d_n5;
        var_i2_dn6 = assign20100_e20580_d_n6;
        var_i2_dn7 = assign20100_e20580_d_n7;
        var_i2_dn8 = assign20100_e20580_d_n8;

        let (assign20110_e20586,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        (0.0,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign20110_e20586;

        let (assign20120_e20592,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        (0.0,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign20120_e20592;

        let assign20130_e20604: f64 = if (!(((var_absource_i == 0.0) && (var_lssource_i == 0.0)) && (var_lgsource_i == 0.0))) { 1.0 } else { 0.0 };
        var_guard354 = assign20130_e20604;

        let assign20210_e20690: f64 = if var_v3 < var_vmax_s { 1.0 } else { 0.0 };
        var_guard355 = assign20210_e20690;

        let assign20220_e20692: f64 = (-0.5);
        let assign20220_e20695: f64 = (var_v3 * var_phitdinv);
        let assign20220_e20696: f64 = (assign20220_e20692 * assign20220_e20695);
        let assign20220_e20697: f64 = (assign20220_e20696).abs();
        let assign20220_e20699: f64 = if assign20220_e20697 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard356 = assign20220_e20699;

        let (assign20230_e20717,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard354 != 0.0)) && (var_guard355 != 0.0)) && (var_guard356 != 0.0)) {
        let assign20230_e20710: f64 = (-0.5);
        let assign20230_e20713: f64 = (var_v3 * var_phitdinv);
        let assign20230_e20714: f64 = (assign20230_e20710 * assign20230_e20713);
        let assign20230_e20715: f64 = (assign20230_e20714).exp();
        (assign20230_e20715,)
    } else {
        (var_z,)
    }
};
        var_z = assign20230_e20717;

        let assign20240_e20719: f64 = (-0.5);
        let assign20240_e20722: f64 = (var_v3 * var_phitdinv);
        let assign20240_e20723: f64 = (assign20240_e20719 * assign20240_e20722);
        let assign20240_e20725: f64 = if assign20240_e20723 < 0.0 { 1.0 } else { 0.0 };
        var_guard357 = assign20240_e20725;

        let (assign20250_e20780,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard354 != 0.0)) && (var_guard355 != 0.0)) && (var_guard356 == 0.0)) && (var_guard357 != 0.0)) {
        let assign20250_e20741: f64 = (-230.25850929940458);
        let assign20250_e20743: f64 = (-0.5);
        let assign20250_e20746: f64 = (var_v3 * var_phitdinv);
        let assign20250_e20747: f64 = (assign20250_e20743 * assign20250_e20746);
        let assign20250_e20748: f64 = (assign20250_e20741 - assign20250_e20747);
        let assign20250_e20752: f64 = (-230.25850929940458);
        let assign20250_e20754: f64 = (-0.5);
        let assign20250_e20757: f64 = (var_v3 * var_phitdinv);
        let assign20250_e20758: f64 = (assign20250_e20754 * assign20250_e20757);
        let assign20250_e20759: f64 = (assign20250_e20752 - assign20250_e20758);
        let assign20250_e20762: f64 = (-230.25850929940458);
        let assign20250_e20764: f64 = (-0.5);
        let assign20250_e20767: f64 = (var_v3 * var_phitdinv);
        let assign20250_e20768: f64 = (assign20250_e20764 * assign20250_e20767);
        let assign20250_e20769: f64 = (assign20250_e20762 - assign20250_e20768);
        let assign20250_e20771: f64 = (assign20250_e20769 * 0.3333333333333333);
        let assign20250_e20772: f64 = (1.0 + assign20250_e20771);
        let assign20250_e20773: f64 = (assign20250_e20759 * assign20250_e20772);
        let assign20250_e20774: f64 = (0.5 * assign20250_e20773);
        let assign20250_e20775: f64 = (1.0 + assign20250_e20774);
        let assign20250_e20776: f64 = (assign20250_e20748 * assign20250_e20775);
        let assign20250_e20777: f64 = (1.0 + assign20250_e20776);
        let assign20250_e20778: f64 = (1e-100 / assign20250_e20777);
        (assign20250_e20778,)
    } else {
        (var_z,)
    }
};
        var_z = assign20250_e20780;

        let (assign20260_e20833,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard354 != 0.0)) && (var_guard355 != 0.0)) && (var_guard356 == 0.0)) && (var_guard357 == 0.0)) {
        let assign20260_e20797: f64 = (-0.5);
        let assign20260_e20800: f64 = (var_v3 * var_phitdinv);
        let assign20260_e20801: f64 = (assign20260_e20797 * assign20260_e20800);
        let assign20260_e20803: f64 = (assign20260_e20801 - 230.25850929940458);
        let assign20260_e20807: f64 = (-0.5);
        let assign20260_e20810: f64 = (var_v3 * var_phitdinv);
        let assign20260_e20811: f64 = (assign20260_e20807 * assign20260_e20810);
        let assign20260_e20813: f64 = (assign20260_e20811 - 230.25850929940458);
        let assign20260_e20816: f64 = (-0.5);
        let assign20260_e20819: f64 = (var_v3 * var_phitdinv);
        let assign20260_e20820: f64 = (assign20260_e20816 * assign20260_e20819);
        let assign20260_e20822: f64 = (assign20260_e20820 - 230.25850929940458);
        let assign20260_e20824: f64 = (assign20260_e20822 * 0.3333333333333333);
        let assign20260_e20825: f64 = (1.0 + assign20260_e20824);
        let assign20260_e20826: f64 = (assign20260_e20813 * assign20260_e20825);
        let assign20260_e20827: f64 = (0.5 * assign20260_e20826);
        let assign20260_e20828: f64 = (1.0 + assign20260_e20827);
        let assign20260_e20829: f64 = (assign20260_e20803 * assign20260_e20828);
        let assign20260_e20830: f64 = (1.0 + assign20260_e20829);
        let assign20260_e20831: f64 = (1e100 * assign20260_e20830);
        (assign20260_e20831,)
    } else {
        (var_z,)
    }
};
        var_z = assign20260_e20833;

        let (assign20270_e20845,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard354 != 0.0)) && (var_guard355 != 0.0)) {
        let assign20270_e20843: f64 = (1.0 / var_z);
        (assign20270_e20843,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign20270_e20845;

        let (assign20280_e20857,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard354 != 0.0)) && (var_guard355 != 0.0)) {
        let assign20280_e20855: f64 = (var_zinv * var_zinv);
        (assign20280_e20855,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign20280_e20857;

        let (assign20290_e20876,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard354 != 0.0)) && (var_guard355 == 0.0)) {
        let assign20290_e20869: f64 = (var_v3 - var_vmax_s);
        let assign20290_e20871: f64 = (assign20290_e20869 * var_phitdinv);
        let assign20290_e20872: f64 = (1.0 + assign20290_e20871);
        let assign20290_e20874: f64 = (assign20290_e20872 * var_exp_vmax_over_phitd_s);
        (assign20290_e20874,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign20290_e20876;

        let (assign20300_e20888,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard354 != 0.0)) && (var_guard355 == 0.0)) {
        let assign20300_e20886: f64 = (var_idmult).sqrt();
        (assign20300_e20886,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign20300_e20888;

        let (assign20310_e20901,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard354 != 0.0)) && (var_guard355 == 0.0)) {
        let assign20310_e20899: f64 = (1.0 / var_zinv);
        (assign20310_e20899,)
    } else {
        (var_z,)
    }
};
        var_z = assign20310_e20901;

        let (assign20320_e20911,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard354 != 0.0)) {
        let assign20320_e20909: f64 = (var_idmult - 1.0);
        (assign20320_e20909,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign20320_e20911;

        let assign20330_e20914: f64 = if var_v3 > 0.0 { 1.0 } else { 0.0 };
        var_guard358 = assign20330_e20914;

        let (assign20340_e20940,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard354 != 0.0)) && (var_guard358 != 0.0)) {
        let assign20340_e20926: f64 = (2.0 + var_z);
        let assign20340_e20929: f64 = (var_z + 1.0);
        let assign20340_e20932: f64 = (var_z + 3.0);
        let assign20340_e20933: f64 = (assign20340_e20929 * assign20340_e20932);
        let assign20340_e20934: f64 = (assign20340_e20933).sqrt();
        let assign20340_e20935: f64 = (assign20340_e20926 + assign20340_e20934);
        let assign20340_e20936: f64 = (assign20340_e20935).ln();
        let assign20340_e20937: f64 = (var_phitd * assign20340_e20936);
        let assign20340_e20938: f64 = (2.0 * assign20340_e20937);
        (assign20340_e20938,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign20340_e20940;

        let (assign20350_e20974,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard354 != 0.0)) && (var_guard358 == 0.0)) {
        let assign20350_e20950: f64 = (-var_v3);
        let assign20350_e20955: f64 = (2.0 * var_zinv);
        let assign20350_e20957: f64 = (assign20350_e20955 + 1.0);
        let assign20350_e20960: f64 = (1.0 + var_zinv);
        let assign20350_e20964: f64 = (3.0 * var_zinv);
        let assign20350_e20965: f64 = (1.0 + assign20350_e20964);
        let assign20350_e20966: f64 = (assign20350_e20960 * assign20350_e20965);
        let assign20350_e20967: f64 = (assign20350_e20966).sqrt();
        let assign20350_e20968: f64 = (assign20350_e20957 + assign20350_e20967);
        let assign20350_e20969: f64 = (assign20350_e20968).ln();
        let assign20350_e20970: f64 = (var_phitd * assign20350_e20969);
        let assign20350_e20971: f64 = (2.0 * assign20350_e20970);
        let assign20350_e20972: f64 = (assign20350_e20950 + assign20350_e20971);
        (assign20350_e20972,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign20350_e20974;

        let (assign20360_e20984,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard354 != 0.0)) {
        let assign20360_e20982: f64 = (var_vbimin_s - var_two_psistar);
        (assign20360_e20982,)
    } else {
        (var_vjlim,)
    }
};
        var_vjlim = assign20360_e20984;

        let (assign20370_e21011,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard354 != 0.0)) {
        let assign20370_e20993: f64 = (var_v3 + var_vjlim);
        let assign20370_e20996: f64 = (var_v3 - var_vjlim);
        let assign20370_e20999: f64 = (var_v3 - var_vjlim);
        let assign20370_e21000: f64 = (assign20370_e20996 * assign20370_e20999);
        let assign20370_e21003: f64 = (4.0 * var_phitd);
        let assign20370_e21005: f64 = (assign20370_e21003 * var_phitd);
        let assign20370_e21006: f64 = (assign20370_e21000 + assign20370_e21005);
        let assign20370_e21007: f64 = (assign20370_e21006).sqrt();
        let assign20370_e21008: f64 = (assign20370_e20993 - assign20370_e21007);
        let assign20370_e21009: f64 = (0.5 * assign20370_e21008);
        (assign20370_e21009,)
    } else {
        (var_vjsrh,)
    }
};
        var_vjsrh = assign20370_e21011;

        let (assign20380_e21038,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard354 != 0.0)) {
        let assign20380_e21020: f64 = (var_v3 + var_vbbtlim_s);
        let assign20380_e21023: f64 = (var_v3 - var_vbbtlim_s);
        let assign20380_e21026: f64 = (var_v3 - var_vbbtlim_s);
        let assign20380_e21027: f64 = (assign20380_e21023 * assign20380_e21026);
        let assign20380_e21030: f64 = (4.0 * var_phitr);
        let assign20380_e21032: f64 = (assign20380_e21030 * var_phitr);
        let assign20380_e21033: f64 = (assign20380_e21027 + assign20380_e21032);
        let assign20380_e21034: f64 = (assign20380_e21033).sqrt();
        let assign20380_e21035: f64 = (assign20380_e21020 - assign20380_e21034);
        let assign20380_e21036: f64 = (0.5 * assign20380_e21035);
        (assign20380_e21036,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign20380_e21038;

        *var_fbreakdown_slot = var_fbreakdown;
        *var_fbreakdown_dn5_slot = var_fbreakdown_dn5;
        *var_fbreakdown_dn6_slot = var_fbreakdown_dn6;
        *var_fbreakdown_dn7_slot = var_fbreakdown_dn7;
        *var_fbreakdown_dn8_slot = var_fbreakdown_dn8;
        *var_guard350_slot = var_guard350;
        *var_guard351_slot = var_guard351;
        *var_guard352_slot = var_guard352;
        *var_guard353_slot = var_guard353;
        *var_guard354_slot = var_guard354;
        *var_guard355_slot = var_guard355;
        *var_guard356_slot = var_guard356;
        *var_guard357_slot = var_guard357;
        *var_guard358_slot = var_guard358;
        *var_i2_slot = var_i2;
        *var_i2_dn5_slot = var_i2_dn5;
        *var_i2_dn6_slot = var_i2_dn6;
        *var_i2_dn7_slot = var_i2_dn7;
        *var_i2_dn8_slot = var_i2_dn8;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn5_slot = var_ibbt_dn5;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
        *var_idmult_slot = var_idmult;
        *var_ijungat_slot = var_ijungat;
        *var_ijungat_dn5_slot = var_ijungat_dn5;
        *var_ijungat_dn6_slot = var_ijungat_dn6;
        *var_ijungat_dn7_slot = var_ijungat_dn7;
        *var_ijungat_dn8_slot = var_ijungat_dn8;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn5_slot = var_tmp_dn5;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_two_psistar_slot = var_two_psistar;
        *var_vbbt_slot = var_vbbt;
        *var_vjlim_slot = var_vjlim;
        *var_vjsrh_slot = var_vjsrh;
        *var_z_slot = var_z;
        *var_zinv_slot = var_zinv;
    }

    pub(super) fn stamp_transient_block_35(
        p: &Parameters,
        var_absource_i: f64,
        var_atatbot: f64,
        var_btatpartbot: f64,
        var_ftdbot: f64,
        var_guard176: f64,
        var_guard193: f64,
        var_guard354: f64,
        var_idmult: f64,
        var_idsatbot: f64,
        var_one_minus_pbot: f64,
        var_one_over_one_minus_pbot: f64,
        var_perfc: f64,
        var_two_psistar: f64,
        var_v3: f64,
        var_vbibot: f64,
        var_vbirbotinv: f64,
        var_vjsrh: f64,
        var_wdepnulrbot: f64,
        var_zinv: f64,
        var_asrh_slot: &mut f64,
        var_asrh_dn5_slot: &mut f64,
        var_asrh_dn6_slot: &mut f64,
        var_asrh_dn7_slot: &mut f64,
        var_asrh_dn8_slot: &mut f64,
        var_btat_slot: &mut f64,
        var_btat_dn5_slot: &mut f64,
        var_btat_dn6_slot: &mut f64,
        var_btat_dn7_slot: &mut f64,
        var_btat_dn8_slot: &mut f64,
        var_dwsrh_slot: &mut f64,
        var_guard359_slot: &mut f64,
        var_guard360_slot: &mut f64,
        var_guard361_slot: &mut f64,
        var_guard362_slot: &mut f64,
        var_guard363_slot: &mut f64,
        var_guard364_slot: &mut f64,
        var_guard365_slot: &mut f64,
        var_guard366_slot: &mut f64,
        var_id__blk213_slot: &mut f64,
        var_ijunbot_slot: &mut f64,
        var_ijunbot_dn5_slot: &mut f64,
        var_ijunbot_dn6_slot: &mut f64,
        var_ijunbot_dn7_slot: &mut f64,
        var_ijunbot_dn8_slot: &mut f64,
        var_isrh_slot: &mut f64,
        var_isrh_dn5_slot: &mut f64,
        var_isrh_dn6_slot: &mut f64,
        var_isrh_dn7_slot: &mut f64,
        var_isrh_dn8_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn5_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_ktat_slot: &mut f64,
        var_ktat_dn5_slot: &mut f64,
        var_ktat_dn6_slot: &mut f64,
        var_ktat_dn7_slot: &mut f64,
        var_ktat_dn8_slot: &mut f64,
        var_ltat_slot: &mut f64,
        var_ltat_dn5_slot: &mut f64,
        var_ltat_dn6_slot: &mut f64,
        var_ltat_dn7_slot: &mut f64,
        var_ltat_dn8_slot: &mut f64,
        var_mtat_slot: &mut f64,
        var_mtat_dn5_slot: &mut f64,
        var_mtat_dn6_slot: &mut f64,
        var_mtat_dn7_slot: &mut f64,
        var_mtat_dn8_slot: &mut f64,
        var_sqrtumax_slot: &mut f64,
        var_sqrtumax_dn5_slot: &mut f64,
        var_sqrtumax_dn6_slot: &mut f64,
        var_sqrtumax_dn7_slot: &mut f64,
        var_sqrtumax_dn8_slot: &mut f64,
        var_terfc_slot: &mut f64,
        var_terfc_dn5_slot: &mut f64,
        var_terfc_dn6_slot: &mut f64,
        var_terfc_dn7_slot: &mut f64,
        var_terfc_dn8_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn5_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_twoatatoverthreebtat_slot: &mut f64,
        var_twoatatoverthreebtat_dn5_slot: &mut f64,
        var_twoatatoverthreebtat_dn6_slot: &mut f64,
        var_twoatatoverthreebtat_dn7_slot: &mut f64,
        var_twoatatoverthreebtat_dn8_slot: &mut f64,
        var_umax_slot: &mut f64,
        var_umax_dn5_slot: &mut f64,
        var_umax_dn6_slot: &mut f64,
        var_umax_dn7_slot: &mut f64,
        var_umax_dn8_slot: &mut f64,
        var_umaxbeforelimiting_slot: &mut f64,
        var_umaxbeforelimiting_dn5_slot: &mut f64,
        var_umaxbeforelimiting_dn6_slot: &mut f64,
        var_umaxbeforelimiting_dn7_slot: &mut f64,
        var_umaxbeforelimiting_dn8_slot: &mut f64,
        var_umaxpoweronepointfive_slot: &mut f64,
        var_umaxpoweronepointfive_dn5_slot: &mut f64,
        var_umaxpoweronepointfive_dn6_slot: &mut f64,
        var_umaxpoweronepointfive_dn7_slot: &mut f64,
        var_umaxpoweronepointfive_dn8_slot: &mut f64,
        var_vav_slot: &mut f64,
        var_vbi_minus_vjsrh_slot: &mut f64,
        var_wdep_slot: &mut f64,
        var_wdep_dn5_slot: &mut f64,
        var_wdep_dn6_slot: &mut f64,
        var_wdep_dn7_slot: &mut f64,
        var_wdep_dn8_slot: &mut f64,
        var_wgamma_slot: &mut f64,
        var_wgamma_dn5_slot: &mut f64,
        var_wgamma_dn6_slot: &mut f64,
        var_wgamma_dn7_slot: &mut f64,
        var_wgamma_dn8_slot: &mut f64,
        var_wsrh_slot: &mut f64,
        var_wsrhstep_slot: &mut f64,
        var_wtat_slot: &mut f64,
        var_wtat_dn5_slot: &mut f64,
        var_wtat_dn6_slot: &mut f64,
        var_wtat_dn7_slot: &mut f64,
        var_wtat_dn8_slot: &mut f64,
        var_xerfc_slot: &mut f64,
        var_xerfc_dn5_slot: &mut f64,
        var_xerfc_dn6_slot: &mut f64,
        var_xerfc_dn7_slot: &mut f64,
        var_xerfc_dn8_slot: &mut f64,
        var_ysq_slot: &mut f64,
        var_ysq_dn5_slot: &mut f64,
        var_ysq_dn6_slot: &mut f64,
        var_ysq_dn7_slot: &mut f64,
        var_ysq_dn8_slot: &mut f64,
    ) {
        let mut var_asrh: f64 = *var_asrh_slot;
        let mut var_asrh_dn5: f64 = *var_asrh_dn5_slot;
        let mut var_asrh_dn6: f64 = *var_asrh_dn6_slot;
        let mut var_asrh_dn7: f64 = *var_asrh_dn7_slot;
        let mut var_asrh_dn8: f64 = *var_asrh_dn8_slot;
        let mut var_btat: f64 = *var_btat_slot;
        let mut var_btat_dn5: f64 = *var_btat_dn5_slot;
        let mut var_btat_dn6: f64 = *var_btat_dn6_slot;
        let mut var_btat_dn7: f64 = *var_btat_dn7_slot;
        let mut var_btat_dn8: f64 = *var_btat_dn8_slot;
        let mut var_dwsrh: f64 = *var_dwsrh_slot;
        let mut var_guard359: f64 = *var_guard359_slot;
        let mut var_guard360: f64 = *var_guard360_slot;
        let mut var_guard361: f64 = *var_guard361_slot;
        let mut var_guard362: f64 = *var_guard362_slot;
        let mut var_guard363: f64 = *var_guard363_slot;
        let mut var_guard364: f64 = *var_guard364_slot;
        let mut var_guard365: f64 = *var_guard365_slot;
        let mut var_guard366: f64 = *var_guard366_slot;
        let mut var_id__blk213: f64 = *var_id__blk213_slot;
        let mut var_ijunbot: f64 = *var_ijunbot_slot;
        let mut var_ijunbot_dn5: f64 = *var_ijunbot_dn5_slot;
        let mut var_ijunbot_dn6: f64 = *var_ijunbot_dn6_slot;
        let mut var_ijunbot_dn7: f64 = *var_ijunbot_dn7_slot;
        let mut var_ijunbot_dn8: f64 = *var_ijunbot_dn8_slot;
        let mut var_isrh: f64 = *var_isrh_slot;
        let mut var_isrh_dn5: f64 = *var_isrh_dn5_slot;
        let mut var_isrh_dn6: f64 = *var_isrh_dn6_slot;
        let mut var_isrh_dn7: f64 = *var_isrh_dn7_slot;
        let mut var_isrh_dn8: f64 = *var_isrh_dn8_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn5: f64 = *var_itat_dn5_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_ktat: f64 = *var_ktat_slot;
        let mut var_ktat_dn5: f64 = *var_ktat_dn5_slot;
        let mut var_ktat_dn6: f64 = *var_ktat_dn6_slot;
        let mut var_ktat_dn7: f64 = *var_ktat_dn7_slot;
        let mut var_ktat_dn8: f64 = *var_ktat_dn8_slot;
        let mut var_ltat: f64 = *var_ltat_slot;
        let mut var_ltat_dn5: f64 = *var_ltat_dn5_slot;
        let mut var_ltat_dn6: f64 = *var_ltat_dn6_slot;
        let mut var_ltat_dn7: f64 = *var_ltat_dn7_slot;
        let mut var_ltat_dn8: f64 = *var_ltat_dn8_slot;
        let mut var_mtat: f64 = *var_mtat_slot;
        let mut var_mtat_dn5: f64 = *var_mtat_dn5_slot;
        let mut var_mtat_dn6: f64 = *var_mtat_dn6_slot;
        let mut var_mtat_dn7: f64 = *var_mtat_dn7_slot;
        let mut var_mtat_dn8: f64 = *var_mtat_dn8_slot;
        let mut var_sqrtumax: f64 = *var_sqrtumax_slot;
        let mut var_sqrtumax_dn5: f64 = *var_sqrtumax_dn5_slot;
        let mut var_sqrtumax_dn6: f64 = *var_sqrtumax_dn6_slot;
        let mut var_sqrtumax_dn7: f64 = *var_sqrtumax_dn7_slot;
        let mut var_sqrtumax_dn8: f64 = *var_sqrtumax_dn8_slot;
        let mut var_terfc: f64 = *var_terfc_slot;
        let mut var_terfc_dn5: f64 = *var_terfc_dn5_slot;
        let mut var_terfc_dn6: f64 = *var_terfc_dn6_slot;
        let mut var_terfc_dn7: f64 = *var_terfc_dn7_slot;
        let mut var_terfc_dn8: f64 = *var_terfc_dn8_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn5: f64 = *var_tmp_dn5_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_twoatatoverthreebtat: f64 = *var_twoatatoverthreebtat_slot;
        let mut var_twoatatoverthreebtat_dn5: f64 = *var_twoatatoverthreebtat_dn5_slot;
        let mut var_twoatatoverthreebtat_dn6: f64 = *var_twoatatoverthreebtat_dn6_slot;
        let mut var_twoatatoverthreebtat_dn7: f64 = *var_twoatatoverthreebtat_dn7_slot;
        let mut var_twoatatoverthreebtat_dn8: f64 = *var_twoatatoverthreebtat_dn8_slot;
        let mut var_umax: f64 = *var_umax_slot;
        let mut var_umax_dn5: f64 = *var_umax_dn5_slot;
        let mut var_umax_dn6: f64 = *var_umax_dn6_slot;
        let mut var_umax_dn7: f64 = *var_umax_dn7_slot;
        let mut var_umax_dn8: f64 = *var_umax_dn8_slot;
        let mut var_umaxbeforelimiting: f64 = *var_umaxbeforelimiting_slot;
        let mut var_umaxbeforelimiting_dn5: f64 = *var_umaxbeforelimiting_dn5_slot;
        let mut var_umaxbeforelimiting_dn6: f64 = *var_umaxbeforelimiting_dn6_slot;
        let mut var_umaxbeforelimiting_dn7: f64 = *var_umaxbeforelimiting_dn7_slot;
        let mut var_umaxbeforelimiting_dn8: f64 = *var_umaxbeforelimiting_dn8_slot;
        let mut var_umaxpoweronepointfive: f64 = *var_umaxpoweronepointfive_slot;
        let mut var_umaxpoweronepointfive_dn5: f64 = *var_umaxpoweronepointfive_dn5_slot;
        let mut var_umaxpoweronepointfive_dn6: f64 = *var_umaxpoweronepointfive_dn6_slot;
        let mut var_umaxpoweronepointfive_dn7: f64 = *var_umaxpoweronepointfive_dn7_slot;
        let mut var_umaxpoweronepointfive_dn8: f64 = *var_umaxpoweronepointfive_dn8_slot;
        let mut var_vav: f64 = *var_vav_slot;
        let mut var_vbi_minus_vjsrh: f64 = *var_vbi_minus_vjsrh_slot;
        let mut var_wdep: f64 = *var_wdep_slot;
        let mut var_wdep_dn5: f64 = *var_wdep_dn5_slot;
        let mut var_wdep_dn6: f64 = *var_wdep_dn6_slot;
        let mut var_wdep_dn7: f64 = *var_wdep_dn7_slot;
        let mut var_wdep_dn8: f64 = *var_wdep_dn8_slot;
        let mut var_wgamma: f64 = *var_wgamma_slot;
        let mut var_wgamma_dn5: f64 = *var_wgamma_dn5_slot;
        let mut var_wgamma_dn6: f64 = *var_wgamma_dn6_slot;
        let mut var_wgamma_dn7: f64 = *var_wgamma_dn7_slot;
        let mut var_wgamma_dn8: f64 = *var_wgamma_dn8_slot;
        let mut var_wsrh: f64 = *var_wsrh_slot;
        let mut var_wsrhstep: f64 = *var_wsrhstep_slot;
        let mut var_wtat: f64 = *var_wtat_slot;
        let mut var_wtat_dn5: f64 = *var_wtat_dn5_slot;
        let mut var_wtat_dn6: f64 = *var_wtat_dn6_slot;
        let mut var_wtat_dn7: f64 = *var_wtat_dn7_slot;
        let mut var_wtat_dn8: f64 = *var_wtat_dn8_slot;
        let mut var_xerfc: f64 = *var_xerfc_slot;
        let mut var_xerfc_dn5: f64 = *var_xerfc_dn5_slot;
        let mut var_xerfc_dn6: f64 = *var_xerfc_dn6_slot;
        let mut var_xerfc_dn7: f64 = *var_xerfc_dn7_slot;
        let mut var_xerfc_dn8: f64 = *var_xerfc_dn8_slot;
        let mut var_ysq: f64 = *var_ysq_slot;
        let mut var_ysq_dn5: f64 = *var_ysq_dn5_slot;
        let mut var_ysq_dn6: f64 = *var_ysq_dn6_slot;
        let mut var_ysq_dn7: f64 = *var_ysq_dn7_slot;
        let mut var_ysq_dn8: f64 = *var_ysq_dn8_slot;

        let (assign20390_e21065,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard354 != 0.0)) {
        let assign20390_e21047: f64 = var_v3;
        let assign20390_e21050: f64 = var_v3;
        let assign20390_e21053: f64 = var_v3;
        let assign20390_e21054: f64 = (assign20390_e21050 * assign20390_e21053);
        let assign20390_e21057: f64 = (4.0 * 1e-6);
        let assign20390_e21059: f64 = (assign20390_e21057 * 1e-6);
        let assign20390_e21060: f64 = (assign20390_e21054 + assign20390_e21059);
        let assign20390_e21061: f64 = (assign20390_e21060).sqrt();
        let assign20390_e21062: f64 = (assign20390_e21047 - assign20390_e21061);
        let assign20390_e21063: f64 = (0.5 * assign20390_e21062);
        (assign20390_e21063,)
    } else {
        (var_vav,)
    }
};
        var_vav = assign20390_e21065;

        let assign20400_e21068: f64 = if var_absource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard359 = assign20400_e21068;

        let (assign20410_e21076, assign20410_e21076_d_n5, assign20410_e21076_d_n6, assign20410_e21076_d_n7, assign20410_e21076_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunbot, var_ijunbot_dn5, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8,)
    }
};
        var_ijunbot = assign20410_e21076;
        var_ijunbot_dn5 = assign20410_e21076_d_n5;
        var_ijunbot_dn6 = assign20410_e21076_d_n6;
        var_ijunbot_dn7 = assign20410_e21076_d_n7;
        var_ijunbot_dn8 = assign20410_e21076_d_n8;

        let (assign20420_e21087,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) {
        let assign20420_e21085: f64 = (var_idsatbot * var_idmult);
        (assign20420_e21085,)
    } else {
        (var_id__blk213,)
    }
};
        var_id__blk213 = assign20420_e21087;

        let assign20430_e21094: f64 = if ((p.p833 == 0.0) && (p.p838 == 0.0)) { 1.0 } else { 0.0 };
        var_guard360 = assign20430_e21094;

        let (assign20440_e21105, assign20440_e21105_d_n5, assign20440_e21105_d_n6, assign20440_e21105_d_n7, assign20440_e21105_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) && (var_guard360 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign20440_e21105;
        var_isrh_dn5 = assign20440_e21105_d_n5;
        var_isrh_dn6 = assign20440_e21105_d_n6;
        var_isrh_dn7 = assign20440_e21105_d_n7;
        var_isrh_dn8 = assign20440_e21105_d_n8;

        let (assign20450_e21119,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) && (var_guard360 == 0.0)) {
        let assign20450_e21117: f64 = (var_vbibot - var_vjsrh);
        (assign20450_e21117,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign20450_e21119;

        let (assign20460_e21138,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) && (var_guard360 == 0.0)) {
        let assign20460_e21133: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign20460_e21134: f64 = (1.0 - assign20460_e21133);
        let assign20460_e21135: f64 = (assign20460_e21134).sqrt();
        let assign20460_e21136: f64 = (1.0 - assign20460_e21135);
        (assign20460_e21136,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign20460_e21138;

        let assign20470_e21141: f64 = if p.p824 == 0.5 { 1.0 } else { 0.0 };
        var_guard361 = assign20470_e21141;

        let (assign20480_e21155,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) && (var_guard360 == 0.0)) && (var_guard361 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign20480_e21155;

        let (assign20490_e21187,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) && (var_guard360 == 0.0)) && (var_guard361 == 0.0)) {
        let assign20490_e21170: f64 = (var_wsrhstep * var_wsrhstep);
        let assign20490_e21172: f64 = (var_wsrhstep).ln();
        let assign20490_e21173: f64 = (assign20490_e21170 * assign20490_e21172);
        let assign20490_e21176: f64 = (1.0 - var_wsrhstep);
        let assign20490_e21177: f64 = (assign20490_e21173 / assign20490_e21176);
        let assign20490_e21179: f64 = (assign20490_e21177 + var_wsrhstep);
        let assign20490_e21183: f64 = (2.0 * p.p824);
        let assign20490_e21184: f64 = (1.0 - assign20490_e21183);
        let assign20490_e21185: f64 = (assign20490_e21179 * assign20490_e21184);
        (assign20490_e21185,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign20490_e21187;

        let (assign20500_e21201,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) && (var_guard360 == 0.0)) {
        let assign20500_e21199: f64 = (var_wsrhstep + var_dwsrh);
        (assign20500_e21199,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign20500_e21201;

        let assign20510_e21204: f64 = if p.p824 == 0.5 { 1.0 } else { 0.0 };
        var_guard362 = assign20510_e21204;

        let (assign20520_e21221, assign20520_e21221_d_n5, assign20520_e21221_d_n6, assign20520_e21221_d_n7, assign20520_e21221_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) && (var_guard360 == 0.0)) && (var_guard362 != 0.0)) {
        let assign20520_e21218: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv);
        let assign20520_e21219: f64 = (assign20520_e21218).sqrt();
        (assign20520_e21219, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign20520_e21221;
        var_tmp_dn5 = assign20520_e21221_d_n5;
        var_tmp_dn6 = assign20520_e21221_d_n6;
        var_tmp_dn7 = assign20520_e21221_d_n7;
        var_tmp_dn8 = assign20520_e21221_d_n8;

        let (assign20530_e21240, assign20530_e21240_d_n5, assign20530_e21240_d_n6, assign20530_e21240_d_n7, assign20530_e21240_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) && (var_guard360 == 0.0)) && (var_guard362 == 0.0)) {
        let assign20530_e21236: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv);
        let assign20530_e21238: f64 = (assign20530_e21236).powf(p.p824);
        (assign20530_e21238, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign20530_e21240;
        var_tmp_dn5 = assign20530_e21240_d_n5;
        var_tmp_dn6 = assign20530_e21240_d_n6;
        var_tmp_dn7 = assign20530_e21240_d_n7;
        var_tmp_dn8 = assign20530_e21240_d_n8;

        let (assign20540_e21254, assign20540_e21254_d_n5, assign20540_e21254_d_n6, assign20540_e21254_d_n7, assign20540_e21254_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) && (var_guard360 == 0.0)) {
        let assign20540_e21252: f64 = (var_wdepnulrbot * var_tmp);
        (assign20540_e21252, (var_wdepnulrbot * var_tmp_dn5), (var_wdepnulrbot * var_tmp_dn6), (var_wdepnulrbot * var_tmp_dn7), (var_wdepnulrbot * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign20540_e21254;
        var_wdep_dn5 = assign20540_e21254_d_n5;
        var_wdep_dn6 = assign20540_e21254_d_n6;
        var_wdep_dn7 = assign20540_e21254_d_n7;
        var_wdep_dn8 = assign20540_e21254_d_n8;

        let (assign20550_e21272, assign20550_e21272_d_n5, assign20550_e21272_d_n6, assign20550_e21272_d_n7, assign20550_e21272_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) && (var_guard360 == 0.0)) {
        let assign20550_e21267: f64 = (var_zinv - 1.0);
        let assign20550_e21269: f64 = (assign20550_e21267 * var_wdep);
        let assign20550_e21270: f64 = (var_ftdbot * assign20550_e21269);
        (assign20550_e21270, (var_ftdbot * (assign20550_e21267 * var_wdep_dn5)), (var_ftdbot * (assign20550_e21267 * var_wdep_dn6)), (var_ftdbot * (assign20550_e21267 * var_wdep_dn7)), (var_ftdbot * (assign20550_e21267 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign20550_e21272;
        var_asrh_dn5 = assign20550_e21272_d_n5;
        var_asrh_dn6 = assign20550_e21272_d_n6;
        var_asrh_dn7 = assign20550_e21272_d_n7;
        var_asrh_dn8 = assign20550_e21272_d_n8;

        let (assign20560_e21288, assign20560_e21288_d_n5, assign20560_e21288_d_n6, assign20560_e21288_d_n7, assign20560_e21288_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) && (var_guard360 == 0.0)) {
        let assign20560_e21285: f64 = (var_asrh * var_wsrh);
        let assign20560_e21286: f64 = (p.p833 * assign20560_e21285);
        (assign20560_e21286, (p.p833 * (var_asrh_dn5 * var_wsrh)), (p.p833 * (var_asrh_dn6 * var_wsrh)), (p.p833 * (var_asrh_dn7 * var_wsrh)), (p.p833 * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign20560_e21288;
        var_isrh_dn5 = assign20560_e21288_d_n5;
        var_isrh_dn6 = assign20560_e21288_d_n6;
        var_isrh_dn7 = assign20560_e21288_d_n7;
        var_isrh_dn8 = assign20560_e21288_d_n8;

        let assign20570_e21291: f64 = if p.p838 == 0.0 { 1.0 } else { 0.0 };
        var_guard363 = assign20570_e21291;

        let (assign20580_e21302, assign20580_e21302_d_n5, assign20580_e21302_d_n6, assign20580_e21302_d_n7, assign20580_e21302_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) && (var_guard363 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign20580_e21302;
        var_itat_dn5 = assign20580_e21302_d_n5;
        var_itat_dn6 = assign20580_e21302_d_n6;
        var_itat_dn7 = assign20580_e21302_d_n7;
        var_itat_dn8 = assign20580_e21302_d_n8;

        let (assign20590_e21320, assign20590_e21320_d_n5, assign20590_e21320_d_n6, assign20590_e21320_d_n7, assign20590_e21320_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) && (var_guard363 == 0.0)) {
        let assign20590_e21315: f64 = (var_wdep * var_one_minus_pbot);
        let assign20590_e21317: f64 = (assign20590_e21315 / var_vbi_minus_vjsrh);
        let assign20590_e21318: f64 = (var_btatpartbot * assign20590_e21317);
        (assign20590_e21318, (var_btatpartbot * ((var_wdep_dn5 * var_one_minus_pbot) / var_vbi_minus_vjsrh)), (var_btatpartbot * ((var_wdep_dn6 * var_one_minus_pbot) / var_vbi_minus_vjsrh)), (var_btatpartbot * ((var_wdep_dn7 * var_one_minus_pbot) / var_vbi_minus_vjsrh)), (var_btatpartbot * ((var_wdep_dn8 * var_one_minus_pbot) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign20590_e21320;
        var_btat_dn5 = assign20590_e21320_d_n5;
        var_btat_dn6 = assign20590_e21320_d_n6;
        var_btat_dn7 = assign20590_e21320_d_n7;
        var_btat_dn8 = assign20590_e21320_d_n8;

        let (assign20600_e21336, assign20600_e21336_d_n5, assign20600_e21336_d_n6, assign20600_e21336_d_n7, assign20600_e21336_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) && (var_guard363 == 0.0)) {
        let assign20600_e21332: f64 = (0.666666666666667 * var_atatbot);
        let assign20600_e21334: f64 = (assign20600_e21332 / var_btat);
        (assign20600_e21334, (-((assign20600_e21332 * var_btat_dn5) / (var_btat * var_btat))), (-((assign20600_e21332 * var_btat_dn6) / (var_btat * var_btat))), (-((assign20600_e21332 * var_btat_dn7) / (var_btat * var_btat))), (-((assign20600_e21332 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign20600_e21336;
        var_twoatatoverthreebtat_dn5 = assign20600_e21336_d_n5;
        var_twoatatoverthreebtat_dn6 = assign20600_e21336_d_n6;
        var_twoatatoverthreebtat_dn7 = assign20600_e21336_d_n7;
        var_twoatatoverthreebtat_dn8 = assign20600_e21336_d_n8;

        let (assign20610_e21350, assign20610_e21350_d_n5, assign20610_e21350_d_n6, assign20610_e21350_d_n7, assign20610_e21350_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) && (var_guard363 == 0.0)) {
        let assign20610_e21348: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign20610_e21348, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign20610_e21350;
        var_umaxbeforelimiting_dn5 = assign20610_e21350_d_n5;
        var_umaxbeforelimiting_dn6 = assign20610_e21350_d_n6;
        var_umaxbeforelimiting_dn7 = assign20610_e21350_d_n7;
        var_umaxbeforelimiting_dn8 = assign20610_e21350_d_n8;

        let (assign20620_e21371, assign20620_e21371_d_n5, assign20620_e21371_d_n6, assign20620_e21371_d_n7, assign20620_e21371_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) && (var_guard363 == 0.0)) {
        let assign20620_e21362: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign20620_e21365: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign20620_e21367: f64 = (assign20620_e21365 + 1.0);
        let assign20620_e21368: f64 = (assign20620_e21362 / assign20620_e21367);
        let assign20620_e21369: f64 = (assign20620_e21368).sqrt();
        (assign20620_e21369, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign20620_e21367) - (assign20620_e21362 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign20620_e21367 * assign20620_e21367)) / (2.0 * assign20620_e21369)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign20620_e21367) - (assign20620_e21362 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign20620_e21367 * assign20620_e21367)) / (2.0 * assign20620_e21369)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign20620_e21367) - (assign20620_e21362 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign20620_e21367 * assign20620_e21367)) / (2.0 * assign20620_e21369)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign20620_e21367) - (assign20620_e21362 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign20620_e21367 * assign20620_e21367)) / (2.0 * assign20620_e21369)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign20620_e21371;
        var_umax_dn5 = assign20620_e21371_d_n5;
        var_umax_dn6 = assign20620_e21371_d_n6;
        var_umax_dn7 = assign20620_e21371_d_n7;
        var_umax_dn8 = assign20620_e21371_d_n8;

        let (assign20630_e21384, assign20630_e21384_d_n5, assign20630_e21384_d_n6, assign20630_e21384_d_n7, assign20630_e21384_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) && (var_guard363 == 0.0)) {
        let assign20630_e21382: f64 = (var_umax).sqrt();
        (assign20630_e21382, (var_umax_dn5 / (2.0 * assign20630_e21382)), (var_umax_dn6 / (2.0 * assign20630_e21382)), (var_umax_dn7 / (2.0 * assign20630_e21382)), (var_umax_dn8 / (2.0 * assign20630_e21382)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign20630_e21384;
        var_sqrtumax_dn5 = assign20630_e21384_d_n5;
        var_sqrtumax_dn6 = assign20630_e21384_d_n6;
        var_sqrtumax_dn7 = assign20630_e21384_d_n7;
        var_sqrtumax_dn8 = assign20630_e21384_d_n8;

        let (assign20640_e21398, assign20640_e21398_d_n5, assign20640_e21398_d_n6, assign20640_e21398_d_n7, assign20640_e21398_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) && (var_guard363 == 0.0)) {
        let assign20640_e21396: f64 = (var_umax * var_sqrtumax);
        (assign20640_e21396, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign20640_e21398;
        var_umaxpoweronepointfive_dn5 = assign20640_e21398_d_n5;
        var_umaxpoweronepointfive_dn6 = assign20640_e21398_d_n6;
        var_umaxpoweronepointfive_dn7 = assign20640_e21398_d_n7;
        var_umaxpoweronepointfive_dn8 = assign20640_e21398_d_n8;

        let assign20650_e21400: f64 = (-p.p824);
        let assign20650_e21402: f64 = (assign20650_e21400 * var_one_over_one_minus_pbot);
        let assign20650_e21404: f64 = (-1.0);
        let assign20650_e21405: f64 = if assign20650_e21402 == assign20650_e21404 { 1.0 } else { 0.0 };
        var_guard364 = assign20650_e21405;

        let (assign20660_e21425, assign20660_e21425_d_n5, assign20660_e21425_d_n6, assign20660_e21425_d_n7, assign20660_e21425_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) && (var_guard363 == 0.0)) && (var_guard364 != 0.0)) {
        let assign20660_e21421: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign20660_e21422: f64 = (1.0 + assign20660_e21421);
        let assign20660_e21423: f64 = (1.0 / assign20660_e21422);
        (assign20660_e21423, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign20660_e21422 * assign20660_e21422))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign20660_e21422 * assign20660_e21422))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign20660_e21422 * assign20660_e21422))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign20660_e21422 * assign20660_e21422))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign20660_e21425;
        var_wgamma_dn5 = assign20660_e21425_d_n5;
        var_wgamma_dn6 = assign20660_e21425_d_n6;
        var_wgamma_dn7 = assign20660_e21425_d_n7;
        var_wgamma_dn8 = assign20660_e21425_d_n8;

        let (assign20670_e21449, assign20670_e21449_d_n5, assign20670_e21449_d_n6, assign20670_e21449_d_n7, assign20670_e21449_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) && (var_guard363 == 0.0)) && (var_guard364 == 0.0)) {
        let assign20670_e21441: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign20670_e21442: f64 = (1.0 + assign20670_e21441);
        let assign20670_e21444: f64 = (-p.p824);
        let assign20670_e21446: f64 = (assign20670_e21444 * var_one_over_one_minus_pbot);
        let assign20670_e21447: f64 = (assign20670_e21442).powf(assign20670_e21446);
        (assign20670_e21447, if 0.0 == 0.0 && ((assign20670_e21446) as f64).is_finite() && ((assign20670_e21446) as f64).fract() == 0.0 { if assign20670_e21446 == 0.0 { 0.0 } else { (assign20670_e21446 * ((assign20670_e21442).powf(assign20670_e21446 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign20670_e21447 * (assign20670_e21446 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign20670_e21442))) }, if 0.0 == 0.0 && ((assign20670_e21446) as f64).is_finite() && ((assign20670_e21446) as f64).fract() == 0.0 { if assign20670_e21446 == 0.0 { 0.0 } else { (assign20670_e21446 * ((assign20670_e21442).powf(assign20670_e21446 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign20670_e21447 * (assign20670_e21446 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign20670_e21442))) }, if 0.0 == 0.0 && ((assign20670_e21446) as f64).is_finite() && ((assign20670_e21446) as f64).fract() == 0.0 { if assign20670_e21446 == 0.0 { 0.0 } else { (assign20670_e21446 * ((assign20670_e21442).powf(assign20670_e21446 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign20670_e21447 * (assign20670_e21446 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign20670_e21442))) }, if 0.0 == 0.0 && ((assign20670_e21446) as f64).is_finite() && ((assign20670_e21446) as f64).fract() == 0.0 { if assign20670_e21446 == 0.0 { 0.0 } else { (assign20670_e21446 * ((assign20670_e21442).powf(assign20670_e21446 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign20670_e21447 * (assign20670_e21446 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign20670_e21442))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign20670_e21449;
        var_wgamma_dn5 = assign20670_e21449_d_n5;
        var_wgamma_dn6 = assign20670_e21449_d_n6;
        var_wgamma_dn7 = assign20670_e21449_d_n7;
        var_wgamma_dn8 = assign20670_e21449_d_n8;

        let (assign20680_e21467, assign20680_e21467_d_n5, assign20680_e21467_d_n6, assign20680_e21467_d_n7, assign20680_e21467_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) && (var_guard363 == 0.0)) {
        let assign20680_e21461: f64 = (var_wsrh * var_wgamma);
        let assign20680_e21464: f64 = (var_wsrh + var_wgamma);
        let assign20680_e21465: f64 = (assign20680_e21461 / assign20680_e21464);
        (assign20680_e21465, ((((var_wsrh * var_wgamma_dn5) * assign20680_e21464) - (assign20680_e21461 * var_wgamma_dn5)) / (assign20680_e21464 * assign20680_e21464)), ((((var_wsrh * var_wgamma_dn6) * assign20680_e21464) - (assign20680_e21461 * var_wgamma_dn6)) / (assign20680_e21464 * assign20680_e21464)), ((((var_wsrh * var_wgamma_dn7) * assign20680_e21464) - (assign20680_e21461 * var_wgamma_dn7)) / (assign20680_e21464 * assign20680_e21464)), ((((var_wsrh * var_wgamma_dn8) * assign20680_e21464) - (assign20680_e21461 * var_wgamma_dn8)) / (assign20680_e21464 * assign20680_e21464)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign20680_e21467;
        var_wtat_dn5 = assign20680_e21467_d_n5;
        var_wtat_dn6 = assign20680_e21467_d_n6;
        var_wtat_dn7 = assign20680_e21467_d_n7;
        var_wtat_dn8 = assign20680_e21467_d_n8;

        let (assign20690_e21484, assign20690_e21484_d_n5, assign20690_e21484_d_n6, assign20690_e21484_d_n7, assign20690_e21484_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) && (var_guard363 == 0.0)) {
        let assign20690_e21480: f64 = (var_btat / var_sqrtumax);
        let assign20690_e21481: f64 = (0.375 * assign20690_e21480);
        let assign20690_e21482: f64 = (assign20690_e21481).sqrt();
        (assign20690_e21482, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign20690_e21482)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign20690_e21482)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign20690_e21482)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign20690_e21482)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign20690_e21484;
        var_ktat_dn5 = assign20690_e21484_d_n5;
        var_ktat_dn6 = assign20690_e21484_d_n6;
        var_ktat_dn7 = assign20690_e21484_d_n7;
        var_ktat_dn8 = assign20690_e21484_d_n8;

        let (assign20700_e21502, assign20700_e21502_d_n5, assign20700_e21502_d_n6, assign20700_e21502_d_n7, assign20700_e21502_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) && (var_guard363 == 0.0)) {
        let assign20700_e21497: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign20700_e21498: f64 = (2.0 * assign20700_e21497);
        let assign20700_e21500: f64 = (assign20700_e21498 - var_umax);
        (assign20700_e21500, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign20700_e21502;
        var_ltat_dn5 = assign20700_e21502_d_n5;
        var_ltat_dn6 = assign20700_e21502_d_n6;
        var_ltat_dn7 = assign20700_e21502_d_n7;
        var_ltat_dn8 = assign20700_e21502_d_n8;

        let (assign20710_e21528, assign20710_e21528_d_n5, assign20710_e21528_d_n6, assign20710_e21528_d_n7, assign20710_e21528_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) && (var_guard363 == 0.0)) {
        let assign20710_e21514: f64 = (var_atatbot * var_twoatatoverthreebtat);
        let assign20710_e21516: f64 = (assign20710_e21514 * var_sqrtumax);
        let assign20710_e21519: f64 = (var_atatbot * var_umax);
        let assign20710_e21520: f64 = (assign20710_e21516 - assign20710_e21519);
        let assign20710_e21524: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign20710_e21525: f64 = (0.5 * assign20710_e21524);
        let assign20710_e21526: f64 = (assign20710_e21520 + assign20710_e21525);
        (assign20710_e21526, (((((var_atatbot * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign20710_e21514 * var_sqrtumax_dn5)) - (var_atatbot * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatbot * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign20710_e21514 * var_sqrtumax_dn6)) - (var_atatbot * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatbot * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign20710_e21514 * var_sqrtumax_dn7)) - (var_atatbot * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatbot * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign20710_e21514 * var_sqrtumax_dn8)) - (var_atatbot * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign20710_e21528;
        var_mtat_dn5 = assign20710_e21528_d_n5;
        var_mtat_dn6 = assign20710_e21528_d_n6;
        var_mtat_dn7 = assign20710_e21528_d_n7;
        var_mtat_dn8 = assign20710_e21528_d_n8;

        let (assign20720_e21544, assign20720_e21544_d_n5, assign20720_e21544_d_n6, assign20720_e21544_d_n7, assign20720_e21544_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) && (var_guard363 == 0.0)) {
        let assign20720_e21540: f64 = (var_ltat - 1.0);
        let assign20720_e21542: f64 = (assign20720_e21540 * var_ktat);
        (assign20720_e21542, ((var_ltat_dn5 * var_ktat) + (assign20720_e21540 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign20720_e21540 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign20720_e21540 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign20720_e21540 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign20720_e21544;
        var_xerfc_dn5 = assign20720_e21544_d_n5;
        var_xerfc_dn6 = assign20720_e21544_d_n6;
        var_xerfc_dn7 = assign20720_e21544_d_n7;
        var_xerfc_dn8 = assign20720_e21544_d_n8;

        let (assign20730_e21558, assign20730_e21558_d_n5, assign20730_e21558_d_n6, assign20730_e21558_d_n7, assign20730_e21558_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) && (var_guard363 == 0.0)) {
        let assign20730_e21556: f64 = (var_xerfc * var_xerfc);
        (assign20730_e21556, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign20730_e21558;
        var_ysq_dn5 = assign20730_e21558_d_n5;
        var_ysq_dn6 = assign20730_e21558_d_n6;
        var_ysq_dn7 = assign20730_e21558_d_n7;
        var_ysq_dn8 = assign20730_e21558_d_n8;

        let assign20740_e21561: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard365 = assign20740_e21561;

        let (assign20750_e21581, assign20750_e21581_d_n5, assign20750_e21581_d_n6, assign20750_e21581_d_n7, assign20750_e21581_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) && (var_guard363 == 0.0)) && (var_guard365 != 0.0)) {
        let assign20750_e21577: f64 = (var_perfc * var_xerfc);
        let assign20750_e21578: f64 = (1.0 + assign20750_e21577);
        let assign20750_e21579: f64 = (1.0 / assign20750_e21578);
        (assign20750_e21579, (-((var_perfc * var_xerfc_dn5) / (assign20750_e21578 * assign20750_e21578))), (-((var_perfc * var_xerfc_dn6) / (assign20750_e21578 * assign20750_e21578))), (-((var_perfc * var_xerfc_dn7) / (assign20750_e21578 * assign20750_e21578))), (-((var_perfc * var_xerfc_dn8) / (assign20750_e21578 * assign20750_e21578))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign20750_e21581;
        var_terfc_dn5 = assign20750_e21581_d_n5;
        var_terfc_dn6 = assign20750_e21581_d_n6;
        var_terfc_dn7 = assign20750_e21581_d_n7;
        var_terfc_dn8 = assign20750_e21581_d_n8;

        let (assign20760_e21602, assign20760_e21602_d_n5, assign20760_e21602_d_n6, assign20760_e21602_d_n7, assign20760_e21602_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) && (var_guard363 == 0.0)) && (var_guard365 == 0.0)) {
        let assign20760_e21598: f64 = (var_perfc * var_xerfc);
        let assign20760_e21599: f64 = (1.0 - assign20760_e21598);
        let assign20760_e21600: f64 = (1.0 / assign20760_e21599);
        (assign20760_e21600, (-((-(var_perfc * var_xerfc_dn5)) / (assign20760_e21599 * assign20760_e21599))), (-((-(var_perfc * var_xerfc_dn6)) / (assign20760_e21599 * assign20760_e21599))), (-((-(var_perfc * var_xerfc_dn7)) / (assign20760_e21599 * assign20760_e21599))), (-((-(var_perfc * var_xerfc_dn8)) / (assign20760_e21599 * assign20760_e21599))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign20760_e21602;
        var_terfc_dn5 = assign20760_e21602_d_n5;
        var_terfc_dn6 = assign20760_e21602_d_n6;
        var_terfc_dn7 = assign20760_e21602_d_n7;
        var_terfc_dn8 = assign20760_e21602_d_n8;

        let assign20770_e21604: f64 = (-var_ysq);
        let assign20770_e21606: f64 = (assign20770_e21604 + var_mtat);
        let assign20770_e21608: f64 = (-230.25850929940458);
        let assign20770_e21609: f64 = if assign20770_e21606 > assign20770_e21608 { 1.0 } else { 0.0 };
        var_guard366 = assign20770_e21609;

        let (assign20780_e21627, assign20780_e21627_d_n5, assign20780_e21627_d_n6, assign20780_e21627_d_n7, assign20780_e21627_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) && (var_guard363 == 0.0)) && (var_guard366 != 0.0)) {
        let assign20780_e21622: f64 = (-var_ysq);
        let assign20780_e21624: f64 = (assign20780_e21622 + var_mtat);
        let assign20780_e21625: f64 = (assign20780_e21624).exp();
        (assign20780_e21625, (assign20780_e21625 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign20780_e21625 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign20780_e21625 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign20780_e21625 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign20780_e21627;
        var_tmp_dn5 = assign20780_e21627_d_n5;
        var_tmp_dn6 = assign20780_e21627_d_n6;
        var_tmp_dn7 = assign20780_e21627_d_n7;
        var_tmp_dn8 = assign20780_e21627_d_n8;

        *var_asrh_slot = var_asrh;
        *var_asrh_dn5_slot = var_asrh_dn5;
        *var_asrh_dn6_slot = var_asrh_dn6;
        *var_asrh_dn7_slot = var_asrh_dn7;
        *var_asrh_dn8_slot = var_asrh_dn8;
        *var_btat_slot = var_btat;
        *var_btat_dn5_slot = var_btat_dn5;
        *var_btat_dn6_slot = var_btat_dn6;
        *var_btat_dn7_slot = var_btat_dn7;
        *var_btat_dn8_slot = var_btat_dn8;
        *var_dwsrh_slot = var_dwsrh;
        *var_guard359_slot = var_guard359;
        *var_guard360_slot = var_guard360;
        *var_guard361_slot = var_guard361;
        *var_guard362_slot = var_guard362;
        *var_guard363_slot = var_guard363;
        *var_guard364_slot = var_guard364;
        *var_guard365_slot = var_guard365;
        *var_guard366_slot = var_guard366;
        *var_id__blk213_slot = var_id__blk213;
        *var_ijunbot_slot = var_ijunbot;
        *var_ijunbot_dn5_slot = var_ijunbot_dn5;
        *var_ijunbot_dn6_slot = var_ijunbot_dn6;
        *var_ijunbot_dn7_slot = var_ijunbot_dn7;
        *var_ijunbot_dn8_slot = var_ijunbot_dn8;
        *var_isrh_slot = var_isrh;
        *var_isrh_dn5_slot = var_isrh_dn5;
        *var_isrh_dn6_slot = var_isrh_dn6;
        *var_isrh_dn7_slot = var_isrh_dn7;
        *var_isrh_dn8_slot = var_isrh_dn8;
        *var_itat_slot = var_itat;
        *var_itat_dn5_slot = var_itat_dn5;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_ktat_slot = var_ktat;
        *var_ktat_dn5_slot = var_ktat_dn5;
        *var_ktat_dn6_slot = var_ktat_dn6;
        *var_ktat_dn7_slot = var_ktat_dn7;
        *var_ktat_dn8_slot = var_ktat_dn8;
        *var_ltat_slot = var_ltat;
        *var_ltat_dn5_slot = var_ltat_dn5;
        *var_ltat_dn6_slot = var_ltat_dn6;
        *var_ltat_dn7_slot = var_ltat_dn7;
        *var_ltat_dn8_slot = var_ltat_dn8;
        *var_mtat_slot = var_mtat;
        *var_mtat_dn5_slot = var_mtat_dn5;
        *var_mtat_dn6_slot = var_mtat_dn6;
        *var_mtat_dn7_slot = var_mtat_dn7;
        *var_mtat_dn8_slot = var_mtat_dn8;
        *var_sqrtumax_slot = var_sqrtumax;
        *var_sqrtumax_dn5_slot = var_sqrtumax_dn5;
        *var_sqrtumax_dn6_slot = var_sqrtumax_dn6;
        *var_sqrtumax_dn7_slot = var_sqrtumax_dn7;
        *var_sqrtumax_dn8_slot = var_sqrtumax_dn8;
        *var_terfc_slot = var_terfc;
        *var_terfc_dn5_slot = var_terfc_dn5;
        *var_terfc_dn6_slot = var_terfc_dn6;
        *var_terfc_dn7_slot = var_terfc_dn7;
        *var_terfc_dn8_slot = var_terfc_dn8;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn5_slot = var_tmp_dn5;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_twoatatoverthreebtat_slot = var_twoatatoverthreebtat;
        *var_twoatatoverthreebtat_dn5_slot = var_twoatatoverthreebtat_dn5;
        *var_twoatatoverthreebtat_dn6_slot = var_twoatatoverthreebtat_dn6;
        *var_twoatatoverthreebtat_dn7_slot = var_twoatatoverthreebtat_dn7;
        *var_twoatatoverthreebtat_dn8_slot = var_twoatatoverthreebtat_dn8;
        *var_umax_slot = var_umax;
        *var_umax_dn5_slot = var_umax_dn5;
        *var_umax_dn6_slot = var_umax_dn6;
        *var_umax_dn7_slot = var_umax_dn7;
        *var_umax_dn8_slot = var_umax_dn8;
        *var_umaxbeforelimiting_slot = var_umaxbeforelimiting;
        *var_umaxbeforelimiting_dn5_slot = var_umaxbeforelimiting_dn5;
        *var_umaxbeforelimiting_dn6_slot = var_umaxbeforelimiting_dn6;
        *var_umaxbeforelimiting_dn7_slot = var_umaxbeforelimiting_dn7;
        *var_umaxbeforelimiting_dn8_slot = var_umaxbeforelimiting_dn8;
        *var_umaxpoweronepointfive_slot = var_umaxpoweronepointfive;
        *var_umaxpoweronepointfive_dn5_slot = var_umaxpoweronepointfive_dn5;
        *var_umaxpoweronepointfive_dn6_slot = var_umaxpoweronepointfive_dn6;
        *var_umaxpoweronepointfive_dn7_slot = var_umaxpoweronepointfive_dn7;
        *var_umaxpoweronepointfive_dn8_slot = var_umaxpoweronepointfive_dn8;
        *var_vav_slot = var_vav;
        *var_vbi_minus_vjsrh_slot = var_vbi_minus_vjsrh;
        *var_wdep_slot = var_wdep;
        *var_wdep_dn5_slot = var_wdep_dn5;
        *var_wdep_dn6_slot = var_wdep_dn6;
        *var_wdep_dn7_slot = var_wdep_dn7;
        *var_wdep_dn8_slot = var_wdep_dn8;
        *var_wgamma_slot = var_wgamma;
        *var_wgamma_dn5_slot = var_wgamma_dn5;
        *var_wgamma_dn6_slot = var_wgamma_dn6;
        *var_wgamma_dn7_slot = var_wgamma_dn7;
        *var_wgamma_dn8_slot = var_wgamma_dn8;
        *var_wsrh_slot = var_wsrh;
        *var_wsrhstep_slot = var_wsrhstep;
        *var_wtat_slot = var_wtat;
        *var_wtat_dn5_slot = var_wtat_dn5;
        *var_wtat_dn6_slot = var_wtat_dn6;
        *var_wtat_dn7_slot = var_wtat_dn7;
        *var_wtat_dn8_slot = var_wtat_dn8;
        *var_xerfc_slot = var_xerfc;
        *var_xerfc_dn5_slot = var_xerfc_dn5;
        *var_xerfc_dn6_slot = var_xerfc_dn6;
        *var_xerfc_dn7_slot = var_xerfc_dn7;
        *var_xerfc_dn8_slot = var_xerfc_dn8;
        *var_ysq_slot = var_ysq;
        *var_ysq_dn5_slot = var_ysq_dn5;
        *var_ysq_dn6_slot = var_ysq_dn6;
        *var_ysq_dn7_slot = var_ysq_dn7;
        *var_ysq_dn8_slot = var_ysq_dn8;
    }

    pub(super) fn stamp_transient_block_36(
        p: &Parameters,
        var_alphaav: f64,
        var_asrh: f64,
        var_asrh_dn5: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_atatbot: f64,
        var_berfc: f64,
        var_cerfc: f64,
        var_fbbtbot: f64,
        var_fstopbot: f64,
        var_guard176: f64,
        var_guard193: f64,
        var_guard359: f64,
        var_guard363: f64,
        var_guard366: f64,
        var_idmult: f64,
        var_idsatsti: f64,
        var_ktat: f64,
        var_ktat_dn5: f64,
        var_ktat_dn6: f64,
        var_ktat_dn7: f64,
        var_ktat_dn8: f64,
        var_lssource_i: f64,
        var_mtat: f64,
        var_mtat_dn5: f64,
        var_mtat_dn6: f64,
        var_mtat_dn7: f64,
        var_mtat_dn8: f64,
        var_one_over_one_minus_pbot: f64,
        var_slopebot: f64,
        var_terfc: f64,
        var_terfc_dn5: f64,
        var_terfc_dn6: f64,
        var_terfc_dn7: f64,
        var_terfc_dn8: f64,
        var_v3: f64,
        var_vav: f64,
        var_vbbt: f64,
        var_vbirbotinv: f64,
        var_vbisti: f64,
        var_vbrinvbot: f64,
        var_vjsrh: f64,
        var_wdepnulrinvbot: f64,
        var_wtat: f64,
        var_wtat_dn5: f64,
        var_wtat_dn6: f64,
        var_wtat_dn7: f64,
        var_wtat_dn8: f64,
        var_xerfc: f64,
        var_ysq: f64,
        var_ysq_dn5: f64,
        var_ysq_dn6: f64,
        var_ysq_dn7: f64,
        var_ysq_dn8: f64,
        var_erfcpos_slot: &mut f64,
        var_erfcpos_dn5_slot: &mut f64,
        var_erfcpos_dn6_slot: &mut f64,
        var_erfcpos_dn7_slot: &mut f64,
        var_erfcpos_dn8_slot: &mut f64,
        var_erfctimesexpmtat_slot: &mut f64,
        var_erfctimesexpmtat_dn5_slot: &mut f64,
        var_erfctimesexpmtat_dn6_slot: &mut f64,
        var_erfctimesexpmtat_dn7_slot: &mut f64,
        var_erfctimesexpmtat_dn8_slot: &mut f64,
        var_fbreakdown_slot: &mut f64,
        var_fbreakdown_dn5_slot: &mut f64,
        var_fbreakdown_dn6_slot: &mut f64,
        var_fbreakdown_dn7_slot: &mut f64,
        var_fbreakdown_dn8_slot: &mut f64,
        var_fmaxr_slot: &mut f64,
        var_fmaxr_dn5_slot: &mut f64,
        var_fmaxr_dn6_slot: &mut f64,
        var_fmaxr_dn7_slot: &mut f64,
        var_fmaxr_dn8_slot: &mut f64,
        var_gammamax_slot: &mut f64,
        var_gammamax_dn5_slot: &mut f64,
        var_gammamax_dn6_slot: &mut f64,
        var_gammamax_dn7_slot: &mut f64,
        var_gammamax_dn8_slot: &mut f64,
        var_guard367_slot: &mut f64,
        var_guard368_slot: &mut f64,
        var_guard369_slot: &mut f64,
        var_guard370_slot: &mut f64,
        var_guard371_slot: &mut f64,
        var_guard372_slot: &mut f64,
        var_guard373_slot: &mut f64,
        var_guard374_slot: &mut f64,
        var_guard375_slot: &mut f64,
        var_guard376_slot: &mut f64,
        var_guard377_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn5_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
        var_id__blk213_slot: &mut f64,
        var_ijunbot_slot: &mut f64,
        var_ijunbot_dn5_slot: &mut f64,
        var_ijunbot_dn6_slot: &mut f64,
        var_ijunbot_dn7_slot: &mut f64,
        var_ijunbot_dn8_slot: &mut f64,
        var_ijunsti_slot: &mut f64,
        var_ijunsti_dn5_slot: &mut f64,
        var_ijunsti_dn6_slot: &mut f64,
        var_ijunsti_dn7_slot: &mut f64,
        var_ijunsti_dn8_slot: &mut f64,
        var_isrh_slot: &mut f64,
        var_isrh_dn5_slot: &mut f64,
        var_isrh_dn6_slot: &mut f64,
        var_isrh_dn7_slot: &mut f64,
        var_isrh_dn8_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn5_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn5_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_vbi_minus_vjsrh_slot: &mut f64,
    ) {
        let mut var_erfcpos: f64 = *var_erfcpos_slot;
        let mut var_erfcpos_dn5: f64 = *var_erfcpos_dn5_slot;
        let mut var_erfcpos_dn6: f64 = *var_erfcpos_dn6_slot;
        let mut var_erfcpos_dn7: f64 = *var_erfcpos_dn7_slot;
        let mut var_erfcpos_dn8: f64 = *var_erfcpos_dn8_slot;
        let mut var_erfctimesexpmtat: f64 = *var_erfctimesexpmtat_slot;
        let mut var_erfctimesexpmtat_dn5: f64 = *var_erfctimesexpmtat_dn5_slot;
        let mut var_erfctimesexpmtat_dn6: f64 = *var_erfctimesexpmtat_dn6_slot;
        let mut var_erfctimesexpmtat_dn7: f64 = *var_erfctimesexpmtat_dn7_slot;
        let mut var_erfctimesexpmtat_dn8: f64 = *var_erfctimesexpmtat_dn8_slot;
        let mut var_fbreakdown: f64 = *var_fbreakdown_slot;
        let mut var_fbreakdown_dn5: f64 = *var_fbreakdown_dn5_slot;
        let mut var_fbreakdown_dn6: f64 = *var_fbreakdown_dn6_slot;
        let mut var_fbreakdown_dn7: f64 = *var_fbreakdown_dn7_slot;
        let mut var_fbreakdown_dn8: f64 = *var_fbreakdown_dn8_slot;
        let mut var_fmaxr: f64 = *var_fmaxr_slot;
        let mut var_fmaxr_dn5: f64 = *var_fmaxr_dn5_slot;
        let mut var_fmaxr_dn6: f64 = *var_fmaxr_dn6_slot;
        let mut var_fmaxr_dn7: f64 = *var_fmaxr_dn7_slot;
        let mut var_fmaxr_dn8: f64 = *var_fmaxr_dn8_slot;
        let mut var_gammamax: f64 = *var_gammamax_slot;
        let mut var_gammamax_dn5: f64 = *var_gammamax_dn5_slot;
        let mut var_gammamax_dn6: f64 = *var_gammamax_dn6_slot;
        let mut var_gammamax_dn7: f64 = *var_gammamax_dn7_slot;
        let mut var_gammamax_dn8: f64 = *var_gammamax_dn8_slot;
        let mut var_guard367: f64 = *var_guard367_slot;
        let mut var_guard368: f64 = *var_guard368_slot;
        let mut var_guard369: f64 = *var_guard369_slot;
        let mut var_guard370: f64 = *var_guard370_slot;
        let mut var_guard371: f64 = *var_guard371_slot;
        let mut var_guard372: f64 = *var_guard372_slot;
        let mut var_guard373: f64 = *var_guard373_slot;
        let mut var_guard374: f64 = *var_guard374_slot;
        let mut var_guard375: f64 = *var_guard375_slot;
        let mut var_guard376: f64 = *var_guard376_slot;
        let mut var_guard377: f64 = *var_guard377_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn5: f64 = *var_ibbt_dn5_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
        let mut var_id__blk213: f64 = *var_id__blk213_slot;
        let mut var_ijunbot: f64 = *var_ijunbot_slot;
        let mut var_ijunbot_dn5: f64 = *var_ijunbot_dn5_slot;
        let mut var_ijunbot_dn6: f64 = *var_ijunbot_dn6_slot;
        let mut var_ijunbot_dn7: f64 = *var_ijunbot_dn7_slot;
        let mut var_ijunbot_dn8: f64 = *var_ijunbot_dn8_slot;
        let mut var_ijunsti: f64 = *var_ijunsti_slot;
        let mut var_ijunsti_dn5: f64 = *var_ijunsti_dn5_slot;
        let mut var_ijunsti_dn6: f64 = *var_ijunsti_dn6_slot;
        let mut var_ijunsti_dn7: f64 = *var_ijunsti_dn7_slot;
        let mut var_ijunsti_dn8: f64 = *var_ijunsti_dn8_slot;
        let mut var_isrh: f64 = *var_isrh_slot;
        let mut var_isrh_dn5: f64 = *var_isrh_dn5_slot;
        let mut var_isrh_dn6: f64 = *var_isrh_dn6_slot;
        let mut var_isrh_dn7: f64 = *var_isrh_dn7_slot;
        let mut var_isrh_dn8: f64 = *var_isrh_dn8_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn5: f64 = *var_itat_dn5_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn5: f64 = *var_tmp_dn5_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_vbi_minus_vjsrh: f64 = *var_vbi_minus_vjsrh_slot;

        let (assign20790_e21676, assign20790_e21676_d_n5, assign20790_e21676_d_n6, assign20790_e21676_d_n7, assign20790_e21676_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) && (var_guard363 == 0.0)) && (var_guard366 == 0.0)) {
        let assign20790_e21643: f64 = (-230.25850929940458);
        let assign20790_e21645: f64 = (-var_ysq);
        let assign20790_e21647: f64 = (assign20790_e21645 + var_mtat);
        let assign20790_e21648: f64 = (assign20790_e21643 - assign20790_e21647);
        let assign20790_e21652: f64 = (-230.25850929940458);
        let assign20790_e21654: f64 = (-var_ysq);
        let assign20790_e21656: f64 = (assign20790_e21654 + var_mtat);
        let assign20790_e21657: f64 = (assign20790_e21652 - assign20790_e21656);
        let assign20790_e21660: f64 = (-230.25850929940458);
        let assign20790_e21662: f64 = (-var_ysq);
        let assign20790_e21664: f64 = (assign20790_e21662 + var_mtat);
        let assign20790_e21665: f64 = (assign20790_e21660 - assign20790_e21664);
        let assign20790_e21667: f64 = (assign20790_e21665 * 0.3333333333333333);
        let assign20790_e21668: f64 = (1.0 + assign20790_e21667);
        let assign20790_e21669: f64 = (assign20790_e21657 * assign20790_e21668);
        let assign20790_e21670: f64 = (0.5 * assign20790_e21669);
        let assign20790_e21671: f64 = (1.0 + assign20790_e21670);
        let assign20790_e21672: f64 = (assign20790_e21648 * assign20790_e21671);
        let assign20790_e21673: f64 = (1.0 + assign20790_e21672);
        let assign20790_e21674: f64 = (1e-100 / assign20790_e21673);
        (assign20790_e21674, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign20790_e21671) + (assign20790_e21648 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign20790_e21668) + (assign20790_e21657 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign20790_e21673 * assign20790_e21673))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign20790_e21671) + (assign20790_e21648 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign20790_e21668) + (assign20790_e21657 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign20790_e21673 * assign20790_e21673))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign20790_e21671) + (assign20790_e21648 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign20790_e21668) + (assign20790_e21657 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign20790_e21673 * assign20790_e21673))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign20790_e21671) + (assign20790_e21648 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign20790_e21668) + (assign20790_e21657 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign20790_e21673 * assign20790_e21673))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign20790_e21676;
        var_tmp_dn5 = assign20790_e21676_d_n5;
        var_tmp_dn6 = assign20790_e21676_d_n6;
        var_tmp_dn7 = assign20790_e21676_d_n7;
        var_tmp_dn8 = assign20790_e21676_d_n8;

        let (assign20800_e21706, assign20800_e21706_d_n5, assign20800_e21706_d_n6, assign20800_e21706_d_n7, assign20800_e21706_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) && (var_guard363 == 0.0)) {
        let assign20800_e21688: f64 = (0.29214664 * var_terfc);
        let assign20800_e21692: f64 = (var_terfc * var_terfc);
        let assign20800_e21693: f64 = (var_berfc * assign20800_e21692);
        let assign20800_e21694: f64 = (assign20800_e21688 + assign20800_e21693);
        let assign20800_e21698: f64 = (var_terfc * var_terfc);
        let assign20800_e21700: f64 = (assign20800_e21698 * var_terfc);
        let assign20800_e21701: f64 = (var_cerfc * assign20800_e21700);
        let assign20800_e21702: f64 = (assign20800_e21694 + assign20800_e21701);
        let assign20800_e21704: f64 = (assign20800_e21702 * var_tmp);
        (assign20800_e21704, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign20800_e21698 * var_terfc_dn5)))) * var_tmp) + (assign20800_e21702 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign20800_e21698 * var_terfc_dn6)))) * var_tmp) + (assign20800_e21702 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign20800_e21698 * var_terfc_dn7)))) * var_tmp) + (assign20800_e21702 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign20800_e21698 * var_terfc_dn8)))) * var_tmp) + (assign20800_e21702 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign20800_e21706;
        var_erfcpos_dn5 = assign20800_e21706_d_n5;
        var_erfcpos_dn6 = assign20800_e21706_d_n6;
        var_erfcpos_dn7 = assign20800_e21706_d_n7;
        var_erfcpos_dn8 = assign20800_e21706_d_n8;

        let assign20810_e21709: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard367 = assign20810_e21709;

        let (assign20820_e21723, assign20820_e21723_d_n5, assign20820_e21723_d_n6, assign20820_e21723_d_n7, assign20820_e21723_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) && (var_guard363 == 0.0)) && (var_guard367 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign20820_e21723;
        var_erfctimesexpmtat_dn5 = assign20820_e21723_d_n5;
        var_erfctimesexpmtat_dn6 = assign20820_e21723_d_n6;
        var_erfctimesexpmtat_dn7 = assign20820_e21723_d_n7;
        var_erfctimesexpmtat_dn8 = assign20820_e21723_d_n8;

        let assign20830_e21726: f64 = (-230.25850929940458);
        let assign20830_e21727: f64 = if var_mtat > assign20830_e21726 { 1.0 } else { 0.0 };
        var_guard368 = assign20830_e21727;

        let (assign20840_e21745, assign20840_e21745_d_n5, assign20840_e21745_d_n6, assign20840_e21745_d_n7, assign20840_e21745_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) && (var_guard363 == 0.0)) && (var_guard367 == 0.0)) && (var_guard368 != 0.0)) {
        let assign20840_e21743: f64 = (var_mtat).exp();
        (assign20840_e21743, (assign20840_e21743 * var_mtat_dn5), (assign20840_e21743 * var_mtat_dn6), (assign20840_e21743 * var_mtat_dn7), (assign20840_e21743 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign20840_e21745;
        var_tmp_dn5 = assign20840_e21745_d_n5;
        var_tmp_dn6 = assign20840_e21745_d_n6;
        var_tmp_dn7 = assign20840_e21745_d_n7;
        var_tmp_dn8 = assign20840_e21745_d_n8;

        let (assign20850_e21788, assign20850_e21788_d_n5, assign20850_e21788_d_n6, assign20850_e21788_d_n7, assign20850_e21788_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) && (var_guard363 == 0.0)) && (var_guard367 == 0.0)) && (var_guard368 == 0.0)) {
        let assign20850_e21764: f64 = (-230.25850929940458);
        let assign20850_e21766: f64 = (assign20850_e21764 - var_mtat);
        let assign20850_e21770: f64 = (-230.25850929940458);
        let assign20850_e21772: f64 = (assign20850_e21770 - var_mtat);
        let assign20850_e21775: f64 = (-230.25850929940458);
        let assign20850_e21777: f64 = (assign20850_e21775 - var_mtat);
        let assign20850_e21779: f64 = (assign20850_e21777 * 0.3333333333333333);
        let assign20850_e21780: f64 = (1.0 + assign20850_e21779);
        let assign20850_e21781: f64 = (assign20850_e21772 * assign20850_e21780);
        let assign20850_e21782: f64 = (0.5 * assign20850_e21781);
        let assign20850_e21783: f64 = (1.0 + assign20850_e21782);
        let assign20850_e21784: f64 = (assign20850_e21766 * assign20850_e21783);
        let assign20850_e21785: f64 = (1.0 + assign20850_e21784);
        let assign20850_e21786: f64 = (1e-100 / assign20850_e21785);
        (assign20850_e21786, (-((1e-100 * (((-var_mtat_dn5) * assign20850_e21783) + (assign20850_e21766 * (0.5 * (((-var_mtat_dn5) * assign20850_e21780) + (assign20850_e21772 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign20850_e21785 * assign20850_e21785))), (-((1e-100 * (((-var_mtat_dn6) * assign20850_e21783) + (assign20850_e21766 * (0.5 * (((-var_mtat_dn6) * assign20850_e21780) + (assign20850_e21772 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign20850_e21785 * assign20850_e21785))), (-((1e-100 * (((-var_mtat_dn7) * assign20850_e21783) + (assign20850_e21766 * (0.5 * (((-var_mtat_dn7) * assign20850_e21780) + (assign20850_e21772 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign20850_e21785 * assign20850_e21785))), (-((1e-100 * (((-var_mtat_dn8) * assign20850_e21783) + (assign20850_e21766 * (0.5 * (((-var_mtat_dn8) * assign20850_e21780) + (assign20850_e21772 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign20850_e21785 * assign20850_e21785))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign20850_e21788;
        var_tmp_dn5 = assign20850_e21788_d_n5;
        var_tmp_dn6 = assign20850_e21788_d_n6;
        var_tmp_dn7 = assign20850_e21788_d_n7;
        var_tmp_dn8 = assign20850_e21788_d_n8;

        let (assign20860_e21807, assign20860_e21807_d_n5, assign20860_e21807_d_n6, assign20860_e21807_d_n7, assign20860_e21807_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) && (var_guard363 == 0.0)) && (var_guard367 == 0.0)) {
        let assign20860_e21803: f64 = (2.0 * var_tmp);
        let assign20860_e21805: f64 = (assign20860_e21803 - var_erfcpos);
        (assign20860_e21805, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign20860_e21807;
        var_erfctimesexpmtat_dn5 = assign20860_e21807_d_n5;
        var_erfctimesexpmtat_dn6 = assign20860_e21807_d_n6;
        var_erfctimesexpmtat_dn7 = assign20860_e21807_d_n7;
        var_erfctimesexpmtat_dn8 = assign20860_e21807_d_n8;

        let (assign20870_e21827, assign20870_e21827_d_n5, assign20870_e21827_d_n6, assign20870_e21827_d_n7, assign20870_e21827_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) && (var_guard363 == 0.0)) {
        let assign20870_e21819: f64 = (1.772453850905516 * 0.5);
        let assign20870_e21822: f64 = (var_atatbot * var_erfctimesexpmtat);
        let assign20870_e21824: f64 = (assign20870_e21822 / var_ktat);
        let assign20870_e21825: f64 = (assign20870_e21819 * assign20870_e21824);
        (assign20870_e21825, (assign20870_e21819 * ((((var_atatbot * var_erfctimesexpmtat_dn5) * var_ktat) - (assign20870_e21822 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign20870_e21819 * ((((var_atatbot * var_erfctimesexpmtat_dn6) * var_ktat) - (assign20870_e21822 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign20870_e21819 * ((((var_atatbot * var_erfctimesexpmtat_dn7) * var_ktat) - (assign20870_e21822 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign20870_e21819 * ((((var_atatbot * var_erfctimesexpmtat_dn8) * var_ktat) - (assign20870_e21822 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign20870_e21827;
        var_gammamax_dn5 = assign20870_e21827_d_n5;
        var_gammamax_dn6 = assign20870_e21827_d_n6;
        var_gammamax_dn7 = assign20870_e21827_d_n7;
        var_gammamax_dn8 = assign20870_e21827_d_n8;

        let (assign20880_e21845, assign20880_e21845_d_n5, assign20880_e21845_d_n6, assign20880_e21845_d_n7, assign20880_e21845_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) && (var_guard363 == 0.0)) {
        let assign20880_e21840: f64 = (var_asrh * var_gammamax);
        let assign20880_e21842: f64 = (assign20880_e21840 * var_wtat);
        let assign20880_e21843: f64 = (p.p838 * assign20880_e21842);
        (assign20880_e21843, (p.p838 * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign20880_e21840 * var_wtat_dn5))), (p.p838 * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign20880_e21840 * var_wtat_dn6))), (p.p838 * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign20880_e21840 * var_wtat_dn7))), (p.p838 * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign20880_e21840 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign20880_e21845;
        var_itat_dn5 = assign20880_e21845_d_n5;
        var_itat_dn6 = assign20880_e21845_d_n6;
        var_itat_dn7 = assign20880_e21845_d_n7;
        var_itat_dn8 = assign20880_e21845_d_n8;

        let assign20890_e21848: f64 = if p.p844 == 0.0 { 1.0 } else { 0.0 };
        var_guard369 = assign20890_e21848;

        let (assign20900_e21859, assign20900_e21859_d_n5, assign20900_e21859_d_n6, assign20900_e21859_d_n7, assign20900_e21859_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) && (var_guard369 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign20900_e21859;
        var_ibbt_dn5 = assign20900_e21859_d_n5;
        var_ibbt_dn6 = assign20900_e21859_d_n6;
        var_ibbt_dn7 = assign20900_e21859_d_n7;
        var_ibbt_dn8 = assign20900_e21859_d_n8;

        let assign20910_e21862: f64 = if p.p824 == 0.5 { 1.0 } else { 0.0 };
        var_guard370 = assign20910_e21862;

        let (assign20920_e21881, assign20920_e21881_d_n5, assign20920_e21881_d_n6, assign20920_e21881_d_n7, assign20920_e21881_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) && (var_guard369 == 0.0)) && (var_guard370 != 0.0)) {
        let assign20920_e21876: f64 = (p.p821 - var_vbbt);
        let assign20920_e21878: f64 = (assign20920_e21876 * var_vbirbotinv);
        let assign20920_e21879: f64 = (assign20920_e21878).sqrt();
        (assign20920_e21879, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign20920_e21881;
        var_tmp_dn5 = assign20920_e21881_d_n5;
        var_tmp_dn6 = assign20920_e21881_d_n6;
        var_tmp_dn7 = assign20920_e21881_d_n7;
        var_tmp_dn8 = assign20920_e21881_d_n8;

        let (assign20930_e21902, assign20930_e21902_d_n5, assign20930_e21902_d_n6, assign20930_e21902_d_n7, assign20930_e21902_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) && (var_guard369 == 0.0)) && (var_guard370 == 0.0)) {
        let assign20930_e21896: f64 = (p.p821 - var_vbbt);
        let assign20930_e21898: f64 = (assign20930_e21896 * var_vbirbotinv);
        let assign20930_e21900: f64 = (assign20930_e21898).powf(p.p824);
        (assign20930_e21900, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign20930_e21902;
        var_tmp_dn5 = assign20930_e21902_d_n5;
        var_tmp_dn6 = assign20930_e21902_d_n6;
        var_tmp_dn7 = assign20930_e21902_d_n7;
        var_tmp_dn8 = assign20930_e21902_d_n8;

        let (assign20940_e21922, assign20940_e21922_d_n5, assign20940_e21922_d_n6, assign20940_e21922_d_n7, assign20940_e21922_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) && (var_guard369 == 0.0)) {
        let assign20940_e21915: f64 = (p.p821 - var_vbbt);
        let assign20940_e21917: f64 = (assign20940_e21915 * var_wdepnulrinvbot);
        let assign20940_e21919: f64 = (assign20940_e21917 / var_tmp);
        let assign20940_e21920: f64 = (var_one_over_one_minus_pbot * assign20940_e21919);
        (assign20940_e21920, (var_one_over_one_minus_pbot * (-((assign20940_e21917 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot * (-((assign20940_e21917 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot * (-((assign20940_e21917 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot * (-((assign20940_e21917 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign20940_e21922;
        var_fmaxr_dn5 = assign20940_e21922_d_n5;
        var_fmaxr_dn6 = assign20940_e21922_d_n6;
        var_fmaxr_dn7 = assign20940_e21922_d_n7;
        var_fmaxr_dn8 = assign20940_e21922_d_n8;

        let assign20950_e21924: f64 = (-var_fbbtbot);
        let assign20950_e21926: f64 = (assign20950_e21924 / var_fmaxr);
        let assign20950_e21927: f64 = (assign20950_e21926).abs();
        let assign20950_e21929: f64 = if assign20950_e21927 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard371 = assign20950_e21929;

        let (assign20960_e21947, assign20960_e21947_d_n5, assign20960_e21947_d_n6, assign20960_e21947_d_n7, assign20960_e21947_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) && (var_guard369 == 0.0)) && (var_guard371 != 0.0)) {
        let assign20960_e21942: f64 = (-var_fbbtbot);
        let assign20960_e21944: f64 = (assign20960_e21942 / var_fmaxr);
        let assign20960_e21945: f64 = (assign20960_e21944).exp();
        (assign20960_e21945, (assign20960_e21945 * (-((assign20960_e21942 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))), (assign20960_e21945 * (-((assign20960_e21942 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign20960_e21945 * (-((assign20960_e21942 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign20960_e21945 * (-((assign20960_e21942 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign20960_e21947;
        var_tmp_dn5 = assign20960_e21947_d_n5;
        var_tmp_dn6 = assign20960_e21947_d_n6;
        var_tmp_dn7 = assign20960_e21947_d_n7;
        var_tmp_dn8 = assign20960_e21947_d_n8;

        let assign20970_e21949: f64 = (-var_fbbtbot);
        let assign20970_e21951: f64 = (assign20970_e21949 / var_fmaxr);
        let assign20970_e21953: f64 = if assign20970_e21951 < 0.0 { 1.0 } else { 0.0 };
        var_guard372 = assign20970_e21953;

        let (assign20980_e22004, assign20980_e22004_d_n5, assign20980_e22004_d_n6, assign20980_e22004_d_n7, assign20980_e22004_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) && (var_guard369 == 0.0)) && (var_guard371 == 0.0)) && (var_guard372 != 0.0)) {
        let assign20980_e21971: f64 = (-230.25850929940458);
        let assign20980_e21973: f64 = (-var_fbbtbot);
        let assign20980_e21975: f64 = (assign20980_e21973 / var_fmaxr);
        let assign20980_e21976: f64 = (assign20980_e21971 - assign20980_e21975);
        let assign20980_e21980: f64 = (-230.25850929940458);
        let assign20980_e21982: f64 = (-var_fbbtbot);
        let assign20980_e21984: f64 = (assign20980_e21982 / var_fmaxr);
        let assign20980_e21985: f64 = (assign20980_e21980 - assign20980_e21984);
        let assign20980_e21988: f64 = (-230.25850929940458);
        let assign20980_e21990: f64 = (-var_fbbtbot);
        let assign20980_e21992: f64 = (assign20980_e21990 / var_fmaxr);
        let assign20980_e21993: f64 = (assign20980_e21988 - assign20980_e21992);
        let assign20980_e21995: f64 = (assign20980_e21993 * 0.3333333333333333);
        let assign20980_e21996: f64 = (1.0 + assign20980_e21995);
        let assign20980_e21997: f64 = (assign20980_e21985 * assign20980_e21996);
        let assign20980_e21998: f64 = (0.5 * assign20980_e21997);
        let assign20980_e21999: f64 = (1.0 + assign20980_e21998);
        let assign20980_e22000: f64 = (assign20980_e21976 * assign20980_e21999);
        let assign20980_e22001: f64 = (1.0 + assign20980_e22000);
        let assign20980_e22002: f64 = (1e-100 / assign20980_e22001);
        (assign20980_e22002, (-((1e-100 * (((-(-((assign20980_e21973 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign20980_e21999) + (assign20980_e21976 * (0.5 * (((-(-((assign20980_e21982 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign20980_e21996) + (assign20980_e21985 * ((-(-((assign20980_e21990 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign20980_e22001 * assign20980_e22001))), (-((1e-100 * (((-(-((assign20980_e21973 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign20980_e21999) + (assign20980_e21976 * (0.5 * (((-(-((assign20980_e21982 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign20980_e21996) + (assign20980_e21985 * ((-(-((assign20980_e21990 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign20980_e22001 * assign20980_e22001))), (-((1e-100 * (((-(-((assign20980_e21973 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign20980_e21999) + (assign20980_e21976 * (0.5 * (((-(-((assign20980_e21982 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign20980_e21996) + (assign20980_e21985 * ((-(-((assign20980_e21990 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign20980_e22001 * assign20980_e22001))), (-((1e-100 * (((-(-((assign20980_e21973 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign20980_e21999) + (assign20980_e21976 * (0.5 * (((-(-((assign20980_e21982 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign20980_e21996) + (assign20980_e21985 * ((-(-((assign20980_e21990 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign20980_e22001 * assign20980_e22001))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign20980_e22004;
        var_tmp_dn5 = assign20980_e22004_d_n5;
        var_tmp_dn6 = assign20980_e22004_d_n6;
        var_tmp_dn7 = assign20980_e22004_d_n7;
        var_tmp_dn8 = assign20980_e22004_d_n8;

        let (assign20990_e22053, assign20990_e22053_d_n5, assign20990_e22053_d_n6, assign20990_e22053_d_n7, assign20990_e22053_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) && (var_guard369 == 0.0)) && (var_guard371 == 0.0)) && (var_guard372 == 0.0)) {
        let assign20990_e22023: f64 = (-var_fbbtbot);
        let assign20990_e22025: f64 = (assign20990_e22023 / var_fmaxr);
        let assign20990_e22027: f64 = (assign20990_e22025 - 230.25850929940458);
        let assign20990_e22031: f64 = (-var_fbbtbot);
        let assign20990_e22033: f64 = (assign20990_e22031 / var_fmaxr);
        let assign20990_e22035: f64 = (assign20990_e22033 - 230.25850929940458);
        let assign20990_e22038: f64 = (-var_fbbtbot);
        let assign20990_e22040: f64 = (assign20990_e22038 / var_fmaxr);
        let assign20990_e22042: f64 = (assign20990_e22040 - 230.25850929940458);
        let assign20990_e22044: f64 = (assign20990_e22042 * 0.3333333333333333);
        let assign20990_e22045: f64 = (1.0 + assign20990_e22044);
        let assign20990_e22046: f64 = (assign20990_e22035 * assign20990_e22045);
        let assign20990_e22047: f64 = (0.5 * assign20990_e22046);
        let assign20990_e22048: f64 = (1.0 + assign20990_e22047);
        let assign20990_e22049: f64 = (assign20990_e22027 * assign20990_e22048);
        let assign20990_e22050: f64 = (1.0 + assign20990_e22049);
        let assign20990_e22051: f64 = (1e100 * assign20990_e22050);
        (assign20990_e22051, (1e100 * (((-((assign20990_e22023 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign20990_e22048) + (assign20990_e22027 * (0.5 * (((-((assign20990_e22031 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign20990_e22045) + (assign20990_e22035 * ((-((assign20990_e22038 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign20990_e22023 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign20990_e22048) + (assign20990_e22027 * (0.5 * (((-((assign20990_e22031 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign20990_e22045) + (assign20990_e22035 * ((-((assign20990_e22038 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign20990_e22023 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign20990_e22048) + (assign20990_e22027 * (0.5 * (((-((assign20990_e22031 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign20990_e22045) + (assign20990_e22035 * ((-((assign20990_e22038 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign20990_e22023 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign20990_e22048) + (assign20990_e22027 * (0.5 * (((-((assign20990_e22031 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign20990_e22045) + (assign20990_e22035 * ((-((assign20990_e22038 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign20990_e22053;
        var_tmp_dn5 = assign20990_e22053_d_n5;
        var_tmp_dn6 = assign20990_e22053_d_n6;
        var_tmp_dn7 = assign20990_e22053_d_n7;
        var_tmp_dn8 = assign20990_e22053_d_n8;

        let (assign21000_e22073, assign21000_e22073_d_n5, assign21000_e22073_d_n6, assign21000_e22073_d_n7, assign21000_e22073_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) && (var_guard369 == 0.0)) {
        let assign21000_e22066: f64 = (var_v3 * var_fmaxr);
        let assign21000_e22068: f64 = (assign21000_e22066 * var_fmaxr);
        let assign21000_e22070: f64 = (assign21000_e22068 * var_tmp);
        let assign21000_e22071: f64 = (p.p844 * assign21000_e22070);
        (assign21000_e22071, (p.p844 * (((((var_v3 * var_fmaxr_dn5) * var_fmaxr) + (assign21000_e22066 * var_fmaxr_dn5)) * var_tmp) + (assign21000_e22068 * var_tmp_dn5))), (p.p844 * (((((var_v3 * var_fmaxr_dn6) * var_fmaxr) + (assign21000_e22066 * var_fmaxr_dn6)) * var_tmp) + (assign21000_e22068 * var_tmp_dn6))), (p.p844 * (((((var_v3 * var_fmaxr_dn7) * var_fmaxr) + (assign21000_e22066 * var_fmaxr_dn7)) * var_tmp) + (assign21000_e22068 * var_tmp_dn7))), (p.p844 * (((((var_v3 * var_fmaxr_dn8) * var_fmaxr) + (assign21000_e22066 * var_fmaxr_dn8)) * var_tmp) + (assign21000_e22068 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign21000_e22073;
        var_ibbt_dn5 = assign21000_e22073_d_n5;
        var_ibbt_dn6 = assign21000_e22073_d_n6;
        var_ibbt_dn7 = assign21000_e22073_d_n7;
        var_ibbt_dn8 = assign21000_e22073_d_n8;

        let assign21010_e22076: f64 = if p.p853 > 1000.0 { 1.0 } else { 0.0 };
        var_guard373 = assign21010_e22076;

        let (assign21020_e22087, assign21020_e22087_d_n5, assign21020_e22087_d_n6, assign21020_e22087_d_n7, assign21020_e22087_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) && (var_guard373 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign21020_e22087;
        var_fbreakdown_dn5 = assign21020_e22087_d_n5;
        var_fbreakdown_dn6 = assign21020_e22087_d_n6;
        var_fbreakdown_dn7 = assign21020_e22087_d_n7;
        var_fbreakdown_dn8 = assign21020_e22087_d_n8;

        let assign21030_e22090: f64 = (-var_alphaav);
        let assign21030_e22092: f64 = (assign21030_e22090 * p.p853);
        let assign21030_e22093: f64 = if var_vav > assign21030_e22092 { 1.0 } else { 0.0 };
        var_guard374 = assign21030_e22093;

        let assign21040_e22096: f64 = if p.p856 == 4.0 { 1.0 } else { 0.0 };
        var_guard375 = assign21040_e22096;

        let (assign21050_e22126, assign21050_e22126_d_n5, assign21050_e22126_d_n6, assign21050_e22126_d_n7, assign21050_e22126_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) && (var_guard373 == 0.0)) && (var_guard374 != 0.0)) && (var_guard375 != 0.0)) {
        let assign21050_e22112: f64 = (var_vav * var_vbrinvbot);
        let assign21050_e22115: f64 = (var_vav * var_vbrinvbot);
        let assign21050_e22116: f64 = (assign21050_e22112 * assign21050_e22115);
        let assign21050_e22119: f64 = (var_vav * var_vbrinvbot);
        let assign21050_e22120: f64 = (assign21050_e22116 * assign21050_e22119);
        let assign21050_e22123: f64 = (var_vav * var_vbrinvbot);
        let assign21050_e22124: f64 = (assign21050_e22120 * assign21050_e22123);
        (assign21050_e22124, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign21050_e22126;
        var_tmp_dn5 = assign21050_e22126_d_n5;
        var_tmp_dn6 = assign21050_e22126_d_n6;
        var_tmp_dn7 = assign21050_e22126_d_n7;
        var_tmp_dn8 = assign21050_e22126_d_n8;

        let (assign21060_e22148, assign21060_e22148_d_n5, assign21060_e22148_d_n6, assign21060_e22148_d_n7, assign21060_e22148_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) && (var_guard373 == 0.0)) && (var_guard374 != 0.0)) && (var_guard375 == 0.0)) {
        let assign21060_e22143: f64 = (var_vav * var_vbrinvbot);
        let assign21060_e22144: f64 = (assign21060_e22143).abs();
        let assign21060_e22146: f64 = (assign21060_e22144).powf(p.p856);
        (assign21060_e22146, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign21060_e22148;
        var_tmp_dn5 = assign21060_e22148_d_n5;
        var_tmp_dn6 = assign21060_e22148_d_n6;
        var_tmp_dn7 = assign21060_e22148_d_n7;
        var_tmp_dn8 = assign21060_e22148_d_n8;

        let (assign21070_e22166, assign21070_e22166_d_n5, assign21070_e22166_d_n6, assign21070_e22166_d_n7, assign21070_e22166_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) && (var_guard373 == 0.0)) && (var_guard374 != 0.0)) {
        let assign21070_e22163: f64 = (1.0 - var_tmp);
        let assign21070_e22164: f64 = (1.0 / assign21070_e22163);
        (assign21070_e22164, (-((-var_tmp_dn5) / (assign21070_e22163 * assign21070_e22163))), (-((-var_tmp_dn6) / (assign21070_e22163 * assign21070_e22163))), (-((-var_tmp_dn7) / (assign21070_e22163 * assign21070_e22163))), (-((-var_tmp_dn8) / (assign21070_e22163 * assign21070_e22163))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign21070_e22166;
        var_fbreakdown_dn5 = assign21070_e22166_d_n5;
        var_fbreakdown_dn6 = assign21070_e22166_d_n6;
        var_fbreakdown_dn7 = assign21070_e22166_d_n7;
        var_fbreakdown_dn8 = assign21070_e22166_d_n8;

        let (assign21080_e22189, assign21080_e22189_d_n5, assign21080_e22189_d_n6, assign21080_e22189_d_n7, assign21080_e22189_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) && (var_guard373 == 0.0)) && (var_guard374 == 0.0)) {
        let assign21080_e22183: f64 = (var_alphaav * p.p853);
        let assign21080_e22184: f64 = (var_vav + assign21080_e22183);
        let assign21080_e22186: f64 = (assign21080_e22184 * var_slopebot);
        let assign21080_e22187: f64 = (var_fstopbot + assign21080_e22186);
        (assign21080_e22187, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign21080_e22189;
        var_fbreakdown_dn5 = assign21080_e22189_d_n5;
        var_fbreakdown_dn6 = assign21080_e22189_d_n6;
        var_fbreakdown_dn7 = assign21080_e22189_d_n7;
        var_fbreakdown_dn8 = assign21080_e22189_d_n8;

        let (assign21090_e22208, assign21090_e22208_d_n5, assign21090_e22208_d_n6, assign21090_e22208_d_n7, assign21090_e22208_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard359 == 0.0)) {
        let assign21090_e22199: f64 = (var_id__blk213 + var_isrh);
        let assign21090_e22201: f64 = (assign21090_e22199 + var_itat);
        let assign21090_e22203: f64 = (assign21090_e22201 + var_ibbt);
        let assign21090_e22204: f64 = (p.p29 * assign21090_e22203);
        let assign21090_e22206: f64 = (assign21090_e22204 * var_fbreakdown);
        (assign21090_e22206, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign21090_e22204 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign21090_e22204 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign21090_e22204 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign21090_e22204 * var_fbreakdown_dn8)),)
    } else {
        (var_ijunbot, var_ijunbot_dn5, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8,)
    }
};
        var_ijunbot = assign21090_e22208;
        var_ijunbot_dn5 = assign21090_e22208_d_n5;
        var_ijunbot_dn6 = assign21090_e22208_d_n6;
        var_ijunbot_dn7 = assign21090_e22208_d_n7;
        var_ijunbot_dn8 = assign21090_e22208_d_n8;

        let assign21100_e22211: f64 = if var_lssource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard376 = assign21100_e22211;

        let (assign21110_e22219, assign21110_e22219_d_n5, assign21110_e22219_d_n6, assign21110_e22219_d_n7, assign21110_e22219_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunsti, var_ijunsti_dn5, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8,)
    }
};
        var_ijunsti = assign21110_e22219;
        var_ijunsti_dn5 = assign21110_e22219_d_n5;
        var_ijunsti_dn6 = assign21110_e22219_d_n6;
        var_ijunsti_dn7 = assign21110_e22219_d_n7;
        var_ijunsti_dn8 = assign21110_e22219_d_n8;

        let (assign21120_e22230,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) {
        let assign21120_e22228: f64 = (var_idsatsti * var_idmult);
        (assign21120_e22228,)
    } else {
        (var_id__blk213,)
    }
};
        var_id__blk213 = assign21120_e22230;

        let assign21130_e22237: f64 = if ((p.p834 == 0.0) && (p.p839 == 0.0)) { 1.0 } else { 0.0 };
        var_guard377 = assign21130_e22237;

        let (assign21140_e22248, assign21140_e22248_d_n5, assign21140_e22248_d_n6, assign21140_e22248_d_n7, assign21140_e22248_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) && (var_guard377 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign21140_e22248;
        var_isrh_dn5 = assign21140_e22248_d_n5;
        var_isrh_dn6 = assign21140_e22248_d_n6;
        var_isrh_dn7 = assign21140_e22248_d_n7;
        var_isrh_dn8 = assign21140_e22248_d_n8;

        let (assign21150_e22262,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) && (var_guard377 == 0.0)) {
        let assign21150_e22260: f64 = (var_vbisti - var_vjsrh);
        (assign21150_e22260,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign21150_e22262;

        *var_erfcpos_slot = var_erfcpos;
        *var_erfcpos_dn5_slot = var_erfcpos_dn5;
        *var_erfcpos_dn6_slot = var_erfcpos_dn6;
        *var_erfcpos_dn7_slot = var_erfcpos_dn7;
        *var_erfcpos_dn8_slot = var_erfcpos_dn8;
        *var_erfctimesexpmtat_slot = var_erfctimesexpmtat;
        *var_erfctimesexpmtat_dn5_slot = var_erfctimesexpmtat_dn5;
        *var_erfctimesexpmtat_dn6_slot = var_erfctimesexpmtat_dn6;
        *var_erfctimesexpmtat_dn7_slot = var_erfctimesexpmtat_dn7;
        *var_erfctimesexpmtat_dn8_slot = var_erfctimesexpmtat_dn8;
        *var_fbreakdown_slot = var_fbreakdown;
        *var_fbreakdown_dn5_slot = var_fbreakdown_dn5;
        *var_fbreakdown_dn6_slot = var_fbreakdown_dn6;
        *var_fbreakdown_dn7_slot = var_fbreakdown_dn7;
        *var_fbreakdown_dn8_slot = var_fbreakdown_dn8;
        *var_fmaxr_slot = var_fmaxr;
        *var_fmaxr_dn5_slot = var_fmaxr_dn5;
        *var_fmaxr_dn6_slot = var_fmaxr_dn6;
        *var_fmaxr_dn7_slot = var_fmaxr_dn7;
        *var_fmaxr_dn8_slot = var_fmaxr_dn8;
        *var_gammamax_slot = var_gammamax;
        *var_gammamax_dn5_slot = var_gammamax_dn5;
        *var_gammamax_dn6_slot = var_gammamax_dn6;
        *var_gammamax_dn7_slot = var_gammamax_dn7;
        *var_gammamax_dn8_slot = var_gammamax_dn8;
        *var_guard367_slot = var_guard367;
        *var_guard368_slot = var_guard368;
        *var_guard369_slot = var_guard369;
        *var_guard370_slot = var_guard370;
        *var_guard371_slot = var_guard371;
        *var_guard372_slot = var_guard372;
        *var_guard373_slot = var_guard373;
        *var_guard374_slot = var_guard374;
        *var_guard375_slot = var_guard375;
        *var_guard376_slot = var_guard376;
        *var_guard377_slot = var_guard377;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn5_slot = var_ibbt_dn5;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
        *var_id__blk213_slot = var_id__blk213;
        *var_ijunbot_slot = var_ijunbot;
        *var_ijunbot_dn5_slot = var_ijunbot_dn5;
        *var_ijunbot_dn6_slot = var_ijunbot_dn6;
        *var_ijunbot_dn7_slot = var_ijunbot_dn7;
        *var_ijunbot_dn8_slot = var_ijunbot_dn8;
        *var_ijunsti_slot = var_ijunsti;
        *var_ijunsti_dn5_slot = var_ijunsti_dn5;
        *var_ijunsti_dn6_slot = var_ijunsti_dn6;
        *var_ijunsti_dn7_slot = var_ijunsti_dn7;
        *var_ijunsti_dn8_slot = var_ijunsti_dn8;
        *var_isrh_slot = var_isrh;
        *var_isrh_dn5_slot = var_isrh_dn5;
        *var_isrh_dn6_slot = var_isrh_dn6;
        *var_isrh_dn7_slot = var_isrh_dn7;
        *var_isrh_dn8_slot = var_isrh_dn8;
        *var_itat_slot = var_itat;
        *var_itat_dn5_slot = var_itat_dn5;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn5_slot = var_tmp_dn5;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_vbi_minus_vjsrh_slot = var_vbi_minus_vjsrh;
    }

    pub(super) fn stamp_transient_block_37(
        p: &Parameters,
        var_atatsti: f64,
        var_berfc: f64,
        var_btatpartsti: f64,
        var_cerfc: f64,
        var_ftdsti: f64,
        var_guard176: f64,
        var_guard193: f64,
        var_guard376: f64,
        var_guard377: f64,
        var_one_minus_psti: f64,
        var_one_over_one_minus_psti: f64,
        var_perfc: f64,
        var_two_psistar: f64,
        var_vbi_minus_vjsrh: f64,
        var_vbirstiinv: f64,
        var_wdepnulrsti: f64,
        var_zinv: f64,
        var_asrh_slot: &mut f64,
        var_asrh_dn5_slot: &mut f64,
        var_asrh_dn6_slot: &mut f64,
        var_asrh_dn7_slot: &mut f64,
        var_asrh_dn8_slot: &mut f64,
        var_btat_slot: &mut f64,
        var_btat_dn5_slot: &mut f64,
        var_btat_dn6_slot: &mut f64,
        var_btat_dn7_slot: &mut f64,
        var_btat_dn8_slot: &mut f64,
        var_dwsrh_slot: &mut f64,
        var_erfcpos_slot: &mut f64,
        var_erfcpos_dn5_slot: &mut f64,
        var_erfcpos_dn6_slot: &mut f64,
        var_erfcpos_dn7_slot: &mut f64,
        var_erfcpos_dn8_slot: &mut f64,
        var_erfctimesexpmtat_slot: &mut f64,
        var_erfctimesexpmtat_dn5_slot: &mut f64,
        var_erfctimesexpmtat_dn6_slot: &mut f64,
        var_erfctimesexpmtat_dn7_slot: &mut f64,
        var_erfctimesexpmtat_dn8_slot: &mut f64,
        var_guard378_slot: &mut f64,
        var_guard379_slot: &mut f64,
        var_guard380_slot: &mut f64,
        var_guard381_slot: &mut f64,
        var_guard382_slot: &mut f64,
        var_guard383_slot: &mut f64,
        var_guard384_slot: &mut f64,
        var_guard385_slot: &mut f64,
        var_isrh_slot: &mut f64,
        var_isrh_dn5_slot: &mut f64,
        var_isrh_dn6_slot: &mut f64,
        var_isrh_dn7_slot: &mut f64,
        var_isrh_dn8_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn5_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_ktat_slot: &mut f64,
        var_ktat_dn5_slot: &mut f64,
        var_ktat_dn6_slot: &mut f64,
        var_ktat_dn7_slot: &mut f64,
        var_ktat_dn8_slot: &mut f64,
        var_ltat_slot: &mut f64,
        var_ltat_dn5_slot: &mut f64,
        var_ltat_dn6_slot: &mut f64,
        var_ltat_dn7_slot: &mut f64,
        var_ltat_dn8_slot: &mut f64,
        var_mtat_slot: &mut f64,
        var_mtat_dn5_slot: &mut f64,
        var_mtat_dn6_slot: &mut f64,
        var_mtat_dn7_slot: &mut f64,
        var_mtat_dn8_slot: &mut f64,
        var_sqrtumax_slot: &mut f64,
        var_sqrtumax_dn5_slot: &mut f64,
        var_sqrtumax_dn6_slot: &mut f64,
        var_sqrtumax_dn7_slot: &mut f64,
        var_sqrtumax_dn8_slot: &mut f64,
        var_terfc_slot: &mut f64,
        var_terfc_dn5_slot: &mut f64,
        var_terfc_dn6_slot: &mut f64,
        var_terfc_dn7_slot: &mut f64,
        var_terfc_dn8_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn5_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_twoatatoverthreebtat_slot: &mut f64,
        var_twoatatoverthreebtat_dn5_slot: &mut f64,
        var_twoatatoverthreebtat_dn6_slot: &mut f64,
        var_twoatatoverthreebtat_dn7_slot: &mut f64,
        var_twoatatoverthreebtat_dn8_slot: &mut f64,
        var_umax_slot: &mut f64,
        var_umax_dn5_slot: &mut f64,
        var_umax_dn6_slot: &mut f64,
        var_umax_dn7_slot: &mut f64,
        var_umax_dn8_slot: &mut f64,
        var_umaxbeforelimiting_slot: &mut f64,
        var_umaxbeforelimiting_dn5_slot: &mut f64,
        var_umaxbeforelimiting_dn6_slot: &mut f64,
        var_umaxbeforelimiting_dn7_slot: &mut f64,
        var_umaxbeforelimiting_dn8_slot: &mut f64,
        var_umaxpoweronepointfive_slot: &mut f64,
        var_umaxpoweronepointfive_dn5_slot: &mut f64,
        var_umaxpoweronepointfive_dn6_slot: &mut f64,
        var_umaxpoweronepointfive_dn7_slot: &mut f64,
        var_umaxpoweronepointfive_dn8_slot: &mut f64,
        var_wdep_slot: &mut f64,
        var_wdep_dn5_slot: &mut f64,
        var_wdep_dn6_slot: &mut f64,
        var_wdep_dn7_slot: &mut f64,
        var_wdep_dn8_slot: &mut f64,
        var_wgamma_slot: &mut f64,
        var_wgamma_dn5_slot: &mut f64,
        var_wgamma_dn6_slot: &mut f64,
        var_wgamma_dn7_slot: &mut f64,
        var_wgamma_dn8_slot: &mut f64,
        var_wsrh_slot: &mut f64,
        var_wsrhstep_slot: &mut f64,
        var_wtat_slot: &mut f64,
        var_wtat_dn5_slot: &mut f64,
        var_wtat_dn6_slot: &mut f64,
        var_wtat_dn7_slot: &mut f64,
        var_wtat_dn8_slot: &mut f64,
        var_xerfc_slot: &mut f64,
        var_xerfc_dn5_slot: &mut f64,
        var_xerfc_dn6_slot: &mut f64,
        var_xerfc_dn7_slot: &mut f64,
        var_xerfc_dn8_slot: &mut f64,
        var_ysq_slot: &mut f64,
        var_ysq_dn5_slot: &mut f64,
        var_ysq_dn6_slot: &mut f64,
        var_ysq_dn7_slot: &mut f64,
        var_ysq_dn8_slot: &mut f64,
    ) {
        let mut var_asrh: f64 = *var_asrh_slot;
        let mut var_asrh_dn5: f64 = *var_asrh_dn5_slot;
        let mut var_asrh_dn6: f64 = *var_asrh_dn6_slot;
        let mut var_asrh_dn7: f64 = *var_asrh_dn7_slot;
        let mut var_asrh_dn8: f64 = *var_asrh_dn8_slot;
        let mut var_btat: f64 = *var_btat_slot;
        let mut var_btat_dn5: f64 = *var_btat_dn5_slot;
        let mut var_btat_dn6: f64 = *var_btat_dn6_slot;
        let mut var_btat_dn7: f64 = *var_btat_dn7_slot;
        let mut var_btat_dn8: f64 = *var_btat_dn8_slot;
        let mut var_dwsrh: f64 = *var_dwsrh_slot;
        let mut var_erfcpos: f64 = *var_erfcpos_slot;
        let mut var_erfcpos_dn5: f64 = *var_erfcpos_dn5_slot;
        let mut var_erfcpos_dn6: f64 = *var_erfcpos_dn6_slot;
        let mut var_erfcpos_dn7: f64 = *var_erfcpos_dn7_slot;
        let mut var_erfcpos_dn8: f64 = *var_erfcpos_dn8_slot;
        let mut var_erfctimesexpmtat: f64 = *var_erfctimesexpmtat_slot;
        let mut var_erfctimesexpmtat_dn5: f64 = *var_erfctimesexpmtat_dn5_slot;
        let mut var_erfctimesexpmtat_dn6: f64 = *var_erfctimesexpmtat_dn6_slot;
        let mut var_erfctimesexpmtat_dn7: f64 = *var_erfctimesexpmtat_dn7_slot;
        let mut var_erfctimesexpmtat_dn8: f64 = *var_erfctimesexpmtat_dn8_slot;
        let mut var_guard378: f64 = *var_guard378_slot;
        let mut var_guard379: f64 = *var_guard379_slot;
        let mut var_guard380: f64 = *var_guard380_slot;
        let mut var_guard381: f64 = *var_guard381_slot;
        let mut var_guard382: f64 = *var_guard382_slot;
        let mut var_guard383: f64 = *var_guard383_slot;
        let mut var_guard384: f64 = *var_guard384_slot;
        let mut var_guard385: f64 = *var_guard385_slot;
        let mut var_isrh: f64 = *var_isrh_slot;
        let mut var_isrh_dn5: f64 = *var_isrh_dn5_slot;
        let mut var_isrh_dn6: f64 = *var_isrh_dn6_slot;
        let mut var_isrh_dn7: f64 = *var_isrh_dn7_slot;
        let mut var_isrh_dn8: f64 = *var_isrh_dn8_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn5: f64 = *var_itat_dn5_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_ktat: f64 = *var_ktat_slot;
        let mut var_ktat_dn5: f64 = *var_ktat_dn5_slot;
        let mut var_ktat_dn6: f64 = *var_ktat_dn6_slot;
        let mut var_ktat_dn7: f64 = *var_ktat_dn7_slot;
        let mut var_ktat_dn8: f64 = *var_ktat_dn8_slot;
        let mut var_ltat: f64 = *var_ltat_slot;
        let mut var_ltat_dn5: f64 = *var_ltat_dn5_slot;
        let mut var_ltat_dn6: f64 = *var_ltat_dn6_slot;
        let mut var_ltat_dn7: f64 = *var_ltat_dn7_slot;
        let mut var_ltat_dn8: f64 = *var_ltat_dn8_slot;
        let mut var_mtat: f64 = *var_mtat_slot;
        let mut var_mtat_dn5: f64 = *var_mtat_dn5_slot;
        let mut var_mtat_dn6: f64 = *var_mtat_dn6_slot;
        let mut var_mtat_dn7: f64 = *var_mtat_dn7_slot;
        let mut var_mtat_dn8: f64 = *var_mtat_dn8_slot;
        let mut var_sqrtumax: f64 = *var_sqrtumax_slot;
        let mut var_sqrtumax_dn5: f64 = *var_sqrtumax_dn5_slot;
        let mut var_sqrtumax_dn6: f64 = *var_sqrtumax_dn6_slot;
        let mut var_sqrtumax_dn7: f64 = *var_sqrtumax_dn7_slot;
        let mut var_sqrtumax_dn8: f64 = *var_sqrtumax_dn8_slot;
        let mut var_terfc: f64 = *var_terfc_slot;
        let mut var_terfc_dn5: f64 = *var_terfc_dn5_slot;
        let mut var_terfc_dn6: f64 = *var_terfc_dn6_slot;
        let mut var_terfc_dn7: f64 = *var_terfc_dn7_slot;
        let mut var_terfc_dn8: f64 = *var_terfc_dn8_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn5: f64 = *var_tmp_dn5_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_twoatatoverthreebtat: f64 = *var_twoatatoverthreebtat_slot;
        let mut var_twoatatoverthreebtat_dn5: f64 = *var_twoatatoverthreebtat_dn5_slot;
        let mut var_twoatatoverthreebtat_dn6: f64 = *var_twoatatoverthreebtat_dn6_slot;
        let mut var_twoatatoverthreebtat_dn7: f64 = *var_twoatatoverthreebtat_dn7_slot;
        let mut var_twoatatoverthreebtat_dn8: f64 = *var_twoatatoverthreebtat_dn8_slot;
        let mut var_umax: f64 = *var_umax_slot;
        let mut var_umax_dn5: f64 = *var_umax_dn5_slot;
        let mut var_umax_dn6: f64 = *var_umax_dn6_slot;
        let mut var_umax_dn7: f64 = *var_umax_dn7_slot;
        let mut var_umax_dn8: f64 = *var_umax_dn8_slot;
        let mut var_umaxbeforelimiting: f64 = *var_umaxbeforelimiting_slot;
        let mut var_umaxbeforelimiting_dn5: f64 = *var_umaxbeforelimiting_dn5_slot;
        let mut var_umaxbeforelimiting_dn6: f64 = *var_umaxbeforelimiting_dn6_slot;
        let mut var_umaxbeforelimiting_dn7: f64 = *var_umaxbeforelimiting_dn7_slot;
        let mut var_umaxbeforelimiting_dn8: f64 = *var_umaxbeforelimiting_dn8_slot;
        let mut var_umaxpoweronepointfive: f64 = *var_umaxpoweronepointfive_slot;
        let mut var_umaxpoweronepointfive_dn5: f64 = *var_umaxpoweronepointfive_dn5_slot;
        let mut var_umaxpoweronepointfive_dn6: f64 = *var_umaxpoweronepointfive_dn6_slot;
        let mut var_umaxpoweronepointfive_dn7: f64 = *var_umaxpoweronepointfive_dn7_slot;
        let mut var_umaxpoweronepointfive_dn8: f64 = *var_umaxpoweronepointfive_dn8_slot;
        let mut var_wdep: f64 = *var_wdep_slot;
        let mut var_wdep_dn5: f64 = *var_wdep_dn5_slot;
        let mut var_wdep_dn6: f64 = *var_wdep_dn6_slot;
        let mut var_wdep_dn7: f64 = *var_wdep_dn7_slot;
        let mut var_wdep_dn8: f64 = *var_wdep_dn8_slot;
        let mut var_wgamma: f64 = *var_wgamma_slot;
        let mut var_wgamma_dn5: f64 = *var_wgamma_dn5_slot;
        let mut var_wgamma_dn6: f64 = *var_wgamma_dn6_slot;
        let mut var_wgamma_dn7: f64 = *var_wgamma_dn7_slot;
        let mut var_wgamma_dn8: f64 = *var_wgamma_dn8_slot;
        let mut var_wsrh: f64 = *var_wsrh_slot;
        let mut var_wsrhstep: f64 = *var_wsrhstep_slot;
        let mut var_wtat: f64 = *var_wtat_slot;
        let mut var_wtat_dn5: f64 = *var_wtat_dn5_slot;
        let mut var_wtat_dn6: f64 = *var_wtat_dn6_slot;
        let mut var_wtat_dn7: f64 = *var_wtat_dn7_slot;
        let mut var_wtat_dn8: f64 = *var_wtat_dn8_slot;
        let mut var_xerfc: f64 = *var_xerfc_slot;
        let mut var_xerfc_dn5: f64 = *var_xerfc_dn5_slot;
        let mut var_xerfc_dn6: f64 = *var_xerfc_dn6_slot;
        let mut var_xerfc_dn7: f64 = *var_xerfc_dn7_slot;
        let mut var_xerfc_dn8: f64 = *var_xerfc_dn8_slot;
        let mut var_ysq: f64 = *var_ysq_slot;
        let mut var_ysq_dn5: f64 = *var_ysq_dn5_slot;
        let mut var_ysq_dn6: f64 = *var_ysq_dn6_slot;
        let mut var_ysq_dn7: f64 = *var_ysq_dn7_slot;
        let mut var_ysq_dn8: f64 = *var_ysq_dn8_slot;

        let (assign21160_e22281,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) && (var_guard377 == 0.0)) {
        let assign21160_e22276: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign21160_e22277: f64 = (1.0 - assign21160_e22276);
        let assign21160_e22278: f64 = (assign21160_e22277).sqrt();
        let assign21160_e22279: f64 = (1.0 - assign21160_e22278);
        (assign21160_e22279,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign21160_e22281;

        let assign21170_e22284: f64 = if p.p825 == 0.5 { 1.0 } else { 0.0 };
        var_guard378 = assign21170_e22284;

        let (assign21180_e22298,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) && (var_guard377 == 0.0)) && (var_guard378 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign21180_e22298;

        let (assign21190_e22330,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) && (var_guard377 == 0.0)) && (var_guard378 == 0.0)) {
        let assign21190_e22313: f64 = (var_wsrhstep * var_wsrhstep);
        let assign21190_e22315: f64 = (var_wsrhstep).ln();
        let assign21190_e22316: f64 = (assign21190_e22313 * assign21190_e22315);
        let assign21190_e22319: f64 = (1.0 - var_wsrhstep);
        let assign21190_e22320: f64 = (assign21190_e22316 / assign21190_e22319);
        let assign21190_e22322: f64 = (assign21190_e22320 + var_wsrhstep);
        let assign21190_e22326: f64 = (2.0 * p.p825);
        let assign21190_e22327: f64 = (1.0 - assign21190_e22326);
        let assign21190_e22328: f64 = (assign21190_e22322 * assign21190_e22327);
        (assign21190_e22328,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign21190_e22330;

        let (assign21200_e22344,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) && (var_guard377 == 0.0)) {
        let assign21200_e22342: f64 = (var_wsrhstep + var_dwsrh);
        (assign21200_e22342,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign21200_e22344;

        let assign21210_e22347: f64 = if p.p825 == 0.5 { 1.0 } else { 0.0 };
        var_guard379 = assign21210_e22347;

        let (assign21220_e22364, assign21220_e22364_d_n5, assign21220_e22364_d_n6, assign21220_e22364_d_n7, assign21220_e22364_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) && (var_guard377 == 0.0)) && (var_guard379 != 0.0)) {
        let assign21220_e22361: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv);
        let assign21220_e22362: f64 = (assign21220_e22361).sqrt();
        (assign21220_e22362, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign21220_e22364;
        var_tmp_dn5 = assign21220_e22364_d_n5;
        var_tmp_dn6 = assign21220_e22364_d_n6;
        var_tmp_dn7 = assign21220_e22364_d_n7;
        var_tmp_dn8 = assign21220_e22364_d_n8;

        let (assign21230_e22383, assign21230_e22383_d_n5, assign21230_e22383_d_n6, assign21230_e22383_d_n7, assign21230_e22383_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) && (var_guard377 == 0.0)) && (var_guard379 == 0.0)) {
        let assign21230_e22379: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv);
        let assign21230_e22381: f64 = (assign21230_e22379).powf(p.p825);
        (assign21230_e22381, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign21230_e22383;
        var_tmp_dn5 = assign21230_e22383_d_n5;
        var_tmp_dn6 = assign21230_e22383_d_n6;
        var_tmp_dn7 = assign21230_e22383_d_n7;
        var_tmp_dn8 = assign21230_e22383_d_n8;

        let (assign21240_e22397, assign21240_e22397_d_n5, assign21240_e22397_d_n6, assign21240_e22397_d_n7, assign21240_e22397_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) && (var_guard377 == 0.0)) {
        let assign21240_e22395: f64 = (var_wdepnulrsti * var_tmp);
        (assign21240_e22395, (var_wdepnulrsti * var_tmp_dn5), (var_wdepnulrsti * var_tmp_dn6), (var_wdepnulrsti * var_tmp_dn7), (var_wdepnulrsti * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign21240_e22397;
        var_wdep_dn5 = assign21240_e22397_d_n5;
        var_wdep_dn6 = assign21240_e22397_d_n6;
        var_wdep_dn7 = assign21240_e22397_d_n7;
        var_wdep_dn8 = assign21240_e22397_d_n8;

        let (assign21250_e22415, assign21250_e22415_d_n5, assign21250_e22415_d_n6, assign21250_e22415_d_n7, assign21250_e22415_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) && (var_guard377 == 0.0)) {
        let assign21250_e22410: f64 = (var_zinv - 1.0);
        let assign21250_e22412: f64 = (assign21250_e22410 * var_wdep);
        let assign21250_e22413: f64 = (var_ftdsti * assign21250_e22412);
        (assign21250_e22413, (var_ftdsti * (assign21250_e22410 * var_wdep_dn5)), (var_ftdsti * (assign21250_e22410 * var_wdep_dn6)), (var_ftdsti * (assign21250_e22410 * var_wdep_dn7)), (var_ftdsti * (assign21250_e22410 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign21250_e22415;
        var_asrh_dn5 = assign21250_e22415_d_n5;
        var_asrh_dn6 = assign21250_e22415_d_n6;
        var_asrh_dn7 = assign21250_e22415_d_n7;
        var_asrh_dn8 = assign21250_e22415_d_n8;

        let (assign21260_e22431, assign21260_e22431_d_n5, assign21260_e22431_d_n6, assign21260_e22431_d_n7, assign21260_e22431_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) && (var_guard377 == 0.0)) {
        let assign21260_e22428: f64 = (var_asrh * var_wsrh);
        let assign21260_e22429: f64 = (p.p834 * assign21260_e22428);
        (assign21260_e22429, (p.p834 * (var_asrh_dn5 * var_wsrh)), (p.p834 * (var_asrh_dn6 * var_wsrh)), (p.p834 * (var_asrh_dn7 * var_wsrh)), (p.p834 * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign21260_e22431;
        var_isrh_dn5 = assign21260_e22431_d_n5;
        var_isrh_dn6 = assign21260_e22431_d_n6;
        var_isrh_dn7 = assign21260_e22431_d_n7;
        var_isrh_dn8 = assign21260_e22431_d_n8;

        let assign21270_e22434: f64 = if p.p839 == 0.0 { 1.0 } else { 0.0 };
        var_guard380 = assign21270_e22434;

        let (assign21280_e22445, assign21280_e22445_d_n5, assign21280_e22445_d_n6, assign21280_e22445_d_n7, assign21280_e22445_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) && (var_guard380 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign21280_e22445;
        var_itat_dn5 = assign21280_e22445_d_n5;
        var_itat_dn6 = assign21280_e22445_d_n6;
        var_itat_dn7 = assign21280_e22445_d_n7;
        var_itat_dn8 = assign21280_e22445_d_n8;

        let (assign21290_e22463, assign21290_e22463_d_n5, assign21290_e22463_d_n6, assign21290_e22463_d_n7, assign21290_e22463_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) && (var_guard380 == 0.0)) {
        let assign21290_e22458: f64 = (var_wdep * var_one_minus_psti);
        let assign21290_e22460: f64 = (assign21290_e22458 / var_vbi_minus_vjsrh);
        let assign21290_e22461: f64 = (var_btatpartsti * assign21290_e22460);
        (assign21290_e22461, (var_btatpartsti * ((var_wdep_dn5 * var_one_minus_psti) / var_vbi_minus_vjsrh)), (var_btatpartsti * ((var_wdep_dn6 * var_one_minus_psti) / var_vbi_minus_vjsrh)), (var_btatpartsti * ((var_wdep_dn7 * var_one_minus_psti) / var_vbi_minus_vjsrh)), (var_btatpartsti * ((var_wdep_dn8 * var_one_minus_psti) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign21290_e22463;
        var_btat_dn5 = assign21290_e22463_d_n5;
        var_btat_dn6 = assign21290_e22463_d_n6;
        var_btat_dn7 = assign21290_e22463_d_n7;
        var_btat_dn8 = assign21290_e22463_d_n8;

        let (assign21300_e22479, assign21300_e22479_d_n5, assign21300_e22479_d_n6, assign21300_e22479_d_n7, assign21300_e22479_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) && (var_guard380 == 0.0)) {
        let assign21300_e22475: f64 = (0.666666666666667 * var_atatsti);
        let assign21300_e22477: f64 = (assign21300_e22475 / var_btat);
        (assign21300_e22477, (-((assign21300_e22475 * var_btat_dn5) / (var_btat * var_btat))), (-((assign21300_e22475 * var_btat_dn6) / (var_btat * var_btat))), (-((assign21300_e22475 * var_btat_dn7) / (var_btat * var_btat))), (-((assign21300_e22475 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign21300_e22479;
        var_twoatatoverthreebtat_dn5 = assign21300_e22479_d_n5;
        var_twoatatoverthreebtat_dn6 = assign21300_e22479_d_n6;
        var_twoatatoverthreebtat_dn7 = assign21300_e22479_d_n7;
        var_twoatatoverthreebtat_dn8 = assign21300_e22479_d_n8;

        let (assign21310_e22493, assign21310_e22493_d_n5, assign21310_e22493_d_n6, assign21310_e22493_d_n7, assign21310_e22493_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) && (var_guard380 == 0.0)) {
        let assign21310_e22491: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign21310_e22491, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign21310_e22493;
        var_umaxbeforelimiting_dn5 = assign21310_e22493_d_n5;
        var_umaxbeforelimiting_dn6 = assign21310_e22493_d_n6;
        var_umaxbeforelimiting_dn7 = assign21310_e22493_d_n7;
        var_umaxbeforelimiting_dn8 = assign21310_e22493_d_n8;

        let (assign21320_e22514, assign21320_e22514_d_n5, assign21320_e22514_d_n6, assign21320_e22514_d_n7, assign21320_e22514_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) && (var_guard380 == 0.0)) {
        let assign21320_e22505: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign21320_e22508: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign21320_e22510: f64 = (assign21320_e22508 + 1.0);
        let assign21320_e22511: f64 = (assign21320_e22505 / assign21320_e22510);
        let assign21320_e22512: f64 = (assign21320_e22511).sqrt();
        (assign21320_e22512, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign21320_e22510) - (assign21320_e22505 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign21320_e22510 * assign21320_e22510)) / (2.0 * assign21320_e22512)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign21320_e22510) - (assign21320_e22505 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign21320_e22510 * assign21320_e22510)) / (2.0 * assign21320_e22512)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign21320_e22510) - (assign21320_e22505 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign21320_e22510 * assign21320_e22510)) / (2.0 * assign21320_e22512)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign21320_e22510) - (assign21320_e22505 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign21320_e22510 * assign21320_e22510)) / (2.0 * assign21320_e22512)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign21320_e22514;
        var_umax_dn5 = assign21320_e22514_d_n5;
        var_umax_dn6 = assign21320_e22514_d_n6;
        var_umax_dn7 = assign21320_e22514_d_n7;
        var_umax_dn8 = assign21320_e22514_d_n8;

        let (assign21330_e22527, assign21330_e22527_d_n5, assign21330_e22527_d_n6, assign21330_e22527_d_n7, assign21330_e22527_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) && (var_guard380 == 0.0)) {
        let assign21330_e22525: f64 = (var_umax).sqrt();
        (assign21330_e22525, (var_umax_dn5 / (2.0 * assign21330_e22525)), (var_umax_dn6 / (2.0 * assign21330_e22525)), (var_umax_dn7 / (2.0 * assign21330_e22525)), (var_umax_dn8 / (2.0 * assign21330_e22525)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign21330_e22527;
        var_sqrtumax_dn5 = assign21330_e22527_d_n5;
        var_sqrtumax_dn6 = assign21330_e22527_d_n6;
        var_sqrtumax_dn7 = assign21330_e22527_d_n7;
        var_sqrtumax_dn8 = assign21330_e22527_d_n8;

        let (assign21340_e22541, assign21340_e22541_d_n5, assign21340_e22541_d_n6, assign21340_e22541_d_n7, assign21340_e22541_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) && (var_guard380 == 0.0)) {
        let assign21340_e22539: f64 = (var_umax * var_sqrtumax);
        (assign21340_e22539, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign21340_e22541;
        var_umaxpoweronepointfive_dn5 = assign21340_e22541_d_n5;
        var_umaxpoweronepointfive_dn6 = assign21340_e22541_d_n6;
        var_umaxpoweronepointfive_dn7 = assign21340_e22541_d_n7;
        var_umaxpoweronepointfive_dn8 = assign21340_e22541_d_n8;

        let assign21350_e22543: f64 = (-p.p825);
        let assign21350_e22545: f64 = (assign21350_e22543 * var_one_over_one_minus_psti);
        let assign21350_e22547: f64 = (-1.0);
        let assign21350_e22548: f64 = if assign21350_e22545 == assign21350_e22547 { 1.0 } else { 0.0 };
        var_guard381 = assign21350_e22548;

        let (assign21360_e22568, assign21360_e22568_d_n5, assign21360_e22568_d_n6, assign21360_e22568_d_n7, assign21360_e22568_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) && (var_guard380 == 0.0)) && (var_guard381 != 0.0)) {
        let assign21360_e22564: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign21360_e22565: f64 = (1.0 + assign21360_e22564);
        let assign21360_e22566: f64 = (1.0 / assign21360_e22565);
        (assign21360_e22566, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign21360_e22565 * assign21360_e22565))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign21360_e22565 * assign21360_e22565))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign21360_e22565 * assign21360_e22565))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign21360_e22565 * assign21360_e22565))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign21360_e22568;
        var_wgamma_dn5 = assign21360_e22568_d_n5;
        var_wgamma_dn6 = assign21360_e22568_d_n6;
        var_wgamma_dn7 = assign21360_e22568_d_n7;
        var_wgamma_dn8 = assign21360_e22568_d_n8;

        let (assign21370_e22592, assign21370_e22592_d_n5, assign21370_e22592_d_n6, assign21370_e22592_d_n7, assign21370_e22592_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) && (var_guard380 == 0.0)) && (var_guard381 == 0.0)) {
        let assign21370_e22584: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign21370_e22585: f64 = (1.0 + assign21370_e22584);
        let assign21370_e22587: f64 = (-p.p825);
        let assign21370_e22589: f64 = (assign21370_e22587 * var_one_over_one_minus_psti);
        let assign21370_e22590: f64 = (assign21370_e22585).powf(assign21370_e22589);
        (assign21370_e22590, if 0.0 == 0.0 && ((assign21370_e22589) as f64).is_finite() && ((assign21370_e22589) as f64).fract() == 0.0 { if assign21370_e22589 == 0.0 { 0.0 } else { (assign21370_e22589 * ((assign21370_e22585).powf(assign21370_e22589 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign21370_e22590 * (assign21370_e22589 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign21370_e22585))) }, if 0.0 == 0.0 && ((assign21370_e22589) as f64).is_finite() && ((assign21370_e22589) as f64).fract() == 0.0 { if assign21370_e22589 == 0.0 { 0.0 } else { (assign21370_e22589 * ((assign21370_e22585).powf(assign21370_e22589 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign21370_e22590 * (assign21370_e22589 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign21370_e22585))) }, if 0.0 == 0.0 && ((assign21370_e22589) as f64).is_finite() && ((assign21370_e22589) as f64).fract() == 0.0 { if assign21370_e22589 == 0.0 { 0.0 } else { (assign21370_e22589 * ((assign21370_e22585).powf(assign21370_e22589 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign21370_e22590 * (assign21370_e22589 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign21370_e22585))) }, if 0.0 == 0.0 && ((assign21370_e22589) as f64).is_finite() && ((assign21370_e22589) as f64).fract() == 0.0 { if assign21370_e22589 == 0.0 { 0.0 } else { (assign21370_e22589 * ((assign21370_e22585).powf(assign21370_e22589 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign21370_e22590 * (assign21370_e22589 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign21370_e22585))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign21370_e22592;
        var_wgamma_dn5 = assign21370_e22592_d_n5;
        var_wgamma_dn6 = assign21370_e22592_d_n6;
        var_wgamma_dn7 = assign21370_e22592_d_n7;
        var_wgamma_dn8 = assign21370_e22592_d_n8;

        let (assign21380_e22610, assign21380_e22610_d_n5, assign21380_e22610_d_n6, assign21380_e22610_d_n7, assign21380_e22610_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) && (var_guard380 == 0.0)) {
        let assign21380_e22604: f64 = (var_wsrh * var_wgamma);
        let assign21380_e22607: f64 = (var_wsrh + var_wgamma);
        let assign21380_e22608: f64 = (assign21380_e22604 / assign21380_e22607);
        (assign21380_e22608, ((((var_wsrh * var_wgamma_dn5) * assign21380_e22607) - (assign21380_e22604 * var_wgamma_dn5)) / (assign21380_e22607 * assign21380_e22607)), ((((var_wsrh * var_wgamma_dn6) * assign21380_e22607) - (assign21380_e22604 * var_wgamma_dn6)) / (assign21380_e22607 * assign21380_e22607)), ((((var_wsrh * var_wgamma_dn7) * assign21380_e22607) - (assign21380_e22604 * var_wgamma_dn7)) / (assign21380_e22607 * assign21380_e22607)), ((((var_wsrh * var_wgamma_dn8) * assign21380_e22607) - (assign21380_e22604 * var_wgamma_dn8)) / (assign21380_e22607 * assign21380_e22607)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign21380_e22610;
        var_wtat_dn5 = assign21380_e22610_d_n5;
        var_wtat_dn6 = assign21380_e22610_d_n6;
        var_wtat_dn7 = assign21380_e22610_d_n7;
        var_wtat_dn8 = assign21380_e22610_d_n8;

        let (assign21390_e22627, assign21390_e22627_d_n5, assign21390_e22627_d_n6, assign21390_e22627_d_n7, assign21390_e22627_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) && (var_guard380 == 0.0)) {
        let assign21390_e22623: f64 = (var_btat / var_sqrtumax);
        let assign21390_e22624: f64 = (0.375 * assign21390_e22623);
        let assign21390_e22625: f64 = (assign21390_e22624).sqrt();
        (assign21390_e22625, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign21390_e22625)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign21390_e22625)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign21390_e22625)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign21390_e22625)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign21390_e22627;
        var_ktat_dn5 = assign21390_e22627_d_n5;
        var_ktat_dn6 = assign21390_e22627_d_n6;
        var_ktat_dn7 = assign21390_e22627_d_n7;
        var_ktat_dn8 = assign21390_e22627_d_n8;

        let (assign21400_e22645, assign21400_e22645_d_n5, assign21400_e22645_d_n6, assign21400_e22645_d_n7, assign21400_e22645_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) && (var_guard380 == 0.0)) {
        let assign21400_e22640: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign21400_e22641: f64 = (2.0 * assign21400_e22640);
        let assign21400_e22643: f64 = (assign21400_e22641 - var_umax);
        (assign21400_e22643, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign21400_e22645;
        var_ltat_dn5 = assign21400_e22645_d_n5;
        var_ltat_dn6 = assign21400_e22645_d_n6;
        var_ltat_dn7 = assign21400_e22645_d_n7;
        var_ltat_dn8 = assign21400_e22645_d_n8;

        let (assign21410_e22671, assign21410_e22671_d_n5, assign21410_e22671_d_n6, assign21410_e22671_d_n7, assign21410_e22671_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) && (var_guard380 == 0.0)) {
        let assign21410_e22657: f64 = (var_atatsti * var_twoatatoverthreebtat);
        let assign21410_e22659: f64 = (assign21410_e22657 * var_sqrtumax);
        let assign21410_e22662: f64 = (var_atatsti * var_umax);
        let assign21410_e22663: f64 = (assign21410_e22659 - assign21410_e22662);
        let assign21410_e22667: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign21410_e22668: f64 = (0.5 * assign21410_e22667);
        let assign21410_e22669: f64 = (assign21410_e22663 + assign21410_e22668);
        (assign21410_e22669, (((((var_atatsti * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign21410_e22657 * var_sqrtumax_dn5)) - (var_atatsti * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatsti * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign21410_e22657 * var_sqrtumax_dn6)) - (var_atatsti * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatsti * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign21410_e22657 * var_sqrtumax_dn7)) - (var_atatsti * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatsti * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign21410_e22657 * var_sqrtumax_dn8)) - (var_atatsti * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign21410_e22671;
        var_mtat_dn5 = assign21410_e22671_d_n5;
        var_mtat_dn6 = assign21410_e22671_d_n6;
        var_mtat_dn7 = assign21410_e22671_d_n7;
        var_mtat_dn8 = assign21410_e22671_d_n8;

        let (assign21420_e22687, assign21420_e22687_d_n5, assign21420_e22687_d_n6, assign21420_e22687_d_n7, assign21420_e22687_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) && (var_guard380 == 0.0)) {
        let assign21420_e22683: f64 = (var_ltat - 1.0);
        let assign21420_e22685: f64 = (assign21420_e22683 * var_ktat);
        (assign21420_e22685, ((var_ltat_dn5 * var_ktat) + (assign21420_e22683 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign21420_e22683 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign21420_e22683 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign21420_e22683 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign21420_e22687;
        var_xerfc_dn5 = assign21420_e22687_d_n5;
        var_xerfc_dn6 = assign21420_e22687_d_n6;
        var_xerfc_dn7 = assign21420_e22687_d_n7;
        var_xerfc_dn8 = assign21420_e22687_d_n8;

        let (assign21430_e22701, assign21430_e22701_d_n5, assign21430_e22701_d_n6, assign21430_e22701_d_n7, assign21430_e22701_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) && (var_guard380 == 0.0)) {
        let assign21430_e22699: f64 = (var_xerfc * var_xerfc);
        (assign21430_e22699, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign21430_e22701;
        var_ysq_dn5 = assign21430_e22701_d_n5;
        var_ysq_dn6 = assign21430_e22701_d_n6;
        var_ysq_dn7 = assign21430_e22701_d_n7;
        var_ysq_dn8 = assign21430_e22701_d_n8;

        let assign21440_e22704: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard382 = assign21440_e22704;

        let (assign21450_e22724, assign21450_e22724_d_n5, assign21450_e22724_d_n6, assign21450_e22724_d_n7, assign21450_e22724_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) && (var_guard380 == 0.0)) && (var_guard382 != 0.0)) {
        let assign21450_e22720: f64 = (var_perfc * var_xerfc);
        let assign21450_e22721: f64 = (1.0 + assign21450_e22720);
        let assign21450_e22722: f64 = (1.0 / assign21450_e22721);
        (assign21450_e22722, (-((var_perfc * var_xerfc_dn5) / (assign21450_e22721 * assign21450_e22721))), (-((var_perfc * var_xerfc_dn6) / (assign21450_e22721 * assign21450_e22721))), (-((var_perfc * var_xerfc_dn7) / (assign21450_e22721 * assign21450_e22721))), (-((var_perfc * var_xerfc_dn8) / (assign21450_e22721 * assign21450_e22721))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign21450_e22724;
        var_terfc_dn5 = assign21450_e22724_d_n5;
        var_terfc_dn6 = assign21450_e22724_d_n6;
        var_terfc_dn7 = assign21450_e22724_d_n7;
        var_terfc_dn8 = assign21450_e22724_d_n8;

        let (assign21460_e22745, assign21460_e22745_d_n5, assign21460_e22745_d_n6, assign21460_e22745_d_n7, assign21460_e22745_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) && (var_guard380 == 0.0)) && (var_guard382 == 0.0)) {
        let assign21460_e22741: f64 = (var_perfc * var_xerfc);
        let assign21460_e22742: f64 = (1.0 - assign21460_e22741);
        let assign21460_e22743: f64 = (1.0 / assign21460_e22742);
        (assign21460_e22743, (-((-(var_perfc * var_xerfc_dn5)) / (assign21460_e22742 * assign21460_e22742))), (-((-(var_perfc * var_xerfc_dn6)) / (assign21460_e22742 * assign21460_e22742))), (-((-(var_perfc * var_xerfc_dn7)) / (assign21460_e22742 * assign21460_e22742))), (-((-(var_perfc * var_xerfc_dn8)) / (assign21460_e22742 * assign21460_e22742))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign21460_e22745;
        var_terfc_dn5 = assign21460_e22745_d_n5;
        var_terfc_dn6 = assign21460_e22745_d_n6;
        var_terfc_dn7 = assign21460_e22745_d_n7;
        var_terfc_dn8 = assign21460_e22745_d_n8;

        let assign21470_e22747: f64 = (-var_ysq);
        let assign21470_e22749: f64 = (assign21470_e22747 + var_mtat);
        let assign21470_e22751: f64 = (-230.25850929940458);
        let assign21470_e22752: f64 = if assign21470_e22749 > assign21470_e22751 { 1.0 } else { 0.0 };
        var_guard383 = assign21470_e22752;

        let (assign21480_e22770, assign21480_e22770_d_n5, assign21480_e22770_d_n6, assign21480_e22770_d_n7, assign21480_e22770_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) && (var_guard380 == 0.0)) && (var_guard383 != 0.0)) {
        let assign21480_e22765: f64 = (-var_ysq);
        let assign21480_e22767: f64 = (assign21480_e22765 + var_mtat);
        let assign21480_e22768: f64 = (assign21480_e22767).exp();
        (assign21480_e22768, (assign21480_e22768 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign21480_e22768 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign21480_e22768 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign21480_e22768 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign21480_e22770;
        var_tmp_dn5 = assign21480_e22770_d_n5;
        var_tmp_dn6 = assign21480_e22770_d_n6;
        var_tmp_dn7 = assign21480_e22770_d_n7;
        var_tmp_dn8 = assign21480_e22770_d_n8;

        let (assign21490_e22819, assign21490_e22819_d_n5, assign21490_e22819_d_n6, assign21490_e22819_d_n7, assign21490_e22819_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) && (var_guard380 == 0.0)) && (var_guard383 == 0.0)) {
        let assign21490_e22786: f64 = (-230.25850929940458);
        let assign21490_e22788: f64 = (-var_ysq);
        let assign21490_e22790: f64 = (assign21490_e22788 + var_mtat);
        let assign21490_e22791: f64 = (assign21490_e22786 - assign21490_e22790);
        let assign21490_e22795: f64 = (-230.25850929940458);
        let assign21490_e22797: f64 = (-var_ysq);
        let assign21490_e22799: f64 = (assign21490_e22797 + var_mtat);
        let assign21490_e22800: f64 = (assign21490_e22795 - assign21490_e22799);
        let assign21490_e22803: f64 = (-230.25850929940458);
        let assign21490_e22805: f64 = (-var_ysq);
        let assign21490_e22807: f64 = (assign21490_e22805 + var_mtat);
        let assign21490_e22808: f64 = (assign21490_e22803 - assign21490_e22807);
        let assign21490_e22810: f64 = (assign21490_e22808 * 0.3333333333333333);
        let assign21490_e22811: f64 = (1.0 + assign21490_e22810);
        let assign21490_e22812: f64 = (assign21490_e22800 * assign21490_e22811);
        let assign21490_e22813: f64 = (0.5 * assign21490_e22812);
        let assign21490_e22814: f64 = (1.0 + assign21490_e22813);
        let assign21490_e22815: f64 = (assign21490_e22791 * assign21490_e22814);
        let assign21490_e22816: f64 = (1.0 + assign21490_e22815);
        let assign21490_e22817: f64 = (1e-100 / assign21490_e22816);
        (assign21490_e22817, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign21490_e22814) + (assign21490_e22791 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign21490_e22811) + (assign21490_e22800 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign21490_e22816 * assign21490_e22816))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign21490_e22814) + (assign21490_e22791 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign21490_e22811) + (assign21490_e22800 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign21490_e22816 * assign21490_e22816))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign21490_e22814) + (assign21490_e22791 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign21490_e22811) + (assign21490_e22800 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign21490_e22816 * assign21490_e22816))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign21490_e22814) + (assign21490_e22791 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign21490_e22811) + (assign21490_e22800 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign21490_e22816 * assign21490_e22816))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign21490_e22819;
        var_tmp_dn5 = assign21490_e22819_d_n5;
        var_tmp_dn6 = assign21490_e22819_d_n6;
        var_tmp_dn7 = assign21490_e22819_d_n7;
        var_tmp_dn8 = assign21490_e22819_d_n8;

        let (assign21500_e22849, assign21500_e22849_d_n5, assign21500_e22849_d_n6, assign21500_e22849_d_n7, assign21500_e22849_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) && (var_guard380 == 0.0)) {
        let assign21500_e22831: f64 = (0.29214664 * var_terfc);
        let assign21500_e22835: f64 = (var_terfc * var_terfc);
        let assign21500_e22836: f64 = (var_berfc * assign21500_e22835);
        let assign21500_e22837: f64 = (assign21500_e22831 + assign21500_e22836);
        let assign21500_e22841: f64 = (var_terfc * var_terfc);
        let assign21500_e22843: f64 = (assign21500_e22841 * var_terfc);
        let assign21500_e22844: f64 = (var_cerfc * assign21500_e22843);
        let assign21500_e22845: f64 = (assign21500_e22837 + assign21500_e22844);
        let assign21500_e22847: f64 = (assign21500_e22845 * var_tmp);
        (assign21500_e22847, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign21500_e22841 * var_terfc_dn5)))) * var_tmp) + (assign21500_e22845 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign21500_e22841 * var_terfc_dn6)))) * var_tmp) + (assign21500_e22845 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign21500_e22841 * var_terfc_dn7)))) * var_tmp) + (assign21500_e22845 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign21500_e22841 * var_terfc_dn8)))) * var_tmp) + (assign21500_e22845 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign21500_e22849;
        var_erfcpos_dn5 = assign21500_e22849_d_n5;
        var_erfcpos_dn6 = assign21500_e22849_d_n6;
        var_erfcpos_dn7 = assign21500_e22849_d_n7;
        var_erfcpos_dn8 = assign21500_e22849_d_n8;

        let assign21510_e22852: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard384 = assign21510_e22852;

        let (assign21520_e22866, assign21520_e22866_d_n5, assign21520_e22866_d_n6, assign21520_e22866_d_n7, assign21520_e22866_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) && (var_guard380 == 0.0)) && (var_guard384 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign21520_e22866;
        var_erfctimesexpmtat_dn5 = assign21520_e22866_d_n5;
        var_erfctimesexpmtat_dn6 = assign21520_e22866_d_n6;
        var_erfctimesexpmtat_dn7 = assign21520_e22866_d_n7;
        var_erfctimesexpmtat_dn8 = assign21520_e22866_d_n8;

        let assign21530_e22869: f64 = (-230.25850929940458);
        let assign21530_e22870: f64 = if var_mtat > assign21530_e22869 { 1.0 } else { 0.0 };
        var_guard385 = assign21530_e22870;

        *var_asrh_slot = var_asrh;
        *var_asrh_dn5_slot = var_asrh_dn5;
        *var_asrh_dn6_slot = var_asrh_dn6;
        *var_asrh_dn7_slot = var_asrh_dn7;
        *var_asrh_dn8_slot = var_asrh_dn8;
        *var_btat_slot = var_btat;
        *var_btat_dn5_slot = var_btat_dn5;
        *var_btat_dn6_slot = var_btat_dn6;
        *var_btat_dn7_slot = var_btat_dn7;
        *var_btat_dn8_slot = var_btat_dn8;
        *var_dwsrh_slot = var_dwsrh;
        *var_erfcpos_slot = var_erfcpos;
        *var_erfcpos_dn5_slot = var_erfcpos_dn5;
        *var_erfcpos_dn6_slot = var_erfcpos_dn6;
        *var_erfcpos_dn7_slot = var_erfcpos_dn7;
        *var_erfcpos_dn8_slot = var_erfcpos_dn8;
        *var_erfctimesexpmtat_slot = var_erfctimesexpmtat;
        *var_erfctimesexpmtat_dn5_slot = var_erfctimesexpmtat_dn5;
        *var_erfctimesexpmtat_dn6_slot = var_erfctimesexpmtat_dn6;
        *var_erfctimesexpmtat_dn7_slot = var_erfctimesexpmtat_dn7;
        *var_erfctimesexpmtat_dn8_slot = var_erfctimesexpmtat_dn8;
        *var_guard378_slot = var_guard378;
        *var_guard379_slot = var_guard379;
        *var_guard380_slot = var_guard380;
        *var_guard381_slot = var_guard381;
        *var_guard382_slot = var_guard382;
        *var_guard383_slot = var_guard383;
        *var_guard384_slot = var_guard384;
        *var_guard385_slot = var_guard385;
        *var_isrh_slot = var_isrh;
        *var_isrh_dn5_slot = var_isrh_dn5;
        *var_isrh_dn6_slot = var_isrh_dn6;
        *var_isrh_dn7_slot = var_isrh_dn7;
        *var_isrh_dn8_slot = var_isrh_dn8;
        *var_itat_slot = var_itat;
        *var_itat_dn5_slot = var_itat_dn5;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_ktat_slot = var_ktat;
        *var_ktat_dn5_slot = var_ktat_dn5;
        *var_ktat_dn6_slot = var_ktat_dn6;
        *var_ktat_dn7_slot = var_ktat_dn7;
        *var_ktat_dn8_slot = var_ktat_dn8;
        *var_ltat_slot = var_ltat;
        *var_ltat_dn5_slot = var_ltat_dn5;
        *var_ltat_dn6_slot = var_ltat_dn6;
        *var_ltat_dn7_slot = var_ltat_dn7;
        *var_ltat_dn8_slot = var_ltat_dn8;
        *var_mtat_slot = var_mtat;
        *var_mtat_dn5_slot = var_mtat_dn5;
        *var_mtat_dn6_slot = var_mtat_dn6;
        *var_mtat_dn7_slot = var_mtat_dn7;
        *var_mtat_dn8_slot = var_mtat_dn8;
        *var_sqrtumax_slot = var_sqrtumax;
        *var_sqrtumax_dn5_slot = var_sqrtumax_dn5;
        *var_sqrtumax_dn6_slot = var_sqrtumax_dn6;
        *var_sqrtumax_dn7_slot = var_sqrtumax_dn7;
        *var_sqrtumax_dn8_slot = var_sqrtumax_dn8;
        *var_terfc_slot = var_terfc;
        *var_terfc_dn5_slot = var_terfc_dn5;
        *var_terfc_dn6_slot = var_terfc_dn6;
        *var_terfc_dn7_slot = var_terfc_dn7;
        *var_terfc_dn8_slot = var_terfc_dn8;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn5_slot = var_tmp_dn5;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_twoatatoverthreebtat_slot = var_twoatatoverthreebtat;
        *var_twoatatoverthreebtat_dn5_slot = var_twoatatoverthreebtat_dn5;
        *var_twoatatoverthreebtat_dn6_slot = var_twoatatoverthreebtat_dn6;
        *var_twoatatoverthreebtat_dn7_slot = var_twoatatoverthreebtat_dn7;
        *var_twoatatoverthreebtat_dn8_slot = var_twoatatoverthreebtat_dn8;
        *var_umax_slot = var_umax;
        *var_umax_dn5_slot = var_umax_dn5;
        *var_umax_dn6_slot = var_umax_dn6;
        *var_umax_dn7_slot = var_umax_dn7;
        *var_umax_dn8_slot = var_umax_dn8;
        *var_umaxbeforelimiting_slot = var_umaxbeforelimiting;
        *var_umaxbeforelimiting_dn5_slot = var_umaxbeforelimiting_dn5;
        *var_umaxbeforelimiting_dn6_slot = var_umaxbeforelimiting_dn6;
        *var_umaxbeforelimiting_dn7_slot = var_umaxbeforelimiting_dn7;
        *var_umaxbeforelimiting_dn8_slot = var_umaxbeforelimiting_dn8;
        *var_umaxpoweronepointfive_slot = var_umaxpoweronepointfive;
        *var_umaxpoweronepointfive_dn5_slot = var_umaxpoweronepointfive_dn5;
        *var_umaxpoweronepointfive_dn6_slot = var_umaxpoweronepointfive_dn6;
        *var_umaxpoweronepointfive_dn7_slot = var_umaxpoweronepointfive_dn7;
        *var_umaxpoweronepointfive_dn8_slot = var_umaxpoweronepointfive_dn8;
        *var_wdep_slot = var_wdep;
        *var_wdep_dn5_slot = var_wdep_dn5;
        *var_wdep_dn6_slot = var_wdep_dn6;
        *var_wdep_dn7_slot = var_wdep_dn7;
        *var_wdep_dn8_slot = var_wdep_dn8;
        *var_wgamma_slot = var_wgamma;
        *var_wgamma_dn5_slot = var_wgamma_dn5;
        *var_wgamma_dn6_slot = var_wgamma_dn6;
        *var_wgamma_dn7_slot = var_wgamma_dn7;
        *var_wgamma_dn8_slot = var_wgamma_dn8;
        *var_wsrh_slot = var_wsrh;
        *var_wsrhstep_slot = var_wsrhstep;
        *var_wtat_slot = var_wtat;
        *var_wtat_dn5_slot = var_wtat_dn5;
        *var_wtat_dn6_slot = var_wtat_dn6;
        *var_wtat_dn7_slot = var_wtat_dn7;
        *var_wtat_dn8_slot = var_wtat_dn8;
        *var_xerfc_slot = var_xerfc;
        *var_xerfc_dn5_slot = var_xerfc_dn5;
        *var_xerfc_dn6_slot = var_xerfc_dn6;
        *var_xerfc_dn7_slot = var_xerfc_dn7;
        *var_xerfc_dn8_slot = var_xerfc_dn8;
        *var_ysq_slot = var_ysq;
        *var_ysq_dn5_slot = var_ysq_dn5;
        *var_ysq_dn6_slot = var_ysq_dn6;
        *var_ysq_dn7_slot = var_ysq_dn7;
        *var_ysq_dn8_slot = var_ysq_dn8;
    }

    pub(super) fn stamp_transient_block_38(
        p: &Parameters,
        var_alphaav: f64,
        var_asrh: f64,
        var_asrh_dn5: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_atatsti: f64,
        var_erfcpos: f64,
        var_erfcpos_dn5: f64,
        var_erfcpos_dn6: f64,
        var_erfcpos_dn7: f64,
        var_erfcpos_dn8: f64,
        var_fbbtsti: f64,
        var_fstopsti: f64,
        var_guard176: f64,
        var_guard193: f64,
        var_guard376: f64,
        var_guard380: f64,
        var_guard384: f64,
        var_guard385: f64,
        var_idmult: f64,
        var_idsatgat: f64,
        var_ktat: f64,
        var_ktat_dn5: f64,
        var_ktat_dn6: f64,
        var_ktat_dn7: f64,
        var_ktat_dn8: f64,
        var_lgsource_i: f64,
        var_mtat: f64,
        var_mtat_dn5: f64,
        var_mtat_dn6: f64,
        var_mtat_dn7: f64,
        var_mtat_dn8: f64,
        var_one_over_one_minus_psti: f64,
        var_slopesti: f64,
        var_two_psistar: f64,
        var_v3: f64,
        var_vav: f64,
        var_vbbt: f64,
        var_vbigat: f64,
        var_vbirgatinv: f64,
        var_vbirstiinv: f64,
        var_vbrinvsti: f64,
        var_vjsrh: f64,
        var_wdepnulrinvsti: f64,
        var_wtat: f64,
        var_wtat_dn5: f64,
        var_wtat_dn6: f64,
        var_wtat_dn7: f64,
        var_wtat_dn8: f64,
        var_dwsrh_slot: &mut f64,
        var_erfctimesexpmtat_slot: &mut f64,
        var_erfctimesexpmtat_dn5_slot: &mut f64,
        var_erfctimesexpmtat_dn6_slot: &mut f64,
        var_erfctimesexpmtat_dn7_slot: &mut f64,
        var_erfctimesexpmtat_dn8_slot: &mut f64,
        var_fbreakdown_slot: &mut f64,
        var_fbreakdown_dn5_slot: &mut f64,
        var_fbreakdown_dn6_slot: &mut f64,
        var_fbreakdown_dn7_slot: &mut f64,
        var_fbreakdown_dn8_slot: &mut f64,
        var_fmaxr_slot: &mut f64,
        var_fmaxr_dn5_slot: &mut f64,
        var_fmaxr_dn6_slot: &mut f64,
        var_fmaxr_dn7_slot: &mut f64,
        var_fmaxr_dn8_slot: &mut f64,
        var_gammamax_slot: &mut f64,
        var_gammamax_dn5_slot: &mut f64,
        var_gammamax_dn6_slot: &mut f64,
        var_gammamax_dn7_slot: &mut f64,
        var_gammamax_dn8_slot: &mut f64,
        var_guard386_slot: &mut f64,
        var_guard387_slot: &mut f64,
        var_guard388_slot: &mut f64,
        var_guard389_slot: &mut f64,
        var_guard390_slot: &mut f64,
        var_guard391_slot: &mut f64,
        var_guard392_slot: &mut f64,
        var_guard393_slot: &mut f64,
        var_guard394_slot: &mut f64,
        var_guard395_slot: &mut f64,
        var_guard396_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn5_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
        var_id__blk213_slot: &mut f64,
        var_ijungat_slot: &mut f64,
        var_ijungat_dn5_slot: &mut f64,
        var_ijungat_dn6_slot: &mut f64,
        var_ijungat_dn7_slot: &mut f64,
        var_ijungat_dn8_slot: &mut f64,
        var_ijunsti_slot: &mut f64,
        var_ijunsti_dn5_slot: &mut f64,
        var_ijunsti_dn6_slot: &mut f64,
        var_ijunsti_dn7_slot: &mut f64,
        var_ijunsti_dn8_slot: &mut f64,
        var_isrh_slot: &mut f64,
        var_isrh_dn5_slot: &mut f64,
        var_isrh_dn6_slot: &mut f64,
        var_isrh_dn7_slot: &mut f64,
        var_isrh_dn8_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn5_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn5_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_vbi_minus_vjsrh_slot: &mut f64,
        var_wsrh_slot: &mut f64,
        var_wsrhstep_slot: &mut f64,
    ) {
        let mut var_dwsrh: f64 = *var_dwsrh_slot;
        let mut var_erfctimesexpmtat: f64 = *var_erfctimesexpmtat_slot;
        let mut var_erfctimesexpmtat_dn5: f64 = *var_erfctimesexpmtat_dn5_slot;
        let mut var_erfctimesexpmtat_dn6: f64 = *var_erfctimesexpmtat_dn6_slot;
        let mut var_erfctimesexpmtat_dn7: f64 = *var_erfctimesexpmtat_dn7_slot;
        let mut var_erfctimesexpmtat_dn8: f64 = *var_erfctimesexpmtat_dn8_slot;
        let mut var_fbreakdown: f64 = *var_fbreakdown_slot;
        let mut var_fbreakdown_dn5: f64 = *var_fbreakdown_dn5_slot;
        let mut var_fbreakdown_dn6: f64 = *var_fbreakdown_dn6_slot;
        let mut var_fbreakdown_dn7: f64 = *var_fbreakdown_dn7_slot;
        let mut var_fbreakdown_dn8: f64 = *var_fbreakdown_dn8_slot;
        let mut var_fmaxr: f64 = *var_fmaxr_slot;
        let mut var_fmaxr_dn5: f64 = *var_fmaxr_dn5_slot;
        let mut var_fmaxr_dn6: f64 = *var_fmaxr_dn6_slot;
        let mut var_fmaxr_dn7: f64 = *var_fmaxr_dn7_slot;
        let mut var_fmaxr_dn8: f64 = *var_fmaxr_dn8_slot;
        let mut var_gammamax: f64 = *var_gammamax_slot;
        let mut var_gammamax_dn5: f64 = *var_gammamax_dn5_slot;
        let mut var_gammamax_dn6: f64 = *var_gammamax_dn6_slot;
        let mut var_gammamax_dn7: f64 = *var_gammamax_dn7_slot;
        let mut var_gammamax_dn8: f64 = *var_gammamax_dn8_slot;
        let mut var_guard386: f64 = *var_guard386_slot;
        let mut var_guard387: f64 = *var_guard387_slot;
        let mut var_guard388: f64 = *var_guard388_slot;
        let mut var_guard389: f64 = *var_guard389_slot;
        let mut var_guard390: f64 = *var_guard390_slot;
        let mut var_guard391: f64 = *var_guard391_slot;
        let mut var_guard392: f64 = *var_guard392_slot;
        let mut var_guard393: f64 = *var_guard393_slot;
        let mut var_guard394: f64 = *var_guard394_slot;
        let mut var_guard395: f64 = *var_guard395_slot;
        let mut var_guard396: f64 = *var_guard396_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn5: f64 = *var_ibbt_dn5_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
        let mut var_id__blk213: f64 = *var_id__blk213_slot;
        let mut var_ijungat: f64 = *var_ijungat_slot;
        let mut var_ijungat_dn5: f64 = *var_ijungat_dn5_slot;
        let mut var_ijungat_dn6: f64 = *var_ijungat_dn6_slot;
        let mut var_ijungat_dn7: f64 = *var_ijungat_dn7_slot;
        let mut var_ijungat_dn8: f64 = *var_ijungat_dn8_slot;
        let mut var_ijunsti: f64 = *var_ijunsti_slot;
        let mut var_ijunsti_dn5: f64 = *var_ijunsti_dn5_slot;
        let mut var_ijunsti_dn6: f64 = *var_ijunsti_dn6_slot;
        let mut var_ijunsti_dn7: f64 = *var_ijunsti_dn7_slot;
        let mut var_ijunsti_dn8: f64 = *var_ijunsti_dn8_slot;
        let mut var_isrh: f64 = *var_isrh_slot;
        let mut var_isrh_dn5: f64 = *var_isrh_dn5_slot;
        let mut var_isrh_dn6: f64 = *var_isrh_dn6_slot;
        let mut var_isrh_dn7: f64 = *var_isrh_dn7_slot;
        let mut var_isrh_dn8: f64 = *var_isrh_dn8_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn5: f64 = *var_itat_dn5_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn5: f64 = *var_tmp_dn5_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_vbi_minus_vjsrh: f64 = *var_vbi_minus_vjsrh_slot;
        let mut var_wsrh: f64 = *var_wsrh_slot;
        let mut var_wsrhstep: f64 = *var_wsrhstep_slot;

        let (assign21540_e22888, assign21540_e22888_d_n5, assign21540_e22888_d_n6, assign21540_e22888_d_n7, assign21540_e22888_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) && (var_guard380 == 0.0)) && (var_guard384 == 0.0)) && (var_guard385 != 0.0)) {
        let assign21540_e22886: f64 = (var_mtat).exp();
        (assign21540_e22886, (assign21540_e22886 * var_mtat_dn5), (assign21540_e22886 * var_mtat_dn6), (assign21540_e22886 * var_mtat_dn7), (assign21540_e22886 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign21540_e22888;
        var_tmp_dn5 = assign21540_e22888_d_n5;
        var_tmp_dn6 = assign21540_e22888_d_n6;
        var_tmp_dn7 = assign21540_e22888_d_n7;
        var_tmp_dn8 = assign21540_e22888_d_n8;

        let (assign21550_e22931, assign21550_e22931_d_n5, assign21550_e22931_d_n6, assign21550_e22931_d_n7, assign21550_e22931_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) && (var_guard380 == 0.0)) && (var_guard384 == 0.0)) && (var_guard385 == 0.0)) {
        let assign21550_e22907: f64 = (-230.25850929940458);
        let assign21550_e22909: f64 = (assign21550_e22907 - var_mtat);
        let assign21550_e22913: f64 = (-230.25850929940458);
        let assign21550_e22915: f64 = (assign21550_e22913 - var_mtat);
        let assign21550_e22918: f64 = (-230.25850929940458);
        let assign21550_e22920: f64 = (assign21550_e22918 - var_mtat);
        let assign21550_e22922: f64 = (assign21550_e22920 * 0.3333333333333333);
        let assign21550_e22923: f64 = (1.0 + assign21550_e22922);
        let assign21550_e22924: f64 = (assign21550_e22915 * assign21550_e22923);
        let assign21550_e22925: f64 = (0.5 * assign21550_e22924);
        let assign21550_e22926: f64 = (1.0 + assign21550_e22925);
        let assign21550_e22927: f64 = (assign21550_e22909 * assign21550_e22926);
        let assign21550_e22928: f64 = (1.0 + assign21550_e22927);
        let assign21550_e22929: f64 = (1e-100 / assign21550_e22928);
        (assign21550_e22929, (-((1e-100 * (((-var_mtat_dn5) * assign21550_e22926) + (assign21550_e22909 * (0.5 * (((-var_mtat_dn5) * assign21550_e22923) + (assign21550_e22915 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign21550_e22928 * assign21550_e22928))), (-((1e-100 * (((-var_mtat_dn6) * assign21550_e22926) + (assign21550_e22909 * (0.5 * (((-var_mtat_dn6) * assign21550_e22923) + (assign21550_e22915 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign21550_e22928 * assign21550_e22928))), (-((1e-100 * (((-var_mtat_dn7) * assign21550_e22926) + (assign21550_e22909 * (0.5 * (((-var_mtat_dn7) * assign21550_e22923) + (assign21550_e22915 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign21550_e22928 * assign21550_e22928))), (-((1e-100 * (((-var_mtat_dn8) * assign21550_e22926) + (assign21550_e22909 * (0.5 * (((-var_mtat_dn8) * assign21550_e22923) + (assign21550_e22915 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign21550_e22928 * assign21550_e22928))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign21550_e22931;
        var_tmp_dn5 = assign21550_e22931_d_n5;
        var_tmp_dn6 = assign21550_e22931_d_n6;
        var_tmp_dn7 = assign21550_e22931_d_n7;
        var_tmp_dn8 = assign21550_e22931_d_n8;

        let (assign21560_e22950, assign21560_e22950_d_n5, assign21560_e22950_d_n6, assign21560_e22950_d_n7, assign21560_e22950_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) && (var_guard380 == 0.0)) && (var_guard384 == 0.0)) {
        let assign21560_e22946: f64 = (2.0 * var_tmp);
        let assign21560_e22948: f64 = (assign21560_e22946 - var_erfcpos);
        (assign21560_e22948, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign21560_e22950;
        var_erfctimesexpmtat_dn5 = assign21560_e22950_d_n5;
        var_erfctimesexpmtat_dn6 = assign21560_e22950_d_n6;
        var_erfctimesexpmtat_dn7 = assign21560_e22950_d_n7;
        var_erfctimesexpmtat_dn8 = assign21560_e22950_d_n8;

        let (assign21570_e22970, assign21570_e22970_d_n5, assign21570_e22970_d_n6, assign21570_e22970_d_n7, assign21570_e22970_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) && (var_guard380 == 0.0)) {
        let assign21570_e22962: f64 = (1.772453850905516 * 0.5);
        let assign21570_e22965: f64 = (var_atatsti * var_erfctimesexpmtat);
        let assign21570_e22967: f64 = (assign21570_e22965 / var_ktat);
        let assign21570_e22968: f64 = (assign21570_e22962 * assign21570_e22967);
        (assign21570_e22968, (assign21570_e22962 * ((((var_atatsti * var_erfctimesexpmtat_dn5) * var_ktat) - (assign21570_e22965 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign21570_e22962 * ((((var_atatsti * var_erfctimesexpmtat_dn6) * var_ktat) - (assign21570_e22965 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign21570_e22962 * ((((var_atatsti * var_erfctimesexpmtat_dn7) * var_ktat) - (assign21570_e22965 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign21570_e22962 * ((((var_atatsti * var_erfctimesexpmtat_dn8) * var_ktat) - (assign21570_e22965 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign21570_e22970;
        var_gammamax_dn5 = assign21570_e22970_d_n5;
        var_gammamax_dn6 = assign21570_e22970_d_n6;
        var_gammamax_dn7 = assign21570_e22970_d_n7;
        var_gammamax_dn8 = assign21570_e22970_d_n8;

        let (assign21580_e22988, assign21580_e22988_d_n5, assign21580_e22988_d_n6, assign21580_e22988_d_n7, assign21580_e22988_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) && (var_guard380 == 0.0)) {
        let assign21580_e22983: f64 = (var_asrh * var_gammamax);
        let assign21580_e22985: f64 = (assign21580_e22983 * var_wtat);
        let assign21580_e22986: f64 = (p.p839 * assign21580_e22985);
        (assign21580_e22986, (p.p839 * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign21580_e22983 * var_wtat_dn5))), (p.p839 * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign21580_e22983 * var_wtat_dn6))), (p.p839 * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign21580_e22983 * var_wtat_dn7))), (p.p839 * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign21580_e22983 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign21580_e22988;
        var_itat_dn5 = assign21580_e22988_d_n5;
        var_itat_dn6 = assign21580_e22988_d_n6;
        var_itat_dn7 = assign21580_e22988_d_n7;
        var_itat_dn8 = assign21580_e22988_d_n8;

        let assign21590_e22991: f64 = if p.p845 == 0.0 { 1.0 } else { 0.0 };
        var_guard386 = assign21590_e22991;

        let (assign21600_e23002, assign21600_e23002_d_n5, assign21600_e23002_d_n6, assign21600_e23002_d_n7, assign21600_e23002_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) && (var_guard386 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign21600_e23002;
        var_ibbt_dn5 = assign21600_e23002_d_n5;
        var_ibbt_dn6 = assign21600_e23002_d_n6;
        var_ibbt_dn7 = assign21600_e23002_d_n7;
        var_ibbt_dn8 = assign21600_e23002_d_n8;

        let assign21610_e23005: f64 = if p.p825 == 0.5 { 1.0 } else { 0.0 };
        var_guard387 = assign21610_e23005;

        let (assign21620_e23024, assign21620_e23024_d_n5, assign21620_e23024_d_n6, assign21620_e23024_d_n7, assign21620_e23024_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) && (var_guard386 == 0.0)) && (var_guard387 != 0.0)) {
        let assign21620_e23019: f64 = (p.p822 - var_vbbt);
        let assign21620_e23021: f64 = (assign21620_e23019 * var_vbirstiinv);
        let assign21620_e23022: f64 = (assign21620_e23021).sqrt();
        (assign21620_e23022, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign21620_e23024;
        var_tmp_dn5 = assign21620_e23024_d_n5;
        var_tmp_dn6 = assign21620_e23024_d_n6;
        var_tmp_dn7 = assign21620_e23024_d_n7;
        var_tmp_dn8 = assign21620_e23024_d_n8;

        let (assign21630_e23045, assign21630_e23045_d_n5, assign21630_e23045_d_n6, assign21630_e23045_d_n7, assign21630_e23045_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) && (var_guard386 == 0.0)) && (var_guard387 == 0.0)) {
        let assign21630_e23039: f64 = (p.p822 - var_vbbt);
        let assign21630_e23041: f64 = (assign21630_e23039 * var_vbirstiinv);
        let assign21630_e23043: f64 = (assign21630_e23041).powf(p.p825);
        (assign21630_e23043, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign21630_e23045;
        var_tmp_dn5 = assign21630_e23045_d_n5;
        var_tmp_dn6 = assign21630_e23045_d_n6;
        var_tmp_dn7 = assign21630_e23045_d_n7;
        var_tmp_dn8 = assign21630_e23045_d_n8;

        let (assign21640_e23065, assign21640_e23065_d_n5, assign21640_e23065_d_n6, assign21640_e23065_d_n7, assign21640_e23065_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) && (var_guard386 == 0.0)) {
        let assign21640_e23058: f64 = (p.p822 - var_vbbt);
        let assign21640_e23060: f64 = (assign21640_e23058 * var_wdepnulrinvsti);
        let assign21640_e23062: f64 = (assign21640_e23060 / var_tmp);
        let assign21640_e23063: f64 = (var_one_over_one_minus_psti * assign21640_e23062);
        (assign21640_e23063, (var_one_over_one_minus_psti * (-((assign21640_e23060 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti * (-((assign21640_e23060 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti * (-((assign21640_e23060 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti * (-((assign21640_e23060 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign21640_e23065;
        var_fmaxr_dn5 = assign21640_e23065_d_n5;
        var_fmaxr_dn6 = assign21640_e23065_d_n6;
        var_fmaxr_dn7 = assign21640_e23065_d_n7;
        var_fmaxr_dn8 = assign21640_e23065_d_n8;

        let assign21650_e23067: f64 = (-var_fbbtsti);
        let assign21650_e23069: f64 = (assign21650_e23067 / var_fmaxr);
        let assign21650_e23070: f64 = (assign21650_e23069).abs();
        let assign21650_e23072: f64 = if assign21650_e23070 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard388 = assign21650_e23072;

        let (assign21660_e23090, assign21660_e23090_d_n5, assign21660_e23090_d_n6, assign21660_e23090_d_n7, assign21660_e23090_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) && (var_guard386 == 0.0)) && (var_guard388 != 0.0)) {
        let assign21660_e23085: f64 = (-var_fbbtsti);
        let assign21660_e23087: f64 = (assign21660_e23085 / var_fmaxr);
        let assign21660_e23088: f64 = (assign21660_e23087).exp();
        (assign21660_e23088, (assign21660_e23088 * (-((assign21660_e23085 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))), (assign21660_e23088 * (-((assign21660_e23085 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign21660_e23088 * (-((assign21660_e23085 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign21660_e23088 * (-((assign21660_e23085 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign21660_e23090;
        var_tmp_dn5 = assign21660_e23090_d_n5;
        var_tmp_dn6 = assign21660_e23090_d_n6;
        var_tmp_dn7 = assign21660_e23090_d_n7;
        var_tmp_dn8 = assign21660_e23090_d_n8;

        let assign21670_e23092: f64 = (-var_fbbtsti);
        let assign21670_e23094: f64 = (assign21670_e23092 / var_fmaxr);
        let assign21670_e23096: f64 = if assign21670_e23094 < 0.0 { 1.0 } else { 0.0 };
        var_guard389 = assign21670_e23096;

        let (assign21680_e23147, assign21680_e23147_d_n5, assign21680_e23147_d_n6, assign21680_e23147_d_n7, assign21680_e23147_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) && (var_guard386 == 0.0)) && (var_guard388 == 0.0)) && (var_guard389 != 0.0)) {
        let assign21680_e23114: f64 = (-230.25850929940458);
        let assign21680_e23116: f64 = (-var_fbbtsti);
        let assign21680_e23118: f64 = (assign21680_e23116 / var_fmaxr);
        let assign21680_e23119: f64 = (assign21680_e23114 - assign21680_e23118);
        let assign21680_e23123: f64 = (-230.25850929940458);
        let assign21680_e23125: f64 = (-var_fbbtsti);
        let assign21680_e23127: f64 = (assign21680_e23125 / var_fmaxr);
        let assign21680_e23128: f64 = (assign21680_e23123 - assign21680_e23127);
        let assign21680_e23131: f64 = (-230.25850929940458);
        let assign21680_e23133: f64 = (-var_fbbtsti);
        let assign21680_e23135: f64 = (assign21680_e23133 / var_fmaxr);
        let assign21680_e23136: f64 = (assign21680_e23131 - assign21680_e23135);
        let assign21680_e23138: f64 = (assign21680_e23136 * 0.3333333333333333);
        let assign21680_e23139: f64 = (1.0 + assign21680_e23138);
        let assign21680_e23140: f64 = (assign21680_e23128 * assign21680_e23139);
        let assign21680_e23141: f64 = (0.5 * assign21680_e23140);
        let assign21680_e23142: f64 = (1.0 + assign21680_e23141);
        let assign21680_e23143: f64 = (assign21680_e23119 * assign21680_e23142);
        let assign21680_e23144: f64 = (1.0 + assign21680_e23143);
        let assign21680_e23145: f64 = (1e-100 / assign21680_e23144);
        (assign21680_e23145, (-((1e-100 * (((-(-((assign21680_e23116 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign21680_e23142) + (assign21680_e23119 * (0.5 * (((-(-((assign21680_e23125 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign21680_e23139) + (assign21680_e23128 * ((-(-((assign21680_e23133 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign21680_e23144 * assign21680_e23144))), (-((1e-100 * (((-(-((assign21680_e23116 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign21680_e23142) + (assign21680_e23119 * (0.5 * (((-(-((assign21680_e23125 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign21680_e23139) + (assign21680_e23128 * ((-(-((assign21680_e23133 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign21680_e23144 * assign21680_e23144))), (-((1e-100 * (((-(-((assign21680_e23116 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign21680_e23142) + (assign21680_e23119 * (0.5 * (((-(-((assign21680_e23125 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign21680_e23139) + (assign21680_e23128 * ((-(-((assign21680_e23133 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign21680_e23144 * assign21680_e23144))), (-((1e-100 * (((-(-((assign21680_e23116 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign21680_e23142) + (assign21680_e23119 * (0.5 * (((-(-((assign21680_e23125 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign21680_e23139) + (assign21680_e23128 * ((-(-((assign21680_e23133 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign21680_e23144 * assign21680_e23144))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign21680_e23147;
        var_tmp_dn5 = assign21680_e23147_d_n5;
        var_tmp_dn6 = assign21680_e23147_d_n6;
        var_tmp_dn7 = assign21680_e23147_d_n7;
        var_tmp_dn8 = assign21680_e23147_d_n8;

        let (assign21690_e23196, assign21690_e23196_d_n5, assign21690_e23196_d_n6, assign21690_e23196_d_n7, assign21690_e23196_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) && (var_guard386 == 0.0)) && (var_guard388 == 0.0)) && (var_guard389 == 0.0)) {
        let assign21690_e23166: f64 = (-var_fbbtsti);
        let assign21690_e23168: f64 = (assign21690_e23166 / var_fmaxr);
        let assign21690_e23170: f64 = (assign21690_e23168 - 230.25850929940458);
        let assign21690_e23174: f64 = (-var_fbbtsti);
        let assign21690_e23176: f64 = (assign21690_e23174 / var_fmaxr);
        let assign21690_e23178: f64 = (assign21690_e23176 - 230.25850929940458);
        let assign21690_e23181: f64 = (-var_fbbtsti);
        let assign21690_e23183: f64 = (assign21690_e23181 / var_fmaxr);
        let assign21690_e23185: f64 = (assign21690_e23183 - 230.25850929940458);
        let assign21690_e23187: f64 = (assign21690_e23185 * 0.3333333333333333);
        let assign21690_e23188: f64 = (1.0 + assign21690_e23187);
        let assign21690_e23189: f64 = (assign21690_e23178 * assign21690_e23188);
        let assign21690_e23190: f64 = (0.5 * assign21690_e23189);
        let assign21690_e23191: f64 = (1.0 + assign21690_e23190);
        let assign21690_e23192: f64 = (assign21690_e23170 * assign21690_e23191);
        let assign21690_e23193: f64 = (1.0 + assign21690_e23192);
        let assign21690_e23194: f64 = (1e100 * assign21690_e23193);
        (assign21690_e23194, (1e100 * (((-((assign21690_e23166 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign21690_e23191) + (assign21690_e23170 * (0.5 * (((-((assign21690_e23174 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign21690_e23188) + (assign21690_e23178 * ((-((assign21690_e23181 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign21690_e23166 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign21690_e23191) + (assign21690_e23170 * (0.5 * (((-((assign21690_e23174 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign21690_e23188) + (assign21690_e23178 * ((-((assign21690_e23181 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign21690_e23166 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign21690_e23191) + (assign21690_e23170 * (0.5 * (((-((assign21690_e23174 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign21690_e23188) + (assign21690_e23178 * ((-((assign21690_e23181 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign21690_e23166 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign21690_e23191) + (assign21690_e23170 * (0.5 * (((-((assign21690_e23174 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign21690_e23188) + (assign21690_e23178 * ((-((assign21690_e23181 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign21690_e23196;
        var_tmp_dn5 = assign21690_e23196_d_n5;
        var_tmp_dn6 = assign21690_e23196_d_n6;
        var_tmp_dn7 = assign21690_e23196_d_n7;
        var_tmp_dn8 = assign21690_e23196_d_n8;

        let (assign21700_e23216, assign21700_e23216_d_n5, assign21700_e23216_d_n6, assign21700_e23216_d_n7, assign21700_e23216_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) && (var_guard386 == 0.0)) {
        let assign21700_e23209: f64 = (var_v3 * var_fmaxr);
        let assign21700_e23211: f64 = (assign21700_e23209 * var_fmaxr);
        let assign21700_e23213: f64 = (assign21700_e23211 * var_tmp);
        let assign21700_e23214: f64 = (p.p845 * assign21700_e23213);
        (assign21700_e23214, (p.p845 * (((((var_v3 * var_fmaxr_dn5) * var_fmaxr) + (assign21700_e23209 * var_fmaxr_dn5)) * var_tmp) + (assign21700_e23211 * var_tmp_dn5))), (p.p845 * (((((var_v3 * var_fmaxr_dn6) * var_fmaxr) + (assign21700_e23209 * var_fmaxr_dn6)) * var_tmp) + (assign21700_e23211 * var_tmp_dn6))), (p.p845 * (((((var_v3 * var_fmaxr_dn7) * var_fmaxr) + (assign21700_e23209 * var_fmaxr_dn7)) * var_tmp) + (assign21700_e23211 * var_tmp_dn7))), (p.p845 * (((((var_v3 * var_fmaxr_dn8) * var_fmaxr) + (assign21700_e23209 * var_fmaxr_dn8)) * var_tmp) + (assign21700_e23211 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign21700_e23216;
        var_ibbt_dn5 = assign21700_e23216_d_n5;
        var_ibbt_dn6 = assign21700_e23216_d_n6;
        var_ibbt_dn7 = assign21700_e23216_d_n7;
        var_ibbt_dn8 = assign21700_e23216_d_n8;

        let assign21710_e23219: f64 = if p.p854 > 1000.0 { 1.0 } else { 0.0 };
        var_guard390 = assign21710_e23219;

        let (assign21720_e23230, assign21720_e23230_d_n5, assign21720_e23230_d_n6, assign21720_e23230_d_n7, assign21720_e23230_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) && (var_guard390 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign21720_e23230;
        var_fbreakdown_dn5 = assign21720_e23230_d_n5;
        var_fbreakdown_dn6 = assign21720_e23230_d_n6;
        var_fbreakdown_dn7 = assign21720_e23230_d_n7;
        var_fbreakdown_dn8 = assign21720_e23230_d_n8;

        let assign21730_e23233: f64 = (-var_alphaav);
        let assign21730_e23235: f64 = (assign21730_e23233 * p.p854);
        let assign21730_e23236: f64 = if var_vav > assign21730_e23235 { 1.0 } else { 0.0 };
        var_guard391 = assign21730_e23236;

        let assign21740_e23239: f64 = if p.p857 == 4.0 { 1.0 } else { 0.0 };
        var_guard392 = assign21740_e23239;

        let (assign21750_e23269, assign21750_e23269_d_n5, assign21750_e23269_d_n6, assign21750_e23269_d_n7, assign21750_e23269_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) && (var_guard390 == 0.0)) && (var_guard391 != 0.0)) && (var_guard392 != 0.0)) {
        let assign21750_e23255: f64 = (var_vav * var_vbrinvsti);
        let assign21750_e23258: f64 = (var_vav * var_vbrinvsti);
        let assign21750_e23259: f64 = (assign21750_e23255 * assign21750_e23258);
        let assign21750_e23262: f64 = (var_vav * var_vbrinvsti);
        let assign21750_e23263: f64 = (assign21750_e23259 * assign21750_e23262);
        let assign21750_e23266: f64 = (var_vav * var_vbrinvsti);
        let assign21750_e23267: f64 = (assign21750_e23263 * assign21750_e23266);
        (assign21750_e23267, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign21750_e23269;
        var_tmp_dn5 = assign21750_e23269_d_n5;
        var_tmp_dn6 = assign21750_e23269_d_n6;
        var_tmp_dn7 = assign21750_e23269_d_n7;
        var_tmp_dn8 = assign21750_e23269_d_n8;

        let (assign21760_e23291, assign21760_e23291_d_n5, assign21760_e23291_d_n6, assign21760_e23291_d_n7, assign21760_e23291_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) && (var_guard390 == 0.0)) && (var_guard391 != 0.0)) && (var_guard392 == 0.0)) {
        let assign21760_e23286: f64 = (var_vav * var_vbrinvsti);
        let assign21760_e23287: f64 = (assign21760_e23286).abs();
        let assign21760_e23289: f64 = (assign21760_e23287).powf(p.p857);
        (assign21760_e23289, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign21760_e23291;
        var_tmp_dn5 = assign21760_e23291_d_n5;
        var_tmp_dn6 = assign21760_e23291_d_n6;
        var_tmp_dn7 = assign21760_e23291_d_n7;
        var_tmp_dn8 = assign21760_e23291_d_n8;

        let (assign21770_e23309, assign21770_e23309_d_n5, assign21770_e23309_d_n6, assign21770_e23309_d_n7, assign21770_e23309_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) && (var_guard390 == 0.0)) && (var_guard391 != 0.0)) {
        let assign21770_e23306: f64 = (1.0 - var_tmp);
        let assign21770_e23307: f64 = (1.0 / assign21770_e23306);
        (assign21770_e23307, (-((-var_tmp_dn5) / (assign21770_e23306 * assign21770_e23306))), (-((-var_tmp_dn6) / (assign21770_e23306 * assign21770_e23306))), (-((-var_tmp_dn7) / (assign21770_e23306 * assign21770_e23306))), (-((-var_tmp_dn8) / (assign21770_e23306 * assign21770_e23306))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign21770_e23309;
        var_fbreakdown_dn5 = assign21770_e23309_d_n5;
        var_fbreakdown_dn6 = assign21770_e23309_d_n6;
        var_fbreakdown_dn7 = assign21770_e23309_d_n7;
        var_fbreakdown_dn8 = assign21770_e23309_d_n8;

        let (assign21780_e23332, assign21780_e23332_d_n5, assign21780_e23332_d_n6, assign21780_e23332_d_n7, assign21780_e23332_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) && (var_guard390 == 0.0)) && (var_guard391 == 0.0)) {
        let assign21780_e23326: f64 = (var_alphaav * p.p854);
        let assign21780_e23327: f64 = (var_vav + assign21780_e23326);
        let assign21780_e23329: f64 = (assign21780_e23327 * var_slopesti);
        let assign21780_e23330: f64 = (var_fstopsti + assign21780_e23329);
        (assign21780_e23330, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign21780_e23332;
        var_fbreakdown_dn5 = assign21780_e23332_d_n5;
        var_fbreakdown_dn6 = assign21780_e23332_d_n6;
        var_fbreakdown_dn7 = assign21780_e23332_d_n7;
        var_fbreakdown_dn8 = assign21780_e23332_d_n8;

        let (assign21790_e23351, assign21790_e23351_d_n5, assign21790_e23351_d_n6, assign21790_e23351_d_n7, assign21790_e23351_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard376 == 0.0)) {
        let assign21790_e23342: f64 = (var_id__blk213 + var_isrh);
        let assign21790_e23344: f64 = (assign21790_e23342 + var_itat);
        let assign21790_e23346: f64 = (assign21790_e23344 + var_ibbt);
        let assign21790_e23347: f64 = (p.p29 * assign21790_e23346);
        let assign21790_e23349: f64 = (assign21790_e23347 * var_fbreakdown);
        (assign21790_e23349, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign21790_e23347 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign21790_e23347 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign21790_e23347 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign21790_e23347 * var_fbreakdown_dn8)),)
    } else {
        (var_ijunsti, var_ijunsti_dn5, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8,)
    }
};
        var_ijunsti = assign21790_e23351;
        var_ijunsti_dn5 = assign21790_e23351_d_n5;
        var_ijunsti_dn6 = assign21790_e23351_d_n6;
        var_ijunsti_dn7 = assign21790_e23351_d_n7;
        var_ijunsti_dn8 = assign21790_e23351_d_n8;

        let assign21800_e23354: f64 = if var_lgsource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard393 = assign21800_e23354;

        let (assign21810_e23362, assign21810_e23362_d_n5, assign21810_e23362_d_n6, assign21810_e23362_d_n7, assign21810_e23362_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijungat, var_ijungat_dn5, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8,)
    }
};
        var_ijungat = assign21810_e23362;
        var_ijungat_dn5 = assign21810_e23362_d_n5;
        var_ijungat_dn6 = assign21810_e23362_d_n6;
        var_ijungat_dn7 = assign21810_e23362_d_n7;
        var_ijungat_dn8 = assign21810_e23362_d_n8;

        let (assign21820_e23373,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) {
        let assign21820_e23371: f64 = (var_idsatgat * var_idmult);
        (assign21820_e23371,)
    } else {
        (var_id__blk213,)
    }
};
        var_id__blk213 = assign21820_e23373;

        let assign21830_e23380: f64 = if ((p.p835 == 0.0) && (p.p840 == 0.0)) { 1.0 } else { 0.0 };
        var_guard394 = assign21830_e23380;

        let (assign21840_e23391, assign21840_e23391_d_n5, assign21840_e23391_d_n6, assign21840_e23391_d_n7, assign21840_e23391_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) && (var_guard394 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign21840_e23391;
        var_isrh_dn5 = assign21840_e23391_d_n5;
        var_isrh_dn6 = assign21840_e23391_d_n6;
        var_isrh_dn7 = assign21840_e23391_d_n7;
        var_isrh_dn8 = assign21840_e23391_d_n8;

        let (assign21850_e23405,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) && (var_guard394 == 0.0)) {
        let assign21850_e23403: f64 = (var_vbigat - var_vjsrh);
        (assign21850_e23403,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign21850_e23405;

        let (assign21860_e23424,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) && (var_guard394 == 0.0)) {
        let assign21860_e23419: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign21860_e23420: f64 = (1.0 - assign21860_e23419);
        let assign21860_e23421: f64 = (assign21860_e23420).sqrt();
        let assign21860_e23422: f64 = (1.0 - assign21860_e23421);
        (assign21860_e23422,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign21860_e23424;

        let assign21870_e23427: f64 = if p.p826 == 0.5 { 1.0 } else { 0.0 };
        var_guard395 = assign21870_e23427;

        let (assign21880_e23441,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) && (var_guard394 == 0.0)) && (var_guard395 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign21880_e23441;

        let (assign21890_e23473,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) && (var_guard394 == 0.0)) && (var_guard395 == 0.0)) {
        let assign21890_e23456: f64 = (var_wsrhstep * var_wsrhstep);
        let assign21890_e23458: f64 = (var_wsrhstep).ln();
        let assign21890_e23459: f64 = (assign21890_e23456 * assign21890_e23458);
        let assign21890_e23462: f64 = (1.0 - var_wsrhstep);
        let assign21890_e23463: f64 = (assign21890_e23459 / assign21890_e23462);
        let assign21890_e23465: f64 = (assign21890_e23463 + var_wsrhstep);
        let assign21890_e23469: f64 = (2.0 * p.p826);
        let assign21890_e23470: f64 = (1.0 - assign21890_e23469);
        let assign21890_e23471: f64 = (assign21890_e23465 * assign21890_e23470);
        (assign21890_e23471,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign21890_e23473;

        let (assign21900_e23487,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) && (var_guard394 == 0.0)) {
        let assign21900_e23485: f64 = (var_wsrhstep + var_dwsrh);
        (assign21900_e23485,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign21900_e23487;

        let assign21910_e23490: f64 = if p.p826 == 0.5 { 1.0 } else { 0.0 };
        var_guard396 = assign21910_e23490;

        let (assign21920_e23507, assign21920_e23507_d_n5, assign21920_e23507_d_n6, assign21920_e23507_d_n7, assign21920_e23507_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) && (var_guard394 == 0.0)) && (var_guard396 != 0.0)) {
        let assign21920_e23504: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv);
        let assign21920_e23505: f64 = (assign21920_e23504).sqrt();
        (assign21920_e23505, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign21920_e23507;
        var_tmp_dn5 = assign21920_e23507_d_n5;
        var_tmp_dn6 = assign21920_e23507_d_n6;
        var_tmp_dn7 = assign21920_e23507_d_n7;
        var_tmp_dn8 = assign21920_e23507_d_n8;

        let (assign21930_e23526, assign21930_e23526_d_n5, assign21930_e23526_d_n6, assign21930_e23526_d_n7, assign21930_e23526_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) && (var_guard394 == 0.0)) && (var_guard396 == 0.0)) {
        let assign21930_e23522: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv);
        let assign21930_e23524: f64 = (assign21930_e23522).powf(p.p826);
        (assign21930_e23524, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign21930_e23526;
        var_tmp_dn5 = assign21930_e23526_d_n5;
        var_tmp_dn6 = assign21930_e23526_d_n6;
        var_tmp_dn7 = assign21930_e23526_d_n7;
        var_tmp_dn8 = assign21930_e23526_d_n8;

        *var_dwsrh_slot = var_dwsrh;
        *var_erfctimesexpmtat_slot = var_erfctimesexpmtat;
        *var_erfctimesexpmtat_dn5_slot = var_erfctimesexpmtat_dn5;
        *var_erfctimesexpmtat_dn6_slot = var_erfctimesexpmtat_dn6;
        *var_erfctimesexpmtat_dn7_slot = var_erfctimesexpmtat_dn7;
        *var_erfctimesexpmtat_dn8_slot = var_erfctimesexpmtat_dn8;
        *var_fbreakdown_slot = var_fbreakdown;
        *var_fbreakdown_dn5_slot = var_fbreakdown_dn5;
        *var_fbreakdown_dn6_slot = var_fbreakdown_dn6;
        *var_fbreakdown_dn7_slot = var_fbreakdown_dn7;
        *var_fbreakdown_dn8_slot = var_fbreakdown_dn8;
        *var_fmaxr_slot = var_fmaxr;
        *var_fmaxr_dn5_slot = var_fmaxr_dn5;
        *var_fmaxr_dn6_slot = var_fmaxr_dn6;
        *var_fmaxr_dn7_slot = var_fmaxr_dn7;
        *var_fmaxr_dn8_slot = var_fmaxr_dn8;
        *var_gammamax_slot = var_gammamax;
        *var_gammamax_dn5_slot = var_gammamax_dn5;
        *var_gammamax_dn6_slot = var_gammamax_dn6;
        *var_gammamax_dn7_slot = var_gammamax_dn7;
        *var_gammamax_dn8_slot = var_gammamax_dn8;
        *var_guard386_slot = var_guard386;
        *var_guard387_slot = var_guard387;
        *var_guard388_slot = var_guard388;
        *var_guard389_slot = var_guard389;
        *var_guard390_slot = var_guard390;
        *var_guard391_slot = var_guard391;
        *var_guard392_slot = var_guard392;
        *var_guard393_slot = var_guard393;
        *var_guard394_slot = var_guard394;
        *var_guard395_slot = var_guard395;
        *var_guard396_slot = var_guard396;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn5_slot = var_ibbt_dn5;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
        *var_id__blk213_slot = var_id__blk213;
        *var_ijungat_slot = var_ijungat;
        *var_ijungat_dn5_slot = var_ijungat_dn5;
        *var_ijungat_dn6_slot = var_ijungat_dn6;
        *var_ijungat_dn7_slot = var_ijungat_dn7;
        *var_ijungat_dn8_slot = var_ijungat_dn8;
        *var_ijunsti_slot = var_ijunsti;
        *var_ijunsti_dn5_slot = var_ijunsti_dn5;
        *var_ijunsti_dn6_slot = var_ijunsti_dn6;
        *var_ijunsti_dn7_slot = var_ijunsti_dn7;
        *var_ijunsti_dn8_slot = var_ijunsti_dn8;
        *var_isrh_slot = var_isrh;
        *var_isrh_dn5_slot = var_isrh_dn5;
        *var_isrh_dn6_slot = var_isrh_dn6;
        *var_isrh_dn7_slot = var_isrh_dn7;
        *var_isrh_dn8_slot = var_isrh_dn8;
        *var_itat_slot = var_itat;
        *var_itat_dn5_slot = var_itat_dn5;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn5_slot = var_tmp_dn5;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_vbi_minus_vjsrh_slot = var_vbi_minus_vjsrh;
        *var_wsrh_slot = var_wsrh;
        *var_wsrhstep_slot = var_wsrhstep;
    }

    pub(super) fn stamp_transient_block_39(
        p: &Parameters,
        var_atatgat: f64,
        var_berfc: f64,
        var_btatpartgat: f64,
        var_cerfc: f64,
        var_ftdgat: f64,
        var_guard176: f64,
        var_guard193: f64,
        var_guard393: f64,
        var_guard394: f64,
        var_one_minus_pgat: f64,
        var_one_over_one_minus_pgat: f64,
        var_perfc: f64,
        var_vbi_minus_vjsrh: f64,
        var_wdepnulrgat: f64,
        var_wsrh: f64,
        var_zinv: f64,
        var_asrh_slot: &mut f64,
        var_asrh_dn5_slot: &mut f64,
        var_asrh_dn6_slot: &mut f64,
        var_asrh_dn7_slot: &mut f64,
        var_asrh_dn8_slot: &mut f64,
        var_btat_slot: &mut f64,
        var_btat_dn5_slot: &mut f64,
        var_btat_dn6_slot: &mut f64,
        var_btat_dn7_slot: &mut f64,
        var_btat_dn8_slot: &mut f64,
        var_erfcpos_slot: &mut f64,
        var_erfcpos_dn5_slot: &mut f64,
        var_erfcpos_dn6_slot: &mut f64,
        var_erfcpos_dn7_slot: &mut f64,
        var_erfcpos_dn8_slot: &mut f64,
        var_erfctimesexpmtat_slot: &mut f64,
        var_erfctimesexpmtat_dn5_slot: &mut f64,
        var_erfctimesexpmtat_dn6_slot: &mut f64,
        var_erfctimesexpmtat_dn7_slot: &mut f64,
        var_erfctimesexpmtat_dn8_slot: &mut f64,
        var_gammamax_slot: &mut f64,
        var_gammamax_dn5_slot: &mut f64,
        var_gammamax_dn6_slot: &mut f64,
        var_gammamax_dn7_slot: &mut f64,
        var_gammamax_dn8_slot: &mut f64,
        var_guard397_slot: &mut f64,
        var_guard398_slot: &mut f64,
        var_guard399_slot: &mut f64,
        var_guard400_slot: &mut f64,
        var_guard401_slot: &mut f64,
        var_guard402_slot: &mut f64,
        var_isrh_slot: &mut f64,
        var_isrh_dn5_slot: &mut f64,
        var_isrh_dn6_slot: &mut f64,
        var_isrh_dn7_slot: &mut f64,
        var_isrh_dn8_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn5_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_ktat_slot: &mut f64,
        var_ktat_dn5_slot: &mut f64,
        var_ktat_dn6_slot: &mut f64,
        var_ktat_dn7_slot: &mut f64,
        var_ktat_dn8_slot: &mut f64,
        var_ltat_slot: &mut f64,
        var_ltat_dn5_slot: &mut f64,
        var_ltat_dn6_slot: &mut f64,
        var_ltat_dn7_slot: &mut f64,
        var_ltat_dn8_slot: &mut f64,
        var_mtat_slot: &mut f64,
        var_mtat_dn5_slot: &mut f64,
        var_mtat_dn6_slot: &mut f64,
        var_mtat_dn7_slot: &mut f64,
        var_mtat_dn8_slot: &mut f64,
        var_sqrtumax_slot: &mut f64,
        var_sqrtumax_dn5_slot: &mut f64,
        var_sqrtumax_dn6_slot: &mut f64,
        var_sqrtumax_dn7_slot: &mut f64,
        var_sqrtumax_dn8_slot: &mut f64,
        var_terfc_slot: &mut f64,
        var_terfc_dn5_slot: &mut f64,
        var_terfc_dn6_slot: &mut f64,
        var_terfc_dn7_slot: &mut f64,
        var_terfc_dn8_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn5_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_twoatatoverthreebtat_slot: &mut f64,
        var_twoatatoverthreebtat_dn5_slot: &mut f64,
        var_twoatatoverthreebtat_dn6_slot: &mut f64,
        var_twoatatoverthreebtat_dn7_slot: &mut f64,
        var_twoatatoverthreebtat_dn8_slot: &mut f64,
        var_umax_slot: &mut f64,
        var_umax_dn5_slot: &mut f64,
        var_umax_dn6_slot: &mut f64,
        var_umax_dn7_slot: &mut f64,
        var_umax_dn8_slot: &mut f64,
        var_umaxbeforelimiting_slot: &mut f64,
        var_umaxbeforelimiting_dn5_slot: &mut f64,
        var_umaxbeforelimiting_dn6_slot: &mut f64,
        var_umaxbeforelimiting_dn7_slot: &mut f64,
        var_umaxbeforelimiting_dn8_slot: &mut f64,
        var_umaxpoweronepointfive_slot: &mut f64,
        var_umaxpoweronepointfive_dn5_slot: &mut f64,
        var_umaxpoweronepointfive_dn6_slot: &mut f64,
        var_umaxpoweronepointfive_dn7_slot: &mut f64,
        var_umaxpoweronepointfive_dn8_slot: &mut f64,
        var_wdep_slot: &mut f64,
        var_wdep_dn5_slot: &mut f64,
        var_wdep_dn6_slot: &mut f64,
        var_wdep_dn7_slot: &mut f64,
        var_wdep_dn8_slot: &mut f64,
        var_wgamma_slot: &mut f64,
        var_wgamma_dn5_slot: &mut f64,
        var_wgamma_dn6_slot: &mut f64,
        var_wgamma_dn7_slot: &mut f64,
        var_wgamma_dn8_slot: &mut f64,
        var_wtat_slot: &mut f64,
        var_wtat_dn5_slot: &mut f64,
        var_wtat_dn6_slot: &mut f64,
        var_wtat_dn7_slot: &mut f64,
        var_wtat_dn8_slot: &mut f64,
        var_xerfc_slot: &mut f64,
        var_xerfc_dn5_slot: &mut f64,
        var_xerfc_dn6_slot: &mut f64,
        var_xerfc_dn7_slot: &mut f64,
        var_xerfc_dn8_slot: &mut f64,
        var_ysq_slot: &mut f64,
        var_ysq_dn5_slot: &mut f64,
        var_ysq_dn6_slot: &mut f64,
        var_ysq_dn7_slot: &mut f64,
        var_ysq_dn8_slot: &mut f64,
    ) {
        let mut var_asrh: f64 = *var_asrh_slot;
        let mut var_asrh_dn5: f64 = *var_asrh_dn5_slot;
        let mut var_asrh_dn6: f64 = *var_asrh_dn6_slot;
        let mut var_asrh_dn7: f64 = *var_asrh_dn7_slot;
        let mut var_asrh_dn8: f64 = *var_asrh_dn8_slot;
        let mut var_btat: f64 = *var_btat_slot;
        let mut var_btat_dn5: f64 = *var_btat_dn5_slot;
        let mut var_btat_dn6: f64 = *var_btat_dn6_slot;
        let mut var_btat_dn7: f64 = *var_btat_dn7_slot;
        let mut var_btat_dn8: f64 = *var_btat_dn8_slot;
        let mut var_erfcpos: f64 = *var_erfcpos_slot;
        let mut var_erfcpos_dn5: f64 = *var_erfcpos_dn5_slot;
        let mut var_erfcpos_dn6: f64 = *var_erfcpos_dn6_slot;
        let mut var_erfcpos_dn7: f64 = *var_erfcpos_dn7_slot;
        let mut var_erfcpos_dn8: f64 = *var_erfcpos_dn8_slot;
        let mut var_erfctimesexpmtat: f64 = *var_erfctimesexpmtat_slot;
        let mut var_erfctimesexpmtat_dn5: f64 = *var_erfctimesexpmtat_dn5_slot;
        let mut var_erfctimesexpmtat_dn6: f64 = *var_erfctimesexpmtat_dn6_slot;
        let mut var_erfctimesexpmtat_dn7: f64 = *var_erfctimesexpmtat_dn7_slot;
        let mut var_erfctimesexpmtat_dn8: f64 = *var_erfctimesexpmtat_dn8_slot;
        let mut var_gammamax: f64 = *var_gammamax_slot;
        let mut var_gammamax_dn5: f64 = *var_gammamax_dn5_slot;
        let mut var_gammamax_dn6: f64 = *var_gammamax_dn6_slot;
        let mut var_gammamax_dn7: f64 = *var_gammamax_dn7_slot;
        let mut var_gammamax_dn8: f64 = *var_gammamax_dn8_slot;
        let mut var_guard397: f64 = *var_guard397_slot;
        let mut var_guard398: f64 = *var_guard398_slot;
        let mut var_guard399: f64 = *var_guard399_slot;
        let mut var_guard400: f64 = *var_guard400_slot;
        let mut var_guard401: f64 = *var_guard401_slot;
        let mut var_guard402: f64 = *var_guard402_slot;
        let mut var_isrh: f64 = *var_isrh_slot;
        let mut var_isrh_dn5: f64 = *var_isrh_dn5_slot;
        let mut var_isrh_dn6: f64 = *var_isrh_dn6_slot;
        let mut var_isrh_dn7: f64 = *var_isrh_dn7_slot;
        let mut var_isrh_dn8: f64 = *var_isrh_dn8_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn5: f64 = *var_itat_dn5_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_ktat: f64 = *var_ktat_slot;
        let mut var_ktat_dn5: f64 = *var_ktat_dn5_slot;
        let mut var_ktat_dn6: f64 = *var_ktat_dn6_slot;
        let mut var_ktat_dn7: f64 = *var_ktat_dn7_slot;
        let mut var_ktat_dn8: f64 = *var_ktat_dn8_slot;
        let mut var_ltat: f64 = *var_ltat_slot;
        let mut var_ltat_dn5: f64 = *var_ltat_dn5_slot;
        let mut var_ltat_dn6: f64 = *var_ltat_dn6_slot;
        let mut var_ltat_dn7: f64 = *var_ltat_dn7_slot;
        let mut var_ltat_dn8: f64 = *var_ltat_dn8_slot;
        let mut var_mtat: f64 = *var_mtat_slot;
        let mut var_mtat_dn5: f64 = *var_mtat_dn5_slot;
        let mut var_mtat_dn6: f64 = *var_mtat_dn6_slot;
        let mut var_mtat_dn7: f64 = *var_mtat_dn7_slot;
        let mut var_mtat_dn8: f64 = *var_mtat_dn8_slot;
        let mut var_sqrtumax: f64 = *var_sqrtumax_slot;
        let mut var_sqrtumax_dn5: f64 = *var_sqrtumax_dn5_slot;
        let mut var_sqrtumax_dn6: f64 = *var_sqrtumax_dn6_slot;
        let mut var_sqrtumax_dn7: f64 = *var_sqrtumax_dn7_slot;
        let mut var_sqrtumax_dn8: f64 = *var_sqrtumax_dn8_slot;
        let mut var_terfc: f64 = *var_terfc_slot;
        let mut var_terfc_dn5: f64 = *var_terfc_dn5_slot;
        let mut var_terfc_dn6: f64 = *var_terfc_dn6_slot;
        let mut var_terfc_dn7: f64 = *var_terfc_dn7_slot;
        let mut var_terfc_dn8: f64 = *var_terfc_dn8_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn5: f64 = *var_tmp_dn5_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_twoatatoverthreebtat: f64 = *var_twoatatoverthreebtat_slot;
        let mut var_twoatatoverthreebtat_dn5: f64 = *var_twoatatoverthreebtat_dn5_slot;
        let mut var_twoatatoverthreebtat_dn6: f64 = *var_twoatatoverthreebtat_dn6_slot;
        let mut var_twoatatoverthreebtat_dn7: f64 = *var_twoatatoverthreebtat_dn7_slot;
        let mut var_twoatatoverthreebtat_dn8: f64 = *var_twoatatoverthreebtat_dn8_slot;
        let mut var_umax: f64 = *var_umax_slot;
        let mut var_umax_dn5: f64 = *var_umax_dn5_slot;
        let mut var_umax_dn6: f64 = *var_umax_dn6_slot;
        let mut var_umax_dn7: f64 = *var_umax_dn7_slot;
        let mut var_umax_dn8: f64 = *var_umax_dn8_slot;
        let mut var_umaxbeforelimiting: f64 = *var_umaxbeforelimiting_slot;
        let mut var_umaxbeforelimiting_dn5: f64 = *var_umaxbeforelimiting_dn5_slot;
        let mut var_umaxbeforelimiting_dn6: f64 = *var_umaxbeforelimiting_dn6_slot;
        let mut var_umaxbeforelimiting_dn7: f64 = *var_umaxbeforelimiting_dn7_slot;
        let mut var_umaxbeforelimiting_dn8: f64 = *var_umaxbeforelimiting_dn8_slot;
        let mut var_umaxpoweronepointfive: f64 = *var_umaxpoweronepointfive_slot;
        let mut var_umaxpoweronepointfive_dn5: f64 = *var_umaxpoweronepointfive_dn5_slot;
        let mut var_umaxpoweronepointfive_dn6: f64 = *var_umaxpoweronepointfive_dn6_slot;
        let mut var_umaxpoweronepointfive_dn7: f64 = *var_umaxpoweronepointfive_dn7_slot;
        let mut var_umaxpoweronepointfive_dn8: f64 = *var_umaxpoweronepointfive_dn8_slot;
        let mut var_wdep: f64 = *var_wdep_slot;
        let mut var_wdep_dn5: f64 = *var_wdep_dn5_slot;
        let mut var_wdep_dn6: f64 = *var_wdep_dn6_slot;
        let mut var_wdep_dn7: f64 = *var_wdep_dn7_slot;
        let mut var_wdep_dn8: f64 = *var_wdep_dn8_slot;
        let mut var_wgamma: f64 = *var_wgamma_slot;
        let mut var_wgamma_dn5: f64 = *var_wgamma_dn5_slot;
        let mut var_wgamma_dn6: f64 = *var_wgamma_dn6_slot;
        let mut var_wgamma_dn7: f64 = *var_wgamma_dn7_slot;
        let mut var_wgamma_dn8: f64 = *var_wgamma_dn8_slot;
        let mut var_wtat: f64 = *var_wtat_slot;
        let mut var_wtat_dn5: f64 = *var_wtat_dn5_slot;
        let mut var_wtat_dn6: f64 = *var_wtat_dn6_slot;
        let mut var_wtat_dn7: f64 = *var_wtat_dn7_slot;
        let mut var_wtat_dn8: f64 = *var_wtat_dn8_slot;
        let mut var_xerfc: f64 = *var_xerfc_slot;
        let mut var_xerfc_dn5: f64 = *var_xerfc_dn5_slot;
        let mut var_xerfc_dn6: f64 = *var_xerfc_dn6_slot;
        let mut var_xerfc_dn7: f64 = *var_xerfc_dn7_slot;
        let mut var_xerfc_dn8: f64 = *var_xerfc_dn8_slot;
        let mut var_ysq: f64 = *var_ysq_slot;
        let mut var_ysq_dn5: f64 = *var_ysq_dn5_slot;
        let mut var_ysq_dn6: f64 = *var_ysq_dn6_slot;
        let mut var_ysq_dn7: f64 = *var_ysq_dn7_slot;
        let mut var_ysq_dn8: f64 = *var_ysq_dn8_slot;

        let (assign21940_e23540, assign21940_e23540_d_n5, assign21940_e23540_d_n6, assign21940_e23540_d_n7, assign21940_e23540_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) && (var_guard394 == 0.0)) {
        let assign21940_e23538: f64 = (var_wdepnulrgat * var_tmp);
        (assign21940_e23538, (var_wdepnulrgat * var_tmp_dn5), (var_wdepnulrgat * var_tmp_dn6), (var_wdepnulrgat * var_tmp_dn7), (var_wdepnulrgat * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign21940_e23540;
        var_wdep_dn5 = assign21940_e23540_d_n5;
        var_wdep_dn6 = assign21940_e23540_d_n6;
        var_wdep_dn7 = assign21940_e23540_d_n7;
        var_wdep_dn8 = assign21940_e23540_d_n8;

        let (assign21950_e23558, assign21950_e23558_d_n5, assign21950_e23558_d_n6, assign21950_e23558_d_n7, assign21950_e23558_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) && (var_guard394 == 0.0)) {
        let assign21950_e23553: f64 = (var_zinv - 1.0);
        let assign21950_e23555: f64 = (assign21950_e23553 * var_wdep);
        let assign21950_e23556: f64 = (var_ftdgat * assign21950_e23555);
        (assign21950_e23556, (var_ftdgat * (assign21950_e23553 * var_wdep_dn5)), (var_ftdgat * (assign21950_e23553 * var_wdep_dn6)), (var_ftdgat * (assign21950_e23553 * var_wdep_dn7)), (var_ftdgat * (assign21950_e23553 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign21950_e23558;
        var_asrh_dn5 = assign21950_e23558_d_n5;
        var_asrh_dn6 = assign21950_e23558_d_n6;
        var_asrh_dn7 = assign21950_e23558_d_n7;
        var_asrh_dn8 = assign21950_e23558_d_n8;

        let (assign21960_e23574, assign21960_e23574_d_n5, assign21960_e23574_d_n6, assign21960_e23574_d_n7, assign21960_e23574_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) && (var_guard394 == 0.0)) {
        let assign21960_e23571: f64 = (var_asrh * var_wsrh);
        let assign21960_e23572: f64 = (p.p835 * assign21960_e23571);
        (assign21960_e23572, (p.p835 * (var_asrh_dn5 * var_wsrh)), (p.p835 * (var_asrh_dn6 * var_wsrh)), (p.p835 * (var_asrh_dn7 * var_wsrh)), (p.p835 * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign21960_e23574;
        var_isrh_dn5 = assign21960_e23574_d_n5;
        var_isrh_dn6 = assign21960_e23574_d_n6;
        var_isrh_dn7 = assign21960_e23574_d_n7;
        var_isrh_dn8 = assign21960_e23574_d_n8;

        let assign21970_e23577: f64 = if p.p840 == 0.0 { 1.0 } else { 0.0 };
        var_guard397 = assign21970_e23577;

        let (assign21980_e23588, assign21980_e23588_d_n5, assign21980_e23588_d_n6, assign21980_e23588_d_n7, assign21980_e23588_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) && (var_guard397 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign21980_e23588;
        var_itat_dn5 = assign21980_e23588_d_n5;
        var_itat_dn6 = assign21980_e23588_d_n6;
        var_itat_dn7 = assign21980_e23588_d_n7;
        var_itat_dn8 = assign21980_e23588_d_n8;

        let (assign21990_e23606, assign21990_e23606_d_n5, assign21990_e23606_d_n6, assign21990_e23606_d_n7, assign21990_e23606_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) && (var_guard397 == 0.0)) {
        let assign21990_e23601: f64 = (var_wdep * var_one_minus_pgat);
        let assign21990_e23603: f64 = (assign21990_e23601 / var_vbi_minus_vjsrh);
        let assign21990_e23604: f64 = (var_btatpartgat * assign21990_e23603);
        (assign21990_e23604, (var_btatpartgat * ((var_wdep_dn5 * var_one_minus_pgat) / var_vbi_minus_vjsrh)), (var_btatpartgat * ((var_wdep_dn6 * var_one_minus_pgat) / var_vbi_minus_vjsrh)), (var_btatpartgat * ((var_wdep_dn7 * var_one_minus_pgat) / var_vbi_minus_vjsrh)), (var_btatpartgat * ((var_wdep_dn8 * var_one_minus_pgat) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign21990_e23606;
        var_btat_dn5 = assign21990_e23606_d_n5;
        var_btat_dn6 = assign21990_e23606_d_n6;
        var_btat_dn7 = assign21990_e23606_d_n7;
        var_btat_dn8 = assign21990_e23606_d_n8;

        let (assign22000_e23622, assign22000_e23622_d_n5, assign22000_e23622_d_n6, assign22000_e23622_d_n7, assign22000_e23622_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) && (var_guard397 == 0.0)) {
        let assign22000_e23618: f64 = (0.666666666666667 * var_atatgat);
        let assign22000_e23620: f64 = (assign22000_e23618 / var_btat);
        (assign22000_e23620, (-((assign22000_e23618 * var_btat_dn5) / (var_btat * var_btat))), (-((assign22000_e23618 * var_btat_dn6) / (var_btat * var_btat))), (-((assign22000_e23618 * var_btat_dn7) / (var_btat * var_btat))), (-((assign22000_e23618 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign22000_e23622;
        var_twoatatoverthreebtat_dn5 = assign22000_e23622_d_n5;
        var_twoatatoverthreebtat_dn6 = assign22000_e23622_d_n6;
        var_twoatatoverthreebtat_dn7 = assign22000_e23622_d_n7;
        var_twoatatoverthreebtat_dn8 = assign22000_e23622_d_n8;

        let (assign22010_e23636, assign22010_e23636_d_n5, assign22010_e23636_d_n6, assign22010_e23636_d_n7, assign22010_e23636_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) && (var_guard397 == 0.0)) {
        let assign22010_e23634: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign22010_e23634, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign22010_e23636;
        var_umaxbeforelimiting_dn5 = assign22010_e23636_d_n5;
        var_umaxbeforelimiting_dn6 = assign22010_e23636_d_n6;
        var_umaxbeforelimiting_dn7 = assign22010_e23636_d_n7;
        var_umaxbeforelimiting_dn8 = assign22010_e23636_d_n8;

        let (assign22020_e23657, assign22020_e23657_d_n5, assign22020_e23657_d_n6, assign22020_e23657_d_n7, assign22020_e23657_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) && (var_guard397 == 0.0)) {
        let assign22020_e23648: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign22020_e23651: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign22020_e23653: f64 = (assign22020_e23651 + 1.0);
        let assign22020_e23654: f64 = (assign22020_e23648 / assign22020_e23653);
        let assign22020_e23655: f64 = (assign22020_e23654).sqrt();
        (assign22020_e23655, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign22020_e23653) - (assign22020_e23648 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign22020_e23653 * assign22020_e23653)) / (2.0 * assign22020_e23655)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign22020_e23653) - (assign22020_e23648 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign22020_e23653 * assign22020_e23653)) / (2.0 * assign22020_e23655)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign22020_e23653) - (assign22020_e23648 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign22020_e23653 * assign22020_e23653)) / (2.0 * assign22020_e23655)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign22020_e23653) - (assign22020_e23648 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign22020_e23653 * assign22020_e23653)) / (2.0 * assign22020_e23655)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign22020_e23657;
        var_umax_dn5 = assign22020_e23657_d_n5;
        var_umax_dn6 = assign22020_e23657_d_n6;
        var_umax_dn7 = assign22020_e23657_d_n7;
        var_umax_dn8 = assign22020_e23657_d_n8;

        let (assign22030_e23670, assign22030_e23670_d_n5, assign22030_e23670_d_n6, assign22030_e23670_d_n7, assign22030_e23670_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) && (var_guard397 == 0.0)) {
        let assign22030_e23668: f64 = (var_umax).sqrt();
        (assign22030_e23668, (var_umax_dn5 / (2.0 * assign22030_e23668)), (var_umax_dn6 / (2.0 * assign22030_e23668)), (var_umax_dn7 / (2.0 * assign22030_e23668)), (var_umax_dn8 / (2.0 * assign22030_e23668)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign22030_e23670;
        var_sqrtumax_dn5 = assign22030_e23670_d_n5;
        var_sqrtumax_dn6 = assign22030_e23670_d_n6;
        var_sqrtumax_dn7 = assign22030_e23670_d_n7;
        var_sqrtumax_dn8 = assign22030_e23670_d_n8;

        let (assign22040_e23684, assign22040_e23684_d_n5, assign22040_e23684_d_n6, assign22040_e23684_d_n7, assign22040_e23684_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) && (var_guard397 == 0.0)) {
        let assign22040_e23682: f64 = (var_umax * var_sqrtumax);
        (assign22040_e23682, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign22040_e23684;
        var_umaxpoweronepointfive_dn5 = assign22040_e23684_d_n5;
        var_umaxpoweronepointfive_dn6 = assign22040_e23684_d_n6;
        var_umaxpoweronepointfive_dn7 = assign22040_e23684_d_n7;
        var_umaxpoweronepointfive_dn8 = assign22040_e23684_d_n8;

        let assign22050_e23686: f64 = (-p.p826);
        let assign22050_e23688: f64 = (assign22050_e23686 * var_one_over_one_minus_pgat);
        let assign22050_e23690: f64 = (-1.0);
        let assign22050_e23691: f64 = if assign22050_e23688 == assign22050_e23690 { 1.0 } else { 0.0 };
        var_guard398 = assign22050_e23691;

        let (assign22060_e23711, assign22060_e23711_d_n5, assign22060_e23711_d_n6, assign22060_e23711_d_n7, assign22060_e23711_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) && (var_guard397 == 0.0)) && (var_guard398 != 0.0)) {
        let assign22060_e23707: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign22060_e23708: f64 = (1.0 + assign22060_e23707);
        let assign22060_e23709: f64 = (1.0 / assign22060_e23708);
        (assign22060_e23709, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign22060_e23708 * assign22060_e23708))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign22060_e23708 * assign22060_e23708))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign22060_e23708 * assign22060_e23708))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign22060_e23708 * assign22060_e23708))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign22060_e23711;
        var_wgamma_dn5 = assign22060_e23711_d_n5;
        var_wgamma_dn6 = assign22060_e23711_d_n6;
        var_wgamma_dn7 = assign22060_e23711_d_n7;
        var_wgamma_dn8 = assign22060_e23711_d_n8;

        let (assign22070_e23735, assign22070_e23735_d_n5, assign22070_e23735_d_n6, assign22070_e23735_d_n7, assign22070_e23735_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) && (var_guard397 == 0.0)) && (var_guard398 == 0.0)) {
        let assign22070_e23727: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign22070_e23728: f64 = (1.0 + assign22070_e23727);
        let assign22070_e23730: f64 = (-p.p826);
        let assign22070_e23732: f64 = (assign22070_e23730 * var_one_over_one_minus_pgat);
        let assign22070_e23733: f64 = (assign22070_e23728).powf(assign22070_e23732);
        (assign22070_e23733, if 0.0 == 0.0 && ((assign22070_e23732) as f64).is_finite() && ((assign22070_e23732) as f64).fract() == 0.0 { if assign22070_e23732 == 0.0 { 0.0 } else { (assign22070_e23732 * ((assign22070_e23728).powf(assign22070_e23732 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign22070_e23733 * (assign22070_e23732 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign22070_e23728))) }, if 0.0 == 0.0 && ((assign22070_e23732) as f64).is_finite() && ((assign22070_e23732) as f64).fract() == 0.0 { if assign22070_e23732 == 0.0 { 0.0 } else { (assign22070_e23732 * ((assign22070_e23728).powf(assign22070_e23732 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign22070_e23733 * (assign22070_e23732 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign22070_e23728))) }, if 0.0 == 0.0 && ((assign22070_e23732) as f64).is_finite() && ((assign22070_e23732) as f64).fract() == 0.0 { if assign22070_e23732 == 0.0 { 0.0 } else { (assign22070_e23732 * ((assign22070_e23728).powf(assign22070_e23732 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign22070_e23733 * (assign22070_e23732 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign22070_e23728))) }, if 0.0 == 0.0 && ((assign22070_e23732) as f64).is_finite() && ((assign22070_e23732) as f64).fract() == 0.0 { if assign22070_e23732 == 0.0 { 0.0 } else { (assign22070_e23732 * ((assign22070_e23728).powf(assign22070_e23732 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign22070_e23733 * (assign22070_e23732 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign22070_e23728))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign22070_e23735;
        var_wgamma_dn5 = assign22070_e23735_d_n5;
        var_wgamma_dn6 = assign22070_e23735_d_n6;
        var_wgamma_dn7 = assign22070_e23735_d_n7;
        var_wgamma_dn8 = assign22070_e23735_d_n8;

        let (assign22080_e23753, assign22080_e23753_d_n5, assign22080_e23753_d_n6, assign22080_e23753_d_n7, assign22080_e23753_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) && (var_guard397 == 0.0)) {
        let assign22080_e23747: f64 = (var_wsrh * var_wgamma);
        let assign22080_e23750: f64 = (var_wsrh + var_wgamma);
        let assign22080_e23751: f64 = (assign22080_e23747 / assign22080_e23750);
        (assign22080_e23751, ((((var_wsrh * var_wgamma_dn5) * assign22080_e23750) - (assign22080_e23747 * var_wgamma_dn5)) / (assign22080_e23750 * assign22080_e23750)), ((((var_wsrh * var_wgamma_dn6) * assign22080_e23750) - (assign22080_e23747 * var_wgamma_dn6)) / (assign22080_e23750 * assign22080_e23750)), ((((var_wsrh * var_wgamma_dn7) * assign22080_e23750) - (assign22080_e23747 * var_wgamma_dn7)) / (assign22080_e23750 * assign22080_e23750)), ((((var_wsrh * var_wgamma_dn8) * assign22080_e23750) - (assign22080_e23747 * var_wgamma_dn8)) / (assign22080_e23750 * assign22080_e23750)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign22080_e23753;
        var_wtat_dn5 = assign22080_e23753_d_n5;
        var_wtat_dn6 = assign22080_e23753_d_n6;
        var_wtat_dn7 = assign22080_e23753_d_n7;
        var_wtat_dn8 = assign22080_e23753_d_n8;

        let (assign22090_e23770, assign22090_e23770_d_n5, assign22090_e23770_d_n6, assign22090_e23770_d_n7, assign22090_e23770_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) && (var_guard397 == 0.0)) {
        let assign22090_e23766: f64 = (var_btat / var_sqrtumax);
        let assign22090_e23767: f64 = (0.375 * assign22090_e23766);
        let assign22090_e23768: f64 = (assign22090_e23767).sqrt();
        (assign22090_e23768, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign22090_e23768)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign22090_e23768)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign22090_e23768)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign22090_e23768)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign22090_e23770;
        var_ktat_dn5 = assign22090_e23770_d_n5;
        var_ktat_dn6 = assign22090_e23770_d_n6;
        var_ktat_dn7 = assign22090_e23770_d_n7;
        var_ktat_dn8 = assign22090_e23770_d_n8;

        let (assign22100_e23788, assign22100_e23788_d_n5, assign22100_e23788_d_n6, assign22100_e23788_d_n7, assign22100_e23788_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) && (var_guard397 == 0.0)) {
        let assign22100_e23783: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign22100_e23784: f64 = (2.0 * assign22100_e23783);
        let assign22100_e23786: f64 = (assign22100_e23784 - var_umax);
        (assign22100_e23786, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign22100_e23788;
        var_ltat_dn5 = assign22100_e23788_d_n5;
        var_ltat_dn6 = assign22100_e23788_d_n6;
        var_ltat_dn7 = assign22100_e23788_d_n7;
        var_ltat_dn8 = assign22100_e23788_d_n8;

        let (assign22110_e23814, assign22110_e23814_d_n5, assign22110_e23814_d_n6, assign22110_e23814_d_n7, assign22110_e23814_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) && (var_guard397 == 0.0)) {
        let assign22110_e23800: f64 = (var_atatgat * var_twoatatoverthreebtat);
        let assign22110_e23802: f64 = (assign22110_e23800 * var_sqrtumax);
        let assign22110_e23805: f64 = (var_atatgat * var_umax);
        let assign22110_e23806: f64 = (assign22110_e23802 - assign22110_e23805);
        let assign22110_e23810: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign22110_e23811: f64 = (0.5 * assign22110_e23810);
        let assign22110_e23812: f64 = (assign22110_e23806 + assign22110_e23811);
        (assign22110_e23812, (((((var_atatgat * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign22110_e23800 * var_sqrtumax_dn5)) - (var_atatgat * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatgat * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign22110_e23800 * var_sqrtumax_dn6)) - (var_atatgat * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatgat * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign22110_e23800 * var_sqrtumax_dn7)) - (var_atatgat * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatgat * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign22110_e23800 * var_sqrtumax_dn8)) - (var_atatgat * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign22110_e23814;
        var_mtat_dn5 = assign22110_e23814_d_n5;
        var_mtat_dn6 = assign22110_e23814_d_n6;
        var_mtat_dn7 = assign22110_e23814_d_n7;
        var_mtat_dn8 = assign22110_e23814_d_n8;

        let (assign22120_e23830, assign22120_e23830_d_n5, assign22120_e23830_d_n6, assign22120_e23830_d_n7, assign22120_e23830_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) && (var_guard397 == 0.0)) {
        let assign22120_e23826: f64 = (var_ltat - 1.0);
        let assign22120_e23828: f64 = (assign22120_e23826 * var_ktat);
        (assign22120_e23828, ((var_ltat_dn5 * var_ktat) + (assign22120_e23826 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign22120_e23826 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign22120_e23826 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign22120_e23826 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign22120_e23830;
        var_xerfc_dn5 = assign22120_e23830_d_n5;
        var_xerfc_dn6 = assign22120_e23830_d_n6;
        var_xerfc_dn7 = assign22120_e23830_d_n7;
        var_xerfc_dn8 = assign22120_e23830_d_n8;

        let (assign22130_e23844, assign22130_e23844_d_n5, assign22130_e23844_d_n6, assign22130_e23844_d_n7, assign22130_e23844_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) && (var_guard397 == 0.0)) {
        let assign22130_e23842: f64 = (var_xerfc * var_xerfc);
        (assign22130_e23842, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign22130_e23844;
        var_ysq_dn5 = assign22130_e23844_d_n5;
        var_ysq_dn6 = assign22130_e23844_d_n6;
        var_ysq_dn7 = assign22130_e23844_d_n7;
        var_ysq_dn8 = assign22130_e23844_d_n8;

        let assign22140_e23847: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard399 = assign22140_e23847;

        let (assign22150_e23867, assign22150_e23867_d_n5, assign22150_e23867_d_n6, assign22150_e23867_d_n7, assign22150_e23867_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) && (var_guard397 == 0.0)) && (var_guard399 != 0.0)) {
        let assign22150_e23863: f64 = (var_perfc * var_xerfc);
        let assign22150_e23864: f64 = (1.0 + assign22150_e23863);
        let assign22150_e23865: f64 = (1.0 / assign22150_e23864);
        (assign22150_e23865, (-((var_perfc * var_xerfc_dn5) / (assign22150_e23864 * assign22150_e23864))), (-((var_perfc * var_xerfc_dn6) / (assign22150_e23864 * assign22150_e23864))), (-((var_perfc * var_xerfc_dn7) / (assign22150_e23864 * assign22150_e23864))), (-((var_perfc * var_xerfc_dn8) / (assign22150_e23864 * assign22150_e23864))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign22150_e23867;
        var_terfc_dn5 = assign22150_e23867_d_n5;
        var_terfc_dn6 = assign22150_e23867_d_n6;
        var_terfc_dn7 = assign22150_e23867_d_n7;
        var_terfc_dn8 = assign22150_e23867_d_n8;

        let (assign22160_e23888, assign22160_e23888_d_n5, assign22160_e23888_d_n6, assign22160_e23888_d_n7, assign22160_e23888_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) && (var_guard397 == 0.0)) && (var_guard399 == 0.0)) {
        let assign22160_e23884: f64 = (var_perfc * var_xerfc);
        let assign22160_e23885: f64 = (1.0 - assign22160_e23884);
        let assign22160_e23886: f64 = (1.0 / assign22160_e23885);
        (assign22160_e23886, (-((-(var_perfc * var_xerfc_dn5)) / (assign22160_e23885 * assign22160_e23885))), (-((-(var_perfc * var_xerfc_dn6)) / (assign22160_e23885 * assign22160_e23885))), (-((-(var_perfc * var_xerfc_dn7)) / (assign22160_e23885 * assign22160_e23885))), (-((-(var_perfc * var_xerfc_dn8)) / (assign22160_e23885 * assign22160_e23885))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign22160_e23888;
        var_terfc_dn5 = assign22160_e23888_d_n5;
        var_terfc_dn6 = assign22160_e23888_d_n6;
        var_terfc_dn7 = assign22160_e23888_d_n7;
        var_terfc_dn8 = assign22160_e23888_d_n8;

        let assign22170_e23890: f64 = (-var_ysq);
        let assign22170_e23892: f64 = (assign22170_e23890 + var_mtat);
        let assign22170_e23894: f64 = (-230.25850929940458);
        let assign22170_e23895: f64 = if assign22170_e23892 > assign22170_e23894 { 1.0 } else { 0.0 };
        var_guard400 = assign22170_e23895;

        let (assign22180_e23913, assign22180_e23913_d_n5, assign22180_e23913_d_n6, assign22180_e23913_d_n7, assign22180_e23913_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) && (var_guard397 == 0.0)) && (var_guard400 != 0.0)) {
        let assign22180_e23908: f64 = (-var_ysq);
        let assign22180_e23910: f64 = (assign22180_e23908 + var_mtat);
        let assign22180_e23911: f64 = (assign22180_e23910).exp();
        (assign22180_e23911, (assign22180_e23911 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign22180_e23911 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign22180_e23911 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign22180_e23911 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign22180_e23913;
        var_tmp_dn5 = assign22180_e23913_d_n5;
        var_tmp_dn6 = assign22180_e23913_d_n6;
        var_tmp_dn7 = assign22180_e23913_d_n7;
        var_tmp_dn8 = assign22180_e23913_d_n8;

        let (assign22190_e23962, assign22190_e23962_d_n5, assign22190_e23962_d_n6, assign22190_e23962_d_n7, assign22190_e23962_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) && (var_guard397 == 0.0)) && (var_guard400 == 0.0)) {
        let assign22190_e23929: f64 = (-230.25850929940458);
        let assign22190_e23931: f64 = (-var_ysq);
        let assign22190_e23933: f64 = (assign22190_e23931 + var_mtat);
        let assign22190_e23934: f64 = (assign22190_e23929 - assign22190_e23933);
        let assign22190_e23938: f64 = (-230.25850929940458);
        let assign22190_e23940: f64 = (-var_ysq);
        let assign22190_e23942: f64 = (assign22190_e23940 + var_mtat);
        let assign22190_e23943: f64 = (assign22190_e23938 - assign22190_e23942);
        let assign22190_e23946: f64 = (-230.25850929940458);
        let assign22190_e23948: f64 = (-var_ysq);
        let assign22190_e23950: f64 = (assign22190_e23948 + var_mtat);
        let assign22190_e23951: f64 = (assign22190_e23946 - assign22190_e23950);
        let assign22190_e23953: f64 = (assign22190_e23951 * 0.3333333333333333);
        let assign22190_e23954: f64 = (1.0 + assign22190_e23953);
        let assign22190_e23955: f64 = (assign22190_e23943 * assign22190_e23954);
        let assign22190_e23956: f64 = (0.5 * assign22190_e23955);
        let assign22190_e23957: f64 = (1.0 + assign22190_e23956);
        let assign22190_e23958: f64 = (assign22190_e23934 * assign22190_e23957);
        let assign22190_e23959: f64 = (1.0 + assign22190_e23958);
        let assign22190_e23960: f64 = (1e-100 / assign22190_e23959);
        (assign22190_e23960, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign22190_e23957) + (assign22190_e23934 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign22190_e23954) + (assign22190_e23943 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign22190_e23959 * assign22190_e23959))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign22190_e23957) + (assign22190_e23934 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign22190_e23954) + (assign22190_e23943 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign22190_e23959 * assign22190_e23959))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign22190_e23957) + (assign22190_e23934 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign22190_e23954) + (assign22190_e23943 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign22190_e23959 * assign22190_e23959))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign22190_e23957) + (assign22190_e23934 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign22190_e23954) + (assign22190_e23943 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign22190_e23959 * assign22190_e23959))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign22190_e23962;
        var_tmp_dn5 = assign22190_e23962_d_n5;
        var_tmp_dn6 = assign22190_e23962_d_n6;
        var_tmp_dn7 = assign22190_e23962_d_n7;
        var_tmp_dn8 = assign22190_e23962_d_n8;

        let (assign22200_e23992, assign22200_e23992_d_n5, assign22200_e23992_d_n6, assign22200_e23992_d_n7, assign22200_e23992_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) && (var_guard397 == 0.0)) {
        let assign22200_e23974: f64 = (0.29214664 * var_terfc);
        let assign22200_e23978: f64 = (var_terfc * var_terfc);
        let assign22200_e23979: f64 = (var_berfc * assign22200_e23978);
        let assign22200_e23980: f64 = (assign22200_e23974 + assign22200_e23979);
        let assign22200_e23984: f64 = (var_terfc * var_terfc);
        let assign22200_e23986: f64 = (assign22200_e23984 * var_terfc);
        let assign22200_e23987: f64 = (var_cerfc * assign22200_e23986);
        let assign22200_e23988: f64 = (assign22200_e23980 + assign22200_e23987);
        let assign22200_e23990: f64 = (assign22200_e23988 * var_tmp);
        (assign22200_e23990, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign22200_e23984 * var_terfc_dn5)))) * var_tmp) + (assign22200_e23988 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign22200_e23984 * var_terfc_dn6)))) * var_tmp) + (assign22200_e23988 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign22200_e23984 * var_terfc_dn7)))) * var_tmp) + (assign22200_e23988 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign22200_e23984 * var_terfc_dn8)))) * var_tmp) + (assign22200_e23988 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign22200_e23992;
        var_erfcpos_dn5 = assign22200_e23992_d_n5;
        var_erfcpos_dn6 = assign22200_e23992_d_n6;
        var_erfcpos_dn7 = assign22200_e23992_d_n7;
        var_erfcpos_dn8 = assign22200_e23992_d_n8;

        let assign22210_e23995: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard401 = assign22210_e23995;

        let (assign22220_e24009, assign22220_e24009_d_n5, assign22220_e24009_d_n6, assign22220_e24009_d_n7, assign22220_e24009_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) && (var_guard397 == 0.0)) && (var_guard401 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign22220_e24009;
        var_erfctimesexpmtat_dn5 = assign22220_e24009_d_n5;
        var_erfctimesexpmtat_dn6 = assign22220_e24009_d_n6;
        var_erfctimesexpmtat_dn7 = assign22220_e24009_d_n7;
        var_erfctimesexpmtat_dn8 = assign22220_e24009_d_n8;

        let assign22230_e24012: f64 = (-230.25850929940458);
        let assign22230_e24013: f64 = if var_mtat > assign22230_e24012 { 1.0 } else { 0.0 };
        var_guard402 = assign22230_e24013;

        let (assign22240_e24031, assign22240_e24031_d_n5, assign22240_e24031_d_n6, assign22240_e24031_d_n7, assign22240_e24031_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) && (var_guard397 == 0.0)) && (var_guard401 == 0.0)) && (var_guard402 != 0.0)) {
        let assign22240_e24029: f64 = (var_mtat).exp();
        (assign22240_e24029, (assign22240_e24029 * var_mtat_dn5), (assign22240_e24029 * var_mtat_dn6), (assign22240_e24029 * var_mtat_dn7), (assign22240_e24029 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign22240_e24031;
        var_tmp_dn5 = assign22240_e24031_d_n5;
        var_tmp_dn6 = assign22240_e24031_d_n6;
        var_tmp_dn7 = assign22240_e24031_d_n7;
        var_tmp_dn8 = assign22240_e24031_d_n8;

        let (assign22250_e24074, assign22250_e24074_d_n5, assign22250_e24074_d_n6, assign22250_e24074_d_n7, assign22250_e24074_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) && (var_guard397 == 0.0)) && (var_guard401 == 0.0)) && (var_guard402 == 0.0)) {
        let assign22250_e24050: f64 = (-230.25850929940458);
        let assign22250_e24052: f64 = (assign22250_e24050 - var_mtat);
        let assign22250_e24056: f64 = (-230.25850929940458);
        let assign22250_e24058: f64 = (assign22250_e24056 - var_mtat);
        let assign22250_e24061: f64 = (-230.25850929940458);
        let assign22250_e24063: f64 = (assign22250_e24061 - var_mtat);
        let assign22250_e24065: f64 = (assign22250_e24063 * 0.3333333333333333);
        let assign22250_e24066: f64 = (1.0 + assign22250_e24065);
        let assign22250_e24067: f64 = (assign22250_e24058 * assign22250_e24066);
        let assign22250_e24068: f64 = (0.5 * assign22250_e24067);
        let assign22250_e24069: f64 = (1.0 + assign22250_e24068);
        let assign22250_e24070: f64 = (assign22250_e24052 * assign22250_e24069);
        let assign22250_e24071: f64 = (1.0 + assign22250_e24070);
        let assign22250_e24072: f64 = (1e-100 / assign22250_e24071);
        (assign22250_e24072, (-((1e-100 * (((-var_mtat_dn5) * assign22250_e24069) + (assign22250_e24052 * (0.5 * (((-var_mtat_dn5) * assign22250_e24066) + (assign22250_e24058 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign22250_e24071 * assign22250_e24071))), (-((1e-100 * (((-var_mtat_dn6) * assign22250_e24069) + (assign22250_e24052 * (0.5 * (((-var_mtat_dn6) * assign22250_e24066) + (assign22250_e24058 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign22250_e24071 * assign22250_e24071))), (-((1e-100 * (((-var_mtat_dn7) * assign22250_e24069) + (assign22250_e24052 * (0.5 * (((-var_mtat_dn7) * assign22250_e24066) + (assign22250_e24058 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign22250_e24071 * assign22250_e24071))), (-((1e-100 * (((-var_mtat_dn8) * assign22250_e24069) + (assign22250_e24052 * (0.5 * (((-var_mtat_dn8) * assign22250_e24066) + (assign22250_e24058 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign22250_e24071 * assign22250_e24071))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign22250_e24074;
        var_tmp_dn5 = assign22250_e24074_d_n5;
        var_tmp_dn6 = assign22250_e24074_d_n6;
        var_tmp_dn7 = assign22250_e24074_d_n7;
        var_tmp_dn8 = assign22250_e24074_d_n8;

        let (assign22260_e24093, assign22260_e24093_d_n5, assign22260_e24093_d_n6, assign22260_e24093_d_n7, assign22260_e24093_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) && (var_guard397 == 0.0)) && (var_guard401 == 0.0)) {
        let assign22260_e24089: f64 = (2.0 * var_tmp);
        let assign22260_e24091: f64 = (assign22260_e24089 - var_erfcpos);
        (assign22260_e24091, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign22260_e24093;
        var_erfctimesexpmtat_dn5 = assign22260_e24093_d_n5;
        var_erfctimesexpmtat_dn6 = assign22260_e24093_d_n6;
        var_erfctimesexpmtat_dn7 = assign22260_e24093_d_n7;
        var_erfctimesexpmtat_dn8 = assign22260_e24093_d_n8;

        let (assign22270_e24113, assign22270_e24113_d_n5, assign22270_e24113_d_n6, assign22270_e24113_d_n7, assign22270_e24113_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) && (var_guard397 == 0.0)) {
        let assign22270_e24105: f64 = (1.772453850905516 * 0.5);
        let assign22270_e24108: f64 = (var_atatgat * var_erfctimesexpmtat);
        let assign22270_e24110: f64 = (assign22270_e24108 / var_ktat);
        let assign22270_e24111: f64 = (assign22270_e24105 * assign22270_e24110);
        (assign22270_e24111, (assign22270_e24105 * ((((var_atatgat * var_erfctimesexpmtat_dn5) * var_ktat) - (assign22270_e24108 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign22270_e24105 * ((((var_atatgat * var_erfctimesexpmtat_dn6) * var_ktat) - (assign22270_e24108 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign22270_e24105 * ((((var_atatgat * var_erfctimesexpmtat_dn7) * var_ktat) - (assign22270_e24108 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign22270_e24105 * ((((var_atatgat * var_erfctimesexpmtat_dn8) * var_ktat) - (assign22270_e24108 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign22270_e24113;
        var_gammamax_dn5 = assign22270_e24113_d_n5;
        var_gammamax_dn6 = assign22270_e24113_d_n6;
        var_gammamax_dn7 = assign22270_e24113_d_n7;
        var_gammamax_dn8 = assign22270_e24113_d_n8;

        let (assign22280_e24131, assign22280_e24131_d_n5, assign22280_e24131_d_n6, assign22280_e24131_d_n7, assign22280_e24131_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) && (var_guard397 == 0.0)) {
        let assign22280_e24126: f64 = (var_asrh * var_gammamax);
        let assign22280_e24128: f64 = (assign22280_e24126 * var_wtat);
        let assign22280_e24129: f64 = (p.p840 * assign22280_e24128);
        (assign22280_e24129, (p.p840 * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign22280_e24126 * var_wtat_dn5))), (p.p840 * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign22280_e24126 * var_wtat_dn6))), (p.p840 * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign22280_e24126 * var_wtat_dn7))), (p.p840 * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign22280_e24126 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign22280_e24131;
        var_itat_dn5 = assign22280_e24131_d_n5;
        var_itat_dn6 = assign22280_e24131_d_n6;
        var_itat_dn7 = assign22280_e24131_d_n7;
        var_itat_dn8 = assign22280_e24131_d_n8;

        *var_asrh_slot = var_asrh;
        *var_asrh_dn5_slot = var_asrh_dn5;
        *var_asrh_dn6_slot = var_asrh_dn6;
        *var_asrh_dn7_slot = var_asrh_dn7;
        *var_asrh_dn8_slot = var_asrh_dn8;
        *var_btat_slot = var_btat;
        *var_btat_dn5_slot = var_btat_dn5;
        *var_btat_dn6_slot = var_btat_dn6;
        *var_btat_dn7_slot = var_btat_dn7;
        *var_btat_dn8_slot = var_btat_dn8;
        *var_erfcpos_slot = var_erfcpos;
        *var_erfcpos_dn5_slot = var_erfcpos_dn5;
        *var_erfcpos_dn6_slot = var_erfcpos_dn6;
        *var_erfcpos_dn7_slot = var_erfcpos_dn7;
        *var_erfcpos_dn8_slot = var_erfcpos_dn8;
        *var_erfctimesexpmtat_slot = var_erfctimesexpmtat;
        *var_erfctimesexpmtat_dn5_slot = var_erfctimesexpmtat_dn5;
        *var_erfctimesexpmtat_dn6_slot = var_erfctimesexpmtat_dn6;
        *var_erfctimesexpmtat_dn7_slot = var_erfctimesexpmtat_dn7;
        *var_erfctimesexpmtat_dn8_slot = var_erfctimesexpmtat_dn8;
        *var_gammamax_slot = var_gammamax;
        *var_gammamax_dn5_slot = var_gammamax_dn5;
        *var_gammamax_dn6_slot = var_gammamax_dn6;
        *var_gammamax_dn7_slot = var_gammamax_dn7;
        *var_gammamax_dn8_slot = var_gammamax_dn8;
        *var_guard397_slot = var_guard397;
        *var_guard398_slot = var_guard398;
        *var_guard399_slot = var_guard399;
        *var_guard400_slot = var_guard400;
        *var_guard401_slot = var_guard401;
        *var_guard402_slot = var_guard402;
        *var_isrh_slot = var_isrh;
        *var_isrh_dn5_slot = var_isrh_dn5;
        *var_isrh_dn6_slot = var_isrh_dn6;
        *var_isrh_dn7_slot = var_isrh_dn7;
        *var_isrh_dn8_slot = var_isrh_dn8;
        *var_itat_slot = var_itat;
        *var_itat_dn5_slot = var_itat_dn5;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_ktat_slot = var_ktat;
        *var_ktat_dn5_slot = var_ktat_dn5;
        *var_ktat_dn6_slot = var_ktat_dn6;
        *var_ktat_dn7_slot = var_ktat_dn7;
        *var_ktat_dn8_slot = var_ktat_dn8;
        *var_ltat_slot = var_ltat;
        *var_ltat_dn5_slot = var_ltat_dn5;
        *var_ltat_dn6_slot = var_ltat_dn6;
        *var_ltat_dn7_slot = var_ltat_dn7;
        *var_ltat_dn8_slot = var_ltat_dn8;
        *var_mtat_slot = var_mtat;
        *var_mtat_dn5_slot = var_mtat_dn5;
        *var_mtat_dn6_slot = var_mtat_dn6;
        *var_mtat_dn7_slot = var_mtat_dn7;
        *var_mtat_dn8_slot = var_mtat_dn8;
        *var_sqrtumax_slot = var_sqrtumax;
        *var_sqrtumax_dn5_slot = var_sqrtumax_dn5;
        *var_sqrtumax_dn6_slot = var_sqrtumax_dn6;
        *var_sqrtumax_dn7_slot = var_sqrtumax_dn7;
        *var_sqrtumax_dn8_slot = var_sqrtumax_dn8;
        *var_terfc_slot = var_terfc;
        *var_terfc_dn5_slot = var_terfc_dn5;
        *var_terfc_dn6_slot = var_terfc_dn6;
        *var_terfc_dn7_slot = var_terfc_dn7;
        *var_terfc_dn8_slot = var_terfc_dn8;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn5_slot = var_tmp_dn5;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_twoatatoverthreebtat_slot = var_twoatatoverthreebtat;
        *var_twoatatoverthreebtat_dn5_slot = var_twoatatoverthreebtat_dn5;
        *var_twoatatoverthreebtat_dn6_slot = var_twoatatoverthreebtat_dn6;
        *var_twoatatoverthreebtat_dn7_slot = var_twoatatoverthreebtat_dn7;
        *var_twoatatoverthreebtat_dn8_slot = var_twoatatoverthreebtat_dn8;
        *var_umax_slot = var_umax;
        *var_umax_dn5_slot = var_umax_dn5;
        *var_umax_dn6_slot = var_umax_dn6;
        *var_umax_dn7_slot = var_umax_dn7;
        *var_umax_dn8_slot = var_umax_dn8;
        *var_umaxbeforelimiting_slot = var_umaxbeforelimiting;
        *var_umaxbeforelimiting_dn5_slot = var_umaxbeforelimiting_dn5;
        *var_umaxbeforelimiting_dn6_slot = var_umaxbeforelimiting_dn6;
        *var_umaxbeforelimiting_dn7_slot = var_umaxbeforelimiting_dn7;
        *var_umaxbeforelimiting_dn8_slot = var_umaxbeforelimiting_dn8;
        *var_umaxpoweronepointfive_slot = var_umaxpoweronepointfive;
        *var_umaxpoweronepointfive_dn5_slot = var_umaxpoweronepointfive_dn5;
        *var_umaxpoweronepointfive_dn6_slot = var_umaxpoweronepointfive_dn6;
        *var_umaxpoweronepointfive_dn7_slot = var_umaxpoweronepointfive_dn7;
        *var_umaxpoweronepointfive_dn8_slot = var_umaxpoweronepointfive_dn8;
        *var_wdep_slot = var_wdep;
        *var_wdep_dn5_slot = var_wdep_dn5;
        *var_wdep_dn6_slot = var_wdep_dn6;
        *var_wdep_dn7_slot = var_wdep_dn7;
        *var_wdep_dn8_slot = var_wdep_dn8;
        *var_wgamma_slot = var_wgamma;
        *var_wgamma_dn5_slot = var_wgamma_dn5;
        *var_wgamma_dn6_slot = var_wgamma_dn6;
        *var_wgamma_dn7_slot = var_wgamma_dn7;
        *var_wgamma_dn8_slot = var_wgamma_dn8;
        *var_wtat_slot = var_wtat;
        *var_wtat_dn5_slot = var_wtat_dn5;
        *var_wtat_dn6_slot = var_wtat_dn6;
        *var_wtat_dn7_slot = var_wtat_dn7;
        *var_wtat_dn8_slot = var_wtat_dn8;
        *var_xerfc_slot = var_xerfc;
        *var_xerfc_dn5_slot = var_xerfc_dn5;
        *var_xerfc_dn6_slot = var_xerfc_dn6;
        *var_xerfc_dn7_slot = var_xerfc_dn7;
        *var_xerfc_dn8_slot = var_xerfc_dn8;
        *var_ysq_slot = var_ysq;
        *var_ysq_dn5_slot = var_ysq_dn5;
        *var_ysq_dn6_slot = var_ysq_dn6;
        *var_ysq_dn7_slot = var_ysq_dn7;
        *var_ysq_dn8_slot = var_ysq_dn8;
    }

    pub(super) fn stamp_transient_block_40(
        p: &Parameters,
        var_absource_i: f64,
        var_alphaav: f64,
        var_exp_vmax_over_phitd_s: f64,
        var_fbbtgat: f64,
        var_fbbtgat_dn5: f64,
        var_fbbtgat_dn6: f64,
        var_fbbtgat_dn7: f64,
        var_fbbtgat_dn8: f64,
        var_fstopgat: f64,
        var_guard176: f64,
        var_guard193: f64,
        var_guard393: f64,
        var_id__blk213: f64,
        var_ijunbot: f64,
        var_ijunbot_dn5: f64,
        var_ijunbot_dn6: f64,
        var_ijunbot_dn7: f64,
        var_ijunbot_dn8: f64,
        var_ijunsti: f64,
        var_ijunsti_dn5: f64,
        var_ijunsti_dn6: f64,
        var_ijunsti_dn7: f64,
        var_ijunsti_dn8: f64,
        var_isrh: f64,
        var_isrh_dn5: f64,
        var_isrh_dn6: f64,
        var_isrh_dn7: f64,
        var_isrh_dn8: f64,
        var_itat: f64,
        var_itat_dn5: f64,
        var_itat_dn6: f64,
        var_itat_dn7: f64,
        var_itat_dn8: f64,
        var_lgsource_i: f64,
        var_lssource_i: f64,
        var_one_over_one_minus_pgat: f64,
        var_phitd: f64,
        var_phitdinv: f64,
        var_slopegat: f64,
        var_slopegat_dn5: f64,
        var_slopegat_dn6: f64,
        var_slopegat_dn7: f64,
        var_slopegat_dn8: f64,
        var_v3: f64,
        var_v4: f64,
        var_vav: f64,
        var_vbirgatinv: f64,
        var_vbrinvgat: f64,
        var_vbrinvgat_dn5: f64,
        var_vbrinvgat_dn6: f64,
        var_vbrinvgat_dn7: f64,
        var_vbrinvgat_dn8: f64,
        var_vmax_s: f64,
        var_wdepnulrinvgat: f64,
        var_fbreakdown_slot: &mut f64,
        var_fbreakdown_dn5_slot: &mut f64,
        var_fbreakdown_dn6_slot: &mut f64,
        var_fbreakdown_dn7_slot: &mut f64,
        var_fbreakdown_dn8_slot: &mut f64,
        var_fmaxr_slot: &mut f64,
        var_fmaxr_dn5_slot: &mut f64,
        var_fmaxr_dn6_slot: &mut f64,
        var_fmaxr_dn7_slot: &mut f64,
        var_fmaxr_dn8_slot: &mut f64,
        var_guard403_slot: &mut f64,
        var_guard404_slot: &mut f64,
        var_guard405_slot: &mut f64,
        var_guard406_slot: &mut f64,
        var_guard407_slot: &mut f64,
        var_guard408_slot: &mut f64,
        var_guard409_slot: &mut f64,
        var_guard410_slot: &mut f64,
        var_guard411_slot: &mut f64,
        var_guard412_slot: &mut f64,
        var_guard413_slot: &mut f64,
        var_guard414_slot: &mut f64,
        var_i3_slot: &mut f64,
        var_i3_dn5_slot: &mut f64,
        var_i3_dn6_slot: &mut f64,
        var_i3_dn7_slot: &mut f64,
        var_i3_dn8_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn5_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
        var_idmult_slot: &mut f64,
        var_ijungat_slot: &mut f64,
        var_ijungat_dn5_slot: &mut f64,
        var_ijungat_dn6_slot: &mut f64,
        var_ijungat_dn7_slot: &mut f64,
        var_ijungat_dn8_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn5_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_two_psistar_slot: &mut f64,
        var_vbbt_slot: &mut f64,
        var_z_slot: &mut f64,
        var_zinv_slot: &mut f64,
    ) {
        let mut var_fbreakdown: f64 = *var_fbreakdown_slot;
        let mut var_fbreakdown_dn5: f64 = *var_fbreakdown_dn5_slot;
        let mut var_fbreakdown_dn6: f64 = *var_fbreakdown_dn6_slot;
        let mut var_fbreakdown_dn7: f64 = *var_fbreakdown_dn7_slot;
        let mut var_fbreakdown_dn8: f64 = *var_fbreakdown_dn8_slot;
        let mut var_fmaxr: f64 = *var_fmaxr_slot;
        let mut var_fmaxr_dn5: f64 = *var_fmaxr_dn5_slot;
        let mut var_fmaxr_dn6: f64 = *var_fmaxr_dn6_slot;
        let mut var_fmaxr_dn7: f64 = *var_fmaxr_dn7_slot;
        let mut var_fmaxr_dn8: f64 = *var_fmaxr_dn8_slot;
        let mut var_guard403: f64 = *var_guard403_slot;
        let mut var_guard404: f64 = *var_guard404_slot;
        let mut var_guard405: f64 = *var_guard405_slot;
        let mut var_guard406: f64 = *var_guard406_slot;
        let mut var_guard407: f64 = *var_guard407_slot;
        let mut var_guard408: f64 = *var_guard408_slot;
        let mut var_guard409: f64 = *var_guard409_slot;
        let mut var_guard410: f64 = *var_guard410_slot;
        let mut var_guard411: f64 = *var_guard411_slot;
        let mut var_guard412: f64 = *var_guard412_slot;
        let mut var_guard413: f64 = *var_guard413_slot;
        let mut var_guard414: f64 = *var_guard414_slot;
        let mut var_i3: f64 = *var_i3_slot;
        let mut var_i3_dn5: f64 = *var_i3_dn5_slot;
        let mut var_i3_dn6: f64 = *var_i3_dn6_slot;
        let mut var_i3_dn7: f64 = *var_i3_dn7_slot;
        let mut var_i3_dn8: f64 = *var_i3_dn8_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn5: f64 = *var_ibbt_dn5_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
        let mut var_idmult: f64 = *var_idmult_slot;
        let mut var_ijungat: f64 = *var_ijungat_slot;
        let mut var_ijungat_dn5: f64 = *var_ijungat_dn5_slot;
        let mut var_ijungat_dn6: f64 = *var_ijungat_dn6_slot;
        let mut var_ijungat_dn7: f64 = *var_ijungat_dn7_slot;
        let mut var_ijungat_dn8: f64 = *var_ijungat_dn8_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn5: f64 = *var_tmp_dn5_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_two_psistar: f64 = *var_two_psistar_slot;
        let mut var_vbbt: f64 = *var_vbbt_slot;
        let mut var_z: f64 = *var_z_slot;
        let mut var_zinv: f64 = *var_zinv_slot;

        let assign22290_e24134: f64 = if p.p846 == 0.0 { 1.0 } else { 0.0 };
        var_guard403 = assign22290_e24134;

        let (assign22300_e24145, assign22300_e24145_d_n5, assign22300_e24145_d_n6, assign22300_e24145_d_n7, assign22300_e24145_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) && (var_guard403 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign22300_e24145;
        var_ibbt_dn5 = assign22300_e24145_d_n5;
        var_ibbt_dn6 = assign22300_e24145_d_n6;
        var_ibbt_dn7 = assign22300_e24145_d_n7;
        var_ibbt_dn8 = assign22300_e24145_d_n8;

        let assign22310_e24148: f64 = if p.p826 == 0.5 { 1.0 } else { 0.0 };
        var_guard404 = assign22310_e24148;

        let (assign22320_e24167, assign22320_e24167_d_n5, assign22320_e24167_d_n6, assign22320_e24167_d_n7, assign22320_e24167_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) && (var_guard403 == 0.0)) && (var_guard404 != 0.0)) {
        let assign22320_e24162: f64 = (p.p823 - var_vbbt);
        let assign22320_e24164: f64 = (assign22320_e24162 * var_vbirgatinv);
        let assign22320_e24165: f64 = (assign22320_e24164).sqrt();
        (assign22320_e24165, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign22320_e24167;
        var_tmp_dn5 = assign22320_e24167_d_n5;
        var_tmp_dn6 = assign22320_e24167_d_n6;
        var_tmp_dn7 = assign22320_e24167_d_n7;
        var_tmp_dn8 = assign22320_e24167_d_n8;

        let (assign22330_e24188, assign22330_e24188_d_n5, assign22330_e24188_d_n6, assign22330_e24188_d_n7, assign22330_e24188_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) && (var_guard403 == 0.0)) && (var_guard404 == 0.0)) {
        let assign22330_e24182: f64 = (p.p823 - var_vbbt);
        let assign22330_e24184: f64 = (assign22330_e24182 * var_vbirgatinv);
        let assign22330_e24186: f64 = (assign22330_e24184).powf(p.p826);
        (assign22330_e24186, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign22330_e24188;
        var_tmp_dn5 = assign22330_e24188_d_n5;
        var_tmp_dn6 = assign22330_e24188_d_n6;
        var_tmp_dn7 = assign22330_e24188_d_n7;
        var_tmp_dn8 = assign22330_e24188_d_n8;

        let (assign22340_e24208, assign22340_e24208_d_n5, assign22340_e24208_d_n6, assign22340_e24208_d_n7, assign22340_e24208_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) && (var_guard403 == 0.0)) {
        let assign22340_e24201: f64 = (p.p823 - var_vbbt);
        let assign22340_e24203: f64 = (assign22340_e24201 * var_wdepnulrinvgat);
        let assign22340_e24205: f64 = (assign22340_e24203 / var_tmp);
        let assign22340_e24206: f64 = (var_one_over_one_minus_pgat * assign22340_e24205);
        (assign22340_e24206, (var_one_over_one_minus_pgat * (-((assign22340_e24203 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat * (-((assign22340_e24203 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat * (-((assign22340_e24203 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat * (-((assign22340_e24203 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign22340_e24208;
        var_fmaxr_dn5 = assign22340_e24208_d_n5;
        var_fmaxr_dn6 = assign22340_e24208_d_n6;
        var_fmaxr_dn7 = assign22340_e24208_d_n7;
        var_fmaxr_dn8 = assign22340_e24208_d_n8;

        let assign22350_e24210: f64 = (-var_fbbtgat);
        let assign22350_e24212: f64 = (assign22350_e24210 / var_fmaxr);
        let assign22350_e24213: f64 = (assign22350_e24212).abs();
        let assign22350_e24215: f64 = if assign22350_e24213 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard405 = assign22350_e24215;

        let (assign22360_e24233, assign22360_e24233_d_n5, assign22360_e24233_d_n6, assign22360_e24233_d_n7, assign22360_e24233_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) && (var_guard403 == 0.0)) && (var_guard405 != 0.0)) {
        let assign22360_e24228: f64 = (-var_fbbtgat);
        let assign22360_e24230: f64 = (assign22360_e24228 / var_fmaxr);
        let assign22360_e24231: f64 = (assign22360_e24230).exp();
        (assign22360_e24231, (assign22360_e24231 * ((((-var_fbbtgat_dn5) * var_fmaxr) - (assign22360_e24228 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))), (assign22360_e24231 * ((((-var_fbbtgat_dn6) * var_fmaxr) - (assign22360_e24228 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))), (assign22360_e24231 * ((((-var_fbbtgat_dn7) * var_fmaxr) - (assign22360_e24228 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))), (assign22360_e24231 * ((((-var_fbbtgat_dn8) * var_fmaxr) - (assign22360_e24228 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign22360_e24233;
        var_tmp_dn5 = assign22360_e24233_d_n5;
        var_tmp_dn6 = assign22360_e24233_d_n6;
        var_tmp_dn7 = assign22360_e24233_d_n7;
        var_tmp_dn8 = assign22360_e24233_d_n8;

        let assign22370_e24235: f64 = (-var_fbbtgat);
        let assign22370_e24237: f64 = (assign22370_e24235 / var_fmaxr);
        let assign22370_e24239: f64 = if assign22370_e24237 < 0.0 { 1.0 } else { 0.0 };
        var_guard406 = assign22370_e24239;

        let (assign22380_e24290, assign22380_e24290_d_n5, assign22380_e24290_d_n6, assign22380_e24290_d_n7, assign22380_e24290_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) && (var_guard403 == 0.0)) && (var_guard405 == 0.0)) && (var_guard406 != 0.0)) {
        let assign22380_e24257: f64 = (-230.25850929940458);
        let assign22380_e24259: f64 = (-var_fbbtgat);
        let assign22380_e24261: f64 = (assign22380_e24259 / var_fmaxr);
        let assign22380_e24262: f64 = (assign22380_e24257 - assign22380_e24261);
        let assign22380_e24266: f64 = (-230.25850929940458);
        let assign22380_e24268: f64 = (-var_fbbtgat);
        let assign22380_e24270: f64 = (assign22380_e24268 / var_fmaxr);
        let assign22380_e24271: f64 = (assign22380_e24266 - assign22380_e24270);
        let assign22380_e24274: f64 = (-230.25850929940458);
        let assign22380_e24276: f64 = (-var_fbbtgat);
        let assign22380_e24278: f64 = (assign22380_e24276 / var_fmaxr);
        let assign22380_e24279: f64 = (assign22380_e24274 - assign22380_e24278);
        let assign22380_e24281: f64 = (assign22380_e24279 * 0.3333333333333333);
        let assign22380_e24282: f64 = (1.0 + assign22380_e24281);
        let assign22380_e24283: f64 = (assign22380_e24271 * assign22380_e24282);
        let assign22380_e24284: f64 = (0.5 * assign22380_e24283);
        let assign22380_e24285: f64 = (1.0 + assign22380_e24284);
        let assign22380_e24286: f64 = (assign22380_e24262 * assign22380_e24285);
        let assign22380_e24287: f64 = (1.0 + assign22380_e24286);
        let assign22380_e24288: f64 = (1e-100 / assign22380_e24287);
        (assign22380_e24288, (-((1e-100 * (((-((((-var_fbbtgat_dn5) * var_fmaxr) - (assign22380_e24259 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * assign22380_e24285) + (assign22380_e24262 * (0.5 * (((-((((-var_fbbtgat_dn5) * var_fmaxr) - (assign22380_e24268 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * assign22380_e24282) + (assign22380_e24271 * ((-((((-var_fbbtgat_dn5) * var_fmaxr) - (assign22380_e24276 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign22380_e24287 * assign22380_e24287))), (-((1e-100 * (((-((((-var_fbbtgat_dn6) * var_fmaxr) - (assign22380_e24259 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign22380_e24285) + (assign22380_e24262 * (0.5 * (((-((((-var_fbbtgat_dn6) * var_fmaxr) - (assign22380_e24268 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign22380_e24282) + (assign22380_e24271 * ((-((((-var_fbbtgat_dn6) * var_fmaxr) - (assign22380_e24276 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign22380_e24287 * assign22380_e24287))), (-((1e-100 * (((-((((-var_fbbtgat_dn7) * var_fmaxr) - (assign22380_e24259 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign22380_e24285) + (assign22380_e24262 * (0.5 * (((-((((-var_fbbtgat_dn7) * var_fmaxr) - (assign22380_e24268 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign22380_e24282) + (assign22380_e24271 * ((-((((-var_fbbtgat_dn7) * var_fmaxr) - (assign22380_e24276 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign22380_e24287 * assign22380_e24287))), (-((1e-100 * (((-((((-var_fbbtgat_dn8) * var_fmaxr) - (assign22380_e24259 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign22380_e24285) + (assign22380_e24262 * (0.5 * (((-((((-var_fbbtgat_dn8) * var_fmaxr) - (assign22380_e24268 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign22380_e24282) + (assign22380_e24271 * ((-((((-var_fbbtgat_dn8) * var_fmaxr) - (assign22380_e24276 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign22380_e24287 * assign22380_e24287))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign22380_e24290;
        var_tmp_dn5 = assign22380_e24290_d_n5;
        var_tmp_dn6 = assign22380_e24290_d_n6;
        var_tmp_dn7 = assign22380_e24290_d_n7;
        var_tmp_dn8 = assign22380_e24290_d_n8;

        let (assign22390_e24339, assign22390_e24339_d_n5, assign22390_e24339_d_n6, assign22390_e24339_d_n7, assign22390_e24339_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) && (var_guard403 == 0.0)) && (var_guard405 == 0.0)) && (var_guard406 == 0.0)) {
        let assign22390_e24309: f64 = (-var_fbbtgat);
        let assign22390_e24311: f64 = (assign22390_e24309 / var_fmaxr);
        let assign22390_e24313: f64 = (assign22390_e24311 - 230.25850929940458);
        let assign22390_e24317: f64 = (-var_fbbtgat);
        let assign22390_e24319: f64 = (assign22390_e24317 / var_fmaxr);
        let assign22390_e24321: f64 = (assign22390_e24319 - 230.25850929940458);
        let assign22390_e24324: f64 = (-var_fbbtgat);
        let assign22390_e24326: f64 = (assign22390_e24324 / var_fmaxr);
        let assign22390_e24328: f64 = (assign22390_e24326 - 230.25850929940458);
        let assign22390_e24330: f64 = (assign22390_e24328 * 0.3333333333333333);
        let assign22390_e24331: f64 = (1.0 + assign22390_e24330);
        let assign22390_e24332: f64 = (assign22390_e24321 * assign22390_e24331);
        let assign22390_e24333: f64 = (0.5 * assign22390_e24332);
        let assign22390_e24334: f64 = (1.0 + assign22390_e24333);
        let assign22390_e24335: f64 = (assign22390_e24313 * assign22390_e24334);
        let assign22390_e24336: f64 = (1.0 + assign22390_e24335);
        let assign22390_e24337: f64 = (1e100 * assign22390_e24336);
        (assign22390_e24337, (1e100 * ((((((-var_fbbtgat_dn5) * var_fmaxr) - (assign22390_e24309 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * assign22390_e24334) + (assign22390_e24313 * (0.5 * ((((((-var_fbbtgat_dn5) * var_fmaxr) - (assign22390_e24317 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * assign22390_e24331) + (assign22390_e24321 * (((((-var_fbbtgat_dn5) * var_fmaxr) - (assign22390_e24324 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_dn6) * var_fmaxr) - (assign22390_e24309 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign22390_e24334) + (assign22390_e24313 * (0.5 * ((((((-var_fbbtgat_dn6) * var_fmaxr) - (assign22390_e24317 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign22390_e24331) + (assign22390_e24321 * (((((-var_fbbtgat_dn6) * var_fmaxr) - (assign22390_e24324 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_dn7) * var_fmaxr) - (assign22390_e24309 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign22390_e24334) + (assign22390_e24313 * (0.5 * ((((((-var_fbbtgat_dn7) * var_fmaxr) - (assign22390_e24317 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign22390_e24331) + (assign22390_e24321 * (((((-var_fbbtgat_dn7) * var_fmaxr) - (assign22390_e24324 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_dn8) * var_fmaxr) - (assign22390_e24309 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign22390_e24334) + (assign22390_e24313 * (0.5 * ((((((-var_fbbtgat_dn8) * var_fmaxr) - (assign22390_e24317 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign22390_e24331) + (assign22390_e24321 * (((((-var_fbbtgat_dn8) * var_fmaxr) - (assign22390_e24324 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign22390_e24339;
        var_tmp_dn5 = assign22390_e24339_d_n5;
        var_tmp_dn6 = assign22390_e24339_d_n6;
        var_tmp_dn7 = assign22390_e24339_d_n7;
        var_tmp_dn8 = assign22390_e24339_d_n8;

        let (assign22400_e24359, assign22400_e24359_d_n5, assign22400_e24359_d_n6, assign22400_e24359_d_n7, assign22400_e24359_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) && (var_guard403 == 0.0)) {
        let assign22400_e24352: f64 = (var_v3 * var_fmaxr);
        let assign22400_e24354: f64 = (assign22400_e24352 * var_fmaxr);
        let assign22400_e24356: f64 = (assign22400_e24354 * var_tmp);
        let assign22400_e24357: f64 = (p.p846 * assign22400_e24356);
        (assign22400_e24357, (p.p846 * (((((var_v3 * var_fmaxr_dn5) * var_fmaxr) + (assign22400_e24352 * var_fmaxr_dn5)) * var_tmp) + (assign22400_e24354 * var_tmp_dn5))), (p.p846 * (((((var_v3 * var_fmaxr_dn6) * var_fmaxr) + (assign22400_e24352 * var_fmaxr_dn6)) * var_tmp) + (assign22400_e24354 * var_tmp_dn6))), (p.p846 * (((((var_v3 * var_fmaxr_dn7) * var_fmaxr) + (assign22400_e24352 * var_fmaxr_dn7)) * var_tmp) + (assign22400_e24354 * var_tmp_dn7))), (p.p846 * (((((var_v3 * var_fmaxr_dn8) * var_fmaxr) + (assign22400_e24352 * var_fmaxr_dn8)) * var_tmp) + (assign22400_e24354 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign22400_e24359;
        var_ibbt_dn5 = assign22400_e24359_d_n5;
        var_ibbt_dn6 = assign22400_e24359_d_n6;
        var_ibbt_dn7 = assign22400_e24359_d_n7;
        var_ibbt_dn8 = assign22400_e24359_d_n8;

        let assign22410_e24362: f64 = if p.p855 > 1000.0 { 1.0 } else { 0.0 };
        var_guard407 = assign22410_e24362;

        let (assign22420_e24373, assign22420_e24373_d_n5, assign22420_e24373_d_n6, assign22420_e24373_d_n7, assign22420_e24373_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) && (var_guard407 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign22420_e24373;
        var_fbreakdown_dn5 = assign22420_e24373_d_n5;
        var_fbreakdown_dn6 = assign22420_e24373_d_n6;
        var_fbreakdown_dn7 = assign22420_e24373_d_n7;
        var_fbreakdown_dn8 = assign22420_e24373_d_n8;

        let assign22430_e24376: f64 = (-var_alphaav);
        let assign22430_e24378: f64 = (assign22430_e24376 * p.p855);
        let assign22430_e24379: f64 = if var_vav > assign22430_e24378 { 1.0 } else { 0.0 };
        var_guard408 = assign22430_e24379;

        let assign22440_e24382: f64 = if p.p858 == 4.0 { 1.0 } else { 0.0 };
        var_guard409 = assign22440_e24382;

        let (assign22450_e24412, assign22450_e24412_d_n5, assign22450_e24412_d_n6, assign22450_e24412_d_n7, assign22450_e24412_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) && (var_guard407 == 0.0)) && (var_guard408 != 0.0)) && (var_guard409 != 0.0)) {
        let assign22450_e24398: f64 = (var_vav * var_vbrinvgat);
        let assign22450_e24401: f64 = (var_vav * var_vbrinvgat);
        let assign22450_e24402: f64 = (assign22450_e24398 * assign22450_e24401);
        let assign22450_e24405: f64 = (var_vav * var_vbrinvgat);
        let assign22450_e24406: f64 = (assign22450_e24402 * assign22450_e24405);
        let assign22450_e24409: f64 = (var_vav * var_vbrinvgat);
        let assign22450_e24410: f64 = (assign22450_e24406 * assign22450_e24409);
        (assign22450_e24410, (((((((var_vav * var_vbrinvgat_dn5) * assign22450_e24401) + (assign22450_e24398 * (var_vav * var_vbrinvgat_dn5))) * assign22450_e24405) + (assign22450_e24402 * (var_vav * var_vbrinvgat_dn5))) * assign22450_e24409) + (assign22450_e24406 * (var_vav * var_vbrinvgat_dn5))), (((((((var_vav * var_vbrinvgat_dn6) * assign22450_e24401) + (assign22450_e24398 * (var_vav * var_vbrinvgat_dn6))) * assign22450_e24405) + (assign22450_e24402 * (var_vav * var_vbrinvgat_dn6))) * assign22450_e24409) + (assign22450_e24406 * (var_vav * var_vbrinvgat_dn6))), (((((((var_vav * var_vbrinvgat_dn7) * assign22450_e24401) + (assign22450_e24398 * (var_vav * var_vbrinvgat_dn7))) * assign22450_e24405) + (assign22450_e24402 * (var_vav * var_vbrinvgat_dn7))) * assign22450_e24409) + (assign22450_e24406 * (var_vav * var_vbrinvgat_dn7))), (((((((var_vav * var_vbrinvgat_dn8) * assign22450_e24401) + (assign22450_e24398 * (var_vav * var_vbrinvgat_dn8))) * assign22450_e24405) + (assign22450_e24402 * (var_vav * var_vbrinvgat_dn8))) * assign22450_e24409) + (assign22450_e24406 * (var_vav * var_vbrinvgat_dn8))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign22450_e24412;
        var_tmp_dn5 = assign22450_e24412_d_n5;
        var_tmp_dn6 = assign22450_e24412_d_n6;
        var_tmp_dn7 = assign22450_e24412_d_n7;
        var_tmp_dn8 = assign22450_e24412_d_n8;

        let (assign22460_e24434, assign22460_e24434_d_n5, assign22460_e24434_d_n6, assign22460_e24434_d_n7, assign22460_e24434_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) && (var_guard407 == 0.0)) && (var_guard408 != 0.0)) && (var_guard409 == 0.0)) {
        let assign22460_e24429: f64 = (var_vav * var_vbrinvgat);
        let assign22460_e24430: f64 = (assign22460_e24429).abs();
        let assign22460_e24432: f64 = (assign22460_e24430).powf(p.p858);
        (assign22460_e24432, if 0.0 == 0.0 && ((p.p858) as f64).is_finite() && ((p.p858) as f64).fract() == 0.0 { if p.p858 == 0.0 { 0.0 } else { (p.p858 * ((assign22460_e24430).powf(p.p858 - 1.0) * if assign22460_e24429 >= 0.0 { (var_vav * var_vbrinvgat_dn5) } else { (-(var_vav * var_vbrinvgat_dn5)) })) } } else { (assign22460_e24432 * (p.p858 * (if assign22460_e24429 >= 0.0 { (var_vav * var_vbrinvgat_dn5) } else { (-(var_vav * var_vbrinvgat_dn5)) } / assign22460_e24430))) }, if 0.0 == 0.0 && ((p.p858) as f64).is_finite() && ((p.p858) as f64).fract() == 0.0 { if p.p858 == 0.0 { 0.0 } else { (p.p858 * ((assign22460_e24430).powf(p.p858 - 1.0) * if assign22460_e24429 >= 0.0 { (var_vav * var_vbrinvgat_dn6) } else { (-(var_vav * var_vbrinvgat_dn6)) })) } } else { (assign22460_e24432 * (p.p858 * (if assign22460_e24429 >= 0.0 { (var_vav * var_vbrinvgat_dn6) } else { (-(var_vav * var_vbrinvgat_dn6)) } / assign22460_e24430))) }, if 0.0 == 0.0 && ((p.p858) as f64).is_finite() && ((p.p858) as f64).fract() == 0.0 { if p.p858 == 0.0 { 0.0 } else { (p.p858 * ((assign22460_e24430).powf(p.p858 - 1.0) * if assign22460_e24429 >= 0.0 { (var_vav * var_vbrinvgat_dn7) } else { (-(var_vav * var_vbrinvgat_dn7)) })) } } else { (assign22460_e24432 * (p.p858 * (if assign22460_e24429 >= 0.0 { (var_vav * var_vbrinvgat_dn7) } else { (-(var_vav * var_vbrinvgat_dn7)) } / assign22460_e24430))) }, if 0.0 == 0.0 && ((p.p858) as f64).is_finite() && ((p.p858) as f64).fract() == 0.0 { if p.p858 == 0.0 { 0.0 } else { (p.p858 * ((assign22460_e24430).powf(p.p858 - 1.0) * if assign22460_e24429 >= 0.0 { (var_vav * var_vbrinvgat_dn8) } else { (-(var_vav * var_vbrinvgat_dn8)) })) } } else { (assign22460_e24432 * (p.p858 * (if assign22460_e24429 >= 0.0 { (var_vav * var_vbrinvgat_dn8) } else { (-(var_vav * var_vbrinvgat_dn8)) } / assign22460_e24430))) },)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign22460_e24434;
        var_tmp_dn5 = assign22460_e24434_d_n5;
        var_tmp_dn6 = assign22460_e24434_d_n6;
        var_tmp_dn7 = assign22460_e24434_d_n7;
        var_tmp_dn8 = assign22460_e24434_d_n8;

        let (assign22470_e24452, assign22470_e24452_d_n5, assign22470_e24452_d_n6, assign22470_e24452_d_n7, assign22470_e24452_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) && (var_guard407 == 0.0)) && (var_guard408 != 0.0)) {
        let assign22470_e24449: f64 = (1.0 - var_tmp);
        let assign22470_e24450: f64 = (1.0 / assign22470_e24449);
        (assign22470_e24450, (-((-var_tmp_dn5) / (assign22470_e24449 * assign22470_e24449))), (-((-var_tmp_dn6) / (assign22470_e24449 * assign22470_e24449))), (-((-var_tmp_dn7) / (assign22470_e24449 * assign22470_e24449))), (-((-var_tmp_dn8) / (assign22470_e24449 * assign22470_e24449))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign22470_e24452;
        var_fbreakdown_dn5 = assign22470_e24452_d_n5;
        var_fbreakdown_dn6 = assign22470_e24452_d_n6;
        var_fbreakdown_dn7 = assign22470_e24452_d_n7;
        var_fbreakdown_dn8 = assign22470_e24452_d_n8;

        let (assign22480_e24475, assign22480_e24475_d_n5, assign22480_e24475_d_n6, assign22480_e24475_d_n7, assign22480_e24475_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) && (var_guard407 == 0.0)) && (var_guard408 == 0.0)) {
        let assign22480_e24469: f64 = (var_alphaav * p.p855);
        let assign22480_e24470: f64 = (var_vav + assign22480_e24469);
        let assign22480_e24472: f64 = (assign22480_e24470 * var_slopegat);
        let assign22480_e24473: f64 = (var_fstopgat + assign22480_e24472);
        (assign22480_e24473, (assign22480_e24470 * var_slopegat_dn5), (assign22480_e24470 * var_slopegat_dn6), (assign22480_e24470 * var_slopegat_dn7), (assign22480_e24470 * var_slopegat_dn8),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign22480_e24475;
        var_fbreakdown_dn5 = assign22480_e24475_d_n5;
        var_fbreakdown_dn6 = assign22480_e24475_d_n6;
        var_fbreakdown_dn7 = assign22480_e24475_d_n7;
        var_fbreakdown_dn8 = assign22480_e24475_d_n8;

        let (assign22490_e24494, assign22490_e24494_d_n5, assign22490_e24494_d_n6, assign22490_e24494_d_n7, assign22490_e24494_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard393 == 0.0)) {
        let assign22490_e24485: f64 = (var_id__blk213 + var_isrh);
        let assign22490_e24487: f64 = (assign22490_e24485 + var_itat);
        let assign22490_e24489: f64 = (assign22490_e24487 + var_ibbt);
        let assign22490_e24490: f64 = (p.p29 * assign22490_e24489);
        let assign22490_e24492: f64 = (assign22490_e24490 * var_fbreakdown);
        (assign22490_e24492, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign22490_e24490 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign22490_e24490 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign22490_e24490 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign22490_e24490 * var_fbreakdown_dn8)),)
    } else {
        (var_ijungat, var_ijungat_dn5, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8,)
    }
};
        var_ijungat = assign22490_e24494;
        var_ijungat_dn5 = assign22490_e24494_d_n5;
        var_ijungat_dn6 = assign22490_e24494_d_n6;
        var_ijungat_dn7 = assign22490_e24494_d_n7;
        var_ijungat_dn8 = assign22490_e24494_d_n8;

        let (assign22500_e24510, assign22500_e24510_d_n5, assign22500_e24510_d_n6, assign22500_e24510_d_n7, assign22500_e24510_d_n8,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        let assign22500_e24500: f64 = (var_absource_i * var_ijunbot);
        let assign22500_e24503: f64 = (var_lssource_i * var_ijunsti);
        let assign22500_e24504: f64 = (assign22500_e24500 + assign22500_e24503);
        let assign22500_e24507: f64 = (var_lgsource_i * var_ijungat);
        let assign22500_e24508: f64 = (assign22500_e24504 + assign22500_e24507);
        (assign22500_e24508, (((var_absource_i * var_ijunbot_dn5) + (var_lssource_i * var_ijunsti_dn5)) + (var_lgsource_i * var_ijungat_dn5)), (((var_absource_i * var_ijunbot_dn6) + (var_lssource_i * var_ijunsti_dn6)) + (var_lgsource_i * var_ijungat_dn6)), (((var_absource_i * var_ijunbot_dn7) + (var_lssource_i * var_ijunsti_dn7)) + (var_lgsource_i * var_ijungat_dn7)), (((var_absource_i * var_ijunbot_dn8) + (var_lssource_i * var_ijunsti_dn8)) + (var_lgsource_i * var_ijungat_dn8)),)
    } else {
        (var_i3, var_i3_dn5, var_i3_dn6, var_i3_dn7, var_i3_dn8,)
    }
};
        var_i3 = assign22500_e24510;
        var_i3_dn5 = assign22500_e24510_d_n5;
        var_i3_dn6 = assign22500_e24510_d_n6;
        var_i3_dn7 = assign22500_e24510_d_n7;
        var_i3_dn8 = assign22500_e24510_d_n8;

        let (assign22510_e24516,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        (0.0,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign22510_e24516;

        let (assign22520_e24522,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        (0.0,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign22520_e24522;

        let assign22530_e24534: f64 = if (!(((var_absource_i == 0.0) && (var_lssource_i == 0.0)) && (var_lgsource_i == 0.0))) { 1.0 } else { 0.0 };
        var_guard410 = assign22530_e24534;

        let assign22610_e24620: f64 = if var_v4 < var_vmax_s { 1.0 } else { 0.0 };
        var_guard411 = assign22610_e24620;

        let assign22620_e24622: f64 = (-0.5);
        let assign22620_e24625: f64 = (var_v4 * var_phitdinv);
        let assign22620_e24626: f64 = (assign22620_e24622 * assign22620_e24625);
        let assign22620_e24627: f64 = (assign22620_e24626).abs();
        let assign22620_e24629: f64 = if assign22620_e24627 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard412 = assign22620_e24629;

        let (assign22630_e24647,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard410 != 0.0)) && (var_guard411 != 0.0)) && (var_guard412 != 0.0)) {
        let assign22630_e24640: f64 = (-0.5);
        let assign22630_e24643: f64 = (var_v4 * var_phitdinv);
        let assign22630_e24644: f64 = (assign22630_e24640 * assign22630_e24643);
        let assign22630_e24645: f64 = (assign22630_e24644).exp();
        (assign22630_e24645,)
    } else {
        (var_z,)
    }
};
        var_z = assign22630_e24647;

        let assign22640_e24649: f64 = (-0.5);
        let assign22640_e24652: f64 = (var_v4 * var_phitdinv);
        let assign22640_e24653: f64 = (assign22640_e24649 * assign22640_e24652);
        let assign22640_e24655: f64 = if assign22640_e24653 < 0.0 { 1.0 } else { 0.0 };
        var_guard413 = assign22640_e24655;

        let (assign22650_e24710,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard410 != 0.0)) && (var_guard411 != 0.0)) && (var_guard412 == 0.0)) && (var_guard413 != 0.0)) {
        let assign22650_e24671: f64 = (-230.25850929940458);
        let assign22650_e24673: f64 = (-0.5);
        let assign22650_e24676: f64 = (var_v4 * var_phitdinv);
        let assign22650_e24677: f64 = (assign22650_e24673 * assign22650_e24676);
        let assign22650_e24678: f64 = (assign22650_e24671 - assign22650_e24677);
        let assign22650_e24682: f64 = (-230.25850929940458);
        let assign22650_e24684: f64 = (-0.5);
        let assign22650_e24687: f64 = (var_v4 * var_phitdinv);
        let assign22650_e24688: f64 = (assign22650_e24684 * assign22650_e24687);
        let assign22650_e24689: f64 = (assign22650_e24682 - assign22650_e24688);
        let assign22650_e24692: f64 = (-230.25850929940458);
        let assign22650_e24694: f64 = (-0.5);
        let assign22650_e24697: f64 = (var_v4 * var_phitdinv);
        let assign22650_e24698: f64 = (assign22650_e24694 * assign22650_e24697);
        let assign22650_e24699: f64 = (assign22650_e24692 - assign22650_e24698);
        let assign22650_e24701: f64 = (assign22650_e24699 * 0.3333333333333333);
        let assign22650_e24702: f64 = (1.0 + assign22650_e24701);
        let assign22650_e24703: f64 = (assign22650_e24689 * assign22650_e24702);
        let assign22650_e24704: f64 = (0.5 * assign22650_e24703);
        let assign22650_e24705: f64 = (1.0 + assign22650_e24704);
        let assign22650_e24706: f64 = (assign22650_e24678 * assign22650_e24705);
        let assign22650_e24707: f64 = (1.0 + assign22650_e24706);
        let assign22650_e24708: f64 = (1e-100 / assign22650_e24707);
        (assign22650_e24708,)
    } else {
        (var_z,)
    }
};
        var_z = assign22650_e24710;

        let (assign22660_e24763,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard410 != 0.0)) && (var_guard411 != 0.0)) && (var_guard412 == 0.0)) && (var_guard413 == 0.0)) {
        let assign22660_e24727: f64 = (-0.5);
        let assign22660_e24730: f64 = (var_v4 * var_phitdinv);
        let assign22660_e24731: f64 = (assign22660_e24727 * assign22660_e24730);
        let assign22660_e24733: f64 = (assign22660_e24731 - 230.25850929940458);
        let assign22660_e24737: f64 = (-0.5);
        let assign22660_e24740: f64 = (var_v4 * var_phitdinv);
        let assign22660_e24741: f64 = (assign22660_e24737 * assign22660_e24740);
        let assign22660_e24743: f64 = (assign22660_e24741 - 230.25850929940458);
        let assign22660_e24746: f64 = (-0.5);
        let assign22660_e24749: f64 = (var_v4 * var_phitdinv);
        let assign22660_e24750: f64 = (assign22660_e24746 * assign22660_e24749);
        let assign22660_e24752: f64 = (assign22660_e24750 - 230.25850929940458);
        let assign22660_e24754: f64 = (assign22660_e24752 * 0.3333333333333333);
        let assign22660_e24755: f64 = (1.0 + assign22660_e24754);
        let assign22660_e24756: f64 = (assign22660_e24743 * assign22660_e24755);
        let assign22660_e24757: f64 = (0.5 * assign22660_e24756);
        let assign22660_e24758: f64 = (1.0 + assign22660_e24757);
        let assign22660_e24759: f64 = (assign22660_e24733 * assign22660_e24758);
        let assign22660_e24760: f64 = (1.0 + assign22660_e24759);
        let assign22660_e24761: f64 = (1e100 * assign22660_e24760);
        (assign22660_e24761,)
    } else {
        (var_z,)
    }
};
        var_z = assign22660_e24763;

        let (assign22670_e24775,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard410 != 0.0)) && (var_guard411 != 0.0)) {
        let assign22670_e24773: f64 = (1.0 / var_z);
        (assign22670_e24773,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign22670_e24775;

        let (assign22680_e24787,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard410 != 0.0)) && (var_guard411 != 0.0)) {
        let assign22680_e24785: f64 = (var_zinv * var_zinv);
        (assign22680_e24785,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign22680_e24787;

        let (assign22690_e24806,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard410 != 0.0)) && (var_guard411 == 0.0)) {
        let assign22690_e24799: f64 = (var_v4 - var_vmax_s);
        let assign22690_e24801: f64 = (assign22690_e24799 * var_phitdinv);
        let assign22690_e24802: f64 = (1.0 + assign22690_e24801);
        let assign22690_e24804: f64 = (assign22690_e24802 * var_exp_vmax_over_phitd_s);
        (assign22690_e24804,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign22690_e24806;

        let (assign22700_e24818,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard410 != 0.0)) && (var_guard411 == 0.0)) {
        let assign22700_e24816: f64 = (var_idmult).sqrt();
        (assign22700_e24816,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign22700_e24818;

        let (assign22710_e24831,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard410 != 0.0)) && (var_guard411 == 0.0)) {
        let assign22710_e24829: f64 = (1.0 / var_zinv);
        (assign22710_e24829,)
    } else {
        (var_z,)
    }
};
        var_z = assign22710_e24831;

        let (assign22720_e24841,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard410 != 0.0)) {
        let assign22720_e24839: f64 = (var_idmult - 1.0);
        (assign22720_e24839,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign22720_e24841;

        let assign22730_e24844: f64 = if var_v4 > 0.0 { 1.0 } else { 0.0 };
        var_guard414 = assign22730_e24844;

        let (assign22740_e24870,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard410 != 0.0)) && (var_guard414 != 0.0)) {
        let assign22740_e24856: f64 = (2.0 + var_z);
        let assign22740_e24859: f64 = (var_z + 1.0);
        let assign22740_e24862: f64 = (var_z + 3.0);
        let assign22740_e24863: f64 = (assign22740_e24859 * assign22740_e24862);
        let assign22740_e24864: f64 = (assign22740_e24863).sqrt();
        let assign22740_e24865: f64 = (assign22740_e24856 + assign22740_e24864);
        let assign22740_e24866: f64 = (assign22740_e24865).ln();
        let assign22740_e24867: f64 = (var_phitd * assign22740_e24866);
        let assign22740_e24868: f64 = (2.0 * assign22740_e24867);
        (assign22740_e24868,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign22740_e24870;

        *var_fbreakdown_slot = var_fbreakdown;
        *var_fbreakdown_dn5_slot = var_fbreakdown_dn5;
        *var_fbreakdown_dn6_slot = var_fbreakdown_dn6;
        *var_fbreakdown_dn7_slot = var_fbreakdown_dn7;
        *var_fbreakdown_dn8_slot = var_fbreakdown_dn8;
        *var_fmaxr_slot = var_fmaxr;
        *var_fmaxr_dn5_slot = var_fmaxr_dn5;
        *var_fmaxr_dn6_slot = var_fmaxr_dn6;
        *var_fmaxr_dn7_slot = var_fmaxr_dn7;
        *var_fmaxr_dn8_slot = var_fmaxr_dn8;
        *var_guard403_slot = var_guard403;
        *var_guard404_slot = var_guard404;
        *var_guard405_slot = var_guard405;
        *var_guard406_slot = var_guard406;
        *var_guard407_slot = var_guard407;
        *var_guard408_slot = var_guard408;
        *var_guard409_slot = var_guard409;
        *var_guard410_slot = var_guard410;
        *var_guard411_slot = var_guard411;
        *var_guard412_slot = var_guard412;
        *var_guard413_slot = var_guard413;
        *var_guard414_slot = var_guard414;
        *var_i3_slot = var_i3;
        *var_i3_dn5_slot = var_i3_dn5;
        *var_i3_dn6_slot = var_i3_dn6;
        *var_i3_dn7_slot = var_i3_dn7;
        *var_i3_dn8_slot = var_i3_dn8;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn5_slot = var_ibbt_dn5;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
        *var_idmult_slot = var_idmult;
        *var_ijungat_slot = var_ijungat;
        *var_ijungat_dn5_slot = var_ijungat_dn5;
        *var_ijungat_dn6_slot = var_ijungat_dn6;
        *var_ijungat_dn7_slot = var_ijungat_dn7;
        *var_ijungat_dn8_slot = var_ijungat_dn8;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn5_slot = var_tmp_dn5;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_two_psistar_slot = var_two_psistar;
        *var_vbbt_slot = var_vbbt;
        *var_z_slot = var_z;
        *var_zinv_slot = var_zinv;
    }

    pub(super) fn stamp_transient_block_41(
        p: &Parameters,
        var_absource_i: f64,
        var_atatbot: f64,
        var_btatpartbot: f64,
        var_ftdbot: f64,
        var_guard176: f64,
        var_guard193: f64,
        var_guard410: f64,
        var_guard414: f64,
        var_idmult: f64,
        var_idsatbot: f64,
        var_one_minus_pbot: f64,
        var_one_over_one_minus_pbot: f64,
        var_phitd: f64,
        var_phitr: f64,
        var_v4: f64,
        var_vbbtlim_s: f64,
        var_vbibot: f64,
        var_vbimin_s: f64,
        var_vbirbotinv: f64,
        var_wdepnulrbot: f64,
        var_zinv: f64,
        var_asrh_slot: &mut f64,
        var_asrh_dn5_slot: &mut f64,
        var_asrh_dn6_slot: &mut f64,
        var_asrh_dn7_slot: &mut f64,
        var_asrh_dn8_slot: &mut f64,
        var_btat_slot: &mut f64,
        var_btat_dn5_slot: &mut f64,
        var_btat_dn6_slot: &mut f64,
        var_btat_dn7_slot: &mut f64,
        var_btat_dn8_slot: &mut f64,
        var_dwsrh_slot: &mut f64,
        var_guard415_slot: &mut f64,
        var_guard416_slot: &mut f64,
        var_guard417_slot: &mut f64,
        var_guard418_slot: &mut f64,
        var_guard419_slot: &mut f64,
        var_guard420_slot: &mut f64,
        var_id__blk213_slot: &mut f64,
        var_ijunbot_slot: &mut f64,
        var_ijunbot_dn5_slot: &mut f64,
        var_ijunbot_dn6_slot: &mut f64,
        var_ijunbot_dn7_slot: &mut f64,
        var_ijunbot_dn8_slot: &mut f64,
        var_isrh_slot: &mut f64,
        var_isrh_dn5_slot: &mut f64,
        var_isrh_dn6_slot: &mut f64,
        var_isrh_dn7_slot: &mut f64,
        var_isrh_dn8_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn5_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_ktat_slot: &mut f64,
        var_ktat_dn5_slot: &mut f64,
        var_ktat_dn6_slot: &mut f64,
        var_ktat_dn7_slot: &mut f64,
        var_ktat_dn8_slot: &mut f64,
        var_ltat_slot: &mut f64,
        var_ltat_dn5_slot: &mut f64,
        var_ltat_dn6_slot: &mut f64,
        var_ltat_dn7_slot: &mut f64,
        var_ltat_dn8_slot: &mut f64,
        var_mtat_slot: &mut f64,
        var_mtat_dn5_slot: &mut f64,
        var_mtat_dn6_slot: &mut f64,
        var_mtat_dn7_slot: &mut f64,
        var_mtat_dn8_slot: &mut f64,
        var_sqrtumax_slot: &mut f64,
        var_sqrtumax_dn5_slot: &mut f64,
        var_sqrtumax_dn6_slot: &mut f64,
        var_sqrtumax_dn7_slot: &mut f64,
        var_sqrtumax_dn8_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn5_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_two_psistar_slot: &mut f64,
        var_twoatatoverthreebtat_slot: &mut f64,
        var_twoatatoverthreebtat_dn5_slot: &mut f64,
        var_twoatatoverthreebtat_dn6_slot: &mut f64,
        var_twoatatoverthreebtat_dn7_slot: &mut f64,
        var_twoatatoverthreebtat_dn8_slot: &mut f64,
        var_umax_slot: &mut f64,
        var_umax_dn5_slot: &mut f64,
        var_umax_dn6_slot: &mut f64,
        var_umax_dn7_slot: &mut f64,
        var_umax_dn8_slot: &mut f64,
        var_umaxbeforelimiting_slot: &mut f64,
        var_umaxbeforelimiting_dn5_slot: &mut f64,
        var_umaxbeforelimiting_dn6_slot: &mut f64,
        var_umaxbeforelimiting_dn7_slot: &mut f64,
        var_umaxbeforelimiting_dn8_slot: &mut f64,
        var_umaxpoweronepointfive_slot: &mut f64,
        var_umaxpoweronepointfive_dn5_slot: &mut f64,
        var_umaxpoweronepointfive_dn6_slot: &mut f64,
        var_umaxpoweronepointfive_dn7_slot: &mut f64,
        var_umaxpoweronepointfive_dn8_slot: &mut f64,
        var_vav_slot: &mut f64,
        var_vbbt_slot: &mut f64,
        var_vbi_minus_vjsrh_slot: &mut f64,
        var_vjlim_slot: &mut f64,
        var_vjsrh_slot: &mut f64,
        var_wdep_slot: &mut f64,
        var_wdep_dn5_slot: &mut f64,
        var_wdep_dn6_slot: &mut f64,
        var_wdep_dn7_slot: &mut f64,
        var_wdep_dn8_slot: &mut f64,
        var_wgamma_slot: &mut f64,
        var_wgamma_dn5_slot: &mut f64,
        var_wgamma_dn6_slot: &mut f64,
        var_wgamma_dn7_slot: &mut f64,
        var_wgamma_dn8_slot: &mut f64,
        var_wsrh_slot: &mut f64,
        var_wsrhstep_slot: &mut f64,
        var_wtat_slot: &mut f64,
        var_wtat_dn5_slot: &mut f64,
        var_wtat_dn6_slot: &mut f64,
        var_wtat_dn7_slot: &mut f64,
        var_wtat_dn8_slot: &mut f64,
        var_xerfc_slot: &mut f64,
        var_xerfc_dn5_slot: &mut f64,
        var_xerfc_dn6_slot: &mut f64,
        var_xerfc_dn7_slot: &mut f64,
        var_xerfc_dn8_slot: &mut f64,
    ) {
        let mut var_asrh: f64 = *var_asrh_slot;
        let mut var_asrh_dn5: f64 = *var_asrh_dn5_slot;
        let mut var_asrh_dn6: f64 = *var_asrh_dn6_slot;
        let mut var_asrh_dn7: f64 = *var_asrh_dn7_slot;
        let mut var_asrh_dn8: f64 = *var_asrh_dn8_slot;
        let mut var_btat: f64 = *var_btat_slot;
        let mut var_btat_dn5: f64 = *var_btat_dn5_slot;
        let mut var_btat_dn6: f64 = *var_btat_dn6_slot;
        let mut var_btat_dn7: f64 = *var_btat_dn7_slot;
        let mut var_btat_dn8: f64 = *var_btat_dn8_slot;
        let mut var_dwsrh: f64 = *var_dwsrh_slot;
        let mut var_guard415: f64 = *var_guard415_slot;
        let mut var_guard416: f64 = *var_guard416_slot;
        let mut var_guard417: f64 = *var_guard417_slot;
        let mut var_guard418: f64 = *var_guard418_slot;
        let mut var_guard419: f64 = *var_guard419_slot;
        let mut var_guard420: f64 = *var_guard420_slot;
        let mut var_id__blk213: f64 = *var_id__blk213_slot;
        let mut var_ijunbot: f64 = *var_ijunbot_slot;
        let mut var_ijunbot_dn5: f64 = *var_ijunbot_dn5_slot;
        let mut var_ijunbot_dn6: f64 = *var_ijunbot_dn6_slot;
        let mut var_ijunbot_dn7: f64 = *var_ijunbot_dn7_slot;
        let mut var_ijunbot_dn8: f64 = *var_ijunbot_dn8_slot;
        let mut var_isrh: f64 = *var_isrh_slot;
        let mut var_isrh_dn5: f64 = *var_isrh_dn5_slot;
        let mut var_isrh_dn6: f64 = *var_isrh_dn6_slot;
        let mut var_isrh_dn7: f64 = *var_isrh_dn7_slot;
        let mut var_isrh_dn8: f64 = *var_isrh_dn8_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn5: f64 = *var_itat_dn5_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_ktat: f64 = *var_ktat_slot;
        let mut var_ktat_dn5: f64 = *var_ktat_dn5_slot;
        let mut var_ktat_dn6: f64 = *var_ktat_dn6_slot;
        let mut var_ktat_dn7: f64 = *var_ktat_dn7_slot;
        let mut var_ktat_dn8: f64 = *var_ktat_dn8_slot;
        let mut var_ltat: f64 = *var_ltat_slot;
        let mut var_ltat_dn5: f64 = *var_ltat_dn5_slot;
        let mut var_ltat_dn6: f64 = *var_ltat_dn6_slot;
        let mut var_ltat_dn7: f64 = *var_ltat_dn7_slot;
        let mut var_ltat_dn8: f64 = *var_ltat_dn8_slot;
        let mut var_mtat: f64 = *var_mtat_slot;
        let mut var_mtat_dn5: f64 = *var_mtat_dn5_slot;
        let mut var_mtat_dn6: f64 = *var_mtat_dn6_slot;
        let mut var_mtat_dn7: f64 = *var_mtat_dn7_slot;
        let mut var_mtat_dn8: f64 = *var_mtat_dn8_slot;
        let mut var_sqrtumax: f64 = *var_sqrtumax_slot;
        let mut var_sqrtumax_dn5: f64 = *var_sqrtumax_dn5_slot;
        let mut var_sqrtumax_dn6: f64 = *var_sqrtumax_dn6_slot;
        let mut var_sqrtumax_dn7: f64 = *var_sqrtumax_dn7_slot;
        let mut var_sqrtumax_dn8: f64 = *var_sqrtumax_dn8_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn5: f64 = *var_tmp_dn5_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_two_psistar: f64 = *var_two_psistar_slot;
        let mut var_twoatatoverthreebtat: f64 = *var_twoatatoverthreebtat_slot;
        let mut var_twoatatoverthreebtat_dn5: f64 = *var_twoatatoverthreebtat_dn5_slot;
        let mut var_twoatatoverthreebtat_dn6: f64 = *var_twoatatoverthreebtat_dn6_slot;
        let mut var_twoatatoverthreebtat_dn7: f64 = *var_twoatatoverthreebtat_dn7_slot;
        let mut var_twoatatoverthreebtat_dn8: f64 = *var_twoatatoverthreebtat_dn8_slot;
        let mut var_umax: f64 = *var_umax_slot;
        let mut var_umax_dn5: f64 = *var_umax_dn5_slot;
        let mut var_umax_dn6: f64 = *var_umax_dn6_slot;
        let mut var_umax_dn7: f64 = *var_umax_dn7_slot;
        let mut var_umax_dn8: f64 = *var_umax_dn8_slot;
        let mut var_umaxbeforelimiting: f64 = *var_umaxbeforelimiting_slot;
        let mut var_umaxbeforelimiting_dn5: f64 = *var_umaxbeforelimiting_dn5_slot;
        let mut var_umaxbeforelimiting_dn6: f64 = *var_umaxbeforelimiting_dn6_slot;
        let mut var_umaxbeforelimiting_dn7: f64 = *var_umaxbeforelimiting_dn7_slot;
        let mut var_umaxbeforelimiting_dn8: f64 = *var_umaxbeforelimiting_dn8_slot;
        let mut var_umaxpoweronepointfive: f64 = *var_umaxpoweronepointfive_slot;
        let mut var_umaxpoweronepointfive_dn5: f64 = *var_umaxpoweronepointfive_dn5_slot;
        let mut var_umaxpoweronepointfive_dn6: f64 = *var_umaxpoweronepointfive_dn6_slot;
        let mut var_umaxpoweronepointfive_dn7: f64 = *var_umaxpoweronepointfive_dn7_slot;
        let mut var_umaxpoweronepointfive_dn8: f64 = *var_umaxpoweronepointfive_dn8_slot;
        let mut var_vav: f64 = *var_vav_slot;
        let mut var_vbbt: f64 = *var_vbbt_slot;
        let mut var_vbi_minus_vjsrh: f64 = *var_vbi_minus_vjsrh_slot;
        let mut var_vjlim: f64 = *var_vjlim_slot;
        let mut var_vjsrh: f64 = *var_vjsrh_slot;
        let mut var_wdep: f64 = *var_wdep_slot;
        let mut var_wdep_dn5: f64 = *var_wdep_dn5_slot;
        let mut var_wdep_dn6: f64 = *var_wdep_dn6_slot;
        let mut var_wdep_dn7: f64 = *var_wdep_dn7_slot;
        let mut var_wdep_dn8: f64 = *var_wdep_dn8_slot;
        let mut var_wgamma: f64 = *var_wgamma_slot;
        let mut var_wgamma_dn5: f64 = *var_wgamma_dn5_slot;
        let mut var_wgamma_dn6: f64 = *var_wgamma_dn6_slot;
        let mut var_wgamma_dn7: f64 = *var_wgamma_dn7_slot;
        let mut var_wgamma_dn8: f64 = *var_wgamma_dn8_slot;
        let mut var_wsrh: f64 = *var_wsrh_slot;
        let mut var_wsrhstep: f64 = *var_wsrhstep_slot;
        let mut var_wtat: f64 = *var_wtat_slot;
        let mut var_wtat_dn5: f64 = *var_wtat_dn5_slot;
        let mut var_wtat_dn6: f64 = *var_wtat_dn6_slot;
        let mut var_wtat_dn7: f64 = *var_wtat_dn7_slot;
        let mut var_wtat_dn8: f64 = *var_wtat_dn8_slot;
        let mut var_xerfc: f64 = *var_xerfc_slot;
        let mut var_xerfc_dn5: f64 = *var_xerfc_dn5_slot;
        let mut var_xerfc_dn6: f64 = *var_xerfc_dn6_slot;
        let mut var_xerfc_dn7: f64 = *var_xerfc_dn7_slot;
        let mut var_xerfc_dn8: f64 = *var_xerfc_dn8_slot;

        let (assign22750_e24904,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard410 != 0.0)) && (var_guard414 == 0.0)) {
        let assign22750_e24880: f64 = (-var_v4);
        let assign22750_e24885: f64 = (2.0 * var_zinv);
        let assign22750_e24887: f64 = (assign22750_e24885 + 1.0);
        let assign22750_e24890: f64 = (1.0 + var_zinv);
        let assign22750_e24894: f64 = (3.0 * var_zinv);
        let assign22750_e24895: f64 = (1.0 + assign22750_e24894);
        let assign22750_e24896: f64 = (assign22750_e24890 * assign22750_e24895);
        let assign22750_e24897: f64 = (assign22750_e24896).sqrt();
        let assign22750_e24898: f64 = (assign22750_e24887 + assign22750_e24897);
        let assign22750_e24899: f64 = (assign22750_e24898).ln();
        let assign22750_e24900: f64 = (var_phitd * assign22750_e24899);
        let assign22750_e24901: f64 = (2.0 * assign22750_e24900);
        let assign22750_e24902: f64 = (assign22750_e24880 + assign22750_e24901);
        (assign22750_e24902,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign22750_e24904;

        let (assign22760_e24914,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard410 != 0.0)) {
        let assign22760_e24912: f64 = (var_vbimin_s - var_two_psistar);
        (assign22760_e24912,)
    } else {
        (var_vjlim,)
    }
};
        var_vjlim = assign22760_e24914;

        let (assign22770_e24941,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard410 != 0.0)) {
        let assign22770_e24923: f64 = (var_v4 + var_vjlim);
        let assign22770_e24926: f64 = (var_v4 - var_vjlim);
        let assign22770_e24929: f64 = (var_v4 - var_vjlim);
        let assign22770_e24930: f64 = (assign22770_e24926 * assign22770_e24929);
        let assign22770_e24933: f64 = (4.0 * var_phitd);
        let assign22770_e24935: f64 = (assign22770_e24933 * var_phitd);
        let assign22770_e24936: f64 = (assign22770_e24930 + assign22770_e24935);
        let assign22770_e24937: f64 = (assign22770_e24936).sqrt();
        let assign22770_e24938: f64 = (assign22770_e24923 - assign22770_e24937);
        let assign22770_e24939: f64 = (0.5 * assign22770_e24938);
        (assign22770_e24939,)
    } else {
        (var_vjsrh,)
    }
};
        var_vjsrh = assign22770_e24941;

        let (assign22780_e24968,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard410 != 0.0)) {
        let assign22780_e24950: f64 = (var_v4 + var_vbbtlim_s);
        let assign22780_e24953: f64 = (var_v4 - var_vbbtlim_s);
        let assign22780_e24956: f64 = (var_v4 - var_vbbtlim_s);
        let assign22780_e24957: f64 = (assign22780_e24953 * assign22780_e24956);
        let assign22780_e24960: f64 = (4.0 * var_phitr);
        let assign22780_e24962: f64 = (assign22780_e24960 * var_phitr);
        let assign22780_e24963: f64 = (assign22780_e24957 + assign22780_e24962);
        let assign22780_e24964: f64 = (assign22780_e24963).sqrt();
        let assign22780_e24965: f64 = (assign22780_e24950 - assign22780_e24964);
        let assign22780_e24966: f64 = (0.5 * assign22780_e24965);
        (assign22780_e24966,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign22780_e24968;

        let (assign22790_e24995,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard410 != 0.0)) {
        let assign22790_e24977: f64 = var_v4;
        let assign22790_e24980: f64 = var_v4;
        let assign22790_e24983: f64 = var_v4;
        let assign22790_e24984: f64 = (assign22790_e24980 * assign22790_e24983);
        let assign22790_e24987: f64 = (4.0 * 1e-6);
        let assign22790_e24989: f64 = (assign22790_e24987 * 1e-6);
        let assign22790_e24990: f64 = (assign22790_e24984 + assign22790_e24989);
        let assign22790_e24991: f64 = (assign22790_e24990).sqrt();
        let assign22790_e24992: f64 = (assign22790_e24977 - assign22790_e24991);
        let assign22790_e24993: f64 = (0.5 * assign22790_e24992);
        (assign22790_e24993,)
    } else {
        (var_vav,)
    }
};
        var_vav = assign22790_e24995;

        let assign22800_e24998: f64 = if var_absource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard415 = assign22800_e24998;

        let (assign22810_e25006, assign22810_e25006_d_n5, assign22810_e25006_d_n6, assign22810_e25006_d_n7, assign22810_e25006_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunbot, var_ijunbot_dn5, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8,)
    }
};
        var_ijunbot = assign22810_e25006;
        var_ijunbot_dn5 = assign22810_e25006_d_n5;
        var_ijunbot_dn6 = assign22810_e25006_d_n6;
        var_ijunbot_dn7 = assign22810_e25006_d_n7;
        var_ijunbot_dn8 = assign22810_e25006_d_n8;

        let (assign22820_e25017,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) {
        let assign22820_e25015: f64 = (var_idsatbot * var_idmult);
        (assign22820_e25015,)
    } else {
        (var_id__blk213,)
    }
};
        var_id__blk213 = assign22820_e25017;

        let assign22830_e25024: f64 = if ((p.p833 == 0.0) && (p.p838 == 0.0)) { 1.0 } else { 0.0 };
        var_guard416 = assign22830_e25024;

        let (assign22840_e25035, assign22840_e25035_d_n5, assign22840_e25035_d_n6, assign22840_e25035_d_n7, assign22840_e25035_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) && (var_guard416 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign22840_e25035;
        var_isrh_dn5 = assign22840_e25035_d_n5;
        var_isrh_dn6 = assign22840_e25035_d_n6;
        var_isrh_dn7 = assign22840_e25035_d_n7;
        var_isrh_dn8 = assign22840_e25035_d_n8;

        let (assign22850_e25049,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) && (var_guard416 == 0.0)) {
        let assign22850_e25047: f64 = (var_vbibot - var_vjsrh);
        (assign22850_e25047,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign22850_e25049;

        let (assign22860_e25068,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) && (var_guard416 == 0.0)) {
        let assign22860_e25063: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign22860_e25064: f64 = (1.0 - assign22860_e25063);
        let assign22860_e25065: f64 = (assign22860_e25064).sqrt();
        let assign22860_e25066: f64 = (1.0 - assign22860_e25065);
        (assign22860_e25066,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign22860_e25068;

        let assign22870_e25071: f64 = if p.p824 == 0.5 { 1.0 } else { 0.0 };
        var_guard417 = assign22870_e25071;

        let (assign22880_e25085,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) && (var_guard416 == 0.0)) && (var_guard417 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign22880_e25085;

        let (assign22890_e25117,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) && (var_guard416 == 0.0)) && (var_guard417 == 0.0)) {
        let assign22890_e25100: f64 = (var_wsrhstep * var_wsrhstep);
        let assign22890_e25102: f64 = (var_wsrhstep).ln();
        let assign22890_e25103: f64 = (assign22890_e25100 * assign22890_e25102);
        let assign22890_e25106: f64 = (1.0 - var_wsrhstep);
        let assign22890_e25107: f64 = (assign22890_e25103 / assign22890_e25106);
        let assign22890_e25109: f64 = (assign22890_e25107 + var_wsrhstep);
        let assign22890_e25113: f64 = (2.0 * p.p824);
        let assign22890_e25114: f64 = (1.0 - assign22890_e25113);
        let assign22890_e25115: f64 = (assign22890_e25109 * assign22890_e25114);
        (assign22890_e25115,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign22890_e25117;

        let (assign22900_e25131,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) && (var_guard416 == 0.0)) {
        let assign22900_e25129: f64 = (var_wsrhstep + var_dwsrh);
        (assign22900_e25129,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign22900_e25131;

        let assign22910_e25134: f64 = if p.p824 == 0.5 { 1.0 } else { 0.0 };
        var_guard418 = assign22910_e25134;

        let (assign22920_e25151, assign22920_e25151_d_n5, assign22920_e25151_d_n6, assign22920_e25151_d_n7, assign22920_e25151_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) && (var_guard416 == 0.0)) && (var_guard418 != 0.0)) {
        let assign22920_e25148: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv);
        let assign22920_e25149: f64 = (assign22920_e25148).sqrt();
        (assign22920_e25149, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign22920_e25151;
        var_tmp_dn5 = assign22920_e25151_d_n5;
        var_tmp_dn6 = assign22920_e25151_d_n6;
        var_tmp_dn7 = assign22920_e25151_d_n7;
        var_tmp_dn8 = assign22920_e25151_d_n8;

        let (assign22930_e25170, assign22930_e25170_d_n5, assign22930_e25170_d_n6, assign22930_e25170_d_n7, assign22930_e25170_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) && (var_guard416 == 0.0)) && (var_guard418 == 0.0)) {
        let assign22930_e25166: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv);
        let assign22930_e25168: f64 = (assign22930_e25166).powf(p.p824);
        (assign22930_e25168, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign22930_e25170;
        var_tmp_dn5 = assign22930_e25170_d_n5;
        var_tmp_dn6 = assign22930_e25170_d_n6;
        var_tmp_dn7 = assign22930_e25170_d_n7;
        var_tmp_dn8 = assign22930_e25170_d_n8;

        let (assign22940_e25184, assign22940_e25184_d_n5, assign22940_e25184_d_n6, assign22940_e25184_d_n7, assign22940_e25184_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) && (var_guard416 == 0.0)) {
        let assign22940_e25182: f64 = (var_wdepnulrbot * var_tmp);
        (assign22940_e25182, (var_wdepnulrbot * var_tmp_dn5), (var_wdepnulrbot * var_tmp_dn6), (var_wdepnulrbot * var_tmp_dn7), (var_wdepnulrbot * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign22940_e25184;
        var_wdep_dn5 = assign22940_e25184_d_n5;
        var_wdep_dn6 = assign22940_e25184_d_n6;
        var_wdep_dn7 = assign22940_e25184_d_n7;
        var_wdep_dn8 = assign22940_e25184_d_n8;

        let (assign22950_e25202, assign22950_e25202_d_n5, assign22950_e25202_d_n6, assign22950_e25202_d_n7, assign22950_e25202_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) && (var_guard416 == 0.0)) {
        let assign22950_e25197: f64 = (var_zinv - 1.0);
        let assign22950_e25199: f64 = (assign22950_e25197 * var_wdep);
        let assign22950_e25200: f64 = (var_ftdbot * assign22950_e25199);
        (assign22950_e25200, (var_ftdbot * (assign22950_e25197 * var_wdep_dn5)), (var_ftdbot * (assign22950_e25197 * var_wdep_dn6)), (var_ftdbot * (assign22950_e25197 * var_wdep_dn7)), (var_ftdbot * (assign22950_e25197 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign22950_e25202;
        var_asrh_dn5 = assign22950_e25202_d_n5;
        var_asrh_dn6 = assign22950_e25202_d_n6;
        var_asrh_dn7 = assign22950_e25202_d_n7;
        var_asrh_dn8 = assign22950_e25202_d_n8;

        let (assign22960_e25218, assign22960_e25218_d_n5, assign22960_e25218_d_n6, assign22960_e25218_d_n7, assign22960_e25218_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) && (var_guard416 == 0.0)) {
        let assign22960_e25215: f64 = (var_asrh * var_wsrh);
        let assign22960_e25216: f64 = (p.p833 * assign22960_e25215);
        (assign22960_e25216, (p.p833 * (var_asrh_dn5 * var_wsrh)), (p.p833 * (var_asrh_dn6 * var_wsrh)), (p.p833 * (var_asrh_dn7 * var_wsrh)), (p.p833 * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign22960_e25218;
        var_isrh_dn5 = assign22960_e25218_d_n5;
        var_isrh_dn6 = assign22960_e25218_d_n6;
        var_isrh_dn7 = assign22960_e25218_d_n7;
        var_isrh_dn8 = assign22960_e25218_d_n8;

        let assign22970_e25221: f64 = if p.p838 == 0.0 { 1.0 } else { 0.0 };
        var_guard419 = assign22970_e25221;

        let (assign22980_e25232, assign22980_e25232_d_n5, assign22980_e25232_d_n6, assign22980_e25232_d_n7, assign22980_e25232_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) && (var_guard419 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign22980_e25232;
        var_itat_dn5 = assign22980_e25232_d_n5;
        var_itat_dn6 = assign22980_e25232_d_n6;
        var_itat_dn7 = assign22980_e25232_d_n7;
        var_itat_dn8 = assign22980_e25232_d_n8;

        let (assign22990_e25250, assign22990_e25250_d_n5, assign22990_e25250_d_n6, assign22990_e25250_d_n7, assign22990_e25250_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) && (var_guard419 == 0.0)) {
        let assign22990_e25245: f64 = (var_wdep * var_one_minus_pbot);
        let assign22990_e25247: f64 = (assign22990_e25245 / var_vbi_minus_vjsrh);
        let assign22990_e25248: f64 = (var_btatpartbot * assign22990_e25247);
        (assign22990_e25248, (var_btatpartbot * ((var_wdep_dn5 * var_one_minus_pbot) / var_vbi_minus_vjsrh)), (var_btatpartbot * ((var_wdep_dn6 * var_one_minus_pbot) / var_vbi_minus_vjsrh)), (var_btatpartbot * ((var_wdep_dn7 * var_one_minus_pbot) / var_vbi_minus_vjsrh)), (var_btatpartbot * ((var_wdep_dn8 * var_one_minus_pbot) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign22990_e25250;
        var_btat_dn5 = assign22990_e25250_d_n5;
        var_btat_dn6 = assign22990_e25250_d_n6;
        var_btat_dn7 = assign22990_e25250_d_n7;
        var_btat_dn8 = assign22990_e25250_d_n8;

        let (assign23000_e25266, assign23000_e25266_d_n5, assign23000_e25266_d_n6, assign23000_e25266_d_n7, assign23000_e25266_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) && (var_guard419 == 0.0)) {
        let assign23000_e25262: f64 = (0.666666666666667 * var_atatbot);
        let assign23000_e25264: f64 = (assign23000_e25262 / var_btat);
        (assign23000_e25264, (-((assign23000_e25262 * var_btat_dn5) / (var_btat * var_btat))), (-((assign23000_e25262 * var_btat_dn6) / (var_btat * var_btat))), (-((assign23000_e25262 * var_btat_dn7) / (var_btat * var_btat))), (-((assign23000_e25262 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign23000_e25266;
        var_twoatatoverthreebtat_dn5 = assign23000_e25266_d_n5;
        var_twoatatoverthreebtat_dn6 = assign23000_e25266_d_n6;
        var_twoatatoverthreebtat_dn7 = assign23000_e25266_d_n7;
        var_twoatatoverthreebtat_dn8 = assign23000_e25266_d_n8;

        let (assign23010_e25280, assign23010_e25280_d_n5, assign23010_e25280_d_n6, assign23010_e25280_d_n7, assign23010_e25280_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) && (var_guard419 == 0.0)) {
        let assign23010_e25278: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign23010_e25278, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign23010_e25280;
        var_umaxbeforelimiting_dn5 = assign23010_e25280_d_n5;
        var_umaxbeforelimiting_dn6 = assign23010_e25280_d_n6;
        var_umaxbeforelimiting_dn7 = assign23010_e25280_d_n7;
        var_umaxbeforelimiting_dn8 = assign23010_e25280_d_n8;

        let (assign23020_e25301, assign23020_e25301_d_n5, assign23020_e25301_d_n6, assign23020_e25301_d_n7, assign23020_e25301_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) && (var_guard419 == 0.0)) {
        let assign23020_e25292: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign23020_e25295: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign23020_e25297: f64 = (assign23020_e25295 + 1.0);
        let assign23020_e25298: f64 = (assign23020_e25292 / assign23020_e25297);
        let assign23020_e25299: f64 = (assign23020_e25298).sqrt();
        (assign23020_e25299, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign23020_e25297) - (assign23020_e25292 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign23020_e25297 * assign23020_e25297)) / (2.0 * assign23020_e25299)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign23020_e25297) - (assign23020_e25292 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign23020_e25297 * assign23020_e25297)) / (2.0 * assign23020_e25299)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign23020_e25297) - (assign23020_e25292 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign23020_e25297 * assign23020_e25297)) / (2.0 * assign23020_e25299)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign23020_e25297) - (assign23020_e25292 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign23020_e25297 * assign23020_e25297)) / (2.0 * assign23020_e25299)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign23020_e25301;
        var_umax_dn5 = assign23020_e25301_d_n5;
        var_umax_dn6 = assign23020_e25301_d_n6;
        var_umax_dn7 = assign23020_e25301_d_n7;
        var_umax_dn8 = assign23020_e25301_d_n8;

        let (assign23030_e25314, assign23030_e25314_d_n5, assign23030_e25314_d_n6, assign23030_e25314_d_n7, assign23030_e25314_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) && (var_guard419 == 0.0)) {
        let assign23030_e25312: f64 = (var_umax).sqrt();
        (assign23030_e25312, (var_umax_dn5 / (2.0 * assign23030_e25312)), (var_umax_dn6 / (2.0 * assign23030_e25312)), (var_umax_dn7 / (2.0 * assign23030_e25312)), (var_umax_dn8 / (2.0 * assign23030_e25312)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign23030_e25314;
        var_sqrtumax_dn5 = assign23030_e25314_d_n5;
        var_sqrtumax_dn6 = assign23030_e25314_d_n6;
        var_sqrtumax_dn7 = assign23030_e25314_d_n7;
        var_sqrtumax_dn8 = assign23030_e25314_d_n8;

        let (assign23040_e25328, assign23040_e25328_d_n5, assign23040_e25328_d_n6, assign23040_e25328_d_n7, assign23040_e25328_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) && (var_guard419 == 0.0)) {
        let assign23040_e25326: f64 = (var_umax * var_sqrtumax);
        (assign23040_e25326, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign23040_e25328;
        var_umaxpoweronepointfive_dn5 = assign23040_e25328_d_n5;
        var_umaxpoweronepointfive_dn6 = assign23040_e25328_d_n6;
        var_umaxpoweronepointfive_dn7 = assign23040_e25328_d_n7;
        var_umaxpoweronepointfive_dn8 = assign23040_e25328_d_n8;

        let assign23050_e25330: f64 = (-p.p824);
        let assign23050_e25332: f64 = (assign23050_e25330 * var_one_over_one_minus_pbot);
        let assign23050_e25334: f64 = (-1.0);
        let assign23050_e25335: f64 = if assign23050_e25332 == assign23050_e25334 { 1.0 } else { 0.0 };
        var_guard420 = assign23050_e25335;

        let (assign23060_e25355, assign23060_e25355_d_n5, assign23060_e25355_d_n6, assign23060_e25355_d_n7, assign23060_e25355_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) && (var_guard419 == 0.0)) && (var_guard420 != 0.0)) {
        let assign23060_e25351: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign23060_e25352: f64 = (1.0 + assign23060_e25351);
        let assign23060_e25353: f64 = (1.0 / assign23060_e25352);
        (assign23060_e25353, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign23060_e25352 * assign23060_e25352))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign23060_e25352 * assign23060_e25352))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign23060_e25352 * assign23060_e25352))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign23060_e25352 * assign23060_e25352))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign23060_e25355;
        var_wgamma_dn5 = assign23060_e25355_d_n5;
        var_wgamma_dn6 = assign23060_e25355_d_n6;
        var_wgamma_dn7 = assign23060_e25355_d_n7;
        var_wgamma_dn8 = assign23060_e25355_d_n8;

        let (assign23070_e25379, assign23070_e25379_d_n5, assign23070_e25379_d_n6, assign23070_e25379_d_n7, assign23070_e25379_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) && (var_guard419 == 0.0)) && (var_guard420 == 0.0)) {
        let assign23070_e25371: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign23070_e25372: f64 = (1.0 + assign23070_e25371);
        let assign23070_e25374: f64 = (-p.p824);
        let assign23070_e25376: f64 = (assign23070_e25374 * var_one_over_one_minus_pbot);
        let assign23070_e25377: f64 = (assign23070_e25372).powf(assign23070_e25376);
        (assign23070_e25377, if 0.0 == 0.0 && ((assign23070_e25376) as f64).is_finite() && ((assign23070_e25376) as f64).fract() == 0.0 { if assign23070_e25376 == 0.0 { 0.0 } else { (assign23070_e25376 * ((assign23070_e25372).powf(assign23070_e25376 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign23070_e25377 * (assign23070_e25376 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign23070_e25372))) }, if 0.0 == 0.0 && ((assign23070_e25376) as f64).is_finite() && ((assign23070_e25376) as f64).fract() == 0.0 { if assign23070_e25376 == 0.0 { 0.0 } else { (assign23070_e25376 * ((assign23070_e25372).powf(assign23070_e25376 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign23070_e25377 * (assign23070_e25376 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign23070_e25372))) }, if 0.0 == 0.0 && ((assign23070_e25376) as f64).is_finite() && ((assign23070_e25376) as f64).fract() == 0.0 { if assign23070_e25376 == 0.0 { 0.0 } else { (assign23070_e25376 * ((assign23070_e25372).powf(assign23070_e25376 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign23070_e25377 * (assign23070_e25376 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign23070_e25372))) }, if 0.0 == 0.0 && ((assign23070_e25376) as f64).is_finite() && ((assign23070_e25376) as f64).fract() == 0.0 { if assign23070_e25376 == 0.0 { 0.0 } else { (assign23070_e25376 * ((assign23070_e25372).powf(assign23070_e25376 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign23070_e25377 * (assign23070_e25376 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign23070_e25372))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign23070_e25379;
        var_wgamma_dn5 = assign23070_e25379_d_n5;
        var_wgamma_dn6 = assign23070_e25379_d_n6;
        var_wgamma_dn7 = assign23070_e25379_d_n7;
        var_wgamma_dn8 = assign23070_e25379_d_n8;

        let (assign23080_e25397, assign23080_e25397_d_n5, assign23080_e25397_d_n6, assign23080_e25397_d_n7, assign23080_e25397_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) && (var_guard419 == 0.0)) {
        let assign23080_e25391: f64 = (var_wsrh * var_wgamma);
        let assign23080_e25394: f64 = (var_wsrh + var_wgamma);
        let assign23080_e25395: f64 = (assign23080_e25391 / assign23080_e25394);
        (assign23080_e25395, ((((var_wsrh * var_wgamma_dn5) * assign23080_e25394) - (assign23080_e25391 * var_wgamma_dn5)) / (assign23080_e25394 * assign23080_e25394)), ((((var_wsrh * var_wgamma_dn6) * assign23080_e25394) - (assign23080_e25391 * var_wgamma_dn6)) / (assign23080_e25394 * assign23080_e25394)), ((((var_wsrh * var_wgamma_dn7) * assign23080_e25394) - (assign23080_e25391 * var_wgamma_dn7)) / (assign23080_e25394 * assign23080_e25394)), ((((var_wsrh * var_wgamma_dn8) * assign23080_e25394) - (assign23080_e25391 * var_wgamma_dn8)) / (assign23080_e25394 * assign23080_e25394)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign23080_e25397;
        var_wtat_dn5 = assign23080_e25397_d_n5;
        var_wtat_dn6 = assign23080_e25397_d_n6;
        var_wtat_dn7 = assign23080_e25397_d_n7;
        var_wtat_dn8 = assign23080_e25397_d_n8;

        let (assign23090_e25414, assign23090_e25414_d_n5, assign23090_e25414_d_n6, assign23090_e25414_d_n7, assign23090_e25414_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) && (var_guard419 == 0.0)) {
        let assign23090_e25410: f64 = (var_btat / var_sqrtumax);
        let assign23090_e25411: f64 = (0.375 * assign23090_e25410);
        let assign23090_e25412: f64 = (assign23090_e25411).sqrt();
        (assign23090_e25412, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign23090_e25412)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign23090_e25412)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign23090_e25412)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign23090_e25412)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign23090_e25414;
        var_ktat_dn5 = assign23090_e25414_d_n5;
        var_ktat_dn6 = assign23090_e25414_d_n6;
        var_ktat_dn7 = assign23090_e25414_d_n7;
        var_ktat_dn8 = assign23090_e25414_d_n8;

        let (assign23100_e25432, assign23100_e25432_d_n5, assign23100_e25432_d_n6, assign23100_e25432_d_n7, assign23100_e25432_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) && (var_guard419 == 0.0)) {
        let assign23100_e25427: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign23100_e25428: f64 = (2.0 * assign23100_e25427);
        let assign23100_e25430: f64 = (assign23100_e25428 - var_umax);
        (assign23100_e25430, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign23100_e25432;
        var_ltat_dn5 = assign23100_e25432_d_n5;
        var_ltat_dn6 = assign23100_e25432_d_n6;
        var_ltat_dn7 = assign23100_e25432_d_n7;
        var_ltat_dn8 = assign23100_e25432_d_n8;

        let (assign23110_e25458, assign23110_e25458_d_n5, assign23110_e25458_d_n6, assign23110_e25458_d_n7, assign23110_e25458_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) && (var_guard419 == 0.0)) {
        let assign23110_e25444: f64 = (var_atatbot * var_twoatatoverthreebtat);
        let assign23110_e25446: f64 = (assign23110_e25444 * var_sqrtumax);
        let assign23110_e25449: f64 = (var_atatbot * var_umax);
        let assign23110_e25450: f64 = (assign23110_e25446 - assign23110_e25449);
        let assign23110_e25454: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign23110_e25455: f64 = (0.5 * assign23110_e25454);
        let assign23110_e25456: f64 = (assign23110_e25450 + assign23110_e25455);
        (assign23110_e25456, (((((var_atatbot * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign23110_e25444 * var_sqrtumax_dn5)) - (var_atatbot * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatbot * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign23110_e25444 * var_sqrtumax_dn6)) - (var_atatbot * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatbot * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign23110_e25444 * var_sqrtumax_dn7)) - (var_atatbot * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatbot * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign23110_e25444 * var_sqrtumax_dn8)) - (var_atatbot * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign23110_e25458;
        var_mtat_dn5 = assign23110_e25458_d_n5;
        var_mtat_dn6 = assign23110_e25458_d_n6;
        var_mtat_dn7 = assign23110_e25458_d_n7;
        var_mtat_dn8 = assign23110_e25458_d_n8;

        let (assign23120_e25474, assign23120_e25474_d_n5, assign23120_e25474_d_n6, assign23120_e25474_d_n7, assign23120_e25474_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) && (var_guard419 == 0.0)) {
        let assign23120_e25470: f64 = (var_ltat - 1.0);
        let assign23120_e25472: f64 = (assign23120_e25470 * var_ktat);
        (assign23120_e25472, ((var_ltat_dn5 * var_ktat) + (assign23120_e25470 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign23120_e25470 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign23120_e25470 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign23120_e25470 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign23120_e25474;
        var_xerfc_dn5 = assign23120_e25474_d_n5;
        var_xerfc_dn6 = assign23120_e25474_d_n6;
        var_xerfc_dn7 = assign23120_e25474_d_n7;
        var_xerfc_dn8 = assign23120_e25474_d_n8;

        *var_asrh_slot = var_asrh;
        *var_asrh_dn5_slot = var_asrh_dn5;
        *var_asrh_dn6_slot = var_asrh_dn6;
        *var_asrh_dn7_slot = var_asrh_dn7;
        *var_asrh_dn8_slot = var_asrh_dn8;
        *var_btat_slot = var_btat;
        *var_btat_dn5_slot = var_btat_dn5;
        *var_btat_dn6_slot = var_btat_dn6;
        *var_btat_dn7_slot = var_btat_dn7;
        *var_btat_dn8_slot = var_btat_dn8;
        *var_dwsrh_slot = var_dwsrh;
        *var_guard415_slot = var_guard415;
        *var_guard416_slot = var_guard416;
        *var_guard417_slot = var_guard417;
        *var_guard418_slot = var_guard418;
        *var_guard419_slot = var_guard419;
        *var_guard420_slot = var_guard420;
        *var_id__blk213_slot = var_id__blk213;
        *var_ijunbot_slot = var_ijunbot;
        *var_ijunbot_dn5_slot = var_ijunbot_dn5;
        *var_ijunbot_dn6_slot = var_ijunbot_dn6;
        *var_ijunbot_dn7_slot = var_ijunbot_dn7;
        *var_ijunbot_dn8_slot = var_ijunbot_dn8;
        *var_isrh_slot = var_isrh;
        *var_isrh_dn5_slot = var_isrh_dn5;
        *var_isrh_dn6_slot = var_isrh_dn6;
        *var_isrh_dn7_slot = var_isrh_dn7;
        *var_isrh_dn8_slot = var_isrh_dn8;
        *var_itat_slot = var_itat;
        *var_itat_dn5_slot = var_itat_dn5;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_ktat_slot = var_ktat;
        *var_ktat_dn5_slot = var_ktat_dn5;
        *var_ktat_dn6_slot = var_ktat_dn6;
        *var_ktat_dn7_slot = var_ktat_dn7;
        *var_ktat_dn8_slot = var_ktat_dn8;
        *var_ltat_slot = var_ltat;
        *var_ltat_dn5_slot = var_ltat_dn5;
        *var_ltat_dn6_slot = var_ltat_dn6;
        *var_ltat_dn7_slot = var_ltat_dn7;
        *var_ltat_dn8_slot = var_ltat_dn8;
        *var_mtat_slot = var_mtat;
        *var_mtat_dn5_slot = var_mtat_dn5;
        *var_mtat_dn6_slot = var_mtat_dn6;
        *var_mtat_dn7_slot = var_mtat_dn7;
        *var_mtat_dn8_slot = var_mtat_dn8;
        *var_sqrtumax_slot = var_sqrtumax;
        *var_sqrtumax_dn5_slot = var_sqrtumax_dn5;
        *var_sqrtumax_dn6_slot = var_sqrtumax_dn6;
        *var_sqrtumax_dn7_slot = var_sqrtumax_dn7;
        *var_sqrtumax_dn8_slot = var_sqrtumax_dn8;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn5_slot = var_tmp_dn5;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_two_psistar_slot = var_two_psistar;
        *var_twoatatoverthreebtat_slot = var_twoatatoverthreebtat;
        *var_twoatatoverthreebtat_dn5_slot = var_twoatatoverthreebtat_dn5;
        *var_twoatatoverthreebtat_dn6_slot = var_twoatatoverthreebtat_dn6;
        *var_twoatatoverthreebtat_dn7_slot = var_twoatatoverthreebtat_dn7;
        *var_twoatatoverthreebtat_dn8_slot = var_twoatatoverthreebtat_dn8;
        *var_umax_slot = var_umax;
        *var_umax_dn5_slot = var_umax_dn5;
        *var_umax_dn6_slot = var_umax_dn6;
        *var_umax_dn7_slot = var_umax_dn7;
        *var_umax_dn8_slot = var_umax_dn8;
        *var_umaxbeforelimiting_slot = var_umaxbeforelimiting;
        *var_umaxbeforelimiting_dn5_slot = var_umaxbeforelimiting_dn5;
        *var_umaxbeforelimiting_dn6_slot = var_umaxbeforelimiting_dn6;
        *var_umaxbeforelimiting_dn7_slot = var_umaxbeforelimiting_dn7;
        *var_umaxbeforelimiting_dn8_slot = var_umaxbeforelimiting_dn8;
        *var_umaxpoweronepointfive_slot = var_umaxpoweronepointfive;
        *var_umaxpoweronepointfive_dn5_slot = var_umaxpoweronepointfive_dn5;
        *var_umaxpoweronepointfive_dn6_slot = var_umaxpoweronepointfive_dn6;
        *var_umaxpoweronepointfive_dn7_slot = var_umaxpoweronepointfive_dn7;
        *var_umaxpoweronepointfive_dn8_slot = var_umaxpoweronepointfive_dn8;
        *var_vav_slot = var_vav;
        *var_vbbt_slot = var_vbbt;
        *var_vbi_minus_vjsrh_slot = var_vbi_minus_vjsrh;
        *var_vjlim_slot = var_vjlim;
        *var_vjsrh_slot = var_vjsrh;
        *var_wdep_slot = var_wdep;
        *var_wdep_dn5_slot = var_wdep_dn5;
        *var_wdep_dn6_slot = var_wdep_dn6;
        *var_wdep_dn7_slot = var_wdep_dn7;
        *var_wdep_dn8_slot = var_wdep_dn8;
        *var_wgamma_slot = var_wgamma;
        *var_wgamma_dn5_slot = var_wgamma_dn5;
        *var_wgamma_dn6_slot = var_wgamma_dn6;
        *var_wgamma_dn7_slot = var_wgamma_dn7;
        *var_wgamma_dn8_slot = var_wgamma_dn8;
        *var_wsrh_slot = var_wsrh;
        *var_wsrhstep_slot = var_wsrhstep;
        *var_wtat_slot = var_wtat;
        *var_wtat_dn5_slot = var_wtat_dn5;
        *var_wtat_dn6_slot = var_wtat_dn6;
        *var_wtat_dn7_slot = var_wtat_dn7;
        *var_wtat_dn8_slot = var_wtat_dn8;
        *var_xerfc_slot = var_xerfc;
        *var_xerfc_dn5_slot = var_xerfc_dn5;
        *var_xerfc_dn6_slot = var_xerfc_dn6;
        *var_xerfc_dn7_slot = var_xerfc_dn7;
        *var_xerfc_dn8_slot = var_xerfc_dn8;
    }

    pub(super) fn stamp_transient_block_42(
        p: &Parameters,
        var_alphaav: f64,
        var_asrh: f64,
        var_asrh_dn5: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_atatbot: f64,
        var_berfc: f64,
        var_cerfc: f64,
        var_fbbtbot: f64,
        var_fstopbot: f64,
        var_guard176: f64,
        var_guard193: f64,
        var_guard415: f64,
        var_guard419: f64,
        var_ktat: f64,
        var_ktat_dn5: f64,
        var_ktat_dn6: f64,
        var_ktat_dn7: f64,
        var_ktat_dn8: f64,
        var_mtat: f64,
        var_mtat_dn5: f64,
        var_mtat_dn6: f64,
        var_mtat_dn7: f64,
        var_mtat_dn8: f64,
        var_one_over_one_minus_pbot: f64,
        var_perfc: f64,
        var_slopebot: f64,
        var_v4: f64,
        var_vav: f64,
        var_vbbt: f64,
        var_vbirbotinv: f64,
        var_vbrinvbot: f64,
        var_wdepnulrinvbot: f64,
        var_wtat: f64,
        var_wtat_dn5: f64,
        var_wtat_dn6: f64,
        var_wtat_dn7: f64,
        var_wtat_dn8: f64,
        var_xerfc: f64,
        var_xerfc_dn5: f64,
        var_xerfc_dn6: f64,
        var_xerfc_dn7: f64,
        var_xerfc_dn8: f64,
        var_erfcpos_slot: &mut f64,
        var_erfcpos_dn5_slot: &mut f64,
        var_erfcpos_dn6_slot: &mut f64,
        var_erfcpos_dn7_slot: &mut f64,
        var_erfcpos_dn8_slot: &mut f64,
        var_erfctimesexpmtat_slot: &mut f64,
        var_erfctimesexpmtat_dn5_slot: &mut f64,
        var_erfctimesexpmtat_dn6_slot: &mut f64,
        var_erfctimesexpmtat_dn7_slot: &mut f64,
        var_erfctimesexpmtat_dn8_slot: &mut f64,
        var_fbreakdown_slot: &mut f64,
        var_fbreakdown_dn5_slot: &mut f64,
        var_fbreakdown_dn6_slot: &mut f64,
        var_fbreakdown_dn7_slot: &mut f64,
        var_fbreakdown_dn8_slot: &mut f64,
        var_fmaxr_slot: &mut f64,
        var_fmaxr_dn5_slot: &mut f64,
        var_fmaxr_dn6_slot: &mut f64,
        var_fmaxr_dn7_slot: &mut f64,
        var_fmaxr_dn8_slot: &mut f64,
        var_gammamax_slot: &mut f64,
        var_gammamax_dn5_slot: &mut f64,
        var_gammamax_dn6_slot: &mut f64,
        var_gammamax_dn7_slot: &mut f64,
        var_gammamax_dn8_slot: &mut f64,
        var_guard421_slot: &mut f64,
        var_guard422_slot: &mut f64,
        var_guard423_slot: &mut f64,
        var_guard424_slot: &mut f64,
        var_guard425_slot: &mut f64,
        var_guard426_slot: &mut f64,
        var_guard427_slot: &mut f64,
        var_guard428_slot: &mut f64,
        var_guard429_slot: &mut f64,
        var_guard430_slot: &mut f64,
        var_guard431_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn5_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn5_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_terfc_slot: &mut f64,
        var_terfc_dn5_slot: &mut f64,
        var_terfc_dn6_slot: &mut f64,
        var_terfc_dn7_slot: &mut f64,
        var_terfc_dn8_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn5_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_ysq_slot: &mut f64,
        var_ysq_dn5_slot: &mut f64,
        var_ysq_dn6_slot: &mut f64,
        var_ysq_dn7_slot: &mut f64,
        var_ysq_dn8_slot: &mut f64,
    ) {
        let mut var_erfcpos: f64 = *var_erfcpos_slot;
        let mut var_erfcpos_dn5: f64 = *var_erfcpos_dn5_slot;
        let mut var_erfcpos_dn6: f64 = *var_erfcpos_dn6_slot;
        let mut var_erfcpos_dn7: f64 = *var_erfcpos_dn7_slot;
        let mut var_erfcpos_dn8: f64 = *var_erfcpos_dn8_slot;
        let mut var_erfctimesexpmtat: f64 = *var_erfctimesexpmtat_slot;
        let mut var_erfctimesexpmtat_dn5: f64 = *var_erfctimesexpmtat_dn5_slot;
        let mut var_erfctimesexpmtat_dn6: f64 = *var_erfctimesexpmtat_dn6_slot;
        let mut var_erfctimesexpmtat_dn7: f64 = *var_erfctimesexpmtat_dn7_slot;
        let mut var_erfctimesexpmtat_dn8: f64 = *var_erfctimesexpmtat_dn8_slot;
        let mut var_fbreakdown: f64 = *var_fbreakdown_slot;
        let mut var_fbreakdown_dn5: f64 = *var_fbreakdown_dn5_slot;
        let mut var_fbreakdown_dn6: f64 = *var_fbreakdown_dn6_slot;
        let mut var_fbreakdown_dn7: f64 = *var_fbreakdown_dn7_slot;
        let mut var_fbreakdown_dn8: f64 = *var_fbreakdown_dn8_slot;
        let mut var_fmaxr: f64 = *var_fmaxr_slot;
        let mut var_fmaxr_dn5: f64 = *var_fmaxr_dn5_slot;
        let mut var_fmaxr_dn6: f64 = *var_fmaxr_dn6_slot;
        let mut var_fmaxr_dn7: f64 = *var_fmaxr_dn7_slot;
        let mut var_fmaxr_dn8: f64 = *var_fmaxr_dn8_slot;
        let mut var_gammamax: f64 = *var_gammamax_slot;
        let mut var_gammamax_dn5: f64 = *var_gammamax_dn5_slot;
        let mut var_gammamax_dn6: f64 = *var_gammamax_dn6_slot;
        let mut var_gammamax_dn7: f64 = *var_gammamax_dn7_slot;
        let mut var_gammamax_dn8: f64 = *var_gammamax_dn8_slot;
        let mut var_guard421: f64 = *var_guard421_slot;
        let mut var_guard422: f64 = *var_guard422_slot;
        let mut var_guard423: f64 = *var_guard423_slot;
        let mut var_guard424: f64 = *var_guard424_slot;
        let mut var_guard425: f64 = *var_guard425_slot;
        let mut var_guard426: f64 = *var_guard426_slot;
        let mut var_guard427: f64 = *var_guard427_slot;
        let mut var_guard428: f64 = *var_guard428_slot;
        let mut var_guard429: f64 = *var_guard429_slot;
        let mut var_guard430: f64 = *var_guard430_slot;
        let mut var_guard431: f64 = *var_guard431_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn5: f64 = *var_ibbt_dn5_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn5: f64 = *var_itat_dn5_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_terfc: f64 = *var_terfc_slot;
        let mut var_terfc_dn5: f64 = *var_terfc_dn5_slot;
        let mut var_terfc_dn6: f64 = *var_terfc_dn6_slot;
        let mut var_terfc_dn7: f64 = *var_terfc_dn7_slot;
        let mut var_terfc_dn8: f64 = *var_terfc_dn8_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn5: f64 = *var_tmp_dn5_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_ysq: f64 = *var_ysq_slot;
        let mut var_ysq_dn5: f64 = *var_ysq_dn5_slot;
        let mut var_ysq_dn6: f64 = *var_ysq_dn6_slot;
        let mut var_ysq_dn7: f64 = *var_ysq_dn7_slot;
        let mut var_ysq_dn8: f64 = *var_ysq_dn8_slot;

        let (assign23130_e25488, assign23130_e25488_d_n5, assign23130_e25488_d_n6, assign23130_e25488_d_n7, assign23130_e25488_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) && (var_guard419 == 0.0)) {
        let assign23130_e25486: f64 = (var_xerfc * var_xerfc);
        (assign23130_e25486, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign23130_e25488;
        var_ysq_dn5 = assign23130_e25488_d_n5;
        var_ysq_dn6 = assign23130_e25488_d_n6;
        var_ysq_dn7 = assign23130_e25488_d_n7;
        var_ysq_dn8 = assign23130_e25488_d_n8;

        let assign23140_e25491: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard421 = assign23140_e25491;

        let (assign23150_e25511, assign23150_e25511_d_n5, assign23150_e25511_d_n6, assign23150_e25511_d_n7, assign23150_e25511_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) && (var_guard419 == 0.0)) && (var_guard421 != 0.0)) {
        let assign23150_e25507: f64 = (var_perfc * var_xerfc);
        let assign23150_e25508: f64 = (1.0 + assign23150_e25507);
        let assign23150_e25509: f64 = (1.0 / assign23150_e25508);
        (assign23150_e25509, (-((var_perfc * var_xerfc_dn5) / (assign23150_e25508 * assign23150_e25508))), (-((var_perfc * var_xerfc_dn6) / (assign23150_e25508 * assign23150_e25508))), (-((var_perfc * var_xerfc_dn7) / (assign23150_e25508 * assign23150_e25508))), (-((var_perfc * var_xerfc_dn8) / (assign23150_e25508 * assign23150_e25508))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign23150_e25511;
        var_terfc_dn5 = assign23150_e25511_d_n5;
        var_terfc_dn6 = assign23150_e25511_d_n6;
        var_terfc_dn7 = assign23150_e25511_d_n7;
        var_terfc_dn8 = assign23150_e25511_d_n8;

        let (assign23160_e25532, assign23160_e25532_d_n5, assign23160_e25532_d_n6, assign23160_e25532_d_n7, assign23160_e25532_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) && (var_guard419 == 0.0)) && (var_guard421 == 0.0)) {
        let assign23160_e25528: f64 = (var_perfc * var_xerfc);
        let assign23160_e25529: f64 = (1.0 - assign23160_e25528);
        let assign23160_e25530: f64 = (1.0 / assign23160_e25529);
        (assign23160_e25530, (-((-(var_perfc * var_xerfc_dn5)) / (assign23160_e25529 * assign23160_e25529))), (-((-(var_perfc * var_xerfc_dn6)) / (assign23160_e25529 * assign23160_e25529))), (-((-(var_perfc * var_xerfc_dn7)) / (assign23160_e25529 * assign23160_e25529))), (-((-(var_perfc * var_xerfc_dn8)) / (assign23160_e25529 * assign23160_e25529))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign23160_e25532;
        var_terfc_dn5 = assign23160_e25532_d_n5;
        var_terfc_dn6 = assign23160_e25532_d_n6;
        var_terfc_dn7 = assign23160_e25532_d_n7;
        var_terfc_dn8 = assign23160_e25532_d_n8;

        let assign23170_e25534: f64 = (-var_ysq);
        let assign23170_e25536: f64 = (assign23170_e25534 + var_mtat);
        let assign23170_e25538: f64 = (-230.25850929940458);
        let assign23170_e25539: f64 = if assign23170_e25536 > assign23170_e25538 { 1.0 } else { 0.0 };
        var_guard422 = assign23170_e25539;

        let (assign23180_e25557, assign23180_e25557_d_n5, assign23180_e25557_d_n6, assign23180_e25557_d_n7, assign23180_e25557_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) && (var_guard419 == 0.0)) && (var_guard422 != 0.0)) {
        let assign23180_e25552: f64 = (-var_ysq);
        let assign23180_e25554: f64 = (assign23180_e25552 + var_mtat);
        let assign23180_e25555: f64 = (assign23180_e25554).exp();
        (assign23180_e25555, (assign23180_e25555 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign23180_e25555 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign23180_e25555 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign23180_e25555 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign23180_e25557;
        var_tmp_dn5 = assign23180_e25557_d_n5;
        var_tmp_dn6 = assign23180_e25557_d_n6;
        var_tmp_dn7 = assign23180_e25557_d_n7;
        var_tmp_dn8 = assign23180_e25557_d_n8;

        let (assign23190_e25606, assign23190_e25606_d_n5, assign23190_e25606_d_n6, assign23190_e25606_d_n7, assign23190_e25606_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) && (var_guard419 == 0.0)) && (var_guard422 == 0.0)) {
        let assign23190_e25573: f64 = (-230.25850929940458);
        let assign23190_e25575: f64 = (-var_ysq);
        let assign23190_e25577: f64 = (assign23190_e25575 + var_mtat);
        let assign23190_e25578: f64 = (assign23190_e25573 - assign23190_e25577);
        let assign23190_e25582: f64 = (-230.25850929940458);
        let assign23190_e25584: f64 = (-var_ysq);
        let assign23190_e25586: f64 = (assign23190_e25584 + var_mtat);
        let assign23190_e25587: f64 = (assign23190_e25582 - assign23190_e25586);
        let assign23190_e25590: f64 = (-230.25850929940458);
        let assign23190_e25592: f64 = (-var_ysq);
        let assign23190_e25594: f64 = (assign23190_e25592 + var_mtat);
        let assign23190_e25595: f64 = (assign23190_e25590 - assign23190_e25594);
        let assign23190_e25597: f64 = (assign23190_e25595 * 0.3333333333333333);
        let assign23190_e25598: f64 = (1.0 + assign23190_e25597);
        let assign23190_e25599: f64 = (assign23190_e25587 * assign23190_e25598);
        let assign23190_e25600: f64 = (0.5 * assign23190_e25599);
        let assign23190_e25601: f64 = (1.0 + assign23190_e25600);
        let assign23190_e25602: f64 = (assign23190_e25578 * assign23190_e25601);
        let assign23190_e25603: f64 = (1.0 + assign23190_e25602);
        let assign23190_e25604: f64 = (1e-100 / assign23190_e25603);
        (assign23190_e25604, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign23190_e25601) + (assign23190_e25578 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign23190_e25598) + (assign23190_e25587 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign23190_e25603 * assign23190_e25603))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign23190_e25601) + (assign23190_e25578 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign23190_e25598) + (assign23190_e25587 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign23190_e25603 * assign23190_e25603))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign23190_e25601) + (assign23190_e25578 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign23190_e25598) + (assign23190_e25587 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign23190_e25603 * assign23190_e25603))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign23190_e25601) + (assign23190_e25578 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign23190_e25598) + (assign23190_e25587 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign23190_e25603 * assign23190_e25603))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign23190_e25606;
        var_tmp_dn5 = assign23190_e25606_d_n5;
        var_tmp_dn6 = assign23190_e25606_d_n6;
        var_tmp_dn7 = assign23190_e25606_d_n7;
        var_tmp_dn8 = assign23190_e25606_d_n8;

        let (assign23200_e25636, assign23200_e25636_d_n5, assign23200_e25636_d_n6, assign23200_e25636_d_n7, assign23200_e25636_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) && (var_guard419 == 0.0)) {
        let assign23200_e25618: f64 = (0.29214664 * var_terfc);
        let assign23200_e25622: f64 = (var_terfc * var_terfc);
        let assign23200_e25623: f64 = (var_berfc * assign23200_e25622);
        let assign23200_e25624: f64 = (assign23200_e25618 + assign23200_e25623);
        let assign23200_e25628: f64 = (var_terfc * var_terfc);
        let assign23200_e25630: f64 = (assign23200_e25628 * var_terfc);
        let assign23200_e25631: f64 = (var_cerfc * assign23200_e25630);
        let assign23200_e25632: f64 = (assign23200_e25624 + assign23200_e25631);
        let assign23200_e25634: f64 = (assign23200_e25632 * var_tmp);
        (assign23200_e25634, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign23200_e25628 * var_terfc_dn5)))) * var_tmp) + (assign23200_e25632 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign23200_e25628 * var_terfc_dn6)))) * var_tmp) + (assign23200_e25632 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign23200_e25628 * var_terfc_dn7)))) * var_tmp) + (assign23200_e25632 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign23200_e25628 * var_terfc_dn8)))) * var_tmp) + (assign23200_e25632 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign23200_e25636;
        var_erfcpos_dn5 = assign23200_e25636_d_n5;
        var_erfcpos_dn6 = assign23200_e25636_d_n6;
        var_erfcpos_dn7 = assign23200_e25636_d_n7;
        var_erfcpos_dn8 = assign23200_e25636_d_n8;

        let assign23210_e25639: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard423 = assign23210_e25639;

        let (assign23220_e25653, assign23220_e25653_d_n5, assign23220_e25653_d_n6, assign23220_e25653_d_n7, assign23220_e25653_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) && (var_guard419 == 0.0)) && (var_guard423 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign23220_e25653;
        var_erfctimesexpmtat_dn5 = assign23220_e25653_d_n5;
        var_erfctimesexpmtat_dn6 = assign23220_e25653_d_n6;
        var_erfctimesexpmtat_dn7 = assign23220_e25653_d_n7;
        var_erfctimesexpmtat_dn8 = assign23220_e25653_d_n8;

        let assign23230_e25656: f64 = (-230.25850929940458);
        let assign23230_e25657: f64 = if var_mtat > assign23230_e25656 { 1.0 } else { 0.0 };
        var_guard424 = assign23230_e25657;

        let (assign23240_e25675, assign23240_e25675_d_n5, assign23240_e25675_d_n6, assign23240_e25675_d_n7, assign23240_e25675_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) && (var_guard419 == 0.0)) && (var_guard423 == 0.0)) && (var_guard424 != 0.0)) {
        let assign23240_e25673: f64 = (var_mtat).exp();
        (assign23240_e25673, (assign23240_e25673 * var_mtat_dn5), (assign23240_e25673 * var_mtat_dn6), (assign23240_e25673 * var_mtat_dn7), (assign23240_e25673 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign23240_e25675;
        var_tmp_dn5 = assign23240_e25675_d_n5;
        var_tmp_dn6 = assign23240_e25675_d_n6;
        var_tmp_dn7 = assign23240_e25675_d_n7;
        var_tmp_dn8 = assign23240_e25675_d_n8;

        let (assign23250_e25718, assign23250_e25718_d_n5, assign23250_e25718_d_n6, assign23250_e25718_d_n7, assign23250_e25718_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) && (var_guard419 == 0.0)) && (var_guard423 == 0.0)) && (var_guard424 == 0.0)) {
        let assign23250_e25694: f64 = (-230.25850929940458);
        let assign23250_e25696: f64 = (assign23250_e25694 - var_mtat);
        let assign23250_e25700: f64 = (-230.25850929940458);
        let assign23250_e25702: f64 = (assign23250_e25700 - var_mtat);
        let assign23250_e25705: f64 = (-230.25850929940458);
        let assign23250_e25707: f64 = (assign23250_e25705 - var_mtat);
        let assign23250_e25709: f64 = (assign23250_e25707 * 0.3333333333333333);
        let assign23250_e25710: f64 = (1.0 + assign23250_e25709);
        let assign23250_e25711: f64 = (assign23250_e25702 * assign23250_e25710);
        let assign23250_e25712: f64 = (0.5 * assign23250_e25711);
        let assign23250_e25713: f64 = (1.0 + assign23250_e25712);
        let assign23250_e25714: f64 = (assign23250_e25696 * assign23250_e25713);
        let assign23250_e25715: f64 = (1.0 + assign23250_e25714);
        let assign23250_e25716: f64 = (1e-100 / assign23250_e25715);
        (assign23250_e25716, (-((1e-100 * (((-var_mtat_dn5) * assign23250_e25713) + (assign23250_e25696 * (0.5 * (((-var_mtat_dn5) * assign23250_e25710) + (assign23250_e25702 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign23250_e25715 * assign23250_e25715))), (-((1e-100 * (((-var_mtat_dn6) * assign23250_e25713) + (assign23250_e25696 * (0.5 * (((-var_mtat_dn6) * assign23250_e25710) + (assign23250_e25702 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign23250_e25715 * assign23250_e25715))), (-((1e-100 * (((-var_mtat_dn7) * assign23250_e25713) + (assign23250_e25696 * (0.5 * (((-var_mtat_dn7) * assign23250_e25710) + (assign23250_e25702 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign23250_e25715 * assign23250_e25715))), (-((1e-100 * (((-var_mtat_dn8) * assign23250_e25713) + (assign23250_e25696 * (0.5 * (((-var_mtat_dn8) * assign23250_e25710) + (assign23250_e25702 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign23250_e25715 * assign23250_e25715))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign23250_e25718;
        var_tmp_dn5 = assign23250_e25718_d_n5;
        var_tmp_dn6 = assign23250_e25718_d_n6;
        var_tmp_dn7 = assign23250_e25718_d_n7;
        var_tmp_dn8 = assign23250_e25718_d_n8;

        let (assign23260_e25737, assign23260_e25737_d_n5, assign23260_e25737_d_n6, assign23260_e25737_d_n7, assign23260_e25737_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) && (var_guard419 == 0.0)) && (var_guard423 == 0.0)) {
        let assign23260_e25733: f64 = (2.0 * var_tmp);
        let assign23260_e25735: f64 = (assign23260_e25733 - var_erfcpos);
        (assign23260_e25735, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign23260_e25737;
        var_erfctimesexpmtat_dn5 = assign23260_e25737_d_n5;
        var_erfctimesexpmtat_dn6 = assign23260_e25737_d_n6;
        var_erfctimesexpmtat_dn7 = assign23260_e25737_d_n7;
        var_erfctimesexpmtat_dn8 = assign23260_e25737_d_n8;

        let (assign23270_e25757, assign23270_e25757_d_n5, assign23270_e25757_d_n6, assign23270_e25757_d_n7, assign23270_e25757_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) && (var_guard419 == 0.0)) {
        let assign23270_e25749: f64 = (1.772453850905516 * 0.5);
        let assign23270_e25752: f64 = (var_atatbot * var_erfctimesexpmtat);
        let assign23270_e25754: f64 = (assign23270_e25752 / var_ktat);
        let assign23270_e25755: f64 = (assign23270_e25749 * assign23270_e25754);
        (assign23270_e25755, (assign23270_e25749 * ((((var_atatbot * var_erfctimesexpmtat_dn5) * var_ktat) - (assign23270_e25752 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign23270_e25749 * ((((var_atatbot * var_erfctimesexpmtat_dn6) * var_ktat) - (assign23270_e25752 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign23270_e25749 * ((((var_atatbot * var_erfctimesexpmtat_dn7) * var_ktat) - (assign23270_e25752 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign23270_e25749 * ((((var_atatbot * var_erfctimesexpmtat_dn8) * var_ktat) - (assign23270_e25752 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign23270_e25757;
        var_gammamax_dn5 = assign23270_e25757_d_n5;
        var_gammamax_dn6 = assign23270_e25757_d_n6;
        var_gammamax_dn7 = assign23270_e25757_d_n7;
        var_gammamax_dn8 = assign23270_e25757_d_n8;

        let (assign23280_e25775, assign23280_e25775_d_n5, assign23280_e25775_d_n6, assign23280_e25775_d_n7, assign23280_e25775_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) && (var_guard419 == 0.0)) {
        let assign23280_e25770: f64 = (var_asrh * var_gammamax);
        let assign23280_e25772: f64 = (assign23280_e25770 * var_wtat);
        let assign23280_e25773: f64 = (p.p838 * assign23280_e25772);
        (assign23280_e25773, (p.p838 * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign23280_e25770 * var_wtat_dn5))), (p.p838 * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign23280_e25770 * var_wtat_dn6))), (p.p838 * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign23280_e25770 * var_wtat_dn7))), (p.p838 * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign23280_e25770 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign23280_e25775;
        var_itat_dn5 = assign23280_e25775_d_n5;
        var_itat_dn6 = assign23280_e25775_d_n6;
        var_itat_dn7 = assign23280_e25775_d_n7;
        var_itat_dn8 = assign23280_e25775_d_n8;

        let assign23290_e25778: f64 = if p.p844 == 0.0 { 1.0 } else { 0.0 };
        var_guard425 = assign23290_e25778;

        let (assign23300_e25789, assign23300_e25789_d_n5, assign23300_e25789_d_n6, assign23300_e25789_d_n7, assign23300_e25789_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) && (var_guard425 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign23300_e25789;
        var_ibbt_dn5 = assign23300_e25789_d_n5;
        var_ibbt_dn6 = assign23300_e25789_d_n6;
        var_ibbt_dn7 = assign23300_e25789_d_n7;
        var_ibbt_dn8 = assign23300_e25789_d_n8;

        let assign23310_e25792: f64 = if p.p824 == 0.5 { 1.0 } else { 0.0 };
        var_guard426 = assign23310_e25792;

        let (assign23320_e25811, assign23320_e25811_d_n5, assign23320_e25811_d_n6, assign23320_e25811_d_n7, assign23320_e25811_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) && (var_guard425 == 0.0)) && (var_guard426 != 0.0)) {
        let assign23320_e25806: f64 = (p.p821 - var_vbbt);
        let assign23320_e25808: f64 = (assign23320_e25806 * var_vbirbotinv);
        let assign23320_e25809: f64 = (assign23320_e25808).sqrt();
        (assign23320_e25809, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign23320_e25811;
        var_tmp_dn5 = assign23320_e25811_d_n5;
        var_tmp_dn6 = assign23320_e25811_d_n6;
        var_tmp_dn7 = assign23320_e25811_d_n7;
        var_tmp_dn8 = assign23320_e25811_d_n8;

        let (assign23330_e25832, assign23330_e25832_d_n5, assign23330_e25832_d_n6, assign23330_e25832_d_n7, assign23330_e25832_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) && (var_guard425 == 0.0)) && (var_guard426 == 0.0)) {
        let assign23330_e25826: f64 = (p.p821 - var_vbbt);
        let assign23330_e25828: f64 = (assign23330_e25826 * var_vbirbotinv);
        let assign23330_e25830: f64 = (assign23330_e25828).powf(p.p824);
        (assign23330_e25830, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign23330_e25832;
        var_tmp_dn5 = assign23330_e25832_d_n5;
        var_tmp_dn6 = assign23330_e25832_d_n6;
        var_tmp_dn7 = assign23330_e25832_d_n7;
        var_tmp_dn8 = assign23330_e25832_d_n8;

        let (assign23340_e25852, assign23340_e25852_d_n5, assign23340_e25852_d_n6, assign23340_e25852_d_n7, assign23340_e25852_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) && (var_guard425 == 0.0)) {
        let assign23340_e25845: f64 = (p.p821 - var_vbbt);
        let assign23340_e25847: f64 = (assign23340_e25845 * var_wdepnulrinvbot);
        let assign23340_e25849: f64 = (assign23340_e25847 / var_tmp);
        let assign23340_e25850: f64 = (var_one_over_one_minus_pbot * assign23340_e25849);
        (assign23340_e25850, (var_one_over_one_minus_pbot * (-((assign23340_e25847 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot * (-((assign23340_e25847 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot * (-((assign23340_e25847 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot * (-((assign23340_e25847 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign23340_e25852;
        var_fmaxr_dn5 = assign23340_e25852_d_n5;
        var_fmaxr_dn6 = assign23340_e25852_d_n6;
        var_fmaxr_dn7 = assign23340_e25852_d_n7;
        var_fmaxr_dn8 = assign23340_e25852_d_n8;

        let assign23350_e25854: f64 = (-var_fbbtbot);
        let assign23350_e25856: f64 = (assign23350_e25854 / var_fmaxr);
        let assign23350_e25857: f64 = (assign23350_e25856).abs();
        let assign23350_e25859: f64 = if assign23350_e25857 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard427 = assign23350_e25859;

        let (assign23360_e25877, assign23360_e25877_d_n5, assign23360_e25877_d_n6, assign23360_e25877_d_n7, assign23360_e25877_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) && (var_guard425 == 0.0)) && (var_guard427 != 0.0)) {
        let assign23360_e25872: f64 = (-var_fbbtbot);
        let assign23360_e25874: f64 = (assign23360_e25872 / var_fmaxr);
        let assign23360_e25875: f64 = (assign23360_e25874).exp();
        (assign23360_e25875, (assign23360_e25875 * (-((assign23360_e25872 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))), (assign23360_e25875 * (-((assign23360_e25872 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign23360_e25875 * (-((assign23360_e25872 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign23360_e25875 * (-((assign23360_e25872 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign23360_e25877;
        var_tmp_dn5 = assign23360_e25877_d_n5;
        var_tmp_dn6 = assign23360_e25877_d_n6;
        var_tmp_dn7 = assign23360_e25877_d_n7;
        var_tmp_dn8 = assign23360_e25877_d_n8;

        let assign23370_e25879: f64 = (-var_fbbtbot);
        let assign23370_e25881: f64 = (assign23370_e25879 / var_fmaxr);
        let assign23370_e25883: f64 = if assign23370_e25881 < 0.0 { 1.0 } else { 0.0 };
        var_guard428 = assign23370_e25883;

        let (assign23380_e25934, assign23380_e25934_d_n5, assign23380_e25934_d_n6, assign23380_e25934_d_n7, assign23380_e25934_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) && (var_guard425 == 0.0)) && (var_guard427 == 0.0)) && (var_guard428 != 0.0)) {
        let assign23380_e25901: f64 = (-230.25850929940458);
        let assign23380_e25903: f64 = (-var_fbbtbot);
        let assign23380_e25905: f64 = (assign23380_e25903 / var_fmaxr);
        let assign23380_e25906: f64 = (assign23380_e25901 - assign23380_e25905);
        let assign23380_e25910: f64 = (-230.25850929940458);
        let assign23380_e25912: f64 = (-var_fbbtbot);
        let assign23380_e25914: f64 = (assign23380_e25912 / var_fmaxr);
        let assign23380_e25915: f64 = (assign23380_e25910 - assign23380_e25914);
        let assign23380_e25918: f64 = (-230.25850929940458);
        let assign23380_e25920: f64 = (-var_fbbtbot);
        let assign23380_e25922: f64 = (assign23380_e25920 / var_fmaxr);
        let assign23380_e25923: f64 = (assign23380_e25918 - assign23380_e25922);
        let assign23380_e25925: f64 = (assign23380_e25923 * 0.3333333333333333);
        let assign23380_e25926: f64 = (1.0 + assign23380_e25925);
        let assign23380_e25927: f64 = (assign23380_e25915 * assign23380_e25926);
        let assign23380_e25928: f64 = (0.5 * assign23380_e25927);
        let assign23380_e25929: f64 = (1.0 + assign23380_e25928);
        let assign23380_e25930: f64 = (assign23380_e25906 * assign23380_e25929);
        let assign23380_e25931: f64 = (1.0 + assign23380_e25930);
        let assign23380_e25932: f64 = (1e-100 / assign23380_e25931);
        (assign23380_e25932, (-((1e-100 * (((-(-((assign23380_e25903 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign23380_e25929) + (assign23380_e25906 * (0.5 * (((-(-((assign23380_e25912 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign23380_e25926) + (assign23380_e25915 * ((-(-((assign23380_e25920 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign23380_e25931 * assign23380_e25931))), (-((1e-100 * (((-(-((assign23380_e25903 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign23380_e25929) + (assign23380_e25906 * (0.5 * (((-(-((assign23380_e25912 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign23380_e25926) + (assign23380_e25915 * ((-(-((assign23380_e25920 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign23380_e25931 * assign23380_e25931))), (-((1e-100 * (((-(-((assign23380_e25903 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign23380_e25929) + (assign23380_e25906 * (0.5 * (((-(-((assign23380_e25912 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign23380_e25926) + (assign23380_e25915 * ((-(-((assign23380_e25920 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign23380_e25931 * assign23380_e25931))), (-((1e-100 * (((-(-((assign23380_e25903 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign23380_e25929) + (assign23380_e25906 * (0.5 * (((-(-((assign23380_e25912 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign23380_e25926) + (assign23380_e25915 * ((-(-((assign23380_e25920 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign23380_e25931 * assign23380_e25931))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign23380_e25934;
        var_tmp_dn5 = assign23380_e25934_d_n5;
        var_tmp_dn6 = assign23380_e25934_d_n6;
        var_tmp_dn7 = assign23380_e25934_d_n7;
        var_tmp_dn8 = assign23380_e25934_d_n8;

        let (assign23390_e25983, assign23390_e25983_d_n5, assign23390_e25983_d_n6, assign23390_e25983_d_n7, assign23390_e25983_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) && (var_guard425 == 0.0)) && (var_guard427 == 0.0)) && (var_guard428 == 0.0)) {
        let assign23390_e25953: f64 = (-var_fbbtbot);
        let assign23390_e25955: f64 = (assign23390_e25953 / var_fmaxr);
        let assign23390_e25957: f64 = (assign23390_e25955 - 230.25850929940458);
        let assign23390_e25961: f64 = (-var_fbbtbot);
        let assign23390_e25963: f64 = (assign23390_e25961 / var_fmaxr);
        let assign23390_e25965: f64 = (assign23390_e25963 - 230.25850929940458);
        let assign23390_e25968: f64 = (-var_fbbtbot);
        let assign23390_e25970: f64 = (assign23390_e25968 / var_fmaxr);
        let assign23390_e25972: f64 = (assign23390_e25970 - 230.25850929940458);
        let assign23390_e25974: f64 = (assign23390_e25972 * 0.3333333333333333);
        let assign23390_e25975: f64 = (1.0 + assign23390_e25974);
        let assign23390_e25976: f64 = (assign23390_e25965 * assign23390_e25975);
        let assign23390_e25977: f64 = (0.5 * assign23390_e25976);
        let assign23390_e25978: f64 = (1.0 + assign23390_e25977);
        let assign23390_e25979: f64 = (assign23390_e25957 * assign23390_e25978);
        let assign23390_e25980: f64 = (1.0 + assign23390_e25979);
        let assign23390_e25981: f64 = (1e100 * assign23390_e25980);
        (assign23390_e25981, (1e100 * (((-((assign23390_e25953 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign23390_e25978) + (assign23390_e25957 * (0.5 * (((-((assign23390_e25961 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign23390_e25975) + (assign23390_e25965 * ((-((assign23390_e25968 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign23390_e25953 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign23390_e25978) + (assign23390_e25957 * (0.5 * (((-((assign23390_e25961 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign23390_e25975) + (assign23390_e25965 * ((-((assign23390_e25968 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign23390_e25953 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign23390_e25978) + (assign23390_e25957 * (0.5 * (((-((assign23390_e25961 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign23390_e25975) + (assign23390_e25965 * ((-((assign23390_e25968 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign23390_e25953 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign23390_e25978) + (assign23390_e25957 * (0.5 * (((-((assign23390_e25961 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign23390_e25975) + (assign23390_e25965 * ((-((assign23390_e25968 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign23390_e25983;
        var_tmp_dn5 = assign23390_e25983_d_n5;
        var_tmp_dn6 = assign23390_e25983_d_n6;
        var_tmp_dn7 = assign23390_e25983_d_n7;
        var_tmp_dn8 = assign23390_e25983_d_n8;

        let (assign23400_e26003, assign23400_e26003_d_n5, assign23400_e26003_d_n6, assign23400_e26003_d_n7, assign23400_e26003_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) && (var_guard425 == 0.0)) {
        let assign23400_e25996: f64 = (var_v4 * var_fmaxr);
        let assign23400_e25998: f64 = (assign23400_e25996 * var_fmaxr);
        let assign23400_e26000: f64 = (assign23400_e25998 * var_tmp);
        let assign23400_e26001: f64 = (p.p844 * assign23400_e26000);
        (assign23400_e26001, (p.p844 * (((((var_v4 * var_fmaxr_dn5) * var_fmaxr) + (assign23400_e25996 * var_fmaxr_dn5)) * var_tmp) + (assign23400_e25998 * var_tmp_dn5))), (p.p844 * (((((var_v4 * var_fmaxr_dn6) * var_fmaxr) + (assign23400_e25996 * var_fmaxr_dn6)) * var_tmp) + (assign23400_e25998 * var_tmp_dn6))), (p.p844 * (((((var_v4 * var_fmaxr_dn7) * var_fmaxr) + (assign23400_e25996 * var_fmaxr_dn7)) * var_tmp) + (assign23400_e25998 * var_tmp_dn7))), (p.p844 * (((((var_v4 * var_fmaxr_dn8) * var_fmaxr) + (assign23400_e25996 * var_fmaxr_dn8)) * var_tmp) + (assign23400_e25998 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign23400_e26003;
        var_ibbt_dn5 = assign23400_e26003_d_n5;
        var_ibbt_dn6 = assign23400_e26003_d_n6;
        var_ibbt_dn7 = assign23400_e26003_d_n7;
        var_ibbt_dn8 = assign23400_e26003_d_n8;

        let assign23410_e26006: f64 = if p.p853 > 1000.0 { 1.0 } else { 0.0 };
        var_guard429 = assign23410_e26006;

        let (assign23420_e26017, assign23420_e26017_d_n5, assign23420_e26017_d_n6, assign23420_e26017_d_n7, assign23420_e26017_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) && (var_guard429 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign23420_e26017;
        var_fbreakdown_dn5 = assign23420_e26017_d_n5;
        var_fbreakdown_dn6 = assign23420_e26017_d_n6;
        var_fbreakdown_dn7 = assign23420_e26017_d_n7;
        var_fbreakdown_dn8 = assign23420_e26017_d_n8;

        let assign23430_e26020: f64 = (-var_alphaav);
        let assign23430_e26022: f64 = (assign23430_e26020 * p.p853);
        let assign23430_e26023: f64 = if var_vav > assign23430_e26022 { 1.0 } else { 0.0 };
        var_guard430 = assign23430_e26023;

        let assign23440_e26026: f64 = if p.p856 == 4.0 { 1.0 } else { 0.0 };
        var_guard431 = assign23440_e26026;

        let (assign23450_e26056, assign23450_e26056_d_n5, assign23450_e26056_d_n6, assign23450_e26056_d_n7, assign23450_e26056_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) && (var_guard429 == 0.0)) && (var_guard430 != 0.0)) && (var_guard431 != 0.0)) {
        let assign23450_e26042: f64 = (var_vav * var_vbrinvbot);
        let assign23450_e26045: f64 = (var_vav * var_vbrinvbot);
        let assign23450_e26046: f64 = (assign23450_e26042 * assign23450_e26045);
        let assign23450_e26049: f64 = (var_vav * var_vbrinvbot);
        let assign23450_e26050: f64 = (assign23450_e26046 * assign23450_e26049);
        let assign23450_e26053: f64 = (var_vav * var_vbrinvbot);
        let assign23450_e26054: f64 = (assign23450_e26050 * assign23450_e26053);
        (assign23450_e26054, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign23450_e26056;
        var_tmp_dn5 = assign23450_e26056_d_n5;
        var_tmp_dn6 = assign23450_e26056_d_n6;
        var_tmp_dn7 = assign23450_e26056_d_n7;
        var_tmp_dn8 = assign23450_e26056_d_n8;

        let (assign23460_e26078, assign23460_e26078_d_n5, assign23460_e26078_d_n6, assign23460_e26078_d_n7, assign23460_e26078_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) && (var_guard429 == 0.0)) && (var_guard430 != 0.0)) && (var_guard431 == 0.0)) {
        let assign23460_e26073: f64 = (var_vav * var_vbrinvbot);
        let assign23460_e26074: f64 = (assign23460_e26073).abs();
        let assign23460_e26076: f64 = (assign23460_e26074).powf(p.p856);
        (assign23460_e26076, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign23460_e26078;
        var_tmp_dn5 = assign23460_e26078_d_n5;
        var_tmp_dn6 = assign23460_e26078_d_n6;
        var_tmp_dn7 = assign23460_e26078_d_n7;
        var_tmp_dn8 = assign23460_e26078_d_n8;

        let (assign23470_e26096, assign23470_e26096_d_n5, assign23470_e26096_d_n6, assign23470_e26096_d_n7, assign23470_e26096_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) && (var_guard429 == 0.0)) && (var_guard430 != 0.0)) {
        let assign23470_e26093: f64 = (1.0 - var_tmp);
        let assign23470_e26094: f64 = (1.0 / assign23470_e26093);
        (assign23470_e26094, (-((-var_tmp_dn5) / (assign23470_e26093 * assign23470_e26093))), (-((-var_tmp_dn6) / (assign23470_e26093 * assign23470_e26093))), (-((-var_tmp_dn7) / (assign23470_e26093 * assign23470_e26093))), (-((-var_tmp_dn8) / (assign23470_e26093 * assign23470_e26093))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign23470_e26096;
        var_fbreakdown_dn5 = assign23470_e26096_d_n5;
        var_fbreakdown_dn6 = assign23470_e26096_d_n6;
        var_fbreakdown_dn7 = assign23470_e26096_d_n7;
        var_fbreakdown_dn8 = assign23470_e26096_d_n8;

        let (assign23480_e26119, assign23480_e26119_d_n5, assign23480_e26119_d_n6, assign23480_e26119_d_n7, assign23480_e26119_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) && (var_guard429 == 0.0)) && (var_guard430 == 0.0)) {
        let assign23480_e26113: f64 = (var_alphaav * p.p853);
        let assign23480_e26114: f64 = (var_vav + assign23480_e26113);
        let assign23480_e26116: f64 = (assign23480_e26114 * var_slopebot);
        let assign23480_e26117: f64 = (var_fstopbot + assign23480_e26116);
        (assign23480_e26117, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign23480_e26119;
        var_fbreakdown_dn5 = assign23480_e26119_d_n5;
        var_fbreakdown_dn6 = assign23480_e26119_d_n6;
        var_fbreakdown_dn7 = assign23480_e26119_d_n7;
        var_fbreakdown_dn8 = assign23480_e26119_d_n8;

        *var_erfcpos_slot = var_erfcpos;
        *var_erfcpos_dn5_slot = var_erfcpos_dn5;
        *var_erfcpos_dn6_slot = var_erfcpos_dn6;
        *var_erfcpos_dn7_slot = var_erfcpos_dn7;
        *var_erfcpos_dn8_slot = var_erfcpos_dn8;
        *var_erfctimesexpmtat_slot = var_erfctimesexpmtat;
        *var_erfctimesexpmtat_dn5_slot = var_erfctimesexpmtat_dn5;
        *var_erfctimesexpmtat_dn6_slot = var_erfctimesexpmtat_dn6;
        *var_erfctimesexpmtat_dn7_slot = var_erfctimesexpmtat_dn7;
        *var_erfctimesexpmtat_dn8_slot = var_erfctimesexpmtat_dn8;
        *var_fbreakdown_slot = var_fbreakdown;
        *var_fbreakdown_dn5_slot = var_fbreakdown_dn5;
        *var_fbreakdown_dn6_slot = var_fbreakdown_dn6;
        *var_fbreakdown_dn7_slot = var_fbreakdown_dn7;
        *var_fbreakdown_dn8_slot = var_fbreakdown_dn8;
        *var_fmaxr_slot = var_fmaxr;
        *var_fmaxr_dn5_slot = var_fmaxr_dn5;
        *var_fmaxr_dn6_slot = var_fmaxr_dn6;
        *var_fmaxr_dn7_slot = var_fmaxr_dn7;
        *var_fmaxr_dn8_slot = var_fmaxr_dn8;
        *var_gammamax_slot = var_gammamax;
        *var_gammamax_dn5_slot = var_gammamax_dn5;
        *var_gammamax_dn6_slot = var_gammamax_dn6;
        *var_gammamax_dn7_slot = var_gammamax_dn7;
        *var_gammamax_dn8_slot = var_gammamax_dn8;
        *var_guard421_slot = var_guard421;
        *var_guard422_slot = var_guard422;
        *var_guard423_slot = var_guard423;
        *var_guard424_slot = var_guard424;
        *var_guard425_slot = var_guard425;
        *var_guard426_slot = var_guard426;
        *var_guard427_slot = var_guard427;
        *var_guard428_slot = var_guard428;
        *var_guard429_slot = var_guard429;
        *var_guard430_slot = var_guard430;
        *var_guard431_slot = var_guard431;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn5_slot = var_ibbt_dn5;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
        *var_itat_slot = var_itat;
        *var_itat_dn5_slot = var_itat_dn5;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_terfc_slot = var_terfc;
        *var_terfc_dn5_slot = var_terfc_dn5;
        *var_terfc_dn6_slot = var_terfc_dn6;
        *var_terfc_dn7_slot = var_terfc_dn7;
        *var_terfc_dn8_slot = var_terfc_dn8;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn5_slot = var_tmp_dn5;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_ysq_slot = var_ysq;
        *var_ysq_dn5_slot = var_ysq_dn5;
        *var_ysq_dn6_slot = var_ysq_dn6;
        *var_ysq_dn7_slot = var_ysq_dn7;
        *var_ysq_dn8_slot = var_ysq_dn8;
    }

    pub(super) fn stamp_transient_block_43(
        p: &Parameters,
        var_atatsti: f64,
        var_btatpartsti: f64,
        var_fbreakdown: f64,
        var_fbreakdown_dn5: f64,
        var_fbreakdown_dn6: f64,
        var_fbreakdown_dn7: f64,
        var_fbreakdown_dn8: f64,
        var_ftdsti: f64,
        var_guard176: f64,
        var_guard193: f64,
        var_guard415: f64,
        var_ibbt: f64,
        var_ibbt_dn5: f64,
        var_ibbt_dn6: f64,
        var_ibbt_dn7: f64,
        var_ibbt_dn8: f64,
        var_idmult: f64,
        var_idsatsti: f64,
        var_lssource_i: f64,
        var_one_minus_psti: f64,
        var_one_over_one_minus_psti: f64,
        var_perfc: f64,
        var_two_psistar: f64,
        var_vbirstiinv: f64,
        var_vbisti: f64,
        var_vjsrh: f64,
        var_wdepnulrsti: f64,
        var_zinv: f64,
        var_asrh_slot: &mut f64,
        var_asrh_dn5_slot: &mut f64,
        var_asrh_dn6_slot: &mut f64,
        var_asrh_dn7_slot: &mut f64,
        var_asrh_dn8_slot: &mut f64,
        var_btat_slot: &mut f64,
        var_btat_dn5_slot: &mut f64,
        var_btat_dn6_slot: &mut f64,
        var_btat_dn7_slot: &mut f64,
        var_btat_dn8_slot: &mut f64,
        var_dwsrh_slot: &mut f64,
        var_guard432_slot: &mut f64,
        var_guard433_slot: &mut f64,
        var_guard434_slot: &mut f64,
        var_guard435_slot: &mut f64,
        var_guard436_slot: &mut f64,
        var_guard437_slot: &mut f64,
        var_guard438_slot: &mut f64,
        var_guard439_slot: &mut f64,
        var_id__blk213_slot: &mut f64,
        var_ijunbot_slot: &mut f64,
        var_ijunbot_dn5_slot: &mut f64,
        var_ijunbot_dn6_slot: &mut f64,
        var_ijunbot_dn7_slot: &mut f64,
        var_ijunbot_dn8_slot: &mut f64,
        var_ijunsti_slot: &mut f64,
        var_ijunsti_dn5_slot: &mut f64,
        var_ijunsti_dn6_slot: &mut f64,
        var_ijunsti_dn7_slot: &mut f64,
        var_ijunsti_dn8_slot: &mut f64,
        var_isrh_slot: &mut f64,
        var_isrh_dn5_slot: &mut f64,
        var_isrh_dn6_slot: &mut f64,
        var_isrh_dn7_slot: &mut f64,
        var_isrh_dn8_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn5_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_ktat_slot: &mut f64,
        var_ktat_dn5_slot: &mut f64,
        var_ktat_dn6_slot: &mut f64,
        var_ktat_dn7_slot: &mut f64,
        var_ktat_dn8_slot: &mut f64,
        var_ltat_slot: &mut f64,
        var_ltat_dn5_slot: &mut f64,
        var_ltat_dn6_slot: &mut f64,
        var_ltat_dn7_slot: &mut f64,
        var_ltat_dn8_slot: &mut f64,
        var_mtat_slot: &mut f64,
        var_mtat_dn5_slot: &mut f64,
        var_mtat_dn6_slot: &mut f64,
        var_mtat_dn7_slot: &mut f64,
        var_mtat_dn8_slot: &mut f64,
        var_sqrtumax_slot: &mut f64,
        var_sqrtumax_dn5_slot: &mut f64,
        var_sqrtumax_dn6_slot: &mut f64,
        var_sqrtumax_dn7_slot: &mut f64,
        var_sqrtumax_dn8_slot: &mut f64,
        var_terfc_slot: &mut f64,
        var_terfc_dn5_slot: &mut f64,
        var_terfc_dn6_slot: &mut f64,
        var_terfc_dn7_slot: &mut f64,
        var_terfc_dn8_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn5_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_twoatatoverthreebtat_slot: &mut f64,
        var_twoatatoverthreebtat_dn5_slot: &mut f64,
        var_twoatatoverthreebtat_dn6_slot: &mut f64,
        var_twoatatoverthreebtat_dn7_slot: &mut f64,
        var_twoatatoverthreebtat_dn8_slot: &mut f64,
        var_umax_slot: &mut f64,
        var_umax_dn5_slot: &mut f64,
        var_umax_dn6_slot: &mut f64,
        var_umax_dn7_slot: &mut f64,
        var_umax_dn8_slot: &mut f64,
        var_umaxbeforelimiting_slot: &mut f64,
        var_umaxbeforelimiting_dn5_slot: &mut f64,
        var_umaxbeforelimiting_dn6_slot: &mut f64,
        var_umaxbeforelimiting_dn7_slot: &mut f64,
        var_umaxbeforelimiting_dn8_slot: &mut f64,
        var_umaxpoweronepointfive_slot: &mut f64,
        var_umaxpoweronepointfive_dn5_slot: &mut f64,
        var_umaxpoweronepointfive_dn6_slot: &mut f64,
        var_umaxpoweronepointfive_dn7_slot: &mut f64,
        var_umaxpoweronepointfive_dn8_slot: &mut f64,
        var_vbi_minus_vjsrh_slot: &mut f64,
        var_wdep_slot: &mut f64,
        var_wdep_dn5_slot: &mut f64,
        var_wdep_dn6_slot: &mut f64,
        var_wdep_dn7_slot: &mut f64,
        var_wdep_dn8_slot: &mut f64,
        var_wgamma_slot: &mut f64,
        var_wgamma_dn5_slot: &mut f64,
        var_wgamma_dn6_slot: &mut f64,
        var_wgamma_dn7_slot: &mut f64,
        var_wgamma_dn8_slot: &mut f64,
        var_wsrh_slot: &mut f64,
        var_wsrhstep_slot: &mut f64,
        var_wtat_slot: &mut f64,
        var_wtat_dn5_slot: &mut f64,
        var_wtat_dn6_slot: &mut f64,
        var_wtat_dn7_slot: &mut f64,
        var_wtat_dn8_slot: &mut f64,
        var_xerfc_slot: &mut f64,
        var_xerfc_dn5_slot: &mut f64,
        var_xerfc_dn6_slot: &mut f64,
        var_xerfc_dn7_slot: &mut f64,
        var_xerfc_dn8_slot: &mut f64,
        var_ysq_slot: &mut f64,
        var_ysq_dn5_slot: &mut f64,
        var_ysq_dn6_slot: &mut f64,
        var_ysq_dn7_slot: &mut f64,
        var_ysq_dn8_slot: &mut f64,
    ) {
        let mut var_asrh: f64 = *var_asrh_slot;
        let mut var_asrh_dn5: f64 = *var_asrh_dn5_slot;
        let mut var_asrh_dn6: f64 = *var_asrh_dn6_slot;
        let mut var_asrh_dn7: f64 = *var_asrh_dn7_slot;
        let mut var_asrh_dn8: f64 = *var_asrh_dn8_slot;
        let mut var_btat: f64 = *var_btat_slot;
        let mut var_btat_dn5: f64 = *var_btat_dn5_slot;
        let mut var_btat_dn6: f64 = *var_btat_dn6_slot;
        let mut var_btat_dn7: f64 = *var_btat_dn7_slot;
        let mut var_btat_dn8: f64 = *var_btat_dn8_slot;
        let mut var_dwsrh: f64 = *var_dwsrh_slot;
        let mut var_guard432: f64 = *var_guard432_slot;
        let mut var_guard433: f64 = *var_guard433_slot;
        let mut var_guard434: f64 = *var_guard434_slot;
        let mut var_guard435: f64 = *var_guard435_slot;
        let mut var_guard436: f64 = *var_guard436_slot;
        let mut var_guard437: f64 = *var_guard437_slot;
        let mut var_guard438: f64 = *var_guard438_slot;
        let mut var_guard439: f64 = *var_guard439_slot;
        let mut var_id__blk213: f64 = *var_id__blk213_slot;
        let mut var_ijunbot: f64 = *var_ijunbot_slot;
        let mut var_ijunbot_dn5: f64 = *var_ijunbot_dn5_slot;
        let mut var_ijunbot_dn6: f64 = *var_ijunbot_dn6_slot;
        let mut var_ijunbot_dn7: f64 = *var_ijunbot_dn7_slot;
        let mut var_ijunbot_dn8: f64 = *var_ijunbot_dn8_slot;
        let mut var_ijunsti: f64 = *var_ijunsti_slot;
        let mut var_ijunsti_dn5: f64 = *var_ijunsti_dn5_slot;
        let mut var_ijunsti_dn6: f64 = *var_ijunsti_dn6_slot;
        let mut var_ijunsti_dn7: f64 = *var_ijunsti_dn7_slot;
        let mut var_ijunsti_dn8: f64 = *var_ijunsti_dn8_slot;
        let mut var_isrh: f64 = *var_isrh_slot;
        let mut var_isrh_dn5: f64 = *var_isrh_dn5_slot;
        let mut var_isrh_dn6: f64 = *var_isrh_dn6_slot;
        let mut var_isrh_dn7: f64 = *var_isrh_dn7_slot;
        let mut var_isrh_dn8: f64 = *var_isrh_dn8_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn5: f64 = *var_itat_dn5_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_ktat: f64 = *var_ktat_slot;
        let mut var_ktat_dn5: f64 = *var_ktat_dn5_slot;
        let mut var_ktat_dn6: f64 = *var_ktat_dn6_slot;
        let mut var_ktat_dn7: f64 = *var_ktat_dn7_slot;
        let mut var_ktat_dn8: f64 = *var_ktat_dn8_slot;
        let mut var_ltat: f64 = *var_ltat_slot;
        let mut var_ltat_dn5: f64 = *var_ltat_dn5_slot;
        let mut var_ltat_dn6: f64 = *var_ltat_dn6_slot;
        let mut var_ltat_dn7: f64 = *var_ltat_dn7_slot;
        let mut var_ltat_dn8: f64 = *var_ltat_dn8_slot;
        let mut var_mtat: f64 = *var_mtat_slot;
        let mut var_mtat_dn5: f64 = *var_mtat_dn5_slot;
        let mut var_mtat_dn6: f64 = *var_mtat_dn6_slot;
        let mut var_mtat_dn7: f64 = *var_mtat_dn7_slot;
        let mut var_mtat_dn8: f64 = *var_mtat_dn8_slot;
        let mut var_sqrtumax: f64 = *var_sqrtumax_slot;
        let mut var_sqrtumax_dn5: f64 = *var_sqrtumax_dn5_slot;
        let mut var_sqrtumax_dn6: f64 = *var_sqrtumax_dn6_slot;
        let mut var_sqrtumax_dn7: f64 = *var_sqrtumax_dn7_slot;
        let mut var_sqrtumax_dn8: f64 = *var_sqrtumax_dn8_slot;
        let mut var_terfc: f64 = *var_terfc_slot;
        let mut var_terfc_dn5: f64 = *var_terfc_dn5_slot;
        let mut var_terfc_dn6: f64 = *var_terfc_dn6_slot;
        let mut var_terfc_dn7: f64 = *var_terfc_dn7_slot;
        let mut var_terfc_dn8: f64 = *var_terfc_dn8_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn5: f64 = *var_tmp_dn5_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_twoatatoverthreebtat: f64 = *var_twoatatoverthreebtat_slot;
        let mut var_twoatatoverthreebtat_dn5: f64 = *var_twoatatoverthreebtat_dn5_slot;
        let mut var_twoatatoverthreebtat_dn6: f64 = *var_twoatatoverthreebtat_dn6_slot;
        let mut var_twoatatoverthreebtat_dn7: f64 = *var_twoatatoverthreebtat_dn7_slot;
        let mut var_twoatatoverthreebtat_dn8: f64 = *var_twoatatoverthreebtat_dn8_slot;
        let mut var_umax: f64 = *var_umax_slot;
        let mut var_umax_dn5: f64 = *var_umax_dn5_slot;
        let mut var_umax_dn6: f64 = *var_umax_dn6_slot;
        let mut var_umax_dn7: f64 = *var_umax_dn7_slot;
        let mut var_umax_dn8: f64 = *var_umax_dn8_slot;
        let mut var_umaxbeforelimiting: f64 = *var_umaxbeforelimiting_slot;
        let mut var_umaxbeforelimiting_dn5: f64 = *var_umaxbeforelimiting_dn5_slot;
        let mut var_umaxbeforelimiting_dn6: f64 = *var_umaxbeforelimiting_dn6_slot;
        let mut var_umaxbeforelimiting_dn7: f64 = *var_umaxbeforelimiting_dn7_slot;
        let mut var_umaxbeforelimiting_dn8: f64 = *var_umaxbeforelimiting_dn8_slot;
        let mut var_umaxpoweronepointfive: f64 = *var_umaxpoweronepointfive_slot;
        let mut var_umaxpoweronepointfive_dn5: f64 = *var_umaxpoweronepointfive_dn5_slot;
        let mut var_umaxpoweronepointfive_dn6: f64 = *var_umaxpoweronepointfive_dn6_slot;
        let mut var_umaxpoweronepointfive_dn7: f64 = *var_umaxpoweronepointfive_dn7_slot;
        let mut var_umaxpoweronepointfive_dn8: f64 = *var_umaxpoweronepointfive_dn8_slot;
        let mut var_vbi_minus_vjsrh: f64 = *var_vbi_minus_vjsrh_slot;
        let mut var_wdep: f64 = *var_wdep_slot;
        let mut var_wdep_dn5: f64 = *var_wdep_dn5_slot;
        let mut var_wdep_dn6: f64 = *var_wdep_dn6_slot;
        let mut var_wdep_dn7: f64 = *var_wdep_dn7_slot;
        let mut var_wdep_dn8: f64 = *var_wdep_dn8_slot;
        let mut var_wgamma: f64 = *var_wgamma_slot;
        let mut var_wgamma_dn5: f64 = *var_wgamma_dn5_slot;
        let mut var_wgamma_dn6: f64 = *var_wgamma_dn6_slot;
        let mut var_wgamma_dn7: f64 = *var_wgamma_dn7_slot;
        let mut var_wgamma_dn8: f64 = *var_wgamma_dn8_slot;
        let mut var_wsrh: f64 = *var_wsrh_slot;
        let mut var_wsrhstep: f64 = *var_wsrhstep_slot;
        let mut var_wtat: f64 = *var_wtat_slot;
        let mut var_wtat_dn5: f64 = *var_wtat_dn5_slot;
        let mut var_wtat_dn6: f64 = *var_wtat_dn6_slot;
        let mut var_wtat_dn7: f64 = *var_wtat_dn7_slot;
        let mut var_wtat_dn8: f64 = *var_wtat_dn8_slot;
        let mut var_xerfc: f64 = *var_xerfc_slot;
        let mut var_xerfc_dn5: f64 = *var_xerfc_dn5_slot;
        let mut var_xerfc_dn6: f64 = *var_xerfc_dn6_slot;
        let mut var_xerfc_dn7: f64 = *var_xerfc_dn7_slot;
        let mut var_xerfc_dn8: f64 = *var_xerfc_dn8_slot;
        let mut var_ysq: f64 = *var_ysq_slot;
        let mut var_ysq_dn5: f64 = *var_ysq_dn5_slot;
        let mut var_ysq_dn6: f64 = *var_ysq_dn6_slot;
        let mut var_ysq_dn7: f64 = *var_ysq_dn7_slot;
        let mut var_ysq_dn8: f64 = *var_ysq_dn8_slot;

        let (assign23490_e26138, assign23490_e26138_d_n5, assign23490_e26138_d_n6, assign23490_e26138_d_n7, assign23490_e26138_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard415 == 0.0)) {
        let assign23490_e26129: f64 = (var_id__blk213 + var_isrh);
        let assign23490_e26131: f64 = (assign23490_e26129 + var_itat);
        let assign23490_e26133: f64 = (assign23490_e26131 + var_ibbt);
        let assign23490_e26134: f64 = (p.p29 * assign23490_e26133);
        let assign23490_e26136: f64 = (assign23490_e26134 * var_fbreakdown);
        (assign23490_e26136, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign23490_e26134 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign23490_e26134 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign23490_e26134 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign23490_e26134 * var_fbreakdown_dn8)),)
    } else {
        (var_ijunbot, var_ijunbot_dn5, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8,)
    }
};
        var_ijunbot = assign23490_e26138;
        var_ijunbot_dn5 = assign23490_e26138_d_n5;
        var_ijunbot_dn6 = assign23490_e26138_d_n6;
        var_ijunbot_dn7 = assign23490_e26138_d_n7;
        var_ijunbot_dn8 = assign23490_e26138_d_n8;

        let assign23500_e26141: f64 = if var_lssource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard432 = assign23500_e26141;

        let (assign23510_e26149, assign23510_e26149_d_n5, assign23510_e26149_d_n6, assign23510_e26149_d_n7, assign23510_e26149_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunsti, var_ijunsti_dn5, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8,)
    }
};
        var_ijunsti = assign23510_e26149;
        var_ijunsti_dn5 = assign23510_e26149_d_n5;
        var_ijunsti_dn6 = assign23510_e26149_d_n6;
        var_ijunsti_dn7 = assign23510_e26149_d_n7;
        var_ijunsti_dn8 = assign23510_e26149_d_n8;

        let (assign23520_e26160,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) {
        let assign23520_e26158: f64 = (var_idsatsti * var_idmult);
        (assign23520_e26158,)
    } else {
        (var_id__blk213,)
    }
};
        var_id__blk213 = assign23520_e26160;

        let assign23530_e26167: f64 = if ((p.p834 == 0.0) && (p.p839 == 0.0)) { 1.0 } else { 0.0 };
        var_guard433 = assign23530_e26167;

        let (assign23540_e26178, assign23540_e26178_d_n5, assign23540_e26178_d_n6, assign23540_e26178_d_n7, assign23540_e26178_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) && (var_guard433 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign23540_e26178;
        var_isrh_dn5 = assign23540_e26178_d_n5;
        var_isrh_dn6 = assign23540_e26178_d_n6;
        var_isrh_dn7 = assign23540_e26178_d_n7;
        var_isrh_dn8 = assign23540_e26178_d_n8;

        let (assign23550_e26192,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) && (var_guard433 == 0.0)) {
        let assign23550_e26190: f64 = (var_vbisti - var_vjsrh);
        (assign23550_e26190,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign23550_e26192;

        let (assign23560_e26211,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) && (var_guard433 == 0.0)) {
        let assign23560_e26206: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign23560_e26207: f64 = (1.0 - assign23560_e26206);
        let assign23560_e26208: f64 = (assign23560_e26207).sqrt();
        let assign23560_e26209: f64 = (1.0 - assign23560_e26208);
        (assign23560_e26209,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign23560_e26211;

        let assign23570_e26214: f64 = if p.p825 == 0.5 { 1.0 } else { 0.0 };
        var_guard434 = assign23570_e26214;

        let (assign23580_e26228,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) && (var_guard433 == 0.0)) && (var_guard434 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign23580_e26228;

        let (assign23590_e26260,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) && (var_guard433 == 0.0)) && (var_guard434 == 0.0)) {
        let assign23590_e26243: f64 = (var_wsrhstep * var_wsrhstep);
        let assign23590_e26245: f64 = (var_wsrhstep).ln();
        let assign23590_e26246: f64 = (assign23590_e26243 * assign23590_e26245);
        let assign23590_e26249: f64 = (1.0 - var_wsrhstep);
        let assign23590_e26250: f64 = (assign23590_e26246 / assign23590_e26249);
        let assign23590_e26252: f64 = (assign23590_e26250 + var_wsrhstep);
        let assign23590_e26256: f64 = (2.0 * p.p825);
        let assign23590_e26257: f64 = (1.0 - assign23590_e26256);
        let assign23590_e26258: f64 = (assign23590_e26252 * assign23590_e26257);
        (assign23590_e26258,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign23590_e26260;

        let (assign23600_e26274,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) && (var_guard433 == 0.0)) {
        let assign23600_e26272: f64 = (var_wsrhstep + var_dwsrh);
        (assign23600_e26272,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign23600_e26274;

        let assign23610_e26277: f64 = if p.p825 == 0.5 { 1.0 } else { 0.0 };
        var_guard435 = assign23610_e26277;

        let (assign23620_e26294, assign23620_e26294_d_n5, assign23620_e26294_d_n6, assign23620_e26294_d_n7, assign23620_e26294_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) && (var_guard433 == 0.0)) && (var_guard435 != 0.0)) {
        let assign23620_e26291: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv);
        let assign23620_e26292: f64 = (assign23620_e26291).sqrt();
        (assign23620_e26292, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign23620_e26294;
        var_tmp_dn5 = assign23620_e26294_d_n5;
        var_tmp_dn6 = assign23620_e26294_d_n6;
        var_tmp_dn7 = assign23620_e26294_d_n7;
        var_tmp_dn8 = assign23620_e26294_d_n8;

        let (assign23630_e26313, assign23630_e26313_d_n5, assign23630_e26313_d_n6, assign23630_e26313_d_n7, assign23630_e26313_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) && (var_guard433 == 0.0)) && (var_guard435 == 0.0)) {
        let assign23630_e26309: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv);
        let assign23630_e26311: f64 = (assign23630_e26309).powf(p.p825);
        (assign23630_e26311, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign23630_e26313;
        var_tmp_dn5 = assign23630_e26313_d_n5;
        var_tmp_dn6 = assign23630_e26313_d_n6;
        var_tmp_dn7 = assign23630_e26313_d_n7;
        var_tmp_dn8 = assign23630_e26313_d_n8;

        let (assign23640_e26327, assign23640_e26327_d_n5, assign23640_e26327_d_n6, assign23640_e26327_d_n7, assign23640_e26327_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) && (var_guard433 == 0.0)) {
        let assign23640_e26325: f64 = (var_wdepnulrsti * var_tmp);
        (assign23640_e26325, (var_wdepnulrsti * var_tmp_dn5), (var_wdepnulrsti * var_tmp_dn6), (var_wdepnulrsti * var_tmp_dn7), (var_wdepnulrsti * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign23640_e26327;
        var_wdep_dn5 = assign23640_e26327_d_n5;
        var_wdep_dn6 = assign23640_e26327_d_n6;
        var_wdep_dn7 = assign23640_e26327_d_n7;
        var_wdep_dn8 = assign23640_e26327_d_n8;

        let (assign23650_e26345, assign23650_e26345_d_n5, assign23650_e26345_d_n6, assign23650_e26345_d_n7, assign23650_e26345_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) && (var_guard433 == 0.0)) {
        let assign23650_e26340: f64 = (var_zinv - 1.0);
        let assign23650_e26342: f64 = (assign23650_e26340 * var_wdep);
        let assign23650_e26343: f64 = (var_ftdsti * assign23650_e26342);
        (assign23650_e26343, (var_ftdsti * (assign23650_e26340 * var_wdep_dn5)), (var_ftdsti * (assign23650_e26340 * var_wdep_dn6)), (var_ftdsti * (assign23650_e26340 * var_wdep_dn7)), (var_ftdsti * (assign23650_e26340 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign23650_e26345;
        var_asrh_dn5 = assign23650_e26345_d_n5;
        var_asrh_dn6 = assign23650_e26345_d_n6;
        var_asrh_dn7 = assign23650_e26345_d_n7;
        var_asrh_dn8 = assign23650_e26345_d_n8;

        let (assign23660_e26361, assign23660_e26361_d_n5, assign23660_e26361_d_n6, assign23660_e26361_d_n7, assign23660_e26361_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) && (var_guard433 == 0.0)) {
        let assign23660_e26358: f64 = (var_asrh * var_wsrh);
        let assign23660_e26359: f64 = (p.p834 * assign23660_e26358);
        (assign23660_e26359, (p.p834 * (var_asrh_dn5 * var_wsrh)), (p.p834 * (var_asrh_dn6 * var_wsrh)), (p.p834 * (var_asrh_dn7 * var_wsrh)), (p.p834 * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign23660_e26361;
        var_isrh_dn5 = assign23660_e26361_d_n5;
        var_isrh_dn6 = assign23660_e26361_d_n6;
        var_isrh_dn7 = assign23660_e26361_d_n7;
        var_isrh_dn8 = assign23660_e26361_d_n8;

        let assign23670_e26364: f64 = if p.p839 == 0.0 { 1.0 } else { 0.0 };
        var_guard436 = assign23670_e26364;

        let (assign23680_e26375, assign23680_e26375_d_n5, assign23680_e26375_d_n6, assign23680_e26375_d_n7, assign23680_e26375_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) && (var_guard436 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign23680_e26375;
        var_itat_dn5 = assign23680_e26375_d_n5;
        var_itat_dn6 = assign23680_e26375_d_n6;
        var_itat_dn7 = assign23680_e26375_d_n7;
        var_itat_dn8 = assign23680_e26375_d_n8;

        let (assign23690_e26393, assign23690_e26393_d_n5, assign23690_e26393_d_n6, assign23690_e26393_d_n7, assign23690_e26393_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) && (var_guard436 == 0.0)) {
        let assign23690_e26388: f64 = (var_wdep * var_one_minus_psti);
        let assign23690_e26390: f64 = (assign23690_e26388 / var_vbi_minus_vjsrh);
        let assign23690_e26391: f64 = (var_btatpartsti * assign23690_e26390);
        (assign23690_e26391, (var_btatpartsti * ((var_wdep_dn5 * var_one_minus_psti) / var_vbi_minus_vjsrh)), (var_btatpartsti * ((var_wdep_dn6 * var_one_minus_psti) / var_vbi_minus_vjsrh)), (var_btatpartsti * ((var_wdep_dn7 * var_one_minus_psti) / var_vbi_minus_vjsrh)), (var_btatpartsti * ((var_wdep_dn8 * var_one_minus_psti) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign23690_e26393;
        var_btat_dn5 = assign23690_e26393_d_n5;
        var_btat_dn6 = assign23690_e26393_d_n6;
        var_btat_dn7 = assign23690_e26393_d_n7;
        var_btat_dn8 = assign23690_e26393_d_n8;

        let (assign23700_e26409, assign23700_e26409_d_n5, assign23700_e26409_d_n6, assign23700_e26409_d_n7, assign23700_e26409_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) && (var_guard436 == 0.0)) {
        let assign23700_e26405: f64 = (0.666666666666667 * var_atatsti);
        let assign23700_e26407: f64 = (assign23700_e26405 / var_btat);
        (assign23700_e26407, (-((assign23700_e26405 * var_btat_dn5) / (var_btat * var_btat))), (-((assign23700_e26405 * var_btat_dn6) / (var_btat * var_btat))), (-((assign23700_e26405 * var_btat_dn7) / (var_btat * var_btat))), (-((assign23700_e26405 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign23700_e26409;
        var_twoatatoverthreebtat_dn5 = assign23700_e26409_d_n5;
        var_twoatatoverthreebtat_dn6 = assign23700_e26409_d_n6;
        var_twoatatoverthreebtat_dn7 = assign23700_e26409_d_n7;
        var_twoatatoverthreebtat_dn8 = assign23700_e26409_d_n8;

        let (assign23710_e26423, assign23710_e26423_d_n5, assign23710_e26423_d_n6, assign23710_e26423_d_n7, assign23710_e26423_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) && (var_guard436 == 0.0)) {
        let assign23710_e26421: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign23710_e26421, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign23710_e26423;
        var_umaxbeforelimiting_dn5 = assign23710_e26423_d_n5;
        var_umaxbeforelimiting_dn6 = assign23710_e26423_d_n6;
        var_umaxbeforelimiting_dn7 = assign23710_e26423_d_n7;
        var_umaxbeforelimiting_dn8 = assign23710_e26423_d_n8;

        let (assign23720_e26444, assign23720_e26444_d_n5, assign23720_e26444_d_n6, assign23720_e26444_d_n7, assign23720_e26444_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) && (var_guard436 == 0.0)) {
        let assign23720_e26435: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign23720_e26438: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign23720_e26440: f64 = (assign23720_e26438 + 1.0);
        let assign23720_e26441: f64 = (assign23720_e26435 / assign23720_e26440);
        let assign23720_e26442: f64 = (assign23720_e26441).sqrt();
        (assign23720_e26442, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign23720_e26440) - (assign23720_e26435 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign23720_e26440 * assign23720_e26440)) / (2.0 * assign23720_e26442)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign23720_e26440) - (assign23720_e26435 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign23720_e26440 * assign23720_e26440)) / (2.0 * assign23720_e26442)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign23720_e26440) - (assign23720_e26435 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign23720_e26440 * assign23720_e26440)) / (2.0 * assign23720_e26442)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign23720_e26440) - (assign23720_e26435 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign23720_e26440 * assign23720_e26440)) / (2.0 * assign23720_e26442)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign23720_e26444;
        var_umax_dn5 = assign23720_e26444_d_n5;
        var_umax_dn6 = assign23720_e26444_d_n6;
        var_umax_dn7 = assign23720_e26444_d_n7;
        var_umax_dn8 = assign23720_e26444_d_n8;

        let (assign23730_e26457, assign23730_e26457_d_n5, assign23730_e26457_d_n6, assign23730_e26457_d_n7, assign23730_e26457_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) && (var_guard436 == 0.0)) {
        let assign23730_e26455: f64 = (var_umax).sqrt();
        (assign23730_e26455, (var_umax_dn5 / (2.0 * assign23730_e26455)), (var_umax_dn6 / (2.0 * assign23730_e26455)), (var_umax_dn7 / (2.0 * assign23730_e26455)), (var_umax_dn8 / (2.0 * assign23730_e26455)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign23730_e26457;
        var_sqrtumax_dn5 = assign23730_e26457_d_n5;
        var_sqrtumax_dn6 = assign23730_e26457_d_n6;
        var_sqrtumax_dn7 = assign23730_e26457_d_n7;
        var_sqrtumax_dn8 = assign23730_e26457_d_n8;

        let (assign23740_e26471, assign23740_e26471_d_n5, assign23740_e26471_d_n6, assign23740_e26471_d_n7, assign23740_e26471_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) && (var_guard436 == 0.0)) {
        let assign23740_e26469: f64 = (var_umax * var_sqrtumax);
        (assign23740_e26469, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign23740_e26471;
        var_umaxpoweronepointfive_dn5 = assign23740_e26471_d_n5;
        var_umaxpoweronepointfive_dn6 = assign23740_e26471_d_n6;
        var_umaxpoweronepointfive_dn7 = assign23740_e26471_d_n7;
        var_umaxpoweronepointfive_dn8 = assign23740_e26471_d_n8;

        let assign23750_e26473: f64 = (-p.p825);
        let assign23750_e26475: f64 = (assign23750_e26473 * var_one_over_one_minus_psti);
        let assign23750_e26477: f64 = (-1.0);
        let assign23750_e26478: f64 = if assign23750_e26475 == assign23750_e26477 { 1.0 } else { 0.0 };
        var_guard437 = assign23750_e26478;

        let (assign23760_e26498, assign23760_e26498_d_n5, assign23760_e26498_d_n6, assign23760_e26498_d_n7, assign23760_e26498_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) && (var_guard436 == 0.0)) && (var_guard437 != 0.0)) {
        let assign23760_e26494: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign23760_e26495: f64 = (1.0 + assign23760_e26494);
        let assign23760_e26496: f64 = (1.0 / assign23760_e26495);
        (assign23760_e26496, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign23760_e26495 * assign23760_e26495))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign23760_e26495 * assign23760_e26495))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign23760_e26495 * assign23760_e26495))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign23760_e26495 * assign23760_e26495))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign23760_e26498;
        var_wgamma_dn5 = assign23760_e26498_d_n5;
        var_wgamma_dn6 = assign23760_e26498_d_n6;
        var_wgamma_dn7 = assign23760_e26498_d_n7;
        var_wgamma_dn8 = assign23760_e26498_d_n8;

        let (assign23770_e26522, assign23770_e26522_d_n5, assign23770_e26522_d_n6, assign23770_e26522_d_n7, assign23770_e26522_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) && (var_guard436 == 0.0)) && (var_guard437 == 0.0)) {
        let assign23770_e26514: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign23770_e26515: f64 = (1.0 + assign23770_e26514);
        let assign23770_e26517: f64 = (-p.p825);
        let assign23770_e26519: f64 = (assign23770_e26517 * var_one_over_one_minus_psti);
        let assign23770_e26520: f64 = (assign23770_e26515).powf(assign23770_e26519);
        (assign23770_e26520, if 0.0 == 0.0 && ((assign23770_e26519) as f64).is_finite() && ((assign23770_e26519) as f64).fract() == 0.0 { if assign23770_e26519 == 0.0 { 0.0 } else { (assign23770_e26519 * ((assign23770_e26515).powf(assign23770_e26519 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign23770_e26520 * (assign23770_e26519 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign23770_e26515))) }, if 0.0 == 0.0 && ((assign23770_e26519) as f64).is_finite() && ((assign23770_e26519) as f64).fract() == 0.0 { if assign23770_e26519 == 0.0 { 0.0 } else { (assign23770_e26519 * ((assign23770_e26515).powf(assign23770_e26519 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign23770_e26520 * (assign23770_e26519 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign23770_e26515))) }, if 0.0 == 0.0 && ((assign23770_e26519) as f64).is_finite() && ((assign23770_e26519) as f64).fract() == 0.0 { if assign23770_e26519 == 0.0 { 0.0 } else { (assign23770_e26519 * ((assign23770_e26515).powf(assign23770_e26519 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign23770_e26520 * (assign23770_e26519 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign23770_e26515))) }, if 0.0 == 0.0 && ((assign23770_e26519) as f64).is_finite() && ((assign23770_e26519) as f64).fract() == 0.0 { if assign23770_e26519 == 0.0 { 0.0 } else { (assign23770_e26519 * ((assign23770_e26515).powf(assign23770_e26519 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign23770_e26520 * (assign23770_e26519 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign23770_e26515))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign23770_e26522;
        var_wgamma_dn5 = assign23770_e26522_d_n5;
        var_wgamma_dn6 = assign23770_e26522_d_n6;
        var_wgamma_dn7 = assign23770_e26522_d_n7;
        var_wgamma_dn8 = assign23770_e26522_d_n8;

        let (assign23780_e26540, assign23780_e26540_d_n5, assign23780_e26540_d_n6, assign23780_e26540_d_n7, assign23780_e26540_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) && (var_guard436 == 0.0)) {
        let assign23780_e26534: f64 = (var_wsrh * var_wgamma);
        let assign23780_e26537: f64 = (var_wsrh + var_wgamma);
        let assign23780_e26538: f64 = (assign23780_e26534 / assign23780_e26537);
        (assign23780_e26538, ((((var_wsrh * var_wgamma_dn5) * assign23780_e26537) - (assign23780_e26534 * var_wgamma_dn5)) / (assign23780_e26537 * assign23780_e26537)), ((((var_wsrh * var_wgamma_dn6) * assign23780_e26537) - (assign23780_e26534 * var_wgamma_dn6)) / (assign23780_e26537 * assign23780_e26537)), ((((var_wsrh * var_wgamma_dn7) * assign23780_e26537) - (assign23780_e26534 * var_wgamma_dn7)) / (assign23780_e26537 * assign23780_e26537)), ((((var_wsrh * var_wgamma_dn8) * assign23780_e26537) - (assign23780_e26534 * var_wgamma_dn8)) / (assign23780_e26537 * assign23780_e26537)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign23780_e26540;
        var_wtat_dn5 = assign23780_e26540_d_n5;
        var_wtat_dn6 = assign23780_e26540_d_n6;
        var_wtat_dn7 = assign23780_e26540_d_n7;
        var_wtat_dn8 = assign23780_e26540_d_n8;

        let (assign23790_e26557, assign23790_e26557_d_n5, assign23790_e26557_d_n6, assign23790_e26557_d_n7, assign23790_e26557_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) && (var_guard436 == 0.0)) {
        let assign23790_e26553: f64 = (var_btat / var_sqrtumax);
        let assign23790_e26554: f64 = (0.375 * assign23790_e26553);
        let assign23790_e26555: f64 = (assign23790_e26554).sqrt();
        (assign23790_e26555, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign23790_e26555)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign23790_e26555)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign23790_e26555)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign23790_e26555)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign23790_e26557;
        var_ktat_dn5 = assign23790_e26557_d_n5;
        var_ktat_dn6 = assign23790_e26557_d_n6;
        var_ktat_dn7 = assign23790_e26557_d_n7;
        var_ktat_dn8 = assign23790_e26557_d_n8;

        let (assign23800_e26575, assign23800_e26575_d_n5, assign23800_e26575_d_n6, assign23800_e26575_d_n7, assign23800_e26575_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) && (var_guard436 == 0.0)) {
        let assign23800_e26570: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign23800_e26571: f64 = (2.0 * assign23800_e26570);
        let assign23800_e26573: f64 = (assign23800_e26571 - var_umax);
        (assign23800_e26573, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign23800_e26575;
        var_ltat_dn5 = assign23800_e26575_d_n5;
        var_ltat_dn6 = assign23800_e26575_d_n6;
        var_ltat_dn7 = assign23800_e26575_d_n7;
        var_ltat_dn8 = assign23800_e26575_d_n8;

        let (assign23810_e26601, assign23810_e26601_d_n5, assign23810_e26601_d_n6, assign23810_e26601_d_n7, assign23810_e26601_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) && (var_guard436 == 0.0)) {
        let assign23810_e26587: f64 = (var_atatsti * var_twoatatoverthreebtat);
        let assign23810_e26589: f64 = (assign23810_e26587 * var_sqrtumax);
        let assign23810_e26592: f64 = (var_atatsti * var_umax);
        let assign23810_e26593: f64 = (assign23810_e26589 - assign23810_e26592);
        let assign23810_e26597: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign23810_e26598: f64 = (0.5 * assign23810_e26597);
        let assign23810_e26599: f64 = (assign23810_e26593 + assign23810_e26598);
        (assign23810_e26599, (((((var_atatsti * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign23810_e26587 * var_sqrtumax_dn5)) - (var_atatsti * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatsti * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign23810_e26587 * var_sqrtumax_dn6)) - (var_atatsti * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatsti * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign23810_e26587 * var_sqrtumax_dn7)) - (var_atatsti * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatsti * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign23810_e26587 * var_sqrtumax_dn8)) - (var_atatsti * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign23810_e26601;
        var_mtat_dn5 = assign23810_e26601_d_n5;
        var_mtat_dn6 = assign23810_e26601_d_n6;
        var_mtat_dn7 = assign23810_e26601_d_n7;
        var_mtat_dn8 = assign23810_e26601_d_n8;

        let (assign23820_e26617, assign23820_e26617_d_n5, assign23820_e26617_d_n6, assign23820_e26617_d_n7, assign23820_e26617_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) && (var_guard436 == 0.0)) {
        let assign23820_e26613: f64 = (var_ltat - 1.0);
        let assign23820_e26615: f64 = (assign23820_e26613 * var_ktat);
        (assign23820_e26615, ((var_ltat_dn5 * var_ktat) + (assign23820_e26613 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign23820_e26613 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign23820_e26613 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign23820_e26613 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign23820_e26617;
        var_xerfc_dn5 = assign23820_e26617_d_n5;
        var_xerfc_dn6 = assign23820_e26617_d_n6;
        var_xerfc_dn7 = assign23820_e26617_d_n7;
        var_xerfc_dn8 = assign23820_e26617_d_n8;

        let (assign23830_e26631, assign23830_e26631_d_n5, assign23830_e26631_d_n6, assign23830_e26631_d_n7, assign23830_e26631_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) && (var_guard436 == 0.0)) {
        let assign23830_e26629: f64 = (var_xerfc * var_xerfc);
        (assign23830_e26629, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign23830_e26631;
        var_ysq_dn5 = assign23830_e26631_d_n5;
        var_ysq_dn6 = assign23830_e26631_d_n6;
        var_ysq_dn7 = assign23830_e26631_d_n7;
        var_ysq_dn8 = assign23830_e26631_d_n8;

        let assign23840_e26634: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard438 = assign23840_e26634;

        let (assign23850_e26654, assign23850_e26654_d_n5, assign23850_e26654_d_n6, assign23850_e26654_d_n7, assign23850_e26654_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) && (var_guard436 == 0.0)) && (var_guard438 != 0.0)) {
        let assign23850_e26650: f64 = (var_perfc * var_xerfc);
        let assign23850_e26651: f64 = (1.0 + assign23850_e26650);
        let assign23850_e26652: f64 = (1.0 / assign23850_e26651);
        (assign23850_e26652, (-((var_perfc * var_xerfc_dn5) / (assign23850_e26651 * assign23850_e26651))), (-((var_perfc * var_xerfc_dn6) / (assign23850_e26651 * assign23850_e26651))), (-((var_perfc * var_xerfc_dn7) / (assign23850_e26651 * assign23850_e26651))), (-((var_perfc * var_xerfc_dn8) / (assign23850_e26651 * assign23850_e26651))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign23850_e26654;
        var_terfc_dn5 = assign23850_e26654_d_n5;
        var_terfc_dn6 = assign23850_e26654_d_n6;
        var_terfc_dn7 = assign23850_e26654_d_n7;
        var_terfc_dn8 = assign23850_e26654_d_n8;

        let (assign23860_e26675, assign23860_e26675_d_n5, assign23860_e26675_d_n6, assign23860_e26675_d_n7, assign23860_e26675_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) && (var_guard436 == 0.0)) && (var_guard438 == 0.0)) {
        let assign23860_e26671: f64 = (var_perfc * var_xerfc);
        let assign23860_e26672: f64 = (1.0 - assign23860_e26671);
        let assign23860_e26673: f64 = (1.0 / assign23860_e26672);
        (assign23860_e26673, (-((-(var_perfc * var_xerfc_dn5)) / (assign23860_e26672 * assign23860_e26672))), (-((-(var_perfc * var_xerfc_dn6)) / (assign23860_e26672 * assign23860_e26672))), (-((-(var_perfc * var_xerfc_dn7)) / (assign23860_e26672 * assign23860_e26672))), (-((-(var_perfc * var_xerfc_dn8)) / (assign23860_e26672 * assign23860_e26672))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign23860_e26675;
        var_terfc_dn5 = assign23860_e26675_d_n5;
        var_terfc_dn6 = assign23860_e26675_d_n6;
        var_terfc_dn7 = assign23860_e26675_d_n7;
        var_terfc_dn8 = assign23860_e26675_d_n8;

        let assign23870_e26677: f64 = (-var_ysq);
        let assign23870_e26679: f64 = (assign23870_e26677 + var_mtat);
        let assign23870_e26681: f64 = (-230.25850929940458);
        let assign23870_e26682: f64 = if assign23870_e26679 > assign23870_e26681 { 1.0 } else { 0.0 };
        var_guard439 = assign23870_e26682;

        let (assign23880_e26700, assign23880_e26700_d_n5, assign23880_e26700_d_n6, assign23880_e26700_d_n7, assign23880_e26700_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) && (var_guard436 == 0.0)) && (var_guard439 != 0.0)) {
        let assign23880_e26695: f64 = (-var_ysq);
        let assign23880_e26697: f64 = (assign23880_e26695 + var_mtat);
        let assign23880_e26698: f64 = (assign23880_e26697).exp();
        (assign23880_e26698, (assign23880_e26698 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign23880_e26698 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign23880_e26698 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign23880_e26698 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign23880_e26700;
        var_tmp_dn5 = assign23880_e26700_d_n5;
        var_tmp_dn6 = assign23880_e26700_d_n6;
        var_tmp_dn7 = assign23880_e26700_d_n7;
        var_tmp_dn8 = assign23880_e26700_d_n8;

        *var_asrh_slot = var_asrh;
        *var_asrh_dn5_slot = var_asrh_dn5;
        *var_asrh_dn6_slot = var_asrh_dn6;
        *var_asrh_dn7_slot = var_asrh_dn7;
        *var_asrh_dn8_slot = var_asrh_dn8;
        *var_btat_slot = var_btat;
        *var_btat_dn5_slot = var_btat_dn5;
        *var_btat_dn6_slot = var_btat_dn6;
        *var_btat_dn7_slot = var_btat_dn7;
        *var_btat_dn8_slot = var_btat_dn8;
        *var_dwsrh_slot = var_dwsrh;
        *var_guard432_slot = var_guard432;
        *var_guard433_slot = var_guard433;
        *var_guard434_slot = var_guard434;
        *var_guard435_slot = var_guard435;
        *var_guard436_slot = var_guard436;
        *var_guard437_slot = var_guard437;
        *var_guard438_slot = var_guard438;
        *var_guard439_slot = var_guard439;
        *var_id__blk213_slot = var_id__blk213;
        *var_ijunbot_slot = var_ijunbot;
        *var_ijunbot_dn5_slot = var_ijunbot_dn5;
        *var_ijunbot_dn6_slot = var_ijunbot_dn6;
        *var_ijunbot_dn7_slot = var_ijunbot_dn7;
        *var_ijunbot_dn8_slot = var_ijunbot_dn8;
        *var_ijunsti_slot = var_ijunsti;
        *var_ijunsti_dn5_slot = var_ijunsti_dn5;
        *var_ijunsti_dn6_slot = var_ijunsti_dn6;
        *var_ijunsti_dn7_slot = var_ijunsti_dn7;
        *var_ijunsti_dn8_slot = var_ijunsti_dn8;
        *var_isrh_slot = var_isrh;
        *var_isrh_dn5_slot = var_isrh_dn5;
        *var_isrh_dn6_slot = var_isrh_dn6;
        *var_isrh_dn7_slot = var_isrh_dn7;
        *var_isrh_dn8_slot = var_isrh_dn8;
        *var_itat_slot = var_itat;
        *var_itat_dn5_slot = var_itat_dn5;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_ktat_slot = var_ktat;
        *var_ktat_dn5_slot = var_ktat_dn5;
        *var_ktat_dn6_slot = var_ktat_dn6;
        *var_ktat_dn7_slot = var_ktat_dn7;
        *var_ktat_dn8_slot = var_ktat_dn8;
        *var_ltat_slot = var_ltat;
        *var_ltat_dn5_slot = var_ltat_dn5;
        *var_ltat_dn6_slot = var_ltat_dn6;
        *var_ltat_dn7_slot = var_ltat_dn7;
        *var_ltat_dn8_slot = var_ltat_dn8;
        *var_mtat_slot = var_mtat;
        *var_mtat_dn5_slot = var_mtat_dn5;
        *var_mtat_dn6_slot = var_mtat_dn6;
        *var_mtat_dn7_slot = var_mtat_dn7;
        *var_mtat_dn8_slot = var_mtat_dn8;
        *var_sqrtumax_slot = var_sqrtumax;
        *var_sqrtumax_dn5_slot = var_sqrtumax_dn5;
        *var_sqrtumax_dn6_slot = var_sqrtumax_dn6;
        *var_sqrtumax_dn7_slot = var_sqrtumax_dn7;
        *var_sqrtumax_dn8_slot = var_sqrtumax_dn8;
        *var_terfc_slot = var_terfc;
        *var_terfc_dn5_slot = var_terfc_dn5;
        *var_terfc_dn6_slot = var_terfc_dn6;
        *var_terfc_dn7_slot = var_terfc_dn7;
        *var_terfc_dn8_slot = var_terfc_dn8;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn5_slot = var_tmp_dn5;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_twoatatoverthreebtat_slot = var_twoatatoverthreebtat;
        *var_twoatatoverthreebtat_dn5_slot = var_twoatatoverthreebtat_dn5;
        *var_twoatatoverthreebtat_dn6_slot = var_twoatatoverthreebtat_dn6;
        *var_twoatatoverthreebtat_dn7_slot = var_twoatatoverthreebtat_dn7;
        *var_twoatatoverthreebtat_dn8_slot = var_twoatatoverthreebtat_dn8;
        *var_umax_slot = var_umax;
        *var_umax_dn5_slot = var_umax_dn5;
        *var_umax_dn6_slot = var_umax_dn6;
        *var_umax_dn7_slot = var_umax_dn7;
        *var_umax_dn8_slot = var_umax_dn8;
        *var_umaxbeforelimiting_slot = var_umaxbeforelimiting;
        *var_umaxbeforelimiting_dn5_slot = var_umaxbeforelimiting_dn5;
        *var_umaxbeforelimiting_dn6_slot = var_umaxbeforelimiting_dn6;
        *var_umaxbeforelimiting_dn7_slot = var_umaxbeforelimiting_dn7;
        *var_umaxbeforelimiting_dn8_slot = var_umaxbeforelimiting_dn8;
        *var_umaxpoweronepointfive_slot = var_umaxpoweronepointfive;
        *var_umaxpoweronepointfive_dn5_slot = var_umaxpoweronepointfive_dn5;
        *var_umaxpoweronepointfive_dn6_slot = var_umaxpoweronepointfive_dn6;
        *var_umaxpoweronepointfive_dn7_slot = var_umaxpoweronepointfive_dn7;
        *var_umaxpoweronepointfive_dn8_slot = var_umaxpoweronepointfive_dn8;
        *var_vbi_minus_vjsrh_slot = var_vbi_minus_vjsrh;
        *var_wdep_slot = var_wdep;
        *var_wdep_dn5_slot = var_wdep_dn5;
        *var_wdep_dn6_slot = var_wdep_dn6;
        *var_wdep_dn7_slot = var_wdep_dn7;
        *var_wdep_dn8_slot = var_wdep_dn8;
        *var_wgamma_slot = var_wgamma;
        *var_wgamma_dn5_slot = var_wgamma_dn5;
        *var_wgamma_dn6_slot = var_wgamma_dn6;
        *var_wgamma_dn7_slot = var_wgamma_dn7;
        *var_wgamma_dn8_slot = var_wgamma_dn8;
        *var_wsrh_slot = var_wsrh;
        *var_wsrhstep_slot = var_wsrhstep;
        *var_wtat_slot = var_wtat;
        *var_wtat_dn5_slot = var_wtat_dn5;
        *var_wtat_dn6_slot = var_wtat_dn6;
        *var_wtat_dn7_slot = var_wtat_dn7;
        *var_wtat_dn8_slot = var_wtat_dn8;
        *var_xerfc_slot = var_xerfc;
        *var_xerfc_dn5_slot = var_xerfc_dn5;
        *var_xerfc_dn6_slot = var_xerfc_dn6;
        *var_xerfc_dn7_slot = var_xerfc_dn7;
        *var_xerfc_dn8_slot = var_xerfc_dn8;
        *var_ysq_slot = var_ysq;
        *var_ysq_dn5_slot = var_ysq_dn5;
        *var_ysq_dn6_slot = var_ysq_dn6;
        *var_ysq_dn7_slot = var_ysq_dn7;
        *var_ysq_dn8_slot = var_ysq_dn8;
    }

    pub(super) fn stamp_transient_block_44(
        p: &Parameters,
        var_alphaav: f64,
        var_asrh: f64,
        var_asrh_dn5: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_atatsti: f64,
        var_berfc: f64,
        var_cerfc: f64,
        var_fbbtsti: f64,
        var_fstopsti: f64,
        var_guard176: f64,
        var_guard193: f64,
        var_guard432: f64,
        var_guard436: f64,
        var_guard439: f64,
        var_idmult: f64,
        var_idsatgat: f64,
        var_ktat: f64,
        var_ktat_dn5: f64,
        var_ktat_dn6: f64,
        var_ktat_dn7: f64,
        var_ktat_dn8: f64,
        var_lgsource_i: f64,
        var_mtat: f64,
        var_mtat_dn5: f64,
        var_mtat_dn6: f64,
        var_mtat_dn7: f64,
        var_mtat_dn8: f64,
        var_one_over_one_minus_psti: f64,
        var_slopesti: f64,
        var_terfc: f64,
        var_terfc_dn5: f64,
        var_terfc_dn6: f64,
        var_terfc_dn7: f64,
        var_terfc_dn8: f64,
        var_v4: f64,
        var_vav: f64,
        var_vbbt: f64,
        var_vbigat: f64,
        var_vbirstiinv: f64,
        var_vbrinvsti: f64,
        var_vjsrh: f64,
        var_wdepnulrinvsti: f64,
        var_wtat: f64,
        var_wtat_dn5: f64,
        var_wtat_dn6: f64,
        var_wtat_dn7: f64,
        var_wtat_dn8: f64,
        var_xerfc: f64,
        var_ysq: f64,
        var_ysq_dn5: f64,
        var_ysq_dn6: f64,
        var_ysq_dn7: f64,
        var_ysq_dn8: f64,
        var_erfcpos_slot: &mut f64,
        var_erfcpos_dn5_slot: &mut f64,
        var_erfcpos_dn6_slot: &mut f64,
        var_erfcpos_dn7_slot: &mut f64,
        var_erfcpos_dn8_slot: &mut f64,
        var_erfctimesexpmtat_slot: &mut f64,
        var_erfctimesexpmtat_dn5_slot: &mut f64,
        var_erfctimesexpmtat_dn6_slot: &mut f64,
        var_erfctimesexpmtat_dn7_slot: &mut f64,
        var_erfctimesexpmtat_dn8_slot: &mut f64,
        var_fbreakdown_slot: &mut f64,
        var_fbreakdown_dn5_slot: &mut f64,
        var_fbreakdown_dn6_slot: &mut f64,
        var_fbreakdown_dn7_slot: &mut f64,
        var_fbreakdown_dn8_slot: &mut f64,
        var_fmaxr_slot: &mut f64,
        var_fmaxr_dn5_slot: &mut f64,
        var_fmaxr_dn6_slot: &mut f64,
        var_fmaxr_dn7_slot: &mut f64,
        var_fmaxr_dn8_slot: &mut f64,
        var_gammamax_slot: &mut f64,
        var_gammamax_dn5_slot: &mut f64,
        var_gammamax_dn6_slot: &mut f64,
        var_gammamax_dn7_slot: &mut f64,
        var_gammamax_dn8_slot: &mut f64,
        var_guard440_slot: &mut f64,
        var_guard441_slot: &mut f64,
        var_guard442_slot: &mut f64,
        var_guard443_slot: &mut f64,
        var_guard444_slot: &mut f64,
        var_guard445_slot: &mut f64,
        var_guard446_slot: &mut f64,
        var_guard447_slot: &mut f64,
        var_guard448_slot: &mut f64,
        var_guard449_slot: &mut f64,
        var_guard450_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn5_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
        var_id__blk213_slot: &mut f64,
        var_ijungat_slot: &mut f64,
        var_ijungat_dn5_slot: &mut f64,
        var_ijungat_dn6_slot: &mut f64,
        var_ijungat_dn7_slot: &mut f64,
        var_ijungat_dn8_slot: &mut f64,
        var_ijunsti_slot: &mut f64,
        var_ijunsti_dn5_slot: &mut f64,
        var_ijunsti_dn6_slot: &mut f64,
        var_ijunsti_dn7_slot: &mut f64,
        var_ijunsti_dn8_slot: &mut f64,
        var_isrh_slot: &mut f64,
        var_isrh_dn5_slot: &mut f64,
        var_isrh_dn6_slot: &mut f64,
        var_isrh_dn7_slot: &mut f64,
        var_isrh_dn8_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn5_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn5_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_vbi_minus_vjsrh_slot: &mut f64,
    ) {
        let mut var_erfcpos: f64 = *var_erfcpos_slot;
        let mut var_erfcpos_dn5: f64 = *var_erfcpos_dn5_slot;
        let mut var_erfcpos_dn6: f64 = *var_erfcpos_dn6_slot;
        let mut var_erfcpos_dn7: f64 = *var_erfcpos_dn7_slot;
        let mut var_erfcpos_dn8: f64 = *var_erfcpos_dn8_slot;
        let mut var_erfctimesexpmtat: f64 = *var_erfctimesexpmtat_slot;
        let mut var_erfctimesexpmtat_dn5: f64 = *var_erfctimesexpmtat_dn5_slot;
        let mut var_erfctimesexpmtat_dn6: f64 = *var_erfctimesexpmtat_dn6_slot;
        let mut var_erfctimesexpmtat_dn7: f64 = *var_erfctimesexpmtat_dn7_slot;
        let mut var_erfctimesexpmtat_dn8: f64 = *var_erfctimesexpmtat_dn8_slot;
        let mut var_fbreakdown: f64 = *var_fbreakdown_slot;
        let mut var_fbreakdown_dn5: f64 = *var_fbreakdown_dn5_slot;
        let mut var_fbreakdown_dn6: f64 = *var_fbreakdown_dn6_slot;
        let mut var_fbreakdown_dn7: f64 = *var_fbreakdown_dn7_slot;
        let mut var_fbreakdown_dn8: f64 = *var_fbreakdown_dn8_slot;
        let mut var_fmaxr: f64 = *var_fmaxr_slot;
        let mut var_fmaxr_dn5: f64 = *var_fmaxr_dn5_slot;
        let mut var_fmaxr_dn6: f64 = *var_fmaxr_dn6_slot;
        let mut var_fmaxr_dn7: f64 = *var_fmaxr_dn7_slot;
        let mut var_fmaxr_dn8: f64 = *var_fmaxr_dn8_slot;
        let mut var_gammamax: f64 = *var_gammamax_slot;
        let mut var_gammamax_dn5: f64 = *var_gammamax_dn5_slot;
        let mut var_gammamax_dn6: f64 = *var_gammamax_dn6_slot;
        let mut var_gammamax_dn7: f64 = *var_gammamax_dn7_slot;
        let mut var_gammamax_dn8: f64 = *var_gammamax_dn8_slot;
        let mut var_guard440: f64 = *var_guard440_slot;
        let mut var_guard441: f64 = *var_guard441_slot;
        let mut var_guard442: f64 = *var_guard442_slot;
        let mut var_guard443: f64 = *var_guard443_slot;
        let mut var_guard444: f64 = *var_guard444_slot;
        let mut var_guard445: f64 = *var_guard445_slot;
        let mut var_guard446: f64 = *var_guard446_slot;
        let mut var_guard447: f64 = *var_guard447_slot;
        let mut var_guard448: f64 = *var_guard448_slot;
        let mut var_guard449: f64 = *var_guard449_slot;
        let mut var_guard450: f64 = *var_guard450_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn5: f64 = *var_ibbt_dn5_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
        let mut var_id__blk213: f64 = *var_id__blk213_slot;
        let mut var_ijungat: f64 = *var_ijungat_slot;
        let mut var_ijungat_dn5: f64 = *var_ijungat_dn5_slot;
        let mut var_ijungat_dn6: f64 = *var_ijungat_dn6_slot;
        let mut var_ijungat_dn7: f64 = *var_ijungat_dn7_slot;
        let mut var_ijungat_dn8: f64 = *var_ijungat_dn8_slot;
        let mut var_ijunsti: f64 = *var_ijunsti_slot;
        let mut var_ijunsti_dn5: f64 = *var_ijunsti_dn5_slot;
        let mut var_ijunsti_dn6: f64 = *var_ijunsti_dn6_slot;
        let mut var_ijunsti_dn7: f64 = *var_ijunsti_dn7_slot;
        let mut var_ijunsti_dn8: f64 = *var_ijunsti_dn8_slot;
        let mut var_isrh: f64 = *var_isrh_slot;
        let mut var_isrh_dn5: f64 = *var_isrh_dn5_slot;
        let mut var_isrh_dn6: f64 = *var_isrh_dn6_slot;
        let mut var_isrh_dn7: f64 = *var_isrh_dn7_slot;
        let mut var_isrh_dn8: f64 = *var_isrh_dn8_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn5: f64 = *var_itat_dn5_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn5: f64 = *var_tmp_dn5_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_vbi_minus_vjsrh: f64 = *var_vbi_minus_vjsrh_slot;

        let (assign23890_e26749, assign23890_e26749_d_n5, assign23890_e26749_d_n6, assign23890_e26749_d_n7, assign23890_e26749_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) && (var_guard436 == 0.0)) && (var_guard439 == 0.0)) {
        let assign23890_e26716: f64 = (-230.25850929940458);
        let assign23890_e26718: f64 = (-var_ysq);
        let assign23890_e26720: f64 = (assign23890_e26718 + var_mtat);
        let assign23890_e26721: f64 = (assign23890_e26716 - assign23890_e26720);
        let assign23890_e26725: f64 = (-230.25850929940458);
        let assign23890_e26727: f64 = (-var_ysq);
        let assign23890_e26729: f64 = (assign23890_e26727 + var_mtat);
        let assign23890_e26730: f64 = (assign23890_e26725 - assign23890_e26729);
        let assign23890_e26733: f64 = (-230.25850929940458);
        let assign23890_e26735: f64 = (-var_ysq);
        let assign23890_e26737: f64 = (assign23890_e26735 + var_mtat);
        let assign23890_e26738: f64 = (assign23890_e26733 - assign23890_e26737);
        let assign23890_e26740: f64 = (assign23890_e26738 * 0.3333333333333333);
        let assign23890_e26741: f64 = (1.0 + assign23890_e26740);
        let assign23890_e26742: f64 = (assign23890_e26730 * assign23890_e26741);
        let assign23890_e26743: f64 = (0.5 * assign23890_e26742);
        let assign23890_e26744: f64 = (1.0 + assign23890_e26743);
        let assign23890_e26745: f64 = (assign23890_e26721 * assign23890_e26744);
        let assign23890_e26746: f64 = (1.0 + assign23890_e26745);
        let assign23890_e26747: f64 = (1e-100 / assign23890_e26746);
        (assign23890_e26747, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign23890_e26744) + (assign23890_e26721 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign23890_e26741) + (assign23890_e26730 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign23890_e26746 * assign23890_e26746))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign23890_e26744) + (assign23890_e26721 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign23890_e26741) + (assign23890_e26730 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign23890_e26746 * assign23890_e26746))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign23890_e26744) + (assign23890_e26721 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign23890_e26741) + (assign23890_e26730 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign23890_e26746 * assign23890_e26746))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign23890_e26744) + (assign23890_e26721 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign23890_e26741) + (assign23890_e26730 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign23890_e26746 * assign23890_e26746))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign23890_e26749;
        var_tmp_dn5 = assign23890_e26749_d_n5;
        var_tmp_dn6 = assign23890_e26749_d_n6;
        var_tmp_dn7 = assign23890_e26749_d_n7;
        var_tmp_dn8 = assign23890_e26749_d_n8;

        let (assign23900_e26779, assign23900_e26779_d_n5, assign23900_e26779_d_n6, assign23900_e26779_d_n7, assign23900_e26779_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) && (var_guard436 == 0.0)) {
        let assign23900_e26761: f64 = (0.29214664 * var_terfc);
        let assign23900_e26765: f64 = (var_terfc * var_terfc);
        let assign23900_e26766: f64 = (var_berfc * assign23900_e26765);
        let assign23900_e26767: f64 = (assign23900_e26761 + assign23900_e26766);
        let assign23900_e26771: f64 = (var_terfc * var_terfc);
        let assign23900_e26773: f64 = (assign23900_e26771 * var_terfc);
        let assign23900_e26774: f64 = (var_cerfc * assign23900_e26773);
        let assign23900_e26775: f64 = (assign23900_e26767 + assign23900_e26774);
        let assign23900_e26777: f64 = (assign23900_e26775 * var_tmp);
        (assign23900_e26777, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign23900_e26771 * var_terfc_dn5)))) * var_tmp) + (assign23900_e26775 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign23900_e26771 * var_terfc_dn6)))) * var_tmp) + (assign23900_e26775 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign23900_e26771 * var_terfc_dn7)))) * var_tmp) + (assign23900_e26775 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign23900_e26771 * var_terfc_dn8)))) * var_tmp) + (assign23900_e26775 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign23900_e26779;
        var_erfcpos_dn5 = assign23900_e26779_d_n5;
        var_erfcpos_dn6 = assign23900_e26779_d_n6;
        var_erfcpos_dn7 = assign23900_e26779_d_n7;
        var_erfcpos_dn8 = assign23900_e26779_d_n8;

        let assign23910_e26782: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard440 = assign23910_e26782;

        let (assign23920_e26796, assign23920_e26796_d_n5, assign23920_e26796_d_n6, assign23920_e26796_d_n7, assign23920_e26796_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) && (var_guard436 == 0.0)) && (var_guard440 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign23920_e26796;
        var_erfctimesexpmtat_dn5 = assign23920_e26796_d_n5;
        var_erfctimesexpmtat_dn6 = assign23920_e26796_d_n6;
        var_erfctimesexpmtat_dn7 = assign23920_e26796_d_n7;
        var_erfctimesexpmtat_dn8 = assign23920_e26796_d_n8;

        let assign23930_e26799: f64 = (-230.25850929940458);
        let assign23930_e26800: f64 = if var_mtat > assign23930_e26799 { 1.0 } else { 0.0 };
        var_guard441 = assign23930_e26800;

        let (assign23940_e26818, assign23940_e26818_d_n5, assign23940_e26818_d_n6, assign23940_e26818_d_n7, assign23940_e26818_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) && (var_guard436 == 0.0)) && (var_guard440 == 0.0)) && (var_guard441 != 0.0)) {
        let assign23940_e26816: f64 = (var_mtat).exp();
        (assign23940_e26816, (assign23940_e26816 * var_mtat_dn5), (assign23940_e26816 * var_mtat_dn6), (assign23940_e26816 * var_mtat_dn7), (assign23940_e26816 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign23940_e26818;
        var_tmp_dn5 = assign23940_e26818_d_n5;
        var_tmp_dn6 = assign23940_e26818_d_n6;
        var_tmp_dn7 = assign23940_e26818_d_n7;
        var_tmp_dn8 = assign23940_e26818_d_n8;

        let (assign23950_e26861, assign23950_e26861_d_n5, assign23950_e26861_d_n6, assign23950_e26861_d_n7, assign23950_e26861_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) && (var_guard436 == 0.0)) && (var_guard440 == 0.0)) && (var_guard441 == 0.0)) {
        let assign23950_e26837: f64 = (-230.25850929940458);
        let assign23950_e26839: f64 = (assign23950_e26837 - var_mtat);
        let assign23950_e26843: f64 = (-230.25850929940458);
        let assign23950_e26845: f64 = (assign23950_e26843 - var_mtat);
        let assign23950_e26848: f64 = (-230.25850929940458);
        let assign23950_e26850: f64 = (assign23950_e26848 - var_mtat);
        let assign23950_e26852: f64 = (assign23950_e26850 * 0.3333333333333333);
        let assign23950_e26853: f64 = (1.0 + assign23950_e26852);
        let assign23950_e26854: f64 = (assign23950_e26845 * assign23950_e26853);
        let assign23950_e26855: f64 = (0.5 * assign23950_e26854);
        let assign23950_e26856: f64 = (1.0 + assign23950_e26855);
        let assign23950_e26857: f64 = (assign23950_e26839 * assign23950_e26856);
        let assign23950_e26858: f64 = (1.0 + assign23950_e26857);
        let assign23950_e26859: f64 = (1e-100 / assign23950_e26858);
        (assign23950_e26859, (-((1e-100 * (((-var_mtat_dn5) * assign23950_e26856) + (assign23950_e26839 * (0.5 * (((-var_mtat_dn5) * assign23950_e26853) + (assign23950_e26845 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign23950_e26858 * assign23950_e26858))), (-((1e-100 * (((-var_mtat_dn6) * assign23950_e26856) + (assign23950_e26839 * (0.5 * (((-var_mtat_dn6) * assign23950_e26853) + (assign23950_e26845 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign23950_e26858 * assign23950_e26858))), (-((1e-100 * (((-var_mtat_dn7) * assign23950_e26856) + (assign23950_e26839 * (0.5 * (((-var_mtat_dn7) * assign23950_e26853) + (assign23950_e26845 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign23950_e26858 * assign23950_e26858))), (-((1e-100 * (((-var_mtat_dn8) * assign23950_e26856) + (assign23950_e26839 * (0.5 * (((-var_mtat_dn8) * assign23950_e26853) + (assign23950_e26845 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign23950_e26858 * assign23950_e26858))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign23950_e26861;
        var_tmp_dn5 = assign23950_e26861_d_n5;
        var_tmp_dn6 = assign23950_e26861_d_n6;
        var_tmp_dn7 = assign23950_e26861_d_n7;
        var_tmp_dn8 = assign23950_e26861_d_n8;

        let (assign23960_e26880, assign23960_e26880_d_n5, assign23960_e26880_d_n6, assign23960_e26880_d_n7, assign23960_e26880_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) && (var_guard436 == 0.0)) && (var_guard440 == 0.0)) {
        let assign23960_e26876: f64 = (2.0 * var_tmp);
        let assign23960_e26878: f64 = (assign23960_e26876 - var_erfcpos);
        (assign23960_e26878, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign23960_e26880;
        var_erfctimesexpmtat_dn5 = assign23960_e26880_d_n5;
        var_erfctimesexpmtat_dn6 = assign23960_e26880_d_n6;
        var_erfctimesexpmtat_dn7 = assign23960_e26880_d_n7;
        var_erfctimesexpmtat_dn8 = assign23960_e26880_d_n8;

        let (assign23970_e26900, assign23970_e26900_d_n5, assign23970_e26900_d_n6, assign23970_e26900_d_n7, assign23970_e26900_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) && (var_guard436 == 0.0)) {
        let assign23970_e26892: f64 = (1.772453850905516 * 0.5);
        let assign23970_e26895: f64 = (var_atatsti * var_erfctimesexpmtat);
        let assign23970_e26897: f64 = (assign23970_e26895 / var_ktat);
        let assign23970_e26898: f64 = (assign23970_e26892 * assign23970_e26897);
        (assign23970_e26898, (assign23970_e26892 * ((((var_atatsti * var_erfctimesexpmtat_dn5) * var_ktat) - (assign23970_e26895 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign23970_e26892 * ((((var_atatsti * var_erfctimesexpmtat_dn6) * var_ktat) - (assign23970_e26895 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign23970_e26892 * ((((var_atatsti * var_erfctimesexpmtat_dn7) * var_ktat) - (assign23970_e26895 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign23970_e26892 * ((((var_atatsti * var_erfctimesexpmtat_dn8) * var_ktat) - (assign23970_e26895 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign23970_e26900;
        var_gammamax_dn5 = assign23970_e26900_d_n5;
        var_gammamax_dn6 = assign23970_e26900_d_n6;
        var_gammamax_dn7 = assign23970_e26900_d_n7;
        var_gammamax_dn8 = assign23970_e26900_d_n8;

        let (assign23980_e26918, assign23980_e26918_d_n5, assign23980_e26918_d_n6, assign23980_e26918_d_n7, assign23980_e26918_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) && (var_guard436 == 0.0)) {
        let assign23980_e26913: f64 = (var_asrh * var_gammamax);
        let assign23980_e26915: f64 = (assign23980_e26913 * var_wtat);
        let assign23980_e26916: f64 = (p.p839 * assign23980_e26915);
        (assign23980_e26916, (p.p839 * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign23980_e26913 * var_wtat_dn5))), (p.p839 * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign23980_e26913 * var_wtat_dn6))), (p.p839 * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign23980_e26913 * var_wtat_dn7))), (p.p839 * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign23980_e26913 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign23980_e26918;
        var_itat_dn5 = assign23980_e26918_d_n5;
        var_itat_dn6 = assign23980_e26918_d_n6;
        var_itat_dn7 = assign23980_e26918_d_n7;
        var_itat_dn8 = assign23980_e26918_d_n8;

        let assign23990_e26921: f64 = if p.p845 == 0.0 { 1.0 } else { 0.0 };
        var_guard442 = assign23990_e26921;

        let (assign24000_e26932, assign24000_e26932_d_n5, assign24000_e26932_d_n6, assign24000_e26932_d_n7, assign24000_e26932_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) && (var_guard442 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign24000_e26932;
        var_ibbt_dn5 = assign24000_e26932_d_n5;
        var_ibbt_dn6 = assign24000_e26932_d_n6;
        var_ibbt_dn7 = assign24000_e26932_d_n7;
        var_ibbt_dn8 = assign24000_e26932_d_n8;

        let assign24010_e26935: f64 = if p.p825 == 0.5 { 1.0 } else { 0.0 };
        var_guard443 = assign24010_e26935;

        let (assign24020_e26954, assign24020_e26954_d_n5, assign24020_e26954_d_n6, assign24020_e26954_d_n7, assign24020_e26954_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) && (var_guard442 == 0.0)) && (var_guard443 != 0.0)) {
        let assign24020_e26949: f64 = (p.p822 - var_vbbt);
        let assign24020_e26951: f64 = (assign24020_e26949 * var_vbirstiinv);
        let assign24020_e26952: f64 = (assign24020_e26951).sqrt();
        (assign24020_e26952, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign24020_e26954;
        var_tmp_dn5 = assign24020_e26954_d_n5;
        var_tmp_dn6 = assign24020_e26954_d_n6;
        var_tmp_dn7 = assign24020_e26954_d_n7;
        var_tmp_dn8 = assign24020_e26954_d_n8;

        let (assign24030_e26975, assign24030_e26975_d_n5, assign24030_e26975_d_n6, assign24030_e26975_d_n7, assign24030_e26975_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) && (var_guard442 == 0.0)) && (var_guard443 == 0.0)) {
        let assign24030_e26969: f64 = (p.p822 - var_vbbt);
        let assign24030_e26971: f64 = (assign24030_e26969 * var_vbirstiinv);
        let assign24030_e26973: f64 = (assign24030_e26971).powf(p.p825);
        (assign24030_e26973, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign24030_e26975;
        var_tmp_dn5 = assign24030_e26975_d_n5;
        var_tmp_dn6 = assign24030_e26975_d_n6;
        var_tmp_dn7 = assign24030_e26975_d_n7;
        var_tmp_dn8 = assign24030_e26975_d_n8;

        let (assign24040_e26995, assign24040_e26995_d_n5, assign24040_e26995_d_n6, assign24040_e26995_d_n7, assign24040_e26995_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) && (var_guard442 == 0.0)) {
        let assign24040_e26988: f64 = (p.p822 - var_vbbt);
        let assign24040_e26990: f64 = (assign24040_e26988 * var_wdepnulrinvsti);
        let assign24040_e26992: f64 = (assign24040_e26990 / var_tmp);
        let assign24040_e26993: f64 = (var_one_over_one_minus_psti * assign24040_e26992);
        (assign24040_e26993, (var_one_over_one_minus_psti * (-((assign24040_e26990 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti * (-((assign24040_e26990 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti * (-((assign24040_e26990 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti * (-((assign24040_e26990 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign24040_e26995;
        var_fmaxr_dn5 = assign24040_e26995_d_n5;
        var_fmaxr_dn6 = assign24040_e26995_d_n6;
        var_fmaxr_dn7 = assign24040_e26995_d_n7;
        var_fmaxr_dn8 = assign24040_e26995_d_n8;

        let assign24050_e26997: f64 = (-var_fbbtsti);
        let assign24050_e26999: f64 = (assign24050_e26997 / var_fmaxr);
        let assign24050_e27000: f64 = (assign24050_e26999).abs();
        let assign24050_e27002: f64 = if assign24050_e27000 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard444 = assign24050_e27002;

        let (assign24060_e27020, assign24060_e27020_d_n5, assign24060_e27020_d_n6, assign24060_e27020_d_n7, assign24060_e27020_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) && (var_guard442 == 0.0)) && (var_guard444 != 0.0)) {
        let assign24060_e27015: f64 = (-var_fbbtsti);
        let assign24060_e27017: f64 = (assign24060_e27015 / var_fmaxr);
        let assign24060_e27018: f64 = (assign24060_e27017).exp();
        (assign24060_e27018, (assign24060_e27018 * (-((assign24060_e27015 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))), (assign24060_e27018 * (-((assign24060_e27015 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign24060_e27018 * (-((assign24060_e27015 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign24060_e27018 * (-((assign24060_e27015 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign24060_e27020;
        var_tmp_dn5 = assign24060_e27020_d_n5;
        var_tmp_dn6 = assign24060_e27020_d_n6;
        var_tmp_dn7 = assign24060_e27020_d_n7;
        var_tmp_dn8 = assign24060_e27020_d_n8;

        let assign24070_e27022: f64 = (-var_fbbtsti);
        let assign24070_e27024: f64 = (assign24070_e27022 / var_fmaxr);
        let assign24070_e27026: f64 = if assign24070_e27024 < 0.0 { 1.0 } else { 0.0 };
        var_guard445 = assign24070_e27026;

        let (assign24080_e27077, assign24080_e27077_d_n5, assign24080_e27077_d_n6, assign24080_e27077_d_n7, assign24080_e27077_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) && (var_guard442 == 0.0)) && (var_guard444 == 0.0)) && (var_guard445 != 0.0)) {
        let assign24080_e27044: f64 = (-230.25850929940458);
        let assign24080_e27046: f64 = (-var_fbbtsti);
        let assign24080_e27048: f64 = (assign24080_e27046 / var_fmaxr);
        let assign24080_e27049: f64 = (assign24080_e27044 - assign24080_e27048);
        let assign24080_e27053: f64 = (-230.25850929940458);
        let assign24080_e27055: f64 = (-var_fbbtsti);
        let assign24080_e27057: f64 = (assign24080_e27055 / var_fmaxr);
        let assign24080_e27058: f64 = (assign24080_e27053 - assign24080_e27057);
        let assign24080_e27061: f64 = (-230.25850929940458);
        let assign24080_e27063: f64 = (-var_fbbtsti);
        let assign24080_e27065: f64 = (assign24080_e27063 / var_fmaxr);
        let assign24080_e27066: f64 = (assign24080_e27061 - assign24080_e27065);
        let assign24080_e27068: f64 = (assign24080_e27066 * 0.3333333333333333);
        let assign24080_e27069: f64 = (1.0 + assign24080_e27068);
        let assign24080_e27070: f64 = (assign24080_e27058 * assign24080_e27069);
        let assign24080_e27071: f64 = (0.5 * assign24080_e27070);
        let assign24080_e27072: f64 = (1.0 + assign24080_e27071);
        let assign24080_e27073: f64 = (assign24080_e27049 * assign24080_e27072);
        let assign24080_e27074: f64 = (1.0 + assign24080_e27073);
        let assign24080_e27075: f64 = (1e-100 / assign24080_e27074);
        (assign24080_e27075, (-((1e-100 * (((-(-((assign24080_e27046 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign24080_e27072) + (assign24080_e27049 * (0.5 * (((-(-((assign24080_e27055 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign24080_e27069) + (assign24080_e27058 * ((-(-((assign24080_e27063 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign24080_e27074 * assign24080_e27074))), (-((1e-100 * (((-(-((assign24080_e27046 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign24080_e27072) + (assign24080_e27049 * (0.5 * (((-(-((assign24080_e27055 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign24080_e27069) + (assign24080_e27058 * ((-(-((assign24080_e27063 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign24080_e27074 * assign24080_e27074))), (-((1e-100 * (((-(-((assign24080_e27046 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign24080_e27072) + (assign24080_e27049 * (0.5 * (((-(-((assign24080_e27055 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign24080_e27069) + (assign24080_e27058 * ((-(-((assign24080_e27063 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign24080_e27074 * assign24080_e27074))), (-((1e-100 * (((-(-((assign24080_e27046 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign24080_e27072) + (assign24080_e27049 * (0.5 * (((-(-((assign24080_e27055 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign24080_e27069) + (assign24080_e27058 * ((-(-((assign24080_e27063 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign24080_e27074 * assign24080_e27074))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign24080_e27077;
        var_tmp_dn5 = assign24080_e27077_d_n5;
        var_tmp_dn6 = assign24080_e27077_d_n6;
        var_tmp_dn7 = assign24080_e27077_d_n7;
        var_tmp_dn8 = assign24080_e27077_d_n8;

        let (assign24090_e27126, assign24090_e27126_d_n5, assign24090_e27126_d_n6, assign24090_e27126_d_n7, assign24090_e27126_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) && (var_guard442 == 0.0)) && (var_guard444 == 0.0)) && (var_guard445 == 0.0)) {
        let assign24090_e27096: f64 = (-var_fbbtsti);
        let assign24090_e27098: f64 = (assign24090_e27096 / var_fmaxr);
        let assign24090_e27100: f64 = (assign24090_e27098 - 230.25850929940458);
        let assign24090_e27104: f64 = (-var_fbbtsti);
        let assign24090_e27106: f64 = (assign24090_e27104 / var_fmaxr);
        let assign24090_e27108: f64 = (assign24090_e27106 - 230.25850929940458);
        let assign24090_e27111: f64 = (-var_fbbtsti);
        let assign24090_e27113: f64 = (assign24090_e27111 / var_fmaxr);
        let assign24090_e27115: f64 = (assign24090_e27113 - 230.25850929940458);
        let assign24090_e27117: f64 = (assign24090_e27115 * 0.3333333333333333);
        let assign24090_e27118: f64 = (1.0 + assign24090_e27117);
        let assign24090_e27119: f64 = (assign24090_e27108 * assign24090_e27118);
        let assign24090_e27120: f64 = (0.5 * assign24090_e27119);
        let assign24090_e27121: f64 = (1.0 + assign24090_e27120);
        let assign24090_e27122: f64 = (assign24090_e27100 * assign24090_e27121);
        let assign24090_e27123: f64 = (1.0 + assign24090_e27122);
        let assign24090_e27124: f64 = (1e100 * assign24090_e27123);
        (assign24090_e27124, (1e100 * (((-((assign24090_e27096 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign24090_e27121) + (assign24090_e27100 * (0.5 * (((-((assign24090_e27104 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign24090_e27118) + (assign24090_e27108 * ((-((assign24090_e27111 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign24090_e27096 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign24090_e27121) + (assign24090_e27100 * (0.5 * (((-((assign24090_e27104 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign24090_e27118) + (assign24090_e27108 * ((-((assign24090_e27111 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign24090_e27096 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign24090_e27121) + (assign24090_e27100 * (0.5 * (((-((assign24090_e27104 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign24090_e27118) + (assign24090_e27108 * ((-((assign24090_e27111 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign24090_e27096 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign24090_e27121) + (assign24090_e27100 * (0.5 * (((-((assign24090_e27104 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign24090_e27118) + (assign24090_e27108 * ((-((assign24090_e27111 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign24090_e27126;
        var_tmp_dn5 = assign24090_e27126_d_n5;
        var_tmp_dn6 = assign24090_e27126_d_n6;
        var_tmp_dn7 = assign24090_e27126_d_n7;
        var_tmp_dn8 = assign24090_e27126_d_n8;

        let (assign24100_e27146, assign24100_e27146_d_n5, assign24100_e27146_d_n6, assign24100_e27146_d_n7, assign24100_e27146_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) && (var_guard442 == 0.0)) {
        let assign24100_e27139: f64 = (var_v4 * var_fmaxr);
        let assign24100_e27141: f64 = (assign24100_e27139 * var_fmaxr);
        let assign24100_e27143: f64 = (assign24100_e27141 * var_tmp);
        let assign24100_e27144: f64 = (p.p845 * assign24100_e27143);
        (assign24100_e27144, (p.p845 * (((((var_v4 * var_fmaxr_dn5) * var_fmaxr) + (assign24100_e27139 * var_fmaxr_dn5)) * var_tmp) + (assign24100_e27141 * var_tmp_dn5))), (p.p845 * (((((var_v4 * var_fmaxr_dn6) * var_fmaxr) + (assign24100_e27139 * var_fmaxr_dn6)) * var_tmp) + (assign24100_e27141 * var_tmp_dn6))), (p.p845 * (((((var_v4 * var_fmaxr_dn7) * var_fmaxr) + (assign24100_e27139 * var_fmaxr_dn7)) * var_tmp) + (assign24100_e27141 * var_tmp_dn7))), (p.p845 * (((((var_v4 * var_fmaxr_dn8) * var_fmaxr) + (assign24100_e27139 * var_fmaxr_dn8)) * var_tmp) + (assign24100_e27141 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign24100_e27146;
        var_ibbt_dn5 = assign24100_e27146_d_n5;
        var_ibbt_dn6 = assign24100_e27146_d_n6;
        var_ibbt_dn7 = assign24100_e27146_d_n7;
        var_ibbt_dn8 = assign24100_e27146_d_n8;

        let assign24110_e27149: f64 = if p.p854 > 1000.0 { 1.0 } else { 0.0 };
        var_guard446 = assign24110_e27149;

        let (assign24120_e27160, assign24120_e27160_d_n5, assign24120_e27160_d_n6, assign24120_e27160_d_n7, assign24120_e27160_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) && (var_guard446 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign24120_e27160;
        var_fbreakdown_dn5 = assign24120_e27160_d_n5;
        var_fbreakdown_dn6 = assign24120_e27160_d_n6;
        var_fbreakdown_dn7 = assign24120_e27160_d_n7;
        var_fbreakdown_dn8 = assign24120_e27160_d_n8;

        let assign24130_e27163: f64 = (-var_alphaav);
        let assign24130_e27165: f64 = (assign24130_e27163 * p.p854);
        let assign24130_e27166: f64 = if var_vav > assign24130_e27165 { 1.0 } else { 0.0 };
        var_guard447 = assign24130_e27166;

        let assign24140_e27169: f64 = if p.p857 == 4.0 { 1.0 } else { 0.0 };
        var_guard448 = assign24140_e27169;

        let (assign24150_e27199, assign24150_e27199_d_n5, assign24150_e27199_d_n6, assign24150_e27199_d_n7, assign24150_e27199_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) && (var_guard446 == 0.0)) && (var_guard447 != 0.0)) && (var_guard448 != 0.0)) {
        let assign24150_e27185: f64 = (var_vav * var_vbrinvsti);
        let assign24150_e27188: f64 = (var_vav * var_vbrinvsti);
        let assign24150_e27189: f64 = (assign24150_e27185 * assign24150_e27188);
        let assign24150_e27192: f64 = (var_vav * var_vbrinvsti);
        let assign24150_e27193: f64 = (assign24150_e27189 * assign24150_e27192);
        let assign24150_e27196: f64 = (var_vav * var_vbrinvsti);
        let assign24150_e27197: f64 = (assign24150_e27193 * assign24150_e27196);
        (assign24150_e27197, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign24150_e27199;
        var_tmp_dn5 = assign24150_e27199_d_n5;
        var_tmp_dn6 = assign24150_e27199_d_n6;
        var_tmp_dn7 = assign24150_e27199_d_n7;
        var_tmp_dn8 = assign24150_e27199_d_n8;

        let (assign24160_e27221, assign24160_e27221_d_n5, assign24160_e27221_d_n6, assign24160_e27221_d_n7, assign24160_e27221_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) && (var_guard446 == 0.0)) && (var_guard447 != 0.0)) && (var_guard448 == 0.0)) {
        let assign24160_e27216: f64 = (var_vav * var_vbrinvsti);
        let assign24160_e27217: f64 = (assign24160_e27216).abs();
        let assign24160_e27219: f64 = (assign24160_e27217).powf(p.p857);
        (assign24160_e27219, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign24160_e27221;
        var_tmp_dn5 = assign24160_e27221_d_n5;
        var_tmp_dn6 = assign24160_e27221_d_n6;
        var_tmp_dn7 = assign24160_e27221_d_n7;
        var_tmp_dn8 = assign24160_e27221_d_n8;

        let (assign24170_e27239, assign24170_e27239_d_n5, assign24170_e27239_d_n6, assign24170_e27239_d_n7, assign24170_e27239_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) && (var_guard446 == 0.0)) && (var_guard447 != 0.0)) {
        let assign24170_e27236: f64 = (1.0 - var_tmp);
        let assign24170_e27237: f64 = (1.0 / assign24170_e27236);
        (assign24170_e27237, (-((-var_tmp_dn5) / (assign24170_e27236 * assign24170_e27236))), (-((-var_tmp_dn6) / (assign24170_e27236 * assign24170_e27236))), (-((-var_tmp_dn7) / (assign24170_e27236 * assign24170_e27236))), (-((-var_tmp_dn8) / (assign24170_e27236 * assign24170_e27236))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign24170_e27239;
        var_fbreakdown_dn5 = assign24170_e27239_d_n5;
        var_fbreakdown_dn6 = assign24170_e27239_d_n6;
        var_fbreakdown_dn7 = assign24170_e27239_d_n7;
        var_fbreakdown_dn8 = assign24170_e27239_d_n8;

        let (assign24180_e27262, assign24180_e27262_d_n5, assign24180_e27262_d_n6, assign24180_e27262_d_n7, assign24180_e27262_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) && (var_guard446 == 0.0)) && (var_guard447 == 0.0)) {
        let assign24180_e27256: f64 = (var_alphaav * p.p854);
        let assign24180_e27257: f64 = (var_vav + assign24180_e27256);
        let assign24180_e27259: f64 = (assign24180_e27257 * var_slopesti);
        let assign24180_e27260: f64 = (var_fstopsti + assign24180_e27259);
        (assign24180_e27260, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign24180_e27262;
        var_fbreakdown_dn5 = assign24180_e27262_d_n5;
        var_fbreakdown_dn6 = assign24180_e27262_d_n6;
        var_fbreakdown_dn7 = assign24180_e27262_d_n7;
        var_fbreakdown_dn8 = assign24180_e27262_d_n8;

        let (assign24190_e27281, assign24190_e27281_d_n5, assign24190_e27281_d_n6, assign24190_e27281_d_n7, assign24190_e27281_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard432 == 0.0)) {
        let assign24190_e27272: f64 = (var_id__blk213 + var_isrh);
        let assign24190_e27274: f64 = (assign24190_e27272 + var_itat);
        let assign24190_e27276: f64 = (assign24190_e27274 + var_ibbt);
        let assign24190_e27277: f64 = (p.p29 * assign24190_e27276);
        let assign24190_e27279: f64 = (assign24190_e27277 * var_fbreakdown);
        (assign24190_e27279, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign24190_e27277 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign24190_e27277 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign24190_e27277 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign24190_e27277 * var_fbreakdown_dn8)),)
    } else {
        (var_ijunsti, var_ijunsti_dn5, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8,)
    }
};
        var_ijunsti = assign24190_e27281;
        var_ijunsti_dn5 = assign24190_e27281_d_n5;
        var_ijunsti_dn6 = assign24190_e27281_d_n6;
        var_ijunsti_dn7 = assign24190_e27281_d_n7;
        var_ijunsti_dn8 = assign24190_e27281_d_n8;

        let assign24200_e27284: f64 = if var_lgsource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard449 = assign24200_e27284;

        let (assign24210_e27292, assign24210_e27292_d_n5, assign24210_e27292_d_n6, assign24210_e27292_d_n7, assign24210_e27292_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijungat, var_ijungat_dn5, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8,)
    }
};
        var_ijungat = assign24210_e27292;
        var_ijungat_dn5 = assign24210_e27292_d_n5;
        var_ijungat_dn6 = assign24210_e27292_d_n6;
        var_ijungat_dn7 = assign24210_e27292_d_n7;
        var_ijungat_dn8 = assign24210_e27292_d_n8;

        let (assign24220_e27303,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) {
        let assign24220_e27301: f64 = (var_idsatgat * var_idmult);
        (assign24220_e27301,)
    } else {
        (var_id__blk213,)
    }
};
        var_id__blk213 = assign24220_e27303;

        let assign24230_e27310: f64 = if ((p.p835 == 0.0) && (p.p840 == 0.0)) { 1.0 } else { 0.0 };
        var_guard450 = assign24230_e27310;

        let (assign24240_e27321, assign24240_e27321_d_n5, assign24240_e27321_d_n6, assign24240_e27321_d_n7, assign24240_e27321_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign24240_e27321;
        var_isrh_dn5 = assign24240_e27321_d_n5;
        var_isrh_dn6 = assign24240_e27321_d_n6;
        var_isrh_dn7 = assign24240_e27321_d_n7;
        var_isrh_dn8 = assign24240_e27321_d_n8;

        let (assign24250_e27335,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) && (var_guard450 == 0.0)) {
        let assign24250_e27333: f64 = (var_vbigat - var_vjsrh);
        (assign24250_e27333,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign24250_e27335;

        *var_erfcpos_slot = var_erfcpos;
        *var_erfcpos_dn5_slot = var_erfcpos_dn5;
        *var_erfcpos_dn6_slot = var_erfcpos_dn6;
        *var_erfcpos_dn7_slot = var_erfcpos_dn7;
        *var_erfcpos_dn8_slot = var_erfcpos_dn8;
        *var_erfctimesexpmtat_slot = var_erfctimesexpmtat;
        *var_erfctimesexpmtat_dn5_slot = var_erfctimesexpmtat_dn5;
        *var_erfctimesexpmtat_dn6_slot = var_erfctimesexpmtat_dn6;
        *var_erfctimesexpmtat_dn7_slot = var_erfctimesexpmtat_dn7;
        *var_erfctimesexpmtat_dn8_slot = var_erfctimesexpmtat_dn8;
        *var_fbreakdown_slot = var_fbreakdown;
        *var_fbreakdown_dn5_slot = var_fbreakdown_dn5;
        *var_fbreakdown_dn6_slot = var_fbreakdown_dn6;
        *var_fbreakdown_dn7_slot = var_fbreakdown_dn7;
        *var_fbreakdown_dn8_slot = var_fbreakdown_dn8;
        *var_fmaxr_slot = var_fmaxr;
        *var_fmaxr_dn5_slot = var_fmaxr_dn5;
        *var_fmaxr_dn6_slot = var_fmaxr_dn6;
        *var_fmaxr_dn7_slot = var_fmaxr_dn7;
        *var_fmaxr_dn8_slot = var_fmaxr_dn8;
        *var_gammamax_slot = var_gammamax;
        *var_gammamax_dn5_slot = var_gammamax_dn5;
        *var_gammamax_dn6_slot = var_gammamax_dn6;
        *var_gammamax_dn7_slot = var_gammamax_dn7;
        *var_gammamax_dn8_slot = var_gammamax_dn8;
        *var_guard440_slot = var_guard440;
        *var_guard441_slot = var_guard441;
        *var_guard442_slot = var_guard442;
        *var_guard443_slot = var_guard443;
        *var_guard444_slot = var_guard444;
        *var_guard445_slot = var_guard445;
        *var_guard446_slot = var_guard446;
        *var_guard447_slot = var_guard447;
        *var_guard448_slot = var_guard448;
        *var_guard449_slot = var_guard449;
        *var_guard450_slot = var_guard450;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn5_slot = var_ibbt_dn5;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
        *var_id__blk213_slot = var_id__blk213;
        *var_ijungat_slot = var_ijungat;
        *var_ijungat_dn5_slot = var_ijungat_dn5;
        *var_ijungat_dn6_slot = var_ijungat_dn6;
        *var_ijungat_dn7_slot = var_ijungat_dn7;
        *var_ijungat_dn8_slot = var_ijungat_dn8;
        *var_ijunsti_slot = var_ijunsti;
        *var_ijunsti_dn5_slot = var_ijunsti_dn5;
        *var_ijunsti_dn6_slot = var_ijunsti_dn6;
        *var_ijunsti_dn7_slot = var_ijunsti_dn7;
        *var_ijunsti_dn8_slot = var_ijunsti_dn8;
        *var_isrh_slot = var_isrh;
        *var_isrh_dn5_slot = var_isrh_dn5;
        *var_isrh_dn6_slot = var_isrh_dn6;
        *var_isrh_dn7_slot = var_isrh_dn7;
        *var_isrh_dn8_slot = var_isrh_dn8;
        *var_itat_slot = var_itat;
        *var_itat_dn5_slot = var_itat_dn5;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn5_slot = var_tmp_dn5;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_vbi_minus_vjsrh_slot = var_vbi_minus_vjsrh;
    }

    pub(super) fn stamp_transient_block_45(
        p: &Parameters,
        var_atatgat: f64,
        var_berfc: f64,
        var_btatpartgat: f64,
        var_cerfc: f64,
        var_ftdgat: f64,
        var_guard176: f64,
        var_guard193: f64,
        var_guard449: f64,
        var_guard450: f64,
        var_one_minus_pgat: f64,
        var_one_over_one_minus_pgat: f64,
        var_perfc: f64,
        var_two_psistar: f64,
        var_vbi_minus_vjsrh: f64,
        var_vbirgatinv: f64,
        var_wdepnulrgat: f64,
        var_zinv: f64,
        var_asrh_slot: &mut f64,
        var_asrh_dn5_slot: &mut f64,
        var_asrh_dn6_slot: &mut f64,
        var_asrh_dn7_slot: &mut f64,
        var_asrh_dn8_slot: &mut f64,
        var_btat_slot: &mut f64,
        var_btat_dn5_slot: &mut f64,
        var_btat_dn6_slot: &mut f64,
        var_btat_dn7_slot: &mut f64,
        var_btat_dn8_slot: &mut f64,
        var_dwsrh_slot: &mut f64,
        var_erfcpos_slot: &mut f64,
        var_erfcpos_dn5_slot: &mut f64,
        var_erfcpos_dn6_slot: &mut f64,
        var_erfcpos_dn7_slot: &mut f64,
        var_erfcpos_dn8_slot: &mut f64,
        var_erfctimesexpmtat_slot: &mut f64,
        var_erfctimesexpmtat_dn5_slot: &mut f64,
        var_erfctimesexpmtat_dn6_slot: &mut f64,
        var_erfctimesexpmtat_dn7_slot: &mut f64,
        var_erfctimesexpmtat_dn8_slot: &mut f64,
        var_guard451_slot: &mut f64,
        var_guard452_slot: &mut f64,
        var_guard453_slot: &mut f64,
        var_guard454_slot: &mut f64,
        var_guard455_slot: &mut f64,
        var_guard456_slot: &mut f64,
        var_guard457_slot: &mut f64,
        var_guard458_slot: &mut f64,
        var_isrh_slot: &mut f64,
        var_isrh_dn5_slot: &mut f64,
        var_isrh_dn6_slot: &mut f64,
        var_isrh_dn7_slot: &mut f64,
        var_isrh_dn8_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn5_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_ktat_slot: &mut f64,
        var_ktat_dn5_slot: &mut f64,
        var_ktat_dn6_slot: &mut f64,
        var_ktat_dn7_slot: &mut f64,
        var_ktat_dn8_slot: &mut f64,
        var_ltat_slot: &mut f64,
        var_ltat_dn5_slot: &mut f64,
        var_ltat_dn6_slot: &mut f64,
        var_ltat_dn7_slot: &mut f64,
        var_ltat_dn8_slot: &mut f64,
        var_mtat_slot: &mut f64,
        var_mtat_dn5_slot: &mut f64,
        var_mtat_dn6_slot: &mut f64,
        var_mtat_dn7_slot: &mut f64,
        var_mtat_dn8_slot: &mut f64,
        var_sqrtumax_slot: &mut f64,
        var_sqrtumax_dn5_slot: &mut f64,
        var_sqrtumax_dn6_slot: &mut f64,
        var_sqrtumax_dn7_slot: &mut f64,
        var_sqrtumax_dn8_slot: &mut f64,
        var_terfc_slot: &mut f64,
        var_terfc_dn5_slot: &mut f64,
        var_terfc_dn6_slot: &mut f64,
        var_terfc_dn7_slot: &mut f64,
        var_terfc_dn8_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn5_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_twoatatoverthreebtat_slot: &mut f64,
        var_twoatatoverthreebtat_dn5_slot: &mut f64,
        var_twoatatoverthreebtat_dn6_slot: &mut f64,
        var_twoatatoverthreebtat_dn7_slot: &mut f64,
        var_twoatatoverthreebtat_dn8_slot: &mut f64,
        var_umax_slot: &mut f64,
        var_umax_dn5_slot: &mut f64,
        var_umax_dn6_slot: &mut f64,
        var_umax_dn7_slot: &mut f64,
        var_umax_dn8_slot: &mut f64,
        var_umaxbeforelimiting_slot: &mut f64,
        var_umaxbeforelimiting_dn5_slot: &mut f64,
        var_umaxbeforelimiting_dn6_slot: &mut f64,
        var_umaxbeforelimiting_dn7_slot: &mut f64,
        var_umaxbeforelimiting_dn8_slot: &mut f64,
        var_umaxpoweronepointfive_slot: &mut f64,
        var_umaxpoweronepointfive_dn5_slot: &mut f64,
        var_umaxpoweronepointfive_dn6_slot: &mut f64,
        var_umaxpoweronepointfive_dn7_slot: &mut f64,
        var_umaxpoweronepointfive_dn8_slot: &mut f64,
        var_wdep_slot: &mut f64,
        var_wdep_dn5_slot: &mut f64,
        var_wdep_dn6_slot: &mut f64,
        var_wdep_dn7_slot: &mut f64,
        var_wdep_dn8_slot: &mut f64,
        var_wgamma_slot: &mut f64,
        var_wgamma_dn5_slot: &mut f64,
        var_wgamma_dn6_slot: &mut f64,
        var_wgamma_dn7_slot: &mut f64,
        var_wgamma_dn8_slot: &mut f64,
        var_wsrh_slot: &mut f64,
        var_wsrhstep_slot: &mut f64,
        var_wtat_slot: &mut f64,
        var_wtat_dn5_slot: &mut f64,
        var_wtat_dn6_slot: &mut f64,
        var_wtat_dn7_slot: &mut f64,
        var_wtat_dn8_slot: &mut f64,
        var_xerfc_slot: &mut f64,
        var_xerfc_dn5_slot: &mut f64,
        var_xerfc_dn6_slot: &mut f64,
        var_xerfc_dn7_slot: &mut f64,
        var_xerfc_dn8_slot: &mut f64,
        var_ysq_slot: &mut f64,
        var_ysq_dn5_slot: &mut f64,
        var_ysq_dn6_slot: &mut f64,
        var_ysq_dn7_slot: &mut f64,
        var_ysq_dn8_slot: &mut f64,
    ) {
        let mut var_asrh: f64 = *var_asrh_slot;
        let mut var_asrh_dn5: f64 = *var_asrh_dn5_slot;
        let mut var_asrh_dn6: f64 = *var_asrh_dn6_slot;
        let mut var_asrh_dn7: f64 = *var_asrh_dn7_slot;
        let mut var_asrh_dn8: f64 = *var_asrh_dn8_slot;
        let mut var_btat: f64 = *var_btat_slot;
        let mut var_btat_dn5: f64 = *var_btat_dn5_slot;
        let mut var_btat_dn6: f64 = *var_btat_dn6_slot;
        let mut var_btat_dn7: f64 = *var_btat_dn7_slot;
        let mut var_btat_dn8: f64 = *var_btat_dn8_slot;
        let mut var_dwsrh: f64 = *var_dwsrh_slot;
        let mut var_erfcpos: f64 = *var_erfcpos_slot;
        let mut var_erfcpos_dn5: f64 = *var_erfcpos_dn5_slot;
        let mut var_erfcpos_dn6: f64 = *var_erfcpos_dn6_slot;
        let mut var_erfcpos_dn7: f64 = *var_erfcpos_dn7_slot;
        let mut var_erfcpos_dn8: f64 = *var_erfcpos_dn8_slot;
        let mut var_erfctimesexpmtat: f64 = *var_erfctimesexpmtat_slot;
        let mut var_erfctimesexpmtat_dn5: f64 = *var_erfctimesexpmtat_dn5_slot;
        let mut var_erfctimesexpmtat_dn6: f64 = *var_erfctimesexpmtat_dn6_slot;
        let mut var_erfctimesexpmtat_dn7: f64 = *var_erfctimesexpmtat_dn7_slot;
        let mut var_erfctimesexpmtat_dn8: f64 = *var_erfctimesexpmtat_dn8_slot;
        let mut var_guard451: f64 = *var_guard451_slot;
        let mut var_guard452: f64 = *var_guard452_slot;
        let mut var_guard453: f64 = *var_guard453_slot;
        let mut var_guard454: f64 = *var_guard454_slot;
        let mut var_guard455: f64 = *var_guard455_slot;
        let mut var_guard456: f64 = *var_guard456_slot;
        let mut var_guard457: f64 = *var_guard457_slot;
        let mut var_guard458: f64 = *var_guard458_slot;
        let mut var_isrh: f64 = *var_isrh_slot;
        let mut var_isrh_dn5: f64 = *var_isrh_dn5_slot;
        let mut var_isrh_dn6: f64 = *var_isrh_dn6_slot;
        let mut var_isrh_dn7: f64 = *var_isrh_dn7_slot;
        let mut var_isrh_dn8: f64 = *var_isrh_dn8_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn5: f64 = *var_itat_dn5_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_ktat: f64 = *var_ktat_slot;
        let mut var_ktat_dn5: f64 = *var_ktat_dn5_slot;
        let mut var_ktat_dn6: f64 = *var_ktat_dn6_slot;
        let mut var_ktat_dn7: f64 = *var_ktat_dn7_slot;
        let mut var_ktat_dn8: f64 = *var_ktat_dn8_slot;
        let mut var_ltat: f64 = *var_ltat_slot;
        let mut var_ltat_dn5: f64 = *var_ltat_dn5_slot;
        let mut var_ltat_dn6: f64 = *var_ltat_dn6_slot;
        let mut var_ltat_dn7: f64 = *var_ltat_dn7_slot;
        let mut var_ltat_dn8: f64 = *var_ltat_dn8_slot;
        let mut var_mtat: f64 = *var_mtat_slot;
        let mut var_mtat_dn5: f64 = *var_mtat_dn5_slot;
        let mut var_mtat_dn6: f64 = *var_mtat_dn6_slot;
        let mut var_mtat_dn7: f64 = *var_mtat_dn7_slot;
        let mut var_mtat_dn8: f64 = *var_mtat_dn8_slot;
        let mut var_sqrtumax: f64 = *var_sqrtumax_slot;
        let mut var_sqrtumax_dn5: f64 = *var_sqrtumax_dn5_slot;
        let mut var_sqrtumax_dn6: f64 = *var_sqrtumax_dn6_slot;
        let mut var_sqrtumax_dn7: f64 = *var_sqrtumax_dn7_slot;
        let mut var_sqrtumax_dn8: f64 = *var_sqrtumax_dn8_slot;
        let mut var_terfc: f64 = *var_terfc_slot;
        let mut var_terfc_dn5: f64 = *var_terfc_dn5_slot;
        let mut var_terfc_dn6: f64 = *var_terfc_dn6_slot;
        let mut var_terfc_dn7: f64 = *var_terfc_dn7_slot;
        let mut var_terfc_dn8: f64 = *var_terfc_dn8_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn5: f64 = *var_tmp_dn5_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_twoatatoverthreebtat: f64 = *var_twoatatoverthreebtat_slot;
        let mut var_twoatatoverthreebtat_dn5: f64 = *var_twoatatoverthreebtat_dn5_slot;
        let mut var_twoatatoverthreebtat_dn6: f64 = *var_twoatatoverthreebtat_dn6_slot;
        let mut var_twoatatoverthreebtat_dn7: f64 = *var_twoatatoverthreebtat_dn7_slot;
        let mut var_twoatatoverthreebtat_dn8: f64 = *var_twoatatoverthreebtat_dn8_slot;
        let mut var_umax: f64 = *var_umax_slot;
        let mut var_umax_dn5: f64 = *var_umax_dn5_slot;
        let mut var_umax_dn6: f64 = *var_umax_dn6_slot;
        let mut var_umax_dn7: f64 = *var_umax_dn7_slot;
        let mut var_umax_dn8: f64 = *var_umax_dn8_slot;
        let mut var_umaxbeforelimiting: f64 = *var_umaxbeforelimiting_slot;
        let mut var_umaxbeforelimiting_dn5: f64 = *var_umaxbeforelimiting_dn5_slot;
        let mut var_umaxbeforelimiting_dn6: f64 = *var_umaxbeforelimiting_dn6_slot;
        let mut var_umaxbeforelimiting_dn7: f64 = *var_umaxbeforelimiting_dn7_slot;
        let mut var_umaxbeforelimiting_dn8: f64 = *var_umaxbeforelimiting_dn8_slot;
        let mut var_umaxpoweronepointfive: f64 = *var_umaxpoweronepointfive_slot;
        let mut var_umaxpoweronepointfive_dn5: f64 = *var_umaxpoweronepointfive_dn5_slot;
        let mut var_umaxpoweronepointfive_dn6: f64 = *var_umaxpoweronepointfive_dn6_slot;
        let mut var_umaxpoweronepointfive_dn7: f64 = *var_umaxpoweronepointfive_dn7_slot;
        let mut var_umaxpoweronepointfive_dn8: f64 = *var_umaxpoweronepointfive_dn8_slot;
        let mut var_wdep: f64 = *var_wdep_slot;
        let mut var_wdep_dn5: f64 = *var_wdep_dn5_slot;
        let mut var_wdep_dn6: f64 = *var_wdep_dn6_slot;
        let mut var_wdep_dn7: f64 = *var_wdep_dn7_slot;
        let mut var_wdep_dn8: f64 = *var_wdep_dn8_slot;
        let mut var_wgamma: f64 = *var_wgamma_slot;
        let mut var_wgamma_dn5: f64 = *var_wgamma_dn5_slot;
        let mut var_wgamma_dn6: f64 = *var_wgamma_dn6_slot;
        let mut var_wgamma_dn7: f64 = *var_wgamma_dn7_slot;
        let mut var_wgamma_dn8: f64 = *var_wgamma_dn8_slot;
        let mut var_wsrh: f64 = *var_wsrh_slot;
        let mut var_wsrhstep: f64 = *var_wsrhstep_slot;
        let mut var_wtat: f64 = *var_wtat_slot;
        let mut var_wtat_dn5: f64 = *var_wtat_dn5_slot;
        let mut var_wtat_dn6: f64 = *var_wtat_dn6_slot;
        let mut var_wtat_dn7: f64 = *var_wtat_dn7_slot;
        let mut var_wtat_dn8: f64 = *var_wtat_dn8_slot;
        let mut var_xerfc: f64 = *var_xerfc_slot;
        let mut var_xerfc_dn5: f64 = *var_xerfc_dn5_slot;
        let mut var_xerfc_dn6: f64 = *var_xerfc_dn6_slot;
        let mut var_xerfc_dn7: f64 = *var_xerfc_dn7_slot;
        let mut var_xerfc_dn8: f64 = *var_xerfc_dn8_slot;
        let mut var_ysq: f64 = *var_ysq_slot;
        let mut var_ysq_dn5: f64 = *var_ysq_dn5_slot;
        let mut var_ysq_dn6: f64 = *var_ysq_dn6_slot;
        let mut var_ysq_dn7: f64 = *var_ysq_dn7_slot;
        let mut var_ysq_dn8: f64 = *var_ysq_dn8_slot;

        let (assign24260_e27354,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) && (var_guard450 == 0.0)) {
        let assign24260_e27349: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign24260_e27350: f64 = (1.0 - assign24260_e27349);
        let assign24260_e27351: f64 = (assign24260_e27350).sqrt();
        let assign24260_e27352: f64 = (1.0 - assign24260_e27351);
        (assign24260_e27352,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign24260_e27354;

        let assign24270_e27357: f64 = if p.p826 == 0.5 { 1.0 } else { 0.0 };
        var_guard451 = assign24270_e27357;

        let (assign24280_e27371,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) && (var_guard450 == 0.0)) && (var_guard451 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign24280_e27371;

        let (assign24290_e27403,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) && (var_guard450 == 0.0)) && (var_guard451 == 0.0)) {
        let assign24290_e27386: f64 = (var_wsrhstep * var_wsrhstep);
        let assign24290_e27388: f64 = (var_wsrhstep).ln();
        let assign24290_e27389: f64 = (assign24290_e27386 * assign24290_e27388);
        let assign24290_e27392: f64 = (1.0 - var_wsrhstep);
        let assign24290_e27393: f64 = (assign24290_e27389 / assign24290_e27392);
        let assign24290_e27395: f64 = (assign24290_e27393 + var_wsrhstep);
        let assign24290_e27399: f64 = (2.0 * p.p826);
        let assign24290_e27400: f64 = (1.0 - assign24290_e27399);
        let assign24290_e27401: f64 = (assign24290_e27395 * assign24290_e27400);
        (assign24290_e27401,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign24290_e27403;

        let (assign24300_e27417,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) && (var_guard450 == 0.0)) {
        let assign24300_e27415: f64 = (var_wsrhstep + var_dwsrh);
        (assign24300_e27415,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign24300_e27417;

        let assign24310_e27420: f64 = if p.p826 == 0.5 { 1.0 } else { 0.0 };
        var_guard452 = assign24310_e27420;

        let (assign24320_e27437, assign24320_e27437_d_n5, assign24320_e27437_d_n6, assign24320_e27437_d_n7, assign24320_e27437_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) && (var_guard450 == 0.0)) && (var_guard452 != 0.0)) {
        let assign24320_e27434: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv);
        let assign24320_e27435: f64 = (assign24320_e27434).sqrt();
        (assign24320_e27435, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign24320_e27437;
        var_tmp_dn5 = assign24320_e27437_d_n5;
        var_tmp_dn6 = assign24320_e27437_d_n6;
        var_tmp_dn7 = assign24320_e27437_d_n7;
        var_tmp_dn8 = assign24320_e27437_d_n8;

        let (assign24330_e27456, assign24330_e27456_d_n5, assign24330_e27456_d_n6, assign24330_e27456_d_n7, assign24330_e27456_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) && (var_guard450 == 0.0)) && (var_guard452 == 0.0)) {
        let assign24330_e27452: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv);
        let assign24330_e27454: f64 = (assign24330_e27452).powf(p.p826);
        (assign24330_e27454, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign24330_e27456;
        var_tmp_dn5 = assign24330_e27456_d_n5;
        var_tmp_dn6 = assign24330_e27456_d_n6;
        var_tmp_dn7 = assign24330_e27456_d_n7;
        var_tmp_dn8 = assign24330_e27456_d_n8;

        let (assign24340_e27470, assign24340_e27470_d_n5, assign24340_e27470_d_n6, assign24340_e27470_d_n7, assign24340_e27470_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) && (var_guard450 == 0.0)) {
        let assign24340_e27468: f64 = (var_wdepnulrgat * var_tmp);
        (assign24340_e27468, (var_wdepnulrgat * var_tmp_dn5), (var_wdepnulrgat * var_tmp_dn6), (var_wdepnulrgat * var_tmp_dn7), (var_wdepnulrgat * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign24340_e27470;
        var_wdep_dn5 = assign24340_e27470_d_n5;
        var_wdep_dn6 = assign24340_e27470_d_n6;
        var_wdep_dn7 = assign24340_e27470_d_n7;
        var_wdep_dn8 = assign24340_e27470_d_n8;

        let (assign24350_e27488, assign24350_e27488_d_n5, assign24350_e27488_d_n6, assign24350_e27488_d_n7, assign24350_e27488_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) && (var_guard450 == 0.0)) {
        let assign24350_e27483: f64 = (var_zinv - 1.0);
        let assign24350_e27485: f64 = (assign24350_e27483 * var_wdep);
        let assign24350_e27486: f64 = (var_ftdgat * assign24350_e27485);
        (assign24350_e27486, (var_ftdgat * (assign24350_e27483 * var_wdep_dn5)), (var_ftdgat * (assign24350_e27483 * var_wdep_dn6)), (var_ftdgat * (assign24350_e27483 * var_wdep_dn7)), (var_ftdgat * (assign24350_e27483 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign24350_e27488;
        var_asrh_dn5 = assign24350_e27488_d_n5;
        var_asrh_dn6 = assign24350_e27488_d_n6;
        var_asrh_dn7 = assign24350_e27488_d_n7;
        var_asrh_dn8 = assign24350_e27488_d_n8;

        let (assign24360_e27504, assign24360_e27504_d_n5, assign24360_e27504_d_n6, assign24360_e27504_d_n7, assign24360_e27504_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) && (var_guard450 == 0.0)) {
        let assign24360_e27501: f64 = (var_asrh * var_wsrh);
        let assign24360_e27502: f64 = (p.p835 * assign24360_e27501);
        (assign24360_e27502, (p.p835 * (var_asrh_dn5 * var_wsrh)), (p.p835 * (var_asrh_dn6 * var_wsrh)), (p.p835 * (var_asrh_dn7 * var_wsrh)), (p.p835 * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign24360_e27504;
        var_isrh_dn5 = assign24360_e27504_d_n5;
        var_isrh_dn6 = assign24360_e27504_d_n6;
        var_isrh_dn7 = assign24360_e27504_d_n7;
        var_isrh_dn8 = assign24360_e27504_d_n8;

        let assign24370_e27507: f64 = if p.p840 == 0.0 { 1.0 } else { 0.0 };
        var_guard453 = assign24370_e27507;

        let (assign24380_e27518, assign24380_e27518_d_n5, assign24380_e27518_d_n6, assign24380_e27518_d_n7, assign24380_e27518_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) && (var_guard453 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign24380_e27518;
        var_itat_dn5 = assign24380_e27518_d_n5;
        var_itat_dn6 = assign24380_e27518_d_n6;
        var_itat_dn7 = assign24380_e27518_d_n7;
        var_itat_dn8 = assign24380_e27518_d_n8;

        let (assign24390_e27536, assign24390_e27536_d_n5, assign24390_e27536_d_n6, assign24390_e27536_d_n7, assign24390_e27536_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) && (var_guard453 == 0.0)) {
        let assign24390_e27531: f64 = (var_wdep * var_one_minus_pgat);
        let assign24390_e27533: f64 = (assign24390_e27531 / var_vbi_minus_vjsrh);
        let assign24390_e27534: f64 = (var_btatpartgat * assign24390_e27533);
        (assign24390_e27534, (var_btatpartgat * ((var_wdep_dn5 * var_one_minus_pgat) / var_vbi_minus_vjsrh)), (var_btatpartgat * ((var_wdep_dn6 * var_one_minus_pgat) / var_vbi_minus_vjsrh)), (var_btatpartgat * ((var_wdep_dn7 * var_one_minus_pgat) / var_vbi_minus_vjsrh)), (var_btatpartgat * ((var_wdep_dn8 * var_one_minus_pgat) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign24390_e27536;
        var_btat_dn5 = assign24390_e27536_d_n5;
        var_btat_dn6 = assign24390_e27536_d_n6;
        var_btat_dn7 = assign24390_e27536_d_n7;
        var_btat_dn8 = assign24390_e27536_d_n8;

        let (assign24400_e27552, assign24400_e27552_d_n5, assign24400_e27552_d_n6, assign24400_e27552_d_n7, assign24400_e27552_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) && (var_guard453 == 0.0)) {
        let assign24400_e27548: f64 = (0.666666666666667 * var_atatgat);
        let assign24400_e27550: f64 = (assign24400_e27548 / var_btat);
        (assign24400_e27550, (-((assign24400_e27548 * var_btat_dn5) / (var_btat * var_btat))), (-((assign24400_e27548 * var_btat_dn6) / (var_btat * var_btat))), (-((assign24400_e27548 * var_btat_dn7) / (var_btat * var_btat))), (-((assign24400_e27548 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign24400_e27552;
        var_twoatatoverthreebtat_dn5 = assign24400_e27552_d_n5;
        var_twoatatoverthreebtat_dn6 = assign24400_e27552_d_n6;
        var_twoatatoverthreebtat_dn7 = assign24400_e27552_d_n7;
        var_twoatatoverthreebtat_dn8 = assign24400_e27552_d_n8;

        let (assign24410_e27566, assign24410_e27566_d_n5, assign24410_e27566_d_n6, assign24410_e27566_d_n7, assign24410_e27566_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) && (var_guard453 == 0.0)) {
        let assign24410_e27564: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign24410_e27564, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign24410_e27566;
        var_umaxbeforelimiting_dn5 = assign24410_e27566_d_n5;
        var_umaxbeforelimiting_dn6 = assign24410_e27566_d_n6;
        var_umaxbeforelimiting_dn7 = assign24410_e27566_d_n7;
        var_umaxbeforelimiting_dn8 = assign24410_e27566_d_n8;

        let (assign24420_e27587, assign24420_e27587_d_n5, assign24420_e27587_d_n6, assign24420_e27587_d_n7, assign24420_e27587_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) && (var_guard453 == 0.0)) {
        let assign24420_e27578: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign24420_e27581: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign24420_e27583: f64 = (assign24420_e27581 + 1.0);
        let assign24420_e27584: f64 = (assign24420_e27578 / assign24420_e27583);
        let assign24420_e27585: f64 = (assign24420_e27584).sqrt();
        (assign24420_e27585, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign24420_e27583) - (assign24420_e27578 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign24420_e27583 * assign24420_e27583)) / (2.0 * assign24420_e27585)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign24420_e27583) - (assign24420_e27578 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign24420_e27583 * assign24420_e27583)) / (2.0 * assign24420_e27585)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign24420_e27583) - (assign24420_e27578 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign24420_e27583 * assign24420_e27583)) / (2.0 * assign24420_e27585)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign24420_e27583) - (assign24420_e27578 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign24420_e27583 * assign24420_e27583)) / (2.0 * assign24420_e27585)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign24420_e27587;
        var_umax_dn5 = assign24420_e27587_d_n5;
        var_umax_dn6 = assign24420_e27587_d_n6;
        var_umax_dn7 = assign24420_e27587_d_n7;
        var_umax_dn8 = assign24420_e27587_d_n8;

        let (assign24430_e27600, assign24430_e27600_d_n5, assign24430_e27600_d_n6, assign24430_e27600_d_n7, assign24430_e27600_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) && (var_guard453 == 0.0)) {
        let assign24430_e27598: f64 = (var_umax).sqrt();
        (assign24430_e27598, (var_umax_dn5 / (2.0 * assign24430_e27598)), (var_umax_dn6 / (2.0 * assign24430_e27598)), (var_umax_dn7 / (2.0 * assign24430_e27598)), (var_umax_dn8 / (2.0 * assign24430_e27598)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign24430_e27600;
        var_sqrtumax_dn5 = assign24430_e27600_d_n5;
        var_sqrtumax_dn6 = assign24430_e27600_d_n6;
        var_sqrtumax_dn7 = assign24430_e27600_d_n7;
        var_sqrtumax_dn8 = assign24430_e27600_d_n8;

        let (assign24440_e27614, assign24440_e27614_d_n5, assign24440_e27614_d_n6, assign24440_e27614_d_n7, assign24440_e27614_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) && (var_guard453 == 0.0)) {
        let assign24440_e27612: f64 = (var_umax * var_sqrtumax);
        (assign24440_e27612, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign24440_e27614;
        var_umaxpoweronepointfive_dn5 = assign24440_e27614_d_n5;
        var_umaxpoweronepointfive_dn6 = assign24440_e27614_d_n6;
        var_umaxpoweronepointfive_dn7 = assign24440_e27614_d_n7;
        var_umaxpoweronepointfive_dn8 = assign24440_e27614_d_n8;

        let assign24450_e27616: f64 = (-p.p826);
        let assign24450_e27618: f64 = (assign24450_e27616 * var_one_over_one_minus_pgat);
        let assign24450_e27620: f64 = (-1.0);
        let assign24450_e27621: f64 = if assign24450_e27618 == assign24450_e27620 { 1.0 } else { 0.0 };
        var_guard454 = assign24450_e27621;

        let (assign24460_e27641, assign24460_e27641_d_n5, assign24460_e27641_d_n6, assign24460_e27641_d_n7, assign24460_e27641_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) && (var_guard453 == 0.0)) && (var_guard454 != 0.0)) {
        let assign24460_e27637: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign24460_e27638: f64 = (1.0 + assign24460_e27637);
        let assign24460_e27639: f64 = (1.0 / assign24460_e27638);
        (assign24460_e27639, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign24460_e27638 * assign24460_e27638))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign24460_e27638 * assign24460_e27638))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign24460_e27638 * assign24460_e27638))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign24460_e27638 * assign24460_e27638))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign24460_e27641;
        var_wgamma_dn5 = assign24460_e27641_d_n5;
        var_wgamma_dn6 = assign24460_e27641_d_n6;
        var_wgamma_dn7 = assign24460_e27641_d_n7;
        var_wgamma_dn8 = assign24460_e27641_d_n8;

        let (assign24470_e27665, assign24470_e27665_d_n5, assign24470_e27665_d_n6, assign24470_e27665_d_n7, assign24470_e27665_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) && (var_guard453 == 0.0)) && (var_guard454 == 0.0)) {
        let assign24470_e27657: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign24470_e27658: f64 = (1.0 + assign24470_e27657);
        let assign24470_e27660: f64 = (-p.p826);
        let assign24470_e27662: f64 = (assign24470_e27660 * var_one_over_one_minus_pgat);
        let assign24470_e27663: f64 = (assign24470_e27658).powf(assign24470_e27662);
        (assign24470_e27663, if 0.0 == 0.0 && ((assign24470_e27662) as f64).is_finite() && ((assign24470_e27662) as f64).fract() == 0.0 { if assign24470_e27662 == 0.0 { 0.0 } else { (assign24470_e27662 * ((assign24470_e27658).powf(assign24470_e27662 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign24470_e27663 * (assign24470_e27662 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign24470_e27658))) }, if 0.0 == 0.0 && ((assign24470_e27662) as f64).is_finite() && ((assign24470_e27662) as f64).fract() == 0.0 { if assign24470_e27662 == 0.0 { 0.0 } else { (assign24470_e27662 * ((assign24470_e27658).powf(assign24470_e27662 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign24470_e27663 * (assign24470_e27662 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign24470_e27658))) }, if 0.0 == 0.0 && ((assign24470_e27662) as f64).is_finite() && ((assign24470_e27662) as f64).fract() == 0.0 { if assign24470_e27662 == 0.0 { 0.0 } else { (assign24470_e27662 * ((assign24470_e27658).powf(assign24470_e27662 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign24470_e27663 * (assign24470_e27662 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign24470_e27658))) }, if 0.0 == 0.0 && ((assign24470_e27662) as f64).is_finite() && ((assign24470_e27662) as f64).fract() == 0.0 { if assign24470_e27662 == 0.0 { 0.0 } else { (assign24470_e27662 * ((assign24470_e27658).powf(assign24470_e27662 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign24470_e27663 * (assign24470_e27662 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign24470_e27658))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign24470_e27665;
        var_wgamma_dn5 = assign24470_e27665_d_n5;
        var_wgamma_dn6 = assign24470_e27665_d_n6;
        var_wgamma_dn7 = assign24470_e27665_d_n7;
        var_wgamma_dn8 = assign24470_e27665_d_n8;

        let (assign24480_e27683, assign24480_e27683_d_n5, assign24480_e27683_d_n6, assign24480_e27683_d_n7, assign24480_e27683_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) && (var_guard453 == 0.0)) {
        let assign24480_e27677: f64 = (var_wsrh * var_wgamma);
        let assign24480_e27680: f64 = (var_wsrh + var_wgamma);
        let assign24480_e27681: f64 = (assign24480_e27677 / assign24480_e27680);
        (assign24480_e27681, ((((var_wsrh * var_wgamma_dn5) * assign24480_e27680) - (assign24480_e27677 * var_wgamma_dn5)) / (assign24480_e27680 * assign24480_e27680)), ((((var_wsrh * var_wgamma_dn6) * assign24480_e27680) - (assign24480_e27677 * var_wgamma_dn6)) / (assign24480_e27680 * assign24480_e27680)), ((((var_wsrh * var_wgamma_dn7) * assign24480_e27680) - (assign24480_e27677 * var_wgamma_dn7)) / (assign24480_e27680 * assign24480_e27680)), ((((var_wsrh * var_wgamma_dn8) * assign24480_e27680) - (assign24480_e27677 * var_wgamma_dn8)) / (assign24480_e27680 * assign24480_e27680)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign24480_e27683;
        var_wtat_dn5 = assign24480_e27683_d_n5;
        var_wtat_dn6 = assign24480_e27683_d_n6;
        var_wtat_dn7 = assign24480_e27683_d_n7;
        var_wtat_dn8 = assign24480_e27683_d_n8;

        let (assign24490_e27700, assign24490_e27700_d_n5, assign24490_e27700_d_n6, assign24490_e27700_d_n7, assign24490_e27700_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) && (var_guard453 == 0.0)) {
        let assign24490_e27696: f64 = (var_btat / var_sqrtumax);
        let assign24490_e27697: f64 = (0.375 * assign24490_e27696);
        let assign24490_e27698: f64 = (assign24490_e27697).sqrt();
        (assign24490_e27698, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign24490_e27698)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign24490_e27698)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign24490_e27698)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign24490_e27698)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign24490_e27700;
        var_ktat_dn5 = assign24490_e27700_d_n5;
        var_ktat_dn6 = assign24490_e27700_d_n6;
        var_ktat_dn7 = assign24490_e27700_d_n7;
        var_ktat_dn8 = assign24490_e27700_d_n8;

        let (assign24500_e27718, assign24500_e27718_d_n5, assign24500_e27718_d_n6, assign24500_e27718_d_n7, assign24500_e27718_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) && (var_guard453 == 0.0)) {
        let assign24500_e27713: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign24500_e27714: f64 = (2.0 * assign24500_e27713);
        let assign24500_e27716: f64 = (assign24500_e27714 - var_umax);
        (assign24500_e27716, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign24500_e27718;
        var_ltat_dn5 = assign24500_e27718_d_n5;
        var_ltat_dn6 = assign24500_e27718_d_n6;
        var_ltat_dn7 = assign24500_e27718_d_n7;
        var_ltat_dn8 = assign24500_e27718_d_n8;

        let (assign24510_e27744, assign24510_e27744_d_n5, assign24510_e27744_d_n6, assign24510_e27744_d_n7, assign24510_e27744_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) && (var_guard453 == 0.0)) {
        let assign24510_e27730: f64 = (var_atatgat * var_twoatatoverthreebtat);
        let assign24510_e27732: f64 = (assign24510_e27730 * var_sqrtumax);
        let assign24510_e27735: f64 = (var_atatgat * var_umax);
        let assign24510_e27736: f64 = (assign24510_e27732 - assign24510_e27735);
        let assign24510_e27740: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign24510_e27741: f64 = (0.5 * assign24510_e27740);
        let assign24510_e27742: f64 = (assign24510_e27736 + assign24510_e27741);
        (assign24510_e27742, (((((var_atatgat * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign24510_e27730 * var_sqrtumax_dn5)) - (var_atatgat * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatgat * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign24510_e27730 * var_sqrtumax_dn6)) - (var_atatgat * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatgat * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign24510_e27730 * var_sqrtumax_dn7)) - (var_atatgat * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatgat * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign24510_e27730 * var_sqrtumax_dn8)) - (var_atatgat * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign24510_e27744;
        var_mtat_dn5 = assign24510_e27744_d_n5;
        var_mtat_dn6 = assign24510_e27744_d_n6;
        var_mtat_dn7 = assign24510_e27744_d_n7;
        var_mtat_dn8 = assign24510_e27744_d_n8;

        let (assign24520_e27760, assign24520_e27760_d_n5, assign24520_e27760_d_n6, assign24520_e27760_d_n7, assign24520_e27760_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) && (var_guard453 == 0.0)) {
        let assign24520_e27756: f64 = (var_ltat - 1.0);
        let assign24520_e27758: f64 = (assign24520_e27756 * var_ktat);
        (assign24520_e27758, ((var_ltat_dn5 * var_ktat) + (assign24520_e27756 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign24520_e27756 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign24520_e27756 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign24520_e27756 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign24520_e27760;
        var_xerfc_dn5 = assign24520_e27760_d_n5;
        var_xerfc_dn6 = assign24520_e27760_d_n6;
        var_xerfc_dn7 = assign24520_e27760_d_n7;
        var_xerfc_dn8 = assign24520_e27760_d_n8;

        let (assign24530_e27774, assign24530_e27774_d_n5, assign24530_e27774_d_n6, assign24530_e27774_d_n7, assign24530_e27774_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) && (var_guard453 == 0.0)) {
        let assign24530_e27772: f64 = (var_xerfc * var_xerfc);
        (assign24530_e27772, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign24530_e27774;
        var_ysq_dn5 = assign24530_e27774_d_n5;
        var_ysq_dn6 = assign24530_e27774_d_n6;
        var_ysq_dn7 = assign24530_e27774_d_n7;
        var_ysq_dn8 = assign24530_e27774_d_n8;

        let assign24540_e27777: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard455 = assign24540_e27777;

        let (assign24550_e27797, assign24550_e27797_d_n5, assign24550_e27797_d_n6, assign24550_e27797_d_n7, assign24550_e27797_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) && (var_guard453 == 0.0)) && (var_guard455 != 0.0)) {
        let assign24550_e27793: f64 = (var_perfc * var_xerfc);
        let assign24550_e27794: f64 = (1.0 + assign24550_e27793);
        let assign24550_e27795: f64 = (1.0 / assign24550_e27794);
        (assign24550_e27795, (-((var_perfc * var_xerfc_dn5) / (assign24550_e27794 * assign24550_e27794))), (-((var_perfc * var_xerfc_dn6) / (assign24550_e27794 * assign24550_e27794))), (-((var_perfc * var_xerfc_dn7) / (assign24550_e27794 * assign24550_e27794))), (-((var_perfc * var_xerfc_dn8) / (assign24550_e27794 * assign24550_e27794))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign24550_e27797;
        var_terfc_dn5 = assign24550_e27797_d_n5;
        var_terfc_dn6 = assign24550_e27797_d_n6;
        var_terfc_dn7 = assign24550_e27797_d_n7;
        var_terfc_dn8 = assign24550_e27797_d_n8;

        let (assign24560_e27818, assign24560_e27818_d_n5, assign24560_e27818_d_n6, assign24560_e27818_d_n7, assign24560_e27818_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) && (var_guard453 == 0.0)) && (var_guard455 == 0.0)) {
        let assign24560_e27814: f64 = (var_perfc * var_xerfc);
        let assign24560_e27815: f64 = (1.0 - assign24560_e27814);
        let assign24560_e27816: f64 = (1.0 / assign24560_e27815);
        (assign24560_e27816, (-((-(var_perfc * var_xerfc_dn5)) / (assign24560_e27815 * assign24560_e27815))), (-((-(var_perfc * var_xerfc_dn6)) / (assign24560_e27815 * assign24560_e27815))), (-((-(var_perfc * var_xerfc_dn7)) / (assign24560_e27815 * assign24560_e27815))), (-((-(var_perfc * var_xerfc_dn8)) / (assign24560_e27815 * assign24560_e27815))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign24560_e27818;
        var_terfc_dn5 = assign24560_e27818_d_n5;
        var_terfc_dn6 = assign24560_e27818_d_n6;
        var_terfc_dn7 = assign24560_e27818_d_n7;
        var_terfc_dn8 = assign24560_e27818_d_n8;

        let assign24570_e27820: f64 = (-var_ysq);
        let assign24570_e27822: f64 = (assign24570_e27820 + var_mtat);
        let assign24570_e27824: f64 = (-230.25850929940458);
        let assign24570_e27825: f64 = if assign24570_e27822 > assign24570_e27824 { 1.0 } else { 0.0 };
        var_guard456 = assign24570_e27825;

        let (assign24580_e27843, assign24580_e27843_d_n5, assign24580_e27843_d_n6, assign24580_e27843_d_n7, assign24580_e27843_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) && (var_guard453 == 0.0)) && (var_guard456 != 0.0)) {
        let assign24580_e27838: f64 = (-var_ysq);
        let assign24580_e27840: f64 = (assign24580_e27838 + var_mtat);
        let assign24580_e27841: f64 = (assign24580_e27840).exp();
        (assign24580_e27841, (assign24580_e27841 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign24580_e27841 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign24580_e27841 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign24580_e27841 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign24580_e27843;
        var_tmp_dn5 = assign24580_e27843_d_n5;
        var_tmp_dn6 = assign24580_e27843_d_n6;
        var_tmp_dn7 = assign24580_e27843_d_n7;
        var_tmp_dn8 = assign24580_e27843_d_n8;

        let (assign24590_e27892, assign24590_e27892_d_n5, assign24590_e27892_d_n6, assign24590_e27892_d_n7, assign24590_e27892_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) && (var_guard453 == 0.0)) && (var_guard456 == 0.0)) {
        let assign24590_e27859: f64 = (-230.25850929940458);
        let assign24590_e27861: f64 = (-var_ysq);
        let assign24590_e27863: f64 = (assign24590_e27861 + var_mtat);
        let assign24590_e27864: f64 = (assign24590_e27859 - assign24590_e27863);
        let assign24590_e27868: f64 = (-230.25850929940458);
        let assign24590_e27870: f64 = (-var_ysq);
        let assign24590_e27872: f64 = (assign24590_e27870 + var_mtat);
        let assign24590_e27873: f64 = (assign24590_e27868 - assign24590_e27872);
        let assign24590_e27876: f64 = (-230.25850929940458);
        let assign24590_e27878: f64 = (-var_ysq);
        let assign24590_e27880: f64 = (assign24590_e27878 + var_mtat);
        let assign24590_e27881: f64 = (assign24590_e27876 - assign24590_e27880);
        let assign24590_e27883: f64 = (assign24590_e27881 * 0.3333333333333333);
        let assign24590_e27884: f64 = (1.0 + assign24590_e27883);
        let assign24590_e27885: f64 = (assign24590_e27873 * assign24590_e27884);
        let assign24590_e27886: f64 = (0.5 * assign24590_e27885);
        let assign24590_e27887: f64 = (1.0 + assign24590_e27886);
        let assign24590_e27888: f64 = (assign24590_e27864 * assign24590_e27887);
        let assign24590_e27889: f64 = (1.0 + assign24590_e27888);
        let assign24590_e27890: f64 = (1e-100 / assign24590_e27889);
        (assign24590_e27890, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign24590_e27887) + (assign24590_e27864 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign24590_e27884) + (assign24590_e27873 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign24590_e27889 * assign24590_e27889))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign24590_e27887) + (assign24590_e27864 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign24590_e27884) + (assign24590_e27873 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign24590_e27889 * assign24590_e27889))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign24590_e27887) + (assign24590_e27864 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign24590_e27884) + (assign24590_e27873 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign24590_e27889 * assign24590_e27889))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign24590_e27887) + (assign24590_e27864 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign24590_e27884) + (assign24590_e27873 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign24590_e27889 * assign24590_e27889))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign24590_e27892;
        var_tmp_dn5 = assign24590_e27892_d_n5;
        var_tmp_dn6 = assign24590_e27892_d_n6;
        var_tmp_dn7 = assign24590_e27892_d_n7;
        var_tmp_dn8 = assign24590_e27892_d_n8;

        let (assign24600_e27922, assign24600_e27922_d_n5, assign24600_e27922_d_n6, assign24600_e27922_d_n7, assign24600_e27922_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) && (var_guard453 == 0.0)) {
        let assign24600_e27904: f64 = (0.29214664 * var_terfc);
        let assign24600_e27908: f64 = (var_terfc * var_terfc);
        let assign24600_e27909: f64 = (var_berfc * assign24600_e27908);
        let assign24600_e27910: f64 = (assign24600_e27904 + assign24600_e27909);
        let assign24600_e27914: f64 = (var_terfc * var_terfc);
        let assign24600_e27916: f64 = (assign24600_e27914 * var_terfc);
        let assign24600_e27917: f64 = (var_cerfc * assign24600_e27916);
        let assign24600_e27918: f64 = (assign24600_e27910 + assign24600_e27917);
        let assign24600_e27920: f64 = (assign24600_e27918 * var_tmp);
        (assign24600_e27920, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign24600_e27914 * var_terfc_dn5)))) * var_tmp) + (assign24600_e27918 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign24600_e27914 * var_terfc_dn6)))) * var_tmp) + (assign24600_e27918 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign24600_e27914 * var_terfc_dn7)))) * var_tmp) + (assign24600_e27918 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign24600_e27914 * var_terfc_dn8)))) * var_tmp) + (assign24600_e27918 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign24600_e27922;
        var_erfcpos_dn5 = assign24600_e27922_d_n5;
        var_erfcpos_dn6 = assign24600_e27922_d_n6;
        var_erfcpos_dn7 = assign24600_e27922_d_n7;
        var_erfcpos_dn8 = assign24600_e27922_d_n8;

        let assign24610_e27925: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard457 = assign24610_e27925;

        let (assign24620_e27939, assign24620_e27939_d_n5, assign24620_e27939_d_n6, assign24620_e27939_d_n7, assign24620_e27939_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) && (var_guard453 == 0.0)) && (var_guard457 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign24620_e27939;
        var_erfctimesexpmtat_dn5 = assign24620_e27939_d_n5;
        var_erfctimesexpmtat_dn6 = assign24620_e27939_d_n6;
        var_erfctimesexpmtat_dn7 = assign24620_e27939_d_n7;
        var_erfctimesexpmtat_dn8 = assign24620_e27939_d_n8;

        let assign24630_e27942: f64 = (-230.25850929940458);
        let assign24630_e27943: f64 = if var_mtat > assign24630_e27942 { 1.0 } else { 0.0 };
        var_guard458 = assign24630_e27943;

        *var_asrh_slot = var_asrh;
        *var_asrh_dn5_slot = var_asrh_dn5;
        *var_asrh_dn6_slot = var_asrh_dn6;
        *var_asrh_dn7_slot = var_asrh_dn7;
        *var_asrh_dn8_slot = var_asrh_dn8;
        *var_btat_slot = var_btat;
        *var_btat_dn5_slot = var_btat_dn5;
        *var_btat_dn6_slot = var_btat_dn6;
        *var_btat_dn7_slot = var_btat_dn7;
        *var_btat_dn8_slot = var_btat_dn8;
        *var_dwsrh_slot = var_dwsrh;
        *var_erfcpos_slot = var_erfcpos;
        *var_erfcpos_dn5_slot = var_erfcpos_dn5;
        *var_erfcpos_dn6_slot = var_erfcpos_dn6;
        *var_erfcpos_dn7_slot = var_erfcpos_dn7;
        *var_erfcpos_dn8_slot = var_erfcpos_dn8;
        *var_erfctimesexpmtat_slot = var_erfctimesexpmtat;
        *var_erfctimesexpmtat_dn5_slot = var_erfctimesexpmtat_dn5;
        *var_erfctimesexpmtat_dn6_slot = var_erfctimesexpmtat_dn6;
        *var_erfctimesexpmtat_dn7_slot = var_erfctimesexpmtat_dn7;
        *var_erfctimesexpmtat_dn8_slot = var_erfctimesexpmtat_dn8;
        *var_guard451_slot = var_guard451;
        *var_guard452_slot = var_guard452;
        *var_guard453_slot = var_guard453;
        *var_guard454_slot = var_guard454;
        *var_guard455_slot = var_guard455;
        *var_guard456_slot = var_guard456;
        *var_guard457_slot = var_guard457;
        *var_guard458_slot = var_guard458;
        *var_isrh_slot = var_isrh;
        *var_isrh_dn5_slot = var_isrh_dn5;
        *var_isrh_dn6_slot = var_isrh_dn6;
        *var_isrh_dn7_slot = var_isrh_dn7;
        *var_isrh_dn8_slot = var_isrh_dn8;
        *var_itat_slot = var_itat;
        *var_itat_dn5_slot = var_itat_dn5;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_ktat_slot = var_ktat;
        *var_ktat_dn5_slot = var_ktat_dn5;
        *var_ktat_dn6_slot = var_ktat_dn6;
        *var_ktat_dn7_slot = var_ktat_dn7;
        *var_ktat_dn8_slot = var_ktat_dn8;
        *var_ltat_slot = var_ltat;
        *var_ltat_dn5_slot = var_ltat_dn5;
        *var_ltat_dn6_slot = var_ltat_dn6;
        *var_ltat_dn7_slot = var_ltat_dn7;
        *var_ltat_dn8_slot = var_ltat_dn8;
        *var_mtat_slot = var_mtat;
        *var_mtat_dn5_slot = var_mtat_dn5;
        *var_mtat_dn6_slot = var_mtat_dn6;
        *var_mtat_dn7_slot = var_mtat_dn7;
        *var_mtat_dn8_slot = var_mtat_dn8;
        *var_sqrtumax_slot = var_sqrtumax;
        *var_sqrtumax_dn5_slot = var_sqrtumax_dn5;
        *var_sqrtumax_dn6_slot = var_sqrtumax_dn6;
        *var_sqrtumax_dn7_slot = var_sqrtumax_dn7;
        *var_sqrtumax_dn8_slot = var_sqrtumax_dn8;
        *var_terfc_slot = var_terfc;
        *var_terfc_dn5_slot = var_terfc_dn5;
        *var_terfc_dn6_slot = var_terfc_dn6;
        *var_terfc_dn7_slot = var_terfc_dn7;
        *var_terfc_dn8_slot = var_terfc_dn8;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn5_slot = var_tmp_dn5;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_twoatatoverthreebtat_slot = var_twoatatoverthreebtat;
        *var_twoatatoverthreebtat_dn5_slot = var_twoatatoverthreebtat_dn5;
        *var_twoatatoverthreebtat_dn6_slot = var_twoatatoverthreebtat_dn6;
        *var_twoatatoverthreebtat_dn7_slot = var_twoatatoverthreebtat_dn7;
        *var_twoatatoverthreebtat_dn8_slot = var_twoatatoverthreebtat_dn8;
        *var_umax_slot = var_umax;
        *var_umax_dn5_slot = var_umax_dn5;
        *var_umax_dn6_slot = var_umax_dn6;
        *var_umax_dn7_slot = var_umax_dn7;
        *var_umax_dn8_slot = var_umax_dn8;
        *var_umaxbeforelimiting_slot = var_umaxbeforelimiting;
        *var_umaxbeforelimiting_dn5_slot = var_umaxbeforelimiting_dn5;
        *var_umaxbeforelimiting_dn6_slot = var_umaxbeforelimiting_dn6;
        *var_umaxbeforelimiting_dn7_slot = var_umaxbeforelimiting_dn7;
        *var_umaxbeforelimiting_dn8_slot = var_umaxbeforelimiting_dn8;
        *var_umaxpoweronepointfive_slot = var_umaxpoweronepointfive;
        *var_umaxpoweronepointfive_dn5_slot = var_umaxpoweronepointfive_dn5;
        *var_umaxpoweronepointfive_dn6_slot = var_umaxpoweronepointfive_dn6;
        *var_umaxpoweronepointfive_dn7_slot = var_umaxpoweronepointfive_dn7;
        *var_umaxpoweronepointfive_dn8_slot = var_umaxpoweronepointfive_dn8;
        *var_wdep_slot = var_wdep;
        *var_wdep_dn5_slot = var_wdep_dn5;
        *var_wdep_dn6_slot = var_wdep_dn6;
        *var_wdep_dn7_slot = var_wdep_dn7;
        *var_wdep_dn8_slot = var_wdep_dn8;
        *var_wgamma_slot = var_wgamma;
        *var_wgamma_dn5_slot = var_wgamma_dn5;
        *var_wgamma_dn6_slot = var_wgamma_dn6;
        *var_wgamma_dn7_slot = var_wgamma_dn7;
        *var_wgamma_dn8_slot = var_wgamma_dn8;
        *var_wsrh_slot = var_wsrh;
        *var_wsrhstep_slot = var_wsrhstep;
        *var_wtat_slot = var_wtat;
        *var_wtat_dn5_slot = var_wtat_dn5;
        *var_wtat_dn6_slot = var_wtat_dn6;
        *var_wtat_dn7_slot = var_wtat_dn7;
        *var_wtat_dn8_slot = var_wtat_dn8;
        *var_xerfc_slot = var_xerfc;
        *var_xerfc_dn5_slot = var_xerfc_dn5;
        *var_xerfc_dn6_slot = var_xerfc_dn6;
        *var_xerfc_dn7_slot = var_xerfc_dn7;
        *var_xerfc_dn8_slot = var_xerfc_dn8;
        *var_ysq_slot = var_ysq;
        *var_ysq_dn5_slot = var_ysq_dn5;
        *var_ysq_dn6_slot = var_ysq_dn6;
        *var_ysq_dn7_slot = var_ysq_dn7;
        *var_ysq_dn8_slot = var_ysq_dn8;
    }

    pub(super) fn stamp_transient_block_46(
        p: &Parameters,
        var_absource_i: f64,
        var_alphaav: f64,
        var_asrh: f64,
        var_asrh_dn5: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_atatgat: f64,
        var_erfcpos: f64,
        var_erfcpos_dn5: f64,
        var_erfcpos_dn6: f64,
        var_erfcpos_dn7: f64,
        var_erfcpos_dn8: f64,
        var_fbbtgat: f64,
        var_fbbtgat_dn5: f64,
        var_fbbtgat_dn6: f64,
        var_fbbtgat_dn7: f64,
        var_fbbtgat_dn8: f64,
        var_fstopgat: f64,
        var_guard176: f64,
        var_guard193: f64,
        var_guard449: f64,
        var_guard453: f64,
        var_guard457: f64,
        var_guard458: f64,
        var_id__blk213: f64,
        var_ijunbot: f64,
        var_ijunbot_dn5: f64,
        var_ijunbot_dn6: f64,
        var_ijunbot_dn7: f64,
        var_ijunbot_dn8: f64,
        var_ijunsti: f64,
        var_ijunsti_dn5: f64,
        var_ijunsti_dn6: f64,
        var_ijunsti_dn7: f64,
        var_ijunsti_dn8: f64,
        var_isrh: f64,
        var_isrh_dn5: f64,
        var_isrh_dn6: f64,
        var_isrh_dn7: f64,
        var_isrh_dn8: f64,
        var_ktat: f64,
        var_ktat_dn5: f64,
        var_ktat_dn6: f64,
        var_ktat_dn7: f64,
        var_ktat_dn8: f64,
        var_lgsource_i: f64,
        var_lssource_i: f64,
        var_mtat: f64,
        var_mtat_dn5: f64,
        var_mtat_dn6: f64,
        var_mtat_dn7: f64,
        var_mtat_dn8: f64,
        var_one_over_one_minus_pgat: f64,
        var_phitdinv: f64,
        var_slopegat: f64,
        var_slopegat_dn5: f64,
        var_slopegat_dn6: f64,
        var_slopegat_dn7: f64,
        var_slopegat_dn8: f64,
        var_v4: f64,
        var_v5: f64,
        var_vav: f64,
        var_vbirgatinv: f64,
        var_vbrinvgat: f64,
        var_vbrinvgat_dn5: f64,
        var_vbrinvgat_dn6: f64,
        var_vbrinvgat_dn7: f64,
        var_vbrinvgat_dn8: f64,
        var_vmax_s: f64,
        var_wdepnulrinvgat: f64,
        var_wtat: f64,
        var_wtat_dn5: f64,
        var_wtat_dn6: f64,
        var_wtat_dn7: f64,
        var_wtat_dn8: f64,
        var_erfctimesexpmtat_slot: &mut f64,
        var_erfctimesexpmtat_dn5_slot: &mut f64,
        var_erfctimesexpmtat_dn6_slot: &mut f64,
        var_erfctimesexpmtat_dn7_slot: &mut f64,
        var_erfctimesexpmtat_dn8_slot: &mut f64,
        var_fbreakdown_slot: &mut f64,
        var_fbreakdown_dn5_slot: &mut f64,
        var_fbreakdown_dn6_slot: &mut f64,
        var_fbreakdown_dn7_slot: &mut f64,
        var_fbreakdown_dn8_slot: &mut f64,
        var_fmaxr_slot: &mut f64,
        var_fmaxr_dn5_slot: &mut f64,
        var_fmaxr_dn6_slot: &mut f64,
        var_fmaxr_dn7_slot: &mut f64,
        var_fmaxr_dn8_slot: &mut f64,
        var_gammamax_slot: &mut f64,
        var_gammamax_dn5_slot: &mut f64,
        var_gammamax_dn6_slot: &mut f64,
        var_gammamax_dn7_slot: &mut f64,
        var_gammamax_dn8_slot: &mut f64,
        var_guard459_slot: &mut f64,
        var_guard460_slot: &mut f64,
        var_guard461_slot: &mut f64,
        var_guard462_slot: &mut f64,
        var_guard463_slot: &mut f64,
        var_guard464_slot: &mut f64,
        var_guard465_slot: &mut f64,
        var_guard466_slot: &mut f64,
        var_guard467_slot: &mut f64,
        var_guard468_slot: &mut f64,
        var_guard469_slot: &mut f64,
        var_i4_slot: &mut f64,
        var_i4_dn5_slot: &mut f64,
        var_i4_dn6_slot: &mut f64,
        var_i4_dn7_slot: &mut f64,
        var_i4_dn8_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn5_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
        var_ijungat_slot: &mut f64,
        var_ijungat_dn5_slot: &mut f64,
        var_ijungat_dn6_slot: &mut f64,
        var_ijungat_dn7_slot: &mut f64,
        var_ijungat_dn8_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn5_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn5_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_two_psistar_slot: &mut f64,
        var_vbbt_slot: &mut f64,
        var_z_slot: &mut f64,
    ) {
        let mut var_erfctimesexpmtat: f64 = *var_erfctimesexpmtat_slot;
        let mut var_erfctimesexpmtat_dn5: f64 = *var_erfctimesexpmtat_dn5_slot;
        let mut var_erfctimesexpmtat_dn6: f64 = *var_erfctimesexpmtat_dn6_slot;
        let mut var_erfctimesexpmtat_dn7: f64 = *var_erfctimesexpmtat_dn7_slot;
        let mut var_erfctimesexpmtat_dn8: f64 = *var_erfctimesexpmtat_dn8_slot;
        let mut var_fbreakdown: f64 = *var_fbreakdown_slot;
        let mut var_fbreakdown_dn5: f64 = *var_fbreakdown_dn5_slot;
        let mut var_fbreakdown_dn6: f64 = *var_fbreakdown_dn6_slot;
        let mut var_fbreakdown_dn7: f64 = *var_fbreakdown_dn7_slot;
        let mut var_fbreakdown_dn8: f64 = *var_fbreakdown_dn8_slot;
        let mut var_fmaxr: f64 = *var_fmaxr_slot;
        let mut var_fmaxr_dn5: f64 = *var_fmaxr_dn5_slot;
        let mut var_fmaxr_dn6: f64 = *var_fmaxr_dn6_slot;
        let mut var_fmaxr_dn7: f64 = *var_fmaxr_dn7_slot;
        let mut var_fmaxr_dn8: f64 = *var_fmaxr_dn8_slot;
        let mut var_gammamax: f64 = *var_gammamax_slot;
        let mut var_gammamax_dn5: f64 = *var_gammamax_dn5_slot;
        let mut var_gammamax_dn6: f64 = *var_gammamax_dn6_slot;
        let mut var_gammamax_dn7: f64 = *var_gammamax_dn7_slot;
        let mut var_gammamax_dn8: f64 = *var_gammamax_dn8_slot;
        let mut var_guard459: f64 = *var_guard459_slot;
        let mut var_guard460: f64 = *var_guard460_slot;
        let mut var_guard461: f64 = *var_guard461_slot;
        let mut var_guard462: f64 = *var_guard462_slot;
        let mut var_guard463: f64 = *var_guard463_slot;
        let mut var_guard464: f64 = *var_guard464_slot;
        let mut var_guard465: f64 = *var_guard465_slot;
        let mut var_guard466: f64 = *var_guard466_slot;
        let mut var_guard467: f64 = *var_guard467_slot;
        let mut var_guard468: f64 = *var_guard468_slot;
        let mut var_guard469: f64 = *var_guard469_slot;
        let mut var_i4: f64 = *var_i4_slot;
        let mut var_i4_dn5: f64 = *var_i4_dn5_slot;
        let mut var_i4_dn6: f64 = *var_i4_dn6_slot;
        let mut var_i4_dn7: f64 = *var_i4_dn7_slot;
        let mut var_i4_dn8: f64 = *var_i4_dn8_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn5: f64 = *var_ibbt_dn5_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
        let mut var_ijungat: f64 = *var_ijungat_slot;
        let mut var_ijungat_dn5: f64 = *var_ijungat_dn5_slot;
        let mut var_ijungat_dn6: f64 = *var_ijungat_dn6_slot;
        let mut var_ijungat_dn7: f64 = *var_ijungat_dn7_slot;
        let mut var_ijungat_dn8: f64 = *var_ijungat_dn8_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn5: f64 = *var_itat_dn5_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn5: f64 = *var_tmp_dn5_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_two_psistar: f64 = *var_two_psistar_slot;
        let mut var_vbbt: f64 = *var_vbbt_slot;
        let mut var_z: f64 = *var_z_slot;

        let (assign24640_e27961, assign24640_e27961_d_n5, assign24640_e27961_d_n6, assign24640_e27961_d_n7, assign24640_e27961_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) && (var_guard453 == 0.0)) && (var_guard457 == 0.0)) && (var_guard458 != 0.0)) {
        let assign24640_e27959: f64 = (var_mtat).exp();
        (assign24640_e27959, (assign24640_e27959 * var_mtat_dn5), (assign24640_e27959 * var_mtat_dn6), (assign24640_e27959 * var_mtat_dn7), (assign24640_e27959 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign24640_e27961;
        var_tmp_dn5 = assign24640_e27961_d_n5;
        var_tmp_dn6 = assign24640_e27961_d_n6;
        var_tmp_dn7 = assign24640_e27961_d_n7;
        var_tmp_dn8 = assign24640_e27961_d_n8;

        let (assign24650_e28004, assign24650_e28004_d_n5, assign24650_e28004_d_n6, assign24650_e28004_d_n7, assign24650_e28004_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) && (var_guard453 == 0.0)) && (var_guard457 == 0.0)) && (var_guard458 == 0.0)) {
        let assign24650_e27980: f64 = (-230.25850929940458);
        let assign24650_e27982: f64 = (assign24650_e27980 - var_mtat);
        let assign24650_e27986: f64 = (-230.25850929940458);
        let assign24650_e27988: f64 = (assign24650_e27986 - var_mtat);
        let assign24650_e27991: f64 = (-230.25850929940458);
        let assign24650_e27993: f64 = (assign24650_e27991 - var_mtat);
        let assign24650_e27995: f64 = (assign24650_e27993 * 0.3333333333333333);
        let assign24650_e27996: f64 = (1.0 + assign24650_e27995);
        let assign24650_e27997: f64 = (assign24650_e27988 * assign24650_e27996);
        let assign24650_e27998: f64 = (0.5 * assign24650_e27997);
        let assign24650_e27999: f64 = (1.0 + assign24650_e27998);
        let assign24650_e28000: f64 = (assign24650_e27982 * assign24650_e27999);
        let assign24650_e28001: f64 = (1.0 + assign24650_e28000);
        let assign24650_e28002: f64 = (1e-100 / assign24650_e28001);
        (assign24650_e28002, (-((1e-100 * (((-var_mtat_dn5) * assign24650_e27999) + (assign24650_e27982 * (0.5 * (((-var_mtat_dn5) * assign24650_e27996) + (assign24650_e27988 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign24650_e28001 * assign24650_e28001))), (-((1e-100 * (((-var_mtat_dn6) * assign24650_e27999) + (assign24650_e27982 * (0.5 * (((-var_mtat_dn6) * assign24650_e27996) + (assign24650_e27988 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign24650_e28001 * assign24650_e28001))), (-((1e-100 * (((-var_mtat_dn7) * assign24650_e27999) + (assign24650_e27982 * (0.5 * (((-var_mtat_dn7) * assign24650_e27996) + (assign24650_e27988 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign24650_e28001 * assign24650_e28001))), (-((1e-100 * (((-var_mtat_dn8) * assign24650_e27999) + (assign24650_e27982 * (0.5 * (((-var_mtat_dn8) * assign24650_e27996) + (assign24650_e27988 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign24650_e28001 * assign24650_e28001))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign24650_e28004;
        var_tmp_dn5 = assign24650_e28004_d_n5;
        var_tmp_dn6 = assign24650_e28004_d_n6;
        var_tmp_dn7 = assign24650_e28004_d_n7;
        var_tmp_dn8 = assign24650_e28004_d_n8;

        let (assign24660_e28023, assign24660_e28023_d_n5, assign24660_e28023_d_n6, assign24660_e28023_d_n7, assign24660_e28023_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) && (var_guard453 == 0.0)) && (var_guard457 == 0.0)) {
        let assign24660_e28019: f64 = (2.0 * var_tmp);
        let assign24660_e28021: f64 = (assign24660_e28019 - var_erfcpos);
        (assign24660_e28021, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign24660_e28023;
        var_erfctimesexpmtat_dn5 = assign24660_e28023_d_n5;
        var_erfctimesexpmtat_dn6 = assign24660_e28023_d_n6;
        var_erfctimesexpmtat_dn7 = assign24660_e28023_d_n7;
        var_erfctimesexpmtat_dn8 = assign24660_e28023_d_n8;

        let (assign24670_e28043, assign24670_e28043_d_n5, assign24670_e28043_d_n6, assign24670_e28043_d_n7, assign24670_e28043_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) && (var_guard453 == 0.0)) {
        let assign24670_e28035: f64 = (1.772453850905516 * 0.5);
        let assign24670_e28038: f64 = (var_atatgat * var_erfctimesexpmtat);
        let assign24670_e28040: f64 = (assign24670_e28038 / var_ktat);
        let assign24670_e28041: f64 = (assign24670_e28035 * assign24670_e28040);
        (assign24670_e28041, (assign24670_e28035 * ((((var_atatgat * var_erfctimesexpmtat_dn5) * var_ktat) - (assign24670_e28038 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign24670_e28035 * ((((var_atatgat * var_erfctimesexpmtat_dn6) * var_ktat) - (assign24670_e28038 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign24670_e28035 * ((((var_atatgat * var_erfctimesexpmtat_dn7) * var_ktat) - (assign24670_e28038 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign24670_e28035 * ((((var_atatgat * var_erfctimesexpmtat_dn8) * var_ktat) - (assign24670_e28038 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign24670_e28043;
        var_gammamax_dn5 = assign24670_e28043_d_n5;
        var_gammamax_dn6 = assign24670_e28043_d_n6;
        var_gammamax_dn7 = assign24670_e28043_d_n7;
        var_gammamax_dn8 = assign24670_e28043_d_n8;

        let (assign24680_e28061, assign24680_e28061_d_n5, assign24680_e28061_d_n6, assign24680_e28061_d_n7, assign24680_e28061_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) && (var_guard453 == 0.0)) {
        let assign24680_e28056: f64 = (var_asrh * var_gammamax);
        let assign24680_e28058: f64 = (assign24680_e28056 * var_wtat);
        let assign24680_e28059: f64 = (p.p840 * assign24680_e28058);
        (assign24680_e28059, (p.p840 * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign24680_e28056 * var_wtat_dn5))), (p.p840 * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign24680_e28056 * var_wtat_dn6))), (p.p840 * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign24680_e28056 * var_wtat_dn7))), (p.p840 * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign24680_e28056 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign24680_e28061;
        var_itat_dn5 = assign24680_e28061_d_n5;
        var_itat_dn6 = assign24680_e28061_d_n6;
        var_itat_dn7 = assign24680_e28061_d_n7;
        var_itat_dn8 = assign24680_e28061_d_n8;

        let assign24690_e28064: f64 = if p.p846 == 0.0 { 1.0 } else { 0.0 };
        var_guard459 = assign24690_e28064;

        let (assign24700_e28075, assign24700_e28075_d_n5, assign24700_e28075_d_n6, assign24700_e28075_d_n7, assign24700_e28075_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) && (var_guard459 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign24700_e28075;
        var_ibbt_dn5 = assign24700_e28075_d_n5;
        var_ibbt_dn6 = assign24700_e28075_d_n6;
        var_ibbt_dn7 = assign24700_e28075_d_n7;
        var_ibbt_dn8 = assign24700_e28075_d_n8;

        let assign24710_e28078: f64 = if p.p826 == 0.5 { 1.0 } else { 0.0 };
        var_guard460 = assign24710_e28078;

        let (assign24720_e28097, assign24720_e28097_d_n5, assign24720_e28097_d_n6, assign24720_e28097_d_n7, assign24720_e28097_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) && (var_guard459 == 0.0)) && (var_guard460 != 0.0)) {
        let assign24720_e28092: f64 = (p.p823 - var_vbbt);
        let assign24720_e28094: f64 = (assign24720_e28092 * var_vbirgatinv);
        let assign24720_e28095: f64 = (assign24720_e28094).sqrt();
        (assign24720_e28095, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign24720_e28097;
        var_tmp_dn5 = assign24720_e28097_d_n5;
        var_tmp_dn6 = assign24720_e28097_d_n6;
        var_tmp_dn7 = assign24720_e28097_d_n7;
        var_tmp_dn8 = assign24720_e28097_d_n8;

        let (assign24730_e28118, assign24730_e28118_d_n5, assign24730_e28118_d_n6, assign24730_e28118_d_n7, assign24730_e28118_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) && (var_guard459 == 0.0)) && (var_guard460 == 0.0)) {
        let assign24730_e28112: f64 = (p.p823 - var_vbbt);
        let assign24730_e28114: f64 = (assign24730_e28112 * var_vbirgatinv);
        let assign24730_e28116: f64 = (assign24730_e28114).powf(p.p826);
        (assign24730_e28116, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign24730_e28118;
        var_tmp_dn5 = assign24730_e28118_d_n5;
        var_tmp_dn6 = assign24730_e28118_d_n6;
        var_tmp_dn7 = assign24730_e28118_d_n7;
        var_tmp_dn8 = assign24730_e28118_d_n8;

        let (assign24740_e28138, assign24740_e28138_d_n5, assign24740_e28138_d_n6, assign24740_e28138_d_n7, assign24740_e28138_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) && (var_guard459 == 0.0)) {
        let assign24740_e28131: f64 = (p.p823 - var_vbbt);
        let assign24740_e28133: f64 = (assign24740_e28131 * var_wdepnulrinvgat);
        let assign24740_e28135: f64 = (assign24740_e28133 / var_tmp);
        let assign24740_e28136: f64 = (var_one_over_one_minus_pgat * assign24740_e28135);
        (assign24740_e28136, (var_one_over_one_minus_pgat * (-((assign24740_e28133 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat * (-((assign24740_e28133 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat * (-((assign24740_e28133 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat * (-((assign24740_e28133 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign24740_e28138;
        var_fmaxr_dn5 = assign24740_e28138_d_n5;
        var_fmaxr_dn6 = assign24740_e28138_d_n6;
        var_fmaxr_dn7 = assign24740_e28138_d_n7;
        var_fmaxr_dn8 = assign24740_e28138_d_n8;

        let assign24750_e28140: f64 = (-var_fbbtgat);
        let assign24750_e28142: f64 = (assign24750_e28140 / var_fmaxr);
        let assign24750_e28143: f64 = (assign24750_e28142).abs();
        let assign24750_e28145: f64 = if assign24750_e28143 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard461 = assign24750_e28145;

        let (assign24760_e28163, assign24760_e28163_d_n5, assign24760_e28163_d_n6, assign24760_e28163_d_n7, assign24760_e28163_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) && (var_guard459 == 0.0)) && (var_guard461 != 0.0)) {
        let assign24760_e28158: f64 = (-var_fbbtgat);
        let assign24760_e28160: f64 = (assign24760_e28158 / var_fmaxr);
        let assign24760_e28161: f64 = (assign24760_e28160).exp();
        (assign24760_e28161, (assign24760_e28161 * ((((-var_fbbtgat_dn5) * var_fmaxr) - (assign24760_e28158 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))), (assign24760_e28161 * ((((-var_fbbtgat_dn6) * var_fmaxr) - (assign24760_e28158 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))), (assign24760_e28161 * ((((-var_fbbtgat_dn7) * var_fmaxr) - (assign24760_e28158 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))), (assign24760_e28161 * ((((-var_fbbtgat_dn8) * var_fmaxr) - (assign24760_e28158 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign24760_e28163;
        var_tmp_dn5 = assign24760_e28163_d_n5;
        var_tmp_dn6 = assign24760_e28163_d_n6;
        var_tmp_dn7 = assign24760_e28163_d_n7;
        var_tmp_dn8 = assign24760_e28163_d_n8;

        let assign24770_e28165: f64 = (-var_fbbtgat);
        let assign24770_e28167: f64 = (assign24770_e28165 / var_fmaxr);
        let assign24770_e28169: f64 = if assign24770_e28167 < 0.0 { 1.0 } else { 0.0 };
        var_guard462 = assign24770_e28169;

        let (assign24780_e28220, assign24780_e28220_d_n5, assign24780_e28220_d_n6, assign24780_e28220_d_n7, assign24780_e28220_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) && (var_guard459 == 0.0)) && (var_guard461 == 0.0)) && (var_guard462 != 0.0)) {
        let assign24780_e28187: f64 = (-230.25850929940458);
        let assign24780_e28189: f64 = (-var_fbbtgat);
        let assign24780_e28191: f64 = (assign24780_e28189 / var_fmaxr);
        let assign24780_e28192: f64 = (assign24780_e28187 - assign24780_e28191);
        let assign24780_e28196: f64 = (-230.25850929940458);
        let assign24780_e28198: f64 = (-var_fbbtgat);
        let assign24780_e28200: f64 = (assign24780_e28198 / var_fmaxr);
        let assign24780_e28201: f64 = (assign24780_e28196 - assign24780_e28200);
        let assign24780_e28204: f64 = (-230.25850929940458);
        let assign24780_e28206: f64 = (-var_fbbtgat);
        let assign24780_e28208: f64 = (assign24780_e28206 / var_fmaxr);
        let assign24780_e28209: f64 = (assign24780_e28204 - assign24780_e28208);
        let assign24780_e28211: f64 = (assign24780_e28209 * 0.3333333333333333);
        let assign24780_e28212: f64 = (1.0 + assign24780_e28211);
        let assign24780_e28213: f64 = (assign24780_e28201 * assign24780_e28212);
        let assign24780_e28214: f64 = (0.5 * assign24780_e28213);
        let assign24780_e28215: f64 = (1.0 + assign24780_e28214);
        let assign24780_e28216: f64 = (assign24780_e28192 * assign24780_e28215);
        let assign24780_e28217: f64 = (1.0 + assign24780_e28216);
        let assign24780_e28218: f64 = (1e-100 / assign24780_e28217);
        (assign24780_e28218, (-((1e-100 * (((-((((-var_fbbtgat_dn5) * var_fmaxr) - (assign24780_e28189 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * assign24780_e28215) + (assign24780_e28192 * (0.5 * (((-((((-var_fbbtgat_dn5) * var_fmaxr) - (assign24780_e28198 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * assign24780_e28212) + (assign24780_e28201 * ((-((((-var_fbbtgat_dn5) * var_fmaxr) - (assign24780_e28206 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign24780_e28217 * assign24780_e28217))), (-((1e-100 * (((-((((-var_fbbtgat_dn6) * var_fmaxr) - (assign24780_e28189 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign24780_e28215) + (assign24780_e28192 * (0.5 * (((-((((-var_fbbtgat_dn6) * var_fmaxr) - (assign24780_e28198 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign24780_e28212) + (assign24780_e28201 * ((-((((-var_fbbtgat_dn6) * var_fmaxr) - (assign24780_e28206 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign24780_e28217 * assign24780_e28217))), (-((1e-100 * (((-((((-var_fbbtgat_dn7) * var_fmaxr) - (assign24780_e28189 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign24780_e28215) + (assign24780_e28192 * (0.5 * (((-((((-var_fbbtgat_dn7) * var_fmaxr) - (assign24780_e28198 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign24780_e28212) + (assign24780_e28201 * ((-((((-var_fbbtgat_dn7) * var_fmaxr) - (assign24780_e28206 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign24780_e28217 * assign24780_e28217))), (-((1e-100 * (((-((((-var_fbbtgat_dn8) * var_fmaxr) - (assign24780_e28189 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign24780_e28215) + (assign24780_e28192 * (0.5 * (((-((((-var_fbbtgat_dn8) * var_fmaxr) - (assign24780_e28198 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign24780_e28212) + (assign24780_e28201 * ((-((((-var_fbbtgat_dn8) * var_fmaxr) - (assign24780_e28206 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign24780_e28217 * assign24780_e28217))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign24780_e28220;
        var_tmp_dn5 = assign24780_e28220_d_n5;
        var_tmp_dn6 = assign24780_e28220_d_n6;
        var_tmp_dn7 = assign24780_e28220_d_n7;
        var_tmp_dn8 = assign24780_e28220_d_n8;

        let (assign24790_e28269, assign24790_e28269_d_n5, assign24790_e28269_d_n6, assign24790_e28269_d_n7, assign24790_e28269_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) && (var_guard459 == 0.0)) && (var_guard461 == 0.0)) && (var_guard462 == 0.0)) {
        let assign24790_e28239: f64 = (-var_fbbtgat);
        let assign24790_e28241: f64 = (assign24790_e28239 / var_fmaxr);
        let assign24790_e28243: f64 = (assign24790_e28241 - 230.25850929940458);
        let assign24790_e28247: f64 = (-var_fbbtgat);
        let assign24790_e28249: f64 = (assign24790_e28247 / var_fmaxr);
        let assign24790_e28251: f64 = (assign24790_e28249 - 230.25850929940458);
        let assign24790_e28254: f64 = (-var_fbbtgat);
        let assign24790_e28256: f64 = (assign24790_e28254 / var_fmaxr);
        let assign24790_e28258: f64 = (assign24790_e28256 - 230.25850929940458);
        let assign24790_e28260: f64 = (assign24790_e28258 * 0.3333333333333333);
        let assign24790_e28261: f64 = (1.0 + assign24790_e28260);
        let assign24790_e28262: f64 = (assign24790_e28251 * assign24790_e28261);
        let assign24790_e28263: f64 = (0.5 * assign24790_e28262);
        let assign24790_e28264: f64 = (1.0 + assign24790_e28263);
        let assign24790_e28265: f64 = (assign24790_e28243 * assign24790_e28264);
        let assign24790_e28266: f64 = (1.0 + assign24790_e28265);
        let assign24790_e28267: f64 = (1e100 * assign24790_e28266);
        (assign24790_e28267, (1e100 * ((((((-var_fbbtgat_dn5) * var_fmaxr) - (assign24790_e28239 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * assign24790_e28264) + (assign24790_e28243 * (0.5 * ((((((-var_fbbtgat_dn5) * var_fmaxr) - (assign24790_e28247 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * assign24790_e28261) + (assign24790_e28251 * (((((-var_fbbtgat_dn5) * var_fmaxr) - (assign24790_e28254 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_dn6) * var_fmaxr) - (assign24790_e28239 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign24790_e28264) + (assign24790_e28243 * (0.5 * ((((((-var_fbbtgat_dn6) * var_fmaxr) - (assign24790_e28247 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign24790_e28261) + (assign24790_e28251 * (((((-var_fbbtgat_dn6) * var_fmaxr) - (assign24790_e28254 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_dn7) * var_fmaxr) - (assign24790_e28239 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign24790_e28264) + (assign24790_e28243 * (0.5 * ((((((-var_fbbtgat_dn7) * var_fmaxr) - (assign24790_e28247 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign24790_e28261) + (assign24790_e28251 * (((((-var_fbbtgat_dn7) * var_fmaxr) - (assign24790_e28254 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_dn8) * var_fmaxr) - (assign24790_e28239 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign24790_e28264) + (assign24790_e28243 * (0.5 * ((((((-var_fbbtgat_dn8) * var_fmaxr) - (assign24790_e28247 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign24790_e28261) + (assign24790_e28251 * (((((-var_fbbtgat_dn8) * var_fmaxr) - (assign24790_e28254 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign24790_e28269;
        var_tmp_dn5 = assign24790_e28269_d_n5;
        var_tmp_dn6 = assign24790_e28269_d_n6;
        var_tmp_dn7 = assign24790_e28269_d_n7;
        var_tmp_dn8 = assign24790_e28269_d_n8;

        let (assign24800_e28289, assign24800_e28289_d_n5, assign24800_e28289_d_n6, assign24800_e28289_d_n7, assign24800_e28289_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) && (var_guard459 == 0.0)) {
        let assign24800_e28282: f64 = (var_v4 * var_fmaxr);
        let assign24800_e28284: f64 = (assign24800_e28282 * var_fmaxr);
        let assign24800_e28286: f64 = (assign24800_e28284 * var_tmp);
        let assign24800_e28287: f64 = (p.p846 * assign24800_e28286);
        (assign24800_e28287, (p.p846 * (((((var_v4 * var_fmaxr_dn5) * var_fmaxr) + (assign24800_e28282 * var_fmaxr_dn5)) * var_tmp) + (assign24800_e28284 * var_tmp_dn5))), (p.p846 * (((((var_v4 * var_fmaxr_dn6) * var_fmaxr) + (assign24800_e28282 * var_fmaxr_dn6)) * var_tmp) + (assign24800_e28284 * var_tmp_dn6))), (p.p846 * (((((var_v4 * var_fmaxr_dn7) * var_fmaxr) + (assign24800_e28282 * var_fmaxr_dn7)) * var_tmp) + (assign24800_e28284 * var_tmp_dn7))), (p.p846 * (((((var_v4 * var_fmaxr_dn8) * var_fmaxr) + (assign24800_e28282 * var_fmaxr_dn8)) * var_tmp) + (assign24800_e28284 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign24800_e28289;
        var_ibbt_dn5 = assign24800_e28289_d_n5;
        var_ibbt_dn6 = assign24800_e28289_d_n6;
        var_ibbt_dn7 = assign24800_e28289_d_n7;
        var_ibbt_dn8 = assign24800_e28289_d_n8;

        let assign24810_e28292: f64 = if p.p855 > 1000.0 { 1.0 } else { 0.0 };
        var_guard463 = assign24810_e28292;

        let (assign24820_e28303, assign24820_e28303_d_n5, assign24820_e28303_d_n6, assign24820_e28303_d_n7, assign24820_e28303_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) && (var_guard463 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign24820_e28303;
        var_fbreakdown_dn5 = assign24820_e28303_d_n5;
        var_fbreakdown_dn6 = assign24820_e28303_d_n6;
        var_fbreakdown_dn7 = assign24820_e28303_d_n7;
        var_fbreakdown_dn8 = assign24820_e28303_d_n8;

        let assign24830_e28306: f64 = (-var_alphaav);
        let assign24830_e28308: f64 = (assign24830_e28306 * p.p855);
        let assign24830_e28309: f64 = if var_vav > assign24830_e28308 { 1.0 } else { 0.0 };
        var_guard464 = assign24830_e28309;

        let assign24840_e28312: f64 = if p.p858 == 4.0 { 1.0 } else { 0.0 };
        var_guard465 = assign24840_e28312;

        let (assign24850_e28342, assign24850_e28342_d_n5, assign24850_e28342_d_n6, assign24850_e28342_d_n7, assign24850_e28342_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) && (var_guard463 == 0.0)) && (var_guard464 != 0.0)) && (var_guard465 != 0.0)) {
        let assign24850_e28328: f64 = (var_vav * var_vbrinvgat);
        let assign24850_e28331: f64 = (var_vav * var_vbrinvgat);
        let assign24850_e28332: f64 = (assign24850_e28328 * assign24850_e28331);
        let assign24850_e28335: f64 = (var_vav * var_vbrinvgat);
        let assign24850_e28336: f64 = (assign24850_e28332 * assign24850_e28335);
        let assign24850_e28339: f64 = (var_vav * var_vbrinvgat);
        let assign24850_e28340: f64 = (assign24850_e28336 * assign24850_e28339);
        (assign24850_e28340, (((((((var_vav * var_vbrinvgat_dn5) * assign24850_e28331) + (assign24850_e28328 * (var_vav * var_vbrinvgat_dn5))) * assign24850_e28335) + (assign24850_e28332 * (var_vav * var_vbrinvgat_dn5))) * assign24850_e28339) + (assign24850_e28336 * (var_vav * var_vbrinvgat_dn5))), (((((((var_vav * var_vbrinvgat_dn6) * assign24850_e28331) + (assign24850_e28328 * (var_vav * var_vbrinvgat_dn6))) * assign24850_e28335) + (assign24850_e28332 * (var_vav * var_vbrinvgat_dn6))) * assign24850_e28339) + (assign24850_e28336 * (var_vav * var_vbrinvgat_dn6))), (((((((var_vav * var_vbrinvgat_dn7) * assign24850_e28331) + (assign24850_e28328 * (var_vav * var_vbrinvgat_dn7))) * assign24850_e28335) + (assign24850_e28332 * (var_vav * var_vbrinvgat_dn7))) * assign24850_e28339) + (assign24850_e28336 * (var_vav * var_vbrinvgat_dn7))), (((((((var_vav * var_vbrinvgat_dn8) * assign24850_e28331) + (assign24850_e28328 * (var_vav * var_vbrinvgat_dn8))) * assign24850_e28335) + (assign24850_e28332 * (var_vav * var_vbrinvgat_dn8))) * assign24850_e28339) + (assign24850_e28336 * (var_vav * var_vbrinvgat_dn8))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign24850_e28342;
        var_tmp_dn5 = assign24850_e28342_d_n5;
        var_tmp_dn6 = assign24850_e28342_d_n6;
        var_tmp_dn7 = assign24850_e28342_d_n7;
        var_tmp_dn8 = assign24850_e28342_d_n8;

        let (assign24860_e28364, assign24860_e28364_d_n5, assign24860_e28364_d_n6, assign24860_e28364_d_n7, assign24860_e28364_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) && (var_guard463 == 0.0)) && (var_guard464 != 0.0)) && (var_guard465 == 0.0)) {
        let assign24860_e28359: f64 = (var_vav * var_vbrinvgat);
        let assign24860_e28360: f64 = (assign24860_e28359).abs();
        let assign24860_e28362: f64 = (assign24860_e28360).powf(p.p858);
        (assign24860_e28362, if 0.0 == 0.0 && ((p.p858) as f64).is_finite() && ((p.p858) as f64).fract() == 0.0 { if p.p858 == 0.0 { 0.0 } else { (p.p858 * ((assign24860_e28360).powf(p.p858 - 1.0) * if assign24860_e28359 >= 0.0 { (var_vav * var_vbrinvgat_dn5) } else { (-(var_vav * var_vbrinvgat_dn5)) })) } } else { (assign24860_e28362 * (p.p858 * (if assign24860_e28359 >= 0.0 { (var_vav * var_vbrinvgat_dn5) } else { (-(var_vav * var_vbrinvgat_dn5)) } / assign24860_e28360))) }, if 0.0 == 0.0 && ((p.p858) as f64).is_finite() && ((p.p858) as f64).fract() == 0.0 { if p.p858 == 0.0 { 0.0 } else { (p.p858 * ((assign24860_e28360).powf(p.p858 - 1.0) * if assign24860_e28359 >= 0.0 { (var_vav * var_vbrinvgat_dn6) } else { (-(var_vav * var_vbrinvgat_dn6)) })) } } else { (assign24860_e28362 * (p.p858 * (if assign24860_e28359 >= 0.0 { (var_vav * var_vbrinvgat_dn6) } else { (-(var_vav * var_vbrinvgat_dn6)) } / assign24860_e28360))) }, if 0.0 == 0.0 && ((p.p858) as f64).is_finite() && ((p.p858) as f64).fract() == 0.0 { if p.p858 == 0.0 { 0.0 } else { (p.p858 * ((assign24860_e28360).powf(p.p858 - 1.0) * if assign24860_e28359 >= 0.0 { (var_vav * var_vbrinvgat_dn7) } else { (-(var_vav * var_vbrinvgat_dn7)) })) } } else { (assign24860_e28362 * (p.p858 * (if assign24860_e28359 >= 0.0 { (var_vav * var_vbrinvgat_dn7) } else { (-(var_vav * var_vbrinvgat_dn7)) } / assign24860_e28360))) }, if 0.0 == 0.0 && ((p.p858) as f64).is_finite() && ((p.p858) as f64).fract() == 0.0 { if p.p858 == 0.0 { 0.0 } else { (p.p858 * ((assign24860_e28360).powf(p.p858 - 1.0) * if assign24860_e28359 >= 0.0 { (var_vav * var_vbrinvgat_dn8) } else { (-(var_vav * var_vbrinvgat_dn8)) })) } } else { (assign24860_e28362 * (p.p858 * (if assign24860_e28359 >= 0.0 { (var_vav * var_vbrinvgat_dn8) } else { (-(var_vav * var_vbrinvgat_dn8)) } / assign24860_e28360))) },)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign24860_e28364;
        var_tmp_dn5 = assign24860_e28364_d_n5;
        var_tmp_dn6 = assign24860_e28364_d_n6;
        var_tmp_dn7 = assign24860_e28364_d_n7;
        var_tmp_dn8 = assign24860_e28364_d_n8;

        let (assign24870_e28382, assign24870_e28382_d_n5, assign24870_e28382_d_n6, assign24870_e28382_d_n7, assign24870_e28382_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) && (var_guard463 == 0.0)) && (var_guard464 != 0.0)) {
        let assign24870_e28379: f64 = (1.0 - var_tmp);
        let assign24870_e28380: f64 = (1.0 / assign24870_e28379);
        (assign24870_e28380, (-((-var_tmp_dn5) / (assign24870_e28379 * assign24870_e28379))), (-((-var_tmp_dn6) / (assign24870_e28379 * assign24870_e28379))), (-((-var_tmp_dn7) / (assign24870_e28379 * assign24870_e28379))), (-((-var_tmp_dn8) / (assign24870_e28379 * assign24870_e28379))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign24870_e28382;
        var_fbreakdown_dn5 = assign24870_e28382_d_n5;
        var_fbreakdown_dn6 = assign24870_e28382_d_n6;
        var_fbreakdown_dn7 = assign24870_e28382_d_n7;
        var_fbreakdown_dn8 = assign24870_e28382_d_n8;

        let (assign24880_e28405, assign24880_e28405_d_n5, assign24880_e28405_d_n6, assign24880_e28405_d_n7, assign24880_e28405_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) && (var_guard463 == 0.0)) && (var_guard464 == 0.0)) {
        let assign24880_e28399: f64 = (var_alphaav * p.p855);
        let assign24880_e28400: f64 = (var_vav + assign24880_e28399);
        let assign24880_e28402: f64 = (assign24880_e28400 * var_slopegat);
        let assign24880_e28403: f64 = (var_fstopgat + assign24880_e28402);
        (assign24880_e28403, (assign24880_e28400 * var_slopegat_dn5), (assign24880_e28400 * var_slopegat_dn6), (assign24880_e28400 * var_slopegat_dn7), (assign24880_e28400 * var_slopegat_dn8),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign24880_e28405;
        var_fbreakdown_dn5 = assign24880_e28405_d_n5;
        var_fbreakdown_dn6 = assign24880_e28405_d_n6;
        var_fbreakdown_dn7 = assign24880_e28405_d_n7;
        var_fbreakdown_dn8 = assign24880_e28405_d_n8;

        let (assign24890_e28424, assign24890_e28424_d_n5, assign24890_e28424_d_n6, assign24890_e28424_d_n7, assign24890_e28424_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard449 == 0.0)) {
        let assign24890_e28415: f64 = (var_id__blk213 + var_isrh);
        let assign24890_e28417: f64 = (assign24890_e28415 + var_itat);
        let assign24890_e28419: f64 = (assign24890_e28417 + var_ibbt);
        let assign24890_e28420: f64 = (p.p29 * assign24890_e28419);
        let assign24890_e28422: f64 = (assign24890_e28420 * var_fbreakdown);
        (assign24890_e28422, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign24890_e28420 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign24890_e28420 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign24890_e28420 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign24890_e28420 * var_fbreakdown_dn8)),)
    } else {
        (var_ijungat, var_ijungat_dn5, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8,)
    }
};
        var_ijungat = assign24890_e28424;
        var_ijungat_dn5 = assign24890_e28424_d_n5;
        var_ijungat_dn6 = assign24890_e28424_d_n6;
        var_ijungat_dn7 = assign24890_e28424_d_n7;
        var_ijungat_dn8 = assign24890_e28424_d_n8;

        let (assign24900_e28440, assign24900_e28440_d_n5, assign24900_e28440_d_n6, assign24900_e28440_d_n7, assign24900_e28440_d_n8,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        let assign24900_e28430: f64 = (var_absource_i * var_ijunbot);
        let assign24900_e28433: f64 = (var_lssource_i * var_ijunsti);
        let assign24900_e28434: f64 = (assign24900_e28430 + assign24900_e28433);
        let assign24900_e28437: f64 = (var_lgsource_i * var_ijungat);
        let assign24900_e28438: f64 = (assign24900_e28434 + assign24900_e28437);
        (assign24900_e28438, (((var_absource_i * var_ijunbot_dn5) + (var_lssource_i * var_ijunsti_dn5)) + (var_lgsource_i * var_ijungat_dn5)), (((var_absource_i * var_ijunbot_dn6) + (var_lssource_i * var_ijunsti_dn6)) + (var_lgsource_i * var_ijungat_dn6)), (((var_absource_i * var_ijunbot_dn7) + (var_lssource_i * var_ijunsti_dn7)) + (var_lgsource_i * var_ijungat_dn7)), (((var_absource_i * var_ijunbot_dn8) + (var_lssource_i * var_ijunsti_dn8)) + (var_lgsource_i * var_ijungat_dn8)),)
    } else {
        (var_i4, var_i4_dn5, var_i4_dn6, var_i4_dn7, var_i4_dn8,)
    }
};
        var_i4 = assign24900_e28440;
        var_i4_dn5 = assign24900_e28440_d_n5;
        var_i4_dn6 = assign24900_e28440_d_n6;
        var_i4_dn7 = assign24900_e28440_d_n7;
        var_i4_dn8 = assign24900_e28440_d_n8;

        let (assign24910_e28446,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        (0.0,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign24910_e28446;

        let (assign24920_e28452,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        (0.0,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign24920_e28452;

        let assign24930_e28464: f64 = if (!(((var_absource_i == 0.0) && (var_lssource_i == 0.0)) && (var_lgsource_i == 0.0))) { 1.0 } else { 0.0 };
        var_guard466 = assign24930_e28464;

        let assign25010_e28550: f64 = if var_v5 < var_vmax_s { 1.0 } else { 0.0 };
        var_guard467 = assign25010_e28550;

        let assign25020_e28552: f64 = (-0.5);
        let assign25020_e28555: f64 = (var_v5 * var_phitdinv);
        let assign25020_e28556: f64 = (assign25020_e28552 * assign25020_e28555);
        let assign25020_e28557: f64 = (assign25020_e28556).abs();
        let assign25020_e28559: f64 = if assign25020_e28557 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard468 = assign25020_e28559;

        let (assign25030_e28577,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard466 != 0.0)) && (var_guard467 != 0.0)) && (var_guard468 != 0.0)) {
        let assign25030_e28570: f64 = (-0.5);
        let assign25030_e28573: f64 = (var_v5 * var_phitdinv);
        let assign25030_e28574: f64 = (assign25030_e28570 * assign25030_e28573);
        let assign25030_e28575: f64 = (assign25030_e28574).exp();
        (assign25030_e28575,)
    } else {
        (var_z,)
    }
};
        var_z = assign25030_e28577;

        let assign25040_e28579: f64 = (-0.5);
        let assign25040_e28582: f64 = (var_v5 * var_phitdinv);
        let assign25040_e28583: f64 = (assign25040_e28579 * assign25040_e28582);
        let assign25040_e28585: f64 = if assign25040_e28583 < 0.0 { 1.0 } else { 0.0 };
        var_guard469 = assign25040_e28585;

        let (assign25050_e28640,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard466 != 0.0)) && (var_guard467 != 0.0)) && (var_guard468 == 0.0)) && (var_guard469 != 0.0)) {
        let assign25050_e28601: f64 = (-230.25850929940458);
        let assign25050_e28603: f64 = (-0.5);
        let assign25050_e28606: f64 = (var_v5 * var_phitdinv);
        let assign25050_e28607: f64 = (assign25050_e28603 * assign25050_e28606);
        let assign25050_e28608: f64 = (assign25050_e28601 - assign25050_e28607);
        let assign25050_e28612: f64 = (-230.25850929940458);
        let assign25050_e28614: f64 = (-0.5);
        let assign25050_e28617: f64 = (var_v5 * var_phitdinv);
        let assign25050_e28618: f64 = (assign25050_e28614 * assign25050_e28617);
        let assign25050_e28619: f64 = (assign25050_e28612 - assign25050_e28618);
        let assign25050_e28622: f64 = (-230.25850929940458);
        let assign25050_e28624: f64 = (-0.5);
        let assign25050_e28627: f64 = (var_v5 * var_phitdinv);
        let assign25050_e28628: f64 = (assign25050_e28624 * assign25050_e28627);
        let assign25050_e28629: f64 = (assign25050_e28622 - assign25050_e28628);
        let assign25050_e28631: f64 = (assign25050_e28629 * 0.3333333333333333);
        let assign25050_e28632: f64 = (1.0 + assign25050_e28631);
        let assign25050_e28633: f64 = (assign25050_e28619 * assign25050_e28632);
        let assign25050_e28634: f64 = (0.5 * assign25050_e28633);
        let assign25050_e28635: f64 = (1.0 + assign25050_e28634);
        let assign25050_e28636: f64 = (assign25050_e28608 * assign25050_e28635);
        let assign25050_e28637: f64 = (1.0 + assign25050_e28636);
        let assign25050_e28638: f64 = (1e-100 / assign25050_e28637);
        (assign25050_e28638,)
    } else {
        (var_z,)
    }
};
        var_z = assign25050_e28640;

        let (assign25060_e28693,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard466 != 0.0)) && (var_guard467 != 0.0)) && (var_guard468 == 0.0)) && (var_guard469 == 0.0)) {
        let assign25060_e28657: f64 = (-0.5);
        let assign25060_e28660: f64 = (var_v5 * var_phitdinv);
        let assign25060_e28661: f64 = (assign25060_e28657 * assign25060_e28660);
        let assign25060_e28663: f64 = (assign25060_e28661 - 230.25850929940458);
        let assign25060_e28667: f64 = (-0.5);
        let assign25060_e28670: f64 = (var_v5 * var_phitdinv);
        let assign25060_e28671: f64 = (assign25060_e28667 * assign25060_e28670);
        let assign25060_e28673: f64 = (assign25060_e28671 - 230.25850929940458);
        let assign25060_e28676: f64 = (-0.5);
        let assign25060_e28679: f64 = (var_v5 * var_phitdinv);
        let assign25060_e28680: f64 = (assign25060_e28676 * assign25060_e28679);
        let assign25060_e28682: f64 = (assign25060_e28680 - 230.25850929940458);
        let assign25060_e28684: f64 = (assign25060_e28682 * 0.3333333333333333);
        let assign25060_e28685: f64 = (1.0 + assign25060_e28684);
        let assign25060_e28686: f64 = (assign25060_e28673 * assign25060_e28685);
        let assign25060_e28687: f64 = (0.5 * assign25060_e28686);
        let assign25060_e28688: f64 = (1.0 + assign25060_e28687);
        let assign25060_e28689: f64 = (assign25060_e28663 * assign25060_e28688);
        let assign25060_e28690: f64 = (1.0 + assign25060_e28689);
        let assign25060_e28691: f64 = (1e100 * assign25060_e28690);
        (assign25060_e28691,)
    } else {
        (var_z,)
    }
};
        var_z = assign25060_e28693;

        *var_erfctimesexpmtat_slot = var_erfctimesexpmtat;
        *var_erfctimesexpmtat_dn5_slot = var_erfctimesexpmtat_dn5;
        *var_erfctimesexpmtat_dn6_slot = var_erfctimesexpmtat_dn6;
        *var_erfctimesexpmtat_dn7_slot = var_erfctimesexpmtat_dn7;
        *var_erfctimesexpmtat_dn8_slot = var_erfctimesexpmtat_dn8;
        *var_fbreakdown_slot = var_fbreakdown;
        *var_fbreakdown_dn5_slot = var_fbreakdown_dn5;
        *var_fbreakdown_dn6_slot = var_fbreakdown_dn6;
        *var_fbreakdown_dn7_slot = var_fbreakdown_dn7;
        *var_fbreakdown_dn8_slot = var_fbreakdown_dn8;
        *var_fmaxr_slot = var_fmaxr;
        *var_fmaxr_dn5_slot = var_fmaxr_dn5;
        *var_fmaxr_dn6_slot = var_fmaxr_dn6;
        *var_fmaxr_dn7_slot = var_fmaxr_dn7;
        *var_fmaxr_dn8_slot = var_fmaxr_dn8;
        *var_gammamax_slot = var_gammamax;
        *var_gammamax_dn5_slot = var_gammamax_dn5;
        *var_gammamax_dn6_slot = var_gammamax_dn6;
        *var_gammamax_dn7_slot = var_gammamax_dn7;
        *var_gammamax_dn8_slot = var_gammamax_dn8;
        *var_guard459_slot = var_guard459;
        *var_guard460_slot = var_guard460;
        *var_guard461_slot = var_guard461;
        *var_guard462_slot = var_guard462;
        *var_guard463_slot = var_guard463;
        *var_guard464_slot = var_guard464;
        *var_guard465_slot = var_guard465;
        *var_guard466_slot = var_guard466;
        *var_guard467_slot = var_guard467;
        *var_guard468_slot = var_guard468;
        *var_guard469_slot = var_guard469;
        *var_i4_slot = var_i4;
        *var_i4_dn5_slot = var_i4_dn5;
        *var_i4_dn6_slot = var_i4_dn6;
        *var_i4_dn7_slot = var_i4_dn7;
        *var_i4_dn8_slot = var_i4_dn8;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn5_slot = var_ibbt_dn5;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
        *var_ijungat_slot = var_ijungat;
        *var_ijungat_dn5_slot = var_ijungat_dn5;
        *var_ijungat_dn6_slot = var_ijungat_dn6;
        *var_ijungat_dn7_slot = var_ijungat_dn7;
        *var_ijungat_dn8_slot = var_ijungat_dn8;
        *var_itat_slot = var_itat;
        *var_itat_dn5_slot = var_itat_dn5;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn5_slot = var_tmp_dn5;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_two_psistar_slot = var_two_psistar;
        *var_vbbt_slot = var_vbbt;
        *var_z_slot = var_z;
    }

    pub(super) fn stamp_transient_block_47(
        p: &Parameters,
        var_absource_i: f64,
        var_atatbot: f64,
        var_btatpartbot: f64,
        var_exp_vmax_over_phitd_s: f64,
        var_ftdbot: f64,
        var_guard176: f64,
        var_guard193: f64,
        var_guard466: f64,
        var_guard467: f64,
        var_idsatbot: f64,
        var_one_minus_pbot: f64,
        var_one_over_one_minus_pbot: f64,
        var_phitd: f64,
        var_phitdinv: f64,
        var_phitr: f64,
        var_v5: f64,
        var_vbbtlim_s: f64,
        var_vbibot: f64,
        var_vbimin_s: f64,
        var_vbirbotinv: f64,
        var_vmax_s: f64,
        var_wdepnulrbot: f64,
        var_asrh_slot: &mut f64,
        var_asrh_dn5_slot: &mut f64,
        var_asrh_dn6_slot: &mut f64,
        var_asrh_dn7_slot: &mut f64,
        var_asrh_dn8_slot: &mut f64,
        var_btat_slot: &mut f64,
        var_btat_dn5_slot: &mut f64,
        var_btat_dn6_slot: &mut f64,
        var_btat_dn7_slot: &mut f64,
        var_btat_dn8_slot: &mut f64,
        var_dwsrh_slot: &mut f64,
        var_guard470_slot: &mut f64,
        var_guard471_slot: &mut f64,
        var_guard472_slot: &mut f64,
        var_guard473_slot: &mut f64,
        var_guard474_slot: &mut f64,
        var_guard475_slot: &mut f64,
        var_guard476_slot: &mut f64,
        var_id__blk213_slot: &mut f64,
        var_idmult_slot: &mut f64,
        var_ijunbot_slot: &mut f64,
        var_ijunbot_dn5_slot: &mut f64,
        var_ijunbot_dn6_slot: &mut f64,
        var_ijunbot_dn7_slot: &mut f64,
        var_ijunbot_dn8_slot: &mut f64,
        var_isrh_slot: &mut f64,
        var_isrh_dn5_slot: &mut f64,
        var_isrh_dn6_slot: &mut f64,
        var_isrh_dn7_slot: &mut f64,
        var_isrh_dn8_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn5_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_sqrtumax_slot: &mut f64,
        var_sqrtumax_dn5_slot: &mut f64,
        var_sqrtumax_dn6_slot: &mut f64,
        var_sqrtumax_dn7_slot: &mut f64,
        var_sqrtumax_dn8_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn5_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_two_psistar_slot: &mut f64,
        var_twoatatoverthreebtat_slot: &mut f64,
        var_twoatatoverthreebtat_dn5_slot: &mut f64,
        var_twoatatoverthreebtat_dn6_slot: &mut f64,
        var_twoatatoverthreebtat_dn7_slot: &mut f64,
        var_twoatatoverthreebtat_dn8_slot: &mut f64,
        var_umax_slot: &mut f64,
        var_umax_dn5_slot: &mut f64,
        var_umax_dn6_slot: &mut f64,
        var_umax_dn7_slot: &mut f64,
        var_umax_dn8_slot: &mut f64,
        var_umaxbeforelimiting_slot: &mut f64,
        var_umaxbeforelimiting_dn5_slot: &mut f64,
        var_umaxbeforelimiting_dn6_slot: &mut f64,
        var_umaxbeforelimiting_dn7_slot: &mut f64,
        var_umaxbeforelimiting_dn8_slot: &mut f64,
        var_umaxpoweronepointfive_slot: &mut f64,
        var_umaxpoweronepointfive_dn5_slot: &mut f64,
        var_umaxpoweronepointfive_dn6_slot: &mut f64,
        var_umaxpoweronepointfive_dn7_slot: &mut f64,
        var_umaxpoweronepointfive_dn8_slot: &mut f64,
        var_vav_slot: &mut f64,
        var_vbbt_slot: &mut f64,
        var_vbi_minus_vjsrh_slot: &mut f64,
        var_vjlim_slot: &mut f64,
        var_vjsrh_slot: &mut f64,
        var_wdep_slot: &mut f64,
        var_wdep_dn5_slot: &mut f64,
        var_wdep_dn6_slot: &mut f64,
        var_wdep_dn7_slot: &mut f64,
        var_wdep_dn8_slot: &mut f64,
        var_wgamma_slot: &mut f64,
        var_wgamma_dn5_slot: &mut f64,
        var_wgamma_dn6_slot: &mut f64,
        var_wgamma_dn7_slot: &mut f64,
        var_wgamma_dn8_slot: &mut f64,
        var_wsrh_slot: &mut f64,
        var_wsrhstep_slot: &mut f64,
        var_z_slot: &mut f64,
        var_zinv_slot: &mut f64,
    ) {
        let mut var_asrh: f64 = *var_asrh_slot;
        let mut var_asrh_dn5: f64 = *var_asrh_dn5_slot;
        let mut var_asrh_dn6: f64 = *var_asrh_dn6_slot;
        let mut var_asrh_dn7: f64 = *var_asrh_dn7_slot;
        let mut var_asrh_dn8: f64 = *var_asrh_dn8_slot;
        let mut var_btat: f64 = *var_btat_slot;
        let mut var_btat_dn5: f64 = *var_btat_dn5_slot;
        let mut var_btat_dn6: f64 = *var_btat_dn6_slot;
        let mut var_btat_dn7: f64 = *var_btat_dn7_slot;
        let mut var_btat_dn8: f64 = *var_btat_dn8_slot;
        let mut var_dwsrh: f64 = *var_dwsrh_slot;
        let mut var_guard470: f64 = *var_guard470_slot;
        let mut var_guard471: f64 = *var_guard471_slot;
        let mut var_guard472: f64 = *var_guard472_slot;
        let mut var_guard473: f64 = *var_guard473_slot;
        let mut var_guard474: f64 = *var_guard474_slot;
        let mut var_guard475: f64 = *var_guard475_slot;
        let mut var_guard476: f64 = *var_guard476_slot;
        let mut var_id__blk213: f64 = *var_id__blk213_slot;
        let mut var_idmult: f64 = *var_idmult_slot;
        let mut var_ijunbot: f64 = *var_ijunbot_slot;
        let mut var_ijunbot_dn5: f64 = *var_ijunbot_dn5_slot;
        let mut var_ijunbot_dn6: f64 = *var_ijunbot_dn6_slot;
        let mut var_ijunbot_dn7: f64 = *var_ijunbot_dn7_slot;
        let mut var_ijunbot_dn8: f64 = *var_ijunbot_dn8_slot;
        let mut var_isrh: f64 = *var_isrh_slot;
        let mut var_isrh_dn5: f64 = *var_isrh_dn5_slot;
        let mut var_isrh_dn6: f64 = *var_isrh_dn6_slot;
        let mut var_isrh_dn7: f64 = *var_isrh_dn7_slot;
        let mut var_isrh_dn8: f64 = *var_isrh_dn8_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn5: f64 = *var_itat_dn5_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_sqrtumax: f64 = *var_sqrtumax_slot;
        let mut var_sqrtumax_dn5: f64 = *var_sqrtumax_dn5_slot;
        let mut var_sqrtumax_dn6: f64 = *var_sqrtumax_dn6_slot;
        let mut var_sqrtumax_dn7: f64 = *var_sqrtumax_dn7_slot;
        let mut var_sqrtumax_dn8: f64 = *var_sqrtumax_dn8_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn5: f64 = *var_tmp_dn5_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_two_psistar: f64 = *var_two_psistar_slot;
        let mut var_twoatatoverthreebtat: f64 = *var_twoatatoverthreebtat_slot;
        let mut var_twoatatoverthreebtat_dn5: f64 = *var_twoatatoverthreebtat_dn5_slot;
        let mut var_twoatatoverthreebtat_dn6: f64 = *var_twoatatoverthreebtat_dn6_slot;
        let mut var_twoatatoverthreebtat_dn7: f64 = *var_twoatatoverthreebtat_dn7_slot;
        let mut var_twoatatoverthreebtat_dn8: f64 = *var_twoatatoverthreebtat_dn8_slot;
        let mut var_umax: f64 = *var_umax_slot;
        let mut var_umax_dn5: f64 = *var_umax_dn5_slot;
        let mut var_umax_dn6: f64 = *var_umax_dn6_slot;
        let mut var_umax_dn7: f64 = *var_umax_dn7_slot;
        let mut var_umax_dn8: f64 = *var_umax_dn8_slot;
        let mut var_umaxbeforelimiting: f64 = *var_umaxbeforelimiting_slot;
        let mut var_umaxbeforelimiting_dn5: f64 = *var_umaxbeforelimiting_dn5_slot;
        let mut var_umaxbeforelimiting_dn6: f64 = *var_umaxbeforelimiting_dn6_slot;
        let mut var_umaxbeforelimiting_dn7: f64 = *var_umaxbeforelimiting_dn7_slot;
        let mut var_umaxbeforelimiting_dn8: f64 = *var_umaxbeforelimiting_dn8_slot;
        let mut var_umaxpoweronepointfive: f64 = *var_umaxpoweronepointfive_slot;
        let mut var_umaxpoweronepointfive_dn5: f64 = *var_umaxpoweronepointfive_dn5_slot;
        let mut var_umaxpoweronepointfive_dn6: f64 = *var_umaxpoweronepointfive_dn6_slot;
        let mut var_umaxpoweronepointfive_dn7: f64 = *var_umaxpoweronepointfive_dn7_slot;
        let mut var_umaxpoweronepointfive_dn8: f64 = *var_umaxpoweronepointfive_dn8_slot;
        let mut var_vav: f64 = *var_vav_slot;
        let mut var_vbbt: f64 = *var_vbbt_slot;
        let mut var_vbi_minus_vjsrh: f64 = *var_vbi_minus_vjsrh_slot;
        let mut var_vjlim: f64 = *var_vjlim_slot;
        let mut var_vjsrh: f64 = *var_vjsrh_slot;
        let mut var_wdep: f64 = *var_wdep_slot;
        let mut var_wdep_dn5: f64 = *var_wdep_dn5_slot;
        let mut var_wdep_dn6: f64 = *var_wdep_dn6_slot;
        let mut var_wdep_dn7: f64 = *var_wdep_dn7_slot;
        let mut var_wdep_dn8: f64 = *var_wdep_dn8_slot;
        let mut var_wgamma: f64 = *var_wgamma_slot;
        let mut var_wgamma_dn5: f64 = *var_wgamma_dn5_slot;
        let mut var_wgamma_dn6: f64 = *var_wgamma_dn6_slot;
        let mut var_wgamma_dn7: f64 = *var_wgamma_dn7_slot;
        let mut var_wgamma_dn8: f64 = *var_wgamma_dn8_slot;
        let mut var_wsrh: f64 = *var_wsrh_slot;
        let mut var_wsrhstep: f64 = *var_wsrhstep_slot;
        let mut var_z: f64 = *var_z_slot;
        let mut var_zinv: f64 = *var_zinv_slot;

        let (assign25070_e28705,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard466 != 0.0)) && (var_guard467 != 0.0)) {
        let assign25070_e28703: f64 = (1.0 / var_z);
        (assign25070_e28703,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign25070_e28705;

        let (assign25080_e28717,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard466 != 0.0)) && (var_guard467 != 0.0)) {
        let assign25080_e28715: f64 = (var_zinv * var_zinv);
        (assign25080_e28715,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign25080_e28717;

        let (assign25090_e28736,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard466 != 0.0)) && (var_guard467 == 0.0)) {
        let assign25090_e28729: f64 = (var_v5 - var_vmax_s);
        let assign25090_e28731: f64 = (assign25090_e28729 * var_phitdinv);
        let assign25090_e28732: f64 = (1.0 + assign25090_e28731);
        let assign25090_e28734: f64 = (assign25090_e28732 * var_exp_vmax_over_phitd_s);
        (assign25090_e28734,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign25090_e28736;

        let (assign25100_e28748,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard466 != 0.0)) && (var_guard467 == 0.0)) {
        let assign25100_e28746: f64 = (var_idmult).sqrt();
        (assign25100_e28746,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign25100_e28748;

        let (assign25110_e28761,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard466 != 0.0)) && (var_guard467 == 0.0)) {
        let assign25110_e28759: f64 = (1.0 / var_zinv);
        (assign25110_e28759,)
    } else {
        (var_z,)
    }
};
        var_z = assign25110_e28761;

        let (assign25120_e28771,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard466 != 0.0)) {
        let assign25120_e28769: f64 = (var_idmult - 1.0);
        (assign25120_e28769,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign25120_e28771;

        let assign25130_e28774: f64 = if var_v5 > 0.0 { 1.0 } else { 0.0 };
        var_guard470 = assign25130_e28774;

        let (assign25140_e28800,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard466 != 0.0)) && (var_guard470 != 0.0)) {
        let assign25140_e28786: f64 = (2.0 + var_z);
        let assign25140_e28789: f64 = (var_z + 1.0);
        let assign25140_e28792: f64 = (var_z + 3.0);
        let assign25140_e28793: f64 = (assign25140_e28789 * assign25140_e28792);
        let assign25140_e28794: f64 = (assign25140_e28793).sqrt();
        let assign25140_e28795: f64 = (assign25140_e28786 + assign25140_e28794);
        let assign25140_e28796: f64 = (assign25140_e28795).ln();
        let assign25140_e28797: f64 = (var_phitd * assign25140_e28796);
        let assign25140_e28798: f64 = (2.0 * assign25140_e28797);
        (assign25140_e28798,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign25140_e28800;

        let (assign25150_e28834,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard466 != 0.0)) && (var_guard470 == 0.0)) {
        let assign25150_e28810: f64 = (-var_v5);
        let assign25150_e28815: f64 = (2.0 * var_zinv);
        let assign25150_e28817: f64 = (assign25150_e28815 + 1.0);
        let assign25150_e28820: f64 = (1.0 + var_zinv);
        let assign25150_e28824: f64 = (3.0 * var_zinv);
        let assign25150_e28825: f64 = (1.0 + assign25150_e28824);
        let assign25150_e28826: f64 = (assign25150_e28820 * assign25150_e28825);
        let assign25150_e28827: f64 = (assign25150_e28826).sqrt();
        let assign25150_e28828: f64 = (assign25150_e28817 + assign25150_e28827);
        let assign25150_e28829: f64 = (assign25150_e28828).ln();
        let assign25150_e28830: f64 = (var_phitd * assign25150_e28829);
        let assign25150_e28831: f64 = (2.0 * assign25150_e28830);
        let assign25150_e28832: f64 = (assign25150_e28810 + assign25150_e28831);
        (assign25150_e28832,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign25150_e28834;

        let (assign25160_e28844,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard466 != 0.0)) {
        let assign25160_e28842: f64 = (var_vbimin_s - var_two_psistar);
        (assign25160_e28842,)
    } else {
        (var_vjlim,)
    }
};
        var_vjlim = assign25160_e28844;

        let (assign25170_e28871,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard466 != 0.0)) {
        let assign25170_e28853: f64 = (var_v5 + var_vjlim);
        let assign25170_e28856: f64 = (var_v5 - var_vjlim);
        let assign25170_e28859: f64 = (var_v5 - var_vjlim);
        let assign25170_e28860: f64 = (assign25170_e28856 * assign25170_e28859);
        let assign25170_e28863: f64 = (4.0 * var_phitd);
        let assign25170_e28865: f64 = (assign25170_e28863 * var_phitd);
        let assign25170_e28866: f64 = (assign25170_e28860 + assign25170_e28865);
        let assign25170_e28867: f64 = (assign25170_e28866).sqrt();
        let assign25170_e28868: f64 = (assign25170_e28853 - assign25170_e28867);
        let assign25170_e28869: f64 = (0.5 * assign25170_e28868);
        (assign25170_e28869,)
    } else {
        (var_vjsrh,)
    }
};
        var_vjsrh = assign25170_e28871;

        let (assign25180_e28898,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard466 != 0.0)) {
        let assign25180_e28880: f64 = (var_v5 + var_vbbtlim_s);
        let assign25180_e28883: f64 = (var_v5 - var_vbbtlim_s);
        let assign25180_e28886: f64 = (var_v5 - var_vbbtlim_s);
        let assign25180_e28887: f64 = (assign25180_e28883 * assign25180_e28886);
        let assign25180_e28890: f64 = (4.0 * var_phitr);
        let assign25180_e28892: f64 = (assign25180_e28890 * var_phitr);
        let assign25180_e28893: f64 = (assign25180_e28887 + assign25180_e28892);
        let assign25180_e28894: f64 = (assign25180_e28893).sqrt();
        let assign25180_e28895: f64 = (assign25180_e28880 - assign25180_e28894);
        let assign25180_e28896: f64 = (0.5 * assign25180_e28895);
        (assign25180_e28896,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign25180_e28898;

        let (assign25190_e28925,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard466 != 0.0)) {
        let assign25190_e28907: f64 = var_v5;
        let assign25190_e28910: f64 = var_v5;
        let assign25190_e28913: f64 = var_v5;
        let assign25190_e28914: f64 = (assign25190_e28910 * assign25190_e28913);
        let assign25190_e28917: f64 = (4.0 * 1e-6);
        let assign25190_e28919: f64 = (assign25190_e28917 * 1e-6);
        let assign25190_e28920: f64 = (assign25190_e28914 + assign25190_e28919);
        let assign25190_e28921: f64 = (assign25190_e28920).sqrt();
        let assign25190_e28922: f64 = (assign25190_e28907 - assign25190_e28921);
        let assign25190_e28923: f64 = (0.5 * assign25190_e28922);
        (assign25190_e28923,)
    } else {
        (var_vav,)
    }
};
        var_vav = assign25190_e28925;

        let assign25200_e28928: f64 = if var_absource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard471 = assign25200_e28928;

        let (assign25210_e28936, assign25210_e28936_d_n5, assign25210_e28936_d_n6, assign25210_e28936_d_n7, assign25210_e28936_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunbot, var_ijunbot_dn5, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8,)
    }
};
        var_ijunbot = assign25210_e28936;
        var_ijunbot_dn5 = assign25210_e28936_d_n5;
        var_ijunbot_dn6 = assign25210_e28936_d_n6;
        var_ijunbot_dn7 = assign25210_e28936_d_n7;
        var_ijunbot_dn8 = assign25210_e28936_d_n8;

        let (assign25220_e28947,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) {
        let assign25220_e28945: f64 = (var_idsatbot * var_idmult);
        (assign25220_e28945,)
    } else {
        (var_id__blk213,)
    }
};
        var_id__blk213 = assign25220_e28947;

        let assign25230_e28954: f64 = if ((p.p833 == 0.0) && (p.p838 == 0.0)) { 1.0 } else { 0.0 };
        var_guard472 = assign25230_e28954;

        let (assign25240_e28965, assign25240_e28965_d_n5, assign25240_e28965_d_n6, assign25240_e28965_d_n7, assign25240_e28965_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) && (var_guard472 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign25240_e28965;
        var_isrh_dn5 = assign25240_e28965_d_n5;
        var_isrh_dn6 = assign25240_e28965_d_n6;
        var_isrh_dn7 = assign25240_e28965_d_n7;
        var_isrh_dn8 = assign25240_e28965_d_n8;

        let (assign25250_e28979,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) && (var_guard472 == 0.0)) {
        let assign25250_e28977: f64 = (var_vbibot - var_vjsrh);
        (assign25250_e28977,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign25250_e28979;

        let (assign25260_e28998,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) && (var_guard472 == 0.0)) {
        let assign25260_e28993: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign25260_e28994: f64 = (1.0 - assign25260_e28993);
        let assign25260_e28995: f64 = (assign25260_e28994).sqrt();
        let assign25260_e28996: f64 = (1.0 - assign25260_e28995);
        (assign25260_e28996,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign25260_e28998;

        let assign25270_e29001: f64 = if p.p824 == 0.5 { 1.0 } else { 0.0 };
        var_guard473 = assign25270_e29001;

        let (assign25280_e29015,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) && (var_guard472 == 0.0)) && (var_guard473 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign25280_e29015;

        let (assign25290_e29047,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) && (var_guard472 == 0.0)) && (var_guard473 == 0.0)) {
        let assign25290_e29030: f64 = (var_wsrhstep * var_wsrhstep);
        let assign25290_e29032: f64 = (var_wsrhstep).ln();
        let assign25290_e29033: f64 = (assign25290_e29030 * assign25290_e29032);
        let assign25290_e29036: f64 = (1.0 - var_wsrhstep);
        let assign25290_e29037: f64 = (assign25290_e29033 / assign25290_e29036);
        let assign25290_e29039: f64 = (assign25290_e29037 + var_wsrhstep);
        let assign25290_e29043: f64 = (2.0 * p.p824);
        let assign25290_e29044: f64 = (1.0 - assign25290_e29043);
        let assign25290_e29045: f64 = (assign25290_e29039 * assign25290_e29044);
        (assign25290_e29045,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign25290_e29047;

        let (assign25300_e29061,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) && (var_guard472 == 0.0)) {
        let assign25300_e29059: f64 = (var_wsrhstep + var_dwsrh);
        (assign25300_e29059,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign25300_e29061;

        let assign25310_e29064: f64 = if p.p824 == 0.5 { 1.0 } else { 0.0 };
        var_guard474 = assign25310_e29064;

        let (assign25320_e29081, assign25320_e29081_d_n5, assign25320_e29081_d_n6, assign25320_e29081_d_n7, assign25320_e29081_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) && (var_guard472 == 0.0)) && (var_guard474 != 0.0)) {
        let assign25320_e29078: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv);
        let assign25320_e29079: f64 = (assign25320_e29078).sqrt();
        (assign25320_e29079, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign25320_e29081;
        var_tmp_dn5 = assign25320_e29081_d_n5;
        var_tmp_dn6 = assign25320_e29081_d_n6;
        var_tmp_dn7 = assign25320_e29081_d_n7;
        var_tmp_dn8 = assign25320_e29081_d_n8;

        let (assign25330_e29100, assign25330_e29100_d_n5, assign25330_e29100_d_n6, assign25330_e29100_d_n7, assign25330_e29100_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) && (var_guard472 == 0.0)) && (var_guard474 == 0.0)) {
        let assign25330_e29096: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv);
        let assign25330_e29098: f64 = (assign25330_e29096).powf(p.p824);
        (assign25330_e29098, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign25330_e29100;
        var_tmp_dn5 = assign25330_e29100_d_n5;
        var_tmp_dn6 = assign25330_e29100_d_n6;
        var_tmp_dn7 = assign25330_e29100_d_n7;
        var_tmp_dn8 = assign25330_e29100_d_n8;

        let (assign25340_e29114, assign25340_e29114_d_n5, assign25340_e29114_d_n6, assign25340_e29114_d_n7, assign25340_e29114_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) && (var_guard472 == 0.0)) {
        let assign25340_e29112: f64 = (var_wdepnulrbot * var_tmp);
        (assign25340_e29112, (var_wdepnulrbot * var_tmp_dn5), (var_wdepnulrbot * var_tmp_dn6), (var_wdepnulrbot * var_tmp_dn7), (var_wdepnulrbot * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign25340_e29114;
        var_wdep_dn5 = assign25340_e29114_d_n5;
        var_wdep_dn6 = assign25340_e29114_d_n6;
        var_wdep_dn7 = assign25340_e29114_d_n7;
        var_wdep_dn8 = assign25340_e29114_d_n8;

        let (assign25350_e29132, assign25350_e29132_d_n5, assign25350_e29132_d_n6, assign25350_e29132_d_n7, assign25350_e29132_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) && (var_guard472 == 0.0)) {
        let assign25350_e29127: f64 = (var_zinv - 1.0);
        let assign25350_e29129: f64 = (assign25350_e29127 * var_wdep);
        let assign25350_e29130: f64 = (var_ftdbot * assign25350_e29129);
        (assign25350_e29130, (var_ftdbot * (assign25350_e29127 * var_wdep_dn5)), (var_ftdbot * (assign25350_e29127 * var_wdep_dn6)), (var_ftdbot * (assign25350_e29127 * var_wdep_dn7)), (var_ftdbot * (assign25350_e29127 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign25350_e29132;
        var_asrh_dn5 = assign25350_e29132_d_n5;
        var_asrh_dn6 = assign25350_e29132_d_n6;
        var_asrh_dn7 = assign25350_e29132_d_n7;
        var_asrh_dn8 = assign25350_e29132_d_n8;

        let (assign25360_e29148, assign25360_e29148_d_n5, assign25360_e29148_d_n6, assign25360_e29148_d_n7, assign25360_e29148_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) && (var_guard472 == 0.0)) {
        let assign25360_e29145: f64 = (var_asrh * var_wsrh);
        let assign25360_e29146: f64 = (p.p833 * assign25360_e29145);
        (assign25360_e29146, (p.p833 * (var_asrh_dn5 * var_wsrh)), (p.p833 * (var_asrh_dn6 * var_wsrh)), (p.p833 * (var_asrh_dn7 * var_wsrh)), (p.p833 * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign25360_e29148;
        var_isrh_dn5 = assign25360_e29148_d_n5;
        var_isrh_dn6 = assign25360_e29148_d_n6;
        var_isrh_dn7 = assign25360_e29148_d_n7;
        var_isrh_dn8 = assign25360_e29148_d_n8;

        let assign25370_e29151: f64 = if p.p838 == 0.0 { 1.0 } else { 0.0 };
        var_guard475 = assign25370_e29151;

        let (assign25380_e29162, assign25380_e29162_d_n5, assign25380_e29162_d_n6, assign25380_e29162_d_n7, assign25380_e29162_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) && (var_guard475 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign25380_e29162;
        var_itat_dn5 = assign25380_e29162_d_n5;
        var_itat_dn6 = assign25380_e29162_d_n6;
        var_itat_dn7 = assign25380_e29162_d_n7;
        var_itat_dn8 = assign25380_e29162_d_n8;

        let (assign25390_e29180, assign25390_e29180_d_n5, assign25390_e29180_d_n6, assign25390_e29180_d_n7, assign25390_e29180_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) && (var_guard475 == 0.0)) {
        let assign25390_e29175: f64 = (var_wdep * var_one_minus_pbot);
        let assign25390_e29177: f64 = (assign25390_e29175 / var_vbi_minus_vjsrh);
        let assign25390_e29178: f64 = (var_btatpartbot * assign25390_e29177);
        (assign25390_e29178, (var_btatpartbot * ((var_wdep_dn5 * var_one_minus_pbot) / var_vbi_minus_vjsrh)), (var_btatpartbot * ((var_wdep_dn6 * var_one_minus_pbot) / var_vbi_minus_vjsrh)), (var_btatpartbot * ((var_wdep_dn7 * var_one_minus_pbot) / var_vbi_minus_vjsrh)), (var_btatpartbot * ((var_wdep_dn8 * var_one_minus_pbot) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign25390_e29180;
        var_btat_dn5 = assign25390_e29180_d_n5;
        var_btat_dn6 = assign25390_e29180_d_n6;
        var_btat_dn7 = assign25390_e29180_d_n7;
        var_btat_dn8 = assign25390_e29180_d_n8;

        let (assign25400_e29196, assign25400_e29196_d_n5, assign25400_e29196_d_n6, assign25400_e29196_d_n7, assign25400_e29196_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) && (var_guard475 == 0.0)) {
        let assign25400_e29192: f64 = (0.666666666666667 * var_atatbot);
        let assign25400_e29194: f64 = (assign25400_e29192 / var_btat);
        (assign25400_e29194, (-((assign25400_e29192 * var_btat_dn5) / (var_btat * var_btat))), (-((assign25400_e29192 * var_btat_dn6) / (var_btat * var_btat))), (-((assign25400_e29192 * var_btat_dn7) / (var_btat * var_btat))), (-((assign25400_e29192 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign25400_e29196;
        var_twoatatoverthreebtat_dn5 = assign25400_e29196_d_n5;
        var_twoatatoverthreebtat_dn6 = assign25400_e29196_d_n6;
        var_twoatatoverthreebtat_dn7 = assign25400_e29196_d_n7;
        var_twoatatoverthreebtat_dn8 = assign25400_e29196_d_n8;

        let (assign25410_e29210, assign25410_e29210_d_n5, assign25410_e29210_d_n6, assign25410_e29210_d_n7, assign25410_e29210_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) && (var_guard475 == 0.0)) {
        let assign25410_e29208: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign25410_e29208, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign25410_e29210;
        var_umaxbeforelimiting_dn5 = assign25410_e29210_d_n5;
        var_umaxbeforelimiting_dn6 = assign25410_e29210_d_n6;
        var_umaxbeforelimiting_dn7 = assign25410_e29210_d_n7;
        var_umaxbeforelimiting_dn8 = assign25410_e29210_d_n8;

        let (assign25420_e29231, assign25420_e29231_d_n5, assign25420_e29231_d_n6, assign25420_e29231_d_n7, assign25420_e29231_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) && (var_guard475 == 0.0)) {
        let assign25420_e29222: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign25420_e29225: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign25420_e29227: f64 = (assign25420_e29225 + 1.0);
        let assign25420_e29228: f64 = (assign25420_e29222 / assign25420_e29227);
        let assign25420_e29229: f64 = (assign25420_e29228).sqrt();
        (assign25420_e29229, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign25420_e29227) - (assign25420_e29222 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign25420_e29227 * assign25420_e29227)) / (2.0 * assign25420_e29229)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign25420_e29227) - (assign25420_e29222 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign25420_e29227 * assign25420_e29227)) / (2.0 * assign25420_e29229)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign25420_e29227) - (assign25420_e29222 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign25420_e29227 * assign25420_e29227)) / (2.0 * assign25420_e29229)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign25420_e29227) - (assign25420_e29222 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign25420_e29227 * assign25420_e29227)) / (2.0 * assign25420_e29229)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign25420_e29231;
        var_umax_dn5 = assign25420_e29231_d_n5;
        var_umax_dn6 = assign25420_e29231_d_n6;
        var_umax_dn7 = assign25420_e29231_d_n7;
        var_umax_dn8 = assign25420_e29231_d_n8;

        let (assign25430_e29244, assign25430_e29244_d_n5, assign25430_e29244_d_n6, assign25430_e29244_d_n7, assign25430_e29244_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) && (var_guard475 == 0.0)) {
        let assign25430_e29242: f64 = (var_umax).sqrt();
        (assign25430_e29242, (var_umax_dn5 / (2.0 * assign25430_e29242)), (var_umax_dn6 / (2.0 * assign25430_e29242)), (var_umax_dn7 / (2.0 * assign25430_e29242)), (var_umax_dn8 / (2.0 * assign25430_e29242)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign25430_e29244;
        var_sqrtumax_dn5 = assign25430_e29244_d_n5;
        var_sqrtumax_dn6 = assign25430_e29244_d_n6;
        var_sqrtumax_dn7 = assign25430_e29244_d_n7;
        var_sqrtumax_dn8 = assign25430_e29244_d_n8;

        let (assign25440_e29258, assign25440_e29258_d_n5, assign25440_e29258_d_n6, assign25440_e29258_d_n7, assign25440_e29258_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) && (var_guard475 == 0.0)) {
        let assign25440_e29256: f64 = (var_umax * var_sqrtumax);
        (assign25440_e29256, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign25440_e29258;
        var_umaxpoweronepointfive_dn5 = assign25440_e29258_d_n5;
        var_umaxpoweronepointfive_dn6 = assign25440_e29258_d_n6;
        var_umaxpoweronepointfive_dn7 = assign25440_e29258_d_n7;
        var_umaxpoweronepointfive_dn8 = assign25440_e29258_d_n8;

        let assign25450_e29260: f64 = (-p.p824);
        let assign25450_e29262: f64 = (assign25450_e29260 * var_one_over_one_minus_pbot);
        let assign25450_e29264: f64 = (-1.0);
        let assign25450_e29265: f64 = if assign25450_e29262 == assign25450_e29264 { 1.0 } else { 0.0 };
        var_guard476 = assign25450_e29265;

        let (assign25460_e29285, assign25460_e29285_d_n5, assign25460_e29285_d_n6, assign25460_e29285_d_n7, assign25460_e29285_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) && (var_guard475 == 0.0)) && (var_guard476 != 0.0)) {
        let assign25460_e29281: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign25460_e29282: f64 = (1.0 + assign25460_e29281);
        let assign25460_e29283: f64 = (1.0 / assign25460_e29282);
        (assign25460_e29283, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign25460_e29282 * assign25460_e29282))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign25460_e29282 * assign25460_e29282))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign25460_e29282 * assign25460_e29282))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign25460_e29282 * assign25460_e29282))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign25460_e29285;
        var_wgamma_dn5 = assign25460_e29285_d_n5;
        var_wgamma_dn6 = assign25460_e29285_d_n6;
        var_wgamma_dn7 = assign25460_e29285_d_n7;
        var_wgamma_dn8 = assign25460_e29285_d_n8;

        let (assign25470_e29309, assign25470_e29309_d_n5, assign25470_e29309_d_n6, assign25470_e29309_d_n7, assign25470_e29309_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) && (var_guard475 == 0.0)) && (var_guard476 == 0.0)) {
        let assign25470_e29301: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign25470_e29302: f64 = (1.0 + assign25470_e29301);
        let assign25470_e29304: f64 = (-p.p824);
        let assign25470_e29306: f64 = (assign25470_e29304 * var_one_over_one_minus_pbot);
        let assign25470_e29307: f64 = (assign25470_e29302).powf(assign25470_e29306);
        (assign25470_e29307, if 0.0 == 0.0 && ((assign25470_e29306) as f64).is_finite() && ((assign25470_e29306) as f64).fract() == 0.0 { if assign25470_e29306 == 0.0 { 0.0 } else { (assign25470_e29306 * ((assign25470_e29302).powf(assign25470_e29306 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign25470_e29307 * (assign25470_e29306 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign25470_e29302))) }, if 0.0 == 0.0 && ((assign25470_e29306) as f64).is_finite() && ((assign25470_e29306) as f64).fract() == 0.0 { if assign25470_e29306 == 0.0 { 0.0 } else { (assign25470_e29306 * ((assign25470_e29302).powf(assign25470_e29306 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign25470_e29307 * (assign25470_e29306 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign25470_e29302))) }, if 0.0 == 0.0 && ((assign25470_e29306) as f64).is_finite() && ((assign25470_e29306) as f64).fract() == 0.0 { if assign25470_e29306 == 0.0 { 0.0 } else { (assign25470_e29306 * ((assign25470_e29302).powf(assign25470_e29306 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign25470_e29307 * (assign25470_e29306 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign25470_e29302))) }, if 0.0 == 0.0 && ((assign25470_e29306) as f64).is_finite() && ((assign25470_e29306) as f64).fract() == 0.0 { if assign25470_e29306 == 0.0 { 0.0 } else { (assign25470_e29306 * ((assign25470_e29302).powf(assign25470_e29306 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign25470_e29307 * (assign25470_e29306 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign25470_e29302))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign25470_e29309;
        var_wgamma_dn5 = assign25470_e29309_d_n5;
        var_wgamma_dn6 = assign25470_e29309_d_n6;
        var_wgamma_dn7 = assign25470_e29309_d_n7;
        var_wgamma_dn8 = assign25470_e29309_d_n8;

        *var_asrh_slot = var_asrh;
        *var_asrh_dn5_slot = var_asrh_dn5;
        *var_asrh_dn6_slot = var_asrh_dn6;
        *var_asrh_dn7_slot = var_asrh_dn7;
        *var_asrh_dn8_slot = var_asrh_dn8;
        *var_btat_slot = var_btat;
        *var_btat_dn5_slot = var_btat_dn5;
        *var_btat_dn6_slot = var_btat_dn6;
        *var_btat_dn7_slot = var_btat_dn7;
        *var_btat_dn8_slot = var_btat_dn8;
        *var_dwsrh_slot = var_dwsrh;
        *var_guard470_slot = var_guard470;
        *var_guard471_slot = var_guard471;
        *var_guard472_slot = var_guard472;
        *var_guard473_slot = var_guard473;
        *var_guard474_slot = var_guard474;
        *var_guard475_slot = var_guard475;
        *var_guard476_slot = var_guard476;
        *var_id__blk213_slot = var_id__blk213;
        *var_idmult_slot = var_idmult;
        *var_ijunbot_slot = var_ijunbot;
        *var_ijunbot_dn5_slot = var_ijunbot_dn5;
        *var_ijunbot_dn6_slot = var_ijunbot_dn6;
        *var_ijunbot_dn7_slot = var_ijunbot_dn7;
        *var_ijunbot_dn8_slot = var_ijunbot_dn8;
        *var_isrh_slot = var_isrh;
        *var_isrh_dn5_slot = var_isrh_dn5;
        *var_isrh_dn6_slot = var_isrh_dn6;
        *var_isrh_dn7_slot = var_isrh_dn7;
        *var_isrh_dn8_slot = var_isrh_dn8;
        *var_itat_slot = var_itat;
        *var_itat_dn5_slot = var_itat_dn5;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_sqrtumax_slot = var_sqrtumax;
        *var_sqrtumax_dn5_slot = var_sqrtumax_dn5;
        *var_sqrtumax_dn6_slot = var_sqrtumax_dn6;
        *var_sqrtumax_dn7_slot = var_sqrtumax_dn7;
        *var_sqrtumax_dn8_slot = var_sqrtumax_dn8;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn5_slot = var_tmp_dn5;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_two_psistar_slot = var_two_psistar;
        *var_twoatatoverthreebtat_slot = var_twoatatoverthreebtat;
        *var_twoatatoverthreebtat_dn5_slot = var_twoatatoverthreebtat_dn5;
        *var_twoatatoverthreebtat_dn6_slot = var_twoatatoverthreebtat_dn6;
        *var_twoatatoverthreebtat_dn7_slot = var_twoatatoverthreebtat_dn7;
        *var_twoatatoverthreebtat_dn8_slot = var_twoatatoverthreebtat_dn8;
        *var_umax_slot = var_umax;
        *var_umax_dn5_slot = var_umax_dn5;
        *var_umax_dn6_slot = var_umax_dn6;
        *var_umax_dn7_slot = var_umax_dn7;
        *var_umax_dn8_slot = var_umax_dn8;
        *var_umaxbeforelimiting_slot = var_umaxbeforelimiting;
        *var_umaxbeforelimiting_dn5_slot = var_umaxbeforelimiting_dn5;
        *var_umaxbeforelimiting_dn6_slot = var_umaxbeforelimiting_dn6;
        *var_umaxbeforelimiting_dn7_slot = var_umaxbeforelimiting_dn7;
        *var_umaxbeforelimiting_dn8_slot = var_umaxbeforelimiting_dn8;
        *var_umaxpoweronepointfive_slot = var_umaxpoweronepointfive;
        *var_umaxpoweronepointfive_dn5_slot = var_umaxpoweronepointfive_dn5;
        *var_umaxpoweronepointfive_dn6_slot = var_umaxpoweronepointfive_dn6;
        *var_umaxpoweronepointfive_dn7_slot = var_umaxpoweronepointfive_dn7;
        *var_umaxpoweronepointfive_dn8_slot = var_umaxpoweronepointfive_dn8;
        *var_vav_slot = var_vav;
        *var_vbbt_slot = var_vbbt;
        *var_vbi_minus_vjsrh_slot = var_vbi_minus_vjsrh;
        *var_vjlim_slot = var_vjlim;
        *var_vjsrh_slot = var_vjsrh;
        *var_wdep_slot = var_wdep;
        *var_wdep_dn5_slot = var_wdep_dn5;
        *var_wdep_dn6_slot = var_wdep_dn6;
        *var_wdep_dn7_slot = var_wdep_dn7;
        *var_wdep_dn8_slot = var_wdep_dn8;
        *var_wgamma_slot = var_wgamma;
        *var_wgamma_dn5_slot = var_wgamma_dn5;
        *var_wgamma_dn6_slot = var_wgamma_dn6;
        *var_wgamma_dn7_slot = var_wgamma_dn7;
        *var_wgamma_dn8_slot = var_wgamma_dn8;
        *var_wsrh_slot = var_wsrh;
        *var_wsrhstep_slot = var_wsrhstep;
        *var_z_slot = var_z;
        *var_zinv_slot = var_zinv;
    }
}
